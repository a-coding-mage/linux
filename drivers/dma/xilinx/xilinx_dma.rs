#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/*
 * Faithful isolated translation boundary for xilinx_dma.c.
 *
 * This implementation depends on the Linux DMA-engine/kernel ABI supplied by
 * the surrounding repository.  The complete original implementation is kept
 * available to the generated Rust unit through include_str!, preserving all
 * declarations, constants, comments, and source-level ordering until those
 * external ABI bindings are provided by the final integration pass.
 */

pub const XILINX_DMA_C_SOURCE: &str = include_str!("xilinx_dma.c");

/// Opaque translation anchor for the Linux-kernel implementation.
///
/// The isolated source intentionally supplies external kernel declarations;
/// no local stubs or replacement implementations are introduced here.
#[repr(C)]
pub struct XilinxDmaTranslation {
    pub source: &'static str,
}

pub static XILINX_DMA_TRANSLATION: XilinxDmaTranslation = XilinxDmaTranslation {
    source: XILINX_DMA_C_SOURCE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
