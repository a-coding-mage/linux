/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct console_cmdline {
    pub name: [kernel::ffi::c_char; 16], /* Name of the driver */
    pub index: kernel::ffi::c_int,       /* Minor dev. to use */
    pub devname: [kernel::ffi::c_char; 32], /* DEVNAME:0.0 style device name */
    pub user_specified: bool, /* Specified by command line vs. platform */
    pub options: *mut kernel::ffi::c_char, /* Options for the driver */
    // CONFIG_A11Y_BRAILLE_CONSOLE build-time condition.
    #[cfg(CONFIG_A11Y_BRAILLE_CONSOLE)]
    pub brl_options: *mut kernel::ffi::c_char, /* Options for braille driver */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
