/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Landlock LSM - Object management
 *
 * Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 */

/* Dependencies from the original header:
 * <linux/compiler_types.h>
 * <linux/refcount.h>
 * <linux/spinlock.h>
 */

use core::ffi::c_void;

pub enum rcu_head {}
pub enum refcount_t {}
pub enum spinlock_t {}

#[repr(C)]
pub struct landlock_object {
    /**
     * @usage: This counter is used to tie an object to the rules matching
     * it or to keep it alive while adding a new rule.  If this counter
     * reaches zero, this struct must not be modified, but this counter can
     * still be read from within an RCU read-side critical section.  When
     * adding a new rule to an object with a usage counter of zero, we must
     * wait until the pointer to this object is set to NULL (or recycled).
     */
    pub usage: refcount_t,
    /**
     * @lock: Protects against concurrent modifications.  This lock must be
     * held from the time @usage drops to zero until any weak references
     * from @underobj to this object have been cleaned up.
     *
     * Lock ordering: inode->i_lock nests inside this.
     */
    pub lock: spinlock_t,
    /**
     * @underobj: Used when cleaning up an object and to mark an object as
     * tied to its underlying kernel structure.  This pointer is protected
     * by @lock.  Cf. landlock_release_inodes() and release_inode().
     */
    pub underobj: *mut c_void,
    pub anonymous: landlock_object__bindgen_ty_1,
}

/**
 * struct landlock_object_underops - Operations on an underlying object
 */
#[repr(C)]
pub struct landlock_object_underops {
    /**
     * @release: Releases the underlying object (e.g. iput() for an inode).
     *
     * Original annotation: __releases(object->lock)
     */
    pub release: Option<unsafe extern "C" fn(object: *mut landlock_object)>,
}

#[repr(C)]
pub union landlock_object__bindgen_ty_1 {
    /**
     * @rcu_free: Enables lockless use of @usage, @lock and
     * @underobj from within an RCU read-side critical section.
     * @rcu_free and @underops are only used by
     * landlock_put_object().
     */
    pub rcu_free: core::mem::ManuallyDrop<rcu_head>,
    /**
     * @underops: Enables landlock_put_object() to release the
     * underlying object (e.g. inode).
     */
    pub underops: *const landlock_object_underops,
}

unsafe extern "C" {
    pub fn landlock_create_object(
        underops: *const landlock_object_underops,
        underobj: *mut c_void,
    ) -> *mut landlock_object;

    pub fn landlock_put_object(object: *mut landlock_object);

    pub fn refcount_inc(r: *mut refcount_t);
}

#[inline]
pub unsafe fn landlock_get_object(object: *mut landlock_object) {
    if !object.is_null() {
        unsafe {
            refcount_inc(&mut (*object).usage);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
