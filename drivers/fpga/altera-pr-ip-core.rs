// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for Altera Partial Reconfiguration IP Core
 *
 * Copyright (C) 2016-2017 Intel Corporation
 *
 * Based on socfpga-a10.c Copyright (C) 2015-2016 Altera Corporation
 *  by Alan Tull <atull@opensource.altera.com>
 */
// Linux kernel dependencies supplied by the surrounding translation unit.

const ALT_PR_DATA_OFST: usize = 0x00;
const ALT_PR_CSR_OFST: usize = 0x04;

const ALT_PR_CSR_PR_START: u32 = 1 << 0;
const ALT_PR_CSR_STATUS_SFT: u32 = 2;
const ALT_PR_CSR_STATUS_MSK: u32 = 7 << ALT_PR_CSR_STATUS_SFT;
const ALT_PR_CSR_STATUS_NRESET: u32 = 0 << ALT_PR_CSR_STATUS_SFT;
const ALT_PR_CSR_STATUS_PR_ERR: u32 = 1 << ALT_PR_CSR_STATUS_SFT;
const ALT_PR_CSR_STATUS_CRC_ERR: u32 = 2 << ALT_PR_CSR_STATUS_SFT;
const ALT_PR_CSR_STATUS_BAD_BITS: u32 = 3 << ALT_PR_CSR_STATUS_SFT;
const ALT_PR_CSR_STATUS_PR_IN_PROG: u32 = 4 << ALT_PR_CSR_STATUS_SFT;
const ALT_PR_CSR_STATUS_PR_SUCCESS: u32 = 5 << ALT_PR_CSR_STATUS_SFT;

#[repr(C)]
struct alt_pr_priv {
    reg_base: *mut core::ffi::c_void,
}

unsafe fn alt_pr_fpga_state(mgr: *mut fpga_manager) -> fpga_mgr_states {
    let priv_ = (*mgr).priv_ as *mut alt_pr_priv;
    let mut err: *const core::ffi::c_char = c"unknown".as_ptr();
    let mut ret = FPGA_MGR_STATE_UNKNOWN;
    let mut val: u32;

    val = readl((*priv_).reg_base.cast::<u8>().add(ALT_PR_CSR_OFST).cast());
    val &= ALT_PR_CSR_STATUS_MSK;

    match val {
        ALT_PR_CSR_STATUS_NRESET => return FPGA_MGR_STATE_RESET,
        ALT_PR_CSR_STATUS_PR_ERR => {
            err = c"pr error".as_ptr();
            ret = FPGA_MGR_STATE_WRITE_ERR;
        }
        ALT_PR_CSR_STATUS_CRC_ERR => {
            err = c"crc error".as_ptr();
            ret = FPGA_MGR_STATE_WRITE_ERR;
        }
        ALT_PR_CSR_STATUS_BAD_BITS => {
            err = c"bad bits".as_ptr();
            ret = FPGA_MGR_STATE_WRITE_ERR;
        }
        ALT_PR_CSR_STATUS_PR_IN_PROG => return FPGA_MGR_STATE_WRITE,
        ALT_PR_CSR_STATUS_PR_SUCCESS => return FPGA_MGR_STATE_OPERATING,
        _ => {}
    }

    dev_err(&(*mgr).dev, c"encountered error code %d (%s) in %s()\n".as_ptr(), val, err, c"alt_pr_fpga_state".as_ptr());
    ret
}

unsafe fn alt_pr_fpga_write_init(mgr: *mut fpga_manager, info: *mut fpga_image_info, _buf: *const core::ffi::c_char, _count: usize) -> i32 {
    let priv_ = (*mgr).priv_ as *mut alt_pr_priv;
    if ((*info).flags & FPGA_MGR_PARTIAL_RECONFIG) == 0 {
        dev_err(&(*mgr).dev, c"%s Partial Reconfiguration flag not set\n".as_ptr(), c"alt_pr_fpga_write_init".as_ptr());
        return -EINVAL;
    }
    let val = readl((*priv_).reg_base.cast::<u8>().add(ALT_PR_CSR_OFST).cast());
    if (val & ALT_PR_CSR_PR_START) != 0 {
        dev_err(&(*mgr).dev, c"%s Partial Reconfiguration already started\n".as_ptr(), c"alt_pr_fpga_write_init".as_ptr());
        return -EINVAL;
    }
    writel(val | ALT_PR_CSR_PR_START, (*priv_).reg_base.cast());
    0
}

unsafe fn alt_pr_fpga_write(mgr: *mut fpga_manager, buf: *const core::ffi::c_char, mut count: usize) -> i32 {
    let priv_ = (*mgr).priv_ as *mut alt_pr_priv;
    let buffer_32 = buf as *const u32;
    let mut i = 0usize;
    if count == 0 { return -EINVAL; }
    while count >= core::mem::size_of::<u32>() {
        writel(*buffer_32.add(i), (*priv_).reg_base.cast());
        i += 1;
        count -= core::mem::size_of::<u32>();
    }
    match count {
        3 => writel(*buffer_32.add(i) & 0x00ffffff, (*priv_).reg_base.cast()),
        2 => writel(*buffer_32.add(i) & 0x0000ffff, (*priv_).reg_base.cast()),
        1 => writel(*buffer_32.add(i) & 0x000000ff, (*priv_).reg_base.cast()),
        0 => {}
        _ => return -EFAULT,
    }
    if alt_pr_fpga_state(mgr) == FPGA_MGR_STATE_WRITE_ERR { return -EIO; }
    0
}

unsafe fn alt_pr_fpga_write_complete(mgr: *mut fpga_manager, info: *mut fpga_image_info) -> i32 {
    let mut i: u32 = 0;
    loop {
        match alt_pr_fpga_state(mgr) {
            FPGA_MGR_STATE_WRITE_ERR => return -EIO,
            FPGA_MGR_STATE_OPERATING => {
                dev_info(&(*mgr).dev, c"successful partial reconfiguration\n".as_ptr());
                return 0;
            }
            _ => {}
        }
        udelay(1);
        i = i.wrapping_add(1);
        if !((*info).config_complete_timeout_us > i as usize) { break; }
    }
    dev_err(&(*mgr).dev, c"timed out waiting for write to complete\n".as_ptr());
    -ETIMEDOUT
}

static alt_pr_ops: fpga_manager_ops = fpga_manager_ops {
    state: Some(alt_pr_fpga_state),
    write_init: Some(alt_pr_fpga_write_init),
    write: Some(alt_pr_fpga_write),
    write_complete: Some(alt_pr_fpga_write_complete),
};

unsafe fn alt_pr_register(dev: *mut device, reg_base: *mut core::ffi::c_void) -> i32 {
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<alt_pr_priv>(), GFP_KERNEL) as *mut alt_pr_priv;
    if priv_.is_null() { return -ENOMEM; }
    (*priv_).reg_base = reg_base;
    let val = readl(reg_base.cast::<u8>().add(ALT_PR_CSR_OFST).cast());
    dev_dbg(dev, c"%s status=%d start=%d\n".as_ptr(), c"alt_pr_register".as_ptr(), (val & ALT_PR_CSR_STATUS_MSK) >> ALT_PR_CSR_STATUS_SFT, val & ALT_PR_CSR_PR_START);
    let mgr = devm_fpga_mgr_register(dev, dev_name(dev), &alt_pr_ops, priv_);
    PTR_ERR_OR_ZERO(mgr)
}

// EXPORT_SYMBOL_GPL(alt_pr_register);
// MODULE_AUTHOR("Matthew Gerlach <matthew.gerlach@linux.intel.com>");
// MODULE_DESCRIPTION("Altera Partial Reconfiguration IP Core");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
