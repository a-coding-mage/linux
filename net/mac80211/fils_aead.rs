// SPDX-License-Identifier: GPL-2.0-only
/*
 * FILS AEAD for (Re)Association Request/Response frames
 * Copyright 2016, Qualcomm Atheros, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

const AES_BLOCK_SIZE: usize = 16;

unsafe fn gf_mulx(pad: *mut u8) {
    let a = u64::from_be_bytes(*(pad as *const [u8; 8]));
    let b = u64::from_be_bytes(*(pad.add(8) as *const [u8; 8]));
    *(pad as *mut [u8; 8]) = ((a << 1) | (b >> 63)).to_be_bytes();
    *(pad.add(8) as *mut [u8; 8]) = ((b << 1) ^ if (a >> 63) != 0 { 0x87 } else { 0 }).to_be_bytes();
}

unsafe fn aes_s2v(
    in_key: *const u8, key_len: usize, num_elem: usize,
    addr: *const *const u8, len: *const usize, v: *mut u8,
) -> i32 {
    let mut d = [0u8; AES_BLOCK_SIZE];
    let mut tmp = [0u8; AES_BLOCK_SIZE];
    let mut key = core::mem::MaybeUninit::<aes_cmac_key>::uninit();
    let mut ctx = core::mem::MaybeUninit::<aes_cmac_ctx>::uninit();
    let mut res: i32;

    res = aes_cmac_preparekey(key.as_mut_ptr(), in_key, key_len);
    if res != 0 { return res; }
    aes_cmac(key.as_ptr(), tmp.as_ptr(), AES_BLOCK_SIZE, d.as_mut_ptr());

    let mut i = 0usize;
    while i < num_elem - 1 {
        gf_mulx(d.as_mut_ptr());
        aes_cmac(key.as_ptr(), *addr.add(i), *len.add(i), tmp.as_mut_ptr());
        crypto_xor(d.as_mut_ptr(), tmp.as_ptr(), AES_BLOCK_SIZE);
        i += 1;
    }

    aes_cmac_init(ctx.as_mut_ptr(), key.as_ptr());
    if *len.add(i) >= AES_BLOCK_SIZE {
        aes_cmac_update(ctx.as_mut_ptr(), *addr.add(i), *len.add(i) - AES_BLOCK_SIZE);
        crypto_xor(d.as_mut_ptr(), (*addr.add(i)).add(*len.add(i) - AES_BLOCK_SIZE), AES_BLOCK_SIZE);
    } else {
        gf_mulx(d.as_mut_ptr());
        crypto_xor(d.as_mut_ptr(), *addr.add(i), *len.add(i));
        d[*len.add(i)] ^= 0x80;
    }
    aes_cmac_update(ctx.as_mut_ptr(), d.as_ptr(), AES_BLOCK_SIZE);
    aes_cmac_final(ctx.as_mut_ptr(), v);
    0
}

unsafe fn aes_siv_encrypt(
    key: *const u8, key_len_in: usize, plain: *const u8, plain_len: usize,
    mut num_elem: usize, addr: *mut *const u8, len: *mut usize, out: *mut u8,
) -> i32 {
    let mut v = [0u8; AES_BLOCK_SIZE];
    let mut key_len = key_len_in / 2;
    *addr.add(num_elem) = plain;
    *len.add(num_elem) = plain_len;
    num_elem += 1;
    let mut res = aes_s2v(key, key_len, num_elem, addr, len, v.as_mut_ptr());
    if res != 0 { return res; }
    let tmp = kmemdup(plain, plain_len, GFP_KERNEL);
    if tmp.is_null() { return -ENOMEM; }
    core::ptr::copy_nonoverlapping(v.as_ptr(), out, AES_BLOCK_SIZE);
    v[8] &= 0x7f;
    v[12] &= 0x7f;
    let tfm2 = crypto_alloc_skcipher(c"ctr(aes)".as_ptr(), 0, CRYPTO_ALG_ASYNC);
    if is_err(tfm2) { kfree(tmp); return ptr_err(tfm2); }
    res = crypto_skcipher_setkey(tfm2, key.add(key_len), key_len);
    if res == 0 {
        let req = skcipher_request_alloc(tfm2, GFP_KERNEL);
        if req.is_null() { res = -ENOMEM; }
        else {
            let mut src = core::mem::MaybeUninit::<scatterlist>::uninit();
            let mut dst = core::mem::MaybeUninit::<scatterlist>::uninit();
            sg_init_one(src.as_mut_ptr(), tmp, plain_len);
            sg_init_one(dst.as_mut_ptr(), out.add(AES_BLOCK_SIZE), plain_len);
            skcipher_request_set_crypt(req, src.as_mut_ptr(), dst.as_mut_ptr(), plain_len, v.as_mut_ptr());
            res = crypto_skcipher_encrypt(req);
            skcipher_request_free(req);
        }
    }
    kfree(tmp);
    crypto_free_skcipher(tfm2);
    res
}

unsafe fn aes_siv_decrypt(
    key: *const u8, key_len_in: usize, iv_crypt: *const u8, iv_c_len: usize,
    mut num_elem: usize, addr: *mut *const u8, len: *mut usize, out: *mut u8,
) -> i32 {
    let crypt_len = iv_c_len - AES_BLOCK_SIZE;
    let key_len = key_len_in / 2;
    *addr.add(num_elem) = out;
    *len.add(num_elem) = crypt_len;
    num_elem += 1;
    let mut frame_iv = [0u8; AES_BLOCK_SIZE];
    let mut iv = [0u8; AES_BLOCK_SIZE];
    let mut check = [0u8; AES_BLOCK_SIZE];
    core::ptr::copy_nonoverlapping(iv_crypt, iv.as_mut_ptr(), AES_BLOCK_SIZE);
    core::ptr::copy_nonoverlapping(iv_crypt, frame_iv.as_mut_ptr(), AES_BLOCK_SIZE);
    iv[8] &= 0x7f; iv[12] &= 0x7f;
    let tfm2 = crypto_alloc_skcipher(c"ctr(aes)".as_ptr(), 0, CRYPTO_ALG_ASYNC);
    if is_err(tfm2) { return ptr_err(tfm2); }
    let mut res = crypto_skcipher_setkey(tfm2, key.add(key_len), key_len);
    if res == 0 {
        let req = skcipher_request_alloc(tfm2, GFP_KERNEL);
        if req.is_null() { res = -ENOMEM; }
        else {
            let mut src = core::mem::MaybeUninit::<scatterlist>::uninit();
            let mut dst = core::mem::MaybeUninit::<scatterlist>::uninit();
            sg_init_one(src.as_mut_ptr(), iv_crypt.add(AES_BLOCK_SIZE), crypt_len);
            sg_init_one(dst.as_mut_ptr(), out, crypt_len);
            skcipher_request_set_crypt(req, src.as_mut_ptr(), dst.as_mut_ptr(), crypt_len, iv.as_mut_ptr());
            res = crypto_skcipher_decrypt(req);
            skcipher_request_free(req);
        }
    }
    crypto_free_skcipher(tfm2);
    if res != 0 { return res; }
    res = aes_s2v(key, key_len, num_elem, addr, len, check.as_mut_ptr());
    if res != 0 { return res; }
    if core::slice::from_raw_parts(check.as_ptr(), AES_BLOCK_SIZE) != core::slice::from_raw_parts(frame_iv.as_ptr(), AES_BLOCK_SIZE) { return -EINVAL; }
    0
}

pub unsafe fn fils_encrypt_assoc_req(skb: *mut sk_buff, assoc_data: *mut ieee80211_mgd_assoc_data) -> i32 {
    let mgmt = (*skb).data as *mut ieee80211_mgmt;
    let (capab, ies) = if ieee80211_is_reassoc_req((*mgmt).frame_control) { (&mut (*mgmt).u.reassoc_req.capab_info as *mut _ as *mut u8, (*mgmt).u.reassoc_req.variable.as_mut_ptr()) } else { (&mut (*mgmt).u.assoc_req.capab_info as *mut _ as *mut u8, (*mgmt).u.assoc_req.variable.as_mut_ptr()) };
    let session = cfg80211_find_ext_elem(WLAN_EID_EXT_FILS_SESSION, ies, (*skb).data.add((*skb).len) as usize - ies as usize);
    if session.is_null() || (*session).datalen != 1 + 8 { return -EINVAL; }
    let encr = (*session).data.add(1 + 8);
    let mut addr = [core::ptr::null(); 6]; let mut len = [0usize; 6];
    addr[0] = (*mgmt).sa.as_ptr(); len[0] = ETH_ALEN; addr[1] = (*mgmt).da.as_ptr(); len[1] = ETH_ALEN;
    addr[2] = (*assoc_data).fils_nonces.as_ptr(); len[2] = FILS_NONCE_LEN; addr[3] = (*assoc_data).fils_nonces.as_ptr().add(FILS_NONCE_LEN); len[3] = FILS_NONCE_LEN;
    addr[4] = capab; len[4] = encr as usize - capab as usize;
    let crypt_len = (*skb).data.add((*skb).len) as usize - encr as usize; skb_put(skb, AES_BLOCK_SIZE);
    aes_siv_encrypt((*assoc_data).fils_kek.as_ptr(), (*assoc_data).fils_kek_len, encr, crypt_len, 5, addr.as_mut_ptr(), len.as_mut_ptr(), encr)
}

pub unsafe fn fils_decrypt_assoc_resp(sdata: *mut ieee80211_sub_if_data, frame: *mut u8, frame_len: *mut usize, assoc_data: *mut ieee80211_mgd_assoc_data) -> i32 {
    let mgmt = frame as *mut ieee80211_mgmt;
    if *frame_len < 24 + 6 { return -EINVAL; }
    let capab = &mut (*mgmt).u.assoc_resp.capab_info as *mut _ as *mut u8;
    let ies = (*mgmt).u.assoc_resp.variable.as_mut_ptr();
    let session = cfg80211_find_ext_elem(WLAN_EID_EXT_FILS_SESSION, ies, frame.add(*frame_len) as usize - ies as usize);
    if session.is_null() || (*session).datalen != 1 + 8 { mlme_dbg(sdata, c"No (valid) FILS Session element in (Re)Association Response frame from %pM", (*mgmt).sa.as_ptr()); return -EINVAL; }
    let encr = (*session).data.add(1 + 8); let mut addr = [core::ptr::null(); 6]; let mut len = [0usize; 6];
    addr[0] = (*mgmt).sa.as_ptr(); len[0] = ETH_ALEN; addr[1] = (*mgmt).da.as_ptr(); len[1] = ETH_ALEN; addr[2] = (*assoc_data).fils_nonces.as_ptr().add(FILS_NONCE_LEN); len[2] = FILS_NONCE_LEN; addr[3] = (*assoc_data).fils_nonces.as_ptr(); len[3] = FILS_NONCE_LEN; addr[4] = capab; len[4] = encr as usize - capab as usize;
    let crypt_len = frame.add(*frame_len) as usize - encr as usize; if crypt_len < AES_BLOCK_SIZE { return -EINVAL; }
    let res = aes_siv_decrypt((*assoc_data).fils_kek.as_ptr(), (*assoc_data).fils_kek_len, encr, crypt_len, 5, addr.as_mut_ptr(), len.as_mut_ptr(), encr);
    if res != 0 { return res; } *frame_len -= AES_BLOCK_SIZE; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
