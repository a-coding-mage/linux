// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * 6LoWPAN Extension Header compression according to RFC7400
 */

// Dependency provided by the surrounding 6LoWPAN NHC implementation.

pub const LOWPAN_GHC_EXT_DEST_ID_0: u8 = 0xb6;
pub const LOWPAN_GHC_EXT_DEST_MASK_0: u8 = 0xfe;

lowpan_nhc!(
    ghc_ext_dest,
    "RFC7400 Destination Extension Header",
    NEXTHDR_DEST,
    0,
    LOWPAN_GHC_EXT_DEST_ID_0,
    LOWPAN_GHC_EXT_DEST_MASK_0,
    None,
    None
);

module_lowpan_nhc!(ghc_ext_dest);
module_description!("6LoWPAN generic header destination extension compression");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
