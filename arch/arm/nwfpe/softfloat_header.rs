/* Translation of softfloat.h. */

pub type float32 = u32;
pub type float64 = u64;
pub type flag = u8;
pub type bits32 = u32;
pub type bits64 = u64;
pub type int32 = i32;

#[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
#[repr(C, packed(4))]
pub struct floatx80 {
    #[cfg(target_endian = "big")]
    pub __padding: u16,
    #[cfg(target_endian = "big")]
    pub high: u16,
    #[cfg(target_endian = "little")]
    pub high: u16,
    #[cfg(target_endian = "little")]
    pub __padding: u16,
    pub low: u64,
}

pub struct roundingData;

extern "C" {
    pub static mut float_detect_tininess: i8;
    pub fn float_raise(a: i8);

    pub fn int32_to_float32(a: *mut roundingData, b: i32) -> float32;
    pub fn int32_to_float64(a: i32) -> float64;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn int32_to_floatx80(a: i32) -> floatx80;

    pub fn float32_to_int32(a: *mut roundingData, b: float32) -> i32;
    pub fn float32_to_int32_round_to_zero(a: float32) -> i32;
    pub fn float32_to_float64(a: float32) -> float64;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn float32_to_floatx80(a: float32) -> floatx80;

    pub fn float32_round_to_int(a: *mut roundingData, b: float32) -> float32;
    pub fn float32_add(a: *mut roundingData, b: float32, c: float32) -> float32;
    pub fn float32_sub(a: *mut roundingData, b: float32, c: float32) -> float32;
    pub fn float32_mul(a: *mut roundingData, b: float32, c: float32) -> float32;
    pub fn float32_div(a: *mut roundingData, b: float32, c: float32) -> float32;
    pub fn float32_rem(a: *mut roundingData, b: float32, c: float32) -> float32;
    pub fn float32_sqrt(a: *mut roundingData, b: float32) -> float32;
    pub fn float32_eq(a: float32, b: float32) -> i8;
    pub fn float32_le(a: float32, b: float32) -> i8;
    pub fn float32_lt(a: float32, b: float32) -> i8;
    pub fn float32_eq_signaling(a: float32, b: float32) -> i8;
    pub fn float32_le_quiet(a: float32, b: float32) -> i8;
    pub fn float32_lt_quiet(a: float32, b: float32) -> i8;
    pub fn float32_is_signaling_nan(a: float32) -> i8;

    pub fn float64_to_int32(a: *mut roundingData, b: float64) -> i32;
    pub fn float64_to_int32_round_to_zero(a: float64) -> i32;
    pub fn float64_to_float32(a: *mut roundingData, b: float64) -> float32;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn float64_to_floatx80(a: float64) -> floatx80;

    pub fn float64_round_to_int(a: *mut roundingData, b: float64) -> float64;
    pub fn float64_add(a: *mut roundingData, b: float64, c: float64) -> float64;
    pub fn float64_sub(a: *mut roundingData, b: float64, c: float64) -> float64;
    pub fn float64_mul(a: *mut roundingData, b: float64, c: float64) -> float64;
    pub fn float64_div(a: *mut roundingData, b: float64, c: float64) -> float64;
    pub fn float64_rem(a: *mut roundingData, b: float64, c: float64) -> float64;
    pub fn float64_sqrt(a: *mut roundingData, b: float64) -> float64;
    pub fn float64_eq(a: float64, b: float64) -> i8;
    pub fn float64_le(a: float64, b: float64) -> i8;
    pub fn float64_lt(a: float64, b: float64) -> i8;
    pub fn float64_eq_signaling(a: float64, b: float64) -> i8;
    pub fn float64_le_quiet(a: float64, b: float64) -> i8;
    pub fn float64_lt_quiet(a: float64, b: float64) -> i8;
    pub fn float64_is_signaling_nan(a: float64) -> i8;

    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_to_int32(a: *mut roundingData, b: floatx80) -> i32;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_to_int32_round_to_zero(a: floatx80) -> i32;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_to_float32(a: *mut roundingData, b: floatx80) -> float32;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_to_float64(a: *mut roundingData, b: floatx80) -> float64;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_round_to_int(a: *mut roundingData, b: floatx80) -> floatx80;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_add(a: *mut roundingData, b: floatx80, c: floatx80) -> floatx80;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_sub(a: *mut roundingData, b: floatx80, c: floatx80) -> floatx80;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_mul(a: *mut roundingData, b: floatx80, c: floatx80) -> floatx80;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_div(a: *mut roundingData, b: floatx80, c: floatx80) -> floatx80;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_rem(a: *mut roundingData, b: floatx80, c: floatx80) -> floatx80;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_sqrt(a: *mut roundingData, b: floatx80) -> floatx80;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_eq(a: floatx80, b: floatx80) -> i8;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_le(a: floatx80, b: floatx80) -> i8;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_lt(a: floatx80, b: floatx80) -> i8;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_eq_signaling(a: floatx80, b: floatx80) -> i8;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_le_quiet(a: floatx80, b: floatx80) -> i8;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_lt_quiet(a: floatx80, b: floatx80) -> i8;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_is_signaling_nan(a: floatx80) -> i8;
    #[cfg(feature = "CONFIG_FPE_NWFPE_XP")]
    pub fn floatx80_is_nan(a: floatx80) -> flag;

    pub fn float32_is_nan(a: float32) -> flag;
    pub fn float64_is_nan(a: float64) -> flag;
    pub fn float64_to_uint32(a: *mut roundingData, b: float64) -> int32;
    pub fn float64_to_uint32_round_to_zero(a: float64) -> int32;
}

pub const float_tininess_after_rounding: i32 = 0;
pub const float_tininess_before_rounding: i32 = 1;
pub const float_round_nearest_even: i32 = 0;
pub const float_round_to_zero: i32 = 1;
pub const float_round_down: i32 = 2;
pub const float_round_up: i32 = 3;
pub const float_flag_invalid: i32 = 1;
pub const float_flag_divbyzero: i32 = 2;
pub const float_flag_overflow: i32 = 4;
pub const float_flag_underflow: i32 = 8;
pub const float_flag_inexact: i32 = 16;

#[inline]
pub fn extractFloat32Sign(a: float32) -> flag { (a >> 31) as flag }

#[inline]
pub fn float32_eq_nocheck(a: float32, b: float32) -> flag {
    (a == b || ((a | b).wrapping_shl(1) == 0)) as flag
}

#[inline]
pub fn float32_lt_nocheck(a: float32, b: float32) -> flag {
    let a_sign = extractFloat32Sign(a);
    let b_sign = extractFloat32Sign(b);
    if a_sign != b_sign {
        return (a_sign != 0 && (a | b).wrapping_shl(1) != 0) as flag;
    }
    ((a != b) && ((a_sign != 0) ^ (a < b))) as flag
}

#[inline]
pub fn extractFloat64Sign(a: float64) -> flag { (a >> 63) as flag }

#[inline]
pub fn float64_eq_nocheck(a: float64, b: float64) -> flag {
    (a == b || ((a | b).wrapping_shl(1) == 0)) as flag
}

#[inline]
pub fn float64_lt_nocheck(a: float64, b: float64) -> flag {
    let a_sign = extractFloat64Sign(a);
    let b_sign = extractFloat64Sign(b);
    if a_sign != b_sign {
        return (a_sign != 0 && (a | b).wrapping_shl(1) != 0) as flag;
    }
    ((a != b) && ((a_sign != 0) ^ (a < b))) as flag
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
