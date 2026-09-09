/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * inet_dscp.h: helpers for handling differentiated services codepoints (DSCP)
 *
 * DSCP is defined in RFC 2474:
 *
 *        0   1   2   3   4   5   6   7
 *      +---+---+---+---+---+---+---+---+
 *      |         DSCP          |  CU   |
 *      +---+---+---+---+---+---+---+---+
 *
 *        DSCP: differentiated services codepoint
 *        CU:   currently unused
 *
 * The whole DSCP + CU bits form the DS field.
 * The DS field is also commonly called TOS or Traffic Class (for IPv6).
 *
 * Note: the CU bits are now used for Explicit Congestion Notification
 *       (RFC 3168).
 */

/* Special type for storing DSCP values.
 *
 * A dscp_t variable stores a DS field with the CU (ECN) bits cleared.
 * Using dscp_t allows to strictly separate DSCP and ECN bits, thus avoiding
 * bugs where ECN bits are erroneously taken into account during FIB lookups
 * or policy routing.
 *
 * Note: to get the real DSCP value contained in a dscp_t variable one would
 * have to do a bit shift after calling inet_dscp_to_dsfield(). We could have
 * a helper for that, but there's currently no users.
 */
pub type dscp_t = u8;

pub const INET_DSCP_MASK: u8 = 0xfc;

/* A few places in the IPv4 code need to ignore the three high order bits of
 * DSCP because of backward compatibility (as these bits used to represent the
 * IPv4 Precedence in RFC 791's TOS field and were ignored).
 */
pub const INET_DSCP_LEGACY_TOS_MASK: dscp_t = 0x1c;

#[inline]
pub fn inet_dsfield_to_dscp(dsfield: u8) -> dscp_t {
	(dsfield & INET_DSCP_MASK) as dscp_t
}

#[inline]
pub fn inet_dscp_to_dsfield(dscp: dscp_t) -> u8 {
	dscp as u8
}

#[inline]
pub fn inet_validate_dscp(val: u8) -> bool {
	(val & !INET_DSCP_MASK) == 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
