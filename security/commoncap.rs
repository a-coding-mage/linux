// SPDX-License-Identifier: GPL-2.0-or-later
// Common capabilities, needed by capability module

use core::ffi::CStr;
use core::mem;
use core::ptr::{self, NonNull};

// External kernel types and functions (dependencies from other kernel headers)
#[repr(C)]
pub struct cred {
    pub cap_effective: kernel_cap_t,
    pub cap_permitted: kernel_cap_t,
    pub cap_inheritable: kernel_cap_t,
    pub cap_ambient: kernel_cap_t,
    pub cap_bset: kernel_cap_t,
    pub euid: kuid_t,
    pub uid: kuid_t,
    pub suid: kuid_t,
    pub fsuid: kuid_t,
    pub egid: kgid_t,
    pub gid: kgid_t,
    pub sgid: kgid_t,
    pub fsgid: kgid_t,
    pub user_ns: *mut user_namespace,
    pub securebits: u32,
}

#[repr(C)]
pub struct user_namespace {
    pub parent: *mut user_namespace,
    pub level: u32,
    pub owner: kuid_t,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    pub d_sb: *mut super_block,
}

#[repr(C)]
pub struct super_block {
    pub s_user_ns: *mut user_namespace,
}

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
}

#[repr(C)]
pub struct mnt_idmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct linux_binprm {
    pub cred: *mut cred,
    pub filename: *const u8,
    pub unsafe_: u32,
    pub per_clear: u32,
    pub secureexec: u32,
}

#[repr(C)]
pub struct file {
    pub f_path: file_path,
}

#[repr(C)]
pub struct file_path {
    pub mnt: *mut vfsmount,
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct vfsmount {
    pub mnt_sb: *mut super_block,
}

#[repr(C)]
pub struct timespec64 {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}

#[repr(C)]
pub struct kernel_cap_t {
    pub val: u64,
}

#[repr(C)]
pub struct cpu_vfs_cap_data {
    pub magic_etc: u32,
    pub permitted: kernel_cap_t,
    pub inheritable: kernel_cap_t,
    pub rootid: kuid_t,
}

#[repr(C)]
pub struct vfs_cap_data {
    pub magic_etc: u32,
    pub data: [[u32; 2]; 2],
}

#[repr(C)]
pub struct vfs_ns_cap_data {
    pub magic_etc: u32,
    pub rootid: u32,
    pub data: [[u32; 2]; 2],
}

#[repr(transparent)]
pub struct kuid_t(u32);

#[repr(transparent)]
pub struct kgid_t(u32);

#[repr(transparent)]
pub struct vfsuid_t {
    val: u32,
}

#[repr(transparent)]
pub struct uid_t(u32);

#[repr(transparent)]
pub struct gid_t(u32);

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

// Constants (from kernel headers)
pub const CAP_SYS_TIME: i32 = 25;
pub const CAP_SYS_PTRACE: i32 = 19;
pub const CAP_SETPCAP: i32 = 8;
pub const CAP_SYS_ADMIN: i32 = 21;
pub const CAP_SYS_RAWIO: i32 = 17;
pub const CAP_SETFCAP: i32 = 31;
pub const CAP_SYS_NICE: i32 = 23;

pub const EPERM: i32 = -1;
pub const EINVAL: i32 = -22;
pub const ENOMEM: i32 = -12;
pub const ENODATA: i32 = -61;
pub const EOPNOTSUPP: i32 = -95;
pub const EOVERFLOW: i32 = -75;
pub const ENOSYS: i32 = -38;

pub const CAP_OPT_NONE: u32 = 0;
pub const CAP_OPT_NOAUDIT: u32 = 1;

pub const VFS_CAP_REVISION_1: u32 = 0x01000000;
pub const VFS_CAP_REVISION_2: u32 = 0x02000000;
pub const VFS_CAP_REVISION_3: u32 = 0x03000000;
pub const VFS_CAP_REVISION_MASK: u32 = 0xff000000;
pub const VFS_CAP_FLAGS_EFFECTIVE: u32 = 0x000001;
pub const CAP_VALID_MASK: u64 = 0x0000003fffffffff;
pub const CAP_FULL_SET: kernel_cap_t = kernel_cap_t { val: 0x0000003fffffffff };

pub const XATTR_CAPS_SZ_1: usize = 12;
pub const XATTR_CAPS_SZ_2: usize = 20;
pub const XATTR_CAPS_SZ_3: usize = 24;
pub const XATTR_CAPS_SZ: usize = 24;

pub const VFS_CAP_U32: usize = 2;

pub const SECURE_KEEP_CAPS: u32 = 4;
pub const SECURE_KEEP_CAPS_LOCKED: u32 = 5;
pub const SECURE_NOROOT: u32 = 0;
pub const SECURE_NOROOT_LOCKED: u32 = 1;
pub const SECURE_NO_SETUID_FIXUP: u32 = 2;
pub const SECURE_NO_SETUID_FIXUP_LOCKED: u32 = 3;
pub const SECURE_NO_CAP_AMBIENT_RAISE: u32 = 6;
pub const SECURE_NO_CAP_AMBIENT_RAISE_LOCKED: u32 = 7;

pub const SECURE_ALL_BITS: u32 = 0x0000007f;
pub const SECURE_ALL_LOCKS: u32 = 0x00000055;
pub const SECURE_ALL_UNPRIVILEGED: u32 = 0x00000000;

pub const PR_CAPBSET_READ: i32 = 23;
pub const PR_CAPBSET_DROP: i32 = 24;
pub const PR_SET_SECUREBITS: i32 = 28;
pub const PR_GET_SECUREBITS: i32 = 29;
pub const PR_GET_KEEPCAPS: i32 = 30;
pub const PR_SET_KEEPCAPS: i32 = 8;
pub const PR_CAP_AMBIENT: i32 = 47;
pub const PR_CAP_AMBIENT_IS_SET: i32 = 1;
pub const PR_CAP_AMBIENT_RAISE: i32 = 2;
pub const PR_CAP_AMBIENT_LOWER: i32 = 3;
pub const PR_CAP_AMBIENT_CLEAR_ALL: i32 = 4;

pub const LSM_SETID_RE: i32 = 0;
pub const LSM_SETID_ID: i32 = 1;
pub const LSM_SETID_RES: i32 = 2;
pub const LSM_SETID_FS: i32 = 4;

pub const LSM_UNSAFE_PTRACE: u32 = 1;
pub const LSM_UNSAFE_NO_NEW_PRIVS: u32 = 4;

pub const PTRACE_MODE_FSCREDS: u32 = 8;

pub const PER_CLEAR_ON_SETID: u32 = 0x0040000;
pub const PF_SUPERPRIV: u32 = 0x00000100;

pub const ARRAY_SIZE: usize = 17;

const KERN_INFO: &str = "KERN_INFO";
const KERN_NOTICE: &str = "KERN_NOTICE";

// External kernel functions
extern "C" {
    pub fn cap_capable_helper(
        cred: *const cred,
        target_ns: *mut user_namespace,
        cred_ns: *const user_namespace,
        cap: i32,
    ) -> i32;

    pub fn cap_raised(cap: kernel_cap_t, cap_bit: i32) -> bool;

    pub fn cap_issubset(a: kernel_cap_t, b: kernel_cap_t) -> bool;

    pub fn cap_intersect(a: kernel_cap_t, b: kernel_cap_t) -> kernel_cap_t;

    pub fn cap_combine(a: kernel_cap_t, b: kernel_cap_t) -> kernel_cap_t;

    pub fn cap_drop_fs_set(cap: kernel_cap_t) -> kernel_cap_t;

    pub fn cap_raise_fs_set(cap: kernel_cap_t, permitted: kernel_cap_t) -> kernel_cap_t;

    pub fn cap_clear(cap: *mut kernel_cap_t);

    pub fn cap_lower(cap: *mut kernel_cap_t, cap_bit: i32);

    pub fn cap_raise(cap: *mut kernel_cap_t, cap_bit: i32);

    pub fn cap_valid(cap: i32) -> bool;

    pub fn cap_ambient_invariant_ok(cred: *const cred) -> bool;

    pub fn current_cred() -> *const cred;

    pub fn current_user_ns() -> *mut user_namespace;

    pub fn __task_cred(task: *const task_struct) -> *const cred;

    pub fn d_backing_inode(dentry: *const dentry) -> *mut inode;

    pub fn d_find_any_alias(inode: *const inode) -> *mut dentry;

    pub fn dput(dentry: *mut dentry);

    pub fn rcu_read_lock();

    pub fn rcu_read_unlock();

    pub fn ns_capable(ns: *mut user_namespace, cap: i32) -> bool;

    pub fn capable(cap: i32) -> bool;

    pub fn has_ns_capability(task: *const task_struct, ns: *mut user_namespace, cap: i32) -> bool;

    pub fn uid_eq(a: kuid_t, b: kuid_t) -> bool;

    pub fn gid_eq(a: kgid_t, b: kgid_t) -> bool;

    pub fn in_group_p(gid: kgid_t) -> bool;

    pub fn uid_valid(uid: kuid_t) -> bool;

    pub fn vfsuid_valid(vfsuid: vfsuid_t) -> bool;

    pub fn vfsuid_into_kuid(vfsuid: vfsuid_t) -> kuid_t;

    pub fn make_kuid(ns: *mut user_namespace, uid: u32) -> kuid_t;

    pub fn make_vfsuid(
        idmap: *mut mnt_idmap,
        fs_ns: *mut user_namespace,
        kuid: kuid_t,
    ) -> vfsuid_t;

    pub fn from_kuid(ns: *mut user_namespace, kuid: kuid_t) -> u32;

    pub fn from_vfsuid(idmap: *mut mnt_idmap, fs_ns: *mut user_namespace, vfsuid: vfsuid_t)
        -> kuid_t;

    pub fn VFSUIDT_INIT(kuid: kuid_t) -> vfsuid_t;

    pub fn __vfs_getxattr(
        dentry: *const dentry,
        inode: *mut inode,
        name: *const u8,
        value: *mut u8,
        size: usize,
    ) -> i32;

    pub fn __vfs_removexattr(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        name: *const u8,
    ) -> i32;

    pub fn vfs_getxattr_alloc(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        name: *const u8,
        buf: *mut *mut u8,
        buf_size: usize,
        gfp: u32,
    ) -> i32;

    pub fn le32_to_cpu(value: u32) -> u32;

    pub fn cpu_to_le32(value: u32) -> u32;

    pub fn kzalloc(size: usize, flags: u32) -> *mut u8;

    pub fn kmalloc(size: usize, flags: u32) -> *mut u8;

    pub fn kfree(ptr: *mut u8);

    pub fn memcpy(dst: *mut u8, src: *const u8, size: usize);

    pub fn printk(format: *const u8, ...);

    pub fn issecure(secure_bit: u32) -> bool;

    pub fn issecure_mask(secure_bit: u32) -> u32;

    pub fn trace_cap_capable(
        cred: *const cred,
        target_ns: *mut user_namespace,
        cred_ns: *const user_namespace,
        cap: i32,
        ret: i32,
    );

    pub fn ptracer_capable(task: *const task_struct, ns: *mut user_namespace) -> bool;

    pub fn file_mnt_idmap(file: *const file) -> *mut mnt_idmap;

    pub fn current_in_userns(ns: *mut user_namespace) -> bool;

    pub fn mnt_may_suid(mnt: *mut vfsmount) -> bool;

    pub fn prepare_creds() -> *mut cred;

    pub fn commit_creds(new: *mut cred) -> i32;

    pub fn audit_log_bprm_fcaps(
        bprm: *const linux_binprm,
        new: *const cred,
        old: *const cred,
    ) -> i32;

    pub fn capable_wrt_inode_uidgid(
        idmap: *mut mnt_idmap,
        inode: *mut inode,
        cap: i32,
    ) -> bool;

    pub fn get_vfs_caps_from_disk(
        idmap: *mut mnt_idmap,
        dentry: *const dentry,
        cpu_caps: *mut cpu_vfs_cap_data,
    ) -> i32;

    pub static mut init_user_ns: user_namespace;
    pub static mut nop_mnt_idmap: mnt_idmap;
    pub static mut dac_mmap_min_addr: usize;
    pub static file_caps_enabled: bool;

    pub fn current() -> *mut task_struct;
}

const XATTR_NAME_CAPS: &[u8] = b"security.capability";
const XATTR_SECURITY_PREFIX: &[u8] = b"security.";
const XATTR_SECURITY_PREFIX_LEN: usize = 9;

const GFP_NOFS: u32 = 0x00;
const GFP_ATOMIC: u32 = 0x01;

static mut WARNED: i32 = 0;

unsafe fn warn_setuid_and_fcaps_mixed(fname: *const u8) {
    if WARNED == 0 {
        let msg = b"warning: `%s' has both setuid-root and effective capabilities. Therefore not raising all capabilities.\n";
        printk(msg.as_ptr(), fname);
        WARNED = 1;
    }
}

#[inline]
unsafe fn cap_capable_helper_inline(
    cred: *const cred,
    target_ns: *mut user_namespace,
    cred_ns: *const user_namespace,
    cap: i32,
) -> i32 {
    let mut ns = target_ns;
    loop {
        if ns == cred_ns as *mut user_namespace {
            return if cap_raised((*cred).cap_effective, cap) {
                0
            } else {
                EPERM
            };
        }

        if (*ns).level <= (*cred_ns).level {
            return EPERM;
        }

        if (*ns).parent == cred_ns as *mut user_namespace
            && uid_eq((*ns).owner, (*cred).euid)
        {
            return 0;
        }

        ns = (*ns).parent;
    }
}

pub unsafe extern "C" fn cap_capable(
    cred: *const cred,
    target_ns: *mut user_namespace,
    cap: i32,
    _opts: u32,
) -> i32 {
    let cred_ns = (*cred).user_ns;
    let ret = cap_capable_helper_inline(cred, target_ns, cred_ns, cap);
    trace_cap_capable(cred, target_ns, cred_ns, cap, ret);
    ret
}

pub unsafe extern "C" fn cap_settime(_ts: *const timespec64, _tz: *const timezone) -> i32 {
    if !capable(CAP_SYS_TIME) {
        return EPERM;
    }
    0
}

pub unsafe extern "C" fn cap_ptrace_access_check(child: *mut task_struct, mode: u32) -> i32 {
    let mut ret = 0;
    let cred: *const cred;
    let child_cred: *const cred;
    let caller_caps: *const kernel_cap_t;

    rcu_read_lock();
    cred = current_cred();
    child_cred = __task_cred(child);
    if (mode & PTRACE_MODE_FSCREDS) != 0 {
        caller_caps = &(*cred).cap_effective;
    } else {
        caller_caps = &(*cred).cap_permitted;
    }
    if (*cred).user_ns == (*child_cred).user_ns
        && cap_issubset((*child_cred).cap_permitted, *caller_caps)
    {
        // out:
        rcu_read_unlock();
        return ret;
    }
    if ns_capable((*child_cred).user_ns, CAP_SYS_PTRACE) {
        // out:
        rcu_read_unlock();
        return ret;
    }
    ret = EPERM;
    rcu_read_unlock();
    ret
}

pub unsafe extern "C" fn cap_ptrace_traceme(parent: *mut task_struct) -> i32 {
    let mut ret = 0;
    let cred: *const cred;
    let child_cred: *const cred;

    rcu_read_lock();
    cred = __task_cred(parent);
    child_cred = current_cred();
    if (*cred).user_ns == (*child_cred).user_ns
        && cap_issubset((*child_cred).cap_permitted, (*cred).cap_permitted)
    {
        // out:
        rcu_read_unlock();
        return ret;
    }
    if has_ns_capability(parent, (*child_cred).user_ns, CAP_SYS_PTRACE) {
        // out:
        rcu_read_unlock();
        return ret;
    }
    ret = EPERM;
    rcu_read_unlock();
    ret
}

pub unsafe extern "C" fn cap_capget(
    target: *const task_struct,
    effective: *mut kernel_cap_t,
    inheritable: *mut kernel_cap_t,
    permitted: *mut kernel_cap_t,
) -> i32 {
    let cred: *const cred;

    rcu_read_lock();
    cred = __task_cred(target);
    *effective = (*cred).cap_effective;
    *inheritable = (*cred).cap_inheritable;
    *permitted = (*cred).cap_permitted;
    rcu_read_unlock();
    0
}

#[inline]
unsafe fn cap_inh_is_capped() -> i32 {
    if cap_capable(current_cred(), (*current_cred()).user_ns, CAP_SETPCAP, CAP_OPT_NONE) == 0 {
        return 0;
    }
    1
}

pub unsafe extern "C" fn cap_capset(
    new: *mut cred,
    old: *const cred,
    effective: *const kernel_cap_t,
    inheritable: *const kernel_cap_t,
    permitted: *const kernel_cap_t,
) -> i32 {
    if cap_inh_is_capped() != 0
        && !cap_issubset(
            *inheritable,
            cap_combine((*old).cap_inheritable, (*old).cap_permitted),
        )
    {
        return EPERM;
    }

    if !cap_issubset(
        *inheritable,
        cap_combine((*old).cap_inheritable, (*old).cap_bset),
    ) {
        return EPERM;
    }

    if !cap_issubset(*permitted, (*old).cap_permitted) {
        return EPERM;
    }

    if !cap_issubset(*effective, *permitted) {
        return EPERM;
    }

    (*new).cap_effective = *effective;
    (*new).cap_inheritable = *inheritable;
    (*new).cap_permitted = *permitted;

    (*new).cap_ambient = cap_intersect((*new).cap_ambient, cap_intersect(*permitted, *inheritable));
    if !cap_ambient_invariant_ok(new) {
        return EINVAL;
    }
    0
}

pub unsafe extern "C" fn cap_inode_need_killpriv(dentry: *mut dentry) -> i32 {
    let inode = d_backing_inode(dentry);
    let error = __vfs_getxattr(dentry, inode, XATTR_NAME_CAPS.as_ptr(), ptr::null_mut(), 0);
    if error > 0 {
        1
    } else {
        0
    }
}

pub unsafe extern "C" fn cap_inode_killpriv(idmap: *mut mnt_idmap, dentry: *mut dentry) -> i32 {
    let mut error = __vfs_removexattr(idmap, dentry, XATTR_NAME_CAPS.as_ptr());
    if error == EOPNOTSUPP {
        error = 0;
    }
    error
}

#[inline]
unsafe fn kuid_root_in_ns(kuid: kuid_t, ns: *mut user_namespace) -> bool {
    let mut ns = ns;
    loop {
        if from_kuid(ns, kuid) == 0 {
            return true;
        }
        if ns == &mut init_user_ns as *mut user_namespace {
            break;
        }
        ns = (*ns).parent;
    }
    false
}

#[inline]
unsafe fn vfsuid_root_in_currentns(vfsuid: vfsuid_t) -> bool {
    if !vfsuid_valid(vfsuid) {
        return false;
    }
    let kuid = vfsuid_into_kuid(vfsuid);
    kuid_root_in_ns(kuid, current_user_ns())
}

#[inline]
fn sansflags(m: u32) -> u32 {
    m & !VFS_CAP_FLAGS_EFFECTIVE
}

#[inline]
unsafe fn is_v2header(size: usize, cap: *const vfs_cap_data) -> bool {
    if size != XATTR_CAPS_SZ_2 {
        return false;
    }
    sansflags(le32_to_cpu((*cap).magic_etc)) == VFS_CAP_REVISION_2
}

#[inline]
unsafe fn is_v3header(size: usize, cap: *const vfs_cap_data) -> bool {
    if size != XATTR_CAPS_SZ_3 {
        return false;
    }
    sansflags(le32_to_cpu((*cap).magic_etc)) == VFS_CAP_REVISION_3
}

pub unsafe extern "C" fn cap_inode_getsecurity(
    idmap: *mut mnt_idmap,
    inode: *mut inode,
    name: *const u8,
    buffer: *mut *mut u8,
    alloc: bool,
) -> i32 {
    let mut size: i32 = 0;
    let mut kroot: kuid_t;
    let mut vfsroot: vfsuid_t;
    let mut nsmagic: u32;
    let mut magic: u32;
    let mut root: u32;
    let mut mappedroot: u32;
    let mut tmpbuf: *mut u8 = ptr::null_mut();
    let mut cap: *mut vfs_cap_data;
    let mut nscap: *mut vfs_ns_cap_data = ptr::null_mut();
    let mut dentry: *mut dentry;
    let mut fs_ns: *mut user_namespace;

    if !CStr::from_ptr(name as *const i8)
        .to_bytes()
        .eq(b"capability")
    {
        return EOPNOTSUPP;
    }

    dentry = d_find_any_alias(inode);
    if dentry.is_null() {
        return EINVAL;
    }
    size = vfs_getxattr_alloc(
        idmap,
        dentry,
        XATTR_NAME_CAPS.as_ptr(),
        &mut tmpbuf,
        mem::size_of::<vfs_ns_cap_data>(),
        GFP_NOFS,
    );
    dput(dentry);
    if size < 0 || tmpbuf.is_null() {
        goto_out_free(&mut tmpbuf);
        return size;
    }

    fs_ns = (*(*inode).i_sb).s_user_ns;
    cap = tmpbuf as *mut vfs_cap_data;
    if is_v2header(size as usize, cap) {
        root = 0;
    } else if is_v3header(size as usize, cap) {
        nscap = tmpbuf as *mut vfs_ns_cap_data;
        root = le32_to_cpu((*nscap).rootid);
    } else {
        size = EINVAL;
        goto_out_free(&mut tmpbuf);
        return size;
    }

    kroot = make_kuid(fs_ns, root);
    vfsroot = make_vfsuid(idmap, fs_ns, kroot);
    mappedroot = from_kuid(current_user_ns(), vfsuid_into_kuid(vfsroot));

    if mappedroot != u32::MAX && mappedroot != 0 {
        size = mem::size_of::<vfs_ns_cap_data>() as i32;
        if alloc {
            if nscap.is_null() {
                nscap = kzalloc(size as usize, GFP_ATOMIC) as *mut vfs_ns_cap_data;
                if nscap.is_null() {
                    size = ENOMEM;
                    goto_out_free(&mut tmpbuf);
                    return size;
                }
                nsmagic = VFS_CAP_REVISION_3;
                magic = le32_to_cpu((*cap).magic_etc);
                if (magic & VFS_CAP_FLAGS_EFFECTIVE) != 0 {
                    nsmagic |= VFS_CAP_FLAGS_EFFECTIVE;
                }
                memcpy(
                    &mut (*nscap).data as *mut _ as *mut u8,
                    &(*cap).data as *const _ as *const u8,
                    mem::size_of::<u32>() * 2 * VFS_CAP_U32,
                );
                (*nscap).magic_etc = cpu_to_le32(nsmagic);
            } else {
                tmpbuf = ptr::null_mut();
            }
            (*nscap).rootid = cpu_to_le32(mappedroot);
            *buffer = nscap as *mut u8;
        }
        goto_out_free(&mut tmpbuf);
        return size;
    }

    if !vfsuid_root_in_currentns(vfsroot) {
        size = EOVERFLOW;
        goto_out_free(&mut tmpbuf);
        return size;
    }

    size = mem::size_of::<vfs_cap_data>() as i32;
    if alloc {
        if !nscap.is_null() {
            cap = kzalloc(size as usize, GFP_ATOMIC) as *mut vfs_cap_data;
            if cap.is_null() {
                size = ENOMEM;
                goto_out_free(&mut tmpbuf);
                return size;
            }
            magic = VFS_CAP_REVISION_2;
            nsmagic = le32_to_cpu((*nscap).magic_etc);
            if (nsmagic & VFS_CAP_FLAGS_EFFECTIVE) != 0 {
                magic |= VFS_CAP_FLAGS_EFFECTIVE;
            }
            memcpy(
                &mut (*cap).data as *mut _ as *mut u8,
                &(*nscap).data as *const _ as *const u8,
                mem::size_of::<u32>() * 2 * VFS_CAP_U32,
            );
            (*cap).magic_etc = cpu_to_le32(magic);
        } else {
            tmpbuf = ptr::null_mut();
        }
        *buffer = cap as *mut u8;
    }
    goto_out_free(&mut tmpbuf);
    size
}

#[inline]
fn goto_out_free(tmpbuf: *mut *mut u8) {
    unsafe {
        kfree(*tmpbuf);
    }
}

#[inline]
unsafe fn rootid_from_xattr(value: *const u8, size: usize, task_ns: *mut user_namespace) -> vfsuid_t {
    let nscap = value as *const vfs_ns_cap_data;
    let mut rootid: u32 = 0;

    if size == XATTR_CAPS_SZ_3 {
        rootid = le32_to_cpu((*nscap).rootid);
    }

    VFSUIDT_INIT(make_kuid(task_ns, rootid))
}

#[inline]
unsafe fn validheader(size: usize, cap: *const vfs_cap_data) -> bool {
    is_v2header(size, cap) || is_v3header(size, cap)
}

pub unsafe extern "C" fn cap_convert_nscap(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    ivalue: *mut *const u8,
    size: usize,
) -> i32 {
    let mut nscap: *mut vfs_ns_cap_data;
    let mut nsrootid: u32;
    let cap = *ivalue as *const vfs_cap_data;
    let mut magic: u32;
    let mut nsmagic: u32;
    let inode = d_backing_inode(dentry);
    let task_ns = current_user_ns();
    let fs_ns = (*(*inode).i_sb).s_user_ns;
    let mut rootid: kuid_t;
    let mut vfsrootid: vfsuid_t;
    let newsize: usize;

    if (*ivalue).is_null() {
        return EINVAL;
    }
    if !validheader(size, cap) {
        return EINVAL;
    }
    if !capable_wrt_inode_uidgid(idmap, inode, CAP_SETFCAP) {
        return EPERM;
    }
    if size == XATTR_CAPS_SZ_2 && idmap == &nop_mnt_idmap {
        if ns_capable((*inode).i_sb.(*).s_user_ns, CAP_SETFCAP) {
            return size as i32;
        }
    }

    vfsrootid = rootid_from_xattr(*ivalue, size, task_ns);
    if !vfsuid_valid(vfsrootid) {
        return EINVAL;
    }

    rootid = from_vfsuid(idmap, fs_ns, vfsrootid);
    if !uid_valid(rootid) {
        return EINVAL;
    }

    nsrootid = from_kuid(fs_ns, rootid);
    if nsrootid == u32::MAX {
        return EINVAL;
    }

    newsize = mem::size_of::<vfs_ns_cap_data>();
    nscap = kmalloc(newsize, GFP_ATOMIC) as *mut vfs_ns_cap_data;
    if nscap.is_null() {
        return ENOMEM;
    }
    (*nscap).rootid = cpu_to_le32(nsrootid);
    nsmagic = VFS_CAP_REVISION_3;
    magic = le32_to_cpu((*cap).magic_etc);
    if (magic & VFS_CAP_FLAGS_EFFECTIVE) != 0 {
        nsmagic |= VFS_CAP_FLAGS_EFFECTIVE;
    }
    (*nscap).magic_etc = cpu_to_le32(nsmagic);
    memcpy(
        &mut (*nscap).data as *mut _ as *mut u8,
        &(*cap).data as *const _ as *const u8,
        mem::size_of::<u32>() * 2 * VFS_CAP_U32,
    );

    *ivalue = nscap as *const u8;
    newsize as i32
}

#[inline]
unsafe fn bprm_caps_from_vfs_caps(
    caps: *const cpu_vfs_cap_data,
    bprm: *mut linux_binprm,
    effective: *mut bool,
    has_fcap: *mut bool,
) -> i32 {
    let new = (*bprm).cred;
    let mut ret = 0;

    if ((*caps).magic_etc & VFS_CAP_FLAGS_EFFECTIVE) != 0 {
        *effective = true;
    }

    if ((*caps).magic_etc & VFS_CAP_REVISION_MASK) != 0 {
        *has_fcap = true;
    }

    (*new).cap_permitted.val =
        ((*new).cap_bset.val & (*caps).permitted.val)
            | ((*new).cap_inheritable.val & (*caps).inheritable.val);

    if ((*caps).permitted.val & !(*new).cap_permitted.val) != 0 {
        ret = EPERM;
    }

    if *effective {
        ret
    } else {
        0
    }
}

pub unsafe extern "C" fn get_vfs_caps_from_disk(
    idmap: *mut mnt_idmap,
    dentry: *const dentry,
    cpu_caps: *mut cpu_vfs_cap_data,
) -> i32 {
    let inode = d_backing_inode(dentry);
    let mut magic_etc: u32;
    let mut size: i32;
    let mut data: vfs_ns_cap_data = mem::zeroed();
    let mut nscaps = &mut data as *mut vfs_ns_cap_data;
    let caps = &mut data as *mut _ as *mut vfs_cap_data;
    let mut rootkuid: kuid_t;
    let mut rootvfsuid: vfsuid_t;
    let fs_ns: *mut user_namespace;

    memset(cpu_caps as *mut u8, 0, mem::size_of::<cpu_vfs_cap_data>());

    if inode.is_null() {
        return ENODATA;
    }

    fs_ns = (*(*inode).i_sb).s_user_ns;
    size = __vfs_getxattr(
        dentry,
        inode,
        XATTR_NAME_CAPS.as_ptr(),
        &mut data as *mut _ as *mut u8,
        XATTR_CAPS_SZ,
    );
    if size == ENODATA || size == EOPNOTSUPP {
        return ENODATA;
    }

    if size < 0 {
        return size;
    }

    if size < (mem::size_of::<u32>() as i32) {
        return EINVAL;
    }

    (*cpu_caps).magic_etc = magic_etc = le32_to_cpu((*caps).magic_etc);

    rootkuid = make_kuid(fs_ns, 0);
    match magic_etc & VFS_CAP_REVISION_MASK {
        VFS_CAP_REVISION_1 => {
            if size as usize != XATTR_CAPS_SZ_1 {
                return EINVAL;
            }
        }
        VFS_CAP_REVISION_2 => {
            if size as usize != XATTR_CAPS_SZ_2 {
                return EINVAL;
            }
        }
        VFS_CAP_REVISION_3 => {
            if size as usize != XATTR_CAPS_SZ_3 {
                return EINVAL;
            }
            rootkuid = make_kuid(fs_ns, le32_to_cpu((*nscaps).rootid));
        }
        _ => {
            return EINVAL;
        }
    }

    rootvfsuid = make_vfsuid(idmap, fs_ns, rootkuid);
    if !vfsuid_valid(rootvfsuid) {
        return ENODATA;
    }

    if !vfsuid_root_in_currentns(rootvfsuid) {
        return ENODATA;
    }

    (*cpu_caps).permitted.val = le32_to_cpu((*caps).data[0].0 as u32) as u32;
    (*cpu_caps).inheritable.val = le32_to_cpu((*caps).data[0].1 as u32) as u32;

    if (magic_etc & VFS_CAP_REVISION_MASK) != VFS_CAP_REVISION_1 {
        (*cpu_caps).permitted.val +=
            (le32_to_cpu((*caps).data[1].0 as u32) as u64) << 32;
        (*cpu_caps).inheritable.val +=
            (le32_to_cpu((*caps).data[1].1 as u32) as u64) << 32;
    }

    (*cpu_caps).permitted.val &= CAP_VALID_MASK;
    (*cpu_caps).inheritable.val &= CAP_VALID_MASK;

    (*cpu_caps).rootid = vfsuid_into_kuid(rootvfsuid);

    0
}

#[inline]
unsafe fn get_file_caps(
    bprm: *mut linux_binprm,
    file: *const file,
    effective: *mut bool,
    has_fcap: *mut bool,
) -> i32 {
    let mut rc = 0;
    let mut vcaps: cpu_vfs_cap_data = mem::zeroed();

    cap_clear(&mut (*(*bprm).cred).cap_permitted);

    if !file_caps_enabled {
        return 0;
    }

    if !mnt_may_suid((*file).f_path.mnt) {
        return 0;
    }

    if !current_in_userns((*(*(*file).f_path.mnt).mnt_sb).s_user_ns) {
        return 0;
    }

    rc = get_vfs_caps_from_disk(
        file_mnt_idmap(file),
        (*file).f_path.dentry,
        &mut vcaps,
    );
    if rc < 0 {
        if rc == EINVAL {
            let msg = b"Invalid argument reading file caps for %s\n";
            printk(msg.as_ptr(), (*bprm).filename);
        } else if rc == ENODATA {
            rc = 0;
        }
        return rc;
    }

    rc = bprm_caps_from_vfs_caps(&vcaps, bprm, effective, has_fcap);

    if rc != 0 {
        cap_clear(&mut (*(*bprm).cred).cap_permitted);
    }

    rc
}

#[inline]
fn root_privileged() -> bool {
    unsafe { !issecure(SECURE_NOROOT) }
}

#[inline]
unsafe fn __is_real(uid: kuid_t, cred: *const cred) -> bool {
    uid_eq((*cred).uid, uid)
}

#[inline]
unsafe fn __is_eff(uid: kuid_t, cred: *const cred) -> bool {
    uid_eq((*cred).euid, uid)
}

#[inline]
unsafe fn __is_suid(uid: kuid_t, cred: *const cred) -> bool {
    !__is_real(uid, cred) && __is_eff(uid, cred)
}

#[inline]
unsafe fn handle_privileged_root(
    bprm: *mut linux_binprm,
    has_fcap: bool,
    effective: *mut bool,
    root_uid: kuid_t,
) {
    let old = current_cred();
    let new = (*bprm).cred;

    if !root_privileged() {
        return;
    }
    if has_fcap && __is_suid(root_uid, new) {
        warn_setuid_and_fcaps_mixed((*bprm).filename);
        return;
    }
    if __is_eff(root_uid, new) || __is_real(root_uid, new) {
        (*new).cap_permitted =
            cap_combine((*old).cap_bset, (*old).cap_inheritable);
    }
    if __is_eff(root_uid, new) {
        *effective = true;
    }
}

macro_rules! cap_gained {
    ($field:ident, $target:expr, $source:expr) => {
        !cap_issubset(
            (*$target).cap_permitted,
            (*$source).cap_permitted,
        )
    };
}

macro_rules! cap_grew {
    ($target:ident, $source:ident, $cred:expr) => {
        !cap_issubset(
            (*$cred).cap_permitted,
            (*$cred).cap_permitted,
        )
    };
}

macro_rules! cap_full {
    ($field:ident, $cred:expr) => {
        cap_issubset(CAP_FULL_SET, (*$cred).cap_permitted)
    };
}

#[inline]
unsafe fn nonroot_raised_pE(
    new: *mut cred,
    old: *const cred,
    root: kuid_t,
    has_fcap: bool,
) -> bool {
    let mut ret = false;

    if ((__cap_grew_effective_ambient(new) &&
        !(cap_full!(permitted, new) && (__is_eff(root, new) || __is_real(root, new)) && root_privileged())) ||
        (root_privileged() && __is_suid(root, new) && !cap_full!(permitted, new)) ||
        (uid_eq((*new).euid, (*old).euid)
            && ((has_fcap && __cap_gained_permitted(new, old))
                || __cap_gained_ambient(new, old))))
    {
        ret = true;
    }

    ret
}

#[inline]
unsafe fn __cap_grew_effective_ambient(new: *const cred) -> bool {
    !cap_issubset((*new).cap_effective, (*new).cap_ambient)
}

#[inline]
unsafe fn __cap_gained_permitted(new: *const cred, old: *const cred) -> bool {
    !cap_issubset((*new).cap_permitted, (*old).cap_permitted)
}

#[inline]
unsafe fn __cap_gained_ambient(new: *const cred, old: *const cred) -> bool {
    !cap_issubset((*new).cap_ambient, (*old).cap_ambient)
}

pub unsafe extern "C" fn cap_bprm_creds_from_file(
    bprm: *mut linux_binprm,
    file: *const file,
) -> i32 {
    let old = current_cred();
    let new = (*bprm).cred;
    let mut effective = false;
    let mut has_fcap = false;
    let mut id_changed: bool;
    let mut ret: i32;
    let root_uid: kuid_t;

    if !cap_ambient_invariant_ok(old) {
        return EPERM;
    }

    ret = get_file_caps(bprm, file, &mut effective, &mut has_fcap);
    if ret < 0 {
        return ret;
    }

    root_uid = make_kuid((*new).user_ns, 0);

    handle_privileged_root(bprm, has_fcap, &mut effective, root_uid);

    if cap_gained!(permitted, new, old) {
        (*bprm).per_clear |= PER_CLEAR_ON_SETID;
    }

    id_changed = !uid_eq((*new).euid, (*old).euid) || !in_group_p((*new).egid);

    if (id_changed || cap_gained!(permitted, new, old))
        && (((*bprm).unsafe_ & !LSM_UNSAFE_PTRACE) != 0
            || !ptracer_capable(current(), (*new).user_ns))
    {
        if !ns_capable((*new).user_ns, CAP_SETUID)
            || (((*bprm).unsafe_ & LSM_UNSAFE_NO_NEW_PRIVS) != 0)
        {
            (*new).euid = (*new).uid;
            (*new).egid = (*new).gid;
        }
        (*new).cap_permitted = cap_intersect((*new).cap_permitted, (*old).cap_permitted);
    }

    (*new).suid = (*new).fsuid;
    (*new).fsuid = (*new).euid;
    (*new).sgid = (*new).fsgid;
    (*new).fsgid = (*new).egid;

    if has_fcap || id_changed {
        cap_clear(&mut (*new).cap_ambient);
    }

    (*new).cap_permitted = cap_combine((*new).cap_permitted, (*new).cap_ambient);

    if effective {
        (*new).cap_effective = (*new).cap_permitted;
    } else {
        (*new).cap_effective = (*new).cap_ambient;
    }

    if !cap_ambient_invariant_ok(new) {
        return EPERM;
    }

    if nonroot_raised_pE(new, old, root_uid, has_fcap) {
        ret = audit_log_bprm_fcaps(bprm, new, old);
        if ret < 0 {
            return ret;
        }
    }

    (*new).securebits &= !issecure_mask(SECURE_KEEP_CAPS);

    if !cap_ambient_invariant_ok(new) {
        return EPERM;
    }

    if id_changed || !uid_eq((*new).euid, (*old).uid) || !gid_eq((*new).egid, (*old).gid)
        || (!__is_real(root_uid, new) && (effective || __cap_grew_effective_ambient(new)))
    {
        (*bprm).secureexec = 1;
    }

    0
}

pub unsafe extern "C" fn cap_inode_setxattr(
    dentry: *const dentry,
    name: *const u8,
    _value: *const u8,
    _size: usize,
    _flags: i32,
) -> i32 {
    let user_ns = (*(*dentry).d_sb).s_user_ns;

    if strncmp(name, XATTR_SECURITY_PREFIX.as_ptr(), XATTR_SECURITY_PREFIX_LEN) != 0 {
        return 0;
    }

    if strcmp(name, XATTR_NAME_CAPS.as_ptr()) == 0 {
        return 0;
    }

    if !ns_capable(user_ns, CAP_SYS_ADMIN) {
        return EPERM;
    }
    0
}

pub unsafe extern "C" fn cap_inode_removexattr(
    idmap: *mut mnt_idmap,
    dentry: *mut dentry,
    name: *const u8,
) -> i32 {
    let user_ns = (*(*dentry).d_sb).s_user_ns;

    if strncmp(name, XATTR_SECURITY_PREFIX.as_ptr(), XATTR_SECURITY_PREFIX_LEN) != 0 {
        return 0;
    }

    if strcmp(name, XATTR_NAME_CAPS.as_ptr()) == 0 {
        let inode = d_backing_inode(dentry);
        if inode.is_null() {
            return EINVAL;
        }
        if !capable_wrt_inode_uidgid(idmap, inode, CAP_SETFCAP) {
            return EPERM;
        }
        return 0;
    }

    if !ns_capable(user_ns, CAP_SYS_ADMIN) {
        return EPERM;
    }
    0
}

#[inline]
unsafe fn cap_emulate_setxuid(new: *mut cred, old: *const cred) {
    let root_uid = make_kuid((*old).user_ns, 0);

    if (uid_eq((*old).uid, root_uid)
        || uid_eq((*old).euid, root_uid)
        || uid_eq((*old).suid, root_uid))
        && (!uid_eq((*new).uid, root_uid)
            && !uid_eq((*new).euid, root_uid)
            && !uid_eq((*new).suid, root_uid))
    {
        if !issecure(SECURE_KEEP_CAPS) {
            cap_clear(&mut (*new).cap_permitted);
            cap_clear(&mut (*new).cap_effective);
        }

        cap_clear(&mut (*new).cap_ambient);
    }
    if uid_eq((*old).euid, root_uid) && !uid_eq((*new).euid, root_uid) {
        cap_clear(&mut (*new).cap_effective);
    }
    if !uid_eq((*old).euid, root_uid) && uid_eq((*new).euid, root_uid) {
        (*new).cap_effective = (*new).cap_permitted;
    }
}

pub unsafe extern "C" fn cap_task_fix_setuid(
    new: *mut cred,
    old: *const cred,
    flags: i32,
) -> i32 {
    match flags {
        LSM_SETID_RE | LSM_SETID_ID | LSM_SETID_RES => {
            if !issecure(SECURE_NO_SETUID_FIXUP) {
                cap_emulate_setxuid(new, old);
            }
        }

        LSM_SETID_FS => {
            if !issecure(SECURE_NO_SETUID_FIXUP) {
                let root_uid = make_kuid((*old).user_ns, 0);
                if uid_eq((*old).fsuid, root_uid) && !uid_eq((*new).fsuid, root_uid) {
                    (*new).cap_effective = cap_drop_fs_set((*new).cap_effective);
                }

                if !uid_eq((*old).fsuid, root_uid) && uid_eq((*new).fsuid, root_uid) {
                    (*new).cap_effective = cap_raise_fs_set((*new).cap_effective, (*new).cap_permitted);
                }
            }
        }

        _ => {
            return EINVAL;
        }
    }

    0
}

#[inline]
unsafe fn cap_safe_nice(p: *const task_struct) -> i32 {
    let mut ret = 0;

    rcu_read_lock();
    let is_subset = cap_issubset(
        (*__task_cred(p)).cap_permitted,
        (*current_cred()).cap_permitted,
    );
    if !is_subset && !ns_capable((*__task_cred(p)).user_ns, CAP_SYS_NICE) {
        ret = EPERM;
    }
    rcu_read_unlock();

    ret
}

pub unsafe extern "C" fn cap_task_setscheduler(p: *mut task_struct) -> i32 {
    cap_safe_nice(p)
}

pub unsafe extern "C" fn cap_task_setioprio(p: *mut task_struct, _ioprio: i32) -> i32 {
    cap_safe_nice(p)
}

pub unsafe extern "C" fn cap_task_setnice(p: *mut task_struct, _nice: i32) -> i32 {
    cap_safe_nice(p)
}

#[inline]
unsafe fn cap_prctl_drop(cap: usize) -> i32 {
    let new: *mut cred;

    if !ns_capable(current_user_ns(), CAP_SETPCAP) {
        return EPERM;
    }
    if !cap_valid(cap as i32) {
        return EINVAL;
    }

    new = prepare_creds();
    if new.is_null() {
        return ENOMEM;
    }
    cap_lower(&mut (*new).cap_bset, cap as i32);
    commit_creds(new)
}

pub unsafe extern "C" fn cap_task_prctl(
    option: i32,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> i32 {
    let old = current_cred();
    let mut new: *mut cred;

    match option {
        PR_CAPBSET_READ => {
            if !cap_valid(arg2 as i32) {
                return EINVAL;
            }
            return if cap_raised((*old).cap_bset, arg2 as i32) {
                1
            } else {
                0
            };
        }

        PR_CAPBSET_DROP => {
            return cap_prctl_drop(arg2);
        }

        PR_SET_SECUREBITS => {
            if ((((*old).securebits & SECURE_ALL_LOCKS) >> 1) & ((*old).securebits ^ (arg2 as u32)))
                != 0
                || (((*old).securebits & SECURE_ALL_LOCKS & !(arg2 as u32)) != 0)
                || ((arg2 as u32) & !(SECURE_ALL_LOCKS | SECURE_ALL_BITS) != 0)
            {
                return EPERM;
            }

            if cap_capable(current_cred(), (*current_cred()).user_ns, CAP_SETPCAP, CAP_OPT_NONE)
                != 0
            {
                let unpriv_and_locks = SECURE_ALL_UNPRIVILEGED | (SECURE_ALL_UNPRIVILEGED << 1);
                let changed = (*old).securebits ^ (arg2 as u32);

                if changed == 0 {
                    return EPERM;
                }

                if (changed & !unpriv_and_locks) != 0 {
                    return EPERM;
                }
            }

            new = prepare_creds();
            if new.is_null() {
                return ENOMEM;
            }
            (*new).securebits = arg2 as u32;
            return commit_creds(new);
        }

        PR_GET_SECUREBITS => {
            return (*old).securebits as i32;
        }

        PR_GET_KEEPCAPS => {
            return if issecure(SECURE_KEEP_CAPS) { 1 } else { 0 };
        }

        PR_SET_KEEPCAPS => {
            if arg2 > 1 {
                return EINVAL;
            }
            if issecure(SECURE_KEEP_CAPS_LOCKED) {
                return EPERM;
            }

            new = prepare_creds();
            if new.is_null() {
                return ENOMEM;
            }
            if arg2 != 0 {
                (*new).securebits |= issecure_mask(SECURE_KEEP_CAPS);
            } else {
                (*new).securebits &= !issecure_mask(SECURE_KEEP_CAPS);
            }
            return commit_creds(new);
        }

        PR_CAP_AMBIENT => {
            if arg2 as i32 == PR_CAP_AMBIENT_CLEAR_ALL {
                if (arg3 | arg4 | arg5) != 0 {
                    return EINVAL;
                }

                new = prepare_creds();
                if new.is_null() {
                    return ENOMEM;
                }
                cap_clear(&mut (*new).cap_ambient);
                return commit_creds(new);
            }

            if (!cap_valid(arg3 as i32) as usize | arg4 | arg5) != 0 {
                return EINVAL;
            }

            if arg2 as i32 == PR_CAP_AMBIENT_IS_SET {
                return if cap_raised((*current_cred()).cap_ambient, arg3 as i32) {
                    1
                } else {
                    0
                };
            } else if arg2 as i32 != PR_CAP_AMBIENT_RAISE && arg2 as i32 != PR_CAP_AMBIENT_LOWER {
                return EINVAL;
            } else {
                if arg2 as i32 == PR_CAP_AMBIENT_RAISE
                    && (!cap_raised((*current_cred()).cap_permitted, arg3 as i32)
                        || !cap_raised((*current_cred()).cap_inheritable, arg3 as i32)
                        || issecure(SECURE_NO_CAP_AMBIENT_RAISE))
                {
                    return EPERM;
                }

                new = prepare_creds();
                if new.is_null() {
                    return ENOMEM;
                }
                if arg2 as i32 == PR_CAP_AMBIENT_RAISE {
                    cap_raise(&mut (*new).cap_ambient, arg3 as i32);
                } else {
                    cap_lower(&mut (*new).cap_ambient, arg3 as i32);
                }
                return commit_creds(new);
            }
        }

        _ => {
            return ENOSYS;
        }
    }
}

pub unsafe extern "C" fn cap_vm_enough_memory(_mm: *mut mm_struct, _pages: isize) -> i32 {
    cap_capable(
        current_cred(),
        &mut init_user_ns,
        CAP_SYS_ADMIN,
        CAP_OPT_NOAUDIT,
    )
}

pub unsafe extern "C" fn cap_mmap_addr(addr: usize) -> i32 {
    let mut ret = 0;

    if addr < dac_mmap_min_addr {
        ret = cap_capable(
            current_cred(),
            &mut init_user_ns,
            CAP_SYS_RAWIO,
            CAP_OPT_NONE,
        );
        if ret == 0 {
            (*current()).flags |= PF_SUPERPRIV;
        }
    }
    ret
}

#[repr(C)]
pub struct lsm_id {
    pub name: *const u8,
    pub id: u32,
}

#[repr(C)]
pub struct security_hook_list {
    pub hook: *const u8,
    pub head: *mut u8,
}

extern "C" {
    pub static mut capability_lsmid: lsm_id;
    pub static mut capability_hooks: [security_hook_list; ARRAY_SIZE];

    pub fn security_add_hooks(hooks: *const security_hook_list, count: usize, lsmid: *const lsm_id);

    pub fn memset(dst: *mut u8, value: i32, size: usize);
    pub fn strcmp(s1: *const u8, s2: *const u8) -> i32;
    pub fn strncmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
}

pub unsafe extern "C" fn capability_init() -> i32 {
    security_add_hooks(
        capability_hooks.as_ptr(),
        ARRAY_SIZE,
        &capability_lsmid,
    );
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
