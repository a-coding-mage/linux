/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// #include <linux/kconfig.h>
// #include <linux/types.h>

unsafe extern "C" {
    pub fn video_get_options(name: *const core::ffi::c_char) -> *const core::ffi::c_char;
}

// Conditional on IS_ENABLED(CONFIG_FB_CORE) in the C build.
// Exported for compatibility with fbdev; don't use in new code.
unsafe extern "C" {
    pub fn __video_get_options(
        name: *const core::ffi::c_char,
        option: *mut *const core::ffi::c_char,
        is_of: bool,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
