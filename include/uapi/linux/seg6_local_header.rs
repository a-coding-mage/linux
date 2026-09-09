/*
 *  SR-IPv6 implementation
 *
 *  Author:
 *  David Lebrun <david.lebrun@uclouvain.be>
 *
 *
 *  This program is free software; you can redistribute it and/or
 *      modify it under the terms of the GNU General Public License
 *      as published by the Free Software Foundation; either version
 *      2 of the License, or (at your option) any later version.
 */

// Dependency supplied by linux/seg6.h in the original header.

pub const SEG6_LOCAL_UNSPEC: i32 = 0;
pub const SEG6_LOCAL_ACTION: i32 = 1;
pub const SEG6_LOCAL_SRH: i32 = 2;
pub const SEG6_LOCAL_TABLE: i32 = 3;
pub const SEG6_LOCAL_NH4: i32 = 4;
pub const SEG6_LOCAL_NH6: i32 = 5;
pub const SEG6_LOCAL_IIF: i32 = 6;
pub const SEG6_LOCAL_OIF: i32 = 7;
pub const SEG6_LOCAL_BPF: i32 = 8;
pub const SEG6_LOCAL_VRFTABLE: i32 = 9;
pub const SEG6_LOCAL_COUNTERS: i32 = 10;
pub const SEG6_LOCAL_FLAVORS: i32 = 11;
pub const __SEG6_LOCAL_MAX: i32 = 12;
pub const SEG6_LOCAL_MAX: i32 = __SEG6_LOCAL_MAX - 1;

pub const SEG6_LOCAL_ACTION_UNSPEC: i32 = 0;
/* node segment */
pub const SEG6_LOCAL_ACTION_END: i32 = 1;
/* adjacency segment (IPv6 cross-connect) */
pub const SEG6_LOCAL_ACTION_END_X: i32 = 2;
/* lookup of next seg NH in table */
pub const SEG6_LOCAL_ACTION_END_T: i32 = 3;
/* decap and L2 cross-connect */
pub const SEG6_LOCAL_ACTION_END_DX2: i32 = 4;
/* decap and IPv6 cross-connect */
pub const SEG6_LOCAL_ACTION_END_DX6: i32 = 5;
/* decap and IPv4 cross-connect */
pub const SEG6_LOCAL_ACTION_END_DX4: i32 = 6;
/* decap and lookup of DA in v6 table */
pub const SEG6_LOCAL_ACTION_END_DT6: i32 = 7;
/* decap and lookup of DA in v4 table */
pub const SEG6_LOCAL_ACTION_END_DT4: i32 = 8;
/* binding segment with insertion */
pub const SEG6_LOCAL_ACTION_END_B6: i32 = 9;
/* binding segment with encapsulation */
pub const SEG6_LOCAL_ACTION_END_B6_ENCAP: i32 = 10;
/* binding segment with MPLS encap */
pub const SEG6_LOCAL_ACTION_END_BM: i32 = 11;
/* lookup last seg in table */
pub const SEG6_LOCAL_ACTION_END_S: i32 = 12;
/* forward to SR-unaware VNF with static proxy */
pub const SEG6_LOCAL_ACTION_END_AS: i32 = 13;
/* forward to SR-unaware VNF with masquerading */
pub const SEG6_LOCAL_ACTION_END_AM: i32 = 14;
/* custom BPF action */
pub const SEG6_LOCAL_ACTION_END_BPF: i32 = 15;
/* decap and lookup of DA in v4 or v6 table */
pub const SEG6_LOCAL_ACTION_END_DT46: i32 = 16;
pub const __SEG6_LOCAL_ACTION_MAX: i32 = 17;
pub const SEG6_LOCAL_ACTION_MAX: i32 = __SEG6_LOCAL_ACTION_MAX - 1;

pub const SEG6_LOCAL_BPF_PROG_UNSPEC: i32 = 0;
pub const SEG6_LOCAL_BPF_PROG: i32 = 1;
pub const SEG6_LOCAL_BPF_PROG_NAME: i32 = 2;
pub const __SEG6_LOCAL_BPF_PROG_MAX: i32 = 3;
pub const SEG6_LOCAL_BPF_PROG_MAX: i32 = __SEG6_LOCAL_BPF_PROG_MAX - 1;

/* SRv6 Behavior counters are encoded as netlink attributes guaranteeing the
 * correct alignment.
 * Each counter is identified by a different attribute type (i.e.
 * SEG6_LOCAL_CNT_PACKETS).
 *
 * - SEG6_LOCAL_CNT_PACKETS: identifies a counter that counts the number of
 *   packets that have been CORRECTLY processed by an SRv6 Behavior instance
 *   (i.e., packets that generate errors or are dropped are NOT counted).
 *
 * - SEG6_LOCAL_CNT_BYTES: identifies a counter that counts the total amount
 *   of traffic in bytes of all packets that have been CORRECTLY processed by
 *   an SRv6 Behavior instance (i.e., packets that generate errors or are
 *   dropped are NOT counted).
 *
 * - SEG6_LOCAL_CNT_ERRORS: identifies a counter that counts the number of
 *   packets that have NOT been properly processed by an SRv6 Behavior instance
 *   (i.e., packets that generate errors or are dropped).
 */
pub const SEG6_LOCAL_CNT_UNSPEC: i32 = 0;
pub const SEG6_LOCAL_CNT_PAD: i32 = 1;
pub const SEG6_LOCAL_CNT_PACKETS: i32 = 2;
pub const SEG6_LOCAL_CNT_BYTES: i32 = 3;
pub const SEG6_LOCAL_CNT_ERRORS: i32 = 4;
pub const __SEG6_LOCAL_CNT_MAX: i32 = 5;
pub const SEG6_LOCAL_CNT_MAX: i32 = __SEG6_LOCAL_CNT_MAX - 1;

/* SRv6 End* Flavor attributes */
pub const SEG6_LOCAL_FLV_UNSPEC: i32 = 0;
pub const SEG6_LOCAL_FLV_OPERATION: i32 = 1;
pub const SEG6_LOCAL_FLV_LCBLOCK_BITS: i32 = 2;
pub const SEG6_LOCAL_FLV_LCNODE_FN_BITS: i32 = 3;
pub const __SEG6_LOCAL_FLV_MAX: i32 = 4;
pub const SEG6_LOCAL_FLV_MAX: i32 = __SEG6_LOCAL_FLV_MAX - 1;

/* Designed flavor operations for SRv6 End* Behavior */
pub const SEG6_LOCAL_FLV_OP_UNSPEC: i32 = 0;
pub const SEG6_LOCAL_FLV_OP_PSP: i32 = 1;
pub const SEG6_LOCAL_FLV_OP_USP: i32 = 2;
pub const SEG6_LOCAL_FLV_OP_USD: i32 = 3;
pub const SEG6_LOCAL_FLV_OP_NEXT_CSID: i32 = 4;
pub const __SEG6_LOCAL_FLV_OP_MAX: i32 = 5;
pub const SEG6_LOCAL_FLV_OP_MAX: i32 = __SEG6_LOCAL_FLV_OP_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
