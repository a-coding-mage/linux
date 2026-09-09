/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of mptcp/protocol.h. Included kernel types and helpers are external dependencies. */

pub const MPTCP_SUPPORTED_VERSION: u32 = 1;

pub const OPTION_MPTCP_MPC_SYN: u32 = 1 << 0;
pub const OPTION_MPTCP_MPC_SYNACK: u32 = 1 << 1;
pub const OPTION_MPTCP_MPC_ACK: u32 = 1 << 2;
pub const OPTION_MPTCP_MPJ_SYN: u32 = 1 << 3;
pub const OPTION_MPTCP_MPJ_SYNACK: u32 = 1 << 4;
pub const OPTION_MPTCP_MPJ_ACK: u32 = 1 << 5;
pub const OPTION_MPTCP_ADD_ADDR: u32 = 1 << 6;
pub const OPTION_MPTCP_RM_ADDR: u32 = 1 << 7;
pub const OPTION_MPTCP_FASTCLOSE: u32 = 1 << 8;
pub const OPTION_MPTCP_PRIO: u32 = 1 << 9;
pub const OPTION_MPTCP_RST: u32 = 1 << 10;
pub const OPTION_MPTCP_DSS: u32 = 1 << 11;
pub const OPTION_MPTCP_FAIL: u32 = 1 << 12;
pub const OPTION_MPTCP_CSUMREQD: u32 = 1 << 13;
pub const OPTIONS_MPTCP_MPC: u32 = OPTION_MPTCP_MPC_SYN | OPTION_MPTCP_MPC_SYNACK | OPTION_MPTCP_MPC_ACK;
pub const OPTIONS_MPTCP_MPJ: u32 = OPTION_MPTCP_MPJ_SYN | OPTION_MPTCP_MPJ_SYNACK | OPTION_MPTCP_MPJ_ACK;
pub const OPTIONS_MPTCP_DSS: u32 = OPTION_MPTCP_DSS | OPTION_MPTCP_CSUMREQD;

pub const MPTCPOPT_MP_CAPABLE: u8 = 0; pub const MPTCPOPT_MP_JOIN: u8 = 1;
pub const MPTCPOPT_DSS: u8 = 2; pub const MPTCPOPT_ADD_ADDR: u8 = 3;
pub const MPTCPOPT_RM_ADDR: u8 = 4; pub const MPTCPOPT_MP_PRIO: u8 = 5;
pub const MPTCPOPT_MP_FAIL: u8 = 6; pub const MPTCPOPT_MP_FASTCLOSE: u8 = 7;
pub const MPTCPOPT_RST: u8 = 8;

pub const TCPOLEN_MPTCP_MPC_SYN: usize = 4; pub const TCPOLEN_MPTCP_MPC_SYNACK: usize = 12;
pub const TCPOLEN_MPTCP_MPC_ACK: usize = 20; pub const TCPOLEN_MPTCP_MPC_ACK_DATA: usize = 22;
pub const TCPOLEN_MPTCP_MPJ_SYN: usize = 12; pub const TCPOLEN_MPTCP_MPJ_SYNACK: usize = 16;
pub const TCPOLEN_MPTCP_MPJ_ACK: usize = 24; pub const TCPOLEN_MPTCP_DSS_BASE: usize = 4;
pub const TCPOLEN_MPTCP_DSS_ACK32: usize = 4; pub const TCPOLEN_MPTCP_DSS_ACK64: usize = 8;
pub const TCPOLEN_MPTCP_DSS_MAP32: usize = 10; pub const TCPOLEN_MPTCP_DSS_MAP64: usize = 14;
pub const TCPOLEN_MPTCP_DSS_CHECKSUM: usize = 2; pub const TCPOLEN_MPTCP_ADD_ADDR: usize = 16;
pub const TCPOLEN_MPTCP_ADD_ADDR_PORT: usize = 18; pub const TCPOLEN_MPTCP_ADD_ADDR_BASE: usize = 8;
pub const TCPOLEN_MPTCP_ADD_ADDR_BASE_PORT: usize = 10; pub const TCPOLEN_MPTCP_ADD_ADDR6: usize = 28;
pub const TCPOLEN_MPTCP_ADD_ADDR6_PORT: usize = 30; pub const TCPOLEN_MPTCP_ADD_ADDR6_BASE: usize = 20;
pub const TCPOLEN_MPTCP_ADD_ADDR6_BASE_PORT: usize = 22; pub const TCPOLEN_MPTCP_PORT_LEN: usize = 2;
pub const TCPOLEN_MPTCP_PORT_ALIGN: usize = 2; pub const TCPOLEN_MPTCP_RM_ADDR_BASE: usize = 3;
pub const TCPOLEN_MPTCP_PRIO: usize = 3; pub const TCPOLEN_MPTCP_PRIO_ALIGN: usize = 4;
pub const TCPOLEN_MPTCP_FASTCLOSE: usize = 12; pub const TCPOLEN_MPTCP_RST: usize = 4;
pub const TCPOLEN_MPTCP_FAIL: usize = 12;
pub const TCPOLEN_MPTCP_MPC_ACK_DATA_CSUM: usize = TCPOLEN_MPTCP_DSS_CHECKSUM + TCPOLEN_MPTCP_MPC_ACK_DATA;

pub const MPTCPOPT_BACKUP: u8 = 1 << 0; pub const MPTCPOPT_THMAC_LEN: usize = 8;
pub const MPTCP_VERSION_MASK: u8 = 0x0f; pub const MPTCP_CAP_CHECKSUM_REQD: u8 = 1 << 7;
pub const MPTCP_CAP_EXTENSIBILITY: u8 = 1 << 6; pub const MPTCP_CAP_DENY_JOIN_ID0: u8 = 1 << 5;
pub const MPTCP_CAP_HMAC_SHA256: u8 = 1; pub const MPTCP_CAP_FLAG_MASK: u8 = 0x1f;
pub const MPTCP_DSS_DATA_FIN: u8 = 1 << 4; pub const MPTCP_DSS_DSN64: u8 = 1 << 3;
pub const MPTCP_DSS_HAS_MAP: u8 = 1 << 2; pub const MPTCP_DSS_ACK64: u8 = 1 << 1;
pub const MPTCP_DSS_HAS_ACK: u8 = 1; pub const MPTCP_DSS_FLAG_MASK: u8 = 0x1f;
pub const MPTCP_ADDR_ECHO: u8 = 1; pub const MPTCP_PRIO_BKUP: u8 = 1; pub const MPTCP_RST_TRANSIENT: u8 = 1;
pub const MPTCP_WORK_RTX: u32 = 1; pub const MPTCP_FALLBACK_DONE: u32 = 2; pub const MPTCP_WORK_CLOSE_SUBFLOW: u32 = 3;
pub const MPTCP_PUSH_PENDING: u32 = 1; pub const MPTCP_CLEAN_UNA: u32 = 2; pub const MPTCP_ERROR_REPORT: u32 = 3;
pub const MPTCP_RETRANSMIT: u32 = 4; pub const MPTCP_FLUSH_JOIN_LIST: u32 = 5; pub const MPTCP_SYNC_STATE: u32 = 6; pub const MPTCP_SYNC_SNDBUF: u32 = 7;

#[repr(C)] pub struct mptcp_skb_cb { pub map_seq: u64, pub end_seq: u64, pub offset: u32, pub has_rxtstamp: u8, pub cant_coalesce: u8 }
pub unsafe fn before64(seq1: u64, seq2: u64) -> bool { (seq1.wrapping_sub(seq2) as i64) < 0 }
pub unsafe fn after64(seq2: u64, seq1: u64) -> bool { before64(seq1, seq2) }

#[repr(C)] pub struct mptcp_options_received {
 pub sndr_key:u64, pub rcvr_key:u64, pub data_ack:u64, pub data_seq:u64, pub subflow_seq:u32, pub data_len:u16, pub csum: u16,
 pub suboptions:u16, pub use_map:u8, pub dsn64:u8, pub data_fin:u8, pub use_ack:u8, pub ack64:u8, pub mpc_map:u8, pub reset_reason:u8, pub reset_transient:u8, pub echo:u8, pub backup:u8, pub deny_join_id0:u8,
 pub join_id:u8, pub token:u32, pub nonce:u32, pub thmac:u64, pub hmac:[u8; MPTCPOPT_THMAC_LEN], pub addr:mptcp_addr_info, pub rm_list:mptcp_rm_list, pub ahmac:u64, pub fail_seq:u64,
}
pub unsafe fn mptcp_option(subopt:u8,len:u8,nib:u8,field:u8)->u32 { htonl(((TCPOPT_MPTCP as u32)<<24)|((len as u32)<<16)|((subopt as u32)<<12)|(((nib&0xf) as u32)<<8)|field as u32) }

#[repr(C)] #[derive(Copy,Clone)] pub enum mptcp_pm_status { MPTCP_PM_ADD_ADDR_RECEIVED, MPTCP_PM_ADD_ADDR_SEND_ACK, MPTCP_PM_RM_ADDR_RECEIVED, MPTCP_PM_ESTABLISHED, MPTCP_PM_SUBFLOW_ESTABLISHED, MPTCP_PM_ALREADY_ESTABLISHED, MPTCP_PM_MPC_ENDPOINT_ACCOUNTED, MPTCP_PM_DESTROYING }
#[repr(C)] #[derive(Copy,Clone)] pub enum mptcp_pm_type { MPTCP_PM_TYPE_KERNEL=0, MPTCP_PM_TYPE_USERSPACE, __MPTCP_PM_TYPE_NR, __MPTCP_PM_TYPE_MAX=__MPTCP_PM_TYPE_NR as isize-1 }
pub const MPTCP_PM_WORK_MASK: u32 = (1 << (mptcp_pm_status::MPTCP_PM_ALREADY_ESTABLISHED as u32))-1;
#[repr(C)] pub enum mptcp_addr_signal_status { MPTCP_ADD_ADDR_SIGNAL, MPTCP_ADD_ADDR_ECHO, MPTCP_RM_ADDR_SIGNAL }
pub const MPTCP_PM_MAX_ADDR_ID: u8 = u8::MAX;

#[repr(C)] pub struct mptcp_pm_data { pub local:mptcp_addr_info, pub remote:mptcp_addr_info, pub anno_list:list_head, pub userspace_pm_local_addr_list:list_head, pub lock:spinlock_t, pub addr_signal:u8, pub server_side:bool, pub work_pending:bool, pub accept_addr:bool, pub accept_subflow:bool, pub remote_deny_join_id0:bool, pub add_addr_signaled:u8, pub add_addr_accepted:u8, pub local_addr_used:u8, pub pm_type:u8, pub extra_subflows:u8, pub status:u8, pub id_avail_bitmap:[usize; (u8::MAX as usize+1+usize::BITS as usize-1)/usize::BITS as usize], pub rm_list_tx:mptcp_rm_list, pub rm_list_rx:mptcp_rm_list }
#[repr(C)] pub struct mptcp_pm_local { pub addr:mptcp_addr_info, pub flags:u32, pub ifindex:i32 }
#[repr(C)] pub struct mptcp_pm_addr_entry { pub list:list_head, pub addr:mptcp_addr_info, pub flags:u32, pub ifindex:i32, pub lsk:*mut socket }
#[repr(C)] pub struct mptcp_data_frag { pub list:list_head, pub data_seq:u64, pub data_len:u16, pub offset:u16, pub overhead:u8, pub eor:u8, pub already_sent:u16, pub page:*mut page }
pub const MPTCP_RTT_SAMPLES: usize = 5;

/* The following C structs and inline operations retain their source ABI and depend on kernel declarations. */
#[repr(C)] pub struct mptcp_sock { pub sk:inet_connection_sock, pub local_key:u64, pub remote_key:u64, pub write_seq:u64, pub bytes_sent:u64, pub snd_nxt:u64, pub bytes_received:u64, pub ack_seq:u64, pub rcv_wnd_sent:atomic64_t, pub rcv_data_fin_seq:u64, pub bytes_retrans:u64, pub bytes_consumed:u64, pub snd_burst:i32, pub old_wspace:i32, pub recovery_snd_nxt:u64, pub bytes_acked:u64, pub snd_una:u64, pub wnd_end:u64, pub last_data_sent:u32, pub last_data_recv:u32, pub last_ack_recv:u32, pub timer_ival:usize, pub token:u32, pub flags:usize, pub cb_flags:usize, pub recovery:bool, pub can_ack:bool, pub fully_established:bool, pub rcv_data_fin:bool, pub snd_data_fin_enable:bool, pub rcv_fastclose:bool, pub use_64bit_ack:bool, pub csum_enabled:bool, pub allow_infinite_fallback:bool, pub pending_state:u8, pub mpc_endpoint_id:u8, pub recvmsg_inq:u8, pub notsent_lowat:u32, pub keepalive_cnt:i32, pub keepalive_idle:i32, pub keepalive_intvl:i32, pub maxseg:i32, pub work:work_struct, pub ooo_last_skb:*mut sk_buff, pub out_of_order_queue:rb_root, pub conn_list:list_head, pub rtx_queue:list_head, pub first_pending:*mut mptcp_data_frag, pub join_list:list_head, pub first:*mut sock, pub pm:mptcp_pm_data, pub sched:*mut mptcp_sched_ops, pub rcv_rtt_est:[u32;MPTCP_RTT_SAMPLES+1], pub rcvq_space:[u64;3], pub scaling_ratio:u8, pub allow_subflows:bool, pub subflow_id:u32, pub setsockopt_seq:u32, pub ca_name:[i8;TCP_CA_NAME_MAX], pub fallback_lock:spinlock_t, pub backlog_list:list_head, pub backlog_len:u32, pub backlog_unaccounted:u32 }

pub const MPTCP_DELEGATE_SCHEDULED:u32=0; pub const MPTCP_DELEGATE_SEND:u32=1; pub const MPTCP_DELEGATE_ACK:u32=2; pub const MPTCP_DELEGATE_SNDBUF:u32=3; pub const MPTCP_DELEGATE_ACTIONS_MASK:usize=!(1usize<<MPTCP_DELEGATE_SCHEDULED);
#[repr(C)] pub struct csum_pseudo_header { pub data_seq:u64,pub subflow_seq:u32,pub data_len:u16,pub csum:u16 }
#[repr(C)] pub struct mptcp_subflow_request_sock { pub sk:tcp_request_sock,pub mp_capable:u8,pub mp_join:u8,pub backup:u8,pub request_bkup:u8,pub csum_reqd:u8,pub allow_join_id0:u8,pub local_id:u8,pub remote_id:u8,pub local_key:u64,pub idsn:u64,pub token:u32,pub ssn_offset:u32,pub thmac:u64,pub local_nonce:u32,pub remote_nonce:u32,pub msk:*mut mptcp_sock,pub token_node:hlist_nulls_node }
pub unsafe fn mptcp_subflow_rsk(rsk:*const request_sock)->*mut mptcp_subflow_request_sock { rsk as *mut _ }
#[repr(C)] pub struct mptcp_delegated_action { pub napi:napi_struct,pub bh_lock:local_lock_t,pub head:list_head }
#[repr(C)] pub struct mptcp_subflow_context { pub node:list_head,pub avg_pacing_rate:usize,pub local_key:u64,pub remote_key:u64,pub idsn:u64,pub map_seq:u64,pub rcv_wnd_sent:u64,pub snd_isn:u32,pub token:u32,pub rel_write_seq:u32,pub map_subflow_seq:u32,pub ssn_offset:u32,pub map_data_len:u32,pub map_data_csum:u32,pub map_csum_len:u32,pub prev_rtt_seq:u32,pub request_mptcp:u8,pub request_join:u8,pub request_bkup:u8,pub mp_capable:u8,pub mp_join:u8,pub pm_notified:u8,pub conn_finished:u8,pub map_valid:u8,pub map_csum_reqd:u8,pub map_data_fin:u8,pub mpc_map:u8,pub backup:u8,pub send_mp_prio:u8,pub send_mp_fail:u8,pub send_fastclose:u8,pub send_infinite_map:u8,pub remote_key_valid:u8,pub disposable:u8,pub closing:u8,pub valid_csum_seen:u8,pub is_mptfo:u8,pub close_event_done:u8,pub mpc_drop:u8,pub data_avail:bool,pub scheduled:bool,pub pm_listener:bool,pub fully_established:bool,pub lent_mem_frag:u32,pub remote_nonce:u32,pub thmac:u64,pub local_nonce:u32,pub remote_token:u32,pub hmac_or_iasn:[u8;MPTCPOPT_THMAC_LEN],pub local_id:i16,pub remote_id:u8,pub reset_seen:u8,pub reset_transient:u8,pub reset_reason:u8,pub stale_count:u8,pub stale:u8,pub subflow_id:u32,pub delegated_status:isize,pub fail_tout:usize,pub delegated_node:list_head,pub setsockopt_seq:u32,pub stale_rcv_tstamp:u32,pub cached_sndbuf:i32,pub tcp_sock:*mut sock,pub conn:*mut sock,pub icsk_af_ops:*const inet_connection_sock_af_ops,pub tcp_state_change:Option<unsafe extern "C" fn(*mut sock)>,pub tcp_error_report:Option<unsafe extern "C" fn(*mut sock)>,pub rcu:rcu_head }

extern "C" { pub static mut mptcp_genl_family:genl_family; pub fn mptcp_subflow_process_delegated(ssk:*mut sock, actions:isize); }
/* Remaining function declarations from the C header. */
extern "C" { pub fn mptcp_is_enabled(net:*const net)->i32; pub fn mptcp_get_add_addr_timeout(net:*const net)->u32; pub fn mptcp_is_checksum_enabled(net:*const net)->i32; pub fn mptcp_allow_join_id0(net:*const net)->i32; pub fn mptcp_stale_loss_cnt(net:*const net)->u32; pub fn mptcp_close_timeout(sk:*const sock)->u32; pub fn mptcp_get_pm_type(net:*const net)->i32; pub fn mptcp_get_path_manager(net:*const net)->*const i8; pub fn mptcp_get_scheduler(net:*const net)->*const i8; pub fn mptcp_active_disable(sk:*mut sock); pub fn mptcp_active_should_disable(ssk:*mut sock)->bool; pub fn mptcp_active_enable(sk:*mut sock); pub fn mptcp_get_available_schedulers(buf:*mut i8,maxlen:usize); pub fn mptcp_set_state(sk:*mut sock,state:i32); pub fn mptcp_set_rcvlowat(sk:*mut sock,val:i32)->i32; pub fn mptcp_subflow_set_active(subflow:*mut mptcp_subflow_context); pub fn mptcp_subflow_active(subflow:*mut mptcp_subflow_context)->bool; pub fn mptcp_subflow_drop_ctx(ssk:*mut sock); pub fn mptcp_proto_init(); pub fn mptcp_finish_connect(sk:*mut sock); pub fn mptcp_get_options(skb:*const sk_buff,mp_opt:*mut mptcp_options_received); }

extern "C" {
 pub fn __mptcp_subflow_fully_established(msk:*mut mptcp_sock,subflow:*mut mptcp_subflow_context,mp_opt:*const mptcp_options_received);
 pub fn __mptcp_retransmit_pending_data(sk:*mut sock)->bool; pub fn mptcp_check_and_set_pending(sk:*mut sock); pub fn __mptcp_push_pending(sk:*mut sock,flags:u32)->(); pub fn mptcp_subflow_data_available(sk:*mut sock)->bool;
 pub fn mptcp_subflow_init(); pub fn mptcp_subflow_shutdown(sk:*mut sock,ssk:*mut sock,how:i32); pub fn mptcp_close_ssk(sk:*mut sock,ssk:*mut sock,subflow:*mut mptcp_subflow_context); pub fn __mptcp_subflow_send_ack(ssk:*mut sock); pub fn mptcp_subflow_reset(ssk:*mut sock); pub fn mptcp_subflow_queue_clean(sk:*mut sock,ssk:*mut sock); pub fn mptcp_sock_graft(sk:*mut sock,parent:*mut socket); pub fn __mptcp_nmpc_sk(msk:*mut mptcp_sock)->*mut sock; pub fn __mptcp_close(sk:*mut sock,timeout:isize)->bool; pub fn mptcp_cancel_work(sk:*mut sock); pub fn __mptcp_unaccepted_force_close(sk:*mut sock);
 pub fn mptcp_addresses_equal(a:*const mptcp_addr_info,b:*const mptcp_addr_info,use_port:bool)->bool; pub fn mptcp_local_address(skc:*const sock_common,addr:*mut mptcp_addr_info); pub fn mptcp_remote_address(skc:*const sock_common,addr:*mut mptcp_addr_info);
 pub fn __mptcp_subflow_connect(sk:*mut sock,local:*const mptcp_pm_local,remote:*const mptcp_addr_info)->i32; pub fn mptcp_subflow_create_socket(sk:*mut sock,family:u16,new_sock:*mut *mut socket)->i32; pub fn mptcp_info2sockaddr(info:*const mptcp_addr_info,addr:*mut sockaddr_storage,family:u16);
 pub fn mptcp_sched_find(name:*const i8)->*mut mptcp_sched_ops; pub fn mptcp_validate_scheduler(sched:*mut mptcp_sched_ops)->i32; pub fn mptcp_register_scheduler(sched:*mut mptcp_sched_ops)->i32; pub fn mptcp_unregister_scheduler(sched:*mut mptcp_sched_ops); pub fn mptcp_sched_init(); pub fn mptcp_init_sched(msk:*mut mptcp_sock,sched:*mut mptcp_sched_ops)->i32; pub fn mptcp_release_sched(msk:*mut mptcp_sock); pub fn mptcp_subflow_set_scheduled(subflow:*mut mptcp_subflow_context,scheduled:bool); pub fn mptcp_subflow_get_send(msk:*mut mptcp_sock)->*mut sock; pub fn mptcp_subflow_get_retrans(msk:*mut mptcp_sock)->*mut sock; pub fn mptcp_sched_get_send(msk:*mut mptcp_sock)->i32; pub fn mptcp_sched_get_retrans(msk:*mut mptcp_sock)->i32;
 pub fn mptcp_sk_clone_init(sk:*const sock,mp_opt:*const mptcp_options_received,ssk:*mut sock,req:*mut request_sock)->*mut sock; pub fn __mptcp_sync_state(sk:*mut sock,state:i32); pub fn mptcp_reset_tout_timer(msk:*mut mptcp_sock,fail_tout:usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
