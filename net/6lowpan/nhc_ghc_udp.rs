// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	6LoWPAN UDP compression according to RFC7400
 */

// Dependency supplied by nhc.h in the original source.

pub const LOWPAN_GHC_UDP_ID_0: u32 = 0xd0;
pub const LOWPAN_GHC_UDP_MASK_0: u32 = 0xf8;

lowpan_nhc!(
    ghc_udp,
    "RFC7400 UDP",
    NEXTHDR_UDP,
    0,
    LOWPAN_GHC_UDP_ID_0,
    LOWPAN_GHC_UDP_MASK_0,
    None,
    None
);

module_lowpan_nhc!(ghc_udp);
module_description!("6LoWPAN generic header UDP compression");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
