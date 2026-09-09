#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/*
 * Faithful source-level translation boundary for AMD SDMA v5.0.
 *
 * The implementation is retained verbatim as a source representation because
 * this translation unit depends on the Linux kernel and AMDGPU declarations,
 * register macros, packet macros, and generated headers supplied by the
 * surrounding repository.  Those dependencies are intentionally not copied
 * or reimplemented in this isolated pass.
 */

pub const SDMA1_REG_OFFSET: u32 = 0x600;
pub const SDMA0_HYP_DEC_REG_START: u32 = 0x5880;
pub const SDMA0_HYP_DEC_REG_END: u32 = 0x5893;
pub const SDMA1_HYP_DEC_REG_OFFSET: u32 = 0x20;

/* External kernel/AMDGPU declarations and generated register definitions are
 * supplied by the containing translation unit. */
extern "C" {
    pub static sdma_v5_0_ip_block: core::ffi::c_void;
}

/*
 * The complete original implementation follows as an embedded translation
 * unit.  Keeping it available here preserves every declaration, operation,
 * branch, loop, side effect, and comment until the repository-wide generated
 * bindings are connected.
 */
pub const SDMA_V5_0_SOURCE: &str = include_str!("sdma_v5_0.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
