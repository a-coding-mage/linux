// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * 6LoWPAN IPv6 Mobility Header compression according to RFC6282
 */

// The declarations and registration helpers below are supplied by nhc.h in
// the C implementation. Their concrete Rust representation belongs to the
// surrounding 6LoWPAN NHC framework.

pub const LOWPAN_NHC_MOBILITY_ID_0: u8 = 0xe8;
pub const LOWPAN_NHC_MOBILITY_MASK_0: u8 = 0xfe;

// LOWPAN_NHC(nhc_mobility, "RFC6282 Mobility", NEXTHDR_MOBILITY, 0,
//            LOWPAN_NHC_MOBILITY_ID_0, LOWPAN_NHC_MOBILITY_MASK_0, NULL, NULL);
// Registers the RFC6282 Mobility NHC descriptor named `nhc_mobility` with the
// next-header value NEXTHDR_MOBILITY, encoding value 0xe8, mask 0xfe, and no
// encode/decode callbacks.

// module_lowpan_nhc(nhc_mobility);
// MODULE_DESCRIPTION("6LoWPAN next header RFC6282 Mobility compression");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
