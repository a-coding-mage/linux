// SPDX-License-Identifier: GPL-2.0-only
/*
 * Smart reflex Class 3 specific implementations
 *
 * Author: Thara Gopinath       <thara@ti.com>
 *
 * Copyright (C) 2010 Texas Instruments, Inc.
 * Thara Gopinath <thara@ti.com>
 */

// Dependencies supplied by <linux/power/smartreflex.h>, "soc.h", and
// "voltage.h" are intentionally left as external interfaces.

extern "C" {
    fn voltdm_get_voltage(voltdm: *mut voltage_domain) -> ::core::ffi::c_ulong;
    fn pr_warn(fmt: *const ::core::ffi::c_char, ...);
    fn omap_vp_enable(voltdm: *mut voltage_domain);
    fn sr_enable(sr: *mut omap_sr, volt: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn sr_disable_errgen(sr: *mut omap_sr);
    fn omap_vp_disable(voltdm: *mut voltage_domain);
    fn sr_disable(sr: *mut omap_sr);
    fn voltdm_reset(voltdm: *mut voltage_domain);
    fn sr_configure_errgen(sr: *mut omap_sr) -> ::core::ffi::c_int;
    fn pr_info(fmt: *const ::core::ffi::c_char, ...);
    fn sr_register_class(data: *mut omap_sr_class_data) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct voltage_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct omap_sr {
    pub voltdm: *mut voltage_domain,
    pub name: *const ::core::ffi::c_char,
}

pub type SrClassCallback = unsafe extern "C" fn(*mut omap_sr) -> ::core::ffi::c_int;
pub type SrClassDisableCallback =
    unsafe extern "C" fn(*mut omap_sr, ::core::ffi::c_int) -> ::core::ffi::c_int;

#[repr(C)]
pub struct omap_sr_class_data {
    pub enable: Option<SrClassCallback>,
    pub disable: Option<SrClassDisableCallback>,
    pub configure: Option<SrClassCallback>,
    pub class_type: ::core::ffi::c_int,
}

// These values are provided by the corresponding kernel headers.
extern "C" {
    static ENODATA: ::core::ffi::c_int;
}

unsafe extern "C" fn sr_class3_enable(sr: *mut omap_sr) -> ::core::ffi::c_int {
    let volt: ::core::ffi::c_ulong = voltdm_get_voltage((*sr).voltdm);

    if volt == 0 {
        pr_warn(
            b"%s: Curr voltage unknown. Cannot enable %s\n\0".as_ptr() as *const _,
            b"sr_class3_enable\0".as_ptr(),
            (*sr).name,
        );
        return -ENODATA;
    }

    omap_vp_enable((*sr).voltdm);
    sr_enable(sr, volt)
}

unsafe extern "C" fn sr_class3_disable(
    sr: *mut omap_sr,
    is_volt_reset: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    sr_disable_errgen(sr);
    omap_vp_disable((*sr).voltdm);
    sr_disable(sr);
    if is_volt_reset != 0 {
        voltdm_reset((*sr).voltdm);
    }

    0
}

unsafe extern "C" fn sr_class3_configure(sr: *mut omap_sr) -> ::core::ffi::c_int {
    sr_configure_errgen(sr)
}

/* SR class3 structure */
static mut class3_data: omap_sr_class_data = omap_sr_class_data {
    enable: Some(sr_class3_enable),
    disable: Some(sr_class3_disable),
    configure: Some(sr_class3_configure),
    class_type: SR_CLASS3,
};

// Smartreflex Class3 init API to be called from board file
unsafe extern "C" fn sr_class3_init() -> ::core::ffi::c_int {
    pr_info(b"SmartReflex Class3 initialized\n\0".as_ptr() as *const _);
    sr_register_class(&raw mut class3_data)
}

// omap_late_initcall(sr_class3_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
