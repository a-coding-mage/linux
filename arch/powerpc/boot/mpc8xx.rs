// SPDX-License-Identifier: GPL-2.0-only
/*
 * MPC8xx support functions
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding implementation.
extern "C" {
    fn fsl_get_immr() -> *mut u32;
    fn printf(format: *const u8, ...);
    fn in_be32(addr: *const u32) -> u32;
    fn dt_fixup_cpu_clocks(sysclk: u32, tbclk: u32, busclk: u32);
    fn finddevice(path: *const u8) -> *mut core::ffi::c_void;
    fn setprop(node: *mut core::ffi::c_void, name: *const u8, value: *const core::ffi::c_void, len: u32);
}

const MPC8XX_PLPRCR: usize = 0x284 / 4;

/* Return system clock from crystal frequency */
pub unsafe fn mpc885_get_clock(crystal: u32) -> u32 {
    let immr: *mut u32;
    let plprcr: u32;
    let mut mfi: i32;
    let mfn: i32;
    let mfd: i32;
    let pdf: i32;
    let mut ret: u32;

    immr = fsl_get_immr();
    if immr.is_null() {
        printf(b"mpc885_get_clock: Couldn't get IMMR base.\r\n\0".as_ptr());
        return 0;
    }

    plprcr = in_be32(immr.add(MPC8XX_PLPRCR));

    mfi = ((plprcr >> 16) & 15) as i32;
    if mfi < 5 {
        printf(b"Warning: PLPRCR[MFI] value of %d out-of-bounds\r\n\0".as_ptr(), mfi);
        mfi = 5;
    }

    pdf = ((plprcr >> 1) & 0xf) as i32;
    mfd = ((plprcr >> 22) & 0x1f) as i32;
    mfn = ((plprcr >> 27) & 0x1f) as i32;

    ret = crystal.wrapping_mul(mfi as u32);

    if mfn != 0 {
        ret = ret.wrapping_add(crystal.wrapping_mul(mfn as u32) / (mfd as u32 + 1));
    }

    ret / (pdf as u32 + 1)
}

/* Set common device tree fields based on the given clock frequencies. */
pub unsafe fn mpc8xx_set_clocks(sysclk: u32) {
    let mut node: *mut core::ffi::c_void;

    dt_fixup_cpu_clocks(sysclk, sysclk / 16, sysclk);

    node = finddevice(b"/soc/cpm\0".as_ptr());
    if !node.is_null() {
        setprop(node, b"clock-frequency\0".as_ptr(), &sysclk as *const u32 as *const core::ffi::c_void, 4);
    }

    node = finddevice(b"/soc/cpm/brg\0".as_ptr());
    if !node.is_null() {
        setprop(node, b"clock-frequency\0".as_ptr(), &sysclk as *const u32 as *const core::ffi::c_void, 4);
    }
}

pub unsafe fn mpc885_fixup_clocks(crystal: u32) -> i32 {
    let sysclk = mpc885_get_clock(crystal);
    if sysclk == 0 {
        return 0;
    }

    mpc8xx_set_clocks(sysclk);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
