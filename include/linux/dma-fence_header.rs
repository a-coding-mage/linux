/* SPDX-License-Identifier: GPL-2.0-only */
/* Fence mechanism for dma-buf to allow asynchronous dma access. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented in this header translation.

#[repr(C)]
pub union DmaFenceLock {
    pub extern_lock: *mut spinlock_t,
    pub inline_lock: spinlock_t,
}

#[repr(C)]
pub union DmaFenceCallbackList {
    pub cb_list: list_head,
    pub timestamp: ktime_t,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct dma_fence {
    pub lock: DmaFenceLock,
    pub ops: *const dma_fence_ops,
    pub cb_list: DmaFenceCallbackList,
    pub context: u64,
    pub seqno: u64,
    pub flags: c_ulong,
    pub refcount: kref,
    pub error: c_int,
}

#[repr(C)]
pub struct dma_fence_cb {
    pub node: list_head,
    pub func: Option<dma_fence_func_t>,
}

pub type dma_fence_func_t = unsafe extern "C" fn(
    fence: *mut dma_fence,
    cb: *mut dma_fence_cb,
);

#[repr(C)]
pub struct dma_fence_ops {
    pub get_driver_name: Option<unsafe extern "C" fn(*mut dma_fence) -> *const c_char>,
    pub get_timeline_name: Option<unsafe extern "C" fn(*mut dma_fence) -> *const c_char>,
    pub enable_signaling: Option<unsafe extern "C" fn(*mut dma_fence) -> bool>,
    pub signaled: Option<unsafe extern "C" fn(*mut dma_fence) -> bool>,
    pub wait: Option<unsafe extern "C" fn(*mut dma_fence, bool, c_long) -> c_long>,
    pub release: Option<unsafe extern "C" fn(*mut dma_fence)>,
    pub set_deadline: Option<unsafe extern "C" fn(*mut dma_fence, ktime_t)>,
}

pub const DMA_FENCE_FLAG_INITIALIZED_BIT: u32 = 0;
pub const DMA_FENCE_FLAG_INLINE_LOCK_BIT: u32 = 1;
pub const DMA_FENCE_FLAG_SEQNO64_BIT: u32 = 2;
pub const DMA_FENCE_FLAG_SIGNALED_BIT: u32 = 3;
pub const DMA_FENCE_FLAG_TIMESTAMP_BIT: u32 = 4;
pub const DMA_FENCE_FLAG_ENABLE_SIGNAL_BIT: u32 = 5;
pub const DMA_FENCE_FLAG_USER_BITS: u32 = 6;

extern "C" {
    pub fn dma_fence_init(fence: *mut dma_fence, ops: *const dma_fence_ops,
                          lock: *mut spinlock_t, context: u64, seqno: u64);
    pub fn dma_fence_init64(fence: *mut dma_fence, ops: *const dma_fence_ops,
                            lock: *mut spinlock_t, context: u64, seqno: u64);
    pub fn dma_fence_release(kref: *mut kref);
    pub fn dma_fence_free(fence: *mut dma_fence);
    pub fn dma_fence_describe(fence: *mut dma_fence, seq: *mut seq_file);
    pub fn dma_fence_signal(fence: *mut dma_fence);
    pub fn dma_fence_check_and_signal(fence: *mut dma_fence) -> bool;
    pub fn dma_fence_check_and_signal_locked(fence: *mut dma_fence) -> bool;
    pub fn dma_fence_signal_locked(fence: *mut dma_fence);
    pub fn dma_fence_signal_timestamp(fence: *mut dma_fence, timestamp: ktime_t);
    pub fn dma_fence_signal_timestamp_locked(fence: *mut dma_fence, timestamp: ktime_t);
    pub fn dma_fence_default_wait(fence: *mut dma_fence, intr: bool, timeout: c_long) -> c_long;
    pub fn dma_fence_add_callback(fence: *mut dma_fence, cb: *mut dma_fence_cb,
                                  func: dma_fence_func_t) -> c_int;
    pub fn dma_fence_remove_callback(fence: *mut dma_fence, cb: *mut dma_fence_cb) -> bool;
    pub fn dma_fence_enable_signaling(fence: *mut dma_fence);
    pub fn dma_fence_driver_name(fence: *mut dma_fence) -> *const c_char;
    pub fn dma_fence_timeline_name(fence: *mut dma_fence) -> *const c_char;
    pub fn dma_fence_get_status(fence: *mut dma_fence) -> c_int;
    pub fn dma_fence_wait_timeout(fence: *mut dma_fence, intr: bool, timeout: c_long) -> c_long;
    pub fn dma_fence_wait_any_timeout(fences: *mut *mut dma_fence, count: u32, intr: bool,
                                      timeout: c_long, idx: *mut u32) -> c_long;
    pub fn dma_fence_set_deadline(fence: *mut dma_fence, deadline: ktime_t);
    pub fn dma_fence_get_stub() -> *mut dma_fence;
    pub fn dma_fence_allocate_private_stub(timestamp: ktime_t) -> *mut dma_fence;
    pub fn dma_fence_context_alloc(num: c_uint) -> u64;
}

#[inline]
pub unsafe fn dma_fence_was_initialized(fence: *mut dma_fence) -> bool {
    !fence.is_null() && test_bit(DMA_FENCE_FLAG_INITIALIZED_BIT, &(*fence).flags)
}

#[inline]
pub unsafe fn dma_fence_put(fence: *mut dma_fence) {
    if !fence.is_null() { kref_put(&mut (*fence).refcount, dma_fence_release); }
}

#[inline]
pub unsafe fn dma_fence_get(fence: *mut dma_fence) -> *mut dma_fence {
    if !fence.is_null() { kref_get(&mut (*fence).refcount); }
    fence
}

#[inline]
pub unsafe fn dma_fence_get_rcu(fence: *mut dma_fence) -> *mut dma_fence {
    if kref_get_unless_zero(&mut (*fence).refcount) { fence } else { core::ptr::null_mut() }
}

#[inline]
pub unsafe fn dma_fence_spinlock(fence: *mut dma_fence) -> *mut spinlock_t {
    if test_bit(DMA_FENCE_FLAG_INLINE_LOCK_BIT, &(*fence).flags) {
        &mut (*fence).lock.inline_lock
    } else {
        (*fence).lock.extern_lock
    }
}

#[inline]
pub unsafe fn dma_fence_test_signaled_flag(fence: *mut dma_fence) -> bool {
    test_bit(DMA_FENCE_FLAG_SIGNALED_BIT, &(*fence).flags)
}

#[inline]
pub unsafe fn dma_fence_is_signaled_locked(fence: *mut dma_fence) -> bool {
    if dma_fence_test_signaled_flag(fence) { return true; }
    rcu_read_lock();
    let ops = rcu_dereference((*fence).ops);
    let signaled = !ops.is_null() && (*ops).signaled.map_or(false, |f| f(fence));
    rcu_read_unlock();
    if signaled { dma_fence_signal_locked(fence); true } else { false }
}

#[inline]
pub unsafe fn dma_fence_is_signaled(fence: *mut dma_fence) -> bool {
    if dma_fence_test_signaled_flag(fence) { return true; }
    rcu_read_lock();
    let ops = rcu_dereference((*fence).ops);
    let signaled = !ops.is_null() && (*ops).signaled.map_or(false, |f| f(fence));
    rcu_read_unlock();
    if signaled { dma_fence_signal(fence); true } else { false }
}

#[inline]
pub unsafe fn __dma_fence_is_later(fence: *mut dma_fence, f1: u64, f2: u64) -> bool {
    if test_bit(DMA_FENCE_FLAG_SEQNO64_BIT, &(*fence).flags) { return f1 > f2; }
    (lower_32_bits(f1).wrapping_sub(lower_32_bits(f2)) as i32) > 0
}

#[inline]
pub unsafe fn dma_fence_is_later(f1: *mut dma_fence, f2: *mut dma_fence) -> bool {
    if warn_on((*f1).context != (*f2).context) { return false; }
    __dma_fence_is_later(f1, (*f1).seqno, (*f2).seqno)
}

#[inline]
pub unsafe fn dma_fence_is_later_or_same(f1: *mut dma_fence, f2: *mut dma_fence) -> bool {
    f1 == f2 || dma_fence_is_later(f1, f2)
}

#[inline]
pub unsafe fn dma_fence_later(f1: *mut dma_fence, f2: *mut dma_fence) -> *mut dma_fence {
    if warn_on((*f1).context != (*f2).context) { return core::ptr::null_mut(); }
    if dma_fence_is_later(f1, f2) {
        if dma_fence_is_signaled(f1) { core::ptr::null_mut() } else { f1 }
    } else if dma_fence_is_signaled(f2) { core::ptr::null_mut() } else { f2 }
}

#[inline]
pub unsafe fn dma_fence_get_status_locked(fence: *mut dma_fence) -> c_int {
    if dma_fence_is_signaled_locked(fence) { if (*fence).error != 0 { (*fence).error } else { 1 } } else { 0 }
}

#[inline]
pub unsafe fn dma_fence_set_error(fence: *mut dma_fence, error: c_int) {
    warn_on(test_bit(DMA_FENCE_FLAG_SIGNALED_BIT, &(*fence).flags));
    warn_on(error >= 0 || error < -MAX_ERRNO);
    (*fence).error = error;
}

#[inline]
pub unsafe fn dma_fence_wait(fence: *mut dma_fence, intr: bool) -> c_long {
    let ret = dma_fence_wait_timeout(fence, intr, MAX_SCHEDULE_TIMEOUT);
    if ret < 0 { ret } else { 0 }
}

#[inline]
pub unsafe fn dma_fence_is_array(fence: *mut dma_fence) -> bool {
    rcu_access_pointer((*fence).ops) == &dma_fence_array_ops
}

#[inline]
pub unsafe fn dma_fence_is_chain(fence: *mut dma_fence) -> bool {
    rcu_access_pointer((*fence).ops) == &dma_fence_chain_ops
}

#[inline]
pub unsafe fn dma_fence_is_container(fence: *mut dma_fence) -> bool {
    dma_fence_is_array(fence) || dma_fence_is_chain(fence)
}

extern "C" {
    pub static dma_fence_array_ops: dma_fence_ops;
    pub static dma_fence_chain_ops: dma_fence_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
