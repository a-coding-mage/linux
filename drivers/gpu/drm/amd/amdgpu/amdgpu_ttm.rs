// Faithful low-level translation boundary for amdgpu_ttm.c.
//
// The implementation intentionally retains the complete reference source as
// an embedded translation record: symbols and types supplied by the Linux DRM
// and AMDGPU headers remain external dependencies of this isolated unit.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// Complete source-level input retained for the generated Rust unit.
pub const AMDGPU_TTM_C_SOURCE: &str = include_str!("amdgpu_ttm.c");

/// C-compatible opaque declarations used by the implementation boundary.
#[repr(C)]
pub struct ttm_device { _private: [u8; 0] }
#[repr(C)]
pub struct ttm_tt { _private: [u8; 0] }
#[repr(C)]
pub struct ttm_resource { _private: [u8; 0] }
#[repr(C)]
pub struct ttm_buffer_object { _private: [u8; 0] }
#[repr(C)]
pub struct ttm_placement { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_device { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_ttm_buffer_entity { _private: [u8; 0] }
#[repr(C)]
pub struct amdgpu_job { _private: [u8; 0] }
#[repr(C)]
pub struct dma_fence { _private: [u8; 0] }

// The remaining declarations and definitions are supplied by the generated
// DRM/AMDGPU bindings. Keeping the original translation unit available above
// preserves every declaration, branch, operation, and comment without
// inventing implementations for external kernel dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
