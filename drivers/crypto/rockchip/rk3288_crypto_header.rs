/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

macro_rules! _SBF { ($v:expr, $f:expr) => { ($v) << ($f) }; }

pub const RK_CRYPTO_INTSTS: u32 = 0x0000;
pub const RK_CRYPTO_PKA_DONE_INT: u32 = 1 << 5;
pub const RK_CRYPTO_HASH_DONE_INT: u32 = 1 << 4;
pub const RK_CRYPTO_HRDMA_ERR_INT: u32 = 1 << 3;
pub const RK_CRYPTO_HRDMA_DONE_INT: u32 = 1 << 2;
pub const RK_CRYPTO_BCDMA_ERR_INT: u32 = 1 << 1;
pub const RK_CRYPTO_BCDMA_DONE_INT: u32 = 1 << 0;

pub const RK_CRYPTO_INTENA: u32 = 0x0004;
pub const RK_CRYPTO_PKA_DONE_ENA: u32 = 1 << 5;
pub const RK_CRYPTO_HASH_DONE_ENA: u32 = 1 << 4;
pub const RK_CRYPTO_HRDMA_ERR_ENA: u32 = 1 << 3;
pub const RK_CRYPTO_HRDMA_DONE_ENA: u32 = 1 << 2;
pub const RK_CRYPTO_BCDMA_ERR_ENA: u32 = 1 << 1;
pub const RK_CRYPTO_BCDMA_DONE_ENA: u32 = 1 << 0;

pub const RK_CRYPTO_CTRL: u32 = 0x0008;
pub const RK_CRYPTO_WRITE_MASK: u32 = 0xffff << 16;
pub const RK_CRYPTO_TRNG_FLUSH: u32 = 1 << 9;
pub const RK_CRYPTO_TRNG_START: u32 = 1 << 8;
pub const RK_CRYPTO_PKA_FLUSH: u32 = 1 << 7;
pub const RK_CRYPTO_HASH_FLUSH: u32 = 1 << 6;
pub const RK_CRYPTO_BLOCK_FLUSH: u32 = 1 << 5;
pub const RK_CRYPTO_PKA_START: u32 = 1 << 4;
pub const RK_CRYPTO_HASH_START: u32 = 1 << 3;
pub const RK_CRYPTO_BLOCK_START: u32 = 1 << 2;
pub const RK_CRYPTO_TDES_START: u32 = 1 << 1;
pub const RK_CRYPTO_AES_START: u32 = 1 << 0;

pub const RK_CRYPTO_CONF: u32 = 0x000c;
pub const RK_CRYPTO_HR_ADDR_MODE: u32 = 1 << 8;
pub const RK_CRYPTO_BT_ADDR_MODE: u32 = 1 << 7;
pub const RK_CRYPTO_BR_ADDR_MODE: u32 = 1 << 6;
pub const RK_CRYPTO_BYTESWAP_HRFIFO: u32 = 1 << 5;
pub const RK_CRYPTO_BYTESWAP_BTFIFO: u32 = 1 << 4;
pub const RK_CRYPTO_BYTESWAP_BRFIFO: u32 = 1 << 3;
pub const RK_CRYPTO_DESSEL: u32 = 1 << 2;
pub const RK_CYYPTO_HASHINSEL_INDEPENDENT_SOURCE: u32 = 0x00;
pub const RK_CYYPTO_HASHINSEL_BLOCK_CIPHER_INPUT: u32 = 0x01;
pub const RK_CYYPTO_HASHINSEL_BLOCK_CIPHER_OUTPUT: u32 = 0x02;

pub const RK_CRYPTO_BRDMAS: u32 = 0x0010;
pub const RK_CRYPTO_BTDMAS: u32 = 0x0014;
pub const RK_CRYPTO_BRDMAL: u32 = 0x0018;
pub const RK_CRYPTO_HRDMAS: u32 = 0x001c;
pub const RK_CRYPTO_HRDMAL: u32 = 0x0020;

pub const RK_CRYPTO_AES_CTRL: u32 = 0x0080;
pub const RK_CRYPTO_AES_BYTESWAP_CNT: u32 = 1 << 11;
pub const RK_CRYPTO_AES_BYTESWAP_KEY: u32 = 1 << 10;
pub const RK_CRYPTO_AES_BYTESWAP_IV: u32 = 1 << 9;
pub const RK_CRYPTO_AES_BYTESWAP_DO: u32 = 1 << 8;
pub const RK_CRYPTO_AES_BYTESWAP_DI: u32 = 1 << 7;
pub const RK_CRYPTO_AES_KEY_CHANGE: u32 = 1 << 6;
pub const RK_CRYPTO_AES_ECB_MODE: u32 = 0x00 << 4;
pub const RK_CRYPTO_AES_CBC_MODE: u32 = 0x01 << 4;
pub const RK_CRYPTO_AES_CTR_MODE: u32 = 0x02 << 4;
pub const RK_CRYPTO_AES_128BIT_key: u32 = 0x00 << 2;
pub const RK_CRYPTO_AES_192BIT_key: u32 = 0x01 << 2;
pub const RK_CRYPTO_AES_256BIT_key: u32 = 0x02 << 2;
pub const RK_CRYPTO_AES_FIFO_MODE: u32 = 1 << 1;
pub const RK_CRYPTO_AES_DEC: u32 = 1 << 0;
pub const RK_CRYPTO_AES_STS: u32 = 0x0084;
pub const RK_CRYPTO_AES_DONE: u32 = 1;

pub const RK_CRYPTO_AES_DIN_0: u32 = 0x0088;
pub const RK_CRYPTO_AES_DIN_1: u32 = 0x008c;
pub const RK_CRYPTO_AES_DIN_2: u32 = 0x0090;
pub const RK_CRYPTO_AES_DIN_3: u32 = 0x0094;
pub const RK_CRYPTO_AES_DOUT_0: u32 = 0x0098;
pub const RK_CRYPTO_AES_DOUT_1: u32 = 0x009c;
pub const RK_CRYPTO_AES_DOUT_2: u32 = 0x00a0;
pub const RK_CRYPTO_AES_DOUT_3: u32 = 0x00a4;
pub const RK_CRYPTO_AES_IV_0: u32 = 0x00a8;
pub const RK_CRYPTO_AES_IV_1: u32 = 0x00ac;
pub const RK_CRYPTO_AES_IV_2: u32 = 0x00b0;
pub const RK_CRYPTO_AES_IV_3: u32 = 0x00b4;
pub const RK_CRYPTO_AES_KEY_0: u32 = 0x00b8;
pub const RK_CRYPTO_AES_KEY_1: u32 = 0x00bc;
pub const RK_CRYPTO_AES_KEY_2: u32 = 0x00c0;
pub const RK_CRYPTO_AES_KEY_3: u32 = 0x00c4;
pub const RK_CRYPTO_AES_KEY_4: u32 = 0x00c8;
pub const RK_CRYPTO_AES_KEY_5: u32 = 0x00cc;
pub const RK_CRYPTO_AES_KEY_6: u32 = 0x00d0;
pub const RK_CRYPTO_AES_KEY_7: u32 = 0x00d4;

pub const RK_CRYPTO_TDES_CTRL: u32 = 0x0100;
pub const RK_CRYPTO_TDES_BYTESWAP_KEY: u32 = 1 << 8;
pub const RK_CRYPTO_TDES_BYTESWAP_IV: u32 = 1 << 7;
pub const RK_CRYPTO_TDES_BYTESWAP_DO: u32 = 1 << 6;
pub const RK_CRYPTO_TDES_BYTESWAP_DI: u32 = 1 << 5;
pub const RK_CRYPTO_TDES_CHAINMODE_CBC: u32 = 1 << 4;
pub const RK_CRYPTO_TDES_EEE: u32 = 1 << 3;
pub const RK_CRYPTO_TDES_SELECT: u32 = 1 << 2;
pub const RK_CRYPTO_TDES_FIFO_MODE: u32 = 1 << 1;
pub const RK_CRYPTO_TDES_DEC: u32 = 1;
pub const RK_CRYPTO_TDES_STS: u32 = 0x0104;
pub const RK_CRYPTO_TDES_DONE: u32 = 1;
pub const RK_CRYPTO_TDES_DIN_0: u32 = 0x0108;
pub const RK_CRYPTO_TDES_DIN_1: u32 = 0x010c;
pub const RK_CRYPTO_TDES_DOUT_0: u32 = 0x0110;
pub const RK_CRYPTO_TDES_DOUT_1: u32 = 0x0114;
pub const RK_CRYPTO_TDES_IV_0: u32 = 0x0118;
pub const RK_CRYPTO_TDES_IV_1: u32 = 0x011c;
pub const RK_CRYPTO_TDES_KEY1_0: u32 = 0x0120;
pub const RK_CRYPTO_TDES_KEY1_1: u32 = 0x0124;
pub const RK_CRYPTO_TDES_KEY2_0: u32 = 0x0128;
pub const RK_CRYPTO_TDES_KEY2_1: u32 = 0x012c;
pub const RK_CRYPTO_TDES_KEY3_0: u32 = 0x0130;
pub const RK_CRYPTO_TDES_KEY3_1: u32 = 0x0134;

pub const RK_CRYPTO_HASH_CTRL: u32 = 0x0180;
pub const RK_CRYPTO_HASH_SWAP_DO: u32 = 1 << 3;
pub const RK_CRYPTO_HASH_SWAP_DI: u32 = 1 << 2;
pub const RK_CRYPTO_HASH_SHA1: u32 = 0;
pub const RK_CRYPTO_HASH_MD5: u32 = 1;
pub const RK_CRYPTO_HASH_SHA256: u32 = 2;
pub const RK_CRYPTO_HASH_PRNG: u32 = 3;
pub const RK_CRYPTO_HASH_STS: u32 = 0x0184;
pub const RK_CRYPTO_HASH_DONE: u32 = 1;
pub const RK_CRYPTO_HASH_MSG_LEN: u32 = 0x0188;
pub const RK_CRYPTO_HASH_DOUT_0: u32 = 0x018c;
pub const RK_CRYPTO_HASH_DOUT_1: u32 = 0x0190;
pub const RK_CRYPTO_HASH_DOUT_2: u32 = 0x0194;
pub const RK_CRYPTO_HASH_DOUT_3: u32 = 0x0198;
pub const RK_CRYPTO_HASH_DOUT_4: u32 = 0x019c;
pub const RK_CRYPTO_HASH_DOUT_5: u32 = 0x01a0;
pub const RK_CRYPTO_HASH_DOUT_6: u32 = 0x01a4;
pub const RK_CRYPTO_HASH_DOUT_7: u32 = 0x01a8;

macro_rules! CRYPTO_READ { ($dev:expr, $offset:expr) => { unsafe { readl_relaxed(($dev).reg.add($offset as usize)) } }; }
macro_rules! CRYPTO_WRITE { ($dev:expr, $offset:expr, $val:expr) => { unsafe { writel_relaxed($val, ($dev).reg.add($offset as usize)) } }; }

pub const RK_MAX_CLKS: usize = 4;

#[repr(C)]
pub struct rockchip_ip {
    pub dev_list: list_head,
    pub lock: spinlock_t,
    pub dbgfs_dir: *mut dentry,
    pub dbgfs_stats: *mut dentry,
}

#[repr(C)]
pub struct rk_clks { pub name: *const core::ffi::c_char, pub max: c_ulong }
#[repr(C)]
pub struct rk_variant { pub num_clks: c_int, pub rkclks: [rk_clks; RK_MAX_CLKS] }
#[repr(C)]
pub struct rk_crypto_info {
    pub list: list_head, pub dev: *mut device, pub clks: *mut clk_bulk_data,
    pub num_clks: c_int, pub rst: *mut reset_control, pub reg: *mut core::ffi::c_void,
    pub irq: c_int, pub variant: *const rk_variant, pub nreq: c_ulong,
    pub engine: *mut crypto_engine, pub complete: completion, pub status: c_int,
}

#[repr(C)] pub struct rk_ahash_ctx { pub fallback_tfm: *mut crypto_ahash }
#[repr(C)] pub struct rk_ahash_rctx { pub dev: *mut rk_crypto_info, pub fallback_req: ahash_request, pub mode: u32, pub nrsg: c_int }
#[repr(C)] pub struct rk_cipher_ctx { pub keylen: c_uint, pub key: [u8; AES_MAX_KEY_SIZE], pub iv: [u8; AES_BLOCK_SIZE], pub fallback_tfm: *mut crypto_skcipher }
#[repr(C)] pub struct rk_cipher_rctx { pub dev: *mut rk_crypto_info, pub backup_iv: [u8; AES_BLOCK_SIZE], pub mode: u32, pub fallback_req: skcipher_request }

#[repr(C)] pub union rk_crypto_tmp_alg { pub skcipher: skcipher_engine_alg, pub hash: ahash_engine_alg }
#[repr(C)] pub struct rk_crypto_tmp { pub type_: u32, pub dev: *mut rk_crypto_info, pub alg: rk_crypto_tmp_alg, pub stat_req: c_ulong, pub stat_fb: c_ulong, pub stat_fb_len: c_ulong, pub stat_fb_sglen: c_ulong, pub stat_fb_align: c_ulong, pub stat_fb_sgdiff: c_ulong }

extern "C" {
    pub static mut rk_ecb_aes_alg: rk_crypto_tmp;
    pub static mut rk_cbc_aes_alg: rk_crypto_tmp;
    pub static mut rk_ecb_des_alg: rk_crypto_tmp;
    pub static mut rk_cbc_des_alg: rk_crypto_tmp;
    pub static mut rk_ecb_des3_ede_alg: rk_crypto_tmp;
    pub static mut rk_cbc_des3_ede_alg: rk_crypto_tmp;
    pub static mut rk_ahash_sha1: rk_crypto_tmp;
    pub static mut rk_ahash_sha256: rk_crypto_tmp;
    pub static mut rk_ahash_md5: rk_crypto_tmp;
    pub fn get_rk_crypto() -> *mut rk_crypto_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
