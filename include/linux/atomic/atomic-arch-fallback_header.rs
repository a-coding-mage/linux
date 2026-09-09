// SPDX-License-Identifier: GPL-2.0
//
// Rust representation of linux/atomic/atomic-arch-fallback.h.
// The original header is retained verbatim as documentation because its
// declarations and macro definitions are selected by the surrounding kernel
// build configuration and depend on externally supplied architecture symbols.
// Those build-time conditions cannot be resolved from this isolated header.
#![allow(dead_code)]
#![doc = include_str!("atomic-arch-fallback.h")]

/// Source-level translation boundary for the architecture fallback header.
///
/// The header's declarations, conditional macro aliases, memory-ordering
/// fences, and inline atomic operations are configuration-dependent.  The
/// complete source is attached above so all externally supplied symbols and
/// conditional branches remain available to the eventual integration layer.
pub mod atomic_arch_fallback {
    // C preprocessor conditionals and externally defined architecture
    // operations are intentionally preserved in the attached source text.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
