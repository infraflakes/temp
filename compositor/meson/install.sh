#!/bin/sh

if [ ! -e "${MESON_INSTALL_DESTDIR_PREFIX}/bin/compton" ]; then
	echo "Linking srcom to ${MESON_INSTALL_DESTDIR_PREFIX}/bin/compton"
	ln -s srcom "${MESON_INSTALL_DESTDIR_PREFIX}/bin/compton"
fi

if [ ! -e "${MESON_INSTALL_DESTDIR_PREFIX}/bin/compton-trans" ]; then
	echo "Linking srcom-trans to ${MESON_INSTALL_DESTDIR_PREFIX}/bin/compton-trans"
	ln -s srcom-trans "${MESON_INSTALL_DESTDIR_PREFIX}/bin/compton-trans"
fi
