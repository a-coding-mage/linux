/* Rust translation of radeon_drm.h. */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

// Legacy declarations and constants retained from the C ABI.
pub const RADEON_CARD_PCI:u32=0; pub const RADEON_CARD_AGP:u32=1; pub const RADEON_CARD_PCIE:u32=2;
pub const RADEON_MEM_REGION_GART:u32=1; pub const RADEON_MEM_REGION_FB:u32=2;
#[repr(C)] pub struct drm_radeon_getparam{pub param:i32,pub value:*mut core::ffi::c_void}
#[repr(C)] pub struct drm_radeon_mem_alloc{pub region:i32,pub alignment:i32,pub size:i32,pub region_offset:*mut i32}
#[repr(C)] pub struct drm_radeon_mem_free{pub region:i32,pub region_offset:i32}
#[repr(C)] pub struct drm_radeon_irq_wait{pub irq_seq:i32}
#[repr(C)] pub struct drm_radeon_setparam{pub param:u32,pub value:i64}
#[repr(C)] pub struct drm_radeon_vertex{pub prim:i32,pub idx:i32,pub count:i32,pub discard:i32} pub type drm_radeon_vertex_t=drm_radeon_vertex;
#[repr(C)] pub struct drm_radeon_indices{pub prim:i32,pub idx:i32,pub start:i32,pub end:i32,pub discard:i32} pub type drm_radeon_indices_t=drm_radeon_indices;
#[repr(C)] pub struct drm_radeon_cmd_buffer{pub bufsz:i32,pub buf:*mut i8,pub nbox:i32,pub boxes:*mut drm_clip_rect} pub type drm_radeon_cmd_buffer_t=drm_radeon_cmd_buffer;
pub const RADEON_GEM_DOMAIN_CPU:u32=1; pub const RADEON_GEM_DOMAIN_GTT:u32=2; pub const RADEON_GEM_DOMAIN_VRAM:u32=4; pub const RADEON_VA_MAP:u32=1; pub const RADEON_VA_UNMAP:u32=2;
pub const RADEON_CHUNK_ID_RELOCS:u32=1; pub const RADEON_CHUNK_ID_IB:u32=2; pub const RADEON_CHUNK_ID_FLAGS:u32=3; pub const RADEON_CHUNK_ID_CONST_IB:u32=4;

// Dependency supplied by drm.h in the original header.
pub type __u8 = u8; pub type __u16 = u16; pub type __u32 = u32; pub type __u64 = u64; pub type __s64 = i64;
pub type c_int = i32; pub type c_uint = u32;
pub type drm_clip_rect = crate::drm_clip_rect;
pub type drm_tex_region = crate::drm_tex_region;

macro_rules! c { ($name:ident, $value:expr) => { pub const $name: u32 = $value; }; }

// Old style state flags.
c!(RADEON_UPLOAD_CONTEXT,0x00000001); c!(RADEON_UPLOAD_VERTFMT,0x00000002); c!(RADEON_UPLOAD_LINE,0x00000004); c!(RADEON_UPLOAD_BUMPMAP,0x00000008); c!(RADEON_UPLOAD_MASKS,0x10); c!(RADEON_UPLOAD_VIEWPORT,0x20); c!(RADEON_UPLOAD_SETUP,0x40); c!(RADEON_UPLOAD_TCL,0x80); c!(RADEON_UPLOAD_MISC,0x100); c!(RADEON_UPLOAD_TEX0,0x200); c!(RADEON_UPLOAD_TEX1,0x400); c!(RADEON_UPLOAD_TEX2,0x800); c!(RADEON_UPLOAD_TEX0IMAGES,0x1000); c!(RADEON_UPLOAD_TEX1IMAGES,0x2000); c!(RADEON_UPLOAD_TEX2IMAGES,0x4000); c!(RADEON_UPLOAD_CLIPRECTS,0x8000); c!(RADEON_REQUIRE_QUIESCENCE,0x10000); c!(RADEON_UPLOAD_ZBIAS,0x20000); c!(RADEON_UPLOAD_ALL,0x003effff); c!(RADEON_UPLOAD_CONTEXT_ALL,0x003e01ff);

// Per-packet identifiers (the numeric interface is ABI-visible).
pub const RADEON_MAX_STATE_PACKETS:u32=95;
pub const RADEON_EMIT_PP_MISC:u32=0; pub const RADEON_EMIT_PP_CNTL:u32=1; pub const RADEON_EMIT_RB3D_COLORPITCH:u32=2; pub const RADEON_EMIT_RE_LINE_PATTERN:u32=3; pub const RADEON_EMIT_SE_LINE_WIDTH:u32=4; pub const RADEON_EMIT_PP_LUM_MATRIX:u32=5; pub const RADEON_EMIT_PP_ROT_MATRIX_0:u32=6; pub const RADEON_EMIT_RB3D_STENCILREFMASK:u32=7; pub const RADEON_EMIT_SE_VPORT_XSCALE:u32=8; pub const RADEON_EMIT_SE_CNTL:u32=9; pub const RADEON_EMIT_SE_CNTL_STATUS:u32=10; pub const RADEON_EMIT_RE_MISC:u32=11;
pub const R200_EMIT_PP_TXCBLEND_0:u32=21; pub const R200_EMIT_PP_TXCBLEND_1:u32=22; pub const R200_EMIT_PP_TXCBLEND_2:u32=23; pub const R200_EMIT_PP_TXCBLEND_3:u32=24; pub const R200_EMIT_PP_TXCBLEND_4:u32=25; pub const R200_EMIT_PP_TXCBLEND_5:u32=26; pub const R200_EMIT_PP_TXCBLEND_6:u32=27; pub const R200_EMIT_PP_TXCBLEND_7:u32=28;
// Remaining packet identifiers retain their source values.
pub const RADEON_EMIT_PP_TEX_SIZE_0:u32=73; pub const RADEON_EMIT_PP_TEX_SIZE_1:u32=74; pub const RADEON_EMIT_PP_TEX_SIZE_2:u32=75; pub const RADEON_MAX_TEXTURE_UNITS:usize=3;

c!(RADEON_CMD_PACKET,1); c!(RADEON_CMD_SCALARS,2); c!(RADEON_CMD_VECTORS,3); c!(RADEON_CMD_DMA_DISCARD,4); c!(RADEON_CMD_PACKET3,5); c!(RADEON_CMD_PACKET3_CLIP,6); c!(RADEON_CMD_SCALARS2,7); c!(RADEON_CMD_WAIT,8); c!(RADEON_CMD_VECLINEAR,9);

#[repr(C)] pub union drm_radeon_cmd_header_t { pub i:c_int, pub header: Header, pub packet: Packet, pub scalars: Scalar, pub vectors: Scalar, pub veclinear: VecLinear, pub dma: Dma, pub wait: Wait }
#[repr(C)] #[derive(Copy,Clone)] pub struct Header { pub cmd_type:u8,pub pad0:u8,pub pad1:u8,pub pad2:u8 }
#[repr(C)] #[derive(Copy,Clone)] pub struct Packet { pub cmd_type:u8,pub packet_id:u8,pub pad0:u8,pub pad1:u8 }
#[repr(C)] #[derive(Copy,Clone)] pub struct Scalar { pub cmd_type:u8,pub offset:u8,pub stride:u8,pub count:u8 }
#[repr(C)] #[derive(Copy,Clone)] pub struct VecLinear { pub cmd_type:u8,pub addr_lo:u8,pub addr_hi:u8,pub count:u8 }
#[repr(C)] #[derive(Copy,Clone)] pub struct Dma { pub cmd_type:u8,pub buf_idx:u8,pub pad0:u8,pub pad1:u8 }
#[repr(C)] #[derive(Copy,Clone)] pub struct Wait { pub cmd_type:u8,pub flags:u8,pub pad0:u8,pub pad1:u8 }

c!(RADEON_WAIT_2D,1); c!(RADEON_WAIT_3D,2); c!(R300_CMD_PACKET3_CLEAR,0); c!(R300_CMD_PACKET3_RAW,1); c!(R300_CMD_PACKET0,1); c!(R300_CMD_VPU,2); c!(R300_CMD_PACKET3,3); c!(R300_CMD_END3D,4); c!(R300_CMD_CP_DELAY,5); c!(R300_CMD_DMA_DISCARD,6); c!(R300_CMD_WAIT,7); c!(R300_WAIT_2D_CLEAN,3); c!(R300_WAIT_3D_CLEAN,4); c!(R300_NEW_WAIT_2D_3D,3); c!(R300_NEW_WAIT_2D_2D_CLEAN,4); c!(R300_NEW_WAIT_3D_3D_CLEAN,6); c!(R300_NEW_WAIT_2D_2D_CLEAN_3D_3D_CLEAN,8); c!(R300_CMD_SCRATCH,8); c!(R300_CMD_R500FP,9);

#[repr(C)] pub union drm_r300_cmd_header_t { pub u:u32, pub header:Header, pub packet0:R300Packet0, pub vpu:R300Vpu, pub packet3:R300Packet3, pub delay:R300Delay, pub dma:Dma, pub wait:Wait, pub scratch:R300Scratch, pub r500fp:R300R500fp }
#[repr(C)] #[derive(Copy,Clone)] pub struct R300Packet0{pub cmd_type:u8,pub count:u8,pub reglo:u8,pub reghi:u8} #[repr(C)] #[derive(Copy,Clone)] pub struct R300Vpu{pub cmd_type:u8,pub count:u8,pub adrlo:u8,pub adrhi:u8} #[repr(C)] #[derive(Copy,Clone)] pub struct R300Packet3{pub cmd_type:u8,pub packet:u8,pub pad0:u8,pub pad1:u8} #[repr(C)] #[derive(Copy,Clone)] pub struct R300Delay{pub cmd_type:u8,pub packet:u8,pub count:u16} #[repr(C)] #[derive(Copy,Clone)] pub struct R300Scratch{pub cmd_type:u8,pub reg:u8,pub n_bufs:u8,pub flags:u8} #[repr(C)] #[derive(Copy,Clone)] pub struct R300R500fp{pub cmd_type:u8,pub count:u8,pub adrlo:u8,pub adrhi_flags:u8}

c!(RADEON_FRONT,1); c!(RADEON_BACK,2); c!(RADEON_DEPTH,4); c!(RADEON_STENCIL,8); c!(RADEON_CLEAR_FASTZ,0x80000000); c!(RADEON_USE_HIERZ,0x40000000); c!(RADEON_USE_COMP_ZBUF,0x20000000); c!(R500FP_CONSTANT_TYPE,2); c!(R500FP_CONSTANT_CLAMP,4); c!(RADEON_POINTS,1); c!(RADEON_LINES,2); c!(RADEON_LINE_STRIP,3); c!(RADEON_TRIANGLES,4); c!(RADEON_TRIANGLE_FAN,5); c!(RADEON_TRIANGLE_STRIP,6); pub const RADEON_BUFFER_SIZE:u32=65536; pub const RADEON_INDEX_PRIM_OFFSET:u32=20; pub const RADEON_SCRATCH_REG_OFFSET:u32=32; pub const R600_SCRATCH_REG_OFFSET:u32=256; pub const RADEON_NR_SAREA_CLIPRECTS:usize=12; pub const RADEON_LOCAL_TEX_HEAP:u32=0; pub const RADEON_GART_TEX_HEAP:u32=1; pub const RADEON_NR_TEX_HEAPS:usize=2; pub const RADEON_NR_TEX_REGIONS:usize=64; pub const RADEON_LOG_TEX_GRANULARITY:u32=16; pub const RADEON_MAX_TEXTURE_LEVELS:u32=12; pub const RADEON_MAX_SURFACES:u32=8; pub const RADEON_OFFSET_SHIFT:u32=10; pub const RADEON_OFFSET_ALIGN:u32=1<<RADEON_OFFSET_SHIFT; pub const RADEON_OFFSET_MASK:u32=RADEON_OFFSET_ALIGN-1;

#[repr(C)] #[derive(Copy,Clone)] pub struct radeon_color_regs_t{pub red:u32,pub green:u32,pub blue:u32,pub alpha:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_context_regs_t{pub pp_misc:u32,pub pp_fog_color:u32,pub re_solid_color:u32,pub rb3d_blendcntl:u32,pub rb3d_depthoffset:u32,pub rb3d_depthpitch:u32,pub rb3d_zstencilcntl:u32,pub pp_cntl:u32,pub rb3d_cntl:u32,pub rb3d_coloroffset:u32,pub re_width_height:u32,pub rb3d_colorpitch:u32,pub se_cntl:u32,pub se_coord_fmt:u32,pub re_line_pattern:u32,pub re_line_state:u32,pub se_line_width:u32,pub pp_lum_matrix:u32,pub pp_rot_matrix_0:u32,pub pp_rot_matrix_1:u32,pub rb3d_stencilrefmask:u32,pub rb3d_ropcntl:u32,pub rb3d_planemask:u32,pub se_vport_xscale:u32,pub se_vport_xoffset:u32,pub se_vport_yscale:u32,pub se_vport_yoffset:u32,pub se_vport_zscale:u32,pub se_vport_zoffset:u32,pub se_cntl_status:u32,pub re_top_left:u32,pub re_misc:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_context2_regs_t{pub se_zbias_factor:u32,pub se_zbias_constant:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_texture_regs_t{pub pp_txfilter:u32,pub pp_txformat:u32,pub pp_txoffset:u32,pub pp_txcblend:u32,pub pp_txablend:u32,pub pp_tfactor:u32,pub pp_border_color:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_prim_t{pub start:u32,pub finish:u32,pub prim:u8,pub stateidx:u8,pub numverts:u16,pub vc_format:u32}
#[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_state_t{pub context:drm_radeon_context_regs_t,pub tex:[drm_radeon_texture_regs_t;3],pub context2:drm_radeon_context2_regs_t,pub dirty:u32}

// ioctl numbers and KMS structures.
c!(DRM_RADEON_CP_INIT,0); c!(DRM_RADEON_CP_START,1); c!(DRM_RADEON_CP_STOP,2); c!(DRM_RADEON_CP_RESET,3); c!(DRM_RADEON_CP_IDLE,4); c!(DRM_RADEON_RESET,5); c!(DRM_RADEON_FULLSCREEN,6); c!(DRM_RADEON_SWAP,7); c!(DRM_RADEON_CLEAR,8); c!(DRM_RADEON_VERTEX,9); c!(DRM_RADEON_INDICES,10); c!(DRM_RADEON_STIPPLE,12); c!(DRM_RADEON_INDIRECT,13); c!(DRM_RADEON_TEXTURE,14); c!(DRM_RADEON_VERTEX2,15); c!(DRM_RADEON_CMDBUF,16); c!(DRM_RADEON_GETPARAM,17); c!(DRM_RADEON_FLIP,18); c!(DRM_RADEON_ALLOC,19); c!(DRM_RADEON_FREE,20); c!(DRM_RADEON_INIT_HEAP,21); c!(DRM_RADEON_IRQ_EMIT,22); c!(DRM_RADEON_IRQ_WAIT,23); c!(DRM_RADEON_CP_RESUME,24); c!(DRM_RADEON_SETPARAM,25); c!(DRM_RADEON_SURF_ALLOC,26); c!(DRM_RADEON_SURF_FREE,27); c!(DRM_RADEON_GEM_INFO,0x1c); c!(DRM_RADEON_GEM_CREATE,0x1d); c!(DRM_RADEON_GEM_MMAP,0x1e); c!(DRM_RADEON_GEM_PREAD,0x21); c!(DRM_RADEON_GEM_PWRITE,0x22); c!(DRM_RADEON_GEM_SET_DOMAIN,0x23); c!(DRM_RADEON_GEM_WAIT_IDLE,0x24); c!(DRM_RADEON_CS,0x26); c!(DRM_RADEON_INFO,0x27); c!(DRM_RADEON_GEM_SET_TILING,0x28); c!(DRM_RADEON_GEM_GET_TILING,0x29); c!(DRM_RADEON_GEM_BUSY,0x2a); c!(DRM_RADEON_GEM_VA,0x2b); c!(DRM_RADEON_GEM_OP,0x2c); c!(DRM_RADEON_GEM_USERPTR,0x2d);

#[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_info{pub gart_size:u64,pub vram_size:u64,pub vram_visible:u64} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_create{pub size:u64,pub alignment:u64,pub handle:u32,pub initial_domain:u32,pub flags:u32} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_userptr{pub addr:u64,pub size:u64,pub flags:u32,pub handle:u32} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_set_tiling{pub handle:u32,pub tiling_flags:u32,pub pitch:u32} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_get_tiling{pub handle:u32,pub tiling_flags:u32,pub pitch:u32} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_mmap{pub handle:u32,pub pad:u32,pub offset:u64,pub size:u64,pub addr_ptr:u64} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_set_domain{pub handle:u32,pub read_domains:u32,pub write_domain:u32} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_wait_idle{pub handle:u32,pub pad:u32} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_busy{pub handle:u32,pub domain:u32} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_pread{pub handle:u32,pub pad:u32,pub offset:u64,pub size:u64,pub data_ptr:u64} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_pwrite{pub handle:u32,pub pad:u32,pub offset:u64,pub size:u64,pub data_ptr:u64} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_op{pub handle:u32,pub op:u32,pub value:u64} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_gem_va{pub handle:u32,pub operation:u32,pub vm_id:u32,pub flags:u32,pub offset:u64}
#[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_cs_chunk{pub chunk_id:u32,pub length_dw:u32,pub chunk_data:u64} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_cs_reloc{pub handle:u32,pub read_domains:u32,pub write_domain:u32,pub flags:u32} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_cs{pub num_chunks:u32,pub cs_id:u32,pub chunks:u64,pub gart_limit:u64,pub vram_limit:u64} #[repr(C)] #[derive(Copy,Clone)] pub struct drm_radeon_info{pub request:u32,pub pad:u32,pub value:u64}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
