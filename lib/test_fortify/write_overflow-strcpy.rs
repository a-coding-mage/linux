// SPDX-License-Identifier: GPL-2.0-only

// Dependency supplied by the original "test_fortify.h" include.
unsafe extern "C" {
    fn strcpy(dest: *mut core::ffi::c_char, src: *const core::ffi::c_char)
        -> *mut core::ffi::c_char;
}

macro_rules! TEST {
    () => {{
        unsafe { strcpy(small, large_src) }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
