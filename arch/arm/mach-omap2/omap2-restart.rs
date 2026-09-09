// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap2-restart.c - code common to all OMAP2xxx machines.
 *
 * Copyright (C) 2012 Texas Instruments
 * Paul Walmsley
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_char;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

pub type reboot_mode = i32;

extern "C" {
    fn clk_get(dev: *mut core::ffi::c_void, id: *const c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn clk_set_rate(clk: *mut clk, rate: u32) -> i32;
    fn omap_prm_reset_system();
    fn IS_ERR(ptr: *mut clk) -> bool;
}

const EINVAL: i32 = 22;

/*
 * reset_virt_prcm_set_ck, reset_sys_ck: pointers to the virt_prcm_set
 * clock and the sys_ck.  Used during the reset process
 */
static mut reset_virt_prcm_set_ck: *mut clk = core::ptr::null_mut();
static mut reset_sys_ck: *mut clk = core::ptr::null_mut();

/* Reboot handling */

/**
 * omap2xxx_restart - Set DPLL to bypass mode for reboot to work
 *
 * Set the DPLL to bypass so that reboot completes successfully.  No
 * return value.
 */
pub unsafe extern "C" fn omap2xxx_restart(_mode: reboot_mode, _cmd: *const c_char) {
    let rate: u32;

    rate = clk_get_rate(reset_sys_ck);
    clk_set_rate(reset_virt_prcm_set_ck, rate);

    /* XXX Should save the cmd argument for use after the reboot */

    omap_prm_reset_system();
}

/**
 * omap2xxx_common_look_up_clks_for_reset - look up clocks needed for restart
 *
 * Some clocks need to be looked up in advance for the SoC restart
 * operation to work - see omap2xxx_restart().  Returns -EINVAL upon
 * error or 0 upon success.
 */
pub unsafe extern "C" fn omap2xxx_common_look_up_clks_for_reset() -> i32 {
    reset_virt_prcm_set_ck = clk_get(core::ptr::null_mut(), b"virt_prcm_set\0".as_ptr() as *const c_char);
    if IS_ERR(reset_virt_prcm_set_ck) {
        return -EINVAL;
    }

    reset_sys_ck = clk_get(core::ptr::null_mut(), b"sys_ck\0".as_ptr() as *const c_char);
    if IS_ERR(reset_sys_ck) {
        return -EINVAL;
    }

    0
}

// omap_postcore_initcall(omap2xxx_common_look_up_clks_for_reset);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
