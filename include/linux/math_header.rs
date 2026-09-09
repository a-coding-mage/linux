/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding translated kernel headers. */

/*
 * This looks more complex than it should. But we need to
 * get the type for the ~ right in round_down (it needs to be
 * as wide as the result!), and we want to evaluate the macro
 * arguments just once each.
 */
macro_rules! __round_mask {
    ($x:expr, $y:expr) => { (($y) - 1) as _ };
}

/**
 * round_up - round up to next specified power of 2
 * @x: the value to round
 * @y: multiple to round up to (must be a power of 2)
 */
macro_rules! round_up {
    ($x:expr, $y:expr) => { ((($x) - 1) | __round_mask!($x, $y)) + 1 };
}

/**
 * round_down - round down to next specified power of 2
 * @x: the value to round
 * @y: multiple to round down to (must be a power of 2)
 */
macro_rules! round_down {
    ($x:expr, $y:expr) => { ($x) & !__round_mask!($x, $y) };
}

macro_rules! DIV_ROUND_UP_POW2 {
    ($n:expr, $d:expr) => { ($n) / ($d) + if (($n) & (($d) - 1)) != 0 { 1 } else { 0 } };
}

macro_rules! DIV_ROUND_UP {
    ($n:expr, $d:expr) => { (($n) + ($d) - 1) / ($d) };
}

macro_rules! DIV_ROUND_DOWN_ULL {
    ($ll:expr, $d:expr) => {{
        let mut _tmp = $ll as u64;
        _tmp /= $d as u64;
        _tmp
    }};
}

macro_rules! DIV_ROUND_UP_ULL {
    ($ll:expr, $d:expr) => {
        DIV_ROUND_DOWN_ULL!(($ll as u64) + ($d as u64) - 1, $d)
    };
}

/* The C header selects this according to BITS_PER_LONG. */
macro_rules! DIV_ROUND_UP_SECTOR_T {
    ($ll:expr, $d:expr) => { DIV_ROUND_UP!($ll, $d) };
}

macro_rules! roundup {
    ($x:expr, $y:expr) => {{
        let __y = $y;
        (($x) + (__y - 1)) / __y * __y
    }};
}

macro_rules! rounddown {
    ($x:expr, $y:expr) => {{
        let __x = $x;
        __x - (__x % ($y))
    }};
}

macro_rules! DIV_ROUND_CLOSEST {
    ($x:expr, $d:expr) => { (($x) + ($d) / 2) / ($d) };
}

/* Same as above but for u64 dividends. divisor must be a 32-bit number. */
macro_rules! DIV_ROUND_CLOSEST_ULL {
    ($x:expr, $divisor:expr) => {{
        let __d = $divisor;
        let mut _tmp = ($x as u64) + (__d as u64) / 2;
        _tmp /= __d as u64;
        _tmp
    }};
}

#[repr(C)]
pub struct s8_fract { pub numerator: i8, pub denominator: i8 }
#[repr(C)]
pub struct u8_fract { pub numerator: u8, pub denominator: u8 }
#[repr(C)]
pub struct s16_fract { pub numerator: i16, pub denominator: i16 }
#[repr(C)]
pub struct u16_fract { pub numerator: u16, pub denominator: u16 }
#[repr(C)]
pub struct s32_fract { pub numerator: i32, pub denominator: i32 }
#[repr(C)]
pub struct u32_fract { pub numerator: u32, pub denominator: u32 }

/* Calculate "x * n / d" without unnecessary overflow or loss of precision. */
macro_rules! mult_frac {
    ($x:expr, $n:expr, $d:expr) => {{
        let x_ = $x;
        let n_ = $n;
        let d_ = $d;
        let q = x_ / d_;
        let r = x_ % d_;
        q * n_ + r * n_ / d_
    }};
}

macro_rules! sector_div {
    ($a:expr, $b:expr) => {{
        $a /= $b;
        $a
    }};
}

/** abs - return absolute value of an argument */
macro_rules! abs {
    ($x:expr) => {{
        let __x = $x;
        if __x < 0 { -__x } else { __x }
    }};
}

/** abs_diff - return absolute value of the difference between the arguments */
macro_rules! abs_diff {
    ($a:expr, $b:expr) => {{
        let __a = $a;
        let __b = $b;
        if __a > __b { __a - __b } else { __b - __a }
    }};
}

/** reciprocal_scale - "scale" a value into range [0, ep_ro) */
#[inline]
pub fn reciprocal_scale(val: u32, ep_ro: u32) -> u32 {
    (((val as u64) * (ep_ro as u64)) >> 32) as u32
}

extern "C" {
    pub fn int_pow(base: u64, exp: u32) -> u64;
    pub fn int_sqrt(x: c_ulong) -> c_ulong;
}

/* If BITS_PER_LONG < 64, this is an external declaration; otherwise it is inline. */
#[inline]
pub unsafe fn int_sqrt64(x: u64) -> u32 {
    int_sqrt(x as c_ulong) as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
