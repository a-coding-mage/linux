// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/m68k/mm/memory.c
 *
 *  Copyright (C) 1995  Hamish Macdonald
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

#[inline(always)]
unsafe fn clear040(paddr: ::core::ffi::c_ulong) {
    ::core::arch::asm!(
        "nop\n\t",
        ".chip 68040\n\t",
        "cinvp %bc,({0})\n\t",
        ".chip 68k",
        in("a0") paddr,
    );
}

#[inline(always)]
unsafe fn cleari040(paddr: ::core::ffi::c_ulong) {
    ::core::arch::asm!(
        "nop\n\t",
        ".chip 68040\n\t",
        "cinvp %ic,({0})\n\t",
        ".chip 68k",
        in("a0") paddr,
    );
}

#[inline(always)]
unsafe fn push040(paddr: ::core::ffi::c_ulong) {
    ::core::arch::asm!(
        "nop\n\t",
        ".chip 68040\n\t",
        "cpushp %bc,({0})\n\t",
        ".chip 68k",
        in("a0") paddr,
    );
}

#[inline(always)]
unsafe fn pushcl040(paddr: ::core::ffi::c_ulong) {
    let mut flags: ::core::ffi::c_ulong = 0;
    local_irq_save(&mut flags);
    push040(paddr);
    if CPU_IS_060 {
        clear040(paddr);
    }
    local_irq_restore(flags);
}

extern "C" {
    fn local_irq_save(flags: *mut ::core::ffi::c_ulong);
    fn local_irq_restore(flags: ::core::ffi::c_ulong);
    fn clear_cf_bcache(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    fn flush_cf_bcache(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    static mach_l2_flush: Option<unsafe extern "C" fn(which: i32)>;
}

// Architecture-provided constants and CPU predicates.
extern "C" {
    static CPU_IS_COLDFIRE: bool;
    static CPU_IS_040_OR_060: bool;
    static CPU_IS_060: bool;
    static PAGE_SIZE: ::core::ffi::c_ulong;
    static PAGE_MASK: ::core::ffi::c_ulong;
    static DCACHE_MAX_ADDR: ::core::ffi::c_ulong;
    static FLUSH_I_AND_D: u16;
    static FLUSH_I: u16;
}

#[no_mangle]
pub unsafe extern "C" fn cache_clear(mut paddr: ::core::ffi::c_ulong, mut len: i32) {
    if CPU_IS_COLDFIRE {
        clear_cf_bcache(0, DCACHE_MAX_ADDR);
    } else if CPU_IS_040_OR_060 {
        let mut tmp: i32;

        tmp = ((0 as ::core::ffi::c_ulong).wrapping_sub(paddr) & (PAGE_SIZE - 1)) as i32;
        if tmp != 0 {
            pushcl040(paddr & PAGE_MASK);
            len -= tmp;
            if len <= 0 {
                return;
            }
            paddr = paddr.wrapping_add(tmp as ::core::ffi::c_ulong);
        }
        tmp = PAGE_SIZE as i32;
        paddr &= PAGE_MASK;
        while {
            len -= tmp;
            len >= 0
        } {
            clear040(paddr);
            paddr = paddr.wrapping_add(tmp as ::core::ffi::c_ulong);
        }
        len += tmp;
        if len != 0 {
            pushcl040(paddr);
        }
    } else {
        ::core::arch::asm!(
            "movec %cacr,%d0\n\t",
            "oriw {0},%d0\n\t",
            "movec %d0,%cacr",
            const FLUSH_I_AND_D,
            out("d0") _,
        );
    }
    #[cfg(CONFIG_M68K_L2_CACHE)]
    if let Some(flush) = mach_l2_flush {
        flush(0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cache_push(mut paddr: ::core::ffi::c_ulong, mut len: i32) {
    if CPU_IS_COLDFIRE {
        flush_cf_bcache(0, DCACHE_MAX_ADDR);
    } else if CPU_IS_040_OR_060 {
        let tmp = PAGE_SIZE as i32;
        len += (paddr & (PAGE_SIZE - 1)) as i32;
        paddr &= PAGE_MASK;
        loop {
            push040(paddr);
            paddr = paddr.wrapping_add(tmp as ::core::ffi::c_ulong);
            len -= tmp;
            if len <= 0 {
                break;
            }
        }
    } else {
        ::core::arch::asm!(
            "movec %cacr,%d0\n\t",
            "oriw {0},%d0\n\t",
            "movec %d0,%cacr",
            const FLUSH_I,
            out("d0") _,
        );
    }
    #[cfg(CONFIG_M68K_L2_CACHE)]
    if let Some(flush) = mach_l2_flush {
        flush(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
