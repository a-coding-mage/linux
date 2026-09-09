/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// Dependency: <asm-generic/module.h>

/* Microblaze Relocations */
pub const R_MICROBLAZE_NONE: i32 = 0;
pub const R_MICROBLAZE_32: i32 = 1;
pub const R_MICROBLAZE_32_PCREL: i32 = 2;
pub const R_MICROBLAZE_64_PCREL: i32 = 3;
pub const R_MICROBLAZE_32_PCREL_LO: i32 = 4;
pub const R_MICROBLAZE_64: i32 = 5;
pub const R_MICROBLAZE_32_LO: i32 = 6;
pub const R_MICROBLAZE_SRO32: i32 = 7;
pub const R_MICROBLAZE_SRW32: i32 = 8;
pub const R_MICROBLAZE_64_NONE: i32 = 9;
pub const R_MICROBLAZE_32_SYM_OP_SYM: i32 = 10;
/* Keep this the last entry. */
pub const R_MICROBLAZE_NUM: i32 = 11;

#[repr(C)]
pub struct module_t {
    pub counter: core::cell::UnsafeCell<i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
