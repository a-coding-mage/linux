// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2020-2022 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn hl_mmu_v2_hr_get_pgt_info(ctx: *mut hl_ctx, phys_hop_addr: u64) -> *mut pgt_info {
    let mut pgt_info: *mut pgt_info = core::ptr::null_mut();
    // C hash_for_each_possible: iterate the context's physical-page-table hash.
    hash_for_each_possible((*ctx).hr_mmu_phys_hash, pgt_info, node, phys_hop_addr as usize) {
        if phys_hop_addr == (*pgt_info).phys_addr { break; }
    }
    pgt_info
}

unsafe fn hl_mmu_v2_hr_add_pgt_info(ctx: *mut hl_ctx, pgt_info: *mut pgt_info, phys_addr: dma_addr_t) {
    hash_add((*ctx).hr_mmu_phys_hash, &mut (*pgt_info).node, phys_addr);
}

unsafe fn hl_mmu_v2_hr_get_hop0_pgt_info(ctx: *mut hl_ctx) -> *mut pgt_info {
    &mut (*(*ctx).hdev).mmu_priv.hr.mmu_asid_hop0[(*ctx).asid as usize]
}

unsafe fn hl_mmu_v2_hr_init(hdev: *mut hl_device) -> i32 {
    let prop = &(*hdev).asic_prop;
    hl_mmu_hr_init(hdev, &mut (*hdev).mmu_priv.hr, prop.pmmu.hop_table_size, prop.mmu_pgt_size)
}

unsafe fn hl_mmu_v2_hr_fini(hdev: *mut hl_device) {
    let prop = &(*hdev).asic_prop;
    hl_mmu_hr_fini(hdev, &mut (*hdev).mmu_priv.hr, prop.pmmu.hop_table_size);
}

unsafe fn hl_mmu_v2_hr_ctx_init(ctx: *mut hl_ctx) -> i32 {
    hash_init((*ctx).hr_mmu_phys_hash);
    0
}

unsafe fn hl_mmu_v2_hr_ctx_fini(ctx: *mut hl_ctx) {
    let hdev = (*ctx).hdev;
    let mut pgt_info: *mut pgt_info = core::ptr::null_mut();
    let mut tmp: *mut hlist_node = core::ptr::null_mut();
    let mut i: i32 = 0;
    if !hash_empty((*ctx).hr_mmu_phys_hash) {
        dev_err((*hdev).dev, "ctx %d is freed while it has pgts in use\n", (*ctx).asid);
    }
    hash_for_each_safe((*ctx).hr_mmu_phys_hash, i, tmp, pgt_info, node) {
        dev_err_ratelimited((*hdev).dev, "pgt_info of addr 0x%llx of asid %d was not destroyed, num_ptes: %d\n", (*pgt_info).phys_addr, (*ctx).asid, (*pgt_info).num_of_ptes);
        hl_mmu_hr_free_hop_remove_pgt(pgt_info, &mut (*(*ctx).hdev).mmu_priv.hr, (*(*ctx).hdev).asic_prop.pmmu.hop_table_size);
    }
}

unsafe fn _hl_mmu_v2_hr_unmap(ctx: *mut hl_ctx, virt_addr: u64, is_dram_addr: bool) -> i32 {
    let mut curr_pte: u64;
    let mut scrambled_virt_addr: u64;
    let mut hop_pte_phys_addr = [0u64; MMU_ARCH_6_HOPS];
    let mut hops_pgt_info = [core::ptr::null_mut(); MMU_ARCH_6_HOPS];
    let hdev = (*ctx).hdev;
    let prop = &(*hdev).asic_prop;
    let mmu_prop = if is_dram_addr { &prop.dmmu } else { &prop.pmmu };
    let mut hop_last = mmu_prop.num_hops - 1;
    scrambled_virt_addr = (*(*hdev).asic_funcs).scramble_addr(hdev, virt_addr);
    curr_pte = 0;
    let mut i = 0;
    while i < mmu_prop.num_hops {
        hops_pgt_info[i as usize] = if i == 0 { hl_mmu_v2_hr_get_hop0_pgt_info(ctx) } else { hl_mmu_hr_get_next_hop_pgt_info(ctx, &(*hdev).mmu_func[MMU_HR_PGT].hr_funcs, curr_pte) };
        if hops_pgt_info[i as usize].is_null() { dev_err((*hdev).dev, "virt addr 0x%llx is not mapped to phys addr\n", virt_addr); return -EINVAL; }
        hop_pte_phys_addr[i as usize] = hl_mmu_get_hop_pte_phys_addr(ctx, mmu_prop, i, (*hops_pgt_info[i as usize]).phys_addr, scrambled_virt_addr);
        if hop_pte_phys_addr[i as usize] == u64::MAX { return -EFAULT; }
        curr_pte = *(hl_mmu_hr_pte_phys_to_virt(ctx, hops_pgt_info[i as usize], hop_pte_phys_addr[i as usize], (*hdev).asic_prop.pmmu.hop_table_size) as *const u64);
        if i < hop_last && (curr_pte & mmu_prop.last_mask) != 0 { hop_last = i; break; }
        i += 1;
    }
    if is_dram_addr && hop_last == mmu_prop.num_hops - 1 { dev_err((*hdev).dev, "DRAM unmapping should use huge pages only\n"); return -EFAULT; }
    if curr_pte & PAGE_PRESENT_MASK == 0 { dev_err((*hdev).dev, "virt addr 0x%llx is not mapped to phys addr\n", virt_addr); return -EINVAL; }
    i = hop_last;
    while i > 0 {
        hl_mmu_hr_clear_pte(ctx, hops_pgt_info[i as usize], hop_pte_phys_addr[i as usize], (*hdev).asic_prop.pmmu.hop_table_size);
        if hl_mmu_hr_put_pte(ctx, hops_pgt_info[i as usize], &mut (*ctx).hdev.mmu_priv.hr, (*hdev).asic_prop.pmmu.hop_table_size) != 0 { break; }
        i -= 1;
    }
    hl_mmu_hr_clear_pte(ctx, hops_pgt_info[0], hop_pte_phys_addr[0], (*hdev).asic_prop.pmmu.hop_table_size);
    return 0;
}

unsafe fn hl_mmu_v2_get_last_hop(mmu_prop: *mut hl_mmu_properties, page_size: u32) -> i32 {
    let mut hop = (*mmu_prop).num_hops - 1;
    while hop != 0 { if (*mmu_prop).hop_shifts[hop as usize] != 0 && page_size <= (1u32 << (*mmu_prop).hop_shifts[hop as usize]) { break; } hop -= 1; }
    hop
}

unsafe fn _hl_mmu_v2_hr_map(ctx: *mut hl_ctx, virt_addr: u64, phys_addr: u64, page_size: u32, is_dram_addr: bool) -> i32 {
    let hdev = (*ctx).hdev; let prop = &(*hdev).asic_prop;
    let mmu_prop = if is_dram_addr { &prop.dmmu } else if page_size == prop.pmmu_huge.page_size { &prop.pmmu_huge } else { &prop.pmmu };
    let hop_last = hl_mmu_v2_get_last_hop(mmu_prop, page_size); if hop_last <= 0 { dev_err((*hdev).dev, "Invalid last HOP %d\n", hop_last); return -EFAULT; }
    let va = (*(*hdev).asic_funcs).scramble_addr(hdev, virt_addr); let pa = (*(*hdev).asic_funcs).scramble_addr(hdev, phys_addr);
    let mut curr_pte = 0u64; let mut hop_pte_phys_addr = [0u64; MMU_ARCH_6_HOPS]; let mut hops_pgt_info = [core::ptr::null_mut(); MMU_ARCH_6_HOPS]; let mut hop_new = [false; MMU_ARCH_6_HOPS];
    let mut i = 0; while i <= hop_last { hops_pgt_info[i as usize] = if i == 0 { hl_mmu_v2_hr_get_hop0_pgt_info(ctx) } else { hl_mmu_hr_get_alloc_next_hop(ctx, &mut (*hdev).mmu_priv.hr, &(*hdev).mmu_func[MMU_HR_PGT].hr_funcs, mmu_prop, curr_pte, &mut hop_new[i as usize]) }; if hops_pgt_info[i as usize].is_null() { break; } hop_pte_phys_addr[i as usize] = hl_mmu_get_hop_pte_phys_addr(ctx, mmu_prop, i, (*hops_pgt_info[i as usize]).phys_addr, va); curr_pte = *(hl_mmu_hr_pte_phys_to_virt(ctx, hops_pgt_info[i as usize], hop_pte_phys_addr[i as usize], (*hdev).asic_prop.pmmu.hop_table_size) as *const u64); i += 1; }
    if i <= hop_last { for j in 1..=hop_last { if hop_new[j as usize] && !hops_pgt_info[j as usize].is_null() { hl_mmu_hr_free_hop_remove_pgt(hops_pgt_info[j as usize], &mut (*ctx).hdev.mmu_priv.hr, (*hdev).asic_prop.pmmu.hop_table_size); } } return -ENOMEM; }
    if curr_pte & PAGE_PRESENT_MASK != 0 { dev_err((*hdev).dev, "mapping already exists for virt_addr 0x%llx\n", va); return -EINVAL; }
    curr_pte = (pa & HOP_PHYS_ADDR_MASK) | mmu_prop.last_mask | PAGE_PRESENT_MASK; hl_mmu_hr_write_pte(ctx, hops_pgt_info[hop_last as usize], hop_pte_phys_addr[hop_last as usize], curr_pte, (*hdev).asic_prop.pmmu.hop_table_size);
    i = 1; while i <= hop_last { if hop_new[i as usize] { curr_pte = ((*hops_pgt_info[i as usize]).phys_addr & HOP_PHYS_ADDR_MASK) | PAGE_PRESENT_MASK; hl_mmu_hr_write_pte(ctx, hops_pgt_info[(i-1) as usize], hop_pte_phys_addr[(i-1) as usize], curr_pte, (*hdev).asic_prop.pmmu.hop_table_size); if i-1 != 0 { hl_mmu_hr_get_pte(ctx, &(*hdev).mmu_func[MMU_HR_PGT].hr_funcs, (*hops_pgt_info[(i-1) as usize]).phys_addr); } } i += 1; }
    hl_mmu_hr_get_pte(ctx, &(*hdev).mmu_func[MMU_HR_PGT].hr_funcs, (*hops_pgt_info[hop_last as usize]).phys_addr); 0
}

unsafe fn hl_mmu_v2_hr_swap_out(_ctx: *mut hl_ctx) {}
unsafe fn hl_mmu_v2_hr_swap_in(_ctx: *mut hl_ctx) {}

unsafe fn hl_mmu_v2_hr_get_tlb_mapping_params(hdev: *mut hl_device, mmu_prop: *mut *mut hl_mmu_properties, hops: *mut hl_mmu_hop_info, virt_addr: u64, is_huge: *mut bool) -> i32 {
    let prop = &(*hdev).asic_prop;
    if hl_mem_area_inside_range(virt_addr, prop.dmmu.page_size, prop.dmmu.start_addr, prop.dmmu.end_addr) { *mmu_prop = &mut prop.dmmu; *is_huge = true; (*hops).range_type = HL_VA_RANGE_TYPE_DRAM; }
    else if hl_mem_area_inside_range(virt_addr, prop.pmmu.page_size, prop.pmmu.start_addr, prop.pmmu.end_addr) { *mmu_prop = &mut prop.pmmu; *is_huge = false; (*hops).range_type = HL_VA_RANGE_TYPE_HOST; }
    else if hl_mem_area_inside_range(virt_addr, prop.pmmu_huge.page_size, prop.pmmu_huge.start_addr, prop.pmmu_huge.end_addr) { *mmu_prop = &mut prop.pmmu_huge; *is_huge = true; (*hops).range_type = HL_VA_RANGE_TYPE_HOST_HUGE; } else { return -EINVAL; } 0
}

unsafe fn hl_mmu_v2_hr_get_tlb_info(ctx: *mut hl_ctx, virt_addr: u64, hops: *mut hl_mmu_hop_info) -> i32 { hl_mmu_hr_get_tlb_info(ctx, virt_addr, hops, &(*(*ctx).hdev).mmu_func[MMU_HR_PGT].hr_funcs) }

pub unsafe fn hl_mmu_v2_hr_set_funcs(hdev: *mut hl_device, mmu: *mut hl_mmu_funcs) {
    (*mmu).init = Some(hl_mmu_v2_hr_init); (*mmu).fini = Some(hl_mmu_v2_hr_fini); (*mmu).ctx_init = Some(hl_mmu_v2_hr_ctx_init); (*mmu).ctx_fini = Some(hl_mmu_v2_hr_ctx_fini); (*mmu).map = Some(_hl_mmu_v2_hr_map); (*mmu).unmap = Some(_hl_mmu_v2_hr_unmap); (*mmu).flush = Some(hl_mmu_hr_flush); (*mmu).swap_out = Some(hl_mmu_v2_hr_swap_out); (*mmu).swap_in = Some(hl_mmu_v2_hr_swap_in); (*mmu).get_tlb_info = Some(hl_mmu_v2_hr_get_tlb_info); (*mmu).hr_funcs.get_hop0_pgt_info = Some(hl_mmu_v2_hr_get_hop0_pgt_info); (*mmu).hr_funcs.get_pgt_info = Some(hl_mmu_v2_hr_get_pgt_info); (*mmu).hr_funcs.add_pgt_info = Some(hl_mmu_v2_hr_add_pgt_info); (*mmu).hr_funcs.get_tlb_mapping_params = Some(hl_mmu_v2_hr_get_tlb_mapping_params);
    let _ = hdev;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
