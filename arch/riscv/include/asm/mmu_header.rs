/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

/* C header guard and __ASSEMBLER__ exclusion omitted from Rust source. */

#[repr(C)]
pub struct mm_context_t {
    /* The following fields follow the C build-time CONFIG_MMU selection. */
    #[cfg(not(feature = "CONFIG_MMU"))]
    pub end_brk: libc::c_ulong,
    #[cfg(feature = "CONFIG_MMU")]
    pub id: atomic_long_t,

    pub vdso: *mut core::ffi::c_void,

    /* CONFIG_SMP */
    #[cfg(feature = "CONFIG_SMP")]
    pub icache_stale_mask: cpumask_t,
    #[cfg(feature = "CONFIG_SMP")]
    pub force_icache_flush: bool,

    /* CONFIG_BINFMT_ELF_FDPIC */
    #[cfg(feature = "CONFIG_BINFMT_ELF_FDPIC")]
    pub exec_fdpic_loadmap: libc::c_ulong,
    #[cfg(feature = "CONFIG_BINFMT_ELF_FDPIC")]
    pub interp_fdpic_loadmap: libc::c_ulong,

    pub flags: libc::c_ulong,

    /* CONFIG_RISCV_ISA_SUPM */
    #[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
    pub pmlen: u8,
}

/* Lock the pointer masking mode because this mm is multithreaded. */
pub const MM_CONTEXT_LOCK_PMLEN: i32 = 0;

#[inline]
pub const fn cntx2asid(cntx: usize) -> usize {
    cntx & SATP_ASID_MASK
}

#[inline]
pub const fn cntx2version(cntx: usize) -> usize {
    cntx & !SATP_ASID_MASK
}

extern "C" {
    pub fn create_pgd_mapping(
        pgdp: *mut pgd_t,
        va: uintptr_t,
        pa: phys_addr_t,
        sz: phys_addr_t,
        prot: pgprot_t,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
