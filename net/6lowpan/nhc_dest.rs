// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	6LoWPAN IPv6 Destination Options Header compression according to
 *	RFC6282
 */

// Dependency supplied by nhc.h in the C source.

pub const LOWPAN_NHC_DEST_ID_0: u8 = 0xe6;
pub const LOWPAN_NHC_DEST_MASK_0: u8 = 0xfe;

// LOWPAN_NHC(nhc_dest, "RFC6282 Destination Options", NEXTHDR_DEST, 0,
//            LOWPAN_NHC_DEST_ID_0, LOWPAN_NHC_DEST_MASK_0, NULL, NULL);
// The LOWPAN_NHC registration macro and NEXTHDR_DEST are supplied by nhc.h.

// module_lowpan_nhc(nhc_dest);
// MODULE_DESCRIPTION("6LoWPAN next header RFC6282 Destination Options compression");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
