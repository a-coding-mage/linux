// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 * Copyright (C) 2011 Google, Inc.
 *
 * Author:
 *     Jay Cheng <jacheng@nvidia.com>
 *     James Wylder <james.wylder@motorola.com>
 *     Benoit Goby <benoit@android.com>
 *     Colin Cross <ccross@android.com>
 *     Hiroshi DOYU <hdoyu@nvidia.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const DRV_NAME: &str = "tegra-ahb";
const AHB_ARBITRATION_DISABLE: u32 = 0x04;
const AHB_ARBITRATION_PRIORITY_CTRL: u32 = 0x08;
const AHB_GIZMO_AHB_MEM: u32 = 0x10;
const AHB_GIZMO_APB_DMA: u32 = 0x14;
const AHB_GIZMO_IDE: u32 = 0x1c;
const AHB_GIZMO_USB: u32 = 0x20;
const AHB_GIZMO_AHB_XBAR_BRIDGE: u32 = 0x24;
const AHB_GIZMO_CPU_AHB_BRIDGE: u32 = 0x28;
const AHB_GIZMO_COP_AHB_BRIDGE: u32 = 0x2c;
const AHB_GIZMO_XBAR_APB_CTLR: u32 = 0x30;
const AHB_GIZMO_VCP_AHB_BRIDGE: u32 = 0x34;
const AHB_GIZMO_NAND: u32 = 0x40;
const AHB_GIZMO_SDMMC4: u32 = 0x48;
const AHB_GIZMO_XIO: u32 = 0x4c;
const AHB_GIZMO_BSEV: u32 = 0x64;
const AHB_GIZMO_BSEA: u32 = 0x74;
const AHB_GIZMO_NOR: u32 = 0x78;
const AHB_GIZMO_USB2: u32 = 0x7c;
const AHB_GIZMO_USB3: u32 = 0x80;
const AHB_GIZMO_SDMMC1: u32 = 0x84;
const AHB_GIZMO_SDMMC2: u32 = 0x88;
const AHB_GIZMO_SDMMC3: u32 = 0x8c;
const AHB_MEM_PREFETCH_CFG_X: u32 = 0xdc;
const AHB_ARBITRATION_XBAR_CTRL: u32 = 0xe0;
const AHB_MEM_PREFETCH_CFG3: u32 = 0xe4;
const AHB_MEM_PREFETCH_CFG4: u32 = 0xe8;
const AHB_MEM_PREFETCH_CFG1: u32 = 0xf0;
const AHB_MEM_PREFETCH_CFG2: u32 = 0xf4;
const AHB_ARBITRATION_AHB_MEM_WRQUE_MST_ID: u32 = 0xfc;
const AHB_ARBITRATION_XBAR_CTRL_SMMU_INIT_DONE: u32 = 1 << 17;
const INCORRECT_BASE_ADDR_LOW_BYTE: usize = 0x4;

const ENB_FAST_REARBITRATE: u32 = 1 << 2;
const DONT_SPLIT_AHB_WR: u32 = 1 << 7;
const IMMEDIATE: u32 = 1 << 18;
const PREFETCH_ENB: u32 = 1 << 31;
const PRIORITY_SELECT_USB: u32 = 1 << 6;
const PRIORITY_SELECT_USB2: u32 = 1 << 18;
const PRIORITY_SELECT_USB3: u32 = 1 << 17;

const fn ahb_priority_weight(x: u32) -> u32 { (x & 0x7) << 29 }
const fn mst_id(x: u32) -> u32 { (x & 0x1f) << 26 }
const fn addr_bndry(x: u32) -> u32 { (x & 0xf) << 21 }
const fn inactivity_timeout(x: u32) -> u32 { x & 0xffff }
const AHBDMA_MST_ID: u32 = mst_id(5);
const USB_MST_ID: u32 = mst_id(6);
const USB2_MST_ID: u32 = mst_id(18);
const USB3_MST_ID: u32 = mst_id(17);

static TEGRA_AHB_GIZMO: [u32; 29] = [
    AHB_ARBITRATION_DISABLE, AHB_ARBITRATION_PRIORITY_CTRL,
    AHB_GIZMO_AHB_MEM, AHB_GIZMO_APB_DMA, AHB_GIZMO_IDE, AHB_GIZMO_USB,
    AHB_GIZMO_AHB_XBAR_BRIDGE, AHB_GIZMO_CPU_AHB_BRIDGE,
    AHB_GIZMO_COP_AHB_BRIDGE, AHB_GIZMO_XBAR_APB_CTLR,
    AHB_GIZMO_VCP_AHB_BRIDGE, AHB_GIZMO_NAND, AHB_GIZMO_SDMMC4,
    AHB_GIZMO_XIO, AHB_GIZMO_BSEV, AHB_GIZMO_BSEA, AHB_GIZMO_NOR,
    AHB_GIZMO_USB2, AHB_GIZMO_USB3, AHB_GIZMO_SDMMC1, AHB_GIZMO_SDMMC2,
    AHB_GIZMO_SDMMC3, AHB_MEM_PREFETCH_CFG_X, AHB_ARBITRATION_XBAR_CTRL,
    AHB_MEM_PREFETCH_CFG3, AHB_MEM_PREFETCH_CFG4, AHB_MEM_PREFETCH_CFG1,
    AHB_MEM_PREFETCH_CFG2, AHB_ARBITRATION_AHB_MEM_WRQUE_MST_ID,
];

#[repr(C)]
struct tegra_ahb { regs: *mut core::ffi::c_void, dev: *mut device, ctx: [u32; 0] }

unsafe fn gizmo_readl(ahb: *mut tegra_ahb, offset: u32) -> u32 {
    readl((*ahb).regs.cast::<u8>().add(offset as usize).cast())
}
unsafe fn gizmo_writel(ahb: *mut tegra_ahb, value: u32, offset: u32) {
    writel(value, (*ahb).regs.cast::<u8>().add(offset as usize).cast())
}

#[cfg(CONFIG_TEGRA_IOMMU_SMMU)]
unsafe fn tegra_ahb_enable_smmu(dn: *mut device_node) -> i32 {
    let dev = driver_find_device_by_of_node(&mut (*tegra_ahb_driver).driver, dn);
    if dev.is_null() { return -EPROBE_DEFER; }
    let ahb = dev_get_drvdata(dev) as *mut tegra_ahb;
    put_device(dev);
    let val = gizmo_readl(ahb, AHB_ARBITRATION_XBAR_CTRL) | AHB_ARBITRATION_XBAR_CTRL_SMMU_INIT_DONE;
    gizmo_writel(ahb, val, AHB_ARBITRATION_XBAR_CTRL);
    0
}

unsafe fn tegra_ahb_suspend(dev: *mut device) -> i32 {
    let ahb = dev_get_drvdata(dev) as *mut tegra_ahb;
    for (i, offset) in TEGRA_AHB_GIZMO.iter().enumerate() { (*ahb).ctx.as_mut_ptr().add(i).write(gizmo_readl(ahb, *offset)); }
    0
}
unsafe fn tegra_ahb_resume(dev: *mut device) -> i32 {
    let ahb = dev_get_drvdata(dev) as *mut tegra_ahb;
    for (i, offset) in TEGRA_AHB_GIZMO.iter().enumerate() { gizmo_writel(ahb, (*ahb).ctx.as_ptr().add(i).read(), *offset); }
    0
}

unsafe fn tegra_ahb_gizmo_init(ahb: *mut tegra_ahb) {
    let mut val = gizmo_readl(ahb, AHB_GIZMO_AHB_MEM); val |= ENB_FAST_REARBITRATE | IMMEDIATE | DONT_SPLIT_AHB_WR; gizmo_writel(ahb, val, AHB_GIZMO_AHB_MEM);
    for off in [AHB_GIZMO_USB, AHB_GIZMO_USB2, AHB_GIZMO_USB3] { val = gizmo_readl(ahb, off) | IMMEDIATE; gizmo_writel(ahb, val, off); }
    val = gizmo_readl(ahb, AHB_ARBITRATION_PRIORITY_CTRL) | PRIORITY_SELECT_USB | PRIORITY_SELECT_USB2 | PRIORITY_SELECT_USB3 | ahb_priority_weight(7); gizmo_writel(ahb, val, AHB_ARBITRATION_PRIORITY_CTRL);
    for (off, id) in [(AHB_MEM_PREFETCH_CFG1, AHBDMA_MST_ID), (AHB_MEM_PREFETCH_CFG2, USB_MST_ID), (AHB_MEM_PREFETCH_CFG3, USB3_MST_ID), (AHB_MEM_PREFETCH_CFG4, USB2_MST_ID)] { val = gizmo_readl(ahb, off) & !mst_id(!0); val |= PREFETCH_ENB | id | addr_bndry(0xc) | inactivity_timeout(0x1000); gizmo_writel(ahb, val, off); }
}

unsafe fn tegra_ahb_probe(pdev: *mut platform_device) -> i32 {
    let bytes = core::mem::size_of::<tegra_ahb>() + core::mem::size_of::<u32>() * TEGRA_AHB_GIZMO.len();
    let ahb = devm_kzalloc(&mut (*pdev).dev, bytes, GFP_KERNEL) as *mut tegra_ahb;
    if ahb.is_null() { return -ENOMEM; }
    let res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if !res.is_null() && ((*res).start & INCORRECT_BASE_ADDR_LOW_BYTE) == INCORRECT_BASE_ADDR_LOW_BYTE {
        dev_warn(&mut (*pdev).dev, "incorrect AHB base address in DT data - enabling workaround\n");
        (*res).start -= INCORRECT_BASE_ADDR_LOW_BYTE;
    }
    (*ahb).regs = devm_ioremap_resource(&mut (*pdev).dev, res);
    if is_err((*ahb).regs) { return ptr_err((*ahb).regs); }
    (*ahb).dev = &mut (*pdev).dev;
    platform_set_drvdata(pdev, ahb.cast());
    tegra_ahb_gizmo_init(ahb);
    0
}

static TEGRA_AHB_OF_MATCH: [of_device_id; 3] = [
    of_device_id { compatible: "nvidia,tegra30-ahb" },
    of_device_id { compatible: "nvidia,tegra20-ahb" },
    of_device_id { compatible: core::ptr::null() },
];

static mut tegra_ahb_driver: platform_driver = platform_driver {
    probe: Some(tegra_ahb_probe),
    driver: driver { name: DRV_NAME, of_match_table: TEGRA_AHB_OF_MATCH.as_ptr(), pm: core::ptr::null() },
};

// module_platform_driver(tegra_ahb_driver);
// MODULE_AUTHOR("Hiroshi DOYU <hdoyu@nvidia.com>");
// MODULE_DESCRIPTION("Tegra AHB driver");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
