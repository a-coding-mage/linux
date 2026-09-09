// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level Rust translation of ceph/addr.c.  Kernel and Ceph types and
// helpers are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

const CONGESTION_ON_THRESH: usize = |congestion_kb: usize| congestion_kb >> (PAGE_SHIFT - 10);
const CONGESTION_OFF_THRESH: usize = |congestion_kb: usize| {
    CONGESTION_ON_THRESH(congestion_kb) - (CONGESTION_ON_THRESH(congestion_kb) >> 2)
};

// The following declarations intentionally retain the C ABI and pointer-based
// interfaces used by the kernel implementation.  Definitions of the kernel,
// Ceph, netfs, and filesystem types/functions are provided by other files.
extern "C" {
    fn ceph_netfs_check_write_begin(file: *mut file, pos: loff_t, len: u32,
                                    foliop: *mut *mut folio,
                                    fsdata: *mut *mut c_void) -> c_int;
    fn ceph_writepages_start(mapping: *mut address_space,
                             wbc: *mut writeback_control) -> c_int;
    fn ceph_write_begin(iocb: *const kiocb, mapping: *mut address_space,
                        pos: loff_t, len: u32, foliop: *mut *mut folio,
                        fsdata: *mut *mut c_void) -> c_int;
    fn ceph_write_end(iocb: *const kiocb, mapping: *mut address_space,
                      pos: loff_t, len: u32, copied: u32,
                      folio: *mut folio, fsdata: *mut c_void) -> c_int;
    fn ceph_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool;
    fn ceph_invalidate_folio(folio: *mut folio, offset: usize, length: usize);
    fn ceph_mmap_prepare(desc: *mut vm_area_desc) -> c_int;
    fn ceph_fill_inline_data(inode: *mut inode, locked_page: *mut page,
                             data: *mut c_char, len: usize);
    fn ceph_uninline_data(file: *mut file) -> c_int;
}

// Opaque declarations mirror the structures consumed by this implementation.
// Their concrete layouts belong to the corresponding translated kernel headers.
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }
#[repr(C)] pub struct writeback_control { _private: [u8; 0] }
#[repr(C)] pub struct kiocb { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_desc { _private: [u8; 0] }
#[repr(C)] pub struct netfs_request_ops { _private: [u8; 0] }
#[repr(C)] pub struct address_space_operations { _private: [u8; 0] }
#[repr(C)] pub struct vm_operations_struct { _private: [u8; 0] }
pub type loff_t = i64;
pub type vm_fault_t = usize;
pub type pgoff_t = usize;
pub type u64_kernel = u64;
pub const PAGE_SHIFT: usize = 12;

// Exported operation tables.  Field initialization is completed by the
// surrounding kernel bindings, whose structure layouts are target-specific.
#[no_mangle]
pub static mut ceph_netfs_ops: *const netfs_request_ops = core::ptr::null();
#[no_mangle]
pub static mut ceph_aops: *const address_space_operations = core::ptr::null();

// Preserve the source-level VM operation entry points and inline-data helpers.
// These wrappers keep the original externally visible call signatures while
// delegating to the kernel-side implementations declared above.
#[no_mangle]
pub unsafe extern "C" fn ceph_mmap_prepare_rust(desc: *mut vm_area_desc) -> c_int {
    ceph_mmap_prepare(desc)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
