/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Apple Onboard Audio Alsa private helpers
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

// C dependency: #include "../aoa.h"

unsafe extern "C" {
    pub fn aoa_alsa_init(
        name: *mut core::ffi::c_char,
        mod_: *mut module,
        dev: *mut device,
    ) -> core::ffi::c_int;
    pub fn aoa_alsa_cleanup();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
