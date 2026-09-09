/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright 2006 PathScale, Inc. All Rights Reserved. */

// C dependencies: linux/sizes.h, linux/types.h, linux/init.h, asm/io.h,
// and asm/page.h supply the referenced types and functions.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[cfg(not(feature = "__iowrite32_copy"))]
extern "C" {
    pub fn __iowrite32_copy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize);
}
extern "C" {
    pub fn __ioread32_copy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize);
}
#[cfg(not(feature = "__iowrite64_copy"))]
extern "C" {
    pub fn __iowrite64_copy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize);
}

#[cfg(feature = "CONFIG_MMU")]
extern "C" {
    pub fn ioremap_page_range(addr: usize, end: usize, phys_addr: phys_addr_t, prot: pgprot_t) -> i32;
    pub fn vmap_page_range(addr: usize, end: usize, phys_addr: phys_addr_t, prot: pgprot_t) -> i32;
}
#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe fn ioremap_page_range(_addr: usize, _end: usize, _phys_addr: phys_addr_t, _prot: pgprot_t) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe fn vmap_page_range(_addr: usize, _end: usize, _phys_addr: phys_addr_t, _prot: pgprot_t) -> i32 { 0 }

// Managed iomap interface.
#[cfg(feature = "CONFIG_HAS_IOPORT_MAP")]
extern "C" {
    pub fn devm_ioport_map(dev: *mut device, port: usize, nr: u32) -> *mut core::ffi::c_void;
    pub fn devm_ioport_unmap(dev: *mut device, addr: *mut core::ffi::c_void);
}
#[cfg(not(feature = "CONFIG_HAS_IOPORT_MAP"))]
pub unsafe fn devm_ioport_map(_dev: *mut device, _port: usize, _nr: u32) -> *mut core::ffi::c_void { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_HAS_IOPORT_MAP"))]
pub unsafe fn devm_ioport_unmap(_dev: *mut device, _addr: *mut core::ffi::c_void) {}

extern "C" {
    pub fn devm_ioremap(dev: *mut device, offset: resource_size_t, size: resource_size_t) -> *mut core::ffi::c_void;
    pub fn devm_ioremap_uc(dev: *mut device, offset: resource_size_t, size: resource_size_t) -> *mut core::ffi::c_void;
    pub fn devm_ioremap_wc(dev: *mut device, offset: resource_size_t, size: resource_size_t) -> *mut core::ffi::c_void;
    pub fn devm_iounmap(dev: *mut device, addr: *mut core::ffi::c_void);
    pub fn check_signature(io_addr: *const core::ffi::c_void, signature: *const u8, length: i32) -> i32;
    pub fn devm_ioremap_release(dev: *mut device, res: *mut core::ffi::c_void);
    pub fn devm_memremap(dev: *mut device, offset: resource_size_t, size: usize, flags: usize) -> *mut core::ffi::c_void;
    pub fn devm_memunmap(dev: *mut device, addr: *mut core::ffi::c_void);
    pub fn early_memremap_pgprot_adjust(phys_addr: resource_size_t, size: usize, prot: pgprot_t) -> pgprot_t;
}

#[cfg(feature = "CONFIG_PCI")]
#[cfg(not(feature = "pci_remap_cfgspace"))]
pub unsafe fn pci_remap_cfgspace(offset: phys_addr_t, size: usize) -> *mut core::ffi::c_void {
    let mapped = ioremap_np(offset, size);
    if !mapped.is_null() { mapped } else { ioremap(offset, size) }
}

// Architectures may override arch_has_dev_port(); the default is true.
#[cfg(not(feature = "arch_has_dev_port"))]
pub const fn arch_has_dev_port() -> i32 { 1 }

#[cfg(not(feature = "arch_phys_wc_add"))]
pub unsafe fn arch_phys_wc_add(_base: usize, _size: usize) -> i32 { 0 }
#[cfg(not(feature = "arch_phys_wc_add"))]
pub unsafe fn arch_phys_wc_del(_handle: i32) {}
#[cfg(not(feature = "arch_phys_wc_add"))]
pub unsafe fn arch_phys_wc_index(_handle: i32) -> i32 { -1 }

extern "C" {
    pub fn devm_arch_phys_wc_add(dev: *mut device, base: usize, size: usize) -> i32;
}

pub const MEMREMAP_WB: u32 = 1 << 0;
pub const MEMREMAP_WT: u32 = 1 << 1;
pub const MEMREMAP_WC: u32 = 1 << 2;
pub const MEMREMAP_ENC: u32 = 1 << 3;
pub const MEMREMAP_DEC: u32 = 1 << 4;

extern "C" {
    pub fn memremap(offset: resource_size_t, size: usize, flags: usize) -> *mut core::ffi::c_void;
    pub fn memunmap(addr: *mut core::ffi::c_void);
    pub fn devm_arch_io_reserve_memtype_wc(dev: *mut device, start: resource_size_t, size: resource_size_t) -> i32;
}

#[cfg(not(feature = "arch_io_reserve_memtype_wc"))]
pub unsafe fn arch_io_reserve_memtype_wc(_base: resource_size_t, _size: resource_size_t) -> i32 { 0 }
#[cfg(not(feature = "arch_io_reserve_memtype_wc"))]
pub unsafe fn arch_io_free_memtype_wc(_base: resource_size_t, _size: resource_size_t) {}

#[cfg(feature = "CONFIG_STRICT_DEVMEM")]
pub unsafe fn range_is_allowed(mut pfn: usize, size: usize) -> i32 {
    let from: u64 = (pfn as u64) << PAGE_SHIFT;
    let to = from.wrapping_add(size as u64);
    let mut cursor = from;
    while cursor < to {
        if devmem_is_allowed(pfn) == 0 { return 0; }
        cursor = cursor.wrapping_add(PAGE_SIZE as u64);
        pfn = pfn.wrapping_add(1);
    }
    1
}
#[cfg(not(feature = "CONFIG_STRICT_DEVMEM"))]
pub unsafe fn range_is_allowed(_pfn: usize, _size: usize) -> i32 { 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
