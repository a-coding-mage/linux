/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2012,2013 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 *
 * Derived from arch/arm/include/uapi/asm/kvm.h:
 * Copyright (C) 2012 - Virtual Open Systems and Columbia University
 * Author: Christoffer Dall <c.dall@virtualopensystems.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

/* Dependencies from C includes: linux/psci.h, linux/types.h, asm/ptrace.h, asm/sve_context.h. */

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

pub const KVM_SPSR_EL1: u32 = 0;
pub const KVM_SPSR_SVC: u32 = KVM_SPSR_EL1;
pub const KVM_SPSR_ABT: u32 = 1;
pub const KVM_SPSR_UND: u32 = 2;
pub const KVM_SPSR_IRQ: u32 = 3;
pub const KVM_SPSR_FIQ: u32 = 4;
pub const KVM_NR_SPSR: usize = 5;

pub const __KVM_HAVE_IRQ_LINE: bool = true;
pub const __KVM_HAVE_VCPU_EVENTS: bool = true;

pub const KVM_COALESCED_MMIO_PAGE_OFFSET: u32 = 1;
pub const KVM_DIRTY_LOG_PAGE_OFFSET: u32 = 64;

#[repr(C)]
pub struct kvm_regs {
    pub regs: user_pt_regs, /* sp = sp_el0 */
    pub sp_el1: __u64,
    pub elr_el1: __u64,
    pub spsr: [__u64; KVM_NR_SPSR],
    pub fp_regs: user_fpsimd_state,
}

/*
 * Supported CPU Targets - Adding a new target type is not recommended,
 * unless there are some special registers not supported by the
 * genericv8 syreg table.
 */
pub const KVM_ARM_TARGET_AEM_V8: u32 = 0;
pub const KVM_ARM_TARGET_FOUNDATION_V8: u32 = 1;
pub const KVM_ARM_TARGET_CORTEX_A57: u32 = 2;
pub const KVM_ARM_TARGET_XGENE_POTENZA: u32 = 3;
pub const KVM_ARM_TARGET_CORTEX_A53: u32 = 4;
/* Generic ARM v8 target */
pub const KVM_ARM_TARGET_GENERIC_V8: u32 = 5;

pub const KVM_ARM_NUM_TARGETS: u32 = 6;

/* KVM_ARM_SET_DEVICE_ADDR ioctl id encoding */
pub const KVM_ARM_DEVICE_TYPE_SHIFT: u32 = 0;
pub const KVM_ARM_DEVICE_TYPE_MASK: __u64 =
    __GENMASK(KVM_ARM_DEVICE_TYPE_SHIFT + 15, KVM_ARM_DEVICE_TYPE_SHIFT);
pub const KVM_ARM_DEVICE_ID_SHIFT: u32 = 16;
pub const KVM_ARM_DEVICE_ID_MASK: __u64 =
    __GENMASK(KVM_ARM_DEVICE_ID_SHIFT + 15, KVM_ARM_DEVICE_ID_SHIFT);

/* Supported device IDs */
pub const KVM_ARM_DEVICE_VGIC_V2: u32 = 0;

/* Supported VGIC address types  */
pub const KVM_VGIC_V2_ADDR_TYPE_DIST: u32 = 0;
pub const KVM_VGIC_V2_ADDR_TYPE_CPU: u32 = 1;

pub const KVM_VGIC_V2_DIST_SIZE: u32 = 0x1000;
pub const KVM_VGIC_V2_CPU_SIZE: u32 = 0x2000;

/* Supported VGICv3 address types  */
pub const KVM_VGIC_V3_ADDR_TYPE_DIST: u32 = 2;
pub const KVM_VGIC_V3_ADDR_TYPE_REDIST: u32 = 3;
pub const KVM_VGIC_ITS_ADDR_TYPE: u32 = 4;
pub const KVM_VGIC_V3_ADDR_TYPE_REDIST_REGION: u32 = 5;

pub const KVM_VGIC_V3_DIST_SIZE: __u64 = SZ_64K;
pub const KVM_VGIC_V3_REDIST_SIZE: __u64 = 2 * SZ_64K;
pub const KVM_VGIC_V3_ITS_SIZE: __u64 = 2 * SZ_64K;

pub const KVM_ARM_VCPU_POWER_OFF: u32 = 0; /* CPU is started in OFF state */
pub const KVM_ARM_VCPU_EL1_32BIT: u32 = 1; /* CPU running a 32bit VM */
pub const KVM_ARM_VCPU_PSCI_0_2: u32 = 2; /* CPU uses PSCI v0.2 */
pub const KVM_ARM_VCPU_PMU_V3: u32 = 3; /* Support guest PMUv3 */
pub const KVM_ARM_VCPU_SVE: u32 = 4; /* enable SVE for this CPU */
pub const KVM_ARM_VCPU_PTRAUTH_ADDRESS: u32 = 5; /* VCPU uses address authentication */
pub const KVM_ARM_VCPU_PTRAUTH_GENERIC: u32 = 6; /* VCPU uses generic authentication */
pub const KVM_ARM_VCPU_HAS_EL2: u32 = 7; /* Support nested virtualization */
pub const KVM_ARM_VCPU_HAS_EL2_E2H0: u32 = 8; /* Limit NV support to E2H RES0 */

#[repr(C)]
pub struct kvm_vcpu_init {
    pub target: __u32,
    pub features: [__u32; 7],
}

#[repr(C)]
pub struct kvm_sregs {}

#[repr(C)]
pub struct kvm_fpu {}

/*
 * See v8 ARM ARM D7.3: Debug Registers
 *
 * The architectural limit is 16 debug registers of each type although
 * in practice there are usually less (see ID_AA64DFR0_EL1).
 *
 * Although the control registers are architecturally defined as 32
 * bits wide we use a 64 bit structure here to keep parity with
 * KVM_GET/SET_ONE_REG behaviour which treats all system registers as
 * 64 bit values. It also allows for the possibility of the
 * architecture expanding the control registers without having to
 * change the userspace ABI.
 */
pub const KVM_ARM_MAX_DBG_REGS: usize = 16;
#[repr(C)]
pub struct kvm_guest_debug_arch {
    pub dbg_bcr: [__u64; KVM_ARM_MAX_DBG_REGS],
    pub dbg_bvr: [__u64; KVM_ARM_MAX_DBG_REGS],
    pub dbg_wcr: [__u64; KVM_ARM_MAX_DBG_REGS],
    pub dbg_wvr: [__u64; KVM_ARM_MAX_DBG_REGS],
}

pub const KVM_DEBUG_ARCH_HSR_HIGH_VALID: u32 = 1 << 0;
#[repr(C)]
pub struct kvm_debug_exit_arch {
    pub hsr: __u32,
    pub hsr_high: __u32, /* ESR_EL2[61:32] */
    pub far: __u64,      /* used for watchpoints */
}

/*
 * Architecture specific defines for kvm_guest_debug->control
 */

pub const KVM_GUESTDBG_USE_SW_BP: u32 = 1 << 16;
pub const KVM_GUESTDBG_USE_HW: u32 = 1 << 17;

#[repr(C)]
pub struct kvm_sync_regs {
    /* Used with KVM_CAP_ARM_USER_IRQ */
    pub device_irq_level: __u64,
}

/* Bits for run->s.regs.device_irq_level */
pub const KVM_ARM_DEV_EL1_VTIMER: u32 = 1 << 0;
pub const KVM_ARM_DEV_EL1_PTIMER: u32 = 1 << 1;
pub const KVM_ARM_DEV_PMU: u32 = 1 << 2;

/*
 * PMU filter structure. Describe a range of events with a particular
 * action. To be used with KVM_ARM_VCPU_PMU_V3_FILTER.
 */
#[repr(C)]
pub struct kvm_pmu_event_filter {
    pub base_event: __u16,
    pub nevents: __u16,
    pub action: __u8,
    pub pad: [__u8; 3],
}

pub const KVM_PMU_EVENT_ALLOW: u32 = 0;
pub const KVM_PMU_EVENT_DENY: u32 = 1;

/* for KVM_GET/SET_VCPU_EVENTS */
#[repr(C)]
pub struct kvm_vcpu_events_exception {
    pub serror_pending: __u8,
    pub serror_has_esr: __u8,
    pub ext_dabt_pending: __u8,
    /* Align it to 8 bytes */
    pub pad: [__u8; 5],
    pub serror_esr: __u64,
}

#[repr(C)]
pub struct kvm_vcpu_events {
    pub exception: kvm_vcpu_events_exception,
    pub reserved: [__u32; 12],
}

#[repr(C)]
pub struct kvm_arm_copy_mte_tags {
    pub guest_ipa: __u64,
    pub length: __u64,
    pub addr: *mut core::ffi::c_void,
    pub flags: __u64,
    pub reserved: [__u64; 2],
}

/*
 * Counter/Timer offset structure. Describe the virtual/physical offset.
 * To be used with KVM_ARM_SET_COUNTER_OFFSET.
 */
#[repr(C)]
pub struct kvm_arm_counter_offset {
    pub counter_offset: __u64,
    pub reserved: __u64,
}

pub const KVM_ARM_TAGS_TO_GUEST: u32 = 0;
pub const KVM_ARM_TAGS_FROM_GUEST: u32 = 1;

/* If you need to interpret the index values, here is the key: */
pub const KVM_REG_ARM_COPROC_MASK: __u64 = 0x000000000FFF0000;
pub const KVM_REG_ARM_COPROC_SHIFT: u32 = 16;

/* Normal registers are mapped as coprocessor 16. */
pub const KVM_REG_ARM_CORE: __u64 = 0x0010 << KVM_REG_ARM_COPROC_SHIFT;
#[macro_export]
macro_rules! KVM_REG_ARM_CORE_REG {
    ($name:tt) => {
        core::mem::offset_of!(kvm_regs, $name) / core::mem::size_of::<__u32>()
    };
}

/* Some registers need more space to represent values. */
pub const KVM_REG_ARM_DEMUX: __u64 = 0x0011 << KVM_REG_ARM_COPROC_SHIFT;
pub const KVM_REG_ARM_DEMUX_ID_MASK: __u64 = 0x000000000000FF00;
pub const KVM_REG_ARM_DEMUX_ID_SHIFT: u32 = 8;
pub const KVM_REG_ARM_DEMUX_ID_CCSIDR: __u64 = 0x00 << KVM_REG_ARM_DEMUX_ID_SHIFT;
pub const KVM_REG_ARM_DEMUX_VAL_MASK: __u64 = 0x00000000000000FF;
pub const KVM_REG_ARM_DEMUX_VAL_SHIFT: u32 = 0;

/* AArch64 system registers */
pub const KVM_REG_ARM64_SYSREG: __u64 = 0x0013 << KVM_REG_ARM_COPROC_SHIFT;
pub const KVM_REG_ARM64_SYSREG_OP0_MASK: __u64 = 0x000000000000c000;
pub const KVM_REG_ARM64_SYSREG_OP0_SHIFT: u32 = 14;
pub const KVM_REG_ARM64_SYSREG_OP1_MASK: __u64 = 0x0000000000003800;
pub const KVM_REG_ARM64_SYSREG_OP1_SHIFT: u32 = 11;
pub const KVM_REG_ARM64_SYSREG_CRN_MASK: __u64 = 0x0000000000000780;
pub const KVM_REG_ARM64_SYSREG_CRN_SHIFT: u32 = 7;
pub const KVM_REG_ARM64_SYSREG_CRM_MASK: __u64 = 0x0000000000000078;
pub const KVM_REG_ARM64_SYSREG_CRM_SHIFT: u32 = 3;
pub const KVM_REG_ARM64_SYSREG_OP2_MASK: __u64 = 0x0000000000000007;
pub const KVM_REG_ARM64_SYSREG_OP2_SHIFT: u32 = 0;

pub const fn ARM64_SYS_REG_SHIFT_MASK(x: __u64, shift: u32, mask: __u64) -> __u64 {
    (x << shift) & mask
}

pub const fn __ARM64_SYS_REG(op0: __u64, op1: __u64, crn: __u64, crm: __u64, op2: __u64) -> __u64 {
    KVM_REG_ARM64
        | KVM_REG_ARM64_SYSREG
        | ARM64_SYS_REG_SHIFT_MASK(op0, KVM_REG_ARM64_SYSREG_OP0_SHIFT, KVM_REG_ARM64_SYSREG_OP0_MASK)
        | ARM64_SYS_REG_SHIFT_MASK(op1, KVM_REG_ARM64_SYSREG_OP1_SHIFT, KVM_REG_ARM64_SYSREG_OP1_MASK)
        | ARM64_SYS_REG_SHIFT_MASK(crn, KVM_REG_ARM64_SYSREG_CRN_SHIFT, KVM_REG_ARM64_SYSREG_CRN_MASK)
        | ARM64_SYS_REG_SHIFT_MASK(crm, KVM_REG_ARM64_SYSREG_CRM_SHIFT, KVM_REG_ARM64_SYSREG_CRM_MASK)
        | ARM64_SYS_REG_SHIFT_MASK(op2, KVM_REG_ARM64_SYSREG_OP2_SHIFT, KVM_REG_ARM64_SYSREG_OP2_MASK)
}

pub const fn ARM64_SYS_REG(op0: __u64, op1: __u64, crn: __u64, crm: __u64, op2: __u64) -> __u64 {
    __ARM64_SYS_REG(op0, op1, crn, crm, op2) | KVM_REG_SIZE_U64
}

/* Physical Timer EL0 Registers */
pub const KVM_REG_ARM_PTIMER_CTL: __u64 = ARM64_SYS_REG(3, 3, 14, 2, 1);
pub const KVM_REG_ARM_PTIMER_CVAL: __u64 = ARM64_SYS_REG(3, 3, 14, 2, 2);
pub const KVM_REG_ARM_PTIMER_CNT: __u64 = ARM64_SYS_REG(3, 3, 14, 0, 1);

/*
 * EL0 Virtual Timer Registers
 *
 * WARNING:
 *      KVM_REG_ARM_TIMER_CVAL and KVM_REG_ARM_TIMER_CNT are not defined
 *      with the appropriate register encodings.  Their values have been
 *      accidentally swapped.  As this is set API, the definitions here
 *      must be used, rather than ones derived from the encodings.
 */
pub const KVM_REG_ARM_TIMER_CTL: __u64 = ARM64_SYS_REG(3, 3, 14, 3, 1);
pub const KVM_REG_ARM_TIMER_CVAL: __u64 = ARM64_SYS_REG(3, 3, 14, 0, 2);
pub const KVM_REG_ARM_TIMER_CNT: __u64 = ARM64_SYS_REG(3, 3, 14, 3, 2);

/* KVM-as-firmware specific pseudo-registers */
pub const KVM_REG_ARM_FW: __u64 = 0x0014 << KVM_REG_ARM_COPROC_SHIFT;
pub const fn KVM_REG_ARM_FW_REG(r: __u64) -> __u64 {
    KVM_REG_ARM64 | KVM_REG_SIZE_U64 | KVM_REG_ARM_FW | (r & 0xffff)
}
pub const KVM_REG_ARM_PSCI_VERSION: __u64 = KVM_REG_ARM_FW_REG(0);
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1: __u64 = KVM_REG_ARM_FW_REG(1);
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_NOT_AVAIL: u32 = 0;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_AVAIL: u32 = 1;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_NOT_REQUIRED: u32 = 2;

/*
 * Only two states can be presented by the host kernel:
 * - NOT_REQUIRED: the guest doesn't need to do anything
 * - NOT_AVAIL: the guest isn't mitigated (it can still use SSBS if available)
 *
 * All the other values are deprecated. The host still accepts all
 * values (they are ABI), but will narrow them to the above two.
 */
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2: __u64 = KVM_REG_ARM_FW_REG(2);
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_AVAIL: u32 = 0;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_UNKNOWN: u32 = 1;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_AVAIL: u32 = 2;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_REQUIRED: u32 = 3;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_ENABLED: u32 = 1u32 << 4;

pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3: __u64 = KVM_REG_ARM_FW_REG(3);
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3_NOT_AVAIL: u32 = 0;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3_AVAIL: u32 = 1;
pub const KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_3_NOT_REQUIRED: u32 = 2;

/* SVE registers */
pub const KVM_REG_ARM64_SVE: __u64 = 0x15 << KVM_REG_ARM_COPROC_SHIFT;

/* Z- and P-regs occupy blocks at the following offsets within this range: */
pub const KVM_REG_ARM64_SVE_ZREG_BASE: __u64 = 0;
pub const KVM_REG_ARM64_SVE_PREG_BASE: __u64 = 0x400;
pub const KVM_REG_ARM64_SVE_FFR_BASE: __u64 = 0x600;

pub const KVM_ARM64_SVE_NUM_ZREGS: __u64 = __SVE_NUM_ZREGS;
pub const KVM_ARM64_SVE_NUM_PREGS: __u64 = __SVE_NUM_PREGS;

pub const KVM_ARM64_SVE_MAX_SLICES: __u64 = 32;

pub const fn KVM_REG_ARM64_SVE_ZREG(n: __u64, i: __u64) -> __u64 {
    KVM_REG_ARM64
        | KVM_REG_ARM64_SVE
        | KVM_REG_ARM64_SVE_ZREG_BASE
        | KVM_REG_SIZE_U2048
        | ((n & (KVM_ARM64_SVE_NUM_ZREGS - 1)) << 5)
        | (i & (KVM_ARM64_SVE_MAX_SLICES - 1))
}

pub const fn KVM_REG_ARM64_SVE_PREG(n: __u64, i: __u64) -> __u64 {
    KVM_REG_ARM64
        | KVM_REG_ARM64_SVE
        | KVM_REG_ARM64_SVE_PREG_BASE
        | KVM_REG_SIZE_U256
        | ((n & (KVM_ARM64_SVE_NUM_PREGS - 1)) << 5)
        | (i & (KVM_ARM64_SVE_MAX_SLICES - 1))
}

pub const fn KVM_REG_ARM64_SVE_FFR(i: __u64) -> __u64 {
    KVM_REG_ARM64
        | KVM_REG_ARM64_SVE
        | KVM_REG_ARM64_SVE_FFR_BASE
        | KVM_REG_SIZE_U256
        | (i & (KVM_ARM64_SVE_MAX_SLICES - 1))
}

/*
 * Register values for KVM_REG_ARM64_SVE_ZREG(), KVM_REG_ARM64_SVE_PREG() and
 * KVM_REG_ARM64_SVE_FFR() are represented in memory in an endianness-
 * invariant layout which differs from the layout used for the FPSIMD
 * V-registers on big-endian systems: see sigcontext.h for more explanation.
 */

pub const KVM_ARM64_SVE_VQ_MIN: __u64 = __SVE_VQ_MIN;
pub const KVM_ARM64_SVE_VQ_MAX: __u64 = __SVE_VQ_MAX;

/* Vector lengths pseudo-register: */
pub const KVM_REG_ARM64_SVE_VLS: __u64 =
    KVM_REG_ARM64 | KVM_REG_ARM64_SVE | KVM_REG_SIZE_U512 | 0xffff;
pub const KVM_ARM64_SVE_VLS_WORDS: __u64 =
    (KVM_ARM64_SVE_VQ_MAX - KVM_ARM64_SVE_VQ_MIN) / 64 + 1;

/* Bitmap feature firmware registers */
pub const KVM_REG_ARM_FW_FEAT_BMAP: __u64 = 0x0016 << KVM_REG_ARM_COPROC_SHIFT;
pub const fn KVM_REG_ARM_FW_FEAT_BMAP_REG(r: __u64) -> __u64 {
    KVM_REG_ARM64 | KVM_REG_SIZE_U64 | KVM_REG_ARM_FW_FEAT_BMAP | (r & 0xffff)
}

pub const KVM_REG_ARM_STD_BMAP: __u64 = KVM_REG_ARM_FW_FEAT_BMAP_REG(0);

pub const KVM_REG_ARM_STD_BIT_TRNG_V1_0: u32 = 0;
/* __KERNEL__ only: KVM_REG_ARM_STD_BMAP_BIT_COUNT */

pub const KVM_REG_ARM_STD_HYP_BMAP: __u64 = KVM_REG_ARM_FW_FEAT_BMAP_REG(1);

pub const KVM_REG_ARM_STD_HYP_BIT_PV_TIME: u32 = 0;
/* __KERNEL__ only: KVM_REG_ARM_STD_HYP_BMAP_BIT_COUNT */

/* Vendor hyper call function numbers 0-63 */
pub const KVM_REG_ARM_VENDOR_HYP_BMAP: __u64 = KVM_REG_ARM_FW_FEAT_BMAP_REG(2);

pub const KVM_REG_ARM_VENDOR_HYP_BIT_FUNC_FEAT: u32 = 0;
pub const KVM_REG_ARM_VENDOR_HYP_BIT_PTP: u32 = 1;
/* __KERNEL__ only: KVM_REG_ARM_VENDOR_HYP_BMAP_BIT_COUNT */

/* Vendor hyper call function numbers 64-127 */
pub const KVM_REG_ARM_VENDOR_HYP_BMAP_2: __u64 = KVM_REG_ARM_FW_FEAT_BMAP_REG(3);

pub const KVM_REG_ARM_VENDOR_HYP_BIT_DISCOVER_IMPL_VER: u32 = 0;
pub const KVM_REG_ARM_VENDOR_HYP_BIT_DISCOVER_IMPL_CPUS: u32 = 1;
/* __KERNEL__ only: KVM_REG_ARM_VENDOR_HYP_BMAP_2_BIT_COUNT */

/* Device Control API on vm fd */
pub const KVM_ARM_VM_SMCCC_CTRL: u32 = 0;
pub const KVM_ARM_VM_SMCCC_FILTER: u32 = 0;

/* Device Control API: ARM VGIC */
pub const KVM_DEV_ARM_VGIC_GRP_ADDR: u32 = 0;
pub const KVM_DEV_ARM_VGIC_GRP_DIST_REGS: u32 = 1;
pub const KVM_DEV_ARM_VGIC_GRP_CPU_REGS: u32 = 2;
pub const KVM_DEV_ARM_VGIC_CPUID_SHIFT: u32 = 32;
pub const KVM_DEV_ARM_VGIC_CPUID_MASK: __u64 = 0xffu64 << KVM_DEV_ARM_VGIC_CPUID_SHIFT;
pub const KVM_DEV_ARM_VGIC_V3_MPIDR_SHIFT: u32 = 32;
pub const KVM_DEV_ARM_VGIC_V3_MPIDR_MASK: __u64 =
    0xffffffffu64 << KVM_DEV_ARM_VGIC_V3_MPIDR_SHIFT;
pub const KVM_DEV_ARM_VGIC_OFFSET_SHIFT: u32 = 0;
pub const KVM_DEV_ARM_VGIC_OFFSET_MASK: __u64 = 0xffffffffu64 << KVM_DEV_ARM_VGIC_OFFSET_SHIFT;
pub const KVM_DEV_ARM_VGIC_SYSREG_INSTR_MASK: __u64 = 0xffff;
pub const KVM_DEV_ARM_VGIC_GRP_NR_IRQS: u32 = 3;
pub const KVM_DEV_ARM_VGIC_GRP_CTRL: u32 = 4;
pub const KVM_DEV_ARM_VGIC_GRP_REDIST_REGS: u32 = 5;
pub const KVM_DEV_ARM_VGIC_GRP_CPU_SYSREGS: u32 = 6;
pub const KVM_DEV_ARM_VGIC_GRP_LEVEL_INFO: u32 = 7;
pub const KVM_DEV_ARM_VGIC_GRP_ITS_REGS: u32 = 8;
pub const KVM_DEV_ARM_VGIC_GRP_MAINT_IRQ: u32 = 9;
pub const KVM_DEV_ARM_VGIC_LINE_LEVEL_INFO_SHIFT: u32 = 10;
pub const KVM_DEV_ARM_VGIC_LINE_LEVEL_INFO_MASK: __u64 =
    0x3fffffu64 << KVM_DEV_ARM_VGIC_LINE_LEVEL_INFO_SHIFT;
pub const KVM_DEV_ARM_VGIC_LINE_LEVEL_INTID_MASK: u32 = 0x3ff;
pub const VGIC_LEVEL_INFO_LINE_LEVEL: u32 = 0;

pub const KVM_DEV_ARM_VGIC_CTRL_INIT: u32 = 0;
pub const KVM_DEV_ARM_ITS_SAVE_TABLES: u32 = 1;
pub const KVM_DEV_ARM_ITS_RESTORE_TABLES: u32 = 2;
pub const KVM_DEV_ARM_VGIC_SAVE_PENDING_TABLES: u32 = 3;
pub const KVM_DEV_ARM_ITS_CTRL_RESET: u32 = 4;
pub const KVM_DEV_ARM_VGIC_USERSPACE_PPIS: u32 = 5;

/* Device Control API on vcpu fd */
pub const KVM_ARM_VCPU_PMU_V3_CTRL: u32 = 0;
pub const KVM_ARM_VCPU_PMU_V3_IRQ: u32 = 0;
pub const KVM_ARM_VCPU_PMU_V3_INIT: u32 = 1;
pub const KVM_ARM_VCPU_PMU_V3_FILTER: u32 = 2;
pub const KVM_ARM_VCPU_PMU_V3_SET_PMU: u32 = 3;
pub const KVM_ARM_VCPU_PMU_V3_SET_NR_COUNTERS: u32 = 4;
pub const KVM_ARM_VCPU_TIMER_CTRL: u32 = 1;
pub const KVM_ARM_VCPU_TIMER_IRQ_VTIMER: u32 = 0;
pub const KVM_ARM_VCPU_TIMER_IRQ_PTIMER: u32 = 1;
pub const KVM_ARM_VCPU_TIMER_IRQ_HVTIMER: u32 = 2;
pub const KVM_ARM_VCPU_TIMER_IRQ_HPTIMER: u32 = 3;
pub const KVM_ARM_VCPU_PVTIME_CTRL: u32 = 2;
pub const KVM_ARM_VCPU_PVTIME_IPA: u32 = 0;

/* KVM_IRQ_LINE irq field index values */
pub const KVM_ARM_IRQ_VCPU2_SHIFT: u32 = 28;
pub const KVM_ARM_IRQ_VCPU2_MASK: u32 = 0xf;
pub const KVM_ARM_IRQ_TYPE_SHIFT: u32 = 24;
pub const KVM_ARM_IRQ_TYPE_MASK: u32 = 0xf;
pub const KVM_ARM_IRQ_VCPU_SHIFT: u32 = 16;
pub const KVM_ARM_IRQ_VCPU_MASK: u32 = 0xff;
pub const KVM_ARM_IRQ_NUM_SHIFT: u32 = 0;
pub const KVM_ARM_IRQ_NUM_MASK: u32 = 0xffff;

/* irq_type field */
pub const KVM_ARM_IRQ_TYPE_CPU: u32 = 0;
pub const KVM_ARM_IRQ_TYPE_SPI: u32 = 1;
pub const KVM_ARM_IRQ_TYPE_PPI: u32 = 2;

/* out-of-kernel GIC cpu interrupt injection irq_number field */
pub const KVM_ARM_IRQ_CPU_IRQ: u32 = 0;
pub const KVM_ARM_IRQ_CPU_FIQ: u32 = 1;

/*
 * This used to hold the highest supported SPI, but it is now obsolete
 * and only here to provide source code level compatibility with older
 * userland. The highest SPI number can be set via KVM_DEV_ARM_VGIC_GRP_NR_IRQS.
 */
/* Defined when not __KERNEL__. */
pub const KVM_ARM_IRQ_GIC_MAX: u32 = 127;

/* One single KVM irqchip, ie. the VGIC */
pub const KVM_NR_IRQCHIPS: u32 = 1;

/* PSCI interface */
pub const KVM_PSCI_FN_BASE: u32 = 0x95c1ba5e;
pub const fn KVM_PSCI_FN(n: u32) -> u32 {
    KVM_PSCI_FN_BASE.wrapping_add(n)
}

pub const KVM_PSCI_FN_CPU_SUSPEND: u32 = KVM_PSCI_FN(0);
pub const KVM_PSCI_FN_CPU_OFF: u32 = KVM_PSCI_FN(1);
pub const KVM_PSCI_FN_CPU_ON: u32 = KVM_PSCI_FN(2);
pub const KVM_PSCI_FN_MIGRATE: u32 = KVM_PSCI_FN(3);

pub const KVM_PSCI_RET_SUCCESS: i32 = PSCI_RET_SUCCESS;
pub const KVM_PSCI_RET_NI: i32 = PSCI_RET_NOT_SUPPORTED;
pub const KVM_PSCI_RET_INVAL: i32 = PSCI_RET_INVALID_PARAMS;
pub const KVM_PSCI_RET_DENIED: i32 = PSCI_RET_DENIED;

/* arm64-specific kvm_run::system_event flags */
/*
 * Reset caused by a PSCI v1.1 SYSTEM_RESET2 call.
 * Valid only when the system event has a type of KVM_SYSTEM_EVENT_RESET.
 */
pub const KVM_SYSTEM_EVENT_RESET_FLAG_PSCI_RESET2: __u64 = 1u64 << 0;

/*
 * Shutdown caused by a PSCI v1.3 SYSTEM_OFF2 call.
 * Valid only when the system event has a type of KVM_SYSTEM_EVENT_SHUTDOWN.
 */
pub const KVM_SYSTEM_EVENT_SHUTDOWN_FLAG_PSCI_OFF2: __u64 = 1u64 << 0;

/* run->fail_entry.hardware_entry_failure_reason codes. */
pub const KVM_EXIT_FAIL_ENTRY_CPU_UNSUPPORTED: __u64 = 1u64 << 0;

#[repr(C)]
pub enum kvm_smccc_filter_action {
    KVM_SMCCC_FILTER_HANDLE = 0,
    KVM_SMCCC_FILTER_DENY = 1,
    KVM_SMCCC_FILTER_FWD_TO_USER = 2,
    /* __KERNEL__ only: NR_SMCCC_FILTER_ACTIONS */
}

#[repr(C)]
pub struct kvm_smccc_filter {
    pub base: __u32,
    pub nr_functions: __u32,
    pub action: __u8,
    pub pad: [__u8; 15],
}

/* arm64-specific KVM_EXIT_HYPERCALL flags */
pub const KVM_HYPERCALL_EXIT_SMC: u32 = 1u32 << 0;
pub const KVM_HYPERCALL_EXIT_16BIT: u32 = 1u32 << 1;

/*
 * Get feature ID registers userspace writable mask.
 *
 * From DDI0487J.a, D19.2.66 ("ID_AA64MMFR2_EL1, AArch64 Memory Model
 * Feature Register 2"):
 *
 * "The Feature ID space is defined as the System register space in
 * AArch64 with op0==3, op1=={0, 1, 3}, CRn==0, CRm=={0-7},
 * op2=={0-7}."
 *
 * This covers all currently known R/O registers that indicate
 * anything useful feature wise, including the ID registers.
 *
 * If we ever need to introduce a new range, it will be described as
 * such in the range field.
 */
pub const fn KVM_ARM_FEATURE_ID_RANGE_IDX(
    _op0: __u64,
    op1: __u64,
    _crn: __u64,
    crm: __u64,
    op2: __u64,
) -> __u64 {
    let mut __op1 = op1 & 3;
    __op1 -= (__op1 == 3) as __u64;
    (__op1 << 6) | ((crm & 7) << 3) | op2
}

pub const KVM_ARM_FEATURE_ID_RANGE: u32 = 0;
pub const KVM_ARM_FEATURE_ID_RANGE_SIZE: u32 = 3 * 8 * 8;

#[repr(C)]
pub struct reg_mask_range {
    pub addr: __u64,       /* Pointer to mask array */
    pub range: __u32,      /* Requested range */
    pub reserved: [__u32; 13],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
