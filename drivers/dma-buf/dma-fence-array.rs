// SPDX-License-Identifier: GPL-2.0-only
/*
 * dma-fence-array: aggregate fences to be waited together
 *
 * Copyright (C) 2016 Collabora Ltd
 * Copyright (C) 2016 Advanced Micro Devices, Inc.
 * Authors:
 *	Gustavo Padovan <gustavo@padovan.org>
 *	Christian König <christian.koenig@amd.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const PENDING_ERROR: i32 = 1;

unsafe extern "C" {
    fn dma_fence_get_driver_name(fence: *mut dma_fence) -> *const core::ffi::c_char;
    fn dma_fence_array_clear_pending_error(array: *mut dma_fence_array);
}

#[repr(C)]
pub struct dma_fence {
    pub context: u64,
    pub error: i32,
    pub flags: usize,
    pub inline_lock: lock,
}

#[repr(C)]
pub struct dma_fence_cb {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct dma_fence_array_cb {
    pub cb: dma_fence_cb,
    pub array: *mut dma_fence_array,
}

#[repr(C)]
pub struct dma_fence_array {
    pub base: dma_fence,
    pub work: irq_work,
    pub num_fences: u32,
    pub num_pending: atomic_t,
    pub fences: *mut *mut dma_fence,
    pub callbacks: *mut dma_fence_array_cb,
}

#[repr(C)]
pub struct dma_fence_ops {
    pub get_driver_name: Option<unsafe extern "C" fn(*mut dma_fence) -> *const core::ffi::c_char>,
    pub get_timeline_name: Option<unsafe extern "C" fn(*mut dma_fence) -> *const core::ffi::c_char>,
    pub enable_signaling: Option<unsafe extern "C" fn(*mut dma_fence) -> bool>,
    pub signaled: Option<unsafe extern "C" fn(*mut dma_fence) -> bool>,
    pub release: Option<unsafe extern "C" fn(*mut dma_fence)>,
    pub set_deadline: Option<unsafe extern "C" fn(*mut dma_fence, ktime_t)>,
}

#[repr(C)] pub struct irq_work { _opaque: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct lock { _opaque: [u8; 0] }
#[repr(C)] pub struct lock_class_key { _opaque: [u8; 0] }
pub type ktime_t = i64;

unsafe extern "C" {
    fn cmpxchg(ptr: *mut i32, old: i32, new: i32) -> i32;
    fn atomic_dec_and_test(v: *mut atomic_t) -> bool;
    fn atomic_read_acquire(v: *const atomic_t) -> i32;
    fn atomic_set(v: *mut atomic_t, i: i32);
    fn irq_work_queue(work: *mut irq_work);
    fn init_irq_work(work: *mut irq_work, func: unsafe extern "C" fn(*mut irq_work));
    fn dma_fence_signal(fence: *mut dma_fence);
    fn dma_fence_put(fence: *mut dma_fence);
    fn dma_fence_get(fence: *mut dma_fence);
    fn dma_fence_add_callback(fence: *mut dma_fence, cb: *mut dma_fence_cb, func: unsafe extern "C" fn(*mut dma_fence, *mut dma_fence_cb)) -> i32;
    fn dma_fence_is_signaled(fence: *mut dma_fence) -> bool;
    fn dma_fence_is_container(fence: *mut dma_fence) -> bool;
    fn dma_fence_set_deadline(fence: *mut dma_fence, deadline: ktime_t);
    fn dma_fence_init(fence: *mut dma_fence, ops: *const dma_fence_ops, lock: *mut core::ffi::c_void, context: u64, seqno: u32);
    fn dma_fence_free(fence: *mut dma_fence);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn lockdep_set_class(lock: *mut lock, key: *mut lock_class_key);
    fn warn_on(condition: bool) -> bool;
}

unsafe fn dma_fence_array_get_driver_name(_: *mut dma_fence) -> *const core::ffi::c_char { b"dma_fence_array\0".as_ptr() as _ }
unsafe fn dma_fence_array_get_timeline_name(_: *mut dma_fence) -> *const core::ffi::c_char { b"unbound\0".as_ptr() as _ }

unsafe fn dma_fence_array_set_pending_error(array: *mut dma_fence_array, error: i32) {
    if error != 0 { cmpxchg(&mut (*array).base.error, PENDING_ERROR, error); }
}

unsafe fn dma_fence_array_clear_pending_error(array: *mut dma_fence_array) {
    cmpxchg(&mut (*array).base.error, PENDING_ERROR, 0);
}

unsafe extern "C" fn irq_dma_fence_array_work(wrk: *mut irq_work) {
    let array = (wrk as *mut u8).sub(core::mem::offset_of!(dma_fence_array, work)) as *mut dma_fence_array;
    dma_fence_array_clear_pending_error(array);
    dma_fence_signal(&mut (*array).base);
    dma_fence_put(&mut (*array).base);
}

unsafe extern "C" fn dma_fence_array_cb_func(f: *mut dma_fence, cb: *mut dma_fence_cb) {
    let array_cb = cb as *mut dma_fence_array_cb;
    let array = (*array_cb).array;
    dma_fence_array_set_pending_error(array, (*f).error);
    if atomic_dec_and_test(&mut (*array).num_pending) { irq_work_queue(&mut (*array).work); }
    else { dma_fence_put(&mut (*array).base); }
}

unsafe extern "C" fn dma_fence_array_enable_signaling(fence: *mut dma_fence) -> bool {
    let array = fence as *mut dma_fence_array;
    for i in 0..(*array).num_fences {
        let cb = (*array).callbacks.add(i as usize);
        (*cb).array = array;
        dma_fence_get(&mut (*array).base);
        if dma_fence_add_callback(*(*array).fences.add(i as usize), &mut (*cb).cb, dma_fence_array_cb_func) != 0 {
            dma_fence_array_set_pending_error(array, (*(*array).fences.add(i as usize)).error);
            dma_fence_put(&mut (*array).base);
            if atomic_dec_and_test(&mut (*array).num_pending) {
                dma_fence_array_clear_pending_error(array);
                return false;
            }
        }
    }
    true
}

unsafe extern "C" fn dma_fence_array_signaled(fence: *mut dma_fence) -> bool {
    let array = fence as *mut dma_fence_array;
    let mut num_pending = atomic_read_acquire(&(*array).num_pending);
    if ((*array).base.flags & (1usize << 1)) != 0 {
        if num_pending <= 0 { dma_fence_array_clear_pending_error(array); return true; }
        return false;
    }
    for i in 0..(*array).num_fences {
        if dma_fence_is_signaled(*(*array).fences.add(i as usize)) { num_pending -= 1; if num_pending == 0 { dma_fence_array_clear_pending_error(array); return true; } }
    }
    false
}

unsafe extern "C" fn dma_fence_array_release(fence: *mut dma_fence) {
    let array = fence as *mut dma_fence_array;
    for i in 0..(*array).num_fences { dma_fence_put(*(*array).fences.add(i as usize)); }
    kfree((*array).fences as _);
    dma_fence_free(fence);
}

unsafe extern "C" fn dma_fence_array_set_deadline(fence: *mut dma_fence, deadline: ktime_t) {
    let array = fence as *mut dma_fence_array;
    for i in 0..(*array).num_fences { dma_fence_set_deadline(*(*array).fences.add(i as usize), deadline); }
}

#[no_mangle]
pub static dma_fence_array_ops: dma_fence_ops = dma_fence_ops { get_driver_name: Some(dma_fence_array_get_driver_name), get_timeline_name: Some(dma_fence_array_get_timeline_name), enable_signaling: Some(dma_fence_array_enable_signaling), signaled: Some(dma_fence_array_signaled), release: Some(dma_fence_array_release), set_deadline: Some(dma_fence_array_set_deadline) };

#[no_mangle]
pub unsafe extern "C" fn dma_fence_array_alloc(num_fences: i32) -> *mut dma_fence_array { extern "C" { fn kzalloc_flex(size: usize) -> *mut dma_fence_array; } kzalloc_flex(num_fences as usize) }

#[no_mangle]
pub unsafe extern "C" fn dma_fence_array_init(array: *mut dma_fence_array, num_fences: i32, fences: *mut *mut dma_fence, context: u64, seqno: u32) {
    static mut DMA_FENCE_ARRAY_LOCK_KEY: lock_class_key = lock_class_key { _opaque: [] };
    warn_on(num_fences == 0 || fences.is_null());
    (*array).num_fences = num_fences as u32;
    dma_fence_init(&mut (*array).base, &dma_fence_array_ops, core::ptr::null_mut(), context, seqno);
    init_irq_work(&mut (*array).work, irq_dma_fence_array_work);
    lockdep_set_class(&mut (*array).base.inline_lock, &raw mut DMA_FENCE_ARRAY_LOCK_KEY);
    atomic_set(&mut (*array).num_pending, num_fences);
    (*array).fences = fences;
    (*array).base.error = PENDING_ERROR;
    let mut n = num_fences;
    while n != 0 { n -= 1; warn_on(dma_fence_is_container(*fences.add(n as usize))); }
}

#[no_mangle]
pub unsafe extern "C" fn dma_fence_array_create(num_fences: i32, fences: *mut *mut dma_fence, context: u64, seqno: u32) -> *mut dma_fence_array {
    let array = dma_fence_array_alloc(num_fences); if array.is_null() { return core::ptr::null_mut(); }
    dma_fence_array_init(array, num_fences, fences, context, seqno); array
}

#[no_mangle]
pub unsafe extern "C" fn dma_fence_match_context(fence: *mut dma_fence, context: u64) -> bool {
    let array = fence as *mut dma_fence_array;
    if (*array).num_fences == 0 { return (*fence).context == context; }
    for i in 0..(*array).num_fences { if (*(*array).fences.add(i as usize)).context != context { return false; } }
    true
}

#[no_mangle]
pub unsafe extern "C" fn dma_fence_array_first(head: *mut dma_fence) -> *mut dma_fence {
    if head.is_null() { return core::ptr::null_mut(); }
    let array = head as *mut dma_fence_array; if (*array).num_fences == 0 { return core::ptr::null_mut(); } (*array).fences.read()
}

#[no_mangle]
pub unsafe extern "C" fn dma_fence_array_next(head: *mut dma_fence, index: u32) -> *mut dma_fence {
    if head.is_null() { return core::ptr::null_mut(); }
    let array = head as *mut dma_fence_array; if index >= (*array).num_fences { return core::ptr::null_mut(); } *(*array).fences.add(index as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
