/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct console_cmdline {
    pub name: [core::ffi::c_char; 16], /* Name of the driver */
    pub index: core::ffi::c_int,       /* Minor dev. to use */
    pub devname: [core::ffi::c_char; 32], /* DEVNAME:0.0 style device name */
    pub user_specified: bool, /* Specified by command line vs. platform */
    pub options: *mut core::ffi::c_char, /* Options for the driver */
    // CONFIG_A11Y_BRAILLE_CONSOLE build-time condition.
    #[cfg(feature = "CONFIG_A11Y_BRAILLE_CONSOLE")]
    pub brl_options: *mut core::ffi::c_char, /* Options for braille driver */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
