/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright IBM Corp. 2008
 *
 * Authors: Hollis Blanchard <hollisb@us.ibm.com>
 */

// Dependency supplied by asm/kvm_guest.h.
unsafe extern "C" {
    fn is_kvm_guest() -> bool;
    fn epapr_hypercall0_1(token: ::core::ffi::c_ulong, r: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int;
}

// KVM_HCALL_TOKEN and KVM_HC_FEATURES are supplied by uapi/asm/kvm_para.h.

#[inline]
pub unsafe fn kvm_para_available() -> ::core::ffi::c_int {
    (cfg!(feature = "CONFIG_KVM_GUEST") && unsafe { is_kvm_guest() }) as ::core::ffi::c_int
}

#[inline]
pub unsafe fn kvm_arch_para_features() -> ::core::ffi::c_uint {
    let mut r: ::core::ffi::c_ulong = 0;

    if unsafe { kvm_para_available() } == 0 {
        return 0;
    }

    if unsafe { epapr_hypercall0_1(KVM_HCALL_TOKEN(KVM_HC_FEATURES), &mut r) } != 0 {
        return 0;
    }

    r as ::core::ffi::c_uint
}

#[inline]
pub unsafe fn kvm_arch_para_hints() -> ::core::ffi::c_uint {
    0
}

#[inline]
pub unsafe fn kvm_check_and_clear_guest_paused() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
