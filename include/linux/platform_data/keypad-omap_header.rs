/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright (C) 2006 Komal Shah <komal_shah802003@yahoo.com>
 */

/*
 * The C header provides these definitions when CONFIG_ARCH_OMAP1 is not set.
 * The build-time condition is preserved here for the eventual Rust
 * configuration; the referenced register accessors are supplied externally.
 */
#[cfg(not(CONFIG_ARCH_OMAP1))]
#[inline(always)]
pub unsafe fn omap_readw<T>(_reg: T) -> i32 {
    0
}

#[cfg(not(CONFIG_ARCH_OMAP1))]
#[inline(always)]
pub unsafe fn omap_writew<T>(_val: T, _reg: T) {
}

#[repr(C)]
pub struct omap_kp_platform_data {
    pub rows: core::ffi::c_int,
    pub cols: core::ffi::c_int,
    pub keymap_data: *const matrix_keymap_data,
    pub rep: bool,
    pub delay: core::ffi::c_ulong,
    pub dbounce: bool,
}

/* Group (0..3) -- when multiple keys are pressed, only the
 * keys pressed in the same group are considered as pressed. This is
 * in order to workaround certain crappy HW designs that produce ghost
 * keypresses. Two free bits, not used by neither row/col nor keynum,
 * must be available for use as group bits. The below GROUP_SHIFT
 * macro definition is based on some prior knowledge of the
 * matrix_keypad defined KEY() macro internals.
 */
pub const GROUP_SHIFT: u32 = 14;
pub const GROUP_0: u32 = 0 << GROUP_SHIFT;
pub const GROUP_1: u32 = 1 << GROUP_SHIFT;
pub const GROUP_2: u32 = 2 << GROUP_SHIFT;
pub const GROUP_3: u32 = 3 << GROUP_SHIFT;
pub const GROUP_MASK: u32 = GROUP_3;

/* The C header rejects configurations where key-number bits overlap the
 * group bits: `#if KEY_MAX & GROUP_MASK` / `#error ...`.
 * KEY_MAX is supplied by the matrix-keypad dependency.
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
