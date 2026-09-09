/* SPDX-License-Identifier: GPL-2.0 */

// Under the kernel build, `kernel_ulong_t` is supplied by the Linux type
// definitions and is an unsigned long.
#[cfg(feature = "kernel")]
pub type KernelULong = usize;

pub const ACPI_ID_LEN: usize = 16;

#[repr(C)]
pub struct acpi_device_id {
    pub id: [u8; ACPI_ID_LEN],
    pub driver_data: KernelULong,
    pub cls: u32,
    pub cls_msk: u32,
}

/**
 * ACPI_DEVICE_CLASS - macro used to describe an ACPI device with
 * the PCI-defined class-code information
 *
 * @_cls : the class, subclass, prog-if triple for this device
 * @_msk : the class mask for this device
 *
 * This macro is used to create a struct acpi_device_id that matches a
 * specific PCI class. The .id and .driver_data fields will be left
 * initialized with the default value.
 */
#[macro_export]
macro_rules! ACPI_DEVICE_CLASS {
    ($cls:expr, $msk:expr) => {
        cls: ($cls),
        cls_msk: ($msk),
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
