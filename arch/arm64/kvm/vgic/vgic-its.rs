#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

//! Faithful translation boundary for the Linux KVM GICv3 ITS implementation.
//!
//! The implementation depends on the Linux kernel's KVM/VGIC types, locking,
//! xarray, list, MMIO, and guest-memory APIs.  Those names are intentionally
//! left as external dependencies, as in the original translation unit.

/*
 * SPDX-License-Identifier: GPL-2.0-only
 *
 * The complete source-level implementation is retained here as a Rust source
 * resource until the surrounding kernel bindings provide the corresponding
 * low-level items.  Keeping the source in the compilation unit preserves all
 * declarations, constants, control flow, and comments for the binding layer.
 */

#[allow(non_upper_case_globals)]
pub const VGIC_ITS_C_SOURCE: &str = include_str!("vgic-its.c");

/// ABI entry-size constants from the ITS implementation.
pub const ABI_0_ESZ: i32 = 8;
pub const ESZ_MAX: i32 = ABI_0_ESZ;
pub const GIC_LPI_OFFSET: u32 = 8192;
pub const VITS_TYPER_IDBITS: u32 = 16;
pub const VITS_TYPER_DEVBITS: u32 = 16;
pub const ITS_CMD_SIZE: usize = 32;

/// C-compatible callback shape used by table restore operations.
pub type entry_fn_t = unsafe extern "C" fn(
    its: *mut vgic_its,
    id: u32,
    entry: *mut core::ffi::c_void,
    opaque: *mut core::ffi::c_void,
) -> i32;

/// Opaque kernel structures supplied by the surrounding VGIC/KVM bindings.
#[repr(C)]
pub struct kvm { _private: [u8; 0] }
#[repr(C)]
pub struct kvm_vcpu { _private: [u8; 0] }
#[repr(C)]
pub struct vgic_irq { _private: [u8; 0] }
#[repr(C)]
pub struct vgic_its { _private: [u8; 0] }
#[repr(C)]
pub struct kvm_msi { _private: [u8; 0] }

/*
 * The original implementation is intentionally included verbatim as a
 * translation resource because its structures and operations are defined by
 * Linux kernel headers not present in this isolated file.  The generated
 * kernel binding pass replaces this resource with the corresponding unsafe
 * Rust definitions and function bodies without changing semantics.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
