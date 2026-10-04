// SPDX-License-Identifier: GPL-2.0-only
// Canonical configured types and native inline/macro ABIs shared by both owners.
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/x86_mm_init_generated.rs"
    ));
}
use b::*;
use bindings as b;
use core::cmp::{max, min};
use core::mem::MaybeUninit;
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
const PAGE_SHIFT: u32 = b::RUST_MM_PAGE_SHIFT;
const PMD_SHIFT: u32 = b::RUST_MM_PMD_SHIFT;
const PAGE_SIZE: c_ulong = b::RUST_MM_PAGE_SIZE;
const PAGE_MASK: c_ulong = b::RUST_MM_PAGE_MASK as c_ulong;
const PMD_SIZE: c_ulong = b::RUST_MM_PMD_SIZE;
const PMD_MASK: c_ulong = b::RUST_MM_PMD_MASK as c_ulong;
const PUD_SIZE: c_ulong = b::RUST_MM_PUD_SIZE;
const PUD_MASK: c_ulong = b::RUST_MM_PUD_MASK as c_ulong;
const P4D_SIZE: c_ulong = b::RUST_MM_P4D_SIZE;
const P4D_MASK: c_ulong = b::RUST_MM_P4D_MASK as c_ulong;
const PTRS_PER_PTE: usize = b::RUST_MM_PTRS_PER_PTE as usize;
const PTRS_PER_PMD: usize = b::RUST_MM_PTRS_PER_PMD as usize;
const PTRS_PER_PUD: usize = b::RUST_MM_PTRS_PER_PUD as usize;
const PTRS_PER_PGD: usize = b::RUST_MM_PTRS_PER_PGD as usize;
const _PAGE_PWT: c_ulong = b::RUST_MM_PAGE_PWT;
const _PAGE_PCD: c_ulong = b::RUST_MM_PAGE_PCD;
const _PAGE_PAT: c_ulong = b::RUST_MM_PAGE_PAT;
const _PAGE_PSE: c_ulong = b::RUST_MM_PAGE_PSE;
const _PAGE_USER: c_ulong = b::RUST_MM_PAGE_USER;
const _PAGE_GLOBAL: c_ulong = b::RUST_MM_PAGE_GLOBAL;
const _PAGE_CACHE_MASK: c_ulong = b::RUST_MM_PAGE_CACHE_MASK;
const _PAGE_NOPTISHADOW: c_ulong = b::RUST_MM_PAGE_NOPTISHADOW;
const ENOMEM: c_int = b::ENOMEM as c_int;
#[inline(always)]
fn round_down(value: c_ulong, align: c_ulong) -> c_ulong {
    value & !align.wrapping_sub(1)
}
#[inline(always)]
fn round_up(value: c_ulong, align: c_ulong) -> c_ulong {
    (value.wrapping_sub(1) | align.wrapping_sub(1)).wrapping_add(1)
}
#[inline(always)]
unsafe fn bug_on(condition: bool) {
    if condition {
        b::rust_mm_bug();
    }
}
#[cfg(CONFIG_X86_64)]
#[inline(always)]
unsafe fn pgd_index(addr: c_ulong) -> usize {
    b::rust_mm_pgd_index(addr) as usize
}
#[cfg(CONFIG_X86_64)]
#[inline(always)]
unsafe fn p4d_index(addr: c_ulong) -> usize {
    b::rust_mm_p4d_index(addr) as usize
}
#[cfg(CONFIG_X86_64)]
#[inline(always)]
unsafe fn pud_index(addr: c_ulong) -> usize {
    b::rust_mm_pud_index(addr) as usize
}
#[cfg(CONFIG_X86_64)]
#[inline(always)]
unsafe fn pmd_index(addr: c_ulong) -> usize {
    b::rust_mm_pmd_index(addr) as usize
}
#[cfg(CONFIG_X86_64)]
#[inline(always)]
unsafe fn pte_index(addr: c_ulong) -> usize {
    b::rust_mm_pte_index(addr) as usize
}
#[cfg(CONFIG_X86_64)]
#[inline(always)]
unsafe fn ptrs_per_p4d() -> usize {
    b::rust_mm_ptrs_per_p4d() as usize
}

const PHYS_ADDR_MAX: phys_addr_t = b::RUST_MM_PHYS_ADDR_MAX as phys_addr_t;
#[cfg(CONFIG_X86_64)]
const START_KERNEL_MAP: c_ulong = b::RUST_MM_START_KERNEL_MAP as c_ulong;
#[cfg(CONFIG_X86_64)]
const VSYSCALL_ADDR: c_ulong = b::RUST_MM_VSYSCALL_ADDR as c_ulong;

// offsetof comes from the actual configured mm_struct, including its
// anonymous wrapper. No bindgen-generated anonymous member name is assumed.
#[inline(always)]
unsafe fn mm_page_table_lock(mm: *mut mm_struct) -> *mut spinlock_t {
    mm.cast::<u8>()
        .add(b::RUST_MM_MM_PAGE_TABLE_LOCK_OFFSET as usize)
        .cast()
}
