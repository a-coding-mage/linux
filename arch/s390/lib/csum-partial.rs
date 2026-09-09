// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

// Types and operations supplied by the kernel headers.
#[allow(non_camel_case_types)]
type __wsum = u32;

unsafe extern "C" {
    fn cpu_has_vx() -> bool;
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize);
    fn cksm(src: *const c_void, len: i32, sum: __wsum) -> __wsum;
    fn kernel_fpu_begin(vxstate: *mut c_void, mask: u32);
    fn kernel_fpu_end(vxstate: *mut c_void, mask: u32);
    fn fpu_vlvgf(reg: i32, value: u32, index: i32);
    fn fpu_vzero(reg: i32);
    fn fpu_vlm(first: i32, last: i32, src: *const c_void);
    fn fpu_vstm(first: i32, last: i32, dst: *mut c_void);
    fn fpu_vcksm(dst: i32, lhs: i32, rhs: i32);
    fn fpu_vl(reg: i32, src: *const c_void);
    fn fpu_vst(reg: i32, dst: *mut c_void);
    fn fpu_vll(reg: i32, length: i32, src: *const c_void);
    fn fpu_vstl(reg: i32, length: i32, dst: *mut c_void);
    fn fpu_vlgvf(reg: i32, index: i32) -> u32;
}

const KERNEL_VXR_V16V23: u32 = 0;

/*
 * Computes the checksum of a memory block at src, length len,
 * and adds in "sum" (32-bit). If copy is true copies to dst.
 *
 * Returns a 32-bit number suitable for feeding into itself
 * or csum_tcpudp_magic.
 *
 * This function must be called with even lengths, except
 * for the last fragment, which may be odd.
 *
 * It's best to have src and dst aligned on a 64-bit boundary.
 */
#[inline(always)]
unsafe fn csum_copy(
    mut dst: *mut c_void,
    mut src: *const c_void,
    mut len: i32,
    mut sum: __wsum,
    copy: bool,
) -> __wsum {
    // DECLARE_KERNEL_FPU_ONSTACK8(vxstate);
    let mut vxstate = core::mem::MaybeUninit::<[u8; 0]>::uninit();

    if !cpu_has_vx() {
        if copy {
            memcpy(dst, src, len as usize);
        }
        return cksm(src, len, sum);
    }
    kernel_fpu_begin(vxstate.as_mut_ptr() as *mut c_void, KERNEL_VXR_V16V23);
    fpu_vlvgf(16, sum, 1);
    fpu_vzero(17);
    fpu_vzero(18);
    fpu_vzero(19);
    while len >= 64 {
        fpu_vlm(20, 23, src);
        if copy {
            fpu_vstm(20, 23, dst);
            dst = dst.add(64);
        }
        fpu_vcksm(16, 20, 16);
        fpu_vcksm(17, 21, 17);
        fpu_vcksm(18, 22, 18);
        fpu_vcksm(19, 23, 19);
        src = src.add(64);
        len -= 64;
    }
    while len >= 32 {
        fpu_vlm(20, 21, src);
        if copy {
            fpu_vstm(20, 21, dst);
            dst = dst.add(32);
        }
        fpu_vcksm(16, 20, 16);
        fpu_vcksm(17, 21, 17);
        src = src.add(32);
        len -= 32;
    }
    while len >= 16 {
        fpu_vl(20, src);
        if copy {
            fpu_vst(20, dst);
            dst = dst.add(16);
        }
        fpu_vcksm(16, 20, 16);
        src = src.add(16);
        len -= 16;
    }
    if len != 0 {
        fpu_vll(20, len - 1, src);
        if copy {
            fpu_vstl(20, len - 1, dst);
        }
        fpu_vcksm(16, 20, 16);
    }
    fpu_vcksm(18, 19, 18);
    fpu_vcksm(16, 17, 16);
    fpu_vcksm(16, 18, 16);
    sum = fpu_vlgvf(16, 1);
    kernel_fpu_end(vxstate.as_mut_ptr() as *mut c_void, KERNEL_VXR_V16V23);
    sum
}

pub unsafe fn csum_partial(buff: *const c_void, len: i32, sum: __wsum) -> __wsum {
    csum_copy(core::ptr::null_mut(), buff, len, sum, false)
}

// EXPORT_SYMBOL(csum_partial);

pub unsafe fn csum_partial_copy_nocheck(
    src: *const c_void,
    dst: *mut c_void,
    len: i32,
) -> __wsum {
    csum_copy(dst, src, len, 0, true)
}

// EXPORT_SYMBOL(csum_partial_copy_nocheck);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
