/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The CONFIG_LANTIQ branch includes the external lantiq_soc.h declarations.
 * Those dependencies are supplied by other translated files.
 */
// Under CONFIG_LANTIQ, declarations from <lantiq_soc.h> are supplied externally.

#[cfg(not(feature = "CONFIG_LANTIQ"))]
pub const LTQ_EARLY_ASC: i32 = 0;

#[cfg(not(feature = "CONFIG_LANTIQ"))]
#[macro_export]
macro_rules! CPHYSADDR {
    ($a:expr) => {
        0
    };
}

#[cfg(not(feature = "CONFIG_LANTIQ"))]
#[allow(non_camel_case_types)]
pub struct clk;

#[cfg(not(feature = "CONFIG_LANTIQ"))]
#[inline]
pub fn clk_get_fpi() -> *mut clk {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
