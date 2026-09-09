/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2024 Institute of Software, CAS.
 *
 * Definitions for RISC-V RAID-6 code
 */

// C dependencies supplied by other translation units:
// `asm/vector.h` and `algos.h`.

// The C preprocessor token-pasting in RAID6_RVV_WRAPPER cannot be expressed
// directly by stable Rust identifier syntax.  The generated identifiers are
// therefore supplied explicitly at the invocation site.
#[macro_export]
macro_rules! RAID6_RVV_WRAPPER {
    (
        $gen_syndrome:ident,
        $xor_syndrome:ident,
        $gen_syndrome_real:ident,
        $xor_syndrome_real:ident,
        $calls:ident,
        $name:expr
    ) => {
        extern "C" {
            fn kernel_vector_begin();
            fn kernel_vector_end();
            fn $gen_syndrome_real(d: ::core::ffi::c_int, b: ::core::ffi::c_ulong, p: *mut *mut ::core::ffi::c_void);
            fn $xor_syndrome_real(
                d: ::core::ffi::c_int,
                s1: ::core::ffi::c_int,
                s2: ::core::ffi::c_int,
                b: ::core::ffi::c_ulong,
                p: *mut *mut ::core::ffi::c_void,
            );
        }

        unsafe fn $gen_syndrome(
            disks: ::core::ffi::c_int,
            bytes: usize,
            ptrs: *mut *mut ::core::ffi::c_void,
        ) {
            kernel_vector_begin();
            $gen_syndrome_real(disks, bytes as ::core::ffi::c_ulong, ptrs);
            kernel_vector_end();
        }

        unsafe fn $xor_syndrome(
            disks: ::core::ffi::c_int,
            start: ::core::ffi::c_int,
            stop: ::core::ffi::c_int,
            bytes: usize,
            ptrs: *mut *mut ::core::ffi::c_void,
        ) {
            kernel_vector_begin();
            $xor_syndrome_real(
                disks,
                start,
                stop,
                bytes as ::core::ffi::c_ulong,
                ptrs,
            );
            kernel_vector_end();
        }

        // Corresponds to `struct raid6_calls const` in algos.h.  The
        // dependency-provided type and function-pointer layout are retained.
        static $calls: raid6_calls = raid6_calls {
            gen_syndrome: Some($gen_syndrome),
            xor_syndrome: Some($xor_syndrome),
            name: $name,
        };
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
