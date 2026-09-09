/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arch/powerpc/platforms/embedded6xx/usbgecko_udbg.h
 *
 * udbg serial input/output routines for the USB Gecko adapter.
 * Copyright (C) 2008-2009 The GameCube Linux Team
 * Copyright (C) 2008,2009 Albert Herranz
 */

// The C header guard `__USBGECKO_UDBG_H` has no direct Rust equivalent.

// CONFIG_USBGECKO_UDBG is a build-time C configuration condition.  The
// corresponding Rust configuration is represented here by the feature of
// the same name.
#[cfg(feature = "CONFIG_USBGECKO_UDBG")]
extern "C" {
    pub fn ug_udbg_init();
}

#[cfg(not(feature = "CONFIG_USBGECKO_UDBG"))]
#[inline]
pub unsafe fn ug_udbg_init() {
}

extern "C" {
    pub fn udbg_init_usbgecko();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
