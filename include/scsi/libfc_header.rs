/* SPDX-License-Identifier: GPL-2.0-only */
/* Translation of libfc.h. External kernel types and functions are dependencies. */
use core::ffi::c_void;
use core::ffi::{c_int, c_uint};
type ulong = usize;

pub const FC_FC4_PROV_SIZE: usize = (FC_TYPE_FCP + 1) as usize;
pub const FC_NO_ERR: i32 = 0;
pub const FC_EX_TIMEOUT: i32 = 1;
pub const FC_EX_CLOSED: i32 = 2;
pub const FC_EX_ALLOC_ERR: i32 = 3;
pub const FC_EX_XMIT_ERR: i32 = 4;
pub const FC_EX_ELS_RJT: i32 = 5;
pub const FC_EX_INV_LOGIN: i32 = 6;
pub const FC_EX_SEQ_ERR: i32 = 6;

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_lport_state { LPORT_ST_DISABLED=0, LPORT_ST_FLOGI, LPORT_ST_DNS, LPORT_ST_RNN_ID, LPORT_ST_RSNN_NN, LPORT_ST_RSPN_ID, LPORT_ST_RFT_ID, LPORT_ST_RFF_ID, LPORT_ST_FDMI, LPORT_ST_RHBA, LPORT_ST_RPA, LPORT_ST_DHBA, LPORT_ST_DPRT, LPORT_ST_SCR, LPORT_ST_READY, LPORT_ST_LOGO, LPORT_ST_RESET }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_disc_event { DISC_EV_NONE=0, DISC_EV_SUCCESS, DISC_EV_FAILED }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_rport_state { RPORT_ST_INIT=0, RPORT_ST_FLOGI, RPORT_ST_PLOGI_WAIT, RPORT_ST_PLOGI, RPORT_ST_PRLI, RPORT_ST_RTV, RPORT_ST_READY, RPORT_ST_ADISC, RPORT_ST_DELETE }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_rport_event { RPORT_EV_NONE=0, RPORT_EV_READY, RPORT_EV_FAILED, RPORT_EV_STOP, RPORT_EV_LOGO }

#[repr(C)] pub struct fc_disc_port { pub lp:*mut fc_lport, pub peers:list_head, pub rport_work:work_struct, pub port_id:u32 }
#[repr(C)] pub struct fc_rport_operations { pub event_callback: Option<unsafe extern "C" fn(*mut fc_lport,*mut fc_rport_priv,fc_rport_event)> }
#[repr(C)] pub struct fc_rport_libfc_priv { pub local_port:*mut fc_lport, pub rp_state:fc_rport_state, pub flags:u16, pub e_d_tov:c_uint, pub r_a_tov:c_uint }
pub const FC_RP_FLAGS_REC_SUPPORTED:u16=1<<0; pub const FC_RP_FLAGS_RETRY:u16=1<<1; pub const FC_RP_STARTED:u16=1<<2; pub const FC_RP_FLAGS_CONF_REQ:u16=1<<3;
#[repr(C)] pub struct fc_rport_priv { pub local_port:*mut fc_lport,pub rport:*mut fc_rport,pub kref:kref,pub rp_state:fc_rport_state,pub ids:fc_rport_identifiers,pub flags:u16,pub max_seq:u16,pub disc_id:u16,pub maxframe_size:u16,pub retries:c_uint,pub major_retries:c_uint,pub e_d_tov:c_uint,pub r_a_tov:c_uint,pub rp_mutex:mutex,pub retry_work:delayed_work,pub event:fc_rport_event,pub ops:*mut fc_rport_operations,pub peers:list_head,pub event_work:work_struct,pub supported_classes:u32,pub prli_count:u16,pub rcu:rcu_head,pub sp_features:u16,pub spp_type:u8,pub lld_event_callback:Option<unsafe extern "C" fn(*mut fc_lport,*mut fc_rport_priv,fc_rport_event)> }

#[repr(C)] pub struct fc_stats { pub SecondsSinceLastReset:u64,pub TxFrames:u64,pub TxWords:u64,pub RxFrames:u64,pub RxWords:u64,pub ErrorFrames:u64,pub DumpedFrames:u64,pub FcpPktAllocFails:u64,pub FcpPktAborts:u64,pub FcpFrameAllocFails:u64,pub LinkFailureCount:u64,pub LossOfSignalCount:u64,pub InvalidTxWordCount:u64,pub InvalidCRCCount:u64,pub InputRequests:u64,pub OutputRequests:u64,pub ControlRequests:u64,pub InputBytes:u64,pub OutputBytes:u64,pub VLinkFailureCount:u64,pub MissDiscAdvCount:u64 }
#[repr(C)] pub struct fc_seq_els_data { pub reason:fc_els_rjt_reason,pub explan:fc_els_rjt_explan }
#[repr(C)] pub struct fc_fcp_pkt { pub scsi_pkt_lock:spinlock_t,pub ref_cnt:refcount_t,pub data_len:u32,pub cmd:*mut scsi_cmnd,pub list:list_head,pub lp:*mut fc_lport,pub state:u8,pub cdb_status:u8,pub status_code:u8,pub scsi_comp_flags:u8,pub io_status:u32,pub req_flags:u32,pub scsi_resid:u32,pub xfer_len:usize,pub cdb_cmd:fcp_cmnd,pub xfer_contig_end:u32,pub max_payload:u16,pub xfer_ddp:u16,pub rport:*mut fc_rport,pub seq_ptr:*mut fc_seq,pub timer:timer_list,pub wait_for_comp:c_int,pub timer_delay:c_int,pub recov_retry:u32,pub recov_seq:*mut fc_seq,pub tm_done:completion }
#[repr(C)] pub struct libfc_cmd_priv { pub fsp:*mut fc_fcp_pkt,pub resid_len:u32,pub status:u8 }
pub struct fc_exch_mgr; pub struct fc_exch_mgr_anchor; pub static mut fc_cpu_mask:u16;
#[repr(C)] pub struct fc_seq { pub id:u8,pub ssb_stat:u16,pub cnt:u16,pub rec_data:u32 }
pub const FC_EX_DONE:u32=1<<0; pub const FC_EX_RST_CLEANUP:u32=1<<1; pub const FC_EX_QUARANTINE:u32=1<<2;
#[repr(C)] pub struct fc_exch { pub ex_lock:spinlock_t,pub ex_refcnt:atomic_t,pub class:fc_class,pub em:*mut fc_exch_mgr,pub pool:*mut fc_exch_pool,pub ex_list:list_head,pub lp:*mut fc_lport,pub esb_stat:u32,pub state:u8,pub fh_type:u8,pub seq_id:u8,pub encaps:u8,pub xid:u16,pub oxid:u16,pub rxid:u16,pub oid:u32,pub sid:u32,pub did:u32,pub r_a_tov:u32,pub f_ctl:u32,pub seq:fc_seq,pub resp_active:c_int,pub resp_task:*mut task_struct,pub resp_wq:wait_queue_head_t,pub resp:Option<unsafe extern "C" fn(*mut fc_seq,*mut fc_frame,*mut c_void)>,pub arg:*mut c_void,pub destructor:Option<unsafe extern "C" fn(*mut fc_seq,*mut c_void)>,pub timeout_work:delayed_work }
#[repr(C)] pub struct libfc_function_template { pub frame_send:Option<unsafe extern "C" fn(*mut fc_lport,*mut fc_frame)->c_int>, pub elsct_send:Option<unsafe extern "C" fn(*mut fc_lport,u32,*mut fc_frame,c_uint,Option<unsafe extern "C" fn(*mut fc_seq,*mut fc_frame,*mut c_void)>,*mut c_void,u32)->*mut fc_seq>, pub ddp_setup:Option<unsafe extern "C" fn(*mut fc_lport,u16,*mut scatterlist,c_uint)->c_int>, pub ddp_done:Option<unsafe extern "C" fn(*mut fc_lport,u16)->c_int>, pub ddp_target:Option<unsafe extern "C" fn(*mut fc_lport,u16,*mut scatterlist,c_uint)->c_int>, pub get_lesb:Option<unsafe extern "C" fn(*mut fc_lport,*mut fc_els_lesb)>, pub exch_mgr_reset:Option<unsafe extern "C" fn(*mut fc_lport,u32,u32)>, pub lport_set_port_id:Option<unsafe extern "C" fn(*mut fc_lport,u32,*mut fc_frame)>, pub rport_event_callback:Option<unsafe extern "C" fn(*mut fc_lport,*mut fc_rport_priv,fc_rport_event)>, pub fcp_cmd_send:Option<unsafe extern "C" fn(*mut fc_lport,*mut fc_fcp_pkt,Option<unsafe extern "C" fn(*mut fc_seq,*mut fc_frame,*mut c_void)>)->c_int>, pub fcp_cleanup:Option<unsafe extern "C" fn(*mut fc_lport)>, pub fcp_abort_io:Option<unsafe extern "C" fn(*mut fc_lport)>, pub disc_recv_req:Option<unsafe extern "C" fn(*mut fc_lport,*mut fc_frame)>, pub disc_start:Option<unsafe extern "C" fn(Option<unsafe extern "C" fn(*mut fc_lport,fc_disc_event)>,*mut fc_lport)>, pub disc_stop:Option<unsafe extern "C" fn(*mut fc_lport)>, pub disc_stop_final:Option<unsafe extern "C" fn(*mut fc_lport)> }
#[repr(C)] pub struct fc_disc { pub retry_count:u8,pub pending:u8,pub requested:u8,pub seq_count:u16,pub buf_len:u8,pub disc_id:u16,pub rports:list_head,pub priv_:*mut c_void,pub disc_mutex:mutex,pub partial_buf:fc_gpn_ft_resp,pub disc_work:delayed_work,pub disc_callback:Option<unsafe extern "C" fn(*mut fc_lport,fc_disc_event)> }
#[repr(C)] pub struct fc_lport { pub host:*mut Scsi_Host,pub ema_list:list_head,pub dns_rdata:*mut fc_rport_priv,pub ms_rdata:*mut fc_rport_priv,pub ptp_rdata:*mut fc_rport_priv,pub scsi_priv:*mut c_void,pub disc:fc_disc,pub vports:list_head,pub vport:*mut fc_vport,pub tt:libfc_function_template,pub link_up:u8,pub qfull:u8,pub vlan:u16,pub state:fc_lport_state,pub boot_time:ulong,pub host_stats:fc_host_statistics,pub stats:*mut fc_stats,pub retry_count:u8,pub port_id:u32,pub wwpn:u64,pub wwnn:u64,pub service_params:c_uint,pub e_d_tov:c_uint,pub r_a_tov:c_uint,pub rnid_gen:fc_els_rnid_gen,pub sg_supp:u32,pub seq_offload:u32,pub crc_offload:u32,pub lro_enabled:u32,pub does_npiv:u32,pub npiv_enabled:u32,pub point_to_multipoint:u32,pub fdmi_enabled:u32,pub mfs:u32,pub max_retry_count:u8,pub max_rport_retry_count:u8,pub rport_priv_size:u16,pub link_speed:u16,pub link_supported_speeds:u16,pub lro_xid:u16,pub lso_max:c_uint,pub fcts:fc_ns_fts,pub lp_mutex:mutex,pub list:list_head,pub retry_work:delayed_work,pub prov:[*mut c_void; FC_FC4_PROV_SIZE],pub lport_list:list_head }
#[repr(C)] pub struct fc4_prov { pub prli:Option<unsafe extern "C" fn(*mut fc_rport_priv,u32,*const fc_els_spp,*mut fc_els_spp)->c_int>,pub prlo:Option<unsafe extern "C" fn(*mut fc_rport_priv)>,pub recv:Option<unsafe extern "C" fn(*mut fc_lport,*mut fc_frame)>,pub module:*mut module }

#[inline] pub unsafe fn fc_lport_test_ready(l:*const fc_lport)->c_int { ((*l).state==fc_lport_state::LPORT_ST_READY) as c_int }
#[inline] pub unsafe fn fc_set_wwnn(l:*mut fc_lport,v:u64){(*l).wwnn=v} #[inline] pub unsafe fn fc_set_wwpn(l:*mut fc_lport,v:u64){(*l).wwpn=v}
#[inline] pub unsafe fn fc_lport_state_enter(l:*mut fc_lport,s:fc_lport_state){if s!=(*l).state{(*l).retry_count=0}(*l).state=s}
#[inline] pub unsafe fn lport_priv(l:*const fc_lport)->*mut c_void{(l.add(1)) as *mut c_void}
#[inline] pub unsafe fn fc_fcp_is_read(f:*const fc_fcp_pkt)->bool{!f.is_null()&&!(*f).cmd.is_null()&&(*(*f).cmd).sc_data_direction==DMA_FROM_DEVICE}
#[inline] pub unsafe fn fc_disc_lport(d:*mut fc_disc)->*mut fc_lport{(d as *mut u8).sub(core::mem::offset_of!(fc_lport,disc)) as *mut fc_lport}
#[inline] pub unsafe fn fc_lport_init_stats(l:*mut fc_lport)->c_int{(*l).stats=alloc_percpu_fc_stats();if (*l).stats.is_null(){-12}else{0}}
#[inline] pub unsafe fn fc_lport_free_stats(l:*mut fc_lport){free_percpu_fc_stats((*l).stats)}
extern "C" { fn alloc_percpu_fc_stats()->*mut fc_stats; fn free_percpu_fc_stats(*mut fc_stats); }

extern "C" {
 pub fn fc_fc4_register_provider(t:fc_fh_type,p:*mut fc4_prov)->c_int; pub fn fc_fc4_deregister_provider(t:fc_fh_type,p:*mut fc4_prov);
 pub fn fc_lport_init(*mut fc_lport)->c_int; pub fn fc_lport_destroy(*mut fc_lport)->c_int; pub fn fc_fabric_logoff(*mut fc_lport)->c_int; pub fn fc_fabric_login(*mut fc_lport)->c_int; pub fn __fc_linkup(*mut fc_lport); pub fn fc_linkup(*mut fc_lport); pub fn __fc_linkdown(*mut fc_lport); pub fn fc_linkdown(*mut fc_lport); pub fn fc_vport_setlink(*mut fc_lport); pub fn fc_vports_linkchange(*mut fc_lport); pub fn fc_lport_config(*mut fc_lport)->c_int; pub fn fc_lport_reset(*mut fc_lport)->c_int; pub fn fc_lport_recv(*mut fc_lport,*mut fc_frame); pub fn fc_set_mfs(*mut fc_lport,u32)->c_int; pub fn libfc_vport_create(*mut fc_vport,c_int)->*mut fc_lport; pub fn fc_vport_id_lookup(*mut fc_lport,u32)->*mut fc_lport; pub fn fc_lport_bsg_request(*mut bsg_job)->c_int; pub fn fc_lport_set_local_id(*mut fc_lport,u32); pub fn fc_lport_iterate(Option<unsafe extern "C" fn(*mut fc_lport,*mut c_void)>,*mut c_void);
 pub fn fc_rport_terminate_io(*mut fc_rport); pub fn fc_rport_lookup(*const fc_lport,u32)->*mut fc_rport_priv; pub fn fc_rport_create(*mut fc_lport,u32)->*mut fc_rport_priv; pub fn fc_rport_destroy(*mut kref); pub fn fc_rport_login(*mut fc_rport_priv)->c_int; pub fn fc_rport_logoff(*mut fc_rport_priv)->c_int; pub fn fc_rport_recv_req(*mut fc_lport,*mut fc_frame); pub fn fc_rport_flush_queue();
 pub fn fc_disc_init(*mut fc_lport); pub fn fc_disc_config(*mut fc_lport,*mut c_void); pub fn fc_fcp_init(*mut fc_lport)->c_int; pub fn fc_fcp_destroy(*mut fc_lport);
 pub fn fc_queuecommand(*mut Scsi_Host,*mut scsi_cmnd)->scsi_qc_status; pub fn fc_eh_abort(*mut scsi_cmnd)->c_int; pub fn fc_eh_device_reset(*mut scsi_cmnd)->c_int; pub fn fc_eh_host_reset(*mut scsi_cmnd)->c_int; pub fn fc_sdev_init(*mut scsi_device)->c_int;
 pub fn fc_elsct_init(*mut fc_lport)->c_int; pub fn fc_elsct_send(*mut fc_lport,u32,*mut fc_frame,c_uint,Option<unsafe extern "C" fn(*mut fc_seq,*mut fc_frame,*mut c_void)>,*mut c_void,u32)->*mut fc_seq; pub fn fc_lport_flogi_resp(*mut fc_seq,*mut fc_frame,*mut c_void); pub fn fc_lport_logo_resp(*mut fc_seq,*mut fc_frame,*mut c_void); pub fn fc_fill_reply_hdr(*mut fc_frame,*const fc_frame,fc_rctl,u32); pub fn fc_fill_hdr(*mut fc_frame,*const fc_frame,fc_rctl,u32,u16,u32);
 pub fn fc_exch_init(*mut fc_lport)->c_int; pub fn fc_exch_update_stats(*mut fc_lport); pub fn fc_exch_recv(*mut fc_lport,*mut fc_frame); pub fn fc_exch_mgr_reset(*mut fc_lport,u32,u32); pub fn fc_seq_send(*mut fc_lport,*mut fc_seq,*mut fc_frame)->c_int; pub fn fc_seq_exch_abort(*const fc_seq,c_uint)->c_int; pub fn fc_exch_done(*mut fc_seq); pub fn fc_get_host_speed(*mut Scsi_Host); pub fn fc_get_host_port_state(*mut Scsi_Host); pub fn fc_set_rport_loss_tmo(*mut fc_rport,u32); pub fn fc_get_host_stats(*mut Scsi_Host)->*mut fc_host_statistics;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
