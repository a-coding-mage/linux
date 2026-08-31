// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta, Inc */

/*
 * C header guard __SCX_USERLAND_COMMON_H omitted in Rust.
 */

/*
 * An instance of a task that has been enqueued by the kernel for consumption
 * by a user space global scheduler thread.
 */
#[repr(C)]
pub struct scx_userland_enqueued_task {
    pub pid: i32,
    pub sum_exec_runtime: u64,
    pub weight: u64,
}
