// SPDX-License-Identifier: GPL-2.0-only
// AppArmor security module
// This file contains AppArmor auditing function definitions.
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.

// External dependencies: linux/audit.h, linux/fs.h, linux/lsm_audit.h, linux/sched.h, linux/slab.h
// Local dependencies: file.h, label.h

use std::ffi::{c_char, c_void};
use std::os::raw::{c_int, c_uint, c_ulong};

extern "C" { pub static audit_mode_names: *const *const c_char; }
#[repr(i32)] #[derive(Debug, Clone, Copy)] pub enum audit_mode { AUDIT_NORMAL = 0, AUDIT_QUIET_DENIED, AUDIT_QUIET_ALLOWED, AUDIT_QUIET, AUDIT_NOQUIET, AUDIT_ALL, AUDIT_MODE_NAMES_COUNT }
#[repr(i32)] #[derive(Debug, Clone, Copy)] pub enum audit_type { AUDIT_APPARMOR_AUDIT = 0, AUDIT_APPARMOR_ALLOWED, AUDIT_APPARMOR_DENIED, AUDIT_APPARMOR_HINT, AUDIT_APPARMOR_STATUS, AUDIT_APPARMOR_ERROR, AUDIT_APPARMOR_KILL, AUDIT_APPARMOR_AUTO }

pub const OP_NULL: *const c_char = std::ptr::null();
pub const OP_SYSCTL: &[u8] = b"sysctl"; pub const OP_CAPABLE: &[u8] = b"capable";
pub const OP_UNLINK: &[u8] = b"unlink"; pub const OP_MKDIR: &[u8] = b"mkdir"; pub const OP_RMDIR: &[u8] = b"rmdir"; pub const OP_MKNOD: &[u8] = b"mknod"; pub const OP_TRUNC: &[u8] = b"truncate"; pub const OP_LINK: &[u8] = b"link"; pub const OP_SYMLINK: &[u8] = b"symlink"; pub const OP_RENAME_SRC: &[u8] = b"rename_src"; pub const OP_RENAME_DEST: &[u8] = b"rename_dest"; pub const OP_CHMOD: &[u8] = b"chmod"; pub const OP_CHOWN: &[u8] = b"chown"; pub const OP_GETATTR: &[u8] = b"getattr"; pub const OP_OPEN: &[u8] = b"open";
pub const OP_FRECEIVE: &[u8] = b"file_receive"; pub const OP_FPERM: &[u8] = b"file_perm"; pub const OP_FLOCK: &[u8] = b"file_lock"; pub const OP_FMMAP: &[u8] = b"file_mmap"; pub const OP_FMPROT: &[u8] = b"file_mprotect"; pub const OP_INHERIT: &[u8] = b"file_inherit";
pub const OP_PIVOTROOT: &[u8] = b"pivotroot"; pub const OP_MOUNT: &[u8] = b"mount"; pub const OP_UMOUNT: &[u8] = b"umount";
pub const OP_CREATE: &[u8] = b"create"; pub const OP_POST_CREATE: &[u8] = b"post_create"; pub const OP_BIND: &[u8] = b"bind"; pub const OP_CONNECT: &[u8] = b"connect"; pub const OP_LISTEN: &[u8] = b"listen"; pub const OP_ACCEPT: &[u8] = b"accept"; pub const OP_SENDMSG: &[u8] = b"sendmsg"; pub const OP_RECVMSG: &[u8] = b"recvmsg"; pub const OP_GETSOCKNAME: &[u8] = b"getsockname"; pub const OP_GETPEERNAME: &[u8] = b"getpeername"; pub const OP_GETSOCKOPT: &[u8] = b"getsockopt"; pub const OP_SETSOCKOPT: &[u8] = b"setsockopt"; pub const OP_SHUTDOWN: &[u8] = b"socket_shutdown";
pub const OP_PTRACE: &[u8] = b"ptrace"; pub const OP_SIGNAL: &[u8] = b"signal"; pub const OP_EXEC: &[u8] = b"exec";
pub const OP_CHANGE_HAT: &[u8] = b"change_hat"; pub const OP_CHANGE_PROFILE: &[u8] = b"change_profile"; pub const OP_CHANGE_ONEXEC: &[u8] = b"change_onexec"; pub const OP_STACK: &[u8] = b"stack"; pub const OP_STACK_ONEXEC: &[u8] = b"stack_onexec";
pub const OP_SETPROCATTR: &[u8] = b"setprocattr"; pub const OP_SETRLIMIT: &[u8] = b"setrlimit"; pub const OP_PROF_REPL: &[u8] = b"profile_replace"; pub const OP_PROF_LOAD: &[u8] = b"profile_load"; pub const OP_PROF_RM: &[u8] = b"profile_remove"; pub const OP_USERNS_CREATE: &[u8] = b"userns_create"; pub const OP_URING_OVERRIDE: &[u8] = b"uring_override"; pub const OP_URING_SQPOLL: &[u8] = b"uring_sqpoll";

#[repr(transparent)] pub struct cred { _private: [u8; 0] } #[repr(transparent)] pub struct aa_label { _private: [u8; 0] } #[repr(transparent)] pub struct aa_profile { _private: [u8; 0] } #[repr(transparent)] pub struct aa_perms { _private: [u8; 0] } #[repr(transparent)] pub struct kuid_t { _private: [u8; 0] } #[repr(transparent)] pub struct common_audit_data { _private: [u8; 0] } #[repr(transparent)] pub struct audit_buffer { _private: [u8; 0] } #[repr(transparent)] pub struct audit_krule { _private: [u8; 0] } #[repr(transparent)] pub struct lsm_prop { _private: [u8; 0] }
#[repr(C)] pub struct apparmor_audit_data { pub error: c_int, pub type_: c_int, pub class: u16, pub op: *const c_char, pub subj_cred: *const cred, pub subj_label: *mut aa_label, pub name: *const c_char, pub info: *const c_char, pub request: u32, pub denied: u32, pub tags: u32, pub u: apparmor_audit_data_union, pub common: common_audit_data }
#[repr(C)] pub union apparmor_audit_data_union { pub peer_data: peer_union_data, pub iface: iface_data, pub mnt: mnt_data, pub uring: uring_data }
#[repr(C)] pub struct peer_union_data { pub peer: *mut aa_label, pub inner: peer_inner_union } #[repr(C)] pub union peer_inner_union { pub fs: fs_data, pub rlim: rlim_data, pub signal_data: signal_data, pub net: net_data }
#[repr(C)] pub struct fs_data { pub target: *const c_char, pub ouid: kuid_t } #[repr(C)] pub struct rlim_data { pub rlim: c_int, pub max: c_ulong } #[repr(C)] pub struct signal_data { pub signal: c_int, pub unmappedsig: c_int } #[repr(C)] pub struct net_data { pub type_: c_int, pub protocol: c_int, pub addr: *mut c_void, pub addrlen: c_int, pub peer: net_peer_data } #[repr(C)] pub struct net_peer_data { pub addr: *mut c_void, pub addrlen: c_int }
#[repr(C)] pub struct iface_data { pub profile: *mut aa_profile, pub ns: *const c_char, pub pos: i64 } #[repr(C)] pub struct mnt_data { pub src_name: *const c_char, pub type_: *const c_char, pub trans: *const c_char, pub data: *const c_char, pub flags: c_ulong } #[repr(C)] pub struct uring_data { pub target: *mut aa_label }

// aad(SA) is container_of(SA, struct apparmor_audit_data, common).
// aad_of_va(VA) applies aad to a common_audit_data pointer.
// DEFINE_AUDIT_DATA initializes class, op, common.type, common.u.tsk, and the back-pointer.
// aa_audit_error sets AD->error, audits an error message, and returns AD->error.
extern "C" { pub fn aa_select_audit_type(denied: u32, perms: *const aa_perms) -> c_int; pub fn aa_audit_msg(type_: c_int, ad: *mut apparmor_audit_data, cb: Option<extern "C" fn(*mut audit_buffer, *mut c_void)>); pub fn aa_audit(type_: c_int, profile: *mut aa_profile, ad: *mut apparmor_audit_data, cb: Option<extern "C" fn(*mut audit_buffer, *mut c_void)>) -> c_int; pub fn aa_audit_perm_error(label: *mut aa_label, request: u32, error: c_int, ad: *mut apparmor_audit_data, cb: Option<extern "C" fn(*mut audit_buffer, *mut c_void)>) -> c_int; pub fn aa_audit_rule_free(vrule: *mut c_void); pub fn aa_audit_rule_init(field: u32, op: u32, rulestr: *mut c_char, vrule: *mut *mut c_void, gfp: c_uint) -> c_int; pub fn aa_audit_rule_known(rule: *mut audit_krule) -> c_int; pub fn aa_audit_rule_match(prop: *mut lsm_prop, field: u32, op: u32, vrule: *mut c_void) -> c_int; }
#[inline] pub fn complain_error(error: c_int) -> c_int { if error == -1 || error == -13 { 0 } else { error } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
