/* SPDX-License-Identifier: GPL-2.0
 *
 * mt6833-afe-gpio.h  --  Mediatek 6833 afe gpio ctrl definition
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Jiaxin Yu <jiaxin.yu@mediatek.com>
 */

#[repr(C)]
pub struct mtk_base_afe {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn mt8186_afe_gpio_init(dev: *mut device) -> ::std::os::raw::c_int;

    pub fn mt8186_afe_gpio_request(
        dev: *mut device,
        enable: bool,
        dai: ::std::os::raw::c_int,
        uplink: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
