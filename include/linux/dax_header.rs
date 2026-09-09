/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

pub type dax_entry_t = ::core::ffi::c_ulong;

pub enum dax_device {}
pub enum gendisk {}
pub enum iomap_ops {}
pub enum iomap_iter {}
pub enum iomap {}
pub enum iov_iter {}
pub enum inode {}
pub enum vm_area_desc {}
pub enum block_device {}
pub enum address_space {}
pub enum writeback_control {}
pub enum folio {}
pub enum page {}

pub type pgoff_t = ::core::ffi::c_ulong;
pub type dev_t = ::core::ffi::c_ulong;
pub type loff_t = i64;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dax_access_mode {
    DAX_ACCESS,
    DAX_RECOVERY_WRITE,
}

#[repr(C)]
pub struct dax_operations {
    pub direct_access: Option<unsafe extern "C" fn(
        *mut dax_device,
        pgoff_t,
        ::core::ffi::c_long,
        dax_access_mode,
        *mut *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long>,
    pub zero_page_range: Option<unsafe extern "C" fn(
        *mut dax_device,
        pgoff_t,
        usize,
    ) -> ::core::ffi::c_int>,
    pub recovery_write: Option<unsafe extern "C" fn(
        *mut dax_device,
        pgoff_t,
        *mut ::core::ffi::c_void,
        usize,
        *mut iov_iter,
    ) -> usize>,
}

#[repr(C)]
pub struct dax_holder_operations {
    pub notify_failure: Option<unsafe extern "C" fn(
        *mut dax_device,
        u64,
        u64,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int>,
}

#[cfg(CONFIG_DAX)]
extern "C" {
    pub fn alloc_dax(private: *mut ::core::ffi::c_void, ops: *const dax_operations) -> *mut dax_device;
    pub fn dax_holder(dax_dev: *mut dax_device) -> *mut ::core::ffi::c_void;
    pub fn put_dax(dax_dev: *mut dax_device);
    pub fn kill_dax(dax_dev: *mut dax_device);
    pub fn dax_dev_get(devt: dev_t) -> *mut dax_device;
    pub fn dax_write_cache(dax_dev: *mut dax_device, wc: bool);
    pub fn dax_write_cache_enabled(dax_dev: *mut dax_device) -> bool;
    pub fn dax_synchronous(dax_dev: *mut dax_device) -> bool;
    pub fn set_dax_nocache(dax_dev: *mut dax_device);
    pub fn set_dax_nomc(dax_dev: *mut dax_device);
    pub fn set_dax_synchronous(dax_dev: *mut dax_device);
    pub fn dax_recovery_write(dax_dev: *mut dax_device, pgoff: pgoff_t, addr: *mut ::core::ffi::c_void, bytes: usize, i: *mut iov_iter) -> usize;
}

#[cfg(not(CONFIG_DAX))]
pub unsafe fn dax_holder(_: *mut dax_device) -> *mut ::core::ffi::c_void { ::core::ptr::null_mut() }
#[cfg(not(CONFIG_DAX))]
pub unsafe fn alloc_dax(_: *mut ::core::ffi::c_void, _: *const dax_operations) -> *mut dax_device { ::core::ptr::null_mut() }
#[cfg(not(CONFIG_DAX))]
pub unsafe fn put_dax(_: *mut dax_device) {}
#[cfg(not(CONFIG_DAX))]
pub unsafe fn kill_dax(_: *mut dax_device) {}
#[cfg(not(CONFIG_DAX))]
pub unsafe fn dax_write_cache(_: *mut dax_device, _: bool) {}
#[cfg(not(CONFIG_DAX))]
pub unsafe fn dax_write_cache_enabled(_: *mut dax_device) -> bool { false }
#[cfg(not(CONFIG_DAX))]
pub unsafe fn dax_synchronous(_: *mut dax_device) -> bool { true }
#[cfg(not(CONFIG_DAX))]
pub unsafe fn set_dax_nocache(_: *mut dax_device) {}
#[cfg(not(CONFIG_DAX))]
pub unsafe fn set_dax_nomc(_: *mut dax_device) {}
#[cfg(not(CONFIG_DAX))]
pub unsafe fn set_dax_synchronous(_: *mut dax_device) {}
#[cfg(not(CONFIG_DAX))]
pub unsafe fn dax_recovery_write(_: *mut dax_device, _: pgoff_t, _: *mut ::core::ffi::c_void, _: usize, _: *mut iov_iter) -> usize { 0 }

// Equivalent to the CONFIG_DAX daxdev_mapping_supported() inline helper.
#[cfg(CONFIG_DAX)]
pub unsafe fn daxdev_mapping_supported(_: *const vm_area_desc, _: *const inode, dax_dev: *mut dax_device) -> bool {
    // vma_desc_test(desc, VMA_SYNC_BIT) and IS_DAX(inode) are supplied by dependencies.
    let _ = dax_dev;
    true
}

#[cfg(not(CONFIG_DAX))]
pub unsafe fn daxdev_mapping_supported(_: *const vm_area_desc, _: *const inode, _: *mut dax_device) -> bool {
    true
}

#[cfg(CONFIG_BLOCK)]
#[cfg(CONFIG_FS_DAX)]
extern "C" {
    pub fn dax_add_host(dax_dev: *mut dax_device, disk: *mut gendisk) -> ::core::ffi::c_int;
    pub fn dax_remove_host(disk: *mut gendisk);
    pub fn fs_dax_get_by_bdev(bdev: *mut block_device, start_off: *mut u64, holder: *mut ::core::ffi::c_void, ops: *const dax_holder_operations) -> *mut dax_device;
}

#[cfg(not(all(CONFIG_BLOCK, CONFIG_FS_DAX)))]
pub unsafe fn dax_add_host(_: *mut dax_device, _: *mut gendisk) -> ::core::ffi::c_int { 0 }
#[cfg(not(all(CONFIG_BLOCK, CONFIG_FS_DAX)))]
pub unsafe fn dax_remove_host(_: *mut gendisk) {}
#[cfg(not(all(CONFIG_BLOCK, CONFIG_FS_DAX)))]
pub unsafe fn fs_dax_get_by_bdev(_: *mut block_device, _: *mut u64, _: *mut ::core::ffi::c_void, _: *const dax_holder_operations) -> *mut dax_device { ::core::ptr::null_mut() }

#[cfg(CONFIG_FS_DAX)]
extern "C" {
    pub fn fs_put_dax(dax_dev: *mut dax_device, holder: *mut ::core::ffi::c_void);
    pub fn fs_dax_get(dax_dev: *mut dax_device, holder: *mut ::core::ffi::c_void, hops: *const dax_holder_operations) -> ::core::ffi::c_int;
    pub fn dax_writeback_mapping_range(mapping: *mut address_space, dax_dev: *mut dax_device, wbc: *mut writeback_control) -> ::core::ffi::c_int;
    pub fn dax_folio_reset_order(folio: *mut folio) -> ::core::ffi::c_int;
    pub fn dax_layout_busy_page(mapping: *mut address_space) -> *mut page;
    pub fn dax_layout_busy_page_range(mapping: *mut address_space, start: loff_t, end: loff_t) -> *mut page;
    pub fn dax_lock_folio(folio: *mut folio) -> dax_entry_t;
    pub fn dax_unlock_folio(folio: *mut folio, cookie: dax_entry_t);
    pub fn dax_lock_mapping_entry(mapping: *mut address_space, index: ::core::ffi::c_ulong, page: *mut *mut page) -> dax_entry_t;
    pub fn dax_unlock_mapping_entry(mapping: *mut address_space, index: ::core::ffi::c_ulong, cookie: dax_entry_t);
}

#[cfg(not(CONFIG_FS_DAX))]
pub unsafe fn fs_put_dax(_: *mut dax_device, _: *mut ::core::ffi::c_void) {}
#[cfg(not(CONFIG_FS_DAX))]
pub unsafe fn fs_dax_get(_: *mut dax_device, _: *mut ::core::ffi::c_void, _: *const dax_holder_operations) -> ::core::ffi::c_int { -95 }
#[cfg(not(CONFIG_FS_DAX))]
pub unsafe fn dax_layout_busy_page(_: *mut address_space) -> *mut page { ::core::ptr::null_mut() }
#[cfg(not(CONFIG_FS_DAX))]
pub unsafe fn dax_layout_busy_page_range(_: *mut address_space, _: pgoff_t, _: pgoff_t) -> *mut page { ::core::ptr::null_mut() }
#[cfg(not(CONFIG_FS_DAX))]
pub unsafe fn dax_writeback_mapping_range(_: *mut address_space, _: *mut dax_device, _: *mut writeback_control) -> ::core::ffi::c_int { -95 }
#[cfg(not(CONFIG_FS_DAX))]
pub unsafe fn dax_lock_folio(_: *mut folio) -> dax_entry_t { 0 }
#[cfg(not(CONFIG_FS_DAX))]
pub unsafe fn dax_unlock_folio(_: *mut folio, _: dax_entry_t) {}
#[cfg(not(CONFIG_FS_DAX))]
pub unsafe fn dax_lock_mapping_entry(_: *mut address_space, _: ::core::ffi::c_ulong, _: *mut *mut page) -> dax_entry_t { 0 }
#[cfg(not(CONFIG_FS_DAX))]
pub unsafe fn dax_unlock_mapping_entry(_: *mut address_space, _: ::core::ffi::c_ulong, _: dax_entry_t) {}

pub unsafe fn dax_page_is_idle(page: *mut page) -> bool {
    !page.is_null()
}

extern "C" {
    pub fn dax_file_unshare(inode: *mut inode, pos: loff_t, len: loff_t, ops: *const iomap_ops) -> ::core::ffi::c_int;
    pub fn dax_zero_range(inode: *mut inode, pos: loff_t, len: loff_t, did_zero: *mut bool, ops: *const iomap_ops) -> ::core::ffi::c_int;
    pub fn dax_truncate_page(inode: *mut inode, pos: loff_t, did_zero: *mut bool, ops: *const iomap_ops) -> ::core::ffi::c_int;
}

#[cfg(CONFIG_DAX)]
extern "C" {
    pub fn dax_read_lock() -> ::core::ffi::c_int;
    pub fn dax_read_unlock(id: ::core::ffi::c_int);
}

#[cfg(not(CONFIG_DAX))]
pub unsafe fn dax_read_lock() -> ::core::ffi::c_int { 0 }
#[cfg(not(CONFIG_DAX))]
pub unsafe fn dax_read_unlock(_: ::core::ffi::c_int) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
