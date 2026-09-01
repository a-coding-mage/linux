/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header dependencies:
 *   <linux/types.h>
 *   <linux/bug.h>
 *   <linux/kernel.h>
 *   <linux/bitops.h>
 *   <linux/gfp.h>
 *   <linux/rcupdate.h>
 *
 * C header guards are intentionally omitted in Rust.
 */

/*
 * C fallback macros, when the kernel/module definitions are not already
 * provided by included headers.
 */
macro_rules! module_init {
    ($x:expr) => {};
}

macro_rules! module_exit {
    ($x:expr) => {};
}

macro_rules! MODULE_AUTHOR {
    ($x:expr) => {};
}

macro_rules! MODULE_LICENSE {
    ($x:expr) => {};
}

macro_rules! MODULE_DESCRIPTION {
    ($x:expr) => {};
}

macro_rules! dump_stack {
    () => {
        assert!(false)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
