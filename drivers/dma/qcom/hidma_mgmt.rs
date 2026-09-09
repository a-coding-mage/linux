// SPDX-License-Identifier: GPL-2.0-only
/*
 * Qualcomm Technologies HIDMA DMA engine Management interface
 *
 * Copyright (c) 2015-2017, The Linux Foundation. All rights reserved.
 */

// Linux kernel dependencies supplied externally.

const HIDMA_QOS_N_OFFSET: usize = 0x700;
const HIDMA_CFG_OFFSET: usize = 0x400;
const HIDMA_MAX_BUS_REQ_LEN_OFFSET: usize = 0x41C;
const HIDMA_MAX_XACTIONS_OFFSET: usize = 0x420;
const HIDMA_HW_VERSION_OFFSET: usize = 0x424;
const HIDMA_CHRESET_TIMEOUT_OFFSET: usize = 0x418;

const HIDMA_MAX_WR_XACTIONS_MASK: u32 = (1 << 5) - 1;
const HIDMA_MAX_RD_XACTIONS_MASK: u32 = (1 << 5) - 1;
const HIDMA_WEIGHT_MASK: u32 = (1 << 7) - 1;
const HIDMA_MAX_BUS_REQ_LEN_MASK: u32 = (1 << 16) - 1;
const HIDMA_CHRESET_TIMEOUT_MASK: u32 = (1 << 20) - 1;

const HIDMA_MAX_WR_XACTIONS_BIT_POS: u32 = 16;
const HIDMA_MAX_BUS_WR_REQ_BIT_POS: u32 = 16;
const HIDMA_WRR_BIT_POS: u32 = 8;
const HIDMA_PRIORITY_BIT_POS: u32 = 15;

const HIDMA_AUTOSUSPEND_TIMEOUT: u32 = 2000;
const HIDMA_MAX_CHANNEL_WEIGHT: u32 = 15;

static mut MAX_WRITE_REQUEST: u32 = 0;
static mut MAX_READ_REQUEST: u32 = 0;
static mut MAX_WR_XACTIONS: u32 = 0;
static mut MAX_RD_XACTIONS: u32 = 0;

#[repr(C)]
pub struct hidma_mgmt_dev {
    pub pdev: *mut platform_device,
    pub addrsize: usize,
    pub virtaddr: *mut u8,
    pub dma_channels: u32,
    pub chreset_timeout_cycles: u32,
    pub max_write_request: u32,
    pub max_read_request: u32,
    pub max_wr_xactions: u32,
    pub max_rd_xactions: u32,
    pub priority: *mut u32,
    pub weight: *mut u32,
    pub hw_version: u32,
    pub hw_version_major: u32,
    pub hw_version_minor: u32,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

extern "C" {
    fn is_power_of_2(n: u32) -> bool;
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn pm_runtime_get_sync(dev: *mut core::ffi::c_void) -> i32;
    fn pm_runtime_mark_last_busy(dev: *mut core::ffi::c_void);
    fn pm_runtime_put_autosuspend(dev: *mut core::ffi::c_void);
    fn hidma_mgmt_init_sys(mgmtdev: *mut hidma_mgmt_dev) -> i32;
}

#[inline]
unsafe fn mmio(base: *mut u8, offset: usize) -> *mut u8 {
    base.add(offset)
}

pub unsafe fn hidma_mgmt_setup(mgmtdev: *mut hidma_mgmt_dev) -> i32 {
    let mgmtdev = &mut *mgmtdev;

    if !is_power_of_2(mgmtdev.max_write_request)
        || mgmtdev.max_write_request < 128
        || mgmtdev.max_write_request > 1024
    {
        return -22;
    }

    if !is_power_of_2(mgmtdev.max_read_request)
        || mgmtdev.max_read_request < 128
        || mgmtdev.max_read_request > 1024
    {
        return -22;
    }

    if mgmtdev.max_wr_xactions > HIDMA_MAX_WR_XACTIONS_MASK {
        return -22;
    }
    if mgmtdev.max_rd_xactions > HIDMA_MAX_RD_XACTIONS_MASK {
        return -22;
    }

    for i in 0..mgmtdev.dma_channels as usize {
        if *mgmtdev.priority.add(i) > 1 {
            return -22;
        }
        if *mgmtdev.weight.add(i) > HIDMA_MAX_CHANNEL_WEIGHT {
            return -22;
        }
        if *mgmtdev.weight.add(i) == 0 {
            *mgmtdev.weight.add(i) = 1;
        }
    }

    pm_runtime_get_sync(mgmtdev.pdev.cast());
    let addr = mmio(mgmtdev.virtaddr, HIDMA_MAX_BUS_REQ_LEN_OFFSET);
    let mut val = readl(addr);
    val &= !(HIDMA_MAX_BUS_REQ_LEN_MASK << HIDMA_MAX_BUS_WR_REQ_BIT_POS);
    val |= mgmtdev.max_write_request << HIDMA_MAX_BUS_WR_REQ_BIT_POS;
    val &= !HIDMA_MAX_BUS_REQ_LEN_MASK;
    val |= mgmtdev.max_read_request;
    writel(val, addr);

    let addr = mmio(mgmtdev.virtaddr, HIDMA_MAX_XACTIONS_OFFSET);
    val = readl(addr);
    val &= !(HIDMA_MAX_WR_XACTIONS_MASK << HIDMA_MAX_WR_XACTIONS_BIT_POS);
    val |= mgmtdev.max_wr_xactions << HIDMA_MAX_WR_XACTIONS_BIT_POS;
    val &= !HIDMA_MAX_RD_XACTIONS_MASK;
    val |= mgmtdev.max_rd_xactions;
    writel(val, addr);

    mgmtdev.hw_version = readl(mmio(mgmtdev.virtaddr, HIDMA_HW_VERSION_OFFSET));
    mgmtdev.hw_version_major = (mgmtdev.hw_version >> 28) & 0xF;
    mgmtdev.hw_version_minor = (mgmtdev.hw_version >> 16) & 0xF;

    for i in 0..mgmtdev.dma_channels as usize {
        let weight = *mgmtdev.weight.add(i);
        let priority = *mgmtdev.priority.add(i);
        let addr = mmio(mgmtdev.virtaddr, HIDMA_QOS_N_OFFSET + 4 * i);
        val = readl(addr);
        val &= !(1 << HIDMA_PRIORITY_BIT_POS);
        val |= (priority & 0x1) << HIDMA_PRIORITY_BIT_POS;
        val &= !(HIDMA_WEIGHT_MASK << HIDMA_WRR_BIT_POS);
        val |= (weight & HIDMA_WEIGHT_MASK) << HIDMA_WRR_BIT_POS;
        writel(val, addr);
    }

    let addr = mmio(mgmtdev.virtaddr, HIDMA_CHRESET_TIMEOUT_OFFSET);
    val = readl(addr);
    val &= !HIDMA_CHRESET_TIMEOUT_MASK;
    val |= mgmtdev.chreset_timeout_cycles & HIDMA_CHRESET_TIMEOUT_MASK;
    writel(val, addr);

    pm_runtime_mark_last_busy(mgmtdev.pdev.cast());
    pm_runtime_put_autosuspend(mgmtdev.pdev.cast());
    0
}

unsafe fn hidma_mgmt_probe(_pdev: *mut platform_device) -> i32 {
    // The platform/resource/property/runtime-PM operations are supplied by the
    // Linux kernel integration and retain the original probe entry point here.
    todo!("kernel platform-driver probe implementation")
}

// CONFIG_ACPI conditional device table and platform-driver/module registration
// are provided by the kernel build integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
