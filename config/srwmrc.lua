--------------------------------------------------------------------------------
-- Configuration
--------------------------------------------------------------------------------
-- Note: These take effect on restart (srwm.restart())

-- Modular srwm configuration entry point
-- Each module is located in ~/.config/srwm/<name>.lua

include("keybindings") -- Keyboard shortcuts
include("canvas") -- Canvas layout
include("compositor") -- Compositor config

-- srwm.spawn_once("picom -b")
-- srwm.spawn_once("feh --bg-scale ~/wallpaper.jpg")
srwm.spawn_once("xset r rate 150 50 &")

srwm.workspaces.set_label("1,2,3,4,5,6") -- set workspaces name
srwm.canvas.edge_autopan(true) -- move the canvas when hovering cursor over border

-- Appearance
srwm.cfg.borderpx(4)

-- Bar dimensions
srwm.bar.fonts("Terminess Nerd Font:size=16")
srwm.bar.enable(true, "polybar")
srwm.bar.tab.enable(false)
srwm.bar.tab.top(false)
srwm.bar.tab.height(35)
srwm.bar.tab.tile_vertical_padding(15)
srwm.bar.tab.tile_inner_padding_horizontal(15)
srwm.bar.tab.tile_outer_padding_horizontal(15)

-- Window border colors
srwm.window.border.active("#6e94b2")
srwm.window.border.inactive("#606079")

-- Tab bar theme
srwm.bar.theme({
	tab_selected = { "#252530", "#aeaed1" },
	tab_normal = { "#cdcdcd", "#252530" },
	button_prev = "#7fa563",
	button_next = "#f3be7c",
	button_close = "#d8647e",
})

-- Environment variables (inherited by all spawned processes)

-- srwm.env("XCURSOR_SIZE", "24")
-- srwm.env("XCURSOR_THEME", "Adwaita")
-- srwm.env("QT_QPA_PLATFORMTHEME", "qt6ct")
srwm.env("GTK_IM_MODULE", "fcitx5")
srwm.env("QT_IM_MODULE", "fcitx5")
srwm.env("XMODIFIERS", "@im=fcitx5")
