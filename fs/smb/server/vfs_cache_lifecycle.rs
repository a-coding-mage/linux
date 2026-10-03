// SPDX-License-Identifier: GPL-2.0-or-later
// Source-body translation of vfs_cache.c, OPEN_ID_TYPE_VOLATILE_ID through EOF.
// Included into vfs_cache.rs; all layouts come from configured C bindings.

const OPEN_ID_TYPE_VOLATILE_ID: c_int = 0;
const OPEN_ID_TYPE_PERSISTENT_ID: c_int = 1;

unsafe fn __open_id_set(fp: *mut ksmbd_file, id: u64, kind: c_int) {
    if kind == OPEN_ID_TYPE_VOLATILE_ID {
        (*fp).volatile_id = id;
    }
    if kind == OPEN_ID_TYPE_PERSISTENT_ID {
        (*fp).persistent_id = id;
    }
}

unsafe fn __open_id(
    ft: *mut ksmbd_file_table,
    fp: *mut ksmbd_file,
    kind: c_int,
) -> c_int {
    if kind == OPEN_ID_TYPE_VOLATILE_ID && fd_limit_depleted() {
        __open_id_set(fp, KSMBD_NO_FID as u64, kind);
        return -(EMFILE as c_int);
    }

    idr_preload(RVC_DEFAULT_GFP);
    rvc_write_lock(addr_of_mut!((*ft).lock));
    let mut ret = idr_alloc_cyclic(
        (*ft).idr,
        fp.cast(),
        KSMBD_START_FID as c_int,
        c_int::MAX - 1,
        RVC_GFP_NOWAIT,
    );
    let id;
    if ret >= 0 {
        id = ret as u64;
        ret = 0;
    } else {
        id = KSMBD_NO_FID as u64;
        fd_limit_close();
    }
    __open_id_set(fp, id, kind);
    rvc_write_unlock(addr_of_mut!((*ft).lock));
    rvc_idr_preload_end();
    ret
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_open_durable_fd(fp: *mut ksmbd_file) -> c_uint {
    __open_id(addr_of_mut!(global_ft), fp, OPEN_ID_TYPE_PERSISTENT_ID);
    (*fp).persistent_id as c_uint
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_open_fd(
    work: *mut ksmbd_work,
    filp: *mut file,
) -> *mut ksmbd_file {
    let fp = rvc_cache_zalloc(filp_cache, RVC_DEFAULT_GFP).cast::<ksmbd_file>();
    if fp.is_null() {
        rvc_err_alloc();
        return rvc_err_ptr(-(ENOMEM as c_long)).cast();
    }

    rvc_list_init(addr_of_mut!((*fp).blocked_works));
    rvc_list_init(addr_of_mut!((*fp).node));
    rvc_list_init(addr_of_mut!((*fp).lock_list));
    rvc_list_init(addr_of_mut!((*fp).notify_pendings));
    rvc_spin_lock_init(addr_of_mut!((*fp).f_lock));
    rvc_mutex_init(addr_of_mut!((*fp).readdir_lock));
    rvc_atomic_set(addr_of_mut!((*fp).refcount), 1);

    (*fp).filp = filp;
    // Keep the connection alive until durable preservation, final close, or
    // either failure below releases the file's own connection reference.
    (*fp).conn = ksmbd_conn_get((*work).conn);
    (*fp).tcon = (*work).tcon;
    (*fp).volatile_id = KSMBD_NO_FID as u64;
    (*fp).persistent_id = KSMBD_NO_FID as u64;
    (*fp).f_state = FP_NEW as c_uint;
    (*fp).f_ci = ksmbd_inode_get(fp);

    let ret = if (*fp).f_ci.is_null() {
        -(ENOMEM as c_int)
    } else {
        let ret = __open_id(
            addr_of_mut!((*(*work).sess).file_table),
            fp,
            OPEN_ID_TYPE_VOLATILE_ID,
        );
        if ret == 0 {
            rvc_atomic_inc(addr_of_mut!((*(*work).conn).stats.open_files_count));
            return fp;
        }
        ksmbd_inode_put((*fp).f_ci);
        ret
    };

    ksmbd_conn_put((*fp).conn);
    kmem_cache_free(filp_cache, fp.cast());
    rvc_err_ptr(ret as c_long).cast()
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_update_fstate(
    ft: *mut ksmbd_file_table,
    fp: *mut ksmbd_file,
    state: c_uint,
) -> c_int {
    if fp.is_null() {
        return -(ENOENT as c_int);
    }

    rvc_write_lock(addr_of_mut!((*ft).lock));
    let ret = if state == FP_INITED as c_uint
        && ((*fp).f_state != FP_NEW as c_uint || !has_file_id((*fp).volatile_id))
    {
        -(ENOENT as c_int)
    } else {
        (*fp).f_state = state;
        0
    };
    rvc_write_unlock(addr_of_mut!((*ft).lock));
    ret
}

// Called with the publishing table's lock held. An FP_NEW opener retains its
// original reference until its attempted FP_INITED transition sees no ID.
unsafe fn ksmbd_mark_fp_closed(fp: *mut ksmbd_file) -> c_int {
    if (*fp).f_state == FP_INITED as c_uint {
        set_close_state_blocked_works(fp);
        (*fp).f_state = FP_CLOSED as c_uint;
        return 2;
    }
    1
}

unsafe fn __close_file_table_ids(
    sess: *mut ksmbd_session,
    tcon: *mut ksmbd_tree_connect,
    skip: unsafe fn(*mut ksmbd_tree_connect, *mut ksmbd_file, *mut ksmbd_user) -> bool,
    skip_preserves_fp: bool,
) -> c_int {
    let ft = addr_of_mut!((*sess).file_table);
    let mut id: c_uint = 0;
    let mut num: c_int = 0;

    loop {
        rvc_write_lock(addr_of_mut!((*ft).lock));
        let fp = idr_get_next((*ft).idr, addr_of_mut!(id).cast::<c_int>())
            .cast::<ksmbd_file>();
        if fp.is_null() {
            rvc_write_unlock(addr_of_mut!((*ft).lock));
            break;
        }
        if !rvc_atomic_inc_not_zero(addr_of_mut!((*fp).refcount)) {
            id = id.wrapping_add(1);
            rvc_write_unlock(addr_of_mut!((*ft).lock));
            continue;
        }

        let n_to_drop;
        if skip_preserves_fp {
            // Session preservation can sleep and rewrites per-connection
            // fields, so remove lookup visibility before dropping ft->lock.
            idr_remove((*ft).idr, id as c_ulong);
            (*fp).durable_volatile_id = (*fp).volatile_id;
            (*fp).volatile_id = KSMBD_NO_FID as u64;
            rvc_write_unlock(addr_of_mut!((*ft).lock));

            if skip(tcon, fp, (*sess).user) {
                // The old table reference becomes the durable lifetime ref.
                rvc_atomic_dec(addr_of_mut!((*fp).refcount));
                id = id.wrapping_add(1);
                continue;
            }

            // Synchronize with an in-flight FP_NEW opener's state update.
            rvc_write_lock(addr_of_mut!((*ft).lock));
            n_to_drop = ksmbd_mark_fp_closed(fp);
            rvc_write_unlock(addr_of_mut!((*ft).lock));
        } else {
            // Tree comparison cannot sleep or mutate fp, so keep comparison,
            // unpublication, and the close-state transition under one lock.
            if skip(tcon, fp, (*sess).user) {
                rvc_atomic_dec(addr_of_mut!((*fp).refcount));
                rvc_write_unlock(addr_of_mut!((*ft).lock));
                id = id.wrapping_add(1);
                continue;
            }
            idr_remove((*ft).idr, id as c_ulong);
            (*fp).volatile_id = KSMBD_NO_FID as u64;
            n_to_drop = ksmbd_mark_fp_closed(fp);
            rvc_write_unlock(addr_of_mut!((*ft).lock));
        }

        // Final close skips this unlink once volatile_id has been cleared.
        down_write(addr_of_mut!((*(*fp).f_ci).m_lock));
        rvc_list_del_init(addr_of_mut!((*fp).node));
        up_write(addr_of_mut!((*(*fp).f_ci).m_lock));

        // Count only closes finalized here. A remaining holder's final put
        // accounts for its own connection open_files_count decrement.
        if rvc_atomic_sub_and_test(n_to_drop, addr_of_mut!((*fp).refcount)) {
            __ksmbd_close_fd(null_mut(), fp);
            num = num.wrapping_add(1);
        }
        id = id.wrapping_add(1);
    }
    num
}

unsafe fn is_reconnectable(fp: *mut ksmbd_file) -> bool {
    let opinfo = opinfo_get(fp);
    if opinfo.is_null() {
        return false;
    }
    if (*opinfo).op_state != OPLOCK_STATE_NONE as c_int {
        opinfo_put(opinfo);
        return false;
    }

    let reconn = if (*fp).is_resilient || (*fp).is_persistent {
        true
    } else if (*fp).is_durable
        && (*opinfo).is_lease
        && ((*(*opinfo).o_lease).state & RVC_LEASE_HANDLE_LE) != 0
    {
        true
    } else {
        (*fp).is_durable && (*opinfo).level == SMB2_OPLOCK_LEVEL_BATCH as c_int
    };
    opinfo_put(opinfo);
    reconn
}

unsafe fn tree_conn_fd_check(
    tcon: *mut ksmbd_tree_connect,
    fp: *mut ksmbd_file,
    _user: *mut ksmbd_user,
) -> bool {
    (*fp).tcon != tcon
}

// The wait-event macro calls this function through a C function pointer.
unsafe extern "C" fn ksmbd_durable_scavenger_alive() -> bool {
    if !durable_scavenger_running {
        return false;
    }
    if kthread_should_stop() {
        return false;
    }
    if rvc_idr_is_empty(global_ft.idr) {
        return false;
    }
    true
}

unsafe fn ksmbd_scavenger_dispose_dh(fp: *mut ksmbd_file) {
    // The node can still belong to the inode's share-mode list; never reuse
    // it as a private scavenger-list node.
    down_write(addr_of_mut!((*(*fp).f_ci).m_lock));
    rvc_list_del_init(addr_of_mut!((*fp).node));
    up_write(addr_of_mut!((*(*fp).f_ci).m_lock));

    // Release the durable lifetime and scavenger transient references. Any
    // inode-list lookup that raced the unlink may own the eventual final put.
    if rvc_atomic_sub_and_test(2, addr_of_mut!((*fp).refcount)) {
        __ksmbd_close_fd(null_mut(), fp);
    }
}

unsafe extern "C" fn ksmbd_durable_scavenger(_dummy: *mut c_void) -> c_int {
    let mut min_timeout: c_uint = 1;
    rvc_module_get();
    rvc_set_freezable();

    while ksmbd_durable_scavenger_alive() {
        if rvc_try_to_freeze() {
            continue;
        }

        let remaining_jiffies = rvc_wait_timeout(
            addr_of_mut!(dh_wq),
            Some(ksmbd_durable_scavenger_alive),
            rvc_msecs_to_jiffies(min_timeout) as c_long,
        ) as c_ulong;
        if (remaining_jiffies as c_long) > 0 {
            min_timeout = rvc_jiffies_to_msecs(remaining_jiffies);
        } else {
            min_timeout = DURABLE_HANDLE_MAX_TIMEOUT as c_uint;
        }

        let mut found_fp_timeout;
        loop {
            let mut expired_fp: *mut ksmbd_file = null_mut();
            found_fp_timeout = false;
            rvc_write_lock(addr_of_mut!(global_ft.lock));
            let mut id: c_uint = 0;
            loop {
                let fp = idr_get_next(global_ft.idr, addr_of_mut!(id).cast::<c_int>())
                    .cast::<ksmbd_file>();
                if fp.is_null() {
                    break;
                }
                if (*fp).durable_timeout == 0 {
                    id = id.wrapping_add(1);
                    continue;
                }
                if rvc_atomic_read(addr_of!((*fp).refcount)) > 1 || !(*fp).conn.is_null() {
                    id = id.wrapping_add(1);
                    continue;
                }

                found_fp_timeout = true;
                if (*fp).durable_scavenger_timeout <= rvc_jiffies_to_msecs(rvc_jiffies()) {
                    __ksmbd_remove_durable_fd(fp);
                    // Hold fp until it is unlinked from the inode list after
                    // dropping global_ft.lock, even if a lookup races us.
                    rvc_atomic_inc(addr_of_mut!((*fp).refcount));
                    expired_fp = fp;
                    break;
                }

                // Both operands in C are unsigned int before widening.
                let durable_timeout = (*fp)
                    .durable_scavenger_timeout
                    .wrapping_sub(rvc_jiffies_to_msecs(rvc_jiffies()))
                    as c_ulong;
                if (min_timeout as c_ulong) > durable_timeout {
                    min_timeout = durable_timeout as c_uint;
                }
                id = id.wrapping_add(1);
            }
            rvc_write_unlock(addr_of_mut!(global_ft.lock));

            if expired_fp.is_null() {
                break;
            }
            ksmbd_scavenger_dispose_dh(expired_fp);
        }

        if !found_fp_timeout {
            break;
        }
    }

    durable_scavenger_running = false;
    rvc_module_put();
    0
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_launch_ksmbd_durable_scavenger() {
    if (server_conf.flags & RVC_DURABLE_FLAG) == 0 {
        return;
    }

    rvc_mutex_lock(addr_of_mut!(rvc_durable_scavenger_lock));
    if durable_scavenger_running {
        rvc_mutex_unlock(addr_of_mut!(rvc_durable_scavenger_lock));
        return;
    }
    durable_scavenger_running = true;
    server_conf.dh_task = rvc_kthread_run(
        Some(ksmbd_durable_scavenger),
        null_mut(),
        b"ksmbd-durable-scavenger\0".as_ptr().cast(),
    );
    if rvc_is_err(server_conf.dh_task.cast()) {
        rvc_err_thread(rvc_ptr_err(server_conf.dh_task.cast()));
        server_conf.dh_task = null_mut();
        durable_scavenger_running = false;
    }
    rvc_mutex_unlock(addr_of_mut!(rvc_durable_scavenger_lock));
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_stop_durable_scavenger() {
    if (server_conf.flags & RVC_DURABLE_FLAG) == 0 {
        return;
    }

    rvc_mutex_lock(addr_of_mut!(rvc_durable_scavenger_lock));
    if !durable_scavenger_running {
        rvc_mutex_unlock(addr_of_mut!(rvc_durable_scavenger_lock));
        return;
    }
    durable_scavenger_running = false;
    if rvc_wait_active(addr_of_mut!(dh_wq)) {
        rvc_wake_up(addr_of_mut!(dh_wq));
    }
    rvc_mutex_unlock(addr_of_mut!(rvc_durable_scavenger_lock));
    kthread_stop(server_conf.dh_task);
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_vfs_set_durable_owner(
    fp: *mut ksmbd_file,
    user: *mut ksmbd_user,
) -> c_int {
    if user.is_null() {
        return -(EINVAL as c_int);
    }
    let name = kstrdup((*user).name, RVC_GFP_KERNEL);
    if name.is_null() {
        return -(ENOMEM as c_int);
    }

    rvc_spin_lock(addr_of_mut!((*fp).f_lock));
    let old_name = (*fp).owner.name;
    (*fp).owner.uid = (*user).uid;
    (*fp).owner.gid = (*user).gid;
    (*fp).owner.name = name;
    rvc_spin_unlock(addr_of_mut!((*fp).f_lock));
    kfree(old_name.cast());
    0
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_vfs_compare_durable_owner(
    fp: *mut ksmbd_file,
    user: *mut ksmbd_user,
) -> bool {
    if user.is_null() {
        return false;
    }

    rvc_spin_lock(addr_of_mut!((*fp).f_lock));
    let ret = !(*fp).owner.name.is_null()
        && (*fp).owner.uid == (*user).uid
        && (*fp).owner.gid == (*user).gid
        && strcmp((*fp).owner.name, (*user).name) == 0;
    rvc_spin_unlock(addr_of_mut!((*fp).f_lock));
    ret
}

unsafe fn session_fd_check(
    _tcon: *mut ksmbd_tree_connect,
    fp: *mut ksmbd_file,
    user: *mut ksmbd_user,
) -> bool {
    if !is_reconnectable(fp) {
        return false;
    }
    if (*fp).f_state != FP_INITED as c_uint {
        return false;
    }
    if rvc_warn_missing_conn((*fp).conn.is_null()) {
        return false;
    }
    if ksmbd_vfs_set_durable_owner(fp, user) != 0 {
        return false;
    }

    // fp owns a strong conn reference throughout the oplock and lock puts.
    let conn = (*fp).conn;
    let ci = (*fp).f_ci;
    down_write(addr_of_mut!((*ci).m_lock));
    rvc_session_op_list_check(ci);
    let op_head = addr_of_mut!((*ci).m_op_list);
    let mut op_node = rvc_rcu_list_next(op_head);
    while op_node != op_head {
        let op = op_node.cast::<u8>().sub(offset_of!(oplock_info, op_entry))
            .cast::<oplock_info>();
        if (*op).conn == conn {
            ksmbd_conn_put((*op).conn);
            (*op).conn = null_mut();
            (*op).sess = null_mut();
        }
        op_node = rvc_rcu_list_next(op_node);
    }
    up_write(addr_of_mut!((*ci).m_lock));

    let lock_head = addr_of_mut!((*fp).lock_list);
    let mut lock_node = (*lock_head).next;
    while lock_node != lock_head {
        // Save next before processing, matching list_for_each_entry_safe.
        let next_node = (*lock_node).next;
        let smb_lock = lock_node.cast::<u8>().sub(offset_of!(ksmbd_lock, flist))
            .cast::<ksmbd_lock>();
        let lock_conn = (*smb_lock).conn;
        if !lock_conn.is_null() {
            rvc_spin_lock(addr_of_mut!((*lock_conn).llist_lock));
            rvc_list_del_init(addr_of_mut!((*smb_lock).clist));
            (*smb_lock).conn = null_mut();
            rvc_spin_unlock(addr_of_mut!((*lock_conn).llist_lock));
            ksmbd_conn_put(lock_conn);
        }
        lock_node = next_node;
    }

    (*fp).conn = null_mut();
    (*fp).tcon = null_mut();
    (*fp).volatile_id = KSMBD_NO_FID as u64;
    if (*fp).durable_timeout != 0 {
        (*fp).durable_scavenger_timeout = rvc_jiffies_to_msecs(rvc_jiffies())
            .wrapping_add((*fp).durable_timeout);
    }
    ksmbd_conn_put(conn);
    true
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_close_tree_conn_fds(work: *mut ksmbd_work) {
    let num = __close_file_table_ids((*work).sess, (*work).tcon, tree_conn_fd_check, false);
    rvc_atomic_sub(num, addr_of_mut!((*(*work).conn).stats.open_files_count));
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_close_session_fds(work: *mut ksmbd_work) {
    let num = __close_file_table_ids((*work).sess, (*work).tcon, session_fd_check, true);
    rvc_atomic_sub(num, addr_of_mut!((*(*work).conn).stats.open_files_count));
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_init_global_file_table() -> c_int {
    if create_proc_files() != 0 {
        rvc_warn_proc();
    }
    ksmbd_init_file_table(addr_of_mut!(global_ft))
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_free_global_file_table() {
    let mut id: c_uint = 0;
    loop {
        let fp = idr_get_next(global_ft.idr, addr_of_mut!(id).cast::<c_int>())
            .cast::<ksmbd_file>();
        if fp.is_null() {
            break;
        }
        ksmbd_remove_durable_fd(fp);
        __ksmbd_close_fd(null_mut(), fp);
        id = id.wrapping_add(1);
    }
    idr_destroy(global_ft.idr);
    kfree(global_ft.idr.cast());
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_validate_name_reconnect(
    share: *mut ksmbd_share_config,
    fp: *mut ksmbd_file,
    name: *mut c_char,
) -> c_int {
    let pathname = rvc_kmalloc(PATH_MAX as usize, RVC_DEFAULT_GFP).cast::<c_char>();
    if pathname.is_null() {
        return -(EACCES as c_int);
    }
    let ab_pathname = d_path(file_path((*fp).filp), pathname, PATH_MAX as c_int);
    if rvc_is_err(ab_pathname.cast()) {
        kfree(pathname.cast());
        return -(EACCES as c_int);
    }
    let mut ret = 0;
    if !name.is_null()
        && strcmp(ab_pathname.add((*share).path_sz.wrapping_add(1) as usize), name) != 0
    {
        rvc_debug_reconnect(name);
        ret = -(EINVAL as c_int);
    }
    kfree(pathname.cast());
    ret
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_reopen_durable_fd(
    work: *mut ksmbd_work,
    fp: *mut ksmbd_file,
) -> c_int {
    let conn = (*work).conn;
    rvc_write_lock(addr_of_mut!(global_ft.lock));
    if (!(*fp).is_durable && !(*fp).is_persistent)
        || !(*fp).conn.is_null()
        || !(*fp).tcon.is_null()
    {
        rvc_write_unlock(addr_of_mut!(global_ft.lock));
        rvc_err_durable((*fp).conn.cast(), (*fp).tcon.cast());
        return -(EBADF as c_int);
    }
    if has_file_id((*fp).volatile_id) {
        rvc_write_unlock(addr_of_mut!(global_ft.lock));
        rvc_err_in_use((*fp).volatile_id);
        return -(EBADF as c_int);
    }

    // Establish the strong connection binding before publishing the new ID.
    (*fp).conn = ksmbd_conn_get(conn);
    (*fp).tcon = (*work).tcon;
    rvc_write_unlock(addr_of_mut!(global_ft.lock));

    let old_f_state = (*fp).f_state;
    (*fp).f_state = FP_NEW as c_uint;
    __open_id(
        addr_of_mut!((*(*work).sess).file_table),
        fp,
        OPEN_ID_TYPE_VOLATILE_ID,
    );
    if !has_file_id((*fp).volatile_id) {
        rvc_write_lock(addr_of_mut!(global_ft.lock));
        (*fp).conn = null_mut();
        (*fp).tcon = null_mut();
        rvc_write_unlock(addr_of_mut!(global_ft.lock));
        ksmbd_conn_put(conn);
        (*fp).f_state = old_f_state;
        return -(EBADF as c_int);
    }

    let lock_head = addr_of_mut!((*fp).lock_list);
    let mut lock_node = (*lock_head).next;
    while lock_node != lock_head {
        let smb_lock = lock_node.cast::<u8>().sub(offset_of!(ksmbd_lock, flist))
            .cast::<ksmbd_lock>();
        (*smb_lock).conn = ksmbd_conn_get(conn);
        rvc_spin_lock(addr_of_mut!((*conn).llist_lock));
        rvc_list_add_tail(addr_of_mut!((*smb_lock).clist), addr_of_mut!((*conn).lock_list));
        rvc_spin_unlock(addr_of_mut!((*conn).llist_lock));
        lock_node = (*lock_node).next;
    }

    let ci = (*fp).f_ci;
    down_write(addr_of_mut!((*ci).m_lock));
    rvc_reopen_op_list_check(ci);
    let op_head = addr_of_mut!((*ci).m_op_list);
    let mut op_node = rvc_rcu_list_next(op_head);
    while op_node != op_head {
        let op = op_node.cast::<u8>().sub(offset_of!(oplock_info, op_entry))
            .cast::<oplock_info>();
        if (*op).conn.is_null() && (*op).o_fp == fp {
            (*op).conn = ksmbd_conn_get((*fp).conn);
            (*op).sess = (*work).sess;
        }
        op_node = rvc_rcu_list_next(op_node);
    }
    up_write(addr_of_mut!((*ci).m_lock));

    rvc_spin_lock(addr_of_mut!((*fp).f_lock));
    (*fp).owner.gid = 0;
    (*fp).owner.uid = 0;
    kfree((*fp).owner.name.cast());
    (*fp).owner.name = null_mut();
    rvc_spin_unlock(addr_of_mut!((*fp).f_lock));
    0
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_init_file_table(ft: *mut ksmbd_file_table) -> c_int {
    (*ft).idr = rvc_kzalloc(size_of::<idr>(), RVC_DEFAULT_GFP).cast();
    if (*ft).idr.is_null() {
        return -(ENOMEM as c_int);
    }
    rvc_idr_init((*ft).idr);
    rvc_rwlock_init(addr_of_mut!((*ft).lock));
    0
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_destroy_file_table(sess: *mut ksmbd_session) {
    let ft = addr_of_mut!((*sess).file_table);
    if (*ft).idr.is_null() {
        return;
    }
    __close_file_table_ids(sess, null_mut(), session_fd_check, true);
    idr_destroy((*ft).idr);
    kfree((*ft).idr.cast());
    (*ft).idr = null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_init_file_cache() -> c_int {
    filp_cache = rvc_cache_create(
        b"ksmbd_file_cache\0".as_ptr().cast(),
        size_of::<ksmbd_file>() as c_uint,
        0,
        RVC_SLAB_HWCACHE_ALIGN,
    );
    if filp_cache.is_null() {
        rvc_err_cache();
        return -(ENOMEM as c_int);
    }
    rvc_wait_init(addr_of_mut!(dh_wq));
    0
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_exit_file_cache() {
    kmem_cache_destroy(filp_cache);
}
