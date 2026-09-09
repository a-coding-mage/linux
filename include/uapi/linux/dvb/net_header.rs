/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/*
 * net.h
 *
 * Copyright (C) 2000 Marcus Metzler <marcus@convergence.de>
 *                  & Ralph  Metzler <ralph@convergence.de>
 *                    for convergence integrated media GmbH
 */

// Dependency intent: the C header includes <linux/types.h>.

/// struct dvb_net_if - describes a DVB network interface
///
/// @pid: Packet ID (PID) of the MPEG-TS that contains data
/// @if_num: number of the Digital TV interface.
/// @feedtype: Encapsulation type of the feed.
///
/// A MPEG-TS stream may contain packet IDs with IP packages on it.
/// This struct describes it, and the type of encoding.
///
/// @feedtype can be:
///
///     - %DVB_NET_FEEDTYPE_MPE for MPE encoding
///     - %DVB_NET_FEEDTYPE_ULE for ULE encoding.
#[repr(C)]
pub struct dvb_net_if {
    pub pid: u16,
    pub if_num: u16,
    pub feedtype: u8,
}

pub const DVB_NET_FEEDTYPE_MPE: u8 = 0; // multi protocol encapsulation
pub const DVB_NET_FEEDTYPE_ULE: u8 = 1; // ultra lightweight encapsulation

// Dependency intent: _IOWR and _IO are supplied by the Linux ioctl definitions.
pub const NET_ADD_IF: u32 = _IOWR!(b'o', 52, dvb_net_if);
pub const NET_REMOVE_IF: u32 = _IO!(b'o', 53);
pub const NET_GET_IF: u32 = _IOWR!(b'o', 54, dvb_net_if);

/* binary compatibility cruft: */
#[repr(C)]
pub struct __dvb_net_if_old {
    pub pid: u16,
    pub if_num: u16,
}

pub const __NET_ADD_IF_OLD: u32 = _IOWR!(b'o', 52, __dvb_net_if_old);
pub const __NET_GET_IF_OLD: u32 = _IOWR!(b'o', 54, __dvb_net_if_old);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
