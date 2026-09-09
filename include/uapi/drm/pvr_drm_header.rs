/* SPDX-License-Identifier: (GPL-2.0-only WITH Linux-syscall-note) OR MIT */
/* Rust translation of pvr_drm.h. External DRM constants/types are supplied by dependencies. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_pvr_obj_array { pub stride: u32, pub count: u32, pub array: u64 }

#[macro_export]
macro_rules! DRM_PVR_OBJ_ARRAY { ($cnt:expr, $ptr:expr) => { $crate::drm_pvr_obj_array { stride: core::mem::size_of_val(&($ptr)[0]) as u32, count: $cnt, array: ($ptr as *const _ as usize) as u64 } }; }

/* PVR_IOCTL uses the externally supplied DRM_IO* and DRM_COMMAND_BASE definitions. */
#[macro_export]
macro_rules! PVR_IOCTL { ($ioctl:expr, $mode:ident, $data:ty) => { $mode(DRM_COMMAND_BASE + ($ioctl), $data) }; }

#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_dev_query_gpu_info { pub gpu_id:u64, pub num_phantoms:u32, pub _padding_c:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_dev_query_runtime_info { pub free_list_min_pages:u64, pub free_list_max_pages:u64, pub common_store_alloc_region_size:u32, pub common_store_partition_space_size:u32, pub max_coeffs:u32, pub cdm_max_local_mem_size_regs:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_dev_query_quirks { pub quirks:u64, pub count:u16, pub musthave_count:u16, pub _padding_c:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_dev_query_enhancements { pub enhancements:u64, pub count:u16, pub _padding_a:u16, pub _padding_c:u32 }

#[repr(u32)] #[derive(Copy, Clone)] pub enum drm_pvr_heap_id { DRM_PVR_HEAP_GENERAL=0, DRM_PVR_HEAP_PDS_CODE_DATA, DRM_PVR_HEAP_USC_CODE, DRM_PVR_HEAP_RGNHDR, DRM_PVR_HEAP_VIS_TEST, DRM_PVR_HEAP_TRANSFER_FRAG, DRM_PVR_HEAP_COUNT }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_heap { pub base:u64, pub size:u64, pub flags:u32, pub page_size_log2:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_dev_query_heap_info { pub heaps:drm_pvr_obj_array }
#[repr(u32)] #[derive(Copy, Clone)] pub enum drm_pvr_static_data_area_usage { DRM_PVR_STATIC_DATA_AREA_EOT=0, DRM_PVR_STATIC_DATA_AREA_FENCE, DRM_PVR_STATIC_DATA_AREA_VDM_SYNC, DRM_PVR_STATIC_DATA_AREA_YUV_CSC }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_static_data_area { pub area_usage:u16, pub location_heap_id:u16, pub size:u32, pub offset:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_dev_query_static_data_areas { pub static_data_areas:drm_pvr_obj_array }
#[repr(u32)] #[derive(Copy, Clone)] pub enum drm_pvr_dev_query { DRM_PVR_DEV_QUERY_GPU_INFO_GET=0, DRM_PVR_DEV_QUERY_RUNTIME_INFO_GET, DRM_PVR_DEV_QUERY_QUIRKS_GET, DRM_PVR_DEV_QUERY_ENHANCEMENTS_GET, DRM_PVR_DEV_QUERY_HEAP_INFO_GET, DRM_PVR_DEV_QUERY_STATIC_DATA_AREAS_GET }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_dev_query_args { pub type_:u32, pub size:u32, pub pointer:u64 }

pub const DRM_PVR_BO_BYPASS_DEVICE_CACHE:u64=1u64<<0; pub const DRM_PVR_BO_PM_FW_PROTECT:u64=1u64<<1; pub const DRM_PVR_BO_ALLOW_CPU_USERSPACE_ACCESS:u64=1u64<<2;
pub const DRM_PVR_BO_FLAGS_MASK:u64=DRM_PVR_BO_BYPASS_DEVICE_CACHE|DRM_PVR_BO_PM_FW_PROTECT|DRM_PVR_BO_ALLOW_CPU_USERSPACE_ACCESS;
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_create_bo_args { pub size:u64, pub handle:u32, pub _padding_c:u32, pub flags:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_get_bo_mmap_offset_args { pub handle:u32, pub _padding_4:u32, pub offset:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_create_vm_context_args { pub handle:u32, pub _padding_4:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_destroy_vm_context_args { pub handle:u32, pub _padding_4:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_vm_map_args { pub vm_context_handle:u32, pub flags:u32, pub device_addr:u64, pub handle:u32, pub _padding_14:u32, pub offset:u64, pub size:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_vm_unmap_args { pub vm_context_handle:u32, pub _padding_4:u32, pub device_addr:u64, pub size:u64 }
#[repr(i32)] #[derive(Copy, Clone)] pub enum drm_pvr_ctx_priority { DRM_PVR_CTX_PRIORITY_LOW=-512, DRM_PVR_CTX_PRIORITY_NORMAL=0, DRM_PVR_CTX_PRIORITY_HIGH=512 }
#[repr(u32)] #[derive(Copy, Clone)] pub enum drm_pvr_ctx_type { DRM_PVR_CTX_TYPE_RENDER=0, DRM_PVR_CTX_TYPE_COMPUTE, DRM_PVR_CTX_TYPE_TRANSFER_FRAG }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_create_context_args { pub type_:u32, pub flags:u32, pub priority:i32, pub handle:u32, pub static_context_state:u64, pub static_context_state_len:u32, pub vm_context_handle:u32, pub callstack_addr:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_destroy_context_args { pub handle:u32, pub _padding_4:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_create_free_list_args { pub free_list_gpu_addr:u64, pub initial_num_pages:u32, pub max_num_pages:u32, pub grow_num_pages:u32, pub grow_threshold:u32, pub vm_context_handle:u32, pub handle:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_destroy_free_list_args { pub handle:u32, pub _padding_4:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_create_hwrt_geom_data_args { pub tpc_dev_addr:u64, pub tpc_size:u32, pub tpc_stride:u32, pub vheap_table_dev_addr:u64, pub rtc_dev_addr:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_create_hwrt_rt_data_args { pub pm_mlist_dev_addr:u64, pub macrotile_array_dev_addr:u64, pub region_header_dev_addr:u64 }
pub const PVR_DRM_HWRT_FREE_LIST_LOCAL:u32=0; pub const PVR_DRM_HWRT_FREE_LIST_GLOBAL:u32=1;
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_create_hwrt_dataset_args { pub geom_data_args:drm_pvr_create_hwrt_geom_data_args, pub rt_data_args:[drm_pvr_create_hwrt_rt_data_args;2], pub free_list_handles:[u32;2], pub width:u32, pub height:u32, pub samples:u32, pub layers:u32, pub isp_merge_lower_x:u32, pub isp_merge_lower_y:u32, pub isp_merge_scale_x:u32, pub isp_merge_scale_y:u32, pub isp_merge_upper_x:u32, pub isp_merge_upper_y:u32, pub region_header_size:u32, pub handle:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_destroy_hwrt_dataset_args { pub handle:u32, pub _padding_4:u32 }

pub const DRM_PVR_SYNC_OP_FLAG_HANDLE_TYPE_MASK:u32=0xf; pub const DRM_PVR_SYNC_OP_FLAG_HANDLE_TYPE_SYNCOBJ:u32=0; pub const DRM_PVR_SYNC_OP_FLAG_HANDLE_TYPE_TIMELINE_SYNCOBJ:u32=1; pub const DRM_PVR_SYNC_OP_FLAG_SIGNAL:u32=1u32<<31; pub const DRM_PVR_SYNC_OP_FLAG_WAIT:u32=0; pub const DRM_PVR_SYNC_OP_FLAGS_MASK:u32=DRM_PVR_SYNC_OP_FLAG_HANDLE_TYPE_MASK|DRM_PVR_SYNC_OP_FLAG_SIGNAL;
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_sync_op { pub handle:u32, pub flags:u32, pub value:u64 }
pub const DRM_PVR_SUBMIT_JOB_GEOM_CMD_FIRST:u64=1<<0; pub const DRM_PVR_SUBMIT_JOB_GEOM_CMD_LAST:u64=1<<1; pub const DRM_PVR_SUBMIT_JOB_GEOM_CMD_SINGLE_CORE:u64=1<<2; pub const DRM_PVR_SUBMIT_JOB_GEOM_CMD_FLAGS_MASK:u64=7;
pub const DRM_PVR_SUBMIT_JOB_FRAG_CMD_SINGLE_CORE:u64=1<<0; pub const DRM_PVR_SUBMIT_JOB_FRAG_CMD_DEPTHBUFFER:u64=1<<1; pub const DRM_PVR_SUBMIT_JOB_FRAG_CMD_STENCILBUFFER:u64=1<<2; pub const DRM_PVR_SUBMIT_JOB_FRAG_CMD_PREVENT_CDM_OVERLAP:u64=1<<3; pub const DRM_PVR_SUBMIT_JOB_FRAG_CMD_SCRATCHBUFFER:u64=1<<4; pub const DRM_PVR_SUBMIT_JOB_FRAG_CMD_GET_VIS_RESULTS:u64=1<<5; pub const DRM_PVR_SUBMIT_JOB_FRAG_CMD_PARTIAL_RENDER:u64=1<<6; pub const DRM_PVR_SUBMIT_JOB_FRAG_CMD_DISABLE_PIXELMERGE:u64=1<<7; pub const DRM_PVR_SUBMIT_JOB_FRAG_CMD_FLAGS_MASK:u64=0xff;
pub const DRM_PVR_SUBMIT_JOB_COMPUTE_CMD_PREVENT_ALL_OVERLAP:u64=1; pub const DRM_PVR_SUBMIT_JOB_COMPUTE_CMD_SINGLE_CORE:u64=2; pub const DRM_PVR_SUBMIT_JOB_COMPUTE_CMD_FLAGS_MASK:u64=3; pub const DRM_PVR_SUBMIT_JOB_TRANSFER_CMD_SINGLE_CORE:u64=1; pub const DRM_PVR_SUBMIT_JOB_TRANSFER_CMD_FLAGS_MASK:u64=1;
#[repr(u32)] #[derive(Copy, Clone)] pub enum drm_pvr_job_type { DRM_PVR_JOB_TYPE_GEOMETRY=0, DRM_PVR_JOB_TYPE_FRAGMENT, DRM_PVR_JOB_TYPE_COMPUTE, DRM_PVR_JOB_TYPE_TRANSFER_FRAG }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_hwrt_data_ref { pub set_handle:u32, pub data_index:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_job { pub type_:u32, pub context_handle:u32, pub flags:u32, pub cmd_stream_len:u32, pub cmd_stream:u64, pub sync_ops:drm_pvr_obj_array, pub hwrt:drm_pvr_hwrt_data_ref }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_pvr_ioctl_submit_jobs_args { pub jobs:drm_pvr_obj_array }

/* IOCTL constants retain the source macro's dependency on DRM_IOWR/DRM_IOW. */
macro_rules! DRM_IOCTL_PVR_DEV_QUERY { () => { PVR_IOCTL!(0x00, DRM_IOWR, drm_pvr_ioctl_dev_query_args) }; }
macro_rules! DRM_IOCTL_PVR_CREATE_BO { () => { PVR_IOCTL!(0x01, DRM_IOWR, drm_pvr_ioctl_create_bo_args) }; }
macro_rules! DRM_IOCTL_PVR_GET_BO_MMAP_OFFSET { () => { PVR_IOCTL!(0x02, DRM_IOWR, drm_pvr_ioctl_get_bo_mmap_offset_args) }; }
macro_rules! DRM_IOCTL_PVR_CREATE_VM_CONTEXT { () => { PVR_IOCTL!(0x03, DRM_IOWR, drm_pvr_ioctl_create_vm_context_args) }; }
macro_rules! DRM_IOCTL_PVR_DESTROY_VM_CONTEXT { () => { PVR_IOCTL!(0x04, DRM_IOW, drm_pvr_ioctl_destroy_vm_context_args) }; }
macro_rules! DRM_IOCTL_PVR_VM_MAP { () => { PVR_IOCTL!(0x05, DRM_IOW, drm_pvr_ioctl_vm_map_args) }; }
macro_rules! DRM_IOCTL_PVR_VM_UNMAP { () => { PVR_IOCTL!(0x06, DRM_IOW, drm_pvr_ioctl_vm_unmap_args) }; }
macro_rules! DRM_IOCTL_PVR_CREATE_CONTEXT { () => { PVR_IOCTL!(0x07, DRM_IOWR, drm_pvr_ioctl_create_context_args) }; }
macro_rules! DRM_IOCTL_PVR_DESTROY_CONTEXT { () => { PVR_IOCTL!(0x08, DRM_IOW, drm_pvr_ioctl_destroy_context_args) }; }
macro_rules! DRM_IOCTL_PVR_CREATE_FREE_LIST { () => { PVR_IOCTL!(0x09, DRM_IOWR, drm_pvr_ioctl_create_free_list_args) }; }
macro_rules! DRM_IOCTL_PVR_DESTROY_FREE_LIST { () => { PVR_IOCTL!(0x0a, DRM_IOW, drm_pvr_ioctl_destroy_free_list_args) }; }
macro_rules! DRM_IOCTL_PVR_CREATE_HWRT_DATASET { () => { PVR_IOCTL!(0x0b, DRM_IOWR, drm_pvr_ioctl_create_hwrt_dataset_args) }; }
macro_rules! DRM_IOCTL_PVR_DESTROY_HWRT_DATASET { () => { PVR_IOCTL!(0x0c, DRM_IOW, drm_pvr_ioctl_destroy_hwrt_dataset_args) }; }
macro_rules! DRM_IOCTL_PVR_SUBMIT_JOBS { () => { PVR_IOCTL!(0x0d, DRM_IOW, drm_pvr_ioctl_submit_jobs_args) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
