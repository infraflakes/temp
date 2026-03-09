#!/bin/sh
# srwm widget: wifi

iface=$(find /sys/class/net -name 'wl*' -type l 2>/dev/null | head -1 | xargs basename 2>/dev/null)
[ -z "$iface" ] && exit 0

state=$(cat /sys/class/net/"$iface"/operstate 2>/dev/null)

case "$state" in
up)
	ssid=$(iw dev "$iface" info 2>/dev/null | grep -Po '(?<=ssid ).*')
	printf "^c{black}^ ^b{blue}^ 󰤨  ^b{darkblue}^ %s" "${ssid:-Unknown}"
	;;
down)
	printf "^c{black}^ ^b{blue}^ 󰤭  ^b{darkblue}^ Disconnected"
	;;
esac
