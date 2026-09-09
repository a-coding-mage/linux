/* SPDX-License-Identifier: GPL-2.0 */

// Dependency corresponding to <uapi/asm-generic/kvm_para.h> is supplied externally.

/*
 * This function is used by architectures that support kvm to avoid issuing
 * false soft lockup messages.
 */
#[inline]
fn kvm_check_and_clear_guest_paused() -> bool {
    false
}

#[inline]
fn kvm_arch_para_features() -> u32 {
    0
}

#[inline]
fn kvm_arch_para_hints() -> u32 {
    0
}

#[inline]
fn kvm_para_available() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
