/* SPDX-License-Identifier: GPL-2.0-or-later */
/* AMCC SoC PPC4xx Crypto Driver: security association format. */

pub const AES_IV_SIZE: u32 = 16;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DynamicSaContentsBf { pub raw: u32 }
impl DynamicSaContentsBf {
    pub fn arc4_state_ptr(&self) -> u32 { self.raw & 1 }
    pub fn arc4_ij_ptr(&self) -> u32 { (self.raw >> 1) & 1 }
    pub fn state_ptr(&self) -> u32 { (self.raw >> 2) & 1 }
    pub fn iv3(&self) -> u32 { (self.raw >> 3) & 1 }
    pub fn iv2(&self) -> u32 { (self.raw >> 4) & 1 }
    pub fn iv1(&self) -> u32 { (self.raw >> 5) & 1 }
    pub fn iv0(&self) -> u32 { (self.raw >> 6) & 1 }
    pub fn seq_num_mask3(&self) -> u32 { (self.raw >> 7) & 1 }
    pub fn seq_num_mask2(&self) -> u32 { (self.raw >> 8) & 1 }
    pub fn seq_num_mask1(&self) -> u32 { (self.raw >> 9) & 1 }
    pub fn seq_num_mask0(&self) -> u32 { (self.raw >> 10) & 1 }
    pub fn seq_num1(&self) -> u32 { (self.raw >> 11) & 1 }
    pub fn seq_num0(&self) -> u32 { (self.raw >> 12) & 1 }
    pub fn spi(&self) -> u32 { (self.raw >> 13) & 1 }
    pub fn outer_size(&self) -> u32 { (self.raw >> 14) & 0x1f }
    pub fn inner_size(&self) -> u32 { (self.raw >> 19) & 0x1f }
    pub fn key_size(&self) -> u32 { (self.raw >> 24) & 0xf }
    pub fn cmd_size(&self) -> u32 { (self.raw >> 28) & 0xf }
}
#[repr(C, packed)]
pub union DynamicSaContents { pub bf: DynamicSaContentsBf, pub w: u32 }

pub const DIR_OUTBOUND: u32 = 0; pub const DIR_INBOUND: u32 = 1;
pub const SA_OP_GROUP_BASIC: u32 = 0;
pub const SA_OPCODE_ENCRYPT: u32 = 0; pub const SA_OPCODE_DECRYPT: u32 = 0;
pub const SA_OPCODE_ENCRYPT_HASH: u32 = 1; pub const SA_OPCODE_HASH_DECRYPT: u32 = 1;
pub const SA_OPCODE_HASH: u32 = 3;
pub const SA_CIPHER_ALG_DES: u32 = 0; pub const SA_CIPHER_ALG_3DES: u32 = 1;
pub const SA_CIPHER_ALG_ARC4: u32 = 2; pub const SA_CIPHER_ALG_AES: u32 = 3;
pub const SA_CIPHER_ALG_KASUMI: u32 = 4; pub const SA_CIPHER_ALG_NULL: u32 = 15;
pub const SA_HASH_ALG_MD5: u32 = 0; pub const SA_HASH_ALG_SHA1: u32 = 1;
pub const SA_HASH_ALG_GHASH: u32 = 12; pub const SA_HASH_ALG_CBC_MAC: u32 = 14;
pub const SA_HASH_ALG_NULL: u32 = 15; pub const SA_HASH_ALG_SHA1_DIGEST_SIZE: u32 = 20;
pub const SA_LOAD_HASH_FROM_SA: u32 = 0; pub const SA_LOAD_HASH_FROM_STATE: u32 = 2;
pub const SA_NOT_LOAD_HASH: u32 = 3; pub const SA_LOAD_IV_FROM_SA: u32 = 0;
pub const SA_LOAD_IV_FROM_INPUT: u32 = 1; pub const SA_LOAD_IV_FROM_STATE: u32 = 2;
pub const SA_LOAD_IV_GEN_IV: u32 = 3; pub const SA_PAD_TYPE_CONSTANT: u32 = 2;
pub const SA_PAD_TYPE_ZERO: u32 = 3; pub const SA_PAD_TYPE_TLS: u32 = 5;
pub const SA_PAD_TYPE_DTLS: u32 = 5; pub const SA_NOT_SAVE_HASH: u32 = 0;
pub const SA_SAVE_HASH: u32 = 1; pub const SA_NOT_SAVE_IV: u32 = 0; pub const SA_SAVE_IV: u32 = 1;
pub const SA_HEADER_PROC: u32 = 1; pub const SA_NO_HEADER_PROC: u32 = 0;

#[repr(C, packed)] pub union SaCommand0 { pub bf: u32, pub w: u32 }
pub const CRYPTO_MODE_ECB: u32 = 0; pub const CRYPTO_MODE_CBC: u32 = 1;
pub const CRYPTO_MODE_OFB: u32 = 2; pub const CRYPTO_MODE_CFB: u32 = 3; pub const CRYPTO_MODE_CTR: u32 = 4;
pub const CRYPTO_FEEDBACK_MODE_NO_FB: u32 = 0; pub const CRYPTO_FEEDBACK_MODE_64BIT_OFB: u32 = 0;
pub const CRYPTO_FEEDBACK_MODE_8BIT_CFB: u32 = 1; pub const CRYPTO_FEEDBACK_MODE_1BIT_CFB: u32 = 2;
pub const CRYPTO_FEEDBACK_MODE_128BIT_CFB: u32 = 3;
pub const SA_AES_KEY_LEN_128: u32 = 2; pub const SA_AES_KEY_LEN_192: u32 = 3; pub const SA_AES_KEY_LEN_256: u32 = 4;
pub const SA_REV2: u32 = 1;
pub const SA_HASH_MODE_HASH: u32 = 0; pub const SA_HASH_MODE_HMAC: u32 = 1;
pub const SA_MC_ENABLE: u32 = 0; pub const SA_MC_DISABLE: u32 = 1;
pub const SA_NOT_COPY_HDR: u32 = 0; pub const SA_COPY_HDR: u32 = 1; pub const SA_NOT_COPY_PAD: u32 = 0; pub const SA_COPY_PAD: u32 = 1;
pub const SA_NOT_COPY_PAYLOAD: u32 = 0; pub const SA_COPY_PAYLOAD: u32 = 1; pub const SA_EXTENDED_SN_OFF: u32 = 0; pub const SA_EXTENDED_SN_ON: u32 = 1;
pub const SA_SEQ_MASK_OFF: u32 = 0; pub const SA_SEQ_MASK_ON: u32 = 1;
#[repr(C, packed)] pub union SaCommand1 { pub bf: u32, pub w: u32 }

#[repr(C, packed)] pub struct DynamicSaCtl { pub sa_contents: DynamicSaContents, pub sa_command_0: SaCommand0, pub sa_command_1: SaCommand1 }
#[repr(C, packed)] pub struct SaStateRecord { pub save_iv: [u32;4], pub save_hash_byte_cnt: [u32;2], pub save_digest: [u32;16] }
#[repr(C, packed)] pub struct DynamicSaAes128 { pub ctrl: DynamicSaCtl, pub key: [u32;4], pub iv: [u32;4], pub state_ptr: u32, pub reserved: u32 }
pub const SA_AES128_LEN: usize = core::mem::size_of::<DynamicSaAes128>() / 4; pub const SA_AES128_CONTENTS: u32 = 0x3e000042;
#[repr(C, packed)] pub struct DynamicSaAes192 { pub ctrl: DynamicSaCtl, pub key: [u32;6], pub iv: [u32;4], pub state_ptr: u32, pub reserved: u32 }
pub const SA_AES192_LEN: usize = core::mem::size_of::<DynamicSaAes192>() / 4; pub const SA_AES192_CONTENTS: u32 = 0x3e000062;
#[repr(C, packed)] pub struct DynamicSaAes256 { pub ctrl: DynamicSaCtl, pub key: [u32;8], pub iv: [u32;4], pub state_ptr: u32, pub reserved: u32 }
pub const SA_AES256_LEN: usize = core::mem::size_of::<DynamicSaAes256>() / 4; pub const SA_AES256_CONTENTS: u32 = 0x3e000082; pub const SA_AES_CONTENTS: u32 = 0x3e000002;
#[repr(C, packed)] pub struct DynamicSaAes128Ccm { pub ctrl: DynamicSaCtl, pub key: [u32;4], pub iv: [u32;4], pub state_ptr: u32, pub reserved: u32 }
pub const SA_AES128_CCM_LEN: usize = core::mem::size_of::<DynamicSaAes128Ccm>() / 4; pub const SA_AES128_CCM_CONTENTS: u32 = 0x3e000042; pub const SA_AES_CCM_CONTENTS: u32 = 0x3e000002;
#[repr(C, packed)] pub struct DynamicSaAes128Gcm { pub ctrl: DynamicSaCtl, pub key: [u32;4], pub inner_digest: [u32;4], pub iv: [u32;4], pub state_ptr: u32, pub reserved: u32 }
pub const SA_AES128_GCM_LEN: usize = core::mem::size_of::<DynamicSaAes128Gcm>() / 4; pub const SA_AES128_GCM_CONTENTS: u32 = 0x3e000442; pub const SA_AES_GCM_CONTENTS: u32 = 0x3e000402;
#[repr(C, packed)] pub struct DynamicSaHash160 { pub ctrl: DynamicSaCtl, pub inner_digest: [u32;5], pub outer_digest: [u32;5], pub state_ptr: u32, pub reserved: u32 }
pub const SA_HASH160_LEN: usize = core::mem::size_of::<DynamicSaHash160>() / 4; pub const SA_HASH160_CONTENTS: u32 = 0x2000a502;

pub unsafe fn get_dynamic_sa_offset_state_ptr_field(cts: *mut DynamicSaCtl) -> usize {
    let c = &(*cts).sa_contents;
    let bf = c.bf;
    core::mem::size_of::<DynamicSaCtl>() + (bf.key_size() + bf.inner_size() + bf.outer_size() + bf.spi() + bf.seq_num0() + bf.seq_num1() + bf.seq_num_mask0() + bf.seq_num_mask1() + bf.seq_num_mask2() + bf.seq_num_mask3() + bf.iv0() + bf.iv1() + bf.iv2() + bf.iv3()) as usize * 4
}
pub unsafe fn get_dynamic_sa_key_field(cts: *mut DynamicSaCtl) -> *mut u32 { (cts as *mut u8).add(core::mem::size_of::<DynamicSaCtl>()) as *mut u32 }
pub unsafe fn get_dynamic_sa_inner_digest(cts: *mut DynamicSaCtl) -> *mut u32 { (cts as *mut u8).add(core::mem::size_of::<DynamicSaCtl>() + (*cts).sa_contents.bf.key_size() as usize * 4) as *mut u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
