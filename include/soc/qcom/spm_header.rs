/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2011-2014, The Linux Foundation. All rights reserved.
 * Copyright (c) 2014,2015, Linaro Ltd.
 */

#[repr(C)]
pub enum pm_sleep_mode {
    PM_SLEEP_MODE_STBY,
    PM_SLEEP_MODE_RET,
    PM_SLEEP_MODE_SPC,
    PM_SLEEP_MODE_PC,
    PM_SLEEP_MODE_NR,
}

#[repr(C)]
pub struct spm_driver_data {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn spm_set_low_power_mode(drv: *mut spm_driver_data, mode: pm_sleep_mode);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
