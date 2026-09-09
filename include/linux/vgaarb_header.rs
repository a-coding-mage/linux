/* SPDX-License-Identifier: MIT */

/*
 * The VGA aribiter manages VGA space routing and VGA resource decode to
 * allow multiple VGA devices to be used in a system in a safe way.
 *
 * (C) Copyright 2005 Benjamin Herrenschmidt <benh@kernel.crashing.org>
 * (C) Copyright 2007 Paulo R. Zanoni <przanoni@gmail.com>
 * (C) Copyright 2007, 2009 Tiago Vignatti <vignatti@freedesktop.org>
 */

// Dependency supplied by the surrounding kernel translation.

#[repr(C)]
pub struct pci_dev;

/* Legacy VGA regions */
pub const VGA_RSRC_NONE: u32 = 0x00;
pub const VGA_RSRC_LEGACY_IO: u32 = 0x01;
pub const VGA_RSRC_LEGACY_MEM: u32 = 0x02;
pub const VGA_RSRC_LEGACY_MASK: u32 = VGA_RSRC_LEGACY_IO | VGA_RSRC_LEGACY_MEM;
/* Non-legacy access */
pub const VGA_RSRC_NORMAL_IO: u32 = 0x04;
pub const VGA_RSRC_NORMAL_MEM: u32 = 0x08;

// CONFIG_VGA_ARB condition from the original header.
#[cfg(CONFIG_VGA_ARB)]
extern "C" {
    pub fn vga_set_legacy_decoding(pdev: *mut pci_dev, decodes: u32);
    pub fn vga_get(pdev: *mut pci_dev, rsrc: u32, interruptible: i32) -> i32;
    pub fn vga_put(pdev: *mut pci_dev, rsrc: u32);
    pub fn vga_default_device() -> *mut pci_dev;
    pub fn vga_set_default_device(pdev: *mut pci_dev);
    pub fn vga_remove_vgacon(pdev: *mut pci_dev) -> i32;
    pub fn vga_client_register(
        pdev: *mut pci_dev,
        set_decode: Option<unsafe extern "C" fn(pdev: *mut pci_dev, state: bool) -> u32>,
    ) -> i32;
}

#[cfg(not(CONFIG_VGA_ARB))]
pub unsafe fn vga_set_legacy_decoding(_pdev: *mut pci_dev, _decodes: u32) {}

#[cfg(not(CONFIG_VGA_ARB))]
pub unsafe fn vga_get(_pdev: *mut pci_dev, _rsrc: u32, _interruptible: i32) -> i32 {
    0
}

#[cfg(not(CONFIG_VGA_ARB))]
pub unsafe fn vga_put(_pdev: *mut pci_dev, _rsrc: u32) {}

#[cfg(not(CONFIG_VGA_ARB))]
pub unsafe fn vga_default_device() -> *mut pci_dev {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_VGA_ARB))]
pub unsafe fn vga_set_default_device(_pdev: *mut pci_dev) {}

#[cfg(not(CONFIG_VGA_ARB))]
pub unsafe fn vga_remove_vgacon(_pdev: *mut pci_dev) -> i32 {
    0
}

#[cfg(not(CONFIG_VGA_ARB))]
pub unsafe fn vga_client_register(
    _pdev: *mut pci_dev,
    _set_decode: Option<unsafe extern "C" fn(pdev: *mut pci_dev, state: bool) -> u32>,
) -> i32 {
    0
}

/**
 * vga_get_interruptible
 * @pdev: pci device of the VGA card or NULL for the system default
 * @rsrc: bit mask of resources to acquire and lock
 *
 * Shortcut to vga_get with interruptible set to true.
 *
 * On success, release the VGA resource again with vga_put().
 */
pub unsafe fn vga_get_interruptible(pdev: *mut pci_dev, rsrc: u32) -> i32 {
    vga_get(pdev, rsrc, 1)
}

/**
 * vga_get_uninterruptible - shortcut to vga_get()
 * @pdev: pci device of the VGA card or NULL for the system default
 * @rsrc: bit mask of resources to acquire and lock
 *
 * Shortcut to vga_get with interruptible set to false.
 *
 * On success, release the VGA resource again with vga_put().
 */
pub unsafe fn vga_get_uninterruptible(pdev: *mut pci_dev, rsrc: u32) -> i32 {
    vga_get(pdev, rsrc, 0)
}

pub unsafe fn vga_client_unregister(pdev: *mut pci_dev) {
    vga_client_register(pdev, None);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
