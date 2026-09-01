// SPDX-License-Identifier: GPL-2.0-only
/*
 * TS3A227E Autonous Audio Accessory Detection and Configureation Switch
 *
 * Copyright (C) 2014 Google, Inc.
 */

unsafe extern "C" {
    pub fn ts3a227e_enable_jack_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
