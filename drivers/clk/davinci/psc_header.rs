/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Clock driver for TI Davinci PSC controllers
 *
 * Copyright (C) 2018 David Lechner <david@lechnology.com>
 */

// Translated from the C header. Kernel-provided types and symbols are external
// dependencies of this translation.

/* PSC quirk flags */
pub const LPSC_ALWAYS_ENABLED: u32 = 1u32 << 0; // never disable this clock
pub const LPSC_SET_RATE_PARENT: u32 = 1u32 << 1; // propagate set_rate to parent clock
pub const LPSC_FORCE: u32 = 1u32 << 2; // requires MDCTL FORCE bit
pub const LPSC_LOCAL_RESET: u32 = 1u32 << 3; // acts as reset provider

#[repr(C)]
pub struct davinci_lpsc_clkdev_info {
    pub con_id: *const core::ffi::c_char,
    pub dev_id: *const core::ffi::c_char,
}

#[macro_export]
macro_rules! LPSC_CLKDEV {
    ($c:expr, $d:expr) => {
        $crate::davinci_lpsc_clkdev_info {
            con_id: $c,
            dev_id: $d,
        }
    };
}

#[macro_export]
macro_rules! LPSC_CLKDEV1 {
    ($n:ident, $c:expr, $d:expr) => {
        static $n: [$crate::davinci_lpsc_clkdev_info; 2] = [
            $crate::LPSC_CLKDEV!($c, $d),
            $crate::davinci_lpsc_clkdev_info {
                con_id: core::ptr::null(),
                dev_id: core::ptr::null(),
            },
        ];
    };
}

#[macro_export]
macro_rules! LPSC_CLKDEV2 {
    ($n:ident, $c1:expr, $d1:expr, $c2:expr, $d2:expr) => {
        static $n: [$crate::davinci_lpsc_clkdev_info; 3] = [
            $crate::LPSC_CLKDEV!($c1, $d1),
            $crate::LPSC_CLKDEV!($c2, $d2),
            $crate::davinci_lpsc_clkdev_info {
                con_id: core::ptr::null(),
                dev_id: core::ptr::null(),
            },
        ];
    };
}

#[macro_export]
macro_rules! LPSC_CLKDEV3 {
    ($n:ident, $c1:expr, $d1:expr, $c2:expr, $d2:expr, $c3:expr, $d3:expr) => {
        static $n: [$crate::davinci_lpsc_clkdev_info; 4] = [
            $crate::LPSC_CLKDEV!($c1, $d1),
            $crate::LPSC_CLKDEV!($c2, $d2),
            $crate::LPSC_CLKDEV!($c3, $d3),
            $crate::davinci_lpsc_clkdev_info {
                con_id: core::ptr::null(),
                dev_id: core::ptr::null(),
            },
        ];
    };
}

/**
 * davinci_lpsc_clk_info - LPSC module-specific clock information
 * @name: the clock name
 * @parent: the parent clock name
 * @cdevs: optional array of clkdev lookup table info
 * @md: the local module domain (LPSC id)
 * @pd: the power domain id
 * @flags: bitmask of LPSC_* flags
 */
#[repr(C)]
pub struct davinci_lpsc_clk_info {
    pub name: *const core::ffi::c_char,
    pub parent: *const core::ffi::c_char,
    pub cdevs: *const davinci_lpsc_clkdev_info,
    pub md: u32,
    pub pd: u32,
    pub flags: usize,
}

#[macro_export]
macro_rules! LPSC {
    ($m:expr, $d:expr, $n:ident, $p:ident, $c:expr, $f:expr) => {
        $crate::davinci_lpsc_clk_info {
            name: concat!(stringify!($n), "\0").as_ptr() as *const core::ffi::c_char,
            parent: concat!(stringify!($p), "\0").as_ptr() as *const core::ffi::c_char,
            cdevs: $c,
            md: $m,
            pd: $d,
            flags: $f,
        }
    };
}

extern "C" {
    pub fn davinci_psc_register_clocks(
        dev: *mut device,
        info: *const davinci_lpsc_clk_info,
        num_clks: u8,
        base: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;

    pub fn of_davinci_psc_clk_init(
        dev: *mut device,
        info: *const davinci_lpsc_clk_info,
        num_clks: u8,
        base: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}

/* Device-specific data */

#[repr(C)]
pub struct davinci_psc_init_data {
    pub parent_clks: *mut clk_bulk_data,
    pub num_parent_clks: core::ffi::c_int,
    pub psc_init: Option<unsafe extern "C" fn(*mut device, *mut core::ffi::c_void) -> core::ffi::c_int>,
}

extern "C" {
    pub static da850_psc0_init_data: davinci_psc_init_data;
    pub static da850_psc1_init_data: davinci_psc_init_data;
    pub static of_da850_psc0_init_data: davinci_psc_init_data;
    pub static of_da850_psc1_init_data: davinci_psc_init_data;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
