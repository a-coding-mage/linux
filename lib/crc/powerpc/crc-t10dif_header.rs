// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Calculate a CRC T10-DIF with vpmsum acceleration
 *
 * Copyright 2017, Daniel Axtens, IBM Corporation.
 * [based on crc32c-vpmsum_glue.c]
 */

// Dependencies supplied by the surrounding kernel translation.

const VMX_ALIGN: usize = 16;
const VMX_ALIGN_MASK: usize = VMX_ALIGN - 1;

const VECTOR_BREAKPOINT: usize = 64;

// static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_vec_crypto);
extern "C" {
    static mut have_vec_crypto: core::ffi::c_void;

    fn __crct10dif_vpmsum(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc_t10dif_generic(crc: u32, p: *const u8, len: usize) -> u16;
    fn static_branch_likely(key: *const core::ffi::c_void) -> bool;
    fn may_use_simd() -> bool;
    fn preempt_disable();
    fn pagefault_disable();
    fn enable_kernel_altivec();
    fn disable_kernel_altivec();
    fn pagefault_enable();
    fn preempt_enable();
    fn cpu_has_feature(feature: u32) -> bool;
    static mut cur_cpu_spec: CpuSpec;
    fn static_branch_enable(key: *mut core::ffi::c_void);
}

#[repr(C)]
struct CpuSpec {
    cpu_user_features2: u64,
}

const CPU_FTR_ARCH_207S: u32 = 0;
const PPC_FEATURE2_VEC_CRYPTO: u64 = 0;

#[inline]
unsafe fn crc_t10dif_arch(mut crci: u16, mut p: *const u8, mut len: usize) -> u16 {
    let prealign: usize;
    let tail: usize;
    let mut crc = crci as u32;

    if len < VECTOR_BREAKPOINT + VMX_ALIGN
        || !static_branch_likely(&have_vec_crypto as *const _ as *const core::ffi::c_void)
        || !may_use_simd()
    {
        return crc_t10dif_generic(crc, p, len);
    }

    if (p as usize) & VMX_ALIGN_MASK != 0 {
        prealign = VMX_ALIGN - ((p as usize) & VMX_ALIGN_MASK);
        crc = crc_t10dif_generic(crc, p, prealign) as u32;
        len -= prealign;
        p = p.add(prealign);
    }

    if len & !VMX_ALIGN_MASK != 0 {
        crc <<= 16;
        preempt_disable();
        pagefault_disable();
        enable_kernel_altivec();
        crc = __crct10dif_vpmsum(crc, p, len & !VMX_ALIGN_MASK);
        disable_kernel_altivec();
        pagefault_enable();
        preempt_enable();
        crc >>= 16;
    }

    tail = len & VMX_ALIGN_MASK;
    if tail != 0 {
        p = p.add(len & !VMX_ALIGN_MASK);
        crc = crc_t10dif_generic(crc, p, tail) as u32;
    }

    (crc & 0xffff) as u16
}

// #define crc_t10dif_mod_init_arch crc_t10dif_mod_init_arch
unsafe fn crc_t10dif_mod_init_arch() {
    if cpu_has_feature(CPU_FTR_ARCH_207S)
        && (cur_cpu_spec.cpu_user_features2 & PPC_FEATURE2_VEC_CRYPTO) != 0
    {
        static_branch_enable(&mut have_vec_crypto as *mut _ as *mut core::ffi::c_void);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
