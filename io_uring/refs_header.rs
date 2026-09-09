/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: <linux/atomic.h>
// Dependency intent: <linux/io_uring_types.h>

/*
 * Shamelessly stolen from the mm implementation of page reference checking,
 * see commit f958d7b528b1 for details.
 */

use core::sync::atomic::{AtomicI32, Ordering};

// Rust representation of the externally supplied C type used by this header.
#[repr(C)]
pub struct io_kiocb {
    pub flags: u32,
    pub refs: AtomicI32,
}

pub const REQ_F_REFCOUNT: u32 = 1 << 0;

#[inline]
pub unsafe fn req_ref_zero_or_close_to_overflow(req: *const io_kiocb) -> bool {
    ((*(*req).refs.get_mut()) as u32).wrapping_add(127u32) <= 127u32
}

#[inline]
pub unsafe fn req_ref_inc_not_zero(req: *mut io_kiocb) -> bool {
    WARN_ON_ONCE((*req).flags & REQ_F_REFCOUNT == 0);
    atomic_inc_not_zero(&(*req).refs)
}

#[inline]
pub unsafe fn req_ref_put_and_test_atomic(req: *mut io_kiocb) -> bool {
    WARN_ON_ONCE(data_race((*req).flags) & REQ_F_REFCOUNT == 0);
    WARN_ON_ONCE(req_ref_zero_or_close_to_overflow(req));
    atomic_dec_and_test(&(*req).refs)
}

#[inline]
pub unsafe fn req_ref_put_and_test(req: *mut io_kiocb) -> bool {
    if likely((*req).flags & REQ_F_REFCOUNT == 0) {
        return true;
    }

    WARN_ON_ONCE(req_ref_zero_or_close_to_overflow(req));
    atomic_dec_and_test(&(*req).refs)
}

#[inline]
pub unsafe fn req_ref_get(req: *mut io_kiocb) {
    WARN_ON_ONCE((*req).flags & REQ_F_REFCOUNT == 0);
    WARN_ON_ONCE(req_ref_zero_or_close_to_overflow(req));
    atomic_inc(&(*req).refs);
}

#[inline]
pub unsafe fn req_ref_put(req: *mut io_kiocb) {
    WARN_ON_ONCE((*req).flags & REQ_F_REFCOUNT == 0);
    WARN_ON_ONCE(req_ref_zero_or_close_to_overflow(req));
    atomic_dec(&(*req).refs);
}

#[inline]
pub unsafe fn __io_req_set_refcount(req: *mut io_kiocb, nr: i32) {
    if (*req).flags & REQ_F_REFCOUNT == 0 {
        (*req).flags |= REQ_F_REFCOUNT;
        atomic_set(&(*req).refs, nr);
    }
}

#[inline]
pub unsafe fn io_req_set_refcount(req: *mut io_kiocb) {
    __io_req_set_refcount(req, 1);
}

#[inline]
unsafe fn atomic_inc_not_zero(value: &AtomicI32) -> bool {
    let mut current = value.load(Ordering::Relaxed);
    loop {
        if current == 0 {
            return false;
        }
        match value.compare_exchange_weak(current, current.wrapping_add(1), Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return true,
            Err(actual) => current = actual,
        }
    }
}

#[inline]
unsafe fn atomic_dec_and_test(value: &AtomicI32) -> bool {
    value.fetch_sub(1, Ordering::Relaxed).wrapping_sub(1) == 0
}

#[inline]
unsafe fn atomic_inc(value: &AtomicI32) {
    value.fetch_add(1, Ordering::Relaxed);
}

#[inline]
unsafe fn atomic_dec(value: &AtomicI32) {
    value.fetch_sub(1, Ordering::Relaxed);
}

#[inline]
unsafe fn atomic_set(value: &AtomicI32, nr: i32) {
    value.store(nr, Ordering::Relaxed);
}

#[inline]
fn likely(value: bool) -> bool {
    value
}

#[inline]
fn data_race<T>(value: T) -> T {
    value
}

#[inline]
fn WARN_ON_ONCE(_condition: bool) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
