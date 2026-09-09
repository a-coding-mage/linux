/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by video/sh_mobile_lcdc.h.

// The C condition is preserved as a Rust configuration condition. The
// corresponding configuration feature names are supplied by the build.
#[cfg(any(feature = "CONFIG_FB_SH_MOBILE_LCDC", feature = "CONFIG_FB_SH_MOBILE_LCDC_MODULE"))]
unsafe extern "C" {
    pub fn kfr2r09_lcd_setup(
        sys_ops_handle: *mut core::ffi::c_void,
        sys_ops: *mut sh_mobile_lcdc_sys_bus_ops,
    ) -> core::ffi::c_int;

    pub fn kfr2r09_lcd_start(
        sys_ops_handle: *mut core::ffi::c_void,
        sys_ops: *mut sh_mobile_lcdc_sys_bus_ops,
    );
}

// Opaque type supplied by video/sh_mobile_lcdc.h.
#[allow(non_camel_case_types)]
pub enum sh_mobile_lcdc_sys_bus_ops {}

#[cfg(not(any(feature = "CONFIG_FB_SH_MOBILE_LCDC", feature = "CONFIG_FB_SH_MOBILE_LCDC_MODULE")))]
pub unsafe fn kfr2r09_lcd_setup(
    _sys_ops_handle: *mut core::ffi::c_void,
    _sys_ops: *mut sh_mobile_lcdc_sys_bus_ops,
) -> core::ffi::c_int {
    -ENODEV
}

#[cfg(not(any(feature = "CONFIG_FB_SH_MOBILE_LCDC", feature = "CONFIG_FB_SH_MOBILE_LCDC_MODULE")))]
pub unsafe fn kfr2r09_lcd_start(
    _sys_ops_handle: *mut core::ffi::c_void,
    _sys_ops: *mut sh_mobile_lcdc_sys_bus_ops,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
