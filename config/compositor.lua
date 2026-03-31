-- srwm's compositor config carries characteristics of picom

local comp = srwm.compositor

comp.enable()

comp.vsync(true)

comp.border_blur({
	enable = true,
	dim = 0.3,
})

comp.shadow({
	enable = true,
	radius = 30,
	opacity = 0.65,
	offset_x = -30,
	offset_y = -30,
	color = "#000000",
})

comp.fade({
	enable = true,
	step_in = 0.5,
	step_out = 1,
	delta = 10,
})

comp.corner_radius(10)

comp.blur({
	method = "dual_kawase", -- or "none", "box", "gaussian", "kernel"
	strength = 10,
})

comp.animate("open", {
	preset = "zoom",
	duration = 0.85,
	scale = 0.5,
	curve = "cubic-bezier(0.22,1.3,0.36,1)",
})

comp.animate("close", {
	preset = "zoom",
	duration = 0.4,
	scale = 0.9,
	curve = "cubic-bezier(0.22,0.9,0.36,1)",
})

comp.animate("geometry", {
	duration = 0.7,
	curve = "cubic-bezier(0.22,1.3,0.36,1)",
})

comp.rule("fullscreen", {
	corner_radius = 0,
})

comp.rule("window_type = 'dock'", {
	corner_radius = 0,
	shadow = false,
})

-- comp.rule("class_g = 'dmenu'", {
-- 	corner_radius = 0,
-- 	shadow = false,
-- 	blur = false,
-- 	animate_open = { preset = "slide-in", direction = "up", duration = 0.2 },
-- 	animate_close = { preset = "slide-out", direction = "up", duration = 0.1 },
-- })

comp.rule("window_type = 'srwm'", {
	shadow = false,
	blur = false,
	corner_radius = 0,
})
