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

/* Depends on C header <linux/seg6.h>. */

pub const SEG6_LOCAL_UNSPEC: u32 = 0;
pub const SEG6_LOCAL_ACTION: u32 = 1;
pub const SEG6_LOCAL_SRH: u32 = 2;
pub const SEG6_LOCAL_TABLE: u32 = 3;
pub const SEG6_LOCAL_NH4: u32 = 4;
pub const SEG6_LOCAL_NH6: u32 = 5;
pub const SEG6_LOCAL_IIF: u32 = 6;
pub const SEG6_LOCAL_OIF: u32 = 7;
pub const SEG6_LOCAL_BPF: u32 = 8;
pub const __SEG6_LOCAL_MAX: u32 = 9;
pub const SEG6_LOCAL_MAX: u32 = __SEG6_LOCAL_MAX - 1;

pub const SEG6_LOCAL_ACTION_UNSPEC: u32 = 0;
/* node segment */
pub const SEG6_LOCAL_ACTION_END: u32 = 1;
/* adjacency segment (IPv6 cross-connect) */
pub const SEG6_LOCAL_ACTION_END_X: u32 = 2;
/* lookup of next seg NH in table */
pub const SEG6_LOCAL_ACTION_END_T: u32 = 3;
/* decap and L2 cross-connect */
pub const SEG6_LOCAL_ACTION_END_DX2: u32 = 4;
/* decap and IPv6 cross-connect */
pub const SEG6_LOCAL_ACTION_END_DX6: u32 = 5;
/* decap and IPv4 cross-connect */
pub const SEG6_LOCAL_ACTION_END_DX4: u32 = 6;
/* decap and lookup of DA in v6 table */
pub const SEG6_LOCAL_ACTION_END_DT6: u32 = 7;
/* decap and lookup of DA in v4 table */
pub const SEG6_LOCAL_ACTION_END_DT4: u32 = 8;
/* binding segment with insertion */
pub const SEG6_LOCAL_ACTION_END_B6: u32 = 9;
/* binding segment with encapsulation */
pub const SEG6_LOCAL_ACTION_END_B6_ENCAP: u32 = 10;
/* binding segment with MPLS encap */
pub const SEG6_LOCAL_ACTION_END_BM: u32 = 11;
/* lookup last seg in table */
pub const SEG6_LOCAL_ACTION_END_S: u32 = 12;
/* forward to SR-unaware VNF with static proxy */
pub const SEG6_LOCAL_ACTION_END_AS: u32 = 13;
/* forward to SR-unaware VNF with masquerading */
pub const SEG6_LOCAL_ACTION_END_AM: u32 = 14;
/* custom BPF action */
pub const SEG6_LOCAL_ACTION_END_BPF: u32 = 15;
pub const __SEG6_LOCAL_ACTION_MAX: u32 = 16;

pub const SEG6_LOCAL_ACTION_MAX: u32 = __SEG6_LOCAL_ACTION_MAX - 1;

pub const SEG6_LOCAL_BPF_PROG_UNSPEC: u32 = 0;
pub const SEG6_LOCAL_BPF_PROG: u32 = 1;
pub const SEG6_LOCAL_BPF_PROG_NAME: u32 = 2;
pub const __SEG6_LOCAL_BPF_PROG_MAX: u32 = 3;

pub const SEG6_LOCAL_BPF_PROG_MAX: u32 = __SEG6_LOCAL_BPF_PROG_MAX - 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
