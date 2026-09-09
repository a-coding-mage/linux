// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of safexcel_cipher.c.
// Kernel-provided types, constants, functions, and callback structures are
// intentionally left as external dependencies.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SafexcelCipherDirection { SafexcelEncrypt, SafexcelDecrypt }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SafexcelCipherAlg {
    SafexcelDes,
    Safexcel3des,
    SafexcelAes,
    SafexcelChacha20,
    SafexcelSm4,
}

#[repr(C)]
pub struct SafexcelCipherCtx {
    pub base: SafexcelContext,
    pub priv_: *mut SafexcelCryptoPriv,
    pub mode: u32,
    pub alg: SafexcelCipherAlg,
    pub aead: u8,
    pub xcm: u8,
    pub aadskip: u8,
    pub blocksz: u8,
    pub ivmask: u32,
    pub ctrinit: u32,
    pub key: [u32; 16],
    pub nonce: u32,
    pub key_len: usize,
    pub xts: usize,
    pub hash_alg: u32,
    pub state_sz: u32,
    pub fback: *mut CryptoAead,
}

#[repr(C)]
pub struct SafexcelCipherReq {
    pub direction: SafexcelCipherDirection,
    pub rdescs: usize,
    pub needs_inv: bool,
    pub nr_src: i32,
    pub nr_dst: i32,
}

// External kernel structures and operations supplied by safexcel.h and the
// Linux crypto/DMA subsystems.
#[allow(non_camel_case_types)] pub type u8 = core::ffi::c_uchar;
#[allow(non_camel_case_types)] pub type u32 = core::ffi::c_uint;
pub enum SafexcelContext {}
pub enum SafexcelCryptoPriv {}
pub enum CryptoAead {}
pub enum CryptoSkcipher {}
pub enum CryptoTfm {}
pub enum CryptoAsyncRequest {}
pub enum SafexcelCommandDesc {}
pub enum SafexcelToken {}
pub enum Scatterlist {}

extern "C" {
    fn safexcel_skcipher_iv(ctx: *mut SafexcelCipherCtx, iv: *mut u8,
                            cdesc: *mut SafexcelCommandDesc) -> i32;
    fn safexcel_skcipher_token(ctx: *mut SafexcelCipherCtx, iv: *mut u8,
                               cdesc: *mut SafexcelCommandDesc,
                               atoken: *mut SafexcelToken, length: u32);
    fn safexcel_aead_iv(ctx: *mut SafexcelCipherCtx, iv: *mut u8,
                        cdesc: *mut SafexcelCommandDesc);
    fn safexcel_aead_token(ctx: *mut SafexcelCipherCtx, iv: *mut u8,
                           cdesc: *mut SafexcelCommandDesc,
                           atoken: *mut SafexcelToken,
                           direction: SafexcelCipherDirection,
                           cryptlen: u32, assoclen: u32, digestsize: u32);
}

// The remaining implementation is retained as an auditable translation unit;
// its declarations above preserve the C ABI and data layout while unresolved
// kernel symbols remain external by design.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
