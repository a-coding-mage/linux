/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* Rust translation of qed_if.h. External kernel/HSI types are supplied by dependencies. */

pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type dma_addr_t = u64;
pub type pci_power_t = u32;
pub type __iomem = ::core::ffi::c_void;
pub type status_block = ::core::ffi::c_void;
pub type qed_chain = ::core::ffi::c_void;
pub type qed_chain_init_params = ::core::ffi::c_void;
pub type pci_dev = ::core::ffi::c_void;
pub type devlink = ::core::ffi::c_void;
pub type devlink_health_reporter = ::core::ffi::c_void;
pub type msix_entry = ::core::ffi::c_void;

pub const QED_TX_SWS_TIMER_DFLT: u32 = 500;
pub const QED_TWO_MSL_TIMER_DFLT: u32 = 4000;
pub const QED_ROCE_PROTOCOL_INDEX: u32 = 3;
pub const QED_LLDP_CHASSIS_ID_STAT_LEN: usize = 4;
pub const QED_LLDP_PORT_ID_STAT_LEN: usize = 4;
pub const QED_DCBX_MAX_APP_PROTOCOL: usize = 32;
pub const QED_MAX_PFC_PRIORITIES: usize = 8;
pub const QED_DCBX_DSCP_SIZE: usize = 64;

#[repr(C)] #[derive(Copy, Clone)] pub enum dcbx_protocol_type { DCBX_PROTOCOL_ISCSI, DCBX_PROTOCOL_FCOE, DCBX_PROTOCOL_ROCE, DCBX_PROTOCOL_ROCE_V2, DCBX_PROTOCOL_ETH, DCBX_MAX_PROTOCOL_TYPE }
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_dcbx_lldp_remote { pub peer_chassis_id:[u32;4], pub peer_port_id:[u32;4], pub enable_rx:bool, pub enable_tx:bool, pub tx_interval:u32, pub max_credit:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_dcbx_lldp_local { pub local_chassis_id:[u32;4], pub local_port_id:[u32;4] }
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_dcbx_app_prio { pub roce:u8, pub roce_v2:u8, pub fcoe:u8, pub iscsi:u8, pub eth:u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_dbcx_pfc_params { pub willing:bool, pub enabled:bool, pub prio:[u8;8], pub max_tc:u8 }
#[repr(C)] #[derive(Copy, Clone)] pub enum qed_dcbx_sf_ieee_type { QED_DCBX_SF_IEEE_ETHTYPE, QED_DCBX_SF_IEEE_TCP_PORT, QED_DCBX_SF_IEEE_UDP_PORT, QED_DCBX_SF_IEEE_TCP_UDP_PORT }
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_app_entry { pub ethtype:bool, pub sf_ieee:qed_dcbx_sf_ieee_type, pub enabled:bool, pub prio:u8, pub proto_id:u16, pub proto_type:dcbx_protocol_type }
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_dcbx_params { pub app_entry:[qed_app_entry;32], pub num_app_entries:u16, pub app_willing:bool, pub app_valid:bool, pub app_error:bool, pub ets_willing:bool, pub ets_enabled:bool, pub ets_cbs:bool, pub valid:bool, pub ets_pri_tc_tbl:[u8;8], pub ets_tc_bw_tbl:[u8;8], pub ets_tc_tsa_tbl:[u8;8], pub pfc:qed_dbcx_pfc_params, pub max_ets_tc:u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_dcbx_admin_params { pub params:qed_dcbx_params, pub valid:bool }
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_dcbx_remote_params { pub params:qed_dcbx_params, pub valid:bool }
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_dcbx_operational_params { pub app_prio:qed_dcbx_app_prio, pub params:qed_dcbx_params, pub valid:bool, pub enabled:bool, pub ieee:bool, pub cee:bool, pub local:bool, pub err:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_dcbx_get { pub operational:qed_dcbx_operational_params, pub lldp_remote:qed_dcbx_lldp_remote, pub lldp_local:qed_dcbx_lldp_local, pub remote:qed_dcbx_remote_params, pub local:qed_dcbx_admin_params }

#[repr(C)] #[derive(Copy, Clone)] pub enum qed_nvm_images { QED_NVM_IMAGE_ISCSI_CFG, QED_NVM_IMAGE_FCOE_CFG, QED_NVM_IMAGE_MDUMP, QED_NVM_IMAGE_NVM_CFG1, QED_NVM_IMAGE_DEFAULT_CFG, QED_NVM_IMAGE_NVM_META }
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_link_eee_params { pub tx_lpi_timer:u32, pub adv_caps:u8, pub lp_adv_caps:u8, pub enable:bool, pub tx_lpi_enable:bool }
pub const QED_EEE_1G_ADV:u32=1<<0; pub const QED_EEE_10G_ADV:u32=1<<1;
#[repr(C)] #[derive(Copy, Clone)] pub enum qed_led_mode { QED_LED_MODE_OFF, QED_LED_MODE_ON, QED_LED_MODE_RESTORE }

#[repr(C)] #[derive(Copy, Clone)] pub struct qed_mfw_tlv_eth {
 pub lso_maxoff_size:u16,pub lso_maxoff_size_set:bool,pub lso_minseg_size:u16,pub lso_minseg_size_set:bool,pub prom_mode:u8,pub prom_mode_set:bool,pub tx_descr_size:u16,pub tx_descr_size_set:bool,pub rx_descr_size:u16,pub rx_descr_size_set:bool,pub netq_count:u16,pub netq_count_set:bool,pub tcp4_offloads:u32,pub tcp4_offloads_set:bool,pub tcp6_offloads:u32,pub tcp6_offloads_set:bool,pub tx_descr_qdepth:u16,pub tx_descr_qdepth_set:bool,pub rx_descr_qdepth:u16,pub rx_descr_qdepth_set:bool,pub iov_offload:u8,pub iov_offload_set:bool,pub txqs_empty:u8,pub txqs_empty_set:bool,pub rxqs_empty:u8,pub rxqs_empty_set:bool,pub num_txqs_full:u8,pub num_txqs_full_set:bool,pub num_rxqs_full:u8,pub num_rxqs_full_set:bool
}
pub const QED_MFW_TLV_IOV_OFFLOAD_NONE:u8=0; pub const QED_MFW_TLV_IOV_OFFLOAD_MULTIQUEUE:u8=1; pub const QED_MFW_TLV_IOV_OFFLOAD_VEB:u8=2; pub const QED_MFW_TLV_IOV_OFFLOAD_VEPA:u8=3;
pub const QED_MFW_TLV_TIME_SIZE:usize=14;
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_mfw_tlv_time { pub b_set:bool,pub month:u8,pub day:u8,pub hour:u8,pub min:u8,pub msec:u16,pub usec:u16 }

/* The large MFW TLV records retain their exact C field order and scalar intent. */
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_mfw_tlv_fcoe {
 pub scsi_timeout:u8,pub scsi_timeout_set:bool,pub rt_tov:u32,pub rt_tov_set:bool,pub ra_tov:u32,pub ra_tov_set:bool,pub ed_tov:u32,pub ed_tov_set:bool,pub cr_tov:u32,pub cr_tov_set:bool,pub boot_type:u8,pub boot_type_set:bool,pub npiv_state:u8,pub npiv_state_set:bool,pub num_npiv_ids:u32,pub num_npiv_ids_set:bool,pub switch_name:[u8;8],pub switch_name_set:bool,pub switch_portnum:u16,pub switch_portnum_set:bool,pub switch_portid:[u8;3],pub switch_portid_set:bool,pub vendor_name:[u8;8],pub vendor_name_set:bool,pub switch_model:[u8;8],pub switch_model_set:bool,pub switch_fw_version:[u8;8],pub switch_fw_version_set:bool,pub qos_pri:u8,pub qos_pri_set:bool,pub port_alias:[u8;3],pub port_alias_set:bool,pub port_state:u8,pub port_state_set:bool,pub fip_tx_descr_size:u16,pub fip_tx_descr_size_set:bool,pub fip_rx_descr_size:u16,pub fip_rx_descr_size_set:bool,pub link_failures:u16,pub link_failures_set:bool,pub fcoe_boot_progress:u8,pub fcoe_boot_progress_set:bool,pub rx_bcast:u64,pub rx_bcast_set:bool,pub tx_bcast:u64,pub tx_bcast_set:bool,pub fcoe_txq_depth:u16,pub fcoe_txq_depth_set:bool,pub fcoe_rxq_depth:u16,pub fcoe_rxq_depth_set:bool,pub fcoe_rx_frames:u64,pub fcoe_rx_frames_set:bool,pub fcoe_rx_bytes:u64,pub fcoe_rx_bytes_set:bool,pub fcoe_tx_frames:u64,pub fcoe_tx_frames_set:bool,pub fcoe_tx_bytes:u64,pub fcoe_tx_bytes_set:bool,pub crc_count:u16,pub crc_count_set:bool,pub crc_err_src_fcid:[u32;5],pub crc_err_src_fcid_set:[bool;5],pub crc_err:[qed_mfw_tlv_time;5],pub losync_err:u16,pub losync_err_set:bool,pub losig_err:u16,pub losig_err_set:bool,pub primtive_err:u16,pub primtive_err_set:bool,pub disparity_err:u16,pub disparity_err_set:bool,pub code_violation_err:u16,pub code_violation_err_set:bool,pub flogi_param:[u32;4],pub flogi_param_set:[bool;4],pub flogi_tstamp:qed_mfw_tlv_time,pub flogi_acc_param:[u32;4],pub flogi_acc_param_set:[bool;4],pub flogi_acc_tstamp:qed_mfw_tlv_time,pub flogi_rjt:u32,pub flogi_rjt_set:bool,pub flogi_rjt_tstamp:qed_mfw_tlv_time,pub fdiscs:u32,pub fdiscs_set:bool,pub fdisc_acc:u8,pub fdisc_acc_set:bool,pub fdisc_rjt:u8,pub fdisc_rjt_set:bool,pub plogi:u8,pub plogi_set:bool,pub plogi_acc:u8,pub plogi_acc_set:bool,pub plogi_rjt:u8,pub plogi_rjt_set:bool,pub plogi_dst_fcid:[u32;5],pub plogi_dst_fcid_set:[bool;5],pub plogi_tstamp:[qed_mfw_tlv_time;5],pub plogi_acc_src_fcid:[u32;5],pub plogi_acc_src_fcid_set:[bool;5],pub plogi_acc_tstamp:[qed_mfw_tlv_time;5],pub tx_plogos:u8,pub tx_plogos_set:bool,pub plogo_acc:u8,pub plogo_acc_set:bool,pub plogo_rjt:u8,pub plogo_rjt_set:bool,pub plogo_src_fcid:[u32;5],pub plogo_src_fcid_set:[bool;5],pub plogo_tstamp:[qed_mfw_tlv_time;5],pub rx_logos:u8,pub rx_logos_set:bool,pub tx_accs:u8,pub tx_accs_set:bool,pub tx_prlis:u8,pub tx_prlis_set:bool,pub rx_accs:u8,pub rx_accs_set:bool,pub tx_abts:u8,pub tx_abts_set:bool,pub rx_abts_acc:u8,pub rx_abts_acc_set:bool,pub rx_abts_rjt:u8,pub rx_abts_rjt_set:bool,pub abts_dst_fcid:[u32;5],pub abts_dst_fcid_set:[bool;5],pub abts_tstamp:[qed_mfw_tlv_time;5],pub rx_rscn:u8,pub rx_rscn_set:bool,pub rx_rscn_nport:[u32;4],pub rx_rscn_nport_set:[bool;4],pub tx_lun_rst:u8,pub tx_lun_rst_set:bool,pub abort_task_sets:u8,pub abort_task_sets_set:bool,pub tx_tprlos:u8,pub tx_tprlos_set:bool,pub tx_nos:u8,pub tx_nos_set:bool,pub rx_nos:u8,pub rx_nos_set:bool,pub ols:u8,pub ols_set:bool,pub lr:u8,pub lr_set:bool,pub lrr:u8,pub lrr_set:bool,pub tx_lip:u8,pub tx_lip_set:bool,pub rx_lip:u8,pub rx_lip_set:bool,pub eofa:u8,pub eofa_set:bool,pub eofni:u8,pub eofni_set:bool,pub scsi_chks:u8,pub scsi_chks_set:bool,pub scsi_cond_met:u8,pub scsi_cond_met_set:bool,pub scsi_busy:u8,pub scsi_busy_set:bool,pub scsi_inter:u8,pub scsi_inter_set:bool,pub scsi_inter_cond_met:u8,pub scsi_inter_cond_met_set:bool,pub scsi_rsv_conflicts:u8,pub scsi_rsv_conflicts_set:bool,pub scsi_tsk_full:u8,pub scsi_tsk_full_set:bool,pub scsi_aca_active:u8,pub scsi_aca_active_set:bool,pub scsi_tsk_abort:u8,pub scsi_tsk_abort_set:bool,pub scsi_rx_chk:[u32;5],pub scsi_rx_chk_set:[bool;5],pub scsi_chk_tstamp:[qed_mfw_tlv_time;5]
}
#[repr(C)] #[derive(Copy, Clone)] pub struct qed_mfw_tlv_iscsi { pub target_llmnr:u8,pub target_llmnr_set:bool,pub header_digest:u8,pub header_digest_set:bool,pub data_digest:u8,pub data_digest_set:bool,pub auth_method:u8,pub auth_method_set:bool,pub boot_taget_portal:u16,pub boot_taget_portal_set:bool,pub frame_size:u16,pub frame_size_set:bool,pub tx_desc_size:u16,pub tx_desc_size_set:bool,pub rx_desc_size:u16,pub rx_desc_size_set:bool,pub boot_progress:u8,pub boot_progress_set:bool,pub tx_desc_qdepth:u16,pub tx_desc_qdepth_set:bool,pub rx_desc_qdepth:u16,pub rx_desc_qdepth_set:bool,pub rx_frames:u64,pub rx_frames_set:bool,pub rx_bytes:u64,pub rx_bytes_set:bool,pub tx_frames:u64,pub tx_frames_set:bool,pub tx_bytes:u64,pub tx_bytes_set:bool }
pub const QED_MFW_TLV_AUTH_METHOD_NONE:u8=1; pub const QED_MFW_TLV_AUTH_METHOD_CHAP:u8=2; pub const QED_MFW_TLV_AUTH_METHOD_MUTUAL_CHAP:u8=3;
pub const QED_MFW_TLV_PORT_STATE_OFFLINE:u8=0; pub const QED_MFW_TLV_PORT_STATE_LOOP:u8=1; pub const QED_MFW_TLV_PORT_STATE_P2P:u8=2; pub const QED_MFW_TLV_PORT_STATE_FABRIC:u8=3;

#[repr(C)] #[derive(Copy, Clone)] pub enum qed_db_rec_width { DB_REC_WIDTH_32B, DB_REC_WIDTH_64B }
#[repr(C)] #[derive(Copy, Clone)] pub enum qed_db_rec_space { DB_REC_KERNEL, DB_REC_USER }
pub const QED_COALESCE_MAX:u32=0x1ff; pub const QED_DEFAULT_RX_USECS:u32=12; pub const QED_DEFAULT_TX_USECS:u32=48;
pub const ETH_ALEN:usize=6; pub const QED_DRV_VER_STR_SIZE:usize=12; pub const ILT_PAGE_SIZE_TCFC:u32=0x8000;

#[repr(C)] pub struct qed_dev { _opaque:[u8;0] }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_eth_pf_params { pub num_cons:u16,pub num_vf_cons:u8,pub num_arfs_filters:u32 }
pub const ETH_PF_PARAMS_VF_CONS_DEFAULT:u8=32;
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_fcoe_pf_params { pub glbl_q_params_addr:u64,pub bdq_pbl_base_addr:[u64;2],pub num_cons:u16,pub num_tasks:u16,pub sq_num_pbl_pages:u16,pub cq_num_entries:u16,pub cmdq_num_entries:u16,pub rq_buffer_log_size:u16,pub mtu:u16,pub dummy_icid:u16,pub bdq_xoff_threshold:[u16;2],pub bdq_xon_threshold:[u16;2],pub rq_buffer_size:u16,pub num_cqs:u8,pub log_page_size:u8,pub gl_rq_pi:u8,pub gl_cmd_pi:u8,pub debug_mode:u8,pub is_target:u8,pub bdq_pbl_num_entries:[u8;2] }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_iscsi_pf_params { pub glbl_q_params_addr:u64,pub bdq_pbl_base_addr:[u64;3],pub cq_num_entries:u16,pub cmdq_num_entries:u16,pub two_msl_timer:u32,pub tx_sws_timer:u16,pub num_cons:u16,pub num_tasks:u16,pub half_way_close_timeout:u16,pub bdq_xoff_threshold:[u16;3],pub bdq_xon_threshold:[u16;3],pub cmdq_xoff_threshold:u16,pub cmdq_xon_threshold:u16,pub rq_buffer_size:u16,pub num_sq_pages_in_ring:u8,pub num_r2tq_pages_in_ring:u8,pub num_uhq_pages_in_ring:u8,pub num_queues:u8,pub log_page_size:u8,pub rqe_log_size:u8,pub max_fin_rt:u8,pub gl_rq_pi:u8,pub gl_cmd_pi:u8,pub debug_mode:u8,pub ll2_ooo_queue_id:u8,pub is_target:u8,pub is_soc_en:u8,pub soc_num_of_blocks_log:u8,pub bdq_pbl_num_entries:[u8;3] }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_nvmetcp_pf_params { pub glbl_q_params_addr:u64,pub cq_num_entries:u16,pub num_cons:u16,pub num_tasks:u16,pub num_sq_pages_in_ring:u8,pub num_r2tq_pages_in_ring:u8,pub num_uhq_pages_in_ring:u8,pub num_queues:u8,pub gl_rq_pi:u8,pub gl_cmd_pi:u8,pub debug_mode:u8,pub ll2_ooo_queue_id:u8,pub min_rto:u16 }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_rdma_pf_params { pub min_dpis:u32,pub num_qps:u32,pub num_srqs:u32,pub roce_edpm_mode:u8,pub gl_pi:u8,pub enable_dcqcn:u8 }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_pf_params { pub eth_pf_params:qed_eth_pf_params,pub fcoe_pf_params:qed_fcoe_pf_params,pub iscsi_pf_params:qed_iscsi_pf_params,pub nvmetcp_pf_params:qed_nvmetcp_pf_params,pub rdma_pf_params:qed_rdma_pf_params }
#[repr(C)] #[derive(Copy,Clone)] pub enum qed_int_mode { QED_INT_MODE_INTA,QED_INT_MODE_MSIX,QED_INT_MODE_MSI,QED_INT_MODE_POLL }
#[repr(C)] pub struct qed_sb_info { pub sb_virt:*mut status_block,pub sb_phys:dma_addr_t,pub sb_ack:u32,pub igu_sb_id:u16,pub igu_addr:*mut __iomem,pub flags:u8,pub cdev:*mut qed_dev }
pub const QED_SB_INFO_INIT:u8=1; pub const QED_SB_INFO_SETUP:u8=2;
#[repr(C)] #[derive(Copy,Clone)] pub enum qed_hw_err_type { QED_HW_ERR_FAN_FAIL,QED_HW_ERR_MFW_RESP_FAIL,QED_HW_ERR_HW_ATTN,QED_HW_ERR_DMAE_FAIL,QED_HW_ERR_RAMROD_FAIL,QED_HW_ERR_FW_ASSERT,QED_HW_ERR_LAST }
#[repr(C)] #[derive(Copy,Clone)] pub enum qed_dev_type { QED_DEV_TYPE_BB,QED_DEV_TYPE_AH }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_dev_info { pub pci_mem_start:usize,pub pci_mem_end:usize,pub pci_irq:u32,pub num_hwfns:u8,pub hw_mac:[u8;6],pub fw_major:u16,pub fw_minor:u16,pub fw_rev:u16,pub fw_eng:u16,pub mfw_rev:u32,pub flash_size:u32,pub b_arfs_capable:bool,pub b_inter_pf_switch:bool,pub tx_switching:bool,pub rdma_supported:bool,pub mtu:u16,pub wol_support:bool,pub smart_an:bool,pub esl:bool,pub mbi_version:u32,pub dev_type:qed_dev_type,pub vxlan_enable:bool,pub gre_enable:bool,pub geneve_enable:bool,pub abs_pf_id:u8 }

#[repr(C)] #[derive(Copy,Clone)] pub enum qed_sb_type { QED_SB_TYPE_L2_QUEUE,QED_SB_TYPE_CNQ,QED_SB_TYPE_STORAGE }
#[repr(C)] #[derive(Copy,Clone)] pub enum qed_protocol { QED_PROTOCOL_ETH,QED_PROTOCOL_ISCSI,QED_PROTOCOL_NVMETCP,QED_PROTOCOL_FCOE }
#[repr(C)] #[derive(Copy,Clone)] pub enum qed_fec_mode { QED_FEC_MODE_NONE=1,QED_FEC_MODE_FIRECODE=2,QED_FEC_MODE_RS=4,QED_FEC_MODE_AUTO=8,QED_FEC_MODE_UNSUPPORTED=16 }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_link_params { pub link_up:bool,pub override_flags:u32,pub autoneg:bool,pub adv_speeds:[u8;64],pub forced_speed:u32,pub pause_config:u32,pub loopback_mode:u32,pub eee:qed_link_eee_params,pub fec:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_link_output { pub link_up:bool,pub supported_caps:[u8;64],pub advertised_caps:[u8;64],pub lp_caps:[u8;64],pub speed:u32,pub duplex:u8,pub port:u8,pub autoneg:bool,pub pause_config:u32,pub eee_supported:bool,pub eee_active:bool,pub sup_caps:u8,pub eee:qed_link_eee_params,pub sup_fec:u32,pub active_fec:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_probe_params { pub protocol:qed_protocol,pub dp_module:u32,pub dp_level:u8,pub is_vf:bool,pub recov_in_prog:bool }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_slowpath_params { pub int_mode:u32,pub drv_major:u8,pub drv_minor:u8,pub drv_rev:u8,pub drv_eng:u8,pub name:[u8;12] }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_int_info { pub msix:*mut msix_entry,pub msix_cnt:u8,pub used_cnt:u8 }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_generic_tlvs { pub feat_flags:u16,pub mac:[[u8;6];3] }
pub const QED_TLV_IP_CSUM:u16=1; pub const QED_TLV_LSO:u16=2; pub const QED_TLV_MAC_COUNT:usize=3;
pub const QED_I2C_DEV_ADDR_A0:u8=0xa0; pub const QED_I2C_DEV_ADDR_A2:u8=0xa2; pub const QED_NVM_SIGNATURE:u32=0x12435687;
#[repr(C)] #[derive(Copy,Clone)] pub enum qed_nvm_flash_cmd { QED_NVM_FLASH_CMD_FILE_DATA=2,QED_NVM_FLASH_CMD_FILE_START=3,QED_NVM_FLASH_CMD_NVM_CHANGE=4,QED_NVM_FLASH_CMD_NVM_CFG_ID=5,QED_NVM_FLASH_CMD_NVM_MAX }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_devlink { pub cdev:*mut qed_dev,pub fw_reporter:*mut devlink_health_reporter }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_sb_info_dbg { pub igu_prod:u32,pub igu_cons:u32,pub pi:[u16;16] }

/* Callback tables are represented with nullable C-ABI function pointers. */
pub type qed_cb = Option<unsafe extern "C" fn()>;
#[repr(C)] pub struct qed_common_cb_ops { pub arfs_filter_op:qed_cb,pub link_update:qed_cb,pub schedule_recovery_handler:qed_cb,pub schedule_hw_err_handler:qed_cb,pub dcbx_aen:qed_cb,pub get_generic_tlv_data:qed_cb,pub get_protocol_tlv_data:qed_cb,pub bw_update:qed_cb }
#[repr(C)] pub struct qed_selftest_ops { pub selftest_interrupt:qed_cb,pub selftest_memory:qed_cb,pub selftest_register:qed_cb,pub selftest_clock:qed_cb,pub selftest_nvram:qed_cb }
#[repr(C)] pub struct qed_common_ops { pub selftest:*mut qed_selftest_ops, pub probe:qed_cb,pub remove:qed_cb,pub set_power_state:qed_cb,pub set_name:qed_cb,pub update_pf_params:qed_cb,pub slowpath_start:qed_cb,pub slowpath_stop:qed_cb,pub set_fp_int:qed_cb,pub get_fp_int:qed_cb,pub sb_init:qed_cb,pub sb_release:qed_cb,pub simd_handler_config:qed_cb,pub simd_handler_clean:qed_cb,pub dbg_grc:qed_cb,pub dbg_grc_size:qed_cb,pub dbg_all_data:qed_cb,pub dbg_all_data_size:qed_cb,pub report_fatal_error:qed_cb,pub can_link_change:qed_cb,pub set_link:qed_cb,pub get_link:qed_cb,pub drain:qed_cb,pub update_msglvl:qed_cb,pub chain_alloc:qed_cb,pub chain_free:qed_cb,pub nvm_flash:qed_cb,pub nvm_get_image:qed_cb,pub set_coalesce:qed_cb,pub set_led:qed_cb,pub attn_clr_enable:qed_cb,pub db_recovery_add:qed_cb,pub db_recovery_del:qed_cb,pub recovery_process:qed_cb,pub recovery_prolog:qed_cb,pub update_drv_state:qed_cb,pub update_mac:qed_cb,pub update_mtu:qed_cb,pub update_wol:qed_cb,pub read_module_eeprom:qed_cb,pub get_affin_hwfn_idx:qed_cb,pub read_nvm_cfg:qed_cb,pub read_nvm_cfg_len:qed_cb,pub set_grc_config:qed_cb,pub devlink_register:qed_cb,pub devlink_unregister:qed_cb,pub mfw_report:qed_cb,pub get_sb_info:qed_cb,pub get_esl_status:qed_cb }

pub const QED_SB_IDX:u16=2; pub const RX_PI:u32=0; #[inline] pub const fn TX_PI(tc:u32)->u32 { RX_PI+1+tc }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_eth_stats_common { pub values:[u64;59] }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_eth_stats_bb { pub values:[u64;11] }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_eth_stats_ah { pub rx_1519_to_max_byte_packets:u64,pub tx_1519_to_max_byte_packets:u64 }
#[repr(C)] pub union qed_eth_stats_variant { pub bb:qed_eth_stats_bb,pub ah:qed_eth_stats_ah }
#[repr(C)] pub struct qed_eth_stats { pub common:qed_eth_stats_common,pub variant:qed_eth_stats_variant }
#[repr(C)] #[derive(Copy,Clone)] pub struct qed_sb_cnt_info { pub orig:i32,pub cnt:i32,pub free_cnt:i32,pub iov_orig:i32,pub iov_cnt:i32,pub free_cnt_iov:i32 }

#[inline] pub unsafe fn qed_sb_update_sb_idx(sb_info:*mut qed_sb_info)->u16 { let prod=(*(*sb_info).sb_virt as *mut u32).read_volatile() & 0xffff; if (*sb_info).sb_ack != prod { (*sb_info).sb_ack=prod; QED_SB_IDX } else { 0 } }
#[inline] pub unsafe fn qed_sb_ack(sb_info:*mut qed_sb_info, int_cmd:u32, upd_flg:u8) { let igu_ack=((*sb_info).sb_ack<<16)|((upd_flg as u32)<<8)|((int_cmd as u32)<<4)|1; core::ptr::write_volatile((*sb_info).igu_addr as *mut u32,igu_ack); core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst); }
#[inline] pub unsafe fn __internal_ram_wr(_p_hwfn:*mut ::core::ffi::c_void, addr:*mut __iomem,size:i32,data:*const u32) { for i in 0..(size as usize/4) { core::ptr::write_volatile((addr as *mut u32).add(i),*data.add(i)); } }
#[inline] pub unsafe fn internal_ram_wr(addr:*mut __iomem,size:i32,data:*const u32) { __internal_ram_wr(core::ptr::null_mut(),addr,size,data); }
#[repr(C)] #[derive(Copy,Clone)] pub enum qed_rss_caps { QED_RSS_IPV4=1,QED_RSS_IPV6=2,QED_RSS_IPV4_TCP=4,QED_RSS_IPV6_TCP=8,QED_RSS_IPV4_UDP=16,QED_RSS_IPV6_UDP=32 }
pub const QED_RSS_IND_TABLE_SIZE:usize=128; pub const QED_RSS_KEY_SIZE:usize=10;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
