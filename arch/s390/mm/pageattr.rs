// SPDX-License-Identifier: GPL-2.0
/* Copyright IBM Corp. 2011; Author(s): Jan Glauber <jang@linux.vnet.ibm.com> */

pub unsafe fn __storage_key_init_range(mut start: c_ulong, end: c_ulong) {
    let (mut boundary, mut size): (c_ulong, c_ulong);
    while start < end {
        if cpu_has_edat1() {
            // set storage keys for a 1MB frame
            size = 1usize.wrapping_shl(20) as c_ulong;
            boundary = (start.wrapping_add(size)) & !(size.wrapping_sub(1));
            if boundary <= end {
                loop {
                    start = sske_frame(start, PAGE_DEFAULT_KEY);
                    if start >= boundary { break; }
                }
                continue;
            }
        }
        page_set_storage_key(start, PAGE_DEFAULT_KEY, 1);
        start = start.wrapping_add(PAGE_SIZE);
    }
}

#[cfg(CONFIG_PROC_FS)]
pub static mut direct_pages_count: [atomic_long_t; PG_DIRECT_MAP_MAX] =
    [/* __bootdata_preserved */ unsafe { core::mem::zeroed() }; PG_DIRECT_MAP_MAX];

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn arch_report_meminfo(m: *mut seq_file) {
    seq_printf(m, "DirectMap4k:    %8lu kB\n", atomic_long_read(&direct_pages_count[PG_DIRECT_MAP_4K]) << 2);
    seq_printf(m, "DirectMap1M:    %8lu kB\n", atomic_long_read(&direct_pages_count[PG_DIRECT_MAP_1M]) << 10);
    seq_printf(m, "DirectMap2G:    %8lu kB\n", atomic_long_read(&direct_pages_count[PG_DIRECT_MAP_2G]) << 21);
}

unsafe fn pgt_set(old: *mut c_ulong, new: c_ulong, addr: c_ulong, dtt: c_ulong) {
    let mask: c_ulong = if cpu_has_edat2() {
        match dtt {
            CRDTE_DTT_REGION3 => !(PTRS_PER_PUD * core::mem::size_of::<pud_t>() - 1) as c_ulong,
            CRDTE_DTT_SEGMENT => !(PTRS_PER_PMD * core::mem::size_of::<pmd_t>() - 1) as c_ulong,
            CRDTE_DTT_PAGE => !(PTRS_PER_PTE * core::mem::size_of::<pte_t>() - 1) as c_ulong,
            _ => 0,
        }
    } else { 0 };
    if cpu_has_edat2() {
        let table = ((old as c_ulong) & mask) as *mut c_ulong;
        crdte(*old, new, table, dtt, addr, (*get_lowcore()).kernel_asce.val);
    } else {
        cspg(old, *old, new);
    }
}

unsafe fn walk_pte_level(pmdp: *mut pmd_t, mut addr: c_ulong, end: c_ulong, flags: c_ulong) -> c_int {
    if flags == SET_MEMORY_4K { return 0; }
    let mut ptep = pte_offset_kernel(pmdp, addr);
    loop {
        let mut new = ptep_get(ptep);
        if pte_none(new) { return -EINVAL; }
        if flags & SET_MEMORY_RO != 0 { new = pte_wrprotect(new); }
        else if flags & SET_MEMORY_RW != 0 { new = pte_mkwrite_novma(pte_mkdirty(new)); }
        if flags & SET_MEMORY_NX != 0 { new = set_pte_bit(new, __pgprot(_PAGE_NOEXEC)); }
        else if flags & SET_MEMORY_X != 0 { new = clear_pte_bit(new, __pgprot(_PAGE_NOEXEC)); }
        if flags & SET_MEMORY_INV != 0 { new = set_pte_bit(new, __pgprot(_PAGE_INVALID)); }
        else if flags & SET_MEMORY_DEF != 0 { new = set_pte_bit(__pte(pte_val(new) & PAGE_MASK), PAGE_KERNEL); }
        pgt_set(ptep as *mut c_ulong, pte_val(new), addr, CRDTE_DTT_PAGE);
        ptep = ptep.add(1); addr = addr.wrapping_add(PAGE_SIZE);
        if addr >= end { break; }
    }
    0
}

unsafe fn split_pmd_page(pmdp: *mut pmd_t, addr: c_ulong) -> c_int {
    let pt_dir = vmem_pte_alloc(); if pt_dir.is_null() { return -ENOMEM; }
    let pmd = pmdp_get(pmdp); let mut pte_addr = pmd_pfn(pmd) << PAGE_SHIFT;
    let ro = (pmd_val(pmd) & _SEGMENT_ENTRY_PROTECT) != 0;
    let nx = (pmd_val(pmd) & _SEGMENT_ENTRY_NOEXEC) != 0;
    let mut prot = pgprot_val(if ro { PAGE_KERNEL_RO } else { PAGE_KERNEL });
    if !nx { prot &= !_PAGE_NOEXEC; }
    let mut ptep = pt_dir;
    for _ in 0..PTRS_PER_PTE { set_pte(ptep, __pte(pte_addr | prot)); pte_addr += PAGE_SIZE; ptep = ptep.add(1); }
    let new = __pmd(__pa(pt_dir) | _SEGMENT_ENTRY);
    pgt_set(pmdp as *mut c_ulong, pmd_val(new), addr, CRDTE_DTT_SEGMENT);
    update_page_count(PG_DIRECT_MAP_4K, PTRS_PER_PTE); update_page_count(PG_DIRECT_MAP_1M, -1); 0
}

unsafe fn modify_pmd_page(pmdp: *mut pmd_t, addr: c_ulong, flags: c_ulong) {
    let mut new = pmdp_get(pmdp);
    if flags & SET_MEMORY_RO != 0 { new = pmd_wrprotect(new); } else if flags & SET_MEMORY_RW != 0 { new = pmd_mkwrite_novma(pmd_mkdirty(new)); }
    if flags & SET_MEMORY_NX != 0 { new = set_pmd_bit(new, __pgprot(_SEGMENT_ENTRY_NOEXEC)); } else if flags & SET_MEMORY_X != 0 { new = clear_pmd_bit(new, __pgprot(_SEGMENT_ENTRY_NOEXEC)); }
    if flags & SET_MEMORY_INV != 0 { new = set_pmd_bit(new, __pgprot(_SEGMENT_ENTRY_INVALID)); } else if flags & SET_MEMORY_DEF != 0 { new = set_pmd_bit(__pmd(pmd_val(new) & PMD_MASK), SEGMENT_KERNEL); }
    pgt_set(pmdp as *mut c_ulong, pmd_val(new), addr, CRDTE_DTT_SEGMENT);
}

unsafe fn walk_pmd_level(pudp: *mut pud_t, mut addr: c_ulong, end: c_ulong, flags: c_ulong) -> c_int {
    let mut pmdp = pmd_offset(pudp, addr); let mut rc = 0;
    loop { let pmd = pmdp_get(pmdp); if pmd_none(pmd) { return -EINVAL; } let next = pmd_addr_end(addr,end);
        if pmd_leaf(pmd) { let split = flags & SET_MEMORY_4K != 0 || addr & !PMD_MASK != 0 || addr + PMD_SIZE > next; if split { rc=split_pmd_page(pmdp,addr); if rc!=0{return rc;} continue; } modify_pmd_page(pmdp,addr,flags); }
        else { rc=walk_pte_level(pmdp,addr,next,flags); if rc!=0{return rc;} }
        pmdp=pmdp.add(1); addr=next; if addr>=end {break;}
    } rc
}

pub unsafe fn split_pud_page(pudp: *mut pud_t, addr: c_ulong) -> c_int {
    let pm_dir=vmem_crst_alloc(_SEGMENT_ENTRY_EMPTY); if pm_dir.is_null(){return -ENOMEM;}
    let pud=pudp_get(pudp); let mut pmd_addr=pud_pfn(pud)<<PAGE_SHIFT; let ro=pud_val(pud)&_REGION_ENTRY_PROTECT!=0; let nx=pud_val(pud)&_REGION_ENTRY_NOEXEC!=0; let mut prot=pgprot_val(if ro{SEGMENT_KERNEL_RO}else{SEGMENT_KERNEL}); if !nx{prot &= !_SEGMENT_ENTRY_NOEXEC;}
    let mut pmdp=pm_dir; for _ in 0..PTRS_PER_PMD {set_pmd(pmdp,__pmd(pmd_addr|prot));pmd_addr+=PMD_SIZE;pmdp=pmdp.add(1);}
    let new=__pud(__pa(pm_dir)|_REGION3_ENTRY);pgt_set(pudp as *mut c_ulong,pud_val(new),addr,CRDTE_DTT_REGION3);update_page_count(PG_DIRECT_MAP_1M,PTRS_PER_PMD);update_page_count(PG_DIRECT_MAP_2G,-1);0
}

unsafe fn modify_pud_page(pudp:*mut pud_t,addr:c_ulong,flags:c_ulong){let mut new=pudp_get(pudp);if flags&SET_MEMORY_RO!=0{new=pud_wrprotect(new)}else if flags&SET_MEMORY_RW!=0{new=pud_mkwrite(pud_mkdirty(new));}if flags&SET_MEMORY_NX!=0{new=set_pud_bit(new,__pgprot(_REGION_ENTRY_NOEXEC))}else if flags&SET_MEMORY_X!=0{new=clear_pud_bit(new,__pgprot(_REGION_ENTRY_NOEXEC));}if flags&SET_MEMORY_INV!=0{new=set_pud_bit(new,__pgprot(_REGION_ENTRY_INVALID))}else if flags&SET_MEMORY_DEF!=0{new=set_pud_bit(__pud(pud_val(new)&PUD_MASK),REGION3_KERNEL);}pgt_set(pudp as *mut c_ulong,pud_val(new),addr,CRDTE_DTT_REGION3);}

unsafe fn walk_pud_level(p4d:*mut p4d_t,mut addr:c_ulong,end:c_ulong,flags:c_ulong)->c_int{let mut pudp=pud_offset(p4d,addr);let mut rc=0;loop{let pud=pudp_get(pudp);if pud_none(pud){return -EINVAL;}let next=pud_addr_end(addr,end);if pud_leaf(pud){let split=flags&SET_MEMORY_4K!=0||addr&!PUD_MASK!=0||addr+PUD_SIZE>next;if split{rc=split_pud_page(pudp,addr);if rc!=0{break;}continue;}modify_pud_page(pudp,addr,flags);}else{rc=walk_pmd_level(pudp,addr,next,flags);}pudp=pudp.add(1);addr=next;if addr>=end||rc!=0{break;}}rc}
unsafe fn walk_p4d_level(pgd:*mut pgd_t,mut addr:c_ulong,end:c_ulong,flags:c_ulong)->c_int{let mut p4dp=p4d_offset(pgd,addr);let mut rc=0;loop{if p4d_none(p4dp_get(p4dp)){return -EINVAL;}let next=p4d_addr_end(addr,end);rc=walk_pud_level(p4dp,addr,next,flags);p4dp=p4dp.add(1);addr=next;if addr>=end||rc!=0{break;}}rc}

DEFINE_MUTEX!(cpa_mutex);
unsafe fn change_page_attr(mut addr:c_ulong,end:c_ulong,flags:c_ulong)->c_int{let mut rc=-EINVAL;let mut pgdp=pgd_offset_k(addr);loop{if pgd_none(pgdp_get(pgdp)){break;}let next=pgd_addr_end(addr,end);rc=walk_p4d_level(pgdp,addr,next,flags);if rc!=0{break;}pgdp=pgdp.add(1);addr=next;if addr>=end{break;}}rc}
unsafe fn change_page_attr_alias(mut addr:c_ulong,end:c_ulong,mut flags:c_ulong)->c_int{let mut area: *mut vm_struct=core::ptr::null_mut();let mut rc=0;flags&=SET_MEMORY_RO|SET_MEMORY_RW;if flags==0{return 0;}while addr<end{if area.is_null(){area=find_vm_area(addr as *mut c_void);}if area.is_null()||(*area).flags&VM_ALLOC==0{return 0;}let va_start=(*area).addr as c_ulong;let va_end=va_start+(*area).nr_pages*PAGE_SIZE;let offset=(addr-va_start)>>PAGE_SHIFT;let alias=page_address(*(*area).pages.add(offset as usize)) as c_ulong;rc=change_page_attr(alias,alias+PAGE_SIZE,flags);if rc!=0{break;}addr+=PAGE_SIZE;if addr>=va_end{area=core::ptr::null_mut();}}rc}
pub unsafe fn __set_memory(mut addr:c_ulong,numpages:c_ulong,mut flags:c_ulong)->c_int{if !cpu_has_nx(){flags&=!(SET_MEMORY_NX|SET_MEMORY_X);}if flags==0||numpages==0{return 0;}addr&=PAGE_MASK;let end=addr+numpages*PAGE_SIZE;mutex_lock(&cpa_mutex);let mut rc=change_page_attr(addr,end,flags);if rc==0{rc=change_page_attr_alias(addr,end,flags);}mutex_unlock(&cpa_mutex);rc}
pub unsafe fn set_direct_map_invalid_noflush(page:*mut page)->c_int{__set_memory(page_to_virt(page) as c_ulong,1,SET_MEMORY_INV)}
pub unsafe fn set_direct_map_default_noflush(page:*mut page)->c_int{__set_memory(page_to_virt(page) as c_ulong,1,SET_MEMORY_DEF)}
pub unsafe fn set_direct_map_valid_noflush(page:*mut page,nr:c_uint,valid:bool)->c_int{__set_memory(page_to_virt(page) as c_ulong,nr as c_ulong,if valid{SET_MEMORY_DEF}else{SET_MEMORY_INV})}
pub unsafe fn kernel_page_present(page:*mut page)->bool{let addr=page_address(page) as c_ulong;let cc:c_uint=0;let _=addr;let _=cc;CC_TRANSFORM(cc)==0}

#[cfg(any(CONFIG_DEBUG_PAGEALLOC,CONFIG_KFENCE))]
unsafe fn ipte_range(mut pte:*mut pte_t,mut address:c_ulong,nr:c_int){if test_facility(13){__ptep_ipte_range(address,nr-1,pte,IPTE_GLOBAL);return;}for _ in 0..nr{__ptep_ipte(address,pte,0,0,IPTE_GLOBAL);address+=PAGE_SIZE;pte=pte.add(1);}}
#[cfg(any(CONFIG_DEBUG_PAGEALLOC,CONFIG_KFENCE))]
pub unsafe fn __kernel_map_pages(page:*mut page,numpages:c_int,enable:c_int){let mut i=0;while i<numpages{let address=page_to_virt(page.add(i as usize)) as c_ulong;let mut ptep=virt_to_kpte(address);let mut nr=(ptep as c_ulong)>>ilog2(core::mem::size_of::<c_ulong>());nr=PTRS_PER_PTE-(nr&(PTRS_PER_PTE-1));nr=core::cmp::min((numpages-i) as usize,nr as usize);if enable!=0{for _ in 0..nr{let pte=clear_pte_bit(ptep_get(ptep),__pgprot(_PAGE_INVALID));set_pte(ptep,pte);ptep=ptep.add(1);}}else{ipte_range(ptep,address,nr as c_int);}i+=nr as c_int;}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
