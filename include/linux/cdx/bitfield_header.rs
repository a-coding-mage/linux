/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2005-2006 Fen Systems Ltd.
 * Copyright 2006-2013 Solarflare Communications Inc.
 * Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
 */

// Dependency equivalent of <linux/bitfield.h> is supplied externally.

/* Lowest bit numbers and widths */
pub const CDX_DWORD_LBN: u32 = 0;
pub const CDX_DWORD_WIDTH: u32 = 32;

/* Specified attribute (e.g. LBN) of the specified field.
 * Rust has no stable identifier concatenation in macro_rules!, so callers
 * pass the corresponding attribute expression directly to these macros. */
#[macro_export]
macro_rules! CDX_VAL {
    ($field:expr, $attribute:expr) => { $attribute };
}

#[macro_export]
macro_rules! CDX_LOW_BIT {
    ($field:expr) => { $field.0 };
}

#[macro_export]
macro_rules! CDX_WIDTH {
    ($field:expr) => { $field.1 };
}

#[macro_export]
macro_rules! CDX_HIGH_BIT {
    ($field:expr) => { $field.0 + $field.1 - 1 };
}

/* A doubleword (i.e. 4 byte) datatype - little-endian in HW */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_dword {
    pub cdx_u32: u32,
}

#[inline]
pub fn CDX_DWORD_VAL(dword: &cdx_dword) -> u32 {
    u32::from_le(dword.cdx_u32)
}

#[inline]
pub fn cdx_field_get(mask: u32, value: u32) -> u32 {
    (value & mask) >> mask.trailing_zeros()
}

#[inline]
pub fn cdx_field_prep(mask: u32, value: u32) -> u32 {
    (value << mask.trailing_zeros()) & mask
}

/* Extract bit field portion [low,high) from the 32-bit little-endian
 * element which contains bits [min,max) */
#[macro_export]
macro_rules! CDX_DWORD_FIELD {
    ($dword:expr, $field:expr) => {
        $crate::cdx_field_get(
            (((1u32 << ($field.1 + $field.0)) - 1) & !((1u32 << $field.0) - 1)),
            u32::from_le(($dword).cdx_u32),
        )
    };
}

/* Creates the portion of the named bit field that lies within the range
 * [min,max). */
#[macro_export]
macro_rules! CDX_INSERT_FIELD {
    ($field:expr, $value:expr) => {
        $crate::cdx_field_prep(
            (((1u32 << ($field.1 + $field.0)) - 1) & !((1u32 << $field.0) - 1)),
            $value,
        )
    };
}

/* Creates the portion of the named bit fields that lie within the range
 * [min,max). */
#[macro_export]
macro_rules! CDX_INSERT_FIELDS {
    ($field1:expr, $value1:expr, $field2:expr, $value2:expr,
     $field3:expr, $value3:expr, $field4:expr, $value4:expr,
     $field5:expr, $value5:expr, $field6:expr, $value6:expr,
     $field7:expr, $value7:expr) => {
        CDX_INSERT_FIELD!($field1, $value1) |
        CDX_INSERT_FIELD!($field2, $value2) |
        CDX_INSERT_FIELD!($field3, $value3) |
        CDX_INSERT_FIELD!($field4, $value4) |
        CDX_INSERT_FIELD!($field5, $value5) |
        CDX_INSERT_FIELD!($field6, $value6) |
        CDX_INSERT_FIELD!($field7, $value7)
    };
}

#[macro_export]
macro_rules! CDX_POPULATE_DWORD {
    ($dword:expr, $($args:expr),+ $(,)?) => {
        ($dword).cdx_u32 = u32::to_le(CDX_INSERT_FIELDS!($($args),+));
    };
}

/* Populate a dword field with various numbers of arguments */
#[macro_export]
macro_rules! CDX_POPULATE_DWORD_7 { ($dword:expr, $($args:expr),+ $(,)?) => { CDX_POPULATE_DWORD!($dword, $($args),+); }; }
#[macro_export]
macro_rules! CDX_POPULATE_DWORD_6 { ($dword:expr, $($args:expr),+ $(,)?) => { CDX_POPULATE_DWORD_7!($dword, CDX_DWORD, 0, $($args),+); }; }
#[macro_export]
macro_rules! CDX_POPULATE_DWORD_5 { ($dword:expr, $($args:expr),+ $(,)?) => { CDX_POPULATE_DWORD_6!($dword, CDX_DWORD, 0, $($args),+); }; }
#[macro_export]
macro_rules! CDX_POPULATE_DWORD_4 { ($dword:expr, $($args:expr),+ $(,)?) => { CDX_POPULATE_DWORD_5!($dword, CDX_DWORD, 0, $($args),+); }; }
#[macro_export]
macro_rules! CDX_POPULATE_DWORD_3 { ($dword:expr, $($args:expr),+ $(,)?) => { CDX_POPULATE_DWORD_4!($dword, CDX_DWORD, 0, $($args),+); }; }
#[macro_export]
macro_rules! CDX_POPULATE_DWORD_2 { ($dword:expr, $($args:expr),+ $(,)?) => { CDX_POPULATE_DWORD_3!($dword, CDX_DWORD, 0, $($args),+); }; }
#[macro_export]
macro_rules! CDX_POPULATE_DWORD_1 { ($dword:expr, $($args:expr),+ $(,)?) => { CDX_POPULATE_DWORD_2!($dword, CDX_DWORD, 0, $($args),+); }; }
#[macro_export]
macro_rules! CDX_SET_DWORD { ($dword:expr) => { CDX_POPULATE_DWORD_1!($dword, CDX_DWORD, 0xffffffff); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
