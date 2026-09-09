// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit helpers for backtrace suppression
 *
 * Copyright (C) 2025 Alessandro Carminati <acarmina@redhat.com>
 * Copyright (C) 2024 Guenter Roeck <linux@roeck-us.net>
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct kunit_suppressed_warning {
    pub node: list_head,
    pub task: *mut task_struct,
    pub test: *mut kunit,
    pub counter: atomic_t,
}

static mut suppressed_warnings: list_head = list_head {};
static mut suppressed_warnings_lock: spinlock_t = spinlock_t {};

unsafe fn kunit_suppress_warning_remove(w: *mut kunit_suppressed_warning) {
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&raw mut suppressed_warnings_lock, &mut flags);
    list_del_rcu(&mut (*w).node);
    spin_unlock_irqrestore(&raw mut suppressed_warnings_lock, flags);
    put_task_struct((*w).task);
}

// KUNIT_DEFINE_ACTION_WRAPPER(kunit_suppress_warning_cleanup,
//                             kunit_suppress_warning_remove,
//                             struct kunit_suppressed_warning *);
pub unsafe extern "C" fn kunit_suppress_warning_cleanup(
    w: *mut kunit_suppressed_warning,
) {
    kunit_suppress_warning_remove(w);
}

pub unsafe extern "C" fn kunit_has_active_suppress_warning() -> bool {
    __kunit_is_suppressed_warning_impl(false)
}

pub unsafe extern "C" fn kunit_start_suppress_warning(
    test: *mut kunit,
) -> *mut kunit_suppressed_warning {
    let mut w: *mut kunit_suppressed_warning;
    let mut flags: c_ulong = 0;
    let ret: c_int;

    if kunit_has_active_suppress_warning() {
        KUNIT_FAIL(test, "Another suppression block is already active");
        return core::ptr::null_mut();
    }

    w = kunit_kzalloc(test, core::mem::size_of::<kunit_suppressed_warning>(), GFP_KERNEL);
    if w.is_null() {
        KUNIT_FAIL(test, "Failed to allocate suppression handle.");
        return core::ptr::null_mut();
    }

    (*w).task = get_task_struct(current);
    (*w).test = test;

    spin_lock_irqsave(&raw mut suppressed_warnings_lock, &mut flags);
    list_add_rcu(&mut (*w).node, &raw mut suppressed_warnings);
    spin_unlock_irqrestore(&raw mut suppressed_warnings_lock, flags);

    ret = kunit_add_action_or_reset(test, kunit_suppress_warning_cleanup, w);
    if ret != 0 {
        KUNIT_FAIL(test, "Failed to add suppression cleanup action.");
        return core::ptr::null_mut();
    }

    w
}

pub unsafe extern "C" fn kunit_end_suppress_warning(
    test: *mut kunit,
    w: *mut kunit_suppressed_warning,
) {
    if w.is_null() {
        return;
    }
    kunit_release_action(test, kunit_suppress_warning_cleanup, w);
}

pub unsafe extern "C" fn __kunit_suppress_auto_cleanup(
    wp: *mut *mut kunit_suppressed_warning,
) {
    if !(*wp).is_null() {
        kunit_end_suppress_warning((**wp).test, *wp);
    }
}

pub unsafe extern "C" fn kunit_suppressed_warning_count(
    w: *mut kunit_suppressed_warning,
) -> c_int {
    if !w.is_null() {
        atomic_read(&raw mut (*w).counter)
    } else {
        0
    }
}

pub unsafe extern "C" fn __kunit_is_suppressed_warning_impl(count: bool) -> bool {
    // guard(rcu)();
    let mut w: *mut kunit_suppressed_warning;
    // list_for_each_entry_rcu(w, &suppressed_warnings, node) {
    for_each_entry_rcu!(w, &raw mut suppressed_warnings, node);
    {
        if (*w).task == current {
            if count {
                atomic_inc(&raw mut (*w).counter);
            }
            return true;
        }
    }

    false
}

// External kernel types, globals, functions, constants, and macros referenced
// above are supplied by the corresponding translated dependencies.
extern "C" {
    type list_head;
    type task_struct;
    type kunit;
    type atomic_t;
    type spinlock_t;
    type c_ulong;
    type c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
