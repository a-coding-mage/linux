/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of linux/mm_types.h.  Types supplied by included kernel
// headers remain external dependencies of this translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const AT_VECTOR_SIZE_ARCH: usize = 0;
pub const AT_VECTOR_SIZE: usize = 2 * (AT_VECTOR_SIZE_ARCH + AT_VECTOR_SIZE_BASE + 1);

#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct futex_private_hash { _private: [u8; 0] }
#[repr(C)] pub struct mem_cgroup { _private: [u8; 0] }

#[repr(C)] #[derive(Copy, Clone)] pub struct memdesc_flags_t { pub f: c_ulong }

#[repr(C)] pub struct page {
    pub flags: memdesc_flags_t,
    pub data: page_data,
    pub page_type_or_mapcount: page_type_or_mapcount,
    pub _refcount: atomic_t,
    #[cfg(feature = "CONFIG_MEMCG")] pub memcg_data: c_ulong,
    #[cfg(all(not(feature = "CONFIG_MEMCG"), feature = "CONFIG_SLAB_OBJ_EXT"))]
    pub _unused_slab_obj_exts: c_ulong,
    #[cfg(feature = "WANT_PAGE_VIRTUAL")] pub virtual_: *mut c_void,
    #[cfg(feature = "LAST_CPUPID_NOT_IN_PAGE_FLAGS")] pub _last_cpupid: c_int,
    #[cfg(feature = "CONFIG_KMSAN")] pub kmsan_shadow: *mut page,
    #[cfg(feature = "CONFIG_KMSAN")] pub kmsan_origin: *mut page,
}
#[repr(C)] pub union page_data {
    pub cache: page_cache_data,
    pub page_pool: page_pool_data,
    pub compound_info: c_ulong,
    pub zone_device: zone_device_data,
    pub rcu_head: rcu_head,
}
#[repr(C)] pub struct page_cache_data { pub lru: list_head, pub mapping: *mut address_space, pub index: c_ulong, pub private: c_ulong }
#[repr(C)] pub struct page_pool_data { pub pp_magic: c_ulong, pub pp: *mut page_pool, pub _pp_mapping_pad: c_ulong, pub dma_addr: c_ulong, pub pp_ref_count: atomic_long_t }
#[repr(C)] pub struct zone_device_data { pub _unused_pgmap_compound_info: *mut c_void, pub zone_device_data: *mut c_void }
#[repr(C)] pub union page_type_or_mapcount { pub page_type: c_uint, pub _mapcount: atomic_t }

#[repr(C)] pub struct encoded_page { _private: [u8; 0] }
pub const ENCODED_PAGE_BITS: c_ulong = 3;
pub const ENCODED_PAGE_BIT_DELAY_RMAP: c_ulong = 1;
pub const ENCODED_PAGE_BIT_NR_PAGES_NEXT: c_ulong = 2;
#[inline] pub unsafe fn encode_page(page: *mut page, flags: c_ulong) -> *mut encoded_page { (flags | page as c_ulong) as *mut encoded_page }
#[inline] pub fn encoded_page_flags(page: *mut encoded_page) -> c_ulong { ENCODED_PAGE_BITS & page as c_ulong }
#[inline] pub fn encoded_page_ptr(page: *mut encoded_page) -> *mut page { (!ENCODED_PAGE_BITS & page as c_ulong) as *mut page }
#[inline] pub fn encode_nr_pages(nr: c_ulong) -> *mut encoded_page { (nr << 2) as *mut encoded_page }
#[inline] pub fn encoded_nr_pages(page: *mut encoded_page) -> c_ulong { page as c_ulong >> 2 }

#[repr(C)] #[derive(Copy, Clone)] pub struct swp_entry_t { pub val: c_ulong }
pub type softleaf_t = swp_entry_t;
pub type mm_id_mapcount_t = c_int;
pub type mm_id_t = c_uint;
pub const MM_ID_DUMMY: mm_id_t = 0;
pub const MM_ID_MIN: mm_id_t = MM_ID_DUMMY + 1;
pub const MM_ID_BITS: usize = core::mem::size_of::<mm_id_t>() * 8 - 1;
pub const MM_ID_MASK: mm_id_t = (1u32 << MM_ID_BITS) - 1;
pub const MM_ID_MAX: mm_id_t = MM_ID_MASK;
pub const FOLIO_MM_IDS_LOCK_BITNUM: usize = MM_ID_BITS;
pub const FOLIO_MM_IDS_LOCK_BIT: c_ulong = 1u64 << FOLIO_MM_IDS_LOCK_BITNUM;
pub const FOLIO_MM_IDS_SHARED_BITNUM: usize = 2 * MM_ID_BITS + 1;
pub const FOLIO_MM_IDS_SHARED_BIT: c_ulong = 1u64 << FOLIO_MM_IDS_SHARED_BITNUM;

#[repr(C)] pub struct folio { pub page: page }

#[repr(C)] pub struct ptdesc {
    pub pt_flags: memdesc_flags_t,
    pub pt_data: ptdesc_data,
    pub __page_mapping: c_ulong,
    pub pt_index: pgoff_t,
    pub _pt_pad_2: c_ulong,
    pub __page_type: c_uint,
    pub __page_refcount: atomic_t,
    #[cfg(feature = "CONFIG_MEMCG")] pub pt_memcg_data: c_ulong,
}
#[repr(C)] pub union ptdesc_data { pub pt_rcu_head: rcu_head, pub pt_list: list_head, pub pmd_huge_pte: pgtable_t }
#[inline] pub const unsafe fn ptdesc_page(pt: *mut ptdesc) -> *mut page { pt as *mut page }
#[inline] pub const unsafe fn ptdesc_folio(pt: *mut ptdesc) -> *mut folio { pt as *mut folio }
#[inline] pub const unsafe fn page_ptdesc(p: *mut page) -> *mut ptdesc { p as *mut ptdesc }

pub const STRUCT_PAGE_MAX_SHIFT: usize = order_base_2(core::mem::size_of::<page>());
#[inline] pub unsafe fn page_private(page: *mut page) -> c_ulong { (*page).data.cache.private }
#[inline] pub unsafe fn set_page_private(page: *mut page, private: c_ulong) { (*page).data.cache.private = private; }
#[inline] pub unsafe fn folio_get_private(folio: *const folio) -> *mut c_void { (*folio).page.data.cache.private as *mut c_void }
pub type vm_flags_t = c_ulong;
#[repr(C)] pub struct freeptr_t { pub v: c_ulong }

#[repr(C)] pub struct vm_region { pub vm_rb: rb_node, pub vm_flags: vm_flags_t, pub vm_start: c_ulong, pub vm_end: c_ulong, pub vm_top: c_ulong, pub vm_pgoff: c_ulong, pub vm_file: *mut file, pub vm_usage: c_int, pub vm_icache_flushed: bool }
#[repr(C)] pub struct vm_userfaultfd_ctx { pub ctx: *mut userfaultfd_ctx }
#[repr(C)] pub struct anon_vma_name { pub kref: kref, pub name: [c_char; 0] }
extern "C" { pub fn anon_vma_name(vma: *mut vm_area_struct) -> *mut anon_vma_name; pub fn anon_vma_name_alloc(name: *const c_char) -> *mut anon_vma_name; pub fn anon_vma_name_free(kref: *mut kref); }

pub const VM_REFCNT_EXCLUDE_READERS_BIT: u32 = 30;
pub const VM_REFCNT_EXCLUDE_READERS_FLAG: u32 = 1 << VM_REFCNT_EXCLUDE_READERS_BIT;
pub const VM_REFCNT_LIMIT: u32 = VM_REFCNT_EXCLUDE_READERS_FLAG - 1;
#[repr(C)] pub struct vma_numab_state { pub next_scan: c_ulong, pub pids_active_reset: c_ulong, pub pids_active: [c_ulong; 2], pub start_scan_seq: c_int, pub prev_scan_seq: c_int }
#[repr(C)] pub struct pfnmap_track_ctx { pub kref: kref, pub pfn: c_ulong, pub size: c_ulong }

#[repr(C)] #[derive(Copy, Clone)] pub enum mmap_action_type { MMAP_NOTHING, MMAP_REMAP_PFN, MMAP_IO_REMAP_PFN, MMAP_SIMPLE_IO_REMAP, MMAP_MAP_KERNEL_PAGES }
#[repr(C)] pub struct mmap_action { pub data: mmap_action_data, pub r#type: mmap_action_type, pub error_override: c_int, pub hide_from_rmap_until_complete: bool }
#[repr(C)] pub union mmap_action_data { pub remap: mmap_remap, pub simple_ioremap: mmap_simple_ioremap, pub map_kernel: mmap_map_kernel }
#[repr(C)] pub struct mmap_remap { pub start: c_ulong, pub start_pfn: c_ulong, pub size: c_ulong, pub pgprot: pgprot_t }
#[repr(C)] pub struct mmap_simple_ioremap { pub start_phys_addr: phys_addr_t, pub size: c_ulong }
#[repr(C)] pub struct mmap_map_kernel { pub start: c_ulong, pub pages: *mut *mut page, pub nr_pages: c_ulong, pub pgoff: pgoff_t }
#[repr(C)] pub struct vma_flags_t { pub __vma_flags: [c_ulong; 1] }
pub const NUM_VMA_FLAG_BITS: usize = core::mem::size_of::<c_ulong>() * 8;
#[inline] pub unsafe fn vma_flags_empty(flags: *const vma_flags_t) -> bool { (*flags).__vma_flags[0] == 0 }

#[repr(C)] pub struct vm_area_desc { pub mm: *mut mm_struct, pub file: *mut file, pub start: c_ulong, pub end: c_ulong, pub pgoff: pgoff_t, pub vm_file: *mut file, pub vma_flags: vma_flags_t, pub page_prot: pgprot_t, pub vm_ops: *const vm_operations_struct, pub private_data: *mut c_void, pub action: mmap_action }
#[repr(C)] pub struct vm_area_struct {
    pub vm_start: c_ulong, pub vm_end: c_ulong, pub vm_mm: *mut mm_struct, pub vm_page_prot: pgprot_t, pub flags: vma_flags_t,
    pub __vm_anon_pgoff_lo: c_uint, pub anon_vma_chain: list_head, pub anon_vma: *mut anon_vma, pub vm_ops: *const vm_operations_struct,
    pub vm_pgoff: c_ulong, pub vm_file: *mut file, pub vm_private_data: *mut c_void,
    pub vm_region: *mut vm_region, pub vm_policy: *mut mempolicy, pub numab_state: *mut vma_numab_state,
    pub shared: vm_area_shared, pub anon_name: *mut anon_vma_name, pub vm_userfaultfd_ctx: vm_userfaultfd_ctx,
    pub pfnmap_track_ctx: *mut pfnmap_track_ctx,
}
#[repr(C)] pub struct vm_area_shared { pub rb: rb_node, pub rb_subtree_last: c_ulong }

#[repr(C)] pub struct mm_flags_t { pub __mm_flags: [c_ulong; 1] }
#[repr(C)] pub struct mm_struct { pub mm_count: atomic_t, pub mm_mt: maple_tree, pub mmap_base: c_ulong, pub mmap_legacy_base: c_ulong, pub task_size: c_ulong, pub pgd: *mut pgd_t, pub mm_users: atomic_t, pub page_table_lock: spinlock_t, pub mmap_lock: rw_semaphore, pub mmlist: list_head, pub futex: futex_mm_data, pub total_vm: c_ulong, pub locked_vm: c_ulong, pub pinned_vm: atomic64_t, pub flags: mm_flags_t, pub flexible_array: [c_char; 0] }
extern "C" { pub static mut init_mm: mm_struct; }
#[inline] pub unsafe fn mm_cpumask(mm: *mut mm_struct) -> *mut cpumask { (&mut (*mm).flexible_array as *mut _ as *mut cpumask) }

#[repr(C)] pub struct vma_iterator { pub mas: ma_state }
#[inline] pub unsafe fn vma_iter_init(vmi: *mut vma_iterator, mm: *mut mm_struct, addr: c_ulong) { mas_init(&mut (*vmi).mas, &mut (*mm).mm_mt, addr); }

pub type vm_fault_t = c_uint;
#[repr(C)] pub enum vm_fault_reason { VM_FAULT_OOM=0x1, VM_FAULT_SIGBUS=0x2, VM_FAULT_MAJOR=0x4, VM_FAULT_HWPOISON=0x10, VM_FAULT_HWPOISON_LARGE=0x20, VM_FAULT_SIGSEGV=0x40, VM_FAULT_NOPAGE=0x100, VM_FAULT_LOCKED=0x200, VM_FAULT_RETRY=0x400, VM_FAULT_FALLBACK=0x800, VM_FAULT_DONE_COW=0x1000, VM_FAULT_NEEDDSYNC=0x2000, VM_FAULT_COMPLETED=0x4000, VM_FAULT_HINDEX_MASK=0xf0000 }
pub const VM_FAULT_ERROR: u32 = 0x1|0x2|0x40|0x10|0x20|0x800;
#[inline] pub const fn VM_FAULT_SET_HINDEX(x: u32) -> vm_fault_t { x << 16 }
#[inline] pub const fn VM_FAULT_GET_HINDEX(x: vm_fault_t) -> u32 { (x >> 16) & 0xf }
#[repr(C)] pub struct vm_special_mapping { pub name: *const c_char, pub pages: *mut *mut page, pub fault: Option<unsafe extern "C" fn(*const vm_special_mapping,*mut vm_area_struct,*mut vm_fault)->vm_fault_t>, pub mremap: Option<unsafe extern "C" fn(*const vm_special_mapping,*mut vm_area_struct)->c_int>, pub close: Option<unsafe extern "C" fn(*const vm_special_mapping,*mut vm_area_struct)> }
#[repr(C)] pub struct vm_fault { _private: [u8; 0] }
#[repr(C)] pub enum tlb_flush_reason { TLB_FLUSH_ON_TASK_SWITCH, TLB_REMOTE_SHOOTDOWN, TLB_LOCAL_SHOOTDOWN, TLB_LOCAL_MM_SHOOTDOWN, TLB_REMOTE_SEND_IPI, TLB_REMOTE_WRONG_CPU }
pub type zap_flags_t = c_uint;
pub type cydp_t = c_int;
pub const CYDP_CLEAR_YOUNG: cydp_t = 1;
pub const CYDP_CLEAR_DIRTY: cydp_t = 2;
pub const FOLL_WRITE: u32 = 1<<0; pub const FOLL_GET: u32 = 1<<1; pub const FOLL_DUMP: u32 = 1<<2; pub const FOLL_FORCE: u32 = 1<<3; pub const FOLL_NOWAIT: u32 = 1<<4; pub const FOLL_NOFAULT: u32 = 1<<5; pub const FOLL_HWPOISON: u32 = 1<<6; pub const FOLL_ANON: u32 = 1<<7; pub const FOLL_LONGTERM: u32 = 1<<8; pub const FOLL_SPLIT_PMD: u32 = 1<<9; pub const FOLL_PCI_P2PDMA: u32 = 1<<10; pub const FOLL_INTERRUPTIBLE: u32 = 1<<11; pub const FOLL_HONOR_NUMA_FAULT: u32 = 1<<12;
pub const MMF_DUMPABLE_BITS: u32 = 2; pub const MMF_DUMP_ANON_PRIVATE: u32 = 2; pub const MMF_DUMP_ANON_SHARED: u32 = 3; pub const MMF_DUMP_MAPPED_PRIVATE: u32 = 4; pub const MMF_DUMP_MAPPED_SHARED: u32 = 5; pub const MMF_DUMP_ELF_HEADERS: u32 = 6; pub const MMF_DUMP_HUGETLB_PRIVATE: u32 = 7; pub const MMF_DUMP_HUGETLB_SHARED: u32 = 8; pub const MMF_DUMP_DAX_PRIVATE: u32 = 9; pub const MMF_DUMP_DAX_SHARED: u32 = 10;
pub const MMF_VM_MERGEABLE: u32 = 16; pub const MMF_VM_HUGEPAGE: u32 = 17; pub const MMF_HUGE_ZERO_FOLIO: u32 = 18; pub const MMF_HAS_UPROBES: u32 = 19; pub const MMF_RECALC_UPROBES: u32 = 20; pub const MMF_OOM_SKIP: u32 = 21; pub const MMF_UNSTABLE: u32 = 22; pub const MMF_OOM_REAP_QUEUED: u32 = 25; pub const MMF_MULTIPROCESS: u32 = 26; pub const MMF_HAS_PINNED: u32 = 27; pub const MMF_HAS_MDWE: u32 = 28; pub const MMF_HAS_MDWE_NO_INHERIT: u32 = 29; pub const MMF_VM_MERGE_ANY: u32 = 30; pub const MMF_TOPDOWN: u32 = 31;
#[inline] pub fn mmf_init_legacy_flags(mut flags: c_ulong) -> c_ulong { if flags & (1 << MMF_HAS_MDWE_NO_INHERIT) != 0 { flags &= !((1 << MMF_HAS_MDWE) | (1 << MMF_HAS_MDWE_NO_INHERIT)); } flags }

// External kernel types and helpers referenced above are intentionally not
// implemented here; they are provided by the corresponding translated headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
