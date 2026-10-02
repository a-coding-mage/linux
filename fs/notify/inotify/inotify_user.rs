// SPDX-License-Identifier: GPL-2.0-or-later
/* Userspace inotify, translated from the unchanged inotify_user.c.
 * All configured layouts come from the kernel headers. C glue contains only
 * declaration/macro/header-inline boundaries, not inotify runtime policy. */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/inotify_user_generated.rs"));
}
use bindings::*;
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null, null_mut};
use kernel::ffi::{c_char, c_int, c_long, c_ulong};

#[link_section = ".data..read_mostly"]
static mut inotify_max_queued_events: c_int = 0;
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut inotify_inode_mark_cachep: *mut kmem_cache = null_mut();

const INOTIFY_MARK_FLAGS: u32 = FSNOTIFY_MARK_FLAG_EXCL_UNLINK | FSNOTIFY_MARK_FLAG_IN_ONESHOT;
const _: () = {
    assert!(IN_CLOEXEC == O_CLOEXEC);
    assert!(IN_NONBLOCK == O_NONBLOCK);
    assert!(IN_ACCESS == FS_ACCESS);
    assert!(IN_MODIFY == FS_MODIFY);
    assert!(IN_ATTRIB == FS_ATTRIB);
    assert!(IN_CLOSE_WRITE == FS_CLOSE_WRITE);
    assert!(IN_CLOSE_NOWRITE == FS_CLOSE_NOWRITE);
    assert!(IN_OPEN == FS_OPEN);
    assert!(IN_MOVED_FROM == FS_MOVED_FROM);
    assert!(IN_MOVED_TO == FS_MOVED_TO);
    assert!(IN_CREATE == FS_CREATE);
    assert!(IN_DELETE == FS_DELETE);
    assert!(IN_DELETE_SELF == FS_DELETE_SELF);
    assert!(IN_MOVE_SELF == FS_MOVE_SELF);
    assert!(IN_UNMOUNT == FS_UNMOUNT);
    assert!(IN_Q_OVERFLOW == FS_Q_OVERFLOW);
    assert!(IN_IGNORED == FS_IN_IGNORED);
    assert!(IN_ISDIR == FS_ISDIR);
    assert!(ALL_INOTIFY_BITS.count_ones() == 22);
};

#[inline]
fn is_err<T>(p: *const T) -> bool { p as usize >= usize::MAX - 4094 }
#[inline]
fn err_ptr<T>(error: c_int) -> *mut T { error as isize as *mut T }
#[inline]
unsafe fn group_data(g: *mut fsnotify_group) -> *mut fsnotify_group__bindgen_ty_1_inotify_group_private_data {
    addr_of_mut!((*g).__bindgen_anon_1.inotify_data)
}
#[inline]
unsafe fn event_info(event: *mut fsnotify_event) -> *mut inotify_event_info {
    event.cast::<u8>().sub(offset_of!(inotify_event_info, fse)).cast()
}
#[inline]
unsafe fn inode_mark(mark: *mut fsnotify_mark) -> *mut inotify_inode_mark {
    mark.cast::<u8>().sub(offset_of!(inotify_inode_mark, fsn_mark)).cast()
}
#[inline]
unsafe fn inotify_arg_to_mask(inode: *mut inode, arg: u32) -> u32 {
    let mut mask = FS_UNMOUNT;
    if rust_inotify_inode_is_dir(inode) { mask |= FS_EVENT_ON_CHILD; }
    mask | (arg & IN_ALL_EVENTS)
}
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

unsafe extern "C" fn inotify_poll(file: *mut file, wait: *mut poll_table_struct) -> __poll_t {
    let group = rust_inotify_file_group(file);
    rust_inotify_poll_wait(file, addr_of_mut!((*group).notification_waitq), wait);
    rust_inotify_spin_lock(addr_of_mut!((*group).notification_lock));
    let ret = if !rust_inotify_queue_empty(group) {
        RUST_INOTIFY_EPOLLIN | RUST_INOTIFY_EPOLLRDNORM
    } else { 0 };
    rust_inotify_spin_unlock(addr_of_mut!((*group).notification_lock));
    ret
}

unsafe fn round_event_name_len(event: *mut fsnotify_event) -> c_int {
    let len = (*event_info(event)).name_len;
    if len == 0 { return 0; }
    let align = size_of::<inotify_event>();
    (((len.wrapping_add(1) as usize).wrapping_add(align - 1) / align) * align) as c_int
}

unsafe fn get_one_event(group: *mut fsnotify_group, count: usize) -> *mut fsnotify_event {
    let event = fsnotify_peek_first_event(group);
    if event.is_null() { return null_mut(); }
    rust_inotify_debug_get(group, event);
    let event_size = size_of::<inotify_event>().wrapping_add(round_event_name_len(event) as usize);
    if event_size > count { return err_ptr(-(EINVAL as c_int)); }
    fsnotify_remove_first_event(group);
    event
}

unsafe fn copy_event_to_user(group: *mut fsnotify_group, event: *mut fsnotify_event, mut buf: *mut c_char) -> isize {
    rust_inotify_debug_copy(group, event);
    let info = event_info(event);
    let name_len = (*info).name_len as usize;
    let pad_name_len = round_event_name_len(event) as usize;
    let user_event = inotify_event {
        wd: (*info).wd, mask: inotify_mask_to_arg((*info).mask),
        cookie: (*info).sync_cookie, len: pad_name_len as u32,
        ..zeroed()
    };
    let mut event_size = size_of::<inotify_event>();
    if rust_inotify_copy_to_user(buf.cast(), addr_of!(user_event).cast(), event_size) != 0 {
        return -(EFAULT as isize);
    }
    buf = buf.wrapping_add(event_size);
    if pad_name_len != 0 {
        if rust_inotify_copy_to_user(buf.cast(), addr_of!((*info).name).cast(), name_len) != 0 {
            return -(EFAULT as isize);
        }
        buf = buf.wrapping_add(name_len);
        if rust_inotify_clear_user(buf.cast(), pad_name_len.wrapping_sub(name_len)) != 0 {
            return -(EFAULT as isize);
        }
        event_size = event_size.wrapping_add(pad_name_len);
    }
    event_size as isize
}

unsafe extern "C" fn inotify_read(file: *mut file, mut buf: *mut c_char, mut count: usize, _pos: *mut loff_t) -> isize {
    let group = rust_inotify_file_group(file);
    let start = buf;
    let mut ret: c_int;
    let mut wait: wait_queue_entry = zeroed();
    rust_inotify_init_wait(addr_of_mut!(wait));
    add_wait_queue(addr_of_mut!((*group).notification_waitq), addr_of_mut!(wait));
    loop {
        rust_inotify_spin_lock(addr_of_mut!((*group).notification_lock));
        let event = get_one_event(group, count);
        rust_inotify_spin_unlock(addr_of_mut!((*group).notification_lock));
        rust_inotify_debug_read(group, event);
        if !event.is_null() {
            ret = event as isize as c_int;
            if is_err(event) { break; }
            ret = copy_event_to_user(group, event, buf) as c_int;
            fsnotify_destroy_event(group, event);
            if ret < 0 { break; }
            buf = buf.wrapping_add(ret as usize);
            count = count.wrapping_sub(ret as usize);
            continue;
        }
        ret = -(EAGAIN as c_int);
        if rust_inotify_file_flags(file) & O_NONBLOCK != 0 { break; }
        ret = -(ERESTARTSYS as c_int);
        if rust_inotify_signal_pending() { break; }
        if start != buf { break; }
        wait_woken(addr_of_mut!(wait), TASK_INTERRUPTIBLE, c_long::MAX);
    }
    remove_wait_queue(addr_of_mut!((*group).notification_waitq), addr_of_mut!(wait));
    if start != buf && ret != -(EFAULT as c_int) {
        ret = (buf as usize).wrapping_sub(start as usize) as c_int;
    }
    ret as isize
}

unsafe extern "C" fn inotify_release(_ignored: *mut inode, file: *mut file) -> c_int {
    let group = rust_inotify_file_group(file);
    rust_inotify_debug_release(group);
    fsnotify_destroy_group(group);
    0
}

unsafe extern "C" fn inotify_ioctl(file: *mut file, cmd: u32, arg: c_ulong) -> c_long {
    let group = rust_inotify_file_group(file);
    rust_inotify_debug_ioctl(group, cmd);
    if cmd == FIONREAD {
        let mut send_len = 0usize;
        rust_inotify_spin_lock(addr_of_mut!((*group).notification_lock));
        let head = addr_of_mut!((*group).notification_list);
        let mut pos = (*head).next;
        while pos != head {
            let event = pos.cast::<u8>().sub(offset_of!(fsnotify_event, list)).cast();
            send_len = send_len.wrapping_add(size_of::<inotify_event>()).wrapping_add(round_event_name_len(event) as usize);
            pos = (*pos).next;
        }
        rust_inotify_spin_unlock(addr_of_mut!((*group).notification_lock));
        return rust_inotify_put_int(arg as *mut c_int, send_len as c_int) as c_long;
    }
    #[cfg(CONFIG_CHECKPOINT_RESTORE)]
    if cmd == RUST_INOTIFY_SETNEXTWD {
        if arg < 1 || arg > c_int::MAX as c_ulong { return -(EINVAL as c_long); }
        let data = group_data(group);
        rust_inotify_spin_lock(addr_of_mut!((*data).idr_lock));
        rust_inotify_idr_cursor(addr_of_mut!((*data).idr), arg as u32);
        rust_inotify_spin_unlock(addr_of_mut!((*data).idr_lock));
        return 0;
    }
    -(ENOTTY as c_long)
}

struct InotifyFops(file_operations);
unsafe impl Sync for InotifyFops {}
static INOTIFY_FOPS: InotifyFops = InotifyFops(file_operations {
    #[cfg(CONFIG_PROC_FS)]
    show_fdinfo: Some(inotify_show_fdinfo),
    poll: Some(inotify_poll), read: Some(inotify_read), fasync: Some(fsnotify_fasync),
    release: Some(inotify_release), unlocked_ioctl: Some(inotify_ioctl),
    compat_ioctl: Some(inotify_ioctl), llseek: Some(noop_llseek),
    ..unsafe { zeroed() }
});

unsafe fn inotify_find_inode(dirname: *const c_char, path: *mut path, flags: u32, mask: u64) -> c_int {
    let mut error = rust_inotify_user_path(dirname, flags, path);
    if error != 0 { return error; }
    error = rust_inotify_path_permission(path, MAY_READ as c_int);
    if error != 0 { path_put(path); return error; }
    error = rust_inotify_security_path(path, mask);
    if error != 0 { path_put(path); }
    error
}

unsafe fn inotify_add_to_idr(idr: *mut idr, lock: *mut spinlock_t, mark: *mut inotify_inode_mark) -> c_int {
    idr_preload(RUST_INOTIFY_GFP_KERNEL);
    rust_inotify_spin_lock(lock);
    let ret = idr_alloc_cyclic(idr, mark.cast(), 1, 0, RUST_INOTIFY_GFP_NOWAIT);
    if ret >= 0 {
        (*mark).wd = ret;
        fsnotify_get_mark(addr_of_mut!((*mark).fsn_mark));
    }
    rust_inotify_spin_unlock(lock);
    rust_inotify_idr_preload_end();
    if ret < 0 { ret } else { 0 }
}

unsafe fn inotify_idr_find_locked(group: *mut fsnotify_group, wd: c_int) -> *mut inotify_inode_mark {
    let data = group_data(group);
    rust_inotify_assert_locked(addr_of_mut!((*data).idr_lock));
    let mark: *mut inotify_inode_mark = idr_find(addr_of_mut!((*data).idr), wd as c_ulong).cast();
    if !mark.is_null() {
        fsnotify_get_mark(addr_of_mut!((*mark).fsn_mark));
        if rust_inotify_refcount(addr_of_mut!((*mark).fsn_mark.refcnt)) < 2 { kernel::bindings::BUG(); }
    }
    mark
}

unsafe fn inotify_idr_find(group: *mut fsnotify_group, wd: c_int) -> *mut inotify_inode_mark {
    let lock = addr_of_mut!((*group_data(group)).idr_lock);
    rust_inotify_spin_lock(lock);
    let mark = inotify_idr_find_locked(group, wd);
    rust_inotify_spin_unlock(lock);
    mark
}

unsafe fn inotify_remove_from_idr(group: *mut fsnotify_group, mark: *mut inotify_inode_mark) {
    let data = group_data(group);
    rust_inotify_spin_lock(addr_of_mut!((*data).idr_lock));
    let wd = (*mark).wd;
    let mut found: *mut inotify_inode_mark = null_mut();
    if wd == -1 {
        rust_inotify_warn_no_wd(mark);
    } else {
        found = inotify_idr_find_locked(group, wd);
        if found.is_null() {
            rust_inotify_warn_missing(mark);
        } else if found != mark {
            rust_inotify_warn_mismatch(mark, found);
        } else {
            if rust_inotify_refcount(addr_of_mut!((*mark).fsn_mark.refcnt)) < 2 {
                rust_inotify_bad_refcount_log(mark);
                kernel::bindings::BUG();
            }
            idr_remove(addr_of_mut!((*data).idr), wd as c_ulong);
            fsnotify_put_mark(addr_of_mut!((*mark).fsn_mark));
        }
    }
    (*mark).wd = -1;
    rust_inotify_spin_unlock(addr_of_mut!((*data).idr_lock));
    if !found.is_null() { fsnotify_put_mark(addr_of_mut!((*found).fsn_mark)); }
}

#[no_mangle]
pub unsafe extern "C" fn inotify_ignored_and_remove_idr(mark: *mut fsnotify_mark, group: *mut fsnotify_group) {
    inotify_handle_inode_event(mark, FS_IN_IGNORED, null_mut(), null_mut(), null(), 0);
    inotify_remove_from_idr(group, inode_mark(mark));
    rust_inotify_dec_watches((*group_data(group)).ucounts);
}

unsafe fn inotify_update_existing_watch(group: *mut fsnotify_group, inode: *mut inode, arg: u32) -> c_int {
    let mark = fsnotify_find_mark(inode.cast(), FSNOTIFY_OBJ_TYPE_INODE as u32, group);
    if mark.is_null() { return -(ENOENT as c_int); }
    let ret;
    if arg & IN_MASK_CREATE != 0 {
        ret = -(EEXIST as c_int);
    } else {
        rust_inotify_spin_lock(addr_of_mut!((*mark).lock));
        if arg & IN_MASK_ADD == 0 {
            (*mark).mask = 0;
            (*mark).flags &= !INOTIFY_MARK_FLAGS;
        }
        (*mark).mask |= inotify_arg_to_mask(inode, arg);
        (*mark).flags |= inotify_arg_to_flags(arg);
        rust_inotify_spin_unlock(addr_of_mut!((*mark).lock));
        fsnotify_recalc_mask((*mark).connector);
        ret = (*inode_mark(mark)).wd;
    }
    fsnotify_put_mark(mark);
    ret
}

unsafe fn inotify_new_watch(group: *mut fsnotify_group, inode: *mut inode, arg: u32) -> c_int {
    let mark = rust_inotify_alloc_mark(inotify_inode_mark_cachep);
    if mark.is_null() { return -(ENOMEM as c_int); }
    let fsn_mark = addr_of_mut!((*mark).fsn_mark);
    fsnotify_init_mark(fsn_mark, group);
    (*fsn_mark).mask = inotify_arg_to_mask(inode, arg);
    (*fsn_mark).flags = inotify_arg_to_flags(arg);
    (*mark).wd = -1;
    let data = group_data(group);
    let mut ret = inotify_add_to_idr(addr_of_mut!((*data).idr), addr_of_mut!((*data).idr_lock), mark);
    if ret == 0 {
        if rust_inotify_inc_watches((*data).ucounts).is_null() {
            inotify_remove_from_idr(group, mark);
            ret = -(ENOSPC as c_int);
        } else {
            ret = fsnotify_add_mark_locked(fsn_mark, inode.cast(), FSNOTIFY_OBJ_TYPE_INODE as u32, 0);
            if ret != 0 {
                inotify_remove_from_idr(group, mark);
                rust_inotify_dec_watches((*data).ucounts);
            } else {
                ret = (*mark).wd;
            }
        }
    }
    fsnotify_put_mark(fsn_mark);
    ret
}

unsafe fn inotify_update_watch(group: *mut fsnotify_group, inode: *mut inode, arg: u32) -> c_int {
    rust_inotify_group_lock(group);
    let mut ret = inotify_update_existing_watch(group, inode, arg);
    if ret == -(ENOENT as c_int) { ret = inotify_new_watch(group, inode, arg); }
    rust_inotify_group_unlock(group);
    ret
}

unsafe fn inotify_new_group(max_events: u32) -> *mut fsnotify_group {
    let group = fsnotify_alloc_group(addr_of!(inotify_fsnotify_ops), FSNOTIFY_GROUP_USER as c_int);
    if is_err(group) { return group; }
    let overflow = rust_inotify_alloc_overflow();
    if overflow.is_null() {
        fsnotify_destroy_group(group);
        return err_ptr(-(ENOMEM as c_int));
    }
    (*group).overflow_event = addr_of_mut!((*overflow).fse);
    rust_inotify_init_event((*group).overflow_event);
    (*overflow).mask = FS_Q_OVERFLOW;
    (*overflow).wd = -1;
    (*overflow).sync_cookie = 0;
    (*overflow).name_len = 0;
    (*group).max_events = max_events;
    (*group).memcg = rust_inotify_current_memcg();
    let data = group_data(group);
    rust_inotify_spin_init(addr_of_mut!((*data).idr_lock));
    rust_inotify_idr_init(addr_of_mut!((*data).idr));
    (*data).ucounts = rust_inotify_inc_instances();
    if (*data).ucounts.is_null() {
        fsnotify_destroy_group(group);
        return err_ptr(-(EMFILE as c_int));
    }
    group
}

unsafe fn do_inotify_init(flags: c_int) -> c_int {
    if flags & !((IN_CLOEXEC | IN_NONBLOCK) as c_int) != 0 { return -(EINVAL as c_int); }
    let group = inotify_new_group(inotify_max_queued_events as u32);
    if is_err(group) { return group as isize as c_int; }
    let ret = anon_inode_getfd(c"inotify".as_ptr().cast(), addr_of!(INOTIFY_FOPS.0), group.cast(), O_RDONLY as c_int | flags);
    if ret < 0 { fsnotify_destroy_group(group); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn rust_inotify_init1(flags: c_int) -> c_long { do_inotify_init(flags) as c_long }
#[no_mangle]
pub unsafe extern "C" fn rust_inotify_init() -> c_long { do_inotify_init(0) as c_long }

// Mirrors CLASS(fd, f): every return after fdget drops exactly the borrowed or
// cloned fd reference represented by the configured struct fd.
struct InotifyFd(fd);
impl Drop for InotifyFd { fn drop(&mut self) { unsafe { rust_inotify_fdput(self.0); } } }

#[no_mangle]
pub unsafe extern "C" fn rust_inotify_add_watch(fd: c_int, pathname: *const c_char, mask: u32) -> c_long {
    if mask & !ALL_INOTIFY_BITS != 0 || mask & ALL_INOTIFY_BITS == 0 { return -(EINVAL as c_long); }
    let f = InotifyFd(fdget(fd as u32));
    if f.0.word == 0 { return -(EBADF as c_long); }
    if mask & IN_MASK_ADD != 0 && mask & IN_MASK_CREATE != 0 { return -(EINVAL as c_long); }
    let file = rust_inotify_fd_file(f.0);
    if rust_inotify_file_ops(file) != addr_of!(INOTIFY_FOPS.0) { return -(EINVAL as c_long); }
    let mut flags = 0;
    if mask & IN_DONT_FOLLOW == 0 { flags |= RUST_INOTIFY_LOOKUP_FOLLOW; }
    if mask & IN_ONLYDIR != 0 { flags |= RUST_INOTIFY_LOOKUP_DIRECTORY; }
    let mut path: path = zeroed();
    let ret = inotify_find_inode(pathname, addr_of_mut!(path), flags, (mask & IN_ALL_EVENTS) as u64);
    if ret != 0 { return ret as c_long; }
    let inode = rust_inotify_path_inode(addr_of_mut!(path));
    let group = rust_inotify_file_group(file);
    let ret = inotify_update_watch(group, inode, mask);
    path_put(addr_of_mut!(path));
    ret as c_long
}

#[no_mangle]
pub unsafe extern "C" fn rust_inotify_rm_watch(fd: c_int, wd: i32) -> c_long {
    let f = InotifyFd(fdget(fd as u32));
    if f.0.word == 0 { return -(EBADF as c_long); }
    let file = rust_inotify_fd_file(f.0);
    if rust_inotify_file_ops(file) != addr_of!(INOTIFY_FOPS.0) { return -(EINVAL as c_long); }
    let group = rust_inotify_file_group(file);
    let mark = inotify_idr_find(group, wd);
    if mark.is_null() { return -(EINVAL as c_long); }
    fsnotify_destroy_mark(addr_of_mut!((*mark).fsn_mark), group);
    fsnotify_put_mark(addr_of_mut!((*mark).fsn_mark));
    0
}

#[cfg(CONFIG_SYSCTL)]
static mut it_zero: c_long = 0;
#[cfg(CONFIG_SYSCTL)]
static mut it_int_max: c_long = c_int::MAX as c_long;
#[cfg(CONFIG_SYSCTL)]
#[link_section = ".data..ro_after_init"]
static mut inotify_table: [ctl_table; 3] = unsafe { zeroed() };

#[cfg(CONFIG_SYSCTL)]
#[link_section = ".init.text"]
unsafe fn inotify_sysctls_init() {
    let table = addr_of_mut!(inotify_table).cast::<ctl_table>();
    *table = ctl_table {
        procname: c"max_user_instances".as_ptr().cast(), data: rust_inotify_ucount_max(UCOUNT_INOTIFY_INSTANCES).cast(),
        maxlen: size_of::<c_long>() as c_int, mode: 0o644, proc_handler: Some(proc_doulongvec_minmax),
        extra1: addr_of_mut!(it_zero).cast(), extra2: addr_of_mut!(it_int_max).cast(), ..zeroed()
    };
    *table.add(1) = ctl_table {
        procname: c"max_user_watches".as_ptr().cast(), data: rust_inotify_ucount_max(UCOUNT_INOTIFY_WATCHES).cast(),
        maxlen: size_of::<c_long>() as c_int, mode: 0o644, proc_handler: Some(proc_doulongvec_minmax),
        extra1: addr_of_mut!(it_zero).cast(), extra2: addr_of_mut!(it_int_max).cast(), ..zeroed()
    };
    *table.add(2) = ctl_table {
        procname: c"max_queued_events".as_ptr().cast(), data: addr_of_mut!(inotify_max_queued_events).cast(),
        maxlen: size_of::<c_int>() as c_int, mode: 0o644, proc_handler: Some(proc_dointvec_minmax),
        extra1: addr_of!(sysctl_vals).cast_mut().cast(), ..zeroed()
    };
    register_sysctl_sz(c"fs/inotify".as_ptr().cast(), table, 3);
}

#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_inotify_user_setup() -> c_int {
    let mut si: sysinfo = zeroed();
    si_meminfo(addr_of_mut!(si));
    let watches = ((si.totalram.wrapping_sub(si.totalhigh) / 100).wrapping_shl(RUST_INOTIFY_PAGE_SHIFT))
        / RUST_INOTIFY_WATCH_COST as c_ulong;
    let watches = watches.clamp(8192, 1048576);
    inotify_inode_mark_cachep = rust_inotify_create_cache();
    inotify_max_queued_events = 16384;
    *rust_inotify_ucount_max(UCOUNT_INOTIFY_INSTANCES) = 128;
    *rust_inotify_ucount_max(UCOUNT_INOTIFY_WATCHES) = watches as c_long;
    #[cfg(CONFIG_SYSCTL)]
    inotify_sysctls_init();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
