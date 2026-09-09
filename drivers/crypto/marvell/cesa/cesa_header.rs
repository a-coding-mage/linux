/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the C header. Kernel-provided types and functions remain external dependencies.

pub const fn cesa_engine_off(i: u32) -> u32 { i * 0x2000 }
pub const CESA_TDMA_BYTE_CNT: u32 = 0x800;
pub const CESA_TDMA_SRC_ADDR: u32 = 0x810;
pub const CESA_TDMA_DST_ADDR: u32 = 0x820;
pub const CESA_TDMA_NEXT_ADDR: u32 = 0x830;
pub const CESA_TDMA_CONTROL: u32 = 0x840;
pub const CESA_TDMA_DST_BURST: u32 = 0x7;
pub const CESA_TDMA_DST_BURST_32B: u32 = 3;
pub const CESA_TDMA_DST_BURST_128B: u32 = 4;
pub const CESA_TDMA_OUT_RD_EN: u32 = 1 << 4;
pub const CESA_TDMA_SRC_BURST: u32 = 0x1c0;
pub const CESA_TDMA_SRC_BURST_32B: u32 = 3 << 6;
pub const CESA_TDMA_SRC_BURST_128B: u32 = 4 << 6;
pub const CESA_TDMA_CHAIN: u32 = 1 << 9;
pub const CESA_TDMA_BYTE_SWAP: u32 = 1 << 11;
pub const CESA_TDMA_NO_BYTE_SWAP: u32 = 1 << 11;
pub const CESA_TDMA_EN: u32 = 1 << 12;
pub const CESA_TDMA_FETCH_ND: u32 = 1 << 13;
pub const CESA_TDMA_ACT: u32 = 1 << 14;
pub const CESA_TDMA_CUR: u32 = 0x870;
pub const CESA_TDMA_ERROR_CAUSE: u32 = 0x8c8;
pub const CESA_TDMA_ERROR_MSK: u32 = 0x8cc;
pub const fn cesa_tdma_window_base(x: u32) -> u32 { x * 8 + 0xa00 }
pub const fn cesa_tdma_window_ctrl(x: u32) -> u32 { x * 8 + 0xa04 }
pub const fn cesa_ivdig(x: u32) -> u32 { 0xdd00 + x * 4 + if x < 5 { 0 } else { 0x14 } }
pub const CESA_SA_CMD: u32 = 0xde00;
pub const CESA_SA_CMD_EN_CESA_SA_ACCL0: u32 = 1;
pub const CESA_SA_CMD_EN_CESA_SA_ACCL1: u32 = 2;
pub const CESA_SA_CMD_DISABLE_SEC: u32 = 4;
pub const CESA_SA_DESC_P0: u32 = 0xde04;
pub const CESA_SA_DESC_P1: u32 = 0xde14;
pub const CESA_SA_CFG: u32 = 0xde08;
pub const CESA_SA_CFG_STOP_DIG_ERR: u32 = 3;
pub const CESA_SA_CFG_DIG_ERR_CONT: u32 = 0;
pub const CESA_SA_CFG_DIG_ERR_SKIP: u32 = 1;
pub const CESA_SA_CFG_DIG_ERR_STOP: u32 = 3;
pub const CESA_SA_CFG_CH0_W_IDMA: u32 = 1 << 7;
pub const CESA_SA_CFG_CH1_W_IDMA: u32 = 1 << 8;
pub const CESA_SA_CFG_ACT_CH0_IDMA: u32 = 1 << 9;
pub const CESA_SA_CFG_ACT_CH1_IDMA: u32 = 1 << 10;
pub const CESA_SA_CFG_MULTI_PKT: u32 = 1 << 11;
pub const CESA_SA_CFG_PARA_DIS: u32 = 1 << 13;
pub const CESA_SA_ACCEL_STATUS: u32 = 0xde0c;
pub const CESA_SA_ST_ACT_0: u32 = 1;
pub const CESA_SA_ST_ACT_1: u32 = 2;
/* FPGA status is documented only in Errata 4.12 and appears to be legacy. */
pub const CESA_SA_FPGA_INT_STATUS: u32 = 0xdd68;
pub const CESA_SA_INT_STATUS: u32 = 0xde20;
pub const CESA_SA_INT_AUTH_DONE: u32 = 1;
pub const CESA_SA_INT_DES_E_DONE: u32 = 1 << 1;
pub const CESA_SA_INT_AES_E_DONE: u32 = 1 << 2;
pub const CESA_SA_INT_AES_D_DONE: u32 = 1 << 3;
pub const CESA_SA_INT_ENC_DONE: u32 = 1 << 4;
pub const CESA_SA_INT_ACCEL0_DONE: u32 = 1 << 5;
pub const CESA_SA_INT_ACCEL1_DONE: u32 = 1 << 6;
pub const CESA_SA_INT_ACC0_IDMA_DONE: u32 = 1 << 7;
pub const CESA_SA_INT_ACC1_IDMA_DONE: u32 = 1 << 8;
pub const CESA_SA_INT_IDMA_DONE: u32 = 1 << 9;
pub const CESA_SA_INT_IDMA_OWN_ERR: u32 = 1 << 10;
pub const CESA_SA_INT_MSK: u32 = 0xde24;

pub const CESA_SA_DESC_CFG_OP_MAC_ONLY: u32 = 0;
pub const CESA_SA_DESC_CFG_OP_CRYPT_ONLY: u32 = 1;
pub const CESA_SA_DESC_CFG_OP_MAC_CRYPT: u32 = 2;
pub const CESA_SA_DESC_CFG_OP_CRYPT_MAC: u32 = 3;
pub const CESA_SA_DESC_CFG_OP_MSK: u32 = 3;
pub const CESA_SA_DESC_CFG_MACM_SHA256: u32 = 1 << 4;
pub const CESA_SA_DESC_CFG_MACM_HMAC_SHA256: u32 = 3 << 4;
pub const CESA_SA_DESC_CFG_MACM_MD5: u32 = 4 << 4;
pub const CESA_SA_DESC_CFG_MACM_SHA1: u32 = 5 << 4;
pub const CESA_SA_DESC_CFG_MACM_HMAC_MD5: u32 = 6 << 4;
pub const CESA_SA_DESC_CFG_MACM_HMAC_SHA1: u32 = 7 << 4;
pub const CESA_SA_DESC_CFG_MACM_MSK: u32 = 0x70;
pub const CESA_SA_DESC_CFG_CRYPTM_DES: u32 = 1 << 8;
pub const CESA_SA_DESC_CFG_CRYPTM_3DES: u32 = 2 << 8;
pub const CESA_SA_DESC_CFG_CRYPTM_AES: u32 = 3 << 8;
pub const CESA_SA_DESC_CFG_CRYPTM_MSK: u32 = 3 << 8;
pub const CESA_SA_DESC_CFG_DIR_ENC: u32 = 0;
pub const CESA_SA_DESC_CFG_DIR_DEC: u32 = 1 << 12;
pub const CESA_SA_DESC_CFG_CRYPTCM_ECB: u32 = 0;
pub const CESA_SA_DESC_CFG_CRYPTCM_CBC: u32 = 1 << 16;
pub const CESA_SA_DESC_CFG_CRYPTCM_MSK: u32 = 1 << 16;
pub const CESA_SA_DESC_CFG_3DES_EEE: u32 = 0;
pub const CESA_SA_DESC_CFG_3DES_EDE: u32 = 1 << 20;
pub const CESA_SA_DESC_CFG_AES_LEN_128: u32 = 0;
pub const CESA_SA_DESC_CFG_AES_LEN_192: u32 = 1 << 24;
pub const CESA_SA_DESC_CFG_AES_LEN_256: u32 = 2 << 24;
pub const CESA_SA_DESC_CFG_AES_LEN_MSK: u32 = 3 << 24;
pub const CESA_SA_DESC_CFG_NOT_FRAG: u32 = 0;
pub const CESA_SA_DESC_CFG_FIRST_FRAG: u32 = 1 << 30;
pub const CESA_SA_DESC_CFG_LAST_FRAG: u32 = 2 << 30;
pub const CESA_SA_DESC_CFG_MID_FRAG: u32 = 3 << 30;
pub const CESA_SA_DESC_CFG_FRAG_MSK: u32 = 3 << 30;

pub const CESA_SA_CFG_SRAM_OFFSET: u32 = 0;
pub const CESA_SA_DATA_SRAM_OFFSET: u32 = 0x80;
pub const CESA_SA_CRYPT_KEY_SRAM_OFFSET: u32 = 0x20;
pub const CESA_SA_CRYPT_IV_SRAM_OFFSET: u32 = 0x40;
pub const CESA_SA_MAC_IIV_SRAM_OFFSET: u32 = 0x20;
pub const CESA_SA_MAC_OIV_SRAM_OFFSET: u32 = 0x40;
pub const CESA_SA_MAC_DIG_SRAM_OFFSET: u32 = 0x60;
pub const fn cesa_sa_desc_crypt_data(offset: u32) -> u32 { (CESA_SA_DATA_SRAM_OFFSET + offset) | ((CESA_SA_DATA_SRAM_OFFSET + offset) << 16) }
pub const fn cesa_sa_desc_crypt_iv(offset: u32) -> u32 { (CESA_SA_CRYPT_IV_SRAM_OFFSET + offset) | ((CESA_SA_CRYPT_IV_SRAM_OFFSET + offset) << 16) }
pub const fn cesa_sa_desc_crypt_key(offset: u32) -> u32 { CESA_SA_CRYPT_KEY_SRAM_OFFSET + offset }
pub const fn cesa_sa_desc_mac_data(offset: u32) -> u32 { CESA_SA_DATA_SRAM_OFFSET + offset }
pub const CESA_SA_DESC_MAC_DATA_MSK: u32 = 0xffff;
pub const fn cesa_sa_desc_mac_total_len(total_len: u32) -> u32 { total_len << 16 }
pub const CESA_SA_DESC_MAC_TOTAL_LEN_MSK: u32 = 0xffff0000;
pub const CESA_SA_DESC_MAC_SRC_TOTAL_LEN_MAX: u32 = 0xffff;
pub const fn cesa_sa_desc_mac_digest(offset: u32) -> u32 { CESA_SA_MAC_DIG_SRAM_OFFSET + offset }
pub const CESA_SA_DESC_MAC_DIGEST_MSK: u32 = 0xffff;
pub const fn cesa_sa_desc_mac_frag_len(frag_len: u32) -> u32 { frag_len << 16 }
pub const CESA_SA_DESC_MAC_FRAG_LEN_MSK: u32 = 0xffff0000;
pub const fn cesa_sa_desc_mac_iv(offset: u32) -> u32 { (CESA_SA_MAC_IIV_SRAM_OFFSET + offset) | ((CESA_SA_MAC_OIV_SRAM_OFFSET + offset) << 16) }
pub const CESA_SA_SRAM_SIZE: usize = 2048;
pub const CESA_SA_DEFAULT_SRAM_SIZE: usize = 2048;
pub const CESA_SA_MIN_SRAM_SIZE: usize = 1024;
pub const CESA_SA_SRAM_MSK: usize = 2047;
pub const CESA_MAX_HASH_BLOCK_SIZE: usize = 64;
pub const CESA_HASH_BLOCK_SIZE_MSK: usize = CESA_MAX_HASH_BLOCK_SIZE - 1;

#[repr(C)] pub struct MvCesaSecAccelDesc { pub config: u32, pub enc_p: u32, pub enc_len: u32, pub enc_key_p: u32, pub enc_iv: u32, pub mac_src_p: u32, pub mac_digest: u32, pub mac_iv: u32 }
#[repr(C)] pub struct MvCesaSkcipherOpCtx { pub key: [u32; 8], pub iv: [u32; 4] }
#[repr(C)] pub struct MvCesaHashOpCtx { pub iv: [u32; 16], pub hash: [u32; 8] }
#[repr(C)] pub union MvCesaOpCtxUnion { pub skcipher: MvCesaSkcipherOpCtx, pub hash: MvCesaHashOpCtx }
#[repr(C)] pub struct MvCesaOpCtx { pub desc: MvCesaSecAccelDesc, pub ctx: MvCesaOpCtxUnion }

pub const CESA_TDMA_DST_IN_SRAM: u32 = 1 << 31;
pub const CESA_TDMA_SRC_IN_SRAM: u32 = 1 << 30;
pub const CESA_TDMA_END_OF_REQ: u32 = 1 << 29;
pub const CESA_TDMA_BREAK_CHAIN: u32 = 1 << 28;
pub const CESA_TDMA_SET_STATE: u32 = 1 << 27;
pub const CESA_TDMA_TYPE_MSK: u32 = 0x07ffffff;
pub const CESA_TDMA_DUMMY: u32 = 0;
pub const CESA_TDMA_DATA: u32 = 1;
pub const CESA_TDMA_OP: u32 = 2;
pub const CESA_TDMA_RESULT: u32 = 3;

#[repr(C)] pub union MvCesaTdmaSrc { pub src: u32, pub src_dma: u32 }
#[repr(C)] pub union MvCesaTdmaDst { pub dst: u32, pub dst_dma: u32 }
#[repr(C)] pub union MvCesaTdmaPayload { pub op: *mut MvCesaOpCtx, pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct MvCesaTdmaDesc { pub byte_cnt: u32, pub source: MvCesaTdmaSrc, pub dest: MvCesaTdmaDst, pub next_dma: u32, pub cur_dma: usize, pub next: *mut MvCesaTdmaDesc, pub payload: MvCesaTdmaPayload, pub flags: u32 }
#[repr(C)] pub struct MvCesaSgDmaIter { pub dir: i32, pub sg: *mut core::ffi::c_void, pub offset: u32, pub op_offset: u32 }
#[repr(C)] pub struct MvCesaDmaIter { pub len: u32, pub offset: u32, pub op_len: u32 }
#[repr(C)] pub struct MvCesaTdmaChain { pub first: *mut MvCesaTdmaDesc, pub last: *mut MvCesaTdmaDesc }

// The remaining declarations depend on Linux kernel types and list/queue helpers.
// They are preserved as opaque external interfaces for consumers supplying those dependencies.
extern "C" {
    pub static mut cesa_dev: *mut MvCesaDev;
    pub fn mv_cesa_queue_req(req: *mut core::ffi::c_void, creq: *mut MvCesaReq) -> i32;
    pub fn mv_cesa_dequeue_req_locked(engine: *mut MvCesaEngine, backlog: *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn mv_cesa_req_dma_iter_next_transfer(chain: *mut MvCesaDmaIter, sgiter: *mut MvCesaSgDmaIter, len: u32) -> bool;
    pub fn mv_cesa_dma_step(dreq: *mut MvCesaReq);
    pub fn mv_cesa_dma_prepare(dreq: *mut MvCesaReq, engine: *mut MvCesaEngine);
    pub fn mv_cesa_dma_cleanup(dreq: *mut MvCesaReq);
    pub fn mv_cesa_tdma_chain(engine: *mut MvCesaEngine, dreq: *mut MvCesaReq);
    pub fn mv_cesa_tdma_process(engine: *mut MvCesaEngine, status: u32) -> i32;
    pub fn mv_cesa_dma_add_result_op(chain: *mut MvCesaTdmaChain, src: usize, size: u32, flags: u32, gfp_flags: u32) -> i32;
    pub fn mv_cesa_dma_add_op(chain: *mut MvCesaTdmaChain, op_templ: *const MvCesaOpCtx, skip_ctx: bool, flags: u32) -> *mut MvCesaOpCtx;
    pub fn mv_cesa_dma_add_data_transfer(chain: *mut MvCesaTdmaChain, dst: usize, src: usize, size: u32, flags: u32, gfp_flags: u32) -> i32;
    pub fn mv_cesa_dma_add_dummy_launch(chain: *mut MvCesaTdmaChain, flags: u32) -> i32;
    pub fn mv_cesa_dma_add_dummy_end(chain: *mut MvCesaTdmaChain, flags: u32) -> i32;
}

#[repr(C)] pub struct MvCesaReq { pub engine: *mut MvCesaEngine, pub chain: MvCesaTdmaChain }
#[repr(C)] pub struct MvCesaSgStdIter { pub iter: [u8; 0], pub offset: u32 }
#[repr(C)] pub struct MvCesaSkcipherStdReq { pub op: MvCesaOpCtx, pub offset: u32, pub size: u32, pub skip_ctx: bool }
#[repr(C)] pub struct MvCesaSkcipherReq { pub base: MvCesaReq, pub std: MvCesaSkcipherStdReq, pub src_nents: i32, pub dst_nents: i32 }
#[repr(C)] pub struct MvCesaAhashStdReq { pub offset: u32 }
#[repr(C)] pub struct MvCesaAhashDmaReq { pub padding: *mut u8, pub padding_dma: usize, pub cache: *mut u8, pub cache_dma: usize }
#[repr(C)] pub union MvCesaAhashReqUnion { pub dma: MvCesaAhashDmaReq, pub std: MvCesaAhashStdReq }
#[repr(C)] pub struct MvCesaAhashReq { pub base: MvCesaReq, pub req: MvCesaAhashReqUnion, pub op_tmpl: MvCesaOpCtx, pub cache: [u8; CESA_MAX_HASH_BLOCK_SIZE], pub cache_ptr: u32, pub len: u64, pub src_nents: i32, pub last_req: bool, pub algo_le: bool, pub state: [u32; 8] }
#[repr(C)] pub struct MvCesaCtx { pub ops: *const MvCesaReqOps }
#[repr(C)] pub struct MvCesaHashCtx { pub base: MvCesaCtx }
#[repr(C)] pub struct MvCesaHmacCtx { pub base: MvCesaCtx, pub iv: [u32; 16] }
#[repr(C)] pub struct MvCesaReqOps { pub process: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32) -> i32>, pub step: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub cleanup: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub complete: Option<unsafe extern "C" fn(*mut core::ffi::c_void)> }
#[repr(C)] pub struct MvCesaEngine { pub _opaque: [u8; 0] }
#[repr(C)] pub struct MvCesaDev { pub _opaque: [u8; 0] }

#[repr(i32)] pub enum MvCesaReqType { CESA_STD_REQ, CESA_DMA_REQ }

pub const fn mv_cesa_req_get_type(req: &MvCesaReq) -> MvCesaReqType { if req.chain.first.is_null() { MvCesaReqType::CESA_STD_REQ } else { MvCesaReqType::CESA_DMA_REQ } }
pub const fn mv_cesa_set_crypt_op_len(op: &mut MvCesaOpCtx, len: i32) { op.desc.enc_len = len as u32; }
pub const fn mv_cesa_set_op_cfg(op: &mut MvCesaOpCtx, cfg: u32) { op.desc.config = cfg; }
pub const fn mv_cesa_get_op_cfg(op: &MvCesaOpCtx) -> u32 { op.desc.config }
pub const fn mv_cesa_set_mac_op_total_len(op: &mut MvCesaOpCtx, len: i32) { op.desc.mac_src_p = (op.desc.mac_src_p & !CESA_SA_DESC_MAC_TOTAL_LEN_MSK) | cesa_sa_desc_mac_total_len(len as u32); }
pub const fn mv_cesa_set_mac_op_frag_len(op: &mut MvCesaOpCtx, len: i32) { op.desc.mac_digest = (op.desc.mac_digest & !CESA_SA_DESC_MAC_FRAG_LEN_MSK) | cesa_sa_desc_mac_frag_len(len as u32); }
pub const fn mv_cesa_mac_op_is_first_frag(op: &MvCesaOpCtx) -> bool { (op.desc.config & CESA_SA_DESC_CFG_FRAG_MSK) == CESA_SA_DESC_CFG_FIRST_FRAG }
pub const fn mv_cesa_req_needs_cleanup(_req: *mut core::ffi::c_void, ret: i32) -> bool { ret != -115 && ret != -16 }
pub const fn mv_cesa_req_dma_iter_init(iter: &mut MvCesaDmaIter, len: u32, payload_size: u32) { iter.len = len; iter.op_len = if len < payload_size { len } else { payload_size }; iter.offset = 0; }
pub const fn mv_cesa_req_dma_iter_next_op(iter: &mut MvCesaDmaIter, payload_size: u32) -> bool { iter.offset += iter.op_len; let rem = iter.len - iter.offset; iter.op_len = if rem < payload_size { rem } else { payload_size }; iter.op_len != 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
