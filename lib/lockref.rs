// SPDX-License-Identifier: GPL-2.0
// Dependencies are supplied by the Linux lockref headers/build environment.

#[cfg(USE_CMPXCHG_LOCKREF)]
macro_rules! cmpxchg_loop {
    ($lockref:expr, $code:block, $success:block) => {{
        let mut retry: i32 = 100;
        let mut old = unsafe { core::ptr::read_volatile(&(*$lockref).count) };
        while unsafe { arch_spin_value_unlocked((*$lockref).lock.rlock.raw_lock) } {
            let mut new = old;
            $code
            if unsafe { try_cmpxchg64_relaxed(&mut (*$lockref).count, &mut old, new) } {
                $success
            }
            retry -= 1;
            if retry == 0 { break; }
        }
    }};
}

#[cfg(not(USE_CMPXCHG_LOCKREF))]
macro_rules! cmpxchg_loop {
    ($lockref:expr, $code:block, $success:block) => {{}};
}

extern "C" {
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn assert_spin_locked(lock: *mut spinlock_t);
    fn arch_spin_value_unlocked(lock: raw_spinlock_t) -> bool;
    fn try_cmpxchg64_relaxed(ptr: *mut i64, old: *mut i64, new: i64) -> bool;
}

#[repr(C)]
pub struct lockref {
    pub lock: spinlock_t,
    pub count: i32,
}

#[repr(C)]
pub struct spinlock_t {
    pub rlock: raw_spinlock_t_wrapper,
}

#[repr(C)]
pub struct raw_spinlock_t_wrapper {
    pub raw_lock: raw_spinlock_t,
}

pub type raw_spinlock_t = usize;
pub const __LOCKREF_DEAD_VAL: i32 = -128;

pub unsafe extern "C" fn lockref_get(lockref: *mut lockref) {
    cmpxchg_loop!(lockref, {
        new = new.wrapping_add(1);
    }, {
        return;
    });

    spin_lock(&mut (*lockref).lock);
    (*lockref).count = (*lockref).count.wrapping_add(1);
    spin_unlock(&mut (*lockref).lock);
}

pub unsafe extern "C" fn lockref_get_not_zero(lockref: *mut lockref) -> bool {
    let mut retval = false;
    cmpxchg_loop!(lockref, {
        new = new.wrapping_add(1);
        if old <= 0 { return false; }
    }, {
        return true;
    });

    spin_lock(&mut (*lockref).lock);
    if (*lockref).count > 0 {
        (*lockref).count = (*lockref).count.wrapping_add(1);
        retval = true;
    }
    spin_unlock(&mut (*lockref).lock);
    retval
}

pub unsafe extern "C" fn lockref_put_return(lockref: *mut lockref) -> i32 {
    cmpxchg_loop!(lockref, {
        new = new.wrapping_sub(1);
        if old <= 0 { return -1; }
    }, {
        return new as i32;
    });
    -1
}

pub unsafe extern "C" fn lockref_put_or_lock(lockref: *mut lockref) -> bool {
    cmpxchg_loop!(lockref, {
        new = new.wrapping_sub(1);
        if old <= 1 { break; }
    }, {
        return true;
    });

    spin_lock(&mut (*lockref).lock);
    if (*lockref).count <= 1 { return false; }
    (*lockref).count = (*lockref).count.wrapping_sub(1);
    spin_unlock(&mut (*lockref).lock);
    true
}

pub unsafe extern "C" fn lockref_mark_dead(lockref: *mut lockref) {
    assert_spin_locked(&mut (*lockref).lock);
    (*lockref).count = __LOCKREF_DEAD_VAL;
}

pub unsafe extern "C" fn lockref_get_not_dead(lockref: *mut lockref) -> bool {
    let mut retval = false;
    cmpxchg_loop!(lockref, {
        new = new.wrapping_add(1);
        if old < 0 { return false; }
    }, {
        return true;
    });

    spin_lock(&mut (*lockref).lock);
    if (*lockref).count >= 0 {
        (*lockref).count = (*lockref).count.wrapping_add(1);
        retval = true;
    }
    spin_unlock(&mut (*lockref).lock);
    retval
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
