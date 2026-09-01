// SPDX-License-Identifier: GPL-2.0
//
// CS35L41 ALSA HDA Property driver
//
// Copyright 2023 Cirrus Logic, Inc.
//
// Author: Stefan Binding <sbinding@opensource.cirrus.com>

// C header dependencies:
// #include <linux/device.h>
// #include "cs35l41_hda.h"

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    pub fn cs35l41_add_dsd_properties(
        cs35l41: *mut cs35l41_hda,
        physdev: *mut device,
        id: c_int,
        hid: *const c_char,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
