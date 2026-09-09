/* Translated from mes_v12_api_def.h. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

pub const MES_API_VERSION: u32 = 0x14;
pub const AMDGPU_MES_LOG_BUFFER_SIZE: u32 = 0xC000;
pub const API_FRAME_SIZE_IN_DWORDS: usize = 64;
pub const API_NUMBER_OF_COMMAND_MAX: u32 = 32;

#[repr(u32)] #[derive(Copy, Clone)] pub enum MES_API_TYPE { MES_API_TYPE_SCHEDULER = 1, MES_API_TYPE_MAX }
#[repr(u32)] #[derive(Copy, Clone)] pub enum MES_SCH_API_OPCODE {
    MES_SCH_API_SET_HW_RSRC=0, MES_SCH_API_SET_SCHEDULING_CONFIG=1, MES_SCH_API_ADD_QUEUE=2,
    MES_SCH_API_REMOVE_QUEUE=3, MES_SCH_API_PERFORM_YIELD=4, MES_SCH_API_SET_GANG_PRIORITY_LEVEL=5,
    MES_SCH_API_SUSPEND=6, MES_SCH_API_RESUME=7, MES_SCH_API_RESET=8, MES_SCH_API_SET_LOG_BUFFER=9,
    MES_SCH_API_CHANGE_GANG_PRORITY=10, MES_SCH_API_QUERY_SCHEDULER_STATUS=11, MES_SCH_API_SET_DEBUG_VMID=13,
    MES_SCH_API_MISC=14, MES_SCH_API_UPDATE_ROOT_PAGE_TABLE=15, MES_SCH_API_AMD_LOG=16,
    MES_SCH_API_SET_SE_MODE=17, MES_SCH_API_SET_GANG_SUBMIT=18, MES_SCH_API_SET_HW_RSRC_1=19,
    MES_SCH_API_INV_TLBS=20, MES_SCH_API_MAX=0xff
}
#[repr(u32)] #[derive(Copy, Clone)] pub enum MES_RRMT_MODE { MES_RRMT_MODE_LOCAL_XCD, MES_RRMT_MODE_LOCAL_REMOTE_AID, MES_RRMT_MODE_REMOTE_XCD, MES_RRMT_MODE_REMOTE_MID }
#[repr(C)] #[derive(Copy, Clone)] pub union MES_API_HEADER { pub u32All: u32, pub bits: MES_API_HEADER_bits }
#[repr(C)] #[derive(Copy, Clone)] pub struct MES_API_HEADER_bits { pub type_: u32, pub opcode: u32, pub dwsize: u32, pub reserved: u32 }
#[repr(u32)] #[derive(Copy, Clone)] pub enum MES_AMD_PRIORITY_LEVEL { AMD_PRIORITY_LEVEL_LOW, AMD_PRIORITY_LEVEL_NORMAL, AMD_PRIORITY_LEVEL_MEDIUM, AMD_PRIORITY_LEVEL_HIGH, AMD_PRIORITY_LEVEL_REALTIME, AMD_PRIORITY_NUM_LEVELS }
#[repr(u32)] #[derive(Copy, Clone)] pub enum MES_QUEUE_TYPE { MES_QUEUE_TYPE_GFX, MES_QUEUE_TYPE_COMPUTE, MES_QUEUE_TYPE_SDMA, MES_QUEUE_TYPE_MAX, MES_QUEUE_TYPE_SCHQ = 3 }
#[repr(C)] #[derive(Copy, Clone)] pub struct MES_API_STATUS { pub api_completion_fence_addr: u64, pub api_completion_fence_value: u64 }
pub const MES_SCH_ERROR_CODE_HEADER_SHIFT_12:u32=8; pub const MES_SCH_ERROR_CODE_MISC_OP_SHIFT_12:u32=16; pub const MES_ERROR_CATEGORY_SHIFT_12:u32=24; pub const MES_API_STATUS_ERROR_SHIFT_12:u32=31;
pub const MES_ERROR_API:u32=1; pub const MES_ERROR_SCHEDULING:u32=2; pub const MES_ERROR_UNKNOWN:u32=3;
#[inline] pub const fn MES_ERR_CODE(api_err:u64, opcode:u64, misc_op:u64, category:u64)->u64 { (api_err | (opcode<<MES_SCH_ERROR_CODE_HEADER_SHIFT_12) | (misc_op<<MES_SCH_ERROR_CODE_MISC_OP_SHIFT_12) | (category<<MES_ERROR_CATEGORY_SHIFT_12) | (1<<MES_API_STATUS_ERROR_SHIFT_12))<<32 }
pub const MAX_COMPUTE_PIPES:usize=8; pub const MAX_GFX_PIPES:usize=2; pub const MAX_SDMA_PIPES:usize=2; pub const MAX_COMPUTE_HQD_PER_PIPE:u32=8; pub const MAX_GFX_HQD_PER_PIPE:u32=8; pub const MAX_SDMA_HQD_PER_PIPE:u32=10; pub const MAX_SDMA_HQD_PER_PIPE_11_0:u32=8; pub const MAX_QUEUES_IN_A_GANG:u32=8;
#[repr(u32)] #[derive(Copy,Clone)] pub enum VM_HUB_TYPE { VM_HUB_TYPE_GC=0, VM_HUB_TYPE_MM=1, VM_HUB_TYPE_MAX }
pub const VMID_INVALID:u32=0xffff; pub const MAX_VMID_GCHUB:u32=16; pub const MAX_VMID_MMHUB:u32=16;
#[repr(u32)] #[derive(Copy,Clone)] pub enum SET_DEBUG_VMID_OPERATIONS { DEBUG_VMID_OP_PROGRAM, DEBUG_VMID_OP_ALLOCATE, DEBUG_VMID_OP_RELEASE, DEBUG_VMID_OP_VM_SETUP }
#[repr(u32)] #[derive(Copy,Clone)] pub enum MES_MS_LOG_CONTEXT_STATE { MES_LOG_CONTEXT_STATE_IDLE, MES_LOG_CONTEXT_STATE_RUNNING, MES_LOG_CONTEXT_STATE_READY, MES_LOG_CONTEXT_STATE_READY_STANDBY=3, MES_LOG_CONTEXT_STATE_INVALID=0xf }
#[repr(u32)] #[derive(Copy,Clone)] pub enum MES_MS_LOG_OPERATION { MES_LOG_OPERATION_CONTEXT_STATE_CHANGE, MES_LOG_OPERATION_QUEUE_NEW_WORK, MES_LOG_OPERATION_QUEUE_UNWAIT_SYNC_OBJECT, MES_LOG_OPERATION_QUEUE_NO_MORE_WORK, MES_LOG_OPERATION_QUEUE_WAIT_SYNC_OBJECT, MES_LOG_OPERATION_QUEUE_INVALID=0xf }
#[repr(C)] #[derive(Copy,Clone)] pub struct MES_LOG_CONTEXT_STATE_CHANGE { pub h_context:u64, pub new_context_state:MES_MS_LOG_CONTEXT_STATE }
#[repr(C)] #[derive(Copy,Clone)] pub struct MES_LOG_QUEUE_NEW_WORK { pub h_queue:u64, pub reserved:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub struct MES_LOG_QUEUE_UNWAIT_SYNC_OBJECT { pub h_queue:u64, pub h_sync_object:u64 }
pub type MES_LOG_QUEUE_NO_MORE_WORK=MES_LOG_QUEUE_NEW_WORK; pub type MES_LOG_QUEUE_WAIT_SYNC_OBJECT=MES_LOG_QUEUE_UNWAIT_SYNC_OBJECT;
#[repr(C)] #[derive(Copy,Clone)] pub struct MES_LOG_ENTRY_HEADER { pub first_free_entry_index:u32, pub wraparound_count:u32, pub number_of_entries:u64, pub reserved:[u64;2] }
#[repr(C)] #[derive(Copy,Clone)] pub union MES_LOG_ENTRY_DATA { pub context_state_change:MES_LOG_CONTEXT_STATE_CHANGE, pub queue_new_work:MES_LOG_QUEUE_NEW_WORK, pub queue_unwait_sync_object:MES_LOG_QUEUE_UNWAIT_SYNC_OBJECT, pub queue_no_more_work:MES_LOG_QUEUE_NO_MORE_WORK, pub queue_wait_sync_object:MES_LOG_QUEUE_WAIT_SYNC_OBJECT, pub all:[u64;2] }
#[repr(C)] pub struct MES_LOG_BUFFER { pub header:MES_LOG_ENTRY_HEADER, pub entries:[MES_LOG_ENTRY_DATA;0] }
pub const MES_MAX_HWIP_SEGMENT:usize=8;

#[repr(C)] #[derive(Copy,Clone)] pub struct MES_BITFIELD { pub value:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI_SET_HW_RESOURCES_fields { pub header:MES_API_HEADER, pub vmid_mask_mmhub:u32, pub vmid_mask_gfxhub:u32, pub gds_size:u32, pub paging_vmid:u32, pub compute_hqd_mask:[u32;8], pub gfx_hqd_mask:[u32;2], pub sdma_hqd_mask:[u32;2], pub aggregated_doorbells:[u32;5], pub g_sch_ctx_gpu_mc_ptr:u64, pub query_status_fence_gpu_mc_ptr:u64, pub gc_base:[u32;8], pub mmhub_base:[u32;8], pub osssys_base:[u32;8], pub api_status:MES_API_STATUS, pub flags:MES_BITFIELD, pub oversubscription_timer:u32, pub doorbell_info:u64, pub event_intr_history_gpu_mc_ptr:u64, pub timestamp:u64, pub os_tdr_timeout_in_sec:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI_SET_HW_RESOURCES { pub data:MESAPI_SET_HW_RESOURCES_fields, pub max_dwords_in_api:[u32;64] }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI_SET_HW_RESOURCES_1_fields { pub header:MES_API_HEADER, pub api_status:MES_API_STATUS, pub timestamp:u64, pub flags:MES_BITFIELD, pub mes_debug_ctx_mc_addr:u64, pub mes_debug_ctx_size:u32, pub mes_kiq_unmap_timeout:u32, pub coop_sch_shared_mc_addr:u64, pub cleaner_shader_fence_mc_addr:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI_SET_HW_RESOURCES_1 { pub data:MESAPI_SET_HW_RESOURCES_1_fields, pub max_dwords_in_api:[u32;64] }

#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__ADD_QUEUE_fields { pub header:MES_API_HEADER, pub process_id:u32, pub page_table_base_addr:u64, pub process_va_start:u64, pub process_va_end:u64, pub process_quantum:u64, pub process_context_addr:u64, pub gang_quantum:u64, pub gang_context_addr:u64, pub inprocess_gang_priority:u32, pub gang_global_priority_level:MES_AMD_PRIORITY_LEVEL, pub doorbell_offset:u32, pub mqd_addr:u64, pub wptr_addr:u64, pub h_context:u64, pub h_queue:u64, pub queue_type:MES_QUEUE_TYPE, pub gds_base:u32, pub gds_size:u32, pub gws_base:u32, pub gws_size:u32, pub oa_mask:u32, pub trap_handler_addr:u64, pub vm_context_cntl:u32, pub flags:MES_BITFIELD, pub api_status:MES_API_STATUS, pub tma_addr:u64, pub sch_id:u32, pub timestamp:u64, pub process_context_array_index:u32, pub gang_context_array_index:u32, pub pipe_id:u32, pub queue_id:u32, pub alignment_mode_setting:u32, pub full_sh_mem_config_data:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__ADD_QUEUE { pub data:MESAPI__ADD_QUEUE_fields, pub max_dwords_in_api:[u32;64] }

#[repr(u32)] #[derive(Copy,Clone)] pub enum MES_API_QUERY_MES_OPCODE { MES_API_QUERY_MES__GET_CTX_ARRAY_SIZE, MES_API_QUERY_MES__CHECK_HEALTHY, MES_API_QUERY_MES__MAX }
pub const QUERY_MES_MAX_SIZE_IN_DWORDS:usize=20;
#[repr(C)] #[derive(Copy,Clone)] pub struct MES_API_QUERY_MES__CTX_ARRAY_SIZE { pub proc_ctx_array_size_addr:u64, pub gang_ctx_array_size_addr:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub struct MES_API_QUERY_MES__HEALTHY_CHECK { pub healthy_addr:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__QUERY_MES_STATUS_fields { pub header:MES_API_HEADER, pub subopcode:MES_API_QUERY_MES_OPCODE, pub api_status:MES_API_STATUS, pub timestamp:u64, pub data:[u32;20] }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__QUERY_MES_STATUS { pub data:MESAPI__QUERY_MES_STATUS_fields, pub max_dwords_in_api:[u32;64] }

#[repr(u32)] #[derive(Copy,Clone)] pub enum MESAPI_MISC_OPCODE { MESAPI_MISC__WRITE_REG, MESAPI_MISC__INV_GART, MESAPI_MISC__QUERY_STATUS, MESAPI_MISC__READ_REG, MESAPI_MISC__WAIT_REG_MEM, MESAPI_MISC__SET_SHADER_DEBUGGER, MESAPI_MISC__NOTIFY_WORK_ON_UNMAPPED_QUEUE, MESAPI_MISC__NOTIFY_TO_UNMAP_PROCESSES, MESAPI_MISC__QUERY_HUNG_ENGINE_ID, MESAPI_MISC__CHANGE_CONFIG, MESAPI_MISC__LAUNCH_CLEANER_SHADER, MESAPI_MISC__SETUP_MES_DBGEXT, MESAPI_MISC__MAX }
pub const MISC_DATA_MAX_SIZE_IN_DWORDS:usize=20;
#[repr(C)] #[derive(Copy,Clone)] pub struct RRMT_OPTION { pub all:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct WRITE_REG { pub reg_offset:u32, pub reg_value:u32, pub rrmt_opt:RRMT_OPTION }
#[repr(C)] #[derive(Copy,Clone)] pub struct READ_REG { pub reg_offset:u32, pub buffer_addr:u64, pub option:u32, pub rrmt_opt:RRMT_OPTION }
#[repr(C)] #[derive(Copy,Clone)] pub struct INV_GART { pub inv_range_va_start:u64, pub inv_range_size:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub struct QUERY_STATUS { pub context_id:u32 }
#[repr(u32)] #[derive(Copy,Clone)] pub enum WRM_OPERATION { WRM_OPERATION__WAIT_REG_MEM, WRM_OPERATION__WR_WAIT_WR_REG, WRM_OPERATION__MAX }
#[repr(C)] #[derive(Copy,Clone)] pub struct WAIT_REG_MEM { pub op:WRM_OPERATION, pub reference:u32, pub mask:u32, pub reg_offset1:u32, pub reg_offset2:u32, pub rrmt_opt1:RRMT_OPTION, pub rrmt_opt2:RRMT_OPTION }
#[repr(C)] #[derive(Copy,Clone)] pub struct SET_SHADER_DEBUGGER { pub process_context_addr:u64, pub flags:MES_BITFIELD, pub spi_gdbg_per_vmid_cntl:u32, pub tcp_watch_cntl:[u32;4], pub trap_en:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct SET_GANG_SUBMIT { pub gang_context_addr:u64, pub slave_gang_context_addr:u64, pub gang_context_array_index:u32, pub slave_gang_context_array_index:u32 }
#[repr(u32)] #[derive(Copy,Clone)] pub enum MESAPI_MISC__CHANGE_CONFIG_OPTION { MESAPI_MISC__CHANGE_CONFIG_OPTION_LIMIT_SINGLE_PROCESS, MESAPI_MISC__CHANGE_CONFIG_OPTION_ENABLE_HWS_LOGGING_BUFFER, MESAPI_MISC__CHANGE_CONFIG_OPTION_CHANGE_TDR_CONFIG, MESAPI_MISC__CHANGE_CONFIG_OPTION_MAX=0x1f }
#[repr(C)] #[derive(Copy,Clone)] pub struct CHANGE_CONFIG { pub opcode:MESAPI_MISC__CHANGE_CONFIG_OPTION, pub option:u32, pub tdr_level:u32, pub tdr_delay:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__MISC_fields { pub header:MES_API_HEADER, pub opcode:MESAPI_MISC_OPCODE, pub api_status:MES_API_STATUS, pub data:[u32;20], pub timestamp:u64, pub doorbell_offset:u32, pub os_fence:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__MISC { pub data:MESAPI__MISC_fields, pub max_dwords_in_api:[u32;64] }

#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__UPDATE_ROOT_PAGE_TABLE_fields { pub header:MES_API_HEADER, pub page_table_base_addr:u64, pub process_context_addr:u64, pub api_status:MES_API_STATUS, pub timestamp:u64, pub process_context_array_index:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__UPDATE_ROOT_PAGE_TABLE { pub data:MESAPI__UPDATE_ROOT_PAGE_TABLE_fields, pub max_dwords_in_api:[u32;64] }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI_AMD_LOG_fields { pub header:MES_API_HEADER, pub p_buffer_memory:u64, pub p_buffer_size_used:u64, pub api_status:MES_API_STATUS, pub timestamp:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI_AMD_LOG { pub data:MESAPI_AMD_LOG_fields, pub max_dwords_in_api:[u32;64] }
#[repr(u32)] #[derive(Copy,Clone)] pub enum MES_SE_MODE { MES_SE_MODE_INVALID=0, MES_SE_MODE_SINGLE_SE=1, MES_SE_MODE_DUAL_SE=2, MES_SE_MODE_LOWER_POWER=3 }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__SET_SE_MODE_fields { pub header:MES_API_HEADER, pub new_se_mode:MES_SE_MODE, pub cpg_ctxt_sync_fence_addr:u64, pub cpg_ctxt_sync_fence_value:u32, pub log_seq_time:u32, pub api_status:MES_API_STATUS }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__SET_SE_MODE { pub data:MESAPI__SET_SE_MODE_fields, pub max_dwords_in_api:[u32;64] }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__SET_GANG_SUBMIT_fields { pub header:MES_API_HEADER, pub api_status:MES_API_STATUS, pub set_gang_submit:SET_GANG_SUBMIT }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__SET_GANG_SUBMIT { pub data:MESAPI__SET_GANG_SUBMIT_fields, pub max_dwords_in_api:[u32;64] }
#[repr(C)] #[derive(Copy,Clone)] pub struct INV_TLBS { pub inv_sel:u8, pub flush_type:u8, pub inv_sel_id:u16, pub hub_id:u32, pub inv_range_va_start:u64, pub inv_range_size:u64, pub reserved:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__INV_TLBS_fields { pub header:MES_API_HEADER, pub api_status:MES_API_STATUS, pub invalidate_tlbs:INV_TLBS }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__INV_TLBS { pub data:MESAPI__INV_TLBS_fields, pub max_dwords_in_api:[u32;64] }

#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__REMOVE_QUEUE_fields { pub header:MES_API_HEADER, pub doorbell_offset:u32, pub gang_context_addr:u64, pub flags:u32, pub api_status:MES_API_STATUS, pub pipe_id:u32, pub queue_id:u32, pub tf_addr:u64, pub tf_data:u32, pub queue_type:MES_QUEUE_TYPE, pub timestamp:u64, pub gang_context_array_index:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__REMOVE_QUEUE { pub data:MESAPI__REMOVE_QUEUE_fields, pub max_dwords_in_api:[u32;64] }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__SET_SCHEDULING_CONFIG_fields { pub header:MES_API_HEADER, pub grace_period_other_levels:[u64;5], pub process_quantum_for_level:[u64;5], pub process_grace_period_same_level:[u64;5], pub normal_yield_percent:u32, pub api_status:MES_API_STATUS, pub timestamp:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__SET_SCHEDULING_CONFIG { pub data:MESAPI__SET_SCHEDULING_CONFIG_fields, pub max_dwords_in_api:[u32;64] }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__PERFORM_YIELD_fields { pub header:MES_API_HEADER, pub dummy:u32, pub api_status:MES_API_STATUS, pub timestamp:u64 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__PERFORM_YIELD { pub data:MESAPI__PERFORM_YIELD_fields, pub max_dwords_in_api:[u32;64] }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__CHANGE_GANG_PRIORITY_LEVEL_fields { pub header:MES_API_HEADER, pub inprocess_gang_priority:u32, pub gang_global_priority_level:MES_AMD_PRIORITY_LEVEL, pub gang_quantum:u64, pub gang_context_addr:u64, pub api_status:MES_API_STATUS, pub doorbell_offset:u32, pub timestamp:u64, pub gang_context_array_index:u32, pub flags:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__CHANGE_GANG_PRIORITY_LEVEL { pub data:MESAPI__CHANGE_GANG_PRIORITY_LEVEL_fields, pub max_dwords_in_api:[u32;64] }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__SUSPEND_fields { pub header:MES_API_HEADER, pub flags:u32, pub gang_context_addr:u64, pub suspend_fence_addr:u64, pub suspend_fence_value:u32, pub api_status:MES_API_STATUS, pub return_value:u32, pub doorbell_offset:u32, pub timestamp:u64, pub legacy_uq_type:MES_QUEUE_TYPE, pub legacy_uq_priority_level:MES_AMD_PRIORITY_LEVEL, pub gang_context_array_index:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__SUSPEND { pub data:MESAPI__SUSPEND_fields, pub max_dwords_in_api:[u32;64] }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__RESUME_fields { pub header:MES_API_HEADER, pub flags:u32, pub gang_context_addr:u64, pub api_status:MES_API_STATUS, pub doorbell_offset:u32, pub timestamp:u64, pub gang_context_array_index:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__RESUME { pub data:MESAPI__RESUME_fields, pub max_dwords_in_api:[u32;64] }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__RESET_fields { pub header:MES_API_HEADER, pub flags:u32, pub gang_context_addr:u64, pub doorbell_offset:u32, pub doorbell_offset_addr:u64, pub queue_type:MES_QUEUE_TYPE, pub pipe_id_lp:u32, pub queue_id_lp:u32, pub vmid_id_lp:u32, pub mqd_mc_addr_lp:u64, pub doorbell_offset_lp:u32, pub wptr_addr_lp:u64, pub pipe_id_hp:u32, pub queue_id_hp:u32, pub vmid_id_hp:u32, pub mqd_mc_addr_hp:u64, pub doorbell_offset_hp:u32, pub wptr_addr_hp:u64, pub api_status:MES_API_STATUS, pub active_vmids:u32, pub timestamp:u64, pub gang_context_array_index:u32, pub connected_queue_index:u32, pub connected_queue_index_p1:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__RESET { pub data:MESAPI__RESET_fields, pub max_dwords_in_api:[u32;64] }
#[repr(C)] #[derive(Copy,Clone)] pub struct MESAPI__SET_LOGGING_BUFFER_fields { pub header:MES_API_HEADER, pub log_type:MES_QUEUE_TYPE, pub logging_buffer_addr:u64, pub number_of_entries:u32, pub interrupt_entry:u32, pub api_status:MES_API_STATUS, pub timestamp:u64, pub vmid:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub union MESAPI__SET_LOGGING_BUFFER { pub data:MESAPI__SET_LOGGING_BUFFER_fields, pub max_dwords_in_api:[u32;64] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
