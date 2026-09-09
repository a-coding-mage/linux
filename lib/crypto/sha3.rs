// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SHA-3, as specified in
 * https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
 *
 * SHA-3 code by Jeff Garzik <jeff@garzik.org>
 *               Ard Biesheuvel <ard.biesheuvel@linaro.org>
 *               David Howells <dhowells@redhat.com>
 *
 * See also Documentation/crypto/sha3.rst
 */

// Dependencies supplied by the surrounding kernel translation.
const SHA3_KECCAK_ROUNDS: usize = 24;

static SHA3_KECCAKF_RNDC: [u64; SHA3_KECCAK_ROUNDS] = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808a,
    0x8000000080008000, 0x000000000000808b, 0x0000000080000001,
    0x8000000080008081, 0x8000000000008009, 0x000000000000008a,
    0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
    0x000000008000808b, 0x800000000000008b, 0x8000000000008089,
    0x8000000000008003, 0x8000000000008002, 0x8000000000000080,
    0x000000000000800a, 0x800000008000000a, 0x8000000080008081,
    0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
];

/* Perform a single round of Keccak mixing. */
#[inline]
unsafe fn sha3_keccakf_one_round_generic(st: *mut u64, round: i32) {
    let mut t = [0u64; 5];
    let mut tt: u64;
    let mut bc = [0u64; 5];
    let s = |i: usize| *st.add(i);
    let r = |v: u64, n: u32| v.rotate_left(n);

    bc[0] = s(0) ^ s(5) ^ s(10) ^ s(15) ^ s(20);
    bc[1] = s(1) ^ s(6) ^ s(11) ^ s(16) ^ s(21);
    bc[2] = s(2) ^ s(7) ^ s(12) ^ s(17) ^ s(22);
    bc[3] = s(3) ^ s(8) ^ s(13) ^ s(18) ^ s(23);
    bc[4] = s(4) ^ s(9) ^ s(14) ^ s(19) ^ s(24);
    t[0] = bc[4] ^ r(bc[1], 1); t[1] = bc[0] ^ r(bc[2], 1);
    t[2] = bc[1] ^ r(bc[3], 1); t[3] = bc[2] ^ r(bc[4], 1);
    t[4] = bc[3] ^ r(bc[0], 1);
    *st.add(0) ^= t[0];

    tt = s(1);
    *st.add(1) = r(s(6) ^ t[1], 44); *st.add(6) = r(s(9) ^ t[4], 20);
    *st.add(9) = r(s(22) ^ t[2], 61); *st.add(22) = r(s(14) ^ t[4], 39);
    *st.add(14) = r(s(20) ^ t[0], 18); *st.add(20) = r(s(2) ^ t[2], 62);
    *st.add(2) = r(s(12) ^ t[2], 43); *st.add(12) = r(s(13) ^ t[3], 25);
    *st.add(13) = r(s(19) ^ t[4], 8); *st.add(19) = r(s(23) ^ t[3], 56);
    *st.add(23) = r(s(15) ^ t[0], 41); *st.add(15) = r(s(4) ^ t[4], 27);
    *st.add(4) = r(s(24) ^ t[4], 14); *st.add(24) = r(s(21) ^ t[1], 2);
    *st.add(21) = r(s(8) ^ t[3], 55); *st.add(8) = r(s(16) ^ t[1], 45);
    *st.add(16) = r(s(5) ^ t[0], 36); *st.add(5) = r(s(3) ^ t[3], 28);
    *st.add(3) = r(s(18) ^ t[3], 21); *st.add(18) = r(s(17) ^ t[2], 15);
    *st.add(17) = r(s(11) ^ t[1], 10); *st.add(11) = r(s(7) ^ t[2], 6);
    *st.add(7) = r(s(10) ^ t[0], 3); *st.add(10) = r(tt ^ t[1], 1);

    for base in [0usize, 5, 10, 15, 20] {
        bc[0] = !s(base + 1) & s(base + 2);
        bc[1] = !s(base + 2) & s(base + 3);
        bc[2] = !s(base + 3) & s(base + 4);
        bc[3] = !s(base + 4) & s(base);
        bc[4] = !s(base) & s(base + 1);
        for j in 0..5 { *st.add(base + j) ^= bc[j]; }
    }
    *st.add(0) ^= SHA3_KECCAKF_RNDC[round as usize];
}

/* Generic implementation of the Keccak-f[1600] permutation */
unsafe fn sha3_keccakf_generic(state: &mut sha3_state) {
    for i in 0..25 { state.native_words[i] = u64::from_le(state.words[i]); }
    for round in 0..SHA3_KECCAK_ROUNDS { sha3_keccakf_one_round_generic(state.native_words.as_mut_ptr(), round as i32); }
    for i in 0..25 { state.words[i] = state.native_words[i].to_le(); }
}

/* Generic implementation of absorbing full blocks into Keccak. */
unsafe fn sha3_absorb_blocks_generic(state: &mut sha3_state, mut data: *const u8, mut nblocks: usize, block_size: usize) {
    loop {
        for i in (0..block_size).step_by(8) { state.words[i / 8] ^= (data.add(i) as *const u64).read_unaligned(); }
        sha3_keccakf_generic(state); data = data.add(block_size); nblocks -= 1;
        if nblocks == 0 { break; }
    }
}

// Architecture-specific implementations may replace these aliases.
unsafe fn sha3_keccakf(state: &mut sha3_state) { sha3_keccakf_generic(state); }
unsafe fn sha3_absorb_blocks(state: &mut sha3_state, data: *const u8, nblocks: usize, block_size: usize) { sha3_absorb_blocks_generic(state, data, nblocks, block_size); }

pub unsafe fn __sha3_update(ctx: &mut __sha3_ctx, mut input: *const u8, mut in_len: usize) {
    let block_size = ctx.block_size;
    let mut absorb_offset = ctx.absorb_offset;
    if absorb_offset != 0 && absorb_offset + in_len >= block_size {
        crypto_xor(ctx.state.bytes.as_mut_ptr().add(absorb_offset), input, block_size - absorb_offset);
        input = input.add(block_size - absorb_offset); in_len -= block_size - absorb_offset;
        sha3_keccakf(&mut ctx.state); absorb_offset = 0;
    }
    if in_len >= block_size { let nblocks = in_len / block_size; sha3_absorb_blocks(&mut ctx.state, input, nblocks, block_size); input = input.add(nblocks * block_size); in_len -= nblocks * block_size; }
    if in_len != 0 { crypto_xor(ctx.state.bytes.as_mut_ptr().add(absorb_offset), input, in_len); absorb_offset += in_len; }
    ctx.absorb_offset = absorb_offset;
}

pub unsafe fn sha3_final(sha3_ctx: &mut sha3_ctx, out: *mut u8) {
    let ctx = &mut sha3_ctx.ctx;
    ctx.state.bytes[ctx.absorb_offset] ^= 0x06;
    ctx.state.bytes[ctx.block_size - 1] ^= 0x80;
    sha3_keccakf(&mut ctx.state);
    core::ptr::copy_nonoverlapping(ctx.state.bytes.as_ptr(), out, ctx.digest_size);
    sha3_zeroize_ctx(sha3_ctx);
}

pub unsafe fn shake_squeeze(shake_ctx: &mut shake_ctx, mut out: *mut u8, mut out_len: usize) {
    let ctx = &mut shake_ctx.ctx; let block_size = ctx.block_size; let mut squeeze_offset = ctx.squeeze_offset;
    if ctx.absorb_offset < block_size { ctx.state.bytes[ctx.absorb_offset] ^= 0x1f; ctx.state.bytes[block_size - 1] ^= 0x80; ctx.absorb_offset = block_size; squeeze_offset = block_size; }
    while out_len != 0 { if squeeze_offset == block_size { sha3_keccakf(&mut ctx.state); squeeze_offset = 0; } let copy = core::cmp::min(out_len, block_size - squeeze_offset); core::ptr::copy_nonoverlapping(ctx.state.bytes.as_ptr().add(squeeze_offset), out, copy); out = out.add(copy); out_len -= copy; squeeze_offset += copy; }
    ctx.squeeze_offset = squeeze_offset;
}

pub unsafe fn sha3_224(input: *const u8, len: usize, out: *mut u8) { let mut ctx = sha3_ctx::default(); sha3_224_init(&mut ctx); sha3_update(&mut ctx, input, len); sha3_final(&mut ctx, out); }
pub unsafe fn sha3_256(input: *const u8, len: usize, out: *mut u8) { let mut ctx = sha3_ctx::default(); sha3_256_init(&mut ctx); sha3_update(&mut ctx, input, len); sha3_final(&mut ctx, out); }
pub unsafe fn sha3_384(input: *const u8, len: usize, out: *mut u8) { let mut ctx = sha3_ctx::default(); sha3_384_init(&mut ctx); sha3_update(&mut ctx, input, len); sha3_final(&mut ctx, out); }
pub unsafe fn sha3_512(input: *const u8, len: usize, out: *mut u8) { let mut ctx = sha3_ctx::default(); sha3_512_init(&mut ctx); sha3_update(&mut ctx, input, len); sha3_final(&mut ctx, out); }
pub unsafe fn shake128(input: *const u8, len: usize, out: *mut u8, out_len: usize) { let mut ctx = shake_ctx::default(); shake128_init(&mut ctx); shake_update(&mut ctx, input, len); shake_squeeze(&mut ctx, out, out_len); shake_zeroize_ctx(&mut ctx); }
pub unsafe fn shake256(input: *const u8, len: usize, out: *mut u8, out_len: usize) { let mut ctx = shake_ctx::default(); shake256_init(&mut ctx); shake_update(&mut ctx, input, len); shake_squeeze(&mut ctx, out, out_len); shake_zeroize_ctx(&mut ctx); }

// Module metadata and FIPS initialization are supplied by the kernel build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
