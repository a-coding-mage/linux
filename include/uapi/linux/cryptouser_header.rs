/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Crypto user configuration API.
 *
 * Translated from the Linux UAPI header.  The __u32 and __u64 types are
 * supplied by the corresponding Linux type definitions.
 */

use core::ffi::c_char;

pub const CRYPTO_MSG_BASE: u32 = 0x10;
pub const CRYPTO_MSG_NEWALG: u32 = 0x10;
pub const CRYPTO_MSG_DELALG: u32 = CRYPTO_MSG_NEWALG + 1;
pub const CRYPTO_MSG_UPDATEALG: u32 = CRYPTO_MSG_DELALG + 1;
pub const CRYPTO_MSG_GETALG: u32 = CRYPTO_MSG_UPDATEALG + 1;
pub const CRYPTO_MSG_DELRNG: u32 = CRYPTO_MSG_GETALG + 1;
pub const CRYPTO_MSG_GETSTAT: u32 = CRYPTO_MSG_DELRNG + 1; // No longer supported, do not use.
pub const __CRYPTO_MSG_MAX: u32 = CRYPTO_MSG_GETSTAT + 1;
pub const CRYPTO_MSG_MAX: u32 = __CRYPTO_MSG_MAX - 1;
pub const CRYPTO_NR_MSGTYPES: u32 = CRYPTO_MSG_MAX + 1 - CRYPTO_MSG_BASE;

pub const CRYPTO_MAX_NAME: usize = 64;

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum crypto_attr_type_t {
    CRYPTOCFGA_UNSPEC = 0,
    CRYPTOCFGA_PRIORITY_VAL,
    CRYPTOCFGA_REPORT_LARVAL,
    CRYPTOCFGA_REPORT_HASH,
    CRYPTOCFGA_REPORT_BLKCIPHER,
    CRYPTOCFGA_REPORT_AEAD,
    CRYPTOCFGA_REPORT_COMPRESS,
    CRYPTOCFGA_REPORT_RNG,
    CRYPTOCFGA_REPORT_CIPHER,
    CRYPTOCFGA_REPORT_AKCIPHER,
    CRYPTOCFGA_REPORT_KPP,
    CRYPTOCFGA_REPORT_ACOMP,
    CRYPTOCFGA_STAT_LARVAL,
    CRYPTOCFGA_STAT_HASH,
    CRYPTOCFGA_STAT_BLKCIPHER,
    CRYPTOCFGA_STAT_AEAD,
    CRYPTOCFGA_STAT_COMPRESS,
    CRYPTOCFGA_STAT_RNG,
    CRYPTOCFGA_STAT_CIPHER,
    CRYPTOCFGA_STAT_AKCIPHER,
    CRYPTOCFGA_STAT_KPP,
    CRYPTOCFGA_STAT_ACOMP,
    CRYPTOCFGA_REPORT_SIG,
    __CRYPTOCFGA_MAX,
}

pub const CRYPTOCFGA_MAX: u32 = crypto_attr_type_t::__CRYPTOCFGA_MAX as u32 - 1;

#[repr(C)]
pub struct crypto_user_alg {
    pub cru_name: [c_char; CRYPTO_MAX_NAME],
    pub cru_driver_name: [c_char; CRYPTO_MAX_NAME],
    pub cru_module_name: [c_char; CRYPTO_MAX_NAME],
    pub cru_type: __u32,
    pub cru_mask: __u32,
    pub cru_refcnt: __u32,
    pub cru_flags: __u32,
}

#[repr(C)]
pub struct crypto_stat_aead { pub r#type: [c_char; CRYPTO_MAX_NAME], pub stat_encrypt_cnt: __u64, pub stat_encrypt_tlen: __u64, pub stat_decrypt_cnt: __u64, pub stat_decrypt_tlen: __u64, pub stat_err_cnt: __u64 }
#[repr(C)]
pub struct crypto_stat_akcipher { pub r#type: [c_char; CRYPTO_MAX_NAME], pub stat_encrypt_cnt: __u64, pub stat_encrypt_tlen: __u64, pub stat_decrypt_cnt: __u64, pub stat_decrypt_tlen: __u64, pub stat_verify_cnt: __u64, pub stat_sign_cnt: __u64, pub stat_err_cnt: __u64 }
#[repr(C)]
pub struct crypto_stat_cipher { pub r#type: [c_char; CRYPTO_MAX_NAME], pub stat_encrypt_cnt: __u64, pub stat_encrypt_tlen: __u64, pub stat_decrypt_cnt: __u64, pub stat_decrypt_tlen: __u64, pub stat_err_cnt: __u64 }
#[repr(C)]
pub struct crypto_stat_compress { pub r#type: [c_char; CRYPTO_MAX_NAME], pub stat_compress_cnt: __u64, pub stat_compress_tlen: __u64, pub stat_decompress_cnt: __u64, pub stat_decompress_tlen: __u64, pub stat_err_cnt: __u64 }
#[repr(C)]
pub struct crypto_stat_hash { pub r#type: [c_char; CRYPTO_MAX_NAME], pub stat_hash_cnt: __u64, pub stat_hash_tlen: __u64, pub stat_err_cnt: __u64 }
#[repr(C)]
pub struct crypto_stat_kpp { pub r#type: [c_char; CRYPTO_MAX_NAME], pub stat_setsecret_cnt: __u64, pub stat_generate_public_key_cnt: __u64, pub stat_compute_shared_secret_cnt: __u64, pub stat_err_cnt: __u64 }
#[repr(C)]
pub struct crypto_stat_rng { pub r#type: [c_char; CRYPTO_MAX_NAME], pub stat_generate_cnt: __u64, pub stat_generate_tlen: __u64, pub stat_seed_cnt: __u64, pub stat_err_cnt: __u64 }
#[repr(C)]
pub struct crypto_stat_larval { pub r#type: [c_char; CRYPTO_MAX_NAME] }

#[repr(C)]
pub struct crypto_report_larval { pub r#type: [c_char; CRYPTO_MAX_NAME] }
#[repr(C)]
pub struct crypto_report_hash { pub r#type: [c_char; CRYPTO_MAX_NAME], pub blocksize: core::ffi::c_uint, pub digestsize: core::ffi::c_uint }
#[repr(C)]
pub struct crypto_report_cipher { pub r#type: [c_char; CRYPTO_MAX_NAME], pub blocksize: core::ffi::c_uint, pub min_keysize: core::ffi::c_uint, pub max_keysize: core::ffi::c_uint }
#[repr(C)]
pub struct crypto_report_blkcipher { pub r#type: [c_char; CRYPTO_MAX_NAME], pub geniv: [c_char; CRYPTO_MAX_NAME], pub blocksize: core::ffi::c_uint, pub min_keysize: core::ffi::c_uint, pub max_keysize: core::ffi::c_uint, pub ivsize: core::ffi::c_uint }
#[repr(C)]
pub struct crypto_report_aead { pub r#type: [c_char; CRYPTO_MAX_NAME], pub geniv: [c_char; CRYPTO_MAX_NAME], pub blocksize: core::ffi::c_uint, pub maxauthsize: core::ffi::c_uint, pub ivsize: core::ffi::c_uint }
#[repr(C)] pub struct crypto_report_comp { pub r#type: [c_char; CRYPTO_MAX_NAME] }
#[repr(C)] pub struct crypto_report_rng { pub r#type: [c_char; CRYPTO_MAX_NAME], pub seedsize: core::ffi::c_uint }
#[repr(C)] pub struct crypto_report_akcipher { pub r#type: [c_char; CRYPTO_MAX_NAME] }
#[repr(C)] pub struct crypto_report_kpp { pub r#type: [c_char; CRYPTO_MAX_NAME] }
#[repr(C)] pub struct crypto_report_acomp { pub r#type: [c_char; CRYPTO_MAX_NAME] }
#[repr(C)] pub struct crypto_report_sig { pub r#type: [c_char; CRYPTO_MAX_NAME] }

pub const CRYPTO_REPORT_MAXSIZE: usize = core::mem::size_of::<crypto_user_alg>() + core::mem::size_of::<crypto_report_blkcipher>();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
