// SPDX-License-Identifier: GPL-2.0-only
// Dependencies are supplied by the surrounding kernel translation.

pub unsafe fn set_fs_root(fs: *mut fs_struct, path: *const path) {
    let mut old_root: path;
    path_get(path);
    write_seqlock(&mut (*fs).seq);
    old_root = (*fs).root;
    (*fs).root = *path;
    write_sequnlock(&mut (*fs).seq);
    if !old_root.dentry.is_null() {
        path_put(&mut old_root);
    }
}

pub unsafe fn set_fs_pwd(fs: *mut fs_struct, path: *const path) {
    let mut old_pwd: path;
    path_get(path);
    write_seqlock(&mut (*fs).seq);
    old_pwd = (*fs).pwd;
    (*fs).pwd = *path;
    write_sequnlock(&mut (*fs).seq);
    if !old_pwd.dentry.is_null() {
        path_put(&mut old_pwd);
    }
}

#[inline]
unsafe fn replace_path(p: *mut path, old: *const path, new: *const path) -> i32 {
    if likely((*p).dentry != (*old).dentry || (*p).mnt != (*old).mnt) {
        return 0;
    }
    *p = *new;
    1
}

pub unsafe fn chroot_fs_refs(old_root: *const path, new_root: *const path) {
    let mut count = 0;
    read_lock(&mut tasklist_lock);
    for_each_process_thread!(g, p) {
        if ((*p).flags & (PF_KTHREAD | PF_EXITING | PF_DUMPCORE)) != 0 {
            continue;
        }
        task_lock(p);
        let fs = (*p).real_fs;
        if !fs.is_null() {
            let mut hits = 0;
            write_seqlock(&mut (*fs).seq);
            hits += replace_path(&mut (*fs).root, old_root, new_root);
            hits += replace_path(&mut (*fs).pwd, old_root, new_root);
            while hits > 0 {
                count += 1;
                path_get(new_root);
                hits -= 1;
            }
            write_sequnlock(&mut (*fs).seq);
        }
        task_unlock(p);
    }
    read_unlock(&mut tasklist_lock);
    while count > 0 {
        path_put(old_root);
        count -= 1;
    }
}

pub unsafe fn free_fs_struct(fs: *mut fs_struct) {
    path_put(&mut (*fs).root);
    path_put(&mut (*fs).pwd);
    kmem_cache_free(fs_cachep, fs as *mut _);
}

pub unsafe fn exit_fs(tsk: *mut task_struct) {
    let fs = (*tsk).real_fs;
    if !fs.is_null() {
        let kill;
        task_lock(tsk);
        read_seqlock_excl(&mut (*fs).seq);
        (*tsk).real_fs = core::ptr::null_mut();
        (*tsk).fs = core::ptr::null_mut();
        (*fs).users -= 1;
        kill = (*fs).users == 0;
        read_sequnlock_excl(&mut (*fs).seq);
        task_unlock(tsk);
        if kill { free_fs_struct(fs); }
    }
}

pub unsafe fn copy_fs_struct(old: *mut fs_struct) -> *mut fs_struct {
    let fs = kmem_cache_alloc(fs_cachep, GFP_KERNEL);
    /* We don't need to lock fs - think why ;-) */
    if !fs.is_null() {
        (*fs).users = 1;
        (*fs).in_exec = 0;
        seqlock_init(&mut (*fs).seq);
        (*fs).umask = (*old).umask;
        read_seqlock_excl(&mut (*old).seq);
        (*fs).root = (*old).root;
        path_get(&(*fs).root);
        (*fs).pwd = (*old).pwd;
        path_get(&(*fs).pwd);
        read_sequnlock_excl(&mut (*old).seq);
    }
    fs
}

pub unsafe fn unshare_fs_struct() -> i32 {
    let fs = (*current).real_fs;
    let new_fs = copy_fs_struct(fs);
    if new_fs.is_null() { return -ENOMEM; }
    task_lock(current);
    read_seqlock_excl(&mut (*fs).seq);
    VFS_WARN_ON_ONCE!(fs != (*current).fs);
    (*fs).users -= 1;
    (*current).fs = new_fs;
    (*current).real_fs = new_fs;
    read_sequnlock_excl(&mut (*fs).seq);
    task_unlock(current);
    if (*fs).users == 0 { free_fs_struct(fs); }
    0
}

// EXPORT_SYMBOL_GPL(unshare_fs_struct)

#[inline]
unsafe fn validate_fs_switch(old_fs: *mut fs_struct) {
    might_sleep();
    if likely((*current).pid != 1) { return; }
    /* @old_fs may be dangling but for comparison it's fine */
    if old_fs != userspace_init_fs { return; }
    pr_warn!("VFS: Pid 1 stopped sharing filesystem state\n");
    set_fs_root(userspace_init_fs, &init_fs.root);
    set_fs_pwd(userspace_init_fs, &init_fs.root);
}

pub unsafe fn switch_fs_struct(mut new_fs: *mut fs_struct) -> *mut fs_struct {
    let fs;
    scoped_guard!(task_lock, current, {
        fs = (*current).fs;
        VFS_WARN_ON_ONCE!(fs != (*current).real_fs);
        read_seqlock_excl(&mut (*fs).seq);
        (*current).fs = new_fs;
        (*current).real_fs = new_fs;
        (*fs).users -= 1;
        if (*fs).users != 0 { new_fs = core::ptr::null_mut(); } else { new_fs = fs; }
        read_sequnlock_excl(&mut (*fs).seq);
    });
    validate_fs_switch(fs);
    new_fs
}

/* to be mentioned only in INIT_TASK */
pub static mut init_fs: fs_struct = fs_struct {
    users: 1,
    seq: __SEQLOCK_UNLOCKED!(init_fs.seq),
    umask: 0o022,
    ..unsafe { core::mem::zeroed() }
};

pub static mut userspace_init_fs: *mut fs_struct = core::ptr::null_mut();

pub unsafe fn init_userspace_fs() {
    let m;
    let mut root: path;
    /* Move PID 1 from nullfs into the initramfs. */
    m = topmost_overmount((*(*current).nsproxy).mnt_ns.root);
    root.mnt = &mut (*m).mnt;
    root.dentry = (*root.mnt).mnt_root;
    VFS_WARN_ON_ONCE!((*current).pid != 1);
    set_fs_root((*current).fs, &root);
    set_fs_pwd((*current).fs, &root);
    /* Hold a reference for the global pointer. */
    read_seqlock_excl(&mut (*(*current).fs).seq);
    (*(*current).fs).users += 1;
    read_sequnlock_excl(&mut (*(*current).fs).seq);
    userspace_init_fs = (*current).fs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
