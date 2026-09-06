// SPDX-License-Identifier: GPL-2.0-only
/* Updated: Karl MacMillan <kmacmillan@tresys.com>
 *
 *	Added conditional policy language extensions
 *
 *  Updated: Hewlett-Packard <paul@paul-moore.com>
 *
 *	Added support for the policy capability bitmap
 *
 * Copyright (C) 2007 Hewlett-Packard Development Company, L.P.
 * Copyright (C) 2003 - 2004 Tresys Technology, LLC
 * Copyright (C) 2004 Red Hat, Inc., James Morris <jmorris@redhat.com>
 */

/* selinuxfs pseudo filesystem for exporting the security policy API.
   Based on the proc code and the fs/nfsd/nfsctl.c code. */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type ino_t = c_ulong;
type umode_t = u16;
type u16_t = u16;
type u32_t = u32;
type vm_fault_t = c_uint;
type bool_t = bool;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const EPERM: c_int = 1;
const EFAULT: c_int = 14;
const EACCES: c_int = 13;
const ERANGE: c_int = 34;
const ENAMETOOLONG: c_int = 36;

const PAGE_SIZE: usize = 4096;
const PAGE_SHIFT: c_ulong = 12;
const VM_WRITE: c_ulong = 0x00000002;
const VM_MAYWRITE: c_ulong = 0x00000020;
const VM_SHARED: c_ulong = 0x00000008;
const VM_DONTEXPAND: c_ulong = 0x00040000;
const VM_DONTDUMP: c_ulong = 0x04000000;
const FAULT_FLAG_WRITE: c_uint = 0x01;
const FAULT_FLAG_MKWRITE: c_uint = 0x02;
const VM_FAULT_SIGBUS: vm_fault_t = 0x0002;
const GFP_KERNEL: c_uint = 0;
const AUDIT_MAC_STATUS: c_int = 1404;
const AUDIT_MAC_POLICY_LOAD: c_int = 1403;
const LSM_POLICY_CHANGE: c_int = 1;
const S_IFREG: umode_t = 0o100000;
const S_IFDIR: umode_t = 0o040000;
const S_IFCHR: umode_t = 0o020000;
const S_IRUSR: umode_t = 0o400;
const S_IWUSR: umode_t = 0o200;
const S_IRUGO: umode_t = 0o444;
const S_IWUGO: umode_t = 0o222;
const S_IXUGO: umode_t = 0o111;
const MEM_MAJOR: c_uint = 1;
const SELINUX_MAGIC: c_ulong = 0xf97cff8c;
const SIMPLE_TRANSACTION_LIMIT: usize = 4096;
const POLICYDB_VERSION_MAX: c_uint = 33;
const POLICYDB_CAP_MAX: c_uint = 8;
const SECINITSID_SECURITY: u32_t = 1;
const SECINITSID_DEVNULL: u32_t = 27;
const SECINITSID_NUM: c_uint = 27;
const SECCLASS_SECURITY: u16_t = 2;
const SECCLASS_FILE: u16_t = 6;
const SECCLASS_CHR_FILE: u16_t = 10;
const SECURITY__SETENFORCE: u32_t = 0x20;
const SECURITY__READ_POLICY: u32_t = 0x400;
const SECURITY__LOAD_POLICY: u32_t = 0x2;
const SECURITY__CHECK_CONTEXT: u32_t = 0x200;
const SECURITY__VALIDATE_TRANS: u32_t = 0x800;
const SECURITY__COMPUTE_AV: u32_t = 0x4;
const SECURITY__COMPUTE_CREATE: u32_t = 0x8;
const SECURITY__COMPUTE_RELABEL: u32_t = 0x10;
const SECURITY__COMPUTE_MEMBER: u32_t = 0x40;
const SECURITY__SETBOOL: u32_t = 0x80;
const SECURITY__SETSECPARAM: u32_t = 0x100;
const LABEL_INITIALIZED: c_uint = 1;
const SEL_VEC_MAX: c_ulong = 32;

#[repr(C)]
pub enum sel_inos {
    SEL_ROOT_INO = 2,
    SEL_LOAD,
    SEL_ENFORCE,
    SEL_CONTEXT,
    SEL_ACCESS,
    SEL_CREATE,
    SEL_RELABEL,
    SEL_USER,
    SEL_POLICYVERS,
    SEL_COMMIT_BOOLS,
    SEL_MLS,
    SEL_DISABLE,
    SEL_MEMBER,
    SEL_CHECKREQPROT,
    SEL_COMPAT_NET,
    SEL_REJECT_UNKNOWN,
    SEL_DENY_UNKNOWN,
    SEL_STATUS,
    SEL_POLICY,
    SEL_VALIDATE_TRANS,
    SEL_INO_NEXT,
}

const SEL_INITCON_INO_OFFSET: c_ulong = 0x01000000;
const SEL_BOOL_INO_OFFSET: c_ulong = 0x02000000;
const SEL_CLASS_INO_OFFSET: c_ulong = 0x04000000;
const SEL_POLICYCAP_INO_OFFSET: c_ulong = 0x08000000;
const SEL_INO_MASK: c_ulong = 0x00ffffff;

const BOOL_DIR_NAME: &[u8] = b"booleans\0";
const CLASS_DIR_NAME: &[u8] = b"class\0";
const NULL_FILE_NAME: &[u8] = b"null\0";
const TMPBUFLEN: usize = 12;

#[repr(C)]
pub struct dentry {
    pub d_inode: *mut inode,
    pub d_sb: *mut super_block,
    pub d_name: qstr,
}

#[repr(C)]
pub struct qstr {
    pub name: *const c_char,
}

#[repr(C)]
pub struct path {
    pub mnt: *mut vfsmount,
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct f_path {
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
    pub f_path: f_path,
}

#[repr(C)]
pub struct super_block {
    pub s_fs_info: *mut c_void,
    pub s_root: *mut dentry,
}

#[repr(C)]
pub struct inode {
    pub i_ino: ino_t,
    pub i_mode: umode_t,
    pub i_sb: *mut super_block,
    pub i_fop: *const file_operations,
    pub i_op: *const inode_operations,
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_end: c_ulong,
    pub vm_start: c_ulong,
    pub vm_pgoff: c_ulong,
    pub vm_flags: c_ulong,
    pub vm_page_prot: c_ulong,
    pub vm_ops: *const vm_operations_struct,
    pub vm_file: *mut file,
}

#[repr(C)]
pub struct vm_fault {
    pub vma: *mut vm_area_struct,
    pub flags: c_uint,
    pub pgoff: c_ulong,
    pub page: *mut page,
}

#[repr(C)]
pub struct page;
#[repr(C)]
pub struct selinux_kernel_status;
#[repr(C)]
pub struct selinux_policy;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct fs_context {
    pub ops: *const fs_context_operations,
}
#[repr(C)]
pub struct vfsmount {
    pub mnt_root: *mut dentry,
}
#[repr(C)]
pub struct mnt_idmap;
#[repr(C)]
pub struct kobject;
#[repr(C)]
pub struct task_struct {
    pub comm: [c_char; 16],
    pub pid: c_int,
}

#[repr(C)]
pub struct inode_security_struct {
    pub sid: u32_t,
    pub sclass: u16_t,
    pub initialized: c_uint,
}

#[repr(C)]
pub struct av_decision {
    pub allowed: u32_t,
    pub auditallow: u32_t,
    pub auditdeny: u32_t,
    pub seqno: u32_t,
    pub flags: u32_t,
}

#[repr(C)]
pub struct selinux_load_state {
    pub policy: *mut selinux_policy,
}

#[repr(C)]
pub struct renamedata {
    pub old_parent: *mut dentry,
    pub new_parent: *mut dentry,
}

#[repr(C)]
pub struct tree_descr {
    pub name: *const c_char,
    pub ops: *const file_operations,
    pub mode: umode_t,
}

#[repr(C)]
pub struct avc_cache_stats {
    pub lookups: c_uint,
    pub misses: c_uint,
    pub allocations: c_uint,
    pub reclaims: c_uint,
    pub frees: c_uint,
}

#[repr(C)]
pub struct seq_file;

#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub llseek: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct vm_operations_struct {
    pub fault: Option<unsafe extern "C" fn(*mut vm_fault) -> vm_fault_t>,
    pub page_mkwrite: Option<unsafe extern "C" fn(*mut vm_fault) -> vm_fault_t>,
}

#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut c_void>,
    pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut loff_t) -> *mut c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct inode_operations {
    pub lookup: Option<unsafe extern "C" fn()>,
    pub permission: Option<unsafe extern "C" fn(*mut mnt_idmap, *mut inode, c_int) -> c_int>,
}

#[repr(C)]
pub struct fs_context_operations {
    pub get_tree: Option<unsafe extern "C" fn(*mut fs_context) -> c_int>,
}

#[repr(C)]
pub struct file_system_type {
    pub name: *const c_char,
    pub init_fs_context: Option<unsafe extern "C" fn(*mut fs_context) -> c_int>,
    pub kill_sb: Option<unsafe extern "C" fn(*mut super_block)>,
}

#[repr(C)]
pub struct selinux_state_type {
    pub policy_mutex: mutex,
}

#[repr(C)]
pub struct selinux_fs_info {
    pub bool_dir: *mut dentry,
    pub bool_num: c_uint,
    pub bool_pending_names: *mut *mut c_char,
    pub bool_pending_values: *mut c_int,
    pub class_dir: *mut dentry,
    pub last_class_ino: c_ulong,
    pub last_ino: c_ulong,
    pub sb: *mut super_block,
}

#[repr(C)]
pub struct policy_load_memory {
    pub len: size_t,
    pub data: *mut c_void,
}

unsafe extern "C" {
    static mut selinux_state: selinux_state_type;
    static mut init_user_ns: c_void;
    static mut current: *mut task_struct;
    static mut fs_kobj: *mut kobject;
    static mut selinux_enabled_boot: bool_t;
    static mut simple_dir_inode_operations: inode_operations;
    static mut simple_dir_operations: file_operations;
    static mut selinux_policycap_names: [*const c_char; 0];
    static mut nr_cpu_ids: loff_t;
    static mut avc_cache_stats: avc_cache_stats;
    static mut SEQ_START_TOKEN: *mut c_void;
    static mut generic_file_llseek: unsafe extern "C" fn();
    static mut simple_transaction_read: unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t;
    static mut simple_transaction_release: unsafe extern "C" fn(*mut inode, *mut file) -> c_int;
    static mut seq_read: unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t;
    static mut seq_lseek: unsafe extern "C" fn();
    static mut seq_release: unsafe extern "C" fn(*mut inode, *mut file) -> c_int;
    static mut simple_lookup: unsafe extern "C" fn();

    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn vmalloc(size: size_t) -> *mut c_void;
    fn vfree(ptr: *mut c_void);
    fn memdup_user_nul(buf: *const c_char, count: size_t) -> *mut c_char;
    fn simple_read_from_buffer(to: *mut c_char, count: size_t, ppos: *mut loff_t, from: *const c_void, available: size_t) -> ssize_t;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> ssize_t;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> ssize_t;
    fn sscanf(buf: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn hex_to_bin(ch: c_char) -> c_int;

    fn enforcing_enabled() -> bool_t;
    fn enforcing_set(value: bool_t);
    fn current_sid() -> u32_t;
    fn avc_has_perm(ssid: u32_t, tsid: u32_t, tclass: u16_t, requested: u32_t, auditdata: *mut c_void) -> c_int;
    fn audit_context() -> *mut c_void;
    fn audit_log(ctx: *mut c_void, gfp: c_uint, typ: c_int, fmt: *const c_char, ...);
    fn from_kuid(ns: *mut c_void, kuid: c_uint) -> c_uint;
    fn audit_get_loginuid(task: *mut task_struct) -> c_uint;
    fn audit_get_sessionid(task: *mut task_struct) -> c_uint;
    fn avc_ss_reset(seqno: c_uint);
    fn selnl_notify_setenforce(value: bool_t);
    fn selinux_status_update_setenforce(value: bool_t);
    fn call_blocking_lsm_notifier(val: c_int, data: *mut c_void);
    fn selinux_ima_measure_state();
    fn security_get_reject_unknown() -> c_int;
    fn security_get_allow_unknown() -> c_int;
    fn selinux_kernel_status_page() -> *mut page;
    fn page_address(page: *mut page) -> *mut c_void;
    fn remap_pfn_range(vma: *mut vm_area_struct, addr: c_ulong, pfn: c_ulong, size: c_ulong, prot: c_ulong) -> c_int;
    fn page_to_pfn(page: *mut page) -> c_ulong;
    fn vm_flags_clear(vma: *mut vm_area_struct, flags: c_ulong);
    fn vm_flags_set(vma: *mut vm_area_struct, flags: c_ulong);
    fn security_mls_enabled() -> c_int;
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn security_read_policy(data: *mut *mut c_void, len: *mut size_t) -> c_int;
    fn i_size_read(inode: *mut inode) -> i64;
    fn inode_lock(inode: *mut inode);
    fn i_size_write(inode: *mut inode, size: size_t);
    fn inode_unlock(inode: *mut inode);
    fn vmalloc_to_page(addr: *mut c_void) -> *mut page;
    fn get_page(page: *mut page);
    fn security_load_policy(data: *mut c_void, count: size_t, load_state: *mut selinux_load_state) -> ssize_t;
    fn selinux_policy_cancel(load_state: *mut selinux_load_state);
    fn selinux_policy_commit(load_state: *mut selinux_load_state);
    fn copy_from_user(dst: *mut c_void, src: *const c_char, count: size_t) -> size_t;
    fn security_context_to_sid(s: *mut c_char, len: size_t, sid: *mut u32_t, gfp: c_uint) -> c_int;
    fn security_context_str_to_sid(s: *mut c_char, sid: *mut u32_t, gfp: c_uint) -> c_int;
    fn security_sid_to_context(sid: u32_t, s: *mut *mut c_char, len: *mut u32_t) -> c_int;
    fn checkreqprot_get() -> c_uint;
    fn security_validate_transition_user(osid: u32_t, nsid: u32_t, tsid: u32_t, tclass: u16_t) -> c_int;
    fn simple_transaction_get(file: *mut file, buf: *const c_char, size: size_t) -> *mut c_char;
    fn simple_transaction_set(file: *mut file, n: size_t);
    fn security_compute_av_user(ssid: u32_t, tsid: u32_t, tclass: u16_t, avd: *mut av_decision);
    fn security_transition_sid_user(ssid: u32_t, tsid: u32_t, tclass: u16_t, name: *const c_char, sid: *mut u32_t) -> c_int;
    fn security_change_sid(ssid: u32_t, tsid: u32_t, tclass: u16_t, sid: *mut u32_t) -> c_int;
    fn security_member_sid(ssid: u32_t, tsid: u32_t, tclass: u16_t, sid: *mut u32_t) -> c_int;
    fn new_inode(sb: *mut super_block) -> *mut inode;
    fn simple_inode_init_ts(inode: *mut inode);
    fn d_alloc_name(parent: *mut dentry, name: *const c_char) -> *mut dentry;
    fn iput(inode: *mut inode);
    fn d_make_persistent(dentry: *mut dentry, inode: *mut inode);
    fn dput(dentry: *mut dentry);
    fn file_inode(file: *mut file) -> *mut inode;
    fn security_get_bool_value(index: c_uint) -> c_int;
    fn security_set_bools(num: c_uint, values: *mut c_int) -> c_int;
    fn security_get_bools(policy: *mut selinux_policy, num: *mut u32_t, names: *mut *mut *mut c_char, values: *mut *mut c_int) -> c_int;
    fn selinux_inode(inode: *mut inode) -> *mut inode_security_struct;
    fn selinux_policy_genfs_sid(policy: *mut selinux_policy, fstype: *const c_char, path: *const c_char, class: u16_t, sid: *mut u32_t) -> c_int;
    fn avc_get_cache_threshold() -> c_uint;
    fn avc_set_cache_threshold(value: c_uint);
    fn avc_get_hash_stats(page: *mut c_char) -> ssize_t;
    fn security_sidtab_hash_stats(page: *mut c_char) -> ssize_t;
    fn cpu_possible(cpu: loff_t) -> bool_t;
    fn seq_puts(seq: *mut seq_file, s: *const c_char) -> c_int;
    fn seq_printf(seq: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn seq_open(file: *mut file, ops: *const seq_operations) -> c_int;
    fn security_get_initial_sid_context(i: c_uint) -> *const c_char;
    fn security_get_permissions(policy: *mut selinux_policy, objclass: *mut c_char, perms: *mut *mut *mut c_char, nperms: *mut u32_t) -> c_int;
    fn security_get_classes(policy: *mut selinux_policy, classes: *mut *mut *mut c_char, nclasses: *mut u32_t) -> c_int;
    fn security_policycap_supported(cap: c_ulong) -> c_int;
    fn inc_nlink(inode: *mut inode);
    fn d_inode(dentry: *mut dentry) -> *mut inode;
    fn simple_start_creating(parent: *mut dentry, name: *const c_char) -> *mut dentry;
    fn simple_done_creating(dentry: *mut dentry);
    fn simple_recursive_removal(dentry: *mut dentry, cb: *mut c_void);
    fn d_exchange(a: *mut dentry, b: *mut dentry);
    fn start_renaming_two_dentries(rd: *mut renamedata, a: *mut dentry, b: *mut dentry) -> c_int;
    fn end_renaming(rd: *mut renamedata);
    fn init_special_inode(inode: *mut inode, mode: umode_t, dev: c_uint);
    fn simple_fill_super(sb: *mut super_block, magic: c_ulong, files: *const tree_descr) -> c_int;
    fn get_tree_single(fc: *mut fs_context, fill: unsafe extern "C" fn(*mut super_block, *mut fs_context) -> c_int) -> c_int;
    fn kill_anon_super(sb: *mut super_block);
    fn sysfs_create_mount_point(kobj: *mut kobject, name: *const c_char) -> c_int;
    fn register_filesystem(fs: *mut file_system_type) -> c_int;
    fn kern_mount(fs: *mut file_system_type) -> *mut vfsmount;
    fn try_lookup_noperm(name: *const qstr, base: *mut dentry) -> *mut dentry;
    fn kern_unmount(mnt: *mut vfsmount);
    fn unregister_filesystem(fs: *mut file_system_type);
    fn sysfs_remove_mount_point(kobj: *mut kobject, name: *const c_char);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_err_once(fmt: *const c_char, ...);
    fn pr_warn_ratelimited(fmt: *const c_char, ...);
}

unsafe fn IS_ERR<T>(p: *mut T) -> bool {
    (p as isize) < 0 && (p as isize) >= -4095
}

unsafe fn PTR_ERR<T>(p: *mut T) -> c_int {
    p as isize as c_int
}

unsafe fn ERR_PTR<T>(err: c_int) -> *mut T {
    err as isize as *mut T
}

unsafe fn PTR_ERR_OR_ZERO<T>(p: *mut T) -> c_int {
    if IS_ERR(p) { PTR_ERR(p) } else { 0 }
}

unsafe fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

unsafe fn roundup(x: size_t, y: size_t) -> size_t {
    ((x + y - 1) / y) * y
}

unsafe fn MKDEV(major: c_uint, minor: c_uint) -> c_uint {
    (major << 8) | minor
}

unsafe extern "C" fn selinux_fs_info_create(sb: *mut super_block) -> c_int {
    let fsi = kzalloc(size_of::<selinux_fs_info>(), GFP_KERNEL) as *mut selinux_fs_info;
    if fsi.is_null() {
        return -ENOMEM;
    }
    (*fsi).last_ino = sel_inos::SEL_INO_NEXT as c_ulong - 1;
    (*fsi).sb = sb;
    (*sb).s_fs_info = fsi as *mut c_void;
    0
}

unsafe extern "C" fn selinux_fs_info_free(fsi: *mut selinux_fs_info) {
    let mut i: c_uint;
    if !fsi.is_null() {
        i = 0;
        while i < (*fsi).bool_num {
            kfree(*(*fsi).bool_pending_names.add(i as usize) as *mut c_void);
            i += 1;
        }
        kfree((*fsi).bool_pending_names as *mut c_void);
        kfree((*fsi).bool_pending_values as *mut c_void);
    }
    kfree(fsi as *mut c_void);
}

unsafe extern "C" fn sel_read_enforce(_filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut tmpbuf = [0 as c_char; TMPBUFLEN];
    let length = scnprintf(tmpbuf.as_mut_ptr(), TMPBUFLEN, c"%d".as_ptr(), enforcing_enabled() as c_int);
    simple_read_from_buffer(buf, count, ppos, tmpbuf.as_ptr() as *const c_void, length as size_t)
}

/* CONFIG_SECURITY_SELINUX_DEVELOP: when disabled, sel_write_enforce is NULL in C. */
unsafe extern "C" fn sel_write_enforce(_file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut page: *mut c_char = null_mut();
    let mut length: ssize_t;
    let mut scan_value: c_int = 0;

    if count >= PAGE_SIZE {
        return -ENOMEM as ssize_t;
    }
    /* No partial writes. */
    if *ppos != 0 {
        return -EINVAL as ssize_t;
    }
    page = memdup_user_nul(buf, count);
    if IS_ERR(page) {
        return PTR_ERR(page) as ssize_t;
    }
    length = -EINVAL as ssize_t;
    if sscanf(page, c"%d".as_ptr(), &mut scan_value) != 1 {
        goto_out_enforce(page, length)
    } else {
        let new_value = scan_value != 0;
        let old_value = enforcing_enabled();
        if new_value != old_value {
            length = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__SETENFORCE, null_mut()) as ssize_t;
            if length != 0 {
                return goto_out_enforce(page, length);
            }
            audit_log(audit_context(), GFP_KERNEL, AUDIT_MAC_STATUS,
                c"enforcing=%d old_enforcing=%d auid=%u ses=%u enabled=1 old-enabled=1 lsm=selinux res=1".as_ptr(),
                new_value as c_int, old_value as c_int,
                from_kuid(&raw mut init_user_ns, audit_get_loginuid(current)),
                audit_get_sessionid(current));
            enforcing_set(new_value);
            if new_value {
                avc_ss_reset(0);
            }
            selnl_notify_setenforce(new_value);
            selinux_status_update_setenforce(new_value);
            if !new_value {
                call_blocking_lsm_notifier(LSM_POLICY_CHANGE, null_mut());
            }
            selinux_ima_measure_state();
        }
        length = count as ssize_t;
        goto_out_enforce(page, length)
    }
}

unsafe fn goto_out_enforce(page: *mut c_char, length: ssize_t) -> ssize_t {
    kfree(page as *mut c_void);
    length
}

static sel_enforce_ops: file_operations = file_operations {
    open: None,
    read: Some(sel_read_enforce),
    write: Some(sel_write_enforce),
    mmap: None,
    release: None,
    llseek: unsafe { Some(generic_file_llseek) },
};

unsafe extern "C" fn sel_read_handle_unknown(filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut tmpbuf = [0 as c_char; TMPBUFLEN];
    let ino = (*file_inode(filp)).i_ino;
    let handle_unknown = if ino == sel_inos::SEL_REJECT_UNKNOWN as ino_t {
        security_get_reject_unknown()
    } else {
        (!security_get_allow_unknown()) as c_int
    };
    let length = scnprintf(tmpbuf.as_mut_ptr(), TMPBUFLEN, c"%d".as_ptr(), handle_unknown);
    simple_read_from_buffer(buf, count, ppos, tmpbuf.as_ptr() as *const c_void, length as size_t)
}

static sel_handle_unknown_ops: file_operations = file_operations {
    open: None,
    read: Some(sel_read_handle_unknown),
    write: None,
    mmap: None,
    release: None,
    llseek: unsafe { Some(generic_file_llseek) },
};

unsafe extern "C" fn sel_open_handle_status(_inode: *mut inode, filp: *mut file) -> c_int {
    let status = selinux_kernel_status_page();
    if status.is_null() {
        return -ENOMEM;
    }
    (*filp).private_data = status as *mut c_void;
    0
}

unsafe extern "C" fn sel_read_handle_status(filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let status = (*filp).private_data as *mut page;
    assert!(!status.is_null());
    simple_read_from_buffer(buf, count, ppos, page_address(status), size_of::<selinux_kernel_status>())
}

unsafe extern "C" fn sel_mmap_handle_status(filp: *mut file, vma: *mut vm_area_struct) -> c_int {
    let status = (*filp).private_data as *mut page;
    let size = (*vma).vm_end - (*vma).vm_start;
    assert!(!status.is_null());
    /* only allows one page from the head */
    if (*vma).vm_pgoff > 0 || size != PAGE_SIZE as c_ulong {
        return -EIO;
    }
    /* disallow writable mapping */
    if ((*vma).vm_flags & VM_WRITE) != 0 {
        return -EPERM;
    }
    /* disallow mprotect() turns it into writable */
    vm_flags_clear(vma, VM_MAYWRITE);
    remap_pfn_range(vma, (*vma).vm_start, page_to_pfn(status), size, (*vma).vm_page_prot)
}

static sel_handle_status_ops: file_operations = file_operations {
    open: Some(sel_open_handle_status),
    read: Some(sel_read_handle_status),
    write: None,
    mmap: Some(sel_mmap_handle_status),
    release: None,
    llseek: unsafe { Some(generic_file_llseek) },
};

unsafe extern "C" fn sel_write_disable(_file: *mut file, _buf: *const c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    /*
     * Setting disable is no longer supported, see
     * https://github.com/SELinuxProject/selinux-kernel/wiki/DEPRECATE-runtime-disable
     */
    pr_err_once(c"SELinux: %s (%d) wrote to disable. This is no longer supported.\n".as_ptr(), (*current).comm.as_ptr(), (*current).pid);
    count as ssize_t
}

static sel_disable_ops: file_operations = file_operations {
    open: None,
    read: None,
    write: Some(sel_write_disable),
    mmap: None,
    release: None,
    llseek: unsafe { Some(generic_file_llseek) },
};

unsafe extern "C" fn sel_read_policyvers(_filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut tmpbuf = [0 as c_char; TMPBUFLEN];
    let length = scnprintf(tmpbuf.as_mut_ptr(), TMPBUFLEN, c"%u".as_ptr(), POLICYDB_VERSION_MAX);
    simple_read_from_buffer(buf, count, ppos, tmpbuf.as_ptr() as *const c_void, length as size_t)
}

static sel_policyvers_ops: file_operations = file_operations {
    open: None,
    read: Some(sel_read_policyvers),
    write: None,
    mmap: None,
    release: None,
    llseek: unsafe { Some(generic_file_llseek) },
};

unsafe extern "C" fn sel_read_mls(_filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut tmpbuf = [0 as c_char; TMPBUFLEN];
    let length = scnprintf(tmpbuf.as_mut_ptr(), TMPBUFLEN, c"%d".as_ptr(), security_mls_enabled());
    simple_read_from_buffer(buf, count, ppos, tmpbuf.as_ptr() as *const c_void, length as size_t)
}

static sel_mls_ops: file_operations = file_operations {
    open: None,
    read: Some(sel_read_mls),
    write: None,
    mmap: None,
    release: None,
    llseek: unsafe { Some(generic_file_llseek) },
};

unsafe extern "C" fn sel_open_policy(inodep: *mut inode, filp: *mut file) -> c_int {
    let mut rc: c_int;
    rc = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__READ_POLICY, null_mut());
    if rc != 0 {
        return rc;
    }
    let plm = kzalloc(size_of::<policy_load_memory>(), GFP_KERNEL) as *mut policy_load_memory;
    if plm.is_null() {
        return -ENOMEM;
    }
    mutex_lock(&raw mut selinux_state.policy_mutex);
    rc = security_read_policy(&mut (*plm).data, &mut (*plm).len);
    if rc != 0 {
        mutex_unlock(&raw mut selinux_state.policy_mutex);
        if !plm.is_null() {
            vfree((*plm).data);
        }
        kfree(plm as *mut c_void);
        return rc;
    }
    if i_size_read(inodep) as size_t != (*plm).len {
        inode_lock(inodep);
        i_size_write(inodep, (*plm).len);
        inode_unlock(inodep);
    }
    mutex_unlock(&raw mut selinux_state.policy_mutex);
    (*filp).private_data = plm as *mut c_void;
    0
}

unsafe extern "C" fn sel_release_policy(_inode: *mut inode, filp: *mut file) -> c_int {
    let plm = (*filp).private_data as *mut policy_load_memory;
    vfree((*plm).data);
    kfree(plm as *mut c_void);
    0
}

unsafe extern "C" fn sel_read_policy(filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let plm = (*filp).private_data as *mut policy_load_memory;
    let ret = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__READ_POLICY, null_mut());
    if ret != 0 {
        return ret as ssize_t;
    }
    simple_read_from_buffer(buf, count, ppos, (*plm).data, (*plm).len)
}

unsafe extern "C" fn sel_mmap_policy_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let plm = (*(*(*(*vmf).vma).vm_file).private_data.cast::<policy_load_memory>());
    if ((*vmf).flags & (FAULT_FLAG_MKWRITE | FAULT_FLAG_WRITE)) != 0 {
        return VM_FAULT_SIGBUS;
    }
    let offset = (*vmf).pgoff << PAGE_SHIFT;
    if offset >= roundup(plm.len, PAGE_SIZE) as c_ulong {
        return VM_FAULT_SIGBUS;
    }
    let pagep = vmalloc_to_page((plm.data as *mut u8).add(offset as usize) as *mut c_void);
    get_page(pagep);
    (*vmf).page = pagep;
    0
}

static sel_mmap_policy_ops: vm_operations_struct = vm_operations_struct {
    fault: Some(sel_mmap_policy_fault),
    page_mkwrite: Some(sel_mmap_policy_fault),
};

unsafe extern "C" fn sel_mmap_policy(_filp: *mut file, vma: *mut vm_area_struct) -> c_int {
    if ((*vma).vm_flags & VM_SHARED) != 0 {
        /* do not allow mprotect to make mapping writable */
        vm_flags_clear(vma, VM_MAYWRITE);
        if ((*vma).vm_flags & VM_WRITE) != 0 {
            return -EACCES;
        }
    }
    vm_flags_set(vma, VM_DONTEXPAND | VM_DONTDUMP);
    (*vma).vm_ops = &sel_mmap_policy_ops;
    0
}

static sel_policy_ops: file_operations = file_operations {
    open: Some(sel_open_policy),
    read: Some(sel_read_policy),
    write: None,
    mmap: Some(sel_mmap_policy),
    release: Some(sel_release_policy),
    llseek: unsafe { Some(generic_file_llseek) },
};

unsafe extern "C" fn sel_remove_old_bool_data(bool_num: c_uint, bool_names: *mut *mut c_char, bool_values: *mut c_int) {
    let mut i: u32_t = 0;
    /* bool_dir cleanup */
    while i < bool_num {
        kfree(*bool_names.add(i as usize) as *mut c_void);
        i += 1;
    }
    kfree(bool_names as *mut c_void);
    kfree(bool_values as *mut c_void);
}

unsafe extern "C" fn sel_make_policy_nodes(fsi: *mut selinux_fs_info, newpolicy: *mut selinux_policy) -> c_int {
    let mut ret: c_int = 0;
    let mut rd: renamedata = renamedata { old_parent: null_mut(), new_parent: null_mut() };
    let mut bool_num: c_uint = 0;
    let mut bool_names: *mut *mut c_char = null_mut();
    let mut bool_values: *mut c_int = null_mut();
    let mut tmp_ino = (*fsi).last_ino; /* Don't increment last_ino in this function */
    let tmp_parent = sel_make_swapover_dir((*fsi).sb, &mut tmp_ino);
    if IS_ERR(tmp_parent) {
        return PTR_ERR(tmp_parent);
    }
    tmp_ino = (*(*(*fsi).bool_dir).d_inode).i_ino - 1; /* sel_make_dir will increment and set */
    let tmp_bool_dir = sel_make_dir(tmp_parent, BOOL_DIR_NAME.as_ptr() as *const c_char, &mut tmp_ino);
    if IS_ERR(tmp_bool_dir) {
        ret = PTR_ERR(tmp_bool_dir);
        sel_remove_old_bool_data(bool_num, bool_names, bool_values);
        simple_recursive_removal(tmp_parent, null_mut());
        return ret;
    }
    tmp_ino = (*(*(*fsi).class_dir).d_inode).i_ino - 1; /* sel_make_dir will increment and set */
    let tmp_class_dir = sel_make_dir(tmp_parent, CLASS_DIR_NAME.as_ptr() as *const c_char, &mut tmp_ino);
    if IS_ERR(tmp_class_dir) {
        ret = PTR_ERR(tmp_class_dir);
        sel_remove_old_bool_data(bool_num, bool_names, bool_values);
        simple_recursive_removal(tmp_parent, null_mut());
        return ret;
    }
    ret = sel_make_bools(newpolicy, tmp_bool_dir, &mut bool_num, &mut bool_names, &mut bool_values);
    if ret == 0 {
        ret = sel_make_classes(newpolicy, tmp_class_dir, &mut (*fsi).last_class_ino);
    }
    if ret == 0 {
        rd.old_parent = tmp_parent;
        rd.new_parent = (*(*fsi).sb).s_root;
        /* booleans */
        ret = start_renaming_two_dentries(&mut rd, tmp_bool_dir, (*fsi).bool_dir);
        if ret == 0 {
            d_exchange(tmp_bool_dir, (*fsi).bool_dir);
            core::mem::swap(&mut (*fsi).bool_num, &mut bool_num);
            core::mem::swap(&mut (*fsi).bool_pending_names, &mut bool_names);
            core::mem::swap(&mut (*fsi).bool_pending_values, &mut bool_values);
            (*fsi).bool_dir = tmp_bool_dir;
            end_renaming(&mut rd);
            /* classes */
            ret = start_renaming_two_dentries(&mut rd, tmp_class_dir, (*fsi).class_dir);
            if ret == 0 {
                d_exchange(tmp_class_dir, (*fsi).class_dir);
                (*fsi).class_dir = tmp_class_dir;
                end_renaming(&mut rd);
            }
        }
    }
    sel_remove_old_bool_data(bool_num, bool_names, bool_values);
    /* Since the other temporary dirs are children of tmp_parent
     * this will handle all the cleanup in the case of a failure before
     * the swapover
     */
    simple_recursive_removal(tmp_parent, null_mut());
    ret
}

unsafe extern "C" fn sel_write_load(filep: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut load_state: selinux_load_state = core::mem::zeroed();
    let mut length: ssize_t;
    let data: *mut c_void;
    /* no partial writes */
    if *ppos != 0 {
        return -EINVAL as ssize_t;
    }
    /* no empty policies */
    if count == 0 {
        return -EINVAL as ssize_t;
    }
    length = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__LOAD_POLICY, null_mut()) as ssize_t;
    if length != 0 {
        return length;
    }
    data = vmalloc(count);
    if data.is_null() {
        return -ENOMEM as ssize_t;
    }
    if copy_from_user(data, buf, count) != 0 {
        vfree(data);
        return -EFAULT as ssize_t;
    }
    mutex_lock(&raw mut selinux_state.policy_mutex);
    length = security_load_policy(data, count, &mut load_state);
    if length != 0 {
        pr_warn_ratelimited(c"SELinux: failed to load policy\n".as_ptr());
    } else {
        let fsi = (*file_inode(filep)).i_sb.as_mut().unwrap().s_fs_info as *mut selinux_fs_info;
        length = sel_make_policy_nodes(fsi, load_state.policy) as ssize_t;
        if length != 0 {
            pr_warn_ratelimited(c"SELinux: failed to initialize selinuxfs\n".as_ptr());
            selinux_policy_cancel(&mut load_state);
        } else {
            selinux_policy_commit(&mut load_state);
            length = count as ssize_t;
            audit_log(audit_context(), GFP_KERNEL, AUDIT_MAC_POLICY_LOAD,
                c"auid=%u ses=%u lsm=selinux res=1".as_ptr(),
                from_kuid(&raw mut init_user_ns, audit_get_loginuid(current)),
                audit_get_sessionid(current));
        }
    }
    mutex_unlock(&raw mut selinux_state.policy_mutex);
    vfree(data);
    length
}

static sel_load_ops: file_operations = file_operations {
    open: None,
    read: None,
    write: Some(sel_write_load),
    mmap: None,
    release: None,
    llseek: unsafe { Some(generic_file_llseek) },
};

unsafe extern "C" fn sel_write_context(_file: *mut file, buf: *mut c_char, size: size_t) -> ssize_t {
    let mut canon: *mut c_char = null_mut();
    let mut sid: u32_t = 0;
    let mut len: u32_t = 0;
    let mut length = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__CHECK_CONTEXT, null_mut()) as ssize_t;
    if length == 0 {
        length = security_context_to_sid(buf, size, &mut sid, GFP_KERNEL) as ssize_t;
    }
    if length == 0 {
        length = security_sid_to_context(sid, &mut canon, &mut len) as ssize_t;
    }
    if length == 0 {
        length = -ERANGE as ssize_t;
        if len as usize > SIMPLE_TRANSACTION_LIMIT {
            pr_err(c"SELinux: %s:  context size (%u) exceeds payload max\n".as_ptr(), c"sel_write_context".as_ptr(), len);
        } else {
            memcpy(buf as *mut c_void, canon as *const c_void, len as size_t);
            length = len as ssize_t;
        }
    }
    kfree(canon as *mut c_void);
    length
}

unsafe extern "C" fn sel_read_checkreqprot(_filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut tmpbuf = [0 as c_char; TMPBUFLEN];
    let length = scnprintf(tmpbuf.as_mut_ptr(), TMPBUFLEN, c"%u".as_ptr(), checkreqprot_get());
    simple_read_from_buffer(buf, count, ppos, tmpbuf.as_ptr() as *const c_void, length as size_t)
}

unsafe extern "C" fn sel_write_checkreqprot(_file: *mut file, _buf: *const c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    /*
     * Setting checkreqprot is no longer supported, see
     * https://github.com/SELinuxProject/selinux-kernel/wiki/DEPRECATE-checkreqprot
     */
    pr_err_once(c"SELinux: %s (%d) wrote to checkreqprot. This is no longer supported.\n".as_ptr(), (*current).comm.as_ptr(), (*current).pid);
    count as ssize_t
}

static sel_checkreqprot_ops: file_operations = file_operations {
    open: None,
    read: Some(sel_read_checkreqprot),
    write: Some(sel_write_checkreqprot),
    mmap: None,
    release: None,
    llseek: unsafe { Some(generic_file_llseek) },
};

unsafe extern "C" fn sel_write_validatetrans(_file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut oldcon: *mut c_char = null_mut();
    let mut newcon: *mut c_char = null_mut();
    let mut taskcon: *mut c_char = null_mut();
    let mut req: *mut c_char = null_mut();
    let mut osid: u32_t = 0;
    let mut nsid: u32_t = 0;
    let mut tsid: u32_t = 0;
    let mut tclass: u16_t = 0;
    let mut rc = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__VALIDATE_TRANS, null_mut());
    if rc != 0 { goto_validatetrans(req, oldcon, newcon, taskcon, rc as ssize_t) } else if count >= PAGE_SIZE {
        goto_validatetrans(req, oldcon, newcon, taskcon, -ENOMEM as ssize_t)
    } else if *ppos != 0 {
        goto_validatetrans(req, oldcon, newcon, taskcon, -EINVAL as ssize_t)
    } else {
        req = memdup_user_nul(buf, count);
        if IS_ERR(req) {
            rc = PTR_ERR(req);
            req = null_mut();
            return goto_validatetrans(req, oldcon, newcon, taskcon, rc as ssize_t);
        }
        oldcon = kzalloc(count + 1, GFP_KERNEL) as *mut c_char;
        newcon = kzalloc(count + 1, GFP_KERNEL) as *mut c_char;
        taskcon = kzalloc(count + 1, GFP_KERNEL) as *mut c_char;
        if oldcon.is_null() || newcon.is_null() || taskcon.is_null() {
            return goto_validatetrans(req, oldcon, newcon, taskcon, -ENOMEM as ssize_t);
        }
        if sscanf(req, c"%s %s %hu %s".as_ptr(), oldcon, newcon, &mut tclass, taskcon) != 4 {
            return goto_validatetrans(req, oldcon, newcon, taskcon, -EINVAL as ssize_t);
        }
        rc = security_context_str_to_sid(oldcon, &mut osid, GFP_KERNEL);
        if rc == 0 { rc = security_context_str_to_sid(newcon, &mut nsid, GFP_KERNEL); }
        if rc == 0 { rc = security_context_str_to_sid(taskcon, &mut tsid, GFP_KERNEL); }
        if rc == 0 { rc = security_validate_transition_user(osid, nsid, tsid, tclass); }
        if rc == 0 { rc = count as c_int; }
        goto_validatetrans(req, oldcon, newcon, taskcon, rc as ssize_t)
    }
}

unsafe fn goto_validatetrans(req: *mut c_char, oldcon: *mut c_char, newcon: *mut c_char, taskcon: *mut c_char, rc: ssize_t) -> ssize_t {
    kfree(req as *mut c_void);
    kfree(oldcon as *mut c_void);
    kfree(newcon as *mut c_void);
    kfree(taskcon as *mut c_void);
    rc
}

static sel_transition_ops: file_operations = file_operations {
    open: None,
    read: None,
    write: Some(sel_write_validatetrans),
    mmap: None,
    release: None,
    llseek: unsafe { Some(generic_file_llseek) },
};

type write_op_t = Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t) -> ssize_t>;

static write_op: [write_op_t; sel_inos::SEL_CONTEXT as usize + 1] = {
    let mut a: [write_op_t; sel_inos::SEL_CONTEXT as usize + 1] = [None; sel_inos::SEL_CONTEXT as usize + 1];
    a[sel_inos::SEL_ACCESS as usize] = Some(sel_write_access);
    a[sel_inos::SEL_CREATE as usize] = Some(sel_write_create);
    a[sel_inos::SEL_RELABEL as usize] = Some(sel_write_relabel);
    a[sel_inos::SEL_USER as usize] = Some(sel_write_user);
    a[sel_inos::SEL_MEMBER as usize] = Some(sel_write_member);
    a[sel_inos::SEL_CONTEXT as usize] = Some(sel_write_context);
    a
};

unsafe extern "C" fn selinux_transaction_write(filep: *mut file, buf: *const c_char, size: size_t, _pos: *mut loff_t) -> ssize_t {
    let ino = (*file_inode(filep)).i_ino as usize;
    if ino >= write_op.len() || write_op[ino].is_none() {
        return -EINVAL as ssize_t;
    }
    let data = simple_transaction_get(filep, buf, size);
    if IS_ERR(data) {
        return PTR_ERR(data) as ssize_t;
    }
    let mut rv = write_op[ino].unwrap()(filep, data, size);
    if rv > 0 {
        simple_transaction_set(filep, rv as size_t);
        rv = size as ssize_t;
    }
    rv
}

static transaction_ops: file_operations = file_operations {
    open: None,
    read: unsafe { Some(simple_transaction_read) },
    write: Some(selinux_transaction_write),
    mmap: None,
    release: unsafe { Some(simple_transaction_release) },
    llseek: unsafe { Some(generic_file_llseek) },
};

unsafe extern "C" fn sel_write_access(_file: *mut file, buf: *mut c_char, size: size_t) -> ssize_t {
    let mut scon: *mut c_char = null_mut();
    let mut tcon: *mut c_char = null_mut();
    let mut ssid: u32_t = 0;
    let mut tsid: u32_t = 0;
    let mut tclass: u16_t = 0;
    let mut avd: av_decision = core::mem::zeroed();
    let mut length = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__COMPUTE_AV, null_mut()) as ssize_t;
    if length == 0 {
        scon = kzalloc(size + 1, GFP_KERNEL) as *mut c_char;
        tcon = kzalloc(size + 1, GFP_KERNEL) as *mut c_char;
        if scon.is_null() || tcon.is_null() {
            length = -ENOMEM as ssize_t;
        } else if sscanf(buf, c"%s %s %hu".as_ptr(), scon, tcon, &mut tclass) != 3 {
            length = -EINVAL as ssize_t;
        } else {
            length = security_context_str_to_sid(scon, &mut ssid, GFP_KERNEL) as ssize_t;
            if length == 0 { length = security_context_str_to_sid(tcon, &mut tsid, GFP_KERNEL) as ssize_t; }
            if length == 0 {
                security_compute_av_user(ssid, tsid, tclass, &mut avd);
                length = scnprintf(buf, SIMPLE_TRANSACTION_LIMIT, c"%x %x %x %x %u %x".as_ptr(), avd.allowed, 0xffffffffu32, avd.auditallow, avd.auditdeny, avd.seqno, avd.flags);
            }
        }
    }
    kfree(tcon as *mut c_void);
    kfree(scon as *mut c_void);
    length
}

unsafe extern "C" fn sel_write_create(_file: *mut file, buf: *mut c_char, size: size_t) -> ssize_t {
    let mut scon: *mut c_char = null_mut();
    let mut tcon: *mut c_char = null_mut();
    let mut namebuf: *mut c_char = null_mut();
    let mut objname: *mut c_char = null_mut();
    let mut ssid: u32_t = 0;
    let mut tsid: u32_t = 0;
    let mut newsid: u32_t = 0;
    let mut tclass: u16_t = 0;
    let mut newcon: *mut c_char = null_mut();
    let mut len: u32_t = 0;
    let mut length = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__COMPUTE_CREATE, null_mut()) as ssize_t;
    if length == 0 {
        scon = kzalloc(size + 1, GFP_KERNEL) as *mut c_char;
        tcon = kzalloc(size + 1, GFP_KERNEL) as *mut c_char;
        namebuf = kzalloc(size + 1, GFP_KERNEL) as *mut c_char;
        if scon.is_null() || tcon.is_null() || namebuf.is_null() {
            length = -ENOMEM as ssize_t;
        } else {
            let nargs = sscanf(buf, c"%s %s %hu %s".as_ptr(), scon, tcon, &mut tclass, namebuf);
            if nargs < 3 || nargs > 4 {
                length = -EINVAL as ssize_t;
            } else {
                if nargs == 4 {
                    /*
                     * If and when the name of new object to be queried contains
                     * either whitespace or multibyte characters, they shall be
                     * encoded based on the percentage-encoding rule.
                     * If not encoded, the sscanf logic picks up only left-half
                     * of the supplied name; split by a whitespace unexpectedly.
                     */
                    let mut r = namebuf;
                    let mut w = namebuf;
                    loop {
                        let mut c1 = *r as c_int;
                        r = r.add(1);
                        if c1 == b'+' as c_int {
                            c1 = b' ' as c_int;
                        } else if c1 == b'%' as c_int {
                            c1 = hex_to_bin(*r);
                            r = r.add(1);
                            if c1 < 0 { length = -EINVAL as ssize_t; break; }
                            let c2 = hex_to_bin(*r);
                            r = r.add(1);
                            if c2 < 0 { length = -EINVAL as ssize_t; break; }
                            c1 = (c1 << 4) | c2;
                        }
                        *w = c1 as c_char;
                        w = w.add(1);
                        if c1 == 0 { break; }
                    }
                    objname = namebuf;
                }
                if length == 0 {
                    length = security_context_str_to_sid(scon, &mut ssid, GFP_KERNEL) as ssize_t;
                    if length == 0 { length = security_context_str_to_sid(tcon, &mut tsid, GFP_KERNEL) as ssize_t; }
                    if length == 0 { length = security_transition_sid_user(ssid, tsid, tclass, objname, &mut newsid) as ssize_t; }
                    if length == 0 { length = security_sid_to_context(newsid, &mut newcon, &mut len) as ssize_t; }
                    if length == 0 {
                        length = -ERANGE as ssize_t;
                        if len as usize > SIMPLE_TRANSACTION_LIMIT {
                            pr_err(c"SELinux: %s:  context size (%u) exceeds payload max\n".as_ptr(), c"sel_write_create".as_ptr(), len);
                        } else {
                            memcpy(buf as *mut c_void, newcon as *const c_void, len as size_t);
                            length = len as ssize_t;
                        }
                    }
                }
            }
        }
    }
    kfree(newcon as *mut c_void);
    kfree(namebuf as *mut c_void);
    kfree(tcon as *mut c_void);
    kfree(scon as *mut c_void);
    length
}

unsafe extern "C" fn sel_write_relabel(_file: *mut file, buf: *mut c_char, size: size_t) -> ssize_t {
    let mut scon: *mut c_char = null_mut();
    let mut tcon: *mut c_char = null_mut();
    let mut ssid: u32_t = 0;
    let mut tsid: u32_t = 0;
    let mut newsid: u32_t = 0;
    let mut tclass: u16_t = 0;
    let mut newcon: *mut c_char = null_mut();
    let mut len: u32_t = 0;
    let mut length = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__COMPUTE_RELABEL, null_mut()) as ssize_t;
    if length == 0 {
        scon = kzalloc(size + 1, GFP_KERNEL) as *mut c_char;
        tcon = kzalloc(size + 1, GFP_KERNEL) as *mut c_char;
        if scon.is_null() || tcon.is_null() {
            length = -ENOMEM as ssize_t;
        } else if sscanf(buf, c"%s %s %hu".as_ptr(), scon, tcon, &mut tclass) != 3 {
            length = -EINVAL as ssize_t;
        } else {
            length = security_context_str_to_sid(scon, &mut ssid, GFP_KERNEL) as ssize_t;
            if length == 0 { length = security_context_str_to_sid(tcon, &mut tsid, GFP_KERNEL) as ssize_t; }
            if length == 0 { length = security_change_sid(ssid, tsid, tclass, &mut newsid) as ssize_t; }
            if length == 0 { length = security_sid_to_context(newsid, &mut newcon, &mut len) as ssize_t; }
            if length == 0 {
                length = -ERANGE as ssize_t;
                if len as usize <= SIMPLE_TRANSACTION_LIMIT {
                    memcpy(buf as *mut c_void, newcon as *const c_void, len as size_t);
                    length = len as ssize_t;
                }
            }
        }
    }
    kfree(newcon as *mut c_void);
    kfree(tcon as *mut c_void);
    kfree(scon as *mut c_void);
    length
}

unsafe extern "C" fn sel_write_user(_file: *mut file, buf: *mut c_char, _size: size_t) -> ssize_t {
    pr_err_once(c"SELinux: %s (%d) wrote to user. This is no longer supported.\n".as_ptr(), (*current).comm.as_ptr(), (*current).pid);
    *buf.add(0) = b'0' as c_char;
    *buf.add(1) = 0;
    2
}

unsafe extern "C" fn sel_write_member(_file: *mut file, buf: *mut c_char, size: size_t) -> ssize_t {
    let mut scon: *mut c_char = null_mut();
    let mut tcon: *mut c_char = null_mut();
    let mut ssid: u32_t = 0;
    let mut tsid: u32_t = 0;
    let mut newsid: u32_t = 0;
    let mut tclass: u16_t = 0;
    let mut newcon: *mut c_char = null_mut();
    let mut len: u32_t = 0;
    let mut length = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__COMPUTE_MEMBER, null_mut()) as ssize_t;
    if length == 0 {
        scon = kzalloc(size + 1, GFP_KERNEL) as *mut c_char;
        tcon = kzalloc(size + 1, GFP_KERNEL) as *mut c_char;
        if scon.is_null() || tcon.is_null() {
            length = -ENOMEM as ssize_t;
        } else if sscanf(buf, c"%s %s %hu".as_ptr(), scon, tcon, &mut tclass) != 3 {
            length = -EINVAL as ssize_t;
        } else {
            length = security_context_str_to_sid(scon, &mut ssid, GFP_KERNEL) as ssize_t;
            if length == 0 { length = security_context_str_to_sid(tcon, &mut tsid, GFP_KERNEL) as ssize_t; }
            if length == 0 { length = security_member_sid(ssid, tsid, tclass, &mut newsid) as ssize_t; }
            if length == 0 { length = security_sid_to_context(newsid, &mut newcon, &mut len) as ssize_t; }
            if length == 0 {
                length = -ERANGE as ssize_t;
                if len as usize > SIMPLE_TRANSACTION_LIMIT {
                    pr_err(c"SELinux: %s:  context size (%u) exceeds payload max\n".as_ptr(), c"sel_write_member".as_ptr(), len);
                } else {
                    memcpy(buf as *mut c_void, newcon as *const c_void, len as size_t);
                    length = len as ssize_t;
                }
            }
        }
    }
    kfree(newcon as *mut c_void);
    kfree(tcon as *mut c_void);
    kfree(scon as *mut c_void);
    length
}

unsafe extern "C" fn sel_make_inode(sb: *mut super_block, mode: umode_t) -> *mut inode {
    let ret = new_inode(sb);
    if !ret.is_null() {
        (*ret).i_mode = mode;
        simple_inode_init_ts(ret);
    }
    ret
}

unsafe extern "C" fn sel_attach(parent: *mut dentry, name: *const c_char, inodep: *mut inode) -> *mut dentry {
    let dentryp = d_alloc_name(parent, name);
    if dentryp.is_null() {
        iput(inodep);
        return ERR_PTR(-ENOMEM);
    }
    d_make_persistent(dentryp, inodep);
    dput(dentryp);
    dentryp
}

unsafe extern "C" fn sel_attach_file(parent: *mut dentry, name: *const c_char, inodep: *mut inode) -> c_int {
    let dentryp = sel_attach(parent, name, inodep);
    PTR_ERR_OR_ZERO(dentryp)
}

unsafe extern "C" fn sel_read_bool(filep: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let fsi = (*(*file_inode(filep)).i_sb).s_fs_info as *mut selinux_fs_info;
    let mut buffer = [0 as c_char; 4];
    let index = ((*file_inode(filep)).i_ino & SEL_INO_MASK) as c_uint;
    let name = (*(*filep).f_path.dentry).d_name.name;
    mutex_lock(&raw mut selinux_state.policy_mutex);
    let mut ret: ssize_t = -EINVAL as ssize_t;
    if index < (*fsi).bool_num && strcmp(name, *(*fsi).bool_pending_names.add(index as usize)) == 0 {
        let cur_enforcing = security_get_bool_value(index);
        if cur_enforcing < 0 {
            ret = cur_enforcing as ssize_t;
        } else {
            let length = scnprintf(buffer.as_mut_ptr(), buffer.len(), c"%d %d".as_ptr(), (cur_enforcing != 0) as c_int, (*(*fsi).bool_pending_values.add(index as usize) != 0) as c_int);
            mutex_unlock(&raw mut selinux_state.policy_mutex);
            return simple_read_from_buffer(buf, count, ppos, buffer.as_ptr() as *const c_void, length as size_t);
        }
    }
    mutex_unlock(&raw mut selinux_state.policy_mutex);
    ret
}

unsafe extern "C" fn sel_write_bool(filep: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let fsi = (*(*file_inode(filep)).i_sb).s_fs_info as *mut selinux_fs_info;
    let index = ((*file_inode(filep)).i_ino & SEL_INO_MASK) as c_uint;
    let name = (*(*filep).f_path.dentry).d_name.name;
    let mut new_value: c_int = 0;
    if count >= PAGE_SIZE { return -ENOMEM as ssize_t; }
    /* No partial writes. */
    if *ppos != 0 { return -EINVAL as ssize_t; }
    let page = memdup_user_nul(buf, count);
    if IS_ERR(page) { return PTR_ERR(page) as ssize_t; }
    mutex_lock(&raw mut selinux_state.policy_mutex);
    let mut length = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__SETBOOL, null_mut()) as ssize_t;
    if length == 0 {
        if index >= (*fsi).bool_num || strcmp(name, *(*fsi).bool_pending_names.add(index as usize)) != 0 {
            length = -EINVAL as ssize_t;
        } else if sscanf(page, c"%d".as_ptr(), &mut new_value) != 1 {
            length = -EINVAL as ssize_t;
        } else {
            if new_value != 0 { new_value = 1; }
            *(*fsi).bool_pending_values.add(index as usize) = new_value;
            length = count as ssize_t;
        }
    }
    mutex_unlock(&raw mut selinux_state.policy_mutex);
    kfree(page as *mut c_void);
    length
}

static sel_bool_ops: file_operations = file_operations {
    open: None,
    read: Some(sel_read_bool),
    write: Some(sel_write_bool),
    mmap: None,
    release: None,
    llseek: unsafe { Some(generic_file_llseek) },
};

unsafe extern "C" fn sel_commit_bools_write(filep: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let fsi = (*(*file_inode(filep)).i_sb).s_fs_info as *mut selinux_fs_info;
    let mut new_value: c_int = 0;
    if count >= PAGE_SIZE { return -ENOMEM as ssize_t; }
    /* No partial writes. */
    if *ppos != 0 { return -EINVAL as ssize_t; }
    let page = memdup_user_nul(buf, count);
    if IS_ERR(page) { return PTR_ERR(page) as ssize_t; }
    mutex_lock(&raw mut selinux_state.policy_mutex);
    let mut length = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__SETBOOL, null_mut()) as ssize_t;
    if length == 0 {
        if sscanf(page, c"%d".as_ptr(), &mut new_value) != 1 {
            length = -EINVAL as ssize_t;
        } else {
            length = 0;
            if new_value != 0 && !(*fsi).bool_pending_values.is_null() {
                length = security_set_bools((*fsi).bool_num, (*fsi).bool_pending_values) as ssize_t;
            }
            if length == 0 {
                length = count as ssize_t;
            }
        }
    }
    mutex_unlock(&raw mut selinux_state.policy_mutex);
    kfree(page as *mut c_void);
    length
}

static sel_commit_bools_ops: file_operations = file_operations {
    open: None,
    read: None,
    write: Some(sel_commit_bools_write),
    mmap: None,
    release: None,
    llseek: unsafe { Some(generic_file_llseek) },
};

unsafe extern "C" fn sel_make_bools(newpolicy: *mut selinux_policy, bool_dir: *mut dentry, bool_num: *mut c_uint, bool_pending_names: *mut *mut *mut c_char, bool_pending_values: *mut *mut c_int) -> c_int {
    let mut names: *mut *mut c_char = null_mut();
    let mut num: u32_t = 0;
    let page = kzalloc(PAGE_SIZE, GFP_KERNEL) as *mut c_char;
    if page.is_null() { return -ENOMEM; }
    let mut ret = security_get_bools(newpolicy, &mut num, &mut names, bool_pending_values);
    if ret == 0 {
        *bool_num = num;
        *bool_pending_names = names;
        let mut i: u32_t = 0;
        while ret == 0 && i < num {
            let len = snprintf(page, PAGE_SIZE, c"/%s/%s".as_ptr(), BOOL_DIR_NAME.as_ptr(), *names.add(i as usize));
            if len as usize >= PAGE_SIZE {
                ret = -ENAMETOOLONG;
                break;
            }
            let inodep = sel_make_inode((*bool_dir).d_sb, S_IFREG | S_IRUGO | S_IWUSR);
            if inodep.is_null() {
                ret = -ENOMEM;
                break;
            }
            let isec = selinux_inode(inodep);
            let mut sid: u32_t = 0;
            ret = selinux_policy_genfs_sid(newpolicy, c"selinuxfs".as_ptr(), page, SECCLASS_FILE, &mut sid);
            if ret != 0 {
                pr_warn_ratelimited(c"SELinux: no sid found, defaulting to security isid for %s\n".as_ptr(), page);
                sid = SECINITSID_SECURITY;
            }
            (*isec).sid = sid;
            (*isec).initialized = LABEL_INITIALIZED;
            (*inodep).i_fop = &sel_bool_ops;
            (*inodep).i_ino = i as ino_t | SEL_BOOL_INO_OFFSET;
            ret = sel_attach_file(bool_dir, *names.add(i as usize), inodep);
            i += 1;
        }
    }
    kfree(page as *mut c_void);
    ret
}

unsafe extern "C" fn sel_read_avc_cache_threshold(_filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut tmpbuf = [0 as c_char; TMPBUFLEN];
    let length = scnprintf(tmpbuf.as_mut_ptr(), TMPBUFLEN, c"%u".as_ptr(), avc_get_cache_threshold());
    simple_read_from_buffer(buf, count, ppos, tmpbuf.as_ptr() as *const c_void, length as size_t)
}

unsafe extern "C" fn sel_write_avc_cache_threshold(_file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut new_value: c_uint = 0;
    let mut ret = avc_has_perm(current_sid(), SECINITSID_SECURITY, SECCLASS_SECURITY, SECURITY__SETSECPARAM, null_mut()) as ssize_t;
    if ret != 0 { return ret; }
    if count >= PAGE_SIZE { return -ENOMEM as ssize_t; }
    /* No partial writes. */
    if *ppos != 0 { return -EINVAL as ssize_t; }
    let page = memdup_user_nul(buf, count);
    if IS_ERR(page) { return PTR_ERR(page) as ssize_t; }
    ret = -EINVAL as ssize_t;
    if sscanf(page, c"%u".as_ptr(), &mut new_value) == 1 {
        avc_set_cache_threshold(new_value);
        ret = count as ssize_t;
    }
    kfree(page as *mut c_void);
    ret
}

unsafe extern "C" fn sel_read_avc_hash_stats(_filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let page = kmalloc(PAGE_SIZE, GFP_KERNEL) as *mut c_char;
    if page.is_null() { return -ENOMEM as ssize_t; }
    let mut length = avc_get_hash_stats(page);
    if length >= 0 {
        length = simple_read_from_buffer(buf, count, ppos, page as *const c_void, length as size_t);
    }
    kfree(page as *mut c_void);
    length
}

unsafe extern "C" fn sel_read_sidtab_hash_stats(_filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let page = kmalloc(PAGE_SIZE, GFP_KERNEL) as *mut c_char;
    if page.is_null() { return -ENOMEM as ssize_t; }
    let mut length = security_sidtab_hash_stats(page);
    if length >= 0 {
        length = simple_read_from_buffer(buf, count, ppos, page as *const c_void, length as size_t);
    }
    kfree(page as *mut c_void);
    length
}

static sel_sidtab_hash_stats_ops: file_operations = file_operations { open: None, read: Some(sel_read_sidtab_hash_stats), write: None, mmap: None, release: None, llseek: unsafe { Some(generic_file_llseek) } };
static sel_avc_cache_threshold_ops: file_operations = file_operations { open: None, read: Some(sel_read_avc_cache_threshold), write: Some(sel_write_avc_cache_threshold), mmap: None, release: None, llseek: unsafe { Some(generic_file_llseek) } };
static sel_avc_hash_stats_ops: file_operations = file_operations { open: None, read: Some(sel_read_avc_hash_stats), write: None, mmap: None, release: None, llseek: unsafe { Some(generic_file_llseek) } };

/* CONFIG_SECURITY_SELINUX_AVC_STATS */
unsafe extern "C" fn sel_avc_get_stat_idx(idx: *mut loff_t) -> *mut avc_cache_stats {
    let mut cpu = *idx;
    while cpu < nr_cpu_ids {
        if cpu_possible(cpu) {
            *idx = cpu + 1;
            return &raw mut avc_cache_stats;
        }
        cpu += 1;
    }
    *idx += 1;
    null_mut()
}

unsafe extern "C" fn sel_avc_stats_seq_start(_seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut n = *pos - 1;
    if *pos == 0 { return SEQ_START_TOKEN; }
    sel_avc_get_stat_idx(&mut n) as *mut c_void
}

unsafe extern "C" fn sel_avc_stats_seq_next(_seq: *mut seq_file, _v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    sel_avc_get_stat_idx(pos) as *mut c_void
}

unsafe extern "C" fn sel_avc_stats_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let st = v as *mut avc_cache_stats;
    if v == SEQ_START_TOKEN {
        seq_puts(seq, c"lookups hits misses allocations reclaims frees\n".as_ptr());
    } else {
        let lookups = (*st).lookups;
        let misses = (*st).misses;
        let hits = lookups.wrapping_sub(misses);
        seq_printf(seq, c"%u %u %u %u %u %u\n".as_ptr(), lookups, hits, misses, (*st).allocations, (*st).reclaims, (*st).frees);
    }
    0
}

unsafe extern "C" fn sel_avc_stats_seq_stop(_seq: *mut seq_file, _v: *mut c_void) {}

static sel_avc_cache_stats_seq_ops: seq_operations = seq_operations {
    start: Some(sel_avc_stats_seq_start),
    next: Some(sel_avc_stats_seq_next),
    stop: Some(sel_avc_stats_seq_stop),
    show: Some(sel_avc_stats_seq_show),
};

unsafe extern "C" fn sel_open_avc_cache_stats(_inode: *mut inode, filep: *mut file) -> c_int {
    seq_open(filep, &sel_avc_cache_stats_seq_ops)
}

static sel_avc_cache_stats_ops: file_operations = file_operations {
    open: Some(sel_open_avc_cache_stats),
    read: unsafe { Some(seq_read) },
    write: None,
    mmap: None,
    release: unsafe { Some(seq_release) },
    llseek: unsafe { Some(seq_lseek) },
};

unsafe extern "C" fn sel_make_avc_files(dir: *mut dentry) -> c_int {
    let sb = (*dir).d_sb;
    let fsi = (*sb).s_fs_info as *mut selinux_fs_info;
    let files = [
        tree_descr { name: c"cache_threshold".as_ptr(), ops: &sel_avc_cache_threshold_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: c"hash_stats".as_ptr(), ops: &sel_avc_hash_stats_ops, mode: S_IRUGO },
        tree_descr { name: c"cache_stats".as_ptr(), ops: &sel_avc_cache_stats_ops, mode: S_IRUGO },
    ];
    let mut err = 0;
    let mut i = 0;
    while err == 0 && i < files.len() {
        let inodep = sel_make_inode((*dir).d_sb, S_IFREG | files[i].mode);
        if inodep.is_null() { return -ENOMEM; }
        (*inodep).i_fop = files[i].ops;
        (*fsi).last_ino += 1;
        (*inodep).i_ino = (*fsi).last_ino;
        err = sel_attach_file(dir, files[i].name, inodep);
        i += 1;
    }
    err
}

unsafe extern "C" fn sel_make_ss_files(dir: *mut dentry) -> c_int {
    let sb = (*dir).d_sb;
    let fsi = (*sb).s_fs_info as *mut selinux_fs_info;
    let files = [tree_descr { name: c"sidtab_hash_stats".as_ptr(), ops: &sel_sidtab_hash_stats_ops, mode: S_IRUGO }];
    let mut err = 0;
    let mut i = 0;
    while err == 0 && i < files.len() {
        let inodep = sel_make_inode((*dir).d_sb, S_IFREG | files[i].mode);
        if inodep.is_null() { return -ENOMEM; }
        (*inodep).i_fop = files[i].ops;
        (*fsi).last_ino += 1;
        (*inodep).i_ino = (*fsi).last_ino;
        err = sel_attach_file(dir, files[i].name, inodep);
        i += 1;
    }
    err
}

unsafe extern "C" fn sel_read_initcon(filep: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut con: *mut c_char = null_mut();
    let mut len: u32_t = 0;
    let sid = ((*file_inode(filep)).i_ino & SEL_INO_MASK) as u32_t;
    let mut ret = security_sid_to_context(sid, &mut con, &mut len) as ssize_t;
    if ret == 0 {
        ret = simple_read_from_buffer(buf, count, ppos, con as *const c_void, len as size_t);
        kfree(con as *mut c_void);
    }
    ret
}

static sel_initcon_ops: file_operations = file_operations { open: None, read: Some(sel_read_initcon), write: None, mmap: None, release: None, llseek: unsafe { Some(generic_file_llseek) } };

unsafe extern "C" fn sel_make_initcon_files(dir: *mut dentry) -> c_int {
    let mut err = 0;
    let mut i: c_uint = 1;
    while err == 0 && i <= SECINITSID_NUM {
        let s = security_get_initial_sid_context(i);
        if !s.is_null() {
            let inodep = sel_make_inode((*dir).d_sb, S_IFREG | S_IRUGO);
            if inodep.is_null() { return -ENOMEM; }
            (*inodep).i_fop = &sel_initcon_ops;
            (*inodep).i_ino = i as ino_t | SEL_INITCON_INO_OFFSET;
            err = sel_attach_file(dir, s, inodep);
        }
        i += 1;
    }
    err
}

unsafe fn sel_class_to_ino(class: u16_t) -> c_ulong {
    (class as c_ulong * (SEL_VEC_MAX + 1)) | SEL_CLASS_INO_OFFSET
}

unsafe fn sel_ino_to_class(ino: c_ulong) -> u16_t {
    ((ino & SEL_INO_MASK) / (SEL_VEC_MAX + 1)) as u16_t
}

unsafe fn sel_perm_to_ino(class: u16_t, perm: u32_t) -> c_ulong {
    (class as c_ulong * (SEL_VEC_MAX + 1) + perm as c_ulong) | SEL_CLASS_INO_OFFSET
}

unsafe fn sel_ino_to_perm(ino: c_ulong) -> u32_t {
    ((ino & SEL_INO_MASK) % (SEL_VEC_MAX + 1)) as u32_t
}

unsafe extern "C" fn sel_read_class(filep: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let ino = (*file_inode(filep)).i_ino;
    let mut res = [0 as c_char; TMPBUFLEN];
    let len = scnprintf(res.as_mut_ptr(), res.len(), c"%d".as_ptr(), sel_ino_to_class(ino) as c_int);
    simple_read_from_buffer(buf, count, ppos, res.as_ptr() as *const c_void, len as size_t)
}

static sel_class_ops: file_operations = file_operations { open: None, read: Some(sel_read_class), write: None, mmap: None, release: None, llseek: unsafe { Some(generic_file_llseek) } };

unsafe extern "C" fn sel_read_perm(filep: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let ino = (*file_inode(filep)).i_ino;
    let mut res = [0 as c_char; TMPBUFLEN];
    let len = scnprintf(res.as_mut_ptr(), res.len(), c"%d".as_ptr(), sel_ino_to_perm(ino) as c_int);
    simple_read_from_buffer(buf, count, ppos, res.as_ptr() as *const c_void, len as size_t)
}

static sel_perm_ops: file_operations = file_operations { open: None, read: Some(sel_read_perm), write: None, mmap: None, release: None, llseek: unsafe { Some(generic_file_llseek) } };

unsafe extern "C" fn sel_read_policycap(filep: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut tmpbuf = [0 as c_char; TMPBUFLEN];
    let i_ino = (*file_inode(filep)).i_ino;
    let value = security_policycap_supported(i_ino & SEL_INO_MASK);
    let length = scnprintf(tmpbuf.as_mut_ptr(), TMPBUFLEN, c"%d".as_ptr(), value);
    simple_read_from_buffer(buf, count, ppos, tmpbuf.as_ptr() as *const c_void, length as size_t)
}

static sel_policycap_ops: file_operations = file_operations { open: None, read: Some(sel_read_policycap), write: None, mmap: None, release: None, llseek: unsafe { Some(generic_file_llseek) } };

unsafe extern "C" fn sel_make_perm_files(newpolicy: *mut selinux_policy, objclass: *mut c_char, classvalue: c_int, dir: *mut dentry) -> c_int {
    let mut nperms: u32_t = 0;
    let mut perms: *mut *mut c_char = null_mut();
    let mut rc = security_get_permissions(newpolicy, objclass, &mut perms, &mut nperms);
    if rc == 0 {
        let mut i = 0;
        while rc == 0 && i < nperms {
            let inodep = sel_make_inode((*dir).d_sb, S_IFREG | S_IRUGO);
            if inodep.is_null() {
                rc = -ENOMEM;
                break;
            }
            (*inodep).i_fop = &sel_perm_ops;
            /* i+1 since perm values are 1-indexed */
            (*inodep).i_ino = sel_perm_to_ino(classvalue as u16_t, i + 1);
            rc = sel_attach_file(dir, *perms.add(i as usize), inodep);
            i += 1;
        }
    }
    let mut i = 0;
    while i < nperms {
        kfree(*perms.add(i as usize) as *mut c_void);
        i += 1;
    }
    kfree(perms as *mut c_void);
    rc
}

unsafe extern "C" fn sel_make_class_dir_entries(newpolicy: *mut selinux_policy, classname: *mut c_char, index: c_int, dir: *mut dentry) -> c_int {
    let sb = (*dir).d_sb;
    let fsi = (*sb).s_fs_info as *mut selinux_fs_info;
    let inodep = sel_make_inode((*dir).d_sb, S_IFREG | S_IRUGO);
    if inodep.is_null() { return -ENOMEM; }
    (*inodep).i_fop = &sel_class_ops;
    (*inodep).i_ino = sel_class_to_ino(index as u16_t);
    let err = sel_attach_file(dir, c"index".as_ptr(), inodep);
    if err != 0 { return err; }
    let dentryp = sel_make_dir(dir, c"perms".as_ptr(), &mut (*fsi).last_class_ino);
    if IS_ERR(dentryp) { return PTR_ERR(dentryp); }
    sel_make_perm_files(newpolicy, classname, index, dentryp)
}

unsafe extern "C" fn sel_make_classes(newpolicy: *mut selinux_policy, class_dir: *mut dentry, last_class_ino: *mut c_ulong) -> c_int {
    let mut nclasses: u32_t = 0;
    let mut classes: *mut *mut c_char = null_mut();
    let mut rc = security_get_classes(newpolicy, &mut classes, &mut nclasses);
    if rc != 0 { return rc; }
    /* +2 since classes are 1-indexed */
    *last_class_ino = sel_class_to_ino((nclasses + 2) as u16_t);
    let mut i = 0;
    while i < nclasses {
        let class_name_dir = sel_make_dir(class_dir, *classes.add(i as usize), last_class_ino);
        if IS_ERR(class_name_dir) {
            rc = PTR_ERR(class_name_dir);
            break;
        }
        /* i+1 since class values are 1-indexed */
        rc = sel_make_class_dir_entries(newpolicy, *classes.add(i as usize), (i + 1) as c_int, class_name_dir);
        if rc != 0 { break; }
        i += 1;
    }
    let mut j = 0;
    while j < nclasses {
        kfree(*classes.add(j as usize) as *mut c_void);
        j += 1;
    }
    kfree(classes as *mut c_void);
    rc
}

unsafe extern "C" fn sel_make_policycap(dir: *mut dentry) -> c_int {
    let sb = (*dir).d_sb;
    let mut err = 0;
    let mut iter: c_uint = 0;
    while err == 0 && iter <= POLICYDB_CAP_MAX {
        let name = c"unknown".as_ptr();
        let inodep = sel_make_inode(sb, S_IFREG | 0o444);
        if inodep.is_null() { return -ENOMEM; }
        (*inodep).i_fop = &sel_policycap_ops;
        (*inodep).i_ino = iter as ino_t | SEL_POLICYCAP_INO_OFFSET;
        err = sel_attach_file(dir, name, inodep);
        iter += 1;
    }
    err
}

unsafe extern "C" fn sel_make_dir(dir: *mut dentry, name: *const c_char, ino: *mut c_ulong) -> *mut dentry {
    let inodep = sel_make_inode((*dir).d_sb, S_IFDIR | S_IRUGO | S_IXUGO);
    if inodep.is_null() { return ERR_PTR(-ENOMEM); }
    (*inodep).i_op = &raw const simple_dir_inode_operations;
    (*inodep).i_fop = &raw const simple_dir_operations;
    *ino += 1;
    (*inodep).i_ino = *ino;
    /* directory inodes start off with i_nlink == 2 (for "." entry) */
    inc_nlink(inodep);
    /* bump link count on parent directory, too */
    inc_nlink(d_inode(dir));
    sel_attach(dir, name, inodep)
}

unsafe extern "C" fn reject_all(_idmap: *mut mnt_idmap, _inode: *mut inode, _mask: c_int) -> c_int {
    -EPERM /* no access for anyone, root or no root. */
}

static swapover_dir_inode_operations: inode_operations = inode_operations {
    lookup: unsafe { Some(simple_lookup) },
    permission: Some(reject_all),
};

unsafe extern "C" fn sel_make_swapover_dir(sb: *mut super_block, ino: *mut c_ulong) -> *mut dentry {
    let inodep = sel_make_inode(sb, S_IFDIR);
    if inodep.is_null() { return ERR_PTR(-ENOMEM); }
    let dentryp = simple_start_creating((*sb).s_root, c".swapover".as_ptr());
    if IS_ERR(dentryp) {
        iput(inodep);
        return dentryp;
    }
    (*inodep).i_op = &swapover_dir_inode_operations;
    *ino += 1;
    (*inodep).i_ino = *ino;
    /* directory inodes start off with i_nlink == 2 (for "." entry) */
    inc_nlink(inodep);
    d_make_persistent(dentryp, inodep);
    inc_nlink((*(*sb).s_root).d_inode);
    simple_done_creating(dentryp);
    dentryp /* borrowed */
}

unsafe extern "C" fn sel_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int {
    let _ = fc;
    let selinux_files = [
        tree_descr { name: c"".as_ptr(), ops: null(), mode: 0 },
        tree_descr { name: c"load".as_ptr(), ops: &sel_load_ops, mode: S_IRUSR | S_IWUSR },
        tree_descr { name: c"enforce".as_ptr(), ops: &sel_enforce_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: c"context".as_ptr(), ops: &transaction_ops, mode: S_IRUGO | S_IWUGO },
        tree_descr { name: c"access".as_ptr(), ops: &transaction_ops, mode: S_IRUGO | S_IWUGO },
        tree_descr { name: c"create".as_ptr(), ops: &transaction_ops, mode: S_IRUGO | S_IWUGO },
        tree_descr { name: c"relabel".as_ptr(), ops: &transaction_ops, mode: S_IRUGO | S_IWUGO },
        tree_descr { name: c"user".as_ptr(), ops: &transaction_ops, mode: S_IRUGO | S_IWUGO },
        tree_descr { name: c"policyvers".as_ptr(), ops: &sel_policyvers_ops, mode: S_IRUGO },
        tree_descr { name: c"commit_pending_bools".as_ptr(), ops: &sel_commit_bools_ops, mode: S_IWUSR },
        tree_descr { name: c"mls".as_ptr(), ops: &sel_mls_ops, mode: S_IRUGO },
        tree_descr { name: c"disable".as_ptr(), ops: &sel_disable_ops, mode: S_IWUSR },
        tree_descr { name: c"member".as_ptr(), ops: &transaction_ops, mode: S_IRUGO | S_IWUGO },
        tree_descr { name: c"checkreqprot".as_ptr(), ops: &sel_checkreqprot_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: c"reject_unknown".as_ptr(), ops: &sel_handle_unknown_ops, mode: S_IRUGO },
        tree_descr { name: c"deny_unknown".as_ptr(), ops: &sel_handle_unknown_ops, mode: S_IRUGO },
        tree_descr { name: c"status".as_ptr(), ops: &sel_handle_status_ops, mode: S_IRUGO },
        tree_descr { name: c"policy".as_ptr(), ops: &sel_policy_ops, mode: S_IRUGO },
        tree_descr { name: c"validatetrans".as_ptr(), ops: &sel_transition_ops, mode: S_IWUGO },
        tree_descr { name: c"".as_ptr(), ops: null(), mode: 0 },
    ];
    let mut ret = selinux_fs_info_create(sb);
    if ret != 0 { return sel_fill_super_err(ret); }
    ret = simple_fill_super(sb, SELINUX_MAGIC, selinux_files.as_ptr());
    if ret != 0 { return sel_fill_super_err(ret); }
    let fsi = (*sb).s_fs_info as *mut selinux_fs_info;
    (*fsi).bool_dir = sel_make_dir((*sb).s_root, BOOL_DIR_NAME.as_ptr() as *const c_char, &mut (*fsi).last_ino);
    if IS_ERR((*fsi).bool_dir) {
        ret = PTR_ERR((*fsi).bool_dir);
        (*fsi).bool_dir = null_mut();
        return sel_fill_super_err(ret);
    }
    let inodep = sel_make_inode(sb, S_IFCHR | S_IRUGO | S_IWUGO);
    if inodep.is_null() { return sel_fill_super_err(-ENOMEM); }
    (*fsi).last_ino += 1;
    (*inodep).i_ino = (*fsi).last_ino;
    let isec = selinux_inode(inodep);
    (*isec).sid = SECINITSID_DEVNULL;
    (*isec).sclass = SECCLASS_CHR_FILE;
    (*isec).initialized = LABEL_INITIALIZED;
    init_special_inode(inodep, S_IFCHR | S_IRUGO | S_IWUGO, MKDEV(MEM_MAJOR, 3));
    ret = sel_attach_file((*sb).s_root, NULL_FILE_NAME.as_ptr() as *const c_char, inodep);
    if ret != 0 { return sel_fill_super_err(ret); }
    let mut dentryp = sel_make_dir((*sb).s_root, c"avc".as_ptr(), &mut (*fsi).last_ino);
    if IS_ERR(dentryp) { return sel_fill_super_err(PTR_ERR(dentryp)); }
    ret = sel_make_avc_files(dentryp);
    if ret != 0 { return sel_fill_super_err(ret); }
    dentryp = sel_make_dir((*sb).s_root, c"ss".as_ptr(), &mut (*fsi).last_ino);
    if IS_ERR(dentryp) { return sel_fill_super_err(PTR_ERR(dentryp)); }
    ret = sel_make_ss_files(dentryp);
    if ret != 0 { return sel_fill_super_err(ret); }
    dentryp = sel_make_dir((*sb).s_root, c"initial_contexts".as_ptr(), &mut (*fsi).last_ino);
    if IS_ERR(dentryp) { return sel_fill_super_err(PTR_ERR(dentryp)); }
    ret = sel_make_initcon_files(dentryp);
    if ret != 0 { return sel_fill_super_err(ret); }
    (*fsi).class_dir = sel_make_dir((*sb).s_root, CLASS_DIR_NAME.as_ptr() as *const c_char, &mut (*fsi).last_ino);
    if IS_ERR((*fsi).class_dir) {
        ret = PTR_ERR((*fsi).class_dir);
        (*fsi).class_dir = null_mut();
        return sel_fill_super_err(ret);
    }
    dentryp = sel_make_dir((*sb).s_root, c"policy_capabilities".as_ptr(), &mut (*fsi).last_ino);
    if IS_ERR(dentryp) { return sel_fill_super_err(PTR_ERR(dentryp)); }
    ret = sel_make_policycap(dentryp);
    if ret != 0 {
        pr_err(c"SELinux: failed to load policy capabilities\n".as_ptr());
        return sel_fill_super_err(ret);
    }
    0
}

unsafe fn sel_fill_super_err(ret: c_int) -> c_int {
    pr_err(c"SELinux: %s:  failed while creating inodes\n".as_ptr(), c"sel_fill_super".as_ptr());
    ret
}

unsafe extern "C" fn sel_get_tree(fc: *mut fs_context) -> c_int {
    get_tree_single(fc, sel_fill_super)
}

static sel_context_ops: fs_context_operations = fs_context_operations {
    get_tree: Some(sel_get_tree),
};

unsafe extern "C" fn sel_init_fs_context(fc: *mut fs_context) -> c_int {
    (*fc).ops = &sel_context_ops;
    0
}

unsafe extern "C" fn sel_kill_sb(sb: *mut super_block) {
    let fsi = (*sb).s_fs_info as *mut selinux_fs_info;
    kill_anon_super(sb);
    selinux_fs_info_free(fsi);
}

static mut sel_fs_type: file_system_type = file_system_type {
    name: c"selinuxfs".as_ptr(),
    init_fs_context: Some(sel_init_fs_context),
    kill_sb: Some(sel_kill_sb),
};

#[unsafe(no_mangle)]
pub static mut selinux_null: path = path {
    mnt: null_mut(),
    dentry: null_mut(),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_sel_fs() -> c_int {
    let null_name = qstr { name: NULL_FILE_NAME.as_ptr() as *const c_char };
    if !selinux_enabled_boot {
        return 0;
    }
    let mut err = sysfs_create_mount_point(fs_kobj, c"selinux".as_ptr());
    if err != 0 { return err; }
    err = register_filesystem(&raw mut sel_fs_type);
    if err != 0 {
        sysfs_remove_mount_point(fs_kobj, c"selinux".as_ptr());
        return err;
    }
    selinux_null.mnt = kern_mount(&raw mut sel_fs_type);
    if IS_ERR(selinux_null.mnt) {
        pr_err(c"selinuxfs:  could not mount!\n".as_ptr());
        err = PTR_ERR(selinux_null.mnt);
        selinux_null.mnt = null_mut();
        unregister_filesystem(&raw mut sel_fs_type);
        sysfs_remove_mount_point(fs_kobj, c"selinux".as_ptr());
        return err;
    }
    selinux_null.dentry = try_lookup_noperm(&null_name, (*selinux_null.mnt).mnt_root);
    if IS_ERR(selinux_null.dentry) {
        pr_err(c"selinuxfs:  could not lookup null!\n".as_ptr());
        err = PTR_ERR(selinux_null.dentry);
        selinux_null.dentry = null_mut();
        kern_unmount(selinux_null.mnt);
        selinux_null.mnt = null_mut();
        unregister_filesystem(&raw mut sel_fs_type);
        sysfs_remove_mount_point(fs_kobj, c"selinux".as_ptr());
        return err;
    }
    /*
     * Try to pre-allocate the status page, so the sequence number of the
     * initial policy load can be stored.
     */
    let _ = selinux_kernel_status_page();
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
