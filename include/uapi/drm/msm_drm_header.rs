/* Rust translation of msm_drm.h. */

pub const MSM_PIPE_NONE: u32 = 0x00;
pub const MSM_PIPE_2D0: u32 = 0x01;
pub const MSM_PIPE_2D1: u32 = 0x02;
pub const MSM_PIPE_3D0: u32 = 0x10;
pub const MSM_PIPE_ID_MASK: u32 = 0xffff;
#[inline] pub const fn MSM_PIPE_ID(x: u32) -> u32 { x & MSM_PIPE_ID_MASK }
#[inline] pub const fn MSM_PIPE_FLAGS(x: u32) -> u32 { x & !MSM_PIPE_ID_MASK }

#[repr(C)]
pub struct drm_msm_timespec { pub tv_sec: i64, pub tv_nsec: i64 }

pub const MSM_PARAM_GPU_ID: u32 = 0x01;
pub const MSM_PARAM_GMEM_SIZE: u32 = 0x02;
pub const MSM_PARAM_CHIP_ID: u32 = 0x03;
pub const MSM_PARAM_MAX_FREQ: u32 = 0x04;
pub const MSM_PARAM_TIMESTAMP: u32 = 0x05;
pub const MSM_PARAM_GMEM_BASE: u32 = 0x06;
pub const MSM_PARAM_PRIORITIES: u32 = 0x07;
pub const MSM_PARAM_PP_PGTABLE: u32 = 0x08;
pub const MSM_PARAM_FAULTS: u32 = 0x09;
pub const MSM_PARAM_SUSPENDS: u32 = 0x0a;
pub const MSM_PARAM_SYSPROF: u32 = 0x0b;
pub const MSM_PARAM_COMM: u32 = 0x0c;
pub const MSM_PARAM_CMDLINE: u32 = 0x0d;
pub const MSM_PARAM_VA_START: u32 = 0x0e;
pub const MSM_PARAM_VA_SIZE: u32 = 0x0f;
pub const MSM_PARAM_HIGHEST_BANK_BIT: u32 = 0x10;
pub const MSM_PARAM_RAYTRACING: u32 = 0x11;
pub const MSM_PARAM_UBWC_SWIZZLE: u32 = 0x12;
pub const MSM_PARAM_MACROTILE_MODE: u32 = 0x13;
pub const MSM_PARAM_UCHE_TRAP_BASE: u32 = 0x14;
pub const MSM_PARAM_HAS_PRR: u32 = 0x15;
pub const MSM_PARAM_EN_VM_BIND: u32 = 0x16;
pub const MSM_PARAM_AQE: u32 = 0x17;
pub const MSM_PARAM_NR_RINGS: u32 = MSM_PARAM_PRIORITIES;

#[repr(C)] pub struct drm_msm_param { pub pipe: u32, pub param: u32, pub value: u64, pub len: u32, pub pad: u32 }
pub const MSM_BO_SCANOUT: u32 = 0x00000001;
pub const MSM_BO_GPU_READONLY: u32 = 0x00000002;
pub const MSM_BO_NO_SHARE: u32 = 0x00000004;
pub const MSM_BO_CACHE_MASK: u32 = 0x000f0000;
pub const MSM_BO_CACHED: u32 = 0x00010000;
pub const MSM_BO_WC: u32 = 0x00020000;
pub const MSM_BO_UNCACHED: u32 = 0x00040000;
pub const MSM_BO_CACHED_COHERENT: u32 = 0x080000;
pub const MSM_BO_FLAGS: u32 = MSM_BO_SCANOUT | MSM_BO_GPU_READONLY | MSM_BO_NO_SHARE | MSM_BO_CACHE_MASK;
#[repr(C)] pub struct drm_msm_gem_new { pub size: u64, pub flags: u32, pub handle: u32 }
pub const MSM_INFO_GET_OFFSET: u32 = 0x00;
pub const MSM_INFO_GET_IOVA: u32 = 0x01;
pub const MSM_INFO_SET_NAME: u32 = 0x02;
pub const MSM_INFO_GET_NAME: u32 = 0x03;
pub const MSM_INFO_SET_IOVA: u32 = 0x04;
pub const MSM_INFO_GET_FLAGS: u32 = 0x05;
pub const MSM_INFO_SET_METADATA: u32 = 0x06;
pub const MSM_INFO_GET_METADATA: u32 = 0x07;
#[repr(C)] pub struct drm_msm_gem_info { pub handle: u32, pub info: u32, pub value: u64, pub len: u32, pub pad: u32 }
pub const MSM_PREP_READ: u32 = 0x01; pub const MSM_PREP_WRITE: u32 = 0x02; pub const MSM_PREP_NOSYNC: u32 = 0x04; pub const MSM_PREP_BOOST: u32 = 0x08;
pub const MSM_PREP_FLAGS: u32 = MSM_PREP_READ | MSM_PREP_WRITE | MSM_PREP_NOSYNC | MSM_PREP_BOOST;
#[repr(C)] pub struct drm_msm_gem_cpu_prep { pub handle: u32, pub op: u32, pub timeout: drm_msm_timespec }
#[repr(C)] pub struct drm_msm_gem_cpu_fini { pub handle: u32 }
pub const MSM_SYNCOBJ_RESET: u32 = 0x00000001; pub const MSM_SYNCOBJ_FLAGS: u32 = MSM_SYNCOBJ_RESET;
#[repr(C)] pub struct drm_msm_syncobj { pub handle: u32, pub flags: u32, pub point: u64 }

#[repr(C)] pub struct drm_msm_gem_submit_reloc { pub submit_offset: u32, pub r#or: u32, pub shift: i32, pub reloc_idx: u32, pub reloc_offset: u64 }
pub const MSM_SUBMIT_CMD_BUF: u32 = 0x0001; pub const MSM_SUBMIT_CMD_IB_TARGET_BUF: u32 = 0x0002; pub const MSM_SUBMIT_CMD_CTX_RESTORE_BUF: u32 = 0x0003;
#[repr(C)] pub union drm_msm_gem_submit_cmd__bindgen_ty_1 { pub relocs: u64, pub iova: u64 }
#[repr(C)] pub struct drm_msm_gem_submit_cmd { pub type_: u32, pub submit_idx: u32, pub submit_offset: u32, pub size: u32, pub pad: u32, pub nr_relocs: u32, pub __bindgen_anon_1: drm_msm_gem_submit_cmd__bindgen_ty_1 }
pub const MSM_SUBMIT_BO_READ: u32 = 0x0001; pub const MSM_SUBMIT_BO_WRITE: u32 = 0x0002; pub const MSM_SUBMIT_BO_DUMP: u32 = 0x0004; pub const MSM_SUBMIT_BO_NO_IMPLICIT: u32 = 0x0008;
pub const MSM_SUBMIT_BO_FLAGS: u32 = MSM_SUBMIT_BO_READ | MSM_SUBMIT_BO_WRITE | MSM_SUBMIT_BO_DUMP | MSM_SUBMIT_BO_NO_IMPLICIT;
#[repr(C)] pub struct drm_msm_gem_submit_bo { pub flags: u32, pub handle: u32, pub presumed: u64 }
pub const MSM_SUBMIT_NO_IMPLICIT: u32 = 0x80000000; pub const MSM_SUBMIT_FENCE_FD_IN: u32 = 0x40000000; pub const MSM_SUBMIT_FENCE_FD_OUT: u32 = 0x20000000; pub const MSM_SUBMIT_SUDO: u32 = 0x10000000; pub const MSM_SUBMIT_SYNCOBJ_IN: u32 = 0x08000000; pub const MSM_SUBMIT_SYNCOBJ_OUT: u32 = 0x04000000; pub const MSM_SUBMIT_FENCE_SN_IN: u32 = 0x02000000;
pub const MSM_SUBMIT_FLAGS: u32 = MSM_SUBMIT_NO_IMPLICIT | MSM_SUBMIT_FENCE_FD_IN | MSM_SUBMIT_FENCE_FD_OUT | MSM_SUBMIT_SUDO | MSM_SUBMIT_SYNCOBJ_IN | MSM_SUBMIT_SYNCOBJ_OUT | MSM_SUBMIT_FENCE_SN_IN;
#[repr(C)] pub struct drm_msm_gem_submit { pub flags:u32,pub fence:u32,pub nr_bos:u32,pub nr_cmds:u32,pub bos:u64,pub cmds:u64,pub fence_fd:i32,pub queueid:u32,pub in_syncobjs:u64,pub out_syncobjs:u64,pub nr_in_syncobjs:u32,pub nr_out_syncobjs:u32,pub syncobj_stride:u32,pub pad:u32 }

pub const MSM_VM_BIND_OP_UNMAP:u32=0; pub const MSM_VM_BIND_OP_MAP:u32=1; pub const MSM_VM_BIND_OP_MAP_NULL:u32=2; pub const MSM_VM_BIND_OP_DUMP:u32=1; pub const MSM_VM_BIND_OP_FLAGS:u32=MSM_VM_BIND_OP_DUMP;
#[repr(C)] pub struct drm_msm_vm_bind_op { pub op:u32,pub handle:u32,pub obj_offset:u64,pub iova:u64,pub range:u64,pub flags:u32,pub pad:u32 }
pub const MSM_VM_BIND_FENCE_FD_IN:u32=1; pub const MSM_VM_BIND_FENCE_FD_OUT:u32=2; pub const MSM_VM_BIND_FLAGS:u32=MSM_VM_BIND_FENCE_FD_IN|MSM_VM_BIND_FENCE_FD_OUT;
#[repr(C)] pub union drm_msm_vm_bind__bindgen_ty_1 { pub op: drm_msm_vm_bind_op, pub ops:u64 }
#[repr(C)] pub struct drm_msm_vm_bind { pub flags:u32,pub nr_ops:u32,pub fence_fd:i32,pub queue_id:u32,pub in_syncobjs:u64,pub out_syncobjs:u64,pub nr_in_syncobjs:u32,pub nr_out_syncobjs:u32,pub syncobj_stride:u32,pub op_stride:u32,pub __bindgen_anon_1:drm_msm_vm_bind__bindgen_ty_1 }
pub const MSM_WAIT_FENCE_BOOST:u32=1; pub const MSM_WAIT_FENCE_FLAGS:u32=MSM_WAIT_FENCE_BOOST;
#[repr(C)] pub struct drm_msm_wait_fence { pub fence:u32,pub flags:u32,pub timeout:drm_msm_timespec,pub queueid:u32 }
pub const MSM_MADV_WILLNEED:u32=0; pub const MSM_MADV_DONTNEED:u32=1; pub const __MSM_MADV_PURGED:u32=2;
#[repr(C)] pub struct drm_msm_gem_madvise { pub handle:u32,pub madv:u32,pub retained:u32 }
pub const MSM_SUBMITQUEUE_ALLOW_PREEMPT:u32=1; pub const MSM_SUBMITQUEUE_VM_BIND:u32=2; pub const MSM_SUBMITQUEUE_FLAGS:u32=MSM_SUBMITQUEUE_ALLOW_PREEMPT|MSM_SUBMITQUEUE_VM_BIND;
#[repr(C)] pub struct drm_msm_submitqueue { pub flags:u32,pub prio:u32,pub id:u32 }
pub const MSM_SUBMITQUEUE_PARAM_FAULTS:u32=0;
#[repr(C)] pub struct drm_msm_submitqueue_query { pub data:u64,pub id:u32,pub param:u32,pub len:u32,pub pad:u32 }
pub const MSM_PERFCNTR_STREAM:u32=1; pub const MSM_PERFCNTR_UPDATE:u32=2; pub const MSM_PERFCNTR_FLAGS:u32=MSM_PERFCNTR_STREAM|MSM_PERFCNTR_UPDATE;
#[repr(C)] pub struct drm_msm_perfcntr_group { pub group_name:[core::ffi::c_char;16],pub nr_countables:u32,pub pad:u32,pub countables:u64 }
#[repr(C)] pub struct drm_msm_perfcntr_config { pub flags:u32,pub nr_groups:u32,pub groups:u64,pub period:u64,pub bufsz_shift:u32,pub group_stride:u32 }

pub const DRM_MSM_GET_PARAM:u32=0x00; pub const DRM_MSM_SET_PARAM:u32=0x01; pub const DRM_MSM_GEM_NEW:u32=0x02; pub const DRM_MSM_GEM_INFO:u32=0x03; pub const DRM_MSM_GEM_CPU_PREP:u32=0x04; pub const DRM_MSM_GEM_CPU_FINI:u32=0x05; pub const DRM_MSM_GEM_SUBMIT:u32=0x06; pub const DRM_MSM_WAIT_FENCE:u32=0x07; pub const DRM_MSM_GEM_MADVISE:u32=0x08; pub const DRM_MSM_SUBMITQUEUE_NEW:u32=0x0A; pub const DRM_MSM_SUBMITQUEUE_CLOSE:u32=0x0B; pub const DRM_MSM_SUBMITQUEUE_QUERY:u32=0x0C; pub const DRM_MSM_VM_BIND:u32=0x0D; pub const DRM_MSM_PERFCNTR_CONFIG:u32=0x0E;
// DRM ioctl encodings depend on declarations supplied by drm.h; preserve the source macros here.
/* DRM_IOCTL_MSM_* are DRM_IOW/DRM_IOWR(DRM_COMMAND_BASE + DRM_MSM_*, type). */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
