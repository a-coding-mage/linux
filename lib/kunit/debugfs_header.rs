/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020, Oracle and/or its affiliates.
 */

// Dependency provided by <kunit/test.h>.
use crate::kunit::test::kunit_suite;

#[cfg(CONFIG_KUNIT_DEBUGFS)]
extern "C" {
    pub fn kunit_debugfs_create_suite(suite: *mut kunit_suite);
    pub fn kunit_debugfs_destroy_suite(suite: *mut kunit_suite);
    pub fn kunit_debugfs_init();
    pub fn kunit_debugfs_cleanup();
}

#[cfg(not(CONFIG_KUNIT_DEBUGFS))]
#[inline]
pub unsafe fn kunit_debugfs_create_suite(_suite: *mut kunit_suite) {}

#[cfg(not(CONFIG_KUNIT_DEBUGFS))]
#[inline]
pub unsafe fn kunit_debugfs_destroy_suite(_suite: *mut kunit_suite) {}

#[cfg(not(CONFIG_KUNIT_DEBUGFS))]
#[inline]
pub unsafe fn kunit_debugfs_init() {}

#[cfg(not(CONFIG_KUNIT_DEBUGFS))]
#[inline]
pub unsafe fn kunit_debugfs_cleanup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
