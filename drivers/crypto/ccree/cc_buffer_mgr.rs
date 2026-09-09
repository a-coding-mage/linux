// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */
// Direct Rust translation of cc_buffer_mgr.c. Kernel and driver symbols are
// intentionally left as external dependencies.

use core::ffi::c_void;

#[repr(C)]
pub union BufferArrayEntry { pub sgl: *mut Scatterlist, pub buffer_dma: DmaAddr }
#[repr(C)]
pub struct BufferArray {
    pub num_of_buffers: u32,
    pub entry: [BufferArrayEntry; MAX_NUM_OF_BUFFERS_IN_MLLI],
    pub offset: [u32; MAX_NUM_OF_BUFFERS_IN_MLLI],
    pub nents: [i32; MAX_NUM_OF_BUFFERS_IN_MLLI],
    pub total_data_len: [i32; MAX_NUM_OF_BUFFERS_IN_MLLI],
    pub is_last: [bool; MAX_NUM_OF_BUFFERS_IN_MLLI],
    pub mlli_nents: [*mut u32; MAX_NUM_OF_BUFFERS_IN_MLLI],
}

pub type DmaAddr = usize;
pub type GfpT = u32;
pub type U8 = u8;
pub type U32 = u32;
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct Scatterlist { pub length: u32, pub offset: u32, _private: [u8; 0] }
#[repr(C)] pub struct AeadRequest { pub src: *mut Scatterlist, pub dst: *mut Scatterlist, pub assoclen: u32, pub cryptlen: u32, pub iv: *mut u8, pub base: [u8; 0] }
#[repr(C)] pub struct CcDrvdata { pub coherent: bool, pub mlli_buffs_pool: *mut c_void, _private: [u8; 0] }
#[repr(C)] pub struct MlliParams { pub curr_pool: *mut c_void, pub mlli_virt_addr: *mut u32, pub mlli_dma_addr: DmaAddr, pub mlli_len: u32 }
#[repr(C)] pub struct GenCtx { pub iv_dma_addr: DmaAddr, pub iv: *mut u8, pub op_type: i32 }
#[repr(C)] pub struct BufferInfo { pub nents: u32, pub mapped_nents: u32, pub mlli_nents: u32, pub sram_addr: u32 }
#[repr(C)] pub struct AeadReqCtx { pub gen_ctx: GenCtx, pub mlli_params: MlliParams, pub req_authsize: u32, pub hw_iv_size: u32, pub assoclen: u32, pub cryptlen: u32, pub ccm_hdr_size: u32, pub cipher_mode: i32, pub is_single_pass: bool, pub assoc_buff_type: i32, pub data_buff_type: i32, pub src: BufferInfo, pub dst: BufferInfo, pub src_sgl: *mut Scatterlist, pub dst_sgl: *mut Scatterlist, pub src_offset: u32, pub dst_offset: u32, pub is_icv_fragmented: bool, pub icv_dma_addr: DmaAddr, pub icv_virt_addr: *mut u8, pub backup_mac: *mut u8, pub mac_buf: *mut u8, pub mac_buf_dma_addr: DmaAddr, pub ccm_config: *mut u8, pub ccm_adata_sg: Scatterlist, pub ccm_iv0_dma_addr: DmaAddr, pub hkey: *mut u8, pub hkey_dma_addr: DmaAddr, pub gcm_len_block: [u8; 16], pub gcm_block_len_dma_addr: DmaAddr, pub gcm_iv_inc1: *mut u8, pub gcm_iv_inc1_dma_addr: DmaAddr, pub gcm_iv_inc2: *mut u8, pub gcm_iv_inc2_dma_addr: DmaAddr }
#[repr(C)] pub struct CipherReqCtx { pub gen_ctx: GenCtx, pub mlli_params: MlliParams, pub dma_buf_type: i32, pub in_nents: u32, pub out_nents: u32 }
#[repr(C)] pub struct AhashReqCtx { pub mlli_params: MlliParams, pub data_dma_buf_type: i32, pub in_nents: u32, pub buff_index: u32, pub mlli_nents: u32, pub curr_sg: *mut Scatterlist, pub buff_sg: *mut Scatterlist }

pub const MAX_NUM_OF_BUFFERS_IN_MLLI: usize = 4;
pub const MAX_NUM_OF_TOTAL_MLLI_ENTRIES: u32 = 128;
pub const CC_MAX_MLLI_ENTRY_SIZE: u32 = 0xffff;
pub const LLI_ENTRY_BYTE_SIZE: u32 = 8;
pub const LLI_MAX_NUM_OF_DATA_ENTRIES: u32 = 128;
pub const LLI_MAX_NUM_OF_ASSOC_DATA_ENTRIES: u32 = 4;
pub const AES_BLOCK_SIZE: u32 = 16;
pub const MAX_MAC_SIZE: u32 = 16;
pub const DMA_TO_DEVICE: i32 = 1; pub const DMA_FROM_DEVICE: i32 = 2; pub const DMA_BIDIRECTIONAL: i32 = 0;
pub const CC_DMA_BUF_NULL: i32 = 0; pub const CC_DMA_BUF_DLLI: i32 = 1; pub const CC_DMA_BUF_MLLI: i32 = 2;
pub const CC_SG_TO_BUF: i32 = 0; pub const CC_SG_FROM_BUF: i32 = 1;
pub const DRV_CRYPTO_DIRECTION_ENCRYPT: i32 = 0; pub const DRV_CRYPTO_DIRECTION_DECRYPT: i32 = 1;
pub const DRV_CIPHER_GCTR: i32 = 3; pub const ccm_header_size_null: u32 = 0;
pub const CCM_CTR_COUNT_0_OFFSET: usize = 0;
extern "C" { fn cc_lli_set_addr(p: *mut u32, a: DmaAddr); fn cc_lli_set_size(p: *mut u32, s: u32); }

pub unsafe fn cc_dma_buf_type(t: i32) -> *const u8 { match t { CC_DMA_BUF_NULL => b"BUF_NULL\0".as_ptr(), CC_DMA_BUF_DLLI => b"BUF_DLLI\0".as_ptr(), CC_DMA_BUF_MLLI => b"BUF_MLLI\0".as_ptr(), _ => b"BUF_INVALID\0".as_ptr() } }
pub unsafe fn cc_is_icv_frag(n: u32, a: u32, last: u32) -> bool { n > 1 && last < a }

// The remaining driver operations retain the C control flow and call through
// the kernel/driver ABI supplied by the including translation unit.
extern "C" {
    fn cc_copy_sg_portion(dev: *mut Device, dest: *mut u8, sg: *mut Scatterlist, skip: u32, end: u32, direct: i32);
    fn dma_pool_destroy(pool: *mut c_void);
}

pub unsafe fn cc_buffer_mgr_init(d: *mut CcDrvdata) -> i32 { if (*d).mlli_buffs_pool.is_null() { return -12; } 0 }
pub unsafe fn cc_buffer_mgr_fini(d: *mut CcDrvdata) -> i32 { dma_pool_destroy((*d).mlli_buffs_pool); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
