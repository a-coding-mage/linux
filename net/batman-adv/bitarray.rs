// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Simon Wunderlich, Marek Lindner
 */

// Dependencies supplied by the surrounding translation unit:
// bitarray.h, main.h, linux/bitmap.h, and log.h.

extern "C" {
    fn bitmap_shift_left(
        dst: *mut ::std::os::raw::c_ulong,
        src: *const ::std::os::raw::c_ulong,
        shift: i32,
        nbits: usize,
    );
    fn bitmap_zero(dst: *mut ::std::os::raw::c_ulong, nbits: usize);
    fn batadv_set_bit(seq_bits: *mut ::std::os::raw::c_ulong, bit: i32);
    fn batadv_dbg(
        dbg_level: i32,
        bat_priv: *mut batadv_priv,
        fmt: *const ::std::os::raw::c_char,
        ...,
    );
}

#[repr(C)]
pub struct batadv_priv {
    _private: [u8; 0],
}

/**
 * batadv_bitmap_shift_left() - shift the sequence number bitmap left
 * @seq_bits: the sequence number bitmap to shift
 * @n: number of positions to shift left
 *
 * Shift @seq_bits by @n positions. No-op if @n is not within the bounds of
 * the bitmap.
 */
unsafe fn batadv_bitmap_shift_left(seq_bits: *mut ::std::os::raw::c_ulong, n: i32) {
    if n <= 0 || n >= BATADV_TQ_LOCAL_WINDOW_SIZE {
        return;
    }

    unsafe {
        bitmap_shift_left(
            seq_bits,
            seq_bits as *const ::std::os::raw::c_ulong,
            n,
            BATADV_TQ_LOCAL_WINDOW_SIZE as usize,
        );
    }
}

/**
 * batadv_bit_get_packet() - receive and process one packet within the sequence
 *  number window
 * @priv: the bat priv with all the mesh interface information
 * @seq_bits: pointer to the sequence number bitmap of received packets
 * @seq_num_diff: difference between the current/received sequence number and
 *  the last sequence number
 * @set_mark: whether this packet should be marked in seq_bits
 *
 * Return: true if the window was moved (either new or very old),
 *  false if the window was not moved/shifted.
 */
pub unsafe fn batadv_bit_get_packet(
    priv_: *mut ::std::ffi::c_void,
    seq_bits: *mut ::std::os::raw::c_ulong,
    seq_num_diff: i32,
    set_mark: i32,
) -> bool {
    let bat_priv = priv_ as *mut batadv_priv;

    /* sequence number is slightly older. We already got a sequence number
     * higher than this one, so we just mark it.
     */
    if seq_num_diff <= 0 && seq_num_diff > -BATADV_TQ_LOCAL_WINDOW_SIZE {
        if set_mark != 0 {
            unsafe { batadv_set_bit(seq_bits, -seq_num_diff) };
        }
        return false;
    }

    /* sequence number is slightly newer, so we shift the window and
     * set the mark if required
     */
    if seq_num_diff > 0 && seq_num_diff < BATADV_TQ_LOCAL_WINDOW_SIZE {
        unsafe { batadv_bitmap_shift_left(seq_bits, seq_num_diff) };

        if set_mark != 0 {
            unsafe { batadv_set_bit(seq_bits, 0) };
        }
        return true;
    }

    /* sequence number is much newer, probably missed a lot of packets */
    if seq_num_diff >= BATADV_TQ_LOCAL_WINDOW_SIZE
        && seq_num_diff < BATADV_EXPECTED_SEQNO_RANGE
    {
        unsafe {
            batadv_dbg(
                BATADV_DBG_BATMAN,
                bat_priv,
                b"We missed a lot of packets (%i) !\n\0".as_ptr() as *const _,
                seq_num_diff - 1,
            );
            bitmap_zero(seq_bits, BATADV_TQ_LOCAL_WINDOW_SIZE as usize);
        }
        if set_mark != 0 {
            unsafe { batadv_set_bit(seq_bits, 0) };
        }
        return true;
    }

    /* received a much older packet. The other host either restarted
     * or the old packet got delayed somewhere in the network. The
     * packet should be dropped without calling this function if the
     * seqno window is protected.
     *
     * seq_num_diff <= -BATADV_TQ_LOCAL_WINDOW_SIZE
     * or
     * seq_num_diff >= BATADV_EXPECTED_SEQNO_RANGE
     */
    unsafe {
        batadv_dbg(
            BATADV_DBG_BATMAN,
            bat_priv,
            b"Other host probably restarted!\n\0".as_ptr() as *const _,
        );
        bitmap_zero(seq_bits, BATADV_TQ_LOCAL_WINDOW_SIZE as usize);
    }
    if set_mark != 0 {
        unsafe { batadv_set_bit(seq_bits, 0) };
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
