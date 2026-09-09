/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from target_core_base.h. Kernel types and functions are external dependencies. */

pub const TARGET_CORE_VERSION: &str = "v5.0";
pub const TCM_MAX_COMMAND_SIZE: usize = 32;
pub const TRANSPORT_SENSE_BUFFER: usize = 96;
pub const SPC_SENSE_KEY_OFFSET: usize = 2;
pub const SPC_ADD_SENSE_LEN_OFFSET: usize = 7;
pub const SPC_DESC_TYPE_OFFSET: usize = 8;
pub const SPC_ADDITIONAL_DESC_LEN_OFFSET: usize = 9;
pub const SPC_VALIDITY_OFFSET: usize = 10;
pub const SPC_ASC_KEY_OFFSET: usize = 12;
pub const SPC_ASCQ_KEY_OFFSET: usize = 13;
pub const TRANSPORT_IQN_LEN: usize = 224;
pub const LU_GROUP_NAME_BUF: usize = 256;
pub const TG_PT_GROUP_NAME_BUF: usize = 256;
pub const VPD_TMP_BUF_SIZE: usize = 254;
pub const READ_BLOCK_LEN: usize = 6;
pub const READ_CAP_LEN: usize = 8;
pub const READ_POSITION_LEN: usize = 20;
pub const INQUIRY_LEN: usize = 36;
pub const INQUIRY_VPD_SERIAL_LEN: usize = 254;
pub const INQUIRY_VPD_DEVICE_IDENTIFIER_LEN: usize = 254;
pub const PYX_TRANSPORT_WINDOW_CLOSED_THRESHOLD: i32 = 3;
pub const PYX_TRANSPORT_WINDOW_CLOSED_WAIT_SHORT: i32 = 3;
pub const PYX_TRANSPORT_WINDOW_CLOSED_WAIT_LONG: i32 = 10;
pub const PYX_TRANSPORT_STATUS_INTERVAL: i32 = 5;
pub const DA_MAX_UNMAP_LBA_COUNT: i32 = 0;
pub const DA_MAX_UNMAP_BLOCK_DESC_COUNT: i32 = 0;
pub const DA_UNMAP_GRANULARITY_DEFAULT: i32 = 0;
pub const DA_UNMAP_GRANULARITY_ALIGNMENT_DEFAULT: i32 = 0;
pub const DA_UNMAP_ZEROES_DATA_DEFAULT: i32 = 0;
pub const DA_MAX_WRITE_SAME_LEN: i32 = 0;
pub const DA_EMULATE_MODEL_ALIAS: i32 = 0;
pub const DA_EMULATE_WRITE_CACHE: i32 = 0;
pub const DA_EMULATE_TAS: i32 = 1;
pub const DA_EMULATE_TPU: i32 = 0;
pub const DA_EMULATE_TPWS: i32 = 0;
pub const DA_EMULATE_CAW: i32 = 1;
pub const DA_EMULATE_3PC: i32 = 1;
pub const DA_EMULATE_ALUA: i32 = 0;
pub const DA_EMULATE_PR: i32 = 1;
pub const DA_EMULATE_RSOC: i32 = 1;
pub const DA_ENFORCE_PR_ISIDS: i32 = 1;
pub const DA_FORCE_PR_APTPL: i32 = 0;
pub const DA_STATUS_MAX_SECTORS_MIN: i32 = 16;
pub const DA_STATUS_MAX_SECTORS_MAX: i32 = 8192;
pub const DA_IS_NONROT: i32 = 0;
pub const DA_EMULATE_REST_REORD: i32 = 0;
pub const SE_INQUIRY_BUF: usize = 1024;
pub const SE_MODE_PAGE_BUF: usize = 512;
pub const SE_SENSE_BUF: usize = 96;
pub const PD_TEXT_ID_INFO_LEN: usize = 256;

#[repr(C)] pub enum target_compl_type { TARGET_FABRIC_DEFAULT_COMPL, TARGET_DIRECT_COMPL, TARGET_QUEUE_COMPL }
#[repr(C)] pub enum target_submit_type { TARGET_FABRIC_DEFAULT_SUBMIT, TARGET_DIRECT_SUBMIT, TARGET_QUEUE_SUBMIT }
#[repr(C)] pub enum hba_flags_table { HBA_FLAGS_INTERNAL_USE = 1, HBA_FLAGS_PSCSI_MODE = 2 }
#[repr(C)] pub enum transport_state_table { TRANSPORT_NO_STATE=0, TRANSPORT_NEW_CMD=1, TRANSPORT_WRITE_PENDING=3, TRANSPORT_PROCESSING=5, TRANSPORT_COMPLETE=6, TRANSPORT_ISTATE_PROCESSING=11, TRANSPORT_COMPLETE_QF_WP=18, TRANSPORT_COMPLETE_QF_OK=19, TRANSPORT_COMPLETE_QF_ERR=20 }
#[repr(C)] pub enum se_cmd_flags_table {
    SCF_SUPPORTED_SAM_OPCODE=1<<0, SCF_TRANSPORT_TASK_SENSE=1<<1, SCF_EMULATED_TASK_SENSE=1<<2, SCF_SCSI_DATA_CDB=1<<3, SCF_SCSI_TMR_CDB=1<<4, SCF_FUA=1<<5, SCF_SE_LUN_CMD=1<<6, SCF_BIDI=1<<7, SCF_SENT_CHECK_CONDITION=1<<8, SCF_OVERFLOW_BIT=1<<9, SCF_UNDERFLOW_BIT=1<<10, SCF_ALUA_NON_OPTIMIZED=1<<11, SCF_PASSTHROUGH_SG_TO_MEM_NOALLOC=1<<12, SCF_COMPARE_AND_WRITE=1<<13, SCF_PASSTHROUGH_PROT_SG_TO_MEM_NOALLOC=1<<14, SCF_ACK_KREF=1<<15, SCF_USE_CPUID=1<<16, SCF_TASK_ATTR_SET=1<<17, SCF_TREAT_READ_AS_NORMAL=1<<18, SCF_TASK_ORDERED_SYNC=1<<19, SCF_ATOMIC=1<<20,
}
pub type sense_reason_t = u32;
#[repr(C)] pub enum tcm_sense_reason_table { TCM_NO_SENSE=0, TCM_NON_EXISTENT_LUN=1, TCM_UNSUPPORTED_SCSI_OPCODE=2, TCM_INCORRECT_AMOUNT_OF_DATA=3, TCM_UNEXPECTED_UNSOLICITED_DATA=4, TCM_SERVICE_CRC_ERROR=5, TCM_SNACK_REJECTED=6, TCM_SECTOR_COUNT_TOO_MANY=7, TCM_INVALID_CDB_FIELD=8, TCM_INVALID_PARAMETER_LIST=9, TCM_LOGICAL_UNIT_COMMUNICATION_FAILURE=0x0a, TCM_UNKNOWN_MODE_PAGE=0x0b, TCM_WRITE_PROTECTED=0x0c, TCM_CHECK_CONDITION_ABORT_CMD=0x0d, TCM_CHECK_CONDITION_UNIT_ATTENTION=0x0e, TCM_RESERVATION_CONFLICT=0x10, TCM_ADDRESS_OUT_OF_RANGE=0x11, TCM_OUT_OF_RESOURCES=0x12, TCM_PARAMETER_LIST_LENGTH_ERROR=0x13, TCM_MISCOMPARE_VERIFY=0x14, TCM_LOGICAL_BLOCK_GUARD_CHECK_FAILED=0x15, TCM_LOGICAL_BLOCK_APP_TAG_CHECK_FAILED=0x16, TCM_LOGICAL_BLOCK_REF_TAG_CHECK_FAILED=0x17, TCM_COPY_TARGET_DEVICE_NOT_REACHABLE=0x18, TCM_TOO_MANY_TARGET_DESCS=0x19, TCM_UNSUPPORTED_TARGET_DESC_TYPE_CODE=0x1a, TCM_TOO_MANY_SEGMENT_DESCS=0x1b, TCM_UNSUPPORTED_SEGMENT_DESC_TYPE_CODE=0x1c, TCM_INSUFFICIENT_REGISTRATION_RESOURCES=0x1d, TCM_LUN_BUSY=0x1e, TCM_INVALID_FIELD_IN_COMMAND_IU=0x1f, TCM_ALUA_TG_PT_STANDBY=0x20, TCM_ALUA_TG_PT_UNAVAILABLE=0x21, TCM_ALUA_STATE_TRANSITION=0x22, TCM_ALUA_OFFLINE=0x23 }
#[repr(C)] pub enum target_sc_flags_table { TARGET_SCF_BIDI_OP=1, TARGET_SCF_ACK_KREF=2, TARGET_SCF_UNKNOWN_SIZE=4, TARGET_SCF_USE_CPUID=8 }
#[repr(C)] pub enum tcm_tmreq_table { TMR_ABORT_TASK=1, TMR_ABORT_TASK_SET=2, TMR_CLEAR_ACA=3, TMR_CLEAR_TASK_SET=4, TMR_LUN_RESET=5, TMR_TARGET_WARM_RESET=6, TMR_TARGET_COLD_RESET=7, TMR_LUN_RESET_PRO=0x80, TMR_UNKNOWN=0xff }
#[repr(C)] pub enum tcm_tmrsp_table { TMR_FUNCTION_FAILED=0, TMR_FUNCTION_COMPLETE=1, TMR_TASK_DOES_NOT_EXIST=2, TMR_LUN_DOES_NOT_EXIST=3, TMR_TASK_MGMT_FUNCTION_NOT_SUPPORTED=4, TMR_FUNCTION_REJECTED=5 }
pub type scsi_index_t = u32;
pub const SCSI_INST_INDEX: scsi_index_t = 0; pub const SCSI_AUTH_INTR_INDEX: scsi_index_t = 1; pub const SCSI_INDEX_TYPE_MAX: scsi_index_t = 2;

/* External kernel structures referenced by this header. */
#[allow(non_camel_case_types)] pub type u8 = ::core::ffi::c_uchar; pub type u16 = ::core::ffi::c_ushort; pub type u32 = ::core::ffi::c_uint; pub type u64 = ::core::ffi::c_ulonglong;
extern "C" { pub fn container_of<T>(ptr: *mut config_item, member: *mut T) -> *mut T; pub fn to_config_group(item: *mut config_item) -> *mut config_group; pub fn smp_mb__before_atomic(); pub fn smp_mb__after_atomic(); pub fn atomic_inc(v: *mut atomic_t); pub fn atomic_dec(v: *mut atomic_t); pub fn sbitmap_queue_clear(q: *mut sbitmap_queue, tag: u32, cpu: i32); }
#[repr(C)] pub struct config_item { _private: [u8;0] } #[repr(C)] pub struct config_group { _private: [u8;0] } #[repr(C)] pub struct list_head { _private: [u8;0] } #[repr(C)] pub struct hlist_head { _private: [u8;0] } #[repr(C)] pub struct hlist_node { _private: [u8;0] } #[repr(C)] pub struct rcu_head { _private: [u8;0] } #[repr(C)] pub struct spinlock_t { _private: [u8;0] } #[repr(C)] pub struct mutex { _private: [u8;0] } #[repr(C)] pub struct atomic_t { _private: [u8;0] } #[repr(C)] pub struct atomic_long_t { _private: [u8;0] } #[repr(C)] pub struct completion { _private: [u8;0] } #[repr(C)] pub struct semaphore { _private: [u8;0] } #[repr(C)] pub struct work_struct { _private: [u8;0] } #[repr(C)] pub struct kref { _private: [u8;0] } #[repr(C)] pub struct llist_node { _private: [u8;0] } #[repr(C)] pub struct llist_head { _private: [u8;0] } #[repr(C)] pub struct scatterlist { _private: [u8;0] } #[repr(C)] pub struct percpu_ref { _private: [u8;0] } #[repr(C)] pub struct wait_queue_head_t { _private: [u8;0] } #[repr(C)] pub struct sbitmap_queue { _private: [u8;0] }
#[repr(C)] pub enum dma_data_direction { DMA_BIDIRECTIONAL, DMA_TO_DEVICE, DMA_FROM_DEVICE, DMA_NONE }
pub enum se_device {} pub enum se_lun {} pub enum se_session {} pub enum se_node_acl {} pub enum se_portal_group {} pub enum se_hba {} pub enum se_dev_entry {} pub enum target_backend {} pub enum target_fabric_configfs {} pub enum target_core_fabric_ops {} pub enum target_backend_ops {} pub enum target_opcode_descriptor {}

#[repr(C)] pub struct t10_alua_lba_map_member { pub lba_map_mem_list:list_head, pub lba_map_mem_alua_state:i32, pub lba_map_mem_alua_pg_id:i32 }
#[repr(C)] pub struct t10_alua_lba_map { pub lba_map_first_lba:u64, pub lba_map_last_lba:u64, pub lba_map_list:list_head, pub lba_map_mem_list:list_head }
#[repr(C)] pub struct t10_alua { pub alua_tg_pt_gps_counter:u16, pub alua_tg_pt_gps_count:u32, pub lba_map_lock:spinlock_t, pub lba_map_segment_size:u32, pub lba_map_segment_multiplier:u32, pub lba_map_list:list_head, pub tg_pt_gps_lock:spinlock_t, pub t10_dev:*mut se_device, pub default_tg_pt_gp:*mut t10_alua_tg_pt_gp, pub alua_tg_pt_gps_group:config_group, pub tg_pt_gps_list:list_head }
#[repr(C)] pub struct t10_alua_lu_gp { pub lu_gp_id:u16, pub lu_gp_valid_id:i32, pub lu_gp_members:u32, pub lu_gp_ref_cnt:atomic_t, pub lu_gp_lock:spinlock_t, pub lu_gp_group:config_group, pub lu_gp_node:list_head, pub lu_gp_mem_list:list_head }
#[repr(C)] pub struct t10_alua_lu_gp_member { pub lu_gp_assoc:bool, pub lu_gp_mem_ref_cnt:atomic_t, pub lu_gp_mem_lock:spinlock_t, pub lu_gp:*mut t10_alua_lu_gp, pub lu_gp_mem_dev:*mut se_device, pub lu_gp_mem_list:list_head }
#[repr(C)] pub struct t10_alua_tg_pt_gp { pub tg_pt_gp_id:u16, pub tg_pt_gp_valid_id:i32, pub tg_pt_gp_alua_supported_states:i32, pub tg_pt_gp_alua_access_status:i32, pub tg_pt_gp_alua_access_type:i32, pub tg_pt_gp_nonop_delay_msecs:i32, pub tg_pt_gp_trans_delay_msecs:i32, pub tg_pt_gp_implicit_trans_secs:i32, pub tg_pt_gp_pref:i32, pub tg_pt_gp_write_metadata:i32, pub tg_pt_gp_members:u32, pub tg_pt_gp_alua_access_state:i32, pub tg_pt_gp_ref_cnt:atomic_t, pub tg_pt_gp_lock:spinlock_t, pub tg_pt_gp_transition_mutex:mutex, pub tg_pt_gp_dev:*mut se_device, pub tg_pt_gp_group:config_group, pub tg_pt_gp_list:list_head, pub tg_pt_gp_lun_list:list_head, pub tg_pt_gp_alua_lun:*mut se_lun, pub tg_pt_gp_alua_nacl:*mut se_node_acl }
#[repr(C)] pub struct t10_vpd { pub device_identifier:[u8;INQUIRY_VPD_DEVICE_IDENTIFIER_LEN], pub protocol_identifier_set:i32, pub protocol_identifier:u32, pub device_identifier_code_set:u32, pub association:u32, pub device_identifier_type:u32, pub vpd_list:list_head }
#[repr(C)] pub struct t10_wwn { pub vendor:[i8;INQUIRY_VENDOR_LEN], pub model:[i8;INQUIRY_MODEL_LEN], pub revision:[i8;INQUIRY_REVISION_LEN], pub unit_serial:[i8;INQUIRY_VPD_SERIAL_LEN], pub company_id:u32, pub t10_vpd_lock:spinlock_t, pub t10_dev:*mut se_device, pub t10_wwn_group:config_group, pub t10_vpd_list:list_head, pub pd_text_id_info:[i8;PD_TEXT_ID_INFO_LEN] }

#[repr(C)] pub enum target_prot_op { TARGET_PROT_NORMAL=0, TARGET_PROT_DIN_INSERT=1, TARGET_PROT_DOUT_INSERT=2, TARGET_PROT_DIN_STRIP=4, TARGET_PROT_DOUT_STRIP=8, TARGET_PROT_DIN_PASS=16, TARGET_PROT_DOUT_PASS=32 }
pub const TARGET_PROT_ALL:u32 = 1|2|4|8|16|32;
#[repr(C)] pub enum target_prot_type { TARGET_DIF_TYPE0_PROT, TARGET_DIF_TYPE1_PROT, TARGET_DIF_TYPE2_PROT, TARGET_DIF_TYPE3_PROT }
#[repr(C)] pub enum target_ua_intlck_ctrl { TARGET_UA_INTLCK_CTRL_CLEAR=0, TARGET_UA_INTLCK_CTRL_NO_CLEAR=1, TARGET_UA_INTLCK_CTRL_ESTABLISH_UA=2 }
#[repr(C)] pub enum target_core_dif_check { TARGET_DIF_CHECK_GUARD=1, TARGET_DIF_CHECK_APPTAG=2, TARGET_DIF_CHECK_REFTAG=4 }
pub const TCM_SIMPLE_TAG:u32=0x20; pub const TCM_HEAD_TAG:u32=0x21; pub const TCM_ORDERED_TAG:u32=0x22; pub const TCM_ACA_TAG:u32=0x24;

/* The remaining declarations preserve the C header's public layout. */
#[repr(C)] pub struct se_cmd { pub sense_reason:sense_reason_t, pub scsi_status:u8, pub scsi_sense_length:u16, pub unknown_data_length:u32, pub state_active:bool, pub tag:u64, pub alua_nonop_delay:i32, pub data_direction:dma_data_direction, pub sam_task_attr:i32, pub map_tag:u32, pub map_cpu:i32, pub t_state:transport_state_table, pub se_cmd_flags:u32, pub data_length:u32, pub residual_count:u32, pub orig_fe_lun:u64, pub pr_res_key:u64, pub sense_buffer:*mut ::core::ffi::c_void, pub se_delayed_node:list_head, pub se_qf_node:list_head, pub se_dev:*mut se_device, pub se_lun:*mut se_lun, pub se_sess:*mut se_session, pub cmd_cnt:*mut target_cmd_counter, pub se_tmr_req:*mut se_tmr_req, pub se_cmd_list:llist_node, pub free_compl:*mut completion, pub abrt_compl:*mut completion, pub se_tfo:*const target_core_fabric_ops, pub execute_cmd:Option<unsafe extern "C" fn(*mut se_cmd)->sense_reason_t>, pub transport_complete_callback:Option<unsafe extern "C" fn(*mut se_cmd,bool,*mut i32)->sense_reason_t>, pub protocol_data:*mut ::core::ffi::c_void, pub t_task_cdb:*mut u8, pub _t_task_cdb:[u8;TCM_MAX_COMMAND_SIZE], pub t_task_lba:u64, pub t_task_nolb:u32, pub transport_state:u32, pub t_state_lock:spinlock_t, pub cmd_kref:kref, pub t_transport_stop_comp:completion, pub work:work_struct, pub t_data_sg:*mut scatterlist, pub t_data_sg_orig:*mut scatterlist, pub t_data_nents:u32, pub t_data_nents_orig:u32, pub t_data_vmap:*mut ::core::ffi::c_void, pub t_bidi_data_sg:*mut scatterlist, pub t_bidi_data_nents:u32, pub lun_ref_active:i32, pub state_list:list_head, pub priv_:*mut ::core::ffi::c_void, pub prot_op:target_prot_op, pub prot_type:target_prot_type, pub prot_checks:u8, pub prot_pto:bool, pub prot_length:u32, pub reftag_seed:u32, pub t_prot_sg:*mut scatterlist, pub t_prot_nents:u32, pub pi_err:sense_reason_t, pub sense_info:u64, pub cpuid:i32 }
pub const CMD_T_ABORTED:u32=1<<0; pub const CMD_T_ACTIVE:u32=1<<1; pub const CMD_T_COMPLETE:u32=1<<2; pub const CMD_T_SENT:u32=1<<4; pub const CMD_T_STOP:u32=1<<5; pub const CMD_T_TAS:u32=1<<10; pub const CMD_T_FABRIC_STOP:u32=1<<11;
#[repr(C)] pub struct se_ua { pub ua_asc:u8, pub ua_ascq:u8, pub ua_nacl_list:list_head }
#[repr(C)] pub struct target_cmd_counter { pub refcnt:percpu_ref, pub refcnt_wq:wait_queue_head_t, pub stop_done:completion, pub stopped:atomic_t }
#[repr(C)] pub struct se_session { pub sess_bin_isid:u64, pub sup_prot_ops:target_prot_op, pub sess_prot_type:target_prot_type, pub se_node_acl:*mut se_node_acl, pub se_tpg:*mut se_portal_group, pub fabric_sess_ptr:*mut ::core::ffi::c_void, pub sess_list:list_head, pub sess_acl_list:list_head, pub sess_cmd_lock:spinlock_t, pub sess_cmd_map:*mut ::core::ffi::c_void, pub sess_tag_pool:sbitmap_queue, pub cmd_cnt:*mut target_cmd_counter }
#[repr(C)] pub struct se_tmr_req { pub function:u8, pub response:u8, pub call_transport:i32, pub ref_task_tag:u64, pub fabric_tmr_ptr:*mut ::core::ffi::c_void, pub task_cmd:*mut se_cmd, pub tmr_dev:*mut se_device, pub tmr_list:list_head }
#[repr(C)] pub struct se_dev_entry_io_stats { pub total_cmds:u64, pub read_bytes:u64, pub write_bytes:u64 }
#[repr(C)] pub struct se_lun_acl { pub mapped_lun:u64, pub se_lun_nacl:*mut se_node_acl, pub se_lun:*mut se_lun, pub se_lun_group:config_group }
#[repr(C)] pub struct se_port_stat_grps { pub stat_group:config_group, pub scsi_port_group:config_group, pub scsi_tgt_port_group:config_group, pub scsi_transport_group:config_group }
#[repr(C)] pub struct scsi_port_stats { pub cmd_pdus:u64, pub tx_data_octets:u64, pub rx_data_octets:u64 }
#[repr(C)] pub struct se_dev_stat_grps { pub stat_group:config_group, pub scsi_dev_group:config_group, pub scsi_tgt_dev_group:config_group, pub scsi_lu_group:config_group }
#[repr(C)] pub struct se_cmd_queue { pub cmd_list:llist_head, pub work:work_struct }
#[repr(C)] pub struct se_dev_plug { pub se_dev:*mut se_device }
#[repr(C)] pub struct se_device_queue { pub state_list:list_head, pub lock:spinlock_t, pub sq:se_cmd_queue }
#[repr(C)] pub struct se_dev_io_stats { pub total_cmds:u64, pub read_bytes:u64, pub write_bytes:u64 }

/* Inline helpers. */
pub unsafe fn atomic_inc_mb(v:*mut atomic_t) { smp_mb__before_atomic(); atomic_inc(v); smp_mb__after_atomic(); }
pub unsafe fn atomic_dec_mb(v:*mut atomic_t) { smp_mb__before_atomic(); atomic_dec(v); smp_mb__after_atomic(); }
pub unsafe fn target_free_tag(sess:*mut se_session, cmd:*mut se_cmd) { sbitmap_queue_clear(&mut (*sess).sess_tag_pool, (*cmd).map_tag, (*cmd).map_cpu); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
