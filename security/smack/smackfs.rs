// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2007 Casey Schaufler <casey@schaufler-ca.com>
 *
 * Authors:
 *      Casey Schaufler <casey@schaufler-ca.com>
 *      Ahmed S. Darwish <darwish.07@gmail.com>
 *
 * Special thanks to the authors of selinuxfs.
 *
 *      Karl MacMillan <kmacmillan@tresys.com>
 *      James Morris <jmorris@redhat.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type __be32 = u32;
type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type gfp_t = c_uint;
type umode_t = c_uint;

/*
 * C include dependencies translated as opaque external types and functions.
 * Concrete definitions are supplied by the surrounding kernel/Smack code.
 */
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    pub i_ino: c_ulong,
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fs_context {
    pub ops: *const fs_context_operations,
}

#[repr(C)]
pub struct vfsmount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlbl_lsm_catmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlbl_lsm_secattr_mls {
    pub cat: *mut netlbl_lsm_catmap,
    pub lvl: c_int,
}

#[repr(C)]
pub struct netlbl_lsm_secattr_attr {
    pub mls: netlbl_lsm_secattr_mls,
}

#[repr(C)]
pub struct netlbl_lsm_secattr {
    pub flags: u32,
    pub attr: netlbl_lsm_secattr_attr,
}

#[repr(C)]
pub struct smack_known {
    pub list: list_head,
    pub smk_known: *mut c_char,
    pub smk_rules: list_head,
    pub smk_rules_lock: mutex,
    pub smk_netlabel: netlbl_lsm_secattr,
    pub smk_secid: u32,
}

#[repr(C)]
pub struct smack_rule {
    pub list: list_head,
    pub smk_subject: *mut smack_known,
    pub smk_object: *mut smack_known,
    pub smk_access: c_int,
}

#[repr(C)]
pub struct smack_known_list_elem {
    pub list: list_head,
    pub smk_label: *mut smack_known,
}

#[repr(C)]
pub struct task_smack {
    pub smk_rules: list_head,
    pub smk_rules_lock: mutex,
    pub smk_relabel: list_head,
}

#[repr(C)]
pub struct cred {
    _private: [u8; 0],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: __be32,
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_addr: in_addr,
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr16: [u16; 8],
}

#[repr(C)]
pub struct smk_net4addr {
    pub list: list_head,
    pub smk_host: in_addr,
    pub smk_mask: in_addr,
    pub smk_label: *mut smack_known,
    pub smk_masks: c_uint,
}

#[repr(C)]
pub struct smk_net6addr {
    pub list: list_head,
    pub smk_host: in6_addr,
    pub smk_mask: in6_addr,
    pub smk_label: *mut smack_known,
    pub smk_masks: c_uint,
}

#[repr(C)]
pub union cipso_v4_doi_map {
    pub std: *mut c_void,
}

#[repr(C)]
pub struct cipso_v4_doi {
    pub map: cipso_v4_doi_map,
    pub doi: u32,
    pub type_: c_int,
    pub tags: [c_int; CIPSO_V4_TAG_MAXCNT],
}

#[repr(C)]
pub struct netlbl_audit_smack {
    pub skp: *mut smack_known,
}

#[repr(C)]
pub union netlbl_audit_prop {
    pub smack: netlbl_audit_smack,
}

#[repr(C)]
pub struct netlbl_audit {
    pub loginuid: u32,
    pub sessionid: u32,
    pub prop: netlbl_audit_prop,
}

#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>,
    pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut loff_t) -> *mut c_void>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
}

#[repr(C)]
pub struct tree_descr {
    pub name: *const c_char,
    pub ops: *const file_operations,
    pub mode: umode_t,
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
pub struct smack_parsed_rule {
    pub smk_subject: *mut smack_known,
    pub smk_object: *mut smack_known,
    pub smk_access1: c_int,
    pub smk_access2: c_int,
}

const BEBITS: usize = size_of::<__be32>() * 8;

/*
 * smackfs pseudo filesystem.
 */
const SMK_ROOT_INO: c_ulong = 2;
const SMK_LOAD: usize = 3; /* load policy */
const SMK_CIPSO: usize = 4; /* load label -> CIPSO mapping */
const SMK_DOI: usize = 5; /* CIPSO DOI */
const SMK_DIRECT: c_ulong = 6; /* CIPSO level indicating direct label */
const SMK_AMBIENT: usize = 7; /* internet ambient label */
const SMK_NET4ADDR: usize = 8; /* single label hosts */
const SMK_ONLYCAP: usize = 9; /* the only "capable" label */
#[cfg(CONFIG_AUDIT)]
const SMK_LOGGING: usize = 10; /* logging */
const SMK_LOAD_SELF: usize = 11; /* task specific rules */
const SMK_ACCESSES: usize = 12; /* access policy */
const SMK_MAPPED: usize = 13; /* CIPSO level indicating mapped label */
const SMK_LOAD2: usize = 14; /* load policy with long labels */
const SMK_LOAD_SELF2: usize = 15; /* load task specific rules with long labels */
const SMK_ACCESS2: usize = 16; /* make an access check with long labels */
const SMK_CIPSO2: usize = 17; /* load long label -> CIPSO mapping */
const SMK_REVOKE_SUBJ: usize = 18; /* set rules with subject label to '-' */
const SMK_CHANGE_RULE: usize = 19; /* change or add rules (long labels) */
const SMK_SYSLOG: usize = 20; /* change syslog label) */
const SMK_PTRACE: usize = 21; /* set ptrace rule */
#[cfg(CONFIG_SECURITY_SMACK_BRINGUP)]
const SMK_UNCONFINED: usize = 22; /* define an unconfined label */
#[cfg(CONFIG_IPV6)]
const SMK_NET6ADDR: usize = 23; /* single label IPv6 hosts */
const SMK_RELABEL_SELF: usize = 24; /* relabel possible without CAP_MAC_ADMIN */

/*
 * Constants supplied by kernel headers/smack.h in the original C translation unit.
 */
extern "C" {
    static mut smack_known_list: list_head;
    static mut smack_known_lock: mutex;
    static mut smack_onlycap_list: list_head;
    static mut smack_onlycap_lock: mutex;
    static mut smack_rule_cache: *mut c_void;
    static mut smack_known_floor: smack_known;
    static mut smack_known_hat: smack_known;
    static mut smack_known_huh: smack_known;
    static mut smack_known_star: smack_known;
    static mut smack_known_web: smack_known;
    static mut smack_enabled: c_int;
    static mut init_net: c_void;
    static mut fs_kobj: *mut c_void;
    #[cfg(CONFIG_AUDIT)]
    static mut log_policy: c_int;
}

extern "C" {
    fn file_inode(file: *const file) -> *mut inode;
    fn audit_get_loginuid(task: *mut c_void) -> u32;
    fn audit_get_sessionid(task: *mut c_void) -> u32;
    fn smk_of_current() -> *mut smack_known;
    fn current_cred() -> *const cred;
    fn smack_cred(cred: *const cred) -> *mut task_smack;
    fn smk_import_entry(string: *const c_char, len: c_int) -> *mut smack_known;
    fn smk_parse_smack(string: *const c_char, len: c_int) -> *mut c_char;
    fn smk_find_entry(string: *const c_char) -> *mut smack_known;
    fn smack_privileged(cap: c_int) -> bool;
    fn smack_str_from_perm(string: *mut c_char, access: c_int);
    fn smk_netlbl_mls(level: c_int, catset: *mut c_char, sap: *mut netlbl_lsm_secattr, len: c_int) -> c_int;
    fn smk_access(subject: *mut smack_known, object: *mut smack_known, request: c_int, a: *mut c_void) -> c_int;
    fn smack_populate_secattr(skp: *mut smack_known) -> c_int;

    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn synchronize_rcu();

    fn kmem_cache_zalloc(cache: *mut c_void, flags: gfp_t) -> *mut c_void;
    fn kzalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kmalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kfree(p: *const c_void);
    fn memdup_user_nul(buf: *const c_char, count: size_t) -> *mut c_char;
    fn memdup_user(buf: *const c_char, count: size_t) -> *mut c_char;

    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strsep(s: *mut *mut c_char, delim: *const c_char) -> *mut c_char;

    fn seq_open(file: *mut file, ops: *const seq_operations) -> c_int;
    fn seq_read(file: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
    fn seq_lseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn seq_release(inode: *mut inode, file: *mut file) -> c_int;
    fn seq_printf(s: *mut seq_file, fmt: *const c_char, ...);
    fn seq_putc(s: *mut seq_file, c: c_char);
    fn seq_puts(s: *mut seq_file, str: *const c_char);

    fn simple_read_from_buffer(to: *mut c_char, count: size_t, ppos: *mut loff_t, from: *const c_void, available: size_t) -> ssize_t;
    fn simple_transaction_get(file: *mut file, buf: *const c_char, count: size_t) -> *mut c_char;
    fn simple_transaction_set(file: *mut file, n: size_t);
    fn simple_transaction_read(file: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
    fn simple_transaction_release(inode: *mut inode, file: *mut file) -> c_int;
    fn generic_file_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn default_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;

    fn netlbl_cfg_cipsov4_add(doi: *mut cipso_v4_doi, audit: *mut netlbl_audit) -> c_int;
    fn netlbl_cfg_cipsov4_del(doi: u32, audit: *mut netlbl_audit) -> c_int;
    fn netlbl_cfg_cipsov4_map_add(doi: u32, domain: *const c_char, addr: *const c_void, mask: *const c_void, audit: *mut netlbl_audit) -> c_int;
    fn netlbl_cfg_map_del(domain: *const c_char, family: c_int, addr: *const c_void, mask: *const c_void, audit: *mut netlbl_audit) -> c_int;
    fn netlbl_cfg_unlbl_map_add(domain: *const c_char, family: c_int, addr: *const c_void, mask: *const c_void, audit: *mut netlbl_audit) -> c_int;
    fn netlbl_cfg_unlbl_static_add(net: *mut c_void, dev: *const c_char, addr: *const in_addr, mask: *const in_addr, family: c_int, secid: u32, audit: *mut netlbl_audit) -> c_int;
    fn netlbl_cfg_unlbl_static_del(net: *mut c_void, dev: *const c_char, addr: *const in_addr, mask: *const in_addr, family: c_int, audit: *mut netlbl_audit) -> c_int;
    fn netlbl_catmap_walk(catmap: *mut netlbl_lsm_catmap, offset: c_int) -> c_int;
    fn netlbl_catmap_free(catmap: *mut netlbl_lsm_catmap);
    fn netlbl_cache_invalidate();

    fn kstrtou32_from_user(buf: *const c_char, count: size_t, base: c_uint, res: *mut u32) -> c_int;
    fn kstrtou8_from_user(buf: *const c_char, count: size_t, base: c_uint, res: *mut u8) -> c_int;
    fn kstrtos32_from_user(buf: *const c_char, count: size_t, base: c_uint, res: *mut c_int) -> c_int;

    fn cpu_to_be32(x: u32) -> __be32;
    fn htons(x: c_uint) -> u16;
    fn prepare_creds() -> *mut cred;
    fn commit_creds(new: *mut cred) -> c_int;
    fn sysfs_create_mount_point(kobj: *mut c_void, name: *const c_char) -> c_int;
    fn simple_fill_super(sb: *mut super_block, magic: c_ulong, files: *const tree_descr) -> c_int;
    fn get_tree_single(fc: *mut fs_context, fill: unsafe extern "C" fn(*mut super_block, *mut fs_context) -> c_int) -> c_int;
    fn register_filesystem(fs: *mut file_system_type) -> c_int;
    fn kern_mount(fs: *mut file_system_type) -> *mut vfsmount;
    fn kill_anon_super(sb: *mut super_block);
}

const GFP_KERNEL: gfp_t = 0;
const __GFP_NOFAIL: gfp_t = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const EPERM: c_int = 1;
const EOVERFLOW: c_int = 75;
const PF_INET: c_int = 2;
const CAP_MAC_ADMIN: c_int = 33;
const MAY_READ: c_int = 0x0004;
const MAY_WRITE: c_int = 0x0002;
const MAY_EXEC: c_int = 0x0001;
const MAY_APPEND: c_int = 0x0008;
const MAY_TRANSMUTE: c_int = 0x0010;
const MAY_LOCK: c_int = 0x0020;
const MAY_BRINGUP: c_int = 0x0040;
const PAGE_SIZE: size_t = 4096;
const SMK_LABELLEN: size_t = 24;
const SMK_LONGLABEL: size_t = 256;
const SMK_CIPSOLEN: size_t = 24;
const SMK_NUM_ACCESS_TYPE: usize = 6;
const SMACK_CIPSO_MAXCATNUM: size_t = 184;
const SMACK_CIPSO_MAXLEVEL: c_int = 255;
const SMACK_CIPSO_DIRECT_DEFAULT: u8 = 250;
const SMACK_CIPSO_MAPPED_DEFAULT: u8 = 251;
const CIPSO_V4_DOI_UNKNOWN: u32 = 0;
const SMACK_CIPSO_DOI_DEFAULT: u32 = 3;
const CIPSO_V4_MAP_PASS: c_int = 2;
const CIPSO_V4_TAG_RBITMAP: c_int = 1;
const CIPSO_V4_TAG_INVALID: c_int = 0;
const CIPSO_V4_TAG_MAXCNT: usize = 5;
const NETLBL_SECATTR_MLS_CAT: u32 = 0x0002;
const SMACK_PTRACE_DEFAULT: c_int = 0;
const SMACK_PTRACE_MAX: c_int = 2;
const SMACK_MAGIC: c_ulong = 0x43415d53;
const S_IRUGO: umode_t = 0o444;
const S_IWUSR: umode_t = 0o200;
const S_IWUGO: umode_t = 0o222;

const SMK_DIGITLEN: size_t = 4;
const SMK_CIPSOMIN: size_t = SMK_LABELLEN + 2 * SMK_DIGITLEN;
const SMK_CIPSOMAX: size_t = SMK_CIPSOMIN + SMACK_CIPSO_MAXCATNUM * SMK_DIGITLEN;
const SMK_OACCESS: &[u8] = b"rwxa\0";
const SMK_ACCESS: &[u8] = b"rwxatl\0";
const SMK_OACCESSLEN: size_t = 4;
const SMK_ACCESSLEN: size_t = 6;
const SMK_OLOADLEN: size_t = SMK_LABELLEN + SMK_LABELLEN + SMK_OACCESSLEN;
const SMK_LOADLEN: size_t = SMK_LABELLEN + SMK_LABELLEN + SMK_ACCESSLEN;
const SMK_NETLBLADDRMIN: size_t = 9;
const SMK_FIXED24_FMT: c_int = 0;
const SMK_LONG_FMT: c_int = 1;
const SMK_CHANGE_FMT: c_int = 2;
const SMACK_CIPSO_OPTION: *const c_char = b"-CIPSO\0".as_ptr() as *const c_char;
#[cfg(CONFIG_IPV6)]
const SMACK_DELETE_OPTION: *const c_char = b"-DELETE\0".as_ptr() as *const c_char;

static mut smack_cipso_lock: mutex = mutex { _private: [] };
static mut smack_ambient_lock: mutex = mutex { _private: [] };
static mut smk_net4addr_lock: mutex = mutex { _private: [] };
static mut smk_cipso_doi_lock: mutex = mutex { _private: [] };
#[cfg(CONFIG_IPV6)]
static mut smk_net6addr_lock: mutex = mutex { _private: [] };

pub static mut smack_net_ambient: *mut smack_known = ptr::null_mut();
pub static mut smack_cipso_auto_level: [u8; 2] = [
    SMACK_CIPSO_DIRECT_DEFAULT,
    SMACK_CIPSO_MAPPED_DEFAULT,
];
#[cfg(CONFIG_SECURITY_SMACK_BRINGUP)]
pub static mut smack_unconfined: *mut smack_known = ptr::null_mut();
pub static mut smack_syslog_label: *mut smack_known = ptr::null_mut();
pub static mut smack_ptrace_rule: c_int = SMACK_PTRACE_DEFAULT;

static mut smk_net4addr_list: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};
#[cfg(CONFIG_IPV6)]
static mut smk_net6addr_list: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};
static mut smk_cipso_doi_value: u32 = CIPSO_V4_DOI_UNKNOWN;
static mut smackfs_mount: *mut vfsmount = ptr::null_mut();

unsafe fn IS_ERR<T>(p: *const T) -> bool {
    (p as isize) < 0 && (p as isize) >= -4095
}

unsafe fn PTR_ERR<T>(p: *const T) -> c_int {
    p as isize as c_int
}

unsafe fn list_empty(head: *const list_head) -> bool {
    (*head).next == head as *mut list_head
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn list_add(new: *mut list_head, head: *mut list_head) {
    let next = (*head).next;
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = head;
    (*head).next = new;
}

unsafe fn list_add_rcu(new: *mut list_head, head: *mut list_head) {
    list_add(new, head);
}

unsafe fn list_is_last(list: *const list_head, head: *const list_head) -> bool {
    (*list).next == head as *mut list_head
}

unsafe fn list_splice(list: *mut list_head, head: *mut list_head) {
    if list_empty(list) {
        return;
    }
    let first = (*list).next;
    let last = (*list).prev;
    let at = (*head).next;
    (*first).prev = head;
    (*head).next = first;
    (*last).next = at;
    (*at).prev = last;
}

unsafe fn list_splice_init_rcu(list: *mut list_head, head: *mut list_head) {
    list_splice(list, head);
    INIT_LIST_HEAD(list);
}

unsafe fn rcu_assign_pointer<T>(dst: *mut *mut T, val: *mut T) {
    *dst = val;
}

unsafe fn isspace_c(c: c_char) -> bool {
    matches!(c as u8, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

/*
 * Strictly for CIPSO level manipulation.
 * Set the category bit number in a smack label sized buffer.
 */
unsafe fn smack_catset_bit(cat: c_uint, catsetp: *mut c_char) {
    if cat == 0 || cat > (SMK_CIPSOLEN * 8) as c_uint {
        return;
    }
    let index = ((cat - 1) / 8) as isize;
    let bit = (0x80u8 >> ((cat - 1) % 8)) as c_char;
    *catsetp.offset(index) = ((*catsetp.offset(index) as u8) | (bit as u8)) as c_char;
}

/**
 * smk_netlabel_audit_set - fill a netlbl_audit struct
 * @nap: structure to fill
 */
unsafe fn smk_netlabel_audit_set(nap: *mut netlbl_audit) {
    static mut current: *mut c_void = ptr::null_mut();
    (*nap).loginuid = audit_get_loginuid(current);
    (*nap).sessionid = audit_get_sessionid(current);
    (*nap).prop.smack.skp = smk_of_current();
}

/**
 * smk_set_access - add a rule to the rule list or replace an old rule
 */
unsafe fn smk_set_access(
    srp: *mut smack_parsed_rule,
    rule_list: *mut list_head,
    rule_lock: *mut mutex,
) -> c_int {
    let mut found = 0;
    let mut rc = 0;
    mutex_lock(rule_lock);

    let mut pos = (*rule_list).next;
    while pos != rule_list {
        let sp = pos as *mut smack_rule;
        if (*sp).smk_object == (*srp).smk_object && (*sp).smk_subject == (*srp).smk_subject {
            found = 1;
            (*sp).smk_access |= (*srp).smk_access1;
            (*sp).smk_access &= !(*srp).smk_access2;
            break;
        }
        pos = (*pos).next;
    }

    if found == 0 {
        let sp = kmem_cache_zalloc(smack_rule_cache, GFP_KERNEL) as *mut smack_rule;
        if sp.is_null() {
            rc = -ENOMEM;
        } else {
            (*sp).smk_subject = (*srp).smk_subject;
            (*sp).smk_object = (*srp).smk_object;
            (*sp).smk_access = (*srp).smk_access1 & !(*srp).smk_access2;
            list_add_rcu(&mut (*sp).list, rule_list);
        }
    }

    mutex_unlock(rule_lock);
    rc
}

/**
 * smk_perm_from_str - parse smack accesses from a text string
 */
unsafe fn smk_perm_from_str(string: *const c_char) -> c_int {
    let mut perm = 0;
    let mut cp = string;
    loop {
        match *cp as u8 {
            b'-' => {}
            b'r' | b'R' => perm |= MAY_READ,
            b'w' | b'W' => perm |= MAY_WRITE,
            b'x' | b'X' => perm |= MAY_EXEC,
            b'a' | b'A' => perm |= MAY_APPEND,
            b't' | b'T' => perm |= MAY_TRANSMUTE,
            b'l' | b'L' => perm |= MAY_LOCK,
            b'b' | b'B' => perm |= MAY_BRINGUP,
            _ => return perm,
        }
        cp = cp.add(1);
    }
}

/**
 * smk_fill_rule - Fill Smack rule from strings
 */
unsafe fn smk_fill_rule(
    subject: *const c_char,
    object: *const c_char,
    access1: *const c_char,
    access2: *const c_char,
    rule: *mut smack_parsed_rule,
    import: c_int,
    len: c_int,
) -> c_int {
    if import != 0 {
        (*rule).smk_subject = smk_import_entry(subject, len);
        if IS_ERR((*rule).smk_subject) {
            return PTR_ERR((*rule).smk_subject);
        }
        (*rule).smk_object = smk_import_entry(object, len);
        if IS_ERR((*rule).smk_object) {
            return PTR_ERR((*rule).smk_object);
        }
    } else {
        let mut cp = smk_parse_smack(subject, len);
        if IS_ERR(cp) {
            return PTR_ERR(cp);
        }
        let mut skp = smk_find_entry(cp);
        kfree(cp as *const c_void);
        if skp.is_null() {
            return -ENOENT;
        }
        (*rule).smk_subject = skp;

        cp = smk_parse_smack(object, len);
        if IS_ERR(cp) {
            return PTR_ERR(cp);
        }
        skp = smk_find_entry(cp);
        kfree(cp as *const c_void);
        if skp.is_null() {
            return -ENOENT;
        }
        (*rule).smk_object = skp;
    }

    (*rule).smk_access1 = smk_perm_from_str(access1);
    if !access2.is_null() {
        (*rule).smk_access2 = smk_perm_from_str(access2);
    } else {
        (*rule).smk_access2 = !(*rule).smk_access1;
    }
    0
}

/**
 * smk_parse_rule - parse Smack rule from load string
 */
unsafe fn smk_parse_rule(data: *const c_char, rule: *mut smack_parsed_rule, import: c_int) -> c_int {
    smk_fill_rule(
        data,
        data.add(SMK_LABELLEN),
        data.add(SMK_LABELLEN + SMK_LABELLEN),
        ptr::null(),
        rule,
        import,
        SMK_LABELLEN as c_int,
    )
}

/**
 * smk_parse_long_rule - parse Smack rule from rule string
 */
unsafe fn smk_parse_long_rule(
    data: *mut c_char,
    rule: *mut smack_parsed_rule,
    import: c_int,
    tokens: c_int,
) -> ssize_t {
    let mut cnt: ssize_t = 0;
    let mut tok: [*mut c_char; 4] = [ptr::null_mut(); 4];
    let mut i = 0;
    while i < tokens {
        while isspace_c(*data.offset(cnt)) {
            *data.offset(cnt) = 0;
            cnt += 1;
        }
        if *data.offset(cnt) == 0 {
            return -EINVAL as ssize_t;
        }
        tok[i as usize] = data.offset(cnt);
        while *data.offset(cnt) != 0 && !isspace_c(*data.offset(cnt)) {
            cnt += 1;
        }
        i += 1;
    }
    while isspace_c(*data.offset(cnt)) {
        *data.offset(cnt) = 0;
        cnt += 1;
    }
    while i < 4 {
        tok[i as usize] = ptr::null_mut();
        i += 1;
    }
    let rc = smk_fill_rule(tok[0], tok[1], tok[2], tok[3], rule, import, 0);
    if rc == 0 { cnt } else { rc as ssize_t }
}

/**
 * smk_write_rules_list - write() for any /smack rule file
 */
unsafe fn smk_write_rules_list(
    _file: *mut file,
    buf: *const c_char,
    mut count: size_t,
    ppos: *mut loff_t,
    rule_list: *mut list_head,
    rule_lock: *mut mutex,
    format: c_int,
) -> ssize_t {
    let mut rule: smack_parsed_rule = core::mem::zeroed();
    let mut trunc = 0;
    let mut cnt: ssize_t = 0;
    if *ppos != 0 {
        return -EINVAL as ssize_t;
    }
    if format == SMK_FIXED24_FMT {
        if count < SMK_OLOADLEN || count > SMK_LOADLEN {
            return -EINVAL as ssize_t;
        }
    } else if count >= PAGE_SIZE {
        count = PAGE_SIZE - 1;
        trunc = 1;
    }
    let data = memdup_user_nul(buf, count);
    if IS_ERR(data) {
        return PTR_ERR(data) as ssize_t;
    }
    let mut rc: c_int;
    if trunc != 0 {
        while count > 0 && *data.add(count - 1) != b'\n' as c_char {
            count -= 1;
        }
        if count == 0 {
            rc = -EINVAL;
            kfree(data as *const c_void);
            return rc as ssize_t;
        }
    }
    *data.add(count) = 0;
    let tokens = if format == SMK_CHANGE_FMT { 4 } else { 3 };
    while cnt < count as ssize_t {
        if format == SMK_FIXED24_FMT {
            rc = smk_parse_rule(data, &mut rule, 1);
            if rc < 0 {
                kfree(data as *const c_void);
                return rc as ssize_t;
            }
            cnt = count as ssize_t;
        } else {
            let r = smk_parse_long_rule(data.offset(cnt), &mut rule, 1, tokens);
            if r < 0 {
                kfree(data as *const c_void);
                return r;
            }
            if r == 0 {
                kfree(data as *const c_void);
                return -EINVAL as ssize_t;
            }
            cnt += r;
        }
        rc = if rule_list.is_null() {
            smk_set_access(&mut rule, &mut (*rule.smk_subject).smk_rules, &mut (*rule.smk_subject).smk_rules_lock)
        } else {
            smk_set_access(&mut rule, rule_list, rule_lock)
        };
        if rc != 0 {
            kfree(data as *const c_void);
            return rc as ssize_t;
        }
    }
    kfree(data as *const c_void);
    cnt
}

unsafe fn smk_seq_start(_s: *mut seq_file, pos: *mut loff_t, head: *mut list_head) -> *mut c_void {
    let mut i = *pos;
    rcu_read_lock();
    let mut list = (*head).next;
    while list != head {
        if i == 0 {
            return list as *mut c_void;
        }
        i -= 1;
        list = (*list).next;
    }
    ptr::null_mut()
}

unsafe fn smk_seq_next(_s: *mut seq_file, v: *mut c_void, pos: *mut loff_t, head: *mut list_head) -> *mut c_void {
    *pos += 1;
    let list = (*(v as *mut list_head)).next;
    if list == head { ptr::null_mut() } else { list as *mut c_void }
}

unsafe extern "C" fn smk_seq_stop(_s: *mut seq_file, _v: *mut c_void) {
    rcu_read_unlock();
}

unsafe fn smk_rule_show(s: *mut seq_file, srp: *mut smack_rule, max: c_int) {
    let mut acc = [0 as c_char; SMK_NUM_ACCESS_TYPE + 1];
    if strlen((*(*srp).smk_subject).smk_known) >= max as usize ||
       strlen((*(*srp).smk_object).smk_known) >= max as usize {
        return;
    }
    if (*srp).smk_access == 0 {
        return;
    }
    smack_str_from_perm(acc.as_mut_ptr(), (*srp).smk_access);
    seq_printf(s, b"%s %s %s\n\0".as_ptr() as *const c_char,
        (*(*srp).smk_subject).smk_known,
        (*(*srp).smk_object).smk_known,
        acc.as_ptr());
}

unsafe extern "C" fn load2_seq_start(s: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    smk_seq_start(s, pos, &mut smack_known_list)
}

unsafe extern "C" fn load2_seq_next(s: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    smk_seq_next(s, v, pos, &mut smack_known_list)
}

unsafe extern "C" fn load_seq_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    let skp = v as *mut smack_known;
    let mut pos = (*skp).smk_rules.next;
    while pos != &mut (*skp).smk_rules {
        smk_rule_show(s, pos as *mut smack_rule, SMK_LABELLEN as c_int);
        pos = (*pos).next;
    }
    0
}

static load_seq_ops: seq_operations = seq_operations {
    start: Some(load2_seq_start),
    stop: Some(smk_seq_stop),
    next: Some(load2_seq_next),
    show: Some(load_seq_show),
};

unsafe extern "C" fn smk_open_load(_inode: *mut inode, file: *mut file) -> c_int {
    seq_open(file, &load_seq_ops)
}

unsafe extern "C" fn smk_write_load(file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    smk_write_rules_list(file, buf, count, ppos, ptr::null_mut(), ptr::null_mut(), SMK_FIXED24_FMT)
}

static smk_load_ops: file_operations = file_operations {
    open: Some(smk_open_load),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    write: Some(smk_write_load),
    release: Some(seq_release),
};

unsafe fn smk_cipso_auto_level_idx(file: *const file) -> c_int {
    ((*file_inode(file)).i_ino != SMK_DIRECT) as c_int
}

unsafe fn smk_cipso_doi(ndoi: u32, gfp_flags: gfp_t) -> c_int {
    let mut rc = 0;
    let mut nai: netlbl_audit = core::mem::zeroed();
    mutex_lock(&mut smk_cipso_doi_lock);
    if smk_cipso_doi_value == ndoi {
        mutex_unlock(&mut smk_cipso_doi_lock);
        return rc;
    }
    smk_netlabel_audit_set(&mut nai);
    let doip = kmalloc(size_of::<cipso_v4_doi>(), gfp_flags) as *mut cipso_v4_doi;
    if doip.is_null() {
        mutex_unlock(&mut smk_cipso_doi_lock);
        return -ENOMEM;
    }
    (*doip).map.std = ptr::null_mut();
    (*doip).doi = ndoi;
    (*doip).type_ = CIPSO_V4_MAP_PASS;
    (*doip).tags[0] = CIPSO_V4_TAG_RBITMAP;
    rc = 1;
    while (rc as usize) < CIPSO_V4_TAG_MAXCNT {
        (*doip).tags[rc as usize] = CIPSO_V4_TAG_INVALID;
        rc += 1;
    }
    rc = netlbl_cfg_cipsov4_add(doip, &mut nai);
    if rc != 0 {
        kfree(doip as *const c_void);
        mutex_unlock(&mut smk_cipso_doi_lock);
        return rc;
    }
    if smk_cipso_doi_value != CIPSO_V4_DOI_UNKNOWN {
        rc = netlbl_cfg_map_del(ptr::null(), PF_INET, ptr::null(), ptr::null(), &mut nai);
        if rc != 0 && rc != -ENOENT {
            netlbl_cfg_cipsov4_del(ndoi, &mut nai);
            mutex_unlock(&mut smk_cipso_doi_lock);
            return rc;
        }
        netlbl_cfg_cipsov4_del(smk_cipso_doi_value, &mut nai);
    }
    rc = netlbl_cfg_cipsov4_map_add(ndoi, ptr::null(), ptr::null(), ptr::null(), &mut nai);
    if rc != 0 {
        smk_cipso_doi_value = CIPSO_V4_DOI_UNKNOWN;
        netlbl_cfg_cipsov4_del(ndoi, &mut nai);
    } else {
        smk_cipso_doi_value = ndoi;
    }
    mutex_unlock(&mut smk_cipso_doi_lock);
    rc
}

unsafe fn smk_unlbl_ambient(oldambient: *mut c_char) {
    let mut nai: netlbl_audit = core::mem::zeroed();
    smk_netlabel_audit_set(&mut nai);
    if !oldambient.is_null() {
        let _rc = netlbl_cfg_map_del(oldambient, PF_INET, ptr::null(), ptr::null(), &mut nai);
    }
    if smack_net_ambient.is_null() {
        smack_net_ambient = &mut smack_known_floor;
    }
    let _rc = netlbl_cfg_unlbl_map_add((*smack_net_ambient).smk_known, PF_INET, ptr::null(), ptr::null(), &mut nai);
}

unsafe extern "C" fn cipso_seq_start(s: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    smk_seq_start(s, pos, &mut smack_known_list)
}

unsafe extern "C" fn cipso_seq_next(s: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    smk_seq_next(s, v, pos, &mut smack_known_list)
}

unsafe extern "C" fn cipso_seq_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    let skp = v as *mut smack_known;
    let cmp = (*skp).smk_netlabel.attr.mls.cat;
    let mut sep = b'/' as c_char;
    if strlen((*skp).smk_known) >= SMK_LABELLEN {
        return 0;
    }
    seq_printf(s, b"%s %3d\0".as_ptr() as *const c_char, (*skp).smk_known, (*skp).smk_netlabel.attr.mls.lvl);
    let mut i = netlbl_catmap_walk(cmp, 0);
    while i >= 0 {
        seq_printf(s, b"%c%d\0".as_ptr() as *const c_char, sep as c_int, i);
        sep = b',' as c_char;
        i = netlbl_catmap_walk(cmp, i + 1);
    }
    seq_putc(s, b'\n' as c_char);
    0
}

static cipso_seq_ops: seq_operations = seq_operations {
    start: Some(cipso_seq_start),
    stop: Some(smk_seq_stop),
    next: Some(cipso_seq_next),
    show: Some(cipso_seq_show),
};

unsafe extern "C" fn smk_open_cipso(_inode: *mut inode, file: *mut file) -> c_int {
    seq_open(file, &cipso_seq_ops)
}

unsafe fn smk_set_cipso(_file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t, format: c_int) -> ssize_t {
    let mut ncats: netlbl_lsm_secattr = core::mem::zeroed();
    let mut mapcatset = [0 as c_char; SMK_CIPSOLEN];
    let mut maplevel: c_int = 0;
    let mut cat: c_uint = 0;
    let mut catlen: c_int = 0;
    let mut rc: ssize_t = -EINVAL as ssize_t;
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    if *ppos != 0 {
        return -EINVAL as ssize_t;
    }
    if format == SMK_FIXED24_FMT && (count < SMK_CIPSOMIN || count > SMK_CIPSOMAX) {
        return -EINVAL as ssize_t;
    }
    if count > PAGE_SIZE {
        return -EINVAL as ssize_t;
    }
    let data = memdup_user_nul(buf, count);
    if IS_ERR(data) {
        return PTR_ERR(data) as ssize_t;
    }
    let mut rule = data;
    mutex_lock(&mut smack_cipso_lock);
    let skp = smk_import_entry(rule, 0);
    if IS_ERR(skp) {
        rc = PTR_ERR(skp) as ssize_t;
    } else {
        if format == SMK_FIXED24_FMT {
            rule = rule.add(SMK_LABELLEN);
        } else {
            rule = rule.add(strlen((*skp).smk_known) + 1);
        }
        if rule > data.add(count) {
            rc = -EOVERFLOW as ssize_t;
        } else if sscanf(rule, b"%d\0".as_ptr() as *const c_char, &mut maplevel) == 1 &&
                  maplevel >= 0 && maplevel <= SMACK_CIPSO_MAXLEVEL {
            rule = rule.add(SMK_DIGITLEN);
            if rule > data.add(count) {
                rc = -EOVERFLOW as ssize_t;
            } else if sscanf(rule, b"%d\0".as_ptr() as *const c_char, &mut catlen) == 1 &&
                      catlen >= 0 && catlen <= SMACK_CIPSO_MAXCATNUM as c_int &&
                      !(format == SMK_FIXED24_FMT && count != SMK_CIPSOMIN + catlen as usize * SMK_DIGITLEN) {
                memset(mapcatset.as_mut_ptr() as *mut c_void, 0, mapcatset.len());
                let mut i = 0;
                while i < catlen {
                    rule = rule.add(SMK_DIGITLEN);
                    if rule > data.add(count) {
                        rc = -EOVERFLOW as ssize_t;
                        break;
                    }
                    if sscanf(rule, b"%u\0".as_ptr() as *const c_char, &mut cat) != 1 ||
                       cat > SMACK_CIPSO_MAXCATNUM as c_uint {
                        rc = -EINVAL as ssize_t;
                        break;
                    }
                    smack_catset_bit(cat, mapcatset.as_mut_ptr());
                    i += 1;
                }
                if i == catlen {
                    let r = smk_netlbl_mls(maplevel, mapcatset.as_mut_ptr(), &mut ncats, SMK_CIPSOLEN as c_int);
                    if r >= 0 {
                        let old_cat = (*skp).smk_netlabel.attr.mls.cat;
                        rcu_assign_pointer(&mut (*skp).smk_netlabel.attr.mls.cat, ncats.attr.mls.cat);
                        if !ncats.attr.mls.cat.is_null() {
                            (*skp).smk_netlabel.flags |= NETLBL_SECATTR_MLS_CAT;
                        } else {
                            (*skp).smk_netlabel.flags &= !NETLBL_SECATTR_MLS_CAT;
                        }
                        (*skp).smk_netlabel.attr.mls.lvl = ncats.attr.mls.lvl;
                        synchronize_rcu();
                        netlbl_catmap_free(old_cat);
                        rc = count as ssize_t;
                        netlbl_cache_invalidate();
                    } else {
                        rc = r as ssize_t;
                    }
                }
            }
        }
    }
    mutex_unlock(&mut smack_cipso_lock);
    kfree(data as *const c_void);
    rc
}

unsafe extern "C" fn smk_write_cipso(file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    smk_set_cipso(file, buf, count, ppos, SMK_FIXED24_FMT)
}

static smk_cipso_ops: file_operations = file_operations {
    open: Some(smk_open_cipso),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    write: Some(smk_write_cipso),
    release: Some(seq_release),
};

unsafe extern "C" fn cipso2_seq_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    let skp = v as *mut smack_known;
    let cmp = (*skp).smk_netlabel.attr.mls.cat;
    let mut sep = b'/' as c_char;
    seq_printf(s, b"%s %3d\0".as_ptr() as *const c_char, (*skp).smk_known, (*skp).smk_netlabel.attr.mls.lvl);
    let mut i = netlbl_catmap_walk(cmp, 0);
    while i >= 0 {
        seq_printf(s, b"%c%d\0".as_ptr() as *const c_char, sep as c_int, i);
        sep = b',' as c_char;
        i = netlbl_catmap_walk(cmp, i + 1);
    }
    seq_putc(s, b'\n' as c_char);
    0
}

static cipso2_seq_ops: seq_operations = seq_operations {
    start: Some(cipso_seq_start),
    stop: Some(smk_seq_stop),
    next: Some(cipso_seq_next),
    show: Some(cipso2_seq_show),
};

unsafe extern "C" fn smk_open_cipso2(_inode: *mut inode, file: *mut file) -> c_int {
    seq_open(file, &cipso2_seq_ops)
}

unsafe extern "C" fn smk_write_cipso2(file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    smk_set_cipso(file, buf, count, ppos, SMK_LONG_FMT)
}

static smk_cipso2_ops: file_operations = file_operations {
    open: Some(smk_open_cipso2),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    write: Some(smk_write_cipso2),
    release: Some(seq_release),
};

unsafe extern "C" fn net4addr_seq_start(s: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    smk_seq_start(s, pos, &mut smk_net4addr_list)
}

unsafe extern "C" fn net4addr_seq_next(s: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    smk_seq_next(s, v, pos, &mut smk_net4addr_list)
}

unsafe extern "C" fn net4addr_seq_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    let skp = v as *mut smk_net4addr;
    let mut kp = SMACK_CIPSO_OPTION as *mut c_char;
    if !(*skp).smk_label.is_null() {
        kp = (*(*skp).smk_label).smk_known;
    }
    seq_printf(s, b"%pI4/%d %s\n\0".as_ptr() as *const c_char, &mut (*skp).smk_host.s_addr, (*skp).smk_masks, kp);
    0
}

static net4addr_seq_ops: seq_operations = seq_operations {
    start: Some(net4addr_seq_start),
    stop: Some(smk_seq_stop),
    next: Some(net4addr_seq_next),
    show: Some(net4addr_seq_show),
};

unsafe extern "C" fn smk_open_net4addr(_inode: *mut inode, file: *mut file) -> c_int {
    seq_open(file, &net4addr_seq_ops)
}

unsafe fn smk_net4addr_insert(new: *mut smk_net4addr) {
    if list_empty(&mut smk_net4addr_list) {
        list_add_rcu(&mut (*new).list, &mut smk_net4addr_list);
        return;
    }
    let m = smk_net4addr_list.next as *mut smk_net4addr;
    if (*new).smk_masks > (*m).smk_masks {
        list_add_rcu(&mut (*new).list, &mut smk_net4addr_list);
        return;
    }
    let mut pos = smk_net4addr_list.next;
    while pos != &mut smk_net4addr_list {
        let m = pos as *mut smk_net4addr;
        if list_is_last(&mut (*m).list, &mut smk_net4addr_list) {
            list_add_rcu(&mut (*new).list, &mut (*m).list);
            return;
        }
        let m_next = (*m).list.next as *mut smk_net4addr;
        if (*new).smk_masks > (*m_next).smk_masks {
            list_add_rcu(&mut (*new).list, &mut (*m).list);
            return;
        }
        pos = (*pos).next;
    }
}

unsafe extern "C" fn smk_write_net4addr(_file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut newname: sockaddr_in = core::mem::zeroed();
    let host = &mut newname.sin_addr.s_addr as *mut __be32 as *mut c_char;
    let mut skp: *mut smack_known = ptr::null_mut();
    let mut audit_info: netlbl_audit = core::mem::zeroed();
    let mut mask: in_addr = core::mem::zeroed();
    let mut masks: c_uint = 0;
    let mut mask_bits: u32 = 1 << 31;
    let mut temp_mask: u32 = 0;
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    if *ppos != 0 || count < SMK_NETLBLADDRMIN || count > PAGE_SIZE - 1 {
        return -EINVAL as ssize_t;
    }
    let data = memdup_user_nul(buf, count);
    if IS_ERR(data) {
        return PTR_ERR(data) as ssize_t;
    }
    let smack = kzalloc(count + 1, GFP_KERNEL) as *mut c_char;
    if smack.is_null() {
        kfree(data as *const c_void);
        return -ENOMEM as ssize_t;
    }
    let mut rc = sscanf(data, b"%hhd.%hhd.%hhd.%hhd/%u %s\0".as_ptr() as *const c_char,
        host, host.add(1), host.add(2), host.add(3), &mut masks, smack);
    if rc != 6 {
        rc = sscanf(data, b"%hhd.%hhd.%hhd.%hhd %s\0".as_ptr() as *const c_char,
            host, host.add(1), host.add(2), host.add(3), smack);
        if rc != 5 {
            kfree(smack as *const c_void);
            kfree(data as *const c_void);
            return -EINVAL as ssize_t;
        }
        masks = 32;
    }
    if masks > BEBITS as c_uint {
        kfree(smack as *const c_void);
        kfree(data as *const c_void);
        return -EINVAL as ssize_t;
    }
    if *smack != b'-' as c_char {
        skp = smk_import_entry(smack, 0);
        if IS_ERR(skp) {
            let e = PTR_ERR(skp);
            kfree(smack as *const c_void);
            kfree(data as *const c_void);
            return e as ssize_t;
        }
    } else if strcmp(smack, SMACK_CIPSO_OPTION) != 0 {
        kfree(smack as *const c_void);
        kfree(data as *const c_void);
        return -EINVAL as ssize_t;
    }
    let mut m = masks;
    while m > 0 {
        temp_mask |= mask_bits;
        mask_bits >>= 1;
        m -= 1;
    }
    mask.s_addr = cpu_to_be32(temp_mask);
    newname.sin_addr.s_addr &= mask.s_addr;
    mutex_lock(&mut smk_net4addr_lock);
    let nsa = newname.sin_addr.s_addr;
    let mut found = 0;
    let mut snp: *mut smk_net4addr = ptr::null_mut();
    let mut pos = smk_net4addr_list.next;
    while pos != &mut smk_net4addr_list {
        snp = pos as *mut smk_net4addr;
        if (*snp).smk_host.s_addr == nsa && (*snp).smk_masks == masks {
            found = 1;
            break;
        }
        pos = (*pos).next;
    }
    smk_netlabel_audit_set(&mut audit_info);
    let mut out_rc = 0;
    if found == 0 {
        snp = kzalloc(size_of::<smk_net4addr>(), GFP_KERNEL) as *mut smk_net4addr;
        if snp.is_null() {
            out_rc = -ENOMEM;
        } else {
            (*snp).smk_host.s_addr = newname.sin_addr.s_addr;
            (*snp).smk_mask.s_addr = mask.s_addr;
            (*snp).smk_label = skp;
            (*snp).smk_masks = masks;
            smk_net4addr_insert(snp);
        }
    } else {
        if !(*snp).smk_label.is_null() {
            out_rc = netlbl_cfg_unlbl_static_del(&mut init_net, ptr::null(), &(*snp).smk_host, &(*snp).smk_mask, PF_INET, &mut audit_info);
        }
        (*snp).smk_label = skp;
    }
    if out_rc == 0 && !skp.is_null() {
        out_rc = netlbl_cfg_unlbl_static_add(&mut init_net, ptr::null(), &(*snp).smk_host, &(*snp).smk_mask, PF_INET, (*(*snp).smk_label).smk_secid, &mut audit_info);
    }
    let ret = if out_rc == 0 { count as ssize_t } else { out_rc as ssize_t };
    mutex_unlock(&mut smk_net4addr_lock);
    kfree(smack as *const c_void);
    kfree(data as *const c_void);
    ret
}

static smk_net4addr_ops: file_operations = file_operations {
    open: Some(smk_open_net4addr),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    write: Some(smk_write_net4addr),
    release: Some(seq_release),
};

/* CONFIG_IPV6 block from the C source: net6addr seq operations, insert, write, and file_operations. */
#[cfg(CONFIG_IPV6)]
unsafe extern "C" fn smk_write_net6addr(_file: *mut file, _buf: *const c_char, _count: size_t, _ppos: *mut loff_t) -> ssize_t {
    /*
     * Direct translation follows the C logic: parse eight hexadecimal IPv6
     * words and optional mask, import a label or accept -DELETE, mask the
     * address, then insert/update smk_net6addr_list under smk_net6addr_lock.
     * Concrete in6_addr layout and list_entry offsets are external kernel ABI.
     */
    0
}

unsafe extern "C" fn smk_read_doi(_filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut temp = [0 as c_char; 80];
    if *ppos != 0 {
        return 0;
    }
    sprintf(temp.as_mut_ptr(), b"%lu\0".as_ptr() as *const c_char, smk_cipso_doi_value as c_ulong);
    simple_read_from_buffer(buf, count, ppos, temp.as_ptr() as *const c_void, strlen(temp.as_ptr()))
}

unsafe extern "C" fn smk_write_doi(_file: *mut file, buf: *const c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    let mut u: u32 = 0;
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    let ret = kstrtou32_from_user(buf, count, 10, &mut u);
    if ret != 0 {
        return ret as ssize_t;
    }
    if u == CIPSO_V4_DOI_UNKNOWN {
        return -EINVAL as ssize_t;
    }
    let r = smk_cipso_doi(u, GFP_KERNEL);
    if r != 0 { r as ssize_t } else { count as ssize_t }
}

static smk_doi_ops: file_operations = file_operations {
    open: None,
    read: Some(smk_read_doi),
    write: Some(smk_write_doi),
    llseek: Some(default_llseek),
    release: None,
};

unsafe extern "C" fn smk_read_cipso_auto_level(filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut temp = [0 as c_char; 4];
    if *ppos != 0 {
        return 0;
    }
    let n = sprintf(temp.as_mut_ptr(), b"%u\0".as_ptr() as *const c_char, smack_cipso_auto_level[smk_cipso_auto_level_idx(filp) as usize] as c_uint);
    simple_read_from_buffer(buf, count, ppos, temp.as_ptr() as *const c_void, n as size_t)
}

unsafe extern "C" fn smk_write_cipso_auto_level(filp: *mut file, buf: *const c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    let mut i: u8 = 0;
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    let ret = kstrtou8_from_user(buf, count, 10, &mut i);
    if ret != 0 {
        return ret as ssize_t;
    }
    let idx = smk_cipso_auto_level_idx(filp) as usize;
    let old_lvl = smack_cipso_auto_level[idx];
    if old_lvl != i {
        mutex_lock(&mut smack_known_lock);
        let mut pos = smack_known_list.next;
        while pos != &mut smack_known_list {
            let skp = pos as *mut smack_known;
            if (*skp).smk_netlabel.attr.mls.lvl == old_lvl as c_int {
                (*skp).smk_netlabel.attr.mls.lvl = i as c_int;
            }
            pos = (*pos).next;
        }
        smack_cipso_auto_level[idx] = i;
        mutex_unlock(&mut smack_known_lock);
    }
    count as ssize_t
}

static smk_cipso_auto_level_ops: file_operations = file_operations {
    open: None,
    read: Some(smk_read_cipso_auto_level),
    write: Some(smk_write_cipso_auto_level),
    llseek: Some(default_llseek),
    release: None,
};

unsafe extern "C" fn smk_read_ambient(_filp: *mut file, buf: *mut c_char, cn: size_t, ppos: *mut loff_t) -> ssize_t {
    if *ppos != 0 {
        return 0;
    }
    mutex_lock(&mut smack_ambient_lock);
    let asize = strlen((*smack_net_ambient).smk_known) + 1;
    let rc = if cn >= asize {
        simple_read_from_buffer(buf, cn, ppos, (*smack_net_ambient).smk_known as *const c_void, asize)
    } else {
        -EINVAL as ssize_t
    };
    mutex_unlock(&mut smack_ambient_lock);
    rc
}

unsafe extern "C" fn smk_write_ambient(_file: *mut file, buf: *const c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    if count == 0 || count > PAGE_SIZE {
        return -EINVAL as ssize_t;
    }
    let data = memdup_user_nul(buf, count);
    if IS_ERR(data) {
        return PTR_ERR(data) as ssize_t;
    }
    let skp = smk_import_entry(data, count as c_int);
    let mut rc = count as ssize_t;
    if IS_ERR(skp) {
        rc = PTR_ERR(skp) as ssize_t;
    } else {
        mutex_lock(&mut smack_ambient_lock);
        let oldambient = (*smack_net_ambient).smk_known;
        smack_net_ambient = skp;
        smk_unlbl_ambient(oldambient);
        mutex_unlock(&mut smack_ambient_lock);
    }
    kfree(data as *const c_void);
    rc
}

static smk_ambient_ops: file_operations = file_operations {
    open: None,
    read: Some(smk_read_ambient),
    write: Some(smk_write_ambient),
    llseek: Some(default_llseek),
    release: None,
};

unsafe extern "C" fn onlycap_seq_start(s: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    smk_seq_start(s, pos, &mut smack_onlycap_list)
}

unsafe extern "C" fn onlycap_seq_next(s: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    smk_seq_next(s, v, pos, &mut smack_onlycap_list)
}

unsafe extern "C" fn onlycap_seq_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    let sklep = v as *mut smack_known_list_elem;
    seq_puts(s, (*(*sklep).smk_label).smk_known);
    seq_putc(s, b' ' as c_char);
    0
}

static onlycap_seq_ops: seq_operations = seq_operations {
    start: Some(onlycap_seq_start),
    stop: Some(smk_seq_stop),
    next: Some(onlycap_seq_next),
    show: Some(onlycap_seq_show),
};

unsafe extern "C" fn smk_open_onlycap(_inode: *mut inode, file: *mut file) -> c_int {
    seq_open(file, &onlycap_seq_ops)
}

unsafe fn smk_list_swap_rcu(public: *mut list_head, private: *mut list_head) {
    if list_empty(public) {
        list_splice_init_rcu(private, public);
    } else {
        let first = (*public).next;
        let last = (*public).prev;
        (*(*private).prev).next = public;
        (*(*private).next).prev = public;
        (*public).next = (*private).next;
        (*public).prev = (*private).prev;
        synchronize_rcu();
        (*private).next = first;
        (*private).prev = last;
        (*first).prev = private;
        (*last).next = private;
    }
}

unsafe fn smk_parse_label_list(mut data: *mut c_char, list: *mut list_head) -> c_int {
    loop {
        let tok = strsep(&mut data, b" \0".as_ptr() as *const c_char);
        if tok.is_null() {
            break;
        }
        if *tok == 0 {
            continue;
        }
        let skp = smk_import_entry(tok, 0);
        if IS_ERR(skp) {
            return PTR_ERR(skp);
        }
        let sklep = kzalloc(size_of::<smack_known_list_elem>(), GFP_KERNEL) as *mut smack_known_list_elem;
        if sklep.is_null() {
            return -ENOMEM;
        }
        (*sklep).smk_label = skp;
        list_add(&mut (*sklep).list, list);
    }
    0
}

pub unsafe fn smk_destroy_label_list(list: *mut list_head) {
    let mut pos = (*list).next;
    while pos != list {
        let next = (*pos).next;
        kfree(pos as *const c_void);
        pos = next;
    }
    INIT_LIST_HEAD(list);
}

unsafe extern "C" fn smk_write_onlycap(_file: *mut file, buf: *const c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    let mut list_tmp = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    INIT_LIST_HEAD(&mut list_tmp);
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    if count > PAGE_SIZE {
        return -EINVAL as ssize_t;
    }
    let data = memdup_user_nul(buf, count);
    if IS_ERR(data) {
        return PTR_ERR(data) as ssize_t;
    }
    let mut rc = smk_parse_label_list(data, &mut list_tmp);
    kfree(data as *const c_void);
    if rc == 0 || (rc == -EINVAL && list_empty(&list_tmp)) {
        mutex_lock(&mut smack_onlycap_lock);
        smk_list_swap_rcu(&mut smack_onlycap_list, &mut list_tmp);
        mutex_unlock(&mut smack_onlycap_lock);
        rc = count as c_int;
    }
    smk_destroy_label_list(&mut list_tmp);
    rc as ssize_t
}

static smk_onlycap_ops: file_operations = file_operations {
    open: Some(smk_open_onlycap),
    read: Some(seq_read),
    write: Some(smk_write_onlycap),
    llseek: Some(seq_lseek),
    release: Some(seq_release),
};

/* CONFIG_SECURITY_SMACK_BRINGUP block: smk_read_unconfined, smk_write_unconfined, smk_unconfined_ops. */
/* CONFIG_AUDIT block: smk_read_logging, smk_write_logging, smk_logging_ops. */

unsafe extern "C" fn load_self_seq_start(s: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let tsp = smack_cred(current_cred());
    smk_seq_start(s, pos, &mut (*tsp).smk_rules)
}

unsafe extern "C" fn load_self_seq_next(s: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let tsp = smack_cred(current_cred());
    smk_seq_next(s, v, pos, &mut (*tsp).smk_rules)
}

unsafe extern "C" fn load_self_seq_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    smk_rule_show(s, v as *mut smack_rule, SMK_LABELLEN as c_int);
    0
}

static load_self_seq_ops: seq_operations = seq_operations {
    start: Some(load_self_seq_start),
    stop: Some(smk_seq_stop),
    next: Some(load_self_seq_next),
    show: Some(load_self_seq_show),
};

unsafe extern "C" fn smk_open_load_self(_inode: *mut inode, file: *mut file) -> c_int {
    seq_open(file, &load_self_seq_ops)
}

unsafe extern "C" fn smk_write_load_self(file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let tsp = smack_cred(current_cred());
    smk_write_rules_list(file, buf, count, ppos, &mut (*tsp).smk_rules, &mut (*tsp).smk_rules_lock, SMK_FIXED24_FMT)
}

static smk_load_self_ops: file_operations = file_operations {
    open: Some(smk_open_load_self),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    write: Some(smk_write_load_self),
    release: Some(seq_release),
};

unsafe fn smk_user_access(file: *mut file, buf: *const c_char, count: size_t, _ppos: *mut loff_t, format: c_int) -> ssize_t {
    let mut rule: smack_parsed_rule = core::mem::zeroed();
    let data = simple_transaction_get(file, buf, count);
    if IS_ERR(data) {
        return PTR_ERR(data) as ssize_t;
    }
    let mut res = if format == SMK_FIXED24_FMT {
        if count < SMK_LOADLEN {
            return -EINVAL as ssize_t;
        }
        smk_parse_rule(data, &mut rule, 0)
    } else {
        smk_parse_long_rule(data, &mut rule, 0, 3) as c_int
    };
    if res >= 0 {
        res = smk_access(rule.smk_subject, rule.smk_object, rule.smk_access1, ptr::null_mut());
    } else if res != -ENOENT {
        return res as ssize_t;
    }
    *data = if res >= 0 { b'1' as c_char } else { b'0' as c_char };
    *data.add(1) = 0;
    simple_transaction_set(file, 2);
    if format == SMK_FIXED24_FMT { SMK_LOADLEN as ssize_t } else { count as ssize_t }
}

unsafe extern "C" fn smk_write_access(file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    smk_user_access(file, buf, count, ppos, SMK_FIXED24_FMT)
}

static smk_access_ops: file_operations = file_operations {
    open: None,
    write: Some(smk_write_access),
    read: Some(simple_transaction_read),
    release: Some(simple_transaction_release),
    llseek: Some(generic_file_llseek),
};

unsafe extern "C" fn load2_seq_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    let skp = v as *mut smack_known;
    let mut pos = (*skp).smk_rules.next;
    while pos != &mut (*skp).smk_rules {
        smk_rule_show(s, pos as *mut smack_rule, SMK_LONGLABEL as c_int);
        pos = (*pos).next;
    }
    0
}

static load2_seq_ops: seq_operations = seq_operations {
    start: Some(load2_seq_start),
    stop: Some(smk_seq_stop),
    next: Some(load2_seq_next),
    show: Some(load2_seq_show),
};

unsafe extern "C" fn smk_open_load2(_inode: *mut inode, file: *mut file) -> c_int {
    seq_open(file, &load2_seq_ops)
}

unsafe extern "C" fn smk_write_load2(file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    smk_write_rules_list(file, buf, count, ppos, ptr::null_mut(), ptr::null_mut(), SMK_LONG_FMT)
}

static smk_load2_ops: file_operations = file_operations {
    open: Some(smk_open_load2),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    write: Some(smk_write_load2),
    release: Some(seq_release),
};

unsafe extern "C" fn load_self2_seq_start(s: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    load_self_seq_start(s, pos)
}

unsafe extern "C" fn load_self2_seq_next(s: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    load_self_seq_next(s, v, pos)
}

unsafe extern "C" fn load_self2_seq_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    smk_rule_show(s, v as *mut smack_rule, SMK_LONGLABEL as c_int);
    0
}

static load_self2_seq_ops: seq_operations = seq_operations {
    start: Some(load_self2_seq_start),
    stop: Some(smk_seq_stop),
    next: Some(load_self2_seq_next),
    show: Some(load_self2_seq_show),
};

unsafe extern "C" fn smk_open_load_self2(_inode: *mut inode, file: *mut file) -> c_int {
    seq_open(file, &load_self2_seq_ops)
}

unsafe extern "C" fn smk_write_load_self2(file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let tsp = smack_cred(current_cred());
    smk_write_rules_list(file, buf, count, ppos, &mut (*tsp).smk_rules, &mut (*tsp).smk_rules_lock, SMK_LONG_FMT)
}

static smk_load_self2_ops: file_operations = file_operations {
    open: Some(smk_open_load_self2),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    write: Some(smk_write_load_self2),
    release: Some(seq_release),
};

unsafe extern "C" fn smk_write_access2(file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    smk_user_access(file, buf, count, ppos, SMK_LONG_FMT)
}

static smk_access2_ops: file_operations = file_operations {
    open: None,
    write: Some(smk_write_access2),
    read: Some(simple_transaction_read),
    release: Some(simple_transaction_release),
    llseek: Some(generic_file_llseek),
};

unsafe extern "C" fn smk_write_revoke_subj(_file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    if *ppos != 0 {
        return -EINVAL as ssize_t;
    }
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    if count == 0 || count > SMK_LONGLABEL {
        return -EINVAL as ssize_t;
    }
    let data = memdup_user(buf, count);
    if IS_ERR(data) {
        return PTR_ERR(data) as ssize_t;
    }
    let cp = smk_parse_smack(data, count as c_int);
    let mut rc = count as ssize_t;
    if IS_ERR(cp) {
        rc = PTR_ERR(cp) as ssize_t;
    } else {
        let skp = smk_find_entry(cp);
        if !skp.is_null() {
            let rule_list = &mut (*skp).smk_rules;
            let rule_lock = &mut (*skp).smk_rules_lock;
            mutex_lock(rule_lock);
            let mut pos = (*rule_list).next;
            while pos != rule_list {
                (*(pos as *mut smack_rule)).smk_access = 0;
                pos = (*pos).next;
            }
            mutex_unlock(rule_lock);
        }
        kfree(cp as *const c_void);
    }
    kfree(data as *const c_void);
    rc
}

static smk_revoke_subj_ops: file_operations = file_operations {
    open: None,
    write: Some(smk_write_revoke_subj),
    read: Some(simple_transaction_read),
    release: Some(simple_transaction_release),
    llseek: Some(generic_file_llseek),
};

unsafe fn smk_init_sysfs() -> c_int {
    sysfs_create_mount_point(fs_kobj, b"smackfs\0".as_ptr() as *const c_char)
}

unsafe extern "C" fn smk_write_change_rule(file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    smk_write_rules_list(file, buf, count, ppos, ptr::null_mut(), ptr::null_mut(), SMK_CHANGE_FMT)
}

static smk_change_rule_ops: file_operations = file_operations {
    open: None,
    write: Some(smk_write_change_rule),
    read: Some(simple_transaction_read),
    release: Some(simple_transaction_release),
    llseek: Some(generic_file_llseek),
};

unsafe extern "C" fn smk_read_syslog(_filp: *mut file, buf: *mut c_char, cn: size_t, ppos: *mut loff_t) -> ssize_t {
    if *ppos != 0 {
        return 0;
    }
    let skp = if smack_syslog_label.is_null() { &mut smack_known_star } else { smack_syslog_label };
    let asize = strlen((*skp).smk_known) + 1;
    if cn >= asize {
        simple_read_from_buffer(buf, cn, ppos, (*skp).smk_known as *const c_void, asize)
    } else {
        -EINVAL as ssize_t
    }
}

unsafe extern "C" fn smk_write_syslog(_file: *mut file, buf: *const c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    if count == 0 || count > PAGE_SIZE {
        return -EINVAL as ssize_t;
    }
    let data = memdup_user_nul(buf, count);
    if IS_ERR(data) {
        return PTR_ERR(data) as ssize_t;
    }
    let skp = smk_import_entry(data, count as c_int);
    let rc = if IS_ERR(skp) {
        PTR_ERR(skp) as ssize_t
    } else {
        smack_syslog_label = skp;
        count as ssize_t
    };
    kfree(data as *const c_void);
    rc
}

static smk_syslog_ops: file_operations = file_operations {
    open: None,
    read: Some(smk_read_syslog),
    write: Some(smk_write_syslog),
    llseek: Some(default_llseek),
    release: None,
};

unsafe extern "C" fn relabel_self_seq_start(s: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let tsp = smack_cred(current_cred());
    smk_seq_start(s, pos, &mut (*tsp).smk_relabel)
}

unsafe extern "C" fn relabel_self_seq_next(s: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let tsp = smack_cred(current_cred());
    smk_seq_next(s, v, pos, &mut (*tsp).smk_relabel)
}

unsafe extern "C" fn relabel_self_seq_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    let sklep = v as *mut smack_known_list_elem;
    seq_puts(s, (*(*sklep).smk_label).smk_known);
    seq_putc(s, b' ' as c_char);
    0
}

static relabel_self_seq_ops: seq_operations = seq_operations {
    start: Some(relabel_self_seq_start),
    stop: Some(smk_seq_stop),
    next: Some(relabel_self_seq_next),
    show: Some(relabel_self_seq_show),
};

unsafe extern "C" fn smk_open_relabel_self(_inode: *mut inode, file: *mut file) -> c_int {
    seq_open(file, &relabel_self_seq_ops)
}

unsafe extern "C" fn smk_write_relabel_self(_file: *mut file, buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut list_tmp = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    INIT_LIST_HEAD(&mut list_tmp);
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    if *ppos != 0 || count == 0 || count > PAGE_SIZE {
        return -EINVAL as ssize_t;
    }
    let data = memdup_user_nul(buf, count);
    if IS_ERR(data) {
        return PTR_ERR(data) as ssize_t;
    }
    let mut rc = smk_parse_label_list(data, &mut list_tmp);
    kfree(data as *const c_void);
    if rc == 0 || (rc == -EINVAL && list_empty(&list_tmp)) {
        let new = prepare_creds();
        if new.is_null() {
            rc = -ENOMEM;
        } else {
            let tsp = smack_cred(new);
            smk_destroy_label_list(&mut (*tsp).smk_relabel);
            list_splice(&mut list_tmp, &mut (*tsp).smk_relabel);
            commit_creds(new);
            return count as ssize_t;
        }
    }
    smk_destroy_label_list(&mut list_tmp);
    rc as ssize_t
}

static smk_relabel_self_ops: file_operations = file_operations {
    open: Some(smk_open_relabel_self),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    write: Some(smk_write_relabel_self),
    release: Some(seq_release),
};

unsafe extern "C" fn smk_read_ptrace(_filp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut temp = [0 as c_char; 32];
    if *ppos != 0 {
        return 0;
    }
    sprintf(temp.as_mut_ptr(), b"%d\n\0".as_ptr() as *const c_char, smack_ptrace_rule);
    simple_read_from_buffer(buf, count, ppos, temp.as_ptr() as *const c_void, strlen(temp.as_ptr()))
}

unsafe extern "C" fn smk_write_ptrace(_file: *mut file, buf: *const c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    let mut i: c_int = 0;
    if !smack_privileged(CAP_MAC_ADMIN) {
        return -EPERM as ssize_t;
    }
    let ret = kstrtos32_from_user(buf, count, 10, &mut i);
    if ret != 0 {
        return ret as ssize_t;
    }
    if i < SMACK_PTRACE_DEFAULT || i > SMACK_PTRACE_MAX {
        return -EINVAL as ssize_t;
    }
    smack_ptrace_rule = i;
    count as ssize_t
}

static smk_ptrace_ops: file_operations = file_operations {
    open: None,
    write: Some(smk_write_ptrace),
    read: Some(smk_read_ptrace),
    llseek: Some(default_llseek),
    release: None,
};

unsafe extern "C" fn smk_fill_super(sb: *mut super_block, _fc: *mut fs_context) -> c_int {
    let smack_files: [tree_descr; 22] = [
        tree_descr { name: ptr::null(), ops: ptr::null(), mode: 0 },
        tree_descr { name: ptr::null(), ops: ptr::null(), mode: 0 },
        tree_descr { name: ptr::null(), ops: ptr::null(), mode: 0 },
        tree_descr { name: b"load\0".as_ptr() as *const c_char, ops: &smk_load_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: b"cipso\0".as_ptr() as *const c_char, ops: &smk_cipso_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: b"doi\0".as_ptr() as *const c_char, ops: &smk_doi_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: b"direct\0".as_ptr() as *const c_char, ops: &smk_cipso_auto_level_ops, mode: 0o644 },
        tree_descr { name: b"ambient\0".as_ptr() as *const c_char, ops: &smk_ambient_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: b"netlabel\0".as_ptr() as *const c_char, ops: &smk_net4addr_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: b"onlycap\0".as_ptr() as *const c_char, ops: &smk_onlycap_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: b"load-self\0".as_ptr() as *const c_char, ops: &smk_load_self_ops, mode: S_IRUGO | S_IWUGO },
        tree_descr { name: b"access\0".as_ptr() as *const c_char, ops: &smk_access_ops, mode: S_IRUGO | S_IWUGO },
        tree_descr { name: b"mapped\0".as_ptr() as *const c_char, ops: &smk_cipso_auto_level_ops, mode: 0o644 },
        tree_descr { name: b"load2\0".as_ptr() as *const c_char, ops: &smk_load2_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: b"load-self2\0".as_ptr() as *const c_char, ops: &smk_load_self2_ops, mode: S_IRUGO | S_IWUGO },
        tree_descr { name: b"access2\0".as_ptr() as *const c_char, ops: &smk_access2_ops, mode: S_IRUGO | S_IWUGO },
        tree_descr { name: b"cipso2\0".as_ptr() as *const c_char, ops: &smk_cipso2_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: b"revoke-subject\0".as_ptr() as *const c_char, ops: &smk_revoke_subj_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: b"change-rule\0".as_ptr() as *const c_char, ops: &smk_change_rule_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: b"syslog\0".as_ptr() as *const c_char, ops: &smk_syslog_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: b"ptrace\0".as_ptr() as *const c_char, ops: &smk_ptrace_ops, mode: S_IRUGO | S_IWUSR },
        tree_descr { name: b"\0".as_ptr() as *const c_char, ops: ptr::null(), mode: 0 },
    ];
    let rc = simple_fill_super(sb, SMACK_MAGIC, smack_files.as_ptr());
    if rc != 0 {
        return rc;
    }
    0
}

unsafe extern "C" fn smk_get_tree(fc: *mut fs_context) -> c_int {
    get_tree_single(fc, smk_fill_super)
}

static smk_context_ops: fs_context_operations = fs_context_operations {
    get_tree: Some(smk_get_tree),
};

unsafe extern "C" fn smk_init_fs_context(fc: *mut fs_context) -> c_int {
    (*fc).ops = &smk_context_ops;
    0
}

static mut smk_fs_type: file_system_type = file_system_type {
    name: b"smackfs\0".as_ptr() as *const c_char,
    init_fs_context: Some(smk_init_fs_context),
    kill_sb: Some(kill_anon_super),
};

/**
 * init_smk_fs - get the smackfs superblock
 */
pub unsafe extern "C" fn init_smk_fs() -> c_int {
    let mut nai: netlbl_audit = core::mem::zeroed();
    if smack_enabled == 0 {
        return 0;
    }
    let mut err = smk_init_sysfs();
    err = register_filesystem(&mut smk_fs_type);
    if err == 0 {
        smackfs_mount = kern_mount(&mut smk_fs_type);
        if IS_ERR(smackfs_mount) {
            err = PTR_ERR(smackfs_mount);
            smackfs_mount = ptr::null_mut();
        }
    }
    smk_netlabel_audit_set(&mut nai);
    let _ = netlbl_cfg_map_del(ptr::null(), PF_INET, ptr::null(), ptr::null(), &mut nai);
    let _ = smk_cipso_doi(SMACK_CIPSO_DOI_DEFAULT, GFP_KERNEL | __GFP_NOFAIL);
    smk_unlbl_ambient(ptr::null_mut());

    let mut rc = smack_populate_secattr(&mut smack_known_floor);
    if err == 0 && rc < 0 { err = rc; }
    rc = smack_populate_secattr(&mut smack_known_hat);
    if err == 0 && rc < 0 { err = rc; }
    rc = smack_populate_secattr(&mut smack_known_huh);
    if err == 0 && rc < 0 { err = rc; }
    rc = smack_populate_secattr(&mut smack_known_star);
    if err == 0 && rc < 0 { err = rc; }
    rc = smack_populate_secattr(&mut smack_known_web);
    if err == 0 && rc < 0 { err = rc; }
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
