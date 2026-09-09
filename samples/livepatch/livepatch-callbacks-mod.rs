// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017 Joe Lawrence <joe.lawrence@redhat.com>
 */

/*
 * livepatch-callbacks-mod.c - (un)patching callbacks demo support module
 *
 * Purpose
 * -------
 *
 * Simple module to demonstrate livepatch (un)patching callbacks.
 *
 * Usage
 * -----
 *
 * This module is not intended to be standalone.  See the "Usage"
 * section of livepatch-callbacks-demo.c.
 */

// C headers provide these kernel declarations.
use core::ffi::c_char;

extern "C" {
    fn pr_info(fmt: *const c_char, ...);
}

static LIVEPATCH_CALLBACKS_MOD_INIT_FMT: &[u8] = b"livepatch_callbacks_mod_init\n\0";
static LIVEPATCH_CALLBACKS_MOD_EXIT_FMT: &[u8] = b"livepatch_callbacks_mod_exit\n\0";

unsafe fn livepatch_callbacks_mod_init() -> i32 {
    pr_info(LIVEPATCH_CALLBACKS_MOD_INIT_FMT.as_ptr() as *const c_char);
    0
}

unsafe fn livepatch_callbacks_mod_exit() {
    pr_info(LIVEPATCH_CALLBACKS_MOD_EXIT_FMT.as_ptr() as *const c_char);
}

// Equivalent to module_init(livepatch_callbacks_mod_init).
// Equivalent to module_exit(livepatch_callbacks_mod_exit).
const MODULE_DESCRIPTION: &str =
    "Live patching demo for (un)patching callbacks, support module";
const MODULE_LICENSE: &str = "GPL";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
