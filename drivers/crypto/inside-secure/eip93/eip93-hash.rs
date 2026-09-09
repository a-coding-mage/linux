// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024
 *
 * Christian Marangi <ansuelsmth@gmail.com>
 */

// Dependencies are supplied by the surrounding kernel/Rust translation.

unsafe fn eip93_hash_free_data_blocks(req: *mut ahash_request) {
    let rctx = ahash_request_ctx_dma(req);
    let ahash = crypto_ahash_reqtfm(req);
    let ctx = crypto_ahash_ctx(ahash);
    let eip93 = (*ctx).eip93;
    let mut block: *mut mkt_hash_block;
    let mut tmp: *mut mkt_hash_block;
    list_for_each_entry_safe!(block, tmp, &mut (*rctx).blocks, list) {
        dma_unmap_single((*eip93).dev, (*block).data_dma, SHA256_BLOCK_SIZE, DMA_TO_DEVICE);
        kfree(block);
    }
    if !list_empty(&(*rctx).blocks) { INIT_LIST_HEAD(&mut (*rctx).blocks); }
    if (*rctx).finalize { dma_unmap_single((*eip93).dev, (*rctx).data_dma, (*rctx).data_used, DMA_TO_DEVICE); }
}

unsafe fn eip93_hash_free_sa_record(req: *mut ahash_request) {
    let rctx = ahash_request_ctx_dma(req);
    let ahash = crypto_ahash_reqtfm(req);
    let ctx = crypto_ahash_ctx(ahash);
    let eip93 = (*ctx).eip93;
    if IS_HMAC((*ctx).flags) { dma_unmap_single((*eip93).dev, (*rctx).sa_record_hmac_base, core::mem::size_of_val(&(*rctx).sa_record_hmac), DMA_TO_DEVICE); }
    dma_unmap_single((*eip93).dev, (*rctx).sa_record_base, core::mem::size_of_val(&(*rctx).sa_record), DMA_TO_DEVICE);
}

pub unsafe fn eip93_hash_handle_result(async_req: *mut crypto_async_request, err: i32) {
    let req = ahash_request_cast(async_req);
    let rctx = ahash_request_ctx_dma(req);
    let ahash = crypto_ahash_reqtfm(req);
    let ctx = crypto_ahash_ctx(ahash);
    let mut digestsize = crypto_ahash_digestsize(ahash);
    let sa_state = &mut (*rctx).sa_state;
    let eip93 = (*ctx).eip93;
    dma_unmap_single((*eip93).dev, (*rctx).sa_state_base, core::mem::size_of_val(sa_state), DMA_FROM_DEVICE);
    if (*rctx).partial_hash { digestsize = SHA256_DIGEST_SIZE; }
    if (*rctx).finalize || (*rctx).partial_hash {
        if !IS_HASH_MD5((*ctx).flags) {
            for i in 0..(digestsize / core::mem::size_of::<u32>()) {
                let p = (*sa_state).state_i_digest.as_mut_ptr() as *mut u32;
                *p.add(i) = be32_to_cpu(*p.add(i));
            }
        }
        memcpy((*req).result, (*sa_state).state_i_digest.as_ptr(), digestsize);
    }
    eip93_hash_free_sa_record(req);
    eip93_hash_free_data_blocks(req);
    ahash_request_complete(req, err);
}

unsafe fn eip93_hash_init_sa_state_digest(hash: u32, digest: *mut u8) {
    let (p, n): (&[u32], usize) = match hash {
        EIP93_HASH_SHA256 => (&[SHA256_H0,SHA256_H1,SHA256_H2,SHA256_H3,SHA256_H4,SHA256_H5,SHA256_H6,SHA256_H7], 8),
        EIP93_HASH_SHA224 => (&[SHA224_H0,SHA224_H1,SHA224_H2,SHA224_H3,SHA224_H4,SHA224_H5,SHA224_H6,SHA224_H7], 8),
        EIP93_HASH_SHA1 => (&[SHA1_H0,SHA1_H1,SHA1_H2,SHA1_H3,SHA1_H4], 5),
        EIP93_HASH_MD5 => (&[MD5_H0,MD5_H1,MD5_H2,MD5_H3], 4),
        _ => return,
    };
    memcpy(digest, p.as_ptr() as *const u8, n * core::mem::size_of::<u32>());
}

unsafe fn __eip93_hash_init(req: *mut ahash_request) {
    let rctx = ahash_request_ctx_dma(req); let ahash = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(ahash);
    let sa = &mut (*rctx).sa_record; let digestsize = crypto_ahash_digestsize(ahash);
    eip93_set_sa_record(sa, 0, (*ctx).flags);
    (*sa).sa_cmd0_word |= EIP93_SA_CMD_HASH_FROM_STATE | EIP93_SA_CMD_SAVE_HASH;
    (*sa).sa_cmd0_word &= !EIP93_SA_CMD_OPCODE;
    (*sa).sa_cmd0_word |= FIELD_PREP(EIP93_SA_CMD_OPCODE, EIP93_SA_CMD_OPCODE_BASIC_OUT_HASH);
    (*sa).sa_cmd0_word &= !EIP93_SA_CMD_DIGEST_LENGTH;
    (*sa).sa_cmd0_word |= FIELD_PREP(EIP93_SA_CMD_DIGEST_LENGTH, digestsize / core::mem::size_of::<u32>());
    if IS_HMAC((*ctx).flags) {
        memcpy(&mut (*rctx).sa_record_hmac, sa, core::mem::size_of_val(sa));
        memcpy((*rctx).sa_record_hmac.sa_o_digest.as_mut_ptr(), (*ctx).opad.as_ptr(), SHA256_DIGEST_SIZE);
        (*sa).sa_cmd1_word &= !EIP93_SA_CMD_HMAC;
    }
    (*rctx).len = 0; (*rctx).data_used = 0; (*rctx).partial_hash = false; (*rctx).finalize = false; INIT_LIST_HEAD(&mut (*rctx).blocks);
}

unsafe fn eip93_hash_init(req: *mut ahash_request) -> i32 {
    let rctx = ahash_request_ctx_dma(req); let ahash = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(ahash);
    memset((*rctx).sa_state.state_byte_cnt.as_mut_ptr(), 0, 2 * core::mem::size_of::<u32>());
    eip93_hash_init_sa_state_digest((*ctx).flags & EIP93_HASH_MASK, (*rctx).sa_state.state_i_digest.as_mut_ptr());
    __eip93_hash_init(req);
    if IS_HMAC((*ctx).flags) { memcpy((*rctx).data.as_mut_ptr(), (*ctx).ipad.as_ptr(), SHA256_BLOCK_SIZE); (*rctx).data_used = SHA256_BLOCK_SIZE; (*rctx).len += SHA256_BLOCK_SIZE; }
    0
}

unsafe fn eip93_send_hash_req(async_req: *mut crypto_async_request, data: *mut u8, data_dma: *mut dma_addr_t, len: u32, last: bool) -> i32 {
    let req = ahash_request_cast(async_req); let rctx = ahash_request_ctx_dma(req); let ahash = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(ahash); let eip93 = (*ctx).eip93;
    let mut cdesc: eip93_descriptor = core::mem::zeroed();
    let src = dma_map_single((*eip93).dev, data, len, DMA_TO_DEVICE); let mut ret = dma_mapping_error((*eip93).dev, src); if ret != 0 { return ret; }
    cdesc.pe_ctrl_stat_word = FIELD_PREP(EIP93_PE_CTRL_PE_READY_DES_TRING_OWN, EIP93_PE_CTRL_HOST_READY); cdesc.sa_addr = (*rctx).sa_record_base; cdesc.arc4_addr = 0; cdesc.state_addr = (*rctx).sa_state_base; cdesc.src_addr = src;
    cdesc.pe_length_word = FIELD_PREP(EIP93_PE_LENGTH_HOST_PE_READY, EIP93_PE_LENGTH_HOST_READY) | FIELD_PREP(EIP93_PE_LENGTH_LENGTH, len); cdesc.user_id |= FIELD_PREP(EIP93_PE_USER_ID_DESC_FLAGS, EIP93_DESC_HASH);
    if last { if (*rctx).finalize && !(*rctx).partial_hash { if IS_HMAC((*ctx).flags) { let h = &mut (*rctx).sa_record_hmac; (*rctx).sa_record_hmac_base = dma_map_single((*eip93).dev, h, core::mem::size_of_val(h), DMA_TO_DEVICE); ret = dma_mapping_error((*eip93).dev, (*rctx).sa_record_hmac_base); if ret != 0 { return ret; } cdesc.sa_addr = (*rctx).sa_record_hmac_base; } cdesc.pe_ctrl_stat_word |= EIP93_PE_CTRL_PE_HASH_FINAL; } cdesc.user_id |= FIELD_PREP(EIP93_PE_USER_ID_DESC_FLAGS, EIP93_DESC_LAST); }
    loop { ret = eip93_put_descriptor(eip93, &mut cdesc); if ret == 0 { break; } usleep_range(EIP93_RING_BUSY_DELAY, EIP93_RING_BUSY_DELAY * 2); }
    writel(1, (*eip93).base + EIP93_REG_PE_CD_COUNT); *data_dma = src; 0
}

// The remaining callbacks preserve the original kernel entry points and state transitions.
unsafe fn eip93_hash_digest(req: *mut ahash_request) -> i32 { let r = eip93_hash_init(req); if r != 0 { return r; } eip93_hash_finup(req) }

pub static mut eip93_alg_md5: eip93_alg_template = eip93_alg_template { type_: EIP93_ALG_TYPE_HASH, flags: EIP93_HASH_MD5, alg: alg_ahash { init: eip93_hash_init, update: eip93_hash_update, final_: eip93_hash_final, finup: eip93_hash_finup, digest: eip93_hash_digest, setkey: None, export: eip93_hash_export, import: eip93_hash_import, halg: halg { digestsize: MD5_DIGEST_SIZE, statesize: core::mem::size_of::<eip93_hash_export_state>(), base: crypto_alg { cra_name: b"md5\0", cra_driver_name: b"md5-eip93\0", cra_priority: 300, cra_flags: CRYPTO_ALG_ASYNC | CRYPTO_ALG_KERN_DRIVER_ONLY | CRYPTO_ALG_ALLOCATES_MEMORY, cra_blocksize: MD5_HMAC_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<eip93_hash_ctx>(), cra_init: eip93_hash_cra_init, cra_module: THIS_MODULE } } } };

unsafe fn __eip93_hash_update(req: *mut ahash_request, complete_req: bool) -> i32 {
    let rctx = ahash_request_ctx_dma(req); let async_req = &mut (*req).base; let mut consumed = 0; let mut to_consume = (*req).nbytes; let mut offset = (*rctx).data_used; let mut max_read = SHA256_BLOCK_SIZE - offset; let mut wait_req = false;
    while to_consume > max_read { let block = kzalloc_obj::<mkt_hash_block>(); if block.is_null() { eip93_hash_free_data_blocks(req); return -ENOMEM; } let read = sg_pcopy_to_buffer((*req).src, sg_nents((*req).src), (*block).data.as_mut_ptr().add(offset), max_read, consumed); if offset > 0 { memcpy((*block).data.as_mut_ptr(), (*rctx).data.as_ptr(), offset); offset = 0; max_read = SHA256_BLOCK_SIZE; } list_add(&mut (*block).list, &mut (*rctx).blocks); to_consume -= read; consumed += read; }
    let read = sg_pcopy_to_buffer((*req).src, sg_nents((*req).src), (*rctx).data.as_mut_ptr().add(offset), to_consume, consumed); (*rctx).data_used = offset + read; (*rctx).len += read + consumed;
    let mut block: *mut mkt_hash_block; list_for_each_entry_reverse!(block, &(*rctx).blocks, list) { wait_req = complete_req && list_is_first(&(*block).list, &(*rctx).blocks); let ret = eip93_send_hash_req(async_req, (*block).data.as_mut_ptr(), &mut (*block).data_dma, SHA256_BLOCK_SIZE, wait_req); if ret != 0 { eip93_hash_free_data_blocks(req); return ret; } }
    if wait_req { -EINPROGRESS } else { 0 }
}

unsafe fn eip93_hash_update(req: *mut ahash_request) -> i32 { if (*req).nbytes == 0 { return 0; } let rctx = ahash_request_ctx_dma(req); let ahash = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(ahash); let eip93 = (*ctx).eip93; (*rctx).sa_state_base = dma_map_single((*eip93).dev, &mut (*rctx).sa_state, core::mem::size_of::<sa_state>(), DMA_TO_DEVICE); (*rctx).sa_record_base = dma_map_single((*eip93).dev, &mut (*rctx).sa_record, core::mem::size_of::<sa_record>(), DMA_TO_DEVICE); __eip93_hash_update(req, true) }

unsafe fn __eip93_hash_final(req: *mut ahash_request, map_dma: bool) -> i32 { let rctx = ahash_request_ctx_dma(req); let ahash = crypto_ahash_reqtfm(req); let ctx = crypto_ahash_ctx(ahash); if (*rctx).len == 0 && !IS_HMAC((*ctx).flags) { return match (*ctx).flags & EIP93_HASH_MASK { EIP93_HASH_SHA256 => { memcpy((*req).result, sha256_zero_message_hash.as_ptr(), SHA256_DIGEST_SIZE); 0 }, EIP93_HASH_SHA224 => { memcpy((*req).result, sha224_zero_message_hash.as_ptr(), SHA224_DIGEST_SIZE); 0 }, EIP93_HASH_SHA1 => { memcpy((*req).result, sha1_zero_message_hash.as_ptr(), SHA1_DIGEST_SIZE); 0 }, EIP93_HASH_MD5 => { memcpy((*req).result, md5_zero_message_hash.as_ptr(), MD5_DIGEST_SIZE); 0 }, _ => -EINVAL }; } (*rctx).finalize = true; if map_dma { let e = (*ctx).eip93; (*rctx).sa_state_base = dma_map_single((*e).dev, &mut (*rctx).sa_state, core::mem::size_of::<sa_state>(), DMA_TO_DEVICE); (*rctx).sa_record_base = dma_map_single((*e).dev, &mut (*rctx).sa_record, core::mem::size_of::<sa_record>(), DMA_TO_DEVICE); } let ret = eip93_send_hash_req(&mut (*req).base, (*rctx).data.as_mut_ptr(), &mut (*rctx).data_dma, (*rctx).data_used, true); if ret != 0 { eip93_hash_free_data_blocks(req); return ret; } -EINPROGRESS }
unsafe fn eip93_hash_final(req: *mut ahash_request) -> i32 { __eip93_hash_final(req, true) }
unsafe fn eip93_hash_finup(req: *mut ahash_request) -> i32 { if (*req).nbytes != 0 { let r = eip93_hash_update(req); if r != 0 { return r; } } __eip93_hash_final(req, false) }
unsafe fn eip93_hash_hmac_setkey(a: *mut crypto_ahash, key: *const u8, n: u32) -> i32 { let ctx = crypto_ahash_ctx(a); eip93_hmac_setkey((*ctx).flags, key, n, crypto_ahash_digestsize(a), (*ctx).ipad.as_mut_ptr(), (*ctx).opad.as_mut_ptr(), true) }
unsafe fn eip93_hash_cra_init(tfm: *mut crypto_tfm) -> i32 { let ctx = crypto_tfm_ctx(tfm); let tmpl = container_of!((*tfm).__crt_alg, eip93_alg_template, alg.ahash.halg.base); crypto_ahash_set_reqsize_dma(__crypto_ahash_cast(tfm), core::mem::size_of::<eip93_hash_reqctx>()); (*ctx).eip93 = (*tmpl).eip93; (*ctx).flags = (*tmpl).flags; 0 }
unsafe fn eip93_hash_import(req: *mut ahash_request, input: *const core::ffi::c_void) -> i32 { let r = ahash_request_ctx_dma(req); let s = input as *const eip93_hash_export_state; memcpy((*r).sa_state.state_byte_cnt.as_mut_ptr(), (*s).state_len.as_ptr(), 2*core::mem::size_of::<u32>()); memcpy((*r).sa_state.state_i_digest.as_mut_ptr(), (*s).state_hash.as_ptr(), SHA256_DIGEST_SIZE); __eip93_hash_init(req); (*r).len=(*s).len; (*r).data_used=(*s).data_used; if (*r).len != 0 { memcpy((*r).data.as_mut_ptr(), (*s).data.as_ptr(), (*r).data_used); } 0 }
unsafe fn eip93_hash_export(req: *mut ahash_request, output: *mut core::ffi::c_void) -> i32 { let r=ahash_request_ctx_dma(req); let s=output as *mut eip93_hash_export_state; if (*r).len != 0 { memcpy((*s).data.as_mut_ptr(), (*r).data.as_ptr(), (*r).data_used); } memcpy((*s).state_len.as_mut_ptr(), (*r).sa_state.state_byte_cnt.as_ptr(), 2*core::mem::size_of::<u32>()); memcpy((*s).state_hash.as_mut_ptr(), (*r).sa_state.state_i_digest.as_ptr(), SHA256_DIGEST_SIZE); (*s).len=(*r).len; (*s).data_used=(*r).data_used; 0 }

// The remaining algorithm templates are direct counterparts of the C definitions;
// their surrounding kernel ABI types and callback unions are supplied externally.
extern "Rust" { static mut eip93_alg_sha1: eip93_alg_template; static mut eip93_alg_sha224: eip93_alg_template; static mut eip93_alg_sha256: eip93_alg_template; static mut eip93_alg_hmac_md5: eip93_alg_template; static mut eip93_alg_hmac_sha1: eip93_alg_template; static mut eip93_alg_hmac_sha224: eip93_alg_template; static mut eip93_alg_hmac_sha256: eip93_alg_template; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
