/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header guard: LINUX_DEVICE_ID_ISHTP_H
 *
 * The kernel-only include and typedef are supplied by the surrounding Rust
 * environment when building the kernel configuration.
 */

/* ISHTP (Integrated Sensor Hub Transport Protocol) */

pub const ISHTP_MODULE_PREFIX: &str = "ishtp:";

/**
 * struct ishtp_device_id - ISHTP device identifier
 * @guid: GUID of the device.
 * @driver_data: pointer to driver specific data
 */
#[repr(C)]
pub struct ishtp_device_id {
    pub guid: guid_t,
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
