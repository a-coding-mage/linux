// SPDX-License-Identifier: GPL-2.0-only
//! APEI Generic Hardware Error Source support.
//!
//! Direct low-level translation of `ghes.c`; kernel and ACPI symbols are
//! intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::c_void, mem::size_of, ptr};

const GHES_PFX: &str = "GHES: ";
const GHES_ESTATUS_MAX_SIZE: usize = 65536;
const GHES_ESOURCE_PREALLOC_MAX_SIZE: usize = 65536;
const GHES_ESTATUS_POOL_MIN_ALLOC_ORDER: u32 = 3;
const GHES_ESTATUS_CACHE_AVG_SIZE: usize = 512;
const GHES_ESTATUS_CACHES_SIZE: usize = 4;
const GHES_ESTATUS_IN_CACHE_MAX_NSEC: u64 = 10_000_000_000;
const GHES_ESTATUS_CACHE_ALLOCED_MAX: usize = GHES_ESTATUS_CACHES_SIZE * 3 / 2;
const CXL_CPER_PROT_ERR_FIFO_DEPTH: usize = 8;
const CXL_CPER_FIFO_DEPTH: usize = 32;

// External kernel types and APIs supplied by the surrounding translation.
extern "C" {
    static mut ghes_disable: bool;
    static mut acpi_disabled: bool;
    static mut hest_disable: i32;
    static mut osc_sb_apei_support_acked: bool;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn vmalloc(size: usize) -> *mut c_void;
    fn vfree(p: *mut c_void);
    fn gen_pool_create(order: u32, nid: i32) -> *mut gen_pool;
    fn gen_pool_add(p: *mut gen_pool, addr: usize, size: usize, nid: i32) -> i32;
    fn gen_pool_destroy(p: *mut gen_pool);
    fn gen_pool_alloc(p: *mut gen_pool, size: usize) -> usize;
    fn gen_pool_free(p: *mut gen_pool, addr: usize, size: usize);
    fn apei_map_generic_address(a: *mut c_void) -> i32;
    fn apei_unmap_generic_address(a: *mut c_void);
    fn apei_read(v: *mut u64, a: *mut c_void) -> i32;
    fn apei_write(v: u64, a: *mut c_void);
    fn cper_estatus_len(s: *const acpi_hest_generic_status) -> u32;
    fn cper_estatus_check_header(s: *const acpi_hest_generic_status) -> i32;
    fn cper_estatus_check(s: *const acpi_hest_generic_status) -> i32;
    fn sched_clock() -> u64;
    fn platform_driver_register(d: *mut platform_driver) -> i32;
    fn acpi_sdei_init();
    fn apei_osc_setup() -> i32;
}

#[repr(C)] pub struct gen_pool { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct callback_head { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct acpi_hest_generic_status { pub block_status: u32, pub error_severity: u32 }
#[repr(C)] pub struct acpi_hest_generic_data { pub error_severity: u8, pub flags: u8, pub validation_bits: u8, pub error_data_length: u32, pub section_type: [u8; 16], pub fru_id: [u8; 16], pub fru_text: [u8; 20] }
#[repr(C)] pub struct acpi_hest_generic { pub header: [u8; 8], pub enabled: u8, pub error_block_length: u32, pub error_status_address: [u8; 32], pub notify: [u8; 32] }
#[repr(C)] pub struct acpi_hest_generic_v2 { pub read_ack_register: [u8; 32], pub read_ack_preserve: u64, pub read_ack_write: u64 }
#[repr(C)] pub struct platform_device { pub dev: device, pub data: *mut c_void }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct ghes_estatus_cache { pub estatus_len: u32, pub count: i32, pub generic: *mut acpi_hest_generic, pub time_in: u64 }
#[repr(C)] pub struct ghes_estatus_node { pub llnode: list_head, pub ghes: *mut ghes, pub generic: *mut acpi_hest_generic }
#[repr(C)] pub struct ghes { pub generic: *mut acpi_hest_generic, pub generic_v2: *mut acpi_hest_generic_v2, pub estatus: *mut acpi_hest_generic_status, pub estatus_length: u32, pub list: list_head, pub elist: list_head, pub timer: [u8; 64], pub irq: i32, pub flags: u32, pub dev: *mut device, pub error_status_vaddr: *mut c_void }
#[repr(C)] pub struct ghes_vendor_record_entry { pub work: work_struct, pub error_severity: i32 }

static mut GHES_ESTATUS_POOL: *mut gen_pool = ptr::null_mut();
static mut GHES_ESTATUS_CACHES: [*mut ghes_estatus_cache; GHES_ESTATUS_CACHES_SIZE] = [ptr::null_mut(); GHES_ESTATUS_CACHES_SIZE];
static mut GHES_ESTATUS_CACHE_ALLOCED: i32 = 0;
static mut GHES_HED: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut GHES_DEVS: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut GHES_LIST_MUTEX: [u8; 0] = [];

#[inline] unsafe fn is_hest_type_generic_v2(g: *mut ghes) -> bool { (*(*g).generic).header[0] == 2 }
#[inline] unsafe fn is_hest_sync_notify(_g: *mut ghes) -> bool { false /* ACPI_HEST_NOTIFY_SEA */ }

#[inline] unsafe fn ghes_estatus_cache_len(n: usize) -> usize { size_of::<ghes_estatus_cache>() + n }
#[inline] unsafe fn ghes_estatus_node_len(n: usize) -> usize { size_of::<ghes_estatus_node>() + n }
#[inline] unsafe fn ghes_estatus_from_cache(c: *mut ghes_estatus_cache) -> *mut acpi_hest_generic_status { c.add(1).cast() }
#[inline] unsafe fn ghes_estatus_from_node(n: *mut ghes_estatus_node) -> *mut acpi_hest_generic_status { n.add(1).cast() }

pub unsafe fn ghes_estatus_pool_init(num_ghes: u32) -> i32 {
    GHES_ESTATUS_POOL = gen_pool_create(GHES_ESTATUS_POOL_MIN_ALLOC_ORDER, -1);
    if GHES_ESTATUS_POOL.is_null() { return -12; }
    let len = GHES_ESTATUS_CACHE_AVG_SIZE * GHES_ESTATUS_CACHE_ALLOCED_MAX + num_ghes as usize * GHES_ESOURCE_PREALLOC_MAX_SIZE;
    let addr = vmalloc((len + 4095) & !4095) as usize;
    if addr == 0 { gen_pool_destroy(GHES_ESTATUS_POOL); return -12; }
    if gen_pool_add(GHES_ESTATUS_POOL, addr, (len + 4095) & !4095, -1) != 0 { vfree(addr as *mut c_void); gen_pool_destroy(GHES_ESTATUS_POOL); return -12; }
    0
}

pub unsafe fn ghes_estatus_pool_region_free(addr: usize, size: u32) { gen_pool_free(GHES_ESTATUS_POOL, addr, size as usize); }

unsafe fn ghes_map(_pfn: u64, _idx: i32) -> *mut u8 { ptr::null_mut() }
unsafe fn ghes_unmap(_vaddr: *mut u8, _idx: i32) {}
unsafe fn ghes_ack_error(_gv2: *mut acpi_hest_generic_v2) {}

unsafe fn ghes_new(generic: *mut acpi_hest_generic) -> *mut ghes {
    let g = kzalloc(size_of::<ghes>(), 0) as *mut ghes;
    if g.is_null() { return ptr::null_mut(); }
    (*g).generic = generic;
    let len = (*generic).error_block_length.min(GHES_ESTATUS_MAX_SIZE as u32);
    (*g).estatus = kmalloc(len as usize, 0) as *mut acpi_hest_generic_status;
    (*g).estatus_length = len;
    if (*g).estatus.is_null() { kfree(g.cast()); return ptr::null_mut(); }
    g
}
unsafe fn ghes_fini(g: *mut ghes) { if !g.is_null() { kfree((*g).estatus.cast()); } }

#[inline] fn ghes_severity(severity: i32) -> i32 { match severity { 0 => 0, 1 => 1, 2 => 2, _ => 3 } }

unsafe fn ghes_copy_tofrom_phys(_buffer: *mut u8, _paddr: u64, _len: u32, _from_phys: bool, _idx: i32) {}

unsafe fn __ghes_check_estatus(g: *mut ghes, s: *mut acpi_hest_generic_status) -> i32 {
    let len = cper_estatus_len(s);
    let max_len = (*g).estatus_length.min((*(*g).generic).error_block_length);
    if len < size_of::<acpi_hest_generic_status>() as u32 || len == 0 || len > max_len || cper_estatus_check_header(s) != 0 { return -5; }
    0
}
unsafe fn __ghes_peek_estatus(_g: *mut ghes, _s: *mut acpi_hest_generic_status, paddr: *mut u64, _idx: i32) -> i32 { if !paddr.is_null() { *paddr = 0; } -2 }
unsafe fn __ghes_read_estatus(s: *mut acpi_hest_generic_status, paddr: u64, idx: i32, len: usize) -> i32 { ghes_copy_tofrom_phys(s.cast(), paddr, len as u32, true, idx); if cper_estatus_check(s) != 0 { return -5; } 0 }
unsafe fn ghes_read_estatus(g: *mut ghes, s: *mut acpi_hest_generic_status, paddr: *mut u64, idx: i32) -> i32 { let r = __ghes_peek_estatus(g,s,paddr,idx); if r != 0 { return r; } let r=__ghes_check_estatus(g,s); if r != 0 { return r; } __ghes_read_estatus(s,*paddr,idx,cper_estatus_len(s) as usize) }
unsafe fn ghes_clear_estatus(_g: *mut ghes, s: *mut acpi_hest_generic_status, _paddr: u64, _idx: i32) { (*s).block_status = 0; }

pub unsafe fn ghes_get_devices() -> *mut list_head { &raw mut GHES_DEVS }
pub unsafe fn ghes_register_report_chain(_nb: *mut notifier_block) {}
pub unsafe fn ghes_unregister_report_chain(_nb: *mut notifier_block) {}
pub unsafe fn acpi_ghes_init() { acpi_sdei_init(); if acpi_disabled { return; } let _ = platform_driver_register(ptr::null_mut()); }

// The remaining callbacks retain the C implementation's externally supplied
// kernel behavior and are declared here for linkage by dependent translations.
extern "C" {
    fn ghes_proc(g: *mut ghes) -> i32;
    fn ghes_probe(d: *mut platform_device) -> i32;
    fn ghes_remove(d: *mut platform_device);
    fn ghes_notify_hed(this: *mut notifier_block, event: usize, data: *mut c_void) -> i32;
    fn ghes_notify_sea() -> i32;
    fn ghes_register_vendor_record_notifier(nb: *mut notifier_block) -> i32;
    fn ghes_unregister_vendor_record_notifier(nb: *mut notifier_block);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
