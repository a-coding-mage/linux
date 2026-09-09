/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

pub const UMSCH_API_VERSION: u32 = 1;
pub const API_FRAME_SIZE_IN_DWORDS: usize = 64;
pub const API_NUMBER_OF_COMMAND_MAX: u32 = 32;
pub const UMSCH_INSTANCE_DB_OFFSET_MAX: u32 = 16;

#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UMSCH_API_TYPE { UMSCH_API_TYPE_SCHEDULER = 1, UMSCH_API_TYPE_MAX }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UMSCH_MS_LOG_CONTEXT_STATE { UMSCH_LOG_CONTEXT_STATE_IDLE=0, UMSCH_LOG_CONTEXT_STATE_RUNNING=1, UMSCH_LOG_CONTEXT_STATE_READY=2, UMSCH_LOG_CONTEXT_STATE_READY_STANDBY=3, UMSCH_LOG_CONTEXT_STATE_INVALID=0xF }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UMSCH_MS_LOG_OPERATION { UMSCH_LOG_OPERATION_CONTEXT_STATE_CHANGE=0, UMSCH_LOG_OPERATION_QUEUE_NEW_WORK=1, UMSCH_LOG_OPERATION_QUEUE_UNWAIT_SYNC_OBJECT=2, UMSCH_LOG_OPERATION_QUEUE_NO_MORE_WORK=3, UMSCH_LOG_OPERATION_QUEUE_WAIT_SYNC_OBJECT=4, UMSCH_LOG_OPERATION_QUEUE_INVALID=0xF }

#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCH_INSTANCE_DB_OFFSET { pub instance_index:u32, pub doorbell_offset:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCH_LOG_CONTEXT_STATE_CHANGE { pub h_context:u64, pub new_context_state:UMSCH_MS_LOG_CONTEXT_STATE }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCH_LOG_QUEUE_NEW_WORK { pub h_queue:u64, pub reserved:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCH_LOG_QUEUE_UNWAIT_SYNC_OBJECT { pub h_queue:u64, pub h_sync_object:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCH_LOG_QUEUE_NO_MORE_WORK { pub h_queue:u64, pub reserved:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCH_LOG_QUEUE_WAIT_SYNC_OBJECT { pub h_queue:u64, pub h_sync_object:u64 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCH_LOG_ENTRY_HEADER { pub first_free_entry_index:u32, pub wraparound_count:u32, pub number_of_entries:u64, pub reserved:[u64;2] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCH_LOG_ENTRY_DATA_UNION { pub context_state_change:UMSCH_LOG_CONTEXT_STATE_CHANGE, pub queue_new_work:UMSCH_LOG_QUEUE_NEW_WORK, pub queue_unwait_sync_object:UMSCH_LOG_QUEUE_UNWAIT_SYNC_OBJECT, pub queue_no_more_work:UMSCH_LOG_QUEUE_NO_MORE_WORK, pub queue_wait_sync_object:UMSCH_LOG_QUEUE_WAIT_SYNC_OBJECT, pub all:[u64;2] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCH_LOG_ENTRY_DATA { pub gpu_time_stamp:u64, pub operation_type:u32, pub reserved_operation_type_bits:u32, pub data:UMSCH_LOG_ENTRY_DATA_UNION }
#[repr(C, packed(4))] pub struct UMSCH_LOG_BUFFER { pub header:UMSCH_LOG_ENTRY_HEADER, pub entries:[UMSCH_LOG_ENTRY_DATA;1] }

#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum UMSCH_API_OPCODE { UMSCH_API_SET_HW_RSRC=0, UMSCH_API_SET_SCHEDULING_CONFIG=1, UMSCH_API_ADD_QUEUE=2, UMSCH_API_REMOVE_QUEUE=3, UMSCH_API_PERFORM_YIELD=4, UMSCH_API_SUSPEND=5, UMSCH_API_RESUME=6, UMSCH_API_RESET=7, UMSCH_API_SET_LOG_BUFFER=8, UMSCH_API_CHANGE_CONTEXT_PRIORITY=9, UMSCH_API_QUERY_SCHEDULER_STATUS=0xA, UMSCH_API_UPDATE_AFFINITY=0xB, UMSCH_API_MAX=0xFF }
#[repr(C)] #[derive(Copy, Clone)] pub struct UMSCH_API_HEADER_BITS { pub type_:u32, pub opcode:u32, pub dwsize:u32, pub reserved:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub union UMSCH_API_HEADER { pub bits:UMSCH_API_HEADER_BITS, pub u32All:u32 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum UMSCH_AMD_PRIORITY_LEVEL { AMD_PRIORITY_LEVEL_IDLE=0, AMD_PRIORITY_LEVEL_NORMAL=1, AMD_PRIORITY_LEVEL_FOCUS=2, AMD_PRIORITY_LEVEL_REALTIME=3, AMD_PRIORITY_NUM_LEVELS }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum UMSCH_ENGINE_TYPE { UMSCH_ENGINE_TYPE_VCN0=0, UMSCH_ENGINE_TYPE_VCN1=1, UMSCH_ENGINE_TYPE_VCN=2, UMSCH_ENGINE_TYPE_VPE=3, UMSCH_ENGINE_TYPE_MAX }
pub const AFFINITY_DISABLE:u32=0; pub const AFFINITY_ENABLE:u32=1; pub const AFFINITY_MAX:u32=2;
#[repr(C)] #[derive(Copy, Clone)] pub struct UMSCH_AFFINITY_BITS { pub vcn0Affinity:u32, pub vcn1Affinity:u32, pub reserved:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub union UMSCH_AFFINITY { pub bits:UMSCH_AFFINITY_BITS, pub u32All:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCH_API_STATUS { pub api_completion_fence_addr:u64, pub api_completion_fence_value:u32 }
pub const MAX_VCN0_INSTANCES:usize=1; pub const MAX_VCN1_INSTANCES:usize=1; pub const MAX_VCN_INSTANCES:usize=2; pub const MAX_VPE_INSTANCES:usize=1; pub const MAX_VCN_QUEUES:u32=4; pub const MAX_VPE_QUEUES:u32=8; pub const MAX_QUEUES_IN_A_CONTEXT:u32=1; pub const UMSCH_MAX_HWIP_SEGMENT:usize=8;
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum VM_HUB_TYPE { VM_HUB_TYPE_GC=0, VM_HUB_TYPE_MM=1, VM_HUB_TYPE_MAX }
pub const VMID_INVALID:u32=0xffff; pub const MAX_VMID_MMHUB:u32=16;

#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCHAPI_SET_HW_RESOURCES { pub header:UMSCH_API_HEADER, pub vmid_mask_mm_vcn:u32, pub vmid_mask_mm_vpe:u32, pub collaboration_mask_vpe:u32, pub engine_mask:u32, pub logging_vmid:u32, pub vcn0_hqd_mask:[u32;1], pub vcn1_hqd_mask:[u32;1], pub vcn_hqd_mask:[u32;2], pub vpe_hqd_mask:[u32;1], pub g_sch_ctx_gpu_mc_ptr:u64, pub mmhub_base:[u32;8], pub mmhub_version:u32, pub osssys_base:[u32;8], pub osssys_version:u32, pub vcn_version:u32, pub vpe_version:u32, pub api_status:UMSCH_API_STATUS, pub flags:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCHAPI__SET_HW_RESOURCES { pub data:UMSCHAPI_SET_HW_RESOURCES, pub max_dwords_in_api:[u32;64] }

#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCHAPI_SET_SCHEDULING_CONFIG { pub header:UMSCH_API_HEADER, pub grace_period_other_levels:[u64;4], pub process_quantum_for_level:[u64;4], pub process_grace_period_same_level:[u64;4], pub normal_yield_percent:u32, pub api_status:UMSCH_API_STATUS }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCHAPI__SET_SCHEDULING_CONFIG { pub data:UMSCHAPI_SET_SCHEDULING_CONFIG, pub max_dwords_in_api:[u32;64] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCHAPI_ADD_QUEUE { pub header:UMSCH_API_HEADER, pub process_id:u32, pub page_table_base_addr:u64, pub process_va_start:u64, pub process_va_end:u64, pub process_quantum:u64, pub process_csa_addr:u64, pub context_quantum:u64, pub context_csa_addr:u64, pub inprocess_context_priority:u32, pub context_global_priority_level:UMSCH_AMD_PRIORITY_LEVEL, pub doorbell_offset_0:u32, pub doorbell_offset_1:u32, pub affinity:UMSCH_AFFINITY, pub mqd_addr:u64, pub h_context:u64, pub h_queue:u64, pub engine_type:UMSCH_ENGINE_TYPE, pub vm_context_cntl:u32, pub context_flags:u32, pub api_status:UMSCH_API_STATUS, pub process_csa_array_index:u32, pub context_csa_array_index:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCHAPI__ADD_QUEUE { pub data:UMSCHAPI_ADD_QUEUE, pub max_dwords_in_api:[u32;64] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCHAPI_REMOVE_QUEUE { pub header:UMSCH_API_HEADER, pub doorbell_offset_0:u32, pub doorbell_offset_1:u32, pub context_csa_addr:u64, pub api_status:UMSCH_API_STATUS, pub context_csa_array_index:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCHAPI__REMOVE_QUEUE { pub data:UMSCHAPI_REMOVE_QUEUE, pub max_dwords_in_api:[u32;64] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCHAPI_PERFORM_YIELD { pub header:UMSCH_API_HEADER, pub dummy:u32, pub api_status:UMSCH_API_STATUS }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCHAPI__PERFORM_YIELD { pub data:UMSCHAPI_PERFORM_YIELD, pub max_dwords_in_api:[u32;64] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCHAPI_SUSPEND { pub header:UMSCH_API_HEADER, pub context_csa_addr:u64, pub suspend_fence_addr:u64, pub suspend_fence_value:u32, pub api_status:UMSCH_API_STATUS, pub context_csa_array_index:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCHAPI__SUSPEND { pub data:UMSCHAPI_SUSPEND, pub max_dwords_in_api:[u32;64] }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum UMSCH_RESUME_OPTION { CONTEXT_RESUME=0, ENGINE_SCHEDULE_RESUME=1 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCHAPI_RESUME { pub header:UMSCH_API_HEADER, pub resume_option:UMSCH_RESUME_OPTION, pub context_csa_addr:u64, pub engine_type:UMSCH_ENGINE_TYPE, pub api_status:UMSCH_API_STATUS, pub context_csa_array_index:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCHAPI__RESUME { pub data:UMSCHAPI_RESUME, pub max_dwords_in_api:[u32;64] }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum UMSCH_RESET_OPTION { HANG_DETECT_AND_RESET=0, HANG_DETECT_ONLY=1 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCHAPI_RESET { pub header:UMSCH_API_HEADER, pub reset_option:UMSCH_RESET_OPTION, pub doorbell_offset_addr:u64, pub engine_type:UMSCH_ENGINE_TYPE, pub api_status:UMSCH_API_STATUS }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCHAPI__RESET { pub data:UMSCHAPI_RESET, pub max_dwords_in_api:[u32;64] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCHAPI_SET_LOGGING_BUFFER { pub header:UMSCH_API_HEADER, pub log_type:UMSCH_ENGINE_TYPE, pub logging_buffer_addr:u64, pub number_of_entries:u32, pub interrupt_entry:u32, pub api_status:UMSCH_API_STATUS }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCHAPI__SET_LOGGING_BUFFER { pub data:UMSCHAPI_SET_LOGGING_BUFFER, pub max_dwords_in_api:[u32;64] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCHAPI_UPDATE_AFFINITY { pub header:UMSCH_API_HEADER, pub affinity:UMSCH_AFFINITY, pub context_csa_addr:u64, pub api_status:UMSCH_API_STATUS, pub context_csa_array_index:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCHAPI__UPDATE_AFFINITY { pub data:UMSCHAPI_UPDATE_AFFINITY, pub max_dwords_in_api:[u32;64] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCHAPI_CHANGE_CONTEXT_PRIORITY_LEVEL { pub header:UMSCH_API_HEADER, pub inprocess_context_priority:u32, pub context_global_priority_level:UMSCH_AMD_PRIORITY_LEVEL, pub context_quantum:u64, pub context_csa_addr:u64, pub api_status:UMSCH_API_STATUS, pub context_csa_array_index:u32 }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCHAPI__CHANGE_CONTEXT_PRIORITY_LEVEL { pub data:UMSCHAPI_CHANGE_CONTEXT_PRIORITY_LEVEL, pub max_dwords_in_api:[u32;64] }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub struct UMSCHAPI_QUERY_UMSCH_STATUS { pub header:UMSCH_API_HEADER, pub umsch_mm_healthy:bool, pub api_status:UMSCH_API_STATUS }
#[repr(C, packed(4))] #[derive(Copy, Clone)] pub union UMSCHAPI__QUERY_UMSCH_STATUS { pub data:UMSCHAPI_QUERY_UMSCH_STATUS, pub max_dwords_in_api:[u32;64] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
