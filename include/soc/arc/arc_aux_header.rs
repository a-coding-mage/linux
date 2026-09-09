/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016-2017 Synopsys, Inc. (www.synopsys.com)
 */

/* CONFIG_ARC selects the ARC auxiliary-register builtins. */
#[cfg(feature = "CONFIG_ARC")]
#[allow(non_snake_case)]
unsafe extern "C" {
    fn __builtin_arc_lr(r: u32) -> u32;
    fn __builtin_arc_sr(v: u32, r: u32);
}

#[cfg(feature = "CONFIG_ARC")]
#[inline]
pub unsafe fn read_aux_reg(r: u32) -> u32 {
    __builtin_arc_lr(r)
}

#[cfg(feature = "CONFIG_ARC")]
#[inline]
pub unsafe fn write_aux_reg(r: u32, v: u32) {
    __builtin_arc_sr(v as u32, r);
}

#[cfg(not(feature = "CONFIG_ARC"))]
#[inline]
pub unsafe fn read_aux_reg(_r: u32) -> i32 {
    0
}

/*
 * Function helps elide unused variable warning.
 * See: https://lists.infradead.org/pipermail/linux-snps-arc/2016-November/001748.html
 */
#[cfg(not(feature = "CONFIG_ARC"))]
#[inline]
pub unsafe fn write_aux_reg(_r: u32, _v: u32) {}

unsafe extern "C" {
    fn bogus_undefined();
}

#[macro_export]
macro_rules! READ_BCR {
    ($reg:expr, $into:expr) => {{
        let tmp: u32 = unsafe { $crate::read_aux_reg($reg) as u32 };
        if core::mem::size_of_val(&$into) == core::mem::size_of::<u32>() {
            unsafe {
                core::ptr::write(
                    (&mut $into as *mut _).cast::<u8>() as *mut _,
                    core::ptr::read((&tmp as *const u32).cast::<u8>() as *const _),
                );
            }
        } else {
            unsafe { $crate::bogus_undefined() };
        }
    }};
}

#[macro_export]
macro_rules! WRITE_AUX {
    ($reg:expr, $into:expr) => {{
        let tmp: u32;
        if core::mem::size_of_val(&$into) == core::mem::size_of::<u32>() {
            tmp = unsafe {
                core::ptr::read((&$into as *const _).cast::<u8>() as *const u32)
            };
            unsafe { $crate::write_aux_reg($reg, tmp) };
        } else {
            unsafe { $crate::bogus_undefined() };
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
