/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header guard is omitted in Rust; module inclusion provides the
 * corresponding uniqueness semantics.
 */

/*
 * In the C kernel build, `kernel_ulong_t` is defined as `unsigned long`.
 * `__u32` and `kernel_ulong_t` are supplied by the surrounding kernel
 * environment.
 */

/**
 * struct coreboot_device_id - Identifies a coreboot table entry
 * @tag: tag ID
 * @driver_data: driver specific data
 */
#[repr(C)]
pub struct coreboot_device_id {
    pub tag: __u32,
    pub driver_data: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
