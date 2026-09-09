// SPDX-License-Identifier: GPL-2.0
/*
 * Hash function and HMAC support for StarFive driver
 *
 * Copyright (c) 2022 StarFive Technology
 */

// C dependencies supplied by the surrounding kernel translation unit.

const STARFIVE_HASH_REGS_OFFSET: u32 = 0x300;
const STARFIVE_HASH_SHACSR: u32 = STARFIVE_HASH_REGS_OFFSET + 0x0;
const STARFIVE_HASH_SHAWDR: u32 = STARFIVE_HASH_REGS_OFFSET + 0x4;
const STARFIVE_HASH_SHARDR: u32 = STARFIVE_HASH_REGS_OFFSET + 0x8;
const STARFIVE_HASH_SHAWSR: u32 = STARFIVE_HASH_REGS_OFFSET + 0xc;
const STARFIVE_HASH_SHAWLEN3: u32 = STARFIVE_HASH_REGS_OFFSET + 0x10;
const STARFIVE_HASH_SHAWLEN2: u32 = STARFIVE_HASH_REGS_OFFSET + 0x14;
const STARFIVE_HASH_SHAWLEN1: u32 = STARFIVE_HASH_REGS_OFFSET + 0x18;
const STARFIVE_HASH_SHAWLEN0: u32 = STARFIVE_HASH_REGS_OFFSET + 0x1c;
const STARFIVE_HASH_SHAWKR: u32 = STARFIVE_HASH_REGS_OFFSET + 0x20;
const STARFIVE_HASH_SHAWKLEN: u32 = STARFIVE_HASH_REGS_OFFSET + 0x24;
const STARFIVE_HASH_BUFLEN: usize = SHA512_BLOCK_SIZE as usize;
const STARFIVE_HASH_RESET: u32 = 0x2;

#[inline]
unsafe fn starfive_hash_wait_busy(cryp: *mut starfive_cryp_dev) -> c_int {
    let mut status: u32 = 0;
    readl_relaxed_poll_timeout((*cryp).base.add(STARFIVE_HASH_SHACSR as usize), &mut status,
        (status & STARFIVE_HASH_BUSY) == 0, 10, 100000)
}

#[inline]
unsafe fn starfive_hash_wait_hmac_done(cryp: *mut starfive_cryp_dev) -> c_int {
    let mut status: u32 = 0;
    readl_relaxed_poll_timeout((*cryp).base.add(STARFIVE_HASH_SHACSR as usize), &mut status,
        (status & STARFIVE_HASH_HMAC_DONE) != 0, 10, 100000)
}

#[inline]
unsafe fn starfive_hash_wait_key_done(ctx: *mut starfive_cryp_ctx) -> c_int {
    let cryp = (*ctx).cryp;
    let mut status: u32 = 0;
    readl_relaxed_poll_timeout((*cryp).base.add(STARFIVE_HASH_SHACSR as usize), &mut status,
        (status & STARFIVE_HASH_KEY_DONE) != 0, 10, 100000)
}

unsafe fn starfive_hash_hmac_key(ctx: *mut starfive_cryp_ctx) -> c_int {
    let rctx = (*ctx).rctx;
    let cryp = (*ctx).cryp;
    let klen = (*ctx).keylen as usize;
    let mut key = (*ctx).key as *mut u32;
    writel((*ctx).keylen, (*cryp).base.add(STARFIVE_HASH_SHAWKLEN as usize));
    (*rctx).csr.hash.hmac = 1;
    (*rctx).csr.hash.key_flag = 1;
    writel((*rctx).csr.hash.v, (*cryp).base.add(STARFIVE_HASH_SHACSR as usize));
    for _ in 0..(klen / core::mem::size_of::<u32>()) {
        writel(*key, (*cryp).base.add(STARFIVE_HASH_SHAWKR as usize));
        key = key.add(1);
    }
    if (klen & 3) != 0 {
        let mut cl = key as *mut u8;
        for _ in 0..(klen & 3) {
            writeb(*cl, (*cryp).base.add(STARFIVE_HASH_SHAWKR as usize));
            cl = cl.add(1);
        }
    }
    if starfive_hash_wait_key_done(ctx) != 0 {
        return dev_err_probe((*cryp).dev, -ETIMEDOUT, c"starfive_hash_wait_key_done error\n".as_ptr());
    }
    0
}

unsafe fn starfive_hash_start(cryp: *mut starfive_cryp_dev) {
    let mut csr = union_starfive_hash_csr { v: readl((*cryp).base.add(STARFIVE_HASH_SHACSR as usize)) };
    (*(&mut csr as *mut _ as *mut starfive_hash_csr)).firstb = 0;
    (*(&mut csr as *mut _ as *mut starfive_hash_csr)).final_ = 1;
    writel(csr.v, (*cryp).base.add(STARFIVE_HASH_SHACSR as usize));
}

unsafe extern "C" fn starfive_hash_dma_callback(param: *mut c_void) {
    complete(&mut (*(param as *mut starfive_cryp_dev)).dma_done);
}

unsafe fn starfive_hash_dma_init(cryp: *mut starfive_cryp_dev) {
    (*cryp).cfg_in.src_addr_width = DMA_SLAVE_BUSWIDTH_16_BYTES;
    (*cryp).cfg_in.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*cryp).cfg_in.src_maxburst = (*cryp).dma_maxburst;
    (*cryp).cfg_in.dst_maxburst = (*cryp).dma_maxburst;
    (*cryp).cfg_in.dst_addr = (*cryp).phys_base + STARFIVE_ALG_FIFO_OFFSET;
    dmaengine_slave_config((*cryp).tx, &mut (*cryp).cfg_in);
    init_completion(&mut (*cryp).dma_done);
}

unsafe fn starfive_hash_dma_xfer(cryp: *mut starfive_cryp_dev, sg: *mut scatterlist) -> c_int {
    let mut alg_cr = union_starfive_alg_cr { v: 0 };
    alg_cr.start = 1;
    alg_cr.hash_dma_en = 1;
    writel(alg_cr.v, (*cryp).base.add(STARFIVE_ALG_CR_OFFSET as usize));
    writel(sg_dma_len(sg), (*cryp).base.add(STARFIVE_DMA_IN_LEN_OFFSET as usize));
    sg_dma_len(sg) = ALIGN(sg_dma_len(sg), core::mem::size_of::<u32>());
    let desc = dmaengine_prep_slave_sg((*cryp).tx, sg, 1, DMA_MEM_TO_DEV,
        DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if desc.is_null() {
        alg_cr.v = 0; alg_cr.clear = 1;
        writel(alg_cr.v, (*cryp).base.add(STARFIVE_ALG_CR_OFFSET as usize));
        return -EINVAL;
    }
    reinit_completion(&mut (*cryp).dma_done);
    (*desc).callback = Some(starfive_hash_dma_callback);
    (*desc).callback_param = cryp as *mut c_void;
    dmaengine_submit(desc);
    dma_async_issue_pending((*cryp).tx);
    let mut ret = 0;
    if wait_for_completion_timeout(&mut (*cryp).dma_done, msecs_to_jiffies(1000)) == 0 { ret = -ETIMEDOUT; }
    alg_cr.v = 0; alg_cr.clear = 1;
    writel(alg_cr.v, (*cryp).base.add(STARFIVE_ALG_CR_OFFSET as usize));
    ret
}

unsafe fn starfive_hash_copy_hash(req: *mut ahash_request) -> c_int {
    let rctx = ahash_request_ctx(req);
    let tfm = crypto_ahash_reqtfm(req);
    let ctx = crypto_ahash_ctx(tfm);
    if (*req).result.is_null() { return 0; }
    let mlen = (*rctx).digsize / core::mem::size_of::<u32>() as u32;
    let data = (*req).result as *mut u32;
    for count in 0..mlen {
        put_unaligned(readl((*ctx).cryp.base.add(STARFIVE_HASH_SHARDR as usize)), data.add(count as usize));
    }
    0
}

unsafe fn starfive_hash_done_task(cryp: *mut starfive_cryp_dev) {
    let mut err = (*cryp).err;
    if err == 0 { err = starfive_hash_copy_hash((*cryp).req.hreq); }
    crypto_finalize_hash_request((*cryp).engine, (*cryp).req.hreq, err);
}

unsafe extern "C" fn starfive_hash_one_request(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    let req = container_of!(areq, ahash_request, base);
    let ctx = crypto_ahash_ctx(crypto_ahash_reqtfm(req));
    let rctx = (*ctx).rctx;
    let cryp = (*ctx).cryp;
    writel(STARFIVE_HASH_RESET, (*cryp).base.add(STARFIVE_HASH_SHACSR as usize));
    if starfive_hash_wait_busy(cryp) != 0 { return dev_err_probe((*cryp).dev, -ETIMEDOUT, c"Error resetting hardware\n".as_ptr()); }
    (*rctx).csr.hash.v = 0;
    (*rctx).csr.hash.mode = (*ctx).hash_mode;
    if (*ctx).is_hmac { let ret = starfive_hash_hmac_key(ctx); if ret != 0 { return ret; } }
    else { (*rctx).csr.hash.start = 1; (*rctx).csr.hash.firstb = 1; writel((*rctx).csr.hash.v, (*cryp).base.add(STARFIVE_HASH_SHACSR as usize)); }
    if (*rctx).total != 0 {
        starfive_hash_dma_init(cryp);
        let mut tsg = (*rctx).in_sg;
        for i in 0..(*rctx).in_sg_len {
            let n = dma_map_sg((*cryp).dev, tsg, 1, DMA_TO_DEVICE);
            if n == 0 { return -ENOMEM; }
            let ret = starfive_hash_dma_xfer(cryp, tsg);
            dma_unmap_sg((*cryp).dev, tsg, 1, DMA_TO_DEVICE);
            if ret != 0 { return ret; }
            tsg = sg_next(tsg);
            if tsg.is_null() && i + 1 < (*rctx).in_sg_len { break; }
        }
    }
    starfive_hash_start(cryp);
    if starfive_hash_wait_busy(cryp) != 0 { return dev_err_probe((*cryp).dev, -ETIMEDOUT, c"Error generating digest\n".as_ptr()); }
    if (*ctx).is_hmac { (*cryp).err = starfive_hash_wait_hmac_done(cryp); }
    starfive_hash_done_task(cryp);
    0
}

// The remaining callbacks are direct fallback-driver forwarding wrappers.
unsafe fn starfive_hash_init(req: *mut ahash_request) -> c_int { starfive_hash_forward(req, 0) }
unsafe fn starfive_hash_update(req: *mut ahash_request) -> c_int { starfive_hash_forward(req, 1) }
unsafe fn starfive_hash_final(req: *mut ahash_request) -> c_int { starfive_hash_forward(req, 2) }
unsafe fn starfive_hash_finup(req: *mut ahash_request) -> c_int { starfive_hash_forward(req, 3) }

unsafe fn starfive_hash_forward(req: *mut ahash_request, op: c_int) -> c_int {
    let rctx = ahash_request_ctx(req); let ctx = crypto_ahash_ctx(crypto_ahash_reqtfm(req));
    ahash_request_set_tfm(&mut (*rctx).ahash_fbk_req, (*ctx).ahash_fbk);
    ahash_request_set_callback(&mut (*rctx).ahash_fbk_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP,
        (*req).base.complete, (*req).base.data);
    ahash_request_set_crypt(&mut (*rctx).ahash_fbk_req, (*req).src, (*req).result, (*req).nbytes);
    match op { 0 => crypto_ahash_init(&mut (*rctx).ahash_fbk_req), 1 => crypto_ahash_update(&mut (*rctx).ahash_fbk_req), 2 => crypto_ahash_final(&mut (*rctx).ahash_fbk_req), _ => crypto_ahash_finup(&mut (*rctx).ahash_fbk_req) }
}

unsafe fn starfive_hash_digest(req: *mut ahash_request) -> c_int {
    let tfm = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(tfm); let rctx = ahash_request_ctx(req); let cryp = (*ctx).cryp;
    memset(rctx as *mut c_void, 0, core::mem::size_of::<starfive_cryp_request_ctx>());
    (*cryp).req.hreq = req; (*rctx).total = (*req).nbytes; (*rctx).in_sg = (*req).src;
    (*rctx).blksize = crypto_tfm_alg_blocksize(crypto_ahash_tfm(tfm)); (*rctx).digsize = crypto_ahash_digestsize(tfm);
    let sg_len = sg_nents_for_len((*rctx).in_sg, (*rctx).total); if sg_len < 0 { return sg_len; }
    (*rctx).in_sg_len = sg_len as u32; (*ctx).rctx = rctx;
    crypto_transfer_hash_request_to_engine((*cryp).engine, req)
}

unsafe fn starfive_hash_export(req: *mut ahash_request, out: *mut c_void) -> c_int { starfive_hash_state_io(req, out, 0) }
unsafe fn starfive_hash_import(req: *mut ahash_request, input: *const c_void) -> c_int { starfive_hash_state_io(req, input as *mut c_void, 1) }
unsafe fn starfive_hash_state_io(req: *mut ahash_request, data: *mut c_void, import: c_int) -> c_int {
    let rctx = ahash_request_ctx(req); let ctx = crypto_ahash_ctx(crypto_ahash_reqtfm(req));
    ahash_request_set_tfm(&mut (*rctx).ahash_fbk_req, (*ctx).ahash_fbk);
    ahash_request_set_callback(&mut (*rctx).ahash_fbk_req, (*req).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP, (*req).base.complete, (*req).base.data);
    if import == 0 { crypto_ahash_export(&mut (*rctx).ahash_fbk_req, data) } else { crypto_ahash_import(&mut (*rctx).ahash_fbk_req, data) }
}

unsafe fn starfive_hash_init_tfm(hash: *mut crypto_ahash, alg_name: *const c_char, mode: u32, is_hmac: bool) -> c_int {
    let ctx = crypto_ahash_ctx(hash);
    (*ctx).cryp = starfive_cryp_find_dev(ctx);
    if (*ctx).cryp.is_null() { return -ENODEV; }
    (*ctx).ahash_fbk = crypto_alloc_ahash(alg_name, 0, CRYPTO_ALG_NEED_FALLBACK);
    if IS_ERR((*ctx).ahash_fbk) { return dev_err_probe((*(*ctx).cryp).dev, PTR_ERR((*ctx).ahash_fbk), c"starfive_hash: Could not load fallback driver.\n".as_ptr()); }
    crypto_ahash_set_statesize(hash, crypto_ahash_statesize((*ctx).ahash_fbk));
    crypto_ahash_set_reqsize(hash, core::mem::size_of::<starfive_cryp_request_ctx>() + crypto_ahash_reqsize((*ctx).ahash_fbk));
    (*ctx).is_hmac = is_hmac; (*ctx).hash_mode = mode; 0
}

unsafe fn starfive_hash_exit_tfm(hash: *mut crypto_ahash) {
    let ctx = crypto_ahash_ctx(hash); crypto_free_ahash((*ctx).ahash_fbk);
}

unsafe fn starfive_hash_long_setkey(ctx: *mut starfive_cryp_ctx, key: *const u8, keylen: u32, alg_name: *const c_char) -> c_int {
    let mut wait = core::mem::MaybeUninit::<crypto_wait>::zeroed().assume_init();
    let tfm = crypto_alloc_ahash(alg_name, 0, 0); if IS_ERR(tfm) { return PTR_ERR(tfm); }
    let req = ahash_request_alloc(tfm, GFP_KERNEL); if req.is_null() { crypto_free_ahash(tfm); return -ENOMEM; }
    crypto_init_wait(&mut wait); ahash_request_set_callback(req, CRYPTO_TFM_REQ_MAY_BACKLOG, crypto_req_done, &mut wait as *mut _ as *mut c_void); crypto_ahash_clear_flags(tfm, !0);
    let buf = kzalloc(keylen as usize + STARFIVE_HASH_BUFLEN, GFP_KERNEL); if buf.is_null() { ahash_request_free(req); crypto_free_ahash(tfm); return -ENOMEM; }
    memcpy(buf, key as *const c_void, keylen as usize); let mut sg = core::mem::zeroed(); sg_init_one(&mut sg, buf, keylen); ahash_request_set_crypt(req, &mut sg, (*ctx).key, keylen);
    let ret = crypto_wait_req(crypto_ahash_digest(req), &mut wait); kfree(buf); ahash_request_free(req); crypto_free_ahash(tfm); ret
}

unsafe fn starfive_hash_setkey(hash: *mut crypto_ahash, key: *const u8, keylen: u32) -> c_int {
    let ctx = crypto_ahash_ctx(hash); let digestsize = crypto_ahash_digestsize(hash); let blocksize = crypto_ahash_blocksize(hash);
    crypto_ahash_setkey((*ctx).ahash_fbk, key, keylen);
    if keylen <= blocksize { memcpy((*ctx).key as *mut c_void, key as *const c_void, keylen as usize); (*ctx).keylen = keylen; return 0; }
    (*ctx).keylen = digestsize;
    let alg = match digestsize { SHA224_DIGEST_SIZE => c"sha224-starfive", SHA256_DIGEST_SIZE if (*ctx).hash_mode == STARFIVE_HASH_SM3 => c"sm3-starfive", SHA256_DIGEST_SIZE => c"sha256-starfive", SHA384_DIGEST_SIZE => c"sha384-starfive", SHA512_DIGEST_SIZE => c"sha512-starfive", _ => return -EINVAL };
    starfive_hash_long_setkey(ctx, key, keylen, alg.as_ptr())
}

unsafe fn starfive_sha224_init_tfm(h: *mut crypto_ahash) -> c_int { starfive_hash_init_tfm(h, c"sha224-lib".as_ptr(), STARFIVE_HASH_SHA224, false) }
unsafe fn starfive_sha256_init_tfm(h: *mut crypto_ahash) -> c_int { starfive_hash_init_tfm(h, c"sha256-lib".as_ptr(), STARFIVE_HASH_SHA256, false) }
unsafe fn starfive_sha384_init_tfm(h: *mut crypto_ahash) -> c_int { starfive_hash_init_tfm(h, c"sha384-lib".as_ptr(), STARFIVE_HASH_SHA384, false) }
unsafe fn starfive_sha512_init_tfm(h: *mut crypto_ahash) -> c_int { starfive_hash_init_tfm(h, c"sha512-lib".as_ptr(), STARFIVE_HASH_SHA512, false) }
unsafe fn starfive_sm3_init_tfm(h: *mut crypto_ahash) -> c_int { starfive_hash_init_tfm(h, c"sm3-lib".as_ptr(), STARFIVE_HASH_SM3, false) }
unsafe fn starfive_hmac_sha224_init_tfm(h: *mut crypto_ahash) -> c_int { starfive_hash_init_tfm(h, c"hmac-sha224-lib".as_ptr(), STARFIVE_HASH_SHA224, true) }
unsafe fn starfive_hmac_sha256_init_tfm(h: *mut crypto_ahash) -> c_int { starfive_hash_init_tfm(h, c"hmac-sha256-lib".as_ptr(), STARFIVE_HASH_SHA256, true) }
unsafe fn starfive_hmac_sha384_init_tfm(h: *mut crypto_ahash) -> c_int { starfive_hash_init_tfm(h, c"hmac-sha384-lib".as_ptr(), STARFIVE_HASH_SHA384, true) }
unsafe fn starfive_hmac_sha512_init_tfm(h: *mut crypto_ahash) -> c_int { starfive_hash_init_tfm(h, c"hmac-sha512-lib".as_ptr(), STARFIVE_HASH_SHA512, true) }
unsafe fn starfive_hmac_sm3_init_tfm(h: *mut crypto_ahash) -> c_int { starfive_hash_init_tfm(h, c"hmac(sm3-lib)".as_ptr(), STARFIVE_HASH_SM3, true) }

// Algorithm descriptors and registration are represented with the same external kernel ABI.
extern "C" {
    static mut algs_sha2_sm3: [ahash_engine_alg; 10];
}

pub unsafe fn starfive_hash_register_algs() -> c_int {
    crypto_engine_register_ahashes(algs_sha2_sm3.as_mut_ptr(), 10)
}

pub unsafe fn starfive_hash_unregister_algs() {
    crypto_engine_unregister_ahashes(algs_sha2_sm3.as_mut_ptr(), 10);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
