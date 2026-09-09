// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2002 H. Peter Anvin - All Rights Reserved
 *
 * MMX implementation of RAID-6 syndrome functions.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct raid6_mmx_constants_t {
    pub x1d: u64,
}

#[no_mangle]
pub static raid6_mmx_constants: raid6_mmx_constants_t = raid6_mmx_constants_t {
    x1d: 0x1d1d1d1d1d1d1d1d_u64,
};

extern "C" {
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
}

unsafe fn raid6_mmx1_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) {
    let dptr = ptrs as *mut *mut u8;
    let z0 = disks - 3;
    let p = *dptr.add((z0 + 1) as usize);
    let q = *dptr.add((z0 + 2) as usize);

    kernel_fpu_begin();
    core::arch::asm!("movq [{constants}], %mm0", constants = in(reg) &raid6_mmx_constants.x1d);
    core::arch::asm!("pxor %mm5, %mm5");

    let mut d = 0usize;
    while d < bytes {
        let mut z = z0 - 1;
        core::arch::asm!("movq [{value}], %mm2", value = in(reg) (*dptr.add(z0 as usize)).add(d));
        core::arch::asm!("movq %mm2, %mm4");
        while z >= 0 {
            core::arch::asm!("movq [{value}], %mm6", value = in(reg) (*dptr.add(z as usize)).add(d));
            core::arch::asm!("pcmpgtb %mm4, %mm5; paddb %mm4, %mm4; pand %mm0, %mm5; pxor %mm5, %mm4; pxor %mm5, %mm5; pxor %mm6, %mm2; pxor %mm6, %mm4");
            z -= 1;
        }
        core::arch::asm!("movq %mm2, [{value}]", value = in(reg) p.add(d));
        core::arch::asm!("pxor %mm2, %mm2");
        core::arch::asm!("movq %mm4, [{value}]", value = in(reg) q.add(d));
        core::arch::asm!("pxor %mm4, %mm4");
        d += 8;
    }
    kernel_fpu_end();
}

#[repr(C)]
pub struct raid6_calls {
    pub gen_syndrome: unsafe fn(i32, usize, *mut *mut core::ffi::c_void),
    pub name: *const u8,
}

#[no_mangle]
pub static raid6_mmxx1: raid6_calls = raid6_calls {
    gen_syndrome: raid6_mmx1_gen_syndrome,
    name: b"mmxx1\0".as_ptr(),
};

unsafe fn raid6_mmx2_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) {
    let dptr = ptrs as *mut *mut u8;
    let z0 = disks - 3;
    let p = *dptr.add((z0 + 1) as usize);
    let q = *dptr.add((z0 + 2) as usize);

    kernel_fpu_begin();
    core::arch::asm!("movq [{constants}], %mm0", constants = in(reg) &raid6_mmx_constants.x1d);
    core::arch::asm!("pxor %mm5, %mm5; pxor %mm7, %mm7");

    let mut d = 0usize;
    while d < bytes {
        core::arch::asm!("movq [{value}], %mm2", value = in(reg) (*dptr.add(z0 as usize)).add(d));
        core::arch::asm!("movq [{value}], %mm3", value = in(reg) (*dptr.add(z0 as usize)).add(d + 8));
        core::arch::asm!("movq %mm2, %mm4; movq %mm3, %mm6");
        let mut z = z0 - 1;
        while z >= 0 {
            core::arch::asm!("pcmpgtb %mm4, %mm5; pcmpgtb %mm6, %mm7; paddb %mm4, %mm4; paddb %mm6, %mm6; pand %mm0, %mm5; pand %mm0, %mm7; pxor %mm5, %mm4; pxor %mm7, %mm6");
            core::arch::asm!("movq [{value}], %mm5", value = in(reg) (*dptr.add(z as usize)).add(d));
            core::arch::asm!("movq [{value}], %mm7", value = in(reg) (*dptr.add(z as usize)).add(d + 8));
            core::arch::asm!("pxor %mm5, %mm2; pxor %mm7, %mm3; pxor %mm5, %mm4; pxor %mm7, %mm6; pxor %mm5, %mm5; pxor %mm7, %mm7");
            z -= 1;
        }
        core::arch::asm!("movq %mm2, [{value}]", value = in(reg) p.add(d));
        core::arch::asm!("movq %mm3, [{value}]", value = in(reg) p.add(d + 8));
        core::arch::asm!("movq %mm4, [{value}]", value = in(reg) q.add(d));
        core::arch::asm!("movq %mm6, [{value}]", value = in(reg) q.add(d + 8));
        d += 16;
    }
    kernel_fpu_end();
}

#[no_mangle]
pub static raid6_mmxx2: raid6_calls = raid6_calls {
    gen_syndrome: raid6_mmx2_gen_syndrome,
    name: b"mmxx2\0".as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
