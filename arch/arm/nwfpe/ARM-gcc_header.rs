/* SPDX-License-Identifier: GPL-2.0 */
/*
-------------------------------------------------------------------------------
The macro `BITS64' can be defined to indicate that 64-bit integer types are
supported by the compiler.
-------------------------------------------------------------------------------
*/
pub const BITS64: bool = true;

/* Integer types corresponding to the C typedefs in the source header. */
pub type flag = i8; // C `char'
pub type uint8 = u8;
pub type int8 = i8;
pub type uint16 = i32;
pub type int16 = i32;
pub type uint32 = u32;
pub type int32 = i32;
pub type bits64 = u64;
pub type sbits64 = i64;

pub type bits8 = u8;
pub type sbits8 = i8;
pub type bits16 = u16;
pub type sbits16 = i16;
pub type bits32 = u32;
pub type sbits32 = i32;
pub type uint64 = u64;
pub type int64 = i64;

/* C's token-pasting LIT64(a) macro; Rust callers provide an integer expression. */
#[macro_export]
macro_rules! LIT64 {
    ($a:expr) => { $a as i64 };
}

/* INLINE was `static inline' in C. Rust functions are declared inline where used. */

/* For use as a GCC soft-float library we need some special function names. */
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float32_add { ($($args:tt)*) => { __addsf3($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float32_sub { ($($args:tt)*) => { __subsf3($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float32_mul { ($($args:tt)*) => { __mulsf3($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float32_div { ($($args:tt)*) => { __divsf3($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! int32_to_float32 { ($($args:tt)*) => { __floatsisf($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float32_to_int32_round_to_zero { ($($args:tt)*) => { __fixsfsi($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float32_to_uint32_round_to_zero { ($($args:tt)*) => { __fixunssfsi($($args)*) }; }

#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float32_eq { ($($args:tt)*) => { ___float32_eq($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float32_le { ($($args:tt)*) => { ___float32_le($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float32_lt { ($($args:tt)*) => { ___float32_lt($($args)*) }; }

#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float64_add { ($($args:tt)*) => { ___float64_add($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float64_sub { ($($args:tt)*) => { ___float64_sub($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float64_mul { ($($args:tt)*) => { ___float64_mul($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float64_div { ($($args:tt)*) => { ___float64_div($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! int32_to_float64 { ($($args:tt)*) => { ___int32_to_float64($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float64_to_int32_round_to_zero { ($($args:tt)*) => { ___float64_to_int32_round_to_zero($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float64_to_uint32_round_to_zero { ($($args:tt)*) => { ___float64_to_uint32_round_to_zero($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float64_to_float32 { ($($args:tt)*) => { ___float64_to_float32($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float32_to_float64 { ($($args:tt)*) => { ___float32_to_float64($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float64_eq { ($($args:tt)*) => { ___float64_eq($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float64_le { ($($args:tt)*) => { ___float64_le($($args)*) }; }
#[cfg(feature = "__LIBFLOAT__")]
macro_rules! float64_lt { ($($args:tt)*) => { ___float64_lt($($args)*) }; }

/* The source's #if 0 alternate mappings are intentionally inactive. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
