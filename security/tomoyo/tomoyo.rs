// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of security/tomoyo/tomoyo.c. */

// Kernel headers and symbols referenced below are supplied by the surrounding
// kernel translation unit.

pub unsafe fn tomoyo_domain() -> *mut tomoyo_domain_info {
    let s = tomoyo_task(current);

    if !(*s).old_domain_info.is_null() && !(*current).in_execve {
        (*(*s).old_domain_info).users.dec();
        (*s).old_domain_info = core::ptr::null_mut();
    }
    (*s).domain_info
}

unsafe fn tomoyo_cred_prepare(
    _new: *mut cred,
    _old: *const cred,
    _gfp: gfp_t,
) -> i32 {
    // Restore old_domain_info saved by previous execve() request.
    let s = tomoyo_task(current);

    if !(*s).old_domain_info.is_null() && !(*current).in_execve {
        (*(*s).domain_info).users.dec();
        (*s).domain_info = (*s).old_domain_info;
        (*s).old_domain_info = core::ptr::null_mut();
    }
    0
}

unsafe fn tomoyo_bprm_committed_creds(_bprm: *const linux_binprm) {
    // Clear old_domain_info saved by execve() request.
    let s = tomoyo_task(current);

    (*(*s).old_domain_info).users.dec();
    (*s).old_domain_info = core::ptr::null_mut();
}

#[cfg(not(CONFIG_SECURITY_TOMOYO_OMIT_USERSPACE_LOADER))]
unsafe fn tomoyo_bprm_creds_for_exec(bprm: *mut linux_binprm) -> i32 {
    // Load policy if /sbin/tomoyo-init exists and /sbin/init is requested for the first time.
    if !tomoyo_policy_loaded {
        tomoyo_load_policy((*bprm).filename);
    }
    0
}

unsafe fn tomoyo_bprm_check_security(bprm: *mut linux_binprm) -> i32 {
    let s = tomoyo_task(current);

    if (*s).old_domain_info.is_null() {
        let idx = tomoyo_read_lock();
        let err = tomoyo_find_next_domain(bprm);
        tomoyo_read_unlock(idx);
        return err;
    }
    tomoyo_check_open_permission((*s).domain_info, &(*(*bprm).file).f_path, O_RDONLY)
}

unsafe fn tomoyo_inode_getattr(path: *const path) -> i32 {
    tomoyo_path_perm(TOMOYO_TYPE_GETATTR, path, core::ptr::null())
}

unsafe fn tomoyo_path_truncate(path: *const path) -> i32 {
    tomoyo_path_perm(TOMOYO_TYPE_TRUNCATE, path, core::ptr::null())
}

unsafe fn tomoyo_file_truncate(file: *mut file) -> i32 {
    tomoyo_path_truncate(&(*file).f_path)
}

unsafe fn tomoyo_path_unlink(parent: *const path, dentry: *mut dentry) -> i32 {
    let path = path { mnt: (*parent).mnt, dentry };
    tomoyo_path_perm(TOMOYO_TYPE_UNLINK, &path, core::ptr::null())
}

unsafe fn tomoyo_path_mkdir(parent: *const path, dentry: *mut dentry, mode: umode_t) -> i32 {
    let path = path { mnt: (*parent).mnt, dentry };
    tomoyo_path_number_perm(TOMOYO_TYPE_MKDIR, &path, mode & S_IALLUGO)
}

unsafe fn tomoyo_path_rmdir(parent: *const path, dentry: *mut dentry) -> i32 {
    let path = path { mnt: (*parent).mnt, dentry };
    tomoyo_path_perm(TOMOYO_TYPE_RMDIR, &path, core::ptr::null())
}

unsafe fn tomoyo_path_symlink(parent: *const path, dentry: *mut dentry, old_name: *const i8) -> i32 {
    let path = path { mnt: (*parent).mnt, dentry };
    tomoyo_path_perm(TOMOYO_TYPE_SYMLINK, &path, old_name)
}

unsafe fn tomoyo_path_mknod(parent: *const path, dentry: *mut dentry, mode: umode_t, dev: u32) -> i32 {
    let path = path { mnt: (*parent).mnt, dentry };
    let mut ty = TOMOYO_TYPE_CREATE;
    let perm = mode & S_IALLUGO;
    match mode & S_IFMT {
        S_IFCHR => ty = TOMOYO_TYPE_MKCHAR,
        S_IFBLK => ty = TOMOYO_TYPE_MKBLOCK,
        _ => return tomoyo_path_number_perm(match mode & S_IFMT {
            S_IFIFO => TOMOYO_TYPE_MKFIFO,
            S_IFSOCK => TOMOYO_TYPE_MKSOCK,
            _ => ty,
        }, &path, perm),
    }
    tomoyo_mkdev_perm(ty, &path, perm, dev)
}

unsafe fn tomoyo_path_link(old_dentry: *mut dentry, new_dir: *const path, new_dentry: *mut dentry) -> i32 {
    let path1 = path { mnt: (*new_dir).mnt, dentry: old_dentry };
    let path2 = path { mnt: (*new_dir).mnt, dentry: new_dentry };
    tomoyo_path2_perm(TOMOYO_TYPE_LINK, &path1, &path2)
}

unsafe fn tomoyo_path_rename(old_parent: *const path, old_dentry: *mut dentry, new_parent: *const path, new_dentry: *mut dentry, flags: u32) -> i32 {
    let path1 = path { mnt: (*old_parent).mnt, dentry: old_dentry };
    let path2 = path { mnt: (*new_parent).mnt, dentry: new_dentry };
    if flags & RENAME_EXCHANGE != 0 {
        let err = tomoyo_path2_perm(TOMOYO_TYPE_RENAME, &path2, &path1);
        if err != 0 { return err; }
    }
    tomoyo_path2_perm(TOMOYO_TYPE_RENAME, &path1, &path2)
}

unsafe fn tomoyo_file_fcntl(file: *mut file, cmd: u32, arg: usize) -> i32 {
    if !(cmd == F_SETFL && ((arg ^ (*file).f_flags as usize) & O_APPEND as usize) != 0) { return 0; }
    tomoyo_check_open_permission(tomoyo_domain(), &(*file).f_path, O_WRONLY | (arg as u32 & O_APPEND))
}

unsafe fn tomoyo_file_open(f: *mut file) -> i32 {
    if (*f).f_flags & __FMODE_EXEC != 0 { return 0; }
    tomoyo_check_open_permission(tomoyo_domain(), &(*f).f_path, (*f).f_flags)
}

unsafe fn tomoyo_file_ioctl(file: *mut file, _cmd: u32, cmd: usize) -> i32 {
    tomoyo_path_number_perm(TOMOYO_TYPE_IOCTL, &(*file).f_path, cmd as u32)
}

unsafe fn tomoyo_path_chmod(path: *const path, mode: umode_t) -> i32 {
    tomoyo_path_number_perm(TOMOYO_TYPE_CHMOD, path, mode & S_IALLUGO)
}

unsafe fn tomoyo_path_chown(path: *const path, uid: kuid_t, gid: kgid_t) -> i32 {
    let mut error = 0;
    if uid_valid(uid) { error = tomoyo_path_number_perm(TOMOYO_TYPE_CHOWN, path, from_kuid(&init_user_ns, uid)); }
    if error == 0 && gid_valid(gid) { error = tomoyo_path_number_perm(TOMOYO_TYPE_CHGRP, path, from_kgid(&init_user_ns, gid)); }
    error
}

unsafe fn tomoyo_path_chroot(path: *const path) -> i32 { tomoyo_path_perm(TOMOYO_TYPE_CHROOT, path, core::ptr::null()) }
unsafe fn tomoyo_sb_mount(dev_name: *const i8, path: *const path, ty: *const i8, flags: usize, data: *mut core::ffi::c_void) -> i32 { tomoyo_mount_permission(dev_name, path, ty, flags, data) }
unsafe fn tomoyo_sb_umount(mnt: *mut vfsmount, _flags: i32) -> i32 { let path = path { mnt, dentry: (*mnt).mnt_root }; tomoyo_path_perm(TOMOYO_TYPE_UMOUNT, &path, core::ptr::null()) }
unsafe fn tomoyo_sb_pivotroot(old_path: *const path, new_path: *const path) -> i32 { tomoyo_path2_perm(TOMOYO_TYPE_PIVOT_ROOT, new_path, old_path) }
unsafe fn tomoyo_socket_listen(sock: *mut socket, _backlog: i32) -> i32 { tomoyo_socket_listen_permission(sock) }
unsafe fn tomoyo_socket_connect(sock: *mut socket, addr: *mut sockaddr, addr_len: i32) -> i32 { tomoyo_socket_connect_permission(sock, addr, addr_len) }
unsafe fn tomoyo_socket_bind(sock: *mut socket, addr: *mut sockaddr, addr_len: i32) -> i32 { tomoyo_socket_bind_permission(sock, addr, addr_len) }
unsafe fn tomoyo_socket_sendmsg(sock: *mut socket, msg: *mut msghdr, size: i32) -> i32 { tomoyo_socket_sendmsg_permission(sock, msg, size) }

pub static mut tomoyo_blob_sizes: lsm_blob_sizes = lsm_blob_sizes { lbs_task: core::mem::size_of::<tomoyo_task>() };

unsafe fn tomoyo_task_alloc(task: *mut task_struct, _clone_flags: u64) -> i32 {
    let old = tomoyo_task(current);
    let new = tomoyo_task(task);
    (*new).domain_info = (*old).domain_info;
    (*(*new).domain_info).users.inc();
    (*new).old_domain_info = core::ptr::null_mut();
    0
}

unsafe fn tomoyo_task_free(task: *mut task_struct) {
    let s = tomoyo_task(task);
    if !(*s).domain_info.is_null() { (*(*s).domain_info).users.dec(); (*s).domain_info = core::ptr::null_mut(); }
    if !(*s).old_domain_info.is_null() { (*(*s).old_domain_info).users.dec(); (*s).old_domain_info = core::ptr::null_mut(); }
}

static tomoyo_lsmid: lsm_id = lsm_id { name: b"tomoyo\0".as_ptr() as *const i8, id: LSM_ID_TOMOYO };
pub static mut tomoyo_enabled: i32 = 1;

unsafe fn tomoyo_init() -> i32 {
    let s = tomoyo_task(current);
    security_add_hooks(tomoyo_hooks.as_ptr(), tomoyo_hooks.len(), &tomoyo_lsmid);
    pr_info!("TOMOYO Linux initialized\n");
    (*s).domain_info = &mut tomoyo_kernel_domain;
    (*tomoyo_kernel_domain.users).inc();
    (*s).old_domain_info = core::ptr::null_mut();
    tomoyo_mm_init();
    0
}

// LSM_HOOK_INIT/DEFINE_SRCU/DEFINE_LSM are kernel registration macros. Their
// declarations and hook ordering are preserved here for the surrounding ABI.
extern "C" {
    static mut tomoyo_hooks: [security_hook_list; 29];
    fn tomoyo_interface_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
