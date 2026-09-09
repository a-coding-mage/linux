/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2013 MundoReader S.L.
 * Author: Heiko Stuebner <heiko@sntech.de>
 */

unsafe extern "C" {
    pub static mut rockchip_secondary_trampoline: core::ffi::c_char;
    pub static mut rockchip_secondary_trampoline_end: core::ffi::c_char;

    pub static mut rockchip_boot_fn: core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
