// SPDX-License-Identifier: GPL-2.0-only
// mm/memory.c: walk_to_pmd through apply_to_existing_page_range.
unsafe fn walk_to_pmd(mm: *mut mm_struct, addr: c_ulong) -> *mut pmd_t {
    let pgd = rust_memory_pgd_offset(mm, addr);
    let p4d = rust_memory_p4d_alloc(mm, pgd, addr);
    if p4d.is_null() {
        return null_mut();
    }
    let pud = rust_memory_pud_alloc(mm, p4d, addr);
    if pud.is_null() {
        return null_mut();
    }
    let pmd = rust_memory_pmd_alloc(mm, pud, addr);
    if pmd.is_null() {
        return null_mut();
    }
    #[cfg(CONFIG_DEBUG_VM)]
    rust_memory_bug_walk_huge(rust_memory_pmd_trans_huge(*pmd));
    pmd
}
#[no_mangle]
pub unsafe extern "C" fn get_locked_pte(
    mm: *mut mm_struct,
    addr: c_ulong,
    ptl: *mut *mut spinlock_t,
) -> *mut pte_t {
    let pmd = walk_to_pmd(mm, addr);
    if pmd.is_null() {
        return null_mut();
    }
    rust_memory_pte_alloc_map_lock(mm, pmd, addr, ptl)
}
unsafe fn vm_mixed_zeropage_allowed(vma: *mut vm_area_struct) -> bool {
    let flags = rust_memory_vma_vm_flags(vma);
    #[cfg(CONFIG_DEBUG_VM)]
    rust_memory_warn_mixed_pfnmap(flags & RUST_MEMORY_VM_PFNMAP != 0);
    if rust_memory_mm_forbids_zeropage((*vma).vm_mm) {
        return false;
    }
    if rust_memory_vma_is_cow_mapping(vma) {
        return true;
    }
    if flags & (RUST_MEMORY_VM_WRITE | RUST_MEMORY_VM_MAYWRITE) == 0 {
        return true;
    }
    !(*vma).vm_ops.is_null()
        && (*(*vma).vm_ops).pfn_mkwrite.is_some()
        && (rust_memory_vma_is_fsdax(vma) || flags & RUST_MEMORY_VM_IO != 0)
}
unsafe fn validate_page_before_insert(vma: *mut vm_area_struct, page: *mut page) -> c_int {
    let folio = rust_memory_page_folio(page);
    if rust_memory_folio_ref_count(folio) == 0 {
        return -(EINVAL as c_int);
    }
    if rust_memory_is_zero_folio(folio) {
        return if vm_mixed_zeropage_allowed(vma) {
            0
        } else {
            -(EINVAL as c_int)
        };
    }
    if rust_memory_folio_test_anon(folio) || rust_memory_page_has_type(page) {
        return -(EINVAL as c_int);
    }
    rust_memory_flush_dcache_folio(folio);
    0
}
unsafe fn insert_page_into_pte_locked(
    vma: *mut vm_area_struct,
    pte: *mut pte_t,
    addr: c_ulong,
    page: *mut page,
    prot: pgprot_t,
    mkwrite: bool,
) -> c_int {
    let folio = rust_memory_page_folio(page);
    let mut pteval = rust_memory_ptep_get(pte);
    if !rust_memory_pte_none(pteval) {
        if !mkwrite {
            return -(EBUSY as c_int);
        }
        if rust_memory_pte_pfn(pteval) != rust_memory_page_to_pfn(page) {
            rust_memory_warn_insert_pfn_mismatch(!rust_memory_is_zero_pfn(rust_memory_pte_pfn(
                pteval,
            )));
            return -(EFAULT as c_int);
        }
        pteval = rust_memory_maybe_mkwrite(pteval, vma);
        pteval = rust_memory_pte_mkyoung(pteval);
        if rust_memory_ptep_set_access_flags(vma, addr, pte, pteval, 1) != 0 {
            rust_memory_update_mmu_cache(vma, addr, pte);
        }
        return 0;
    }
    pteval = rust_memory_mk_pte(page, prot);
    if rust_memory_is_zero_folio(folio) {
        pteval = rust_memory_pte_mkspecial(pteval);
    } else {
        rust_memory_folio_get(folio);
        pteval = rust_memory_mk_pte(page, prot);
        if mkwrite {
            pteval = rust_memory_pte_mkyoung(pteval);
            pteval = rust_memory_maybe_mkwrite(rust_memory_pte_mkdirty(pteval), vma);
        }
        rust_memory_inc_mm_counter((*vma).vm_mm, rust_memory_mm_counter_file(folio));
        rust_memory_folio_add_file_rmap_pte(folio, page, vma);
    }
    rust_memory_set_pte_at((*vma).vm_mm, addr, pte, pteval);
    0
}
unsafe fn insert_page(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    page: *mut page,
    prot: pgprot_t,
    mkwrite: bool,
) -> c_int {
    let ret = validate_page_before_insert(vma, page);
    if ret != 0 {
        return ret;
    }
    let mut ptl = null_mut();
    let pte = get_locked_pte((*vma).vm_mm, addr, &mut ptl);
    if pte.is_null() {
        return -(ENOMEM as c_int);
    }
    let ret = insert_page_into_pte_locked(vma, pte, addr, page, prot, mkwrite);
    rust_memory_pte_unmap_unlock(pte, ptl);
    ret
}
unsafe fn insert_page_in_batch_locked(
    vma: *mut vm_area_struct,
    pte: *mut pte_t,
    addr: c_ulong,
    page: *mut page,
    prot: pgprot_t,
) -> c_int {
    let err = validate_page_before_insert(vma, page);
    if err != 0 {
        return err;
    }
    insert_page_into_pte_locked(vma, pte, addr, page, prot, false)
}
unsafe fn insert_pages(
    vma: *mut vm_area_struct,
    mut addr: c_ulong,
    pages: *mut *mut page,
    num: *mut c_ulong,
    prot: pgprot_t,
) -> c_int {
    let mm = (*vma).vm_mm;
    let mut idx: c_ulong = 0;
    let mut remaining = *num;
    let ret = 'more: loop {
        let pmd = walk_to_pmd(mm, addr);
        if pmd.is_null() {
            break -(EFAULT as c_int);
        }
        let mut in_pmd = core::cmp::min(
            remaining,
            (RUST_MEMORY_PTRS_PER_PTE as c_ulong).wrapping_sub(rust_memory_pte_index(addr)),
        );
        if rust_memory_pte_alloc(mm, pmd) != 0 {
            break -(ENOMEM as c_int);
        }
        while in_pmd != 0 {
            let batch = core::cmp::min(in_pmd as c_int, 8);
            let mut ptl = null_mut();
            let start_pte = rust_memory_pte_offset_map_lock(mm, pmd, addr, &mut ptl);
            if start_pte.is_null() {
                break 'more -(EFAULT as c_int);
            }
            let mut pte_idx = 0;
            while pte_idx < batch {
                let err = insert_page_in_batch_locked(
                    vma,
                    start_pte.offset(pte_idx as isize),
                    addr,
                    *pages.add(idx as usize),
                    prot,
                );
                if err != 0 {
                    rust_memory_pte_unmap_unlock(start_pte, ptl);
                    remaining = remaining.wrapping_sub(pte_idx as c_ulong);
                    break 'more err;
                }
                addr = addr.wrapping_add(PAGE_SIZE);
                idx = idx.wrapping_add(1);
                pte_idx += 1;
            }
            rust_memory_pte_unmap_unlock(start_pte, ptl);
            in_pmd = in_pmd.wrapping_sub(batch as c_ulong);
            remaining = remaining.wrapping_sub(batch as c_ulong);
        }
        if remaining == 0 {
            break 0;
        }
    };
    *num = remaining;
    ret
}
#[no_mangle]
pub unsafe extern "C" fn vm_insert_pages(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pages: *mut *mut page,
    num: *mut c_ulong,
) -> c_int {
    let end = addr.wrapping_add(PAGE_SIZE.wrapping_mul(*num));
    if !rust_memory_range_in_vma(vma, addr, end) {
        return -(EFAULT as c_int);
    }
    if rust_memory_vma_vm_flags(vma) & RUST_MEMORY_VM_MIXEDMAP == 0 {
        #[cfg(CONFIG_DEBUG_VM)]
        rust_memory_warn_insert_mmap_lock(rust_memory_mmap_read_trylock((*vma).vm_mm));
        #[cfg(CONFIG_DEBUG_VM)]
        rust_memory_warn_insert_pages_pfnmap(
            rust_memory_vma_vm_flags(vma) & RUST_MEMORY_VM_PFNMAP != 0,
        );
        rust_memory_vm_flags_set(vma, RUST_MEMORY_VM_MIXEDMAP);
    }
    insert_pages(vma, addr, pages, num, (*vma).vm_page_prot)
}
#[no_mangle]
pub unsafe extern "C" fn map_kernel_pages_prepare(desc: *mut vm_area_desc) -> c_int {
    let action = addr_of_mut!((*desc).action);
    let addr = rust_memory_action_kernel_start(action);
    if !rust_memory_vma_desc_test(desc, VMA_MIXEDMAP_BIT as _) {
        #[cfg(CONFIG_DEBUG_VM)]
        rust_memory_warn_kernel_mmap_lock(rust_memory_mmap_read_trylock((*desc).mm));
        #[cfg(CONFIG_DEBUG_VM)]
        rust_memory_warn_kernel_pfnmap(rust_memory_vma_desc_test(desc, VMA_PFNMAP_BIT as _));
        rust_memory_vma_desc_set_flags(desc, VMA_MIXEDMAP_BIT as _);
    }
    let end = addr.wrapping_add(PAGE_SIZE.wrapping_mul(rust_memory_action_kernel_nr_pages(action)));
    if !rust_memory_range_in_vma_desc(desc, addr, end) {
        return -(EFAULT as c_int);
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn map_kernel_pages_complete(
    vma: *mut vm_area_struct,
    action: *mut mmap_action,
) -> c_int {
    let mut nr = rust_memory_action_kernel_nr_pages(action);
    insert_pages(
        vma,
        rust_memory_action_kernel_start(action),
        rust_memory_action_kernel_pages(action),
        &mut nr,
        (*vma).vm_page_prot,
    )
}
#[no_mangle]
pub unsafe extern "C" fn vm_insert_page(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    page: *mut page,
) -> c_int {
    if addr < rust_memory_vma_start(vma) || addr >= rust_memory_vma_end(vma) {
        return -(EFAULT as c_int);
    }
    if rust_memory_vma_vm_flags(vma) & RUST_MEMORY_VM_MIXEDMAP == 0 {
        rust_memory_bug_insert_mmap_lock(rust_memory_mmap_read_trylock((*vma).vm_mm));
        rust_memory_bug_insert_pfnmap(rust_memory_vma_vm_flags(vma) & RUST_MEMORY_VM_PFNMAP != 0);
        rust_memory_vm_flags_set(vma, RUST_MEMORY_VM_MIXEDMAP);
    }
    insert_page(vma, addr, page, (*vma).vm_page_prot, false)
}
unsafe fn __vm_map_pages(
    vma: *mut vm_area_struct,
    pages: *mut *mut page,
    num: c_ulong,
    offset: c_ulong,
) -> c_int {
    let mut count = rust_memory_vma_pages(vma);
    if offset >= num || count > num.wrapping_sub(offset) {
        return -(ENXIO as c_int);
    }
    vm_insert_pages(
        vma,
        rust_memory_vma_start(vma),
        pages.add(offset as usize),
        &mut count,
    )
}
#[no_mangle]
pub unsafe extern "C" fn vm_map_pages(
    vma: *mut vm_area_struct,
    pages: *mut *mut page,
    num: c_ulong,
) -> c_int {
    __vm_map_pages(vma, pages, num, rust_memory_vma_start_pgoff(vma))
}
#[no_mangle]
pub unsafe extern "C" fn vm_map_pages_zero(
    vma: *mut vm_area_struct,
    pages: *mut *mut page,
    num: c_ulong,
) -> c_int {
    __vm_map_pages(vma, pages, num, 0)
}
unsafe fn insert_pfn(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pfn: c_ulong,
    prot: pgprot_t,
    mkwrite: bool,
) -> vm_fault_t {
    let mm = (*vma).vm_mm;
    let mut ptl = null_mut();
    let pte = get_locked_pte(mm, addr, &mut ptl);
    if pte.is_null() {
        return RUST_MEMORY_VM_FAULT_OOM as _;
    }
    let mut entry = rust_memory_ptep_get(pte);
    if !rust_memory_pte_none(entry) {
        if mkwrite {
            if rust_memory_pte_pfn(entry) != pfn {
                rust_memory_warn_pfn_insert_mismatch(!rust_memory_is_zero_pfn(
                    rust_memory_pte_pfn(entry),
                ));
            } else {
                entry = rust_memory_pte_mkyoung(entry);
                entry = rust_memory_maybe_mkwrite(rust_memory_pte_mkdirty(entry), vma);
                if rust_memory_ptep_set_access_flags(vma, addr, pte, entry, 1) != 0 {
                    rust_memory_update_mmu_cache(vma, addr, pte);
                }
            }
        }
    } else {
        entry = rust_memory_pte_mkspecial(rust_memory_pfn_pte(pfn, prot));
        if mkwrite {
            entry = rust_memory_pte_mkyoung(entry);
            entry = rust_memory_maybe_mkwrite(rust_memory_pte_mkdirty(entry), vma);
        }
        rust_memory_set_pte_at(mm, addr, pte, entry);
        rust_memory_update_mmu_cache(vma, addr, pte);
    }
    rust_memory_pte_unmap_unlock(pte, ptl);
    RUST_MEMORY_VM_FAULT_NOPAGE as _
}
#[no_mangle]
pub unsafe extern "C" fn vmf_insert_pfn_prot(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pfn: c_ulong,
    mut pgprot: pgprot_t,
) -> vm_fault_t {
    let flags = rust_memory_vma_vm_flags(vma);
    let both = RUST_MEMORY_VM_PFNMAP | RUST_MEMORY_VM_MIXEDMAP;
    rust_memory_bug_pfn_insert_no_flag(flags & both == 0);
    rust_memory_bug_pfn_insert_both(flags & both == both);
    rust_memory_bug_pfn_insert_cow(
        flags & RUST_MEMORY_VM_PFNMAP != 0 && rust_memory_vma_is_cow_mapping(vma),
    );
    rust_memory_bug_pfn_insert_valid(
        flags & RUST_MEMORY_VM_MIXEDMAP != 0 && rust_memory_pfn_valid(pfn),
    );
    if addr < rust_memory_vma_start(vma) || addr >= rust_memory_vma_end(vma) {
        return RUST_MEMORY_VM_FAULT_SIGBUS as _;
    }
    if !rust_memory_pfn_modify_allowed(pfn, pgprot) {
        return RUST_MEMORY_VM_FAULT_SIGBUS as _;
    }
    rust_memory_pfnmap_setup_cachemode_pfn(pfn, &mut pgprot);
    insert_pfn(vma, addr, pfn, pgprot, false)
}
#[no_mangle]
pub unsafe extern "C" fn vmf_insert_pfn(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pfn: c_ulong,
) -> vm_fault_t {
    vmf_insert_pfn_prot(vma, addr, pfn, (*vma).vm_page_prot)
}
unsafe fn vm_mixed_ok(vma: *mut vm_area_struct, pfn: c_ulong, mkwrite: bool) -> bool {
    if rust_memory_is_zero_pfn(pfn) && (mkwrite || !vm_mixed_zeropage_allowed(vma)) {
        return false;
    }
    if rust_memory_vma_vm_flags(vma) & RUST_MEMORY_VM_MIXEDMAP != 0 {
        return true;
    }
    rust_memory_is_zero_pfn(pfn)
}
unsafe fn __vm_insert_mixed(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pfn: c_ulong,
    mkwrite: bool,
) -> vm_fault_t {
    let mut pgprot = (*vma).vm_page_prot;
    if !vm_mixed_ok(vma, pfn, mkwrite) {
        return RUST_MEMORY_VM_FAULT_SIGBUS as _;
    }
    if addr < rust_memory_vma_start(vma) || addr >= rust_memory_vma_end(vma) {
        return RUST_MEMORY_VM_FAULT_SIGBUS as _;
    }
    rust_memory_pfnmap_setup_cachemode_pfn(pfn, &mut pgprot);
    if !rust_memory_pfn_modify_allowed(pfn, pgprot) {
        return RUST_MEMORY_VM_FAULT_SIGBUS as _;
    }
    if !cfg!(CONFIG_ARCH_HAS_PTE_SPECIAL) && rust_memory_pfn_valid(pfn) {
        let err = insert_page(vma, addr, rust_memory_pfn_to_page(pfn), pgprot, mkwrite);
        if err == -(ENOMEM as c_int) {
            return RUST_MEMORY_VM_FAULT_OOM as _;
        }
        if err < 0 && err != -(EBUSY as c_int) {
            return RUST_MEMORY_VM_FAULT_SIGBUS as _;
        }
        RUST_MEMORY_VM_FAULT_NOPAGE as _
    } else {
        insert_pfn(vma, addr, pfn, pgprot, mkwrite)
    }
}
#[no_mangle]
pub unsafe extern "C" fn vmf_insert_page_mkwrite(
    vmf: *mut vm_fault,
    page: *mut page,
    write: bool,
) -> vm_fault_t {
    let vma = rust_memory_vmf_vma(vmf);
    let addr = rust_memory_vmf_address(vmf);
    if addr < rust_memory_vma_start(vma) || addr >= rust_memory_vma_end(vma) {
        return RUST_MEMORY_VM_FAULT_SIGBUS as _;
    }
    let err = insert_page(vma, addr, page, (*vma).vm_page_prot, write);
    if err == -(ENOMEM as c_int) {
        return RUST_MEMORY_VM_FAULT_OOM as _;
    }
    if err < 0 && err != -(EBUSY as c_int) {
        return RUST_MEMORY_VM_FAULT_SIGBUS as _;
    }
    RUST_MEMORY_VM_FAULT_NOPAGE as _
}
#[no_mangle]
pub unsafe extern "C" fn vmf_insert_mixed(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pfn: c_ulong,
) -> vm_fault_t {
    __vm_insert_mixed(vma, addr, pfn, false)
}
#[no_mangle]
pub unsafe extern "C" fn vmf_insert_mixed_mkwrite(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pfn: c_ulong,
) -> vm_fault_t {
    __vm_insert_mixed(vma, addr, pfn, true)
}
unsafe fn remap_pte_range(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    mut addr: c_ulong,
    end: c_ulong,
    mut pfn: c_ulong,
    prot: pgprot_t,
) -> c_int {
    let mut ptl = null_mut();
    let mapped_pte = rust_memory_pte_alloc_map_lock(mm, pmd, addr, &mut ptl);
    if mapped_pte.is_null() {
        return -(ENOMEM as c_int);
    }
    let mut pte = mapped_pte;
    let mut err = 0;
    rust_memory_lazy_mmu_mode_enable();
    loop {
        rust_memory_bug_remap_not_none(!rust_memory_pte_none(rust_memory_ptep_get(pte)));
        if !rust_memory_pfn_modify_allowed(pfn, prot) {
            err = -(EACCES as c_int);
            break;
        }
        rust_memory_set_pte_at(
            mm,
            addr,
            pte,
            rust_memory_pte_mkspecial(rust_memory_pfn_pte(pfn, prot)),
        );
        pfn = pfn.wrapping_add(1);
        pte = pte.add(1);
        addr = addr.wrapping_add(PAGE_SIZE);
        if addr == end {
            break;
        }
    }
    rust_memory_lazy_mmu_mode_disable();
    rust_memory_pte_unmap_unlock(mapped_pte, ptl);
    err
}
macro_rules! remap_level {
    ($name:ident, $parent:ty, $alloc:ident, $end:ident, $child:ident, $check:expr) => {
        unsafe fn $name(
            mm: *mut mm_struct,
            parent: *mut $parent,
            mut addr: c_ulong,
            end: c_ulong,
            pfn: c_ulong,
            prot: pgprot_t,
        ) -> c_int {
            let pfn = pfn.wrapping_sub(addr >> PAGE_SHIFT);
            let mut table = $alloc(mm, parent, addr);
            if table.is_null() {
                return -(ENOMEM as c_int);
            }
            ($check)(table);
            loop {
                let next = $end(addr, end);
                let err = $child(
                    mm,
                    table,
                    addr,
                    next,
                    pfn.wrapping_add(addr >> PAGE_SHIFT),
                    prot,
                );
                if err != 0 {
                    return err;
                }
                table = table.add(1);
                addr = next;
                if addr == end {
                    break;
                }
            }
            0
        }
    };
}
remap_level!(
    remap_pmd_range,
    pud_t,
    rust_memory_pmd_alloc,
    rust_memory_pmd_addr_end,
    remap_pte_range,
    |p: *mut pmd_t| {
        #[cfg(CONFIG_DEBUG_VM)]
        rust_memory_bug_remap_huge(rust_memory_pmd_trans_huge(*p));
    }
);
remap_level!(
    remap_pud_range,
    p4d_t,
    rust_memory_pud_alloc,
    rust_memory_pud_addr_end,
    remap_pmd_range,
    |_p: *mut pud_t| {}
);
remap_level!(
    remap_p4d_range,
    pgd_t,
    rust_memory_p4d_alloc,
    rust_memory_p4d_addr_end,
    remap_pud_range,
    |_p: *mut p4d_t| {}
);
unsafe fn get_remap_pgoff(
    is_cow: bool,
    addr: c_ulong,
    end: c_ulong,
    vm_start: c_ulong,
    vm_end: c_ulong,
    pfn: c_ulong,
    vm_pgoff: *mut pgoff_t,
) -> c_int {
    if is_cow {
        if addr != vm_start || end != vm_end {
            return -(EINVAL as c_int);
        }
        *vm_pgoff = pfn;
    }
    0
}
fn memory_page_align(n: c_ulong) -> c_ulong {
    n.wrapping_add(PAGE_SIZE - 1) & PAGE_MASK
}
unsafe fn remap_pfn_range_internal(
    vma: *mut vm_area_struct,
    mut addr: c_ulong,
    pfn: c_ulong,
    size: c_ulong,
    prot: pgprot_t,
) -> c_int {
    let end = addr.wrapping_add(memory_page_align(size));
    let mm = (*vma).vm_mm;
    if rust_memory_warn_remap_unaligned(addr & !PAGE_MASK != 0) {
        return -(EINVAL as c_int);
    }
    #[cfg(CONFIG_DEBUG_VM)]
    rust_memory_warn_remap_flags(!rust_memory_vma_remap_flags(vma));
    rust_memory_bug_remap_empty(addr >= end);
    let pfn = pfn.wrapping_sub(addr >> PAGE_SHIFT);
    let mut pgd = rust_memory_pgd_offset(mm, addr);
    rust_memory_flush_cache_range(vma, addr, end);
    loop {
        let next = rust_memory_pgd_addr_end(addr, end);
        let err = remap_p4d_range(
            mm,
            pgd,
            addr,
            next,
            pfn.wrapping_add(addr >> PAGE_SHIFT),
            prot,
        );
        if err != 0 {
            return err;
        }
        pgd = pgd.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    0
}
unsafe fn remap_pfn_range_notrack(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pfn: c_ulong,
    size: c_ulong,
    prot: pgprot_t,
) -> c_int {
    let error = remap_pfn_range_internal(vma, addr, pfn, size, prot);
    if error == 0 {
        return 0;
    }
    zap_vma_range(vma, addr, size);
    error
}
#[cfg(RUST_MEMORY_HAVE_PFNMAP_TRACKING)]
unsafe fn pfnmap_track_ctx_alloc(
    pfn: c_ulong,
    size: c_ulong,
    prot: *mut pgprot_t,
) -> *mut pfnmap_track_ctx {
    if rust_memory_pfnmap_track(pfn, size, prot) != 0 {
        return (-(EINVAL as isize)) as *mut pfnmap_track_ctx;
    }
    let ctx = rust_memory_alloc_pfnmap_track_ctx();
    if ctx.is_null() {
        rust_memory_pfnmap_untrack(pfn, size);
        return (-(ENOMEM as isize)) as *mut pfnmap_track_ctx;
    }
    (*ctx).pfn = pfn;
    (*ctx).size = size;
    rust_memory_kref_init(addr_of_mut!((*ctx).kref));
    ctx
}
#[cfg(RUST_MEMORY_HAVE_PFNMAP_TRACKING)]
#[no_mangle]
pub unsafe extern "C" fn pfnmap_track_ctx_release(reference: *mut kref) {
    let ctx = reference
        .cast::<u8>()
        .sub(core::mem::offset_of!(pfnmap_track_ctx, kref))
        .cast::<pfnmap_track_ctx>();
    rust_memory_pfnmap_untrack((*ctx).pfn, (*ctx).size);
    kfree(ctx.cast());
}
#[cfg(RUST_MEMORY_HAVE_PFNMAP_TRACKING)]
unsafe fn remap_pfn_range_track(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pfn: c_ulong,
    size: c_ulong,
    mut prot: pgprot_t,
) -> c_int {
    let size = memory_page_align(size);
    let mut ctx = null_mut();
    if addr == rust_memory_vma_start(vma) && addr.wrapping_add(size) == rust_memory_vma_end(vma) {
        if !(*vma).pfnmap_track_ctx.is_null() {
            return -(EINVAL as c_int);
        }
        ctx = pfnmap_track_ctx_alloc(pfn, size, &mut prot);
        if rust_memory_is_err(ctx.cast()) {
            return ctx as c_long as c_int;
        }
    } else if rust_memory_pfnmap_setup_cachemode(pfn, size, &mut prot) != 0 {
        return -(EINVAL as c_int);
    }
    let err = remap_pfn_range_notrack(vma, addr, pfn, size, prot);
    if !ctx.is_null() {
        if err != 0 {
            rust_memory_kref_put(addr_of_mut!((*ctx).kref), Some(pfnmap_track_ctx_release));
        } else {
            (*vma).pfnmap_track_ctx = ctx;
        }
    }
    err
}
unsafe fn do_remap_pfn_range(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pfn: c_ulong,
    size: c_ulong,
    prot: pgprot_t,
) -> c_int {
    #[cfg(RUST_MEMORY_HAVE_PFNMAP_TRACKING)]
    {
        remap_pfn_range_track(vma, addr, pfn, size, prot)
    }
    #[cfg(not(RUST_MEMORY_HAVE_PFNMAP_TRACKING))]
    {
        remap_pfn_range_notrack(vma, addr, pfn, size, prot)
    }
}
#[no_mangle]
pub unsafe extern "C" fn remap_pfn_range_prepare(desc: *mut vm_area_desc) -> c_int {
    let action = addr_of_mut!((*desc).action);
    let start = rust_memory_action_remap_start(action);
    let end = start.wrapping_add(rust_memory_action_remap_size(action));
    let pfn = rust_memory_action_remap_pfn(action);
    let cow = rust_memory_vma_desc_is_cow_mapping(desc);
    if !rust_memory_range_in_vma_desc(desc, start, end) {
        return -(EFAULT as c_int);
    }
    let err = get_remap_pgoff(
        cow,
        start,
        end,
        (*desc).start,
        (*desc).end,
        pfn,
        addr_of_mut!((*desc).pgoff),
    );
    if err != 0 {
        return err;
    }
    rust_memory_vma_desc_set_remap_flags(desc);
    0
}
unsafe fn remap_pfn_range_prepare_vma(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pfn: c_ulong,
    size: c_ulong,
) -> c_int {
    let end = addr.wrapping_add(memory_page_align(size));
    let err = get_remap_pgoff(
        rust_memory_vma_is_cow_mapping(vma),
        addr,
        end,
        rust_memory_vma_start(vma),
        rust_memory_vma_end(vma),
        pfn,
        rust_memory_vma_vm_pgoff(vma),
    );
    if err != 0 {
        return err;
    }
    rust_memory_vma_set_remap_flags(vma);
    0
}
#[no_mangle]
pub unsafe extern "C" fn remap_pfn_range(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    pfn: c_ulong,
    size: c_ulong,
    prot: pgprot_t,
) -> c_int {
    let err = remap_pfn_range_prepare_vma(vma, addr, pfn, size);
    if err != 0 {
        return err;
    }
    do_remap_pfn_range(vma, addr, pfn, size, prot)
}
#[no_mangle]
pub unsafe extern "C" fn remap_pfn_range_complete(
    vma: *mut vm_area_struct,
    action: *mut mmap_action,
) -> c_int {
    do_remap_pfn_range(
        vma,
        rust_memory_action_remap_start(action),
        rust_memory_action_remap_pfn(action),
        rust_memory_action_remap_size(action),
        rust_memory_action_remap_pgprot(action),
    )
}
unsafe fn __simple_ioremap_prep(
    vm_len: c_ulong,
    vm_pgoff: pgoff_t,
    start_phys: phys_addr_t,
    mut size: c_ulong,
    pfnp: *mut c_ulong,
) -> c_int {
    if start_phys.wrapping_add(size as phys_addr_t) < start_phys {
        return -(EINVAL as c_int);
    }
    size = size.wrapping_add((start_phys & (!PAGE_MASK as phys_addr_t)) as c_ulong);
    let mut pfn = (start_phys >> PAGE_SHIFT) as c_ulong;
    let mut pages = size.wrapping_add(!PAGE_MASK) >> PAGE_SHIFT;
    if pfn.wrapping_add(pages) < pfn {
        return -(EINVAL as c_int);
    }
    if vm_pgoff > pages {
        return -(EINVAL as c_int);
    }
    pfn = pfn.wrapping_add(vm_pgoff);
    pages = pages.wrapping_sub(vm_pgoff);
    if (vm_len >> PAGE_SHIFT) > pages {
        return -(EINVAL as c_int);
    }
    *pfnp = pfn;
    0
}
#[no_mangle]
pub unsafe extern "C" fn simple_ioremap_prepare(desc: *mut vm_area_desc) -> c_int {
    let action = addr_of_mut!((*desc).action);
    let mut pfn = 0;
    let err = __simple_ioremap_prep(
        rust_memory_vma_desc_size(desc),
        (*desc).pgoff,
        rust_memory_action_ioremap_start_phys(action),
        rust_memory_action_ioremap_size(action),
        &mut pfn,
    );
    if err != 0 {
        return err;
    }
    rust_memory_mmap_action_ioremap_full(desc, pfn);
    rust_memory_io_remap_pfn_range_prepare(desc)
}
#[no_mangle]
pub unsafe extern "C" fn vm_iomap_memory(
    vma: *mut vm_area_struct,
    start: phys_addr_t,
    len: c_ulong,
) -> c_int {
    let vm_len = rust_memory_vma_end(vma).wrapping_sub(rust_memory_vma_start(vma));
    let mut pfn = 0;
    let err = __simple_ioremap_prep(
        vm_len,
        rust_memory_vma_start_pgoff(vma),
        start,
        len,
        &mut pfn,
    );
    if err != 0 {
        return err;
    }
    rust_memory_io_remap_pfn_range(
        vma,
        rust_memory_vma_start(vma),
        pfn,
        vm_len,
        (*vma).vm_page_prot,
    )
}
unsafe fn apply_to_pte_range(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    mut addr: c_ulong,
    end: c_ulong,
    callback: pte_fn_t,
    data: *mut c_void,
    create: bool,
    mask: *mut pgtbl_mod_mask,
) -> c_int {
    let mut ptl = null_mut();
    let kernel = mm == addr_of_mut!(init_mm);
    let mapped_pte = if create {
        if kernel {
            rust_memory_pte_alloc_kernel_track(pmd, addr, mask)
        } else {
            rust_memory_pte_alloc_map_lock(mm, pmd, addr, &mut ptl)
        }
    } else if kernel {
        rust_memory_pte_offset_kernel(pmd, addr)
    } else {
        rust_memory_pte_offset_map_lock(mm, pmd, addr, &mut ptl)
    };
    if mapped_pte.is_null() {
        return if create {
            -(ENOMEM as c_int)
        } else {
            -(EINVAL as c_int)
        };
    }
    let mut pte = mapped_pte;
    let mut err = 0;
    rust_memory_lazy_mmu_mode_enable();
    if let Some(f) = callback {
        loop {
            if create || !rust_memory_pte_none(rust_memory_ptep_get(pte)) {
                err = f(pte, addr, data);
                if err != 0 {
                    break;
                }
            }
            pte = pte.add(1);
            addr = addr.wrapping_add(PAGE_SIZE);
            if addr == end {
                break;
            }
        }
    }
    *mask |= RUST_MEMORY_PGTBL_PTE_MODIFIED as pgtbl_mod_mask;
    rust_memory_lazy_mmu_mode_disable();
    if !kernel {
        rust_memory_pte_unmap_unlock(mapped_pte, ptl);
    }
    err
}
macro_rules! apply_level {
    ($name:ident, $parent:ty, $alloc:ident, $offset:ident, $end:ident, $none:ident,
     $leaf:ident, $bad:ident, $clear:ident, $warn_leaf:ident, $warn_bad:ident, $child:ident, $check:expr) => {
        unsafe fn $name(
            mm: *mut mm_struct,
            parent: *mut $parent,
            mut addr: c_ulong,
            end: c_ulong,
            callback: pte_fn_t,
            data: *mut c_void,
            create: bool,
            mask: *mut pgtbl_mod_mask,
        ) -> c_int {
            ($check)(parent);
            let mut table = if create {
                $alloc(mm, parent, addr, mask)
            } else {
                $offset(parent, addr)
            };
            if create && table.is_null() {
                return -(ENOMEM as c_int);
            }
            let mut err = 0;
            loop {
                let next = $end(addr, end);
                if !($none(*table) && !create) {
                    if $warn_leaf($leaf(*table)) {
                        return -(EINVAL as c_int);
                    }
                    let bad = !$none(*table) && $warn_bad($bad(*table));
                    if !bad || create {
                        if bad {
                            $clear(table);
                        }
                        err = $child(mm, table, addr, next, callback, data, create, mask);
                        if err != 0 {
                            break;
                        }
                    }
                }
                table = table.add(1);
                addr = next;
                if addr == end {
                    break;
                }
            }
            err
        }
    };
}
apply_level!(
    apply_to_pmd_range,
    pud_t,
    rust_memory_pmd_alloc_track,
    rust_memory_pmd_offset,
    rust_memory_pmd_addr_end,
    rust_memory_pmd_none,
    rust_memory_pmd_leaf,
    rust_memory_pmd_bad,
    rust_memory_pmd_clear_bad,
    rust_memory_warn_apply_pmd_leaf,
    rust_memory_warn_apply_pmd_bad,
    apply_to_pte_range,
    |p: *mut pud_t| {
        rust_memory_bug_apply_pud_leaf(rust_memory_pud_leaf(*p));
    }
);
apply_level!(
    apply_to_pud_range,
    p4d_t,
    rust_memory_pud_alloc_track,
    rust_memory_pud_offset,
    rust_memory_pud_addr_end,
    rust_memory_pud_none,
    rust_memory_pud_leaf,
    rust_memory_pud_bad,
    rust_memory_pud_clear_bad,
    rust_memory_warn_apply_pud_leaf,
    rust_memory_warn_apply_pud_bad,
    apply_to_pmd_range,
    |_p: *mut p4d_t| {}
);
apply_level!(
    apply_to_p4d_range,
    pgd_t,
    rust_memory_p4d_alloc_track,
    rust_memory_p4d_offset,
    rust_memory_p4d_addr_end,
    rust_memory_p4d_none,
    rust_memory_p4d_leaf,
    rust_memory_p4d_bad,
    rust_memory_p4d_clear_bad,
    rust_memory_warn_apply_p4d_leaf,
    rust_memory_warn_apply_p4d_bad,
    apply_to_pud_range,
    |_p: *mut pgd_t| {}
);
unsafe fn __apply_to_page_range(
    mm: *mut mm_struct,
    mut addr: c_ulong,
    size: c_ulong,
    callback: pte_fn_t,
    data: *mut c_void,
    create: bool,
) -> c_int {
    let start = addr;
    let end = addr.wrapping_add(size);
    let mut mask: pgtbl_mod_mask = 0;
    let mut err = 0;
    if rust_memory_warn_apply_range(addr >= end) {
        return -(EINVAL as c_int);
    }
    let mut pgd = rust_memory_pgd_offset(mm, addr);
    loop {
        let next = rust_memory_pgd_addr_end(addr, end);
        if !(rust_memory_pgd_none(*pgd) && !create) {
            if rust_memory_warn_apply_pgd_leaf(rust_memory_pgd_leaf(*pgd)) {
                err = -(EINVAL as c_int);
                break;
            }
            let bad = !rust_memory_pgd_none(*pgd)
                && rust_memory_warn_apply_pgd_bad(rust_memory_pgd_bad(*pgd));
            if !bad || create {
                if bad {
                    rust_memory_pgd_clear_bad(pgd);
                }
                err = apply_to_p4d_range(mm, pgd, addr, next, callback, data, create, &mut mask);
                if err != 0 {
                    break;
                }
            }
        }
        pgd = pgd.add(1);
        addr = next;
        if addr == end {
            break;
        }
    }
    if mask & rust_memory_arch_page_table_sync_mask() != 0 {
        rust_memory_arch_sync_kernel_mappings(start, start.wrapping_add(size));
    }
    err
}
#[no_mangle]
pub unsafe extern "C" fn apply_to_page_range(
    mm: *mut mm_struct,
    addr: c_ulong,
    size: c_ulong,
    callback: pte_fn_t,
    data: *mut c_void,
) -> c_int {
    __apply_to_page_range(mm, addr, size, callback, data, true)
}
#[no_mangle]
pub unsafe extern "C" fn apply_to_existing_page_range(
    mm: *mut mm_struct,
    addr: c_ulong,
    size: c_ulong,
    callback: pte_fn_t,
    data: *mut c_void,
) -> c_int {
    __apply_to_page_range(mm, addr, size, callback, data, false)
}
