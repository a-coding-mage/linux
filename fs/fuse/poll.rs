// SPDX-License-Identifier: GPL-2.0-only

// Translated from poll.c. Declarations supplied by dev.h and fuse_i.h remain
// external dependencies of this translation unit.

pub unsafe fn fuse_end_polls(fc: *mut FuseConn) {
    let mut p: *mut RbNode;

    spin_lock(&mut (*fc).lock);
    p = rb_first(&(*fc).polled_files);

    while !p.is_null() {
        let ff: *mut FuseFile = rb_entry(p, FuseFile, polled_node);
        wake_up_interruptible_all(&mut (*ff).poll_wait);

        p = rb_next(p);
    }
    spin_unlock(&mut (*fc).lock);
}

/*
 * All files which have been polled are linked to RB tree
 * fuse_conn->polled_files which is indexed by kh.  Walk the tree and
 * find the matching one.
 */
unsafe fn fuse_find_polled_node(
    fc: *mut FuseConn,
    kh: u64,
    parent_out: *mut *mut RbNode,
) -> *mut *mut RbNode {
    let mut link: *mut *mut RbNode = &mut (*fc).polled_files.rb_node;
    let mut last: *mut RbNode = core::ptr::null_mut();

    while !(*link).is_null() {
        let ff: *mut FuseFile;

        last = *link;
        ff = rb_entry(last, FuseFile, polled_node);

        if kh < (*ff).kh {
            link = &mut (*last).rb_left;
        } else if kh > (*ff).kh {
            link = &mut (*last).rb_right;
        } else {
            return link;
        }
    }

    if !parent_out.is_null() {
        *parent_out = last;
    }
    link
}

/*
 * The file is about to be polled.  Make sure it's on the polled_files
 * RB tree.  Note that files once added to the polled_files tree are
 * not removed before the file is released.  This is because a file
 * polled once is likely to be polled again.
 */
unsafe fn fuse_register_polled_file(fc: *mut FuseConn, ff: *mut FuseFile) {
    spin_lock(&mut (*fc).lock);
    if rb_empty_node(&mut (*ff).polled_node) {
        let mut parent: *mut RbNode = core::ptr::null_mut();

        let link = fuse_find_polled_node(fc, (*ff).kh, &mut parent);
        bug_on(!(*link).is_null());
        rb_link_node(&mut (*ff).polled_node, parent, link);
        rb_insert_color(&mut (*ff).polled_node, &mut (*fc).polled_files);
    }
    spin_unlock(&mut (*fc).lock);
}

pub unsafe fn fuse_file_poll(file: *mut File, wait: *mut PollTable) -> PollT {
    let ff: *mut FuseFile = (*file).private_data as *mut FuseFile;
    let fm: *mut FuseMount = (*ff).fm;
    let mut inarg = FusePollIn {
        fh: (*ff).fh,
        kh: (*ff).kh,
        ..core::mem::zeroed()
    };
    let mut outarg: FusePollOut = core::mem::zeroed();
    let mut args: FuseArgs = core::mem::zeroed();
    let mut err: i32;

    if (*(*fm).fc).no_poll {
        return DEFAULT_POLLMASK;
    }

    poll_wait(file, &mut (*ff).poll_wait, wait);
    inarg.events = mangle_poll(poll_requested_events(wait));

    /*
     * Ask for notification iff there's someone waiting for it.
     * The client may ignore the flag and always notify.
     */
    if waitqueue_active(&mut (*ff).poll_wait) {
        inarg.flags |= FUSE_POLL_SCHEDULE_NOTIFY;
        fuse_register_polled_file((*fm).fc, ff);
    }

    args.opcode = FUSE_POLL;
    args.nodeid = (*ff).nodeid;
    args.in_numargs = 1;
    args.in_args[0].size = core::mem::size_of::<FusePollIn>();
    args.in_args[0].value = &mut inarg as *mut FusePollIn as *mut core::ffi::c_void;
    args.out_numargs = 1;
    args.out_args[0].size = core::mem::size_of::<FusePollOut>();
    args.out_args[0].value = &mut outarg as *mut FusePollOut as *mut core::ffi::c_void;
    err = fuse_simple_request(fm, &mut args);

    if err == 0 {
        return demangle_poll(outarg.revents);
    }
    if err == -ENOSYS {
        (*(*fm).fc).no_poll = true;
        return DEFAULT_POLLMASK;
    }
    EPOLLERR
}

/*
 * This is called from fuse_handle_notify() on FUSE_NOTIFY_POLL and
 * wakes up the poll waiters.
 */
pub unsafe fn fuse_notify_poll_wakeup(
    fc: *mut FuseConn,
    outarg: *mut FuseNotifyPollWakeupOut,
) -> i32 {
    let kh = (*outarg).kh;
    let link: *mut *mut RbNode;

    spin_lock(&mut (*fc).lock);

    link = fuse_find_polled_node(fc, kh, core::ptr::null_mut());
    if !(*link).is_null() {
        let ff: *mut FuseFile = rb_entry(*link, FuseFile, polled_node);
        wake_up_interruptible_sync(&mut (*ff).poll_wait);
    }

    spin_unlock(&mut (*fc).lock);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
