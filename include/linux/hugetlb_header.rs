/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/hugetlb.h. Kernel-provided types and functions are external dependencies.

pub const __NR_USED_SUBPAGE: ::core::ffi::c_int = 3;

#[repr(C)] pub struct hugepage_subpool { pub lock: spinlock_t, pub count: c_long, pub max_hpages: c_long, pub used_hpages: c_long, pub hstate: *mut hstate, pub min_hpages: c_long, pub rsv_hpages: c_long }
#[repr(C)] pub struct resv_map { pub refs: kref, pub lock: spinlock_t, pub regions: list_head, pub adds_in_progress: c_long, pub region_cache: list_head, pub region_cache_count: c_long, pub rw_sema: rw_semaphore, #[cfg(CONFIG_CGROUP_HUGETLB)] pub reservation_counter: *mut page_counter, #[cfg(CONFIG_CGROUP_HUGETLB)] pub pages_per_hpage: c_ulong, #[cfg(CONFIG_CGROUP_HUGETLB)] pub css: *mut cgroup_subsys_state }
#[repr(C)] pub struct file_region { pub link: list_head, pub from: c_long, pub to: c_long, #[cfg(CONFIG_CGROUP_HUGETLB)] pub reservation_counter: *mut page_counter, #[cfg(CONFIG_CGROUP_HUGETLB)] pub css: *mut cgroup_subsys_state }
#[repr(C)] pub struct hugetlb_vma_lock { pub refs: kref, pub rw_sema: rw_semaphore, pub vma: *mut vm_area_struct }

pub const HUGETLB_ANON_FILE: &str = "anon_hugepage";
pub const HUGETLB_SHMFS_INODE: c_int = 1;
pub const HUGETLB_ANONHUGE_INODE: c_int = 2;
pub const HSTATE_NAME_LEN: usize = 32;
pub const HUGE_BOOTMEM_HVO: c_int = 0x0001;
pub const HUGE_BOOTMEM_ZONES_VALID: c_int = 0x0002;
pub const HUGE_BOOTMEM_CMA: c_int = 0x0004;

#[repr(C)] pub struct mempolicy_interpreted { pub nid: c_int, pub nodemask: *mut nodemask_t, pub mode: mempolicy_mode }
#[repr(C)] pub struct hstate { pub resize_lock: mutex, pub resize_key: lock_class_key, pub next_nid_to_alloc: c_int, pub next_nid_to_free: c_int, pub order: c_uint, pub demote_order: c_uint, pub mask: c_ulong, pub max_huge_pages: c_ulong, pub nr_huge_pages: c_ulong, pub free_huge_pages: c_ulong, pub resv_huge_pages: c_ulong, pub surplus_huge_pages: c_ulong, pub nr_overcommit_huge_pages: c_ulong, pub hugepage_activelist: list_head, pub hugepage_freelists: [list_head; MAX_NUMNODES], pub max_huge_pages_node: [c_uint; MAX_NUMNODES], pub nr_huge_pages_node: [c_uint; MAX_NUMNODES], pub free_huge_pages_node: [c_uint; MAX_NUMNODES], pub surplus_huge_pages_node: [c_uint; MAX_NUMNODES], pub name: [c_char; HSTATE_NAME_LEN] }

#[repr(C)] pub struct hugetlbfs_sb_info { pub max_inodes: c_long, pub free_inodes: c_long, pub stat_lock: spinlock_t, pub hstate: *mut hstate, pub spool: *mut hugepage_subpool, pub uid: kuid_t, pub gid: kgid_t, pub mode: umode_t }
#[repr(C)] pub struct hugetlbfs_inode_info { pub vfs_inode: inode, pub resv_map: *mut resv_map, pub seals: c_uint }

#[repr(u32)] pub enum hugetlb_page_flags { HPG_restore_reserve=0, HPG_migratable, HPG_temporary, HPG_freed, HPG_vmemmap_optimized, HPG_raw_hwp_unreliable, HPG_cma, __NR_HPAGEFLAGS }
#[repr(u32)] pub enum hugetlb_alloc_flag { HUGETLB_ALLOC_CHARGE_CGROUP_RSVD_BIT=0, HUGETLB_ALLOC_USE_GLOBAL_RESERVATIONS_BIT }
pub const HUGETLB_ALLOC_CHARG_CGROUP_RSVD: u32 = 1 << 0;
pub const HUGETLB_ALLOC_USE_GLOBAL_RESERVATIONS: u32 = 1 << 1;

extern "C" {
    pub fn free_huge_folio(folio: *mut folio);
    pub static mut hugetlb_lock: spinlock_t;
    pub static mut hugetlb_max_hstate: c_int;
    pub static mut hstates: [hstate; HUGE_MAX_HSTATE];
    pub static mut default_hstate_idx: c_uint;
    pub fn resv_map_alloc() -> *mut resv_map; pub fn resv_map_release(r: *mut kref);
    pub fn hugepage_new_subpool(h: *mut hstate, max: c_long, min: c_long) -> *mut hugepage_subpool; pub fn hugepage_put_subpool(s: *mut hugepage_subpool);
    pub fn hugetlb_dup_vma_private(v: *mut vm_area_struct); pub fn clear_vma_resv_huge_pages(v: *mut vm_area_struct);
    pub fn move_hugetlb_page_tables(v: *mut vm_area_struct,n: *mut vm_area_struct,o:c_ulong,nn:c_ulong,l:c_ulong)->c_int;
    pub fn copy_hugetlb_page_range(a:*mut mm_struct,b:*mut mm_struct,c:*mut vm_area_struct,d:*mut vm_area_struct)->c_int;
    pub fn hugetlb_total_pages()->c_ulong; pub fn hugetlb_fault(mm:*mut mm_struct,v:*mut vm_area_struct,a:c_ulong,f:c_uint)->vm_fault_t;
    pub fn hugetlb_report_meminfo(m:*mut seq_file); pub fn hugetlb_report_node_meminfo(b:*mut c_char,l:c_int,n:c_int)->c_int; pub fn hugetlb_show_meminfo_node(n:c_int);
    pub fn huge_pte_alloc(mm:*mut mm_struct,v:*mut vm_area_struct,a:c_ulong,s:c_ulong)->*mut pte_t; pub fn huge_pte_offset(mm:*mut mm_struct,a:c_ulong,s:c_ulong)->*mut pte_t;
    pub fn hugetlb_mask_last_page(h:*mut hstate)->c_ulong; pub fn huge_pmd_unshare(t:*mut mmu_gather,v:*mut vm_area_struct,a:c_ulong,p:*mut pte_t)->c_int;
    pub fn huge_pmd_unshare_flush(t:*mut mmu_gather,v:*mut vm_area_struct); pub fn adjust_range_if_pmd_sharing_possible(v:*mut vm_area_struct,s:*mut c_ulong,e:*mut c_ulong);
    pub fn hugetlb_vma_lock_read(v:*mut vm_area_struct); pub fn hugetlb_vma_unlock_read(v:*mut vm_area_struct); pub fn hugetlb_vma_lock_write(v:*mut vm_area_struct); pub fn hugetlb_vma_unlock_write(v:*mut vm_area_struct); pub fn hugetlb_vma_trylock_write(v:*mut vm_area_struct)->c_int;
    pub fn isolate_or_dissolve_huge_folio(f:*mut folio,l:*mut list_head)->c_int; pub fn replace_free_hugepage_folios(s:c_ulong,e:c_ulong)->c_int; pub fn wait_for_freed_hugetlb_folios();
    pub fn hugetlb_alloc_folio(h:*mut hstate,m:*mut mempolicy_interpreted,f:u8)->*mut folio; pub fn size_to_hstate(s:c_ulong)->*mut hstate; pub fn dissolve_free_hugetlb_folio(f:*mut folio)->c_int; pub fn dissolve_free_hugetlb_folios(s:c_ulong,e:c_ulong)->c_int;
    pub fn hugetlb_report_usage(m:*mut seq_file,mm:*mut mm_struct);
}

#[inline] pub unsafe fn default_hstate() -> *mut hstate { &mut hstates[default_hstate_idx as usize] }
#[inline] pub unsafe fn huge_page_size(h:*const hstate)->c_ulong { (PAGE_SIZE as c_ulong) << (*h).order }
#[inline] pub unsafe fn huge_page_mask(h:*mut hstate)->c_ulong { (*h).mask }
#[inline] pub unsafe fn huge_page_order(h:*mut hstate)->c_uint { (*h).order }
#[inline] pub unsafe fn huge_page_shift(h:*mut hstate)->c_uint { (*h).order + PAGE_SHIFT }
#[inline] pub unsafe fn order_is_gigantic(o:c_uint)->bool { o > MAX_PAGE_ORDER }
#[inline] pub unsafe fn hstate_is_gigantic(h:*mut hstate)->bool { order_is_gigantic((*h).order) }
#[inline] pub unsafe fn pages_per_huge_page(h:*const hstate)->c_uint { 1u32 << (*h).order }
#[inline] pub unsafe fn blocks_per_huge_page(h:*mut hstate)->c_uint { (huge_page_size(h) / 512) as c_uint }
#[inline] pub unsafe fn hstate_index_to_shift(i:c_uint)->c_uint { hstates[i as usize].order + PAGE_SHIFT }
#[inline] pub unsafe fn hstate_index(h:*mut hstate)->c_int { h.offset_from(hstates.as_mut_ptr()) as c_int }

// The remaining inline helpers retain their C semantics and use kernel-provided operations.
#[inline] pub unsafe fn hugepage_migration_supported(_h:*mut hstate)->bool { false }
#[inline] pub unsafe fn hugepage_movable_supported(h:*mut hstate)->bool { hugepage_migration_supported(h) }
#[inline] pub unsafe fn htlb_allow_alloc_fallback(_reason:migrate_reason)->bool { false }
#[inline] pub unsafe fn hugetlb_count_init(mm:*mut mm_struct) { atomic_long_set(&mut (*mm).hugetlb_usage, 0); }
#[inline] pub unsafe fn hugetlb_count_add(l:c_long,mm:*mut mm_struct) { atomic_long_add(l,&mut (*mm).hugetlb_usage); }
#[inline] pub unsafe fn hugetlb_count_sub(l:c_long,mm:*mut mm_struct) { atomic_long_sub(l,&mut (*mm).hugetlb_usage); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
