/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * NET		An implementation of the SOCKET network access protocol.
 *		This is the master header file for the Linux NET layer,
 *		or, in plain English: the networking handling part of the
 *		kernel.
 *
 * Version:	@(#)net.h	1.0.3	05/25/93
 *
 * Authors:	Orest Zborowski, <obz@Kodak.COM>
 *		Ross Biro
 *		Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *
 *		This program is free software; you can redistribute it and/or
 *		modify it under the terms of the GNU General Public License
 *		as published by the Free Software Foundation; either version
 *		2 of the License, or (at your option) any later version.
 */

// Dependencies supplied by the Linux socket and architecture-specific socket headers.

pub const NPROTO: i32 = AF_MAX;

pub const SYS_SOCKET: i32 = 1; // sys_socket(2)
pub const SYS_BIND: i32 = 2; // sys_bind(2)
pub const SYS_CONNECT: i32 = 3; // sys_connect(2)
pub const SYS_LISTEN: i32 = 4; // sys_listen(2)
pub const SYS_ACCEPT: i32 = 5; // sys_accept(2)
pub const SYS_GETSOCKNAME: i32 = 6; // sys_getsockname(2)
pub const SYS_GETPEERNAME: i32 = 7; // sys_getpeername(2)
pub const SYS_SOCKETPAIR: i32 = 8; // sys_socketpair(2)
pub const SYS_SEND: i32 = 9; // sys_send(2)
pub const SYS_RECV: i32 = 10; // sys_recv(2)
pub const SYS_SENDTO: i32 = 11; // sys_sendto(2)
pub const SYS_RECVFROM: i32 = 12; // sys_recvfrom(2)
pub const SYS_SHUTDOWN: i32 = 13; // sys_shutdown(2)
pub const SYS_SETSOCKOPT: i32 = 14; // sys_setsockopt(2)
pub const SYS_GETSOCKOPT: i32 = 15; // sys_getsockopt(2)
pub const SYS_SENDMSG: i32 = 16; // sys_sendmsg(2)
pub const SYS_RECVMSG: i32 = 17; // sys_recvmsg(2)
pub const SYS_ACCEPT4: i32 = 18; // sys_accept4(2)
pub const SYS_RECVMMSG: i32 = 19; // sys_recvmmsg(2)
pub const SYS_SENDMMSG: i32 = 20; // sys_sendmmsg(2)

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum socket_state {
	SS_FREE = 0, // not allocated
	SS_UNCONNECTED, // unconnected to any socket
	SS_CONNECTING, // in process of connecting
	SS_CONNECTED, // connected to socket
	SS_DISCONNECTING, // in process of disconnecting
}

pub const __SO_ACCEPTCON: i32 = 1 << 16; // performed a listen

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
