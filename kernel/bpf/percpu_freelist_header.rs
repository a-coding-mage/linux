/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2016 Facebook
 */

#[repr(C)]
pub struct pcpu_freelist_head {
    pub first: *mut pcpu_freelist_node,
    pub lock: rqspinlock_t,
}

#[repr(C)]
pub struct pcpu_freelist {
    /* C __percpu pointer; per-CPU storage semantics are supplied externally. */
    pub freelist: *mut pcpu_freelist_head,
}

#[repr(C)]
pub struct pcpu_freelist_node {
    pub next: *mut pcpu_freelist_node,
}

/* pcpu_freelist_* do spin_lock_irqsave. */
extern "C" {
    pub fn pcpu_freelist_push(
        s: *mut pcpu_freelist,
        node: *mut pcpu_freelist_node,
    );
    pub fn pcpu_freelist_pop(s: *mut pcpu_freelist) -> *mut pcpu_freelist_node;
    /* __pcpu_freelist_* do spin_lock only. caller must disable irqs. */
    pub fn __pcpu_freelist_push(
        s: *mut pcpu_freelist,
        node: *mut pcpu_freelist_node,
    );
    pub fn __pcpu_freelist_pop(s: *mut pcpu_freelist) -> *mut pcpu_freelist_node;
    pub fn pcpu_freelist_populate(
        s: *mut pcpu_freelist,
        buf: *mut core::ffi::c_void,
        elem_size: u32,
        nr_elems: u32,
    );
    pub fn pcpu_freelist_init(s: *mut pcpu_freelist) -> i32;
    pub fn pcpu_freelist_destroy(s: *mut pcpu_freelist);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
