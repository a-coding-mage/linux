// SPDX-License-Identifier: GPL-2.0
/*
 * Out-of-line refcount functions.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/mutex.h, linux/refcount.h, linux/spinlock.h, linux/bug.h

extern "C" {
    fn refcount_set(r: *mut refcount_t, value: i32);
    fn atomic_try_cmpxchg_release(refs: *mut atomic_t, old: *mut i32, new: i32) -> bool;
    fn atomic_read(refs: *const atomic_t) -> u32;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: unsigned_long);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: unsigned_long);
    fn warn_once(condition: bool, message: *const u8) -> bool;
}

// External types and constants supplied by the surrounding kernel translation unit.
#[allow(non_camel_case_types)]
type unsigned_long = usize;

#[allow(non_camel_case_types)]
enum refcount_t {}
#[allow(non_camel_case_types)]
enum atomic_t {}
#[allow(non_camel_case_types)]
enum mutex {}
#[allow(non_camel_case_types)]
enum spinlock_t {}

const REFCOUNT_SATURATED: u32 = 0x4000_0000;

#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum refcount_saturation_type {
    REFCOUNT_ADD_NOT_ZERO_OVF,
    REFCOUNT_ADD_OVF,
    REFCOUNT_ADD_UAF,
    REFCOUNT_SUB_UAF,
    REFCOUNT_DEC_LEAK,
}

#[inline]
pub unsafe fn refcount_warn_saturate(r: *mut refcount_t, t: refcount_saturation_type) {
    refcount_set(r, REFCOUNT_SATURATED as i32);

    match t {
        refcount_saturation_type::REFCOUNT_ADD_NOT_ZERO_OVF => {
            warn_once(true, b"refcount_t: saturated; leaking memory.\0".as_ptr());
        }
        refcount_saturation_type::REFCOUNT_ADD_OVF => {
            warn_once(true, b"refcount_t: saturated; leaking memory.\0".as_ptr());
        }
        refcount_saturation_type::REFCOUNT_ADD_UAF => {
            warn_once(true, b"refcount_t: addition on 0; use-after-free.\0".as_ptr());
        }
        refcount_saturation_type::REFCOUNT_SUB_UAF => {
            warn_once(true, b"refcount_t: underflow; use-after-free.\0".as_ptr());
        }
        refcount_saturation_type::REFCOUNT_DEC_LEAK => {
            warn_once(true, b"refcount_t: decrement hit 0; leaking memory.\0".as_ptr());
        }
        _ => {
            warn_once(true, b"refcount_t: unknown saturation event!?.\0".as_ptr());
        }
    }
}

#[inline]
pub unsafe fn refcount_dec_if_one(r: *mut refcount_t) -> bool {
    let mut val: i32 = 1;
    atomic_try_cmpxchg_release(r as *mut atomic_t, &mut val, 0)
}

#[inline]
pub unsafe fn refcount_dec_not_one(r: *mut refcount_t) -> bool {
    let mut val: u32 = atomic_read(r as *const atomic_t);

    loop {
        if val == REFCOUNT_SATURATED {
            return true;
        }

        if val == 1 {
            return false;
        }

        let new = val - 1;
        if new > val {
            warn_once(true, b"refcount_t: underflow; use-after-free.\0".as_ptr());
            return true;
        }

        let mut old = val as i32;
        if atomic_try_cmpxchg_release(r as *mut atomic_t, &mut old, new as i32) {
            return true;
        }
        val = old as u32;
    }
}

#[inline]
pub unsafe fn refcount_dec_and_mutex_lock(r: *mut refcount_t, lock: *mut mutex) -> bool {
    if refcount_dec_not_one(r) {
        return false;
    }

    mutex_lock(lock);
    if !refcount_dec_and_test(r) {
        mutex_unlock(lock);
        return false;
    }

    true
}

#[inline]
pub unsafe fn refcount_dec_and_lock(r: *mut refcount_t, lock: *mut spinlock_t) -> bool {
    if refcount_dec_not_one(r) {
        return false;
    }

    spin_lock(lock);
    if !refcount_dec_and_test(r) {
        spin_unlock(lock);
        return false;
    }

    true
}

#[inline]
pub unsafe fn refcount_dec_and_lock_irqsave(
    r: *mut refcount_t,
    lock: *mut spinlock_t,
    flags: *mut unsigned_long,
) -> bool {
    if refcount_dec_not_one(r) {
        return false;
    }

    spin_lock_irqsave(lock, *flags);
    if !refcount_dec_and_test(r) {
        spin_unlock_irqrestore(lock, *flags);
        return false;
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
