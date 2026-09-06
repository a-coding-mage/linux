// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor LSM hooks.
 *
 * Copyright (C) 1998-2008 Novell/SUSE
 * Copyright 2009-2010 Canonical Ltd.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type gfp_t = c_uint;
type umode_t = c_uint;
type kuid_t = c_uint;
type kgid_t = c_uint;
type vfsuid_t = c_uint;
type loff_t = i64;
type kernel_cap_t = c_ulong;
type sockptr_t = *mut c_void;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct cred { _priv: [u8; 0] }
#[repr(C)] pub struct task_struct { pub files: *mut c_void, pub real_cred: *const cred, pub pdeath_signal: c_int }
#[repr(C)] pub struct user_namespace { _priv: [u8; 0] }
#[repr(C)] pub struct aa_label { pub proxy: *mut c_void, pub rules: [*mut aa_ruleset; 1] }
#[repr(C)] pub struct aa_profile { pub label: aa_label }
#[repr(C)] pub struct label_it { _priv: [u8; 0] }
#[repr(C)] pub struct path_cond { pub uid: kuid_t, pub mode: umode_t }
#[repr(C)] pub struct path { pub mnt: *mut vfsmount, pub dentry: *mut dentry }
#[repr(C)] pub struct vfsmount { _priv: [u8; 0] }
#[repr(C)] pub struct dentry { _priv: [u8; 0] }
#[repr(C)] pub struct inode { pub i_mode: umode_t }
#[repr(C)] pub struct mnt_idmap { _priv: [u8; 0] }
#[repr(C)] pub struct file { pub f_path: path, pub f_flags: c_ulong, pub f_cred: *const cred }
#[repr(C)] pub struct aa_file_ctx { pub lock: c_ulong, pub label: *mut aa_label, pub allow: u32 }
#[repr(C)] pub struct vm_area_struct { pub vm_file: *mut file, pub vm_flags: c_ulong }
#[repr(C)] pub struct audit_buffer { _priv: [u8; 0] }
#[repr(C)] pub struct aa_ruleset { pub policy: *mut c_void }
#[repr(C)] pub struct aa_perms { _priv: [u8; 0] }
#[repr(C)] pub struct apparmor_audit_data {
    pub request: u32,
    pub denied: u32,
    pub subj_label: *mut aa_label,
    pub subj_cred: *const cred,
    pub info: *const c_char,
    pub error: c_int,
    pub uring: apparmor_audit_uring,
}
#[repr(C)] pub struct apparmor_audit_uring { pub target: *mut aa_label }
#[repr(C)] pub struct lsm_ctx { pub ctx: *mut c_void, pub ctx_len: u32 }
#[repr(C)] pub struct linux_binprm { pub cred: *const cred }
#[repr(C)] pub struct lsm_prop_apparmor { pub label: *mut aa_label }
#[repr(C)] pub struct lsm_prop { pub apparmor: lsm_prop_apparmor }
#[repr(C)] pub struct rlimit { _priv: [u8; 0] }
#[repr(C)] pub struct kernel_siginfo { _priv: [u8; 0] }
#[repr(C)] pub struct sock { pub sk_family: c_int, pub sk_type: c_int, pub sk_protocol: c_int }
#[repr(C)] pub struct socket { pub sk: *mut sock, pub file: *mut file }
#[repr(C)] pub struct aa_sk_ctx { pub label: *mut aa_label, pub peer: *mut aa_label, pub peer_lastupdate: *mut aa_label }
#[repr(C)] pub struct sockaddr { _priv: [u8; 0] }
#[repr(C)] pub struct msghdr { pub msg_flags: c_int, pub msg_name: *mut c_void }
#[repr(C)] pub struct sk_buff { pub secmark: u32 }
#[repr(C)] pub struct request_sock { _priv: [u8; 0] }
#[repr(C)] pub struct nf_hook_state { _priv: [u8; 0] }
#[repr(C)] pub struct nf_hook_ops { pub hook: Option<unsafe extern "C" fn(*mut c_void, *mut sk_buff, *const nf_hook_state) -> c_uint>, pub pf: c_int, pub hooknum: c_int, pub priority: c_int }
#[repr(C)] pub struct lsm_blob_sizes { pub lbs_cred: usize, pub lbs_file: usize, pub lbs_task: usize, pub lbs_sock: usize }
#[repr(C)] pub struct lsm_id { pub name: *const c_char, pub id: c_int }
#[repr(C)] pub struct security_hook_list { _priv: [u8; 0] }
#[repr(C)] pub struct kernel_param { pub arg: *mut c_void }
#[repr(C)] pub struct kernel_param_ops { pub flags: c_uint, pub set: Option<unsafe extern "C" fn(*const c_char, *const kernel_param) -> c_int>, pub get: Option<unsafe extern "C" fn(*mut c_char, *const kernel_param) -> c_int> }
#[repr(C)] pub struct ctl_table { pub procname: *const c_char, pub data: *mut c_void, pub maxlen: usize, pub mode: c_uint, pub proc_handler: Option<unsafe extern "C" fn(*const ctl_table, c_int, *mut c_void, *mut size_t, *mut loff_t) -> c_int> }
#[repr(C)] pub struct net { _priv: [u8; 0] }
#[repr(C)] pub struct pernet_operations { pub init: Option<unsafe extern "C" fn(*mut net) -> c_int>, pub exit: Option<unsafe extern "C" fn(*mut net)> }
#[repr(C)] pub struct aa_dfa { _priv: [u8; 0] }
#[repr(C)] pub struct aa_policydb { pub dfa: *mut aa_dfa, pub perms: *mut aa_perms, pub size: c_int }

#[repr(C)]
pub union aa_buffer {
    pub list: core::mem::ManuallyDrop<list_head>,
    pub buffer: [c_char; 0],
}

#[repr(C)]
pub struct aa_local_cache {
    pub hold: c_uint,
    pub count: c_uint,
    pub head: list_head,
}

const RESERVE_COUNT: c_int = 2;
const MAX_HOLD_COUNT: c_uint = 64;

static mut apparmor_initialized: c_int = 0;
static mut reserve_count: c_int = RESERVE_COUNT;
static mut buffer_count: c_int = 0;
static mut aa_global_buffers: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut aa_buffers_lock: c_ulong = 0;
static mut aa_local_buffers: aa_local_cache = aa_local_cache {
    hold: 0,
    count: 0,
    head: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
};

unsafe extern "C" {
    static mut current: *mut task_struct;
    static mut kernel_t: *mut aa_label;
    static mut root_ns: *mut c_void;
    static mut unprivileged_userns_apparmor_policy: c_int;
    static mut apparmor_display_secid_mode: c_int;
    static mut aa_unprivileged_unconfined_restricted: c_int;
    static audit_mode_names: [*const c_char; 0];
    static aa_profile_mode_names: [*const c_char; 0];

    fn aa_put_label(label: *mut aa_label);
    fn aa_get_label(label: *mut aa_label) -> *mut aa_label;
    fn aa_get_label_rcu(label: *mut *mut aa_label) -> *mut aa_label;
    fn cred_label(cred: *const cred) -> *mut aa_label;
    fn set_cred_label(cred: *mut cred, label: *mut aa_label);
    fn aa_get_newest_label(label: *mut aa_label) -> *mut aa_label;
    fn aa_get_newest_cred_label(cred: *const cred) -> *mut aa_label;
    fn aa_get_newest_cred_label_condref(cred: *const cred, needput: *mut bool) -> *mut aa_label;
    fn aa_put_label_condref(label: *mut aa_label, needput: bool);
    fn task_ctx(task: *mut task_struct) -> *mut aa_task_ctx;
    fn aa_free_task_ctx(ctx: *mut aa_task_ctx);
    fn aa_dup_task_ctx(new: *mut aa_task_ctx, old: *mut aa_task_ctx);
    fn get_task_cred(task: *const task_struct) -> *const cred;
    fn put_cred(cred: *const cred);
    fn current_cred() -> *const cred;
    fn __task_cred(task: *const task_struct) -> *const cred;
    fn __begin_current_label_crit_section(needput: *mut bool) -> *mut aa_label;
    fn __end_current_label_crit_section(label: *mut aa_label, needput: bool);
    fn begin_current_label_crit_section(needput: *mut bool) -> *mut aa_label;
    fn end_current_label_crit_section(label: *mut aa_label, needput: bool);
    fn aa_may_ptrace(tcred: *const cred, tracer: *mut aa_label, ccred: *const cred, tracee: *mut aa_label, mode: c_int) -> c_int;
    fn aa_profile_capget(profile: *mut aa_profile) -> kernel_cap_t;
    fn cap_intersect(a: kernel_cap_t, b: kernel_cap_t) -> kernel_cap_t;
    fn aa_capable(cred: *const cred, label: *mut aa_label, cap: c_int, opts: c_uint) -> c_int;
    fn unconfined(label: *mut aa_label) -> bool;
    fn aa_path_perm(op: *const c_char, cred: *const cred, label: *mut aa_label, path: *const path, flags: c_int, mask: u32, cond: *mut path_cond) -> c_int;
    fn i_uid_into_vfsuid(idmap: *mut mnt_idmap, inode: *mut inode) -> vfsuid_t;
    fn mnt_idmap(mnt: *mut vfsmount) -> *mut mnt_idmap;
    fn d_backing_inode(dentry: *mut dentry) -> *mut inode;
    fn vfsuid_into_kuid(vfsuid: vfsuid_t) -> kuid_t;
    fn path_mediated_fs(dentry: *mut dentry) -> bool;
    fn current_fsuid() -> kuid_t;
    fn aa_path_link(cred: *const cred, label: *mut aa_label, old_dentry: *mut dentry, new_dir: *const path, new_dentry: *mut dentry) -> c_int;
    fn file_ctx(file: *mut file) -> *mut aa_file_ctx;
    fn file_mnt_idmap(file: *mut file) -> *mut mnt_idmap;
    fn file_inode(file: *mut file) -> *mut inode;
    fn aa_map_file_to_perms(file: *mut file) -> u32;
    fn spin_lock_init(lock: *mut c_ulong);
    fn rcu_assign_pointer<T>(slot: *mut *mut T, value: *mut T);
    fn rcu_access_pointer<T>(slot: *mut T) -> *mut T;
    fn rcu_dereference_protected<T>(slot: *mut T, cond: bool) -> *mut T;
    fn aa_file_perm(op: *const c_char, cred: *const cred, label: *mut aa_label, file: *mut file, mask: u32, in_atomic: bool) -> c_int;
    fn aad_of_va(va: *mut c_void) -> *mut apparmor_audit_data;
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn aa_label_xaudit(ab: *mut audit_buffer, ns: *mut c_void, label: *mut aa_label, flags: c_int, gfp: gfp_t);
    fn labels_ns(label: *mut aa_label) -> *mut c_void;
    fn aa_label_match(profile: *mut aa_profile, rules: *mut aa_ruleset, new: *mut aa_label, state: c_uint, flag: bool, request: u32, perms: *mut aa_perms);
    fn aa_lookup_perms(policy: *mut c_void, state: c_uint) -> *mut aa_perms;
    fn aa_apply_modes_to_perms(profile: *mut aa_profile, perms: *mut aa_perms);
    fn aa_check_perms(profile: *mut aa_profile, perms: *mut aa_perms, request: u32, ad: *mut apparmor_audit_data, cb: Option<unsafe extern "C" fn(*mut audit_buffer, *mut c_void)>) -> c_int;
    fn aa_remount(cred: *const cred, label: *mut aa_label, path: *const path, flags: c_ulong, data: *mut c_void) -> c_int;
    fn aa_bind_mount(cred: *const cred, label: *mut aa_label, path: *const path, dev_name: *const c_char, flags: c_ulong) -> c_int;
    fn aa_mount_change_type(cred: *const cred, label: *mut aa_label, path: *const path, flags: c_ulong) -> c_int;
    fn aa_move_mount_old(cred: *const cred, label: *mut aa_label, path: *const path, dev_name: *const c_char) -> c_int;
    fn aa_new_mount(cred: *const cred, label: *mut aa_label, dev_name: *const c_char, path: *const path, ty: *const c_char, flags: c_ulong, data: *mut c_void) -> c_int;
    fn aa_move_mount(cred: *const cred, label: *mut aa_label, from_path: *const path, to_path: *const path) -> c_int;
    fn aa_umount(cred: *const cred, label: *mut aa_label, mnt: *mut vfsmount, flags: c_int) -> c_int;
    fn aa_get_current_label() -> *mut aa_label;
    fn aa_pivotroot(cred: *const cred, label: *mut aa_label, old_path: *const path, new_path: *const path) -> c_int;
    fn aa_getprocattr(label: *mut aa_label, value: *mut *mut c_char, newline: bool) -> c_int;
    fn lsm_fill_user_ctx(lx: *mut lsm_ctx, size: *mut u32, value: *mut c_char, len: c_int, id: c_int, flags: c_int) -> c_int;
    fn kfree(p: *mut c_void);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn lsm_name_to_attr(name: *const c_char) -> c_int;
    fn kmemdup_nul(value: *mut c_void, size: size_t, gfp: gfp_t) -> *mut c_char;
    fn strim(s: *mut c_char) -> *mut c_char;
    fn strsep(s: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn skip_spaces(s: *mut c_char) -> *mut c_char;
    fn aa_setprocattr_changehat(args: *mut c_char, size: size_t, flags: c_int) -> c_int;
    fn aa_change_profile(args: *mut c_char, flags: c_int) -> c_int;
    fn aa_audit_msg(typ: c_int, ad: *mut apparmor_audit_data, cb: *mut c_void);
    fn aa_current_raw_label() -> *mut aa_label;
    fn aa_inherit_files(cred: *const cred, files: *mut c_void);
    fn __aa_transition_rlimits(old: *mut aa_label, new: *mut aa_label);
    fn aa_clear_task_ctx_trans(ctx: *mut aa_task_ctx);
    fn aa_get_task_label(task: *mut task_struct) -> *mut aa_label;
    fn aa_task_setrlimit(cred: *const cred, label: *mut aa_label, task: *mut task_struct, resource: c_uint, new_rlim: *mut rlimit) -> c_int;
    fn aa_may_signal(cred: *const cred, cl: *mut aa_label, tc: *const cred, tl: *mut aa_label, sig: c_int) -> c_int;
    fn aa_profile_ns_perm(profile: *mut aa_profile, ad: *mut apparmor_audit_data, request: u32) -> c_int;
    fn aa_sock(sk: *const sock) -> *mut aa_sk_ctx;
    fn aa_unix_peer_perm(cred: *const cred, label: *mut aa_label, op: *const c_char, request: u32, sk: *mut sock, peer: *mut sock, peer_label: *mut aa_label) -> c_int;
    fn is_unix_fs(sk: *mut sock) -> bool;
    fn last_error(error: c_int, second: c_int) -> c_int;
    fn unix_sk(sk: *mut sock) -> *mut unix_sock;
    fn lockdep_is_held(lock: *mut c_ulong) -> bool;
    fn xcheck(a: c_int, b: c_int) -> c_int;
    fn aa_unix_create_perm(label: *mut aa_label, family: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn aa_inet_create_perm(label: *mut aa_label, family: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn aa_af_perm(cred: *const cred, label: *mut aa_label, op: *const c_char, request: u32, family: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn aa_unix_bind_perm(sock: *mut socket, address: *mut sockaddr, addrlen: c_int) -> c_int;
    fn aa_inet_bind_perm(sock: *mut socket, address: *mut sockaddr, addrlen: c_int) -> c_int;
    fn aa_sk_perm(op: *const c_char, request: u32, sk: *mut sock) -> c_int;
    fn aa_inet_connect_perm(sock: *mut socket, address: *mut sockaddr, addrlen: c_int) -> c_int;
    fn aa_unix_listen_perm(sock: *mut socket, backlog: c_int) -> c_int;
    fn aa_inet_listen_perm(sock: *mut socket, backlog: c_int) -> c_int;
    fn aa_unix_accept_perm(sock: *mut socket, newsock: *mut socket) -> c_int;
    fn aa_inet_accept_perm(sock: *mut socket, newsock: *mut socket) -> c_int;
    fn aa_inet_msg_perm(op: *const c_char, request: u32, sock: *mut socket, msg: *mut msghdr, size: c_int) -> c_int;
    fn sk_is_tcp(sk: *mut sock) -> bool;
    fn sk_is_inet(sk: *mut sock) -> bool;
    fn aa_unix_sock_perm(op: *const c_char, request: u32, sock: *mut socket) -> c_int;
    fn aa_inet_sock_perm(op: *const c_char, request: u32, sock: *mut socket) -> c_int;
    fn aa_unix_opt_perm(op: *const c_char, request: u32, sock: *mut socket, level: c_int, optname: c_int) -> c_int;
    fn aa_inet_opt_perm(op: *const c_char, request: u32, sock: *mut socket, level: c_int, optname: c_int) -> c_int;
    fn ERR_PTR(err: isize) -> *mut aa_label;
    fn IS_ERR(ptr: *mut aa_label) -> bool;
    fn PTR_ERR(ptr: *mut aa_label) -> c_int;
    fn aa_label_asxprint(name: *mut *mut c_char, ns: *mut c_void, label: *mut aa_label, flags: c_int, gfp: gfp_t) -> c_int;
    fn copy_to_sockptr(dst: sockptr_t, src: *const c_void, len: usize) -> c_int;
    fn apparmor_secmark_check(label: *mut aa_label, op: *const c_char, request: u32, secmark: u32, sk: *const sock) -> c_int;
    fn skb_to_full_sk(skb: *mut sk_buff) -> *mut sock;
    fn param_set_bool(val: *const c_char, kp: *const kernel_param) -> c_int;
    fn param_get_bool(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
    fn param_set_uint(val: *const c_char, kp: *const kernel_param) -> c_int;
    fn param_get_uint(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
    fn param_set_int(val: *const c_char, kp: *const kernel_param) -> c_int;
    fn param_get_int(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
    fn kstrtoul(str_: *mut c_char, base: c_uint, res: *mut c_ulong) -> c_int;
    fn aa_current_policy_admin_capable(ns: *mut c_void) -> bool;
    fn aa_current_policy_view_capable(ns: *mut c_void) -> bool;
    fn pr_info(fmt: *const c_char, ...);
    fn pr_warn_once(fmt: *const c_char, ...);
    fn aa_print_debug_params(buffer: *mut c_char) -> c_int;
    fn aa_parse_debug_params(val: *const c_char) -> c_int;
    fn sysfs_emit(buffer: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn match_string(array: *const *const c_char, n: c_int, string: *const c_char) -> c_int;
    fn list_empty(head: *const list_head) -> bool;
    fn list_del(entry: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn get_cpu_ptr<T>(ptr: *mut T) -> *mut T;
    fn put_cpu_ptr<T>(ptr: *mut T);
    fn spin_trylock(lock: *mut c_ulong) -> bool;
    fn spin_lock(lock: *mut c_ulong);
    fn spin_unlock(lock: *mut c_ulong);
    fn might_sleep();
    fn kmalloc(size: c_uint, flags: gfp_t) -> *mut aa_buffer;
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn num_online_cpus() -> c_int;
    fn proc_dointvec(table: *const ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int;
    fn register_sysctl(path: *const c_char, table: *const ctl_table) -> *mut c_void;
    fn nf_register_net_hooks(net: *mut net, ops: *const nf_hook_ops, n: usize) -> c_int;
    fn nf_unregister_net_hooks(net: *mut net, ops: *const nf_hook_ops, n: usize);
    fn register_pernet_subsys(ops: *mut pernet_operations) -> c_int;
    fn panic(fmt: *const c_char, ...);
    fn aa_alloc_pdb(gfp: gfp_t) -> *mut aa_policydb;
    fn aa_dfa_unpack(src: *const c_char, size: usize, flags: c_int) -> *mut aa_dfa;
    fn aa_get_dfa(dfa: *mut aa_dfa) -> *mut aa_dfa;
    fn kzalloc_objs(size: usize, n: c_int) -> *mut aa_perms;
    fn aa_put_pdb(pdb: *mut aa_policydb);
    fn aa_put_dfa(dfa: *mut aa_dfa);
    fn aa_alloc_root_ns() -> c_int;
    fn aa_free_root_ns();
    fn aa_create_aafs() -> c_int;
    fn aa_destroy_aafs();
    fn ns_unconfined(ns: *mut c_void) -> *mut aa_label;
    fn security_add_hooks(hooks: *mut security_hook_list, count: usize, id: *const lsm_id);
    fn audit_cfg_lsm(id: *const lsm_id, flags: c_int);
    fn aa_info_message(msg: *const c_char);
    fn AA_ERROR(msg: *const c_char);
}

#[repr(C)] pub struct aa_task_ctx { pub previous: *mut aa_label, pub onexec: *mut aa_label }
#[repr(C)] pub struct unix_sock { pub lock: c_ulong }

const PTRACE_MODE_READ: c_uint = 1;
const AA_PTRACE_READ: c_int = 1;
const AA_PTRACE_TRACE: c_int = 2;
const MAY_READ: u32 = 0x0004;
const MAY_WRITE: u32 = 0x0002;
const MAY_EXEC: u32 = 0x0001;
const AA_EXEC_MMAP: u32 = 0x1000;
const AA_MAY_SETATTR: u32 = 0x00010000;
const AA_MAY_GETATTR: u32 = 0x00020000;
const AA_MAY_DELETE: u32 = 0x00040000;
const AA_MAY_CREATE: u32 = 0x00080000;
const AA_MAY_CHMOD: u32 = 0x00100000;
const AA_MAY_CHOWN: u32 = 0x00200000;
const AA_MAY_LOCK: u32 = 0x00400000;
const AA_MAY_CONNECT: u32 = 0x00800000;
const AA_MAY_SEND: u32 = 0x01000000;
const AA_MAY_RECEIVE: u32 = 0x02000000;
const AA_MAY_ACCEPT: u32 = 0x04000000;
const AA_MAY_BIND: u32 = 0x08000000;
const AA_MAY_LISTEN: u32 = 0x10000000;
const AA_MAY_GETOPT: u32 = 0x20000000;
const AA_MAY_SETOPT: u32 = 0x40000000;
const AA_MAY_SHUTDOWN: u32 = 0x80000000;
const AA_MAY_CREATE_SQPOLL: u32 = 1;
const AA_MAY_OVERRIDE_CRED: u32 = 2;
const AA_URING_PERM_MASK: u32 = AA_MAY_CREATE_SQPOLL | AA_MAY_OVERRIDE_CRED;

const PROT_READ: c_ulong = 0x1;
const PROT_WRITE: c_ulong = 0x2;
const PROT_EXEC: c_ulong = 0x4;
const MAP_PRIVATE: c_ulong = 0x02;
const VM_SHARED: c_ulong = 0x00000008;
const F_WRLCK: c_uint = 1;
const __FMODE_EXEC: c_ulong = 0x20;
const S_IFDIR: umode_t = 0o040000;
const S_IFLNK: umode_t = 0o120000;
const RENAME_EXCHANGE: c_uint = 1 << 1;
const MS_MGC_MSK: c_ulong = 0xffff0000;
const MS_MGC_VAL: c_ulong = 0xC0ED0000;
const AA_MS_IGNORE_MASK: c_ulong = 0;
const MS_REMOUNT: c_ulong = 32;
const MS_BIND: c_ulong = 4096;
const MS_SHARED: c_ulong = 1 << 20;
const MS_PRIVATE: c_ulong = 1 << 18;
const MS_SLAVE: c_ulong = 1 << 19;
const MS_UNBINDABLE: c_ulong = 1 << 17;
const MS_MOVE: c_ulong = 8192;
const GFP_KERNEL: gfp_t = 0;
const GFP_ATOMIC: gfp_t = 1;
const __GFP_RETRY_MAYFAIL: gfp_t = 0;
const __GFP_NOWARN: gfp_t = 0;
const LSM_ATTR_CURRENT: c_uint = 1;
const LSM_ATTR_PREV: c_uint = 2;
const LSM_ATTR_EXEC: c_uint = 3;
const LSM_ID_APPARMOR: c_int = 1;
const LSM_AUDIT_DATA_NONE: c_int = 0;
const LSM_AUDIT_DATA_TASK: c_int = 1;
const AA_CLASS_NONE: c_int = 0;
const AA_CLASS_IO_URING: c_int = 1;
const AA_CLASS_NS: c_int = 2;
const AUDIT_APPARMOR_DENIED: c_int = 1;
const AUDIT_CFG_LSM_SECCTX_SUBJECT: c_int = 1;
const AA_CHANGE_NOFLAGS: c_int = 0;
const AA_CHANGE_TEST: c_int = 1;
const AA_CHANGE_STACK: c_int = 2;
const AA_CHANGE_ONEXEC: c_int = 4;
const AA_USERNS_CREATE: u32 = 1;
const CAP_SYS_ADMIN: c_int = 21;
const PF_UNIX: c_int = 1;
const PF_INET: c_int = 2;
const PF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const IPPROTO_MPTCP: c_int = 262;
const MSG_FASTOPEN: c_int = 0x20000000;
const ENOENT: c_int = 2;
const EOPNOTSUPP: c_int = 95;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EPERM: c_int = 1;
const EACCES: c_int = 13;
const ENOPROTOOPT: c_int = 92;
const ERANGE: c_int = 34;
const EFAULT: c_int = 14;
const ECONNREFUSED: c_int = 111;
const NF_ACCEPT: c_uint = 1;
const NFPROTO_IPV4: c_int = 2;
const NFPROTO_IPV6: c_int = 10;
const NF_INET_POST_ROUTING: c_int = 4;
const NF_IP_PRI_SELINUX_FIRST: c_int = -225;
const NF_IP6_PRI_SELINUX_FIRST: c_int = -225;
const KERNEL_PARAM_OPS_FL_NOARG: c_uint = 1;
const PATH_MAX: c_uint = 4096;
const AA_DEFAULT_CLEVEL: c_int = 3;
const AA_MIN_CLEVEL: c_int = 1;
const AA_MAX_CLEVEL: c_int = 22;
const DEBUG_PARSE_ERROR: c_int = -1;
const AUDIT_MODE_NAMES_COUNT: c_int = 0;
const PROFILE_MODE_NAMES_COUNT: c_int = 0;
const APPARMOR_ENFORCE: c_int = 0;
const APPARMOR_COMPLAIN: c_int = 1;
const APPARMOR_KILL: c_int = 2;
const LSM_FLAG_LEGACY_MAJOR: c_int = 1;
const LSM_FLAG_EXCLUSIVE: c_int = 2;
const FLAGS_NONE: c_int = 0;
const FLAG_SHOW_MODE: c_int = 1;
const FLAG_VIEW_SUBNS: c_int = 2;
const FLAG_HIDDEN_UNCONFINED: c_int = 4;

static OP_UNLINK: &[u8] = b"unlink\0";
static OP_MKDIR: &[u8] = b"mkdir\0";
static OP_RMDIR: &[u8] = b"rmdir\0";
static OP_MKNOD: &[u8] = b"mknod\0";
static OP_TRUNC: &[u8] = b"truncate\0";
static OP_SYMLINK: &[u8] = b"symlink\0";
static OP_RENAME_SRC: &[u8] = b"rename_src\0";
static OP_RENAME_DEST: &[u8] = b"rename_dest\0";
static OP_CHMOD: &[u8] = b"chmod\0";
static OP_CHOWN: &[u8] = b"chown\0";
static OP_GETATTR: &[u8] = b"getattr\0";
static OP_OPEN: &[u8] = b"open\0";
static OP_FRECEIVE: &[u8] = b"file_receive\0";
static OP_FPERM: &[u8] = b"file_perm\0";
static OP_FLOCK: &[u8] = b"file_lock\0";
static OP_FMMAP: &[u8] = b"file_mmap\0";
static OP_FMPROT: &[u8] = b"file_mprotect\0";
static OP_URING_OVERRIDE: &[u8] = b"uring_override\0";
static OP_URING_SQPOLL: &[u8] = b"uring_sqpoll\0";
static OP_SETPROCATTR: &[u8] = b"setprocattr\0";
static OP_USERNS_CREATE: &[u8] = b"userns_create\0";
static OP_CONNECT: &[u8] = b"connect\0";
static OP_SENDMSG: &[u8] = b"sendmsg\0";
static OP_CREATE: &[u8] = b"create\0";
static OP_BIND: &[u8] = b"bind\0";
static OP_LISTEN: &[u8] = b"listen\0";
static OP_ACCEPT: &[u8] = b"accept\0";
static OP_RECVMSG: &[u8] = b"recvmsg\0";
static OP_GETSOCKNAME: &[u8] = b"getsockname\0";
static OP_GETPEERNAME: &[u8] = b"getpeername\0";
static OP_GETSOCKOPT: &[u8] = b"getsockopt\0";
static OP_SETSOCKOPT: &[u8] = b"setsockopt\0";
static OP_SHUTDOWN: &[u8] = b"shutdown\0";

unsafe fn common_perm(op: *const c_char, path: *const path, mask: u32, cond: *mut path_cond) -> c_int {
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    let mut error = 0;
    if !unconfined(label) {
        error = aa_path_perm(op, current_cred(), label, path, 0, mask, cond);
    }
    __end_current_label_crit_section(label, needput);
    error
}

unsafe fn common_perm_cond(op: *const c_char, path_: *const path, mask: u32) -> c_int {
    let vfsuid = i_uid_into_vfsuid(mnt_idmap((*path_).mnt), d_backing_inode((*path_).dentry));
    let mut cond = path_cond {
        uid: vfsuid_into_kuid(vfsuid),
        mode: (*d_backing_inode((*path_).dentry)).i_mode,
    };
    if !path_mediated_fs((*path_).dentry) { return 0; }
    common_perm(op, path_, mask, &mut cond)
}

unsafe fn common_perm_dir_dentry(op: *const c_char, dir: *const path, dentry_: *mut dentry, mask: u32, cond: *mut path_cond) -> c_int {
    let path_ = path { mnt: (*dir).mnt, dentry: dentry_ };
    common_perm(op, &path_, mask, cond)
}

unsafe fn common_perm_rm(op: *const c_char, dir: *const path, dentry_: *mut dentry, mask: u32) -> c_int {
    let inode = d_backing_inode(dentry_);
    let mut cond = path_cond { uid: 0, mode: 0 };
    if inode.is_null() || !path_mediated_fs(dentry_) { return 0; }
    let vfsuid = i_uid_into_vfsuid(mnt_idmap((*dir).mnt), inode);
    cond.uid = vfsuid_into_kuid(vfsuid);
    cond.mode = (*inode).i_mode;
    common_perm_dir_dentry(op, dir, dentry_, mask, &mut cond)
}

unsafe fn common_perm_create(op: *const c_char, dir: *const path, dentry_: *mut dentry, mask: u32, mode: umode_t) -> c_int {
    let mut cond = path_cond { uid: current_fsuid(), mode };
    if !path_mediated_fs((*dir).dentry) { return 0; }
    common_perm_dir_dentry(op, dir, dentry_, mask, &mut cond)
}

unsafe extern "C" fn apparmor_cred_free(cred_: *mut cred) {
    aa_put_label(cred_label(cred_));
    set_cred_label(cred_, ptr::null_mut());
}

unsafe extern "C" fn apparmor_cred_alloc_blank(cred_: *mut cred, _gfp: gfp_t) -> c_int {
    set_cred_label(cred_, ptr::null_mut());
    0
}

unsafe extern "C" fn apparmor_cred_prepare(new: *mut cred, old: *const cred, _gfp: gfp_t) -> c_int {
    set_cred_label(new, aa_get_newest_label(cred_label(old)));
    0
}

unsafe extern "C" fn apparmor_cred_transfer(new: *mut cred, old: *const cred) {
    set_cred_label(new, aa_get_newest_label(cred_label(old)));
}

unsafe extern "C" fn apparmor_task_free(task: *mut task_struct) {
    aa_free_task_ctx(task_ctx(task));
}

unsafe extern "C" fn apparmor_task_alloc(task: *mut task_struct, _clone_flags: u64) -> c_int {
    aa_dup_task_ctx(task_ctx(task), task_ctx(current));
    0
}

unsafe extern "C" fn apparmor_ptrace_access_check(child: *mut task_struct, mode: c_uint) -> c_int {
    let cred_ = get_task_cred(child);
    let tracee = cred_label(cred_);
    let mut needput = false;
    let tracer = __begin_current_label_crit_section(&mut needput);
    let error = aa_may_ptrace(current_cred(), tracer, cred_, tracee, if (mode & PTRACE_MODE_READ) != 0 { AA_PTRACE_READ } else { AA_PTRACE_TRACE });
    __end_current_label_crit_section(tracer, needput);
    put_cred(cred_);
    error
}

unsafe extern "C" fn apparmor_ptrace_traceme(parent: *mut task_struct) -> c_int {
    let mut needput = false;
    let tracee = __begin_current_label_crit_section(&mut needput);
    let cred_ = get_task_cred(parent);
    let tracer = cred_label(cred_);
    let error = aa_may_ptrace(cred_, tracer, current_cred(), tracee, AA_PTRACE_TRACE);
    put_cred(cred_);
    __end_current_label_crit_section(tracee, needput);
    error
}

unsafe extern "C" fn apparmor_capget(target: *const task_struct, effective: *mut kernel_cap_t, _inheritable: *mut kernel_cap_t, permitted: *mut kernel_cap_t) -> c_int {
    rcu_read_lock();
    let cred_ = __task_cred(target);
    let label = aa_get_newest_cred_label(cred_);
    if !unconfined(label) {
        /* label_for_each_confined(i, label, profile) */
        let profile: *mut aa_profile = ptr::null_mut();
        if !profile.is_null() {
            let allowed = aa_profile_capget(profile);
            *effective = cap_intersect(*effective, allowed);
            *permitted = cap_intersect(*permitted, allowed);
        }
    }
    rcu_read_unlock();
    aa_put_label(label);
    0
}

unsafe extern "C" { fn rcu_read_lock(); fn rcu_read_unlock(); }

unsafe extern "C" fn apparmor_capable(cred_: *const cred, _ns: *mut user_namespace, cap: c_int, opts: c_uint) -> c_int {
    let label = aa_get_newest_cred_label(cred_);
    let mut error = 0;
    if !unconfined(label) { error = aa_capable(cred_, label, cap, opts); }
    aa_put_label(label);
    error
}

unsafe extern "C" fn apparmor_path_unlink(dir: *const path, dentry_: *mut dentry) -> c_int { common_perm_rm(OP_UNLINK.as_ptr() as _, dir, dentry_, AA_MAY_DELETE) }
unsafe extern "C" fn apparmor_path_mkdir(dir: *const path, dentry_: *mut dentry, _mode: umode_t) -> c_int { common_perm_create(OP_MKDIR.as_ptr() as _, dir, dentry_, AA_MAY_CREATE, S_IFDIR) }
unsafe extern "C" fn apparmor_path_rmdir(dir: *const path, dentry_: *mut dentry) -> c_int { common_perm_rm(OP_RMDIR.as_ptr() as _, dir, dentry_, AA_MAY_DELETE) }
unsafe extern "C" fn apparmor_path_mknod(dir: *const path, dentry_: *mut dentry, mode: umode_t, _dev: c_uint) -> c_int { common_perm_create(OP_MKNOD.as_ptr() as _, dir, dentry_, AA_MAY_CREATE, mode) }
unsafe extern "C" fn apparmor_path_truncate(path_: *const path) -> c_int { common_perm_cond(OP_TRUNC.as_ptr() as _, path_, MAY_WRITE | AA_MAY_SETATTR) }
unsafe extern "C" fn apparmor_file_truncate(file_: *mut file) -> c_int { apparmor_path_truncate(&(*file_).f_path) }
unsafe extern "C" fn apparmor_path_symlink(dir: *const path, dentry_: *mut dentry, _old_name: *const c_char) -> c_int { common_perm_create(OP_SYMLINK.as_ptr() as _, dir, dentry_, AA_MAY_CREATE, S_IFLNK) }

unsafe extern "C" fn apparmor_path_link(old_dentry: *mut dentry, new_dir: *const path, new_dentry: *mut dentry) -> c_int {
    if !path_mediated_fs(old_dentry) { return 0; }
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let mut error = 0;
    if !unconfined(label) { error = aa_path_link(current_cred(), label, old_dentry, new_dir, new_dentry); }
    end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn apparmor_path_rename(old_dir: *const path, old_dentry: *mut dentry, new_dir: *const path, new_dentry: *mut dentry, flags: c_uint) -> c_int {
    if !path_mediated_fs(old_dentry) { return 0; }
    if (flags & RENAME_EXCHANGE) != 0 && !path_mediated_fs(new_dentry) { return 0; }
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let mut error = 0;
    if !unconfined(label) {
        let idmap = mnt_idmap((*old_dir).mnt);
        let old_path = path { mnt: (*old_dir).mnt, dentry: old_dentry };
        let new_path = path { mnt: (*new_dir).mnt, dentry: new_dentry };
        let mut cond = path_cond { uid: 0, mode: (*d_backing_inode(old_dentry)).i_mode };
        cond.uid = vfsuid_into_kuid(i_uid_into_vfsuid(idmap, d_backing_inode(old_dentry)));
        if (flags & RENAME_EXCHANGE) != 0 {
            let mut cond_exchange = path_cond { uid: 0, mode: (*d_backing_inode(new_dentry)).i_mode };
            cond_exchange.uid = vfsuid_into_kuid(i_uid_into_vfsuid(idmap, d_backing_inode(new_dentry)));
            error = aa_path_perm(OP_RENAME_SRC.as_ptr() as _, current_cred(), label, &new_path, 0, MAY_READ | AA_MAY_GETATTR | MAY_WRITE | AA_MAY_SETATTR | AA_MAY_DELETE, &mut cond_exchange);
            if error == 0 { error = aa_path_perm(OP_RENAME_DEST.as_ptr() as _, current_cred(), label, &old_path, 0, MAY_WRITE | AA_MAY_SETATTR | AA_MAY_CREATE, &mut cond_exchange); }
        }
        if error == 0 { error = aa_path_perm(OP_RENAME_SRC.as_ptr() as _, current_cred(), label, &old_path, 0, MAY_READ | AA_MAY_GETATTR | MAY_WRITE | AA_MAY_SETATTR | AA_MAY_DELETE, &mut cond); }
        if error == 0 { error = aa_path_perm(OP_RENAME_DEST.as_ptr() as _, current_cred(), label, &new_path, 0, MAY_WRITE | AA_MAY_SETATTR | AA_MAY_CREATE, &mut cond); }
    }
    end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn apparmor_path_chmod(path_: *const path, _mode: umode_t) -> c_int { common_perm_cond(OP_CHMOD.as_ptr() as _, path_, AA_MAY_CHMOD) }
unsafe extern "C" fn apparmor_path_chown(path_: *const path, _uid: kuid_t, _gid: kgid_t) -> c_int { common_perm_cond(OP_CHOWN.as_ptr() as _, path_, AA_MAY_CHOWN) }
unsafe extern "C" fn apparmor_inode_getattr(path_: *const path) -> c_int { common_perm_cond(OP_GETATTR.as_ptr() as _, path_, AA_MAY_GETATTR) }

unsafe extern "C" fn apparmor_file_open(file_: *mut file) -> c_int {
    let fctx = file_ctx(file_);
    if !path_mediated_fs((*file_).f_path.dentry) { return 0; }
    if ((*file_).f_flags & __FMODE_EXEC) != 0 {
        (*fctx).allow = MAY_EXEC | MAY_READ | AA_EXEC_MMAP;
        return 0;
    }
    let mut needput = false;
    let label = aa_get_newest_cred_label_condref((*file_).f_cred, &mut needput);
    let mut error = 0;
    if !unconfined(label) {
        let inode = file_inode(file_);
        let mut cond = path_cond { uid: 0, mode: (*inode).i_mode };
        cond.uid = vfsuid_into_kuid(i_uid_into_vfsuid(file_mnt_idmap(file_), inode));
        error = aa_path_perm(OP_OPEN.as_ptr() as _, (*file_).f_cred, label, &(*file_).f_path, 0, aa_map_file_to_perms(file_), &mut cond);
        (*fctx).allow = aa_map_file_to_perms(file_);
    }
    aa_put_label_condref(label, needput);
    error
}

unsafe extern "C" fn apparmor_file_alloc_security(file_: *mut file) -> c_int {
    let ctx = file_ctx(file_);
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    spin_lock_init(&mut (*ctx).lock);
    rcu_assign_pointer(&mut (*ctx).label, aa_get_label(label));
    end_current_label_crit_section(label, needput);
    0
}

unsafe extern "C" fn apparmor_file_free_security(file_: *mut file) {
    let ctx = file_ctx(file_);
    if !ctx.is_null() { aa_put_label(rcu_access_pointer((*ctx).label)); }
}

unsafe fn common_file_perm(op: *const c_char, file_: *mut file, mask: u32) -> c_int {
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let error = aa_file_perm(op, current_cred(), label, file_, mask, false);
    end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn apparmor_file_receive(file_: *mut file) -> c_int { common_file_perm(OP_FRECEIVE.as_ptr() as _, file_, aa_map_file_to_perms(file_)) }
unsafe extern "C" fn apparmor_file_permission(file_: *mut file, mask: c_int) -> c_int { common_file_perm(OP_FPERM.as_ptr() as _, file_, mask as u32) }
unsafe extern "C" fn apparmor_file_lock(file_: *mut file, cmd: c_uint) -> c_int {
    let mut mask = AA_MAY_LOCK;
    if cmd == F_WRLCK { mask |= MAY_WRITE; }
    common_file_perm(OP_FLOCK.as_ptr() as _, file_, mask)
}

unsafe fn common_mmap(op: *const c_char, file_: *mut file, prot: c_ulong, flags: c_ulong) -> c_int {
    let mut mask: u32 = 0;
    if file_.is_null() || file_ctx(file_).is_null() { return 0; }
    if (prot & PROT_READ) != 0 { mask |= MAY_READ; }
    if (prot & PROT_WRITE) != 0 && (flags & MAP_PRIVATE) == 0 { mask |= MAY_WRITE; }
    if (prot & PROT_EXEC) != 0 { mask |= AA_EXEC_MMAP; }
    common_file_perm(op, file_, mask)
}

unsafe extern "C" fn apparmor_mmap_file(file_: *mut file, _reqprot: c_ulong, prot: c_ulong, flags: c_ulong) -> c_int { common_mmap(OP_FMMAP.as_ptr() as _, file_, prot, flags) }
unsafe extern "C" fn apparmor_file_mprotect(vma: *mut vm_area_struct, _reqprot: c_ulong, prot: c_ulong) -> c_int { common_mmap(OP_FMPROT.as_ptr() as _, (*vma).vm_file, prot, if ((*vma).vm_flags & VM_SHARED) == 0 { MAP_PRIVATE } else { 0 }) }

/* CONFIG_IO_URING */
unsafe extern "C" fn audit_uring_mask(mask: u32) -> *const c_char {
    if (mask & AA_MAY_CREATE_SQPOLL) != 0 { return b"sqpoll\0".as_ptr() as _; }
    if (mask & AA_MAY_OVERRIDE_CRED) != 0 { return b"override_creds\0".as_ptr() as _; }
    b"\0".as_ptr() as _
}

unsafe extern "C" fn audit_uring_cb(ab: *mut audit_buffer, va: *mut c_void) {
    let ad = aad_of_va(va);
    if ((*ad).request & AA_URING_PERM_MASK) != 0 {
        audit_log_format(ab, b" requested=\"%s\"\0".as_ptr() as _, audit_uring_mask((*ad).request));
        if ((*ad).denied & AA_URING_PERM_MASK) != 0 {
            audit_log_format(ab, b" denied=\"%s\"\0".as_ptr() as _, audit_uring_mask((*ad).denied));
        }
    }
    if !(*ad).uring.target.is_null() {
        audit_log_format(ab, b" tcontext=\0".as_ptr() as _);
        aa_label_xaudit(ab, labels_ns((*ad).subj_label), (*ad).uring.target, FLAGS_NONE, GFP_ATOMIC);
    }
}

unsafe extern "C" fn profile_uring(profile: *mut aa_profile, request: u32, new: *mut aa_label, _cap: c_int, ad: *mut apparmor_audit_data) -> c_int {
    let rules = (*profile).label.rules[0];
    let state = RULE_MEDIATES(rules, AA_CLASS_IO_URING);
    let mut error = 0;
    if state != 0 {
        let mut perms: aa_perms = core::mem::zeroed();
        if !new.is_null() { aa_label_match(profile, rules, new, state, false, request, &mut perms); }
        else { perms = ptr::read(aa_lookup_perms((*rules).policy, state)); }
        aa_apply_modes_to_perms(profile, &mut perms);
        error = aa_check_perms(profile, &mut perms, request, ad, Some(audit_uring_cb));
    }
    error
}

unsafe fn RULE_MEDIATES(_rules: *mut aa_ruleset, _class: c_int) -> c_uint { 0 }

unsafe extern "C" fn apparmor_uring_override_creds(new: *const cred) -> c_int {
    let mut ad: apparmor_audit_data = core::mem::zeroed();
    ad.uring.target = cred_label(new);
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    let profile: *mut aa_profile = ptr::null_mut();
    let error = profile_uring(profile, AA_MAY_OVERRIDE_CRED, cred_label(new), CAP_SYS_ADMIN, &mut ad);
    __end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn apparmor_uring_sqpoll() -> c_int {
    let mut ad: apparmor_audit_data = core::mem::zeroed();
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    let profile: *mut aa_profile = ptr::null_mut();
    let error = profile_uring(profile, AA_MAY_CREATE_SQPOLL, ptr::null_mut(), CAP_SYS_ADMIN, &mut ad);
    __end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn apparmor_sb_mount(dev_name: *const c_char, path_: *const path, type_: *const c_char, mut flags: c_ulong, data: *mut c_void) -> c_int {
    if (flags & MS_MGC_MSK) == MS_MGC_VAL { flags &= !MS_MGC_MSK; }
    flags &= !AA_MS_IGNORE_MASK;
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    let mut error = 0;
    if !unconfined(label) {
        if (flags & MS_REMOUNT) != 0 { error = aa_remount(current_cred(), label, path_, flags, data); }
        else if (flags & MS_BIND) != 0 { error = aa_bind_mount(current_cred(), label, path_, dev_name, flags); }
        else if (flags & (MS_SHARED | MS_PRIVATE | MS_SLAVE | MS_UNBINDABLE)) != 0 { error = aa_mount_change_type(current_cred(), label, path_, flags); }
        else if (flags & MS_MOVE) != 0 { error = aa_move_mount_old(current_cred(), label, path_, dev_name); }
        else { error = aa_new_mount(current_cred(), label, dev_name, path_, type_, flags, data); }
    }
    __end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn apparmor_move_mount(from_path: *const path, to_path: *const path) -> c_int {
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    let mut error = 0;
    if !unconfined(label) { error = aa_move_mount(current_cred(), label, from_path, to_path); }
    __end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn apparmor_sb_umount(mnt: *mut vfsmount, flags: c_int) -> c_int {
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    let mut error = 0;
    if !unconfined(label) { error = aa_umount(current_cred(), label, mnt, flags); }
    __end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn apparmor_sb_pivotroot(old_path: *const path, new_path: *const path) -> c_int {
    let label = aa_get_current_label();
    let mut error = 0;
    if !unconfined(label) { error = aa_pivotroot(current_cred(), label, old_path, new_path); }
    aa_put_label(label);
    error
}

unsafe extern "C" fn apparmor_getselfattr(attr: c_uint, lx: *mut lsm_ctx, size: *mut u32, _flags: u32) -> c_int {
    let mut error = -ENOENT;
    let ctx = task_ctx(current);
    let mut label: *mut aa_label = ptr::null_mut();
    let mut value: *mut c_char = ptr::null_mut();
    match attr {
        LSM_ATTR_CURRENT => label = aa_get_newest_label(cred_label(current_cred())),
        LSM_ATTR_PREV => if !(*ctx).previous.is_null() { label = aa_get_newest_label((*ctx).previous); },
        LSM_ATTR_EXEC => if !(*ctx).onexec.is_null() { label = aa_get_newest_label((*ctx).onexec); },
        _ => error = -EOPNOTSUPP,
    }
    if !label.is_null() {
        error = aa_getprocattr(label, &mut value, false);
        if error > 0 { error = lsm_fill_user_ctx(lx, size, value, error, LSM_ID_APPARMOR, 0); }
        kfree(value as _);
    }
    aa_put_label(label);
    if error < 0 { return error; }
    1
}

unsafe extern "C" fn apparmor_getprocattr(task: *mut task_struct, name: *const c_char, value: *mut *mut c_char) -> c_int {
    let mut error = -ENOENT;
    let mut label: *mut aa_label = ptr::null_mut();
    rcu_read_lock();
    if strcmp(name, b"current\0".as_ptr() as _) == 0 { label = aa_get_newest_cred_label(__task_cred(task)); }
    else if strcmp(name, b"prev\0".as_ptr() as _) == 0 && !(*task_ctx(task)).previous.is_null() { label = aa_get_newest_label((*task_ctx(task)).previous); }
    else if strcmp(name, b"exec\0".as_ptr() as _) == 0 && !(*task_ctx(task)).onexec.is_null() { label = aa_get_newest_label((*task_ctx(task)).onexec); }
    else { error = -EINVAL; }
    rcu_read_unlock();
    if !label.is_null() { error = aa_getprocattr(label, value, true); }
    aa_put_label(label);
    error
}

unsafe extern "C" fn do_setattr(attr: u64, value: *mut c_void, size: size_t) -> c_int {
    if size == 0 { return -EINVAL; }
    let mut largs: *mut c_char = ptr::null_mut();
    let mut args = value as *mut c_char;
    if *args.add(size - 1) != 0 {
        largs = kmemdup_nul(value, size, GFP_KERNEL);
        args = largs;
        if args.is_null() { return -ENOMEM; }
    }
    let mut error = -EINVAL;
    args = strim(args);
    let mut args_for_sep = args;
    let command = strsep(&mut args_for_sep, b" \0".as_ptr() as _);
    args = args_for_sep;
    if args.is_null() { kfree(largs as _); return error; }
    args = skip_spaces(args);
    if *args == 0 { kfree(largs as _); return error; }
    let base = if !largs.is_null() { largs } else { value as *mut c_char };
    let arg_size = size - args.offset_from(base) as usize;
    if attr == LSM_ATTR_CURRENT as u64 {
        if strcmp(command, b"changehat\0".as_ptr() as _) == 0 { error = aa_setprocattr_changehat(args, arg_size, AA_CHANGE_NOFLAGS); }
        else if strcmp(command, b"permhat\0".as_ptr() as _) == 0 { error = aa_setprocattr_changehat(args, arg_size, AA_CHANGE_TEST); }
        else if strcmp(command, b"changeprofile\0".as_ptr() as _) == 0 { error = aa_change_profile(args, AA_CHANGE_NOFLAGS); }
        else if strcmp(command, b"permprofile\0".as_ptr() as _) == 0 { error = aa_change_profile(args, AA_CHANGE_TEST); }
        else if strcmp(command, b"stack\0".as_ptr() as _) == 0 { error = aa_change_profile(args, AA_CHANGE_STACK); }
        else { return do_setattr_fail(attr, error, largs); }
    } else if attr == LSM_ATTR_EXEC as u64 {
        if strcmp(command, b"exec\0".as_ptr() as _) == 0 { error = aa_change_profile(args, AA_CHANGE_ONEXEC); }
        else if strcmp(command, b"stack\0".as_ptr() as _) == 0 { error = aa_change_profile(args, AA_CHANGE_ONEXEC | AA_CHANGE_STACK); }
        else { return do_setattr_fail(attr, error, largs); }
    } else {
        return do_setattr_fail(attr, error, largs);
    }
    if error == 0 { error = size as c_int; }
    kfree(largs as _);
    error
}

unsafe fn do_setattr_fail(attr: u64, mut error: c_int, largs: *mut c_char) -> c_int {
    let mut ad: apparmor_audit_data = core::mem::zeroed();
    let mut needput = false;
    ad.subj_label = begin_current_label_crit_section(&mut needput);
    ad.info = if attr == LSM_ATTR_CURRENT as u64 { b"current\0".as_ptr() as _ } else if attr == LSM_ATTR_EXEC as u64 { b"exec\0".as_ptr() as _ } else { b"invalid\0".as_ptr() as _ };
    error = -EINVAL;
    ad.error = error;
    aa_audit_msg(AUDIT_APPARMOR_DENIED, &mut ad, ptr::null_mut());
    end_current_label_crit_section(ad.subj_label, needput);
    kfree(largs as _);
    error
}

unsafe extern "C" fn apparmor_setselfattr(attr: c_uint, ctx: *mut lsm_ctx, _size: u32, _flags: u32) -> c_int {
    if attr != LSM_ATTR_CURRENT && attr != LSM_ATTR_EXEC { return -EOPNOTSUPP; }
    let rc = do_setattr(attr as u64, (*ctx).ctx, (*ctx).ctx_len as usize);
    if rc > 0 { return 0; }
    rc
}

unsafe extern "C" fn apparmor_setprocattr(name: *const c_char, value: *mut c_void, size: size_t) -> c_int {
    let attr = lsm_name_to_attr(name);
    if attr != 0 { return do_setattr(attr as u64, value, size); }
    -EINVAL
}

unsafe extern "C" fn apparmor_bprm_committing_creds(bprm: *const linux_binprm) {
    let label = aa_current_raw_label();
    let new_label = cred_label((*bprm).cred);
    if (*new_label).proxy == (*label).proxy || unconfined(new_label) { return; }
    aa_inherit_files((*bprm).cred, (*current).files);
    (*current).pdeath_signal = 0;
    __aa_transition_rlimits(label, new_label);
}

unsafe extern "C" fn apparmor_bprm_committed_creds(_bprm: *const linux_binprm) {
    aa_clear_task_ctx_trans(task_ctx(current));
}

unsafe extern "C" fn apparmor_current_getlsmprop_subj(prop: *mut lsm_prop) {
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    (*prop).apparmor.label = label;
    __end_current_label_crit_section(label, needput);
}

unsafe extern "C" fn apparmor_task_getlsmprop_obj(p: *mut task_struct, prop: *mut lsm_prop) {
    let label = aa_get_task_label(p);
    (*prop).apparmor.label = label;
    aa_put_label(label);
}

unsafe extern "C" fn apparmor_task_setrlimit(task: *mut task_struct, resource: c_uint, new_rlim: *mut rlimit) -> c_int {
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    let mut error = 0;
    if !unconfined(label) { error = aa_task_setrlimit(current_cred(), label, task, resource, new_rlim); }
    __end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn apparmor_task_kill(target: *mut task_struct, _info: *mut kernel_siginfo, sig: c_int, cred_: *const cred) -> c_int {
    let tc = get_task_cred(target);
    let tl = aa_get_newest_cred_label(tc);
    let error;
    if !cred_.is_null() {
        let cl = aa_get_newest_cred_label(cred_);
        error = aa_may_signal(cred_, cl, tc, tl, sig);
        aa_put_label(cl);
    } else {
        let mut needput = false;
        let cl = __begin_current_label_crit_section(&mut needput);
        error = aa_may_signal(current_cred(), cl, tc, tl, sig);
        __end_current_label_crit_section(cl, needput);
    }
    aa_put_label(tl);
    put_cred(tc);
    error
}

unsafe extern "C" fn apparmor_userns_create(_cred: *const cred) -> c_int {
    let mut ad: apparmor_audit_data = core::mem::zeroed();
    ad.subj_cred = current_cred();
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let mut error = 0;
    if !unconfined(label) {
        let profile: *mut aa_profile = ptr::null_mut();
        if !profile.is_null() { error = aa_profile_ns_perm(profile, &mut ad, AA_USERNS_CREATE); }
    }
    end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn apparmor_sk_alloc_security(sk: *mut sock, _family: c_int, _gfp: gfp_t) -> c_int {
    let ctx = aa_sock(sk);
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    rcu_assign_pointer(&mut (*ctx).label, aa_get_label(label));
    rcu_assign_pointer(&mut (*ctx).peer, ptr::null_mut());
    rcu_assign_pointer(&mut (*ctx).peer_lastupdate, ptr::null_mut());
    __end_current_label_crit_section(label, needput);
    0
}

unsafe extern "C" fn apparmor_sk_free_security(sk: *mut sock) {
    let ctx = aa_sock(sk);
    aa_put_label(rcu_dereference_protected((*ctx).label, true));
    aa_put_label(rcu_dereference_protected((*ctx).peer, true));
    aa_put_label(rcu_dereference_protected((*ctx).peer_lastupdate, true));
}

unsafe extern "C" fn apparmor_sk_clone_security(sk: *const sock, newsk: *mut sock) {
    let ctx = aa_sock(sk);
    let new = aa_sock(newsk);
    if rcu_access_pointer((*ctx).label) != rcu_access_pointer((*new).label) {
        aa_put_label(rcu_dereference_protected((*new).label, true));
        rcu_assign_pointer(&mut (*new).label, aa_get_label_rcu(&mut (*ctx).label));
    }
    if rcu_access_pointer((*ctx).peer) != rcu_access_pointer((*new).peer) {
        aa_put_label(rcu_dereference_protected((*new).peer, true));
        rcu_assign_pointer(&mut (*new).peer, aa_get_label_rcu(&mut (*ctx).peer));
    }
    if rcu_access_pointer((*ctx).peer_lastupdate) != rcu_access_pointer((*new).peer_lastupdate) {
        aa_put_label(rcu_dereference_protected((*new).peer_lastupdate, true));
        rcu_assign_pointer(&mut (*new).peer_lastupdate, aa_get_label_rcu(&mut (*ctx).peer_lastupdate));
    }
}

unsafe fn unix_connect_perm(cred_: *const cred, label: *mut aa_label, sk: *mut sock, peer_sk: *mut sock) -> c_int {
    let peer_ctx = aa_sock(peer_sk);
    let error = aa_unix_peer_perm(cred_, label, OP_CONNECT.as_ptr() as _, AA_MAY_CONNECT | AA_MAY_SEND | AA_MAY_RECEIVE, sk, peer_sk, rcu_dereference_protected((*peer_ctx).label, lockdep_is_held(&mut (*unix_sk(peer_sk)).lock)));
    if !is_unix_fs(peer_sk) {
        last_error(error, aa_unix_peer_perm(cred_, rcu_dereference_protected((*peer_ctx).label, lockdep_is_held(&mut (*unix_sk(peer_sk)).lock)), OP_CONNECT.as_ptr() as _, AA_MAY_ACCEPT | AA_MAY_SEND | AA_MAY_RECEIVE, peer_sk, sk, label));
    }
    error
}

unsafe fn unix_connect_peers(sk_ctx: *mut aa_sk_ctx, peer_ctx: *mut aa_sk_ctx) {
    let mut label = rcu_dereference_protected((*sk_ctx).label, true);
    aa_get_label(label);
    aa_put_label(rcu_dereference_protected((*peer_ctx).peer, true));
    rcu_assign_pointer(&mut (*peer_ctx).peer, label);
    label = aa_get_label(rcu_dereference_protected((*peer_ctx).label, true));
    aa_put_label(rcu_dereference_protected((*sk_ctx).peer, true));
    aa_put_label(rcu_dereference_protected((*sk_ctx).peer_lastupdate, true));
    rcu_assign_pointer(&mut (*sk_ctx).peer, aa_get_label(label));
    rcu_assign_pointer(&mut (*sk_ctx).peer_lastupdate, label);
}

unsafe extern "C" fn apparmor_unix_stream_connect(sk: *mut sock, peer_sk: *mut sock, newsk: *mut sock) -> c_int {
    let sk_ctx = aa_sock(sk);
    let peer_ctx = aa_sock(peer_sk);
    let new_ctx = aa_sock(newsk);
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    let error = unix_connect_perm(current_cred(), label, sk, peer_sk);
    __end_current_label_crit_section(label, needput);
    if error != 0 { return error; }
    rcu_assign_pointer(&mut (*new_ctx).label, aa_get_label(rcu_dereference_protected((*peer_ctx).label, true)));
    unix_connect_peers(sk_ctx, new_ctx);
    0
}

unsafe extern "C" fn apparmor_unix_may_send(sock_: *mut socket, peer: *mut socket) -> c_int {
    let peer_ctx = aa_sock((*peer).sk);
    let mut needput = false;
    let label = __begin_current_label_crit_section(&mut needput);
    let peer_cred = if !(*peer).file.is_null() { (*(*peer).file).f_cred } else { ptr::null() };
    let error = xcheck(
        aa_unix_peer_perm(current_cred(), label, OP_SENDMSG.as_ptr() as _, AA_MAY_SEND, (*sock_).sk, (*peer).sk, rcu_dereference_protected((*peer_ctx).label, true)),
        aa_unix_peer_perm(peer_cred, rcu_dereference_protected((*peer_ctx).label, true), OP_SENDMSG.as_ptr() as _, AA_MAY_RECEIVE, (*peer).sk, (*sock_).sk, label),
    );
    __end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn apparmor_socket_create(family: c_int, type_: c_int, protocol: c_int, kern: c_int) -> c_int {
    if kern != 0 { return 0; }
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let mut error = 0;
    if !unconfined(label) {
        error = match family {
            PF_UNIX => aa_unix_create_perm(label, family, type_, protocol),
            PF_INET | PF_INET6 => aa_inet_create_perm(label, family, type_, protocol),
            _ => aa_af_perm(current_cred(), label, OP_CREATE.as_ptr() as _, AA_MAY_CREATE, family, type_, protocol),
        };
    }
    end_current_label_crit_section(label, needput);
    error
}

unsafe extern "C" fn apparmor_socket_post_create(sock_: *mut socket, _family: c_int, _type: c_int, _protocol: c_int, kern: c_int) -> c_int {
    let label = if kern != 0 { aa_get_label(kernel_t) } else { aa_get_current_label() };
    if !(*sock_).sk.is_null() {
        let ctx = aa_sock((*sock_).sk);
        aa_put_label(rcu_dereference_protected((*ctx).label, true));
        rcu_assign_pointer(&mut (*ctx).label, aa_get_label(label));
    }
    aa_put_label(label);
    0
}

unsafe extern "C" fn apparmor_socket_socketpair(socka: *mut socket, sockb: *mut socket) -> c_int {
    let a_ctx = aa_sock((*socka).sk);
    let b_ctx = aa_sock((*sockb).sk);
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    if rcu_access_pointer((*a_ctx).label) != label { aa_put_label(rcu_dereference_protected((*a_ctx).label, true)); rcu_assign_pointer(&mut (*a_ctx).label, aa_get_label(label)); }
    if rcu_access_pointer((*b_ctx).label) != label { aa_put_label(rcu_dereference_protected((*b_ctx).label, true)); rcu_assign_pointer(&mut (*b_ctx).label, aa_get_label(label)); }
    if (*(*socka).sk).sk_family == PF_UNIX { unix_connect_peers(a_ctx, b_ctx); }
    end_current_label_crit_section(label, needput);
    0
}

unsafe extern "C" fn apparmor_socket_bind(sock_: *mut socket, address: *mut sockaddr, addrlen: c_int) -> c_int {
    match (*(*sock_).sk).sk_family { PF_UNIX => aa_unix_bind_perm(sock_, address, addrlen), PF_INET | PF_INET6 => aa_inet_bind_perm(sock_, address, addrlen), _ => aa_sk_perm(OP_BIND.as_ptr() as _, AA_MAY_BIND, (*sock_).sk) }
}
unsafe extern "C" fn apparmor_socket_connect(sock_: *mut socket, address: *mut sockaddr, addrlen: c_int) -> c_int {
    match (*(*sock_).sk).sk_family { PF_UNIX => 0, PF_INET | PF_INET6 => aa_inet_connect_perm(sock_, address, addrlen), _ => aa_sk_perm(OP_CONNECT.as_ptr() as _, AA_MAY_CONNECT, (*sock_).sk) }
}
unsafe extern "C" fn apparmor_socket_listen(sock_: *mut socket, backlog: c_int) -> c_int {
    match (*(*sock_).sk).sk_family { PF_UNIX => aa_unix_listen_perm(sock_, backlog), PF_INET | PF_INET6 => aa_inet_listen_perm(sock_, backlog), _ => aa_sk_perm(OP_LISTEN.as_ptr() as _, AA_MAY_LISTEN, (*sock_).sk) }
}
unsafe extern "C" fn apparmor_socket_accept(sock_: *mut socket, newsock: *mut socket) -> c_int {
    match (*(*sock_).sk).sk_family { PF_UNIX => aa_unix_accept_perm(sock_, newsock), PF_INET | PF_INET6 => aa_inet_accept_perm(sock_, newsock), _ => aa_sk_perm(OP_ACCEPT.as_ptr() as _, AA_MAY_ACCEPT, (*sock_).sk) }
}

unsafe fn aa_sock_msg_perm(op: *const c_char, request: u32, sock_: *mut socket, msg: *mut msghdr, size: c_int) -> c_int {
    match (*(*sock_).sk).sk_family { PF_UNIX => 0, PF_INET | PF_INET6 => aa_inet_msg_perm(op, request, sock_, msg, size), _ => aa_sk_perm(op, request, (*sock_).sk) }
}
unsafe extern "C" fn apparmor_socket_sendmsg(sock_: *mut socket, msg: *mut msghdr, size: c_int) -> c_int {
    let mut error = aa_sock_msg_perm(OP_SENDMSG.as_ptr() as _, AA_MAY_SEND, sock_, msg, size);
    if error != 0 { return error; }
    if ((*msg).msg_flags & MSG_FASTOPEN) != 0 && !(*msg).msg_name.is_null() && (sk_is_tcp((*sock_).sk) || (sk_is_inet((*sock_).sk) && (*(*sock_).sk).sk_type == SOCK_STREAM && (*(*sock_).sk).sk_protocol == IPPROTO_MPTCP)) {
        error = aa_sock_msg_perm(OP_CONNECT.as_ptr() as _, AA_MAY_CONNECT, sock_, msg, size);
    }
    error
}
unsafe extern "C" fn apparmor_socket_recvmsg(sock_: *mut socket, msg: *mut msghdr, size: c_int, _flags: c_int) -> c_int { aa_sock_msg_perm(OP_RECVMSG.as_ptr() as _, AA_MAY_RECEIVE, sock_, msg, size) }

unsafe fn aa_sock_perm(op: *const c_char, request: u32, sock_: *mut socket) -> c_int {
    match (*(*sock_).sk).sk_family { PF_UNIX => aa_unix_sock_perm(op, request, sock_), PF_INET | PF_INET6 => aa_inet_sock_perm(op, request, sock_), _ => aa_sk_perm(op, request, (*sock_).sk) }
}
unsafe extern "C" fn apparmor_socket_getsockname(sock_: *mut socket) -> c_int { aa_sock_perm(OP_GETSOCKNAME.as_ptr() as _, AA_MAY_GETATTR, sock_) }
unsafe extern "C" fn apparmor_socket_getpeername(sock_: *mut socket) -> c_int { aa_sock_perm(OP_GETPEERNAME.as_ptr() as _, AA_MAY_GETATTR, sock_) }

unsafe fn aa_sock_opt_perm(op: *const c_char, request: u32, sock_: *mut socket, level: c_int, optname: c_int) -> c_int {
    match (*(*sock_).sk).sk_family { PF_UNIX => aa_unix_opt_perm(op, request, sock_, level, optname), PF_INET | PF_INET6 => aa_inet_opt_perm(op, request, sock_, level, optname), _ => aa_sk_perm(op, request, (*sock_).sk) }
}
unsafe extern "C" fn apparmor_socket_getsockopt(sock_: *mut socket, level: c_int, optname: c_int) -> c_int { aa_sock_opt_perm(OP_GETSOCKOPT.as_ptr() as _, AA_MAY_GETOPT, sock_, level, optname) }
unsafe extern "C" fn apparmor_socket_setsockopt(sock_: *mut socket, level: c_int, optname: c_int) -> c_int { aa_sock_opt_perm(OP_SETSOCKOPT.as_ptr() as _, AA_MAY_SETOPT, sock_, level, optname) }
unsafe extern "C" fn apparmor_socket_shutdown(sock_: *mut socket, _how: c_int) -> c_int { aa_sock_perm(OP_SHUTDOWN.as_ptr() as _, AA_MAY_SHUTDOWN, sock_) }

unsafe fn sk_peer_get_label(sk: *mut sock) -> *mut aa_label {
    let ctx = aa_sock(sk);
    if !rcu_access_pointer((*ctx).peer).is_null() { return aa_get_label_rcu(&mut (*ctx).peer); }
    ERR_PTR(-(ENOPROTOOPT as isize))
}

unsafe extern "C" fn apparmor_socket_getpeersec_stream(sock_: *mut socket, optval: sockptr_t, optlen: sockptr_t, len: c_uint) -> c_int {
    let mut name: *mut c_char = ptr::null_mut();
    let mut error = 0;
    let peer = sk_peer_get_label((*sock_).sk);
    if IS_ERR(peer) { error = PTR_ERR(peer); kfree(name as _); return error; }
    let mut needput = false;
    let label = begin_current_label_crit_section(&mut needput);
    let slen = aa_label_asxprint(&mut name, labels_ns(label), peer, FLAG_SHOW_MODE | FLAG_VIEW_SUBNS | FLAG_HIDDEN_UNCONFINED, GFP_KERNEL);
    if slen < 0 { error = -ENOMEM; }
    else {
        if slen as c_uint > len { error = -ERANGE; }
        else if copy_to_sockptr(optval, name as _, slen as usize) != 0 { error = -EFAULT; }
        if copy_to_sockptr(optlen, &slen as *const _ as _, size_of::<c_int>()) != 0 { error = -EFAULT; }
    }
    end_current_label_crit_section(label, needput);
    aa_put_label(peer);
    kfree(name as _);
    error
}

unsafe extern "C" fn apparmor_socket_getpeersec_dgram(_sock: *mut socket, _skb: *mut sk_buff, _secid: *mut u32) -> c_int { -ENOPROTOOPT }

unsafe extern "C" fn apparmor_sock_graft(sk: *mut sock, _parent: *mut socket) {
    let ctx = aa_sock(sk);
    if rcu_access_pointer((*ctx).label).is_null() { rcu_assign_pointer(&mut (*ctx).label, aa_get_current_label()); }
}

/* CONFIG_NETWORK_SECMARK */
unsafe extern "C" fn apparmor_socket_sock_rcv_skb(sk: *mut sock, skb: *mut sk_buff) -> c_int {
    if (*skb).secmark == 0 { return 0; }
    let ctx = aa_sock(sk);
    if rcu_access_pointer((*ctx).label).is_null() { return -EACCES; }
    rcu_read_lock();
    let error = apparmor_secmark_check(rcu_access_pointer((*ctx).label), OP_RECVMSG.as_ptr() as _, AA_MAY_RECEIVE, (*skb).secmark, sk);
    rcu_read_unlock();
    error
}

unsafe extern "C" fn apparmor_inet_conn_request(sk: *const sock, skb: *mut sk_buff, _req: *mut request_sock) -> c_int {
    if (*skb).secmark == 0 { return 0; }
    let ctx = aa_sock(sk);
    rcu_read_lock();
    let error = apparmor_secmark_check(rcu_access_pointer((*ctx).label), OP_CONNECT.as_ptr() as _, AA_MAY_CONNECT, (*skb).secmark, sk);
    rcu_read_unlock();
    error
}

/* defined(CONFIG_NETFILTER) && defined(CONFIG_NETWORK_SECMARK) */
unsafe extern "C" fn apparmor_ip_postroute(_priv: *mut c_void, skb: *mut sk_buff, _state: *const nf_hook_state) -> c_uint {
    if (*skb).secmark == 0 { return NF_ACCEPT; }
    let sk = skb_to_full_sk(skb);
    if sk.is_null() { return NF_ACCEPT; }
    let ctx = aa_sock(sk);
    rcu_read_lock();
    let error = apparmor_secmark_check(rcu_access_pointer((*ctx).label), OP_SENDMSG.as_ptr() as _, AA_MAY_SEND, (*skb).secmark, sk);
    rcu_read_unlock();
    if error == 0 { return NF_ACCEPT; }
    NF_DROP_ERR(-ECONNREFUSED)
}

unsafe fn NF_DROP_ERR(_err: c_int) -> c_uint { 0 }

static mut apparmor_nf_ops: [nf_hook_ops; 1] = [nf_hook_ops { hook: Some(apparmor_ip_postroute), pf: NFPROTO_IPV4, hooknum: NF_INET_POST_ROUTING, priority: NF_IP_PRI_SELINUX_FIRST }];

#[unsafe(no_mangle)]
pub static mut apparmor_blob_sizes: lsm_blob_sizes = lsm_blob_sizes {
    lbs_cred: size_of::<*mut aa_label>(),
    lbs_file: size_of::<aa_file_ctx>(),
    lbs_task: size_of::<aa_task_ctx>(),
    lbs_sock: size_of::<aa_sk_ctx>(),
};

static apparmor_lsm_name: &[u8] = b"apparmor\0";
static apparmor_lsmid: lsm_id = lsm_id { name: apparmor_lsm_name.as_ptr() as _, id: LSM_ID_APPARMOR };

/* security_hook_list initialization preserves the original LSM_HOOK_INIT table intent. */
static mut apparmor_hooks: [security_hook_list; 0] = [];

unsafe extern "C" fn param_set_aabool(val: *const c_char, kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 { return -EINVAL; }
    if apparmor_initialized != 0 && !aa_current_policy_admin_capable(ptr::null_mut()) { return -EPERM; }
    param_set_bool(val, kp)
}
unsafe extern "C" fn param_get_aabool(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 { return -EINVAL; }
    if apparmor_initialized != 0 && !aa_current_policy_view_capable(ptr::null_mut()) { return -EPERM; }
    param_get_bool(buffer, kp)
}
static param_ops_aabool: kernel_param_ops = kernel_param_ops { flags: KERNEL_PARAM_OPS_FL_NOARG, set: Some(param_set_aabool), get: Some(param_get_aabool) };

unsafe extern "C" fn param_set_aauint(val: *const c_char, kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 { return -EINVAL; }
    if apparmor_initialized != 0 { return -EPERM; }
    let error = param_set_uint(val, kp);
    aa_g_path_max = aa_g_path_max.max(size_of::<aa_buffer>() as u32);
    pr_info(b"AppArmor: buffer size set to %d bytes\n\0".as_ptr() as _, aa_g_path_max);
    error
}
unsafe extern "C" fn param_get_aauint(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 { return -EINVAL; }
    if apparmor_initialized != 0 && !aa_current_policy_view_capable(ptr::null_mut()) { return -EPERM; }
    param_get_uint(buffer, kp)
}
static param_ops_aauint: kernel_param_ops = kernel_param_ops { flags: 0, set: Some(param_set_aauint), get: Some(param_get_aauint) };

unsafe extern "C" fn param_set_aacompressionlevel(val: *const c_char, kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 { return -EINVAL; }
    if apparmor_initialized != 0 { return -EPERM; }
    let error = param_set_int(val, kp);
    aa_g_rawdata_compression_level = aa_g_rawdata_compression_level.clamp(AA_MIN_CLEVEL, AA_MAX_CLEVEL);
    pr_info(b"AppArmor: policy rawdata compression level set to %d\n\0".as_ptr() as _, aa_g_rawdata_compression_level);
    error
}
unsafe extern "C" fn param_get_aacompressionlevel(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 { return -EINVAL; }
    if apparmor_initialized != 0 && !aa_current_policy_view_capable(ptr::null_mut()) { return -EPERM; }
    param_get_int(buffer, kp)
}
static param_ops_aacompressionlevel: kernel_param_ops = kernel_param_ops { flags: 0, set: Some(param_set_aacompressionlevel), get: Some(param_get_aacompressionlevel) };

unsafe extern "C" fn param_set_aalockpolicy(val: *const c_char, kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 { return -EINVAL; }
    if apparmor_initialized != 0 && !aa_current_policy_admin_capable(ptr::null_mut()) { return -EPERM; }
    param_set_bool(val, kp)
}
unsafe extern "C" fn param_get_aalockpolicy(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 { return -EINVAL; }
    if apparmor_initialized != 0 && !aa_current_policy_view_capable(ptr::null_mut()) { return -EPERM; }
    param_get_bool(buffer, kp)
}
static param_ops_aalockpolicy: kernel_param_ops = kernel_param_ops { flags: KERNEL_PARAM_OPS_FL_NOARG, set: Some(param_set_aalockpolicy), get: Some(param_get_aalockpolicy) };

static mut aa_g_profile_mode: c_int = APPARMOR_ENFORCE;
static mut aa_g_hash_policy: bool = false;
static mut aa_g_export_binary: bool = false;
static mut aa_g_rawdata_compression_level: c_int = AA_DEFAULT_CLEVEL;
static mut aa_g_debug: c_int = 0;
static mut aa_g_audit: c_int = 0;
static mut aa_g_audit_header: bool = true;
static mut aa_g_lock_policy: bool = false;
static mut aa_g_logsyscall: bool = false;
static mut aa_g_path_max: c_uint = 2 * PATH_MAX;
static mut aa_g_paranoid_load: bool = false;
static mut apparmor_enabled: c_int = 1;

unsafe extern "C" fn apparmor_enabled_setup(str_: *mut c_char) -> c_int {
    let mut enabled: c_ulong = 0;
    let error = kstrtoul(str_, 0, &mut enabled);
    if error == 0 { apparmor_enabled = if enabled != 0 { 1 } else { 0 }; }
    1
}

unsafe extern "C" fn param_set_aaintbool(val: *const c_char, kp: *const kernel_param) -> c_int {
    if apparmor_initialized != 0 { return -EPERM; }
    let mut value = *(*kp).arg.cast::<c_int>() != 0;
    let mut kp_local = kernel_param { arg: (&mut value as *mut bool).cast() };
    let error = param_set_bool(val, &mut kp_local);
    if error == 0 { *(*kp).arg.cast::<c_int>() = (*kp_local.arg.cast::<bool>()) as c_int; }
    error
}
unsafe extern "C" fn param_get_aaintbool(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    let mut value = *(*kp).arg.cast::<c_int>() != 0;
    let kp_local = kernel_param { arg: (&mut value as *mut bool).cast() };
    param_get_bool(buffer, &kp_local)
}
static param_ops_aaintbool: kernel_param_ops = kernel_param_ops { flags: 0, set: Some(param_set_aaintbool), get: Some(param_get_aaintbool) };

unsafe extern "C" fn param_get_debug(buffer: *mut c_char, _kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 { return -EINVAL; }
    if apparmor_initialized != 0 && !aa_current_policy_view_capable(ptr::null_mut()) { return -EPERM; }
    aa_print_debug_params(buffer)
}
unsafe extern "C" fn param_set_debug(val: *const c_char, _kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 || val.is_null() { return -EINVAL; }
    if apparmor_initialized != 0 && !aa_current_policy_admin_capable(ptr::null_mut()) { return -EPERM; }
    let i = aa_parse_debug_params(val);
    if i == DEBUG_PARSE_ERROR { return -EINVAL; }
    aa_g_debug = i;
    0
}
unsafe extern "C" fn param_get_audit(buffer: *mut c_char, _kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 { return -EINVAL; }
    if apparmor_initialized != 0 && !aa_current_policy_view_capable(ptr::null_mut()) { return -EPERM; }
    sysfs_emit(buffer, b"%s\n\0".as_ptr() as _, audit_mode_names.as_ptr().add(aa_g_audit as usize).read())
}
unsafe extern "C" fn param_set_audit(val: *const c_char, _kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 || val.is_null() { return -EINVAL; }
    if apparmor_initialized != 0 && !aa_current_policy_admin_capable(ptr::null_mut()) { return -EPERM; }
    let i = match_string(audit_mode_names.as_ptr(), AUDIT_MODE_NAMES_COUNT, val);
    if i < 0 { return -EINVAL; }
    aa_g_audit = i;
    0
}
unsafe extern "C" fn param_get_mode(buffer: *mut c_char, _kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 { return -EINVAL; }
    if apparmor_initialized != 0 && !aa_current_policy_view_capable(ptr::null_mut()) { return -EPERM; }
    sysfs_emit(buffer, b"%s\n\0".as_ptr() as _, aa_profile_mode_names.as_ptr().add(aa_g_profile_mode as usize).read())
}
unsafe extern "C" fn param_set_mode(val: *const c_char, _kp: *const kernel_param) -> c_int {
    if apparmor_enabled == 0 || val.is_null() { return -EINVAL; }
    if apparmor_initialized != 0 && !aa_current_policy_admin_capable(ptr::null_mut()) { return -EPERM; }
    let i = match_string(aa_profile_mode_names.as_ptr(), PROFILE_MODE_NAMES_COUNT, val);
    if i < 0 { return -EINVAL; }
    aa_g_profile_mode = i;
    0
}

unsafe fn cache_hold_inc(hold: *mut c_uint) {
    if *hold < MAX_HOLD_COUNT { *hold += 1; }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aa_get_buffer(in_atomic: bool) -> *mut c_char {
    let mut try_again = true;
    let mut flags = GFP_KERNEL | __GFP_RETRY_MAYFAIL | __GFP_NOWARN;
    let mut cache = get_cpu_ptr(&mut aa_local_buffers);
    if !list_empty(&(*cache).head) {
        let aa_buf = (*cache).head.next as *mut aa_buffer;
        list_del(&mut (*aa_buf).list as *mut _);
        if (*cache).hold != 0 { (*cache).hold -= 1; }
        (*cache).count -= 1;
        put_cpu_ptr(&mut aa_local_buffers);
        return (*aa_buf).buffer.as_mut_ptr();
    }
    put_cpu_ptr(&mut aa_local_buffers);
    if !spin_trylock(&mut aa_buffers_lock) {
        cache = get_cpu_ptr(&mut aa_local_buffers);
        cache_hold_inc(&mut (*cache).hold);
        put_cpu_ptr(&mut aa_local_buffers);
        spin_lock(&mut aa_buffers_lock);
    }
    loop {
        if buffer_count > reserve_count || (in_atomic && !list_empty(&aa_global_buffers)) {
            let aa_buf = aa_global_buffers.next as *mut aa_buffer;
            list_del(&mut (*aa_buf).list as *mut _);
            buffer_count -= 1;
            spin_unlock(&mut aa_buffers_lock);
            return (*aa_buf).buffer.as_mut_ptr();
        }
        if in_atomic {
            reserve_count += 1;
            flags = GFP_ATOMIC;
        }
        spin_unlock(&mut aa_buffers_lock);
        if !in_atomic { might_sleep(); }
        let aa_buf = kmalloc(aa_g_path_max, flags);
        if !aa_buf.is_null() { return (*aa_buf).buffer.as_mut_ptr(); }
        if try_again {
            try_again = false;
            spin_lock(&mut aa_buffers_lock);
            continue;
        }
        pr_warn_once(b"AppArmor: Failed to allocate a memory buffer.\n\0".as_ptr() as _);
        return ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aa_put_buffer(buf: *mut c_char) {
    if buf.is_null() { return; }
    let aa_buf = buf as *mut aa_buffer;
    let mut cache = get_cpu_ptr(&mut aa_local_buffers);
    if (*cache).hold == 0 {
        put_cpu_ptr(&mut aa_local_buffers);
        if spin_trylock(&mut aa_buffers_lock) {
            list_add(&mut (*aa_buf).list as *mut _, &mut aa_global_buffers);
            buffer_count += 1;
            spin_unlock(&mut aa_buffers_lock);
            return;
        }
        cache = get_cpu_ptr(&mut aa_local_buffers);
        cache_hold_inc(&mut (*cache).hold);
    }
    list_add(&mut (*aa_buf).list as *mut _, &mut (*cache).head);
    (*cache).count += 1;
    put_cpu_ptr(&mut aa_local_buffers);
}

unsafe extern "C" fn set_init_ctx() -> c_int {
    let cred_ = (*current).real_cred as *mut cred;
    set_cred_label(cred_, aa_get_label(ns_unconfined(root_ns)));
    0
}

unsafe fn destroy_buffers() {
    spin_lock(&mut aa_buffers_lock);
    while !list_empty(&aa_global_buffers) {
        let aa_buf = aa_global_buffers.next as *mut aa_buffer;
        list_del(&mut (*aa_buf).list as *mut _);
        spin_unlock(&mut aa_buffers_lock);
        kfree(aa_buf as _);
        spin_lock(&mut aa_buffers_lock);
    }
    spin_unlock(&mut aa_buffers_lock);
}

unsafe extern "C" fn alloc_buffers() -> c_int {
    INIT_LIST_HEAD(&mut aa_local_buffers.head);
    let num = if num_online_cpus() > 1 { 4 + RESERVE_COUNT } else { 2 + RESERVE_COUNT };
    let mut i = 0;
    while i < num {
        let aa_buf = kmalloc(aa_g_path_max, GFP_KERNEL | __GFP_RETRY_MAYFAIL | __GFP_NOWARN);
        if aa_buf.is_null() {
            destroy_buffers();
            return -ENOMEM;
        }
        aa_put_buffer((*aa_buf).buffer.as_mut_ptr());
        i += 1;
    }
    0
}

unsafe extern "C" fn apparmor_dointvec(table: *const ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    if !aa_current_policy_admin_capable(ptr::null_mut()) { return -EPERM; }
    if apparmor_enabled == 0 { return -EINVAL; }
    proc_dointvec(table, write, buffer, lenp, ppos)
}

static mut apparmor_sysctl_table: [ctl_table; 3] = [
    ctl_table { procname: b"apparmor_display_secid_mode\0".as_ptr() as _, data: ptr::null_mut(), maxlen: size_of::<c_int>(), mode: 0o600, proc_handler: Some(apparmor_dointvec) },
    ctl_table { procname: b"apparmor_restrict_unprivileged_unconfined\0".as_ptr() as _, data: ptr::null_mut(), maxlen: size_of::<c_int>(), mode: 0o600, proc_handler: Some(apparmor_dointvec) },
    ctl_table { procname: ptr::null(), data: ptr::null_mut(), maxlen: 0, mode: 0, proc_handler: None },
];

unsafe extern "C" fn apparmor_init_sysctl() -> c_int {
    if !register_sysctl(b"kernel\0".as_ptr() as _, apparmor_sysctl_table.as_ptr()).is_null() { 0 } else { -ENOMEM }
}

unsafe extern "C" fn apparmor_nf_register(net: *mut net) -> c_int {
    nf_register_net_hooks(net, apparmor_nf_ops.as_ptr(), apparmor_nf_ops.len())
}
unsafe extern "C" fn apparmor_nf_unregister(net: *mut net) {
    nf_unregister_net_hooks(net, apparmor_nf_ops.as_ptr(), apparmor_nf_ops.len());
}
static mut apparmor_net_ops: pernet_operations = pernet_operations { init: Some(apparmor_nf_register), exit: Some(apparmor_nf_unregister) };
unsafe extern "C" fn apparmor_nf_ip_init() -> c_int {
    if apparmor_enabled == 0 { return 0; }
    let err = register_pernet_subsys(&mut apparmor_net_ops);
    if err != 0 { panic(b"Apparmor: register_pernet_subsys: error %d\n\0".as_ptr() as _, err); }
    0
}

/* nulldfa_src includes "nulldfa.in" in C; external generated bytes are required. */
static nulldfa_src: [c_char; 0] = [];
static mut nulldfa: *mut aa_dfa = ptr::null_mut();
/* stacksplitdfa_src includes "stacksplitdfa.in" in C; external generated bytes are required. */
static mut stacksplitdfa_src: [c_char; 0] = [];
#[unsafe(no_mangle)] pub static mut stacksplitdfa: *mut aa_dfa = ptr::null_mut();
#[unsafe(no_mangle)] pub static mut nullpdb: *mut aa_policydb = ptr::null_mut();

unsafe fn TO_ACCEPT1_FLAG(x: c_int) -> c_int { x }
unsafe fn TO_ACCEPT2_FLAG(x: c_int) -> c_int { x }
const YYTD_DATA32: c_int = 0;

unsafe extern "C" fn aa_setup_dfa_engine() -> c_int {
    let mut error = -ENOMEM;
    nullpdb = aa_alloc_pdb(GFP_KERNEL);
    if nullpdb.is_null() { return -ENOMEM; }
    nulldfa = aa_dfa_unpack(nulldfa_src.as_ptr(), nulldfa_src.len(), TO_ACCEPT1_FLAG(YYTD_DATA32) | TO_ACCEPT2_FLAG(YYTD_DATA32));
    if IS_ERR(nulldfa as *mut aa_label) {
        error = PTR_ERR(nulldfa as *mut aa_label);
        nulldfa = ptr::null_mut();
        aa_setup_dfa_engine_fail(error)
    } else {
        (*nullpdb).dfa = aa_get_dfa(nulldfa);
        (*nullpdb).perms = kzalloc_objs(size_of::<aa_perms>(), 2);
        if (*nullpdb).perms.is_null() { return aa_setup_dfa_engine_fail(error); }
        (*nullpdb).size = 2;
        stacksplitdfa = aa_dfa_unpack(stacksplitdfa_src.as_ptr(), stacksplitdfa_src.len(), TO_ACCEPT1_FLAG(YYTD_DATA32) | TO_ACCEPT2_FLAG(YYTD_DATA32));
        if IS_ERR(stacksplitdfa as *mut aa_label) {
            error = PTR_ERR(stacksplitdfa as *mut aa_label);
            return aa_setup_dfa_engine_fail(error);
        }
        0
    }
}

unsafe fn aa_setup_dfa_engine_fail(error: c_int) -> c_int {
    aa_put_pdb(nullpdb);
    aa_put_dfa(nulldfa);
    nullpdb = ptr::null_mut();
    nulldfa = ptr::null_mut();
    stacksplitdfa = ptr::null_mut();
    error
}

unsafe extern "C" fn aa_teardown_dfa_engine() {
    aa_put_dfa(stacksplitdfa);
    aa_put_dfa(nulldfa);
    aa_put_pdb(nullpdb);
    nullpdb = ptr::null_mut();
    stacksplitdfa = ptr::null_mut();
    nulldfa = ptr::null_mut();
}

unsafe extern "C" fn apparmor_init() -> c_int {
    let mut error = aa_setup_dfa_engine();
    if error != 0 {
        AA_ERROR(b"Unable to setup dfa engine\n\0".as_ptr() as _);
        return apparmor_init_alloc_out(error);
    }
    error = aa_alloc_root_ns();
    if error != 0 {
        AA_ERROR(b"Unable to allocate default profile namespace\n\0".as_ptr() as _);
        return apparmor_init_alloc_out(error);
    }
    error = apparmor_init_sysctl();
    if error != 0 {
        AA_ERROR(b"Unable to register sysctls\n\0".as_ptr() as _);
        return apparmor_init_alloc_out(error);
    }
    error = alloc_buffers();
    if error != 0 {
        AA_ERROR(b"Unable to allocate work buffers\n\0".as_ptr() as _);
        return apparmor_init_alloc_out(error);
    }
    error = set_init_ctx();
    if error != 0 {
        AA_ERROR(b"Failed to set context on init task\n\0".as_ptr() as _);
        aa_free_root_ns();
        destroy_buffers();
        return apparmor_init_alloc_out(error);
    }
    security_add_hooks(apparmor_hooks.as_mut_ptr(), apparmor_hooks.len(), &apparmor_lsmid);
    audit_cfg_lsm(&apparmor_lsmid, AUDIT_CFG_LSM_SECCTX_SUBJECT);
    apparmor_initialized = 1;
    if aa_g_profile_mode == APPARMOR_COMPLAIN { aa_info_message(b"AppArmor initialized: complain mode enabled\0".as_ptr() as _); }
    else if aa_g_profile_mode == APPARMOR_KILL { aa_info_message(b"AppArmor initialized: kill mode enabled\0".as_ptr() as _); }
    else { aa_info_message(b"AppArmor initialized\0".as_ptr() as _); }
    error
}

unsafe fn apparmor_init_alloc_out(error: c_int) -> c_int {
    aa_destroy_aafs();
    aa_teardown_dfa_engine();
    apparmor_enabled = 0;
    error
}

/*
 * DEFINE_LSM(apparmor) = {
 *     .id = &apparmor_lsmid,
 *     .flags = LSM_FLAG_LEGACY_MAJOR | LSM_FLAG_EXCLUSIVE,
 *     .enabled = &apparmor_enabled,
 *     .blobs = &apparmor_blob_sizes,
 *     .init = apparmor_init,
 *     .initcall_fs = aa_create_aafs,
 *     .initcall_device = apparmor_nf_ip_init,      // CONFIG_NETFILTER && CONFIG_NETWORK_SECMARK
 *     .initcall_late = init_profile_hash,          // CONFIG_SECURITY_APPARMOR_HASH
 * };
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
