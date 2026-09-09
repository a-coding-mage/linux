/* Translated from v3d_drm.h. */

pub const DRM_V3D_SUBMIT_CL: u32 = 0x00;
pub const DRM_V3D_WAIT_BO: u32 = 0x01;
pub const DRM_V3D_CREATE_BO: u32 = 0x02;
pub const DRM_V3D_MMAP_BO: u32 = 0x03;
pub const DRM_V3D_GET_PARAM: u32 = 0x04;
pub const DRM_V3D_GET_BO_OFFSET: u32 = 0x05;
pub const DRM_V3D_SUBMIT_TFU: u32 = 0x06;
pub const DRM_V3D_SUBMIT_CSD: u32 = 0x07;
pub const DRM_V3D_PERFMON_CREATE: u32 = 0x08;
pub const DRM_V3D_PERFMON_DESTROY: u32 = 0x09;
pub const DRM_V3D_PERFMON_GET_VALUES: u32 = 0x0a;
pub const DRM_V3D_SUBMIT_CPU: u32 = 0x0b;
pub const DRM_V3D_PERFMON_GET_COUNTER: u32 = 0x0c;
pub const DRM_V3D_PERFMON_SET_GLOBAL: u32 = 0x0d;

// DRM_IOCTL_* definitions depend on DRM_IOWR/DRM_IOW and DRM_COMMAND_BASE from drm.h.
pub const DRM_V3D_SUBMIT_CL_FLUSH_CACHE: u32 = 0x01;
pub const DRM_V3D_SUBMIT_EXTENSION: u32 = 0x02;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_extension { pub next: __u64, pub id: __u32, pub flags: __u32 }
pub const DRM_V3D_EXT_ID_MULTI_SYNC: u32 = 0x01;
pub const DRM_V3D_EXT_ID_CPU_INDIRECT_CSD: u32 = 0x02;
pub const DRM_V3D_EXT_ID_CPU_TIMESTAMP_QUERY: u32 = 0x03;
pub const DRM_V3D_EXT_ID_CPU_RESET_TIMESTAMP_QUERY: u32 = 0x04;
pub const DRM_V3D_EXT_ID_CPU_COPY_TIMESTAMP_QUERY: u32 = 0x05;
pub const DRM_V3D_EXT_ID_CPU_RESET_PERFORMANCE_QUERY: u32 = 0x06;
pub const DRM_V3D_EXT_ID_CPU_COPY_PERFORMANCE_QUERY: u32 = 0x07;
#[repr(C)] pub enum v3d_queue { V3D_BIN, V3D_RENDER, V3D_TFU, V3D_CSD, V3D_CACHE_CLEAN, V3D_CPU }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_sem { pub handle: __u32, pub flags: __u32, pub point: __u64, pub mbz: [__u64; 2] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_multi_sync {
    pub base: drm_v3d_extension, pub in_syncs: __u64, pub out_syncs: __u64,
    pub in_sync_count: __u32, pub out_sync_count: __u32, pub wait_stage: __u32, pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_submit_cl {
    pub bcl_start: __u32, pub bcl_end: __u32, pub rcl_start: __u32, pub rcl_end: __u32,
    pub in_sync_bcl: __u32, pub in_sync_rcl: __u32, pub out_sync: __u32, pub qma: __u32,
    pub qms: __u32, pub qts: __u32, pub bo_handles: __u64, pub bo_handle_count: __u32,
    pub flags: __u32, pub perfmon_id: __u32, pub pad: __u32, pub extensions: __u64,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_wait_bo { pub handle: __u32, pub pad: __u32, pub timeout_ns: __u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_create_bo { pub size: __u32, pub flags: __u32, pub handle: __u32, pub offset: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_mmap_bo { pub handle: __u32, pub flags: __u32, pub offset: __u64 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_get_param { pub param: __u32, pub pad: __u32, pub value: __u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_get_bo_offset { pub handle: __u32, pub offset: __u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_submit_tfu {
    pub icfg: __u32, pub iia: __u32, pub iis: __u32, pub ica: __u32, pub iua: __u32,
    pub ioa: __u32, pub ios: __u32, pub coef: [__u32; 4], pub bo_handles: [__u32; 4],
    pub in_sync: __u32, pub out_sync: __u32, pub flags: __u32, pub extensions: __u64,
    pub v71: drm_v3d_submit_tfu_v71,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_submit_tfu_v71 { pub ioc: __u32, pub pad: __u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_v3d_submit_csd {
    pub cfg: [__u32; 7], pub coef: [__u32; 4], pub bo_handles: __u64, pub bo_handle_count: __u32,
    pub in_sync: __u32, pub out_sync: __u32, pub perfmon_id: __u32, pub extensions: __u64,
    pub flags: __u32, pub pad: __u32,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_indirect_csd {
    pub base: drm_v3d_extension, pub submit: drm_v3d_submit_csd, pub indirect: __u32,
    pub offset: __u32, pub wg_size: __u32, pub wg_uniform_offsets: [__u32; 3],
}
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_timestamp_query {
    pub base: drm_v3d_extension, pub offsets: __u64, pub syncs: __u64, pub count: __u32, pub pad: __u32,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_reset_timestamp_query {
    pub base: drm_v3d_extension, pub syncs: __u64, pub offset: __u32, pub count: __u32,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_copy_timestamp_query {
    pub base: drm_v3d_extension, pub do_64bit: __u8, pub do_partial: __u8, pub availability_bit: __u8,
    pub pad: __u8, pub offset: __u32, pub stride: __u32, pub count: __u32, pub offsets: __u64, pub syncs: __u64,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_reset_performance_query {
    pub base: drm_v3d_extension, pub syncs: __u64, pub count: __u32, pub nperfmons: __u32, pub kperfmon_ids: __u64,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_copy_performance_query {
    pub base: drm_v3d_extension, pub do_64bit: __u8, pub do_partial: __u8, pub availability_bit: __u8,
    pub pad: __u8, pub offset: __u32, pub stride: __u32, pub nperfmons: __u32, pub ncounters: __u32,
    pub count: __u32, pub syncs: __u64, pub kperfmon_ids: __u64,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_submit_cpu {
    pub bo_handles: __u64, pub bo_handle_count: __u32, pub flags: __u32, pub extensions: __u64,
}

// Deprecated V3D 4.2 performance counter identifiers.
pub const DRM_V3D_MAX_PERF_COUNTERS: usize = 32;
pub const DRM_V3D_PERFCNT_MAX_NAME: usize = 64;
pub const DRM_V3D_PERFCNT_MAX_CATEGORY: usize = 32;
pub const DRM_V3D_PERFCNT_MAX_DESCRIPTION: usize = 256;
#[repr(C)] pub enum v3d_perfcnt {
    V3D_PERFCNT_FEP_VALID_PRIMTS_NO_PIXELS, V3D_PERFCNT_FEP_VALID_PRIMS, V3D_PERFCNT_FEP_EZ_NFCLIP_QUADS,
    V3D_PERFCNT_FEP_VALID_QUADS, V3D_PERFCNT_TLB_QUADS_STENCIL_FAIL, V3D_PERFCNT_TLB_QUADS_STENCILZ_FAIL,
    V3D_PERFCNT_TLB_QUADS_STENCILZ_PASS, V3D_PERFCNT_TLB_QUADS_ZERO_COV, V3D_PERFCNT_TLB_QUADS_NONZERO_COV,
    V3D_PERFCNT_TLB_QUADS_WRITTEN, V3D_PERFCNT_PTB_PRIM_VIEWPOINT_DISCARD, V3D_PERFCNT_PTB_PRIM_CLIP,
    V3D_PERFCNT_PTB_PRIM_REV, V3D_PERFCNT_QPU_IDLE_CYCLES, V3D_PERFCNT_QPU_ACTIVE_CYCLES_VERTEX_COORD_USER,
    V3D_PERFCNT_QPU_ACTIVE_CYCLES_FRAG, V3D_PERFCNT_QPU_CYCLES_VALID_INSTR, V3D_PERFCNT_QPU_CYCLES_TMU_STALL,
    V3D_PERFCNT_QPU_CYCLES_SCOREBOARD_STALL, V3D_PERFCNT_QPU_CYCLES_VARYINGS_STALL, V3D_PERFCNT_QPU_IC_HIT,
    V3D_PERFCNT_QPU_IC_MISS, V3D_PERFCNT_QPU_UC_HIT, V3D_PERFCNT_QPU_UC_MISS, V3D_PERFCNT_TMU_TCACHE_ACCESS,
    V3D_PERFCNT_TMU_TCACHE_MISS, V3D_PERFCNT_VPM_VDW_STALL, V3D_PERFCNT_VPM_VCD_STALL, V3D_PERFCNT_BIN_ACTIVE,
    V3D_PERFCNT_RDR_ACTIVE, V3D_PERFCNT_L2T_HITS, V3D_PERFCNT_L2T_MISSES, V3D_PERFCNT_CYCLE_COUNT,
    V3D_PERFCNT_QPU_CYCLES_STALLED_VERTEX_COORD_USER, V3D_PERFCNT_QPU_CYCLES_STALLED_FRAGMENT,
    V3D_PERFCNT_PTB_PRIMS_BINNED, V3D_PERFCNT_AXI_WRITES_WATCH_0, V3D_PERFCNT_AXI_READS_WATCH_0,
    V3D_PERFCNT_AXI_WRITE_STALLS_WATCH_0, V3D_PERFCNT_AXI_READ_STALLS_WATCH_0, V3D_PERFCNT_AXI_WRITE_BYTES_WATCH_0,
    V3D_PERFCNT_AXI_READ_BYTES_WATCH_0, V3D_PERFCNT_AXI_WRITES_WATCH_1, V3D_PERFCNT_AXI_READS_WATCH_1,
    V3D_PERFCNT_AXI_WRITE_STALLS_WATCH_1, V3D_PERFCNT_AXI_READ_STALLS_WATCH_1, V3D_PERFCNT_AXI_WRITE_BYTES_WATCH_1,
    V3D_PERFCNT_AXI_READ_BYTES_WATCH_1, V3D_PERFCNT_TLB_PARTIAL_QUADS, V3D_PERFCNT_TMU_CONFIG_ACCESSES,
    V3D_PERFCNT_L2T_NO_ID_STALL, V3D_PERFCNT_L2T_COM_QUE_STALL, V3D_PERFCNT_L2T_TMU_WRITES,
    V3D_PERFCNT_TMU_ACTIVE_CYCLES, V3D_PERFCNT_TMU_STALLED_CYCLES, V3D_PERFCNT_CLE_ACTIVE,
    V3D_PERFCNT_L2T_TMU_READS, V3D_PERFCNT_L2T_CLE_READS, V3D_PERFCNT_L2T_VCD_READS, V3D_PERFCNT_L2T_TMUCFG_READS,
    V3D_PERFCNT_L2T_SLC0_READS, V3D_PERFCNT_L2T_SLC1_READS, V3D_PERFCNT_L2T_SLC2_READS, V3D_PERFCNT_L2T_TMU_W_MISSES,
    V3D_PERFCNT_L2T_TMU_R_MISSES, V3D_PERFCNT_L2T_CLE_MISSES, V3D_PERFCNT_L2T_VCD_MISSES,
    V3D_PERFCNT_L2T_TMUCFG_MISSES, V3D_PERFCNT_L2T_SLC0_MISSES, V3D_PERFCNT_L2T_SLC1_MISSES,
    V3D_PERFCNT_L2T_SLC2_MISSES, V3D_PERFCNT_CORE_MEM_WRITES, V3D_PERFCNT_L2T_MEM_WRITES,
    V3D_PERFCNT_PTB_MEM_WRITES, V3D_PERFCNT_TLB_MEM_WRITES, V3D_PERFCNT_CORE_MEM_READS,
    V3D_PERFCNT_L2T_MEM_READS, V3D_PERFCNT_PTB_MEM_READS, V3D_PERFCNT_PSE_MEM_READS, V3D_PERFCNT_TLB_MEM_READS,
    V3D_PERFCNT_GMP_MEM_READS, V3D_PERFCNT_PTB_W_MEM_WORDS, V3D_PERFCNT_TLB_W_MEM_WORDS,
    V3D_PERFCNT_PSE_R_MEM_WORDS, V3D_PERFCNT_TLB_R_MEM_WORDS, V3D_PERFCNT_TMU_MRU_HITS,
    V3D_PERFCNT_COMPUTE_ACTIVE, V3D_PERFCNT_NUM,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_perfmon_create { pub id: __u32, pub ncounters: __u32, pub counters: [__u8; DRM_V3D_MAX_PERF_COUNTERS] }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_perfmon_destroy { pub id: __u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_perfmon_get_values { pub id: __u32, pub pad: __u32, pub values_ptr: __u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_perfmon_get_counter {
    pub counter: __u8, pub name: [__u8; DRM_V3D_PERFCNT_MAX_NAME], pub category: [__u8; DRM_V3D_PERFCNT_MAX_CATEGORY],
    pub description: [__u8; DRM_V3D_PERFCNT_MAX_DESCRIPTION], pub reserved: [__u8; 7],
}
pub const DRM_V3D_PERFMON_CLEAR_GLOBAL: u32 = 0x0001;
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_v3d_perfmon_set_global { pub flags: __u32, pub id: __u32 }

#[repr(C)] pub enum drm_v3d_param {
    DRM_V3D_PARAM_V3D_UIFCFG, DRM_V3D_PARAM_V3D_HUB_IDENT1, DRM_V3D_PARAM_V3D_HUB_IDENT2,
    DRM_V3D_PARAM_V3D_HUB_IDENT3, DRM_V3D_PARAM_V3D_CORE0_IDENT0, DRM_V3D_PARAM_V3D_CORE0_IDENT1,
    DRM_V3D_PARAM_V3D_CORE0_IDENT2, DRM_V3D_PARAM_SUPPORTS_TFU, DRM_V3D_PARAM_SUPPORTS_CSD,
    DRM_V3D_PARAM_SUPPORTS_CACHE_FLUSH, DRM_V3D_PARAM_SUPPORTS_PERFMON, DRM_V3D_PARAM_SUPPORTS_MULTISYNC_EXT,
    DRM_V3D_PARAM_SUPPORTS_CPU_QUEUE, DRM_V3D_PARAM_MAX_PERF_COUNTERS, DRM_V3D_PARAM_SUPPORTS_SUPER_PAGES,
    DRM_V3D_PARAM_GLOBAL_RESET_COUNTER, DRM_V3D_PARAM_CONTEXT_RESET_COUNTER,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
