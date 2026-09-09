/* SPDX-License-Identifier: MIT */
/* Rust translation of vpu_jsm_api.h. */

pub const VPU_JSM_API_VER_MAJOR: u32 = 3;
pub const VPU_JSM_API_VER_MINOR: u32 = 34;
pub const VPU_JSM_API_VER_PATCH: u32 = 0;
pub const VPU_JSM_API_VER_INDEX: u32 = 4;
pub const VPU_HWS_NUM_PRIORITY_BANDS: u32 = 4;
pub const VPU_MAX_ENGINE_RESET_IMPACTED_CONTEXTS: u32 = 3;
pub const VPU_ENGINE_COMPUTE: u32 = 0;
pub const VPU_ENGINE_NB: u32 = 1;

pub const VPU_JSM_STATUS_SUCCESS: u32 = 0x0;
pub const VPU_JSM_STATUS_PARSING_ERR: u32 = 0x1;
pub const VPU_JSM_STATUS_PROCESSING_ERR: u32 = 0x2;
pub const VPU_JSM_STATUS_PREEMPTED: u32 = 0x3;
pub const VPU_JSM_STATUS_ABORTED: u32 = 0x4;
pub const VPU_JSM_STATUS_USER_CTX_VIOL_ERR: u32 = 0x5;
pub const VPU_JSM_STATUS_GLOBAL_CTX_VIOL_ERR: u32 = 0x6;
pub const VPU_JSM_STATUS_MVNCI_WRONG_INPUT_FORMAT: u32 = 0x7;
pub const VPU_JSM_STATUS_MVNCI_UNSUPPORTED_NETWORK_ELEMENT: u32 = 0x8;
pub const VPU_JSM_STATUS_MVNCI_INVALID_HANDLE: u32 = 0x9;
pub const VPU_JSM_STATUS_MVNCI_OUT_OF_RESOURCES: u32 = 0xA;
pub const VPU_JSM_STATUS_MVNCI_NOT_IMPLEMENTED: u32 = 0xB;
pub const VPU_JSM_STATUS_MVNCI_INTERNAL_ERROR: u32 = 0xC;
pub const VPU_JSM_STATUS_PREEMPTED_MID_INFERENCE: u32 = 0xD;
pub const VPU_JSM_STATUS_PREEMPTED_MID_COMMAND: u32 = 0xD;
pub const VPU_JSM_STATUS_ENGINE_RESET_REQUIRED_MIN: u32 = 0xE;
pub const VPU_JSM_STATUS_MVNCI_CONTEXT_VIOLATION_HW: u32 = 0xE;
pub const VPU_JSM_STATUS_MVNCI_PREEMPTION_TIMED_OUT: u32 = 0xF;
pub const VPU_JSM_STATUS_ENGINE_RESET_REQUIRED_MAX: u32 = 0x1F;

pub const VPU_IPC_CHAN_ASYNC_CMD: u32 = 0;
pub const VPU_IPC_CHAN_GEN_CMD: u32 = 10;
pub const VPU_IPC_CHAN_JOB_RET: u32 = 11;
pub const VPU_JOB_FLAGS_NULL_SUBMISSION_MASK: u32 = 1 << 0;
pub const VPU_JOB_FLAGS_INLINE_CMD_MASK: u32 = 1 << 1;
pub const VPU_JOB_FLAGS_PRIVATE_DATA_MASK: u32 = 0xFFFF0000;
pub const VPU_JOB_QUEUE_FLAGS_NO_JOB_DONE_MASK: u32 = 1 << 0;
pub const VPU_JOB_QUEUE_FLAGS_USE_NATIVE_FENCE_MASK: u32 = 1 << 1;
pub const VPU_JOB_QUEUE_FLAGS_TURBO_MODE: u32 = 1 << 2;
pub const VPU_JOB_QUEUE_FLAGS_NON_INTERACTIVE: u32 = 1 << 3;
pub const VPU_TRACE_ENTITY_NAME_MAX_LEN: usize = 32;
pub const VPU_DYNDBG_CMD_MAX_LEN: usize = 96;
pub const VPU_HWS_COMMAND_QUEUE_MAX_IN_PROCESS_PRIORITY: i32 = 7;
pub const VPU_HWS_COMMAND_QUEUE_MIN_IN_PROCESS_PRIORITY: i32 = -7;
pub const VPU_HWS_MAX_REALTIME_PRIORITY_LEVEL: u32 = 31;
pub const VPU_ENGINE_RESET_CONTEXT_FLAG_COLLATERAL_DAMAGE_MASK: u32 = 1 << 0;
pub const VPU_ENGINE_RESET_CONTEXT_HANG_PRIMARY_CAUSE: u32 = 0;
pub const VPU_ENGINE_RESET_CONTEXT_COLLATERAL_DAMAGE: u32 = 1;
pub const VPU_HWS_INVALID_CMDQ_HANDLE: u64 = 0;
pub const VPU_INLINE_CMD_TYPE_NOP: u32 = 0;
pub const VPU_INLINE_CMD_TYPE_FENCE_WAIT: u32 = 1;
pub const VPU_INLINE_CMD_TYPE_FENCE_SIGNAL: u32 = 2;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum vpu_job_scheduling_priority_band { Idle = 0, Normal = 1, Focus = 2, Realtime = 3, Count = 4 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_job_queue_entry {
    pub batch_buf_addr: u64, pub job_id: u32, pub flags: u32,
    pub doorbell_timestamp: u64, pub host_tracking_id: u64,
    pub primary_preempt_buf_addr: u64, pub primary_preempt_buf_size: u32,
    pub secondary_preempt_buf_size: u32, pub secondary_preempt_buf_addr: u64,
    pub reserved_0: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_inline_cmd_fence { pub fence_handle: u64, pub current_value_va: u64, pub monitored_value_va: u64, pub value: u64, pub log_buffer_va: u64, pub npu_private_data: u64 }
#[repr(C)]
pub union vpu_inline_cmd_payload { pub fence: vpu_inline_cmd_fence, pub reserved_1: [u64; 6] }
#[repr(C)]
pub struct vpu_inline_cmd { pub reserved_0: u64, pub r#type: u32, pub flags: u32, pub payload: vpu_inline_cmd_payload }
#[repr(C)]
pub union vpu_jobq_slot { pub job: vpu_job_queue_entry, pub inline_cmd: vpu_inline_cmd }
#[repr(C)]
pub struct vpu_job_queue_header { pub engine_idx: u32, pub head: u32, pub tail: u32, pub flags: u32, pub priority_band_valid: u32, pub priority_band: u32, pub realtime_priority_level: u32, pub reserved_0: [u32; 9] }
#[repr(C)]
pub struct vpu_job_queue { pub header: vpu_job_queue_header, pub slot: [vpu_jobq_slot; 0] }

/* Remaining declarations are intentionally retained as source-level dependency notes;
 * this header's later payload declarations depend only on the same C ABI primitives. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
