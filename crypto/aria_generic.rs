// SPDX-License-Identifier: GPL-2.0-or-later
/* Cryptographic API. ARIA Cipher Algorithm. */

// Dependencies supplied by the surrounding kernel translation.

static KEY_RC: [u32; 20] = [
    0x517cc1b7, 0x27220a94, 0xfe13abe8, 0xfa9a6ee0,
    0x6db14acc, 0x9e21c820, 0xff28b1d5, 0xef5de2b0,
    0xdb92371d, 0x2126e970, 0x03249775, 0x04e8c90e,
    0x517cc1b7, 0x27220a94, 0xfe13abe8, 0xfa9a6ee0,
    0x6db14acc, 0x9e21c820, 0xff28b1d5, 0xef5de2b0,
];

unsafe fn aria_set_encrypt_key(ctx: *mut aria_ctx, in_key: *const u8, key_len: u32) {
    let mut w0 = [0u32; 4]; let mut w1 = [0u32; 4];
    let mut w2 = [0u32; 4]; let mut w3 = [0u32; 4];
    let mut reg0; let mut reg1; let mut reg2; let mut reg3;
    let ck = KEY_RC.as_ptr().add(((key_len - 16) / 2) as usize);
    w0[0] = get_unaligned_be32(in_key.add(0)); w0[1] = get_unaligned_be32(in_key.add(4));
    w0[2] = get_unaligned_be32(in_key.add(8)); w0[3] = get_unaligned_be32(in_key.add(12));
    reg0 = w0[0] ^ *ck.add(0); reg1 = w0[1] ^ *ck.add(1);
    reg2 = w0[2] ^ *ck.add(2); reg3 = w0[3] ^ *ck.add(3);
    aria_subst_diff_odd(&mut reg0, &mut reg1, &mut reg2, &mut reg3);
    if key_len > 16 {
        w1[0] = get_unaligned_be32(in_key.add(16)); w1[1] = get_unaligned_be32(in_key.add(20));
        if key_len > 24 { w1[2] = get_unaligned_be32(in_key.add(24)); w1[3] = get_unaligned_be32(in_key.add(28)); }
    }
    w1[0] ^= reg0; w1[1] ^= reg1; w1[2] ^= reg2; w1[3] ^= reg3;
    reg0 = w1[0] ^ *ck.add(4); reg1 = w1[1] ^ *ck.add(5);
    reg2 = w1[2] ^ *ck.add(6); reg3 = w1[3] ^ *ck.add(7);
    aria_subst_diff_even(&mut reg0, &mut reg1, &mut reg2, &mut reg3);
    reg0 ^= w0[0]; reg1 ^= w0[1]; reg2 ^= w0[2]; reg3 ^= w0[3];
    w2 = [reg0, reg1, reg2, reg3];
    reg0 ^= *ck.add(8); reg1 ^= *ck.add(9); reg2 ^= *ck.add(10); reg3 ^= *ck.add(11);
    aria_subst_diff_odd(&mut reg0, &mut reg1, &mut reg2, &mut reg3);
    w3 = [reg0 ^ w1[0], reg1 ^ w1[1], reg2 ^ w1[2], reg3 ^ w1[3]];
    let mut r = 0usize;
    for (a, b, n) in [(&w0,&w1,19),(&w1,&w2,19),(&w2,&w3,19),(&w3,&w0,19),
                       (&w0,&w1,31),(&w1,&w2,31),(&w2,&w3,31),(&w3,&w0,31),
                       (&w0,&w1,67),(&w1,&w2,67),(&w2,&w3,67),(&w3,&w0,67)] {
        aria_gsrk((*ctx).enc_key[r], *a, *b, n); r += 1;
    }
    aria_gsrk((*ctx).enc_key[r], w0, w1, 97); r += 1;
    if key_len > 16 { aria_gsrk((*ctx).enc_key[r], w1, w2, 97); r += 1; aria_gsrk((*ctx).enc_key[r], w2, w3, 97); r += 1;
        if key_len > 24 { r += 1; aria_gsrk((*ctx).enc_key[r-1], w3, w0, 97); aria_gsrk((*ctx).enc_key[r], w0, w1, 109); }
    }
}

unsafe fn aria_set_decrypt_key(ctx: *mut aria_ctx) {
    for i in 0..4 { (*ctx).dec_key[0][i] = (*ctx).enc_key[(*ctx).rounds as usize][i]; (*ctx).dec_key[(*ctx).rounds as usize][i] = (*ctx).enc_key[0][i]; }
    for i in 1..(*ctx).rounds as usize { for j in 0..4 { (*ctx).dec_key[i][j] = aria_m((*ctx).enc_key[(*ctx).rounds as usize-i][j]); }
        aria_diff_word(&mut (*ctx).dec_key[i][0],&mut (*ctx).dec_key[i][1],&mut (*ctx).dec_key[i][2],&mut (*ctx).dec_key[i][3]);
        aria_diff_byte(&mut (*ctx).dec_key[i][1],&mut (*ctx).dec_key[i][2],&mut (*ctx).dec_key[i][3]);
        aria_diff_word(&mut (*ctx).dec_key[i][0],&mut (*ctx).dec_key[i][1],&mut (*ctx).dec_key[i][2],&mut (*ctx).dec_key[i][3]); }
}

pub unsafe fn aria_set_key(tfm: *mut crypto_tfm, in_key: *const u8, key_len: u32) -> i32 {
    let ctx = crypto_tfm_ctx(tfm); if key_len != 16 && key_len != 24 && key_len != 32 { return -22; }
    (*ctx).key_length = key_len; (*ctx).rounds = (key_len + 32) / 4;
    aria_set_encrypt_key(ctx, in_key, key_len); aria_set_decrypt_key(ctx); 0
}

unsafe fn __aria_crypt(ctx: *mut aria_ctx, out: *mut u8, input: *const u8, key: *mut [u32; 4]) {
    let mut r0=get_unaligned_be32(input); let mut r1=get_unaligned_be32(input.add(4)); let mut r2=get_unaligned_be32(input.add(8)); let mut r3=get_unaligned_be32(input.add(12));
    let rounds=(*ctx).rounds as usize; let mut i=0; aria_add_round_key(*key.add(i),&mut r0,&mut r1,&mut r2,&mut r3); i+=1;
    aria_subst_diff_odd(&mut r0,&mut r1,&mut r2,&mut r3); aria_add_round_key(*key.add(i),&mut r0,&mut r1,&mut r2,&mut r3); i+=1;
    let mut n=rounds; while { n-=2; n>0 } { aria_subst_diff_even(&mut r0,&mut r1,&mut r2,&mut r3); aria_add_round_key(*key.add(i),&mut r0,&mut r1,&mut r2,&mut r3); i+=1; aria_subst_diff_odd(&mut r0,&mut r1,&mut r2,&mut r3); aria_add_round_key(*key.add(i),&mut r0,&mut r1,&mut r2,&mut r3); i+=1; }
    r0=key.add(i).read()[0]^make_u32(x1[get_u8(r0,0) as usize] as u8,(x2[get_u8(r0,1) as usize]>>8) as u8,s1[get_u8(r0,2) as usize] as u8,s2[get_u8(r0,3) as usize] as u8);
    r1=key.add(i).read()[1]^make_u32(x1[get_u8(r1,0) as usize] as u8,(x2[get_u8(r1,1) as usize]>>8) as u8,s1[get_u8(r1,2) as usize] as u8,s2[get_u8(r1,3) as usize] as u8);
    r2=key.add(i).read()[2]^make_u32(x1[get_u8(r2,0) as usize] as u8,(x2[get_u8(r2,1) as usize]>>8) as u8,s1[get_u8(r2,2) as usize] as u8,s2[get_u8(r2,3) as usize] as u8);
    r3=key.add(i).read()[3]^make_u32(x1[get_u8(r3,0) as usize] as u8,(x2[get_u8(r3,1) as usize]>>8) as u8,s1[get_u8(r3,2) as usize] as u8,s2[get_u8(r3,3) as usize] as u8);
    put_unaligned_be32(r0,out); put_unaligned_be32(r1,out.add(4)); put_unaligned_be32(r2,out.add(8)); put_unaligned_be32(r3,out.add(12));
}

pub unsafe fn aria_encrypt(c:*mut core::ffi::c_void,o:*mut u8,i:*const u8){let x=c as *mut aria_ctx;__aria_crypt(x,o,i,(*x).enc_key.as_mut_ptr());}
pub unsafe fn aria_decrypt(c:*mut core::ffi::c_void,o:*mut u8,i:*const u8){let x=c as *mut aria_ctx;__aria_crypt(x,o,i,(*x).dec_key.as_mut_ptr());}

unsafe fn __aria_encrypt(tfm: *mut crypto_tfm, out: *mut u8, input: *const u8) {
    let ctx = crypto_tfm_ctx(tfm); __aria_crypt(ctx, out, input, (*ctx).enc_key.as_mut_ptr());
}
unsafe fn __aria_decrypt(tfm: *mut crypto_tfm, out: *mut u8, input: *const u8) {
    let ctx = crypto_tfm_ctx(tfm); __aria_crypt(ctx, out, input, (*ctx).dec_key.as_mut_ptr());
}

// Kernel registration metadata and module_init/module_exit hooks are supplied by
// the surrounding kernel translation; the original algorithm is "aria-generic".
unsafe fn aria_init() -> i32 { crypto_register_alg(&mut aria_alg) }
unsafe fn aria_fini() { crypto_unregister_alg(&mut aria_alg); }

static mut aria_alg: crypto_alg = crypto_alg {
    cra_name: "aria", cra_driver_name: "aria-generic", cra_priority: 100,
    cra_flags: CRYPTO_ALG_TYPE_CIPHER, cra_blocksize: ARIA_BLOCK_SIZE,
    cra_ctxsize: core::mem::size_of::<aria_ctx>(), cra_module: THIS_MODULE,
    cra_u: crypto_alg_union { cipher: crypto_cipher {
        cia_min_keysize: ARIA_MIN_KEY_SIZE, cia_max_keysize: ARIA_MAX_KEY_SIZE,
        cia_setkey: aria_set_key, cia_encrypt: __aria_encrypt, cia_decrypt: __aria_decrypt,
    }},
};

// MODULE_DESCRIPTION("ARIA Cipher Algorithm");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Taehee Yoo <ap420073@gmail.com>");
// MODULE_ALIAS_CRYPTO("aria");
// MODULE_ALIAS_CRYPTO("aria-generic");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
