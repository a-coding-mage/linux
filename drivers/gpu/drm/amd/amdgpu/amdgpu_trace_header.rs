/*
 * Faithful Rust representation of amdgpu_trace.h.
 *
 * Linux TRACE_EVENT/DECLARE_EVENT_CLASS/DEFINE_EVENT are preprocessor
 * declarations which generate tracepoint plumbing outside this header.  The
 * payloads below preserve their externally visible field names, order, and
 * C layout; the original TP_fast_assign and TP_printk expressions are kept
 * as comments because their implementations are supplied by the tracepoint
 * framework.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

// TRACE_SYSTEM amdgpu; TRACE_INCLUDE_FILE amdgpu_trace
// AMDGPU_JOB_GET_TIMELINE_NAME(job) expands to
// job->base.s_fence->finished.ops->get_timeline_name(&job->base.s_fence->finished)

#[repr(C)]
pub struct amdgpu_device_rreg { pub did: u32, pub reg: u32, pub value: u32 }
pub type amdgpu_device_wreg = amdgpu_device_rreg;

#[repr(C)]
pub struct amdgpu_iv {
    pub ih: u32, pub client_id: u32, pub src_id: u32, pub ring_id: u32,
    pub vmid: u32, pub vmid_src: u32, pub timestamp: u64,
    pub timestamp_src: u32, pub pasid: u32, pub src_data: [u32; 4],
}

#[repr(C)]
pub struct amdgpu_bo_create {
    pub bo: *mut c_void, pub pages: u32, pub type_: u32, pub prefer: u32,
    pub allow: u32, pub visible: u32,
}

#[repr(C)]
pub struct amdgpu_cs {
    pub bo_list: *mut c_void, pub ring: u32, pub dw: u32, pub fences: u32,
}

#[repr(C)]
pub struct amdgpu_cs_ioctl {
    pub timeline: *const core::ffi::c_char, pub context: u64, pub seqno: u64,
    pub fence: *mut c_void, pub ring: *const core::ffi::c_char, pub num_ibs: u32,
}
pub type amdgpu_sched_run_job = amdgpu_cs_ioctl;

#[repr(C)]
pub struct amdgpu_vm_grab_id {
    pub pasid: u32, pub ring: *const core::ffi::c_char, pub ring_id: u32,
    pub vmid: u32, pub vm_hub: u32, pub pd_addr: u64, pub needs_flush: u32,
}

#[repr(C)]
pub struct amdgpu_vm_bo_map {
    pub bo: *mut c_void, pub start: isize, pub last: isize,
    pub offset: u64, pub flags: u64,
}
pub type amdgpu_vm_bo_unmap = amdgpu_vm_bo_map;

#[repr(C)]
pub struct amdgpu_vm_mapping { pub soffset: u64, pub eoffset: u64, pub flags: u64 }
pub type amdgpu_vm_bo_update = amdgpu_vm_mapping;
pub type amdgpu_vm_bo_mapping = amdgpu_vm_mapping;
pub type amdgpu_vm_bo_cs = amdgpu_vm_mapping;

#[repr(C)]
pub struct amdgpu_vm_update_ptes {
    pub start: u64, pub end: u64, pub flags: u64, pub nptes: u32,
    pub incr: u64, pub pid: i32, pub vm_ctx: u64,
    // __dynamic_array(u64, dst, nptes)
    pub dst: *mut u64,
}

#[repr(C)]
pub struct amdgpu_vm_set_ptes {
    pub pe: u64, pub addr: u64, pub count: u32, pub incr: u32,
    pub flags: u64, pub immediate: bool,
}

#[repr(C)]
pub struct amdgpu_vm_copy_ptes {
    pub pe: u64, pub src: u64, pub count: u32, pub immediate: bool,
}

#[repr(C)]
pub struct amdgpu_vm_flush {
    pub ring: *const core::ffi::c_char, pub vmid: u32, pub vm_hub: u32,
    pub pd_addr: u64,
}

#[repr(C)]
pub struct amdgpu_pasid { pub pasid: u32 }
pub type amdgpu_pasid_allocated = amdgpu_pasid;
pub type amdgpu_pasid_freed = amdgpu_pasid;

#[repr(C)]
pub struct amdgpu_isolation { pub prev: *mut c_void, pub next: *mut c_void }

#[repr(C)]
pub struct amdgpu_cleaner_shader {
    pub ring: *const core::ffi::c_char, pub seqno: u64,
}

#[repr(C)]
pub struct amdgpu_bo_list_set {
    pub list: *mut c_void, pub bo: *mut c_void, pub bo_size: u64,
}

#[repr(C)]
pub struct amdgpu_cs_bo_status { pub total_bo: u64, pub total_size: u64 }

#[repr(C)]
pub struct amdgpu_bo_move {
    pub bo: *mut c_void, pub bo_size: u64, pub new_placement: u32,
    pub old_placement: u32,
}

#[repr(C)]
pub struct amdgpu_ib_pipe_sync {
    pub ring: *const core::ffi::c_char, pub fence: *mut c_void,
    pub ctx: u64, pub seqno: u64,
}

#[repr(C)]
pub struct amdgpu_reset_reg_dumps { pub address: u32, pub value: u32 }

#[repr(C)]
pub struct amdgpu_userq_queue {
    pub queue: *mut c_void, pub doorbell_index: u64, pub queue_type: i32,
    pub state: i32, pub xcp_id: u32,
}
pub type amdgpu_userq_create_start = amdgpu_userq_queue;
pub type amdgpu_userq_destroy_start = amdgpu_userq_queue;

#[repr(C)]
pub struct amdgpu_userq_queue_result {
    pub queue: *mut c_void, pub doorbell_index: u64, pub queue_type: i32,
    pub state: i32, pub xcp_id: u32, pub result: i32,
}
pub type amdgpu_userq_create_end = amdgpu_userq_queue_result;
pub type amdgpu_userq_destroy_end = amdgpu_userq_queue_result;

#[repr(C)]
pub struct amdgpu_userq_emit_fence {
    pub fence_context: u64, pub fence_seqno: u64, pub dev: *const core::ffi::c_char,
    pub doorbell_index: u64, pub client_id: u64, pub queue_type: u32,
}

#[repr(C)]
pub struct amdgpu_userq_wait_deps {
    pub context: u64, pub dep_context: u64, pub dep_seqno: u64,
    pub dev: *const core::ffi::c_char, pub doorbell_index: u64,
    pub client_id: u64, pub queue_type: u32,
}

#[repr(C)]
pub struct amdgpu_userq_state_start {
    pub doorbell_index: u64, pub client_id: u64, pub queue_type: u32, pub from: u32,
}

#[repr(C)]
pub struct amdgpu_userq_state_changed {
    pub doorbell_index: u64, pub client_id: u64, pub queue_type: u32, pub to: u32,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
