// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	6LoWPAN ICMPv6 compression according to RFC7400
 */

// Dependency equivalent of: #include "nhc.h"

pub const LOWPAN_GHC_ICMPV6_ID_0: u8 = 0xdf;
pub const LOWPAN_GHC_ICMPV6_MASK_0: u8 = 0xff;

// Equivalent of LOWPAN_NHC(ghc_icmpv6, "RFC7400 ICMPv6", NEXTHDR_ICMP, 0,
//                          LOWPAN_GHC_ICMPV6_ID_0, LOWPAN_GHC_ICMPV6_MASK_0,
//                          NULL, NULL);
// The LOWPAN_NHC registration macro is supplied by the dependent NHC layer.

// Equivalent of module_lowpan_nhc(ghc_icmpv6);
// Equivalent module metadata:
// MODULE_DESCRIPTION("6LoWPAN generic header ICMPv6 compression");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
