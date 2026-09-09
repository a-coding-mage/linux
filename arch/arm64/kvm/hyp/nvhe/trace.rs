// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 Google LLC
 * Author: Vincent Donnefort <vdonnefort@google.com>
 */

// Dependencies supplied by the surrounding nvhe/asm implementation are intentionally external.

use core::ffi::c_void;

extern "C" {
    static mut __simple_rbs: simple_rb_per_cpu;
    static mut hyp_nr_cpus: i32;

    fn this_cpu_ptr<T>(ptr: *mut T) -> *mut T;
    fn per_cpu_ptr<T>(ptr: *mut T, cpu: i32) -> *mut T;
    fn simple_ring_buffer_reserve(rb: *mut simple_rb_per_cpu, length: usize, clock: u64) -> *mut c_void;
    fn simple_ring_buffer_commit(rb: *mut simple_rb_per_cpu);
    fn trace_hyp_clock() -> u64;
    fn is_protected_kvm_enabled() -> bool;
    fn __pkvm_host_donate_hyp(pfn: u64, pages: u64) -> i32;
    fn __pkvm_hyp_donate_host(pfn: u64, pages: u64) -> i32;
    fn hyp_virt_to_pfn(addr: *mut c_void) -> u64;
    fn kern_hyp_va(addr: *mut c_void) -> *mut c_void;
    fn hyp_pin_shared_mem(start: *mut c_void, end: *mut c_void) -> i32;
    fn hyp_unpin_shared_mem(start: *mut c_void, end: *mut c_void);
    fn simple_ring_buffer_unload_mm(rb: *mut simple_rb_per_cpu, unpin: unsafe extern "C" fn(*mut c_void));
    fn simple_ring_buffer_init_mm(
        rb: *mut simple_rb_per_cpu,
        bpages: *mut simple_buffer_page,
        desc: *mut ring_buffer_desc,
        pin: unsafe extern "C" fn(usize) -> *mut c_void,
        unpin: unsafe extern "C" fn(*mut c_void),
    ) -> i32;
    fn hyp_assert_lock_held(lock: *mut hyp_spinlock_t);
    fn hyp_spin_lock(lock: *mut hyp_spinlock_t);
    fn hyp_spin_unlock(lock: *mut hyp_spinlock_t);
    fn memset(dst: *mut c_void, value: i32, size: usize) -> *mut c_void;
    fn warn_on(value: bool);
}

#[repr(C)]
pub struct simple_rb_per_cpu { _private: [u8; 0] }
#[repr(C)]
pub struct simple_buffer_page { _private: [u8; 0] }
#[repr(C)]
pub struct hyp_spinlock_t { _private: [u8; 0] }
#[repr(C)]
pub struct ring_buffer_desc {
    pub cpu: u32,
    pub nr_page_va: usize,
    pub page_va: [usize; 0],
}
#[repr(C)]
pub struct hyp_trace_desc {
    pub bpages_backing_start: usize,
    pub bpages_backing_size: usize,
    pub trace_buffer_desc: ring_buffer_desc,
}

const PAGE_SHIFT: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const EINVAL: i32 = 22;

#[repr(C)]
struct hyp_trace_buffer {
    simple_rbs: *mut simple_rb_per_cpu,
    bpages_backing_start: *mut c_void,
    bpages_backing_size: usize,
    lock: hyp_spinlock_t,
}

static mut trace_buffer: hyp_trace_buffer = hyp_trace_buffer {
    simple_rbs: unsafe { &raw mut __simple_rbs },
    bpages_backing_start: core::ptr::null_mut(),
    bpages_backing_size: 0,
    lock: hyp_spinlock_t { _private: [] },
};

unsafe fn hyp_trace_buffer_loaded(trace_buffer: *mut hyp_trace_buffer) -> bool {
    (*trace_buffer).bpages_backing_size > 0
}

pub unsafe fn tracing_reserve_entry(length: usize) -> *mut c_void {
    simple_ring_buffer_reserve(this_cpu_ptr(trace_buffer.simple_rbs), length, trace_hyp_clock())
}

pub unsafe fn tracing_commit_entry() {
    simple_ring_buffer_commit(this_cpu_ptr(trace_buffer.simple_rbs));
}

unsafe fn __admit_host_mem(start: *mut c_void, size: u64) -> i32 {
    if (start as usize) % PAGE_SIZE != 0 || (size as usize) % PAGE_SIZE != 0 || size == 0 {
        return -EINVAL;
    }
    if !is_protected_kvm_enabled() { return 0; }
    __pkvm_host_donate_hyp(hyp_virt_to_pfn(start), size >> PAGE_SHIFT)
}

unsafe fn __release_host_mem(start: *mut c_void, size: u64) {
    if !is_protected_kvm_enabled() { return; }
    warn_on(__pkvm_hyp_donate_host(hyp_virt_to_pfn(start), size >> PAGE_SHIFT) != 0);
}

unsafe fn hyp_trace_buffer_load_bpage_backing(tb: *mut hyp_trace_buffer, desc: *mut hyp_trace_desc) -> i32 {
    let start = kern_hyp_va((*desc).bpages_backing_start as *mut c_void);
    let size = (*desc).bpages_backing_size;
    let ret = __admit_host_mem(start, size as u64);
    if ret != 0 { return ret; }
    memset(start, 0, size);
    (*tb).bpages_backing_start = start;
    (*tb).bpages_backing_size = size;
    0
}

unsafe fn hyp_trace_buffer_unload_bpage_backing(tb: *mut hyp_trace_buffer) {
    let start = (*tb).bpages_backing_start;
    let size = (*tb).bpages_backing_size;
    if size == 0 { return; }
    memset(start, 0, size);
    __release_host_mem(start, size as u64);
    (*tb).bpages_backing_start = core::ptr::null_mut();
    (*tb).bpages_backing_size = 0;
}

unsafe extern "C" fn __pin_shared_page(kern_va: usize) -> *mut c_void {
    let va = kern_hyp_va(kern_va as *mut c_void);
    if !is_protected_kvm_enabled() { return va; }
    if hyp_pin_shared_mem(va, va.add(PAGE_SIZE)) != 0 { core::ptr::null_mut() } else { va }
}

unsafe extern "C" fn __unpin_shared_page(va: *mut c_void) {
    if is_protected_kvm_enabled() { hyp_unpin_shared_mem(va, va.add(PAGE_SIZE)); }
}

unsafe fn hyp_trace_buffer_unload(tb: *mut hyp_trace_buffer) {
    hyp_assert_lock_held(&mut (*tb).lock);
    if !hyp_trace_buffer_loaded(tb) { return; }
    for cpu in 0..hyp_nr_cpus {
        simple_ring_buffer_unload_mm(per_cpu_ptr((*tb).simple_rbs, cpu), __unpin_shared_page);
    }
    hyp_trace_buffer_unload_bpage_backing(tb);
}

unsafe fn hyp_trace_buffer_load(tb: *mut hyp_trace_buffer, desc: *mut hyp_trace_desc) -> i32 {
    hyp_assert_lock_held(&mut (*tb).lock);
    if hyp_trace_buffer_loaded(tb) { return -EINVAL; }
    let ret = hyp_trace_buffer_load_bpage_backing(tb, desc);
    if ret != 0 { return ret; }
    let mut bpages = (*tb).bpages_backing_start as *mut simple_buffer_page;
    let mut ret = 0;
    for cpu in 0..hyp_nr_cpus {
        let rb_desc = &mut (*desc).trace_buffer_desc;
        ret = simple_ring_buffer_init_mm(per_cpu_ptr((*tb).simple_rbs, cpu), bpages, rb_desc, __pin_shared_page, __unpin_shared_page);
        if ret != 0 { break; }
        bpages = bpages.add(rb_desc.nr_page_va);
    }
    if ret != 0 { hyp_trace_buffer_unload(tb); }
    ret
}

pub unsafe fn __tracing_load(desc_hva: usize, desc_size: usize) -> i32 {
    let desc = kern_hyp_va(desc_hva as *mut c_void) as *mut hyp_trace_desc;
    let mut ret = __admit_host_mem(desc as *mut c_void, desc_size as u64);
    if ret == 0 {
        hyp_spin_lock(&mut trace_buffer.lock);
        ret = hyp_trace_buffer_load(&mut trace_buffer, desc);
        hyp_spin_unlock(&mut trace_buffer.lock);
    }
    __release_host_mem(desc as *mut c_void, desc_size as u64);
    ret
}

pub unsafe fn __tracing_unload() {
    hyp_spin_lock(&mut trace_buffer.lock);
    hyp_trace_buffer_unload(&mut trace_buffer);
    hyp_spin_unlock(&mut trace_buffer.lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
