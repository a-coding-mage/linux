// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2023 SiFive
 * Author: Andy Chiu <andy.chiu@sifive.com>
 */

// C dependencies: <linux/linkage.h>, <asm/asm.h>, <asm/vector.h>, and
// <asm/simd.h>.  Under CONFIG_MMU, <asm/asm-prototypes.h> is also required.

#[cfg(CONFIG_MMU)]
pub static mut riscv_v_usercopy_threshold: usize = 0; // CONFIG_RISCV_ISA_V_UCOPY_THRESHOLD

#[cfg(CONFIG_MMU)]
extern "C" {
    fn __asm_vector_usercopy(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, n: usize) -> i32;
    fn __asm_vector_usercopy_sum_enabled(
        dst: *mut core::ffi::c_void,
        src: *mut core::ffi::c_void,
        n: usize,
    ) -> i32;
    fn fallback_scalar_usercopy(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, n: usize) -> i32;
    fn fallback_scalar_usercopy_sum_enabled(
        dst: *mut core::ffi::c_void,
        src: *mut core::ffi::c_void,
        n: usize,
    ) -> i32;
    fn may_use_simd() -> bool;
    fn kernel_vector_begin();
    fn kernel_vector_end();
}

#[cfg(CONFIG_MMU)]
#[no_mangle]
pub unsafe extern "C" fn enter_vector_usercopy(
    mut dst: *mut core::ffi::c_void,
    mut src: *mut core::ffi::c_void,
    mut n: usize,
    enable_sum: bool,
) -> i32 {
    let remain: usize;
    let copied: usize;

    // skip has_vector() check because it has been done by the asm
    if !may_use_simd() {
        return if enable_sum {
            fallback_scalar_usercopy(dst, src, n)
        } else {
            fallback_scalar_usercopy_sum_enabled(dst, src, n)
        };
    }

    kernel_vector_begin();
    remain = if enable_sum {
        __asm_vector_usercopy(dst, src, n) as usize
    } else {
        __asm_vector_usercopy_sum_enabled(dst, src, n) as usize
    };
    kernel_vector_end();

    if remain != 0 {
        copied = n - remain;
        dst = (dst as *mut u8).add(copied) as *mut core::ffi::c_void;
        src = (src as *mut u8).add(copied) as *mut core::ffi::c_void;
        n = remain;
        return if enable_sum {
            fallback_scalar_usercopy(dst, src, n)
        } else {
            fallback_scalar_usercopy_sum_enabled(dst, src, n)
        };
    }

    remain as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
