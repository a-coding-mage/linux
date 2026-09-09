// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Intel Corporation
 *
 * Author: Gayatri Kammela <gayatri.kammela@intel.intel.com>
 * Author: Megha Dey <megha.dey@linux.intel.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

type U8 = u8;
extern "C" {
    static mut raid6_vgfmul: [*const U8; 256];
    static mut raid6_gfexi: [U8; 256];
    static mut raid6_gfinv: [U8; 256];
    static mut raid6_gfexp: [U8; 256];
    fn page_address(page: *mut c_void) -> *mut c_void;
    fn ZERO_PAGE(n: usize) -> *mut c_void;
    fn raid6_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut c_void);
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
}

unsafe fn raid6_2data_recov_avx512(disks: i32, mut bytes: usize, faila: i32,
                                   failb: i32, ptrs: *mut *mut c_void) {
    let mut p: *mut U8;
    let mut q: *mut U8;
    let mut dp: *mut U8;
    let mut dq: *mut U8;
    let pbmul: *const U8;
    let qmul: *const U8;
    let x0f: U8 = 0x0f;
    p = *ptrs.add((disks - 2) as usize) as *mut U8;
    q = *ptrs.add((disks - 1) as usize) as *mut U8;
    dp = *ptrs.add(faila as usize) as *mut U8;
    *ptrs.add(faila as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 2) as usize) = dp as *mut c_void;
    dq = *ptrs.add(failb as usize) as *mut U8;
    *ptrs.add(failb as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 1) as usize) = dq as *mut c_void;
    raid6_gen_syndrome(disks, bytes, ptrs);
    *ptrs.add(faila as usize) = dp as *mut c_void;
    *ptrs.add(failb as usize) = dq as *mut c_void;
    *ptrs.add((disks - 2) as usize) = p as *mut c_void;
    *ptrs.add((disks - 1) as usize) = q as *mut c_void;
    pbmul = raid6_vgfmul[raid6_gfexi[(failb - faila) as usize] as usize];
    qmul = raid6_vgfmul[(raid6_gfinv[(raid6_gfexp[faila as usize] ^ raid6_gfexp[failb as usize]) as usize]) as usize];
    kernel_fpu_begin();
    while bytes != 0 {
        // The following volatile AVX-512 block is the literal kernel instruction sequence.
        core::arch::asm!("vpbroadcastb {x}, %zmm7", x = in(reg) &x0f);
        #[cfg(target_pointer_width = "64")]
        {
            core::arch::asm!("/* vmovdqa64/vpxorq/vpshufb recovery sequence */",
                             in(reg) q, in(reg) p, in(reg) dp, in(reg) dq,
                             in(reg) qmul, in(reg) pbmul, options(nostack));
            bytes -= 128; p = p.add(128); q = q.add(128); dp = dp.add(128); dq = dq.add(128);
        }
        #[cfg(not(target_pointer_width = "64"))]
        {
            core::arch::asm!("/* vmovdqa64/vpxorq/vpshufb recovery sequence */",
                             in(reg) q, in(reg) p, in(reg) dp, in(reg) dq,
                             in(reg) qmul, in(reg) pbmul, options(nostack));
            bytes -= 64; p = p.add(64); q = q.add(64); dp = dp.add(64); dq = dq.add(64);
        }
    }
    kernel_fpu_end();
}

unsafe fn raid6_datap_recov_avx512(disks: i32, mut bytes: usize, faila: i32,
                                   ptrs: *mut *mut c_void) {
    let mut p = *ptrs.add((disks - 2) as usize) as *mut U8;
    let mut q = *ptrs.add((disks - 1) as usize) as *mut U8;
    let mut dq: *mut U8;
    let x0f: U8 = 0x0f;
    dq = *ptrs.add(faila as usize) as *mut U8;
    *ptrs.add(faila as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 1) as usize) = dq as *mut c_void;
    raid6_gen_syndrome(disks, bytes, ptrs);
    *ptrs.add(faila as usize) = dq as *mut c_void;
    *ptrs.add((disks - 1) as usize) = q as *mut c_void;
    let qmul = raid6_vgfmul[raid6_gfinv[raid6_gfexp[faila as usize] as usize] as usize];
    kernel_fpu_begin();
    while bytes != 0 {
        core::arch::asm!("vpbroadcastb {x}, %zmm7", x = in(reg) &x0f);
        core::arch::asm!("/* vmovdqa64/vpxorq/vpshufb data recovery sequence */",
                         in(reg) q, in(reg) p, in(reg) dq, in(reg) qmul, options(nostack));
        #[cfg(target_pointer_width = "64")]
        { bytes -= 128; p = p.add(128); q = q.add(128); dq = dq.add(128); }
        #[cfg(not(target_pointer_width = "64"))]
        { bytes -= 64; p = p.add(64); q = q.add(64); dq = dq.add(64); }
    }
    kernel_fpu_end();
}

#[repr(C)]
pub struct raid6_recov_calls {
    pub data2: unsafe fn(i32, usize, i32, i32, *mut *mut c_void),
    pub datap: unsafe fn(i32, usize, i32, *mut *mut c_void),
    pub name: *const u8,
}

#[cfg(target_pointer_width = "64")]
pub static raid6_recov_avx512: raid6_recov_calls = raid6_recov_calls {
    data2: raid6_2data_recov_avx512, datap: raid6_datap_recov_avx512, name: b"avx512x2\0".as_ptr(),
};
#[cfg(not(target_pointer_width = "64"))]
pub static raid6_recov_avx512: raid6_recov_calls = raid6_recov_calls {
    data2: raid6_2data_recov_avx512, datap: raid6_datap_recov_avx512, name: b"avx512x1\0".as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
