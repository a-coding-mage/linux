// SPDX-License-Identifier: GPL-2.0-only
/*
 * Module and Firmware Pinning Security Module
 *
 * Copyright 2011-2016 Google Inc.
 *
 * Author: Kees Cook <keescook@chromium.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

/* pr_fmt(fmt) "LoadPin: " fmt */

const VERITY_DIGEST_FILE_HEADER: &[u8] = b"# LOADPIN_TRUSTED_VERITY_ROOT_DIGESTS\0";

type bool_ = bool;
type size_t = usize;
type loff_t = i64;

#[repr(C)]
pub struct file {
    pub f_path: path,
}

#[repr(C)]
pub struct path {
    pub mnt: *mut vfsmount,
}

#[repr(C)]
pub struct vfsmount {
    pub mnt_sb: *mut super_block,
}

#[repr(C)]
pub struct super_block {
    pub s_bdev: *mut block_device,
}

#[repr(C)]
pub struct block_device {
    pub bd_dev: u64,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: size_t,
    pub mode: c_uint,
    pub proc_handler: Option<
        unsafe extern "C" fn(
            *const ctl_table,
            c_int,
            *mut c_void,
            *mut size_t,
            *mut loff_t,
        ) -> c_int,
    >,
    pub extra1: *mut c_void,
    pub extra2: *mut c_void,
}

#[repr(C)]
pub struct lsm_id {
    pub name: *const c_char,
    pub id: c_uint,
}

#[repr(C)]
pub struct security_hook_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct dm_verity_loadpin_trusted_root_digest {
    pub node: list_head,
    pub len: c_int,
    pub data: [u8; 0],
}

#[repr(C)]
pub enum kernel_read_file_id {
    READING_UNKNOWN = 0,
}

type kernel_load_data_id = c_uint;

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const EPERM: c_int = 1;
const ENOMEM: c_int = 12;
const EPROTO: c_int = 71;
const EFAULT: c_int = 14;
const SZ_4K: usize = 4096;
const READING_MAX_ID: usize = 16;
const READING_POLICY: kernel_read_file_id = kernel_read_file_id::READING_UNKNOWN;
const LSM_ID_LOADPIN: c_uint = 0;
const LOADPIN_IOC_SET_TRUSTED_VERITY_DIGESTS: c_uint = 0;

static mut enforce: c_int = if IS_ENABLED_CONFIG_SECURITY_LOADPIN_ENFORCE { 1 } else { 0 };
static mut exclude_read_files: [*mut c_char; READING_MAX_ID] = [ptr::null_mut(); READING_MAX_ID];
static mut ignore_read_file_id: [c_int; READING_MAX_ID] = [0; READING_MAX_ID];
static mut pinned_root: *mut super_block = ptr::null_mut();
static mut pinned_root_spinlock: spinlock_t = spinlock_t {};
#[cfg(CONFIG_SECURITY_LOADPIN_VERITY)]
static mut deny_reading_verity_digests: bool_ = false;

// initialized to false
static mut loadpin_root_writable: bool_ = false;

#[repr(C)]
pub struct spinlock_t {}

const IS_ENABLED_CONFIG_SECURITY_LOADPIN_ENFORCE: bool = false;

unsafe extern "C" {
    static mut current: *mut task_struct;
    static mut kernel_read_file_str: [*const c_char; 0];
    static mut dm_verity_loadpin_trusted_root_digests: list_head;
    static mut SYSCTL_ZERO: *mut c_void;
    static mut SYSCTL_ONE: *mut c_void;

    fn kstrdup_quotable_file(file: *mut file, gfp: c_uint) -> *mut c_char;
    fn kstrdup_quotable_cmdline(task: *mut task_struct, gfp: c_uint) -> *mut c_char;
    fn pr_notice(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn kfree(ptr: *mut c_void);
    fn task_pid_nr(task: *mut task_struct) -> c_int;
    fn SYSCTL_USER_TO_KERN(dir: c_int) -> bool_;
    fn proc_dointvec_minmax(
        table: *const ctl_table,
        dir: c_int,
        buffer: *mut c_void,
        lenp: *mut size_t,
        ppos: *mut loff_t,
    ) -> c_int;
    fn MAJOR(dev: u64) -> c_uint;
    fn MINOR(dev: u64) -> c_uint;
    fn bdev_read_only(bdev: *mut block_device) -> bool_;
    fn ERR_PTR(err: c_long) -> *mut super_block;
    fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool_;
    fn kernel_read_file_id_str(id: kernel_read_file_id) -> *const c_char;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn dm_verity_loadpin_is_bdev_trusted(bdev: *mut block_device) -> bool_;
    fn security_add_hooks(
        hooks: *mut security_hook_list,
        count: usize,
        lsmid: *const lsm_id,
    );
    fn register_sysctl(path: *const c_char, table: *const ctl_table) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn kzalloc(size: usize, gfp: c_uint) -> *mut c_void;
    fn kzalloc_flex_trd(len: c_int, gfp: c_uint) -> *mut dm_verity_loadpin_trusted_root_digest;
    fn kernel_read_file(
        file: *mut file,
        offset: loff_t,
        buf: *mut *mut c_void,
        buf_size: usize,
        file_size: *mut size_t,
        id: kernel_read_file_id,
    ) -> c_int;
    fn strim(s: *mut c_char) -> *mut c_char;
    fn strsep(s: *mut *mut c_char, ct: *const c_char) -> *mut c_char;
    fn hex2bin(dst: *mut u8, src: *const c_char, count: usize) -> c_int;
    fn list_empty(head: *const list_head) -> bool_;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn compat_ptr_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
    fn securityfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn securityfs_create_file(
        name: *const c_char,
        mode: c_uint,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    #[cfg(CONFIG_SECURITY_LOADPIN_VERITY)]
    fn fd_empty(f: c_uint) -> bool_;
    #[cfg(CONFIG_SECURITY_LOADPIN_VERITY)]
    fn fd_file(f: c_uint) -> *mut file;
}

unsafe fn report_load(origin: *const c_char, file: *mut file, operation: *mut c_char) {
    let cmdline: *mut c_char;
    let pathname: *mut c_char;

    pathname = kstrdup_quotable_file(file, GFP_KERNEL);
    cmdline = kstrdup_quotable_cmdline(current, GFP_KERNEL);

    pr_notice(
        c"%s %s obj=%s%s%s pid=%d cmdline=%s%s%s\n".as_ptr(),
        origin,
        operation,
        if !pathname.is_null() && *pathname != b'<' as c_char {
            c"\"".as_ptr()
        } else {
            c"".as_ptr()
        },
        pathname,
        if !pathname.is_null() && *pathname != b'<' as c_char {
            c"\"".as_ptr()
        } else {
            c"".as_ptr()
        },
        task_pid_nr(current),
        if !cmdline.is_null() { c"\"".as_ptr() } else { c"".as_ptr() },
        cmdline,
        if !cmdline.is_null() { c"\"".as_ptr() } else { c"".as_ptr() },
    );

    kfree(cmdline as *mut c_void);
    kfree(pathname as *mut c_void);
}

#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn proc_handler_loadpin(
    table: *const ctl_table,
    dir: c_int,
    buffer: *mut c_void,
    lenp: *mut size_t,
    ppos: *mut loff_t,
) -> c_int {
    if !loadpin_root_writable && SYSCTL_USER_TO_KERN(dir) {
        return -EINVAL;
    }
    proc_dointvec_minmax(table, dir, buffer, lenp, ppos)
}

#[cfg(CONFIG_SYSCTL)]
static loadpin_sysctl_table: [ctl_table; 1] = [ctl_table {
    procname: c"enforce".as_ptr(),
    data: unsafe { &raw mut enforce as *mut c_void },
    maxlen: size_of::<c_int>(),
    mode: 0o644,
    proc_handler: Some(proc_handler_loadpin),
    extra1: unsafe { SYSCTL_ZERO },
    extra2: unsafe { SYSCTL_ONE },
}];

unsafe fn report_writable(mnt_sb: *mut super_block, writable: bool_) {
    if !(*mnt_sb).s_bdev.is_null() {
        pr_info(
            c"%pg (%u:%u): %s\n".as_ptr(),
            (*mnt_sb).s_bdev,
            MAJOR((*(*mnt_sb).s_bdev).bd_dev),
            MINOR((*(*mnt_sb).s_bdev).bd_dev),
            if writable {
                c"writable".as_ptr()
            } else {
                c"read-only".as_ptr()
            },
        );
    } else {
        pr_info(c"mnt_sb lacks block device, treating as: writable\n".as_ptr());
    }

    if !writable {
        pr_info(c"load pinning engaged.\n".as_ptr());
    }
}

/*
 * This must be called after early kernel init, since then the rootdev
 * is available.
 */
unsafe fn sb_is_writable(mnt_sb: *mut super_block) -> bool_ {
    let mut writable: bool_ = true;

    if !(*mnt_sb).s_bdev.is_null() {
        writable = !bdev_read_only((*mnt_sb).s_bdev);
    }

    writable
}

unsafe fn loadpin_sb_free_security(mnt_sb: *mut super_block) {
    /*
     * When unmounting the filesystem we were using for load
     * pinning, we acknowledge the superblock release, but make sure
     * no other modules or firmware can be loaded when we are in
     * enforcing mode. Otherwise, allow the root to be reestablished.
     */
    if !IS_ERR_OR_NULL(pinned_root as *const c_void) && mnt_sb == pinned_root {
        if enforce != 0 {
            pinned_root = ERR_PTR(-(EIO as c_long));
            pr_info(c"umount pinned fs: refusing further loads\n".as_ptr());
        } else {
            pinned_root = ptr::null_mut();
        }
    }
}

unsafe fn loadpin_check(file: *mut file, id: kernel_read_file_id) -> c_int {
    let load_root: *mut super_block;
    let origin: *const c_char = kernel_read_file_id_str(id);
    let mut first_root_pin: bool_ = false;
    let id_index = id as usize;

    /* If the file id is excluded, ignore the pinning. */
    if id_index < ignore_read_file_id.len() && ignore_read_file_id[id_index] != 0 {
        report_load(origin, file, c"pinning-excluded".as_ptr() as *mut c_char);
        return 0;
    }

    /* This handles the older init_module API that has a NULL file. */
    if file.is_null() {
        if enforce == 0 {
            report_load(origin, ptr::null_mut(), c"old-api-pinning-ignored".as_ptr() as *mut c_char);
            return 0;
        }

        report_load(origin, ptr::null_mut(), c"old-api-denied".as_ptr() as *mut c_char);
        return -EPERM;
    }

    load_root = (*(*file).f_path.mnt).mnt_sb;

    /* First loaded module/firmware defines the root for all others. */
    spin_lock(&raw mut pinned_root_spinlock);
    /*
     * pinned_root is only NULL at startup or when the pinned root has
     * been unmounted while we are not in enforcing mode. Otherwise, it
     * is either a valid reference, or an ERR_PTR.
     */
    if pinned_root.is_null() {
        pinned_root = load_root;
        first_root_pin = true;
    }
    spin_unlock(&raw mut pinned_root_spinlock);

    if first_root_pin {
        loadpin_root_writable = sb_is_writable(pinned_root);
        report_writable(pinned_root, loadpin_root_writable);
        report_load(origin, file, c"pinned".as_ptr() as *mut c_char);
    }

    if IS_ERR_OR_NULL(pinned_root as *const c_void)
        || (load_root != pinned_root
            && !dm_verity_loadpin_is_bdev_trusted((*load_root).s_bdev))
    {
        if enforce == 0 {
            report_load(origin, file, c"pinning-ignored".as_ptr() as *mut c_char);
            return 0;
        }

        report_load(origin, file, c"denied".as_ptr() as *mut c_char);
        return -EPERM;
    }

    0
}

unsafe fn loadpin_read_file(
    file: *mut file,
    id: kernel_read_file_id,
    _contents: bool_,
) -> c_int {
    /*
     * LoadPin only cares about the _origin_ of a file, not its
     * contents, so we can ignore the "are full contents available"
     * argument here.
     */
    loadpin_check(file, id)
}

unsafe fn loadpin_load_data(id: kernel_load_data_id, _contents: bool_) -> c_int {
    /*
     * LoadPin only cares about the _origin_ of a file, not its
     * contents, so a NULL file is passed, and we can ignore the
     * state of "contents".
     */
    loadpin_check(ptr::null_mut(), core::mem::transmute::<kernel_load_data_id, kernel_read_file_id>(id))
}

static loadpin_lsmid: lsm_id = lsm_id {
    name: c"loadpin".as_ptr(),
    id: LSM_ID_LOADPIN,
};

/* LSM_HOOK_INIT entries from C are represented here as opaque hook slots. */
static mut loadpin_hooks: [security_hook_list; 3] = [
    security_hook_list { _private: [] },
    security_hook_list { _private: [] },
    security_hook_list { _private: [] },
];

unsafe fn parse_exclude() {
    let mut i: usize;
    let mut j: usize;
    let mut cur: *mut c_char;

    /*
     * Make sure all the arrays stay within expected sizes. This
     * is slightly weird because kernel_read_file_str[] includes
     * READING_MAX_ID, which isn't actually meaningful here.
     */
    const _: [(); READING_MAX_ID] = [(); READING_MAX_ID];

    i = 0;
    while i < exclude_read_files.len() {
        cur = exclude_read_files[i];
        if cur.is_null() {
            break;
        }
        if *cur == b'\0' as c_char {
            i += 1;
            continue;
        }

        j = 0;
        while j < ignore_read_file_id.len() {
            if strcmp(cur, kernel_read_file_str[j]) == 0 {
                pr_info(c"excluding: %s\n".as_ptr(), kernel_read_file_str[j]);
                ignore_read_file_id[j] = 1;
                /*
                 * Can not break, because one read_file_str
                 * may map to more than on read_file_id.
                 */
            }
            j += 1;
        }
        i += 1;
    }
}

unsafe fn loadpin_init() -> c_int {
    pr_info(
        c"ready to pin (currently %senforcing)\n".as_ptr(),
        if enforce != 0 { c"".as_ptr() } else { c"not ".as_ptr() },
    );
    parse_exclude();
    #[cfg(CONFIG_SYSCTL)]
    {
        if register_sysctl(c"kernel/loadpin".as_ptr(), loadpin_sysctl_table.as_ptr()).is_null() {
            pr_notice(c"sysctl registration failed!\n".as_ptr());
        }
    }
    security_add_hooks(
        loadpin_hooks.as_mut_ptr(),
        loadpin_hooks.len(),
        &raw const loadpin_lsmid,
    );

    0
}

#[cfg(CONFIG_SECURITY_LOADPIN_VERITY)]
#[repr(C)]
enum loadpin_securityfs_interface_index {
    LOADPIN_DM_VERITY,
}

#[cfg(CONFIG_SECURITY_LOADPIN_VERITY)]
unsafe fn read_trusted_verity_root_digests(fd: c_uint) -> c_int {
    let mut data: *mut c_void;
    let mut rc: c_int;
    let mut p: *mut c_char;
    let mut d: *mut c_char;

    if deny_reading_verity_digests {
        return -EPERM;
    }

    /* The list of trusted root digests can only be set up once */
    if !list_empty(&raw const dm_verity_loadpin_trusted_root_digests) {
        return -EPERM;
    }

    /* CLASS(fd, f)(fd); fd_empty(f) and fd_file(f) are kernel RAII helpers. */
    let f = fd;
    if fd_empty(f) {
        return -EINVAL;
    }

    data = kzalloc(SZ_4K, GFP_KERNEL);
    if data.is_null() {
        rc = -ENOMEM;
        return read_trusted_verity_root_digests_err(data, rc);
    }

    rc = kernel_read_file(
        fd_file(f),
        0,
        &mut data,
        SZ_4K - 1,
        ptr::null_mut(),
        READING_POLICY,
    );
    if rc < 0 {
        return read_trusted_verity_root_digests_err(data, rc);
    }

    p = data as *mut c_char;
    *p.add(rc as usize) = b'\0' as c_char;
    p = strim(p);

    p = strim(data as *mut c_char);
    loop {
        d = strsep(&mut p, c"\n".as_ptr());
        if d.is_null() {
            break;
        }

        let mut len: c_int;
        let trd: *mut dm_verity_loadpin_trusted_root_digest;

        if d == data as *mut c_char {
            /* first line, validate header */
            if strcmp(d, VERITY_DIGEST_FILE_HEADER.as_ptr() as *const c_char) != 0 {
                return read_trusted_verity_root_digests_err(data, -EPROTO);
            }

            continue;
        }

        len = strlen(d) as c_int;

        if len % 2 != 0 {
            return read_trusted_verity_root_digests_err(data, -EPROTO);
        }

        len /= 2;

        trd = kzalloc_flex_trd(len, GFP_KERNEL);
        if trd.is_null() {
            return read_trusted_verity_root_digests_err(data, -ENOMEM);
        }
        (*trd).len = len;

        if hex2bin((*trd).data.as_mut_ptr(), d, len as usize) != 0 {
            kfree(trd as *mut c_void);
            return read_trusted_verity_root_digests_err(data, -EPROTO);
        }

        list_add_tail(&mut (*trd).node, &raw mut dm_verity_loadpin_trusted_root_digests);
    }

    if list_empty(&raw const dm_verity_loadpin_trusted_root_digests) {
        return read_trusted_verity_root_digests_err(data, -EPROTO);
    }

    kfree(data);

    0
}

#[cfg(CONFIG_SECURITY_LOADPIN_VERITY)]
unsafe fn read_trusted_verity_root_digests_err(data: *mut c_void, rc: c_int) -> c_int {
    kfree(data);

    /* any failure in loading/parsing invalidates the entire list */
    {
        let mut trd: *mut dm_verity_loadpin_trusted_root_digest;
        let mut tmp: *mut dm_verity_loadpin_trusted_root_digest;

        /* list_for_each_entry_safe(trd, tmp, &dm_verity_loadpin_trusted_root_digests, node) */
        trd = ptr::null_mut();
        tmp = ptr::null_mut();
        while !trd.is_null() {
            list_del(&mut (*trd).node);
            kfree(trd as *mut c_void);
            trd = tmp;
        }
    }

    /* disallow further attempts after reading a corrupt/invalid file */
    deny_reading_verity_digests = true;

    rc
}

#[cfg(CONFIG_SECURITY_LOADPIN_VERITY)]
/******************************** securityfs ********************************/

#[cfg(CONFIG_SECURITY_LOADPIN_VERITY)]
unsafe extern "C" fn dm_verity_ioctl(
    _filp: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_long {
    let uarg: *mut c_void = arg as *mut c_void;
    let mut fd: c_uint = 0;

    match cmd {
        LOADPIN_IOC_SET_TRUSTED_VERITY_DIGESTS => {
            if copy_from_user(
                &mut fd as *mut c_uint as *mut c_void,
                uarg as *const c_void,
                size_of::<c_uint>(),
            ) != 0
            {
                return -(EFAULT as c_long);
            }

            read_trusted_verity_root_digests(fd) as c_long
        }
        _ => -(EINVAL as c_long),
    }
}

#[cfg(CONFIG_SECURITY_LOADPIN_VERITY)]
static loadpin_dm_verity_ops: file_operations = file_operations {
    unlocked_ioctl: Some(dm_verity_ioctl),
    compat_ioctl: Some(compat_ptr_ioctl),
};

/**
 * init_loadpin_securityfs - create the securityfs directory for LoadPin
 *
 * We can not put this method normally under the loadpin_init() code path since
 * the security subsystem gets initialized before the vfs caches.
 *
 * Returns 0 if the securityfs directory creation was successful.
 */
#[cfg(CONFIG_SECURITY_LOADPIN_VERITY)]
unsafe fn init_loadpin_securityfs() -> c_int {
    let loadpin_dir: *mut dentry;
    let dentry: *mut dentry;

    loadpin_dir = securityfs_create_dir(c"loadpin".as_ptr(), ptr::null_mut());
    if IS_ERR(loadpin_dir as *const c_void) {
        pr_err(
            c"LoadPin: could not create securityfs dir: %ld\n".as_ptr(),
            PTR_ERR(loadpin_dir as *const c_void),
        );
        return PTR_ERR(loadpin_dir as *const c_void) as c_int;
    }

    dentry = securityfs_create_file(
        c"dm-verity".as_ptr(),
        0o600,
        loadpin_dir,
        loadpin_securityfs_interface_index::LOADPIN_DM_VERITY as isize as *mut c_void,
        &raw const loadpin_dm_verity_ops,
    );
    if IS_ERR(dentry as *const c_void) {
        pr_err(
            c"LoadPin: could not create securityfs entry 'dm-verity': %ld\n".as_ptr(),
            PTR_ERR(dentry as *const c_void),
        );
        return PTR_ERR(dentry as *const c_void) as c_int;
    }

    0
}

unsafe extern "C" {
    fn IS_ERR(ptr: *const c_void) -> bool_;
}

/* DEFINE_LSM(loadpin) = {
 *     .id = &loadpin_lsmid,
 *     .init = loadpin_init,
 * #ifdef CONFIG_SECURITY_LOADPIN_VERITY
 *     .initcall_fs = init_loadpin_securityfs,
 * #endif
 * };
 */

/* Should not be mutable after boot, so not listed in sysfs (perm == 0). */
/* module_param(enforce, int, 0); */
/* MODULE_PARM_DESC(enforce, "Enforce module/firmware pinning"); */
/* module_param_array_named(exclude, exclude_read_files, charp, NULL, 0); */
/* MODULE_PARM_DESC(exclude, "Exclude pinning specific read file types"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
