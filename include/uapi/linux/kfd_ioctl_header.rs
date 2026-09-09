/* Rust translation of kfd_ioctl.h. */

use core::ffi::c_void;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s32 = i32;

pub const KFD_IOCTL_MAJOR_VERSION: u32 = 1;
pub const KFD_IOCTL_MINOR_VERSION: u32 = 23;

#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_get_version_args { pub major_version: __u32, pub minor_version: __u32 }
pub const KFD_IOC_QUEUE_TYPE_COMPUTE:u32=0; pub const KFD_IOC_QUEUE_TYPE_SDMA:u32=1; pub const KFD_IOC_QUEUE_TYPE_COMPUTE_AQL:u32=2; pub const KFD_IOC_QUEUE_TYPE_SDMA_XGMI:u32=3; pub const KFD_IOC_QUEUE_TYPE_SDMA_BY_ENG_ID:u32=4;
pub const KFD_MAX_QUEUE_PERCENTAGE:u32=100; pub const KFD_MAX_QUEUE_PRIORITY:u32=15; pub const KFD_MIN_QUEUE_RING_SIZE:u32=1024;
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_create_queue_args { pub ring_base_address:__u64,pub write_pointer_address:__u64,pub read_pointer_address:__u64,pub doorbell_offset:__u64,pub ring_size:__u32,pub gpu_id:__u32,pub queue_type:__u32,pub queue_percentage:__u32,pub queue_priority:__u32,pub queue_id:__u32,pub eop_buffer_address:__u64,pub eop_buffer_size:__u64,pub ctx_save_restore_address:__u64,pub ctx_save_restore_size:__u32,pub ctl_stack_size:__u32,pub sdma_engine_id:__u32,pub metadata_ring_size:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_destroy_queue_args { pub queue_id:__u32,pub pad:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_update_queue_args { pub ring_base_address:__u64,pub queue_id:__u32,pub ring_size:__u32,pub queue_percentage:__u32,pub queue_priority:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_set_cu_mask_args { pub queue_id:__u32,pub num_cu_mask:__u32,pub cu_mask_ptr:__u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_get_queue_wave_state_args { pub ctl_stack_address:__u64,pub ctl_stack_used_size:__u32,pub save_area_used_size:__u32,pub queue_id:__u32,pub pad:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_get_available_memory_args { pub available:__u64,pub gpu_id:__u32,pub pad:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_dbg_device_info_entry { pub exception_status:__u64,pub lds_base:__u64,pub lds_limit:__u64,pub scratch_base:__u64,pub scratch_limit:__u64,pub gpuvm_base:__u64,pub gpuvm_limit:__u64,pub gpu_id:__u32,pub location_id:__u32,pub vendor_id:__u32,pub device_id:__u32,pub revision_id:__u32,pub subsystem_vendor_id:__u32,pub subsystem_device_id:__u32,pub fw_version:__u32,pub gfx_target_version:__u32,pub simd_count:__u32,pub max_waves_per_simd:__u32,pub array_count:__u32,pub simd_arrays_per_engine:__u32,pub num_xcc:__u32,pub capability:__u32,pub debug_prop:__u32,pub capability2:__u32,pub pad:__u32 }
pub const KFD_IOC_CACHE_POLICY_COHERENT:u32=0; pub const KFD_IOC_CACHE_POLICY_NONCOHERENT:u32=1; pub const KFD_PROC_FLAG_MFMA_HIGH_PRECISION:u32=1;
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_set_memory_policy_args { pub alternate_aperture_base:__u64,pub alternate_aperture_size:__u64,pub gpu_id:__u32,pub default_policy:__u32,pub alternate_policy:__u32,pub misc_process_flag:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_get_clock_counters_args { pub gpu_clock_counter:__u64,pub cpu_clock_counter:__u64,pub system_clock_counter:__u64,pub system_clock_freq:__u64,pub gpu_id:__u32,pub pad:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_process_device_apertures { pub lds_base:__u64,pub lds_limit:__u64,pub scratch_base:__u64,pub scratch_limit:__u64,pub gpuvm_base:__u64,pub gpuvm_limit:__u64,pub gpu_id:__u32,pub pad:__u32 }
pub const NUM_OF_SUPPORTED_GPUS:usize=7;
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_get_process_apertures_args { pub process_apertures:[kfd_process_device_apertures;NUM_OF_SUPPORTED_GPUS],pub num_of_nodes:__u32,pub pad:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_get_process_apertures_new_args { pub kfd_process_device_apertures_ptr:__u64,pub num_of_nodes:__u32,pub pad:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_dbg_register_args { pub gpu_id:__u32,pub pad:__u32 }
pub type kfd_ioctl_dbg_unregister_args=kfd_ioctl_dbg_register_args;
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_dbg_address_watch_args { pub content_ptr:__u64,pub gpu_id:__u32,pub buf_size_in_bytes:__u32 }
pub type kfd_ioctl_dbg_wave_control_args=kfd_ioctl_dbg_address_watch_args;
pub const KFD_INVALID_FD:u32=0xffff_ffff; pub const KFD_SIGNAL_EVENT_LIMIT:u32=4096;
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_create_event_args { pub event_page_offset:__u64,pub event_trigger_data:__u32,pub event_type:__u32,pub auto_reset:__u32,pub node_id:__u32,pub event_id:__u32,pub event_slot_index:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_destroy_event_args { pub event_id:__u32,pub pad:__u32 }
pub type kfd_ioctl_set_event_args=kfd_ioctl_destroy_event_args; pub type kfd_ioctl_reset_event_args=kfd_ioctl_destroy_event_args;
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_memory_exception_failure { pub NotPresent:__u32,pub ReadOnly:__u32,pub NoExecute:__u32,pub imprecise:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_hsa_memory_exception_data { pub failure:kfd_memory_exception_failure,pub va:__u64,pub gpu_id:__u32,pub ErrorType:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_hsa_hw_exception_data { pub reset_type:__u32,pub reset_cause:__u32,pub memory_lost:__u32,pub gpu_id:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_hsa_signal_event_data { pub last_event_age:__u64 }
#[repr(C)] pub union kfd_event_data_union { pub memory_exception_data:kfd_hsa_memory_exception_data,pub hw_exception_data:kfd_hsa_hw_exception_data,pub signal_event_data:kfd_hsa_signal_event_data }
#[repr(C)] pub struct kfd_event_data { pub data:kfd_event_data_union,pub kfd_event_data_ext:__u64,pub event_id:__u32,pub pad:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_wait_events_args { pub events_ptr:__u64,pub num_events:__u32,pub wait_for_all:__u32,pub timeout:__u32,pub wait_result:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_set_scratch_backing_va_args { pub va_addr:__u64,pub gpu_id:__u32,pub pad:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_set_trap_handler_args { pub tba_addr:__u64,pub tma_addr:__u64,pub gpu_id:__u32,pub pad:__u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_ioctl_acquire_vm_args { pub drm_fd:__u32,pub gpu_id:__u32 }
pub const KFD_IOC_ALLOC_MEM_FLAGS_VRAM:u32=1<<0; pub const KFD_IOC_ALLOC_MEM_FLAGS_GTT:u32=1<<1; pub const KFD_IOC_ALLOC_MEM_FLAGS_USERPTR:u32=1<<2; pub const KFD_IOC_ALLOC_MEM_FLAGS_DOORBELL:u32=1<<3; pub const KFD_IOC_ALLOC_MEM_FLAGS_MMIO_REMAP:u32=1<<4; pub const KFD_IOC_ALLOC_MEM_FLAGS_WRITABLE:u32=1<<31; pub const KFD_IOC_ALLOC_MEM_FLAGS_EXECUTABLE:u32=1<<30; pub const KFD_IOC_ALLOC_MEM_FLAGS_PUBLIC:u32=1<<29; pub const KFD_IOC_ALLOC_MEM_FLAGS_NO_SUBSTITUTE:u32=1<<28; pub const KFD_IOC_ALLOC_MEM_FLAGS_AQL_QUEUE_MEM:u32=1<<27; pub const KFD_IOC_ALLOC_MEM_FLAGS_COHERENT:u32=1<<26; pub const KFD_IOC_ALLOC_MEM_FLAGS_UNCACHED:u32=1<<25; pub const KFD_IOC_ALLOC_MEM_FLAGS_EXT_COHERENT:u32=1<<24; pub const KFD_IOC_ALLOC_MEM_FLAGS_CONTIGUOUS:u32=1<<23;
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_alloc_memory_of_gpu_args { pub va_addr:__u64,pub size:__u64,pub handle:__u64,pub mmap_offset:__u64,pub gpu_id:__u32,pub flags:__u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_free_memory_of_gpu_args { pub handle:__u64 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_map_memory_to_gpu_args { pub handle:__u64,pub device_ids_array_ptr:__u64,pub n_devices:__u32,pub n_success:__u32 }
pub type kfd_ioctl_unmap_memory_from_gpu_args=kfd_ioctl_map_memory_to_gpu_args;
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_alloc_queue_gws_args { pub queue_id:__u32,pub num_gws:__u32,pub first_gws:__u32,pub pad:__u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_get_dmabuf_info_args { pub size:__u64,pub metadata_ptr:__u64,pub metadata_size:__u32,pub gpu_id:__u32,pub flags:__u32,pub dmabuf_fd:__u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_import_dmabuf_args { pub va_addr:__u64,pub handle:__u64,pub gpu_id:__u32,pub dmabuf_fd:__u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_export_dmabuf_args { pub handle:__u64,pub flags:__u32,pub dmabuf_fd:__u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_smi_events_args { pub gpuid:__u32,pub anon_fd:__u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_set_xnack_mode_args { pub xnack_enabled:__s32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_svm_attribute { pub type_:__u32,pub value:__u32 }
#[repr(C)] pub struct kfd_ioctl_svm_args { pub start_addr:__u64,pub size:__u64,pub op:__u32,pub nattr:__u32,pub attrs:[kfd_ioctl_svm_attribute;0] }
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_criu_device_bucket { pub user_gpu_id:__u32,pub actual_gpu_id:__u32,pub drm_fd:__u32,pub pad:__u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_criu_bo_bucket { pub addr:__u64,pub size:__u64,pub offset:__u64,pub restored_offset:__u64,pub gpu_id:__u32,pub alloc_flags:__u32,pub dmabuf_fd:__u32,pub pad:__u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_criu_args { pub devices:__u64,pub bos:__u64,pub priv_data:__u64,pub priv_data_size:__u64,pub num_devices:__u32,pub num_bos:__u32,pub num_objects:__u32,pub pid:__u32,pub op:__u32 }
pub const KFD_IOCTL_SVM_FLAG_HOST_ACCESS:u32=1; pub const KFD_IOCTL_SVM_FLAG_COHERENT:u32=2; pub const KFD_IOCTL_SVM_FLAG_HIVE_LOCAL:u32=4; pub const KFD_IOCTL_SVM_FLAG_GPU_RO:u32=8; pub const KFD_IOCTL_SVM_FLAG_GPU_EXEC:u32=16; pub const KFD_IOCTL_SVM_FLAG_GPU_READ_MOSTLY:u32=32; pub const KFD_IOCTL_SVM_FLAG_GPU_ALWAYS_MAPPED:u32=64; pub const KFD_IOCTL_SVM_FLAG_EXT_COHERENT:u32=128;
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_runtime_enable_args { pub r_debug:__u64,pub mode_mask:__u32,pub capabilities_mask:__u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_queue_snapshot_entry { pub exception_status:__u64,pub ring_base_address:__u64,pub write_pointer_address:__u64,pub read_pointer_address:__u64,pub ctx_save_restore_address:__u64,pub queue_id:__u32,pub gpu_id:__u32,pub ring_size:__u32,pub queue_type:__u32,pub ctx_save_restore_area_size:__u32,pub reserved:__u32 }
pub const KFD_DBG_QUEUE_ERROR_BIT:u32=30; pub const KFD_DBG_QUEUE_INVALID_BIT:u32=31; pub const KFD_DBG_QUEUE_ERROR_MASK:u32=1<<30; pub const KFD_DBG_QUEUE_INVALID_MASK:u32=1<<31;
#[repr(C)] #[derive(Copy,Clone)] pub struct kfd_ioctl_profiler_args { pub op:__u32,pub data:[__u8;12] }
pub type __u8=u8;
/* ioctl encodings depend on the platform's Linux _IOC ABI; retain the source declarations as named constants. */
pub const AMDKFD_IOCTL_BASE:u8=b'K'; pub const AMDKFD_COMMAND_START:u32=0x01; pub const AMDKFD_COMMAND_END:u32=0x29;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
