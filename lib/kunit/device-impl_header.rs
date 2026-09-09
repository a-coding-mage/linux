/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KUnit internal header for device helpers
 *
 * Header for KUnit-internal driver / bus management.
 *
 * Copyright (C) 2023, Google LLC.
 * Author: David Gow <davidgow@google.com>
 */

// For internal use only -- registers the kunit_bus.
extern "C" {
    pub fn kunit_bus_init() -> core::ffi::c_int;
}

// For internal use only -- unregisters the kunit_bus.
extern "C" {
    pub fn kunit_bus_shutdown();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
