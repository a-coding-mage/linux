/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Dependency supplied by <asm/vdso.h>.
#[repr(C)]
pub struct hexagon_vdso {
    _private: [u8; 0],
}

/*
 * Architecture-specific state for a mm_struct.
 * For the Hexagon Virtual Machine, it can be a copy
 * of the pointer to the page table base.
 */
#[repr(C)]
pub struct mm_context {
    pub generation: u64,
    pub ptbase: usize,
    pub vdso: *mut hexagon_vdso,
}

pub type mm_context_t = mm_context;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
