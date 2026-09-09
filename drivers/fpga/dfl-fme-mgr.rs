// SPDX-License-Identifier: GPL-2.0
/*
 * FPGA Manager Driver for FPGA Management Engine (FME)
 *
 * Copyright (C) 2017-2018 Intel Corporation, Inc.
 *
 * Authors:
 *   Kang Luwei <luwei.kang@intel.com>
 *   Xiao Guangrong <guangrong.xiao@linux.intel.com>
 *   Wu Hao <hao.wu@intel.com>
 *   Joseph Grecco <joe.grecco@intel.com>
 *   Enno Luebbers <enno.luebbers@intel.com>
 *   Tim Whisonant <tim.whisonant@intel.com>
 *   Ananda Ravuri <ananda.ravuri@intel.com>
 *   Christopher Rauer <christopher.rauer@intel.com>
 *   Henry Mitchel <henry.mitchel@intel.com>
 */

// Dependencies supplied by the surrounding kernel/Rust bindings.

const FME_PR_DFH: usize = 0x0;
const FME_PR_CTRL: usize = 0x8;
const FME_PR_STS: usize = 0x10;
const FME_PR_DATA: usize = 0x18;
const FME_PR_ERR: usize = 0x20;
const FME_PR_INTFC_ID_L: usize = 0xA8;
const FME_PR_INTFC_ID_H: usize = 0xB0;

const FME_PR_CTRL_PR_RST: u64 = 1u64 << 0;
const FME_PR_CTRL_PR_RSTACK: u64 = 1u64 << 4;
const FME_PR_CTRL_PR_RGN_ID: u64 = 0x7u64 << 7;
const FME_PR_CTRL_PR_START: u64 = 1u64 << 12;
const FME_PR_CTRL_PR_COMPLETE: u64 = 1u64 << 13;

const FME_PR_STS_PR_CREDIT: u64 = (1u64 << 9) - 1;
const FME_PR_STS_PR_STS: u64 = 1u64 << 16;
const FME_PR_STS_PR_STS_IDLE: u64 = 0;
const FME_PR_STS_PR_CTRLR_STS: u64 = 0x7u64 << 20;
const FME_PR_STS_PR_HOST_STS: u64 = 0xfu64 << 24;

const FME_PR_DATA_PR_DATA_RAW: u64 = (1u64 << 33) - 1;

const FME_PR_ERR_OPERATION_ERR: u64 = 1u64 << 0;
const FME_PR_ERR_CRC_ERR: u64 = 1u64 << 1;
const FME_PR_ERR_INCOMPATIBLE_BS: u64 = 1u64 << 2;
const FME_PR_ERR_PROTOCOL_ERR: u64 = 1u64 << 3;
const FME_PR_ERR_FIFO_OVERFLOW: u64 = 1u64 << 4;

const PR_WAIT_TIMEOUT: i32 = 8000000;
const PR_HOST_STATUS_IDLE: i32 = 0;

#[repr(C)]
struct fme_mgr_priv {
    ioaddr: *mut core::ffi::c_void,
    pr_error: u64,
}

unsafe fn pr_error_to_mgr_status(err: u64) -> u64 {
    let mut status = 0u64;
    if err & FME_PR_ERR_OPERATION_ERR != 0 { status |= FPGA_MGR_STATUS_OPERATION_ERR; }
    if err & FME_PR_ERR_CRC_ERR != 0 { status |= FPGA_MGR_STATUS_CRC_ERR; }
    if err & FME_PR_ERR_INCOMPATIBLE_BS != 0 { status |= FPGA_MGR_STATUS_INCOMPATIBLE_IMAGE_ERR; }
    if err & FME_PR_ERR_PROTOCOL_ERR != 0 { status |= FPGA_MGR_STATUS_IP_PROTOCOL_ERR; }
    if err & FME_PR_ERR_FIFO_OVERFLOW != 0 { status |= FPGA_MGR_STATUS_FIFO_OVERFLOW_ERR; }
    status
}

unsafe fn fme_mgr_pr_error_handle(fme_pr: *mut core::ffi::c_void) -> u64 {
    let pr_status = readq(fme_pr.add(FME_PR_STS));
    if pr_status & FME_PR_STS_PR_STS == 0 { return 0; }
    let pr_error = readq(fme_pr.add(FME_PR_ERR));
    writeq(pr_error, fme_pr.add(FME_PR_ERR));
    pr_error
}

unsafe fn fme_mgr_write_init(mgr: *mut fpga_manager, info: *mut fpga_image_info, _buf: *const i8, _count: usize) -> i32 {
    let priv_ = (*mgr).priv_ as *mut fme_mgr_priv;
    let fme_pr = (*priv_).ioaddr;
    if (*info).flags & FPGA_MGR_PARTIAL_RECONFIG == 0 { return -EINVAL; }
    let mut pr_ctrl = readq(fme_pr.add(FME_PR_CTRL));
    pr_ctrl |= FME_PR_CTRL_PR_RST; writeq(pr_ctrl, fme_pr.add(FME_PR_CTRL));
    if readq_poll_timeout(fme_pr.add(FME_PR_CTRL), &mut pr_ctrl, pr_ctrl & FME_PR_CTRL_PR_RSTACK, 1, PR_WAIT_TIMEOUT) != 0 { return -ETIMEDOUT; }
    pr_ctrl = readq(fme_pr.add(FME_PR_CTRL)); pr_ctrl &= !FME_PR_CTRL_PR_RST; writeq(pr_ctrl, fme_pr.add(FME_PR_CTRL));
    let mut pr_status = 0u64;
    if readq_poll_timeout(fme_pr.add(FME_PR_STS), &mut pr_status, (pr_status & FME_PR_STS_PR_STS) == FME_PR_STS_PR_STS_IDLE, 1, PR_WAIT_TIMEOUT) != 0 {
        (*priv_).pr_error = fme_mgr_pr_error_handle(fme_pr); return -ETIMEDOUT;
    }
    (*priv_).pr_error = fme_mgr_pr_error_handle(fme_pr);
    pr_ctrl = readq(fme_pr.add(FME_PR_CTRL)); pr_ctrl &= !FME_PR_CTRL_PR_RGN_ID;
    pr_ctrl |= ((*info).region_id as u64) << 7; writeq(pr_ctrl, fme_pr.add(FME_PR_CTRL));
    0
}

unsafe fn fme_mgr_write(mgr: *mut fpga_manager, buf: *const i8, mut count: usize) -> i32 {
    let priv_ = (*mgr).priv_ as *mut fme_mgr_priv; let fme_pr = (*priv_).ioaddr;
    let mut pr_ctrl = readq(fme_pr.add(FME_PR_CTRL)); pr_ctrl |= FME_PR_CTRL_PR_START; writeq(pr_ctrl, fme_pr.add(FME_PR_CTRL));
    let mut pr_status = readq(fme_pr.add(FME_PR_STS)); let mut pr_credit = (pr_status & FME_PR_STS_PR_CREDIT) as i32;
    let mut delay = 0i32; let mut i = 0usize;
    while count > 0 {
        while pr_credit <= 1 { if delay > PR_WAIT_TIMEOUT { return -ETIMEDOUT; } delay += 1; udelay(1); pr_status = readq(fme_pr.add(FME_PR_STS)); pr_credit = (pr_status & FME_PR_STS_PR_CREDIT) as i32; }
        if count < 4 { return -EINVAL; }
        let pr_data = *(buf as *const u32).add(i) as u64; writeq(pr_data & FME_PR_DATA_PR_DATA_RAW, fme_pr.add(FME_PR_DATA)); count -= 4; pr_credit -= 1; i += 1;
    }
    0
}

unsafe fn fme_mgr_write_complete(mgr: *mut fpga_manager, _info: *mut fpga_image_info) -> i32 {
    let priv_ = (*mgr).priv_ as *mut fme_mgr_priv; let fme_pr = (*priv_).ioaddr;
    let mut pr_ctrl = readq(fme_pr.add(FME_PR_CTRL)); pr_ctrl |= FME_PR_CTRL_PR_COMPLETE; writeq(pr_ctrl, fme_pr.add(FME_PR_CTRL));
    if readq_poll_timeout(fme_pr.add(FME_PR_CTRL), &mut pr_ctrl, !(pr_ctrl & FME_PR_CTRL_PR_START), 1, PR_WAIT_TIMEOUT) != 0 { return -ETIMEDOUT; }
    (*priv_).pr_error = fme_mgr_pr_error_handle(fme_pr); if (*priv_).pr_error != 0 { return -EIO; } 0
}

unsafe fn fme_mgr_status(mgr: *mut fpga_manager) -> u64 { pr_error_to_mgr_status((*((*mgr).priv_ as *mut fme_mgr_priv)).pr_error) }

#[repr(C)]
struct fpga_manager_ops { write_init: Option<unsafe fn(*mut fpga_manager, *mut fpga_image_info, *const i8, usize) -> i32>, write: Option<unsafe fn(*mut fpga_manager, *const i8, usize) -> i32>, write_complete: Option<unsafe fn(*mut fpga_manager, *mut fpga_image_info) -> i32>, status: Option<unsafe fn(*mut fpga_manager) -> u64> }

static FME_MGR_OPS: fpga_manager_ops = fpga_manager_ops { write_init: Some(fme_mgr_write_init), write: Some(fme_mgr_write), write_complete: Some(fme_mgr_write_complete), status: Some(fme_mgr_status) };

unsafe fn fme_mgr_get_compat_id(fme_pr: *mut core::ffi::c_void, id: *mut fpga_compat_id) {
    (*id).id_l = readq(fme_pr.add(FME_PR_INTFC_ID_L));
    (*id).id_h = readq(fme_pr.add(FME_PR_INTFC_ID_H));
}

unsafe fn fme_mgr_probe(pdev: *mut platform_device) -> i32 {
    let pdata = dev_get_platdata((*pdev).dev);
    let mut info: fpga_manager_info = core::mem::zeroed();
    let dev = (*pdev).dev;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<fme_mgr_priv>(), GFP_KERNEL) as *mut fme_mgr_priv;
    if priv_.is_null() { return -ENOMEM; }
    if !(*pdata).ioaddr.is_null() { (*priv_).ioaddr = (*pdata).ioaddr; }
    if (*priv_).ioaddr.is_null() {
        (*priv_).ioaddr = devm_platform_ioremap_resource(pdev, 0);
        if is_err((*priv_).ioaddr) { return ptr_err((*priv_).ioaddr); }
    }
    info.name = "DFL FME FPGA Manager";
    info.mops = &FME_MGR_OPS;
    info.priv = priv_ as *mut core::ffi::c_void;
    info.compat_id = devm_kzalloc(dev, core::mem::size_of::<fpga_compat_id>(), GFP_KERNEL) as *mut fpga_compat_id;
    if info.compat_id.is_null() { return -ENOMEM; }
    fme_mgr_get_compat_id((*priv_).ioaddr, info.compat_id);
    let mgr = devm_fpga_mgr_register_full(dev, &info);
    ptr_err_or_zero(mgr)
}

// Equivalent to the C module_platform_driver and module metadata declarations.
#[repr(C)]
struct platform_driver { name: *const i8, probe: Option<unsafe fn(*mut platform_device) -> i32> }
static FME_MGR_DRIVER: platform_driver = platform_driver { name: DFL_FPGA_FME_MGR.as_ptr() as *const i8, probe: Some(fme_mgr_probe) };
const MODULE_DESCRIPTION: &str = "FPGA Manager for DFL FPGA Management Engine";
const MODULE_AUTHOR: &str = "Intel Corporation";
const MODULE_LICENSE: &str = "GPL v2";
const MODULE_ALIAS: &str = "platform:dfl-fme-mgr";

// External kernel types, constants, and functions are supplied by dependencies.
extern "C" {
    fn readq(addr: *mut core::ffi::c_void) -> u64; fn writeq(value: u64, addr: *mut core::ffi::c_void);
    fn readq_poll_timeout(addr: *mut core::ffi::c_void, value: *mut u64, condition: u64, delay: u64, timeout: i32) -> i32;
    fn udelay(usecs: u32);
    fn dev_get_platdata(dev: *mut device) -> *mut dfl_fme_mgr_pdata;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn is_err(ptr: *mut core::ffi::c_void) -> bool; fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn devm_fpga_mgr_register_full(dev: *mut device, info: *const fpga_manager_info) -> *mut fpga_manager;
    fn ptr_err_or_zero(ptr: *mut fpga_manager) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
