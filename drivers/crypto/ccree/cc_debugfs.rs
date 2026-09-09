// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2012-2019 ARM Limited or its affiliates. */

// Translated dependencies:
// <linux/kernel.h>, <linux/debugfs.h>, <linux/stringify.h>,
// "cc_driver.h", "cc_crypto_ctx.h", and "cc_debugfs.h"

use core::ffi::{c_char, c_int, c_void};

// External types and functions supplied by the surrounding kernel code.
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub name: *const c_char,
}

#[repr(C)]
pub struct debugfs_reg32 {
    pub name: *const c_char,
    pub offset: usize,
}

#[repr(C)]
pub struct debugfs_regset32 {
    pub regs: *const debugfs_reg32,
    pub nregs: usize,
    pub base: *mut c_void,
    pub dev: *mut device,
}

#[repr(C)]
pub struct cc_drvdata {
    pub cc_base: *mut c_void,
    pub plat_dev: *mut platform_device,
    pub dir: *mut dentry,
    pub coherent: bool,
    pub hw_rev: u32,
    pub sig_offset: usize,
    pub ver_offset: usize,
}

extern "C" {
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_remove(dentry: *mut dentry);
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn debugfs_create_regset32(
        name: *const c_char,
        mode: u16,
        parent: *mut dentry,
        regset: *mut debugfs_regset32,
    ) -> *mut dentry;
    fn debugfs_create_bool(
        name: *const c_char,
        mode: u16,
        parent: *mut dentry,
        value: *mut bool,
    ) -> *mut dentry;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn drvdata_to_dev(drvdata: *mut cc_drvdata) -> *mut device;
}

// Build-time kernel constants and register definitions supplied externally.
extern "C" {
    static CC_DEBUGFS_GFP_KERNEL: u32;
}

const GFP_KERNEL: u32 = unsafe { CC_DEBUGFS_GFP_KERNEL };
const CC_HW_REV_712: u32 = 0x712;

const PERIPHERAL_ID_0: usize = 0;
const PERIPHERAL_ID_1: usize = 1;
const PERIPHERAL_ID_2: usize = 2;
const PERIPHERAL_ID_3: usize = 3;
const PERIPHERAL_ID_4: usize = 4;
const COMPONENT_ID_0: usize = 5;
const COMPONENT_ID_1: usize = 6;
const COMPONENT_ID_2: usize = 7;
const COMPONENT_ID_3: usize = 8;
const HOST_IRR: usize = 9;
const HOST_POWER_DOWN_EN: usize = 10;
const AXIM_MON_ERR: usize = 11;
const DSCRPTR_QUEUE_CONTENT: usize = 12;
const HOST_IMR: usize = 13;
const AXIM_CFG: usize = 14;
const AXIM_CACHE_PARAMS: usize = 15;
const GPR_HOST: usize = 16;
const AXIM_MON_COMP: usize = 17;

const fn cc_reg(x: usize) -> usize { x }

macro_rules! cc_debug_reg {
    ($name:ident) => {
        debugfs_reg32 { name: concat!(stringify!($name), "\0").as_ptr() as *const c_char,
                        offset: cc_reg($name) }
    };
}

/*
 * This is a global var for the dentry of the
 * debugfs ccree/ dir. It is not tied down to
 * a specific instance of ccree, hence it is
 * global.
 */
static mut cc_debugfs_dir: *mut dentry = core::ptr::null_mut();

static mut ver_sig_regs: [debugfs_reg32; 2] = [
    debugfs_reg32 { name: b"SIGNATURE\0".as_ptr() as *const c_char, offset: 0 }, /* Must be 0th */
    debugfs_reg32 { name: b"VERSION\0".as_ptr() as *const c_char, offset: 0 }, /* Must be 1st */
];

static pid_cid_regs: [debugfs_reg32; 9] = [
    cc_debug_reg!(PERIPHERAL_ID_0), cc_debug_reg!(PERIPHERAL_ID_1),
    cc_debug_reg!(PERIPHERAL_ID_2), cc_debug_reg!(PERIPHERAL_ID_3),
    cc_debug_reg!(PERIPHERAL_ID_4), cc_debug_reg!(COMPONENT_ID_0),
    cc_debug_reg!(COMPONENT_ID_1), cc_debug_reg!(COMPONENT_ID_2),
    cc_debug_reg!(COMPONENT_ID_3),
];

static debug_regs: [debugfs_reg32; 9] = [
    cc_debug_reg!(HOST_IRR), cc_debug_reg!(HOST_POWER_DOWN_EN),
    cc_debug_reg!(AXIM_MON_ERR), cc_debug_reg!(DSCRPTR_QUEUE_CONTENT),
    cc_debug_reg!(HOST_IMR), cc_debug_reg!(AXIM_CFG),
    cc_debug_reg!(AXIM_CACHE_PARAMS), cc_debug_reg!(GPR_HOST),
    cc_debug_reg!(AXIM_MON_COMP),
];

pub unsafe fn cc_debugfs_global_init() {
    cc_debugfs_dir = debugfs_create_dir(b"ccree\0".as_ptr() as *const c_char, core::ptr::null_mut());
}

pub unsafe fn cc_debugfs_global_fini() {
    debugfs_remove(cc_debugfs_dir);
}

pub unsafe fn cc_debugfs_init(drvdata: *mut cc_drvdata) -> c_int {
    let dev = drvdata_to_dev(drvdata);
    let regset = devm_kzalloc(dev, core::mem::size_of::<debugfs_regset32>(), GFP_KERNEL)
        as *mut debugfs_regset32;
    if regset.is_null() { return -12; }

    (*regset).regs = debug_regs.as_ptr();
    (*regset).nregs = debug_regs.len();
    (*regset).base = (*drvdata).cc_base;
    (*regset).dev = dev;

    (*drvdata).dir = debugfs_create_dir((*(*drvdata).plat_dev).name, cc_debugfs_dir);
    debugfs_create_regset32(b"regs\0".as_ptr() as *const c_char, 0o400, (*drvdata).dir, regset);
    debugfs_create_bool(b"coherent\0".as_ptr() as *const c_char, 0o400,
                        (*drvdata).dir, &mut (*drvdata).coherent);

    let verset = devm_kzalloc(dev, core::mem::size_of::<debugfs_regset32>(), GFP_KERNEL)
        as *mut debugfs_regset32;
    // Failing here is not important enough to fail the module load
    if verset.is_null() { return 0; }

    if (*drvdata).hw_rev <= CC_HW_REV_712 {
        ver_sig_regs[0].offset = (*drvdata).sig_offset;
        ver_sig_regs[1].offset = (*drvdata).ver_offset;
        (*verset).regs = ver_sig_regs.as_ptr();
        (*verset).nregs = ver_sig_regs.len();
    } else {
        (*verset).regs = pid_cid_regs.as_ptr();
        (*verset).nregs = pid_cid_regs.len();
    }
    (*verset).base = (*drvdata).cc_base;
    (*verset).dev = dev;
    debugfs_create_regset32(b"version\0".as_ptr() as *const c_char, 0o400, (*drvdata).dir, verset);
    0
}

pub unsafe fn cc_debugfs_fini(drvdata: *mut cc_drvdata) {
    debugfs_remove_recursive((*drvdata).dir);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
