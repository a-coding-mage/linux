// SPDX-License-Identifier: GPL-2.0
// pr_fmt(fmt) = "kasan: " fmt
// cpu_feature_enabled() cannot be used this early
// #define USE_EARLY_PGTABLE_L5

extern "C" {
    static mut pfn_mapped: [range; E820_MAX_ENTRIES];
    static mut tmp_p4d_table: [p4d_t; MAX_PTRS_PER_P4D];
}

unsafe fn early_alloc(size: usize, nid: i32, should_panic: bool) -> *mut core::ffi::c_void {
    let ptr = memblock_alloc_try_nid(size, size, __pa(MAX_DMA_ADDRESS), MEMBLOCK_ALLOC_ACCESSIBLE, nid);
    if ptr.is_null() && should_panic {
        panic_("%pS: Failed to allocate page, nid=%d from=%lx\n", _RET_IP_, nid, __pa(MAX_DMA_ADDRESS));
    }
    ptr
}

unsafe fn kasan_populate_pmd(mut pmd: *mut pmd_t, mut addr: usize, end: usize, nid: i32) {
    if pmd_none(*pmd) {
        let mut p: *mut core::ffi::c_void;
        if boot_cpu_has(X86_FEATURE_PSE) && end.wrapping_sub(addr) == PMD_SIZE && IS_ALIGNED(addr, PMD_SIZE) {
            p = early_alloc(PMD_SIZE, nid, false);
            if !p.is_null() && pmd_set_huge(pmd, __pa(p), PAGE_KERNEL) { return; }
            memblock_free(p, PMD_SIZE);
        }
        p = early_alloc(PAGE_SIZE, nid, true);
        pmd_populate_kernel(&init_mm, pmd, p);
    }
    let mut pte = pte_offset_kernel(pmd, addr);
    loop {
        if pte_none(*pte) {
            let p = early_alloc(PAGE_SIZE, nid, true);
            let entry = pfn_pte(PFN_DOWN(__pa(p)), PAGE_KERNEL);
            set_pte_at(&init_mm, addr, pte, entry);
        }
        pte = pte.add(1); addr += PAGE_SIZE;
        if addr == end { break; }
    }
}

unsafe fn kasan_populate_pud(mut pud: *mut pud_t, mut addr: usize, end: usize, nid: i32) {
    if pud_none(*pud) {
        let mut p: *mut core::ffi::c_void;
        if boot_cpu_has(X86_FEATURE_GBPAGES) && end.wrapping_sub(addr) == PUD_SIZE && IS_ALIGNED(addr, PUD_SIZE) {
            p = early_alloc(PUD_SIZE, nid, false);
            if !p.is_null() && pud_set_huge(pud, __pa(p), PAGE_KERNEL) { return; }
            memblock_free(p, PUD_SIZE);
        }
        p = early_alloc(PAGE_SIZE, nid, true); pud_populate(&init_mm, pud, p);
    }
    let mut pmd = pmd_offset(pud, addr);
    loop { let next = pmd_addr_end(addr, end); if !pmd_leaf(*pmd) { kasan_populate_pmd(pmd, addr, next, nid); } pmd=pmd.add(1); addr=next; if addr==end {break;} }
}

unsafe fn kasan_populate_p4d(mut p4d: *mut p4d_t, mut addr: usize, end: usize, nid: i32) {
    if p4d_none(*p4d) { p4d_populate(&init_mm, p4d, early_alloc(PAGE_SIZE, nid, true)); }
    let mut pud = pud_offset(p4d, addr);
    loop { let next=pud_addr_end(addr,end); if !pud_leaf(*pud) { kasan_populate_pud(pud,addr,next,nid); } pud=pud.add(1); addr=next; if addr==end {break;} }
}

unsafe fn kasan_populate_pgd(mut pgd: *mut pgd_t, mut addr: usize, end: usize, nid: i32) {
    if pgd_none(*pgd) { pgd_populate(&init_mm, pgd, early_alloc(PAGE_SIZE,nid,true)); }
    let mut p4d=p4d_offset(pgd,addr);
    loop { let next=p4d_addr_end(addr,end); kasan_populate_p4d(p4d,addr,next,nid); p4d=p4d.add(1); addr=next; if addr==end {break;} }
}

unsafe fn kasan_populate_shadow(mut addr: usize, end0: usize, nid: i32) {
    let end=round_up(end0,PAGE_SIZE); addr &= PAGE_MASK; let mut pgd=pgd_offset_k(addr);
    loop { let next=pgd_addr_end(addr,end); kasan_populate_pgd(pgd,addr,next,nid); pgd=pgd.add(1); addr=next; if addr==end {break;} }
}

unsafe fn map_range(r: *mut range) { let start=kasan_mem_to_shadow(pfn_to_kaddr((*r).start)) as usize; let end=kasan_mem_to_shadow(pfn_to_kaddr((*r).end)) as usize; kasan_populate_shadow(start,end,early_pfn_to_nid((*r).start)); }

unsafe fn clear_pgds(mut start: usize, end: usize) {
    let pgd_end=end & PGDIR_MASK; while start<pgd_end { let pgd=pgd_offset_k(start); if pgtable_l5_enabled() { pgd_clear(pgd); } else { p4d_clear(p4d_offset(pgd,start)); } start+=PGDIR_SIZE; }
    let pgd=pgd_offset_k(start); while start<end { p4d_clear(p4d_offset(pgd,start)); start+=P4D_SIZE; }
}

unsafe fn early_p4d_offset(pgd: *mut pgd_t, addr: usize) -> *mut p4d_t { if !pgtable_l5_enabled() { return pgd as *mut p4d_t; } let p4d=(pgd_val(*pgd)&PTE_PFN_MASK)+__START_KERNEL_map-phys_base; (p4d as *mut p4d_t).add(p4d_index(addr)) }

unsafe fn kasan_early_p4d_populate(mut pgd: *mut pgd_t, mut addr: usize, end: usize) {
    if pgd_none(*pgd) { set_pgd(pgd,__pgd(_KERNPG_TABLE|__pa_nodebug(kasan_early_shadow_p4d))); }
    let mut p4d=early_p4d_offset(pgd,addr); loop { let next=p4d_addr_end(addr,end); if !p4d_none(*p4d) { } else { set_p4d(p4d,__p4d(_KERNPG_TABLE|__pa_nodebug(kasan_early_shadow_pud))); } p4d=p4d.add(1); addr=next; if addr==end || !p4d_none(*p4d) {break;} }
}

unsafe fn kasan_map_early_shadow(mut pgd: *mut pgd_t) { let mut addr=KASAN_SHADOW_START&PGDIR_MASK; let end=KASAN_SHADOW_END; pgd=pgd.add(pgd_index(addr)); loop { let next=pgd_addr_end(addr,end); kasan_early_p4d_populate(pgd,addr,next); pgd=pgd.add(1); addr=next; if addr==end {break;} } }

unsafe fn kasan_shallow_populate_p4ds(pgd:*mut pgd_t,mut addr:usize,end:usize) { let mut p4d=p4d_offset(pgd,addr); loop { let next=p4d_addr_end(addr,end); if p4d_none(*p4d) { p4d_populate(&init_mm,p4d,early_alloc(PAGE_SIZE,NUMA_NO_NODE,true)); } p4d=p4d.add(1); addr=next; if addr==end {break;} } }
unsafe fn kasan_shallow_populate_pgds(start:*mut core::ffi::c_void,end:*mut core::ffi::c_void) { let mut addr=start as usize; let e=end as usize; let mut pgd=pgd_offset_k(addr); loop { let next=pgd_addr_end(addr,e); if pgd_none(*pgd) { pgd_populate(&init_mm,pgd,early_alloc(PAGE_SIZE,NUMA_NO_NODE,true)); } kasan_shallow_populate_p4ds(pgd,addr,next); pgd=pgd.add(1); addr=next; if addr==e {break;} } }

pub unsafe extern "C" fn kasan_early_init() { let mut i=0; let mut pte_val=__pa_nodebug(kasan_early_shadow_page)|__PAGE_KERNEL|_PAGE_ENC; let mut pmd_val=__pa_nodebug(kasan_early_shadow_pte)|_KERNPG_TABLE; let mut pud_val=__pa_nodebug(kasan_early_shadow_pmd)|_KERNPG_TABLE; let mut p4d_val=__pa_nodebug(kasan_early_shadow_pud)|_KERNPG_TABLE; pte_val&=__default_kernel_pte_mask; pmd_val&=__default_kernel_pte_mask; pud_val&=__default_kernel_pte_mask; p4d_val&=__default_kernel_pte_mask; while i<PTRS_PER_PTE { kasan_early_shadow_pte[i]=__pte(pte_val); i+=1;} i=0; while i<PTRS_PER_PMD {kasan_early_shadow_pmd[i]=__pmd(pmd_val);i+=1;} i=0; while i<PTRS_PER_PUD {kasan_early_shadow_pud[i]=__pud(pud_val);i+=1;} i=0; while pgtable_l5_enabled()&&i<PTRS_PER_P4D {kasan_early_shadow_p4d[i]=__p4d(p4d_val);i+=1;} kasan_map_early_shadow(early_top_pgt); kasan_map_early_shadow(init_top_pgt); }

unsafe fn kasan_mem_to_shadow_align_down(va:usize)->usize { round_down(kasan_mem_to_shadow(va as *mut _ ) as usize,PAGE_SIZE) }
unsafe fn kasan_mem_to_shadow_align_up(va:usize)->usize { round_up(kasan_mem_to_shadow(va as *mut _ ) as usize,PAGE_SIZE) }
pub unsafe extern "C" fn kasan_populate_shadow_for_vaddr(va:*mut core::ffi::c_void,size:usize,nid:i32) { kasan_populate_shadow(kasan_mem_to_shadow_align_down(va as usize),kasan_mem_to_shadow_align_up((va as usize)+size),nid); }

pub unsafe extern "C" fn kasan_init() {
    let mut shadow_cea_begin; let mut shadow_cea_per_cpu_begin; let mut shadow_cea_end; let mut i;
    memcpy(early_top_pgt,init_top_pgt,core::mem::size_of_val(&early_top_pgt));
    if pgtable_l5_enabled() { let ptr=pgd_page_vaddr(*pgd_offset_k(KASAN_SHADOW_END)); memcpy(tmp_p4d_table.as_mut_ptr() as *mut _,ptr,core::mem::size_of_val(&tmp_p4d_table)); set_pgd(&mut early_top_pgt[pgd_index(KASAN_SHADOW_END)],__pgd(__pa(tmp_p4d_table.as_mut_ptr())|_KERNPG_TABLE)); }
    load_cr3(early_top_pgt); __flush_tlb_all(); clear_pgds(KASAN_SHADOW_START&PGDIR_MASK,KASAN_SHADOW_END); kasan_populate_early_shadow((KASAN_SHADOW_START&PGDIR_MASK) as *mut _,kasan_mem_to_shadow(PAGE_OFFSET as *mut _));
    i=0; while i<E820_MAX_ENTRIES { if pfn_mapped[i].end==0 {break;} map_range(&mut pfn_mapped[i]); i+=1; }
    shadow_cea_begin=kasan_mem_to_shadow_align_down(CPU_ENTRY_AREA_BASE); shadow_cea_per_cpu_begin=kasan_mem_to_shadow_align_up(CPU_ENTRY_AREA_PER_CPU); shadow_cea_end=kasan_mem_to_shadow_align_up(CPU_ENTRY_AREA_BASE+CPU_ENTRY_AREA_MAP_SIZE);
    kasan_populate_early_shadow(kasan_mem_to_shadow((PAGE_OFFSET+MAXMEM) as *mut _),kasan_mem_to_shadow(VMALLOC_START as *mut _));
    if IS_ENABLED(CONFIG_KASAN_VMALLOC) { kasan_shallow_populate_pgds(kasan_mem_to_shadow(VMALLOC_START as *mut _),kasan_mem_to_shadow(VMALLOC_END as *mut _)); } else { kasan_populate_early_shadow(kasan_mem_to_shadow(VMALLOC_START as *mut _),kasan_mem_to_shadow(VMALLOC_END as *mut _)); }
    kasan_populate_early_shadow(kasan_mem_to_shadow((VMALLOC_END+1) as *mut _),shadow_cea_begin as *mut _); kasan_populate_shadow(shadow_cea_begin,shadow_cea_per_cpu_begin,0); kasan_populate_early_shadow(shadow_cea_end as *mut _,kasan_mem_to_shadow(__START_KERNEL_map as *mut _)); kasan_populate_shadow(kasan_mem_to_shadow(_stext) as usize,kasan_mem_to_shadow(_end) as usize,early_pfn_to_nid(__pa(_stext))); kasan_populate_early_shadow(kasan_mem_to_shadow(MODULES_END as *mut _),(KASAN_SHADOW_END) as *mut _);
    load_cr3(init_top_pgt); __flush_tlb_all(); memset(kasan_early_shadow_page,0,PAGE_SIZE); i=0; while i<PTRS_PER_PTE { let prot=__pgprot(__PAGE_KERNEL_RO|_PAGE_ENC); let pte=__pte(__pa(kasan_early_shadow_page)|(pgprot_val(prot)&__default_kernel_pte_mask)); set_pte(&mut kasan_early_shadow_pte[i],pte); i+=1; } __flush_tlb_all(); init_task.kasan_depth=0; kasan_init_generic();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
