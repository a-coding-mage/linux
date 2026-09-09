/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C header dependency: <linux/ioctl.h>

/* /dev/nvram ioctls */
// _IO('p', nr): no data transfer, type 'p', command number nr.
pub const NVRAM_INIT: u32 = (('p' as u32) << 8) | 0x40;
pub const NVRAM_SETCKS: u32 = (('p' as u32) << 8) | 0x41;

/* for all current systems, this is where NVRAM starts */
pub const NVRAM_FIRST_BYTE: i32 = 14;

/* all these functions expect an NVRAM offset, not an absolute */
#[macro_export]
macro_rules! NVRAM_OFFSET {
    ($x:expr) => {
        ($x) - $crate::NVRAM_FIRST_BYTE
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
