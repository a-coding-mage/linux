/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Functions to access Menelaus power management chip
 */

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct menelaus_platform_data {
    pub late_init: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
}

unsafe extern "C" {
    pub fn menelaus_register_mmc_callback(
        callback: Option<unsafe extern "C" fn(data: *mut c_void, card_mask: u8)>,
        data: *mut c_void,
    ) -> c_int;
    pub fn menelaus_unregister_mmc_callback();
    pub fn menelaus_set_mmc_opendrain(slot: c_int, enable: c_int) -> c_int;
    pub fn menelaus_set_mmc_slot(
        slot: c_int,
        enable: c_int,
        power: c_int,
        cd_on: c_int,
    ) -> c_int;

    pub fn menelaus_set_vmem(mv: c_uint) -> c_int;
    pub fn menelaus_set_vio(mv: c_uint) -> c_int;
    pub fn menelaus_set_vmmc(mv: c_uint) -> c_int;
    pub fn menelaus_set_vaux(mv: c_uint) -> c_int;
    pub fn menelaus_set_vdcdc(dcdc: c_int, mv: c_uint) -> c_int;
    pub fn menelaus_set_slot_sel(enable: c_int) -> c_int;
    pub fn menelaus_get_slot_pin_states() -> c_int;
    pub fn menelaus_set_vcore_hw(roof_mv: c_uint, floor_mv: c_uint) -> c_int;

    pub fn menelaus_set_regulator_sleep(enable: c_int, val: u32) -> c_int;
}

pub const EN_VPLL_SLEEP: c_int = 1 << 7;
pub const EN_VMMC_SLEEP: c_int = 1 << 6;
pub const EN_VAUX_SLEEP: c_int = 1 << 5;
pub const EN_VIO_SLEEP: c_int = 1 << 4;
pub const EN_VMEM_SLEEP: c_int = 1 << 3;
pub const EN_DC3_SLEEP: c_int = 1 << 2;
pub const EN_DC2_SLEEP: c_int = 1 << 1;
pub const EN_VC_SLEEP: c_int = 1 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
