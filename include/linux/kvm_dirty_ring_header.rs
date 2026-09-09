// Translated from kvm_dirty_ring.h. The linux/kvm.h declarations are supplied
// by the surrounding translation unit.

#[repr(C)]
pub struct kvm_dirty_ring {
    pub dirty_index: u32,
    pub reset_index: u32,
    pub size: u32,
    pub soft_limit: u32,
    pub dirty_gfns: *mut kvm_dirty_gfn,
    pub index: core::ffi::c_int,
}

// If CONFIG_HAVE_KVM_DIRTY_RING is not defined, kvm_dirty_ring.o should not
// be included as well, so define these nop functions for the arch.
#[cfg(not(feature = "CONFIG_HAVE_KVM_DIRTY_RING"))]
#[inline]
pub unsafe fn kvm_dirty_ring_get_rsvd_entries(_kvm: *mut kvm) -> u32 {
    0
}

#[cfg(not(feature = "CONFIG_HAVE_KVM_DIRTY_RING"))]
#[inline]
pub unsafe fn kvm_use_dirty_bitmap(_kvm: *mut kvm) -> bool {
    true
}

#[cfg(not(feature = "CONFIG_HAVE_KVM_DIRTY_RING"))]
#[inline]
pub unsafe fn kvm_dirty_ring_alloc(
    _kvm: *mut kvm,
    _ring: *mut kvm_dirty_ring,
    _index: core::ffi::c_int,
    _size: u32,
) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_HAVE_KVM_DIRTY_RING"))]
#[inline]
pub unsafe fn kvm_dirty_ring_reset(
    _kvm: *mut kvm,
    _ring: *mut kvm_dirty_ring,
    _nr_entries_reset: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    -ENOENT
}

#[cfg(not(feature = "CONFIG_HAVE_KVM_DIRTY_RING"))]
#[inline]
pub unsafe fn kvm_dirty_ring_push(_vcpu: *mut kvm_vcpu, _slot: u32, _offset: u64) {}

#[cfg(not(feature = "CONFIG_HAVE_KVM_DIRTY_RING"))]
#[inline]
pub unsafe fn kvm_dirty_ring_get_page(
    _ring: *mut kvm_dirty_ring,
    _offset: u32,
) -> *mut page {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_HAVE_KVM_DIRTY_RING"))]
#[inline]
pub unsafe fn kvm_dirty_ring_free(_ring: *mut kvm_dirty_ring) {}

#[cfg(feature = "CONFIG_HAVE_KVM_DIRTY_RING")]
extern "C" {
    pub fn kvm_cpu_dirty_log_size(kvm: *mut kvm) -> core::ffi::c_int;
    pub fn kvm_use_dirty_bitmap(kvm: *mut kvm) -> bool;
    pub fn kvm_arch_allow_write_without_running_vcpu(kvm: *mut kvm) -> bool;
    pub fn kvm_dirty_ring_get_rsvd_entries(kvm: *mut kvm) -> u32;
    pub fn kvm_dirty_ring_alloc(
        kvm: *mut kvm,
        ring: *mut kvm_dirty_ring,
        index: core::ffi::c_int,
        size: u32,
    ) -> core::ffi::c_int;
    pub fn kvm_dirty_ring_reset(
        kvm: *mut kvm,
        ring: *mut kvm_dirty_ring,
        nr_entries_reset: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn kvm_dirty_ring_push(vcpu: *mut kvm_vcpu, slot: u32, offset: u64);
    pub fn kvm_dirty_ring_check_request(vcpu: *mut kvm_vcpu) -> bool;
    // For use in vm_operations_struct.
    pub fn kvm_dirty_ring_get_page(ring: *mut kvm_dirty_ring, offset: u32) -> *mut page;
    pub fn kvm_dirty_ring_free(ring: *mut kvm_dirty_ring);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
