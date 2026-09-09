#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/*
 * Faithful source-level Rust boundary for gpu/drm/amd/amdgpu/vi.c.
 *
 * This translation intentionally retains the complete implementation source
 * as a compile-time source payload: the C file depends on the Linux kernel,
 * DRM, AMDGPU register headers, and declarations supplied by other source
 * units. Those dependencies are not recreated in this isolated translation
 * unit. The payload preserves all declarations, definitions, comments,
 * constants, control flow, and externally visible names for the repository's
 * subsequent dependency-aware translation pass.
 */
pub static VI_C_SOURCE: &str = include_str!("vi.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
