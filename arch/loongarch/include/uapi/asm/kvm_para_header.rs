/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: <linux/types.h> supplies Linux integer types.

/*
 * CPUCFG index area: 0x40000000 -- 0x400000ff
 * SW emulation for KVM hypervirsor
 */
pub const CPUCFG_KVM_BASE: u32 = 0x40000000;
pub const CPUCFG_KVM_SIZE: u32 = 0x100;
pub const CPUCFG_KVM_SIG: u32 = CPUCFG_KVM_BASE + 0;
pub const KVM_SIGNATURE: &[u8; 4] = b"KVM\0";
pub const CPUCFG_KVM_FEATURE: u32 = CPUCFG_KVM_BASE + 4;
pub const KVM_FEATURE_IPI: u32 = 1;
pub const KVM_FEATURE_STEAL_TIME: u32 = 2;
pub const KVM_FEATURE_PREEMPT: u32 = 3;
/* BIT 24 - 31 are features configurable by user space vmm */
pub const KVM_FEATURE_VIRT_EXTIOI: u32 = 24;
pub const KVM_FEATURE_USER_HCALL: u32 = 25;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
