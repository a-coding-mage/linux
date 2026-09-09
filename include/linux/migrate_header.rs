/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

pub type new_folio_t = unsafe extern "C" fn(folio: *mut folio, private: c_ulong) -> *mut folio;
pub type free_folio_t = unsafe extern "C" fn(folio: *mut folio, private: c_ulong);

#[repr(C)]
pub struct movable_operations {
    pub isolate_page: Option<unsafe extern "C" fn(*mut page, isolate_mode_t) -> bool>,
    pub migrate_page: Option<unsafe extern "C" fn(*mut page, *mut page, migrate_mode) -> c_int>,
    pub putback_page: Option<unsafe extern "C" fn(*mut page)>,
}

extern "C" {
    pub static migrate_reason_names: [*const c_char; MR_TYPES as usize];
}

#[cfg(feature = "CONFIG_MIGRATION")]
extern "C" {
    pub fn putback_movable_pages(l: *mut list_head);
    pub fn migrate_folio(mapping: *mut address_space, dst: *mut folio, src: *mut folio,
                         mode: migrate_mode) -> c_int;
    pub fn migrate_pages(l: *mut list_head, new: new_folio_t, free: free_folio_t,
                         private: c_ulong, mode: migrate_mode, reason: migrate_reason,
                         ret_succeeded: *mut c_uint) -> c_int;
    pub fn alloc_migration_target(src: *mut folio, private: c_ulong) -> *mut folio;
    pub fn isolate_movable_ops_page(page: *mut page, mode: isolate_mode_t) -> bool;
    pub fn isolate_folio_to_list(folio: *mut folio, list: *mut list_head) -> bool;
    pub fn migrate_huge_page_move_mapping(mapping: *mut address_space, dst: *mut folio,
                                          src: *mut folio) -> c_int;
    pub fn softleaf_entry_wait_on_locked(entry: softleaf_t, ptl: *mut spinlock_t);
    pub fn folio_migrate_flags(newfolio: *mut folio, folio: *mut folio);
    pub fn folio_migrate_mapping(mapping: *mut address_space, newfolio: *mut folio,
                                 folio: *mut folio, extra_count: c_int) -> c_int;
    pub fn set_movable_ops(ops: *const movable_operations, type_: pagetype) -> c_int;
}

#[cfg(not(feature = "CONFIG_MIGRATION"))]
pub unsafe fn putback_movable_pages(_l: *mut list_head) {}

#[cfg(not(feature = "CONFIG_MIGRATION"))]
pub unsafe fn migrate_pages(_l: *mut list_head, _new: new_folio_t, _free: free_folio_t,
                            _private: c_ulong, _mode: migrate_mode, _reason: migrate_reason,
                            _ret_succeeded: *mut c_uint) -> c_int { -ENOSYS }

#[cfg(not(feature = "CONFIG_MIGRATION"))]
pub unsafe fn alloc_migration_target(_src: *mut folio, _private: c_ulong) -> *mut folio { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_MIGRATION"))]
pub unsafe fn isolate_movable_ops_page(_page: *mut page, _mode: isolate_mode_t) -> bool { false }

#[cfg(not(feature = "CONFIG_MIGRATION"))]
pub unsafe fn isolate_folio_to_list(_folio: *mut folio, _list: *mut list_head) -> bool { false }

#[cfg(not(feature = "CONFIG_MIGRATION"))]
pub unsafe fn migrate_huge_page_move_mapping(_mapping: *mut address_space, _dst: *mut folio,
                                             _src: *mut folio) -> c_int { -ENOSYS }

#[cfg(not(feature = "CONFIG_MIGRATION"))]
pub unsafe fn set_movable_ops(_ops: *const movable_operations, _type: pagetype) -> c_int { -ENOSYS }

#[cfg(not(feature = "CONFIG_MIGRATION"))]
pub unsafe fn softleaf_entry_wait_on_locked(_entry: softleaf_t, ptl: *mut spinlock_t) {
    WARN_ON_ONCE(1);
    spin_unlock(ptl);
}

#[cfg(feature = "CONFIG_NUMA_BALANCING")]
extern "C" {
    pub fn migrate_misplaced_folio_prepare(folio: *mut folio, vma: *mut vm_area_struct,
                                            node: c_int) -> c_int;
    pub fn migrate_misplaced_folio(folio: *mut folio, node: c_int) -> c_int;
}

#[cfg(not(feature = "CONFIG_NUMA_BALANCING"))]
pub unsafe fn migrate_misplaced_folio_prepare(_folio: *mut folio, _vma: *mut vm_area_struct,
                                              _node: c_int) -> c_int { -EAGAIN }

#[cfg(not(feature = "CONFIG_NUMA_BALANCING"))]
pub unsafe fn migrate_misplaced_folio(_folio: *mut folio, _node: c_int) -> c_int { -EAGAIN }

#[cfg(feature = "CONFIG_MIGRATION")]
pub const MIGRATE_PFN_VALID: c_ulong = 1UL << 0;
#[cfg(feature = "CONFIG_MIGRATION")]
pub const MIGRATE_PFN_MIGRATE: c_ulong = 1UL << 1;
#[cfg(feature = "CONFIG_MIGRATION")]
pub const MIGRATE_PFN_WRITE: c_ulong = 1UL << 3;
#[cfg(feature = "CONFIG_MIGRATION")]
pub const MIGRATE_PFN_COMPOUND: c_ulong = 1UL << 4;
#[cfg(feature = "CONFIG_MIGRATION")]
pub const MIGRATE_PFN_SHIFT: c_uint = 6;

#[cfg(feature = "CONFIG_MIGRATION")]
pub unsafe fn migrate_pfn_to_page(mpfn: c_ulong) -> *mut page {
    if (mpfn & MIGRATE_PFN_VALID) == 0 { return core::ptr::null_mut(); }
    pfn_to_page(mpfn >> MIGRATE_PFN_SHIFT)
}

#[cfg(feature = "CONFIG_MIGRATION")]
pub const fn migrate_pfn(pfn: c_ulong) -> c_ulong {
    (pfn << MIGRATE_PFN_SHIFT) | MIGRATE_PFN_VALID
}

#[repr(C)]
pub enum migrate_vma_direction {
    MIGRATE_VMA_SELECT_SYSTEM = 1 << 0,
    MIGRATE_VMA_SELECT_DEVICE_PRIVATE = 1 << 1,
    MIGRATE_VMA_SELECT_DEVICE_COHERENT = 1 << 2,
    MIGRATE_VMA_SELECT_COMPOUND = 1 << 3,
}

#[repr(C)]
pub struct migrate_vma {
    pub vma: *mut vm_area_struct,
    pub dst: *mut c_ulong,
    pub src: *mut c_ulong,
    pub cpages: c_ulong,
    pub npages: c_ulong,
    pub start: c_ulong,
    pub end: c_ulong,
    pub pgmap_owner: *mut c_void,
    pub flags: c_ulong,
    pub fault_page: *mut page,
}

#[cfg(feature = "CONFIG_MIGRATION")]
extern "C" {
    pub fn migrate_vma_setup(args: *mut migrate_vma) -> c_int;
    pub fn migrate_vma_pages(migrate: *mut migrate_vma);
    pub fn migrate_vma_finalize(migrate: *mut migrate_vma);
    pub fn migrate_device_range(src_pfns: *mut c_ulong, start: c_ulong, npages: c_ulong) -> c_int;
    pub fn migrate_device_pfns(src_pfns: *mut c_ulong, npages: c_ulong) -> c_int;
    pub fn migrate_device_pages(src_pfns: *mut c_ulong, dst_pfns: *mut c_ulong, npages: c_ulong);
    pub fn migrate_device_finalize(src_pfns: *mut c_ulong, dst_pfns: *mut c_ulong,
                                   npages: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
