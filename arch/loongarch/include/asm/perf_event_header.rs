/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependency supplied by asm/ptrace.h in the original header.

/// Equivalent to `perf_arch_bpf_user_pt_regs(regs)`.
#[macro_export]
macro_rules! perf_arch_bpf_user_pt_regs {
    ($regs:expr) => {
        $regs as *mut user_pt_regs
    };
}

/// Equivalent to `perf_arch_fetch_caller_regs(regs, __ip)`.
///
/// The frame-address operation is retained as an external declaration because
/// it is supplied by the target/compiler environment in the original C code.
#[macro_export]
macro_rules! perf_arch_fetch_caller_regs {
    ($regs:expr, $__ip:expr) => {{
        unsafe {
            (*$regs).csr_era = $__ip;
            (*$regs).regs[3] =
                __builtin_frame_address(0) as usize;
        }
    }};
}

extern "C" {
    fn __builtin_frame_address(level: i32) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
