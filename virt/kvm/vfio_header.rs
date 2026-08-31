// SPDX-License-Identifier: GPL-2.0

// C header dependency/conditional intent:
// #ifdef CONFIG_KVM_VFIO

#[cfg(CONFIG_KVM_VFIO)]
extern "C" {
    pub fn kvm_vfio_ops_init() -> ::std::os::raw::c_int;
    pub fn kvm_vfio_ops_exit();
}

#[cfg(not(CONFIG_KVM_VFIO))]
#[inline]
pub fn kvm_vfio_ops_init() -> ::std::os::raw::c_int {
    0
}

#[cfg(not(CONFIG_KVM_VFIO))]
#[inline]
pub fn kvm_vfio_ops_exit() {}
