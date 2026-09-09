/* SPDX-License-Identifier: GPL-2.0+ */

/* Dependencies supplied by the surrounding kernel/Rust translation. */

/* HACE register definitions */
pub const ASPEED_HACE_SRC: usize = 0x00;
pub const ASPEED_HACE_DEST: usize = 0x04;
pub const ASPEED_HACE_CONTEXT: usize = 0x08;
pub const ASPEED_HACE_DATA_LEN: usize = 0x0C;
pub const ASPEED_HACE_CMD: usize = 0x10;

/* G5 */
pub const ASPEED_HACE_TAG: usize = 0x18;
/* G6 */
pub const ASPEED_HACE_GCM_ADD_LEN: usize = 0x14;
pub const ASPEED_HACE_GCM_TAG_BASE_ADDR: usize = 0x18;
pub const ASPEED_HACE_STS: usize = 0x1C;
pub const ASPEED_HACE_HASH_SRC: usize = 0x20;
pub const ASPEED_HACE_HASH_DIGEST_BUFF: usize = 0x24;
pub const ASPEED_HACE_HASH_KEY_BUFF: usize = 0x28;
pub const ASPEED_HACE_HASH_DATA_LEN: usize = 0x2C;
pub const ASPEED_HACE_HASH_CMD: usize = 0x30;

pub const HACE_CMD_SINGLE_DES: u32 = 0;
pub const HACE_CMD_TRIPLE_DES: u32 = 1 << 17;
pub const HACE_CMD_AES_SELECT: u32 = 0;
pub const HACE_CMD_DES_SELECT: u32 = 1 << 16;
pub const HACE_CMD_ISR_EN: u32 = 1 << 12;
pub const HACE_CMD_CONTEXT_SAVE_ENABLE: u32 = 0;
pub const HACE_CMD_CONTEXT_SAVE_DISABLE: u32 = 1 << 9;
pub const HACE_CMD_AES: u32 = 0;
pub const HACE_CMD_DES: u32 = 0;
pub const HACE_CMD_RC4: u32 = 1 << 8;
pub const HACE_CMD_DECRYPT: u32 = 0;
pub const HACE_CMD_ENCRYPT: u32 = 1 << 7;
pub const HACE_CMD_ECB: u32 = 0x0 << 4;
pub const HACE_CMD_CBC: u32 = 0x1 << 4;
pub const HACE_CMD_CFB: u32 = 0x2 << 4;
pub const HACE_CMD_OFB: u32 = 0x3 << 4;
pub const HACE_CMD_CTR: u32 = 0x4 << 4;
pub const HACE_CMD_OP_MODE_MASK: u32 = 0x7 << 4;
pub const HACE_CMD_AES128: u32 = 0x0 << 2;
pub const HACE_CMD_AES192: u32 = 0x1 << 2;
pub const HACE_CMD_AES256: u32 = 0x2 << 2;
pub const HACE_CMD_OP_CASCADE: u32 = 0x3;
pub const HACE_CMD_OP_INDEPENDENT: u32 = 0x1;
pub const HACE_CMD_RI_WO_DATA_ENABLE: u32 = 0;
pub const HACE_CMD_RI_WO_DATA_DISABLE: u32 = 1 << 11;
pub const HACE_CMD_CONTEXT_LOAD_ENABLE: u32 = 0;
pub const HACE_CMD_CONTEXT_LOAD_DISABLE: u32 = 1 << 10;
pub const HACE_CMD_AES_KEY_FROM_OTP: u32 = 1 << 24;
pub const HACE_CMD_GHASH_TAG_XOR_EN: u32 = 1 << 23;
pub const HACE_CMD_GHASH_PAD_LEN_INV: u32 = 1 << 22;
pub const HACE_CMD_GCM_TAG_ADDR_SEL: u32 = 1 << 21;
pub const HACE_CMD_MBUS_REQ_SYNC_EN: u32 = 1 << 20;
pub const HACE_CMD_DES_SG_CTRL: u32 = 1 << 19;
pub const HACE_CMD_SRC_SG_CTRL: u32 = 1 << 18;
pub const HACE_CMD_CTR_IV_AES_96: u32 = 0x1 << 14;
pub const HACE_CMD_CTR_IV_DES_32: u32 = 0x1 << 14;
pub const HACE_CMD_CTR_IV_AES_64: u32 = 0x2 << 14;
pub const HACE_CMD_CTR_IV_AES_32: u32 = 0x3 << 14;
pub const HACE_CMD_AES_KEY_HW_EXP: u32 = 1 << 13;
pub const HACE_CMD_GCM: u32 = 0x5 << 4;

pub const HACE_CRYPTO_ISR: u32 = 1 << 12;
pub const HACE_HASH_ISR: u32 = 1 << 9;
pub const HACE_HASH_BUSY: u32 = 1 << 0;
pub const HASH_CMD_MBUS_REQ_SYNC_EN: u32 = 1 << 20;
pub const HASH_CMD_HASH_SRC_SG_CTRL: u32 = 1 << 18;
pub const HASH_CMD_SHA512_224: u32 = 0x3 << 10;
pub const HASH_CMD_SHA512_256: u32 = 0x2 << 10;
pub const HASH_CMD_SHA384: u32 = 0x1 << 10;
pub const HASH_CMD_SHA512: u32 = 0;
pub const HASH_CMD_INT_ENABLE: u32 = 1 << 9;
pub const HASH_CMD_HMAC: u32 = 0x1 << 7;
pub const HASH_CMD_ACC_MODE: u32 = 0x2 << 7;
pub const HASH_CMD_HMAC_KEY: u32 = 0x3 << 7;
pub const HASH_CMD_SHA1: u32 = 0x2 << 4;
pub const HASH_CMD_SHA224: u32 = 0x4 << 4;
pub const HASH_CMD_SHA256: u32 = 0x5 << 4;
pub const HASH_CMD_SHA512_SER: u32 = 0x6 << 4;
pub const HASH_CMD_SHA_SWAP: u32 = 0x2 << 2;
pub const HASH_SG_LAST_LIST: u32 = 1 << 31;
pub const CRYPTO_FLAGS_BUSY: u32 = 1 << 1;
pub const SHA_OP_UPDATE: u32 = 1;
pub const SHA_OP_FINAL: u32 = 2;
pub const SHA_FLAGS_SHA1: u32 = 1 << 0;
pub const SHA_FLAGS_SHA224: u32 = 1 << 1;
pub const SHA_FLAGS_SHA256: u32 = 1 << 2;
pub const SHA_FLAGS_SHA384: u32 = 1 << 3;
pub const SHA_FLAGS_SHA512: u32 = 1 << 4;
pub const SHA_FLAGS_SHA512_224: u32 = 1 << 5;
pub const SHA_FLAGS_SHA512_256: u32 = 1 << 6;
pub const SHA_FLAGS_FINUP: u32 = 1 << 9;
pub const SHA_FLAGS_MASK: u32 = 0xff;
pub const ASPEED_CRYPTO_SRC_DMA_BUF_LEN: usize = 0xa000;
pub const ASPEED_CRYPTO_DST_DMA_BUF_LEN: usize = 0xa000;
pub const ASPEED_CRYPTO_GCM_TAG_OFFSET: usize = 0x9ff0;
pub const ASPEED_HASH_SRC_DMA_BUF_LEN: usize = 0xa000;
pub const ASPEED_HASH_QUEUE_LENGTH: usize = 50;
pub const HACE_CMD_IV_REQUIRE: u32 = HACE_CMD_CBC | HACE_CMD_CFB | HACE_CMD_OFB | HACE_CMD_CTR;

pub struct aspeed_hace_dev;
pub struct scatterlist;
pub type aspeed_hace_fn_t = unsafe extern "C" fn(*mut aspeed_hace_dev) -> i32;

#[repr(C)]
pub struct aspeed_sg_list { pub len: __le32, pub phy_addr: __le32 }

#[repr(C)]
pub struct aspeed_engine_hash {
    pub done_task: tasklet_struct, pub flags: c_ulong, pub req: *mut ahash_request,
    pub ahash_src_addr: *mut c_void, pub ahash_src_dma_addr: dma_addr_t,
    pub src_dma: dma_addr_t, pub digest_dma: dma_addr_t, pub src_length: usize,
    pub resume: aspeed_hace_fn_t, pub dma_prepare: aspeed_hace_fn_t,
}
#[repr(C)] pub struct aspeed_sham_ctx { pub hace_dev: *mut aspeed_hace_dev }
#[repr(C)] pub struct aspeed_sham_reqctx {
    pub digest: [u8; SHA512_DIGEST_SIZE], pub digcnt: [u64; 2], pub flags: c_ulong, pub cmd: u32,
    pub src_sg: *mut scatterlist, pub src_nents: i32, pub offset: c_uint, pub total: c_uint,
    pub digsize: usize, pub block_size: usize, pub ivsize: usize, pub buffer_dma_addr: dma_addr_t,
    pub digest_dma_addr: dma_addr_t, pub buffer: [u8; SHA512_BLOCK_SIZE + 16],
}
#[repr(C)] pub struct aspeed_engine_crypto {
    pub done_task: tasklet_struct, pub flags: c_ulong, pub req: *mut skcipher_request,
    pub cipher_ctx: *mut c_void, pub cipher_ctx_dma: dma_addr_t, pub cipher_addr: *mut c_void,
    pub cipher_dma_addr: dma_addr_t, pub dst_sg_addr: *mut c_void, pub dst_sg_dma_addr: dma_addr_t,
    pub resume: aspeed_hace_fn_t,
}
#[repr(C)] pub struct aspeed_cipher_ctx {
    pub hace_dev: *mut aspeed_hace_dev, pub key_len: i32, pub key: [u8; AES_MAX_KEYLENGTH],
    pub start: aspeed_hace_fn_t, pub fallback_tfm: *mut crypto_skcipher,
}
#[repr(C)] pub struct aspeed_cipher_reqctx {
    pub enc_cmd: i32, pub src_nents: i32, pub dst_nents: i32, pub fallback_req: skcipher_request,
}
#[repr(C)] pub struct aspeed_hace_dev {
    pub regs: *mut c_void, pub dev: *mut device, pub irq: i32, pub clk: *mut clk, pub version: c_ulong,
    pub crypt_engine_hash: *mut crypto_engine, pub crypt_engine_crypto: *mut crypto_engine,
    pub hash_engine: aspeed_engine_hash, pub crypto_engine: aspeed_engine_crypto,
}
#[repr(C)] pub union aspeed_hace_alg_union { pub skcipher: skcipher_engine_alg, pub ahash: ahash_engine_alg }
#[repr(C)] pub struct aspeed_hace_alg { pub hace_dev: *mut aspeed_hace_dev, pub alg_base: *const c_char, pub alg: aspeed_hace_alg_union }

#[repr(C)] pub enum aspeed_version { AST2500_VERSION = 5, AST2600_VERSION }

pub unsafe fn ast_hace_write(hace: *mut aspeed_hace_dev, val: u32, offset: usize) { writel(val, ((*hace).regs as *mut u8).add(offset) as *mut c_void); }
pub unsafe fn ast_hace_read(hace: *mut aspeed_hace_dev, offset: usize) -> u32 { readl(((*hace).regs as *mut u8).add(offset) as *const c_void) }

unsafe extern "C" {
    pub fn aspeed_register_hace_hash_algs(hace_dev: *mut aspeed_hace_dev);
    pub fn aspeed_unregister_hace_hash_algs(hace_dev: *mut aspeed_hace_dev);
    pub fn aspeed_register_hace_crypto_algs(hace_dev: *mut aspeed_hace_dev);
    pub fn aspeed_unregister_hace_crypto_algs(hace_dev: *mut aspeed_hace_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
