// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Filesystem management and hooks
 *
 * Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 * Copyright © 2021-2025 Microsoft Corporation
 * Copyright © 2022 Günther Noack <gnoack3000@gmail.com>
 * Copyright © 2023-2024 Google LLC
 */

/* Dependencies from:
 * <asm/ioctls.h>, <kunit/test.h>, <linux/...>, <net/af_unix.h>,
 * <uapi/linux/...>, "access.h", "common.h", "cred.h", "domain.h",
 * "fs.h", "limits.h", "log.h", "object.h", "ruleset.h", "setup.h",
 * and <trace/events/landlock.h>.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type bool_t = bool;
type size_t = usize;
type u32 = u32;
type umode_t = u16;
type dev_t = u64;
type access_mask_t = u64;

const LANDLOCK_MAX_NUM_LAYERS: usize = 16;

const EINVAL: c_int = 22;
const EACCES: c_int = 13;
const ENOENT: c_int = 2;
const EPERM: c_int = 1;
const EXDEV: c_int = 18;

const LANDLOCK_KEY_INODE: c_int = 1;
const LANDLOCK_REQUEST_FS_ACCESS: c_int = 1;
const LANDLOCK_REQUEST_FS_CHANGE_TOPOLOGY: c_int = 2;
const LSM_AUDIT_DATA_PATH: c_int = 1;
const LSM_AUDIT_DATA_DENTRY: c_int = 2;
const LSM_AUDIT_DATA_FILE: c_int = 3;
const LSM_AUDIT_DATA_IOCTL_OP: c_int = 4;

const LANDLOCK_ACCESS_FS_EXECUTE: access_mask_t = 1 << 0;
const LANDLOCK_ACCESS_FS_WRITE_FILE: access_mask_t = 1 << 1;
const LANDLOCK_ACCESS_FS_READ_FILE: access_mask_t = 1 << 2;
const LANDLOCK_ACCESS_FS_READ_DIR: access_mask_t = 1 << 3;
const LANDLOCK_ACCESS_FS_REMOVE_DIR: access_mask_t = 1 << 4;
const LANDLOCK_ACCESS_FS_REMOVE_FILE: access_mask_t = 1 << 5;
const LANDLOCK_ACCESS_FS_MAKE_CHAR: access_mask_t = 1 << 6;
const LANDLOCK_ACCESS_FS_MAKE_DIR: access_mask_t = 1 << 7;
const LANDLOCK_ACCESS_FS_MAKE_REG: access_mask_t = 1 << 8;
const LANDLOCK_ACCESS_FS_MAKE_SOCK: access_mask_t = 1 << 9;
const LANDLOCK_ACCESS_FS_MAKE_FIFO: access_mask_t = 1 << 10;
const LANDLOCK_ACCESS_FS_MAKE_BLOCK: access_mask_t = 1 << 11;
const LANDLOCK_ACCESS_FS_MAKE_SYM: access_mask_t = 1 << 12;
const LANDLOCK_ACCESS_FS_REFER: access_mask_t = 1 << 13;
const LANDLOCK_ACCESS_FS_TRUNCATE: access_mask_t = 1 << 14;
const LANDLOCK_ACCESS_FS_IOCTL_DEV: access_mask_t = 1 << 15;
const LANDLOCK_ACCESS_FS_RESOLVE_UNIX: access_mask_t = 1 << 16;
const LANDLOCK_MASK_ACCESS_FS: access_mask_t = !0;
const _LANDLOCK_ACCESS_FS_INITIALLY_DENIED: access_mask_t = 0;
const _LANDLOCK_ACCESS_FS_OPTIONAL: access_mask_t =
    LANDLOCK_ACCESS_FS_TRUNCATE | LANDLOCK_ACCESS_FS_IOCTL_DEV;

const LANDLOCK_SCOPE_SIGNAL: access_mask_t = 1 << 0;

const ACCESS_FILE: access_mask_t = LANDLOCK_ACCESS_FS_EXECUTE
    | LANDLOCK_ACCESS_FS_WRITE_FILE
    | LANDLOCK_ACCESS_FS_READ_FILE
    | LANDLOCK_ACCESS_FS_TRUNCATE
    | LANDLOCK_ACCESS_FS_IOCTL_DEV
    | LANDLOCK_ACCESS_FS_RESOLVE_UNIX;

const S_IFMT: umode_t = 0o170000;
const S_IFSOCK: umode_t = 0o140000;
const S_IFLNK: umode_t = 0o120000;
const S_IFREG: umode_t = 0o100000;
const S_IFBLK: umode_t = 0o060000;
const S_IFDIR: umode_t = 0o040000;
const S_IFCHR: umode_t = 0o020000;
const S_IFIFO: umode_t = 0o010000;
const WHITEOUT_MODE: umode_t = 0;
const WHITEOUT_DEV: dev_t = 0;

const SB_NOUSER: c_ulong = 1 << 31;
const MNT_INTERNAL: c_uint = 1 << 0;
const I_FREEING: c_ulong = 1 << 0;
const I_WILL_FREE: c_ulong = 1 << 1;
const I_NEW: c_ulong = 1 << 2;
const FMODE_READ: c_uint = 1 << 0;
const FMODE_WRITE: c_uint = 1 << 1;
const __FMODE_EXEC: c_uint = 1 << 2;
const RENAME_EXCHANGE: c_uint = 1 << 1;
const RENAME_WHITEOUT: c_uint = 1 << 2;
const SOCK_COREDUMP: c_int = 1 << 2;
const SOCK_DEAD: c_int = 1;
const PIDTYPE_PID: c_int = 0;
const PIDTYPE_TGID: c_int = 1;

/* IOCTL constants are provided by external kernel headers in the original C. */
const FIOCLEX: c_uint = 0x5451;
const FIONCLEX: c_uint = 0x5450;
const FIONBIO: c_uint = 0x5421;
const FIOASYNC: c_uint = 0x5452;
const FIOQSIZE: c_uint = 0x5460;
const FIFREEZE: c_uint = 0xC0045877;
const FITHAW: c_uint = 0xC0045878;
const FS_IOC_FIEMAP: c_uint = 0xC020660B;
const FIGETBSZ: c_uint = 0x00000002;
const FICLONE: c_uint = 0x40049409;
const FICLONERANGE: c_uint = 0x4020940D;
const FIDEDUPERANGE: c_uint = 0xC0189436;
const FS_IOC_GETFSUUID: c_uint = 0;
const FS_IOC_GETFSSYSFSPATH: c_uint = 0;
const FS_IOC32_GETFLAGS: c_uint = 0;
const FS_IOC32_SETFLAGS: c_uint = 0;

#[repr(C)]
pub struct landlock_object {
    pub lock: spinlock_t,
    pub underobj: *mut inode,
    pub usage: refcount_t,
}

#[repr(C)]
pub struct landlock_object_underops {
    pub release: Option<unsafe extern "C" fn(*mut landlock_object)>,
}

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
    pub i_lock: spinlock_t,
    pub i_mode: umode_t,
    pub i_rdev: dev_t,
    pub i_sb_list: list_head,
}

#[repr(C)]
pub struct super_block {
    pub s_flags: c_ulong,
    pub s_inode_list_lock: spinlock_t,
    pub s_inodes: list_head,
    pub s_root: *mut dentry,
}

#[repr(C)]
pub struct dentry {
    pub d_sb: *mut super_block,
    pub d_parent: *mut dentry,
}

#[repr(C)]
pub struct vfsmount {
    pub mnt_root: *mut dentry,
    pub mnt_flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct path {
    pub mnt: *mut vfsmount,
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct landlock_inode_security {
    pub object: *mut landlock_object,
}

#[repr(C)]
pub struct landlock_superblock_security {
    pub inode_refs: atomic_long_t,
}

#[repr(C)]
pub struct landlock_ruleset {
    pub lock: mutex,
    pub handled_masks: access_masks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct access_masks {
    pub fs: access_mask_t,
    pub scope: access_mask_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct layer_mask {
    pub access: access_mask_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct layer_masks {
    pub layers: [layer_mask; LANDLOCK_MAX_NUM_LAYERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union landlock_key {
    pub object: *mut landlock_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_id {
    pub key: landlock_key,
    pub type_: c_int,
}

#[repr(C)]
pub struct landlock_rule;

#[repr(C)]
pub struct landlock_hierarchy {
    pub parent: *const landlock_hierarchy,
}

#[repr(C)]
pub struct landlock_domain {
    pub num_layers: c_int,
    pub hierarchy: *const landlock_hierarchy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_cred_security {
    pub domain: *mut landlock_domain,
}

#[repr(C)]
pub struct landlock_request {
    pub type_: c_int,
    pub audit: lsm_audit_data,
    pub all_existing_optional_access: access_mask_t,
    pub access: access_mask_t,
    pub layer_masks: *mut layer_masks,
    pub layer_plus_one: size_t,
    pub deny_masks: access_masks,
    pub quiet_optional_accesses: access_masks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_audit_data {
    pub type_: c_int,
    pub u: lsm_audit_data_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union lsm_audit_data_union {
    pub path: path,
    pub dentry: *mut dentry,
    pub file: *mut file,
    pub op: *const lsm_ioctlop_audit,
}

#[repr(C)]
pub struct lsm_ioctlop_audit {
    pub path: path,
    pub cmd: c_uint,
}

#[repr(C)]
pub struct file {
    pub f_mode: c_uint,
    pub f_flags: c_uint,
    pub f_cred: *const cred,
    pub f_path: path,
}

#[repr(C)]
pub struct landlock_file_security {
    pub allowed_access: access_mask_t,
    pub deny_masks: access_masks,
    pub quiet_optional_accesses: access_masks,
    pub fown_subject: landlock_cred_security,
    pub fown_tg: *mut pid,
    pub fown_layer: size_t,
}

#[repr(C)]
pub struct fown_struct {
    pub lock: spinlock_t,
    pub pid_type: c_int,
    pub pid: *mut pid,
}

#[repr(C)]
pub struct sock {
    pub sk_socket: *mut socket,
}

#[repr(C)]
pub struct socket {
    pub file: *mut file,
}

#[repr(C)]
pub struct task_struct;
#[repr(C)]
pub struct cred;
#[repr(C)]
pub struct pid;
#[repr(C)]
pub struct security_hook_list;
#[repr(C)]
pub struct kunit;
#[repr(C)]
pub struct kunit_case;
#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub test_cases: *mut kunit_case,
}
#[repr(C)]
pub struct spinlock_t;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct refcount_t;
#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct atomic_long_t;

extern "C" {
    static mut landlock_initialized: bool_t;
    static mut landlock_blob_sizes: landlock_blob_sizes_t;
    static mut landlock_lsmid: c_void;

    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn iput(inode: *mut inode);
    fn iput_not_last(inode: *mut inode);
    fn ihold(inode: *mut inode);
    fn __iget(inode: *mut inode);
    fn dput(dentry: *mut dentry);
    fn dget(dentry: *mut dentry);
    fn path_get(path: *mut path);
    fn path_put(path: *mut path);
    fn wake_up_var(var: *mut atomic_long_t);
    fn wait_var_event_inode_refs(var: *mut atomic_long_t);
    fn atomic_long_inc(v: *mut atomic_long_t);
    fn atomic_long_dec_and_test(v: *mut atomic_long_t) -> bool_t;
    fn atomic_long_read(v: *mut atomic_long_t) -> c_ulong;
    fn refcount_inc_not_zero(r: *mut refcount_t) -> bool_t;
    fn kfree(p: *mut c_void);
    fn __getname() -> *mut c_char;
    fn __putname(p: *mut c_char);
    fn resolve_path_for_trace(path: *const path, buffer: *mut c_char) -> *const c_char;
    fn trace_landlock_add_rule_fs_enabled() -> bool_t;
    fn trace_landlock_add_rule_fs(
        ruleset: *mut landlock_ruleset,
        access_rights: access_mask_t,
        path: *const path,
        pathname: *const c_char,
    );
    fn trace_landlock_check_rule_fs(
        domain: *const landlock_domain,
        rule: *const landlock_rule,
        access_request: access_mask_t,
        dentry: *const dentry,
    );
    fn landlock_create_object(
        underops: *const landlock_object_underops,
        underobj: *mut inode,
    ) -> *mut landlock_object;
    fn landlock_put_object(object: *mut landlock_object);
    fn landlock_insert_rule(
        ruleset: *mut landlock_ruleset,
        id: landlock_id,
        access_rights: access_mask_t,
        flags: u32,
    ) -> c_int;
    fn landlock_inode(inode: *mut inode) -> *mut landlock_inode_security;
    fn landlock_superblock(sb: *mut super_block) -> *mut landlock_superblock_security;
    fn d_is_dir(dentry: *const dentry) -> bool_t;
    fn d_is_negative(dentry: *const dentry) -> bool_t;
    fn d_is_positive(dentry: *const dentry) -> bool_t;
    fn d_backing_inode(dentry: *const dentry) -> *mut inode;
    fn IS_PRIVATE(inode: *mut inode) -> bool_t;
    fn IS_ROOT(dentry: *mut dentry) -> bool_t;
    fn follow_up(path: *mut path) -> bool_t;
    fn dget_parent(dentry: *mut dentry) -> *mut dentry;
    fn icount_read_once(inode: *mut inode) -> c_int;
    fn inode_state_read(inode: *mut inode) -> c_ulong;
    fn access_mask_subset(access: access_mask_t, superset: access_mask_t) -> bool_t;
    fn landlock_unmask_layers(
        domain: *const landlock_domain,
        id: landlock_id,
        masks: *mut layer_masks,
        rule: *mut *const landlock_rule,
    ) -> bool_t;
    fn landlock_union_access_masks(domain: *const landlock_domain) -> access_masks;
    fn landlock_init_layer_masks(
        domain: *const landlock_domain,
        access_request: access_mask_t,
        masks: *mut layer_masks,
        key_type: c_int,
    ) -> access_mask_t;
    fn current_cred() -> *const cred;
    fn landlock_get_applicable_subject(
        cred: *const cred,
        masks: access_masks,
        handle_layer: *mut size_t,
    ) -> *const landlock_cred_security;
    fn landlock_log_denial(
        subject: *const landlock_cred_security,
        request: *const landlock_request,
    );
    fn new_decode_dev(dev: c_uint) -> dev_t;
    fn file_inode(file: *const file) -> *mut inode;
    fn landlock_file(file: *const file) -> *mut landlock_file_security;
    fn landlock_cred(cred: *const cred) -> *const landlock_cred_security;
    fn landlock_get_deny_masks(
        all_existing_optional_access: access_mask_t,
        optional_access: access_mask_t,
        masks: *const layer_masks,
    ) -> access_masks;
    fn landlock_get_quiet_optional_accesses(
        all_existing_optional_access: access_mask_t,
        deny_masks: access_masks,
        masks: *const layer_masks,
    ) -> access_masks;
    fn unix_state_lock(sock: *mut sock);
    fn unix_state_unlock(sock: *mut sock);
    fn sock_flag(sock: *mut sock, flag: c_int) -> bool_t;
    fn lockdep_assert_held(lock: *mut spinlock_t);
    fn pid_task(pid: *mut pid, pid_type: c_int) -> *mut task_struct;
    fn same_thread_group(p: *mut task_struct, current: *mut task_struct) -> bool_t;
    fn task_tgid(task: *mut task_struct) -> *mut pid;
    fn get_pid(pid: *mut pid) -> *mut pid;
    fn put_pid(pid: *mut pid);
    fn landlock_get_domain(domain: *mut landlock_domain);
    fn landlock_put_domain_deferred(domain: *mut landlock_domain);
    fn file_f_owner(file: *mut file) -> *mut fown_struct;
    static mut current: *mut task_struct;
    fn security_add_hooks(
        hooks: *mut security_hook_list,
        count: size_t,
        lsmid: *mut c_void,
    );
}

#[repr(C)]
pub struct landlock_blob_sizes_t {
    pub lbs_inode: usize,
}

#[inline]
unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

#[inline]
unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

#[inline]
fn WARN_ON_ONCE(cond: bool) -> bool {
    cond
}

#[inline]
fn likely<T>(v: T) -> T {
    v
}

#[inline]
fn unlikely<T>(v: T) -> T {
    v
}

#[inline]
unsafe fn rcu_access_pointer<T>(p: *mut T) -> *mut T {
    p
}

#[inline]
unsafe fn rcu_dereference<T>(p: *mut T) -> *mut T {
    p
}

#[inline]
unsafe fn rcu_assign_pointer<T>(slot: *mut *mut T, value: *mut T) {
    *slot = value;
}

#[inline]
fn S_ISDIR(mode: umode_t) -> bool {
    (mode & S_IFMT) == S_IFDIR
}

#[inline]
fn S_ISBLK(mode: umode_t) -> bool {
    (mode & S_IFMT) == S_IFBLK
}

#[inline]
fn S_ISCHR(mode: umode_t) -> bool {
    (mode & S_IFMT) == S_IFCHR
}

/* Underlying object management */

unsafe extern "C" fn release_inode(object: *mut landlock_object) {
    let inode = (*object).underobj;
    let sb: *mut super_block;

    if inode.is_null() {
        spin_unlock(&mut (*object).lock);
        return;
    }

    /*
     * Protects against concurrent use by hook_sb_delete() of the reference
     * to the underlying inode.
     */
    (*object).underobj = ptr::null_mut();
    /*
     * Makes sure that if the filesystem is concurrently unmounted,
     * hook_sb_delete() will wait for us to finish iput().
     */
    sb = (*inode).i_sb;
    atomic_long_inc(&mut (*landlock_superblock(sb)).inode_refs);
    spin_unlock(&mut (*object).lock);
    /*
     * Because object->underobj was not NULL, hook_sb_delete() and
     * get_inode_object() guarantee that it is safe to reset
     * landlock_inode(inode)->object while it is not NULL.  It is therefore
     * not necessary to lock inode->i_lock.
     */
    rcu_assign_pointer(&mut (*landlock_inode(inode)).object, ptr::null_mut());
    /*
     * Now, new rules can safely be tied to @inode with get_inode_object().
     */

    iput(inode);
    if atomic_long_dec_and_test(&mut (*landlock_superblock(sb)).inode_refs) {
        wake_up_var(&mut (*landlock_superblock(sb)).inode_refs);
    }
}

static landlock_fs_underops: landlock_object_underops = landlock_object_underops {
    release: Some(release_inode),
};

/* IOCTL helpers */

/**
 * is_masked_device_ioctl - Determine whether an IOCTL command is always
 * permitted with Landlock for device files.  These commands can not be
 * restricted on device files by enforcing a Landlock policy.
 *
 * @cmd: The IOCTL command that is supposed to be run.
 *
 * By default, any IOCTL on a device file requires the
 * LANDLOCK_ACCESS_FS_IOCTL_DEV right.  However, we blanket-permit some
 * commands, if:
 *
 * 1. The command is implemented in fs/ioctl.c's do_vfs_ioctl(),
 *    not in f_ops->unlocked_ioctl() or f_ops->compat_ioctl().
 *
 * 2. The command is harmless when invoked on devices.
 *
 * We also permit commands that do not make sense for devices, but where the
 * do_vfs_ioctl() implementation returns a more conventional error code.
 *
 * Any new IOCTL commands that are implemented in fs/ioctl.c's do_vfs_ioctl()
 * should be considered for inclusion here.
 *
 * Return: True if the IOCTL @cmd can not be restricted with Landlock for
 * device files, false otherwise.
 */
fn is_masked_device_ioctl(cmd: c_uint) -> bool {
    match cmd {
        FIOCLEX | FIONCLEX | FIONBIO | FIOASYNC | FIOQSIZE | FIFREEZE | FITHAW
        | FS_IOC_FIEMAP | FIGETBSZ | FICLONE | FICLONERANGE | FIDEDUPERANGE
        | FS_IOC_GETFSUUID | FS_IOC_GETFSSYSFSPATH => true,
        _ => false,
    }
}

/*
 * is_masked_device_ioctl_compat - same as the helper above, but checking the
 * "compat" IOCTL commands.
 *
 * The IOCTL commands with special handling in compat-mode should behave the
 * same as their non-compat counterparts.
 */
fn is_masked_device_ioctl_compat(cmd: c_uint) -> bool {
    match cmd {
        /* FICLONE is permitted, same as in the non-compat variant. */
        FICLONE => true,
        /*
         * CONFIG_X86_64-only FS_IOC_RESVSP_32, FS_IOC_RESVSP64_32,
         * FS_IOC_UNRESVSP_32, FS_IOC_UNRESVSP64_32, FS_IOC_ZERO_RANGE_32:
         * not blanket-permitted, for consistency with non-compat variants.
         */
        FS_IOC32_GETFLAGS | FS_IOC32_SETFLAGS => false,
        _ => is_masked_device_ioctl(cmd),
    }
}

/* Ruleset management */

unsafe fn get_inode_object(inode: *mut inode) -> *mut landlock_object {
    let mut object: *mut landlock_object;
    let new_object: *mut landlock_object;
    let inode_sec = landlock_inode(inode);

    rcu_read_lock();
    loop {
        object = rcu_dereference((*inode_sec).object);
        if !object.is_null() {
            if likely(refcount_inc_not_zero(&mut (*object).usage)) {
                rcu_read_unlock();
                return object;
            }
            /*
             * We are racing with release_inode(), the object is going
             * away.  Wait for release_inode(), then retry.
             */
            spin_lock(&mut (*object).lock);
            spin_unlock(&mut (*object).lock);
            continue;
        }
        break;
    }
    rcu_read_unlock();

    /*
     * If there is no object tied to @inode, then create a new one (without
     * holding any locks).
     */
    new_object = landlock_create_object(&landlock_fs_underops, inode);
    if IS_ERR(new_object) {
        return new_object;
    }

    /*
     * Protects against concurrent calls to get_inode_object() or
     * hook_sb_delete().
     */
    spin_lock(&mut (*inode).i_lock);
    if unlikely(!rcu_access_pointer((*inode_sec).object).is_null()) {
        /* Someone else just created the object, bail out and retry. */
        spin_unlock(&mut (*inode).i_lock);
        kfree(new_object as *mut c_void);

        rcu_read_lock();
        loop {
            object = rcu_dereference((*inode_sec).object);
            if !object.is_null() {
                if likely(refcount_inc_not_zero(&mut (*object).usage)) {
                    rcu_read_unlock();
                    return object;
                }
                spin_lock(&mut (*object).lock);
                spin_unlock(&mut (*object).lock);
                continue;
            }
            break;
        }
        rcu_read_unlock();
        return get_inode_object(inode);
    }

    /*
     * @inode will be released by hook_sb_delete() on its superblock
     * shutdown, or by release_inode() when no more ruleset references the
     * related object.
     */
    ihold(inode);
    rcu_assign_pointer(&mut (*inode_sec).object, new_object);
    spin_unlock(&mut (*inode).i_lock);
    new_object
}

/*
 * @path: Should have been checked by get_path_from_fd().
 */
pub unsafe extern "C" fn landlock_append_fs_rule(
    ruleset: *mut landlock_ruleset,
    path: *const path,
    mut access_rights: access_mask_t,
    flags: u32,
) -> c_int {
    let err: c_int;
    let mut id = landlock_id {
        key: landlock_key { object: ptr::null_mut() },
        type_: LANDLOCK_KEY_INODE,
    };

    /* Files only get access rights that make sense. */
    if !d_is_dir((*path).dentry) && !access_mask_subset(access_rights, ACCESS_FILE) {
        return -EINVAL;
    }

    /* Transforms relative access rights to absolute ones. */
    access_rights |= LANDLOCK_MASK_ACCESS_FS
        & !((*ruleset).handled_masks.fs | _LANDLOCK_ACCESS_FS_INITIALLY_DENIED);
    id.key.object = get_inode_object(d_backing_inode((*path).dentry));
    if IS_ERR(id.key.object) {
        return PTR_ERR(id.key.object);
    }
    mutex_lock(&mut (*ruleset).lock);
    err = landlock_insert_rule(ruleset, id, access_rights, flags);

    /*
     * Emit after the rule insertion succeeds, so every event corresponds to
     * a rule that is actually in the ruleset.  The ruleset lock is still
     * held for BTF consistency (enforced by lockdep_assert_held in
     * TP_fast_assign).
     */
    if err == 0 && trace_landlock_add_rule_fs_enabled() {
        let buffer = __getname();
        let pathname = if !buffer.is_null() {
            resolve_path_for_trace(path, buffer)
        } else {
            b"<no_mem>\0".as_ptr() as *const c_char
        };

        trace_landlock_add_rule_fs(ruleset, access_rights, path, pathname);
        if !buffer.is_null() {
            __putname(buffer);
        }
    }
    mutex_unlock(&mut (*ruleset).lock);

    /*
     * No need to check for an error because landlock_insert_rule()
     * increments the refcount for the new object if needed.
     */
    landlock_put_object(id.key.object);
    err
}

/* Access-control management */

/**
 * get_inode_id - Look up the Landlock object for a dentry
 * @dentry: The dentry to look up.
 * @id: Filled with the inode's Landlock object pointer on success.
 *
 * Extracts the Landlock object pointer from @dentry's inode security blob and
 * stores it in @id for use as a rule-tree lookup key.
 *
 * When this returns false (negative dentry or no Landlock object), no rule can
 * match this inode, so landlock_unmask_layers() need not be called.  Callers
 * that gate landlock_unmask_layers() on this function must handle the NULL
 * masks case independently, since the !masks-returns-true early-return in
 * landlock_unmask_layers() will not be reached.  See the allowed_parent2
 * initialization in is_access_to_paths_allowed().
 *
 * Return: True if a Landlock object exists for @dentry, false otherwise.
 */
unsafe fn get_inode_id(dentry: *const dentry, id: *mut landlock_id) -> bool {
    /* Ignores nonexistent leafs. */
    if d_is_negative(dentry) {
        return false;
    }

    /*
     * rcu_access_pointer() is sufficient: the pointer is used only as a
     * numeric comparison key for rule lookup, not dereferenced.  The object
     * cannot be freed while the domain exists because the domain's rule
     * tree holds its own reference to it.
     */
    (*id).key.object = rcu_access_pointer((*landlock_inode(d_backing_inode(dentry))).object);
    !(*id).key.object.is_null()
}

unsafe fn unmask_layers_fs(
    domain: *const landlock_domain,
    id: landlock_id,
    access_request: access_mask_t,
    masks: *mut layer_masks,
    dentry: *const dentry,
) -> bool {
    let mut rule: *const landlock_rule = ptr::null();
    let ret = landlock_unmask_layers(domain, id, masks, &mut rule);
    if !rule.is_null() {
        trace_landlock_check_rule_fs(domain, rule, access_request, dentry);
    }
    ret
}

/*
 * Allows access to pseudo filesystems that will never be mountable (e.g.
 * sockfs, pipefs), but can still be reachable through
 * /proc/<pid>/fd/<file-descriptor>
 */
unsafe fn is_nouser_or_private(dentry: *const dentry) -> bool {
    ((*(*dentry).d_sb).s_flags & SB_NOUSER) != 0
        || (d_is_positive(dentry) && unlikely(IS_PRIVATE(d_backing_inode(dentry))))
}

static any_fs: access_masks = access_masks { fs: !0, scope: 0 };

/*
 * Returns true iff the child file with the given src_child access rights under
 * src_parent would result in having the same or fewer access rights if it were
 * moved under new_parent.
 */
unsafe fn may_refer(
    src_parent: *const layer_masks,
    src_child: *const layer_masks,
    new_parent: *const layer_masks,
    child_is_dir: bool,
) -> bool {
    for i in 0..(*new_parent).layers.len() {
        let mut child_access =
            (*src_parent).layers[i].access & (*src_child).layers[i].access;
        let mut parent_access = (*new_parent).layers[i].access;

        if !child_is_dir {
            child_access &= ACCESS_FILE;
            parent_access &= ACCESS_FILE;
        }

        if !access_mask_subset(child_access, parent_access) {
            return false;
        }
    }
    true
}

/*
 * Check that a destination file hierarchy has more restrictions than a source
 * file hierarchy.  This is only used for link and rename actions.
 *
 * Return: True if child1 may be moved from parent1 to parent2 without
 * increasing its access rights (if child2 is set, an additional condition is
 * that child2 may be used from parent2 to parent1 without increasing its access
 * rights), false otherwise.
 */
unsafe fn no_more_access(
    parent1: *const layer_masks,
    child1: *const layer_masks,
    child1_is_dir: bool,
    parent2: *const layer_masks,
    child2: *const layer_masks,
    child2_is_dir: bool,
) -> bool {
    if !may_refer(parent1, child1, parent2, child1_is_dir) {
        return false;
    }

    if child2.is_null() {
        return true;
    }

    may_refer(parent2, child2, parent1, child2_is_dir)
}

/* CONFIG_SECURITY_LANDLOCK_KUNIT_TEST: KUnit tests from the C source are omitted
 * from executable Rust here because the external KUnit macros have no file-local
 * Rust equivalent.
 */

unsafe fn is_layer_masks_allowed(masks: *const layer_masks) -> bool {
    for i in 0..(*masks).layers.len() {
        if (*masks).layers[i].access != 0 {
            return false;
        }
    }
    true
}

/*
 * Removes @masks accesses that are not requested.
 *
 * Returns true if the request is allowed, false otherwise.
 */
unsafe fn scope_to_request(access_request: access_mask_t, masks: *mut layer_masks) -> bool {
    let mut saw_unfulfilled_access = false;

    if WARN_ON_ONCE(masks.is_null()) {
        return true;
    }

    for i in 0..(*masks).layers.len() {
        (*masks).layers[i].access &= access_request;
        if (*masks).layers[i].access != 0 {
            saw_unfulfilled_access = true;
        }
    }
    !saw_unfulfilled_access
}

/*
 * Returns true if there is at least one access right different than
 * LANDLOCK_ACCESS_FS_REFER.
 */
unsafe fn is_eacces(masks: *const layer_masks, access_request: access_mask_t) -> bool {
    if masks.is_null() {
        return false;
    }

    for i in 0..(*masks).layers.len() {
        /* LANDLOCK_ACCESS_FS_REFER alone must return -EXDEV. */
        if ((*masks).layers[i].access & access_request & !LANDLOCK_ACCESS_FS_REFER) != 0 {
            return true;
        }
    }
    false
}

/**
 * is_access_to_paths_allowed - Check accesses for requests with a common path
 */
unsafe fn is_access_to_paths_allowed(
    domain: *const landlock_domain,
    path: *const path,
    access_request_parent1: access_mask_t,
    layer_masks_parent1: *mut layer_masks,
    log_request_parent1: *mut landlock_request,
    dentry_child1: *mut dentry,
    access_request_parent2: access_mask_t,
    layer_masks_parent2: *mut layer_masks,
    log_request_parent2: *mut landlock_request,
    dentry_child2: *mut dentry,
) -> bool {
    let mut allowed_parent1 = false;
    let mut allowed_parent2 = false;
    let mut is_dom_check: bool;
    let mut child1_is_directory = true;
    let mut child2_is_directory = true;
    let mut walker_path: path;
    let mut id = landlock_id {
        key: landlock_key { object: ptr::null_mut() },
        type_: LANDLOCK_KEY_INODE,
    };
    let mut access_masked_parent1: access_mask_t;
    let mut access_masked_parent2: access_mask_t;
    let mut _layer_masks_child1: layer_masks = core::mem::zeroed();
    let mut _layer_masks_child2: layer_masks = core::mem::zeroed();
    let mut layer_masks_child1: *mut layer_masks = ptr::null_mut();
    let mut layer_masks_child2: *mut layer_masks = ptr::null_mut();

    if access_request_parent1 == 0 && access_request_parent2 == 0 {
        return true;
    }

    if WARN_ON_ONCE(path.is_null()) {
        return true;
    }

    if is_nouser_or_private((*path).dentry) {
        return true;
    }

    if WARN_ON_ONCE(layer_masks_parent1.is_null()) {
        return false;
    }

    allowed_parent1 = is_layer_masks_allowed(layer_masks_parent1);

    if unlikely(!layer_masks_parent2.is_null()) {
        if WARN_ON_ONCE(dentry_child1.is_null()) {
            return false;
        }

        allowed_parent2 = is_layer_masks_allowed(layer_masks_parent2);

        /*
         * For a double request, first check for potential privilege
         * escalation by looking at domain handled accesses (which are
         * a superset of the meaningful requested accesses).
         */
        access_masked_parent1 = landlock_union_access_masks(domain).fs;
        access_masked_parent2 = access_masked_parent1;
        is_dom_check = true;
    } else {
        if WARN_ON_ONCE(!dentry_child1.is_null() || !dentry_child2.is_null()) {
            return false;
        }
        /* For a simple request, only check for requested accesses. */
        access_masked_parent1 = access_request_parent1;
        access_masked_parent2 = access_request_parent2;
        /*
         * Simple requests have no parent2 to check, so parent2 is
         * trivially allowed.
         */
        allowed_parent2 = true;
        is_dom_check = false;
    }

    if unlikely(!dentry_child1.is_null()) {
        let mut id = landlock_id {
            key: landlock_key { object: ptr::null_mut() },
            type_: LANDLOCK_KEY_INODE,
        };
        let handled = landlock_init_layer_masks(
            domain,
            LANDLOCK_MASK_ACCESS_FS,
            &mut _layer_masks_child1,
            LANDLOCK_KEY_INODE,
        );
        if handled != 0 && get_inode_id(dentry_child1, &mut id) {
            unmask_layers_fs(domain, id, handled, &mut _layer_masks_child1, dentry_child1);
        }
        layer_masks_child1 = &mut _layer_masks_child1;
        child1_is_directory = d_is_dir(dentry_child1);
    }
    if unlikely(!dentry_child2.is_null()) {
        let mut id = landlock_id {
            key: landlock_key { object: ptr::null_mut() },
            type_: LANDLOCK_KEY_INODE,
        };
        let handled = landlock_init_layer_masks(
            domain,
            LANDLOCK_MASK_ACCESS_FS,
            &mut _layer_masks_child2,
            LANDLOCK_KEY_INODE,
        );
        if handled != 0 && get_inode_id(dentry_child2, &mut id) {
            unmask_layers_fs(domain, id, handled, &mut _layer_masks_child2, dentry_child2);
        }
        layer_masks_child2 = &mut _layer_masks_child2;
        child2_is_directory = d_is_dir(dentry_child2);
    }

    walker_path = *path;
    path_get(&mut walker_path);
    /*
     * We need to walk through all the hierarchy to not miss any relevant
     * restriction.
     */
    loop {
        if unlikely(
            is_dom_check
                && no_more_access(
                    layer_masks_parent1,
                    layer_masks_child1,
                    child1_is_directory,
                    layer_masks_parent2,
                    layer_masks_child2,
                    child2_is_directory,
                ),
        ) {
            /*
             * Now, downgrades the remaining checks from domain
             * handled accesses to requested accesses.
             */
            is_dom_check = false;
            access_masked_parent1 = access_request_parent1;
            access_masked_parent2 = access_request_parent2;

            allowed_parent1 = allowed_parent1
                || scope_to_request(access_masked_parent1, layer_masks_parent1);
            allowed_parent2 = allowed_parent2
                || scope_to_request(access_masked_parent2, layer_masks_parent2);

            /* Stops when all accesses are granted. */
            if allowed_parent1 && allowed_parent2 {
                break;
            }
        }

        if get_inode_id(walker_path.dentry, &mut id) {
            allowed_parent1 = allowed_parent1
                || unmask_layers_fs(
                    domain,
                    id,
                    access_masked_parent1,
                    layer_masks_parent1,
                    walker_path.dentry,
                );
            allowed_parent2 = allowed_parent2
                || unmask_layers_fs(
                    domain,
                    id,
                    access_masked_parent2,
                    layer_masks_parent2,
                    walker_path.dentry,
                );
        }

        /* Stops when a rule from each layer grants access. */
        if allowed_parent1 && allowed_parent2 {
            break;
        }

        loop {
            if walker_path.dentry == (*walker_path.mnt).mnt_root {
                if follow_up(&mut walker_path) {
                    /* Ignores hidden mount points. */
                    continue;
                } else {
                    /*
                     * Stops at the real root.  Denies access
                     * because not all layers have granted access.
                     */
                    break;
                }
            }

            if unlikely(IS_ROOT(walker_path.dentry)) {
                if likely(((*walker_path.mnt).mnt_flags & MNT_INTERNAL) != 0) {
                    /*
                     * Stops and allows access when reaching disconnected root
                     * directories that are part of internal filesystems.
                     */
                    allowed_parent1 = true;
                    allowed_parent2 = true;
                    break;
                }

                /*
                 * We reached a disconnected root directory from a bind mount.
                 * Let's continue the walk with the mount point we missed.
                 */
                dput(walker_path.dentry);
                walker_path.dentry = (*walker_path.mnt).mnt_root;
                dget(walker_path.dentry);
            } else {
                let parent_dentry = dget_parent(walker_path.dentry);
                dput(walker_path.dentry);
                walker_path.dentry = parent_dentry;
            }
            break;
        }
        if walker_path.dentry == (*walker_path.mnt).mnt_root && !follow_up(&mut walker_path) {
            break;
        }
    }
    path_put(&mut walker_path);

    /*
     * CONFIG_SECURITY_LANDLOCK_LOG conditional block.
     */
    if !allowed_parent1 && !log_request_parent1.is_null() {
        (*log_request_parent1).type_ = LANDLOCK_REQUEST_FS_ACCESS;
        (*log_request_parent1).audit.type_ = LSM_AUDIT_DATA_PATH;
        (*log_request_parent1).audit.u.path = *path;
        (*log_request_parent1).access = access_masked_parent1;
        (*log_request_parent1).layer_masks = layer_masks_parent1;
    }

    if !allowed_parent2 && !log_request_parent2.is_null() {
        (*log_request_parent2).type_ = LANDLOCK_REQUEST_FS_ACCESS;
        (*log_request_parent2).audit.type_ = LSM_AUDIT_DATA_PATH;
        (*log_request_parent2).audit.u.path = *path;
        (*log_request_parent2).access = access_masked_parent2;
        (*log_request_parent2).layer_masks = layer_masks_parent2;
    }

    allowed_parent1 && allowed_parent2
}

unsafe fn current_check_access_path(path: *const path, mut access_request: access_mask_t) -> c_int {
    let masks = access_masks {
        fs: access_request,
        scope: 0,
    };
    let subject = landlock_get_applicable_subject(current_cred(), masks, ptr::null_mut());
    let mut layer_masks: layer_masks = core::mem::zeroed();
    let mut request: landlock_request = core::mem::zeroed();

    if subject.is_null() {
        return 0;
    }

    access_request = landlock_init_layer_masks(
        (*subject).domain,
        access_request,
        &mut layer_masks,
        LANDLOCK_KEY_INODE,
    );
    if is_access_to_paths_allowed(
        (*subject).domain,
        path,
        access_request,
        &mut layer_masks,
        &mut request,
        ptr::null_mut(),
        0,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    ) {
        return 0;
    }

    landlock_log_denial(subject, &request);
    -EACCES
}

fn get_mode_access(mode: umode_t, dev: dev_t) -> access_mask_t {
    match mode & S_IFMT {
        S_IFLNK => LANDLOCK_ACCESS_FS_MAKE_SYM,
        S_IFDIR => LANDLOCK_ACCESS_FS_MAKE_DIR,
        S_IFCHR => {
            /* Whiteout objects are guarded with MAKE_REG. */
            if dev == WHITEOUT_DEV {
                LANDLOCK_ACCESS_FS_MAKE_REG
            } else {
                LANDLOCK_ACCESS_FS_MAKE_CHAR
            }
        }
        S_IFBLK => LANDLOCK_ACCESS_FS_MAKE_BLOCK,
        S_IFIFO => LANDLOCK_ACCESS_FS_MAKE_FIFO,
        S_IFSOCK => LANDLOCK_ACCESS_FS_MAKE_SOCK,
        S_IFREG | 0 => LANDLOCK_ACCESS_FS_MAKE_REG,
        /* Treats weird files as regular files. */
        _ => LANDLOCK_ACCESS_FS_MAKE_REG,
    }
}

unsafe fn get_dentry_access(dentry: *const dentry) -> access_mask_t {
    let inode = d_backing_inode(dentry);
    get_mode_access((*inode).i_mode, (*inode).i_rdev)
}

unsafe fn maybe_remove(dentry: *const dentry) -> access_mask_t {
    if d_is_negative(dentry) {
        return 0;
    }
    if d_is_dir(dentry) {
        LANDLOCK_ACCESS_FS_REMOVE_DIR
    } else {
        LANDLOCK_ACCESS_FS_REMOVE_FILE
    }
}

/**
 * collect_domain_accesses - Walk through a file path and collect accesses
 */
unsafe fn collect_domain_accesses(
    domain: *const landlock_domain,
    mnt_root: *const dentry,
    mut dir: *mut dentry,
    layer_masks_dom: *mut layer_masks,
) -> bool {
    let mut ret = false;
    let access_masked_dom: access_mask_t;

    if WARN_ON_ONCE(domain.is_null() || mnt_root.is_null() || dir.is_null() || layer_masks_dom.is_null()) {
        return true;
    }
    if is_nouser_or_private(dir) {
        return true;
    }

    access_masked_dom = landlock_init_layer_masks(
        domain,
        LANDLOCK_MASK_ACCESS_FS,
        layer_masks_dom,
        LANDLOCK_KEY_INODE,
    );
    if access_masked_dom == 0 {
        return true;
    }

    dget(dir);
    loop {
        let parent_dentry: *mut dentry;
        let mut id = landlock_id {
            key: landlock_key { object: ptr::null_mut() },
            type_: LANDLOCK_KEY_INODE,
        };

        /* Gets all layers allowing all domain accesses. */
        if get_inode_id(dir, &mut id)
            && unmask_layers_fs(domain, id, access_masked_dom, layer_masks_dom, dir)
        {
            /*
             * Stops when all handled accesses are allowed by at
             * least one rule in each layer.
             */
            ret = true;
            break;
        }

        /*
         * Stops at the mount point or the filesystem root for a disconnected
         * directory.
         */
        if dir == mnt_root as *mut dentry || unlikely(IS_ROOT(dir)) {
            break;
        }

        parent_dentry = dget_parent(dir);
        dput(dir);
        dir = parent_dentry;
    }
    dput(dir);
    ret
}

/**
 * current_check_refer_path - Check if a rename or link action is allowed
 */
unsafe fn current_check_refer_path(
    old_dentry: *mut dentry,
    new_dir: *const path,
    new_dentry: *mut dentry,
    removable: bool,
    exchange: bool,
    whiteout: bool,
) -> c_int {
    let subject = landlock_get_applicable_subject(current_cred(), any_fs, ptr::null_mut());
    let allow_parent1: bool;
    let allow_parent2: bool;
    let mut access_request_parent1: access_mask_t;
    let mut access_request_parent2: access_mask_t;
    let mut mnt_dir: path;
    let old_parent: *mut dentry;
    let mut layer_masks_parent1: layer_masks = core::mem::zeroed();
    let mut layer_masks_parent2: layer_masks = core::mem::zeroed();
    let mut request1: landlock_request = core::mem::zeroed();
    let mut request2: landlock_request = core::mem::zeroed();

    if subject.is_null() {
        return 0;
    }

    if unlikely(d_is_negative(old_dentry)) {
        return -ENOENT;
    }
    if exchange {
        if unlikely(d_is_negative(new_dentry)) {
            return -ENOENT;
        }
        access_request_parent1 = get_dentry_access(new_dentry);
    } else {
        access_request_parent1 = 0;
    }
    access_request_parent2 = get_dentry_access(old_dentry);
    if removable {
        access_request_parent1 |= maybe_remove(old_dentry);
        access_request_parent2 |= maybe_remove(new_dentry);
    }

    /*
     * In case of renameat2(2) with RENAME_WHITEOUT, a whiteout object is
     * created in the source location, so we require an additional access
     * right there.
     */
    if whiteout {
        access_request_parent1 |= get_mode_access(S_IFCHR | WHITEOUT_MODE, WHITEOUT_DEV);
    }

    /* The mount points are the same for old and new paths, cf. EXDEV. */
    if (*old_dentry).d_parent == (*new_dir).dentry {
        /*
         * The LANDLOCK_ACCESS_FS_REFER access right is not required
         * for same-directory referer (i.e. no reparenting).
         */
        access_request_parent1 = landlock_init_layer_masks(
            (*subject).domain,
            access_request_parent1 | access_request_parent2,
            &mut layer_masks_parent1,
            LANDLOCK_KEY_INODE,
        );
        if is_access_to_paths_allowed(
            (*subject).domain,
            new_dir,
            access_request_parent1,
            &mut layer_masks_parent1,
            &mut request1,
            ptr::null_mut(),
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        ) {
            return 0;
        }

        landlock_log_denial(subject, &request1);
        return -EACCES;
    }

    access_request_parent1 |= LANDLOCK_ACCESS_FS_REFER;
    access_request_parent2 |= LANDLOCK_ACCESS_FS_REFER;

    /* Saves the common mount point. */
    mnt_dir.mnt = (*new_dir).mnt;
    mnt_dir.dentry = (*(*new_dir).mnt).mnt_root;

    /*
     * old_dentry may be the root of the common mount point and
     * !IS_ROOT(old_dentry) at the same time (e.g. with open_tree() and
     * OPEN_TREE_CLONE).  We do not need to call dget(old_parent) because
     * we keep a reference to old_dentry.
     */
    old_parent = if old_dentry == mnt_dir.dentry {
        old_dentry
    } else {
        (*old_dentry).d_parent
    };

    /* new_dir->dentry is equal to new_dentry->d_parent */
    allow_parent1 = collect_domain_accesses(
        (*subject).domain,
        mnt_dir.dentry,
        old_parent,
        &mut layer_masks_parent1,
    );
    allow_parent2 = collect_domain_accesses(
        (*subject).domain,
        mnt_dir.dentry,
        (*new_dir).dentry,
        &mut layer_masks_parent2,
    );
    if allow_parent1 && allow_parent2 {
        return 0;
    }

    if is_access_to_paths_allowed(
        (*subject).domain,
        &mnt_dir,
        access_request_parent1,
        &mut layer_masks_parent1,
        &mut request1,
        old_dentry,
        access_request_parent2,
        &mut layer_masks_parent2,
        &mut request2,
        if exchange { new_dentry } else { ptr::null_mut() },
    ) {
        return 0;
    }

    if request1.access != 0 {
        request1.audit.u.path.dentry = old_parent;
        landlock_log_denial(subject, &request1);
    }
    if request2.access != 0 {
        request2.audit.u.path.dentry = (*new_dir).dentry;
        landlock_log_denial(subject, &request2);
    }

    /*
     * This prioritizes EACCES over EXDEV for all actions, including
     * renames with RENAME_EXCHANGE.
     */
    if likely(
        is_eacces(&layer_masks_parent1, access_request_parent1)
            || is_eacces(&layer_masks_parent2, access_request_parent2),
    ) {
        return -EACCES;
    }

    /*
     * Gracefully forbids reparenting if the destination directory
     * hierarchy is not a superset of restrictions of the source directory
     * hierarchy, or if LANDLOCK_ACCESS_FS_REFER is not allowed by the
     * source or the destination.
     */
    -EXDEV
}

/* Inode hooks */

unsafe fn hook_inode_free_security_rcu(inode_security: *mut c_void) {
    let inode_sec: *mut landlock_inode_security;

    /*
     * All inodes must already have been untied from their object by
     * release_inode() or hook_sb_delete().
     */
    inode_sec = (inode_security as *mut u8).add(landlock_blob_sizes.lbs_inode)
        as *mut landlock_inode_security;
    WARN_ON_ONCE(!(*inode_sec).object.is_null());
}

/* Super-block hooks */

/*
 * Release the inodes used in a security policy.
 *
 * Cf. fsnotify_unmount_inodes() and evict_inodes()
 */
unsafe fn hook_sb_delete(sb: *mut super_block) {
    let mut prev_inode: *mut inode = ptr::null_mut();

    if !landlock_initialized {
        return;
    }

    spin_lock(&mut (*sb).s_inode_list_lock);
    /* list_for_each_entry(inode, &sb->s_inodes, i_sb_list) */
    let mut inode: *mut inode = ptr::null_mut();
    while !inode.is_null() {
        let object: *mut landlock_object;

        /* Only handles referenced inodes. */
        if icount_read_once(inode) == 0 {
            continue;
        }

        /*
         * Protects against concurrent modification of inode (e.g.
         * from get_inode_object()).
         */
        spin_lock(&mut (*inode).i_lock);
        /*
         * Checks I_FREEING and I_WILL_FREE  to protect against a race
         * condition when release_inode() just called iput(), which
         * could lead to a NULL dereference of inode->security or a
         * second call to iput() for the same Landlock object.  Also
         * checks I_NEW because such inode cannot be tied to an object.
         */
        if (inode_state_read(inode) & (I_FREEING | I_WILL_FREE | I_NEW)) != 0 {
            spin_unlock(&mut (*inode).i_lock);
            continue;
        }

        rcu_read_lock();
        object = rcu_dereference((*landlock_inode(inode)).object);
        if object.is_null() {
            rcu_read_unlock();
            spin_unlock(&mut (*inode).i_lock);
            continue;
        }
        /* Keeps a reference to this inode until the next loop walk. */
        __iget(inode);
        spin_unlock(&mut (*inode).i_lock);

        /*
         * If there is no concurrent release_inode() ongoing, then we
         * are in charge of calling iput() on this inode, otherwise we
         * will just wait for it to finish.
         */
        spin_lock(&mut (*object).lock);
        if (*object).underobj == inode {
            (*object).underobj = ptr::null_mut();
            spin_unlock(&mut (*object).lock);
            rcu_read_unlock();

            rcu_assign_pointer(&mut (*landlock_inode(inode)).object, ptr::null_mut());
            iput_not_last(inode);
        } else {
            spin_unlock(&mut (*object).lock);
            rcu_read_unlock();
        }

        if !prev_inode.is_null() {
            spin_unlock(&mut (*sb).s_inode_list_lock);
            iput(prev_inode);
            /* cond_resched(); */
            spin_lock(&mut (*sb).s_inode_list_lock);
        }
        prev_inode = inode;
    }
    spin_unlock(&mut (*sb).s_inode_list_lock);

    /* Puts the inode reference from the last loop walk, if any. */
    if !prev_inode.is_null() {
        iput(prev_inode);
    }
    /* Waits for pending iput() in release_inode(). */
    wait_var_event_inode_refs(&mut (*landlock_superblock(sb)).inode_refs);
}

unsafe fn log_fs_change_topology_path(
    subject: *const landlock_cred_security,
    handle_layer: size_t,
    path: *const path,
) {
    let request = landlock_request {
        type_: LANDLOCK_REQUEST_FS_CHANGE_TOPOLOGY,
        audit: lsm_audit_data {
            type_: LSM_AUDIT_DATA_PATH,
            u: lsm_audit_data_union { path: *path },
        },
        all_existing_optional_access: 0,
        access: 0,
        layer_masks: ptr::null_mut(),
        layer_plus_one: handle_layer + 1,
        deny_masks: access_masks { fs: 0, scope: 0 },
        quiet_optional_accesses: access_masks { fs: 0, scope: 0 },
    };
    landlock_log_denial(subject, &request);
}

unsafe fn log_fs_change_topology_dentry(
    subject: *const landlock_cred_security,
    handle_layer: size_t,
    dentry: *mut dentry,
) {
    let request = landlock_request {
        type_: LANDLOCK_REQUEST_FS_CHANGE_TOPOLOGY,
        audit: lsm_audit_data {
            type_: LSM_AUDIT_DATA_DENTRY,
            u: lsm_audit_data_union { dentry },
        },
        all_existing_optional_access: 0,
        access: 0,
        layer_masks: ptr::null_mut(),
        layer_plus_one: handle_layer + 1,
        deny_masks: access_masks { fs: 0, scope: 0 },
        quiet_optional_accesses: access_masks { fs: 0, scope: 0 },
    };
    landlock_log_denial(subject, &request);
}

static signal_scope: access_masks = access_masks {
    fs: 0,
    scope: LANDLOCK_SCOPE_SIGNAL,
};

static fs_resolve_unix: access_masks = access_masks {
    fs: LANDLOCK_ACCESS_FS_RESOLVE_UNIX,
    scope: 0,
};

unsafe fn hook_sb_mount(
    _dev_name: *const c_char,
    path: *const path,
    _type: *const c_char,
    _flags: c_ulong,
    _data: *mut c_void,
) -> c_int {
    let mut handle_layer: size_t = 0;
    let subject = landlock_get_applicable_subject(current_cred(), any_fs, &mut handle_layer);

    if subject.is_null() {
        return 0;
    }

    log_fs_change_topology_path(subject, handle_layer, path);
    -EPERM
}

unsafe fn hook_move_mount(_from_path: *const path, to_path: *const path) -> c_int {
    let mut handle_layer: size_t = 0;
    let subject = landlock_get_applicable_subject(current_cred(), any_fs, &mut handle_layer);

    if subject.is_null() {
        return 0;
    }

    log_fs_change_topology_path(subject, handle_layer, to_path);
    -EPERM
}

unsafe fn hook_sb_umount(mnt: *mut vfsmount, _flags: c_int) -> c_int {
    let mut handle_layer: size_t = 0;
    let subject = landlock_get_applicable_subject(current_cred(), any_fs, &mut handle_layer);

    if subject.is_null() {
        return 0;
    }

    log_fs_change_topology_dentry(subject, handle_layer, (*mnt).mnt_root);
    -EPERM
}

unsafe fn hook_sb_remount(sb: *mut super_block, _mnt_opts: *mut c_void) -> c_int {
    let mut handle_layer: size_t = 0;
    let subject = landlock_get_applicable_subject(current_cred(), any_fs, &mut handle_layer);

    if subject.is_null() {
        return 0;
    }

    log_fs_change_topology_dentry(subject, handle_layer, (*sb).s_root);
    -EPERM
}

unsafe fn hook_sb_pivotroot(_old_path: *const path, new_path: *const path) -> c_int {
    let mut handle_layer: size_t = 0;
    let subject = landlock_get_applicable_subject(current_cred(), any_fs, &mut handle_layer);

    if subject.is_null() {
        return 0;
    }

    log_fs_change_topology_path(subject, handle_layer, new_path);
    -EPERM
}

/* Path hooks */

unsafe fn hook_path_link(
    old_dentry: *mut dentry,
    new_dir: *const path,
    new_dentry: *mut dentry,
) -> c_int {
    current_check_refer_path(old_dentry, new_dir, new_dentry, false, false, false)
}

unsafe fn hook_path_rename(
    _old_dir: *const path,
    old_dentry: *mut dentry,
    new_dir: *const path,
    new_dentry: *mut dentry,
    flags: c_uint,
) -> c_int {
    /* old_dir refers to old_dentry->d_parent and new_dir->mnt */
    current_check_refer_path(
        old_dentry,
        new_dir,
        new_dentry,
        true,
        (flags & RENAME_EXCHANGE) != 0,
        (flags & RENAME_WHITEOUT) != 0,
    )
}

unsafe fn hook_path_mkdir(dir: *const path, _dentry: *mut dentry, _mode: umode_t) -> c_int {
    current_check_access_path(dir, LANDLOCK_ACCESS_FS_MAKE_DIR)
}

unsafe fn hook_path_mknod(
    dir: *const path,
    _dentry: *mut dentry,
    mode: umode_t,
    dev: c_uint,
) -> c_int {
    current_check_access_path(dir, get_mode_access(mode, new_decode_dev(dev)))
}

unsafe fn hook_path_symlink(
    dir: *const path,
    _dentry: *mut dentry,
    _old_name: *const c_char,
) -> c_int {
    current_check_access_path(dir, LANDLOCK_ACCESS_FS_MAKE_SYM)
}

unsafe fn hook_path_unlink(dir: *const path, _dentry: *mut dentry) -> c_int {
    current_check_access_path(dir, LANDLOCK_ACCESS_FS_REMOVE_FILE)
}

unsafe fn hook_path_rmdir(dir: *const path, _dentry: *mut dentry) -> c_int {
    current_check_access_path(dir, LANDLOCK_ACCESS_FS_REMOVE_DIR)
}

unsafe fn hook_path_truncate(path: *const path) -> c_int {
    current_check_access_path(path, LANDLOCK_ACCESS_FS_TRUNCATE)
}

/**
 * unmask_scoped_access - Remove access right bits in @masks in all layers
 *                        where @client and @server have the same domain
 */
unsafe fn unmask_scoped_access(
    client: *const landlock_domain,
    server: *const landlock_domain,
    masks: *mut layer_masks,
    access: access_mask_t,
) {
    let mut client_layer: c_int;
    let mut server_layer: c_int;
    let mut client_walker: *const landlock_hierarchy;
    let mut server_walker: *const landlock_hierarchy;

    /* This should not happen. */
    if WARN_ON_ONCE(client.is_null()) {
        return;
    }

    /* Server has no Landlock domain; nothing to clear. */
    if server.is_null() {
        return;
    }

    client_layer = (*client).num_layers - 1;
    client_walker = (*client).hierarchy;
    server_layer = (*server).num_layers - 1;
    server_walker = (*server).hierarchy;

    /*
     * Clears the access bits at all layers where the client domain is the
     * same as the server domain.
     */
    while client_layer > server_layer {
        client_walker = (*client_walker).parent;
        client_layer -= 1;
    }

    while server_layer > client_layer {
        server_walker = (*server_walker).parent;
        server_layer -= 1;
    }

    while client_layer >= 0 {
        let i = client_layer as usize;
        if ((*masks).layers[i].access & access) != 0 && client_walker == server_walker {
            (*masks).layers[i].access &= !access;
        }

        client_walker = (*client_walker).parent;
        server_walker = (*server_walker).parent;
        client_layer -= 1;
    }
}

unsafe fn hook_unix_find(path: *const path, other: *mut sock, flags: c_int) -> c_int {
    let dom_other: *const landlock_domain;
    let subject: *const landlock_cred_security;
    let mut layer_masks: layer_masks = core::mem::zeroed();
    let mut request: landlock_request = core::mem::zeroed();

    /* Lookup for the purpose of saving coredumps is OK. */
    if unlikely((flags & SOCK_COREDUMP) != 0) {
        return 0;
    }

    subject = landlock_get_applicable_subject(current_cred(), fs_resolve_unix, ptr::null_mut());

    if subject.is_null() {
        return 0;
    }

    /*
     * Ignoring return value: that the domains apply was already checked in
     * landlock_get_applicable_subject() above.
     */
    landlock_init_layer_masks(
        (*subject).domain,
        fs_resolve_unix.fs,
        &mut layer_masks,
        LANDLOCK_KEY_INODE,
    );

    /* Checks the layers in which we are connecting within the same domain. */
    unix_state_lock(other);
    if unlikely(
        sock_flag(other, SOCK_DEAD)
            || (*other).sk_socket.is_null()
            || (*(*other).sk_socket).file.is_null(),
    ) {
        unix_state_unlock(other);
        /*
         * We rely on the caller to catch the (non-reversible) SOCK_DEAD
         * condition and retry the lookup.  If we returned an error
         * here, the lookup would not get retried.
         */
        return 0;
    }
    dom_other = (*landlock_cred((*(*(*other).sk_socket).file).f_cred)).domain;

    /* Access to the same (or a lower) domain is always allowed. */
    unmask_scoped_access((*subject).domain, dom_other, &mut layer_masks, fs_resolve_unix.fs);
    unix_state_unlock(other);

    /* Checks the connections to allow-listed paths. */
    if is_access_to_paths_allowed(
        (*subject).domain,
        path,
        fs_resolve_unix.fs,
        &mut layer_masks,
        &mut request,
        ptr::null_mut(),
        0,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    ) {
        return 0;
    }

    landlock_log_denial(subject, &request);
    -EACCES
}

/* File hooks */

/**
 * get_required_file_open_access - Get access needed to open a file
 */
unsafe fn get_required_file_open_access(file: *const file) -> access_mask_t {
    let mut access: access_mask_t = 0;

    if ((*file).f_mode & FMODE_READ) != 0 {
        /* A directory can only be opened in read mode. */
        if S_ISDIR((*file_inode(file)).i_mode) {
            return LANDLOCK_ACCESS_FS_READ_DIR;
        }
        access = LANDLOCK_ACCESS_FS_READ_FILE;
    }
    if ((*file).f_mode & FMODE_WRITE) != 0 {
        access |= LANDLOCK_ACCESS_FS_WRITE_FILE;
    }
    /* __FMODE_EXEC is indeed part of f_flags, not f_mode. */
    if ((*file).f_flags & __FMODE_EXEC) != 0 {
        access |= LANDLOCK_ACCESS_FS_EXECUTE;
    }
    access
}

unsafe fn hook_file_alloc_security(file: *mut file) -> c_int {
    /*
     * Grants all access rights, even if most of them are not checked later
     * on. It is more consistent.
     */
    (*landlock_file(file)).allowed_access = LANDLOCK_MASK_ACCESS_FS;
    0
}

unsafe fn is_device(file: *const file) -> bool {
    let inode = file_inode(file);
    S_ISBLK((*inode).i_mode) || S_ISCHR((*inode).i_mode)
}

unsafe fn hook_file_open(file: *mut file) -> c_int {
    let mut layer_masks: layer_masks = core::mem::zeroed();
    let open_access_request: access_mask_t;
    let full_access_request: access_mask_t;
    let mut allowed_access: access_mask_t;
    let mut optional_access: access_mask_t;
    let subject = landlock_get_applicable_subject((*file).f_cred, any_fs, ptr::null_mut());
    let mut request: landlock_request = core::mem::zeroed();

    if subject.is_null() {
        return 0;
    }

    open_access_request = get_required_file_open_access(file);

    optional_access = LANDLOCK_ACCESS_FS_TRUNCATE;
    if is_device(file) {
        optional_access |= LANDLOCK_ACCESS_FS_IOCTL_DEV;
    }

    full_access_request = open_access_request | optional_access;

    if is_access_to_paths_allowed(
        (*subject).domain,
        &(*file).f_path,
        landlock_init_layer_masks(
            (*subject).domain,
            full_access_request,
            &mut layer_masks,
            LANDLOCK_KEY_INODE,
        ),
        &mut layer_masks,
        &mut request,
        ptr::null_mut(),
        0,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    ) {
        allowed_access = full_access_request;
    } else {
        /*
         * Calculate the actual allowed access rights from layer_masks.
         * Remove the access rights from the full access request which
         * are still unfulfilled in any of the layers.
         */
        allowed_access = full_access_request;
        for i in 0..layer_masks.layers.len() {
            allowed_access &= !layer_masks.layers[i].access;
        }
    }

    /*
     * For operations on already opened files (i.e. ftruncate()), it is the
     * access rights at the time of open() which decide whether the
     * operation is permitted.
     */
    (*landlock_file(file)).allowed_access = allowed_access;
    (*landlock_file(file)).deny_masks = landlock_get_deny_masks(
        _LANDLOCK_ACCESS_FS_OPTIONAL,
        optional_access,
        &layer_masks,
    );
    (*landlock_file(file)).quiet_optional_accesses = landlock_get_quiet_optional_accesses(
        _LANDLOCK_ACCESS_FS_OPTIONAL,
        (*landlock_file(file)).deny_masks,
        &layer_masks,
    );

    if access_mask_subset(open_access_request, allowed_access) {
        return 0;
    }

    /* Sets access to reflect the actual request. */
    request.access = open_access_request;
    landlock_log_denial(subject, &request);
    -EACCES
}

unsafe fn hook_file_truncate(file: *mut file) -> c_int {
    if ((*landlock_file(file)).allowed_access & LANDLOCK_ACCESS_FS_TRUNCATE) != 0 {
        return 0;
    }

    let request = landlock_request {
        type_: LANDLOCK_REQUEST_FS_ACCESS,
        audit: lsm_audit_data {
            type_: LSM_AUDIT_DATA_FILE,
            u: lsm_audit_data_union { file },
        },
        all_existing_optional_access: _LANDLOCK_ACCESS_FS_OPTIONAL,
        access: LANDLOCK_ACCESS_FS_TRUNCATE,
        layer_masks: ptr::null_mut(),
        layer_plus_one: 0,
        deny_masks: (*landlock_file(file)).deny_masks,
        quiet_optional_accesses: (*landlock_file(file)).quiet_optional_accesses,
    };
    landlock_log_denial(landlock_cred((*file).f_cred), &request);
    -EACCES
}

unsafe fn hook_file_ioctl_common(file: *const file, cmd: c_uint, is_compat: bool) -> c_int {
    let allowed_access = (*landlock_file(file)).allowed_access;

    /*
     * It is the access rights at the time of opening the file which
     * determine whether IOCTL can be used on the opened file later.
     */
    if (allowed_access & LANDLOCK_ACCESS_FS_IOCTL_DEV) != 0 {
        return 0;
    }

    if !is_device(file) {
        return 0;
    }

    if if unlikely(is_compat) {
        is_masked_device_ioctl_compat(cmd)
    } else {
        is_masked_device_ioctl(cmd)
    } {
        return 0;
    }

    let ioctlop = lsm_ioctlop_audit {
        path: (*file).f_path,
        cmd,
    };
    let request = landlock_request {
        type_: LANDLOCK_REQUEST_FS_ACCESS,
        audit: lsm_audit_data {
            type_: LSM_AUDIT_DATA_IOCTL_OP,
            u: lsm_audit_data_union { op: &ioctlop },
        },
        all_existing_optional_access: _LANDLOCK_ACCESS_FS_OPTIONAL,
        access: LANDLOCK_ACCESS_FS_IOCTL_DEV,
        layer_masks: ptr::null_mut(),
        layer_plus_one: 0,
        deny_masks: (*landlock_file(file)).deny_masks,
        quiet_optional_accesses: (*landlock_file(file)).quiet_optional_accesses,
    };
    landlock_log_denial(landlock_cred((*file).f_cred), &request);
    -EACCES
}

unsafe fn hook_file_ioctl(file: *mut file, cmd: c_uint, _arg: c_ulong) -> c_int {
    hook_file_ioctl_common(file, cmd, false)
}

unsafe fn hook_file_ioctl_compat(file: *mut file, cmd: c_uint, _arg: c_ulong) -> c_int {
    hook_file_ioctl_common(file, cmd, true)
}

/*
 * Always allow sending signals between threads of the same process.  This
 * ensures consistency with hook_task_kill().
 */
unsafe fn control_current_fowner(fown: *mut fown_struct) -> bool {
    let p: *mut task_struct;

    /*
     * Lock already held by __f_setown(), see commit 26f204380a3c ("fs: Fix
     * file_set_fowner LSM hook inconsistencies").
     */
    lockdep_assert_held(&mut (*fown).lock);

    /*
     * A process-group or session owner (PIDTYPE_PGID/PIDTYPE_SID) fans the
     * signal out to every member at delivery time, so record the domain and
     * let hook_file_send_sigiotask() check the live scope per recipient.
     */
    if (*fown).pid_type != PIDTYPE_PID && (*fown).pid_type != PIDTYPE_TGID {
        return true;
    }

    /*
     * Some callers (e.g. fcntl_dirnotify) may not be in an RCU read-side
     * critical section.
     */
    rcu_read_lock();
    p = pid_task((*fown).pid, (*fown).pid_type);
    if p.is_null() {
        rcu_read_unlock();
        return true;
    }
    let ret = !same_thread_group(p, current);
    rcu_read_unlock();
    ret
}

unsafe fn hook_file_set_fowner(file: *mut file) {
    let prev_dom: *mut landlock_domain;
    let mut fown_subject: landlock_cred_security = core::mem::zeroed();
    let prev_tg: *mut pid;
    let mut fown_tg: *mut pid = ptr::null_mut();
    let mut fown_layer: size_t = 0;

    if control_current_fowner(file_f_owner(file)) {
        let new_subject =
            landlock_get_applicable_subject(current_cred(), signal_scope, &mut fown_layer);
        if !new_subject.is_null() {
            landlock_get_domain((*new_subject).domain);
            fown_subject = *new_subject;
            fown_tg = get_pid(task_tgid(current));
        }
    }

    prev_dom = (*landlock_file(file)).fown_subject.domain;
    prev_tg = (*landlock_file(file)).fown_tg;
    (*landlock_file(file)).fown_subject = fown_subject;
    (*landlock_file(file)).fown_tg = fown_tg;
    (*landlock_file(file)).fown_layer = fown_layer;

    /* May be called in an RCU read-side critical section. */
    landlock_put_domain_deferred(prev_dom);
    put_pid(prev_tg);
}

unsafe fn hook_file_free_security(file: *mut file) {
    put_pid((*landlock_file(file)).fown_tg);
    landlock_put_domain_deferred((*landlock_file(file)).fown_subject.domain);
}

/*
 * static struct security_hook_list landlock_hooks[] __ro_after_init = {
 *     LSM_HOOK_INIT(...),
 * };
 */
static mut landlock_hooks: [security_hook_list; 0] = [];

pub unsafe extern "C" fn landlock_add_fs_hooks() {
    security_add_hooks(
        landlock_hooks.as_mut_ptr(),
        landlock_hooks.len(),
        &mut landlock_lsmid,
    );
}

/* CONFIG_SECURITY_LANDLOCK_KUNIT_TEST:
 * test_cases, test_suite and kunit_test_suite(test_suite) depend on KUnit
 * macros and are preserved in intent by the test function translations above.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
