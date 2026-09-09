// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Optimized XOR parity functions for alpha EV5 and EV6.
 *
 * The original implementation supplies Alpha assembly through xor_impl.h and
 * xor_arch.h.  These declarations retain the C ABI and the byte-count and
 * pointer semantics of that implementation.
 */

#[inline(always)]
unsafe fn xor_words(bytes: usize, p1: *mut usize, sources: &[*const usize]) {
    let words = bytes / core::mem::size_of::<usize>();
    for i in 0..words {
        let mut value = core::ptr::read_volatile(p1.add(i));
        for &source in sources {
            value ^= core::ptr::read_volatile(source.add(i));
        }
        core::ptr::write_volatile(p1.add(i), value);
    }
}

/// Alpha EV5/EV6 XOR kernels. The original assembly processes 64-byte blocks.
#[no_mangle]
pub unsafe extern "C" fn xor_alpha_2(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
) {
    xor_words(bytes, p1, &[p2]);
}

#[no_mangle]
pub unsafe extern "C" fn xor_alpha_3(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
) {
    xor_words(bytes, p1, &[p2, p3]);
}

#[no_mangle]
pub unsafe extern "C" fn xor_alpha_4(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
    p4: *const usize,
) {
    xor_words(bytes, p1, &[p2, p3, p4]);
}

#[no_mangle]
pub unsafe extern "C" fn xor_alpha_5(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
    p4: *const usize,
    p5: *const usize,
) {
    xor_words(bytes, p1, &[p2, p3, p4, p5]);
}

// The prefetch variants have the same externally visible computation. The
// source Alpha assembly additionally issues cache-prefetch loads 256 bytes
// ahead of the active block.
#[no_mangle]
pub unsafe extern "C" fn xor_alpha_prefetch_2(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
) {
    xor_words(bytes, p1, &[p2]);
}

#[no_mangle]
pub unsafe extern "C" fn xor_alpha_prefetch_3(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
) {
    xor_words(bytes, p1, &[p2, p3]);
}

#[no_mangle]
pub unsafe extern "C" fn xor_alpha_prefetch_4(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
    p4: *const usize,
) {
    xor_words(bytes, p1, &[p2, p3, p4]);
}

#[no_mangle]
pub unsafe extern "C" fn xor_alpha_prefetch_5(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
    p4: *const usize,
    p5: *const usize,
) {
    xor_words(bytes, p1, &[p2, p3, p4, p5]);
}

/*
 * DO_XOR_BLOCKS(alpha, ...), DO_XOR_BLOCKS(alpha_prefetch, ...) and the two
 * xor_block_template initializers are supplied by xor_impl.h in the source.
 * Their declarations are intentionally left to the consuming kernel crate,
 * where the corresponding template types and xor_gen functions are defined.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
