// SPDX-License-Identifier: GPL-2.0
/* Copyright 2016-2019 HabanaLabs, Ltd. All Rights Reserved. */

// Dependencies supplied by the surrounding kernel translation.

const MMU_V1_MAX_HOPS: usize = MMU_HOP4 as usize + 1;

#[inline]
unsafe fn get_hop_pte_addr(ctx: *mut hl_ctx, mmu_prop: *mut hl_mmu_properties,
    hop_addr_arr: *mut u64, virt_addr: u64, hop_idx: usize) -> u64 {
    let mask = (*mmu_prop).hop_masks[hop_idx];
    let shift = (*mmu_prop).hop_shifts[hop_idx];
    *hop_addr_arr.add(hop_idx) + (*(*ctx).hdev).asic_prop.mmu_pte_size * ((virt_addr & mask) >> shift)
}

unsafe fn dram_default_mapping_init(ctx: *mut hl_ctx) -> i32 {
    let hdev = (*ctx).hdev;
    let prop = &mut (*hdev).asic_prop;
    if !prop.dram_supports_virtual_memory || !(*hdev).dram_default_page_mapping || (*ctx).asid == HL_KERNEL_ASID_ID { return 0; }
    let mut num_of_hop3 = prop.dram_size_for_default_page_mapping / prop.dram_page_size / HOP_PTE_ENTRIES_512;
    let total_hops = num_of_hop3 + 2;
    (*ctx).dram_default_hops = kcalloc(total_hops, HL_PTE_SIZE, GFP_KERNEL);
    if (*ctx).dram_default_hops.is_null() { return -ENOMEM; }
    let hop0_addr = hl_mmu_dr_get_hop0_addr(ctx);
    let hop1_addr = hl_mmu_dr_alloc_hop(ctx);
    if hop1_addr == ULLONG_MAX { dev_err((*hdev).dev, "failed to alloc hop 1\n"); kfree((*ctx).dram_default_hops); return -ENOMEM; }
    *(*ctx).dram_default_hops.add(total_hops - 1) = hop1_addr;
    let hop2_addr = hl_mmu_dr_alloc_hop(ctx);
    if hop2_addr == ULLONG_MAX { dev_err((*hdev).dev, "failed to alloc hop 2\n"); hl_mmu_dr_free_hop(ctx, hop1_addr); kfree((*ctx).dram_default_hops); return -ENOMEM; }
    *(*ctx).dram_default_hops.add(total_hops - 2) = hop2_addr;
    let mut allocated = 0usize;
    for i in 0..num_of_hop3 as usize { let a = hl_mmu_dr_alloc_hop(ctx); *(*ctx).dram_default_hops.add(i) = a; if a == ULLONG_MAX { dev_err((*hdev).dev, "failed to alloc hop 3, i: %d\n", i); for j in 0..allocated { hl_mmu_dr_free_hop(ctx, *(*ctx).dram_default_hops.add(j)); } hl_mmu_dr_free_hop(ctx, hop2_addr); hl_mmu_dr_free_hop(ctx, hop1_addr); kfree((*ctx).dram_default_hops); return -ENOMEM; } allocated += 1; }
    let mut pte_val = (hop1_addr & HOP_PHYS_ADDR_MASK) | PAGE_PRESENT_MASK;
    hl_mmu_dr_write_pte(ctx, hop0_addr, pte_val);
    pte_val = (hop2_addr & HOP_PHYS_ADDR_MASK) | PAGE_PRESENT_MASK;
    hl_mmu_dr_write_pte(ctx, hop1_addr, pte_val); hl_mmu_dr_get_pte(ctx, hop1_addr);
    let mut pte_addr = hop2_addr;
    for i in 0..num_of_hop3 as usize { pte_val = (*(*ctx).dram_default_hops.add(i) & HOP_PHYS_ADDR_MASK) | PAGE_PRESENT_MASK; hl_mmu_dr_write_pte(ctx, pte_addr, pte_val); hl_mmu_dr_get_pte(ctx, hop2_addr); pte_addr += HL_PTE_SIZE; }
    pte_val = (prop.mmu_dram_default_page_addr & HOP_PHYS_ADDR_MASK) | LAST_MASK | PAGE_PRESENT_MASK;
    for i in 0..num_of_hop3 as usize { let mut a = *(*ctx).dram_default_hops.add(i); for _ in 0..HOP_PTE_ENTRIES_512 { hl_mmu_dr_write_final_pte(ctx, a, pte_val); hl_mmu_dr_get_pte(ctx, *(*ctx).dram_default_hops.add(i)); a += HL_PTE_SIZE; } }
    hl_mmu_dr_flush(ctx); 0
}

unsafe fn dram_default_mapping_fini(ctx: *mut hl_ctx) {
    let hdev = (*ctx).hdev; let prop = &mut (*hdev).asic_prop;
    if !prop.dram_supports_virtual_memory || !(*hdev).dram_default_page_mapping || (*ctx).asid == HL_KERNEL_ASID_ID { return; }
    let n = (prop.dram_size_for_default_page_mapping / prop.dram_page_size / HOP_PTE_ENTRIES_512) as usize; let total = n + 2;
    let hop0 = hl_mmu_dr_get_hop0_addr(ctx); let hop1 = *(*ctx).dram_default_hops.add(total - 1); let hop2 = *(*ctx).dram_default_hops.add(total - 2);
    for i in 0..n { let mut a = *(*ctx).dram_default_hops.add(i); for _ in 0..HOP_PTE_ENTRIES_512 { hl_mmu_dr_clear_pte(ctx, a); hl_mmu_dr_put_pte(ctx, *(*ctx).dram_default_hops.add(i)); a += HL_PTE_SIZE; } }
    let mut a = hop2; for _ in 0..n { hl_mmu_dr_clear_pte(ctx, a); hl_mmu_dr_put_pte(ctx, hop2); a += HL_PTE_SIZE; }
    hl_mmu_dr_clear_pte(ctx, hop1); hl_mmu_dr_put_pte(ctx, hop1); hl_mmu_dr_clear_pte(ctx, hop0); kfree((*ctx).dram_default_hops); hl_mmu_dr_flush(ctx);
}

unsafe fn hl_mmu_v1_ctx_init(ctx: *mut hl_ctx) -> i32 { hash_init((*ctx).mmu_shadow_hash); dram_default_mapping_init(ctx) }

unsafe fn hl_mmu_v1_ctx_fini(ctx: *mut hl_ctx) {
    let hdev = (*ctx).hdev; dram_default_mapping_fini(ctx);
    if !hash_empty((*ctx).mmu_shadow_hash) { dev_err((*hdev).dev, "ctx %d is freed while it has pgts in use\n", (*ctx).asid); }
    let mut i = 0; let mut tmp: *mut hlist_node = core::ptr::null_mut(); let mut p: *mut pgt_info = core::ptr::null_mut();
    hash_for_each_safe((*ctx).mmu_shadow_hash, i, tmp, p, node) { dev_err_ratelimited((*hdev).dev, "pgt_info of addr 0x%llx of asid %d was not destroyed, num_ptes: %d\n", (*p).phys_addr, (*ctx).asid, (*p).num_of_ptes); hl_mmu_dr_free_pgt_node(ctx, p); }
}

unsafe fn hl_mmu_v1_unmap(ctx: *mut hl_ctx, virt_addr: u64, is_dram_addr: bool) -> i32 {
    let hdev = (*ctx).hdev; let prop = &mut (*hdev).asic_prop; let mmu_prop = if is_dram_addr { &mut prop.dmmu } else { &mut prop.pmmu };
    let mut hop_addr = [0u64; MMU_V1_MAX_HOPS]; let mut hop_pte = [0u64; MMU_V1_MAX_HOPS]; let mut curr = 0u64;
    for i in MMU_HOP0 as usize..MMU_HOP4 as usize { hop_addr[i] = if i == MMU_HOP0 as usize { hl_mmu_dr_get_hop0_addr(ctx) } else { let x = hl_mmu_get_next_hop_addr(ctx, curr); if x == ULLONG_MAX { return -EINVAL; } x }; hop_pte[i] = get_hop_pte_addr(ctx, mmu_prop, hop_addr.as_mut_ptr(), virt_addr, i); curr = *(hop_pte[i] as *const u64); }
    let huge = curr & mmu_prop.last_mask != 0; if is_dram_addr && !huge { dev_err((*hdev).dev, "DRAM unmapping should use huge pages only\n"); return -EFAULT; }
    let mut clear_hop3 = true;
    if !huge { let i = MMU_HOP4 as usize; hop_addr[i] = hl_mmu_get_next_hop_addr(ctx, curr); if hop_addr[i] == ULLONG_MAX { return -EINVAL; } hop_pte[i] = get_hop_pte_addr(ctx, mmu_prop, hop_addr.as_mut_ptr(), virt_addr, i); curr = *(hop_pte[i] as *const u64); clear_hop3 = false; }
    if (*hdev).dram_default_page_mapping && is_dram_addr { let d = (prop.mmu_dram_default_page_addr & HOP_PHYS_ADDR_MASK) | mmu_prop.last_mask | PAGE_PRESENT_MASK; if curr == d || curr & PAGE_PRESENT_MASK == 0 { dev_err((*hdev).dev, "DRAM: hop3 PTE cannot be unmapped, va: 0x%llx\n", virt_addr); return -EINVAL; } hl_mmu_dr_write_final_pte(ctx, hop_pte[MMU_HOP3 as usize], d); hl_mmu_dr_put_pte(ctx, hop_addr[MMU_HOP3 as usize]); return 0; }
    if curr & PAGE_PRESENT_MASK == 0 { return -EINVAL; } let i = if hop_addr[MMU_HOP4 as usize] != 0 { MMU_HOP4 as usize } else { MMU_HOP3 as usize }; hl_mmu_dr_clear_pte(ctx, hop_pte[i]); if hop_addr[MMU_HOP4 as usize] != 0 && hl_mmu_dr_put_pte(ctx, hop_addr[MMU_HOP4 as usize]) == 0 { clear_hop3 = true; } if clear_hop3 { for j in (0..=MMU_HOP3 as usize).rev() { hl_mmu_dr_clear_pte(ctx, hop_pte[j]); if j == 0 || hl_mmu_dr_put_pte(ctx, hop_addr[j]) != 0 { break; } } } 0
}

unsafe fn hl_mmu_v1_map(ctx: *mut hl_ctx, virt_addr: u64, phys_addr: u64, page_size: u32, is_dram_addr: bool) -> i32 {
    let hdev = (*ctx).hdev; let prop = &mut (*hdev).asic_prop; let (mmu_prop, huge) = if is_dram_addr { (&mut prop.dmmu, true) } else if page_size == prop.pmmu_huge.page_size { (&mut prop.pmmu_huge, true) } else { (&mut prop.pmmu, false) }; let n = if huge { MMU_V1_MAX_HOPS - 1 } else { MMU_V1_MAX_HOPS }; let mut addr=[0u64;MMU_V1_MAX_HOPS]; let mut pte=[0u64;MMU_V1_MAX_HOPS]; let mut new=[false;MMU_V1_MAX_HOPS]; let mut curr=0;
    for i in 0..n { addr[i]=if i==0 {hl_mmu_dr_get_hop0_addr(ctx)} else {let x=hl_mmu_dr_get_alloc_next_hop_addr(ctx,curr,&mut new[i]); if x==ULLONG_MAX{return -ENOMEM} x}; pte[i]=get_hop_pte_addr(ctx,mmu_prop,addr.as_mut_ptr(),virt_addr,i); curr=*(pte[i] as *const u64); }
    if (*hdev).dram_default_page_mapping && is_dram_addr { let d=(prop.mmu_dram_default_page_addr&HOP_PHYS_ADDR_MASK)|mmu_prop.last_mask|PAGE_PRESENT_MASK; if curr!=d || (1..n).any(|i|new[i]) { return -EINVAL; } } else if curr&PAGE_PRESENT_MASK!=0 { return -EINVAL; }
    curr=(phys_addr&HOP_PHYS_ADDR_MASK)|mmu_prop.last_mask|PAGE_PRESENT_MASK; hl_mmu_dr_write_final_pte(ctx,pte[n-1],curr); for i in 1..n { if new[i] { curr=(addr[i]&HOP_PHYS_ADDR_MASK)|PAGE_PRESENT_MASK; hl_mmu_dr_write_pte(ctx,pte[i-1],curr); if i!=1 {hl_mmu_dr_get_pte(ctx,addr[i-1]);} } } hl_mmu_dr_get_pte(ctx,addr[n-1]); 0
}

unsafe fn hl_mmu_v1_swap_out(_ctx: *mut hl_ctx) {}
unsafe fn hl_mmu_v1_swap_in(_ctx: *mut hl_ctx) {}

unsafe fn hl_mmu_v1_get_tlb_info(ctx: *mut hl_ctx, virt_addr: u64, hops: *mut hl_mmu_hop_info) -> i32 {
    let prop=&(*(*ctx).hdev).asic_prop; let (m,huge)=if hl_mem_area_inside_range(virt_addr,prop.dmmu.page_size,prop.dmmu.start_addr,prop.dmmu.end_addr){(&prop.dmmu,true)}else if hl_mem_area_inside_range(virt_addr,prop.pmmu.page_size,prop.pmmu.start_addr,prop.pmmu.end_addr){(&prop.pmmu,false)}else if hl_mem_area_inside_range(virt_addr,prop.pmmu_huge.page_size,prop.pmmu_huge.start_addr,prop.pmmu_huge.end_addr){(&prop.pmmu_huge,true)}else{return -EINVAL}; let n=m.num_hops as usize-huge as usize; (*hops).hop_info[0].hop_addr=hl_mmu_dr_get_phys_hop0_addr(ctx); (*hops).hop_info[0].hop_pte_addr=hl_mmu_get_hop_pte_phys_addr(ctx,m,0,(*hops).hop_info[0].hop_addr,virt_addr); (*hops).hop_info[0].hop_pte_val=(*(*ctx).hdev).asic_funcs.read_pte((*ctx).hdev,(*hops).hop_info[0].hop_pte_addr); for i in 1..n { (*hops).hop_info[i].hop_addr=hl_mmu_get_next_hop_addr(ctx,(*hops).hop_info[i-1].hop_pte_val); if (*hops).hop_info[i].hop_addr==ULLONG_MAX{return -EFAULT;} (*hops).hop_info[i].hop_pte_addr=hl_mmu_get_hop_pte_phys_addr(ctx,m,i,(*hops).hop_info[i].hop_addr,virt_addr); (*hops).hop_info[i].hop_pte_val=(*(*ctx).hdev).asic_funcs.read_pte((*ctx).hdev,(*hops).hop_info[i].hop_pte_addr); if (*hops).hop_info[i].hop_pte_val&PAGE_PRESENT_MASK==0{return -EFAULT;} if (*hops).hop_info[i].hop_pte_val&m.last_mask!=0{(*hops).used_hops=i+1;return 0;} } -EFAULT
}

pub unsafe fn hl_mmu_v1_set_funcs(hdev: *mut hl_device, mmu: *mut hl_mmu_funcs) { (*mmu).init=hl_mmu_dr_init; (*mmu).fini=hl_mmu_dr_fini; (*mmu).ctx_init=hl_mmu_v1_ctx_init; (*mmu).ctx_fini=hl_mmu_v1_ctx_fini; (*mmu).map=hl_mmu_v1_map; (*mmu).unmap=hl_mmu_v1_unmap; (*mmu).flush=hl_mmu_dr_flush; (*mmu).swap_out=hl_mmu_v1_swap_out; (*mmu).swap_in=hl_mmu_v1_swap_in; (*mmu).get_tlb_info=hl_mmu_v1_get_tlb_info; let _=hdev; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
