/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * This header file provides a method for making a hypercall to the host
 * Architectures should define:
 * - kvm_hypercall0, kvm_hypercall1...
 * - kvm_arch_para_features
 * - kvm_para_available
 */

/* Return values for hypercalls */
pub const KVM_ENOSYS: i32 = 1000;
pub const KVM_EFAULT: _ = EFAULT;
pub const KVM_EINVAL: _ = EINVAL;
pub const KVM_E2BIG: _ = E2BIG;
pub const KVM_EPERM: _ = EPERM;
pub const KVM_EOPNOTSUPP: i32 = 95;

pub const KVM_HC_VAPIC_POLL_IRQ: i32 = 1;
pub const KVM_HC_MMU_OP: i32 = 2;
pub const KVM_HC_FEATURES: i32 = 3;
pub const KVM_HC_PPC_MAP_MAGIC_PAGE: i32 = 4;
pub const KVM_HC_KICK_CPU: i32 = 5;
pub const KVM_HC_MIPS_GET_CLOCK_FREQ: i32 = 6;
pub const KVM_HC_MIPS_EXIT_VM: i32 = 7;
pub const KVM_HC_MIPS_CONSOLE_OUTPUT: i32 = 8;
pub const KVM_HC_CLOCK_PAIRING: i32 = 9;
pub const KVM_HC_SEND_IPI: i32 = 10;
pub const KVM_HC_SCHED_YIELD: i32 = 11;
pub const KVM_HC_MAP_GPA_RANGE: i32 = 12;

/*
 * Hypercalls use architecture-specific declarations from <asm/kvm_para.h>.
 * Those declarations are supplied by the target architecture.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
