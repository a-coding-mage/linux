/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 IBM Corporation
 */

/*
 * Corresponds to:
 * #if defined(CONFIG_PPC_PSERIES) || defined(CONFIG_KVM_GUEST)
 * The build-time configuration is preserved with Rust cfg predicates.
 */

#[cfg(any(CONFIG_PPC_PSERIES, CONFIG_KVM_GUEST))]
extern "C" {
    /* DECLARE_STATIC_KEY_FALSE(kvm_guest); */
    static kvm_guest: StaticKeyFalse;
    fn static_branch_unlikely(key: *const StaticKeyFalse) -> bool;
    fn check_kvm_guest() -> core::ffi::c_int;
}

#[cfg(any(CONFIG_PPC_PSERIES, CONFIG_KVM_GUEST))]
#[inline]
pub unsafe fn is_kvm_guest() -> bool {
    static_branch_unlikely(&kvm_guest as *const StaticKeyFalse)
}

#[cfg(not(any(CONFIG_PPC_PSERIES, CONFIG_KVM_GUEST)))]
#[inline]
pub fn is_kvm_guest() -> bool {
    false
}

#[cfg(not(any(CONFIG_PPC_PSERIES, CONFIG_KVM_GUEST)))]
#[inline]
pub fn check_kvm_guest() -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
