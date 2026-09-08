// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - System call implementations and user space interfaces
 *
 * Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 * Copyright © 2021-2025 Microsoft Corporation
 */

// C dependencies translated as external kernel/UAPI dependencies:
// <asm/current.h>, <linux/anon_inodes.h>, <linux/bitops.h>,
// <linux/build_bug.h>, <linux/capability.h>, <linux/cleanup.h>,
// <linux/compiler_types.h>, <linux/dcache.h>, <linux/err.h>,
// <linux/errno.h>, <linux/fs.h>, <linux/limits.h>, <linux/mount.h>,
// <linux/path.h>, <linux/sched.h>, <linux/sched/signal.h>,
// <linux/security.h>, <linux/stddef.h>, <linux/syscalls.h>,
// <linux/types.h>, <linux/uaccess.h>, <uapi/linux/landlock.h>,
// "cred.h", "domain.h", "fs.h", "limits.h", "net.h", "ruleset.h",
// "setup.h", "tsync.h", and <trace/events/landlock.h>.

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type SizeT = usize;
type SSizeT = isize;
type LoFFt = i64;
type S32 = i32;
type U32 = u32;
type U16 = u16;
type FModeT = u32;
type AccessMaskT = u64;

#[repr(C)]
pub struct Inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FileOperations {
    pub release: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut File, *mut c_char, SizeT, *mut LoFFt) -> SSizeT>,
    pub write:
        Option<unsafe extern "C" fn(*mut File, *const c_char, SizeT, *mut LoFFt) -> SSizeT>,
}

#[repr(C)]
pub struct SuperBlock {
    pub s_flags: u64,
}

#[repr(C)]
pub struct InodeBacking {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dentry {
    pub d_sb: *mut SuperBlock,
}

#[repr(C)]
pub struct Mount {
    pub mnt_flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Path {
    pub mnt: *mut Mount,
    pub dentry: *mut Dentry,
}

#[repr(C)]
pub struct File {
    pub f_op: *const FileOperations,
    pub f_mode: FModeT,
    pub f_path: Path,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct LandlockQuietMasks {
    pub fs: AccessMaskT,
    pub net: AccessMaskT,
    pub scope: AccessMaskT,
}

#[repr(C)]
pub struct LandlockHandledMasks {
    pub fs: AccessMaskT,
    pub net: AccessMaskT,
}

#[repr(C)]
pub struct Mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct LandlockRuleset {
    pub handled_masks: LandlockHandledMasks,
    pub quiet_masks: LandlockQuietMasks,
    pub lock: Mutex,
}

#[repr(C)]
pub struct LandlockHierarchy {
    pub log_same_exec: bool,
    pub log_new_exec: bool,
    pub log_status: c_int,
}

#[repr(C)]
pub struct LandlockDomain {
    pub hierarchy: *mut LandlockHierarchy,
    pub num_layers: U32,
}

#[repr(C)]
pub struct Cred {
    _private: [u8; 0],
}

#[repr(C)]
pub struct LandlockCredSecurity {
    pub domain: *mut LandlockDomain,
    pub log_subdomains_off: bool,
    pub domain_exec: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LandlockRulesetAttr {
    pub handled_access_fs: AccessMaskT,
    pub handled_access_net: AccessMaskT,
    pub scoped: AccessMaskT,
    pub quiet_access_fs: AccessMaskT,
    pub quiet_access_net: AccessMaskT,
    pub quiet_scoped: AccessMaskT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LandlockPathBeneathAttr {
    pub allowed_access: AccessMaskT,
    pub parent_fd: S32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LandlockNetPortAttr {
    pub allowed_access: AccessMaskT,
    pub port: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LandlockRuleType {
    LandlockRulePathBeneath,
    LandlockRuleNetPort,
}

const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const E2BIG: c_int = 7;
const EOPNOTSUPP: c_int = 95;
const ENOMSG: c_int = 42;
const EBADF: c_int = 9;
const EBADFD: c_int = 77;
const EPERM: c_int = 1;
const ENOMEM: c_int = 12;

const LANDLOCK_CREATE_RULESET_VERSION: U32 = 1 << 0;
const LANDLOCK_CREATE_RULESET_ERRATA: U32 = 1 << 1;
const LANDLOCK_ADD_RULE_QUIET: U32 = 1 << 0;
const LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF: U32 = 1 << 0;
const LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON: U32 = 1 << 1;
const LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF: U32 = 1 << 2;
const LANDLOCK_RESTRICT_SELF_TSYNC: U32 = 1 << 3;
const LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS: U32 = 1 << 4;
const LANDLOCK_MASK_RESTRICT_SELF: U32 = LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF
    | LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON
    | LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF
    | LANDLOCK_RESTRICT_SELF_TSYNC
    | LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS;

const LANDLOCK_MASK_ACCESS_FS: AccessMaskT = !0;
const LANDLOCK_MASK_ACCESS_NET: AccessMaskT = !0;
const LANDLOCK_MASK_SCOPE: AccessMaskT = !0;
const PAGE_SIZE: SizeT = 4096;
const U16_MAX: u64 = U16::MAX as u64;
const O_RDWR: c_int = 0o2;
const O_CLOEXEC: c_int = 0o2000000;
const FMODE_CAN_READ: FModeT = 0x1;
const FMODE_CAN_WRITE: FModeT = 0x2;
const MNT_INTERNAL: u64 = 0x4000;
const SB_NOUSER: u64 = 1 << 31;
const CAP_SYS_ADMIN: c_int = 21;
const LANDLOCK_LOG_DISABLED: c_int = 0;
const LANDLOCK_LOG_PENDING: c_int = 1;

unsafe extern "C" {
    static landlock_initialized: bool;
    static landlock_errata: c_int;
    static mut current: *mut c_void;

    fn pr_warn_once(fmt: *const c_char, ...);
    fn copy_struct_from_user(dst: *mut c_void, ksize: SizeT, src: *const c_void, usize: SizeT)
        -> c_int;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, size: SizeT) -> c_int;
    fn landlock_put_ruleset(ruleset: *mut LandlockRuleset);
    fn landlock_get_ruleset(ruleset: *mut LandlockRuleset);
    fn landlock_create_ruleset(
        handled_access_fs: AccessMaskT,
        handled_access_net: AccessMaskT,
        scoped: AccessMaskT,
    ) -> *mut LandlockRuleset;
    fn anon_inode_getfd(
        name: *const c_char,
        fops: *const FileOperations,
        priv_data: *mut c_void,
        flags: c_int,
    ) -> c_int;
    fn trace_landlock_create_ruleset(ruleset: *mut LandlockRuleset);
    fn trace_landlock_create_domain(domain: *mut LandlockDomain, ruleset: *mut LandlockRuleset);
    fn trace_landlock_enforce_domain(
        domain: *mut LandlockDomain,
        enabled: bool,
        process_wide: bool,
        no_new_privs: bool,
    );
    fn fdget(fd: c_int) -> *mut File;
    fn fdput(file: *mut File);
    fn path_get(path: *mut Path);
    fn path_put(path: *mut Path);
    fn d_backing_inode(dentry: *mut Dentry) -> *mut InodeBacking;
    fn IS_PRIVATE(inode: *mut InodeBacking) -> bool;
    fn landlock_append_fs_rule(
        ruleset: *mut LandlockRuleset,
        path: *mut Path,
        allowed_access: AccessMaskT,
        flags: U32,
    ) -> c_int;
    fn landlock_append_net_rule(
        ruleset: *mut LandlockRuleset,
        port: u64,
        allowed_access: AccessMaskT,
        flags: U32,
    ) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn ERR_PTR(err: c_int) -> *mut LandlockRuleset;
    fn prepare_creds() -> *mut Cred;
    fn abort_creds(cred: *mut Cred);
    fn commit_creds(cred: *mut Cred) -> c_int;
    fn landlock_cred(cred: *mut Cred) -> *mut LandlockCredSecurity;
    fn mutex_lock(lock: *mut Mutex);
    fn mutex_unlock(lock: *mut Mutex);
    fn landlock_merge_ruleset(
        domain: *mut LandlockDomain,
        ruleset: *mut LandlockRuleset,
    ) -> *mut LandlockDomain;
    fn landlock_put_domain(domain: *mut LandlockDomain);
    fn landlock_restrict_sibling_threads(
        old: *const Cred,
        new_cred: *mut Cred,
        flags: U32,
    ) -> c_int;
    fn current_cred() -> *const Cred;
    fn task_no_new_privs(task: *mut c_void) -> bool;
    fn task_set_no_new_privs(task: *mut c_void);
    fn current_user_ns() -> *mut c_void;
    fn ns_capable_noaudit(ns: *mut c_void, cap: c_int) -> bool;
    fn get_nr_threads(task: *mut c_void) -> c_int;
}

#[inline]
fn likely(value: bool) -> bool {
    value
}

#[inline]
const fn bit(nr: U32) -> u64 {
    1u64 << nr
}

unsafe fn is_initialized() -> bool {
    if likely(unsafe { landlock_initialized }) {
        return true;
    }

    unsafe {
        pr_warn_once(
            concat!(
                "Disabled but requested by user space. ",
                "You should enable Landlock at boot time: ",
                "https://docs.kernel.org/userspace-api/landlock.html#boot-time-configuration\n\0"
            )
            .as_ptr() as *const c_char,
        );
    }
    false
}

/**
 * copy_min_struct_from_user - Safe future-proof argument copying
 *
 * Extend copy_struct_from_user() to check for consistent user buffer.
 *
 * @dst: Kernel space pointer or NULL.
 * @ksize: Actual size of the data pointed to by @dst.
 * @ksize_min: Minimal required size to be copied.
 * @src: User space pointer or NULL.
 * @usize: (Alleged) size of the data pointed to by @src.
 *
 * Return: 0 on success, -errno on failure.
 */
#[inline(always)]
unsafe fn copy_min_struct_from_user(
    dst: *mut c_void,
    ksize: SizeT,
    ksize_min: SizeT,
    src: *const c_void,
    usize: SizeT,
) -> c_int {
    /* Checks buffer inconsistencies. */
    debug_assert!(!dst.is_null());
    if src.is_null() {
        return -EFAULT;
    }

    /* Checks size ranges. */
    debug_assert!(ksize > 0);
    debug_assert!(ksize >= ksize_min);
    if usize < ksize_min {
        return -EINVAL;
    }
    if usize > PAGE_SIZE {
        return -E2BIG;
    }

    /* Copies user buffer and fills with zeros. */
    unsafe { copy_struct_from_user(dst, ksize, src, usize) }
}

/*
 * This function only contains arithmetic operations with constants, leading to
 * BUILD_BUG_ON().  The related code is evaluated and checked at build time,
 * but it is then ignored thanks to compiler optimizations.
 */
fn build_check_abi() {
    let mut ruleset_size: SizeT;
    let mut path_beneath_size: SizeT;
    let mut net_port_size: SizeT;

    /*
     * For each user space ABI structures, first checks that there is no
     * hole in them, then checks that all architectures have the same
     * struct size.
     */
    ruleset_size = size_of::<AccessMaskT>();
    ruleset_size += size_of::<AccessMaskT>();
    ruleset_size += size_of::<AccessMaskT>();
    ruleset_size += size_of::<AccessMaskT>();
    ruleset_size += size_of::<AccessMaskT>();
    ruleset_size += size_of::<AccessMaskT>();
    const _: [(); 48] = [(); size_of::<LandlockRulesetAttr>()];
    debug_assert_eq!(size_of::<LandlockRulesetAttr>(), ruleset_size);
    debug_assert_eq!(size_of::<LandlockRulesetAttr>(), 48);

    path_beneath_size = size_of::<AccessMaskT>();
    path_beneath_size += size_of::<S32>();
    debug_assert_eq!(size_of::<LandlockPathBeneathAttr>(), path_beneath_size);
    debug_assert_eq!(size_of::<LandlockPathBeneathAttr>(), 12);

    net_port_size = size_of::<AccessMaskT>();
    net_port_size += size_of::<u64>();
    debug_assert_eq!(size_of::<LandlockNetPortAttr>(), net_port_size);
    debug_assert_eq!(size_of::<LandlockNetPortAttr>(), 16);
}

/* Ruleset handling */

unsafe extern "C" fn fop_ruleset_release(_inode: *mut Inode, filp: *mut File) -> c_int {
    let ruleset = unsafe { (*filp).private_data as *mut LandlockRuleset };

    unsafe { landlock_put_ruleset(ruleset) };
    0
}

unsafe extern "C" fn fop_dummy_read(
    _filp: *mut File,
    _buf: *mut c_char,
    _size: SizeT,
    _ppos: *mut LoFFt,
) -> SSizeT {
    /* Dummy handler to enable FMODE_CAN_READ. */
    -(EINVAL as SSizeT)
}

unsafe extern "C" fn fop_dummy_write(
    _filp: *mut File,
    _buf: *const c_char,
    _size: SizeT,
    _ppos: *mut LoFFt,
) -> SSizeT {
    /* Dummy handler to enable FMODE_CAN_WRITE. */
    -(EINVAL as SSizeT)
}

/*
 * A ruleset file descriptor enables to build a ruleset by adding (i.e.
 * writing) rule after rule, without relying on the task's context.  This
 * reentrant design is also used in a read way to enforce the ruleset on the
 * current task.
 */
static RULESET_FOPS: FileOperations = FileOperations {
    release: Some(fop_ruleset_release),
    read: Some(fop_dummy_read),
    write: Some(fop_dummy_write),
};

/*
 * The Landlock ABI version should be incremented for each new Landlock-related
 * user space visible change (e.g. Landlock syscalls).  This version should
 * only be incremented once per Linux release.  When incrementing, the date in
 * Documentation/userspace-api/landlock.rst should be updated to reflect the
 * UAPI change.
 * If the change involves a fix that requires userspace awareness, also update
 * the errata documentation in Documentation/userspace-api/landlock.rst .
 */
pub const landlock_abi_version: c_int = 11;

/**
 * sys_landlock_create_ruleset - Create a new ruleset
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_landlock_create_ruleset(
    attr: *const LandlockRulesetAttr,
    size: SizeT,
    flags: U32,
) -> c_int {
    let mut ruleset_attr: LandlockRulesetAttr = unsafe { core::mem::zeroed() };
    let ruleset: *mut LandlockRuleset;
    let mut err: c_int;
    let ruleset_fd: c_int;

    /* Build-time checks. */
    build_check_abi();

    if !unsafe { is_initialized() } {
        return -EOPNOTSUPP;
    }

    if flags != 0 {
        if !attr.is_null() || size != 0 {
            return -EINVAL;
        }

        if flags == LANDLOCK_CREATE_RULESET_VERSION {
            return landlock_abi_version;
        }

        if flags == LANDLOCK_CREATE_RULESET_ERRATA {
            return unsafe { landlock_errata };
        }

        return -EINVAL;
    }

    /* Copies raw user space buffer. */
    err = unsafe {
        copy_min_struct_from_user(
            &mut ruleset_attr as *mut _ as *mut c_void,
            size_of::<LandlockRulesetAttr>(),
            size_of::<AccessMaskT>(),
            attr as *const c_void,
            size,
        )
    };
    if err != 0 {
        return err;
    }

    /* Checks content (and 32-bits cast). */
    if (ruleset_attr.handled_access_fs | LANDLOCK_MASK_ACCESS_FS) != LANDLOCK_MASK_ACCESS_FS {
        return -EINVAL;
    }

    /* Checks network content (and 32-bits cast). */
    if (ruleset_attr.handled_access_net | LANDLOCK_MASK_ACCESS_NET) != LANDLOCK_MASK_ACCESS_NET {
        return -EINVAL;
    }

    /* Checks IPC scoping content (and 32-bits cast). */
    if (ruleset_attr.scoped | LANDLOCK_MASK_SCOPE) != LANDLOCK_MASK_SCOPE {
        return -EINVAL;
    }

    /*
     * Check that quiet masks are subsets of the respective handled masks.
     * Because of the checks above this is sufficient to also ensure that
     * the quiet masks are valid access masks.
     */
    if (ruleset_attr.quiet_access_fs | ruleset_attr.handled_access_fs)
        != ruleset_attr.handled_access_fs
    {
        return -EINVAL;
    }
    if (ruleset_attr.quiet_access_net | ruleset_attr.handled_access_net)
        != ruleset_attr.handled_access_net
    {
        return -EINVAL;
    }
    if (ruleset_attr.quiet_scoped | ruleset_attr.scoped) != ruleset_attr.scoped {
        return -EINVAL;
    }

    /* Checks arguments and transforms to kernel struct. */
    ruleset = unsafe {
        landlock_create_ruleset(
            ruleset_attr.handled_access_fs,
            ruleset_attr.handled_access_net,
            ruleset_attr.scoped,
        )
    };
    if unsafe { IS_ERR(ruleset as *const c_void) } {
        return unsafe { PTR_ERR(ruleset as *const c_void) };
    }

    unsafe {
        (*ruleset).quiet_masks.fs = ruleset_attr.quiet_access_fs;
        (*ruleset).quiet_masks.net = ruleset_attr.quiet_access_net;
        (*ruleset).quiet_masks.scope = ruleset_attr.quiet_scoped;
    }

    /*
     * Emits before anon_inode_getfd() installs the file descriptor, while
     * the ruleset is still private to this thread: no lock is needed, and
     * the event cannot race a concurrent close() freeing the ruleset under
     * the tracepoint's BTF read.  This is the last point at which the
     * ruleset is guaranteed alive and unshared.
     */
    unsafe { trace_landlock_create_ruleset(ruleset) };

    /* Creates anonymous FD referring to the ruleset. */
    ruleset_fd = unsafe {
        anon_inode_getfd(
            b"[landlock-ruleset]\0".as_ptr() as *const c_char,
            &RULESET_FOPS,
            ruleset as *mut c_void,
            O_RDWR | O_CLOEXEC,
        )
    };
    if ruleset_fd < 0 {
        unsafe { landlock_put_ruleset(ruleset) };
    }
    ruleset_fd
}

/*
 * Returns an owned ruleset from a FD. It is thus needed to call
 * landlock_put_ruleset() on the return value.
 */
unsafe fn get_ruleset_from_fd(fd: c_int, mode: FModeT) -> *mut LandlockRuleset {
    let file = unsafe { fdget(fd) };
    let ruleset: *mut LandlockRuleset;

    if file.is_null() {
        return unsafe { ERR_PTR(-EBADF) };
    }

    /* Checks FD type and access right. */
    if unsafe { (*file).f_op != &RULESET_FOPS as *const FileOperations } {
        unsafe { fdput(file) };
        return unsafe { ERR_PTR(-EBADFD) };
    }
    if unsafe { ((*file).f_mode & mode) == 0 } {
        unsafe { fdput(file) };
        return unsafe { ERR_PTR(-EPERM) };
    }
    ruleset = unsafe { (*file).private_data as *mut LandlockRuleset };
    unsafe {
        landlock_get_ruleset(ruleset);
        fdput(file);
    }
    ruleset
}

/* Path handling */

/*
 * @path: Must call put_path(@path) after the call if it succeeded.
 */
unsafe fn get_path_from_fd(fd: S32, path: *mut Path) -> c_int {
    let file = unsafe { fdget(fd) };

    debug_assert_eq!(size_of::<S32>(), size_of::<S32>());

    if file.is_null() {
        return -EBADF;
    }
    /*
     * Forbids ruleset FDs, internal filesystems (e.g. nsfs), including
     * pseudo filesystems that will never be mountable (e.g. sockfs,
     * pipefs).
     */
    if unsafe {
        (*file).f_op == &RULESET_FOPS as *const FileOperations
            || ((*(*file).f_path.mnt).mnt_flags & MNT_INTERNAL) != 0
            || ((*(*(*file).f_path.dentry).d_sb).s_flags & SB_NOUSER) != 0
            || IS_PRIVATE(d_backing_inode((*file).f_path.dentry))
    } {
        unsafe { fdput(file) };
        return -EBADFD;
    }

    unsafe {
        *path = (*file).f_path;
        path_get(path);
        fdput(file);
    }
    0
}

unsafe fn add_rule_path_beneath(
    ruleset: *mut LandlockRuleset,
    rule_attr: *const c_void,
    flags: U32,
) -> c_int {
    let mut path_beneath_attr: LandlockPathBeneathAttr = unsafe { core::mem::zeroed() };
    let mut path: Path = Path {
        mnt: ptr::null_mut(),
        dentry: ptr::null_mut(),
    };
    let res: c_int;
    let mut err: c_int;
    let mask: AccessMaskT;

    /* Copies raw user space buffer. */
    res = unsafe {
        copy_from_user(
            &mut path_beneath_attr as *mut _ as *mut c_void,
            rule_attr,
            size_of::<LandlockPathBeneathAttr>(),
        )
    };
    if res != 0 {
        return -EFAULT;
    }

    /*
     * Informs about useless rule: empty allowed_access (i.e. deny rules)
     * are ignored in path walks.  However, the rule is not useless if it is
     * there to hold a quiet flag.
     */
    if flags == 0 && path_beneath_attr.allowed_access == 0 {
        return -ENOMSG;
    }

    /* Checks that allowed_access matches the @ruleset constraints. */
    mask = unsafe { (*ruleset).handled_masks.fs };
    if (path_beneath_attr.allowed_access | mask) != mask {
        return -EINVAL;
    }

    /* Checks for useless quiet flag. */
    if (flags & LANDLOCK_ADD_RULE_QUIET) != 0 && unsafe { (*ruleset).quiet_masks.fs } == 0 {
        return -EINVAL;
    }

    /* Gets and checks the new rule. */
    err = unsafe { get_path_from_fd(path_beneath_attr.parent_fd, &mut path) };
    if err != 0 {
        return err;
    }

    /* Imports the new rule. */
    err = unsafe {
        landlock_append_fs_rule(
            ruleset,
            &mut path,
            path_beneath_attr.allowed_access,
            flags,
        )
    };
    unsafe { path_put(&mut path) };
    err
}

unsafe fn add_rule_net_port(
    ruleset: *mut LandlockRuleset,
    rule_attr: *const c_void,
    flags: U32,
) -> c_int {
    let mut net_port_attr: LandlockNetPortAttr = unsafe { core::mem::zeroed() };
    let res: c_int;
    let mask: AccessMaskT;

    /* Copies raw user space buffer. */
    res = unsafe {
        copy_from_user(
            &mut net_port_attr as *mut _ as *mut c_void,
            rule_attr,
            size_of::<LandlockNetPortAttr>(),
        )
    };
    if res != 0 {
        return -EFAULT;
    }

    /*
     * Informs about useless rule: empty allowed_access (i.e. deny rules)
     * are ignored by network actions.  However, the rule is not useless if
     * it is there to hold a quiet flag.
     */
    if flags == 0 && net_port_attr.allowed_access == 0 {
        return -ENOMSG;
    }

    /* Checks that allowed_access matches the @ruleset constraints. */
    mask = unsafe { (*ruleset).handled_masks.net };
    if (net_port_attr.allowed_access | mask) != mask {
        return -EINVAL;
    }

    /* Checks for useless quiet flag. */
    if (flags & LANDLOCK_ADD_RULE_QUIET) != 0 && unsafe { (*ruleset).quiet_masks.net } == 0 {
        return -EINVAL;
    }

    /* Denies inserting a rule with port greater than 65535. */
    if net_port_attr.port > U16_MAX {
        return -EINVAL;
    }

    /* Imports the new rule. */
    unsafe {
        landlock_append_net_rule(
            ruleset,
            net_port_attr.port,
            net_port_attr.allowed_access,
            flags,
        )
    }
}

/**
 * sys_landlock_add_rule - Add a new rule to a ruleset
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_landlock_add_rule(
    ruleset_fd: c_int,
    rule_type: LandlockRuleType,
    rule_attr: *const c_void,
    flags: U32,
) -> c_int {
    let ruleset: *mut LandlockRuleset;
    let ret: c_int;

    if !unsafe { is_initialized() } {
        return -EOPNOTSUPP;
    }

    if flags != 0 && flags != LANDLOCK_ADD_RULE_QUIET {
        return -EINVAL;
    }

    /* Gets and checks the ruleset. */
    ruleset = unsafe { get_ruleset_from_fd(ruleset_fd, FMODE_CAN_WRITE) };
    if unsafe { IS_ERR(ruleset as *const c_void) } {
        return unsafe { PTR_ERR(ruleset as *const c_void) };
    }

    ret = match rule_type {
        LandlockRuleType::LandlockRulePathBeneath => unsafe {
            add_rule_path_beneath(ruleset, rule_attr, flags)
        },
        LandlockRuleType::LandlockRuleNetPort => unsafe {
            add_rule_net_port(ruleset, rule_attr, flags)
        },
        _ => -EINVAL,
    };
    unsafe { landlock_put_ruleset(ruleset) };
    ret
}

/* Enforcement */

/**
 * sys_landlock_restrict_self - Enforce a ruleset on the calling thread
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_landlock_restrict_self(ruleset_fd: c_int, flags: U32) -> c_int {
    let mut ruleset: *mut LandlockRuleset = ptr::null_mut();
    let mut new_dom: *mut LandlockDomain = ptr::null_mut();
    let new_cred: *mut Cred;
    let new_llcred: *mut LandlockCredSecurity;
    let process_wide: bool;
    let log_same_exec: bool;
    let log_new_exec: bool;
    let log_subdomains: bool;
    let mut prev_log_subdomains: bool = false;

    if !unsafe { is_initialized() } {
        return -EOPNOTSUPP;
    }

    if (flags | LANDLOCK_MASK_RESTRICT_SELF) != LANDLOCK_MASK_RESTRICT_SELF {
        return -EINVAL;
    }

    /*
     * Similar checks as for seccomp(2), except that an -EPERM may be
     * returned.  LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS fulfills this
     * requirement.
     */
    if (flags & LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS) == 0
        && !unsafe { task_no_new_privs(current) }
        && !unsafe { ns_capable_noaudit(current_user_ns(), CAP_SYS_ADMIN) }
    {
        return -EPERM;
    }

    /* Translates "off" flag to boolean. */
    log_same_exec = (flags & LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF) == 0;
    /* Translates "on" flag to boolean. */
    log_new_exec = (flags & LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON) != 0;
    /* Translates "off" flag to boolean. */
    log_subdomains = (flags & LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF) == 0;

    /*
     * It is allowed to set LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF with
     * -1 as ruleset_fd, optionally combined with
     * LANDLOCK_RESTRICT_SELF_TSYNC to propagate this configuration to all
     * threads.  No other flag must be set.
     */
    if !(ruleset_fd == -1
        && (flags & !LANDLOCK_RESTRICT_SELF_TSYNC)
            == LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF)
    {
        /* Gets and checks the ruleset. */
        ruleset = unsafe { get_ruleset_from_fd(ruleset_fd, FMODE_CAN_READ) };
        if unsafe { IS_ERR(ruleset as *const c_void) } {
            return unsafe { PTR_ERR(ruleset as *const c_void) };
        }
    }

    /* Prepares new credentials. */
    new_cred = unsafe { prepare_creds() };
    if new_cred.is_null() {
        if !ruleset.is_null() {
            unsafe { landlock_put_ruleset(ruleset) };
        }
        return -ENOMEM;
    }

    new_llcred = unsafe { landlock_cred(new_cred) };

    // #ifdef CONFIG_SECURITY_LANDLOCK_LOG
    prev_log_subdomains = unsafe { !(*new_llcred).log_subdomains_off };
    unsafe {
        (*new_llcred).log_subdomains_off = !prev_log_subdomains || !log_subdomains;
    }
    // #endif /* CONFIG_SECURITY_LANDLOCK_LOG */

    /*
     * The only case when a ruleset may not be set is if
     * LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF is set (optionally with
     * LANDLOCK_RESTRICT_SELF_TSYNC) and ruleset_fd is -1.  We could
     * optimize this case by not calling commit_creds() if this flag was
     * already set, but it is not worth the complexity.
     */
    if !ruleset.is_null() {
        /*
         * There is no possible race condition while copying and
         * manipulating the current credentials because they are
         * dedicated per thread.
         */
        unsafe { mutex_lock(&mut (*ruleset).lock) };
        new_dom = unsafe { landlock_merge_ruleset((*new_llcred).domain, ruleset) };
        if unsafe { IS_ERR(new_dom as *const c_void) } {
            unsafe {
                mutex_unlock(&mut (*ruleset).lock);
                abort_creds(new_cred);
                landlock_put_ruleset(ruleset);
            }
            return unsafe { PTR_ERR(new_dom as *const c_void) };
        }
        /*
         * Emits the domain-creation event while @ruleset->lock is still
         * held, right after the merge, so an eBPF program attached to
         * the tracepoint reads the exact ruleset that was merged into
         * the domain: a consistent snapshot that a concurrent
         * landlock_add_rule() (which holds the same lock) cannot
         * modify.
         *
         * This must come before the thread-sync wait below.  Holding
         * @ruleset->lock across landlock_restrict_sibling_threads()
         * would hang: a sibling thread blocked in landlock_add_rule()
         * on the same @ruleset->lock cannot run the task_work that
         * thread-sync waits for (the lock wait is uninterruptible).
         * Emitting here keeps the lock off the thread-sync path.
         *
         * The trade-off is that the event fires for a domain that a
         * later (rare) thread-sync failure aborts.  That path emits the
         * matching free_domain event so the create/free pair stays
         * balanced (see the thread-sync error path below).
         */
        unsafe {
            trace_landlock_create_domain(new_dom, ruleset);
            mutex_unlock(&mut (*ruleset).lock);
        }

        // #ifdef CONFIG_SECURITY_LANDLOCK_LOG
        unsafe {
            (*(*new_dom).hierarchy).log_same_exec = log_same_exec;
            (*(*new_dom).hierarchy).log_new_exec = log_new_exec;
            /*
             * The creation event fired above, so move the domain out of
             * LANDLOCK_LOG_UNCOMMITTED: its free_domain event must fire
             * too, even if a thread-sync failure aborts it below.  Audit
             * logging may still be disabled (DISABLED); tracing observes it
             * anyway.
             */
            if (!log_same_exec && !log_new_exec) || !prev_log_subdomains {
                (*(*new_dom).hierarchy).log_status = LANDLOCK_LOG_DISABLED;
            } else {
                (*(*new_dom).hierarchy).log_status = LANDLOCK_LOG_PENDING;
            }
        }
        // #endif /* CONFIG_SECURITY_LANDLOCK_LOG */

        /* Replaces the old (prepared) domain. */
        unsafe {
            landlock_put_domain((*new_llcred).domain);
            (*new_llcred).domain = new_dom;
        }

        // #ifdef CONFIG_SECURITY_LANDLOCK_LOG
        unsafe {
            (*new_llcred).domain_exec |= bit((*new_dom).num_layers - 1);
        }
        // #endif /* CONFIG_SECURITY_LANDLOCK_LOG */
    }

    if (flags & LANDLOCK_RESTRICT_SELF_TSYNC) != 0 {
        let err: c_int =
            unsafe { landlock_restrict_sibling_threads(current_cred(), new_cred, flags) };
        if err != 0 {
            /*
             * Thread-sync failed (rare), so the new domain is
             * aborted instead of committed.  Its creation event
             * already fired above, so the imminent free must emit
             * the matching free_domain event to keep the
             * create/free pair balanced; no special log_status is
             * set here.
             */
            unsafe {
                abort_creds(new_cred);
                if !ruleset.is_null() {
                    landlock_put_ruleset(ruleset);
                }
            }
            return err;
        }
    }

    /* Sets no_new_privs past the last point of failure. */
    if (flags & LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS) != 0 {
        unsafe { task_set_no_new_privs(current) };
    }

    /* Whole process: thread-sync swept siblings, or single-threaded. */
    process_wide =
        (flags & LANDLOCK_RESTRICT_SELF_TSYNC) != 0 || unsafe { get_nr_threads(current) } == 1;
    unsafe { commit_creds(new_cred) };

    /* The caller commits last, so its event concludes the operation. */
    if !ruleset.is_null() {
        unsafe {
            trace_landlock_enforce_domain(new_dom, true, process_wide, task_no_new_privs(current));
            landlock_put_ruleset(ruleset);
        }
    }

    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
