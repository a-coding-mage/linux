/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the Linux UAPI header `papr-hvpipe.h`.
// Required types and ioctl helpers are supplied by the surrounding bindings.

/*
 * This header is included in payload between OS and the user
 * space.
 * flags: OS notifies the user space whether the hvpipe is
 *        closed or the buffer has the payload.
 */
#[repr(C)]
pub struct papr_hvpipe_hdr {
    pub version: u8,
    pub reserved: [u8; 3],
    pub flags: u32,
    pub reserved2: [u8; 40],
}

/*
 * ioctl for /dev/papr-hvpipe
 */
pub const PAPR_HVPIPE_IOC_CREATE_HANDLE: u32 =
    _IOW(PAPR_MISCDEV_IOC_ID, 9, core::mem::size_of::<u32>() as u32);

/*
 * hvpipe_hdr flags used for read()
 */
pub const HVPIPE_MSG_AVAILABLE: u32 = 0x01; // Payload is available
pub const HVPIPE_LOST_CONNECTION: u32 = 0x02; // Pipe connection is closed/unavailable

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
