// SPDX-License-Identifier: GPL-2.0

// Dependencies: linux/slab.h, linux/spinlock.h, linux/once.h, linux/random.h,
// linux/module.h.

use core::mem::offset_of;
use core::ptr::addr_of_mut;
use kernel::bindings::{kfree, module, module_put, static_key_true, work_struct};
use kernel::ffi::{c_ulong, c_void};

// work_struct and module embed zero-sized lockdep keys; only pointers cross.
#[allow(improper_ctypes)]
extern "C" {
    fn lupos_once_alloc_work() -> *mut OnceWork;
    fn lupos_once_init_work(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn lupos_once_schedule_work(work: *mut work_struct) -> bool;
    fn lupos_once_key_enabled(key: *mut static_key_true) -> bool;
    fn lupos_once_branch_disable(key: *mut static_key_true);
    fn lupos_once_module_get(module: *mut module);
    fn lupos_once_bug_deferred_disabled();
    fn lupos_once_lock() -> c_ulong;
    fn lupos_once_unlock(flags: c_ulong);
    fn lupos_once_mutex_lock();
    fn lupos_once_mutex_unlock();
}

/// `struct once_work`, private to the original translation unit.
#[repr(C)]
pub struct OnceWork {
    work: work_struct,
    key: *mut static_key_true,
    module: *mut module,
}

/// The deferred work disabling a DO_ONCE() static branch.
///
/// # Safety
/// `w` is the `work` member of a queued `struct once_work`.
unsafe extern "C" fn once_deferred(w: *mut work_struct) {
    // SAFETY: the work item was allocated and queued by once_disable_jump().
    unsafe {
        let work = w.byte_sub(offset_of!(OnceWork, work)).cast::<OnceWork>();
        if !lupos_once_key_enabled((*work).key) {
            lupos_once_bug_deferred_disabled();
        }
        lupos_once_branch_disable((*work).key);
        module_put((*work).module);
        kfree(work.cast::<c_void>());
    }
}

/// # Safety
/// `key` is a DO_ONCE() static key and `mod_` its module (or NULL).
unsafe fn once_disable_jump(key: *mut static_key_true, mod_: *mut module) {
    // SAFETY: the new work item owns a module reference until it runs.
    unsafe {
        let w = lupos_once_alloc_work();
        if w.is_null() {
            return;
        }

        lupos_once_init_work(addr_of_mut!((*w).work), once_deferred);
        (*w).key = key;
        (*w).module = mod_;
        lupos_once_module_get(mod_);
        lupos_once_schedule_work(addr_of_mut!((*w).work));
    }
}

// static DEFINE_SPINLOCK(once_lock) keeps its static lockdep initializer in
// lib/once_helpers.c.

/// Takes once_lock; returns false (lock released) if `*done` is set.
///
/// # Safety
/// `done` and `flags` point to valid storage.
#[no_mangle]
pub unsafe extern "C" fn __do_once_start(done: *mut bool, flags: *mut c_ulong) -> bool {
    // SAFETY: done is read under once_lock, as in the C body.
    unsafe {
        *flags = lupos_once_lock();
        if *done {
            lupos_once_unlock(*flags);
            /* Keep sparse happy by restoring an even lock count on
             * this lock. In case we return here, we don't call into
             * __do_once_done but return early in the DO_ONCE() macro.
             */
            return false;
        }
    }

    true
}
// EXPORT_SYMBOL(__do_once_start);

/// Marks `*done`, releases once_lock and disables the static branch.
///
/// # Safety
/// once_lock is held by a matching __do_once_start().
#[no_mangle]
pub unsafe extern "C" fn __do_once_done(
    done: *mut bool,
    once_key: *mut static_key_true,
    flags: *mut c_ulong,
    mod_: *mut module,
) {
    // SAFETY: forwarded caller contract.
    unsafe {
        *done = true;
        lupos_once_unlock(*flags);
        once_disable_jump(once_key, mod_);
    }
}
// EXPORT_SYMBOL(__do_once_done);

// static DEFINE_MUTEX(once_mutex) is native in lib/once_helpers.c.

/// Takes once_mutex; returns false (mutex released) if `*done` is set.
///
/// # Safety
/// `done` points to valid storage; the caller may sleep.
#[no_mangle]
pub unsafe extern "C" fn __do_once_sleepable_start(done: *mut bool) -> bool {
    // SAFETY: done is read under once_mutex, as in the C body.
    unsafe {
        lupos_once_mutex_lock();
        if *done {
            lupos_once_mutex_unlock();
            /* Keep sparse happy by restoring an even lock count on
             * this mutex. In case we return here, we don't call into
             * __do_once_done but return early in the DO_ONCE_SLEEPABLE() macro.
             */
            return false;
        }
    }

    true
}
// EXPORT_SYMBOL(__do_once_sleepable_start);

/// Marks `*done`, releases once_mutex and disables the static branch.
///
/// # Safety
/// once_mutex is held by a matching __do_once_sleepable_start().
#[no_mangle]
pub unsafe extern "C" fn __do_once_sleepable_done(done: *mut bool, once_key: *mut static_key_true, _mod: *mut module) {
    // SAFETY: forwarded caller contract.
    unsafe {
        *done = true;
        lupos_once_mutex_unlock();
        lupos_once_branch_disable(once_key);
    }
}
// EXPORT_SYMBOL(__do_once_sleepable_done);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
