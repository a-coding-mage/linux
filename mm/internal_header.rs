/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of internal.h.  Linux types and helpers are supplied by dependencies. */

#[repr(C)]
pub struct huge_bootmem_page { pub list: list_head, pub hstate: *mut hstate, pub flags: c_ulong }

extern "C" {
    pub fn workingset_test_recent(shadow: *mut c_void, file: bool, workingset: *mut bool, flush: bool) -> bool;
    pub fn workingset_age_nonresident(lruvec: *mut lruvec, nr_pages: c_ulong);
    pub fn workingset_eviction(folio: *mut folio, target_memcg: *mut mem_cgroup) -> *mut c_void;
    pub fn workingset_refault(folio: *mut folio, shadow: *mut c_void);
    pub fn workingset_activation(folio: *mut folio);
    pub fn folio_add_lru_vma(folio: *mut folio, vma: *mut vm_area_struct);
    pub fn lru_cache_disable(); pub fn lru_add_drain(); pub fn lru_add_drain_cpu(cpu: c_int);
    pub fn lru_add_drain_cpu_zone(zone: *mut zone); pub fn folio_deactivate(folio: *mut folio);
    pub fn folio_mark_lazyfree(folio: *mut folio);
    pub fn zone_reclaimable_pages(zone: *mut zone) -> c_ulong;
    pub fn try_to_free_pages(zonelist: *mut zonelist, order: c_int, gfp_mask: gfp_t, mask: *const nodemask_t) -> c_ulong;
    pub fn lruvec_lru_size(lruvec: *mut lruvec, lru: lru_list, zone_idx: c_int) -> c_ulong;
    pub fn try_to_free_mem_cgroup_pages(memcg: *mut mem_cgroup, nr_pages: c_ulong, gfp_mask: gfp_t, reclaim_options: c_uint, swappiness: *mut c_int) -> c_ulong;
    pub fn mem_cgroup_shrink_node(memcg: *mut mem_cgroup, gfp_mask: gfp_t, noswap: bool, pgdat: *mut pg_data_t, nr_scanned: *mut c_ulong) -> c_ulong;
    pub fn page_writeback_init();
}

pub const MEMCG_RECLAIM_MAY_SWAP: c_uint = 1 << 1;
pub const MEMCG_RECLAIM_PROACTIVE: c_uint = 1 << 2;
pub const MIN_SWAPPINESS: c_int = 0; pub const MAX_SWAPPINESS: c_int = 200;
pub const SWAPPINESS_ANON_ONLY: c_int = MAX_SWAPPINESS + 1;
pub const ENTIRELY_MAPPED: c_uint = 0x800000; pub const FOLIO_PAGES_MAPPED: c_uint = ENTIRELY_MAPPED - 1;
pub const SHOW_MEM_FILTER_NODES: c_uint = 0x0001;

#[repr(C)] pub struct pagetable_move_control { pub old: *mut vm_area_struct, pub new: *mut vm_area_struct, pub old_addr: c_ulong, pub old_end: c_ulong, pub new_addr: c_ulong, pub len_in: c_ulong, pub need_rmap_locks: bool, pub for_stack: bool }
#[macro_export] macro_rules! PAGETABLE_MOVE { ($name:ident,$old:expr,$new:expr,$oa:expr,$na:expr,$len:expr) => { let mut $name = pagetable_move_control { old:$old,new:$new,old_addr:$oa,old_end:$oa+$len,new_addr:$na,len_in:$len,need_rmap_locks:false,for_stack:false }; } }

pub const MAX_RECLAIM_RETRIES: c_int = 16;

#[cfg(feature="mmu")]
extern "C" {
    pub fn unmap_vmas(tlb:*mut mmu_gather, unmap:*mut unmap_desc);
    pub fn cond_install_uffd_wp_ptes(vma:*mut vm_area_struct, addr:c_ulong, ptep:*mut pte_t, pte:pte_t, nr_ptes:c_ulong)->bool;
    pub fn __put_anon_vma(v:*mut anon_vma); pub fn folio_get_anon_vma(f:*const folio)->*mut anon_vma;
    pub fn anon_vma_clone(dst:*mut vm_area_struct, src:*mut vm_area_struct, op:vma_operation)->c_int;
    pub fn anon_vma_fork(vma:*mut vm_area_struct,pvma:*mut vm_area_struct)->c_int;
    pub fn __anon_vma_prepare(vma:*mut vm_area_struct)->c_int; pub fn unlink_anon_vmas(vma:*mut vm_area_struct);
    pub fn folio_pte_batch(folio:*mut folio,ptep:*mut pte_t,pte:pte_t,max_nr:c_uint)->c_uint;
    pub fn do_swap_page(vmf:*mut vm_fault)->vm_fault_t; pub fn folio_rotate_reclaimable(f:*mut folio);
    pub fn __folio_end_writeback(f:*mut folio)->bool; pub fn deactivate_file_folio(f:*mut folio); pub fn folio_activate(f:*mut folio);
    pub fn free_pgtables(tlb:*mut mmu_gather,desc:*mut unmap_desc); pub fn pmd_install(mm:*mut mm_struct,pmd:*mut pmd_t,pte:*mut pgtable_t);
    pub fn zap_vma_for_reaping(vma:*mut vm_area_struct)->c_int; pub fn folio_unmap_invalidate(m:*mut address_space,f:*mut folio,g:gfp_t)->c_int;
}
#[repr(C)] pub enum vma_operation { VMA_OP_SPLIT, VMA_OP_MERGE_UNFAULTED, VMA_OP_REMAP, VMA_OP_FORK }
pub type fpb_t = c_int;
pub const FPB_RESPECT_DIRTY:fpb_t=1; pub const FPB_RESPECT_SOFT_DIRTY:fpb_t=2; pub const FPB_RESPECT_WRITE:fpb_t=4; pub const FPB_MERGE_WRITE:fpb_t=8; pub const FPB_MERGE_YOUNG_DIRTY:fpb_t=16;

pub const FOLL_TOUCH:c_uint=1<<16; pub const FOLL_TRIED:c_uint=1<<17; pub const FOLL_REMOTE:c_uint=1<<18; pub const FOLL_PIN:c_uint=1<<19; pub const FOLL_FAST_ONLY:c_uint=1<<20; pub const FOLL_UNLOCKABLE:c_uint=1<<21; pub const FOLL_MADV_POPULATE:c_uint=1<<22;
pub const INTERNAL_GUP_FLAGS:c_uint=FOLL_TOUCH|FOLL_TRIED|FOLL_REMOTE|FOLL_PIN|FOLL_FAST_ONLY|FOLL_UNLOCKABLE|FOLL_MADV_POPULATE;

extern "C" {
    pub fn acct_reclaim_writeback(folio:*mut folio); pub fn vmf_anon_prepare(vmf:*mut vm_fault)->vm_fault_t;
    pub fn free_zone_device_folio(f:*mut folio); pub fn migrate_device_coherent_folio(f:*mut folio)->c_int;
    pub fn try_grab_folio(f:*mut folio,refs:c_int,flags:c_uint)->c_int; pub fn touch_pud(v:*mut vm_area_struct,a:c_ulong,p:*mut pud_t,w:bool); pub fn touch_pmd(v:*mut vm_area_struct,a:c_ulong,p:*mut pmd_t,w:bool);
    pub fn shrink_slab(g:gfp_t,nid:c_int,memcg:*mut mem_cgroup,priority:c_int)->c_ulong;
    pub fn move_page_tables(pmc:*mut pagetable_move_control)->c_ulong;
    pub fn reclaim_pages(list:*mut list_head)->c_ulong; pub fn reclaim_clean_pages_from_list(z:*mut zone,l:*mut list_head)->c_uint;
    pub fn setup_zone_pageset(z:*mut zone); pub fn numa_migrate_check(f:*mut folio,vmf:*mut vm_fault,addr:c_ulong,flags:*mut c_int,writable:bool,last_cpupid:*mut c_int);
    pub fn get_order_from_str(size:*const c_char,valid:c_ulong)->c_int;
}

// File-local inline helpers retain their C semantics through the external kernel APIs.
#[inline] pub unsafe fn folio_may_be_lru_cached(f:*const folio)->bool { !folio_test_large(f) }
#[inline] pub unsafe fn node_reclaim_enabled(mode:c_int)->bool { (mode & (RECLAIM_ZONE|RECLAIM_WRITE|RECLAIM_UNMAP)) != 0 }
#[inline] pub unsafe fn vma_is_single_threaded_private(vma:*mut vm_area_struct)->bool { ((*vma).vm_flags & VM_SHARED)==0 && atomic_read(&(*(*vma).vm_mm).mm_users)==1 }
#[inline] pub unsafe fn pte_next_swp_offset(pte:pte_t)->pte_t { pte_move_swp_offset(pte,1) }
extern "C" { pub fn pte_move_swp_offset(pte:pte_t,delta:c_long)->pte_t; pub fn pte_soft_dirty(pte:pte_t)->bool; pub fn pmd_soft_dirty(pmd:pmd_t)->bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
