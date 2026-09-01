/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Variant of atomic_t specialized for reference counts.
 *
 * The interface matches the atomic_t interface (to aid in porting) but only
 * provides the few functions one should use for reference counting.
 *
 * It differs in that the counter saturates at UINT_MAX and will not move once
 * there. This avoids wrapping the counter and causing 'spurious'
 * use-after-free issues.
 *
 * Memory ordering rules are slightly relaxed wrt regular atomic_t functions
 * and provide only what is strictly required for refcounts.
 *
 * The increments are fully relaxed; these will not provide ordering. The
 * rationale is that whatever is used to obtain the object we're increasing the
 * reference count on will provide the ordering. For locked data structures,
 * its the lock acquire, for RCU/lockless data structures its the dependent
 * load.
 *
 * Do note that inc_not_zero() provides a control dependency which will order
 * future stores against the inc, this ensures we'll never modify the object
 * if we did not in fact acquire a reference.
 *
 * The decrements will provide release order, such that all the prior loads and
 * stores will be issued before, it also provides a control dependency, which
 * will order us against the subsequent free().
 *
 * The control dependency is against the load of the cmpxchg (ll/sc) that
 * succeeded. This means the stores aren't fully ordered, but this is fine
 * because the 1->0 transition indicates no concurrency.
 *
 * Note that the allocator is responsible for ordering things between free()
 * and alloc().
 *
 */

/* Dependencies in the original C header:
 * #include <linux/atomic.h>
 * #include <linux/kernel.h>
 */

pub const UINT_MAX: u32 = u32::MAX;

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn atomic_set(v: *mut atomic_t, i: u32);
    pub fn atomic_read(v: *const atomic_t) -> u32;
    pub fn atomic_cmpxchg_relaxed(v: *mut atomic_t, old: u32, new: u32) -> u32;
    pub fn atomic_cmpxchg_release(v: *mut atomic_t, old: u32, new: u32) -> u32;
    pub fn ATOMIC_INIT(n: u32) -> atomic_t;
    pub fn BUG_ON(condition: bool);
    pub fn unlikely(condition: bool) -> bool;
}

/*
 * Original conditional macros:
 *
 * #ifdef NDEBUG
 * #define REFCOUNT_WARN(cond, str) (void)(cond)
 * #define __refcount_check
 * #else
 * #define REFCOUNT_WARN(cond, str) BUG_ON(cond)
 * #define __refcount_check __must_check
 * #endif
 *
 * This translation uses cfg(debug_assertions) as the Rust analogue: in debug
 * builds REFCOUNT_WARN calls BUG_ON, while in non-debug builds it evaluates and
 * discards the condition.
 */
#[inline]
pub unsafe fn REFCOUNT_WARN(condition: bool, _str: *const core::ffi::c_char) {
    #[cfg(debug_assertions)]
    {
        unsafe {
            BUG_ON(condition);
        }
    }

    #[cfg(not(debug_assertions))]
    {
        let _ = condition;
    }
}

#[repr(C)]
pub struct refcount_struct {
    pub refs: atomic_t,
}

pub type refcount_t = refcount_struct;

#[macro_export]
macro_rules! REFCOUNT_INIT {
    ($n:expr) => {
        refcount_t {
            refs: unsafe { ATOMIC_INIT($n) },
        }
    };
}

#[inline]
pub unsafe fn refcount_set(r: *mut refcount_t, n: u32) {
    unsafe {
        atomic_set(&mut (*r).refs, n);
    }
}

#[inline]
pub unsafe fn refcount_set_release(r: *mut refcount_t, n: u32) {
    unsafe {
        atomic_set(&mut (*r).refs, n);
    }
}

#[inline]
pub unsafe fn refcount_read(r: *const refcount_t) -> u32 {
    unsafe { atomic_read(&(*r).refs) }
}

/*
 * Similar to atomic_inc_not_zero(), will saturate at UINT_MAX and WARN.
 *
 * Provides no memory ordering, it is assumed the caller has guaranteed the
 * object memory to be stable (RCU, etc.). It does provide a control dependency
 * and thereby orders future stores. See the comment on top.
 */
#[inline]
#[must_use]
pub unsafe fn refcount_inc_not_zero(r: *mut refcount_t) -> bool {
    let mut old: u32;
    let mut new: u32;
    let mut val: u32 = unsafe { atomic_read(&(*r).refs) };

    loop {
        new = val.wrapping_add(1);

        if val == 0 {
            return false;
        }

        if unsafe { unlikely(new == 0) } {
            return true;
        }

        old = unsafe { atomic_cmpxchg_relaxed(&mut (*r).refs, val, new) };
        if old == val {
            break;
        }

        val = old;
    }

    unsafe {
        REFCOUNT_WARN(
            new == UINT_MAX,
            c"refcount_t: saturated; leaking memory.\n".as_ptr(),
        );
    }

    true
}

/*
 * Similar to atomic_inc(), will saturate at UINT_MAX and WARN.
 *
 * Provides no memory ordering, it is assumed the caller already has a
 * reference on the object, will WARN when this is not so.
 */
#[inline]
pub unsafe fn refcount_inc(r: *mut refcount_t) {
    unsafe {
        REFCOUNT_WARN(
            !refcount_inc_not_zero(r),
            c"refcount_t: increment on 0; use-after-free.\n".as_ptr(),
        );
    }
}

/*
 * Similar to atomic_dec_and_test(), it will WARN on underflow and fail to
 * decrement when saturated at UINT_MAX.
 *
 * Provides release memory ordering, such that prior loads and stores are done
 * before, and provides a control dependency such that free() must come after.
 * See the comment on top.
 */
#[inline]
#[must_use]
pub unsafe fn refcount_sub_and_test(i: u32, r: *mut refcount_t) -> bool {
    let mut old: u32;
    let mut new: u32;
    let mut val: u32 = unsafe { atomic_read(&(*r).refs) };

    loop {
        if unsafe { unlikely(val == UINT_MAX) } {
            return false;
        }

        new = val.wrapping_sub(i);
        if new > val {
            unsafe {
                REFCOUNT_WARN(
                    new > val,
                    c"refcount_t: underflow; use-after-free.\n".as_ptr(),
                );
            }
            return false;
        }

        old = unsafe { atomic_cmpxchg_release(&mut (*r).refs, val, new) };
        if old == val {
            break;
        }

        val = old;
    }

    new == 0
}

#[inline]
#[must_use]
pub unsafe fn refcount_dec_and_test(r: *mut refcount_t) -> bool {
    unsafe { refcount_sub_and_test(1, r) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
