// SPDX-License-Identifier: GPL-2.0+
/*
 * Helper functions to sync execution between parent and child processes.
 *
 * Copyright 2018, Thiago Jung Bauermann, IBM Corporation.
 */

use core::ffi::{c_char, c_int, c_uint};

// C dependencies from <semaphore.h> and the including translation unit.
unsafe extern "C" {
    pub static mut errno: c_int;

    pub fn perror(s: *const c_char);
    pub fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
    pub fn sem_destroy(sem: *mut sem_t) -> c_int;
    pub fn sem_wait(sem: *mut sem_t) -> c_int;
    pub fn sem_post(sem: *mut sem_t) -> c_int;
}

pub const ENODEV: c_int = 19;
pub const EINVAL: c_int = 22;

#[allow(non_camel_case_types)]
pub type sem_t = core::ffi::c_void;

/*
 * Information in a shared memory location for synchronization between child and
 * parent.
 */
#[repr(C)]
pub struct child_sync {
    /* The parent waits on this semaphore. */
    pub sem_parent: sem_t,

    /* If true, the child should give up as well. */
    pub parent_gave_up: bool,

    /* The child waits on this semaphore. */
    pub sem_child: sem_t,

    /* If true, the parent should give up as well. */
    pub child_gave_up: bool,
}

#[macro_export]
macro_rules! CHILD_FAIL_IF {
    ($x:expr, $sync:expr) => {{
        if $x {
            eprintln!("[FAIL] Test FAILED on line {}", line!());
            (*$sync).child_gave_up = true;
            prod_parent($sync);
            return 1;
        }
    }};
}

#[macro_export]
macro_rules! PARENT_FAIL_IF {
    ($x:expr, $sync:expr) => {{
        if $x {
            eprintln!("[FAIL] Test FAILED on line {}", line!());
            (*$sync).parent_gave_up = true;
            prod_child($sync);
            return 1;
        }
    }};
}

#[macro_export]
macro_rules! PARENT_SKIP_IF_UNSUPPORTED {
    ($x:expr, $sync:expr, $msg:expr) => {{
        if ($x) == -1 && (errno == ENODEV || errno == EINVAL) {
            (*$sync).parent_gave_up = true;
            prod_child($sync);
            SKIP_IF_MSG!(1, $msg);
        }
    }};
}

pub unsafe fn init_child_sync(sync: *mut child_sync) -> c_int {
    let mut ret: c_int;

    ret = sem_init(&mut (*sync).sem_parent, 1, 0);
    if ret != 0 {
        perror(c"Semaphore initialization failed".as_ptr());
        return 1;
    }

    ret = sem_init(&mut (*sync).sem_child, 1, 0);
    if ret != 0 {
        perror(c"Semaphore initialization failed".as_ptr());
        return 1;
    }

    return 0;
}

pub unsafe fn destroy_child_sync(sync: *mut child_sync) {
    sem_destroy(&mut (*sync).sem_parent);
    sem_destroy(&mut (*sync).sem_child);
}

pub unsafe fn wait_child(sync: *mut child_sync) -> c_int {
    let ret: c_int;

    /* Wait until the child prods us. */
    ret = sem_wait(&mut (*sync).sem_parent);
    if ret != 0 {
        perror(c"Error waiting for child".as_ptr());
        return 1;
    }

    return (*sync).child_gave_up as c_int;
}

pub unsafe fn prod_child(sync: *mut child_sync) -> c_int {
    let ret: c_int;

    /* Unblock the child now. */
    ret = sem_post(&mut (*sync).sem_child);
    if ret != 0 {
        perror(c"Error prodding child".as_ptr());
        return 1;
    }

    return 0;
}

pub unsafe fn wait_parent(sync: *mut child_sync) -> c_int {
    let ret: c_int;

    /* Wait until the parent prods us. */
    ret = sem_wait(&mut (*sync).sem_child);
    if ret != 0 {
        perror(c"Error waiting for parent".as_ptr());
        return 1;
    }

    return (*sync).parent_gave_up as c_int;
}

pub unsafe fn prod_parent(sync: *mut child_sync) -> c_int {
    let ret: c_int;

    /* Unblock the parent now. */
    ret = sem_post(&mut (*sync).sem_parent);
    if ret != 0 {
        perror(c"Error prodding parent".as_ptr());
        return 1;
    }

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
