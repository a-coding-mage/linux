// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 Google LLC
 */
// Source reconstruction of the retained block/blk-crypto-fallback.c.
// Refer to Documentation/block/inline-encryption.rst for the caller contracts.
// Shared and private C layouts come from configured original C declarations.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/blk_crypto_fallback_generated.rs"));
}
use bindings::*;
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_int, c_uint, c_void};

static mut BIO_FALLBACK_CRYPT_CTX_CACHE: *mut kmem_cache = null_mut();
static mut BIO_FALLBACK_CRYPT_CTX_POOL: *mut mempool_t = null_mut();
static mut TFMS_INITED: [bool; BLK_ENCRYPTION_MODE_MAX as usize] =
    [false; BLK_ENCRYPTION_MODE_MAX as usize];
static mut BLK_CRYPTO_KEYSLOTS: *mut blk_crypto_fallback_keyslot = null_mut();
static mut BLK_CRYPTO_FALLBACK_PROFILE: *mut blk_crypto_profile = null_mut();
static mut BLK_CRYPTO_WQ: *mut workqueue_struct = null_mut();
static mut BLK_CRYPTO_BOUNCE_PAGE_POOL: *mut mempool_t = null_mut();
static mut ENC_BIO_SET: bio_set = unsafe { zeroed() };
static mut BLANK_KEY: [u8; BLK_CRYPTO_MAX_RAW_KEY_SIZE as usize] =
    [0; BLK_CRYPTO_MAX_RAW_KEY_SIZE as usize];
static mut BLK_CRYPTO_FALLBACK_INITED: bool = false;

// Plain accesses below are either unpublished initialization, serialized by the
// original profile/mode lock, or covered by the caller's started-key lifetime.
// The mode publication and source-bio status compare-exchange use C LKMM glue.
unsafe fn mode_ptr(mode: blk_crypto_mode_num) -> *const blk_crypto_mode {
    addr_of!(blk_crypto_modes).cast::<blk_crypto_mode>().add(mode as usize)
}
unsafe fn tfm_ptr(slot: *mut blk_crypto_fallback_keyslot, mode: blk_crypto_mode_num)
    -> *mut *mut crypto_sync_skcipher {
    addr_of_mut!((*slot).tfms).cast::<*mut crypto_sync_skcipher>().add(mode as usize)
}
unsafe fn inited_ptr(mode: blk_crypto_mode_num) -> *mut bool {
    addr_of_mut!(TFMS_INITED).cast::<bool>().add(mode as usize)
}
unsafe fn blk_crypto_fallback_evict_keyslot(slot: c_uint) {
    let slotp = BLK_CRYPTO_KEYSLOTS.add(slot as usize);
    let mode = (*slotp).crypto_mode;
    rust_bcf_warn_invalid_slot(mode == BLK_ENCRYPTION_MODE_INVALID);
    let err = rust_bcf_setkey(tfm_ptr(slotp, mode).read(),
        addr_of!(BLANK_KEY).cast(), (*mode_ptr(mode)).keysize);
    rust_bcf_warn_key_clear(err);
    (*slotp).crypto_mode = BLK_ENCRYPTION_MODE_INVALID;
}
unsafe extern "C" fn blk_crypto_fallback_keyslot_program(
    _profile: *mut blk_crypto_profile, key: *const blk_crypto_key, slot: c_uint,
) -> c_int {
    let slotp = BLK_CRYPTO_KEYSLOTS.add(slot as usize);
    let mode = (*key).crypto_cfg.crypto_mode;
    if mode != (*slotp).crypto_mode && (*slotp).crypto_mode != BLK_ENCRYPTION_MODE_INVALID {
        blk_crypto_fallback_evict_keyslot(slot);
    }
    (*slotp).crypto_mode = mode;
    let err = rust_bcf_setkey(tfm_ptr(slotp, mode).read(),
        addr_of!((*key).bytes).cast(), (*key).size);
    if err != 0 {
        blk_crypto_fallback_evict_keyslot(slot);
        return err;
    }
    0
}
unsafe extern "C" fn blk_crypto_fallback_keyslot_evict(
    _profile: *mut blk_crypto_profile, _key: *const blk_crypto_key, slot: c_uint,
) -> c_int {
    blk_crypto_fallback_evict_keyslot(slot);
    0
}

unsafe extern "C" fn blk_crypto_fallback_encrypt_endio(enc_bio: *mut bio) {
    let src_bio = (*enc_bio).bi_private.cast::<bio>();
    let pages = (*enc_bio).bi_io_vec.cast::<*mut page>();
    let bv = rust_bcf_first_bvec(enc_bio);
    // Read a vector's page before overwriting the same storage as page pointers.
    for i in 0..(*enc_bio).bi_vcnt as usize {
        let page = (*bv.add(i)).bv_page;
        pages.add(i).write(page);
    }
    let nr = (*enc_bio).bi_vcnt as c_uint;
    let i = mempool_free_bulk(BLK_CRYPTO_BOUNCE_PAGE_POOL, pages.cast(), nr);
    if i < nr {
        rust_bcf_release_pages(pages.add(i as usize), nr - i);
    }
    if (*enc_bio).bi_status != 0 {
        rust_bcf_cmpxchg_status(src_bio, (*enc_bio).bi_status);
    }
    bio_put(enc_bio);
    bio_endio(src_bio);
}
const PAGE_PTRS_PER_BVEC: usize = size_of::<bio_vec>() / size_of::<*mut page>();
const _: () = assert!(PAGE_PTRS_PER_BVEC > 1);
unsafe fn blk_crypto_alloc_enc_bio(
    bio_src: *mut bio, nr_segs: c_uint, pages_ret: *mut *mut *mut page,
) -> *mut bio {
    let memflags = rust_bcf_noio_save();
    let enc = bio_alloc_bioset((*bio_src).bi_bdev, nr_segs as u16, (*bio_src).bi_opf,
        RUST_BCF_GFP_NOIO, addr_of_mut!(ENC_BIO_SET));
    if rust_bcf_remapped(bio_src) { rust_bcf_set_remapped(enc); }
    (*enc).bi_private = bio_src.cast();
    (*enc).bi_end_io = Some(blk_crypto_fallback_encrypt_endio);
    (*enc).bi_ioprio = (*bio_src).bi_ioprio;
    (*enc).bi_write_hint = (*bio_src).bi_write_hint;
    (*enc).bi_write_stream = (*bio_src).bi_write_stream;
    (*enc).bi_iter.bi_sector = (*bio_src).bi_iter.bi_sector;
    rust_bcf_clone_blkg(enc, bio_src);
    let pages = (*enc).bi_io_vec.cast::<*mut page>()
        .add(nr_segs as usize * (PAGE_PTRS_PER_BVEC - 1));
    core::ptr::write_bytes(pages, 0, nr_segs as usize);
    let nr_allocated = rust_bcf_alloc_pages(nr_segs, pages);
    if nr_allocated < nr_segs {
        rust_bcf_alloc_pool_pages(BLK_CRYPTO_BOUNCE_PAGE_POOL,
            pages.add(nr_allocated as usize).cast(), nr_segs - nr_allocated);
    }
    rust_bcf_noio_restore(memflags);
    pages_ret.write(pages);
    enc
}
unsafe fn blk_crypto_fallback_tfm(slot: *mut blk_crypto_keyslot) -> *mut crypto_sync_skcipher {
    let slotp = BLK_CRYPTO_KEYSLOTS.add(blk_crypto_keyslot_index(slot) as usize);
    tfm_ptr(slotp, (*slotp).crypto_mode).read()
}
unsafe fn blk_crypto_dun_to_iv(dun: *const u64, iv: *mut blk_crypto_iv) {
    for i in 0..RUST_BCF_DUN_ARRAY_SIZE as usize {
        addr_of_mut!((*iv).dun).cast::<u64>().add(i).write(dun.add(i).read().to_le());
    }
}

// C owns the exact SYNC_SKCIPHER_REQUEST_ON_STACK size/alignment and invokes
// these synchronous callbacks before its stack frame ends. All crypto loops,
// BIO submission, allocation decisions and error unwinding stay in Rust.
#[export_name = "rust_bcf_encrypt_with_request"]
unsafe extern "C" fn encrypt_with_request(req: *mut skcipher_request, src_bio: *mut bio) {
    let bc = (*src_bio).bi_crypt_context;
    let data_unit_size = (*(*bc).bc_key).crypto_cfg.data_unit_size;
    let mut curr_dun = (*bc).bc_dun;
    let mut src: scatterlist = zeroed();
    let mut dst: scatterlist = zeroed();
    let mut iv: blk_crypto_iv = zeroed();
    sg_init_table(addr_of_mut!(src), 1);
    sg_init_table(addr_of_mut!(dst), 1);
    rust_bcf_request_setup(req, addr_of_mut!(src), addr_of_mut!(dst),
        data_unit_size, addr_of_mut!(iv).cast());

    'new_bio: loop {
        let nr_enc_pages = core::cmp::min(rust_bcf_segments(src_bio), BIO_MAX_VECS);
        let mut enc_pages = null_mut();
        let enc_bio = blk_crypto_alloc_enc_bio(src_bio, nr_enc_pages, addr_of_mut!(enc_pages));
        let mut enc_idx = 0;
        let failed = 'segments: loop {
            let src_bv = rust_bcf_iter_iovec(src_bio, (*src_bio).bi_iter);
            let enc_page = enc_pages.add(enc_idx as usize).read();
            if (src_bv.bv_len | src_bv.bv_offset) & data_unit_size.wrapping_sub(1) != 0 {
                (*enc_bio).bi_status = RUST_BCF_STS_INVAL as blk_status_t;
                break 'segments true;
            }
            __bio_add_page(enc_bio, enc_page, src_bv.bv_len, src_bv.bv_offset);
            rust_bcf_sg_set_page(addr_of_mut!(src), src_bv.bv_page, data_unit_size, src_bv.bv_offset);
            rust_bcf_sg_set_page(addr_of_mut!(dst), enc_page, data_unit_size, src_bv.bv_offset);
            // The added page must be counted before crypto can fail.
            enc_idx += 1;
            let mut i = 0;
            while i < src_bv.bv_len {
                blk_crypto_dun_to_iv(addr_of!(curr_dun).cast(), addr_of_mut!(iv));
                if crypto_skcipher_encrypt(req) != 0 {
                    (*enc_bio).bi_status = RUST_BCF_STS_IOERR as blk_status_t;
                    break 'segments true;
                }
                bio_crypt_dun_increment(addr_of_mut!(curr_dun).cast(), 1);
                src.offset = src.offset.wrapping_add(data_unit_size);
                dst.offset = dst.offset.wrapping_add(data_unit_size);
                i = i.wrapping_add(data_unit_size);
            }
            rust_bcf_advance_iter(src_bio, addr_of_mut!((*src_bio).bi_iter), src_bv.bv_len);
            if (*src_bio).bi_iter.bi_size == 0 { break 'segments false; }
            if enc_idx == nr_enc_pages {
                rust_bcf_inc_remaining(src_bio);
                submit_bio(enc_bio);
                continue 'new_bio;
            }
        };
        if failed {
            // Add all not-yet-attached pages so normal completion frees them.
            while enc_idx < nr_enc_pages {
                __bio_add_page(enc_bio, enc_pages.add(enc_idx as usize).read(), RUST_BCF_PAGE_SIZE, 0);
                enc_idx += 1;
            }
            bio_endio(enc_bio);
        } else {
            submit_bio(enc_bio);
        }
        return;
    }
}
unsafe fn __blk_crypto_fallback_encrypt_bio(src_bio: *mut bio, tfm: *mut crypto_sync_skcipher) {
    rust_bcf_with_encrypt_request(tfm, src_bio);
}
unsafe fn blk_crypto_fallback_encrypt_bio(src_bio: *mut bio) {
    let bc = (*src_bio).bi_crypt_context;
    let mut slot = null_mut();
    let status = blk_crypto_get_keyslot(BLK_CRYPTO_FALLBACK_PROFILE, (*bc).bc_key, addr_of_mut!(slot));
    if status != RUST_BCF_STS_OK as blk_status_t {
        rust_bcf_endio_status(src_bio, status);
        return;
    }
    __blk_crypto_fallback_encrypt_bio(src_bio, blk_crypto_fallback_tfm(slot));
    blk_crypto_put_keyslot(slot);
}

// Private synchronous Rust callback data, never dereferenced by C.
struct DecryptArgs { bio: *mut bio, bc: *mut bio_crypt_ctx, iter: bvec_iter }
#[export_name = "rust_bcf_decrypt_with_request"]
unsafe extern "C" fn decrypt_with_request(req: *mut skcipher_request, data: *mut c_void) -> blk_status_t {
    let args = data.cast::<DecryptArgs>();
    let bio = (*args).bio;
    let bc = (*args).bc;
    let mut iter = (*args).iter;
    let mut curr_dun = (*bc).bc_dun;
    let data_unit_size = (*(*bc).bc_key).crypto_cfg.data_unit_size;
    let mut sg: scatterlist = zeroed();
    let mut iv: blk_crypto_iv = zeroed();
    sg_init_table(addr_of_mut!(sg), 1);
    rust_bcf_request_setup(req, addr_of_mut!(sg), addr_of_mut!(sg),
        data_unit_size, addr_of_mut!(iv).cast());
    while iter.bi_size != 0 {
        let bv = rust_bcf_iter_iovec(bio, iter);
        if (bv.bv_len | bv.bv_offset) & data_unit_size.wrapping_sub(1) != 0 {
            return RUST_BCF_STS_INVAL as blk_status_t;
        }
        rust_bcf_sg_set_page(addr_of_mut!(sg), bv.bv_page, data_unit_size, bv.bv_offset);
        let mut i = 0;
        while i < bv.bv_len {
            blk_crypto_dun_to_iv(addr_of!(curr_dun).cast(), addr_of_mut!(iv));
            if crypto_skcipher_decrypt(req) != 0 { return RUST_BCF_STS_IOERR as blk_status_t; }
            bio_crypt_dun_increment(addr_of_mut!(curr_dun).cast(), 1);
            sg.offset = sg.offset.wrapping_add(data_unit_size);
            i = i.wrapping_add(data_unit_size);
        }
        rust_bcf_advance_iter(bio, addr_of_mut!(iter), bv.bv_len);
    }
    RUST_BCF_STS_OK as blk_status_t
}
unsafe fn __blk_crypto_fallback_decrypt_bio(
    bio: *mut bio, bc: *mut bio_crypt_ctx, iter: bvec_iter, tfm: *mut crypto_sync_skcipher,
) -> blk_status_t {
    let mut args = DecryptArgs { bio, bc, iter };
    rust_bcf_with_decrypt_request(tfm, addr_of_mut!(args).cast())
}
#[export_name = "rust_bcf_decrypt_work"]
unsafe extern "C" fn blk_crypto_fallback_decrypt_bio(work: *mut work_struct) {
    let f_ctx = rust_bcf_from_work(work);
    let bio = rust_bcf_ctx_bio(f_ctx);
    let bc = addr_of_mut!((*f_ctx).crypt_ctx);
    let mut slot = null_mut();
    let mut status = blk_crypto_get_keyslot(BLK_CRYPTO_FALLBACK_PROFILE, (*bc).bc_key, addr_of_mut!(slot));
    if status == RUST_BCF_STS_OK as blk_status_t {
        status = __blk_crypto_fallback_decrypt_bio(bio, bc, (*f_ctx).crypt_iter, blk_crypto_fallback_tfm(slot));
        blk_crypto_put_keyslot(slot);
    }
    mempool_free(f_ctx.cast(), BIO_FALLBACK_CRYPT_CTX_POOL);
    rust_bcf_endio_status(bio, status);
}
unsafe extern "C" fn blk_crypto_fallback_decrypt_endio(bio: *mut bio) {
    let f_ctx = (*bio).bi_private.cast::<bio_fallback_crypt_ctx>();
    // Restore both union-backed values before INIT_WORK overwrites the union.
    rust_bcf_restore_completion(f_ctx, bio);
    if (*bio).bi_status != 0 {
        mempool_free(f_ctx.cast(), BIO_FALLBACK_CRYPT_CTX_POOL);
        bio_endio(bio);
        return;
    }
    let work = rust_bcf_ctx_work(f_ctx);
    rust_bcf_init_work(work);
    rust_bcf_ctx_set_bio(f_ctx, bio);
    rust_bcf_queue_work(BLK_CRYPTO_WQ, work);
}
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_fallback_bio_prep(bio: *mut bio) -> bool {
    let bc = (*bio).bi_crypt_context;
    let mode = (*(*bc).bc_key).crypto_cfg.crypto_mode;
    if rust_bcf_warn_uninitialized(!inited_ptr(mode).read()) {
        rust_bcf_endio_status(bio, RUST_BCF_STS_IOERR as blk_status_t);
        return false;
    }
    if (*(*bc).bc_key).crypto_cfg.key_type != BLK_CRYPTO_KEY_TYPE_RAW {
        rust_bcf_endio_status(bio, RUST_BCF_STS_NOTSUPP as blk_status_t);
        return false;
    }
    if rust_bcf_is_write(bio) {
        blk_crypto_fallback_encrypt_bio(bio);
        return false;
    }
    let f_ctx = rust_bcf_alloc_ctx(BIO_FALLBACK_CRYPT_CTX_POOL).cast::<bio_fallback_crypt_ctx>();
    addr_of_mut!((*f_ctx).crypt_ctx).write(bc.read());
    addr_of_mut!((*f_ctx).crypt_iter).write((*bio).bi_iter);
    rust_bcf_save_completion(f_ctx, bio);
    (*bio).bi_private = f_ctx.cast();
    (*bio).bi_end_io = Some(blk_crypto_fallback_decrypt_endio);
    rust_bcf_free_crypt_ctx(bio);
    true
}
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_fallback_evict_key(key: *const blk_crypto_key) -> c_int {
    __blk_crypto_evict_key(BLK_CRYPTO_FALLBACK_PROFILE, key)
}

// Stage denotes the last successful original allocation. The fall-through
// cleanup order is exactly C's fail_free_* labels; failed init is retryable.
unsafe fn unwind_init(stage: u8) {
    if stage >= 7 { kmem_cache_destroy(BIO_FALLBACK_CRYPT_CTX_CACHE); }
    if stage >= 6 { mempool_destroy(BLK_CRYPTO_BOUNCE_PAGE_POOL); }
    if stage >= 5 { kfree(BLK_CRYPTO_KEYSLOTS.cast()); }
    if stage >= 4 { destroy_workqueue(BLK_CRYPTO_WQ); }
    if stage >= 3 { blk_crypto_profile_destroy(BLK_CRYPTO_FALLBACK_PROFILE); }
    if stage >= 2 { kfree(BLK_CRYPTO_FALLBACK_PROFILE.cast()); }
    if stage >= 1 { bioset_exit(addr_of_mut!(ENC_BIO_SET)); }
}
unsafe fn blk_crypto_fallback_init() -> c_int {
    if BLK_CRYPTO_FALLBACK_INITED { return 0; }
    get_random_bytes(addr_of_mut!(BLANK_KEY).cast(), size_of::<[u8; BLK_CRYPTO_MAX_RAW_KEY_SIZE as usize]>());
    let err = bioset_init(addr_of_mut!(ENC_BIO_SET), 64, 0, BIOSET_NEED_BVECS as c_int);
    if err != 0 { return err; }
    BLK_CRYPTO_FALLBACK_PROFILE = rust_bcf_alloc_profile();
    if BLK_CRYPTO_FALLBACK_PROFILE.is_null() { unwind_init(1); return -(ENOMEM as c_int); }
    let err = blk_crypto_profile_init(BLK_CRYPTO_FALLBACK_PROFILE, rust_bcf_num_keyslots);
    if err != 0 { unwind_init(2); return err; }
    let mut ops: blk_crypto_ll_ops = zeroed();
    ops.keyslot_program = Some(blk_crypto_fallback_keyslot_program);
    ops.keyslot_evict = Some(blk_crypto_fallback_keyslot_evict);
    // C's designated initializer leaves all four hardware-wrapped ops NULL.
    addr_of_mut!((*BLK_CRYPTO_FALLBACK_PROFILE).ll_ops).write(ops);
    (*BLK_CRYPTO_FALLBACK_PROFILE).max_dun_bytes_supported = BLK_CRYPTO_MAX_IV_SIZE;
    (*BLK_CRYPTO_FALLBACK_PROFILE).key_types_supported = BLK_CRYPTO_KEY_TYPE_RAW;
    for i in 0..BLK_ENCRYPTION_MODE_MAX as usize {
        addr_of_mut!((*BLK_CRYPTO_FALLBACK_PROFILE).modes_supported).cast::<c_uint>().add(i).write(0xffff_ffff);
    }
    addr_of_mut!((*BLK_CRYPTO_FALLBACK_PROFILE).modes_supported).cast::<c_uint>()
        .add(BLK_ENCRYPTION_MODE_INVALID as usize).write(0);
    BLK_CRYPTO_WQ = rust_bcf_alloc_workqueue();
    if BLK_CRYPTO_WQ.is_null() { unwind_init(3); return -(ENOMEM as c_int); }
    BLK_CRYPTO_KEYSLOTS = rust_bcf_alloc_slots(rust_bcf_num_keyslots);
    if BLK_CRYPTO_KEYSLOTS.is_null() { unwind_init(4); return -(ENOMEM as c_int); }
    BLK_CRYPTO_BOUNCE_PAGE_POOL = rust_bcf_create_page_pool(rust_bcf_num_prealloc_bounce_pg);
    if BLK_CRYPTO_BOUNCE_PAGE_POOL.is_null() { unwind_init(5); return -(ENOMEM as c_int); }
    BIO_FALLBACK_CRYPT_CTX_CACHE = rust_bcf_create_ctx_cache();
    if BIO_FALLBACK_CRYPT_CTX_CACHE.is_null() { unwind_init(6); return -(ENOMEM as c_int); }
    BIO_FALLBACK_CRYPT_CTX_POOL = rust_bcf_create_ctx_pool(rust_bcf_num_prealloc_fallback_crypt_ctxs, BIO_FALLBACK_CRYPT_CTX_CACHE);
    if BIO_FALLBACK_CRYPT_CTX_POOL.is_null() { unwind_init(7); return -(ENOMEM as c_int); }
    BLK_CRYPTO_FALLBACK_INITED = true;
    0
}
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_fallback_start_using_mode(mode: blk_crypto_mode_num) -> c_int {
    let cipher = (*mode_ptr(mode)).cipher_str;
    if rust_bcf_load_acquire(inited_ptr(mode)) { return 0; }
    rust_bcf_lock();
    let err = 'locked: {
        if inited_ptr(mode).read() { break 'locked 0; }
        let err = blk_crypto_fallback_init();
        if err != 0 { break 'locked err; }
        for i in 0..rust_bcf_num_keyslots as usize {
            let slotp = BLK_CRYPTO_KEYSLOTS.add(i);
            let tfm = crypto_alloc_sync_skcipher(cipher, 0, 0);
            tfm_ptr(slotp, mode).write(tfm);
            let mut err = rust_bcf_tfm_error(tfm) as c_int;
            if err != 0 {
                if err == -(ENOENT as c_int) {
                    rust_bcf_warn_missing_cipher(cipher);
                    err = -(ENOPKG as c_int);
                }
                tfm_ptr(slotp, mode).write(null_mut());
                for j in 0..rust_bcf_num_keyslots as usize {
                    let slotp = BLK_CRYPTO_KEYSLOTS.add(j);
                    rust_bcf_free_tfm(tfm_ptr(slotp, mode).read());
                    tfm_ptr(slotp, mode).write(null_mut());
                }
                break 'locked err;
            }
            rust_bcf_forbid_weak_keys(tfm);
        }
        rust_bcf_store_release(inited_ptr(mode));
        0
    };
    rust_bcf_unlock();
    err
}
