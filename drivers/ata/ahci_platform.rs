// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * AHCI SATA platform driver
 *
 * Copyright 2004-2005  Red Hat, Inc.
 *   Jeff Garzik <jgarzik@pobox.com>
 * Copyright 2010  MontaVista Software, LLC.
 *   Anton Vorontsov <avorontsov@ru.mvista.com>
 */

// C dependencies supplied by the surrounding kernel translation.

const DRV_NAME: &str = "ahci";

extern "C" {
    static ahci_platform_ops: ata_port_operations;
    static ahci_platform_sht: scsi_host_template;
    static ahci_pm_ops: dev_pm_ops;

    fn ahci_platform_get_resources(
        pdev: *mut platform_device,
        flags: u32,
    ) -> *mut ahci_host_priv;
    fn ahci_platform_enable_resources(hpriv: *mut ahci_host_priv) -> c_int;
    fn device_is_compatible(dev: *mut device, compatible: *const c_char) -> bool;
    fn device_get_match_data(dev: *mut device) -> *const ata_port_info;
    fn ahci_platform_init_host(
        pdev: *mut platform_device,
        hpriv: *mut ahci_host_priv,
        port: *const ata_port_info,
        sht: *const scsi_host_template,
    ) -> c_int;
    fn ahci_platform_disable_resources(hpriv: *mut ahci_host_priv);
    fn ahci_platform_suspend(dev: *mut device) -> c_int;
    fn ahci_platform_resume(dev: *mut device) -> c_int;
    fn ata_platform_remove_one(pdev: *mut platform_device) -> c_int;
    fn ahci_platform_shutdown(pdev: *mut platform_device);
}

#[repr(C)]
struct ata_port_info {
    flags: u32,
    pio_mask: u32,
    udma_mask: u32,
    port_ops: *const ata_port_operations,
}

#[repr(C)]
struct scsi_host_template;
#[repr(C)]
struct ata_port_operations;
#[repr(C)]
struct ahci_host_priv {
    flags: u32,
}
#[repr(C)]
struct device;
#[repr(C)]
struct platform_device {
    dev: device,
}
#[repr(C)]
struct dev_pm_ops;

use core::ffi::{c_char, c_int, c_ulong};

const AHCI_FLAG_COMMON: u32 = 0;
const ATA_FLAG_NO_LPM: u32 = 0;
const ATA_PIO4: u32 = 0;
const ATA_UDMA6: u32 = 0;
const AHCI_HFLAG_NO_FBS: u32 = 0;
const AHCI_HFLAG_NO_NCQ: u32 = 0;
const AHCI_PLATFORM_GET_RESETS: u32 = 0;

static AHCI_PORT_INFO: ata_port_info = ata_port_info {
    flags: AHCI_FLAG_COMMON,
    pio_mask: ATA_PIO4,
    udma_mask: ATA_UDMA6,
    port_ops: unsafe { &ahci_platform_ops },
};

static AHCI_PORT_INFO_NOLPM: ata_port_info = ata_port_info {
    flags: AHCI_FLAG_COMMON | ATA_FLAG_NO_LPM,
    pio_mask: ATA_PIO4,
    udma_mask: ATA_UDMA6,
    port_ops: unsafe { &ahci_platform_ops },
};

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct acpi_device_id {
    id: *const c_char,
    driver_data: c_ulong,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut platform_device)>,
}

unsafe extern "C" fn ahci_probe(pdev: *mut platform_device) -> c_int {
    let dev = unsafe { &mut (*pdev).dev as *mut device };
    let hpriv = unsafe { ahci_platform_get_resources(pdev, AHCI_PLATFORM_GET_RESETS) };
    if hpriv.is_null() {
        return -1;
    }

    let rc = unsafe { ahci_platform_enable_resources(hpriv) };
    if rc != 0 {
        return rc;
    }

    if unsafe { device_is_compatible(dev, b"hisilicon,hisi-ahci\0".as_ptr() as *const c_char) } {
        unsafe { (*hpriv).flags |= AHCI_HFLAG_NO_FBS | AHCI_HFLAG_NO_NCQ };
    }

    let mut port = unsafe { device_get_match_data(dev) };
    if port.is_null() {
        port = &AHCI_PORT_INFO;
    }

    let rc = unsafe { ahci_platform_init_host(pdev, hpriv, port, &ahci_platform_sht) };
    if rc != 0 {
        unsafe { ahci_platform_disable_resources(hpriv) };
        return rc;
    }
    0
}

static AHCI_OF_MATCH: [of_device_id; 5] = [
    of_device_id { compatible: b"generic-ahci\0".as_ptr() as *const c_char },
    // Keep the following compatibles for device tree compatibility.
    of_device_id { compatible: b"ibm,476gtr-ahci\0".as_ptr() as *const c_char },
    of_device_id { compatible: b"hisilicon,hisi-ahci\0".as_ptr() as *const c_char },
    of_device_id { compatible: b"cavium,octeon-7130-ahci\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

static AHCI_ACPI_MATCH: [acpi_device_id; 3] = [
    acpi_device_id { id: b"APMC0D33\0".as_ptr() as *const c_char, driver_data: &AHCI_PORT_INFO_NOLPM as *const _ as c_ulong },
    acpi_device_id { id: core::ptr::null(), driver_data: 0 },
    acpi_device_id { id: core::ptr::null(), driver_data: 0 },
];

static mut AHCI_DRIVER: platform_driver = platform_driver {
    probe: Some(ahci_probe),
    remove: Some(ata_platform_remove_one),
    shutdown: Some(ahci_platform_shutdown),
};

// module_platform_driver(ahci_driver);
// MODULE_DEVICE_TABLE(of, ahci_of_match);
// MODULE_DEVICE_TABLE(acpi, ahci_acpi_match);
// MODULE_DESCRIPTION("AHCI SATA platform driver");
// MODULE_AUTHOR("Anton Vorontsov <avorontsov@ru.mvista.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:ahci");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
