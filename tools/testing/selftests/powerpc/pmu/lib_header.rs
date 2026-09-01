/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

/* C dependencies removed from executable Rust:
 * <stdbool.h>, <stdio.h>, <stdint.h>, <string.h>, <unistd.h>
 */

use core::ffi::c_int;

/* pid_t is supplied by <unistd.h> in the original C header. */
#[allow(non_camel_case_types)]
pub type pid_t = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_fields {
    pub read_fd: c_int,
    pub write_fd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pipe {
    pub fields: pipe_fields,
    pub fds: [c_int; 2],
}

unsafe extern "C" {
    pub fn kill_child_and_wait(child_pid: pid_t) -> c_int;
    pub fn wait_for_child(child_pid: pid_t) -> c_int;
    pub fn sync_with_child(read_pipe: pipe, write_pipe: pipe) -> c_int;
    pub fn wait_for_parent(read_pipe: pipe) -> c_int;
    pub fn notify_parent(write_pipe: pipe) -> c_int;
    pub fn notify_parent_of_error(write_pipe: pipe) -> c_int;
    pub fn eat_cpu(test_function: Option<unsafe extern "C" fn() -> c_int>) -> pid_t;
    pub fn require_paranoia_below(level: c_int) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_range {
    pub first: u64,
    pub last: u64,
}

unsafe extern "C" {
    pub static mut libc: addr_range;
    pub static mut vdso: addr_range;
}

unsafe extern "C" {
    pub fn parse_proc_maps() -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
