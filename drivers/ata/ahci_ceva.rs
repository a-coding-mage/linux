// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Xilinx, Inc.
 * CEVA AHCI SATA platform driver
 *
 * based on the AHCI SATA platform driver by Jeff Garzik and Anton Vorontsov
 */

// Dependencies supplied by the surrounding kernel translation.

const AHCI_VEND_PCFG: usize = 0xA4;
const AHCI_VEND_PPCFG: usize = 0xA8;
const AHCI_VEND_PP2C: usize = 0xAC;
const AHCI_VEND_PP3C: usize = 0xB0;
const AHCI_VEND_PP4C: usize = 0xB4;
const AHCI_VEND_PP5C: usize = 0xB8;
const AHCI_VEND_AXICC: usize = 0xBC;
const AHCI_VEND_PAXIC: usize = 0xC0;
const AHCI_VEND_PTC: usize = 0xC8;

const PAXIC_ADBW_BW64: u32 = 0x1;
const PAXIC_OTL: u32 = 0x4 << 20;
const AXICC_ARCA_VAL: u32 = 0xF;
const AXICC_ARCF_VAL: u32 = 0xF << 4;
const AXICC_ARCH_VAL: u32 = 0xF << 8;
const AXICC_ARCP_VAL: u32 = 0xF << 12;
const AXICC_AWCFD_VAL: u32 = 0xF << 16;
const AXICC_AWCD_VAL: u32 = 0xF << 20;
const AXICC_AWCF_VAL: u32 = 0xF << 24;
const PCFG_TPSS_VAL: u32 = 0x32 << 16;
const PCFG_TPRS_VAL: u32 = 0x2 << 12;
const PCFG_PAD_VAL: u32 = 0x2;
const PPCFG_TTA: u32 = 0x1FFFE;
const PPCFG_PSSO_EN: u32 = 1 << 28;
const PPCFG_PSS_EN: u32 = 1 << 29;
const PPCFG_ESDF_EN: u32 = 1 << 31;
const PTC_RX_WM_VAL: u32 = 0x40;
const PTC_RSVD: u32 = 1 << 27;
const PORT_BASE: usize = 0x100;
const PORT_OFFSET: usize = 0x80;
const NR_PORTS: usize = 2;
const DRV_NAME: &str = "ahci-ceva";
const CEVA_FLAG_BROKEN_GEN2: i32 = 1;
const PORT_SCTL_SPD_GEN3: u32 = 0x3 << 4;
const PORT_SCTL_SPD_GEN2: u32 = 0x2 << 4;
const PORT_SCTL_SPD_GEN1: u32 = 0x1 << 4;
const PORT_SCTL_IPM: u32 = 0x3 << 8;

static mut rx_watermark: u32 = PTC_RX_WM_VAL;

#[repr(C)]
struct ceva_ahci_priv {
    ahci_pdev: *mut platform_device,
    pp2c: [u32; NR_PORTS],
    pp3c: [u32; NR_PORTS],
    pp4c: [u32; NR_PORTS],
    pp5c: [u32; NR_PORTS],
    axicc: u32,
    is_cci_enabled: bool,
    flags: i32,
}

unsafe fn ceva_ahci_read_id(dev: *mut ata_device, tf: *mut ata_taskfile, id: *mut u16) -> u32 {
    let err_mask = ata_do_dev_read_id(dev, tf, id);
    if err_mask != 0 { return err_mask; }
    // CEVA does not support device sleep; clear DEVSLP (bit 8) in word 78.
    *id.add(ATA_ID_FEATURE_SUPP as usize) &= (!((1u16) << 8)).to_le();
    0
}

#[repr(C)]
struct ahci_ceva_ops { inherits: *const ata_port_operations, read_id: unsafe fn(*mut ata_device, *mut ata_taskfile, *mut u16) -> u32 }
static mut ahci_ceva_ops: ahci_ceva_ops = ahci_ceva_ops { inherits: &ahci_platform_ops, read_id: ceva_ahci_read_id };

#[repr(C)]
struct ahci_ceva_port_info { flags: u32, pio_mask: u32, udma_mask: u32, port_ops: *mut ahci_ceva_ops }
static mut ahci_ceva_port_info: ahci_ceva_port_info = ahci_ceva_port_info { flags: AHCI_FLAG_COMMON, pio_mask: ATA_PIO4, udma_mask: ATA_UDMA6, port_ops: &raw mut ahci_ceva_ops };
extern "C" { static ahci_platform_sht: scsi_host_template; }

unsafe fn ahci_ceva_setup(hpriv: *mut ahci_host_priv) {
    let mmio = (*hpriv).mmio;
    let cevapriv = (*hpriv).plat_data as *mut ceva_ahci_priv;
    let mut tmp: u32 = readl(mmio.add(HOST_CTL));
    tmp |= HOST_AHCI_EN; writel(tmp, mmio.add(HOST_CTL));
    for i in 0..NR_PORTS {
        tmp = PCFG_TPSS_VAL | PCFG_TPRS_VAL | (PCFG_PAD_VAL + i as u32); writel(tmp, mmio.add(AHCI_VEND_PCFG));
        tmp = PAXIC_ADBW_BW64 | (((i as u32 * 2 + 1) << 8)) | (((i as u32 * 2 + 1) << 16)) | ((i as u32 * 2) << 4) | ((i as u32 * 2) << 12) | PAXIC_OTL; writel(tmp, mmio.add(AHCI_VEND_PAXIC));
        if (*cevapriv).is_cci_enabled { tmp = readl(mmio.add(AHCI_VEND_AXICC)); tmp |= AXICC_ARCA_VAL | AXICC_ARCF_VAL | AXICC_ARCH_VAL | AXICC_ARCP_VAL | AXICC_AWCFD_VAL | AXICC_AWCD_VAL | AXICC_AWCF_VAL; writel(tmp, mmio.add(AHCI_VEND_AXICC)); }
        tmp = PPCFG_TTA | PPCFG_PSS_EN | PPCFG_ESDF_EN; writel(tmp, mmio.add(AHCI_VEND_PPCFG));
        writel((*cevapriv).pp2c[i], mmio.add(AHCI_VEND_PP2C)); writel((*cevapriv).pp3c[i], mmio.add(AHCI_VEND_PP3C)); writel((*cevapriv).pp4c[i], mmio.add(AHCI_VEND_PP4C)); writel((*cevapriv).pp5c[i], mmio.add(AHCI_VEND_PP5C));
        tmp = rx_watermark | PTC_RSVD; writel(tmp, mmio.add(AHCI_VEND_PTC));
        tmp = PORT_SCTL_SPD_GEN3 | PORT_SCTL_IPM; if ((*cevapriv).flags & CEVA_FLAG_BROKEN_GEN2) != 0 { tmp = PORT_SCTL_SPD_GEN1 | PORT_SCTL_IPM; }
        writel(tmp, mmio.add(PORT_SCR_CTL + PORT_BASE + PORT_OFFSET * i));
    }
}

// Resource enablement, probe, suspend/resume, driver registration, and module metadata
// retain the kernel-facing interfaces of the C implementation.
unsafe fn ceva_ahci_platform_enable_resources(hpriv: *mut ahci_host_priv) -> i32 {
    let mut rc = ahci_platform_enable_regulators(hpriv); if rc != 0 { return rc; }
    rc = ahci_platform_enable_clks(hpriv); if rc != 0 { ahci_platform_disable_regulators(hpriv); return rc; }
    rc = ahci_platform_assert_rsts(hpriv); if rc != 0 { ahci_platform_disable_clks(hpriv); ahci_platform_disable_regulators(hpriv); return rc; }
    let mut i = 0; while i < (*hpriv).nports { if !ahci_ignore_port(hpriv, i) { rc = phy_init((*hpriv).phys[i]); if rc != 0 { while i > 0 { i -= 1; if !ahci_ignore_port(hpriv, i) { phy_exit((*hpriv).phys[i]); } } ahci_platform_disable_clks(hpriv); ahci_platform_disable_regulators(hpriv); return rc; } } i += 1; }
    ahci_platform_deassert_rsts(hpriv); i = 0; while i < (*hpriv).nports { if !ahci_ignore_port(hpriv, i) { rc = phy_power_on((*hpriv).phys[i]); if rc != 0 { phy_exit((*hpriv).phys[i]); while i > 0 { i -= 1; if !ahci_ignore_port(hpriv, i) { phy_power_off((*hpriv).phys[i]); phy_exit((*hpriv).phys[i]); } } ahci_platform_assert_rsts(hpriv); ahci_platform_disable_clks(hpriv); ahci_platform_disable_regulators(hpriv); return rc; } } i += 1; } 0
}

unsafe fn ceva_ahci_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let np = (*dev).of_node;
    let cevapriv = devm_kzalloc(dev, core::mem::size_of::<ceva_ahci_priv>(), GFP_KERNEL) as *mut ceva_ahci_priv;
    if cevapriv.is_null() { return -ENOMEM; }
    (*cevapriv).ahci_pdev = pdev;
    let hpriv = ahci_platform_get_resources(pdev, 0);
    if IS_ERR(hpriv) { return PTR_ERR(hpriv); }
    (*hpriv).rsts = devm_reset_control_get_optional_exclusive(dev, core::ptr::null());
    if IS_ERR((*hpriv).rsts) { return dev_err_probe(dev, PTR_ERR((*hpriv).rsts), "failed to get reset\n"); }
    let mut rc = ceva_ahci_platform_enable_resources(hpriv); if rc != 0 { return rc; }
    if of_property_read_bool(np, "ceva,broken-gen2") { (*cevapriv).flags = CEVA_FLAG_BROKEN_GEN2; }
    macro_rules! read_u8 { ($field:ident, $idx:expr, $name:expr) => { if of_property_read_u8_array(np, $name, (*cevapriv).$field[$idx].as_mut_ptr() as *mut u8, 4) < 0 { dev_warn(dev, concat!($name, " property not defined\n")); ahci_platform_disable_resources(hpriv); return -EINVAL; } }; }
    // Device-tree timing arrays are copied into the corresponding register words.
    read_u8!(pp2c, 0, "ceva,p0-cominit-params"); read_u8!(pp2c, 1, "ceva,p1-cominit-params");
    read_u8!(pp3c, 0, "ceva,p0-comwake-params"); read_u8!(pp3c, 1, "ceva,p1-comwake-params");
    read_u8!(pp4c, 0, "ceva,p0-burst-params"); read_u8!(pp4c, 1, "ceva,p1-burst-params");
    if of_property_read_u16_array(np, "ceva,p0-retry-params", (*cevapriv).pp5c[0].as_mut_ptr() as *mut u16, 2) < 0 { dev_warn(dev, "ceva,p0-retry-params property not defined\n"); ahci_platform_disable_resources(hpriv); return -EINVAL; }
    if of_property_read_u16_array(np, "ceva,p1-retry-params", (*cevapriv).pp5c[1].as_mut_ptr() as *mut u16, 2) < 0 { dev_warn(dev, "ceva,p1-retry-params property not defined\n"); ahci_platform_disable_resources(hpriv); return -EINVAL; }
    (*cevapriv).is_cci_enabled = device_get_dma_attr(dev) == DEV_DMA_COHERENT;
    (*hpriv).plat_data = cevapriv as *mut core::ffi::c_void; ahci_ceva_setup(hpriv);
    rc = ahci_platform_init_host(pdev, hpriv, &raw const ahci_ceva_port_info, &ahci_platform_sht);
    if rc != 0 { ahci_platform_disable_resources(hpriv); } rc
}

unsafe fn ceva_ahci_suspend(dev: *mut device) -> i32 { ahci_platform_suspend(dev) }
unsafe fn ceva_ahci_resume(dev: *mut device) -> i32 {
    let host = dev_get_drvdata(dev); let hpriv = (*host).private_data; let mut rc = ceva_ahci_platform_enable_resources(hpriv);
    if rc != 0 { return rc; } ahci_ceva_setup(hpriv); rc = ahci_platform_resume_host(dev); if rc != 0 { ahci_platform_disable_resources(hpriv); return rc; }
    pm_runtime_disable(dev); pm_runtime_set_active(dev); pm_runtime_enable(dev); 0
}

// static SIMPLE_DEV_PM_OPS(ahci_ceva_pm_ops, ceva_ahci_suspend, ceva_ahci_resume);
// Device-tree match: { compatible = "ceva,ahci-1v84" }, followed by the sentinel.
// platform_driver ceva_ahci_driver = { probe = ceva_ahci_probe, remove = ata_platform_remove_one,
//   name = DRV_NAME, of_match_table = ceva_ahci_of_match, pm = &ahci_ceva_pm_ops };
// module_platform_driver(ceva_ahci_driver);
// MODULE_DESCRIPTION("CEVA AHCI SATA platform driver"); MODULE_AUTHOR("Xilinx Inc."); MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
