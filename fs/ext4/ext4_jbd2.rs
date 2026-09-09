// SPDX-License-Identifier: GPL-2.0
/*
 * Interface between ext4 and JBD
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external; this file is a source-level translation of ext4_jbd2.c.

pub unsafe fn ext4_inode_journal_mode(inode: *mut inode) -> libc::c_int {
    if EXT4_JOURNAL(inode).is_null() {
        return EXT4_INODE_WRITEBACK_DATA_MODE; /* writeback */
    }
    /* We do not support data journalling with delayed allocation */
    if !S_ISREG((*inode).i_mode)
        || ext4_test_inode_flag(inode, EXT4_INODE_EA_INODE)
        || test_opt((*inode).i_sb, DATA_FLAGS) == EXT4_MOUNT_JOURNAL_DATA
        || (ext4_test_inode_flag(inode, EXT4_INODE_JOURNAL_DATA)
            && !test_opt((*inode).i_sb, DELALLOC))
    {
        /* We do not support data journalling for encrypted data */
        if S_ISREG((*inode).i_mode) && IS_ENCRYPTED(inode) {
            return EXT4_INODE_ORDERED_DATA_MODE; /* ordered */
        }
        return EXT4_INODE_JOURNAL_DATA_MODE; /* journal data */
    }
    if test_opt((*inode).i_sb, DATA_FLAGS) == EXT4_MOUNT_ORDERED_DATA {
        return EXT4_INODE_ORDERED_DATA_MODE; /* ordered */
    }
    if test_opt((*inode).i_sb, DATA_FLAGS) == EXT4_MOUNT_WRITEBACK_DATA {
        return EXT4_INODE_WRITEBACK_DATA_MODE; /* writeback */
    }
    BUG();
}

/* Just increment the non-pointer handle value */
unsafe fn ext4_get_nojournal() -> *mut handle_t {
    let mut handle = (*current).journal_info;

    BUG_ON(!handle.is_null() && !(*handle).h_invalid);

    if handle.is_null() {
        handle = jbd2_alloc_handle(GFP_NOFS);
        if handle.is_null() {
            return ERR_PTR(-ENOMEM);
        }
        (*handle).h_invalid = 1;
        /*
         * This is done by start_this_handle() if journalling
         * is enabled.
         */
        (*handle).saved_alloc_context = memalloc_nofs_save();
        (*current).journal_info = handle;
    }
    (*handle).h_ref += 1;
    handle
}

/* Decrement the non-pointer handle value */
unsafe fn ext4_put_nojournal(handle: *mut handle_t) {
    BUG_ON((*handle).h_ref == 0);

    (*handle).h_ref -= 1;
    if (*handle).h_ref == 0 {
        memalloc_nofs_restore((*handle).saved_alloc_context);
        jbd2_free_handle(handle);
        (*current).journal_info = core::ptr::null_mut();
    }
}

/*
 * Wrappers for jbd2_journal_start/end.
 */
unsafe fn ext4_journal_check_start(sb: *mut super_block) -> libc::c_int {
    let ret;
    let journal;

    might_sleep();

    ret = ext4_emergency_state(sb);
    if unlikely(ret) {
        return ret;
    }

    if WARN_ON_ONCE(sb_rdonly(sb)) {
        return -EROFS;
    }

    WARN_ON((*sb).s_writers.frozen == SB_FREEZE_COMPLETE);
    journal = (*EXT4_SB(sb)).s_journal;
    /*
     * Special case here: if the journal has aborted behind our
     * backs (eg. EIO in the commit thread), then we still need to
     * take the FS itself readonly cleanly.
     */
    if !journal.is_null() && is_journal_aborted(journal) {
        ext4_abort(sb, -(*journal).j_errno, c"Detected aborted journal");
        return -EROFS;
    }
    0
}

pub unsafe fn __ext4_journal_start_sb(
    inode: *mut inode,
    sb: *mut super_block,
    line: libc::c_uint,
    r#type: libc::c_int,
    blocks: libc::c_int,
    rsv_blocks: libc::c_int,
    revoke_creds: libc::c_int,
) -> *mut handle_t {
    let journal;
    let err;
    if !inode.is_null() {
        trace_ext4_journal_start_inode(inode, blocks, rsv_blocks, revoke_creds, r#type, _RET_IP_);
    } else {
        trace_ext4_journal_start_sb(sb, blocks, rsv_blocks, revoke_creds, r#type, _RET_IP_);
    }
    err = ext4_journal_check_start(sb);
    if err < 0 {
        return ERR_PTR(err);
    }

    journal = (*EXT4_SB(sb)).s_journal;
    if journal.is_null() || ((*EXT4_SB(sb)).s_mount_state & EXT4_FC_REPLAY) != 0 {
        return ext4_get_nojournal();
    }
    jbd2__journal_start(journal, blocks, rsv_blocks, revoke_creds, GFP_NOFS, r#type, line)
}

pub unsafe fn __ext4_journal_stop(
    where_: *const libc::c_char,
    line: libc::c_uint,
    handle: *mut handle_t,
) -> libc::c_int {
    let sb;
    let err;
    let rc;

    if !ext4_handle_valid(handle) {
        ext4_put_nojournal(handle);
        return 0;
    }

    err = (*handle).h_err;
    if (*handle).h_transaction.is_null() {
        rc = jbd2_journal_stop(handle);
        return if err != 0 { err } else { rc };
    }

    sb = (*(*handle).h_transaction).t_journal.j_private;
    rc = jbd2_journal_stop(handle);

    let mut err = err;
    if err == 0 {
        err = rc;
    }
    if err != 0 {
        __ext4_std_error(sb, where_, line, err);
    }
    err
}

pub unsafe fn __ext4_journal_start_reserved(
    handle: *mut handle_t,
    line: libc::c_uint,
    r#type: libc::c_int,
) -> *mut handle_t {
    let sb;
    let err;

    if !ext4_handle_valid(handle) {
        return ext4_get_nojournal();
    }

    sb = (*(*handle).h_journal).j_private;
    trace_ext4_journal_start_reserved(sb, jbd2_handle_buffer_credits(handle), _RET_IP_);
    err = ext4_journal_check_start(sb);
    if err < 0 {
        jbd2_journal_free_reserved(handle);
        return ERR_PTR(err);
    }

    err = jbd2_journal_start_reserved(handle, r#type, line);
    if err < 0 {
        return ERR_PTR(err);
    }
    handle
}

pub unsafe fn __ext4_journal_ensure_credits(
    handle: *mut handle_t,
    check_cred: libc::c_int,
    mut extend_cred: libc::c_int,
    mut revoke_cred: libc::c_int,
) -> libc::c_int {
    if !ext4_handle_valid(handle) {
        return 0;
    }
    if is_handle_aborted(handle) {
        return -EROFS;
    }
    if jbd2_handle_buffer_credits(handle) >= check_cred
        && (*handle).h_revoke_credits >= revoke_cred
    {
        return 0;
    }
    extend_cred = max(0, extend_cred - jbd2_handle_buffer_credits(handle));
    revoke_cred = max(0, revoke_cred - (*handle).h_revoke_credits);
    ext4_journal_extend(handle, extend_cred, revoke_cred)
}

unsafe fn ext4_journal_abort_handle(
    caller: *const libc::c_char,
    line: libc::c_uint,
    err_fn: *const libc::c_char,
    bh: *mut buffer_head,
    handle: *mut handle_t,
    err: libc::c_int,
) {
    let mut nbuf = [0 as libc::c_char; 16];
    let errstr = ext4_decode_error(core::ptr::null_mut(), err, nbuf.as_mut_ptr());

    BUG_ON(!ext4_handle_valid(handle));

    if !bh.is_null() {
        BUFFER_TRACE(bh, c"abort");
    }

    if (*handle).h_err == 0 {
        (*handle).h_err = err;
    }

    if is_handle_aborted(handle) {
        return;
    }

    printk!(KERN_ERR, c"EXT4-fs: %s:%d: aborting transaction: %s in %s\n", caller, line, errstr, err_fn);
    jbd2_journal_abort_handle(handle);
}

unsafe fn ext4_check_bdev_write_error(sb: *mut super_block) {
    let mapping = (*(*sb).s_bdev).bd_mapping;
    let sbi = EXT4_SB(sb);
    let mut err;

    /*
     * If the block device has write error flag, it may have failed to
     * async write out metadata buffers in the background. In this case,
     * we could read old data from disk and write it out again, which
     * may lead to on-disk filesystem inconsistency.
     */
    if errseq_check(&mut (*mapping).wb_err, READ_ONCE((*sbi).s_bdev_wb_err)) {
        spin_lock(&mut (*sbi).s_bdev_wb_lock);
        err = errseq_check_and_advance(&mut (*mapping).wb_err, &mut (*sbi).s_bdev_wb_err);
        spin_unlock(&mut (*sbi).s_bdev_wb_lock);
        if err != 0 {
            ext4_error_err(sb, -err, c"Error while async write back metadata");
        }
    }
}

pub unsafe fn __ext4_journal_get_write_access(
    where_: *const libc::c_char,
    line: libc::c_uint,
    handle: *mut handle_t,
    sb: *mut super_block,
    bh: *mut buffer_head,
    trigger_type: ext4_journal_trigger_type,
) -> libc::c_int {
    let err;

    might_sleep();

    if ext4_handle_valid(handle) {
        err = jbd2_journal_get_write_access(handle, bh);
        if err != 0 {
            ext4_journal_abort_handle(where_, line, __func__, bh, handle, err);
            return err;
        }
    } else {
        ext4_check_bdev_write_error(sb);
    }
    if trigger_type == EXT4_JTR_NONE || !ext4_has_feature_metadata_csum(sb) {
        return 0;
    }
    BUG_ON(trigger_type >= EXT4_JOURNAL_TRIGGER_COUNT);
    jbd2_journal_set_triggers(bh, &mut (*EXT4_SB(sb)).s_journal_triggers[trigger_type as usize].tr_triggers);
    0
}

/*
 * The ext4 forget function must perform a revoke if we are freeing data
 * which has been journaled.  Metadata (eg. indirect blocks) must be
 * revoked in all cases.
 *
 * "bh" may be NULL: a metadata block may have been freed from memory
 * but there may still be a record of it in the journal, and that record
 * still needs to be revoked.
 */
pub unsafe fn __ext4_forget(
    where_: *const libc::c_char,
    line: libc::c_uint,
    handle: *mut handle_t,
    is_metadata: libc::c_int,
    inode: *mut inode,
    bh: *mut buffer_head,
    blocknr: ext4_fsblk_t,
) -> libc::c_int {
    let mut err;

    might_sleep();

    trace_ext4_forget(inode, is_metadata, blocknr);
    BUFFER_TRACE(bh, c"enter");

    ext4_debug!(c"forgetting bh %p: is_metadata=%d, mode %o, data mode %x\n", bh, is_metadata, (*inode).i_mode, test_opt((*inode).i_sb, DATA_FLAGS));

    /*
     * In the no journal case, we should wait for the ongoing buffer
     * to complete and do a forget.
     */
    if !ext4_handle_valid(handle) {
        if !bh.is_null() {
            clear_buffer_dirty(bh);
            wait_on_buffer(bh);
            __bforget(bh);
        }
        return 0;
    }

    /* Never use the revoke function if we are doing full data
     * journaling: there is no need to, and a V1 superblock won't
     * support it.  Otherwise, only skip the revoke on un-journaled
     * data blocks. */
    if test_opt((*inode).i_sb, DATA_FLAGS) == EXT4_MOUNT_JOURNAL_DATA
        || (!is_metadata != 0 && !ext4_should_journal_data(inode))
    {
        if !bh.is_null() {
            BUFFER_TRACE(bh, c"call jbd2_journal_forget");
            err = jbd2_journal_forget(handle, bh);
            if err != 0 {
                ext4_journal_abort_handle(where_, line, __func__, bh, handle, err);
            }
            return err;
        }
        return 0;
    }

    /*
     * data!=journal && (is_metadata || should_journal_data(inode))
     */
    BUFFER_TRACE(bh, c"call jbd2_journal_revoke");
    err = jbd2_journal_revoke(handle, blocknr, bh);
    if err != 0 {
        ext4_journal_abort_handle(where_, line, __func__, bh, handle, err);
        __ext4_error((*inode).i_sb, where_, line, true, -err, 0, c"error %d when attempting revoke", err);
    }
    BUFFER_TRACE(bh, c"exit");
    err
}

pub unsafe fn __ext4_journal_get_create_access(
    where_: *const libc::c_char,
    line: libc::c_uint,
    handle: *mut handle_t,
    sb: *mut super_block,
    bh: *mut buffer_head,
    trigger_type: ext4_journal_trigger_type,
) -> libc::c_int {
    let err;

    if !ext4_handle_valid(handle) {
        return 0;
    }

    err = jbd2_journal_get_create_access(handle, bh);
    if err != 0 {
        ext4_journal_abort_handle(where_, line, __func__, bh, handle, err);
        return err;
    }
    if trigger_type == EXT4_JTR_NONE || !ext4_has_feature_metadata_csum(sb) {
        return 0;
    }
    BUG_ON(trigger_type >= EXT4_JOURNAL_TRIGGER_COUNT);
    jbd2_journal_set_triggers(bh, &mut (*EXT4_SB(sb)).s_journal_triggers[trigger_type as usize].tr_triggers);
    0
}

unsafe fn ext4_inode_attach_mmb(inode: *mut inode) {
    let mmb: *mut mapping_metadata_bhs;

    /*
     * It's difficult to handle failure when marking buffer dirty without
     * leaving filesystem corrupted
     */
    mmb = kmalloc_obj!(mapping_metadata_bhs, GFP_NOFS | __GFP_NOFAIL | __GFP_ACCOUNT);
    mmb_init(mmb, &mut (*inode).i_data);
    /* Someone swapped another mmb before us? */
    if cmpxchg(&mut (*EXT4_I(inode)).i_metadata_bhs, core::ptr::null_mut(), mmb) != core::ptr::null_mut() {
        kfree(mmb);
    }
}

pub unsafe fn __ext4_handle_dirty_metadata(
    where_: *const libc::c_char,
    line: libc::c_uint,
    handle: *mut handle_t,
    inode: *mut inode,
    bh: *mut buffer_head,
) -> libc::c_int {
    let mut err = 0;

    might_sleep();

    set_buffer_meta(bh);
    set_buffer_prio(bh);
    set_buffer_uptodate(bh);
    if ext4_handle_valid(handle) {
        err = jbd2_journal_dirty_metadata(handle, bh);
        /* Errors can only happen due to aborted journal or a nasty bug */
        if !is_handle_aborted(handle) && WARN_ON_ONCE(err) {
            ext4_journal_abort_handle(where_, line, __func__, bh, handle, err);
            if inode.is_null() {
                pr_err!(c"EXT4: jbd2_journal_dirty_metadata failed: handle type %u started at line %u, credits %u/%u, errcode %d", (*handle).h_type, (*handle).h_line_no, (*handle).h_requested_credits, jbd2_handle_buffer_credits(handle), err);
                return err;
            }
            ext4_error_inode(inode, where_, line, (*bh).b_blocknr, c"journal_dirty_metadata failed: handle type %u started at line %u, credits %u/%u, errcode %d", (*handle).h_type, (*handle).h_line_no, (*handle).h_requested_credits, jbd2_handle_buffer_credits(handle), err);
        }
    } else {
        if !inode.is_null() {
            if ext4_i_metadata_bhs(inode).is_null() {
                ext4_inode_attach_mmb(inode);
            }
            mmb_mark_buffer_dirty(bh, ext4_i_metadata_bhs(inode));
        } else {
            mark_buffer_dirty(bh);
        }
        if !inode.is_null() && inode_needs_sync(inode) {
            sync_dirty_buffer(bh);
            if buffer_req(bh) && !buffer_uptodate(bh) {
                ext4_error_inode_err(inode, where_, line, (*bh).b_blocknr, EIO, c"IO error syncing itable block");
                err = -EIO;
            }
        }
    }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
