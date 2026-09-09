/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/fpstate.h
 *
 *  Copyright (C) 1995 Russell King
 */

/*
 * VFP storage area has:
 *  - FPEXC, FPSCR, FPINST and FPINST2.
 *  - 16 or 32 double precision data registers
 *  - an implementation-dependent word of state for FLDMX/FSTMX (pre-ARMv6)
 *
 *  FPEXC will always be non-zero once the VFP has been used in this process.
 */
#[repr(C)]
pub struct vfp_hard_struct {
    #[cfg(feature = "CONFIG_VFPv3")]
    pub fpregs: [u64; 32],
    #[cfg(not(feature = "CONFIG_VFPv3"))]
    pub fpregs: [u64; 16],
    /* __LINUX_ARM_ARCH__ < 6 */
    #[cfg(feature = "linux_arm_arch_lt_6")]
    pub fpmx_state: u32,
    pub fpexc: u32,
    pub fpscr: u32,
    /*
     * VFP implementation specific state
     */
    pub fpinst: u32,
    pub fpinst2: u32,
    #[cfg(feature = "CONFIG_SMP")]
    pub cpu: u32,
}

#[repr(C)]
pub union vfp_state {
    pub hard: vfp_hard_struct,
}

pub const FP_HARD_SIZE: usize = 35;

#[repr(C)]
pub struct fp_hard_struct {
    pub save: [u32; FP_HARD_SIZE], /* as yet undefined */
}

pub const FP_SOFT_SIZE: usize = 35;

#[repr(C)]
pub struct fp_soft_struct {
    pub save: [u32; FP_SOFT_SIZE], /* undefined information */
}

pub const IWMMXT_SIZE: usize = 0x98;

#[repr(C)]
pub struct iwmmxt_struct {
    pub save: [u32; IWMMXT_SIZE / core::mem::size_of::<u32>()],
}

#[repr(C)]
pub union fp_state {
    pub hard: fp_hard_struct,
    pub soft: fp_soft_struct,
    #[cfg(feature = "CONFIG_IWMMXT")]
    pub iwmmxt: iwmmxt_struct,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
