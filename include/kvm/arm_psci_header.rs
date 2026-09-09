/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012,2013 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Dependency intent preserved from <linux/kvm_host.h> and <uapi/linux/psci.h>.

pub const KVM_ARM_PSCI_0_1: i32 = 0x0000_0001;
pub const KVM_ARM_PSCI_0_2: i32 = 0x0000_0002;
pub const KVM_ARM_PSCI_1_0: i32 = 0x0001_0000;
pub const KVM_ARM_PSCI_1_1: i32 = 0x0001_0001;
pub const KVM_ARM_PSCI_1_2: i32 = 0x0001_0002;
pub const KVM_ARM_PSCI_1_3: i32 = 0x0001_0003;

pub const KVM_ARM_PSCI_LATEST: i32 = KVM_ARM_PSCI_1_3;

pub unsafe fn kvm_psci_version(vcpu: &mut kvm_vcpu) -> i32 {
    /*
     * Our PSCI implementation stays the same across versions from
     * v0.2 onward, only adding the few mandatory functions (such
     * as FEATURES with 1.0) that are required by newer
     * revisions. It is thus safe to return the latest, unless
     * userspace has instructed us otherwise.
     */
    if vcpu_has_feature(vcpu, KVM_ARM_VCPU_PSCI_0_2) {
        if vcpu.kvm.arch.psci_version != 0 {
            return vcpu.kvm.arch.psci_version;
        }

        return KVM_ARM_PSCI_LATEST;
    }

    KVM_ARM_PSCI_0_1
}

/* Narrow the PSCI register arguments (r1 to r3) to 32 bits. */
pub unsafe fn kvm_psci_narrow_to_32bit(vcpu: &mut kvm_vcpu) {
    let mut i: i32;

    /*
     * Zero the input registers' upper 32 bits. They will be fully
     * zeroed on exit, so we're fine changing them in place.
     */
    i = 1;
    while i < 4 {
        vcpu_set_reg(vcpu, i, lower_32_bits(vcpu_get_reg(vcpu, i)));
        i += 1;
    }
}

pub unsafe fn kvm_psci_valid_affinity(
    vcpu: &mut kvm_vcpu,
    affinity: libc::c_ulong,
) -> bool {
    let _ = vcpu;
    (affinity & !MPIDR_HWID_BITMASK) == 0
}

pub unsafe fn kvm_psci_affinity_mask(affinity_level: libc::c_ulong) -> libc::c_ulong {
    if affinity_level <= 3 {
        return MPIDR_HWID_BITMASK
            & !((0x1 as libc::c_ulong)
                .wrapping_shl((affinity_level * MPIDR_LEVEL_BITS) as u32)
                .wrapping_sub(1));
    }

    0
}

extern "C" {
    pub fn kvm_psci_call(vcpu: &mut kvm_vcpu) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
