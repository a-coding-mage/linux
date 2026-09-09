/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2023 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kvm_host.h, asm/pgalloc.h, and asm/tlb.h.

pub const KVM_MMU_CACHE_MIN_PAGES: usize = CONFIG_PGTABLE_LEVELS - 1;

pub const KVM_PAGE_WRITEABLE: usize = _PAGE_MODIFIED;

pub const _KVM_FLUSH_PGTABLE: usize = 0x1;
pub const _KVM_HAS_PGMASK: usize = 0x2;

pub type kvm_pte_t = ::core::ffi::c_ulong;
pub type kvm_ptw_ctx = kvm_ptw_ctx_struct;
pub type kvm_pte_ops = unsafe extern "C" fn(
    pte: *mut kvm_pte_t,
    addr: phys_addr_t,
    ctx: *mut kvm_ptw_ctx,
) -> ::core::ffi::c_int;

#[repr(C)]
pub struct kvm_ptw_ctx_struct {
    pub ops: Option<kvm_pte_ops>,
    pub flag: ::core::ffi::c_ulong,

    /* for kvm_arch_mmu_enable_log_dirty_pt_masked use */
    pub mask: ::core::ffi::c_ulong,
    pub gfn: ::core::ffi::c_ulong,

    /* page walk mmu info */
    pub level: ::core::ffi::c_uint,
    pub pgtable_shift: ::core::ffi::c_ulong,
    pub invalid_entry: ::core::ffi::c_ulong,
    pub invalid_ptes: *mut ::core::ffi::c_ulong,
    pub pte_shifts: *mut ::core::ffi::c_uint,
    pub opaque: *mut ::core::ffi::c_void,

    /* free pte table page list */
    pub list: list_head,
}

extern "C" {
    pub fn kvm_pgd_alloc() -> *mut kvm_pte_t;
}

#[inline]
pub unsafe fn kvm_pfn_pte(pfn: usize, prot: pgprot_t) -> kvm_pte_t {
    ((pfn << PFN_PTE_SHIFT) | pgprot_val(prot)) as kvm_pte_t
}

#[inline]
pub unsafe fn kvm_pte_pfn(x: kvm_pte_t) -> phys_addr_t {
    (((x & _PFN_MASK) >> PFN_PTE_SHIFT) as phys_addr_t)
}

#[inline]
pub unsafe fn kvm_set_pte(ptep: *mut kvm_pte_t, val: kvm_pte_t) {
    WRITE_ONCE(ptep, val);
}

#[inline]
pub unsafe fn kvm_pte_young(pte: kvm_pte_t) -> ::core::ffi::c_int { (pte & _PAGE_ACCESSED) as ::core::ffi::c_int }

#[inline]
pub unsafe fn kvm_pte_huge(pte: kvm_pte_t) -> ::core::ffi::c_int { (pte & _PAGE_HUGE) as ::core::ffi::c_int }

#[inline]
pub unsafe fn kvm_pte_dirty(pte: kvm_pte_t) -> ::core::ffi::c_int { (pte & __WRITEABLE) as ::core::ffi::c_int }

#[inline]
pub unsafe fn kvm_pte_writeable(pte: kvm_pte_t) -> ::core::ffi::c_int { (pte & KVM_PAGE_WRITEABLE) as ::core::ffi::c_int }

#[inline]
pub unsafe fn kvm_pte_mkyoung(pte: kvm_pte_t) -> kvm_pte_t { pte | _PAGE_ACCESSED }

#[inline]
pub unsafe fn kvm_pte_mkold(pte: kvm_pte_t) -> kvm_pte_t { pte & !_PAGE_ACCESSED }

#[inline]
pub unsafe fn kvm_pte_mkdirty(pte: kvm_pte_t) -> kvm_pte_t { pte | __WRITEABLE }

#[inline]
pub unsafe fn kvm_pte_mkclean(pte: kvm_pte_t) -> kvm_pte_t { pte & !__WRITEABLE }

#[inline]
pub unsafe fn kvm_pte_mkhuge(pte: kvm_pte_t) -> kvm_pte_t { pte | _PAGE_HUGE }

#[inline]
pub unsafe fn kvm_pte_mksmall(pte: kvm_pte_t) -> kvm_pte_t { pte & !_PAGE_HUGE }

#[inline]
pub unsafe fn kvm_pte_mkwriteable(pte: kvm_pte_t) -> kvm_pte_t { pte | KVM_PAGE_WRITEABLE }

#[inline]
pub unsafe fn kvm_need_flush(ctx: *mut kvm_ptw_ctx) -> ::core::ffi::c_int {
    ((*ctx).flag & _KVM_FLUSH_PGTABLE) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn kvm_pgtable_offset(
    ctx: *mut kvm_ptw_ctx,
    table: *mut kvm_pte_t,
    addr: phys_addr_t,
) -> *mut kvm_pte_t {
    table.add(((addr >> (*ctx).pgtable_shift) & (PTRS_PER_PTE - 1)) as usize)
}

#[inline]
pub unsafe fn kvm_pgtable_addr_end(ctx: *mut kvm_ptw_ctx, addr: phys_addr_t, end: phys_addr_t) -> phys_addr_t {
    let size: phys_addr_t = 0x1 as phys_addr_t << (*ctx).pgtable_shift;
    let boundary = (addr + size) & !(size - 1);
    if boundary - 1 < end - 1 { boundary } else { end }
}

#[inline]
pub unsafe fn kvm_pte_present(ctx: *mut kvm_ptw_ctx, entry: *mut kvm_pte_t) -> ::core::ffi::c_int {
    if ctx.is_null() || (*ctx).level == 0 {
        ((*entry & _PAGE_PRESENT) != 0) as ::core::ffi::c_int
    } else {
        ((*entry != (*ctx).invalid_entry) as ::core::ffi::c_int)
    }
}

#[inline]
pub unsafe fn kvm_pte_none(ctx: *mut kvm_ptw_ctx, entry: *mut kvm_pte_t) -> ::core::ffi::c_int {
    (*entry == (*ctx).invalid_entry) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn kvm_ptw_enter(ctx: *mut kvm_ptw_ctx) {
    (*ctx).level -= 1;
    (*ctx).pgtable_shift = *(*ctx).pte_shifts.add((*ctx).level as usize) as ::core::ffi::c_ulong;
    (*ctx).invalid_entry = *(*ctx).invalid_ptes.add((*ctx).level as usize);
}

#[inline]
pub unsafe fn kvm_ptw_exit(ctx: *mut kvm_ptw_ctx) {
    (*ctx).level += 1;
    (*ctx).pgtable_shift = *(*ctx).pte_shifts.add((*ctx).level as usize) as ::core::ffi::c_ulong;
    (*ctx).invalid_entry = *(*ctx).invalid_ptes.add((*ctx).level as usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
