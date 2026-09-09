/* SPDX-License-Identifier: MIT */
/* Copyright © 2026 Intel Corporation */

/*
 * Wrappers over the generic fixed width BIT_U*() and GENMASK_U*()
 * implementations, for compatibility reasons with previous implementation.
 *
 * The referenced GENMASK_U*, BIT_U*, FIELD_GET, FIELD_MAX, and build-time
 * checking helpers are supplied by the surrounding kernel translation.
 */

macro_rules! REG_GENMASK { ($high:expr, $low:expr) => { GENMASK_U32!($high, $low) }; }
macro_rules! REG_GENMASK64 { ($high:expr, $low:expr) => { GENMASK_U64!($high, $low) }; }
macro_rules! REG_GENMASK16 { ($high:expr, $low:expr) => { GENMASK_U16!($high, $low) }; }
macro_rules! REG_GENMASK8 { ($high:expr, $low:expr) => { GENMASK_U8!($high, $low) }; }

macro_rules! REG_BIT { ($n:expr) => { BIT_U32!($n) }; }
macro_rules! REG_BIT64 { ($n:expr) => { BIT_U64!($n) }; }
macro_rules! REG_BIT16 { ($n:expr) => { BIT_U16!($n) }; }
macro_rules! REG_BIT8 { ($n:expr) => { BIT_U8!($n) }; }

/* Local integer constant expression version of is_power_of_2(). */
macro_rules! IS_POWER_OF_2 {
    ($x:expr) => { (($x) != 0 && (($x) & (($x) - 1)) == 0) };
}

/**
 * REG_FIELD_PREP8() - Prepare a u8 bitfield value
 * @__mask: shifted mask defining the field's length and position
 * @__val: value to put in the field
 *
 * Local copy of FIELD_PREP() to generate an integer constant expression, force
 * u8 and for consistency with REG_FIELD_GET8(), REG_BIT8() and REG_GENMASK8().
 *
 * @return: @__val masked and shifted into the field defined by @__mask.
 */
macro_rules! REG_FIELD_PREP8 {
    ($mask:expr, $val:expr) => { (($val as u8) << BF_SHF!($mask) & ($mask)) as u8 };
}

/** REG_FIELD_PREP16() - Prepare a u16 bitfield value. */
macro_rules! REG_FIELD_PREP16 {
    ($mask:expr, $val:expr) => { (($val as u16) << BF_SHF!($mask) & ($mask)) as u16 };
}

/** REG_FIELD_PREP() - Prepare a u32 bitfield value. */
macro_rules! REG_FIELD_PREP {
    ($mask:expr, $val:expr) => { (($val as u32) << BF_SHF!($mask) & ($mask)) as u32 };
}

/** REG_FIELD_GET8() - Extract a u8 bitfield value. */
macro_rules! REG_FIELD_GET8 {
    ($mask:expr, $val:expr) => { FIELD_GET!($mask, $val) as u8 };
}

/** REG_FIELD_GET() - Extract a u32 bitfield value. */
macro_rules! REG_FIELD_GET {
    ($mask:expr, $val:expr) => { FIELD_GET!($mask, $val) as u32 };
}

/** REG_FIELD_GET64() - Extract a u64 bitfield value. */
macro_rules! REG_FIELD_GET64 {
    ($mask:expr, $val:expr) => { FIELD_GET!($mask, $val) as u64 };
}

/** REG_FIELD_MAX() - produce the maximum value representable by a field. */
macro_rules! REG_FIELD_MAX {
    ($mask:expr) => { FIELD_MAX!($mask) as u32 };
}

macro_rules! REG_MASKED_FIELD {
    ($mask:expr, $value:expr) => { (($mask) << 16 | ($value)) };
}

macro_rules! REG_MASKED_FIELD_ENABLE {
    ($a:expr) => { REG_MASKED_FIELD!($a, $a) };
}

macro_rules! REG_MASKED_FIELD_DISABLE {
    ($a:expr) => { REG_MASKED_FIELD!($a, 0) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
