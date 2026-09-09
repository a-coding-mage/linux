// SPDX-License-Identifier: GPL-2.0-only
/*
 * RAID6 syndrome calculation using ARM NEON intrinsics
 *
 * Copyright (C) 2013 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// Dependency intent preserved from <asm/simd.h> and "algos.h".

/*
 * These wrappers ensure that the NEON implementations are only called inside
 * the kernel SIMD context. The original scoped_ksimd() macro supplies that
 * context; its implementation is provided by the surrounding kernel.
 */

extern "C" {
    fn raid6_neon1_gen_syndrome_real(disks: i32, bytes: u64, ptrs: *mut *mut core::ffi::c_void);
    fn raid6_neon1_xor_syndrome_real(
        disks: i32,
        start: i32,
        stop: i32,
        bytes: u64,
        ptrs: *mut *mut core::ffi::c_void,
    );
    fn raid6_neon2_gen_syndrome_real(disks: i32, bytes: u64, ptrs: *mut *mut core::ffi::c_void);
    fn raid6_neon2_xor_syndrome_real(
        disks: i32,
        start: i32,
        stop: i32,
        bytes: u64,
        ptrs: *mut *mut core::ffi::c_void,
    );
    fn raid6_neon4_gen_syndrome_real(disks: i32, bytes: u64, ptrs: *mut *mut core::ffi::c_void);
    fn raid6_neon4_xor_syndrome_real(
        disks: i32,
        start: i32,
        stop: i32,
        bytes: u64,
        ptrs: *mut *mut core::ffi::c_void,
    );
    fn raid6_neon8_gen_syndrome_real(disks: i32, bytes: u64, ptrs: *mut *mut core::ffi::c_void);
    fn raid6_neon8_xor_syndrome_real(
        disks: i32,
        start: i32,
        stop: i32,
        bytes: u64,
        ptrs: *mut *mut core::ffi::c_void,
    );
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct raid6_calls {
    pub gen_syndrome: unsafe fn(i32, usize, *mut *mut core::ffi::c_void),
    pub xor_syndrome: unsafe fn(i32, i32, i32, usize, *mut *mut core::ffi::c_void),
    pub name: &'static [u8],
}

macro_rules! raid6_neon_wrapper {
    ($n:literal, $gen:ident, $xor:ident, $real_gen:ident, $real_xor:ident, $name:ident) => {
        unsafe fn $gen(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) {
            // Corresponds to scoped_ksimd().
            unsafe { $real_gen(disks, bytes as u64, ptrs) }
        }

        unsafe fn $xor(
            disks: i32,
            start: i32,
            stop: i32,
            bytes: usize,
            ptrs: *mut *mut core::ffi::c_void,
        ) {
            // Corresponds to scoped_ksimd().
            unsafe { $real_xor(disks, start, stop, bytes as u64, ptrs) }
        }

        pub static $name: raid6_calls = raid6_calls {
            gen_syndrome: $gen,
            xor_syndrome: $xor,
            name: concat!("neonx", stringify!($n), "\0").as_bytes(),
        };
    };
}

raid6_neon_wrapper!(1, raid6_neon1_gen_syndrome, raid6_neon1_xor_syndrome,
    raid6_neon1_gen_syndrome_real, raid6_neon1_xor_syndrome_real, raid6_neonx1);
raid6_neon_wrapper!(2, raid6_neon2_gen_syndrome, raid6_neon2_xor_syndrome,
    raid6_neon2_gen_syndrome_real, raid6_neon2_xor_syndrome_real, raid6_neonx2);
raid6_neon_wrapper!(4, raid6_neon4_gen_syndrome, raid6_neon4_xor_syndrome,
    raid6_neon4_gen_syndrome_real, raid6_neon4_xor_syndrome_real, raid6_neonx4);
raid6_neon_wrapper!(8, raid6_neon8_gen_syndrome, raid6_neon8_xor_syndrome,
    raid6_neon8_gen_syndrome_real, raid6_neon8_xor_syndrome_real, raid6_neonx8);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
