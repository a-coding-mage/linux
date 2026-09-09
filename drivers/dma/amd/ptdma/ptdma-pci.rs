// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Passthru DMA device driver
 * -- Based on the CCP driver
 *
 * Copyright (C) 2016,2021 Advanced Micro Devices, Inc.
 *
 * Author: Sanjay R Mehta <sanju.mehta@amd.com>
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 * Author: Gary R Hook <gary.hook@amd.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct pt_msix {
    pub msix_count: core::ffi::c_int,
    pub msix_entry: msix_entry,
}

#[repr(C)]
pub struct msix_entry {
    pub entry: u32,
    pub vector: u32,
}

// External kernel and PTDMA declarations are supplied by other translated files.
extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn init_list_head(list: *mut list_head);
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn pci_enable_msix_range(pdev: *mut pci_dev, entries: *mut msix_entry, min: i32, max: i32) -> i32;
    fn pci_disable_msix(pdev: *mut pci_dev);
    fn pci_enable_msi(pdev: *mut pci_dev) -> i32;
    fn pci_disable_msi(pdev: *mut pci_dev);
    fn pcim_enable_device(pdev: *mut pci_dev) -> i32;
    fn pci_select_bars(pdev: *mut pci_dev, flags: u32) -> i32;
    fn pcim_iomap_regions(pdev: *mut pci_dev, mask: i32, name: *const core::ffi::c_char) -> i32;
    fn pcim_iomap_table(pdev: *mut pci_dev) -> *mut *mut core::ffi::c_void;
    fn pci_set_master(pdev: *mut pci_dev);
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> i32;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn pt_core_init(pt: *mut pt_device) -> i32;
    fn pt_core_destroy(pt: *mut pt_device);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

#[repr(C)] pub struct device { pub private: [u8; 0] }
#[repr(C)] pub struct pci_dev { pub dev: device, pub irq: i32 }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct pt_dev_vdata { pub bar: usize }
#[repr(C)] pub struct pt_device {
    pub dev: *mut device,
    pub cmd: list_head,
    pub pt_msix: *mut pt_msix,
    pub pt_irq: i32,
    pub dev_vdata: *mut pt_dev_vdata,
    pub io_regs: *mut core::ffi::c_void,
}

unsafe fn pt_alloc_struct(dev: *mut device) -> *mut pt_device {
    let pt = devm_kzalloc(dev, core::mem::size_of::<pt_device>(), 0) as *mut pt_device;
    if pt.is_null() { return core::ptr::null_mut(); }
    (*pt).dev = dev;
    init_list_head(&mut (*pt).cmd);
    pt
}

unsafe fn pt_get_msix_irqs(pt: *mut pt_device) -> i32 {
    let pt_msix = (*pt).pt_msix;
    let pdev = to_pci_dev((*pt).dev);
    (*pt_msix).msix_entry.entry = 0;
    let ret = pci_enable_msix_range(pdev, &mut (*pt_msix).msix_entry, 1, 1);
    if ret < 0 { return ret; }
    (*pt_msix).msix_count = ret;
    (*pt).pt_irq = (*pt_msix).msix_entry.vector as i32;
    0
}

unsafe fn pt_get_msi_irq(pt: *mut pt_device) -> i32 {
    let pdev = to_pci_dev((*pt).dev);
    let ret = pci_enable_msi(pdev);
    if ret != 0 { return ret; }
    (*pt).pt_irq = (*pdev).irq;
    0
}

unsafe fn pt_get_irqs(pt: *mut pt_device) -> i32 {
    let ret = pt_get_msix_irqs(pt);
    if ret == 0 { return 0; }
    dev_err((*pt).dev, b"could not enable MSI-X (%d), trying MSI\0".as_ptr() as _, ret);
    let ret = pt_get_msi_irq(pt);
    if ret == 0 { return 0; }
    dev_err((*pt).dev, b"could not enable MSI (%d)\n\0".as_ptr() as _, ret);
    ret
}

unsafe fn pt_free_irqs(pt: *mut pt_device) {
    let pdev = to_pci_dev((*pt).dev);
    if (*(*pt).pt_msix).msix_count != 0 { pci_disable_msix(pdev); }
    else if (*pt).pt_irq != 0 { pci_disable_msi(pdev); }
    (*pt).pt_irq = 0;
}

unsafe fn pt_pci_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let mut ret: i32 = -12;
    let pt = pt_alloc_struct(dev);
    if pt.is_null() { goto_err(pt, dev, ret); return ret; }
    let pt_msix = devm_kzalloc(dev, core::mem::size_of::<pt_msix>(), 0) as *mut pt_msix;
    if pt_msix.is_null() { goto_err(pt, dev, ret); return ret; }
    (*pt).pt_msix = pt_msix;
    (*pt).dev_vdata = (*id).driver_data as *mut pt_dev_vdata;
    if (*pt).dev_vdata.is_null() { ret = -19; return probe_err(dev, ret); }
    ret = pcim_enable_device(pdev); if ret != 0 { return probe_err(dev, ret); }
    ret = pcim_iomap_regions(pdev, pci_select_bars(pdev, 0x20000000), b"ptdma\0".as_ptr() as _);
    if ret != 0 { return probe_err(dev, ret); }
    let iomap = pcim_iomap_table(pdev); if iomap.is_null() { ret = -12; return probe_err(dev, ret); }
    (*pt).io_regs = *iomap.add((*(*pt).dev_vdata).bar); if (*pt).io_regs.is_null() { ret = -12; return probe_err(dev, ret); }
    ret = pt_get_irqs(pt); if ret != 0 { return probe_err(dev, ret); }
    pci_set_master(pdev);
    ret = dma_set_mask_and_coherent(dev, (1u64 << 48).wrapping_sub(1));
    if ret != 0 { ret = dma_set_mask_and_coherent(dev, 0xffff_ffff); if ret != 0 { return probe_err(dev, ret); } }
    dev_set_drvdata(dev, pt as _);
    ret = pt_core_init(pt); if ret != 0 { return probe_err(dev, ret); }
    0
}

unsafe fn probe_err(dev: *mut device, ret: i32) -> i32 { dev_err(dev, b"initialization failed ret = %d\n\0".as_ptr() as _, ret); ret }
unsafe fn goto_err(_pt: *mut pt_device, dev: *mut device, ret: i32) { dev_err(dev, b"initialization failed ret = %d\n\0".as_ptr() as _, ret); }

unsafe fn pt_pci_remove(pdev: *mut pci_dev) {
    let dev = &mut (*pdev).dev as *mut device;
    let pt = dev_get_drvdata(dev) as *mut pt_device;
    if pt.is_null() { return; }
    if !(*pt).dev_vdata.is_null() { pt_core_destroy(pt); }
    pt_free_irqs(pt);
}

#[repr(C)] pub struct pci_device_id { pub driver_data: u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
