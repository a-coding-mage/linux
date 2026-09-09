/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  IPv6 IOAM Lightweight Tunnel API
 *
 *  Author:
 *  Justin Iurman <justin.iurman@uliege.be>
 */

/* Encap modes:
 *  - inline: direct insertion
 *  - encap: ip6ip6 encapsulation
 *  - auto: inline for local packets, encap for in-transit packets
 */
pub const __IOAM6_IPTUNNEL_MODE_MIN: i32 = 0;

pub const IOAM6_IPTUNNEL_MODE_INLINE: i32 = 1;
pub const IOAM6_IPTUNNEL_MODE_ENCAP: i32 = 2;
pub const IOAM6_IPTUNNEL_MODE_AUTO: i32 = 3;

pub const __IOAM6_IPTUNNEL_MODE_MAX: i32 = 4;

pub const IOAM6_IPTUNNEL_MODE_MIN: i32 = __IOAM6_IPTUNNEL_MODE_MIN + 1;
pub const IOAM6_IPTUNNEL_MODE_MAX: i32 = __IOAM6_IPTUNNEL_MODE_MAX - 1;

pub const IOAM6_IPTUNNEL_UNSPEC: i32 = 0;

/* Encap mode */
pub const IOAM6_IPTUNNEL_MODE: i32 = 1; /* u8 */

/* Tunnel dst address.
 * For encap,auto modes.
 */
pub const IOAM6_IPTUNNEL_DST: i32 = 2; /* struct in6_addr */

/* IOAM Trace Header */
pub const IOAM6_IPTUNNEL_TRACE: i32 = 3; /* struct ioam6_trace_hdr */

/* Insertion frequency:
 * "k over n" packets (0 < k <= n)
 * [0.0001% ... 100%]
 */
pub const IOAM6_IPTUNNEL_FREQ_MIN: u32 = 1;
pub const IOAM6_IPTUNNEL_FREQ_MAX: u32 = 1000000;
pub const IOAM6_IPTUNNEL_FREQ_K: i32 = 4; /* u32 */
pub const IOAM6_IPTUNNEL_FREQ_N: i32 = 5; /* u32 */

/* Tunnel src address.
 * For encap,auto modes.
 * Optional (automatic if not provided).
 */
pub const IOAM6_IPTUNNEL_SRC: i32 = 6; /* struct in6_addr */

pub const __IOAM6_IPTUNNEL_MAX: i32 = 7;
pub const IOAM6_IPTUNNEL_MAX: i32 = __IOAM6_IPTUNNEL_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
