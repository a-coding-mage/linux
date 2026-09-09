/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

pub const _PAGE_ACCESSED_OFFSET: usize = 6;

pub const _PAGE_PRESENT: usize = 1 << 0;
pub const _PAGE_READ: usize = 1 << 1; // Readable
pub const _PAGE_WRITE: usize = 1 << 2; // Writable
pub const _PAGE_EXEC: usize = 1 << 3; // Executable
pub const _PAGE_USER: usize = 1 << 4; // User
pub const _PAGE_GLOBAL: usize = 1 << 5; // Global
pub const _PAGE_ACCESSED: usize = 1 << 6; // Set by hardware on any access
pub const _PAGE_DIRTY: usize = 1 << 7; // Set by hardware on any write
pub const _PAGE_SOFT: usize = 3 << 8; // Reserved for software

pub const _PAGE_SPECIAL: usize = 1 << 8; // RSW: 0x1

/* CONFIG_MEM_SOFT_DIRTY */
#[cfg(feature = "CONFIG_MEM_SOFT_DIRTY")]
pub fn _PAGE_SOFT_DIRTY() -> usize {
    // ext_svrsw60t59b: bit 59 for soft-dirty tracking
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_SVRSW60T59B) {
        1usize << 59
    } else {
        0
    }
}

/*
 * Bit 3 is always zero for swap entry computation, so we
 * can borrow it for swap page soft-dirty tracking.
 */
#[cfg(feature = "CONFIG_MEM_SOFT_DIRTY")]
pub fn _PAGE_SWP_SOFT_DIRTY() -> usize {
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_SVRSW60T59B) {
        _PAGE_EXEC
    } else {
        0
    }
}

#[cfg(not(feature = "CONFIG_MEM_SOFT_DIRTY"))]
pub const _PAGE_SOFT_DIRTY: usize = 0;
#[cfg(not(feature = "CONFIG_MEM_SOFT_DIRTY"))]
pub const _PAGE_SWP_SOFT_DIRTY: usize = 0;

/* CONFIG_HAVE_ARCH_USERFAULTFD_WP */
#[cfg(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP")]
pub fn _PAGE_UFFD() -> usize {
    // ext_svrsw60t59b: Bit(60) for userfaultfd tracking
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_SVRSW60T59B) {
        1usize << 60
    } else {
        0
    }
}

/*
 * Bit 4 is not involved into swap entry computation, so we
 * can borrow it for swap page userfaultfd tracking.
 */
#[cfg(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP")]
pub fn _PAGE_SWP_UFFD() -> usize {
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_SVRSW60T59B) {
        _PAGE_USER
    } else {
        0
    }
}

#[cfg(not(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP"))]
pub const _PAGE_UFFD: usize = 0;
#[cfg(not(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP"))]
pub const _PAGE_SWP_UFFD: usize = 0;

pub const _PAGE_TABLE: usize = _PAGE_PRESENT;

/*
 * _PAGE_PROT_NONE is set on not-present pages (and ignored by the hardware) to
 * distinguish them from swapped out pages
 */
pub const _PAGE_PROT_NONE: usize = _PAGE_GLOBAL;

/* Used for swap PTEs only. */
pub const _PAGE_SWP_EXCLUSIVE: usize = _PAGE_ACCESSED;

pub const _PAGE_PFN_SHIFT: usize = 10;

/*
 * when all of R/W/X are zero, the PTE is a pointer to the next level
 * of the page table; otherwise, it is a leaf PTE.
 */
pub const _PAGE_LEAF: usize = _PAGE_READ | _PAGE_WRITE | _PAGE_EXEC;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
