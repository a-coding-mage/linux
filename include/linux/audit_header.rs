/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of linux/audit.h.  Included C headers provide the referenced types. */

pub const AUDIT_STATUS_ALL: u32 = AUDIT_STATUS_ENABLED | AUDIT_STATUS_FAILURE |
    AUDIT_STATUS_PID | AUDIT_STATUS_RATE_LIMIT | AUDIT_STATUS_BACKLOG_LIMIT |
    AUDIT_STATUS_BACKLOG_WAIT_TIME | AUDIT_STATUS_LOST |
    AUDIT_STATUS_BACKLOG_WAIT_TIME_ACTUAL;
pub const AUDIT_INO_UNSET: u64 = u64::MAX;
pub const AUDIT_DEV_UNSET: dev_t = -1i32 as dev_t;

#[repr(C)] pub struct audit_sig_info { pub uid: uid_t, pub pid: pid_t, pub ctx: [c_char; 0] }
pub enum audit_buffer {}
pub enum audit_context {}
pub enum inode {}
pub enum netlink_skb_parms {}
pub enum path {}
pub enum linux_binprm {}
pub enum mq_attr {}
pub enum mqstat {}
pub enum audit_watch {}
pub enum audit_tree {}
pub enum sk_buff {}
pub enum kern_ipc_perm {}
pub enum lsm_id {}
pub enum lsm_prop {}

#[repr(C)] pub struct audit_krule {
    pub pflags: u32, pub flags: u32, pub listnr: u32, pub action: u32,
    pub mask: [u32; AUDIT_BITMASK_SIZE as usize], pub buflen: u32, pub field_count: u32,
    pub filterkey: *mut c_char, pub fields: *mut audit_field, pub arch_f: *mut audit_field,
    pub inode_f: *mut audit_field, pub watch: *mut audit_watch, pub tree: *mut audit_tree,
    pub exe: *mut audit_fsnotify_mark, pub rlist: list_head, pub list: list_head, pub prio: u64,
}
pub const AUDIT_LOGINUID_LEGACY: u32 = 0x1;
#[repr(C)] pub union audit_field_union { pub val: u32, pub uid: kuid_t, pub gid: kgid_t,
    pub lsm: lsm_field }
#[repr(C)] pub struct lsm_field { pub lsm_str: *mut c_char, pub lsm_rule: *mut c_void }
#[repr(C)] pub struct audit_field { pub type_: u32, pub u: audit_field_union, pub op: u32 }

#[repr(C)] pub enum audit_ntp_type { AUDIT_NTP_OFFSET, AUDIT_NTP_FREQ, AUDIT_NTP_STATUS,
    AUDIT_NTP_TAI, AUDIT_NTP_TICK, AUDIT_NTP_ADJUST, AUDIT_NTP_NVALS }
#[repr(C)] pub struct audit_ntp_val { pub oldval: i64, pub newval: i64 }
#[repr(C)] pub struct audit_ntp_data { pub vals: [audit_ntp_val; AUDIT_NTP_NVALS as usize] }

#[repr(C)] pub enum audit_nfcfgop { AUDIT_XT_OP_REGISTER, AUDIT_XT_OP_REPLACE,
    AUDIT_XT_OP_UNREGISTER, AUDIT_NFT_OP_TABLE_REGISTER, AUDIT_NFT_OP_TABLE_UNREGISTER,
    AUDIT_NFT_OP_CHAIN_REGISTER, AUDIT_NFT_OP_CHAIN_UNREGISTER, AUDIT_NFT_OP_RULE_REGISTER,
    AUDIT_NFT_OP_RULE_UNREGISTER, AUDIT_NFT_OP_SET_REGISTER, AUDIT_NFT_OP_SET_UNREGISTER,
    AUDIT_NFT_OP_SETELEM_REGISTER, AUDIT_NFT_OP_SETELEM_UNREGISTER, AUDIT_NFT_OP_GEN_REGISTER,
    AUDIT_NFT_OP_OBJ_REGISTER, AUDIT_NFT_OP_OBJ_UNREGISTER, AUDIT_NFT_OP_OBJ_RESET,
    AUDIT_NFT_OP_FLOWTABLE_REGISTER, AUDIT_NFT_OP_FLOWTABLE_UNREGISTER,
    AUDIT_NFT_OP_SETELEM_RESET, AUDIT_NFT_OP_RULE_RESET, AUDIT_NFT_OP_INVALID }

extern "C" { pub fn audit_register_class(class: c_int, list: *mut c_uint) -> c_int;
    pub fn audit_classify_syscall(abi: c_int, syscall: c_uint) -> c_int;
    pub fn audit_classify_arch(arch: c_int) -> c_int; }

pub const AUDIT_TYPE_UNKNOWN: u32 = 0; pub const AUDIT_TYPE_NORMAL: u32 = 1;
pub const AUDIT_TYPE_PARENT: u32 = 2; pub const AUDIT_TYPE_CHILD_DELETE: u32 = 3;
pub const AUDIT_TYPE_CHILD_CREATE: u32 = 4; pub const AUDITSC_ARGS: usize = 6;
pub const AUDIT_TTY_ENABLE: u32 = 1 << 0; pub const AUDIT_TTY_LOG_PASSWD: u32 = 1 << 1;
pub const AUDIT_CFG_LSM_SECCTX_SUBJECT: u32 = 1 << 0;
pub const AUDIT_CFG_LSM_SECCTX_OBJECT: u32 = 1 << 1;
pub const AUDIT_OFF: c_int = 0; pub const AUDIT_ON: c_int = 1; pub const AUDIT_LOCKED: c_int = 2;
pub const AUDIT_INODE_PARENT: u32 = 1; pub const AUDIT_INODE_HIDDEN: u32 = 2;
pub const AUDIT_INODE_NOEVAL: u32 = 4;

#[cfg(feature = "CONFIG_AUDITSYSCALL")]
extern "C" {
    pub fn audit_alloc(task: *mut task_struct) -> c_int; pub fn __audit_free(task: *mut task_struct);
    pub fn __audit_uring_entry(op: u8); pub fn __audit_uring_exit(success: c_int, code: c_long);
    pub fn __audit_syscall_entry(major: c_int, a0: c_ulong, a1: c_ulong, a2: c_ulong, a3: c_ulong);
    pub fn __audit_syscall_exit(ret_success: c_int, ret_value: c_long);
    pub fn __audit_getname(name: *mut filename); pub fn __audit_inode(name: *mut filename, dentry: *const dentry, flags: c_uint);
    pub fn __audit_file(file: *const file); pub fn __audit_inode_child(parent: *mut inode, dentry: *const dentry, type_: u8);
    pub fn audit_seccomp(syscall: c_ulong, signr: c_long, code: c_int);
    pub fn audit_seccomp_actions_logged(names: *const c_char, old_names: *const c_char, res: c_int);
    pub fn __audit_ptrace(t: *mut task_struct); pub fn audit_core_dumps(signr: c_long);
    pub fn __audit_ipc_obj(ipcp: *mut kern_ipc_perm); pub fn __audit_bprm(bprm: *mut linux_binprm);
    pub fn __audit_socketcall(nargs: c_int, args: *mut c_ulong) -> c_int;
    pub fn __audit_sockaddr(len: c_int, addr: *mut c_void) -> c_int;
}

#[inline] pub unsafe fn audit_socketcall_compat(nargs: c_int, args: *mut u32) -> c_int {
    let mut a = [0u64; AUDITSC_ARGS];
    if audit_dummy_context() { return 0; }
    for i in 0..nargs as usize { a[i] = *args.add(i) as u64; }
    __audit_socketcall(nargs, a.as_mut_ptr())
}
#[inline] pub unsafe fn audit_dummy_context() -> bool { true }
#[inline] pub unsafe fn audit_loginuid_set(tsk: *mut task_struct) -> bool {
    uid_valid(audit_get_loginuid(tsk))
}
extern "C" { pub fn audit_get_loginuid(tsk: *mut task_struct) -> kuid_t; pub fn uid_valid(uid: kuid_t) -> bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
