// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2007-2008 Paul Mackerras, IBM Corp.
 */

/* Kernel dependencies are supplied by the surrounding translation unit. */

pub unsafe fn subpage_prot_free(mm: *mut mm_struct) {
    let spt = mm_ctx_subpage_prot(&mut (*mm).context);
    let mut addr: c_ulong;
    let mut p: *mut *mut u32;

    if spt.is_null() {
        return;
    }

    for i in 0..4 {
        if !(*spt).low_prot[i].is_null() {
            free_page((*spt).low_prot[i] as c_ulong);
            (*spt).low_prot[i] = core::ptr::null_mut();
        }
    }
    addr = 0;
    for i in 0..(TASK_SIZE_USER64 >> 43) {
        p = (*spt).protptrs[i as usize];
        if p.is_null() {
            continue;
        }
        (*spt).protptrs[i as usize] = core::ptr::null_mut();
        let mut j = 0;
        while j < SBP_L2_COUNT && addr < (*spt).maxaddr {
            if !(*p.add(j as usize)).is_null() {
                free_page(*p.add(j as usize) as c_ulong);
            }
            j += 1;
            addr += PAGE_SIZE;
        }
        free_page(p as c_ulong);
    }
    (*spt).maxaddr = 0;
    kfree(spt as *mut core::ffi::c_void);
}

unsafe fn hpte_flush_range(mm: *mut mm_struct, mut addr: c_ulong, mut npages: c_int) {
    let pgd = pgd_offset(mm, addr);
    let p4d = p4d_offset(pgd, addr);
    if p4d_none(*p4d) { return; }
    let pud = pud_offset(p4d, addr);
    if pud_none(*pud) { return; }
    let pmd = pmd_offset(pud, addr);
    if pmd_none(*pmd) { return; }
    let mut ptl: *mut spinlock_t = core::ptr::null_mut();
    let mut pte = pte_offset_map_lock(mm, pmd, addr, &mut ptl);
    if pte.is_null() { return; }
    lazy_mmu_mode_enable();
    while npages > 0 {
        pte_update(mm, addr, pte, 0, 0, 0);
        addr += PAGE_SIZE;
        pte = pte.add(1);
        npages -= 1;
    }
    lazy_mmu_mode_disable();
    pte_unmap_unlock(pte.sub(1), ptl);
}

/*
 * Clear the subpage protection map for an address range, allowing
 * all accesses that are allowed by the pte permissions.
 */
unsafe fn subpage_prot_clear(mut addr: c_ulong, len: c_ulong) {
    let mm = (*current).mm;
    mmap_write_lock(mm);
    let spt = mm_ctx_subpage_prot(&mut (*mm).context);
    if spt.is_null() { mmap_write_unlock(mm); return; }
    let limit = core::cmp::min(addr + len, (*spt).maxaddr);
    let mut next: c_ulong;
    while addr < limit {
        next = pmd_addr_end(addr, limit);
        let spm = if addr < 0x100000000 { (*spt).low_prot.as_mut_ptr() } else {
            let x = (*spt).protptrs[(addr >> SBP_L3_SHIFT) as usize];
            if x.is_null() { addr = next; continue; }
            x
        };
        let spp = *spm.add(((addr >> SBP_L2_SHIFT) & (SBP_L2_COUNT - 1)) as usize);
        if spp.is_null() { addr = next; continue; }
        let spp = spp.add(((addr >> PAGE_SHIFT) & (SBP_L1_COUNT - 1)) as usize);
        let i = (addr >> PAGE_SHIFT) & (PTRS_PER_PTE - 1);
        let mut nw = PTRS_PER_PTE - i;
        if addr + (nw << PAGE_SHIFT) > next { nw = (next - addr) >> PAGE_SHIFT; }
        core::ptr::write_bytes(spp, 0, nw as usize);
        hpte_flush_range(mm, addr, nw as c_int);
        addr = next;
    }
    mmap_write_unlock(mm);
}

/* CONFIG_TRANSPARENT_HUGEPAGE supplies the same helper behavior externally. */
unsafe fn subpage_mark_vma_nohuge(_mm: *mut mm_struct, _addr: c_ulong, _len: c_ulong) {}

/*
 * Copy in a subpage protection map for an address range.
 * The map has 2 bits per 4k subpage, so 32 bits per 64k page.
 * Each 2-bit field is 0 to allow any access, 1 to prevent writes,
 * 2 or 3 to prevent all accesses.
 * Note that the normal page protections also apply; the subpage
 * protection mechanism is an additional constraint, so putting 0
 * in a 2-bit field won't allow writes to a page that is otherwise
 * write-protected.
 */
pub unsafe fn subpage_prot(mut addr: c_ulong, len: c_ulong, mut map: *const u32) -> c_long {
    let mm = (*current).mm;
    if radix_enabled() { return -ENOENT; }
    if (addr & !PAGE_MASK) != 0 || (len & !PAGE_MASK) != 0 || addr >= (*mm).task_size || len >= (*mm).task_size || addr + len > (*mm).task_size { return -EINVAL; }
    if is_hugepage_only_range(mm, addr, len) { return -EINVAL; }
    if map.is_null() { subpage_prot_clear(addr, len); return 0; }
    if !access_ok(map, (len >> PAGE_SHIFT) * core::mem::size_of::<u32>()) { return -EFAULT; }
    mmap_write_lock(mm);
    let spt = {
        let mut x = mm_ctx_subpage_prot(&mut (*mm).context);
        if x.is_null() {
            x = kzalloc_obj::<subpage_prot_table>();
            if x.is_null() { mmap_write_unlock(mm); return -ENOMEM; }
            (*mm).context.hash_context.as_mut().unwrap().spt = x;
        }
        x
    };
    subpage_mark_vma_nohuge(mm, addr, len);
    let limit = addr + len;
    while addr < limit {
        let next = pmd_addr_end(addr, limit);
        let spm = if addr < 0x100000000 { (*spt).low_prot.as_mut_ptr() } else {
            let slot = &mut (*spt).protptrs[(addr >> SBP_L3_SHIFT) as usize];
            if (*slot).is_null() { *slot = get_zeroed_page(GFP_KERNEL) as *mut *mut u32; if (*slot).is_null() { mmap_write_unlock(mm); return -ENOMEM; } }
            *slot
        }.add(((addr >> SBP_L2_SHIFT) & (SBP_L2_COUNT - 1)) as usize);
        if (*spm).is_null() { *spm = get_zeroed_page(GFP_KERNEL) as *mut u32; if (*spm).is_null() { mmap_write_unlock(mm); return -ENOMEM; } }
        let spp = (*spm).add(((addr >> PAGE_SHIFT) & (SBP_L1_COUNT - 1)) as usize);
        local_irq_disable(); demote_segment_4k(mm, addr); local_irq_enable();
        let i = (addr >> PAGE_SHIFT) & (PTRS_PER_PTE - 1);
        let mut nw = PTRS_PER_PTE - i;
        if addr + (nw << PAGE_SHIFT) > next { nw = (next - addr) >> PAGE_SHIFT; }
        mmap_write_unlock(mm);
        if __copy_from_user(spp, map, nw * core::mem::size_of::<u32>()) != 0 { return -EFAULT; }
        map = map.add(nw as usize);
        mmap_write_lock(mm);
        hpte_flush_range(mm, addr, nw as c_int);
        addr = next;
    }
    if limit > (*spt).maxaddr { (*spt).maxaddr = limit; }
    mmap_write_unlock(mm);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
