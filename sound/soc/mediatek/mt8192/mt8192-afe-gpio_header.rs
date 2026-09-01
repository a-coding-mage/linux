/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt8192-afe-gpio.h  --  Mediatek 8192 afe gpio ctrl definition
 *
 * Copyright (c) 2020 MediaTek Inc.
 * Author: Shane Chien <shane.chien@mediatek.com>
 */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mt8192_afe_gpio_init(dev: *mut device) -> ::core::ffi::c_int;

    pub fn mt8192_afe_gpio_request(
        dev: *mut device,
        enable: bool,
        dai: ::core::ffi::c_int,
        uplink: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
