/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/procinfo.h
 *
 *  Copyright (C) 1996-1999 Russell King
 */

// The C header guard and __KERNEL__ conditional are represented here with a
// Cargo feature.  The declarations below are present when that feature is
// enabled.
#[cfg(feature = "__KERNEL__")]
mod kernel {
    use core::ffi::{c_char, c_uint, c_ulong};

    pub struct CpuTlbFns;
    pub struct CpuUserFns;
    pub struct CpuCacheFns;
    pub struct Processor;

    /*
     * Note!  struct processor is always defined if we're
     * using MULTI_CPU, otherwise this entry is unused,
     * but still exists.
     *
     * NOTE! The following structure is defined by assembly
     * language, NOT C code.  For more information, check:
     *  arch/arm/mm/proc-*.S and arch/arm/kernel/head.S
     */
    #[repr(C)]
    pub struct ProcInfoList {
        pub cpu_val: c_uint,
        pub cpu_mask: c_uint,
        pub __cpu_mm_mmu_flags: c_ulong, /* used by head.S */
        pub __cpu_io_mmu_flags: c_ulong, /* used by head.S */
        pub __cpu_flush: c_ulong,        /* used by head.S */
        pub arch_name: *const c_char,
        pub elf_name: *const c_char,
        pub elf_hwcap: c_uint,
        pub cpu_name: *const c_char,
        pub proc: *mut Processor,
        pub tlb: *mut CpuTlbFns,
        pub user: *mut CpuUserFns,
        pub cache: *mut CpuCacheFns,
    }
}

// When __KERNEL__ is not enabled, the C header includes <asm/elf.h> and emits
// a warning asking callers to include asm/elf.h instead.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
