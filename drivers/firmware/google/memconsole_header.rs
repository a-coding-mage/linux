/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * memconsole.h
 *
 * Internal headers of the memory based BIOS console.
 *
 * Copyright 2017 Google Inc.
 */

/* Dependency equivalent of <linux/types.h>. */

/*
 * memconsole_setup
 *
 * Initialize the memory console, passing the function to handle read accesses.
 */
extern "C" {
    pub fn memconsole_setup(
        read_func: Option<unsafe extern "C" fn(
            *mut ::core::ffi::c_char,
            i64,
            usize,
        ) -> isize>,
    );

    /*
     * memconsole_sysfs_init
     *
     * Update memory console length and create binary file
     * for firmware object.
     */
    pub fn memconsole_sysfs_init() -> ::core::ffi::c_int;

    /* memconsole_exit
     *
     * Unmap the console buffer.
     */
    pub fn memconsole_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
