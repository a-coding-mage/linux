/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2019 HiSilicon Limited. */

pub const SEC_AIV_SIZE: u32 = 12;
pub const SEC_IV_SIZE: u32 = 24;
pub const SEC_MAX_KEY_SIZE: u32 = 64;
pub const SEC_MAX_AKEY_SIZE: u32 = 128;
pub const SEC_COMM_SCENE: u32 = 0;
pub const SEC_MIN_BLOCK_SZ: u32 = 1;

#[repr(i32)]
pub enum sec_calg { SEC_CALG_3DES = 0x1, SEC_CALG_AES = 0x2, SEC_CALG_SM4 = 0x3 }
#[repr(i32)]
pub enum sec_hash_alg { SEC_A_HMAC_SHA1 = 0x10, SEC_A_HMAC_SHA256 = 0x11, SEC_A_HMAC_SHA512 = 0x15 }
#[repr(i32)]
pub enum sec_cmode { SEC_CMODE_ECB = 0x0, SEC_CMODE_CBC = 0x1, SEC_CMODE_CTR = 0x4, SEC_CMODE_CCM = 0x5, SEC_CMODE_GCM = 0x6, SEC_CMODE_XTS = 0x7 }
#[repr(i32)]
pub enum sec_ckey_type { SEC_CKEY_128BIT = 0x0, SEC_CKEY_192BIT = 0x1, SEC_CKEY_256BIT = 0x2, SEC_CKEY_3DES_3KEY = 0x1, SEC_CKEY_3DES_2KEY = 0x3 }
#[repr(i32)]
pub enum sec_bd_type { SEC_BD_TYPE1 = 0x1, SEC_BD_TYPE2 = 0x2, SEC_BD_TYPE3 = 0x3 }
#[repr(i32)]
pub enum sec_auth { SEC_NO_AUTH = 0x0, SEC_AUTH_TYPE1 = 0x1, SEC_AUTH_TYPE2 = 0x2 }
#[repr(i32)]
pub enum sec_cipher_dir { SEC_CIPHER_ENC = 0x1, SEC_CIPHER_DEC = 0x2 }
#[repr(i32)]
pub enum sec_addr_type { SEC_PBUF = 0x0, SEC_SGL = 0x1, SEC_PRP = 0x2 }

pub const AUTHPAD_PAD: u32 = 0;
pub const AUTHPAD_NOPAD: u32 = 1;
pub const AIGEN_GEN: u32 = 0;
pub const AIGEN_NOGEN: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd_status { pub tag: u64, pub done: u8, pub err_type: u8, pub flag: u16, pub icv: u16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_sqe_type2 {
    pub mac_key_alg: __le32, pub icvw_kmode: __le16, pub c_alg: __u8, pub rsvd4: __u8,
    pub alen_ivllen: __le32, pub clen_ivhlen: __le32, pub auth_src_offset: __le16,
    pub cipher_src_offset: __le16, pub cs_ip_header_offset: __le16, pub cs_udp_header_offset: __le16,
    pub pass_word_len: __le16, pub dk_len: __le16, pub salt3: __u8, pub salt2: __u8, pub salt1: __u8, pub salt0: __u8,
    pub tag: __le16, pub rsvd5: __le16, pub cph_pad: __le16, pub c_pad_len_field: __le16,
    pub long_a_data_len: __le64, pub a_ivin_addr: __le64, pub a_key_addr: __le64, pub mac_addr: __le64,
    pub c_ivin_addr: __le64, pub c_key_addr: __le64, pub data_src_addr: __le64, pub data_dst_addr: __le64,
    pub done_flag: __le16, pub error_type: __u8, pub warning_type: __u8, pub mac_i3: __u8, pub mac_i2: __u8,
    pub mac_i1: __u8, pub mac_i0: __u8, pub check_sum_i: __le16, pub tls_pad_len_i: __u8, pub rsvd12: __u8, pub counter: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_sqe { pub type_cipher_auth: __u8, pub sds_sa_type: __u8, pub sdm_addr_type: __u8, pub rsvd0: __u8, pub huk_key_ci: __u8, pub ai_apd_cs: __u8, pub rca_key_frm: __u8, pub iv_tls_ld: __u8, pub type2: sec_sqe_type2 }

#[repr(C, packed(1))]
#[derive(Copy, Clone)]
pub struct bd3_auth_ivin { pub a_ivin_addr: __le64, pub rsvd0: __le32, pub rsvd1: __le32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd3_skip_data { pub rsvd0: __le32, pub gran_num: __le32, pub src_skip_data_len: __le32, pub dst_skip_data_len: __le32 }
#[repr(C, packed(1))]
#[derive(Copy, Clone)]
pub struct bd3_stream_scene { pub c_ivin_addr: __le64, pub long_a_data_len: __le64, pub stream_auth_pad: __u8, pub plaintext_type: __u8, pub pad_len_1p3: __le16 }
#[repr(C, packed(1))]
#[derive(Copy, Clone)]
pub struct bd3_no_scene { pub c_ivin_addr: __le64, pub rsvd0: __le32, pub rsvd1: __le32, pub rsvd2: __le32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd3_check_sum { pub rsvd0: __u8, pub hac_sva_status: __u8, pub check_sum_i: __le16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd3_tls_type_back { pub tls_1p3_type_back: __u8, pub hac_sva_status: __u8, pub pad_len_1p3_back: __le16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub union sec_sqe3_auth_union { pub auth_ivin: bd3_auth_ivin, pub skip_data: bd3_skip_data }
#[repr(C)]
#[derive(Copy, Clone)]
pub union sec_sqe3_scene_union { pub stream_scene: bd3_stream_scene, pub no_scene: bd3_no_scene }
#[repr(C)]
#[derive(Copy, Clone)]
pub union sec_sqe3_mac_union { pub mac_i: __le32, pub kek_key_addr_l: __le32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub union sec_sqe3_key_union { pub kek_key_addr_h: __le32, pub check_sum: bd3_check_sum, pub tls_type_back: bd3_tls_type_back }

#[repr(C, packed(1))]
#[derive(Copy, Clone)]
pub struct sec_sqe3 {
    pub bd_param: __le32, pub c_icv_key: __le16, pub c_mode_alg: __u8, pub huk_iv_seq: __u8,
    pub tag: __le64, pub data_src_addr: __le64, pub a_key_addr: __le64, pub auth_ivin_or_skip_data: sec_sqe3_auth_union,
    pub c_key_addr: __le64, pub auth_mac_key: __le32, pub salt: __le32, pub auth_src_offset: __le16,
    pub cipher_src_offset: __le16, pub a_len_key: __le32, pub c_len_ivin: __le32, pub data_dst_addr: __le64,
    pub mac_addr: __le64, pub stream_scene_or_no_scene: sec_sqe3_scene_union, pub done_flag: __le16,
    pub error_type: __u8, pub warning_type: __u8, pub mac_or_kek_l: sec_sqe3_mac_union, pub kek_h_or_status: sec_sqe3_key_union, pub counter: __le32,
}

extern "C" {
    pub fn sec_register_to_crypto(qm: *mut hisi_qm) -> i32;
    pub fn sec_unregister_from_crypto(qm: *mut hisi_qm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
