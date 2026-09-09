// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * fs/inotify_user.c - inotify support for userspace
 *
 * Authors:
 *\tJohn McCutchan\t<ttb@tentacle.dhs.org>
 *\tRobert Love\t<rml@novell.com>
 *
 * Copyright (C) 2005 John McCutchan
 * Copyright 2006 Hewlett-Packard Development Company, L.P.
 *
 * Copyright (C) 2009 Eric Paris <Red Hat Inc>
 * inotify was largely rewritten to make use of the fsnotify infrastructure
 */

// C dependencies supplied by the surrounding kernel translation unit.

/*
 * Check if 2 events contain the same information.
 */
unsafe fn event_compare(
    old_fsn: *mut fsnotify_event,
    new_fsn: *mut fsnotify_event,
) -> bool {
    let old: *mut inotify_event_info = INOTIFY_E(old_fsn);
    let new: *mut inotify_event_info = INOTIFY_E(new_fsn);

    if (*old).mask & FS_IN_IGNORED != 0 {
        return false;
    }
    if (*old).mask == (*new).mask
        && (*old).wd == (*new).wd
        && (*old).name_len == (*new).name_len
        && ((*old).name_len == 0 || strcmp((*old).name, (*new).name) == 0)
    {
        return true;
    }
    false
}

unsafe fn inotify_merge(
    group: *mut fsnotify_group,
    event: *mut fsnotify_event,
) -> bool {
    let list: *mut list_head = &mut (*group).notification_list;
    let last_event: *mut fsnotify_event = list_entry(
        (*list).prev,
        fsnotify_event,
        list,
    );
    event_compare(last_event, event)
}

pub unsafe fn inotify_handle_inode_event(
    inode_mark: *mut fsnotify_mark,
    mut mask: u32,
    inode: *mut inode,
    dir: *mut inode,
    name: *const qstr,
    cookie: u32,
) -> i32 {
    let mut i_mark: *mut inotify_inode_mark;
    let mut event: *mut inotify_event_info;
    let mut fsn_event: *mut fsnotify_event;
    let group: *mut fsnotify_group = (*inode_mark).group;
    let mut ret: i32;
    let mut len: i32 = 0;
    let mut wd: i32;
    let mut alloc_len: usize = core::mem::size_of::<inotify_event_info>();
    let old_memcg: *mut mem_cgroup;

    if !name.is_null() {
        len = (*name).len;
        alloc_len += len as usize + 1;
    }

    pr_debug!("%s: group=%p mark=%p mask=%x\n", __func__, group, inode_mark, mask);

    i_mark = container_of!(inode_mark, inotify_inode_mark, fsn_mark);

    /*
     * We can be racing with mark being detached. Don't report event with
     * invalid wd.
     */
    wd = READ_ONCE!((*i_mark).wd);
    if wd == -1 {
        return 0;
    }
    /*
     * Whoever is interested in the event, pays for the allocation. Do not
     * trigger OOM killer in the target monitoring memcg as it may have
     * security repercussion.
     */
    old_memcg = set_active_memcg((*group).memcg);
    event = kmalloc(alloc_len, GFP_KERNEL_ACCOUNT | __GFP_RETRY_MAYFAIL) as *mut inotify_event_info;
    set_active_memcg(old_memcg);

    if event.is_null() {
        /*
         * Treat lost event due to ENOMEM the same way as queue
         * overflow to let userspace know event was lost.
         */
        fsnotify_queue_overflow(group);
        return -ENOMEM;
    }

    /*
     * We now report FS_ISDIR flag with MOVE_SELF and DELETE_SELF events
     * for fanotify. inotify never reported IN_ISDIR with those events.
     * It looks like an oversight, but to avoid the risk of breaking
     * existing inotify programs, mask the flag out from those events.
     */
    if mask & (IN_MOVE_SELF | IN_DELETE_SELF) != 0 {
        mask &= !IN_ISDIR;
    }

    fsn_event = &mut (*event).fse;
    fsnotify_init_event(fsn_event);
    (*event).mask = mask;
    (*event).wd = wd;
    (*event).sync_cookie = cookie;
    (*event).name_len = len;
    if len != 0 {
        strscpy!((*event).name, (*name).name, len + 1);
    }

    ret = fsnotify_add_event(group, fsn_event, inotify_merge);
    if ret != 0 {
        /* Our event wasn't used in the end. Free it. */
        fsnotify_destroy_event(group, fsn_event);
    }

    if (*inode_mark).flags & FSNOTIFY_MARK_FLAG_IN_ONESHOT != 0 {
        fsnotify_destroy_mark(inode_mark, group);
    }

    0
}

unsafe fn inotify_freeing_mark(
    fsn_mark: *mut fsnotify_mark,
    group: *mut fsnotify_group,
) {
    inotify_ignored_and_remove_idr(fsn_mark, group);
}

/*
 * This is NEVER supposed to be called. Inotify marks should either have been
 * removed from the idr when the watch was removed or in the
 * fsnotify_destroy_mark_by_group() call when the inotify instance was being
 * torn down. This is only called if the idr is about to be freed but there
 * are still marks in it.
 */
unsafe fn idr_callback(id: i32, p: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> i32 {
    let mut fsn_mark: *mut fsnotify_mark;
    let mut i_mark: *mut inotify_inode_mark;
    static mut WARNED: bool = false;

    if WARNED {
        return 0;
    }

    WARNED = true;
    fsn_mark = p as *mut fsnotify_mark;
    i_mark = container_of!(fsn_mark, inotify_inode_mark, fsn_mark);

    WARN!(1, "inotify closing but id=%d for fsn_mark=%p in group=%p still in idr.  Probably leaking memory\n", id, p, data);

    /*
     * I'm taking the liberty of assuming that the mark in question is a
     * valid address and I'm dereferencing it. This might help to figure
     * out why we got here and the panic is no worse than the original
     * BUG() that was here.
     */
    if !fsn_mark.is_null() {
        printk!(KERN_WARNING, "fsn_mark->group=%p wd=%d\n", (*fsn_mark).group, (*i_mark).wd);
    }
    0
}

unsafe fn inotify_free_group_priv(group: *mut fsnotify_group) {
    /* ideally the idr is empty and we won't hit the BUG in the callback */
    idr_for_each!(&mut (*group).inotify_data.idr, idr_callback, group);
    idr_destroy(&mut (*group).inotify_data.idr);
    if !(*group).inotify_data.ucounts.is_null() {
        dec_inotify_instances((*group).inotify_data.ucounts);
    }
}

unsafe fn inotify_free_event(
    group: *mut fsnotify_group,
    fsn_event: *mut fsnotify_event,
) {
    kfree!(INOTIFY_E(fsn_event));
}

/* ding dong the mark is dead */
unsafe fn inotify_free_mark(fsn_mark: *mut fsnotify_mark) {
    let i_mark: *mut inotify_inode_mark =
        container_of!(fsn_mark, inotify_inode_mark, fsn_mark);
    kmem_cache_free(inotify_inode_mark_cachep, i_mark);
}

#[no_mangle]
pub static inotify_fsnotify_ops: fsnotify_ops = fsnotify_ops {
    handle_inode_event: Some(inotify_handle_inode_event),
    free_group_priv: Some(inotify_free_group_priv),
    free_event: Some(inotify_free_event),
    freeing_mark: Some(inotify_freeing_mark),
    free_mark: Some(inotify_free_mark),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
