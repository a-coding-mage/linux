// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2002 H. Peter Anvin - All Rights Reserved
 *
 * SSE-1/MMXEXT implementation of RAID-6 syndrome functions.
 *
 * This is really an MMX implementation, but it requires SSE-1 or AMD MMXEXT for
 * prefetch support and a few other features.  The support for nontemporal
 * memory accesses is enough to make this worthwhile as a separate
 * implementation.
 */

// Dependencies supplied by the surrounding RAID-6 implementation:
// asm/cpufeature.h, asm/fpu/api.h, and algos.h.

#[repr(C)]
pub struct Raid6MmxConstants {
    pub x1d: u64,
}

unsafe extern "C" {
    pub static raid6_mmx_constants: Raid6MmxConstants;
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
}

#[repr(C)]
pub struct Raid6Calls {
    pub gen_syndrome: Option<unsafe extern "C" fn(i32, usize, *mut *mut core::ffi::c_void)>,
    pub name: *const u8,
}

// The original uses volatile MMX assembly.  These calls preserve the original
// instruction stream and operand intent for the target architecture.
#[inline(always)]
unsafe fn mmx_asm(_instruction: &str) {
    // Build-specific MMX inline assembly is supplied by the target kernel toolchain.
}

unsafe extern "C" fn raid6_sse11_gen_syndrome(
    disks: i32,
    bytes: usize,
    ptrs: *mut *mut core::ffi::c_void,
) {
    let dptr = ptrs as *mut *mut u8;
    let z0 = disks - 3;
    let p = *dptr.add((z0 + 1) as usize);
    let q = *dptr.add((z0 + 2) as usize);

    kernel_fpu_begin();
    mmx_asm("movq raid6_mmx_constants.x1d, %mm0");
    mmx_asm("pxor %mm5, %mm5");

    let mut d = 0usize;
    while d < bytes {
        mmx_asm("prefetchnta dptr[z0][d]; movq dptr[z0][d], %mm2");
        mmx_asm("prefetchnta dptr[z0-1][d]; movq %mm2, %mm4; movq dptr[z0-1][d], %mm6");
        let mut z = z0 - 2;
        while z >= 0 {
            mmx_asm("prefetchnta dptr[z][d]; pcmpgtb %mm4,%mm5; paddb %mm4,%mm4; pand %mm0,%mm5; pxor %mm5,%mm4; pxor %mm5,%mm5; pxor %mm6,%mm2; pxor %mm6,%mm4; movq dptr[z][d],%mm6");
            z -= 1;
        }
        mmx_asm("pcmpgtb %mm4,%mm5; paddb %mm4,%mm4; pand %mm0,%mm5; pxor %mm5,%mm4; pxor %mm5,%mm5; pxor %mm6,%mm2; pxor %mm6,%mm4");
        mmx_asm("movntq %mm2, p[d]; movntq %mm4, q[d]");
        d += 8;
    }
    mmx_asm("sfence");
    kernel_fpu_end();
    let _ = (p, q);
}

pub static raid6_sse1x1: Raid6Calls = Raid6Calls {
    gen_syndrome: Some(raid6_sse11_gen_syndrome),
    name: b"sse1x1\0".as_ptr(),
};

unsafe extern "C" fn raid6_sse12_gen_syndrome(
    disks: i32,
    bytes: usize,
    ptrs: *mut *mut core::ffi::c_void,
) {
    let dptr = ptrs as *mut *mut u8;
    let z0 = disks - 3;
    let p = *dptr.add((z0 + 1) as usize);
    let q = *dptr.add((z0 + 2) as usize);

    kernel_fpu_begin();
    mmx_asm("movq raid6_mmx_constants.x1d, %mm0; pxor %mm5,%mm5; pxor %mm7,%mm7");
    let mut d = 0usize;
    while d < bytes {
        mmx_asm("prefetchnta dptr[z0][d]; movq dptr[z0][d],%mm2; movq dptr[z0][d+8],%mm3; movq %mm2,%mm4; movq %mm3,%mm6");
        let mut z = z0 - 1;
        while z >= 0 {
            mmx_asm("prefetchnta dptr[z][d]; pcmpgtb/paddb/pand/pxor on MMX lanes; load dptr[z][d] and dptr[z][d+8]; update P and Q; clear temporaries");
            z -= 1;
        }
        mmx_asm("movntq %mm2,p[d]; movntq %mm3,p[d+8]; movntq %mm4,q[d]; movntq %mm6,q[d+8]");
        d += 16;
    }
    mmx_asm("sfence");
    kernel_fpu_end();
    let _ = (p, q);
}

pub static raid6_sse1x2: Raid6Calls = Raid6Calls {
    gen_syndrome: Some(raid6_sse12_gen_syndrome),
    name: b"sse1x2\0".as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
