/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Counter character device interface
 * Copyright (C) 2020 William Breathitt Gray
 */

// Dependency supplied by linux/counter.h.

extern "C" {
    pub fn counter_chrdev_add(counter: *mut counter_device) -> ::core::ffi::c_int;
    pub fn counter_chrdev_remove(counter: *mut counter_device);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
