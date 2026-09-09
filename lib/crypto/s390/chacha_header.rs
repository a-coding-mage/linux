/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ChaCha stream cipher (s390 optimized)
 *
 * Copyright IBM Corp. 2021
 */

// Dependencies supplied by the surrounding translation unit:
// linux/cpufeature.h, linux/export.h, linux/kernel.h, linux/sizes.h,
// asm/fpu.h, and chacha-s390.h.

// #define hchacha_block_arch hchacha_block_generic /* not implemented yet */
// The architecture implementation aliases hchacha_block_arch to
// hchacha_block_generic.

unsafe fn chacha_crypt_arch(
    state: *mut chacha_state,
    dst: *mut u8,
    src: *const u8,
    bytes: core::ffi::c_uint,
    nrounds: core::ffi::c_int,
) {
    /* s390 chacha20 implementation has 20 rounds hard-coded,
     * it cannot handle a block of data or less, but otherwise
     * it can handle data of arbitrary size
     */
    if bytes <= CHACHA_BLOCK_SIZE || nrounds != 20 || !cpu_has_vx() {
        chacha_crypt_generic(state, dst, src, bytes, nrounds);
    } else {
        // DECLARE_KERNEL_FPU_ONSTACK32(vxstate)
        let mut vxstate = core::mem::MaybeUninit::<KernelFpuState>::uninit();

        kernel_fpu_begin(vxstate.as_mut_ptr(), KERNEL_VXR);
        chacha20_vx(
            dst,
            src,
            bytes,
            (*state).x.as_mut_ptr().add(4),
            (*state).x.as_mut_ptr().add(12),
        );
        kernel_fpu_end(vxstate.as_mut_ptr(), KERNEL_VXR);

        (*state).x[12] += round_up(bytes, CHACHA_BLOCK_SIZE) / CHACHA_BLOCK_SIZE;
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
