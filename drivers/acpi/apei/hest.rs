// SPDX-License-Identifier: GPL-2.0-only
/*
 * APEI Hardware Error Source Table support
 *
 * HEST describes error sources in detail; communicates operational
 * parameters (i.e. severity levels, masking bits, and threshold
 * values) to Linux as necessary. It also allows the BIOS to report
 * non-standard error sources to Linux (for example, chipset-specific
 * error registers).
 *
 * For more information about HEST, please refer to ACPI Specification
 * version 4.0, section 17.3.2.
 *
 * Copyright 2009 Intel Corp.
 *   Author: Huang Ying <ying.huang@intel.com>
 */

use core::ffi::c_void;

// External kernel/ACPI definitions are supplied by other translation units.
#[repr(C)] pub struct acpi_table_hest { pub header: acpi_table_header, pub error_source_count: u32 }
#[repr(C)] pub struct acpi_table_header { pub length: u32 }
#[repr(C)] pub struct acpi_hest_header { pub type_: u16, pub source_id: u16 }
#[repr(C)] pub struct acpi_hest_ia_corrected { pub header: acpi_hest_header, pub flags: u16, pub num_hardware_banks: u8 }
#[repr(C)] pub struct acpi_hest_ia_machine_check { pub header: acpi_hest_header, pub flags: u16, pub num_hardware_banks: u8 }
#[repr(C)] pub struct acpi_hest_ia_deferred_check { pub header: acpi_hest_header, pub flags: u16, pub num_hardware_banks: u8 }
#[repr(C)] pub struct acpi_hest_ia_error_bank;
#[repr(C)] pub struct acpi_hest_ia_nmi;
#[repr(C)] pub struct acpi_hest_aer_root;
#[repr(C)] pub struct acpi_hest_aer;
#[repr(C)] pub struct acpi_hest_aer_bridge;
#[repr(C)] pub struct acpi_hest_generic { pub header: acpi_hest_header, pub related_source_id: u16, pub enabled: u8 }
#[repr(C)] pub struct acpi_hest_generic_v2;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { pub platform_data: *mut c_void }

pub const ACPI_HEST_TYPE_RESERVED: usize = 9;
pub const ACPI_HEST_TYPE_IA32_CHECK: u16 = 0;
pub const ACPI_HEST_TYPE_IA32_CORRECTED_CHECK: u16 = 1;
pub const ACPI_HEST_TYPE_IA32_NMI: u16 = 2;
pub const ACPI_HEST_TYPE_AER_ROOT_PORT: u16 = 6;
pub const ACPI_HEST_TYPE_AER_ENDPOINT: u16 = 7;
pub const ACPI_HEST_TYPE_AER_BRIDGE: u16 = 8;
pub const ACPI_HEST_TYPE_GENERIC_ERROR: u16 = 9;
pub const ACPI_HEST_TYPE_GENERIC_ERROR_V2: u16 = 10;
pub const ACPI_HEST_TYPE_IA32_DEFERRED_CHECK: u16 = 3;
pub const ACPI_HEST_GHES_ASSIST: u16 = 1;
pub const HEST_DISABLED: i32 = 1;
pub const HEST_NOT_FOUND: i32 = 2;

pub static mut hest_disable: i32 = 0;
static mut hest_tab: *mut acpi_table_hest = core::ptr::null_mut();

struct Mces { cmc: *mut acpi_hest_ia_corrected, mc: *mut acpi_hest_ia_machine_check, dmc: *mut acpi_hest_ia_deferred_check }
static mut mces: Mces = Mces { cmc: core::ptr::null_mut(), mc: core::ptr::null_mut(), dmc: core::ptr::null_mut() };

static hest_esrc_len_tab: [i32; ACPI_HEST_TYPE_RESERVED] = [
    -1, -1, core::mem::size_of::<acpi_hest_ia_nmi>() as i32, 0, 0, 0,
    core::mem::size_of::<acpi_hest_aer_root>() as i32,
    core::mem::size_of::<acpi_hest_aer>() as i32,
];

unsafe fn is_generic_error(h: *mut acpi_hest_header) -> bool {
    (*h).type_ == ACPI_HEST_TYPE_GENERIC_ERROR || (*h).type_ == ACPI_HEST_TYPE_GENERIC_ERROR_V2
}

unsafe fn hest_esrc_len(h: *mut acpi_hest_header) -> i32 {
    let t = (*h).type_ as usize;
    if t >= ACPI_HEST_TYPE_RESERVED { return 0; }
    let mut len = hest_esrc_len_tab[t];
    if t == ACPI_HEST_TYPE_IA32_CORRECTED_CHECK {
        let p = h as *mut acpi_hest_ia_corrected; len = core::mem::size_of::<acpi_hest_ia_corrected>() as i32 + (*p).num_hardware_banks as i32 * core::mem::size_of::<acpi_hest_ia_error_bank>() as i32; mces.cmc = p;
    } else if t == ACPI_HEST_TYPE_IA32_CHECK {
        let p = h as *mut acpi_hest_ia_machine_check; len = core::mem::size_of::<acpi_hest_ia_machine_check>() as i32 + (*p).num_hardware_banks as i32 * core::mem::size_of::<acpi_hest_ia_error_bank>() as i32; mces.mc = p;
    } else if t == ACPI_HEST_TYPE_IA32_DEFERRED_CHECK {
        let p = h as *mut acpi_hest_ia_deferred_check; len = core::mem::size_of::<acpi_hest_ia_deferred_check>() as i32 + (*p).num_hardware_banks as i32 * core::mem::size_of::<acpi_hest_ia_error_bank>() as i32; mces.dmc = p;
    }
    len
}

unsafe fn is_ghes_assist_struct(h: *mut acpi_hest_header) -> bool {
    if !is_generic_error(h) { return false; }
    let id = (*(h as *mut acpi_hest_generic)).related_source_id;
    (mces.cmc != core::ptr::null_mut() && (*mces.cmc).flags & ACPI_HEST_GHES_ASSIST != 0 && id == (*mces.cmc).header.source_id) ||
    (mces.mc != core::ptr::null_mut() && (*mces.mc).flags & ACPI_HEST_GHES_ASSIST != 0 && id == (*mces.mc).header.source_id) ||
    (mces.dmc != core::ptr::null_mut() && (*mces.dmc).flags & ACPI_HEST_GHES_ASSIST != 0 && id == (*mces.dmc).header.source_id)
}

pub type apei_hest_func_t = unsafe extern "C" fn(*mut acpi_hest_header, *mut c_void) -> i32;

unsafe fn apei_hest_parse(func: apei_hest_func_t, data: *mut c_void) -> i32 {
    if hest_disable != 0 || hest_tab.is_null() { return -22; }
    let mut h = (hest_tab.add(1)) as *mut acpi_hest_header;
    for _ in 0..(*hest_tab).error_source_count {
        let len = hest_esrc_len(h); if len == 0 { return -22; }
        if (h as usize).wrapping_add(len as usize) > (hest_tab as usize).wrapping_add((*hest_tab).header.length as usize) { return -22; }
        if is_ghes_assist_struct(h) { h = (h as *mut u8).add(len as usize) as *mut acpi_hest_header; continue; }
        let rc = func(h, data); if rc != 0 { return rc; }
        h = (h as *mut u8).add(len as usize) as *mut acpi_hest_header;
    }
    0
}

unsafe extern "C" { fn arch_apei_enable_cmcff(h: *mut acpi_hest_header, data: *mut c_void) -> i32; fn platform_device_alloc(name: *const u8, id: u16) -> *mut platform_device; fn platform_device_put(d: *mut platform_device); fn platform_device_add_data(d: *mut platform_device, data: *const c_void, size: usize) -> i32; fn platform_device_add(d: *mut platform_device) -> i32; fn platform_device_unregister(d: *mut platform_device); fn kmalloc_array(n: usize, size: usize, flags: u32) -> *mut *mut platform_device; fn kfree(p: *mut *mut platform_device); fn ghes_estatus_pool_init(n: u32) -> i32; fn acpi_put_table(h: *mut acpi_table_header); }

#[repr(C)] struct ghes_arr { ghes_devs: *mut *mut platform_device, count: u32 }

unsafe extern "C" fn hest_parse_cmc(h: *mut acpi_hest_header, data: *mut c_void) -> i32 { if (*h).type_ != ACPI_HEST_TYPE_IA32_CORRECTED_CHECK { return 0; } arch_apei_enable_cmcff(h, data) }
unsafe extern "C" fn hest_parse_ghes_count(h: *mut acpi_hest_header, data: *mut c_void) -> i32 { if is_generic_error(h) { *(data as *mut i32) += 1; } 0 }

unsafe extern "C" fn hest_parse_ghes(h: *mut acpi_hest_header, data: *mut c_void) -> i32 {
    if !is_generic_error(h) || (*(h as *mut acpi_hest_generic)).enabled == 0 { return 0; }
    let a = &mut *(data as *mut ghes_arr);
    for i in 0..a.count {
        let d = *a.ghes_devs.add(i as usize);
        let old = *( (*d).dev.platform_data as *mut *mut acpi_hest_header);
        if (*old).source_id == (*h).source_id { return -5; }
    }
    let d = platform_device_alloc(b"GHES\0".as_ptr(), (*h).source_id);
    if d.is_null() { return -12; }
    let mut rc = platform_device_add_data(d, &h as *const _ as *const c_void, core::mem::size_of::<*mut acpi_hest_header>());
    if rc != 0 { platform_device_put(d); return rc; }
    rc = platform_device_add(d);
    if rc != 0 { platform_device_put(d); return rc; }
    *a.ghes_devs.add(a.count as usize) = d; a.count += 1; 0
}

unsafe fn hest_ghes_dev_register(n: u32) -> i32 {
    let mut a = ghes_arr { ghes_devs: kmalloc_array(n as usize, core::mem::size_of::<*mut platform_device>(), 0), count: 0 };
    if a.ghes_devs.is_null() { return -12; }
    let mut rc = apei_hest_parse(hest_parse_ghes, &mut a as *mut _ as *mut c_void);
    if rc == 0 { rc = ghes_estatus_pool_init(n); }
    if rc != 0 { for i in 0..a.count { platform_device_unregister(*a.ghes_devs.add(i as usize)); } }
    kfree(a.ghes_devs); rc
}

pub unsafe extern "C" fn acpi_hest_init() {
    if hest_disable != 0 { return; }
    // ACPI table acquisition and status handling are provided by the surrounding kernel translation.
    let mut count: i32 = 0;
    let mut rc = apei_hest_parse(hest_parse_cmc, core::ptr::null_mut());
    if rc == 0 { rc = apei_hest_parse(hest_parse_ghes_count, &mut count as *mut _ as *mut c_void); }
    if rc == 0 && count > 0 { rc = hest_ghes_dev_register(count as u32); }
    if rc != 0 { hest_disable = HEST_DISABLED; if !hest_tab.is_null() { acpi_put_table(&mut (*hest_tab).header); } }
}

// GHES device registration and ACPI initialization retain the source-level flow;
// dependent kernel helpers and ACPI status constants are supplied externally.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
