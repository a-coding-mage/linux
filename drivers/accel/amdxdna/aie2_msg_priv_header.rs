/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2022-2024, Advanced Micro Devices, Inc. */

#[repr(u32)]
pub enum aie2_msg_opcode {
    MSG_OP_CREATE_CONTEXT = 0x2, MSG_OP_DESTROY_CONTEXT = 0x3,
    MSG_OP_GET_TELEMETRY = 0x4, MSG_OP_SYNC_BO = 0x7,
    MSG_OP_EXECUTE_BUFFER_CF = 0xC, MSG_OP_QUERY_COL_STATUS = 0xD,
    MSG_OP_QUERY_AIE_TILE_INFO = 0xE, MSG_OP_QUERY_AIE_VERSION = 0xF,
    MSG_OP_EXEC_DPU = 0x10, MSG_OP_CONFIG_CU = 0x11,
    MSG_OP_CHAIN_EXEC_BUFFER_CF = 0x12, MSG_OP_CHAIN_EXEC_DPU = 0x13,
    MSG_OP_CONFIG_DEBUG_BO = 0x14, MSG_OP_CHAIN_EXEC_NPU = 0x18,
    MSG_OP_MAX_XRT_OPCODE, MSG_OP_SUSPEND = 0x101, MSG_OP_RESUME,
    MSG_OP_ASSIGN_MGMT_PASID, MSG_OP_INVOKE_SELF_TEST, MSG_OP_MAP_HOST_BUFFER = 0x106,
    MSG_OP_GET_FIRMWARE_VERSION = 0x108, MSG_OP_SET_RUNTIME_CONFIG = 0x10A,
    MSG_OP_GET_RUNTIME_CONFIG, MSG_OP_REGISTER_ASYNC_EVENT_MSG,
    MSG_OP_UPDATE_PROPERTY = 0x113, MSG_OP_GET_APP_HEALTH, MSG_OP_ADD_HOST_BUFFER,
    MSG_OP_GET_DEV_REVISION = 0x117, MSG_OP_MAX_DRV_OPCODE,
    MSG_OP_GET_PROTOCOL_VERSION = 0x301, MSG_OP_MAX_OPCODE,
}

#[repr(u32)]
pub enum aie2_msg_status {
    AIE2_STATUS_SUCCESS = 0x0,
    AIE2_STATUS_AIE_SATURATION_ERROR = 0x1000001, AIE2_STATUS_AIE_FP_ERROR,
    AIE2_STATUS_AIE_STREAM_ERROR, AIE2_STATUS_AIE_ACCESS_ERROR,
    AIE2_STATUS_AIE_BUS_ERROR, AIE2_STATUS_AIE_INSTRUCTION_ERROR,
    AIE2_STATUS_AIE_ECC_ERROR, AIE2_STATUS_AIE_LOCK_ERROR,
    AIE2_STATUS_AIE_DMA_ERROR, AIE2_STATUS_AIE_MEM_PARITY_ERROR,
    AIE2_STATUS_AIE_PWR_CFG_ERROR, AIE2_STATUS_AIE_BACKTRACK_ERROR,
    AIE2_STATUS_MAX_AIE_STATUS_CODE,
    AIE2_STATUS_MGMT_ERT_SELF_TEST_FAILURE = 0x2000001,
    AIE2_STATUS_MGMT_ERT_HASH_MISMATCH, AIE2_STATUS_MGMT_ERT_NOAVAIL,
    AIE2_STATUS_MGMT_ERT_INVALID_PARAM, AIE2_STATUS_MGMT_ERT_ENTER_SUSPEND_FAILURE,
    AIE2_STATUS_MGMT_ERT_BUSY, AIE2_STATUS_MGMT_ERT_APPLICATION_ACTIVE,
    MAX_MGMT_ERT_STATUS_CODE,
    AIE2_STATUS_APP_ERT_FIRST_ERROR = 0x3000001, AIE2_STATUS_APP_INVALID_INSTR,
    AIE2_STATUS_APP_LOAD_PDI_FAIL, MAX_APP_ERT_STATUS_CODE,
    AIE2_STATUS_INVALID_INPUT_BUFFER = 0x4000001, AIE2_STATUS_INVALID_COMMAND,
    AIE2_STATUS_INVALID_PARAM, AIE2_STATUS_INVALID_OPERATION = 0x4000006,
    AIE2_STATUS_ASYNC_EVENT_MSGS_FULL, AIE2_STATUS_MAX_RTOS_STATUS_CODE,
    MAX_AIE2_STATUS_CODE,
}

#[repr(C, packed)] pub struct assign_mgmt_pasid_req { pub pasid: __u16, pub reserved: __u16 }
#[repr(C, packed)] pub struct assign_mgmt_pasid_resp { pub status: aie2_msg_status }
#[repr(C, packed)] pub struct map_host_buffer_req { pub context_id: __u32, pub buf_addr: __u64, pub buf_size: __u64 }
#[repr(C, packed)] pub struct map_host_buffer_resp { pub status: aie2_msg_status }

pub const MAX_CQ_PAIRS: usize = 2;
#[repr(C)] pub struct cq_info { pub head_addr: __u32, pub tail_addr: __u32, pub buf_addr: __u32, pub buf_size: __u32 }
#[repr(C)] pub struct cq_pair { pub x2i_q: cq_info, pub i2x_q: cq_info }
pub const PRIORITY_REALTIME: u32 = 1; pub const PRIORITY_HIGH: u32 = 2; pub const PRIORITY_NORMAL: u32 = 3; pub const PRIORITY_LOW: u32 = 4;

#[repr(C, packed)] pub struct create_ctx_req { pub aie_type: __u32, pub start_col: __u8, pub num_col: __u8, pub num_unused_col: __u8, pub reserved: __u8, pub num_cq_pairs_requested: __u8, pub reserved1: __u8, pub pasid: __u16, pub pad: [__u32;2], pub sec_comm_target_type: __u32, pub context_priority: __u32 }
#[repr(C, packed)] pub struct create_ctx_resp { pub status: aie2_msg_status, pub context_id: __u32, pub msix_id: __u16, pub num_cq_pairs_allocated: __u8, pub reserved: __u8, pub cq_pair: [cq_pair; MAX_CQ_PAIRS] }
#[repr(C, packed)] pub struct destroy_ctx_req { pub context_id: __u32 }
#[repr(C, packed)] pub struct destroy_ctx_resp { pub status: aie2_msg_status }
#[repr(u32)] pub enum telemetry_type { TELEMETRY_TYPE_DISABLED, TELEMETRY_TYPE_HEALTH, TELEMETRY_TYPE_ERROR_INFO, TELEMETRY_TYPE_PROFILING, TELEMETRY_TYPE_DEBUG, MAX_TELEMETRY_TYPE }
#[repr(C, packed)] pub struct get_telemetry_req { pub r#type: telemetry_type, pub buf_addr: __u64, pub buf_size: __u32 }
#[repr(C, packed)] pub struct get_telemetry_resp { pub major: __u32, pub minor: __u32, pub size: __u32, pub status: aie2_msg_status }
#[repr(C, packed)] pub struct execute_buffer_req { pub cu_idx: __u32, pub payload: [__u32;19] }
#[repr(C, packed)] pub struct exec_dpu_req { pub inst_buf_addr: __u64, pub inst_size: __u32, pub inst_prop_cnt: __u32, pub cu_idx: __u32, pub payload: [__u32;35] }
#[repr(u32)] pub enum exec_npu_type { EXEC_NPU_TYPE_NON_ELF=1, EXEC_NPU_TYPE_PARTIAL_ELF, EXEC_NPU_TYPE_PREEMPT, EXEC_NPU_TYPE_ELF }
#[repr(C)] pub union exec_req { pub ebuf: execute_buffer_req, pub dpu_req: exec_dpu_req }
#[repr(C, packed)] pub struct execute_buffer_resp { pub status: aie2_msg_status }

#[repr(C)] pub struct aie_tile_info { pub size: __u32, pub major: __u16, pub minor: __u16, pub cols: __u16, pub rows: __u16, pub core_rows: __u16, pub mem_rows: __u16, pub shim_rows: __u16, pub core_row_start: __u16, pub mem_row_start: __u16, pub shim_row_start: __u16, pub core_dma_channels: __u16, pub mem_dma_channels: __u16, pub shim_dma_channels: __u16, pub core_locks: __u16, pub mem_locks: __u16, pub shim_locks: __u16, pub core_events: __u16, pub mem_events: __u16, pub shim_events: __u16, pub reserved: __u16 }
#[repr(C, packed)] pub struct aie_tile_info_req { pub reserved: __u32 }
#[repr(C, packed)] pub struct aie_tile_info_resp { pub status: aie2_msg_status, pub info: aie_tile_info }
#[repr(C, packed)] pub struct aie_version_info_req { pub reserved: __u32 }
#[repr(C, packed)] pub struct aie_version_info_resp { pub status: aie2_msg_status, pub major: __u16, pub minor: __u16 }
#[repr(C, packed)] pub struct aie_column_info_req { pub dump_buff_addr: __u64, pub dump_buff_size: __u32, pub num_cols: __u32, pub aie_bitmap: __u32 }
#[repr(C, packed)] pub struct aie_column_info_resp { pub status: aie2_msg_status, pub size: __u32 }

#[repr(C, packed)] pub struct suspend_req { pub place_holder: __u32 }
#[repr(C, packed)] pub struct suspend_resp { pub status: aie2_msg_status }
#[repr(C, packed)] pub struct resume_req { pub place_holder: __u32 }
#[repr(C, packed)] pub struct resume_resp { pub status: aie2_msg_status }
#[repr(C, packed)] pub struct check_header_hash_req { pub hash_high: __u64, pub hash_low: __u64 }
#[repr(C, packed)] pub struct check_header_hash_resp { pub status: aie2_msg_status }
#[repr(C, packed)] pub struct query_error_req { pub buf_addr: __u64, pub buf_size: __u32, pub next_row: __u32, pub next_column: __u32, pub next_module: __u32 }
#[repr(C, packed)] pub struct query_error_resp { pub status: aie2_msg_status, pub num_err: __u32, pub has_next_err: __u32, pub next_row: __u32, pub next_column: __u32, pub next_module: __u32 }
#[repr(C, packed)] pub struct protocol_version_req { pub reserved: __u32 }
#[repr(C, packed)] pub struct protocol_version_resp { pub status: aie2_msg_status, pub major: __u32, pub minor: __u32 }
#[repr(C, packed)] pub struct firmware_version_req { pub reserved: __u32 }
#[repr(C, packed)] pub struct firmware_version_resp { pub status: aie2_msg_status, pub major: __u32, pub minor: __u32, pub sub: __u32, pub build: __u32 }

pub const MAX_NUM_CUS: usize = 32;
pub const AIE2_MSG_CFG_CU_PDI_ADDR: __u32 = 0x1ffff;
pub const AIE2_MSG_CFG_CU_FUNC: __u32 = 0xfe0000;
#[repr(C, packed)] pub struct config_cu_req { pub num_cus: __u32, pub cfgs: [__u32;MAX_NUM_CUS] }
#[repr(C, packed)] pub struct config_cu_resp { pub status: aie2_msg_status }
#[repr(C, packed)] pub struct set_runtime_cfg_req { pub r#type: __u32, pub value: __u64 }
#[repr(C, packed)] pub struct set_runtime_cfg_resp { pub status: aie2_msg_status }
#[repr(C, packed)] pub struct get_runtime_cfg_req { pub r#type: __u32 }
#[repr(C, packed)] pub struct get_runtime_cfg_resp { pub status: aie2_msg_status, pub value: __u64 }
#[repr(u32)] pub enum async_event_type { ASYNC_EVENT_TYPE_AIE_ERROR, ASYNC_EVENT_TYPE_EXCEPTION, MAX_ASYNC_EVENT_TYPE }
// Build-time kernel constant: SZ_8K.
pub const ASYNC_BUF_SIZE: usize = SZ_8K;
#[repr(C, packed)] pub struct async_event_msg_req { pub buf_addr: __u64, pub buf_size: __u32 }
#[repr(C, packed)] pub struct async_event_msg_resp { pub status: aie2_msg_status, pub r#type: async_event_type }
// Build-time kernel constant: SZ_4K.
pub const MAX_CHAIN_CMDBUF_SIZE: usize = SZ_4K;
#[repr(C)] pub struct cmd_chain_slot_execbuf_cf { pub cu_idx: __u32, pub arg_cnt: __u32, pub args: [__u32;0] }
#[repr(C)] pub struct cmd_chain_slot_dpu { pub inst_buf_addr: __u64, pub inst_size: __u32, pub inst_prop_cnt: __u32, pub cu_idx: __u32, pub arg_cnt: __u32, pub args: [__u32;0] }
pub const MAX_DPU_ARGS_SIZE: usize = 34 * core::mem::size_of::<__u32>();
pub const MAX_NPU_ARGS_SIZE: usize = 26 * core::mem::size_of::<__u32>();
pub const AIE2_EXEC_BUFFER_KERNEL_OP_TXN: u32 = 3;
#[repr(C, packed)] pub struct cmd_chain_slot_npu { pub r#type: exec_npu_type, pub inst_buf_addr: u64, pub save_buf_addr: u64, pub restore_buf_addr: u64, pub inst_size: u32, pub save_size: u32, pub restore_size: u32, pub inst_prop_cnt: u32, pub cu_idx: u32, pub arg_cnt: u32, pub args: [u32;0] }
#[repr(C, packed)] pub struct cmd_chain_req { pub buf_addr: __u64, pub buf_size: __u32, pub count: __u32 }
#[repr(C, packed)] pub struct cmd_chain_npu_req { pub flags: u32, pub reserved: u32, pub buf_addr: u64, pub buf_size: u32, pub count: u32 }
#[repr(C)] pub union exec_chain_req { pub npu_req: cmd_chain_npu_req, pub req: cmd_chain_req }
#[repr(C, packed)] pub struct cmd_chain_resp { pub status: aie2_msg_status, pub fail_cmd_idx: __u32, pub fail_cmd_status: aie2_msg_status }
pub const AIE2_MSG_SYNC_BO_SRC_TYPE: __u32 = 0xf; pub const AIE2_MSG_SYNC_BO_DST_TYPE: __u32 = 0xf0;
#[repr(C, packed)] pub struct sync_bo_req { pub src_addr: __u64, pub dst_addr: __u64, pub size: __u32, pub r#type: __u32 }
pub const SYNC_BO_DEV_MEM: u32 = 0; pub const SYNC_BO_HOST_MEM: u32 = 2;
#[repr(C, packed)] pub struct sync_bo_resp { pub status: aie2_msg_status }
pub const DEBUG_BO_UNREGISTER: u32 = 0; pub const DEBUG_BO_REGISTER: u32 = 1;
#[repr(C, packed)] pub struct config_debug_bo_req { pub offset: __u64, pub size: __u64, pub config: __u32 }
#[repr(C, packed)] pub struct config_debug_bo_resp { pub status: aie2_msg_status }

#[repr(C)] pub struct fatal_error_info { pub fatal_type: __u32, pub exception_type: __u32, pub exception_argument: __u32, pub exception_pc: __u32, pub app_module: __u32, pub task_index: __u32, pub reserved: [__u32;127] }
#[repr(C)] pub struct app_health_report { pub major: __u16, pub minor: __u16, pub size: __u32, pub context_id: __u32, pub dpu_pc: __u32, pub txn_op_id: __u32, pub ctx_pc: __u32, pub fatal_info: fatal_error_info, pub run_list_id: __u32 }
#[repr(C, packed)] pub struct get_app_health_req { pub context_id: __u32, pub buf_size: __u32, pub buf_addr: __u64 }
#[repr(C, packed)] pub struct get_app_health_resp { pub status: aie2_msg_status, pub required_buffer_size: __u32, pub reserved: [__u32;7] }
#[repr(C, packed)] pub struct update_property_req { pub r#type: __u32, pub context_id: __u8, pub reserved: [__u8;7], pub time_quota_us: __u32, pub reserved1: __u32 }
pub const UPDATE_PROPERTY_TIME_QUOTA: u32 = 0; pub const AIE2_UPDATE_PROPERTY_ALL_CTX: u8 = 0xFF;
#[repr(C, packed)] pub struct update_property_resp { pub status: aie2_msg_status }
#[repr(u32)] pub enum aie2_dev_revision { AIE2_DEV_REVISION_STXA=1, AIE2_DEV_REVISION_STXB, AIE2_DEV_REVISION_KRK1, AIE2_DEV_REVISION_KRK2, AIE2_DEV_REVISION_HALO, AIE2_DEV_REVISION_GPT1, AIE2_DEV_REVISION_GPT2, AIE2_DEV_REVISION_GPT3, AIE2_DEV_REVISION_UNKN }
#[repr(C, packed)] pub struct get_dev_revision_req { pub place_holder: __u32 }
#[repr(C, packed)] pub struct get_dev_revision_resp { pub status: aie2_msg_status, pub rev: aie2_dev_revision, pub raw_fuse_data: __u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
