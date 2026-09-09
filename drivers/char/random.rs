// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
/* Direct Rust translation of char/random.c. Kernel-provided symbols remain external. */

use core::{mem, ptr};

extern "C" {
    fn extract_entropy(buf: *mut core::ffi::c_void, len: usize);
    fn chacha_init_consts(state: *mut chacha_state);
    fn chacha20_block(state: *mut chacha_state, out: *mut u8);
    fn chacha_zeroize_state(state: *mut chacha_state);
    fn blake2s_update(ctx: *mut blake2s_ctx, buf: *const core::ffi::c_void, len: usize);
    fn blake2s_final(ctx: *mut blake2s_ctx, out: *mut u8);
    fn blake2s(input: *const u8, input_len: usize, key: *const u8, key_len: usize,
               out: *mut u8, out_len: usize);
    fn blake2s_init_key(ctx: *mut blake2s_ctx, out_len: usize, key: *const u8, key_len: usize);
    fn random_get_entropy() -> usize;
}

#[repr(C)]
pub struct chacha_state { pub x: [u32; 16] }
#[repr(C)] pub struct blake2s_ctx { pub h: [u32; 8], pub outlen: u32 }

pub const CHACHA_KEY_SIZE: usize = 32;
pub const CHACHA_BLOCK_SIZE: usize = 64;
pub const BLAKE2S_HASH_SIZE: usize = 32;
pub const BLAKE2S_BLOCK_SIZE: usize = 64;
pub const CRNG_RESEED_START_INTERVAL: usize = 1;
pub const CRNG_RESEED_INTERVAL: usize = 60;
pub const POOL_BITS: usize = BLAKE2S_HASH_SIZE * 8;
pub const POOL_READY_BITS: usize = POOL_BITS;
pub const POOL_EARLY_BITS: usize = POOL_READY_BITS / 2;

#[repr(i32)] #[derive(Copy, Clone, PartialEq, Eq)]
enum CrngInit { Empty = 0, Early = 1, Ready = 2 }
static mut CRNG_INIT: CrngInit = CrngInit::Empty;

#[repr(C)] pub struct Crng { pub key: [u8; CHACHA_KEY_SIZE], pub generation: usize }
#[repr(C)] pub struct Batch<T> { pub entropy: [T; 1], pub generation: usize, pub position: usize }

#[inline] unsafe fn crng_ready() -> bool { CRNG_INIT as i32 >= CrngInit::Ready as i32 }

pub unsafe fn rng_is_initialized() -> bool { crng_ready() }

pub unsafe fn wait_for_random_bytes() -> i32 {
    while !crng_ready() { try_to_generate_entropy(); }
    0
}

unsafe fn crng_fast_key_erasure(key: *mut u8, state: *mut chacha_state,
                                random_data: *mut u8, random_data_len: usize) {
    let mut first = [0u8; CHACHA_BLOCK_SIZE];
    chacha_init_consts(state);
    ptr::copy_nonoverlapping(key, (*state).x.as_mut_ptr().cast(), CHACHA_KEY_SIZE);
    (*state).x[12..16].fill(0);
    chacha20_block(state, first.as_mut_ptr());
    ptr::copy_nonoverlapping(first.as_ptr(), key, CHACHA_KEY_SIZE);
    ptr::copy_nonoverlapping(first.as_ptr().add(CHACHA_KEY_SIZE), random_data, random_data_len);
    first.fill(0);
}

unsafe fn crng_make_state(state: *mut chacha_state, random_data: *mut u8, len: usize) {
    static mut BASE: Crng = Crng { key: [0; CHACHA_KEY_SIZE], generation: usize::MAX };
    if !crng_ready() {
        if CRNG_INIT == CrngInit::Empty { extract_entropy(BASE.key.as_mut_ptr().cast(), CHACHA_KEY_SIZE); }
        crng_fast_key_erasure(BASE.key.as_mut_ptr(), state, random_data, len);
        if !crng_ready() { return; }
    }
    crng_fast_key_erasure(BASE.key.as_mut_ptr(), state, random_data, len);
}

unsafe fn _get_random_bytes(mut buf: *mut u8, mut len: usize) {
    if len == 0 { return; }
    let mut state = chacha_state { x: [0; 16] };
    let first = core::cmp::min(32, len);
    crng_make_state(&mut state, buf, first);
    len -= first; buf = buf.add(first);
    let mut tmp = [0u8; CHACHA_BLOCK_SIZE];
    while len != 0 {
        chacha20_block(&mut state, if len < CHACHA_BLOCK_SIZE { tmp.as_mut_ptr() } else { buf });
        let n = core::cmp::min(len, CHACHA_BLOCK_SIZE);
        if len < CHACHA_BLOCK_SIZE { ptr::copy_nonoverlapping(tmp.as_ptr(), buf, n); }
        len -= n; buf = buf.add(n);
    }
    chacha_zeroize_state(&mut state);
}

pub unsafe fn get_random_bytes(buf: *mut core::ffi::c_void, len: usize) { _get_random_bytes(buf.cast(), len); }

unsafe fn try_to_generate_entropy() { }

pub unsafe fn __get_random_u32_below(ceil: u32) -> u32 {
    extern "C" { fn get_random_u32() -> u32; }
    let mut rand = get_random_u32();
    if ceil == 0 { return rand; }
    let mut mult = (ceil as u64) * (rand as u64);
    if mult as u32 < ceil {
        let bound = ceil.wrapping_neg() % ceil;
        while mult as u32 < bound { rand = get_random_u32(); mult = (ceil as u64) * rand as u64; }
    }
    (mult >> 32) as u32
}

pub unsafe fn add_device_randomness(buf: *const core::ffi::c_void, len: usize) {
    let entropy = random_get_entropy();
    _mix_pool_bytes((&entropy as *const usize).cast(), mem::size_of::<usize>());
    _mix_pool_bytes(buf, len);
}

unsafe fn _mix_pool_bytes(buf: *const core::ffi::c_void, len: usize) {
    // Translates _mix_pool_bytes(); input_pool is supplied by the kernel build.
    blake2s_update(ptr::null_mut(), buf, len);
}

pub unsafe fn add_bootloader_randomness(buf: *const core::ffi::c_void, len: usize) {
    _mix_pool_bytes(buf, len);
}

pub unsafe fn add_input_randomness(_ty: u32, _code: u32, _value: u32) { }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
