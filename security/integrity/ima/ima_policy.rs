// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008 IBM Corporation
 * Author: Mimi Zohar <zohar@us.ibm.com>
 *
 * ima_policy.c
 *	- initialize default measure policy rules
 *
 * Rust source-level translation of integrity/ima/ima_policy.c.
 * Kernel includes and build attributes are represented by extern declarations
 * and comments where they cannot be mapped file-locally.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_t = bool;
type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type gfp_t = u32;
type uid_t = u32;
type gid_t = u32;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uuid_t {
    pub b: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kuid_t {
    pub val: uid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kgid_t {
    pub val: gid_t,
}

pub type vfsuid_t = kuid_t;
pub type vfsgid_t = kgid_t;

#[repr(C)]
pub struct ima_template_field {
    pub field_id: *const c_char,
}

#[repr(C)]
pub struct ima_template_desc {
    pub name: *const c_char,
    pub fmt: *const c_char,
    pub fields: *mut *mut ima_template_field,
    pub num_fields: c_int,
}

#[repr(C)]
pub struct ima_iint_cache {
    pub measured_pcrs: [u8; 0],
}

#[repr(C)]
pub struct ima_rule_opt_list {
    pub count: size_t,
    pub items: [*mut c_char; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_lsm_rule {
    pub rule: *mut c_void,
    pub args_p: *mut c_char,
    pub type_: c_int,
}

#[repr(C)]
pub struct ima_rule_entry {
    pub list: list_head,
    pub action: c_int,
    pub flags: u32,
    pub func: ima_hooks,
    pub mask: c_int,
    pub fsmagic: c_ulong,
    pub fsuuid: uuid_t,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub fowner: kuid_t,
    pub fgroup: kgid_t,
    pub uid_op: Option<unsafe extern "C" fn(kuid_t, kuid_t) -> bool_t>,
    pub gid_op: Option<unsafe extern "C" fn(kgid_t, kgid_t) -> bool_t>,
    pub fowner_op: Option<unsafe extern "C" fn(vfsuid_t, kuid_t) -> bool_t>,
    pub fgroup_op: Option<unsafe extern "C" fn(vfsgid_t, kgid_t) -> bool_t>,
    pub pcr: c_int,
    pub allowed_algos: u32,
    pub lsm: [ima_lsm_rule; MAX_LSM_RULES],
    pub fsname: *mut c_char,
    pub fs_subtype: *mut c_char,
    pub keyrings: *mut ima_rule_opt_list,
    pub label: *mut ima_rule_opt_list,
    pub template: *mut ima_template_desc,
}

#[repr(C)]
pub struct super_block_type {
    pub name: *const c_char,
}

#[repr(C)]
pub struct super_block {
    pub s_magic: c_ulong,
    pub s_type: *mut super_block_type,
    pub s_subtype: *const c_char,
    pub s_uuid: uuid_t,
}

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
}

#[repr(C)]
pub struct cred {
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub suid: kuid_t,
    pub sgid: kgid_t,
    pub euid: kuid_t,
    pub egid: kgid_t,
}

#[repr(C)]
pub struct mnt_idmap;
#[repr(C)]
pub struct lsm_prop;
#[repr(C)]
pub struct notifier_block;
#[repr(C)]
pub struct audit_buffer;
#[repr(C)]
pub struct seq_file {
    pub buf: *mut c_char,
    pub size: size_t,
    pub from: size_t,
    pub count: size_t,
    pub pad_until: size_t,
    pub read_pos: loff_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ima_hooks {
    NONE = 0,
    FILE_CHECK,
    MMAP_CHECK,
    BPRM_CHECK,
    CREDS_CHECK,
    POST_SETATTR,
    MODULE_CHECK,
    FIRMWARE_CHECK,
    KEXEC_KERNEL_CHECK,
    KEXEC_INITRAMFS_CHECK,
    POLICY_CHECK,
    KEXEC_CMDLINE,
    KEY_CHECK,
    CRITICAL_DATA,
    SETXATTR_CHECK,
    MMAP_CHECK_REQPROT,
    MAX_CHECK,
}

pub const IMA_FUNC: u32 = 0x0001;
pub const IMA_MASK: u32 = 0x0002;
pub const IMA_FSMAGIC: u32 = 0x0004;
pub const IMA_UID: u32 = 0x0008;
pub const IMA_FOWNER: u32 = 0x0010;
pub const IMA_FSUUID: u32 = 0x0020;
pub const IMA_INMASK: u32 = 0x0040;
pub const IMA_EUID: u32 = 0x0080;
pub const IMA_PCR: u32 = 0x0100;
pub const IMA_FSNAME: u32 = 0x0200;
pub const IMA_KEYRINGS: u32 = 0x0400;
pub const IMA_LABEL: u32 = 0x0800;
pub const IMA_VALIDATE_ALGOS: u32 = 0x1000;
pub const IMA_GID: u32 = 0x2000;
pub const IMA_EGID: u32 = 0x4000;
pub const IMA_FGROUP: u32 = 0x8000;
pub const IMA_FS_SUBTYPE: u32 = 0x10000;

pub const UNKNOWN: c_int = 0;
pub const MEASURE: c_int = 0x0001;
pub const DONT_MEASURE: c_int = 0x0002;
pub const APPRAISE: c_int = 0x0004;
pub const DONT_APPRAISE: c_int = 0x0008;
pub const AUDIT: c_int = 0x0040;
pub const DONT_AUDIT: c_int = 0x0080;
pub const HASH: c_int = 0x0100;
pub const DONT_HASH: c_int = 0x0200;

pub const MAX_LSM_RULES: usize = 6;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lsm_rule_types {
    LSM_OBJ_USER,
    LSM_OBJ_ROLE,
    LSM_OBJ_TYPE,
    LSM_SUBJ_USER,
    LSM_SUBJ_ROLE,
    LSM_SUBJ_TYPE,
}

pub const ORIGINAL_TCB: c_int = 1;
pub const DEFAULT_TCB: c_int = 2;
pub const IMA_DEFAULT_POLICY: c_int = 1;
pub const IMA_CUSTOM_POLICY: c_int = 2;

/* External constants supplied by the kernel headers and other IMA files. */
extern "C" {
    static GLOBAL_ROOT_UID: kuid_t;
    static INVALID_UID: kuid_t;
    static INVALID_GID: kgid_t;
    static mut current: *mut c_void;
    static mut ima_appraise: c_int;
    static ima_write_mutex: c_void;
    static hash_algo_name: [*const c_char; 0];
    static read_idmap: [ima_hooks; 0];
}

extern "C" {
    fn __vfsuid_val(v: vfsuid_t) -> uid_t;
    fn __vfsgid_val(v: vfsgid_t) -> gid_t;
    fn __kuid_val(v: kuid_t) -> uid_t;
    fn __kgid_val(v: kgid_t) -> gid_t;
    fn uid_eq(a: kuid_t, b: kuid_t) -> bool_t;
    fn gid_eq(a: kgid_t, b: kgid_t) -> bool_t;
    fn uid_gt(a: kuid_t, b: kuid_t) -> bool_t;
    fn gid_gt(a: kgid_t, b: kgid_t) -> bool_t;
    fn uid_lt(a: kuid_t, b: kuid_t) -> bool_t;
    fn gid_lt(a: kgid_t, b: kgid_t) -> bool_t;
    fn vfsuid_eq_kuid(a: vfsuid_t, b: kuid_t) -> bool_t;
    fn vfsgid_eq_kgid(a: vfsgid_t, b: kgid_t) -> bool_t;
    fn uid_valid(uid: kuid_t) -> bool_t;
    fn gid_valid(gid: kgid_t) -> bool_t;
    fn make_kuid(ns: *mut c_void, uid: uid_t) -> kuid_t;
    fn make_kgid(ns: *mut c_void, gid: gid_t) -> kgid_t;
    fn current_user_ns() -> *mut c_void;
    fn i_uid_into_vfsuid(idmap: *mut mnt_idmap, inode: *mut inode) -> vfsuid_t;
    fn i_gid_into_vfsgid(idmap: *mut mnt_idmap, inode: *mut inode) -> vfsgid_t;
    fn has_capability_noaudit(task: *mut c_void, cap: c_int) -> bool_t;
    fn uuid_equal(a: *const uuid_t, b: *const uuid_t) -> bool_t;
    fn uuid_is_null(a: *const uuid_t) -> bool_t;
    fn uuid_parse(src: *const c_char, dst: *mut uuid_t) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strspn(s: *const c_char, accept: *const c_char) -> size_t;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strsep(s: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int;
    fn kstrtoint(s: *const c_char, base: c_uint, res: *mut c_int) -> c_int;
    fn kstrdup(s: *const c_char, flags: gfp_t) -> *mut c_char;
    fn match_strdup(src: *const substring_t) -> *mut c_char;
    fn match_token(s: *mut c_char, table: *const match_token_t, args: *mut substring_t) -> c_int;
    fn match_string(array: *const *const c_char, n: c_int, string: *const c_char) -> c_int;
    fn crypto_has_alg(name: *const c_char, type_: u32, mask: u32) -> bool_t;
    fn kmalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kzalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: size_t, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn atomic_cmpxchg(v: *mut atomic_t, old: c_int, new: c_int) -> c_int;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
    fn list_replace_rcu(old: *mut list_head, new: *mut list_head);
    fn list_splice_tail_init_rcu(list: *mut list_head, head: *mut list_head, sync: unsafe extern "C" fn());
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn synchronize_rcu();
    fn ima_filter_rule_free(rule: *mut c_void);
    fn ima_filter_rule_init(type_: c_int, op: c_int, rulestr: *mut c_char, rule: *mut *mut c_void, gfp: gfp_t) -> c_int;
    fn ima_filter_rule_match(prop: *mut lsm_prop, type_: c_int, op: c_int, rule: *mut c_void) -> c_int;
    fn security_inode_getlsmprop(inode: *mut inode, prop: *mut lsm_prop);
    fn arch_get_ima_policy() -> *mut *const c_char;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: size_t) -> c_int;
    fn ima_template_desc_current() -> *mut ima_template_desc;
    fn lookup_template_desc(name: *const c_char) -> *mut ima_template_desc;
    fn template_desc_init_fields(fmt: *const c_char, fields: *mut *mut *mut ima_template_field, num_fields: *mut c_int) -> c_int;
    fn audit_context() -> *mut c_void;
    fn integrity_audit_log_start(ctx: *mut c_void, gfp: gfp_t, type_: c_int) -> *mut audit_buffer;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_end(ab: *mut audit_buffer);
    fn integrity_audit_msg(type_: c_int, inode: *mut c_void, name: *const c_char, op: *const c_char, cause: *const c_char, result: c_int, info: c_int);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_notice(fmt: *const c_char, ...);
    fn pr_notice_once(fmt: *const c_char, ...);
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn seq_puts(m: *mut seq_file, s: *const c_char) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn seq_has_overflowed(m: *mut seq_file) -> bool_t;
    fn ima_process_queued_keys();
    fn ima_measure_critical_data(ns: *const c_char, event: *const c_char, buf: *const c_void, len: size_t, hash: bool_t, digest: *const c_void, digest_len: size_t);
    fn lockdep_assert_held(lock: *const c_void);
    fn security_locked_down(reason: c_int) -> c_int;
}

type c_uint = u32;

#[repr(C)]
pub struct substring_t {
    pub from: *mut c_char,
    pub to: *mut c_char,
}

#[repr(C)]
pub struct match_token_t {
    pub token: c_int,
    pub pattern: *const c_char,
}

pub const GFP_KERNEL: gfp_t = 0;
pub const GFP_ATOMIC: gfp_t = 0;
pub const ENOMEM: c_int = 12;
pub const EINVAL: c_int = 22;
pub const ESTALE: c_int = 116;
pub const E2BIG: c_int = 7;
pub const NOTIFY_DONE: c_int = 0;
pub const NOTIFY_OK: c_int = 1;
pub const LSM_POLICY_CHANGE: c_ulong = 1;
pub const Audit_equal: c_int = 0;
pub const AUDIT_INTEGRITY_POLICY_RULE: c_int = 0;
pub const AUDIT_INTEGRITY_STATUS: c_int = 0;
pub const AUDIT_INTEGRITY_PCR: c_int = 0;
pub const AUDIT_OBJ_USER: c_int = 0;
pub const AUDIT_OBJ_ROLE: c_int = 0;
pub const AUDIT_OBJ_TYPE: c_int = 0;
pub const AUDIT_SUBJ_USER: c_int = 0;
pub const AUDIT_SUBJ_ROLE: c_int = 0;
pub const AUDIT_SUBJ_TYPE: c_int = 0;
pub const CAP_SETUID: c_int = 0;
pub const CAP_SETGID: c_int = 0;
pub const LOCKDOWN_KEXEC: c_int = 0;
pub const READING_MAX_ID: c_int = 0;
pub const READING_KEXEC_IMAGE: c_int = 0;
pub const IMA_MEASURE: c_int = MEASURE;
pub const IMA_APPRAISE: c_int = APPRAISE;
pub const IMA_DO_MASK: c_int = 0x00ff;
pub const IMA_NONACTION_FLAGS: u32 = 0;
pub const IMA_HASH: c_int = 0x0100;
pub const IMA_FILE_APPRAISE: c_int = 0;
pub const IMA_MMAP_APPRAISE: c_int = 0;
pub const IMA_BPRM_APPRAISE: c_int = 0;
pub const IMA_CREDS_APPRAISE: c_int = 0;
pub const IMA_READ_APPRAISE: c_int = 0;
pub const IMA_FAIL_UNVERIFIABLE_SIGS: c_int = 0;
pub const IMA_APPRAISE_MODULES: c_int = 0;
pub const IMA_APPRAISE_FIRMWARE: c_int = 0;
pub const IMA_APPRAISE_POLICY: c_int = 0;
pub const IMA_APPRAISE_KEXEC: c_int = 0;
pub const IMA_APPRAISE_ENFORCE: c_int = 0;
pub const IMA_DIGSIG_REQUIRED: u32 = 0;
pub const IMA_MODSIG_ALLOWED: u32 = 0;
pub const IMA_CHECK_BLACKLIST: u32 = 0;
pub const IMA_PERMIT_DIRECTIO: u32 = 0;
pub const IMA_VERITY_REQUIRED: u32 = 0;
pub const IMA_SIGV3_REQUIRED: u32 = 0;
pub const MAY_EXEC: c_int = 1;
pub const MAY_WRITE: c_int = 2;
pub const MAY_READ: c_int = 4;
pub const MAY_APPEND: c_int = 8;
pub const HASH_ALGO__LAST: c_int = 0;
pub const MAX_OPT_ARGS: usize = 3;

/* Filesystem magic constants are provided by linux/magic.h. */
pub const PROC_SUPER_MAGIC: c_ulong = 0x9fa0;
pub const SYSFS_MAGIC: c_ulong = 0x62656572;
pub const DEBUGFS_MAGIC: c_ulong = 0x64626720;
pub const TMPFS_MAGIC: c_ulong = 0x01021994;
pub const DEVPTS_SUPER_MAGIC: c_ulong = 0x1cd1;
pub const BINFMTFS_MAGIC: c_ulong = 0x42494e4d;
pub const SECURITYFS_MAGIC: c_ulong = 0x73636673;
pub const SELINUX_MAGIC: c_ulong = 0xf97cff8c;
pub const SMACK_MAGIC: c_ulong = 0x43415d53;
pub const CGROUP_SUPER_MAGIC: c_ulong = 0x27e0eb;
pub const CGROUP2_SUPER_MAGIC: c_ulong = 0x63677270;
pub const NSFS_MAGIC: c_ulong = 0x6e736673;
pub const EFIVARFS_MAGIC: c_ulong = 0xde5e81e4;
pub const RAMFS_MAGIC: c_ulong = 0x858458f6;
pub const BINFMTFS_MAGIC_ALIAS: c_ulong = BINFMTFS_MAGIC;

static mut max_rule_len: size_t = 0;
pub static mut ima_policy_flag: c_int = 0;
static mut temp_ima_appraise: c_int = 0;
static mut build_ima_appraise: c_int = 0;
pub static mut ima_setxattr_allowed_hash_algorithms: atomic_t = atomic_t { counter: 0 };

unsafe extern "C" fn vfsuid_gt_kuid(vfsuid: vfsuid_t, kuid: kuid_t) -> bool_t {
    __vfsuid_val(vfsuid) > __kuid_val(kuid)
}

unsafe extern "C" fn vfsgid_gt_kgid(vfsgid: vfsgid_t, kgid: kgid_t) -> bool_t {
    __vfsgid_val(vfsgid) > __kgid_val(kgid)
}

unsafe extern "C" fn vfsuid_lt_kuid(vfsuid: vfsuid_t, kuid: kuid_t) -> bool_t {
    __vfsuid_val(vfsuid) < __kuid_val(kuid)
}

unsafe extern "C" fn vfsgid_lt_kgid(vfsgid: vfsgid_t, kgid: kgid_t) -> bool_t {
    __vfsgid_val(vfsgid) < __kgid_val(kgid)
}

const ZERO_LSM: ima_lsm_rule = ima_lsm_rule { rule: ptr::null_mut(), args_p: ptr::null_mut(), type_: 0 };
const ZERO_UUID: uuid_t = uuid_t { b: [0; 16] };
const ZERO_LIST: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
const ZERO_KUID: kuid_t = kuid_t { val: 0 };
const ZERO_KGID: kgid_t = kgid_t { val: 0 };

const fn rule(action: c_int, flags: u32, func: ima_hooks, mask: c_int, fsmagic: c_ulong) -> ima_rule_entry {
    ima_rule_entry {
        list: ZERO_LIST,
        action,
        flags,
        func,
        mask,
        fsmagic,
        fsuuid: ZERO_UUID,
        uid: ZERO_KUID,
        gid: ZERO_KGID,
        fowner: ZERO_KUID,
        fgroup: ZERO_KGID,
        uid_op: None,
        gid_op: None,
        fowner_op: None,
        fgroup_op: None,
        pcr: 0,
        allowed_algos: 0,
        lsm: [ZERO_LSM; MAX_LSM_RULES],
        fsname: ptr::null_mut(),
        fs_subtype: ptr::null_mut(),
        keyrings: ptr::null_mut(),
        label: ptr::null_mut(),
        template: ptr::null_mut(),
    }
}

static mut dont_measure_rules: [ima_rule_entry; 13] = [
    rule(DONT_MEASURE, IMA_FSMAGIC, ima_hooks::NONE, 0, PROC_SUPER_MAGIC),
    rule(DONT_MEASURE, IMA_FSMAGIC, ima_hooks::NONE, 0, SYSFS_MAGIC),
    rule(DONT_MEASURE, IMA_FSMAGIC, ima_hooks::NONE, 0, DEBUGFS_MAGIC),
    rule(DONT_MEASURE, IMA_FSMAGIC | IMA_FUNC, ima_hooks::FILE_CHECK, 0, TMPFS_MAGIC),
    rule(DONT_MEASURE, IMA_FSMAGIC, ima_hooks::NONE, 0, DEVPTS_SUPER_MAGIC),
    rule(DONT_MEASURE, IMA_FSMAGIC, ima_hooks::NONE, 0, BINFMTFS_MAGIC),
    rule(DONT_MEASURE, IMA_FSMAGIC, ima_hooks::NONE, 0, SECURITYFS_MAGIC),
    rule(DONT_MEASURE, IMA_FSMAGIC, ima_hooks::NONE, 0, SELINUX_MAGIC),
    rule(DONT_MEASURE, IMA_FSMAGIC, ima_hooks::NONE, 0, SMACK_MAGIC),
    rule(DONT_MEASURE, IMA_FSMAGIC, ima_hooks::NONE, 0, CGROUP_SUPER_MAGIC),
    rule(DONT_MEASURE, IMA_FSMAGIC, ima_hooks::NONE, 0, CGROUP2_SUPER_MAGIC),
    rule(DONT_MEASURE, IMA_FSMAGIC, ima_hooks::NONE, 0, NSFS_MAGIC),
    rule(DONT_MEASURE, IMA_FSMAGIC, ima_hooks::NONE, 0, EFIVARFS_MAGIC),
];

static mut original_measurement_rules: [ima_rule_entry; 5] = [
    rule(MEASURE, IMA_FUNC | IMA_MASK, ima_hooks::MMAP_CHECK, MAY_EXEC, 0),
    rule(MEASURE, IMA_FUNC | IMA_MASK, ima_hooks::BPRM_CHECK, MAY_EXEC, 0),
    rule(MEASURE, IMA_FUNC | IMA_MASK | IMA_UID, ima_hooks::FILE_CHECK, MAY_READ, 0),
    rule(MEASURE, IMA_FUNC, ima_hooks::MODULE_CHECK, 0, 0),
    rule(MEASURE, IMA_FUNC, ima_hooks::FIRMWARE_CHECK, 0, 0),
];

static mut default_measurement_rules: [ima_rule_entry; 7] = [
    rule(MEASURE, IMA_FUNC | IMA_MASK, ima_hooks::MMAP_CHECK, MAY_EXEC, 0),
    rule(MEASURE, IMA_FUNC | IMA_MASK, ima_hooks::BPRM_CHECK, MAY_EXEC, 0),
    rule(MEASURE, IMA_FUNC | IMA_INMASK | IMA_EUID, ima_hooks::FILE_CHECK, MAY_READ, 0),
    rule(MEASURE, IMA_FUNC | IMA_INMASK | IMA_UID, ima_hooks::FILE_CHECK, MAY_READ, 0),
    rule(MEASURE, IMA_FUNC, ima_hooks::MODULE_CHECK, 0, 0),
    rule(MEASURE, IMA_FUNC, ima_hooks::FIRMWARE_CHECK, 0, 0),
    rule(MEASURE, IMA_FUNC, ima_hooks::POLICY_CHECK, 0, 0),
];

/* CONFIG_IMA_WRITE_POLICY and CONFIG_IMA_APPRAISE_SIGNED_INIT select extra/default-appraise entries in C. */
static mut default_appraise_rules: [ima_rule_entry; 15] = [
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, PROC_SUPER_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, SYSFS_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, DEBUGFS_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, TMPFS_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, RAMFS_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, DEVPTS_SUPER_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, BINFMTFS_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, SECURITYFS_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, SELINUX_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, SMACK_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, NSFS_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, EFIVARFS_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, CGROUP_SUPER_MAGIC),
    rule(DONT_APPRAISE, IMA_FSMAGIC, ima_hooks::NONE, 0, CGROUP2_SUPER_MAGIC),
    rule(APPRAISE, IMA_FOWNER, ima_hooks::NONE, 0, 0),
];

/* Build-time appraise signature rules are conditionally populated in C. */
static mut build_appraise_rules: [ima_rule_entry; 0] = [];

static mut secure_boot_rules: [ima_rule_entry; 4] = [
    rule(APPRAISE, IMA_FUNC | IMA_DIGSIG_REQUIRED | IMA_MODSIG_ALLOWED | IMA_CHECK_BLACKLIST, ima_hooks::MODULE_CHECK, 0, 0),
    rule(APPRAISE, IMA_FUNC | IMA_DIGSIG_REQUIRED, ima_hooks::FIRMWARE_CHECK, 0, 0),
    rule(APPRAISE, IMA_FUNC | IMA_DIGSIG_REQUIRED, ima_hooks::KEXEC_KERNEL_CHECK, 0, 0),
    rule(APPRAISE, IMA_FUNC | IMA_DIGSIG_REQUIRED, ima_hooks::POLICY_CHECK, 0, 0),
];

static mut critical_data_rules: [ima_rule_entry; 1] = [
    rule(MEASURE, IMA_FUNC, ima_hooks::CRITICAL_DATA, 0, 0),
];

static mut arch_policy_entry: *mut ima_rule_entry = ptr::null_mut();
static mut ima_default_rules: list_head = ZERO_LIST;
static mut ima_policy_rules: list_head = ZERO_LIST;
static mut ima_temp_rules: list_head = ZERO_LIST;
static mut ima_rules: *mut list_head = unsafe { &raw mut ima_default_rules };
static mut ima_policy: c_int = 0;
static mut ima_use_appraise_tcb: bool_t = false;
static mut ima_use_secure_boot: bool_t = false;
static mut ima_use_critical_data: bool_t = false;
static mut ima_fail_unverifiable_sigs: bool_t = false;

unsafe fn INVALID_PCR(a: c_int) -> bool_t {
    a < 0 || a >= ((size_of::<[u8; 0]>()) * 8) as c_int
}

unsafe extern "C" fn default_measure_policy_setup(_str: *mut c_char) -> c_int {
    if ima_policy != 0 {
        return 1;
    }
    ima_policy = ORIGINAL_TCB;
    1
}

unsafe extern "C" fn policy_setup(mut str_: *mut c_char) -> c_int {
    let mut p: *mut c_char;
    while {
        p = strsep(&mut str_, c" |\n".as_ptr());
        !p.is_null()
    } {
        if *p == b' ' as c_char {
            continue;
        }
        if strcmp(p, c"tcb".as_ptr()) == 0 && ima_policy == 0 {
            ima_policy = DEFAULT_TCB;
        } else if strcmp(p, c"appraise_tcb".as_ptr()) == 0 {
            ima_use_appraise_tcb = true;
        } else if strcmp(p, c"secure_boot".as_ptr()) == 0 {
            ima_use_secure_boot = true;
        } else if strcmp(p, c"critical_data".as_ptr()) == 0 {
            ima_use_critical_data = true;
        } else if strcmp(p, c"fail_securely".as_ptr()) == 0 {
            ima_fail_unverifiable_sigs = true;
        } else {
            pr_err(c"policy \"%s\" not found".as_ptr(), p);
        }
    }
    1
}

unsafe extern "C" fn default_appraise_policy_setup(_str: *mut c_char) -> c_int {
    ima_use_appraise_tcb = true;
    1
}

unsafe fn ima_alloc_rule_opt_list(src: *const substring_t) -> *mut ima_rule_opt_list {
    let mut count: size_t = 0;
    let src_copy = match_strdup(src);
    if src_copy.is_null() {
        return (-ENOMEM as isize) as *mut ima_rule_opt_list;
    }
    let mut next = src_copy;
    let mut cur: *mut c_char;
    while {
        cur = strsep(&mut next, c"|".as_ptr());
        !cur.is_null()
    } {
        if *cur == 0 {
            kfree(src_copy as *const c_void);
            return (-EINVAL as isize) as *mut ima_rule_opt_list;
        }
        count += 1;
    }
    if count == 0 {
        kfree(src_copy as *const c_void);
        return (-EINVAL as isize) as *mut ima_rule_opt_list;
    }
    let bytes = size_of::<ima_rule_opt_list>() + count * size_of::<*mut c_char>();
    let opt_list = kzalloc(bytes, GFP_KERNEL) as *mut ima_rule_opt_list;
    if opt_list.is_null() {
        kfree(src_copy as *const c_void);
        return (-ENOMEM as isize) as *mut ima_rule_opt_list;
    }
    (*opt_list).count = count;
    cur = src_copy;
    for i in 0..count {
        *(*opt_list).items.as_ptr().add(i).cast_mut() = cur;
        cur = strchr(cur, 0).add(1);
    }
    opt_list
}

unsafe fn ima_free_rule_opt_list(opt_list: *mut ima_rule_opt_list) {
    if opt_list.is_null() {
        return;
    }
    if (*opt_list).count != 0 {
        kfree(*(*opt_list).items.as_ptr() as *const c_void);
        (*opt_list).count = 0;
    }
    kfree(opt_list as *const c_void);
}

unsafe fn ima_lsm_free_rule(entry: *mut ima_rule_entry) {
    for i in 0..MAX_LSM_RULES {
        ima_filter_rule_free((*entry).lsm[i].rule);
        kfree((*entry).lsm[i].args_p as *const c_void);
    }
}

unsafe fn ima_free_rule(entry: *mut ima_rule_entry) {
    if entry.is_null() {
        return;
    }
    kfree((*entry).fsname as *const c_void);
    kfree((*entry).fs_subtype as *const c_void);
    ima_free_rule_opt_list((*entry).keyrings);
    ima_lsm_free_rule(entry);
    kfree(entry as *const c_void);
}

unsafe fn ima_lsm_copy_rule(entry: *mut ima_rule_entry, gfp: gfp_t) -> *mut ima_rule_entry {
    let nentry = kmemdup(entry as *const c_void, size_of::<ima_rule_entry>(), gfp) as *mut ima_rule_entry;
    if nentry.is_null() {
        return ptr::null_mut();
    }
    memset((*nentry).lsm.as_mut_ptr() as *mut c_void, 0, size_of::<[ima_lsm_rule; MAX_LSM_RULES]>());
    for i in 0..MAX_LSM_RULES {
        if (*entry).lsm[i].args_p.is_null() {
            continue;
        }
        (*nentry).lsm[i].type_ = (*entry).lsm[i].type_;
        (*nentry).lsm[i].args_p = (*entry).lsm[i].args_p;
        ima_filter_rule_init((*nentry).lsm[i].type_, Audit_equal, (*nentry).lsm[i].args_p, &mut (*nentry).lsm[i].rule, gfp);
        if (*nentry).lsm[i].rule.is_null() {
            pr_warn(c"rule for LSM '%s' is undefined\n".as_ptr(), (*nentry).lsm[i].args_p);
        }
    }
    nentry
}

unsafe fn ima_lsm_update_rule(entry: *mut ima_rule_entry) -> c_int {
    let nentry = ima_lsm_copy_rule(entry, GFP_KERNEL);
    if nentry.is_null() {
        return -ENOMEM;
    }
    list_replace_rcu(&mut (*entry).list, &mut (*nentry).list);
    synchronize_rcu();
    for i in 0..MAX_LSM_RULES {
        ima_filter_rule_free((*entry).lsm[i].rule);
    }
    kfree(entry as *const c_void);
    0
}

unsafe fn ima_rule_contains_lsm_cond(entry: *mut ima_rule_entry) -> bool_t {
    for i in 0..MAX_LSM_RULES {
        if !(*entry).lsm[i].args_p.is_null() {
            return true;
        }
    }
    false
}

unsafe fn ima_lsm_update_rules() {
    let mut pos = ima_policy_rules.next;
    while pos != &raw mut ima_policy_rules {
        let entry = pos as *mut ima_rule_entry;
        pos = (*pos).next;
        if !ima_rule_contains_lsm_cond(entry) {
            continue;
        }
        let result = ima_lsm_update_rule(entry);
        if result != 0 {
            pr_err(c"lsm rule update error %d\n".as_ptr(), result);
            return;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ima_lsm_policy_change(_nb: *mut notifier_block, event: c_ulong, _lsm_data: *mut c_void) -> c_int {
    if event != LSM_POLICY_CHANGE {
        return NOTIFY_DONE;
    }
    ima_lsm_update_rules();
    NOTIFY_OK
}

unsafe fn ima_match_rule_data(rule: *mut ima_rule_entry, func_data: *const c_char, cred: *const cred) -> bool_t {
    let mut opt_list: *const ima_rule_opt_list = ptr::null();
    let mut matched = false;
    if ((*rule).flags & IMA_UID) != 0 && !((*rule).uid_op.unwrap())((*cred).uid, (*rule).uid) {
        return false;
    }
    match (*rule).func {
        ima_hooks::KEY_CHECK => {
            if (*rule).keyrings.is_null() {
                return true;
            }
            opt_list = (*rule).keyrings;
        }
        ima_hooks::CRITICAL_DATA => {
            if (*rule).label.is_null() {
                return true;
            }
            opt_list = (*rule).label;
        }
        ima_hooks::POLICY_CHECK => return true,
        _ => return false,
    }
    if func_data.is_null() {
        return false;
    }
    for i in 0..(*opt_list).count {
        if strcmp(*(*opt_list).items.as_ptr().add(i), func_data) == 0 {
            matched = true;
            break;
        }
    }
    matched
}

unsafe fn ima_match_rules(rule: *mut ima_rule_entry, idmap: *mut mnt_idmap, inode: *mut inode, cred: *const cred, prop: *mut lsm_prop, func: ima_hooks, mask: c_int, func_data: *const c_char) -> bool_t {
    let mut result = false;
    let mut lsm_rule = rule;
    let mut rule_reinitialized = false;
    if ((*rule).flags & IMA_FUNC) != 0 && ((*rule).func != func && func != ima_hooks::POST_SETATTR) {
        return false;
    }
    match func {
        ima_hooks::POLICY_CHECK => {
            if !inode.is_null() {}
            else if (*rule).func == func && ima_match_rule_data(rule, func_data, cred) { return true; } else { return false; }
        }
        ima_hooks::KEY_CHECK | ima_hooks::CRITICAL_DATA => return (*rule).func == func && ima_match_rule_data(rule, func_data, cred),
        _ => {}
    }
    if ((*rule).flags & IMA_MASK) != 0 && ((*rule).mask != mask && func != ima_hooks::POST_SETATTR) { return false; }
    if ((*rule).flags & IMA_INMASK) != 0 && (((*rule).mask & mask) == 0 && func != ima_hooks::POST_SETATTR) { return false; }
    if ((*rule).flags & IMA_FSMAGIC) != 0 && (*rule).fsmagic != (*(*inode).i_sb).s_magic { return false; }
    if ((*rule).flags & IMA_FSNAME) != 0 && strcmp((*rule).fsname, (*(*(*inode).i_sb).s_type).name) != 0 { return false; }
    if ((*rule).flags & IMA_FS_SUBTYPE) != 0 {
        if (*(*inode).i_sb).s_subtype.is_null() { return false; }
        if strcmp((*rule).fs_subtype, (*(*inode).i_sb).s_subtype) != 0 { return false; }
    }
    if ((*rule).flags & IMA_FSUUID) != 0 && !uuid_equal(&(*rule).fsuuid, &(*(*inode).i_sb).s_uuid) { return false; }
    if ((*rule).flags & IMA_UID) != 0 && !((*rule).uid_op.unwrap())((*cred).uid, (*rule).uid) { return false; }
    if ((*rule).flags & IMA_EUID) != 0 {
        if has_capability_noaudit(current, CAP_SETUID) {
            if !((*rule).uid_op.unwrap())((*cred).euid, (*rule).uid)
                && !((*rule).uid_op.unwrap())((*cred).suid, (*rule).uid)
                && !((*rule).uid_op.unwrap())((*cred).uid, (*rule).uid) { return false; }
        } else if !((*rule).uid_op.unwrap())((*cred).euid, (*rule).uid) { return false; }
    }
    if ((*rule).flags & IMA_GID) != 0 && !((*rule).gid_op.unwrap())((*cred).gid, (*rule).gid) { return false; }
    if ((*rule).flags & IMA_EGID) != 0 {
        if has_capability_noaudit(current, CAP_SETGID) {
            if !((*rule).gid_op.unwrap())((*cred).egid, (*rule).gid)
                && !((*rule).gid_op.unwrap())((*cred).sgid, (*rule).gid)
                && !((*rule).gid_op.unwrap())((*cred).gid, (*rule).gid) { return false; }
        } else if !((*rule).gid_op.unwrap())((*cred).egid, (*rule).gid) { return false; }
    }
    if ((*rule).flags & IMA_FOWNER) != 0 && !((*rule).fowner_op.unwrap())(i_uid_into_vfsuid(idmap, inode), (*rule).fowner) { return false; }
    if ((*rule).flags & IMA_FGROUP) != 0 && !((*rule).fgroup_op.unwrap())(i_gid_into_vfsgid(idmap, inode), (*rule).fgroup) { return false; }
    for i in 0..MAX_LSM_RULES {
        let mut rc = 0;
        let mut inode_prop: lsm_prop = zeroed();
        if (*lsm_rule).lsm[i].rule.is_null() {
            if (*lsm_rule).lsm[i].args_p.is_null() { continue; } else { return false; }
        }
        loop {
            match i {
                0 | 1 | 2 => {
                    security_inode_getlsmprop(inode, &mut inode_prop);
                    rc = ima_filter_rule_match(&mut inode_prop, (*lsm_rule).lsm[i].type_, Audit_equal, (*lsm_rule).lsm[i].rule);
                }
                3 | 4 | 5 => {
                    rc = ima_filter_rule_match(prop, (*lsm_rule).lsm[i].type_, Audit_equal, (*lsm_rule).lsm[i].rule);
                }
                _ => {}
            }
            if rc == -ESTALE && !rule_reinitialized {
                lsm_rule = ima_lsm_copy_rule(rule, GFP_ATOMIC);
                if !lsm_rule.is_null() {
                    rule_reinitialized = true;
                    continue;
                }
            }
            break;
        }
        if rc <= 0 {
            result = false;
            break;
        }
        result = true;
    }
    if rule_reinitialized {
        for i in 0..MAX_LSM_RULES {
            ima_filter_rule_free((*lsm_rule).lsm[i].rule);
        }
        kfree(lsm_rule as *const c_void);
    }
    result
}

unsafe fn get_subaction(rule: *mut ima_rule_entry, func: ima_hooks) -> c_int {
    if ((*rule).flags & IMA_FUNC) == 0 {
        return IMA_FILE_APPRAISE;
    }
    match func {
        ima_hooks::MMAP_CHECK | ima_hooks::MMAP_CHECK_REQPROT => IMA_MMAP_APPRAISE,
        ima_hooks::BPRM_CHECK => IMA_BPRM_APPRAISE,
        ima_hooks::CREDS_CHECK => IMA_CREDS_APPRAISE,
        ima_hooks::FILE_CHECK | ima_hooks::POST_SETATTR => IMA_FILE_APPRAISE,
        _ => IMA_READ_APPRAISE,
    }
}

#[no_mangle]
pub unsafe extern "C" fn ima_match_policy(idmap: *mut mnt_idmap, inode: *mut inode, cred: *const cred, prop: *mut lsm_prop, func: ima_hooks, mask: c_int, flags: c_int, pcr: *mut c_int, template_desc: *mut *mut ima_template_desc, func_data: *const c_char, allowed_algos: *mut u32) -> c_int {
    let mut action = 0;
    let mut actmask = flags | (flags << 1);
    if !template_desc.is_null() && (*template_desc).is_null() {
        *template_desc = ima_template_desc_current();
    }
    rcu_read_lock();
    let mut pos = (*ima_rules).next;
    while pos != ima_rules {
        let entry = pos as *mut ima_rule_entry;
        pos = (*pos).next;
        if ((*entry).action & actmask) == 0 { continue; }
        if !ima_match_rules(entry, idmap, inode, cred, prop, func, mask, func_data) { continue; }
        action |= ((*entry).flags & IMA_NONACTION_FLAGS) as c_int;
        action |= (*entry).action & IMA_DO_MASK;
        if ((*entry).action & IMA_APPRAISE) != 0 {
            action |= get_subaction(entry, func);
            action &= !IMA_HASH;
            if ima_fail_unverifiable_sigs { action |= IMA_FAIL_UNVERIFIABLE_SIGS; }
            if !allowed_algos.is_null() && ((*entry).flags & IMA_VALIDATE_ALGOS) != 0 {
                *allowed_algos = (*entry).allowed_algos;
            }
        }
        if ((*entry).action & IMA_DO_MASK) != 0 {
            actmask &= !((*entry).action | ((*entry).action << 1));
        } else {
            actmask &= !((*entry).action | ((*entry).action >> 1));
        }
        if !pcr.is_null() && ((*entry).flags & IMA_PCR) != 0 { *pcr = (*entry).pcr; }
        if !template_desc.is_null() && !(*entry).template.is_null() { *template_desc = (*entry).template; }
        if actmask == 0 { break; }
    }
    rcu_read_unlock();
    action
}

#[no_mangle]
pub unsafe extern "C" fn ima_update_policy_flags() {
    let mut new_policy_flag = 0;
    rcu_read_lock();
    let mut pos = (*ima_rules).next;
    while pos != ima_rules {
        let entry = pos as *mut ima_rule_entry;
        pos = (*pos).next;
        if (*entry).func == ima_hooks::SETXATTR_CHECK {
            atomic_cmpxchg(&mut ima_setxattr_allowed_hash_algorithms, 0, (*entry).allowed_algos as c_int);
            continue;
        }
        if ((*entry).action & IMA_DO_MASK) != 0 {
            new_policy_flag |= (*entry).action;
        }
    }
    rcu_read_unlock();
    ima_appraise |= build_ima_appraise | temp_ima_appraise;
    if ima_appraise == 0 {
        new_policy_flag &= !IMA_APPRAISE;
    }
    ima_policy_flag = new_policy_flag;
}

unsafe fn ima_appraise_flag(func: ima_hooks) -> c_int {
    if func == ima_hooks::MODULE_CHECK { IMA_APPRAISE_MODULES }
    else if func == ima_hooks::FIRMWARE_CHECK { IMA_APPRAISE_FIRMWARE }
    else if func == ima_hooks::POLICY_CHECK { IMA_APPRAISE_POLICY }
    else if func == ima_hooks::KEXEC_KERNEL_CHECK { IMA_APPRAISE_KEXEC }
    else { 0 }
}

unsafe fn add_rules(entries: *mut ima_rule_entry, count: c_int, policy_rule: c_int) {
    for i in 0..count {
        let e = entries.add(i as usize);
        if (policy_rule & IMA_DEFAULT_POLICY) != 0 {
            list_add_tail(&mut (*e).list, &mut ima_default_rules);
        }
        if (policy_rule & IMA_CUSTOM_POLICY) != 0 {
            let entry = kmemdup(e as *const c_void, size_of::<ima_rule_entry>(), GFP_KERNEL) as *mut ima_rule_entry;
            if entry.is_null() { continue; }
            list_add_tail(&mut (*entry).list, &mut ima_policy_rules);
        }
        if (*e).action == APPRAISE {
            if entries != build_appraise_rules.as_mut_ptr() {
                temp_ima_appraise |= ima_appraise_flag((*e).func);
            } else {
                build_ima_appraise |= ima_appraise_flag((*e).func);
            }
        }
    }
}

unsafe fn ima_init_arch_policy() -> c_int {
    let arch_rules = arch_get_ima_policy();
    let mut arch_entries = 0;
    if arch_rules.is_null() { return arch_entries; }
    let mut rules = arch_rules;
    while !(*rules).is_null() {
        arch_entries += 1;
        rules = rules.add(1);
    }
    arch_policy_entry = kzalloc(size_of::<ima_rule_entry>() * (arch_entries as usize + 1), GFP_KERNEL) as *mut ima_rule_entry;
    if arch_policy_entry.is_null() { return 0; }
    rules = arch_rules;
    let mut i = 0;
    while !(*rules).is_null() {
        let mut rulebuf = [0 as c_char; 255];
        let _result = strscpy(rulebuf.as_mut_ptr(), *rules, rulebuf.len());
        INIT_LIST_HEAD(&mut (*arch_policy_entry.add(i)).list);
        let result = ima_parse_rule(rulebuf.as_mut_ptr(), arch_policy_entry.add(i));
        if result != 0 {
            pr_warn(c"Skipping unknown architecture policy rule: %s\n".as_ptr(), rulebuf.as_mut_ptr());
            memset(arch_policy_entry.add(i) as *mut c_void, 0, size_of::<ima_rule_entry>());
        } else {
            i += 1;
        }
        rules = rules.add(1);
    }
    i as c_int
}

#[no_mangle]
pub unsafe extern "C" fn ima_init_policy() {
    max_rule_len = 255;
    if ima_policy != 0 {
        add_rules(dont_measure_rules.as_mut_ptr(), dont_measure_rules.len() as c_int, IMA_DEFAULT_POLICY);
    }
    match ima_policy {
        ORIGINAL_TCB => add_rules(original_measurement_rules.as_mut_ptr(), original_measurement_rules.len() as c_int, IMA_DEFAULT_POLICY),
        DEFAULT_TCB => add_rules(default_measurement_rules.as_mut_ptr(), default_measurement_rules.len() as c_int, IMA_DEFAULT_POLICY),
        _ => {}
    }
    let arch_entries = ima_init_arch_policy();
    if arch_entries == 0 {
        pr_info(c"No architecture policies found\n".as_ptr());
    } else {
        add_rules(arch_policy_entry, arch_entries, IMA_DEFAULT_POLICY | IMA_CUSTOM_POLICY);
    }
    if ima_use_secure_boot {
        add_rules(secure_boot_rules.as_mut_ptr(), secure_boot_rules.len() as c_int, IMA_DEFAULT_POLICY);
    }
    let build_appraise_entries = build_appraise_rules.len() as c_int;
    if build_appraise_entries != 0 {
        if ima_use_secure_boot {
            add_rules(build_appraise_rules.as_mut_ptr(), build_appraise_entries, IMA_CUSTOM_POLICY);
        } else {
            add_rules(build_appraise_rules.as_mut_ptr(), build_appraise_entries, IMA_DEFAULT_POLICY | IMA_CUSTOM_POLICY);
        }
    }
    if ima_use_appraise_tcb {
        add_rules(default_appraise_rules.as_mut_ptr(), default_appraise_rules.len() as c_int, IMA_DEFAULT_POLICY);
    }
    if ima_use_critical_data {
        add_rules(critical_data_rules.as_mut_ptr(), critical_data_rules.len() as c_int, IMA_DEFAULT_POLICY);
    }
    atomic_set(&mut ima_setxattr_allowed_hash_algorithms, 0);
    ima_update_policy_flags();
}

#[no_mangle]
pub unsafe extern "C" fn ima_check_policy() -> c_int {
    if list_empty(&ima_temp_rules) != 0 { -EINVAL } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn ima_update_policy() {
    let policy = &mut ima_policy_rules as *mut list_head;
    list_splice_tail_init_rcu(&mut ima_temp_rules, policy, synchronize_rcu);
    if ima_rules != policy {
        ima_policy_flag = 0;
        ima_rules = policy;
        kfree(arch_policy_entry as *const c_void);
    }
    ima_update_policy_flags();
    ima_process_queued_keys();
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum policy_opt {
    Opt_measure, Opt_dont_measure,
    Opt_appraise, Opt_dont_appraise,
    Opt_audit, Opt_dont_audit, Opt_hash, Opt_dont_hash,
    Opt_obj_user, Opt_obj_role, Opt_obj_type,
    Opt_subj_user, Opt_subj_role, Opt_subj_type,
    Opt_func, Opt_mask, Opt_fsmagic, Opt_fsname, Opt_fs_subtype, Opt_fsuuid,
    Opt_uid_eq, Opt_euid_eq, Opt_gid_eq, Opt_egid_eq,
    Opt_fowner_eq, Opt_fgroup_eq,
    Opt_uid_gt, Opt_euid_gt, Opt_gid_gt, Opt_egid_gt,
    Opt_fowner_gt, Opt_fgroup_gt,
    Opt_uid_lt, Opt_euid_lt, Opt_gid_lt, Opt_egid_lt,
    Opt_fowner_lt, Opt_fgroup_lt,
    Opt_digest_type,
    Opt_appraise_type, Opt_appraise_flag, Opt_appraise_algos,
    Opt_permit_directio, Opt_pcr, Opt_template, Opt_keyrings,
    Opt_label, Opt_err,
}

macro_rules! mtok {
    ($tok:ident, $pat:literal) => { match_token_t { token: policy_opt::$tok as c_int, pattern: concat!($pat, "\0").as_ptr() as *const c_char } };
}

static policy_tokens: [match_token_t; 50] = [
    mtok!(Opt_measure, "measure"), mtok!(Opt_dont_measure, "dont_measure"),
    mtok!(Opt_appraise, "appraise"), mtok!(Opt_dont_appraise, "dont_appraise"),
    mtok!(Opt_audit, "audit"), mtok!(Opt_dont_audit, "dont_audit"),
    mtok!(Opt_hash, "hash"), mtok!(Opt_dont_hash, "dont_hash"),
    mtok!(Opt_obj_user, "obj_user=%s"), mtok!(Opt_obj_role, "obj_role=%s"), mtok!(Opt_obj_type, "obj_type=%s"),
    mtok!(Opt_subj_user, "subj_user=%s"), mtok!(Opt_subj_role, "subj_role=%s"), mtok!(Opt_subj_type, "subj_type=%s"),
    mtok!(Opt_func, "func=%s"), mtok!(Opt_mask, "mask=%s"), mtok!(Opt_fsmagic, "fsmagic=%s"),
    mtok!(Opt_fsname, "fsname=%s"), mtok!(Opt_fs_subtype, "fs_subtype=%s"), mtok!(Opt_fsuuid, "fsuuid=%s"),
    mtok!(Opt_uid_eq, "uid=%s"), mtok!(Opt_euid_eq, "euid=%s"), mtok!(Opt_gid_eq, "gid=%s"), mtok!(Opt_egid_eq, "egid=%s"),
    mtok!(Opt_fowner_eq, "fowner=%s"), mtok!(Opt_fgroup_eq, "fgroup=%s"),
    mtok!(Opt_uid_gt, "uid>%s"), mtok!(Opt_euid_gt, "euid>%s"), mtok!(Opt_gid_gt, "gid>%s"), mtok!(Opt_egid_gt, "egid>%s"),
    mtok!(Opt_fowner_gt, "fowner>%s"), mtok!(Opt_fgroup_gt, "fgroup>%s"),
    mtok!(Opt_uid_lt, "uid<%s"), mtok!(Opt_euid_lt, "euid<%s"), mtok!(Opt_gid_lt, "gid<%s"), mtok!(Opt_egid_lt, "egid<%s"),
    mtok!(Opt_fowner_lt, "fowner<%s"), mtok!(Opt_fgroup_lt, "fgroup<%s"),
    mtok!(Opt_digest_type, "digest_type=%s"), mtok!(Opt_appraise_type, "appraise_type=%s"), mtok!(Opt_appraise_flag, "appraise_flag=%s"),
    mtok!(Opt_appraise_algos, "appraise_algos=%s"), mtok!(Opt_permit_directio, "permit_directio"), mtok!(Opt_pcr, "pcr=%s"),
    mtok!(Opt_template, "template=%s"), mtok!(Opt_keyrings, "keyrings=%s"), mtok!(Opt_label, "label=%s"),
    match_token_t { token: policy_opt::Opt_err as c_int, pattern: ptr::null() },
];

unsafe fn ima_lsm_rule_init(entry: *mut ima_rule_entry, args: *mut substring_t, lsm_rule: c_int, audit_type: c_int) -> c_int {
    let idx = lsm_rule as usize;
    if !(*entry).lsm[idx].rule.is_null() { return -EINVAL; }
    (*entry).lsm[idx].args_p = match_strdup(args);
    if (*entry).lsm[idx].args_p.is_null() { return -ENOMEM; }
    (*entry).lsm[idx].type_ = audit_type;
    let mut result = ima_filter_rule_init((*entry).lsm[idx].type_, Audit_equal, (*entry).lsm[idx].args_p, &mut (*entry).lsm[idx].rule, GFP_KERNEL);
    if (*entry).lsm[idx].rule.is_null() {
        pr_warn(c"rule for LSM '%s' is undefined\n".as_ptr(), (*entry).lsm[idx].args_p);
        if ima_rules == &raw mut ima_default_rules {
            kfree((*entry).lsm[idx].args_p as *const c_void);
            (*entry).lsm[idx].args_p = ptr::null_mut();
            result = -EINVAL;
        } else {
            result = 0;
        }
    }
    result
}

unsafe fn ima_log_string_op(ab: *mut audit_buffer, key: *mut c_char, value: *mut c_char, rule_operator: policy_opt) {
    if ab.is_null() { return; }
    match rule_operator {
        policy_opt::Opt_uid_gt | policy_opt::Opt_euid_gt | policy_opt::Opt_gid_gt | policy_opt::Opt_egid_gt | policy_opt::Opt_fowner_gt | policy_opt::Opt_fgroup_gt =>
            audit_log_format(ab, c"%s>".as_ptr(), key),
        policy_opt::Opt_uid_lt | policy_opt::Opt_euid_lt | policy_opt::Opt_gid_lt | policy_opt::Opt_egid_lt | policy_opt::Opt_fowner_lt | policy_opt::Opt_fgroup_lt =>
            audit_log_format(ab, c"%s<".as_ptr(), key),
        _ => audit_log_format(ab, c"%s=".as_ptr(), key),
    }
    audit_log_format(ab, c"%s ".as_ptr(), value);
}

unsafe fn ima_log_string(ab: *mut audit_buffer, key: *mut c_char, value: *mut c_char) {
    ima_log_string_op(ab, key, value, policy_opt::Opt_err);
}

unsafe fn check_template_modsig(template: *const ima_template_desc) {
    static mut CHECKED: bool_t = false;
    if CHECKED { return; }
    let mut has_modsig = false;
    let mut has_dmodsig = false;
    for i in 0..(*template).num_fields {
        let id = (*(*(*template).fields.add(i as usize))).field_id;
        if strcmp(id, c"modsig".as_ptr()) == 0 { has_modsig = true; }
        else if strcmp(id, c"d-modsig".as_ptr()) == 0 { has_dmodsig = true; }
    }
    if has_modsig && !has_dmodsig {
        pr_notice(c"template with 'modsig' field also needs 'd-modsig' field\n".as_ptr());
    }
    CHECKED = true;
}

unsafe fn check_template_field(template: *const ima_template_desc, field: *const c_char, msg: *const c_char) {
    for i in 0..(*template).num_fields {
        if strcmp((*(*(*template).fields.add(i as usize))).field_id, field) == 0 { return; }
    }
    pr_notice_once(c"%s".as_ptr(), msg);
}

unsafe fn ima_validate_rule(entry: *mut ima_rule_entry) -> bool_t {
    if (*entry).action == UNKNOWN { return false; }
    if (*entry).action != MEASURE && ((*entry).flags & IMA_PCR) != 0 { return false; }
    if (*entry).action != APPRAISE && ((*entry).flags & (IMA_DIGSIG_REQUIRED | IMA_MODSIG_ALLOWED | IMA_CHECK_BLACKLIST | IMA_VALIDATE_ALGOS)) != 0 { return false; }
    if (((*entry).flags & IMA_FUNC) != 0 && (*entry).func == ima_hooks::NONE) || (((*entry).flags & IMA_FUNC) == 0 && (*entry).func != ima_hooks::NONE) { return false; }
    match (*entry).func {
        ima_hooks::NONE | ima_hooks::FILE_CHECK | ima_hooks::MMAP_CHECK | ima_hooks::MMAP_CHECK_REQPROT | ima_hooks::BPRM_CHECK | ima_hooks::CREDS_CHECK | ima_hooks::POST_SETATTR | ima_hooks::FIRMWARE_CHECK | ima_hooks::POLICY_CHECK => {
            let allowed = IMA_FUNC | IMA_MASK | IMA_FSMAGIC | IMA_UID | IMA_FOWNER | IMA_FSUUID | IMA_INMASK | IMA_EUID | IMA_PCR | IMA_FSNAME | IMA_FS_SUBTYPE | IMA_GID | IMA_EGID | IMA_FGROUP | IMA_DIGSIG_REQUIRED | IMA_PERMIT_DIRECTIO | IMA_VALIDATE_ALGOS | IMA_CHECK_BLACKLIST | IMA_VERITY_REQUIRED | IMA_SIGV3_REQUIRED;
            if ((*entry).flags & !allowed) != 0 { return false; }
        }
        ima_hooks::MODULE_CHECK | ima_hooks::KEXEC_KERNEL_CHECK | ima_hooks::KEXEC_INITRAMFS_CHECK => {
            let allowed = IMA_FUNC | IMA_MASK | IMA_FSMAGIC | IMA_UID | IMA_FOWNER | IMA_FSUUID | IMA_INMASK | IMA_EUID | IMA_PCR | IMA_FSNAME | IMA_FS_SUBTYPE | IMA_GID | IMA_EGID | IMA_FGROUP | IMA_DIGSIG_REQUIRED | IMA_PERMIT_DIRECTIO | IMA_MODSIG_ALLOWED | IMA_CHECK_BLACKLIST | IMA_VALIDATE_ALGOS | IMA_SIGV3_REQUIRED;
            if ((*entry).flags & !allowed) != 0 { return false; }
        }
        ima_hooks::KEXEC_CMDLINE => {
            if ((*entry).action & !(MEASURE | DONT_MEASURE)) != 0 { return false; }
            let allowed = IMA_FUNC | IMA_FSMAGIC | IMA_UID | IMA_FOWNER | IMA_FSUUID | IMA_EUID | IMA_PCR | IMA_FSNAME | IMA_FS_SUBTYPE | IMA_GID | IMA_EGID | IMA_FGROUP;
            if ((*entry).flags & !allowed) != 0 { return false; }
        }
        ima_hooks::KEY_CHECK => {
            if ((*entry).action & !(MEASURE | DONT_MEASURE)) != 0 { return false; }
            if ((*entry).flags & !(IMA_FUNC | IMA_UID | IMA_GID | IMA_PCR | IMA_KEYRINGS)) != 0 { return false; }
            if ima_rule_contains_lsm_cond(entry) { return false; }
        }
        ima_hooks::CRITICAL_DATA => {
            if ((*entry).action & !(MEASURE | DONT_MEASURE)) != 0 { return false; }
            if ((*entry).flags & !(IMA_FUNC | IMA_UID | IMA_GID | IMA_PCR | IMA_LABEL)) != 0 { return false; }
            if ima_rule_contains_lsm_cond(entry) { return false; }
        }
        ima_hooks::SETXATTR_CHECK => {
            if (*entry).action != APPRAISE { return false; }
            if ((*entry).flags & IMA_VALIDATE_ALGOS) == 0 { return false; }
            if ((*entry).flags & !(IMA_FUNC | IMA_VALIDATE_ALGOS)) != 0 { return false; }
        }
        _ => return false,
    }
    if ((*entry).flags & IMA_CHECK_BLACKLIST) != 0 && ((*entry).flags & IMA_DIGSIG_REQUIRED) == 0 { return false; }
    if (*entry).action == APPRAISE && ((*entry).flags & IMA_VERITY_REQUIRED) != 0 && ((*entry).flags & IMA_DIGSIG_REQUIRED) == 0 { return false; }
    true
}

unsafe fn ima_parse_appraise_algos(mut arg: *mut c_char) -> u32 {
    let mut res = 0u32;
    let mut token: *mut c_char;
    while {
        token = strsep(&mut arg, c",".as_ptr());
        !token.is_null()
    } {
        let idx = match_string(hash_algo_name.as_ptr(), HASH_ALGO__LAST, token);
        if idx < 0 {
            pr_err(c"unknown hash algorithm \"%s\"".as_ptr(), token);
            return 0;
        }
        if !crypto_has_alg(*hash_algo_name.as_ptr().add(idx as usize), 0, 0) {
            pr_err(c"unavailable hash algorithm \"%s\", check your kernel configuration".as_ptr(), token);
            return 0;
        }
        res |= 1u32 << idx;
    }
    res
}

unsafe fn ima_parse_rule(mut rule_: *mut c_char, entry: *mut ima_rule_entry) -> c_int {
    let ab = integrity_audit_log_start(audit_context(), GFP_KERNEL, AUDIT_INTEGRITY_POLICY_RULE);
    (*entry).uid = INVALID_UID;
    (*entry).gid = INVALID_GID;
    (*entry).fowner = INVALID_UID;
    (*entry).fgroup = INVALID_GID;
    (*entry).uid_op = Some(uid_eq);
    (*entry).gid_op = Some(gid_eq);
    (*entry).fowner_op = Some(vfsuid_eq_kuid);
    (*entry).fgroup_op = Some(vfsgid_eq_kgid);
    (*entry).action = UNKNOWN;
    let mut result = 0;
    let mut p: *mut c_char;
    while {
        p = strsep(&mut rule_, c" \t".as_ptr());
        !p.is_null()
    } {
        let mut args: [substring_t; MAX_OPT_ARGS] = zeroed();
        let mut lnum: c_ulong = 0;
        if result < 0 || *p == b'#' as c_char { break; }
        if *p == 0 || *p == b' ' as c_char || *p == b'\t' as c_char { continue; }
        let token = match_token(p, policy_tokens.as_ptr(), args.as_mut_ptr());
        match token {
            x if x == policy_opt::Opt_measure as c_int => { ima_log_string(ab, c"action".as_ptr() as *mut _, c"measure".as_ptr() as *mut _); if (*entry).action != UNKNOWN { result = -EINVAL; } (*entry).action = MEASURE; }
            x if x == policy_opt::Opt_dont_measure as c_int => { ima_log_string(ab, c"action".as_ptr() as *mut _, c"dont_measure".as_ptr() as *mut _); if (*entry).action != UNKNOWN { result = -EINVAL; } (*entry).action = DONT_MEASURE; }
            x if x == policy_opt::Opt_appraise as c_int => { ima_log_string(ab, c"action".as_ptr() as *mut _, c"appraise".as_ptr() as *mut _); if (*entry).action != UNKNOWN { result = -EINVAL; } (*entry).action = APPRAISE; }
            x if x == policy_opt::Opt_dont_appraise as c_int => { ima_log_string(ab, c"action".as_ptr() as *mut _, c"dont_appraise".as_ptr() as *mut _); if (*entry).action != UNKNOWN { result = -EINVAL; } (*entry).action = DONT_APPRAISE; }
            x if x == policy_opt::Opt_audit as c_int => { ima_log_string(ab, c"action".as_ptr() as *mut _, c"audit".as_ptr() as *mut _); if (*entry).action != UNKNOWN { result = -EINVAL; } (*entry).action = AUDIT; }
            x if x == policy_opt::Opt_dont_audit as c_int => { ima_log_string(ab, c"action".as_ptr() as *mut _, c"dont_audit".as_ptr() as *mut _); if (*entry).action != UNKNOWN { result = -EINVAL; } (*entry).action = DONT_AUDIT; }
            x if x == policy_opt::Opt_hash as c_int => { ima_log_string(ab, c"action".as_ptr() as *mut _, c"hash".as_ptr() as *mut _); if (*entry).action != UNKNOWN { result = -EINVAL; } (*entry).action = HASH; }
            x if x == policy_opt::Opt_dont_hash as c_int => { ima_log_string(ab, c"action".as_ptr() as *mut _, c"dont_hash".as_ptr() as *mut _); if (*entry).action != UNKNOWN { result = -EINVAL; } (*entry).action = DONT_HASH; }
            x if x == policy_opt::Opt_func as c_int => {
                ima_log_string(ab, c"func".as_ptr() as *mut _, args[0].from);
                if (*entry).func != ima_hooks::NONE { result = -EINVAL; }
                let a = args[0].from;
                if strcmp(a, c"FILE_CHECK".as_ptr()) == 0 || strcmp(a, c"PATH_CHECK".as_ptr()) == 0 { (*entry).func = ima_hooks::FILE_CHECK; }
                else if strcmp(a, c"MODULE_CHECK".as_ptr()) == 0 { (*entry).func = ima_hooks::MODULE_CHECK; }
                else if strcmp(a, c"FIRMWARE_CHECK".as_ptr()) == 0 { (*entry).func = ima_hooks::FIRMWARE_CHECK; }
                else if strcmp(a, c"FILE_MMAP".as_ptr()) == 0 || strcmp(a, c"MMAP_CHECK".as_ptr()) == 0 { (*entry).func = ima_hooks::MMAP_CHECK; }
                else if strcmp(a, c"MMAP_CHECK_REQPROT".as_ptr()) == 0 { (*entry).func = ima_hooks::MMAP_CHECK_REQPROT; }
                else if strcmp(a, c"BPRM_CHECK".as_ptr()) == 0 { (*entry).func = ima_hooks::BPRM_CHECK; }
                else if strcmp(a, c"CREDS_CHECK".as_ptr()) == 0 { (*entry).func = ima_hooks::CREDS_CHECK; }
                else if strcmp(a, c"KEXEC_KERNEL_CHECK".as_ptr()) == 0 { (*entry).func = ima_hooks::KEXEC_KERNEL_CHECK; }
                else if strcmp(a, c"KEXEC_INITRAMFS_CHECK".as_ptr()) == 0 { (*entry).func = ima_hooks::KEXEC_INITRAMFS_CHECK; }
                else if strcmp(a, c"POLICY_CHECK".as_ptr()) == 0 { (*entry).func = ima_hooks::POLICY_CHECK; }
                else if strcmp(a, c"KEXEC_CMDLINE".as_ptr()) == 0 { (*entry).func = ima_hooks::KEXEC_CMDLINE; }
                else if strcmp(a, c"KEY_CHECK".as_ptr()) == 0 { (*entry).func = ima_hooks::KEY_CHECK; }
                else if strcmp(a, c"CRITICAL_DATA".as_ptr()) == 0 { (*entry).func = ima_hooks::CRITICAL_DATA; }
                else if strcmp(a, c"SETXATTR_CHECK".as_ptr()) == 0 { (*entry).func = ima_hooks::SETXATTR_CHECK; }
                else { result = -EINVAL; }
                if result == 0 { (*entry).flags |= IMA_FUNC; }
            }
            x if x == policy_opt::Opt_mask as c_int => {
                ima_log_string(ab, c"mask".as_ptr() as *mut _, args[0].from);
                if (*entry).mask != 0 { result = -EINVAL; }
                let mut from = args[0].from;
                if *from == b'^' as c_char { from = from.add(1); }
                if strcmp(from, c"MAY_EXEC".as_ptr()) == 0 { (*entry).mask = MAY_EXEC; }
                else if strcmp(from, c"MAY_WRITE".as_ptr()) == 0 { (*entry).mask = MAY_WRITE; }
                else if strcmp(from, c"MAY_READ".as_ptr()) == 0 { (*entry).mask = MAY_READ; }
                else if strcmp(from, c"MAY_APPEND".as_ptr()) == 0 { (*entry).mask = MAY_APPEND; }
                else { result = -EINVAL; }
                if result == 0 { (*entry).flags |= if *args[0].from == b'^' as c_char { IMA_INMASK } else { IMA_MASK }; }
            }
            x if x == policy_opt::Opt_fsmagic as c_int => { ima_log_string(ab, c"fsmagic".as_ptr() as *mut _, args[0].from); if (*entry).fsmagic != 0 { result = -EINVAL; } else { result = kstrtoul(args[0].from, 16, &mut (*entry).fsmagic); if result == 0 { (*entry).flags |= IMA_FSMAGIC; } } }
            x if x == policy_opt::Opt_fsname as c_int => { ima_log_string(ab, c"fsname".as_ptr() as *mut _, args[0].from); (*entry).fsname = kstrdup(args[0].from, GFP_KERNEL); if (*entry).fsname.is_null() { result = -ENOMEM; } else { result = 0; (*entry).flags |= IMA_FSNAME; } }
            x if x == policy_opt::Opt_fs_subtype as c_int => { ima_log_string(ab, c"fs_subtype".as_ptr() as *mut _, args[0].from); if !(*entry).fs_subtype.is_null() { result = -EINVAL; } else { (*entry).fs_subtype = kstrdup(args[0].from, GFP_KERNEL); if (*entry).fs_subtype.is_null() { result = -ENOMEM; } else { result = 0; (*entry).flags |= IMA_FS_SUBTYPE; } } }
            x if x == policy_opt::Opt_keyrings as c_int => { ima_log_string(ab, c"keyrings".as_ptr() as *mut _, args[0].from); if !(*entry).keyrings.is_null() { result = -EINVAL; } else { (*entry).keyrings = ima_alloc_rule_opt_list(args.as_ptr()); if ((*entry).keyrings as isize) < 0 { result = (*entry).keyrings as isize as c_int; (*entry).keyrings = ptr::null_mut(); } else { (*entry).flags |= IMA_KEYRINGS; } } }
            x if x == policy_opt::Opt_label as c_int => { ima_log_string(ab, c"label".as_ptr() as *mut _, args[0].from); if !(*entry).label.is_null() { result = -EINVAL; } else { (*entry).label = ima_alloc_rule_opt_list(args.as_ptr()); if ((*entry).label as isize) < 0 { result = (*entry).label as isize as c_int; (*entry).label = ptr::null_mut(); } else { (*entry).flags |= IMA_LABEL; } } }
            x if x == policy_opt::Opt_fsuuid as c_int => { ima_log_string(ab, c"fsuuid".as_ptr() as *mut _, args[0].from); if !uuid_is_null(&(*entry).fsuuid) { result = -EINVAL; } else { result = uuid_parse(args[0].from, &mut (*entry).fsuuid); if result == 0 { (*entry).flags |= IMA_FSUUID; } } }
            x if x == policy_opt::Opt_uid_gt as c_int || x == policy_opt::Opt_euid_gt as c_int || x == policy_opt::Opt_uid_lt as c_int || x == policy_opt::Opt_euid_lt as c_int || x == policy_opt::Opt_uid_eq as c_int || x == policy_opt::Opt_euid_eq as c_int => {
                if token == policy_opt::Opt_uid_gt as c_int || token == policy_opt::Opt_euid_gt as c_int { (*entry).uid_op = Some(uid_gt); }
                if token == policy_opt::Opt_uid_lt as c_int || token == policy_opt::Opt_euid_lt as c_int { (*entry).uid_op = Some(uid_lt); }
                let eid = token == policy_opt::Opt_euid_eq as c_int || token == policy_opt::Opt_euid_gt as c_int || token == policy_opt::Opt_euid_lt as c_int;
                ima_log_string_op(ab, if eid { c"euid".as_ptr() } else { c"uid".as_ptr() } as *mut _, args[0].from, core::mem::transmute(token));
                if uid_valid((*entry).uid) { result = -EINVAL; } else { result = kstrtoul(args[0].from, 10, &mut lnum); if result == 0 { (*entry).uid = make_kuid(current_user_ns(), lnum as uid_t); if !uid_valid((*entry).uid) || (lnum as uid_t as c_ulong) != lnum { result = -EINVAL; } else { (*entry).flags |= if eid { IMA_EUID } else { IMA_UID }; } } }
            }
            x if x == policy_opt::Opt_gid_gt as c_int || x == policy_opt::Opt_egid_gt as c_int || x == policy_opt::Opt_gid_lt as c_int || x == policy_opt::Opt_egid_lt as c_int || x == policy_opt::Opt_gid_eq as c_int || x == policy_opt::Opt_egid_eq as c_int => {
                if token == policy_opt::Opt_gid_gt as c_int || token == policy_opt::Opt_egid_gt as c_int { (*entry).gid_op = Some(gid_gt); }
                if token == policy_opt::Opt_gid_lt as c_int || token == policy_opt::Opt_egid_lt as c_int { (*entry).gid_op = Some(gid_lt); }
                let eid = token == policy_opt::Opt_egid_eq as c_int || token == policy_opt::Opt_egid_gt as c_int || token == policy_opt::Opt_egid_lt as c_int;
                ima_log_string_op(ab, if eid { c"egid".as_ptr() } else { c"gid".as_ptr() } as *mut _, args[0].from, core::mem::transmute(token));
                if gid_valid((*entry).gid) { result = -EINVAL; } else { result = kstrtoul(args[0].from, 10, &mut lnum); if result == 0 { (*entry).gid = make_kgid(current_user_ns(), lnum as gid_t); if !gid_valid((*entry).gid) || (lnum as gid_t as c_ulong) != lnum { result = -EINVAL; } else { (*entry).flags |= if eid { IMA_EGID } else { IMA_GID }; } } }
            }
            x if x == policy_opt::Opt_fowner_gt as c_int || x == policy_opt::Opt_fowner_lt as c_int || x == policy_opt::Opt_fowner_eq as c_int => {
                if token == policy_opt::Opt_fowner_gt as c_int { (*entry).fowner_op = Some(vfsuid_gt_kuid); }
                if token == policy_opt::Opt_fowner_lt as c_int { (*entry).fowner_op = Some(vfsuid_lt_kuid); }
                ima_log_string_op(ab, c"fowner".as_ptr() as *mut _, args[0].from, core::mem::transmute(token));
                if uid_valid((*entry).fowner) { result = -EINVAL; } else { result = kstrtoul(args[0].from, 10, &mut lnum); if result == 0 { (*entry).fowner = make_kuid(current_user_ns(), lnum as uid_t); if !uid_valid((*entry).fowner) || (lnum as uid_t as c_ulong) != lnum { result = -EINVAL; } else { (*entry).flags |= IMA_FOWNER; } } }
            }
            x if x == policy_opt::Opt_fgroup_gt as c_int || x == policy_opt::Opt_fgroup_lt as c_int || x == policy_opt::Opt_fgroup_eq as c_int => {
                if token == policy_opt::Opt_fgroup_gt as c_int { (*entry).fgroup_op = Some(vfsgid_gt_kgid); }
                if token == policy_opt::Opt_fgroup_lt as c_int { (*entry).fgroup_op = Some(vfsgid_lt_kgid); }
                ima_log_string_op(ab, c"fgroup".as_ptr() as *mut _, args[0].from, core::mem::transmute(token));
                if gid_valid((*entry).fgroup) { result = -EINVAL; } else { result = kstrtoul(args[0].from, 10, &mut lnum); if result == 0 { (*entry).fgroup = make_kgid(current_user_ns(), lnum as gid_t); if !gid_valid((*entry).fgroup) || (lnum as gid_t as c_ulong) != lnum { result = -EINVAL; } else { (*entry).flags |= IMA_FGROUP; } } }
            }
            x if x == policy_opt::Opt_obj_user as c_int => { ima_log_string(ab, c"obj_user".as_ptr() as *mut _, args[0].from); result = ima_lsm_rule_init(entry, args.as_mut_ptr(), lsm_rule_types::LSM_OBJ_USER as c_int, AUDIT_OBJ_USER); }
            x if x == policy_opt::Opt_obj_role as c_int => { ima_log_string(ab, c"obj_role".as_ptr() as *mut _, args[0].from); result = ima_lsm_rule_init(entry, args.as_mut_ptr(), lsm_rule_types::LSM_OBJ_ROLE as c_int, AUDIT_OBJ_ROLE); }
            x if x == policy_opt::Opt_obj_type as c_int => { ima_log_string(ab, c"obj_type".as_ptr() as *mut _, args[0].from); result = ima_lsm_rule_init(entry, args.as_mut_ptr(), lsm_rule_types::LSM_OBJ_TYPE as c_int, AUDIT_OBJ_TYPE); }
            x if x == policy_opt::Opt_subj_user as c_int => { ima_log_string(ab, c"subj_user".as_ptr() as *mut _, args[0].from); result = ima_lsm_rule_init(entry, args.as_mut_ptr(), lsm_rule_types::LSM_SUBJ_USER as c_int, AUDIT_SUBJ_USER); }
            x if x == policy_opt::Opt_subj_role as c_int => { ima_log_string(ab, c"subj_role".as_ptr() as *mut _, args[0].from); result = ima_lsm_rule_init(entry, args.as_mut_ptr(), lsm_rule_types::LSM_SUBJ_ROLE as c_int, AUDIT_SUBJ_ROLE); }
            x if x == policy_opt::Opt_subj_type as c_int => { ima_log_string(ab, c"subj_type".as_ptr() as *mut _, args[0].from); result = ima_lsm_rule_init(entry, args.as_mut_ptr(), lsm_rule_types::LSM_SUBJ_TYPE as c_int, AUDIT_SUBJ_TYPE); }
            x if x == policy_opt::Opt_digest_type as c_int => { ima_log_string(ab, c"digest_type".as_ptr() as *mut _, args[0].from); if strcmp(args[0].from, c"verity".as_ptr()) == 0 { (*entry).flags |= IMA_VERITY_REQUIRED; } else { result = -EINVAL; } }
            x if x == policy_opt::Opt_appraise_type as c_int => { ima_log_string(ab, c"appraise_type".as_ptr() as *mut _, args[0].from); if strcmp(args[0].from, c"imasig".as_ptr()) == 0 { if ((*entry).flags & IMA_VERITY_REQUIRED) != 0 { result = -EINVAL; } else { (*entry).flags |= IMA_DIGSIG_REQUIRED | IMA_CHECK_BLACKLIST; } } else if strcmp(args[0].from, c"sigv3".as_ptr()) == 0 { (*entry).flags |= IMA_SIGV3_REQUIRED | IMA_DIGSIG_REQUIRED | IMA_CHECK_BLACKLIST; } else if strcmp(args[0].from, c"imasig|modsig".as_ptr()) == 0 { if ((*entry).flags & IMA_VERITY_REQUIRED) != 0 || ((*entry).flags & IMA_SIGV3_REQUIRED) != 0 { result = -EINVAL; } else { (*entry).flags |= IMA_DIGSIG_REQUIRED | IMA_MODSIG_ALLOWED | IMA_CHECK_BLACKLIST; } } else { result = -EINVAL; } }
            x if x == policy_opt::Opt_appraise_flag as c_int => { ima_log_string(ab, c"appraise_flag".as_ptr() as *mut _, args[0].from); }
            x if x == policy_opt::Opt_appraise_algos as c_int => { ima_log_string(ab, c"appraise_algos".as_ptr() as *mut _, args[0].from); if (*entry).allowed_algos != 0 { result = -EINVAL; } else { (*entry).allowed_algos = ima_parse_appraise_algos(args[0].from); if (*entry).allowed_algos == 0 { result = -EINVAL; } else { (*entry).flags |= IMA_VALIDATE_ALGOS; } } }
            x if x == policy_opt::Opt_permit_directio as c_int => { (*entry).flags |= IMA_PERMIT_DIRECTIO; }
            x if x == policy_opt::Opt_pcr as c_int => { ima_log_string(ab, c"pcr".as_ptr() as *mut _, args[0].from); result = kstrtoint(args[0].from, 10, &mut (*entry).pcr); if result != 0 || INVALID_PCR((*entry).pcr) { result = -EINVAL; } else { (*entry).flags |= IMA_PCR; } }
            x if x == policy_opt::Opt_template as c_int => { ima_log_string(ab, c"template".as_ptr() as *mut _, args[0].from); if (*entry).action != MEASURE { result = -EINVAL; } else { let td = lookup_template_desc(args[0].from); if td.is_null() || !(*entry).template.is_null() { result = -EINVAL; } else { template_desc_init_fields((*td).fmt, &mut (*td).fields, &mut (*td).num_fields); (*entry).template = td; } } }
            _ => { ima_log_string(ab, c"UNKNOWN".as_ptr() as *mut _, p); result = -EINVAL; }
        }
    }
    if result == 0 && !ima_validate_rule(entry) { result = -EINVAL; }
    else if (*entry).action == APPRAISE { temp_ima_appraise |= ima_appraise_flag((*entry).func); }
    if result == 0 && ((*entry).flags & IMA_MODSIG_ALLOWED) != 0 {
        let template_desc = if !(*entry).template.is_null() { (*entry).template } else { ima_template_desc_current() };
        check_template_modsig(template_desc);
    }
    if result == 0 && (*entry).action == MEASURE && ((*entry).flags & IMA_VERITY_REQUIRED) != 0 {
        let template_desc = if !(*entry).template.is_null() { (*entry).template } else { ima_template_desc_current() };
        check_template_field(template_desc, c"d-ngv2".as_ptr(), c"verity rules should include d-ngv2".as_ptr());
    }
    audit_log_format(ab, c"res=%d".as_ptr(), (result == 0) as c_int);
    audit_log_end(ab);
    result
}

#[no_mangle]
pub unsafe extern "C" fn ima_parse_add_rule(mut rule_: *mut c_char) -> ssize_t {
    let op = c"update_policy".as_ptr();
    let mut p = strsep(&mut rule_, c"\n".as_ptr());
    let len = strlen(p) + 1;
    p = p.add(strspn(p, c" \t".as_ptr()));
    if *p == b'#' as c_char || *p == 0 { return len as ssize_t; }
    let entry = kzalloc(size_of::<ima_rule_entry>(), GFP_KERNEL) as *mut ima_rule_entry;
    if entry.is_null() {
        integrity_audit_msg(AUDIT_INTEGRITY_STATUS, ptr::null_mut(), ptr::null(), op, c"-ENOMEM".as_ptr(), -ENOMEM, 0);
        return -ENOMEM as ssize_t;
    }
    INIT_LIST_HEAD(&mut (*entry).list);
    let result = ima_parse_rule(p, entry);
    if result != 0 {
        ima_free_rule(entry);
        integrity_audit_msg(AUDIT_INTEGRITY_STATUS, ptr::null_mut(), ptr::null(), op, c"invalid-policy".as_ptr(), result, 0);
        return result as ssize_t;
    }
    list_add_tail(&mut (*entry).list, &mut ima_temp_rules);
    if len > max_rule_len { max_rule_len = len; }
    len as ssize_t
}

#[no_mangle]
pub unsafe extern "C" fn ima_delete_rules() {
    temp_ima_appraise = 0;
    let mut pos = ima_temp_rules.next;
    while pos != &raw mut ima_temp_rules {
        let entry = pos as *mut ima_rule_entry;
        pos = (*pos).next;
        list_del(&mut (*entry).list);
        ima_free_rule(entry);
    }
}

static func_tokens: [*const c_char; ima_hooks::MAX_CHECK as usize] = [
    c"NONE".as_ptr(), c"FILE_CHECK".as_ptr(), c"MMAP_CHECK".as_ptr(), c"BPRM_CHECK".as_ptr(),
    c"CREDS_CHECK".as_ptr(), c"POST_SETATTR".as_ptr(), c"MODULE_CHECK".as_ptr(),
    c"FIRMWARE_CHECK".as_ptr(), c"KEXEC_KERNEL_CHECK".as_ptr(), c"KEXEC_INITRAMFS_CHECK".as_ptr(),
    c"POLICY_CHECK".as_ptr(), c"KEXEC_CMDLINE".as_ptr(), c"KEY_CHECK".as_ptr(),
    c"CRITICAL_DATA".as_ptr(), c"SETXATTR_CHECK".as_ptr(), c"MMAP_CHECK_REQPROT".as_ptr(),
];

pub const mask_exec: usize = 0;
pub const mask_write: usize = 1;
pub const mask_read: usize = 2;
pub const mask_append: usize = 3;
static mask_tokens: [*const c_char; 4] = [c"^MAY_EXEC".as_ptr(), c"^MAY_WRITE".as_ptr(), c"^MAY_READ".as_ptr(), c"^MAY_APPEND".as_ptr()];

#[no_mangle]
pub unsafe extern "C" fn ima_policy_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut l = *pos;
    rcu_read_lock();
    let mut p = (*ima_rules).next;
    while p != ima_rules {
        let entry = p as *mut ima_rule_entry;
        if l == 0 {
            rcu_read_unlock();
            return entry as *mut c_void;
        }
        l -= 1;
        p = (*p).next;
    }
    rcu_read_unlock();
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ima_policy_next(_m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut entry = v as *mut ima_rule_entry;
    rcu_read_lock();
    entry = (*entry).list.next as *mut ima_rule_entry;
    rcu_read_unlock();
    *pos += 1;
    if &mut (*entry).list as *mut list_head == &raw mut ima_default_rules || &mut (*entry).list as *mut list_head == &raw mut ima_policy_rules {
        ptr::null_mut()
    } else {
        entry as *mut c_void
    }
}

#[no_mangle]
pub unsafe extern "C" fn ima_policy_stop(_m: *mut seq_file, _v: *mut c_void) {}

unsafe fn pt(token: policy_opt) -> *const c_char {
    policy_tokens[token as usize].pattern
}

unsafe fn mt(token: usize) -> *const c_char {
    mask_tokens[token]
}

unsafe fn policy_func_show(m: *mut seq_file, func: ima_hooks) {
    if (func as c_int) > 0 && (func as c_int) < ima_hooks::MAX_CHECK as c_int {
        seq_printf(m, c"func=%s ".as_ptr(), func_tokens[func as usize]);
    } else {
        seq_printf(m, c"func=%d ".as_ptr(), func as c_int);
    }
}

unsafe fn ima_show_rule_opt_list(m: *mut seq_file, opt_list: *const ima_rule_opt_list) {
    for i in 0..(*opt_list).count {
        seq_printf(m, c"%s%s".as_ptr(), if i != 0 { c"|".as_ptr() } else { c"".as_ptr() }, *(*opt_list).items.as_ptr().add(i));
    }
}

unsafe fn ima_policy_show_appraise_algos(m: *mut seq_file, allowed_hashes: u32) {
    let mut list_size = 0;
    for idx in 0..HASH_ALGO__LAST {
        if (allowed_hashes & (1u32 << idx)) == 0 { continue; }
        if list_size != 0 { seq_puts(m, c",".as_ptr()); }
        list_size += 1;
        seq_puts(m, *hash_algo_name.as_ptr().add(idx as usize));
    }
}

#[no_mangle]
pub unsafe extern "C" fn ima_policy_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let entry = v as *mut ima_rule_entry;
    let mut tbuf = [0 as c_char; 64];
    let mut offset = 0;
    rcu_read_lock();
    for i in 0..MAX_LSM_RULES {
        if !(*entry).lsm[i].args_p.is_null() && (*entry).lsm[i].rule.is_null() {
            rcu_read_unlock();
            return 0;
        }
    }
    if ((*entry).action & MEASURE) != 0 { seq_puts(m, pt(policy_opt::Opt_measure)); }
    if ((*entry).action & DONT_MEASURE) != 0 { seq_puts(m, pt(policy_opt::Opt_dont_measure)); }
    if ((*entry).action & APPRAISE) != 0 { seq_puts(m, pt(policy_opt::Opt_appraise)); }
    if ((*entry).action & DONT_APPRAISE) != 0 { seq_puts(m, pt(policy_opt::Opt_dont_appraise)); }
    if ((*entry).action & AUDIT) != 0 { seq_puts(m, pt(policy_opt::Opt_audit)); }
    if ((*entry).action & DONT_AUDIT) != 0 { seq_puts(m, pt(policy_opt::Opt_dont_audit)); }
    if ((*entry).action & HASH) != 0 { seq_puts(m, pt(policy_opt::Opt_hash)); }
    if ((*entry).action & DONT_HASH) != 0 { seq_puts(m, pt(policy_opt::Opt_dont_hash)); }
    seq_puts(m, c" ".as_ptr());
    if ((*entry).flags & IMA_FUNC) != 0 { policy_func_show(m, (*entry).func); }
    if ((*entry).flags & (IMA_MASK | IMA_INMASK)) != 0 {
        if ((*entry).flags & IMA_MASK) != 0 { offset = 1; }
        if ((*entry).mask & MAY_EXEC) != 0 { seq_printf(m, pt(policy_opt::Opt_mask), mt(mask_exec).add(offset)); }
        if ((*entry).mask & MAY_WRITE) != 0 { seq_printf(m, pt(policy_opt::Opt_mask), mt(mask_write).add(offset)); }
        if ((*entry).mask & MAY_READ) != 0 { seq_printf(m, pt(policy_opt::Opt_mask), mt(mask_read).add(offset)); }
        if ((*entry).mask & MAY_APPEND) != 0 { seq_printf(m, pt(policy_opt::Opt_mask), mt(mask_append).add(offset)); }
        seq_puts(m, c" ".as_ptr());
    }
    if ((*entry).flags & IMA_FSMAGIC) != 0 { snprintf(tbuf.as_mut_ptr(), tbuf.len(), c"0x%lx".as_ptr(), (*entry).fsmagic); seq_printf(m, pt(policy_opt::Opt_fsmagic), tbuf.as_mut_ptr()); seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_FSNAME) != 0 { snprintf(tbuf.as_mut_ptr(), tbuf.len(), c"%s".as_ptr(), (*entry).fsname); seq_printf(m, pt(policy_opt::Opt_fsname), tbuf.as_mut_ptr()); seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_FS_SUBTYPE) != 0 { snprintf(tbuf.as_mut_ptr(), tbuf.len(), c"%s".as_ptr(), (*entry).fs_subtype); seq_printf(m, pt(policy_opt::Opt_fs_subtype), tbuf.as_mut_ptr()); seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_KEYRINGS) != 0 { seq_puts(m, c"keyrings=".as_ptr()); ima_show_rule_opt_list(m, (*entry).keyrings); seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_LABEL) != 0 { seq_puts(m, c"label=".as_ptr()); ima_show_rule_opt_list(m, (*entry).label); seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_PCR) != 0 { snprintf(tbuf.as_mut_ptr(), tbuf.len(), c"%d".as_ptr(), (*entry).pcr); seq_printf(m, pt(policy_opt::Opt_pcr), tbuf.as_mut_ptr()); seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_FSUUID) != 0 { seq_printf(m, c"fsuuid=%pU".as_ptr(), &(*entry).fsuuid); seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_UID) != 0 { snprintf(tbuf.as_mut_ptr(), tbuf.len(), c"%d".as_ptr(), __kuid_val((*entry).uid)); if (*entry).uid_op == Some(uid_gt) { seq_printf(m, pt(policy_opt::Opt_uid_gt), tbuf.as_mut_ptr()); } else if (*entry).uid_op == Some(uid_lt) { seq_printf(m, pt(policy_opt::Opt_uid_lt), tbuf.as_mut_ptr()); } else { seq_printf(m, pt(policy_opt::Opt_uid_eq), tbuf.as_mut_ptr()); } seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_EUID) != 0 { snprintf(tbuf.as_mut_ptr(), tbuf.len(), c"%d".as_ptr(), __kuid_val((*entry).uid)); if (*entry).uid_op == Some(uid_gt) { seq_printf(m, pt(policy_opt::Opt_euid_gt), tbuf.as_mut_ptr()); } else if (*entry).uid_op == Some(uid_lt) { seq_printf(m, pt(policy_opt::Opt_euid_lt), tbuf.as_mut_ptr()); } else { seq_printf(m, pt(policy_opt::Opt_euid_eq), tbuf.as_mut_ptr()); } seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_GID) != 0 { snprintf(tbuf.as_mut_ptr(), tbuf.len(), c"%d".as_ptr(), __kgid_val((*entry).gid)); if (*entry).gid_op == Some(gid_gt) { seq_printf(m, pt(policy_opt::Opt_gid_gt), tbuf.as_mut_ptr()); } else if (*entry).gid_op == Some(gid_lt) { seq_printf(m, pt(policy_opt::Opt_gid_lt), tbuf.as_mut_ptr()); } else { seq_printf(m, pt(policy_opt::Opt_gid_eq), tbuf.as_mut_ptr()); } seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_EGID) != 0 { snprintf(tbuf.as_mut_ptr(), tbuf.len(), c"%d".as_ptr(), __kgid_val((*entry).gid)); if (*entry).gid_op == Some(gid_gt) { seq_printf(m, pt(policy_opt::Opt_egid_gt), tbuf.as_mut_ptr()); } else if (*entry).gid_op == Some(gid_lt) { seq_printf(m, pt(policy_opt::Opt_egid_lt), tbuf.as_mut_ptr()); } else { seq_printf(m, pt(policy_opt::Opt_egid_eq), tbuf.as_mut_ptr()); } seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_FOWNER) != 0 { snprintf(tbuf.as_mut_ptr(), tbuf.len(), c"%d".as_ptr(), __kuid_val((*entry).fowner)); if (*entry).fowner_op == Some(vfsuid_gt_kuid) { seq_printf(m, pt(policy_opt::Opt_fowner_gt), tbuf.as_mut_ptr()); } else if (*entry).fowner_op == Some(vfsuid_lt_kuid) { seq_printf(m, pt(policy_opt::Opt_fowner_lt), tbuf.as_mut_ptr()); } else { seq_printf(m, pt(policy_opt::Opt_fowner_eq), tbuf.as_mut_ptr()); } seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_FGROUP) != 0 { snprintf(tbuf.as_mut_ptr(), tbuf.len(), c"%d".as_ptr(), __kgid_val((*entry).fgroup)); if (*entry).fgroup_op == Some(vfsgid_gt_kgid) { seq_printf(m, pt(policy_opt::Opt_fgroup_gt), tbuf.as_mut_ptr()); } else if (*entry).fgroup_op == Some(vfsgid_lt_kgid) { seq_printf(m, pt(policy_opt::Opt_fgroup_lt), tbuf.as_mut_ptr()); } else { seq_printf(m, pt(policy_opt::Opt_fgroup_eq), tbuf.as_mut_ptr()); } seq_puts(m, c" ".as_ptr()); }
    if ((*entry).flags & IMA_VALIDATE_ALGOS) != 0 { seq_puts(m, c"appraise_algos=".as_ptr()); ima_policy_show_appraise_algos(m, (*entry).allowed_algos); seq_puts(m, c" ".as_ptr()); }
    for i in 0..MAX_LSM_RULES {
        if !(*entry).lsm[i].rule.is_null() {
            match i {
                0 => seq_printf(m, pt(policy_opt::Opt_obj_user), (*entry).lsm[i].args_p),
                1 => seq_printf(m, pt(policy_opt::Opt_obj_role), (*entry).lsm[i].args_p),
                2 => seq_printf(m, pt(policy_opt::Opt_obj_type), (*entry).lsm[i].args_p),
                3 => seq_printf(m, pt(policy_opt::Opt_subj_user), (*entry).lsm[i].args_p),
                4 => seq_printf(m, pt(policy_opt::Opt_subj_role), (*entry).lsm[i].args_p),
                5 => seq_printf(m, pt(policy_opt::Opt_subj_type), (*entry).lsm[i].args_p),
                _ => {}
            }
            seq_puts(m, c" ".as_ptr());
        }
    }
    if !(*entry).template.is_null() { seq_printf(m, c"template=%s ".as_ptr(), (*(*entry).template).name); }
    if ((*entry).flags & IMA_DIGSIG_REQUIRED) != 0 {
        if ((*entry).flags & IMA_SIGV3_REQUIRED) != 0 { seq_puts(m, c"appraise_type=sigv3 ".as_ptr()); }
        else if ((*entry).flags & IMA_MODSIG_ALLOWED) != 0 { seq_puts(m, c"appraise_type=imasig|modsig ".as_ptr()); }
        else { seq_puts(m, c"appraise_type=imasig ".as_ptr()); }
    }
    if ((*entry).flags & IMA_VERITY_REQUIRED) != 0 { seq_puts(m, c"digest_type=verity ".as_ptr()); }
    if ((*entry).flags & IMA_PERMIT_DIRECTIO) != 0 { seq_puts(m, c"permit_directio ".as_ptr()); }
    rcu_read_unlock();
    seq_puts(m, c"\n".as_ptr());
    0
}

/* CONFIG_IMA_APPRAISE && CONFIG_INTEGRITY_TRUSTED_KEYRING */
#[no_mangle]
pub unsafe extern "C" fn ima_appraise_signature(id: c_int) -> bool_t {
    let mut found = false;
    if id >= READING_MAX_ID { return false; }
    if id == READING_KEXEC_IMAGE && (ima_appraise & IMA_APPRAISE_ENFORCE) == 0 && security_locked_down(LOCKDOWN_KEXEC) != 0 { return false; }
    let mut func = *read_idmap.as_ptr().add(id as usize);
    if func == ima_hooks::NONE { func = ima_hooks::FILE_CHECK; }
    rcu_read_lock();
    let mut pos = (*ima_rules).next;
    while pos != ima_rules {
        let entry = pos as *mut ima_rule_entry;
        pos = (*pos).next;
        if (*entry).action != APPRAISE { continue; }
        if (*entry).func != ima_hooks::NONE && (*entry).func != func { continue; }
        if ((*entry).flags & IMA_DIGSIG_REQUIRED) != 0 { found = true; }
        break;
    }
    rcu_read_unlock();
    found
}

#[no_mangle]
pub unsafe extern "C" fn ima_measure_loaded_policy() {
    let event_name = c"ima_policy_loaded".as_ptr();
    let op = c"measure_loaded_ima_policy".as_ptr();
    let rule_len = max_rule_len + 2;
    let mut file: seq_file = zeroed();
    let mut result = -ENOMEM;
    let mut file_len: size_t = 0;
    lockdep_assert_held(&ima_write_mutex);
    let rulebuf = kmalloc(rule_len, GFP_KERNEL) as *mut c_char;
    if rulebuf.is_null() {
        integrity_audit_msg(AUDIT_INTEGRITY_PCR, ptr::null_mut(), event_name, op, c"ENOMEM".as_ptr(), result, 0);
        return;
    }
    file.buf = rulebuf;
    file.read_pos = 0;
    file.size = rule_len;
    file.count = 0;
    rcu_read_lock();
    let mut pos = (*ima_rules).next;
    while pos != ima_rules {
        let rule_entry = pos as *mut ima_rule_entry;
        pos = (*pos).next;
        ima_policy_show(&mut file, rule_entry as *mut c_void);
        if seq_has_overflowed(&mut file) {
            result = -E2BIG;
            integrity_audit_msg(AUDIT_INTEGRITY_PCR, ptr::null_mut(), event_name, op, c"rule_length".as_ptr(), result, 0);
            rcu_read_unlock();
            kfree(rulebuf as *const c_void);
            return;
        }
        file_len += file.count;
        file.count = 0;
    }
    rcu_read_unlock();
    file.buf = kmalloc(file_len, GFP_KERNEL) as *mut c_char;
    if file.buf.is_null() {
        integrity_audit_msg(AUDIT_INTEGRITY_PCR, ptr::null_mut(), event_name, op, c"ENOMEM".as_ptr(), result, 0);
        kfree(rulebuf as *const c_void);
        return;
    }
    file.read_pos = 0;
    file.size = file_len;
    file.count = 0;
    rcu_read_lock();
    pos = (*ima_rules).next;
    while pos != ima_rules {
        let rule_entry = pos as *mut ima_rule_entry;
        pos = (*pos).next;
        ima_policy_show(&mut file, rule_entry as *mut c_void);
    }
    rcu_read_unlock();
    ima_measure_critical_data(c"ima_policy".as_ptr(), event_name, file.buf as *const c_void, file.count, false, ptr::null(), 0);
    kfree(file.buf as *const c_void);
    kfree(rulebuf as *const c_void);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
