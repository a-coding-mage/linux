// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2024 Institute of Software, CAS.
 * Author: Chunyan Zhang <zhangchunyan@iscas.ac.cn>
 */

/* Translated from the Linux kernel RVV implementation. */

unsafe fn __raid6_2data_recov_rvv(
    mut bytes: usize,
    mut p: *mut u8,
    mut q: *mut u8,
    mut dp: *mut u8,
    mut dq: *mut u8,
    pbmul: *const u8,
    qmul: *const u8,
) {
    // The original performs: vsetvli x0, 16, e8, m1, ta, ma.
    while bytes != 0 {
        /*
         * RVV operations, equivalent to the original inline assembly:
         *
         * px = *p ^ *dp;
         * qx = qmul[*q ^ *dq];
         * *dq++ = db = pbmul[px] ^ qx;
         * *dp++ = db ^ px;
         *
         * The vector implementation operates on sixteen bytes at a time.
         */
        let mut i = 0usize;
        while i < 16 {
            let px = unsafe { *p.add(i) ^ *dp.add(i) };
            let qx = unsafe { *qmul.add((*q.add(i) ^ *dq.add(i)) as usize) };
            let db = unsafe { *pbmul.add(px as usize) ^ qx };
            unsafe {
                *dq.add(i) = db;
                *dp.add(i) = db ^ px;
            }
            i += 1;
        }
        bytes -= 16;
        p = unsafe { p.add(16) };
        q = unsafe { q.add(16) };
        dp = unsafe { dp.add(16) };
        dq = unsafe { dq.add(16) };
    }
}

unsafe fn __raid6_datap_recov_rvv(
    mut bytes: usize,
    mut p: *mut u8,
    mut q: *mut u8,
    mut dq: *mut u8,
    qmul: *const u8,
) {
    // The original performs: vsetvli x0, 16, e8, m1, ta, ma.
    while bytes != 0 {
        /*
         * RVV operations, equivalent to the original inline assembly:
         * *p++ ^= *dq = qmul[*q ^ *dq];
         */
        let mut i = 0usize;
        while i < 16 {
            let value = unsafe { *qmul.add((*q.add(i) ^ *dq.add(i)) as usize) };
            unsafe {
                *dq.add(i) = value;
                *p.add(i) ^= value;
            }
            i += 1;
        }
        bytes -= 16;
        p = unsafe { p.add(16) };
        q = unsafe { q.add(16) };
        dq = unsafe { dq.add(16) };
    }
}

unsafe fn raid6_2data_recov_rvv(
    disks: i32,
    bytes: usize,
    faila: i32,
    failb: i32,
    ptrs: *mut *mut core::ffi::c_void,
) {
    let p = unsafe { *ptrs.add((disks - 2) as usize) as *mut u8 };
    let q = unsafe { *ptrs.add((disks - 1) as usize) as *mut u8 };

    /* Compute syndrome with zero for the missing data pages. */
    let dp = unsafe { *ptrs.add(faila as usize) as *mut u8 };
    unsafe {
        *ptrs.add(faila as usize) = page_address(ZERO_PAGE(0));
        *ptrs.add((disks - 2) as usize) = dp as *mut core::ffi::c_void;
    }
    let dq = unsafe { *ptrs.add(failb as usize) as *mut u8 };
    unsafe {
        *ptrs.add(failb as usize) = page_address(ZERO_PAGE(0));
        *ptrs.add((disks - 1) as usize) = dq as *mut core::ffi::c_void;
        raid6_gen_syndrome(disks, bytes, ptrs);
        *ptrs.add(faila as usize) = dp as *mut core::ffi::c_void;
        *ptrs.add(failb as usize) = dq as *mut core::ffi::c_void;
        *ptrs.add((disks - 2) as usize) = p as *mut core::ffi::c_void;
        *ptrs.add((disks - 1) as usize) = q as *mut core::ffi::c_void;
    }

    let pbmul = unsafe { raid6_vgfmul[raid6_gfexi[(failb - faila) as usize] as usize].as_ptr() };
    let qmul = unsafe {
        raid6_vgfmul[raid6_gfinv[(raid6_gfexp[faila as usize] ^ raid6_gfexp[failb as usize]) as usize] as usize].as_ptr()
    };
    unsafe {
        kernel_vector_begin();
        __raid6_2data_recov_rvv(bytes, p, q, dp, dq, pbmul, qmul);
        kernel_vector_end();
    }
}

unsafe fn raid6_datap_recov_rvv(
    disks: i32,
    bytes: usize,
    faila: i32,
    ptrs: *mut *mut core::ffi::c_void,
) {
    let p = unsafe { *ptrs.add((disks - 2) as usize) as *mut u8 };
    let q = unsafe { *ptrs.add((disks - 1) as usize) as *mut u8 };
    let dq = unsafe { *ptrs.add(faila as usize) as *mut u8 };
    unsafe {
        *ptrs.add(faila as usize) = page_address(ZERO_PAGE(0));
        *ptrs.add((disks - 1) as usize) = dq as *mut core::ffi::c_void;
        raid6_gen_syndrome(disks, bytes, ptrs);
        *ptrs.add(faila as usize) = dq as *mut core::ffi::c_void;
        *ptrs.add((disks - 1) as usize) = q as *mut core::ffi::c_void;
    }

    let qmul = unsafe { raid6_vgfmul[raid6_gfinv[raid6_gfexp[faila as usize] as usize] as usize].as_ptr() };
    unsafe {
        kernel_vector_begin();
        __raid6_datap_recov_rvv(bytes, p, q, dq, qmul);
        kernel_vector_end();
    }
}

#[repr(C)]
pub struct raid6_recov_calls {
    pub data2: unsafe fn(i32, usize, i32, i32, *mut *mut core::ffi::c_void),
    pub datap: unsafe fn(i32, usize, i32, *mut *mut core::ffi::c_void),
    pub name: *const u8,
}

pub static raid6_recov_rvv: raid6_recov_calls = raid6_recov_calls {
    data2: raid6_2data_recov_rvv,
    datap: raid6_datap_recov_rvv,
    name: b"rvv\0".as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
