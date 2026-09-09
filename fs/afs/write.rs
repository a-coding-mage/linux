// SPDX-License-Identifier: GPL-2.0-or-later
/* handling of writes to regular files and writing back to the server
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies are supplied by the surrounding kernel translation. */

/* completion of write to server */
unsafe fn afs_pages_written_back(vnode: *mut afs_vnode, start: loff_t, len: c_uint) {
    _enter!("{%llx:%llu},{%x @%llx}", (*vnode).fid.vid, (*vnode).fid.vnode, len, start);

    afs_prune_wb_keys(vnode);
    _leave!("");
}

/*
 * Find a key to use for the writeback.  We cached the keys used to author the
 * writes on the vnode.  wreq->netfs_priv2 will contain the last writeback key
 * record used or NULL and we need to start from there if it's set.
 * wreq->netfs_priv will be set to the key itself or NULL.
 */
unsafe fn afs_get_writeback_key(wreq: *mut netfs_io_request) {
    let mut wbk: *mut afs_wb_key;
    let old: *mut afs_wb_key = (*wreq).netfs_priv2;
    let vnode: *mut afs_vnode = AFS_FS_I((*wreq).inode);

    key_put((*wreq).netfs_priv);
    (*wreq).netfs_priv = core::ptr::null_mut();
    (*wreq).netfs_priv2 = core::ptr::null_mut();

    spin_lock(&mut (*vnode).wb_lock);
    if !old.is_null() {
        wbk = list_next_entry!(old, vnode_link);
    } else {
        wbk = list_first_entry!(&(*vnode).wb_keys, afs_wb_key, vnode_link);
    }

    list_for_each_entry_from!(wbk, &(*vnode).wb_keys, vnode_link, {
        _debug!("wbk %u", key_serial((*wbk).key));
        if key_validate((*wbk).key) == 0 {
            refcount_inc(&mut (*wbk).usage);
            (*wreq).netfs_priv = key_get((*wbk).key);
            (*wreq).netfs_priv2 = wbk;
            _debug!("USE WB KEY %u", key_serial((*wbk).key));
            break;
        }
    });

    spin_unlock(&mut (*vnode).wb_lock);

    afs_put_wb_key(old);
}

unsafe fn afs_store_data_success(op: *mut afs_operation) {
    let vnode: *mut afs_vnode = (*op).file[0].vnode;

    (*op).ctime = (*op).file[0].scb.status.mtime_client;
    afs_vnode_commit_status(op, &mut (*op).file[0]);
    if !afs_op_error(op) {
        afs_pages_written_back(vnode, (*op).store.pos, (*op).store.size);
        afs_stat_v(vnode, n_stores);
        atomic_long_add((*op).store.size, &mut afs_v2net(vnode).n_store_bytes);
    }
}

static afs_store_data_operation: afs_operation_ops = afs_operation_ops {
    issue_afs_rpc: Some(afs_fs_store_data),
    issue_yfs_rpc: Some(yfs_fs_store_data),
    success: Some(afs_store_data_success),
};

/*
 * Prepare a subrequest to write to the server.  This sets the max_len
 * parameter.
 */
unsafe fn afs_prepare_write(subreq: *mut netfs_io_subrequest) {
    let stream: *mut netfs_io_stream = &mut (*(*subreq).rreq).io_streams[(*subreq).stream_nr];

    //if (test_bit(NETFS_SREQ_RETRYING, &subreq->flags))
    //  subreq->max_len = 512 * 1024;
    //else
    (*stream).sreq_max_len = 256 * 1024 * 1024;
}

/* Issue a subrequest to write to the server. */
unsafe fn afs_issue_write_worker(work: *mut work_struct) {
    let subreq: *mut netfs_io_subrequest = container_of!(work, netfs_io_subrequest, work);
    let wreq: *mut netfs_io_request = (*subreq).rreq;
    let op: *mut afs_operation;
    let vnode: *mut afs_vnode = AFS_FS_I((*wreq).inode);
    let pos: u64 = (*subreq).start + (*subreq).transferred;
    let len: usize = (*subreq).len - (*subreq).transferred;
    let mut ret: i32 = -ENOKEY;

    _enter!("R=%x[%x],%s{%llx:%llu.%u},%llx,%zx", (*wreq).debug_id,
            (*subreq).debug_index, (*(*vnode).volume).name, (*vnode).fid.vid,
            (*vnode).fid.vnode, (*vnode).fid.unique, pos, len);

    op = afs_alloc_operation((*wreq).netfs_priv, (*vnode).volume);
    if IS_ERR!(op) {
        return netfs_write_subrequest_terminated(subreq, -EAGAIN);
    }

    afs_op_set_vnode(op, 0, vnode);
    (*op).file[0].dv_delta = 1;
    (*op).file[0].modification = true;
    (*op).store.pos = pos;
    (*op).store.size = len;
    (*op).flags |= AFS_OPERATION_UNINTR;
    (*op).ops = &afs_store_data_operation;

    afs_begin_vnode_operation(op);
    (*op).store.write_iter = &mut (*subreq).io_iter;
    (*op).store.i_size = umax(pos + len as u64, netfs_read_remote_i_size(&(*vnode).netfs.inode));
    (*op).mtime = inode_get_mtime(&(*vnode).netfs.inode);

    afs_wait_for_operation(op);
    ret = afs_put_operation(op);
    match ret {
        0 => __set_bit!(NETFS_SREQ_MADE_PROGRESS, &mut (*subreq).flags),
        -EACCES | -EPERM | -ENOKEY | -EKEYEXPIRED | -EKEYREJECTED | -EKEYREVOKED => {
            if !(*wreq).netfs_priv2.is_null() {
                set_bit!(NETFS_SREQ_NEED_RETRY, &mut (*subreq).flags);
            }
        }
        _ => {}
    }

    netfs_write_subrequest_terminated(subreq, if ret < 0 { ret } else { (*subreq).len as i32 });
}

unsafe fn afs_issue_write(subreq: *mut netfs_io_subrequest) {
    (*subreq).work.func = Some(afs_issue_write_worker);
    if !queue_work(system_dfl_wq, &mut (*subreq).work) {
        WARN_ON_ONCE!(1);
    }
}

/* Writeback calls this when it finds a folio that needs uploading. */
unsafe fn afs_begin_writeback(wreq: *mut netfs_io_request) {
    if S_ISREG!((*(*wreq).inode).i_mode) {
        afs_get_writeback_key(wreq);
    }
}

/* Prepare to retry the writes in request.  Use this to try rotating keys. */
unsafe fn afs_retry_request(wreq: *mut netfs_io_request, stream: *mut netfs_io_stream) {
    let subreq: *mut netfs_io_subrequest = list_first_entry!(&(*stream).subrequests, netfs_io_subrequest, rreq_link);

    match (*wreq).origin {
        NETFS_READAHEAD | NETFS_READPAGE | NETFS_READ_GAPS | NETFS_READ_SINGLE |
        NETFS_READ_FOR_WRITE | NETFS_UNBUFFERED_READ | NETFS_DIO_READ => return,
        _ => {}
    }

    match (*subreq).error {
        -EACCES | -EPERM | -ENOKEY | -EKEYEXPIRED | -EKEYREJECTED | -EKEYREVOKED => {
            afs_get_writeback_key(wreq);
            if (*wreq).netfs_priv.is_null() {
                (*stream).failed = true;
            }
        }
        _ => {}
    }
}

/* write some of the pending data back to the server */
unsafe fn afs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32 {
    let vnode: *mut afs_vnode = AFS_FS_I((*mapping).host);
    let ret: i32;

    if (*wbc).sync_mode == WB_SYNC_ALL {
        down_read(&mut (*vnode).validate_lock);
    } else if !down_read_trylock(&mut (*vnode).validate_lock) {
        return 0;
    }

    ret = netfs_writepages(mapping, wbc);
    up_read(&mut (*vnode).validate_lock);
    ret
}

/* flush any dirty pages for this process, and check for write errors. */
unsafe fn afs_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: i32) -> i32 {
    let vnode: *mut afs_vnode = AFS_FS_I(file_inode(file));
    let af: *mut afs_file = (*file).private_data as *mut afs_file;

    _enter!("{%llx:%llu},{n=%pD},%d", (*vnode).fid.vid, (*vnode).fid.vnode, file, datasync);
    let ret = afs_validate(vnode, (*af).key);
    if ret < 0 { return ret; }
    file_write_and_wait_range(file, start, end)
}

/* notification that a previously read-only page is about to become writable */
unsafe fn afs_page_mkwrite(vmf: *mut vm_fault) -> vm_fault_t {
    let file: *mut file = (*(*vmf).vma).vm_file;
    if afs_validate(AFS_FS_I(file_inode(file)), afs_file_key(file)) < 0 {
        return VM_FAULT_SIGBUS;
    }
    netfs_page_mkwrite(vmf, core::ptr::null_mut())
}

/* Prune the keys cached for writeback.  The caller must hold vnode->wb_lock. */
unsafe fn afs_prune_wb_keys(vnode: *mut afs_vnode) {
    let mut graveyard: list_head = LIST_HEAD!();
    let mut wbk: *mut afs_wb_key;
    let mut tmp: *mut afs_wb_key;

    spin_lock(&mut (*vnode).wb_lock);
    if !mapping_tagged(&mut (*vnode).netfs.inode.i_data, PAGECACHE_TAG_WRITEBACK) &&
       !mapping_tagged(&mut (*vnode).netfs.inode.i_data, PAGECACHE_TAG_DIRTY) {
        list_for_each_entry_safe!(wbk, tmp, &(*vnode).wb_keys, vnode_link, {
            if refcount_read(&(*wbk).usage) == 1 {
                list_move(&mut (*wbk).vnode_link, &mut graveyard);
            }
        });
    }
    spin_unlock(&mut (*vnode).wb_lock);

    while !list_empty(&graveyard) {
        wbk = list_entry!(graveyard.next, afs_wb_key, vnode_link);
        list_del(&mut (*wbk).vnode_link);
        afs_put_wb_key(wbk);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
