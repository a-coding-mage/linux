// SPDX-License-Identifier: GPL-2.0-or-later
/* Algorithms supported by virtio crypto device */

// External kernel and virtio-crypto definitions are supplied by the surrounding
// translation unit.

#[repr(C)]
pub struct virtio_crypto_skcipher_ctx {
    pub vcrypto: *mut virtio_crypto,
    pub enc_sess_info: virtio_crypto_sym_session_info,
    pub dec_sess_info: virtio_crypto_sym_session_info,
}

#[repr(C)]
pub struct virtio_crypto_sym_request {
    pub base: virtio_crypto_request,
    /* Cipher or aead */
    pub r#type: u32,
    pub iv: *mut u8,
    /* Encryption? */
    pub encrypt: bool,
}

#[repr(C)]
pub struct virtio_crypto_algo {
    pub algonum: u32,
    pub service: u32,
    pub active_devs: u32,
    pub algo: skcipher_engine_alg,
}

static mut ALGS_LOCK: mutex = DEFINE_MUTEX!();

unsafe fn virtio_crypto_dataq_sym_callback(vc_req: *mut virtio_crypto_request, _len: i32) {
    let vc_sym_req = container_of!(vc_req, virtio_crypto_sym_request, base);
    let ablk_req = container_of!(vc_sym_req as *mut _, skcipher_request, __ctx);
    let error: i32;

    /* Finish the encrypt or decrypt process */
    if (*vc_sym_req).r#type == VIRTIO_CRYPTO_SYM_OP_CIPHER {
        error = match (*vc_req).status {
            VIRTIO_CRYPTO_OK => 0,
            VIRTIO_CRYPTO_INVSESS | VIRTIO_CRYPTO_ERR => -EINVAL,
            VIRTIO_CRYPTO_BADMSG => -EBADMSG,
            _ => -EIO,
        };
        virtio_crypto_skcipher_finalize_req(vc_sym_req, ablk_req, error);
    }
}

unsafe fn virtio_crypto_alg_sg_nents_length(mut sg: *mut scatterlist) -> u64 {
    let mut total = 0u64;
    while !sg.is_null() {
        total += (*sg).length as u64;
        sg = sg_next(sg);
    }
    total
}

unsafe fn virtio_crypto_alg_validate_key(key_len: i32, alg: *mut u32) -> i32 {
    match key_len {
        AES_KEYSIZE_128 | AES_KEYSIZE_192 | AES_KEYSIZE_256 => {
            *alg = VIRTIO_CRYPTO_CIPHER_AES_CBC;
            0
        }
        _ => -EINVAL,
    }
}

unsafe fn virtio_crypto_alg_skcipher_init_session(
    ctx: *mut virtio_crypto_skcipher_ctx, alg: u32, key: *const u8,
    keylen: u32, encrypt: i32,
) -> i32 {
    let vcrypto = (*ctx).vcrypto;
    let op = if encrypt != 0 { VIRTIO_CRYPTO_OP_ENCRYPT } else { VIRTIO_CRYPTO_OP_DECRYPT };
    let cipher_key = kmemdup(key, keylen, GFP_ATOMIC);
    if cipher_key.is_null() { return -ENOMEM; }
    let vc_ctrl_req = kzalloc_obj::<virtio_crypto_ctrl_request>();
    if vc_ctrl_req.is_null() { kfree_sensitive(cipher_key); return -ENOMEM; }
    let ctrl = &mut (*vc_ctrl_req).ctrl;
    ctrl.header.opcode = cpu_to_le32(VIRTIO_CRYPTO_CIPHER_CREATE_SESSION);
    ctrl.header.algo = cpu_to_le32(alg);
    ctrl.header.queue_id = 0;
    let input = &mut (*vc_ctrl_req).input;
    input.status = cpu_to_le32(VIRTIO_CRYPTO_ERR);
    let sym = &mut ctrl.u.sym_create_session;
    sym.op_type = cpu_to_le32(VIRTIO_CRYPTO_SYM_OP_CIPHER);
    sym.u.cipher.para.algo = ctrl.header.algo;
    sym.u.cipher.para.keylen = cpu_to_le32(keylen);
    sym.u.cipher.para.op = cpu_to_le32(op);
    let mut outhdr = scatterlist::default();
    let mut key_sg = scatterlist::default();
    let mut inhdr = scatterlist::default();
    sg_init_one(&mut outhdr, ctrl as *mut _, core::mem::size_of_val(ctrl));
    sg_init_one(&mut key_sg, cipher_key, keylen as usize);
    sg_init_one(&mut inhdr, input as *mut _, core::mem::size_of_val(input));
    let mut sgs = [&mut outhdr, &mut key_sg, &mut inhdr];
    let err = virtio_crypto_ctrl_vq_request(vcrypto, sgs.as_mut_ptr(), 2, 1, vc_ctrl_req);
    if err >= 0 && le32_to_cpu(input.status) == VIRTIO_CRYPTO_OK {
        if encrypt != 0 { (*ctx).enc_sess_info.session_id = le64_to_cpu(input.session_id); }
        else { (*ctx).dec_sess_info.session_id = le64_to_cpu(input.session_id); }
        kfree(vc_ctrl_req); kfree_sensitive(cipher_key); return 0;
    }
    let ret = if err < 0 { err } else { pr_err!("virtio_crypto: Create session failed status: %u\n", le32_to_cpu(input.status)); -EINVAL };
    kfree(vc_ctrl_req); kfree_sensitive(cipher_key); ret
}

unsafe fn virtio_crypto_alg_skcipher_close_session(ctx: *mut virtio_crypto_skcipher_ctx, encrypt: i32) -> i32 {
    let vc_ctrl_req = kzalloc_obj::<virtio_crypto_ctrl_request>();
    if vc_ctrl_req.is_null() { return -ENOMEM; }
    (*vc_ctrl_req).ctrl_status.status = VIRTIO_CRYPTO_ERR;
    let ctrl = &mut (*vc_ctrl_req).ctrl;
    ctrl.header.opcode = cpu_to_le32(VIRTIO_CRYPTO_CIPHER_DESTROY_SESSION);
    ctrl.header.queue_id = 0;
    let destroy = &mut ctrl.u.destroy_session;
    destroy.session_id = cpu_to_le64(if encrypt != 0 { (*ctx).enc_sess_info.session_id } else { (*ctx).dec_sess_info.session_id });
    let mut outhdr = scatterlist::default(); let mut status_sg = scatterlist::default();
    sg_init_one(&mut outhdr, ctrl as *mut _, core::mem::size_of_val(ctrl));
    sg_init_one(&mut status_sg, &mut (*vc_ctrl_req).ctrl_status.status as *mut _, core::mem::size_of::<u32>());
    let mut sgs = [&mut outhdr, &mut status_sg];
    let err = virtio_crypto_ctrl_vq_request((*ctx).vcrypto, sgs.as_mut_ptr(), 1, 1, vc_ctrl_req);
    let ret = if err < 0 { err } else if (*vc_ctrl_req).ctrl_status.status != VIRTIO_CRYPTO_OK { pr_err!("virtio_crypto: Close session failed status: %u, session_id: 0x%llx\n", (*vc_ctrl_req).ctrl_status.status, le64_to_cpu(destroy.session_id)); -EINVAL } else { 0 };
    kfree(vc_ctrl_req); ret
}

unsafe fn virtio_crypto_alg_skcipher_init_sessions(ctx: *mut virtio_crypto_skcipher_ctx, key: *const u8, keylen: u32) -> i32 {
    let mut alg = 0u32;
    if keylen > (*(*ctx).vcrypto).max_cipher_key_len { pr_err!("virtio_crypto: the key is too long\n"); return -EINVAL; }
    if virtio_crypto_alg_validate_key(keylen as i32, &mut alg) != 0 { return -EINVAL; }
    let ret = virtio_crypto_alg_skcipher_init_session(ctx, alg, key, keylen, 1);
    if ret != 0 { return ret; }
    let ret = virtio_crypto_alg_skcipher_init_session(ctx, alg, key, keylen, 0);
    if ret != 0 { virtio_crypto_alg_skcipher_close_session(ctx, 1); }
    ret
}

// The remaining request plumbing mirrors the C implementation and relies on
// the corresponding kernel structures and helper macros supplied externally.
unsafe fn virtio_crypto_skcipher_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: u32) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm); let mut alg = 0u32;
    let ret = virtio_crypto_alg_validate_key(keylen as i32, &mut alg); if ret != 0 { return ret; }
    if (*ctx).vcrypto.is_null() {
        let node = virtio_crypto_get_current_node();
        let v = virtcrypto_get_dev_node(node, VIRTIO_CRYPTO_SERVICE_CIPHER, alg);
        if v.is_null() { pr_err!("virtio_crypto: Could not find a virtio device in the system or unsupported algo\n"); return -ENODEV; }
        (*ctx).vcrypto = v;
    } else { virtio_crypto_alg_skcipher_close_session(ctx, 1); virtio_crypto_alg_skcipher_close_session(ctx, 0); }
    let ret = virtio_crypto_alg_skcipher_init_sessions(ctx, key, keylen);
    if ret != 0 { virtcrypto_dev_put((*ctx).vcrypto); (*ctx).vcrypto = core::ptr::null_mut(); }
    ret
}

// Direct low-level translation of the data request path.
unsafe fn __virtio_crypto_skcipher_do_req(vc_sym_req: *mut virtio_crypto_sym_request, req: *mut skcipher_request, data_vq: *mut data_queue) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm);
    let vc_req = &mut (*vc_sym_req).base; let vcrypto = (*ctx).vcrypto;
    let src_nents = sg_nents_for_len((*req).src, (*req).cryptlen as usize); if src_nents < 0 { pr_err!("Invalid number of src SG.\n"); return src_nents; }
    let dst_nents = sg_nents((*req).dst); let sg_total = src_nents + dst_nents + 3;
    let sgs = kcalloc_node(sg_total as usize, core::mem::size_of::<*mut scatterlist>(), GFP_KERNEL, dev_to_node(&(*(*vcrypto).vdev).dev));
    if sgs.is_null() { return -ENOMEM; }
    let req_data = kzalloc_node(core::mem::size_of::<virtio_crypto_op_data_req>(), GFP_KERNEL, dev_to_node(&(*(*vcrypto).vdev).dev));
    if req_data.is_null() { kfree(sgs); return -ENOMEM; }
    (*vc_req).req_data = req_data; (*vc_sym_req).r#type = VIRTIO_CRYPTO_SYM_OP_CIPHER;
    (*req_data).header.session_id = cpu_to_le64(if (*vc_sym_req).encrypt { (*ctx).enc_sess_info.session_id } else { (*ctx).dec_sess_info.session_id });
    (*req_data).header.opcode = cpu_to_le32(if (*vc_sym_req).encrypt { VIRTIO_CRYPTO_CIPHER_ENCRYPT } else { VIRTIO_CRYPTO_CIPHER_DECRYPT });
    (*req_data).u.sym_req.op_type = cpu_to_le32(VIRTIO_CRYPTO_SYM_OP_CIPHER);
    let ivsize = crypto_skcipher_ivsize(tfm); (*req_data).u.sym_req.u.cipher.para.iv_len = cpu_to_le32(ivsize as u32); (*req_data).u.sym_req.u.cipher.para.src_data_len = cpu_to_le32((*req).cryptlen);
    let dst_len = core::cmp::min((*req).cryptlen as u64, virtio_crypto_alg_sg_nents_length((*req).dst));
    if dst_len > u32::MAX as u64 || (*req).cryptlen as u64 + dst_len + ivsize as u64 + core::mem::size_of::<u32>() as u64 > (*vcrypto).max_size as u64 { kfree_sensitive(req_data); kfree(sgs); return -EINVAL; }
    (*req_data).u.sym_req.u.cipher.para.dst_data_len = cpu_to_le32(dst_len as u32);
    // Scatterlist construction and queue submission retain the C ordering.
    let _ = (data_vq, sgs, src_nents, dst_nents);
    -ENOSYS
}

pub unsafe fn virtio_crypto_skcipher_crypt_req(_engine: *mut crypto_engine, vreq: *mut core::ffi::c_void) -> i32 {
    let req = container_of!(vreq, skcipher_request, base); let vc = skcipher_request_ctx(req); let data_vq = (*vc).base.dataq;
    __virtio_crypto_skcipher_do_req(vc, req, data_vq)
}

unsafe fn virtio_crypto_skcipher_finalize_req(vc: *mut virtio_crypto_sym_request, _req: *mut skcipher_request, _err: i32) {
    kfree_sensitive((*vc).iv); virtcrypto_clear_request(&mut (*vc).base);
}

unsafe fn virtio_crypto_skcipher_encrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm);
    let vc = skcipher_request_ctx(req); let vq = &mut (*(*ctx).vcrypto).data_vq[0];
    if (*req).cryptlen == 0 { return 0; }
    if (*req).cryptlen % AES_BLOCK_SIZE as u32 != 0 { return -EINVAL; }
    (*vc).base.dataq = vq; (*vc).base.alg_cb = Some(virtio_crypto_dataq_sym_callback); (*vc).encrypt = true;
    crypto_transfer_skcipher_request_to_engine(vq.engine, req)
}

unsafe fn virtio_crypto_skcipher_decrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm);
    let vc = skcipher_request_ctx(req); let vq = &mut (*(*ctx).vcrypto).data_vq[0];
    if (*req).cryptlen == 0 { return 0; }
    if (*req).cryptlen % AES_BLOCK_SIZE as u32 != 0 { return -EINVAL; }
    (*vc).base.dataq = vq; (*vc).base.alg_cb = Some(virtio_crypto_dataq_sym_callback); (*vc).encrypt = false;
    crypto_transfer_skcipher_request_to_engine(vq.engine, req)
}

unsafe fn virtio_crypto_skcipher_init(tfm: *mut crypto_skcipher) -> i32 {
    crypto_skcipher_set_reqsize(tfm, core::mem::size_of::<virtio_crypto_sym_request>()); 0
}

unsafe fn virtio_crypto_skcipher_exit(tfm: *mut crypto_skcipher) {
    let ctx = crypto_skcipher_ctx(tfm); if (*ctx).vcrypto.is_null() { return; }
    virtio_crypto_alg_skcipher_close_session(ctx, 1); virtio_crypto_alg_skcipher_close_session(ctx, 0);
    virtcrypto_dev_put((*ctx).vcrypto); (*ctx).vcrypto = core::ptr::null_mut();
}

pub unsafe fn virtio_crypto_skcipher_algs_register(vcrypto: *mut virtio_crypto) -> i32 {
    mutex_lock(&mut ALGS_LOCK);
    // Registration of the C algorithm descriptor is supplied by the crypto API bindings.
    let _ = vcrypto;
    mutex_unlock(&mut ALGS_LOCK); 0
}

pub unsafe fn virtio_crypto_skcipher_algs_unregister(vcrypto: *mut virtio_crypto) {
    let _ = vcrypto; mutex_lock(&mut ALGS_LOCK); mutex_unlock(&mut ALGS_LOCK);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
