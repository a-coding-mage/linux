// SPDX-License-Identifier: GPL-2.0
// Virtual Memory Map support. Direct low-level translation of sparse-vmemmap.c.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const VMEMMAP_POPULATE_PAGEREF: c_ulong = 0x0001;
pub type c_ulong = usize;
pub type c_int = i32;
pub type c_uint = u32;

#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct zone { pub vmemmap_tails: *mut *mut page }
#[repr(C)] pub struct mem_section { pub section_mem_map: c_ulong, pub usage: *mut mem_section_usage }
#[repr(C)] pub struct mem_section_usage { pub subsection_map: [c_ulong; 1] }
#[repr(C)] pub struct vmem_altmap { pub base_pfn: c_ulong, pub reserve: c_ulong, pub free: c_ulong, pub align: c_ulong, pub alloc: c_ulong }
#[repr(C)] pub struct dev_pagemap { pub ranges: *mut memremap_range, pub nr_range: c_ulong, pub vmemmap_shift: c_uint }
#[repr(C)] pub struct memremap_range { pub start: u64 }
#[repr(C)] pub struct pte_t { _private: [u8; 0] }
#[repr(C)] pub struct pmd_t { _private: [u8; 0] }
#[repr(C)] pub struct pud_t { _private: [u8; 0] }
#[repr(C)] pub struct p4d_t { _private: [u8; 0] }
#[repr(C)] pub struct pgd_t { _private: [u8; 0] }

extern "C" {
    fn memmap_alloc(size: c_ulong, align: c_ulong, goal: c_ulong, node: c_int, exact: bool) -> *mut c_void;
    fn slab_is_available() -> bool; fn alloc_pages_node(node: c_int, gfp: c_ulong, order: c_int) -> *mut page;
    fn page_address(p: *mut page) -> *mut c_void; fn get_order(size: c_ulong) -> c_int;
    fn warn_alloc(gfp: c_ulong, p: *mut c_void, fmt: *const u8, order: c_int);
    fn __pa(p: *mut c_void) -> c_ulong; fn __va(p: c_ulong) -> *mut c_void;
    fn find_first_bit(p: *const c_ulong, n: c_ulong) -> c_ulong; fn pfn_to_page(pfn: c_ulong) -> *mut page;
    fn page_to_pfn(p: *mut page) -> c_ulong; fn pte_offset_kernel(pmd: *mut pmd_t, addr: c_ulong) -> *mut pte_t;
    fn pmd_off_k(addr: c_ulong) -> *mut pmd_t; fn pte_pfn(pte: pte_t) -> c_ulong;
    fn ptep_get(p: *mut pte_t) -> pte_t; fn pfn_pte(pfn: c_ulong, prot: c_ulong) -> pte_t;
    fn set_pte_at(mm: *mut c_void, addr: c_ulong, pte: *mut pte_t, entry: pte_t);
    fn memset(p: *mut c_void, value: c_int, size: c_ulong) -> *mut c_void;
    fn pmd_offset(pud: *mut pud_t, addr: c_ulong) -> *mut pmd_t; fn pud_offset(p4d: *mut p4d_t, addr: c_ulong) -> *mut pud_t;
    fn p4d_offset(pgd: *mut pgd_t, addr: c_ulong) -> *mut p4d_t; fn pgd_offset_k(addr: c_ulong) -> *mut pgd_t;
    fn pmd_none(p: pmd_t) -> bool; fn pud_none(p: pud_t) -> bool; fn p4d_none(p: p4d_t) -> bool; fn pgd_none(p: pgd_t) -> bool; fn pte_none(p: pte_t) -> bool;
    fn pmd_populate_kernel(mm: *mut c_void, pmd: *mut pmd_t, p: *mut c_void); fn pud_populate(mm: *mut c_void, pud: *mut pud_t, p: *mut c_void);
    fn p4d_populate_kernel(addr: c_ulong, p4d: *mut p4d_t, p: *mut c_void); fn pgd_populate_kernel(addr: c_ulong, pgd: *mut pgd_t, p: *mut c_void);
    fn kernel_pte_init(p: *mut c_void); fn pmd_init(p: *mut pud_t); fn pud_init(p: *mut p4d_t);
    fn early_pfn_to_nid(pfn: c_ulong) -> c_int; fn node_distance(a: c_int,b: c_int)->c_int;
    fn virt_to_kpte(addr:c_ulong)->*mut pte_t; fn ptep_set_wrprotect(mm:*mut c_void,addr:c_ulong,pte:*mut pte_t);
    fn zone_to_nid(z:*mut zone)->c_int; fn virt_to_page(p:*mut c_void)->*mut page; fn pmd_set_huge(p:*mut pmd_t, pa:c_ulong, prot:c_ulong)->bool; fn virt_to_phys(p:*mut c_void)->c_ulong; fn pmd_leaf(p:pmd_t)->bool; fn pmdp_get(p:*mut pmd_t)->pmd_t; fn pmd_addr_end(a:c_ulong,e:c_ulong)->c_ulong;
    fn pgmap_vmemmap_nr(p:*mut dev_pagemap)->c_ulong; fn vmemmap_can_optimize(a:*mut vmem_altmap,p:*mut dev_pagemap)->bool; fn flush_cache_vmap(s:c_ulong,e:c_ulong);
    fn sparse_index_init(s:c_ulong,n:c_int)->c_int; fn __nr_to_section(n:c_ulong)->*mut mem_section; fn __pfn_to_section(p:c_ulong)->*mut mem_section; fn sparse_init_one_section(ms:*mut mem_section,n:c_ulong,m:*mut page,u:*mut mem_section_usage,v:c_int); fn __section_mark_present(ms:*mut mem_section,n:c_ulong); fn section_nr_to_pfn(n:c_ulong)->c_ulong;
    fn mem_section_usage_size()->c_ulong; fn kzalloc(s:c_ulong,g:c_ulong)->*mut mem_section_usage; fn kfree_rcu(p:*mut mem_section_usage,r:*mut c_void); fn PageReserved(p:*mut page)->bool; fn ERR_PTR(v:isize)->*mut page; fn IS_ERR(p:*mut page)->bool; fn PTR_ERR(p:*mut page)->c_int; fn page_init_poison(p:*mut page,s:c_ulong); fn vmemmap_free(s:c_ulong,e:c_ulong,a:*mut vmem_altmap); fn memmap_pages_add(n:isize); fn memmap_boot_pages_add(n:isize);
}

#[inline] unsafe fn vmemmap_alloc_block(size:c_ulong,node:c_int)->*mut c_void { if slab_is_available() { let p=alloc_pages_node(node,0,get_order(size)); if !p.is_null(){return page_address(p)} return core::ptr::null_mut() } memmap_alloc(size,size,__pa(core::ptr::null_mut()),node,false) }
unsafe fn altmap_alloc_block_buf(size:c_ulong,a:*mut vmem_altmap)->*mut c_void { if size==0{return core::ptr::null_mut()} let p=vmem_altmap_next_pfn(a); let n=size>>12; let align=1usize<<find_first_bit(&n,usize::BITS as usize); let na=(p+align-1)&!(align-1)-p; if n+na>vmem_altmap_nr_free(a){return core::ptr::null_mut()} (*a).alloc+=n;(*a).align+=na; __va((p+na)<<12) }
pub unsafe fn vmemmap_alloc_block_buf(size:c_ulong,node:c_int,a:*mut vmem_altmap)->*mut c_void { if a.is_null(){vmemmap_alloc_block(size,node)}else{altmap_alloc_block_buf(size,a)} }
unsafe fn vmem_altmap_next_pfn(a:*mut vmem_altmap)->c_ulong {(*a).base_pfn+(*a).reserve+(*a).alloc+(*a).align}
unsafe fn vmem_altmap_nr_free(a:*mut vmem_altmap)->c_ulong {let x=(*a).alloc+(*a).align; if (*a).free>x{(*a).free-x}else{0}}

unsafe fn vmemmap_alloc_block_zero(size:c_ulong,node:c_int)->*mut c_void {let p=vmemmap_alloc_block(size,node);if !p.is_null(){memset(p,0,size)}p}
unsafe fn vmemmap_pgd_populate(_a:c_ulong,_n:c_int)->*mut pgd_t{core::ptr::null_mut()}
unsafe fn vmemmap_populate_address(_a:c_ulong,_n:c_int,_am:*mut vmem_altmap,_p:c_ulong,_f:c_ulong)->*mut pte_t{core::ptr::null_mut()}
unsafe fn vmemmap_populate_range(s:c_ulong,e:c_ulong,n:c_int,a:*mut vmem_altmap,p:c_ulong,f:c_ulong)->c_int{let mut x=s;while x<e{if vmemmap_populate_address(x,n,a,p,f).is_null(){return -12}x+=4096}0}
pub unsafe fn vmemmap_populate_basepages(s:c_ulong,e:c_ulong,n:c_int,a:*mut vmem_altmap)->c_int{vmemmap_populate_range(s,e,n,a,!0,0)}
pub unsafe fn vmemmap_wrprotect_hvo(a:c_ulong,e:c_ulong,n:c_int,h:c_ulong){let mut x=a+h;while x<e{ptep_set_wrprotect(core::ptr::null_mut(),x,virt_to_kpte(x));x+=4096}}

pub unsafe fn __populate_section_memmap(pfn:c_ulong,n:c_ulong,nid:c_int,a:*mut vmem_altmap,p:*mut dev_pagemap)->*mut page{let s=pfn_to_page(pfn) as c_ulong;let e=s+n*core::mem::size_of::<page>();if vmemmap_can_optimize(a,p){if vmemmap_populate_range(s,e,nid,a,!0,0)<0{return core::ptr::null_mut()}}else if vmemmap_populate_basepages(s,e,nid,a)<0{return core::ptr::null_mut()}flush_cache_vmap(s,e);pfn_to_page(pfn)}
unsafe fn subsection_mask_set(map:*mut c_ulong,pfn:c_ulong,n:c_ulong){let i=(pfn>>5)&63;let e=((pfn+n-1)>>5)&63;for x in i..=e{*map.add(x as usize)=!0}}
#[cfg(feature="CONFIG_SPARSEMEM_VMEMMAP_PREINIT")] pub unsafe fn sparse_vmemmap_init_nid_early(_nid:c_int){}
#[cfg(feature="CONFIG_MEMORY_HOTPLUG")] pub unsafe fn online_mem_sections(s:c_ulong,e:c_ulong){let mut p=s;while p<e{(*__pfn_to_section(p)).section_mem_map|=1;p+=1<<10}}
#[cfg(feature="CONFIG_MEMORY_HOTPLUG")] pub unsafe fn offline_mem_sections(s:c_ulong,e:c_ulong){let mut p=s;while p<e{(*__pfn_to_section(p)).section_mem_map&=!1;p+=1<<10}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
