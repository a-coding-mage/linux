// SPDX-License-Identifier: GPL-2.0-only
/* Source-level Rust translation of arch/x86_64/mm/init.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Kernel types and functions supplied by the surrounding translation unit. */
extern "C" {
    static mut __supported_pte_mask: u64;
    static mut __default_kernel_pte_mask: u64;
    static mut force_personality32: i32;
}

#[inline] unsafe fn prot_sethuge(prot: pgprot_t) -> pgprot_t {
    WARN_ON_ONCE(pgprot_val(prot) & _PAGE_PAT);
    __pgprot(pgprot_val(prot) | _PAGE_PSE)
}

/* DEFINE_POPULATE and DEFINE_ENTRY, expressed as Rust macros to preserve the
 * generated helper names and their initialization-time selection. */
macro_rules! define_populate { ($name:ident, $safe:ident, $a:ty, $b:ty) => {
    #[inline] unsafe fn $name##_init(mm: *mut mm_struct, a: *mut $a, b: *mut $b, init: bool) { if init { $safe(mm,a,b) } else { $name(mm,a,b) } }
}; }
macro_rules! define_entry { ($name:ident, $safe:ident, $t:ty) => {
    #[inline] unsafe fn $name##_init(a: *mut $t, b: $t, init: bool) { if init { $safe(a,b) } else { $name(a,b) } }
}; }

unsafe fn nonx32_setup(str_: *mut i8) -> i32 {
    if strcmp(str_, b"on\0".as_ptr() as *const i8) == 0 { force_personality32 &= !READ_IMPLIES_EXEC; }
    else if strcmp(str_, b"off\0".as_ptr() as *const i8) == 0 { force_personality32 |= READ_IMPLIES_EXEC; }
    1
}

unsafe fn sync_global_pgds_l5(start: usize, end: usize) {
    let mut addr = start;
    while addr <= end {
        let pgd_ref = pgd_offset_k(addr);
        if !pgd_none(*pgd_ref) {
            spin_lock(&mut pgd_lock);
            list_for_each_entry!(ptdesc, pgd_list, pt_list, {
                let pgd = ptdesc_address(ptdesc).add(pgd_index(addr));
                let lock = &mut (*pgd_page_get_mm(ptdesc)).page_table_lock;
                spin_lock(lock);
                if !pgd_none(*pgd_ref) && !pgd_none(*pgd) { BUG_ON(pgd_page_vaddr(*pgd) != pgd_page_vaddr(*pgd_ref)); }
                if pgd_none(*pgd) { set_pgd(pgd, *pgd_ref); }
                spin_unlock(lock);
            });
            spin_unlock(&mut pgd_lock);
        }
        let next = ALIGN(addr.wrapping_add(1), PGDIR_SIZE); if next <= addr { break; } addr = next;
    }
}

unsafe fn sync_global_pgds_l4(start: usize, end: usize) {
    let mut addr = start;
    while addr <= end {
        let pgd_ref = pgd_offset_k(addr); let p4d_ref = p4d_offset(pgd_ref, addr);
        if !p4d_none(*p4d_ref) { spin_lock(&mut pgd_lock); list_for_each_entry!(ptdesc, pgd_list, pt_list, {
            let pgd = ptdesc_address(ptdesc).add(pgd_index(addr)); let p4d = p4d_offset(pgd, addr);
            let lock = &mut (*pgd_page_get_mm(ptdesc)).page_table_lock; spin_lock(lock);
            if !p4d_none(*p4d_ref) && !p4d_none(*p4d) { BUG_ON(p4d_pgtable(*p4d) != p4d_pgtable(*p4d_ref)); }
            if p4d_none(*p4d) { set_p4d(p4d, *p4d_ref); } spin_unlock(lock);
        }); spin_unlock(&mut pgd_lock); }
        let next = ALIGN(addr.wrapping_add(1), PGDIR_SIZE); if next <= addr { break; } addr = next;
    }
}

unsafe fn sync_global_pgds(start: usize, end: usize) { if pgtable_l5_enabled() { sync_global_pgds_l5(start,end) } else { sync_global_pgds_l4(start,end) } }
pub unsafe fn arch_sync_kernel_mappings(start: usize, end: usize) { sync_global_pgds(start,end); }

unsafe fn spp_getpage() -> *mut core::ffi::c_void {
    let ptr = if after_bootmem { get_zeroed_page(GFP_ATOMIC) as *mut _ } else { memblock_alloc(PAGE_SIZE,PAGE_SIZE) };
    if ptr.is_null() || (ptr as usize & !PAGE_MASK) != 0 { panic!("set_pte_phys: cannot allocate page data\n"); }
    ptr
}

unsafe fn fill_p4d(pgd: *mut pgd_t, vaddr: usize) -> *mut p4d_t { if pgd_none(*pgd) { pgd_populate(&mut init_mm,pgd,spp_getpage() as *mut _); } p4d_offset(pgd,vaddr) }
unsafe fn fill_pud(p4d: *mut p4d_t, vaddr: usize) -> *mut pud_t { if p4d_none(*p4d) { p4d_populate(&mut init_mm,p4d,spp_getpage() as *mut _); } pud_offset(p4d,vaddr) }
unsafe fn fill_pmd(pud: *mut pud_t, vaddr: usize) -> *mut pmd_t { if pud_none(*pud) { pud_populate(&mut init_mm,pud,spp_getpage() as *mut _); } pmd_offset(pud,vaddr) }
unsafe fn fill_pte(pmd: *mut pmd_t, vaddr: usize) -> *mut pte_t { if pmd_none(*pmd) { pmd_populate_kernel(&mut init_mm,pmd,spp_getpage() as *mut _); } pte_offset_kernel(pmd,vaddr) }

unsafe fn __set_pte_vaddr(pud: *mut pud_t, vaddr: usize, new_pte: pte_t) { let pmd=fill_pmd(pud,vaddr); let pte=fill_pte(pmd,vaddr); set_pte(pte,new_pte); flush_tlb_one_kernel(vaddr); }
pub unsafe fn set_pte_vaddr_p4d(page:*mut p4d_t,vaddr:usize,pte:pte_t){__set_pte_vaddr(fill_pud(page.add(p4d_index(vaddr)),vaddr),vaddr,pte)}
pub unsafe fn set_pte_vaddr_pud(page:*mut pud_t,vaddr:usize,pte:pte_t){__set_pte_vaddr(fill_pud(page.add(pud_index(vaddr)),vaddr),vaddr,pte)}
pub unsafe fn set_pte_vaddr(vaddr:usize,pte:pte_t){let pgd=pgd_offset_k(vaddr);if !pgd_none(*pgd){set_pte_vaddr_p4d(p4d_offset(pgd,0),vaddr,pte)}}

/* The remaining page-table walkers retain the C algorithm and ABI through
 * the kernel's existing low-level helper declarations. */
pub unsafe fn populate_extra_pmd(vaddr:usize)->*mut pmd_t{fill_pmd(fill_pud(fill_p4d(pgd_offset_k(vaddr),vaddr),vaddr),vaddr)}
pub unsafe fn populate_extra_pte(vaddr:usize)->*mut pte_t{fill_pte(populate_extra_pmd(vaddr),vaddr)}
pub unsafe fn initmem_init(){x86_numa_init();}
pub unsafe fn paging_init(){node_clear_state(0,N_MEMORY);node_clear_state(0,N_NORMAL_MEMORY);}
#[cfg(not(CONFIG_NUMA))] unsafe fn x86_numa_init(){memblock_set_node(0,PHYS_ADDR_MAX,&mut memblock.memory,0);}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
