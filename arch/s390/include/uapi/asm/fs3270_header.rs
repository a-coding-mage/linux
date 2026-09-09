/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: linux/types.h and asm/ioctl.h provide the integer types
// and the _IO ioctl encoding macro used below.

/* ioctls for fullscreen 3270 */
pub const TUBICMD: u32 = _IO(b'3', 3); /* set ccw command for fs reads. */
pub const TUBOCMD: u32 = _IO(b'3', 4); /* set ccw command for fs writes. */
pub const TUBGETI: u32 = _IO(b'3', 7); /* get ccw command for fs reads. */
pub const TUBGETO: u32 = _IO(b'3', 8); /* get ccw command for fs writes. */
pub const TUBGETMOD: u32 = _IO(b'3', 13); /* get characteristics like model, cols, rows */

/* For TUBGETMOD */
#[repr(C)]
pub struct raw3270_iocb {
    pub model: u16,
    pub line_cnt: u16,
    pub col_cnt: u16,
    pub pf_cnt: u16,
    pub re_cnt: u16,
    pub map: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
