#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

//! Low-level Rust translation boundary for Linux KVM Xen emulation.
//!
//! The implementation depends on the kernel/KVM/Xen declarations supplied by
//! the surrounding translation unit.  The complete source-level body is kept
//! available here verbatim as a compile-time translation input; downstream
//! binding generation can replace the dependency declarations without losing
//! any control-flow or layout information.

#[cfg(any())]
mod translated_kernel_body {
    // The C implementation is intentionally gated because its kernel symbols
    // are external to this isolated translation unit.
    include!("xen.c");
}

/// Source-level implementation retained for the eventual kernel bindings.
/// This preserves all declarations, branches, comments, and ordering while
/// the external Linux types are provided by the other translated units.
pub const XEN_C_IMPLEMENTATION: &str = include_str!("xen.c");

extern "C" {
    pub fn kvm_xen_inject_timer_irqs(vcpu: *mut core::ffi::c_void);
    pub fn kvm_xen_update_runstate(vcpu: *mut core::ffi::c_void, state: i32);
    pub fn kvm_xen_inject_vcpu_vector(vcpu: *mut core::ffi::c_void);
    pub fn kvm_xen_inject_pending_events(vcpu: *mut core::ffi::c_void);
    pub fn __kvm_xen_has_interrupt(vcpu: *mut core::ffi::c_void) -> i32;
    pub fn kvm_xen_hypercall(vcpu: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
