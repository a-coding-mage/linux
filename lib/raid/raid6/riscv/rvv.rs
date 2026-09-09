// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RAID-6 syndrome calculation using RISC-V vector instructions
 *
 * Rust translation of rvv.c.  The original vector instructions operate on
 * byte lanes; the scalar lane below preserves the same byte-level ordering
 * and side effects when the vector instruction interface is unavailable.
 */

type U8 = u8;

#[inline(always)]
unsafe fn raid6_rvv_lane_gen(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void, lanes: usize) {
    let dptr = ptrs as *mut *mut U8;
    let z0 = disks - 3;
    let p = *dptr.add((z0 + 1) as usize);
    let q = *dptr.add((z0 + 2) as usize);
    let n = lanes.max(1);
    let mut d = 0usize;
    while d < bytes {
        let count = (bytes - d).min(n);
        for i in 0..count {
            let mut wq = *(*dptr.add(z0 as usize)).add(d + i);
            let mut wp = wq;
            let mut z = z0 - 1;
            while z >= 0 {
                let w2 = ((wq >> 7) & 0x1d) as u8;
                let w1 = (wq << 1) ^ w2;
                let wd = *(*dptr.add(z as usize)).add(d + i);
                wq = w1 ^ wd;
                wp ^= wd;
                z -= 1;
            }
            *p.add(d + i) = wp;
            *q.add(d + i) = wq;
        }
        d += count;
    }
}

#[inline(always)]
unsafe fn raid6_rvv_lane_xor(disks: i32, start: i32, stop: i32, bytes: usize,
                             ptrs: *mut *mut core::ffi::c_void, lanes: usize) {
    let dptr = ptrs as *mut *mut U8;
    let p = *dptr.add((disks - 2) as usize);
    let q = *dptr.add((disks - 1) as usize);
    let n = lanes.max(1);
    let mut d = 0usize;
    while d < bytes {
        let count = (bytes - d).min(n);
        for i in 0..count {
            let mut wq = *(*dptr.add(stop as usize)).add(d + i);
            let mut wp = wq;
            let mut z = stop - 1;
            while z >= start {
                let w2 = ((wq >> 7) & 0x1d) as u8;
                let w1 = (wq << 1) ^ w2;
                let wd = *(*dptr.add(z as usize)).add(d + i);
                wq = w1 ^ wd;
                wp ^= wd;
                z -= 1;
            }
            let mut z = start - 1;
            while z >= 0 {
                let w2 = ((wq >> 7) & 0x1d) as u8;
                wq = (wq << 1) ^ w2;
                z -= 1;
            }
            *p.add(d + i) ^= wp;
            *q.add(d + i) ^= wq;
        }
        d += count;
    }
}

macro_rules! raid6_rvv_wrapper {
    ($name:ident, $lanes:expr) => {
        #[allow(non_snake_case)]
        unsafe fn $name(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) {
            raid6_rvv_lane_gen(disks, bytes, ptrs, $lanes);
        }
        #[allow(non_snake_case)]
        unsafe fn concat_idents_xor_syndrome(disks: i32, start: i32, stop: i32, bytes: usize,
                                             ptrs: *mut *mut core::ffi::c_void) {
            raid6_rvv_lane_xor(disks, start, stop, bytes, ptrs, $lanes);
        }
    };
}

// The C build selects these implementations through RAID6_RVV_WRAPPER(n).
// Keep the concrete implementation entry points and their source-level
// vector widths available to the surrounding translation unit.
unsafe fn raid6_rvv1_gen_syndrome_real(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) { raid6_rvv_lane_gen(disks, bytes, ptrs, 1); }
unsafe fn raid6_rvv1_xor_syndrome_real(disks: i32, start: i32, stop: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) { raid6_rvv_lane_xor(disks, start, stop, bytes, ptrs, 1); }
unsafe fn raid6_rvv2_gen_syndrome_real(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) { raid6_rvv_lane_gen(disks, bytes, ptrs, 2); }
unsafe fn raid6_rvv2_xor_syndrome_real(disks: i32, start: i32, stop: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) { raid6_rvv_lane_xor(disks, start, stop, bytes, ptrs, 2); }
unsafe fn raid6_rvv4_gen_syndrome_real(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) { raid6_rvv_lane_gen(disks, bytes, ptrs, 4); }
unsafe fn raid6_rvv4_xor_syndrome_real(disks: i32, start: i32, stop: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) { raid6_rvv_lane_xor(disks, start, stop, bytes, ptrs, 4); }
unsafe fn raid6_rvv8_gen_syndrome_real(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) { raid6_rvv_lane_gen(disks, bytes, ptrs, 8); }
unsafe fn raid6_rvv8_xor_syndrome_real(disks: i32, start: i32, stop: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) { raid6_rvv_lane_xor(disks, start, stop, bytes, ptrs, 8); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
