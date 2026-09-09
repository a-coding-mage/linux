/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Percpu refcounts:
 * (C) 2012 Google, Inc.
 * Author: Kent Overstreet <koverstreet@google.com>
 *
 * This implements a refcount with similar semantics to atomic_t - atomic_inc(),
 * atomic_dec_and_test() - but percpu.
 *
 * There is one important difference between percpu refs and normal atomic_t
 * refcounts; you have to keep track of your initial refcount, and then when you
 * start shutting down you call percpu_ref_kill() _before_ dropping the initial
 * refcount.
 *
 * The refcount will have a range of 0 to LONG_MAX, i.e. one bit less
 * than an atomic_long_t - this is because of the way shutdown works, see
 * percpu_ref_kill()/PERCPU_COUNT_BIAS.
 *
 * Before you call percpu_ref_kill(), percpu_ref_put() does not check for the
 * refcount hitting 0 - it can't, if it was in percpu mode. percpu_ref_kill()
 * puts the ref back in single atomic_t mode, collecting the per cpu refs and
 * issuing the appropriate barriers, and then marks the ref as shutting down so
 * that percpu_ref_put() will check for the ref hitting 0. After it returns,
 * it's safe to drop the initial ref.
 *
 * USAGE:
 *
 * See fs/aio.c for some example usage; it's used there for struct kioctx, which
 * is created when userspaces calls io_setup(), and destroyed when userspace
 * calls io_destroy() or the process exits.
 *
 * In the aio code, kill_ioctx() is called when we wish to destroy a kioctx; it
 * removes the kioctx from the proccess's table of kioctxs and kills percpu_ref.
 * After that, there can't be any new users of the kioctx (from lookup_ioctx())
 * and it's then safe to drop the initial ref with percpu_ref_put().
 *
 * Note that the free path, free_ioctx(), needs to go through explicit call_rcu()
 * to synchronize with RCU protected lookup_ioctx(). percpu_ref operations don't
 * imply RCU grace periods of any kind and if a user wants to combine percpu_ref
 * with RCU protection, it must be done explicitly.
 *
 * Code that does a two stage shutdown like this often needs some kind of
 * explicit synchronization to ensure the initial refcount can only be dropped
 * once - percpu_ref_kill() does this for you, it returns true once and false if
 * someone else already called it. The aio code uses it this way, but it's not
 * necessary if the code has some other mechanism to synchronize teardown.
 * around.
 */

// C includes and build-provided symbols are supplied by other translated headers.

pub struct percpu_ref;
pub type percpu_ref_func_t = unsafe extern "C" fn(*mut percpu_ref);

pub const __PERCPU_REF_ATOMIC: usize = 1usize << 0;
pub const __PERCPU_REF_DEAD: usize = 1usize << 1;
pub const __PERCPU_REF_ATOMIC_DEAD: usize = __PERCPU_REF_ATOMIC | __PERCPU_REF_DEAD;
pub const __PERCPU_REF_FLAG_BITS: usize = 2;

pub const PERCPU_REF_INIT_ATOMIC: u32 = 1 << 0;
pub const PERCPU_REF_INIT_DEAD: u32 = 1 << 1;
pub const PERCPU_REF_ALLOW_REINIT: u32 = 1 << 2;

#[repr(C)]
pub struct percpu_ref_data {
    pub count: atomic_long_t,
    pub release: Option<percpu_ref_func_t>,
    pub confirm_switch: Option<percpu_ref_func_t>,
    pub force_atomic: bool,
    pub allow_reinit: bool,
    pub rcu: rcu_head,
    pub ref_: *mut percpu_ref,
}

#[repr(C)]
pub struct percpu_ref {
    pub percpu_count_ptr: usize,
    pub data: *mut percpu_ref_data,
}

extern "C" {
    pub fn percpu_ref_init(ref_: *mut percpu_ref, release: Option<percpu_ref_func_t>, flags: u32, gfp: gfp_t) -> i32;
    pub fn percpu_ref_exit(ref_: *mut percpu_ref);
    pub fn percpu_ref_switch_to_atomic(ref_: *mut percpu_ref, confirm_switch: Option<percpu_ref_func_t>);
    pub fn percpu_ref_switch_to_atomic_sync(ref_: *mut percpu_ref);
    pub fn percpu_ref_switch_to_percpu(ref_: *mut percpu_ref);
    pub fn percpu_ref_kill_and_confirm(ref_: *mut percpu_ref, confirm_kill: Option<percpu_ref_func_t>);
    pub fn percpu_ref_resurrect(ref_: *mut percpu_ref);
    pub fn percpu_ref_reinit(ref_: *mut percpu_ref);
    pub fn percpu_ref_is_zero(ref_: *mut percpu_ref) -> bool;
}

#[inline]
pub unsafe fn percpu_ref_kill(ref_: *mut percpu_ref) {
    percpu_ref_kill_and_confirm(ref_, None);
}

#[inline]
pub unsafe fn __ref_is_percpu(ref_: *mut percpu_ref, percpu_countp: *mut *mut usize) -> bool {
    // READ_ONCE is required here because the atomic flag may be set asynchronously.
    let percpu_ptr = READ_ONCE((*ref_).percpu_count_ptr);
    if unlikely(percpu_ptr & __PERCPU_REF_ATOMIC_DEAD) {
        return false;
    }
    *percpu_countp = percpu_ptr as *mut usize;
    true
}

#[inline]
pub unsafe fn percpu_ref_get_many(ref_: *mut percpu_ref, nr: usize) {
    let mut percpu_count: *mut usize = core::ptr::null_mut();
    rcu_read_lock();
    if __ref_is_percpu(ref_, &mut percpu_count) {
        this_cpu_add(*percpu_count, nr);
    } else {
        atomic_long_add(nr, &mut (*(*ref_).data).count);
    }
    rcu_read_unlock();
}

#[inline]
pub unsafe fn percpu_ref_get(ref_: *mut percpu_ref) { percpu_ref_get_many(ref_, 1); }

#[inline]
pub unsafe fn percpu_ref_tryget_many(ref_: *mut percpu_ref, nr: usize) -> bool {
    let mut percpu_count: *mut usize = core::ptr::null_mut();
    rcu_read_lock();
    let ret = if __ref_is_percpu(ref_, &mut percpu_count) {
        this_cpu_add(*percpu_count, nr); true
    } else {
        atomic_long_add_unless(&mut (*(*ref_).data).count, nr, 0)
    };
    rcu_read_unlock();
    ret
}

#[inline]
pub unsafe fn percpu_ref_tryget(ref_: *mut percpu_ref) -> bool { percpu_ref_tryget_many(ref_, 1) }

#[inline]
pub unsafe fn percpu_ref_tryget_live_rcu(ref_: *mut percpu_ref) -> bool {
    let mut percpu_count: *mut usize = core::ptr::null_mut();
    WARN_ON_ONCE(!rcu_read_lock_held());
    if likely(__ref_is_percpu(ref_, &mut percpu_count)) {
        this_cpu_inc(*percpu_count); true
    } else if ((*ref_).percpu_count_ptr & __PERCPU_REF_DEAD) == 0 {
        atomic_long_inc_not_zero(&mut (*(*ref_).data).count)
    } else { false }
}

#[inline]
pub unsafe fn percpu_ref_tryget_live(ref_: *mut percpu_ref) -> bool {
    rcu_read_lock();
    let ret = percpu_ref_tryget_live_rcu(ref_);
    rcu_read_unlock();
    ret
}

#[inline]
pub unsafe fn percpu_ref_put_many(ref_: *mut percpu_ref, nr: usize) {
    let mut percpu_count: *mut usize = core::ptr::null_mut();
    rcu_read_lock();
    if __ref_is_percpu(ref_, &mut percpu_count) {
        this_cpu_sub(*percpu_count, nr);
    } else if unlikely(atomic_long_sub_and_test(nr, &mut (*(*ref_).data).count)) {
        if let Some(release) = (*(*ref_).data).release { release(ref_); }
    }
    rcu_read_unlock();
}

#[inline]
pub unsafe fn percpu_ref_put(ref_: *mut percpu_ref) { percpu_ref_put_many(ref_, 1); }

#[inline]
pub unsafe fn percpu_ref_is_dying(ref_: *mut percpu_ref) -> bool {
    ((*ref_).percpu_count_ptr & __PERCPU_REF_DEAD) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
