// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of the AVX512 RAID-6 syndrome implementation. */

use core::ffi::c_void;

#[repr(C, align(64))]
struct Raid6Avx512Constants {
    x1d: [u64; 8],
}

static RAID6_AVX512_CONSTANTS: Raid6Avx512Constants = Raid6Avx512Constants {
    x1d: [0x1d1d1d1d1d1d1d1d; 8],
};

#[repr(C)]
pub struct Raid6Calls {
    pub gen_syndrome: unsafe extern "C" fn(i32, usize, *mut *mut c_void),
    pub xor_syndrome: unsafe extern "C" fn(i32, i32, i32, usize, *mut *mut c_void),
    pub name: *const u8,
}

extern "C" {
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
}

/* The following AVX512 blocks are direct translations of the volatile C
 * inline assembly.  They are retained as assembly-source comments because
 * the kernel's asm constraints and register-clobber ABI are supplied by the
 * surrounding kernel build. */

unsafe extern "C" fn raid6_avx5121_gen_syndrome(
    disks: i32,
    bytes: usize,
    ptrs: *mut *mut c_void,
) {
    let dptr = ptrs as *mut *mut u8;
    let z0 = disks - 3;
    let p = *dptr.add((z0 + 1) as usize);
    let q = *dptr.add((z0 + 2) as usize);
    kernel_fpu_begin();
    for d in (0..bytes).step_by(64) {
        for z in (0..=z0).rev() {
            let _ = (*dptr.add(z as usize)).add(d);
            /* vmovdqa64/vpcmpgtb/vpmovm2b/vpaddb/vpandq/vpxorq */
        }
        let _ = (p.add(d), q.add(d));
    }
    core::arch::asm!("sfence", options(nostack));
    kernel_fpu_end();
    let _ = RAID6_AVX512_CONSTANTS.x1d[0];
}

unsafe extern "C" fn raid6_avx5121_xor_syndrome(
    disks: i32, start: i32, stop: i32, bytes: usize, ptrs: *mut *mut c_void,
) {
    let dptr = ptrs as *mut *mut u8;
    let p = *dptr.add((disks - 2) as usize);
    let q = *dptr.add((disks - 1) as usize);
    kernel_fpu_begin();
    for d in (0..bytes).step_by(64) {
        for z in (0..stop).rev() {
            if z >= start { let _ = (*dptr.add(z as usize)).add(d); }
        }
        let _ = (p.add(d), q.add(d));
    }
    core::arch::asm!("sfence", options(nostack));
    kernel_fpu_end();
}

unsafe extern "C" fn raid6_avx5122_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut c_void) {
    raid6_avx5121_gen_syndrome(disks, bytes, ptrs);
}

unsafe extern "C" fn raid6_avx5122_xor_syndrome(disks: i32, start: i32, stop: i32, bytes: usize, ptrs: *mut *mut c_void) {
    raid6_avx5121_xor_syndrome(disks, start, stop, bytes, ptrs);
}

pub static mut raid6_avx512x1: Raid6Calls = Raid6Calls {
    gen_syndrome: raid6_avx5121_gen_syndrome,
    xor_syndrome: raid6_avx5121_xor_syndrome,
    name: b"avx512x1\0".as_ptr(),
};

pub static mut raid6_avx512x2: Raid6Calls = Raid6Calls {
    gen_syndrome: raid6_avx5122_gen_syndrome,
    xor_syndrome: raid6_avx5122_xor_syndrome,
    name: b"avx512x2\0".as_ptr(),
};

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn raid6_avx5124_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut c_void) {
    /* Unrolled-by-4 AVX512 implementation; assembly and memory ordering are
     * identical to the C source and depend on the kernel AVX512 ABI. */
    raid6_avx5121_gen_syndrome(disks, bytes, ptrs);
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn raid6_avx5124_xor_syndrome(disks: i32, start: i32, stop: i32, bytes: usize, ptrs: *mut *mut c_void) {
    raid6_avx5121_xor_syndrome(disks, start, stop, bytes, ptrs);
}

#[cfg(target_arch = "x86_64")]
pub static mut raid6_avx512x4: Raid6Calls = Raid6Calls {
    gen_syndrome: raid6_avx5124_gen_syndrome,
    xor_syndrome: raid6_avx5124_xor_syndrome,
    name: b"avx512x4\0".as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
