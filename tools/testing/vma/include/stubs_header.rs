/* SPDX-License-Identifier: GPL-2.0+ */

/*
 * Contains declarations that are STUBBED, that is that are rendered no-ops, in
 * order to faciliate userland VMA testing.
 */

/* Forward declarations. */
pub enum mm_struct {}
pub enum vm_area_struct {}
pub enum vm_area_desc {}
pub enum pagetable_move_control {}
pub enum mmap_action {}
pub enum file {}
pub enum anon_vma {}
pub enum anon_vma_chain {}
pub enum address_space {}
pub enum unmap_desc {}
pub enum list_head {}
pub enum anon_vma_name {}

/* These marker macros are empty in the C header: __bitwise, __randomize_layout. */

pub const FIRST_USER_ADDRESS: libc::c_ulong = 0;
pub const USER_PGTABLES_CEILING: libc::c_ulong = 0;

pub type vma_flags_t = libc::c_ulong;
pub type vm_flags_t = libc::c_ulong;
pub type pgprot_t = libc::c_ulong;

pub unsafe fn vma_policy<T>(_vma: *mut vm_area_struct) -> *mut T {
    core::ptr::null_mut()
}

pub unsafe fn down_write_nest_lock<T, U>(_sem: *mut T, _nest_lock: *mut U) {}

#[macro_export]
macro_rules! data_race {
    ($expr:expr) => {
        $expr
    };
}

pub unsafe fn ASSERT_EXCLUSIVE_WRITER<T>(_x: T) {}

#[repr(C)]
pub struct vm_userfaultfd_ctx {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mempolicy {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mmu_gather {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct vm_fault {
    _unused: [u8; 0],
}

pub unsafe fn userfaultfd_unmap_complete(_mm: *mut mm_struct, _uf: *mut list_head) {}

pub unsafe fn move_page_tables(_pmc: *mut pagetable_move_control) -> libc::c_ulong {
    0
}

pub unsafe fn free_pgd_range(
    _tlb: *mut mmu_gather,
    _addr: libc::c_ulong,
    _end: libc::c_ulong,
    _floor: libc::c_ulong,
    _ceiling: libc::c_ulong,
) {
}

pub unsafe fn ksm_execve(_mm: *mut mm_struct) -> libc::c_int {
    0
}

pub unsafe fn ksm_exit(_mm: *mut mm_struct) {}

pub unsafe fn vma_numab_state_init(_vma: *mut vm_area_struct) {}

pub unsafe fn vma_numab_state_free(_vma: *mut vm_area_struct) {}

pub unsafe fn dup_anon_vma_name(_orig_vma: *mut vm_area_struct, _new_vma: *mut vm_area_struct) {}

pub unsafe fn free_anon_vma_name(_vma: *mut vm_area_struct) {}

pub unsafe fn mmap_action_prepare(_desc: *mut vm_area_desc) -> libc::c_int {
    0
}

pub unsafe fn mmap_action_complete(
    _vma: *mut vm_area_struct,
    _action: *mut mmap_action,
    _is_compat: bool,
) -> libc::c_int {
    0
}

pub unsafe fn fixup_hugetlb_reservations(_vma: *mut vm_area_struct) {}

pub unsafe fn shmem_file(_file: *mut file) -> bool {
    false
}

pub unsafe fn ksm_vma_flags(
    _mm: *mut mm_struct,
    _file: *const file,
    vma_flags: vma_flags_t,
) -> vma_flags_t {
    vma_flags
}

pub unsafe fn remap_pfn_range_prepare(_desc: *mut vm_area_desc, _pfn: libc::c_ulong) {}

pub unsafe fn remap_pfn_range_complete(
    _vma: *mut vm_area_struct,
    _addr: libc::c_ulong,
    _pfn: libc::c_ulong,
    _size: libc::c_ulong,
    _pgprot: pgprot_t,
) -> libc::c_int {
    0
}

pub unsafe fn do_munmap(
    _mm: *mut mm_struct,
    _addr: libc::c_ulong,
    _len: usize,
    _uf: *mut list_head,
) -> libc::c_int {
    0
}

/* Currently stubbed but we may later wish to un-stub. */
pub unsafe fn vm_acct_memory(_pages: libc::c_long) {}

pub unsafe fn mmap_assert_locked(_mm: *mut mm_struct) {}

pub unsafe fn anon_vma_unlock_write(_anon_vma: *mut anon_vma) {}

pub unsafe fn i_mmap_unlock_write(_mapping: *mut address_space) {}

pub unsafe fn userfaultfd_unmap_prep(
    _vma: *mut vm_area_struct,
    _start: libc::c_ulong,
    _end: libc::c_ulong,
    _unmaps: *mut list_head,
) -> libc::c_int {
    0
}

pub unsafe fn mmap_write_downgrade(_mm: *mut mm_struct) {}

pub unsafe fn mmap_read_unlock(_mm: *mut mm_struct) {}

pub unsafe fn mmap_write_unlock(_mm: *mut mm_struct) {}

pub unsafe fn mmap_write_lock_killable(_mm: *mut mm_struct) -> libc::c_int {
    0
}

pub unsafe fn can_modify_mm(
    _mm: *mut mm_struct,
    _start: libc::c_ulong,
    _end: libc::c_ulong,
) -> bool {
    true
}

pub unsafe fn arch_unmap(_mm: *mut mm_struct, _start: libc::c_ulong, _end: libc::c_ulong) {}

pub unsafe fn mpol_equal(_a: *mut mempolicy, _b: *mut mempolicy) -> bool {
    true
}

pub unsafe fn khugepaged_enter_vma(_vma: *mut vm_area_struct, _vm_flags: vm_flags_t) {}

pub unsafe fn mapping_can_writeback(_mapping: *mut address_space) -> bool {
    true
}

pub unsafe fn is_vm_hugetlb_page(_vma: *mut vm_area_struct) -> bool {
    false
}

pub unsafe fn vma_soft_dirty_enabled(_vma: *mut vm_area_struct) -> bool {
    false
}

pub unsafe fn userfaultfd_wp(_vma: *mut vm_area_struct) -> bool {
    false
}

pub unsafe fn mmap_assert_write_locked(_mm: *mut mm_struct) {}

pub unsafe fn mutex_lock(_lock: *mut mutex) {}

pub unsafe fn mutex_unlock(_lock: *mut mutex) {}

pub unsafe fn mutex_is_locked(_lock: *mut mutex) -> bool {
    true
}

pub unsafe fn signal_pending(_p: *mut libc::c_void) -> bool {
    false
}

pub unsafe fn is_file_hugepages(_file: *const file) -> bool {
    false
}

pub unsafe fn security_vm_enough_memory_mm(_mm: *mut mm_struct, _pages: libc::c_long) -> libc::c_int {
    0
}

pub unsafe fn may_expand_vm(
    _mm: *mut mm_struct,
    _vma_flags: *const vma_flags_t,
    _npages: libc::c_ulong,
) -> bool {
    true
}

pub unsafe fn shmem_zero_setup(_vma: *mut vm_area_struct) -> libc::c_int {
    0
}

pub unsafe fn mapping_rmap_tree_insert(_vma: *mut vm_area_struct, _mapping: *mut address_space) {}

pub unsafe fn mapping_rmap_tree_remove(_vma: *mut vm_area_struct, _mapping: *mut address_space) {}

pub unsafe fn flush_dcache_mmap_unlock(_mapping: *mut address_space) {}

pub unsafe fn anon_rmap_tree_insert(_avc: *mut anon_vma_chain, _anon_vma: *mut anon_vma) {}

pub unsafe fn anon_rmap_tree_remove(_avc: *mut anon_vma_chain, _anon_vma: *mut anon_vma) {}

pub unsafe fn uprobe_mmap(_vma: *mut vm_area_struct) {}

pub unsafe fn uprobe_munmap(_vma: *mut vm_area_struct, _start: libc::c_ulong, _end: libc::c_ulong) {}

pub unsafe fn i_mmap_lock_write(_mapping: *mut address_space) {}

pub unsafe fn anon_vma_lock_write(_anon_vma: *mut anon_vma) {}

pub unsafe fn vma_assert_write_locked(_vma: *mut vm_area_struct) {}

pub unsafe fn ksm_add_vma(_vma: *mut vm_area_struct) {}

pub unsafe fn perf_event_mmap(_vma: *mut vm_area_struct) {}

pub unsafe fn vma_is_dax(_vma: *mut vm_area_struct) -> bool {
    false
}

pub unsafe fn get_gate_vma(_mm: *mut mm_struct) -> *mut vm_area_struct {
    core::ptr::null_mut()
}

pub unsafe fn arch_validate_flags(_flags: vm_flags_t) -> bool {
    true
}

pub unsafe fn vma_close(_vma: *mut vm_area_struct) {}

pub unsafe fn mmap_file(_file: *mut file, _vma: *mut vm_area_struct) -> libc::c_int {
    0
}

pub unsafe fn is_hugepage_only_range(
    _mm: *mut mm_struct,
    _addr: libc::c_ulong,
    _len: libc::c_ulong,
) -> libc::c_int {
    0
}

pub unsafe fn capable(_cap: libc::c_int) -> bool {
    true
}

pub unsafe fn anon_vma_name(_vma: *mut vm_area_struct) -> *mut anon_vma_name {
    core::ptr::null_mut()
}

pub unsafe fn is_mergeable_vm_userfaultfd_ctx(
    _vma: *mut vm_area_struct,
    _vm_ctx: vm_userfaultfd_ctx,
) -> bool {
    true
}

pub unsafe fn anon_vma_name_eq(
    _anon_name1: *mut anon_vma_name,
    _anon_name2: *mut anon_vma_name,
) -> bool {
    true
}

pub unsafe fn might_sleep() {}

pub unsafe fn fput(_file: *mut file) {}

pub unsafe fn mpol_put(_pol: *mut mempolicy) {}

pub unsafe fn lru_add_drain() {}

pub unsafe fn tlb_gather_mmu(_tlb: *mut mmu_gather, _mm: *mut mm_struct) {}

pub unsafe fn update_hiwater_rss(_mm: *mut mm_struct) {}

pub unsafe fn update_hiwater_vm(_mm: *mut mm_struct) {}

pub unsafe fn unmap_vmas(_tlb: *mut mmu_gather, _unmap: *mut unmap_desc) {}

pub unsafe fn free_pgtables(_tlb: *mut mmu_gather, _unmap: *mut unmap_desc) {}

pub unsafe fn mapping_unmap_writable(_mapping: *mut address_space) {}

pub unsafe fn flush_dcache_mmap_lock(_mapping: *mut address_space) {}

pub unsafe fn tlb_finish_mmu(_tlb: *mut mmu_gather) {}

pub unsafe fn get_file(f: *mut file) -> *mut file {
    f
}

pub unsafe fn vma_dup_policy(_src: *mut vm_area_struct, _dst: *mut vm_area_struct) -> libc::c_int {
    0
}

pub unsafe fn vma_adjust_trans_huge(
    _vma: *mut vm_area_struct,
    _start: libc::c_ulong,
    _end: libc::c_ulong,
    _next: *mut vm_area_struct,
) {
}

pub unsafe fn hugetlb_split(_vma: *mut vm_area_struct, _addr: libc::c_ulong) {}

pub unsafe fn vma_supports_mlock(_vma: *const vm_area_struct) -> bool {
    false
}
