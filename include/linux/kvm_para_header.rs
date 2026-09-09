/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <uapi/linux/kvm_para.h>

use core::ffi::{c_uint, c_ulong};

extern "C" {
    pub fn kvm_arch_para_features() -> c_ulong;
    pub fn kvm_arch_para_hints() -> c_ulong;
}

#[inline]
pub unsafe fn kvm_para_has_feature(feature: c_uint) -> bool {
    (kvm_arch_para_features() & (1 as c_ulong).wrapping_shl(feature)) != 0
}

#[inline]
pub unsafe fn kvm_para_has_hint(feature: c_uint) -> bool {
    (kvm_arch_para_hints() & (1 as c_ulong).wrapping_shl(feature)) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
