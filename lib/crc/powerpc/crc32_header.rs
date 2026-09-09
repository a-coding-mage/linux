// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the surrounding kernel translation unit:
// asm/simd.h, asm/switch_to.h, linux/cpufeature.h, linux/jump_label.h,
// linux/preempt.h, linux/uaccess.h

pub const VMX_ALIGN: usize = 16;
pub const VMX_ALIGN_MASK: usize = VMX_ALIGN - 1;

pub const VECTOR_BREAKPOINT: usize = 512;

// Translation of: static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_vec_crypto);
static mut have_vec_crypto: bool = false;

// #define crc32_le_arch crc32_le_base /* not implemented on this arch */
// #define crc32_be_arch crc32_be_base /* not implemented on this arch */

extern "C" {
    pub fn __crc32c_vpmsum(crc: u32, p: *const u8, len: usize) -> u32;
    pub fn crc32c_base(crc: u32, p: *const u8, len: usize) -> u32;
    pub fn static_branch_likely(key: *const bool) -> bool;
    pub fn may_use_simd() -> bool;
    pub fn preempt_disable();
    pub fn pagefault_disable();
    pub fn enable_kernel_altivec();
    pub fn disable_kernel_altivec();
    pub fn pagefault_enable();
    pub fn preempt_enable();
    pub fn cpu_has_feature(feature: u32) -> bool;
    pub fn static_branch_enable(key: *mut bool);
    pub fn static_key_enabled(key: *const bool) -> bool;
}

pub const CPU_FTR_ARCH_207S: u32 = 0; // supplied by linux/cpufeature.h
pub const PPC_FEATURE2_VEC_CRYPTO: u64 = 0; // supplied by asm/processor.h
pub const CRC32C_OPTIMIZATION: u32 = 0; // supplied by the CRC32 implementation

#[repr(C)]
pub struct CpuSpec {
    pub cpu_user_features2: u64,
}

extern "C" {
    pub static cur_cpu_spec: *const CpuSpec;
}

#[inline]
pub unsafe fn crc32c_arch(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    let prealign: usize;
    let tail: usize;

    if len < (VECTOR_BREAKPOINT + VMX_ALIGN)
        || !static_branch_likely(&have_vec_crypto as *const bool)
        || !may_use_simd()
    {
        return crc32c_base(crc, p, len);
    }

    if (p as usize) & VMX_ALIGN_MASK != 0 {
        prealign = VMX_ALIGN - ((p as usize) & VMX_ALIGN_MASK);
        crc = crc32c_base(crc, p, prealign);
        len -= prealign;
        p = p.add(prealign);
    }

    if len & !VMX_ALIGN_MASK != 0 {
        preempt_disable();
        pagefault_disable();
        enable_kernel_altivec();
        crc = __crc32c_vpmsum(crc, p, len & !VMX_ALIGN_MASK);
        disable_kernel_altivec();
        pagefault_enable();
        preempt_enable();
    }

    tail = len & VMX_ALIGN_MASK;
    if tail != 0 {
        p = p.add(len & !VMX_ALIGN_MASK);
        crc = crc32c_base(crc, p, tail);
    }

    crc
}

// #define crc32_mod_init_arch crc32_mod_init_arch
unsafe fn crc32_mod_init_arch() {
    if cpu_has_feature(CPU_FTR_ARCH_207S)
        && ((*cur_cpu_spec).cpu_user_features2 & PPC_FEATURE2_VEC_CRYPTO) != 0
    {
        static_branch_enable(&mut have_vec_crypto as *mut bool);
    }
}

#[inline]
unsafe fn crc32_optimizations_arch() -> u32 {
    if static_key_enabled(&have_vec_crypto as *const bool) {
        return CRC32C_OPTIMIZATION;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
