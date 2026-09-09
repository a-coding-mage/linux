/* SPDX-License-Identifier: GPL-2.0-only */
/* Glue to the underlying cluster stack. */

/* C headers omitted; the following names are supplied by the surrounding kernel bindings. */

pub const DLM_LKF_LOCAL: u32 = 0x0010_0000;
pub const GROUP_NAME_MAX: usize = 64;
pub const CLUSTER_NAME_MAX: usize = 16;

#[repr(C)]
pub struct ocfs2_protocol_version {
    pub pv_major: u8,
    pub pv_minor: u8,
}

#[repr(C)]
pub struct fsdlm_lksb_plus_lvb {
    pub lksb: dlm_lksb,
    pub lvb: [core::ffi::c_char; DLM_LVB_LEN as usize],
}

#[repr(C)]
pub union ocfs2_dlm_lksb_union {
    pub lksb_o2dlm: core::mem::ManuallyDrop<dlm_lockstatus>,
    pub lksb_fsdlm: core::mem::ManuallyDrop<dlm_lksb>,
    pub padding: core::mem::ManuallyDrop<fsdlm_lksb_plus_lvb>,
}

#[repr(C)]
pub struct ocfs2_dlm_lksb {
    pub lksb: ocfs2_dlm_lksb_union,
    pub lksb_conn: *mut ocfs2_cluster_connection,
}

#[repr(C)]
pub struct ocfs2_locking_protocol {
    pub lp_max_version: ocfs2_protocol_version,
    pub lp_lock_ast: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb)>,
    pub lp_blocking_ast: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb, i32)>,
    pub lp_unlock_ast: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb, i32)>,
}

#[repr(C)]
pub struct ocfs2_cluster_connection {
    pub cc_name: [core::ffi::c_char; GROUP_NAME_MAX + 1],
    pub cc_namelen: i32,
    pub cc_cluster_name: [core::ffi::c_char; CLUSTER_NAME_MAX + 1],
    pub cc_cluster_name_len: i32,
    pub cc_version: ocfs2_protocol_version,
    pub cc_proto: *mut ocfs2_locking_protocol,
    pub cc_recovery_handler: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void)>,
    pub cc_recovery_data: *mut core::ffi::c_void,
    pub cc_lockspace: *mut core::ffi::c_void,
    pub cc_private: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct ocfs2_stack_operations {
    pub connect: Option<unsafe extern "C" fn(*mut ocfs2_cluster_connection) -> i32>,
    pub disconnect: Option<unsafe extern "C" fn(*mut ocfs2_cluster_connection) -> i32>,
    pub this_node: Option<unsafe extern "C" fn(*mut ocfs2_cluster_connection, *mut u32) -> i32>,
    pub dlm_lock: Option<unsafe extern "C" fn(*mut ocfs2_cluster_connection, i32, *mut ocfs2_dlm_lksb, u32, *mut core::ffi::c_void, u32) -> i32>,
    pub dlm_unlock: Option<unsafe extern "C" fn(*mut ocfs2_cluster_connection, *mut ocfs2_dlm_lksb, u32) -> i32>,
    pub lock_status: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb) -> i32>,
    pub lvb_valid: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb) -> i32>,
    pub lock_lvb: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb) -> *mut core::ffi::c_void>,
    pub plock: Option<unsafe extern "C" fn(*mut ocfs2_cluster_connection, u64, *mut file, i32, *mut file_lock) -> i32>,
    pub dump_lksb: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb)>,
}

#[repr(C)]
pub struct ocfs2_stack_plugin {
    pub sp_name: *mut core::ffi::c_char,
    pub sp_ops: *const ocfs2_stack_operations,
    pub sp_owner: *mut module,
    pub sp_list: list_head,
    pub sp_count: u32,
    pub sp_max_proto: ocfs2_protocol_version,
}

extern "C" {
    pub fn ocfs2_cluster_connect(stack_name: *const core::ffi::c_char, cluster_name: *const core::ffi::c_char, cluster_name_len: i32, group: *const core::ffi::c_char, grouplen: i32, lproto: *mut ocfs2_locking_protocol, recovery_handler: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void)>, recovery_data: *mut core::ffi::c_void, conn: *mut *mut ocfs2_cluster_connection) -> i32;
    pub fn ocfs2_cluster_connect_agnostic(group: *const core::ffi::c_char, grouplen: i32, lproto: *mut ocfs2_locking_protocol, recovery_handler: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void)>, recovery_data: *mut core::ffi::c_void, conn: *mut *mut ocfs2_cluster_connection) -> i32;
    pub fn ocfs2_cluster_disconnect(conn: *mut ocfs2_cluster_connection, hangup_pending: i32) -> i32;
    pub fn ocfs2_cluster_hangup(group: *const core::ffi::c_char, grouplen: i32);
    pub fn ocfs2_cluster_this_node(conn: *mut ocfs2_cluster_connection, node: *mut u32) -> i32;
    pub fn ocfs2_dlm_lock(conn: *mut ocfs2_cluster_connection, mode: i32, lksb: *mut ocfs2_dlm_lksb, flags: u32, name: *mut core::ffi::c_void, namelen: u32) -> i32;
    pub fn ocfs2_dlm_unlock(conn: *mut ocfs2_cluster_connection, lksb: *mut ocfs2_dlm_lksb, flags: u32) -> i32;
    pub fn ocfs2_dlm_lock_status(lksb: *mut ocfs2_dlm_lksb) -> i32;
    pub fn ocfs2_dlm_lvb_valid(lksb: *mut ocfs2_dlm_lksb) -> i32;
    pub fn ocfs2_dlm_lvb(lksb: *mut ocfs2_dlm_lksb) -> *mut core::ffi::c_void;
    pub fn ocfs2_dlm_dump_lksb(lksb: *mut ocfs2_dlm_lksb);
    pub fn ocfs2_stack_supports_plocks() -> i32;
    pub fn ocfs2_plock(conn: *mut ocfs2_cluster_connection, ino: u64, file: *mut file, cmd: i32, fl: *mut file_lock) -> i32;
    pub fn ocfs2_stack_glue_set_max_proto_version(max_proto: *mut ocfs2_protocol_version);
    pub fn ocfs2_stack_glue_register(plugin: *mut ocfs2_stack_plugin) -> i32;
    pub fn ocfs2_stack_glue_unregister(plugin: *mut ocfs2_stack_plugin);
    pub static mut ocfs2_kset: *mut kset;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
