/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2007 Casey Schaufler <casey@schaufler-ca.com>
 *
 * Author:
 *      Casey Schaufler <casey@schaufler-ca.com>
 */

use core::ffi::{c_char, c_int, c_uchar, c_void};

/* Original C dependencies:
 * linux/capability.h, linux/spinlock.h, linux/lsm_hooks.h, linux/in.h,
 * linux/in6.h when CONFIG_IPV6 is enabled, net/netlabel.h, linux/list.h,
 * linux/rculist.h, linux/lsm_audit.h, and linux/msg.h.
 */

pub type u8 = c_uchar;
pub type u32 = u32;
pub type gfp_t = c_uint;
pub type c_uint = u32;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

#[repr(C)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlbl_lsm_secattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct in6_addr {
    pub in6_u: [u8; 16],
}

#[repr(C)]
pub struct sock {
    pub sk_security: *mut c_void,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cred {
    pub security: *mut c_void,
}

#[repr(C)]
pub struct file {
    pub f_security: *mut c_void,
}

#[repr(C)]
pub struct inode {
    pub i_security: *mut c_void,
}

#[repr(C)]
pub struct msg_msg {
    pub security: *mut c_void,
}

#[repr(C)]
pub struct kern_ipc_perm {
    pub security: *mut c_void,
}

#[repr(C)]
pub struct super_block {
    pub s_security: *mut c_void,
}

#[repr(C)]
pub struct key {
    pub security: *mut c_void,
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lsm_network_audit {
    pub sk: *mut sock,
}

#[repr(C)]
pub union common_audit_data_u {
    pub net: *mut lsm_network_audit,
    pub tsk: *mut task_struct,
    pub dentry: *mut dentry,
    pub inode: *mut inode,
    pub path: path,
}

#[repr(C)]
pub struct common_audit_data {
    pub type_: c_char,
    pub smack_audit_data: *mut smack_audit_data,
    pub u: common_audit_data_u,
}

#[repr(C)]
pub struct lsm_blob_sizes {
    pub lbs_cred: usize,
    pub lbs_file: usize,
    pub lbs_inode: usize,
    pub lbs_msg_msg: usize,
    pub lbs_ipc: usize,
    pub lbs_superblock: usize,
    pub lbs_sock: usize,
    pub lbs_key: usize,
}

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

/* Use IPv6 port labeling if IPv6 is enabled and secmarks are not being used. */
#[cfg(all(CONFIG_IPV6, not(CONFIG_SECURITY_SMACK_NETFILTER)))]
pub const SMACK_IPV6_PORT_LABELING: c_int = 1;

#[cfg(all(CONFIG_IPV6, CONFIG_SECURITY_SMACK_NETFILTER))]
pub const SMACK_IPV6_SECMARK_LABELING: c_int = 1;

/*
 * Smack labels were limited to 23 characters for a long time.
 */
pub const SMK_LABELLEN: c_int = 24;
pub const SMK_LONGLABEL: c_int = 256;

/*
 * This is the repository for labels seen so that it is
 * not necessary to keep allocating tiny chunks of memory
 * and so that they can be shared.
 *
 * Labels are never modified in place. Anytime a label
 * is imported (e.g. xattrset on a file) the list is checked
 * for it and it is added if it doesn't exist. The address
 * is passed out in either case. Entries are added, but
 * never deleted.
 *
 * Since labels are hanging around anyway it doesn't
 * hurt to maintain a secid for those awkward situations
 * where kernel components that ought to use LSM independent
 * interfaces don't. The secid should go away when all of
 * these components have been repaired.
 *
 * The cipso value associated with the label gets stored here, too.
 *
 * Keep the access rules for this subject label here so that
 * the entire set of rules does not need to be examined every
 * time.
 */
#[repr(C)]
pub struct smack_known {
    pub list: list_head,
    pub smk_hashed: hlist_node,
    pub smk_known: *mut c_char,
    pub smk_secid: u32,
    pub smk_netlabel: netlbl_lsm_secattr, /* on wire labels */
    pub smk_rules: list_head,             /* access rules */
    pub smk_rules_lock: mutex,            /* lock for rules */
}

/*
 * Maximum number of bytes for the levels in a CIPSO IP option.
 * Why 23? CIPSO is constrained to 30, so a 32 byte buffer is
 * bigger than can be used, and 24 is the next lower multiple
 * of 8, and there are too many issues if there isn't space set
 * aside for the terminating null byte.
 */
pub const SMK_CIPSOLEN: c_int = 24;

#[repr(C)]
pub struct superblock_smack {
    pub smk_root: *mut smack_known,
    pub smk_floor: *mut smack_known,
    pub smk_hat: *mut smack_known,
    pub smk_default: *mut smack_known,
    pub smk_flags: c_int,
}

/*
 * Superblock flags
 */
pub const SMK_SB_INITIALIZED: c_int = 0x01;
pub const SMK_SB_UNTRUSTED: c_int = 0x02;

#[repr(C)]
pub struct socket_smack {
    pub smk_out: *mut smack_known,    /* outbound label */
    pub smk_in: *mut smack_known,     /* inbound label */
    pub smk_packet: *mut smack_known, /* TCP peer label */
    pub smk_state: c_int,             /* netlabel socket states */
}

pub const SMK_NETLBL_UNSET: c_int = 0;
pub const SMK_NETLBL_UNLABELED: c_int = 1;
pub const SMK_NETLBL_LABELED: c_int = 2;
pub const SMK_NETLBL_REQSKB: c_int = 3;

/*
 * Inode smack data
 */
#[repr(C)]
pub struct inode_smack {
    pub smk_inode: *mut smack_known, /* label of the fso */
    pub smk_task: *mut smack_known,  /* label of the task */
    pub smk_mmap: *mut smack_known,  /* label of the mmap domain */
    pub smk_flags: c_int,            /* smack inode flags */
}

#[repr(C)]
pub struct task_smack {
    pub smk_task: *mut smack_known,       /* label for access control */
    pub smk_forked: *mut smack_known,     /* label when forked */
    pub smk_transmuted: *mut smack_known, /* label when transmuted */
    pub smk_rules: list_head,             /* per task access rules */
    pub smk_rules_lock: mutex,            /* lock for the rules */
    pub smk_relabel: list_head,           /* transit allowed labels */
}

pub const SMK_INODE_INSTANT: c_int = 0x01; /* inode is instantiated */
pub const SMK_INODE_TRANSMUTE: c_int = 0x02; /* directory is transmuting */
pub const SMK_INODE_CHANGED: c_int = 0x04; /* smack was transmuted (unused) */
pub const SMK_INODE_IMPURE: c_int = 0x08; /* involved in an impure transaction */

/*
 * A label access rule.
 */
#[repr(C)]
pub struct smack_rule {
    pub list: list_head,
    pub smk_subject: *mut smack_known,
    pub smk_object: *mut smack_known,
    pub smk_access: c_int,
}

/*
 * An entry in the table identifying IPv4 hosts.
 */
#[repr(C)]
pub struct smk_net4addr {
    pub list: list_head,
    pub smk_host: in_addr,          /* network address */
    pub smk_mask: in_addr,          /* network mask */
    pub smk_masks: c_int,           /* mask size */
    pub smk_label: *mut smack_known, /* label */
}

#[cfg(CONFIG_IPV6)]
/*
 * An entry in the table identifying IPv6 hosts.
 */
#[repr(C)]
pub struct smk_net6addr {
    pub list: list_head,
    pub smk_host: in6_addr,         /* network address */
    pub smk_mask: in6_addr,         /* network mask */
    pub smk_masks: c_int,           /* mask size */
    pub smk_label: *mut smack_known, /* label */
}

#[cfg(SMACK_IPV6_PORT_LABELING)]
/*
 * An entry in the table identifying ports.
 */
#[repr(C)]
pub struct smk_port_label {
    pub list: list_head,
    pub smk_sock: *mut sock,        /* socket initialized on */
    pub smk_port: c_ushort,         /* the port number */
    pub smk_in: *mut smack_known,   /* inbound label */
    pub smk_out: *mut smack_known,  /* outgoing label */
    pub smk_sock_type: c_short,     /* Socket type */
    pub smk_can_reuse: c_short,
}

pub type c_ushort = u16;
pub type c_short = i16;

#[repr(C)]
pub struct smack_known_list_elem {
    pub list: list_head,
    pub smk_label: *mut smack_known,
}

pub const Opt_error: c_int = -1;
pub const Opt_fsdefault: c_int = 0;
pub const Opt_fsfloor: c_int = 1;
pub const Opt_fshat: c_int = 2;
pub const Opt_fsroot: c_int = 3;
pub const Opt_fstransmute: c_int = 4;

pub const SMACK_DELETE_OPTION: &[u8; 8] = b"-DELETE\0";
pub const SMACK_CIPSO_OPTION: &[u8; 7] = b"-CIPSO\0";

/*
 * CIPSO defaults.
 */
pub const SMACK_CIPSO_DOI_DEFAULT: c_int = 3; /* Historical */
pub const SMACK_CIPSO_DOI_INVALID: c_int = -1; /* Not a DOI */
pub const SMACK_CIPSO_DIRECT_DEFAULT: c_int = 250; /* Arbitrary */
pub const SMACK_CIPSO_MAPPED_DEFAULT: c_int = 251; /* Also arbitrary */
pub const SMACK_CIPSO_MAXLEVEL: c_int = 255; /* CIPSO 2.2 standard */
/*
 * CIPSO 2.2 standard is 239, but Smack wants to use the
 * categories in a structured way that limits the value to
 * the bits in 23 bytes, hence the unusual number.
 */
pub const SMACK_CIPSO_MAXCATNUM: c_int = 184; /* 23 * 8 */

/*
 * Ptrace rules
 */
pub const SMACK_PTRACE_DEFAULT: c_int = 0;
pub const SMACK_PTRACE_EXACT: c_int = 1;
pub const SMACK_PTRACE_DRACONIAN: c_int = 2;
pub const SMACK_PTRACE_MAX: c_int = SMACK_PTRACE_DRACONIAN;

/*
 * Flags for untraditional access modes.
 * It shouldn't be necessary to avoid conflicts with definitions
 * in fs.h, but do so anyway.
 */
pub const MAY_TRANSMUTE: c_int = 0x00001000; /* Controls directory labeling */
pub const MAY_LOCK: c_int = 0x00002000; /* Locks should be writes, but ... */
pub const MAY_BRINGUP: c_int = 0x00004000; /* Report use of this rule */

/* External permission bits are supplied by future dependencies. */
extern "C" {
    pub static MAY_APPEND: c_int;
    pub static MAY_WRITE: c_int;
    pub static MAY_READ: c_int;
    pub static MAY_EXEC: c_int;
}

/*
 * The policy for delivering signals is configurable.
 * It is usually "write", but can be "append".
 */
#[cfg(CONFIG_SECURITY_SMACK_APPEND_SIGNALS)]
pub unsafe fn MAY_DELIVER() -> c_int {
    MAY_APPEND /* Signal delivery requires append */
}

#[cfg(not(CONFIG_SECURITY_SMACK_APPEND_SIGNALS))]
pub unsafe fn MAY_DELIVER() -> c_int {
    MAY_WRITE /* Signal delivery requires write */
}

pub const SMACK_BRINGUP_ALLOW: c_int = 1; /* Allow bringup mode */
pub const SMACK_UNCONFINED_SUBJECT: c_int = 2; /* Allow unconfined label */
pub const SMACK_UNCONFINED_OBJECT: c_int = 3; /* Allow unconfined label */

/*
 * Just to make the common cases easier to deal with
 */
pub unsafe fn MAY_ANYREAD() -> c_int {
    MAY_READ | MAY_EXEC
}

pub unsafe fn MAY_READWRITE() -> c_int {
    MAY_READ | MAY_WRITE
}

pub const MAY_NOT: c_int = 0;

/*
 * Number of access types used by Smack (rwxatlb)
 */
pub const SMK_NUM_ACCESS_TYPE: c_int = 7;

/* SMACK data */
#[repr(C)]
pub struct smack_audit_data {
    pub function: *const c_char,
    pub subject: *mut c_char,
    pub object: *mut c_char,
    pub request: *mut c_char,
    pub subj_tsk: *mut task_struct,
    pub result: c_int,
}

/*
 * Smack audit data; is empty if CONFIG_AUDIT not set
 * to save some stack
 */
#[cfg(CONFIG_AUDIT)]
#[repr(C)]
pub struct smk_audit_info {
    pub a: common_audit_data,
    pub sad: smack_audit_data,
}

#[cfg(not(CONFIG_AUDIT))]
#[repr(C)]
pub struct smk_audit_info {
    _private: [u8; 0],
}

extern "C" {
    #[cfg(CONFIG_SECURITY_SMACK_NETFILTER)]
    pub fn smack_nf_ip_init() -> c_int;

    pub fn init_smk_fs() -> c_int;
    pub fn smack_initcall() -> c_int;

    pub fn smk_access_entry(arg1: *mut c_char, arg2: *mut c_char, arg3: *mut list_head) -> c_int;
    pub fn smk_access(
        arg1: *mut smack_known,
        arg2: *mut smack_known,
        arg3: c_int,
        arg4: *mut smk_audit_info,
    ) -> c_int;
    pub fn smk_tskacc(
        arg1: *mut task_smack,
        arg2: *mut smack_known,
        arg3: u32,
        arg4: *mut smk_audit_info,
    ) -> c_int;
    pub fn smk_curacc(arg1: *mut smack_known, arg2: u32, arg3: *mut smk_audit_info) -> c_int;
    pub fn smack_str_from_perm(string: *mut c_char, access: c_int) -> c_int;
    pub fn smack_from_secid(arg1: u32) -> *mut smack_known;
    pub fn smk_parse_label_len(string: *const c_char, len: c_int) -> c_int;
    pub fn smk_parse_smack(string: *const c_char, len: c_int) -> *mut c_char;
    pub fn smk_netlbl_mls(
        arg1: c_int,
        arg2: *mut c_char,
        arg3: *mut netlbl_lsm_secattr,
        arg4: c_int,
    ) -> c_int;
    pub fn smk_import_entry(arg1: *const c_char, arg2: c_int) -> *mut smack_known;
    pub fn smk_import_valid_label(
        label: *const c_char,
        label_len: c_int,
        gfp: gfp_t,
    ) -> *mut smack_known;
    pub fn smk_insert_entry(skp: *mut smack_known);
    pub fn smk_find_entry(arg1: *const c_char) -> *mut smack_known;
    pub fn smack_privileged(cap: c_int) -> bool;
    pub fn smack_privileged_cred(cap: c_int, cred: *const cred) -> bool;
    pub fn smk_destroy_label_list(list: *mut list_head);
    pub fn smack_populate_secattr(skp: *mut smack_known) -> c_int;

    pub static mut smack_enabled: c_int; /* __initdata */
    pub static mut smack_cipso_auto_level: [u8; 2];
    pub static mut smack_net_ambient: *mut smack_known;
    pub static mut smack_syslog_label: *mut smack_known;
    #[cfg(CONFIG_SECURITY_SMACK_BRINGUP)]
    pub static mut smack_unconfined: *mut smack_known;
    pub static mut smack_ptrace_rule: c_int;
    pub static mut smack_blob_sizes: lsm_blob_sizes;

    pub static mut smack_known_floor: smack_known;
    pub static mut smack_known_hat: smack_known;
    pub static mut smack_known_huh: smack_known;
    pub static mut smack_known_star: smack_known;
    pub static mut smack_known_web: smack_known;

    pub static mut smack_known_lock: mutex;
    pub static mut smack_known_list: list_head;
    pub static mut smk_net4addr_list: list_head;
    #[cfg(CONFIG_IPV6)]
    pub static mut smk_net6addr_list: list_head;

    pub static mut smack_onlycap_lock: mutex;
    pub static mut smack_onlycap_list: list_head;

    pub static mut smack_known_hash: [hlist_head; SMACK_HASH_SLOTS as usize];
    pub static mut smack_rule_cache: *mut kmem_cache;

    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn __task_cred(t: *const task_struct) -> *const cred;
    pub fn current_cred() -> *const cred;
    pub fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

pub unsafe fn smack_cipso_direct() -> u8 {
    smack_cipso_auto_level[0]
}

pub unsafe fn smack_cipso_mapped() -> u8 {
    smack_cipso_auto_level[1]
}

pub const SMACK_HASH_SLOTS: c_int = 16;

#[cfg(not(CONFIG_SECURITY_SMACK_NETFILTER))]
pub fn smack_nf_ip_init() -> c_int {
    0
}

pub unsafe fn smack_cred(cred: *const cred) -> *mut task_smack {
    ((*cred).security as *mut u8).add(smack_blob_sizes.lbs_cred) as *mut task_smack
}

pub unsafe fn smack_file(file: *const file) -> *mut *mut smack_known {
    ((*file).f_security as *mut u8).add(smack_blob_sizes.lbs_file) as *mut *mut smack_known
}

pub unsafe fn smack_inode(inode: *const inode) -> *mut inode_smack {
    ((*inode).i_security as *mut u8).add(smack_blob_sizes.lbs_inode) as *mut inode_smack
}

pub unsafe fn smack_msg_msg(msg: *const msg_msg) -> *mut *mut smack_known {
    ((*msg).security as *mut u8).add(smack_blob_sizes.lbs_msg_msg) as *mut *mut smack_known
}

pub unsafe fn smack_ipc(ipc: *const kern_ipc_perm) -> *mut *mut smack_known {
    ((*ipc).security as *mut u8).add(smack_blob_sizes.lbs_ipc) as *mut *mut smack_known
}

pub unsafe fn smack_superblock(superblock: *const super_block) -> *mut superblock_smack {
    ((*superblock).s_security as *mut u8).add(smack_blob_sizes.lbs_superblock)
        as *mut superblock_smack
}

pub unsafe fn smack_sock(sock: *const sock) -> *mut socket_smack {
    ((*sock).sk_security as *mut u8).add(smack_blob_sizes.lbs_sock) as *mut socket_smack
}

#[cfg(CONFIG_KEYS)]
pub unsafe fn smack_key(key: *const key) -> *mut *mut smack_known {
    ((*key).security as *mut u8).add(smack_blob_sizes.lbs_key) as *mut *mut smack_known
}

/*
 * Is the directory transmuting?
 */
pub unsafe fn smk_inode_transmutable(isp: *const inode) -> c_int {
    let sip = smack_inode(isp);
    (((*sip).smk_flags & SMK_INODE_TRANSMUTE) != 0) as c_int
}

/*
 * Present a pointer to the smack label entry in an inode blob.
 */
pub unsafe fn smk_of_inode(isp: *const inode) -> *mut smack_known {
    let sip = smack_inode(isp);
    (*sip).smk_inode
}

/*
 * Present a pointer to the smack label entry in an task blob.
 */
pub unsafe fn smk_of_task(tsp: *const task_smack) -> *mut smack_known {
    (*tsp).smk_task
}

pub unsafe fn smk_of_task_struct_obj(t: *const task_struct) -> *mut smack_known {
    let skp: *mut smack_known;
    let cred: *const cred;

    rcu_read_lock();

    cred = __task_cred(t);
    skp = smk_of_task(smack_cred(cred));

    rcu_read_unlock();

    skp
}

/*
 * Present a pointer to the forked smack label entry in an task blob.
 */
pub unsafe fn smk_of_forked(tsp: *const task_smack) -> *mut smack_known {
    (*tsp).smk_forked
}

/*
 * Present a pointer to the smack label in the current task blob.
 */
pub unsafe fn smk_of_current() -> *mut smack_known {
    smk_of_task(smack_cred(current_cred()))
}

extern "C" {
    pub fn smack_log(
        subject_label: *mut c_char,
        object_label: *mut c_char,
        request: c_int,
        result: c_int,
        auditdata: *mut smk_audit_info,
    );
}

#[cfg(CONFIG_AUDIT)]
pub const SMACK_AUDIT_DENIED: c_int = 0x1;
#[cfg(CONFIG_AUDIT)]
pub const SMACK_AUDIT_ACCEPT: c_int = 0x2;

#[cfg(CONFIG_AUDIT)]
extern "C" {
    pub static mut log_policy: c_int;
}

#[cfg(CONFIG_AUDIT)]
pub unsafe fn smk_ad_init(a: *mut smk_audit_info, func: *const c_char, type_: c_char) {
    memset(
        &mut (*a).sad as *mut smack_audit_data as *mut c_void,
        0,
        core::mem::size_of::<smack_audit_data>(),
    );
    (*a).a.type_ = type_;
    (*a).a.smack_audit_data = &mut (*a).sad;
    (*(*a).a.smack_audit_data).function = func;
}

#[cfg(CONFIG_AUDIT)]
pub unsafe fn smk_ad_init_net(
    a: *mut smk_audit_info,
    func: *const c_char,
    type_: c_char,
    net: *mut lsm_network_audit,
) {
    smk_ad_init(a, func, type_);
    memset(
        net as *mut c_void,
        0,
        core::mem::size_of::<lsm_network_audit>(),
    );
    (*a).a.u.net = net;
}

#[cfg(CONFIG_AUDIT)]
pub unsafe fn smk_ad_setfield_u_tsk(a: *mut smk_audit_info, t: *mut task_struct) {
    (*a).a.u.tsk = t;
}

#[cfg(CONFIG_AUDIT)]
pub unsafe fn smk_ad_setfield_u_fs_path_dentry(a: *mut smk_audit_info, d: *mut dentry) {
    (*a).a.u.dentry = d;
}

#[cfg(CONFIG_AUDIT)]
pub unsafe fn smk_ad_setfield_u_fs_inode(a: *mut smk_audit_info, i: *mut inode) {
    (*a).a.u.inode = i;
}

#[cfg(CONFIG_AUDIT)]
pub unsafe fn smk_ad_setfield_u_fs_path(a: *mut smk_audit_info, p: path) {
    (*a).a.u.path = p;
}

#[cfg(CONFIG_AUDIT)]
pub unsafe fn smk_ad_setfield_u_net_sk(a: *mut smk_audit_info, sk: *mut sock) {
    (*(*a).a.u.net).sk = sk;
}

#[cfg(not(CONFIG_AUDIT))]
pub unsafe fn smk_ad_init(_a: *mut smk_audit_info, _func: *const c_char, _type_: c_char) {}

#[cfg(not(CONFIG_AUDIT))]
pub unsafe fn smk_ad_setfield_u_tsk(_a: *mut smk_audit_info, _t: *mut task_struct) {}

#[cfg(not(CONFIG_AUDIT))]
pub unsafe fn smk_ad_setfield_u_fs_path_dentry(_a: *mut smk_audit_info, _d: *mut dentry) {}

#[cfg(not(CONFIG_AUDIT))]
pub unsafe fn smk_ad_setfield_u_fs_inode(_a: *mut smk_audit_info, _i: *mut inode) {}

#[cfg(not(CONFIG_AUDIT))]
pub unsafe fn smk_ad_setfield_u_fs_path(_a: *mut smk_audit_info, _p: path) {}

#[cfg(not(CONFIG_AUDIT))]
pub unsafe fn smk_ad_setfield_u_net_sk(_a: *mut smk_audit_info, _sk: *mut sock) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
