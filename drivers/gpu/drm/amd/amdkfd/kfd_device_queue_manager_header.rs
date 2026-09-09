/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright 2014-2022 Advanced Micro Devices, Inc.
 *
 * Translated from kfd_device_queue_manager.h. Linux/kernel dependencies and
 * declarations supplied by other headers remain external to this translation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

pub const VMID_NUM: usize = 16;
pub const KFD_MES_PROCESS_QUANTUM: u32 = 100000;
pub const KFD_MES_GANG_QUANTUM: u32 = 10000;

#[repr(C)] pub struct qcm_process_device { _private: [u8; 0] }
#[repr(C)] pub struct queue { _private: [u8; 0] }
#[repr(C)] pub struct kernel_queue { _private: [u8; 0] }
#[repr(C)] pub struct kfd_criu_queue_priv_data { _private: [u8; 0] }
#[repr(C)] pub struct mqd_update_info { _private: [u8; 0] }
#[repr(C)] pub struct kfd_node { pub kfd2kgd: *mut kfd2kgd_callbacks, pub adev: *mut c_void, pub xcc_mask: u32 }
#[repr(C)] pub struct kfd2kgd_callbacks { pub get_iq_wait_times: Option<unsafe extern "C" fn(*mut c_void, *mut u32, i32)> }
#[repr(C)] pub struct packet_manager { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kfd_mem_obj { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_mes_hung_queue_hqd_info { _private: [u8; 0] }
#[repr(C)] pub struct kfd_queue_snapshot_entry { _private: [u8; 0] }
#[repr(C)] pub struct kfd_process { _private: [u8; 0] }
#[repr(C)] pub struct kfd_process_device { pub lds_base: u64 }
#[repr(C)] pub struct mqd_manager { _private: [u8; 0] }

pub type u32 = core::ffi::c_uint;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;
pub type cache_policy = i32;
pub type KFD_MQD_TYPE = i32;
pub type kfd_queue_type = i32;

#[repr(C)] pub struct device_process_node { pub qpd: *mut qcm_process_device, pub list: list_head }

#[repr(C)] pub union SQ_CMD_BITS {
    pub bitfields: u32,
    pub bits: u32,
    pub u32All: u32,
    pub i32All: i32,
    pub f32All: f32,
}

#[repr(C)] pub union GRBM_GFX_INDEX_BITS {
    pub bitfields: u32,
    pub bits: u32,
    pub u32All: u32,
    pub i32All: i32,
    pub f32All: f32,
}

pub type create_queue_fn = unsafe extern "C" fn(*mut device_queue_manager, *mut queue, *mut qcm_process_device, *const kfd_criu_queue_priv_data, *const c_void, *const c_void) -> i32;
pub type destroy_queue_fn = unsafe extern "C" fn(*mut device_queue_manager, *mut qcm_process_device, *mut queue) -> i32;
pub type update_queue_fn = unsafe extern "C" fn(*mut device_queue_manager, *mut queue, *mut mqd_update_info) -> i32;
pub type qpd_fn = unsafe extern "C" fn(*mut device_queue_manager, *mut qcm_process_device) -> i32;
pub type dqm_fn = unsafe extern "C" fn(*mut device_queue_manager) -> i32;
pub type dqm_void_fn = unsafe extern "C" fn(*mut device_queue_manager);

#[repr(C)] pub struct device_queue_manager_ops {
    pub create_queue: Option<create_queue_fn>, pub destroy_queue: Option<destroy_queue_fn>,
    pub update_queue: Option<update_queue_fn>, pub register_process: Option<qpd_fn>,
    pub unregister_process: Option<qpd_fn>, pub initialize: Option<dqm_fn>, pub start: Option<dqm_fn>,
    pub stop: Option<dqm_fn>, pub uninitialize: Option<dqm_void_fn>, pub halt: Option<dqm_fn>, pub unhalt: Option<dqm_fn>,
    pub create_kernel_queue: Option<unsafe extern "C" fn(*mut device_queue_manager, *mut kernel_queue, *mut qcm_process_device) -> i32>,
    pub destroy_kernel_queue: Option<unsafe extern "C" fn(*mut device_queue_manager, *mut kernel_queue, *mut qcm_process_device)>,
    pub set_cache_memory_policy: Option<unsafe extern "C" fn(*mut device_queue_manager, *mut qcm_process_device, cache_policy, cache_policy, *mut c_void, u64, u32) -> bool>,
    pub process_termination: Option<qpd_fn>, pub evict_process_queues: Option<qpd_fn>, pub restore_process_queues: Option<qpd_fn>,
    pub get_wave_state: Option<unsafe extern "C" fn(*mut device_queue_manager, *mut queue, *mut c_void, *mut u32, *mut u32) -> i32>,
    pub reset_queues: Option<unsafe extern "C" fn(*mut device_queue_manager, u16) -> i32>,
    pub get_queue_checkpoint_info: Option<unsafe extern "C" fn(*mut device_queue_manager, *const queue, *mut u32, *mut u32) -> i32>,
    pub checkpoint_mqd: Option<unsafe extern "C" fn(*mut device_queue_manager, *const queue, *mut c_void, *mut c_void) -> i32>,
    pub set_perfcount: Option<unsafe extern "C" fn(*mut device_queue_manager, i32)>,
}

#[repr(C)] pub struct device_queue_manager_asic_ops {
    pub update_qpd: Option<qpd_fn>, pub set_cache_memory_policy: Option<unsafe extern "C" fn(*mut device_queue_manager, *mut qcm_process_device, cache_policy, cache_policy, *mut c_void, u64, u32) -> bool>,
    pub init_sdma_vm: Option<unsafe extern "C" fn(*mut device_queue_manager, *mut queue, *mut qcm_process_device)>,
    pub mqd_manager_init: Option<unsafe extern "C" fn(KFD_MQD_TYPE, *mut kfd_node) -> *mut mqd_manager>,
}

#[repr(C)] pub struct dqm_detect_hang_info { pub pipe_id: i32, pub queue_id: i32, pub xcc_id: i32, pub queue_address: u64 }

#[repr(C)] pub struct device_queue_manager {
    pub ops: device_queue_manager_ops, pub asic_ops: device_queue_manager_asic_ops,
    pub mqd_mgrs: [*mut mqd_manager; 16], pub packet_mgr: packet_manager, pub dev: *mut kfd_node,
    pub lock_hidden: mutex, pub queues: list_head, pub saved_flags: u32, pub processes_count: u32,
    pub active_queue_count: u32, pub active_cp_queue_count: u32, pub gws_queue_count: u32, pub total_queue_count: u32,
    pub next_pipe_to_allocate: u32, pub allocated_queues: *mut u32, pub sdma_bitmap: [u64; 1], pub xgmi_sdma_bitmap: [u64; 1],
    pub vmid_pasid: [u16; VMID_NUM], pub pipelines_addr: u64, pub fence_gpu_addr: u64, pub fence_addr: *mut u64,
    pub fence_mem: *mut kfd_mem_obj, pub active_runlist: bool, pub sched_policy: i32, pub trap_debug_vmid: u32,
    pub is_hws_hang: bool, pub is_resetting: bool, pub hiq_sdma_mqd: kfd_mem_obj, pub sched_running: bool, pub sched_halt: bool,
    pub current_logical_xcc_start: u32, pub wait_times: u32, pub destroy_wait: wait_queue_head_t,
    pub detect_hang_info: *mut dqm_detect_hang_info, pub detect_hang_info_size: usize, pub detect_hang_count: i32,
    pub hung_db_array: *mut u32, pub hqd_info: *mut amdgpu_mes_hung_queue_hqd_info,
}

extern "C" {
    pub fn device_queue_manager_init_cik(ops: *mut device_queue_manager_asic_ops);
    pub fn device_queue_manager_init_vi(ops: *mut device_queue_manager_asic_ops);
    pub fn device_queue_manager_init_v9(ops: *mut device_queue_manager_asic_ops);
    pub fn device_queue_manager_init_v10(ops: *mut device_queue_manager_asic_ops);
    pub fn device_queue_manager_init_v11(ops: *mut device_queue_manager_asic_ops);
    pub fn device_queue_manager_init_v12(ops: *mut device_queue_manager_asic_ops);
    pub fn device_queue_manager_init_v12_1(ops: *mut device_queue_manager_asic_ops);
    pub fn program_sh_mem_settings(dqm: *mut device_queue_manager, qpd: *mut qcm_process_device);
    pub fn get_cp_queues_num(dqm: *mut device_queue_manager) -> u32;
    pub fn get_queues_per_pipe(dqm: *mut device_queue_manager) -> u32;
    pub fn get_pipes_per_mec(dqm: *mut device_queue_manager) -> u32;
    pub fn get_num_sdma_queues(dqm: *mut device_queue_manager) -> u32;
    pub fn get_num_xgmi_sdma_queues(dqm: *mut device_queue_manager) -> u32;
    pub fn reserve_debug_trap_vmid(dqm: *mut device_queue_manager, qpd: *mut qcm_process_device) -> i32;
    pub fn release_debug_trap_vmid(dqm: *mut device_queue_manager, qpd: *mut qcm_process_device) -> i32;
    pub fn suspend_queues(p: *mut kfd_process, num_queues: u32, grace_period: u32, exception_clear_mask: u64, usr_queue_id_array: *mut u32) -> i32;
    pub fn resume_queues(p: *mut kfd_process, num_queues: u32, usr_queue_id_array: *mut u32) -> i32;
    pub fn set_queue_snapshot_entry(q: *mut queue, exception_clear_mask: u64, qss_entry: *mut kfd_queue_snapshot_entry);
    pub fn debug_lock_and_unmap(dqm: *mut device_queue_manager) -> i32;
    pub fn debug_map_and_unlock(dqm: *mut device_queue_manager) -> i32;
    pub fn debug_refresh_runlist(dqm: *mut device_queue_manager) -> i32;
    pub fn kfd_dqm_is_queue_in_process(dqm: *mut device_queue_manager, qpd: *mut qcm_process_device, doorbell_off: i32, queue_format: *mut u32) -> bool;
    pub fn kfd_reset_queue_mes(dqm: *mut device_queue_manager, queue_type: i32, pipe: i32, queue: i32, db: u32) -> i32;
    pub fn mqd_size_from_queue_type(dqm: *mut device_queue_manager, ty: kfd_queue_type) -> usize;
}

#[inline] pub unsafe fn get_sh_mem_bases_32(pdd: *mut kfd_process_device) -> u32 { ((*pdd).lds_base >> 16 & 0xFF) as u32 }
#[inline] pub unsafe fn get_sh_mem_bases_nybble_64(pdd: *mut kfd_process_device) -> u32 { ((*pdd).lds_base >> 60 & 0x0E) as u32 }

/* The DQM lock can be taken in MMU notifiers; reclaim-FS must be disabled while holding it. */
#[inline] pub unsafe fn dqm_lock(dqm: *mut device_queue_manager) { mutex_lock(&mut (*dqm).lock_hidden); (*dqm).saved_flags = memalloc_noreclaim_save(); }
#[inline] pub unsafe fn dqm_unlock(dqm: *mut device_queue_manager) { memalloc_noreclaim_restore((*dqm).saved_flags); mutex_unlock(&mut (*dqm).lock_hidden); }
#[inline] pub unsafe fn read_sdma_queue_counter(q_rptr: *const u64, val: *mut u64) -> i32 { get_user(val, q_rptr.add(1)) }
#[inline] pub unsafe fn update_dqm_wait_times(dqm: *mut device_queue_manager) { if let Some(f) = (*(*dqm).dev).kfd2kgd.as_ref().and_then(|x| x.get_iq_wait_times) { f((*(*dqm).dev).adev, &mut (*dqm).wait_times, 0); } }

extern "C" { fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex); fn memalloc_noreclaim_save() -> u32; fn memalloc_noreclaim_restore(flags: u32); fn get_user(val: *mut u64, src: *const u64) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
