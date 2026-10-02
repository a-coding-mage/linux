// SPDX-License-Identifier: GPL-2.0
/* Copyright 2019 Google LLC */
/* Macro/inline and configured ABI glue only. No fallback crypto algorithm. */
#define pr_fmt(fmt) "blk-crypto-fallback: " fmt
#include <linux/module.h>
#include "blk-crypto-fallback-bindings.h"

/* The provider keeps the original built-in parameter namespace. */
#undef MODULE_PARAM_PREFIX
#define MODULE_PARAM_PREFIX "blk_crypto_fallback."
#undef __MODULE_INFO_PREFIX
#define __MODULE_INFO_PREFIX "blk_crypto_fallback."
unsigned int rust_bcf_num_prealloc_bounce_pg = BIO_MAX_VECS;
module_param_named(num_prealloc_bounce_pg, rust_bcf_num_prealloc_bounce_pg, uint, 0);
MODULE_PARM_DESC(num_prealloc_bounce_pg,
 "Number of preallocated bounce pages for the blk-crypto crypto API fallback");
unsigned int rust_bcf_num_keyslots = 100;
module_param_named(num_keyslots, rust_bcf_num_keyslots, uint, 0);
MODULE_PARM_DESC(num_keyslots,
 "Number of keyslots for the blk-crypto crypto API fallback");
unsigned int rust_bcf_num_prealloc_fallback_crypt_ctxs = 128;
module_param_named(num_prealloc_fallback_crypt_ctxs,
                   rust_bcf_num_prealloc_fallback_crypt_ctxs, uint, 0);
MODULE_PARM_DESC(num_prealloc_crypt_fallback_ctxs,
 "Number of preallocated bio fallback crypto contexts for blk-crypto to use during crypto API fallback");

static DEFINE_MUTEX(tfms_init_lock);
void rust_bcf_lock(void) { mutex_lock(&tfms_init_lock); }
void rust_bcf_unlock(void) { mutex_unlock(&tfms_init_lock); }
bool rust_bcf_load_acquire(const bool *p) { return smp_load_acquire(p); }
void rust_bcf_store_release(bool *p) { smp_store_release(p, true); }
void rust_bcf_warn_invalid_slot(bool value) { WARN_ON(value); }
void rust_bcf_warn_key_clear(int err) { WARN_ON(err); }
bool rust_bcf_warn_uninitialized(bool value) { return WARN_ON_ONCE(value); }
void rust_bcf_warn_missing_cipher(const char *cipher)
{
    pr_warn_once("Missing crypto API support for \"%s\"\n", cipher);
}
long rust_bcf_tfm_error(struct crypto_sync_skcipher *tfm)
{
    return IS_ERR(tfm) ? PTR_ERR(tfm) : 0;
}
int rust_bcf_setkey(struct crypto_sync_skcipher *tfm, const u8 *key, unsigned int len)
{
    return crypto_sync_skcipher_setkey(tfm, key, len);
}
void rust_bcf_forbid_weak_keys(struct crypto_sync_skcipher *tfm)
{
    crypto_sync_skcipher_set_flags(tfm, CRYPTO_TFM_REQ_FORBID_WEAK_KEYS);
}
void rust_bcf_free_tfm(struct crypto_sync_skcipher *tfm) { crypto_free_sync_skcipher(tfm); }
void rust_bcf_with_encrypt_request(struct crypto_sync_skcipher *tfm, struct bio *bio)
{
    SYNC_SKCIPHER_REQUEST_ON_STACK(req, tfm);
    rust_bcf_encrypt_with_request(req, bio);
}
blk_status_t rust_bcf_with_decrypt_request(struct crypto_sync_skcipher *tfm, void *data)
{
    SYNC_SKCIPHER_REQUEST_ON_STACK(req, tfm);
    return rust_bcf_decrypt_with_request(req, data);
}
void rust_bcf_request_setup(struct skcipher_request *req, struct scatterlist *src,
                           struct scatterlist *dst, unsigned int len, void *iv)
{
    skcipher_request_set_callback(req,
        CRYPTO_TFM_REQ_MAY_BACKLOG | CRYPTO_TFM_REQ_MAY_SLEEP, NULL, NULL);
    skcipher_request_set_crypt(req, src, dst, len, iv);
}
void rust_bcf_sg_set_page(struct scatterlist *sg, struct page *page,
                         unsigned int len, unsigned int offset)
{
    sg_set_page(sg, page, len, offset);
}
struct bio_vec *rust_bcf_first_bvec(struct bio *bio) { return bio_first_bvec_all(bio); }
struct bio_vec rust_bcf_iter_iovec(struct bio *bio, struct bvec_iter iter)
{
    return bio_iter_iovec(bio, iter);
}
void rust_bcf_advance_iter(struct bio *bio, struct bvec_iter *iter, unsigned int len)
{
    bio_advance_iter_single(bio, iter, len);
}
unsigned int rust_bcf_segments(struct bio *bio) { return bio_segments(bio); }
bool rust_bcf_remapped(struct bio *bio) { return bio_flagged(bio, BIO_REMAPPED); }
void rust_bcf_set_remapped(struct bio *bio) { bio_set_flag(bio, BIO_REMAPPED); }
void rust_bcf_clone_blkg(struct bio *dst, struct bio *src) { bio_clone_blkg_association(dst, src); }
void rust_bcf_inc_remaining(struct bio *bio) { bio_inc_remaining(bio); }
void rust_bcf_endio_status(struct bio *bio, blk_status_t status) { bio_endio_status(bio, status); }
bool rust_bcf_is_write(struct bio *bio) { return bio_data_dir(bio) == WRITE; }
void rust_bcf_free_crypt_ctx(struct bio *bio) { bio_crypt_free_ctx(bio); }
void rust_bcf_cmpxchg_status(struct bio *bio, blk_status_t status)
{
    cmpxchg(&bio->bi_status, 0, status);
}
unsigned int rust_bcf_noio_save(void) { return memalloc_noio_save(); }
void rust_bcf_noio_restore(unsigned int flags) { memalloc_noio_restore(flags); }
unsigned int rust_bcf_alloc_pages(unsigned int nr, struct page **pages)
{
    return alloc_pages_bulk(GFP_KERNEL, nr, pages);
}
void rust_bcf_alloc_pool_pages(mempool_t *pool, void **pages, unsigned int nr)
{
    mempool_alloc_bulk(pool, pages, nr);
}
void rust_bcf_release_pages(struct page **pages, unsigned int nr) { release_pages(pages, nr); }
void *rust_bcf_alloc_ctx(mempool_t *pool) { return mempool_alloc(pool, GFP_NOIO); }
struct blk_crypto_profile *rust_bcf_alloc_profile(void)
{
    return kzalloc_obj(struct blk_crypto_profile);
}
struct blk_crypto_fallback_keyslot *rust_bcf_alloc_slots(unsigned int nr)
{
    return kzalloc_objs(struct blk_crypto_fallback_keyslot, nr);
}
struct workqueue_struct *rust_bcf_alloc_workqueue(void)
{
    return alloc_workqueue("blk_crypto_wq", WQ_UNBOUND | WQ_HIGHPRI | WQ_MEM_RECLAIM,
                           num_online_cpus());
}
mempool_t *rust_bcf_create_page_pool(unsigned int nr) { return mempool_create_page_pool(nr, 0); }
struct kmem_cache *rust_bcf_create_ctx_cache(void) { return KMEM_CACHE(bio_fallback_crypt_ctx, 0); }
mempool_t *rust_bcf_create_ctx_pool(unsigned int nr, struct kmem_cache *cache)
{
    return mempool_create_slab_pool(nr, cache);
}
struct bio_fallback_crypt_ctx *rust_bcf_from_work(struct work_struct *work)
{
    return container_of(work, struct bio_fallback_crypt_ctx, work);
}
struct work_struct *rust_bcf_ctx_work(struct bio_fallback_crypt_ctx *ctx) { return &ctx->work; }
struct bio *rust_bcf_ctx_bio(struct bio_fallback_crypt_ctx *ctx) { return ctx->bio; }
void rust_bcf_ctx_set_bio(struct bio_fallback_crypt_ctx *ctx, struct bio *bio) { ctx->bio = bio; }
/* Anonymous-union member access only; Rust controls completion sequencing. */
void rust_bcf_save_completion(struct bio_fallback_crypt_ctx *ctx, struct bio *bio)
{
    ctx->bi_private_orig = bio->bi_private;
    ctx->bi_end_io_orig = bio->bi_end_io;
}
void rust_bcf_restore_completion(struct bio_fallback_crypt_ctx *ctx, struct bio *bio)
{
    bio->bi_private = ctx->bi_private_orig;
    bio->bi_end_io = ctx->bi_end_io_orig;
}
void rust_bcf_init_work(struct work_struct *work) { INIT_WORK(work, rust_bcf_decrypt_work); }
bool rust_bcf_queue_work(struct workqueue_struct *wq, struct work_struct *work)
{
    return queue_work(wq, work);
}
