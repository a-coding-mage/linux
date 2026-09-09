/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * kref.h - library routines for handling generic reference counted objects
 *
 * Copyright (C) 2004 Greg Kroah-Hartman <greg@kroah.com>
 * Copyright (C) 2004 IBM Corp.
 *
 * based on kobject.h which was:
 * Copyright (C) 2002-2003 Patrick Mochel <mochel@osdl.org>
 * Copyright (C) 2002-2003 Open Source Development Labs
 */

/* Dependencies supplied by linux/spinlock.h and linux/refcount.h. */

#[repr(C)]
pub struct kref {
    pub refcount: refcount_t,
}

#[macro_export]
macro_rules! KREF_INIT {
    ($n:expr) => {
        $crate::kref {
            refcount: REFCOUNT_INIT!($n),
        }
    };
}

extern "C" {
    fn refcount_set(r: *mut refcount_t, n: i32);
    fn refcount_read(r: *const refcount_t) -> core::ffi::c_uint;
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    fn refcount_dec_and_mutex_lock(r: *mut refcount_t, mutex: *mut mutex) -> bool;
    fn refcount_dec_and_lock(r: *mut refcount_t, lock: *mut spinlock_t) -> bool;
    fn refcount_inc_not_zero(r: *mut refcount_t) -> i32;
}

/**
 * kref_init - initialize object.
 * @kref: object in question.
 */
#[inline]
pub unsafe fn kref_init(kref: *mut kref) {
    refcount_set(&mut (*kref).refcount, 1);
}

#[inline]
pub unsafe fn kref_read(kref: *const kref) -> core::ffi::c_uint {
    refcount_read(&(*kref).refcount)
}

/**
 * kref_get - increment refcount for object.
 * @kref: object.
 */
#[inline]
pub unsafe fn kref_get(kref: *mut kref) {
    refcount_inc(&mut (*kref).refcount);
}

/**
 * kref_put - Decrement refcount for object
 * @kref: Object
 * @release: Pointer to the function that will clean up the object when the
 *           last reference to the object is released.
 *
 * Decrement the refcount, and if 0, call @release.  The caller may not
 * pass NULL or kfree() as the release function.
 *
 * Return: 1 if this call removed the object, otherwise return 0.  Beware,
 * if this function returns 0, another caller may have removed the object
 * by the time this function returns.  The return value is only certain
 * if you want to see if the object is definitely released.
 */
#[inline]
pub unsafe fn kref_put(kref: *mut kref, release: unsafe extern "C" fn(*mut kref)) -> i32 {
    if refcount_dec_and_test(&mut (*kref).refcount) {
        release(kref);
        return 1;
    }
    0
}

/**
 * kref_put_mutex - Decrement refcount for object
 * @kref: Object
 * @release: Pointer to the function that will clean up the object when the
 *           last reference to the object is released.
 * @mutex: Mutex which protects the release function.
 *
 * This variant of kref_lock() calls the @release function with the @mutex
 * held.  The @release function will release the mutex.
 */
#[inline]
pub unsafe fn kref_put_mutex(
    kref: *mut kref,
    release: unsafe extern "C" fn(*mut kref),
    mutex: *mut mutex,
) -> i32 {
    if refcount_dec_and_mutex_lock(&mut (*kref).refcount, mutex) {
        release(kref);
        return 1;
    }
    0
}

/**
 * kref_put_lock - Decrement refcount for object
 * @kref: Object
 * @release: Pointer to the function that will clean up the object when the
 *           last reference to the object is released.
 * @lock: Spinlock which protects the release function.
 *
 * This variant of kref_lock() calls the @release function with the @lock
 * held.  The @release function will release the lock.
 */
#[inline]
pub unsafe fn kref_put_lock(
    kref: *mut kref,
    release: unsafe extern "C" fn(*mut kref),
    lock: *mut spinlock_t,
) -> i32 {
    if refcount_dec_and_lock(&mut (*kref).refcount, lock) {
        release(kref);
        return 1;
    }
    0
}

/**
 * kref_get_unless_zero - Increment refcount for object unless it is zero.
 * @kref: object.
 *
 * This function is intended to simplify locking around refcounting for
 * objects that can be looked up from a lookup structure, and which are
 * removed from that lookup structure in the object destructor.
 * Operations on such objects require at least a read lock around
 * lookup + kref_get, and a write lock around kref_put + remove from lookup
 * structure. Furthermore, RCU implementations become extremely tricky.
 * With a lookup followed by a kref_get_unless_zero *with return value check*
 * locking in the kref_put path can be deferred to the actual removal from
 * the lookup structure and RCU lookups become trivial.
 *
 * Return: non-zero if the increment succeeded. Otherwise return 0.
 */
#[inline]
pub unsafe fn kref_get_unless_zero(kref: *mut kref) -> i32 {
    refcount_inc_not_zero(&mut (*kref).refcount)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
