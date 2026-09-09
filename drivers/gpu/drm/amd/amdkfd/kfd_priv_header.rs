/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/* Direct Rust translation of kfd_priv.h. Kernel includes and configuration
 * conditions are external dependencies and are intentionally not expanded. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

/* External kernel types. */
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic64_t { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct hlist_node { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct kfifo { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct idr { _private: [u8; 0] }
#[repr(C)] pub struct ida { _private: [u8; 0] }
#[repr(C)] pub struct iosys_map { _private: [u8; 0] }
#[repr(C)] pub struct mmu_notifier { _private: [u8; 0] }
#[repr(C)] pub struct rb_root_cached { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct srcu_struct { _private: [u8; 0] }
#[repr(C)] pub struct dma_fence { _private: [u8; 0] }
#[repr(C)] pub struct semaphore { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct drm_device { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { pub kfd: *mut kfd_dev }
#[repr(C)] pub struct amdgpu_xcp { pub ddev: *mut drm_device }
#[repr(C)] pub struct amdgpu_bo { _private: [u8; 0] }
#[repr(C)] pub struct amdgpu_vm { _private: [u8; 0] }
#[repr(C)] pub struct kfd2kgd_calls { _private: [u8; 0] }
#[repr(C)] pub struct kgd2kfd_shared_resources { _private: [u8; 0] }
#[repr(C)] pub struct kfd_local_mem_info { _private: [u8; 0] }
#[repr(C)] pub struct device_queue_manager { _private: [u8; 0] }
#[repr(C)] pub struct kgd_mem { _private: [u8; 0] }
#[repr(C)] pub struct mqd_manager { _private: [u8; 0] }
#[repr(C)] pub struct kernel_queue { _private: [u8; 0] }
#[repr(C)] pub struct kfd_topology_device { _private: [u8; 0] }
#[repr(C)] pub struct kfd_vm_fault_info { _private: [u8; 0] }
#[repr(C)] pub struct kfd_ioctl_svm_attribute { _private: [u8; 0] }
#[repr(C)] pub struct kfd_hsa_memory_exception_data { _private: [u8; 0] }
#[repr(C)] pub struct kfd_hsa_hw_exception_data { _private: [u8; 0] }
#[repr(C)] pub struct kfd_device_global_init_class { _private: [u8; 0] }

pub type u32_ = u32; pub type u64_ = u64; pub type u16_ = u16; pub type u8_ = u8;
pub type loff_t = i64; pub type phys_addr_t = u64; pub type size_t = usize;

pub const KFD_MAX_RING_ENTRY_SIZE: u32 = 8;
pub const KFD_SYSFS_FILE_MODE: u32 = 0o444;
pub const KFD_GPU_ID_HASH_WIDTH: u32 = 16;
pub const KFD_MMAP_TYPE_SHIFT: u32 = 62;
pub const KFD_MMAP_TYPE_MASK: u64 = 0x3u64 << KFD_MMAP_TYPE_SHIFT;
pub const KFD_MMAP_TYPE_DOORBELL: u64 = 0x3u64 << KFD_MMAP_TYPE_SHIFT;
pub const KFD_MMAP_TYPE_EVENTS: u64 = 0x2u64 << KFD_MMAP_TYPE_SHIFT;
pub const KFD_MMAP_TYPE_RESERVED_MEM: u64 = 0x1u64 << KFD_MMAP_TYPE_SHIFT;
pub const KFD_MMAP_TYPE_MMIO: u64 = 0;
pub const KFD_MMAP_GPU_ID_SHIFT: u32 = 46;
pub const KFD_MMAP_GPU_ID_MASK: u64 = ((1u64 << KFD_GPU_ID_HASH_WIDTH) - 1) << KFD_MMAP_GPU_ID_SHIFT;
#[inline] pub const fn KFD_MMAP_GPU_ID(gpu_id: u64) -> u64 { (gpu_id << KFD_MMAP_GPU_ID_SHIFT) & KFD_MMAP_GPU_ID_MASK }
#[inline] pub const fn KFD_MMAP_GET_GPU_ID(offset: u64) -> u64 { (offset & KFD_MMAP_GPU_ID_MASK) >> KFD_MMAP_GPU_ID_SHIFT }
pub const KFD_CIK_HIQ_PIPE: u32 = 4; pub const KFD_CIK_HIQ_QUEUE: u32 = 0;
pub const KFD_MAX_NUM_OF_PROCESSES: u32 = 512;
pub const KFD_MAX_NUM_OF_QUEUES_PER_PROCESS: u32 = 1024;
pub const KFD_KERNEL_QUEUE_SIZE: u32 = 2048;
pub const KFD_MAX_SDMA_QUEUES: u32 = 128;
pub const KFD_QUEUE_DOORBELL_MIRROR_OFFSET: u32 = 512;
pub const MAX_KFD_NODES: usize = 8;
pub const MAX_SYSFS_FILENAME_LEN: usize = 15;
pub const SDMA_ACTIVITY_DIVISOR: u32 = 100;
pub const KFD_PROCESS_TABLE_SIZE: u32 = 8;
pub const KFD_CONTEXT_ID_PRIMARY: u16 = 0xffff;
pub const KFD_CONTEXT_ID_MIN: u16 = 0;
pub const KFD_CRIU_PRIV_VERSION: u32 = 1;
pub const KFD_FENCE_COMPLETED: u32 = 100; pub const KFD_FENCE_INIT: u32 = 10;

#[repr(C)] pub struct kfd_event_interrupt_class {
    pub interrupt_isr: Option<unsafe extern "C" fn(*mut kfd_node, *const u32, *mut u32, *mut bool) -> bool>,
    pub interrupt_wq: Option<unsafe extern "C" fn(*mut kfd_node, *const u32)>,
}
#[repr(C)] pub struct kfd_device_info { pub gfx_target_version:u32, pub event_interrupt_class:*const kfd_event_interrupt_class, pub max_pasid_bits:core::ffi::c_uint, pub max_no_of_hqd:core::ffi::c_uint, pub doorbell_size:core::ffi::c_uint, pub ih_ring_entry_size:usize, pub num_of_watch_points:u8, pub mqd_size_aligned:u16, pub supports_cwsr:bool, pub needs_pci_atomics:bool, pub no_atomic_fw_version:u32, pub num_sdma_queues_per_engine:core::ffi::c_uint, pub num_reserved_sdma_queues_per_engine:core::ffi::c_uint }
#[repr(C)] pub struct kfd_mem_obj { pub range_start:u32, pub range_end:u32, pub gpu_addr:u64, pub cpu_ptr:*mut u32, pub mem:*mut c_void }
#[repr(C)] pub struct kfd_vmid_info { pub first_vmid_kfd:u32, pub last_vmid_kfd:u32, pub vmid_num_kfd:u32 }

pub enum cache_policy { cache_policy_coherent, cache_policy_noncoherent }
pub enum kfd_ioctl_flags { KFD_IOC_FLAG_CHECKPOINT_RESTORE = 1 }
pub enum kfd_mempool { KFD_MEMPOOL_SYSTEM_CACHEABLE=1, KFD_MEMPOOL_SYSTEM_WRITECOMBINE=2, KFD_MEMPOOL_FRAMEBUFFER=3 }
pub enum kfd_unmap_queues_filter { KFD_UNMAP_QUEUES_FILTER_ALL_QUEUES=1, KFD_UNMAP_QUEUES_FILTER_DYNAMIC_QUEUES=2, KFD_UNMAP_QUEUES_FILTER_BY_PASID=3 }
pub enum kfd_queue_type { KFD_QUEUE_TYPE_COMPUTE, KFD_QUEUE_TYPE_SDMA, KFD_QUEUE_TYPE_HIQ, KFD_QUEUE_TYPE_SDMA_XGMI, KFD_QUEUE_TYPE_SDMA_BY_ENG_ID, KFD_QUEUE_TYPE_MAX }
pub enum kfd_queue_format { KFD_QUEUE_FORMAT_PM4, KFD_QUEUE_FORMAT_AQL }
pub enum KFD_QUEUE_PRIORITY { KFD_QUEUE_PRIORITY_MINIMUM=0, KFD_QUEUE_PRIORITY_MAXIMUM=15 }
pub enum mqd_update_flag { UPDATE_FLAG_DBG_WA_ENABLE=1, UPDATE_FLAG_DBG_WA_DISABLE=2, UPDATE_FLAG_IS_GWS=4, UPDATE_FLAG_PERFCOUNT_ENABLE=5, UPDATE_FLAG_PERFCOUNT_DISABLE=6 }
pub enum KFD_MQD_TYPE { KFD_MQD_TYPE_HIQ=0, KFD_MQD_TYPE_CP, KFD_MQD_TYPE_SDMA, KFD_MQD_TYPE_DIQ, KFD_MQD_TYPE_MAX }
pub enum KFD_PIPE_PRIORITY { KFD_PIPE_PRIORITY_CS_LOW=0, KFD_PIPE_PRIORITY_CS_MEDIUM, KFD_PIPE_PRIORITY_CS_HIGH }
pub enum kfd_pdd_bound { PDD_UNBOUND=0, PDD_BOUND, PDD_BOUND_SUSPENDED }
pub enum kfd_criu_object_type { KFD_CRIU_OBJECT_TYPE_QUEUE, KFD_CRIU_OBJECT_TYPE_EVENT, KFD_CRIU_OBJECT_TYPE_SVM_RANGE }
pub enum kfd_config_dequeue_wait_counts_cmd { KFD_DEQUEUE_WAIT_INIT=1, KFD_DEQUEUE_WAIT_RESET=2, KFD_DEQUEUE_WAIT_SET_SCH_WAVE=3 }

#[repr(C)] pub struct queue_properties { pub type_:kfd_queue_type, pub format:kfd_queue_format, pub queue_id:u32, pub queue_address:u64, pub queue_size:u64, pub metadata_queue_size:u64, pub priority:u32, pub queue_percent:u32, pub read_ptr:*mut c_void, pub write_ptr:*mut c_void, pub doorbell_ptr:*mut c_void, pub doorbell_off:u32, pub is_interop:bool, pub is_evicted:bool, pub is_suspended:bool, pub is_being_destroyed:bool, pub is_active:bool, pub is_gws:bool, pub pm4_target_xcc:u32, pub is_dbg_wa:bool, pub is_user_cu_masked:bool, pub is_reset:bool, pub vmid:u32, pub sdma_engine_id:u32, pub sdma_queue_id:u32, pub sdma_vm_addr:u32, pub eop_ring_buffer_address:u64, pub eop_ring_buffer_size:u32, pub ctx_save_restore_area_address:u64, pub ctx_save_restore_area_size:u32, pub ctl_stack_size:u32, pub tba_addr:u64, pub tma_addr:u64, pub exception_status:u64, pub wptr_bo:*mut amdgpu_bo, pub rptr_bo:*mut amdgpu_bo, pub ring_bo:*mut amdgpu_bo, pub eop_buf_bo:*mut amdgpu_bo, pub cwsr_bo:*mut amdgpu_bo }
#[inline] pub unsafe fn QUEUE_IS_ACTIVE(q:&queue_properties)->bool { q.queue_size>0 && q.queue_address!=0 && q.queue_percent>0 && !q.is_evicted && !q.is_suspended }
#[repr(C)] pub struct mqd_update_info { pub cu_mask: cu_mask_union, pub update_flag:mqd_update_flag }
#[repr(C)] pub union cu_mask_union { pub cu_mask: cu_mask_fields }
#[repr(C)] pub struct cu_mask_fields { pub count:u32, pub ptr:*mut u32 }

#[repr(C)] pub struct scheduling_resources { pub vmid_mask:u32, pub type_:kfd_queue_type, pub queue_mask:u64, pub gws_mask:u64, pub oac_mask:u32, pub gds_heap_base:u32, pub gds_heap_size:u32 }
#[repr(C)] pub struct process_queue_manager { pub process:*mut kfd_process, pub queues:list_head, pub queue_slot_bitmap:*mut usize }
#[repr(C)] pub struct qcm_process_device { pub dqm:*mut device_queue_manager, pub pqm:*mut process_queue_manager, pub queues_list:list_head, pub priv_queue_list:list_head, pub queue_count:u32, pub vmid:u32, pub is_debug:bool, pub evicted:u32, pub reset_wavefronts:bool, pub mapped_gws_queue:bool, pub gds_context_area:u64, pub page_table_base:u64, pub sh_mem_config:u32, pub sh_mem_bases:u32, pub sh_mem_ape1_base:u32, pub sh_mem_ape1_limit:u32, pub gds_size:u32, pub num_gws:u32, pub num_oac:u32, pub sh_hidden_private_base:u32, pub vm_cntx_cntl:u32, pub cwsr_mem:*mut kgd_mem, pub cwsr_map:iosys_map, pub cwsr_base:u64, pub tba_addr:u64, pub tma_addr:u64, pub ib_mem:*mut kgd_mem, pub ib_base:u64, pub ib_kaddr:*mut c_void, pub proc_doorbells:*mut amdgpu_bo, pub doorbell_bitmap:*mut usize }
#[repr(C)] pub struct kfd_node { pub node_id:u32, pub adev:*mut amdgpu_device, pub kfd2kgd:*const kfd2kgd_calls, pub vm_info:kfd_vmid_info, pub id:u32, pub xcc_mask:u32, pub xcp:*mut amdgpu_xcp, pub ih_fifo:kfifo, pub interrupt_work:work_struct, pub interrupt_lock:spinlock_t, pub interrupts_active:bool, pub interrupt_bitmap:u32, pub dqm:*mut device_queue_manager, pub gws:*mut c_void, pub smi_clients:list_head, pub smi_lock:spinlock_t, pub reset_seq_num:u32, pub sram_ecc_flag:atomic_t, pub spm_pasid:u32, pub max_proc_per_quantum:u32, pub compute_vmid_bitmap:u32, pub local_mem_info:kfd_local_mem_info, pub kfd:*mut kfd_dev, pub alloc_watch_ids:u32, pub watch_points_lock:spinlock_t }
#[repr(C)] pub struct kfd_dev { pub adev:*mut amdgpu_device, pub device_info:kfd_device_info, pub doorbell_kernel_ptr:*mut u32, pub shared_resources:kgd2kfd_shared_resources, pub kfd2kgd:*const kfd2kgd_calls, pub doorbell_mutex:mutex, pub gtt_mem:*mut c_void, pub gtt_start_gpu_addr:u64, pub gtt_start_cpu_ptr:*mut c_void, pub gtt_sa_bitmap:*mut c_void, pub gtt_sa_lock:mutex, pub gtt_sa_chunk_size:u32, pub gtt_sa_num_of_chunks:u32, pub init_complete:bool, pub mec_fw_version:u16, pub mec2_fw_version:u16, pub sdma_fw_version:u16, pub cwsr_enabled:bool, pub cwsr_isa:*const c_void, pub cwsr_isa_size:u32, pub hive_id:u64, pub pci_atomic_requested:bool, pub compute_profile:atomic_t, pub doorbell_ida:ida, pub max_doorbell_slices:u32, pub noretry:i32, pub nodes:[*mut kfd_node;MAX_KFD_NODES], pub num_nodes:u32, pub ih_wq:*mut c_void, pub doorbells:*mut amdgpu_bo, pub doorbell_bitmap:*mut usize, pub kfd_dev_lock:i32, pub kfd_processes_count:atomic_t, pub profiler_lock:mutex, pub profiler_process:*mut kfd_process }

#[repr(C)] pub struct queue { pub list:list_head, pub mqd:*mut c_void, pub mqd_mem_obj:*mut kfd_mem_obj, pub gart_mqd_addr:u64, pub properties:queue_properties, pub mec:u32, pub pipe:u32, pub queue:u32, pub sdma_id:u32, pub doorbell_id:u32, pub process:*mut kfd_process, pub device:*mut kfd_node, pub gws:*mut c_void, pub kobj:kobject, pub gang_ctx_bo:*mut c_void, pub gang_ctx_gpu_addr:u64, pub gang_ctx_cpu_ptr:*mut c_void, pub gang_ctx_array_index:u32, pub wptr_bo_gart:*mut amdgpu_bo, pub needs_mqd_repin:bool }
#[repr(C)] pub struct kfd_process_device { pub dev:*mut kfd_node, pub process:*mut kfd_process, pub qpd:qcm_process_device, pub lds_base:u64, pub lds_limit:u64, pub gpuvm_base:u64, pub gpuvm_limit:u64, pub scratch_base:u64, pub scratch_limit:u64, pub drm_file:*mut file, pub drm_priv:*mut c_void, pub alloc_idr:idr, pub already_dequeued:bool, pub runtime_inuse:bool, pub bound:kfd_pdd_bound, pub vram_usage:atomic64_t, pub attr_vram:attribute, pub vram_filename:[u8;MAX_SYSFS_FILENAME_LEN], pub sdma_past_activity_counter:u64, pub attr_sdma:attribute, pub sdma_filename:[u8;MAX_SYSFS_FILENAME_LEN], pub last_evict_timestamp:u64, pub evict_duration_counter:atomic64_t, pub attr_evict:attribute, pub kobj_stats:*mut kobject, pub attr_cu_occupancy:attribute, pub kobj_counters:*mut kobject, pub attr_faults:attribute, pub attr_page_in:attribute, pub attr_page_out:attribute, pub faults:u64, pub page_in:u64, pub page_out:u64, pub exception_status:u64, pub vm_fault_exc_data:*mut c_void, pub vm_fault_exc_data_size:usize, pub spi_dbg_override:u32, pub spi_dbg_launch_mode:u32, pub watch_points:[u32;4], pub alloc_watch_ids:u32, pub user_gpu_id:u32, pub proc_ctx_bo:*mut c_void, pub proc_ctx_gpu_addr:u64, pub proc_ctx_cpu_ptr:*mut c_void, pub proc_ctx_array_index:u32, pub has_reset_queue:bool, pub pasid:u32, pub ptl_disable_req:bool }
#[repr(C)] pub struct svm_range_list { pub lock:mutex, pub objects:rb_root_cached, pub list:list_head, pub deferred_list_work:work_struct, pub deferred_range_list:list_head, pub criu_svm_metadata_list:list_head, pub deferred_list_lock:spinlock_t, pub evicted_ranges:atomic_t, pub drain_pagefaults:atomic_t, pub restore_work:delayed_work, pub bitmap_supported:[usize;8], pub faulting_task:*mut task_struct, pub checkpoint_ts:[atomic64_t;8], pub default_granularity:u8 }
#[repr(C)] pub struct kfd_process { pub kfd_processes:hlist_node, pub mm:*mut c_void, pub ref_:c_void, pub release_work:work_struct, pub mutex:mutex, pub lead_thread:*mut task_struct, pub mmu_notifier:mmu_notifier, pub pdds:*mut kfd_process_device, pub n_pdds:u32, pub pqm:process_queue_manager, pub is_32bit_user_mode:bool, pub event_mutex:mutex, pub event_idr:idr, pub signal_handle:u64, pub signal_page:*mut u64, pub signal_mapped_size:usize, pub signal_event_count:usize, pub signal_event_limit_reached:bool, pub kfd_sigbus_delay_ms:atomic_t, pub signal_work:delayed_work, pub kgd_process_info:*mut c_void, pub ef:*mut dma_fence, pub eviction_work:delayed_work, pub restore_work:delayed_work, pub last_eviction_seqno:u32, pub last_restore_timestamp:usize, pub debug_trap_enabled:bool, pub dbg_ev_file:*mut file, pub debugged_process_count:atomic_t, pub debugger_process:*mut kfd_process, pub kobj:*mut kobject, pub kobj_queues:*mut kobject, pub attr_pasid:attribute, pub exception_enable_mask:u64, pub exception_status:u64, pub wait_irq_drain:wait_queue_head_t, pub irq_drain_is_open:bool, pub svms:svm_range_list, pub xnack_enabled:bool, pub debug_event_workarea:work_struct, pub dbg_flags:u32, pub poison:atomic_t, pub queues_paused:bool, pub runtime_enable_sema:semaphore, pub is_runtime_retry:bool, pub runtime_info:c_void, pub gpu_page_fault:bool, pub context_id:u16, pub id_table:ida }

#[repr(C)] pub struct kfd_criu_process_priv_data { pub version:u32, pub xnack_mode:u32 }
#[repr(C)] pub struct kfd_criu_device_priv_data { pub reserved:u64 }
#[repr(C)] pub struct kfd_criu_bo_priv_data { pub user_addr:u64, pub idr_handle:u32, pub mapped_gpuids:[u32;8] }
#[repr(C)] pub struct kfd_criu_svm_range_priv_data { pub object_type:u32, pub start_addr:u64, pub size:u64, pub attrs:[kfd_ioctl_svm_attribute;0] }
#[repr(C)] pub struct kfd_criu_queue_priv_data { pub object_type:u32, pub q_address:u64, pub q_size:u64, pub read_ptr_addr:u64, pub write_ptr_addr:u64, pub doorbell_off:u64, pub eop_ring_buffer_address:u64, pub ctx_save_restore_area_address:u64, pub gpu_id:u32, pub type_:u32, pub format:u32, pub q_id:u32, pub priority:u32, pub q_percent:u32, pub doorbell_id:u32, pub gws:u32, pub sdma_id:u32, pub eop_ring_buffer_size:u32, pub ctx_save_restore_area_size:u32, pub ctl_stack_size:u32, pub mqd_size:u32 }
#[repr(C)] pub union kfd_criu_event_exception { pub memory_exception_data:kfd_hsa_memory_exception_data, pub hw_exception_data:kfd_hsa_hw_exception_data }
#[repr(C)] pub struct kfd_criu_event_priv_data { pub object_type:u32, pub user_handle:u64, pub event_id:u32, pub auto_reset:u32, pub type_:u32, pub signaled:u32, pub exception:kfd_criu_event_exception }

#[repr(C)] pub struct packet_manager { pub dqm:*mut device_queue_manager, pub priv_queue:*mut kernel_queue, pub lock:mutex, pub allocated:bool, pub ib_buffer_obj:*mut kfd_mem_obj, pub ib_size_bytes:u32, pub is_over_subscription:bool, pub pmf:*const packet_manager_funcs }
#[repr(C)] pub struct packet_manager_funcs { pub map_process:Option<unsafe extern "C" fn(*mut packet_manager,*mut u32,*mut qcm_process_device)->i32>, pub runlist:Option<unsafe extern "C" fn(*mut packet_manager,*mut u32,u64,usize,bool)->i32>, pub set_resources:Option<unsafe extern "C" fn(*mut packet_manager,*mut u32,*mut scheduling_resources)->i32>, pub map_queues:Option<unsafe extern "C" fn(*mut packet_manager,*mut u32,*mut queue,bool)->i32>, pub unmap_queues:Option<unsafe extern "C" fn(*mut packet_manager,*mut u32,kfd_unmap_queues_filter,u32,bool)->i32>, pub config_dequeue_wait_counts:Option<unsafe extern "C" fn(*mut packet_manager,*mut u32,kfd_config_dequeue_wait_counts_cmd,u32)->i32>, pub query_status:Option<unsafe extern "C" fn(*mut packet_manager,*mut u32,u64,u64)->i32>, pub release_mem:Option<unsafe extern "C" fn(u64,*mut u32)->i32>, pub map_process_size:i32, pub runlist_size:i32, pub set_resources_size:i32, pub map_queues_size:i32, pub unmap_queues_size:i32, pub config_dequeue_wait_counts_size:i32, pub query_status_size:i32, pub release_mem_size:i32 }

/* External declarations corresponding to all functions/globals in the header. */
extern "C" {
    pub static mut max_num_of_queues_per_device:i32; pub static mut sched_policy:i32; pub static mut hws_max_conc_proc:i32; pub static mut cwsr_enable:i32; pub static mut send_sigterm:i32; pub static mut debug_largebar:i32; pub static mut amdgpu_noretry:i32; pub static mut halt_if_hws_hang:i32; pub static mut hws_gws_support:bool; pub static mut queue_preemption_timeout_ms:i32; pub static mut amdgpu_no_queue_eviction_on_vm_fault:i32; pub static mut debug_evictions:bool; pub static mut kfd_processes_mutex:mutex; pub static mut kfd_device:*mut device;
    pub fn kfd_get_num_sdma_engines(kdev:*mut kfd_node)->u32; pub fn kfd_get_num_xgmi_sdma_engines(kdev:*mut kfd_node)->u32;
    pub fn kfd_chardev_init()->i32; pub fn kfd_chardev_exit(); pub fn kfd_dev_unmap_mapping_range(holebegin:loff_t,holelen:loff_t);
    pub fn kfd_process_gpuidx_from_gpuid(p:*mut kfd_process,gpu_id:u32)->i32; pub fn kfd_unref_process(p:*mut kfd_process); pub fn kfd_process_evict_queues(p:*mut kfd_process,trigger:u32)->i32; pub fn kfd_process_restore_queues(p:*mut kfd_process)->i32;
    pub fn kfd_topology_init()->i32; pub fn kfd_topology_shutdown(); pub fn kfd_device_by_id(gpu_id:u32)->*mut kfd_node; pub fn kfd_topology_get_num_devices()->u32;
    pub fn enqueue_ih_ring_entry(kfd:*mut kfd_node,ih_ring_entry:*const c_void)->bool; pub fn kfd_interrupt_init(dev:*mut kfd_node)->i32; pub fn kfd_interrupt_exit(dev:*mut kfd_node);
    pub fn init_queue(q:*mut *mut queue,properties:*const queue_properties)->i32; pub fn uninit_queue(q:*mut queue); pub fn print_queue_properties(q:*mut queue); pub fn print_queue(q:*mut queue);
    pub fn pm_init(pm:*mut packet_manager,dqm:*mut device_queue_manager)->i32; pub fn pm_uninit(pm:*mut packet_manager); pub fn pm_release_ib(pm:*mut packet_manager);
}

#[inline] pub unsafe fn kfd_process_gpuid_from_gpuidx(p:*mut kfd_process,gpuidx:u32,gpuid:*mut u32)->i32 { if gpuidx < (*p).n_pdds { *gpuid=(*(*p).pdds.add(gpuidx as usize)).dev.as_ref().unwrap().id; 0 } else { -22 } }
#[inline] pub unsafe fn kfd_process_device_from_gpuidx(p:*mut kfd_process,gpuidx:u32)->*mut kfd_process_device { if gpuidx < (*p).n_pdds { (*p).pdds.add(gpuidx as usize) } else { core::ptr::null_mut() } }
#[inline] pub unsafe fn kfd_irq_is_from_node(node:*mut kfd_node,node_id:u32,vmid:u32)->bool { ((*node).interrupt_bitmap & (1u32<<node_id)) != 0 && ((*node).compute_vmid_bitmap & (1u32<<vmid)) != 0 }
#[inline] pub unsafe fn kfd_is_first_node(node:*mut kfd_node)->bool { node == (*node).kfd.as_ref().unwrap().nodes[0] }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
