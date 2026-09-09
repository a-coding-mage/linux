// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	6LoWPAN Extension Header compression according to RFC7400
 */

// C dependency: #include "nhc.h"

pub const LOWPAN_GHC_EXT_FRAG_ID_0: u8 = 0xb4;
pub const LOWPAN_GHC_EXT_FRAG_MASK_0: u8 = 0xfe;

// C registration macro from nhc.h:
// LOWPAN_NHC(ghc_ext_frag, "RFC7400 Fragmentation Extension Header",
//            NEXTHDR_FRAGMENT, 0, LOWPAN_GHC_EXT_FRAG_ID_0,
//            LOWPAN_GHC_EXT_FRAG_MASK_0, NULL, NULL);

// C module registration macro from the kernel/module environment:
// module_lowpan_nhc(ghc_ext_frag);
// MODULE_DESCRIPTION("6LoWPAN generic header fragmentation extension compression");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
