// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Andes Technology Corporation

/* Dependencies are supplied by the surrounding kernel translation unit. */

static mut TMP_PG_DIR: [pgd_t; PTRS_PER_PGD] = [pgd_t::default(); PTRS_PER_PGD];
static mut TMP_P4D: [p4d_t; PTRS_PER_P4D] = [p4d_t::default(); PTRS_PER_P4D];
static mut TMP_PUD: [pud_t; PTRS_PER_PUD] = [pud_t::default(); PTRS_PER_PUD];

unsafe fn kasan_populate_pte(mut pmd: *mut pmd_t, mut vaddr: usize, end: usize) {
    let mut phys_addr: phys_addr_t;
    let p: *mut pte_t;
    if pmd_none(pmdp_get(pmd)) {
        p = memblock_alloc_or_panic(PTRS_PER_PTE * core::mem::size_of::<pte_t>(), PAGE_SIZE) as *mut pte_t;
        set_pmd(pmd, pfn_pmd(PFN_DOWN(__pa(p as usize)), PAGE_TABLE));
    }
    let mut ptep = pte_offset_kernel(pmd, vaddr);
    loop {
        if pte_none(ptep_get(ptep)) {
            phys_addr = memblock_phys_alloc(PAGE_SIZE, PAGE_SIZE);
            set_pte(ptep, pfn_pte(PFN_DOWN(phys_addr), PAGE_KERNEL));
            memset(__va(phys_addr), KASAN_SHADOW_INIT, PAGE_SIZE);
        }
        ptep = ptep.add(1); vaddr = vaddr.wrapping_add(PAGE_SIZE);
        if vaddr == end { break; }
    }
}

unsafe fn kasan_populate_pmd(mut pud: *mut pud_t, mut vaddr: usize, end: usize) {
    let p: *mut pmd_t;
    if pud_none(pudp_get(pud)) {
        p = memblock_alloc_or_panic(PTRS_PER_PMD * core::mem::size_of::<pmd_t>(), PAGE_SIZE) as *mut pmd_t;
        set_pud(pud, pfn_pud(PFN_DOWN(__pa(p as usize)), PAGE_TABLE));
    }
    let mut pmdp = pmd_offset(pud, vaddr);
    loop {
        let next = pmd_addr_end(vaddr, end);
        if pmd_none(pmdp_get(pmdp)) && IS_ALIGNED(vaddr, PMD_SIZE) && next - vaddr >= PMD_SIZE {
            let phys_addr = memblock_phys_alloc(PMD_SIZE, PMD_SIZE);
            if phys_addr != 0 {
                set_pmd(pmdp, pfn_pmd(PFN_DOWN(phys_addr), PAGE_KERNEL));
                memset(__va(phys_addr), KASAN_SHADOW_INIT, PMD_SIZE);
                pmdp = pmdp.add(1); vaddr = next;
                if vaddr == end { break; } else { continue; }
            }
        }
        kasan_populate_pte(pmdp, vaddr, next);
        pmdp = pmdp.add(1); vaddr = next;
        if vaddr == end { break; }
    }
}

unsafe fn kasan_populate_pud(mut p4d: *mut p4d_t, mut vaddr: usize, end: usize) {
    let p: *mut pud_t;
    if p4d_none(p4dp_get(p4d)) {
        p = memblock_alloc_or_panic(PTRS_PER_PUD * core::mem::size_of::<pud_t>(), PAGE_SIZE) as *mut pud_t;
        set_p4d(p4d, pfn_p4d(PFN_DOWN(__pa(p as usize)), PAGE_TABLE));
    }
    let mut pudp = pud_offset(p4d, vaddr);
    loop {
        let next = pud_addr_end(vaddr, end);
        if pud_none(pudp_get(pudp)) && IS_ALIGNED(vaddr, PUD_SIZE) && next - vaddr >= PUD_SIZE {
            let phys_addr = memblock_phys_alloc(PUD_SIZE, PUD_SIZE);
            if phys_addr != 0 { set_pud(pudp, pfn_pud(PFN_DOWN(phys_addr), PAGE_KERNEL)); memset(__va(phys_addr), KASAN_SHADOW_INIT, PUD_SIZE); pudp = pudp.add(1); vaddr = next; if vaddr == end { break; } else { continue; } }
        }
        kasan_populate_pmd(pudp, vaddr, next); pudp = pudp.add(1); vaddr = next; if vaddr == end { break; }
    }
}

unsafe fn kasan_populate_p4d(mut pgd: *mut pgd_t, mut vaddr: usize, end: usize) {
    let p: *mut p4d_t;
    if pgd_none(pgdp_get(pgd)) { p = memblock_alloc_or_panic(PTRS_PER_P4D * core::mem::size_of::<p4d_t>(), PAGE_SIZE) as *mut p4d_t; set_pgd(pgd, pfn_pgd(PFN_DOWN(__pa(p as usize)), PAGE_TABLE)); }
    let mut p4dp = p4d_offset(pgd, vaddr);
    loop { let next = p4d_addr_end(vaddr, end); if p4d_none(p4dp_get(p4dp)) && IS_ALIGNED(vaddr, P4D_SIZE) && next-vaddr >= P4D_SIZE { let a=memblock_phys_alloc(P4D_SIZE,P4D_SIZE); if a != 0 { set_p4d(p4dp,pfn_p4d(PFN_DOWN(a),PAGE_KERNEL)); memset(__va(a),KASAN_SHADOW_INIT,P4D_SIZE); p4dp=p4dp.add(1);vaddr=next;if vaddr==end{break}else{continue;} } } kasan_populate_pud(p4dp,vaddr,next);p4dp=p4dp.add(1);vaddr=next;if vaddr==end{break;} }
}

unsafe fn kasan_populate_pgd(mut pgdp: *mut pgd_t, mut vaddr: usize, end: usize) {
    loop { let next=pgd_addr_end(vaddr,end); if pgd_none(pgdp_get(pgdp)) && IS_ALIGNED(vaddr,PGDIR_SIZE) && next-vaddr>=PGDIR_SIZE { let a=memblock_phys_alloc(PGDIR_SIZE,PGDIR_SIZE); if a!=0 {set_pgd(pgdp,pfn_pgd(PFN_DOWN(a),PAGE_KERNEL));memset(__va(a),KASAN_SHADOW_INIT,PGDIR_SIZE);pgdp=pgdp.add(1);vaddr=next;if vaddr==end{break}else{continue;}}} kasan_populate_p4d(pgdp,vaddr,next);pgdp=pgdp.add(1);vaddr=next;if vaddr==end{break;} }
}

unsafe fn kasan_early_clear_pud(mut p4dp:*mut p4d_t,mut vaddr:usize,end:usize){let mut pudp:*mut pud_t;if !pgtable_l4_enabled{pudp=p4dp as *mut pud_t;}else{let b=pt_ops.get_pud_virt(pfn_to_phys(_p4d_pfn(p4dp_get(p4dp))));pudp=b.add(pud_index(vaddr));}loop{let next=pud_addr_end(vaddr,end);if IS_ALIGNED(vaddr,PUD_SIZE)&&next-vaddr>=PUD_SIZE{pud_clear(pudp);pudp=pudp.add(1);vaddr=next;if vaddr==end{break}else{continue;}}BUG();}}
unsafe fn kasan_early_clear_p4d(mut pgdp:*mut pgd_t,mut vaddr:usize,end:usize){let mut p4dp:*mut p4d_t;if !pgtable_l5_enabled{p4dp=pgdp as *mut p4d_t;}else{let b=pt_ops.get_p4d_virt(pfn_to_phys(_pgd_pfn(pgdp_get(pgdp))));p4dp=b.add(p4d_index(vaddr));}loop{let next=p4d_addr_end(vaddr,end);if pgtable_l4_enabled&&IS_ALIGNED(vaddr,P4D_SIZE)&&next-vaddr>=P4D_SIZE{p4d_clear(p4dp);p4dp=p4dp.add(1);vaddr=next;if vaddr==end{break}else{continue;}}kasan_early_clear_pud(p4dp,vaddr,next);p4dp=p4dp.add(1);vaddr=next;if vaddr==end{break;}}}
unsafe fn kasan_early_clear_pgd(mut pgdp:*mut pgd_t,mut vaddr:usize,end:usize){loop{let next=pgd_addr_end(vaddr,end);if pgtable_l5_enabled&&IS_ALIGNED(vaddr,PGDIR_SIZE)&&next-vaddr>=PGDIR_SIZE{pgd_clear(pgdp);pgdp=pgdp.add(1);vaddr=next;if vaddr==end{break}else{continue;}}kasan_early_clear_p4d(pgdp,vaddr,next);pgdp=pgdp.add(1);vaddr=next;if vaddr==end{break;}}}

unsafe fn kasan_early_populate_pud(mut p4dp:*mut p4d_t,mut vaddr:usize,end:usize){let mut pudp:*mut pud_t;if !pgtable_l4_enabled{pudp=p4dp as *mut pud_t;}else{let b=pt_ops.get_pud_virt(pfn_to_phys(_p4d_pfn(p4dp_get(p4dp))));pudp=b.add(pud_index(vaddr));}loop{let next=pud_addr_end(vaddr,end);if pud_none(pudp_get(pudp))&&IS_ALIGNED(vaddr,PUD_SIZE)&&next-vaddr>=PUD_SIZE{let a=__pa(kasan_early_shadow_pmd as usize);set_pud(pudp,pfn_pud(PFN_DOWN(a),PAGE_TABLE));pudp=pudp.add(1);vaddr=next;if vaddr==end{break}else{continue;}}BUG();}}
unsafe fn kasan_early_populate_p4d(mut pgdp:*mut pgd_t,mut vaddr:usize,end:usize){let mut p4dp:*mut p4d_t;if !pgtable_l5_enabled{p4dp=pgdp as *mut p4d_t;}else{let b=pt_ops.get_p4d_virt(pfn_to_phys(_pgd_pfn(pgdp_get(pgdp))));p4dp=b.add(p4d_index(vaddr));}loop{let next=p4d_addr_end(vaddr,end);if p4d_none(p4dp_get(p4dp))&&IS_ALIGNED(vaddr,P4D_SIZE)&&next-vaddr>=P4D_SIZE{let a=__pa(kasan_early_shadow_pud as usize);set_p4d(p4dp,pfn_p4d(PFN_DOWN(a),PAGE_TABLE));p4dp=p4dp.add(1);vaddr=next;if vaddr==end{break}else{continue;}}kasan_early_populate_pud(p4dp,vaddr,next);p4dp=p4dp.add(1);vaddr=next;if vaddr==end{break;}}}
unsafe fn kasan_early_populate_pgd(mut pgdp:*mut pgd_t,mut vaddr:usize,end:usize){loop{let next=pgd_addr_end(vaddr,end);if pgd_none(pgdp_get(pgdp))&&IS_ALIGNED(vaddr,PGDIR_SIZE)&&next-vaddr>=PGDIR_SIZE{let a=__pa(kasan_early_shadow_p4d as usize);set_pgd(pgdp,pfn_pgd(PFN_DOWN(a),PAGE_TABLE));pgdp=pgdp.add(1);vaddr=next;if vaddr==end{break}else{continue;}}kasan_early_populate_p4d(pgdp,vaddr,next);pgdp=pgdp.add(1);vaddr=next;if vaddr==end{break;}}}

#[no_mangle] pub unsafe extern "C" fn kasan_early_init(){BUILD_BUG_ON(KASAN_SHADOW_OFFSET!=KASAN_SHADOW_END-(1usize<<(64-KASAN_SHADOW_SCALE_SHIFT)));for i in 0..PTRS_PER_PTE{set_pte(kasan_early_shadow_pte.add(i),pfn_pte(virt_to_pfn(kasan_early_shadow_page),PAGE_KERNEL));}for i in 0..PTRS_PER_PMD{set_pmd(kasan_early_shadow_pmd.add(i),pfn_pmd(PFN_DOWN(__pa(kasan_early_shadow_pte as usize)),PAGE_TABLE));}if pgtable_l4_enabled{for i in 0..PTRS_PER_PUD{set_pud(kasan_early_shadow_pud.add(i),pfn_pud(PFN_DOWN(__pa(kasan_early_shadow_pmd as usize)),PAGE_TABLE));}}if pgtable_l5_enabled{for i in 0..PTRS_PER_P4D{set_p4d(kasan_early_shadow_p4d.add(i),pfn_p4d(PFN_DOWN(__pa(kasan_early_shadow_pud as usize)),PAGE_TABLE));}}kasan_early_populate_pgd(early_pg_dir.add(pgd_index(KASAN_SHADOW_START)),KASAN_SHADOW_START,KASAN_SHADOW_END);local_flush_tlb_all();}
pub unsafe extern "C" fn kasan_swapper_init(){kasan_early_populate_pgd(pgd_offset_k(KASAN_SHADOW_START),KASAN_SHADOW_START,KASAN_SHADOW_END);local_flush_tlb_all();}
unsafe fn kasan_populate(start:*mut core::ffi::c_void,end:*mut core::ffi::c_void){let v=(start as usize)&PAGE_MASK;let e=PAGE_ALIGN(end as usize);kasan_populate_pgd(pgd_offset_k(v),v,e);}
unsafe fn kasan_shallow_populate_pud(mut p4d:*mut p4d_t,mut vaddr:usize,end:usize){let mut p=pud_offset(p4d,vaddr);loop{let n=pud_addr_end(vaddr,end);if pud_none(pudp_get(p)){let x=memblock_alloc_or_panic(PAGE_SIZE,PAGE_SIZE);set_pud(p,pfn_pud(PFN_DOWN(__pa(x as usize)),PAGE_TABLE));}else{BUG();}p=p.add(1);vaddr=n;if vaddr==end{break;}}}
unsafe fn kasan_shallow_populate_p4d(mut pgd:*mut pgd_t,mut vaddr:usize,end:usize){let mut p=p4d_offset(pgd,vaddr);loop{let n=p4d_addr_end(vaddr,end);if p4d_none(p4dp_get(p)){let x=memblock_alloc_or_panic(PAGE_SIZE,PAGE_SIZE);set_p4d(p,pfn_p4d(PFN_DOWN(__pa(x as usize)),PAGE_TABLE));}else{kasan_shallow_populate_pud(p,vaddr,end);}p=p.add(1);vaddr=n;if vaddr==end{break;}}}
unsafe fn kasan_shallow_populate_pgd(mut vaddr:usize,end:usize){let mut p=pgd_offset_k(vaddr);loop{let n=pgd_addr_end(vaddr,end);if pgd_none(pgdp_get(p)){let x=memblock_alloc_or_panic(PAGE_SIZE,PAGE_SIZE);set_pgd(p,pfn_pgd(PFN_DOWN(__pa(x as usize)),PAGE_TABLE));}else{kasan_shallow_populate_p4d(p,vaddr,n);}p=p.add(1);vaddr=n;if vaddr==end{break;}}}
unsafe fn kasan_shallow_populate(start:*mut core::ffi::c_void,end:*mut core::ffi::c_void){let v=start as usize&PAGE_MASK;kasan_shallow_populate_pgd(v,PAGE_ALIGN(end as usize));}

#[cfg(feature="CONFIG_KASAN_VMALLOC")] pub unsafe extern "C" fn kasan_populate_early_vm_area_shadow(start:*mut core::ffi::c_void,size:usize){kasan_populate(kasan_mem_to_shadow(start),kasan_mem_to_shadow((start as usize+size) as *mut _));}
unsafe fn create_tmp_mapping(){let ptr;let base:*mut p4d_t;memcpy(TMP_PG_DIR.as_mut_ptr(),swapper_pg_dir as *const _,core::mem::size_of::<pgd_t>()*PTRS_PER_PGD);if pgtable_l5_enabled{ptr=pgd_page_vaddr(pgdp_get(pgd_offset_k(KASAN_SHADOW_END))) as *mut p4d_t;memcpy(TMP_P4D.as_mut_ptr(),ptr,core::mem::size_of::<p4d_t>()*PTRS_PER_P4D);set_pgd(TMP_PG_DIR.as_mut_ptr().add(pgd_index(KASAN_SHADOW_END)),pfn_pgd(PFN_DOWN(__pa(TMP_P4D.as_ptr() as usize)),PAGE_TABLE));base=TMP_P4D.as_mut_ptr();}else{base=TMP_PG_DIR.as_mut_ptr() as *mut p4d_t;}if pgtable_l4_enabled{ptr=p4d_page_vaddr(p4dp_get(base.add(p4d_index(KASAN_SHADOW_END)))) as *mut p4d_t;memcpy(TMP_PUD.as_mut_ptr(),ptr,core::mem::size_of::<pud_t>()*PTRS_PER_PUD);set_p4d(base.add(p4d_index(KASAN_SHADOW_END)),pfn_p4d(PFN_DOWN(__pa(TMP_PUD.as_ptr() as usize)),PAGE_TABLE));}}

pub unsafe extern "C" fn kasan_init(){let mut p_start=0;let mut p_end=0;let mut i=0u64;create_tmp_mapping();csr_write(CSR_SATP,PFN_DOWN(__pa(TMP_PG_DIR.as_ptr() as usize))|satp_mode);kasan_early_clear_pgd(pgd_offset_k(KASAN_SHADOW_START),KASAN_SHADOW_START,KASAN_SHADOW_END);kasan_populate_early_shadow(kasan_mem_to_shadow(FIXADDR_START as *mut _),kasan_mem_to_shadow(VMALLOC_START as *mut _));if IS_ENABLED(CONFIG_KASAN_VMALLOC){kasan_shallow_populate(kasan_mem_to_shadow(VMALLOC_START as *mut _),kasan_mem_to_shadow(VMALLOC_END as *mut _));kasan_shallow_populate(kasan_mem_to_shadow(MODULES_VADDR as *mut _),kasan_mem_to_shadow(MODULES_END as *mut _));}else{kasan_populate_early_shadow(kasan_mem_to_shadow(VMALLOC_START as *mut _),kasan_mem_to_shadow(VMALLOC_END as *mut _));}for_each_mem_range(&mut i,&mut p_start,&mut p_end){kasan_populate(kasan_mem_to_shadow(__va(p_start)),kasan_mem_to_shadow(__va(p_end)));}kasan_populate(kasan_mem_to_shadow(MODULES_END as *const _),kasan_mem_to_shadow((MODULES_VADDR+SZ_2G) as *const _));for i in 0..PTRS_PER_PTE{set_pte(kasan_early_shadow_pte.add(i),mk_pte(virt_to_page(kasan_early_shadow_page),__pgprot(_PAGE_PRESENT|_PAGE_READ|_PAGE_ACCESSED)));}memset(kasan_early_shadow_page,KASAN_SHADOW_INIT,PAGE_SIZE);init_task.kasan_depth=0;csr_write(CSR_SATP,PFN_DOWN(__pa(swapper_pg_dir as usize))|satp_mode);local_flush_tlb_all();kasan_init_generic();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
