// SPDX-License-Identifier: GPL-2.0-or-later
/* The AEGIS-128 Authenticated-Encryption Algorithm */

// External kernel and aegis definitions are supplied by the surrounding crate.

const AEGIS128_NONCE_SIZE: usize = 16;
const AEGIS128_STATE_BLOCKS: usize = 5;
const AEGIS128_KEY_SIZE: usize = 16;
const AEGIS128_MIN_AUTH_SIZE: usize = 8;
const AEGIS128_MAX_AUTH_SIZE: usize = 16;

#[repr(C)]
struct aegis_state {
    blocks: [aegis_block; AEGIS128_STATE_BLOCKS],
}

#[repr(C)]
struct aegis_ctx {
    key: aegis_block,
}

static mut have_simd: bool = false;

static crypto_aegis_const: [aegis_block; 2] = [
    aegis_block { words64: [0x0d08050302010100u64.to_le(), 0x6279e99059372215u64.to_le()] },
    aegis_block { words64: [0xf12fc26d55183ddbu64.to_le(), 0xdd28b57342311120u64.to_le()] },
];

unsafe fn aegis128_do_simd() -> bool {
    #[cfg(CONFIG_CRYPTO_AEGIS128_SIMD)]
    {
        if static_branch_likely(&have_simd) { return crypto_simd_usable(); }
    }
    false
}

unsafe fn crypto_aegis128_update(state: *mut aegis_state) {
    let mut tmp = (*state).blocks[AEGIS128_STATE_BLOCKS - 1];
    let mut i = AEGIS128_STATE_BLOCKS - 1;
    while i > 0 {
        crypto_aegis_aesenc(&mut (*state).blocks[i], &(*state).blocks[i - 1], &(*state).blocks[i]);
        i -= 1;
    }
    crypto_aegis_aesenc(&mut (*state).blocks[0], &tmp, &(*state).blocks[0]);
}

unsafe fn crypto_aegis128_update_a(state: *mut aegis_state, msg: *const aegis_block, do_simd: bool) {
    if cfg!(CONFIG_CRYPTO_AEGIS128_SIMD) && do_simd { crypto_aegis128_update_simd(state, msg); return; }
    crypto_aegis128_update(state);
    crypto_aegis_block_xor(&mut (*state).blocks[0], msg);
}

unsafe fn crypto_aegis128_update_u(state: *mut aegis_state, msg: *const u8, do_simd: bool) {
    if cfg!(CONFIG_CRYPTO_AEGIS128_SIMD) && do_simd { crypto_aegis128_update_simd(state, msg); return; }
    crypto_aegis128_update(state);
    crypto_xor((*state).blocks[0].bytes.as_mut_ptr(), msg, AEGIS_BLOCK_SIZE);
}

unsafe fn crypto_aegis128_init(state: *mut aegis_state, key: *const aegis_block, iv: *const u8) {
    let mut key_iv = *key;
    crypto_xor(key_iv.bytes.as_mut_ptr(), iv, AEGIS_BLOCK_SIZE);
    (*state).blocks[0] = key_iv;
    (*state).blocks[1] = crypto_aegis_const[1];
    (*state).blocks[2] = crypto_aegis_const[0];
    (*state).blocks[3] = *key;
    (*state).blocks[4] = *key;
    crypto_aegis_block_xor(&mut (*state).blocks[3], &crypto_aegis_const[0]);
    crypto_aegis_block_xor(&mut (*state).blocks[4], &crypto_aegis_const[1]);
    for _ in 0..5 { crypto_aegis128_update_a(state, key, false); crypto_aegis128_update_a(state, &key_iv, false); }
}

unsafe fn crypto_aegis128_ad(state: *mut aegis_state, mut src: *const u8, mut size: usize, do_simd: bool) {
    if AEGIS_ALIGNED(src) {
        let mut src_blk = src as *const aegis_block;
        while size >= AEGIS_BLOCK_SIZE { crypto_aegis128_update_a(state, src_blk, do_simd); size -= AEGIS_BLOCK_SIZE; src_blk = src_blk.add(1); }
    } else {
        while size >= AEGIS_BLOCK_SIZE { crypto_aegis128_update_u(state, src, do_simd); size -= AEGIS_BLOCK_SIZE; src = src.add(AEGIS_BLOCK_SIZE); }
    }
}

unsafe fn crypto_aegis128_wipe_chunk(_state: *mut aegis_state, dst: *mut u8, _src: *const u8, size: usize) { memzero_explicit(dst, size); }

unsafe fn crypto_aegis128_encrypt_chunk(state: *mut aegis_state, mut dst: *mut u8, mut src: *const u8, mut size: usize) {
    let mut tmp: aegis_block;
    if AEGIS_ALIGNED(src) && AEGIS_ALIGNED(dst) {
        while size >= AEGIS_BLOCK_SIZE {
            let db = dst as *mut aegis_block; let sb = src as *const aegis_block;
            tmp = (*state).blocks[2]; crypto_aegis_block_and(&mut tmp, &(*state).blocks[3]); crypto_aegis_block_xor(&mut tmp, &(*state).blocks[4]); crypto_aegis_block_xor(&mut tmp, &(*state).blocks[1]); crypto_aegis_block_xor(&mut tmp, sb);
            crypto_aegis128_update_a(state, sb, false); *db = tmp; size -= AEGIS_BLOCK_SIZE; src = src.add(AEGIS_BLOCK_SIZE); dst = dst.add(AEGIS_BLOCK_SIZE);
        }
    } else { while size >= AEGIS_BLOCK_SIZE { tmp = (*state).blocks[2]; crypto_aegis_block_and(&mut tmp, &(*state).blocks[3]); crypto_aegis_block_xor(&mut tmp, &(*state).blocks[4]); crypto_aegis_block_xor(&mut tmp, &(*state).blocks[1]); crypto_xor(tmp.bytes.as_mut_ptr(), src, AEGIS_BLOCK_SIZE); crypto_aegis128_update_u(state, src, false); memcpy(dst, tmp.bytes.as_ptr(), AEGIS_BLOCK_SIZE); size -= AEGIS_BLOCK_SIZE; src = src.add(AEGIS_BLOCK_SIZE); dst = dst.add(AEGIS_BLOCK_SIZE); } }
    if size > 0 { let mut msg = aegis_block::zeroed(); memcpy(msg.bytes.as_mut_ptr(), src, size); tmp = (*state).blocks[2]; crypto_aegis_block_and(&mut tmp, &(*state).blocks[3]); crypto_aegis_block_xor(&mut tmp, &(*state).blocks[4]); crypto_aegis_block_xor(&mut tmp, &(*state).blocks[1]); crypto_aegis128_update_a(state, &msg, false); crypto_aegis_block_xor(&mut msg, &tmp); memcpy(dst, msg.bytes.as_ptr(), size); }
}

unsafe fn crypto_aegis128_decrypt_chunk(state: *mut aegis_state, mut dst: *mut u8, mut src: *const u8, mut size: usize) {
    let mut tmp: aegis_block;
    while size >= AEGIS_BLOCK_SIZE { tmp = (*state).blocks[2]; crypto_aegis_block_and(&mut tmp, &(*state).blocks[3]); crypto_aegis_block_xor(&mut tmp, &(*state).blocks[4]); crypto_aegis_block_xor(&mut tmp, &(*state).blocks[1]); crypto_xor(tmp.bytes.as_mut_ptr(), src, AEGIS_BLOCK_SIZE); crypto_aegis128_update_u(state, &tmp, false); memcpy(dst, tmp.bytes.as_ptr(), AEGIS_BLOCK_SIZE); size -= AEGIS_BLOCK_SIZE; src = src.add(AEGIS_BLOCK_SIZE); dst = dst.add(AEGIS_BLOCK_SIZE); }
    if size > 0 { let mut msg = aegis_block::zeroed(); memcpy(msg.bytes.as_mut_ptr(), src, size); tmp = (*state).blocks[2]; crypto_aegis_block_and(&mut tmp, &(*state).blocks[3]); crypto_aegis_block_xor(&mut tmp, &(*state).blocks[4]); crypto_aegis_block_xor(&mut tmp, &(*state).blocks[1]); crypto_aegis_block_xor(&mut msg, &tmp); memset(msg.bytes.as_mut_ptr().add(size), 0, AEGIS_BLOCK_SIZE - size); crypto_aegis128_update_a(state, &msg, false); memcpy(dst, msg.bytes.as_ptr(), size); }
}

// The remaining kernel-facing routines preserve the original external interfaces.
unsafe fn crypto_aegis128_final(state: *mut aegis_state, tag_xor: *mut aegis_block, assoclen: u64, cryptlen: u64) { let mut tmp = aegis_block::zeroed(); tmp.words64[0] = (assoclen * 8).to_le(); tmp.words64[1] = (cryptlen * 8).to_le(); crypto_aegis_block_xor(&mut tmp, &(*state).blocks[3]); for _ in 0..7 { crypto_aegis128_update_a(state, &tmp, false); } for i in 0..AEGIS128_STATE_BLOCKS { crypto_aegis_block_xor(tag_xor, &(*state).blocks[i]); } }

// Declarations and registration structures below are supplied by the kernel integration layer.
extern "C" {
    fn crypto_aegis128_init_simd(state: *mut aegis_state, key: *const aegis_block, iv: *const u8);
    fn crypto_aegis128_final_simd(state: *mut aegis_state, tag: *mut aegis_block, assoclen: u64, cryptlen: u64, authsize: usize) -> i32;
    fn crypto_aegis128_update_simd(state: *mut aegis_state, msg: *const aegis_block);
    fn crypto_aegis128_encrypt_chunk_simd(state: *mut aegis_state, dst: *mut u8, src: *const u8, size: usize);
    fn crypto_aegis128_decrypt_chunk_simd(state: *mut aegis_state, dst: *mut u8, src: *const u8, size: usize);
}

unsafe fn crypto_aegis128_process_ad(state: *mut aegis_state, sg_src: *mut scatterlist, mut assoclen: usize, do_simd: bool) {
    let mut walk = scatter_walk::zeroed(); let mut buf = aegis_block::zeroed(); let mut pos = 0usize;
    scatterwalk_start(&mut walk, sg_src);
    while assoclen != 0 { let size = scatterwalk_next(&mut walk, assoclen); let mut src = walk.addr; let mut left = size;
        if pos + size >= AEGIS_BLOCK_SIZE { if pos > 0 { let fill = AEGIS_BLOCK_SIZE - pos; memcpy(buf.bytes.as_mut_ptr().add(pos), src, fill); crypto_aegis128_update_a(state, &buf, do_simd); pos = 0; left -= fill; src = src.add(fill); } crypto_aegis128_ad(state, src, left, do_simd); src = src.add(left & !(AEGIS_BLOCK_SIZE - 1)); left &= AEGIS_BLOCK_SIZE - 1; }
        memcpy(buf.bytes.as_mut_ptr().add(pos), src, left); pos += left; assoclen -= size; scatterwalk_done_src(&mut walk, size);
    }
    if pos > 0 { memset(buf.bytes.as_mut_ptr().add(pos), 0, AEGIS_BLOCK_SIZE - pos); crypto_aegis128_update_a(state, &buf, do_simd); }
}

unsafe fn crypto_aegis128_process_crypt(state: *mut aegis_state, walk: *mut skcipher_walk, crypt: unsafe fn(*mut aegis_state, *mut u8, *const u8, usize)) -> i32 {
    let mut err = 0; while (*walk).nbytes != 0 { let mut nbytes = (*walk).nbytes; if nbytes < (*walk).total { nbytes = round_down(nbytes, (*walk).stride); } crypt(state, (*walk).dst.virt.addr, (*walk).src.virt.addr, nbytes); err = skcipher_walk_done(walk, (*walk).nbytes - nbytes); } err
}

unsafe fn crypto_aegis128_setkey(aead: *mut crypto_aead, key: *const u8, keylen: usize) -> i32 { if keylen != AEGIS128_KEY_SIZE { return -EINVAL; } memcpy((*crypto_aead_ctx(aead)).key.bytes.as_mut_ptr(), key, AEGIS128_KEY_SIZE); 0 }
unsafe fn crypto_aegis128_setauthsize(_tfm: *mut crypto_aead, authsize: usize) -> i32 { if authsize > AEGIS128_MAX_AUTH_SIZE || authsize < AEGIS128_MIN_AUTH_SIZE { -EINVAL } else { 0 } }

// Generic and SIMD request entry points retain the C control flow through the kernel ABI.
unsafe fn crypto_aegis128_encrypt_generic(req: *mut aead_request) -> i32 { let tfm = crypto_aead_reqtfm(req); let mut tag = aegis_block::zeroed(); let authsize = crypto_aead_authsize(tfm); let ctx = crypto_aead_ctx(tfm); let cryptlen = (*req).cryptlen; let mut walk = skcipher_walk::zeroed(); let mut state = aegis_state::zeroed(); skcipher_walk_aead_encrypt(&mut walk, req, false); crypto_aegis128_init(&mut state, &(*ctx).key, (*req).iv); crypto_aegis128_process_ad(&mut state, (*req).src, (*req).assoclen, false); crypto_aegis128_process_crypt(&mut walk, &mut walk, crypto_aegis128_encrypt_chunk); crypto_aegis128_final(&mut state, &mut tag, (*req).assoclen as u64, cryptlen as u64); scatterwalk_map_and_copy(tag.bytes.as_ptr(), (*req).dst, (*req).assoclen + cryptlen, authsize, 1); 0 }

// Kernel module initialization/exit and algorithm registration are intentionally represented as extern integration points.
extern "C" { fn crypto_aegis128_module_init() -> i32; fn crypto_aegis128_module_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
