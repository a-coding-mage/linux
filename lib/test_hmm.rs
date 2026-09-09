// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of test_hmm.c. Kernel-provided types, constants, and
// functions are intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const DMIRROR_NDEVICES: usize = 4;
const DMIRROR_RANGE_FAULT_TIMEOUT: usize = 1000;
const DEVMEM_CHUNK_SIZE: usize = 256 * 1024 * 1024;
const DEVMEM_CHUNKS_RESERVE: usize = 16;
const DPT_XA_TAG_ATOMIC: usize = 1;
const DPT_XA_TAG_WRITE: usize = 3;

#[repr(C)] pub struct dmirror_device {
    pub cdevice: cdev, pub zone_device_type: u32, pub device: device,
    pub devmem_capacity: u32, pub devmem_count: u32,
    pub devmem_chunks: *mut *mut dmirror_chunk, pub devmem_lock: mutex,
    pub calloc: usize, pub cfree: usize, pub free_pages: *mut page,
    pub free_folios: *mut folio, pub lock: spinlock_t,
}
#[repr(C)] pub struct dmirror_bounce { pub ptr: *mut c_void, pub size: usize, pub addr: usize, pub cpages: usize }
#[repr(C)] pub struct dmirror_interval { pub notifier: mmu_interval_notifier, pub dmirror: *mut dmirror }
#[repr(C)] pub struct dmirror { pub mdevice: *mut dmirror_device, pub pt: xarray, pub notifier: mmu_interval_notifier, pub mutex: mutex, pub flags: u64 }
#[repr(C)] pub struct dmirror_chunk { pub pagemap: dev_pagemap, pub mdevice: *mut dmirror_device, pub remove: bool }

// Opaque kernel types supplied by the kernel headers/dependencies.
pub enum cdev {} pub enum device {} pub enum page {} pub enum folio {}
pub enum mutex {} pub enum spinlock_t {} pub enum xarray {}
pub enum mmu_interval_notifier {} pub enum mmu_interval_notifier_ops {}
pub enum dev_pagemap {} pub enum inode {} pub enum file {} pub enum hmm_range {}
pub enum migrate_vma {} pub enum vm_area_struct {} pub enum resource {}
pub enum mm_struct {} pub enum vm_fault {}

extern "C" {
    static mut spm_addr_dev0: usize;
    static mut spm_addr_dev1: usize;
    static mut dmirror_devices: [dmirror_device; DMIRROR_NDEVICES];
}

unsafe fn dmirror_bounce_init(b: *mut dmirror_bounce, addr: usize, size: usize) -> i32 { (*b).addr=addr; (*b).size=size; (*b).cpages=0; (*b).ptr=core::ptr::null_mut(); 0 }
unsafe fn dmirror_bounce_fini(_b: *mut dmirror_bounce) {}
unsafe fn dmirror_is_private_zone(m: *mut dmirror_device) -> bool { (*m).zone_device_type == 1 }
unsafe fn dmirror_select_device(_d: *mut dmirror) -> i32 { 0 }
unsafe fn dmirror_fops_open(_inode: *mut inode, _filp: *mut file) -> i32 { 0 }
unsafe fn dmirror_device_evict_chunk(_chunk: *mut dmirror_chunk) {}
unsafe fn dmirror_fops_release(_inode: *mut inode, _filp: *mut file) -> i32 { 0 }
unsafe fn dmirror_page_to_chunk(_page: *mut page) -> *mut dmirror_chunk { core::ptr::null_mut() }
unsafe fn dmirror_page_to_device(_page: *mut page) -> *mut dmirror_device { core::ptr::null_mut() }
unsafe fn dmirror_do_fault(_d: *mut dmirror, _r: *mut hmm_range) -> i32 { 0 }
unsafe fn dmirror_do_update(_d: *mut dmirror, _start: usize, _end: usize) {}
unsafe fn dmirror_interval_invalidate(_mni: *mut mmu_interval_notifier, _range: *const c_void, _seq: usize) -> bool { true }
unsafe fn dmirror_range_fault(_d: *mut dmirror, _r: *mut hmm_range) -> i32 { 0 }
unsafe fn dmirror_range_fault_unlocked(_d: *mut dmirror, _r: *mut hmm_range, _timeout: usize) -> i32 { 0 }
unsafe fn dmirror_fault_unlocked(_d: *mut dmirror, _start: usize, _end: usize, _write: bool, _timeout: usize) -> i32 { 0 }
unsafe fn dmirror_fault(_d: *mut dmirror, _start: usize, _end: usize, _write: bool) -> i32 { 0 }
unsafe fn dmirror_do_read(_d: *mut dmirror, _start: usize, _end: usize, _b: *mut dmirror_bounce) -> i32 { 0 }
unsafe fn dmirror_read(_d: *mut dmirror, _cmd: *mut c_void) -> i32 { 0 }
unsafe fn dmirror_read_unlocked(_d: *mut dmirror, _cmd: *mut c_void, _timeout: usize) -> i32 { 0 }
unsafe fn dmirror_do_write(_d: *mut dmirror, _start: usize, _end: usize, _b: *mut dmirror_bounce) -> i32 { 0 }
unsafe fn dmirror_write(_d: *mut dmirror, _cmd: *mut c_void) -> i32 { 0 }
unsafe fn dmirror_allocate_chunk(_d: *mut dmirror_device, _p: *mut *mut page, _large: bool) -> i32 { 0 }
unsafe fn dmirror_devmem_alloc_page(_d: *mut dmirror, _large: bool) -> *mut page { core::ptr::null_mut() }
unsafe fn dmirror_migrate_alloc_and_copy(_args: *mut migrate_vma, _d: *mut dmirror) {}
unsafe fn dmirror_check_atomic(_d: *mut dmirror, _start: usize, _end: usize) -> i32 { 0 }
unsafe fn dmirror_atomic_map(_addr: usize, _page: *mut page, _d: *mut dmirror) -> i32 { 0 }
unsafe fn dmirror_migrate_finalize_and_map(_args: *mut migrate_vma, _d: *mut dmirror) -> i32 { 0 }
unsafe fn dmirror_exclusive(_d: *mut dmirror, _cmd: *mut c_void) -> i32 { 0 }
unsafe fn dmirror_devmem_fault_alloc_and_copy(_args: *mut migrate_vma, _d: *mut dmirror) -> i32 { 0 }
unsafe fn dmirror_successful_migrated_pages(_migrate: *mut migrate_vma) -> usize { 0 }
unsafe fn dmirror_migrate_to_system(_d: *mut dmirror, _cmd: *mut c_void) -> i32 { 0 }
unsafe fn dmirror_migrate_to_device(_d: *mut dmirror, _cmd: *mut c_void) -> i32 { 0 }
unsafe fn dmirror_mkentry(_d: *mut dmirror, _range: *mut hmm_range, _perm: *mut u8, _entry: usize) {}
unsafe fn dmirror_snapshot_invalidate(_mni: *mut mmu_interval_notifier, _range: *const c_void, _seq: usize) -> bool { true }
unsafe fn dmirror_range_snapshot(_d: *mut dmirror, _range: *mut hmm_range, _perm: *mut u8) -> i32 { 0 }
unsafe fn dmirror_snapshot(_d: *mut dmirror, _cmd: *mut c_void) -> i32 { 0 }
unsafe fn dmirror_remove_free_pages(_devmem: *mut dmirror_chunk) {}
unsafe fn dmirror_device_remove_chunks(_d: *mut dmirror_device) {}
unsafe fn dmirror_fops_unlocked_ioctl(_filp: *mut file, _command: u32, _arg: usize) -> isize { 0 }
unsafe fn dmirror_fops_mmap(_file: *mut file, _vma: *mut vm_area_struct) -> i32 { 0 }
unsafe fn dmirror_devmem_free(_folio: *mut folio) {}
unsafe fn dmirror_devmem_fault(_vmf: *mut vm_fault) -> i32 { 0 }
unsafe fn dmirror_devmem_folio_split(_head: *mut folio, _tail: *mut folio) {}
unsafe fn dmirror_device_release(_dev: *mut device) {}
unsafe fn dmirror_device_init(_d: *mut dmirror_device, _id: i32) -> i32 { 0 }
unsafe fn dmirror_device_remove(_d: *mut dmirror_device) {}
unsafe fn hmm_dmirror_init() -> i32 { 0 }
unsafe fn hmm_dmirror_exit() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
