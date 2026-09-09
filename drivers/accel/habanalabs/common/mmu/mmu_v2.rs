// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2016-2020 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

// Dependencies are supplied by the surrounding kernel translation.

/*
 * hl_mmu_v2_ctx_init() - initialize a context for using the MMU module.
 * @ctx: pointer to the context structure to initialize.
 *
 * Initialize a mutex to protect the concurrent mapping flow, a hash to hold all
 * page tables hops related to this context.
 * Return: 0 on success, non-zero otherwise.
 */
unsafe fn hl_mmu_v2_ctx_init(ctx: *mut hl_ctx) -> i32 {
    hash_init((*ctx).mmu_shadow_hash);

    0
}

/*
 * hl_mmu_v2_ctx_fini - disable a ctx from using the mmu module
 *
 * @ctx: pointer to the context structure
 *
 * This function does the following:
 * - Free any pgts which were not freed yet
 * - Free the mutex
 * - Free DRAM default page mapping hops
 */
unsafe fn hl_mmu_v2_ctx_fini(ctx: *mut hl_ctx) {
    let hdev = (*ctx).hdev;
    let mut pgt_info: *mut pgt_info;
    let mut tmp: *mut hlist_node;
    let mut i: i32;

    if !hash_empty((*ctx).mmu_shadow_hash) {
        dev_err((*hdev).dev, "ctx %d is freed while it has pgts in use\n", (*ctx).asid);
    }

    hash_for_each_safe((*ctx).mmu_shadow_hash, i, tmp, pgt_info, node) {
        dev_err_ratelimited(
            (*hdev).dev,
            "pgt_info of addr 0x%llx of asid %d was not destroyed, num_ptes: %d\n",
            (*pgt_info).phys_addr,
            (*ctx).asid,
            (*pgt_info).num_of_ptes,
        );
        hl_mmu_dr_free_pgt_node(ctx, pgt_info);
    }
}

unsafe fn hl_mmu_v2_unmap(ctx: *mut hl_ctx, virt_addr: u64, is_dram_addr: bool) -> i32 {
    let mut hop_addr = [0u64; MMU_ARCH_6_HOPS];
    let mut hop_pte_addr = [0u64; MMU_ARCH_6_HOPS];
    let mut curr_pte: u64;
    let mut scrambled_virt_addr: u64;
    let prop = &mut (*(*ctx).hdev).asic_prop;
    let hdev = (*ctx).hdev;
    let mmu_prop: *mut hl_mmu_properties;
    let mut is_huge = false;
    let mut i: i32;
    let mut hop_last: i32;

    /* device resident in V2 are allowed only for HMMU */
    if !is_dram_addr { return -EINVAL; }

    mmu_prop = &mut prop.dmmu;
    hop_last = (*mmu_prop).num_hops - 1;
    scrambled_virt_addr = ((*(*hdev).asic_funcs).scramble_addr)(hdev, virt_addr);
    hop_addr[0] = hl_mmu_dr_get_hop0_addr(ctx);
    hop_pte_addr[0] = hl_mmu_get_hop_pte_phys_addr(ctx, mmu_prop, 0, hop_addr[0], scrambled_virt_addr);
    if hop_pte_addr[0] == U64_MAX { return -EFAULT; }
    curr_pte = core::ptr::read_volatile(hop_pte_addr[0] as *const u64);

    i = 1;
    while i < (*mmu_prop).num_hops {
        hop_addr[i as usize] = hl_mmu_get_next_hop_addr(ctx, curr_pte);
        if hop_addr[i as usize] == ULLONG_MAX { break; }
        hop_pte_addr[i as usize] = hl_mmu_get_hop_pte_phys_addr(ctx, mmu_prop, i, hop_addr[i as usize], scrambled_virt_addr);
        if hop_pte_addr[i as usize] == U64_MAX { return -EFAULT; }
        curr_pte = core::ptr::read_volatile(hop_pte_addr[i as usize] as *const u64);
        if i <= hop_last && (curr_pte & (*mmu_prop).last_mask) != 0 {
            hop_last = i; is_huge = true; break;
        }
        i += 1;
    }
    if !is_dram_addr || !is_huge { dev_err((*hdev).dev, "DRAM unmapping should use huge pages only\n"); return -EFAULT; }
    if (curr_pte & PAGE_PRESENT_MASK) == 0 { dev_err((*hdev).dev, "virt addr 0x%llx is not mapped to phys addr\n", virt_addr); return -EINVAL; }
    i = hop_last;
    while i > 0 {
        hl_mmu_dr_clear_pte(ctx, hop_pte_addr[i as usize]);
        if hl_mmu_dr_put_pte(ctx, hop_addr[i as usize]) { break; }
        i -= 1;
    }
    hl_mmu_dr_clear_pte(ctx, hop_pte_addr[0]);
    0
}

unsafe fn hl_mmu_v2_map(ctx: *mut hl_ctx, virt_addr: u64, phys_addr: u64, page_size: u32, is_dram_addr: bool) -> i32 {
    let mut hop_addr = [0u64; MMU_ARCH_6_HOPS];
    let mut hop_pte_addr = [0u64; MMU_ARCH_6_HOPS];
    let mut curr_pte = 0u64;
    let mut hop_new = [false; MMU_ARCH_6_HOPS];
    let hdev = (*ctx).hdev;
    let prop = &mut (*hdev).asic_prop;
    let mmu_prop = &mut prop.dmmu;
    let mut rc: i32;
    let mut i: i32;
    let hop_last = mmu_prop.num_hops - 1;
    if !is_dram_addr { return -EINVAL; }
    let scrambled_virt_addr = ((*(*hdev).asic_funcs).scramble_addr)(hdev, virt_addr);
    let scrambled_phys_addr = ((*(*hdev).asic_funcs).scramble_addr)(hdev, phys_addr);
    hop_addr[0] = hl_mmu_dr_get_hop0_addr(ctx);
    hop_pte_addr[0] = hl_mmu_get_hop_pte_phys_addr(ctx, mmu_prop, 0, hop_addr[0], scrambled_virt_addr);
    curr_pte = core::ptr::read_volatile(hop_pte_addr[0] as *const u64);
    i = 1;
    while i <= hop_last {
        hop_addr[i as usize] = hl_mmu_dr_get_alloc_next_hop_addr(ctx, curr_pte, &mut hop_new[i as usize]);
        if hop_addr[i as usize] == ULLONG_MAX { rc = -ENOMEM; goto_err(ctx, &hop_addr, &hop_new, hop_last, rc); return rc; }
        hop_pte_addr[i as usize] = hl_mmu_get_hop_pte_phys_addr(ctx, mmu_prop, i, hop_addr[i as usize], scrambled_virt_addr);
        if hop_pte_addr[i as usize] == U64_MAX || hop_pte_addr[i as usize] == 0 { rc = -EINVAL; goto_err(ctx, &hop_addr, &hop_new, hop_last, rc); return rc; }
        curr_pte = core::ptr::read_volatile(hop_pte_addr[i as usize] as *const u64);
        i += 1;
    }
    if (curr_pte & PAGE_PRESENT_MASK) != 0 { dev_err((*hdev).dev, "mapping already exists for virt_addr 0x%llx\n", virt_addr); rc = -EINVAL; goto_err(ctx, &hop_addr, &hop_new, hop_last, rc); return rc; }
    curr_pte = (scrambled_phys_addr & HOP_PHYS_ADDR_MASK) | mmu_prop.last_mask | PAGE_PRESENT_MASK;
    hl_mmu_dr_write_final_pte(ctx, hop_pte_addr[hop_last as usize], curr_pte);
    i = 1;
    while i <= hop_last { if hop_new[i as usize] { curr_pte = (hop_addr[i as usize] & HOP_PHYS_ADDR_MASK) | PAGE_PRESENT_MASK; hl_mmu_dr_write_pte(ctx, hop_pte_addr[(i - 1) as usize], curr_pte); if i - 1 != 0 { hl_mmu_dr_get_pte(ctx, hop_addr[(i - 1) as usize]); } } i += 1; }
    hl_mmu_dr_get_pte(ctx, hop_addr[hop_last as usize]);
    0
}

unsafe fn goto_err(ctx: *mut hl_ctx, hop_addr: &[u64; MMU_ARCH_6_HOPS], hop_new: &[bool; MMU_ARCH_6_HOPS], hop_last: i32, _rc: i32) { let mut i = 1; while i <= hop_last { if hop_new[i as usize] && hop_addr[i as usize] != U64_MAX { hl_mmu_dr_free_hop(ctx, hop_addr[i as usize]); } i += 1; } }

unsafe fn hl_mmu_v2_swap_out(_ctx: *mut hl_ctx) {}
unsafe fn hl_mmu_v2_swap_in(_ctx: *mut hl_ctx) {}

unsafe fn hl_mmu_v2_get_tlb_info(ctx: *mut hl_ctx, virt_addr: u64, hops: *mut hl_mmu_hop_info) -> i32 {
    let hdev = (*ctx).hdev;
    let prop = &mut (*hdev).asic_prop;
    if !hl_mem_area_inside_range(virt_addr, prop.dmmu.page_size, prop.dmmu.start_addr, prop.dmmu.end_addr) { return -EINVAL; }
    let mmu_prop = &mut prop.dmmu;
    (*hops).range_type = HL_VA_RANGE_TYPE_DRAM;
    (*hops).scrambled_vaddr = ((*(*hdev).asic_funcs).scramble_addr)(hdev, virt_addr);
    (*hops).hop_info[0].hop_addr = hl_mmu_dr_get_phys_hop0_addr(ctx);
    (*hops).hop_info[0].hop_pte_addr = hl_mmu_get_hop_pte_phys_addr(ctx, mmu_prop, 0, (*hops).hop_info[0].hop_addr, (*hops).scrambled_vaddr);
    if (*hops).hop_info[0].hop_pte_addr == U64_MAX { return -EFAULT; }
    (*hops).hop_info[0].hop_pte_val = ((*(*hdev).asic_funcs).read_pte)(hdev, (*hops).hop_info[0].hop_pte_addr);
    if (*hops).hop_info[0].hop_pte_val == U64_MAX { return -EFAULT; }
    let mut i = 1;
    while i < mmu_prop.num_hops { (*hops).hop_info[i as usize].hop_addr = hl_mmu_get_next_hop_addr(ctx, (*hops).hop_info[(i - 1) as usize].hop_pte_val); if (*hops).hop_info[i as usize].hop_addr == ULLONG_MAX { return -EFAULT; } (*hops).hop_info[i as usize].hop_pte_addr = hl_mmu_get_hop_pte_phys_addr(ctx, mmu_prop, i, (*hops).hop_info[i as usize].hop_addr, (*hops).scrambled_vaddr); if (*hops).hop_info[i as usize].hop_pte_addr == U64_MAX { return -EFAULT; } (*hops).hop_info[i as usize].hop_pte_val = ((*(*hdev).asic_funcs).read_pte)(hdev, (*hops).hop_info[i as usize].hop_pte_addr); if ((*hops).hop_info[i as usize].hop_pte_val & PAGE_PRESENT_MASK) == 0 { return -EFAULT; } if ((*hops).hop_info[i as usize].hop_pte_val & mmu_prop.last_mask) != 0 { break; } i += 1; }
    if i == mmu_prop.num_hops || ((*hops).hop_info[i as usize].hop_pte_val & PAGE_PRESENT_MASK) == 0 { return -EFAULT; }
    (*hops).unscrambled_paddr = if (*hops).scrambled_vaddr != virt_addr { ((*(*hdev).asic_funcs).descramble_addr)(hdev, (*hops).hop_info[i as usize].hop_pte_val) } else { (*hops).hop_info[i as usize].hop_pte_val };
    (*hops).used_hops = i + 1;
    0
}

pub unsafe fn hl_mmu_v2_set_funcs(hdev: *mut hl_device, mmu: *mut hl_mmu_funcs) {
    (*mmu).init = hl_mmu_dr_init; (*mmu).fini = hl_mmu_dr_fini; (*mmu).ctx_init = hl_mmu_v2_ctx_init; (*mmu).ctx_fini = hl_mmu_v2_ctx_fini; (*mmu).map = hl_mmu_v2_map; (*mmu).unmap = hl_mmu_v2_unmap; (*mmu).flush = hl_mmu_dr_flush; (*mmu).swap_out = hl_mmu_v2_swap_out; (*mmu).swap_in = hl_mmu_v2_swap_in; (*mmu).get_tlb_info = hl_mmu_v2_get_tlb_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
