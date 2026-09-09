/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding Linux translation.

/* Metric prefixes in accordance with Système international (d'unités) */
pub const PETA: u64 = 1000000000000000;
pub const TERA: u64 = 1000000000000;
pub const GIGA: u64 = 1000000000;
pub const MEGA: u64 = 1000000;
pub const KILO: u64 = 1000;
pub const HECTO: u64 = 100;
pub const DECA: u64 = 10;
pub const DECI: u64 = 10;
pub const CENTI: u64 = 100;
pub const MILLI: u64 = 1000;
pub const MICRO: u64 = 1000000;
pub const NANO: u64 = 1000000000;
pub const PICO: u64 = 1000000000000;
pub const FEMTO: u64 = 1000000000000000;

/*
 * Percentage and related scaling units
 *
 * These macros define scaling factors used to convert between ratio and
 * percentage-based representations with different decimal resolutions.
 * They are used for precise fractional calculations in engineering, finance,
 * and measurement applications.
 *
 * Examples:
 *   1%     = 0.01    = 1 / PERCENT
 *   0.1%   = 0.001   = 1 / PERMILLE
 *   0.01%  = 0.0001  = 1 / PERMYRIAD (1 basis point)
 *   0.001% = 0.00001 = 1 / PERCENTMILLE
 */
pub const PERCENT: i64 = 100;
pub const PERMILLE: i64 = 1000;
pub const PERMYRIAD: i64 = 10000;
pub const PERCENTMILLE: i64 = 100000;

pub const NANOHZ_PER_HZ: u64 = 1000000000;
pub const MICROHZ_PER_HZ: u64 = 1000000;
pub const MILLIHZ_PER_HZ: u64 = 1000;

/* Hz based multipliers */
pub const HZ_PER_KHZ: u64 = 1000;
pub const HZ_PER_MHZ: u64 = 1000000;
pub const HZ_PER_GHZ: u64 = 1000000000;

/* kHz based multipliers */
pub const KHZ_PER_MHZ: u64 = 1000;
pub const KHZ_PER_GHZ: u64 = 1000000;

pub const MILLIWATT_PER_WATT: u64 = 1000;
pub const MICROWATT_PER_MILLIWATT: u64 = 1000;
pub const MICROWATT_PER_WATT: u64 = 1000000;

pub const MICROJOULE_PER_JOULE: u64 = 1000000;
pub const NANOJOULE_PER_JOULE: u64 = 1000000000;

pub const BYTES_PER_KBIT: u64 = KILO / BITS_PER_BYTE;
pub const BYTES_PER_MBIT: u64 = MEGA / BITS_PER_BYTE;
pub const BYTES_PER_GBIT: u64 = GIGA / BITS_PER_BYTE;

pub const ABSOLUTE_ZERO_MILLICELSIUS: i64 = -273150;

#[inline]
pub fn milli_kelvin_to_millicelsius(t: i64) -> i64 {
    t + ABSOLUTE_ZERO_MILLICELSIUS
}

#[inline]
pub fn millicelsius_to_milli_kelvin(t: i64) -> i64 {
    t - ABSOLUTE_ZERO_MILLICELSIUS
}

pub const MILLIDEGREE_PER_DEGREE: i64 = 1000;
pub const MILLIDEGREE_PER_DECIDEGREE: i64 = 100;

#[inline]
pub fn kelvin_to_millicelsius(t: i64) -> i64 {
    milli_kelvin_to_millicelsius(t * MILLIDEGREE_PER_DEGREE)
}

#[inline]
pub fn millicelsius_to_kelvin(mut t: i64) -> i64 {
    t = millicelsius_to_milli_kelvin(t);
    DIV_ROUND_CLOSEST(t, MILLIDEGREE_PER_DEGREE)
}

#[inline]
pub fn deci_kelvin_to_celsius(mut t: i64) -> i64 {
    t = milli_kelvin_to_millicelsius(t * MILLIDEGREE_PER_DECIDEGREE);
    DIV_ROUND_CLOSEST(t, MILLIDEGREE_PER_DEGREE)
}

#[inline]
pub fn celsius_to_deci_kelvin(mut t: i64) -> i64 {
    t = millicelsius_to_milli_kelvin(t * MILLIDEGREE_PER_DEGREE);
    DIV_ROUND_CLOSEST(t, MILLIDEGREE_PER_DECIDEGREE)
}

/**
 * deci_kelvin_to_millicelsius_with_offset - convert Kelvin to Celsius
 * @t: temperature value in decidegrees Kelvin
 * @offset: difference between Kelvin and Celsius in millidegrees
 *
 * Return: temperature value in millidegrees Celsius
 */
#[inline]
pub fn deci_kelvin_to_millicelsius_with_offset(t: i64, offset: i64) -> i64 {
    t * MILLIDEGREE_PER_DECIDEGREE - offset
}

#[inline]
pub fn deci_kelvin_to_millicelsius(t: i64) -> i64 {
    milli_kelvin_to_millicelsius(t * MILLIDEGREE_PER_DECIDEGREE)
}

#[inline]
pub fn millicelsius_to_deci_kelvin(mut t: i64) -> i64 {
    t = millicelsius_to_milli_kelvin(t);
    DIV_ROUND_CLOSEST(t, MILLIDEGREE_PER_DECIDEGREE)
}

#[inline]
pub fn kelvin_to_celsius(t: i64) -> i64 {
    t + DIV_ROUND_CLOSEST(ABSOLUTE_ZERO_MILLICELSIUS, MILLIDEGREE_PER_DEGREE)
}

#[inline]
pub fn celsius_to_kelvin(t: i64) -> i64 {
    t - DIV_ROUND_CLOSEST(ABSOLUTE_ZERO_MILLICELSIUS, MILLIDEGREE_PER_DEGREE)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
