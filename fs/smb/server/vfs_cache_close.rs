// SPDX-License-Identifier: GPL-2.0-or-later
// Rust translation of vfs_cache.c: durable removal, close, and lookup paths.

unsafe fn __ksmbd_remove_durable_fd(fp: *mut ksmbd_file) {
    if !has_file_id((*fp).persistent_id) {
        return;
    }

    idr_remove(global_ft.idr, (*fp).persistent_id as c_ulong);
    // A delayed final put must not remove a newly reused persistent ID.
    (*fp).persistent_id = KSMBD_NO_FID as u64;
}

unsafe fn ksmbd_remove_durable_fd(fp: *mut ksmbd_file) {
    rvc_write_lock(addr_of_mut!(global_ft.lock));
    __ksmbd_remove_durable_fd(fp);
    rvc_write_unlock(addr_of_mut!(global_ft.lock));
    if rvc_wait_active(addr_of_mut!(dh_wq)) {
        rvc_wake_up(addr_of_mut!(dh_wq));
    }
}

unsafe fn __ksmbd_remove_fd(ft: *mut ksmbd_file_table, fp: *mut ksmbd_file) {
    down_write(addr_of_mut!((*(*fp).f_ci).m_lock));
    rvc_list_del_init(addr_of_mut!((*fp).node));
    up_write(addr_of_mut!((*(*fp).f_ci).m_lock));

    if !has_file_id((*fp).volatile_id) {
        return;
    }

    rvc_write_lock(addr_of_mut!((*ft).lock));
    idr_remove((*ft).idr, (*fp).volatile_id as c_ulong);
    rvc_write_unlock(addr_of_mut!((*ft).lock));
}

unsafe fn __ksmbd_close_fd(ft: *mut ksmbd_file_table, fp: *mut ksmbd_file) {
    fd_limit_close();
    ksmbd_remove_durable_fd(fp);
    if !ft.is_null() {
        __ksmbd_remove_fd(ft, fp);
    }

    close_id_del_oplock(fp);
    let filp = (*fp).filp;

    __ksmbd_inode_close(fp);
    if !rvc_is_err_or_null(filp.cast()) {
        fput(filp);
    }

    // The zero reference count serializes this list. VFS requests blocked
    // below its locks still need to be released before the locks are freed.
    let lock_head = addr_of_mut!((*fp).lock_list);
    let mut lock_node = (*lock_head).next;
    while lock_node != lock_head {
        let next = (*lock_node).next;
        let smb_lock = lock_node
            .byte_sub(offset_of!(ksmbd_lock, flist))
            .cast::<ksmbd_lock>();
        let conn = (*smb_lock).conn;

        if !conn.is_null() {
            rvc_spin_lock(addr_of_mut!((*conn).llist_lock));
            rvc_list_del_init(addr_of_mut!((*smb_lock).clist));
            (*smb_lock).conn = null_mut();
            rvc_spin_unlock(addr_of_mut!((*conn).llist_lock));
            ksmbd_conn_put(conn);
        }

        rvc_list_del_init(addr_of_mut!((*smb_lock).flist));
        ksmbd_vfs_posix_lock_unblock((*smb_lock).fl);
        locks_free_lock((*smb_lock).fl);
        kfree(smb_lock.cast());
        lock_node = next;
    }

    // Claim one CHANGE_NOTIFY under f_lock, leaving its entry self-linked
    // so a concurrent CANCEL recognizes that it lost the claim. Sending can
    // sleep, and therefore runs only after releasing f_lock.
    loop {
        rvc_spin_lock(addr_of_mut!((*fp).f_lock));
        let pending = addr_of_mut!((*fp).notify_pendings);
        if rvc_list_empty(pending) {
            rvc_spin_unlock(addr_of_mut!((*fp).f_lock));
            break;
        }
        let cn_work = (*pending)
            .next
            .byte_sub(offset_of!(ksmbd_work, notify_entry))
            .cast::<ksmbd_work>();
        rvc_list_del_init(addr_of_mut!((*cn_work).notify_entry));
        rvc_spin_unlock(addr_of_mut!((*fp).f_lock));

        ksmbd_conn_write(cn_work);
        // Removes async_requests membership and releases cancel_argv and
        // async_id before the work structure is freed.
        release_async_work(cn_work);
        ksmbd_free_work_struct(cn_work);
    }

    // A disconnected durable handle has already dropped this connection
    // reference in session_fd_check(). Other handles still own it here.
    if !(*fp).conn.is_null() {
        ksmbd_conn_put((*fp).conn);
        (*fp).conn = null_mut();
    }

    if ksmbd_stream_fd(fp) {
        kfree((*fp).stream.name.cast());
    }
    kfree((*fp).owner.name.cast());
    kmem_cache_free(filp_cache, fp.cast());
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_close_disconnected_durable_delete_on_close(
    dentry: *mut dentry,
) -> bool {
    let mut dispose = list_head {
        next: null_mut(),
        prev: null_mut(),
    };
    let dispose_head = addr_of_mut!(dispose);
    rvc_list_init(dispose_head);
    let mut closed = false;

    let ci = ksmbd_inode_lookup_lock(dentry);
    if ci.is_null() {
        return false;
    }

    down_write(addr_of_mut!((*ci).m_lock));
    if (*ci).m_flags & (S_DEL_ON_CLS | S_DEL_ON_CLS_STREAM | S_DEL_PENDING) != 0 {
        let head = addr_of_mut!((*ci).m_fp_list);
        let mut node = (*head).next;
        while node != head {
            let next = (*node).next;
            let fp = node
                .byte_sub(offset_of!(ksmbd_file, node))
                .cast::<ksmbd_file>();
            if (*fp).conn.is_null()
                && (*fp).is_durable
                && (*fp).f_state == FP_INITED as c_uint
            {
                // Only the durable lifetime reference may remain. Take a
                // transient reference before claiming and detaching it.
                rvc_write_lock(addr_of_mut!(global_ft.lock));
                if rvc_atomic_read(addr_of!((*fp).refcount)) == 1 {
                    rvc_atomic_inc(addr_of_mut!((*fp).refcount));
                    __ksmbd_remove_durable_fd(fp);
                    ksmbd_mark_fp_closed(fp);
                    rvc_list_move_tail(addr_of_mut!((*fp).node), dispose_head);
                }
                rvc_write_unlock(addr_of_mut!(global_ft.lock));
            }
            node = next;
        }
    }
    up_write(addr_of_mut!((*ci).m_lock));

    // The collected handles keep ci alive. Drop the lookup reference first
    // so the last handle close can promote delete-pending and unlink.
    ksmbd_inode_put(ci);

    while !rvc_list_empty(dispose_head) {
        let fp = (*dispose_head)
            .next
            .byte_sub(offset_of!(ksmbd_file, node))
            .cast::<ksmbd_file>();
        rvc_list_del_init(addr_of_mut!((*fp).node));
        if rvc_atomic_sub_and_test(2, addr_of_mut!((*fp).refcount)) {
            __ksmbd_close_fd(null_mut(), fp);
            closed = true;
        }
    }

    closed
}

unsafe fn ksmbd_fp_get(fp: *mut ksmbd_file) -> *mut ksmbd_file {
    if (*fp).f_state != FP_INITED as c_uint {
        return null_mut();
    }
    if !rvc_atomic_inc_not_zero(addr_of_mut!((*fp).refcount)) {
        return null_mut();
    }
    fp
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_file_get(fp: *mut ksmbd_file) -> *mut ksmbd_file {
    ksmbd_fp_get(fp)
}

unsafe fn __ksmbd_lookup_fd(ft: *mut ksmbd_file_table, id: u64) -> *mut ksmbd_file {
    if !has_file_id(id) {
        return null_mut();
    }

    rvc_read_lock(addr_of_mut!((*ft).lock));
    let mut fp = idr_find((*ft).idr, id as c_ulong).cast::<ksmbd_file>();
    if !fp.is_null() {
        fp = ksmbd_fp_get(fp);
    }
    rvc_read_unlock(addr_of_mut!((*ft).lock));
    fp
}

unsafe fn __put_fd_final(work: *mut ksmbd_work, fp: *mut ksmbd_file) {
    // Detached durable handles are no longer counted by any connection.
    // A final put from an unrelated inode walker must not decrement its
    // connection's open-file counter or use its file table.
    if (*fp).conn.is_null() {
        __ksmbd_close_fd(null_mut(), fp);
        return;
    }
    __ksmbd_close_fd(addr_of_mut!((*(*work).sess).file_table), fp);
    rvc_atomic_dec(addr_of_mut!((*(*work).conn).stats.open_files_count));
}

unsafe fn set_close_state_blocked_works(fp: *mut ksmbd_file) {
    rvc_spin_lock(addr_of_mut!((*fp).f_lock));
    let head = addr_of_mut!((*fp).blocked_works);
    let mut node = (*head).next;
    while node != head {
        let cancel_work = node
            .byte_sub(offset_of!(ksmbd_work, fp_entry))
            .cast::<ksmbd_work>();
        (*cancel_work).state = KSMBD_WORK_CLOSED as u8;
        // The C work-list contract requires a valid cancellation callback.
        ((*cancel_work).cancel_fn.unwrap_unchecked())((*cancel_work).cancel_argv);
        node = (*node).next;
    }
    rvc_spin_unlock(addr_of_mut!((*fp).f_lock));
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_close_fd(work: *mut ksmbd_work, id: u64) -> c_int {
    let mut closed = false;
    if !has_file_id(id) {
        return 0;
    }

    let ft = addr_of_mut!((*(*work).sess).file_table);
    rvc_write_lock(addr_of_mut!((*ft).lock));
    let mut fp = idr_find((*ft).idr, id as c_ulong).cast::<ksmbd_file>();
    if !fp.is_null() {
        set_close_state_blocked_works(fp);

        if (*fp).f_state != FP_INITED as c_uint {
            fp = null_mut();
        } else {
            (*fp).f_state = FP_CLOSED as c_uint;
            idr_remove((*ft).idr, id as c_ulong);
            (*fp).volatile_id = KSMBD_NO_FID as u64;
            closed = true;
            if !rvc_atomic_dec_and_test(addr_of_mut!((*fp).refcount)) {
                fp = null_mut();
            }
        }
    }
    rvc_write_unlock(addr_of_mut!((*ft).lock));

    if fp.is_null() {
        return if closed { 0 } else { -(EINVAL as c_int) };
    }

    __put_fd_final(work, fp);
    0
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_fd_put(work: *mut ksmbd_work, fp: *mut ksmbd_file) {
    if fp.is_null() {
        return;
    }
    if !rvc_atomic_dec_and_test(addr_of_mut!((*fp).refcount)) {
        return;
    }
    __put_fd_final(work, fp);
}

unsafe fn __sanity_check(tcon: *mut ksmbd_tree_connect, fp: *mut ksmbd_file) -> bool {
    if fp.is_null() {
        return false;
    }
    if (*fp).tcon != tcon {
        return false;
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_lookup_foreign_fd(
    work: *mut ksmbd_work,
    id: u64,
) -> *mut ksmbd_file {
    __ksmbd_lookup_fd(addr_of_mut!((*(*work).sess).file_table), id)
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_lookup_fd_fast(
    work: *mut ksmbd_work,
    id: u64,
) -> *mut ksmbd_file {
    let fp = __ksmbd_lookup_fd(addr_of_mut!((*(*work).sess).file_table), id);
    if __sanity_check((*work).tcon, fp) {
        return fp;
    }
    ksmbd_fd_put(work, fp);
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_lookup_fd_slow(
    work: *mut ksmbd_work,
    mut id: u64,
    mut pid: u64,
) -> *mut ksmbd_file {
    if !has_file_id(id) {
        id = (*work).compound_fid;
        pid = (*work).compound_pfid;
    }

    let fp = __ksmbd_lookup_fd(addr_of_mut!((*(*work).sess).file_table), id);
    if !__sanity_check((*work).tcon, fp) {
        ksmbd_fd_put(work, fp);
        return null_mut();
    }
    if (*fp).persistent_id != pid {
        ksmbd_fd_put(work, fp);
        return null_mut();
    }
    fp
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_lookup_global_fd(id: u64) -> *mut ksmbd_file {
    __ksmbd_lookup_fd(addr_of_mut!(global_ft), id)
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_lookup_durable_fd(id: u64) -> *mut ksmbd_file {
    let mut fp = __ksmbd_lookup_fd(addr_of_mut!(global_ft), id);
    if !fp.is_null()
        && ((*fp).durable_reconnect_disabled
            || !(*fp).conn.is_null()
            || ((*fp).durable_scavenger_timeout != 0
                && (*fp).durable_scavenger_timeout < rvc_jiffies_to_msecs(rvc_jiffies())))
    {
        ksmbd_put_durable_fd(fp);
        fp = null_mut();
    }
    fp
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_put_durable_fd(fp: *mut ksmbd_file) {
    if !rvc_atomic_dec_and_test(addr_of_mut!((*fp).refcount)) {
        return;
    }
    __ksmbd_close_fd(null_mut(), fp);
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_has_other_active_fd(fp: *mut ksmbd_file) -> bool {
    let ci = (*fp).f_ci;
    let mut ret = false;

    down_read(addr_of_mut!((*ci).m_lock));
    let head = addr_of_mut!((*ci).m_fp_list);
    let mut node = (*head).next;
    while node != head {
        let lfp = node
            .byte_sub(offset_of!(ksmbd_file, node))
            .cast::<ksmbd_file>();
        if lfp != fp
            && (*lfp).f_state == FP_INITED as c_uint
            && (!rvc_read_conn(lfp).is_null() || !rvc_read_tcon(lfp).is_null())
        {
            ret = true;
            break;
        }
        node = (*node).next;
    }
    up_read(addr_of_mut!((*ci).m_lock));
    ret
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_lookup_fd_app_instance_id(
    app_instance_id: *mut c_char,
) -> *mut ksmbd_file {
    let mut id: u32 = 0;
    rvc_read_lock(addr_of_mut!(global_ft.lock));
    let mut fp;
    loop {
        fp = idr_get_next(global_ft.idr, &mut id as *mut u32 as *mut c_int)
            .cast::<ksmbd_file>();
        if fp.is_null() {
            break;
        }
        if (*fp).has_app_instance_id
            && memcmp(
                addr_of!((*fp).app_instance_id).cast(),
                app_instance_id.cast(),
                SMB2_CREATE_GUID_SIZE as _,
            ) == 0
        {
            fp = ksmbd_fp_get(fp);
            break;
        }
        id = id.wrapping_add(1);
    }
    rvc_read_unlock(addr_of_mut!(global_ft.lock));
    fp
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_close_fd_app_instance_id(
    app_instance_id: *mut c_char,
) -> c_int {
    let fp = ksmbd_lookup_fd_app_instance_id(app_instance_id);
    if fp.is_null() {
        return 0;
    }

    let opinfo = opinfo_get(fp);
    if opinfo.is_null() {
        ksmbd_put_durable_fd(fp);
        return 0;
    }

    down_read(addr_of_mut!((*(*fp).f_ci).m_lock));
    if (*opinfo).conn.is_null() {
        up_read(addr_of_mut!((*(*fp).f_ci).m_lock));
        opinfo_put(opinfo);
        ksmbd_put_durable_fd(fp);
        return 0;
    }

    let ft = addr_of_mut!((*(*opinfo).sess).file_table);
    let mut n_to_drop = 0;
    rvc_write_lock(addr_of_mut!((*ft).lock));
    if (*fp).f_state == FP_INITED as c_uint && has_file_id((*fp).volatile_id) {
        idr_remove((*ft).idr, (*fp).volatile_id as c_ulong);
        (*fp).volatile_id = KSMBD_NO_FID as u64;
        n_to_drop = ksmbd_mark_fp_closed(fp);
    }
    rvc_write_unlock(addr_of_mut!((*ft).lock));
    up_read(addr_of_mut!((*(*fp).f_ci).m_lock));
    opinfo_put(opinfo);

    if n_to_drop == 0 {
        ksmbd_put_durable_fd(fp);
        return 0;
    }

    down_write(addr_of_mut!((*(*fp).f_ci).m_lock));
    rvc_list_del_init(addr_of_mut!((*fp).node));
    up_write(addr_of_mut!((*(*fp).f_ci).m_lock));

    if rvc_atomic_sub_and_test(n_to_drop, addr_of_mut!((*fp).refcount)) {
        if !(*fp).conn.is_null() {
            rvc_atomic_dec(addr_of_mut!((*(*fp).conn).stats.open_files_count));
        }
        __ksmbd_close_fd(null_mut(), fp);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_invalidate_durable_fd(id: u64) -> c_int {
    let fp = ksmbd_lookup_global_fd(id);
    if fp.is_null() {
        return -(ENOENT as c_int);
    }

    (*fp).durable_reconnect_disabled = true;
    if !(*fp).conn.is_null() {
        ksmbd_put_durable_fd(fp);
        return -(ENOENT as c_int);
    }

    (*fp).durable_timeout = 1;
    (*fp).durable_scavenger_timeout = rvc_jiffies_to_msecs(rvc_jiffies());
    ksmbd_put_durable_fd(fp);
    if rvc_wait_active(addr_of_mut!(dh_wq)) {
        rvc_wake_up(addr_of_mut!(dh_wq));
    }
    -(ENOENT as c_int)
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_lookup_fd_cguid(cguid: *mut c_char) -> *mut ksmbd_file {
    let mut id: u32 = 0;
    rvc_read_lock(addr_of_mut!(global_ft.lock));
    let mut fp;
    loop {
        fp = idr_get_next(global_ft.idr, &mut id as *mut u32 as *mut c_int)
            .cast::<ksmbd_file>();
        if fp.is_null() {
            break;
        }
        if memcmp(
            addr_of!((*fp).create_guid).cast(),
            cguid.cast(),
            SMB2_CREATE_GUID_SIZE as _,
        ) == 0
        {
            fp = ksmbd_fp_get(fp);
            break;
        }
        id = id.wrapping_add(1);
    }
    rvc_read_unlock(addr_of_mut!(global_ft.lock));
    fp
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_lookup_fd_inode(dentry: *mut dentry) -> *mut ksmbd_file {
    let inode = rvc_d_inode(dentry);

    rvc_read_lock(addr_of_mut!(rvc_inode_hash_lock));
    let ci = __ksmbd_inode_lookup(dentry);
    rvc_read_unlock(addr_of_mut!(rvc_inode_hash_lock));
    if ci.is_null() {
        return null_mut();
    }

    down_read(addr_of_mut!((*ci).m_lock));
    let head = addr_of_mut!((*ci).m_fp_list);
    let mut node = (*head).next;
    while node != head {
        let mut lfp = node
            .byte_sub(offset_of!(ksmbd_file, node))
            .cast::<ksmbd_file>();
        if inode == rvc_file_inode((*lfp).filp) {
            lfp = ksmbd_fp_get(lfp);
            up_read(addr_of_mut!((*ci).m_lock));
            ksmbd_inode_put(ci);
            return lfp;
        }
        node = (*node).next;
    }
    up_read(addr_of_mut!((*ci).m_lock));
    ksmbd_inode_put(ci);
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_has_other_nonposix_open(dentry: *mut dentry) -> bool {
    let inode = rvc_d_inode(dentry);
    if inode.is_null() {
        return false;
    }

    let mut id: u32 = 0;
    let mut ret = false;
    rvc_read_lock(addr_of_mut!(global_ft.lock));
    loop {
        let fp = idr_get_next(global_ft.idr, &mut id as *mut u32 as *mut c_int)
            .cast::<ksmbd_file>();
        if fp.is_null() {
            break;
        }
        if rvc_read_fstate(fp) == FP_INITED as c_uint
            && inode == rvc_file_inode((*fp).filp)
            && !(*fp).is_posix_ctxt
        {
            ret = true;
            break;
        }
        id = id.wrapping_add(1);
    }
    rvc_read_unlock(addr_of_mut!(global_ft.lock));
    ret
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_has_nonposix_open_child(old_fp: *mut ksmbd_file) -> bool {
    let dentry = file_dentry((*old_fp).filp);
    let mut id: u32 = 0;
    let mut ret = false;

    rvc_read_lock(addr_of_mut!(global_ft.lock));
    loop {
        let fp = idr_get_next(global_ft.idr, &mut id as *mut u32 as *mut c_int)
            .cast::<ksmbd_file>();
        if fp.is_null() {
            break;
        }
        let fp_dentry = file_dentry((*fp).filp);
        if (*fp).f_state == FP_INITED as c_uint
            && fp_dentry != dentry
            && !((*old_fp).is_posix_ctxt && (*fp).is_posix_ctxt)
            && is_subdir(fp_dentry, dentry)
        {
            ret = true;
            break;
        }
        id = id.wrapping_add(1);
    }
    rvc_read_unlock(addr_of_mut!(global_ft.lock));
    ret
}
