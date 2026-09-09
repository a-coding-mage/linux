// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of the SafeXcel driver implementation.
// The declarations used by this translation (register definitions, Linux
// kernel interfaces, and SafeXcel data structures) are supplied externally.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// Kernel and device-specific symbols are intentionally external: this file is
// the direct translation unit and does not provide dependency implementations.
extern "C" {
    static mut max_rings: u32;
    fn eip197_trc_cache_setupvirt(priv_: *mut safexcel_crypto_priv);
    fn eip197_trc_cache_banksel(priv_: *mut safexcel_crypto_priv, addrmid: u32, actbank: *mut i32);
    fn eip197_trc_cache_probe(priv_: *mut safexcel_crypto_priv, maxbanks: i32, probemask: u32, stride: u32) -> u32;
    fn eip197_trc_cache_clear(priv_: *mut safexcel_crypto_priv, cs_rc_max: i32, cs_ht_wc: i32);
    fn eip197_trc_cache_init(priv_: *mut safexcel_crypto_priv) -> i32;
    fn eip197_init_firmware(priv_: *mut safexcel_crypto_priv);
    fn eip197_write_firmware(priv_: *mut safexcel_crypto_priv, fw: *const firmware) -> i32;
    fn poll_fw_ready(priv_: *mut safexcel_crypto_priv, fpp: i32) -> bool;
    fn eip197_start_firmware(priv_: *mut safexcel_crypto_priv, ipuesz: i32, ifppsz: i32, minifw: i32) -> bool;
    fn eip197_load_firmwares(priv_: *mut safexcel_crypto_priv) -> i32;
    fn safexcel_hw_setup_cdesc_rings(priv_: *mut safexcel_crypto_priv) -> i32;
    fn safexcel_hw_setup_rdesc_rings(priv_: *mut safexcel_crypto_priv) -> i32;
    fn safexcel_hw_init(priv_: *mut safexcel_crypto_priv) -> i32;
    fn safexcel_try_push_requests(priv_: *mut safexcel_crypto_priv, ring: i32);
    pub fn safexcel_dequeue(priv_: *mut safexcel_crypto_priv, ring: i32);
    pub fn safexcel_rdesc_check_errors(priv_: *mut safexcel_crypto_priv, rdp: *mut c_void) -> i32;
    pub fn safexcel_rdr_req_set(priv_: *mut safexcel_crypto_priv, ring: i32, rdesc: *mut safexcel_result_desc, req: *mut crypto_async_request);
    pub fn safexcel_rdr_req_get(priv_: *mut safexcel_crypto_priv, ring: i32) -> *mut crypto_async_request;
    pub fn safexcel_complete(priv_: *mut safexcel_crypto_priv, ring: i32);
    pub fn safexcel_invalidate_cache(async_: *mut crypto_async_request, priv_: *mut safexcel_crypto_priv, ctxr_dma: dma_addr_t, ring: i32) -> i32;
    fn safexcel_handle_result_descriptor(priv_: *mut safexcel_crypto_priv, ring: i32);
    fn safexcel_dequeue_work(work: *mut work_struct);
    fn safexcel_irq_ring(irq: i32, data: *mut c_void) -> irqreturn_t;
    fn safexcel_irq_ring_thread(irq: i32, data: *mut c_void) -> irqreturn_t;
    fn safexcel_request_ring_irq(pdev: *mut c_void, irqid: i32, is_pci_dev: i32, ring_id: i32, handler: *const c_void, threaded_handler: *const c_void, ring_irq_priv: *mut safexcel_ring_irq_data) -> i32;
    fn safexcel_register_algorithms(priv_: *mut safexcel_crypto_priv) -> i32;
    fn safexcel_unregister_algorithms(priv_: *mut safexcel_crypto_priv);
    fn safexcel_configure(priv_: *mut safexcel_crypto_priv);
    fn safexcel_init_register_offsets(priv_: *mut safexcel_crypto_priv);
    fn safexcel_probe_generic(pdev: *mut c_void, priv_: *mut safexcel_crypto_priv, is_pci_dev: i32) -> i32;
    fn safexcel_hw_reset_rings(priv_: *mut safexcel_crypto_priv);
    fn safexcel_probe(pdev: *mut platform_device) -> i32;
    fn safexcel_remove(pdev: *mut platform_device);
    fn safexcel_pci_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32;
    fn safexcel_pci_remove(pdev: *mut pci_dev);
    fn safexcel_init() -> i32;
    fn safexcel_exit();
}

// Opaque representations preserve the C ABI and layout ownership of the
// included kernel/SafeXcel headers.
#[repr(C)] pub struct safexcel_crypto_priv { _private: [u8; 0] }
#[repr(C)] pub struct safexcel_result_desc { _private: [u8; 0] }
#[repr(C)] pub struct crypto_async_request { _private: [u8; 0] }
#[repr(C)] pub struct firmware { pub data: *const u8, pub size: usize }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct safexcel_ring_irq_data { pub priv_: *mut safexcel_crypto_priv, pub ring: i32 }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct pci_device_id { _private: [u8; 0] }
pub type dma_addr_t = u64;
pub type irqreturn_t = i32;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
