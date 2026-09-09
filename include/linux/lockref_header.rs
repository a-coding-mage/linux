/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Locked reference counts.
 *
 * These are different from just plain atomic refcounts in that they
 * are atomic with respect to the spinlock that goes with them.  In
 * particular, there can be implementations that don't actually get
 * the spinlock for the common decrement/increment operations, but they
 * still have to check that the operation is done semantically as if
 * the spinlock had been taken (using a cmpxchg operation that covers
 * both the lock and the count word, or using memory transactions, for
 * example).
 */

/* The C header includes linux/spinlock.h and generated/bounds.h. */

/*
 * USE_CMPXCHG_LOCKREF is enabled when
 * CONFIG_ARCH_USE_CMPXCHG_LOCKREF, CONFIG_SMP, and SPINLOCK_SIZE <= 4.
 */

extern "C" {
    pub type spinlock_t;
    pub fn spin_lock_init(lock: *mut spinlock_t);
}

#[repr(C)]
pub struct LockrefLockCount {
    pub lock: spinlock_t,
    pub count: i32,
}

#[repr(C)]
pub union lockref_union {
    /* Present when USE_CMPXCHG_LOCKREF is enabled. */
    pub lock_count: u64,
    pub fields: core::mem::ManuallyDrop<LockrefLockCount>,
}

#[repr(C)]
pub struct lockref {
    pub value: lockref_union,
}

pub const __LOCKREF_DEAD_VAL: i32 = -128;

/**
 * lockref_init - Initialize a lockref
 * @lockref: pointer to lockref structure
 *
 * Initializes @lockref->count to 1.
 */
#[inline]
pub unsafe fn lockref_init(lockref: *mut lockref) {
    spin_lock_init(&mut (*lockref).value.fields.lock);
    (*lockref).value.fields.count = 1;
}

extern "C" {
    pub fn lockref_get(lockref: *mut lockref);
    pub fn lockref_put_return(lockref: *mut lockref) -> i32;
    pub fn lockref_get_not_zero(lockref: *mut lockref) -> bool;
    pub fn lockref_put_or_lock(lockref: *mut lockref) -> bool;

    pub fn lockref_mark_dead(lockref: *mut lockref);
    pub fn lockref_get_not_dead(lockref: *mut lockref) -> bool;
}

/* Must be called under spinlock for reliable results */
#[inline]
pub unsafe fn lockref_is_dead(l: *const lockref) -> bool {
    core::ptr::read_volatile(&(*l).value.fields.count) == __LOCKREF_DEAD_VAL
}

#[inline]
pub unsafe fn lockref_is_dead_or_zero(l: *const lockref) -> bool {
    let count = core::ptr::read_volatile(&(*l).value.fields.count);
    count == __LOCKREF_DEAD_VAL || count == 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
