/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_A11Y_BRAILLE_CONSOLE conditional from the original header. */

#[cfg(feature = "CONFIG_A11Y_BRAILLE_CONSOLE")]
#[inline]
pub unsafe fn braille_set_options(
    c: *mut console_cmdline,
    brl_options: *mut core::ffi::c_char,
) {
    (*c).brl_options = brl_options;
}

#[cfg(feature = "CONFIG_A11Y_BRAILLE_CONSOLE")]
unsafe extern "C" {
    pub fn _braille_console_setup(
        str_: *mut *mut core::ffi::c_char,
        brl_options: *mut *mut core::ffi::c_char,
    ) -> core::ffi::c_int;

    pub fn _braille_register_console(
        console: *mut console,
        c: *mut console_cmdline,
    ) -> core::ffi::c_int;

    pub fn _braille_unregister_console(console: *mut console) -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_A11Y_BRAILLE_CONSOLE"))]
#[inline]
pub unsafe fn braille_set_options(
    _c: *mut console_cmdline,
    _brl_options: *mut core::ffi::c_char,
) {
}

#[cfg(not(feature = "CONFIG_A11Y_BRAILLE_CONSOLE"))]
#[inline]
pub unsafe fn _braille_console_setup(
    _str: *mut *mut core::ffi::c_char,
    _brl_options: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_A11Y_BRAILLE_CONSOLE"))]
#[inline]
pub unsafe fn _braille_register_console(
    _console: *mut console,
    _c: *mut console_cmdline,
) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_A11Y_BRAILLE_CONSOLE"))]
#[inline]
pub unsafe fn _braille_unregister_console(_console: *mut console) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
