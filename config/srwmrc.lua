--------------------------------------------------------------------------------
-- Configuration
--------------------------------------------------------------------------------
-- Note: These take effect on restart (srwm.restart())

-- Modular srwm configuration entry point
-- Each module is located in ~/.config/srwm/<name>.lua

include("env") -- Environment Variables
include("bar") -- Bar settings
include("keybindings") -- Keyboard shortcuts
include("theming") -- Palette, colors, widgets, bar layout
include("startup") -- Autostart applications
include("canvas") -- Canvas layout
include("compositor") -- Compositor

-- Define workspace names
srwm.workspaces.set_label("1,2,3")

-- Appearance
srwm.cfg.borderpx(0)
