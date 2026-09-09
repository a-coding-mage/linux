/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * padata.h - header for the padata parallelization interface
 *
 * Copyright (C) 2008, 2009 secunet Security Networks AG
 * Copyright (C) 2008, 2009 Steffen Klassert <steffen.klassert@secunet.com>
 *
 * Copyright (c) 2020 Oracle and/or its affiliates.
 * Author: Daniel Jordan <daniel.m.jordan@oracle.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub const PADATA_CPU_SERIAL: u32 = 0x01;
pub const PADATA_CPU_PARALLEL: u32 = 0x02;

#[repr(C)]
pub struct padata_priv {
    pub list: list_head,
    pub pd: *mut parallel_data,
    pub cb_cpu: i32,
    pub seq_nr: u32,
    pub info: i32,
    pub parallel: Option<unsafe extern "C" fn(padata: *mut padata_priv)>,
    pub serial: Option<unsafe extern "C" fn(padata: *mut padata_priv)>,
}

#[repr(C)]
pub struct padata_list {
    pub list: list_head,
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct padata_serial_queue {
    pub serial: padata_list,
    pub work: work_struct,
    pub pd: *mut parallel_data,
}

#[repr(C)]
pub struct padata_cpumask {
    pub pcpu: cpumask_var_t,
    pub cbcpu: cpumask_var_t,
}

#[repr(C)]
pub struct parallel_data {
    pub ps: *mut padata_shell,
    pub reorder_list: *mut padata_list,
    pub squeue: *mut padata_serial_queue,
    pub refcnt: refcount_t,
    pub seq_nr: u32,
    pub processed: u32,
    pub cpu: i32,
    pub cpumask: padata_cpumask,
}

#[repr(C)]
pub struct padata_shell {
    pub pinst: *mut padata_instance,
    pub pd: *mut parallel_data,
    pub opd: *mut parallel_data,
    pub list: list_head,
}

#[repr(C)]
pub struct padata_mt_job {
    pub thread_fn: Option<unsafe extern "C" fn(start: c_ulong, end: c_ulong, arg: *mut c_void)>,
    pub fn_arg: *mut c_void,
    pub start: c_ulong,
    pub size: c_ulong,
    pub align: c_ulong,
    pub min_chunk: c_ulong,
    pub max_threads: i32,
    pub numa_aware: bool,
}

#[repr(C)]
pub struct padata_instance {
    pub cpuhp_node: hlist_node,
    pub parallel_wq: *mut workqueue_struct,
    pub serial_wq: *mut workqueue_struct,
    pub pslist: list_head,
    pub cpumask: padata_cpumask,
    pub validate_cpumask: cpumask_var_t,
    pub kobj: kobject,
    pub lock: mutex,
    pub flags: u8,
}

pub const PADATA_INIT: u8 = 1;
pub const PADATA_RESET: u8 = 2;
pub const PADATA_INVALID: u8 = 4;

#[cfg(CONFIG_PADATA)]
unsafe extern "C" {
    pub fn padata_init();
    pub fn padata_alloc(name: *const c_char) -> *mut padata_instance;
    pub fn padata_free(pinst: *mut padata_instance);
    pub fn padata_alloc_shell(pinst: *mut padata_instance) -> *mut padata_shell;
    pub fn padata_free_shell(ps: *mut padata_shell);
    pub fn padata_do_parallel(
        ps: *mut padata_shell,
        padata: *mut padata_priv,
        cb_cpu: *mut i32,
    ) -> i32;
    pub fn padata_do_serial(padata: *mut padata_priv);
    pub fn padata_do_multithreaded(job: *mut padata_mt_job);
    pub fn padata_set_cpumask(
        pinst: *mut padata_instance,
        cpumask_type: i32,
        cpumask: cpumask_var_t,
    ) -> i32;
}

#[cfg(not(CONFIG_PADATA))]
pub unsafe fn padata_init() {}

#[cfg(not(CONFIG_PADATA))]
pub unsafe fn padata_do_multithreaded(job: *mut padata_mt_job) {
    ((*job).thread_fn.unwrap())(
        (*job).start,
        (*job).start.wrapping_add((*job).size),
        (*job).fn_arg,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
