/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* Faithful Rust translation of fcoe_common.h.  External types are supplied by
 * the surrounding driver translation. */

pub const FC_ABTS_REPLY_MAX_PAYLOAD_LEN: u32 = 12;

#[repr(C)] #[derive(Copy, Clone)] pub struct protection_info_ctx { pub flags: u16, pub dix_block_size: u8, pub dst_size: u8 }
#[repr(C)] pub union protection_info_union_ctx { pub info: protection_info_ctx, pub value: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcoe_fcp_cmd_payload { pub opaque: [u32; 8] }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcoe_fcp_rsp_payload { pub opaque: [u32; 6] }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcp_rsp_payload_padded { pub rsp_payload: fcoe_fcp_rsp_payload, pub reserved: [u32; 2] }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcoe_fcp_xfer_payload { pub opaque: [u32; 3] }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcp_xfer_payload_padded { pub xfer_payload: fcoe_fcp_xfer_payload, pub reserved: [u32; 5] }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcoe_tx_data_params { pub data_offset:u32, pub offset_in_io:u32, pub flags:u8, pub dif_residual:u8, pub seq_cnt:u16, pub single_sge_saved_offset:u16, pub next_dif_offset:u16, pub seq_id:u16, pub reserved3:u16 }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcoe_tx_mid_path_params { pub parameter:u32, pub r_ctl:u8, pub type_:u8, pub cs_ctl:u8, pub df_ctl:u8, pub rx_id:u16, pub ox_id:u16 }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcoe_tx_params { pub data:fcoe_tx_data_params, pub mid_path:fcoe_tx_mid_path_params }
#[repr(C)] pub union fcoe_tx_info_union_ctx { pub fcp_cmd_payload:fcoe_fcp_cmd_payload, pub fcp_rsp_payload:fcp_rsp_payload_padded, pub fcp_xfer_payload:fcp_xfer_payload_padded, pub tx_params:fcoe_tx_params }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcoe_slow_sgl_ctx { pub base_sgl_addr:regpair, pub curr_sge_off:u16, pub remainder_num_sges:u16, pub curr_sgl_index:u16, pub reserved:u16 }
#[repr(C)] pub union fcoe_dix_desc_ctx { pub dix_sgl:fcoe_slow_sgl_ctx, pub cached_dix_sge:scsi_sge }

#[repr(C)] #[derive(Copy, Clone)] pub struct ystorm_fcoe_task_st_ctx { pub task_type:u8,pub sgl_mode:u8,pub cached_dix_sge:u8,pub expect_first_xfer:u8,pub num_pbf_zero_write:u32,pub protection_info_union:protection_info_union_ctx,pub data_2_trns_rem:u32,pub sgl_params:scsi_sgl_params,pub reserved1:[u8;12],pub tx_info_union:fcoe_tx_info_union_ctx,pub dix_desc:fcoe_dix_desc_ctx,pub data_desc:scsi_cached_sges,pub ox_id:u16,pub rx_id:u16,pub task_rety_identifier:u32,pub reserved2:[u8;8] }
#[repr(C)] #[derive(Copy, Clone)] pub struct ystorm_fcoe_task_ag_ctx { pub byte0:u8,pub byte1:u8,pub word0:u16,pub flags0:u8,pub flags1:u8,pub flags2:u8,pub byte2:u8,pub reg0:u32,pub byte3:u8,pub byte4:u8,pub rx_id:u16,pub word2:u16,pub word3:u16,pub word4:u16,pub word5:u16,pub reg1:u32,pub reg2:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tstorm_fcoe_task_ag_ctx { pub reserved:u8,pub byte1:u8,pub icid:u16,pub flags0:u8,pub flags1:u8,pub flags2:u8,pub flags3:u8,pub flags4:u8,pub cleanup_state:u8,pub last_sent_tid:u16,pub rec_rr_tov_exp_timeout:u32,pub byte3:u8,pub byte4:u8,pub word2:u16,pub word3:u16,pub word4:u16,pub data_offset_end_of_seq:u32,pub data_offset_next:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcoe_exp_ro { pub data_offset:u32,pub reserved:u32 }
#[repr(C)] pub union fcoe_cleanup_addr_exp_ro_union { pub abts_rsp_fc_payload_hi:regpair,pub exp_ro:fcoe_exp_ro }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcoe_abts_pkt { pub abts_rsp_fc_payload_lo:u32,pub abts_rsp_rx_id:u16,pub abts_rsp_rctl:u8,pub reserved2:u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcoe_tstorm_fcoe_task_st_ctx_read_write { pub cleanup_addr_exp_ro_union:fcoe_cleanup_addr_exp_ro_union,pub flags:u16,pub seq_cnt:u16,pub seq_id:u8,pub ooo_rx_seq_id:u8,pub rx_id:u16,pub abts_data:fcoe_abts_pkt,pub e_d_tov_exp_timeout_val:u32,pub ooo_rx_seq_cnt:u16,pub reserved1:u16 }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcoe_tstorm_fcoe_task_st_ctx_read_only { pub task_type:u8,pub dev_type:u8,pub conf_supported:u8,pub glbl_q_num:u8,pub cid:u32,pub fcp_cmd_trns_size:u32,pub rsrv:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct tstorm_fcoe_task_st_ctx { pub read_write:fcoe_tstorm_fcoe_task_st_ctx_read_write,pub read_only:fcoe_tstorm_fcoe_task_st_ctx_read_only }
#[repr(C)] #[derive(Copy, Clone)] pub struct mstorm_fcoe_task_ag_ctx { pub byte0:u8,pub byte1:u8,pub icid:u16,pub flags0:u8,pub flags1:u8,pub flags2:u8,pub cleanup_state:u8,pub received_bytes:u32,pub byte3:u8,pub glbl_q_num:u8,pub word1:u16,pub tid_to_xfer:u16,pub word3:u16,pub word4:u16,pub word5:u16,pub expected_bytes:u32,pub reg2:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mstorm_fcoe_task_st_ctx { pub rsp_buf_addr:regpair,pub rsrv:[u32;2],pub sgl_params:scsi_sgl_params,pub data_2_trns_rem:u32,pub data_buffer_offset:u32,pub parent_id:u16,pub flags:u16,pub data_desc:scsi_cached_sges }
#[repr(C)] #[derive(Copy, Clone)] pub struct ustorm_fcoe_task_ag_ctx { pub reserved:u8,pub byte1:u8,pub icid:u16,pub flags0:u8,pub flags1:u8,pub flags2:u8,pub flags3:u8,pub dif_err_intervals:u32,pub dif_error_1st_interval:u32,pub global_cq_num:u32,pub reg3:u32,pub reg4:u32,pub reg5:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct fcoe_task_context { pub ystorm_st_context:ystorm_fcoe_task_st_ctx,pub ystorm_st_padding:[regpair;2],pub tdif_context:tdif_task_context,pub ystorm_ag_context:ystorm_fcoe_task_ag_ctx,pub tstorm_ag_context:tstorm_fcoe_task_ag_ctx,pub timer_context:timers_context,pub tstorm_st_context:tstorm_fcoe_task_st_ctx,pub tstorm_st_padding:[regpair;2],pub mstorm_ag_context:mstorm_fcoe_task_ag_ctx,pub mstorm_st_context:mstorm_fcoe_task_st_ctx,pub ustorm_ag_context:ustorm_fcoe_task_ag_ctx,pub rdif_context:rdif_task_context }
#[repr(C)] pub union fcoe_additional_info_union { pub previous_tid:u32,pub parent_tid:u32,pub burst_length:u32,pub seq_rec_updated_offset:u32 }

#[repr(C)] #[derive(Copy,Clone)] pub struct fc_addr_nw { pub addr_lo:u8,pub addr_mid:u8,pub addr_hi:u8 }
#[repr(C)] #[derive(Copy,Clone)] pub struct fcoe_conn_terminate_ramrod_data { pub terminate_params_addr:regpair }
#[repr(C)] #[derive(Copy,Clone)] pub struct fcoe_fast_sgl_ctx { pub sgl_start_addr:regpair,pub sgl_byte_offset:u32,pub task_reuse_cnt:u16,pub init_offset_in_first_sge:u16 }
#[repr(C)] #[derive(Copy,Clone)] pub struct fcoe_stat_ramrod_data { pub stat_params_addr:regpair }
#[repr(C)] #[derive(Copy,Clone)] pub struct fcoe_tx_stat { pub fcoe_tx_byte_cnt:regpair,pub fcoe_tx_data_pkt_cnt:regpair,pub fcoe_tx_xfer_pkt_cnt:regpair,pub fcoe_tx_other_pkt_cnt:regpair }
#[repr(C)] #[derive(Copy,Clone)] pub struct xfrqe_prot_flags { pub flags:u8 }
#[repr(C)] #[derive(Copy,Clone)] pub struct fcoe_db_data { pub params:u8,pub agg_flags:u8,pub sq_prod:u16 }

#[repr(C)] #[derive(Copy,Clone)] pub struct fcoe_wqe { pub task_id:u16,pub flags:u16,pub additional_info_union:fcoe_additional_info_union }

pub const FCOE_COMPLETION_STATUS_SUCCESS:u32=0; pub const FCOE_COMPLETION_STATUS_FCOE_VER_ERR:u32=1; pub const FCOE_COMPLETION_STATUS_SRC_MAC_ADD_ARR_ERR:u32=2; pub const MAX_FCOE_COMPLETION_STATUS:u32=3;
pub const FCOE_TASK_DEV_TYPE_DISK:u32=0; pub const FCOE_TASK_DEV_TYPE_TAPE:u32=1; pub const MAX_FCOE_DEVICE_TYPE:u32=2;
pub const FCOE_INITIATOR_MODE:u32=0; pub const FCOE_TARGET_MODE:u32=1; pub const FCOE_BOTH_OR_NOT_CHOSEN:u32=3; pub const MAX_FCOE_MODE_TYPE:u32=4;
pub const SEND_FCOE_CMD:u32=0; pub const SEND_FCOE_MIDPATH:u32=1; pub const SEND_FCOE_ABTS_REQUEST:u32=2; pub const FCOE_EXCHANGE_CLEANUP:u32=3; pub const FCOE_SEQUENCE_RECOVERY:u32=4; pub const SEND_FCOE_XFER_RDY:u32=5; pub const SEND_FCOE_RSP:u32=6; pub const SEND_FCOE_RSP_WITH_SENSE_DATA:u32=7; pub const SEND_FCOE_TARGET_DATA:u32=8; pub const SEND_FCOE_INITIATOR_DATA:u32=9; pub const SEND_FCOE_XFER_CONTINUATION_RDY:u32=10; pub const SEND_FCOE_TARGET_ABTS_RSP:u32=11; pub const MAX_FCOE_SQE_REQUEST_TYPE:u32=12;
pub const FCOE_TASK_TYPE_WRITE_INITIATOR:u32=0; pub const FCOE_TASK_TYPE_READ_INITIATOR:u32=1; pub const FCOE_TASK_TYPE_MIDPATH:u32=2; pub const FCOE_TASK_TYPE_UNSOLICITED:u32=3; pub const FCOE_TASK_TYPE_ABTS:u32=4; pub const FCOE_TASK_TYPE_EXCHANGE_CLEANUP:u32=5; pub const FCOE_TASK_TYPE_SEQUENCE_CLEANUP:u32=6; pub const FCOE_TASK_TYPE_WRITE_TARGET:u32=7; pub const FCOE_TASK_TYPE_READ_TARGET:u32=8; pub const FCOE_TASK_TYPE_RSP:u32=9; pub const FCOE_TASK_TYPE_RSP_SENSE_DATA:u32=10; pub const FCOE_TASK_TYPE_ABTS_TARGET:u32=11; pub const FCOE_TASK_TYPE_ENUM_SIZE:u32=12; pub const MAX_FCOE_TASK_TYPE:u32=13;

#[repr(C)] #[derive(Copy,Clone)] pub struct fcoe_conn_offload_ramrod_data { pub sq_pbl_addr:regpair,pub sq_curr_page_addr:regpair,pub sq_next_page_addr:regpair,pub xferq_pbl_addr:regpair,pub xferq_curr_page_addr:regpair,pub xferq_next_page_addr:regpair,pub respq_pbl_addr:regpair,pub respq_curr_page_addr:regpair,pub respq_next_page_addr:regpair,pub dst_mac_addr_lo:u16,pub dst_mac_addr_mid:u16,pub dst_mac_addr_hi:u16,pub src_mac_addr_lo:u16,pub src_mac_addr_mid:u16,pub src_mac_addr_hi:u16,pub tx_max_fc_pay_len:u16,pub e_d_tov_timer_val:u16,pub rx_max_fc_pay_len:u16,pub vlan_tag:u16,pub physical_q0:u16,pub rec_rr_tov_timer_val:u16,pub s_id:fc_addr_nw,pub max_conc_seqs_c3:u8,pub d_id:fc_addr_nw,pub flags:u8,pub conn_id:u16,pub def_q_idx:u8,pub reserved:[u8;5] }
#[repr(C)] #[derive(Copy,Clone)] pub struct fcoe_init_func_ramrod_data { pub func_params:scsi_init_func_params,pub q_params:scsi_init_func_queues,pub mtu:u16,pub sq_num_pages_in_pbl:u16,pub reserved:[u32;3] }
#[repr(C)] #[derive(Copy,Clone)] pub struct fcoe_rx_stat { pub fcoe_rx_byte_cnt:regpair,pub fcoe_rx_data_pkt_cnt:regpair,pub fcoe_rx_xfer_pkt_cnt:regpair,pub fcoe_rx_other_pkt_cnt:regpair,pub fcoe_silent_drop_pkt_cmdq_full_cnt:u32,pub fcoe_silent_drop_pkt_rq_full_cnt:u32,pub fcoe_silent_drop_pkt_crc_error_cnt:u32,pub fcoe_silent_drop_pkt_task_invalid_cnt:u32,pub fcoe_silent_drop_total_pkt_cnt:u32,pub rsrv:u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
