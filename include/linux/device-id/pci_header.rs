/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <linux/types.h> when building in the kernel.
// `__u32` is supplied by that dependency.
pub type kernel_ulong_t = usize;

pub const PCI_ANY_ID: u32 = u32::MAX;

pub const PCI_ID_F_VFIO_DRIVER_OVERRIDE: i32 = 1;

/**
 * struct pci_device_id - PCI device ID structure
 * @vendor:          Vendor ID to match (or PCI_ANY_ID)
 * @device:          Device ID to match (or PCI_ANY_ID)
 * @subvendor:       Subsystem vendor ID to match (or PCI_ANY_ID)
 * @subdevice:       Subsystem device ID to match (or PCI_ANY_ID)
 * @class:            Device class, subclass, and "interface" to match.
 *                    See Appendix D of the PCI Local Bus Spec or
 *                    include/linux/pci_ids.h for a full list of classes.
 *                    Most drivers do not need to specify class/class_mask
 *                    as vendor/device is normally sufficient.
 * @class_mask:      Limit which sub-fields of the class field are compared.
 *                    See drivers/scsi/sym53c8xx_2/ for example of usage.
 * @driver_data:     Data private to the driver.
 *                    Most drivers don't need to use driver_data field.
 *                    Best practice is to use driver_data as an index
 *                    into a static list of equivalent device types,
 *                    instead of using it as a pointer.
 * @override_only:    Match only when dev->driver_override is this driver.
 */
#[repr(C)]
pub struct pci_device_id {
    pub vendor: __u32,
    pub device: __u32,
    pub subvendor: __u32,
    pub subdevice: __u32,
    pub class: __u32,
    pub class_mask: __u32,
    pub driver_data: kernel_ulong_t,
    pub override_only: __u32,
}

/* pci_epf */

pub const PCI_EPF_NAME_SIZE: usize = 20;
pub const PCI_EPF_MODULE_PREFIX: &str = "pci_epf:";

#[repr(C)]
pub struct pci_epf_device_id {
    pub name: [core::ffi::c_char; PCI_EPF_NAME_SIZE],
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
