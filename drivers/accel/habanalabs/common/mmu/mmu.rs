// SPDX-License-Identifier: GPL-2.0
/* Rust translation of mmu.c; kernel types, constants, and helpers are external dependencies. */

unsafe fn hl_mmu_get_funcs(hdev: *mut hl_device, pgt_residency: i32, _is_dram_addr: bool) -> *mut hl_mmu_funcs {
    (*hdev).mmu_func.add(pgt_residency as usize)
}

pub unsafe fn hl_is_dram_va(hdev: *mut hl_device, virt_addr: u64) -> bool {
    let prop = &(*hdev).asic_prop;
    hl_mem_area_inside_range(virt_addr, prop.dmmu.page_size, prop.dmmu.start_addr, prop.dmmu.end_addr)
}

pub unsafe fn hl_mmu_init(hdev: *mut hl_device) -> i32 {
    let mut rc = -EOPNOTSUPP;
    if (*hdev).mmu_disable { return 0; }
    mutex_init(&mut (*hdev).mmu_lock);
    if (*hdev).mmu_func[MMU_DR_PGT].init.is_some() {
        rc = ((*hdev).mmu_func[MMU_DR_PGT].init.unwrap())(hdev); if rc != 0 { return rc; }
    }
    if (*hdev).mmu_func[MMU_HR_PGT].init.is_some() {
        rc = ((*hdev).mmu_func[MMU_HR_PGT].init.unwrap())(hdev); if rc != 0 { goto fini_dr_mmu; }
    }
    return 0;
fini_dr_mmu:
    if (*hdev).mmu_func[MMU_DR_PGT].fini.is_some() { ((*hdev).mmu_func[MMU_DR_PGT].fini.unwrap())(hdev); }
    rc
}

pub unsafe fn hl_mmu_fini(hdev: *mut hl_device) {
    if (*hdev).mmu_disable { return; }
    if (*hdev).mmu_func[MMU_DR_PGT].fini.is_some() { ((*hdev).mmu_func[MMU_DR_PGT].fini.unwrap())(hdev); }
    if (*hdev).mmu_func[MMU_HR_PGT].fini.is_some() { ((*hdev).mmu_func[MMU_HR_PGT].fini.unwrap())(hdev); }
    mutex_destroy(&mut (*hdev).mmu_lock);
}

pub unsafe fn hl_mmu_ctx_init(ctx: *mut hl_ctx) -> i32 {
    let hdev = (*ctx).hdev; let mut rc = -EOPNOTSUPP;
    if (*hdev).mmu_disable { return 0; }
    if (*hdev).mmu_func[MMU_DR_PGT].ctx_init.is_some() { rc = ((*hdev).mmu_func[MMU_DR_PGT].ctx_init.unwrap())(ctx); if rc != 0 { return rc; } }
    if (*hdev).mmu_func[MMU_HR_PGT].ctx_init.is_some() { rc = ((*hdev).mmu_func[MMU_HR_PGT].ctx_init.unwrap())(ctx); if rc != 0 { goto fini_dr_ctx; } }
    return 0;
fini_dr_ctx:
    if (*hdev).mmu_func[MMU_DR_PGT].fini.is_some() { ((*hdev).mmu_func[MMU_DR_PGT].fini.unwrap())(hdev); } rc
}

pub unsafe fn hl_mmu_ctx_fini(ctx: *mut hl_ctx) {
    let hdev = (*ctx).hdev; if (*hdev).mmu_disable { return; }
    if (*hdev).mmu_func[MMU_DR_PGT].ctx_fini.is_some() { ((*hdev).mmu_func[MMU_DR_PGT].ctx_fini.unwrap())(ctx); }
    if (*hdev).mmu_func[MMU_HR_PGT].ctx_fini.is_some() { ((*hdev).mmu_func[MMU_HR_PGT].ctx_fini.unwrap())(ctx); }
}

pub unsafe fn hl_mmu_get_real_page_size(hdev: *mut hl_device, mmu_prop: *mut hl_mmu_properties, page_size: u32, real_page_size: *mut u32, _is_dram_addr: bool) -> i32 {
    if page_size % (*mmu_prop).page_size == 0 { *real_page_size = (*mmu_prop).page_size; return 0; }
    dev_err((*hdev).dev, "page size of %u is not %uKB aligned, can't map\n", page_size, (*mmu_prop).page_size >> 10); -EFAULT
}

unsafe fn hl_mmu_get_prop(hdev: *mut hl_device, page_size: u32, is_dram_addr: bool) -> *mut hl_mmu_properties {
    let prop = &mut (*hdev).asic_prop;
    if is_dram_addr { &mut prop.dmmu } else if page_size % prop.pmmu_huge.page_size == 0 { &mut prop.pmmu_huge } else { &mut prop.pmmu }
}

pub unsafe fn hl_mmu_unmap_page(ctx: *mut hl_ctx, virt_addr: u64, page_size: u32, flush_pte: bool) -> i32 {
    let hdev = (*ctx).hdev; if (*hdev).mmu_disable { return 0; }
    let dram = hl_is_dram_va(hdev, virt_addr); let prop = hl_mmu_get_prop(hdev, page_size, dram);
    let funcs = hl_mmu_get_funcs(hdev, if (*prop).host_resident { MMU_HR_PGT as i32 } else { MMU_DR_PGT as i32 }, dram);
    let mut real = 0; let rc = ((*hdev).asic_funcs.mmu_get_real_page_size.unwrap())(hdev, prop, page_size, &mut real, dram); if rc != 0 { return rc; }
    let mut addr = virt_addr; let mut rc = 0; for _ in 0..(page_size / real) { rc = ((*funcs).unmap.unwrap())(ctx, addr, dram); if rc != 0 { break; } addr += real as u64; }
    if flush_pte { ((*funcs).flush.unwrap())(ctx); } rc
}

pub unsafe fn hl_mmu_map_page(ctx: *mut hl_ctx, virt_addr: u64, phys_addr: u64, page_size: u32, flush_pte: bool) -> i32 {
    let hdev = (*ctx).hdev; if (*hdev).mmu_disable { return 0; }
    let dram = hl_is_dram_va(hdev, virt_addr); let prop = hl_mmu_get_prop(hdev, page_size, dram);
    let funcs = hl_mmu_get_funcs(hdev, if (*prop).host_resident { MMU_HR_PGT as i32 } else { MMU_DR_PGT as i32 }, dram);
    let mut real = 0; let mut rc = ((*hdev).asic_funcs.mmu_get_real_page_size.unwrap())(hdev, prop, page_size, &mut real, dram); if rc != 0 { return rc; }
    let mut va = virt_addr; let mut pa = phys_addr; let mut mapped = 0; for _ in 0..(page_size / real) { rc = ((*funcs).map.unwrap())(ctx, va, pa, real, dram); if rc != 0 { break; } va += real as u64; pa += real as u64; mapped += 1; }
    if rc != 0 { let mut a = virt_addr; for _ in 0..mapped { ((*funcs).unmap.unwrap())(ctx, a, dram); a += real as u64; } ((*funcs).flush.unwrap())(ctx); return rc; }
    if flush_pte { ((*funcs).flush.unwrap())(ctx); } trace_habanalabs_mmu_map(&(*hdev).pdev.dev, virt_addr, phys_addr, page_size, flush_pte); 0
}

pub unsafe fn hl_mmu_map_contiguous(ctx: *mut hl_ctx, va: u64, pa: u64, size: u32) -> i32 { let p=&(*(*ctx).hdev).asic_prop; let ps=if hl_mem_area_inside_range(va,size,p.dmmu.start_addr,p.dmmu.end_addr){p.dmmu.page_size}else if hl_mem_area_inside_range(va,size,p.pmmu.start_addr,p.pmmu.end_addr){p.pmmu.page_size}else if hl_mem_area_inside_range(va,size,p.pmmu_huge.start_addr,p.pmmu_huge.end_addr){p.pmmu_huge.page_size}else{return -EINVAL}; let mut rc=0; let mut off=0; while off<size { rc=hl_mmu_map_page(ctx,va+off as u64,pa+off as u64,ps,off+ps>=size); if rc!=0 { off-=ps; while off as i32>=0 { let f=off<ps; hl_mmu_unmap_page(ctx,va+off as u64,ps,f); if off<ps {break} off-=ps; } break } off+=ps; } rc }

pub unsafe fn hl_mmu_unmap_contiguous(ctx:*mut hl_ctx,va:u64,size:u32)->i32 { let p=&(*(*ctx).hdev).asic_prop; let ps=if hl_mem_area_inside_range(va,size,p.dmmu.start_addr,p.dmmu.end_addr){p.dmmu.page_size}else if hl_mem_area_inside_range(va,size,p.pmmu.start_addr,p.pmmu.end_addr){p.pmmu.page_size}else if hl_mem_area_inside_range(va,size,p.pmmu_huge.start_addr,p.pmmu_huge.end_addr){p.pmmu_huge.page_size}else{return -EINVAL}; let mut rc=0; let mut off=0; while off<size {rc=hl_mmu_unmap_page(ctx,va+off as u64,ps,off+ps>=size); off+=ps;} rc }

pub unsafe fn hl_mmu_va_to_pa(ctx:*mut hl_ctx, va:u64, pa:*mut u64)->i32 { let mut hops=core::mem::zeroed::<hl_mmu_hop_info>(); let rc=hl_mmu_get_tlb_info(ctx,va,&mut hops); if rc!=0{return rc;} hl_mmu_pa_page_with_offset(ctx,va,&mut hops,pa); 0 }
unsafe fn hl_mmu_pa_page_with_offset(_ctx:*mut hl_ctx,_va:u64,_hops:*mut hl_mmu_hop_info,_pa:*mut u64) { /* dependency-specific hop layout and arithmetic are external. */ }

pub unsafe fn hl_mmu_get_next_hop_addr(_ctx:*mut hl_ctx,pte:u64)->u64 { if pte&PAGE_PRESENT_MASK != 0 {pte&HOP_PHYS_ADDR_MASK} else {ULLONG_MAX} }

pub unsafe fn hl_mmu_get_tlb_info(ctx:*mut hl_ctx, va:u64, hops:*mut hl_mmu_hop_info)->i32 {
    let hdev=(*ctx).hdev; if (*hdev).mmu_disable{return -EOPNOTSUPP;}
    (*hops).scrambled_vaddr=va; let dram=hl_mem_area_inside_range(va,(*hdev).asic_prop.dmmu.page_size,(*hdev).asic_prop.dmmu.start_addr,(*hdev).asic_prop.dmmu.end_addr);
    let p=if dram {&mut (*hdev).asic_prop.dmmu} else {&mut (*hdev).asic_prop.pmmu}; let f=hl_mmu_get_funcs(hdev,if p.host_resident{MMU_HR_PGT as i32}else{MMU_DR_PGT as i32},dram);
    mutex_lock(&mut (*hdev).mmu_lock); let rc=((*f).get_tlb_info.unwrap())(ctx,va,hops); mutex_unlock(&mut (*hdev).mmu_lock); rc
}

pub unsafe fn hl_mmu_if_set_funcs(hdev:*mut hl_device)->i32 { if (*hdev).mmu_disable{return 0;} match (*hdev).asic_type { ASIC_GOYA|ASIC_GAUDI|ASIC_GAUDI_SEC=>hl_mmu_v1_set_funcs(hdev,&mut (*hdev).mmu_func[MMU_DR_PGT]), ASIC_GAUDI2|ASIC_GAUDI2B|ASIC_GAUDI2C|ASIC_GAUDI2D=>{hl_mmu_v2_set_funcs(hdev,&mut (*hdev).mmu_func[MMU_DR_PGT]);if (*hdev).asic_prop.pmmu.host_resident{hl_mmu_v2_hr_set_funcs(hdev,&mut (*hdev).mmu_func[MMU_HR_PGT])} 0}, _=>-EOPNOTSUPP} }
pub unsafe fn hl_mmu_scramble_addr(_hdev:*mut hl_device,a:u64)->u64{a}
pub unsafe fn hl_mmu_descramble_addr(_hdev:*mut hl_device,a:u64)->u64{a}
pub unsafe fn hl_mmu_invalidate_cache(hdev:*mut hl_device,hard:bool,flags:u32)->i32{((*hdev).asic_funcs.mmu_invalidate_cache.unwrap())(hdev,hard,flags)}
pub unsafe fn hl_mmu_invalidate_cache_range(hdev:*mut hl_device,hard:bool,flags:u32,asid:u32,va:u64,size:u64)->i32{((*hdev).asic_funcs.mmu_invalidate_cache_range.unwrap())(hdev,hard,flags,asid,va,size)}
pub unsafe fn hl_mmu_hr_flush(_ctx:*mut hl_ctx){mb();}
pub unsafe fn hl_mmu_get_hop_pte_phys_addr(ctx:*mut hl_ctx,p:*mut hl_mmu_properties,idx:u8,hop:u64,va:u64)->u64{if idx as u32>=(*p).num_hops{return U64_MAX;}hop+(*(*ctx).hdev).asic_prop.mmu_pte_size*((va&(*p).hop_masks[idx as usize])>>(*p).hop_shifts[idx as usize])}
pub unsafe fn hl_mmu_hr_pte_phys_to_virt(_ctx:*mut hl_ctx,pgt:*mut pgt_info,phys:u64,size:u32)->u64{(*pgt).virt_addr+(phys&(size as u64-1))}
pub unsafe fn hl_mmu_prefetch_cache_range(ctx:*mut hl_ctx,flags:u32,asid:u32,va:u64,size:u64)->i32{((*(*ctx).hdev).asic_funcs.mmu_prefetch_cache_range.unwrap())(ctx,flags,asid,va,size);0}
pub unsafe fn hl_mmu_hr_free_hop_remove_pgt(pgt:*mut pgt_info,hr:*mut hl_mmu_hr_priv,size:u32){gen_pool_free((*hr).mmu_pgt_pool,(*pgt).virt_addr,size);hash_del(&mut (*pgt).node);kfree(pgt as *mut core::ffi::c_void);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
