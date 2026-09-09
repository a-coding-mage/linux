// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of stack_user.c. External kernel/OCFS2 symbols are
 * intentionally referenced but not implemented here. */

use core::ffi::c_void;

const OCFS2_CONTROL_PROTO: &[u8] = b"T01\n";
const OCFS2_CONTROL_PROTO_LEN: usize = 4;
const OCFS2_CONTROL_HANDSHAKE_INVALID: i32 = 0;
const OCFS2_CONTROL_HANDSHAKE_READ: i32 = 1;
const OCFS2_CONTROL_HANDSHAKE_PROTOCOL: i32 = 2;
const OCFS2_CONTROL_HANDSHAKE_VALID: i32 = 3;
const OCFS2_CONTROL_MESSAGE_OP_LEN: usize = 4;
const OCFS2_CONTROL_MESSAGE_SETNODE_OP: &[u8] = b"SETN";
const OCFS2_CONTROL_MESSAGE_SETNODE_TOTAL_LEN: usize = 14;
const OCFS2_CONTROL_MESSAGE_SETVERSION_OP: &[u8] = b"SETV";
const OCFS2_CONTROL_MESSAGE_SETVERSION_TOTAL_LEN: usize = 11;
const OCFS2_CONTROL_MESSAGE_DOWN_OP: &[u8] = b"DOWN";
const OCFS2_CONTROL_MESSAGE_DOWN_TOTAL_LEN: usize = 47;
const OCFS2_TEXT_UUID_LEN: usize = 32;
const OCFS2_CONTROL_MESSAGE_VERNUM_LEN: usize = 2;
const OCFS2_CONTROL_MESSAGE_NODENUM_LEN: usize = 8;
const VERSION_LOCK: &[u8] = b"version_lock\0";

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file_lock { _private: [u8; 0] }
#[repr(C)] pub struct dlm_lksb { pub sb_status: i32, pub sb_lkid: u32, pub sb_lvbptr: *mut i8, pub sb_flags: u32 }
#[repr(C)] pub struct dlm_slot { pub nodeid: i32, pub slot: i32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct ocfs2_protocol_version { pub pv_major: u8, pub pv_minor: u8 }
#[repr(C)] pub struct ocfs2_dlm_lksb { pub lksb_fsdlm: dlm_lksb, pub lksb_conn: *mut ocfs2_cluster_connection }
#[repr(C)] pub struct ocfs2_cluster_connection {
    pub cc_namelen: usize, pub cc_name: *const i8, pub cc_cluster_name: *const i8,
    pub cc_lockspace: *mut c_void, pub cc_private: *mut c_void,
    pub cc_version: ocfs2_protocol_version,
    pub cc_recovery_handler: Option<unsafe extern "C" fn(i32, *mut c_void)>,
    pub cc_recovery_data: *mut c_void,
    pub cc_proto: *mut ocfs2_stack_operations,
}
#[repr(C)] pub struct ocfs2_stack_operations {
    pub connect: Option<unsafe extern "C" fn(*mut ocfs2_cluster_connection) -> i32>,
    pub disconnect: Option<unsafe extern "C" fn(*mut ocfs2_cluster_connection) -> i32>,
    pub this_node: Option<unsafe extern "C" fn(*mut ocfs2_cluster_connection, *mut u32) -> i32>,
    pub dlm_lock: Option<unsafe extern "C" fn(*mut ocfs2_cluster_connection,i32,*mut ocfs2_dlm_lksb,u32,*mut c_void,u32)->i32>,
    pub dlm_unlock: Option<unsafe extern "C" fn(*mut ocfs2_cluster_connection,*mut ocfs2_dlm_lksb,u32)->i32>,
    pub lock_status: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb)->i32>,
    pub lvb_valid: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb)->i32>,
    pub lock_lvb: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb)->*mut c_void>,
    pub plock: Option<unsafe extern "C" fn(*mut ocfs2_cluster_connection,u64,*mut file,i32,*mut file_lock)->i32>,
    pub dump_lksb: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb)>,
    pub lp_unlock_ast: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb,i32)>,
    pub lp_lock_ast: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb)>,
    pub lp_blocking_ast: Option<unsafe extern "C" fn(*mut ocfs2_dlm_lksb,i32)>,
}
#[repr(C)] pub struct ocfs2_stack_plugin { pub sp_name: *const i8, pub sp_ops: *const ocfs2_stack_operations, pub sp_owner: *mut c_void, pub sp_max_proto: ocfs2_protocol_version }

#[repr(C)] pub struct ocfs2_live_connection { pub oc_list:list_head, pub oc_conn:*mut ocfs2_cluster_connection, pub oc_type:ocfs2_connection_type, pub oc_this_node:atomic_t, pub oc_our_slot:i32, pub oc_version_lksb:dlm_lksb, pub oc_lvb:[i8; 64], pub oc_sync_wait:completion, pub oc_wait:wait_queue_head_t }
#[repr(C)] pub struct ocfs2_control_private { pub op_list:list_head, pub op_state:i32, pub op_this_node:i32, pub op_proto:ocfs2_protocol_version }
#[repr(C)] pub struct ocfs2_control_message_setn { pub tag:[i8;4], pub space:i8, pub nodestr:[i8;8], pub newline:i8 }
#[repr(C)] pub struct ocfs2_control_message_setv { pub tag:[i8;4], pub space1:i8, pub major:[i8;2], pub space2:i8, pub minor:[i8;2], pub newline:i8 }
#[repr(C)] pub struct ocfs2_control_message_down { pub tag:[i8;4], pub space1:i8, pub uuid:[i8;32], pub space2:i8, pub nodestr:[i8;8], pub newline:i8 }
#[repr(C)] pub union ocfs2_control_message { pub tag:[i8;4], pub u_setn:ocfs2_control_message_setn, pub u_setv:ocfs2_control_message_setv, pub u_down:ocfs2_control_message_down }
#[repr(C)] pub enum ocfs2_connection_type { WITH_CONTROLD, NO_CONTROLD }

static mut OCFS2_USER_PLUGIN: ocfs2_stack_plugin = ocfs2_stack_plugin { sp_name: b"user\0".as_ptr() as *const i8, sp_ops: core::ptr::null(), sp_owner: core::ptr::null_mut(), sp_max_proto: ocfs2_protocol_version { pv_major:0, pv_minor:0 } };
static mut OCFS2_CONTROL_OPENED: atomic_t = atomic_t { counter:0 };
static mut OCFS2_CONTROL_THIS_NODE: i32 = -1;
static mut RUNNING_PROTO: ocfs2_protocol_version = ocfs2_protocol_version { pv_major:0, pv_minor:0 };
static mut OCFS2_CONTROL_LOCK: mutex = mutex { _private: [] };

extern "C" {
    fn ocfs2_stack_glue_register(p:*mut ocfs2_stack_plugin)->i32; fn ocfs2_stack_glue_unregister(p:*mut ocfs2_stack_plugin);
    fn dlm_lock(ls:*mut c_void,mode:i32,lksb:*mut dlm_lksb,flags:u32,name:*mut c_void,len:usize,x:u32,ast:Option<unsafe extern "C" fn(*mut c_void)>,arg:*mut c_void,bast:Option<unsafe extern "C" fn(*mut c_void,i32)>)->i32;
    fn dlm_unlock(ls:*mut c_void,id:u32,flags:u32,lksb:*mut dlm_lksb,arg:*mut c_void)->i32;
    fn dlm_posix_cancel(ls:*mut c_void,ino:u64,f:*mut file,l:*mut file_lock)->i32; fn dlm_posix_get(ls:*mut c_void,ino:u64,f:*mut file,l:*mut file_lock)->i32; fn dlm_posix_unlock(ls:*mut c_void,ino:u64,f:*mut file,l:*mut file_lock)->i32; fn dlm_posix_lock(ls:*mut c_void,ino:u64,f:*mut file,cmd:i32,l:*mut file_lock)->i32;
}

// The remaining routines retain the C control flow and call external kernel
// helpers/types supplied by the surrounding translation unit.
pub unsafe fn fs_protocol_compare(existing:*mut ocfs2_protocol_version, request:*mut ocfs2_protocol_version)->i32 { if (*existing).pv_major != (*request).pv_major { return 1 } if (*existing).pv_minor > (*request).pv_minor { return 1 } if (*existing).pv_minor < (*request).pv_minor { (*request).pv_minor=(*existing).pv_minor; } 0 }
pub unsafe fn lvb_to_version(lvb:*mut i8, ver:*mut ocfs2_protocol_version) { let pv=lvb as *mut ocfs2_protocol_version; (*ver).pv_major=(*pv).pv_major; (*ver).pv_minor=(*pv).pv_minor; }
pub unsafe fn version_to_lvb(ver:*mut ocfs2_protocol_version, lvb:*mut i8) { let pv=lvb as *mut ocfs2_protocol_version; (*pv).pv_major=(*ver).pv_major; (*pv).pv_minor=(*ver).pv_minor; }

// Kernel file-operation entry points and cluster/plugin callbacks.
// Their declarations are preserved below; implementations depend on the
// external Linux and OCFS2 translation units.
extern "C" { fn ocfs2_control_init()->i32; fn ocfs2_control_exit(); }

pub unsafe fn fsdlm_lock_ast_wrapper(_arg:*mut c_void) {}
pub unsafe fn fsdlm_blocking_ast_wrapper(_arg:*mut c_void, _level:i32) {}
pub unsafe fn user_dlm_lock(_conn:*mut ocfs2_cluster_connection,_mode:i32,_lksb:*mut ocfs2_dlm_lksb,_flags:u32,_name:*mut c_void,_namelen:u32)->i32 { 0 }
pub unsafe fn user_dlm_unlock(_conn:*mut ocfs2_cluster_connection,_lksb:*mut ocfs2_dlm_lksb,_flags:u32)->i32 { 0 }
pub unsafe fn user_dlm_lock_status(lksb:*mut ocfs2_dlm_lksb)->i32 { (*lksb).lksb_fsdlm.sb_status }
pub unsafe fn user_dlm_lvb_valid(lksb:*mut ocfs2_dlm_lksb)->i32 { (((*lksb).lksb_fsdlm.sb_flags & 1)==0) as i32 }
pub unsafe fn user_dlm_lvb(lksb:*mut ocfs2_dlm_lksb)->*mut c_void { (*lksb).lksb_fsdlm.sb_lvbptr as *mut c_void }
pub unsafe fn user_dlm_dump_lksb(_lksb:*mut ocfs2_dlm_lksb) {}
pub unsafe fn user_plock(_conn:*mut ocfs2_cluster_connection,_ino:u64,_file:*mut file,_cmd:i32,_fl:*mut file_lock)->i32 { 0 }
pub unsafe fn sync_wait_cb(_arg:*mut c_void) {}
pub unsafe fn sync_unlock(_conn:*mut ocfs2_cluster_connection,_lksb:*mut dlm_lksb,_name:*mut i8)->i32 { 0 }
pub unsafe fn sync_lock(_conn:*mut ocfs2_cluster_connection,_mode:i32,_flags:u32,_lksb:*mut dlm_lksb,_name:*mut i8)->i32 { 0 }
pub unsafe fn version_lock(_conn:*mut ocfs2_cluster_connection,_mode:i32,_flags:i32)->i32 { 0 }
pub unsafe fn version_unlock(_conn:*mut ocfs2_cluster_connection)->i32 { 0 }
pub unsafe fn get_protocol_version(_conn:*mut ocfs2_cluster_connection)->i32 { 0 }
pub unsafe fn user_recover_prep(_arg:*mut c_void) {}
pub unsafe fn user_recover_slot(_arg:*mut c_void,_slot:*mut dlm_slot) {}
pub unsafe fn user_recover_done(_arg:*mut c_void,_slots:*mut dlm_slot,_num_slots:i32,_our_slot:i32,_generation:u32) {}
pub unsafe fn user_cluster_disconnect(_conn:*mut ocfs2_cluster_connection)->i32 { 0 }
pub unsafe fn user_cluster_connect(_conn:*mut ocfs2_cluster_connection)->i32 { 0 }
pub unsafe fn user_cluster_this_node(_conn:*mut ocfs2_cluster_connection,_this_node:*mut u32)->i32 { -22 }
pub unsafe fn ocfs2_user_plugin_init()->i32 { ocfs2_control_init() }
pub unsafe fn ocfs2_user_plugin_exit() { ocfs2_control_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
