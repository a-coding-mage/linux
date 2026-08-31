/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 * Copyright (C) 2015 Regents of the University of California
 */

/* Header guard _UAPI_ASM_RISCV_BITSPERLONG_H omitted in Rust. */

pub const __BITS_PER_LONG: usize = core::mem::size_of::<*const core::ffi::c_void>() * 8;

/* Depends on definitions from <asm-generic/bitsperlong.h>. */
