local M = {}

M.theme = {
	purple = "#bebeda",
	darkpurple = "#aeaed1",
	black = "#252530",
	green = "#99b782",
	darkgreen = "#7fa563",
	white = "#d7d7d7",
	grey = "#606079",
	blue = "#8ba9c1",
	darkblue = "#6e94b2",
	red = "#e08398",
	darkred = "#d8647e",
	yellow = "#f5cb96",
	darkyellow = "#f3be7c",
}

-- helper to read the first line of a file/command
function M.read_first_line(cmd)
	local f = io.popen(cmd)
	if not f then
		return ""
	end
	local line = f:read("*l") or ""
	f:close()
	return line
end

-- helper to read all output
function M.read_all(cmd)
	local f = io.popen(cmd)
	if not f then
		return ""
	end
	local txt = f:read("*a") or ""
	f:close()
	return txt
end

local function battery()
	local cap = M.read_first_line("cat /sys/class/power_supply/BAT0/capacity 2>/dev/null")
	if cap == "" then
		return ""
	end
	return string.format("^c%s^ ^b%s^ 󱐋 ^b%s^ %s ", M.theme.black, M.theme.yellow, M.theme.darkyellow, cap)
end

local function brightness()
	local b = M.read_first_line("cat /sys/class/backlight/*/brightness 2>/dev/null")
	if b == "" then
		return ""
	end
	local pct = tonumber(b)
	if not pct then
		pct = 0
	end
	return string.format("^c%s^^b%s^   ^b%s^ %d%%", M.theme.black, M.theme.red, M.theme.darkred, pct)
end

local function volume()
	local wp_info = M.read_first_line("wpctl get-volume @DEFAULT_AUDIO_SINK@ 2>/dev/null")
	if wp_info == "" then
		return string.format("^c%s^ ^b%s^ 󰝟 Error", M.theme.black, M.theme.red)
	end

	local vol_raw = string.match(wp_info, "Volume:%s*(%d%.%d+)")
	if not vol_raw then
		vol_raw = "0"
	end
	local vol_int = math.floor(tonumber(vol_raw) * 100)

	local is_muted = string.match(wp_info, "%[MUTED%]")
	if is_muted then
		return string.format("^c%s^ ^b%s^ 󰝟 Muted", M.theme.black, M.theme.red)
	end

	local icon = "󰕾"
	if vol_int == 0 then
		icon = "󰝟"
	elseif vol_int <= 33 then
		icon = "󰕿"
	elseif vol_int <= 66 then
		icon = "󰖀"
	end

	return string.format("^c%s^ ^b%s^ %s ^b%s^ %d%%", M.theme.black, M.theme.green, icon, M.theme.darkgreen, vol_int)
end

local function wlan()
	-- Get interface name
	local iface = M.read_first_line("find /sys/class/net -name 'wl*' -type l | head -1 | xargs basename 2>/dev/null")
	if iface == "" then
		return ""
	end

	local state = M.read_first_line("cat /sys/class/net/" .. iface .. "/operstate 2>/dev/null")
	if state == "down" or state == "" then
		return string.format("^c%s^ ^b%s^ 󰤭  ^b%s^ Disconnected", M.theme.black, M.theme.blue, M.theme.darkblue)
	end

	local iw_info = M.read_all("iw dev " .. iface .. " info 2>/dev/null")
	local ssid = string.match(iw_info, "ssid ([^\n]+)")
	if not ssid then
		ssid = "Unknown"
	end

	return string.format("^c%s^ ^b%s^ 󰤨  ^b%s^ %s", M.theme.black, M.theme.blue, M.theme.darkblue, ssid)
end

local function clock()
	local t = os.date("%H:%M:%S")
	return string.format("^c%s^ ^b%s^ 󱑆  ^b%s^ %s", M.theme.black, M.theme.purple, M.theme.darkpurple, t)
end

local function gap()
	return string.format("^c%s^ ^b%s^", M.theme.black, M.theme.black)
end

function M.refresh()
	local parts = {
		brightness(),
		gap(),
		volume(),
		gap(),
		wlan(),
		gap(),
		clock(),
		gap(),
		battery(),
	}

	-- filter empty strings
	local text = ""
	for _, p in ipairs(parts) do
		text = text .. p
	end

	srwm.set_status(text)
end

function M.run()
	while true do
		M.refresh()
		srwm.sleep(1)
	end
end

return M
