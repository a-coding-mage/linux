/* SPDX-License-Identifier: GPL-2.0 */
// Translated from iscsi_target_core.h. Kernel dependencies are supplied externally.

pub const ISCSIT_VERSION: &str = "v4.1.0";
pub const ISCSI_MAX_DATASN_MISSING_COUNT: i32 = 16;
pub const ISCSI_TX_THREAD_TCP_TIMEOUT: i32 = 2;
pub const ISCSI_RX_THREAD_TCP_TIMEOUT: i32 = 2;
pub const SECONDS_FOR_ASYNC_LOGOUT: i32 = 10;
pub const SECONDS_FOR_ASYNC_TEXT: i32 = 10;
pub const SECONDS_FOR_LOGOUT_COMP: i32 = 15;
pub const WHITE_SPACE: &str = " \t\u{b}\u{c}\n\r";
pub const ISCSIT_MIN_TAGS: i32 = 16;
pub const ISCSIT_EXTRA_TAGS: i32 = 8;
pub const ISCSIT_TCP_BACKLOG: i32 = 256;
pub const ISCSI_RX_THREAD_NAME: &str = "iscsi_trx";
pub const ISCSI_TX_THREAD_NAME: &str = "iscsi_ttx";
pub const ISCSI_IQN_LEN: usize = 224;
pub const NA_AUTHENTICATION_INHERITED: i32 = -1;
pub const NA_DATAOUT_TIMEOUT: u32 = 3;
pub const NA_DATAOUT_TIMEOUT_MAX: u32 = 60;
pub const NA_DATAOUT_TIMEOUT_MIX: u32 = 2;
pub const NA_DATAOUT_TIMEOUT_RETRIES: u32 = 5;
pub const NA_DATAOUT_TIMEOUT_RETRIES_MAX: u32 = 15;
pub const NA_DATAOUT_TIMEOUT_RETRIES_MIN: u32 = 1;
pub const NA_NOPIN_TIMEOUT: u32 = 15;
pub const NA_NOPIN_TIMEOUT_MAX: u32 = 60;
pub const NA_NOPIN_TIMEOUT_MIN: u32 = 3;
pub const NA_NOPIN_RESPONSE_TIMEOUT: u32 = 30;
pub const NA_NOPIN_RESPONSE_TIMEOUT_MAX: u32 = 60;
pub const NA_NOPIN_RESPONSE_TIMEOUT_MIN: u32 = 3;
pub const NA_RANDOM_DATAIN_PDU_OFFSETS: u32 = 0;
pub const NA_RANDOM_DATAIN_SEQ_OFFSETS: u32 = 0;
pub const NA_RANDOM_R2T_OFFSETS: u32 = 0;
pub const TA_AUTHENTICATION: u32 = 1;
pub const TA_LOGIN_TIMEOUT: u32 = 15;
pub const TA_LOGIN_TIMEOUT_MAX: u32 = 30;
pub const TA_LOGIN_TIMEOUT_MIN: u32 = 5;
pub const TA_GENERATE_NODE_ACLS: u32 = 0;
pub const TA_DEFAULT_CMDSN_DEPTH: u32 = 64;
pub const TA_DEFAULT_CMDSN_DEPTH_MAX: u32 = 512;
pub const TA_DEFAULT_CMDSN_DEPTH_MIN: u32 = 1;
pub const TA_CACHE_DYNAMIC_ACLS: u32 = 0;
pub const TA_DEMO_MODE_WRITE_PROTECT: u32 = 1;
pub const TA_PROD_MODE_WRITE_PROTECT: u32 = 0;
pub const TA_DEMO_MODE_DISCOVERY: u32 = 1;
pub const TA_DEFAULT_ERL: u32 = 0;
pub const TA_CACHE_CORE_NPS: u32 = 0;
pub const TA_DEFAULT_T10_PI: u32 = 0;
pub const TA_DEFAULT_FABRIC_PROT_TYPE: u32 = 0;
pub const TA_DEFAULT_TPG_ENABLED_SENDTARGETS: u32 = 1;
pub const TA_DEFAULT_LOGIN_KEYS_WORKAROUND: u32 = 1;
pub const ISCSI_IOV_DATA_BUFFER: usize = 5;

#[repr(C)] #[derive(Copy, Clone)] pub enum iscsit_transport_type { ISCSI_TCP=0, ISCSI_SCTP_TCP=1, ISCSI_SCTP_UDP=2, ISCSI_IWARP_TCP=3, ISCSI_IWARP_SCTP=4, ISCSI_INFINIBAND=5, ISCSI_CXGBIT=6 }
#[repr(C)] #[derive(Copy, Clone)] pub enum target_conn_state_table { TARG_CONN_STATE_FREE=1, TARG_CONN_STATE_XPT_UP=3, TARG_CONN_STATE_IN_LOGIN=4, TARG_CONN_STATE_LOGGED_IN=5, TARG_CONN_STATE_IN_LOGOUT=6, TARG_CONN_STATE_LOGOUT_REQUESTED=7, TARG_CONN_STATE_CLEANUP_WAIT=8 }
#[repr(C)] #[derive(Copy, Clone)] pub enum target_sess_state_table { TARG_SESS_STATE_FREE=1, TARG_SESS_STATE_ACTIVE=2, TARG_SESS_STATE_LOGGED_IN=3, TARG_SESS_STATE_FAILED=4, TARG_SESS_STATE_IN_CONTINUE=5 }
#[repr(C)] #[derive(Copy, Clone)] pub enum data_count_type { ISCSI_RX_DATA=1, ISCSI_TX_DATA=2 }
#[repr(C)] #[derive(Copy, Clone)] pub enum datain_req_comp_table { DATAIN_COMPLETE_NORMAL=1, DATAIN_COMPLETE_WITHIN_COMMAND_RECOVERY=2, DATAIN_COMPLETE_CONNECTION_RECOVERY=3 }
#[repr(C)] #[derive(Copy, Clone)] pub enum datain_req_rec_table { DATAIN_WITHIN_COMMAND_RECOVERY=1, DATAIN_CONNECTION_RECOVERY=2 }
#[repr(C)] #[derive(Copy, Clone)] pub enum tpg_state_table { TPG_STATE_FREE=0, TPG_STATE_ACTIVE=1, TPG_STATE_INACTIVE=2, TPG_STATE_COLD_RESET=3 }
#[repr(C)] #[derive(Copy, Clone)] pub enum tiqn_state_table { TIQN_STATE_ACTIVE=1, TIQN_STATE_SHUTDOWN=2 }
#[repr(C)] #[derive(Copy, Clone)] pub enum cmd_flags_table { ICF_GOT_LAST_DATAOUT=1, ICF_GOT_DATACK_SNACK=2, ICF_NON_IMMEDIATE_UNSOLICITED_DATA=4, ICF_SENT_LAST_R2T=8, ICF_WITHIN_COMMAND_RECOVERY=0x10, ICF_CONTIG_MEMORY=0x20, ICF_ATTACHED_TO_RQUEUE=0x40, ICF_OOO_CMDSN=0x80, ICF_SENDTARGETS_ALL=0x100, ICF_SENDTARGETS_SINGLE=0x200 }
#[repr(C)] #[derive(Copy, Clone)] pub enum cmd_i_state_table { ISTATE_NO_STATE=0, ISTATE_NEW_CMD=1, ISTATE_DEFERRED_CMD=2, ISTATE_UNSOLICITED_DATA=3, ISTATE_RECEIVE_DATAOUT=4, ISTATE_RECEIVE_DATAOUT_RECOVERY=5, ISTATE_RECEIVED_LAST_DATAOUT=6, ISTATE_WITHIN_DATAOUT_RECOVERY=7, ISTATE_IN_CONNECTION_RECOVERY=8, ISTATE_RECEIVED_TASKMGT=9, ISTATE_SEND_ASYNCMSG=10, ISTATE_SENT_ASYNCMSG=11, ISTATE_SEND_DATAIN=12, ISTATE_SEND_LAST_DATAIN=13, ISTATE_SENT_LAST_DATAIN=14, ISTATE_SEND_LOGOUTRSP=15, ISTATE_SENT_LOGOUTRSP=16, ISTATE_SEND_NOPIN=17, ISTATE_SENT_NOPIN=18, ISTATE_SEND_REJECT=19, ISTATE_SENT_REJECT=20, ISTATE_SEND_R2T=21, ISTATE_SENT_R2T=22, ISTATE_SEND_R2T_RECOVERY=23, ISTATE_SENT_R2T_RECOVERY=24, ISTATE_SEND_LAST_R2T=25, ISTATE_SENT_LAST_R2T=26, ISTATE_SEND_LAST_R2T_RECOVERY=27, ISTATE_SENT_LAST_R2T_RECOVERY=28, ISTATE_SEND_STATUS=29, ISTATE_SEND_STATUS_BROKEN_PC=30, ISTATE_SENT_STATUS=31, ISTATE_SEND_STATUS_RECOVERY=32, ISTATE_SENT_STATUS_RECOVERY=33, ISTATE_SEND_TASKMGTRSP=34, ISTATE_SENT_TASKMGTRSP=35, ISTATE_SEND_TEXTRSP=36, ISTATE_SENT_TEXTRSP=37, ISTATE_SEND_NOPIN_WANT_RESPONSE=38, ISTATE_SENT_NOPIN_WANT_RESPONSE=39, ISTATE_SEND_NOPIN_NO_RESPONSE=40, ISTATE_REMOVE=41, ISTATE_FREE=42 }
#[repr(C)] #[derive(Copy, Clone)] pub enum recover_cmdsn_ret_table { CMDSN_ERROR_CANNOT_RECOVER=-1, CMDSN_NORMAL_OPERATION=0, CMDSN_LOWER_THAN_EXP=1, CMDSN_HIGHER_THAN_EXP=2, CMDSN_MAXCMDSN_OVERRUN=3 }
#[repr(C)] #[derive(Copy, Clone)] pub enum immedate_data_ret_table { IMMEDIATE_DATA_CANNOT_RECOVER=-1, IMMEDIATE_DATA_NORMAL_OPERATION=0, IMMEDIATE_DATA_ERL1_CRC_FAILURE=1 }
#[repr(C)] #[derive(Copy, Clone)] pub enum dataout_action_ret_table { DATAOUT_CANNOT_RECOVER=-1, DATAOUT_NORMAL=0, DATAOUT_SEND_R2T=1, DATAOUT_SEND_TO_TRANSPORT=2, DATAOUT_WITHIN_COMMAND_RECOVERY=3 }
#[repr(C)] #[derive(Copy, Clone)] pub enum naf_flags_table { NAF_USERID_SET=1, NAF_PASSWORD_SET=2, NAF_USERID_IN_SET=4, NAF_PASSWORD_IN_SET=8 }
#[repr(C)] #[derive(Copy, Clone)] pub enum iscsi_timer_flags_table { ISCSI_TF_RUNNING=1, ISCSI_TF_STOP=2, ISCSI_TF_EXPIRED=4 }
#[repr(C)] #[derive(Copy, Clone)] pub enum np_flags_table { NPF_IP_NETWORK=0 }
#[repr(C)] #[derive(Copy, Clone)] pub enum np_thread_state_table { ISCSI_NP_THREAD_ACTIVE=1, ISCSI_NP_THREAD_INACTIVE=2, ISCSI_NP_THREAD_RESET=3, ISCSI_NP_THREAD_SHUTDOWN=4, ISCSI_NP_THREAD_EXIT=5 }

#[repr(C)] pub struct iscsi_conn_ops { pub HeaderDigest:u8, pub DataDigest:u8, pub MaxRecvDataSegmentLength:u32, pub MaxXmitDataSegmentLength:u32, pub InitiatorRecvDataSegmentLength:u32, pub TargetRecvDataSegmentLength:u32 }
#[repr(C)] pub struct iscsi_sess_ops { pub InitiatorName:[i8;ISCSI_IQN_LEN], pub InitiatorAlias:[i8;256], pub TargetName:[i8;ISCSI_IQN_LEN], pub TargetAlias:[i8;256], pub TargetAddress:[i8;256], pub TargetPortalGroupTag:u16, pub MaxConnections:u16, pub InitialR2T:u8, pub ImmediateData:u8, pub MaxBurstLength:u32, pub FirstBurstLength:u32, pub DefaultTime2Wait:u16, pub DefaultTime2Retain:u16, pub MaxOutstandingR2T:u16, pub DataPDUInOrder:u8, pub DataSequenceInOrder:u8, pub ErrorRecoveryLevel:u8, pub SessionType:u8, pub RDMAExtensions:u8 }
#[repr(C)] pub struct iscsi_queue_req { pub state:i32, pub cmd:*mut iscsit_cmd, pub qr_list:list_head }
#[repr(C)] pub struct iscsi_param_list { pub iser:bool, pub param_list:list_head, pub extra_response_list:list_head }
#[repr(C)] pub struct iscsi_datain_req { pub dr_complete:datain_req_comp_table, pub generate_recovery_values:i32, pub recovery:datain_req_rec_table, pub begrun:u32, pub runlength:u32, pub data_length:u32, pub data_offset:u32, pub data_sn:u32, pub next_burst_len:u32, pub read_data_done:u32, pub seq_send_order:u32, pub cmd_datain_node:list_head }
#[repr(C)] pub struct iscsi_ooo_cmdsn { pub cid:u16, pub batch_count:u32, pub cmdsn:u32, pub exp_cmdsn:u32, pub cmd:*mut iscsit_cmd, pub ooo_list:list_head }
#[repr(C)] pub struct iscsi_datain { pub flags:u8, pub data_sn:u32, pub length:u32, pub offset:u32 }
#[repr(C)] pub struct iscsi_r2t { pub seq_complete:i32, pub recovery_r2t:i32, pub sent_r2t:i32, pub r2t_sn:u32, pub offset:u32, pub targ_xfer_tag:u32, pub xfer_len:u32, pub r2t_list:list_head }

// The remaining declarations retain C layout and refer to kernel/external types supplied by the translated dependency set.
#[repr(C)] pub struct iscsit_cmd { pub dataout_timer_flags:iscsi_timer_flags_table, pub dataout_timeout_retries:u8, pub error_recovery_count:u8, pub deferred_i_state:cmd_i_state_table, pub i_state:cmd_i_state_table, pub immediate_cmd:u8, pub immediate_data:u8, pub iscsi_opcode:u8, pub iscsi_response:u8, pub logout_reason:u8, pub logout_response:u8, pub maxcmdsn_inc:u8, pub unsolicited_data:u8, pub reject_reason:u8, pub logout_cid:u16, pub cmd_flags:cmd_flags_table, pub init_task_tag:itt_t, pub targ_xfer_tag:u32, pub cmd_sn:u32, pub exp_stat_sn:u32, pub stat_sn:u32, pub data_sn:u32, pub r2t_sn:u32, pub acked_data_sn:u32, pub buf_ptr_size:u32, pub data_crc:u32, pub outstanding_r2ts:u32, pub r2t_offset:u32, pub iov_data_count:u32, pub orig_iov_data_count:u32, pub iov_misc_count:u32, pub pdu_count:u32, pub pdu_send_order:u32, pub pdu_start:u32, pub seq_send_order:u32, pub seq_count:u32, pub seq_no:u32, pub seq_start_offset:u32, pub seq_end_offset:u32, pub read_data_done:u32, pub write_data_done:u32, pub first_burst_len:u32, pub next_burst_len:u32, pub tx_size:u32, pub buf_ptr:*mut core::ffi::c_void, pub text_in_ptr:*mut core::ffi::c_void, pub data_direction:dma_data_direction, pub pdu:[u8;ISCSI_HDR_LEN+ISCSI_CRC_LEN], pub immed_queue_count:atomic_t, pub response_queue_count:atomic_t, pub datain_lock:spinlock_t, pub dataout_timeout_lock:spinlock_t, pub istate_lock:spinlock_t, pub error_lock:spinlock_t, pub r2t_lock:spinlock_t, pub datain_list:list_head, pub cmd_r2t_list:list_head, pub dataout_timer:timer_list, pub iov_data:*mut kvec, pub overflow_buf:*mut core::ffi::c_void, pub iov_misc:[kvec;5], pub pdu_list:*mut iscsi_pdu, pub pdu_ptr:*mut iscsi_pdu, pub seq_list:*mut iscsi_seq, pub seq_ptr:*mut iscsi_seq, pub tmr_req:*mut iscsi_tmr_req, pub conn:*mut iscsit_conn, pub cr:*mut iscsi_conn_recovery, pub sess:*mut iscsit_session, pub i_conn_node:list_head, pub se_cmd:se_cmd, pub sense_buffer:[u8;ISCSI_SENSE_BUFFER_LEN], pub padding:u32, pub pad_bytes:[u8;4], pub first_data_sg:*mut scatterlist, pub first_data_sg_off:u32, pub kmapped_nents:u32, pub sense_reason:sense_reason_t }
pub const ISCSI_MISC_IOVECS: usize = 5;
pub const ISCSI_SENSE_BUFFER_LEN: usize = TRANSPORT_SENSE_BUFFER + 2;

#[repr(C)] pub struct iscsi_tmr_req { pub task_reassign:bool, pub exp_data_sn:u32, pub ref_cmd:*mut iscsit_cmd, pub conn_recovery:*mut iscsi_conn_recovery, pub se_tmr_req:*mut se_tmr_req }
#[repr(C)] pub struct iscsi_node_attrib { pub authentication:i32, pub dataout_timeout:u32, pub dataout_timeout_retries:u32, pub default_erl:u32, pub nopin_timeout:u32, pub nopin_response_timeout:u32, pub random_datain_pdu_offsets:u32, pub random_datain_seq_offsets:u32, pub random_r2t_offsets:u32, pub tmr_cold_reset:u32, pub tmr_warm_reset:u32, pub nacl:*mut iscsi_node_acl }
pub const MAX_USER_LEN: usize = 256; pub const MAX_PASS_LEN: usize = 256;
#[repr(C)] pub struct iscsi_node_auth { pub naf_flags:naf_flags_table, pub authenticate_target:i32, pub enforce_discovery_auth:i32, pub userid:[i8;MAX_USER_LEN], pub password:[i8;MAX_PASS_LEN], pub userid_mutual:[i8;MAX_USER_LEN], pub password_mutual:[i8;MAX_PASS_LEN] }
#[repr(C)] pub struct iscsi_node_acl { pub se_node_acl:se_node_acl, pub node_attrib:iscsi_node_attrib, pub node_auth:iscsi_node_auth, pub node_stat_grps:iscsi_node_stat_grps }
#[repr(C)] pub struct iscsi_node_stat_grps { pub iscsi_sess_stats_group:config_group, pub iscsi_conn_stats_group:config_group }
#[repr(C)] pub struct iscsi_tpg_attrib { pub authentication:u32, pub login_timeout:u32, pub generate_node_acls:u32, pub cache_dynamic_acls:u32, pub default_cmdsn_depth:u32, pub demo_mode_write_protect:u32, pub prod_mode_write_protect:u32, pub demo_mode_discovery:u32, pub default_erl:u32, pub t10_pi:u8, pub fabric_prot_type:u32, pub tpg_enabled_sendtargets:u32, pub login_keys_workaround:u32, pub tpg:*mut iscsi_portal_group }
#[repr(C)] pub struct iscsi_np { pub np_network_transport:i32, pub np_ip_proto:i32, pub np_sock_type:i32, pub np_thread_state:np_thread_state_table, pub enabled:bool, pub np_reset_count:atomic_t, pub np_exports:u32, pub np_flags:np_flags_table, pub np_thread_lock:spinlock_t, pub np_restart_comp:completion, pub np_socket:*mut socket, pub np_sockaddr:sockaddr_storage, pub np_thread:*mut task_struct, pub np_context:*mut core::ffi::c_void, pub np_transport:*mut iscsit_transport, pub np_list:list_head }
#[repr(C)] pub struct iscsi_portal_group { pub tpg_chap_id:u8, pub tpg_state:tpg_state_table, pub tpgt:u16, pub ntsih:u16, pub nsessions:u32, pub num_tpg_nps:u32, pub sid:u32, pub tpg_np_lock:spinlock_t, pub tpg_state_lock:spinlock_t, pub tpg_se_tpg:se_portal_group, pub tpg_access_lock:mutex, pub np_login_sem:semaphore, pub tpg_attrib:iscsi_tpg_attrib, pub tpg_demo_auth:iscsi_node_auth, pub param_list:*mut iscsi_param_list, pub tpg_tiqn:*mut iscsi_tiqn, pub tpg_gnp_list:list_head, pub tpg_list:list_head }
#[repr(C)] pub struct iscsi_tiqn { pub tiqn:[u8;ISCSI_IQN_LEN], pub tiqn_state:tiqn_state_table, pub tiqn_access_count:i32, pub tiqn_active_tpgs:u32, pub tiqn_ntpgs:u32, pub tiqn_num_tpg_nps:u32, pub tiqn_nsessions:u32, pub tiqn_list:list_head, pub tiqn_tpg_list:list_head, pub tiqn_state_lock:spinlock_t, pub tiqn_tpg_lock:spinlock_t, pub tiqn_wwn:se_wwn, pub tiqn_index:i32 }
#[repr(C)] pub struct iscsi_conn_recovery { pub cid:u16, pub cmd_count:u32, pub maxrecvdatasegmentlength:u32, pub maxxmitdatasegmentlength:u32, pub ready_for_reallegiance:i32, pub conn_recovery_cmd_list:list_head, pub conn_recovery_cmd_lock:spinlock_t, pub time2retain_timer:timer_list, pub sess:*mut iscsit_session, pub cr_list:list_head }
#[repr(C)] pub struct iscsi_login { pub auth_complete:u8, pub checked_for_existing:u8, pub current_stage:u8, pub leading_connection:u8, pub first_request:u8, pub version_min:u8, pub version_max:u8, pub login_complete:u8, pub login_failed:u8, pub zero_tsih:bool, pub isid:[i8;6], pub cmd_sn:u32, pub init_task_tag:itt_t, pub initial_exp_statsn:u32, pub rsp_length:u32, pub cid:u16, pub tsih:u16, pub req:[i8;ISCSI_HDR_LEN], pub rsp:[i8;ISCSI_HDR_LEN], pub req_buf:*mut i8, pub rsp_buf:*mut i8, pub conn:*mut iscsit_conn, pub np:*mut iscsi_np }
#[repr(C)] pub struct iscsit_session { pub initiator_vendor:u8, pub isid:[u8;6], pub time2retain_timer_flags:iscsi_timer_flags_table, pub version_active:u8, pub cid_called:u16, pub conn_recovery_count:u16, pub tsih:u16, pub session_state:u32, pub init_task_tag:itt_t, pub targ_xfer_tag:u32, pub cmdsn_window:u32, pub cmdsn_mutex:mutex, pub exp_cmd_sn:u32, pub max_cmd_sn:atomic_t, pub sess_ooo_cmdsn_list:list_head, pub sid:u32, pub auth_type:[i8;8], pub session_index:i32, pub session_usage_count:i32, pub session_waiting_on_uc:i32, pub cmd_pdus:atomic_long_t, pub rsp_pdus:atomic_long_t, pub tx_data_octets:atomic_long_t, pub rx_data_octets:atomic_long_t, pub conn_digest_errors:atomic_long_t, pub conn_timeout_errors:atomic_long_t, pub creation_time:u64, pub nconn:atomic_t, pub session_continuation:atomic_t, pub session_fall_back_to_erl0:atomic_t, pub session_logout:atomic_t, pub session_reinstatement:atomic_t, pub session_stop_active:atomic_t, pub session_close:atomic_t, pub sess_conn_list:list_head, pub cr_active_list:list_head, pub cr_inactive_list:list_head, pub conn_lock:spinlock_t, pub cr_a_lock:spinlock_t, pub cr_i_lock:spinlock_t, pub session_usage_lock:spinlock_t, pub ttt_lock:spinlock_t, pub async_msg_comp:completion, pub reinstatement_comp:completion, pub session_wait_comp:completion, pub session_waiting_on_uc_comp:completion, pub time2retain_timer:timer_list, pub sess_ops:*mut iscsi_sess_ops, pub se_sess:*mut se_session, pub tpg:*mut iscsi_portal_group }
#[repr(C)] pub struct iscsi_tpg_np { pub tpg_np:*mut iscsi_np, pub tpg:*mut iscsi_portal_group, pub tpg_np_parent:*mut iscsi_tpg_np, pub tpg_np_list:list_head, pub tpg_np_child_list:list_head, pub tpg_np_parent_list:list_head, pub se_tpg_np:se_tpg_np, pub tpg_np_parent_lock:spinlock_t, pub tpg_np_comp:completion, pub tpg_np_kref:kref }
#[repr(C)] pub struct iscsi_wwn_stat_grps { pub iscsi_stat_group:config_group, pub iscsi_instance_group:config_group, pub iscsi_sess_err_group:config_group, pub iscsi_tgt_attr_group:config_group, pub iscsi_login_stats_group:config_group, pub iscsi_logout_stats_group:config_group }
#[repr(C)] pub struct iscsi_sess_err_stats;
#[repr(C)] pub struct iscsi_login_stats;
#[repr(C)] pub struct iscsi_logout_stats;
#[repr(C)] pub struct iscsit_conn { pub queues_wq:wait_queue_head_t, pub auth_complete:u8, pub conn_state:u8, pub conn_logout_reason:u8, pub network_transport:u8, pub nopin_timer_flags:iscsi_timer_flags_table, pub nopin_response_timer_flags:iscsi_timer_flags_table, pub which_thread:u8, pub cid:u16, pub login_port:u16, pub net_size:i32, pub login_family:i32, pub auth_id:u32, pub conn_flags:u32, pub login_itt:itt_t, pub exp_statsn:u32, pub stat_sn:u32, pub login_sockaddr:sockaddr_storage, pub local_sockaddr:sockaddr_storage, pub conn_usage_count:i32, pub conn_waiting_on_uc:i32, pub check_immediate_queue:atomic_t, pub conn_logout_remove:atomic_t, pub connection_exit:atomic_t, pub connection_recovery:atomic_t, pub connection_reinstatement:atomic_t, pub connection_wait_rcfr:atomic_t, pub sleep_on_conn_wait_comp:atomic_t, pub transport_failed:atomic_t, pub conn_post_wait_comp:completion, pub conn_wait_comp:completion, pub conn_wait_rcfr_comp:completion, pub conn_waiting_on_uc_comp:completion, pub conn_logout_comp:completion, pub tx_half_close_comp:completion, pub rx_half_close_comp:completion, pub sock:*mut socket, pub orig_data_ready:Option<unsafe extern "C" fn(*mut sock)>, pub orig_state_change:Option<unsafe extern "C" fn(*mut sock)>, pub login_flags:usize, pub login_work:delayed_work, pub login:*mut iscsi_login, pub nopin_timer:timer_list, pub nopin_response_timer:timer_list, pub login_timer:timer_list, pub login_kworker:*mut task_struct, pub cmd_lock:spinlock_t, pub conn_usage_lock:spinlock_t, pub immed_queue_lock:spinlock_t, pub nopin_timer_lock:spinlock_t, pub response_queue_lock:spinlock_t, pub state_lock:spinlock_t, pub login_timer_lock:spinlock_t, pub login_worker_lock:spinlock_t, pub conn_cpumask:cpumask_var_t, pub allowed_cpumask:cpumask_var_t, pub conn_cmd_list:list_head, pub immed_queue_list:list_head, pub response_queue_list:list_head, pub conn_ops:*mut iscsi_conn_ops, pub conn_login:*mut iscsi_login, pub conn_transport:*mut iscsit_transport, pub param_list:*mut iscsi_param_list, pub auth_protocol:*mut core::ffi::c_void, pub context:*mut core::ffi::c_void, pub login_thread:*mut iscsi_login_thread_s, pub tpg:*mut iscsi_portal_group, pub tpg_np:*mut iscsi_tpg_np, pub sess:*mut iscsit_session, pub cmd_cnt:*mut target_cmd_counter, pub bitmap_id:i32, pub rx_thread_active:i32, pub rx_thread:*mut task_struct, pub rx_login_comp:completion, pub tx_thread_active:i32, pub tx_thread:*mut task_struct, pub conn_list:list_head }
#[repr(C)] pub struct iscsit_global { pub in_shutdown:u32, pub active_ts:u32, pub auth_id:u32, pub inactive_ts:u32, pub ts_bitmap:*mut usize, pub ts_bitmap_lock:spinlock_t, pub allowed_cpumask:cpumask_var_t, pub discovery_acl:iscsi_node_acl, pub discovery_tpg:*mut iscsi_portal_group }

extern "C" { pub fn iscsit_find_cmd_from_itt(conn:*mut iscsit_conn, itt:itt_t) -> *mut iscsit_cmd; pub fn iscsit_thread_check_cpumask(conn:*mut iscsit_conn, p:*mut task_struct, mode:i32); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
