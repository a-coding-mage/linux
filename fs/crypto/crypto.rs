// SPDX-License-Identifier: GPL-2.0-only
/*
 * This contains encryption functions for per-file encryption.
 *
 * Copyright (C) 2015, Google, Inc.
 * Copyright (C) 2015, Motorola Mobility
 *
 * Written by Michael Halcrow, 2014.
 *
 * Filename encryption additions
 *	Uday Savagaonkar, 2014
 * Encryption policy handling additions
 *	Ildar Muslukhov, 2014
 * Add fscrypt_pullback_bio_page()
 *	Jaegeuk Kim, 2015.
 *
 * This has not yet undergone a rigorous security audit.
 */

// C headers and fscrypt_private.h provide the external kernel declarations used below.

static mut NUM_PREALLOC_CRYPTO_PAGES: u32 = 32;
static mut FSCRYPT_BOUNCE_PAGE_POOL: *mut mempool_t = core::ptr::null_mut();
static mut FSCRYPT_INIT_MUTEX: mutex = mutex::new();
static mut fscrypt_inode_info_cachep: *mut kmem_cache = core::ptr::null_mut();

unsafe fn fscrypt_alloc_bounce_page(gfp_flags: gfp_t) -> *mut page {
    if WARN_ON_ONCE(FSCRYPT_BOUNCE_PAGE_POOL.is_null()) {
        return core::ptr::null_mut();
    }
    mempool_alloc(FSCRYPT_BOUNCE_PAGE_POOL, gfp_flags)
}

/// Free a ciphertext bounce page.
#[no_mangle]
pub unsafe extern "C" fn fscrypt_free_bounce_page(bounce_page: *mut page) {
    if bounce_page.is_null() { return; }
    set_page_private(bounce_page, 0);
    ClearPagePrivate(bounce_page);
    mempool_free(bounce_page, FSCRYPT_BOUNCE_PAGE_POOL);
}

pub unsafe extern "C" fn fscrypt_generate_iv(
    iv: *mut fscrypt_iv, index: u64, ci: *const fscrypt_inode_info,
) {
    let flags = fscrypt_policy_flags(&(*ci).ci_policy);
    core::ptr::write_bytes(iv as *mut u8, 0, core::mem::size_of::<fscrypt_iv>());
    if flags & FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64 != 0 {
        WARN_ON_ONCE(index > u32::MAX as u64);
        WARN_ON_ONCE((*(*ci).ci_inode).i_ino > u32::MAX as u64);
        index |= (*(*ci).ci_inode).i_ino << 32;
    } else if flags & FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32 != 0 {
        WARN_ON_ONCE(index > u32::MAX as u64);
        index = ((*ci).ci_hashed_ino.wrapping_add(index)) as u32 as u64;
    } else if flags & FSCRYPT_POLICY_FLAG_DIRECT_KEY != 0 {
        core::ptr::copy_nonoverlapping((*ci).ci_nonce.as_ptr(), (*iv).nonce.as_mut_ptr(), FSCRYPT_FILE_NONCE_SIZE);
    }
    (*iv).index = index.to_le();
}

unsafe fn fscrypt_crypt_data_unit(
    ci: *const fscrypt_inode_info, rw: fscrypt_direction_t, index: u64,
    src_page: *mut page, dest_page: *mut page, len: u32, offs: u32,
) -> i32 {
    if WARN_ON_ONCE(ci.is_null()) { return -ENOKEY; }
    let tfm = (*ci).ci_enc_key.tfm;
    if WARN_ON_ONCE(tfm.is_null()) { return -ENOKEY; }
    if WARN_ON_ONCE(len == 0) { return -EINVAL; }
    if WARN_ON_ONCE(len % FSCRYPT_CONTENTS_ALIGNMENT != 0) { return -EINVAL; }
    let mut iv = core::mem::zeroed::<fscrypt_iv>();
    fscrypt_generate_iv(&mut iv, index, ci);
    let mut req = SYNC_SKCIPHER_REQUEST_ON_STACK!(tfm);
    skcipher_request_set_callback(&mut req, CRYPTO_TFM_REQ_MAY_BACKLOG | CRYPTO_TFM_REQ_MAY_SLEEP, None, core::ptr::null_mut());
    let mut dst = core::mem::zeroed::<scatterlist>();
    let mut src = core::mem::zeroed::<scatterlist>();
    sg_init_table(&mut dst, 1); sg_set_page(&mut dst, dest_page, len, offs);
    sg_init_table(&mut src, 1); sg_set_page(&mut src, src_page, len, offs);
    skcipher_request_set_crypt(&mut req, &mut src, &mut dst, len, &mut iv);
    if rw == FS_DECRYPT { crypto_skcipher_decrypt(&mut req) } else { crypto_skcipher_encrypt(&mut req) }
}

pub unsafe extern "C" fn fscrypt_encrypt_pagecache_blocks(
    folio: *mut folio, len: usize, offs: usize, gfp_flags: gfp_t,
) -> *mut page {
    let inode = (*(*folio).mapping).host;
    let ci = fscrypt_get_inode_info_raw(inode);
    if WARN_ON_ONCE(ci.is_null()) { return ERR_PTR(-ENOKEY); }
    let du_bits = (*ci).ci_data_unit_bits;
    let du_size = 1usize << du_bits;
    let mut index = (folio_pos(folio) + offs) >> du_bits;
    VM_BUG_ON_FOLIO(folio_test_large(folio), folio);
    if WARN_ON_ONCE(!folio_test_locked(folio)) { return ERR_PTR(-EINVAL); }
    if WARN_ON_ONCE(len == 0 || !IS_ALIGNED(len | offs, du_size)) { return ERR_PTR(-EINVAL); }
    let ciphertext_page = fscrypt_alloc_bounce_page(gfp_flags);
    if ciphertext_page.is_null() { return ERR_PTR(-ENOMEM); }
    let mut i = offs;
    while i < offs + len {
        let err = fscrypt_crypt_data_unit(ci, FS_ENCRYPT, index, &mut (*folio).page, ciphertext_page, du_size as u32, i as u32);
        if err != 0 {
            fscrypt_free_bounce_page(ciphertext_page);
            return ERR_PTR(err);
        }
        i += du_size;
        index += 1;
    }
    SetPagePrivate(ciphertext_page);
    set_page_private(ciphertext_page, folio as usize);
    ciphertext_page
}

#[no_mangle]
pub unsafe extern "C" fn fscrypt_encrypt_block_inplace(inode: *const inode, page: *mut page, len: u32, offs: u32, lblk_num: u64) -> i32 {
    if WARN_ON_ONCE((*(*inode).i_sb).s_cop.supports_subblock_data_units) { return -EOPNOTSUPP; }
    fscrypt_crypt_data_unit(fscrypt_get_inode_info_raw(inode), FS_ENCRYPT, lblk_num, page, page, len, offs)
}

#[no_mangle]
pub unsafe extern "C" fn fscrypt_decrypt_block_inplace(inode: *const inode, page: *mut page, len: u32, offs: u32, lblk_num: u64) -> i32 {
    if WARN_ON_ONCE((*(*inode).i_sb).s_cop.supports_subblock_data_units) { return -EOPNOTSUPP; }
    fscrypt_crypt_data_unit(fscrypt_get_inode_info_raw(inode), FS_DECRYPT, lblk_num, page, page, len, offs)
}

#[no_mangle]
pub unsafe extern "C" fn fscrypt_initialize(sb: *mut super_block) -> i32 {
    if !core::ptr::read_volatile(&FSCRYPT_BOUNCE_PAGE_POOL).is_null() { return 0; }
    if !(*sb).s_cop.needs_bounce_pages { return 0; }
    let _guard = mutex_guard(&mut FSCRYPT_INIT_MUTEX);
    if !FSCRYPT_BOUNCE_PAGE_POOL.is_null() { return 0; }
    let pool = mempool_create_page_pool(NUM_PREALLOC_CRYPTO_PAGES, 0);
    if pool.is_null() { return -ENOMEM; }
    core::ptr::write_volatile(&mut FSCRYPT_BOUNCE_PAGE_POOL, pool);
    0
}

pub unsafe extern "C" fn fscrypt_msg(_inode: *const inode, _level: *const i8, _fmt: *const i8, ...) {
    // Variadic kernel logging implementation is supplied by the surrounding kernel bindings.
}

unsafe extern "C" fn fscrypt_init() -> i32 {
    fscrypt_inode_info_cachep = KMEM_CACHE!(fscrypt_inode_info, SLAB_RECLAIM_ACCOUNT | SLAB_PANIC);
    fscrypt_init_keyring();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
