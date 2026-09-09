// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * 6LoWPAN IPv6 Hop-by-Hop Options Header compression according to RFC6282
 */

// The C implementation includes "nhc.h".  Its declarations and the
// LOWPAN_NHC/module registration machinery are supplied by the surrounding
// translation unit.

pub const LOWPAN_NHC_HOP_ID_0: u8 = 0xe0;
pub const LOWPAN_NHC_HOP_MASK_0: u8 = 0xfe;

// LOWPAN_NHC(nhc_hop, "RFC6282 Hop-by-Hop Options", NEXTHDR_HOP, 0,
//            LOWPAN_NHC_HOP_ID_0, LOWPAN_NHC_HOP_MASK_0, NULL, NULL);
// module_lowpan_nhc(nhc_hop);

// MODULE_DESCRIPTION("6LoWPAN next header RFC6282 Hop-by-Hop Options compression");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
