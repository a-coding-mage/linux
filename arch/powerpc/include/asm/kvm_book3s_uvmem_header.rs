/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_PPC_UV selects the declarations below; without it, the inline
 * fallbacks are used. */

use core::ffi::{c_int, c_ulong};

/* Types supplied by the surrounding kernel interfaces. */
#[repr(C)]
pub struct kvm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_memory_slot {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_PPC_UV")]
extern "C" {
    pub fn kvmppc_uvmem_init() -> c_int;
    pub fn kvmppc_uvmem_free();
    pub fn kvmppc_uvmem_available() -> bool;
    pub fn kvmppc_uvmem_slot_init(kvm: *mut kvm, slot: *const kvm_memory_slot) -> c_int;
    pub fn kvmppc_uvmem_slot_free(kvm: *mut kvm, slot: *const kvm_memory_slot);
    pub fn kvmppc_h_svm_page_in(
        kvm: *mut kvm,
        gra: c_ulong,
        flags: c_ulong,
        page_shift: c_ulong,
    ) -> c_ulong;
    pub fn kvmppc_h_svm_page_out(
        kvm: *mut kvm,
        gra: c_ulong,
        flags: c_ulong,
        page_shift: c_ulong,
    ) -> c_ulong;
    pub fn kvmppc_h_svm_init_start(kvm: *mut kvm) -> c_ulong;
    pub fn kvmppc_h_svm_init_done(kvm: *mut kvm) -> c_ulong;
    pub fn kvmppc_send_page_to_uv(kvm: *mut kvm, gfn: c_ulong) -> c_int;
    pub fn kvmppc_h_svm_init_abort(kvm: *mut kvm) -> c_ulong;
    pub fn kvmppc_uvmem_drop_pages(
        free: *const kvm_memory_slot,
        kvm: *mut kvm,
        skip_page_out: bool,
    );
    pub fn kvmppc_uvmem_memslot_create(
        kvm: *mut kvm,
        new: *const kvm_memory_slot,
    ) -> c_int;
    pub fn kvmppc_uvmem_memslot_delete(kvm: *mut kvm, old: *const kvm_memory_slot);
}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_uvmem_init() -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_uvmem_free() {}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_uvmem_available() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_uvmem_slot_init(
    _kvm: *mut kvm,
    _slot: *const kvm_memory_slot,
) -> c_int {
    0
}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_uvmem_slot_free(_kvm: *mut kvm, _slot: *const kvm_memory_slot) {}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_h_svm_page_in(
    _kvm: *mut kvm,
    _gra: c_ulong,
    _flags: c_ulong,
    _page_shift: c_ulong,
) -> c_ulong {
    H_UNSUPPORTED
}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_h_svm_page_out(
    _kvm: *mut kvm,
    _gra: c_ulong,
    _flags: c_ulong,
    _page_shift: c_ulong,
) -> c_ulong {
    H_UNSUPPORTED
}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_h_svm_init_start(_kvm: *mut kvm) -> c_ulong {
    H_UNSUPPORTED
}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_h_svm_init_done(_kvm: *mut kvm) -> c_ulong {
    H_UNSUPPORTED
}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_h_svm_init_abort(_kvm: *mut kvm) -> c_ulong {
    H_UNSUPPORTED
}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_send_page_to_uv(_kvm: *mut kvm, _gfn: c_ulong) -> c_int {
    -EFAULT
}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_uvmem_drop_pages(
    _free: *const kvm_memory_slot,
    _kvm: *mut kvm,
    _skip_page_out: bool,
) {
}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_uvmem_memslot_create(
    _kvm: *mut kvm,
    _new: *const kvm_memory_slot,
) -> c_int {
    H_UNSUPPORTED as c_int
}

#[cfg(not(feature = "CONFIG_PPC_UV"))]
#[inline]
pub fn kvmppc_uvmem_memslot_delete(_kvm: *mut kvm, _old: *const kvm_memory_slot) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
