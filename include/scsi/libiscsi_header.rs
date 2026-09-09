/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of libiscsi.h. External kernel types are supplied elsewhere. */

use core::ffi::c_void;

pub const ISCSI_DEF_XMIT_CMDS_MAX: i32 = 128;
pub const ISCSI_MGMT_CMDS_MAX: i32 = 15;
pub const ISCSI_DEF_CMD_PER_LUN: i32 = 32;
pub const ISID_SIZE: usize = 6;
pub const ISCSI_CONN_FLAG_SUSPEND_TX: i32 = 0;
pub const ISCSI_CONN_FLAG_SUSPEND_RX: i32 = 1;
pub const ISCSI_CONN_FLAG_BOUND: i32 = 2;
pub const ISCSI_ITT_MASK: i32 = 0x1fff;
pub const ISCSI_TOTAL_CMDS_MAX: i32 = 4096;
pub const ISCSI_TOTAL_CMDS_MIN: i32 = 16;
pub const ISCSI_AGE_SHIFT: i32 = 28;
pub const ISCSI_AGE_MASK: i32 = 0xf;
pub const ISCSI_ADDRESS_BUF_LEN: usize = 64;
/* sizeof-based constants retain their source dependency. */
pub const ISCSI_MAX_AHS_SIZE: usize = core::mem::size_of::<iscsi_ecdb_ahdr>() + core::mem::size_of::<iscsi_rlength_ahdr>();
pub const ISCSI_DIGEST_SIZE: usize = core::mem::size_of::<u32>();

pub const TMF_INITIAL: i32 = 0;
pub const TMF_QUEUED: i32 = 1;
pub const TMF_SUCCESS: i32 = 2;
pub const TMF_FAILED: i32 = 3;
pub const TMF_TIMEDOUT: i32 = 4;
pub const TMF_NOT_FOUND: i32 = 5;

pub const ISCSI_TASK_FREE: i32 = 0;
pub const ISCSI_TASK_COMPLETED: i32 = 1;
pub const ISCSI_TASK_PENDING: i32 = 2;
pub const ISCSI_TASK_RUNNING: i32 = 3;
pub const ISCSI_TASK_ABRT_TMF: i32 = 4;
pub const ISCSI_TASK_ABRT_SESS_RECOV: i32 = 5;
pub const ISCSI_TASK_REQUEUE_SCSIQ: i32 = 6;

pub const ISCSI_CONN_INITIAL_STAGE: i32 = 0;
pub const ISCSI_CONN_STARTED: i32 = 1;
pub const ISCSI_CONN_STOPPED: i32 = 2;
pub const ISCSI_CONN_CLEANUP_WAIT: i32 = 3;

pub const ISCSI_STATE_FREE: i32 = 1;
pub const ISCSI_STATE_LOGGED_IN: i32 = 2;
pub const ISCSI_STATE_FAILED: i32 = 3;
pub const ISCSI_STATE_TERMINATE: i32 = 4;
pub const ISCSI_STATE_IN_RECOVERY: i32 = 5;
pub const ISCSI_STATE_RECOVERY_FAILED: i32 = 6;
pub const ISCSI_STATE_LOGGING_OUT: i32 = 7;

pub const ISCSI_HOST_SETUP: i32 = 0;
pub const ISCSI_HOST_REMOVED: i32 = 1;

#[repr(C)]
pub struct iscsi_r2t_info {
    pub ttt: __be32, pub exp_statsn: __be32, pub data_length: u32,
    pub data_offset: u32, pub data_count: i32, pub datasn: i32, pub sent: i32,
}

#[repr(C)]
pub struct iscsi_task {
    pub hdr: *mut iscsi_hdr, pub hdr_max: u16, pub hdr_len: u16, pub hdr_itt: itt_t,
    pub cmdsn: __be32, pub lun: scsi_lun, pub itt: i32, pub imm_count: u32,
    pub unsol_r2t: iscsi_r2t_info, pub data: *mut i8, pub data_count: u32,
    pub sc: *mut scsi_cmnd, pub conn: *mut iscsi_conn, pub last_xfer: usize,
    pub last_timeout: usize, pub have_checked_conn: bool, pub protected: bool,
    pub state: i32, pub refcount: refcount_t, pub running: list_head,
    pub dd_data: *mut c_void,
}

#[inline]
pub unsafe fn iscsi_task_has_unsol_data(task: *mut iscsi_task) -> bool {
    (*task).unsol_r2t.data_length > (*task).unsol_r2t.sent as u32
}
#[inline]
pub unsafe fn iscsi_next_hdr(task: *mut iscsi_task) -> *mut c_void {
    ((*task).hdr as *mut u8).add((*task).hdr_len as usize) as *mut c_void
}
#[inline]
pub unsafe fn iscsi_task_is_completed(task: *mut iscsi_task) -> bool {
    matches!((*task).state, ISCSI_TASK_COMPLETED | ISCSI_TASK_ABRT_TMF | ISCSI_TASK_ABRT_SESS_RECOV)
}

#[repr(C)] pub struct iscsi_cmd { pub task: *mut iscsi_task, pub age: i32 }
#[inline] pub unsafe fn iscsi_cmd(cmd: *mut scsi_cmnd) -> *mut iscsi_cmd { scsi_cmd_priv(cmd) as *mut iscsi_cmd }

#[repr(C)]
pub struct iscsi_conn {
    pub cls_conn: *mut iscsi_cls_conn, pub dd_data: *mut c_void, pub session: *mut iscsi_session,
    pub stop_stage: i32, pub transport_timer: timer_list, pub last_recv: usize, pub last_ping: usize,
    pub ping_timeout: i32, pub recv_timeout: i32, pub ping_task: *mut iscsi_task,
    pub exp_statsn: u32, pub statsn: u32, pub id: i32, pub c_stage: i32, pub data: *mut i8,
    pub login_task: *mut iscsi_task, pub task: *mut iscsi_task, pub mgmtqueue: list_head,
    pub cmdqueue: list_head, pub requeue: list_head, pub xmitwork: work_struct, pub recvwork: work_struct,
    pub flags: usize, pub max_recv_dlength: u32, pub max_xmit_dlength: u32, pub hdrdgst_en: i32,
    pub datadgst_en: i32, pub ifmarker_en: i32, pub ofmarker_en: i32, pub persistent_port: i32,
    pub persistent_address: *mut i8, pub max_segment_size: u32, pub tcp_xmit_wsf: u32, pub tcp_recv_wsf: u32,
    pub keepalive_tmo: u16, pub local_port: u16, pub tcp_timestamp_stat: u8, pub tcp_nagle_disable: u8,
    pub tcp_wsf_disable: u8, pub tcp_timer_scale: u8, pub tcp_timestamp_en: u8, pub fragment_disable: u8,
    pub ipv4_tos: u8, pub ipv6_traffic_class: u8, pub ipv6_flow_label: u8, pub is_fw_assigned_ipv6: u8,
    pub local_ipaddr: *mut i8, pub txdata_octets: u64, pub rxdata_octets: u64,
    pub scsicmd_pdus_cnt: u32, pub dataout_pdus_cnt: u32, pub scsirsp_pdus_cnt: u32,
    pub datain_pdus_cnt: u32, pub r2t_pdus_cnt: u32, pub tmfcmd_pdus_cnt: u32,
    pub tmfrsp_pdus_cnt: i32, pub eh_abort_cnt: u32, pub fmr_unalign_cnt: u32,
}

#[repr(C)] pub struct iscsi_pool { pub queue: kfifo, pub pool: *mut *mut c_void, pub max: i32 }

#[repr(C)]
pub struct iscsi_session {
    pub cls_session: *mut iscsi_cls_session, pub eh_mutex: mutex, pub ehwait: wait_queue_head_t,
    pub tmhdr: iscsi_tm, pub tmf_timer: timer_list, pub tmf_state: i32, pub running_aborted_task: *mut iscsi_task,
    pub cmdsn: u32, pub exp_cmdsn: u32, pub max_cmdsn: u32, pub queued_cmdsn: u32,
    pub abort_timeout: i32, pub lu_reset_timeout: i32, pub tgt_reset_timeout: i32, pub initial_r2t_en: i32,
    pub max_r2t: u16, pub imm_data_en: i32, pub first_burst: u32, pub max_burst: u32, pub time2wait: i32,
    pub time2retain: i32, pub pdu_inorder_en: i32, pub dataseq_inorder_en: i32, pub erl: i32, pub fast_abort: i32,
    pub tpgt: i32, pub username: *mut i8, pub username_in: *mut i8, pub password: *mut i8, pub password_in: *mut i8,
    pub targetname: *mut i8, pub targetalias: *mut i8, pub ifacename: *mut i8, pub initiatorname: *mut i8,
    pub boot_root: *mut i8, pub boot_nic: *mut i8, pub boot_target: *mut i8, pub portal_type: *mut i8,
    pub discovery_parent_type: *mut i8, pub discovery_parent_idx: u16, pub def_taskmgmt_tmo: u16, pub tsid: u16,
    pub auto_snd_tgt_disable: u8, pub discovery_sess: u8, pub chap_auth_en: u8, pub discovery_logout_en: u8,
    pub bidi_chap_en: u8, pub discovery_auth_optional: u8, pub isid: [u8; ISID_SIZE], pub tt: *mut iscsi_transport,
    pub host: *mut Scsi_Host, pub leadconn: *mut iscsi_conn, pub frwd_lock: spinlock_t, pub back_lock: spinlock_t,
    pub state: i32, pub age: i32, pub scsi_cmds_max: i32, pub cmds_max: i32, pub cmds: *mut *mut iscsi_task,
    pub cmdpool: iscsi_pool, pub dd_data: *mut c_void,
}

#[repr(C)] pub struct iscsi_host { pub initiatorname: *mut i8, pub hwaddress: *mut i8, pub netdev: *mut i8, pub session_removal_wq: wait_queue_head_t, pub lock: spinlock_t, pub num_sessions: i32, pub state: i32, pub workq: *mut workqueue_struct }

#[inline] pub fn iscsi_padded(len: u32) -> u32 { (len + ISCSI_PAD_LEN - 1) & !(ISCSI_PAD_LEN - 1) }
#[inline] pub fn iscsi_padding(mut len: u32) -> u32 { len &= ISCSI_PAD_LEN - 1; if len != 0 { len = ISCSI_PAD_LEN - len; } len }

/* Kernel declarations and external functions are intentionally referenced, not implemented here. */
extern "C" {
    pub fn iscsi_eh_abort(sc: *mut scsi_cmnd) -> i32;
    pub fn iscsi_eh_recover_target(sc: *mut scsi_cmnd) -> i32;
    pub fn iscsi_eh_session_reset(sc: *mut scsi_cmnd) -> i32;
    pub fn iscsi_eh_device_reset(sc: *mut scsi_cmnd) -> i32;
    pub fn iscsi_host_set_param(shost: *mut Scsi_Host, param: iscsi_host_param, buf: *mut i8, buflen: i32) -> i32;
    pub fn iscsi_host_get_param(shost: *mut Scsi_Host, param: iscsi_host_param, buf: *mut i8) -> i32;
    pub fn iscsi_host_add(shost: *mut Scsi_Host, pdev: *mut device) -> i32;
    pub fn iscsi_host_remove(shost: *mut Scsi_Host, is_shutdown: bool);
    pub fn iscsi_host_free(shost: *mut Scsi_Host);
    pub fn iscsi_target_alloc(starget: *mut scsi_target) -> i32;
    pub fn iscsi_session_remove(cls_session: *mut iscsi_cls_session);
    pub fn iscsi_session_free(cls_session: *mut iscsi_cls_session);
    pub fn iscsi_conn_teardown(cls_conn: *mut iscsi_cls_conn);
    pub fn iscsi_conn_start(cls_conn: *mut iscsi_cls_conn) -> i32;
    pub fn iscsi_conn_stop(cls_conn: *mut iscsi_cls_conn, flag: i32);
    pub fn iscsi_suspend_tx(conn: *mut iscsi_conn);
    pub fn iscsi_suspend_rx(conn: *mut iscsi_conn);
    pub fn iscsi_suspend_queue(conn: *mut iscsi_conn);
    pub fn iscsi_conn_queue_xmit(conn: *mut iscsi_conn);
    pub fn iscsi_conn_queue_recv(conn: *mut iscsi_conn);
    pub fn iscsi_requeue_task(task: *mut iscsi_task);
    pub fn iscsi_put_task(task: *mut iscsi_task);
    pub fn __iscsi_put_task(task: *mut iscsi_task);
    pub fn iscsi_get_task(task: *mut iscsi_task) -> bool;
    pub fn iscsi_pool_free(pool: *mut iscsi_pool);
    pub fn iscsi_pool_init(pool: *mut iscsi_pool, max: i32, p: *mut *mut *mut c_void, gfp: i32) -> i32;
    pub fn iscsi_eh_cmd_timed_out(sc: *mut scsi_cmnd) -> scsi_timeout_action;
    pub fn iscsi_host_alloc(sht: *const scsi_host_template, dd_data_size: i32, xmit_can_sleep: bool) -> *mut Scsi_Host;
    pub fn iscsi_host_get_max_scsi_cmds(shost: *mut Scsi_Host, requested_cmds_max: u16) -> i32;
    pub fn iscsi_session_setup(tt: *mut iscsi_transport, shost: *mut Scsi_Host, cmds_max: u16, dd_data_size: i32, cmd_per_lun: i32, initial_cmdsn: u32, gfp: u32) -> *mut iscsi_cls_session;
    pub fn iscsi_session_teardown(session: *mut iscsi_cls_session);
    pub fn iscsi_session_recovery_timedout(session: *mut iscsi_cls_session);
    pub fn iscsi_set_param(conn: *mut iscsi_cls_conn, param: iscsi_param, buf: *mut i8, buflen: i32) -> i32;
    pub fn iscsi_session_get_param(session: *mut iscsi_cls_session, param: iscsi_param, buf: *mut i8) -> i32;
    pub fn iscsi_conn_setup(session: *mut iscsi_cls_session, dd_data_size: i32, cid: u32) -> *mut iscsi_cls_conn;
    pub fn iscsi_conn_bind(session: *mut iscsi_cls_session, conn: *mut iscsi_cls_conn, is_leading: i32) -> i32;
    pub fn iscsi_conn_unbind(conn: *mut iscsi_cls_conn, is_active: bool);
    pub fn iscsi_conn_failure(conn: *mut iscsi_conn, err: iscsi_err);
    pub fn iscsi_session_failure(session: *mut iscsi_session, err: iscsi_err);
    pub fn iscsi_conn_get_param(conn: *mut iscsi_cls_conn, param: iscsi_param, buf: *mut i8) -> i32;
    pub fn iscsi_conn_get_addr_param(addr: *mut sockaddr_storage, param: iscsi_param, buf: *mut i8) -> i32;
    pub fn iscsi_update_cmdsn(session: *mut iscsi_session, nopin: *mut iscsi_nopin);
    pub fn iscsi_prep_data_out_pdu(task: *mut iscsi_task, r2t: *mut iscsi_r2t_info, hdr: *mut iscsi_data);
    pub fn iscsi_verify_itt(conn: *mut iscsi_conn, itt: itt_t) -> i32;
    pub fn iscsi_itt_to_ctask(conn: *mut iscsi_conn, itt: itt_t) -> *mut iscsi_task;
    pub fn iscsi_itt_to_task(conn: *mut iscsi_conn, itt: itt_t) -> *mut iscsi_task;
    pub fn iscsi_complete_scsi_task(task: *mut iscsi_task, exp_cmdsn: u32, max_cmdsn: u32);
    pub fn iscsi_switch_str_param(param: *mut *mut i8, buf: *mut i8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
