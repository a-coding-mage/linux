// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2016 Facebook
 */

// Declarations supplied by percpu_freelist.h and the kernel environment are
// intentionally left external here.

use core::ffi::c_void;

#[repr(C)]
pub struct raw_res_spinlock_t {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct pcpu_freelist_node {
    pub next: *mut pcpu_freelist_node,
}

#[repr(C)]
pub struct pcpu_freelist_head {
    pub lock: raw_res_spinlock_t,
    pub first: *mut pcpu_freelist_node,
}

#[repr(C)]
pub struct pcpu_freelist {
    pub freelist: *mut pcpu_freelist_head,
}

extern "C" {
    fn alloc_percpu() -> *mut pcpu_freelist_head;
    fn free_percpu(ptr: *mut pcpu_freelist_head);
    fn raw_res_spin_lock_init(lock: *mut raw_res_spinlock_t);
    fn raw_res_spin_lock(lock: *mut raw_res_spinlock_t) -> i32;
    fn raw_res_spin_unlock(lock: *mut raw_res_spinlock_t);
    fn this_cpu_ptr(ptr: *mut pcpu_freelist_head) -> *mut pcpu_freelist_head;
    fn per_cpu_ptr(ptr: *mut pcpu_freelist_head, cpu: i32) -> *mut pcpu_freelist_head;
    fn raw_smp_processor_id() -> i32;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn num_possible_cpus() -> u32;
    fn for_each_possible_cpu(cpu: *mut i32);
    fn for_each_cpu_wrap(cpu: *mut i32, mask: *const c_void, start: i32);
    static cpu_possible_mask: c_void;
}

#[inline]
unsafe fn pcpu_freelist_push_node(
    head: *mut pcpu_freelist_head,
    node: *mut pcpu_freelist_node,
) {
    (*node).next = (*head).first;
    core::ptr::write_volatile(&mut (*head).first, node);
}

#[inline]
unsafe fn ___pcpu_freelist_push(
    head: *mut pcpu_freelist_head,
    node: *mut pcpu_freelist_node,
) -> bool {
    if raw_res_spin_lock(&mut (*head).lock) != 0 {
        return false;
    }
    pcpu_freelist_push_node(head, node);
    raw_res_spin_unlock(&mut (*head).lock);
    true
}

pub unsafe fn pcpu_freelist_init(s: *mut pcpu_freelist) -> i32 {
    let mut cpu: i32 = 0;

    (*s).freelist = alloc_percpu();
    if (*s).freelist.is_null() {
        return -12;
    }

    for_each_possible_cpu(&mut cpu);
    let head = per_cpu_ptr((*s).freelist, cpu);
    raw_res_spin_lock_init(&mut (*head).lock);
    (*head).first = core::ptr::null_mut();
    0
}

pub unsafe fn pcpu_freelist_destroy(s: *mut pcpu_freelist) {
    free_percpu((*s).freelist);
}

pub unsafe fn __pcpu_freelist_push(
    s: *mut pcpu_freelist,
    node: *mut pcpu_freelist_node,
) {
    let mut head: *mut pcpu_freelist_head;
    let mut cpu: i32 = 0;

    if ___pcpu_freelist_push(this_cpu_ptr((*s).freelist), node) {
        return;
    }

    loop {
        for_each_cpu_wrap(&mut cpu, &cpu_possible_mask, raw_smp_processor_id());
        if cpu == raw_smp_processor_id() {
            continue;
        }
        head = per_cpu_ptr((*s).freelist, cpu);
        if raw_res_spin_lock(&mut (*head).lock) != 0 {
            continue;
        }
        pcpu_freelist_push_node(head, node);
        raw_res_spin_unlock(&mut (*head).lock);
        return;
    }
}

pub unsafe fn pcpu_freelist_push(s: *mut pcpu_freelist, node: *mut pcpu_freelist_node) {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    __pcpu_freelist_push(s, node);
    local_irq_restore(flags);
}

pub unsafe fn pcpu_freelist_populate(
    s: *mut pcpu_freelist,
    mut buf: *mut u8,
    elem_size: u32,
    nr_elems: u32,
) {
    let mut head: *mut pcpu_freelist_head;
    let mut cpu: i32 = 0;
    let mut cpu_idx: u32 = 0;
    let mut i: u32;
    let n = nr_elems / num_possible_cpus();
    let m = nr_elems % num_possible_cpus();

    for_each_possible_cpu(&mut cpu);
    head = per_cpu_ptr((*s).freelist, cpu);
    let j = n + if cpu_idx < m { 1 } else { 0 };
    i = 0;
    while i < j {
        // No locking required as this is not visible yet.
        pcpu_freelist_push_node(head, buf as *mut pcpu_freelist_node);
        buf = buf.add(elem_size as usize);
        i += 1;
    }
    cpu_idx += 1;
}

unsafe fn ___pcpu_freelist_pop(s: *mut pcpu_freelist) -> *mut pcpu_freelist_node {
    let mut node: *mut pcpu_freelist_node = core::ptr::null_mut();
    let mut head: *mut pcpu_freelist_head;
    let mut cpu: i32 = 0;

    for_each_cpu_wrap(&mut cpu, &cpu_possible_mask, raw_smp_processor_id());
    head = per_cpu_ptr((*s).freelist, cpu);
    if core::ptr::read_volatile(&(*head).first).is_null() {
        return node;
    }
    if raw_res_spin_lock(&mut (*head).lock) != 0 {
        return node;
    }
    node = (*head).first;
    if !node.is_null() {
        core::ptr::write_volatile(&mut (*head).first, (*node).next);
        raw_res_spin_unlock(&mut (*head).lock);
        return node;
    }
    raw_res_spin_unlock(&mut (*head).lock);
    node
}

pub unsafe fn __pcpu_freelist_pop(s: *mut pcpu_freelist) -> *mut pcpu_freelist_node {
    ___pcpu_freelist_pop(s)
}

pub unsafe fn pcpu_freelist_pop(s: *mut pcpu_freelist) -> *mut pcpu_freelist_node {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    let ret = __pcpu_freelist_pop(s);
    local_irq_restore(flags);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
