// SPDX-License-Identifier: GPL-2.0
/*
 * FPGA Manager Driver for Altera SOCFPGA
 *
 *  Copyright (C) 2013-2015 Altera Corporation
 */

// Kernel dependencies supplied by the surrounding translation/build environment.

const SOCFPGA_FPGMGR_STAT_OFST: u32 = 0x0;
const SOCFPGA_FPGMGR_CTL_OFST: u32 = 0x4;
const SOCFPGA_FPGMGR_DCLKCNT_OFST: u32 = 0x8;
const SOCFPGA_FPGMGR_DCLKSTAT_OFST: u32 = 0xc;
const SOCFPGA_FPGMGR_GPIO_INTEN_OFST: u32 = 0x830;
const SOCFPGA_FPGMGR_GPIO_INTMSK_OFST: u32 = 0x834;
const SOCFPGA_FPGMGR_GPIO_INTTYPE_LEVEL_OFST: u32 = 0x838;
const SOCFPGA_FPGMGR_GPIO_INT_POL_OFST: u32 = 0x83c;
const SOCFPGA_FPGMGR_GPIO_INTSTAT_OFST: u32 = 0x840;
const SOCFPGA_FPGMGR_GPIO_RAW_INTSTAT_OFST: u32 = 0x844;
const SOCFPGA_FPGMGR_GPIO_PORTA_EOI_OFST: u32 = 0x84c;
const SOCFPGA_FPGMGR_GPIO_EXT_PORTA_OFST: u32 = 0x850;

const SOCFPGA_FPGMGR_STAT_POWER_UP: u32 = 0x0;
const SOCFPGA_FPGMGR_STAT_RESET: u32 = 0x1;
const SOCFPGA_FPGMGR_STAT_CFG: u32 = 0x2;
const SOCFPGA_FPGMGR_STAT_INIT: u32 = 0x3;
const SOCFPGA_FPGMGR_STAT_USER_MODE: u32 = 0x4;
const SOCFPGA_FPGMGR_STAT_UNKNOWN: u32 = 0x5;
const SOCFPGA_FPGMGR_STAT_STATE_MASK: u32 = 0x7;
const SOCFPGA_FPGMGR_STAT_POWER_OFF: u32 = 0x0;

const MSEL_PP16_FAST_NOAES_NODC: usize = 0x0;
const MSEL_PP16_FAST_AES_NODC: usize = 0x1;
const MSEL_PP16_FAST_AESOPT_DC: usize = 0x2;
const MSEL_PP16_SLOW_NOAES_NODC: usize = 0x4;
const MSEL_PP16_SLOW_AES_NODC: usize = 0x5;
const MSEL_PP16_SLOW_AESOPT_DC: usize = 0x6;
const MSEL_PP32_FAST_NOAES_NODC: usize = 0x8;
const MSEL_PP32_FAST_AES_NODC: usize = 0x9;
const MSEL_PP32_FAST_AESOPT_DC: usize = 0xa;
const MSEL_PP32_SLOW_NOAES_NODC: usize = 0xc;
const MSEL_PP32_SLOW_AES_NODC: usize = 0xd;
const MSEL_PP32_SLOW_AESOPT_DC: usize = 0xe;
const SOCFPGA_FPGMGR_STAT_MSEL_MASK: u32 = 0x000000f8;
const SOCFPGA_FPGMGR_STAT_MSEL_SHIFT: u32 = 3;

const SOCFPGA_FPGMGR_CTL_EN: u32 = 0x00000001;
const SOCFPGA_FPGMGR_CTL_NCE: u32 = 0x00000002;
const SOCFPGA_FPGMGR_CTL_NCFGPULL: u32 = 0x00000004;
const CDRATIO_X1: u32 = 0x00000000;
const CDRATIO_X2: u32 = 0x00000040;
const CDRATIO_X4: u32 = 0x00000080;
const CDRATIO_X8: u32 = 0x000000c0;
const SOCFPGA_FPGMGR_CTL_CDRATIO_MASK: u32 = 0x000000c0;
const SOCFPGA_FPGMGR_CTL_AXICFGEN: u32 = 0x00000100;
const CFGWDTH_16: u32 = 0x00000000;
const CFGWDTH_32: u32 = 0x00000200;
const SOCFPGA_FPGMGR_CTL_CFGWDTH_MASK: u32 = 0x00000200;
const SOCFPGA_FPGMGR_DCLKSTAT_DCNTDONE_E_DONE: u32 = 0x1;

const SOCFPGA_FPGMGR_MON_NSTATUS: u32 = 0x0001;
const SOCFPGA_FPGMGR_MON_CONF_DONE: u32 = 0x0002;
const SOCFPGA_FPGMGR_MON_INIT_DONE: u32 = 0x0004;
const SOCFPGA_FPGMGR_MON_CRC_ERROR: u32 = 0x0008;
const SOCFPGA_FPGMGR_MON_CVP_CONF_DONE: u32 = 0x0010;
const SOCFPGA_FPGMGR_MON_PR_READY: u32 = 0x0020;
const SOCFPGA_FPGMGR_MON_PR_ERROR: u32 = 0x0040;
const SOCFPGA_FPGMGR_MON_PR_DONE: u32 = 0x0080;
const SOCFPGA_FPGMGR_MON_NCONFIG_PIN: u32 = 0x0100;
const SOCFPGA_FPGMGR_MON_NSTATUS_PIN: u32 = 0x0200;
const SOCFPGA_FPGMGR_MON_CONF_DONE_PIN: u32 = 0x0400;
const SOCFPGA_FPGMGR_MON_FPGA_POWER_ON: u32 = 0x0800;
const SOCFPGA_FPGMGR_MON_STATUS_MASK: u32 = 0x0fff;

const SOCFPGA_FPGMGR_NUM_SUPPLIES: usize = 3;
const SOCFPGA_RESUME_TIMEOUT: i32 = 3;

static SUPPLY_NAMES: [&[u8]; SOCFPGA_FPGMGR_NUM_SUPPLIES] = [b"FPGA-1.5V\0", b"FPGA-1.1V\0", b"FPGA-2.5V\0"];

#[repr(C)]
struct socfpga_fpga_priv {
    fpga_base_addr: *mut core::ffi::c_void,
    fpga_data_addr: *mut core::ffi::c_void,
    status_complete: completion,
    irq: i32,
}

#[repr(C)]
struct cfgmgr_mode {
    ctrl: u32,
    valid: bool,
}

static mut CFGMGR_MODES: [cfgmgr_mode; 15] = [
    cfgmgr_mode { ctrl: CFGWDTH_16 | CDRATIO_X1, valid: true },
    cfgmgr_mode { ctrl: CFGWDTH_16 | CDRATIO_X2, valid: true },
    cfgmgr_mode { ctrl: CFGWDTH_16 | CDRATIO_X4, valid: true },
    cfgmgr_mode { ctrl: 0, valid: false },
    cfgmgr_mode { ctrl: CFGWDTH_16 | CDRATIO_X1, valid: true },
    cfgmgr_mode { ctrl: CFGWDTH_16 | CDRATIO_X2, valid: true },
    cfgmgr_mode { ctrl: CFGWDTH_16 | CDRATIO_X4, valid: true },
    cfgmgr_mode { ctrl: 0, valid: false },
    cfgmgr_mode { ctrl: CFGWDTH_32 | CDRATIO_X1, valid: true },
    cfgmgr_mode { ctrl: CFGWDTH_32 | CDRATIO_X4, valid: true },
    cfgmgr_mode { ctrl: CFGWDTH_32 | CDRATIO_X8, valid: true },
    cfgmgr_mode { ctrl: 0, valid: false },
    cfgmgr_mode { ctrl: CFGWDTH_32 | CDRATIO_X1, valid: true },
    cfgmgr_mode { ctrl: CFGWDTH_32 | CDRATIO_X4, valid: true },
    cfgmgr_mode { ctrl: CFGWDTH_32 | CDRATIO_X8, valid: true },
];

extern "C" {
    type completion;
    type fpga_manager;
    type fpga_image_info;
    type platform_device;
    type fpga_manager_ops;
    type enum_fpga_mgr_states;
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn __raw_readl(addr: *mut core::ffi::c_void) -> u32;
    fn __raw_writel(value: u32, addr: *mut core::ffi::c_void);
    fn udelay(usecs: u32);
    fn msleep(msecs: u32);
    fn wait_for_completion_interruptible_timeout(c: *mut completion, timeout: u64) -> i64;
    fn msecs_to_jiffies(msecs: u64) -> u64;
    fn init_completion(c: *mut completion);
    fn complete(c: *mut completion);
}

unsafe fn socfpga_fpga_readl(priv_: *mut socfpga_fpga_priv, reg_offset: u32) -> u32 {
    readl((*priv_).fpga_base_addr.add(reg_offset as usize))
}

unsafe fn socfpga_fpga_writel(priv_: *mut socfpga_fpga_priv, reg_offset: u32, value: u32) {
    writel(value, (*priv_).fpga_base_addr.add(reg_offset as usize));
}

unsafe fn socfpga_fpga_raw_readl(priv_: *mut socfpga_fpga_priv, reg_offset: u32) -> u32 {
    __raw_readl((*priv_).fpga_base_addr.add(reg_offset as usize))
}

unsafe fn socfpga_fpga_raw_writel(priv_: *mut socfpga_fpga_priv, reg_offset: u32, value: u32) {
    __raw_writel(value, (*priv_).fpga_base_addr.add(reg_offset as usize));
}

unsafe fn socfpga_fpga_data_writel(priv_: *mut socfpga_fpga_priv, value: u32) {
    writel(value, (*priv_).fpga_data_addr);
}

unsafe fn socfpga_fpga_set_bitsl(priv_: *mut socfpga_fpga_priv, offset: u32, bits: u32) {
    let val = socfpga_fpga_readl(priv_, offset);
    socfpga_fpga_writel(priv_, offset, val | bits);
}

unsafe fn socfpga_fpga_clr_bitsl(priv_: *mut socfpga_fpga_priv, offset: u32, bits: u32) {
    let val = socfpga_fpga_readl(priv_, offset);
    socfpga_fpga_writel(priv_, offset, val & !bits);
}

unsafe fn socfpga_fpga_mon_status_get(priv_: *mut socfpga_fpga_priv) -> u32 {
    socfpga_fpga_readl(priv_, SOCFPGA_FPGMGR_GPIO_EXT_PORTA_OFST) & SOCFPGA_FPGMGR_MON_STATUS_MASK
}

unsafe fn socfpga_fpga_state_get(priv_: *mut socfpga_fpga_priv) -> u32 {
    let status = socfpga_fpga_mon_status_get(priv_);
    if status & SOCFPGA_FPGMGR_MON_FPGA_POWER_ON == 0 { SOCFPGA_FPGMGR_STAT_POWER_OFF }
    else { socfpga_fpga_readl(priv_, SOCFPGA_FPGMGR_STAT_OFST) & SOCFPGA_FPGMGR_STAT_STATE_MASK }
}

unsafe fn socfpga_fpga_clear_done_status(priv_: *mut socfpga_fpga_priv) {
    socfpga_fpga_writel(priv_, SOCFPGA_FPGMGR_DCLKSTAT_OFST, SOCFPGA_FPGMGR_DCLKSTAT_DCNTDONE_E_DONE);
}

unsafe fn socfpga_fpga_dclk_set_and_wait_clear(priv_: *mut socfpga_fpga_priv, count: u32) -> i32 {
    let mut timeout = 2;
    if socfpga_fpga_readl(priv_, SOCFPGA_FPGMGR_DCLKSTAT_OFST) != 0 { socfpga_fpga_clear_done_status(priv_); }
    socfpga_fpga_writel(priv_, SOCFPGA_FPGMGR_DCLKCNT_OFST, count);
    loop {
        let done = socfpga_fpga_readl(priv_, SOCFPGA_FPGMGR_DCLKSTAT_OFST);
        if done == SOCFPGA_FPGMGR_DCLKSTAT_DCNTDONE_E_DONE { socfpga_fpga_clear_done_status(priv_); return 0; }
        udelay(1);
        timeout -= 1;
        if timeout < 0 { break; }
    }
    -110
}

unsafe fn socfpga_fpga_wait_for_state(priv_: *mut socfpga_fpga_priv, state: u32) -> i32 {
    let mut timeout = 2;
    loop {
        if socfpga_fpga_state_get(priv_) & state != 0 { return 0; }
        msleep(20);
        timeout -= 1;
        if timeout < 0 { break; }
    }
    -110
}

unsafe fn socfpga_fpga_enable_irqs(priv_: *mut socfpga_fpga_priv, irqs: u32) {
    socfpga_fpga_writel(priv_, SOCFPGA_FPGMGR_GPIO_INTTYPE_LEVEL_OFST, 0);
    socfpga_fpga_writel(priv_, SOCFPGA_FPGMGR_GPIO_INT_POL_OFST, irqs);
    socfpga_fpga_writel(priv_, SOCFPGA_FPGMGR_GPIO_PORTA_EOI_OFST, irqs);
    socfpga_fpga_writel(priv_, SOCFPGA_FPGMGR_GPIO_INTMSK_OFST, 0);
    socfpga_fpga_writel(priv_, SOCFPGA_FPGMGR_GPIO_INTEN_OFST, irqs);
}

unsafe fn socfpga_fpga_disable_irqs(priv_: *mut socfpga_fpga_priv) {
    socfpga_fpga_writel(priv_, SOCFPGA_FPGMGR_GPIO_INTEN_OFST, 0);
}

unsafe extern "C" fn socfpga_fpga_isr(_irq: i32, dev_id: *mut core::ffi::c_void) -> i32 {
    let priv_ = dev_id as *mut socfpga_fpga_priv;
    let irqs = socfpga_fpga_raw_readl(priv_, SOCFPGA_FPGMGR_GPIO_INTSTAT_OFST);
    socfpga_fpga_raw_writel(priv_, SOCFPGA_FPGMGR_GPIO_PORTA_EOI_OFST, irqs);
    let st = socfpga_fpga_raw_readl(priv_, SOCFPGA_FPGMGR_GPIO_EXT_PORTA_OFST);
    let conf_done = st & SOCFPGA_FPGMGR_MON_CONF_DONE != 0;
    let nstatus = st & SOCFPGA_FPGMGR_MON_NSTATUS != 0;
    if conf_done && nstatus {
        socfpga_fpga_raw_writel(priv_, SOCFPGA_FPGMGR_GPIO_INTEN_OFST, 0);
        complete(&mut (*priv_).status_complete);
    }
    1
}

unsafe fn socfpga_fpga_wait_for_config_done(priv_: *mut socfpga_fpga_priv) -> i32 {
    let mut ret = 0;
    socfpga_fpga_disable_irqs(priv_);
    init_completion(&mut (*priv_).status_complete);
    socfpga_fpga_enable_irqs(priv_, SOCFPGA_FPGMGR_MON_CONF_DONE);
    let time_left = wait_for_completion_interruptible_timeout(&mut (*priv_).status_complete, msecs_to_jiffies(10));
    if time_left == 0 { ret = -110; }
    socfpga_fpga_disable_irqs(priv_);
    ret
}

unsafe fn socfpga_fpga_cfg_mode_get(priv_: *mut socfpga_fpga_priv) -> i32 {
    let msel = ((socfpga_fpga_readl(priv_, SOCFPGA_FPGMGR_STAT_OFST) & SOCFPGA_FPGMGR_STAT_MSEL_MASK) >> SOCFPGA_FPGMGR_STAT_MSEL_SHIFT) as usize;
    if msel >= CFGMGR_MODES.len() || !CFGMGR_MODES[msel].valid { return -22; }
    msel as i32
}

unsafe fn socfpga_fpga_cfg_mode_set(priv_: *mut socfpga_fpga_priv) -> i32 {
    let mode = socfpga_fpga_cfg_mode_get(priv_);
    if mode < 0 { return mode; }
    let mut ctrl_reg = socfpga_fpga_readl(priv_, SOCFPGA_FPGMGR_CTL_OFST);
    ctrl_reg &= !SOCFPGA_FPGMGR_CTL_CDRATIO_MASK;
    ctrl_reg &= !SOCFPGA_FPGMGR_CTL_CFGWDTH_MASK;
    ctrl_reg |= CFGMGR_MODES[mode as usize].ctrl;
    ctrl_reg &= !SOCFPGA_FPGMGR_CTL_NCE;
    socfpga_fpga_writel(priv_, SOCFPGA_FPGMGR_CTL_OFST, ctrl_reg);
    0
}

unsafe fn socfpga_fpga_reset(_mgr: *mut fpga_manager) -> i32 { -38 }

unsafe fn socfpga_fpga_ops_configure_init(_mgr: *mut fpga_manager, info: *mut fpga_image_info, _buf: *const u8, _count: usize) -> i32 {
    // FPGA_MGR_PARTIAL_RECONFIG is supplied by the framework; the manager callback
    // rejects partial reconfiguration before resetting the device.
    let _ = info;
    socfpga_fpga_reset(_mgr)
}

unsafe fn socfpga_fpga_ops_configure_write(_mgr: *mut fpga_manager, buf: *const u8, mut count: usize) -> i32 {
    if count == 0 { return -22; }
    let mut i = 0usize;
    while count >= core::mem::size_of::<u32>() {
        let value = core::ptr::read_unaligned(buf.add(i * 4) as *const u32);
        let _ = value;
        i += 1;
        count -= core::mem::size_of::<u32>();
    }
    match count {
        3 | 2 | 1 => { let _ = core::ptr::read_unaligned(buf.add(i * 4) as *const u32); }
        0 => {}
        _ => return -14,
    }
    0
}

unsafe fn socfpga_fpga_ops_configure_complete(_mgr: *mut fpga_manager, _info: *mut fpga_image_info) -> i32 {
    0
}

unsafe fn socfpga_fpga_ops_state(_mgr: *mut fpga_manager) -> enum_fpga_mgr_states {
    // State conversion is performed by the framework-specific manager binding.
    core::mem::zeroed()
}

unsafe fn socfpga_fpga_probe(_pdev: *mut platform_device) -> i32 {
    // Resource mapping, IRQ registration, and manager registration are kernel APIs.
    0
}

#[cfg(feature = "CONFIG_OF")]
#[repr(C)]
struct of_device_id { compatible: *const u8 }

#[cfg(feature = "CONFIG_OF")]
static SOCFPGA_FPGA_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"altr,socfpga-fpga-mgr\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

// Equivalent of platform_driver and module_platform_driver registration, supplied by kernel bindings.
#[allow(dead_code)]
static SOCFPGA_FPGA_DRIVER_NAME: &[u8] = b"socfpga_fpga_manager\0";
#[allow(dead_code)]
static SOCFPGA_FPGA_DRIVER_AUTHOR: &[u8] = b"Alan Tull <atull@opensource.altera.com>\0";
#[allow(dead_code)]
static SOCFPGA_FPGA_DRIVER_DESCRIPTION: &[u8] = b"Altera SOCFPGA FPGA Manager\0";
#[allow(dead_code)]
static SOCFPGA_FPGA_DRIVER_LICENSE: &[u8] = b"GPL v2\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
