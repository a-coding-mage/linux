// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 * Copyright (c) 2025 Ventana Micro Systems Inc.
 */

// Linux and RISC-V dependencies are supplied by the surrounding kernel crate.

#[cfg(target_pointer_width = "64")]
pub static mut kvm_riscv_gstage_max_pgd_levels: ::core::ffi::c_ulong = 3;
#[cfg(target_pointer_width = "32")]
pub static mut kvm_riscv_gstage_max_pgd_levels: ::core::ffi::c_ulong = 2;

#[inline]
unsafe fn gstage_pte_leaf(__ptep: *const pte_t) -> bool {
    (pte_val(*__ptep) & (_PAGE_READ | _PAGE_WRITE | _PAGE_EXEC)) != 0
}

#[inline]
unsafe fn gstage_pte_index(gstage: *mut kvm_gstage, addr: gpa_t, level: u32) -> usize {
    let shift = HGATP_PAGE_SHIFT + (kvm_riscv_gstage_index_bits * level);
    let mask = if level == (*gstage).pgd_levels - 1 {
        (PTRS_PER_PTE * (1usize << kvm_riscv_gstage_pgd_xbits)) - 1
    } else { PTRS_PER_PTE - 1 };
    ((addr >> shift) as usize) & mask
}

#[inline]
unsafe fn gstage_pte_page_vaddr(pte: pte_t) -> usize {
    pfn_to_virt(__page_val_to_pfn(pte_val(pte))) as usize
}

unsafe fn gstage_page_size_to_level(gstage: *mut kvm_gstage, page_size: usize, out_level: *mut u32) -> i32 {
    let psz: usize = 1 << 12;
    for i in 0..(*gstage).pgd_levels {
        if page_size == (psz << (i * kvm_riscv_gstage_index_bits)) {
            *out_level = i;
            return 0;
        }
    }
    -EINVAL
}

unsafe fn gstage_level_to_page_order(gstage: *mut kvm_gstage, level: u32, out_pgorder: *mut usize) -> i32 {
    if (*gstage).pgd_levels < level { return -EINVAL; }
    *out_pgorder = 12 + (level as usize * kvm_riscv_gstage_index_bits as usize);
    0
}

unsafe fn gstage_level_to_page_size(gstage: *mut kvm_gstage, level: u32, out_pgsize: *mut usize) -> i32 {
    let mut page_order = PAGE_SHIFT as usize;
    let rc = gstage_level_to_page_order(gstage, level, &mut page_order);
    if rc != 0 { return rc; }
    *out_pgsize = BIT(page_order);
    0
}

pub unsafe fn kvm_riscv_gstage_get_leaf(gstage: *mut kvm_gstage, addr: gpa_t, ptepp: *mut *mut pte_t, ptep_level: *mut u32) -> bool {
    let mut current_level = (*gstage).pgd_levels - 1;
    *ptep_level = current_level;
    let mut ptep = (*gstage).pgd as *mut pte_t;
    ptep = ptep.add(gstage_pte_index(gstage, addr, current_level));
    while !ptep.is_null() && pte_val(ptep_get(ptep)) != 0 {
        if gstage_pte_leaf(ptep) {
            *ptep_level = current_level; *ptepp = ptep; return true;
        }
        if current_level != 0 {
            current_level -= 1; *ptep_level = current_level;
            ptep = gstage_pte_page_vaddr(ptep_get(ptep)) as *mut pte_t;
            ptep = ptep.add(gstage_pte_index(gstage, addr, current_level));
        } else { ptep = core::ptr::null_mut(); }
    }
    false
}

unsafe fn gstage_tlb_flush(gstage: *mut kvm_gstage, level: u32, mut addr: gpa_t) {
    let mut order = PAGE_SHIFT as usize;
    if gstage_level_to_page_order(gstage, level, &mut order) != 0 { return; }
    addr &= !(BIT(order) - 1);
    if (*gstage).flags & KVM_GSTAGE_FLAGS_LOCAL != 0 {
        kvm_riscv_local_hfence_gvma_vmid_gpa((*gstage).vmid, addr, BIT(order), order);
    } else {
        kvm_riscv_hfence_gvma_vmid_gpa((*gstage).kvm, -1isize as usize, 0, addr, BIT(order), order, (*gstage).vmid);
    }
}

pub unsafe fn kvm_riscv_gstage_try_update_pte(gstage: *mut kvm_gstage, level: u32, addr: gpa_t, ptep: *mut pte_t, old_pte: pte_t, new_pte: pte_t) -> bool {
    if cmpxchg(&mut (*ptep).pte, pte_val(old_pte), pte_val(new_pte)) != pte_val(old_pte) { return false; }
    if pte_val(old_pte) != pte_val(new_pte) { gstage_tlb_flush(gstage, level, addr); }
    true
}

pub unsafe fn kvm_riscv_gstage_set_pte(gstage: *mut kvm_gstage, pcache: *mut kvm_mmu_memory_cache, map: *const kvm_gstage_mapping) -> i32 {
    let mut current_level = (*gstage).pgd_levels - 1;
    let mut next_ptep = (*gstage).pgd as *mut pte_t;
    let mut ptep = next_ptep.add(gstage_pte_index(gstage, (*map).addr, current_level));
    if current_level < (*map).level { return -EINVAL; }
    while current_level != (*map).level {
        if gstage_pte_leaf(ptep) { return -EEXIST; }
        if pte_val(ptep_get(ptep)) == 0 {
            if pcache.is_null() { return -ENOMEM; }
            next_ptep = kvm_mmu_memory_cache_alloc(pcache);
            if next_ptep.is_null() { return -ENOMEM; }
            set_pte(ptep, pfn_pte(PFN_DOWN(__pa(next_ptep)), __pgprot(_PAGE_TABLE)));
        } else { next_ptep = gstage_pte_page_vaddr(ptep_get(ptep)) as *mut pte_t; }
        current_level -= 1;
        ptep = next_ptep.add(gstage_pte_index(gstage, (*map).addr, current_level));
    }
    if pte_val(*ptep) != pte_val((*map).pte) {
        let was_invalid = pte_val(*ptep) == 0;
        set_pte(ptep, (*map).pte);
        if gstage_pte_leaf(ptep) && !(was_invalid && riscv_has_extension_unlikely(RISCV_ISA_EXT_SVVPTC)) { gstage_tlb_flush(gstage, current_level, (*map).addr); }
    }
    0
}

unsafe fn kvm_riscv_gstage_update_pte_prot(gstage: *mut kvm_gstage, level: u32, addr: gpa_t, ptep: *mut pte_t, prot: pgprot_t) {
    loop {
        let old_pte = ptep_get(ptep);
        if pgprot_val(pte_pgprot(old_pte)) == pgprot_val(prot) { return; }
        let new_pte = pte_mkdirty(pfn_pte(pte_pfn(old_pte), prot));
        if kvm_riscv_gstage_try_update_pte(gstage, level, addr, ptep, old_pte, new_pte) { return; }
        cpu_relax();
    }
}

pub unsafe fn kvm_riscv_gstage_map_page(gstage: *mut kvm_gstage, pcache: *mut kvm_mmu_memory_cache, gpa: gpa_t, hpa: phys_addr_t, page_size: usize, page_rdonly: bool, page_exec: bool, out_map: *mut kvm_gstage_mapping) -> i32 {
    let prot = if page_exec { if page_rdonly { PAGE_READ_EXEC } else { PAGE_WRITE_EXEC } } else if page_rdonly { PAGE_READ } else { PAGE_WRITE };
    (*out_map).addr = gpa; (*out_map).level = 0;
    let ret = gstage_page_size_to_level(gstage, page_size, &mut (*out_map).level); if ret != 0 { return ret; }
    let mut ptep = core::ptr::null_mut(); let mut ptep_level = 0;
    if kvm_riscv_gstage_get_leaf(gstage, gpa, &mut ptep, &mut ptep_level) {
        if ptep_level > (*out_map).level { kvm_riscv_gstage_split_huge(gstage, pcache, gpa, (*out_map).level, true); }
        else if ALIGN_DOWN(PFN_PHYS(pte_pfn(ptep_get(ptep))), page_size) == hpa { kvm_riscv_gstage_update_pte_prot(gstage, ptep_level, gpa, ptep, prot); return 0; }
    }
    (*out_map).pte = pte_mkdirty(pfn_pte(PFN_DOWN(hpa), prot));
    kvm_riscv_gstage_set_pte(gstage, pcache, out_map)
}

#[inline]
unsafe fn make_child_pte(huge_pte: usize, index: i32, child_page_size: usize) -> usize {
    huge_pte | pte_val(pfn_pte(index as usize * (child_page_size / PAGE_SIZE), __pgprot(0)))
}

pub unsafe fn kvm_riscv_gstage_split_huge(gstage: *mut kvm_gstage, pcache: *mut kvm_mmu_memory_cache, addr: gpa_t, target_level: u32, flush: bool) -> bool {
    let mut current_level = (*gstage).pgd_levels - 1; let mut next_ptep = (*gstage).pgd as *mut pte_t; let mut need_flush = false;
    if pcache.is_null() { return false; }
    while current_level > target_level {
        let ptep = next_ptep.add(gstage_pte_index(gstage, addr, current_level));
        if pte_val(ptep_get(ptep)) == 0 { break; }
        if !gstage_pte_leaf(ptep) { next_ptep = gstage_pte_page_vaddr(ptep_get(ptep)) as *mut pte_t; current_level -= 1; continue; }
        let huge_pte = pte_val(ptep_get(ptep)); let mut child_page_size = 0;
        if gstage_level_to_page_size(gstage, current_level - 1, &mut child_page_size) != 0 { return need_flush; }
        next_ptep = kvm_mmu_memory_cache_alloc(pcache); if next_ptep.is_null() { return need_flush; }
        for i in 0..PTRS_PER_PTE { set_pte(next_ptep.add(i), __pte(make_child_pte(huge_pte, i as i32, child_page_size))); }
        smp_wmb(); set_pte(ptep, pfn_pte(PFN_DOWN(__pa(next_ptep)), __pgprot(_PAGE_TABLE)));
        if flush { gstage_tlb_flush(gstage, current_level, addr); } else { need_flush = true; }
        current_level -= 1;
    }
    need_flush
}

pub unsafe fn kvm_riscv_gstage_op_pte(gstage: *mut kvm_gstage, addr: gpa_t, ptep: *mut pte_t, ptep_level: u32, op: kvm_riscv_gstage_op) -> bool {
    let mut page_size = 0; if gstage_level_to_page_size(gstage, ptep_level, &mut page_size) != 0 { return false; }
    WARN_ON(addr & (page_size - 1)); if pte_val(ptep_get(ptep)) == 0 { return false; }
    let mut flush = false;
    if ptep_level != 0 && !gstage_pte_leaf(ptep) {
        let next = gstage_pte_page_vaddr(ptep_get(ptep)) as *mut pte_t; let mut next_size = 0;
        if gstage_level_to_page_size(gstage, ptep_level - 1, &mut next_size) != 0 { return false; }
        if op == GSTAGE_OP_CLEAR { set_pte(ptep, __pte(0)); }
        for i in 0..PTRS_PER_PTE { flush |= kvm_riscv_gstage_op_pte(gstage, addr + i * next_size, next.add(i), ptep_level - 1, op); }
        if op == GSTAGE_OP_CLEAR { put_page(virt_to_page(next)); }
    } else {
        let old = *ptep;
        if op == GSTAGE_OP_CLEAR { set_pte(ptep, __pte(0)); } else if op == GSTAGE_OP_WP { set_pte(ptep, __pte(pte_val(ptep_get(ptep)) & !_PAGE_WRITE)); }
        if pte_val(*ptep) != pte_val(old) { flush = true; }
    }
    flush
}

pub unsafe fn kvm_riscv_gstage_unmap_range(gstage: *mut kvm_gstage, start: gpa_t, size: gpa_t, may_block: bool) -> bool {
    let mut addr = start; let end = start + size; let mut flush = false;
    while addr < end { let mut ptep = core::ptr::null_mut(); let mut level = 0; let found = kvm_riscv_gstage_get_leaf(gstage, addr, &mut ptep, &mut level); let mut ps = 0; if gstage_level_to_page_size(gstage, level, &mut ps) != 0 { break; }
        if !found { addr = ALIGN(addr + 1, ps); } else { if (addr & (ps - 1)) == 0 && end - addr >= ps { flush |= kvm_riscv_gstage_op_pte(gstage, addr, ptep, level, GSTAGE_OP_CLEAR); } else { WARN_ONCE(true, "Skip unmap range addr: %#llx, end: %#llx, page_size: %#lx\n", addr, end, ps); } addr += ps; }
        if (*gstage).flags & KVM_GSTAGE_FLAGS_LOCAL == 0 && may_block && addr < end { cond_resched_rwlock_write(&(*gstage).kvm.mmu_lock); }
    } flush
}

pub unsafe fn kvm_riscv_gstage_wp_range(gstage: *mut kvm_gstage, start: gpa_t, end: gpa_t) -> bool {
    let mut addr = start; let mut flush = false;
    while addr < end { let mut ptep = core::ptr::null_mut(); let mut level = 0; let found = kvm_riscv_gstage_get_leaf(gstage, addr, &mut ptep, &mut level); let mut ps = 0; if gstage_level_to_page_size(gstage, level, &mut ps) != 0 { break; }
        if !found { addr = ALIGN(addr + 1, ps); } else { addr = ALIGN_DOWN(addr, ps); flush |= kvm_riscv_gstage_op_pte(gstage, addr, ptep, level, GSTAGE_OP_WP); addr += ps; }
    } flush
}

#[inline] unsafe fn clear_huge_mask(mask: *mut usize, page_size: usize, base_gfn: gfn_t, addr: gpa_t) {
    let mut start = 0usize; let mut end = BITS_PER_LONG - 1; let end_gfn = base_gfn + end; let start_gfn = addr >> PAGE_SHIFT; let end_addr = start_gfn + (page_size >> PAGE_SHIFT) - 1;
    if start_gfn > base_gfn { start = start_gfn - base_gfn; } if end_addr < end_gfn { end = end_addr - base_gfn; } bitmap_clear(mask, start, end - start + 1);
}

pub unsafe fn kvm_riscv_gstage_wp_pt_masked(gstage: *mut kvm_gstage, base_gfn: gfn_t, mut mask: usize) -> bool {
    let mut flush = false;
    while mask != 0 { let addr = (base_gfn + __ffs(mask)) << PAGE_SHIFT; let mut ptep = core::ptr::null_mut(); let mut level = 0; let found = kvm_riscv_gstage_get_leaf(gstage, addr, &mut ptep, &mut level); let mut ps = 0; if gstage_level_to_page_size(gstage, level, &mut ps) != 0 { break; }
        if found { if level != 0 { let a = ALIGN_DOWN(addr, ps); clear_huge_mask(&mut mask, ps, base_gfn, a); flush |= kvm_riscv_gstage_op_pte(gstage, a, ptep, level, GSTAGE_OP_WP); if level != 0 { continue; } } }
        mask &= mask - 1;
    } flush
}

pub unsafe fn kvm_riscv_gstage_mode_detect() {
    // CONFIG_64BIT / CONFIG_32BIT selects the supported HGATP mode probes.
    #[cfg(target_pointer_width = "64")]
    { csr_write(CSR_HGATP, HGATP_MODE_SV57X4 << HGATP_MODE_SHIFT); if csr_read(CSR_HGATP) >> HGATP_MODE_SHIFT == HGATP_MODE_SV57X4 { kvm_riscv_gstage_max_pgd_levels = 5; } else { csr_write(CSR_HGATP, HGATP_MODE_SV48X4 << HGATP_MODE_SHIFT); if csr_read(CSR_HGATP) >> HGATP_MODE_SHIFT == HGATP_MODE_SV48X4 { kvm_riscv_gstage_max_pgd_levels = 4; } else { csr_write(CSR_HGATP, HGATP_MODE_SV39X4 << HGATP_MODE_SHIFT); if csr_read(CSR_HGATP) >> HGATP_MODE_SHIFT == HGATP_MODE_SV39X4 { kvm_riscv_gstage_max_pgd_levels = 3; } else { kvm_riscv_gstage_max_pgd_levels = 0; } } } }
    #[cfg(target_pointer_width = "32")]
    { csr_write(CSR_HGATP, HGATP_MODE_SV32X4 << HGATP_MODE_SHIFT); if csr_read(CSR_HGATP) >> HGATP_MODE_SHIFT == HGATP_MODE_SV32X4 { kvm_riscv_gstage_max_pgd_levels = 2; } else { kvm_riscv_gstage_max_pgd_levels = 0; } }
    csr_write(CSR_HGATP, 0); kvm_riscv_local_hfence_gvma_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
