// SPDX-License-Identifier: GPL-2.0-only
/*
 * MediaTek AHCI SATA driver
 *
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Ryder Lee <ryder.lee@mediatek.com>
 */

// Kernel dependencies supplied by the surrounding translation unit.

pub const DRV_NAME: &[u8] = b"ahci-mtk\0";

pub const SYS_CFG: u32 = 0x14;
pub const SYS_CFG_SATA_MSK: u32 = 0xc000_0000;
pub const SYS_CFG_SATA_EN: u32 = 1u32 << 31;

#[repr(C)]
pub struct mtk_ahci_plat {
    pub mode: *mut regmap,
    pub axi_rst: *mut reset_control,
    pub sw_rst: *mut reset_control,
    pub reg_rst: *mut reset_control,
}

#[repr(C)]
pub struct ata_port_info {
    pub flags: u32,
    pub pio_mask: u32,
    pub udma_mask: u32,
    pub port_ops: *const core::ffi::c_void,
}

#[repr(C)]
pub struct scsi_host_template;
#[repr(C)]
pub struct ahci_host_priv {
    pub plat_data: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct reset_control;
#[repr(C)]
pub struct of_device_id;
#[repr(C)]
pub struct platform_driver;

extern "C" {
    static ahci_platform_ops: core::ffi::c_void;
    static ahci_platform_sht: scsi_host_template;
    static ahci_pm_ops: core::ffi::c_void;

    fn devm_reset_control_get_optional_exclusive(
        dev: *mut device,
        id: *const u8,
    ) -> *mut reset_control;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn reset_control_assert(rst: *mut reset_control) -> i32;
    fn reset_control_deassert(rst: *mut reset_control) -> i32;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn syscon_regmap_lookup_by_phandle(
        np: *mut device_node,
        name: *const u8,
    ) -> *mut regmap;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn of_property_present(np: *mut device_node, name: *const u8) -> bool;
    fn ahci_platform_get_resources(pdev: *mut platform_device, bar: i32) -> *mut ahci_host_priv;
    fn ahci_platform_enable_resources(hpriv: *mut ahci_host_priv) -> i32;
    fn ahci_platform_init_host(
        pdev: *mut platform_device,
        hpriv: *mut ahci_host_priv,
        info: *const ata_port_info,
        sht: *const scsi_host_template,
    ) -> i32;
    fn ahci_platform_disable_resources(hpriv: *mut ahci_host_priv);
    fn ata_platform_remove_one(pdev: *mut platform_device) -> i32;
}

pub const AHCI_FLAG_COMMON: u32 = 0;
pub const ATA_PIO4: u32 = 0;
pub const ATA_UDMA6: u32 = 0;
pub const GFP_KERNEL: u32 = 0;

pub static ahci_port_info: ata_port_info = ata_port_info {
    flags: AHCI_FLAG_COMMON,
    pio_mask: ATA_PIO4,
    udma_mask: ATA_UDMA6,
    port_ops: unsafe { &ahci_platform_ops as *const _ },
};

unsafe fn mtk_ahci_platform_resets(hpriv: *mut ahci_host_priv, dev: *mut device) -> i32 {
    let plat = (*hpriv).plat_data as *mut mtk_ahci_plat;
    let mut err: i32;

    (*plat).axi_rst = devm_reset_control_get_optional_exclusive(dev, b"axi\0".as_ptr());
    if (*plat).axi_rst as isize == -517 { return (*plat).axi_rst as i32; }
    (*plat).sw_rst = devm_reset_control_get_optional_exclusive(dev, b"sw\0".as_ptr());
    if (*plat).sw_rst as isize == -517 { return (*plat).sw_rst as i32; }
    (*plat).reg_rst = devm_reset_control_get_optional_exclusive(dev, b"reg\0".as_ptr());
    if (*plat).reg_rst as isize == -517 { return (*plat).reg_rst as i32; }

    err = reset_control_assert((*plat).axi_rst);
    if err != 0 { dev_err(dev, b"failed to assert AXI bus\n\0".as_ptr()); return err; }
    err = reset_control_assert((*plat).sw_rst);
    if err != 0 { dev_err(dev, b"failed to assert PHY digital part\n\0".as_ptr()); return err; }
    err = reset_control_assert((*plat).reg_rst);
    if err != 0 { dev_err(dev, b"failed to assert PHY register part\n\0".as_ptr()); return err; }
    err = reset_control_deassert((*plat).reg_rst);
    if err != 0 { dev_err(dev, b"failed to deassert PHY register part\n\0".as_ptr()); return err; }
    err = reset_control_deassert((*plat).sw_rst);
    if err != 0 { dev_err(dev, b"failed to deassert PHY digital part\n\0".as_ptr()); return err; }
    err = reset_control_deassert((*plat).axi_rst);
    if err != 0 { dev_err(dev, b"failed to deassert AXI bus\n\0".as_ptr()); return err; }
    0
}

unsafe fn mtk_ahci_parse_property(hpriv: *mut ahci_host_priv, dev: *mut device) -> i32 {
    let plat = (*hpriv).plat_data as *mut mtk_ahci_plat;
    let np = (*dev).of_node;
    if of_property_present(np, b"mediatek,phy-mode\0".as_ptr()) {
        (*plat).mode = syscon_regmap_lookup_by_phandle(np, b"mediatek,phy-mode\0".as_ptr());
        if (*plat).mode as isize == -1 {
            dev_err(dev, b"missing phy-mode phandle\n\0".as_ptr());
            return (*plat).mode as i32;
        }
        regmap_update_bits((*plat).mode, SYS_CFG, SYS_CFG_SATA_MSK, SYS_CFG_SATA_EN);
    }
    0
}

unsafe fn mtk_ahci_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let plat = devm_kzalloc(dev, core::mem::size_of::<mtk_ahci_plat>(), GFP_KERNEL)
        as *mut mtk_ahci_plat;
    if plat.is_null() { return -12; }
    let hpriv = ahci_platform_get_resources(pdev, 0);
    if hpriv as isize == -1 { return hpriv as i32; }
    (*hpriv).plat_data = plat as *mut core::ffi::c_void;
    let mut err = mtk_ahci_parse_property(hpriv, dev);
    if err != 0 { return err; }
    err = mtk_ahci_platform_resets(hpriv, dev);
    if err != 0 { return err; }
    err = ahci_platform_enable_resources(hpriv);
    if err != 0 { return err; }
    err = ahci_platform_init_host(pdev, hpriv, &ahci_port_info, &ahci_platform_sht);
    if err == 0 { return 0; }
    ahci_platform_disable_resources(hpriv);
    err
}

// SIMPLE_DEV_PM_OPS, device matching, module registration, and metadata are
// retained as declarations/macros to be supplied by the kernel environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
