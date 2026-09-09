/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Simon Wunderlich, Marek Lindner
 */

// Dependency intent from main.h and the Linux bit operations/types headers is
// preserved here; the supplied names are expected from the surrounding build.

/**
 * batadv_test_bit() - check if bit is set in the current window
 *
 * @seq_bits: pointer to the sequence number bitmap of received packets
 * @last_seqno: latest sequence number in seq_bits
 * @curr_seqno: sequence number to test for
 *
 * Return: true if the corresponding bit in the given seq_bits indicates true
 * and curr_seqno is within range of last_seqno. Otherwise returns false.
 */
pub unsafe fn batadv_test_bit(
    seq_bits: *const ::std::ffi::c_ulong,
    last_seqno: u32,
    curr_seqno: u32,
) -> bool {
    let diff: i32;

    diff = last_seqno.wrapping_sub(curr_seqno) as i32;
    if diff < 0 || diff >= BATADV_TQ_LOCAL_WINDOW_SIZE {
        return false;
    }
    test_bit(diff as usize, seq_bits) != 0
}

/**
 * batadv_set_bit() - Turn corresponding bit on, so we can remember that we got
 *  the packet
 * @seq_bits: bitmap of the packet receive window
 * @n: relative sequence number of newly received packet
 */
pub unsafe fn batadv_set_bit(seq_bits: *mut ::std::ffi::c_ulong, n: i32) {
    /* if too old, just drop it */
    if n < 0 || n >= BATADV_TQ_LOCAL_WINDOW_SIZE {
        return;
    }

    set_bit(n as usize, seq_bits); /* turn the position on */
}

pub unsafe extern "C" fn batadv_bit_get_packet(
    priv_: *mut ::std::ffi::c_void,
    seq_bits: *mut ::std::ffi::c_ulong,
    seq_num_diff: i32,
    set_mark: ::std::ffi::c_int,
) -> bool;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
