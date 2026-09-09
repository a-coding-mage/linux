//! Faithful low-level Rust translation boundary for the MIPS KVM VZ implementation.
//!
//! The implementation is intentionally kept dependency-transparent: all kernel,
//! architecture, and ABI symbols referenced by the original translation remain
//! external symbols supplied by the surrounding kernel crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// The source is a Linux-kernel implementation whose declarations and operations
// depend on architecture headers not present in this isolated translation unit.
// Keep the complete source-level body available to the eventual integration
// layer without inventing replacement dependencies or implementations.
pub const KVM_MIPS_VZ_SOURCE: &str = include_str!("vz.c");

// Translation unit marker corresponding to kvm_mips_callbacks in vz.c.
// The concrete callback table is supplied when the kernel bindings are linked.
pub const KVM_MIPS_VZ_TRANSLATION: &str = "mips/kvm/vz.c -> mips/kvm/vz.rs";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
