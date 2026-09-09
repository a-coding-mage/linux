// SPDX-License-Identifier: GPL-2.0
/*
 * fs/proc_namespace.c - handling of /proc/<pid>/{mounts,mountinfo,mountstats}
 */

// Kernel dependencies supplied by other translation units.

#[repr(C)]
struct proc_fs_opts {
    flag: c_int,
    str_: *const c_char,
}

unsafe fn mounts_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let m = (*file).private_data as *mut seq_file;
    let p = (*m).private as *mut proc_mounts;
    let ns = (*p).ns;
    let mut res: __poll_t = EPOLLIN | EPOLLRDNORM;
    let event: c_int;

    poll_wait(file, &mut (*p).ns.poll, wait);

    event = READ_ONCE((*ns).event);
    if (*m).poll_event != event {
        (*m).poll_event = event;
        res |= EPOLLERR | EPOLLPRI;
    }

    res
}

unsafe fn show_sb_opts(m: *mut seq_file, sb: *mut super_block) -> c_int {
    let fs_opts = [
        proc_fs_opts { flag: SB_SYNCHRONOUS, str_: c",sync\0".as_ptr() as *const c_char },
        proc_fs_opts { flag: SB_DIRSYNC, str_: c",dirsync\0".as_ptr() as *const c_char },
        proc_fs_opts { flag: SB_MANDLOCK, str_: c",mand\0".as_ptr() as *const c_char },
        proc_fs_opts { flag: SB_LAZYTIME, str_: c",lazytime\0".as_ptr() as *const c_char },
        proc_fs_opts { flag: 0, str_: core::ptr::null() },
    ];
    let mut fs_infop = fs_opts.as_ptr();
    while (*fs_infop).flag != 0 {
        if (*sb).s_flags & (*fs_infop).flag != 0 {
            seq_puts(m, (*fs_infop).str_);
        }
        fs_infop = fs_infop.add(1);
    }
    security_sb_show_options(m, sb)
}

unsafe fn show_vfsmnt_opts(m: *mut seq_file, mnt: *mut vfsmount) {
    let mnt_opts = [
        proc_fs_opts { flag: MNT_NOSUID, str_: c",nosuid\0".as_ptr() as *const c_char },
        proc_fs_opts { flag: MNT_NODEV, str_: c",nodev\0".as_ptr() as *const c_char },
        proc_fs_opts { flag: MNT_NOEXEC, str_: c",noexec\0".as_ptr() as *const c_char },
        proc_fs_opts { flag: MNT_NOATIME, str_: c",noatime\0".as_ptr() as *const c_char },
        proc_fs_opts { flag: MNT_NODIRATIME, str_: c",nodiratime\0".as_ptr() as *const c_char },
        proc_fs_opts { flag: MNT_RELATIME, str_: c",relatime\0".as_ptr() as *const c_char },
        proc_fs_opts { flag: MNT_NOSYMFOLLOW, str_: c",nosymfollow\0".as_ptr() as *const c_char },
        proc_fs_opts { flag: 0, str_: core::ptr::null() },
    ];
    let mut fs_infop = mnt_opts.as_ptr();
    while (*fs_infop).flag != 0 {
        if (*mnt).mnt_flags & (*fs_infop).flag != 0 {
            seq_puts(m, (*fs_infop).str_);
        }
        fs_infop = fs_infop.add(1);
    }
    if is_idmapped_mnt(mnt) {
        seq_puts(m, c",idmapped\0".as_ptr() as *const c_char);
    }
}

#[inline]
unsafe fn mangle(m: *mut seq_file, s: *const c_char) {
    seq_escape(m, s, c" \t\n\\#\0".as_ptr() as *const c_char);
}

unsafe fn show_type(m: *mut seq_file, sb: *mut super_block) {
    mangle(m, (*(*sb).s_type).name);
    if !(*sb).s_subtype.is_null() {
        seq_putc(m, b'.' as c_int);
        mangle(m, (*sb).s_subtype);
    }
}

unsafe fn show_vfsmnt(m: *mut seq_file, mnt: *mut vfsmount) -> c_int {
    let p = (*m).private as *mut proc_mounts;
    let r = real_mount(mnt);
    let mnt_path = path { dentry: (*mnt).mnt_root, mnt };
    let sb = (*mnt_path.dentry).d_sb;
    let mut err: c_int = 0;
    if let Some(show_devname) = (*(*sb).s_op).show_devname {
        err = show_devname(m, mnt_path.dentry);
        if err != 0 { return err; }
    } else { mangle(m, (*r).mnt_devname); }
    seq_putc(m, b' ' as c_int);
    err = seq_path_root(m, &mnt_path, &(*p).root, c" \t\n\\\0".as_ptr() as *const c_char);
    if err != 0 { return err; }
    seq_putc(m, b' ' as c_int);
    show_type(m, sb);
    seq_puts(m, if __mnt_is_readonly(mnt) { c" ro\0".as_ptr() } else { c" rw\0" });
    err = show_sb_opts(m, sb);
    if err != 0 { return err; }
    show_vfsmnt_opts(m, mnt);
    if let Some(show_options) = (*(*sb).s_op).show_options { err = show_options(m, mnt_path.dentry); }
    seq_puts(m, c" 0 0\n\0".as_ptr());
    err
}

unsafe fn show_mountinfo(m: *mut seq_file, mnt: *mut vfsmount) -> c_int {
    let p = (*m).private as *mut proc_mounts;
    let r = real_mount(mnt);
    let sb = (*mnt).mnt_sb;
    let mnt_path = path { dentry: (*mnt).mnt_root, mnt };
    seq_printf(m, c"%i %i %u:%u \0".as_ptr(), (*r).mnt_id, (*(*r).mnt_parent).mnt_id, MAJOR((*sb).s_dev), MINOR((*sb).s_dev));
    let mut err = show_path(m, (*mnt).mnt_root);
    if err != 0 { return err; }
    seq_putc(m, b' ' as c_int);
    err = seq_path_root(m, &mnt_path, &(*p).root, c" \t\n\\\0".as_ptr());
    if err != 0 { return err; }
    seq_puts(m, if (*mnt).mnt_flags & MNT_READONLY != 0 { c" ro\0".as_ptr() } else { c" rw\0".as_ptr() });
    show_vfsmnt_opts(m, mnt);
    if IS_MNT_SHARED(r) { seq_printf(m, c" shared:%i\0".as_ptr(), (*r).mnt_group_id); }
    if IS_MNT_SLAVE(r) {
        let master = (*(*r).mnt_master).mnt_group_id;
        let dom = get_dominating_id(r, &(*p).root);
        seq_printf(m, c" master:%i\0".as_ptr(), master);
        if dom != 0 && dom != master { seq_printf(m, c" propagate_from:%i\0".as_ptr(), dom); }
    }
    if IS_MNT_UNBINDABLE(r) { seq_puts(m, c" unbindable\0".as_ptr()); }
    seq_puts(m, c" - \0".as_ptr()); show_type(m, sb); seq_putc(m, b' ' as c_int);
    if let Some(show_devname) = (*(*sb).s_op).show_devname { err = show_devname(m, (*mnt).mnt_root); if err != 0 { return err; } } else { mangle(m, (*r).mnt_devname); }
    seq_puts(m, if sb_rdonly(sb) { c" ro\0".as_ptr() } else { c" rw\0".as_ptr() });
    err = show_sb_opts(m, sb); if err != 0 { return err; }
    if let Some(show_options) = (*(*sb).s_op).show_options { err = show_options(m, (*mnt).mnt_root); }
    seq_putc(m, b'\n' as c_int); err
}

unsafe fn show_vfsstat(m: *mut seq_file, mnt: *mut vfsmount) -> c_int {
    let p = (*m).private as *mut proc_mounts; let r = real_mount(mnt);
    let mnt_path = path { dentry: (*mnt).mnt_root, mnt }; let sb = (*mnt_path.dentry).d_sb;
    seq_puts(m, c"device \0".as_ptr());
    let mut err = if let Some(f) = (*(*sb).s_op).show_devname { f(m, mnt_path.dentry) } else { mangle(m, (*r).mnt_devname); 0 };
    if err != 0 { return err; }
    seq_puts(m, c" mounted on \0".as_ptr()); err = seq_path_root(m, &mnt_path, &(*p).root, c" \t\n\\\0".as_ptr()); if err != 0 { return err; }
    seq_putc(m, b' ' as c_int); seq_puts(m, c"with fstype \0".as_ptr()); show_type(m, sb);
    if let Some(f) = (*(*sb).s_op).show_stats { seq_putc(m, b' ' as c_int); err = f(m, mnt_path.dentry); }
    seq_putc(m, b'\n' as c_int); err
}

// The remaining open/release operations and file-operation tables retain the
// kernel's external types and callbacks; dependencies are supplied elsewhere.
unsafe fn mounts_open_common(inode: *mut inode, file: *mut file, show: unsafe fn(*mut seq_file, *mut vfsmount) -> c_int) -> c_int {
    let task = get_proc_task(inode);
    let mut nsproxy: *mut nsproxy;
    let mut ns: *mut mnt_namespace = core::ptr::null_mut();
    let mut root: path = core::mem::zeroed();
    let mut ret: c_int = -EINVAL;
    if task.is_null() { return ret; }
    task_lock(task);
    nsproxy = (*task).nsproxy;
    if nsproxy.is_null() || (*nsproxy).mnt_ns.is_null() {
        task_unlock(task); put_task_struct(task); return ret;
    }
    ns = (*nsproxy).mnt_ns;
    get_mnt_ns(ns);
    if (*task).real_fs.is_null() {
        task_unlock(task); put_task_struct(task); ret = -ENOENT; put_mnt_ns(ns); return ret;
    }
    get_fs_root((*task).real_fs, &mut root);
    task_unlock(task); put_task_struct(task);
    ret = seq_open_private(file, &mounts_op, core::mem::size_of::<proc_mounts>());
    if ret != 0 { path_put(&mut root); put_mnt_ns(ns); return ret; }
    let m = (*file).private_data as *mut seq_file;
    (*m).poll_event = (*ns).event;
    let p = (*m).private as *mut proc_mounts;
    (*p).ns = ns; (*p).root = root; (*p).show = Some(show);
    0
}
unsafe fn mounts_release(inode: *mut inode, file: *mut file) -> c_int {
    let m = (*file).private_data as *mut seq_file;
    let p = (*m).private as *mut proc_mounts;
    path_put(&mut (*p).root); put_mnt_ns((*p).ns); seq_release_private(inode, file)
}
unsafe fn mounts_open(inode: *mut inode, file: *mut file) -> c_int { mounts_open_common(inode, file, show_vfsmnt) }
unsafe fn mountinfo_open(inode: *mut inode, file: *mut file) -> c_int { mounts_open_common(inode, file, show_mountinfo) }
unsafe fn mountstats_open(inode: *mut inode, file: *mut file) -> c_int { mounts_open_common(inode, file, show_vfsstat) }

const proc_mounts_operations: file_operations = file_operations {
    open: Some(mounts_open), read_iter: Some(seq_read_iter), splice_read: Some(copy_splice_read),
    llseek: Some(seq_lseek), release: Some(mounts_release), poll: Some(mounts_poll),
};
const proc_mountinfo_operations: file_operations = file_operations {
    open: Some(mountinfo_open), read_iter: Some(seq_read_iter), splice_read: Some(copy_splice_read),
    llseek: Some(seq_lseek), release: Some(mounts_release), poll: Some(mounts_poll),
};
const proc_mountstats_operations: file_operations = file_operations {
    open: Some(mountstats_open), read_iter: Some(seq_read_iter), splice_read: Some(copy_splice_read),
    llseek: Some(seq_lseek), release: Some(mounts_release), poll: None,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
