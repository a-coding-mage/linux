// SPDX-License-Identifier: GPL-2.0-only
/*
 * PCI driver for the High Speed UART DMA
 *
 * Copyright (C) 2015 Intel Corporation
 * Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
 *
 * Partially based on the bits found in drivers/tty/serial/mfd.c.
 */

// Linux header dependencies are supplied by the surrounding kernel bindings.

const HSU_PCI_DMASR: usize = 0x00;
const HSU_PCI_DMAISR: usize = 0x04;
const HSU_PCI_CHAN_OFFSET: usize = 0x100;

const PCI_DEVICE_ID_INTEL_MFLD_HSU_DMA: u16 = 0x081e;
const PCI_DEVICE_ID_INTEL_MRFLD_HSU_DMA: u16 = 0x1192;

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn hsu_dma_get_status(chip: *mut hsu_dma_chip, channel: u16, status: *mut u32) -> i32;
    fn hsu_dma_do_irq(chip: *mut hsu_dma_chip, channel: u16, status: u32) -> i32;
    fn hsu_dma_remove(chip: *mut core::ffi::c_void);
    fn pcim_enable_device(pdev: *mut pci_dev) -> i32;
    fn pcim_iomap_regions(pdev: *mut pci_dev, mask: u32, name: *const core::ffi::c_char) -> i32;
    fn pci_name(pdev: *mut pci_dev) -> *const core::ffi::c_char;
    fn pci_set_master(pdev: *mut pci_dev);
    fn pci_try_set_mwi(pdev: *mut pci_dev);
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn pci_alloc_irq_vectors(pdev: *mut pci_dev, min: i32, max: i32, flags: u32) -> i32;
    fn pcim_iomap_table(pdev: *mut pci_dev) -> *mut *mut core::ffi::c_void;
    fn pci_resource_len(pdev: *mut pci_dev, bar: u32) -> usize;
    fn pci_irq_vector(pdev: *mut pci_dev, nr: u32) -> i32;
    fn hsu_dma_probe(chip: *mut hsu_dma_chip) -> i32;
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut core::ffi::c_void), data: *mut core::ffi::c_void) -> i32;
    fn devm_request_irq(dev: *mut device, irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t, flags: u32, name: *const core::ffi::c_char, data: *mut core::ffi::c_void) -> i32;
    fn disable_irq_nosync(irq: i32);
    fn pci_set_drvdata(pdev: *mut pci_dev, data: *mut core::ffi::c_void);
}

#[repr(C)]
struct device;
#[repr(C)]
struct pci_dev { device: device, device_id: u16 }
#[repr(C)]
struct pci_device_id;
#[repr(C)]
struct hsu_dma { nr_channels: u16 }
#[repr(C)]
struct hsu_dma_chip {
    dev: *mut device,
    regs: *mut core::ffi::c_void,
    length: usize,
    offset: usize,
    irq: i32,
    hsu: *mut hsu_dma,
}
type irqreturn_t = i32;

unsafe extern "C" fn hsu_pci_irq(_irq: i32, dev: *mut core::ffi::c_void) -> irqreturn_t {
    let chip = dev as *mut hsu_dma_chip;
    let dmaisr = readl((*chip).regs.add(HSU_PCI_DMAISR));
    let mut ret = 0i32;
    let channels = (*(*chip).hsu).nr_channels;
    let mut i = 0u16;
    while i < channels {
        if (dmaisr & (1u32 << i)) != 0 {
            let mut status = 0u32;
            let err = hsu_dma_get_status(chip, i, &mut status);
            if err > 0 { ret |= 1; }
            else if err == 0 { ret |= hsu_dma_do_irq(chip, i, status); }
        }
        i = i.wrapping_add(1);
    }
    ret
}

unsafe extern "C" fn hsu_pci_dma_remove(chip: *mut core::ffi::c_void) {
    hsu_dma_remove(chip);
}

unsafe extern "C" fn hsu_pci_probe(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    let dev = &mut (*pdev).device as *mut device;
    let ret = pcim_enable_device(pdev);
    if ret != 0 { return ret; }
    let ret = pcim_iomap_regions(pdev, 1u32 << 0, pci_name(pdev));
    if ret != 0 { return ret; }
    pci_set_master(pdev);
    pci_try_set_mwi(pdev);
    let ret = dma_set_mask_and_coherent(dev, 0xffff_ffff);
    if ret != 0 { return ret; }
    let chip = devm_kzalloc(dev, core::mem::size_of::<hsu_dma_chip>(), 0) as *mut hsu_dma_chip;
    if chip.is_null() { return -12; }
    let ret = pci_alloc_irq_vectors(pdev, 1, 1, 0xffff_ffff);
    if ret < 0 { return ret; }
    (*chip).dev = dev;
    (*chip).regs = *pcim_iomap_table(pdev);
    (*chip).length = pci_resource_len(pdev, 0);
    (*chip).offset = HSU_PCI_CHAN_OFFSET;
    (*chip).irq = pci_irq_vector(pdev, 0);
    let ret = hsu_dma_probe(chip);
    if ret != 0 { return ret; }
    let ret = devm_add_action_or_reset(dev, hsu_pci_dma_remove, chip as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    let ret = devm_request_irq(dev, (*chip).irq, hsu_pci_irq, 0, b"hsu_dma_pci\0".as_ptr() as *const _, chip as *mut _);
    if ret != 0 { return ret; }
    if (*pdev).device_id == PCI_DEVICE_ID_INTEL_MRFLD_HSU_DMA { disable_irq_nosync((*chip).irq); }
    pci_set_drvdata(pdev, chip as *mut _);
    0
}

// The PCI ID table, driver registration, and module metadata are provided by the kernel bindings.
#[repr(C)]
struct pci_device_id_entry { vendor: u16, device: u16, driver_data: usize }

static HSU_PCI_ID_TABLE: [pci_device_id_entry; 3] = [
    pci_device_id_entry { vendor: 0x8086, device: PCI_DEVICE_ID_INTEL_MFLD_HSU_DMA, driver_data: 0 },
    pci_device_id_entry { vendor: 0x8086, device: PCI_DEVICE_ID_INTEL_MRFLD_HSU_DMA, driver_data: 0 },
    pci_device_id_entry { vendor: 0, device: 0, driver_data: 0 },
];

#[repr(C)]
struct pci_driver {
    name: *const core::ffi::c_char,
    id_table: *const pci_device_id_entry,
    probe: unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> i32,
}

static mut HSU_PCI_DRIVER: pci_driver = pci_driver {
    name: b"hsu_dma_pci\0".as_ptr() as *const _,
    id_table: HSU_PCI_ID_TABLE.as_ptr(),
    probe: hsu_pci_probe,
};

// module_pci_driver(hsu_pci_driver)
// MODULE_DEVICE_TABLE(pci, hsu_pci_id_table)
// MODULE_LICENSE("GPL v2")
// MODULE_DESCRIPTION("High Speed UART DMA PCI driver")
// MODULE_AUTHOR("Andy Shevchenko <andriy.shevchenko@linux.intel.com>")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
