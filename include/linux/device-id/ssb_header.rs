/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <linux/types.h> under __KERNEL__; the corresponding
// integer types are represented directly by Rust's fixed-width integers here.

pub const SSB_ANY_VENDOR: u16 = 0xFFFF;
pub const SSB_ANY_ID: u16 = 0xFFFF;
pub const SSB_ANY_REV: u8 = 0xFF;

/* SSB core, see drivers/ssb/ */
#[repr(C, packed(2))]
pub struct ssb_device_id {
    pub vendor: u16,
    pub coreid: u16,
    pub revision: u8,
    pub __pad: u8,
}

#[macro_export]
macro_rules! SSB_DEVICE {
    ($vendor:expr, $coreid:expr, $revision:expr) => {
        $crate::ssb_device_id {
            vendor: $vendor,
            coreid: $coreid,
            revision: $revision,
            __pad: 0,
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
