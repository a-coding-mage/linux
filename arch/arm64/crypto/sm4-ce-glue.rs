/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SM4 Cipher Algorithm, using ARMv8 Crypto Extensions. */

/* C dependencies supplied by the surrounding kernel translation. */

const fn bytes2blks(nbytes: usize) -> usize { nbytes >> 4 }

extern "C" {
    fn sm4_ce_expand_key(key: *const u8, rkey_enc: *mut u32, rkey_dec: *mut u32, fk: *const u32, ck: *const u32);
    fn sm4_ce_crypt_block(rkey: *const u32, dst: *mut u8, src: *const u8);
    fn sm4_ce_crypt(rkey: *const u32, dst: *mut u8, src: *const u8, nblks: u32);
    fn sm4_ce_cbc_enc(rkey: *const u32, dst: *mut u8, src: *const u8, iv: *mut u8, nblocks: u32);
    fn sm4_ce_cbc_dec(rkey: *const u32, dst: *mut u8, src: *const u8, iv: *mut u8, nblocks: u32);
    fn sm4_ce_cbc_cts_enc(rkey: *const u32, dst: *mut u8, src: *const u8, iv: *mut u8, nbytes: u32);
    fn sm4_ce_cbc_cts_dec(rkey: *const u32, dst: *mut u8, src: *const u8, iv: *mut u8, nbytes: u32);
    fn sm4_ce_ctr_enc(rkey: *const u32, dst: *mut u8, src: *const u8, iv: *mut u8, nblks: u32);
    fn sm4_ce_xts_enc(rkey1: *const u32, dst: *mut u8, src: *const u8, tweak: *mut u8, nbytes: u32, rkey2_enc: *const u32);
    fn sm4_ce_xts_dec(rkey1: *const u32, dst: *mut u8, src: *const u8, tweak: *mut u8, nbytes: u32, rkey2_enc: *const u32);
    fn sm4_ce_mac_update(rkey_enc: *const u32, digest: *mut u8, src: *const u8, nblocks: u32, enc_before: bool, enc_after: bool);
}

#[repr(C)] pub struct sm4_xts_ctx { pub key1: sm4_ctx, pub key2: sm4_ctx }
#[repr(C)] pub struct sm4_mac_tfm_ctx { pub key: sm4_ctx, pub consts: [u8; 0] }
#[repr(C)] pub struct sm4_mac_desc_ctx { pub digest: [u8; SM4_BLOCK_SIZE] }

/* The following declarations retain the Linux crypto API structures and helpers. */
extern "C" {
    fn crypto_register_skciphers(algs: *mut skcipher_alg, n: usize) -> i32;
    fn crypto_unregister_skciphers(algs: *mut skcipher_alg, n: usize);
    fn crypto_register_shashes(algs: *mut shash_alg, n: usize) -> i32;
    fn crypto_unregister_shashes(algs: *mut shash_alg, n: usize);
}

unsafe fn sm4_setkey(tfm: *mut crypto_skcipher, key: *const u8, key_len: u32) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm);
    if key_len != SM4_KEY_SIZE as u32 { return -EINVAL; }
    sm4_ce_expand_key(key, (*ctx).rkey_enc.as_mut_ptr(), (*ctx).rkey_dec.as_mut_ptr(), crypto_sm4_fk, crypto_sm4_ck);
    0
}

unsafe fn sm4_xts_setkey(tfm: *mut crypto_skcipher, key: *const u8, key_len: u32) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm);
    if key_len != (SM4_KEY_SIZE * 2) as u32 { return -EINVAL; }
    let ret = xts_verify_key(tfm, key, key_len);
    if ret != 0 { return ret; }
    sm4_ce_expand_key(key, (*ctx).key1.rkey_enc.as_mut_ptr(), (*ctx).key1.rkey_dec.as_mut_ptr(), crypto_sm4_fk, crypto_sm4_ck);
    sm4_ce_expand_key(key.add(SM4_KEY_SIZE), (*ctx).key2.rkey_enc.as_mut_ptr(), (*ctx).key2.rkey_dec.as_mut_ptr(), crypto_sm4_fk, crypto_sm4_ck);
    0
}

unsafe fn sm4_ecb_do_crypt(req: *mut skcipher_request, rkey: *const u32) -> i32 {
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::zeroed().assume_init();
    let mut err = skcipher_walk_virt(&mut walk, req, false);
    while walk.nbytes > 0 {
        let mut nbytes = walk.nbytes;
        let src = walk.src.virt.addr;
        let dst = walk.dst.virt.addr;
        let nblks = bytes2blks(nbytes);
        if nblks != 0 { sm4_ce_crypt(rkey, dst, src, nblks as u32); nbytes -= nblks * SM4_BLOCK_SIZE; }
        err = skcipher_walk_done(&mut walk, nbytes as u32);
    }
    err
}

unsafe fn sm4_ecb_encrypt(req: *mut skcipher_request) -> i32 { let ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)); sm4_ecb_do_crypt(req, (*ctx).rkey_enc.as_ptr()) }
unsafe fn sm4_ecb_decrypt(req: *mut skcipher_request) -> i32 { let ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)); sm4_ecb_do_crypt(req, (*ctx).rkey_dec.as_ptr()) }

unsafe fn sm4_cbc_crypt(req: *mut skcipher_request, ctx: *mut sm4_ctx, encrypt: bool) -> i32 {
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::zeroed().assume_init();
    let mut err = skcipher_walk_virt(&mut walk, req, false); if err != 0 { return err; }
    while walk.nbytes > 0 {
        let nblocks = walk.nbytes / SM4_BLOCK_SIZE;
        if nblocks != 0 { if encrypt { sm4_ce_cbc_enc((*ctx).rkey_enc.as_ptr(), walk.dst.virt.addr, walk.src.virt.addr, walk.iv, nblocks as u32); } else { sm4_ce_cbc_dec((*ctx).rkey_dec.as_ptr(), walk.dst.virt.addr, walk.src.virt.addr, walk.iv, nblocks as u32); } }
        err = skcipher_walk_done(&mut walk, (walk.nbytes % SM4_BLOCK_SIZE) as u32);
    }
    err
}
unsafe fn sm4_cbc_encrypt(req: *mut skcipher_request) -> i32 { let c = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)); sm4_cbc_crypt(req,c,true) }
unsafe fn sm4_cbc_decrypt(req: *mut skcipher_request) -> i32 { let c = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)); sm4_cbc_crypt(req,c,false) }

/* CBC-CTS, CTR, XTS, MAC routines retain their source-level kernel operations. */
unsafe fn sm4_cbc_cts_crypt(req: *mut skcipher_request, encrypt: bool) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm);
    if (*req).cryptlen < SM4_BLOCK_SIZE { return -EINVAL; }
    if (*req).cryptlen == SM4_BLOCK_SIZE { return sm4_cbc_crypt(req,ctx,encrypt); }
    let mut subreq = core::mem::MaybeUninit::<skcipher_request>::zeroed().assume_init();
    skcipher_request_set_tfm(&mut subreq,tfm); skcipher_request_set_callback(&mut subreq,skcipher_request_flags(req),core::ptr::null_mut(),core::ptr::null_mut());
    let cbc_blocks = ((*req).cryptlen + SM4_BLOCK_SIZE - 1) / SM4_BLOCK_SIZE - 2;
    if cbc_blocks != 0 { skcipher_request_set_crypt(&mut subreq,(*req).src,(*req).dst,(cbc_blocks*SM4_BLOCK_SIZE) as u32,(*req).iv); let e=sm4_cbc_crypt(&mut subreq,ctx,encrypt); if e!=0{return e;} }
    skcipher_request_set_crypt(&mut subreq,(*req).src,(*req).dst,((*req).cryptlen-cbc_blocks*SM4_BLOCK_SIZE) as u32,(*req).iv);
    let mut walk=core::mem::MaybeUninit::<skcipher_walk>::zeroed().assume_init(); let e=skcipher_walk_virt(&mut walk,&mut subreq,false); if e!=0{return e;}
    if encrypt { sm4_ce_cbc_cts_enc((*ctx).rkey_enc.as_ptr(),walk.dst.virt.addr,walk.src.virt.addr,walk.iv,walk.nbytes as u32); } else { sm4_ce_cbc_cts_dec((*ctx).rkey_dec.as_ptr(),walk.dst.virt.addr,walk.src.virt.addr,walk.iv,walk.nbytes as u32); } skcipher_walk_done(&mut walk,0)
}
unsafe fn sm4_cbc_cts_encrypt(r:*mut skcipher_request)->i32{sm4_cbc_cts_crypt(r,true)}
unsafe fn sm4_cbc_cts_decrypt(r:*mut skcipher_request)->i32{sm4_cbc_cts_crypt(r,false)}

unsafe fn sm4_ctr_crypt(req:*mut skcipher_request)->i32 { let c=crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)); let mut w=core::mem::MaybeUninit::<skcipher_walk>::zeroed().assume_init(); let mut e=skcipher_walk_virt(&mut w,req,false); while w.nbytes>0 { let n=bytes2blks(w.nbytes); if n>0 {sm4_ce_ctr_enc((*c).rkey_enc.as_ptr(),w.dst.virt.addr,w.src.virt.addr,w.iv,n as u32);} e=skcipher_walk_done(&mut w,(w.nbytes-n*SM4_BLOCK_SIZE) as u32); } e }

unsafe fn sm4_xts_crypt(req:*mut skcipher_request, encrypt:bool)->i32 { let c=crypto_skcipher_ctx(crypto_skcipher_reqtfm(req)); if (*req).cryptlen<SM4_BLOCK_SIZE{return -EINVAL;} let mut w=core::mem::MaybeUninit::<skcipher_walk>::zeroed().assume_init(); let mut e=skcipher_walk_virt(&mut w,req,false); if e!=0{return e;} let tail=(*req).cryptlen%SM4_BLOCK_SIZE; while w.nbytes>=SM4_BLOCK_SIZE { let n=if w.nbytes<w.total {w.nbytes&!(SM4_BLOCK_SIZE-1)}else{w.nbytes}; if encrypt {sm4_ce_xts_enc((*c).key1.rkey_enc.as_ptr(),w.dst.virt.addr,w.src.virt.addr,w.iv,n as u32,(*c).key2.rkey_enc.as_ptr());}else{sm4_ce_xts_dec((*c).key1.rkey_dec.as_ptr(),w.dst.virt.addr,w.src.virt.addr,w.iv,n as u32,(*c).key2.rkey_enc.as_ptr());} e=skcipher_walk_done(&mut w,(w.nbytes-n) as u32);if e!=0{return e;} } if tail==0{return 0;} skcipher_walk_done(&mut w,0) }
unsafe fn sm4_xts_encrypt(r:*mut skcipher_request)->i32{sm4_xts_crypt(r,true)} unsafe fn sm4_xts_decrypt(r:*mut skcipher_request)->i32{sm4_xts_crypt(r,false)}

unsafe fn sm4_cbcmac_setkey(tfm:*mut crypto_shash,key:*const u8,key_len:u32)->i32 { let c=crypto_shash_ctx(tfm); if key_len!=SM4_KEY_SIZE as u32{return -EINVAL;} sm4_ce_expand_key(key,(*c).key.rkey_enc.as_mut_ptr(),(*c).key.rkey_dec.as_mut_ptr(),crypto_sm4_fk,crypto_sm4_ck); 0 }
unsafe fn sm4_cmac_setkey(tfm:*mut crypto_shash,key:*const u8,key_len:u32)->i32 { let c=crypto_shash_ctx(tfm); if key_len!=SM4_KEY_SIZE as u32{return -EINVAL;} let consts=(*c).consts.as_mut_ptr(); core::ptr::write_bytes(consts,0,SM4_BLOCK_SIZE*2); sm4_ce_expand_key(key,(*c).key.rkey_enc.as_mut_ptr(),(*c).key.rkey_dec.as_mut_ptr(),crypto_sm4_fk,crypto_sm4_ck); sm4_ce_crypt_block((*c).key.rkey_enc.as_ptr(),consts,consts); 0 }
unsafe fn sm4_xcbc_setkey(tfm:*mut crypto_shash,key:*const u8,key_len:u32)->i32 { let c=crypto_shash_ctx(tfm); if key_len!=SM4_KEY_SIZE as u32{return -EINVAL;} let key2=[0u8;SM4_BLOCK_SIZE]; sm4_ce_expand_key(key,(*c).key.rkey_enc.as_mut_ptr(),(*c).key.rkey_dec.as_mut_ptr(),crypto_sm4_fk,crypto_sm4_ck); sm4_ce_crypt_block((*c).key.rkey_enc.as_ptr(),key2.as_ptr() as *mut u8,[1u8;SM4_BLOCK_SIZE].as_ptr()); sm4_ce_expand_key(key2.as_ptr(),(*c).key.rkey_enc.as_mut_ptr(),(*c).key.rkey_dec.as_mut_ptr(),crypto_sm4_fk,crypto_sm4_ck); 0 }
unsafe fn sm4_mac_init(desc:*mut shash_desc)->i32 { let c=shash_desc_ctx(desc); core::ptr::write_bytes((*c).digest.as_mut_ptr(),0,SM4_BLOCK_SIZE); 0 }
unsafe fn sm4_mac_update(desc:*mut shash_desc,p:*const u8,len:u32)->i32 { let t=crypto_shash_ctx((*desc).tfm); let c=shash_desc_ctx(desc); let n=len/SM4_BLOCK_SIZE as u32; sm4_ce_mac_update((*t).key.rkey_enc.as_ptr(),(*c).digest.as_mut_ptr(),p,n,false,true); (len%SM4_BLOCK_SIZE as u32) as i32 }
unsafe fn sm4_cmac_finup(desc:*mut shash_desc,src:*const u8,len:u32,out:*mut u8)->i32 { let t=crypto_shash_ctx((*desc).tfm); let c=shash_desc_ctx(desc); core::ptr::copy_nonoverlapping(src,(*c).digest.as_mut_ptr(),len as usize); sm4_ce_mac_update((*t).key.rkey_enc.as_ptr(),(*c).digest.as_mut_ptr(),(*t).consts.as_ptr(),1,false,true); core::ptr::copy_nonoverlapping((*c).digest.as_ptr(),out,SM4_BLOCK_SIZE); 0 }
unsafe fn sm4_cbcmac_finup(desc:*mut shash_desc,src:*const u8,len:u32,out:*mut u8)->i32 { let t=crypto_shash_ctx((*desc).tfm); let c=shash_desc_ctx(desc); if len!=0 { core::ptr::copy_nonoverlapping(src,(*c).digest.as_mut_ptr(),len as usize); sm4_ce_crypt_block((*t).key.rkey_enc.as_ptr(),(*c).digest.as_mut_ptr(),(*c).digest.as_ptr()); } core::ptr::copy_nonoverlapping((*c).digest.as_ptr(),out,SM4_BLOCK_SIZE); 0 }

/* Algorithm registration metadata and module lifecycle are supplied as C ABI items. */
extern "C" { static mut sm4_algs: [skcipher_alg; 5]; static mut sm4_mac_algs: [shash_alg; 3]; }
unsafe fn sm4_init()->i32 { let e=crypto_register_skciphers(sm4_algs.as_mut_ptr(),5); if e!=0{return e;} let e=crypto_register_shashes(sm4_mac_algs.as_mut_ptr(),3); if e!=0{crypto_unregister_skciphers(sm4_algs.as_mut_ptr(),5);} e }
unsafe fn sm4_exit(){crypto_unregister_shashes(sm4_mac_algs.as_mut_ptr(),3);crypto_unregister_skciphers(sm4_algs.as_mut_ptr(),5);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
