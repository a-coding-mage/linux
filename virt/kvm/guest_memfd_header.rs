/* SPDX-License-Identifier: GPL-2.0-only */

// C header dependency: <linux/kvm_types.h>

// Original C conditional: #ifdef CONFIG_KVM_GUEST_MEMFD
#[cfg(CONFIG_KVM_GUEST_MEMFD)]
unsafe extern "C" {
    pub fn kvm_gmem_init(module: *mut r#module) -> ::core::ffi::c_int;
    pub fn kvm_gmem_exit();
    pub fn kvm_gmem_create(
        kvm: *mut kvm,
        args: *mut kvm_create_guest_memfd,
    ) -> ::core::ffi::c_int;
    pub fn kvm_gmem_bind(
        kvm: *mut kvm,
        slot: *mut kvm_memory_slot,
        fd: ::core::ffi::c_uint,
        offset: uoff_t,
    ) -> ::core::ffi::c_int;
    pub fn kvm_gmem_unbind(slot: *mut kvm_memory_slot);
}

#[cfg(not(CONFIG_KVM_GUEST_MEMFD))]
pub unsafe fn kvm_gmem_init(module: *mut r#module) -> ::core::ffi::c_int {
    let _ = module;
    0
}

#[cfg(not(CONFIG_KVM_GUEST_MEMFD))]
pub unsafe fn kvm_gmem_exit() {}

#[cfg(not(CONFIG_KVM_GUEST_MEMFD))]
pub unsafe fn kvm_gmem_bind(
    kvm: *mut kvm,
    slot: *mut kvm_memory_slot,
    fd: ::core::ffi::c_uint,
    offset: uoff_t,
) -> ::core::ffi::c_int {
    let _ = kvm;
    let _ = slot;
    let _ = fd;
    let _ = offset;

    unsafe {
        WARN_ON_ONCE(1);
    }
    -EIO
}

#[cfg(not(CONFIG_KVM_GUEST_MEMFD))]
pub unsafe fn kvm_gmem_unbind(slot: *mut kvm_memory_slot) {
    let _ = slot;

    unsafe {
        WARN_ON_ONCE(1);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
