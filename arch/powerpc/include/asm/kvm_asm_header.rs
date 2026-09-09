/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright IBM Corp. 2008
 *
 * Authors: Hollis Blanchard <hollisb@us.ibm.com>
 */

// The following assembler-only macros are intentionally preserved as comments;
// they have no direct Rust expression and are used only by PowerPC assembly.
// Under CONFIG_64BIT:
//   PPC_STD(sreg, offset, areg) = std sreg, (offset)(areg)
//   PPC_LD(treg, offset, areg)  = ld treg, (offset)(areg)
// Otherwise:
//   PPC_STD(sreg, offset, areg) = stw sreg, (offset+4)(areg)
//   PPC_LD(treg, offset, areg)  = lwz treg, (offset+4)(areg)

/* IVPR must be 64KiB-aligned. */
pub const VCPU_SIZE_ORDER: u32 = 4;
pub const VCPU_SIZE_LOG: u32 = VCPU_SIZE_ORDER + 12;
pub const VCPU_SIZE_BYTES: u32 = 1 << VCPU_SIZE_LOG;

pub const BOOKE_INTERRUPT_CRITICAL: u32 = 0;
pub const BOOKE_INTERRUPT_MACHINE_CHECK: u32 = 1;
pub const BOOKE_INTERRUPT_DATA_STORAGE: u32 = 2;
pub const BOOKE_INTERRUPT_INST_STORAGE: u32 = 3;
pub const BOOKE_INTERRUPT_EXTERNAL: u32 = 4;
pub const BOOKE_INTERRUPT_ALIGNMENT: u32 = 5;
pub const BOOKE_INTERRUPT_PROGRAM: u32 = 6;
pub const BOOKE_INTERRUPT_FP_UNAVAIL: u32 = 7;
pub const BOOKE_INTERRUPT_SYSCALL: u32 = 8;
pub const BOOKE_INTERRUPT_AP_UNAVAIL: u32 = 9;
pub const BOOKE_INTERRUPT_DECREMENTER: u32 = 10;
pub const BOOKE_INTERRUPT_FIT: u32 = 11;
pub const BOOKE_INTERRUPT_WATCHDOG: u32 = 12;
pub const BOOKE_INTERRUPT_DTLB_MISS: u32 = 13;
pub const BOOKE_INTERRUPT_ITLB_MISS: u32 = 14;
pub const BOOKE_INTERRUPT_DEBUG: u32 = 15;

/* E500; these are present when CONFIG_SPE_POSSIBLE is enabled. */
pub const BOOKE_INTERRUPT_SPE_UNAVAIL: u32 = 32;
pub const BOOKE_INTERRUPT_SPE_FP_DATA: u32 = 33;
pub const BOOKE_INTERRUPT_SPE_FP_ROUND: u32 = 34;

/* These are present when CONFIG_PPC_E500MC is enabled. */
pub const BOOKE_INTERRUPT_ALTIVEC_UNAVAIL: u32 = 32;
pub const BOOKE_INTERRUPT_ALTIVEC_ASSIST: u32 = 33;

pub const BOOKE_INTERRUPT_PERFORMANCE_MONITOR: u32 = 35;
pub const BOOKE_INTERRUPT_DOORBELL: u32 = 36;
pub const BOOKE_INTERRUPT_DOORBELL_CRITICAL: u32 = 37;

/* booke_hv */
pub const BOOKE_INTERRUPT_GUEST_DBELL: u32 = 38;
pub const BOOKE_INTERRUPT_GUEST_DBELL_CRIT: u32 = 39;
pub const BOOKE_INTERRUPT_HV_SYSCALL: u32 = 40;
pub const BOOKE_INTERRUPT_HV_PRIV: u32 = 41;
pub const BOOKE_INTERRUPT_LRAT_ERROR: u32 = 42;

/* book3s */
pub const BOOK3S_INTERRUPT_SYSTEM_RESET: u32 = 0x100;
pub const BOOK3S_INTERRUPT_MACHINE_CHECK: u32 = 0x200;
pub const BOOK3S_INTERRUPT_DATA_STORAGE: u32 = 0x300;
pub const BOOK3S_INTERRUPT_DATA_SEGMENT: u32 = 0x380;
pub const BOOK3S_INTERRUPT_INST_STORAGE: u32 = 0x400;
pub const BOOK3S_INTERRUPT_INST_SEGMENT: u32 = 0x480;
pub const BOOK3S_INTERRUPT_EXTERNAL: u32 = 0x500;
pub const BOOK3S_INTERRUPT_EXTERNAL_HV: u32 = 0x502;
pub const BOOK3S_INTERRUPT_ALIGNMENT: u32 = 0x600;
pub const BOOK3S_INTERRUPT_PROGRAM: u32 = 0x700;
pub const BOOK3S_INTERRUPT_FP_UNAVAIL: u32 = 0x800;
pub const BOOK3S_INTERRUPT_DECREMENTER: u32 = 0x900;
pub const BOOK3S_INTERRUPT_HV_DECREMENTER: u32 = 0x980;
pub const BOOK3S_INTERRUPT_NESTED_HV_DECREMENTER: u32 = 0x1980;
pub const BOOK3S_INTERRUPT_DOORBELL: u32 = 0xa00;
pub const BOOK3S_INTERRUPT_SYSCALL: u32 = 0xc00;
pub const BOOK3S_INTERRUPT_TRACE: u32 = 0xd00;
pub const BOOK3S_INTERRUPT_H_DATA_STORAGE: u32 = 0xe00;
pub const BOOK3S_INTERRUPT_H_INST_STORAGE: u32 = 0xe20;
pub const BOOK3S_INTERRUPT_H_EMUL_ASSIST: u32 = 0xe40;
pub const BOOK3S_INTERRUPT_HMI: u32 = 0xe60;
pub const BOOK3S_INTERRUPT_H_DOORBELL: u32 = 0xe80;
pub const BOOK3S_INTERRUPT_H_VIRT: u32 = 0xea0;
pub const BOOK3S_INTERRUPT_PERFMON: u32 = 0xf00;
pub const BOOK3S_INTERRUPT_ALTIVEC: u32 = 0xf20;
pub const BOOK3S_INTERRUPT_VSX: u32 = 0xf40;
pub const BOOK3S_INTERRUPT_FAC_UNAVAIL: u32 = 0xf60;
pub const BOOK3S_INTERRUPT_H_FAC_UNAVAIL: u32 = 0xf80;

/* book3s_hv */
pub const BOOK3S_INTERRUPT_HV_SOFTPATCH: u32 = 0x1500;

/*
 * Special trap used to indicate to host that this is a
 * passthrough interrupt that could not be handled
 * completely in the guest.
 */
pub const BOOK3S_INTERRUPT_HV_RM_HARD: u32 = 0x5555;

pub const BOOK3S_IRQPRIO_SYSTEM_RESET: u32 = 0;
pub const BOOK3S_IRQPRIO_DATA_SEGMENT: u32 = 1;
pub const BOOK3S_IRQPRIO_INST_SEGMENT: u32 = 2;
pub const BOOK3S_IRQPRIO_DATA_STORAGE: u32 = 3;
pub const BOOK3S_IRQPRIO_INST_STORAGE: u32 = 4;
pub const BOOK3S_IRQPRIO_ALIGNMENT: u32 = 5;
pub const BOOK3S_IRQPRIO_PROGRAM: u32 = 6;
pub const BOOK3S_IRQPRIO_FP_UNAVAIL: u32 = 7;
pub const BOOK3S_IRQPRIO_ALTIVEC: u32 = 8;
pub const BOOK3S_IRQPRIO_VSX: u32 = 9;
pub const BOOK3S_IRQPRIO_FAC_UNAVAIL: u32 = 10;
pub const BOOK3S_IRQPRIO_SYSCALL: u32 = 11;
pub const BOOK3S_IRQPRIO_MACHINE_CHECK: u32 = 12;
pub const BOOK3S_IRQPRIO_DEBUG: u32 = 13;
pub const BOOK3S_IRQPRIO_EXTERNAL: u32 = 14;
pub const BOOK3S_IRQPRIO_DECREMENTER: u32 = 15;
pub const BOOK3S_IRQPRIO_PERFORMANCE_MONITOR: u32 = 16;
pub const BOOK3S_IRQPRIO_MAX: u32 = 17;

pub const BOOK3S_HFLAG_DCBZ32: u32 = 0x1;
pub const BOOK3S_HFLAG_SLB: u32 = 0x2;
pub const BOOK3S_HFLAG_PAIRED_SINGLE: u32 = 0x4;
pub const BOOK3S_HFLAG_NATIVE_PS: u32 = 0x8;
pub const BOOK3S_HFLAG_MULTI_PGSIZE: u32 = 0x10;
pub const BOOK3S_HFLAG_NEW_TLBIE: u32 = 0x20;
pub const BOOK3S_HFLAG_SPLIT_HACK: u32 = 0x40;

pub const RESUME_FLAG_NV: u32 = 1 << 0; /* Reload guest nonvolatile state? */
pub const RESUME_FLAG_HOST: u32 = 1 << 1; /* Resume host? */
pub const RESUME_FLAG_ARCH1: u32 = 1 << 2;
pub const RESUME_FLAG_ARCH2: u32 = 1 << 3;

pub const RESUME_GUEST: u32 = 0;
pub const RESUME_GUEST_NV: u32 = RESUME_FLAG_NV;
pub const RESUME_HOST: u32 = RESUME_FLAG_HOST;
pub const RESUME_HOST_NV: u32 = RESUME_FLAG_HOST | RESUME_FLAG_NV;

pub const KVM_GUEST_MODE_NONE: u32 = 0;
pub const KVM_GUEST_MODE_GUEST: u32 = 1;
pub const KVM_GUEST_MODE_SKIP: u32 = 2;
pub const KVM_GUEST_MODE_GUEST_HV: u32 = 3;
pub const KVM_GUEST_MODE_HOST_HV: u32 = 4;
pub const KVM_GUEST_MODE_HV_P9: u32 = 5; /* ISA >= v3.0 path */

pub const KVM_INST_FETCH_FAILED: i32 = -1;

/* Extract PO and XOP opcode fields */
pub const PO_XOP_OPCODE_MASK: u32 = 0xfc0007fe;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
