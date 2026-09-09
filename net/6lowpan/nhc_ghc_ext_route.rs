// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	6LoWPAN Extension Header compression according to RFC7400
 */

// Dependency supplied by the surrounding lowpan NHC implementation.

pub const LOWPAN_GHC_EXT_ROUTE_ID_0: u8 = 0xb2;
pub const LOWPAN_GHC_EXT_ROUTE_MASK_0: u8 = 0xfe;

LOWPAN_NHC!(
    ghc_ext_route,
    "RFC7400 Routing Extension Header",
    NEXTHDR_ROUTING,
    0,
    LOWPAN_GHC_EXT_ROUTE_ID_0,
    LOWPAN_GHC_EXT_ROUTE_MASK_0,
    core::ptr::null_mut(),
    core::ptr::null_mut()
);

module_lowpan_nhc!(ghc_ext_route);
MODULE_DESCRIPTION!("6LoWPAN generic header routing extension compression");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
