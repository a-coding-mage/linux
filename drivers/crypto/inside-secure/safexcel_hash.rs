// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017 Marvell
 *
 * Antoine Tenart <antoine.tenart@free-electrons.com>
 */

// include <crypto/aes.h>
// include <crypto/hmac.h>
// include <crypto/md5.h>
// include <crypto/sha1.h>
// include <crypto/sha2.h>
// include <crypto/sha3.h>
// include <crypto/skcipher.h>
// include <crypto/sm3.h>
// include <crypto/internal/cipher.h>
// include <linux/device.h>
// include <linux/dma-mapping.h>
// include <linux/dmapool.h>

// include "safexcel.h"

#[repr(C)]
struct safexcel_ahash_ctx {
	struct safexcel_context base;

	u32 alg;
	u8  key_sz;
	bool cbcmac;
	bool do_fallback;
	bool fb_init_done;
	bool fb_do_setkey;

	*mut aes_enckeyaes;
	*mut crypto_ahashfback;
	*mut crypto_shashshpre;
	*mut shash_descshdesc;
};

#[repr(C)]
struct safexcel_ahash_req {
	bool last_req;
	bool finish;
	bool hmac;
	bool needs_inv;
	bool hmac_zlen;
	bool len_is_le;
	bool not_first;
	bool xcbcmac;

	i32 nents;
	dma_addr_t result_dma;

	u32 digest;

	u8 state_sz;    /* expected state size, only set once */
	u8 block_sz;    /* block size, only set once */
	u8 digest_sz;   /* output digest size, only set once */
	u32 state[SHA3_512_BLOCK_SIZE /
		     core::mem::size_of::<u32>()] __aligned(core::mem::size_of::<u32>());

	u64 len;
	u64 processed;

	u8 cache[HASH_CACHE_SIZE] __aligned(core::mem::size_of::<u32>());
	dma_addr_t cache_dma;
	u32 i32 cache_sz;

	u8 cache_next[HASH_CACHE_SIZE] __aligned(core::mem::size_of::<u32>());
};

#[inline]
unsafe fn u64 safexcel_queued_len(*mut safexcel_ahash_reqreq)
{
	return req->len - req->processed;
}

unsafe fn void safexcel_hash_token(*mut safexcel_command_desccdesc,
				u32 input_length, u32 result_length,
				bool cbcmac)
{
	*mut safexcel_tokentoken =
		(*mut safexcel_token)cdesc->control_data.token;

	token[0].opcode = EIP197_TOKEN_OPCODE_DIRECTION;
	token[0].packet_length = input_length;
	token[0].instructions = EIP197_TOKEN_INS_TYPE_HASH;

	input_length &= 15;
	if ((cbcmac && input_length)) {
		token[0].stat =  0;
		token[1].opcode = EIP197_TOKEN_OPCODE_INSERT;
		token[1].packet_length = 16 - input_length;
		token[1].stat = EIP197_TOKEN_STAT_LAST_HASH;
		token[1].instructions = EIP197_TOKEN_INS_TYPE_HASH;
	} else {
		token[0].stat = EIP197_TOKEN_STAT_LAST_HASH;
		eip197_noop_token(&token[1]);
	}

	token[2].opcode = EIP197_TOKEN_OPCODE_INSERT;
	token[2].stat = EIP197_TOKEN_STAT_LAST_HASH |
			EIP197_TOKEN_STAT_LAST_PACKET;
	token[2].packet_length = result_length;
	token[2].instructions = EIP197_TOKEN_INS_TYPE_OUTPUT |
				EIP197_TOKEN_INS_INSERT_HASH_DIGEST;

	eip197_noop_token(&token[3]);
}

unsafe fn void safexcel_context_control(*mut safexcel_ahash_ctxctx,
				     *mut safexcel_ahash_reqreq,
				     *mut safexcel_command_desccdesc)
{
	*mut safexcel_crypto_privpriv = ctx->base.priv;
	u64 count = 0;

	cdesc->control_data.control0 = ctx->alg;
	cdesc->control_data.control1 = 0;

	/*
	 * Copy the input digest if needed, and setup the context
	 * fields. Do this now as we need it to setup the first command
	 * descriptor.
	 */
	if ((req->digest == CONTEXT_CONTROL_DIGEST_XCM)) {
		if (req->xcbcmac)
			core::ptr::copy_nonoverlapping(ctx->base.ctxr->data, &ctx->base.ipad, ctx->key_sz);
		else
			core::ptr::copy_nonoverlapping(ctx->base.ctxr->data, req->state, req->state_sz);

		if (!req->finish && req->xcbcmac)
			cdesc->control_data.control0 |=
				CONTEXT_CONTROL_DIGEST_XCM |
				CONTEXT_CONTROL_TYPE_HASH_OUT  |
				CONTEXT_CONTROL_NO_FINISH_HASH |
				CONTEXT_CONTROL_SIZE(req->state_sz /
						     core::mem::size_of::<u32>());
		else
			cdesc->control_data.control0 |=
				CONTEXT_CONTROL_DIGEST_XCM |
				CONTEXT_CONTROL_TYPE_HASH_OUT  |
				CONTEXT_CONTROL_SIZE(req->state_sz /
						     core::mem::size_of::<u32>());
		return;
	} else if (!req->processed) {
		/* First - and possibly only - block of basic hash only */
		if (req->finish)
			cdesc->control_data.control0 |= req->digest |
				CONTEXT_CONTROL_TYPE_HASH_OUT |
				CONTEXT_CONTROL_RESTART_HASH  |
				/* ensure its not 0! */
				CONTEXT_CONTROL_SIZE(1);
		else
			cdesc->control_data.control0 |= req->digest |
				CONTEXT_CONTROL_TYPE_HASH_OUT  |
				CONTEXT_CONTROL_RESTART_HASH   |
				CONTEXT_CONTROL_NO_FINISH_HASH |
				/* ensure its not 0! */
				CONTEXT_CONTROL_SIZE(1);
		return;
	}

	/* Hash continuation or HMAC, setup (inner) digest from state */
	core::ptr::copy_nonoverlapping(ctx->base.ctxr->data, req->state, req->state_sz);

	if (req->finish) {
		/* Compute digest count for hash/HMAC finish operations */
		if ((req->digest == CONTEXT_CONTROL_DIGEST_PRECOMPUTED) ||
		    req->hmac_zlen || (req->processed != req->block_sz)) {
			count = req->processed / EIP197_COUNTER_BLOCK_SIZE;

			/* This is a hardware limitation, as the
			 * counter must fit into an u32. This represents
			 * a fairly big amount of input data, so we
			 * shouldn't see this.
			 */
			if ((count & 0xffffffff00000000ULL)) {
				dev_warn(priv->dev,
					 "Input data is too big\n");
				return;
			}
		}

		if ((req->digest == CONTEXT_CONTROL_DIGEST_PRECOMPUTED) ||
		    /* Special case: zero length HMAC */
		    req->hmac_zlen ||
		    /* PE HW < 4.4 cannot do HMAC continue, fake using hash */
		    (req->processed != req->block_sz)) {
			/* Basic hash continue operation, need digest + cnt */
			cdesc->control_data.control0 |=
				CONTEXT_CONTROL_SIZE((req->state_sz >> 2) + 1) |
				CONTEXT_CONTROL_TYPE_HASH_OUT |
				CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
			/* For zero-len HMAC, don't finalize, already padded! */
			if (req->hmac_zlen)
				cdesc->control_data.control0 |=
					CONTEXT_CONTROL_NO_FINISH_HASH;
			cdesc->control_data.control1 |=
				CONTEXT_CONTROL_DIGEST_CNT;
			ctx->base.ctxr->data[req->state_sz >> 2] =
				cpu_to_le32(count);
			req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;

			/* Clear zero-length HMAC flag for next operation! */
			req->hmac_zlen = false;
		} else { /* HMAC */
			/* Need outer digest for HMAC finalization */
			core::ptr::copy_nonoverlapping(ctx->base.ctxr->data + (req->state_sz >> 2),
			       &ctx->base.opad, req->state_sz);

			/* Single pass HMAC - no digest count */
			cdesc->control_data.control0 |=
				CONTEXT_CONTROL_SIZE(req->state_sz >> 1) |
				CONTEXT_CONTROL_TYPE_HASH_OUT |
				CONTEXT_CONTROL_DIGEST_HMAC;
		}
	} else { /* Hash continuation, do not finish yet */
		cdesc->control_data.control0 |=
			CONTEXT_CONTROL_SIZE(req->state_sz >> 2) |
			CONTEXT_CONTROL_DIGEST_PRECOMPUTED |
			CONTEXT_CONTROL_TYPE_HASH_OUT |
			CONTEXT_CONTROL_NO_FINISH_HASH;
	}
}

unsafe fn i32 safexcel_ahash_enqueue(*mut ahash_requestareq);

unsafe fn i32 safexcel_handle_req_result(*mut safexcel_crypto_privpriv,
				      i32 ring,
				      *mut crypto_async_requestasync,
				      bool *should_complete, i32 *ret)
{
	*mut safexcel_result_descrdesc;
	*mut ahash_requestareq = ahash_request_cast(async);
	*mut crypto_ahashahash = crypto_ahash_reqtfm(areq);
	*mut safexcel_ahash_reqsreq = ahash_request_ctx_dma(areq);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(ahash);
	u64 cache_len;

	*ret = 0;

	rdesc = safexcel_ring_next_rptr(priv, &priv->ring[ring].rdr);
	if (IS_ERR(rdesc)) {
		dev_err(priv->dev,
			"hash: result: could not retrieve the result descriptor\n");
		*ret = PTR_ERR(rdesc);
	} else {
		*ret = safexcel_rdesc_check_errors(priv, rdesc);
	}

	safexcel_complete(priv, ring);

	if (sreq->nents) {
		dma_unmap_sg(priv->dev, areq->src,
			     sg_nents_for_len(areq->src, areq->nbytes),
			     0);
		sreq->nents = 0;
	}

	if (sreq->result_dma) {
		dma_unmap_single(priv->dev, sreq->result_dma, sreq->digest_sz,
				 0);
		sreq->result_dma = 0;
	}

	if (sreq->cache_dma) {
		dma_unmap_single(priv->dev, sreq->cache_dma, sreq->cache_sz,
				 0);
		sreq->cache_dma = 0;
		sreq->cache_sz = 0;
	}

	if (sreq->finish) {
		if (sreq->hmac &&
		    (sreq->digest != CONTEXT_CONTROL_DIGEST_HMAC)) {
			/* Faking HMAC using hash - need to do outer hash */
			core::ptr::copy_nonoverlapping(sreq->cache, sreq->state,
			       crypto_ahash_digestsize(ahash));

			core::ptr::copy_nonoverlapping(sreq->state, &ctx->base.opad, sreq->digest_sz);

			sreq->len = sreq->block_sz +
				    crypto_ahash_digestsize(ahash);
			sreq->processed = sreq->block_sz;
			sreq->hmac = 0;

			if (priv->flags & EIP197_TRC_CACHE)
				ctx->base.needs_inv = true;
			areq->nbytes = 0;
			safexcel_ahash_enqueue(areq);

			*should_complete = false; /* Not done yet */
			return 1;
		}

		core::ptr::copy_nonoverlapping(areq->result, sreq->state,
		       crypto_ahash_digestsize(ahash));
	}

	cache_len = safexcel_queued_len(sreq);
	if (cache_len)
		core::ptr::copy_nonoverlapping(sreq->cache, sreq->cache_next, cache_len);

	*should_complete = true;

	return 1;
}

unsafe fn i32 safexcel_ahash_send_req(*mut crypto_async_requestasync, i32 ring,
				   i32 *commands, i32 *results)
{
	*mut ahash_requestareq = ahash_request_cast(async);
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_crypto_privpriv = ctx->base.priv;
	*mut safexcel_command_desccdesc, *first_cdesc = core::ptr::null_mut();
	*mut safexcel_result_descrdesc;
	*mut scatterlistsg;
	*mut safexcel_tokendmmy;
	i32 i, extra = 0, n_cdesc = 0, ret = 0, cache_len, skip = 0;
	u64 queued, len;

	queued = safexcel_queued_len(req);
	if (queued <= HASH_CACHE_SIZE)
		cache_len = queued;
	else
		cache_len = queued - areq->nbytes;

	if (!req->finish && !req->last_req) {
		/* If this is not the last request and the queued data does not
		 * fit into full cache blocks, cache it for the next send call.
		 */
		extra = queued & (HASH_CACHE_SIZE - 1);

		/* If this is not the last request and the queued data
		 * is a multiple of a block, cache the last one for now.
		 */
		if (!extra)
			extra = HASH_CACHE_SIZE;

		sg_pcopy_to_buffer(areq->src, sg_nents(areq->src),
				   req->cache_next, extra,
				   areq->nbytes - extra);

		queued -= extra;

		if (!queued) {
			*commands = 0;
			*results = 0;
			return 0;
		}

		extra = 0;
	}

	if ((req->xcbcmac && req->processed > AES_BLOCK_SIZE)) {
		if ((cache_len < AES_BLOCK_SIZE)) {
			/*
			 * Cache contains less than 1 full block, complete.
			 */
			extra = AES_BLOCK_SIZE - cache_len;
			if (queued > cache_len) {
				/* More data follows: borrow bytes */
				u64 tmp = queued - cache_len;

				skip = core::cmp::min(tmp, extra);
				sg_pcopy_to_buffer(areq->src,
					sg_nents(areq->src),
					req->cache + cache_len,
					skip, 0);
			}
			extra -= skip;
			core::ptr::write_bytes(req->cache + cache_len + skip, 0, extra);
			if (!ctx->cbcmac && extra) {
				// 10- padding for XCBCMAC & CMAC
				req->cache[cache_len + skip] = 0x80;
				// HW will use K2 iso K3 - compensate!
				for (i = 0; i < AES_BLOCK_SIZE / 4; i++) {
					u32 *cache = (*mut core::ffi::c_void)req->cache;
					u32 *ipad = ctx->base.ipad.word;
					u32 x;

					x = ipad[i] ^ ipad[i + 4];
					cache[i] ^= swab32(x);
				}
			}
			cache_len = AES_BLOCK_SIZE;
			queued = queued + extra;
		}

		/* XCBC continue: XOR previous result into 1st word */
		crypto_xor(req->cache, (*const u8)req->state, AES_BLOCK_SIZE);
	}

	len = queued;
	/* Add a command descriptor for the cached data, if any */
	if (cache_len) {
		req->cache_dma = dma_map_single(priv->dev, req->cache,
						cache_len, 0);
		if (dma_mapping_error(priv->dev, req->cache_dma))
			return -EINVAL;

		req->cache_sz = cache_len;
		first_cdesc = safexcel_add_cdesc(priv, ring, 1,
						 (cache_len == len),
						 req->cache_dma, cache_len,
						 len, ctx->base.ctxr_dma,
						 &dmmy);
		if (IS_ERR(first_cdesc)) {
			ret = PTR_ERR(first_cdesc);
			goto unmap_cache;
		}
		n_cdesc++;

		queued -= cache_len;
		if (!queued)
			goto send_command;
	}

	/* Now handle the current ahash request buffer(s) */
	req->nents = dma_map_sg(priv->dev, areq->src,
				sg_nents_for_len(areq->src,
						 areq->nbytes),
				0);
	if (!req->nents) {
		ret = -ENOMEM;
		goto cdesc_rollback;
	}

	for_each_sg(areq->src, sg, req->nents, i) {
		i32 sglen = sg_dma_len(sg);

		if ((sglen <= skip)) {
			skip -= sglen;
			continue;
		}

		/* Do not overflow the request */
		if ((queued + skip) <= sglen)
			sglen = queued;
		else
			sglen -= skip;

		cdesc = safexcel_add_cdesc(priv, ring, !n_cdesc,
					   !(queued - sglen),
					   sg_dma_address(sg) + skip, sglen,
					   len, ctx->base.ctxr_dma, &dmmy);
		if (IS_ERR(cdesc)) {
			ret = PTR_ERR(cdesc);
			goto unmap_sg;
		}

		if (!n_cdesc)
			first_cdesc = cdesc;
		n_cdesc++;

		queued -= sglen;
		if (!queued)
			break;
		skip = 0;
	}

send_command:
	/* Setup the context options */
	safexcel_context_control(ctx, req, first_cdesc);

	/* Add the token */
	safexcel_hash_token(first_cdesc, len, req->digest_sz, ctx->cbcmac);

	req->result_dma = dma_map_single(priv->dev, req->state, req->digest_sz,
					 0);
	if (dma_mapping_error(priv->dev, req->result_dma)) {
		ret = -EINVAL;
		goto unmap_sg;
	}

	/* Add a result descriptor */
	rdesc = safexcel_add_rdesc(priv, ring, 1, 1, req->result_dma,
				   req->digest_sz);
	if (IS_ERR(rdesc)) {
		ret = PTR_ERR(rdesc);
		goto unmap_result;
	}

	safexcel_rdr_req_set(priv, ring, rdesc, &areq->base);

	req->processed += len - extra;

	*commands = n_cdesc;
	*results = 1;
	return 0;

unmap_result:
	dma_unmap_single(priv->dev, req->result_dma, req->digest_sz,
			 0);
unmap_sg:
	if (req->nents) {
		dma_unmap_sg(priv->dev, areq->src,
			     sg_nents_for_len(areq->src, areq->nbytes),
			     0);
		req->nents = 0;
	}
cdesc_rollback:
	for (i = 0; i < n_cdesc; i++)
		safexcel_ring_rollback_wptr(priv, &priv->ring[ring].cdr);
unmap_cache:
	if (req->cache_dma) {
		dma_unmap_single(priv->dev, req->cache_dma, req->cache_sz,
				 0);
		req->cache_dma = 0;
		req->cache_sz = 0;
	}

	return ret;
}

unsafe fn i32 safexcel_handle_inv_result(*mut safexcel_crypto_privpriv,
				      i32 ring,
				      *mut crypto_async_requestasync,
				      bool *should_complete, i32 *ret)
{
	*mut safexcel_result_descrdesc;
	*mut ahash_requestareq = ahash_request_cast(async);
	*mut crypto_ahashahash = crypto_ahash_reqtfm(areq);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(ahash);
	i32 enq_ret;

	*ret = 0;

	rdesc = safexcel_ring_next_rptr(priv, &priv->ring[ring].rdr);
	if (IS_ERR(rdesc)) {
		dev_err(priv->dev,
			"hash: invalidate: could not retrieve the result descriptor\n");
		*ret = PTR_ERR(rdesc);
	} else {
		*ret = safexcel_rdesc_check_errors(priv, rdesc);
	}

	safexcel_complete(priv, ring);

	if (ctx->base.exit_inv) {
		dma_pool_free(priv->context_pool, ctx->base.ctxr,
			      ctx->base.ctxr_dma);

		*should_complete = true;
		return 1;
	}

	ring = safexcel_select_ring(priv);
	ctx->base.ring = ring;

	spin_lock_bh(&priv->ring[ring].queue_lock);
	enq_ret = crypto_enqueue_request(&priv->ring[ring].queue, async);
	spin_unlock_bh(&priv->ring[ring].queue_lock);

	if (enq_ret != -EINPROGRESS)
		*ret = enq_ret;

	queue_work(priv->ring[ring].workqueue,
		   &priv->ring[ring].work_data.work);

	*should_complete = false;

	return 1;
}

unsafe fn i32 safexcel_handle_result(*mut safexcel_crypto_privpriv, i32 ring,
				  *mut crypto_async_requestasync,
				  bool *should_complete, i32 *ret)
{
	*mut ahash_requestareq = ahash_request_cast(async);
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);
	i32 err;

	BUG_ON(!(priv->flags & EIP197_TRC_CACHE) && req->needs_inv);

	if (req->needs_inv) {
		req->needs_inv = false;
		err = safexcel_handle_inv_result(priv, ring, async,
						 should_complete, ret);
	} else {
		err = safexcel_handle_req_result(priv, ring, async,
						 should_complete, ret);
	}

	return err;
}

unsafe fn i32 safexcel_ahash_send_inv(*mut crypto_async_requestasync,
				   i32 ring, i32 *commands, i32 *results)
{
	*mut ahash_requestareq = ahash_request_cast(async);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	i32 ret;

	ret = safexcel_invalidate_cache(async, ctx->base.priv,
					ctx->base.ctxr_dma, ring);
	if ((ret))
		return ret;

	*commands = 1;
	*results = 1;

	return 0;
}

unsafe fn i32 safexcel_ahash_send(*mut crypto_async_requestasync,
			       i32 ring, i32 *commands, i32 *results)
{
	*mut ahash_requestareq = ahash_request_cast(async);
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);
	i32 ret;

	if (req->needs_inv)
		ret = safexcel_ahash_send_inv(async, ring, commands, results);
	else
		ret = safexcel_ahash_send_req(async, ring, commands, results);

	return ret;
}

unsafe fn i32 safexcel_ahash_exit_inv(*mut crypto_tfmtfm)
{
	*mut safexcel_ahash_ctxctx = crypto_tfm_ctx(tfm);
	*mut safexcel_crypto_privpriv = ctx->base.priv;
	EIP197_REQUEST_ON_STACK(req, ahash, EIP197_AHASH_REQ_SIZE);
	*mut safexcel_ahash_reqrctx = ahash_request_ctx_dma(req);
	DECLARE_CRYPTO_WAIT(result);
	i32 ring = ctx->base.ring;
	i32 err;

	core::ptr::write_bytes(req, 0, EIP197_AHASH_REQ_SIZE);

	/* create invalidation request */
	init_completion(&result.completion);
	ahash_request_set_callback(req, CRYPTO_TFM_REQ_MAY_BACKLOG,
				   crypto_req_done, &result);

	ahash_request_set_tfm(req, __crypto_ahash_cast(tfm));
	ctx = crypto_tfm_ctx(req->base.tfm);
	ctx->base.exit_inv = true;
	rctx->needs_inv = true;

	spin_lock_bh(&priv->ring[ring].queue_lock);
	crypto_enqueue_request(&priv->ring[ring].queue, &req->base);
	spin_unlock_bh(&priv->ring[ring].queue_lock);

	queue_work(priv->ring[ring].workqueue,
		   &priv->ring[ring].work_data.work);

	err = crypto_wait_req(-EINPROGRESS, &result);

	if (err) {
		dev_warn(priv->dev, "hash: completion error (%d)\n", err);
		return err;
	}

	return 0;
}

/* safexcel_ahash_cache: cache data until at least one request can be sent to
 * the engine, aka. when there is at least 1 block size in the pipe.
 */
unsafe fn i32 safexcel_ahash_cache(*mut ahash_requestareq)
{
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);
	u64 cache_len;

	/* cache_len: everything accepted by the driver but not sent yet,
	 * tot sz handled by update() - last req sz - tot sz handled by send()
	 */
	cache_len = safexcel_queued_len(req);

	/*
	 * In case there isn't enough bytes to proceed (less than a
	 * block size), cache the data until we have enough.
	 */
	if (cache_len + areq->nbytes <= HASH_CACHE_SIZE) {
		sg_pcopy_to_buffer(areq->src, sg_nents(areq->src),
				   req->cache + cache_len,
				   areq->nbytes, 0);
		return 0;
	}

	/* We couldn't cache all the data */
	return -E2BIG;
}

unsafe fn i32 safexcel_ahash_enqueue(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);
	*mut safexcel_crypto_privpriv = ctx->base.priv;
	i32 ret, ring;

	req->needs_inv = false;

	if (ctx->base.ctxr) {
		if (priv->flags & EIP197_TRC_CACHE && !ctx->base.needs_inv &&
		     /* invalidate for *any* non-XCBC continuation */
		   ((req->not_first && !req->xcbcmac) ||
		     /* invalidate if (i)digest changed */
		     memcmp(ctx->base.ctxr->data, req->state, req->state_sz) ||
		     /* invalidate for HMAC finish with odigest changed */
		     (req->finish && req->hmac &&
		      memcmp(ctx->base.ctxr->data + (req->state_sz>>2),
			     &ctx->base.opad, req->state_sz))))
			/*
			 * We're still setting needs_inv here, even though it is
			 * cleared right away, because the needs_inv flag can be
			 * set in other functions and we want to keep the same
			 * logic.
			 */
			ctx->base.needs_inv = true;

		if (ctx->base.needs_inv) {
			ctx->base.needs_inv = false;
			req->needs_inv = true;
		}
	} else {
		ctx->base.ring = safexcel_select_ring(priv);
		ctx->base.ctxr = dma_pool_zalloc(priv->context_pool,
						 EIP197_GFP_FLAGS(areq->base),
						 &ctx->base.ctxr_dma);
		if (!ctx->base.ctxr)
			return -ENOMEM;
	}
	req->not_first = true;

	ring = ctx->base.ring;

	spin_lock_bh(&priv->ring[ring].queue_lock);
	ret = crypto_enqueue_request(&priv->ring[ring].queue, &areq->base);
	spin_unlock_bh(&priv->ring[ring].queue_lock);

	queue_work(priv->ring[ring].workqueue,
		   &priv->ring[ring].work_data.work);

	return ret;
}

unsafe fn i32 safexcel_ahash_update(*mut ahash_requestareq)
{
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);
	i32 ret;

	/* If the request is 0 length, do nothing */
	if (!areq->nbytes)
		return 0;

	/* Add request to the cache if it fits */
	ret = safexcel_ahash_cache(areq);

	/* Update total request length */
	req->len += areq->nbytes;

	/* If not all data could fit into the cache, go process the excess.
	 * Also go process immediately for an HMAC IV precompute, which
	 * will never be finished at all, but needs to be processed anyway.
	 */
	if ((ret && !req->finish) || req->last_req)
		return safexcel_ahash_enqueue(areq);

	return 0;
}

unsafe fn i32 safexcel_ahash_final(*mut ahash_requestareq)
{
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));

	req->finish = true;

	if ((!req->len && !areq->nbytes)) {
		/*
		 * If we have an overall 0 length *hash* request:
		 * The HW cannot do 0 length hash, so we provide the correct
		 * result directly here.
		 */
		if (ctx->alg == CONTEXT_CONTROL_CRYPTO_ALG_MD5)
			core::ptr::copy_nonoverlapping(areq->result, md5_zero_message_hash,
			       MD5_DIGEST_SIZE);
		else if (ctx->alg == CONTEXT_CONTROL_CRYPTO_ALG_SHA1)
			core::ptr::copy_nonoverlapping(areq->result, sha1_zero_message_hash,
			       SHA1_DIGEST_SIZE);
		else if (ctx->alg == CONTEXT_CONTROL_CRYPTO_ALG_SHA224)
			core::ptr::copy_nonoverlapping(areq->result, sha224_zero_message_hash,
			       SHA224_DIGEST_SIZE);
		else if (ctx->alg == CONTEXT_CONTROL_CRYPTO_ALG_SHA256)
			core::ptr::copy_nonoverlapping(areq->result, sha256_zero_message_hash,
			       SHA256_DIGEST_SIZE);
		else if (ctx->alg == CONTEXT_CONTROL_CRYPTO_ALG_SHA384)
			core::ptr::copy_nonoverlapping(areq->result, sha384_zero_message_hash,
			       SHA384_DIGEST_SIZE);
		else if (ctx->alg == CONTEXT_CONTROL_CRYPTO_ALG_SHA512)
			core::ptr::copy_nonoverlapping(areq->result, sha512_zero_message_hash,
			       SHA512_DIGEST_SIZE);
		else if (ctx->alg == CONTEXT_CONTROL_CRYPTO_ALG_SM3) {
			core::ptr::copy_nonoverlapping(areq->result,
			       EIP197_SM3_ZEROM_HASH, SM3_DIGEST_SIZE);
		}

		return 0;
	} else if (unlikely(req->digest == CONTEXT_CONTROL_DIGEST_XCM &&
			    ctx->alg == CONTEXT_CONTROL_CRYPTO_ALG_MD5 &&
			    req->len == core::mem::size_of::<u32>() && !areq->nbytes)) {
		/* Zero length CRC32 */
		core::ptr::copy_nonoverlapping(areq->result, &ctx->base.ipad, core::mem::size_of::<u32>());
		return 0;
	} else if (unlikely(ctx->cbcmac && req->len == AES_BLOCK_SIZE &&
			    !areq->nbytes)) {
		/* Zero length CBC MAC */
		core::ptr::write_bytes(areq->result, 0, AES_BLOCK_SIZE);
		return 0;
	} else if (unlikely(req->xcbcmac && req->len == AES_BLOCK_SIZE &&
			    !areq->nbytes)) {
		/* Zero length (X)CBC/CMAC */
		i32 i;

		for (i = 0; i < AES_BLOCK_SIZE / core::mem::size_of::<u32>(); i++) {
			u32 *result = (*mut core::ffi::c_void)areq->result;

			/* K3 */
			result[i] = swab32(ctx->base.ipad.word[i + 4]);
		}
		areq->result[0] ^= 0x80;			// 10- padding
		aes_encrypt(ctx->aes, areq->result, areq->result);
		return 0;
	} else if (unlikely(req->hmac &&
			    (req->len == req->block_sz) &&
			    !areq->nbytes)) {
		/*
		 * If we have an overall 0 length *HMAC* request:
		 * For HMAC, we need to finalize the inner digest
		 * and then perform the outer hash.
		 */

		/* generate pad block in the cache */
		/* start with a hash block of all zeroes */
		core::ptr::write_bytes(req->cache, 0, req->block_sz);
		/* set the first byte to 0x80 to 'append a 1 bit' */
		req->cache[0] = 0x80;
		/* add the length in bits in the last 2 bytes */
		if (req->len_is_le) {
			/* Little endian length word (e.g. MD5) */
			req->cache[req->block_sz-8] = (req->block_sz << 3) &
						      255;
			req->cache[req->block_sz-7] = (req->block_sz >> 5);
		} else {
			/* Big endian length word (e.g. any SHA) */
			req->cache[req->block_sz-2] = (req->block_sz >> 5);
			req->cache[req->block_sz-1] = (req->block_sz << 3) &
						      255;
		}

		req->len += req->block_sz; /* plus 1 hash block */

		/* Set special zero-length HMAC flag */
		req->hmac_zlen = true;

		/* Finalize HMAC */
		req->digest = CONTEXT_CONTROL_DIGEST_HMAC;
	} else if (req->hmac) {
		/* Finalize HMAC */
		req->digest = CONTEXT_CONTROL_DIGEST_HMAC;
	}

	return safexcel_ahash_enqueue(areq);
}

unsafe fn i32 safexcel_ahash_finup(*mut ahash_requestareq)
{
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	req->finish = true;

	safexcel_ahash_update(areq);
	return safexcel_ahash_final(areq);
}

unsafe fn i32 safexcel_ahash_export(*mut ahash_requestareq, *mut core::ffi::c_voidout)
{
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);
	*mut safexcel_ahash_export_stateexport = out;

	export->len = req->len;
	export->processed = req->processed;

	export->digest = req->digest;

	core::ptr::copy_nonoverlapping(export->state, req->state, req->state_sz);
	core::ptr::copy_nonoverlapping(export->cache, req->cache, HASH_CACHE_SIZE);

	return 0;
}

unsafe fn i32 safexcel_ahash_import(*mut ahash_requestareq, const *mut core::ffi::c_voidin)
{
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);
	const *mut safexcel_ahash_export_stateexport = in;
	i32 ret;

	ret = crypto_ahash_init(areq);
	if (ret)
		return ret;

	req->len = export->len;
	req->processed = export->processed;

	req->digest = export->digest;

	core::ptr::copy_nonoverlapping(req->cache, export->cache, HASH_CACHE_SIZE);
	core::ptr::copy_nonoverlapping(req->state, export->state, req->state_sz);

	return 0;
}

unsafe fn i32 safexcel_ahash_cra_init(*mut crypto_tfmtfm)
{
	*mut safexcel_ahash_ctxctx = crypto_tfm_ctx(tfm);
	*mut safexcel_alg_templatetmpl =
		container_of(__crypto_ahash_alg(tfm->__crt_alg),
			     struct safexcel_alg_template, alg.ahash);

	ctx->base.priv = tmpl->priv;
	ctx->base.send = safexcel_ahash_send;
	ctx->base.handle_result = safexcel_handle_result;
	ctx->fb_do_setkey = false;

	crypto_ahash_set_reqsize_dma(__crypto_ahash_cast(tfm),
				     core::mem::size_of::<struct safexcel_ahash_req>());
	return 0;
}

unsafe fn i32 safexcel_sha1_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA1;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = SHA1_DIGEST_SIZE;
	req->digest_sz = SHA1_DIGEST_SIZE;
	req->block_sz = SHA1_BLOCK_SIZE;

	return 0;
}

unsafe fn i32 safexcel_sha1_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_sha1_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

unsafe fn void safexcel_ahash_cra_exit(*mut crypto_tfmtfm)
{
	*mut safexcel_ahash_ctxctx = crypto_tfm_ctx(tfm);
	*mut safexcel_crypto_privpriv = ctx->base.priv;
	i32 ret;

	/* context not allocated, skip invalidation */
	if (!ctx->base.ctxr)
		return;

	if (priv->flags & EIP197_TRC_CACHE) {
		ret = safexcel_ahash_exit_inv(tfm);
		if (ret)
			dev_warn(priv->dev, "hash: invalidation error %d\n", ret);
	} else {
		dma_pool_free(priv->context_pool, ctx->base.ctxr,
			      ctx->base.ctxr_dma);
	}
}

struct safexcel_alg_template safexcel_alg_sha1 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA1,
	.alg.ahash = {
		.init = safexcel_sha1_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_sha1_digest,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = SHA1_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "sha1",
				.cra_driver_name = "safexcel-sha1",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = SHA1_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_hmac_sha1_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	/* Start from ipad precompute */
	core::ptr::copy_nonoverlapping(req->state, &ctx->base.ipad, SHA1_DIGEST_SIZE);
	/* Already processed the key^ipad part now! */
	req->len	= SHA1_BLOCK_SIZE;
	req->processed	= SHA1_BLOCK_SIZE;

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA1;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = SHA1_DIGEST_SIZE;
	req->digest_sz = SHA1_DIGEST_SIZE;
	req->block_sz = SHA1_BLOCK_SIZE;
	req->hmac = true;

	return 0;
}

unsafe fn i32 safexcel_hmac_sha1_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_hmac_sha1_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

unsafe fn i32 safexcel_hmac_init_pad(*mut ahash_requestareq,
				  u32 i32 blocksize, *const u8key,
				  u32 i32 keylen, *mut u8ipad, *mut u8opad)
{
	DECLARE_CRYPTO_WAIT(result);
	struct scatterlist sg;
	i32 ret, i;
	*mut u8keydup;

	if (keylen <= blocksize) {
		core::ptr::copy_nonoverlapping(ipad, key, keylen);
	} else {
		keydup = kmemdup(key, keylen, 0);
		if (!keydup)
			return -ENOMEM;

		ahash_request_set_callback(areq, CRYPTO_TFM_REQ_MAY_BACKLOG,
					   crypto_req_done, &result);
		sg_init_one(&sg, keydup, keylen);
		ahash_request_set_crypt(areq, &sg, ipad, keylen);

		ret = crypto_ahash_digest(areq);
		ret = crypto_wait_req(ret, &result);

		/* Avoid leaking */
		kfree_sensitive(keydup);

		if (ret)
			return ret;

		keylen = crypto_ahash_digestsize(crypto_ahash_reqtfm(areq));
	}

	core::ptr::write_bytes(ipad + keylen, 0, blocksize - keylen);
	core::ptr::copy_nonoverlapping(opad, ipad, blocksize);

	for (i = 0; i < blocksize; i++) {
		ipad[i] ^= HMAC_IPAD_VALUE;
		opad[i] ^= HMAC_OPAD_VALUE;
	}

	return 0;
}

unsafe fn i32 safexcel_hmac_init_iv(*mut ahash_requestareq,
				 u32 i32 blocksize, *mut u8pad, *mut core::ffi::c_voidstate)
{
	*mut safexcel_ahash_reqreq;
	DECLARE_CRYPTO_WAIT(result);
	struct scatterlist sg;
	i32 ret;

	ahash_request_set_callback(areq, CRYPTO_TFM_REQ_MAY_BACKLOG,
				   crypto_req_done, &result);
	sg_init_one(&sg, pad, blocksize);
	ahash_request_set_crypt(areq, &sg, pad, blocksize);

	ret = crypto_ahash_init(areq);
	if (ret)
		return ret;

	req = ahash_request_ctx_dma(areq);
	req->hmac = true;
	req->last_req = true;

	ret = crypto_ahash_update(areq);
	ret = crypto_wait_req(ret, &result);

	return ret ?: crypto_ahash_export(areq, state);
}

unsafe fn i32 __safexcel_hmac_setkey(const char *alg, *const u8key,
				  u32 i32 keylen,
				  *mut core::ffi::c_voidistate, *mut core::ffi::c_voidostate)
{
	*mut ahash_requestareq;
	*mut crypto_ahashtfm;
	u32 i32 blocksize;
	*mut u8ipad, *opad;
	i32 ret;

	tfm = crypto_alloc_ahash(alg, 0, 0);
	if (IS_ERR(tfm))
		return PTR_ERR(tfm);

	areq = ahash_request_alloc(tfm, 0);
	if (!areq) {
		ret = -ENOMEM;
		goto free_ahash;
	}

	crypto_ahash_clear_flags(tfm, ~0);
	blocksize = crypto_tfm_alg_blocksize(crypto_ahash_tfm(tfm));

	ipad = kcalloc(2, blocksize, 0);
	if (!ipad) {
		ret = -ENOMEM;
		goto free_request;
	}

	opad = ipad + blocksize;

	ret = safexcel_hmac_init_pad(areq, blocksize, key, keylen, ipad, opad);
	if (ret)
		goto free_ipad;

	ret = safexcel_hmac_init_iv(areq, blocksize, ipad, istate);
	if (ret)
		goto free_ipad;

	ret = safexcel_hmac_init_iv(areq, blocksize, opad, ostate);

free_ipad:
	kfree(ipad);
free_request:
	ahash_request_free(areq);
free_ahash:
	crypto_free_ahash(tfm);

	return ret;
}

pub unsafe fn safexcel_hmac_setkey(*mut safexcel_contextbase, *const u8key,
			 u32 i32 keylen, const char *alg,
			 u32 i32 state_sz)
{
	*mut safexcel_crypto_privpriv = base->priv;
	struct safexcel_ahash_export_state istate, ostate;
	i32 ret;

	ret = __safexcel_hmac_setkey(alg, key, keylen, &istate, &ostate);
	if (ret)
		return ret;

	if (priv->flags & EIP197_TRC_CACHE && base->ctxr &&
	    (memcmp(&base->ipad, istate.state, state_sz) ||
	     memcmp(&base->opad, ostate.state, state_sz)))
		base->needs_inv = true;

	core::ptr::copy_nonoverlapping(&base->ipad, &istate.state, state_sz);
	core::ptr::copy_nonoverlapping(&base->opad, &ostate.state, state_sz);

	return 0;
}

unsafe fn i32 safexcel_hmac_alg_setkey(*mut crypto_ahashtfm, *const u8key,
				    u32 i32 keylen, const char *alg,
				    u32 i32 state_sz)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);

	return safexcel_hmac_setkey(&ctx->base, key, keylen, alg, state_sz);
}

unsafe fn i32 safexcel_hmac_sha1_setkey(*mut crypto_ahashtfm, *const u8key,
				     u32 i32 keylen)
{
	return safexcel_hmac_alg_setkey(tfm, key, keylen, "safexcel-sha1",
					SHA1_DIGEST_SIZE);
}

struct safexcel_alg_template safexcel_alg_hmac_sha1 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA1,
	.alg.ahash = {
		.init = safexcel_hmac_sha1_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_hmac_sha1_digest,
		.setkey = safexcel_hmac_sha1_setkey,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = SHA1_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "hmac(sha1)",
				.cra_driver_name = "safexcel-hmac-sha1",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = SHA1_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_sha256_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA256;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = SHA256_DIGEST_SIZE;
	req->digest_sz = SHA256_DIGEST_SIZE;
	req->block_sz = SHA256_BLOCK_SIZE;

	return 0;
}

unsafe fn i32 safexcel_sha256_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_sha256_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_sha256 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA2_256,
	.alg.ahash = {
		.init = safexcel_sha256_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_sha256_digest,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = SHA256_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "sha256",
				.cra_driver_name = "safexcel-sha256",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = SHA256_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_sha224_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA224;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = SHA256_DIGEST_SIZE;
	req->digest_sz = SHA256_DIGEST_SIZE;
	req->block_sz = SHA256_BLOCK_SIZE;

	return 0;
}

unsafe fn i32 safexcel_sha224_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_sha224_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_sha224 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA2_256,
	.alg.ahash = {
		.init = safexcel_sha224_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_sha224_digest,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = SHA224_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "sha224",
				.cra_driver_name = "safexcel-sha224",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = SHA224_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_hmac_sha224_setkey(*mut crypto_ahashtfm, *const u8key,
				       u32 i32 keylen)
{
	return safexcel_hmac_alg_setkey(tfm, key, keylen, "safexcel-sha224",
					SHA256_DIGEST_SIZE);
}

unsafe fn i32 safexcel_hmac_sha224_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	/* Start from ipad precompute */
	core::ptr::copy_nonoverlapping(req->state, &ctx->base.ipad, SHA256_DIGEST_SIZE);
	/* Already processed the key^ipad part now! */
	req->len	= SHA256_BLOCK_SIZE;
	req->processed	= SHA256_BLOCK_SIZE;

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA224;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = SHA256_DIGEST_SIZE;
	req->digest_sz = SHA256_DIGEST_SIZE;
	req->block_sz = SHA256_BLOCK_SIZE;
	req->hmac = true;

	return 0;
}

unsafe fn i32 safexcel_hmac_sha224_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_hmac_sha224_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_hmac_sha224 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA2_256,
	.alg.ahash = {
		.init = safexcel_hmac_sha224_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_hmac_sha224_digest,
		.setkey = safexcel_hmac_sha224_setkey,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = SHA224_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "hmac(sha224)",
				.cra_driver_name = "safexcel-hmac-sha224",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = SHA224_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_hmac_sha256_setkey(*mut crypto_ahashtfm, *const u8key,
				     u32 i32 keylen)
{
	return safexcel_hmac_alg_setkey(tfm, key, keylen, "safexcel-sha256",
					SHA256_DIGEST_SIZE);
}

unsafe fn i32 safexcel_hmac_sha256_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	/* Start from ipad precompute */
	core::ptr::copy_nonoverlapping(req->state, &ctx->base.ipad, SHA256_DIGEST_SIZE);
	/* Already processed the key^ipad part now! */
	req->len	= SHA256_BLOCK_SIZE;
	req->processed	= SHA256_BLOCK_SIZE;

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA256;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = SHA256_DIGEST_SIZE;
	req->digest_sz = SHA256_DIGEST_SIZE;
	req->block_sz = SHA256_BLOCK_SIZE;
	req->hmac = true;

	return 0;
}

unsafe fn i32 safexcel_hmac_sha256_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_hmac_sha256_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_hmac_sha256 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA2_256,
	.alg.ahash = {
		.init = safexcel_hmac_sha256_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_hmac_sha256_digest,
		.setkey = safexcel_hmac_sha256_setkey,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = SHA256_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "hmac(sha256)",
				.cra_driver_name = "safexcel-hmac-sha256",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = SHA256_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_sha512_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA512;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = SHA512_DIGEST_SIZE;
	req->digest_sz = SHA512_DIGEST_SIZE;
	req->block_sz = SHA512_BLOCK_SIZE;

	return 0;
}

unsafe fn i32 safexcel_sha512_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_sha512_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_sha512 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA2_512,
	.alg.ahash = {
		.init = safexcel_sha512_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_sha512_digest,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = SHA512_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "sha512",
				.cra_driver_name = "safexcel-sha512",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = SHA512_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_sha384_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA384;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = SHA512_DIGEST_SIZE;
	req->digest_sz = SHA512_DIGEST_SIZE;
	req->block_sz = SHA512_BLOCK_SIZE;

	return 0;
}

unsafe fn i32 safexcel_sha384_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_sha384_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_sha384 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA2_512,
	.alg.ahash = {
		.init = safexcel_sha384_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_sha384_digest,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = SHA384_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "sha384",
				.cra_driver_name = "safexcel-sha384",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = SHA384_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_hmac_sha512_setkey(*mut crypto_ahashtfm, *const u8key,
				       u32 i32 keylen)
{
	return safexcel_hmac_alg_setkey(tfm, key, keylen, "safexcel-sha512",
					SHA512_DIGEST_SIZE);
}

unsafe fn i32 safexcel_hmac_sha512_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	/* Start from ipad precompute */
	core::ptr::copy_nonoverlapping(req->state, &ctx->base.ipad, SHA512_DIGEST_SIZE);
	/* Already processed the key^ipad part now! */
	req->len	= SHA512_BLOCK_SIZE;
	req->processed	= SHA512_BLOCK_SIZE;

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA512;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = SHA512_DIGEST_SIZE;
	req->digest_sz = SHA512_DIGEST_SIZE;
	req->block_sz = SHA512_BLOCK_SIZE;
	req->hmac = true;

	return 0;
}

unsafe fn i32 safexcel_hmac_sha512_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_hmac_sha512_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_hmac_sha512 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA2_512,
	.alg.ahash = {
		.init = safexcel_hmac_sha512_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_hmac_sha512_digest,
		.setkey = safexcel_hmac_sha512_setkey,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = SHA512_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "hmac(sha512)",
				.cra_driver_name = "safexcel-hmac-sha512",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = SHA512_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_hmac_sha384_setkey(*mut crypto_ahashtfm, *const u8key,
				       u32 i32 keylen)
{
	return safexcel_hmac_alg_setkey(tfm, key, keylen, "safexcel-sha384",
					SHA512_DIGEST_SIZE);
}

unsafe fn i32 safexcel_hmac_sha384_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	/* Start from ipad precompute */
	core::ptr::copy_nonoverlapping(req->state, &ctx->base.ipad, SHA512_DIGEST_SIZE);
	/* Already processed the key^ipad part now! */
	req->len	= SHA512_BLOCK_SIZE;
	req->processed	= SHA512_BLOCK_SIZE;

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA384;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = SHA512_DIGEST_SIZE;
	req->digest_sz = SHA512_DIGEST_SIZE;
	req->block_sz = SHA512_BLOCK_SIZE;
	req->hmac = true;

	return 0;
}

unsafe fn i32 safexcel_hmac_sha384_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_hmac_sha384_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_hmac_sha384 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA2_512,
	.alg.ahash = {
		.init = safexcel_hmac_sha384_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_hmac_sha384_digest,
		.setkey = safexcel_hmac_sha384_setkey,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = SHA384_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "hmac(sha384)",
				.cra_driver_name = "safexcel-hmac-sha384",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = SHA384_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_md5_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_MD5;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = MD5_DIGEST_SIZE;
	req->digest_sz = MD5_DIGEST_SIZE;
	req->block_sz = MD5_HMAC_BLOCK_SIZE;

	return 0;
}

unsafe fn i32 safexcel_md5_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_md5_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_md5 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_MD5,
	.alg.ahash = {
		.init = safexcel_md5_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_md5_digest,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = MD5_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "md5",
				.cra_driver_name = "safexcel-md5",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = MD5_HMAC_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_hmac_md5_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	/* Start from ipad precompute */
	core::ptr::copy_nonoverlapping(req->state, &ctx->base.ipad, MD5_DIGEST_SIZE);
	/* Already processed the key^ipad part now! */
	req->len	= MD5_HMAC_BLOCK_SIZE;
	req->processed	= MD5_HMAC_BLOCK_SIZE;

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_MD5;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = MD5_DIGEST_SIZE;
	req->digest_sz = MD5_DIGEST_SIZE;
	req->block_sz = MD5_HMAC_BLOCK_SIZE;
	req->len_is_le = true; /* MD5 is little endian! ... */
	req->hmac = true;

	return 0;
}

unsafe fn i32 safexcel_hmac_md5_setkey(*mut crypto_ahashtfm, *const u8key,
				     u32 i32 keylen)
{
	return safexcel_hmac_alg_setkey(tfm, key, keylen, "safexcel-md5",
					MD5_DIGEST_SIZE);
}

unsafe fn i32 safexcel_hmac_md5_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_hmac_md5_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_hmac_md5 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_MD5,
	.alg.ahash = {
		.init = safexcel_hmac_md5_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_hmac_md5_digest,
		.setkey = safexcel_hmac_md5_setkey,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = MD5_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "hmac(md5)",
				.cra_driver_name = "safexcel-hmac-md5",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = MD5_HMAC_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_cbcmac_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	/* Start from loaded keys */
	core::ptr::copy_nonoverlapping(req->state, &ctx->base.ipad, ctx->key_sz);
	/* Set processed to non-zero to enable invalidation detection */
	req->len	= AES_BLOCK_SIZE;
	req->processed	= AES_BLOCK_SIZE;

	req->digest   = CONTEXT_CONTROL_DIGEST_XCM;
	req->state_sz = ctx->key_sz;
	req->digest_sz = AES_BLOCK_SIZE;
	req->block_sz = AES_BLOCK_SIZE;
	req->xcbcmac  = true;

	return 0;
}

unsafe fn i32 safexcel_cbcmac_setkey(*mut crypto_ahashtfm, *const u8key,
				 u32 i32 len)
{
	*mut safexcel_ahash_ctxctx = crypto_tfm_ctx(crypto_ahash_tfm(tfm));
	struct crypto_aes_ctx aes;
	i32 ret, i;

	ret = aes_expandkey(&aes, key, len);
	if (ret)
		return ret;

	core::ptr::write_bytes(&ctx->base.ipad, 0, 2 * AES_BLOCK_SIZE);
	for (i = 0; i < len / core::mem::size_of::<u32>(); i++)
		ctx->base.ipad.be[i + 8] = cpu_to_be32(aes.key_enc[i]);

	if (len == AES_KEYSIZE_192) {
		ctx->alg    = CONTEXT_CONTROL_CRYPTO_ALG_XCBC192;
		ctx->key_sz = AES_MAX_KEY_SIZE + 2 * AES_BLOCK_SIZE;
	} else if (len == AES_KEYSIZE_256) {
		ctx->alg    = CONTEXT_CONTROL_CRYPTO_ALG_XCBC256;
		ctx->key_sz = AES_MAX_KEY_SIZE + 2 * AES_BLOCK_SIZE;
	} else {
		ctx->alg    = CONTEXT_CONTROL_CRYPTO_ALG_XCBC128;
		ctx->key_sz = AES_MIN_KEY_SIZE + 2 * AES_BLOCK_SIZE;
	}
	ctx->cbcmac  = true;

	memzero_explicit(&aes, core::mem::size_of::<aes>());
	return 0;
}

unsafe fn i32 safexcel_cbcmac_digest(*mut ahash_requestareq)
{
	return safexcel_cbcmac_init(areq) ?: safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_cbcmac = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = 0,
	.alg.ahash = {
		.init = safexcel_cbcmac_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_cbcmac_digest,
		.setkey = safexcel_cbcmac_setkey,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = AES_BLOCK_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "cbcmac(aes)",
				.cra_driver_name = "safexcel-cbcmac-aes",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = AES_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_xcbcmac_setkey(*mut crypto_ahashtfm, *const u8key,
				 u32 i32 len)
{
	*mut safexcel_ahash_ctxctx = crypto_tfm_ctx(crypto_ahash_tfm(tfm));
	u32 key_tmp[3 * AES_BLOCK_SIZE / core::mem::size_of::<u32>()];
	i32 ret, i;

	ret = aes_prepareenckey(ctx->aes, key, len);
	if (ret)
		return ret;

	/* precompute the XCBC key material */
	aes_encrypt(ctx->aes, (*mut u8)key_tmp + 2 * AES_BLOCK_SIZE,
		    "\x1\x1\x1\x1\x1\x1\x1\x1\x1\x1\x1\x1\x1\x1\x1\x1");
	aes_encrypt(ctx->aes, (*mut u8)key_tmp,
		    "\x2\x2\x2\x2\x2\x2\x2\x2\x2\x2\x2\x2\x2\x2\x2\x2");
	aes_encrypt(ctx->aes, (*mut u8)key_tmp + AES_BLOCK_SIZE,
		    "\x3\x3\x3\x3\x3\x3\x3\x3\x3\x3\x3\x3\x3\x3\x3\x3");
	for (i = 0; i < 3 * AES_BLOCK_SIZE / core::mem::size_of::<u32>(); i++)
		ctx->base.ipad.word[i] = swab32(key_tmp[i]);

	ret = aes_prepareenckey(ctx->aes,
				(*mut u8)key_tmp + 2 * AES_BLOCK_SIZE,
				AES_MIN_KEY_SIZE);
	if (ret)
		return ret;

	ctx->alg    = CONTEXT_CONTROL_CRYPTO_ALG_XCBC128;
	ctx->key_sz = AES_MIN_KEY_SIZE + 2 * AES_BLOCK_SIZE;
	ctx->cbcmac = false;

	return 0;
}

unsafe fn i32 safexcel_xcbcmac_cra_init(*mut crypto_tfmtfm)
{
	*mut safexcel_ahash_ctxctx = crypto_tfm_ctx(tfm);

	safexcel_ahash_cra_init(tfm);
	ctx->aes = kmalloc_obj(*ctx->aes);
	return ctx->aes == core::ptr::null_mut() ? -ENOMEM : 0;
}

unsafe fn void safexcel_xcbcmac_cra_exit(*mut crypto_tfmtfm)
{
	*mut safexcel_ahash_ctxctx = crypto_tfm_ctx(tfm);

	kfree(ctx->aes);
	safexcel_ahash_cra_exit(tfm);
}

struct safexcel_alg_template safexcel_alg_xcbcmac = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = 0,
	.alg.ahash = {
		.init = safexcel_cbcmac_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_cbcmac_digest,
		.setkey = safexcel_xcbcmac_setkey,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = AES_BLOCK_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "xcbc(aes)",
				.cra_driver_name = "safexcel-xcbc-aes",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = AES_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_xcbcmac_cra_init,
				.cra_exit = safexcel_xcbcmac_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_cmac_setkey(*mut crypto_ahashtfm, *const u8key,
				u32 i32 len)
{
	*mut safexcel_ahash_ctxctx = crypto_tfm_ctx(crypto_ahash_tfm(tfm));
	u64 consts[4];
	u64 _const[2];
	u8 msb_mask, gfmask;
	i32 ret, i;

	/* precompute the CMAC key material */
	ret = aes_prepareenckey(ctx->aes, key, len);
	if (ret)
		return ret;

	for (i = 0; i < len / core::mem::size_of::<u32>(); i++)
		ctx->base.ipad.word[i + 8] = get_unaligned_be32(&key[4 * i]);

	/* code below borrowed from crypto/cmac.c */
	/* encrypt the zero block */
	core::ptr::write_bytes(consts, 0, AES_BLOCK_SIZE);
	aes_encrypt(ctx->aes, (*mut u8)consts, (*mut u8)consts);

	gfmask = 0x87;
	_const[0] = be64_to_cpu(consts[1]);
	_const[1] = be64_to_cpu(consts[0]);

	/* gf(2^128) multiply zero-ciphertext with u and u^2 */
	for (i = 0; i < 4; i += 2) {
		msb_mask = ((s64)_const[1] >> 63) & gfmask;
		_const[1] = (_const[1] << 1) | (_const[0] >> 63);
		_const[0] = (_const[0] << 1) ^ msb_mask;

		consts[i + 0] = cpu_to_be64(_const[1]);
		consts[i + 1] = cpu_to_be64(_const[0]);
	}
	/* end of code borrowed from crypto/cmac.c */

	for (i = 0; i < 2 * AES_BLOCK_SIZE / core::mem::size_of::<u32>(); i++)
		ctx->base.ipad.be[i] = cpu_to_be32(((u32 *)consts)[i]);

	if (len == AES_KEYSIZE_192) {
		ctx->alg    = CONTEXT_CONTROL_CRYPTO_ALG_XCBC192;
		ctx->key_sz = AES_MAX_KEY_SIZE + 2 * AES_BLOCK_SIZE;
	} else if (len == AES_KEYSIZE_256) {
		ctx->alg    = CONTEXT_CONTROL_CRYPTO_ALG_XCBC256;
		ctx->key_sz = AES_MAX_KEY_SIZE + 2 * AES_BLOCK_SIZE;
	} else {
		ctx->alg    = CONTEXT_CONTROL_CRYPTO_ALG_XCBC128;
		ctx->key_sz = AES_MIN_KEY_SIZE + 2 * AES_BLOCK_SIZE;
	}
	ctx->cbcmac = false;

	return 0;
}

struct safexcel_alg_template safexcel_alg_cmac = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = 0,
	.alg.ahash = {
		.init = safexcel_cbcmac_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_cbcmac_digest,
		.setkey = safexcel_cmac_setkey,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = AES_BLOCK_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "cmac(aes)",
				.cra_driver_name = "safexcel-cmac-aes",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = AES_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_xcbcmac_cra_init,
				.cra_exit = safexcel_xcbcmac_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_sm3_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SM3;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = SM3_DIGEST_SIZE;
	req->digest_sz = SM3_DIGEST_SIZE;
	req->block_sz = SM3_BLOCK_SIZE;

	return 0;
}

unsafe fn i32 safexcel_sm3_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_sm3_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_sm3 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SM3,
	.alg.ahash = {
		.init = safexcel_sm3_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_sm3_digest,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = SM3_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "sm3",
				.cra_driver_name = "safexcel-sm3",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = SM3_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_hmac_sm3_setkey(*mut crypto_ahashtfm, *const u8key,
				    u32 i32 keylen)
{
	return safexcel_hmac_alg_setkey(tfm, key, keylen, "safexcel-sm3",
					SM3_DIGEST_SIZE);
}

unsafe fn i32 safexcel_hmac_sm3_init(*mut ahash_requestareq)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(crypto_ahash_reqtfm(areq));
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	/* Start from ipad precompute */
	core::ptr::copy_nonoverlapping(req->state, &ctx->base.ipad, SM3_DIGEST_SIZE);
	/* Already processed the key^ipad part now! */
	req->len	= SM3_BLOCK_SIZE;
	req->processed	= SM3_BLOCK_SIZE;

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SM3;
	req->digest = CONTEXT_CONTROL_DIGEST_PRECOMPUTED;
	req->state_sz = SM3_DIGEST_SIZE;
	req->digest_sz = SM3_DIGEST_SIZE;
	req->block_sz = SM3_BLOCK_SIZE;
	req->hmac = true;

	return 0;
}

unsafe fn i32 safexcel_hmac_sm3_digest(*mut ahash_requestareq)
{
	i32 ret = safexcel_hmac_sm3_init(areq);

	if (ret)
		return ret;

	return safexcel_ahash_finup(areq);
}

struct safexcel_alg_template safexcel_alg_hmac_sm3 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SM3,
	.alg.ahash = {
		.init = safexcel_hmac_sm3_init,
		.update = safexcel_ahash_update,
		.final = safexcel_ahash_final,
		.finup = safexcel_ahash_finup,
		.digest = safexcel_hmac_sm3_digest,
		.setkey = safexcel_hmac_sm3_setkey,
		.export = safexcel_ahash_export,
		.import = safexcel_ahash_import,
		.halg = {
			.digestsize = SM3_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "hmac(sm3)",
				.cra_driver_name = "safexcel-hmac-sm3",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_ALLOCATES_MEMORY |
					     CRYPTO_ALG_KERN_DRIVER_ONLY,
				.cra_blocksize = SM3_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_ahash_cra_init,
				.cra_exit = safexcel_ahash_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_sha3_224_init(*mut ahash_requestareq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(areq);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA3_224;
	req->digest = CONTEXT_CONTROL_DIGEST_INITIAL;
	req->state_sz = SHA3_224_DIGEST_SIZE;
	req->digest_sz = SHA3_224_DIGEST_SIZE;
	req->block_sz = SHA3_224_BLOCK_SIZE;
	ctx->do_fallback = false;
	ctx->fb_init_done = false;
	return 0;
}

unsafe fn i32 safexcel_sha3_fbcheck(*mut ahash_requestreq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(req);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut ahash_requestsubreq = ahash_request_ctx_dma(req);
	i32 ret = 0;

	if (ctx->do_fallback) {
		ahash_request_set_tfm(subreq, ctx->fback);
		ahash_request_set_callback(subreq, req->base.flags,
					   req->base.complete, req->base.data);
		ahash_request_set_crypt(subreq, req->src, req->result,
					req->nbytes);
		if (!ctx->fb_init_done) {
			if (ctx->fb_do_setkey) {
				/* Set fallback cipher HMAC key */
				u8 key[SHA3_224_BLOCK_SIZE];

				core::ptr::copy_nonoverlapping(key, &ctx->base.ipad,
				       crypto_ahash_blocksize(ctx->fback) / 2);
				core::ptr::copy_nonoverlapping(key +
				       crypto_ahash_blocksize(ctx->fback) / 2,
				       &ctx->base.opad,
				       crypto_ahash_blocksize(ctx->fback) / 2);
				ret = crypto_ahash_setkey(ctx->fback, key,
					crypto_ahash_blocksize(ctx->fback));
				memzero_explicit(key,
					crypto_ahash_blocksize(ctx->fback));
				ctx->fb_do_setkey = false;
			}
			ret = ret ?: crypto_ahash_init(subreq);
			ctx->fb_init_done = true;
		}
	}
	return ret;
}

unsafe fn i32 safexcel_sha3_update(*mut ahash_requestreq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(req);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut ahash_requestsubreq = ahash_request_ctx_dma(req);

	ctx->do_fallback = true;
	return safexcel_sha3_fbcheck(req) ?: crypto_ahash_update(subreq);
}

unsafe fn i32 safexcel_sha3_final(*mut ahash_requestreq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(req);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut ahash_requestsubreq = ahash_request_ctx_dma(req);

	ctx->do_fallback = true;
	return safexcel_sha3_fbcheck(req) ?: crypto_ahash_final(subreq);
}

unsafe fn i32 safexcel_sha3_finup(*mut ahash_requestreq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(req);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut ahash_requestsubreq = ahash_request_ctx_dma(req);

	ctx->do_fallback |= !req->nbytes;
	if (ctx->do_fallback)
		/* Update or ex/import happened or len 0, cannot use the HW */
		return safexcel_sha3_fbcheck(req) ?:
		       crypto_ahash_finup(subreq);
	else
		return safexcel_ahash_finup(req);
}

unsafe fn i32 safexcel_sha3_digest_fallback(*mut ahash_requestreq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(req);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut ahash_requestsubreq = ahash_request_ctx_dma(req);

	ctx->do_fallback = true;
	ctx->fb_init_done = false;
	return safexcel_sha3_fbcheck(req) ?: crypto_ahash_finup(subreq);
}

unsafe fn i32 safexcel_sha3_224_digest(*mut ahash_requestreq)
{
	if (req->nbytes)
		return safexcel_sha3_224_init(req) ?: safexcel_ahash_finup(req);

	/* HW cannot do zero length hash, use fallback instead */
	return safexcel_sha3_digest_fallback(req);
}

unsafe fn i32 safexcel_sha3_export(*mut ahash_requestreq, *mut core::ffi::c_voidout)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(req);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut ahash_requestsubreq = ahash_request_ctx_dma(req);

	ctx->do_fallback = true;
	return safexcel_sha3_fbcheck(req) ?: crypto_ahash_export(subreq, out);
}

unsafe fn i32 safexcel_sha3_import(*mut ahash_requestreq, const *mut core::ffi::c_voidin)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(req);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut ahash_requestsubreq = ahash_request_ctx_dma(req);

	ctx->do_fallback = true;
	return safexcel_sha3_fbcheck(req) ?: crypto_ahash_import(subreq, in);
	// return safexcel_ahash_import(req, in);
}

unsafe fn i32 safexcel_sha3_cra_init(*mut crypto_tfmtfm)
{
	*mut crypto_ahashahash = __crypto_ahash_cast(tfm);
	*mut safexcel_ahash_ctxctx = crypto_tfm_ctx(tfm);

	safexcel_ahash_cra_init(tfm);

	/* Allocate fallback implementation */
	ctx->fback = crypto_alloc_ahash(crypto_tfm_alg_name(tfm), 0,
					CRYPTO_ALG_ASYNC |
					CRYPTO_ALG_NEED_FALLBACK);
	if (IS_ERR(ctx->fback))
		return PTR_ERR(ctx->fback);

	/* Update statesize from fallback algorithm! */
	crypto_hash_alg_common(ahash)->statesize =
		crypto_ahash_statesize(ctx->fback);
	crypto_ahash_set_reqsize_dma(
		ahash, max(core::mem::size_of::<struct safexcel_ahash_req>(),
			   core::mem::size_of::<struct ahash_request>() +
			   crypto_ahash_reqsize(ctx->fback)));
	return 0;
}

unsafe fn void safexcel_sha3_cra_exit(*mut crypto_tfmtfm)
{
	*mut safexcel_ahash_ctxctx = crypto_tfm_ctx(tfm);

	crypto_free_ahash(ctx->fback);
	safexcel_ahash_cra_exit(tfm);
}

struct safexcel_alg_template safexcel_alg_sha3_224 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA3,
	.alg.ahash = {
		.init = safexcel_sha3_224_init,
		.update = safexcel_sha3_update,
		.final = safexcel_sha3_final,
		.finup = safexcel_sha3_finup,
		.digest = safexcel_sha3_224_digest,
		.export = safexcel_sha3_export,
		.import = safexcel_sha3_import,
		.halg = {
			.digestsize = SHA3_224_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "sha3-224",
				.cra_driver_name = "safexcel-sha3-224",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_KERN_DRIVER_ONLY |
					     CRYPTO_ALG_NEED_FALLBACK,
				.cra_blocksize = SHA3_224_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_sha3_cra_init,
				.cra_exit = safexcel_sha3_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_sha3_256_init(*mut ahash_requestareq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(areq);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA3_256;
	req->digest = CONTEXT_CONTROL_DIGEST_INITIAL;
	req->state_sz = SHA3_256_DIGEST_SIZE;
	req->digest_sz = SHA3_256_DIGEST_SIZE;
	req->block_sz = SHA3_256_BLOCK_SIZE;
	ctx->do_fallback = false;
	ctx->fb_init_done = false;
	return 0;
}

unsafe fn i32 safexcel_sha3_256_digest(*mut ahash_requestreq)
{
	if (req->nbytes)
		return safexcel_sha3_256_init(req) ?: safexcel_ahash_finup(req);

	/* HW cannot do zero length hash, use fallback instead */
	return safexcel_sha3_digest_fallback(req);
}

struct safexcel_alg_template safexcel_alg_sha3_256 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA3,
	.alg.ahash = {
		.init = safexcel_sha3_256_init,
		.update = safexcel_sha3_update,
		.final = safexcel_sha3_final,
		.finup = safexcel_sha3_finup,
		.digest = safexcel_sha3_256_digest,
		.export = safexcel_sha3_export,
		.import = safexcel_sha3_import,
		.halg = {
			.digestsize = SHA3_256_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "sha3-256",
				.cra_driver_name = "safexcel-sha3-256",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_KERN_DRIVER_ONLY |
					     CRYPTO_ALG_NEED_FALLBACK,
				.cra_blocksize = SHA3_256_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_sha3_cra_init,
				.cra_exit = safexcel_sha3_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_sha3_384_init(*mut ahash_requestareq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(areq);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA3_384;
	req->digest = CONTEXT_CONTROL_DIGEST_INITIAL;
	req->state_sz = SHA3_384_DIGEST_SIZE;
	req->digest_sz = SHA3_384_DIGEST_SIZE;
	req->block_sz = SHA3_384_BLOCK_SIZE;
	ctx->do_fallback = false;
	ctx->fb_init_done = false;
	return 0;
}

unsafe fn i32 safexcel_sha3_384_digest(*mut ahash_requestreq)
{
	if (req->nbytes)
		return safexcel_sha3_384_init(req) ?: safexcel_ahash_finup(req);

	/* HW cannot do zero length hash, use fallback instead */
	return safexcel_sha3_digest_fallback(req);
}

struct safexcel_alg_template safexcel_alg_sha3_384 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA3,
	.alg.ahash = {
		.init = safexcel_sha3_384_init,
		.update = safexcel_sha3_update,
		.final = safexcel_sha3_final,
		.finup = safexcel_sha3_finup,
		.digest = safexcel_sha3_384_digest,
		.export = safexcel_sha3_export,
		.import = safexcel_sha3_import,
		.halg = {
			.digestsize = SHA3_384_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "sha3-384",
				.cra_driver_name = "safexcel-sha3-384",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_KERN_DRIVER_ONLY |
					     CRYPTO_ALG_NEED_FALLBACK,
				.cra_blocksize = SHA3_384_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_sha3_cra_init,
				.cra_exit = safexcel_sha3_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_sha3_512_init(*mut ahash_requestareq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(areq);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA3_512;
	req->digest = CONTEXT_CONTROL_DIGEST_INITIAL;
	req->state_sz = SHA3_512_DIGEST_SIZE;
	req->digest_sz = SHA3_512_DIGEST_SIZE;
	req->block_sz = SHA3_512_BLOCK_SIZE;
	ctx->do_fallback = false;
	ctx->fb_init_done = false;
	return 0;
}

unsafe fn i32 safexcel_sha3_512_digest(*mut ahash_requestreq)
{
	if (req->nbytes)
		return safexcel_sha3_512_init(req) ?: safexcel_ahash_finup(req);

	/* HW cannot do zero length hash, use fallback instead */
	return safexcel_sha3_digest_fallback(req);
}

struct safexcel_alg_template safexcel_alg_sha3_512 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA3,
	.alg.ahash = {
		.init = safexcel_sha3_512_init,
		.update = safexcel_sha3_update,
		.final = safexcel_sha3_final,
		.finup = safexcel_sha3_finup,
		.digest = safexcel_sha3_512_digest,
		.export = safexcel_sha3_export,
		.import = safexcel_sha3_import,
		.halg = {
			.digestsize = SHA3_512_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "sha3-512",
				.cra_driver_name = "safexcel-sha3-512",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_KERN_DRIVER_ONLY |
					     CRYPTO_ALG_NEED_FALLBACK,
				.cra_blocksize = SHA3_512_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_sha3_cra_init,
				.cra_exit = safexcel_sha3_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_hmac_sha3_cra_init(*mut crypto_tfmtfm, const char *alg)
{
	*mut safexcel_ahash_ctxctx = crypto_tfm_ctx(tfm);
	i32 ret;

	ret = safexcel_sha3_cra_init(tfm);
	if (ret)
		return ret;

	/* Allocate precalc basic digest implementation */
	ctx->shpre = crypto_alloc_shash(alg, 0, CRYPTO_ALG_NEED_FALLBACK);
	if (IS_ERR(ctx->shpre))
		return PTR_ERR(ctx->shpre);

	ctx->shdesc = kmalloc(core::mem::size_of::<*ctx->shdesc>() +
			      crypto_shash_descsize(ctx->shpre), 0);
	if (!ctx->shdesc) {
		crypto_free_shash(ctx->shpre);
		return -ENOMEM;
	}
	ctx->shdesc->tfm = ctx->shpre;
	return 0;
}

unsafe fn void safexcel_hmac_sha3_cra_exit(*mut crypto_tfmtfm)
{
	*mut safexcel_ahash_ctxctx = crypto_tfm_ctx(tfm);

	crypto_free_ahash(ctx->fback);
	crypto_free_shash(ctx->shpre);
	kfree(ctx->shdesc);
	safexcel_ahash_cra_exit(tfm);
}

unsafe fn i32 safexcel_hmac_sha3_setkey(*mut crypto_ahashtfm, *const u8key,
				     u32 i32 keylen)
{
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	i32 ret = 0;

	if (keylen > crypto_ahash_blocksize(tfm)) {
		/*
		 * If the key is larger than the blocksize, then hash it
		 * first using our fallback cipher
		 */
		ret = crypto_shash_digest(ctx->shdesc, key, keylen,
					  ctx->base.ipad.byte);
		keylen = crypto_shash_digestsize(ctx->shpre);

		/*
		 * If the digest is larger than half the blocksize, we need to
		 * move the rest to opad due to the way our HMAC infra works.
		 */
		if (keylen > crypto_ahash_blocksize(tfm) / 2)
			/* Buffers overlap, need to use memmove iso memcpy! */
			memmove(&ctx->base.opad,
				ctx->base.ipad.byte +
					crypto_ahash_blocksize(tfm) / 2,
				keylen - crypto_ahash_blocksize(tfm) / 2);
	} else {
		/*
		 * Copy the key to our ipad & opad buffers
		 * Note that ipad and opad each contain one half of the key,
		 * to match the existing HMAC driver infrastructure.
		 */
		if (keylen <= crypto_ahash_blocksize(tfm) / 2) {
			core::ptr::copy_nonoverlapping(&ctx->base.ipad, key, keylen);
		} else {
			core::ptr::copy_nonoverlapping(&ctx->base.ipad, key,
			       crypto_ahash_blocksize(tfm) / 2);
			core::ptr::copy_nonoverlapping(&ctx->base.opad,
			       key + crypto_ahash_blocksize(tfm) / 2,
			       keylen - crypto_ahash_blocksize(tfm) / 2);
		}
	}

	/* Pad key with zeroes */
	if (keylen <= crypto_ahash_blocksize(tfm) / 2) {
		core::ptr::write_bytes(ctx->base.ipad.byte + keylen, 0,
		       crypto_ahash_blocksize(tfm) / 2 - keylen);
		core::ptr::write_bytes(&ctx->base.opad, 0, crypto_ahash_blocksize(tfm) / 2);
	} else {
		core::ptr::write_bytes(ctx->base.opad.byte + keylen -
		       crypto_ahash_blocksize(tfm) / 2, 0,
		       crypto_ahash_blocksize(tfm) - keylen);
	}

	/* If doing fallback, still need to set the new key! */
	ctx->fb_do_setkey = true;
	return ret;
}

unsafe fn i32 safexcel_hmac_sha3_224_init(*mut ahash_requestareq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(areq);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	/* Copy (half of) the key */
	core::ptr::copy_nonoverlapping(req->state, &ctx->base.ipad, SHA3_224_BLOCK_SIZE / 2);
	/* Start of HMAC should have len == processed == blocksize */
	req->len	= SHA3_224_BLOCK_SIZE;
	req->processed	= SHA3_224_BLOCK_SIZE;
	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA3_224;
	req->digest = CONTEXT_CONTROL_DIGEST_HMAC;
	req->state_sz = SHA3_224_BLOCK_SIZE / 2;
	req->digest_sz = SHA3_224_DIGEST_SIZE;
	req->block_sz = SHA3_224_BLOCK_SIZE;
	req->hmac = true;
	ctx->do_fallback = false;
	ctx->fb_init_done = false;
	return 0;
}

unsafe fn i32 safexcel_hmac_sha3_224_digest(*mut ahash_requestreq)
{
	if (req->nbytes)
		return safexcel_hmac_sha3_224_init(req) ?:
		       safexcel_ahash_finup(req);

	/* HW cannot do zero length HMAC, use fallback instead */
	return safexcel_sha3_digest_fallback(req);
}

unsafe fn i32 safexcel_hmac_sha3_224_cra_init(*mut crypto_tfmtfm)
{
	return safexcel_hmac_sha3_cra_init(tfm, "sha3-224");
}

struct safexcel_alg_template safexcel_alg_hmac_sha3_224 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA3,
	.alg.ahash = {
		.init = safexcel_hmac_sha3_224_init,
		.update = safexcel_sha3_update,
		.final = safexcel_sha3_final,
		.finup = safexcel_sha3_finup,
		.digest = safexcel_hmac_sha3_224_digest,
		.setkey = safexcel_hmac_sha3_setkey,
		.export = safexcel_sha3_export,
		.import = safexcel_sha3_import,
		.halg = {
			.digestsize = SHA3_224_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "hmac(sha3-224)",
				.cra_driver_name = "safexcel-hmac-sha3-224",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_KERN_DRIVER_ONLY |
					     CRYPTO_ALG_NEED_FALLBACK,
				.cra_blocksize = SHA3_224_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_hmac_sha3_224_cra_init,
				.cra_exit = safexcel_hmac_sha3_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_hmac_sha3_256_init(*mut ahash_requestareq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(areq);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	/* Copy (half of) the key */
	core::ptr::copy_nonoverlapping(req->state, &ctx->base.ipad, SHA3_256_BLOCK_SIZE / 2);
	/* Start of HMAC should have len == processed == blocksize */
	req->len	= SHA3_256_BLOCK_SIZE;
	req->processed	= SHA3_256_BLOCK_SIZE;
	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA3_256;
	req->digest = CONTEXT_CONTROL_DIGEST_HMAC;
	req->state_sz = SHA3_256_BLOCK_SIZE / 2;
	req->digest_sz = SHA3_256_DIGEST_SIZE;
	req->block_sz = SHA3_256_BLOCK_SIZE;
	req->hmac = true;
	ctx->do_fallback = false;
	ctx->fb_init_done = false;
	return 0;
}

unsafe fn i32 safexcel_hmac_sha3_256_digest(*mut ahash_requestreq)
{
	if (req->nbytes)
		return safexcel_hmac_sha3_256_init(req) ?:
		       safexcel_ahash_finup(req);

	/* HW cannot do zero length HMAC, use fallback instead */
	return safexcel_sha3_digest_fallback(req);
}

unsafe fn i32 safexcel_hmac_sha3_256_cra_init(*mut crypto_tfmtfm)
{
	return safexcel_hmac_sha3_cra_init(tfm, "sha3-256");
}

struct safexcel_alg_template safexcel_alg_hmac_sha3_256 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA3,
	.alg.ahash = {
		.init = safexcel_hmac_sha3_256_init,
		.update = safexcel_sha3_update,
		.final = safexcel_sha3_final,
		.finup = safexcel_sha3_finup,
		.digest = safexcel_hmac_sha3_256_digest,
		.setkey = safexcel_hmac_sha3_setkey,
		.export = safexcel_sha3_export,
		.import = safexcel_sha3_import,
		.halg = {
			.digestsize = SHA3_256_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "hmac(sha3-256)",
				.cra_driver_name = "safexcel-hmac-sha3-256",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_KERN_DRIVER_ONLY |
					     CRYPTO_ALG_NEED_FALLBACK,
				.cra_blocksize = SHA3_256_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_hmac_sha3_256_cra_init,
				.cra_exit = safexcel_hmac_sha3_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_hmac_sha3_384_init(*mut ahash_requestareq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(areq);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	/* Copy (half of) the key */
	core::ptr::copy_nonoverlapping(req->state, &ctx->base.ipad, SHA3_384_BLOCK_SIZE / 2);
	/* Start of HMAC should have len == processed == blocksize */
	req->len	= SHA3_384_BLOCK_SIZE;
	req->processed	= SHA3_384_BLOCK_SIZE;
	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA3_384;
	req->digest = CONTEXT_CONTROL_DIGEST_HMAC;
	req->state_sz = SHA3_384_BLOCK_SIZE / 2;
	req->digest_sz = SHA3_384_DIGEST_SIZE;
	req->block_sz = SHA3_384_BLOCK_SIZE;
	req->hmac = true;
	ctx->do_fallback = false;
	ctx->fb_init_done = false;
	return 0;
}

unsafe fn i32 safexcel_hmac_sha3_384_digest(*mut ahash_requestreq)
{
	if (req->nbytes)
		return safexcel_hmac_sha3_384_init(req) ?:
		       safexcel_ahash_finup(req);

	/* HW cannot do zero length HMAC, use fallback instead */
	return safexcel_sha3_digest_fallback(req);
}

unsafe fn i32 safexcel_hmac_sha3_384_cra_init(*mut crypto_tfmtfm)
{
	return safexcel_hmac_sha3_cra_init(tfm, "sha3-384");
}

struct safexcel_alg_template safexcel_alg_hmac_sha3_384 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA3,
	.alg.ahash = {
		.init = safexcel_hmac_sha3_384_init,
		.update = safexcel_sha3_update,
		.final = safexcel_sha3_final,
		.finup = safexcel_sha3_finup,
		.digest = safexcel_hmac_sha3_384_digest,
		.setkey = safexcel_hmac_sha3_setkey,
		.export = safexcel_sha3_export,
		.import = safexcel_sha3_import,
		.halg = {
			.digestsize = SHA3_384_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "hmac(sha3-384)",
				.cra_driver_name = "safexcel-hmac-sha3-384",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_KERN_DRIVER_ONLY |
					     CRYPTO_ALG_NEED_FALLBACK,
				.cra_blocksize = SHA3_384_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_hmac_sha3_384_cra_init,
				.cra_exit = safexcel_hmac_sha3_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

unsafe fn i32 safexcel_hmac_sha3_512_init(*mut ahash_requestareq)
{
	*mut crypto_ahashtfm = crypto_ahash_reqtfm(areq);
	*mut safexcel_ahash_ctxctx = crypto_ahash_ctx(tfm);
	*mut safexcel_ahash_reqreq = ahash_request_ctx_dma(areq);

	core::ptr::write_bytes(req, 0, core::mem::size_of::<*req>());

	/* Copy (half of) the key */
	core::ptr::copy_nonoverlapping(req->state, &ctx->base.ipad, SHA3_512_BLOCK_SIZE / 2);
	/* Start of HMAC should have len == processed == blocksize */
	req->len	= SHA3_512_BLOCK_SIZE;
	req->processed	= SHA3_512_BLOCK_SIZE;
	ctx->alg = CONTEXT_CONTROL_CRYPTO_ALG_SHA3_512;
	req->digest = CONTEXT_CONTROL_DIGEST_HMAC;
	req->state_sz = SHA3_512_BLOCK_SIZE / 2;
	req->digest_sz = SHA3_512_DIGEST_SIZE;
	req->block_sz = SHA3_512_BLOCK_SIZE;
	req->hmac = true;
	ctx->do_fallback = false;
	ctx->fb_init_done = false;
	return 0;
}

unsafe fn i32 safexcel_hmac_sha3_512_digest(*mut ahash_requestreq)
{
	if (req->nbytes)
		return safexcel_hmac_sha3_512_init(req) ?:
		       safexcel_ahash_finup(req);

	/* HW cannot do zero length HMAC, use fallback instead */
	return safexcel_sha3_digest_fallback(req);
}

unsafe fn i32 safexcel_hmac_sha3_512_cra_init(*mut crypto_tfmtfm)
{
	return safexcel_hmac_sha3_cra_init(tfm, "sha3-512");
}
struct safexcel_alg_template safexcel_alg_hmac_sha3_512 = {
	.type = SAFEXCEL_ALG_TYPE_AHASH,
	.algo_mask = SAFEXCEL_ALG_SHA3,
	.alg.ahash = {
		.init = safexcel_hmac_sha3_512_init,
		.update = safexcel_sha3_update,
		.final = safexcel_sha3_final,
		.finup = safexcel_sha3_finup,
		.digest = safexcel_hmac_sha3_512_digest,
		.setkey = safexcel_hmac_sha3_setkey,
		.export = safexcel_sha3_export,
		.import = safexcel_sha3_import,
		.halg = {
			.digestsize = SHA3_512_DIGEST_SIZE,
			.statesize = core::mem::size_of::<struct safexcel_ahash_export_state>(),
			.base = {
				.cra_name = "hmac(sha3-512)",
				.cra_driver_name = "safexcel-hmac-sha3-512",
				.cra_priority = SAFEXCEL_CRA_PRIORITY,
				.cra_flags = CRYPTO_ALG_ASYNC |
					     CRYPTO_ALG_KERN_DRIVER_ONLY |
					     CRYPTO_ALG_NEED_FALLBACK,
				.cra_blocksize = SHA3_512_BLOCK_SIZE,
				.cra_ctxsize = core::mem::size_of::<struct safexcel_ahash_ctx>(),
				.cra_init = safexcel_hmac_sha3_512_cra_init,
				.cra_exit = safexcel_hmac_sha3_cra_exit,
				.cra_module = THIS_MODULE,
			},
		},
	},
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
