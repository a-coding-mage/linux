/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2007 Davud Gibson, IBM Corporation.
 */

#[inline]
pub unsafe fn mfpvr() -> u32 {
    let mut pvr: u32;
    core::arch::asm!("mfpvr {0}", out(reg) pvr);
    pvr
}

macro_rules! __stringify_1 {
    ($x:ident) => {
        stringify!($x)
    };
    ($x:tt) => {
        stringify!($x)
    };
}

macro_rules! __stringify {
    ($x:ident) => {
        __stringify_1!($x)
    };
    ($x:tt) => {
        __stringify_1!($x)
    };
}

macro_rules! mfspr {
    ($rn:tt) => {{
        let rval: u64;
        core::arch::asm!(concat!("mfspr {0},", __stringify!($rn)), out(reg) rval);
        rval
    }};
}

macro_rules! mtspr {
    ($rn:tt, $v:expr) => {
        core::arch::asm!(
            concat!("mtspr ", __stringify!($rn), ",%0"),
            in(reg) $v
        )
    };
}

// C register variable bound to the PowerPC stack-pointer register r1.
#[cfg(target_arch = "powerpc")]
#[no_mangle]
pub static mut __stack_pointer: *mut core::ffi::c_void = core::ptr::null_mut();

macro_rules! get_sp {
    () => {
        __stack_pointer
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
