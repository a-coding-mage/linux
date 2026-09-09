/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *   Focusrite Scarlett 2 Protocol Driver for ALSA
 *   (including Scarlett 2nd Gen, 3rd Gen, 4th Gen, Clarett USB, and
 *   Clarett+ series products)
 *
 *   Copyright (c) 2023 by Geoffrey D. Bennett <g at b4.vu>
 */

// Dependency intent from the C header: <linux/types.h>, <linux/ioctl.h>.

pub const SCARLETT2_HWDEP_MAJOR: u32 = 1;
pub const SCARLETT2_HWDEP_MINOR: u32 = 0;
pub const SCARLETT2_HWDEP_SUBMINOR: u32 = 0;

pub const SCARLETT2_HWDEP_VERSION: u32 =
    (SCARLETT2_HWDEP_MAJOR << 16)
        | (SCARLETT2_HWDEP_MINOR << 8)
        | SCARLETT2_HWDEP_SUBMINOR;

#[inline]
pub const fn SCARLETT2_HWDEP_VERSION_MAJOR(v: u32) -> u32 {
    (v >> 16) & 0xFF
}

#[inline]
pub const fn SCARLETT2_HWDEP_VERSION_MINOR(v: u32) -> u32 {
    (v >> 8) & 0xFF
}

#[inline]
pub const fn SCARLETT2_HWDEP_VERSION_SUBMINOR(v: u32) -> u32 {
    v & 0xFF
}

/* Get protocol version */
pub const SCARLETT2_IOCTL_PVERSION: u32 = crate::_IOR(b'S', 0x60, core::mem::size_of::<i32>());

/* Reboot */
pub const SCARLETT2_IOCTL_REBOOT: u32 = crate::_IO(b'S', 0x61);

/* Select flash segment */
pub const SCARLETT2_SEGMENT_ID_SETTINGS: u32 = 0;
pub const SCARLETT2_SEGMENT_ID_FIRMWARE: u32 = 1;
pub const SCARLETT2_SEGMENT_ID_COUNT: u32 = 2;

pub const SCARLETT2_IOCTL_SELECT_FLASH_SEGMENT: u32 =
    crate::_IOW(b'S', 0x62, core::mem::size_of::<i32>());

/* Erase selected flash segment */
pub const SCARLETT2_IOCTL_ERASE_FLASH_SEGMENT: u32 = crate::_IO(b'S', 0x63);

/* Get selected flash segment erase progress
 * 1 through to num_blocks, or 255 for complete
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scarlett2_flash_segment_erase_progress {
    pub progress: u8,
    pub num_blocks: u8,
}

pub const SCARLETT2_IOCTL_GET_ERASE_PROGRESS: u32 = crate::_IOR(
    b'S',
    0x64,
    core::mem::size_of::<scarlett2_flash_segment_erase_progress>(),
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
