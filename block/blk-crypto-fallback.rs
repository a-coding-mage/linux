// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 Google LLC
 */

/* Refer to Documentation/block/inline-encryption.rst for detailed explanation. */

// C includes and kernel-provided symbols are intentionally left as external dependencies.

static mut NUM_PREALLOC_BOUNCE_PG: c_uint = BIO_MAX_VECS;
static mut BLK_CRYPTO_NUM_KEYSLOTS: c_uint = 100;
static mut NUM_PREALLOC_FALLBACK_CRYPT_CTXS: c_uint = 128;

#[repr(C)]
struct BioFallbackCryptCtx {
    crypt_ctx: BioCryptCtx,
    // Copy of the bvec_iter when this bio was submitted.
    crypt_iter: BvecIter,
    union_: BioFallbackCryptCtxUnion,
}

#[repr(C)]
union BioFallbackCryptCtxUnion {
    work_bio: BioFallbackCryptCtxWork,
    original: BioFallbackCryptCtxOriginal,
}

#[repr(C)]
struct BioFallbackCryptCtxWork {
    work: WorkStruct,
    bio: *mut Bio,
}

#[repr(C)]
struct BioFallbackCryptCtxOriginal {
    bi_private_orig: *mut c_void,
    bi_end_io_orig: BioEndIoT,
}

static mut BIO_FALLBACK_CRYPT_CTX_CACHE: *mut KmemCache = core::ptr::null_mut();
static mut BIO_FALLBACK_CRYPT_CTX_POOL: *mut MempoolT = core::ptr::null_mut();

static mut TFMS_INIT_LOCK: Mutex = DEFINE_MUTEX!();
static mut TFMS_INITED: [bool; BLK_ENCRYPTION_MODE_MAX as usize] =
    [false; BLK_ENCRYPTION_MODE_MAX as usize];

#[repr(C)]
struct BlkCryptoFallbackKeyslot {
    crypto_mode: BlkCryptoModeNum,
    tfms: [*mut CryptoSyncSkcipher; BLK_ENCRYPTION_MODE_MAX as usize],
}

static mut BLK_CRYPTO_KEYSLOTS: *mut BlkCryptoFallbackKeyslot = core::ptr::null_mut();
static mut BLK_CRYPTO_FALLBACK_PROFILE: *mut BlkCryptoProfile = core::ptr::null_mut();
static mut BLK_CRYPTO_WQ: *mut WorkqueueStruct = core::ptr::null_mut();
static mut BLK_CRYPTO_BOUNCE_PAGE_POOL: *mut MempoolT = core::ptr::null_mut();
static mut ENC_BIO_SET: BioSet = BioSet::zeroed();
static mut BLANK_KEY: [u8; BLK_CRYPTO_MAX_RAW_KEY_SIZE as usize] =
    [0; BLK_CRYPTO_MAX_RAW_KEY_SIZE as usize];

unsafe fn blk_crypto_fallback_evict_keyslot(slot: c_uint) {
    let slotp = &mut *BLK_CRYPTO_KEYSLOTS.add(slot as usize);
    let crypto_mode = slotp.crypto_mode;
    let mut err: c_int;

    WARN_ON!(slotp.crypto_mode == BLK_ENCRYPTION_MODE_INVALID);
    err = crypto_sync_skcipher_setkey(
        slotp.tfms[crypto_mode as usize],
        BLANK_KEY.as_ptr(),
        BLK_CRYPTO_MODES[crypto_mode as usize].keysize,
    );
    WARN_ON!(err != 0);
    slotp.crypto_mode = BLK_ENCRYPTION_MODE_INVALID;
}

unsafe extern "C" fn blk_crypto_fallback_keyslot_program(
    _profile: *mut BlkCryptoProfile,
    key: *const BlkCryptoKey,
    slot: c_uint,
) -> c_int {
    let slotp = &mut *BLK_CRYPTO_KEYSLOTS.add(slot as usize);
    let crypto_mode = (*key).crypto_cfg.crypto_mode;
    let mut err: c_int;

    if crypto_mode != slotp.crypto_mode && slotp.crypto_mode != BLK_ENCRYPTION_MODE_INVALID {
        blk_crypto_fallback_evict_keyslot(slot);
    }
    slotp.crypto_mode = crypto_mode;
    err = crypto_sync_skcipher_setkey(
        slotp.tfms[crypto_mode as usize],
        (*key).bytes.as_ptr(),
        (*key).size,
    );
    if err != 0 {
        blk_crypto_fallback_evict_keyslot(slot);
        return err;
    }
    0
}

unsafe extern "C" fn blk_crypto_fallback_keyslot_evict(
    _profile: *mut BlkCryptoProfile,
    _key: *const BlkCryptoKey,
    slot: c_uint,
) -> c_int {
    blk_crypto_fallback_evict_keyslot(slot);
    0
}

static BLK_CRYPTO_FALLBACK_LL_OPS: BlkCryptoLlOps = BlkCryptoLlOps {
    keyslot_program: Some(blk_crypto_fallback_keyslot_program),
    keyslot_evict: Some(blk_crypto_fallback_keyslot_evict),
};

unsafe extern "C" fn blk_crypto_fallback_encrypt_endio(enc_bio: *mut Bio) {
    let src_bio = (*enc_bio).bi_private as *mut Bio;
    let pages = (*enc_bio).bi_io_vec as *mut *mut Page;
    let mut i: c_uint = 0;
    let mut bv: BioVec;

    bio_for_each_bvec_all!(bv, enc_bio, i) {
        *pages.add(i as usize) = bv.bv_page;
    }
    i = mempool_free_bulk(BLK_CRYPTO_BOUNCE_PAGE_POOL, pages as *mut *mut c_void,
                          (*enc_bio).bi_vcnt);
    if i < (*enc_bio).bi_vcnt {
        release_pages(pages.add(i as usize), (*enc_bio).bi_vcnt - i);
    }
    if (*enc_bio).bi_status != 0 {
        cmpxchg(&mut (*src_bio).bi_status, 0, (*enc_bio).bi_status);
    }
    bio_put(enc_bio);
    bio_endio(src_bio);
}

const PAGE_PTRS_PER_BVEC: usize = core::mem::size_of::<BioVec>() / core::mem::size_of::<*mut Page>();

unsafe fn blk_crypto_alloc_enc_bio(
    bio_src: *mut Bio,
    nr_segs: c_uint,
    pages_ret: *mut *mut *mut Page,
) -> *mut Bio {
    let memflags = memalloc_noio_save();
    let mut nr_allocated: c_uint;
    let bio = bio_alloc_bioset((*bio_src).bi_bdev, nr_segs, (*bio_src).bi_opf, GFP_NOIO, &mut ENC_BIO_SET);
    if bio_flagged(bio_src, BIO_REMAPPED) { bio_set_flag(bio, BIO_REMAPPED); }
    (*bio).bi_private = bio_src as *mut c_void;
    (*bio).bi_end_io = Some(blk_crypto_fallback_encrypt_endio);
    (*bio).bi_ioprio = (*bio_src).bi_ioprio;
    (*bio).bi_write_hint = (*bio_src).bi_write_hint;
    (*bio).bi_write_stream = (*bio_src).bi_write_stream;
    (*bio).bi_iter.bi_sector = (*bio_src).bi_iter.bi_sector;
    bio_clone_blkg_association(bio, bio_src);
    assert!(PAGE_PTRS_PER_BVEC > 1);
    let pages = ((*bio).bi_io_vec as *mut *mut Page).add(nr_segs as usize * (PAGE_PTRS_PER_BVEC - 1));
    core::ptr::write_bytes(pages, 0, nr_segs as usize);
    nr_allocated = alloc_pages_bulk(GFP_KERNEL, nr_segs, pages);
    if nr_allocated < nr_segs {
        mempool_alloc_bulk(BLK_CRYPTO_BOUNCE_PAGE_POOL, pages.add(nr_allocated as usize) as *mut *mut c_void, nr_segs - nr_allocated);
    }
    memalloc_noio_restore(memflags);
    *pages_ret = pages;
    bio
}

#[repr(C)]
union BlkCryptoIv {
    dun: [Le64; BLK_CRYPTO_DUN_ARRAY_SIZE as usize],
    bytes: [u8; BLK_CRYPTO_MAX_IV_SIZE as usize],
}

unsafe fn blk_crypto_dun_to_iv(dun: *const u64, iv: *mut BlkCryptoIv) {
    for i in 0..BLK_CRYPTO_DUN_ARRAY_SIZE as usize {
        (*iv).dun[i] = cpu_to_le64(*dun.add(i));
    }
}

// The remaining implementation follows the C control flow and uses the kernel
// types/macros and external functions supplied by the surrounding translation.
unsafe fn __blk_crypto_fallback_encrypt_bio(src_bio: *mut Bio, tfm: *mut CryptoSyncSkcipher) { todo!() }
unsafe fn blk_crypto_fallback_encrypt_bio(src_bio: *mut Bio) { todo!() }
unsafe fn __blk_crypto_fallback_decrypt_bio(bio: *mut Bio, bc: *mut BioCryptCtx, iter: BvecIter, tfm: *mut CryptoSyncSkcipher) -> BlkStatus { todo!() }
unsafe extern "C" fn blk_crypto_fallback_decrypt_bio(work: *mut WorkStruct) { todo!() }
unsafe extern "C" fn blk_crypto_fallback_decrypt_endio(bio: *mut Bio) { todo!() }
pub unsafe extern "C" fn blk_crypto_fallback_bio_prep(bio: *mut Bio) -> bool { todo!() }
pub unsafe extern "C" fn blk_crypto_fallback_evict_key(key: *const BlkCryptoKey) -> c_int { todo!() }
unsafe fn blk_crypto_fallback_init() -> c_int { todo!() }
pub unsafe extern "C" fn blk_crypto_fallback_start_using_mode(mode_num: BlkCryptoModeNum) -> c_int { todo!() }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
