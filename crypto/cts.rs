/*
 * CTS: Cipher Text Stealing mode
 *
 * COPYRIGHT (c) 2008
 * The Regents of the University of Michigan
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS FOR A PARTICULAR PURPOSE. THE REGENTS OF THE
 * UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE FOR ANY DAMAGES,
 * INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR CONSEQUENTIAL
 * DAMAGES, WITH RESPECT TO ANY CLAIM ARISING OUT OF OR IN
 * CONNECTION WITH THE USE OF THE SOFTWARE, EVEN IF IT HAS
 * BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGES.
 */

/* Derived from various:
 * Copyright (c) 2006 Herbert Xu <herbert@gondor.apana.org.au>
 */

/* This is the Cipher Text Stealing mode as described by
 * Section 8 of rfc2040 and referenced by rfc3962.
 * rfc3962 includes errata information in its Appendix A.
 */

// Kernel crypto, scatterlist, module, and allocation dependencies are supplied externally.

#[repr(C)]
pub struct crypto_cts_ctx {
    pub child: *mut crypto_skcipher,
}

#[repr(C)]
pub struct crypto_cts_reqctx {
    pub sg: [scatterlist; 2],
    pub offset: c_uint,
    pub subreq: skcipher_request,
}

#[inline]
pub unsafe fn crypto_cts_reqctx_space(req: *mut skcipher_request) -> *mut u8 {
    let rctx = skcipher_request_ctx(req);
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let child = (*ctx).child;

    ptr_align(
        (rctx.add(1) as *mut u8).add(crypto_skcipher_reqsize(child)),
        crypto_skcipher_alignmask(tfm).wrapping_add(1),
    )
}

pub unsafe fn crypto_cts_setkey(
    parent: *mut crypto_skcipher,
    key: *const u8,
    keylen: c_uint,
) -> c_int {
    let ctx = crypto_skcipher_ctx(parent);
    let child = (*ctx).child;

    crypto_skcipher_clear_flags(child, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(
        child,
        crypto_skcipher_get_flags(parent) & CRYPTO_TFM_REQ_MASK,
    );
    crypto_skcipher_setkey(child, key, keylen)
}

pub unsafe extern "C" fn cts_cbc_crypt_done(data: *mut c_void, err: c_int) {
    let req = data as *mut skcipher_request;
    if err == -EINPROGRESS {
        return;
    }
    skcipher_request_complete(req, err);
}

pub unsafe fn cts_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    let rctx = skcipher_request_ctx(req);
    let tfm = crypto_skcipher_reqtfm(req);
    let subreq = &mut (*rctx).subreq;
    let bsize = crypto_skcipher_blocksize(tfm) as usize;
    let mut d = [0u8; MAX_CIPHER_BLOCKSIZE * 2];
    let offset = (*rctx).offset as usize;
    let lastn = (*req).cryptlen as usize - offset;

    let sg = scatterwalk_ffwd(&mut (*rctx).sg, (*req).dst, offset - bsize);
    scatterwalk_map_and_copy(d.as_mut_ptr().add(bsize), sg, 0, bsize, 0);
    core::ptr::write_bytes(d.as_mut_ptr(), 0, bsize);
    scatterwalk_map_and_copy(d.as_mut_ptr(), (*req).src, offset, lastn, 0);
    scatterwalk_map_and_copy(d.as_mut_ptr(), sg, 0, bsize + lastn, 1);
    memzero_explicit(d.as_mut_ptr() as *mut c_void, d.len());

    skcipher_request_set_callback(
        subreq,
        (*req).base.flags & CRYPTO_TFM_REQ_MAY_BACKLOG,
        Some(cts_cbc_crypt_done),
        req as *mut c_void,
    );
    skcipher_request_set_crypt(subreq, sg, sg, bsize as c_uint, (*req).iv);
    crypto_skcipher_encrypt(subreq)
}

pub unsafe extern "C" fn crypto_cts_encrypt_done(data: *mut c_void, mut err: c_int) {
    let req = data as *mut skcipher_request;
    if err != 0 {
        skcipher_request_complete(req, err);
        return;
    }
    err = cts_cbc_encrypt(req);
    if err == -EINPROGRESS || err == -EBUSY {
        return;
    }
    skcipher_request_complete(req, err);
}

pub unsafe fn crypto_cts_encrypt(req: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(req);
    let rctx = skcipher_request_ctx(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let subreq = &mut (*rctx).subreq;
    let bsize = crypto_skcipher_blocksize(tfm) as c_uint;
    let nbytes = (*req).cryptlen;

    skcipher_request_set_tfm(subreq, (*ctx).child);
    if nbytes < bsize { return -EINVAL; }
    if nbytes == bsize {
        skcipher_request_set_callback(subreq, (*req).base.flags, (*req).base.complete, (*req).base.data);
        skcipher_request_set_crypt(subreq, (*req).src, (*req).dst, nbytes, (*req).iv);
        return crypto_skcipher_encrypt(subreq);
    }
    let offset = rounddown(nbytes - 1, bsize);
    (*rctx).offset = offset;
    skcipher_request_set_callback(subreq, (*req).base.flags, Some(crypto_cts_encrypt_done), req as *mut c_void);
    skcipher_request_set_crypt(subreq, (*req).src, (*req).dst, offset, (*req).iv);
    let err = crypto_skcipher_encrypt(subreq);
    if err != 0 { err } else { cts_cbc_encrypt(req) }
}

pub unsafe fn cts_cbc_decrypt(req: *mut skcipher_request) -> c_int {
    let rctx = skcipher_request_ctx(req);
    let tfm = crypto_skcipher_reqtfm(req);
    let subreq = &mut (*rctx).subreq;
    let bsize = crypto_skcipher_blocksize(tfm) as usize;
    let offset = (*rctx).offset as usize;
    let lastn = (*req).cryptlen as usize - offset;
    let mut d = [0u8; MAX_CIPHER_BLOCKSIZE * 2];
    let sg = scatterwalk_ffwd(&mut (*rctx).sg, (*req).dst, offset - bsize);
    scatterwalk_map_and_copy(d.as_mut_ptr().add(bsize), sg, 0, bsize, 0);
    let space = crypto_cts_reqctx_space(req);
    crypto_xor(d.as_mut_ptr().add(bsize), space, bsize);
    core::ptr::write_bytes(d.as_mut_ptr(), 0, bsize);
    scatterwalk_map_and_copy(d.as_mut_ptr(), (*req).src, offset, lastn, 0);
    crypto_xor(d.as_mut_ptr().add(bsize), d.as_mut_ptr(), lastn);
    core::ptr::copy(d.as_ptr().add(bsize + lastn), d.as_mut_ptr().add(lastn), bsize - lastn);
    scatterwalk_map_and_copy(d.as_mut_ptr(), sg, 0, bsize + lastn, 1);
    memzero_explicit(d.as_mut_ptr() as *mut c_void, d.len());
    skcipher_request_set_callback(subreq, (*req).base.flags & CRYPTO_TFM_REQ_MAY_BACKLOG, Some(cts_cbc_crypt_done), req as *mut c_void);
    skcipher_request_set_crypt(subreq, sg, sg, bsize as c_uint, space);
    crypto_skcipher_decrypt(subreq)
}

pub unsafe extern "C" fn crypto_cts_decrypt_done(data: *mut c_void, mut err: c_int) {
    let req = data as *mut skcipher_request;
    if err != 0 {
        skcipher_request_complete(req, err);
        return;
    }
    err = cts_cbc_decrypt(req);
    if err == -EINPROGRESS || err == -EBUSY { return; }
    skcipher_request_complete(req, err);
}

pub unsafe fn crypto_cts_decrypt(req: *mut skcipher_request) -> c_int {
    let tfm = crypto_skcipher_reqtfm(req);
    let rctx = skcipher_request_ctx(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let subreq = &mut (*rctx).subreq;
    let bsize = crypto_skcipher_blocksize(tfm) as c_uint;
    let nbytes = (*req).cryptlen;
    skcipher_request_set_tfm(subreq, (*ctx).child);
    if nbytes < bsize { return -EINVAL; }
    if nbytes == bsize {
        skcipher_request_set_callback(subreq, (*req).base.flags, (*req).base.complete, (*req).base.data);
        skcipher_request_set_crypt(subreq, (*req).src, (*req).dst, nbytes, (*req).iv);
        return crypto_skcipher_decrypt(subreq);
    }
    skcipher_request_set_callback(subreq, (*req).base.flags, Some(crypto_cts_decrypt_done), req as *mut c_void);
    let space = crypto_cts_reqctx_space(req);
    let offset = rounddown(nbytes - 1, bsize);
    (*rctx).offset = offset;
    if offset <= bsize {
        core::ptr::copy_nonoverlapping((*req).iv, space, bsize as usize);
    } else {
        scatterwalk_map_and_copy(space, (*req).src, (offset - 2 * bsize) as usize, bsize as usize, 0);
    }
    skcipher_request_set_crypt(subreq, (*req).src, (*req).dst, offset, (*req).iv);
    let err = crypto_skcipher_decrypt(subreq);
    if err != 0 { err } else { cts_cbc_decrypt(req) }
}

/* The remaining tfm/template/module registration declarations are supplied by
 * the kernel crypto framework and retain the source-level callback layout. */

pub unsafe extern "C" fn crypto_cts_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    let inst = skcipher_alg_instance(tfm);
    let spawn = skcipher_instance_ctx(inst);
    let ctx = crypto_skcipher_ctx(tfm);
    let cipher = crypto_spawn_skcipher(spawn);
    if is_err(cipher) { return ptr_err(cipher); }
    (*ctx).child = cipher;
    let align = crypto_skcipher_alignmask(tfm);
    let bsize = crypto_skcipher_blocksize(cipher);
    let reqsize = align_up(
        core::mem::size_of::<crypto_cts_reqctx>() + crypto_skcipher_reqsize(cipher),
        crypto_tfm_ctx_alignment(),
    ) + (align & !(crypto_tfm_ctx_alignment() - 1)) + bsize;
    crypto_skcipher_set_reqsize(tfm, reqsize);
    0
}

pub unsafe extern "C" fn crypto_cts_exit_tfm(tfm: *mut crypto_skcipher) {
    let ctx = crypto_skcipher_ctx(tfm);
    crypto_free_skcipher((*ctx).child);
}

pub unsafe extern "C" fn crypto_cts_free(inst: *mut skcipher_instance) {
    crypto_drop_skcipher(skcipher_instance_ctx(inst));
    kfree(inst as *mut c_void);
}

pub unsafe extern "C" fn crypto_cts_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> c_int {
    let inst = kzalloc(core::mem::size_of::<skcipher_instance>() + core::mem::size_of::<crypto_skcipher_spawn>(), GFP_KERNEL) as *mut skcipher_instance;
    if inst.is_null() { return -ENOMEM; }
    let spawn = skcipher_instance_ctx(inst);
    let mut mask: u32 = 0;
    let mut err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_SKCIPHER, &mut mask);
    if err != 0 { kfree(inst as *mut c_void); return err; }
    err = crypto_grab_skcipher(spawn, skcipher_crypto_instance(inst), crypto_attr_alg_name(*tb.add(1)), 0, mask);
    if err != 0 { crypto_cts_free(inst); return err; }
    let alg = crypto_spawn_skcipher_alg_common(spawn);
    err = -EINVAL;
    if (*alg).ivsize != (*alg).base.cra_blocksize || c_strncmp((*alg).base.cra_name, b"cbc(".as_ptr() as *const c_char, 4) != 0 { crypto_cts_free(inst); return err; }
    err = crypto_inst_setname(skcipher_crypto_instance(inst), b"cts\0".as_ptr() as *const c_char, &(*alg).base);
    if err != 0 { crypto_cts_free(inst); return err; }
    skcipher_instance_set_callbacks(inst, Some(crypto_cts_init_tfm), Some(crypto_cts_exit_tfm), Some(crypto_cts_setkey), Some(crypto_cts_encrypt), Some(crypto_cts_decrypt), Some(crypto_cts_free));
    skcipher_register_instance(tmpl, inst)
}

#[repr(C)]
pub struct crypto_template { pub name: *const c_char, pub create: Option<unsafe extern "C" fn(*mut crypto_template, *mut *mut rtattr) -> c_int>, pub module: *mut c_void }

#[no_mangle]
pub static mut crypto_cts_tmpl: crypto_template = crypto_template {
    name: b"cts\0".as_ptr() as *const c_char,
    create: Some(crypto_cts_create),
    module: core::ptr::null_mut(),
};

pub unsafe extern "C" fn crypto_cts_module_init() -> c_int { crypto_register_template(&mut crypto_cts_tmpl) }
pub unsafe extern "C" fn crypto_cts_module_exit() { crypto_unregister_template(&mut crypto_cts_tmpl); }

// Module metadata: Dual BSD/GPL; CTS-CBC CipherText Stealing for CBC; alias "cts".

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
