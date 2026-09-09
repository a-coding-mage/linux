// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	6LoWPAN IPv6 Routing Header compression according to RFC6282
 */

// Dependency supplied by nhc.h in the original source.

pub const LOWPAN_NHC_ROUTING_ID_0: u8 = 0xe2;
pub const LOWPAN_NHC_ROUTING_MASK_0: u8 = 0xfe;

// Original registration:
// LOWPAN_NHC(nhc_routing, "RFC6282 Routing", NEXTHDR_ROUTING, 0,
//            LOWPAN_NHC_ROUTING_ID_0, LOWPAN_NHC_ROUTING_MASK_0, NULL, NULL);
// The LOWPAN_NHC macro and NEXTHDR_ROUTING are supplied by the surrounding
// lowpan NHC implementation.

// Original module registration:
// module_lowpan_nhc(nhc_routing);
// MODULE_DESCRIPTION("6LoWPAN next header RFC6282 Routing compression");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
