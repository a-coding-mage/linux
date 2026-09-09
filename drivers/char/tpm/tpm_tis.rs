// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2005, 2006 IBM Corporation
 * Copyright (C) 2014, 2015 Intel Corporation
 *
 * Device driver for TCG/TCPA TPM (trusted platform module).
 * This is a source-level Rust translation of tpm_tis.c.
 *
 * Kernel-provided types, constants, functions, macros, and registration
 * facilities referenced below are supplied by the surrounding dependencies.
 */

#[repr(C)]
pub struct TpmInfo {
    pub res: Resource,
    pub irq: c_int,
}

#[repr(C)]
pub struct TpmTisTcgPhy {
    pub priv_: TpmTisData,
    pub iobase: *mut c_void,
}

#[inline]
unsafe fn to_tpm_tis_tcg_phy(data: *mut TpmTisData) -> *mut TpmTisTcgPhy {
    (data as *mut u8).sub(offset_of!(TpmTisTcgPhy, priv_)) as *mut TpmTisTcgPhy
}

#[cfg(CONFIG_PREEMPT_RT)]
#[inline]
unsafe fn tpm_tis_flush(iobase: *mut c_void) {
    ioread8((iobase as *mut u8).add(TPM_ACCESS(0) as usize) as *mut c_void);
}

#[cfg(not(CONFIG_PREEMPT_RT))]
#[inline]
unsafe fn tpm_tis_flush(_iobase: *mut c_void) {}

#[inline]
unsafe fn tpm_tis_iowrite8(b: u8, iobase: *mut c_void, addr: u32) {
    iowrite8(b, (iobase as *mut u8).add(addr as usize) as *mut c_void);
    tpm_tis_flush(iobase);
}

#[inline]
unsafe fn tpm_tis_iowrite32(b: u32, iobase: *mut c_void, addr: u32) {
    iowrite32(b, (iobase as *mut u8).add(addr as usize) as *mut c_void);
    tpm_tis_flush(iobase);
}

static mut interrupts: bool = false;
static mut itpm: bool = false;
static mut force: bool = false;

#[cfg(all(CONFIG_PNP, CONFIG_ACPI))]
unsafe fn has_hid(dev: *mut AcpiDevice, hid: *const c_char) -> c_int {
    let mut id: *mut AcpiHardwareId = ptr::null_mut();
    list_for_each_entry!(id, (*dev).pnp.ids, list) {
        if strcmp(hid, (*id).id) == 0 { return 1; }
    }
    0
}

#[cfg(all(CONFIG_PNP, CONFIG_ACPI))]
#[inline]
unsafe fn is_itpm(dev: *mut AcpiDevice) -> c_int {
    if dev.is_null() { return 0; }
    has_hid(dev, b"INTC0102\0".as_ptr() as *const c_char)
}

#[cfg(not(all(CONFIG_PNP, CONFIG_ACPI)))]
#[inline]
unsafe fn is_itpm(_dev: *mut AcpiDevice) -> c_int { 0 }

#[cfg(CONFIG_ACPI)]
const DEVICE_IS_TPM2: usize = 1;

#[cfg(CONFIG_ACPI)]
static tpm_acpi_tbl: [AcpiDeviceId; 2] = [
    AcpiDeviceId { id: *b"MSFT0101\0", driver_data: DEVICE_IS_TPM2 },
    AcpiDeviceId::default(),
];

#[cfg(CONFIG_ACPI)]
unsafe fn check_acpi_tpm2(dev: *mut Device) -> c_int {
    let aid = acpi_match_device(tpm_acpi_tbl.as_ptr(), dev);
    if aid.is_null() || (*aid).driver_data != DEVICE_IS_TPM2 { return 0; }
    let mut tbl: *mut AcpiTableTpm2 = ptr::null_mut();
    let st = acpi_get_table(ACPI_SIG_TPM2, 1, &mut tbl as *mut _ as *mut *mut AcpiTableHeader);
    if ACPI_FAILURE(st) || (*tbl).header.length < core::mem::size_of::<AcpiTableTpm2>() {
        dev_err(dev, "failed to get TPM2 ACPI table\n");
        return -EINVAL;
    }
    let mut ret = 0;
    if (*tbl).start_method != ACPI_TPM2_MEMORY_MAPPED { ret = -ENODEV; }
    acpi_put_table(tbl as *mut AcpiTableHeader);
    ret
}

#[cfg(not(CONFIG_ACPI))]
unsafe fn check_acpi_tpm2(_dev: *mut Device) -> c_int { 0 }

unsafe fn tpm_tcg_read_bytes(data: *mut TpmTisData, addr: u32, mut len: u16,
                             result: *mut u8, io_mode: TpmTisIoMode) -> c_int {
    let phy = to_tpm_tis_tcg_phy(data);
    match io_mode {
        TpmTisIoMode::Phys8 => {
            let mut p = result;
            while len != 0 { *p = ioread8((*phy).iobase.add(addr as usize) as *mut c_void); p = p.add(1); len -= 1; }
        }
        TpmTisIoMode::Phys16 => {
            let value = cpu_to_le16(ioread16((*phy).iobase.add(addr as usize) as *mut c_void));
            ptr::copy_nonoverlapping(&value as *const _ as *const u8, result, 2);
        }
        TpmTisIoMode::Phys32 => {
            let value = cpu_to_le32(ioread32((*phy).iobase.add(addr as usize) as *mut c_void));
            ptr::copy_nonoverlapping(&value as *const _ as *const u8, result, 4);
        }
    }
    0
}

unsafe fn tpm_tcg_write_bytes(data: *mut TpmTisData, addr: u32, mut len: u16,
                              value: *const u8, io_mode: TpmTisIoMode) -> c_int {
    let phy = to_tpm_tis_tcg_phy(data);
    match io_mode {
        TpmTisIoMode::Phys8 => { let mut p = value; while len != 0 { tpm_tis_iowrite8(*p, (*phy).iobase, addr); p = p.add(1); len -= 1; } }
        TpmTisIoMode::Phys16 => return -EINVAL,
        TpmTisIoMode::Phys32 => { let v = le32_to_cpu(*(value as *const u32)); tpm_tis_iowrite32(v, (*phy).iobase, addr); }
    }
    0
}

static tpm_tcg: TpmTisPhyOps = TpmTisPhyOps { read_bytes: Some(tpm_tcg_read_bytes), write_bytes: Some(tpm_tcg_write_bytes) };

unsafe fn tpm_tis_init(dev: *mut Device, info: *mut TpmInfo) -> c_int {
    let mut irq = -1;
    let rc = check_acpi_tpm2(dev); if rc != 0 { return rc; }
    let phy = devm_kzalloc(dev, core::mem::size_of::<TpmTisTcgPhy>(), GFP_KERNEL) as *mut TpmTisTcgPhy;
    if phy.is_null() { return -ENOMEM; }
    (*phy).iobase = devm_ioremap_resource(dev, &mut (*info).res);
    if IS_ERR((*phy).iobase) { return PTR_ERR((*phy).iobase); }
    if interrupts { irq = (*info).irq; }
    if itpm || is_itpm(ACPI_COMPANION(dev)) != 0 { set_bit(TPM_TIS_ITPM_WORKAROUND, &mut (*phy).priv_.flags); }
    tpm_tis_core_init(dev, &mut (*phy).priv_, irq, &tpm_tcg, ACPI_HANDLE(dev))
}

// PNP/platform driver registration and module metadata are preserved as
// declarations and initialization calls against the surrounding kernel API.
unsafe fn tpm_tis_pnp_init(pnp_dev: *mut PnpDev, _pnp_id: *const PnpDeviceId) -> c_int {
    let mut info = TpmInfo { res: core::mem::zeroed(), irq: 0 };
    let res = pnp_get_resource(pnp_dev, IORESOURCE_MEM, 0); if res.is_null() { return -ENODEV; }
    info.res = *res; info.irq = if pnp_irq_valid(pnp_dev, 0) { pnp_irq(pnp_dev, 0) } else { -1 };
    tpm_tis_init(&mut (*pnp_dev).dev, &mut info)
}

unsafe fn tpm_tis_pnp_remove(dev: *mut PnpDev) {
    let chip = pnp_get_drvdata(dev); tpm_chip_unregister(chip); tpm_tis_remove(chip);
}

unsafe fn tpm_tis_plat_probe(pdev: *mut PlatformDevice) -> c_int {
    let mut info = TpmInfo { res: core::mem::zeroed(), irq: 0 };
    let res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() { dev_err(&mut (*pdev).dev, "no memory resource defined\n"); return -ENODEV; }
    info.res = *res; info.irq = platform_get_irq_optional(pdev, 0);
    if info.irq <= 0 { info.irq = if pdev != force_pdev { -1 } else { 0 }; }
    tpm_tis_init(&mut (*pdev).dev, &mut info)
}

unsafe fn tpm_tis_plat_remove(pdev: *mut PlatformDevice) {
    let chip = dev_get_drvdata(&mut (*pdev).dev); tpm_chip_unregister(chip); tpm_tis_remove(chip);
}

static mut force_pdev: *mut PlatformDevice = ptr::null_mut();

unsafe fn tpm_tis_force_device() -> c_int {
    if !force { return 0; }
    static resources: [Resource; 1] = [DEFINE_RES_MEM(0xFED40000, TIS_MEM_LEN)];
    let pdev = platform_device_register_simple(b"tpm_tis\0".as_ptr() as *const c_char, -1, resources.as_ptr(), 1);
    if IS_ERR(pdev) { return PTR_ERR(pdev); }
    force_pdev = pdev; 0
}

unsafe fn init_tis() -> c_int {
    let mut rc = tpm_tis_force_device(); if rc != 0 { return rc; }
    rc = platform_driver_register(&tis_drv); if rc != 0 { if !force_pdev.is_null() { platform_device_unregister(force_pdev); } return rc; }
    if IS_ENABLED(CONFIG_PNP) { rc = pnp_register_driver(&tis_pnp_driver); if rc != 0 { platform_driver_unregister(&tis_drv); if !force_pdev.is_null() { platform_device_unregister(force_pdev); } return rc; } }
    0
}

unsafe fn cleanup_tis() {
    pnp_unregister_driver(&tis_pnp_driver); platform_driver_unregister(&tis_drv);
    if !force_pdev.is_null() { platform_device_unregister(force_pdev); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
