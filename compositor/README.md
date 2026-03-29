# srcom

**srcom** is a compositor for X11, designed as the companion compositor for [srwm](https://github.com/infraflakes/srwm). It is a fork of [picom](https://github.com/yshui/picom) v13 with added support for srwm's true zoom feature via software cursor rendering and X atom-based IPC.

## Features

- All picom v13 features: animations, blur, shadows, rounded corners, transparency, window rules
- **srwm true zoom**: Visual zoom of the entire desktop canvas with accurate software cursor tracking
- Software cursor rendering during zoom for accurate visual-to-input mapping

## Installation

### Download Binary (Recommended)

1. Download the latest release from the [Releases](https://github.com/infraflakes/picom/releases) page.
2. Make it executable and move it to your path:
   ```bash
   chmod +x srcom-v*-linux-amd64
   sudo mv srcom-v*-linux-amd64 /usr/local/bin/srcom
   ```

### Building from Source

#### Dependencies

Assuming you have the usual build tools (gcc, meson, ninja, pkg-config):

- libx11, libx11-xcb, libxcb and xcb-util libraries
- libGL, libEGL, libepoxy
- libev, libconfig (>= 1.7), libpcre2, pixman, uthash

On Arch Linux:
```bash
sudo pacman -S --needed meson ninja gcc libev libconfig pcre2 libx11 libxcb xcb-util xcb-util-image xcb-util-renderutil pixman libepoxy libxcomposite libxdamage libxfixes libxext xcb-proto xorgproto uthash cmake libev pkg-config base-devel
```

On Debian/Ubuntu:
```bash
sudo apt-get install -y libconfig-dev libegl-dev libev-dev libgl-dev libepoxy-dev libpcre2-dev libpixman-1-dev libx11-xcb-dev libxcb1-dev libxcb-composite0-dev libxcb-damage0-dev libxcb-glx0-dev libxcb-image0-dev libxcb-present-dev libxcb-randr0-dev libxcb-render0-dev libxcb-render-util0-dev libxcb-shape0-dev libxcb-util-dev libxcb-xfixes0-dev meson ninja-build uthash-dev
```

#### Build

```bash
meson setup --buildtype=release build -Ddbus=false
ninja -C build
```

The built binary is at `build/src/srcom`.

#### Install

```bash
ninja -C build install
```

Default prefix is `/usr/local`. Change with `meson configure -Dprefix=<path> build`.

## Usage

### With srwm

1. Start srwm
2. Start srcom with your config:
   ```bash
   srcom --config ~/.config/picom/picom.conf --backend egl
   ```
3. Enter canvas mode in srwm and use zoom keybindings -- srcom handles the visual zoom and software cursor automatically

### Standalone

srcom works as a drop-in replacement for picom with any X11 window manager. All picom v13 configuration options are supported. See the [picom wiki](https://github.com/yshui/picom/wiki) for configuration documentation.

## Acknowledgements

- [yshui](https://github.com/yshui) and all contributors to [picom](https://github.com/yshui/picom)
- The original [Compton](https://github.com/chjj/compton/) project

## Licensing

srcom is free software, made available under the [MIT](LICENSES/MIT) and [MPL-2.0](LICENSES/MPL-2.0) software licenses. See the individual source files for details.
