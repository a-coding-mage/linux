// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the kernel and neighboring translation units.

#[cfg(any(CONFIG_PROC_FS, CONFIG_INOTIFY_USER, CONFIG_FANOTIFY))]
unsafe fn show_fdinfo(
    m: *mut seq_file,
    f: *mut file,
    show: unsafe fn(*mut seq_file, *mut fsnotify_mark),
) {
    let group = (*f).private_data as *mut fsnotify_group;
    let mut mark: *mut fsnotify_mark;

    fsnotify_group_lock(group);
    list_for_each_entry!(mark, &(*group).marks_list, g_list, {
        show(m, mark);
        if seq_has_overflowed(m) {
            break;
        }
    });
    fsnotify_group_unlock(group);
}

#[cfg(CONFIG_EXPORTFS)]
unsafe fn show_mark_fhandle(m: *mut seq_file, inode: *mut inode) {
    // DEFINE_FLEX(struct file_handle, f, f_handle, handle_bytes, MAX_HANDLE_SZ)
    // is represented by the corresponding externally supplied file_handle
    // storage and handle fields.
    let mut f: file_handle = core::mem::zeroed();
    let mut size: i32;
    let ret: i32;

    size = ((*f.handle_bytes) >> 2) as i32;

    if !super_trylock_shared((*inode).i_sb) {
        return;
    }

    ret = exportfs_encode_fid(inode, f.f_handle.as_mut_ptr() as *mut fid, &mut size);
    up_read(&(*inode).i_sb.s_umount);

    if ret == FILEID_INVALID || ret < 0 {
        return;
    }

    f.handle_type = ret as u32;
    f.handle_bytes = (size as usize * core::mem::size_of::<u32>()) as u32;

    seq_printf!(m, "fhandle-bytes:{:x} fhandle-type:{:x} f_handle:", f.handle_bytes, f.handle_type);

    let mut i = 0;
    while i < f.handle_bytes {
        seq_printf!(m, "{:02x}", f.f_handle[i as usize] as i32);
        i += 1;
    }
}

#[cfg(not(CONFIG_EXPORTFS))]
unsafe fn show_mark_fhandle(_m: *mut seq_file, _inode: *mut inode) {}

#[cfg(CONFIG_INOTIFY_USER)]
unsafe fn inotify_fdinfo(m: *mut seq_file, mark: *mut fsnotify_mark) {
    let inode_mark: *mut inotify_inode_mark;
    let inode: *mut inode;

    if (*(*mark).connector).type_ != FSNOTIFY_OBJ_TYPE_INODE {
        return;
    }

    inode_mark = container_of!(mark, inotify_inode_mark, fsn_mark);
    inode = igrab(fsnotify_conn_inode((*mark).connector));
    if !inode.is_null() {
        seq_printf!(m, "inotify wd:{:x} ino:{:x} sdev:{:x} mask:{:x} ignored_mask:0 ",
            (*inode_mark).wd, (*inode).i_ino, (*(*inode).i_sb).s_dev,
            inotify_mark_user_mask(mark));
        show_mark_fhandle(m, inode);
        seq_putc!(m, '\n');
        iput(inode);
    }
}

#[cfg(CONFIG_INOTIFY_USER)]
pub unsafe fn inotify_show_fdinfo(m: *mut seq_file, f: *mut file) {
    show_fdinfo(m, f, inotify_fdinfo);
}

#[cfg(CONFIG_FANOTIFY)]
unsafe fn fanotify_fdinfo(m: *mut seq_file, mark: *mut fsnotify_mark) {
    let mflags: u32 = fanotify_mark_user_flags(mark);
    let mut inode: *mut inode;

    if (*(*mark).connector).type_ == FSNOTIFY_OBJ_TYPE_INODE {
        inode = igrab(fsnotify_conn_inode((*mark).connector));
        if inode.is_null() {
            return;
        }
        seq_printf!(m, "fanotify ino:{:x} sdev:{:x} mflags:{:x} mask:{:x} ignored_mask:{:x} ",
            (*inode).i_ino, (*(*inode).i_sb).s_dev, mflags, (*mark).mask, (*mark).ignore_mask);
        show_mark_fhandle(m, inode);
        seq_putc!(m, '\n');
        iput(inode);
    } else if (*(*mark).connector).type_ == FSNOTIFY_OBJ_TYPE_VFSMOUNT {
        let mnt: *mut mount = fsnotify_conn_mount((*mark).connector);
        seq_printf!(m, "fanotify mnt_id:{:x} mflags:{:x} mask:{:x} ignored_mask:{:x}\n",
            (*mnt).mnt_id, mflags, (*mark).mask, (*mark).ignore_mask);
    } else if (*(*mark).connector).type_ == FSNOTIFY_OBJ_TYPE_SB {
        let sb: *mut super_block = fsnotify_conn_sb((*mark).connector);
        seq_printf!(m, "fanotify sdev:{:x} mflags:{:x} mask:{:x} ignored_mask:{:x}\n",
            (*sb).s_dev, mflags, (*mark).mask, (*mark).ignore_mask);
    } else if (*(*mark).connector).type_ == FSNOTIFY_OBJ_TYPE_MNTNS {
        let mnt_ns: *mut mnt_namespace = fsnotify_conn_mntns((*mark).connector);
        seq_printf!(m, "fanotify mnt_ns:{} mflags:{:x} mask:{:x} ignored_mask:{:x}\n",
            (*mnt_ns).ns.inum, mflags, (*mark).mask, (*mark).ignore_mask);
    }
}

#[cfg(CONFIG_FANOTIFY)]
pub unsafe fn fanotify_show_fdinfo(m: *mut seq_file, f: *mut file) {
    let group = (*f).private_data as *mut fsnotify_group;

    seq_printf!(m, "fanotify flags:{:x} event-flags:{:x}\n",
        (*group).fanotify_data.flags & FANOTIFY_INIT_FLAGS,
        (*group).fanotify_data.f_flags);

    show_fdinfo(m, f, fanotify_fdinfo);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
