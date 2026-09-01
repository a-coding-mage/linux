// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Joe Lawrence <joe.lawrence@redhat.com>

// C dependency intent:
// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// #include <linux/module.h>
// #include <linux/kernel.h>

extern "C" {
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

static mut __func___test_klp_callbacks_mod_init: [core::ffi::c_char; 28] =
    *b"test_klp_callbacks_mod_init\0";
static mut __func___test_klp_callbacks_mod_exit: [core::ffi::c_char; 28] =
    *b"test_klp_callbacks_mod_exit\0";

unsafe extern "C" fn test_klp_callbacks_mod_init() -> core::ffi::c_int {
    pr_info(
        b"%s\n\0".as_ptr() as *const core::ffi::c_char,
        __func___test_klp_callbacks_mod_init.as_ptr(),
    );
    0
}

unsafe extern "C" fn test_klp_callbacks_mod_exit() {
    pr_info(
        b"%s\n\0".as_ptr() as *const core::ffi::c_char,
        __func___test_klp_callbacks_mod_exit.as_ptr(),
    );
}

// module_init(test_klp_callbacks_mod_init);
// module_exit(test_klp_callbacks_mod_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Joe Lawrence <joe.lawrence@redhat.com>");
// MODULE_DESCRIPTION("Livepatch test: target module");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
