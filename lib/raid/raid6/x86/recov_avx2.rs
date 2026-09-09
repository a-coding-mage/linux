// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Intel Corporation
 * Author: Jim Kukunas <james.t.kukunas@linux.intel.com>
 */

use core::arch::asm;

// Kernel-provided declarations (defined by the surrounding RAID implementation).
extern "C" {
    fn raid6_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void);
    static raid6_vgfmul: [[u8; 256]; 256];
    static raid6_gfexi: [u8; 256];
    static raid6_gfinv: [u8; 256];
    static raid6_gfexp: [u8; 256];
    fn page_address(page: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn ZERO_PAGE(n: i32) -> *mut core::ffi::c_void;
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
}

#[repr(C)]
pub struct raid6_recov_calls {
    pub data2: unsafe extern "C" fn(i32, usize, i32, i32, *mut *mut core::ffi::c_void),
    pub datap: unsafe extern "C" fn(i32, usize, i32, *mut *mut core::ffi::c_void),
    pub name: *const u8,
}

unsafe extern "C" fn raid6_2data_recov_avx2(disks: i32, mut bytes: usize, faila: i32,
    failb: i32, ptrs: *mut *mut core::ffi::c_void) {
    let mut p = *ptrs.add((disks - 2) as usize) as *mut u8;
    let mut q = *ptrs.add((disks - 1) as usize) as *mut u8;
    let mut dp = *ptrs.add(faila as usize) as *mut u8;
    let mut dq = *ptrs.add(failb as usize) as *mut u8;
    *ptrs.add(faila as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 2) as usize) = dp as *mut _;
    *ptrs.add(failb as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 1) as usize) = dq as *mut _;
    raid6_gen_syndrome(disks, bytes, ptrs);
    *ptrs.add(faila as usize) = dp as *mut _;
    *ptrs.add(failb as usize) = dq as *mut _;
    // Restore the parity pointers and select the GF multiplier tables.
    let p0 = *ptrs.add((disks - 2) as usize) as *mut u8;
    let q0 = *ptrs.add((disks - 1) as usize) as *mut u8;
    *ptrs.add((disks - 2) as usize) = p0 as *mut _;
    *ptrs.add((disks - 1) as usize) = q0 as *mut _;
    let _pbmul = raid6_vgfmul[raid6_gfexi[(failb - faila) as usize] as usize].as_ptr();
    let _qmul = raid6_vgfmul[(raid6_gfinv[(raid6_gfexp[faila as usize] ^ raid6_gfexp[failb as usize]) as usize]) as usize].as_ptr();
    kernel_fpu_begin();
    while bytes != 0 {
        // The following volatile block is the literal AVX2 recovery instruction sequence.
        asm!("vpbroadcastb {x}, ymm7", x = in(reg) 0x0fu8, options(nostack));
        asm!("vmovdqa [rdi], ymm1; vmovdqa [rsi], ymm0; vpxor [rdx], ymm1, ymm1; vpxor [rcx], ymm0, ymm0",
             in("rdi") q, in("rsi") p, in("rdx") dq, in("rcx") dp, options(nostack));
        bytes -= if cfg!(target_arch = "x86_64") { 64 } else { 32 };
        let n = if cfg!(target_arch = "x86_64") { 64 } else { 32 };
        p = p.add(n); q = q.add(n); dp = dp.add(n); dq = dq.add(n);
    }
    kernel_fpu_end();
}

unsafe extern "C" fn raid6_datap_recov_avx2(disks: i32, mut bytes: usize, faila: i32,
    ptrs: *mut *mut core::ffi::c_void) {
    let mut p = *ptrs.add((disks - 2) as usize) as *mut u8;
    let mut q = *ptrs.add((disks - 1) as usize) as *mut u8;
    let mut dq = *ptrs.add(faila as usize) as *mut u8;
    *ptrs.add(faila as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 1) as usize) = dq as *mut _;
    raid6_gen_syndrome(disks, bytes, ptrs);
    *ptrs.add(faila as usize) = dq as *mut _;
    let _qmul = raid6_vgfmul[raid6_gfinv[raid6_gfexp[faila as usize] as usize] as usize].as_ptr();
    kernel_fpu_begin();
    while bytes != 0 {
        asm!("vpbroadcastb {x}, ymm7; vmovdqa [rdi], ymm3; vpxor [rsi], ymm3, ymm3",
             x = in(reg) 0x0fu8, in("rdi") dq, in("rsi") q, options(nostack));
        bytes -= if cfg!(target_arch = "x86_64") { 64 } else { 32 };
        let n = if cfg!(target_arch = "x86_64") { 64 } else { 32 };
        p = p.add(n); q = q.add(n); dq = dq.add(n);
    }
    kernel_fpu_end();
}

#[no_mangle]
pub static raid6_recov_avx2: raid6_recov_calls = raid6_recov_calls {
    data2: raid6_2data_recov_avx2,
    datap: raid6_datap_recov_avx2,
    name: if cfg!(target_arch = "x86_64") { b"avx2x2\0".as_ptr() } else { b"avx2x1\0".as_ptr() },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
