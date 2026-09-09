/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of dlm_internal.h. Included kernel dependencies are
 * intentionally left as external Rust types supplied by other translations. */

use core::mem::ManuallyDrop;
pub type c_ulong = usize;

/* C forward declarations are represented by external types from dependencies. */
pub const DLM_LKSTS_WAITING: i32 = 1;
pub const DLM_LKSTS_GRANTED: i32 = 2;
pub const DLM_LKSTS_CONVERT: i32 = 3;
pub const DLM_IFL_MSTCPY_BIT: u32 = 16;
pub const __DLM_IFL_MIN_BIT: u32 = DLM_IFL_MSTCPY_BIT;
pub const DLM_IFL_RESEND_BIT: u32 = 17;
pub const DLM_IFL_DEAD_BIT: u32 = 18;
pub const DLM_IFL_OVERLAP_UNLOCK_BIT: u32 = 19;
pub const DLM_IFL_OVERLAP_CANCEL_BIT: u32 = 20;
pub const DLM_IFL_ENDOFLIFE_BIT: u32 = 21;
pub const DLM_IFL_DEADLOCK_CANCEL_BIT: u32 = 24;
pub const __DLM_IFL_MAX_BIT: u32 = DLM_IFL_DEADLOCK_CANCEL_BIT;
pub const DLM_DFL_USER_BIT: u32 = 0;
pub const __DLM_DFL_MIN_BIT: u32 = DLM_DFL_USER_BIT;
pub const DLM_DFL_ORPHAN_BIT: u32 = 1;
pub const __DLM_DFL_MAX_BIT: u32 = DLM_DFL_ORPHAN_BIT;
pub const DLM_CB_CAST: u32 = 0x00000001;
pub const DLM_CB_BAST: u32 = 0x00000002;

#[repr(C)]
pub struct dlm_member { pub list: list_head, pub nodeid: i32, pub weight: i32, pub slot: i32, pub slot_prev: i32, pub comm_seq: i32, pub generation: u32 }
#[repr(C)]
pub struct dlm_recover { pub list: list_head, pub nodes: *mut dlm_config_node, pub nodes_count: i32, pub seq: u64 }
#[repr(C)]
pub struct dlm_args { pub flags: u32, pub astfn: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub astparam: *mut core::ffi::c_void, pub bastfn: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)>, pub mode: i32, pub lksb: *mut dlm_lksb }

#[repr(C)]
pub struct dlm_user_args {
    pub proc: *mut dlm_user_proc, pub lksb: dlm_lksb, pub user_lksb: *mut dlm_lksb,
    pub castparam: *mut core::ffi::c_void, pub castaddr: *mut core::ffi::c_void,
    pub bastparam: *mut core::ffi::c_void, pub bastaddr: *mut core::ffi::c_void, pub xid: u64,
}
#[repr(C)]
pub union dlm_callback_union { pub astparam: *mut core::ffi::c_void, pub ua: ManuallyDrop<dlm_user_args> }
#[repr(C)]
pub struct dlm_callback {
    pub flags: u32, pub sb_status: i32, pub sb_flags: u8, pub mode: i8, pub copy_lvb: bool,
    pub lkb_lksb: *mut dlm_lksb, pub lvbptr: [u8; DLM_USER_LVB_LEN], pub _u: dlm_callback_union,
    pub work: work_struct, pub bastfn: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)>,
    pub astfn: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub res_name: [i8; DLM_RESNAME_MAXLEN],
    pub res_length: usize, pub ls_id: u32, pub lkb_id: u32, pub list: list_head,
}

#[repr(C)]
pub union dlm_lkb_union { pub lkb_astparam: *mut core::ffi::c_void, pub lkb_ua: *mut dlm_user_args }
#[repr(C)]
pub struct dlm_lkb {
    pub lkb_resource: *mut dlm_rsb, pub lkb_ref: kref, pub lkb_nodeid: i32, pub lkb_ownpid: i32,
    pub lkb_id: u32, pub lkb_remid: u32, pub lkb_exflags: u32, pub lkb_sbflags: c_ulong,
    pub lkb_dflags: c_ulong, pub lkb_iflags: c_ulong, pub lkb_lvbseq: u32,
    pub lkb_status: i8, pub lkb_rqmode: i8, pub lkb_grmode: i8, pub lkb_highbast: i8,
    pub lkb_wait_type: i8, pub lkb_wait_count: i8, pub lkb_wait_nodeid: i32,
    pub lkb_statequeue: list_head, pub lkb_rsb_lookup: list_head, pub lkb_wait_reply: list_head,
    pub lkb_ownqueue: list_head, pub lkb_timestamp: ktime_t, pub lkb_last_cast_cb_mode: i8,
    pub lkb_last_bast_cb_mode: i8, pub lkb_last_cb_mode: i8, pub lkb_last_cb_flags: u8,
    pub lkb_last_cast_time: ktime_t, pub lkb_last_bast_time: ktime_t, pub lkb_recover_seq: u64,
    pub lkb_lvbptr: *mut i8, pub lkb_lksb: *mut dlm_lksb,
    pub lkb_astfn: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub lkb_bastfn: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)>, pub _u: dlm_lkb_union,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct dlm_rsb {
    pub res_ls: *mut dlm_ls, pub res_ref: kref, pub res_lock: spinlock_t, pub res_flags: c_ulong,
    pub res_length: i32, pub res_nodeid: i32, pub res_master_nodeid: i32, pub res_dir_nodeid: i32,
    pub res_id: c_ulong, pub res_lvbseq: u32, pub res_hash: u32, pub res_toss_time: c_ulong,
    pub res_first_lkid: u32, pub res_lookup: list_head, pub res_node: rhash_head,
    pub res_grantqueue: list_head, pub res_convertqueue: list_head, pub res_waitqueue: list_head,
    pub res_slow_list: list_head, pub res_scan_list: list_head, pub res_root_list: list_head,
    pub res_masters_list: list_head, pub res_recover_list: list_head, pub res_recover_locks_count: i32,
    pub rcu: rcu_head, pub res_lvbptr: *mut i8, pub res_name: [i8; DLM_RESNAME_MAXLEN + 1],
}

pub const DLM_LU_RECOVER_DIR: u32 = 1; pub const DLM_LU_RECOVER_MASTER: u32 = 2;
pub const DLM_LU_MATCH: u32 = 1; pub const DLM_LU_ADD: u32 = 2;
pub const R_REQUEST: u32 = 1; pub const R_RECEIVE_REQUEST: u32 = 2; pub const R_RECEIVE_RECOVER: u32 = 4;
#[repr(C)] pub enum rsb_flags { RSB_MASTER_UNCERTAIN, RSB_VALNOTVALID, RSB_VALNOTVALID_PREV, RSB_NEW_MASTER, RSB_NEW_MASTER2, RSB_RECOVER_CONVERT, RSB_RECOVER_GRANT, RSB_RECOVER_LVB_INVAL, RSB_INACTIVE, RSB_HASHED }

#[inline] pub unsafe fn rsb_set_flag(r: *mut dlm_rsb, flag: rsb_flags) { __set_bit(flag as usize, &mut (*r).res_flags); }
#[inline] pub unsafe fn rsb_clear_flag(r: *mut dlm_rsb, flag: rsb_flags) { __clear_bit(flag as usize, &mut (*r).res_flags); }
#[inline] pub unsafe fn rsb_flag(r: *mut dlm_rsb, flag: rsb_flags) -> i32 { test_bit(flag as usize, &(*r).res_flags) }

pub const DLM_HEADER_MAJOR: u32 = 0x00030000; pub const DLM_HEADER_MINOR: u32 = 2;
pub const DLM_VERSION_3_1: u32 = 0x00030001; pub const DLM_VERSION_3_2: u32 = 0x00030002; pub const DLM_HEADER_SLOTS: u32 = 1;
pub const DLM_MSG: u32 = 1; pub const DLM_RCOM: u32 = 2; pub const DLM_OPTS: u32 = 3; pub const DLM_ACK: u32 = 4; pub const DLM_FIN: u32 = 5;
#[repr(C)] pub union dlm_header_union { pub h_lockspace: __le32, pub h_seq: __le32 }
#[repr(C)] pub struct dlm_header { pub h_version: __le32, pub u: dlm_header_union, pub h_nodeid: __le32, pub h_length: __le16, pub h_cmd: u8, pub h_pad: u8 }

pub const DLM_MSG_REQUEST: u32=1; pub const DLM_MSG_CONVERT: u32=2; pub const DLM_MSG_UNLOCK: u32=3; pub const DLM_MSG_CANCEL: u32=4; pub const DLM_MSG_REQUEST_REPLY: u32=5; pub const DLM_MSG_CONVERT_REPLY: u32=6; pub const DLM_MSG_UNLOCK_REPLY: u32=7; pub const DLM_MSG_CANCEL_REPLY: u32=8; pub const DLM_MSG_GRANT: u32=9; pub const DLM_MSG_BAST: u32=10; pub const DLM_MSG_LOOKUP: u32=11; pub const DLM_MSG_REMOVE: u32=12; pub const DLM_MSG_LOOKUP_REPLY: u32=13; pub const DLM_MSG_PURGE: u32=14;
#[repr(C)] pub struct dlm_message { pub m_header: dlm_header, pub m_type: __le32, pub m_nodeid: __le32, pub m_pid: __le32, pub m_lkid: __le32, pub m_remid: __le32, pub m_parent_lkid: __le32, pub m_parent_remid: __le32, pub m_exflags: __le32, pub m_sbflags: __le32, pub m_flags: __le32, pub m_lvbseq: __le32, pub m_hash: __le32, pub m_status: __le32, pub m_grmode: __le32, pub m_rqmode: __le32, pub m_bastmode: __le32, pub m_asts: __le32, pub m_result: __le32, pub m_extra: [i8; 0] }
#[repr(C)] pub struct dlm_rcom { pub rc_header: dlm_header, pub rc_type: __le32, pub rc_result: __le32, pub rc_id: __le64, pub rc_seq: __le64, pub rc_seq_reply: __le64, pub rc_buf: [i8; 0] }
#[repr(C)] pub struct dlm_opt_header { pub t_type: __le16, pub t_length: __le16, pub t_pad: __le32, pub t_value: [i8; 0] }
#[repr(C)] pub struct dlm_opts { pub o_header: dlm_header, pub o_nextcmd: u8, pub o_pad: u8, pub o_optlen: __le16, pub o_pad2: __le32, pub o_opts: [i8; 0] }
#[repr(C)] pub union dlm_packet { pub header: ManuallyDrop<dlm_header>, pub message: ManuallyDrop<dlm_message>, pub rcom: ManuallyDrop<dlm_rcom>, pub opts: ManuallyDrop<dlm_opts> }

pub const DLM_RSF_NEED_SLOTS: u32 = 1;
#[repr(C)] pub struct rcom_status { pub rs_flags: __le32, pub rs_unused1: __le32, pub rs_unused2: __le64 }
#[repr(C)] pub struct rcom_config { pub rf_lvblen: __le32, pub rf_lsflags: __le32, pub rf_flags: __le32, pub rf_our_slot: __le16, pub rf_num_slots: __le16, pub rf_generation: __le32, pub rf_unused1: __le32, pub rf_unused2: __le64 }
#[repr(C)] pub struct rcom_slot { pub ro_nodeid: __le32, pub ro_slot: __le16, pub ro_unused1: __le16, pub ro_unused2: __le64 }
#[repr(C)] pub struct rcom_lock { pub rl_ownpid: __le32, pub rl_lkid: __le32, pub rl_remid: __le32, pub rl_parent_lkid: __le32, pub rl_parent_remid: __le32, pub rl_exflags: __le32, pub rl_flags: __le32, pub rl_lvbseq: __le32, pub rl_result: __le32, pub rl_rqmode: i8, pub rl_grmode: i8, pub rl_status: i8, pub rl_asts: i8, pub rl_wait_type: __le16, pub rl_namelen: __le16, pub rl_name: [i8; DLM_RESNAME_MAXLEN], pub rl_lvb: [i8; 0] }

pub const LSFL_RECOVER_STOP: u32=0; pub const LSFL_RECOVER_DOWN: u32=1; pub const LSFL_RECOVER_LOCK: u32=2; pub const LSFL_RECOVER_WORK: u32=3; pub const LSFL_RUNNING: u32=4; pub const LSFL_RCOM_READY: u32=5; pub const LSFL_RCOM_WAIT: u32=6; pub const LSFL_UEVENT_WAIT: u32=7; pub const LSFL_CB_DELAY: u32=9; pub const LSFL_NODIR: u32=10; pub const LSFL_RECV_MSG_BLOCKED: u32=11; pub const LSFL_FS: u32=12; pub const LSFL_SOFTIRQ: u32=13;
pub const DLM_PROC_FLAGS_CLOSING: u32=1; pub const DLM_PROC_FLAGS_COMPAT: u32=2;
#[repr(C)] pub struct dlm_ls {
    pub ls_list:list_head,pub ls_global_id:u32,pub ls_generation:u32,pub ls_exflags:u32,pub ls_lvblen:i32,pub ls_count:atomic_t,pub ls_count_wait:wait_queue_head_t,pub ls_create_count:i32,pub ls_flags:c_ulong,pub ls_kobj:kobject,
    pub ls_lkbxa:xarray,pub ls_lkbxa_lock:rwlock_t,pub ls_rsbtbl:rhashtable,pub ls_rsbtbl_lock:rwlock_t,pub ls_slow_inactive:list_head,pub ls_slow_active:list_head,pub ls_scan_timer:timer_list,pub ls_scan_list:list_head,pub ls_scan_lock:spinlock_t,pub ls_waiters_lock:spinlock_t,pub ls_waiters:list_head,pub ls_orphans_lock:spinlock_t,pub ls_orphans:list_head,
    pub ls_nodes:list_head,pub ls_nodes_gone:list_head,pub ls_num_nodes:i32,pub ls_low_nodeid:i32,pub ls_total_weight:i32,pub ls_node_array:*mut i32,pub ls_slot:i32,pub ls_num_slots:i32,pub ls_slots_size:i32,pub ls_slots:*mut dlm_slot,pub ls_local_rsb:dlm_rsb,pub ls_local_lkb:dlm_lkb,
    pub ls_debug_rsb_dentry:*mut dentry,pub ls_debug_waiters_dentry:*mut dentry,pub ls_debug_locks_dentry:*mut dentry,pub ls_debug_all_dentry:*mut dentry,pub ls_debug_toss_dentry:*mut dentry,pub ls_debug_queued_asts_dentry:*mut dentry,pub ls_uevent_wait:wait_queue_head_t,pub ls_uevent_result:i32,pub ls_recovery_done:completion,pub ls_recovery_result:i32,pub ls_device:miscdevice,pub ls_callback_wq:*mut workqueue_struct,
    pub ls_cb_lock:spinlock_t,pub ls_cb_delay:list_head,pub ls_recoverd_task:*mut task_struct,pub ls_recoverd_active:mutex,pub ls_recover_lock:spinlock_t,pub ls_recover_begin:c_ulong,pub ls_recover_status:u32,pub ls_recover_seq:u64,pub ls_recover_args:*mut dlm_recover,pub ls_in_recovery:rw_semaphore,pub ls_recv_active:rwlock_t,pub ls_requestqueue:list_head,pub ls_requestqueue_lock:rwlock_t,pub ls_recover_buf:*mut dlm_rcom,pub ls_recover_nodeid:i32,pub ls_recover_locks_in:u32,pub ls_rcom_seq:u64,pub ls_rcom_spin:spinlock_t,pub ls_recover_list:list_head,pub ls_recover_list_lock:spinlock_t,pub ls_recover_list_count:i32,pub ls_recover_xa:xarray,pub ls_recover_xa_lock:spinlock_t,pub ls_wait_general:wait_queue_head_t,pub ls_recover_lock_wait:wait_queue_head_t,pub ls_clear_proc_locks:spinlock_t,pub ls_masters_list:list_head,pub ls_masters_lock:rwlock_t,pub ls_dir_dump_list:list_head,pub ls_dir_dump_lock:rwlock_t,pub ls_ops:*const dlm_lockspace_ops,pub ls_ops_arg:*mut core::ffi::c_void,pub ls_free_work:work_struct,pub ls_namelen:i32,pub ls_name:[i8;DLM_LOCKSPACE_LEN+1],pub ls_local_ms:dlm_message,
}
#[repr(C)] pub struct dlm_user_proc { pub lockspace: *mut dlm_lockspace_t, pub flags: c_ulong, pub asts: list_head, pub asts_spin: spinlock_t, pub locks: list_head, pub locks_spin: spinlock_t, pub unlocking: list_head, pub wait: wait_queue_head_t }

pub const DLM_SBF_DEMOTED_BIT: u32=0; pub const __DLM_SBF_MIN_BIT: u32=0; pub const DLM_SBF_VALNOTVALID_BIT: u32=1; pub const DLM_SBF_ALTMODE_BIT: u32=2; pub const __DLM_SBF_MAX_BIT: u32=2;
#[inline] pub unsafe fn dlm_flags_val(addr:*const c_ulong,min:u32,max:u32)->u32 { let mut val=0; let mut bit=min; while bit<=max { if test_bit(bit as usize,&*addr)!=0 { val|=1u32<<bit; } bit+=1; } val }
#[inline] pub unsafe fn dlm_iflags_val(lkb:*const dlm_lkb)->u32 { dlm_flags_val(&(*lkb).lkb_iflags,__DLM_IFL_MIN_BIT,__DLM_IFL_MAX_BIT) }
#[inline] pub unsafe fn dlm_dflags_val(lkb:*const dlm_lkb)->u32 { dlm_flags_val(&(*lkb).lkb_dflags,__DLM_DFL_MIN_BIT,__DLM_DFL_MAX_BIT) }
#[inline] pub unsafe fn dlm_sbflags_val(lkb:*const dlm_lkb)->u32 { dlm_flags_val(&(*lkb).lkb_sbflags,__DLM_SBF_MIN_BIT,__DLM_SBF_MAX_BIT) }
#[inline] pub unsafe fn dlm_set_flags_val(addr:*mut c_ulong,val:u32,min:u32,max:u32) { let mut bit=min; while bit<=max { if val&(1<<bit)!=0 { set_bit(bit as usize,addr); } else { clear_bit(bit as usize,addr); } bit+=1; } }
#[inline] pub unsafe fn dlm_set_dflags_val(lkb:*mut dlm_lkb,val:u32) { dlm_set_flags_val(&mut (*lkb).lkb_dflags,val,__DLM_DFL_MIN_BIT,__DLM_DFL_MAX_BIT) }
#[inline] pub unsafe fn dlm_set_sbflags_val(lkb:*mut dlm_lkb,val:u32) { dlm_set_flags_val(&mut (*lkb).lkb_sbflags,val,__DLM_SBF_MIN_BIT,__DLM_SBF_MAX_BIT) }
#[inline] pub unsafe fn dlm_locking_stopped(ls: *mut dlm_ls) -> i32 { !test_bit(LSFL_RUNNING as usize, &(*ls).ls_flags) }
#[inline] pub unsafe fn dlm_recovery_stopped(ls: *mut dlm_ls) -> i32 { test_bit(LSFL_RECOVER_STOP as usize, &(*ls).ls_flags) }
#[inline] pub unsafe fn dlm_no_directory(ls: *mut dlm_ls) -> i32 { test_bit(LSFL_NODIR as usize, &(*ls).ls_flags) }

extern "C" { pub static mut dlm_wq: *mut workqueue_struct; pub fn dlm_plock_init() -> i32; pub fn dlm_plock_exit(); }
#[cfg(feature = "CONFIG_DLM_DEBUG")] extern "C" { pub fn dlm_register_debugfs(); pub fn dlm_unregister_debugfs(); pub fn dlm_create_debug_file(ls: *mut dlm_ls); pub fn dlm_delete_debug_file(ls: *mut dlm_ls); pub fn dlm_create_debug_comms_file(nodeid: i32, data: *mut core::ffi::c_void) -> *mut core::ffi::c_void; pub fn dlm_delete_debug_comms_file(ctx: *mut core::ffi::c_void); }
#[cfg(not(feature = "CONFIG_DLM_DEBUG"))] #[inline] pub unsafe fn dlm_register_debugfs() {}
#[cfg(not(feature = "CONFIG_DLM_DEBUG"))] #[inline] pub unsafe fn dlm_unregister_debugfs() {}
#[cfg(not(feature = "CONFIG_DLM_DEBUG"))] #[inline] pub unsafe fn dlm_create_debug_file(_: *mut dlm_ls) {}
#[cfg(not(feature = "CONFIG_DLM_DEBUG"))] #[inline] pub unsafe fn dlm_delete_debug_file(_: *mut dlm_ls) {}
#[cfg(not(feature = "CONFIG_DLM_DEBUG"))] #[inline] pub unsafe fn dlm_create_debug_comms_file(_: i32, _: *mut core::ffi::c_void) -> *mut core::ffi::c_void { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_DLM_DEBUG"))] #[inline] pub unsafe fn dlm_delete_debug_comms_file(_: *mut core::ffi::c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
