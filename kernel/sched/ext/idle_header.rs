/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
 *
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 * Copyright (c) 2024 Andrea Righi <arighi@nvidia.com>
 */

// Dependency supplied externally: <linux/btf_ids.h>

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sched_ext_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    pub static mut scx_kfunc_ids_idle: btf_id_set8;
    pub static mut scx_kfunc_ids_select_cpu: btf_id_set8;

    pub fn scx_idle_update_selcpu_topology(ops: *mut sched_ext_ops);
    pub fn scx_idle_init_masks();

    pub fn scx_select_cpu_dfl(
        p: *mut task_struct,
        prev_cpu: s32,
        wake_flags: u64,
        cpus_allowed: *const cpumask,
        flags: u64,
    ) -> s32;
    pub fn scx_idle_enable(ops: *mut sched_ext_ops);
    pub fn scx_idle_disable();
    pub fn scx_idle_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
