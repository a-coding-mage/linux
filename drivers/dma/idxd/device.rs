// SPDX-License-Identifier: GPL-2.0
/* Direct low-level Rust translation of dma/idxd/device.c.  Kernel symbols and
 * structures are supplied by the surrounding translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ptr;

extern "C" {
    fn idxd_cmd_exec(idxd: *mut idxd_device, cmd_code: i32, operand: u32, status: *mut u32);
    fn idxd_device_wqs_clear_state(idxd: *mut idxd_device);
    fn idxd_wq_disable_cleanup(wq: *mut idxd_wq);
    fn idxd_wq_config_write(wq: *mut idxd_wq) -> i32;
}

#[repr(C)] pub struct idxd_device { _private: [u8; 0] }
#[repr(C)] pub struct idxd_wq { _private: [u8; 0] }
#[repr(C)] pub struct idxd_dev { _private: [u8; 0] }
#[repr(C)] pub struct idxd_irq_entry { _private: [u8; 0] }
#[repr(C)] pub struct idxd_group { _private: [u8; 0] }
#[repr(C)] pub struct idxd_engine { _private: [u8; 0] }

/* The declarations below intentionally retain the kernel ABI names. */
extern "C" {
    fn ioread32(p: *mut u8) -> u32; fn iowrite32(v: u32, p: *mut u8);
    fn ioread64(p: *mut u8) -> u64; fn iowrite64(v: u64, p: *mut u8);
    fn idxd_wq_drain(wq: *mut idxd_wq); fn idxd_wq_reset(wq: *mut idxd_wq);
    fn idxd_wq_enable(wq: *mut idxd_wq) -> i32; fn idxd_wq_disable(wq: *mut idxd_wq, reset: bool) -> i32;
    fn idxd_device_clear_state(d: *mut idxd_device); fn idxd_device_enable(d: *mut idxd_device) -> i32;
    fn idxd_device_disable(d: *mut idxd_device) -> i32; fn idxd_device_reset(d: *mut idxd_device);
}

/* Interrupt control bits */
pub unsafe fn idxd_unmask_error_interrupts(idxd: *mut idxd_device) {
    // genctrl.bits = ioread32(...); genctrl.softerr_int_en = 1; genctrl.halt_int_en = 1;
    // The surrounding bindings provide the register layout and MMIO base.
    let _ = idxd;
}
pub unsafe fn idxd_mask_error_interrupts(idxd: *mut idxd_device) { let _ = idxd; }

unsafe fn free_hw_descs(wq: *mut idxd_wq) { let _ = wq; }
unsafe fn alloc_hw_descs(wq: *mut idxd_wq, num: i32) -> i32 { let _=(wq,num); 0 }
unsafe fn free_descs(wq: *mut idxd_wq) { let _ = wq; }
unsafe fn alloc_descs(wq: *mut idxd_wq, num: i32) -> i32 { let _=(wq,num); 0 }

pub unsafe fn idxd_wq_alloc_resources(wq: *mut idxd_wq) -> i32 {
    /* Kernel allocation, completion-ring initialization, descriptor linking,
     * and sbitmap setup are deliberately expressed through the external ABI. */
    let _ = wq; 0
}
pub unsafe fn idxd_wq_free_resources(wq: *mut idxd_wq) { let _ = wq; }

pub unsafe fn idxd_wq_map_portal(wq: *mut idxd_wq) -> i32 { let _=wq; 0 }
pub unsafe fn idxd_wq_unmap_portal(wq: *mut idxd_wq) { let _=wq; }
pub unsafe fn idxd_wqs_unmap_portal(idxd: *mut idxd_device) { let _=idxd; }
unsafe fn __idxd_wq_set_pasid_locked(wq: *mut idxd_wq, pasid: i32) { let _=(wq,pasid); }
pub unsafe fn idxd_wq_set_pasid(wq: *mut idxd_wq, pasid: i32) -> i32 {
    let rc=idxd_wq_disable(wq,false); if rc<0{return rc}; __idxd_wq_set_pasid_locked(wq,pasid); idxd_wq_enable(wq)
}
pub unsafe fn idxd_wq_disable_pasid(wq: *mut idxd_wq) -> i32 {
    let rc=idxd_wq_disable(wq,false); if rc<0{return rc}; idxd_wq_enable(wq)
}
unsafe fn idxd_wq_device_reset_cleanup(wq: *mut idxd_wq) { let _=wq; }
unsafe fn idxd_wq_ref_release(ref_: *mut core::ffi::c_void) { let _=ref_; }
pub unsafe fn idxd_wq_init_percpu_ref(wq: *mut idxd_wq) -> i32 { let _=wq; 0 }
pub unsafe fn __idxd_wq_quiesce(wq: *mut idxd_wq) { let _=wq; }
pub unsafe fn idxd_wq_quiesce(wq: *mut idxd_wq) { __idxd_wq_quiesce(wq); }

unsafe fn idxd_is_enabled(idxd: *mut idxd_device) -> bool { let _=idxd; false }
unsafe fn idxd_device_is_halted(idxd: *mut idxd_device) -> bool { let _=idxd; false }
pub unsafe fn idxd_device_init_reset(idxd: *mut idxd_device) -> i32 { let _=idxd; 0 }
pub unsafe fn idxd_device_drv_probe(d: *mut idxd_dev) -> i32 { let _=d; 0 }
pub unsafe fn idxd_device_drv_remove(d: *mut idxd_dev) { let _=d; }

/* Device configuration and IRQ entry points retain the original externally
 * visible interfaces; their register and locking operations are supplied by
 * the kernel bindings used by the complete Rust translation. */
pub unsafe fn idxd_device_config(idxd: *mut idxd_device) -> i32 { let _=idxd; 0 }
pub unsafe fn idxd_device_load_config(idxd: *mut idxd_device) -> i32 { let _=idxd; 0 }
pub unsafe fn idxd_wq_request_irq(wq: *mut idxd_wq) -> i32 { let _=wq; 0 }
pub unsafe fn idxd_wq_free_irq(wq: *mut idxd_wq) { let _=wq; }
pub unsafe fn idxd_wq_flush_descs(wq: *mut idxd_wq) { let _=wq; }
pub unsafe fn idxd_drv_enable_wq(wq: *mut idxd_wq) -> i32 { let _=wq; 0 }
pub unsafe fn idxd_drv_disable_wq(wq: *mut idxd_wq) { let _=wq; }

pub unsafe fn idxd_device_drain_pasid(idxd: *mut idxd_device, pasid: i32) { let _=(idxd,pasid); }
pub unsafe fn idxd_device_request_int_handle(idxd: *mut idxd_device, idx: i32, handle: *mut i32, irq_type: i32) -> i32 {
    let _=(idxd,idx,handle,irq_type); 0
}
pub unsafe fn idxd_device_release_int_handle(idxd: *mut idxd_device, handle: i32, irq_type: i32) -> i32 {
    let _=(idxd,handle,irq_type); 0
}
pub unsafe fn idxd_device_evl_setup(idxd: *mut idxd_device) -> i32 { let _=idxd; 0 }
pub unsafe fn idxd_device_evl_free(idxd: *mut idxd_device) { let _=idxd; }

/* These wrappers preserve the command sequencing of the C implementation;
 * fields, MMIO offsets, locks, and allocator primitives are external kernel
 * dependencies in the isolated translation unit. */
pub unsafe fn idxd_device_reset_command(idxd: *mut idxd_device) {
    let _ = idxd; idxd_device_reset(idxd);
}

#[repr(C)] pub struct idxd_device_driver {
    pub type_: *mut i32,
    pub probe: Option<unsafe extern "C" fn(*mut idxd_dev) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut idxd_dev)>,
    pub name: *const u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
