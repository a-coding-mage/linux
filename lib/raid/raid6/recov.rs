// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2002 H. Peter Anvin - All Rights Reserved
 *
 * RAID-6 data recovery in dual failure mode.  In single failure mode, use the
 * RAID-5 algorithm (or, in the case of Q failure, just reconstruct the
 * syndrome.)
 */

use core::ffi::c_void;

extern "C" {
    static raid6_gfmul: [*const u8; 256];
    static raid6_gfexi: [u8; 256];
    static raid6_gfinv: [u8; 256];
    static raid6_gfexp: [u8; 256];

    fn raid6_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut c_void);
    fn page_address(page: *mut c_void) -> *mut c_void;
    fn zero_page(index: usize) -> *mut c_void;
}

/* Recover two failed data blocks. */
unsafe extern "C" fn raid6_2data_recov_intx1(
    disks: i32,
    mut bytes: usize,
    faila: i32,
    failb: i32,
    ptrs: *mut *mut c_void,
) {
    let mut p: *mut u8;
    let mut q: *mut u8;
    let mut dp: *mut u8;
    let mut dq: *mut u8;
    let mut px: u8;
    let mut qx: u8;
    let mut db: u8;
    let pbmul: *const u8;
    let qmul: *const u8;

    p = *ptrs.add((disks - 2) as usize) as *mut u8;
    q = *ptrs.add((disks - 1) as usize) as *mut u8;

    /* Compute syndrome with zero for the missing data pages
       Use the dead data pages as temporary storage for delta p and delta q */
    dp = *ptrs.add(faila as usize) as *mut u8;
    *ptrs.add(faila as usize) = page_address(zero_page(0));
    *ptrs.add((disks - 2) as usize) = dp as *mut c_void;
    dq = *ptrs.add(failb as usize) as *mut u8;
    *ptrs.add(failb as usize) = page_address(zero_page(0));
    *ptrs.add((disks - 1) as usize) = dq as *mut c_void;

    raid6_gen_syndrome(disks, bytes, ptrs);

    /* Restore pointer table */
    *ptrs.add(faila as usize) = dp as *mut c_void;
    *ptrs.add(failb as usize) = dq as *mut c_void;
    *ptrs.add((disks - 2) as usize) = p as *mut c_void;
    *ptrs.add((disks - 1) as usize) = q as *mut c_void;

    /* Now, pick the proper data tables */
    pbmul = raid6_gfmul[raid6_gfexi[(failb - faila) as usize] as usize];
    qmul = raid6_gfmul[
        raid6_gfinv[(raid6_gfexp[faila as usize] ^ raid6_gfexp[failb as usize]) as usize]
            as usize
    ];

    /* Now do it... */
    while bytes != 0 {
        px = *p ^ *dp;
        qx = *qmul.add((*q ^ *dq) as usize);
        db = *pbmul.add(px as usize) ^ qx;
        *dq = db;
        dq = dq.add(1);
        *dp = db ^ px;
        dp = dp.add(1);
        p = p.add(1);
        q = q.add(1);
        bytes -= 1;
    }
}

/* Recover failure of one data block plus the P block */
unsafe extern "C" fn raid6_datap_recov_intx1(
    disks: i32,
    mut bytes: usize,
    faila: i32,
    ptrs: *mut *mut c_void,
) {
    let mut p: *mut u8;
    let mut q: *mut u8;
    let mut dq: *mut u8;
    let qmul: *const u8;

    p = *ptrs.add((disks - 2) as usize) as *mut u8;
    q = *ptrs.add((disks - 1) as usize) as *mut u8;

    /* Compute syndrome with zero for the missing data page
       Use the dead data page as temporary storage for delta q */
    dq = *ptrs.add(faila as usize) as *mut u8;
    *ptrs.add(faila as usize) = page_address(zero_page(0));
    *ptrs.add((disks - 1) as usize) = dq as *mut c_void;

    raid6_gen_syndrome(disks, bytes, ptrs);

    /* Restore pointer table */
    *ptrs.add(faila as usize) = dq as *mut c_void;
    *ptrs.add((disks - 1) as usize) = q as *mut c_void;

    /* Now, pick the proper data tables */
    qmul = raid6_gfmul[raid6_gfinv[raid6_gfexp[faila as usize] as usize] as usize];

    /* Now do it... */
    while bytes != 0 {
        *p ^= *qmul.add((*q ^ *dq) as usize);
        q = q.add(1);
        dq = dq.add(1);
        p = p.add(1);
        bytes -= 1;
    }
}

#[repr(C)]
pub struct raid6_recov_calls {
    pub data2: Option<unsafe extern "C" fn(i32, usize, i32, i32, *mut *mut c_void)>,
    pub datap: Option<unsafe extern "C" fn(i32, usize, i32, *mut *mut c_void)>,
    pub name: *const u8,
}

#[no_mangle]
pub static raid6_recov_intx1: raid6_recov_calls = raid6_recov_calls {
    data2: Some(raid6_2data_recov_intx1),
    datap: Some(raid6_datap_recov_intx1),
    name: b"intx1\0".as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
