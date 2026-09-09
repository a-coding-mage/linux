/* SPDX-License-Identifier: GPL-2.0 */
/*
 * PCI Peer 2 Peer DMA support.
 *
 * Copyright (c) 2016-2018, Logan Gunthorpe
 * Copyright (c) 2016-2017, Microsemi Corporation
 * Copyright (c) 2017, Christoph Hellwig
 * Copyright (c) 2018, Eideticom Inc.
 */

// Dependency supplied by Linux PCI definitions: <linux/pci.h>

use core::ffi::c_char;

pub struct block_device;
pub struct scatterlist;
pub struct device;
pub struct pci_dev;
pub struct page;

pub type u64 = core::primitive::u64;
pub type u32 = core::primitive::u32;
pub type size_t = usize;
pub type ssize_t = isize;
pub type phys_addr_t = usize;
pub type dma_addr_t = usize;
pub type pci_bus_addr_t = usize;
pub type bool = core::primitive::bool;

#[repr(C)]
pub struct p2pdma_provider {
    pub owner: *mut device,
    pub bus_offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_p2pdma_map_type {
    PCI_P2PDMA_MAP_UNKNOWN = 0,
    PCI_P2PDMA_MAP_NONE,
    PCI_P2PDMA_MAP_NOT_SUPPORTED,
    PCI_P2PDMA_MAP_BUS_ADDR,
    PCI_P2PDMA_MAP_THRU_HOST_BRIDGE,
}

#[cfg(CONFIG_PCI_P2PDMA)]
extern "C" {
    pub fn pcim_p2pdma_init(pdev: *mut pci_dev) -> i32;
    pub fn pcim_p2pdma_provider(pdev: *mut pci_dev, bar: i32) -> *mut p2pdma_provider;
    pub fn pci_p2pdma_add_resource(pdev: *mut pci_dev, bar: i32, size: size_t, offset: u64) -> i32;
    pub fn pci_p2pdma_distance_many(provider: *mut pci_dev, clients: *mut *mut device, num_clients: i32, verbose: bool) -> i32;
    pub fn pci_p2pmem_find_many(clients: *mut *mut device, num_clients: i32) -> *mut pci_dev;
    pub fn pci_alloc_p2pmem(pdev: *mut pci_dev, size: size_t) -> *mut core::ffi::c_void;
    pub fn pci_free_p2pmem(pdev: *mut pci_dev, addr: *mut core::ffi::c_void, size: size_t);
    pub fn pci_p2pmem_virt_to_bus(pdev: *mut pci_dev, addr: *mut core::ffi::c_void) -> pci_bus_addr_t;
    pub fn pci_p2pmem_alloc_sgl(pdev: *mut pci_dev, nents: *mut u32, length: u32) -> *mut scatterlist;
    pub fn pci_p2pmem_free_sgl(pdev: *mut pci_dev, sgl: *mut scatterlist);
    pub fn pci_p2pmem_publish(pdev: *mut pci_dev, publish: bool);
    pub fn pci_p2pdma_enable_store(page: *const c_char, p2p_dev: *mut *mut pci_dev, use_p2pdma: *mut bool) -> i32;
    pub fn pci_p2pdma_enable_show(page: *mut c_char, p2p_dev: *mut pci_dev, use_p2pdma: bool) -> ssize_t;
    pub fn pci_p2pdma_map_type(provider: *mut p2pdma_provider, dev: *mut device) -> pci_p2pdma_map_type;
}

#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pcim_p2pdma_init(_: *mut pci_dev) -> i32 { -95 }
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pcim_p2pdma_provider(_: *mut pci_dev, _: i32) -> *mut p2pdma_provider { core::ptr::null_mut() }
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pci_p2pdma_add_resource(_: *mut pci_dev, _: i32, _: size_t, _: u64) -> i32 { -95 }
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pci_p2pdma_distance_many(_: *mut pci_dev, _: *mut *mut device, _: i32, _: bool) -> i32 { -1 }
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pci_p2pmem_find_many(_: *mut *mut device, _: i32) -> *mut pci_dev { core::ptr::null_mut() }
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pci_alloc_p2pmem(_: *mut pci_dev, _: size_t) -> *mut core::ffi::c_void { core::ptr::null_mut() }
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pci_free_p2pmem(_: *mut pci_dev, _: *mut core::ffi::c_void, _: size_t) {}
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pci_p2pmem_virt_to_bus(_: *mut pci_dev, _: *mut core::ffi::c_void) -> pci_bus_addr_t { 0 }
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pci_p2pmem_alloc_sgl(_: *mut pci_dev, _: *mut u32, _: u32) -> *mut scatterlist { core::ptr::null_mut() }
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pci_p2pmem_free_sgl(_: *mut pci_dev, _: *mut scatterlist) {}
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pci_p2pmem_publish(_: *mut pci_dev, _: bool) {}
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pci_p2pdma_enable_store(_: *const c_char, _: *mut *mut pci_dev, use_p2pdma: *mut bool) -> i32 { *use_p2pdma = false; 0 }
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pci_p2pdma_enable_show(page: *mut c_char, _: *mut pci_dev, _: bool) -> ssize_t {
    // C implementation calls sprintf(page, "none\\n").
    let bytes = b"none\n";
    core::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, page, bytes.len());
    bytes.len() as ssize_t
}
#[cfg(not(CONFIG_PCI_P2PDMA))]
pub unsafe fn pci_p2pdma_map_type(_: *mut p2pdma_provider, _: *mut device) -> pci_p2pdma_map_type { pci_p2pdma_map_type::PCI_P2PDMA_MAP_NOT_SUPPORTED }

pub unsafe fn pci_p2pdma_distance(provider: *mut pci_dev, client: *mut device, verbose: bool) -> i32 {
    pci_p2pdma_distance_many(provider, &mut (client as *mut device), 1, verbose)
}

pub unsafe fn pci_p2pmem_find(client: *mut device) -> *mut pci_dev {
    pci_p2pmem_find_many(&mut (client as *mut device), 1)
}

#[repr(C)]
pub struct pci_p2pdma_map_state {
    pub mem: *mut p2pdma_provider,
    pub map: pci_p2pdma_map_type,
}

extern "C" {
    pub fn __pci_p2pdma_update_state(state: *mut pci_p2pdma_map_state, dev: *mut device, page: *mut page);
}

// External helpers supplied by the kernel.
extern "C" {
    pub fn IS_ENABLED(config: i32) -> bool;
    pub fn is_pci_p2pdma_page(page: *mut page) -> bool;
}

pub unsafe fn pci_p2pdma_state(state: *mut pci_p2pdma_map_state, dev: *mut device, page: *mut page) -> pci_p2pdma_map_type {
    if IS_ENABLED(0) && is_pci_p2pdma_page(page) {
        __pci_p2pdma_update_state(state, dev, page);
        return (*state).map;
    }
    pci_p2pdma_map_type::PCI_P2PDMA_MAP_NONE
}

pub unsafe fn pci_p2pdma_bus_addr_map(provider: *mut p2pdma_provider, paddr: phys_addr_t) -> dma_addr_t {
    paddr.wrapping_add((*provider).bus_offset as dma_addr_t)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
