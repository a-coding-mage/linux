// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	cn_queue.c
 *
 * 2004+ Copyright (c) Evgeniy Polyakov <zbr@ioremap.net>
 * All rights reserved.
 */

// Linux kernel headers and symbols are supplied by external dependencies.

unsafe fn cn_queue_alloc_callback_entry(
    dev: *mut cn_queue_dev,
    name: *const core::ffi::c_char,
    id: *const cb_id,
    callback: Option<unsafe extern "C" fn(*mut cn_msg, *mut netlink_skb_parms)>,
) -> *mut cn_callback_entry {
    let cbq = kzalloc_obj::<cn_callback_entry>();
    if cbq.is_null() {
        pr_err!("Failed to create new callback queue.\n");
        return core::ptr::null_mut();
    }

    refcount_set(&mut (*cbq).refcnt, 1);

    atomic_inc(&mut (*dev).refcnt);
    (*cbq).pdev = dev;

    snprintf((*cbq).id.name.as_mut_ptr(), (*cbq).id.name.len(), "%s", name);
    core::ptr::copy_nonoverlapping(
        id,
        &mut (*cbq).id.id,
        1,
    );
    (*cbq).callback = callback;
    cbq
}

pub unsafe extern "C" fn cn_queue_release_callback(cbq: *mut cn_callback_entry) {
    if !refcount_dec_and_test(&mut (*cbq).refcnt) {
        return;
    }

    atomic_dec(&mut (*(*cbq).pdev).refcnt);
    kfree(cbq);
}

pub unsafe extern "C" fn cn_cb_equal(i1: *const cb_id, i2: *const cb_id) -> i32 {
    ((*i1).idx == (*i2).idx && (*i1).val == (*i2).val) as i32
}

pub unsafe extern "C" fn cn_queue_add_callback(
    dev: *mut cn_queue_dev,
    name: *const core::ffi::c_char,
    id: *const cb_id,
    callback: Option<unsafe extern "C" fn(*mut cn_msg, *mut netlink_skb_parms)>,
) -> i32 {
    let cbq = cn_queue_alloc_callback_entry(dev, name, id, callback);
    if cbq.is_null() {
        return -ENOMEM;
    }

    let mut found = 0;
    spin_lock_bh(&mut (*dev).queue_lock);
    list_for_each_entry!(__cbq, &mut (*dev).queue_list, callback_entry, {
        if cn_cb_equal(&(*__cbq).id.id, id) != 0 {
            found = 1;
            break;
        }
    });
    if found == 0 {
        list_add_tail(&mut (*cbq).callback_entry, &mut (*dev).queue_list);
    }
    spin_unlock_bh(&mut (*dev).queue_lock);

    if found != 0 {
        cn_queue_release_callback(cbq);
        return -EINVAL;
    }

    (*cbq).seq = 0;
    (*cbq).group = (*cbq).id.id.idx;

    0
}

pub unsafe extern "C" fn cn_queue_del_callback(dev: *mut cn_queue_dev, id: *const cb_id) {
    let mut found = 0;
    let mut cbq: *mut cn_callback_entry = core::ptr::null_mut();
    let mut n: *mut cn_callback_entry;

    spin_lock_bh(&mut (*dev).queue_lock);
    list_for_each_entry_safe!(cbq, n, &mut (*dev).queue_list, callback_entry, {
        if cn_cb_equal(&(*cbq).id.id, id) != 0 {
            list_del(&mut (*cbq).callback_entry);
            found = 1;
            break;
        }
    });
    spin_unlock_bh(&mut (*dev).queue_lock);

    if found != 0 {
        cn_queue_release_callback(cbq);
    }
}

pub unsafe extern "C" fn cn_queue_alloc_dev(
    name: *const core::ffi::c_char,
    nls: *mut sock,
) -> *mut cn_queue_dev {
    let dev = kzalloc_obj::<cn_queue_dev>();
    if dev.is_null() {
        return core::ptr::null_mut();
    }

    snprintf((*dev).name.as_mut_ptr(), (*dev).name.len(), "%s", name);
    atomic_set(&mut (*dev).refcnt, 0);
    INIT_LIST_HEAD(&mut (*dev).queue_list);
    spin_lock_init(&mut (*dev).queue_lock);

    (*dev).nls = nls;

    dev
}

pub unsafe extern "C" fn cn_queue_free_dev(mut dev: *mut cn_queue_dev) {
    let mut cbq: *mut cn_callback_entry;
    let mut n: *mut cn_callback_entry;

    spin_lock_bh(&mut (*dev).queue_lock);
    list_for_each_entry_safe!(cbq, n, &mut (*dev).queue_list, callback_entry, {
        list_del(&mut (*cbq).callback_entry);
    });
    spin_unlock_bh(&mut (*dev).queue_lock);

    while atomic_read(&(*dev).refcnt) != 0 {
        pr_info!("Waiting for %s to become free: refcnt=%d.\n", (*dev).name.as_ptr(), atomic_read(&(*dev).refcnt));
        msleep(1000);
    }

    kfree(dev);
    dev = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
