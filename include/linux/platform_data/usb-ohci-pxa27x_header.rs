/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. The original header guard was omitted.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pxaohci_platform_data {
    pub init: Option<unsafe extern "C" fn(*mut device) -> ::core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut device)>,

    pub flags: ::core::ffi::c_ulong,
    pub power_on_delay: ::core::ffi::c_int, /* Power On to Power Good time - in ms
                                             * HCD must wait for this duration before
                                             * accessing a powered on port
                                             */
    pub port_mode: ::core::ffi::c_int,
    pub power_budget: ::core::ffi::c_int,
}

pub const ENABLE_PORT1: ::core::ffi::c_int = 1 << 0;
pub const ENABLE_PORT2: ::core::ffi::c_int = 1 << 1;
pub const ENABLE_PORT3: ::core::ffi::c_int = 1 << 2;
pub const ENABLE_PORT_ALL: ::core::ffi::c_int = ENABLE_PORT1 | ENABLE_PORT2 | ENABLE_PORT3;

pub const POWER_SENSE_LOW: ::core::ffi::c_int = 1 << 3;
pub const POWER_CONTROL_LOW: ::core::ffi::c_int = 1 << 4;
pub const NO_OC_PROTECTION: ::core::ffi::c_int = 1 << 5;
pub const OC_MODE_GLOBAL: ::core::ffi::c_int = 0 << 6;
pub const OC_MODE_PERPORT: ::core::ffi::c_int = 1 << 6;

pub const PMM_NPS_MODE: ::core::ffi::c_int = 1;
pub const PMM_GLOBAL_MODE: ::core::ffi::c_int = 2;
pub const PMM_PERPORT_MODE: ::core::ffi::c_int = 3;

unsafe extern "C" {
    pub fn pxa_set_ohci_info(info: *mut pxaohci_platform_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
