/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for using the procedures in btext.c.
 *
 * Benjamin Herrenschmidt <benh@kernel.crashing.org>
 */

// The declarations in this header are available only when building the kernel.

use core::ffi::{c_char, c_int, c_uint, c_ulong};

extern "C" {
    pub fn btext_find_display(allow_nonstdout: c_int) -> c_int;
    pub fn btext_update_display(
        phys: c_ulong,
        width: c_int,
        height: c_int,
        depth: c_int,
        pitch: c_int,
    );
    pub fn btext_setup_display(
        width: c_int,
        height: c_int,
        depth: c_int,
        pitch: c_int,
        address: c_ulong,
    );

    // CONFIG_PPC32 selects the external implementation in btext.c.
    #[cfg(feature = "CONFIG_PPC32")]
    pub fn btext_prepare_BAT();

    pub fn btext_map();
    pub fn btext_unmap();

    pub fn btext_drawchar(c: c_char);
    pub fn btext_drawstring(str_: *const c_char);
    pub fn btext_drawhex(v: c_ulong);
    pub fn btext_drawtext(c: *const c_char, len: c_uint);

    pub fn btext_clearscreen();
    pub fn btext_flushscreen();
    pub fn btext_flushline();
}

// When CONFIG_PPC32 is not enabled, the C header supplies an empty inline
// implementation. The Rust equivalent retains that behavior.
#[cfg(not(feature = "CONFIG_PPC32"))]
#[inline]
pub unsafe fn btext_prepare_BAT() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
