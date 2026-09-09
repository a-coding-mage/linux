/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Synopsys, Inc. (www.synopsys.com)
 *
 */

/* C header guard: _ASM_ARC_FPU_H */

/* C dependency: <asm/ptrace.h> */

#[cfg(feature = "CONFIG_ARC_FPU_SAVE_RESTORE")]
#[cfg(feature = "CONFIG_ISA_ARCOMPACT")]
#[repr(C)]
pub struct arc_fpu {
    pub aux_dpfp: [ArcFpuAuxDpfp; 2],
}

#[cfg(feature = "CONFIG_ARC_FPU_SAVE_RESTORE")]
#[cfg(feature = "CONFIG_ISA_ARCOMPACT")]
#[repr(C)]
pub struct ArcFpuAuxDpfp {
    pub l: core::ffi::c_uint,
    pub h: core::ffi::c_uint,
}

#[cfg(feature = "CONFIG_ARC_FPU_SAVE_RESTORE")]
#[cfg(feature = "CONFIG_ISA_ARCOMPACT")]
#[macro_export]
macro_rules! fpu_init_task {
    ($regs:expr) => {{
        let _ = &$regs;
    }};
}

/* ARCv2 FPU control and status auxiliary registers. */
#[cfg(feature = "CONFIG_ARC_FPU_SAVE_RESTORE")]
#[cfg(not(feature = "CONFIG_ISA_ARCOMPACT"))]
#[repr(C)]
pub struct arc_fpu {
    pub ctrl: core::ffi::c_uint,
    pub status: core::ffi::c_uint,
}

#[cfg(feature = "CONFIG_ARC_FPU_SAVE_RESTORE")]
#[cfg(not(feature = "CONFIG_ISA_ARCOMPACT"))]
extern "C" {
    pub fn fpu_init_task(regs: *mut pt_regs);
}

/* Declaration supplied by the corresponding task-structure dependency. */
#[cfg(feature = "CONFIG_ARC_FPU_SAVE_RESTORE")]
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_ARC_FPU_SAVE_RESTORE")]
extern "C" {
    pub fn fpu_save_restore(p: *mut task_struct, n: *mut task_struct);
}

#[cfg(not(feature = "CONFIG_ARC_FPU_SAVE_RESTORE"))]
#[macro_export]
macro_rules! fpu_save_restore {
    ($p:expr, $n:expr) => {{
        let _ = (&$p, &$n);
    }};
}

#[cfg(not(feature = "CONFIG_ARC_FPU_SAVE_RESTORE"))]
#[macro_export]
macro_rules! fpu_init_task {
    ($regs:expr) => {{
        let _ = &$regs;
    }};
}

/* External type supplied by the C <asm/ptrace.h> dependency. */
#[cfg(feature = "CONFIG_ARC_FPU_SAVE_RESTORE")]
#[cfg(not(feature = "CONFIG_ISA_ARCOMPACT"))]
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
