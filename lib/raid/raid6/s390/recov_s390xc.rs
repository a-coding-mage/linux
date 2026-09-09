// SPDX-License-Identifier: GPL-2.0
/*
 * RAID-6 data recovery in dual failure mode based on the XC instruction.
 *
 * Copyright IBM Corp. 2016
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

use core::ffi::c_void;

type U8 = u8;

#[repr(C)]
pub struct raid6_recov_calls {
    pub data2: Option<unsafe extern "C" fn(i32, usize, i32, i32, *mut *mut c_void)>,
    pub datap: Option<unsafe extern "C" fn(i32, usize, i32, *mut *mut c_void)>,
    pub name: *const u8,
}

extern "C" {
    static raid6_gfmul: [[U8; 256]; 256];
    static raid6_gfexi: [U8; 256];
    static raid6_gfinv: [U8; 256];
    static raid6_gfexp: [U8; 256];

    fn raid6_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut c_void);
    fn ZERO_PAGE(index: usize) -> *mut c_void;
    fn page_address(page: *mut c_void) -> *mut c_void;
}

unsafe fn xor_block(p1: *mut U8, p2: *const U8) {
    // The s390 XC instruction atomically XORs 256 bytes from p2 into p1.
    for i in 0..256usize {
        let a = core::ptr::read_volatile(p1.add(i));
        let b = core::ptr::read_volatile(p2.add(i));
        core::ptr::write_volatile(p1.add(i), a ^ b);
    }
}

/* Recover two failed data blocks. */
unsafe extern "C" fn raid6_2data_recov_s390xc(
    disks: i32,
    mut bytes: usize,
    faila: i32,
    failb: i32,
    ptrs: *mut *mut c_void,
) {
    let mut p: *mut U8 = *ptrs.add((disks - 2) as usize) as *mut U8;
    let mut q: *mut U8 = *ptrs.add((disks - 1) as usize) as *mut U8;
    let pbmul: *const U8;
    let qmul: *const U8;
    let mut dp: *mut U8;
    let mut dq: *mut U8;

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

    pbmul = raid6_gfmul[raid6_gfexi[(failb - faila) as usize] as usize].as_ptr();
    qmul = raid6_gfmul[
        raid6_gfinv[(raid6_gfexp[faila as usize] ^ raid6_gfexp[failb as usize]) as usize]
            as usize
    ]
    .as_ptr();

    while bytes != 0 {
        xor_block(dp, p);
        xor_block(dq, q);
        for i in 0..256usize {
            *dq.add(i) = *pbmul.add(*dp.add(i) as usize) ^ *qmul.add(*dq.add(i) as usize);
        }
        xor_block(dp, dq);
        p = p.add(256);
        q = q.add(256);
        dp = dp.add(256);
        dq = dq.add(256);
        bytes -= 256;
    }
}

/* Recover failure of one data block plus the P block */
unsafe extern "C" fn raid6_datap_recov_s390xc(
    disks: i32,
    mut bytes: usize,
    faila: i32,
    ptrs: *mut *mut c_void,
) {
    let mut p: *mut U8 = *ptrs.add((disks - 2) as usize) as *mut U8;
    let mut q: *mut U8 = *ptrs.add((disks - 1) as usize) as *mut U8;
    let qmul: *const U8;
    let mut dq: *mut U8;

    dq = *ptrs.add(faila as usize) as *mut U8;
    *ptrs.add(faila as usize) = page_address(ZERO_PAGE(0));
    *ptrs.add((disks - 1) as usize) = dq as *mut c_void;

    raid6_gen_syndrome(disks, bytes, ptrs);

    *ptrs.add(faila as usize) = dq as *mut c_void;
    *ptrs.add((disks - 1) as usize) = q as *mut c_void;

    qmul = raid6_gfmul[raid6_gfinv[raid6_gfexp[faila as usize] as usize] as usize].as_ptr();

    while bytes != 0 {
        xor_block(dq, q);
        for i in 0..256usize {
            *dq.add(i) = *qmul.add(*dq.add(i) as usize);
        }
        xor_block(p, dq);
        p = p.add(256);
        q = q.add(256);
        dq = dq.add(256);
        bytes -= 256;
    }
}

pub static raid6_recov_s390xc: raid6_recov_calls = raid6_recov_calls {
    data2: Some(raid6_2data_recov_s390xc),
    datap: Some(raid6_datap_recov_s390xc),
    name: b"s390xc\0".as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
