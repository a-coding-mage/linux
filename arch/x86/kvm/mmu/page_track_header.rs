/* SPDX-License-Identifier: GPL-2.0 */
// Translated from page_track.h.
// C dependencies: <linux/kvm_host.h> and <asm/kvm_page_track.h>.

extern "C" {
    pub fn kvm_page_track_write_tracking_enabled(kvm: *mut kvm) -> bool;
    pub fn kvm_page_track_write_tracking_alloc(slot: *mut kvm_memory_slot) -> ::core::ffi::c_int;

    pub fn kvm_page_track_free_memslot(slot: *mut kvm_memory_slot);
    pub fn kvm_page_track_create_memslot(
        kvm: *mut kvm,
        slot: *mut kvm_memory_slot,
        npages: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;

    pub fn __kvm_write_track_add_gfn(kvm: *mut kvm, slot: *mut kvm_memory_slot, gfn: gfn_t);
    pub fn __kvm_write_track_remove_gfn(
        kvm: *mut kvm,
        slot: *mut kvm_memory_slot,
        gfn: gfn_t,
    );

    pub fn kvm_gfn_is_write_tracked(
        kvm: *mut kvm,
        slot: *const kvm_memory_slot,
        gfn: gfn_t,
    ) -> bool;
}

// Preserves CONFIG_KVM_EXTERNAL_WRITE_TRACKING. The external declarations are
// available when the corresponding build configuration is enabled.
#[cfg(feature = "CONFIG_KVM_EXTERNAL_WRITE_TRACKING")]
extern "C" {
    pub fn kvm_page_track_init(kvm: *mut kvm) -> ::core::ffi::c_int;
    pub fn kvm_page_track_cleanup(kvm: *mut kvm);

    pub fn __kvm_page_track_write(
        kvm: *mut kvm,
        gpa: gpa_t,
        new: *const u8,
        bytes: ::core::ffi::c_int,
    );
    pub fn kvm_page_track_delete_slot(kvm: *mut kvm, slot: *mut kvm_memory_slot);
}

#[cfg(feature = "CONFIG_KVM_EXTERNAL_WRITE_TRACKING")]
#[inline]
pub unsafe fn kvm_page_track_has_external_user(kvm: *mut kvm) -> bool {
    // Equivalent to: !hlist_empty(&kvm->arch.track_notifier_head.track_notifier_list)
    !hlist_empty(&(*kvm).arch.track_notifier_head.track_notifier_list)
}

#[cfg(not(feature = "CONFIG_KVM_EXTERNAL_WRITE_TRACKING"))]
#[inline]
pub unsafe fn kvm_page_track_init(_kvm: *mut kvm) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_KVM_EXTERNAL_WRITE_TRACKING"))]
#[inline]
pub unsafe fn kvm_page_track_cleanup(_kvm: *mut kvm) {}

#[cfg(not(feature = "CONFIG_KVM_EXTERNAL_WRITE_TRACKING"))]
#[inline]
pub unsafe fn __kvm_page_track_write(
    _kvm: *mut kvm,
    _gpa: gpa_t,
    _new: *const u8,
    _bytes: ::core::ffi::c_int,
) {
}

#[cfg(not(feature = "CONFIG_KVM_EXTERNAL_WRITE_TRACKING"))]
#[inline]
pub unsafe fn kvm_page_track_delete_slot(_kvm: *mut kvm, _slot: *mut kvm_memory_slot) {}

#[cfg(not(feature = "CONFIG_KVM_EXTERNAL_WRITE_TRACKING"))]
#[inline]
pub unsafe fn kvm_page_track_has_external_user(_kvm: *mut kvm) -> bool {
    false
}

#[inline]
pub unsafe fn kvm_page_track_write(
    vcpu: *mut kvm_vcpu,
    gpa: gpa_t,
    new: *const u8,
    bytes: ::core::ffi::c_int,
) {
    __kvm_page_track_write((*vcpu).kvm, gpa, new, bytes);
    kvm_mmu_track_write(vcpu, gpa, new, bytes);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
