/*
 * Rust translation of i915_drm.h. C header guards/includes and C++ extern
 * wrappers were intentionally omitted; declarations from drm.h are referenced
 * as external Rust names where this header used them.
 */

pub type drm_handle_t = u64;
pub type drm_drawable_t = u32;

#[repr(C)]
pub struct drm_tex_region {
    pub next: u8,
    pub prev: u8,
    pub in_use: u8,
    pub padding: u8,
    pub age: u32,
}

#[repr(C)]
pub struct drm_clip_rect {
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
}

pub type drm_vblank_seq_type = u32;

pub const I915_L3_PARITY_UEVENT: &str = "L3_PARITY_ERROR";
pub const I915_ERROR_UEVENT: &str = "ERROR";
pub const I915_RESET_UEVENT: &str = "RESET";

#[repr(C)]
pub struct i915_user_extension { pub next_extension: u64, pub name: u32, pub flags: u32, pub rsvd: [u32; 4] }

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum i915_mocs_table_index { I915_MOCS_UNCACHED = 0, I915_MOCS_PTE = 1, I915_MOCS_CACHED = 2 }

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum drm_i915_gem_engine_class { I915_ENGINE_CLASS_RENDER = 0, I915_ENGINE_CLASS_COPY = 1, I915_ENGINE_CLASS_VIDEO = 2, I915_ENGINE_CLASS_VIDEO_ENHANCE = 3, I915_ENGINE_CLASS_COMPUTE = 4, I915_ENGINE_CLASS_INVALID = -1 }

#[repr(C)]
pub struct i915_engine_class_instance { pub engine_class: u16, pub engine_instance: u16 }
pub const I915_ENGINE_CLASS_INVALID_NONE: i32 = -1;
pub const I915_ENGINE_CLASS_INVALID_VIRTUAL: i32 = -2;

#[repr(i32)]
pub enum drm_i915_pmu_engine_sample { I915_SAMPLE_BUSY = 0, I915_SAMPLE_WAIT = 1, I915_SAMPLE_SEMA = 2 }
pub const I915_PMU_SAMPLE_BITS: u64 = 4;
pub const I915_PMU_SAMPLE_MASK: u64 = 0xf;
pub const I915_PMU_SAMPLE_INSTANCE_BITS: u64 = 8;
pub const I915_PMU_CLASS_SHIFT: u64 = I915_PMU_SAMPLE_BITS + I915_PMU_SAMPLE_INSTANCE_BITS;
pub const fn __I915_PMU_ENGINE(class: u64, instance: u64, sample: u64) -> u64 { (class << I915_PMU_CLASS_SHIFT) | (instance << I915_PMU_SAMPLE_BITS) | sample }
pub const fn I915_PMU_ENGINE_BUSY(class: u64, instance: u64) -> u64 { __I915_PMU_ENGINE(class, instance, drm_i915_pmu_engine_sample::I915_SAMPLE_BUSY as u64) }
pub const fn I915_PMU_ENGINE_WAIT(class: u64, instance: u64) -> u64 { __I915_PMU_ENGINE(class, instance, drm_i915_pmu_engine_sample::I915_SAMPLE_WAIT as u64) }
pub const fn I915_PMU_ENGINE_SEMA(class: u64, instance: u64) -> u64 { __I915_PMU_ENGINE(class, instance, drm_i915_pmu_engine_sample::I915_SAMPLE_SEMA as u64) }
pub const __I915_PMU_GT_SHIFT: u64 = 60;
pub const fn ___I915_PMU_OTHER(gt: u64, x: u64) -> u64 { (__I915_PMU_ENGINE(0xff, 0xff, 0xf) + 1 + x) | (gt << __I915_PMU_GT_SHIFT) }
pub const fn __I915_PMU_OTHER(x: u64) -> u64 { ___I915_PMU_OTHER(0, x) }
pub const I915_PMU_ACTUAL_FREQUENCY: u64 = __I915_PMU_OTHER(0);
pub const I915_PMU_REQUESTED_FREQUENCY: u64 = __I915_PMU_OTHER(1);
pub const I915_PMU_INTERRUPTS: u64 = __I915_PMU_OTHER(2);
pub const I915_PMU_RC6_RESIDENCY: u64 = __I915_PMU_OTHER(3);
pub const I915_PMU_SOFTWARE_GT_AWAKE_TIME: u64 = __I915_PMU_OTHER(4);
pub const I915_PMU_LAST: u64 = I915_PMU_RC6_RESIDENCY;
pub const fn __I915_PMU_ACTUAL_FREQUENCY(gt: u64) -> u64 { ___I915_PMU_OTHER(gt, 0) }
pub const fn __I915_PMU_REQUESTED_FREQUENCY(gt: u64) -> u64 { ___I915_PMU_OTHER(gt, 1) }
pub const fn __I915_PMU_INTERRUPTS(gt: u64) -> u64 { ___I915_PMU_OTHER(gt, 2) }
pub const fn __I915_PMU_RC6_RESIDENCY(gt: u64) -> u64 { ___I915_PMU_OTHER(gt, 3) }
pub const fn __I915_PMU_SOFTWARE_GT_AWAKE_TIME(gt: u64) -> u64 { ___I915_PMU_OTHER(gt, 4) }

pub const I915_NR_TEX_REGIONS: usize = 255;
pub const I915_LOG_MIN_TEX_REGION_SIZE: u32 = 14;

#[repr(i32)]
pub enum drm_i915_init_func { I915_INIT_DMA = 0x01, I915_CLEANUP_DMA = 0x02, I915_RESUME_DMA = 0x03 }
#[repr(C)]
pub struct _drm_i915_init { pub func: drm_i915_init_func, pub mmio_offset: u32, pub sarea_priv_offset: i32, pub ring_start: u32, pub ring_end: u32, pub ring_size: u32, pub front_offset: u32, pub back_offset: u32, pub depth_offset: u32, pub w: u32, pub h: u32, pub pitch: u32, pub pitch_bits: u32, pub back_pitch: u32, pub depth_pitch: u32, pub cpp: u32, pub chipset: u32 }
pub type drm_i915_init_t = _drm_i915_init;

#[repr(C)]
pub struct _drm_i915_sarea { pub texList: [drm_tex_region; I915_NR_TEX_REGIONS + 1], pub last_upload: i32, pub last_enqueue: i32, pub last_dispatch: i32, pub ctxOwner: i32, pub texAge: i32, pub pf_enabled: i32, pub pf_active: i32, pub pf_current_page: i32, pub perf_boxes: i32, pub width: i32, pub height: i32, pub front_handle: drm_handle_t, pub front_offset: i32, pub front_size: i32, pub back_handle: drm_handle_t, pub back_offset: i32, pub back_size: i32, pub depth_handle: drm_handle_t, pub depth_offset: i32, pub depth_size: i32, pub tex_handle: drm_handle_t, pub tex_offset: i32, pub tex_size: i32, pub log_tex_granularity: i32, pub pitch: i32, pub rotation: i32, pub rotated_offset: i32, pub rotated_size: i32, pub rotated_pitch: i32, pub virtualX: i32, pub virtualY: i32, pub front_tiled: u32, pub back_tiled: u32, pub depth_tiled: u32, pub rotated_tiled: u32, pub rotated2_tiled: u32, pub pipeA_x: i32, pub pipeA_y: i32, pub pipeA_w: i32, pub pipeA_h: i32, pub pipeB_x: i32, pub pipeB_y: i32, pub pipeB_w: i32, pub pipeB_h: i32, pub unused_handle: drm_handle_t, pub unused1: u32, pub unused2: u32, pub unused3: u32, pub front_bo_handle: u32, pub back_bo_handle: u32, pub unused_bo_handle: u32, pub depth_bo_handle: u32 }
pub type drm_i915_sarea_t = _drm_i915_sarea;

pub const I915_BOX_RING_EMPTY: u32 = 0x1; pub const I915_BOX_FLIP: u32 = 0x2; pub const I915_BOX_WAIT: u32 = 0x4; pub const I915_BOX_TEXTURE_LOAD: u32 = 0x8; pub const I915_BOX_LOST_CONTEXT: u32 = 0x10;

pub const DRM_I915_INIT: u32 = 0x00; pub const DRM_I915_FLUSH: u32 = 0x01; pub const DRM_I915_FLIP: u32 = 0x02; pub const DRM_I915_BATCHBUFFER: u32 = 0x03; pub const DRM_I915_IRQ_EMIT: u32 = 0x04; pub const DRM_I915_IRQ_WAIT: u32 = 0x05; pub const DRM_I915_GETPARAM: u32 = 0x06; pub const DRM_I915_SETPARAM: u32 = 0x07; pub const DRM_I915_ALLOC: u32 = 0x08; pub const DRM_I915_FREE: u32 = 0x09; pub const DRM_I915_INIT_HEAP: u32 = 0x0a; pub const DRM_I915_CMDBUFFER: u32 = 0x0b; pub const DRM_I915_DESTROY_HEAP: u32 = 0x0c; pub const DRM_I915_SET_VBLANK_PIPE: u32 = 0x0d; pub const DRM_I915_GET_VBLANK_PIPE: u32 = 0x0e; pub const DRM_I915_VBLANK_SWAP: u32 = 0x0f; pub const DRM_I915_HWS_ADDR: u32 = 0x11; pub const DRM_I915_GEM_INIT: u32 = 0x13; pub const DRM_I915_GEM_EXECBUFFER: u32 = 0x14; pub const DRM_I915_GEM_PIN: u32 = 0x15; pub const DRM_I915_GEM_UNPIN: u32 = 0x16; pub const DRM_I915_GEM_BUSY: u32 = 0x17; pub const DRM_I915_GEM_THROTTLE: u32 = 0x18; pub const DRM_I915_GEM_ENTERVT: u32 = 0x19; pub const DRM_I915_GEM_LEAVEVT: u32 = 0x1a; pub const DRM_I915_GEM_CREATE: u32 = 0x1b; pub const DRM_I915_GEM_PREAD: u32 = 0x1c; pub const DRM_I915_GEM_PWRITE: u32 = 0x1d; pub const DRM_I915_GEM_MMAP: u32 = 0x1e; pub const DRM_I915_GEM_SET_DOMAIN: u32 = 0x1f; pub const DRM_I915_GEM_SW_FINISH: u32 = 0x20; pub const DRM_I915_GEM_SET_TILING: u32 = 0x21; pub const DRM_I915_GEM_GET_TILING: u32 = 0x22; pub const DRM_I915_GEM_GET_APERTURE: u32 = 0x23; pub const DRM_I915_GEM_MMAP_GTT: u32 = 0x24; pub const DRM_I915_GET_PIPE_FROM_CRTC_ID: u32 = 0x25; pub const DRM_I915_GEM_MADVISE: u32 = 0x26; pub const DRM_I915_OVERLAY_PUT_IMAGE: u32 = 0x27; pub const DRM_I915_OVERLAY_ATTRS: u32 = 0x28; pub const DRM_I915_GEM_EXECBUFFER2: u32 = 0x29; pub const DRM_I915_GEM_EXECBUFFER2_WR: u32 = DRM_I915_GEM_EXECBUFFER2; pub const DRM_I915_GET_SPRITE_COLORKEY: u32 = 0x2a; pub const DRM_I915_SET_SPRITE_COLORKEY: u32 = 0x2b; pub const DRM_I915_GEM_WAIT: u32 = 0x2c; pub const DRM_I915_GEM_CONTEXT_CREATE: u32 = 0x2d; pub const DRM_I915_GEM_CONTEXT_DESTROY: u32 = 0x2e; pub const DRM_I915_GEM_SET_CACHING: u32 = 0x2f; pub const DRM_I915_GEM_GET_CACHING: u32 = 0x30; pub const DRM_I915_REG_READ: u32 = 0x31; pub const DRM_I915_GET_RESET_STATS: u32 = 0x32; pub const DRM_I915_GEM_USERPTR: u32 = 0x33; pub const DRM_I915_GEM_CONTEXT_GETPARAM: u32 = 0x34; pub const DRM_I915_GEM_CONTEXT_SETPARAM: u32 = 0x35; pub const DRM_I915_PERF_OPEN: u32 = 0x36; pub const DRM_I915_PERF_ADD_CONFIG: u32 = 0x37; pub const DRM_I915_PERF_REMOVE_CONFIG: u32 = 0x38; pub const DRM_I915_QUERY: u32 = 0x39; pub const DRM_I915_GEM_VM_CREATE: u32 = 0x3a; pub const DRM_I915_GEM_VM_DESTROY: u32 = 0x3b; pub const DRM_I915_GEM_CREATE_EXT: u32 = 0x3c;

/* DRM_IOCTL_I915_* macros are C ioctl-number macros using DRM_IO/DRM_IOR/DRM_IOW/DRM_IOWR from drm.h; keep those as external dependency expressions in the final integration. */

#[repr(C)] pub struct drm_i915_batchbuffer { pub start: i32, pub used: i32, pub DR1: i32, pub DR4: i32, pub num_cliprects: i32, pub cliprects: *mut drm_clip_rect } pub type drm_i915_batchbuffer_t = drm_i915_batchbuffer;
#[repr(C)] pub struct _drm_i915_cmdbuffer { pub buf: *mut i8, pub sz: i32, pub DR1: i32, pub DR4: i32, pub num_cliprects: i32, pub cliprects: *mut drm_clip_rect } pub type drm_i915_cmdbuffer_t = _drm_i915_cmdbuffer;
#[repr(C)] pub struct drm_i915_irq_emit { pub irq_seq: *mut i32 } pub type drm_i915_irq_emit_t = drm_i915_irq_emit;
#[repr(C)] pub struct drm_i915_irq_wait { pub irq_seq: i32 } pub type drm_i915_irq_wait_t = drm_i915_irq_wait;

pub const I915_GEM_PPGTT_NONE: u32 = 0; pub const I915_GEM_PPGTT_ALIASING: u32 = 1; pub const I915_GEM_PPGTT_FULL: u32 = 2;

pub const I915_PARAM_IRQ_ACTIVE: i32 = 1; pub const I915_PARAM_ALLOW_BATCHBUFFER: i32 = 2; pub const I915_PARAM_LAST_DISPATCH: i32 = 3; pub const I915_PARAM_CHIPSET_ID: i32 = 4; pub const I915_PARAM_HAS_GEM: i32 = 5; pub const I915_PARAM_NUM_FENCES_AVAIL: i32 = 6; pub const I915_PARAM_HAS_OVERLAY: i32 = 7; pub const I915_PARAM_HAS_PAGEFLIPPING: i32 = 8; pub const I915_PARAM_HAS_EXECBUF2: i32 = 9; pub const I915_PARAM_HAS_BSD: i32 = 10; pub const I915_PARAM_HAS_BLT: i32 = 11; pub const I915_PARAM_HAS_RELAXED_FENCING: i32 = 12; pub const I915_PARAM_HAS_COHERENT_RINGS: i32 = 13; pub const I915_PARAM_HAS_EXEC_CONSTANTS: i32 = 14; pub const I915_PARAM_HAS_RELAXED_DELTA: i32 = 15; pub const I915_PARAM_HAS_GEN7_SOL_RESET: i32 = 16; pub const I915_PARAM_HAS_LLC: i32 = 17; pub const I915_PARAM_HAS_ALIASING_PPGTT: i32 = 18; pub const I915_PARAM_HAS_WAIT_TIMEOUT: i32 = 19; pub const I915_PARAM_HAS_SEMAPHORES: i32 = 20; pub const I915_PARAM_HAS_PRIME_VMAP_FLUSH: i32 = 21; pub const I915_PARAM_HAS_VEBOX: i32 = 22; pub const I915_PARAM_HAS_SECURE_BATCHES: i32 = 23; pub const I915_PARAM_HAS_PINNED_BATCHES: i32 = 24; pub const I915_PARAM_HAS_EXEC_NO_RELOC: i32 = 25; pub const I915_PARAM_HAS_EXEC_HANDLE_LUT: i32 = 26; pub const I915_PARAM_HAS_WT: i32 = 27; pub const I915_PARAM_CMD_PARSER_VERSION: i32 = 28; pub const I915_PARAM_HAS_COHERENT_PHYS_GTT: i32 = 29; pub const I915_PARAM_MMAP_VERSION: i32 = 30; pub const I915_PARAM_HAS_BSD2: i32 = 31; pub const I915_PARAM_REVISION: i32 = 32; pub const I915_PARAM_SUBSLICE_TOTAL: i32 = 33; pub const I915_PARAM_EU_TOTAL: i32 = 34; pub const I915_PARAM_HAS_GPU_RESET: i32 = 35; pub const I915_PARAM_HAS_RESOURCE_STREAMER: i32 = 36; pub const I915_PARAM_HAS_EXEC_SOFTPIN: i32 = 37; pub const I915_PARAM_HAS_POOLED_EU: i32 = 38; pub const I915_PARAM_MIN_EU_IN_POOL: i32 = 39; pub const I915_PARAM_MMAP_GTT_VERSION: i32 = 40; pub const I915_PARAM_HAS_SCHEDULER: i32 = 41; pub const I915_PARAM_HUC_STATUS: i32 = 42; pub const I915_PARAM_HAS_EXEC_ASYNC: i32 = 43; pub const I915_PARAM_HAS_EXEC_FENCE: i32 = 44; pub const I915_PARAM_HAS_EXEC_CAPTURE: i32 = 45; pub const I915_PARAM_SLICE_MASK: i32 = 46; pub const I915_PARAM_SUBSLICE_MASK: i32 = 47; pub const I915_PARAM_HAS_EXEC_BATCH_FIRST: i32 = 48; pub const I915_PARAM_HAS_EXEC_FENCE_ARRAY: i32 = 49; pub const I915_PARAM_HAS_CONTEXT_ISOLATION: i32 = 50; pub const I915_PARAM_CS_TIMESTAMP_FREQUENCY: i32 = 51; pub const I915_PARAM_MMAP_GTT_COHERENT: i32 = 52; pub const I915_PARAM_HAS_EXEC_SUBMIT_FENCE: i32 = 53; pub const I915_PARAM_PERF_REVISION: i32 = 54; pub const I915_PARAM_HAS_EXEC_TIMELINE_FENCES: i32 = 55; pub const I915_PARAM_HAS_USERPTR_PROBE: i32 = 56; pub const I915_PARAM_OA_TIMESTAMP_FREQUENCY: i32 = 57; pub const I915_PARAM_PXP_STATUS: i32 = 58; pub const I915_PARAM_HAS_CONTEXT_FREQ_HINT: i32 = 59;
pub const I915_SCHEDULER_CAP_ENABLED: u64 = 1 << 0; pub const I915_SCHEDULER_CAP_PRIORITY: u64 = 1 << 1; pub const I915_SCHEDULER_CAP_PREEMPTION: u64 = 1 << 2; pub const I915_SCHEDULER_CAP_SEMAPHORES: u64 = 1 << 3; pub const I915_SCHEDULER_CAP_ENGINE_BUSY_STATS: u64 = 1 << 4; pub const I915_SCHEDULER_CAP_STATIC_PRIORITY_MAP: u64 = 1 << 5;

#[repr(C)] pub struct drm_i915_getparam { pub param: i32, pub value: *mut i32 } pub type drm_i915_getparam_t = drm_i915_getparam;
pub const I915_SETPARAM_USE_MI_BATCHBUFFER_START: i32 = 1; pub const I915_SETPARAM_TEX_LRU_LOG_GRANULARITY: i32 = 2; pub const I915_SETPARAM_ALLOW_BATCHBUFFER: i32 = 3; pub const I915_SETPARAM_NUM_USED_FENCES: i32 = 4;
#[repr(C)] pub struct drm_i915_setparam { pub param: i32, pub value: i32 } pub type drm_i915_setparam_t = drm_i915_setparam;
pub const I915_MEM_REGION_AGP: i32 = 1;
#[repr(C)] pub struct drm_i915_mem_alloc { pub region: i32, pub alignment: i32, pub size: i32, pub region_offset: *mut i32 } pub type drm_i915_mem_alloc_t = drm_i915_mem_alloc;
#[repr(C)] pub struct drm_i915_mem_free { pub region: i32, pub region_offset: i32 } pub type drm_i915_mem_free_t = drm_i915_mem_free;
#[repr(C)] pub struct drm_i915_mem_init_heap { pub region: i32, pub size: i32, pub start: i32 } pub type drm_i915_mem_init_heap_t = drm_i915_mem_init_heap;
#[repr(C)] pub struct drm_i915_mem_destroy_heap { pub region: i32 } pub type drm_i915_mem_destroy_heap_t = drm_i915_mem_destroy_heap;
pub const DRM_I915_VBLANK_PIPE_A: i32 = 1; pub const DRM_I915_VBLANK_PIPE_B: i32 = 2;
#[repr(C)] pub struct drm_i915_vblank_pipe { pub pipe: i32 } pub type drm_i915_vblank_pipe_t = drm_i915_vblank_pipe;
#[repr(C)] pub struct drm_i915_vblank_swap { pub drawable: drm_drawable_t, pub seqtype: drm_vblank_seq_type, pub sequence: u32 } pub type drm_i915_vblank_swap_t = drm_i915_vblank_swap;
#[repr(C)] pub struct drm_i915_hws_addr { pub addr: u64 } pub type drm_i915_hws_addr_t = drm_i915_hws_addr;

#[repr(C)] pub struct drm_i915_gem_init { pub gtt_start: u64, pub gtt_end: u64 }
#[repr(C)] pub struct drm_i915_gem_create { pub size: u64, pub handle: u32, pub pad: u32 }
#[repr(C)] pub struct drm_i915_gem_pread { pub handle: u32, pub pad: u32, pub offset: u64, pub size: u64, pub data_ptr: u64 }
#[repr(C)] pub struct drm_i915_gem_pwrite { pub handle: u32, pub pad: u32, pub offset: u64, pub size: u64, pub data_ptr: u64 }
#[repr(C)] pub struct drm_i915_gem_mmap { pub handle: u32, pub pad: u32, pub offset: u64, pub size: u64, pub addr_ptr: u64, pub flags: u64 }
pub const I915_MMAP_WC: u64 = 0x1;
#[repr(C)] pub struct drm_i915_gem_mmap_gtt { pub handle: u32, pub pad: u32, pub offset: u64 }
#[repr(C)] pub struct drm_i915_gem_mmap_offset { pub handle: u32, pub pad: u32, pub offset: u64, pub flags: u64, pub extensions: u64 }
pub const I915_MMAP_OFFSET_GTT: u64 = 0; pub const I915_MMAP_OFFSET_WC: u64 = 1; pub const I915_MMAP_OFFSET_WB: u64 = 2; pub const I915_MMAP_OFFSET_UC: u64 = 3; pub const I915_MMAP_OFFSET_FIXED: u64 = 4;
#[repr(C)] pub struct drm_i915_gem_set_domain { pub handle: u32, pub read_domains: u32, pub write_domain: u32 }
#[repr(C)] pub struct drm_i915_gem_sw_finish { pub handle: u32 }
#[repr(C)] pub struct drm_i915_gem_relocation_entry { pub target_handle: u32, pub delta: u32, pub offset: u64, pub presumed_offset: u64, pub read_domains: u32, pub write_domain: u32 }
pub const I915_GEM_DOMAIN_CPU: u32 = 0x00000001; pub const I915_GEM_DOMAIN_RENDER: u32 = 0x00000002; pub const I915_GEM_DOMAIN_SAMPLER: u32 = 0x00000004; pub const I915_GEM_DOMAIN_COMMAND: u32 = 0x00000008; pub const I915_GEM_DOMAIN_INSTRUCTION: u32 = 0x00000010; pub const I915_GEM_DOMAIN_VERTEX: u32 = 0x00000020; pub const I915_GEM_DOMAIN_GTT: u32 = 0x00000040; pub const I915_GEM_DOMAIN_WC: u32 = 0x00000080;
#[repr(C)] pub struct drm_i915_gem_exec_object { pub handle: u32, pub relocation_count: u32, pub relocs_ptr: u64, pub alignment: u64, pub offset: u64 }
#[repr(C)] pub struct drm_i915_gem_execbuffer { pub buffers_ptr: u64, pub buffer_count: u32, pub batch_start_offset: u32, pub batch_len: u32, pub DR1: u32, pub DR4: u32, pub num_cliprects: u32, pub cliprects_ptr: u64 }
#[repr(C)] pub union drm_i915_gem_exec_object2_union { pub rsvd1: u64, pub pad_to_size: u64 }
#[repr(C)] pub struct drm_i915_gem_exec_object2 { pub handle: u32, pub relocation_count: u32, pub relocs_ptr: u64, pub alignment: u64, pub offset: u64, pub flags: u64, pub u: drm_i915_gem_exec_object2_union, pub rsvd2: u64 }
pub const EXEC_OBJECT_NEEDS_FENCE: u64 = 1 << 0; pub const EXEC_OBJECT_NEEDS_GTT: u64 = 1 << 1; pub const EXEC_OBJECT_WRITE: u64 = 1 << 2; pub const EXEC_OBJECT_SUPPORTS_48B_ADDRESS: u64 = 1 << 3; pub const EXEC_OBJECT_PINNED: u64 = 1 << 4; pub const EXEC_OBJECT_PAD_TO_SIZE: u64 = 1 << 5; pub const EXEC_OBJECT_ASYNC: u64 = 1 << 6; pub const EXEC_OBJECT_CAPTURE: u64 = 1 << 7; pub const __EXEC_OBJECT_UNKNOWN_FLAGS: i64 = -((EXEC_OBJECT_CAPTURE << 1) as i64);
#[repr(C)] pub struct drm_i915_gem_exec_fence { pub handle: u32, pub flags: u32 }
pub const I915_EXEC_FENCE_WAIT: u32 = 1 << 0; pub const I915_EXEC_FENCE_SIGNAL: u32 = 1 << 1; pub const __I915_EXEC_FENCE_UNKNOWN_FLAGS: i32 = -((I915_EXEC_FENCE_SIGNAL << 1) as i32);
#[repr(C)] pub struct drm_i915_gem_execbuffer_ext_timeline_fences { pub base: i915_user_extension, pub fence_count: u64, pub handles_ptr: u64, pub values_ptr: u64 }
pub const DRM_I915_GEM_EXECBUFFER_EXT_TIMELINE_FENCES: u32 = 0;
#[repr(C)] pub struct drm_i915_gem_execbuffer2 { pub buffers_ptr: u64, pub buffer_count: u32, pub batch_start_offset: u32, pub batch_len: u32, pub DR1: u32, pub DR4: u32, pub num_cliprects: u32, pub cliprects_ptr: u64, pub flags: u64, pub rsvd1: u64, pub rsvd2: u64 }
pub const I915_EXEC_RING_MASK: u64 = 0x3f; pub const I915_EXEC_DEFAULT: u64 = 0 << 0; pub const I915_EXEC_RENDER: u64 = 1 << 0; pub const I915_EXEC_BSD: u64 = 2 << 0; pub const I915_EXEC_BLT: u64 = 3 << 0; pub const I915_EXEC_VEBOX: u64 = 4 << 0; pub const I915_EXEC_CONSTANTS_MASK: u64 = 3 << 6; pub const I915_EXEC_CONSTANTS_REL_GENERAL: u64 = 0 << 6; pub const I915_EXEC_CONSTANTS_ABSOLUTE: u64 = 1 << 6; pub const I915_EXEC_CONSTANTS_REL_SURFACE: u64 = 2 << 6; pub const I915_EXEC_GEN7_SOL_RESET: u64 = 1 << 8; pub const I915_EXEC_SECURE: u64 = 1 << 9; pub const I915_EXEC_IS_PINNED: u64 = 1 << 10; pub const I915_EXEC_NO_RELOC: u64 = 1 << 11; pub const I915_EXEC_HANDLE_LUT: u64 = 1 << 12; pub const I915_EXEC_BSD_SHIFT: u64 = 13; pub const I915_EXEC_BSD_MASK: u64 = 3 << I915_EXEC_BSD_SHIFT; pub const I915_EXEC_BSD_DEFAULT: u64 = 0 << I915_EXEC_BSD_SHIFT; pub const I915_EXEC_BSD_RING1: u64 = 1 << I915_EXEC_BSD_SHIFT; pub const I915_EXEC_BSD_RING2: u64 = 2 << I915_EXEC_BSD_SHIFT; pub const I915_EXEC_RESOURCE_STREAMER: u64 = 1 << 15; pub const I915_EXEC_FENCE_IN: u64 = 1 << 16; pub const I915_EXEC_FENCE_OUT: u64 = 1 << 17; pub const I915_EXEC_BATCH_FIRST: u64 = 1 << 18; pub const I915_EXEC_FENCE_ARRAY: u64 = 1 << 19; pub const I915_EXEC_FENCE_SUBMIT: u64 = 1 << 20; pub const I915_EXEC_USE_EXTENSIONS: u64 = 1 << 21; pub const __I915_EXEC_UNKNOWN_FLAGS: i64 = -((I915_EXEC_USE_EXTENSIONS << 1) as i64);
pub const I915_EXEC_CONTEXT_ID_MASK: u64 = 0xffffffff;
pub fn i915_execbuffer2_set_context_id(eb2: &mut drm_i915_gem_execbuffer2, context: u64) { eb2.rsvd1 = context & I915_EXEC_CONTEXT_ID_MASK; }
pub fn i915_execbuffer2_get_context_id(eb2: &drm_i915_gem_execbuffer2) -> u64 { eb2.rsvd1 & I915_EXEC_CONTEXT_ID_MASK }

#[repr(C)] pub struct drm_i915_gem_pin { pub handle: u32, pub pad: u32, pub alignment: u64, pub offset: u64 }
#[repr(C)] pub struct drm_i915_gem_unpin { pub handle: u32, pub pad: u32 }
#[repr(C)] pub struct drm_i915_gem_busy { pub handle: u32, pub busy: u32 }
#[repr(C)] pub struct drm_i915_gem_caching { pub handle: u32, pub caching: u32 }
pub const I915_CACHING_NONE: u32 = 0; pub const I915_CACHING_CACHED: u32 = 1; pub const I915_CACHING_DISPLAY: u32 = 2;
pub const I915_TILING_NONE: u32 = 0; pub const I915_TILING_X: u32 = 1; pub const I915_TILING_Y: u32 = 2; pub const I915_TILING_LAST: u32 = I915_TILING_Y;
pub const I915_BIT_6_SWIZZLE_NONE: u32 = 0; pub const I915_BIT_6_SWIZZLE_9: u32 = 1; pub const I915_BIT_6_SWIZZLE_9_10: u32 = 2; pub const I915_BIT_6_SWIZZLE_9_11: u32 = 3; pub const I915_BIT_6_SWIZZLE_9_10_11: u32 = 4; pub const I915_BIT_6_SWIZZLE_UNKNOWN: u32 = 5; pub const I915_BIT_6_SWIZZLE_9_17: u32 = 6; pub const I915_BIT_6_SWIZZLE_9_10_17: u32 = 7;
#[repr(C)] pub struct drm_i915_gem_set_tiling { pub handle: u32, pub tiling_mode: u32, pub stride: u32, pub swizzle_mode: u32 }
#[repr(C)] pub struct drm_i915_gem_get_tiling { pub handle: u32, pub tiling_mode: u32, pub swizzle_mode: u32, pub phys_swizzle_mode: u32 }
#[repr(C)] pub struct drm_i915_gem_get_aperture { pub aper_size: u64, pub aper_available_size: u64 }
#[repr(C)] pub struct drm_i915_get_pipe_from_crtc_id { pub crtc_id: u32, pub pipe: u32 }
pub const I915_MADV_WILLNEED: u32 = 0; pub const I915_MADV_DONTNEED: u32 = 1; pub const __I915_MADV_PURGED: u32 = 2;
#[repr(C)] pub struct drm_i915_gem_madvise { pub handle: u32, pub madv: u32, pub retained: u32 }

pub const I915_OVERLAY_TYPE_MASK: u32 = 0xff; pub const I915_OVERLAY_YUV_PLANAR: u32 = 0x01; pub const I915_OVERLAY_YUV_PACKED: u32 = 0x02; pub const I915_OVERLAY_RGB: u32 = 0x03; pub const I915_OVERLAY_DEPTH_MASK: u32 = 0xff00; pub const I915_OVERLAY_RGB24: u32 = 0x1000; pub const I915_OVERLAY_RGB16: u32 = 0x2000; pub const I915_OVERLAY_RGB15: u32 = 0x3000; pub const I915_OVERLAY_YUV422: u32 = 0x0100; pub const I915_OVERLAY_YUV411: u32 = 0x0200; pub const I915_OVERLAY_YUV420: u32 = 0x0300; pub const I915_OVERLAY_YUV410: u32 = 0x0400; pub const I915_OVERLAY_SWAP_MASK: u32 = 0xff0000; pub const I915_OVERLAY_NO_SWAP: u32 = 0x000000; pub const I915_OVERLAY_UV_SWAP: u32 = 0x010000; pub const I915_OVERLAY_Y_SWAP: u32 = 0x020000; pub const I915_OVERLAY_Y_AND_UV_SWAP: u32 = 0x030000; pub const I915_OVERLAY_FLAGS_MASK: u32 = 0xff000000; pub const I915_OVERLAY_ENABLE: u32 = 0x01000000;
#[repr(C)] pub struct drm_intel_overlay_put_image { pub flags: u32, pub bo_handle: u32, pub stride_Y: u16, pub stride_UV: u16, pub offset_Y: u32, pub offset_U: u32, pub offset_V: u32, pub src_width: u16, pub src_height: u16, pub src_scan_width: u16, pub src_scan_height: u16, pub crtc_id: u32, pub dst_x: u16, pub dst_y: u16, pub dst_width: u16, pub dst_height: u16 }
pub const I915_OVERLAY_UPDATE_ATTRS: u32 = 1 << 0; pub const I915_OVERLAY_UPDATE_GAMMA: u32 = 1 << 1; pub const I915_OVERLAY_DISABLE_DEST_COLORKEY: u32 = 1 << 2;
#[repr(C)] pub struct drm_intel_overlay_attrs { pub flags: u32, pub color_key: u32, pub brightness: i32, pub contrast: u32, pub saturation: u32, pub gamma0: u32, pub gamma1: u32, pub gamma2: u32, pub gamma3: u32, pub gamma4: u32, pub gamma5: u32 }
pub const I915_SET_COLORKEY_NONE: u32 = 1 << 0; pub const I915_SET_COLORKEY_DESTINATION: u32 = 1 << 1; pub const I915_SET_COLORKEY_SOURCE: u32 = 1 << 2;
#[repr(C)] pub struct drm_intel_sprite_colorkey { pub plane_id: u32, pub min_value: u32, pub channel_mask: u32, pub max_value: u32, pub flags: u32 }
#[repr(C)] pub struct drm_i915_gem_wait { pub bo_handle: u32, pub flags: u32, pub timeout_ns: i64 }
#[repr(C)] pub struct drm_i915_gem_context_create { pub ctx_id: u32, pub pad: u32 }
#[repr(C)] pub struct drm_i915_gem_context_create_ext { pub ctx_id: u32, pub flags: u32, pub extensions: u64 }
pub const I915_CONTEXT_CREATE_FLAGS_USE_EXTENSIONS: u32 = 1 << 0; pub const I915_CONTEXT_CREATE_FLAGS_SINGLE_TIMELINE: u32 = 1 << 1; pub const I915_CONTEXT_CREATE_FLAGS_UNKNOWN: i32 = -((I915_CONTEXT_CREATE_FLAGS_SINGLE_TIMELINE << 1) as i32); pub const I915_CONTEXT_CREATE_EXT_SETPARAM: u64 = 0; pub const I915_CONTEXT_CREATE_EXT_CLONE: u64 = 1;
#[repr(C)] pub struct drm_i915_gem_context_param { pub ctx_id: u32, pub size: u32, pub param: u64, pub value: u64 }
pub const I915_CONTEXT_PARAM_BAN_PERIOD: u64 = 0x1; pub const I915_CONTEXT_PARAM_NO_ZEROMAP: u64 = 0x2; pub const I915_CONTEXT_PARAM_GTT_SIZE: u64 = 0x3; pub const I915_CONTEXT_PARAM_NO_ERROR_CAPTURE: u64 = 0x4; pub const I915_CONTEXT_PARAM_BANNABLE: u64 = 0x5; pub const I915_CONTEXT_PARAM_PRIORITY: u64 = 0x6; pub const I915_CONTEXT_MAX_USER_PRIORITY: i32 = 1023; pub const I915_CONTEXT_DEFAULT_PRIORITY: i32 = 0; pub const I915_CONTEXT_MIN_USER_PRIORITY: i32 = -1023; pub const I915_CONTEXT_PARAM_SSEU: u64 = 0x7; pub const I915_CONTEXT_PARAM_RECOVERABLE: u64 = 0x8; pub const I915_CONTEXT_PARAM_VM: u64 = 0x9; pub const I915_CONTEXT_PARAM_ENGINES: u64 = 0xa; pub const I915_CONTEXT_PARAM_PERSISTENCE: u64 = 0xb; pub const I915_CONTEXT_PARAM_RINGSIZE: u64 = 0xc; pub const I915_CONTEXT_PARAM_PROTECTED_CONTENT: u64 = 0xd; pub const I915_CONTEXT_PARAM_LOW_LATENCY: u64 = 0xe; pub const I915_CONTEXT_PARAM_CONTEXT_IMAGE: u64 = 0xf;
#[repr(C)] pub struct drm_i915_gem_context_param_sseu { pub engine: i915_engine_class_instance, pub flags: u32, pub slice_mask: u64, pub subslice_mask: u64, pub min_eus_per_subslice: u16, pub max_eus_per_subslice: u16, pub rsvd: u32 }
pub const I915_CONTEXT_SSEU_FLAG_ENGINE_INDEX: u32 = 1 << 0;

#[repr(C, packed)] pub struct i915_context_engines_load_balance { pub base: i915_user_extension, pub engine_index: u16, pub num_siblings: u16, pub flags: u32, pub mbz64: u64, pub engines: [i915_engine_class_instance; 0] }
/* I915_DEFINE_CONTEXT_ENGINES_LOAD_BALANCE(name__, N__) creates the same packed prefix with engines[N__]. */
#[repr(C, packed)] pub struct i915_context_engines_bond { pub base: i915_user_extension, pub master: i915_engine_class_instance, pub virtual_index: u16, pub num_bonds: u16, pub flags: u64, pub mbz64: [u64; 4], pub engines: [i915_engine_class_instance; 0] }
/* I915_DEFINE_CONTEXT_ENGINES_BOND(name__, N__) creates the same packed prefix with engines[N__]. */
#[repr(C, packed)] pub struct i915_context_engines_parallel_submit { pub base: i915_user_extension, pub engine_index: u16, pub width: u16, pub num_siblings: u16, pub mbz16: u16, pub flags: u64, pub mbz64: [u64; 3], pub engines: [i915_engine_class_instance; 0] }
/* I915_DEFINE_CONTEXT_ENGINES_PARALLEL_SUBMIT(name__, N__) creates the same packed prefix with engines[N__]. */
#[repr(C, packed)] pub struct i915_context_param_engines { pub extensions: u64, pub engines: [i915_engine_class_instance; 0] }
pub const I915_CONTEXT_ENGINES_EXT_LOAD_BALANCE: u64 = 0; pub const I915_CONTEXT_ENGINES_EXT_BOND: u64 = 1; pub const I915_CONTEXT_ENGINES_EXT_PARALLEL_SUBMIT: u64 = 2;
/* I915_DEFINE_CONTEXT_PARAM_ENGINES(name__, N__) creates the same packed prefix with engines[N__]. */
#[repr(C, packed)] pub struct i915_gem_context_param_context_image { pub engine: i915_engine_class_instance, pub flags: u32, pub size: u32, pub mbz: u32, pub image: u64 }
pub const I915_CONTEXT_IMAGE_FLAG_ENGINE_INDEX: u32 = 1 << 0;
#[repr(C)] pub struct drm_i915_gem_context_create_ext_setparam { pub base: i915_user_extension, pub param: drm_i915_gem_context_param }
#[repr(C)] pub struct drm_i915_gem_context_destroy { pub ctx_id: u32, pub pad: u32 }
#[repr(C)] pub struct drm_i915_gem_vm_control { pub extensions: u64, pub flags: u32, pub vm_id: u32 }
#[repr(C)] pub struct drm_i915_reg_read { pub offset: u64, pub val: u64 }
pub const I915_REG_READ_8B_WA: u64 = 1 << 0;
#[repr(C)] pub struct drm_i915_reset_stats { pub ctx_id: u32, pub flags: u32, pub reset_count: u32, pub batch_active: u32, pub batch_pending: u32, pub pad: u32 }
#[repr(C)] pub struct drm_i915_gem_userptr { pub user_ptr: u64, pub user_size: u64, pub flags: u32, pub handle: u32 }
pub const I915_USERPTR_READ_ONLY: u32 = 0x1; pub const I915_USERPTR_PROBE: u32 = 0x2; pub const I915_USERPTR_UNSYNCHRONIZED: u32 = 0x80000000;

#[repr(i32)] pub enum drm_i915_oa_format { I915_OA_FORMAT_A13 = 1, I915_OA_FORMAT_A29 = 2, I915_OA_FORMAT_A13_B8_C8 = 3, I915_OA_FORMAT_B4_C8 = 4, I915_OA_FORMAT_A45_B8_C8 = 5, I915_OA_FORMAT_B4_C8_A16 = 6, I915_OA_FORMAT_C4_B8 = 7, I915_OA_FORMAT_A12 = 8, I915_OA_FORMAT_A12_B8_C8 = 9, I915_OA_FORMAT_A32u40_A4u32_B8_C8 = 10, I915_OAR_FORMAT_A32u40_A4u32_B8_C8 = 11, I915_OA_FORMAT_A24u40_A14u32_B8_C8 = 12, I915_OAM_FORMAT_MPEC8u64_B8_C8 = 13, I915_OAM_FORMAT_MPEC8u32_B8_C8 = 14, I915_OA_FORMAT_MAX = 15 }
#[repr(i32)] pub enum drm_i915_perf_property_id { DRM_I915_PERF_PROP_CTX_HANDLE = 1, DRM_I915_PERF_PROP_SAMPLE_OA = 2, DRM_I915_PERF_PROP_OA_METRICS_SET = 3, DRM_I915_PERF_PROP_OA_FORMAT = 4, DRM_I915_PERF_PROP_OA_EXPONENT = 5, DRM_I915_PERF_PROP_HOLD_PREEMPTION = 6, DRM_I915_PERF_PROP_GLOBAL_SSEU = 7, DRM_I915_PERF_PROP_POLL_OA_PERIOD = 8, DRM_I915_PERF_PROP_OA_ENGINE_CLASS = 9, DRM_I915_PERF_PROP_OA_ENGINE_INSTANCE = 10, DRM_I915_PERF_PROP_MAX = 11 }
#[repr(C)] pub struct drm_i915_perf_open_param { pub flags: u32, pub num_properties: u32, pub properties_ptr: u64 }
pub const I915_PERF_FLAG_FD_CLOEXEC: u32 = 1 << 0; pub const I915_PERF_FLAG_FD_NONBLOCK: u32 = 1 << 1; pub const I915_PERF_FLAG_DISABLED: u32 = 1 << 2;
/* I915_PERF_IOCTL_ENABLE/DISABLE/CONFIG are _IO('i', 0x0..0x2) ioctl macros from external ioctl definitions. */
#[repr(C)] pub struct drm_i915_perf_record_header { pub type_: u32, pub pad: u16, pub size: u16 }
#[repr(i32)] pub enum drm_i915_perf_record_type { DRM_I915_PERF_RECORD_SAMPLE = 1, DRM_I915_PERF_RECORD_OA_REPORT_LOST = 2, DRM_I915_PERF_RECORD_OA_BUFFER_LOST = 3, DRM_I915_PERF_RECORD_MAX = 4 }
#[repr(C)] pub struct drm_i915_perf_oa_config { pub uuid: [i8; 36], pub n_mux_regs: u32, pub n_boolean_regs: u32, pub n_flex_regs: u32, pub mux_regs_ptr: u64, pub boolean_regs_ptr: u64, pub flex_regs_ptr: u64 }

#[repr(C)] pub struct drm_i915_query_item { pub query_id: u64, pub length: i32, pub flags: u32, pub data_ptr: u64 }
pub const DRM_I915_QUERY_TOPOLOGY_INFO: u64 = 1; pub const DRM_I915_QUERY_ENGINE_INFO: u64 = 2; pub const DRM_I915_QUERY_PERF_CONFIG: u64 = 3; pub const DRM_I915_QUERY_MEMORY_REGIONS: u64 = 4; pub const DRM_I915_QUERY_HWCONFIG_BLOB: u64 = 5; pub const DRM_I915_QUERY_GEOMETRY_SUBSLICES: u64 = 6; pub const DRM_I915_QUERY_GUC_SUBMISSION_VERSION: u64 = 7;
pub const DRM_I915_QUERY_PERF_CONFIG_LIST: u32 = 1; pub const DRM_I915_QUERY_PERF_CONFIG_DATA_FOR_UUID: u32 = 2; pub const DRM_I915_QUERY_PERF_CONFIG_DATA_FOR_ID: u32 = 3;
#[repr(C)] pub struct drm_i915_query { pub num_items: u32, pub flags: u32, pub items_ptr: u64 }
#[repr(C)] pub struct drm_i915_query_topology_info { pub flags: u16, pub max_slices: u16, pub max_subslices: u16, pub max_eus_per_subslice: u16, pub subslice_offset: u16, pub subslice_stride: u16, pub eu_offset: u16, pub eu_stride: u16, pub data: [u8; 0] }
#[repr(C)] pub struct drm_i915_engine_info { pub engine: i915_engine_class_instance, pub rsvd0: u32, pub flags: u64, pub capabilities: u64, pub logical_instance: u16, pub rsvd1: [u16; 3], pub rsvd2: [u64; 3] }
pub const I915_ENGINE_INFO_HAS_LOGICAL_INSTANCE: u64 = 1 << 0; pub const I915_VIDEO_CLASS_CAPABILITY_HEVC: u64 = 1 << 0; pub const I915_VIDEO_AND_ENHANCE_CLASS_CAPABILITY_SFC: u64 = 1 << 1;
#[repr(C)] pub struct drm_i915_query_engine_info { pub num_engines: u32, pub rsvd: [u32; 3], pub engines: [drm_i915_engine_info; 0] }
#[repr(C)] pub union drm_i915_query_perf_config_union { pub n_configs: u64, pub config: u64, pub uuid: [i8; 36] }
#[repr(C)] pub struct drm_i915_query_perf_config { pub u: drm_i915_query_perf_config_union, pub flags: u32, pub data: [u8; 0] }

#[repr(i32)] pub enum drm_i915_gem_memory_class { I915_MEMORY_CLASS_SYSTEM = 0, I915_MEMORY_CLASS_DEVICE = 1 }
#[repr(C)] pub struct drm_i915_gem_memory_class_instance { pub memory_class: u16, pub memory_instance: u16 }
#[repr(C)] pub struct drm_i915_memory_region_info_cpu_visible { pub probed_cpu_visible_size: u64, pub unallocated_cpu_visible_size: u64 }
#[repr(C)] pub union drm_i915_memory_region_info_union { pub rsvd1: [u64; 8], pub cpu_visible: drm_i915_memory_region_info_cpu_visible }
#[repr(C)] pub struct drm_i915_memory_region_info { pub region: drm_i915_gem_memory_class_instance, pub rsvd0: u32, pub probed_size: u64, pub unallocated_size: u64, pub u: drm_i915_memory_region_info_union }
#[repr(C)] pub struct drm_i915_query_memory_regions { pub num_regions: u32, pub rsvd: [u32; 3], pub regions: [drm_i915_memory_region_info; 0] }
#[repr(C)] pub struct drm_i915_query_guc_submission_version { pub branch: u32, pub major: u32, pub minor: u32, pub patch: u32 }

#[repr(C)] pub struct drm_i915_gem_create_ext { pub size: u64, pub handle: u32, pub flags: u32, pub extensions: u64 }
pub const I915_GEM_CREATE_EXT_FLAG_NEEDS_CPU_ACCESS: u32 = 1 << 0; pub const I915_GEM_CREATE_EXT_MEMORY_REGIONS: u64 = 0; pub const I915_GEM_CREATE_EXT_PROTECTED_CONTENT: u64 = 1; pub const I915_GEM_CREATE_EXT_SET_PAT: u64 = 2;
#[repr(C)] pub struct drm_i915_gem_create_ext_memory_regions { pub base: i915_user_extension, pub pad: u32, pub num_regions: u32, pub regions: u64 }
#[repr(C)] pub struct drm_i915_gem_create_ext_protected_content { pub base: i915_user_extension, pub flags: u32 }
#[repr(C)] pub struct drm_i915_gem_create_ext_set_pat { pub base: i915_user_extension, pub pat_index: u32, pub rsvd: u32 }
pub const I915_PROTECTED_CONTENT_DEFAULT_SESSION: u32 = 0xf;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
