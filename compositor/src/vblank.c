// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Yuxuan Shui <yshuiv7@gmail.com>

#include <assert.h>

#include <ev.h>
#include <inttypes.h>
#include <stdatomic.h>
#include <string.h>
#include <time.h>
#include <xcb/xcb.h>
#include <xcb/xproto.h>
#include "compiler.h"
#include "log.h"
#include "utils/dynarr.h"
#include "vblank.h"
#include "x.h"

struct vblank_closure {
	vblank_callback_t fn;
	void *user_data;
};

#define VBLANK_WIND_DOWN 4

struct vblank_scheduler {
	struct x_connection *c;
	/// List of scheduled vblank callbacks, this is a dynarr
	struct vblank_closure *callbacks;
	struct ev_loop *loop;
	/// Request extra vblank events even when no callbacks are scheduled.
	/// This is because when callbacks are scheduled too close to a vblank,
	/// we might send PresentNotifyMsc request too late and miss the vblank event.
	/// So we request extra vblank events right after the last vblank event
	/// to make sure this doesn't happen.
	unsigned int wind_down;
	xcb_window_t target_window;
	enum vblank_scheduler_type type;
	bool vblank_event_requested;
	bool use_realtime_scheduling;
};

struct present_vblank_scheduler {
	struct vblank_scheduler base;

	uint64_t last_msc;
	/// The timestamp for the end of last vblank.
	uint64_t last_ust;
	ev_timer callback_timer;
	xcb_present_event_t event_id;
	xcb_special_event_t *event;
};

struct vblank_scheduler_ops {
	size_t size;
	bool (*init)(struct vblank_scheduler *self);
	void (*deinit)(struct vblank_scheduler *self);
	bool (*schedule)(struct vblank_scheduler *self);
	bool (*handle_x_events)(struct vblank_scheduler *self);
};

static void
vblank_scheduler_invoke_callbacks(struct vblank_scheduler *self, struct vblank_event *event);

static bool present_vblank_scheduler_schedule(struct vblank_scheduler *base) {
	auto self = (struct present_vblank_scheduler *)base;
	log_verbose("Requesting vblank event for window 0x%08x, msc %" PRIu64,
	            base->target_window, self->last_msc + 1);
	assert(!base->vblank_event_requested);
	x_request_vblank_event(base->c, base->target_window, self->last_msc + 1);
	base->vblank_event_requested = true;
	return true;
}

static void present_vblank_callback(EV_P attr_unused, ev_timer *w, int attr_unused revents) {
	auto sched = container_of(w, struct present_vblank_scheduler, callback_timer);
	auto event = (struct vblank_event){
	    .msc = sched->last_msc,
	    .ust = sched->last_ust,
	};
	sched->base.vblank_event_requested = false;
	vblank_scheduler_invoke_callbacks(&sched->base, &event);
}

static bool present_vblank_scheduler_init(struct vblank_scheduler *base) {
	auto self = (struct present_vblank_scheduler *)base;
	base->type = VBLANK_SCHEDULER_PRESENT;
	ev_timer_init(&self->callback_timer, present_vblank_callback, 0, 0);

	self->event_id = x_new_id(base->c);
	auto select_input =
	    xcb_present_select_input(base->c->c, self->event_id, base->target_window,
	                             XCB_PRESENT_EVENT_MASK_COMPLETE_NOTIFY);
	x_set_error_action_abort(base->c, select_input);
	self->event =
	    xcb_register_for_special_xge(base->c->c, &xcb_present_id, self->event_id, NULL);
	return true;
}

static void present_vblank_scheduler_deinit(struct vblank_scheduler *base) {
	auto self = (struct present_vblank_scheduler *)base;
	ev_timer_stop(base->loop, &self->callback_timer);
	auto select_input =
	    xcb_present_select_input(base->c->c, self->event_id, base->target_window, 0);
	x_set_error_action_abort(base->c, select_input);
	xcb_unregister_for_special_event(base->c->c, self->event);
}

/// Handle PresentCompleteNotify events
///
/// Schedule the registered callback to be called when the current vblank ends.
static void handle_present_complete_notify(struct present_vblank_scheduler *self,
                                           xcb_present_complete_notify_event_t *cne) {
	assert(self->base.type == VBLANK_SCHEDULER_PRESENT);

	if (cne->kind != XCB_PRESENT_COMPLETE_KIND_NOTIFY_MSC) {
		return;
	}

	assert(self->base.vblank_event_requested);

	struct timespec now;
	clock_gettime(CLOCK_MONOTONIC, &now);
	auto now_us = (uint64_t)now.tv_sec * 1000000UL + (uint64_t)now.tv_nsec / 1000;

	// X sometimes sends duplicate/bogus MSC events, when screen has just been turned
	// off. Don't use the msc value in these events. We treat this as not receiving a
	// vblank event at all, and try to get a new one.
	//
	// See:
	// https://gitlab.freedesktop.org/xorg/xserver/-/issues/1418
	bool event_is_invalid = cne->msc <= self->last_msc || cne->ust == 0;
	if (event_is_invalid) {
		log_debug("Invalid PresentCompleteNotify event, %" PRIu64 " %" PRIu64
		          ". Trying to recover, reporting a fake vblank.",
		          cne->msc, cne->ust);
		self->last_ust = now_us;
		self->last_msc += 1;
	} else {
		self->last_ust = cne->ust;
		self->last_msc = cne->msc;
	}
	double delay_sec = 0.0;
	if (now_us < cne->ust) {
		log_trace("The end of this vblank is %" PRIu64 " us into the "
		          "future",
		          cne->ust - now_us);
		delay_sec = (double)(cne->ust - now_us) / 1000000.0;
	}
	// Wait until the end of the current vblank to invoke callbacks. If we
	// call it too early, it can mistakenly think the render missed the
	// vblank, and doesn't schedule render for the next vblank, causing frame
	// drops.
	assert(!ev_is_active(&self->callback_timer));
	ev_timer_set(&self->callback_timer, delay_sec, 0);
	ev_timer_start(self->base.loop, &self->callback_timer);
}

static bool handle_present_events(struct vblank_scheduler *base) {
	auto self = (struct present_vblank_scheduler *)base;
	xcb_present_generic_event_t *ev;
	while ((ev = (void *)xcb_poll_for_special_event(base->c->c, self->event))) {
		if (ev->event != self->event_id) {
			// This event doesn't have the right event context, it's not meant
			// for us.
			goto next;
		}

		// We only subscribed to the complete notify event.
		assert(ev->evtype == XCB_PRESENT_EVENT_COMPLETE_NOTIFY);
		handle_present_complete_notify(self, (void *)ev);
	next:
		free(ev);
	}
	return true;
}

static const struct vblank_scheduler_ops vblank_scheduler_ops[LAST_VBLANK_SCHEDULER] = {
    [VBLANK_SCHEDULER_PRESENT] =
        {
            .size = sizeof(struct present_vblank_scheduler),
            .init = present_vblank_scheduler_init,
            .deinit = present_vblank_scheduler_deinit,
            .schedule = present_vblank_scheduler_schedule,
            .handle_x_events = handle_present_events,
        },
};

static bool vblank_scheduler_schedule_internal(struct vblank_scheduler *self) {
	assert(self->type < LAST_VBLANK_SCHEDULER);
	auto fn = vblank_scheduler_ops[self->type].schedule;
	assert(fn != NULL);
	return fn(self);
}

bool vblank_scheduler_schedule(struct vblank_scheduler *self,
                               vblank_callback_t vblank_callback, void *user_data) {
	// Schedule a new vblank event if there are no callbacks currently scheduled.
	if (dynarr_len(self->callbacks) == 0 && self->wind_down == 0 &&
	    !vblank_scheduler_schedule_internal(self)) {
		return false;
	}
	struct vblank_closure closure = {
	    .fn = vblank_callback,
	    .user_data = user_data,
	};
	dynarr_push(self->callbacks, closure);
	return true;
}

static void
vblank_scheduler_invoke_callbacks(struct vblank_scheduler *self, struct vblank_event *event) {
	// callbacks might be added during callback invocation, so we need to
	// copy the callback_count.
	size_t count = dynarr_len(self->callbacks), write_head = 0;
	if (count == 0) {
		self->wind_down--;
	} else {
		self->wind_down = VBLANK_WIND_DOWN;
	}
	for (size_t i = 0; i < count; i++) {
		auto action = self->callbacks[i].fn(event, self->callbacks[i].user_data);
		switch (action) {
		case VBLANK_CALLBACK_AGAIN:
			if (i != write_head) {
				self->callbacks[write_head] = self->callbacks[i];
			}
			write_head++;
		case VBLANK_CALLBACK_DONE:
		default:        // nothing to do
			break;
		}
	}
	memset(self->callbacks + write_head, 0,
	       (count - write_head) * sizeof(*self->callbacks));
	assert(count == dynarr_len(self->callbacks) && "callbacks should not be added "
	                                               "when callbacks are being "
	                                               "invoked.");
	dynarr_len(self->callbacks) = write_head;
	if (write_head || self->wind_down) {
		vblank_scheduler_schedule_internal(self);
	}
}

void vblank_scheduler_free(struct vblank_scheduler *self) {
	assert(self->type < LAST_VBLANK_SCHEDULER);
	auto fn = vblank_scheduler_ops[self->type].deinit;
	if (fn != NULL) {
		fn(self);
	}
	dynarr_free_pod(self->callbacks);
	free(self);
}

struct vblank_scheduler *
vblank_scheduler_new(struct ev_loop *loop, struct x_connection *c, xcb_window_t target_window,
                     enum vblank_scheduler_type type, bool use_realtime_scheduling) {
	size_t object_size = vblank_scheduler_ops[type].size;
	auto init_fn = vblank_scheduler_ops[type].init;
	if (!object_size || !init_fn) {
		log_error("Unsupported or invalid vblank scheduler type: %d", type);
		return NULL;
	}

	assert(object_size >= sizeof(struct vblank_scheduler));
	struct vblank_scheduler *self = calloc(1, object_size);
	self->target_window = target_window;
	self->c = c;
	self->loop = loop;
	self->use_realtime_scheduling = use_realtime_scheduling;
	self->callbacks = dynarr_new(struct vblank_closure, 1);
	init_fn(self);
	return self;
}

bool vblank_handle_x_events(struct vblank_scheduler *self) {
	assert(self->type < LAST_VBLANK_SCHEDULER);
	auto fn = vblank_scheduler_ops[self->type].handle_x_events;
	if (fn != NULL) {
		return fn(self);
	}
	return true;
}
