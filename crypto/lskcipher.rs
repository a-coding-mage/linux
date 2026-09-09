// SPDX-License-Identifier: GPL-2.0-or-later
/* Linear symmetric key cipher operations. */

// Dependency declarations and build-time configuration are supplied by the surrounding kernel bindings.

#[inline]
unsafe fn __crypto_lskcipher_cast(tfm: *mut crypto_tfm) -> *mut crypto_lskcipher {
    container_of!(tfm, crypto_lskcipher, base)
}

#[inline]
unsafe fn __crypto_lskcipher_alg(alg: *mut crypto_alg) -> *mut lskcipher_alg {
    container_of!(alg, lskcipher_alg, co.base)
}

unsafe fn lskcipher_setkey_unaligned(tfm: *mut crypto_lskcipher, key: *const u8, keylen: u32) -> i32 {
    let alignmask = crypto_lskcipher_alignmask(tfm);
    let cipher = crypto_lskcipher_alg(tfm);
    let absize = keylen as usize + alignmask as usize;
    let buffer = kmalloc(absize, GFP_ATOMIC);
    if buffer.is_null() { return -ENOMEM; }
    let alignbuffer = align!(buffer as usize, alignmask as usize + 1) as *mut u8;
    memcpy(alignbuffer, key, keylen as usize);
    let ret = ((*(*cipher).setkey))(tfm, alignbuffer, keylen);
    kfree_sensitive(buffer);
    ret
}

pub unsafe fn crypto_lskcipher_setkey(tfm: *mut crypto_lskcipher, key: *const u8, keylen: u32) -> i32 {
    let alignmask = crypto_lskcipher_alignmask(tfm);
    let cipher = crypto_lskcipher_alg(tfm);
    if keylen < (*cipher).co.min_keysize || keylen > (*cipher).co.max_keysize { return -EINVAL; }
    if (key as usize & alignmask as usize) != 0 { lskcipher_setkey_unaligned(tfm, key, keylen) }
    else { ((*(*cipher).setkey))(tfm, key, keylen) }
}

unsafe fn crypto_lskcipher_crypt_unaligned(tfm: *mut crypto_lskcipher, mut src: *const u8, mut dst: *mut u8, mut len: u32, iv: *mut u8,
    crypt: unsafe extern "C" fn(*mut crypto_lskcipher, *const u8, *mut u8, u32, *mut u8, u32) -> i32) -> i32 {
    let statesize = crypto_lskcipher_statesize(tfm);
    let ivsize = crypto_lskcipher_ivsize(tfm);
    let bs = crypto_lskcipher_blocksize(tfm);
    let cs = crypto_lskcipher_chunksize(tfm);
    let tiv = kmalloc(PAGE_SIZE, GFP_ATOMIC) as *mut u8;
    if tiv.is_null() { return -ENOMEM; }
    memcpy(tiv, iv, (ivsize + statesize) as usize);
    let p = kmalloc(PAGE_SIZE, GFP_ATOMIC) as *mut u8;
    let mut err = -ENOMEM;
    if !p.is_null() {
        while len >= bs {
            let mut chunk = core::cmp::min(PAGE_SIZE as u32, len);
            if chunk > cs { chunk &= !(cs - 1); }
            memcpy(p, src, chunk as usize);
            err = crypt(tfm, p, p, chunk, tiv, CRYPTO_LSKCIPHER_FLAG_FINAL);
            if err != 0 { break; }
            memcpy(dst, p, chunk as usize);
            src = src.add(chunk as usize); dst = dst.add(chunk as usize); len -= chunk;
        }
        if err == 0 || len != 0 { err = if len != 0 { -EINVAL } else { 0 }; }
    }
    memcpy(iv, tiv, (ivsize + statesize) as usize);
    kfree_sensitive(p as *mut core::ffi::c_void); kfree_sensitive(tiv as *mut core::ffi::c_void);
    err
}

unsafe fn crypto_lskcipher_crypt(tfm: *mut crypto_lskcipher, src: *const u8, dst: *mut u8, len: u32, iv: *mut u8,
    crypt: unsafe extern "C" fn(*mut crypto_lskcipher, *const u8, *mut u8, u32, *mut u8, u32) -> i32) -> i32 {
    let mask = crypto_lskcipher_alignmask(tfm);
    if ((src as usize | dst as usize | iv as usize) & mask as usize) != 0 { crypto_lskcipher_crypt_unaligned(tfm, src, dst, len, iv, crypt) }
    else { crypt(tfm, src, dst, len, iv, CRYPTO_LSKCIPHER_FLAG_FINAL) }
}

pub unsafe fn crypto_lskcipher_encrypt(tfm: *mut crypto_lskcipher, src: *const u8, dst: *mut u8, len: u32, iv: *mut u8) -> i32 {
    let alg = crypto_lskcipher_alg(tfm); crypto_lskcipher_crypt(tfm, src, dst, len, iv, (*alg).encrypt)
}
pub unsafe fn crypto_lskcipher_decrypt(tfm: *mut crypto_lskcipher, src: *const u8, dst: *mut u8, len: u32, iv: *mut u8) -> i32 {
    let alg = crypto_lskcipher_alg(tfm); crypto_lskcipher_crypt(tfm, src, dst, len, iv, (*alg).decrypt)
}

unsafe fn crypto_lskcipher_exit_tfm(tfm: *mut crypto_tfm) {
    let skcipher = __crypto_lskcipher_cast(tfm); let alg = crypto_lskcipher_alg(skcipher); ((*alg).exit)(skcipher);
}

unsafe fn crypto_lskcipher_init_tfm(tfm: *mut crypto_tfm) -> i32 {
    let skcipher = __crypto_lskcipher_cast(tfm); let alg = crypto_lskcipher_alg(skcipher);
    if !(*alg).exit.is_none() { (*skcipher).base.exit = Some(crypto_lskcipher_exit_tfm); }
    if !(*alg).init.is_none() { ((*alg).init)(skcipher) } else { 0 }
}

unsafe fn crypto_lskcipher_free_instance(inst: *mut crypto_instance) {
    let skcipher = container_of!(inst, lskcipher_instance, s.base); ((*skcipher).free)(skcipher);
}

// The remaining type-registration and simple-instance routines retain the C ABI and kernel helper calls.
// Their declarations are intentionally expressed as direct Rust equivalents for the supplied bindings.

pub unsafe fn crypto_init_lskcipher_ops_sg(tfm: *mut crypto_tfm) -> i32 {
    let ctx = crypto_tfm_ctx(tfm) as *mut *mut crypto_lskcipher;
    let calg = (*tfm).__crt_alg;
    if !crypto_mod_get(calg) { return -EAGAIN; }
    let skcipher = crypto_create_tfm(calg, &crypto_lskcipher_type);
    if IS_ERR(skcipher) { crypto_mod_put(calg); return PTR_ERR(skcipher); }
    *ctx = skcipher; (*tfm).exit = Some(crypto_lskcipher_exit_tfm_sg); 0
}

unsafe fn crypto_lskcipher_exit_tfm_sg(tfm: *mut crypto_tfm) {
    let ctx = crypto_tfm_ctx(tfm) as *mut *mut crypto_lskcipher; crypto_free_lskcipher(*ctx);
}

pub unsafe fn crypto_grab_lskcipher(spawn: *mut crypto_lskcipher_spawn, inst: *mut crypto_instance, name: *const i8, type_: u32, mask: u32) -> i32 {
    (*spawn).base.frontend = &crypto_lskcipher_type; crypto_grab_spawn(&mut (*spawn).base, inst, name, type_, mask)
}
pub unsafe fn crypto_alloc_lskcipher(name: *const i8, type_: u32, mask: u32) -> *mut crypto_lskcipher { crypto_alloc_tfm(name, &crypto_lskcipher_type, type_, mask) }
pub unsafe fn crypto_unregister_lskcipher(alg: *mut lskcipher_alg) { crypto_unregister_alg(&mut (*alg).co.base); }
pub unsafe fn crypto_register_lskcipher(alg: *mut lskcipher_alg) -> i32 { let e = lskcipher_prepare_alg(alg); if e != 0 { e } else { crypto_register_alg(&mut (*alg).co.base) } }
pub unsafe fn crypto_unregister_lskciphers(algs: *mut lskcipher_alg, count: i32) { for i in (0..count).rev() { crypto_unregister_lskcipher(algs.add(i as usize)); } }
pub unsafe fn crypto_register_lskciphers(algs: *mut lskcipher_alg, count: i32) -> i32 { for i in 0..count { let e = crypto_register_lskcipher(algs.add(i as usize)); if e != 0 { crypto_unregister_lskciphers(algs, i); return e; } } 0 }

unsafe fn lskcipher_prepare_alg(alg: *mut lskcipher_alg) -> i32 {
    let e = skcipher_prepare_alg_common(&mut (*alg).co); if e != 0 { return e; }
    if (*alg).co.chunksize & ((*alg).co.chunksize - 1) != 0 { return -EINVAL; }
    (*alg).co.base.cra_type = &crypto_lskcipher_type; (*alg).co.base.cra_flags |= CRYPTO_ALG_TYPE_LSKCIPHER; 0
}

pub unsafe fn lskcipher_register_instance(tmpl: *mut crypto_template, inst: *mut lskcipher_instance) -> i32 {
    if WARN_ON((*inst).free.is_none()) { return -EINVAL; }
    let e = lskcipher_prepare_alg(&mut (*inst).alg); if e != 0 { return e; }
    crypto_register_instance(tmpl, lskcipher_crypto_instance(inst))
}

unsafe fn crypto_lskcipher_crypt_sg(req: *mut skcipher_request, crypt: unsafe extern "C" fn(*mut crypto_lskcipher,*const u8,*mut u8,u32,*mut u8,u32)->i32) -> i32 {
    let skcipher = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(skcipher) as *mut *mut crypto_lskcipher;
    let tfm = *ctx; let ivsize = crypto_lskcipher_ivsize(tfm); let mut flags = (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP;
    let ivs = ptr_align!(skcipher_request_ctx(req), crypto_skcipher_alignmask(skcipher)+1); memcpy(ivs, (*req).iv, ivsize as usize);
    if (*req).base.flags & CRYPTO_SKCIPHER_REQ_CONT != 0 { flags |= CRYPTO_LSKCIPHER_FLAG_CONT; }
    if (*req).base.flags & CRYPTO_SKCIPHER_REQ_NOTFINAL == 0 { flags |= CRYPTO_LSKCIPHER_FLAG_FINAL; }
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit(); let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, false);
    while (*walk.as_mut_ptr()).nbytes != 0 { let w = walk.as_mut_ptr(); err = crypt(tfm, (*w).src.virt.addr, (*w).dst.virt.addr, (*w).nbytes, ivs, flags & if (*w).nbytes == (*w).total { !CRYPTO_LSKCIPHER_FLAG_FINAL } else { u32::MAX }); err = skcipher_walk_done(w, err); flags |= CRYPTO_LSKCIPHER_FLAG_CONT; }
    memcpy((*req).iv, ivs, ivsize as usize); err
}

pub unsafe fn crypto_lskcipher_encrypt_sg(req: *mut skcipher_request) -> i32 { let s=crypto_skcipher_reqtfm(req); let c=crypto_skcipher_ctx(s) as *mut *mut crypto_lskcipher; let a=crypto_lskcipher_alg(*c); crypto_lskcipher_crypt_sg(req, (*a).encrypt) }
pub unsafe fn crypto_lskcipher_decrypt_sg(req: *mut skcipher_request) -> i32 { let s=crypto_skcipher_reqtfm(req); let c=crypto_skcipher_ctx(s) as *mut *mut crypto_lskcipher; let a=crypto_lskcipher_alg(*c); crypto_lskcipher_crypt_sg(req, (*a).decrypt) }

unsafe fn lskcipher_setkey_simple(tfm:*mut crypto_lskcipher,key:*const u8,keylen:u32)->i32 { let c=lskcipher_cipher_simple(tfm); crypto_lskcipher_clear_flags(c,CRYPTO_TFM_REQ_MASK); crypto_lskcipher_set_flags(c,crypto_lskcipher_get_flags(tfm)&CRYPTO_TFM_REQ_MASK); crypto_lskcipher_setkey(c,key,keylen) }
unsafe fn lskcipher_init_tfm_simple(tfm:*mut crypto_lskcipher)->i32 { let i=lskcipher_alg_instance(tfm); let c=crypto_spawn_lskcipher(lskcipher_instance_ctx(i)); if IS_ERR(c){return PTR_ERR(c)}; *(crypto_lskcipher_ctx(tfm) as *mut *mut crypto_lskcipher)=c;0 }
unsafe fn lskcipher_exit_tfm_simple(tfm:*mut crypto_lskcipher){let c=crypto_lskcipher_ctx(tfm) as *mut *mut crypto_lskcipher;crypto_free_lskcipher(*c);}
unsafe fn lskcipher_free_instance_simple(inst:*mut lskcipher_instance){crypto_drop_lskcipher(lskcipher_instance_ctx(inst));kfree(inst as *mut core::ffi::c_void);}

pub unsafe fn lskcipher_alloc_instance_simple(tmpl:*mut crypto_template,tb:*mut *mut rtattr)->*mut lskcipher_instance {
    let mut mask=0; let mut e=crypto_check_attr_type(tb,CRYPTO_ALG_TYPE_LSKCIPHER,&mut mask); if e!=0{return ERR_PTR(e)};
    let name=crypto_attr_alg_name(*tb.add(1));if IS_ERR(name){return ERR_CAST(name)};
    let inst=kzalloc(core::mem::size_of::<lskcipher_instance>()+core::mem::size_of::<crypto_lskcipher_spawn>(),GFP_KERNEL) as *mut lskcipher_instance;if inst.is_null(){return ERR_PTR(-ENOMEM)};
    let sp=lskcipher_instance_ctx(inst);e=crypto_grab_lskcipher(sp,lskcipher_crypto_instance(inst),name,0,mask);if e!=0{lskcipher_free_instance_simple(inst);return ERR_PTR(e)};
    let ca=crypto_lskcipher_spawn_alg(sp);e=crypto_inst_setname(lskcipher_crypto_instance(inst),(*tmpl).name,&mut (*ca).co.base);if e!=0{lskcipher_free_instance_simple(inst);return ERR_PTR(e)};
    if (*ca).co.ivsize!=0{lskcipher_free_instance_simple(inst);return ERR_PTR(-EINVAL)};(*inst).free=Some(lskcipher_free_instance_simple);
    (*inst).alg.co.base.cra_blocksize=(*ca).co.base.cra_blocksize;(*inst).alg.co.base.cra_alignmask=(*ca).co.base.cra_alignmask;(*inst).alg.co.base.cra_priority=(*ca).co.base.cra_priority;(*inst).alg.co.min_keysize=(*ca).co.min_keysize;(*inst).alg.co.max_keysize=(*ca).co.max_keysize;(*inst).alg.co.ivsize=(*ca).co.base.cra_blocksize;(*inst).alg.co.statesize=(*ca).co.statesize;(*inst).alg.co.base.cra_ctxsize=core::mem::size_of::<*mut crypto_lskcipher>();(*inst).alg.setkey=Some(lskcipher_setkey_simple);(*inst).alg.init=Some(lskcipher_init_tfm_simple);(*inst).alg.exit=Some(lskcipher_exit_tfm_simple);inst
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
