/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright SUSE Linux Products GmbH 2010
 *
 * Authors: Alexander Graf <agraf@suse.de>
 */

// Translated from the C header __ASM_KVM_BOOK3S_32_H__.

#[inline]
pub unsafe fn svcpu_get(
    vcpu: *mut kvm_vcpu,
) -> *mut kvmppc_book3s_shadow_vcpu {
    (*vcpu).arch.shadow_vcpu
}

#[inline]
pub unsafe fn svcpu_put(_svcpu: *mut kvmppc_book3s_shadow_vcpu) {
}

pub const PTE_SIZE: u32 = 12;
pub const VSID_ALL: u32 = 0;
pub const SR_INVALID: u32 = 0x00000001; /* VSID 1 should always be unused */
pub const SR_KP: u32 = 0x20000000;
pub const PTE_V: u32 = 0x80000000;
pub const PTE_SEC: u32 = 0x00000040;
pub const PTE_M: u32 = 0x00000010;
pub const PTE_R: u32 = 0x00000100;
pub const PTE_C: u32 = 0x00000080;

pub const SID_SHIFT: u32 = 28;
pub const ESID_MASK: u32 = 0xf0000000;
pub const VSID_MASK: u64 = 0x00fffffff0000000;
pub const VPN_SHIFT: u32 = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
