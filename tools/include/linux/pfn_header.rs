/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent from C header: #include <linux/mm.h> */

pub const fn PFN_UP(x: usize) -> usize {
    (x.wrapping_add(PAGE_SIZE).wrapping_sub(1)) >> PAGE_SHIFT
}

pub const fn PFN_DOWN(x: usize) -> usize {
    x >> PAGE_SHIFT
}

pub const fn PFN_PHYS(x: usize) -> phys_addr_t {
    (x as phys_addr_t) << PAGE_SHIFT
}

pub const fn PHYS_PFN(x: phys_addr_t) -> libc::c_ulong {
    (x >> PAGE_SHIFT) as libc::c_ulong
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
