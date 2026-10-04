// SPDX-License-Identifier: GPL-2.0
// Rust owner of mm/mprotect.c at 0db90fa02d8bc839349c44c13904a548f7dd062a.
// Native generated types and compiler-evaluated constants are the layout/ABI
// authority. All protection, batching, VMA, accounting, and syscall decisions
// defined by the original translation unit execute here.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod b {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/mprotect_native_generated.rs"
    ));
}
use b::*;
use core::mem::{zeroed, MaybeUninit};
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_int, c_long, c_ulong, c_void};

const PAGE_SIZE: c_ulong = RUST_MPROTECT_PAGE_SIZE as c_ulong;
const PAGE_SHIFT: u32 = RUST_MPROTECT_PAGE_SHIFT as u32;
const PAGE_MASK: c_ulong = !(PAGE_SIZE - 1);
const MM_CP_PROT_NUMA: c_ulong = RUST_MPROTECT_MM_CP_PROT_NUMA as c_ulong;
const MM_CP_TRY_CHANGE_WRITABLE: c_ulong = RUST_MPROTECT_MM_CP_TRY_CHANGE_WRITABLE as c_ulong;
const MM_CP_UFFD_WP: c_ulong = RUST_MPROTECT_MM_CP_UFFD_WP as c_ulong;
const MM_CP_UFFD_WP_RESOLVE: c_ulong = RUST_MPROTECT_MM_CP_UFFD_WP_RESOLVE as c_ulong;
const MM_CP_UFFD_WP_ALL: c_ulong = RUST_MPROTECT_MM_CP_UFFD_WP_ALL as c_ulong;
const MM_CP_UFFD_RWP: c_ulong = RUST_MPROTECT_MM_CP_UFFD_RWP as c_ulong;
const MM_CP_UFFD_RWP_RESOLVE: c_ulong = RUST_MPROTECT_MM_CP_UFFD_RWP_RESOLVE as c_ulong;
const MM_CP_UFFD_RWP_ALL: c_ulong = RUST_MPROTECT_MM_CP_UFFD_RWP_ALL as c_ulong;

#[inline]
unsafe fn mm(v: *mut vm_area_struct) -> *mut mm_struct {
    *rust_mprotect_vma_vm_mm(v)
}
#[inline]
unsafe fn start(v: *mut vm_area_struct) -> c_ulong {
    *rust_mprotect_vma_vm_start(v)
}
#[inline]
unsafe fn end(v: *mut vm_area_struct) -> c_ulong {
    *rust_mprotect_vma_vm_end(v)
}
#[inline]
unsafe fn test(v: *mut vm_area_struct, bit: vma_flag_t) -> bool {
    rust_mprotect_vma_test(v, bit)
}
#[inline]
unsafe fn flags_test(flags: *const vma_flags_t, bit: vma_flag_t) -> bool {
    rust_mprotect_flags_test(flags, bit)
}

unsafe fn maybe_change_pte_writable(vma: *mut vm_area_struct, pte: pte_t) -> bool {
    if rust_mprotect_warn_not_write(!test(vma, VMA_WRITE_BIT as _)) {
        return false;
    }
    if rust_mprotect_pte_protnone(pte) {
        return false;
    }
    if rust_mprotect_pte_needs_soft_dirty_wp(vma, pte) {
        return false;
    }
    if rust_mprotect_userfaultfd_pte_wp(vma, pte) {
        return false;
    }
    true
}

unsafe fn can_change_private_pte_writable(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pte: pte_t,
) -> bool {
    if !maybe_change_pte_writable(vma, pte) {
        return false;
    }
    let page = vm_normal_page(vma, addr, pte);
    !page.is_null() && rust_mprotect_page_anon(page) && rust_mprotect_page_anon_exclusive(page)
}

unsafe fn can_change_shared_pte_writable(vma: *mut vm_area_struct, pte: pte_t) -> bool {
    if !maybe_change_pte_writable(vma, pte) {
        return false;
    }
    #[cfg(CONFIG_DEBUG_VM)]
    rust_mprotect_warn_zero_dirty(
        rust_mprotect_is_zero_pfn(rust_mprotect_pte_pfn(pte)) && rust_mprotect_pte_dirty(pte),
    );
    rust_mprotect_pte_dirty(pte)
}

#[no_mangle]
pub unsafe extern "C" fn can_change_pte_writable(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pte: pte_t,
) -> bool {
    if !test(vma, VMA_SHARED_BIT as _) {
        return can_change_private_pte_writable(vma, addr, pte);
    }
    can_change_shared_pte_writable(vma, pte)
}

unsafe fn mprotect_folio_pte_batch(
    folio: *mut folio,
    ptep: *mut pte_t,
    mut pte: pte_t,
    max_nr_ptes: c_int,
    flags: fpb_t,
) -> c_int {
    if folio.is_null() || !rust_mprotect_folio_test_large(folio) {
        return 1;
    }
    rust_mprotect_folio_pte_batch_flags(folio, null_mut(), ptep, &mut pte, max_nr_ptes as _, flags)
        as c_int
}

#[inline(always)]
unsafe fn prot_commit_flush_ptes(
    vma: *mut vm_area_struct,
    mut addr: c_ulong,
    mut ptep: *mut pte_t,
    mut oldpte: pte_t,
    mut ptent: pte_t,
    nr_ptes: c_int,
    idx: c_int,
    set_write: bool,
    tlb: *mut mmu_gather,
) {
    addr = addr.wrapping_add((idx as c_ulong).wrapping_mul(PAGE_SIZE));
    ptep = ptep.add(idx as usize);
    oldpte = rust_mprotect_pte_advance_pfn(oldpte, idx as _);
    ptent = rust_mprotect_pte_advance_pfn(ptent, idx as _);
    if set_write {
        ptent = rust_mprotect_pte_mkwrite(ptent, vma);
    }
    rust_mprotect_modify_prot_commit_ptes(vma, addr, ptep, oldpte, ptent, nr_ptes);
    if rust_mprotect_pte_needs_flush(oldpte, ptent) {
        rust_mprotect_tlb_flush_pte_range(tlb, addr, (nr_ptes as c_ulong).wrapping_mul(PAGE_SIZE));
    }
}

#[inline(always)]
unsafe fn page_anon_exclusive_batch(
    start_idx: c_int,
    max_len: c_int,
    first_page: *mut page,
    expected: bool,
) -> c_int {
    let mut idx = start_idx + 1;
    while idx < start_idx + max_len {
        if expected != rust_mprotect_page_anon_exclusive(first_page.add(idx as usize)) {
            break;
        }
        idx += 1;
    }
    idx - start_idx
}

#[inline(always)]
unsafe fn commit_anon_folio_batch(
    vma: *mut vm_area_struct,
    folio: *mut folio,
    first_page: *mut page,
    addr: c_ulong,
    ptep: *mut pte_t,
    oldpte: pte_t,
    ptent: pte_t,
    mut nr_ptes: c_int,
    tlb: *mut mmu_gather,
) {
    let mut batch_idx = 0;
    while nr_ptes != 0 {
        let expected = rust_mprotect_page_anon_exclusive(first_page.add(batch_idx as usize));
        let len = page_anon_exclusive_batch(batch_idx, nr_ptes, first_page, expected);
        prot_commit_flush_ptes(
            vma, addr, ptep, oldpte, ptent, len, batch_idx, expected, tlb,
        );
        batch_idx += len;
        nr_ptes -= len;
    }
}

#[inline(always)]
unsafe fn set_write_prot_commit_flush_ptes(
    vma: *mut vm_area_struct,
    folio: *mut folio,
    page: *mut page,
    addr: c_ulong,
    ptep: *mut pte_t,
    oldpte: pte_t,
    ptent: pte_t,
    nr_ptes: c_int,
    tlb: *mut mmu_gather,
) {
    if test(vma, VMA_SHARED_BIT as _) {
        let set_write = can_change_shared_pte_writable(vma, ptent);
        prot_commit_flush_ptes(vma, addr, ptep, oldpte, ptent, nr_ptes, 0, set_write, tlb);
        return;
    }
    let set_write = maybe_change_pte_writable(vma, ptent)
        && !folio.is_null()
        && rust_mprotect_folio_test_anon(folio);
    if !set_write {
        prot_commit_flush_ptes(vma, addr, ptep, oldpte, ptent, nr_ptes, 0, set_write, tlb);
        return;
    }
    commit_anon_folio_batch(vma, folio, page, addr, ptep, oldpte, ptent, nr_ptes, tlb);
}

unsafe fn change_softleaf_pte(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pte: *mut pte_t,
    oldpte: pte_t,
    cp_flags: c_ulong,
) -> c_long {
    let uffd_prot = cp_flags & (MM_CP_UFFD_WP | MM_CP_UFFD_RWP) != 0;
    let uffd_prot_resolve = cp_flags & (MM_CP_UFFD_WP_RESOLVE | MM_CP_UFFD_RWP_RESOLVE) != 0;
    let mut entry = rust_mprotect_softleaf_from_pte(oldpte);
    let mut newpte;
    if rust_mprotect_softleaf_is_migration_write(entry) {
        let folio = rust_mprotect_softleaf_to_folio(entry);
        entry = if rust_mprotect_folio_test_anon(folio) {
            rust_mprotect_make_readable_exclusive_migration_entry(rust_mprotect_swp_offset(entry))
        } else {
            rust_mprotect_make_readable_migration_entry(rust_mprotect_swp_offset(entry))
        };
        newpte = rust_mprotect_swp_entry_to_pte(entry);
        if rust_mprotect_pte_swp_soft_dirty(oldpte) {
            newpte = rust_mprotect_pte_swp_mksoft_dirty(newpte);
        }
    } else if rust_mprotect_softleaf_is_device_private_write(entry) {
        entry = rust_mprotect_make_readable_device_private_entry(rust_mprotect_swp_offset(entry));
        newpte = rust_mprotect_swp_entry_to_pte(entry);
        if rust_mprotect_pte_swp_uffd(oldpte) {
            newpte = rust_mprotect_pte_swp_mkuffd(newpte);
        }
    } else if rust_mprotect_softleaf_is_marker(entry) {
        if rust_mprotect_softleaf_is_poison_marker(entry)
            || rust_mprotect_softleaf_is_guard_marker(entry)
        {
            return 0;
        }
        if uffd_prot_resolve {
            rust_mprotect_pte_clear(mm(vma), addr, pte);
            return 1;
        }
        return 0;
    } else {
        newpte = oldpte;
    }
    if uffd_prot {
        newpte = rust_mprotect_pte_swp_mkuffd(newpte);
    } else if uffd_prot_resolve {
        newpte = rust_mprotect_pte_swp_clear_uffd(newpte);
    }
    if !rust_mprotect_pte_same(oldpte, newpte) {
        rust_mprotect_set_pte_at(mm(vma), addr, pte, newpte);
        return 1;
    }
    0
}

#[inline(always)]
unsafe fn change_present_ptes(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    addr: c_ulong,
    ptep: *mut pte_t,
    nr_ptes: c_int,
    end: c_ulong,
    newprot: pgprot_t,
    folio: *mut folio,
    page: *mut page,
    cp_flags: c_ulong,
) {
    let uffd_prot = cp_flags & (MM_CP_UFFD_WP | MM_CP_UFFD_RWP) != 0;
    let uffd_prot_resolve = cp_flags & (MM_CP_UFFD_WP_RESOLVE | MM_CP_UFFD_RWP_RESOLVE) != 0;
    let oldpte = rust_mprotect_modify_prot_start_ptes(vma, addr, ptep, nr_ptes);
    let mut ptent = rust_mprotect_pte_modify(oldpte, newprot);
    if uffd_prot {
        ptent = rust_mprotect_pte_mkuffd(ptent);
    } else if uffd_prot_resolve {
        ptent = rust_mprotect_pte_clear_uffd(ptent);
    }
    // Preserve RWP's trap-on-any-access semantics after base protection updates.
    if rust_mprotect_userfaultfd_rwp(vma) && rust_mprotect_pte_uffd(ptent) {
        ptent = rust_mprotect_pte_modify(ptent, rust_mprotect_page_none());
    }
    if cp_flags & MM_CP_TRY_CHANGE_WRITABLE != 0 && !rust_mprotect_pte_write(ptent) {
        set_write_prot_commit_flush_ptes(vma, folio, page, addr, ptep, oldpte, ptent, nr_ptes, tlb);
    } else {
        prot_commit_flush_ptes(vma, addr, ptep, oldpte, ptent, nr_ptes, 0, false, tlb);
    }
}

unsafe fn change_pte_range(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    pmd: *mut pmd_t,
    mut addr: c_ulong,
    end: c_ulong,
    newprot: pgprot_t,
    cp_flags: c_ulong,
) -> c_long {
    let mut pages: c_long = 0;
    let prot_numa = cp_flags & MM_CP_PROT_NUMA != 0;
    let uffd_rwp = cp_flags & MM_CP_UFFD_RWP != 0;
    let uffd_wp = cp_flags & MM_CP_UFFD_WP != 0;
    let mut ptl = null_mut();
    rust_mprotect_tlb_change_page_size(tlb, PAGE_SIZE as _);
    let mut pte = rust_mprotect_pte_offset_map_lock(mm(vma), pmd, addr, &mut ptl);
    if pte.is_null() {
        return -(EAGAIN as c_long);
    }
    let is_private_single_threaded = prot_numa && rust_mprotect_vma_is_single_threaded_private(vma);
    rust_mprotect_flush_tlb_batched_pending(mm(vma));
    rust_mprotect_lazy_mmu_mode_enable();
    loop {
        let mut nr_ptes = 1;
        let oldpte = rust_mprotect_ptep_get(pte);
        // The labelled block preserves C do/while continue: every skipped entry
        // still advances both the address and the (possibly batched) PTE cursor.
        'entry: {
            if rust_mprotect_pte_present(oldpte) {
                let flags = (RUST_MPROTECT_FPB_RESPECT_SOFT_DIRTY | RUST_MPROTECT_FPB_RESPECT_WRITE)
                    as fpb_t;
                let max_nr_ptes = (end.wrapping_sub(addr) >> PAGE_SHIFT) as c_int;
                let mut folio = null_mut();
                if prot_numa && rust_mprotect_pte_protnone(oldpte) {
                    break 'entry;
                }
                if uffd_rwp && rust_mprotect_pte_protnone(oldpte) && rust_mprotect_pte_uffd(oldpte)
                {
                    break 'entry;
                }
                let page = vm_normal_page(vma, addr, oldpte);
                if !page.is_null() {
                    folio = rust_mprotect_page_folio(page);
                }
                if prot_numa
                    && !rust_mprotect_folio_can_map_prot_numa(
                        folio,
                        vma,
                        is_private_single_threaded,
                    )
                {
                    nr_ptes = mprotect_folio_pte_batch(folio, pte, oldpte, max_nr_ptes, 0);
                    break 'entry;
                }
                nr_ptes = mprotect_folio_pte_batch(folio, pte, oldpte, max_nr_ptes, flags);
                // Keep the source's constant-size fast path for small folios.
                if nr_ptes == 1 {
                    change_present_ptes(
                        tlb, vma, addr, pte, 1, end, newprot, folio, page, cp_flags,
                    );
                } else {
                    change_present_ptes(
                        tlb, vma, addr, pte, nr_ptes, end, newprot, folio, page, cp_flags,
                    );
                }
                pages += nr_ptes as c_long;
            } else if rust_mprotect_pte_none(oldpte) {
                if !uffd_wp {
                    break 'entry;
                }
                if rust_mprotect_userfaultfd_wp_use_markers(vma) {
                    rust_mprotect_set_pte_at(
                        mm(vma),
                        addr,
                        pte,
                        rust_mprotect_make_pte_marker(RUST_MPROTECT_PTE_MARKER_UFFD_WP as _),
                    );
                    pages += 1;
                }
            } else {
                pages += change_softleaf_pte(vma, addr, pte, oldpte, cp_flags);
            }
        }
        pte = pte.add(nr_ptes as usize);
        addr = addr.wrapping_add((nr_ptes as c_ulong).wrapping_mul(PAGE_SIZE));
        if addr == end {
            break;
        }
    }
    rust_mprotect_lazy_mmu_mode_disable();
    rust_mprotect_pte_unmap_unlock(pte.sub(1), ptl);
    pages
}

#[inline]
unsafe fn pgtable_split_needed(vma: *mut vm_area_struct, cp_flags: c_ulong) -> bool {
    cp_flags & (MM_CP_UFFD_WP | MM_CP_UFFD_RWP) != 0 && !rust_mprotect_vma_is_anonymous(vma)
}
#[inline]
unsafe fn pgtable_populate_needed(vma: *mut vm_area_struct, cp_flags: c_ulong) -> bool {
    if cp_flags & MM_CP_UFFD_WP == 0 {
        return false;
    }
    rust_mprotect_userfaultfd_wp_use_markers(vma)
}
#[inline]
unsafe fn change_pmd_prepare(
    vma: *mut vm_area_struct,
    pmd: *mut pmd_t,
    cp_flags: c_ulong,
) -> c_long {
    if pgtable_populate_needed(vma, cp_flags) && rust_mprotect_pte_alloc(mm(vma), pmd) != 0 {
        return -(ENOMEM as c_long);
    }
    0
}
// The original change_prepare token-pasting macro becomes three typed Rust
// bodies: allocation policy and error decisions remain in this owner.
#[inline]
unsafe fn change_prepare_pmd(
    vma: *mut vm_area_struct,
    pud: *mut pud_t,
    addr: c_ulong,
    cp_flags: c_ulong,
) -> c_long {
    if pgtable_populate_needed(vma, cp_flags)
        && rust_mprotect_pmd_alloc(mm(vma), pud, addr).is_null()
    {
        return -(ENOMEM as c_long);
    }
    0
}
#[inline]
unsafe fn change_prepare_pud(
    vma: *mut vm_area_struct,
    p4d: *mut p4d_t,
    addr: c_ulong,
    cp_flags: c_ulong,
) -> c_long {
    if pgtable_populate_needed(vma, cp_flags)
        && rust_mprotect_pud_alloc(mm(vma), p4d, addr).is_null()
    {
        return -(ENOMEM as c_long);
    }
    0
}
#[inline]
unsafe fn change_prepare_p4d(
    vma: *mut vm_area_struct,
    pgd: *mut pgd_t,
    addr: c_ulong,
    cp_flags: c_ulong,
) -> c_long {
    if pgtable_populate_needed(vma, cp_flags)
        && rust_mprotect_p4d_alloc(mm(vma), pgd, addr).is_null()
    {
        return -(ENOMEM as c_long);
    }
    0
}

#[inline]
unsafe fn change_pmd_range(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    pud: *mut pud_t,
    mut addr: c_ulong,
    end: c_ulong,
    newprot: pgprot_t,
    cp_flags: c_ulong,
) -> c_long {
    let mut pages: c_long = 0;
    let mut nr_huge_updates: c_ulong = 0;
    let mut pmd = rust_mprotect_pmd_offset(pud, addr);
    'range: loop {
        let next = rust_mprotect_pmd_addr_end(addr, end);
        loop {
            let ret = change_pmd_prepare(vma, pmd, cp_flags);
            if ret != 0 {
                pages = ret;
                break 'range;
            }
            if rust_mprotect_pmd_none(*pmd) {
                break;
            }
            let entry = rust_mprotect_pmdp_get_lockless(pmd);
            // Native pmd_is_huge() is always false without THP.
            #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
            if rust_mprotect_pmd_is_huge(entry) {
                if next.wrapping_sub(addr) != RUST_MPROTECT_HPAGE_PMD_SIZE as c_ulong
                    || pgtable_split_needed(vma, cp_flags)
                {
                    rust_mprotect_split_huge_pmd(vma, pmd, addr, false);
                    let ret = change_pmd_prepare(vma, pmd, cp_flags);
                    if ret != 0 {
                        pages = ret;
                        break 'range;
                    }
                } else {
                    let ret = rust_mprotect_change_huge_pmd(tlb, vma, pmd, addr, newprot, cp_flags);
                    if ret != 0 {
                        if ret == RUST_MPROTECT_HPAGE_PMD_NR as c_long {
                            pages += RUST_MPROTECT_HPAGE_PMD_NR as c_long;
                            nr_huge_updates += 1;
                        }
                        break;
                    }
                }
            }
            let ret = change_pte_range(tlb, vma, pmd, addr, next, newprot, cp_flags);
            if ret < 0 {
                continue;
            }
            pages += ret;
            break;
        }
        rust_mprotect_cond_resched();
        pmd = pmd.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    if nr_huge_updates != 0 {
        rust_mprotect_count_vm_numa_huge_updates(nr_huge_updates);
    }
    pages
}

#[inline]
unsafe fn change_pud_range(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    p4d: *mut p4d_t,
    mut addr: c_ulong,
    end: c_ulong,
    newprot: pgprot_t,
    cp_flags: c_ulong,
) -> c_long {
    let mut range: mmu_notifier_range = zeroed();
    let mut pages: c_long = 0;
    let mut pudp = rust_mprotect_pud_offset(p4d, addr);
    'range: loop {
        let next = rust_mprotect_pud_addr_end(addr, end);
        loop {
            let ret = change_prepare_pmd(vma, pudp, addr, cp_flags);
            if ret != 0 {
                pages = ret;
                break 'range;
            }
            let pud = rust_mprotect_pudp_get(pudp);
            if rust_mprotect_pud_none(pud) {
                break;
            }
            if range.start == 0 {
                rust_mprotect_mmu_notifier_range_init(&mut range, mm(vma), addr, end);
                rust_mprotect_mmu_notifier_invalidate_range_start(&mut range);
            }
            #[cfg(CONFIG_PGTABLE_HAS_HUGE_LEAVES)]
            if rust_mprotect_pud_leaf(pud) {
                if next.wrapping_sub(addr) != RUST_MPROTECT_PUD_SIZE as c_ulong
                    || pgtable_split_needed(vma, cp_flags)
                {
                    rust_mprotect_split_huge_pud(vma, pudp, addr);
                    continue;
                }
                let ret = rust_mprotect_change_huge_pud(tlb, vma, pudp, addr, newprot, cp_flags);
                if ret == 0 {
                    continue;
                }
                if ret == RUST_MPROTECT_HPAGE_PUD_NR as c_long {
                    pages += RUST_MPROTECT_HPAGE_PUD_NR as c_long;
                }
                break;
            }
            pages += change_pmd_range(tlb, vma, pudp, addr, next, newprot, cp_flags);
            break;
        }
        pudp = pudp.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    if range.start != 0 {
        rust_mprotect_mmu_notifier_invalidate_range_end(&mut range);
    }
    pages
}

#[inline]
unsafe fn change_p4d_range(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    pgd: *mut pgd_t,
    mut addr: c_ulong,
    end: c_ulong,
    newprot: pgprot_t,
    cp_flags: c_ulong,
) -> c_long {
    let mut pages: c_long = 0;
    let mut p4d = rust_mprotect_p4d_offset(pgd, addr);
    loop {
        let next = rust_mprotect_p4d_addr_end(addr, end);
        let ret = change_prepare_pud(vma, p4d, addr, cp_flags);
        if ret != 0 {
            return ret;
        }
        if !rust_mprotect_p4d_none_or_clear_bad(p4d) {
            pages += change_pud_range(tlb, vma, p4d, addr, next, newprot, cp_flags);
        }
        p4d = p4d.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    pages
}

unsafe fn change_protection_range(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    mut addr: c_ulong,
    end: c_ulong,
    newprot: pgprot_t,
    cp_flags: c_ulong,
) -> c_long {
    let mut pages: c_long = 0;
    rust_mprotect_bug_bad_range(addr >= end);
    let mut pgd = rust_mprotect_pgd_offset(mm(vma), addr);
    rust_mprotect_tlb_start_vma(tlb, vma);
    loop {
        let next = rust_mprotect_pgd_addr_end(addr, end);
        let ret = change_prepare_p4d(vma, pgd, addr, cp_flags);
        if ret != 0 {
            pages = ret;
            break;
        }
        if !rust_mprotect_pgd_none_or_clear_bad(pgd) {
            pages += change_p4d_range(tlb, vma, pgd, addr, next, newprot, cp_flags);
        }
        pgd = pgd.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    rust_mprotect_tlb_end_vma(tlb, vma);
    pages
}

#[no_mangle]
pub unsafe extern "C" fn change_protection(
    tlb: *mut mmu_gather,
    vma: *mut vm_area_struct,
    start: c_ulong,
    end: c_ulong,
    cp_flags: c_ulong,
) -> c_long {
    let mut newprot = *rust_mprotect_vma_vm_page_prot(vma);
    if rust_mprotect_warn_uffd_conflict(
        cp_flags & MM_CP_UFFD_WP_ALL == MM_CP_UFFD_WP_ALL
            || cp_flags & MM_CP_UFFD_RWP_ALL == MM_CP_UFFD_RWP_ALL
            || ((cp_flags & MM_CP_UFFD_WP_ALL != 0) && (cp_flags & MM_CP_UFFD_RWP_ALL != 0)),
    ) {
        return 0;
    }
    #[cfg(CONFIG_NUMA_BALANCING)]
    if cp_flags & MM_CP_PROT_NUMA != 0 {
        newprot = rust_mprotect_page_none();
    }
    #[cfg(not(CONFIG_NUMA_BALANCING))]
    rust_mprotect_warn_numa_disabled(cp_flags & MM_CP_PROT_NUMA != 0);
    #[cfg(CONFIG_ARCH_HAS_PTE_PROTNONE)]
    if cp_flags & MM_CP_UFFD_RWP != 0 {
        newprot = rust_mprotect_page_none();
    }
    if rust_mprotect_is_vm_hugetlb_page(vma) {
        rust_mprotect_hugetlb_change_protection(vma, start, end, newprot, cp_flags)
    } else {
        change_protection_range(tlb, vma, start, end, newprot, cp_flags)
    }
}

unsafe extern "C" fn prot_none_pte_entry(
    pte: *mut pte_t,
    addr: c_ulong,
    next: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    if rust_mprotect_pfn_modify_allowed(
        rust_mprotect_pte_pfn(rust_mprotect_ptep_get(pte)),
        *((*walk).private as *const pgprot_t),
    ) {
        0
    } else {
        -(EACCES as c_int)
    }
}
#[cfg(CONFIG_HUGETLB_PAGE)]
unsafe extern "C" fn prot_none_hugetlb_entry(
    pte: *mut pte_t,
    hmask: c_ulong,
    addr: c_ulong,
    next: c_ulong,
    walk: *mut mm_walk,
) -> c_int {
    let entry = rust_mprotect_huge_ptep_get((*walk).mm, addr, pte);
    if rust_mprotect_pfn_modify_allowed(
        rust_mprotect_pte_pfn(entry),
        *((*walk).private as *const pgprot_t),
    ) {
        0
    } else {
        -(EACCES as c_int)
    }
}
static prot_none_walk_ops: mm_walk_ops = {
    let mut ops: mm_walk_ops = unsafe { zeroed() };
    ops.pte_entry = Some(prot_none_pte_entry);
    #[cfg(CONFIG_HUGETLB_PAGE)]
    {
        ops.hugetlb_entry = Some(prot_none_hugetlb_entry);
    }
    ops.walk_lock = PGWALK_WRLOCK;
    ops
};

#[no_mangle]
pub unsafe extern "C" fn mprotect_fixup(
    vmi: *mut vma_iterator,
    tlb: *mut mmu_gather,
    mut vma: *mut vm_area_struct,
    pprev: *mut *mut vm_area_struct,
    start: c_ulong,
    end: c_ulong,
    mut newflags: vm_flags_t,
) -> c_int {
    let mm = mm(vma);
    let old_vma_flags = rust_mprotect_flags_read_once(vma);
    let mut new_vma_flags = rust_mprotect_legacy_to_flags(newflags);
    let nrpages = (end.wrapping_sub(start) >> PAGE_SHIFT) as c_long;
    let mut mm_cp_flags: c_ulong = 0;
    let mut charged: c_ulong = 0;
    if rust_mprotect_vma_is_sealed(vma) {
        return -(EPERM as c_int);
    }
    if rust_mprotect_flags_same_pair(&old_vma_flags, &new_vma_flags) {
        *pprev = vma;
        return 0;
    }
    if rust_mprotect_arch_has_pfn_modify_check()
        && (flags_test(&old_vma_flags, VMA_PFNMAP_BIT as _)
            || flags_test(&old_vma_flags, VMA_MIXEDMAP_BIT as _))
        && !rust_mprotect_flags_any_access(&new_vma_flags)
    {
        let mut new_pgprot = vm_get_page_prot(newflags);
        let error = walk_page_range_vma(
            vma,
            start,
            end,
            &prot_none_walk_ops,
            &mut new_pgprot as *mut _ as *mut c_void,
        );
        if error != 0 {
            return error;
        }
    }
    if flags_test(&new_vma_flags, VMA_WRITE_BIT as _) {
        if !may_expand_vm(mm, &new_vma_flags, nrpages as _)
            && may_expand_vm(mm, &old_vma_flags, nrpages as _)
        {
            return -(ENOMEM as c_int);
        }
        if !(flags_test(&old_vma_flags, VMA_ACCOUNT_BIT as _)
            || flags_test(&old_vma_flags, VMA_WRITE_BIT as _)
            || flags_test(&old_vma_flags, VMA_HUGETLB_BIT as _)
            || flags_test(&old_vma_flags, VMA_SHARED_BIT as _)
            || flags_test(&old_vma_flags, VMA_NORESERVE_BIT as _))
        {
            charged = nrpages as c_ulong;
            if rust_mprotect_security_vm_enough_memory_mm(mm, charged as _) != 0 {
                return -(ENOMEM as c_int);
            }
            rust_mprotect_flags_set(&mut new_vma_flags, VMA_ACCOUNT_BIT as _);
        }
    } else if flags_test(&old_vma_flags, VMA_ACCOUNT_BIT as _)
        && rust_mprotect_vma_is_anonymous(vma)
        && (*rust_mprotect_vma_anon_vma(vma)).is_null()
    {
        rust_mprotect_flags_clear(&mut new_vma_flags, VMA_ACCOUNT_BIT as _);
    }
    vma = vma_modify_flags(vmi, *pprev, vma, start, end, &mut new_vma_flags);
    if rust_mprotect_is_err(vma.cast()) {
        let error = rust_mprotect_ptr_err(vma.cast()) as c_int;
        rust_mprotect_vm_unacct_memory(charged as c_long);
        return error;
    }
    *pprev = vma;
    rust_mprotect_vma_start_write(vma);
    rust_mprotect_vma_flags_reset_once(vma, &mut new_vma_flags);
    if rust_mprotect_vma_wants_manual_pte_write_upgrade(vma) {
        mm_cp_flags |= MM_CP_TRY_CHANGE_WRITABLE;
    }
    vma_set_page_prot(vma);
    change_protection(tlb, vma, start, end, mm_cp_flags);
    if flags_test(&old_vma_flags, VMA_ACCOUNT_BIT as _)
        && !flags_test(&new_vma_flags, VMA_ACCOUNT_BIT as _)
    {
        rust_mprotect_vm_unacct_memory(nrpages);
    }
    if flags_test(&new_vma_flags, VMA_WRITE_BIT as _)
        && flags_test(&old_vma_flags, VMA_LOCKED_BIT as _)
        && !(flags_test(&old_vma_flags, VMA_WRITE_BIT as _)
            || flags_test(&old_vma_flags, VMA_SHARED_BIT as _))
    {
        populate_vma_page_range(vma, start, end, null_mut());
    }
    vm_stat_account(mm, rust_mprotect_flags_to_legacy(old_vma_flags), -nrpages);
    newflags = rust_mprotect_flags_to_legacy(new_vma_flags);
    vm_stat_account(mm, newflags, nrpages);
    rust_mprotect_perf_event_mmap(vma);
    0
}

unsafe fn do_mprotect_pkey(
    mut start: c_ulong,
    mut len: usize,
    mut prot: c_ulong,
    pkey: c_int,
) -> c_int {
    let grows = prot & ((PROT_GROWSDOWN | PROT_GROWSUP) as c_ulong);
    let rier = rust_mprotect_personality() & READ_IMPLIES_EXEC as u32 != 0
        && prot & PROT_READ as c_ulong != 0;
    start = rust_mprotect_untagged_addr(start);
    prot &= !((PROT_GROWSDOWN | PROT_GROWSUP) as c_ulong);
    if grows == (PROT_GROWSDOWN | PROT_GROWSUP) as c_ulong {
        return -(EINVAL as c_int);
    }
    if start & !PAGE_MASK != 0 {
        return -(EINVAL as c_int);
    }
    if len == 0 {
        return 0;
    }
    len = (len as c_ulong).wrapping_add(PAGE_SIZE - 1) as usize & PAGE_MASK as usize;
    let mut end = start.wrapping_add(len as c_ulong);
    if end <= start {
        return -(ENOMEM as c_int);
    }
    if !rust_mprotect_arch_validate_prot(prot, start) {
        return -(EINVAL as c_int);
    }
    let reqprot = prot;
    let mm = rust_mprotect_current_mm();
    if rust_mprotect_mmap_write_lock_killable(mm) != 0 {
        return -(EINTR as c_int);
    }
    let result = 'locked: {
        if pkey != -1 && !rust_mprotect_mm_pkey_is_allocated(mm, pkey) {
            break 'locked -(EINVAL as c_int);
        }
        let mut vmi = MaybeUninit::<vma_iterator>::uninit();
        rust_mprotect_vma_iter_init(vmi.as_mut_ptr(), mm, start);
        let vmi = vmi.as_mut_ptr();
        let mut vma = rust_mprotect_vma_find(vmi, end);
        if vma.is_null() {
            break 'locked -(ENOMEM as c_int);
        }
        if grows & PROT_GROWSDOWN as c_ulong != 0 {
            if self::start(vma) >= end {
                break 'locked -(ENOMEM as c_int);
            }
            start = self::start(vma);
            if !test(vma, VMA_GROWSDOWN_BIT as _) {
                break 'locked -(EINVAL as c_int);
            }
        } else {
            if self::start(vma) > start {
                break 'locked -(ENOMEM as c_int);
            }
            if grows & PROT_GROWSUP as c_ulong != 0 {
                end = self::end(vma);
                if !rust_mprotect_vma_growsup(vma) {
                    break 'locked -(EINVAL as c_int);
                }
            }
        }
        let mut prev = rust_mprotect_vma_prev(vmi);
        if start > self::start(vma) {
            prev = vma;
        }
        let mut tlb = MaybeUninit::<mmu_gather>::uninit();
        rust_mprotect_tlb_gather_mmu(tlb.as_mut_ptr(), mm);
        // Keep the initialized object at its stable address through finish.
        let tlb = tlb.as_mut_ptr();
        let mut nstart = start;
        let mut tmp = self::start(vma);
        let mut error = if grows != 0 {
            -(EINVAL as c_int)
        } else {
            -(ENOMEM as c_int)
        };
        loop {
            vma = rust_mprotect_vma_find(vmi, end);
            if vma.is_null() {
                break;
            }
            if self::start(vma) != tmp {
                error = -(ENOMEM as c_int);
                break;
            }
            if rier && test(vma, VMA_MAYEXEC_BIT as _) {
                prot |= PROT_EXEC as c_ulong;
            }
            let mask_off_old_flags =
                (RUST_MPROTECT_VM_ACCESS_FLAGS | RUST_MPROTECT_VM_FLAGS_CLEAR) as vm_flags_t;
            let new_vma_pkey = rust_mprotect_arch_override_mprotect_pkey(vma, prot, pkey);
            let mut newflags = rust_mprotect_calc_vm_prot_bits(prot, new_vma_pkey as _);
            newflags |= *rust_mprotect_vma_vm_flags(vma) & !mask_off_old_flags;
            let new_vma_flags = rust_mprotect_legacy_to_flags(newflags);
            if (newflags & !(newflags >> 4)) & RUST_MPROTECT_VM_ACCESS_FLAGS as vm_flags_t != 0 {
                error = -(EACCES as c_int);
                break;
            }
            if rust_mprotect_map_deny_write_exec(rust_mprotect_vma_flags(vma), &new_vma_flags) {
                error = -(EACCES as c_int);
                break;
            }
            if !rust_mprotect_arch_validate_flags(newflags) {
                error = -(EINVAL as c_int);
                break;
            }
            error = rust_mprotect_security_file_mprotect(vma, reqprot, prot);
            if error != 0 {
                break;
            }
            tmp = self::end(vma);
            if tmp > end {
                tmp = end;
            }
            let ops = *rust_mprotect_vma_vm_ops(vma);
            if !ops.is_null() {
                if let Some(mprotect) = (*ops).mprotect {
                    error = mprotect(vma, nstart, tmp, newflags);
                    if error != 0 {
                        break;
                    }
                }
            }
            error = mprotect_fixup(vmi, tlb, vma, &mut prev, nstart, tmp, newflags);
            if error != 0 {
                break;
            }
            tmp = rust_mprotect_vma_iter_end(vmi);
            nstart = tmp;
            prot = reqprot;
        }
        rust_mprotect_tlb_finish_mmu(tlb);
        if error == 0 && tmp < end {
            error = -(ENOMEM as c_int);
        }
        error
    };
    rust_mprotect_mmap_write_unlock(mm);
    result
}

#[no_mangle]
pub unsafe extern "C" fn rust_mprotect_sys_mprotect(
    start: c_ulong,
    len: usize,
    prot: c_ulong,
) -> c_long {
    do_mprotect_pkey(start, len, prot, -1) as c_long
}
#[cfg(CONFIG_ARCH_HAS_PKEYS)]
#[no_mangle]
pub unsafe extern "C" fn rust_mprotect_sys_pkey_mprotect(
    start: c_ulong,
    len: usize,
    prot: c_ulong,
    pkey: c_int,
) -> c_long {
    do_mprotect_pkey(start, len, prot, pkey) as c_long
}
#[cfg(CONFIG_ARCH_HAS_PKEYS)]
#[no_mangle]
pub unsafe extern "C" fn rust_mprotect_sys_pkey_alloc(flags: c_ulong, init_val: c_ulong) -> c_long {
    if flags != 0 {
        return -(EINVAL as c_long);
    }
    if init_val & !(RUST_MPROTECT_PKEY_ACCESS_MASK as c_ulong) != 0 {
        return -(EINVAL as c_long);
    }
    let mm = rust_mprotect_current_mm();
    rust_mprotect_mmap_write_lock(mm);
    let pkey = rust_mprotect_mm_pkey_alloc(mm);
    let ret = if pkey == -1 {
        -(ENOSPC as c_int)
    } else {
        let ret = rust_mprotect_arch_set_user_pkey_access(pkey, init_val);
        if ret != 0 {
            rust_mprotect_mm_pkey_free(mm, pkey);
            ret
        } else {
            pkey
        }
    };
    rust_mprotect_mmap_write_unlock(mm);
    ret as c_long
}
#[cfg(CONFIG_ARCH_HAS_PKEYS)]
#[no_mangle]
pub unsafe extern "C" fn rust_mprotect_sys_pkey_free(pkey: c_int) -> c_long {
    let mm = rust_mprotect_current_mm();
    rust_mprotect_mmap_write_lock(mm);
    let ret = rust_mprotect_mm_pkey_free(mm, pkey);
    rust_mprotect_mmap_write_unlock(mm);
    ret as c_long
}
