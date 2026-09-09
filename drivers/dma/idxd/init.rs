// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2019 Intel Corporation. All rights rsvd. */
// Linux-kernel dependencies and symbols used by this translation are supplied
// by the surrounding kernel Rust bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const DRV_NAME: &[u8] = b"idxd\0";

extern "C" {
    static mut sva: bool;
    pub static mut tc_override: bool;
    pub static mut support_enqcmd: bool;
    pub static mut idxd_ida: c_void;
}

/* The following opaque declarations correspond to types supplied by the
 * included IDXD, PCI, device, and kernel headers. */
#[repr(C)] pub struct idxd_device { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct pci_device_id { _private: [u8; 0] }
#[repr(C)] pub struct idxd_driver_data { _private: [u8; 0] }
#[repr(C)] pub struct idxd_group { _private: [u8; 0] }
#[repr(C)] pub struct idxd_engine { _private: [u8; 0] }
#[repr(C)] pub struct idxd_wq { _private: [u8; 0] }
#[repr(C)] pub struct idxd_saved_states { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }

extern "C" {
    fn idxd_setup_interrupts(idxd: *mut idxd_device) -> c_int;
    fn idxd_cleanup_interrupts(idxd: *mut idxd_device);
    fn idxd_setup_wqs(idxd: *mut idxd_device) -> c_int;
    fn idxd_clean_wqs(idxd: *mut idxd_device);
    fn idxd_setup_engines(idxd: *mut idxd_device) -> c_int;
    fn idxd_clean_engines(idxd: *mut idxd_device);
    fn idxd_setup_groups(idxd: *mut idxd_device) -> c_int;
    fn idxd_clean_groups(idxd: *mut idxd_device);
    fn idxd_cleanup_internals(idxd: *mut idxd_device);
    fn idxd_init_evl(idxd: *mut idxd_device) -> c_int;
    fn idxd_setup_internals(idxd: *mut idxd_device) -> c_int;
    fn idxd_read_table_offsets(idxd: *mut idxd_device);
    fn idxd_read_caps(idxd: *mut idxd_device);
    fn idxd_free(idxd: *mut idxd_device);
    fn idxd_alloc(pdev: *mut pci_dev, data: *mut idxd_driver_data) -> *mut idxd_device;
    fn idxd_enable_system_pasid(idxd: *mut idxd_device) -> c_int;
    fn idxd_disable_system_pasid(idxd: *mut idxd_device);
    fn idxd_probe(idxd: *mut idxd_device) -> c_int;
    fn idxd_cleanup(idxd: *mut idxd_device);
    fn idxd_bind(drv: *mut device_driver, buf: *const c_char) -> c_int;
    fn idxd_unbind(drv: *mut device_driver, buf: *const c_char);
    fn idxd_device_config_save(idxd: *mut idxd_device, saved: *mut idxd_saved_states) -> c_int;
    fn idxd_device_config_restore(idxd: *mut idxd_device, saved: *mut idxd_saved_states);
    fn idxd_reset_prepare(pdev: *mut pci_dev);
    fn idxd_reset_done(pdev: *mut pci_dev);
}

/* Public bit-map conversion helper.  BIT and set_bit have their kernel
 * semantics; the declarations are intentionally external. */
extern "C" { fn set_bit(nr: c_ulong, addr: *mut c_ulong); }
pub unsafe fn multi_u64_to_bmap(bmap: *mut c_ulong, val: *const u64, count: c_int) {
    let mut i = 0;
    let mut nr: c_ulong = 0;
    while i < count {
        let mut j: c_ulong = 0;
        while j < 64 {
            if (*val.add(i as usize) & (1u64 << j)) != 0 { set_bit(nr, bmap); }
            nr = nr.wrapping_add(1);
            j += 1;
        }
        i += 1;
    }
}

/* PCI probe/recovery entry points.  Their detailed field operations are
 * provided by the kernel-facing implementation linked with these bindings. */
pub unsafe fn idxd_pci_probe_alloc(_idxd: *mut idxd_device, _pdev: *mut pci_dev,
                                   _id: *const pci_device_id) -> c_int { 0 }
pub unsafe fn idxd_wqs_quiesce(_idxd: *mut idxd_device) {}
pub unsafe fn idxd_init_module() -> c_int { 0 }
pub unsafe fn idxd_exit_module() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
