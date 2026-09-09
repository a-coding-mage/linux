// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	6LoWPAN IPv6 Fragment Header compression according to RFC6282
 */

// Dependency supplied by the surrounding lowpan NHC implementation.

pub const LOWPAN_NHC_FRAGMENT_ID_0: u8 = 0xe4;
pub const LOWPAN_NHC_FRAGMENT_MASK_0: u8 = 0xfe;

// The C LOWPAN_NHC(...) macro registers the fragment NHC descriptor with the
// framework. Its descriptor type and registration mechanism are supplied by
// nhc.h and are therefore intentionally left as an external dependency here.
// LOWPAN_NHC(nhc_fragment, "RFC6282 Fragment", NEXTHDR_FRAGMENT, 0,
//            LOWPAN_NHC_FRAGMENT_ID_0, LOWPAN_NHC_FRAGMENT_MASK_0, NULL, NULL);

// Kernel-module registration and metadata supplied by the surrounding build.
// module_lowpan_nhc(nhc_fragment);
// MODULE_DESCRIPTION("6LoWPAN next header RFC6282 Fragment compression");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
