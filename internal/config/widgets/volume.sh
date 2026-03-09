#!/bin/sh
# srwm widget: volume

wp_info=$(wpctl get-volume @DEFAULT_AUDIO_SINK@ 2>/dev/null)
[ $? -ne 0 ] && printf "^c{black}^ ^b{red}^ 󰝟 Error" && exit 0

vol_raw=$(echo "$wp_info" | awk '{print $2}')
vol_int=$(echo "$vol_raw * 100 / 1" | bc)

if echo "$wp_info" | grep -q "\[MUTED\]"; then
	printf "^c{black}^ ^b{red}^ 󰝟 Muted"
elif [ "$vol_int" -eq 0 ]; then
	printf "^c{black}^ ^b{green}^ 󰝟 ^b{darkgreen}^ %s%%" "$vol_int"
elif [ "$vol_int" -le 33 ]; then
	printf "^c{black}^ ^b{green}^ 󰕿 ^b{darkgreen}^ %s%%" "$vol_int"
elif [ "$vol_int" -le 66 ]; then
	printf "^c{black}^ ^b{green}^ 󰖀 ^b{darkgreen}^ %s%%" "$vol_int"
else
	printf "^c{black}^ ^b{green}^ 󰕾 ^b{darkgreen}^ %s%%" "$vol_int"
fi
