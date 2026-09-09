// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/drivers/misc/xillybus_pcie.c
 *
 * Copyright 2011 Xillybus Ltd, http://xillybus.com
 *
 * Driver for the Xillybus FPGA/host framework using PCI Express.
 */

// The Linux kernel headers and xillybus.h provide the declarations referenced below.

// MODULE_DESCRIPTION("Xillybus driver for PCIe");
// MODULE_AUTHOR("Eli Billauer, Xillybus Ltd.");
// MODULE_ALIAS("xillybus_pcie");
// MODULE_LICENSE("GPL v2");

const PCI_DEVICE_ID_XILLYBUS: u16 = 0xebeb;
const PCI_VENDOR_ID_ACTEL: u16 = 0x11aa;
const PCI_VENDOR_ID_LATTICE: u16 = 0x1204;

static XILLYNAME: &[u8] = b"xillybus_pcie\0";

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u16,
    pub device: u16,
}

// PCI_VENDOR_ID_XILINX, PCI_VENDOR_ID_ALTERA, and kernel constants are supplied externally.
static XILLYIDS: [pci_device_id; 5] = [
    pci_device_id { vendor: PCI_VENDOR_ID_XILINX, device: PCI_DEVICE_ID_XILLYBUS },
    pci_device_id { vendor: PCI_VENDOR_ID_ALTERA, device: PCI_DEVICE_ID_XILLYBUS },
    pci_device_id { vendor: PCI_VENDOR_ID_ACTEL, device: PCI_DEVICE_ID_XILLYBUS },
    pci_device_id { vendor: PCI_VENDOR_ID_LATTICE, device: PCI_DEVICE_ID_XILLYBUS },
    pci_device_id { vendor: 0, device: 0 },
];

#[repr(C)]
pub struct device;

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: i32,
}

#[repr(C)]
pub struct xilly_endpoint {
    pub dev: *mut device,
    pub owner: *mut core::ffi::c_void,
    pub registers: *mut core::ffi::c_void,
    pub dma_using_dac: i32,
}

extern "C" {
    static THIS_MODULE: core::ffi::c_void;
    static PCI_VENDOR_ID_XILINX: u16;
    static PCI_VENDOR_ID_ALTERA: u16;

    fn xillybus_init_endpoint(dev: *mut device) -> *mut xilly_endpoint;
    fn pci_set_drvdata(pdev: *mut pci_dev, data: *mut xilly_endpoint);
    fn pcim_enable_device(pdev: *mut pci_dev) -> i32;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn pci_disable_link_state(pdev: *mut pci_dev, state: u32);
    fn pci_resource_flags(pdev: *mut pci_dev, bar: u32) -> u64;
    fn pcim_iomap_regions(pdev: *mut pci_dev, mask: u32, name: *const u8) -> i32;
    fn pcim_iomap_table(pdev: *mut pci_dev) -> *mut *mut core::ffi::c_void;
    fn pci_set_master(pdev: *mut pci_dev);
    fn pci_enable_msi(pdev: *mut pci_dev) -> i32;
    fn devm_request_irq(
        dev: *mut device,
        irq: i32,
        handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
        flags: u64,
        name: *const u8,
        data: *mut xilly_endpoint,
    ) -> i32;
    fn xillybus_isr(irq: i32, data: *mut core::ffi::c_void) -> i32;
    fn dma_set_mask(dev: *mut device, mask: u64) -> i32;
    fn xillybus_endpoint_discovery(endpoint: *mut xilly_endpoint) -> i32;
    fn pci_get_drvdata(pdev: *mut pci_dev) -> *mut xilly_endpoint;
    fn xillybus_endpoint_remove(endpoint: *mut xilly_endpoint);
}

const ENOMEM: i32 = 12;
const ENODEV: i32 = 19;
const PCIE_LINK_STATE_L0S: u32 = 1;
const IORESOURCE_MEM: u64 = 0x00000200;

unsafe extern "C" fn xilly_probe(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    let endpoint = xillybus_init_endpoint(&mut (*pdev).dev);

    if endpoint.is_null() {
        return -ENOMEM;
    }

    pci_set_drvdata(pdev, endpoint);
    (*endpoint).owner = &THIS_MODULE as *const _ as *mut _;

    let mut rc = pcim_enable_device(pdev);
    if rc != 0 {
        dev_err((*endpoint).dev, b"pcim_enable_device() failed. Aborting.\n\0".as_ptr());
        return rc;
    }

    /* L0s has caused packet drops. No power saving, thank you. */
    pci_disable_link_state(pdev, PCIE_LINK_STATE_L0S);

    if pci_resource_flags(pdev, 0) & IORESOURCE_MEM == 0 {
        dev_err((*endpoint).dev, b"Incorrect BAR configuration. Aborting.\n\0".as_ptr());
        return -ENODEV;
    }

    rc = pcim_iomap_regions(pdev, 0x01, XILLYNAME.as_ptr());
    if rc != 0 {
        dev_err((*endpoint).dev, b"pcim_iomap_regions() failed. Aborting.\n\0".as_ptr());
        return rc;
    }

    (*endpoint).registers = *pcim_iomap_table(pdev);
    pci_set_master(pdev);

    /* Set up a single MSI interrupt */
    if pci_enable_msi(pdev) != 0 {
        dev_err((*endpoint).dev, b"Failed to enable MSI interrupts. Aborting.\n\0".as_ptr());
        return -ENODEV;
    }
    rc = devm_request_irq(&mut (*pdev).dev, (*pdev).irq, xillybus_isr, 0,
                          XILLYNAME.as_ptr(), endpoint);
    if rc != 0 {
        return -ENODEV;
    }

    /*
     * Some (old and buggy?) hardware drops 64-bit addressed PCIe packets,
     * even when the PCIe driver claims that a 64-bit mask is OK. On the
     * other hand, on some architectures, 64-bit addressing is mandatory.
     * So go for the 64-bit mask only when failing is the other option.
     */
    if dma_set_mask(&mut (*pdev).dev, (1u64 << 32) - 1) == 0 {
        (*endpoint).dma_using_dac = 0;
    } else if dma_set_mask(&mut (*pdev).dev, u64::MAX) == 0 {
        (*endpoint).dma_using_dac = 1;
    } else {
        dev_err((*endpoint).dev, b"Failed to set DMA mask. Aborting.\n\0".as_ptr());
        return -ENODEV;
    }

    xillybus_endpoint_discovery(endpoint)
}

unsafe extern "C" fn xilly_remove(pdev: *mut pci_dev) {
    let endpoint = pci_get_drvdata(pdev);
    xillybus_endpoint_remove(endpoint);
}

// MODULE_DEVICE_TABLE(pci, xillyids);

#[repr(C)]
struct pci_driver {
    name: *const u8,
    id_table: *const pci_device_id,
    probe: unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> i32,
    remove: unsafe extern "C" fn(*mut pci_dev),
}

static XILLYBUS_DRIVER: pci_driver = pci_driver {
    name: XILLYNAME.as_ptr(),
    id_table: XILLYIDS.as_ptr(),
    probe: xilly_probe,
    remove: xilly_remove,
};

// module_pci_driver(xillybus_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
