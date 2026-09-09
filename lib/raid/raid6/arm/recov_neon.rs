// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Intel Corporation
 * Copyright (C) 2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
 */

use core::ffi::c_void;

// Types, tables, and functions supplied by the surrounding RAID/NEON code.
type U8 = u8;

#[repr(C)]
pub struct raid6_recov_calls {
    pub data2: Option<unsafe extern "C" fn(i32, usize, i32, i32, *mut *mut c_void)>,
    pub datap: Option<unsafe extern "C" fn(i32, usize, i32, *mut *mut c_void)>,
    pub name: *const u8,
}

extern "C" {
    static raid6_vgfmul: [*const U8; 256];
    static raid6_gfexi: [u8; 256];
    static raid6_gfinv: [u8; 256];
    static raid6_gfexp: [u8; 256];

    fn raid6_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut c_void);
    fn page_address(page: *mut c_void) -> *mut c_void;
    fn ZERO_PAGE(offset: usize) -> *mut c_void;
    fn __raid6_2data_recov_neon(
        bytes: usize,
        p: *mut U8,
        q: *mut U8,
        dp: *mut U8,
        dq: *mut U8,
        pbmul: *const U8,
        qmul: *const U8,
    );
    fn __raid6_datap_recov_neon(
        bytes: usize,
        p: *mut U8,
        q: *mut U8,
        dq: *mut U8,
        qmul: *const U8,
    );
}

unsafe extern "C" fn raid6_2data_recov_neon(
    disks: i32,
    bytes: usize,
    faila: i32,
    failb: i32,
    ptrs: *mut *mut c_void,
) {
    let p: *mut U8 = *ptrs.add((disks - 2) as usize) as *mut U8;
    let q: *mut U8 = *ptrs.add((disks - 1) as usize) as *mut U8;

    // Compute syndrome with zero for the missing data pages.
    // Use the dead data pages as temporary storage for delta p and delta q.
    let dp: *mut U8 = *ptrs.add(faila as usize) as *mut U8;
    *ptrs.add(faila as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 2) as usize) = dp as *mut c_void;
    let dq: *mut U8 = *ptrs.add(failb as usize) as *mut U8;
    *ptrs.add(failb as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 1) as usize) = dq as *mut c_void;

    raid6_gen_syndrome(disks, bytes, ptrs);

    // Restore pointer table.
    *ptrs.add(faila as usize) = dp as *mut c_void;
    *ptrs.add(failb as usize) = dq as *mut c_void;
    *ptrs.add((disks - 2) as usize) = p as *mut c_void;
    *ptrs.add((disks - 1) as usize) = q as *mut c_void;

    // Now, pick the proper data tables.
    let pbmul = raid6_vgfmul[raid6_gfexi[(failb - faila) as usize] as usize];
    let qmul = raid6_vgfmul[
        raid6_gfinv[(raid6_gfexp[faila as usize] ^ raid6_gfexp[failb as usize]) as usize]
            as usize
    ];

    // scoped_ksimd() enables the architecture SIMD context in the C source.
    __raid6_2data_recov_neon(bytes, p, q, dp, dq, pbmul, qmul);
}

unsafe extern "C" fn raid6_datap_recov_neon(
    disks: i32,
    bytes: usize,
    faila: i32,
    ptrs: *mut *mut c_void,
) {
    let p: *mut U8 = *ptrs.add((disks - 2) as usize) as *mut U8;
    let q: *mut U8 = *ptrs.add((disks - 1) as usize) as *mut U8;

    // Compute syndrome with zero for the missing data page.
    // Use the dead data page as temporary storage for delta q.
    let dq: *mut U8 = *ptrs.add(faila as usize) as *mut U8;
    *ptrs.add(faila as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 1) as usize) = dq as *mut c_void;

    raid6_gen_syndrome(disks, bytes, ptrs);

    // Restore pointer table.
    *ptrs.add(faila as usize) = dq as *mut c_void;
    *ptrs.add((disks - 1) as usize) = q as *mut c_void;

    // Now, pick the proper data tables.
    let qmul = raid6_vgfmul[raid6_gfinv[raid6_gfexp[faila as usize] as usize] as usize];

    // scoped_ksimd() enables the architecture SIMD context in the C source.
    __raid6_datap_recov_neon(bytes, p, q, dq, qmul);
}

#[no_mangle]
pub static raid6_recov_neon: raid6_recov_calls = raid6_recov_calls {
    data2: Some(raid6_2data_recov_neon),
    datap: Some(raid6_datap_recov_neon),
    name: b"neon\0".as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
