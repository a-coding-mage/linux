/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2025, Collabora Ltd.
 */

// Dependencies supplied by the surrounding Linux bitfield/build_bug/limits
// environment are intentionally referenced rather than implemented here.

/**
 * FIELD_PREP_WM16() - prepare a bitfield element with a mask in the upper half
 * @_mask: shifted mask defining the field's length and position
 * @_val:  value to put in the field
 *
 * FIELD_PREP_WM16() masks and shifts up the value, as well as bitwise ORs the
 * result with the mask shifted up by 16.
 *
 * This is useful for a common design of hardware registers where the upper
 * 16-bit half of a 32-bit register is used as a write-enable mask. In such a
 * register, a bit in the lower half is only updated if the corresponding bit
 * in the upper half is high.
 */
#[macro_export]
macro_rules! FIELD_PREP_WM16 {
    ($mask:expr, $val:expr) => {{
        let __val = $val;
        let __mask = $mask;
        // Equivalent to __BF_FIELD_CHECK(__mask, ((u16)0U), __val,
        // "HWORD_UPDATE: ");
        (((__val as u32) << __bf_shf(__mask)) & (__mask as u32)) |
            ((__mask as u32) << 16)
    }};
}

/**
 * FIELD_PREP_WM16_CONST() - prepare a constant bitfield element with a mask in
 *                           the upper half
 * @_mask: shifted mask defining the field's length and position
 * @_val:  value to put in the field
 *
 * FIELD_PREP_WM16_CONST() masks and shifts up the value, as well as bitwise ORs
 * the result with the mask shifted up by 16.
 *
 * This is useful for a common design of hardware registers where the upper
 * 16-bit half of a 32-bit register is used as a write-enable mask. In such a
 * register, a bit in the lower half is only updated if the corresponding bit
 * in the upper half is high.
 *
 * Unlike FIELD_PREP_WM16(), this is a constant expression and can therefore
 * be used in initializers. Error checking is less comfortable for this
 * version.
 */
#[macro_export]
macro_rules! FIELD_PREP_WM16_CONST {
    ($mask:expr, $val:expr) => {
        FIELD_PREP_CONST!($mask, $val) |
            (BUILD_BUG_ON_ZERO!(const_true((($mask) as u64) > U16_MAX)) +
                (($mask) << 16))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
