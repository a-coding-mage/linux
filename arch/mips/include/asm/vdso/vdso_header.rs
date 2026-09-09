/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Imagination Technologies
 * Author: Alex Smith <alex.smith@imgtec.com>
 */

// Dependencies supplied by the surrounding kernel/VDSO build are intentionally
// left external to this translation.

pub const __VDSO_PAGES: usize = 4;

#[allow(non_camel_case_types)]
pub enum vdso_time_data {}

#[inline]
pub unsafe fn get_vdso_time_data() -> *const vdso_time_data {
    let addr: *const vdso_time_data;

    /*
     * We can't use cpu_has_mips_r6 since it needs the cpu_data[]
     * kernel symbol.
     */
    #[cfg(CONFIG_CPU_MIPSR6)]
    {
        /*
         * lapc <symbol> is an alias to addiupc reg, <symbol> - .
         *
         * We can't use addiupc because there is no label-label
         * support for the addiupc reloc.
         */
        core::arch::asm!(
            "lapc {0}, vdso_u_time_data",
            out(reg) addr,
            options(nostack, preserves_flags),
        );
    }

    #[cfg(not(CONFIG_CPU_MIPSR6))]
    {
        /*
         * Get the base load address of the VDSO. We have to avoid generating
         * relocations and references to the GOT because ld.so does not perform
         * relocations on the VDSO. We use the current offset from the VDSO base
         * and perform a PC-relative branch which gives the absolute address in
         * ra, and take the difference. The assembler chokes on
         * "li %0, _start - .", so embed the offset as a word and branch over
         * it.
         */
        core::arch::asm!(
            ".set push",
            ".set noreorder",
            "bal 1f",
            " nop",
            ".word vdso_u_time_data - .",
            "1: lw {0}, 0($31)",
            "PTR_ADDU {0}, $31, {0}",
            ".set pop",
            out(reg) addr,
            clobber_abi("C"),
        );
    }

    addr
}

#[cfg(CONFIG_CLKSRC_MIPS_GIC)]
#[inline]
pub unsafe fn get_gic(data: *const vdso_time_data) -> *mut core::ffi::c_void {
    // Equivalent to (void __iomem *)((unsigned long)data & PAGE_MASK) - PAGE_SIZE.
    ((data as usize & PAGE_MASK) - PAGE_SIZE) as *mut core::ffi::c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
