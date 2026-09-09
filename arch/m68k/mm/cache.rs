// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/m68k/mm/cache.c
 *
 *  Instruction cache handling
 *
 *  Copyright (C) 1995  Hamish Macdonald
 */

// Dependencies supplied by the surrounding kernel build are intentionally
// referenced here rather than implemented in this translation unit.

use core::arch::asm;

extern "C" {
    static CPU_IS_060: bool;
    static CPU_IS_040: bool;
    static CPU_IS_040_OR_060: bool;
    static CPU_IS_COLDFIRE: bool;
    static MMU_R_040: c_ulong;
    static PAGE_MASK: c_ulong;
    static PAGE_SIZE: c_ulong;
    static ICACHE_SET_MASK: c_ulong;
    static ICACHE_MAX_ADDR: c_ulong;
    static FLUSH_I: i16;

    fn flush_cf_icache(start: c_ulong, end: c_ulong);
    fn set_fc(fc: c_int);
    fn page_to_phys(page: *mut page) -> c_ulong;
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

type c_ulong = usize;
type c_int = i32;

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

extern "C" {
    static SUPER_DATA: c_int;
    static USER_DATA: c_int;
}

unsafe fn virt_to_phys_slow(vaddr: c_ulong) -> c_ulong {
    if CPU_IS_060 {
        let mut paddr: c_ulong;

        /* The PLPAR instruction causes an access error if the translation
         * is not possible. To catch this we use the same exception mechanism
         * as for user space accesses in <asm/uaccess.h>. */
        asm!(
            ".chip 68060\n1: plpar ({0})\n.chip 68k\n2:\n.section .fixup,\"ax\"\n   .even\n3: sub.l {0},{0}\n   jra 2b\n.previous\n.section __ex_table,\"a\"\n   .align 4\n   .long 1b,3b\n.previous",
            inout("a") vaddr => paddr,
        );
        paddr
    } else if CPU_IS_040 {
        let mut mmusr: c_ulong;

        asm!(
            ".chip 68040\n\tptestr ({1})\n\tmovec %mmusr, {0}\n\t.chip 68k",
            out("r") mmusr,
            in("a") vaddr,
        );

        if mmusr & MMU_R_040 != 0 {
            (mmusr & PAGE_MASK) | (vaddr & !PAGE_MASK)
        } else {
            0
        }
    } else {
        WARN_ON_ONCE(!CPU_IS_040_OR_060);
        0
    }
}

/* Push n pages at kernel virtual address and clear the icache */
/* RZ: use cpush %bc instead of cpush %dc, cinv %ic */
pub unsafe fn flush_icache_user_range(mut address: c_ulong, endaddr: c_ulong) {
    if CPU_IS_COLDFIRE {
        let mut start = address & ICACHE_SET_MASK;
        let mut end = endaddr & ICACHE_SET_MASK;
        if start > end {
            flush_cf_icache(0, end);
            end = ICACHE_MAX_ADDR;
        }
        flush_cf_icache(start, end);
    } else if CPU_IS_040_OR_060 {
        address &= PAGE_MASK;

        loop {
            asm!(
                "nop\n\t.chip 68040\n\tcpushp %bc,({0})\n\t.chip 68k",
                in("a") virt_to_phys_slow(address),
            );
            address += PAGE_SIZE;
            if address >= endaddr {
                break;
            }
        }
    } else {
        let mut tmp: c_ulong;
        asm!(
            "movec %cacr,{0}\n\torw {1},{0}\n\tmovec {0},%cacr",
            out("d") tmp,
            in("d") FLUSH_I,
        );
    }
}

pub unsafe fn flush_icache_range(address: c_ulong, endaddr: c_ulong) {
    set_fc(SUPER_DATA);
    flush_icache_user_range(address, endaddr);
    set_fc(USER_DATA);
}

#[no_mangle]
pub unsafe extern "C" fn flush_icache_user_page(
    _vma: *mut vm_area_struct,
    page: *mut page,
    addr: c_ulong,
    len: c_int,
) {
    if CPU_IS_COLDFIRE {
        let start = addr & ICACHE_SET_MASK;
        let mut end = (addr + len as c_ulong) & ICACHE_SET_MASK;
        if start > end {
            flush_cf_icache(0, end);
            end = ICACHE_MAX_ADDR;
        }
        flush_cf_icache(start, end);
    } else if CPU_IS_040_OR_060 {
        asm!(
            "nop\n\t.chip 68040\n\tcpushp %bc,({0})\n\t.chip 68k",
            in("a") page_to_phys(page),
        );
    } else {
        let mut tmp: c_ulong;
        asm!(
            "movec %cacr,{0}\n\torw {1},{0}\n\tmovec {0},%cacr",
            out("d") tmp,
            in("d") FLUSH_I,
        );
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
