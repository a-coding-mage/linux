/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Definitions for the TCP protocol sk_state field.
 */

pub const TCP_ESTABLISHED: i32 = 1;
pub const TCP_SYN_SENT: i32 = TCP_ESTABLISHED + 1;
pub const TCP_SYN_RECV: i32 = TCP_SYN_SENT + 1;
pub const TCP_FIN_WAIT1: i32 = TCP_SYN_RECV + 1;
pub const TCP_FIN_WAIT2: i32 = TCP_FIN_WAIT1 + 1;
pub const TCP_TIME_WAIT: i32 = TCP_FIN_WAIT2 + 1;
pub const TCP_CLOSE: i32 = TCP_TIME_WAIT + 1;
pub const TCP_CLOSE_WAIT: i32 = TCP_CLOSE + 1;
pub const TCP_LAST_ACK: i32 = TCP_CLOSE_WAIT + 1;
pub const TCP_LISTEN: i32 = TCP_LAST_ACK + 1;
pub const TCP_CLOSING: i32 = TCP_LISTEN + 1; /* Now a valid state */
pub const TCP_NEW_SYN_RECV: i32 = TCP_CLOSING + 1;
pub const TCP_BOUND_INACTIVE: i32 = TCP_NEW_SYN_RECV + 1; /* Pseudo-state for inet_diag */

pub const TCP_MAX_STATES: i32 = TCP_BOUND_INACTIVE + 1; /* Leave at the end! */

pub const TCP_STATE_MASK: i32 = 0xF;

pub const TCP_ACTION_FIN: i32 = 1 << TCP_CLOSE;

pub const TCPF_ESTABLISHED: i32 = 1 << TCP_ESTABLISHED;
pub const TCPF_SYN_SENT: i32 = 1 << TCP_SYN_SENT;
pub const TCPF_SYN_RECV: i32 = 1 << TCP_SYN_RECV;
pub const TCPF_FIN_WAIT1: i32 = 1 << TCP_FIN_WAIT1;
pub const TCPF_FIN_WAIT2: i32 = 1 << TCP_FIN_WAIT2;
pub const TCPF_TIME_WAIT: i32 = 1 << TCP_TIME_WAIT;
pub const TCPF_CLOSE: i32 = 1 << TCP_CLOSE;
pub const TCPF_CLOSE_WAIT: i32 = 1 << TCP_CLOSE_WAIT;
pub const TCPF_LAST_ACK: i32 = 1 << TCP_LAST_ACK;
pub const TCPF_LISTEN: i32 = 1 << TCP_LISTEN;
pub const TCPF_CLOSING: i32 = 1 << TCP_CLOSING;
pub const TCPF_NEW_SYN_RECV: i32 = 1 << TCP_NEW_SYN_RECV;
pub const TCPF_BOUND_INACTIVE: i32 = 1 << TCP_BOUND_INACTIVE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
