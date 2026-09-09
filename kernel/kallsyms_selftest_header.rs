/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency intent: declarations use the C-compatible `int` type.

unsafe extern "C" {
    pub static mut kallsyms_test_var_bss: i32;
    pub static mut kallsyms_test_var_data: i32;

    pub fn kallsyms_test_func() -> i32;
    // The C declaration is weak; linkage attributes are supplied by the build.
    pub fn kallsyms_test_func_weak() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
