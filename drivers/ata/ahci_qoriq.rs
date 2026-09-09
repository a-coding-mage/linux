// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Freescale QorIQ AHCI SATA platform driver
 *
 * Copyright 2015 Freescale, Inc.
 *   Tang Yuantian <Yuantian.Tang@freescale.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

pub const DRV_NAME: &[u8] = b"ahci-qoriq\0";

pub const PORT_PHY1: usize = 0xA8;
pub const PORT_PHY2: usize = 0xAC;
pub const PORT_PHY3: usize = 0xB0;
pub const PORT_PHY4: usize = 0xB4;
pub const PORT_PHY5: usize = 0xB8;
pub const PORT_AXICC: usize = 0xBC;
pub const PORT_TRANS: usize = 0xC8;

pub const AHCI_PORT_PHY_1_CFG: u32 = 0xa003fffe;
pub const AHCI_PORT_PHY2_CFG: u32 = 0x28184d1f;
pub const AHCI_PORT_PHY3_CFG: u32 = 0x0e081509;
pub const AHCI_PORT_TRANS_CFG: u32 = 0x08000029;
pub const AHCI_PORT_AXICC_CFG: u32 = 0x3fffffff;

pub const LS1021A_PORT_PHY2: u32 = 0x28183414;
pub const LS1021A_PORT_PHY3: u32 = 0x0e080e06;
pub const LS1021A_PORT_PHY4: u32 = 0x064a080b;
pub const LS1021A_PORT_PHY5: u32 = 0x2aa86470;
pub const LS1021A_AXICC_ADDR: usize = 0xC0;

pub const SATA_ECC_DISABLE: u32 = 0x00020000;
pub const ECC_DIS_ARMV8_CH2: u32 = 0x80000000;
pub const ECC_DIS_LS1088A: u32 = 0x40000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ahci_qoriq_type {
    AHCI_LS1021A,
    AHCI_LS1028A,
    AHCI_LS1043A,
    AHCI_LS2080A,
    AHCI_LS1046A,
    AHCI_LS1088A,
    AHCI_LS2088A,
    AHCI_LX2160A,
}

#[repr(C)]
pub struct ahci_qoriq_priv {
    pub reg_base: *mut ccsr_ahci,
    pub r#type: ahci_qoriq_type,
    pub ecc_addr: *mut core::ffi::c_void,
    pub is_dmacoherent: bool,
}

static mut ecc_initialized: bool = false;

// Device-match tables retain the source entries; dependent kernel types/macros are external.
static ahci_qoriq_of_match: [of_device_id; 9] = [
    of_device_id { compatible: b"fsl,ls1021a-ahci\0".as_ptr() as _, data: AHCI_LS1021A as usize as *mut _ },
    of_device_id { compatible: b"fsl,ls1028a-ahci\0".as_ptr() as _, data: AHCI_LS1028A as usize as *mut _ },
    of_device_id { compatible: b"fsl,ls1043a-ahci\0".as_ptr() as _, data: AHCI_LS1043A as usize as *mut _ },
    of_device_id { compatible: b"fsl,ls2080a-ahci\0".as_ptr() as _, data: AHCI_LS2080A as usize as *mut _ },
    of_device_id { compatible: b"fsl,ls1046a-ahci\0".as_ptr() as _, data: AHCI_LS1046A as usize as *mut _ },
    of_device_id { compatible: b"fsl,ls1088a-ahci\0".as_ptr() as _, data: AHCI_LS1088A as usize as *mut _ },
    of_device_id { compatible: b"fsl,ls2088a-ahci\0".as_ptr() as _, data: AHCI_LS2088A as usize as *mut _ },
    of_device_id { compatible: b"fsl,lx2160a-ahci\0".as_ptr() as _, data: AHCI_LX2160A as usize as *mut _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null_mut() },
];

static ahci_qoriq_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: *b"NXP0004\0", driver_data: AHCI_LX2160A as usize },
    acpi_device_id { id: [0; 8], driver_data: 0 },
];

unsafe fn ahci_qoriq_hardreset(link: *mut ata_link, class: *mut u32, deadline: c_ulong) -> c_int {
    let timing = sata_ehc_deb_timing(&(*link).eh_context);
    let port_mmio = ahci_port_base((*link).ap);
    let mut px_cmd: u32 = 0;
    let mut px_is: u32 = 0;
    let mut px_val: u32;
    let ap = (*link).ap;
    let pp = (*ap).private_data as *mut ahci_port_priv;
    let hpriv = (*(*ap).host).private_data as *mut ahci_host_priv;
    let qoriq_priv = (*hpriv).plat_data as *mut ahci_qoriq_priv;
    let d2h_fis = (*pp).rx_fis.add(RX_FIS_D2H_REG);
    let mut tf: ata_taskfile = core::mem::zeroed();
    let mut online = false;
    let ls1021a_workaround = (*qoriq_priv).r#type as u32 == ahci_qoriq_type::AHCI_LS1021A as u32;
    ((*hpriv).stop_engine)(ap);
    if ls1021a_workaround { px_cmd = readl(port_mmio.add(PORT_CMD)); px_is = readl(port_mmio.add(PORT_IRQ_STAT)); }
    ata_tf_init((*link).device, &mut tf);
    tf.status = ATA_BUSY as u8;
    ata_tf_to_fis(&tf, 0, 0, d2h_fis);
    let rc = sata_link_hardreset(link, timing, deadline, &mut online, ahci_check_ready);
    if ls1021a_workaround {
        px_val = readl(port_mmio.add(PORT_CMD)); if px_val != px_cmd { writel(px_cmd, port_mmio.add(PORT_CMD)); }
        px_val = readl(port_mmio.add(PORT_IRQ_STAT)); if px_val != px_is { writel(px_is, port_mmio.add(PORT_IRQ_STAT)); }
    }
    ((*hpriv).start_engine)(ap);
    if online { *class = ahci_dev_classify(ap); }
    rc
}

// The remaining driver registration and platform plumbing are direct kernel bindings.
static mut ahci_qoriq_ops: ata_port_operations = ata_port_operations { inherits: &ahci_ops, reset: reset_ops { hardreset: Some(ahci_qoriq_hardreset) } };
static ahci_qoriq_port_info: ata_port_info = ata_port_info { flags: AHCI_FLAG_COMMON | ATA_FLAG_NCQ, pio_mask: ATA_PIO4, udma_mask: ATA_UDMA6, port_ops: &ahci_qoriq_ops };
static ahci_qoriq_sht: scsi_host_template = AHCI_SHT(DRV_NAME);

unsafe fn ahci_qoriq_phy_init(hpriv: *mut ahci_host_priv) -> c_int {
    let qpriv = (*hpriv).plat_data as *mut ahci_qoriq_priv; let reg_base = (*hpriv).mmio;
    match (*qpriv).r#type {
        ahci_qoriq_type::AHCI_LS1021A => { if !((*qpriv).ecc_addr.is_null() || ecc_initialized) { return -EINVAL; } if !(*qpriv).ecc_addr.is_null() && !ecc_initialized { writel(SATA_ECC_DISABLE, (*qpriv).ecc_addr); } writel(AHCI_PORT_PHY_1_CFG, reg_base.add(PORT_PHY1)); writel(LS1021A_PORT_PHY2, reg_base.add(PORT_PHY2)); writel(LS1021A_PORT_PHY3, reg_base.add(PORT_PHY3)); writel(LS1021A_PORT_PHY4, reg_base.add(PORT_PHY4)); writel(LS1021A_PORT_PHY5, reg_base.add(PORT_PHY5)); writel(AHCI_PORT_TRANS_CFG, reg_base.add(PORT_TRANS)); if (*qpriv).is_dmacoherent { writel(AHCI_PORT_AXICC_CFG, reg_base.add(LS1021A_AXICC_ADDR)); } }
        ahci_qoriq_type::AHCI_LS1043A | ahci_qoriq_type::AHCI_LS1046A => { if !((*qpriv).ecc_addr.is_null() || ecc_initialized) { return -EINVAL; } if !(*qpriv).ecc_addr.is_null() && !ecc_initialized { writel(readl((*qpriv).ecc_addr) | ECC_DIS_ARMV8_CH2, (*qpriv).ecc_addr); } writel(AHCI_PORT_PHY_1_CFG, reg_base.add(PORT_PHY1)); writel(AHCI_PORT_PHY2_CFG, reg_base.add(PORT_PHY2)); writel(AHCI_PORT_PHY3_CFG, reg_base.add(PORT_PHY3)); writel(AHCI_PORT_TRANS_CFG, reg_base.add(PORT_TRANS)); if (*qpriv).is_dmacoherent { writel(AHCI_PORT_AXICC_CFG, reg_base.add(PORT_AXICC)); } }
        ahci_qoriq_type::AHCI_LS1028A | ahci_qoriq_type::AHCI_LS1088A | ahci_qoriq_type::AHCI_LX2160A => { if !((*qpriv).ecc_addr.is_null() || ecc_initialized) { return -EINVAL; } if !(*qpriv).ecc_addr.is_null() && !ecc_initialized { writel(readl((*qpriv).ecc_addr) | ECC_DIS_LS1088A, (*qpriv).ecc_addr); } writel(AHCI_PORT_PHY_1_CFG, reg_base.add(PORT_PHY1)); writel(AHCI_PORT_PHY2_CFG, reg_base.add(PORT_PHY2)); writel(AHCI_PORT_PHY3_CFG, reg_base.add(PORT_PHY3)); writel(AHCI_PORT_TRANS_CFG, reg_base.add(PORT_TRANS)); if (*qpriv).is_dmacoherent { writel(AHCI_PORT_AXICC_CFG, reg_base.add(PORT_AXICC)); } }
        ahci_qoriq_type::AHCI_LS2080A | ahci_qoriq_type::AHCI_LS2088A => { writel(AHCI_PORT_PHY_1_CFG, reg_base.add(PORT_PHY1)); writel(AHCI_PORT_PHY2_CFG, reg_base.add(PORT_PHY2)); writel(AHCI_PORT_PHY3_CFG, reg_base.add(PORT_PHY3)); writel(AHCI_PORT_TRANS_CFG, reg_base.add(PORT_TRANS)); if (*qpriv).is_dmacoherent { writel(AHCI_PORT_AXICC_CFG, reg_base.add(PORT_AXICC)); } }
    } ecc_initialized = true; 0
}

unsafe fn ahci_qoriq_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node; let dev = &mut (*pdev).dev;
    let hpriv = ahci_platform_get_resources(pdev, 0); if IS_ERR(hpriv) { return PTR_ERR(hpriv); }
    let of_id = of_match_node(&ahci_qoriq_of_match, np); let acpi_id = acpi_match_device(&ahci_qoriq_acpi_match, dev);
    if of_id.is_null() && acpi_id.is_null() { return -ENODEV; }
    let qoriq_priv = devm_kzalloc(dev, core::mem::size_of::<ahci_qoriq_priv>(), GFP_KERNEL) as *mut ahci_qoriq_priv;
    if qoriq_priv.is_null() { return -ENOMEM; }
    (*qoriq_priv).r#type = if !of_id.is_null() { (*(*of_id).data as *mut ahci_qoriq_type).clone() } else { core::mem::transmute((*acpi_id).driver_data) };
    if !ecc_initialized {
        let res = platform_get_resource_byname(pdev, IORESOURCE_MEM, b"sata-ecc\0".as_ptr() as _);
        if !res.is_null() { (*qoriq_priv).ecc_addr = devm_ioremap_resource(dev, res); if IS_ERR((*qoriq_priv).ecc_addr) { return PTR_ERR((*qoriq_priv).ecc_addr); } }
    }
    if device_get_dma_attr(dev) == DEV_DMA_COHERENT { (*qoriq_priv).is_dmacoherent = true; }
    let mut rc = ahci_platform_enable_resources(hpriv); if rc != 0 { return rc; }
    (*hpriv).plat_data = qoriq_priv; rc = ahci_qoriq_phy_init(hpriv); if rc != 0 { ahci_platform_disable_resources(hpriv); return rc; }
    rc = ahci_platform_init_host(pdev, hpriv, &ahci_qoriq_port_info, &ahci_qoriq_sht); if rc != 0 { ahci_platform_disable_resources(hpriv); } rc
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn ahci_qoriq_resume(dev: *mut device) -> c_int {
    let host = dev_get_drvdata(dev) as *mut ata_host; let hpriv = (*host).private_data;
    let mut rc = ahci_platform_enable_resources(hpriv); if rc != 0 { return rc; }
    rc = ahci_qoriq_phy_init(hpriv); if rc != 0 { ahci_platform_disable_resources(hpriv); return rc; }
    rc = ahci_platform_resume_host(dev); if rc != 0 { ahci_platform_disable_resources(hpriv); return rc; }
    pm_runtime_disable(dev); pm_runtime_set_active(dev); pm_runtime_enable(dev); 0
}

static ahci_qoriq_pm_ops: dev_pm_ops = SIMPLE_DEV_PM_OPS!(ahci_platform_suspend, ahci_qoriq_resume);
static ahci_qoriq_driver: platform_driver = platform_driver { probe: Some(ahci_qoriq_probe), remove: Some(ata_platform_remove_one), driver: driver { name: DRV_NAME, of_match_table: &ahci_qoriq_of_match, acpi_match_table: &ahci_qoriq_acpi_match, pm: &ahci_qoriq_pm_ops } };
module_platform_driver!(ahci_qoriq_driver);

// MODULE_DESCRIPTION("Freescale QorIQ AHCI SATA platform driver");
// MODULE_AUTHOR("Tang Yuantian <Yuantian.Tang@freescale.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
