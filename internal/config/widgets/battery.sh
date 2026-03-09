#!/bin/sh
# srwm widget: battery

cap=$(cat /sys/class/power_supply/BAT0/capacity 2>/dev/null)
[ -z "$cap" ] && exit 0

printf "^c{black}^ ^b{yellow}^ 󱐋 ^b{darkyellow}^ %s " "$cap"
