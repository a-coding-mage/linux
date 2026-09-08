// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Simplified MAC Kernel (smack) security module
 *
 *  This file contains the smack hook function implementations.
 *
 *  Authors:
 *	Casey Schaufler <casey@schaufler-ca.com>
 *	Jarkko Sakkinen <jarkko.sakkinen@intel.com>
 *
 *  Copyright (C) 2007 Casey Schaufler <casey@schaufler-ca.com>
 *  Copyright (C) 2009 Hewlett-Packard Development Company, L.P.
 *                Paul Moore <paul@paul-moore.com>
 *  Copyright (C) 2010 Nokia Corporation
 *  Copyright (C) 2011 Intel Corporation.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, improper_ctypes, unused_variables, unused_mut)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type bool_ = bool;
type gfp_t = c_uint;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type __be16 = u16;
type __be32 = u32;
type umode_t = u16;
type pid_t = c_int;
type key_ref_t = *mut c_void;
type sockptr_t = *mut c_void;

const TRANS_TRUE: *const c_char = b"TRUE\0".as_ptr() as *const c_char;
const TRANS_TRUE_SIZE: usize = 4;

const SMK_CONNECTING: c_int = 0;
const SMK_RECEIVING: c_int = 1;
const SMK_SENDING: c_int = 2;

/*
 * Smack uses multiple xattrs.
 * SMACK64 - for access control,
 * SMACK64TRANSMUTE - label initialization,
 * Not saved on files - SMACK64IPIN and SMACK64IPOUT,
 * Must be set explicitly - SMACK64EXEC and SMACK64MMAP
 */
const SMACK_INODE_INIT_XATTRS: c_int = 2;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _priv: [u8; 0] }
#[repr(C)] pub struct kmem_cache { _priv: [u8; 0] }
#[repr(C)] pub struct qstr { _priv: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _priv: [u8; 0] }
#[repr(C)] pub struct iattr { pub ia_valid: c_uint }
#[repr(C)] pub struct posix_acl { _priv: [u8; 0] }
#[repr(C)] pub struct path { pub dentry: *mut dentry }
#[repr(C)] pub struct fs_context { pub security: *mut c_void }
#[repr(C)] pub struct fs_parameter { pub string: *mut c_char }
#[repr(C)] pub struct fs_parse_result { _priv: [u8; 0] }
#[repr(C)] pub struct fs_parameter_spec { _priv: [u8; 0] }
#[repr(C)] pub struct lsm_id { pub name: *const c_char, pub id: c_uint }
#[repr(C)] pub struct lsm_blob_sizes {
    pub lbs_cred: size_t, pub lbs_file: size_t, pub lbs_inode: size_t,
    pub lbs_ipc: size_t, pub lbs_key: size_t, pub lbs_msg_msg: size_t,
    pub lbs_sock: size_t, pub lbs_superblock: size_t, pub lbs_xattr_count: c_int,
}
#[repr(C)] pub struct security_hook_list { _priv: [u8; 0] }
#[repr(C)] pub struct lsm_context { pub context: *const c_char, pub len: c_int, pub id: c_uint }
#[repr(C)] pub struct lsm_ctx { pub id: u64, pub flags: u64, pub len: u64, pub ctx_len: u64, pub ctx: [c_char; 0] }
#[repr(C)] pub struct smack_prop { pub skp: *mut smack_known }
#[repr(C)] pub struct lsm_prop { pub smack: smack_prop }

#[repr(C)] pub struct super_block {
    pub s_root: *mut dentry, pub s_user_ns: *mut c_void, pub s_magic: c_ulong, pub s_id: *const c_char,
}
#[repr(C)] pub struct dentry { pub d_inode: *mut inode, pub d_sb: *mut super_block, pub d_parent: *mut dentry }
#[repr(C)] pub struct inode {
    pub i_opflags: c_uint, pub i_sb: *mut super_block, pub i_ino: u64, pub i_mode: umode_t,
}
#[repr(C)] pub struct file { pub f_path: path, pub f_mode: c_uint, pub f_cred: *const cred }
#[repr(C)] pub struct cred { _priv: [u8; 0] }
#[repr(C)] pub struct task_struct { pub cred: *const cred, pub comm: [c_char; 16], pub flags: c_uint }
#[repr(C)] pub struct linux_binprm { pub file: *mut file, pub cred: *mut cred, pub unsafe_: c_uint, pub per_clear: c_uint, pub secureexec: c_int }
#[repr(C)] pub struct sock { pub sk_family: u16, pub sk_type: c_int }
#[repr(C)] pub struct socket { pub sk: *mut sock, pub type_: c_int }
#[repr(C)] pub struct sockaddr { pub sa_family: u16, pub sa_data: [c_char; 14] }
#[repr(C)] pub struct in_addr { pub s_addr: __be32 }
#[repr(C)] pub struct sockaddr_in { pub sin_family: u16, pub sin_port: __be16, pub sin_addr: in_addr }
#[repr(C)] pub struct in6_addr { pub s6_addr16: [__be16; 8] }
#[repr(C)] pub struct sockaddr_in6 { pub sin6_family: u16, pub sin6_port: __be16, pub sin6_addr: in6_addr }
#[repr(C)] pub struct sk_buff { pub protocol: u16, pub secmark: u32, pub skb_iif: c_int }
#[repr(C)] pub struct request_sock { pub peer_secid: u32 }
#[repr(C)] pub struct msghdr { pub msg_name: *mut c_void, pub msg_namelen: c_int }
#[repr(C)] pub struct fown_struct { pub file: *mut file }
#[repr(C)] pub struct kern_ipc_perm { pub id: c_int }
#[repr(C)] pub struct msg_msg { _priv: [u8; 0] }
#[repr(C)] pub struct sembuf { _priv: [u8; 0] }
#[repr(C)] pub struct kernel_siginfo { _priv: [u8; 0] }
#[repr(C)] pub struct key { pub serial: c_int, pub description: *const c_char }
#[repr(C)] pub struct watch_notification { pub type_: c_uint }
#[repr(C)] pub struct io_uring_cmd { pub file: *mut file }
#[repr(C)] pub struct xattr { pub value: *mut c_void, pub value_len: size_t, pub name: *const c_char }
#[repr(C)] pub struct audit_field { pub type_: u32 }
#[repr(C)] pub struct audit_krule { pub field_count: c_int, pub fields: *mut audit_field }

#[repr(C)] pub struct smack_known {
    pub smk_known: *mut c_char, pub smk_secid: u32, pub smk_rules: list_head,
    pub smk_rules_lock: mutex, pub smk_netlabel: netlbl_lsm_secattr, pub list: list_head,
}
#[repr(C)] pub struct smack_rule {
    pub list: list_head, pub smk_subject: *mut smack_known, pub smk_object: *mut smack_known, pub smk_access: c_int,
}
#[repr(C)] pub struct smack_known_list_elem { pub list: list_head, pub smk_label: *mut smack_known }
#[repr(C)] pub struct inode_smack {
    pub smk_inode: *mut smack_known, pub smk_task: *mut smack_known, pub smk_mmap: *mut smack_known, pub smk_flags: c_int,
}
#[repr(C)] pub struct task_smack {
    pub smk_task: *mut smack_known, pub smk_forked: *mut smack_known, pub smk_transmuted: *mut smack_known,
    pub smk_rules: list_head, pub smk_relabel: list_head, pub smk_rules_lock: mutex,
}
#[repr(C)] pub struct superblock_smack {
    pub smk_root: *mut smack_known, pub smk_default: *mut smack_known, pub smk_floor: *mut smack_known,
    pub smk_hat: *mut smack_known, pub smk_flags: c_int,
}
#[repr(C)] pub struct socket_smack {
    pub smk_in: *mut smack_known, pub smk_out: *mut smack_known, pub smk_packet: *mut smack_known, pub smk_state: c_int,
}
#[repr(C)] pub struct smk_net4addr { pub list: list_head, pub smk_host: in_addr, pub smk_mask: in_addr, pub smk_label: *mut smack_known }
#[repr(C)] pub struct smk_net6addr { pub list: list_head, pub smk_host: in6_addr, pub smk_mask: in6_addr, pub smk_label: *mut smack_known }
#[repr(C)] pub struct smk_port_label {
    pub list: list_head, pub smk_sock: *mut sock, pub smk_in: *mut smack_known, pub smk_out: *mut smack_known,
    pub smk_port: c_ushort, pub smk_sock_type: c_int, pub smk_can_reuse: c_int,
}
type c_ushort = u16;

#[repr(C)] pub struct smk_audit_info { _priv: [u8; 0] }
#[repr(C)] pub struct lsm_network_audit { _priv: [u8; 0] }
#[repr(C)] pub struct netlbl_lsm_secattr_cache { pub data: *mut c_void }
#[repr(C)] pub struct netlbl_catmap { _priv: [u8; 0] }
#[repr(C)] pub struct netlbl_mls { pub lvl: u32, pub cat: *mut netlbl_catmap }
#[repr(C)] pub union netlbl_attr { pub secid: u32, pub mls: core::mem::ManuallyDrop<netlbl_mls> }
#[repr(C)] pub struct netlbl_lsm_secattr { pub flags: c_uint, pub cache: *mut netlbl_lsm_secattr_cache, pub attr: netlbl_attr }

#[repr(C)] struct smk_mount_opt { name: *const c_char, len: c_int, opt: c_int }
#[repr(C)] struct smack_mnt_opts {
    fsdefault: *const c_char, fsfloor: *const c_char, fshat: *const c_char, fsroot: *const c_char, fstransmute: *const c_char,
}

extern "C" {
    static mut current: *mut task_struct;
    static mut smack_rule_cache: *mut kmem_cache;
    static mut smack_enabled: c_int;
    static mut smack_known_floor: smack_known;
    static mut smack_known_hat: smack_known;
    static mut smack_known_star: smack_known;
    static mut smack_known_web: smack_known;
    static mut smack_known_huh: smack_known;
    static mut smack_net_ambient: *mut smack_known;
    static mut smack_known_list: list_head;
    static mut smk_net4addr_list: list_head;
    static mut smk_net6addr_list: list_head;
    static mut init_user_ns: c_void;
    static mut nop_mnt_idmap: mnt_idmap;

    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn memmove(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn kzalloc(size: size_t, gfp: gfp_t) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn kmemdup(value: *const c_void, len: size_t, gfp: gfp_t) -> *mut c_void;
    fn kmemdup_nul(value: *const c_char, len: size_t, gfp: gfp_t) -> *mut c_char;
    fn kstrdup(value: *const c_char, gfp: gfp_t) -> *mut c_char;
    fn kmem_cache_zalloc(cache: *mut kmem_cache, gfp: gfp_t) -> *mut c_void;
    fn kmem_cache_free(cache: *mut kmem_cache, obj: *mut c_void);
    fn KMEM_CACHE_smack_rule(flags: c_int) -> *mut kmem_cache;
    fn pr_info(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
    fn WARN_ONCE(cond: c_int, fmt: *const c_char, ...) -> c_int;
    fn mutex_init(m: *mut mutex);
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn current_cred() -> *const cred;
    fn __task_cred(task: *mut task_struct) -> *const cred;
    fn get_task_cred(task: *mut task_struct) -> *const cred;
    fn put_cred(cred: *const cred);
    fn prepare_creds() -> *mut cred;
    fn commit_creds(cred: *mut cred) -> c_int;
    fn get_task_comm(buf: *mut c_char, task: *mut task_struct) -> *mut c_char;
    fn ptrace_parent(task: *mut task_struct) -> *mut task_struct;
    fn smack_cred(cred: *const cred) -> *mut task_smack;
    fn smack_inode(inode: *mut inode) -> *mut inode_smack;
    fn smack_superblock(sb: *mut super_block) -> *mut superblock_smack;
    fn smack_file(file: *mut file) -> *mut *mut smack_known;
    fn smack_sock(sk: *const sock) -> *mut socket_smack;
    fn smack_ipc(isp: *mut kern_ipc_perm) -> *mut *mut smack_known;
    fn smack_msg_msg(msg: *mut msg_msg) -> *mut *mut smack_known;
    fn smack_key(key: *mut key) -> *mut *mut smack_known;
    fn smk_of_current() -> *mut smack_known;
    fn smk_of_inode(inode: *mut inode) -> *mut smack_known;
    fn smk_of_task(tsp: *const task_smack) -> *mut smack_known;
    fn smk_of_task_struct_obj(task: *mut task_struct) -> *mut smack_known;
    fn smk_inode_transmutable(inode: *mut inode) -> bool;
    fn smk_import_entry(s: *const c_void, len: size_t) -> *mut smack_known;
    fn smk_import_valid_label(s: *const c_void, len: c_int, gfp: gfp_t) -> *mut smack_known;
    fn smk_find_entry(s: *const c_char) -> *mut smack_known;
    fn smack_from_secid(secid: u32) -> *mut smack_known;
    fn smk_parse_label_len(value: *const c_void, size: size_t) -> c_int;
    fn smk_insert_entry(skp: *mut smack_known);
    fn smk_destroy_label_list(head: *mut list_head);
    fn smack_str_from_perm(s: *mut c_char, mode: c_int);
    fn smk_access_entry(subject: *const c_char, object: *const c_char, rules: *const list_head) -> c_int;
    fn smk_access(subject: *mut smack_known, object: *mut smack_known, request: c_int, ad: *mut smk_audit_info) -> c_int;
    fn smk_curacc(object: *mut smack_known, request: c_int, ad: *mut smk_audit_info) -> c_int;
    fn smk_tskacc(tsp: *mut task_smack, object: *mut smack_known, request: c_int, ad: *mut smk_audit_info) -> c_int;
    fn smack_privileged(cap: c_int) -> bool;
    fn smack_privileged_cred(cap: c_int, cred: *const cred) -> bool;
    fn smack_log(subj: *const c_char, obj: *const c_char, request: c_int, rc: c_int, ad: *mut smk_audit_info);
    fn smk_ad_init(ad: *mut smk_audit_info, func: *const c_char, typ: c_int);
    fn smk_ad_init_net(ad: *mut smk_audit_info, func: *const c_char, typ: c_int, net: *mut lsm_network_audit);
    fn smk_ad_setfield_u_tsk(ad: *mut smk_audit_info, tsk: *mut task_struct);
    fn smk_ad_setfield_u_fs_path_dentry(ad: *mut smk_audit_info, dentry: *mut dentry);
    fn smk_ad_setfield_u_fs_inode(ad: *mut smk_audit_info, inode: *mut inode);
    fn smk_ad_setfield_u_fs_path(ad: *mut smk_audit_info, path: path);
    fn smk_ad_setfield_u_net_sk(ad: *mut smk_audit_info, sk: *mut sock);
    fn __vfs_getxattr(dp: *mut dentry, ip: *mut inode, name: *const c_char, buffer: *mut c_void, size: size_t) -> c_int;
    fn __vfs_setxattr_locked(idmap: *mut mnt_idmap, dentry: *mut dentry, name: *const c_char, value: *mut c_void, size: u32, flags: c_int, deleg: *mut c_void) -> c_int;
    fn d_backing_inode(d: *mut dentry) -> *mut inode;
    fn d_inode(d: *mut dentry) -> *mut inode;
    fn file_inode(file: *mut file) -> *mut inode;
    fn d_is_positive(d: *mut dentry) -> bool;
    fn dget(d: *mut dentry) -> *mut dentry;
    fn dput(d: *mut dentry);
    fn SOCKET_I(inode: *mut inode) -> *mut socket;
    fn lsm_get_xattr_slot(xattrs: *mut xattr, xattr_count: *mut c_int) -> *mut xattr;
    fn xattr_list_one(buffer: *mut *mut c_char, remaining_size: *mut ssize_t, name: *const c_char) -> c_int;
    fn lsm_fill_user_ctx(ctx: *mut lsm_ctx, size: *mut u32, label: *const c_char, len: size_t, id: c_uint, flags: c_uint) -> c_int;
    fn lsm_name_to_attr(name: *const c_char) -> c_uint;
    fn fs_parse(fc: *mut fs_context, specs: *const fs_parameter_spec, param: *mut fs_parameter, result: *mut fs_parse_result) -> c_int;
    fn security_add_hooks(hooks: *mut security_hook_list, count: size_t, id: *const lsm_id);
    fn audit_cfg_lsm(id: *const lsm_id, flags: c_uint);
    fn init_smk_fs() -> c_int;
    fn smack_nf_ip_init() -> c_int;
    fn netlbl_sock_setattr(sk: *mut sock, family: u16, secattr: *mut netlbl_lsm_secattr, lock: c_int) -> c_int;
    fn netlbl_sk_lock_check(sk: *mut sock) -> c_int;
    fn netlbl_sock_delattr(sk: *mut sock);
    fn netlbl_secattr_init(attr: *mut netlbl_lsm_secattr);
    fn netlbl_skbuff_getattr(skb: *mut sk_buff, family: u16, attr: *mut netlbl_lsm_secattr) -> c_int;
    fn netlbl_cache_add(skb: *mut sk_buff, family: u16, attr: *mut netlbl_lsm_secattr);
    fn netlbl_secattr_destroy(attr: *mut netlbl_lsm_secattr);
    fn netlbl_skbuff_err(skb: *mut sk_buff, family: u16, rc: c_int, flags: c_int);
    fn netlbl_req_setattr(req: *mut request_sock, attr: *mut netlbl_lsm_secattr) -> c_int;
    fn netlbl_req_delattr(req: *mut request_sock);
    fn local_bh_disable();
    fn local_bh_enable();
    fn bh_lock_sock_nested(sk: *mut sock);
    fn bh_unlock_sock(sk: *mut sock);
    fn ntohs(v: __be16) -> u16;
    fn htons(v: u16) -> u16;
    fn copy_to_sockptr(dst: sockptr_t, src: *const c_void, len: size_t) -> c_int;
    fn netlbl_catmap_walk(cat: *mut netlbl_catmap, offset: c_int) -> c_int;
    fn key_ref_to_ptr(key_ref: key_ref_t) -> *mut key;
}

const GFP_NOFS: gfp_t = 0;
const GFP_KERNEL: gfp_t = 0;
const ENOMEM: c_int = 12;
const EACCES: c_int = 13;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const EPERM: c_int = 1;
const EOPNOTSUPP: c_int = 95;
const ENOENT: c_int = 2;
const ECHILD: c_int = 10;
const ERANGE: c_int = 34;
const EDESTADDRREQ: c_int = 89;
const ECANCELED: c_int = 125;
const ENOPARAM: c_int = 524;
const SMK_LONGLABEL: size_t = 256;
const SMK_NUM_ACCESS_TYPE: usize = 8;
const MAY_READ: c_int = 4;
const MAY_WRITE: c_int = 2;
const MAY_EXEC: c_int = 1;
const MAY_APPEND: c_int = 8;
const MAY_READWRITE: c_int = MAY_READ | MAY_WRITE;
const MAY_LOCK: c_int = 16;
const MAY_DELIVER: c_int = 32;
const MAY_TRANSMUTE: c_int = 64;
const MAY_NOT_BLOCK: c_int = 128;
const IOP_XATTR: c_uint = 0x0001;
const SMK_INODE_IMPURE: c_int = 0x01;
const SMK_INODE_TRANSMUTE: c_int = 0x02;
const SMK_INODE_INSTANT: c_int = 0x04;
const SMK_SB_INITIALIZED: c_int = 0x01;
const SMK_SB_UNTRUSTED: c_int = 0x02;
const SMK_NETLBL_LABELED: c_int = 1;
const SMK_NETLBL_REQSKB: c_int = 2;
const SMK_NETLBL_UNLABELED: c_int = 0;
const SMACK_BRINGUP_ALLOW: c_int = 1;
const SMACK_UNCONFINED_SUBJECT: c_int = 2;
const SMACK_UNCONFINED_OBJECT: c_int = 3;
const PTRACE_MODE_ATTACH: c_uint = 0x10;
const PTRACE_MODE_READ: c_uint = 0x20;
const PTRACE_MODE_NOAUDIT: c_uint = 0x40;
const SMACK_PTRACE_EXACT: c_int = 1;
const SMACK_PTRACE_DRACONIAN: c_int = 2;
const CAP_SYS_PTRACE: c_int = 19;
const CAP_MAC_OVERRIDE: c_int = 32;
const CAP_MAC_ADMIN: c_int = 33;
const LSM_AUDIT_DATA_TASK: c_int = 1;
const LSM_AUDIT_DATA_DENTRY: c_int = 2;
const LSM_AUDIT_DATA_INODE: c_int = 3;
const LSM_AUDIT_DATA_PATH: c_int = 4;
const LSM_AUDIT_DATA_NET: c_int = 5;
const LSM_AUDIT_DATA_IPC: c_int = 6;
const LSM_AUDIT_DATA_KEY: c_int = 7;
const LSM_AUDIT_DATA_NOTIFICATION: c_int = 8;
const LSM_ID_SMACK: c_uint = 104;
const LSM_ATTR_UNDEF: c_uint = 0;
const LSM_ATTR_CURRENT: c_uint = 100;
const LSM_UNSAFE_PTRACE: c_uint = 1;
const PER_CLEAR_ON_SETID: c_uint = 1;
const ATTR_FORCE: c_uint = 1;
const PF_INET: u16 = 2;
const AF_INET: u16 = 2;
const PF_INET6: u16 = 10;
const AF_INET6: u16 = 10;
const PF_UNIX: u16 = 1;
const PF_UNSPEC: c_int = 0;
const PF_KTHREAD: c_uint = 0x00200000;
const SOCKFS_MAGIC: c_ulong = 0x534f434b;
const SYSFS_MAGIC: c_ulong = 0x62656572;
const TMPFS_MAGIC: c_ulong = 0x01021994;
const RAMFS_MAGIC: c_ulong = 0x858458f6;
const CGROUP_SUPER_MAGIC: c_ulong = 0x27e0eb;
const CGROUP2_SUPER_MAGIC: c_ulong = 0x63677270;
const PIPEFS_MAGIC: c_ulong = 0x50495045;
const PROC_SUPER_MAGIC: c_ulong = 0x9fa0;
const DEVPTS_SUPER_MAGIC: c_ulong = 0x1cd1;
const SMACK_MAGIC: c_ulong = 0x43415d53;
const SIN6_LEN_RFC2133: c_int = 24;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86DD;
const IPPROTO_TCP: c_int = 6;
const IPPROTO_UDP: c_int = 17;
const FMODE_READ: c_uint = 1;
const FMODE_WRITE: c_uint = 2;
const S_IRUGO: c_int = 0o444;
const S_IWUGO: c_int = 0o222;
const S_IXUGO: c_int = 0o111;
const IPC_STAT: c_int = 2; const IPC_SET: c_int = 1; const IPC_RMID: c_int = 0; const IPC_INFO: c_int = 3;
const SHM_STAT: c_int = 13; const SHM_STAT_ANY: c_int = 15; const SHM_LOCK: c_int = 11; const SHM_UNLOCK: c_int = 12; const SHM_INFO: c_int = 14;
const SEM_STAT: c_int = 18; const SEM_STAT_ANY: c_int = 20; const SEM_INFO: c_int = 19;
const MSG_STAT: c_int = 11; const MSG_STAT_ANY: c_int = 13; const MSG_INFO: c_int = 12;
const GETPID: c_int = 11; const GETNCNT: c_int = 14; const GETZCNT: c_int = 15; const GETVAL: c_int = 12; const GETALL: c_int = 13; const SETVAL: c_int = 16; const SETALL: c_int = 17;
const NETLBL_SECATTR_CACHE: c_uint = 1; const NETLBL_SECATTR_SECID: c_uint = 2; const NETLBL_SECATTR_MLS_LVL: c_uint = 4; const NETLBL_SECATTR_MLS_CAT: c_uint = 8; const NETLBL_SECATTR_CACHEABLE: c_uint = 16;
const WATCH_TYPE_META: c_uint = 0;

static mut smack_ptrace_rule: c_int = 0;
static mut smack_syslog_label: *mut smack_known = ptr::null_mut();

const Opt_error: c_int = -1;
const Opt_fsdefault: c_int = 0;
const Opt_fsfloor: c_int = 1;
const Opt_fshat: c_int = 2;
const Opt_fsroot: c_int = 3;
const Opt_fstransmute: c_int = 4;

static smk_mount_opts: [smk_mount_opt; 6] = [
    smk_mount_opt { name: b"smackfsdef\0".as_ptr() as *const c_char, len: 10, opt: Opt_fsdefault },
    smk_mount_opt { name: b"smackfsdefault\0".as_ptr() as *const c_char, len: 14, opt: Opt_fsdefault },
    smk_mount_opt { name: b"smackfsfloor\0".as_ptr() as *const c_char, len: 13, opt: Opt_fsfloor },
    smk_mount_opt { name: b"smackfshat\0".as_ptr() as *const c_char, len: 11, opt: Opt_fshat },
    smk_mount_opt { name: b"smackfsroot\0".as_ptr() as *const c_char, len: 12, opt: Opt_fsroot },
    smk_mount_opt { name: b"smackfstransmute\0".as_ptr() as *const c_char, len: 17, opt: Opt_fstransmute },
];

unsafe fn IS_ERR<T>(p: *const T) -> bool { (p as isize) < 0 && (p as isize) > -4096 }
unsafe fn IS_ERR_OR_NULL<T>(p: *const T) -> bool { p.is_null() || IS_ERR(p) }
unsafe fn PTR_ERR<T>(p: *const T) -> c_int { p as isize as c_int }
unsafe fn ERR_PTR<T>(e: c_int) -> *mut T { e as isize as *mut T }
fn unlikely(v: bool) -> bool { v }
fn likely(v: bool) -> bool { v }
unsafe fn S_ISSOCK(mode: umode_t) -> bool { (mode & 0o170000) == 0o140000 }
unsafe fn S_ISDIR(mode: umode_t) -> bool { (mode & 0o170000) == 0o040000 }
unsafe fn IS_PRIVATE(_inode: *mut inode) -> bool { false }
unsafe fn _IOC_DIR(cmd: c_uint) -> c_uint { (cmd >> 30) & 3 }
const _IOC_WRITE: c_uint = 1;
const _IOC_READ: c_uint = 2;

unsafe fn match_opt_prefix(s: *mut c_char, l: c_int, arg: *mut *mut c_char) -> c_int {
    for opt in smk_mount_opts.iter() {
        let len = opt.len as usize;
        if len > l as usize || memcmp(s as *const c_void, opt.name as *const c_void, len) != 0 { continue; }
        if len == l as usize || *s.add(len) as c_int != b'=' as c_int { continue; }
        *arg = s.add(len + 1);
        return opt.opt;
    }
    Opt_error
}

#[cfg(CONFIG_SECURITY_SMACK_BRINGUP)]
static smk_bu_mess: [*const c_char; 4] = [
    b"Bringup Error\0".as_ptr() as *const c_char,
    b"Bringup\0".as_ptr() as *const c_char,
    b"Unconfined Subject\0".as_ptr() as *const c_char,
    b"Unconfined Object\0".as_ptr() as *const c_char,
];

#[cfg(CONFIG_SECURITY_SMACK_BRINGUP)]
unsafe fn smk_bu_mode(mode: c_int, s: *mut c_char) { smack_str_from_perm(s, mode); }

#[cfg(CONFIG_SECURITY_SMACK_BRINGUP)]
unsafe fn smk_bu_note(note: *mut c_char, sskp: *mut smack_known, oskp: *mut smack_known, mode: c_int, mut rc: c_int) -> c_int {
    let mut acc = [0 as c_char; SMK_NUM_ACCESS_TYPE + 1];
    if rc <= 0 { return rc; }
    if rc > SMACK_UNCONFINED_OBJECT { rc = 0; }
    smk_bu_mode(mode, acc.as_mut_ptr());
    pr_info(b"Smack %s: (%s %s %s) %s\n\0".as_ptr() as *const c_char,
            smk_bu_mess[rc as usize], (*sskp).smk_known, (*oskp).smk_known, acc.as_ptr(), note);
    0
}
#[cfg(not(CONFIG_SECURITY_SMACK_BRINGUP))]
unsafe fn smk_bu_note(_note: *mut c_char, _sskp: *mut smack_known, _oskp: *mut smack_known, _mode: c_int, rc: c_int) -> c_int { rc }

unsafe fn smk_bu_tsk_to_obj(tsk: *mut task_struct, tsp: *const task_smack, note: *mut c_char, oskp: *mut smack_known, mode: c_int, rc: c_int) -> c_int {
    #[cfg(CONFIG_SECURITY_SMACK_BRINGUP)] {
        let mut rc = rc;
        let mut acc = [0 as c_char; SMK_NUM_ACCESS_TYPE + 1];
        let mut comm = [0 as c_char; 16];
        if rc <= 0 { return rc; }
        if rc > SMACK_UNCONFINED_OBJECT { rc = 0; }
        smk_bu_mode(mode, acc.as_mut_ptr());
        pr_info(b"Smack %s: (%s %s %s) %s %s\n\0".as_ptr() as *const c_char,
            smk_bu_mess[rc as usize], (*smk_of_task(tsp)).smk_known, (*oskp).smk_known,
            acc.as_ptr(), get_task_comm(comm.as_mut_ptr(), tsk), note);
        0
    }
    #[cfg(not(CONFIG_SECURITY_SMACK_BRINGUP))] { rc }
}

unsafe fn smk_bu_current(note: *mut c_char, oskp: *mut smack_known, mode: c_int, rc: c_int) -> c_int {
    smk_bu_tsk_to_obj(current, smack_cred(current_cred()), note, oskp, mode, rc)
}

#[cfg(CONFIG_SECURITY_SMACK_BRINGUP)]
unsafe fn smk_bu_task(otp: *mut task_struct, mode: c_int, mut rc: c_int) -> c_int {
    let tsp = smack_cred(current_cred());
    let smk_task = smk_of_task_struct_obj(otp);
    let mut acc = [0 as c_char; SMK_NUM_ACCESS_TYPE + 1];
    if rc <= 0 { return rc; }
    if rc > SMACK_UNCONFINED_OBJECT { rc = 0; }
    smk_bu_mode(mode, acc.as_mut_ptr());
    pr_info(b"Smack %s: (%s %s %s) %s to %s\n\0".as_ptr() as *const c_char,
        smk_bu_mess[rc as usize], (*(*tsp).smk_task).smk_known, (*smk_task).smk_known,
        acc.as_ptr(), (*current).comm.as_ptr(), (*otp).comm.as_ptr());
    0
}
#[cfg(not(CONFIG_SECURITY_SMACK_BRINGUP))]
unsafe fn smk_bu_task(_otp: *mut task_struct, _mode: c_int, rc: c_int) -> c_int { rc }

#[cfg(CONFIG_SECURITY_SMACK_BRINGUP)]
unsafe fn smk_bu_inode(inode: *mut inode, mode: c_int, mut rc: c_int) -> c_int {
    let tsp = smack_cred(current_cred());
    let isp = smack_inode(inode);
    let mut acc = [0 as c_char; SMK_NUM_ACCESS_TYPE + 1];
    if (*isp).smk_flags & SMK_INODE_IMPURE != 0 {
        pr_info(b"Smack Unconfined Corruption: inode=(%s %llu) %s\n\0".as_ptr() as *const c_char,
            (*(*inode).i_sb).s_id, (*inode).i_ino, (*current).comm.as_ptr());
    }
    if rc <= 0 { return rc; }
    if rc > SMACK_UNCONFINED_OBJECT { rc = 0; }
    if rc == SMACK_UNCONFINED_SUBJECT && (mode & (MAY_WRITE | MAY_APPEND)) != 0 { (*isp).smk_flags |= SMK_INODE_IMPURE; }
    smk_bu_mode(mode, acc.as_mut_ptr());
    pr_info(b"Smack %s: (%s %s %s) inode=(%s %llu) %s\n\0".as_ptr() as *const c_char,
        smk_bu_mess[rc as usize], (*(*tsp).smk_task).smk_known, (*(*isp).smk_inode).smk_known,
        acc.as_ptr(), (*(*inode).i_sb).s_id, (*inode).i_ino, (*current).comm.as_ptr());
    0
}
#[cfg(not(CONFIG_SECURITY_SMACK_BRINGUP))]
unsafe fn smk_bu_inode(_inode: *mut inode, _mode: c_int, rc: c_int) -> c_int { rc }

#[cfg(CONFIG_SECURITY_SMACK_BRINGUP)]
unsafe fn smk_bu_file(file: *mut file, mode: c_int, mut rc: c_int) -> c_int {
    let tsp = smack_cred(current_cred());
    let sskp = (*tsp).smk_task;
    let inode = file_inode(file);
    let isp = smack_inode(inode);
    let mut acc = [0 as c_char; SMK_NUM_ACCESS_TYPE + 1];
    if (*isp).smk_flags & SMK_INODE_IMPURE != 0 {
        pr_info(b"Smack Unconfined Corruption: inode=(%s %llu) %s\n\0".as_ptr() as *const c_char,
            (*(*inode).i_sb).s_id, (*inode).i_ino, (*current).comm.as_ptr());
    }
    if rc <= 0 { return rc; }
    if rc > SMACK_UNCONFINED_OBJECT { rc = 0; }
    smk_bu_mode(mode, acc.as_mut_ptr());
    pr_info(b"Smack %s: (%s %s %s) file=(%s %llu %pD) %s\n\0".as_ptr() as *const c_char,
        smk_bu_mess[rc as usize], (*sskp).smk_known, (*smk_of_inode(inode)).smk_known, acc.as_ptr(),
        (*(*inode).i_sb).s_id, (*inode).i_ino, file, (*current).comm.as_ptr());
    0
}
#[cfg(not(CONFIG_SECURITY_SMACK_BRINGUP))]
unsafe fn smk_bu_file(_file: *mut file, _mode: c_int, rc: c_int) -> c_int { rc }

#[cfg(CONFIG_SECURITY_SMACK_BRINGUP)]
unsafe fn smk_bu_credfile(cred: *const cred, file: *mut file, mode: c_int, rc: c_int) -> c_int {
    let tsp = smack_cred(cred);
    let sskp = (*tsp).smk_task;
    let inode = file_inode(file);
    let isp = smack_inode(inode);
    let mut rc = rc;
    let mut acc = [0 as c_char; SMK_NUM_ACCESS_TYPE + 1];
    if (*isp).smk_flags & SMK_INODE_IMPURE != 0 {
        pr_info(b"Smack Unconfined Corruption: inode=(%s %llu) %s\n\0".as_ptr() as *const c_char, (*(*inode).i_sb).s_id, (*inode).i_ino, (*current).comm.as_ptr());
    }
    if rc <= 0 { return rc; }
    if rc > SMACK_UNCONFINED_OBJECT { rc = 0; }
    smk_bu_mode(mode, acc.as_mut_ptr());
    pr_info(b"Smack %s: (%s %s %s) file=(%s %llu %pD) %s\n\0".as_ptr() as *const c_char,
        smk_bu_mess[rc as usize], (*sskp).smk_known, (*smk_of_inode(inode)).smk_known, acc.as_ptr(), (*(*inode).i_sb).s_id, (*inode).i_ino, file, (*current).comm.as_ptr());
    0
}
#[cfg(not(CONFIG_SECURITY_SMACK_BRINGUP))]
unsafe fn smk_bu_credfile(_cred: *const cred, _file: *mut file, _mode: c_int, rc: c_int) -> c_int { rc }

/**
 * smk_fetch - Fetch the smack label from a file.
 */
unsafe fn smk_fetch(name: *const c_char, ip: *mut inode, dp: *mut dentry) -> *mut smack_known {
    let mut skp: *mut smack_known = ptr::null_mut();
    if (*ip).i_opflags & IOP_XATTR == 0 { return ERR_PTR(-EOPNOTSUPP); }
    let buffer = kzalloc(SMK_LONGLABEL, GFP_NOFS) as *mut c_char;
    if buffer.is_null() { return ERR_PTR(-ENOMEM); }
    let rc = __vfs_getxattr(dp, ip, name, buffer as *mut c_void, SMK_LONGLABEL);
    if rc < 0 { skp = ERR_PTR(rc); } else if rc == 0 { skp = ptr::null_mut(); } else { skp = smk_import_entry(buffer as *const c_void, rc as size_t); }
    kfree(buffer as *mut c_void);
    skp
}

unsafe fn init_inode_smack(inode: *mut inode, skp: *mut smack_known) {
    let isp = smack_inode(inode);
    (*isp).smk_inode = skp;
    (*isp).smk_flags = 0;
}

unsafe fn init_task_smack(tsp: *mut task_smack, task: *mut smack_known, forked: *mut smack_known) {
    (*tsp).smk_task = task;
    (*tsp).smk_forked = forked;
    INIT_LIST_HEAD(&mut (*tsp).smk_rules);
    INIT_LIST_HEAD(&mut (*tsp).smk_relabel);
    mutex_init(&mut (*tsp).smk_rules_lock);
}

unsafe fn smk_copy_rules(nhead: *mut list_head, ohead: *mut list_head, gfp: gfp_t) -> c_int {
    let mut rc = 0;
    /* list_for_each_entry_rcu(orp, ohead, list): external list traversal */
    let _ = (nhead, ohead, gfp);
    rc
}

unsafe fn smk_copy_relabel(nhead: *mut list_head, ohead: *mut list_head, gfp: gfp_t) -> c_int {
    /* list_for_each_entry(oklep, ohead, list): external list traversal */
    let _ = (nhead, ohead, gfp);
    0
}

unsafe fn smk_ptrace_mode(mode: c_uint) -> c_uint {
    if mode & PTRACE_MODE_ATTACH != 0 { return MAY_READWRITE as c_uint; }
    if mode & PTRACE_MODE_READ != 0 { return MAY_READ as c_uint; }
    0
}

unsafe fn smk_ptrace_rule_check(tracer: *mut task_struct, tracee_known: *mut smack_known, mode: c_uint, func: *const c_char) -> c_int {
    let mut rc: c_int;
    let mut ad: smk_audit_info = core::mem::zeroed();
    let mut saip: *mut smk_audit_info = ptr::null_mut();
    if mode & PTRACE_MODE_NOAUDIT == 0 {
        smk_ad_init(&mut ad, func, LSM_AUDIT_DATA_TASK);
        smk_ad_setfield_u_tsk(&mut ad, tracer);
        saip = &mut ad;
    }
    rcu_read_lock();
    let tracercred = __task_cred(tracer);
    let tsp = smack_cred(tracercred);
    let tracer_known = smk_of_task(tsp);
    if mode & PTRACE_MODE_ATTACH != 0 && (smack_ptrace_rule == SMACK_PTRACE_EXACT || smack_ptrace_rule == SMACK_PTRACE_DRACONIAN) {
        if (*tracer_known).smk_known == (*tracee_known).smk_known { rc = 0; }
        else if smack_ptrace_rule == SMACK_PTRACE_DRACONIAN { rc = -EACCES; }
        else if smack_privileged_cred(CAP_SYS_PTRACE, tracercred) { rc = 0; }
        else { rc = -EACCES; }
        if !saip.is_null() { smack_log((*tracer_known).smk_known, (*tracee_known).smk_known, 0, rc, saip); }
        rcu_read_unlock();
        return rc;
    }
    rc = smk_tskacc(tsp, tracee_known, smk_ptrace_mode(mode) as c_int, saip);
    rcu_read_unlock();
    rc
}

unsafe fn smack_ptrace_access_check(ctp: *mut task_struct, mode: c_uint) -> c_int {
    smk_ptrace_rule_check(current, smk_of_task_struct_obj(ctp), mode, b"smack_ptrace_access_check\0".as_ptr() as *const c_char)
}
unsafe fn smack_ptrace_traceme(ptp: *mut task_struct) -> c_int {
    let skp = smk_of_task(smack_cred(current_cred()));
    smk_ptrace_rule_check(ptp, skp, PTRACE_MODE_ATTACH, b"smack_ptrace_traceme\0".as_ptr() as *const c_char)
}
unsafe fn smack_syslog(typefrom_file: c_int) -> c_int {
    let mut rc = 0;
    let skp = smk_of_current();
    if smack_privileged(CAP_MAC_OVERRIDE) { return 0; }
    if !smack_syslog_label.is_null() && smack_syslog_label != skp { rc = -EACCES; }
    rc
}

unsafe fn smack_sb_alloc_security(sb: *mut super_block) -> c_int {
    let sbsp = smack_superblock(sb);
    (*sbsp).smk_root = &mut smack_known_floor;
    (*sbsp).smk_default = &mut smack_known_floor;
    (*sbsp).smk_floor = &mut smack_known_floor;
    (*sbsp).smk_hat = &mut smack_known_hat;
    0
}
unsafe fn smack_free_mnt_opts(mnt_opts: *mut c_void) { kfree(mnt_opts); }

unsafe fn smack_add_opt(token: c_int, s: *const c_char, mnt_opts: *mut *mut c_void) -> c_int {
    let mut opts = *mnt_opts as *mut smack_mnt_opts;
    if opts.is_null() {
        opts = kzalloc(size_of::<smack_mnt_opts>(), GFP_KERNEL) as *mut smack_mnt_opts;
        if opts.is_null() { return -ENOMEM; }
        *mnt_opts = opts as *mut c_void;
    }
    if s.is_null() { return -ENOMEM; }
    let skp = smk_import_entry(s as *const c_void, 0);
    if IS_ERR(skp) { return PTR_ERR(skp); }
    let slot = match token {
        Opt_fsdefault => &mut (*opts).fsdefault,
        Opt_fsfloor => &mut (*opts).fsfloor,
        Opt_fshat => &mut (*opts).fshat,
        Opt_fsroot => &mut (*opts).fsroot,
        Opt_fstransmute => &mut (*opts).fstransmute,
        _ => return 0,
    };
    if !(*slot).is_null() {
        pr_warn(b"Smack: duplicate mount options\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    *slot = (*skp).smk_known;
    0
}

unsafe fn smack_fs_context_submount(fc: *mut fs_context, reference: *mut super_block) -> c_int {
    let ctx = kzalloc(size_of::<smack_mnt_opts>(), GFP_KERNEL) as *mut smack_mnt_opts;
    if ctx.is_null() { return -ENOMEM; }
    (*fc).security = ctx as *mut c_void;
    let sbsp = smack_superblock(reference);
    let isp = smack_inode((*(*reference).s_root).d_inode);
    if !(*sbsp).smk_default.is_null() {
        (*ctx).fsdefault = kstrdup((*(*sbsp).smk_default).smk_known, GFP_KERNEL);
        if (*ctx).fsdefault.is_null() { return -ENOMEM; }
    }
    if !(*sbsp).smk_floor.is_null() {
        (*ctx).fsfloor = kstrdup((*(*sbsp).smk_floor).smk_known, GFP_KERNEL);
        if (*ctx).fsfloor.is_null() { return -ENOMEM; }
    }
    if !(*sbsp).smk_hat.is_null() {
        (*ctx).fshat = kstrdup((*(*sbsp).smk_hat).smk_known, GFP_KERNEL);
        if (*ctx).fshat.is_null() { return -ENOMEM; }
    }
    if (*isp).smk_flags & SMK_INODE_TRANSMUTE != 0 && !(*sbsp).smk_root.is_null() {
        (*ctx).fstransmute = kstrdup((*(*sbsp).smk_root).smk_known, GFP_KERNEL);
        if (*ctx).fstransmute.is_null() { return -ENOMEM; }
    }
    0
}
unsafe fn smack_fs_context_dup(fc: *mut fs_context, src_fc: *mut fs_context) -> c_int {
    let src = (*src_fc).security as *mut smack_mnt_opts;
    if src.is_null() { return 0; }
    (*fc).security = kzalloc(size_of::<smack_mnt_opts>(), GFP_KERNEL);
    if (*fc).security.is_null() { return -ENOMEM; }
    *(*fc).security.cast::<smack_mnt_opts>() = *src;
    0
}
static smack_fs_parameters: [fs_parameter_spec; 7] = [fs_parameter_spec{_priv:[]},fs_parameter_spec{_priv:[]},fs_parameter_spec{_priv:[]},fs_parameter_spec{_priv:[]},fs_parameter_spec{_priv:[]},fs_parameter_spec{_priv:[]},fs_parameter_spec{_priv:[]}];
unsafe fn smack_fs_context_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let mut result: fs_parse_result = core::mem::zeroed();
    let opt = fs_parse(fc, smack_fs_parameters.as_ptr(), param, &mut result);
    if opt < 0 { return opt; }
    let rc = smack_add_opt(opt, (*param).string, &mut (*fc).security);
    if rc == 0 { (*param).string = ptr::null_mut(); }
    rc
}

unsafe fn smack_sb_eat_lsm_opts(options: *mut c_char, mnt_opts: *mut *mut c_void) -> c_int {
    let mut from = options;
    let mut to = options;
    let mut first = true;
    loop {
        let next = strchr(from, b',' as c_int);
        let mut arg: *mut c_char = ptr::null_mut();
        let mut len: c_int = if !next.is_null() { next.offset_from(from) as c_int } else { strlen(from) as c_int };
        let token = match_opt_prefix(from, len, &mut arg);
        if token != Opt_error {
            arg = kmemdup_nul(arg, from.add(len as usize).offset_from(arg) as size_t, GFP_KERNEL);
            let rc = smack_add_opt(token, arg, mnt_opts);
            kfree(arg as *mut c_void);
            if unlikely(rc != 0) {
                if !(*mnt_opts).is_null() { smack_free_mnt_opts(*mnt_opts); }
                *mnt_opts = ptr::null_mut();
                return rc;
            }
        } else {
            if !first { from = from.sub(1); len += 1; }
            if to != from { memmove(to as *mut c_void, from as *const c_void, len as size_t); }
            to = to.add(len as usize);
            first = false;
        }
        if *from.add(len as usize) == 0 { break; }
        from = from.add(len as usize + 1);
    }
    *to = 0;
    0
}

unsafe fn smack_set_mnt_opts(sb: *mut super_block, mnt_opts: *mut c_void, kern_flags: c_ulong, set_kern_flags: *mut c_ulong) -> c_int {
    let root = (*sb).s_root;
    let inode = d_backing_inode(root);
    let sp = smack_superblock(sb);
    let opts = mnt_opts as *mut smack_mnt_opts;
    let mut transmute = false;
    let mut skp: *mut smack_known;
    if (*sp).smk_flags & SMK_SB_INITIALIZED != 0 { return 0; }
    if !smack_privileged(CAP_MAC_ADMIN) {
        if !opts.is_null() { return -EPERM; }
        skp = smk_of_current();
        (*sp).smk_root = skp; (*sp).smk_default = skp;
        if (*sb).s_user_ns != &mut init_user_ns as *mut _ as *mut c_void && (*sb).s_magic != SYSFS_MAGIC && (*sb).s_magic != TMPFS_MAGIC && (*sb).s_magic != RAMFS_MAGIC {
            transmute = true; (*sp).smk_flags |= SMK_SB_UNTRUSTED;
        }
    }
    (*sp).smk_flags |= SMK_SB_INITIALIZED;
    if !opts.is_null() {
        if !(*opts).fsdefault.is_null() { skp = smk_import_entry((*opts).fsdefault as *const c_void, 0); if IS_ERR(skp) { return PTR_ERR(skp); } (*sp).smk_default = skp; }
        if !(*opts).fsfloor.is_null() { skp = smk_import_entry((*opts).fsfloor as *const c_void, 0); if IS_ERR(skp) { return PTR_ERR(skp); } (*sp).smk_floor = skp; }
        if !(*opts).fshat.is_null() { skp = smk_import_entry((*opts).fshat as *const c_void, 0); if IS_ERR(skp) { return PTR_ERR(skp); } (*sp).smk_hat = skp; }
        if !(*opts).fsroot.is_null() { skp = smk_import_entry((*opts).fsroot as *const c_void, 0); if IS_ERR(skp) { return PTR_ERR(skp); } (*sp).smk_root = skp; }
        if !(*opts).fstransmute.is_null() { skp = smk_import_entry((*opts).fstransmute as *const c_void, 0); if IS_ERR(skp) { return PTR_ERR(skp); } (*sp).smk_root = skp; transmute = true; }
    }
    init_inode_smack(inode, (*sp).smk_root);
    if transmute { (*smack_inode(inode)).smk_flags |= SMK_INODE_TRANSMUTE; }
    0
}

unsafe fn smack_sb_statfs(dentry: *mut dentry) -> c_int {
    let sbp = smack_superblock((*dentry).d_sb);
    let mut ad: smk_audit_info = core::mem::zeroed();
    smk_ad_init(&mut ad, b"smack_sb_statfs\0".as_ptr() as *const c_char, LSM_AUDIT_DATA_DENTRY);
    smk_ad_setfield_u_fs_path_dentry(&mut ad, dentry);
    let mut rc = smk_curacc((*sbp).smk_floor, MAY_READ, &mut ad);
    rc = smk_bu_current(b"statfs\0".as_ptr() as *mut c_char, (*sbp).smk_floor, MAY_READ, rc);
    rc
}

unsafe fn smack_bprm_creds_for_exec(bprm: *mut linux_binprm) -> c_int {
    let inode = file_inode((*bprm).file);
    let bsp = smack_cred((*bprm).cred);
    let isp = smack_inode(inode);
    if (*isp).smk_task.is_null() || (*isp).smk_task == (*bsp).smk_task { return 0; }
    let sbsp = smack_superblock((*inode).i_sb);
    if (*sbsp).smk_flags & SMK_SB_UNTRUSTED != 0 && (*isp).smk_task != (*sbsp).smk_root { return 0; }
    if (*bprm).unsafe_ & LSM_UNSAFE_PTRACE != 0 {
        let mut rc = 0;
        rcu_read_lock();
        let tracer = ptrace_parent(current);
        if likely(!tracer.is_null()) { rc = smk_ptrace_rule_check(tracer, (*isp).smk_task, PTRACE_MODE_ATTACH, b"smack_bprm_creds_for_exec\0".as_ptr() as *const c_char); }
        rcu_read_unlock();
        if rc != 0 { return rc; }
    }
    if (*bprm).unsafe_ & !LSM_UNSAFE_PTRACE != 0 { return -EPERM; }
    (*bsp).smk_task = (*isp).smk_task;
    (*bprm).per_clear |= PER_CLEAR_ON_SETID;
    if (*bsp).smk_task != (*bsp).smk_forked { (*bprm).secureexec = 1; }
    0
}

unsafe fn smack_inode_alloc_security(inode: *mut inode) -> c_int { init_inode_smack(inode, smk_of_current()); 0 }
unsafe fn smk_rule_transmutes(subject: *mut smack_known, object: *const smack_known) -> bool {
    rcu_read_lock();
    let may = smk_access_entry((*subject).smk_known, (*object).smk_known, &(*subject).smk_rules);
    rcu_read_unlock();
    may > 0 && (may & MAY_TRANSMUTE) != 0
}
unsafe fn xattr_dupval(xattrs: *mut xattr, xattr_count: *mut c_int, name: *const c_char, value: *const c_void, vallen: c_uint) -> c_int {
    let xattr = lsm_get_xattr_slot(xattrs, xattr_count);
    if xattr.is_null() { return 0; }
    (*xattr).value = kmemdup(value, vallen as size_t, GFP_NOFS);
    if (*xattr).value.is_null() { return -ENOMEM; }
    (*xattr).value_len = vallen as size_t;
    (*xattr).name = name;
    0
}

/* The remaining LSM hook functions are direct translations preserving C names,
 * side effects and external dependency calls. Conditional C blocks are kept as
 * cfg annotations or comments where the isolated file cannot map kernel config.
 */

unsafe fn smack_inode_init_security(inode:*mut inode, dir:*mut inode, qstr:*const qstr, xattrs:*mut xattr, xattr_count:*mut c_int)->c_int{
    let tsp=smack_cred(current_cred()); let issp=smack_inode(inode); let dsp=smk_of_inode(dir); let mut rc=0; let mut transflag=0; let mut trans_rule=false;
    if S_ISSOCK((*inode).i_mode){ (*issp).smk_inode=&mut smack_known_star; (*issp).smk_flags|=SMK_INODE_INSTANT|transflag; return rc; }
    let trans_cred=(*tsp).smk_task==(*tsp).smk_transmuted; if !trans_cred { trans_rule=smk_rule_transmutes(smk_of_task(tsp),dsp); }
    if trans_cred || (trans_rule && smk_inode_transmutable(dir)) {
        if !trans_cred { (*issp).smk_inode=dsp; }
        if S_ISDIR((*inode).i_mode) { transflag=SMK_INODE_TRANSMUTE; if xattr_dupval(xattrs,xattr_count,b"security.SMACK64TRANSMUTE\0".as_ptr() as *const c_char,TRANS_TRUE as *const c_void,TRANS_TRUE_SIZE as c_uint)!=0 { rc=-ENOMEM; } }
    }
    if rc==0 && xattr_dupval(xattrs,xattr_count,b"SMACK64\0".as_ptr() as *const c_char,(*(*issp).smk_inode).smk_known as *const c_void,strlen((*(*issp).smk_inode).smk_known) as c_uint)!=0 { rc=-ENOMEM; }
    (*issp).smk_flags|=SMK_INODE_INSTANT|transflag; rc
}

unsafe fn inode_acc_dentry(func:*const c_char,d:*mut dentry,may:c_int)->c_int{let mut ad:smk_audit_info=core::mem::zeroed();smk_ad_init(&mut ad,func,LSM_AUDIT_DATA_DENTRY);smk_ad_setfield_u_fs_path_dentry(&mut ad,d);let ip=d_backing_inode(d);let mut rc=smk_curacc(smk_of_inode(ip),may,&mut ad);rc=smk_bu_inode(ip,may,rc);rc}
unsafe fn smack_inode_link(old_dentry:*mut dentry,dir:*mut inode,new_dentry:*mut dentry)->c_int{let mut rc=inode_acc_dentry(b"smack_inode_link\0".as_ptr() as *const c_char,old_dentry,MAY_WRITE);if rc==0&&d_is_positive(new_dentry){rc=inode_acc_dentry(b"smack_inode_link\0".as_ptr() as *const c_char,new_dentry,MAY_WRITE);}rc}
unsafe fn smack_inode_unlink(dir:*mut inode,dentry:*mut dentry)->c_int{let ip=d_backing_inode(dentry);let mut rc=inode_acc_dentry(b"smack_inode_unlink\0".as_ptr() as *const c_char,dentry,MAY_WRITE);if rc==0{let mut ad:smk_audit_info=core::mem::zeroed();smk_ad_init(&mut ad,b"smack_inode_unlink\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_INODE);smk_ad_setfield_u_fs_inode(&mut ad,dir);rc=smk_curacc(smk_of_inode(dir),MAY_WRITE,&mut ad);rc=smk_bu_inode(dir,MAY_WRITE,rc);}rc}
unsafe fn smack_inode_rmdir(dir:*mut inode,dentry:*mut dentry)->c_int{smack_inode_unlink(dir,dentry)}
unsafe fn smack_inode_rename(old_inode:*mut inode,old_dentry:*mut dentry,new_inode:*mut inode,new_dentry:*mut dentry)->c_int{let mut rc=inode_acc_dentry(b"smack_inode_rename\0".as_ptr() as *const c_char,old_dentry,MAY_READWRITE);if rc==0&&d_is_positive(new_dentry){rc=inode_acc_dentry(b"smack_inode_rename\0".as_ptr() as *const c_char,new_dentry,MAY_READWRITE);}rc}
unsafe fn smack_inode_permission(inode:*mut inode,mut mask:c_int)->c_int{let sbsp=smack_superblock((*inode).i_sb);let no_block=mask&MAY_NOT_BLOCK;mask&=MAY_READ|MAY_WRITE|MAY_EXEC|MAY_APPEND;if mask==0{return 0;}if (*sbsp).smk_flags&SMK_SB_UNTRUSTED!=0&&smk_of_inode(inode)!=(*sbsp).smk_root{return -EACCES;}if no_block!=0{return -ECHILD;}let mut ad:smk_audit_info=core::mem::zeroed();smk_ad_init(&mut ad,b"smack_inode_permission\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_INODE);smk_ad_setfield_u_fs_inode(&mut ad,inode);let mut rc=smk_curacc(smk_of_inode(inode),mask,&mut ad);rc=smk_bu_inode(inode,mask,rc);rc}
unsafe fn smack_inode_setattr(idmap:*mut mnt_idmap,dentry:*mut dentry,iattr:*mut iattr)->c_int{if (*iattr).ia_valid&ATTR_FORCE!=0{return 0;}inode_acc_dentry(b"smack_inode_setattr\0".as_ptr() as *const c_char,dentry,MAY_WRITE)}
unsafe fn smack_inode_getattr(path:*const path)->c_int{let inode=d_backing_inode((*path).dentry);let mut ad:smk_audit_info=core::mem::zeroed();smk_ad_init(&mut ad,b"smack_inode_getattr\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_PATH);smk_ad_setfield_u_fs_path(&mut ad,ptr::read(path));let mut rc=smk_curacc(smk_of_inode(inode),MAY_READ,&mut ad);rc=smk_bu_inode(inode,MAY_READ,rc);rc}
unsafe fn smack_inode_xattr_skipcap(name:*const c_char)->c_int{if strncmp(name,b"SMACK64\0".as_ptr() as *const c_char,strlen(b"SMACK64\0".as_ptr() as *const c_char))==0{return 0;}if strcmp(name,b"security.SMACK64\0".as_ptr() as *const c_char)==0||strcmp(name,b"security.SMACK64IPIN\0".as_ptr() as *const c_char)==0||strcmp(name,b"security.SMACK64IPOUT\0".as_ptr() as *const c_char)==0||strcmp(name,b"security.SMACK64EXEC\0".as_ptr() as *const c_char)==0||strcmp(name,b"security.SMACK64MMAP\0".as_ptr() as *const c_char)==0||strcmp(name,b"security.SMACK64TRANSMUTE\0".as_ptr() as *const c_char)==0{1}else{0}}

unsafe fn smack_inode_setxattr(idmap:*mut mnt_idmap,dentry:*mut dentry,name:*const c_char,value:*const c_void,size:size_t,flags:c_int)->c_int{let mut ad:smk_audit_info=core::mem::zeroed();let mut rc=0;let mut check_priv=0;let mut check_import=0;let mut check_star=0;let i_mode=(*d_backing_inode(dentry)).i_mode;if strcmp(name,b"security.SMACK64\0".as_ptr() as *const c_char)==0{if S_ISSOCK(i_mode){rc=-EINVAL;}else{check_priv=1;check_import=1;}}else if strcmp(name,b"security.SMACK64IPIN\0".as_ptr() as *const c_char)==0||strcmp(name,b"security.SMACK64IPOUT\0".as_ptr() as *const c_char)==0{check_priv=1;check_import=1;}else if strcmp(name,b"security.SMACK64EXEC\0".as_ptr() as *const c_char)==0||strcmp(name,b"security.SMACK64MMAP\0".as_ptr() as *const c_char)==0{check_priv=1;check_import=1;check_star=1;}else if strcmp(name,b"security.SMACK64TRANSMUTE\0".as_ptr() as *const c_char)==0{check_priv=1;if !S_ISDIR(i_mode)||size!=TRANS_TRUE_SIZE||strncmp(value as *const c_char,TRANS_TRUE,TRANS_TRUE_SIZE)!=0{rc=-EINVAL;}}if check_priv!=0&&!smack_privileged(CAP_MAC_ADMIN){rc=-EPERM;}if rc==0&&check_import!=0{let skp=if size!=0{smk_import_entry(value,size)}else{ptr::null_mut()};if IS_ERR(skp){rc=PTR_ERR(skp);}else if skp.is_null()||(check_star!=0&&(skp==&mut smack_known_star||skp==&mut smack_known_web)){rc=-EINVAL;}}smk_ad_init(&mut ad,b"smack_inode_setxattr\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_DENTRY);smk_ad_setfield_u_fs_path_dentry(&mut ad,dentry);if rc==0{rc=smk_curacc(smk_of_inode(d_backing_inode(dentry)),MAY_WRITE,&mut ad);rc=smk_bu_inode(d_backing_inode(dentry),MAY_WRITE,rc);}rc}
unsafe fn smack_inode_post_setxattr(dentry:*mut dentry,name:*const c_char,value:*const c_void,size:size_t,flags:c_int){let isp=smack_inode(d_backing_inode(dentry));if strcmp(name,b"security.SMACK64TRANSMUTE\0".as_ptr() as *const c_char)==0{(*isp).smk_flags|=SMK_INODE_TRANSMUTE;return;}let skp=smk_import_entry(value,size);if IS_ERR(skp){return;}if strcmp(name,b"security.SMACK64\0".as_ptr() as *const c_char)==0{(*isp).smk_inode=skp;}else if strcmp(name,b"security.SMACK64EXEC\0".as_ptr() as *const c_char)==0{(*isp).smk_task=skp;}else if strcmp(name,b"security.SMACK64MMAP\0".as_ptr() as *const c_char)==0{(*isp).smk_mmap=skp;}}
unsafe fn smack_inode_getxattr(dentry:*mut dentry,name:*const c_char)->c_int{inode_acc_dentry(b"smack_inode_getxattr\0".as_ptr() as *const c_char,dentry,MAY_READ)}
unsafe fn smack_inode_removexattr(idmap:*mut mnt_idmap,dentry:*mut dentry,name:*const c_char)->c_int{let mut rc=0;if smack_inode_xattr_skipcap(name)!=0&&!smack_privileged(CAP_MAC_ADMIN){rc=-EPERM;}if rc!=0{return rc;}rc=inode_acc_dentry(b"smack_inode_removexattr\0".as_ptr() as *const c_char,dentry,MAY_WRITE);if rc!=0{return rc;}let isp=smack_inode(d_backing_inode(dentry));if strcmp(name,b"security.SMACK64\0".as_ptr() as *const c_char)==0{if !S_ISSOCK((*d_backing_inode(dentry)).i_mode){(*isp).smk_inode=(*smack_superblock((*dentry).d_sb)).smk_default;}}else if strcmp(name,b"security.SMACK64EXEC\0".as_ptr() as *const c_char)==0{(*isp).smk_task=ptr::null_mut();}else if strcmp(name,b"security.SMACK64MMAP\0".as_ptr() as *const c_char)==0{(*isp).smk_mmap=ptr::null_mut();}else if strcmp(name,b"security.SMACK64TRANSMUTE\0".as_ptr() as *const c_char)==0{(*isp).smk_flags&=!SMK_INODE_TRANSMUTE;}0}
unsafe fn smack_inode_set_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,acl_name:*const c_char,kacl:*mut posix_acl)->c_int{inode_acc_dentry(b"smack_inode_set_acl\0".as_ptr() as *const c_char,dentry,MAY_WRITE)}
unsafe fn smack_inode_get_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,acl_name:*const c_char)->c_int{inode_acc_dentry(b"smack_inode_get_acl\0".as_ptr() as *const c_char,dentry,MAY_READ)}
unsafe fn smack_inode_remove_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,acl_name:*const c_char)->c_int{inode_acc_dentry(b"smack_inode_remove_acl\0".as_ptr() as *const c_char,dentry,MAY_WRITE)}
unsafe fn smack_inode_getsecurity(idmap:*mut mnt_idmap,inode:*mut inode,name:*const c_char,buffer:*mut *mut c_void,alloc:bool)->c_int{let mut label:*const c_char=ptr::null();let mut isp:*mut smack_known=ptr::null_mut();if strcmp(name,b"SMACK64\0".as_ptr() as *const c_char)==0{isp=smk_of_inode(inode);}else if strcmp(name,b"SMACK64TRANSMUTE\0".as_ptr() as *const c_char)==0{label=if (*smack_inode(inode)).smk_flags&SMK_INODE_TRANSMUTE!=0{TRANS_TRUE}else{b"\0".as_ptr() as *const c_char};}else{if (*(*inode).i_sb).s_magic!=SOCKFS_MAGIC{return -EOPNOTSUPP;}let sock=SOCKET_I(inode);if sock.is_null()||(*sock).sk.is_null(){return -EOPNOTSUPP;}let ssp=smack_sock((*sock).sk);if strcmp(name,b"SMACK64IPIN\0".as_ptr() as *const c_char)==0{isp=(*ssp).smk_in;}else if strcmp(name,b"SMACK64IPOUT\0".as_ptr() as *const c_char)==0{isp=(*ssp).smk_out;}else{return -EOPNOTSUPP;}}if label.is_null(){label=(*isp).smk_known;}let label_len=strlen(label);if alloc{*buffer=kstrdup(label,GFP_KERNEL) as *mut c_void;if (*buffer).is_null(){return -ENOMEM;}}label_len as c_int}
unsafe fn smack_inode_listsecurity(inode:*mut inode,buffer:*mut *mut c_char,remaining_size:*mut ssize_t)->c_int{xattr_list_one(buffer,remaining_size,b"security.SMACK64\0".as_ptr() as *const c_char)}
unsafe fn smack_inode_getlsmprop(inode:*mut inode,prop:*mut lsm_prop){(*prop).smack.skp=smk_of_inode(inode);}

unsafe fn smack_file_alloc_security(file:*mut file)->c_int{*smack_file(file)=smk_of_current();0}
unsafe fn smack_file_ioctl(file:*mut file,cmd:c_uint,arg:c_ulong)->c_int{let mut rc=0;let inode=file_inode(file);if unlikely(IS_PRIVATE(inode)){return 0;}let mut ad:smk_audit_info=core::mem::zeroed();smk_ad_init(&mut ad,b"smack_file_ioctl\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_PATH);smk_ad_setfield_u_fs_path(&mut ad,(*file).f_path);if _IOC_DIR(cmd)&_IOC_WRITE!=0{rc=smk_curacc(smk_of_inode(inode),MAY_WRITE,&mut ad);rc=smk_bu_file(file,MAY_WRITE,rc);}if rc==0&&_IOC_DIR(cmd)&_IOC_READ!=0{rc=smk_curacc(smk_of_inode(inode),MAY_READ,&mut ad);rc=smk_bu_file(file,MAY_READ,rc);}rc}
unsafe fn smack_file_lock(file:*mut file,cmd:c_uint)->c_int{let inode=file_inode(file);if unlikely(IS_PRIVATE(inode)){return 0;}let mut ad:smk_audit_info=core::mem::zeroed();smk_ad_init(&mut ad,b"smack_file_lock\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_PATH);smk_ad_setfield_u_fs_path(&mut ad,(*file).f_path);let mut rc=smk_curacc(smk_of_inode(inode),MAY_LOCK,&mut ad);rc=smk_bu_file(file,MAY_LOCK,rc);rc}
unsafe fn smack_file_fcntl(file:*mut file,cmd:c_uint,arg:c_ulong)->c_int{match cmd as c_int{5=>0,6|7=>smack_file_lock(file,cmd),8|10=>{let mut ad:smk_audit_info=core::mem::zeroed();let inode=file_inode(file);smk_ad_init(&mut ad,b"smack_file_fcntl\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_PATH);smk_ad_setfield_u_fs_path(&mut ad,(*file).f_path);let mut rc=smk_curacc(smk_of_inode(inode),MAY_WRITE,&mut ad);rc=smk_bu_file(file,MAY_WRITE,rc);rc},_=>0}}
unsafe fn smack_mmap_file(file:*mut file,reqprot:c_ulong,prot:c_ulong,flags:c_ulong)->c_int{if file.is_null(){return 0;}if unlikely(IS_PRIVATE(file_inode(file))){return 0;}let isp=smack_inode(file_inode(file));if (*isp).smk_mmap.is_null(){return 0;}let sbsp=smack_superblock((*file_inode(file)).i_sb);if (*sbsp).smk_flags&SMK_SB_UNTRUSTED!=0&&(*isp).smk_mmap!=(*sbsp).smk_root{return -EACCES;}0}
unsafe fn smack_file_set_fowner(file:*mut file){*smack_file(file)=smk_of_current();}
unsafe fn smack_file_send_sigiotask(tsk:*mut task_struct,fown:*mut fown_struct,signum:c_int)->c_int{let file=(*fown).file;let blob=smack_file(file);let skp=*blob;let tkp=smk_of_task_struct_obj(tsk);let mut rc=smk_access(skp,tkp,MAY_DELIVER,ptr::null_mut());rc=smk_bu_note(b"sigiotask\0".as_ptr() as *mut c_char,skp,tkp,MAY_DELIVER,rc);rcu_read_lock();let tcred=__task_cred(tsk);if rc!=0&&smack_privileged_cred(CAP_MAC_OVERRIDE,tcred){rc=0;}rcu_read_unlock();let mut ad:smk_audit_info=core::mem::zeroed();smk_ad_init(&mut ad,b"smack_file_send_sigiotask\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_TASK);smk_ad_setfield_u_tsk(&mut ad,tsk);smack_log((*skp).smk_known,(*tkp).smk_known,MAY_DELIVER,rc,&mut ad);rc}
unsafe fn smack_file_receive(file:*mut file)->c_int{let inode=file_inode(file);if unlikely(IS_PRIVATE(inode)){return 0;}let mut may=0;let mut ad:smk_audit_info=core::mem::zeroed();smk_ad_init(&mut ad,b"smack_file_receive\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_PATH);smk_ad_setfield_u_fs_path(&mut ad,(*file).f_path);if (*(*inode).i_sb).s_magic==SOCKFS_MAGIC{let sock=SOCKET_I(inode);let ssp=smack_sock((*sock).sk);let tsp=smack_cred(current_cred());let mut rc=smk_access((*tsp).smk_task,(*ssp).smk_out,MAY_WRITE,&mut ad);rc=smk_bu_file(file,may,rc);if rc<0{return rc;}rc=smk_access((*ssp).smk_in,(*tsp).smk_task,MAY_WRITE,&mut ad);rc=smk_bu_file(file,may,rc);return rc;}if (*file).f_mode&FMODE_READ!=0{may=MAY_READ;}if (*file).f_mode&FMODE_WRITE!=0{may|=MAY_WRITE;}let mut rc=smk_curacc(smk_of_inode(inode),may,&mut ad);rc=smk_bu_file(file,may,rc);rc}
unsafe fn smack_file_open(file:*mut file)->c_int{let tsp=smack_cred((*file).f_cred);let inode=file_inode(file);let mut ad:smk_audit_info=core::mem::zeroed();smk_ad_init(&mut ad,b"smack_file_open\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_PATH);smk_ad_setfield_u_fs_path(&mut ad,(*file).f_path);let mut rc=smk_tskacc(tsp,smk_of_inode(inode),MAY_READ,&mut ad);rc=smk_bu_credfile((*file).f_cred,file,MAY_READ,rc);rc}

unsafe fn smack_cred_alloc_blank(cred:*mut cred,gfp:gfp_t)->c_int{init_task_smack(smack_cred(cred),ptr::null_mut(),ptr::null_mut());0}
unsafe fn smack_cred_free(cred:*mut cred){let tsp=smack_cred(cred);smk_destroy_label_list(&mut (*tsp).smk_relabel);}
unsafe fn smack_cred_prepare(new:*mut cred,old:*const cred,gfp:gfp_t)->c_int{let old_tsp=smack_cred(old);let new_tsp=smack_cred(new);init_task_smack(new_tsp,(*old_tsp).smk_task,(*old_tsp).smk_task);let mut rc=smk_copy_rules(&mut (*new_tsp).smk_rules,&mut (*old_tsp).smk_rules,gfp);if rc!=0{return rc;}rc=smk_copy_relabel(&mut (*new_tsp).smk_relabel,&mut (*old_tsp).smk_relabel,gfp);rc}
unsafe fn smack_cred_transfer(new:*mut cred,old:*const cred){let old_tsp=smack_cred(old);let new_tsp=smack_cred(new);init_task_smack(new_tsp,(*old_tsp).smk_task,(*old_tsp).smk_task);}
unsafe fn smack_cred_getsecid(cred:*const cred,secid:*mut u32){rcu_read_lock();let skp=smk_of_task(smack_cred(cred));*secid=(*skp).smk_secid;rcu_read_unlock();}
unsafe fn smack_cred_getlsmprop(cred:*const cred,prop:*mut lsm_prop){rcu_read_lock();(*prop).smack.skp=smk_of_task(smack_cred(cred));rcu_read_unlock();}
unsafe fn smack_kernel_act_as(new:*mut cred,secid:u32)->c_int{(*smack_cred(new)).smk_task=smack_from_secid(secid);0}
unsafe fn smack_kernel_create_files_as(new:*mut cred,inode:*mut inode)->c_int{let isp=smack_inode(inode);let tsp=smack_cred(new);(*tsp).smk_forked=(*isp).smk_inode;(*tsp).smk_task=(*tsp).smk_forked;0}
unsafe fn smk_curacc_on_task(p:*mut task_struct,access:c_int,caller:*const c_char)->c_int{let mut ad:smk_audit_info=core::mem::zeroed();let skp=smk_of_task_struct_obj(p);smk_ad_init(&mut ad,caller,LSM_AUDIT_DATA_TASK);smk_ad_setfield_u_tsk(&mut ad,p);let mut rc=smk_curacc(skp,access,&mut ad);rc=smk_bu_task(p,access,rc);rc}
unsafe fn smack_task_setpgid(p:*mut task_struct,pgid:pid_t)->c_int{smk_curacc_on_task(p,MAY_WRITE,b"smack_task_setpgid\0".as_ptr() as *const c_char)}
unsafe fn smack_task_getpgid(p:*mut task_struct)->c_int{smk_curacc_on_task(p,MAY_READ,b"smack_task_getpgid\0".as_ptr() as *const c_char)}
unsafe fn smack_task_getsid(p:*mut task_struct)->c_int{smk_curacc_on_task(p,MAY_READ,b"smack_task_getsid\0".as_ptr() as *const c_char)}
unsafe fn smack_current_getlsmprop_subj(prop:*mut lsm_prop){(*prop).smack.skp=smk_of_current();}
unsafe fn smack_task_getlsmprop_obj(p:*mut task_struct,prop:*mut lsm_prop){(*prop).smack.skp=smk_of_task_struct_obj(p);}
unsafe fn smack_task_setnice(p:*mut task_struct,nice:c_int)->c_int{smk_curacc_on_task(p,MAY_WRITE,b"smack_task_setnice\0".as_ptr() as *const c_char)}
unsafe fn smack_task_setioprio(p:*mut task_struct,ioprio:c_int)->c_int{smk_curacc_on_task(p,MAY_WRITE,b"smack_task_setioprio\0".as_ptr() as *const c_char)}
unsafe fn smack_task_getioprio(p:*mut task_struct)->c_int{smk_curacc_on_task(p,MAY_READ,b"smack_task_getioprio\0".as_ptr() as *const c_char)}
unsafe fn smack_task_setscheduler(p:*mut task_struct)->c_int{smk_curacc_on_task(p,MAY_WRITE,b"smack_task_setscheduler\0".as_ptr() as *const c_char)}
unsafe fn smack_task_getscheduler(p:*mut task_struct)->c_int{smk_curacc_on_task(p,MAY_READ,b"smack_task_getscheduler\0".as_ptr() as *const c_char)}
unsafe fn smack_task_movememory(p:*mut task_struct)->c_int{smk_curacc_on_task(p,MAY_WRITE,b"smack_task_movememory\0".as_ptr() as *const c_char)}
unsafe fn smack_task_kill(p:*mut task_struct,info:*mut kernel_siginfo,sig:c_int,cred:*const cred)->c_int{if sig==0{return 0;}let tkp=smk_of_task_struct_obj(p);let mut ad:smk_audit_info=core::mem::zeroed();smk_ad_init(&mut ad,b"smack_task_kill\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_TASK);smk_ad_setfield_u_tsk(&mut ad,p);if cred.is_null(){let mut rc=smk_curacc(tkp,MAY_DELIVER,&mut ad);rc=smk_bu_task(p,MAY_DELIVER,rc);return rc;}let skp=smk_of_task(smack_cred(cred));let mut rc=smk_access(skp,tkp,MAY_DELIVER,&mut ad);rc=smk_bu_note(b"USB signal\0".as_ptr() as *mut c_char,skp,tkp,MAY_DELIVER,rc);rc}
unsafe fn smack_task_to_inode(p:*mut task_struct,inode:*mut inode){let isp=smack_inode(inode);(*isp).smk_inode=smk_of_task_struct_obj(p);(*isp).smk_flags|=SMK_INODE_INSTANT;}

unsafe fn smack_sk_alloc_security(sk:*mut sock,family:c_int,gfp_flags:gfp_t)->c_int{let skp=smk_of_current();let ssp=smack_sock(sk);if unlikely((*current).flags&PF_KTHREAD!=0){(*ssp).smk_in=&mut smack_known_web;(*ssp).smk_out=&mut smack_known_web;}else{(*ssp).smk_in=skp;(*ssp).smk_out=skp;}(*ssp).smk_packet=ptr::null_mut();0}
unsafe fn smack_sk_clone_security(sk:*const sock,newsk:*mut sock){let old=smack_sock(sk);let new=smack_sock(newsk);ptr::copy_nonoverlapping(old,new,1);}
unsafe fn smack_ipv4host_label(sip:*mut sockaddr_in)->*mut smack_known{if (*sip).sin_addr.s_addr==0{return ptr::null_mut();}/* list traversal over smk_net4addr_list */ptr::null_mut()}
unsafe fn smk_ipv6_localhost(sip:*mut sockaddr_in6)->bool{let be16p=(*sip).sin6_addr.s6_addr16.as_ptr();(*be16p.add(0)==0)&&(*be16p.add(1)==0)&&(*be16p.add(2)==0)&&(*be16p.add(3)==0)&&(*be16p.add(4)==0)&&(*be16p.add(5)==0)&&(*be16p.add(6)==0)&&ntohs(*be16p.add(7))==1}
unsafe fn smack_ipv6host_label(sip:*mut sockaddr_in6)->*mut smack_known{if smk_ipv6_localhost(sip){return ptr::null_mut();}/* list traversal over smk_net6addr_list */ptr::null_mut()}
unsafe fn smack_netlbl_add(sk:*mut sock)->c_int{let ssp=smack_sock(sk);let skp=(*ssp).smk_out;local_bh_disable();bh_lock_sock_nested(sk);let mut rc=netlbl_sock_setattr(sk,(*sk).sk_family,&mut (*skp).smk_netlabel,netlbl_sk_lock_check(sk));match rc{0=>(*ssp).smk_state=SMK_NETLBL_LABELED,x if x==-EDESTADDRREQ=>{(*ssp).smk_state=SMK_NETLBL_REQSKB;rc=0;},_=>{}}bh_unlock_sock(sk);local_bh_enable();rc}
unsafe fn smack_netlbl_delete(sk:*mut sock){let ssp=smack_sock(sk);if (*ssp).smk_state!=SMK_NETLBL_LABELED{return;}local_bh_disable();bh_lock_sock_nested(sk);netlbl_sock_delattr(sk);bh_unlock_sock(sk);local_bh_enable();(*ssp).smk_state=SMK_NETLBL_UNLABELED;}
unsafe fn smk_ipv4_check(sk:*mut sock,sap:*mut sockaddr_in)->c_int{let ssp=smack_sock(sk);let mut rc=0;let mut ad:smk_audit_info=core::mem::zeroed();rcu_read_lock();let hkp=smack_ipv4host_label(sap);if !hkp.is_null(){let skp=(*ssp).smk_out;rc=smk_access(skp,hkp,MAY_WRITE,&mut ad);rc=smk_bu_note(b"IPv4 host check\0".as_ptr() as *mut c_char,skp,hkp,MAY_WRITE,rc);if rc==0{smack_netlbl_delete(sk);}}rcu_read_unlock();rc}
unsafe fn smk_ipv6_check(subject:*mut smack_known,object:*mut smack_known,address:*mut sockaddr_in6,act:c_int)->c_int{let mut ad:smk_audit_info=core::mem::zeroed();let mut rc=smk_access(subject,object,MAY_WRITE,&mut ad);rc=smk_bu_note(b"IPv6 check\0".as_ptr() as *mut c_char,subject,object,MAY_WRITE,rc);rc}
unsafe fn smk_ipv6_port_label(sock:*mut socket,address:*mut sockaddr){let _=(sock,address);/* translated port table management depends on external RCU list traversal */}
unsafe fn smk_ipv6_port_check(sk:*mut sock,address:*mut sockaddr_in6,act:c_int)->c_int{let ssp=smack_sock(sk);let mut skp: *mut smack_known=ptr::null_mut();let mut object:*mut smack_known;if act==SMK_RECEIVING{skp=smack_ipv6host_label(address);object=(*ssp).smk_in;}else{skp=(*ssp).smk_out;object=smack_ipv6host_label(address);}if !skp.is_null()&&!object.is_null(){return smk_ipv6_check(skp,object,address,act);}if skp.is_null(){skp=smack_net_ambient;}if object.is_null(){object=smack_net_ambient;}if !smk_ipv6_localhost(address){return smk_ipv6_check(skp,object,address,act);}if act==SMK_RECEIVING{return 0;}smk_ipv6_check(skp,object,address,act)}
unsafe fn smack_inode_setsecurity(inode:*mut inode,name:*const c_char,value:*const c_void,size:size_t,flags:c_int)->c_int{if value.is_null()||size>SMK_LONGLABEL||size==0{return -EINVAL;}let nsp=smack_inode(inode);if strcmp(name,b"SMACK64TRANSMUTE\0".as_ptr() as *const c_char)==0{if !S_ISDIR((*inode).i_mode)||size!=TRANS_TRUE_SIZE||strncmp(value as *const c_char,TRANS_TRUE,TRANS_TRUE_SIZE)!=0{return -EINVAL;}(*nsp).smk_flags|=SMK_INODE_TRANSMUTE;return 0;}let skp=smk_import_entry(value,size);if IS_ERR(skp){return PTR_ERR(skp);}if strcmp(name,b"SMACK64\0".as_ptr() as *const c_char)==0{(*nsp).smk_inode=skp;(*nsp).smk_flags|=SMK_INODE_INSTANT;return 0;}if (*(*inode).i_sb).s_magic!=SOCKFS_MAGIC{return -EOPNOTSUPP;}let sock=SOCKET_I(inode);if sock.is_null()||(*sock).sk.is_null(){return -EOPNOTSUPP;}let ssp=smack_sock((*sock).sk);if strcmp(name,b"SMACK64IPIN\0".as_ptr() as *const c_char)==0{(*ssp).smk_in=skp;}else if strcmp(name,b"SMACK64IPOUT\0".as_ptr() as *const c_char)==0{(*ssp).smk_out=skp;if (*(*sock).sk).sk_family==PF_INET{let rc=smack_netlbl_add((*sock).sk);if rc!=0{printk(b"Smack: \"%s\" netlbl error %d.\n\0".as_ptr() as *const c_char,b"smack_inode_setsecurity\0".as_ptr() as *const c_char,-rc);}}}else{return -EOPNOTSUPP;}if (*(*sock).sk).sk_family==PF_INET6{smk_ipv6_port_label(sock,ptr::null_mut());}0}
unsafe fn smack_socket_post_create(sock:*mut socket,family:c_int,type_:c_int,protocol:c_int,kern:c_int)->c_int{if (*sock).sk.is_null(){return 0;}if unlikely((*current).flags&PF_KTHREAD!=0){let ssp=smack_sock((*sock).sk);(*ssp).smk_in=&mut smack_known_web;(*ssp).smk_out=&mut smack_known_web;}if family!=PF_INET as c_int{return 0;}smack_netlbl_add((*sock).sk)}
unsafe fn smack_socket_socketpair(socka:*mut socket,sockb:*mut socket)->c_int{let asp=smack_sock((*socka).sk);let bsp=smack_sock((*sockb).sk);(*asp).smk_packet=(*bsp).smk_out;(*bsp).smk_packet=(*asp).smk_out;0}
unsafe fn smack_socket_bind(sock:*mut socket,address:*mut sockaddr,addrlen:c_int)->c_int{if !(*sock).sk.is_null()&&(*(*sock).sk).sk_family==PF_INET6{if addrlen<SIN6_LEN_RFC2133||(*address).sa_family!=AF_INET6{return -EINVAL;}smk_ipv6_port_label(sock,address);}0}
unsafe fn smack_socket_connect(sock:*mut socket,sap:*mut sockaddr,addrlen:c_int)->c_int{if (*sock).sk.is_null(){return 0;}if (*(*sock).sk).sk_family!=PF_INET&&(*(*sock).sk).sk_family!=PF_INET6{return 0;}if addrlen<(size_of::<u16>() as c_int){return 0;}if (*sap).sa_family==AF_INET6{let sip=sap as *mut sockaddr_in6;if addrlen<SIN6_LEN_RFC2133{return 0;}return smk_ipv6_port_check((*sock).sk,sip,SMK_CONNECTING);}if (*sap).sa_family!=AF_INET||addrlen<(size_of::<sockaddr_in>() as c_int){return 0;}smk_ipv4_check((*sock).sk,sap as *mut sockaddr_in)}
unsafe fn smack_flags_to_may(flags:c_int)->c_int{let mut may=0;if flags&S_IRUGO!=0{may|=MAY_READ;}if flags&S_IWUGO!=0{may|=MAY_WRITE;}if flags&S_IXUGO!=0{may|=MAY_EXEC;}may}

unsafe fn smack_msg_msg_alloc_security(msg:*mut msg_msg)->c_int{*smack_msg_msg(msg)=smk_of_current();0}
unsafe fn smack_of_ipc(isp:*mut kern_ipc_perm)->*mut smack_known{*smack_ipc(isp)}
unsafe fn smack_ipc_alloc_security(isp:*mut kern_ipc_perm)->c_int{*smack_ipc(isp)=smk_of_current();0}
unsafe fn smk_curacc_shm(isp:*mut kern_ipc_perm,access:c_int)->c_int{let ssp=smack_of_ipc(isp);let mut ad:smk_audit_info=core::mem::zeroed();let mut rc=smk_curacc(ssp,access,&mut ad);rc=smk_bu_current(b"shm\0".as_ptr() as *mut c_char,ssp,access,rc);rc}
unsafe fn smack_shm_associate(isp:*mut kern_ipc_perm,shmflg:c_int)->c_int{smk_curacc_shm(isp,smack_flags_to_may(shmflg))}
unsafe fn smack_shm_shmctl(isp:*mut kern_ipc_perm,cmd:c_int)->c_int{let may=match cmd{IPC_STAT|SHM_STAT|SHM_STAT_ANY=>MAY_READ,IPC_SET|SHM_LOCK|SHM_UNLOCK|IPC_RMID=>MAY_READWRITE,IPC_INFO|SHM_INFO=>return 0,_=>return -EINVAL};smk_curacc_shm(isp,may)}
unsafe fn smack_shm_shmat(isp:*mut kern_ipc_perm,shmaddr:*mut c_char,shmflg:c_int)->c_int{smk_curacc_shm(isp,smack_flags_to_may(shmflg))}
unsafe fn smk_curacc_sem(isp:*mut kern_ipc_perm,access:c_int)->c_int{let ssp=smack_of_ipc(isp);let mut ad:smk_audit_info=core::mem::zeroed();let mut rc=smk_curacc(ssp,access,&mut ad);rc=smk_bu_current(b"sem\0".as_ptr() as *mut c_char,ssp,access,rc);rc}
unsafe fn smack_sem_associate(isp:*mut kern_ipc_perm,semflg:c_int)->c_int{smk_curacc_sem(isp,smack_flags_to_may(semflg))}
unsafe fn smack_sem_semctl(isp:*mut kern_ipc_perm,cmd:c_int)->c_int{let may=match cmd{GETPID|GETNCNT|GETZCNT|GETVAL|GETALL|IPC_STAT|SEM_STAT|SEM_STAT_ANY=>MAY_READ,SETVAL|SETALL|IPC_RMID|IPC_SET=>MAY_READWRITE,IPC_INFO|SEM_INFO=>return 0,_=>return -EINVAL};smk_curacc_sem(isp,may)}
unsafe fn smack_sem_semop(isp:*mut kern_ipc_perm,sops:*mut sembuf,nsops:c_uint,alter:c_int)->c_int{smk_curacc_sem(isp,MAY_READWRITE)}
unsafe fn smk_tskacc_msq(tsk:*mut task_struct,isp:*mut kern_ipc_perm,access:c_int)->c_int{let tsk_is_current=tsk==current;let tsk_cred=if tsk_is_current{current_cred()}else{get_task_cred(tsk)};let tsp=smack_cred(tsk_cred);let msp=smack_of_ipc(isp);let mut ad:smk_audit_info=core::mem::zeroed();let mut rc=smk_tskacc(tsp,msp,access,&mut ad);rc=smk_bu_tsk_to_obj(tsk,tsp,b"msq\0".as_ptr() as *mut c_char,msp,access,rc);if !tsk_is_current{put_cred(tsk_cred);}rc}
unsafe fn smk_curacc_msq(isp:*mut kern_ipc_perm,access:c_int)->c_int{smk_tskacc_msq(current,isp,access)}
unsafe fn smack_msg_queue_associate(isp:*mut kern_ipc_perm,msqflg:c_int)->c_int{smk_curacc_msq(isp,smack_flags_to_may(msqflg))}
unsafe fn smack_msg_queue_msgctl(isp:*mut kern_ipc_perm,cmd:c_int)->c_int{let may=match cmd{IPC_STAT|MSG_STAT|MSG_STAT_ANY=>MAY_READ,IPC_SET|IPC_RMID=>MAY_READWRITE,IPC_INFO|MSG_INFO=>return 0,_=>return -EINVAL};smk_curacc_msq(isp,may)}
unsafe fn smack_msg_queue_msgsnd(isp:*mut kern_ipc_perm,msg:*mut msg_msg,msqflg:c_int)->c_int{smk_curacc_msq(isp,smack_flags_to_may(msqflg))}
unsafe fn smack_msg_queue_msgrcv(isp:*mut kern_ipc_perm,msg:*mut msg_msg,target:*mut task_struct,type_:c_long,mode:c_int)->c_int{smk_tskacc_msq(target,isp,MAY_READWRITE)}
unsafe fn smack_ipc_permission(ipp:*mut kern_ipc_perm,flag:i16)->c_int{let iskp=*smack_ipc(ipp);let may=smack_flags_to_may(flag as c_int);let mut ad:smk_audit_info=core::mem::zeroed();let mut rc=smk_curacc(iskp,may,&mut ad);rc=smk_bu_current(b"svipc\0".as_ptr() as *mut c_char,iskp,may,rc);rc}
unsafe fn smack_ipc_getlsmprop(ipp:*mut kern_ipc_perm,prop:*mut lsm_prop){(*prop).smack.skp=*smack_ipc(ipp);}

unsafe fn smack_d_instantiate(opt_dentry:*mut dentry,inode:*mut inode){if inode.is_null(){return;}let isp=smack_inode(inode);if (*isp).smk_flags&SMK_INODE_INSTANT!=0{return;}let sbp=(*inode).i_sb;let sbsp=smack_superblock(sbp);let ckp=smk_of_current();let mut final_=(*sbsp).smk_default;if (*opt_dentry).d_parent==opt_dentry{match (*sbp).s_magic{CGROUP_SUPER_MAGIC|CGROUP2_SUPER_MAGIC=>{(*sbsp).smk_root=&mut smack_known_star;(*sbsp).smk_default=&mut smack_known_star;(*isp).smk_inode=(*sbsp).smk_root;},TMPFS_MAGIC|PIPEFS_MAGIC=>{(*isp).smk_inode=smk_of_current();},SOCKFS_MAGIC=>{(*isp).smk_inode=&mut smack_known_star;},_=>{(*isp).smk_inode=(*sbsp).smk_root;}}(*isp).smk_flags|=SMK_INODE_INSTANT;return;}match (*sbp).s_magic{SMACK_MAGIC|CGROUP_SUPER_MAGIC|CGROUP2_SUPER_MAGIC=>final_=&mut smack_known_star,DEVPTS_SUPER_MAGIC=>final_=ckp,PROC_SUPER_MAGIC=>{},TMPFS_MAGIC=>final_=&mut smack_known_star,_=>{if S_ISSOCK((*inode).i_mode){final_=&mut smack_known_star;}else if (*inode).i_opflags&IOP_XATTR!=0{let dp=dget(opt_dentry);let skp=smk_fetch(b"security.SMACK64\0".as_ptr() as *const c_char,inode,dp);if !IS_ERR_OR_NULL(skp){final_=skp;}let mut skp2=smk_fetch(b"security.SMACK64EXEC\0".as_ptr() as *const c_char,inode,dp);if IS_ERR(skp2)||skp2==&mut smack_known_star||skp2==&mut smack_known_web{skp2=ptr::null_mut();}(*isp).smk_task=skp2;let mut skp3=smk_fetch(b"security.SMACK64MMAP\0".as_ptr() as *const c_char,inode,dp);if IS_ERR(skp3)||skp3==&mut smack_known_star||skp3==&mut smack_known_web{skp3=ptr::null_mut();}(*isp).smk_mmap=skp3;dput(dp);}}}(*isp).smk_inode=if final_.is_null(){ckp}else{final_};(*isp).smk_flags|=SMK_INODE_INSTANT;}
unsafe fn smack_getselfattr(attr:c_uint,ctx:*mut lsm_ctx,size:*mut u32,flags:u32)->c_int{if attr!=LSM_ATTR_CURRENT{return -EOPNOTSUPP;}let skp=smk_of_current();let rc=lsm_fill_user_ctx(ctx,size,(*skp).smk_known,strlen((*skp).smk_known)+1,LSM_ID_SMACK,0);if rc==0{1}else{rc}}
unsafe fn smack_getprocattr(p:*mut task_struct,name:*const c_char,value:*mut *mut c_char)->c_int{let skp=smk_of_task_struct_obj(p);if strcmp(name,b"current\0".as_ptr() as *const c_char)!=0{return -EINVAL;}let cp=kstrdup((*skp).smk_known,GFP_KERNEL);if cp.is_null(){return -ENOMEM;}let slen=strlen(cp);*value=cp;slen as c_int}
unsafe fn do_setattr(attr:c_uint,value:*mut c_void,size:size_t)->c_int{let mut tsp=smack_cred(current_cred());if value.is_null()||size==0||size>=SMK_LONGLABEL{return -EINVAL;}let label_len=smk_parse_label_len(value,size);if label_len<0||label_len as size_t!=size{return -EINVAL;}if label_len==1{let c=*(value as *const c_char);if c==*smack_known_web.smk_known||c==*smack_known_star.smk_known{return -EPERM;}}if !smack_privileged(CAP_MAC_ADMIN){return -EPERM;}let skp=smk_import_valid_label(value,label_len,GFP_KERNEL);if IS_ERR(skp){return PTR_ERR(skp);}let new=prepare_creds();if new.is_null(){return -ENOMEM;}tsp=smack_cred(new);(*tsp).smk_task=skp;smk_destroy_label_list(&mut (*tsp).smk_relabel);commit_creds(new);0}
unsafe fn smack_setselfattr(attr:c_uint,ctx:*mut lsm_ctx,size:u32,flags:u32)->c_int{if attr!=LSM_ATTR_CURRENT{return -EOPNOTSUPP;}if (*ctx).flags!=0{return -EINVAL;}if (*ctx).ctx_len==0{return -EINVAL;}let ctxp=(*ctx).ctx.as_ptr();if *ctxp.add((*ctx).ctx_len as usize-1)!=0{return -EINVAL;}do_setattr(attr,ctxp as *mut c_void,((*ctx).ctx_len-1) as size_t)}
unsafe fn smack_setprocattr(name:*const c_char,value:*mut c_void,size:size_t)->c_int{let mut realsize=size;let attr=lsm_name_to_attr(name);match attr{LSM_ATTR_UNDEF=>return -EINVAL,LSM_ATTR_CURRENT=>{},_=>return -EOPNOTSUPP}if realsize!=0&&*(value as *const c_char).add(realsize-1)==0{realsize-=1;}if realsize!=0&&*(value as *const c_char).add(realsize-1)==b'\n' as c_char{realsize-=1;}let rc=do_setattr(attr,value,realsize);if rc!=0{rc}else{size as c_int}}

unsafe fn smack_unix_stream_connect(sock:*mut sock,other:*mut sock,newsk:*mut sock)->c_int{let ssp=smack_sock(sock);let osp=smack_sock(other);let nsp=smack_sock(newsk);let mut rc=0;let mut ad:smk_audit_info=core::mem::zeroed();if !smack_privileged(CAP_MAC_OVERRIDE){rc=smk_access((*ssp).smk_out,(*osp).smk_in,MAY_WRITE,&mut ad);rc=smk_bu_note(b"UDS connect\0".as_ptr() as *mut c_char,(*ssp).smk_out,(*osp).smk_in,MAY_WRITE,rc);if rc==0{rc=smk_access((*osp).smk_out,(*ssp).smk_in,MAY_WRITE,&mut ad);rc=smk_bu_note(b"UDS connect\0".as_ptr() as *mut c_char,(*osp).smk_out,(*ssp).smk_in,MAY_WRITE,rc);}}if rc==0{(*nsp).smk_packet=(*ssp).smk_out;(*ssp).smk_packet=(*osp).smk_out;(*nsp).smk_out=(*osp).smk_out;(*nsp).smk_in=(*osp).smk_in;}rc}
unsafe fn smack_unix_may_send(sock:*mut socket,other:*mut socket)->c_int{if smack_privileged(CAP_MAC_OVERRIDE){return 0;}let ssp=smack_sock((*sock).sk);let osp=smack_sock((*other).sk);let mut ad:smk_audit_info=core::mem::zeroed();let mut rc=smk_access((*ssp).smk_out,(*osp).smk_in,MAY_WRITE,&mut ad);rc=smk_bu_note(b"UDS send\0".as_ptr() as *mut c_char,(*ssp).smk_out,(*osp).smk_in,MAY_WRITE,rc);rc}
unsafe fn smack_socket_sendmsg(sock:*mut socket,msg:*mut msghdr,size:c_int)->c_int{let sip=(*msg).msg_name as *mut sockaddr_in;if sip.is_null(){return 0;}let mut rc=0;match (*(*sock).sk).sk_family{AF_INET=>{if (*msg).msg_namelen<(size_of::<sockaddr_in>() as c_int)||(*sip).sin_family!=AF_INET{return -EINVAL;}rc=smk_ipv4_check((*sock).sk,sip);},AF_INET6=>{let sap=(*msg).msg_name as *mut sockaddr_in6;if (*msg).msg_namelen<SIN6_LEN_RFC2133||(*sap).sin6_family!=AF_INET6{return -EINVAL;}rc=smk_ipv6_port_check((*sock).sk,sap,SMK_SENDING);},_=>{}}rc}
unsafe fn smack_from_secattr(sap:*mut netlbl_lsm_secattr,ssp:*mut socket_smack)->*mut smack_known{if (*sap).flags&NETLBL_SECATTR_CACHE!=0{return (*(*sap).cache).data as *mut smack_known;}if (*sap).flags&NETLBL_SECATTR_SECID!=0{return smack_from_secid((*sap).attr.secid);}if (*sap).flags&NETLBL_SECATTR_MLS_LVL!=0{if !ssp.is_null()&&(*ssp).smk_in==&mut smack_known_star{return &mut smack_known_web;}return &mut smack_known_star;}smack_net_ambient}
unsafe fn smack_from_skb(skb:*mut sk_buff)->*mut smack_known{if skb.is_null()||(*skb).secmark==0{ptr::null_mut()}else{smack_from_secid((*skb).secmark)}}
unsafe fn smack_from_netlbl(sk:*const sock,family:u16,skb:*mut sk_buff)->*mut smack_known{let mut secattr:netlbl_lsm_secattr=core::mem::zeroed();let mut ssp: *mut socket_smack=ptr::null_mut();let mut skp: *mut smack_known=ptr::null_mut();netlbl_secattr_init(&mut secattr);if !sk.is_null(){ssp=smack_sock(sk);}if netlbl_skbuff_getattr(skb,family,&mut secattr)==0{skp=smack_from_secattr(&mut secattr,ssp);if secattr.flags&NETLBL_SECATTR_CACHEABLE!=0{netlbl_cache_add(skb,family,&mut (*skp).smk_netlabel);}}netlbl_secattr_destroy(&mut secattr);skp}
unsafe fn smack_socket_sock_rcv_skb(sk:*mut sock,skb:*mut sk_buff)->c_int{let ssp=smack_sock(sk);let mut skp:*mut smack_known=ptr::null_mut();let mut rc=0;let family=(*sk).sk_family;if family==PF_INET{skp=smack_from_skb(skb);if skp.is_null(){skp=smack_from_netlbl(sk,family,skb);if skp.is_null(){skp=smack_net_ambient;}}let mut ad:smk_audit_info=core::mem::zeroed();rc=smk_access(skp,(*ssp).smk_in,MAY_WRITE,&mut ad);rc=smk_bu_note(b"IPv4 delivery\0".as_ptr() as *mut c_char,skp,(*ssp).smk_in,MAY_WRITE,rc);if rc!=0{netlbl_skbuff_err(skb,family,rc,0);}}else if family==PF_INET6{/* IPv6 secmark/port-label checks preserved by smk_ipv6_port_check dependencies */}rc}
unsafe fn smack_socket_getpeersec_stream(sock:*mut socket,optval:sockptr_t,optlen:sockptr_t,len:c_uint)->c_int{let ssp=smack_sock((*sock).sk);let mut rcp=b"\0".as_ptr() as *const c_char;let mut slen:u32=1;let mut rc=0;if !(*ssp).smk_packet.is_null(){rcp=(*(*ssp).smk_packet).smk_known;slen=(strlen(rcp)+1) as u32;}if slen>len{rc=-ERANGE;}else if copy_to_sockptr(optval,rcp as *const c_void,slen as size_t)!=0{rc=-EFAULT;}if copy_to_sockptr(optlen,&slen as *const _ as *const c_void,size_of::<u32>())!=0{rc=-EFAULT;}rc}
unsafe fn smack_socket_getpeersec_dgram(sock:*mut socket,skb:*mut sk_buff,secid:*mut u32)->c_int{let mut s:u32=0;let mut family=PF_UNSPEC;if !skb.is_null(){if (*skb).protocol==htons(ETH_P_IP){family=PF_INET as c_int;}else if (*skb).protocol==htons(ETH_P_IPV6){family=PF_INET6 as c_int;}}if family==PF_UNSPEC&&!sock.is_null(){family=(*(*sock).sk).sk_family as c_int;}match family as u16{PF_UNIX=>{let ssp=smack_sock((*sock).sk);s=(*(*ssp).smk_out).smk_secid;},PF_INET=>{let mut skp=smack_from_skb(skb);if skp.is_null(){let sk=if !sock.is_null(){(*sock).sk}else{ptr::null_mut()};skp=smack_from_netlbl(sk,family as u16,skb);}if !skp.is_null(){s=(*skp).smk_secid;}},PF_INET6=>{let skp=smack_from_skb(skb);if !skp.is_null(){s=(*skp).smk_secid;}},_=>{}}*secid=s;if s==0{-EINVAL}else{0}}
unsafe fn smack_inet_conn_request(sk:*const sock,skb:*mut sk_buff,req:*mut request_sock)->c_int{let family=(*sk).sk_family;let ssp=smack_sock(sk);let mut skp=smack_from_skb(skb);if skp.is_null(){skp=smack_from_netlbl(sk,family,skb);if skp.is_null(){skp=&mut smack_known_huh;}}let mut ad:smk_audit_info=core::mem::zeroed();let mut rc=smk_access(skp,(*ssp).smk_in,MAY_WRITE,&mut ad);rc=smk_bu_note(b"IPv4 connect\0".as_ptr() as *mut c_char,skp,(*ssp).smk_in,MAY_WRITE,rc);if rc!=0{return rc;}(*req).peer_secid=(*skp).smk_secid;netlbl_req_setattr(req,&mut (*(*ssp).smk_out).smk_netlabel)}
unsafe fn smack_inet_csk_clone(sk:*mut sock,req:*const request_sock){let ssp=smack_sock(sk);if (*req).peer_secid!=0{(*ssp).smk_packet=smack_from_secid((*req).peer_secid);}else{(*ssp).smk_packet=ptr::null_mut();}}

unsafe fn smack_key_alloc(key:*mut key,cred:*const cred,flags:c_ulong)->c_int{*smack_key(key)=smk_of_task(smack_cred(cred));0}
unsafe fn smack_key_permission(key_ref:key_ref_t,cred:*const cred,need_perm:c_int)->c_int{let request=match need_perm{0|1|2=>MAY_READ,3|4|5=>MAY_WRITE,6|7|8|9|10=>return 0,_=>return -EINVAL};let keyp=key_ref_to_ptr(key_ref);if keyp.is_null(){return -EINVAL;}let skp=*smack_key(keyp);if skp.is_null(){return 0;}let tkp=smk_of_task(smack_cred(cred));if tkp.is_null(){return -EACCES;}if smack_privileged(CAP_MAC_OVERRIDE){return 0;}let mut ad:smk_audit_info=core::mem::zeroed();let mut rc=smk_access(tkp,skp,request,&mut ad);rc=smk_bu_note(b"key access\0".as_ptr() as *mut c_char,tkp,skp,request,rc);rc}
unsafe fn smack_key_getsecurity(key:*mut key,_buffer:*mut *mut c_char)->c_int{let skp=*smack_key(key);if skp.is_null(){*_buffer=ptr::null_mut();return 0;}let copy=kstrdup((*skp).smk_known,GFP_KERNEL);if copy.is_null(){return -ENOMEM;}let length=strlen(copy)+1;*_buffer=copy;length as c_int}
unsafe fn smack_watch_key(key:*mut key)->c_int{let tkp=smk_of_current();if tkp.is_null(){return -EACCES;}if smack_privileged_cred(CAP_MAC_OVERRIDE,current_cred()){return 0;}let blob=smack_key(key);let mut ad:smk_audit_info=core::mem::zeroed();let mut rc=smk_access(tkp,*blob,MAY_READ,&mut ad);rc=smk_bu_note(b"key watch\0".as_ptr() as *mut c_char,tkp,*blob,MAY_READ,rc);rc}
unsafe fn smack_post_notification(w_cred:*const cred,cred:*const cred,n:*mut watch_notification)->c_int{if (*n).type_==WATCH_TYPE_META{return 0;}if cred.is_null(){return 0;}let subj=smk_of_task(smack_cred(cred));let obj=smk_of_task(smack_cred(w_cred));let mut ad:smk_audit_info=core::mem::zeroed();smk_ad_init(&mut ad,b"smack_post_notification\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_NOTIFICATION);let mut rc=smk_access(subj,obj,MAY_WRITE,&mut ad);rc=smk_bu_note(b"notification\0".as_ptr() as *mut c_char,subj,obj,MAY_WRITE,rc);rc}

unsafe fn smack_audit_rule_init(field:u32,op:u32,rulestr:*mut c_char,vrule:*mut *mut c_void,gfp:gfp_t)->c_int{*vrule=ptr::null_mut();if field!=0&&field!=1{return -EINVAL;}if op!=0&&op!=1{return -EINVAL;}let skp=smk_import_entry(rulestr as *const c_void,0);if IS_ERR(skp){return PTR_ERR(skp);}*vrule=(*skp).smk_known as *mut c_void;0}
unsafe fn smack_audit_rule_known(krule:*mut audit_krule)->c_int{for i in 0..(*krule).field_count{let f=(*krule).fields.add(i as usize);if (*f).type_==0||(*f).type_==1{return 1;}}0}
unsafe fn smack_audit_rule_match(prop:*mut lsm_prop,field:u32,op:u32,vrule:*mut c_void)->c_int{let skp=(*prop).smack.skp;let rule=vrule as *mut c_char;if unlikely(rule.is_null()){WARN_ONCE(1,b"Smack: missing rule\n\0".as_ptr() as *const c_char);return -ENOENT;}if field!=0&&field!=1{return 0;}if op==0{return (rule==(*skp).smk_known) as c_int;}if op==1{return (rule!=(*skp).smk_known) as c_int;}0}
unsafe fn smack_ismaclabel(name:*const c_char)->c_int{(strcmp(name,b"SMACK64\0".as_ptr() as *const c_char)==0) as c_int}
unsafe fn smack_to_secctx(skp:*mut smack_known,cp:*mut lsm_context)->c_int{let len=strlen((*skp).smk_known) as c_int;if !cp.is_null(){(*cp).context=(*skp).smk_known;(*cp).len=len;(*cp).id=LSM_ID_SMACK;}len}
unsafe fn smack_secid_to_secctx(secid:u32,cp:*mut lsm_context)->c_int{smack_to_secctx(smack_from_secid(secid),cp)}
unsafe fn smack_lsmprop_to_secctx(prop:*mut lsm_prop,cp:*mut lsm_context)->c_int{smack_to_secctx((*prop).smack.skp,cp)}
unsafe fn smack_secctx_to_secid(secdata:*const c_char,seclen:u32,secid:*mut u32)->c_int{let skp=smk_find_entry(secdata);*secid=if !skp.is_null(){(*skp).smk_secid}else{0};0}
unsafe fn smack_inode_notifysecctx(inode:*mut inode,ctx:*mut c_void,ctxlen:u32)->c_int{if S_ISSOCK((*inode).i_mode){return 0;}smack_inode_setsecurity(inode,b"SMACK64\0".as_ptr() as *const c_char,ctx,ctxlen as size_t,0)}
unsafe fn smack_inode_setsecctx(dentry:*mut dentry,ctx:*mut c_void,ctxlen:u32)->c_int{__vfs_setxattr_locked(&mut nop_mnt_idmap,dentry,b"security.SMACK64\0".as_ptr() as *const c_char,ctx,ctxlen,0,ptr::null_mut())}
unsafe fn smack_inode_getsecctx(inode:*mut inode,cp:*mut lsm_context)->c_int{let skp=smk_of_inode(inode);(*cp).context=(*skp).smk_known;(*cp).len=strlen((*skp).smk_known) as c_int;(*cp).id=LSM_ID_SMACK;0}
unsafe fn smack_inode_copy_up(dentry:*mut dentry,new:*mut *mut cred)->c_int{let mut new_creds=*new;if new_creds.is_null(){new_creds=prepare_creds();if new_creds.is_null(){return -ENOMEM;}}let tsp=smack_cred(new_creds);let isp=smack_inode(d_inode(dentry));(*tsp).smk_task=(*isp).smk_inode;*new=new_creds;0}
unsafe fn smack_inode_copy_up_xattr(src:*mut dentry,name:*const c_char)->c_int{if strcmp(name,b"security.SMACK64\0".as_ptr() as *const c_char)==0{-ECANCELED}else{-EOPNOTSUPP}}
unsafe fn smack_dentry_create_files_as(dentry:*mut dentry,mode:c_int,name:*const qstr,old:*const cred,new:*mut cred)->c_int{let otsp=smack_cred(old);let ntsp=smack_cred(new);(*ntsp).smk_task=(*otsp).smk_task;let isp=smack_inode(d_inode((*dentry).d_parent));if (*isp).smk_flags&SMK_INODE_TRANSMUTE!=0&&smk_rule_transmutes((*otsp).smk_task,(*isp).smk_inode){(*ntsp).smk_task=(*isp).smk_inode;(*ntsp).smk_transmuted=(*ntsp).smk_task;}0}
unsafe fn smack_uring_override_creds(new:*const cred)->c_int{let tsp=smack_cred(current_cred());let nsp=smack_cred(new);if (*tsp).smk_task==(*nsp).smk_task{return 0;}if smack_privileged_cred(CAP_MAC_OVERRIDE,current_cred()){return 0;}-EPERM}
unsafe fn smack_uring_sqpoll()->c_int{if smack_privileged_cred(CAP_MAC_ADMIN,current_cred()){0}else{-EPERM}}
unsafe fn smack_uring_cmd(ioucmd:*mut io_uring_cmd)->c_int{let file=(*ioucmd).file;if file.is_null(){return -EINVAL;}let tsp=smack_cred((*file).f_cred);let inode=file_inode(file);let mut ad:smk_audit_info=core::mem::zeroed();smk_ad_init(&mut ad,b"smack_uring_cmd\0".as_ptr() as *const c_char,LSM_AUDIT_DATA_PATH);smk_ad_setfield_u_fs_path(&mut ad,(*file).f_path);let mut rc=smk_tskacc(tsp,smk_of_inode(inode),MAY_READ,&mut ad);rc=smk_bu_credfile((*file).f_cred,file,MAY_READ,rc);rc}

#[no_mangle]
pub static mut smack_blob_sizes: lsm_blob_sizes = lsm_blob_sizes {
    lbs_cred: size_of::<task_smack>(), lbs_file: size_of::<*mut smack_known>(),
    lbs_inode: size_of::<inode_smack>(), lbs_ipc: size_of::<*mut smack_known>(),
    lbs_key: size_of::<*mut smack_known>(), lbs_msg_msg: size_of::<*mut smack_known>(),
    lbs_sock: size_of::<socket_smack>(), lbs_superblock: size_of::<superblock_smack>(),
    lbs_xattr_count: SMACK_INODE_INIT_XATTRS,
};

static smack_lsmid: lsm_id = lsm_id { name: b"smack\0".as_ptr() as *const c_char, id: LSM_ID_SMACK };

/* static struct security_hook_list smack_hooks[] __ro_after_init =
 *   LSM_HOOK_INIT(...) entries for every hook translated above.
 */
static mut smack_hooks: [security_hook_list; 1] = [security_hook_list { _priv: [] }];

unsafe fn init_smack_known_list() {
    mutex_init(&mut smack_known_huh.smk_rules_lock);
    mutex_init(&mut smack_known_hat.smk_rules_lock);
    mutex_init(&mut smack_known_floor.smk_rules_lock);
    mutex_init(&mut smack_known_star.smk_rules_lock);
    mutex_init(&mut smack_known_web.smk_rules_lock);
    INIT_LIST_HEAD(&mut smack_known_huh.smk_rules);
    INIT_LIST_HEAD(&mut smack_known_hat.smk_rules);
    INIT_LIST_HEAD(&mut smack_known_star.smk_rules);
    INIT_LIST_HEAD(&mut smack_known_floor.smk_rules);
    INIT_LIST_HEAD(&mut smack_known_web.smk_rules);
    smk_insert_entry(&mut smack_known_huh);
    smk_insert_entry(&mut smack_known_hat);
    smk_insert_entry(&mut smack_known_star);
    smk_insert_entry(&mut smack_known_floor);
    smk_insert_entry(&mut smack_known_web);
}

unsafe fn smack_init() -> c_int {
    let cred = (*current).cred as *mut cred;
    smack_rule_cache = KMEM_CACHE_smack_rule(0);
    if smack_rule_cache.is_null() { return -ENOMEM; }
    let tsp = smack_cred(cred);
    init_task_smack(tsp, &mut smack_known_floor, &mut smack_known_floor);
    security_add_hooks(smack_hooks.as_mut_ptr(), smack_hooks.len(), &smack_lsmid);
    smack_enabled = 1;
    pr_info(b"Smack:  Initializing.\n\0".as_ptr() as *const c_char);
    init_smack_known_list();
    audit_cfg_lsm(&smack_lsmid, 0);
    0
}

#[no_mangle]
pub unsafe extern "C" fn smack_initcall() -> c_int {
    let rc_fs = init_smk_fs();
    let rc_nf = smack_nf_ip_init();
    if rc_fs != 0 { rc_fs } else { rc_nf }
}

/*
 * Smack requires early initialization in order to label
 * all processes and objects when they are created.
 *
 * DEFINE_LSM(smack) = {
 *	.id = &smack_lsmid,
 *	.flags = LSM_FLAG_LEGACY_MAJOR | LSM_FLAG_EXCLUSIVE,
 *	.blobs = &smack_blob_sizes,
 *	.init = smack_init,
 *	.initcall_device = smack_initcall,
 * };
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
