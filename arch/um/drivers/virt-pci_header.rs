/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header.  The PCI definitions and PCI_STD_NUM_BARS
// are supplied by the corresponding Linux PCI dependency.

use core::ffi::{c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct um_pci_device {
    pub ops: *const um_pci_ops,

    /* for now just standard BARs */
    pub resptr: [u8; PCI_STD_NUM_BARS],

    pub irq: c_int,
}

#[repr(C)]
pub struct um_pci_ops {
    pub cfgspace_read:
        Option<unsafe extern "C" fn(dev: *mut um_pci_device, offset: c_uint, size: c_int) -> c_ulong>,
    pub cfgspace_write: Option<
        unsafe extern "C" fn(
            dev: *mut um_pci_device,
            offset: c_uint,
            size: c_int,
            val: c_ulong,
        ),
    >,

    pub bar_read: Option<
        unsafe extern "C" fn(
            dev: *mut um_pci_device,
            bar: c_int,
            offset: c_uint,
            size: c_int,
        ) -> c_ulong,
    >,
    pub bar_write: Option<
        unsafe extern "C" fn(
            dev: *mut um_pci_device,
            bar: c_int,
            offset: c_uint,
            size: c_int,
            val: c_ulong,
        ),
    >,

    pub bar_copy_from: Option<
        unsafe extern "C" fn(
            dev: *mut um_pci_device,
            bar: c_int,
            buffer: *mut c_void,
            offset: c_uint,
            size: c_int,
        ),
    >,
    pub bar_copy_to: Option<
        unsafe extern "C" fn(
            dev: *mut um_pci_device,
            bar: c_int,
            offset: c_uint,
            buffer: *const c_void,
            size: c_int,
        ),
    >,
    pub bar_set: Option<
        unsafe extern "C" fn(
            dev: *mut um_pci_device,
            bar: c_int,
            offset: c_uint,
            value: u8,
            size: c_int,
        ),
    >,
}

unsafe extern "C" {
    pub fn um_pci_device_register(dev: *mut um_pci_device) -> c_int;
    pub fn um_pci_device_unregister(dev: *mut um_pci_device);

    pub fn um_pci_platform_device_register(dev: *mut um_pci_device) -> c_int;
    pub fn um_pci_platform_device_unregister(dev: *mut um_pci_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
