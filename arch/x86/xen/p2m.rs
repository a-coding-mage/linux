// SPDX-License-Identifier: GPL-2.0
//
// Direct Rust translation of the Xen p2m implementation. Kernel and Xen
// symbols referenced below are supplied by external translation units.

const P2M_MID_PER_PAGE: usize = PAGE_SIZE / core::mem::size_of::<usize>();
const P2M_TOP_PER_PAGE: usize = PAGE_SIZE / core::mem::size_of::<usize>();
const MAX_P2M_PFN: usize = P2M_TOP_PER_PAGE * P2M_MID_PER_PAGE * P2M_PER_PAGE;
const PMDS_PER_MID_PAGE: usize = P2M_MID_PER_PAGE / PTRS_PER_PTE;

#[no_mangle] pub static mut xen_p2m_addr: *mut usize = core::ptr::null_mut();
#[no_mangle] pub static mut xen_p2m_size: usize = 0;
#[no_mangle] pub static mut xen_max_p2m_pfn: usize = 0;

#[cfg(CONFIG_XEN_MEMORY_HOTPLUG_LIMIT)] const P2M_LIMIT: usize = CONFIG_XEN_MEMORY_HOTPLUG_LIMIT;
#[cfg(not(CONFIG_XEN_MEMORY_HOTPLUG_LIMIT))] const P2M_LIMIT: usize = 0;

static mut p2m_update_lock: SpinLock = SpinLock::new();
static mut p2m_mid_missing_mfn: *mut usize = core::ptr::null_mut();
static mut p2m_top_mfn: *mut usize = core::ptr::null_mut();
static mut p2m_top_mfn_p: *mut *mut usize = core::ptr::null_mut();
static mut p2m_missing: *mut usize = core::ptr::null_mut();
static mut p2m_identity: *mut usize = core::ptr::null_mut();
static mut p2m_missing_pte: *mut pte_t = core::ptr::null_mut();
static mut p2m_identity_pte: *mut pte_t = core::ptr::null_mut();
static mut xen_p2m_last_pfn: usize = 0;

#[inline] unsafe fn p2m_top_index(pfn: usize) -> usize { BUG_ON(pfn >= MAX_P2M_PFN); pfn / (P2M_MID_PER_PAGE * P2M_PER_PAGE) }
#[inline] unsafe fn p2m_mid_index(pfn: usize) -> usize { (pfn / P2M_PER_PAGE) % P2M_MID_PER_PAGE }

unsafe fn p2m_top_mfn_init(top: *mut usize) { for i in 0..P2M_TOP_PER_PAGE { *top.add(i) = virt_to_mfn(p2m_mid_missing_mfn); } }
unsafe fn p2m_top_mfn_p_init(top: *mut *mut usize) { for i in 0..P2M_TOP_PER_PAGE { *top.add(i) = p2m_mid_missing_mfn; } }
unsafe fn p2m_mid_mfn_init(mid: *mut usize, leaf: *mut usize) { for i in 0..P2M_MID_PER_PAGE { *mid.add(i) = virt_to_mfn(leaf); } }
unsafe fn p2m_init(p2m: *mut usize) { for i in 0..P2M_PER_PAGE { *p2m.add(i) = INVALID_P2M_ENTRY; } }
unsafe fn p2m_init_identity(p2m: *mut usize, pfn: usize) { for i in 0..P2M_PER_PAGE { *p2m.add(i) = IDENTITY_FRAME(pfn + i); } }

unsafe fn alloc_p2m_page() -> *mut core::ffi::c_void { if !slab_is_available() { memblock_alloc_or_panic(PAGE_SIZE, PAGE_SIZE) } else { __get_free_page(GFP_KERNEL) as *mut _ } }
unsafe fn free_p2m_page(p: *mut core::ffi::c_void) { if !slab_is_available() { memblock_free(p, PAGE_SIZE); } else { free_page(p as usize); } }

pub unsafe fn xen_build_mfn_list_list() {
    if (*xen_start_info).flags & SIF_VIRT_P2M_4TOOLS != 0 { return; }
    if p2m_top_mfn.is_null() {
        p2m_mid_missing_mfn = alloc_p2m_page() as *mut usize; p2m_mid_mfn_init(p2m_mid_missing_mfn, p2m_missing);
        p2m_top_mfn_p = alloc_p2m_page() as *mut *mut usize; p2m_top_mfn_p_init(p2m_top_mfn_p);
        p2m_top_mfn = alloc_p2m_page() as *mut usize; p2m_top_mfn_init(p2m_top_mfn);
    } else { p2m_mid_mfn_init(p2m_mid_missing_mfn, p2m_missing); }
    let mut pfn = 0usize;
    while pfn < xen_max_p2m_pfn && pfn < MAX_P2M_PFN {
        let topidx = p2m_top_index(pfn); let mididx = p2m_mid_index(pfn);
        let mut mid_mfn_p = *p2m_top_mfn_p.add(topidx);
        let mut level = 0u32; let mut ptep = lookup_address(xen_p2m_addr.add(pfn) as usize, &mut level);
        BUG_ON(ptep.is_null() || level != PG_LEVEL_4K); let mfn = pte_mfn(*ptep); ptep = (ptep as usize & !(PAGE_SIZE-1)) as *mut pte_t;
        if ptep == p2m_missing_pte || ptep == p2m_identity_pte { BUG_ON(mididx != 0); BUG_ON(mid_mfn_p != p2m_mid_missing_mfn); *p2m_top_mfn.add(topidx)=virt_to_mfn(p2m_mid_missing_mfn); pfn += (P2M_MID_PER_PAGE-1)*P2M_PER_PAGE; pfn += P2M_PER_PAGE; continue; }
        if mid_mfn_p == p2m_mid_missing_mfn { mid_mfn_p=alloc_p2m_page() as *mut usize; p2m_mid_mfn_init(mid_mfn_p,p2m_missing); *p2m_top_mfn_p.add(topidx)=mid_mfn_p; }
        *p2m_top_mfn.add(topidx)=virt_to_mfn(mid_mfn_p); *mid_mfn_p.add(mididx)=mfn; pfn += P2M_PER_PAGE;
    }
}

pub unsafe fn xen_setup_mfn_list_list() { BUG_ON(HYPERVISOR_shared_info == &xen_dummy_shared_info); if (*xen_start_info).flags & SIF_VIRT_P2M_4TOOLS != 0 { (*HYPERVISOR_shared_info).arch.pfn_to_mfn_frame_list_list=usize::MAX; } else { (*HYPERVISOR_shared_info).arch.pfn_to_mfn_frame_list_list=virt_to_mfn(p2m_top_mfn); } (*HYPERVISOR_shared_info).arch.max_pfn=xen_p2m_last_pfn; (*HYPERVISOR_shared_info).arch.p2m_generation=0; (*HYPERVISOR_shared_info).arch.p2m_vaddr=xen_p2m_addr as usize; (*HYPERVISOR_shared_info).arch.p2m_cr3=xen_pfn_to_cr3(virt_to_mfn(swapper_pg_dir)); }

pub unsafe fn xen_build_dynamic_phys_to_machine() { xen_p2m_addr=(*xen_start_info).mfn_list as *mut usize; xen_p2m_size=ALIGN((*xen_start_info).nr_pages,P2M_PER_PAGE); for pfn in (*xen_start_info).nr_pages..xen_p2m_size { *xen_p2m_addr.add(pfn)=INVALID_P2M_ENTRY; } xen_max_p2m_pfn=xen_p2m_size; }

const P2M_TYPE_IDENTITY: i32=0; const P2M_TYPE_MISSING: i32=1; const P2M_TYPE_PFN: i32=2; const P2M_TYPE_UNKNOWN: i32=3;
unsafe fn xen_p2m_elem_type(pfn: usize)->i32 { if pfn>=xen_p2m_size{return P2M_TYPE_IDENTITY;} let mfn=*xen_p2m_addr.add(pfn); if mfn==INVALID_P2M_ENTRY{return P2M_TYPE_MISSING;} if mfn&IDENTITY_FRAME_BIT!=0{return P2M_TYPE_IDENTITY;} P2M_TYPE_PFN }

pub unsafe fn xen_rebuild_p2m_list(p2m: *mut usize) {
    p2m_missing=alloc_p2m_page() as *mut usize; p2m_init(p2m_missing); p2m_identity=alloc_p2m_page() as *mut usize; p2m_init(p2m_identity);
    p2m_missing_pte=alloc_p2m_page() as *mut pte_t; paravirt_alloc_pte(&mut init_mm,__pa(p2m_missing_pte as usize)>>PAGE_SHIFT); p2m_identity_pte=alloc_p2m_page() as *mut pte_t; paravirt_alloc_pte(&mut init_mm,__pa(p2m_identity_pte as usize)>>PAGE_SHIFT);
    for i in 0..PTRS_PER_PTE { set_pte(p2m_missing_pte.add(i),pfn_pte(PFN_DOWN(__pa(p2m_missing as usize)),PAGE_KERNEL_RO)); set_pte(p2m_identity_pte.add(i),pfn_pte(PFN_DOWN(__pa(p2m_identity as usize)),PAGE_KERNEL_RO)); }
    let mut pfn=0usize; while pfn<xen_max_p2m_pfn { let chunk=if pfn&(P2M_PER_PAGE*P2M_MID_PER_PAGE-1)!=0 {P2M_PER_PAGE} else {P2M_PER_PAGE*P2M_MID_PER_PAGE}; let typ=xen_p2m_elem_type(pfn); let mut i=0; if typ!=P2M_TYPE_PFN { i=1; while i<chunk && xen_p2m_elem_type(pfn+i)==typ {i+=1;} } let chunk=if i<chunk {P2M_PER_PAGE}else{chunk}; if typ==P2M_TYPE_PFN || i<chunk { let mfns=alloc_p2m_page() as *mut usize; copy_page(mfns,xen_p2m_addr.add(pfn)); let ptep=populate_extra_pte(p2m.add(pfn) as usize); set_pte(ptep,pfn_pte(PFN_DOWN(__pa(mfns as usize)),PAGE_KERNEL)); } else if chunk==P2M_PER_PAGE { let mfns=if typ==P2M_TYPE_MISSING{p2m_missing}else{p2m_identity}; let ptep=populate_extra_pte(p2m.add(pfn) as usize); set_pte(ptep,pfn_pte(PFN_DOWN(__pa(mfns as usize)),PAGE_KERNEL_RO)); } else { let ptep=if typ==P2M_TYPE_MISSING{p2m_missing_pte}else{p2m_identity_pte}; for j in 0..PMDS_PER_MID_PAGE { let pmdp=populate_extra_pmd(p2m.add(pfn) as usize+j*PMD_SIZE); set_pmd(pmdp,__pmd(__pa(ptep as usize)|_KERNPG_TABLE)); } } pfn+=chunk; }
}

pub unsafe fn xen_vmalloc_p2m_tree() { static mut vm: vm_struct=vm_struct::zeroed(); xen_p2m_last_pfn=xen_max_p2m_pfn; let p2m_limit=(P2M_LIMIT as u64)*1024*1024*1024/PAGE_SIZE as u64; vm.flags=VM_ALLOC; vm.size=ALIGN(core::mem::size_of::<usize>()*core::cmp::max(xen_max_p2m_pfn,p2m_limit as usize),PMD_SIZE*PMDS_PER_MID_PAGE); vm_area_register_early(&mut vm,PMD_SIZE*PMDS_PER_MID_PAGE); pr_notice!("p2m virtual area at %p, size is %lx\n",vm.addr,vm.size); xen_max_p2m_pfn=vm.size/core::mem::size_of::<usize>(); xen_rebuild_p2m_list(vm.addr as *mut usize); xen_p2m_addr=vm.addr as *mut usize; xen_p2m_size=xen_max_p2m_pfn; xen_inv_extra_mem(); }

pub unsafe fn get_phys_to_machine(pfn: usize)->usize { if pfn>=xen_p2m_size {return if pfn<xen_max_p2m_pfn{xen_chk_extra_mem(pfn)}else{IDENTITY_FRAME(pfn)};} let mut level=0; let ptep=lookup_address(xen_p2m_addr.add(pfn) as usize,&mut level); BUG_ON(ptep.is_null()||level!=PG_LEVEL_4K); if pte_pfn(*ptep)==PFN_DOWN(__pa(p2m_identity as usize)){IDENTITY_FRAME(pfn)}else{*xen_p2m_addr.add(pfn)} }

pub unsafe fn __set_phys_to_machine(pfn: usize,mfn: usize)->bool { if pfn>=xen_p2m_size{return mfn==INVALID_P2M_ENTRY;} if xen_safe_write_ulong(xen_p2m_addr.add(pfn),mfn)==0{return true;} let mut level=0; let ptep=lookup_address(xen_p2m_addr.add(pfn) as usize,&mut level); BUG_ON(ptep.is_null()||level!=PG_LEVEL_4K); if pte_pfn(*ptep)==PFN_DOWN(__pa(p2m_missing as usize)){return mfn==INVALID_P2M_ENTRY;} if pte_pfn(*ptep)==PFN_DOWN(__pa(p2m_identity as usize)){return mfn==IDENTITY_FRAME(pfn);} false }
pub unsafe fn set_phys_to_machine(pfn: usize,mfn: usize)->bool { if !__set_phys_to_machine(pfn,mfn) { if xen_alloc_p2m_entry(pfn)<0{return false;} return __set_phys_to_machine(pfn,mfn); } true }

pub unsafe fn xen_alloc_p2m_entry(pfn: usize)->i32 {
    let addr=xen_p2m_addr.add(pfn) as usize; let mut level=0; let mut ptep=lookup_address(addr,&mut level); BUG_ON(ptep.is_null()||level!=PG_LEVEL_4K); let pte_pg=(ptep as usize&!(PAGE_SIZE-1)) as *mut pte_t;
    if pte_pg==p2m_missing_pte||pte_pg==p2m_identity_pte { let mut pages:[*mut pte_t;PMDS_PER_MID_PAGE]=[core::ptr::null_mut();PMDS_PER_MID_PAGE]; for i in 0..PMDS_PER_MID_PAGE { pages[i]=alloc_p2m_page() as *mut pte_t; if pages[i].is_null(){for j in 0..i{free_p2m_page(pages[j] as *mut _);}return -ENOMEM;} copy_page(pages[i] as *mut _,pte_pg as *mut _); paravirt_alloc_pte(&mut init_mm,__pa(pages[i] as usize)>>PAGE_SHIFT); } let mut vaddr=addr&!(PMD_SIZE*PMDS_PER_MID_PAGE-1); for i in 0..PMDS_PER_MID_PAGE { let pmdp=lookup_pmd_address(vaddr); let mut flags=0; spin_lock_irqsave(&mut p2m_update_lock,&mut flags); let chk=lookup_address(vaddr,&mut level); if chk==pte_pg { (*HYPERVISOR_shared_info).arch.p2m_generation+=1; wmb(); set_pmd(pmdp,__pmd(__pa(pages[i] as usize)|_KERNPG_TABLE)); wmb(); (*HYPERVISOR_shared_info).arch.p2m_generation+=1; pages[i]=core::ptr::null_mut(); } spin_unlock_irqrestore(&mut p2m_update_lock,flags); if !pages[i].is_null(){paravirt_release_pte(__pa(pages[i] as usize)>>PAGE_SHIFT);free_p2m_page(pages[i] as *mut _);} vaddr+=PMD_SIZE;} ptep=lookup_address(addr,&mut level); }
    let p2m_pfn=pte_pfn(*ptep); if p2m_pfn==PFN_DOWN(__pa(p2m_identity as usize))||p2m_pfn==PFN_DOWN(__pa(p2m_missing as usize)){let p2m=alloc_p2m_page() as *mut usize;if p2m.is_null(){return -ENOMEM;}if p2m_pfn==PFN_DOWN(__pa(p2m_missing as usize)){p2m_init(p2m);}else{p2m_init_identity(p2m,pfn&!(P2M_PER_PAGE-1));}let mut flags=0;spin_lock_irqsave(&mut p2m_update_lock,&mut flags);if pte_pfn(*ptep)==p2m_pfn{(*HYPERVISOR_shared_info).arch.p2m_generation+=1;wmb();set_pte(ptep,pfn_pte(PFN_DOWN(__pa(p2m as usize)),PAGE_KERNEL));wmb();(*HYPERVISOR_shared_info).arch.p2m_generation+=1;spin_unlock_irqrestore(&mut p2m_update_lock,flags);}else{spin_unlock_irqrestore(&mut p2m_update_lock,flags);free_p2m_page(p2m as *mut _);}}
    if pfn>=xen_p2m_last_pfn{xen_p2m_last_pfn=ALIGN(pfn+1,P2M_PER_PAGE);(*HYPERVISOR_shared_info).arch.max_pfn=xen_p2m_last_pfn;} 0
}
pub unsafe fn clear_foreign_p2m_mapping(_u:*mut gnttab_unmap_grant_ref,_ku:*mut gnttab_unmap_grant_ref,pages:*mut *mut page,count:usize)->i32 { if !xen_pv_domain(){return 0;} let mut ret=0;for i in 0..count{let p=page_to_pfn(*pages.add(i));let m=__pfn_to_mfn(p);if m!=INVALID_P2M_ENTRY&&(m&FOREIGN_FRAME_BIT)!=0{set_phys_to_machine(p,INVALID_P2M_ENTRY);}else{ret=-EINVAL;}}ret }
pub unsafe fn set_foreign_p2m_mapping(_m:*mut gnttab_map_grant_ref,_k:*mut gnttab_map_grant_ref,_p:*mut *mut page,_c:usize)->i32 { if xen_pv_domain(){0}else{0} }
pub unsafe fn set_phys_range_identity(pfn_s: usize,mut pfn_e: usize)->usize { if pfn_s>=xen_p2m_size||pfn_s>pfn_e{return 0;} if pfn_e>xen_p2m_size{pfn_e=xen_p2m_size;} for pfn in pfn_s..pfn_e {*xen_p2m_addr.add(pfn)=IDENTITY_FRAME(pfn);} pfn_e-pfn_s }

#[repr(C)] pub struct nonram_remap { maddr: phys_addr_t, paddr: phys_addr_t, size: usize }
const NR_NONRAM_REMAP: usize=4; static mut xen_nonram_remap:[nonram_remap;NR_NONRAM_REMAP]=[nonram_remap{maddr:0,paddr:0,size:0};NR_NONRAM_REMAP]; static mut nr_nonram_remap:u32=0;
pub unsafe fn xen_do_remap_nonram() { let mut remapped=0u32; for i in 0..nr_nonram_remap as usize { let r=&xen_nonram_remap[i]; let end=PFN_UP(r.paddr+r.size); let mut pfn=PFN_DOWN(r.paddr); let mut mfn=PFN_DOWN(r.maddr); while pfn<end { if !set_phys_to_machine(pfn,mfn){panic!("Failed to set p2m mapping");} pfn+=1;mfn+=1;remapped+=1; } } pr_info!("Remapped %u non-RAM page(s)\n",remapped); }
pub unsafe fn xen_add_remap_nonram(maddr: phys_addr_t,paddr: phys_addr_t,size: usize) { BUG_ON((maddr&!PAGE_MASK)!=(paddr&!PAGE_MASK)); if nr_nonram_remap as usize==NR_NONRAM_REMAP { xen_raw_console_write("Number of required E820 entry remapping actions exceed maximum value\n"); BUG(); } let r=&mut xen_nonram_remap[nr_nonram_remap as usize]; r.maddr=maddr;r.paddr=paddr;r.size=size;nr_nonram_remap+=1; }

#[cfg(CONFIG_ACPI)]
unsafe fn xen_acpi_os_ioremap(mut phys: acpi_physical_address, size: acpi_size) -> *mut core::ffi::c_void {
    for i in 0..nr_nonram_remap as usize { let r=&xen_nonram_remap[i]; if phys+size>r.maddr && phys<r.maddr+r.size as u64 { WARN_ON(phys<r.maddr || phys+size>r.maddr+r.size as u64); phys+=r.paddr-r.maddr; break; } }
    x86_acpi_os_ioremap(phys,size)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
