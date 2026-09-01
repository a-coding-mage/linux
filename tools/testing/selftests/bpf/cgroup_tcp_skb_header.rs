/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/* Define states of a socket to tracking messages sending to and from the
 * socket.
 *
 * These states are based on rfc9293 with some modifications to support
 * tracking of messages sent out from a socket. For example, when a SYN is
 * received, a new socket is transiting to the SYN_RECV state defined in
 * rfc9293. But, we put it in SYN_RECV_SENDING_SYN_ACK state and when
 * SYN-ACK is sent out, it moves to SYN_RECV state. With this modification,
 * we can track the message sent out from a socket.
 */

pub const INIT: u32 = 0;
pub const CLOSED: u32 = 1;
pub const SYN_SENT: u32 = 2;
pub const SYN_RECV_SENDING_SYN_ACK: u32 = 3;
pub const SYN_RECV: u32 = 4;
pub const ESTABLISHED: u32 = 5;
pub const FIN_WAIT1: u32 = 6;
pub const FIN_WAIT2: u32 = 7;
pub const CLOSE_WAIT_SENDING_ACK: u32 = 8;
pub const CLOSE_WAIT: u32 = 9;
pub const CLOSING: u32 = 10;
pub const LAST_ACK: u32 = 11;
pub const TIME_WAIT_SENDING_ACK: u32 = 12;
pub const TIME_WAIT: u32 = 13;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
