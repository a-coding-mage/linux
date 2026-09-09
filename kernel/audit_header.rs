/* SPDX-License-Identifier: GPL-2.0-or-later */
/* audit -- definition of audit_context structure and supporting types */

/* C header dependencies are supplied by other translation units. */

pub const AUDIT_NAMES: usize = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum audit_state {
    AUDIT_STATE_DISABLED,
    AUDIT_STATE_BUILD,
    AUDIT_STATE_RECORD,
}

pub enum audit_watch {}
pub enum audit_fsnotify_mark {}
pub enum audit_tree {}
pub enum audit_chunk {}

#[repr(C)]
pub struct audit_entry {
    pub list: list_head,
    pub rcu: rcu_head,
    pub rule: audit_krule,
}

#[repr(C)]
pub union audit_cap_data_effective {
    pub fE: ::std::os::raw::c_uint,
    pub effective: kernel_cap_t,
}

#[repr(C)]
pub struct audit_cap_data {
    pub permitted: kernel_cap_t,
    pub inheritable: kernel_cap_t,
    pub _effective: audit_cap_data_effective,
    pub ambient: kernel_cap_t,
    pub rootid: kuid_t,
}

#[repr(C)]
pub struct audit_names {
    pub list: list_head,
    pub name: *mut filename,
    pub name_len: ::std::os::raw::c_int,
    pub hidden: bool,
    pub ino: u64,
    pub dev: dev_t,
    pub mode: umode_t,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub rdev: dev_t,
    pub oprop: lsm_prop,
    pub fcap: audit_cap_data,
    pub fcap_ver: ::std::os::raw::c_uint,
    pub type_: u8,
    pub should_free: bool,
}

#[repr(C)]
pub struct audit_proctitle {
    pub len: ::std::os::raw::c_int,
    pub value: *mut ::std::os::raw::c_char,
}

#[repr(C)]
pub struct audit_stamp {
    pub ctime: timespec64,
    pub serial: ::std::os::raw::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum audit_context_context {
    AUDIT_CTX_UNUSED,
    AUDIT_CTX_SYSCALL,
    AUDIT_CTX_URING,
}

#[repr(C)]
pub union audit_context_data {
    pub socketcall: audit_context_socketcall,
    pub ipc: audit_context_ipc,
    pub mq_getsetattr: audit_context_mq_getsetattr,
    pub mq_notify: audit_context_mq_notify,
    pub mq_sendrecv: audit_context_mq_sendrecv,
    pub mq_open: audit_context_mq_open,
    pub capset: audit_context_capset,
    pub mmap: audit_context_mmap,
    pub openat2: open_how,
    pub execve: audit_context_execve,
    pub module: audit_context_module,
    pub time: audit_context_time,
}

#[repr(C)] pub struct audit_context_socketcall { pub nargs: ::std::os::raw::c_int, pub args: [::std::os::raw::c_long; 6] }
#[repr(C)] pub struct audit_context_ipc { pub uid: kuid_t, pub gid: kgid_t, pub mode: umode_t, pub oprop: lsm_prop, pub has_perm: ::std::os::raw::c_int, pub perm_uid: uid_t, pub perm_gid: gid_t, pub perm_mode: umode_t, pub qbytes: ::std::os::raw::c_ulong }
#[repr(C)] pub struct audit_context_mq_getsetattr { pub mqdes: mqd_t, pub mqstat: mq_attr }
#[repr(C)] pub struct audit_context_mq_notify { pub mqdes: mqd_t, pub sigev_signo: ::std::os::raw::c_int }
#[repr(C)] pub struct audit_context_mq_sendrecv { pub mqdes: mqd_t, pub msg_len: usize, pub msg_prio: ::std::os::raw::c_uint, pub abs_timeout: timespec64 }
#[repr(C)] pub struct audit_context_mq_open { pub oflag: ::std::os::raw::c_int, pub mode: umode_t, pub attr: mq_attr }
#[repr(C)] pub struct audit_context_capset { pub pid: pid_t, pub cap: audit_cap_data }
#[repr(C)] pub struct audit_context_mmap { pub fd: ::std::os::raw::c_int, pub flags: ::std::os::raw::c_int }
#[repr(C)] pub struct audit_context_execve { pub argc: ::std::os::raw::c_int }
#[repr(C)] pub struct audit_context_module { pub name: *const ::std::os::raw::c_char }
#[repr(C)] pub struct audit_context_time { pub ntp_data: audit_ntp_data, pub tk_injoffset: timespec64 }

#[repr(C)]
pub struct audit_context {
    pub dummy: ::std::os::raw::c_int,
    pub context: audit_context_context,
    pub state: audit_state,
    pub current_state: audit_state,
    pub stamp: audit_stamp,
    pub major: ::std::os::raw::c_int,
    pub uring_op: ::std::os::raw::c_int,
    pub argv: [::std::os::raw::c_ulong; 4],
    pub return_code: ::std::os::raw::c_long,
    pub prio: u64,
    pub return_valid: ::std::os::raw::c_int,
    pub preallocated_names: [audit_names; AUDIT_NAMES],
    pub name_count: ::std::os::raw::c_int,
    pub names_list: list_head,
    pub filterkey: *mut ::std::os::raw::c_char,
    pub pwd: path,
    pub aux: *mut audit_aux_data,
    pub aux_pids: *mut audit_aux_data,
    pub sockaddr: *mut sockaddr_storage,
    pub sockaddr_len: usize,
    pub ppid: pid_t,
    pub uid: kuid_t, pub euid: kuid_t, pub suid: kuid_t, pub fsuid: kuid_t,
    pub gid: kgid_t, pub egid: kgid_t, pub sgid: kgid_t, pub fsgid: kgid_t,
    pub personality: ::std::os::raw::c_ulong,
    pub arch: ::std::os::raw::c_int,
    pub target_pid: pid_t,
    pub target_auid: kuid_t,
    pub target_uid: kuid_t,
    pub target_sessionid: ::std::os::raw::c_uint,
    pub target_ref: lsm_prop,
    pub target_comm: [::std::os::raw::c_char; TASK_COMM_LEN],
    pub trees: *mut audit_tree_refs,
    pub first_trees: *mut audit_tree_refs,
    pub killed_trees: list_head,
    pub tree_count: ::std::os::raw::c_int,
    pub type_: ::std::os::raw::c_int,
    pub data: audit_context_data,
    pub fds: [::std::os::raw::c_int; 2],
    pub proctitle: audit_proctitle,
}

pub const AUDIT_INODE_BUCKETS: usize = 32;
pub const AUDIT_NAME_FULL: i32 = -1;

#[repr(C)]
pub struct audit_netlink_list { pub portid: u32, pub net: *mut net, pub q: sk_buff_head }
#[repr(C)]
pub struct audit_watch_ctx { pub dir: *mut inode, pub child: *mut inode }

extern "C" {
    pub static mut audit_ever_enabled: bool;
    pub static mut audit_inode_hash: [list_head; AUDIT_INODE_BUCKETS];
    pub fn audit_serial() -> u32;
    pub fn audit_del_rule(entry: *mut audit_entry) -> ::std::os::raw::c_int;
    pub fn audit_free_rule_rcu(head: *mut rcu_head);
    pub fn audit_dupe_rule(old: *mut audit_krule, ctx: *mut audit_watch_ctx) -> *mut audit_entry;
    pub fn audit_log_d_path_exe(ab: *mut audit_buffer, mm: *mut mm_struct);
    pub fn audit_get_tty() -> *mut tty_struct;
    pub fn audit_put_tty(tty: *mut tty_struct);
    pub fn audit_put_watch(watch: *mut audit_watch);
    pub fn audit_get_watch(watch: *mut audit_watch);
    pub fn audit_to_watch(krule: *mut audit_krule, path: *mut ::std::os::raw::c_char, len: ::std::os::raw::c_int, op: u32) -> ::std::os::raw::c_int;
    pub fn audit_add_watch(krule: *mut audit_krule, list: *mut *mut list_head) -> ::std::os::raw::c_int;
    pub fn audit_remove_watch_rule(krule: *mut audit_krule);
    pub fn audit_watch_path(watch: *mut audit_watch) -> *mut ::std::os::raw::c_char;
    pub fn audit_watch_compare(watch: *mut audit_watch, ino: u64, dev: dev_t) -> ::std::os::raw::c_int;
    pub fn audit_alloc_mark(krule: *mut audit_krule, pathname: *mut ::std::os::raw::c_char, len: ::std::os::raw::c_int, ctx: *mut audit_watch_ctx) -> *mut audit_fsnotify_mark;
    pub fn audit_mark_path(mark: *mut audit_fsnotify_mark) -> *mut ::std::os::raw::c_char;
    pub fn audit_remove_mark(mark: *mut audit_fsnotify_mark);
    pub fn audit_remove_mark_rule(krule: *mut audit_krule);
    pub fn audit_mark_compare(mark: *mut audit_fsnotify_mark, ino: u64, dev: dev_t) -> ::std::os::raw::c_int;
    pub fn audit_dupe_exe(new_: *mut audit_krule, old: *mut audit_krule, ctx: *mut audit_watch_ctx) -> ::std::os::raw::c_int;
    pub fn audit_exe_compare(tsk: *mut task_struct, mark: *mut audit_fsnotify_mark) -> ::std::os::raw::c_int;
    pub fn audit_tree_lookup(inode: *const inode) -> *mut audit_chunk;
    pub fn audit_put_chunk(chunk: *mut audit_chunk);
    pub fn audit_tree_match(chunk: *mut audit_chunk, tree: *mut audit_tree) -> bool;
    pub fn audit_make_tree(rule: *mut audit_krule, pathname: *mut ::std::os::raw::c_char, op: u32) -> ::std::os::raw::c_int;
    pub fn audit_add_tree_rule(rule: *mut audit_krule) -> ::std::os::raw::c_int;
    pub fn audit_remove_tree_rule(rule: *mut audit_krule) -> ::std::os::raw::c_int;
    pub fn audit_trim_trees();
    pub fn audit_tag_tree(old: *mut ::std::os::raw::c_char, new_: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn audit_tree_path(tree: *mut audit_tree) -> *const ::std::os::raw::c_char;
    pub fn audit_put_tree(tree: *mut audit_tree);
    pub fn audit_kill_trees(context: *mut audit_context);
    pub fn audit_signal_info_syscall(t: *mut task_struct) -> ::std::os::raw::c_int;
    pub fn audit_filter_inodes(tsk: *mut task_struct, ctx: *mut audit_context);
    pub fn audit_killed_trees() -> *mut list_head;
    pub fn auditsc_get_stamp(ctx: *mut audit_context, stamp: *mut audit_stamp) -> ::std::os::raw::c_int;
}

#[inline]
pub fn audit_hash_ino(ino: u64) -> u32 { (ino as u32) & ((AUDIT_INODE_BUCKETS - 1) as u32) }

extern "C" {
    pub fn audit_log_session_info(ab: *mut audit_buffer);
    pub fn auditd_test_task(task: *mut task_struct) -> ::std::os::raw::c_int;
    pub fn audit_match_class(class: ::std::os::raw::c_int, syscall: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn audit_comparator(left: u32, op: u32, right: u32) -> ::std::os::raw::c_int;
    pub fn audit_uid_comparator(left: kuid_t, op: u32, right: kuid_t) -> ::std::os::raw::c_int;
    pub fn audit_gid_comparator(left: kgid_t, op: u32, right: kgid_t) -> ::std::os::raw::c_int;
    pub fn parent_len(path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn audit_compare_dname_path(dname: *const qstr, path: *const ::std::os::raw::c_char, plen: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn audit_make_reply(seq: ::std::os::raw::c_int, type_: ::std::os::raw::c_int, done: ::std::os::raw::c_int, multi: ::std::os::raw::c_int, payload: *const ::std::os::raw::c_void, size: ::std::os::raw::c_int) -> *mut sk_buff;
    pub fn audit_panic(message: *const ::std::os::raw::c_char);
    pub fn audit_send_list_thread(dest: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn audit_unpack_string(bufp: *mut *mut ::std::os::raw::c_void, remain: *mut usize, len: usize) -> *mut ::std::os::raw::c_char;
    pub fn audit_filter(msgtype: ::std::os::raw::c_int, listtype: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn audit_ctl_lock();
    pub fn audit_ctl_unlock();
}

/* CONFIG_AUDITSYSCALL-dependent declarations and fallback macros are supplied
 * by the corresponding kernel configuration; they are intentionally retained
 * as an external dependency rather than implemented here. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
