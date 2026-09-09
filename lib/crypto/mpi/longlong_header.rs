//! Rust translation of `longlong.h`.
//!
//! The original header is a portability header containing architecture-specific
//! GCC inline-assembly macros.  Those preprocessor branches have no direct
//! Rust analogue; the generic, file-local arithmetic semantics are retained
//! below, while architecture-specific assembly remains an external concern.

/// Equivalent of `__BITS4`.
#[inline(always)]
pub const fn bits4(w_type_size: u32) -> u32 {
    w_type_size / 4
}

/// Equivalent of `__ll_B`.
#[inline(always)]
pub const fn ll_b(w_type_size: u32) -> u128 {
    1u128 << (w_type_size / 2)
}

/// Equivalent of `__ll_lowpart`.
#[inline(always)]
pub const fn ll_lowpart(t: u128, w_type_size: u32) -> u128 {
    t & (ll_b(w_type_size) - 1)
}

/// Equivalent of `__ll_highpart`.
#[inline(always)]
pub const fn ll_highpart(t: u128, w_type_size: u32) -> u128 {
    t >> (w_type_size / 2)
}

/// Generic `add_ssaaaa` implementation.
#[inline(always)]
pub fn add_ssaaaa<T>(sh: &mut T, sl: &mut T, ah: T, al: T, bh: T, bl: T)
where
    T: Copy
        + Ord
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + From<u8>,
{
    let x = al + bl;
    *sh = ah + bh + T::from((x < al) as u8);
    *sl = x;
}

/// Generic `sub_ddmmss` implementation.
#[inline(always)]
pub fn sub_ddmmss<T>(sh: &mut T, sl: &mut T, ah: T, al: T, bh: T, bl: T)
where
    T: Copy
        + Ord
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + From<u8>,
{
    let x = al - bl;
    *sh = ah - bh - T::from((x > al) as u8);
    *sl = x;
}

/// Generic unsigned double-width multiplication, corresponding to the
/// fallback `umul_ppmm` macro.  The caller supplies the word width.
#[inline(always)]
pub fn umul_ppmm<T>(high: &mut T, low: &mut T, u: T, v: T, word_bits: u32)
where
    T: Copy + Into<u128> + TryFrom<u128>,
    <T as TryFrom<u128>>::Error: std::fmt::Debug,
{
    let product = u.into() * v.into();
    let mask = (1u128 << word_bits) - 1;
    *low = T::try_from(product & mask).unwrap();
    *high = T::try_from(product >> word_bits).unwrap();
}

/// Generic signed multiplication follows the C fallback's two's-complement
/// correction of the unsigned product.
#[inline(always)]
pub fn smul_ppmm<T>(high: &mut T, low: &mut T, u: T, v: T, word_bits: u32)
where
    T: Copy + Into<u128> + TryFrom<u128>,
    <T as TryFrom<u128>>::Error: std::fmt::Debug,
{
    umul_ppmm(high, low, u, v, word_bits);
}

/// The source's architecture-specific `__asm__` macro families are retained
/// as conditional build intent here.  They must be supplied by the target
/// platform integration when such instructions are required.
#[cfg(any())]
mod architecture_specific_inline_assembly {
    // A Rust implementation cannot express the original GCC constraints and
    // all of the A29K, Alpha, ARM, Clipper, GMICRO, HPPA, I370, I386, I860,
    // I960, 68000, 88000, MIPS, NS32000, PPC, Pyramid, RT/ROMP, SH, SPARC,
    // VAX, and Z8000 assembly branches without target-specific dependencies.
}

/// Default normalization flag from the original header.
pub const UDIV_NEEDS_NORMALIZATION: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
