// SPDX-License-Identifier: GPL-2.0-only
/*
 * eCryptfs: Linux filesystem encryption layer
 *
 * Copyright (C) 2004-2008 International Business Machines Corp.
 *   Author(s): Michael A. Halcrow <mhalcrow@us.ibm.com>
 *\t\tTyler Hicks <code@tyhicks.com>
 */

// Dependencies supplied by the kernel and ecryptfs headers are intentionally
// left external to this translation unit.

static mut ECRYPTFS_MSG_CTX_FREE_LIST: list_head = list_head { }; // LIST_HEAD
static mut ECRYPTFS_MSG_CTX_ALLOC_LIST: list_head = list_head { }; // LIST_HEAD
static mut ECRYPTFS_MSG_CTX_LISTS_MUX: mutex = mutex { };

static mut ECRYPTFS_DAEMON_HASH: *mut hlist_head = core::ptr::null_mut();
static mut ECRYPTFS_DAEMON_HASH_MUX: mutex = mutex { };
static mut ECRYPTFS_HASH_BITS: i32 = 0;

static mut ECRYPTFS_MSG_COUNTER: u32 = 0;
static mut ECRYPTFS_MSG_CTX_ARR: *mut ecryptfs_msg_ctx = core::ptr::null_mut();

unsafe fn ecryptfs_current_euid_hash(_uid: kuid_t) -> usize {
    hash_long(from_kuid(&init_user_ns, current_euid()) as usize, ECRYPTFS_HASH_BITS)
}

/// Acquires a context element from the free list and locks its mutex.
unsafe fn ecryptfs_acquire_free_msg_ctx(msg_ctx: *mut *mut ecryptfs_msg_ctx) -> i32 {
    let mut p: *mut list_head;
    let rc: i32;
    if list_empty(&raw mut ECRYPTFS_MSG_CTX_FREE_LIST) {
        printk(KERN_WARNING, "%s: The eCryptfs free context list is empty.  It may be helpful to specify the ecryptfs_message_buf_len parameter to be greater than the current value of [%d]\n", __func__, ecryptfs_message_buf_len);
        rc = -ENOMEM;
        return rc;
    }
    list_for_each!(p, &raw mut ECRYPTFS_MSG_CTX_FREE_LIST) {
        *msg_ctx = list_entry(p, ecryptfs_msg_ctx, node);
        if mutex_trylock(&mut (**msg_ctx).mux) {
            (**msg_ctx).task = current;
            return 0;
        }
    }
    rc = -ENOMEM;
    rc
}

unsafe fn ecryptfs_msg_ctx_free_to_alloc(msg_ctx: *mut ecryptfs_msg_ctx) {
    list_move(&mut (*msg_ctx).node, &raw mut ECRYPTFS_MSG_CTX_ALLOC_LIST);
    (*msg_ctx).state = ECRYPTFS_MSG_CTX_STATE_PENDING;
    ECRYPTFS_MSG_COUNTER = ECRYPTFS_MSG_COUNTER.wrapping_add(1);
    (*msg_ctx).counter = ECRYPTFS_MSG_COUNTER;
}

pub unsafe fn ecryptfs_msg_ctx_alloc_to_free(msg_ctx: *mut ecryptfs_msg_ctx) {
    list_move(&mut (*msg_ctx).node, &raw mut ECRYPTFS_MSG_CTX_FREE_LIST);
    kfree((*msg_ctx).msg);
    (*msg_ctx).msg = core::ptr::null_mut();
    (*msg_ctx).state = ECRYPTFS_MSG_CTX_STATE_FREE;
}

pub unsafe fn ecryptfs_find_daemon_by_euid(daemon: *mut *mut ecryptfs_daemon) -> i32 {
    let mut d: *mut ecryptfs_daemon;
    hlist_for_each_entry!(d, &mut *ECRYPTFS_DAEMON_HASH.add(ecryptfs_current_euid_hash(0)), euid_chain) {
        if uid_eq((*(*d).file).f_cred.euid, current_euid()) {
            *daemon = d;
            return 0;
        }
    }
    -EINVAL
}

pub unsafe fn ecryptfs_spawn_daemon(daemon: *mut *mut ecryptfs_daemon, file: *mut file) -> i32 {
    let mut rc = 0;
    *daemon = kzalloc_obj::<ecryptfs_daemon>();
    if (*daemon).is_null() {
        rc = -ENOMEM;
        return rc;
    }
    (**daemon).file = file;
    mutex_init(&mut (**daemon).mux);
    INIT_LIST_HEAD(&mut (**daemon).msg_ctx_out_queue);
    init_waitqueue_head(&mut (**daemon).wait);
    (**daemon).num_queued_msg_ctx = 0;
    hlist_add_head(&mut (**daemon).euid_chain, &mut *ECRYPTFS_DAEMON_HASH.add(ecryptfs_current_euid_hash(0)));
    rc
}

pub unsafe fn ecryptfs_exorcise_daemon(daemon: *mut ecryptfs_daemon) -> i32 {
    let mut rc = 0;
    mutex_lock(&mut (*daemon).mux);
    if ((*daemon).flags & ECRYPTFS_DAEMON_IN_READ) != 0 || ((*daemon).flags & ECRYPTFS_DAEMON_IN_POLL) != 0 {
        rc = -EBUSY;
        mutex_unlock(&mut (*daemon).mux);
        return rc;
    }
    mutex_lock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX);
    let mut msg_ctx: *mut ecryptfs_msg_ctx;
    let mut msg_ctx_tmp: *mut ecryptfs_msg_ctx;
    list_for_each_entry_safe!(msg_ctx, msg_ctx_tmp, &mut (*daemon).msg_ctx_out_queue, daemon_out_list) {
        list_del(&mut (*msg_ctx).daemon_out_list);
        (*daemon).num_queued_msg_ctx -= 1;
        printk(KERN_WARNING, "%s: Warning: dropping message that is in the out queue of a dying daemon\n", __func__);
        ecryptfs_msg_ctx_alloc_to_free(msg_ctx);
    }
    mutex_unlock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX);
    hlist_del(&mut (*daemon).euid_chain);
    mutex_unlock(&mut (*daemon).mux);
    kfree_sensitive(daemon);
    rc
}

pub unsafe fn ecryptfs_process_response(daemon: *mut ecryptfs_daemon, msg: *mut ecryptfs_message, seq: u32) -> i32 {
    let _ = daemon;
    if (*msg).index >= ecryptfs_message_buf_len { return -EINVAL; }
    let msg_ctx = ECRYPTFS_MSG_CTX_ARR.add((*msg).index as usize);
    mutex_lock(&mut (*msg_ctx).mux);
    if (*msg_ctx).state != ECRYPTFS_MSG_CTX_STATE_PENDING || (*msg_ctx).counter != seq {
        mutex_unlock(&mut (*msg_ctx).mux);
        return -EINVAL;
    }
    let msg_size = struct_size(msg, data, (*msg).data_len);
    (*msg_ctx).msg = kmemdup(msg as *const _, msg_size, GFP_KERNEL);
    if (*msg_ctx).msg.is_null() { mutex_unlock(&mut (*msg_ctx).mux); return -ENOMEM; }
    (*msg_ctx).state = ECRYPTFS_MSG_CTX_STATE_DONE;
    wake_up_process((*msg_ctx).task);
    mutex_unlock(&mut (*msg_ctx).mux);
    0
}

unsafe fn ecryptfs_send_message_locked(data: *mut i8, data_len: i32, msg_type: u8, msg_ctx: *mut *mut ecryptfs_msg_ctx) -> i32 {
    let mut daemon: *mut ecryptfs_daemon = core::ptr::null_mut();
    let mut rc = ecryptfs_find_daemon_by_euid(&mut daemon);
    if rc != 0 { return -ENOTCONN; }
    mutex_lock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX);
    rc = ecryptfs_acquire_free_msg_ctx(msg_ctx);
    if rc != 0 { mutex_unlock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX); return rc; }
    ecryptfs_msg_ctx_free_to_alloc(*msg_ctx);
    mutex_unlock(&mut (**msg_ctx).mux);
    mutex_unlock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX);
    rc = ecryptfs_send_miscdev(data, data_len, *msg_ctx, msg_type, 0, daemon);
    if rc != 0 {
        mutex_lock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX);
        mutex_lock(&mut (**msg_ctx).mux);
        ecryptfs_msg_ctx_alloc_to_free(*msg_ctx);
        mutex_unlock(&mut (**msg_ctx).mux);
        mutex_unlock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX);
        *msg_ctx = core::ptr::null_mut();
    }
    rc
}

pub unsafe fn ecryptfs_send_message(data: *mut i8, data_len: i32, msg_ctx: *mut *mut ecryptfs_msg_ctx) -> i32 {
    mutex_lock(&raw mut ECRYPTFS_DAEMON_HASH_MUX);
    let rc = ecryptfs_send_message_locked(data, data_len, ECRYPTFS_MSG_REQUEST, msg_ctx);
    mutex_unlock(&raw mut ECRYPTFS_DAEMON_HASH_MUX);
    rc
}

pub unsafe fn ecryptfs_wait_for_response(msg_ctx: *mut ecryptfs_msg_ctx, msg: *mut *mut ecryptfs_message) -> i32 {
    let mut timeout: signed_long = ecryptfs_message_wait_timeout * HZ;
    loop {
        timeout = schedule_timeout_interruptible(timeout);
        mutex_lock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX);
        mutex_lock(&mut (*msg_ctx).mux);
        let mut rc = 0;
        if (*msg_ctx).state != ECRYPTFS_MSG_CTX_STATE_DONE {
            if timeout != 0 { mutex_unlock(&mut (*msg_ctx).mux); mutex_unlock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX); continue; }
            rc = -ENOMSG;
        } else { *msg = (*msg_ctx).msg; (*msg_ctx).msg = core::ptr::null_mut(); }
        ecryptfs_msg_ctx_alloc_to_free(msg_ctx);
        mutex_unlock(&mut (*msg_ctx).mux);
        mutex_unlock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX);
        return rc;
    }
}

pub unsafe fn ecryptfs_init_messaging() -> i32 {
    if ecryptfs_number_of_users > ECRYPTFS_MAX_NUM_USERS { ecryptfs_number_of_users = ECRYPTFS_MAX_NUM_USERS; }
    mutex_lock(&raw mut ECRYPTFS_DAEMON_HASH_MUX);
    ECRYPTFS_HASH_BITS = 1;
    while ecryptfs_number_of_users >> ECRYPTFS_HASH_BITS != 0 { ECRYPTFS_HASH_BITS += 1; }
    ECRYPTFS_DAEMON_HASH = kmalloc((core::mem::size_of::<hlist_head>() * (1usize << ECRYPTFS_HASH_BITS)) as _, GFP_KERNEL);
    if ECRYPTFS_DAEMON_HASH.is_null() { mutex_unlock(&raw mut ECRYPTFS_DAEMON_HASH_MUX); return -ENOMEM; }
    for i in 0..(1usize << ECRYPTFS_HASH_BITS) { INIT_HLIST_HEAD(&mut *ECRYPTFS_DAEMON_HASH.add(i)); }
    mutex_unlock(&raw mut ECRYPTFS_DAEMON_HASH_MUX);
    ECRYPTFS_MSG_CTX_ARR = kmalloc((core::mem::size_of::<ecryptfs_msg_ctx>() * ecryptfs_message_buf_len as usize) as _, GFP_KERNEL);
    if ECRYPTFS_MSG_CTX_ARR.is_null() { kfree(ECRYPTFS_DAEMON_HASH); return -ENOMEM; }
    mutex_lock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX);
    ECRYPTFS_MSG_COUNTER = 0;
    for i in 0..ecryptfs_message_buf_len as usize {
        let c = &mut *ECRYPTFS_MSG_CTX_ARR.add(i);
        INIT_LIST_HEAD(&mut c.node); INIT_LIST_HEAD(&mut c.daemon_out_list); mutex_init(&mut c.mux); mutex_lock(&mut c.mux);
        c.index = i as _; c.state = ECRYPTFS_MSG_CTX_STATE_FREE; c.counter = 0; c.task = core::ptr::null_mut(); c.msg = core::ptr::null_mut();
        list_add_tail(&mut c.node, &raw mut ECRYPTFS_MSG_CTX_FREE_LIST); mutex_unlock(&mut c.mux);
    }
    mutex_unlock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX);
    let rc = ecryptfs_init_ecryptfs_miscdev();
    if rc != 0 { ecryptfs_release_messaging(); }
    rc
}

pub unsafe fn ecryptfs_release_messaging() {
    if !ECRYPTFS_MSG_CTX_ARR.is_null() {
        mutex_lock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX);
        for i in 0..ecryptfs_message_buf_len as usize { let c = &mut *ECRYPTFS_MSG_CTX_ARR.add(i); mutex_lock(&mut c.mux); kfree(c.msg); mutex_unlock(&mut c.mux); }
        kfree(ECRYPTFS_MSG_CTX_ARR); mutex_unlock(&raw mut ECRYPTFS_MSG_CTX_LISTS_MUX);
    }
    if !ECRYPTFS_DAEMON_HASH.is_null() {
        mutex_lock(&raw mut ECRYPTFS_DAEMON_HASH_MUX);
        for i in 0..(1usize << ECRYPTFS_HASH_BITS) { let mut daemon: *mut ecryptfs_daemon; let mut n: *mut hlist_node; hlist_for_each_entry_safe!(daemon, n, &mut *ECRYPTFS_DAEMON_HASH.add(i), euid_chain) { let _ = ecryptfs_exorcise_daemon(daemon); } }
        kfree(ECRYPTFS_DAEMON_HASH); mutex_unlock(&raw mut ECRYPTFS_DAEMON_HASH_MUX);
    }
    ecryptfs_destroy_ecryptfs_miscdev();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
