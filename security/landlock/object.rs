// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock LSM - Object management
 *
 * Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 */

// C dependencies:
//   <linux/bug.h>
//   <linux/compiler_types.h>
//   <linux/err.h>
//   <linux/kernel.h>
//   <linux/rcupdate.h>
//   <linux/refcount.h>
//   <linux/slab.h>
//   <linux/spinlock.h>
//   "object.h"

use crate::*;

#[no_mangle]
pub unsafe extern "C" fn landlock_create_object(
    underops: *const landlock_object_underops,
    underobj: *mut core::ffi::c_void,
) -> *mut landlock_object {
    let new_object: *mut landlock_object;

    if WARN_ON_ONCE(underops.is_null() || underobj.is_null()) {
        return ERR_PTR(-ENOENT) as *mut landlock_object;
    }
    new_object = kzalloc_obj(core::mem::size_of::<landlock_object>(), GFP_KERNEL_ACCOUNT)
        as *mut landlock_object;
    if new_object.is_null() {
        return ERR_PTR(-ENOMEM) as *mut landlock_object;
    }
    refcount_set(&mut (*new_object).usage, 1);
    spin_lock_init(&mut (*new_object).lock);
    (*new_object).underops = underops;
    (*new_object).underobj = underobj;
    new_object
}

/*
 * The caller must own the object (i.e. thanks to object->usage) to safely put
 * it.
 */
#[no_mangle]
pub unsafe extern "C" fn landlock_put_object(object: *mut landlock_object) {
    /*
     * The call to @object->underops->release(object) might sleep, e.g.
     * because of iput().
     */
    might_sleep();
    if object.is_null() {
        return;
    }

    /*
     * If the @object's refcount cannot drop to zero, we can just decrement
     * the refcount without holding a lock. Otherwise, the decrement must
     * happen under @object->lock for synchronization with things like
     * get_inode_object().
     */
    if refcount_dec_and_lock(&mut (*object).usage, &mut (*object).lock) {
        __acquire(&mut (*object).lock);
        /*
         * With @object->lock initially held, remove the reference from
         * @object->underobj to @object (if it still exists).
         */
        ((*(*object).underops).release)(object);
        kfree_rcu(object, rcu_free);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
