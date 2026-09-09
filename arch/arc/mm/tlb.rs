// SPDX-License-Identifier: GPL-2.0-only
/* TLB Management (flush/create/diagnostics) for MMUv3 and MMUv4 */

/* External kernel symbols and types are supplied by the surrounding ARC port. */

#[repr(C)]
struct CpuinfoArcMmu {
    ver: u32,
    pg_sz_k: u32,
    s_pg_sz_m: u32,
    pae: u32,
    sets: u32,
    ways: u32,
}

static mut MMUINFO: CpuinfoArcMmu = CpuinfoArcMmu { ver: 0, pg_sz_k: 0, s_pg_sz_m: 0, pae: 0, sets: 0, ways: 0 };

#[inline]
unsafe fn __tlb_entry_erase() {
    write_aux_reg(ARC_REG_TLBPD1, 0);
    if is_pae40_enabled() { write_aux_reg(ARC_REG_TLBPD1HI, 0); }
    write_aux_reg(ARC_REG_TLBPD0, 0);
    write_aux_reg(ARC_REG_TLBCOMMAND, TLBWrite);
}

unsafe fn utlb_invalidate() { write_aux_reg(ARC_REG_TLBCOMMAND, TLBIVUTLB); }

#[cfg(CONFIG_ARC_MMU_V3)]
unsafe fn tlb_entry_lkup(vaddr_n_asid: usize) -> u32 {
    write_aux_reg(ARC_REG_TLBPD0, vaddr_n_asid as _);
    write_aux_reg(ARC_REG_TLBCOMMAND, TLBProbe);
    read_aux_reg(ARC_REG_TLBINDEX)
}

#[cfg(CONFIG_ARC_MMU_V3)]
unsafe fn tlb_entry_erase(vaddr_n_asid: u32) {
    let idx = tlb_entry_lkup(vaddr_n_asid as _) as u32;
    if likely(!(idx & TLB_LKUP_ERR) != 0) { __tlb_entry_erase(); }
    else { WARN(idx == TLB_DUP_ERR, "Probe returned Dup PD for %x\n", vaddr_n_asid); }
}

#[cfg(CONFIG_ARC_MMU_V3)]
unsafe fn tlb_entry_insert(pd0: u32, pd1: usize) {
    let idx = tlb_entry_lkup(pd0 as _);
    if likely((idx & TLB_LKUP_ERR) != 0) { write_aux_reg(ARC_REG_TLBCOMMAND, TLBGetIndex); }
    write_aux_reg(ARC_REG_TLBPD1, pd1 as _);
    write_aux_reg(ARC_REG_TLBCOMMAND, TLBWrite);
}

#[cfg(not(CONFIG_ARC_MMU_V3))]
unsafe fn tlb_entry_erase(vaddr_n_asid: u32) {
    write_aux_reg(ARC_REG_TLBPD0, vaddr_n_asid | _PAGE_PRESENT);
    write_aux_reg(ARC_REG_TLBCOMMAND, TLBDeleteEntry);
}

#[cfg(not(CONFIG_ARC_MMU_V3))]
unsafe fn tlb_entry_insert(pd0: u32, pd1: usize) {
    write_aux_reg(ARC_REG_TLBPD0, pd0);
    if !is_pae40_enabled() { write_aux_reg(ARC_REG_TLBPD1, pd1 as _); }
    else {
        write_aux_reg(ARC_REG_TLBPD1, (pd1 & 0xffff_ffff) as _);
        write_aux_reg(ARC_REG_TLBPD1HI, (pd1 >> 32) as _);
    }
    write_aux_reg(ARC_REG_TLBCOMMAND, TLBInsertEntry);
}

#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_all() {
    let mmu = &MMUINFO;
    let mut flags = 0usize;
    let num_tlb = mmu.sets * mmu.ways;
    local_irq_save(&mut flags);
    write_aux_reg(ARC_REG_TLBPD1, 0);
    if is_pae40_enabled() { write_aux_reg(ARC_REG_TLBPD1HI, 0); }
    write_aux_reg(ARC_REG_TLBPD0, 0);
    let mut entry = 0;
    while entry < num_tlb { write_aux_reg(ARC_REG_TLBINDEX, entry); write_aux_reg(ARC_REG_TLBCOMMAND, TLBWriteNI); entry += 1; }
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)] {
        write_aux_reg(ARC_REG_TLBPD0, _PAGE_HW_SZ);
        entry = 0x800;
        while entry < 0x810 { write_aux_reg(ARC_REG_TLBINDEX, entry); write_aux_reg(ARC_REG_TLBCOMMAND, TLBWriteNI); entry += 1; }
    }
    utlb_invalidate();
    local_irq_restore(flags);
}

#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_mm(mm: *mut mm_struct) {
    if atomic_read(&(*mm).mm_users) == 0 { return; }
    destroy_context(mm);
    if (*current).mm == mm { get_new_mmu_context(mm); }
}

#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_range(vma: *mut vm_area_struct, mut start: usize, end: usize) {
    let cpu = smp_processor_id();
    if end.wrapping_sub(start) >= PAGE_SIZE * 32 { local_flush_tlb_mm((*vma).vm_mm); return; }
    start &= PAGE_MASK;
    let mut flags = 0usize; local_irq_save(&mut flags);
    if asid_mm((*vma).vm_mm, cpu) != MM_CTXT_NO_ASID {
        while start < end { tlb_entry_erase((start | hw_pid((*vma).vm_mm, cpu) as usize) as _); start += PAGE_SIZE; }
    }
    local_irq_restore(flags);
}

#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_kernel_range(mut start: usize, end: usize) {
    if end.wrapping_sub(start) >= PAGE_SIZE * 32 { local_flush_tlb_all(); return; }
    start &= PAGE_MASK; let mut flags = 0usize; local_irq_save(&mut flags);
    while start < end { tlb_entry_erase(start as _); start += PAGE_SIZE; }
    local_irq_restore(flags);
}

#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_page(vma: *mut vm_area_struct, page: usize) {
    let cpu = smp_processor_id(); let mut flags = 0usize; local_irq_save(&mut flags);
    if asid_mm((*vma).vm_mm, cpu) != MM_CTXT_NO_ASID { tlb_entry_erase(((page & PAGE_MASK) | hw_pid((*vma).vm_mm, cpu) as usize) as _); }
    local_irq_restore(flags);
}

#[cfg(CONFIG_SMP)]
#[repr(C)] struct TlbArgs { ta_vma: *mut vm_area_struct, ta_start: usize, ta_end: usize }

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn ipi_flush_tlb_page(arg: *mut core::ffi::c_void) { let ta = &*(arg as *mut TlbArgs); local_flush_tlb_page(ta.ta_vma, ta.ta_start); }
#[cfg(CONFIG_SMP)]
unsafe extern "C" fn ipi_flush_tlb_range(arg: *mut core::ffi::c_void) { let ta = &*(arg as *mut TlbArgs); local_flush_tlb_range(ta.ta_vma, ta.ta_start, ta.ta_end); }
#[cfg(all(CONFIG_SMP, CONFIG_TRANSPARENT_HUGEPAGE))]
unsafe extern "C" fn ipi_flush_pmd_tlb_range(arg: *mut core::ffi::c_void) { let ta = &*(arg as *mut TlbArgs); local_flush_pmd_tlb_range(ta.ta_vma, ta.ta_start, ta.ta_end); }
#[cfg(CONFIG_SMP)]
unsafe extern "C" fn ipi_flush_tlb_kernel_range(arg: *mut core::ffi::c_void) { let ta = &*(arg as *mut TlbArgs); local_flush_tlb_kernel_range(ta.ta_start, ta.ta_end); }

#[cfg(CONFIG_SMP)]
pub unsafe extern "C" fn flush_tlb_all() { on_each_cpu(local_flush_tlb_all, core::ptr::null_mut(), 1); }
#[cfg(CONFIG_SMP)]
pub unsafe extern "C" fn flush_tlb_mm(mm: *mut mm_struct) { on_each_cpu_mask(mm_cpumask(mm), local_flush_tlb_mm, mm as _, 1); }
#[cfg(CONFIG_SMP)]
pub unsafe extern "C" fn flush_tlb_page(vma: *mut vm_area_struct, uaddr: usize) { let ta = TlbArgs { ta_vma:vma, ta_start:uaddr, ta_end:0 }; on_each_cpu_mask(mm_cpumask((*vma).vm_mm), ipi_flush_tlb_page, &ta as *const _ as _, 1); }
#[cfg(CONFIG_SMP)]
pub unsafe extern "C" fn flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize) { let ta = TlbArgs { ta_vma:vma, ta_start:start, ta_end:end }; on_each_cpu_mask(mm_cpumask((*vma).vm_mm), ipi_flush_tlb_range, &ta as *const _ as _, 1); }
#[cfg(all(CONFIG_SMP, CONFIG_TRANSPARENT_HUGEPAGE))]
pub unsafe extern "C" fn flush_pmd_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize) { let ta = TlbArgs { ta_vma:vma, ta_start:start, ta_end:end }; on_each_cpu_mask(mm_cpumask((*vma).vm_mm), ipi_flush_pmd_tlb_range, &ta as *const _ as _, 1); }
#[cfg(CONFIG_SMP)]
pub unsafe extern "C" fn flush_tlb_kernel_range(start: usize, end: usize) { let ta = TlbArgs { ta_vma:core::ptr::null_mut(), ta_start:start, ta_end:end }; on_each_cpu(ipi_flush_tlb_kernel_range, &ta as *const _ as _, 1); }

unsafe fn create_tlb(vma: *mut vm_area_struct, mut vaddr: usize, ptep: *mut pte_t) {
    if (*current).active_mm != (*vma).vm_mm { return; }
    let mut flags = 0usize; local_irq_save(&mut flags); vaddr &= PAGE_MASK;
    pte_val(*ptep) |= _PAGE_PRESENT | _PAGE_ACCESSED;
    let asid_or_sasid = read_aux_reg(ARC_REG_PID) & 0xff;
    let pd0 = vaddr as u32 | asid_or_sasid | (pte_val(*ptep) & PTE_BITS_IN_PD0);
    let mut rwx = pte_val(*ptep) & PTE_BITS_RWX;
    if pte_val(*ptep) & _PAGE_GLOBAL != 0 { rwx <<= 3; } else { rwx |= rwx << 3; }
    let pd1 = rwx as usize | (pte_val(*ptep) & PTE_BITS_NON_RWX_IN_PD1) as usize;
    tlb_entry_insert(pd0, pd1); local_irq_restore(flags);
}

pub unsafe extern "C" fn update_mmu_cache_range(vmf: *mut vm_fault, vma: *mut vm_area_struct, vaddr_unaligned: usize, ptep: *mut pte_t, nr: u32) {
    let mut vaddr = vaddr_unaligned & PAGE_MASK;
    let mut paddr = (pte_val(*ptep) & PAGE_MASK_PHYS) as usize;
    let page = pfn_to_page(pte_pfn(*ptep)); create_tlb(vma, vaddr, ptep);
    if page == ZERO_PAGE(0) { return; }
    if (*vma).vm_flags & VM_EXEC != 0 {
        let folio = page_folio(page); let dirty = !test_and_set_bit(PG_dc_clean, &mut (*folio).flags.f);
        if dirty { let offset = offset_in_folio(folio, paddr); let nr = folio_nr_pages(folio); paddr -= offset; vaddr -= offset; __flush_dcache_pages(paddr, paddr, nr); if (*vma).vm_flags & VM_EXEC != 0 { __inv_icache_pages(paddr, vaddr, nr); } }
    }
}

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe extern "C" fn update_mmu_cache_pmd(vma: *mut vm_area_struct, addr: usize, pmd: *mut pmd_t) { let mut pte = __pte(pmd_val(*pmd)); update_mmu_cache_range(core::ptr::null_mut(), vma, addr, &mut pte, HPAGE_PMD_NR); }
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
pub unsafe extern "C" fn local_flush_pmd_tlb_range(vma: *mut vm_area_struct, start: usize, _end: usize) { let mut flags=0usize; local_irq_save(&mut flags); let cpu=smp_processor_id(); if likely(asid_mm((*vma).vm_mm,cpu) != MM_CTXT_NO_ASID) { tlb_entry_erase((start | _PAGE_HW_SZ as usize | hw_pid((*vma).vm_mm,cpu) as usize) as _); } local_irq_restore(flags); }

pub unsafe extern "C" fn pae40_exist_but_not_enab() -> i32 { (MMUINFO.pae != 0 && !is_pae40_enabled()) as i32 }

pub unsafe extern "C" fn arc_mmu_mumbojumbo(c: i32, buf: *mut i8, len: i32) -> i32 {
    let mmu=&mut MMUINFO; let bcr=read_aux_reg(ARC_REG_MMU_BCR); mmu.ver=bcr>>24;
    let (u_dtlb,u_itlb,sasid) = if is_isa_arcompact() && mmu.ver==3 {
        mmu.pg_sz_k=1 << (((bcr>>0)&7)-1); mmu.sets=1 << ((bcr>>3)&7); mmu.ways=1 << ((bcr>>6)&7);
        (((bcr>>9)&0xf),((bcr>>13)&0xf),((bcr>>17)&1))
    } else {
        mmu.pg_sz_k=1 << (((bcr>>0)&7)-1); mmu.s_pg_sz_m=1 << (((bcr>>3)&7)-11); mmu.sets=64 << ((bcr>>6)&7); mmu.ways=((bcr>>9)&7)*2; mmu.pae=(bcr>>20)&1;
        (((bcr>>12)&0xf)*4,((bcr>>16)&0xf)*4,((bcr>>19)&1))
    };
    let mut super_pg=[0i8;64]; if mmu.s_pg_sz_m!=0 { scnprintf(super_pg.as_mut_ptr(),64,b"/%dM%s\0".as_ptr() as _,mmu.s_pg_sz_m,if IS_ENABLED(CONFIG_TRANSPARENT_HUGEPAGE) { b" (THP enabled)\0".as_ptr() } else { b"\0".as_ptr() }); }
    scnprintf(buf,len,b"MMU [v%x]\t: %dk%s, swalk %d lvl, JTLB %dx%d, uDTLB %d, uITLB %d%s%s%s\n\0".as_ptr() as _,mmu.ver,mmu.pg_sz_k,super_pg.as_ptr(),CONFIG_PGTABLE_LEVELS,mmu.sets,mmu.ways,u_dtlb,u_itlb,IS_AVAIL1(sasid,b", SASID\0".as_ptr() as _),IS_AVAIL2(mmu.pae,b", PAE40 \0".as_ptr() as _,CONFIG_ARC_HAS_PAE40))
}

pub unsafe extern "C" fn arc_mmu_init() {
    let mmu=&MMUINFO; let compat=(is_isa_arcompact() && mmu.ver==3)||(is_isa_arcv2()&&mmu.ver>=4); if !compat { panic!("MMU ver %d doesn't match kernel built for\n",mmu.ver); }
    if mmu.pg_sz_k != TO_KB(PAGE_SIZE) { panic!("MMU pg size != PAGE_SIZE (%luk)\n",TO_KB(PAGE_SIZE)); }
    if IS_ENABLED(CONFIG_TRANSPARENT_HUGEPAGE) && mmu.s_pg_sz_m != TO_MB(HPAGE_PMD_SIZE) { panic!("MMU Super pg size != Linux HPAGE_PMD_SIZE (%luM)\n",TO_MB(HPAGE_PMD_SIZE)); }
    if IS_ENABLED(CONFIG_ARC_HAS_PAE40) && mmu.pae==0 { panic!("Hardware doesn't support PAE40\n"); }
    mmu_setup_asid(core::ptr::null_mut(),0); mmu_setup_pgd(core::ptr::null_mut(),swapper_pg_dir); if pae40_exist_but_not_enab()!=0 { write_aux_reg(ARC_REG_TLBPD1HI,0); }
}

#[allow(dead_code)]
pub static mut dup_pd_silent: i32 = 0;

pub unsafe extern "C" fn do_tlb_overlap_fault(_cause: usize, _address: usize, _regs: *mut pt_regs) {
    let mmu=&MMUINFO; let mut flags=0usize; let n_ways=core::cmp::min(mmu.ways,4); BUG_ON(mmu.ways>4); local_irq_save(&mut flags);
    let mut set=0; while set<mmu.sets { let mut pd0=[0u32;4]; let mut is_valid=0u32; let mut way=0; while way<n_ways { write_aux_reg(ARC_REG_TLBINDEX,set*mmu.ways+way); write_aux_reg(ARC_REG_TLBCOMMAND,TLBRead); pd0[way as usize]=read_aux_reg(ARC_REG_TLBPD0); is_valid |= pd0[way as usize]&_PAGE_PRESENT; pd0[way as usize]&=PAGE_MASK; way+=1; } if is_valid==0 { set+=1; continue; } way=0; while way+1<n_ways { if pd0[way as usize]!=0 { let mut n=way+1; while n<n_ways { if pd0[way as usize]==pd0[n as usize] { if dup_pd_silent==0 { pr_info("Dup TLB PD0 %08x @ set %d ways %d,%d\n",pd0[way as usize],set,way,n); } pd0[way as usize]=0; write_aux_reg(ARC_REG_TLBINDEX,set*mmu.ways+way); __tlb_entry_erase(); } n+=1; } } way+=1; } set+=1; }
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
