// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	6LoWPAN IPv6 Header compression according to RFC6282
 */

// Dependency equivalent of: #include "nhc.h"

pub const LOWPAN_NHC_IPV6_ID_0: u8 = 0xee;
pub const LOWPAN_NHC_IPV6_MASK_0: u8 = 0xfe;

// The LOWPAN_NHC macro is defined by nhc.h and supplies the registration
// object and implementation-specific type details.
// LOWPAN_NHC(
//     nhc_ipv6,
//     "RFC6282 IPv6",
//     NEXTHDR_IPV6,
//     0,
//     LOWPAN_NHC_IPV6_ID_0,
//     LOWPAN_NHC_IPV6_MASK_0,
//     NULL,
//     NULL
// );

// The module_lowpan_nhc macro registers nhc_ipv6 with the lowpan NHC module.
// module_lowpan_nhc(nhc_ipv6);

// Module metadata emitted by the kernel build system.
// MODULE_DESCRIPTION("6LoWPAN next header RFC6282 IPv6 compression");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
