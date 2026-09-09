/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C)2003-2006 Helsinki University of Technology
 * Copyright (C)2003-2006 USAGI/WIDE Project
 */
/*
 * Authors:
 *	Noriaki TAKAMIYA @USAGI
 *	Masahide NAKAMURA @USAGI
 *	YOSHIFUJI Hideaki @USAGI
 */

// C dependencies:
// #include <linux/skbuff.h>
// #include <net/sock.h>

/*
 * Mobility Header
 */
#[repr(C, packed)]
pub struct ip6_mh {
    pub ip6mh_proto: u8,
    pub ip6mh_hdrlen: u8,
    pub ip6mh_type: u8,
    pub ip6mh_reserved: u8,
    pub ip6mh_cksum: u16,
    /* Followed by type specific messages */
    pub data: [u8; 0],
}

pub const IP6_MH_TYPE_BRR: u8 = 0; /* Binding Refresh Request */
pub const IP6_MH_TYPE_HOTI: u8 = 1; /* HOTI Message   */
pub const IP6_MH_TYPE_COTI: u8 = 2; /* COTI Message  */
pub const IP6_MH_TYPE_HOT: u8 = 3; /* HOT Message   */
pub const IP6_MH_TYPE_COT: u8 = 4; /* COT Message  */
pub const IP6_MH_TYPE_BU: u8 = 5; /* Binding Update */
pub const IP6_MH_TYPE_BACK: u8 = 6; /* Binding ACK */
pub const IP6_MH_TYPE_BERROR: u8 = 7; /* Binding Error */
pub const IP6_MH_TYPE_MAX: u8 = IP6_MH_TYPE_BERROR;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
