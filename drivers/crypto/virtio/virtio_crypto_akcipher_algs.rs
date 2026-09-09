// SPDX-License-Identifier: GPL-2.0-or-later
/* Asymmetric algorithms supported by virtio crypto device */

// External kernel and virtio-crypto declarations are supplied by other files.

#[repr(C)]
struct VirtioCryptoRsaCtx { key_size: c_uint }

#[repr(C)]
struct VirtioCryptoAkcipherCtx {
    vcrypto: *mut VirtioCrypto,
    session_valid: bool,
    session_id: u64,
    rsa_ctx: VirtioCryptoRsaCtx,
}

#[repr(C)]
struct VirtioCryptoAkcipherRequest {
    base: VirtioCryptoRequest,
    src_buf: *mut c_void,
    dst_buf: *mut c_void,
    opcode: u32,
}

#[repr(C)]
struct VirtioCryptoAkcipherAlgo {
    algonum: u32,
    service: u32,
    active_devs: c_uint,
    algo: AkcipherEngineAlg,
}

static mut ALGS_LOCK: Mutex = DEFINE_MUTEX!();

unsafe fn virtio_crypto_akcipher_finalize_req(v: *mut VirtioCryptoAkcipherRequest,
                                               req: *mut AkcipherRequest, err: c_int) {
    kfree((*v).src_buf);
    kfree((*v).dst_buf);
    (*v).src_buf = core::ptr::null_mut();
    (*v).dst_buf = core::ptr::null_mut();
    virtcrypto_clear_request(&mut (*v).base);
    crypto_finalize_akcipher_request((*(*v).base.dataq).engine, req, err);
}

unsafe extern "C" fn virtio_crypto_dataq_akcipher_callback(vc_req: *mut VirtioCryptoRequest,
                                                             len: c_int) {
    let v = container_of!(vc_req, VirtioCryptoAkcipherRequest, base);
    let req = container_of!((v as *mut c_void), AkcipherRequest, __ctx);
    let error = match (*vc_req).status {
        VIRTIO_CRYPTO_OK => 0,
        VIRTIO_CRYPTO_INVSESS | VIRTIO_CRYPTO_ERR => -EINVAL,
        VIRTIO_CRYPTO_BADMSG => -EBADMSG,
        _ => -EIO,
    };
    // actual length may be less than dst buffer
    (*req).dst_len = core::cmp::min((len as usize).wrapping_sub(core::mem::size_of_val(&(*vc_req).status)),
                                    (*req).dst_len);
    sg_copy_from_buffer((*req).dst, sg_nents((*req).dst), (*v).dst_buf,
                        (*req).dst_len);
    virtio_crypto_akcipher_finalize_req(v, req, error);
}

unsafe fn virtio_crypto_alg_akcipher_init_session(ctx: *mut VirtioCryptoAkcipherCtx,
    header: *const VirtioCryptoCtrlHeader, para: *const VirtioCryptoAkcipherSessionPara,
    key: *const u8, keylen: c_uint) -> c_int {
    let pkey = kmemdup(key, keylen, GFP_KERNEL);
    if pkey.is_null() { return -ENOMEM; }
    let vc = kzalloc_obj::<VirtioCryptoCtrlRequest>();
    if vc.is_null() { kfree_sensitive(pkey); return -ENOMEM; }
    (*vc).ctrl.header = *header;
    (*vc).ctrl.u.akcipher_create_session.para = *para;
    (*vc).input.status = cpu_to_le32(VIRTIO_CRYPTO_ERR);
    let mut outhdr_sg = core::mem::zeroed::<Scatterlist>();
    let mut key_sg = core::mem::zeroed::<Scatterlist>();
    let mut inhdr_sg = core::mem::zeroed::<Scatterlist>();
    sg_init_one(&mut outhdr_sg, &mut (*vc).ctrl, core::mem::size_of::<VirtioCryptoOpCtrlReq>());
    sg_init_one(&mut key_sg, pkey, keylen as usize);
    sg_init_one(&mut inhdr_sg, &mut (*vc).input, core::mem::size_of::<VirtioCryptoSessionInput>());
    let mut sgs = [&mut outhdr_sg, &mut key_sg, &mut inhdr_sg];
    let mut err = virtio_crypto_ctrl_vq_request((*ctx).vcrypto, sgs.as_mut_ptr(), 2, 1, vc);
    if err >= 0 {
        if le32_to_cpu((*vc).input.status) != VIRTIO_CRYPTO_OK {
            pr_err!("virtio_crypto: Create session failed status: %u\n", le32_to_cpu((*vc).input.status));
            err = -EINVAL;
        } else {
            (*ctx).session_id = le64_to_cpu((*vc).input.session_id);
            (*ctx).session_valid = true;
            err = 0;
        }
    }
    kfree(vc as *mut c_void); kfree_sensitive(pkey); err
}

unsafe fn virtio_crypto_alg_akcipher_close_session(ctx: *mut VirtioCryptoAkcipherCtx) -> c_int {
    if !(*ctx).session_valid { return 0; }
    let vc = kzalloc_obj::<VirtioCryptoCtrlRequest>();
    if vc.is_null() { return -ENOMEM; }
    (*vc).ctrl_status.status = VIRTIO_CRYPTO_ERR;
    (*vc).ctrl.header.opcode = cpu_to_le32(VIRTIO_CRYPTO_AKCIPHER_DESTROY_SESSION);
    (*vc).ctrl.header.queue_id = 0;
    (*vc).ctrl.u.destroy_session.session_id = cpu_to_le64((*ctx).session_id);
    let mut out = core::mem::zeroed::<Scatterlist>(); let mut input = core::mem::zeroed::<Scatterlist>();
    sg_init_one(&mut out, &mut (*vc).ctrl, core::mem::size_of::<VirtioCryptoOpCtrlReq>());
    sg_init_one(&mut input, &mut (*vc).ctrl_status.status, core::mem::size_of::<u8>());
    let mut sgs = [&mut out, &mut input];
    let mut err = virtio_crypto_ctrl_vq_request((*ctx).vcrypto, sgs.as_mut_ptr(), 1, 1, vc);
    if err >= 0 {
        if (*vc).ctrl_status.status != VIRTIO_CRYPTO_OK { err = -EINVAL; } else { (*ctx).session_valid = false; err = 0; }
    }
    kfree(vc as *mut c_void); err
}

unsafe fn __virtio_crypto_akcipher_do_req(v: *mut VirtioCryptoAkcipherRequest, req: *mut AkcipherRequest, q: *mut DataQueue) -> c_int {
    let mut src = kcalloc_node((*req).src_len, 1, GFP_KERNEL, dev_to_node((*(*(*v).base.dataq).vcrypto).vdev));
    if src.is_null() { return -ENOMEM; }
    let dst = kcalloc_node((*req).dst_len, 1, GFP_KERNEL, dev_to_node((*(*(*v).base.dataq).vcrypto).vdev));
    if dst.is_null() { kfree(src); return -ENOMEM; }
    sg_copy_to_buffer((*req).src, sg_nents((*req).src), src, (*req).src_len);
    (*v).src_buf = src; (*v).dst_buf = dst;
    let mut sgs = [core::mem::zeroed::<Scatterlist>(); 4];
    sg_init_one(&mut sgs[0], (*v).base.req_data, core::mem::size_of::<VirtioCryptoOpDataReq>());
    sg_init_one(&mut sgs[1], src, (*req).src_len);
    sg_init_one(&mut sgs[2], dst, (*req).dst_len);
    sg_init_one(&mut sgs[3], &mut (*v).base.status, core::mem::size_of_val(&(*v).base.status));
    let mut flags = 0ul; spin_lock_irqsave(&mut (*q).lock, &mut flags);
    let ret = virtqueue_add_sgs((*q).vq, sgs.as_mut_ptr(), 2, 2, &mut (*v).base, GFP_ATOMIC);
    virtqueue_kick((*q).vq); spin_unlock_irqrestore(&mut (*q).lock, flags);
    if ret != 0 { kfree(dst); kfree(src); return -ENOMEM; } 0
}

unsafe fn virtio_crypto_rsa_do_req(_engine: *mut CryptoEngine, vreq: *mut c_void) -> c_int {
    let req = container_of!(vreq, AkcipherRequest, base);
    let v = akcipher_request_ctx(req); let ctx = akcipher_tfm_ctx(crypto_akcipher_reqtfm(req));
    (*v).base.sgs = core::ptr::null_mut();
    (*v).base.req_data = kzalloc_node(core::mem::size_of::<VirtioCryptoOpDataReq>(), GFP_KERNEL, 0);
    if (*v).base.req_data.is_null() { return -ENOMEM; }
    (*(*v).base.req_data).header.opcode = cpu_to_le32((*v).opcode);
    (*(*v).base.req_data).header.algo = cpu_to_le32(VIRTIO_CRYPTO_AKCIPHER_RSA);
    (*(*v).base.req_data).header.session_id = cpu_to_le64((*ctx).session_id);
    (*(*v).base.req_data).u.akcipher_req.para.src_data_len = cpu_to_le32((*req).src_len as u32);
    (*(*v).base.req_data).u.akcipher_req.para.dst_data_len = cpu_to_le32((*req).dst_len as u32);
    __virtio_crypto_akcipher_do_req(v, req, (*v).base.dataq)
}

unsafe fn virtio_crypto_rsa_req(req: *mut AkcipherRequest, opcode: u32) -> c_int {
    let v = akcipher_request_ctx(req); let ctx = akcipher_tfm_ctx(crypto_akcipher_reqtfm(req));
    (*v).base.dataq = &mut (*(*ctx).vcrypto).data_vq[0]; (*v).base.alg_cb = Some(virtio_crypto_dataq_akcipher_callback); (*v).opcode = opcode;
    crypto_transfer_akcipher_request_to_engine((*(*v).base.dataq).engine, req)
}
unsafe fn virtio_crypto_rsa_encrypt(r: *mut AkcipherRequest) -> c_int { virtio_crypto_rsa_req(r, VIRTIO_CRYPTO_AKCIPHER_ENCRYPT) }
unsafe fn virtio_crypto_rsa_decrypt(r: *mut AkcipherRequest) -> c_int { virtio_crypto_rsa_req(r, VIRTIO_CRYPTO_AKCIPHER_DECRYPT) }

pub unsafe fn virtio_crypto_akcipher_algs_register(_vcrypto: *mut VirtioCrypto) -> c_int { 0 }
pub unsafe fn virtio_crypto_akcipher_algs_unregister(_vcrypto: *mut VirtioCrypto) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
