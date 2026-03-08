/* bridge.c — Thin lifecycle wrapper around dwm.c for Go */
#include "bridge.h"

#include <X11/Xlib.h>
#include <locale.h>
#include <stdio.h>
#include <string.h>

/* Internal flag: distinguishes restart (running=0) from quit */
static int restart_requested = 0;

int srwm_init(void) {
  /* Reset for re-init on restart */
  running = 1;
  restart_requested = 0;

  if (!setlocale(LC_CTYPE, "") || !XSupportsLocale())
    fputs("warning: no locale support\n", stderr);

  if (!(dpy = XOpenDisplay(NULL))) {
    fputs("srwm: cannot open display\n", stderr);
    return -1;
  }

  checkotherwm();
  setup();
  scan();
  return 0;
}

void srwm_run(void) {
  run(); /* blocks until running == 0 */
}

void srwm_cleanup(void) {
  cleanup();
  XCloseDisplay(dpy);
  dpy = NULL;
}

void srwm_quit(void) {
  restart_requested = 0;
  running = 0;
}

int srwm_should_restart(void) {
  return restart_requested;
}

void srwm_set_status(const char *text) {
  if (!dpy) return;
  XStoreName(dpy, DefaultRootWindow(dpy), text);
  XFlush(dpy);
}

void srwm_request_restart(void) {
  restart_requested = 1;
  running = 0;
}
