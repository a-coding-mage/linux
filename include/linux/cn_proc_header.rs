/*
 * cn_proc.h - process events connector
 *
 * Copyright (C) Matt Helsley, IBM Corp. 2005
 * Based on cn_fork.h by Nguyen Anh Quynh and Guillaume Thouvenin
 * Copyright (C) 2005 Nguyen Anh Quynh <aquynh@gmail.com>
 * Copyright (C) 2005 Guillaume Thouvenin <guillaume.thouvenin@bull.net>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2.1 of the GNU Lesser General Public License
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it would be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE.
 */

// Dependency equivalent of: #include <uapi/linux/cn_proc.h>

// Equivalent of the CONFIG_PROC_EVENTS conditional.
#[cfg(feature = "CONFIG_PROC_EVENTS")]
extern "C" {
    pub fn proc_fork_connector(task: *mut task_struct);
    pub fn proc_exec_connector(task: *mut task_struct);
    pub fn proc_id_connector(task: *mut task_struct, which_id: i32);
    pub fn proc_sid_connector(task: *mut task_struct);
    pub fn proc_ptrace_connector(task: *mut task_struct, which_id: i32);
    pub fn proc_comm_connector(task: *mut task_struct);
    pub fn proc_coredump_connector(task: *mut task_struct);
    pub fn proc_exit_connector(task: *mut task_struct);
}

#[cfg(not(feature = "CONFIG_PROC_EVENTS"))]
#[inline]
pub unsafe fn proc_fork_connector(_task: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_PROC_EVENTS"))]
#[inline]
pub unsafe fn proc_exec_connector(_task: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_PROC_EVENTS"))]
#[inline]
pub unsafe fn proc_id_connector(_task: *mut task_struct, _which_id: i32) {}

#[cfg(not(feature = "CONFIG_PROC_EVENTS"))]
#[inline]
pub unsafe fn proc_sid_connector(_task: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_PROC_EVENTS"))]
#[inline]
pub unsafe fn proc_comm_connector(_task: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_PROC_EVENTS"))]
#[inline]
pub unsafe fn proc_ptrace_connector(_task: *mut task_struct, _ptrace_id: i32) {}

#[cfg(not(feature = "CONFIG_PROC_EVENTS"))]
#[inline]
pub unsafe fn proc_coredump_connector(_task: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_PROC_EVENTS"))]
#[inline]
pub unsafe fn proc_exit_connector(_task: *mut task_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
