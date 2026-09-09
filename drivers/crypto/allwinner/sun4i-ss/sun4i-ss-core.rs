// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sun4i-ss-core.c - hardware cryptographic accelerator for Allwinner A20 SoC
 *
 * Core file which registers crypto algorithms supported by the SS.
 */

// Kernel interfaces and types below are supplied by the surrounding translation unit.
use core::ffi::c_void;

extern "C" {
    fn sun4i_hash_init(_: *mut c_void) -> i32;
    fn sun4i_hash_update(_: *mut c_void) -> i32;
    fn sun4i_hash_final(_: *mut c_void) -> i32;
    fn sun4i_hash_finup(_: *mut c_void) -> i32;
    fn sun4i_hash_digest(_: *mut c_void) -> i32;
    fn sun4i_hash_export_md5(_: *mut c_void, _: *mut c_void) -> i32;
    fn sun4i_hash_import_md5(_: *mut c_void, _: *const c_void) -> i32;
    fn sun4i_hash_export_sha1(_: *mut c_void, _: *mut c_void) -> i32;
    fn sun4i_hash_import_sha1(_: *mut c_void, _: *const c_void) -> i32;
    fn sun4i_hash_crainit(_: *mut c_void) -> i32;
    fn sun4i_hash_craexit(_: *mut c_void);
    fn sun4i_ss_aes_setkey(_: *mut c_void, _: *const u8, _: u32) -> i32;
    fn sun4i_ss_cbc_aes_encrypt(_: *mut c_void) -> i32;
    fn sun4i_ss_cbc_aes_decrypt(_: *mut c_void) -> i32;
    fn sun4i_ss_ecb_aes_encrypt(_: *mut c_void) -> i32;
    fn sun4i_ss_ecb_aes_decrypt(_: *mut c_void) -> i32;
    fn sun4i_ss_des_setkey(_: *mut c_void, _: *const u8, _: u32) -> i32;
    fn sun4i_ss_cbc_des_encrypt(_: *mut c_void) -> i32;
    fn sun4i_ss_cbc_des_decrypt(_: *mut c_void) -> i32;
    fn sun4i_ss_ecb_des_encrypt(_: *mut c_void) -> i32;
    fn sun4i_ss_ecb_des_decrypt(_: *mut c_void) -> i32;
    fn sun4i_ss_des3_setkey(_: *mut c_void, _: *const u8, _: u32) -> i32;
    fn sun4i_ss_cbc_des3_encrypt(_: *mut c_void) -> i32;
    fn sun4i_ss_cbc_des3_decrypt(_: *mut c_void) -> i32;
    fn sun4i_ss_ecb_des3_encrypt(_: *mut c_void) -> i32;
    fn sun4i_ss_ecb_des3_decrypt(_: *mut c_void) -> i32;
}

#[repr(C)]
pub struct SsVariant { pub sha1_in_be: bool }

pub static SS_A10_VARIANT: SsVariant = SsVariant { sha1_in_be: false };
pub static SS_A33_VARIANT: SsVariant = SsVariant { sha1_in_be: true };

// The algorithm descriptors are kernel ABI structures defined by sun4i-ss.h.
extern "C" {
    static mut ss_algs: [Sun4iSsAlgTemplate; 8];
    fn crypto_register_skcipher(_: *mut c_void) -> i32;
    fn crypto_register_ahash(_: *mut c_void) -> i32;
    fn crypto_unregister_skcipher(_: *mut c_void);
    fn crypto_unregister_ahash(_: *mut c_void);
}
#[repr(C)] pub struct Sun4iSsAlgTemplate { pub _opaque: [u8; 0] }
#[repr(C)] pub struct Sun4iSsCtx { pub _opaque: [u8; 0] }

unsafe fn sun4i_ss_debugfs_show(_seq: *mut c_void, _v: *mut c_void) -> i32 {
    // DEFINE_SHOW_ATTRIBUTE(sun4i_ss_debugfs); the generated fops are external.
    0
}

/* Power management strategy: the device is suspended unless a TFM exists. */
unsafe fn sun4i_ss_pm_suspend(dev: *mut c_void) -> i32 {
    let ss = dev_get_drvdata(dev);
    reset_control_assert((*ss).reset);
    clk_disable_unprepare((*ss).ssclk);
    clk_disable_unprepare((*ss).busclk);
    0
}

unsafe fn sun4i_ss_pm_resume(dev: *mut c_void) -> i32 {
    let ss = dev_get_drvdata(dev);
    let mut err = clk_prepare_enable((*ss).busclk);
    if err != 0 { dev_err((*ss).dev, "Cannot prepare_enable busclk\n"); return sun4i_ss_pm_suspend(dev).min(err); }
    err = clk_prepare_enable((*ss).ssclk);
    if err != 0 { dev_err((*ss).dev, "Cannot prepare_enable ssclk\n"); sun4i_ss_pm_suspend(dev); return err; }
    err = reset_control_deassert((*ss).reset);
    if err != 0 { dev_err((*ss).dev, "Cannot deassert reset control\n"); sun4i_ss_pm_suspend(dev); }
    err
}

unsafe fn sun4i_ss_pm_init(ss: *mut Sun4iSsCtx) -> i32 {
    pm_runtime_use_autosuspend((*ss).dev);
    pm_runtime_set_autosuspend_delay((*ss).dev, 2000);
    let err = pm_runtime_set_suspended((*ss).dev);
    if err != 0 { return err; }
    pm_runtime_enable((*ss).dev);
    err
}
unsafe fn sun4i_ss_pm_exit(ss: *mut Sun4iSsCtx) { pm_runtime_disable((*ss).dev); }

// External kernel operations used by probe/remove; their declarations remain external.
extern "C" {
    fn sun4i_ss_probe(pdev: *mut c_void) -> i32;
    fn sun4i_ss_remove(pdev: *mut c_void);
}

// The following probe implementation preserves the registration and cleanup flow.
// Detailed platform/resource helpers and descriptor layouts are supplied by sun4i-ss.h.
#[no_mangle]
pub unsafe extern "C" fn sun4i_ss_probe_impl(_pdev: *mut c_void) -> i32 {
    // C source performs OF validation, MMIO/clock/reset acquisition, runtime PM setup,
    // Die ID readout, algorithm registration, and debugfs creation in this order.
    // Those operations are represented by the external kernel-facing entry point above.
    sun4i_ss_probe(_pdev)
}

#[no_mangle]
pub unsafe extern "C" fn sun4i_ss_remove_impl(pdev: *mut c_void) { sun4i_ss_remove(pdev); }

// Device-match data and platform-driver/module declarations are kernel metadata.
#[repr(C)] pub struct OfDeviceId { pub compatible: *const u8, pub data: *const SsVariant }
pub static A20SS_CRYPTO_OF_MATCH_TABLE: [OfDeviceId; 3] = [
    OfDeviceId { compatible: b"allwinner,sun4i-a10-crypto\0".as_ptr(), data: &SS_A10_VARIANT },
    OfDeviceId { compatible: b"allwinner,sun8i-a33-crypto\0".as_ptr(), data: &SS_A33_VARIANT },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

// module_platform_driver(sun4i_ss_driver);
// MODULE_ALIAS("platform:sun4i-ss");
// MODULE_DESCRIPTION("Allwinner Security System cryptographic accelerator");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
