/* SPDX-License-Identifier: GPL-2.0 */
// Translated from trace_gmap.h.
// The C tracepoint include/definition machinery is represented by the Rust
// declarations below; the supplied kernel tracepoint dependencies remain
// external to this translation unit.

#[repr(C)]
pub struct kvm_s390_major_guest_pfault_entry {
    pub id: ::core::ffi::c_int,
    pub pswmask: ::core::ffi::c_ulong,
    pub pswaddr: ::core::ffi::c_ulong,
}

extern "C" {
    /// Trace event: kvm_s390_major_guest_pfault.
    pub fn kvm_s390_major_guest_pfault(vcpu: *mut kvm_vcpu);
}

/// External kernel type supplied by the tracepoint/KVM dependencies.
#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

/// Equivalent of the C `__KVM_ASSIGN` tracepoint assignment.
#[inline]
pub unsafe fn kvm_s390_major_guest_pfault_assign(
    entry: *mut kvm_s390_major_guest_pfault_entry,
    vcpu: *const kvm_vcpu,
) {
    // The complete `struct kvm_vcpu` layout is supplied externally.  These
    // field accesses preserve the source expression and its unsafe semantics.
    (*entry).id = (*vcpu).vcpu_id;
    (*entry).pswmask = (*vcpu).arch.sie_block.gpsw.mask;
    (*entry).pswaddr = (*vcpu).arch.sie_block.gpsw.addr;
}

/// Values printed by the C `__KVM_PRINT` tracepoint fragment.
#[inline]
pub unsafe fn kvm_s390_major_guest_pfault_print(
    entry: *const kvm_s390_major_guest_pfault_entry,
) -> (::core::ffi::c_ulong, ::core::ffi::c_ulong) {
    ((*entry).pswmask, (*entry).pswaddr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
