/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Shared KUnit test cases for AEAD algorithms, including a benchmark. */
/*
 * This is a literal low-level Rust translation of the C header.  AEAD_* names,
 * KUnit helpers, and kernel types/functions are supplied by the including code.
 */

#[repr(C)]
pub struct aead_incremental_info { pub num_data_parts: usize, pub num_ad_parts: usize }

unsafe fn aead_alloc_random_key(test: *mut kunit, tag_len_ret: *mut usize) -> *mut AEAD_KEY {
    let key_len = AEAD_VALID_KEY_LENS[rand32() as usize % AEAD_VALID_KEY_LENS.len()];
    let tag_len = AEAD_VALID_TAG_LENS[rand32() as usize % AEAD_VALID_TAG_LENS.len()];
    let mut raw_key = [0u8; AEAD_MAX_KEY_LEN];
    let key = alloc_buf(test, core::mem::size_of::<AEAD_KEY>()) as *mut AEAD_KEY;
    rand_bytes(raw_key.as_mut_ptr(), key_len);
    let err = AEAD_PREPAREKEY(key, raw_key.as_ptr(), key_len, tag_len);
    KUNIT_ASSERT_EQ!(test, 0, err);
    *tag_len_ret = tag_len; key
}
unsafe fn aead_alloc_random_data(test: *mut kunit, len: usize) -> *mut u8 {
    let buf = alloc_buf(test, len) as *mut u8; rand_bytes(buf, len); buf
}
unsafe fn aead_alloc_random_data_guarded(test: *mut kunit, len: usize) -> *mut u8 {
    let buf = alloc_guarded_buf(test, len) as *mut u8; rand_bytes(buf, len); buf
}

unsafe fn aead_auth_incrementally(ctx: *mut AEAD_CTX, ad: *const u8, ad_len: usize) -> usize {
    let mut n = 0; let mut pos = 0;
    while rand_bool() { let l = rand_length(ad_len - pos); AEAD_AUTH_UPDATE(ctx, ad.add(pos), l); pos += l; n += 1; }
    if pos < ad_len || rand_bool() { AEAD_AUTH_UPDATE(ctx, ad.add(pos), ad_len - pos); n += 1; } n
}
unsafe fn aead_crypt_incrementally(ctx: *mut AEAD_CTX, dst: *mut u8, src: *const u8,
                                    data_len: usize, enc: bool) -> usize {
    let mut n = 0; let mut pos = 0;
    while rand_bool() { let l = rand_length(data_len - pos); if enc { AEAD_ENCRYPT_UPDATE(ctx,dst.add(pos),src.add(pos),l) } else { AEAD_DECRYPT_UPDATE(ctx,dst.add(pos),src.add(pos),l) }; pos += l; n += 1; }
    if pos < data_len || rand_bool() { if enc { AEAD_ENCRYPT_UPDATE(ctx,dst.add(pos),src.add(pos),data_len-pos) } else { AEAD_DECRYPT_UPDATE(ctx,dst.add(pos),src.add(pos),data_len-pos) }; n += 1; } n
}
unsafe fn aead_incr_info_str(test: *mut kunit, info: *const aead_incremental_info) -> *mut i8 {
    let s = alloc_buf(test, 64) as *mut i8;
    snprintf(s, 64, c"num_data_parts=%zu num_ad_parts=%zu", (*info).num_data_parts, (*info).num_ad_parts); s
}

unsafe fn aead_encrypt_incrementally(test:*mut kunit,ctx:*mut AEAD_CTX,dst:*mut u8,src:*const u8,data_len:usize,tag:*mut u8,ad:*const u8,ad_len:usize,nonce:*const u8,nonce_len:usize,key:*const AEAD_KEY)->aead_incremental_info {
    let mut i=aead_incremental_info{num_data_parts:0,num_ad_parts:0}; let e=AEAD_INIT(ctx,data_len,ad_len,nonce,nonce_len,key); KUNIT_ASSERT_EQ!(test,0,e); i.num_ad_parts=aead_auth_incrementally(ctx,ad,ad_len); i.num_data_parts=aead_crypt_incrementally(ctx,dst,src,data_len,true); AEAD_ENCRYPT_FINAL(ctx,tag); KUNIT_ASSERT_TRUE_MSG!(test,mem_is_zero(ctx,core::mem::size_of::<AEAD_CTX>()),c"encrypt_final didn't zeroize context"); i
}
unsafe fn aead_decrypt_incrementally(test:*mut kunit,ctx:*mut AEAD_CTX,dst:*mut u8,src:*const u8,data_len:usize,tag:*const u8,ad:*const u8,ad_len:usize,nonce:*const u8,nonce_len:usize,key:*const AEAD_KEY)->aead_incremental_info {
    let mut i=aead_incremental_info{num_data_parts:0,num_ad_parts:0}; let e=AEAD_INIT(ctx,data_len,ad_len,nonce,nonce_len,key); KUNIT_ASSERT_EQ!(test,0,e); i.num_ad_parts=aead_auth_incrementally(ctx,ad,ad_len); i.num_data_parts=aead_crypt_incrementally(ctx,dst,src,data_len,false); let e=AEAD_DECRYPT_FINAL(ctx,tag); KUNIT_ASSERT_EQ!(test,0,e); KUNIT_ASSERT_TRUE_MSG!(test,mem_is_zero(ctx,core::mem::size_of::<AEAD_CTX>()),c"decrypt_final didn't zeroize context"); i
}

unsafe fn aead_is_key_len_expected_valid(x:usize)->bool { AEAD_VALID_KEY_LENS.iter().any(|&v|v==x) }
unsafe fn aead_is_nonce_len_expected_valid(x:usize)->bool { AEAD_VALID_NONCE_LENS.iter().any(|&v|v==x) }
unsafe fn aead_is_tag_len_expected_valid(x:usize)->bool { AEAD_VALID_TAG_LENS.iter().any(|&v|v==x) }

#[repr(C)] pub struct aead_basic_validation_test_ctx {
    pub key: AEAD_KEY, pub ctx: AEAD_CTX, pub raw_key_buf_end:*mut u8, pub nonce_buf_end:*mut u8, pub tag_buf_end:*mut u8,
    pub pt:[u8;64], pub ct:[u8;64], pub decrypted:[u8;64], pub ad:[u8;16], pub unused_buf:*mut u8, pub data_len:usize, pub ad_len:usize,
}
unsafe fn aead_alloc_basic_validation_test_ctx(test:*mut kunit)->*mut aead_basic_validation_test_ctx {
    let c=alloc_buf(test,core::mem::size_of::<aead_basic_validation_test_ctx>()) as *mut aead_basic_validation_test_ctx; memset(c as *mut _,0,core::mem::size_of::<aead_basic_validation_test_ctx>());
    (*c).raw_key_buf_end=aead_alloc_random_data_guarded(test,AEAD_MAX_KEY_LEN).add(AEAD_MAX_KEY_LEN); (*c).nonce_buf_end=aead_alloc_random_data_guarded(test,AEAD_MAX_NONCE_LEN).add(AEAD_MAX_NONCE_LEN); (*c).tag_buf_end=aead_alloc_random_data_guarded(test,AEAD_MAX_TAG_LEN).add(AEAD_MAX_TAG_LEN); (*c).unused_buf=alloc_buf(test,core::cmp::max(AEAD_MAX_KEY_LEN,core::cmp::max(AEAD_MAX_NONCE_LEN,AEAD_MAX_TAG_LEN))) as *mut u8; (*c).data_len=64; (*c).ad_len=16; c
}

/* The remaining test entry points retain the source header's externally supplied
 * KUnit/AEAD operations and are intentionally declared here for the generated
 * suite to provide. */
extern "C" {
    fn test_aead_all_key_lens(test:*mut kunit); fn test_aead_all_nonce_lens(test:*mut kunit); fn test_aead_all_tag_lens(test:*mut kunit);
    fn test_aead_incremental_updates(test:*mut kunit); fn test_aead_data_buffer_overruns(test:*mut kunit);
    fn test_aead_alignment_consistency(test:*mut kunit); fn test_aead_inplace(test:*mut kunit);
    fn test_aead_monte_carlo(test:*mut kunit); fn test_aead_interrupt_context(test:*mut kunit); fn benchmark_aead(test:*mut kunit);
}

/* clang-format off */
#[macro_export] macro_rules! AEAD_KUNIT_CASES { () => {
    KUNIT_CASE!(test_aead_all_key_lens), KUNIT_CASE!(test_aead_all_nonce_lens), KUNIT_CASE!(test_aead_all_tag_lens),
    KUNIT_CASE!(test_aead_incremental_updates), KUNIT_CASE!(test_aead_data_buffer_overruns),
    KUNIT_CASE!(test_aead_alignment_consistency), KUNIT_CASE!(test_aead_inplace), KUNIT_CASE!(test_aead_monte_carlo),
    KUNIT_CASE!(test_aead_interrupt_context), KUNIT_CASE!(benchmark_aead)
}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
