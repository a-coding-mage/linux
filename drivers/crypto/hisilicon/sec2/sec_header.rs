/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2019 HiSilicon Limited. */

// Dependencies supplied by the surrounding translation unit:
// linux/hisi_acc_qm.h and sec_crypto.h

pub const SEC_PBUF_SZ: usize = 512;
pub const SEC_MAX_MAC_LEN: usize = 64;
pub const SEC_IV_SIZE: usize = 24;
pub const SEC_SGE_NR_NUM: usize = 4;
pub const SEC_SGL_ALIGN_SIZE: usize = 64;

/* Algorithm resource per hardware SEC queue */
#[repr(C)]
pub struct sec_alg_res {
    pub pbuf: *mut u8,
    pub pbuf_dma: dma_addr_t,
    pub c_ivin: *mut u8,
    pub c_ivin_dma: dma_addr_t,
    pub a_ivin: *mut u8,
    pub a_ivin_dma: dma_addr_t,
    pub out_mac: *mut u8,
    pub out_mac_dma: dma_addr_t,
    pub depth: u16,
}

#[repr(C)]
pub struct sec_hw_sge {
    pub buf: dma_addr_t,
    pub page_ctrl: *mut core::ffi::c_void,
    pub len: __le32,
    pub pad: __le32,
    pub pad0: __le32,
    pub pad1: __le32,
}

#[repr(C, align(64))]
pub struct sec_hw_sgl {
    pub next_dma: dma_addr_t,
    pub entry_sum_in_chain: __le16,
    pub entry_sum_in_sgl: __le16,
    pub entry_length_in_sgl: __le16,
    pub pad0: __le16,
    pub pad1: [__le64; 5],
    pub next: *mut sec_hw_sgl,
    pub sge_entries: [sec_hw_sge; SEC_SGE_NR_NUM],
}

#[repr(C)]
pub struct sec_src_dst_buf {
    pub in_: sec_hw_sgl,
    pub out: sec_hw_sgl,
}

#[repr(C)]
pub union sec_request_buf_data {
    pub data_buf: core::mem::ManuallyDrop<sec_src_dst_buf>,
    pub pbuf: [u8; SEC_PBUF_SZ],
}

#[repr(C)]
pub struct sec_request_buf {
    pub data: sec_request_buf_data,
    pub in_dma: dma_addr_t,
    pub out_dma: dma_addr_t,
}

/* Cipher request of SEC private */
#[repr(C)]
pub struct sec_cipher_req {
    pub c_out: *mut hisi_acc_hw_sgl,
    pub c_out_dma: dma_addr_t,
    pub c_ivin: *mut u8,
    pub c_ivin_dma: dma_addr_t,
    pub sk_req: *mut skcipher_request,
    pub c_len: u32,
    pub encrypt: bool,
    pub c_ivin_buf: [u8; SEC_IV_SIZE],
}

#[repr(C)]
pub struct sec_aead_req {
    pub out_mac: *mut u8,
    pub out_mac_dma: dma_addr_t,
    pub a_ivin: *mut u8,
    pub a_ivin_dma: dma_addr_t,
    pub aead_req: *mut aead_request,
    pub a_ivin_buf: [u8; SEC_IV_SIZE],
    pub out_mac_buf: [u8; SEC_MAX_MAC_LEN],
}

/* SEC request of Crypto */
#[repr(C)]
pub union sec_req_sqe {
    pub sec_sqe: core::mem::ManuallyDrop<sec_sqe>,
    pub sec_sqe3: core::mem::ManuallyDrop<sec_sqe3>,
}

#[repr(C)]
pub struct sec_req {
    pub sqe: sec_req_sqe,
    pub ctx: *mut sec_ctx,
    pub qp_ctx: *mut sec_qp_ctx,
    pub in_: *mut hisi_acc_hw_sgl,
    pub in_dma: dma_addr_t,
    pub c_req: sec_cipher_req,
    pub aead_req: sec_aead_req,
    pub base: *mut crypto_async_request,
    pub err_type: core::ffi::c_int,
    pub req_id: core::ffi::c_int,
    pub flag: u32,
    pub use_pbuf: bool,
    pub list: list_head,
    pub buf: sec_request_buf,
}

/* SEC request operations */
#[repr(C)]
pub struct sec_req_op {
    pub buf_map: Option<unsafe extern "C" fn(*mut sec_ctx, *mut sec_req) -> core::ffi::c_int>,
    pub buf_unmap: Option<unsafe extern "C" fn(*mut sec_ctx, *mut sec_req)>,
    pub do_transfer: Option<unsafe extern "C" fn(*mut sec_ctx, *mut sec_req)>,
    pub bd_fill: Option<unsafe extern "C" fn(*mut sec_ctx, *mut sec_req) -> core::ffi::c_int>,
    pub bd_send: Option<unsafe extern "C" fn(*mut sec_ctx, *mut sec_req) -> core::ffi::c_int>,
    pub callback: Option<unsafe extern "C" fn(*mut sec_ctx, *mut sec_req, core::ffi::c_int)>,
    pub process: Option<unsafe extern "C" fn(*mut sec_ctx, *mut sec_req) -> core::ffi::c_int>,
}

/* SEC auth context */
#[repr(C)]
pub struct sec_auth_ctx {
    pub a_key_dma: dma_addr_t,
    pub a_key: *mut u8,
    pub a_key_len: u8,
    pub a_alg: u8,
    pub hash_tfm: *mut crypto_shash,
    pub fallback_aead_tfm: *mut crypto_aead,
}

/* SEC cipher context which cipher's relatives */
#[repr(C)]
pub struct sec_cipher_ctx {
    pub c_key: *mut u8,
    pub c_key_dma: dma_addr_t,
    pub iv_offset: sector_t,
    pub c_gran_size: u32,
    pub ivsize: u32,
    pub c_mode: u8,
    pub c_alg: u8,
    pub c_key_len: u8,
    /* add software support */
    pub fallback: bool,
    pub fbtfm: *mut crypto_sync_skcipher,
}

/* SEC queue context which defines queue's relatives */
#[repr(C)]
pub struct sec_qp_ctx {
    pub qp: *mut hisi_qp,
    pub req_list: *mut *mut sec_req,
    pub req_idr: idr,
    pub res: *mut sec_alg_res,
    pub ctx: *mut sec_ctx,
    pub req_lock: spinlock_t,
    pub id_lock: spinlock_t,
    pub c_in_pool: *mut hisi_acc_sgl_pool,
    pub c_out_pool: *mut hisi_acc_sgl_pool,
    pub send_head: u16,
}

#[repr(C)]
pub enum sec_alg_type {
    SEC_SKCIPHER,
    SEC_AEAD,
}

/* SEC Crypto TFM context which defines queue and cipher .etc relatives */
#[repr(C)]
pub struct sec_ctx {
    pub qp_ctx: *mut sec_qp_ctx,
    pub sec: *mut sec_dev,
    pub req_op: *const sec_req_op,
    pub qps: *mut *mut hisi_qp,
    pub enc_qcyclic: atomic_t,
    pub dec_qcyclic: atomic_t,
    pub alg_type: sec_alg_type,
    pub pbuf_supported: bool,
    pub c_ctx: sec_cipher_ctx,
    pub a_ctx: sec_auth_ctx,
    pub type_supported: u8,
    pub dev: *mut device,
}

#[repr(C)]
pub enum sec_debug_file_index {
    SEC_CLEAR_ENABLE,
    SEC_DEBUG_FILE_NUM,
}

#[repr(C)]
pub struct sec_debug_file {
    pub index: sec_debug_file_index,
    pub lock: spinlock_t,
    pub qm: *mut hisi_qm,
}

#[repr(C)]
pub struct sec_dfx {
    pub send_cnt: atomic64_t,
    pub recv_cnt: atomic64_t,
    pub send_busy_cnt: atomic64_t,
    pub recv_busy_cnt: atomic64_t,
    pub err_bd_cnt: atomic64_t,
    pub invalid_req_cnt: atomic64_t,
    pub done_flag_cnt: atomic64_t,
}

#[repr(C)]
pub struct sec_debug {
    pub dfx: sec_dfx,
    pub files: [sec_debug_file; 2],
}

#[repr(C)]
pub struct sec_dev {
    pub qm: hisi_qm,
    pub debug: sec_debug,
    pub ctx_q_num: u32,
    pub iommu_used: bool,
}

#[repr(C)]
pub enum sec_cap_type {
    SEC_QM_NFE_MASK_CAP = 0x0,
    SEC_QM_RESET_MASK_CAP,
    SEC_QM_OOO_SHUTDOWN_MASK_CAP,
    SEC_QM_CE_MASK_CAP,
    SEC_NFE_MASK_CAP,
    SEC_RESET_MASK_CAP,
    SEC_OOO_SHUTDOWN_MASK_CAP,
    SEC_CE_MASK_CAP,
    SEC_CLUSTER_NUM_CAP,
    SEC_CORE_TYPE_NUM_CAP,
    SEC_CORE_NUM_CAP,
    SEC_CORES_PER_CLUSTER_NUM_CAP,
    SEC_CORE_ENABLE_BITMAP,
    SEC_DRV_ALG_BITMAP_LOW,
    SEC_DRV_ALG_BITMAP_HIGH,
    SEC_DEV_ALG_BITMAP_LOW,
    SEC_DEV_ALG_BITMAP_HIGH,
    SEC_CORE1_ALG_BITMAP_LOW,
    SEC_CORE1_ALG_BITMAP_HIGH,
    SEC_CORE2_ALG_BITMAP_LOW,
    SEC_CORE2_ALG_BITMAP_HIGH,
    SEC_CORE3_ALG_BITMAP_LOW,
    SEC_CORE3_ALG_BITMAP_HIGH,
    SEC_CORE4_ALG_BITMAP_LOW,
    SEC_CORE4_ALG_BITMAP_HIGH,
}

#[repr(C)]
pub enum sec_cap_table_type {
    QM_RAS_NFE_TYPE = 0x0,
    QM_RAS_NFE_RESET,
    QM_RAS_CE_TYPE,
    SEC_RAS_NFE_TYPE,
    SEC_RAS_NFE_RESET,
    SEC_RAS_CE_TYPE,
    SEC_CORE_INFO,
    SEC_CORE_EN,
    SEC_DRV_ALG_BITMAP_LOW_TB,
    SEC_DRV_ALG_BITMAP_HIGH_TB,
    SEC_ALG_BITMAP_LOW,
    SEC_ALG_BITMAP_HIGH,
    SEC_CORE1_BITMAP_LOW,
    SEC_CORE1_BITMAP_HIGH,
    SEC_CORE2_BITMAP_LOW,
    SEC_CORE2_BITMAP_HIGH,
    SEC_CORE3_BITMAP_LOW,
    SEC_CORE3_BITMAP_HIGH,
    SEC_CORE4_BITMAP_LOW,
    SEC_CORE4_BITMAP_HIGH,
}

extern "C" {
    pub fn sec_destroy_qps(qps: *mut *mut hisi_qp, qp_num: core::ffi::c_int);
    pub fn sec_create_qps() -> *mut *mut hisi_qp;
    pub fn sec_get_alg_bitmap(qm: *mut hisi_qm, high: u32, low: u32) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
