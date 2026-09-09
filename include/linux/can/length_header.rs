/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2020 Oliver Hartkopp <socketcan@hartkopp.net>
 * Copyright (C) 2020 Marc Kleine-Budde <kernel@pengutronix.de>
 * Copyright (C) 2020, 2023 Vincent Mailhol <mailhol.vincent@wanadoo.fr>
 */

// Dependencies supplied by linux/bits.h, linux/can.h,
// linux/can/netlink.h, and linux/math.h are referenced below.

pub const CAN_FRAME_HEADER_SFF_BITS: usize = 19;
pub const CAN_FRAME_HEADER_EFF_BITS: usize = 39;
pub const CANFD_FRAME_HEADER_SFF_BITS: usize = 22;
pub const CANFD_FRAME_HEADER_EFF_BITS: usize = 41;
pub const CAN_FRAME_CRC_FIELD_BITS: usize = 16;
pub const CANFD_FRAME_CRC17_FIELD_BITS: usize = 28;
pub const CANFD_FRAME_CRC21_FIELD_BITS: usize = 33;
pub const CAN_FRAME_FOOTER_BITS: usize = 9;
pub const CAN_INTERMISSION_BITS: usize = 3;

/// Calculate the maximum length with bitstuffing.
#[inline]
pub const fn can_bitstuffing_len(destuffed_len: usize) -> usize {
    destuffed_len + (destuffed_len - 1) / 4
}

#[inline]
pub const fn __can_bitstuffing_len(bitstuffing: bool, destuffed_len: usize) -> usize {
    if bitstuffing { can_bitstuffing_len(destuffed_len) } else { destuffed_len }
}

#[inline]
pub const fn __can_cc_frame_bits(
    is_eff: bool,
    bitstuffing: bool,
    intermission: bool,
    data_len: usize,
) -> usize {
    __can_bitstuffing_len(
        bitstuffing,
        (if is_eff { CAN_FRAME_HEADER_EFF_BITS } else { CAN_FRAME_HEADER_SFF_BITS })
            + data_len * BITS_PER_BYTE
            + CAN_FRAME_CRC_FIELD_BITS,
    ) + CAN_FRAME_FOOTER_BITS
        + if intermission { CAN_INTERMISSION_BITS } else { 0 }
}

#[inline]
pub const fn __can_fd_frame_bits(
    is_eff: bool,
    bitstuffing: bool,
    intermission: bool,
    data_len: usize,
) -> usize {
    __can_bitstuffing_len(
        bitstuffing,
        (if is_eff { CANFD_FRAME_HEADER_EFF_BITS } else { CANFD_FRAME_HEADER_SFF_BITS })
            + data_len * BITS_PER_BYTE,
    ) + if data_len <= 16 {
        CANFD_FRAME_CRC17_FIELD_BITS
    } else {
        CANFD_FRAME_CRC21_FIELD_BITS
    } + CAN_FRAME_FOOTER_BITS
        + if intermission { CAN_INTERMISSION_BITS } else { 0 }
}

#[inline]
pub const fn can_frame_bits(
    is_fd: bool,
    is_eff: bool,
    bitstuffing: bool,
    intermission: bool,
    data_len: usize,
) -> usize {
    if is_fd {
        __can_fd_frame_bits(is_eff, bitstuffing, intermission, data_len)
    } else {
        __can_cc_frame_bits(is_eff, bitstuffing, intermission, data_len)
    }
}

#[inline]
pub const fn can_frame_bytes(
    is_fd: bool,
    is_eff: bool,
    bitstuffing: bool,
    data_len: usize,
) -> usize {
    (can_frame_bits(is_fd, is_eff, bitstuffing, true, data_len) + BITS_PER_BYTE - 1)
        / BITS_PER_BYTE
}

pub const CAN_FRAME_LEN_MAX: usize = can_frame_bytes(false, true, false, CAN_MAX_DLEN);
pub const CANFD_FRAME_LEN_MAX: usize = can_frame_bytes(true, true, false, CANFD_MAX_DLEN);

#[inline]
pub const fn can_cc_dlc2len(dlc: u8) -> u8 {
    if dlc < CAN_MAX_DLEN as u8 { dlc } else { CAN_MAX_DLEN as u8 }
}

#[inline]
pub unsafe fn can_get_cc_dlc(cf: *const struct_can_frame, ctrlmode: u32) -> u8 {
    if (ctrlmode & CAN_CTRLMODE_CC_LEN8_DLC) != 0
        && (*cf).len == CAN_MAX_DLEN as u8
        && (*cf).len8_dlc > CAN_MAX_DLEN as u8
        && (*cf).len8_dlc <= CAN_MAX_RAW_DLC as u8
    {
        return (*cf).len8_dlc;
    }
    (*cf).len
}

#[inline]
pub unsafe fn can_frame_set_cc_len(cf: *mut struct_can_frame, dlc: u8, ctrlmode: u32) {
    if (ctrlmode & CAN_CTRLMODE_CC_LEN8_DLC) != 0 && dlc > CAN_MAX_DLEN as u8 {
        (*cf).len8_dlc = dlc;
    }
    (*cf).len = can_cc_dlc2len(dlc);
}

extern "C" {
    pub fn can_fd_dlc2len(dlc: u8) -> u8;
    pub fn can_fd_len2dlc(len: u8) -> u8;
    pub fn can_skb_get_frame_len(skb: *const struct_sk_buff) -> u32;
}

#[inline]
pub unsafe fn canfd_sanitize_len(len: u8) -> u8 {
    can_fd_dlc2len(can_fd_len2dlc(len))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
