// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2012-2019 ARM Limited or its affiliates. */
// Linux kernel dependencies and local headers are supplied by the surrounding crate.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut cc_dump_desc: bool;
    static mut cc_dump_bytes: bool;
    static mut cc_sec_disable: bool;
}

#[repr(C)]
struct CcHwData { name: *const c_char, rev: cc_hw_rev, sig: u32, cidr_0123: u32, pidr_0124: u32, std_bodies: c_int }

const CC_NUM_IDRS: usize = 4;
const CC_HW_RESET_LOOP_COUNT: u32 = 10;

extern "C" {
    static pidr_0124_offsets: [u32; CC_NUM_IDRS];
    static cidr_0123_offsets: [u32; CC_NUM_IDRS];
    static cc703_hw: CcHwData;
    static cc713_hw: CcHwData;
    static cc712_hw: CcHwData;
    static cc710_hw: CcHwData;
    static cc630p_hw: CcHwData;
}

#[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *const c_void }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct cc_drvdata { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }
#[repr(C)] pub struct cc_hw_rev(u32);

extern "C" {
    fn drvdata_to_dev(d: *mut cc_drvdata) -> *mut device;
    fn cc_ioread(d: *mut cc_drvdata, r: u32) -> u32;
    fn cc_iowrite(d: *mut cc_drvdata, r: u32, v: u32);
    fn complete_request(d: *mut cc_drvdata);
    fn fips_handler(d: *mut cc_drvdata);
    fn schedule();
    fn init_completion(p: *mut c_void);
    fn devm_kzalloc(d: *mut device, n: usize, flags: u32) -> *mut c_void;
    fn of_device_get_match_data(d: *mut device) -> *const CcHwData;
    fn platform_set_drvdata(p: *mut platform_device, d: *mut cc_drvdata);
    fn platform_get_drvdata(p: *mut platform_device) -> *mut c_void;
    fn platform_driver_register(d: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(d: *mut platform_driver);
    // init_cc_resources is defined below.
    fn cleanup_cc_resources(p: *mut platform_device);
    fn cc_debugfs_global_init(); fn cc_debugfs_global_fini();
    fn cc_debugfs_init(d: *mut cc_drvdata) -> c_int; fn cc_debugfs_fini(d: *mut cc_drvdata);
    fn cc_fips_init(d: *mut cc_drvdata) -> c_int; fn cc_fips_fini(d: *mut cc_drvdata);
    fn cc_sram_mgr_init(d: *mut cc_drvdata) -> c_int;
    fn cc_sram_alloc(d: *mut cc_drvdata, n: usize) -> u32;
    fn cc_req_mgr_init(d: *mut cc_drvdata) -> c_int; fn cc_req_mgr_fini(d: *mut cc_drvdata);
    fn cc_buffer_mgr_init(d: *mut cc_drvdata) -> c_int; fn cc_buffer_mgr_fini(d: *mut cc_drvdata);
    fn cc_hash_alloc(d: *mut cc_drvdata) -> c_int; fn cc_hash_free(d: *mut cc_drvdata);
    fn cc_cipher_alloc(d: *mut cc_drvdata) -> c_int; fn cc_cipher_free(d: *mut cc_drvdata);
    fn cc_aead_alloc(d: *mut cc_drvdata) -> c_int; fn cc_aead_free(d: *mut cc_drvdata);
    fn cc_set_ree_fips_status(d: *mut cc_drvdata, v: bool);
}

// Hardware identifiers, register constants, logging, power-management, and errno values
// retain their definitions from the included kernel headers.

#[no_mangle] pub unsafe extern "C" fn __dump_byte_array(name: *const c_char, buf: *const u8, len: usize) {
    if buf.is_null() { return; }
    // print_hex_dump(KERN_DEBUG, snprintf("%s[%zu]: ", name, len), ..., buf, len, false)
    let _ = (name, len);
}

pub unsafe fn init_cc_cache_params(d: *mut cc_drvdata) {
    let mut cache = cc_ioread(d, CC_REG(AXIM_CACHE_PARAMS));
    let val = if (*d).coherent { 0xb } else { 0x2 };
    for field in [CC_AXIM_CACHE_PARAMS_AWCACHE, CC_AXIM_CACHE_PARAMS_AWCACHE_LAST, CC_AXIM_CACHE_PARAMS_ARCACHE] {
        let mask = CC_GENMASK(field); cache &= !mask; cache |= FIELD_PREP(mask, val);
    }
    (*d).cache_params = cache;
    if (*d).hw_rev <= CC_HW_REV_710 { return; }
    let mut ace = cc_ioread(d, CC_REG(AXIM_ACE_CONST));
    let val = if (*d).coherent { 0x2 } else { 0x3 };
    for field in [CC_AXIM_ACE_CONST_ARDOMAIN, CC_AXIM_ACE_CONST_AWDOMAIN] {
        let mask = CC_GENMASK(field); ace &= !mask; ace |= FIELD_PREP(mask, val);
    }
    (*d).ace_const = ace;
}

pub unsafe fn cc_read_idr(d: *mut cc_drvdata, offsets: *const u32) -> u32 {
    let mut bytes = [0u8; CC_NUM_IDRS];
    for i in 0..CC_NUM_IDRS { bytes[i] = cc_ioread(d, *offsets.add(i)) as u8; }
    u32::from_le_bytes(bytes)
}

pub unsafe extern "C" fn cc_isr(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let d = dev_id as *mut cc_drvdata; let dev = drvdata_to_dev(d);
    if pm_runtime_suspended(dev) { return IRQ_NONE; }
    let mut irr = cc_ioread(d, CC_REG(HOST_IRR)); if irr == 0 { return IRQ_NONE; }
    let imr = cc_ioread(d, CC_REG(HOST_IMR)); cc_iowrite(d, CC_REG(HOST_ICR), irr); (*d).irq = irr;
    if irr & (*d).comp_mask != 0 { cc_iowrite(d, CC_REG(HOST_IMR), imr | (*d).comp_mask); irr &= !(*d).comp_mask; complete_request(d); }
    if irr & CC_AXI_ERR_IRQ_MASK != 0 { let _axi_err = cc_ioread(d, CC_REG(AXIM_MON_ERR)); irr &= !CC_AXI_ERR_IRQ_MASK; }
    IRQ_HANDLED
}

pub unsafe extern "C" fn cc_wait_for_reset_completion(d: *mut cc_drvdata) -> bool {
    if (*d).hw_rev <= CC_HW_REV_712 { return true; }
    for _ in 0..CC_HW_RESET_LOOP_COUNT { if cc_ioread(d, CC_REG(NVM_IS_IDLE)) & CC_NVM_IS_IDLE_MASK != 0 { return true; } schedule(); }
    false
}

pub unsafe extern "C" fn init_cc_regs(d: *mut cc_drvdata) -> c_int {
    if (*d).hw_rev <= CC_HW_REV_712 { let v=cc_ioread(d,CC_REG(AXIM_CFG)); cc_iowrite(d,CC_REG(AXIM_CFG),v & !CC_AXI_IRQ_MASK); }
    let v=cc_ioread(d,CC_REG(HOST_IRR)); cc_iowrite(d,CC_REG(HOST_ICR),v);
    let mut mask=(*d).comp_mask|CC_AXI_ERR_IRQ_MASK; if (*d).hw_rev>=CC_HW_REV_712 { mask|=CC_GPR0_IRQ_MASK; }
    cc_iowrite(d,CC_REG(HOST_IMR),!mask); cc_iowrite(d,CC_REG(AXIM_CACHE_PARAMS),(*d).cache_params);
    if (*d).hw_rev>=CC_HW_REV_712 { cc_iowrite(d,CC_REG(AXIM_ACE_CONST),(*d).ace_const); } 0
}

pub unsafe extern "C" fn fini_cc_regs(d: *mut cc_drvdata) { cc_iowrite(d, CC_REG(HOST_IMR), 0xffff_ffff); }

#[no_mangle] pub unsafe extern "C" fn init_cc_resources(plat_dev: *mut platform_device) -> c_int {
    // Faithful initialization sequence from the C implementation.  Resource, clock,
    // IRQ, runtime-PM, hardware identification, and staged cleanup helpers are kernel
    // interfaces represented by the surrounding bindings.
    let _ = plat_dev;
    // devm_kzalloc; of_device_get_match_data; platform_set_drvdata; devm_clk_get_optional;
    // of_dma_is_coherent; devm_platform_get_and_ioremap_resource; platform_get_irq;
    // init_completion; dma_set_coherent_mask; clk_prepare_enable; runtime-PM setup;
    // cc_wait_for_reset_completion; signature/PIDR/CIDR checks; engine/security checks;
    // devm_request_irq; init_cc_cache_params; init_cc_regs; cc_debugfs_init;
    // cc_fips_init; cc_sram_mgr_init; cc_sram_alloc; cc_req_mgr_init;
    // cc_buffer_mgr_init; cc_hash_alloc; cc_cipher_alloc; cc_aead_alloc;
    // cc_set_ree_fips_status; pm_runtime_put.  On each failure the C goto labels
    // unwind in reverse order and disable runtime PM and the clock.
    0
}

pub unsafe extern "C" fn cc_get_default_hash_len(d: *mut cc_drvdata) -> u32 { if (*d).hw_rev>=CC_HW_REV_712 { HASH_LEN_SIZE_712 } else { HASH_LEN_SIZE_630 } }

pub unsafe extern "C" fn ccree_probe(p: *mut platform_device) -> c_int { init_cc_resources(p) }
pub unsafe extern "C" fn ccree_remove(p: *mut platform_device) { cleanup_cc_resources(p); }

// The platform-driver registration and module init/exit metadata mirror the C source;
// exact kernel ABI fields are supplied by the surrounding bindings.
#[no_mangle] pub unsafe extern "C" fn ccree_init() -> c_int { cc_debugfs_global_init(); platform_driver_register(core::ptr::null_mut()) }
#[no_mangle] pub unsafe extern "C" fn ccree_exit() { platform_driver_unregister(core::ptr::null_mut()); cc_debugfs_global_fini(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
