// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * 6LoWPAN Extension Header compression according to RFC7400
 */

// Dependency supplied by the surrounding 6LoWPAN implementation:
// #include "nhc.h"

const LOWPAN_GHC_EXT_HOP_ID_0: u8 = 0xb0;
const LOWPAN_GHC_EXT_HOP_MASK_0: u8 = 0xfe;

// LOWPAN_NHC(ghc_ext_hop, "RFC7400 Hop-by-Hop Extension Header", NEXTHDR_HOP,
//            0, LOWPAN_GHC_EXT_HOP_ID_0, LOWPAN_GHC_EXT_HOP_MASK_0, NULL, NULL);
LOWPAN_NHC!(
    ghc_ext_hop,
    "RFC7400 Hop-by-Hop Extension Header",
    NEXTHDR_HOP,
    0,
    LOWPAN_GHC_EXT_HOP_ID_0,
    LOWPAN_GHC_EXT_HOP_MASK_0,
    None,
    None
);

// module_lowpan_nhc(ghc_ext_hop);
module_lowpan_nhc!(ghc_ext_hop);

// MODULE_DESCRIPTION("6LoWPAN generic header hop-by-hop extension compression");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
