/*
 * This file is part of the Chelsio T6 Crypto driver for Linux.
 *
 * Copyright (c) 2003-2016 Chelsio Communications, Inc. All rights reserved.
 */

// C header guard: __CHCR_ALGO_H__

pub const KEY_CONTEXT_CTX_LEN_S: u32 = 24;
pub const KEY_CONTEXT_CTX_LEN_M: u32 = 0xff;
#[macro_export] macro_rules! KEY_CONTEXT_CTX_LEN_V { ($x:expr) => { ($x) << KEY_CONTEXT_CTX_LEN_S }; }
#[macro_export] macro_rules! KEY_CONTEXT_CTX_LEN_G { ($x:expr) => { (($x) >> KEY_CONTEXT_CTX_LEN_S) & KEY_CONTEXT_CTX_LEN_M }; }
pub const KEY_CONTEXT_DUAL_CK_S: u32 = 12;
pub const KEY_CONTEXT_DUAL_CK_M: u32 = 0x1;
#[macro_export] macro_rules! KEY_CONTEXT_DUAL_CK_V { ($x:expr) => { ($x) << KEY_CONTEXT_DUAL_CK_S }; }
#[macro_export] macro_rules! KEY_CONTEXT_DUAL_CK_G { ($x:expr) => { (($x) >> KEY_CONTEXT_DUAL_CK_S) & KEY_CONTEXT_DUAL_CK_M }; }
pub const KEY_CONTEXT_DUAL_CK_F: u32 = KEY_CONTEXT_DUAL_CK_V!(1u32);
pub const KEY_CONTEXT_SALT_PRESENT_S: u32 = 10;
pub const KEY_CONTEXT_SALT_PRESENT_M: u32 = 0x1;
#[macro_export] macro_rules! KEY_CONTEXT_SALT_PRESENT_V { ($x:expr) => { ($x) << KEY_CONTEXT_SALT_PRESENT_S }; }
#[macro_export] macro_rules! KEY_CONTEXT_SALT_PRESENT_G { ($x:expr) => { (($x) >> KEY_CONTEXT_SALT_PRESENT_S) & KEY_CONTEXT_SALT_PRESENT_M }; }
pub const KEY_CONTEXT_SALT_PRESENT_F: u32 = KEY_CONTEXT_SALT_PRESENT_V!(1u32);
pub const KEY_CONTEXT_VALID_S: u32 = 0;
pub const KEY_CONTEXT_VALID_M: u32 = 0x1;
#[macro_export] macro_rules! KEY_CONTEXT_VALID_V { ($x:expr) => { ($x) << KEY_CONTEXT_VALID_S }; }
#[macro_export] macro_rules! KEY_CONTEXT_VALID_G { ($x:expr) => { (($x) >> KEY_CONTEXT_VALID_S) & KEY_CONTEXT_VALID_M }; }
pub const KEY_CONTEXT_VALID_F: u32 = KEY_CONTEXT_VALID_V!(1u32);
pub const KEY_CONTEXT_CK_SIZE_S: u32 = 6;
pub const KEY_CONTEXT_CK_SIZE_M: u32 = 0xf;
#[macro_export] macro_rules! KEY_CONTEXT_CK_SIZE_V { ($x:expr) => { ($x) << KEY_CONTEXT_CK_SIZE_S }; }
#[macro_export] macro_rules! KEY_CONTEXT_CK_SIZE_G { ($x:expr) => { (($x) >> KEY_CONTEXT_CK_SIZE_S) & KEY_CONTEXT_CK_SIZE_M }; }
pub const KEY_CONTEXT_MK_SIZE_S: u32 = 2;
pub const KEY_CONTEXT_MK_SIZE_M: u32 = 0xf;
#[macro_export] macro_rules! KEY_CONTEXT_MK_SIZE_V { ($x:expr) => { ($x) << KEY_CONTEXT_MK_SIZE_S }; }
#[macro_export] macro_rules! KEY_CONTEXT_MK_SIZE_G { ($x:expr) => { (($x) >> KEY_CONTEXT_MK_SIZE_S) & KEY_CONTEXT_MK_SIZE_M }; }
pub const KEY_CONTEXT_OPAD_PRESENT_S: u32 = 11;
pub const KEY_CONTEXT_OPAD_PRESENT_M: u32 = 0x1;
#[macro_export] macro_rules! KEY_CONTEXT_OPAD_PRESENT_V { ($x:expr) => { ($x) << KEY_CONTEXT_OPAD_PRESENT_S }; }
#[macro_export] macro_rules! KEY_CONTEXT_OPAD_PRESENT_G { ($x:expr) => { (($x) >> KEY_CONTEXT_OPAD_PRESENT_S) & KEY_CONTEXT_OPAD_PRESENT_M }; }
pub const KEY_CONTEXT_OPAD_PRESENT_F: u32 = KEY_CONTEXT_OPAD_PRESENT_V!(1u32);

pub const CHCR_HASH_MAX_DIGEST_SIZE: usize = 64;
pub const CHCR_MAX_SHA_DIGEST_SIZE: usize = 64;
pub const IPSEC_TRUNCATED_ICV_SIZE: usize = 12;
pub const TLS_TRUNCATED_HMAC_SIZE: usize = 10;
pub const CBCMAC_DIGEST_SIZE: usize = 16;
pub const MAX_HASH_NAME: usize = 20;
pub const SHA1_INIT_STATE_5X4B: usize = 5;
pub const SHA256_INIT_STATE_8X4B: usize = 8;
pub const SHA512_INIT_STATE_8X8B: usize = 8;
pub const SHA1_INIT_STATE: usize = SHA1_INIT_STATE_5X4B;
pub const SHA224_INIT_STATE: usize = SHA256_INIT_STATE_8X4B;
pub const SHA256_INIT_STATE: usize = SHA256_INIT_STATE_8X4B;
pub const SHA384_INIT_STATE: usize = SHA512_INIT_STATE_8X8B;
pub const SHA512_INIT_STATE: usize = SHA512_INIT_STATE_8X8B;
pub const DUMMY_BYTES: usize = 16;
pub const IPAD_DATA: u32 = 0x36363636;
pub const OPAD_DATA: u32 = 0x5c5c5c5c;

#[macro_export] macro_rules! TRANSHDR_SIZE { ($kctx_len:expr) => { core::mem::size_of::<chcr_wr>() + ($kctx_len) }; }
#[macro_export] macro_rules! CIPHER_TRANSHDR_SIZE { ($kctx_len:expr, $sge_pairs:expr) => { TRANSHDR_SIZE!($kctx_len) + ($sge_pairs) + core::mem::size_of::<cpl_rx_phys_dsgl>() + AES_BLOCK_SIZE }; }
#[macro_export] macro_rules! HASH_TRANSHDR_SIZE { ($kctx_len:expr) => { TRANSHDR_SIZE!($kctx_len) + DUMMY_BYTES }; }
#[macro_export] macro_rules! FILL_SEC_CPL_OP_IVINSR { ($id:expr,$len:expr,$ofst:expr) => { htonl(CPL_TX_SEC_PDU_OPCODE_V!(CPL_TX_SEC_PDU) | CPL_TX_SEC_PDU_RXCHID_V!($id) | CPL_TX_SEC_PDU_ACKFOLLOWS_V!(0) | CPL_TX_SEC_PDU_ULPTXLPBK_V!(1) | CPL_TX_SEC_PDU_CPLLEN_V!($len) | CPL_TX_SEC_PDU_PLACEHOLDER_V!(0) | CPL_TX_SEC_PDU_IVINSRTOFST_V!($ofst)) }; }
#[macro_export] macro_rules! FILL_SEC_CPL_CIPHERSTOP_HI { ($a_start:expr,$a_stop:expr,$c_start:expr,$c_stop_hi:expr) => { htonl(CPL_TX_SEC_PDU_AADSTART_V!($a_start)|CPL_TX_SEC_PDU_AADSTOP_V!($a_stop)|CPL_TX_SEC_PDU_CIPHERSTART_V!($c_start)|CPL_TX_SEC_PDU_CIPHERSTOP_HI_V!($c_stop_hi)) }; }
#[macro_export] macro_rules! FILL_SEC_CPL_AUTHINSERT { ($c_stop_lo:expr,$a_start:expr,$a_stop:expr,$a_inst:expr) => { htonl(CPL_TX_SEC_PDU_CIPHERSTOP_LO_V!($c_stop_lo)|CPL_TX_SEC_PDU_AUTHSTART_V!($a_start)|CPL_TX_SEC_PDU_AUTHSTOP_V!($a_stop)|CPL_TX_SEC_PDU_AUTHINSERT_V!($a_inst)) }; }
#[macro_export] macro_rules! FILL_KEY_CTX_HDR { ($ck:expr,$mk:expr,$d:expr,$op:expr,$len:expr) => { htonl(KEY_CONTEXT_VALID_V!(1)|KEY_CONTEXT_CK_SIZE_V!($ck)|KEY_CONTEXT_MK_SIZE_V!($mk)|KEY_CONTEXT_DUAL_CK_V!($d)|KEY_CONTEXT_OPAD_PRESENT_V!($op)|KEY_CONTEXT_SALT_PRESENT_V!(1)|KEY_CONTEXT_CTX_LEN_V!($len)) }; }
#[macro_export] macro_rules! FILL_KEY_CRX_HDR { ($ck:expr,$mk:expr,$d:expr,$op:expr,$len:expr) => { htonl(TLS_KEYCTX_RXMK_SIZE_V!($mk)|TLS_KEYCTX_RXCK_SIZE_V!($ck)|TLS_KEYCTX_RX_VALID_V!(1)|TLS_KEYCTX_RX_SEQCTR_V!(3)|TLS_KEYCTX_RXAUTH_MODE_V!(4)|TLS_KEYCTX_RXCIPH_MODE_V!(2)|TLS_KEYCTX_RXFLIT_CNT_V!($len)) }; }
#[macro_export] macro_rules! KEYCTX_ALIGN_PAD { ($bs:expr) => { if ($bs) == SHA1_DIGEST_SIZE { 12 } else { 0 } }; }
#[macro_export] macro_rules! CIP_SPACE_LEFT { ($len:expr) => { SGE_MAX_WR_LEN - CIP_WR_MIN_LEN - ($len) }; }
#[macro_export] macro_rules! HASH_SPACE_LEFT { ($len:expr) => { SGE_MAX_WR_LEN - HASH_WR_MIN_LEN - ($len) }; }
pub const MAX_NK: usize = 8; pub const MAX_DSGL_ENT: usize = 32;
pub const MIN_AUTH_SG: usize = 1; pub const MIN_GCM_SG: usize = 1;
pub const MIN_DIGEST_SG: usize = 1; pub const MIN_CCM_SG: usize = 1;

// External types and macros referenced by this header are supplied by dependencies.
#[repr(C)] pub struct algo_param { pub auth_mode: u32, pub mk_size: u32, pub result_size: u32 }
#[repr(C)] pub struct hash_wr_param { pub alg_prm: algo_param, pub opad_needed: u32, pub more: u32, pub last: u32, pub kctx_len: u32, pub sg_len: u32, pub bfr_len: u32, pub hash_size: u32, pub scmd1: u64 }
#[repr(C)] pub struct cipher_wr_param { pub req: *mut skcipher_request, pub iv: *mut i8, pub bytes: i32, pub qid: u16 }
extern "C" { type skcipher_request; }

pub const AES_KEYLENGTH_128BIT: i32 = 128;
pub const AES_KEYLENGTH_192BIT: i32 = 192;
pub const AES_KEYLENGTH_256BIT: i32 = 256;
pub const KEYLENGTH_3BYTES: i32 = 3;
pub const KEYLENGTH_4BYTES: i32 = 4;
pub const KEYLENGTH_6BYTES: i32 = 6;
pub const KEYLENGTH_8BYTES: i32 = 8;
pub const NUMBER_OF_ROUNDS_10: i32 = 10;
pub const NUMBER_OF_ROUNDS_12: i32 = 12;
pub const NUMBER_OF_ROUNDS_14: i32 = 14;
pub const ICV_4: i32 = 4; pub const ICV_6: i32 = 6; pub const ICV_8: i32 = 8;
pub const ICV_10: i32 = 10; pub const ICV_12: i32 = 12; pub const ICV_13: i32 = 13;
pub const ICV_14: i32 = 14; pub const ICV_15: i32 = 15; pub const ICV_16: i32 = 16;

#[repr(C)] pub struct phys_sge_pairs { pub len: [u16; 8], pub addr: [u64; 8] }

pub static CHCR_SHA1_INIT: [u32; 5] = [SHA1_H0, SHA1_H1, SHA1_H2, SHA1_H3, SHA1_H4];
pub static CHCR_SHA224_INIT: [u32; 8] = [SHA224_H0, SHA224_H1, SHA224_H2, SHA224_H3, SHA224_H4, SHA224_H5, SHA224_H6, SHA224_H7];
pub static CHCR_SHA256_INIT: [u32; 8] = [SHA256_H0, SHA256_H1, SHA256_H2, SHA256_H3, SHA256_H4, SHA256_H5, SHA256_H6, SHA256_H7];
pub static CHCR_SHA384_INIT: [u64; 8] = [SHA384_H0, SHA384_H1, SHA384_H2, SHA384_H3, SHA384_H4, SHA384_H5, SHA384_H6, SHA384_H7];
pub static CHCR_SHA512_INIT: [u64; 8] = [SHA512_H0, SHA512_H1, SHA512_H2, SHA512_H3, SHA512_H4, SHA512_H5, SHA512_H6, SHA512_H7];

pub unsafe fn copy_hash_init_values(key: *mut i8, digestsize: i32) {
    let dkey = key as *mut u32;
    let ldkey = key as *mut u64;
    match digestsize {
        SHA1_DIGEST_SIZE => for i in 0..SHA1_INIT_STATE { *dkey.add(i) = u32::from_be(CHCR_SHA1_INIT[i]); },
        SHA224_DIGEST_SIZE => for i in 0..SHA224_INIT_STATE { *dkey.add(i) = u32::from_be(CHCR_SHA224_INIT[i]); },
        SHA256_DIGEST_SIZE => for i in 0..SHA256_INIT_STATE { *dkey.add(i) = u32::from_be(CHCR_SHA256_INIT[i]); },
        SHA384_DIGEST_SIZE => for i in 0..SHA384_INIT_STATE { *ldkey.add(i) = u64::from_be(CHCR_SHA384_INIT[i]); },
        SHA512_DIGEST_SIZE => for i in 0..SHA512_INIT_STATE { *ldkey.add(i) = u64::from_be(CHCR_SHA512_INIT[i]); },
        _ => {}
    }
}

pub const PHYSDSGL_MAX_LEN_SIZE: u16 = 16;
pub fn get_space_for_phys_dsgl(sgl_entr: u32) -> u16 { ((sgl_entr >> 3) + if sgl_entr % 8 != 0 { 1 } else { 0 }) as u16 * PHYSDSGL_MAX_LEN_SIZE + ((sgl_entr << 3) + if sgl_entr % 2 != 0 { 8 } else { 0 }) as u16 }

pub static AES_SBOX: [u8; 256] = [99,124,119,123,242,107,111,197,48,1,103,43,254,215,171,118,202,130,201,125,250,89,71,240,173,212,162,175,156,164,114,192,183,253,147,38,54,63,247,204,52,165,229,241,113,216,49,21,4,199,35,195,24,150,5,154,7,18,128,226,235,39,178,117,9,131,44,26,27,110,90,160,82,59,214,179,41,227,47,132,83,209,0,237,32,252,177,91,106,203,190,57,74,76,88,207,208,239,170,251,67,77,51,133,69,249,2,127,80,60,159,168,81,163,64,143,146,157,56,245,188,182,218,33,16,255,243,210,205,12,19,236,95,151,68,23,196,167,126,61,100,93,25,115,96,129,79,220,34,42,144,136,70,238,184,20,222,94,11,219,224,50,58,10,73,6,36,92,194,211,172,98,145,149,228,121,231,200,55,109,141,213,78,169,108,86,244,234,101,122,174,8,186,120,37,46,28,166,180,198,232,221,116,31,75,189,139,138,112,62,181,102,72,3,246,14,97,53,87,185,134,193,29,158,225,248,152,17,105,217,142,148,155,30,135,233,206,85,40,223,140,161,137,13,191,230,66,104,65,153,45,15,176,84,187,22];

pub unsafe fn aes_ks_subword(w: u32) -> u32 {
    let mut bytes = w.to_ne_bytes();
    for b in &mut bytes { *b = AES_SBOX[*b as usize]; }
    u32::from_ne_bytes(bytes)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
