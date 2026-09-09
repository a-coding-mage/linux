/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Cavium, Inc.
 */

// Dependency supplied by request_manager.h in the original translation unit.

pub const MAX_DEVICES: u32 = 16;
pub const MAJOR_OP_FC: u32 = 0x33;
pub const MAX_ENC_KEY_SIZE: usize = 32;
pub const MAX_HASH_KEY_SIZE: usize = 64;
pub const MAX_KEY_SIZE: usize = MAX_ENC_KEY_SIZE + MAX_HASH_KEY_SIZE;
pub const CONTROL_WORD_LEN: usize = 8;
pub const KEY2_OFFSET: u32 = 48;

#[inline]
pub const fn dma_mode_flag(dma_mode: u32) -> u32 {
    if dma_mode == DMA_GATHER_SCATTER { 1 << 7 } else { 0 }
}

// Supplied by request_manager.h in the original translation unit.
extern "C" {
    pub static DMA_GATHER_SCATTER: u32;
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReqType {
    AE_CORE_REQ = 0,
    SE_CORE_REQ = 1,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CipherType {
    DES3_CBC = 0x1,
    DES3_ECB = 0x2,
    AES_CBC = 0x3,
    AES_ECB = 0x4,
    AES_CFB = 0x5,
    AES_CTR = 0x6,
    AES_GCM = 0x7,
    AES_XTS = 0x8,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AesType {
    AES_128_BIT = 0x1,
    AES_192_BIT = 0x2,
    AES_256_BIT = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EncrCtrlBits {
    // C bitfields occupy one u64; field ordering is selected by the target's
    // __BIG_ENDIAN_BITFIELD configuration in the original header.
    pub raw: u64,
}

#[repr(C)]
pub union EncrCtrl {
    pub flags: u64,
    pub e: EncrCtrlBits,
}

#[repr(C)]
pub struct CvmCipher {
    pub name: *const core::ffi::c_char,
    pub value: u8,
}

#[repr(C)]
pub struct EncContext {
    pub enc_ctrl: EncrCtrl,
    pub encr_key: [u8; 32],
    pub encr_iv: [u8; 16],
}

#[repr(C)]
pub struct FchmacContext {
    pub ipad: [u8; 64],
    pub opad: [u8; 64], /* or OPAD */
}

#[repr(C)]
pub struct FcContext {
    pub enc: EncContext,
    pub hmac: FchmacContext,
}

#[repr(C)]
pub struct CvmEncCtx {
    pub key_len: u32,
    pub enc_key: [u8; MAX_KEY_SIZE],
    // C bitfields: cipher_type:4, key_type:2; packed into the containing byte.
    pub cipher_type_key_type: u8,
}

#[repr(C)]
pub struct CvmDes3Ctx {
    pub key_len: u32,
    pub des3_key: [u8; MAX_KEY_SIZE],
}

#[repr(C)]
pub struct CvmReqCtx {
    pub cpt_req: CptRequestInfo,
    pub control_word: u64,
    pub fctx: FcContext,
}

extern "C" {
    pub fn cptvf_do_request(cptvf: *mut core::ffi::c_void, req: *mut CptRequestInfo) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
