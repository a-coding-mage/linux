/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019  Arm Limited
 * Original author: Dave Martin <Dave.Martin@arm.com>
 */

/* Header guard BTITEST_H omitted in Rust. */

unsafe extern "C" {
    /* Trampolines for calling the test stubs: */
    pub fn call_using_br_x0(arg1: Option<unsafe extern "C" fn()>);
    pub fn call_using_br_x16(arg1: Option<unsafe extern "C" fn()>);
    pub fn call_using_blr(arg1: Option<unsafe extern "C" fn()>);

    /* Test stubs: */
    pub fn nohint_func();
    pub fn bti_none_func();
    pub fn bti_c_func();
    pub fn bti_j_func();
    pub fn bti_jc_func();
    pub fn paciasp_func();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
