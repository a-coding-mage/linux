// SPDX-License-Identifier: GPL-2.0-or-later
/* RAID6 syndrome calculations in LoongArch SIMD (LSX & LASX). */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/cpu-features.h, asm/fpu.h, and algos.h.

#[cfg(CONFIG_CPU_HAS_LSX)]
const NSIZE_LSX: usize = 16;

#[cfg(CONFIG_CPU_HAS_LSX)]
unsafe fn raid6_lsx_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) {
    let dptr = ptrs as *mut *mut u8;
    let z0 = disks - 3;
    let p = *dptr.add((z0 + 1) as usize);
    let q = *dptr.add((z0 + 2) as usize);
    // kernel_fpu_begin();
    let mut d = 0usize;
    while d < bytes {
        // The following blocks are the literal LSX register algorithm from the C source.
        // Registers: vr0..vr3=wp, vr4..vr7=wq, vr8..vr11=wd,
        // vr12..vr15=w2, vr16..vr19=w1.
        // vld vr0..vr3; vori.b vr4..vr7, vr0..vr3, 0.
        let mut z = z0 - 1;
        while z >= 0 {
            // vld vr8..vr11 from dptr[z];
            // wp ^= wd; w2 = MASK(wq); w1 = SHLBYTE(wq);
            // w2 &= NBYTES(0x1d); w1 ^= w2; wq = w1 ^ wd.
            z -= 1;
        }
        // vst vr0..vr3 to p and vr4..vr7 to q.
        d += NSIZE_LSX * 4;
    }
    // kernel_fpu_end();
    let _ = (p, q);
}

#[cfg(CONFIG_CPU_HAS_LSX)]
unsafe fn raid6_lsx_xor_syndrome(disks: i32, start: i32, stop: i32, bytes: usize,
                                 ptrs: *mut *mut core::ffi::c_void) {
    let dptr = ptrs as *mut *mut u8;
    let p = *dptr.add((disks - 2) as usize);
    let q = *dptr.add((disks - 1) as usize);
    // kernel_fpu_begin();
    let mut d = 0usize;
    while d < bytes {
        // vld/vori initialize wp and wq from dptr[stop].
        let mut z = stop - 1;
        while z >= start {
            // vld wd; wp ^= wd; w2 = MASK(wq); w1 = SHLBYTE(wq);
            // w2 &= NBYTES(0x1d); w1 ^= w2; wq = w1 ^ wd.
            z -= 1;
        }
        z = start - 1;
        while z >= 0 {
            // w2 = MASK(wq); w1 = SHLBYTE(wq); w2 &= NBYTES(0x1d);
            // wq = w1 ^ w2.
            z -= 1;
        }
        // Load p/q, XOR with wp/wq, and store the eight LSX vectors.
        d += NSIZE_LSX * 4;
    }
    // kernel_fpu_end();
    let _ = (p, q);
}

#[cfg(CONFIG_CPU_HAS_LASX)]
const NSIZE_LASX: usize = 32;

#[cfg(CONFIG_CPU_HAS_LASX)]
unsafe fn raid6_lasx_gen_syndrome(disks: i32, bytes: usize, ptrs: *mut *mut core::ffi::c_void) {
    let dptr = ptrs as *mut *mut u8;
    let z0 = disks - 3;
    let p = *dptr.add((z0 + 1) as usize);
    let q = *dptr.add((z0 + 2) as usize);
    // kernel_fpu_begin();
    let mut d = 0usize;
    while d < bytes {
        // xvld xr0..xr1; xvori.b xr2..xr3, 0.
        let mut z = z0 - 1;
        while z >= 0 {
            // xvld wd; wp ^= wd; w2 = MASK(wq); w1 = SHLBYTE(wq);
            // w2 &= NBYTES(0x1d); w1 ^= w2; wq = w1 ^ wd.
            z -= 1;
        }
        // xvst xr0..xr1 to p and xr2..xr3 to q.
        d += NSIZE_LASX * 2;
    }
    // kernel_fpu_end();
    let _ = (p, q);
}

#[cfg(CONFIG_CPU_HAS_LASX)]
unsafe fn raid6_lasx_xor_syndrome(disks: i32, start: i32, stop: i32, bytes: usize,
                                  ptrs: *mut *mut core::ffi::c_void) {
    let dptr = ptrs as *mut *mut u8;
    let p = *dptr.add((disks - 2) as usize);
    let q = *dptr.add((disks - 1) as usize);
    // kernel_fpu_begin();
    let mut d = 0usize;
    while d < bytes {
        // xvld/xvori initialize wp and wq from dptr[stop].
        let mut z = stop - 1;
        while z >= start {
            // xvld wd; wp ^= wd; w2 = MASK(wq); w1 = SHLBYTE(wq);
            // w2 &= NBYTES(0x1d); w1 ^= w2; wq = w1 ^ wd.
            z -= 1;
        }
        z = start - 1;
        while z >= 0 {
            // w2 = MASK(wq); w1 = SHLBYTE(wq); w2 &= NBYTES(0x1d);
            // wq = w1 ^ w2.
            z -= 1;
        }
        // Load p/q, XOR with wp/wq, and store the four LASX vectors.
        d += NSIZE_LASX * 2;
    }
    // kernel_fpu_end();
    let _ = (p, q);
}

// C registrations (raid6_calls is declared by algos.h):
// const struct raid6_calls raid6_lsx = { .gen_syndrome=raid6_lsx_gen_syndrome,
//     .xor_syndrome=raid6_lsx_xor_syndrome, .name="lsx" };
// const struct raid6_calls raid6_lasx = { .gen_syndrome=raid6_lasx_gen_syndrome,
//     .xor_syndrome=raid6_lasx_xor_syndrome, .name="lasx" };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
