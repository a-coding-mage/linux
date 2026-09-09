// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit 'Hooks' implementation.
 *
 * This file contains code / structures which should be built-in even when
 * KUnit itself is built as a module.
 *
 * Copyright (C) 2022, Google LLC.
 * Author: David Gow <davidgow@google.com>
 */

// Dependency corresponding to: #include <kunit/test-bug.h>

// DEFINE_STATIC_KEY_FALSE(kunit_running);
// The concrete static-key type and initializer are supplied by the kernel
// dependency represented by the included header.
extern "C" {
    pub static mut kunit_running: kunit_static_key_false;
}

// EXPORT_SYMBOL(kunit_running);

/* Function pointers for hooks. */
extern "C" {
    pub static mut kunit_hooks: kunit_hooks_table;
}

// EXPORT_SYMBOL(kunit_hooks);

// External types supplied by the corresponding kernel headers.
#[allow(non_camel_case_types)]
pub type kunit_static_key_false = crate::kunit_static_key_false;
pub type kunit_hooks_table = crate::kunit_hooks_table;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
