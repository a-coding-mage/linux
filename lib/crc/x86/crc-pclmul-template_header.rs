/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Macros for accessing the [V]PCLMULQDQ-based CRC functions that are
 * instantiated by crc-pclmul-template.S
 *
 * Copyright 2025 Google LLC
 *
 * Author: Eric Biggers <ebiggers@google.com>
 */

// C dependencies: asm/cpufeatures.h, asm/simd.h, linux/static_call.h, and
// crc-pclmul-consts.h provide the symbols referenced below.

/// Declaration equivalent of DECLARE_CRC_PCLMUL_FUNCS.  Rust cannot perform
/// C-style identifier token pasting, so the generated identifiers are passed
/// explicitly.
#[macro_export]
macro_rules! declare_crc_pclmul_funcs {
    ($sse:ident, $avx2:ident, $avx512:ident, $crc_t:ty) => {
        unsafe extern "C" {
            fn $sse(
                crc: $crc_t,
                p: *const u8,
                len: usize,
                consts_ptr: *const core::ffi::c_void,
            ) -> $crc_t;
            fn $avx2(
                crc: $crc_t,
                p: *const u8,
                len: usize,
                consts_ptr: *const core::ffi::c_void,
            ) -> $crc_t;
            fn $avx512(
                crc: $crc_t,
                p: *const u8,
                len: usize,
                consts_ptr: *const core::ffi::c_void,
            ) -> $crc_t;
        }
    };
}

#[inline]
pub fn have_vpclmul() -> bool {
    // Equivalent to boot_cpu_has(X86_FEATURE_VPCLMULQDQ) &&
    // boot_cpu_has(X86_FEATURE_AVX2) &&
    // cpu_has_xfeatures(XFEATURE_MASK_YMM, NULL).
    unsafe {
        boot_cpu_has(X86_FEATURE_VPCLMULQDQ)
            && boot_cpu_has(X86_FEATURE_AVX2)
            && cpu_has_xfeatures(XFEATURE_MASK_YMM, core::ptr::null())
    }
}

#[inline]
pub fn have_avx512() -> bool {
    unsafe {
        boot_cpu_has(X86_FEATURE_AVX512BW)
            && boot_cpu_has(X86_FEATURE_AVX512VL)
            && !boot_cpu_has(X86_FEATURE_PREFER_YMM)
            && cpu_has_xfeatures(XFEATURE_MASK_AVX512, core::ptr::null())
    }
}

/*
 * Call a [V]PCLMULQDQ optimized CRC function if the data length is at least 16
 * bytes, the CPU has PCLMULQDQ support, and the current context may use SIMD.
 *
 * 16 bytes is the minimum length supported by the [V]PCLMULQDQ functions.
 * There is overhead associated with kernel_fpu_begin() and kernel_fpu_end(),
 * varying by CPU and factors such as which parts of the "FPU" state userspace
 * has touched, which could result in a larger cutoff being better.  Indeed, a
 * larger cutoff is usually better for a *single* message.  However, the
 * overhead of the FPU section gets amortized if multiple FPU sections get
 * executed before returning to userspace, since the XSAVE and XRSTOR occur only
 * once.  Considering that and the fact that the [V]PCLMULQDQ code is lighter on
 * the dcache than the table-based code is, a 16-byte cutoff seems to work well.
 */
#[macro_export]
macro_rules! crc_pclmul {
    ($crc:ident, $p:expr, $len:expr, $call:expr, $consts:expr, $have_pclmulqdq:expr) => {{
        if ($len) >= 16
            && static_branch_likely(&($have_pclmulqdq))
            && likely(unsafe { irq_fpu_usable() })
        {
            let consts_ptr = ($consts).fold_across_128_bits_consts;
            unsafe { kernel_fpu_begin() };
            $crc = unsafe { ($call)($crc, $p, $len, consts_ptr) };
            unsafe { kernel_fpu_end() };
            return $crc;
        }
    }};
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
