/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2012  MIPS Technologies, Inc.  All rights reserved.
 * Copyright (C) 2013 Cavium, Inc.
 * Authors: Sanjay Lal <sanjayl@kymasys.com>
 */

/* Depends on linux/types.h for the original C integer typedefs. */
pub type __u32 = u32;
pub type __u64 = u64;

/*
 * KVM MIPS specific structures and definitions.
 *
 * Some parts derived from the x86 version of this file.
 */

/*
 * for KVM_GET_REGS and KVM_SET_REGS
 *
 * If Config[AT] is zero (32-bit CPU), the register contents are
 * stored in the lower 32-bits of the struct kvm_regs fields and sign
 * extended to 64-bits.
 */
#[repr(C)]
pub struct kvm_regs {
    /* out (KVM_GET_REGS) / in (KVM_SET_REGS) */
    pub gpr: [__u64; 32],
    pub hi: __u64,
    pub lo: __u64,
    pub pc: __u64,
}

/*
 * for KVM_GET_FPU and KVM_SET_FPU
 */
#[repr(C)]
pub struct kvm_fpu {}

/*
 * For MIPS, we use KVM_SET_ONE_REG and KVM_GET_ONE_REG to access various
 * registers.  The id field is broken down as follows:
 *
 *  bits[63..52] - As per linux/kvm.h
 *  bits[51..32] - Must be zero.
 *  bits[31..16] - Register set.
 *
 * Register set = 0: GP registers from kvm_regs (see definitions below).
 *
 * Register set = 1: CP0 registers.
 *  bits[15..8]  - Must be zero.
 *  bits[7..3]   - Register 'rd'  index.
 *  bits[2..0]   - Register 'sel' index.
 *
 * Register set = 2: KVM specific registers (see definitions below).
 *
 * Register set = 3: FPU / MSA registers (see definitions below).
 *
 * Other sets registers may be added in the future.  Each set would
 * have its own identifier in bits[31..16].
 */

pub const KVM_REG_MIPS_GP: u64 = KVM_REG_MIPS | 0x0000000000000000_u64;
pub const KVM_REG_MIPS_CP0: u64 = KVM_REG_MIPS | 0x0000000000010000_u64;
pub const KVM_REG_MIPS_KVM: u64 = KVM_REG_MIPS | 0x0000000000020000_u64;
pub const KVM_REG_MIPS_FPU: u64 = KVM_REG_MIPS | 0x0000000000030000_u64;

/*
 * KVM_REG_MIPS_GP - General purpose registers from kvm_regs.
 */

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

/*
 * KVM_REG_MIPS_KVM - KVM specific control registers.
 */

/*
 * CP0_Count control
 * DC:    Set 0: Master disable CP0_Count and set COUNT_RESUME to now
 *        Set 1: Master re-enable CP0_Count with unchanged bias, handling timer
 *               interrupts since COUNT_RESUME
 *        This can be used to freeze the timer to get a consistent snapshot of
 *        the CP0_Count and timer interrupt pending state, while also resuming
 *        safely without losing time or guest timer interrupts.
 * Other: Reserved, do not change.
 */
pub const KVM_REG_MIPS_COUNT_CTL: u64 = KVM_REG_MIPS_KVM | KVM_REG_SIZE_U64 | 0;
pub const KVM_REG_MIPS_COUNT_CTL_DC: u32 = 0x00000001;

/*
 * CP0_Count resume monotonic nanoseconds
 * The monotonic nanosecond time of the last set of COUNT_CTL.DC (master
 * disable). Any reads and writes of Count related registers while
 * COUNT_CTL.DC=1 will appear to occur at this time. When COUNT_CTL.DC is
 * cleared again (master enable) any timer interrupts since this time will be
 * emulated.
 * Modifications to times in the future are rejected.
 */
pub const KVM_REG_MIPS_COUNT_RESUME: u64 = KVM_REG_MIPS_KVM | KVM_REG_SIZE_U64 | 1;
/*
 * CP0_Count rate in Hz
 * Specifies the rate of the CP0_Count timer in Hz. Modifications occur without
 * discontinuities in CP0_Count.
 */
pub const KVM_REG_MIPS_COUNT_HZ: u64 = KVM_REG_MIPS_KVM | KVM_REG_SIZE_U64 | 2;

/*
 * KVM_REG_MIPS_FPU - Floating Point and MIPS SIMD Architecture (MSA) registers.
 *
 *  bits[15..8]  - Register subset (see definitions below).
 *  bits[7..5]   - Must be zero.
 *  bits[4..0]   - Register number within register subset.
 */

pub const KVM_REG_MIPS_FPR: u64 = KVM_REG_MIPS_FPU | 0x0000000000000000_u64;
pub const KVM_REG_MIPS_FCR: u64 = KVM_REG_MIPS_FPU | 0x0000000000000100_u64;
pub const KVM_REG_MIPS_MSACR: u64 = KVM_REG_MIPS_FPU | 0x0000000000000200_u64;

/*
 * KVM_REG_MIPS_FPR - Floating point / Vector registers.
 */
pub const fn KVM_REG_MIPS_FPR_32(n: u64) -> u64 {
    KVM_REG_MIPS_FPR | KVM_REG_SIZE_U32 | n
}

pub const fn KVM_REG_MIPS_FPR_64(n: u64) -> u64 {
    KVM_REG_MIPS_FPR | KVM_REG_SIZE_U64 | n
}

pub const fn KVM_REG_MIPS_VEC_128(n: u64) -> u64 {
    KVM_REG_MIPS_FPR | KVM_REG_SIZE_U128 | n
}

/*
 * KVM_REG_MIPS_FCR - Floating point control registers.
 */
pub const KVM_REG_MIPS_FCR_IR: u64 = KVM_REG_MIPS_FCR | KVM_REG_SIZE_U32 | 0;
pub const KVM_REG_MIPS_FCR_CSR: u64 = KVM_REG_MIPS_FCR | KVM_REG_SIZE_U32 | 31;

/*
 * KVM_REG_MIPS_MSACR - MIPS SIMD Architecture (MSA) control registers.
 */
pub const KVM_REG_MIPS_MSA_IR: u64 = KVM_REG_MIPS_MSACR | KVM_REG_SIZE_U32 | 0;
pub const KVM_REG_MIPS_MSA_CSR: u64 = KVM_REG_MIPS_MSACR | KVM_REG_SIZE_U32 | 1;

/*
 * KVM MIPS specific structures and definitions
 *
 */
#[repr(C)]
pub struct kvm_debug_exit_arch {
    pub epc: __u64,
}

/* for KVM_SET_GUEST_DEBUG */
#[repr(C)]
pub struct kvm_guest_debug_arch {}

/* definition of registers in kvm_run */
#[repr(C)]
pub struct kvm_sync_regs {}

/* dummy definition */
#[repr(C)]
pub struct kvm_sregs {}

#[repr(C)]
pub struct kvm_mips_interrupt {
    /* in */
    pub cpu: __u32,
    pub irq: __u32,
}
