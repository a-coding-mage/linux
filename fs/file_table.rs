// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/fs/file_table.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *  Copyright (C) 1997 David S. Miller (davem@caip.rutgers.edu)
 */

// Kernel dependencies supplied by the surrounding translation unit.

static mut files_stat: files_stat_struct = files_stat_struct { max_files: NR_FILE };

static mut __filp_cache: *mut kmem_cache = core::ptr::null_mut();
// #define filp_cache runtime_const_ptr(__filp_cache)
static mut __bfilp_cache: *mut kmem_cache = core::ptr::null_mut();
// #define bfilp_cache runtime_const_ptr(__bfilp_cache)
static mut nr_files: percpu_counter = unsafe { core::mem::zeroed() };

/* Container for backing file with optional user path */
#[repr(C)]
struct backing_file {
    file: file,
    user_path_or_freeptr: backing_file_union,
    #[cfg(feature = "CONFIG_SECURITY")]
    security: *mut core::ffi::c_void,
}

#[repr(C)]
union backing_file_union {
    user_path: path,
    bf_freeptr: freeptr_t,
}

unsafe fn backing_file(f: *const file) -> *mut backing_file {
    (f as *mut u8).sub(core::mem::offset_of!(backing_file, file)) as *mut backing_file
}

#[no_mangle]
pub unsafe extern "C" fn backing_file_user_path(f: *const file) -> *const path {
    &(*backing_file(f)).user_path_or_freeptr.user_path
}

#[no_mangle]
pub unsafe extern "C" fn backing_file_set_user_path(f: *mut file, path_: *const path) {
    (*backing_file(f)).user_path_or_freeptr.user_path = *path_;
}

#[cfg(feature = "CONFIG_SECURITY")]
pub unsafe extern "C" fn backing_file_security(f: *const file) -> *mut core::ffi::c_void {
    (*backing_file(f)).security
}

#[cfg(feature = "CONFIG_SECURITY")]
pub unsafe extern "C" fn backing_file_set_security(f: *mut file, security: *mut core::ffi::c_void) {
    (*backing_file(f)).security = security;
}

unsafe fn backing_file_free(ff: *mut backing_file) {
    security_backing_file_free(&mut (*ff).file);
    path_put(&mut (*ff).user_path_or_freeptr.user_path);
    kmem_cache_free(bfilp_cache(), ff as *mut core::ffi::c_void);
}

unsafe fn file_free(f: *mut file) {
    security_file_free(f);
    if likely((*f).f_mode & FMODE_NOACCOUNT == 0) { percpu_counter_dec(&mut nr_files); }
    put_cred((*f).f_cred);
    if unlikely((*f).f_mode & FMODE_BACKING != 0) {
        backing_file_free(backing_file(f));
    } else {
        kmem_cache_free(filp_cache(), f as *mut core::ffi::c_void);
    }
}

unsafe fn get_nr_files() -> i64 { percpu_counter_read_positive(&nr_files) }

#[no_mangle]
pub unsafe extern "C" fn get_max_files() -> u64 { files_stat.max_files }

#[cfg(all(feature = "CONFIG_SYSCTL", feature = "CONFIG_PROC_FS"))]
unsafe fn proc_nr_files(table: *const ctl_table, write: i32, buffer: *mut core::ffi::c_void,
                        lenp: *mut usize, ppos: *mut loff_t) -> i32 {
    files_stat.nr_files = percpu_counter_sum_positive(&nr_files);
    proc_doulongvec_minmax(table, write, buffer, lenp, ppos)
}

#[cfg(all(feature = "CONFIG_SYSCTL", feature = "CONFIG_PROC_FS"))]
static fs_stat_sysctls: [ctl_table; 4] = [
    ctl_table { procname: "file-nr", data: core::ptr::addr_of_mut!(files_stat) as *mut _, maxlen: core::mem::size_of::<files_stat_struct>(), mode: 0o444, proc_handler: Some(proc_nr_files), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
    ctl_table { procname: "file-max", data: core::ptr::addr_of_mut!(files_stat.max_files) as *mut _, maxlen: core::mem::size_of::<u64>(), mode: 0o644, proc_handler: Some(proc_doulongvec_minmax), extra1: SYSCTL_LONG_ZERO, extra2: SYSCTL_LONG_MAX },
    ctl_table { procname: "nr_open", data: core::ptr::addr_of_mut!(sysctl_nr_open) as *mut _, maxlen: core::mem::size_of::<u32>(), mode: 0o644, proc_handler: Some(proc_douintvec_minmax), extra1: core::ptr::addr_of_mut!(sysctl_nr_open_min) as *mut _, extra2: core::ptr::addr_of_mut!(sysctl_nr_open_max) as *mut _ },
    ctl_table { procname: core::ptr::null(), data: core::ptr::null_mut(), maxlen: 0, mode: 0, proc_handler: None, extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
];

unsafe fn init_file(f: *mut file, flags: i32, cred: *const cred) -> i32 {
    let error;
    (*f).f_cred = get_cred(cred);
    error = security_file_alloc(f);
    if unlikely(error != 0) { put_cred((*f).f_cred); return error; }
    spin_lock_init(&mut (*f).f_lock);
    mutex_init(&mut (*f).f_pos_lock);
    core::ptr::write_bytes(&mut (*f).__f_path as *mut _, 0, 1);
    core::ptr::write_bytes(&mut (*f).f_ra as *mut _, 0, 1);
    (*f).f_flags = flags;
    (*f).f_mode = OPEN_FMODE(flags);
    file_set_fsnotify_mode(f, FMODE_NONOTIFY_PERM);
    (*f).f_op = core::ptr::null();
    (*f).f_mapping = core::ptr::null_mut();
    (*f).private_data = core::ptr::null_mut();
    (*f).f_inode = core::ptr::null_mut();
    (*f).f_owner = core::ptr::null_mut();
    #[cfg(feature = "CONFIG_EPOLL")]
    { (*f).f_ep = core::ptr::null_mut(); }
    (*f).f_iocb_flags = 0;
    (*f).f_pos = 0;
    (*f).f_wb_err = 0;
    (*f).f_sb_err = 0;
    file_ref_init(&mut (*f).f_ref, 1);
    0
}

unsafe fn alloc_empty_file(flags: i32, cred: *const cred) -> *mut file {
    static mut old_max: i64 = 0;
    if unlikely(get_nr_files() >= files_stat.max_files as i64) && !capable(CAP_SYS_ADMIN) {
        if percpu_counter_sum_positive(&nr_files) >= files_stat.max_files as i64 { return over_files(&mut old_max); }
    }
    let f = kmem_cache_alloc(filp_cache(), GFP_KERNEL) as *mut file;
    if f.is_null() { return ERR_PTR(-ENOMEM); }
    let error = init_file(f, flags, cred);
    if unlikely(error != 0) { kmem_cache_free(filp_cache(), f as *mut _); return ERR_PTR(error); }
    percpu_counter_inc(&mut nr_files);
    f
}

unsafe fn over_files(old_max: *mut i64) -> *mut file {
    if get_nr_files() > *old_max { pr_info!("VFS: file-max limit %lu reached\n", get_max_files()); *old_max = get_nr_files(); }
    ERR_PTR(-ENFILE)
}

unsafe fn alloc_empty_file_noaccount(flags: i32, cred: *const cred) -> *mut file {
    let f = kmem_cache_alloc(filp_cache(), GFP_KERNEL) as *mut file;
    if f.is_null() { return ERR_PTR(-ENOMEM); }
    let error = init_file(f, flags, cred);
    if unlikely(error != 0) { kmem_cache_free(filp_cache(), f as *mut _); return ERR_PTR(error); }
    (*f).f_mode |= FMODE_NOACCOUNT;
    f
}

unsafe fn init_backing_file(ff: *mut backing_file, user_file: *const file) -> i32 {
    core::ptr::write_bytes(&mut (*ff).user_path_or_freeptr.user_path as *mut _, 0, 1);
    backing_file_set_security(&mut (*ff).file, core::ptr::null_mut());
    security_backing_file_alloc(&mut (*ff).file, user_file)
}

#[no_mangle]
pub unsafe extern "C" fn alloc_empty_backing_file(flags: i32, cred: *const cred, user_file: *const file) -> *mut file {
    let ff = kmem_cache_alloc(bfilp_cache(), GFP_KERNEL) as *mut backing_file;
    if ff.is_null() { return ERR_PTR(-ENOMEM); }
    let error = init_file(&mut (*ff).file, flags, cred);
    if unlikely(error != 0) { kmem_cache_free(bfilp_cache(), ff as *mut _); return ERR_PTR(error); }
    (*ff).file.f_mode |= FMODE_BACKING | FMODE_NOACCOUNT;
    let error = init_backing_file(ff, user_file);
    if unlikely(error != 0) { fput(&mut (*ff).file); return ERR_PTR(error); }
    &mut (*ff).file
}

unsafe fn file_init_path(file_: *mut file, path_: *const path, fop: *const file_operations) {
    (*file_).__f_path = *path_;
    (*file_).f_inode = (*(*path_).dentry).d_inode;
    (*file_).f_mapping = (*(*path_).dentry).d_inode.i_mapping;
    (*file_).f_wb_err = filemap_sample_wb_err((*file_).f_mapping);
    (*file_).f_sb_err = file_sample_sb_err(file_);
    if !(*fop).llseek.is_none() { (*file_).f_mode |= FMODE_LSEEK; }
    if (*file_).f_mode & FMODE_READ != 0 && likely(!(*fop).read.is_none() || !(*fop).read_iter.is_none()) { (*file_).f_mode |= FMODE_CAN_READ; }
    if (*file_).f_mode & FMODE_WRITE != 0 && likely(!(*fop).write.is_none() || !(*fop).write_iter.is_none()) { (*file_).f_mode |= FMODE_CAN_WRITE; }
    (*file_).f_iocb_flags = iocb_flags(file_);
    (*file_).f_mode |= FMODE_OPENED;
    (*file_).f_op = fop;
    if (*file_).f_mode & (FMODE_READ | FMODE_WRITE) == FMODE_READ { i_readcount_inc((*path_).dentry.d_inode); }
}

unsafe fn alloc_file(path_: *const path, flags: i32, fop: *const file_operations) -> *mut file {
    let file_ = alloc_empty_file(flags, current_cred());
    if !IS_ERR(file_) { file_init_path(file_, path_, fop); }
    file_
}

unsafe fn alloc_path_pseudo(name: *const i8, inode: *mut inode, mnt: *mut vfsmount, path_: *mut path) -> i32 {
    if WARN_ON_ONCE(S_ISDIR((*inode).i_mode)) { return -EINVAL; }
    (*path_).dentry = d_alloc_pseudo((*mnt).mnt_sb, QSTR(name));
    if (*path_).dentry.is_null() { return -ENOMEM; }
    (*path_).mnt = mntget(mnt);
    d_instantiate((*path_).dentry, inode);
    0
}

#[no_mangle]
pub unsafe extern "C" fn alloc_file_pseudo(inode: *mut inode, mnt: *mut vfsmount, name: *const i8, flags: i32, fops: *const file_operations) -> *mut file {
    let mut path_: path = core::mem::zeroed();
    let ret = alloc_path_pseudo(name, inode, mnt, &mut path_);
    if ret != 0 { return ERR_PTR(ret); }
    let file_ = alloc_file(&path_, flags, fops);
    if IS_ERR(file_) { ihold(inode); path_put(&mut path_); return file_; }
    file_set_fsnotify_mode(file_, FMODE_NONOTIFY);
    file_
}

#[no_mangle]
pub unsafe extern "C" fn alloc_file_pseudo_noaccount(inode: *mut inode, mnt: *mut vfsmount, name: *const i8, flags: i32, fops: *const file_operations) -> *mut file {
    let mut path_: path = core::mem::zeroed();
    let ret = alloc_path_pseudo(name, inode, mnt, &mut path_);
    if ret != 0 { return ERR_PTR(ret); }
    let file_ = alloc_empty_file_noaccount(flags, current_cred());
    if IS_ERR(file_) { ihold(inode); path_put(&mut path_); return file_; }
    file_init_path(file_, &path_, fops);
    file_set_fsnotify_mode(file_, FMODE_NONOTIFY);
    file_
}

#[no_mangle]
pub unsafe extern "C" fn alloc_file_clone(base: *mut file, flags: i32, fops: *const file_operations) -> *mut file {
    let f = alloc_file(&(*base).f_path, flags, fops);
    if !IS_ERR(f) { path_get(&mut (*f).f_path); (*f).f_mapping = (*base).f_mapping; }
    f
}

unsafe fn __fput(file_: *mut file) {
    let dentry = (*file_).f_path.dentry;
    let mnt = (*file_).f_path.mnt;
    let inode = (*file_).f_inode;
    let mode = (*file_).f_mode;
    if unlikely((*file_).f_mode & FMODE_OPENED == 0) { file_free(file_); return; }
    might_sleep();
    fsnotify_close(file_); eventpoll_release(file_); locks_remove_file(file_);
    security_file_release(file_);
    if unlikely((*file_).f_flags & FASYNC != 0) { if let Some(fasync) = (*(*file_).f_op).fasync { fasync(-1, file_, 0); } }
    if let Some(release) = (*(*file_).f_op).release { release(inode, file_); }
    if unlikely(S_ISCHR((*inode).i_mode) && !(*inode).i_cdev.is_null() && mode & FMODE_PATH == 0) { cdev_put((*inode).i_cdev); }
    fops_put((*file_).f_op); file_f_owner_release(file_); put_file_access(file_); dput(dentry);
    if unlikely(mode & FMODE_NEED_UNMOUNT != 0) { dissolve_on_fput(mnt); }
    mntput(mnt); file_free(file_);
}

static mut delayed_fput_list: llist_head = llist_head { first: core::ptr::null_mut() };
unsafe fn delayed_fput(_unused: *mut work_struct) {
    let node = llist_del_all(&mut delayed_fput_list);
    let mut f: *mut file = core::ptr::null_mut();
    let mut t: *mut file = core::ptr::null_mut();
    llist_for_each_entry_safe!(f, t, node, f_llist, __fput);
}
unsafe fn ____fput(work: *mut callback_head) { __fput(container_of!(work, file, f_task_work)); }
static mut delayed_fput_work: delayed_work = unsafe { core::mem::zeroed() };

#[no_mangle]
pub unsafe extern "C" fn flush_delayed_fput() { delayed_fput(core::ptr::null_mut()); flush_delayed_work(&mut delayed_fput_work); }

unsafe fn __fput_deferred(file_: *mut file) {
    let task = current();
    if unlikely((*file_).f_mode & (FMODE_BACKING | FMODE_OPENED) == 0) { file_free(file_); return; }
    if likely(!in_interrupt() && (*task).flags & PF_KTHREAD == 0) {
        init_task_work(&mut (*file_).f_task_work, Some(____fput));
        if !task_work_add(task, &mut (*file_).f_task_work, TWA_RESUME) { return; }
    }
    if llist_add(&mut (*file_).f_llist, &mut delayed_fput_list) { schedule_delayed_work(&mut delayed_fput_work, 1); }
}

#[no_mangle]
pub unsafe extern "C" fn fput(file_: *mut file) { if unlikely(file_ref_put(&mut (*file_).f_ref)) { __fput_deferred(file_); } }

#[no_mangle]
pub unsafe extern "C" fn __fput_sync(file_: *mut file) { if file_ref_put(&mut (*file_).f_ref) { __fput(file_); } }

#[no_mangle]
pub unsafe extern "C" fn fput_close_sync(file_: *mut file) { if likely(file_ref_put_close(&mut (*file_).f_ref)) { __fput(file_); } }

#[no_mangle]
pub unsafe extern "C" fn fput_close(file_: *mut file) { if file_ref_put_close(&mut (*file_).f_ref) { __fput_deferred(file_); } }

#[no_mangle]
pub unsafe extern "C" fn files_init() {
    let mut args = kmem_cache_args { use_freeptr_offset: true, freeptr_offset: core::mem::offset_of!(file, f_freeptr) };
    __filp_cache = kmem_cache_create("filp", core::mem::size_of::<file>(), &mut args, SLAB_HWCACHE_ALIGN | SLAB_PANIC | SLAB_ACCOUNT | SLAB_TYPESAFE_BY_RCU);
    runtime_const_init_ptr(__filp_cache);
    args.freeptr_offset = core::mem::offset_of!(backing_file, user_path_or_freeptr);
    __bfilp_cache = kmem_cache_create("bfilp", core::mem::size_of::<backing_file>(), &mut args, SLAB_HWCACHE_ALIGN | SLAB_PANIC | SLAB_ACCOUNT | SLAB_TYPESAFE_BY_RCU);
    runtime_const_init_ptr(__bfilp_cache);
    percpu_counter_init(&mut nr_files, 0, GFP_KERNEL);
}

#[no_mangle]
pub unsafe extern "C" fn files_maxfiles_init() {
    let nr_pages = totalram_pages();
    let mut memreserve = (nr_pages - nr_free_pages()) * 3 / 2;
    memreserve = core::cmp::min(memreserve, nr_pages - 1);
    let n = ((nr_pages - memreserve) * (PAGE_SIZE / 1024)) / 10;
    files_stat.max_files = max_t(n, NR_FILE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
