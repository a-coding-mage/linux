/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) BitBox Ltd 2010
 */

/* all normal IRQs can be FIQs */
pub const FIQ_START: i32 = 0;

#[repr(C)]
pub struct mxc_extra_irq {
    pub set_irq_fiq:
        Option<unsafe extern "C" fn(irq: core::ffi::c_uint, type_: core::ffi::c_uint) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
