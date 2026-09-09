/* SPDX-License-Identifier: GPL-2.0 */

/* The C header includes <linux/types.h> when __KERNEL__ is defined. */

pub const SERIO_ANY: u8 = 0xff;

#[repr(C)]
pub struct serio_device_id {
    pub type_: u8,
    pub extra: u8,
    pub id: u8,
    pub proto: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
