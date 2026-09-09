/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2012 MIPS Technologies, Inc. All rights reserved.
 * Copyright (C) 2013 Cavium, Inc.
 * Authors: Sanjay Lal <sanjayl@kymasys.com>
 */

// Dependency: linux/types.h

/* KVM MIPS specific structures and definitions. */

pub const KVM_COALESCED_MMIO_PAGE_OFFSET: u64 = 1;

#[repr(C)]
pub struct kvm_regs {
    pub gpr: [u64; 32],
    pub hi: u64,
    pub lo: u64,
    pub pc: u64,
}

#[repr(C)]
pub struct kvm_fpu {}

pub const KVM_REG_MIPS_GP: u64 = KVM_REG_MIPS | 0x0000_0000_0000_0000u64;
pub const KVM_REG_MIPS_CP0: u64 = KVM_REG_MIPS | 0x0000_0000_0001_0000u64;
pub const KVM_REG_MIPS_KVM: u64 = KVM_REG_MIPS | 0x0000_0000_0002_0000u64;
pub const KVM_REG_MIPS_FPU: u64 = KVM_REG_MIPS | 0x0000_0000_0003_0000u64;

pub const KVM_REG_MIPS_R0: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 0;
pub const KVM_REG_MIPS_R1: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 1;
pub const KVM_REG_MIPS_R2: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 2;
pub const KVM_REG_MIPS_R3: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 3;
pub const KVM_REG_MIPS_R4: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 4;
pub const KVM_REG_MIPS_R5: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 5;
pub const KVM_REG_MIPS_R6: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 6;
pub const KVM_REG_MIPS_R7: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 7;
pub const KVM_REG_MIPS_R8: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 8;
pub const KVM_REG_MIPS_R9: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 9;
pub const KVM_REG_MIPS_R10: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 10;
pub const KVM_REG_MIPS_R11: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 11;
pub const KVM_REG_MIPS_R12: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 12;
pub const KVM_REG_MIPS_R13: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 13;
pub const KVM_REG_MIPS_R14: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 14;
pub const KVM_REG_MIPS_R15: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 15;
pub const KVM_REG_MIPS_R16: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 16;
pub const KVM_REG_MIPS_R17: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 17;
pub const KVM_REG_MIPS_R18: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 18;
pub const KVM_REG_MIPS_R19: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 19;
pub const KVM_REG_MIPS_R20: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 20;
pub const KVM_REG_MIPS_R21: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 21;
pub const KVM_REG_MIPS_R22: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 22;
pub const KVM_REG_MIPS_R23: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 23;
pub const KVM_REG_MIPS_R24: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 24;
pub const KVM_REG_MIPS_R25: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 25;
pub const KVM_REG_MIPS_R26: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 26;
pub const KVM_REG_MIPS_R27: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 27;
pub const KVM_REG_MIPS_R28: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 28;
pub const KVM_REG_MIPS_R29: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 29;
pub const KVM_REG_MIPS_R30: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 30;
pub const KVM_REG_MIPS_R31: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 31;
pub const KVM_REG_MIPS_HI: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 32;
pub const KVM_REG_MIPS_LO: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 33;
pub const KVM_REG_MIPS_PC: u64 = KVM_REG_MIPS_GP | KVM_REG_SIZE_U64 | 34;

pub const KVM_REG_MIPS_MAAR: u64 = KVM_REG_MIPS_CP0 | (1u64 << 8);
#[inline]
pub const fn KVM_REG_MIPS_CP0_MAAR(n: u64) -> u64 { KVM_REG_MIPS_MAAR | KVM_REG_SIZE_U64 | n }

pub const KVM_REG_MIPS_COUNT_CTL: u64 = KVM_REG_MIPS_KVM | KVM_REG_SIZE_U64;
pub const KVM_REG_MIPS_COUNT_CTL_DC: u64 = 0x00000001;
pub const KVM_REG_MIPS_COUNT_RESUME: u64 = KVM_REG_MIPS_KVM | KVM_REG_SIZE_U64 | 1;
pub const KVM_REG_MIPS_COUNT_HZ: u64 = KVM_REG_MIPS_KVM | KVM_REG_SIZE_U64 | 2;

pub const KVM_REG_MIPS_FPR: u64 = KVM_REG_MIPS_FPU | 0x0000_0000_0000_0000u64;
pub const KVM_REG_MIPS_FCR: u64 = KVM_REG_MIPS_FPU | 0x0000_0000_0000_0100u64;
pub const KVM_REG_MIPS_MSACR: u64 = KVM_REG_MIPS_FPU | 0x0000_0000_0000_0200u64;
#[inline]
pub const fn KVM_REG_MIPS_FPR_32(n: u64) -> u64 { KVM_REG_MIPS_FPR | KVM_REG_SIZE_U32 | n }
#[inline]
pub const fn KVM_REG_MIPS_FPR_64(n: u64) -> u64 { KVM_REG_MIPS_FPR | KVM_REG_SIZE_U64 | n }
#[inline]
pub const fn KVM_REG_MIPS_VEC_128(n: u64) -> u64 { KVM_REG_MIPS_FPR | KVM_REG_SIZE_U128 | n }
pub const KVM_REG_MIPS_FCR_IR: u64 = KVM_REG_MIPS_FCR | KVM_REG_SIZE_U32;
pub const KVM_REG_MIPS_FCR_CSR: u64 = KVM_REG_MIPS_FCR | KVM_REG_SIZE_U32 | 31;
pub const KVM_REG_MIPS_MSA_IR: u64 = KVM_REG_MIPS_MSACR | KVM_REG_SIZE_U32;
pub const KVM_REG_MIPS_MSA_CSR: u64 = KVM_REG_MIPS_MSACR | KVM_REG_SIZE_U32 | 1;

#[repr(C)]
pub struct kvm_debug_exit_arch { pub epc: u64 }
#[repr(C)]
pub struct kvm_guest_debug_arch {}
#[repr(C)]
pub struct kvm_sync_regs {}
#[repr(C)]
pub struct kvm_sregs {}
#[repr(C)]
pub struct kvm_mips_interrupt {
    pub cpu: u32,
    pub irq: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
