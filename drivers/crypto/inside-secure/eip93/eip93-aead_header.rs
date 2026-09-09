/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2019 - 2021
 *
 * Richard van Schagen <vschagen@icloud.com>
 * Christian Marangi <ansuelsmth@gmail.com>
 */

// C header guard: _EIP93_AEAD_H_

extern "C" {
    pub static mut eip93_alg_authenc_hmac_md5_cbc_aes: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha1_cbc_aes: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha224_cbc_aes: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha256_cbc_aes: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_md5_ctr_aes: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha1_ctr_aes: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha224_ctr_aes: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha256_ctr_aes: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_md5_rfc3686_aes: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha1_rfc3686_aes: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha224_rfc3686_aes: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha256_rfc3686_aes: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_md5_cbc_des: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha1_cbc_des: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha224_cbc_des: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha256_cbc_des: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_md5_cbc_des3_ede: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha1_cbc_des3_ede: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha224_cbc_des3_ede: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha256_cbc_des3_ede: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_md5_ecb_null: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha1_ecb_null: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha224_ecb_null: eip93_alg_template;
    pub static mut eip93_alg_authenc_hmac_sha256_ecb_null: eip93_alg_template;

    pub fn eip93_aead_handle_result(async_: *mut crypto_async_request, err: ::std::os::raw::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
