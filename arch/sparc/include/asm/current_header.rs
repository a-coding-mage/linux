/* SPDX-License-Identifier: GPL-2.0 */
/* include/asm/current.h
 *
 * Copyright (C) 1999 IBM Deutschland Entwicklung GmbH, IBM Corporation
 * Copyright (C) 2002 Pete Zaitcev (zaitcev@yahoo.com)
 * Copyright (C) 2007 David S. Miller (davem@davemloft.net)
 *
 * Derived from "include/asm-s390/current.h" by
 * Martin Schwidefsky (schwidefsky@de.ibm.com)
 * Derived from "include/asm-i386/current.h"
 */

/* C dependency: <linux/thread_info.h> */

#[cfg(CONFIG_SPARC64)]
extern "C" {
    /* C: register struct task_struct *current asm("g4"); */
    pub static mut current: *mut task_struct;
}

#[cfg(CONFIG_SPARC32)]
pub struct task_struct;

#[cfg(CONFIG_SPARC32)]
extern "C" {
    pub fn current_thread_info() -> *mut thread_info;
}

#[cfg(CONFIG_SPARC32)]
pub unsafe fn __get_current() -> *mut task_struct {
    (*current_thread_info()).task
}

/*
 * C macro:
 *     #define current __get_current()
 *
 * Rust has no direct equivalent for a function-like object-replacing macro;
 * callers should use __get_current() at the corresponding call site.
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
