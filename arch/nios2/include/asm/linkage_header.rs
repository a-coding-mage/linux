/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2009 Thomas Chou <thomas@wytron.com.tw>
 *
 * All rights reserved.
 */

/* This file is required by include/linux/linkage.h. */

/* The original assembler alignment directives are retained as strings for
 * consumers that emit assembler source. */
pub const __ALIGN: &str = ".align 4";
pub const __ALIGN_STR: &str = ".align 4";

/* Equivalent of the C _THIS_IP_ statement expression. */
#[inline(always)]
pub unsafe fn _THIS_IP_() -> usize {
    let mut __ip: usize;
    core::arch::asm!("nextpc {0}", out(reg) __ip);
    __ip
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
