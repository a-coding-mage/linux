/* SPDX-License-Identifier: GPL-2.0 */

/*
 * KVM coalesced MMIO
 *
 * Copyright (c) 2008 Bull S.A.S.
 *
 *  Author: Laurent Vivier <Laurent.Vivier@bull.net>
 *
 */

/* C header dependencies:
 * - <linux/list.h>
 * - struct kvm_io_device
 * - struct kvm
 * - struct kvm_coalesced_mmio_zone
 */

#[cfg(CONFIG_KVM_MMIO)]
#[repr(C)]
pub struct kvm_coalesced_mmio_dev {
    pub list: list_head,
    pub dev: kvm_io_device,
    pub kvm: *mut kvm,
    pub zone: kvm_coalesced_mmio_zone,
}

#[cfg(CONFIG_KVM_MMIO)]
extern "C" {
    pub fn kvm_coalesced_mmio_init(kvm: *mut kvm) -> ::std::os::raw::c_int;
    pub fn kvm_coalesced_mmio_free(kvm: *mut kvm);
    pub fn kvm_vm_ioctl_register_coalesced_mmio(
        kvm: *mut kvm,
        zone: *mut kvm_coalesced_mmio_zone,
    ) -> ::std::os::raw::c_int;
    pub fn kvm_vm_ioctl_unregister_coalesced_mmio(
        kvm: *mut kvm,
        zone: *mut kvm_coalesced_mmio_zone,
    ) -> ::std::os::raw::c_int;
}

#[cfg(not(CONFIG_KVM_MMIO))]
#[inline]
pub unsafe fn kvm_coalesced_mmio_init(kvm: *mut kvm) -> ::std::os::raw::c_int {
    let _ = kvm;
    0
}

#[cfg(not(CONFIG_KVM_MMIO))]
#[inline]
pub unsafe fn kvm_coalesced_mmio_free(kvm: *mut kvm) {
    let _ = kvm;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
