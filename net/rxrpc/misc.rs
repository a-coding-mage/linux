// SPDX-License-Identifier: GPL-2.0-or-later
/* Miscellaneous bits
 *
 * Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel and RXRPC declarations are supplied by the surrounding repository.

/*
 * The maximum listening backlog queue size that may be set on a socket by
 * listen().
 */
pub static mut rxrpc_max_backlog: u32 = 10;

/*
 * How long to wait before scheduling an ACK with subtype DELAY (in ms).
 *
 * We use this when we've received new data packets.  If those packets aren't
 * all consumed within this time we will send a DELAY ACK if an ACK was not
 * requested to let the sender know it doesn't need to resend.
 */
pub static mut rxrpc_soft_ack_delay: usize = 1000;

/*
 * How long to wait before scheduling an ACK with subtype IDLE (in ms).
 *
 * We use this when we've consumed some previously soft-ACK'd packets when
 * further packets aren't immediately received to decide when to send an IDLE
 * ACK let the other end know that it can free up its Tx buffer space.
 */
pub static mut rxrpc_idle_ack_delay: usize = 500;

/*
 * Receive window size in packets.  This indicates the maximum number of
 * unconsumed received packets we're willing to retain in memory.  Once this
 * limit is hit, we should generate an EXCEEDS_WINDOW ACK and discard further
 * packets.
 */
pub static mut rxrpc_rx_window_size: u32 = 255;

/*
 * Maximum Rx MTU size.  This indicates to the sender the size of jumbo packet
 * made by gluing normal packets together that we're willing to handle.
 */
pub static mut rxrpc_rx_mtu: u32 = rxrpc_jumbo(46);

/*
 * The maximum number of fragments in a received jumbo packet that we tell the
 * sender that we're willing to handle.
 */
pub static mut rxrpc_rx_jumbo_max: u32 = 46;

#[cfg(CONFIG_AF_RXRPC_INJECT_RX_DELAY)]
/*
 * The delay to inject into packet reception.
 */
pub static mut rxrpc_inject_rx_delay: usize = 0;

// External equivalent of the RXRPC_JUMBO macro used by the source headers.
extern "C" {
    fn rxrpc_jumbo(n: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
