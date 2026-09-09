/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ppp_defs.h - PPP definitions.
 *
 * Copyright 1994-2000 Paul Mackerras.
 */

// Dependencies supplied by the corresponding Linux headers:
// linux/crc-ccitt.h, linux/skbuff.h, and uapi/linux/ppp_defs.h.

/// Equivalent of `PPP_FCS(fcs, c)`.
#[inline]
pub unsafe fn ppp_fcs(fcs: u16, c: u8) -> u16 {
    crc_ccitt_byte(fcs, c)
}

/**
 * ppp_proto_is_valid - checks if PPP protocol is valid
 * @proto: PPP protocol
 *
 * Assumes proto is not compressed.
 * Protocol is valid if the value is odd and the least significant bit of the
 * most significant octet is 0 (see RFC 1661, section 2).
 */
#[inline]
pub fn ppp_proto_is_valid(proto: u16) -> bool {
    ((proto & 0x0101) == 0x0001)
}

/**
 * ppp_skb_is_compressed_proto - checks if PPP protocol in a skb is compressed
 * @skb: skb to check
 *
 * Check if the PPP protocol field is compressed (the least significant
 * bit of the most significant octet is 1). skb->data must point to the PPP
 * protocol header.
 *
 * Return: Whether the PPP protocol field is compressed.
 */
#[inline]
pub unsafe fn ppp_skb_is_compressed_proto(skb: *const sk_buff) -> bool {
    unlikely((*skb).data[0] & 0x01 != 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
