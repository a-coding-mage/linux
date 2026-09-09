/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Xen/Linux headers:
// asm/xen/hypervisor.h and linux/jump_label.h

extern "C" {
    pub fn kvm_init_hyp_services();
    pub fn kvm_arm_hyp_service_available(func_id: u32) -> bool;
    pub fn kvm_arm_target_impl_cpu_init();
}

// DECLARE_STATIC_KEY_FALSE(pkvm_guest);
extern "C" {
    pub static pkvm_guest: StaticKey;
}

// External type and helper supplied by linux/jump_label.h.
pub type StaticKey = core::ffi::c_void;
extern "C" {
    pub fn static_branch_unlikely(key: *const StaticKey) -> bool;
}

// CONFIG_ARM_PKVM_GUEST is a build-time configuration condition.
#[cfg(feature = "CONFIG_ARM_PKVM_GUEST")]
extern "C" {
    pub fn pkvm_init_hyp_services();
}

#[cfg(feature = "CONFIG_ARM_PKVM_GUEST")]
#[inline]
pub unsafe fn is_protected_kvm_guest() -> bool {
    static_branch_unlikely(&pkvm_guest)
}

#[cfg(not(feature = "CONFIG_ARM_PKVM_GUEST"))]
#[inline]
pub unsafe fn pkvm_init_hyp_services() {}

#[cfg(not(feature = "CONFIG_ARM_PKVM_GUEST"))]
#[inline]
pub unsafe fn is_protected_kvm_guest() -> bool {
    false
}

#[inline]
pub unsafe fn kvm_arch_init_hyp_services() {
    pkvm_init_hyp_services();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
