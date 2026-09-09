/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header conditionally includes <linux/uuid.h> for kernel builds.
 * The corresponding Rust definition of `uuid_t` is supplied externally.
 */

/**
 * struct tee_client_device_id - tee based device identifier
 * @uuid: For TEE based client devices we use the device uuid as
 *        the identifier.
 */
#[repr(C)]
pub struct tee_client_device_id {
    pub uuid: uuid_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
