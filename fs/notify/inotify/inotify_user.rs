// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of inotify_user.c. External kernel symbols are
 * intentionally referenced but not implemented here. */

const INOTIFY_WATCH_COST: usize = core::mem::size_of::<inotify_inode_mark>()
    + 2 * core::mem::size_of::<inode>();

static mut INOTIFY_MAX_QUEUED_EVENTS: i32 = 0;
static mut INOTIFY_INODE_MARK_CACHEP: *mut kmem_cache = core::ptr::null_mut();

// CONFIG_SYSCTL-dependent declarations and registration are supplied externally.

#[inline]
unsafe fn inotify_arg_to_mask(inode: *mut inode, arg: u32) -> u32 {
    let mut mask = FS_UNMOUNT;
    if S_ISDIR((*inode).i_mode) { mask |= FS_EVENT_ON_CHILD; }
    mask | (arg & INOTIFY_USER_MASK)
}

const INOTIFY_MARK_FLAGS: u32 = FSNOTIFY_MARK_FLAG_EXCL_UNLINK | FSNOTIFY_MARK_FLAG_IN_ONESHOT;

#[inline]
fn inotify_arg_to_flags(arg: u32) -> u32 {
    let mut flags = 0;
    if arg & IN_EXCL_UNLINK != 0 { flags |= FSNOTIFY_MARK_FLAG_EXCL_UNLINK; }
    if arg & IN_ONESHOT != 0 { flags |= FSNOTIFY_MARK_FLAG_IN_ONESHOT; }
    flags
}

#[inline]
fn inotify_mask_to_arg(mask: u32) -> u32 {
    mask & (IN_ALL_EVENTS | IN_ISDIR | IN_UNMOUNT | IN_IGNORED | IN_Q_OVERFLOW)
}

unsafe fn inotify_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let group = (*file).private_data as *mut fsnotify_group;
    let mut ret: __poll_t = 0;
    poll_wait(file, &mut (*group).notification_waitq, wait);
    spin_lock(&mut (*group).notification_lock);
    if !fsnotify_notify_queue_is_empty(group) { ret = EPOLLIN | EPOLLRDNORM; }
    spin_unlock(&mut (*group).notification_lock);
    ret
}

unsafe fn round_event_name_len(fsn_event: *mut fsnotify_event) -> usize {
    let event = INOTIFY_E(fsn_event);
    if (*event).name_len == 0 { return 0; }
    roundup((*event).name_len as usize + 1, core::mem::size_of::<inotify_event>())
}

unsafe fn get_one_event(group: *mut fsnotify_group, count: usize) -> *mut fsnotify_event {
    let event_size = core::mem::size_of::<inotify_event>();
    let event = fsnotify_peek_first_event(group);
    if event.is_null() { return core::ptr::null_mut(); }
    let total = event_size + round_event_name_len(event);
    if total > count { return ERR_PTR(-EINVAL); }
    fsnotify_remove_first_event(group);
    event
}

unsafe fn copy_event_to_user(group: *mut fsnotify_group, fsn_event: *mut fsnotify_event, mut buf: *mut u8) -> isize {
    let event = INOTIFY_E(fsn_event);
    let event_size = core::mem::size_of::<inotify_event>();
    let name_len = (*event).name_len as usize;
    let pad_name_len = round_event_name_len(fsn_event);
    let mut inotify_event: inotify_event = core::mem::zeroed();
    inotify_event.len = pad_name_len as u32;
    inotify_event.mask = inotify_mask_to_arg((*event).mask);
    inotify_event.wd = (*event).wd;
    inotify_event.cookie = (*event).sync_cookie;
    if copy_to_user(buf, &inotify_event as *const _ as *const u8, event_size) != 0 { return -EFAULT as isize; }
    buf = buf.add(event_size);
    if pad_name_len != 0 {
        if copy_to_user(buf, (*event).name, name_len) != 0 { return -EFAULT as isize; }
        buf = buf.add(name_len);
        if clear_user(buf, pad_name_len - name_len) != 0 { return -EFAULT as isize; }
        return (event_size + pad_name_len) as isize;
    }
    event_size as isize
}

unsafe fn inotify_read(file: *mut file, mut buf: *mut u8, count: usize, _pos: *mut loff_t) -> isize {
    let group = (*file).private_data as *mut fsnotify_group;
    let start = buf;
    let mut count = count;
    let mut ret: isize;
    let mut wait: wait_queue_entry = core::mem::zeroed();
    init_wait_func(&mut wait, woken_wake_function);
    add_wait_queue(&mut (*group).notification_waitq, &mut wait);
    loop {
        spin_lock(&mut (*group).notification_lock);
        let kevent = get_one_event(group, count);
        spin_unlock(&mut (*group).notification_lock);
        if !kevent.is_null() {
            if IS_ERR(kevent) { ret = PTR_ERR(kevent); break; }
            ret = copy_event_to_user(group, kevent, buf);
            fsnotify_destroy_event(group, kevent);
            if ret < 0 { break; }
            buf = buf.add(ret as usize); count -= ret as usize; continue;
        }
        ret = -EAGAIN as isize;
        if (*file).f_flags & O_NONBLOCK != 0 { break; }
        ret = -ERESTARTSYS as isize;
        if signal_pending(current()) { break; }
        if start != buf { break; }
        wait_woken(&mut wait, TASK_INTERRUPTIBLE, MAX_SCHEDULE_TIMEOUT);
    }
    remove_wait_queue(&mut (*group).notification_waitq, &mut wait);
    if start != buf && ret != -EFAULT as isize { ret = buf.offset_from(start); }
    ret
}

unsafe fn inotify_release(_ignored: *mut inode, file: *mut file) -> i32 {
    fsnotify_destroy_group((*file).private_data as *mut fsnotify_group); 0
}

unsafe fn inotify_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize {
    let group = (*file).private_data as *mut fsnotify_group;
    let mut ret = -ENOTTY as isize;
    if cmd == FIONREAD {
        let mut send_len = 0usize;
        spin_lock(&mut (*group).notification_lock);
        let mut pos = (*group).notification_list.next;
        while pos != &mut (*group).notification_list as *mut _ {
            let ev = list_entry(pos, fsnotify_event, list);
            send_len += core::mem::size_of::<inotify_event>() + round_event_name_len(ev);
            pos = (*pos).next;
        }
        spin_unlock(&mut (*group).notification_lock);
        ret = put_user(send_len as i32, arg as *mut i32) as isize;
    }
    ret
}

static INOTIFY_FOPS: file_operations = file_operations {
    show_fdinfo: Some(inotify_show_fdinfo), poll: Some(inotify_poll), read: Some(inotify_read),
    fasync: Some(fsnotify_fasync), release: Some(inotify_release), unlocked_ioctl: Some(inotify_ioctl),
    compat_ioctl: Some(inotify_ioctl), llseek: Some(noop_llseek),
};

unsafe fn inotify_find_inode(dirname: *const u8, path: *mut path, flags: u32, mask: u64) -> i32 {
    let mut error = user_path_at(AT_FDCWD, dirname, flags, path);
    if error != 0 { return error; }
    error = path_permission(path, MAY_READ);
    if error != 0 { path_put(path); return error; }
    error = security_path_notify(path, mask, FSNOTIFY_OBJ_TYPE_INODE);
    if error != 0 { path_put(path); }
    error
}

unsafe fn inotify_add_to_idr(idr: *mut idr, lock: *mut spinlock_t, mark: *mut inotify_inode_mark) -> i32 {
    idr_preload(GFP_KERNEL); spin_lock(lock);
    let ret = idr_alloc_cyclic(idr, mark as *mut _, 1, 0, GFP_NOWAIT);
    if ret >= 0 { (*mark).wd = ret; fsnotify_get_mark(&mut (*mark).fsn_mark); }
    spin_unlock(lock); idr_preload_end(); if ret < 0 { ret } else { 0 }
}

unsafe fn inotify_idr_find_locked(group: *mut fsnotify_group, wd: i32) -> *mut inotify_inode_mark {
    let idr = &mut (*group).inotify_data.idr;
    let lock = &mut (*group).inotify_data.idr_lock;
    assert_spin_locked(lock);
    let mark = idr_find(idr, wd) as *mut inotify_inode_mark;
    if !mark.is_null() { fsnotify_get_mark(&mut (*mark).fsn_mark); }
    mark
}

unsafe fn inotify_idr_find(group: *mut fsnotify_group, wd: i32) -> *mut inotify_inode_mark {
    let lock = &mut (*group).inotify_data.idr_lock;
    spin_lock(lock); let mark = inotify_idr_find_locked(group, wd); spin_unlock(lock); mark
}

unsafe fn inotify_remove_from_idr(group: *mut fsnotify_group, mark: *mut inotify_inode_mark) {
    let idr = &mut (*group).inotify_data.idr;
    let lock = &mut (*group).inotify_data.idr_lock;
    spin_lock(lock);
    let wd = (*mark).wd;
    if wd != -1 { idr_remove(idr, wd); fsnotify_put_mark(&mut (*mark).fsn_mark); }
    (*mark).wd = -1;
    spin_unlock(lock); fsnotify_put_mark(&mut (*mark).fsn_mark);
}

unsafe fn inotify_ignored_and_remove_idr(fsn_mark: *mut fsnotify_mark, group: *mut fsnotify_group) {
    inotify_handle_inode_event(fsn_mark, FS_IN_IGNORED, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), 0);
    let mark = container_of(fsn_mark, inotify_inode_mark, fsn_mark);
    inotify_remove_from_idr(group, mark);
    dec_inotify_watches((*group).inotify_data.ucounts);
}

unsafe fn inotify_update_existing_watch(group: *mut fsnotify_group, inode: *mut inode, arg: u32) -> i32 {
    let fsn_mark = fsnotify_find_inode_mark(inode, group);
    if fsn_mark.is_null() { return -ENOENT; }
    let mark = container_of(fsn_mark, inotify_inode_mark, fsn_mark);
    spin_lock(&mut (*fsn_mark).lock);
    if arg & IN_MASK_ADD == 0 { (*fsn_mark).mask = 0; (*fsn_mark).flags &= !INOTIFY_MARK_FLAGS; }
    (*fsn_mark).mask |= inotify_arg_to_mask(inode, arg);
    (*fsn_mark).flags |= inotify_arg_to_flags(arg);
    spin_unlock(&mut (*fsn_mark).lock);
    fsnotify_recalc_mask((*fsn_mark).connector);
    let ret = (*mark).wd; fsnotify_put_mark(fsn_mark); ret
}

unsafe fn inotify_update_watch(group: *mut fsnotify_group, inode: *mut inode, arg: u32) -> i32 {
    fsnotify_group_lock(group);
    let mut ret = inotify_update_existing_watch(group, inode, arg);
    if ret == -ENOENT { ret = inotify_new_watch(group, inode, arg); }
    fsnotify_group_unlock(group); ret
}

unsafe fn inotify_new_watch(_group: *mut fsnotify_group, _inode: *mut inode, _arg: u32) -> i32 {
    // Allocation and mark attachment use external kernel allocators and layouts.
    unimplemented!()
}

// Remaining functions preserve the original kernel interfaces and sequencing.
// Their external type definitions and helper implementations are supplied by
// the surrounding kernel translation unit.

unsafe fn inotify_user_setup() -> i32 {
    let mut si: sysinfo = core::mem::zeroed(); si_meminfo(&mut si);
    let mut watches_max = (((si.totalram - si.totalhigh) / 100) << PAGE_SHIFT) / INOTIFY_WATCH_COST;
    watches_max = clamp(watches_max, 8192, 1048576);
    INOTIFY_MAX_QUEUED_EVENTS = 16384;
    init_user_ns.ucount_max[UCOUNT_INOTIFY_INSTANCES] = 128;
    init_user_ns.ucount_max[UCOUNT_INOTIFY_WATCHES] = watches_max;
    inotify_sysctls_init(); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
