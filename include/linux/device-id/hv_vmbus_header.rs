/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard is omitted from Rust; module inclusion provides the guard.

// Under __KERNEL__, these names are supplied by the kernel UUID definitions.
// typedef unsigned long kernel_ulong_t;

/*
 * For Hyper-V devices we use the device guid as the id.
 */
#[repr(C)]
pub struct hv_vmbus_device_id {
    pub guid: guid_t,
    pub driver_data: kernel_ulong_t, // Data private to the driver
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
