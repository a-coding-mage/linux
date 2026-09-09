/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the C UAPI header. Linux integer types are represented by
// their fixed-width Rust equivalents.

pub const KVM_CPUID_SIGNATURE: u32 = 0x40000000;
pub const KVM_SIGNATURE: &[u8; 12] = b"KVMKVMKVM\0\0\0";

pub const KVM_CPUID_FEATURES: u32 = 0x40000001;
pub const KVM_FEATURE_CLOCKSOURCE: u32 = 0;
pub const KVM_FEATURE_NOP_IO_DELAY: u32 = 1;
pub const KVM_FEATURE_MMU_OP: u32 = 2;
pub const KVM_FEATURE_CLOCKSOURCE2: u32 = 3;
pub const KVM_FEATURE_ASYNC_PF: u32 = 4;
pub const KVM_FEATURE_STEAL_TIME: u32 = 5;
pub const KVM_FEATURE_PV_EOI: u32 = 6;
pub const KVM_FEATURE_PV_UNHALT: u32 = 7;
pub const KVM_FEATURE_PV_TLB_FLUSH: u32 = 9;
pub const KVM_FEATURE_ASYNC_PF_VMEXIT: u32 = 10;
pub const KVM_FEATURE_PV_SEND_IPI: u32 = 11;
pub const KVM_FEATURE_POLL_CONTROL: u32 = 12;
pub const KVM_FEATURE_PV_SCHED_YIELD: u32 = 13;
pub const KVM_FEATURE_ASYNC_PF_INT: u32 = 14;
pub const KVM_FEATURE_MSI_EXT_DEST_ID: u32 = 15;
pub const KVM_FEATURE_HC_MAP_GPA_RANGE: u32 = 16;
pub const KVM_FEATURE_MIGRATION_CONTROL: u32 = 17;

pub const KVM_HINTS_REALTIME: u32 = 0;
pub const KVM_FEATURE_CLOCKSOURCE_STABLE_BIT: u32 = 24;

pub const MSR_KVM_WALL_CLOCK: u32 = 0x11;
pub const MSR_KVM_SYSTEM_TIME: u32 = 0x12;
pub const KVM_MSR_ENABLED: u32 = 1;
pub const MSR_KVM_WALL_CLOCK_NEW: u32 = 0x4b564d00;
pub const MSR_KVM_SYSTEM_TIME_NEW: u32 = 0x4b564d01;
pub const MSR_KVM_ASYNC_PF_EN: u32 = 0x4b564d02;
pub const MSR_KVM_STEAL_TIME: u32 = 0x4b564d03;
pub const MSR_KVM_PV_EOI_EN: u32 = 0x4b564d04;
pub const MSR_KVM_POLL_CONTROL: u32 = 0x4b564d05;
pub const MSR_KVM_ASYNC_PF_INT: u32 = 0x4b564d06;
pub const MSR_KVM_ASYNC_PF_ACK: u32 = 0x4b564d07;
pub const MSR_KVM_MIGRATION_CONTROL: u32 = 0x4b564d08;

#[repr(C)]
pub struct kvm_steal_time {
    pub steal: u64,
    pub version: u32,
    pub flags: u32,
    pub preempted: u8,
    pub u8_pad: [u8; 3],
    pub pad: [u32; 11],
}

pub const KVM_VCPU_PREEMPTED: u32 = 1 << 0;
pub const KVM_VCPU_FLUSH_TLB: u32 = 1 << 1;
pub const KVM_CLOCK_PAIRING_WALLCLOCK: u32 = 0;

#[repr(C)]
pub struct kvm_clock_pairing {
    pub sec: i64,
    pub nsec: i64,
    pub tsc: u64,
    pub flags: u32,
    pub pad: [u32; 9],
}

pub const KVM_STEAL_ALIGNMENT_BITS: u32 = 5;
pub const KVM_STEAL_VALID_BITS: u64 = !0u64 << (KVM_STEAL_ALIGNMENT_BITS + 1);
pub const KVM_STEAL_RESERVED_MASK: u32 = ((1 << KVM_STEAL_ALIGNMENT_BITS) - 1) << 1;
pub const KVM_MAX_MMU_OP_BATCH: u32 = 32;

pub const KVM_ASYNC_PF_ENABLED: u32 = 1 << 0;
pub const KVM_ASYNC_PF_SEND_ALWAYS: u32 = 1 << 1;
pub const KVM_ASYNC_PF_DELIVERY_AS_PF_VMEXIT: u32 = 1 << 2;
pub const KVM_ASYNC_PF_DELIVERY_AS_INT: u32 = 1 << 3;
pub const KVM_ASYNC_PF_VEC_MASK: u32 = 0xff;
pub const KVM_MIGRATION_READY: u32 = 1 << 0;

pub const KVM_MAP_GPA_RANGE_PAGE_SZ_4K: u32 = 0;
pub const KVM_MAP_GPA_RANGE_PAGE_SZ_2M: u32 = 1 << 0;
pub const KVM_MAP_GPA_RANGE_PAGE_SZ_1G: u32 = 1 << 1;
pub const KVM_MAP_GPA_RANGE_ENC_STAT: fn(u32) -> u32 = |n| n << 4;
pub const KVM_MAP_GPA_RANGE_ENCRYPTED: u32 = 1 << 4;
pub const KVM_MAP_GPA_RANGE_DECRYPTED: u32 = 0;

pub const KVM_MMU_OP_WRITE_PTE: u32 = 1;
pub const KVM_MMU_OP_FLUSH_TLB: u32 = 2;
pub const KVM_MMU_OP_RELEASE_PT: u32 = 3;

#[repr(C)]
pub struct kvm_mmu_op_header {
    pub op: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct kvm_mmu_op_write_pte {
    pub header: kvm_mmu_op_header,
    pub pte_phys: u64,
    pub pte_val: u64,
}

#[repr(C)]
pub struct kvm_mmu_op_flush_tlb {
    pub header: kvm_mmu_op_header,
}

#[repr(C)]
pub struct kvm_mmu_op_release_pt {
    pub header: kvm_mmu_op_header,
    pub pt_phys: u64,
}

pub const KVM_PV_REASON_PAGE_NOT_PRESENT: u32 = 1;
pub const KVM_PV_REASON_PAGE_READY: u32 = 2;

#[repr(C)]
pub struct kvm_vcpu_pv_apf_data {
    pub flags: u32,
    pub token: u32,
    pub pad: [u8; 56],
}

pub const KVM_PV_EOI_BIT: u32 = 0;
pub const KVM_PV_EOI_MASK: u32 = 0x1 << KVM_PV_EOI_BIT;
pub const KVM_PV_EOI_ENABLED: u32 = KVM_PV_EOI_MASK;
pub const KVM_PV_EOI_DISABLED: u32 = 0x0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
