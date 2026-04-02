#include "wm.h"
#include "bridge.h"
#include <X11/Xlib.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static Atom wm_sn;
static Window wm_sn_owner;

static int wm_sn_xerror(Display *d, XErrorEvent *e) {
    if (e->error_code == BadWindow)
        return 0;
    return xerror(d, e);
}

int srwm_wm_sn_acquire(int replace) {
    char atom_name[32];
    Screen* screen_ptr;

    screen_ptr = ScreenOfDisplay(dpy, screen);
    
    snprintf(atom_name, sizeof(atom_name), "WM_S%d", screen);
    wm_sn = XInternAtom(dpy, atom_name, False);
    if (wm_sn == None) {
        fprintf(stderr, "srwm: failed to intern WM_Sn atom\n");
        return 1;
    }

    Window owner = XGetSelectionOwner(dpy, wm_sn);
    if (owner != None && !replace) {
        fprintf(stderr, "srwm: another window manager is already running (WM_Sn is owned)\n");
        return 1;
    }

    wm_sn_owner = XCreateSimpleWindow(dpy, screen_ptr->root, -1, -1, 1, 1, 0, 0, 0);
    XChangeProperty(dpy, wm_sn_owner, XA_WM_CLASS, XA_STRING, 8,
                    PropModeReplace, (unsigned char*)"srwm\0srwm\0", 10);

    XSetSelectionOwner(dpy, wm_sn, wm_sn_owner, CurrentTime);
    XFlush(dpy);

    if (owner != None) {
        XSetErrorHandler(wm_sn_xerror);
        int check_rounds = 150;
        XWindowAttributes attr;
        do {
            usleep(100000);
            if (check_rounds-- == 0) {
                fprintf(stderr, "srwm: the old window manager is not exiting\n");
                XDestroyWindow(dpy, wm_sn_owner);
                wm_sn_owner = None;
                XSetErrorHandler(xerror);
                return 1;
            }
        } while (XGetWindowAttributes(dpy, owner, &attr));
        XSetErrorHandler(xerror);
    }

    XEvent event;
    memset(&event, 0, sizeof(event));
    event.type = ClientMessage;
    event.xclient.window = screen_ptr->root;
    event.xclient.message_type = XInternAtom(dpy, "MANAGER", False);
    event.xclient.format = 32;
    event.xclient.data.l[0] = CurrentTime;
    event.xclient.data.l[1] = wm_sn;
    event.xclient.data.l[2] = wm_sn_owner;

    XSendEvent(dpy, screen_ptr->root, False, StructureNotifyMask, &event);
    XSync(dpy, False);

    return 0;
}

void srwm_wm_sn_handle_selection_clear(XEvent *e) {
    if (e->xselectionclear.selection == wm_sn) {
        running = 0;
    }
}

void srwm_wm_sn_cleanup(void) {
    if (wm_sn_owner != None) {
        XDestroyWindow(dpy, wm_sn_owner);
        wm_sn_owner = None;
    }
}
