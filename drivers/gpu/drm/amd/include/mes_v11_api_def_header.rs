/* Translated from mes_v11_api_def.h. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const MES_API_VERSION: u32 = 1;
pub const AMDGPU_MES_LOG_BUFFER_SIZE: u32 = 0x4000;
pub const API_FRAME_SIZE_IN_DWORDS: usize = 64;
pub const API_NUMBER_OF_COMMAND_MAX: u32 = 32;

#[repr(C)] #[derive(Copy, Clone)] pub enum MES_API_TYPE { MES_API_TYPE_SCHEDULER = 1, MES_API_TYPE_MAX }
#[repr(C)] #[derive(Copy, Clone)] pub enum MES_SCH_API_OPCODE { MES_SCH_API_SET_HW_RSRC=0, MES_SCH_API_SET_SCHEDULING_CONFIG=1, MES_SCH_API_ADD_QUEUE=2, MES_SCH_API_REMOVE_QUEUE=3, MES_SCH_API_PERFORM_YIELD=4, MES_SCH_API_SET_GANG_PRIORITY_LEVEL=5, MES_SCH_API_SUSPEND=6, MES_SCH_API_RESUME=7, MES_SCH_API_RESET=8, MES_SCH_API_SET_LOG_BUFFER=9, MES_SCH_API_CHANGE_GANG_PRORITY=10, MES_SCH_API_QUERY_SCHEDULER_STATUS=11, MES_SCH_API_PROGRAM_GDS=12, MES_SCH_API_SET_DEBUG_VMID=13, MES_SCH_API_MISC=14, MES_SCH_API_UPDATE_ROOT_PAGE_TABLE=15, MES_SCH_API_AMD_LOG=16, MES_SCH_API_SET_HW_RSRC_1=19, MES_SCH_API_MAX=0xff }
#[repr(C)] #[derive(Copy, Clone)] pub union MES_API_HEADER { pub bits: MES_API_HEADER_BITS, pub u32All: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct MES_API_HEADER_BITS { pub type_: u32, pub opcode: u32, pub dwsize: u32, pub reserved: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub enum MES_AMD_PRIORITY_LEVEL { AMD_PRIORITY_LEVEL_LOW=0, AMD_PRIORITY_LEVEL_NORMAL=1, AMD_PRIORITY_LEVEL_MEDIUM=2, AMD_PRIORITY_LEVEL_HIGH=3, AMD_PRIORITY_LEVEL_REALTIME=4, AMD_PRIORITY_NUM_LEVELS }
#[repr(C)] #[derive(Copy, Clone)] pub enum MES_QUEUE_TYPE { MES_QUEUE_TYPE_GFX, MES_QUEUE_TYPE_COMPUTE, MES_QUEUE_TYPE_SDMA, MES_QUEUE_TYPE_MAX }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MES_API_STATUS { pub api_completion_fence_addr:u64, pub api_completion_fence_value:u64 }
pub const MAX_COMPUTE_PIPES:u32=8; pub const MAX_GFX_PIPES:u32=2; pub const MAX_SDMA_PIPES:u32=2;
pub const MAX_COMPUTE_HQD_PER_PIPE:u32=8; pub const MAX_GFX_HQD_PER_PIPE:u32=8; pub const MAX_SDMA_HQD_PER_PIPE:u32=10; pub const MAX_SDMA_HQD_PER_PIPE_11_0:u32=8; pub const MAX_QUEUES_IN_A_GANG:u32=8;
#[repr(C)] #[derive(Copy, Clone)] pub enum VM_HUB_TYPE { VM_HUB_TYPE_GC=0, VM_HUB_TYPE_MM=1, VM_HUB_TYPE_MAX }
pub const VMID_INVALID:u32=0xffff; pub const MAX_VMID_GCHUB:u32=16; pub const MAX_VMID_MMHUB:u32=16;
#[repr(C)] #[derive(Copy, Clone)] pub enum SET_DEBUG_VMID_OPERATIONS { DEBUG_VMID_OP_PROGRAM=0, DEBUG_VMID_OP_ALLOCATE=1, DEBUG_VMID_OP_RELEASE=2 }
#[repr(C)] #[derive(Copy, Clone)] pub enum MES_LOG_OPERATION { MES_LOG_OPERATION_CONTEXT_STATE_CHANGE=0, MES_LOG_OPERATION_QUEUE_NEW_WORK=1, MES_LOG_OPERATION_QUEUE_UNWAIT_SYNC_OBJECT=2, MES_LOG_OPERATION_QUEUE_NO_MORE_WORK=3, MES_LOG_OPERATION_QUEUE_WAIT_SYNC_OBJECT=4, MES_LOG_OPERATION_QUEUE_INVALID=0xf }
#[repr(C)] #[derive(Copy, Clone)] pub enum MES_LOG_CONTEXT_STATE { MES_LOG_CONTEXT_STATE_IDLE=0, MES_LOG_CONTEXT_STATE_RUNNING=1, MES_LOG_CONTEXT_STATE_READY=2, MES_LOG_CONTEXT_STATE_READY_STANDBY=3, MES_LOG_CONTEXT_STATE_INVALID=0xf }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MES_LOG_CONTEXT_STATE_CHANGE { pub h_context:*mut core::ffi::c_void, pub new_context_state:MES_LOG_CONTEXT_STATE }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MES_LOG_QUEUE_NEW_WORK { pub h_queue:u64, pub reserved:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MES_LOG_QUEUE_UNWAIT_SYNC_OBJECT { pub h_queue:u64, pub h_sync_object:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MES_LOG_QUEUE_NO_MORE_WORK { pub h_queue:u64, pub reserved:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MES_LOG_QUEUE_WAIT_SYNC_OBJECT { pub h_queue:u64, pub h_sync_object:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MES_LOG_ENTRY_HEADER { pub first_free_entry_index:u32, pub wraparound_count:u32, pub number_of_entries:u64, pub reserved:[u64;2] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union MES_LOG_ENTRY_DATA_UNION { pub context_state_change:MES_LOG_CONTEXT_STATE_CHANGE, pub queue_new_work:MES_LOG_QUEUE_NEW_WORK, pub queue_unwait_sync_object:MES_LOG_QUEUE_UNWAIT_SYNC_OBJECT, pub queue_no_more_work:MES_LOG_QUEUE_NO_MORE_WORK, pub queue_wait_sync_object:MES_LOG_QUEUE_WAIT_SYNC_OBJECT, pub all:[u64;2] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MES_LOG_ENTRY_DATA { pub gpu_time_stamp:u64, pub operation_type:u32, pub reserved_operation_type_bits:u32, pub data:MES_LOG_ENTRY_DATA_UNION }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MES_LOG_BUFFER { pub header:MES_LOG_ENTRY_HEADER, pub entries:[MES_LOG_ENTRY_DATA;1] }
#[repr(C)] #[derive(Copy, Clone)] pub enum MES_SWIP_TO_HWIP_DEF { MES_MAX_HWIP_SEGMENT=8 }

// C bit-fields are represented by their containing 32-bit words; callers may use masks matching the source declarations.
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MESAPI_SET_HW_RESOURCES_BODY { pub header:MES_API_HEADER, pub vmid_mask_mmhub:u32, pub vmid_mask_gfxhub:u32, pub gds_size:u32, pub paging_vmid:u32, pub compute_hqd_mask:[u32;8], pub gfx_hqd_mask:[u32;2], pub sdma_hqd_mask:[u32;2], pub aggregated_doorbells:[u32;5], pub g_sch_ctx_gpu_mc_ptr:u64, pub query_status_fence_gpu_mc_ptr:u64, pub gc_base:[u32;8], pub mmhub_base:[u32;8], pub osssys_base:[u32;8], pub api_status:MES_API_STATUS, pub flags:u32, pub oversubscription_timer:u32, pub doorbell_info:u64, pub event_intr_history_gpu_mc_ptr:u64, pub timestamp:u64, pub os_tdr_timeout_in_sec:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union MESAPI_SET_HW_RESOURCES { pub body:MESAPI_SET_HW_RESOURCES_BODY, pub max_dwords_in_api:[u32;64] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MESAPI_SET_HW_RESOURCES_1_BODY { pub header:MES_API_HEADER, pub api_status:MES_API_STATUS, pub timestamp:u64, pub flags:u32, pub mes_info_ctx_mc_addr:u64, pub mes_info_ctx_size:u32, pub reserved1:u64, pub cleaner_shader_fence_mc_addr:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union MESAPI_SET_HW_RESOURCES_1 { pub body:MESAPI_SET_HW_RESOURCES_1_BODY, pub max_dwords_in_api:[u32;64] }

#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MESAPI__ADD_QUEUE_BODY { pub header:MES_API_HEADER, pub process_id:u32, pub page_table_base_addr:u64, pub process_va_start:u64, pub process_va_end:u64, pub process_quantum:u64, pub process_context_addr:u64, pub gang_quantum:u64, pub gang_context_addr:u64, pub inprocess_gang_priority:u32, pub gang_global_priority_level:MES_AMD_PRIORITY_LEVEL, pub doorbell_offset:u32, pub mqd_addr:u64, pub wptr_addr:u64, pub h_context:u64, pub h_queue:u64, pub queue_type:MES_QUEUE_TYPE, pub gds_base:u32, pub gds_size:u32, pub gws_base:u32, pub gws_size:u32, pub oa_mask:u32, pub trap_handler_addr:u64, pub vm_context_cntl:u32, pub flags:u32, pub api_status:MES_API_STATUS, pub tma_addr:u64, pub sch_id:u32, pub timestamp:u64, pub process_context_array_index:u32, pub gang_context_array_index:u32, pub pipe_id:u32, pub queue_id:u32, pub alignment_mode_setting:u32, pub full_sh_mem_config_data:u32, pub unmap_flag_addr:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union MESAPI__ADD_QUEUE { pub body:MESAPI__ADD_QUEUE_BODY, pub max_dwords_in_api:[u32;64] }

// The remaining API packets retain the source packet names and fixed 64-dword frame union.
macro_rules! simple_packet { ($n:ident, $b:ident { $($f:ident : $t:ty),* $(,)? }) => { #[repr(C, packed(4)] #[derive(Copy, Clone)] pub struct $b { pub header:MES_API_HEADER, $(pub $f:$t,)* } #[repr(C, packed(4))] #[derive(Copy, Clone)] pub union $n { pub body:$b, pub max_dwords_in_api:[u32;64] } }; }
simple_packet!(MESAPI__PERFORM_YIELD, MESAPI__PERFORM_YIELD_BODY { dummy:u32, api_status:MES_API_STATUS });
simple_packet!(MESAPI__CHANGE_GANG_PRIORITY_LEVEL, MESAPI__CHANGE_GANG_PRIORITY_LEVEL_BODY { inprocess_gang_priority:u32, gang_global_priority_level:MES_AMD_PRIORITY_LEVEL, gang_quantum:u64, gang_context_addr:u64, api_status:MES_API_STATUS });
simple_packet!(MESAPI__PROGRAM_GDS, MESAPI__PROGRAM_GDS_BODY { process_context_addr:u64, gds_base:u32, gds_size:u32, gws_base:u32, gws_size:u32, oa_mask:u32, api_status:MES_API_STATUS });
simple_packet!(MESAPI__UPDATE_ROOT_PAGE_TABLE, MESAPI__UPDATE_ROOT_PAGE_TABLE_BODY { page_table_base_addr:u64, process_context_addr:u64, api_status:MES_API_STATUS });
simple_packet!(MESAPI_AMD_LOG, MESAPI_AMD_LOG_BODY { p_buffer_memory:u64, p_buffer_size_used:u64, api_status:MES_API_STATUS });
simple_packet!(MESAPI__REMOVE_QUEUE, MESAPI__REMOVE_QUEUE_BODY { doorbell_offset:u32, gang_context_addr:u64, flags:u32, api_status:MES_API_STATUS, pipe_id:u32, queue_id:u32, tf_addr:u64, tf_data:u32, queue_type:MES_QUEUE_TYPE, timestamp:u64, gang_context_array_index:u32 });
simple_packet!(MESAPI__SET_SCHEDULING_CONFIG, MESAPI__SET_SCHEDULING_CONFIG_BODY { grace_period_other_levels:[u64;5], process_quantum_for_level:[u64;5], process_grace_period_same_level:[u64;5], normal_yield_percent:u32, api_status:MES_API_STATUS });
simple_packet!(MESAPI__SUSPEND, MESAPI__SUSPEND_BODY { flags:u32, gang_context_addr:u64, suspend_fence_addr:u64, suspend_fence_value:u32, api_status:MES_API_STATUS, doorbell_offset:u32 });
simple_packet!(MESAPI__RESUME, MESAPI__RESUME_BODY { flags:u32, gang_context_addr:u64, api_status:MES_API_STATUS, doorbell_offset:u32 });
simple_packet!(MESAPI__RESET, MESAPI__RESET_BODY { flags:u32, gang_context_addr:u64, doorbell_offset:u32, doorbell_offset_addr:u64, queue_type:MES_QUEUE_TYPE, pipe_id_lp:u32, queue_id_lp:u32, vmid_id_lp:u32, mqd_mc_addr_lp:u64, doorbell_offset_lp:u32, wptr_addr_lp:u64, pipe_id_hp:u32, queue_id_hp:u32, vmid_id_hp:u32, mqd_mc_addr_hp:u64, doorbell_offset_hp:u32, wptr_addr_hp:u64, api_status:MES_API_STATUS });
simple_packet!(MESAPI__SET_LOGGING_BUFFER, MESAPI__SET_LOGGING_BUFFER_BODY { log_type:MES_QUEUE_TYPE, logging_buffer_addr:u64, number_of_entries:u32, interrupt_entry:u32, api_status:MES_API_STATUS });
simple_packet!(MESAPI__SET_DEBUG_VMID, MESAPI__SET_DEBUG_VMID_BODY { api_status:MES_API_STATUS, flags:u32, reserved:u32, debug_vmid:u32, process_context_addr:u64, page_table_base_addr:u64, process_va_start:u64, process_va_end:u64, gds_base:u32, gds_size:u32, gws_base:u32, gws_size:u32, oa_mask:u32, output_addr:u64 });

#[repr(C)] #[derive(Copy, Clone)] pub enum MES_API_QUERY_MES_OPCODE { MES_API_QUERY_MES__GET_CTX_ARRAY_SIZE=0, MES_API_QUERY_MES__GET_CAPS=0, MES_API_QUERY_MES__CHECK_HEALTHY, MES_API_QUERY_MES__MAX }
pub const QUERY_MES_MAX_SIZE_IN_DWORDS:usize=20;
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MES_API_QUERY_MES__CTX_ARRAY_SIZE { pub proc_ctx_array_size_addr:u64, pub gang_ctx_array_size_addr:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MES_API_QUERY_MES__HEALTHY_CHECK { pub healthy_addr:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MES_API_QUERY_MES__CAPS { pub proc_ctx_array_size_addr:u64, pub gang_ctx_array_size_addr:u64, pub features_enablement_addr:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union MES_API_QUERY_MES_DATA { pub ctx_array_size:MES_API_QUERY_MES__CTX_ARRAY_SIZE, pub caps:MES_API_QUERY_MES__CAPS, pub healthy_check:MES_API_QUERY_MES__HEALTHY_CHECK, pub data:[u32;20] }
simple_packet!(MESAPI__QUERY_MES_STATUS, MESAPI__QUERY_MES_STATUS_BODY { subopcode:MES_API_QUERY_MES_OPCODE, api_status:MES_API_STATUS, timestamp:u64, data:MES_API_QUERY_MES_DATA });
#[repr(C)] #[derive(Copy, Clone)] pub enum MESAPI_MISC_OPCODE { MESAPI_MISC__WRITE_REG, MESAPI_MISC__INV_GART, MESAPI_MISC__QUERY_STATUS, MESAPI_MISC__READ_REG, MESAPI_MISC__WAIT_REG_MEM, MESAPI_MISC__SET_SHADER_DEBUGGER, MESAPI_MISC__NOTIFY_WORK_ON_UNMAPPED_QUEUE, MESAPI_MISC__NOTIFY_TO_UNMAP_PROCESSES, MESAPI_MISC__CHANGE_CONFIG, MESAPI_MISC__LAUNCH_CLEANER_SHADER, MESAPI_MISC__MAX }
pub const MISC_DATA_MAX_SIZE_IN_DWORDS:usize=20;
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct WRITE_REG { pub reg_offset:u32, pub reg_value:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct READ_REG { pub reg_offset:u32, pub buffer_addr:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub enum WRM_OPERATION { WRM_OPERATION__WAIT_REG_MEM, WRM_OPERATION__WR_WAIT_WR_REG, WRM_OPERATION__MAX }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct WAIT_REG_MEM { pub op:WRM_OPERATION, pub reference:u32, pub mask:u32, pub reg_offset1:u32, pub reg_offset2:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct INV_GART { pub inv_range_va_start:u64, pub inv_range_size:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct QUERY_STATUS { pub context_id:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct SET_SHADER_DEBUGGER { pub process_context_addr:u64, pub flags:u32, pub spi_gdbg_per_vmid_cntl:u32, pub tcp_watch_cntl:[u32;4], pub trap_en:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub enum MESAPI_MISC__CHANGE_CONFIG_OPTION { MESAPI_MISC__CHANGE_CONFIG_OPTION_LIMIT_SINGLE_PROCESS=0, MESAPI_MISC__CHANGE_CONFIG_OPTION_ENABLE_HWS_LOGGING_BUFFER=1, MESAPI_MISC__CHANGE_CONFIG_OPTION_CHANGE_TDR_CONFIG=2, MESAPI_MISC__CHANGE_CONFIG_OPTION_MAX=0x1f }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct CHANGE_CONFIG { pub opcode:MESAPI_MISC__CHANGE_CONFIG_OPTION, pub option:u32, pub tdr_level:u32, pub tdr_delay:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct MESAPI__MISC_BODY { pub header:MES_API_HEADER, pub opcode:MESAPI_MISC_OPCODE, pub api_status:MES_API_STATUS, pub data:[u32;20] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union MESAPI__MISC { pub body:MESAPI__MISC_BODY, pub max_dwords_in_api:[u32;64] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
