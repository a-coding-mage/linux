/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency supplied by the architecture CPU-feature headers.

#[repr(C)]
pub struct xor_block_template {
    _private: [u8; 0],
}

extern "C" {
    pub static mut xor_block_pII_mmx: xor_block_template;
    pub static mut xor_block_p5_mmx: xor_block_template;
    pub static mut xor_block_sse: xor_block_template;
    pub static mut xor_block_sse_pf64: xor_block_template;
    pub static mut xor_block_avx: xor_block_template;

    pub static mut xor_block_8regs: xor_block_template;
    pub static mut xor_block_8regs_p: xor_block_template;
    pub static mut xor_block_32regs: xor_block_template;
    pub static mut xor_block_32regs_p: xor_block_template;

    fn boot_cpu_has(feature: i32) -> bool;
    fn xor_force(template: *mut xor_block_template);
    fn xor_register(template: *mut xor_block_template);
}

// When SSE is available, use it as it can write around L2.  We may also be able
// to load into the L1 only depending on how the cpu deals with a load to a line
// that is being prefetched.
//
// When AVX2 is available, force using it as it is better by all measures.
//
// 32-bit without MMX can fall back to the generic routines.
#[inline(always)]
pub unsafe fn arch_xor_init() {
    if boot_cpu_has(X86_FEATURE_AVX) && boot_cpu_has(X86_FEATURE_OSXSAVE) {
        xor_force(&raw mut xor_block_avx);
    } else if cfg!(target_arch = "x86_64") || boot_cpu_has(X86_FEATURE_XMM) {
        xor_register(&raw mut xor_block_sse);
        xor_register(&raw mut xor_block_sse_pf64);
    } else if boot_cpu_has(X86_FEATURE_MMX) {
        xor_register(&raw mut xor_block_pII_mmx);
        xor_register(&raw mut xor_block_p5_mmx);
    } else {
        xor_register(&raw mut xor_block_8regs);
        xor_register(&raw mut xor_block_8regs_p);
        xor_register(&raw mut xor_block_32regs);
        xor_register(&raw mut xor_block_32regs_p);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
