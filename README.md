# Serein Window Manager
A floating, zooming X11 window manager. Built with Rust, C and Lua.

![Preview](./assets/screenshot_default.png)

## Installation

### Download Binary (Recommended)
Since `srwm` is built with most dependencies statically linked, you can run it on almost any Linux distribution.

1. Download the latest release from the [Releases](https://github.com/infraflakes/srwm/releases) page.
2. Make it executable and move it to your path:
   ```bash
   chmod +x srwm-v*-linux-amd64
   sudo mv srwm-v*-linux-amd64 /usr/local/bin/srwm
   ```

### Getting started

> [!CAUTION]
> You should modify the font config at ~/.config/srwm/srwmrc.lua
> And keybindings at ~/.config/srwm/keybindings.lua before starting
> The default tab bar uses nerd glyph to display control buttons, you may want to install it

You can generate the default config with `srwm kickstart`:

```bash
srwm kickstart
```

Then start the window manager with `srwm start` (make sure you have xorg-xauth utility installed):
```bash
srwm start
```

srwm by default only shrinks or expand windows to mimick zoom because it requires a compositor, for true zooming visuals you can use [srcom](https://github.com/infraflakes/srcom/releases/).

### Building from scratch

Current the project only supports building with [dagger](https://dagger.io/).

```bash
make build
```

Or for plain output:

```bash
make build-verbose
```

## Acknowledgements

Special thanks to:

- The developers who worked on dwm to make it possible.

- [Siduck](https://github.com/siduck) and other contributors for chadwm which `srwm` originally based on.

- [wh1tepearl](https://codeberg.org/wh1tepearl) for the canvas layout `srwm` based on.
---
