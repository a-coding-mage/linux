/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies: linux/types.h and asm/byteorder.h. */

/* Reference: RFC 5462, RFC 3032
 *
 *  0                   1                   2                   3
 *  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
 * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 * |                Label                  | TC  |S|       TTL     |
 * +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 *
 *	Label:  Label Value, 20 bits
 *	TC:     Traffic Class field, 3 bits
 *	S:      Bottom of Stack, 1 bit
 *	TTL:    Time to Live, 8 bits
 */

#[repr(C)]
pub struct mpls_label {
    pub entry: u32,
}

pub const MPLS_LS_LABEL_MASK: u32 = 0xFFFFF000;
pub const MPLS_LS_LABEL_SHIFT: u32 = 12;
pub const MPLS_LS_TC_MASK: u32 = 0x00000E00;
pub const MPLS_LS_TC_SHIFT: u32 = 9;
pub const MPLS_LS_S_MASK: u32 = 0x00000100;
pub const MPLS_LS_S_SHIFT: u32 = 8;
pub const MPLS_LS_TTL_MASK: u32 = 0x000000FF;
pub const MPLS_LS_TTL_SHIFT: u32 = 0;

/* Reserved labels */
pub const MPLS_LABEL_IPV4NULL: u32 = 0; /* RFC3032 */
pub const MPLS_LABEL_RTALERT: u32 = 1; /* RFC3032 */
pub const MPLS_LABEL_IPV6NULL: u32 = 2; /* RFC3032 */
pub const MPLS_LABEL_IMPLNULL: u32 = 3; /* RFC3032 */
pub const MPLS_LABEL_ENTROPY: u32 = 7; /* RFC6790 */
pub const MPLS_LABEL_GAL: u32 = 13; /* RFC5586 */
pub const MPLS_LABEL_OAMALERT: u32 = 14; /* RFC3429 */
pub const MPLS_LABEL_EXTENSION: u32 = 15; /* RFC7274 */

pub const MPLS_LABEL_FIRST_UNRESERVED: u32 = 16; /* RFC3032 */

/* These are embedded into IFLA_STATS_AF_SPEC:
 * [IFLA_STATS_AF_SPEC]
 * -> [AF_MPLS]
 *    -> [MPLS_STATS_xxx]
 *
 * Attributes:
 * [MPLS_STATS_LINK] = {
 *     struct mpls_link_stats
 * }
 */
pub const MPLS_STATS_UNSPEC: i32 = 0; /* also used as 64bit pad attribute */
pub const MPLS_STATS_LINK: i32 = 1;
pub const __MPLS_STATS_MAX: i32 = 2;

pub const MPLS_STATS_MAX: i32 = __MPLS_STATS_MAX - 1;

#[repr(C)]
pub struct mpls_link_stats {
    pub rx_packets: u64, /* total packets received */
    pub tx_packets: u64, /* total packets transmitted */
    pub rx_bytes: u64, /* total bytes received */
    pub tx_bytes: u64, /* total bytes transmitted */
    pub rx_errors: u64, /* bad packets received */
    pub tx_errors: u64, /* packet transmit problems */
    pub rx_dropped: u64, /* packet dropped on receive */
    pub tx_dropped: u64, /* packet dropped on transmit */
    pub rx_noroute: u64, /* no route for packet dest */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
