/* bridge.h — Go ↔ C lifecycle interface for swm */
#ifndef SWM_BRIDGE_H
#define SWM_BRIDGE_H

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
int  swm_init(void);
void swm_run(void);
void swm_cleanup(void);
void swm_quit(void);
int  swm_should_restart(void);

/* Status bar — Go sets the root window name */
void swm_set_status(const char *text);

/* Called from dwm.c restart() to distinguish restart from quit */
void swm_request_restart(void);

#endif /* SWM_BRIDGE_H */
