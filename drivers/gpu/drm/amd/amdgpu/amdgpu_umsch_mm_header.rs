/* SPDX-License-Identifier: MIT */
/* Direct Rust translation of amdgpu_umsch_mm.h. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum UMSCH_SWIP_ENGINE_TYPE {
    UMSCH_SWIP_ENGINE_TYPE_VCN0 = 0,
    UMSCH_SWIP_ENGINE_TYPE_VCN1 = 1,
    UMSCH_SWIP_ENGINE_TYPE_VCN = 2,
    UMSCH_SWIP_ENGINE_TYPE_VPE = 3,
    UMSCH_SWIP_ENGINE_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum UMSCH_CONTEXT_PRIORITY_LEVEL {
    CONTEXT_PRIORITY_LEVEL_IDLE = 0,
    CONTEXT_PRIORITY_LEVEL_NORMAL = 1,
    CONTEXT_PRIORITY_LEVEL_FOCUS = 2,
    CONTEXT_PRIORITY_LEVEL_REALTIME = 3,
    CONTEXT_PRIORITY_NUM_LEVELS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union umsch_mm_set_resource_input_flags {
    pub bits: u32,
}

#[repr(C)]
pub struct umsch_mm_set_resource_input {
    pub vmid_mask_mm_vcn: u32,
    pub vmid_mask_mm_vpe: u32,
    pub collaboration_mask_vpe: u32,
    pub logging_vmid: u32,
    pub engine_mask: u32,
    pub flags: umsch_mm_set_resource_input_flags,
}

#[repr(C)]
pub struct amdgpu_umsch_fwlog {
    pub rptr: u32,
    pub wptr: u32,
    pub buffer_size: u32,
    pub header_size: u32,
    pub wrapped: u32,
}

#[repr(C)]
pub struct umsch_mm_add_queue_input {
    pub process_id: u32,
    pub page_table_base_addr: u64,
    pub process_va_start: u64,
    pub process_va_end: u64,
    pub process_quantum: u64,
    pub process_csa_addr: u64,
    pub context_quantum: u64,
    pub context_csa_addr: u64,
    pub inprocess_context_priority: u32,
    pub context_global_priority_level: UMSCH_CONTEXT_PRIORITY_LEVEL,
    pub doorbell_offset_0: u32,
    pub doorbell_offset_1: u32,
    pub engine_type: UMSCH_SWIP_ENGINE_TYPE,
    pub affinity: u32,
    pub mqd_addr: u64,
    pub h_context: u64,
    pub h_queue: u64,
    pub vm_context_cntl: u32,
    pub process_csa_array_index: u32,
    pub context_csa_array_index: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct umsch_mm_remove_queue_input {
    pub doorbell_offset_0: u32,
    pub doorbell_offset_1: u32,
    pub context_csa_addr: u64,
    pub context_csa_array_index: u32,
}

#[repr(C)]
pub struct MQD_INFO {
    pub rb_base_hi: u32,
    pub rb_base_lo: u32,
    pub rb_size: u32,
    pub wptr_val: u32,
    pub rptr_val: u32,
    pub unmapped: u32,
    pub vmid: u32,
}

#[repr(C)]
pub struct amdgpu_ring;
#[repr(C)]
pub struct firmware;
#[repr(C)]
pub struct amdgpu_bo;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct amdgpu_device;
#[repr(C)]
pub struct amdgpu_ip_block_version;

#[repr(C)]
pub struct amdgpu_umsch_mm {
    pub ring: amdgpu_ring,
    pub rb_wptr: u32,
    pub rb_rptr: u32,
    pub funcs: *const umsch_mm_funcs,
    pub fw: *const firmware,
    pub fw_version: u32,
    pub feature_version: u32,
    pub ucode_fw_obj: *mut amdgpu_bo,
    pub ucode_fw_gpu_addr: u64,
    pub ucode_fw_ptr: *mut u32,
    pub irq_start_addr: u64,
    pub uc_start_addr: u64,
    pub ucode_size: u32,
    pub data_fw_obj: *mut amdgpu_bo,
    pub data_fw_gpu_addr: u64,
    pub data_fw_ptr: *mut u32,
    pub data_start_addr: u64,
    pub data_size: u32,
    pub cmd_buf_obj: *mut amdgpu_bo,
    pub cmd_buf_gpu_addr: u64,
    pub cmd_buf_ptr: *mut u32,
    pub cmd_buf_curr_ptr: *mut u32,
    pub wb_index: u32,
    pub sch_ctx_gpu_addr: u64,
    pub sch_ctx_cpu_addr: *mut u32,
    pub vmid_mask_mm_vcn: u32,
    pub vmid_mask_mm_vpe: u32,
    pub engine_mask: u32,
    pub vcn0_hqd_mask: u32,
    pub vcn1_hqd_mask: u32,
    pub vcn_hqd_mask: [u32; 2],
    pub vpe_hqd_mask: u32,
    pub agdb_index: [u32; 4],
    pub mutex_hidden: mutex,
    pub dbglog_bo: *mut amdgpu_bo,
    pub log_cpu_addr: *mut core::ffi::c_void,
    pub log_gpu_addr: u64,
    pub mem_size: u32,
    pub log_offset: u32,
}

pub type SetHwResources = unsafe extern "C" fn(*mut amdgpu_umsch_mm) -> i32;
pub type AddQueue = unsafe extern "C" fn(*mut amdgpu_umsch_mm, *mut umsch_mm_add_queue_input) -> i32;
pub type RemoveQueue = unsafe extern "C" fn(*mut amdgpu_umsch_mm, *mut umsch_mm_remove_queue_input) -> i32;
pub type UmschFunc = unsafe extern "C" fn(*mut amdgpu_umsch_mm) -> i32;

#[repr(C)]
pub struct umsch_mm_funcs {
    pub set_hw_resources: Option<SetHwResources>,
    pub add_queue: Option<AddQueue>,
    pub remove_queue: Option<RemoveQueue>,
    pub set_regs: Option<UmschFunc>,
    pub init_microcode: Option<UmschFunc>,
    pub load_microcode: Option<UmschFunc>,
    pub ring_init: Option<UmschFunc>,
    pub ring_start: Option<UmschFunc>,
    pub ring_stop: Option<UmschFunc>,
    pub ring_fini: Option<UmschFunc>,
}

extern "C" {
    pub fn amdgpu_umsch_mm_submit_pkt(umsch: *mut amdgpu_umsch_mm, pkt: *mut core::ffi::c_void, ndws: i32) -> i32;
    pub fn amdgpu_umsch_mm_query_fence(umsch: *mut amdgpu_umsch_mm) -> i32;
    pub fn amdgpu_umsch_mm_init_microcode(umsch: *mut amdgpu_umsch_mm) -> i32;
    pub fn amdgpu_umsch_mm_allocate_ucode_buffer(umsch: *mut amdgpu_umsch_mm) -> i32;
    pub fn amdgpu_umsch_mm_allocate_ucode_data_buffer(umsch: *mut amdgpu_umsch_mm) -> i32;
    pub fn amdgpu_umsch_mm_psp_execute_cmd_buf(umsch: *mut amdgpu_umsch_mm) -> i32;
    pub fn amdgpu_umsch_mm_ring_init(umsch: *mut amdgpu_umsch_mm) -> i32;
    pub fn amdgpu_debugfs_umsch_fwlog_init(adev: *mut amdgpu_device, umsch: *mut amdgpu_umsch_mm);
    pub fn amdgpu_umsch_fwlog_init(umsch_mm: *mut amdgpu_umsch_mm);
    pub fn mutex_lock(lock: *mut mutex);
    pub fn mutex_unlock(lock: *mut mutex);
}

pub unsafe fn amdgpu_umsch_mm_lock(umsch: *mut amdgpu_umsch_mm) {
    mutex_lock(&mut (*umsch).mutex_hidden);
}

pub unsafe fn amdgpu_umsch_mm_unlock(umsch: *mut amdgpu_umsch_mm) {
    mutex_unlock(&mut (*umsch).mutex_hidden);
}

extern "C" {
    pub static umsch_mm_v4_0_ip_block: amdgpu_ip_block_version;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
