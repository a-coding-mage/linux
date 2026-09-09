/* Extracted from GLIBC memcpy.c and memcopy.h, which is:
   Copyright (C) 1991, 1992, 1993, 1997, 2004 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Contributed by Torbjorn Granlund (tege@sics.se).

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */

type OpT = usize;
const OPSIZ: usize = core::mem::size_of::<OpT>();
const OP_T_THRES: usize = 16;

unsafe fn wordcopy_fwd_aligned(mut dstp: usize, mut srcp: usize, mut len: usize) {
    while len > 7 {
        let src = srcp as *const OpT;
        let dst = dstp as *mut OpT;
        let a0 = *src.add(0);
        let a1 = *src.add(1);
        let a2 = *src.add(2);
        let a3 = *src.add(3);
        let a4 = *src.add(4);
        let a5 = *src.add(5);
        let a6 = *src.add(6);
        let a7 = *src.add(7);
        *dst.add(0) = a0;
        *dst.add(1) = a1;
        *dst.add(2) = a2;
        *dst.add(3) = a3;
        *dst.add(4) = a4;
        *dst.add(5) = a5;
        *dst.add(6) = a6;
        *dst.add(7) = a7;
        srcp += 8 * OPSIZ;
        dstp += 8 * OPSIZ;
        len -= 8;
    }
    while len > 0 {
        *(dstp as *mut OpT) = *(srcp as *const OpT);
        srcp += OPSIZ;
        dstp += OPSIZ;
        len -= 1;
    }
}

unsafe fn wordcopy_fwd_dest_aligned(mut dstp: usize, mut srcp: usize, mut len: usize) {
    let sh_1 = 8 * (srcp % OPSIZ);
    let sh_2 = 8 * OPSIZ - sh_1;
    srcp &= ! (OPSIZ - 1);
    let mut ap = *(srcp as *const OpT);
    srcp += OPSIZ;
    while len > 3 {
        let src = srcp as *const OpT;
        let dst = dstp as *mut OpT;
        let a0 = *src.add(0);
        let a1 = *src.add(1);
        let a2 = *src.add(2);
        let a3 = *src.add(3);
        *dst.add(0) = (ap >> sh_1) | (a0 << sh_2);
        *dst.add(1) = (a0 >> sh_1) | (a1 << sh_2);
        *dst.add(2) = (a1 >> sh_1) | (a2 << sh_2);
        *dst.add(3) = (a2 >> sh_1) | (a3 << sh_2);
        ap = a3;
        srcp += 4 * OPSIZ;
        dstp += 4 * OPSIZ;
        len -= 4;
    }
    while len > 0 {
        let a0 = *(srcp as *const OpT);
        *(dstp as *mut OpT) = (ap >> sh_1) | (a0 << sh_2);
        ap = a0;
        srcp += OPSIZ;
        dstp += OPSIZ;
        len -= 1;
    }
}

unsafe fn byte_copy_fwd(mut dstp: usize, mut srcp: usize, mut nbytes: usize) {
    while nbytes > 0 {
        let x = *(srcp as *const u8);
        srcp += 1;
        nbytes -= 1;
        *(dstp as *mut u8) = x;
        dstp += 1;
    }
}

unsafe fn word_copy_fwd(dstp: &mut usize, srcp: &mut usize, nbytes_left: &mut usize, nbytes: usize) {
    if *srcp % OPSIZ == 0 {
        wordcopy_fwd_aligned(*dstp, *srcp, nbytes / OPSIZ);
    } else {
        wordcopy_fwd_dest_aligned(*dstp, *srcp, nbytes / OPSIZ);
    }
    *srcp += nbytes & (! (OPSIZ - 1));
    *dstp += nbytes & (! (OPSIZ - 1));
    *nbytes_left = nbytes % OPSIZ;
}

pub unsafe extern "C" fn memcpy(dstpp: *mut core::ffi::c_void, srcpp: *const core::ffi::c_void, mut len: usize) -> *mut core::ffi::c_void {
    let mut dstp = dstpp as usize;
    let mut srcp = srcpp as usize;
    if len >= OP_T_THRES {
        let align = (0usize.wrapping_sub(dstp)) % OPSIZ;
        len -= align;
        byte_copy_fwd(dstp, srcp, align);
        word_copy_fwd(&mut dstp, &mut srcp, &mut len, len);
    }
    byte_copy_fwd(dstp, srcp, len);
    dstpp
}

pub unsafe extern "C" fn memcpyb(dstpp: *mut core::ffi::c_void, srcpp: *const core::ffi::c_void, len: u32) -> *mut core::ffi::c_void {
    byte_copy_fwd(dstpp as usize, srcpp as usize, len as usize);
    dstpp
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
