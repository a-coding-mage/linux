/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * linux/include/linux/ppdev.h
 *
 * User-space parallel port device driver (header file).
 *
 * Copyright (C) 1998-9 Tim Waugh <tim@cyberelk.demon.co.uk>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 *
 * Added PPGETTIME/PPSETTIME, Fred Barnes, 1999
 * Added PPGETMODES/PPGETMODE/PPGETPHASE, Fred Barnes <frmb2@ukc.ac.uk>, 03/01/2001
 */

// `_IO`, `_IOR`, `_IOW`, and `OBSOLETE__*` are supplied by the platform's
// ioctl definitions.  The following declarations preserve the header's ABI.
pub const PP_IOCTL: u8 = b'p';

#[repr(C)]
pub struct ppdev_frob_struct {
    pub mask: u8,
    pub val: u8,
}

pub const PPSETMODE: u32 = _IOW(PP_IOCTL, 0x80, core::mem::size_of::<i32>());
pub const PPRSTATUS: u32 = _IOR(PP_IOCTL, 0x81, core::mem::size_of::<u8>());
pub const PPWSTATUS: u32 = OBSOLETE__IOW(PP_IOCTL, 0x82, core::mem::size_of::<u8>());
pub const PPRCONTROL: u32 = _IOR(PP_IOCTL, 0x83, core::mem::size_of::<u8>());
pub const PPWCONTROL: u32 = _IOW(PP_IOCTL, 0x84, core::mem::size_of::<u8>());
pub const PPFCONTROL: u32 = _IOW(PP_IOCTL, 0x8e, core::mem::size_of::<ppdev_frob_struct>());
pub const PPRDATA: u32 = _IOR(PP_IOCTL, 0x85, core::mem::size_of::<u8>());
pub const PPWDATA: u32 = _IOW(PP_IOCTL, 0x86, core::mem::size_of::<u8>());
pub const PPRECONTROL: u32 = OBSOLETE__IOR(PP_IOCTL, 0x87, core::mem::size_of::<u8>());
pub const PPWECONTROL: u32 = OBSOLETE__IOW(PP_IOCTL, 0x88, core::mem::size_of::<u8>());
pub const PPRFIFO: u32 = OBSOLETE__IOR(PP_IOCTL, 0x89, core::mem::size_of::<u8>());
pub const PPWFIFO: u32 = OBSOLETE__IOW(PP_IOCTL, 0x8a, core::mem::size_of::<u8>());
pub const PPCLAIM: u32 = _IO(PP_IOCTL, 0x8b);
pub const PPRELEASE: u32 = _IO(PP_IOCTL, 0x8c);
pub const PPYIELD: u32 = _IO(PP_IOCTL, 0x8d);
pub const PPEXCL: u32 = _IO(PP_IOCTL, 0x8f);
pub const PPDATADIR: u32 = _IOW(PP_IOCTL, 0x90, core::mem::size_of::<i32>());
pub const PPNEGOT: u32 = _IOW(PP_IOCTL, 0x91, core::mem::size_of::<i32>());
pub const PPWCTLONIRQ: u32 = _IOW(PP_IOCTL, 0x92, core::mem::size_of::<u8>());
pub const PPCLRIRQ: u32 = _IOR(PP_IOCTL, 0x93, core::mem::size_of::<i32>());
pub const PPSETPHASE: u32 = _IOW(PP_IOCTL, 0x94, core::mem::size_of::<i32>());
// `struct timeval` is an external platform type.
pub const PPGETTIME: u32 = _IOR(PP_IOCTL, 0x95, core::mem::size_of::<crate::timeval>());
pub const PPSETTIME: u32 = _IOW(PP_IOCTL, 0x96, core::mem::size_of::<crate::timeval>());
pub const PPGETMODES: u32 = _IOR(PP_IOCTL, 0x97, core::mem::size_of::<u32>());
pub const PPGETMODE: u32 = _IOR(PP_IOCTL, 0x98, core::mem::size_of::<i32>());
pub const PPGETPHASE: u32 = _IOR(PP_IOCTL, 0x99, core::mem::size_of::<i32>());
pub const PPGETFLAGS: u32 = _IOR(PP_IOCTL, 0x9a, core::mem::size_of::<i32>());
pub const PPSETFLAGS: u32 = _IOW(PP_IOCTL, 0x9b, core::mem::size_of::<i32>());

pub const PP_FASTWRITE: u32 = 1 << 2;
pub const PP_FASTREAD: u32 = 1 << 3;
pub const PP_W91284PIC: u32 = 1 << 4;
pub const PP_FLAGMASK: u32 = PP_FASTWRITE | PP_FASTREAD | PP_W91284PIC;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
