// SPDX-License-Identifier: GPL-2.0-only
/*
 * This module emits "Hello, world" on printk when loaded.
 *
 * It is designed to be used for basic evaluation of the module loading
 * subsystem (for example when validating module signing/verification). It
 * lacks any extra dependencies, and will not normally be loaded by the
 * system unless explicitly requested by name.
 */

// C dependency: pr_fmt(fmt) expands to KBUILD_MODNAME ": " fmt.
// The Linux kernel headers and module registration macros are supplied by
// the surrounding build environment.

use core::ffi::c_char;

unsafe extern "C" {
    fn pr_warn(fmt: *const c_char, ...);
}

#[allow(non_upper_case_globals)]
pub const MODULE_AUTHOR: &str = "Kees Cook <keescook@chromium.org>";
#[allow(non_upper_case_globals)]
pub const MODULE_DESCRIPTION: &str = "module loading subsystem test module";
#[allow(non_upper_case_globals)]
pub const MODULE_LICENSE: &str = "GPL";

#[allow(non_snake_case)]
unsafe fn test_module_init() -> i32 {
    pr_warn(c"Hello, world\n".as_ptr());

    0
}

// C registration: module_init(test_module_init);

#[allow(non_snake_case)]
unsafe fn test_module_exit() {
    pr_warn(c"Goodbye\n".as_ptr());
}

// C registration: module_exit(test_module_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
