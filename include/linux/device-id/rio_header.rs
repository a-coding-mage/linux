/* SPDX-License-Identifier: GPL-2.0 */

/* RapidIO */

pub const RIO_ANY_ID: u16 = 0xffff;

/**
 * struct rio_device_id - RIO device identifier
 * @did: RapidIO device ID
 * @vid: RapidIO vendor ID
 * @asm_did: RapidIO assembly device ID
 * @asm_vid: RapidIO assembly vendor ID
 *
 * Identifies a RapidIO device based on both the device/vendor IDs and
 * the assembly device/vendor IDs.
 */
#[repr(C)]
pub struct rio_device_id {
    pub did: u16,
    pub vid: u16,
    pub asm_did: u16,
    pub asm_vid: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
