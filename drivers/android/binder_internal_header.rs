/* SPDX-License-Identifier: GPL-2.0 */

// Translated from binder_internal.h. Kernel dependencies are supplied by other files.

#[repr(C)]
pub struct binder_context {
    pub binder_context_mgr_node: *mut binder_node,
    pub context_mgr_node_lock: mutex,
    pub binder_context_mgr_uid: kuid_t,
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct binder_device {
    pub hlist: hlist_node,
    pub miscdev: miscdevice,
    pub context: binder_context,
    pub binderfs_inode: *mut inode,
    pub ref_: refcount_t,
}

#[repr(C)]
pub struct binderfs_mount_opts { pub max: core::ffi::c_int, pub stats_mode: core::ffi::c_int }

#[repr(C)]
pub struct binderfs_info {
    pub ipc_ns: *mut ipc_namespace,
    pub control_dentry: *mut dentry,
    pub root_uid: kuid_t,
    pub root_gid: kgid_t,
    pub mount_opts: binderfs_mount_opts,
    pub device_count: core::ffi::c_int,
    pub proc_log_dir: *mut dentry,
}

extern "C" {
    pub static binder_fops: file_operations;
    pub static mut binder_devices_param: *mut core::ffi::c_char;
    pub static binder_debugfs_entries: [binder_debugfs_entry; 0];
    pub fn binder_add_device(device: *mut binder_device);
    pub fn binder_remove_device(device: *mut binder_device);
}

#[cfg(CONFIG_ANDROID_BINDERFS)]
extern "C" {
    pub fn is_binderfs_device(inode: *const inode) -> bool;
    pub fn binderfs_create_file(dir: *mut dentry, name: *const core::ffi::c_char,
        fops: *const file_operations, data: *mut core::ffi::c_void) -> *mut dentry;
    pub fn init_binderfs() -> core::ffi::c_int;
}
#[cfg(not(CONFIG_ANDROID_BINDERFS))]
pub unsafe fn is_binderfs_device(_inode: *const inode) -> bool { false }
#[cfg(not(CONFIG_ANDROID_BINDERFS))]
pub unsafe fn binderfs_create_file(_dir: *mut dentry, _name: *const core::ffi::c_char,
    _fops: *const file_operations, _data: *mut core::ffi::c_void) -> *mut dentry { core::ptr::null_mut() }
#[cfg(not(CONFIG_ANDROID_BINDERFS))]
pub unsafe fn init_binderfs() -> core::ffi::c_int { 0 }

#[repr(C)]
pub struct binder_debugfs_entry {
    pub name: *const core::ffi::c_char,
    pub mode: umode_t,
    pub fops: *const file_operations,
    pub data: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum binder_stat_types { BINDER_STAT_PROC, BINDER_STAT_THREAD, BINDER_STAT_NODE, BINDER_STAT_REF, BINDER_STAT_DEATH, BINDER_STAT_TRANSACTION, BINDER_STAT_TRANSACTION_COMPLETE, BINDER_STAT_FREEZE, BINDER_STAT_COUNT }

#[repr(C)]
pub struct binder_stats {
    pub br: [atomic_t; 0], // _IOC_NR(BR_CLEAR_FREEZE_NOTIFICATION_DONE) + 1
    pub bc: [atomic_t; 0], // _IOC_NR(BC_FREEZE_NOTIFICATION_DONE) + 1
    pub obj_created: [atomic_t; binder_stat_types::BINDER_STAT_COUNT as usize],
    pub obj_deleted: [atomic_t; binder_stat_types::BINDER_STAT_COUNT as usize],
}

#[repr(C)]
pub struct binder_work {
    pub entry: list_head,
    pub type_: binder_work_type,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub enum binder_work_type { BINDER_WORK_TRANSACTION = 1, BINDER_WORK_TRANSACTION_COMPLETE, BINDER_WORK_TRANSACTION_PENDING, BINDER_WORK_TRANSACTION_ONEWAY_SPAM_SUSPECT, BINDER_WORK_RETURN_ERROR, BINDER_WORK_NODE, BINDER_WORK_DEAD_BINDER, BINDER_WORK_DEAD_BINDER_AND_CLEAR, BINDER_WORK_CLEAR_DEATH_NOTIFICATION, BINDER_WORK_FROZEN_BINDER, BINDER_WORK_CLEAR_FREEZE_NOTIFICATION }

#[repr(C)] pub struct binder_error { pub work: binder_work, pub cmd: u32 }

#[repr(C)]
pub union binder_node_rb_or_dead { pub rb_node: rb_node, pub dead_node: hlist_node }
#[repr(C)]
pub struct binder_node {
    pub debug_id: core::ffi::c_int, pub lock: spinlock_t, pub work: binder_work,
    pub rb_or_dead: binder_node_rb_or_dead, pub proc: *mut binder_proc, pub refs: hlist_head,
    pub internal_strong_refs: core::ffi::c_int, pub local_weak_refs: core::ffi::c_int,
    pub local_strong_refs: core::ffi::c_int, pub tmp_refs: core::ffi::c_int,
    pub ptr: binder_uintptr_t, pub cookie: binder_uintptr_t,
    pub has_strong_ref: u8, pub pending_strong_ref: u8, pub has_weak_ref: u8, pub pending_weak_ref: u8,
    pub accept_fds: u8, pub txn_security_ctx: u8, pub min_priority: u8,
    pub has_async_transaction: bool, pub async_todo: list_head,
}

#[repr(C)] pub struct binder_ref_death { pub work: binder_work, pub cookie: binder_uintptr_t }
#[repr(C)] pub struct binder_ref_freeze { pub work: binder_work, pub cookie: binder_uintptr_t, pub is_frozen: u8, pub sent: u8, pub resend: u8 }
#[repr(C)] pub struct binder_ref_data { pub debug_id: core::ffi::c_int, pub desc: u32, pub strong: core::ffi::c_int, pub weak: core::ffi::c_int }
#[repr(C)] pub struct binder_ref { pub data: binder_ref_data, pub rb_node_desc: rb_node, pub rb_node_node: rb_node, pub node_entry: hlist_node, pub proc: *mut binder_proc, pub node: *mut binder_node, pub death: *mut binder_ref_death, pub freeze: *mut binder_ref_freeze }

#[repr(C)]
pub struct binder_proc {
    pub proc_node: hlist_node, pub threads: rb_root, pub nodes: rb_root, pub refs_by_desc: rb_root, pub refs_by_node: rb_root,
    pub waiting_threads: list_head, pub pid: core::ffi::c_int, pub tsk: *mut task_struct, pub cred: *const cred,
    pub deferred_work_node: hlist_node, pub deferred_work: core::ffi::c_int, pub outstanding_txns: core::ffi::c_int,
    pub is_dead: bool, pub is_frozen: bool, pub sync_recv: bool, pub async_recv: bool, pub freeze_wait: wait_queue_head_t,
    pub dmap: dbitmap, pub todo: list_head, pub stats: binder_stats, pub delivered_death: list_head, pub delivered_freeze: list_head,
    pub max_threads: u32, pub requested_threads: core::ffi::c_int, pub requested_threads_started: core::ffi::c_int,
    pub tmp_ref: core::ffi::c_int, pub default_priority: core::ffi::c_long, pub debugfs_entry: *mut dentry,
    pub alloc: binder_alloc, pub context: *mut binder_context, pub inner_lock: spinlock_t, pub outer_lock: spinlock_t,
    pub binderfs_entry: *mut dentry, pub oneway_spam_detection_enabled: bool,
}

#[repr(C)] pub struct binder_thread { pub proc: *mut binder_proc, pub rb_node: rb_node, pub waiting_thread_node: list_head, pub pid: core::ffi::c_int, pub looper: core::ffi::c_int, pub looper_need_return: bool, pub transaction_stack: *mut binder_transaction, pub todo: list_head, pub process_todo: bool, pub return_error: binder_error, pub reply_error: binder_error, pub ee: binder_extended_error, pub wait: wait_queue_head_t, pub stats: binder_stats, pub tmp_ref: atomic_t, pub is_dead: bool }
#[repr(C)] pub struct binder_txn_fd_fixup { pub fixup_entry: list_head, pub file: *mut file, pub offset: usize, pub target_fd: core::ffi::c_int }
#[repr(C)] pub struct binder_transaction { pub debug_id: core::ffi::c_int, pub work: binder_work, pub from: *mut binder_thread, pub from_pid: pid_t, pub from_tid: pid_t, pub from_parent: *mut binder_transaction, pub to_proc: *mut binder_proc, pub to_thread: *mut binder_thread, pub to_parent: *mut binder_transaction, pub is_async: u8, pub is_reply: u8, pub buffer: *mut binder_buffer, pub code: u32, pub flags: u32, pub priority: core::ffi::c_long, pub saved_priority: core::ffi::c_long, pub sender_euid: kuid_t, pub start_time: ktime_t, pub fd_fixups: list_head, pub security_ctx: binder_uintptr_t, pub lock: spinlock_t }
#[repr(C)] pub union binder_object { pub hdr: binder_object_header, pub fbo: flat_binder_object, pub fdo: binder_fd_object, pub bbo: binder_buffer_object, pub fdao: binder_fd_array_object }

// CONFIG_KUNIT declaration: vm_fault_t binder_vm_fault(struct vm_fault *vmf);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
