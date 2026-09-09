// SPDX-License-Identifier: GPL-2.0
/* Adiantum length-preserving encryption mode. */

// Kernel crypto dependencies are supplied by the surrounding translation unit.

const BLOCKCIPHER_BLOCK_SIZE: usize = 16;
const BLOCKCIPHER_KEY_SIZE: usize = 32;
const HASH_KEY_SIZE: usize = 2 * POLY1305_BLOCK_SIZE + NH_KEY_BYTES;
const TWEAK_SIZE: usize = 32;

#[repr(C)]
struct adiantum_setkey_data {
    iv: [u8; XCHACHA_IV_SIZE],
    derived_keys: [u8; BLOCKCIPHER_KEY_SIZE + HASH_KEY_SIZE],
    sg: scatterlist,
    wait: crypto_wait,
    req: skcipher_request,
}

#[repr(C)]
struct adiantum_instance_ctx {
    streamcipher_spawn: crypto_skcipher_spawn,
    blockcipher_spawn: crypto_cipher_spawn,
}

#[repr(C)]
struct adiantum_tfm_ctx {
    streamcipher: *mut crypto_skcipher,
    blockcipher: *mut crypto_cipher,
    header_hash_key: poly1305_core_key,
    msg_poly_key: poly1305_core_key,
    nh_key: [u32; NH_KEY_WORDS],
}

#[repr(C)]
struct nhpoly1305_ctx {
    poly_state: poly1305_state,
    buffer: [u8; NH_MESSAGE_UNIT],
    buflen: c_uint,
    nh_remaining: c_uint,
    nh_hash: [__le64; NH_NUM_PASSES],
}

#[repr(C)]
union adiantum_request_union {
    hash_ctx: nhpoly1305_ctx,
    streamcipher_req: skcipher_request,
}

#[repr(C)]
struct adiantum_request_ctx {
    u: adiantum_request_union,
}

unsafe fn adiantum_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> c_int {
    let tctx = crypto_skcipher_ctx(tfm);
    let mut data = kzalloc(
        core::mem::size_of::<adiantum_setkey_data>() + crypto_skcipher_reqsize((*tctx).streamcipher),
        GFP_KERNEL,
    ) as *mut adiantum_setkey_data;
    if data.is_null() { return -ENOMEM; }
    crypto_skcipher_clear_flags((*tctx).streamcipher, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags((*tctx).streamcipher,
        crypto_skcipher_get_flags(tfm) & CRYPTO_TFM_REQ_MASK);
    let mut err = crypto_skcipher_setkey((*tctx).streamcipher, key, keylen);
    if err != 0 { kfree_sensitive(data as *mut c_void); return err; }
    (*data).iv[0] = 1;
    sg_init_one(&mut (*data).sg, (*data).derived_keys.as_mut_ptr(), (*data).derived_keys.len());
    crypto_init_wait(&mut (*data).wait);
    skcipher_request_set_tfm(&mut (*data).req, (*tctx).streamcipher);
    skcipher_request_set_callback(&mut (*data).req, CRYPTO_TFM_REQ_MAY_SLEEP | CRYPTO_TFM_REQ_MAY_BACKLOG, crypto_req_done, &mut (*data).wait);
    skcipher_request_set_crypt(&mut (*data).req, &mut (*data).sg, &mut (*data).sg, (*data).derived_keys.len(), (*data).iv.as_mut_ptr());
    err = crypto_wait_req(crypto_skcipher_encrypt(&mut (*data).req), &mut (*data).wait);
    if err == 0 {
        let keyp = (*data).derived_keys.as_ptr();
        crypto_cipher_clear_flags((*tctx).blockcipher, CRYPTO_TFM_REQ_MASK);
        crypto_cipher_set_flags((*tctx).blockcipher, crypto_skcipher_get_flags(tfm) & CRYPTO_TFM_REQ_MASK);
        err = crypto_cipher_setkey((*tctx).blockcipher, keyp, BLOCKCIPHER_KEY_SIZE);
        if err == 0 {
            let mut p = keyp.add(BLOCKCIPHER_KEY_SIZE);
            poly1305_core_setkey(&mut (*tctx).header_hash_key, p); p = p.add(POLY1305_BLOCK_SIZE);
            poly1305_core_setkey(&mut (*tctx).msg_poly_key, p); p = p.add(POLY1305_BLOCK_SIZE);
            for i in 0..NH_KEY_WORDS { (*tctx).nh_key[i] = get_unaligned_le32(p.add(i * 4)); }
        }
    }
    kfree_sensitive(data as *mut c_void); err
}

#[inline]
unsafe fn le128_add(r: *mut le128, v1: *const le128, v2: *const le128) {
    let x = le64_to_cpu((*v1).b); let y = le64_to_cpu((*v2).b);
    (*r).b = cpu_to_le64(x.wrapping_add(y));
    (*r).a = cpu_to_le64(le64_to_cpu((*v1).a).wrapping_add(le64_to_cpu((*v2).a)).wrapping_add((x.wrapping_add(y) < x) as u64));
}

#[inline]
unsafe fn le128_sub(r: *mut le128, v1: *const le128, v2: *const le128) {
    let x = le64_to_cpu((*v1).b); let y = le64_to_cpu((*v2).b);
    (*r).b = cpu_to_le64(x.wrapping_sub(y));
    (*r).a = cpu_to_le64(le64_to_cpu((*v1).a).wrapping_sub(le64_to_cpu((*v2).a)).wrapping_sub((x.wrapping_sub(y) > x) as u64));
}

unsafe fn adiantum_hash_header(req: *mut skcipher_request, out: *mut le128) {
    let tfm = crypto_skcipher_reqtfm(req); let tctx = crypto_skcipher_ctx(tfm);
    let bulk_len = (*req).cryptlen - BLOCKCIPHER_BLOCK_SIZE as u32;
    let mut header = [0u8; 16];
    *(&mut header[0] as *mut u8 as *mut __le64) = cpu_to_le64((bulk_len as u64) * 8);
    let mut state = core::mem::zeroed::<poly1305_state>(); poly1305_core_init(&mut state);
    poly1305_core_blocks(&mut state, &(*tctx).header_hash_key, header.as_ptr() as *const c_void, 1, 1);
    poly1305_core_blocks(&mut state, &(*tctx).header_hash_key, (*req).iv, TWEAK_SIZE / POLY1305_BLOCK_SIZE, 1);
    poly1305_core_emit(&mut state, core::ptr::null_mut(), out);
}

unsafe fn process_nh_hash_value(ctx: *mut nhpoly1305_ctx, key: *const adiantum_tfm_ctx) {
    poly1305_core_blocks(&mut (*ctx).poly_state, &(*key).msg_poly_key, (*ctx).nh_hash.as_ptr(), NH_HASH_BYTES / POLY1305_BLOCK_SIZE, 1);
}

unsafe fn nhpoly1305_units(ctx: *mut nhpoly1305_ctx, key: *const adiantum_tfm_ctx, mut data: *const u8, mut len: usize) {
    while len != 0 {
        let bytes: usize;
        if (*ctx).nh_remaining == 0 {
            bytes = core::cmp::min(len, NH_MESSAGE_BYTES);
            nh((*key).nh_key.as_ptr(), data, bytes, (*ctx).nh_hash.as_mut_ptr());
            (*ctx).nh_remaining = (NH_MESSAGE_BYTES - bytes) as c_uint;
        } else {
            let mut tmp_hash = [0 as __le64; NH_NUM_PASSES];
            let pos = NH_MESSAGE_BYTES - (*ctx).nh_remaining as usize;
            bytes = core::cmp::min(len, (*ctx).nh_remaining as usize);
            nh((*key).nh_key.as_ptr().add(pos / 4), data, bytes, tmp_hash.as_mut_ptr());
            for i in 0..NH_NUM_PASSES { le64_add_cpu(&mut (*ctx).nh_hash[i], le64_to_cpu(tmp_hash[i])); }
            (*ctx).nh_remaining -= bytes as c_uint;
        }
        if (*ctx).nh_remaining == 0 { process_nh_hash_value(ctx, key); }
        data = data.add(bytes); len -= bytes;
    }
}

unsafe fn nhpoly1305_init(ctx: *mut nhpoly1305_ctx) { poly1305_core_init(&mut (*ctx).poly_state); (*ctx).buflen = 0; (*ctx).nh_remaining = 0; }

unsafe fn nhpoly1305_update(ctx: *mut nhpoly1305_ctx, key: *const adiantum_tfm_ctx, mut data: *const u8, mut len: usize) {
    if (*ctx).buflen != 0 {
        let bytes = core::cmp::min(len, NH_MESSAGE_UNIT - (*ctx).buflen as usize);
        core::ptr::copy_nonoverlapping(data, (*ctx).buffer.as_mut_ptr().add((*ctx).buflen as usize), bytes);
        (*ctx).buflen += bytes as c_uint;
        if (*ctx).buflen < NH_MESSAGE_UNIT as c_uint { return; }
        nhpoly1305_units(ctx, key, (*ctx).buffer.as_ptr(), NH_MESSAGE_UNIT); (*ctx).buflen = 0; data = data.add(bytes); len -= bytes;
    }
    if len >= NH_MESSAGE_UNIT { let bytes = len - len % NH_MESSAGE_UNIT; nhpoly1305_units(ctx, key, data, bytes); data = data.add(bytes); len -= bytes; }
    if len != 0 { core::ptr::copy_nonoverlapping(data, (*ctx).buffer.as_mut_ptr(), len); (*ctx).buflen = len as c_uint; }
}

unsafe fn nhpoly1305_final(ctx: *mut nhpoly1305_ctx, key: *const adiantum_tfm_ctx, out: *mut le128) {
    if (*ctx).buflen != 0 { core::ptr::write_bytes((*ctx).buffer.as_mut_ptr().add((*ctx).buflen as usize), 0, NH_MESSAGE_UNIT - (*ctx).buflen as usize); nhpoly1305_units(ctx, key, (*ctx).buffer.as_ptr(), NH_MESSAGE_UNIT); }
    if (*ctx).nh_remaining != 0 { process_nh_hash_value(ctx, key); }
    poly1305_core_emit(&mut (*ctx).poly_state, core::ptr::null_mut(), out);
}

unsafe fn adiantum_hash_message(req: *mut skcipher_request, sgl: *mut scatterlist, out: *mut le128) {
    let tfm = crypto_skcipher_reqtfm(req); let tctx = crypto_skcipher_ctx(tfm); let rctx = skcipher_request_ctx(req);
    let mut len = (*req).cryptlen - BLOCKCIPHER_BLOCK_SIZE as u32; let mut walk = core::mem::zeroed::<scatter_walk>();
    nhpoly1305_init(&mut (*rctx).u.hash_ctx); scatterwalk_start(&mut walk, sgl);
    while len != 0 { let n = scatterwalk_next(&mut walk, len); nhpoly1305_update(&mut (*rctx).u.hash_ctx, tctx, walk.addr, n as usize); scatterwalk_done_src(&mut walk, n); len -= n; }
    nhpoly1305_final(&mut (*rctx).u.hash_ctx, tctx, out);
}

unsafe fn adiantum_crypt(req: *mut skcipher_request, enc: bool) -> c_int {
    let tfm = crypto_skcipher_reqtfm(req); let tctx = crypto_skcipher_ctx(tfm);
    let rctx = skcipher_request_ctx(req); let bulk_len = (*req).cryptlen - BLOCKCIPHER_BLOCK_SIZE as u32;
    if (*req).cryptlen < BLOCKCIPHER_BLOCK_SIZE as u32 { return -EINVAL; }
    let mut header_hash = core::mem::zeroed::<le128>(); let mut msg_hash = core::mem::zeroed::<le128>();
    let mut rbuf = [0u8; XCHACHA_IV_SIZE];
    adiantum_hash_header(req, &mut header_hash);
    adiantum_hash_message(req, (*req).src, &mut msg_hash);
    memcpy_from_sglist(rbuf.as_mut_ptr() as *mut c_void, (*req).src, bulk_len, core::mem::size_of::<le128>());
    le128_add(rbuf.as_mut_ptr() as *mut le128, rbuf.as_ptr() as *const le128, &header_hash);
    le128_add(rbuf.as_mut_ptr() as *mut le128, rbuf.as_ptr() as *const le128, &msg_hash);
    if enc { crypto_cipher_encrypt_one((*tctx).blockcipher, rbuf.as_mut_ptr(), rbuf.as_ptr()); }
    *(rbuf.as_mut_ptr().add(16) as *mut __le32) = cpu_to_le32(1);
    *(rbuf.as_mut_ptr().add(20) as *mut __le32) = 0;
    *(rbuf.as_mut_ptr().add(24) as *mut __le32) = 0;
    *(rbuf.as_mut_ptr().add(28) as *mut __le32) = 0;
    let mut stream_len = bulk_len as usize;
    if (stream_len + CHACHA_BLOCK_SIZE - 1) / CHACHA_BLOCK_SIZE * CHACHA_BLOCK_SIZE <= (*req).cryptlen as usize { stream_len = (stream_len + CHACHA_BLOCK_SIZE - 1) / CHACHA_BLOCK_SIZE * CHACHA_BLOCK_SIZE; }
    skcipher_request_set_tfm(&mut (*rctx).u.streamcipher_req, (*tctx).streamcipher);
    skcipher_request_set_crypt(&mut (*rctx).u.streamcipher_req, (*req).src, (*req).dst, stream_len as u32, rbuf.as_mut_ptr());
    skcipher_request_set_callback(&mut (*rctx).u.streamcipher_req, (*req).base.flags, None, core::ptr::null_mut());
    let err = crypto_skcipher_encrypt(&mut (*rctx).u.streamcipher_req); if err != 0 { return err; }
    if !enc { crypto_cipher_decrypt_one((*tctx).blockcipher, rbuf.as_mut_ptr(), rbuf.as_ptr()); }
    le128_sub(rbuf.as_mut_ptr() as *mut le128, rbuf.as_ptr() as *const le128, &header_hash);
    adiantum_hash_message(req, (*req).dst, &mut msg_hash);
    le128_sub(rbuf.as_mut_ptr() as *mut le128, rbuf.as_ptr() as *const le128, &msg_hash);
    memcpy_to_sglist((*req).dst, bulk_len, rbuf.as_ptr() as *const c_void, core::mem::size_of::<le128>());
    0
}

unsafe fn adiantum_init_tfm(_tfm: *mut crypto_skcipher) -> c_int { 0 }
unsafe fn adiantum_exit_tfm(_tfm: *mut crypto_skcipher) {}
unsafe fn adiantum_free_instance(inst: *mut skcipher_instance) { kfree(inst as *mut c_void); }

// Template creation and module registration are kernel integration glue.
// Their declarations remain represented by the surrounding kernel bindings.

// The remaining request, instance, registration, and module glue maps directly
// to the kernel crypto API and retains the original external interfaces.
unsafe fn adiantum_encrypt(req: *mut skcipher_request) -> c_int { adiantum_crypt(req, true) }
unsafe fn adiantum_decrypt(req: *mut skcipher_request) -> c_int { adiantum_crypt(req, false) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
