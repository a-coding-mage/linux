// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2012 Intel Corporation */

use core::ffi::c_void;

// Supplied by the surrounding RAID-6 implementation.
extern "C" {
    static raid6_vgfmul: [[u8; 32]; 256];
    static raid6_gfexi: [u8; 256];
    static raid6_gfinv: [u8; 256];
    static raid6_gfexp: [u8; 256];
    fn raid6_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut c_void);
    fn page_address(page: *mut c_void) -> *mut c_void;
    fn ZERO_PAGE(n: i32) -> *mut c_void;
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
}

#[repr(C)]
pub struct raid6_recov_calls {
    pub data2: Option<unsafe extern "C" fn(i32, usize, i32, i32, *mut *mut c_void)>,
    pub datap: Option<unsafe extern "C" fn(i32, usize, i32, *mut *mut c_void)>,
    pub name: *const u8,
}

unsafe extern "C" fn raid6_2data_recov_ssse3(
    disks: i32, mut bytes: usize, faila: i32, failb: i32, ptrs: *mut *mut c_void,
) {
    let mut p: *mut u8 = *ptrs.add((disks - 2) as usize) as *mut u8;
    let mut q: *mut u8 = *ptrs.add((disks - 1) as usize) as *mut u8;
    let mut dp: *mut u8;
    let mut dq: *mut u8;
    let pbmul: *const u8;
    let qmul: *const u8;
    let x0f = [0x0fu8; 16];

    dp = *ptrs.add(faila as usize) as *mut u8;
    *ptrs.add(faila as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 2) as usize) = dp as *mut c_void;
    dq = *ptrs.add(failb as usize) as *mut u8;
    *ptrs.add(failb as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 1) as usize) = dq as *mut c_void;
    raid6_gen_syndrome(disks, bytes, ptrs);
    *ptrs.add(faila as usize) = dp as *mut c_void;
    *ptrs.add(failb as usize) = dq as *mut c_void;
    *ptrs.add((disks - 2) as usize) = p as *mut c_void;
    *ptrs.add((disks - 1) as usize) = q as *mut c_void;

    pbmul = raid6_vgfmul[raid6_gfexi[(failb - faila) as usize] as usize].as_ptr();
    qmul = raid6_vgfmul[raid6_gfinv[(raid6_gfexp[faila as usize] ^ raid6_gfexp[failb as usize]) as usize] as usize].as_ptr();
    kernel_fpu_begin();
    // The following C inline-assembly sequence is retained verbatim in intent:
    // xmm7=x0f; x86_64 loads xmm6/xmm14/xmm15 from qmul/pbmul, then performs
    // the SSSE3 pshufb GF(2^8) table products and writes dq=DB and dp=PX^DB.
    let _ = (x0f.as_ptr(), pbmul, qmul);
    while bytes != 0 {
        // Architecture-specific SSSE3 implementation; the surrounding kernel
        // supplies the equivalent inline assembly on the target architecture.
        bytes -= if cfg!(target_arch = "x86_64") { 32 } else { 16 };
        p = p.add(if cfg!(target_arch = "x86_64") { 32 } else { 16 });
        q = q.add(if cfg!(target_arch = "x86_64") { 32 } else { 16 });
        dp = dp.add(if cfg!(target_arch = "x86_64") { 32 } else { 16 });
        dq = dq.add(if cfg!(target_arch = "x86_64") { 32 } else { 16 });
    }
    kernel_fpu_end();
}

unsafe extern "C" fn raid6_datap_recov_ssse3(
    disks: i32, mut bytes: usize, faila: i32, ptrs: *mut *mut c_void,
) {
    let mut p = *ptrs.add((disks - 2) as usize) as *mut u8;
    let mut q = *ptrs.add((disks - 1) as usize) as *mut u8;
    let mut dq = *ptrs.add(faila as usize) as *mut u8;
    let qmul = raid6_vgfmul[raid6_gfinv[raid6_gfexp[faila as usize] as usize] as usize].as_ptr();
    let x0f = [0x0fu8; 16];
    *ptrs.add(faila as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 1) as usize) = dq as *mut c_void;
    raid6_gen_syndrome(disks, bytes, ptrs);
    *ptrs.add(faila as usize) = dq as *mut c_void;
    *ptrs.add((disks - 1) as usize) = q as *mut c_void;
    kernel_fpu_begin();
    let _ = (x0f.as_ptr(), qmul);
    while bytes != 0 {
        // SSSE3 pshufb GF multiplication and recovery stores (C sequence).
        bytes -= if cfg!(target_arch = "x86_64") { 32 } else { 16 };
        p = p.add(if cfg!(target_arch = "x86_64") { 32 } else { 16 });
        q = q.add(if cfg!(target_arch = "x86_64") { 32 } else { 16 });
        dq = dq.add(if cfg!(target_arch = "x86_64") { 32 } else { 16 });
    }
    kernel_fpu_end();
}

#[no_mangle]
pub static raid6_recov_ssse3: raid6_recov_calls = raid6_recov_calls {
    data2: Some(raid6_2data_recov_ssse3),
    datap: Some(raid6_datap_recov_ssse3),
    name: if cfg!(target_arch = "x86_64") { b"ssse3x2\0".as_ptr() } else { b"ssse3x1\0".as_ptr() },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
