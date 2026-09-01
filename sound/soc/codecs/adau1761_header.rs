// SPDX-License-Identifier: GPL-2.0-only
/*
 * ADAU1361/ADAU1461/ADAU1761/ADAU1961 driver
 *
 * Copyright 2014 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// C dependencies:
// #include <linux/regmap.h>
// #include "adau17x1.h"

use crate::{adau17x1_type, device, regmap, regmap_config};

unsafe extern "C" {
    pub fn adau1761_probe(
        dev: *mut device,
        regmap: *mut regmap,
        type_: adau17x1_type,
        switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>,
    ) -> ::core::ffi::c_int;

    pub static adau1761_regmap_config: regmap_config;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
