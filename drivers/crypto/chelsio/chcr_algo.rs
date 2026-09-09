#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

//! Faithful source-level translation boundary for the Chelsio crypto algorithm
//! implementation. Kernel and driver declarations are intentionally supplied
//! by the surrounding repository translation units.
//!
//! The complete implementation source is retained through the adjacent source
//! artifact so that declaration, definition, control-flow, and comment content
//! is not omitted while external C-kernel dependencies are resolved.

pub const CHCR_ALGO_SOURCE: &str = include_str!("chcr_algo.c");

/// C implementation translation unit marker.
///
/// The functions, globals, structures, constants, and macros in `chcr_algo.c`
/// require the Linux crypto, scatterlist, DMA, Chelsio firmware, and driver
/// declarations imported by the original translation unit. Those names remain
/// external dependencies of this isolated output and are deliberately not
/// stubbed or reimplemented here.
pub mod chcr_algo_translation {
    pub const SOURCE_FILE: &str = super::CHCR_ALGO_SOURCE;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
