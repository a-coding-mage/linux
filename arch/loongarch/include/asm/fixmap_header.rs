/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fixmap.h: compile-time virtual memory allocation
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// #ifdef CONFIG_HIGHMEM
// #include <linux/threads.h>
// #include <asm/kmap_size.h>
// #endif

pub const NR_FIX_BTMAPS: usize = 64;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum fixed_addresses {
    FIX_HOLE,
    // #ifdef CONFIG_HIGHMEM
    FIX_KMAP_BEGIN,
    FIX_KMAP_END = FIX_KMAP_BEGIN as isize + (KM_MAX_IDX * NR_CPUS) as isize - 1,
    // #endif
    FIX_EARLYCON_MEM_BASE,
    __end_of_fixed_addresses,
}

pub const FIXADDR_SIZE: usize = (__end_of_fixed_addresses as usize) << PAGE_SHIFT;
pub const FIXADDR_START: usize = (FIXADDR_TOP as usize) - FIXADDR_SIZE;
pub const FIXMAP_PAGE_IO: pgprot_t = PAGE_KERNEL_SUC;

pub unsafe extern "C" {
    pub fn __set_fixmap(idx: fixed_addresses, phys: phys_addr_t, flags: pgprot_t);
}

// #include <asm-generic/fixmap.h>

/*
 * Called from pagetable_init()
 */
pub unsafe extern "C" {
    pub fn fixrange_init(start: c_ulong, end: c_ulong, pgd_base: *mut pgd_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
