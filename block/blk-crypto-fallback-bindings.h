/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2019 Google LLC */
/* The original C headers and copied private declarations own the ABI. */
#ifndef BLK_CRYPTO_FALLBACK_RUST_BINDINGS_H
#define BLK_CRYPTO_FALLBACK_RUST_BINDINGS_H
#include <crypto/skcipher.h>
#include <linux/blk-crypto.h>
#include <linux/blk-crypto-profile.h>
#include <linux/blkdev.h>
#include <linux/crypto.h>
#include <linux/mempool.h>
#include <linux/random.h>
#include <linux/scatterlist.h>
#include "blk-cgroup.h"
#include "blk-crypto-internal.h"

struct bio_fallback_crypt_ctx {
	struct bio_crypt_ctx crypt_ctx;
	/*
	 * Copy of the bvec_iter when this bio was submitted.
	 * We only want to en/decrypt the part of the bio as described by the
	 * bvec_iter upon submission because bio might be split before being
	 * resubmitted
	 */
	struct bvec_iter crypt_iter;
	union {
		struct {
			struct work_struct work;
			struct bio *bio;
		};
		struct {
			void *bi_private_orig;
			bio_end_io_t *bi_end_io_orig;
		};
	};
};

struct blk_crypto_fallback_keyslot {
	enum blk_crypto_mode_num crypto_mode;
	struct crypto_sync_skcipher *tfms[BLK_ENCRYPTION_MODE_MAX];
};

union blk_crypto_iv {
	__le64 dun[BLK_CRYPTO_DUN_ARRAY_SIZE];
	u8 bytes[BLK_CRYPTO_MAX_IV_SIZE];
};


extern unsigned int rust_bcf_num_prealloc_bounce_pg;
extern unsigned int rust_bcf_num_keyslots;
extern unsigned int rust_bcf_num_prealloc_fallback_crypt_ctxs;
void rust_bcf_lock(void);
void rust_bcf_unlock(void);
bool rust_bcf_load_acquire(const bool *p);
void rust_bcf_store_release(bool *p);
void rust_bcf_warn_invalid_slot(bool value);
void rust_bcf_warn_key_clear(int err);
bool rust_bcf_warn_uninitialized(bool value);
void rust_bcf_warn_missing_cipher(const char *cipher);
long rust_bcf_tfm_error(struct crypto_sync_skcipher *tfm);
int rust_bcf_setkey(struct crypto_sync_skcipher *tfm, const u8 *key, unsigned int len);
void rust_bcf_forbid_weak_keys(struct crypto_sync_skcipher *tfm);
void rust_bcf_free_tfm(struct crypto_sync_skcipher *tfm);
/* Direct C-to-Rust calls avoid a second nullable function-pointer ABI. */
void rust_bcf_encrypt_with_request(struct skcipher_request *req, struct bio *bio);
blk_status_t rust_bcf_decrypt_with_request(struct skcipher_request *req, void *data);
void rust_bcf_decrypt_work(struct work_struct *work);
void rust_bcf_with_encrypt_request(struct crypto_sync_skcipher *tfm, struct bio *bio);
blk_status_t rust_bcf_with_decrypt_request(struct crypto_sync_skcipher *tfm, void *data);
void rust_bcf_request_setup(struct skcipher_request *req, struct scatterlist *src,
                           struct scatterlist *dst, unsigned int len, void *iv);
void rust_bcf_sg_set_page(struct scatterlist *sg, struct page *page,
                         unsigned int len, unsigned int offset);
struct bio_vec *rust_bcf_first_bvec(struct bio *bio);
struct bio_vec rust_bcf_iter_iovec(struct bio *bio, struct bvec_iter iter);
void rust_bcf_advance_iter(struct bio *bio, struct bvec_iter *iter, unsigned int len);
unsigned int rust_bcf_segments(struct bio *bio);
bool rust_bcf_remapped(struct bio *bio);
void rust_bcf_set_remapped(struct bio *bio);
void rust_bcf_clone_blkg(struct bio *dst, struct bio *src);
void rust_bcf_inc_remaining(struct bio *bio);
void rust_bcf_endio_status(struct bio *bio, blk_status_t status);
bool rust_bcf_is_write(struct bio *bio);
void rust_bcf_free_crypt_ctx(struct bio *bio);
void rust_bcf_cmpxchg_status(struct bio *bio, blk_status_t status);
unsigned int rust_bcf_noio_save(void);
void rust_bcf_noio_restore(unsigned int flags);
unsigned int rust_bcf_alloc_pages(unsigned int nr, struct page **pages);
void rust_bcf_alloc_pool_pages(mempool_t *pool, void **pages, unsigned int nr);
void rust_bcf_release_pages(struct page **pages, unsigned int nr);
void *rust_bcf_alloc_ctx(mempool_t *pool);
struct blk_crypto_profile *rust_bcf_alloc_profile(void);
struct blk_crypto_fallback_keyslot *rust_bcf_alloc_slots(unsigned int nr);
struct workqueue_struct *rust_bcf_alloc_workqueue(void);
mempool_t *rust_bcf_create_page_pool(unsigned int nr);
struct kmem_cache *rust_bcf_create_ctx_cache(void);
mempool_t *rust_bcf_create_ctx_pool(unsigned int nr, struct kmem_cache *cache);
struct bio_fallback_crypt_ctx *rust_bcf_from_work(struct work_struct *work);
struct work_struct *rust_bcf_ctx_work(struct bio_fallback_crypt_ctx *ctx);
struct bio *rust_bcf_ctx_bio(struct bio_fallback_crypt_ctx *ctx);
void rust_bcf_ctx_set_bio(struct bio_fallback_crypt_ctx *ctx, struct bio *bio);
void rust_bcf_save_completion(struct bio_fallback_crypt_ctx *ctx, struct bio *bio);
void rust_bcf_restore_completion(struct bio_fallback_crypt_ctx *ctx, struct bio *bio);
void rust_bcf_init_work(struct work_struct *work);
bool rust_bcf_queue_work(struct workqueue_struct *wq, struct work_struct *work);

/* Evaluate configuration-dependent macros with the configured C compiler. */
enum {
    RUST_BCF_STS_OK = BLK_STS_OK,
    RUST_BCF_STS_IOERR = BLK_STS_IOERR,
    RUST_BCF_STS_INVAL = BLK_STS_INVAL,
    RUST_BCF_STS_NOTSUPP = BLK_STS_NOTSUPP,
    RUST_BCF_GFP_NOIO = GFP_NOIO,
    RUST_BCF_PAGE_SIZE = PAGE_SIZE,
    RUST_BCF_DUN_ARRAY_SIZE = BLK_CRYPTO_DUN_ARRAY_SIZE,
};
#endif
