/* SPDX-License-Identifier: GPL-2.0 */

// C header guard removed: _CS4271_PRIV_H.
// C dependency: #include <linux/regmap.h>

use core::ffi::c_int;

use crate::{device, of_device_id, regmap, regmap_config};

unsafe extern "C" {
    pub static cs4271_dt_ids: [of_device_id; 0];
    pub static cs4271_regmap_config: regmap_config;

    pub fn cs4271_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
