/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/***************************************************************************
 * Linux PPP over L2TP (PPPoL2TP) Socket Implementation (RFC 2661)
 *
 * This file supplies definitions required by the PPP over L2TP driver
 * (l2tp_ppp.c).  All version information wrt this file is located in l2tp_ppp.c
 *
 * License:
 *		This program is free software; you can redistribute it and/or
 *		modify it under the terms of the GNU General Public License
 *		as published by the Free Software Foundation; either version
 *		2 of the License, or (at your option) any later version.
 *
 */

// Dependencies supplied by the corresponding Linux UAPI definitions:
// __kernel_pid_t, __u16, __u32, sockaddr_in, sockaddr_in6, and L2TP_MSG_*.

/* Structure used to connect() the socket to a particular tunnel UDP
 * socket over IPv4.
 */
#[repr(C)]
pub struct pppol2tp_addr {
    pub pid: __kernel_pid_t, /* pid that owns the fd.
                             * 0 => current */
    pub fd: core::ffi::c_int, /* FD of UDP socket to use */

    pub addr: sockaddr_in, /* IP address and port to send to */

    pub s_tunnel: __u16,
    pub s_session: __u16, /* For matching incoming packets */
    pub d_tunnel: __u16,
    pub d_session: __u16, /* For sending outgoing packets */
}

/* Structure used to connect() the socket to a particular tunnel UDP
 * socket over IPv6.
 */
#[repr(C)]
pub struct pppol2tpin6_addr {
    pub pid: __kernel_pid_t, /* pid that owns the fd.
                             * 0 => current */
    pub fd: core::ffi::c_int, /* FD of UDP socket to use */

    pub s_tunnel: __u16,
    pub s_session: __u16, /* For matching incoming packets */
    pub d_tunnel: __u16,
    pub d_session: __u16, /* For sending outgoing packets */

    pub addr: sockaddr_in6, /* IP address and port to send to */
}

/* The L2TPv3 protocol changes tunnel and session ids from 16 to 32
 * bits. So we need a different sockaddr structure.
 */
#[repr(C)]
pub struct pppol2tpv3_addr {
    pub pid: __kernel_pid_t, /* pid that owns the fd.
                             * 0 => current */
    pub fd: core::ffi::c_int, /* FD of UDP or IP socket to use */

    pub addr: sockaddr_in, /* IP address and port to send to */

    pub s_tunnel: __u32,
    pub s_session: __u32, /* For matching incoming packets */
    pub d_tunnel: __u32,
    pub d_session: __u32, /* For sending outgoing packets */
}

#[repr(C)]
pub struct pppol2tpv3in6_addr {
    pub pid: __kernel_pid_t, /* pid that owns the fd.
                             * 0 => current */
    pub fd: core::ffi::c_int, /* FD of UDP or IP socket to use */

    pub s_tunnel: __u32,
    pub s_session: __u32, /* For matching incoming packets */
    pub d_tunnel: __u32,
    pub d_session: __u32, /* For sending outgoing packets */

    pub addr: sockaddr_in6, /* IP address and port to send to */
}

/* Socket options:
 * DEBUG	- bitmask of debug message categories (not used)
 * SENDSEQ	- 0 => don't send packets with sequence numbers
 *		  1 => send packets with sequence numbers
 * RECVSEQ	- 0 => receive packet sequence numbers are optional
 *		  1 => drop receive packets without sequence numbers
 * LNSMODE	- 0 => act as LAC.
 *		  1 => act as LNS.
 * REORDERTO	- reorder timeout (in millisecs). If 0, don't try to reorder.
 */
pub const PPPOL2TP_SO_DEBUG: i32 = 1;
pub const PPPOL2TP_SO_RECVSEQ: i32 = 2;
pub const PPPOL2TP_SO_SENDSEQ: i32 = 3;
pub const PPPOL2TP_SO_LNSMODE: i32 = 4;
pub const PPPOL2TP_SO_REORDERTO: i32 = 5;

/* Debug message categories for the DEBUG socket option (deprecated) */
pub const PPPOL2TP_MSG_DEBUG: i32 = L2TP_MSG_DEBUG;
pub const PPPOL2TP_MSG_CONTROL: i32 = L2TP_MSG_CONTROL;
pub const PPPOL2TP_MSG_SEQ: i32 = L2TP_MSG_SEQ;
pub const PPPOL2TP_MSG_DATA: i32 = L2TP_MSG_DATA;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
