// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/nfs/unlink.c
 *
 * nfs sillydelete handling
 */

// C includes and local headers are supplied by the surrounding translation unit.

unsafe fn nfs_free_unlinkdata(data: *mut nfs_unlinkdata) {
    put_cred((*data).cred);
    kfree((*data).args.name.name as *mut core::ffi::c_void);
    kfree(data as *mut core::ffi::c_void);
}

unsafe fn nfs_async_unlink_done(task: *mut rpc_task, calldata: *mut core::ffi::c_void) {
    let data = calldata as *mut nfs_unlinkdata;
    let dir = d_inode((*(*data).dentry).d_parent);

    trace_nfs_sillyrename_unlink(data, (*task).tk_status);
    if !NFS_PROTO(dir).unlink_done(task, dir) {
        rpc_restart_call_prepare(task);
    }
}

unsafe fn nfs_async_unlink_release(calldata: *mut core::ffi::c_void) {
    let data = calldata as *mut nfs_unlinkdata;
    let dentry = (*data).dentry;
    let sb = (*dentry).d_sb;

    up_read_non_owner(&mut NFS_I(d_inode((*dentry).d_parent)).rmdir_sem);
    d_lookup_done(dentry);
    nfs_free_unlinkdata(data);
    dput(dentry);
    nfs_sb_deactive(sb);
}

unsafe fn nfs_unlink_prepare(task: *mut rpc_task, calldata: *mut core::ffi::c_void) {
    let data = calldata as *mut nfs_unlinkdata;
    let dir = d_inode((*(*data).dentry).d_parent);
    NFS_PROTO(dir).unlink_rpc_prepare(task, data);
}

static mut NFS_UNLINK_OPS: rpc_call_ops = rpc_call_ops {
    rpc_call_done: Some(nfs_async_unlink_done),
    rpc_release: Some(nfs_async_unlink_release),
    rpc_call_prepare: Some(nfs_unlink_prepare),
};

unsafe fn nfs_do_call_unlink(inode: *mut inode, data: *mut nfs_unlinkdata) {
    let mut msg = rpc_message {
        rpc_argp: &mut (*data).args,
        rpc_resp: &mut (*data).res,
        rpc_cred: (*data).cred,
    };
    let mut task_setup_data = rpc_task_setup {
        rpc_message: &mut msg,
        callback_ops: &NFS_UNLINK_OPS,
        callback_data: data as *mut core::ffi::c_void,
        workqueue: nfsiod_workqueue,
        flags: RPC_TASK_ASYNC | RPC_TASK_CRED_NOREF,
        ..core::mem::zeroed()
    };
    let dir = d_inode((*(*data).dentry).d_parent);

    if nfs_server_capable(inode, NFS_CAP_MOVEABLE) {
        task_setup_data.flags |= RPC_TASK_MOVEABLE;
    }
    nfs_sb_active((*dir).i_sb);
    (*data).args.fh = NFS_FH(dir);
    nfs_fattr_init((*data).res.dir_attr);
    NFS_PROTO(dir).unlink_setup(&mut msg, (*data).dentry, inode);
    task_setup_data.rpc_client = NFS_CLIENT(dir);
    let task = rpc_run_task(&mut task_setup_data);
    if !IS_ERR(task) {
        rpc_put_task_async(task);
    }
}

unsafe fn nfs_call_unlink(dentry: *mut dentry, inode: *mut inode, data: *mut nfs_unlinkdata) -> i32 {
    let dir = d_inode((*dentry).d_parent);
    let alias;
    down_read_non_owner(&mut NFS_I(dir).rmdir_sem);
    alias = d_alloc_parallel((*dentry).d_parent, &mut (*data).args.name);
    if IS_ERR(alias) {
        up_read_non_owner(&mut NFS_I(dir).rmdir_sem);
        return 0;
    }
    if !d_in_lookup(alias) {
        let ret;
        let mut devname_garbage: *mut core::ffi::c_void = core::ptr::null_mut();
        spin_lock(&mut (*alias).d_lock);
        if d_really_is_positive(alias)
            && !nfs_compare_fh(NFS_FH(inode), NFS_FH(d_inode(alias)))
            && ((*alias).d_flags & DCACHE_NFSFS_RENAMED) == 0
        {
            devname_garbage = (*alias).d_fsdata;
            (*alias).d_fsdata = data as *mut core::ffi::c_void;
            (*alias).d_flags |= DCACHE_NFSFS_RENAMED;
            ret = 1;
        } else { ret = 0; }
        spin_unlock(&mut (*alias).d_lock);
        dput(alias);
        up_read_non_owner(&mut NFS_I(dir).rmdir_sem);
        kfree(devname_garbage);
        return ret;
    }
    (*data).dentry = alias;
    nfs_do_call_unlink(inode, data);
    1
}

unsafe fn nfs_async_unlink(dentry: *mut dentry, name: *const qstr) -> i32 {
    let data = kzalloc_obj::<nfs_unlinkdata>();
    let mut status = -ENOMEM;
    let mut devname_garbage: *mut core::ffi::c_void = core::ptr::null_mut();
    if data.is_null() { return status; }
    (*data).args.name.name = kstrdup((*name).name, GFP_KERNEL);
    if (*data).args.name.name.is_null() { kfree(data as *mut _); return status; }
    (*data).args.name.len = (*name).len;
    (*data).cred = get_current_cred();
    (*data).res.dir_attr = &mut (*data).dir_attr;
    status = -EBUSY;
    spin_lock(&mut (*dentry).d_lock);
    if ((*dentry).d_flags & DCACHE_NFSFS_RENAMED) != 0 {
        spin_unlock(&mut (*dentry).d_lock);
        put_cred((*data).cred); kfree((*data).args.name.name as *mut _); kfree(data as *mut _);
        return status;
    }
    (*dentry).d_flags |= DCACHE_NFSFS_RENAMED;
    devname_garbage = (*dentry).d_fsdata;
    (*dentry).d_fsdata = data as *mut _;
    spin_unlock(&mut (*dentry).d_lock);
    kfree(devname_garbage);
    0
}

unsafe fn nfs_complete_unlink(dentry: *mut dentry, inode: *mut inode) {
    spin_lock(&mut (*dentry).d_lock);
    (*dentry).d_flags &= !DCACHE_NFSFS_RENAMED;
    let data = (*dentry).d_fsdata as *mut nfs_unlinkdata;
    (*dentry).d_fsdata = core::ptr::null_mut();
    spin_unlock(&mut (*dentry).d_lock);
    NFS_PROTO(inode).return_delegation(inode);
    if NFS_STALE(inode) || nfs_call_unlink(dentry, inode, data) == 0 { nfs_free_unlinkdata(data); }
}

unsafe fn nfs_cancel_async_unlink(dentry: *mut dentry) {
    spin_lock(&mut (*dentry).d_lock);
    if ((*dentry).d_flags & DCACHE_NFSFS_RENAMED) != 0 {
        let data = (*dentry).d_fsdata as *mut nfs_unlinkdata;
        (*dentry).d_flags &= !DCACHE_NFSFS_RENAMED;
        (*dentry).d_fsdata = core::ptr::null_mut();
        spin_unlock(&mut (*dentry).d_lock);
        nfs_free_unlinkdata(data);
        return;
    }
    spin_unlock(&mut (*dentry).d_lock);
}

unsafe fn nfs_async_rename_done(task: *mut rpc_task, calldata: *mut core::ffi::c_void) {
    let data = calldata as *mut nfs_renamedata;
    let old_dir = (*data).old_dir;
    let new_dir = (*data).new_dir;
    trace_nfs_async_rename_done(old_dir, (*data).old_dentry, new_dir, (*data).new_dentry, (*task).tk_status);
    if !NFS_PROTO(old_dir).rename_done(task, old_dir, new_dir) { rpc_restart_call_prepare(task); return; }
    if let Some(complete) = (*data).complete { complete(task, data); }
}

unsafe fn nfs_async_rename_release(calldata: *mut core::ffi::c_void) {
    let data = calldata as *mut nfs_renamedata;
    let sb = (*(*data).old_dir).i_sb;
    if d_really_is_positive((*data).old_dentry) { nfs_mark_for_revalidate(d_inode((*data).old_dentry)); }
    if (*data).cancelled {
        spin_lock(&mut (*(*data).old_dir).i_lock); nfs_force_lookup_revalidate((*data).old_dir); spin_unlock(&mut (*(*data).old_dir).i_lock);
        if (*data).new_dir != (*data).old_dir { spin_lock(&mut (*(*data).new_dir).i_lock); nfs_force_lookup_revalidate((*data).new_dir); spin_unlock(&mut (*(*data).new_dir).i_lock); }
    }
    dput((*data).old_dentry); dput((*data).new_dentry); iput((*data).old_dir); iput((*data).new_dir); nfs_sb_deactive(sb); put_cred((*data).cred); kfree(data as *mut _);
}

unsafe fn nfs_rename_prepare(task: *mut rpc_task, calldata: *mut core::ffi::c_void) {
    let data = calldata as *mut nfs_renamedata;
    NFS_PROTO((*data).old_dir).rename_rpc_prepare(task, data);
}

static mut NFS_RENAME_OPS: rpc_call_ops = rpc_call_ops { rpc_call_done: Some(nfs_async_rename_done), rpc_release: Some(nfs_async_rename_release), rpc_call_prepare: Some(nfs_rename_prepare) };

unsafe fn nfs_async_rename(old_dir: *mut inode, new_dir: *mut inode, old_dentry: *mut dentry, new_dentry: *mut dentry, complete: Option<unsafe fn(*mut rpc_task, *mut nfs_renamedata)>) -> *mut rpc_task {
    let data = kzalloc_obj::<nfs_renamedata>();
    if data.is_null() { return ERR_PTR(-ENOMEM); }
    let mut msg: rpc_message = core::mem::zeroed();
    let mut setup: rpc_task_setup = core::mem::zeroed();
    setup.rpc_message = &mut msg; setup.callback_ops = &NFS_RENAME_OPS; setup.workqueue = nfsiod_workqueue; setup.rpc_client = NFS_CLIENT(old_dir); setup.flags = RPC_TASK_ASYNC | RPC_TASK_CRED_NOREF; setup.callback_data = data as *mut _; setup.task = &mut (*data).task;
    if nfs_server_capable(old_dir, NFS_CAP_MOVEABLE) && nfs_server_capable(new_dir, NFS_CAP_MOVEABLE) { setup.flags |= RPC_TASK_MOVEABLE; }
    (*data).cred = get_current_cred(); msg.rpc_argp = &mut (*data).args; msg.rpc_resp = &mut (*data).res; msg.rpc_cred = (*data).cred;
    (*data).old_dir = old_dir; ihold(old_dir); (*data).new_dir = new_dir; ihold(new_dir); (*data).old_dentry = dget(old_dentry); (*data).new_dentry = dget(new_dentry); nfs_fattr_init(&mut (*data).old_fattr); nfs_fattr_init(&mut (*data).new_fattr); (*data).complete = complete;
    (*data).args.old_dir = NFS_FH(old_dir); (*data).args.old_name = &(*old_dentry).d_name; (*data).args.new_dir = NFS_FH(new_dir); (*data).args.new_name = &(*new_dentry).d_name;
    (*data).res.old_fattr = &mut (*data).old_fattr; (*data).res.new_fattr = &mut (*data).new_fattr;
    nfs_sb_active((*old_dir).i_sb); NFS_PROTO((*data).old_dir).rename_setup(&mut msg, old_dentry, new_dentry, if old_dir == new_dir { old_dir } else { core::ptr::null_mut() }); rpc_run_task(&mut setup)
}

unsafe fn nfs_complete_sillyrename(task: *mut rpc_task, data: *mut nfs_renamedata) { if (*task).tk_status != 0 { nfs_cancel_async_unlink((*data).old_dentry); } }

const SILLYNAME_PREFIX: &[u8] = b".nfs";
const SILLYNAME_PREFIX_LEN: usize = 4;
const SILLYNAME_FILEID_LEN: usize = core::mem::size_of::<u64>() << 1;
const SILLYNAME_COUNTER_LEN: usize = core::mem::size_of::<u32>() << 1;
const SILLYNAME_LEN: usize = SILLYNAME_PREFIX_LEN + SILLYNAME_FILEID_LEN + SILLYNAME_COUNTER_LEN;

unsafe fn nfs_sillyrename(dir: *mut inode, dentry: *mut dentry) -> i32 {
    static mut SILLYCOUNTER: u32 = 0;
    let mut silly = [0u8; SILLYNAME_LEN + 1];
    let inode = d_inode(dentry);
    let mut sdentry: *mut dentry = core::ptr::null_mut();
    let mut error = -EBUSY;
    dfprintk(VFS, "NFS: silly-rename(%pd2, ct=%d)\n", dentry, d_count(dentry));
    nfs_inc_stats(dir, NFSIOS_SILLYRENAME);
    if ((*dentry).d_flags & DCACHE_NFSFS_RENAMED) != 0 { return error; }
    let fileid = (*inode).i_ino;
    loop {
        dput(sdentry); SILLYCOUNTER = SILLYCOUNTER.wrapping_add(1);
        scnprintf(silly.as_mut_ptr(), silly.len(), SILLYNAME_PREFIX.as_ptr(), SILLYNAME_FILEID_LEN, fileid, SILLYNAME_COUNTER_LEN, SILLYCOUNTER);
        dfprintk(VFS, "NFS: trying to rename %pd to %s\n", dentry, silly.as_ptr());
        sdentry = lookup_noperm(&QSTR(silly.as_ptr()), (*dentry).d_parent);
        if IS_ERR(sdentry) { return error; }
        if d_inode(sdentry).is_null() { break; }
    }
    ihold(inode);
    error = nfs_async_unlink(dentry, &(*sdentry).d_name);
    if error != 0 { iput(inode); dput(sdentry); return error; }
    let task = nfs_async_rename(dir, dir, dentry, sdentry, Some(nfs_complete_sillyrename));
    if IS_ERR(task) { nfs_cancel_async_unlink(dentry); iput(inode); dput(sdentry); return -EBUSY; }
    error = rpc_wait_for_completion_task(task); if error == 0 { error = (*task).tk_status; }
    match error {
        0 => { nfs_set_verifier(dentry, nfs_save_change_attribute(dir)); spin_lock(&mut (*inode).i_lock); NFS_I(inode).attr_gencount = nfs_inc_attr_generation_counter(); nfs_set_cache_invalid(inode, NFS_INO_INVALID_CHANGE | NFS_INO_INVALID_CTIME | NFS_INO_REVAL_FORCED); spin_unlock(&mut (*inode).i_lock); d_move(dentry, sdentry); }
        -ERESTARTSYS => { d_drop(dentry); d_drop(sdentry); }
        _ => {}
    }
    rpc_put_task(task); iput(inode); dput(sdentry); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
