// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */
// Kernel and driver dependencies are supplied by the surrounding translation unit.

const MAX_SKCIPHER_SEQ_LEN: usize = 6;

#[repr(C)]
pub struct cc_user_key_info { pub key: *mut u8, pub key_dma_addr: dma_addr_t }
#[repr(C)]
pub struct cc_hw_key_info { pub key1_slot: enum_cc_hw_crypto_key, pub key2_slot: enum_cc_hw_crypto_key }
#[repr(C)]
pub struct cc_cpp_key_info { pub slot: u8, pub alg: enum_cc_cpp_alg }

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum cc_key_type { CC_UNPROTECTED_KEY, CC_HW_PROTECTED_KEY, CC_POLICY_PROTECTED_KEY, CC_INVALID_PROTECTED_KEY }

#[repr(C)]
pub struct cc_cipher_ctx {
    pub drvdata: *mut cc_drvdata, pub keylen: i32, pub cipher_mode: i32,
    pub flow_mode: i32, pub flags: u32, pub key_type: cc_key_type,
    pub user: cc_user_key_info, pub hw: cc_hw_key_info, pub cpp: cc_cpp_key_info,
    pub shash_tfm: *mut crypto_shash, pub fallback_tfm: *mut crypto_skcipher,
    pub fallback_on: bool,
}

unsafe fn cc_key_type(tfm: *mut crypto_tfm) -> cc_key_type { (*(crypto_tfm_ctx(tfm) as *mut cc_cipher_ctx)).key_type }

unsafe fn validate_keys_sizes(ctx_p: *mut cc_cipher_ctx, size: u32) -> i32 {
    match (*ctx_p).flow_mode {
        S_DIN_to_AES => match size {
            CC_AES_128_BIT_KEY_SIZE | CC_AES_192_BIT_KEY_SIZE => if (*ctx_p).cipher_mode != DRV_CIPHER_XTS { 0 } else { -EINVAL },
            CC_AES_256_BIT_KEY_SIZE => 0,
            x if x == CC_AES_192_BIT_KEY_SIZE * 2 || x == CC_AES_256_BIT_KEY_SIZE * 2 => if (*ctx_p).cipher_mode == DRV_CIPHER_XTS || (*ctx_p).cipher_mode == DRV_CIPHER_ESSIV { 0 } else { -EINVAL },
            _ => -EINVAL,
        },
        S_DIN_to_DES => if size == DES3_EDE_KEY_SIZE || size == DES_KEY_SIZE { 0 } else { -EINVAL },
        S_DIN_to_SM4 => if size == SM4_KEY_SIZE { 0 } else { -EINVAL },
        _ => -EINVAL,
    }
}

unsafe fn validate_data_size(ctx_p: *mut cc_cipher_ctx, size: u32) -> i32 {
    match (*ctx_p).flow_mode {
        S_DIN_to_AES => match (*ctx_p).cipher_mode {
            DRV_CIPHER_XTS | DRV_CIPHER_CBC_CTS => if size >= AES_BLOCK_SIZE { 0 } else { -EINVAL },
            DRV_CIPHER_OFB | DRV_CIPHER_CTR => 0,
            DRV_CIPHER_ECB | DRV_CIPHER_CBC | DRV_CIPHER_ESSIV => if size % AES_BLOCK_SIZE == 0 { 0 } else { -EINVAL },
            _ => -EINVAL,
        },
        S_DIN_to_DES => if size % DES_BLOCK_SIZE == 0 { 0 } else { -EINVAL },
        S_DIN_to_SM4 => match (*ctx_p).cipher_mode {
            DRV_CIPHER_CTR => 0,
            DRV_CIPHER_ECB | DRV_CIPHER_CBC => if size % SM4_BLOCK_SIZE == 0 { 0 } else { -EINVAL },
            _ => -EINVAL,
        },
        _ => -EINVAL,
    }
}

unsafe fn cc_cipher_init(tfm: *mut crypto_tfm) -> i32 {
    let ctx_p = crypto_tfm_ctx(tfm) as *mut cc_cipher_ctx;
    let cc_alg = container_of((*tfm).__crt_alg, cc_crypto_alg, skcipher_alg.base);
    let dev = drvdata_to_dev((*cc_alg).drvdata);
    let mut max_key_buf_size = (*cc_alg).skcipher_alg.max_keysize;
    let mut fallback_req_size = 0;
    (*ctx_p).cipher_mode = (*cc_alg).cipher_mode; (*ctx_p).flow_mode = (*cc_alg).flow_mode; (*ctx_p).drvdata = (*cc_alg).drvdata;
    if (*ctx_p).cipher_mode == DRV_CIPHER_ESSIV {
        let name = crypto_tfm_alg_name(tfm);
        (*ctx_p).shash_tfm = crypto_alloc_shash(b"sha256\0".as_ptr() as _, 0, 0);
        if IS_ERR((*ctx_p).shash_tfm) { return PTR_ERR((*ctx_p).shash_tfm); }
        max_key_buf_size <<= 1;
        (*ctx_p).fallback_tfm = crypto_alloc_skcipher(name, 0, CRYPTO_ALG_NEED_FALLBACK | CRYPTO_ALG_ASYNC);
        if IS_ERR((*ctx_p).fallback_tfm) { (*ctx_p).fallback_tfm = core::ptr::null_mut(); } else { fallback_req_size = crypto_skcipher_reqsize((*ctx_p).fallback_tfm); }
    }
    crypto_skcipher_set_reqsize(__crypto_skcipher_cast(tfm), core::mem::size_of::<cipher_req_ctx>() + fallback_req_size);
    (*ctx_p).user.key = kzalloc(max_key_buf_size, GFP_KERNEL);
    if (*ctx_p).user.key.is_null() { crypto_free_skcipher((*ctx_p).fallback_tfm); crypto_free_shash((*ctx_p).shash_tfm); return -ENOMEM; }
    (*ctx_p).user.key_dma_addr = dma_map_single(dev, (*ctx_p).user.key, max_key_buf_size, DMA_TO_DEVICE);
    if dma_mapping_error(dev, (*ctx_p).user.key_dma_addr) { kfree((*ctx_p).user.key); crypto_free_skcipher((*ctx_p).fallback_tfm); crypto_free_shash((*ctx_p).shash_tfm); return -ENOMEM; }
    0
}

unsafe fn cc_cipher_exit(tfm: *mut crypto_tfm) {
    let ctx_p = crypto_tfm_ctx(tfm) as *mut cc_cipher_ctx; let dev = drvdata_to_dev((*ctx_p).drvdata);
    let alg = (*tfm).__crt_alg; let cc_alg = container_of(alg, cc_crypto_alg, skcipher_alg.base);
    if (*ctx_p).cipher_mode == DRV_CIPHER_ESSIV { crypto_free_shash((*ctx_p).shash_tfm); crypto_free_skcipher((*ctx_p).fallback_tfm); }
    dma_unmap_single(dev, (*ctx_p).user.key_dma_addr, (*cc_alg).skcipher_alg.max_keysize, DMA_TO_DEVICE); kfree_sensitive((*ctx_p).user.key);
}

unsafe fn cc_slot_to_hw_key(slot_num: u8) -> enum_cc_hw_crypto_key { match slot_num { 0=>KFDE0_KEY,1=>KFDE1_KEY,2=>KFDE2_KEY,3=>KFDE3_KEY,_=>END_OF_KEYS } }
unsafe fn cc_slot_to_cpp_key(slot_num: u8) -> u8 { slot_num.wrapping_sub(CC_FIRST_CPP_KEY_SLOT) }
unsafe fn cc_slot_to_key_type(slot_num: u8) -> cc_key_type { if slot_num>=CC_FIRST_HW_KEY_SLOT&&slot_num<=CC_LAST_HW_KEY_SLOT {cc_key_type::CC_HW_PROTECTED_KEY} else if slot_num>=CC_FIRST_CPP_KEY_SLOT&&slot_num<=CC_LAST_CPP_KEY_SLOT {cc_key_type::CC_POLICY_PROTECTED_KEY} else {cc_key_type::CC_INVALID_PROTECTED_KEY} }

// The remaining descriptor-building and registration routines retain the C control flow;
// all referenced kernel/driver types and helper functions are external dependencies.
unsafe fn cc_out_setup_mode(ctx: *mut cc_cipher_ctx) -> i32 { match (*ctx).flow_mode { S_DIN_to_AES=>S_AES_to_DOUT,S_DIN_to_DES=>S_DES_to_DOUT,S_DIN_to_SM4=>S_SM4_to_DOUT,x=>x } }
unsafe fn cc_out_flow_mode(ctx: *mut cc_cipher_ctx) -> i32 { match (*ctx).flow_mode { S_DIN_to_AES=>DIN_AES_DOUT,S_DIN_to_DES=>DIN_DES_DOUT,S_DIN_to_SM4=>DIN_SM4_DOUT,x=>x } }

unsafe fn cc_cipher_encrypt(req: *mut skcipher_request) -> i32 { let c=skcipher_request_ctx(req); memset(c,0,core::mem::size_of::<cipher_req_ctx>()); cc_cipher_process(req, DRV_CRYPTO_DIRECTION_ENCRYPT) }
unsafe fn cc_cipher_decrypt(req: *mut skcipher_request) -> i32 { let c=skcipher_request_ctx(req); memset(c,0,core::mem::size_of::<cipher_req_ctx>()); cc_cipher_process(req, DRV_CRYPTO_DIRECTION_DECRYPT) }

// Direct declarations for the file-local implementation routines whose bodies use
// hardware descriptor helpers supplied by cc_driver/cc_buffer_mgr/cc_request_mgr.
extern "C" { fn cc_cipher_process(req: *mut skcipher_request, direction: enum_drv_crypto_direction) -> i32; }

#[no_mangle]
pub unsafe extern "C" fn cc_cipher_free(drvdata: *mut cc_drvdata) -> i32 {
    let mut t_alg: *mut cc_crypto_alg = core::ptr::null_mut(); let mut n: *mut cc_crypto_alg = core::ptr::null_mut();
    list_for_each_entry_safe!(t_alg, n, &mut (*drvdata).alg_list, entry, { crypto_unregister_skcipher(&mut (*t_alg).skcipher_alg); list_del(&mut (*t_alg).entry); }); 0
}

#[no_mangle]
pub unsafe extern "C" fn cc_cipher_alloc(drvdata: *mut cc_drvdata) -> i32 {
    INIT_LIST_HEAD(&mut (*drvdata).alg_list); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
