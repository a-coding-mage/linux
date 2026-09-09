/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2022 Xilinx, Inc.
 */

// C dependency: linux/of_address.h

/*
 * When the break vector gets asserted because of error injection, the break
 * signal must be blocked before exiting from the break handler, Below api
 * updates the manager address and control register and error counter callback
 * arguments, which will be used by the break handler to block the break and
 * call the callback function.
 */
extern "C" {
    pub fn xmb_manager_register(
        phys_baseaddr: usize,
        cr_val: u32,
        callback: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void)>,
        priv_data: *mut core::ffi::c_void,
        reset_callback: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void)>,
    );

    pub fn xmb_inject_err();
}

/* Error injection offset */
pub const XMB_INJECT_ERR_OFFSET: usize = 0x200;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
