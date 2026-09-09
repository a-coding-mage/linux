/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * ChaCha and HChaCha functions (x86_64 optimized)
 *
 * Copyright (C) 2015 Martin Willi
 */

// C dependencies supplied by the surrounding kernel translation.

extern "C" {
    pub fn chacha_block_xor_ssse3(state: *const chacha_state, dst: *mut u8,
                                  src: *const u8, len: u32, nrounds: i32);
    pub fn chacha_4block_xor_ssse3(state: *const chacha_state, dst: *mut u8,
                                   src: *const u8, len: u32, nrounds: i32);
    pub fn hchacha_block_ssse3(state: *const chacha_state,
                               out: *mut u32, nrounds: i32);
    pub fn chacha_2block_xor_avx2(state: *const chacha_state, dst: *mut u8,
                                  src: *const u8, len: u32, nrounds: i32);
    pub fn chacha_4block_xor_avx2(state: *const chacha_state, dst: *mut u8,
                                  src: *const u8, len: u32, nrounds: i32);
    pub fn chacha_8block_xor_avx2(state: *const chacha_state, dst: *mut u8,
                                  src: *const u8, len: u32, nrounds: i32);
    pub fn chacha_2block_xor_avx512vl(state: *const chacha_state, dst: *mut u8,
                                      src: *const u8, len: u32, nrounds: i32);
    pub fn chacha_4block_xor_avx512vl(state: *const chacha_state, dst: *mut u8,
                                      src: *const u8, len: u32, nrounds: i32);
    pub fn chacha_8block_xor_avx512vl(state: *const chacha_state, dst: *mut u8,
                                      src: *const u8, len: u32, nrounds: i32);
    pub fn hchacha_block_generic(state: *const chacha_state, out: *mut u32,
                                 nrounds: i32);
    pub fn chacha_crypt_generic(state: *mut chacha_state, dst: *mut u8,
                                src: *const u8, bytes: u32, nrounds: i32);
    pub fn kernel_fpu_begin();
    pub fn kernel_fpu_end();
    pub fn boot_cpu_has(feature: u32) -> bool;
    pub fn cpu_has_xfeatures(features: u64, feature: *mut core::ffi::c_void) -> bool;
    pub fn static_branch_likely(key: *const bool) -> bool;
    pub fn static_branch_enable(key: *mut bool);
}

extern "Rust" {
    type chacha_state;
}

static mut chacha_use_simd: bool = false;
static mut chacha_use_avx2: bool = false;
static mut chacha_use_avx512vl: bool = false;

unsafe fn chacha_advance(mut len: u32, maxblocks: u32) -> u32 {
    len = core::cmp::min(len, maxblocks * CHACHA_BLOCK_SIZE);
    (len + CHACHA_BLOCK_SIZE - 1) / CHACHA_BLOCK_SIZE
}

unsafe fn chacha_dosimd(mut state: *mut chacha_state, mut dst: *mut u8,
                        mut src: *const u8, mut bytes: u32, nrounds: i32) {
    if static_branch_likely(&chacha_use_avx512vl) {
        while bytes >= CHACHA_BLOCK_SIZE * 8 {
            chacha_8block_xor_avx512vl(state, dst, src, bytes, nrounds);
            bytes -= CHACHA_BLOCK_SIZE * 8; src = src.add((CHACHA_BLOCK_SIZE * 8) as usize);
            dst = dst.add((CHACHA_BLOCK_SIZE * 8) as usize); (*state).x[12] += 8;
        }
        if bytes > CHACHA_BLOCK_SIZE * 4 { chacha_8block_xor_avx512vl(state,dst,src,bytes,nrounds); (*state).x[12] += chacha_advance(bytes,8); return; }
        if bytes > CHACHA_BLOCK_SIZE * 2 { chacha_4block_xor_avx512vl(state,dst,src,bytes,nrounds); (*state).x[12] += chacha_advance(bytes,4); return; }
        if bytes != 0 { chacha_2block_xor_avx512vl(state,dst,src,bytes,nrounds); (*state).x[12] += chacha_advance(bytes,2); return; }
    }
    if static_branch_likely(&chacha_use_avx2) {
        while bytes >= CHACHA_BLOCK_SIZE * 8 {
            chacha_8block_xor_avx2(state,dst,src,bytes,nrounds); bytes -= CHACHA_BLOCK_SIZE*8;
            src=src.add((CHACHA_BLOCK_SIZE*8) as usize); dst=dst.add((CHACHA_BLOCK_SIZE*8) as usize); (*state).x[12]+=8;
        }
        if bytes > CHACHA_BLOCK_SIZE*4 { chacha_8block_xor_avx2(state,dst,src,bytes,nrounds); (*state).x[12]+=chacha_advance(bytes,8); return; }
        if bytes > CHACHA_BLOCK_SIZE*2 { chacha_4block_xor_avx2(state,dst,src,bytes,nrounds); (*state).x[12]+=chacha_advance(bytes,4); return; }
        if bytes > CHACHA_BLOCK_SIZE { chacha_2block_xor_avx2(state,dst,src,bytes,nrounds); (*state).x[12]+=chacha_advance(bytes,2); return; }
    }
    while bytes >= CHACHA_BLOCK_SIZE*4 { chacha_4block_xor_ssse3(state,dst,src,bytes,nrounds); bytes-=CHACHA_BLOCK_SIZE*4; src=src.add((CHACHA_BLOCK_SIZE*4) as usize); dst=dst.add((CHACHA_BLOCK_SIZE*4) as usize); (*state).x[12]+=4; }
    if bytes > CHACHA_BLOCK_SIZE { chacha_4block_xor_ssse3(state,dst,src,bytes,nrounds); (*state).x[12]+=chacha_advance(bytes,4); return; }
    if bytes != 0 { chacha_block_xor_ssse3(state,dst,src,bytes,nrounds); (*state).x[12]+=1; }
}

unsafe fn hchacha_block_arch(state: *const chacha_state, out: *mut u32, nrounds: i32) {
    if !static_branch_likely(&chacha_use_simd) { hchacha_block_generic(state,out,nrounds); }
    else { kernel_fpu_begin(); hchacha_block_ssse3(state,out,nrounds); kernel_fpu_end(); }
}

unsafe fn chacha_crypt_arch(state: *mut chacha_state, mut dst: *mut u8, mut src: *const u8, mut bytes: u32, nrounds: i32) {
    if !static_branch_likely(&chacha_use_simd) || bytes <= CHACHA_BLOCK_SIZE { return chacha_crypt_generic(state,dst,src,bytes,nrounds); }
    while bytes != 0 { let todo = core::cmp::min(bytes, SZ_4K); kernel_fpu_begin(); chacha_dosimd(state,dst,src,todo,nrounds); kernel_fpu_end(); bytes-=todo; src=src.add(todo as usize); dst=dst.add(todo as usize); }
}

unsafe fn chacha_mod_init_arch() {
    if !boot_cpu_has(X86_FEATURE_SSSE3) { return; }
    static_branch_enable(&mut chacha_use_simd);
    if boot_cpu_has(X86_FEATURE_AVX) && boot_cpu_has(X86_FEATURE_AVX2) && cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, core::ptr::null_mut()) {
        static_branch_enable(&mut chacha_use_avx2);
        if boot_cpu_has(X86_FEATURE_AVX512VL) && boot_cpu_has(X86_FEATURE_AVX512BW) { static_branch_enable(&mut chacha_use_avx512vl); }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
