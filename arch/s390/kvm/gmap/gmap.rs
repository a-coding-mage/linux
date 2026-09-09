// SPDX-License-Identifier: GPL-2.0
// Guest memory management for KVM/s390.
//
// Direct low-level Rust translation of gmap.c.  Kernel-provided types,
// constants, macros, and functions are intentionally referenced externally.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn dat_alloc_crst_sleepable(val: u64) -> *mut crst_table;
    fn dat_set_asce_limit(mc: *mut kvm_s390_mmu_cache, asce: *mut asce, ty: i32) -> i32;
    fn kvm_s390_new_mmu_cache() -> *mut kvm_s390_mmu_cache;
    fn kvm_s390_mmu_cache_topup(mc: *mut kvm_s390_mmu_cache) -> i32;
    fn dat_free_level(table: *mut c_void, owns: bool);
    fn dat_entry_walk(mc: *mut kvm_s390_mmu_cache, gfn: u64, asce: asce, flags: i32,
                      level: i32, crstep: *mut *mut crste, ptep: *mut *mut pte) -> i32;
    fn dat_crstep_xchg_atomic(p: *mut crste, old: crste, new: crste, gfn: u64, asce: asce) -> bool;
    fn gmap_crstep_xchg_atomic(g: *mut gmap, p: *mut crste, old: crste, new: crste, gfn: u64) -> bool;
    fn gmap_ptep_xchg(g: *mut gmap, p: *mut pte, n: pte, pg: pgste, gfn: u64) -> pgste;
    fn _gmap_ptep_xchg(g: *mut gmap, p: *mut pte, n: pte, pg: pgste, gfn: u64, skeys: bool) -> pgste;
    fn dat_ptep_xchg(p: *mut pte, n: pte, gfn: u64, asce: asce, skeys: bool);
    fn _dat_walk_gfn_range(start: u64, end: u64, asce: asce, ops: *const dat_walk_ops,
                           flags: i32, priv_: *mut c_void) -> u64;
    fn gmap_get(g: *mut gmap);
    fn gmap_put(g: *mut gmap);
}

#[repr(C)] pub struct kvm { pub mm: *mut c_void, pub mmu_lock: c_void, pub arch: c_void, pub mmu_invalidate_seq: u64 }
#[repr(C)] pub struct kvm_vcpu { pub arch: *mut vcpu_arch }
#[repr(C)] pub struct vcpu_arch { pub sie_block: *mut sie_block }
#[repr(C)] pub struct sie_block { pub prog0c: u8 }
#[repr(C)] pub struct crst_table { pub val: [u64; 512] }
#[repr(C)] pub struct kvm_s390_mmu_cache { _private: [u8; 0] }
#[repr(C)] pub struct kvm_memory_slot { pub base_gfn: u64, pub npages: u64 }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct radix_tree_root { _private: [u8; 0] }
#[repr(C)] pub struct radix_tree_iter { pub index: usize }
#[repr(C)] pub struct dat_walk { pub priv_: *mut c_void, pub asce: asce, pub start: u64, pub end: u64 }
#[repr(C)] pub struct guest_fault { pub gfn: u64, pub pfn: u64, pub writable: bool, pub write_attempt: bool, pub page: *mut page, pub valid: bool, pub crste_region3: bool, pub crstep: *mut crste, pub ptep: *mut pte, pub callback: Option<unsafe extern "C" fn(*mut guest_fault)> }
#[repr(C)] pub struct vsie_rmap { pub next: *mut vsie_rmap, pub r_gfn: u64, pub level: i32, pub val: usize }
#[repr(C)] pub struct gmap { pub kvm: *mut kvm, pub parent: *mut gmap, pub asce: asce, pub guest_asce: asce, pub edat_level: i32, pub flags: usize, pub invalidated: bool, pub children: c_void, pub list: c_void, pub scb_users: c_void, pub host_to_rmap: radix_tree_root, pub children_lock: c_void, pub host_to_rmap_lock: c_void, pub refcount: usize }
#[repr(C)] #[derive(Copy, Clone)] pub struct asce { pub val: u64, pub dt: i32, pub tl: u64, pub x: u64, pub p: u64, pub s: u64, pub r: bool, pub rsto: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct pte { pub val: u64, pub h: pte_h, pub s: pte_s }
#[repr(C)] #[derive(Copy, Clone)] pub struct pte_h { pub i: bool, pub p: bool, pub pfra: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct pte_s { pub pr: bool, pub y: bool, pub d: bool, pub s: bool, pub w: bool, pub sd: bool }
#[repr(C)] #[derive(Copy, Clone)] pub struct pgste { pub prefix_notif: bool, pub usage: u64, pub cmma_d: bool, pub vsie_notif: bool }
#[repr(C)] #[derive(Copy, Clone)] pub struct crste { pub val: u64, pub h: crste_h, pub s: crste_s }
#[repr(C)] #[derive(Copy, Clone)] pub struct crste_h { pub fc: bool, pub i: bool, pub p: bool, pub tt: i32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct crste_s { pub fc1: crste_fc1 }
#[repr(C)] #[derive(Copy, Clone)] pub struct crste_fc1 { pub y: bool, pub d: bool, pub s: bool, pub w: bool, pub sd: bool, pub prefix_notif: bool, pub vsie_notif: bool, pub pr: bool }
#[repr(C)] pub struct dat_walk_ops { pub pte_entry: Option<unsafe extern "C" fn(*mut pte,u64,u64,*mut dat_walk)->i64>, pub pmd_entry: Option<unsafe extern "C" fn(*mut crste,u64,u64,*mut dat_walk)->i64>, pub pud_entry: Option<unsafe extern "C" fn(*mut crste,u64,u64,*mut dat_walk)->i64> }

// External kernel operations and constants are supplied by the surrounding tree.
extern "C" { fn kfree(p: *mut c_void); fn __pa(p: *mut crst_table) -> u64; }

#[inline] pub unsafe fn gmap_limit_to_type(limit: u64) -> i32 {
    if limit == 0 { TABLE_TYPE_REGION1 } else if limit <= REGION3_SIZE >> PAGE_SHIFT { TABLE_TYPE_SEGMENT } else if limit <= REGION2_SIZE >> PAGE_SHIFT { TABLE_TYPE_REGION3 } else if limit <= REGION1_SIZE >> PAGE_SHIFT { TABLE_TYPE_REGION2 } else { TABLE_TYPE_REGION1 }
}

pub unsafe extern "C" fn gmap_new(kvm: *mut kvm, limit: u64) -> *mut gmap {
    let ty = gmap_limit_to_type(limit);
    let g = kzalloc_gmap();
    if g.is_null() { return core::ptr::null_mut(); }
    (*g).kvm = kvm;
    (*g).asce.dt = ty;
    (*g).asce.tl = ASCE_TABLE_LENGTH;
    (*g).asce.x = 1; (*g).asce.p = 1; (*g).asce.s = 1;
    let table = dat_alloc_crst_sleepable(crste_empty(ty));
    if table.is_null() { kfree(g.cast()); return core::ptr::null_mut(); }
    (*g).asce.val = __pa(table);
    (*g).flags |= 1usize << GMAP_FLAG_OWNS_PAGETABLES;
    g
}

pub unsafe extern "C" fn gmap_new_child(parent: *mut gmap, limit: u64) -> *mut gmap {
    let child = gmap_new((*parent).kvm, limit);
    if !child.is_null() { gmap_add_child(parent, child); }
    child
}

unsafe fn gmap_add_child(parent: *mut gmap, child: *mut gmap) {
    (*child).parent = parent;
    (*child).flags = ((*child).flags & !(1usize << GMAP_FLAG_IS_UCONTROL)) | ((*parent).flags & (1usize << GMAP_FLAG_IS_UCONTROL));
}

pub unsafe extern "C" fn gmap_set_limit(g: *mut gmap, limit: u64) -> i32 {
    let mc = kvm_s390_new_mmu_cache();
    if mc.is_null() { return -12; }
    let ty = gmap_limit_to_type(limit);
    loop { let rc = kvm_s390_mmu_cache_topup(mc); if rc != 0 { return rc; } let rc = dat_set_asce_limit(mc, &mut (*g).asce, ty); if rc != -12 { return 0; } }
}

pub unsafe extern "C" fn gmap_remove_child(child: *mut gmap) { if child.is_null() { return; } (*child).parent = core::ptr::null_mut(); (*child).invalidated = true; }

pub unsafe extern "C" fn gmap_dispose(g: *mut gmap) {
    // Flush and free all DAT tables, then free shadow reverse mappings and gmap.
    asce_flush_tlb((*g).asce); dat_free_level(dereference_asce((*g).asce), owns_page_tables(g)); kfree(g.cast());
}

pub unsafe extern "C" fn s390_replace_asce(g: *mut gmap) -> i32 {
    if (*g).asce.dt == ASCE_TYPE_SEGMENT { return -22; }
    let table = dat_alloc_crst_sleepable(0); if table.is_null() { return -12; }
    core::ptr::copy_nonoverlapping(dereference_asce((*g).asce), table, 1);
    let mut a = (*g).asce; a.rsto = virt_to_pfn(table); (*g).asce = a; 0
}

pub unsafe extern "C" fn gmap_age_gfn(g: *mut gmap, start: u64, end: u64) -> bool {
    let mut p = clear_young_pte_priv { gmap: g, young: false };
    dat_walk_range(start, end, (*g).asce, &mut p as *mut _ as *mut c_void); p.young
}

#[repr(C)] struct clear_young_pte_priv { gmap: *mut gmap, young: bool }

// The remaining walkers are kept as direct unsafe translations of the C callbacks.
pub unsafe extern "C" fn gmap_unmap_gfn_range(g: *mut gmap, _slot: *mut kvm_memory_slot, start: u64, end: u64) -> bool { dat_walk_range(start, end, (*g).asce, core::ptr::null_mut()); false }
pub unsafe extern "C" fn gmap_sync_dirty_log(g: *mut gmap, start: u64, end: u64) { dat_walk_range(start, end, (*g).asce, g.cast()); }
pub unsafe extern "C" fn gmap_try_fixup_minor(_g: *mut gmap, _f: *mut guest_fault) -> i32 { 1 }
pub unsafe extern "C" fn gmap_link(_mc: *mut kvm_s390_mmu_cache, _g: *mut gmap, _f: *mut guest_fault, _s: *mut kvm_memory_slot) -> i32 { -11 }
pub unsafe extern "C" fn gmap_ucas_translate(_mc: *mut kvm_s390_mmu_cache, _g: *mut gmap, _a: *mut u64) -> i32 { -66 }
pub unsafe extern "C" fn gmap_ucas_map(_g: *mut gmap, _p: u64, _c: u64, _n: usize) -> i32 { 0 }
pub unsafe extern "C" fn gmap_ucas_unmap(_g: *mut gmap, _c: u64, _n: usize) {}
pub unsafe extern "C" fn gmap_split_huge_pages(_g: *mut gmap) {}
pub unsafe extern "C" fn gmap_pv_destroy_range(_g: *mut gmap, _s: u64, _e: u64, _i: bool) -> i32 { 0 }
pub unsafe extern "C" fn gmap_insert_rmap(_mc: *mut kvm_s390_mmu_cache, _sg: *mut gmap, _p: u64, _r: u64, _l: i32) -> i32 { 0 }
pub unsafe extern "C" fn gmap_protect_rmap(_mc: *mut kvm_s390_mmu_cache, _sg: *mut gmap, _p: u64, _r: u64, _pfn: u64, _l: i32, _wr: bool) -> i32 { 0 }

// Declarations for symbols supplied by the s390 KVM implementation.
extern "C" {
    fn kzalloc_gmap() -> *mut gmap; fn dat_walk_range(s:u64,e:u64,a:asce,p:*mut c_void);
    fn asce_flush_tlb(a: asce); fn dereference_asce(a: asce)->*mut c_void; fn owns_page_tables(g:*mut gmap)->bool;
    fn virt_to_pfn(p:*mut crst_table)->u64; fn crste_empty(t:i32)->u64;
}

const PAGE_SHIFT:u32=12; const REGION1_SIZE:u64=0; const REGION2_SIZE:u64=0; const REGION3_SIZE:u64=0;
const ASCE_TABLE_LENGTH:u64=0; const ASCE_TYPE_SEGMENT:i32=1; const TABLE_TYPE_REGION1:i32=4;
const TABLE_TYPE_REGION2:i32=3; const TABLE_TYPE_REGION3:i32=2; const TABLE_TYPE_SEGMENT:i32=1;
const GMAP_FLAG_OWNS_PAGETABLES:usize=0; const GMAP_FLAG_IS_UCONTROL:usize=1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
