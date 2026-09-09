/* SPDX-License-Identifier: GPL-2.0 */

// `u64` is supplied by the Linux type definitions included by the C header.
// Rust has no native bit-field syntax; the packed storage below preserves the
// 128-bit layout, while the masks document the corresponding named bits.
#[repr(C, packed)]
pub struct CssGeneralChar {
    pub bits: [u64; 2],
}

impl CssGeneralChar {
    pub const DYNIO: u128 = 1u128 << 12;
    pub const EADM: u128 = 1u128 << 17;
    pub const AIF: u128 = 1u128 << 41;
    pub const MCSS: u128 = 1u128 << 45;
    pub const FCS: u128 = 1u128 << 46;
    pub const EXT_MB: u128 = 1u128 << 48;
    pub const AIF_TDD: u128 = 1u128 << 56;
    pub const QEBSM: u128 = 1u128 << 58;
    pub const AIV: u128 = 1u128 << 61;
    pub const AIF_QDIO: u128 = 1u128 << 67;
    pub const EADM_RF: u128 = 1u128 << 80;
    pub const CIB: u128 = 1u128 << 82;
    pub const FCX: u128 = 1u128 << 88;
    pub const ALT_SSI: u128 = 1u128 << 108;
    pub const NARF: u128 = 1u128 << 110;
    pub const ENARF: u128 = 1u128 << 116;
    pub const UTIL_STR: u128 = 1u128 << 123;
}

extern "C" {
    pub static mut css_general_characteristics: CssGeneralChar;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
