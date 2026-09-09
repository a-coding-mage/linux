// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2025 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Dependencies supplied by the surrounding kernel translation.

const FSERROR_DEFAULT_EVENT_POOL_SIZE: usize = 32;

static mut FSERROR_EVENTS_POOL: mempool = mempool {};

pub unsafe fn fserror_mount(sb: *mut super_block) {
    /*
     * The pending error counter is biased by 1 so that we don't wake_var
     * until we're actually trying to unmount.
     */
    refcount_set(&mut (*sb).s_pending_errors, 1);
}

pub unsafe fn fserror_unmount(sb: *mut super_block) {
    /*
     * If we don't drop the pending error count to zero, then wait for it
     * to drop below 1, which means that the pending errors cleared and
     * hopefully we didn't saturate with 1 billion+ concurrent events.
     */
    if !refcount_dec_and_test(&mut (*sb).s_pending_errors) {
        wait_var_event(
            &mut (*sb).s_pending_errors,
            refcount_read(&(*sb).s_pending_errors) < 1,
        );
    }
}

unsafe fn fserror_pending_dec(sb: *mut super_block) {
    if refcount_dec_and_test(&mut (*sb).s_pending_errors) {
        wake_up_var(&mut (*sb).s_pending_errors);
    }
}

unsafe fn fserror_free_event(event: *mut fserror_event) {
    fserror_pending_dec((*event).sb);
    mempool_free(event, &raw mut FSERROR_EVENTS_POOL);
}

unsafe fn fserror_worker(work: *mut work_struct) {
    let event = container_of(work, fserror_event, work);
    let sb = (*event).sb;

    if (*sb).s_flags & SB_ACTIVE != 0 {
        let report = fs_error_report {
            /* send positive error number to userspace */
            error: -(*event).error,
            inode: (*event).inode,
            sb: (*event).sb,
        };

        if (*(*sb).s_op).report_error.is_some() {
            ((*(*sb).s_op).report_error.unwrap())(event);
        }

        fsnotify(FS_ERROR, &report, FSNOTIFY_EVENT_ERROR, core::ptr::null_mut(),
                 core::ptr::null_mut(), core::ptr::null_mut(), 0);
    }

    iput((*event).inode);
    fserror_free_event(event);
}

unsafe fn fserror_alloc_event(sb: *mut super_block, gfp_flags: gfp_t) -> *mut fserror_event {
    let mut event: *mut fserror_event = core::ptr::null_mut();

    /*
     * If pending_errors already reached zero or is no longer active,
     * the superblock is being deactivated so there's no point in
     * continuing.
     *
     * The order of the check of s_pending_errors and SB_ACTIVE are
     * mandated by order of accesses in generic_shutdown_super and
     * fserror_unmount.  Barriers are implicitly provided by the refcount
     * manipulations in this function and fserror_unmount.
     */
    if !refcount_inc_not_zero(&mut (*sb).s_pending_errors) {
        return core::ptr::null_mut();
    }
    if (*sb).s_flags & SB_ACTIVE == 0 {
        fserror_pending_dec(sb);
        return core::ptr::null_mut();
    }

    event = mempool_alloc(&raw mut FSERROR_EVENTS_POOL, gfp_flags);
    if event.is_null() {
        fserror_pending_dec(sb);
        return core::ptr::null_mut();
    }

    /* mempool_alloc doesn't support GFP_ZERO */
    core::ptr::write_bytes(event as *mut u8, 0, core::mem::size_of::<fserror_event>());
    (*event).sb = sb;
    INIT_WORK(&mut (*event).work, fserror_worker);

    event
}

/**
 * fserror_report - report a filesystem error of some kind
 *
 * @sb:        superblock of the filesystem
 * @inode:     inode within that filesystem, if applicable
 * @type:      type of error encountered
 * @pos:       start of inode range affected, if applicable
 * @len:       length of inode range affected, if applicable
 * @error:     error number encountered, must be negative
 * @gfp:       memory allocation flags for conveying the event to a worker,
 *             since this function can be called from atomic contexts
 */
pub unsafe fn fserror_report(
    sb: *mut super_block,
    inode: *mut inode,
    type_: fserror_type,
    pos: loff_t,
    len: u64,
    error: i32,
    gfp: gfp_t,
) {
    let event;

    /* sb and inode must be from the same filesystem */
    WARN_ON_ONCE(!inode.is_null() && (*inode).i_sb != sb);

    /* error number must be negative */
    WARN_ON_ONCE(error >= 0);

    event = fserror_alloc_event(sb, gfp);
    if event.is_null() {
        if !inode.is_null() {
            pr_err_ratelimited("%s: lost file I/O error report for ino %llu type %u pos 0x%llx len 0x%llx error %d", (*sb).s_id, (*inode).i_ino, type_, pos, len, error);
        } else {
            pr_err_ratelimited("%s: lost filesystem error report for type %u error %d", (*sb).s_id, type_, error);
        }
        return;
    }

    (*event).type = type_;
    (*event).pos = pos;
    (*event).len = len;
    (*event).error = error;

    /*
     * Can't iput from non-sleeping context, so grabbing another reference
     * to the inode must be the last thing before submitting the event.
     */
    if !inode.is_null() {
        (*event).inode = igrab(inode);
        if (*event).inode.is_null() {
            fserror_free_event(event);
            if !inode.is_null() {
                pr_err_ratelimited("%s: lost file I/O error report for ino %llu type %u pos 0x%llx len 0x%llx error %d", (*sb).s_id, (*inode).i_ino, type_, pos, len, error);
            }
            return;
        }
    }

    /*
     * Use schedule_work here even if we're already in process context so
     * that fsnotify and super_operations::report_error implementations are
     * guaranteed to run in process context without any locks held.  Since
     * errors are supposed to be rare, the overhead shouldn't kill us any
     * more than the failing device will.
     */
    schedule_work(&mut (*event).work);
}

pub unsafe fn fserror_init() -> i32 {
    mempool_init_kmalloc_pool(
        &raw mut FSERROR_EVENTS_POOL,
        FSERROR_DEFAULT_EVENT_POOL_SIZE,
        core::mem::size_of::<fserror_event>(),
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
