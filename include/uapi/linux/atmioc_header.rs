/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* atmioc.h - ranges for ATM-related ioctl numbers */

/* Written 1995-1999 by Werner Almesberger, EPFL LRC/ICA */

/*
 * See https://icawww1.epfl.ch/linux-atm/magic.html for the complete list of
 * "magic" ioctl numbers.
 */

/*
 * The C header includes <asm/ioctl.h>; users of this translation will also
 * need the corresponding _IO{,R,W,WR} definitions.
 */

pub const ATMIOC_PHYCOM: u32 = 0x00; /* PHY device common ioctls, globally unique */
pub const ATMIOC_PHYCOM_END: u32 = 0x0f;
pub const ATMIOC_PHYTYP: u32 = 0x10; /* PHY dev type ioctls, unique per PHY type */
pub const ATMIOC_PHYTYP_END: u32 = 0x2f;
pub const ATMIOC_PHYPRV: u32 = 0x30; /* PHY dev private ioctls, unique per driver */
pub const ATMIOC_PHYPRV_END: u32 = 0x4f;
pub const ATMIOC_SARCOM: u32 = 0x50; /* SAR device common ioctls, globally unique */
pub const ATMIOC_SARCOM_END: u32 = 0x50;
pub const ATMIOC_SARPRV: u32 = 0x60; /* SAR dev private ioctls, unique per driver */
pub const ATMIOC_SARPRV_END: u32 = 0x7f;
pub const ATMIOC_ITF: u32 = 0x80; /* Interface ioctls, globally unique */
pub const ATMIOC_ITF_END: u32 = 0x8f;
pub const ATMIOC_BACKEND: u32 = 0x90; /* ATM generic backend ioctls, u. per backend */
pub const ATMIOC_BACKEND_END: u32 = 0xaf;
/* 0xb0-0xbf: Reserved for future use */
pub const ATMIOC_AREQUIPA: u32 = 0xc0; /* Application requested IP over ATM, glob. u. */
pub const ATMIOC_LANE: u32 = 0xd0; /* LAN Emulation, globally unique */
pub const ATMIOC_MPOA: u32 = 0xd8; /* MPOA, globally unique */
pub const ATMIOC_CLIP: u32 = 0xe0; /* Classical IP over ATM control, globally u. */
pub const ATMIOC_CLIP_END: u32 = 0xef;
pub const ATMIOC_SPECIAL: u32 = 0xf0; /* Special-purpose controls, globally unique */
pub const ATMIOC_SPECIAL_END: u32 = 0xff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
