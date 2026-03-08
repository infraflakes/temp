/* bridge.h — Go ↔ C lifecycle interface for srwm */
#ifndef SRWM_BRIDGE_H
#define SRWM_BRIDGE_H

#include <X11/Xlib.h>

/* These are defined in dwm.c (non-static) */
extern int running;
extern Display* dpy;

extern void checkotherwm(void);
extern void setup(void);
extern void scan(void);
extern void run(void);
extern void cleanup(void);

/* Bridge lifecycle — called by Go */
int  srwm_init(void);
void srwm_run(void);
void srwm_cleanup(void);
void srwm_quit(void);
int  srwm_should_restart(void);

/* Status bar — Go sets the root window name */
void srwm_set_status(const char *text);

/* Called from dwm.c restart() to distinguish restart from quit */
void srwm_request_restart(void);

#endif /* SRWM_BRIDGE_H */
