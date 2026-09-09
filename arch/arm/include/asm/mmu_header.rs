/* SPDX-License-Identifier: GPL-2.0 */

/* The C header is conditional on CONFIG_MMU. */
#[cfg(feature = "CONFIG_MMU")]
#[repr(C)]
pub struct mm_context_t {
    #[cfg(feature = "CONFIG_CPU_HAS_ASID")]
    pub id: atomic64_t,
    #[cfg(not(feature = "CONFIG_CPU_HAS_ASID"))]
    pub switch_pending: i32,
    pub vmalloc_seq: atomic_t,
    pub sigpage: usize,
    #[cfg(feature = "CONFIG_VDSO")]
    pub vdso: usize,
    #[cfg(feature = "CONFIG_BINFMT_ELF_FDPIC")]
    pub exec_fdpic_loadmap: usize,
    #[cfg(feature = "CONFIG_BINFMT_ELF_FDPIC")]
    pub interp_fdpic_loadmap: usize,
}

#[cfg(feature = "CONFIG_CPU_HAS_ASID")]
pub const ASID_BITS: u32 = 8;

#[cfg(feature = "CONFIG_CPU_HAS_ASID")]
pub const ASID_MASK: u64 = u64::MAX << ASID_BITS;

#[cfg(feature = "CONFIG_CPU_HAS_ASID")]
#[macro_export]
macro_rules! ASID {
    ($mm:expr) => {
        (($mm).context.id.counter & !$crate::ASID_MASK) as u32
    };
}

#[cfg(not(feature = "CONFIG_CPU_HAS_ASID"))]
#[macro_export]
macro_rules! ASID {
    ($mm:expr) => {
        0
    };
}

/*
 * From nommu.h:
 *  Copyright (C) 2002, David McCullough <davidm@snapgear.com>
 *  modified for 2.6 by Hyok S. Choi <hyok.choi@samsung.com>
 */
#[cfg(not(feature = "CONFIG_MMU"))]
#[repr(C)]
pub struct mm_context_t {
    pub end_brk: usize,
    #[cfg(feature = "CONFIG_BINFMT_ELF_FDPIC")]
    pub exec_fdpic_loadmap: usize,
    #[cfg(feature = "CONFIG_BINFMT_ELF_FDPIC")]
    pub interp_fdpic_loadmap: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
