// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 STMicroelectronics Limited
 *
 * Authors: Francesco Virlinzi <francesco.virlinzi@st.com>
 *          Alexandre Torgue <alexandre.torgue@st.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const DRV_NAME: &str = "st_ahci";

const ST_AHCI_OOBR: usize = 0xbc;
const ST_AHCI_OOBR_WE: u32 = 1 << 31;
const ST_AHCI_OOBR_CWMIN_SHIFT: u32 = 24;
const ST_AHCI_OOBR_CWMAX_SHIFT: u32 = 16;
const ST_AHCI_OOBR_CIMIN_SHIFT: u32 = 8;
const ST_AHCI_OOBR_CIMAX_SHIFT: u32 = 0;

#[repr(C)]
struct st_ahci_drv_data {
    pwr: *mut reset_control,
    sw_rst: *mut reset_control,
    pwr_rst: *mut reset_control,
}

#[repr(C)]
struct reset_control;
#[repr(C)]
struct ahci_host_priv {
    plat_data: *mut core::ffi::c_void,
    mmio: *mut u8,
}
#[repr(C)]
struct ata_host {
    private_data: *mut ahci_host_priv,
    dev: *mut device,
}
#[repr(C)]
struct device;
#[repr(C)]
struct platform_device { dev: device }
#[repr(C)]
struct ata_port_operations;
#[repr(C)]
struct ata_port_info;
#[repr(C)]
struct scsi_host_template;
#[repr(C)]
struct platform_driver;
#[repr(C)]
struct of_device_id;

extern "C" {
    static ahci_platform_ops: ata_port_operations;
    static ahci_platform_sht: scsi_host_template;
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn reset_control_deassert(reset: *mut reset_control) -> i32;
    fn reset_control_assert(reset: *mut reset_control) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn ahci_platform_disable_resources(hpriv: *mut ahci_host_priv);
    fn ahci_platform_enable_resources(hpriv: *mut ahci_host_priv) -> i32;
    fn ahci_platform_get_resources(pdev: *mut platform_device, port: i32) -> *mut ahci_host_priv;
    fn ahci_platform_init_host(
        pdev: *mut platform_device,
        hpriv: *mut ahci_host_priv,
        port_info: *const ata_port_info,
        sht: *const scsi_host_template,
    ) -> i32;
    fn ahci_platform_suspend_host(dev: *mut device) -> i32;
    fn ahci_platform_resume_host(dev: *mut device) -> i32;
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn devm_reset_control_get(dev: *mut device, id: *const core::ffi::c_char) -> *mut reset_control;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn ata_platform_remove_one(pdev: *mut platform_device) -> i32;
}

unsafe fn st_ahci_configure_oob(mmio: *mut u8) {
    let new_val = (0x02u32 << ST_AHCI_OOBR_CWMIN_SHIFT)
        | (0x04u32 << ST_AHCI_OOBR_CWMAX_SHIFT)
        | (0x08u32 << ST_AHCI_OOBR_CIMIN_SHIFT)
        | (0x0cu32 << ST_AHCI_OOBR_CIMAX_SHIFT);
    let old_val = readl(mmio.add(ST_AHCI_OOBR));
    writel(old_val | ST_AHCI_OOBR_WE, mmio.add(ST_AHCI_OOBR));
    writel(new_val | ST_AHCI_OOBR_WE, mmio.add(ST_AHCI_OOBR));
    writel(new_val, mmio.add(ST_AHCI_OOBR));
}

unsafe fn st_ahci_deassert_resets(hpriv: *mut ahci_host_priv, dev: *mut device) -> i32 {
    let drv_data = (*hpriv).plat_data as *mut st_ahci_drv_data;
    let mut err: i32;
    if !(*drv_data).pwr.is_null() {
        err = reset_control_deassert((*drv_data).pwr);
        if err != 0 { dev_err(dev, b"unable to bring out of pwrdwn\0".as_ptr() as _,); return err; }
    }
    if !(*drv_data).sw_rst.is_null() {
        err = reset_control_deassert((*drv_data).sw_rst);
        if err != 0 { dev_err(dev, b"unable to bring out of sw-rst\0".as_ptr() as _,); return err; }
    }
    if !(*drv_data).pwr_rst.is_null() {
        err = reset_control_deassert((*drv_data).pwr_rst);
        if err != 0 { dev_err(dev, b"unable to bring out of pwr-rst\0".as_ptr() as _,); return err; }
    }
    0
}

unsafe fn st_ahci_host_stop(host: *mut ata_host) {
    let hpriv = (*host).private_data;
    let drv_data = (*hpriv).plat_data as *mut st_ahci_drv_data;
    let dev = (*host).dev;
    if !(*drv_data).pwr.is_null() {
        let err = reset_control_assert((*drv_data).pwr);
        if err != 0 { dev_err(dev, b"unable to pwrdwn\n\0".as_ptr() as _); }
    }
    ahci_platform_disable_resources(hpriv);
}

unsafe fn st_ahci_probe_resets(hpriv: *mut ahci_host_priv, dev: *mut device) -> i32 {
    let drv_data = (*hpriv).plat_data as *mut st_ahci_drv_data;
    (*drv_data).pwr = devm_reset_control_get(dev, b"pwr-dwn\0".as_ptr() as _);
    if (*drv_data).pwr as usize == usize::MAX { dev_info(dev, b"power reset control not defined\n\0".as_ptr() as _); (*drv_data).pwr = core::ptr::null_mut(); }
    (*drv_data).sw_rst = devm_reset_control_get(dev, b"sw-rst\0".as_ptr() as _);
    if (*drv_data).sw_rst as usize == usize::MAX { dev_info(dev, b"soft reset control not defined\n\0".as_ptr() as _); (*drv_data).sw_rst = core::ptr::null_mut(); }
    (*drv_data).pwr_rst = devm_reset_control_get(dev, b"pwr-rst\0".as_ptr() as _);
    if (*drv_data).pwr_rst as usize == usize::MAX { dev_dbg(dev, b"power soft reset control not defined\n\0".as_ptr() as _); (*drv_data).pwr_rst = core::ptr::null_mut(); }
    st_ahci_deassert_resets(hpriv, dev)
}

unsafe fn st_ahci_probe(pdev: *mut platform_device) -> i32 {
    let drv_data = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<st_ahci_drv_data>(), 0) as *mut st_ahci_drv_data;
    if drv_data.is_null() { return -12; }
    let hpriv = ahci_platform_get_resources(pdev, 0);
    if hpriv as usize == usize::MAX { return -1; }
    (*hpriv).plat_data = drv_data as *mut core::ffi::c_void;
    let mut err = st_ahci_probe_resets(hpriv, &mut (*pdev).dev);
    if err != 0 { return err; }
    err = ahci_platform_enable_resources(hpriv);
    if err != 0 { return err; }
    st_ahci_configure_oob((*hpriv).mmio);
    err = ahci_platform_init_host(pdev, hpriv, &st_ahci_port_info, &ahci_platform_sht);
    if err != 0 { ahci_platform_disable_resources(hpriv); return err; }
    0
}

unsafe fn st_ahci_suspend(dev: *mut device) -> i32 {
    let host = dev_get_drvdata(dev) as *mut ata_host;
    let hpriv = (*host).private_data;
    let drv_data = (*hpriv).plat_data as *mut st_ahci_drv_data;
    let mut err = ahci_platform_suspend_host(dev);
    if err != 0 { return err; }
    if !(*drv_data).pwr.is_null() {
        err = reset_control_assert((*drv_data).pwr);
        if err != 0 { dev_err(dev, b"unable to pwrdwn\0".as_ptr() as _); return err; }
    }
    ahci_platform_disable_resources(hpriv);
    0
}

unsafe fn st_ahci_resume(dev: *mut device) -> i32 {
    let host = dev_get_drvdata(dev) as *mut ata_host;
    let hpriv = (*host).private_data;
    let mut err = ahci_platform_enable_resources(hpriv);
    if err != 0 { return err; }
    err = st_ahci_deassert_resets(hpriv, dev);
    if err != 0 { ahci_platform_disable_resources(hpriv); return err; }
    st_ahci_configure_oob((*hpriv).mmio);
    ahci_platform_resume_host(dev)
}

#[repr(C)]
struct ata_port_operations_def {
    inherits: *const ata_port_operations,
    host_stop: Option<unsafe fn(*mut ata_host)>,
}
static st_ahci_port_ops: ata_port_operations_def = ata_port_operations_def {
    inherits: unsafe { &ahci_platform_ops },
    host_stop: Some(st_ahci_host_stop),
};

#[repr(C)]
struct ata_port_info_def {
    flags: u32,
    pio_mask: u32,
    udma_mask: u32,
    port_ops: *const ata_port_operations_def,
}
static st_ahci_port_info: ata_port_info_def = ata_port_info_def {
    flags: 1,
    pio_mask: 4,
    udma_mask: 6,
    port_ops: &st_ahci_port_ops,
};

static st_ahci_match: [of_device_id; 2] = unsafe { core::mem::zeroed() };
static st_ahci_driver: *const platform_driver = core::ptr::null();

// module_platform_driver(st_ahci_driver);
// MODULE_DEVICE_TABLE(of, st_ahci_match);
// MODULE_AUTHOR("Alexandre Torgue <alexandre.torgue@st.com>");
// MODULE_AUTHOR("Francesco Virlinzi <francesco.virlinzi@st.com>");
// MODULE_DESCRIPTION("STMicroelectronics SATA AHCI Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
