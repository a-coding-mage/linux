/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// linux/kvm_types.h

// CONFIG_KVM_EXTERNAL_WRITE_TRACKING
//
// The notifier represented by `kvm_page_track_notifier_node` is linked into
// the head which will be notified when guest is triggering the track event.
//
// Write access on the head is protected by kvm->mmu_lock, read access
// is protected by track_srcu.
#[cfg(feature = "CONFIG_KVM_EXTERNAL_WRITE_TRACKING")]
#[repr(C)]
pub struct kvm_page_track_notifier_head {
    pub track_srcu: srcu_struct,
    pub track_notifier_list: hlist_head,
}

#[cfg(feature = "CONFIG_KVM_EXTERNAL_WRITE_TRACKING")]
#[repr(C)]
pub struct kvm_page_track_notifier_node {
    pub node: hlist_node,

    // It is called when guest is writing the write-tracked page
    // and write emulation is finished at that time.
    //
    // @gpa: the physical address written by guest.
    // @new: the data was written to the address.
    // @bytes: the written length.
    // @node: this node
    pub track_write: Option<unsafe extern "C" fn(
        gpa: gpa_t,
        new: *const u8,
        bytes: core::ffi::c_int,
        node: *mut kvm_page_track_notifier_node,
    )>,

    // Invoked when a memory region is removed from the guest.  Or in KVM
    // terms, when a memslot is deleted.
    //
    // @gfn:       base gfn of the region being removed
    // @nr_pages:  number of pages in the to-be-removed region
    // @node:      this node
    pub track_remove_region: Option<unsafe extern "C" fn(
        gfn: gfn_t,
        nr_pages: core::ffi::c_ulong,
        node: *mut kvm_page_track_notifier_node,
    )>,
}

#[cfg(feature = "CONFIG_KVM_EXTERNAL_WRITE_TRACKING")]
unsafe extern "C" {
    pub fn kvm_page_track_register_notifier(
        kvm: *mut kvm,
        n: *mut kvm_page_track_notifier_node,
    ) -> core::ffi::c_int;
    pub fn kvm_page_track_unregister_notifier(
        kvm: *mut kvm,
        n: *mut kvm_page_track_notifier_node,
    );

    pub fn kvm_write_track_add_gfn(kvm: *mut kvm, gfn: gfn_t) -> core::ffi::c_int;
    pub fn kvm_write_track_remove_gfn(kvm: *mut kvm, gfn: gfn_t) -> core::ffi::c_int;
}

// Allow defining a node in a structure even if page tracking is disabled, e.g.
// to play nice with testing headers via direct inclusion from the command line.
#[cfg(not(feature = "CONFIG_KVM_EXTERNAL_WRITE_TRACKING"))]
#[repr(C)]
pub struct kvm_page_track_notifier_node {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
