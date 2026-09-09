/* SPDX-License-Identifier: GPL-2.0 */

// Depends on the Linux UAPI integer types supplied by the surrounding code.

#[repr(C)]
pub struct timens_offset {
    pub sec: i64,
    pub nsec: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
