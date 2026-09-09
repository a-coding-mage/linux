/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2016-2017 HiSilicon Limited. */

/* Translated from sec_drv.h. Kernel-provided types and macros are external dependencies. */

pub const SEC_MAX_SGE_NUM: usize = 64;
pub const SEC_HW_RING_NUM: u32 = 3;
pub const SEC_CMD_RING: u32 = 0;
pub const SEC_OUTORDER_RING: u32 = 1;
pub const SEC_DBG_RING: u32 = 2;
pub const SEC_QUEUE_LEN: usize = 512;
pub const SEC_NAME_SIZE: usize = 64;
pub const SEC_Q_NUM: usize = 16;

#[repr(C)]
pub union SecBdAuthAddr {
    pub authkey_addr_lo: u32,
    pub authiv_addr_lo: u32,
}

#[repr(C)]
pub union SecBdAuthAddrHi {
    pub authkey_addr_hi: u32,
    pub authiv_addr_hi: u32,
}

#[repr(C)]
pub struct sec_bd_info {
    pub w0: u32,
    pub w1: u32,
    pub w2: u32,
    pub w3: u32,
    pub auth_addr_lo: SecBdAuthAddr,
    pub auth_addr_hi: SecBdAuthAddrHi,
    pub cipher_key_addr_lo: u32,
    pub cipher_key_addr_hi: u32,
    pub cipher_iv_addr_lo: u32,
    pub cipher_iv_addr_hi: u32,
    pub data_addr_lo: u32,
    pub data_addr_hi: u32,
    pub mac_addr_lo: u32,
    pub mac_addr_hi: u32,
    pub cipher_destin_addr_lo: u32,
    pub cipher_destin_addr_hi: u32,
}

pub const SEC_BD_W0_T_LEN_M: u32 = 0x1f;
pub const SEC_BD_W0_T_LEN_S: u32 = 0;
pub const SEC_BD_W0_C_WIDTH_M: u32 = 0x60;
pub const SEC_BD_W0_C_WIDTH_S: u32 = 5;
pub const SEC_C_WIDTH_AES_128BIT: u32 = 0;
pub const SEC_C_WIDTH_AES_8BIT: u32 = 1;
pub const SEC_C_WIDTH_AES_1BIT: u32 = 2;
pub const SEC_C_WIDTH_DES_64BIT: u32 = 0;
pub const SEC_C_WIDTH_DES_8BIT: u32 = 1;
pub const SEC_C_WIDTH_DES_1BIT: u32 = 2;
pub const SEC_BD_W0_C_MODE_M: u32 = 0x380;
pub const SEC_BD_W0_C_MODE_S: u32 = 7;
pub const SEC_C_MODE_ECB: u32 = 0;
pub const SEC_C_MODE_CBC: u32 = 1;
pub const SEC_C_MODE_CTR: u32 = 4;
pub const SEC_C_MODE_CCM: u32 = 5;
pub const SEC_C_MODE_GCM: u32 = 6;
pub const SEC_C_MODE_XTS: u32 = 7;
pub const SEC_BD_W0_SEQ: u32 = 1 << 10;
pub const SEC_BD_W0_DE: u32 = 1 << 11;
pub const SEC_BD_W0_DAT_SKIP_M: u32 = 0x3000;
pub const SEC_BD_W0_DAT_SKIP_S: u32 = 12;
pub const SEC_BD_W0_C_GRAN_SIZE_19_16_M: u32 = 0x3c000;
pub const SEC_BD_W0_C_GRAN_SIZE_19_16_S: u32 = 14;
pub const SEC_BD_W0_CIPHER_M: u32 = 0xc0000;
pub const SEC_BD_W0_CIPHER_S: u32 = 18;
pub const SEC_CIPHER_NULL: u32 = 0;
pub const SEC_CIPHER_ENCRYPT: u32 = 1;
pub const SEC_CIPHER_DECRYPT: u32 = 2;
pub const SEC_BD_W0_AUTH_M: u32 = 0x300000;
pub const SEC_BD_W0_AUTH_S: u32 = 20;
pub const SEC_AUTH_NULL: u32 = 0;
pub const SEC_AUTH_MAC: u32 = 1;
pub const SEC_AUTH_VERIF: u32 = 2;
pub const SEC_BD_W0_AI_GEN: u32 = 1 << 22;
pub const SEC_BD_W0_CI_GEN: u32 = 1 << 23;
pub const SEC_BD_W0_NO_HPAD: u32 = 1 << 24;
pub const SEC_BD_W0_HM_M: u32 = 0x6000000;
pub const SEC_BD_W0_HM_S: u32 = 25;
pub const SEC_BD_W0_ICV_OR_SKEY_EN_M: u32 = 0x18000000;
pub const SEC_BD_W0_ICV_OR_SKEY_EN_S: u32 = 27;
pub const SEC_BD_W0_FLAG_M: u32 = 0x60000000;
pub const SEC_BD_W0_C_GRAN_SIZE_21_20_M: u32 = 0x60000000;
pub const SEC_BD_W0_FLAG_S: u32 = 29;
pub const SEC_BD_W0_C_GRAN_SIZE_21_20_S: u32 = 29;
pub const SEC_BD_W0_DONE: u32 = 1 << 31;
pub const SEC_BD_W1_AUTH_GRAN_SIZE_M: u32 = 0x3fffff;
pub const SEC_BD_W1_AUTH_GRAN_SIZE_S: u32 = 0;
pub const SEC_BD_W1_M_KEY_EN: u32 = 1 << 22;
pub const SEC_BD_W1_BD_INVALID: u32 = 1 << 23;
pub const SEC_BD_W1_ADDR_TYPE: u32 = 1 << 24;
pub const SEC_BD_W1_A_ALG_M: u32 = 0x1e000000;
pub const SEC_BD_W1_A_ALG_S: u32 = 25;
pub const SEC_A_ALG_SHA1: u32 = 0;
pub const SEC_A_ALG_SHA256: u32 = 1;
pub const SEC_A_ALG_MD5: u32 = 2;
pub const SEC_A_ALG_SHA224: u32 = 3;
pub const SEC_A_ALG_HMAC_SHA1: u32 = 8;
pub const SEC_A_ALG_HMAC_SHA224: u32 = 10;
pub const SEC_A_ALG_HMAC_SHA256: u32 = 11;
pub const SEC_A_ALG_HMAC_MD5: u32 = 12;
pub const SEC_A_ALG_AES_XCBC: u32 = 13;
pub const SEC_A_ALG_AES_CMAC: u32 = 14;
pub const SEC_BD_W1_C_ALG_M: u32 = 0xe0000000;
pub const SEC_BD_W1_C_ALG_S: u32 = 29;
pub const SEC_C_ALG_DES: u32 = 0;
pub const SEC_C_ALG_3DES: u32 = 1;
pub const SEC_C_ALG_AES: u32 = 2;
pub const SEC_BD_W2_C_GRAN_SIZE_15_0_M: u32 = 0xffff;
pub const SEC_BD_W2_C_GRAN_SIZE_15_0_S: u32 = 0;
pub const SEC_BD_W2_GRAN_NUM_M: u32 = 0xffff0000;
pub const SEC_BD_W2_GRAN_NUM_S: u32 = 16;
pub const SEC_BD_W3_AUTH_LEN_OFFSET_M: u32 = 0x3ff;
pub const SEC_BD_W3_AUTH_LEN_OFFSET_S: u32 = 0;
pub const SEC_BD_W3_CIPHER_LEN_OFFSET_M: u32 = 0xffc00;
pub const SEC_BD_W3_CIPHER_LEN_OFFSET_S: u32 = 10;
pub const SEC_BD_W3_MAC_LEN_M: u32 = 0x1f00000;
pub const SEC_BD_W3_MAC_LEN_S: u32 = 20;
pub const SEC_BD_W3_A_KEY_LEN_M: u32 = 0x3e000000;
pub const SEC_BD_W3_A_KEY_LEN_S: u32 = 25;
pub const SEC_BD_W3_C_KEY_LEN_M: u32 = 0xc0000000;
pub const SEC_BD_W3_C_KEY_LEN_S: u32 = 30;
pub const SEC_KEY_LEN_AES_128: u32 = 0;
pub const SEC_KEY_LEN_AES_192: u32 = 1;
pub const SEC_KEY_LEN_AES_256: u32 = 2;
pub const SEC_KEY_LEN_DES: u32 = 1;
pub const SEC_KEY_LEN_3DES_3_KEY: u32 = 1;
pub const SEC_KEY_LEN_3DES_2_KEY: u32 = 3;

#[repr(i32)]
pub enum sec_mem_region { SEC_COMMON = 0, SEC_SAA, SEC_NUM_ADDR_REGIONS }

#[repr(C)]
pub struct sec_queue_ring_cmd { pub used: atomic_t, pub lock: mutex, pub vaddr: *mut sec_bd_info, pub paddr: dma_addr_t, pub callback: Option<unsafe extern "C" fn(*mut sec_bd_info, *mut core::ffi::c_void)> }
#[repr(C)] pub struct sec_debug_bd_info;
#[repr(C)] pub struct sec_queue_ring_db { pub vaddr: *mut sec_debug_bd_info, pub paddr: dma_addr_t }
#[repr(C)] pub struct sec_out_bd_info;
#[repr(C)] pub struct sec_queue_ring_cq { pub vaddr: *mut sec_out_bd_info, pub paddr: dma_addr_t }

#[repr(i32)]
pub enum sec_cipher_alg {
    SEC_C_DES_ECB_64, SEC_C_DES_CBC_64, SEC_C_3DES_ECB_192_3KEY, SEC_C_3DES_ECB_192_2KEY,
    SEC_C_3DES_CBC_192_3KEY, SEC_C_3DES_CBC_192_2KEY, SEC_C_AES_ECB_128, SEC_C_AES_ECB_192,
    SEC_C_AES_ECB_256, SEC_C_AES_CBC_128, SEC_C_AES_CBC_192, SEC_C_AES_CBC_256,
    SEC_C_AES_CTR_128, SEC_C_AES_CTR_192, SEC_C_AES_CTR_256, SEC_C_AES_XTS_128,
    SEC_C_AES_XTS_256, SEC_C_NULL,
}

#[repr(C)]
pub struct sec_alg_tfm_ctx { pub cipher_alg: sec_cipher_alg, pub key: *mut u8, pub pkey: dma_addr_t, pub req_template: sec_bd_info, pub queue: *mut sec_queue, pub lock: mutex, pub auth_buf: *mut u8, pub backlog: list_head }
#[repr(C)]
pub struct sec_request { pub elements: list_head, pub num_elements: i32, pub lock: mutex, pub tfm_ctx: *mut sec_alg_tfm_ctx, pub len_in: i32, pub len_out: i32, pub dma_iv: dma_addr_t, pub err: i32, pub req_base: *mut crypto_async_request, pub cb: Option<unsafe extern "C" fn(*mut sec_bd_info, *mut crypto_async_request)>, pub backlog_head: list_head }
#[repr(C)]
pub struct sec_request_el { pub head: list_head, pub req: sec_bd_info, pub in_: *mut sec_hw_sgl, pub dma_in: dma_addr_t, pub sgl_in: *mut scatterlist, pub out: *mut sec_hw_sgl, pub dma_out: dma_addr_t, pub sgl_out: *mut scatterlist, pub sec_req: *mut sec_request, pub el_length: usize }
#[repr(C)]
pub struct sec_queue { pub dev_info: *mut sec_dev_info, pub task_irq: i32, pub name: [u8; SEC_NAME_SIZE], pub ring_cmd: sec_queue_ring_cmd, pub ring_cq: sec_queue_ring_cq, pub ring_db: sec_queue_ring_db, pub regs: *mut core::ffi::c_void, pub queue_id: u32, pub in_use: bool, pub expected: i32, pub unprocessed: [u64; SEC_QUEUE_LEN / 64], pub softqueue: *mut core::ffi::c_void, pub havesoftqueue: bool, pub queuelock: spinlock_t, pub shadow: [*mut core::ffi::c_void; SEC_QUEUE_LEN] }
#[repr(C)]
pub struct sec_hw_sge { pub buf: dma_addr_t, pub len: u32, pub pad: u32 }
#[repr(C)]
pub struct sec_hw_sgl { pub next_sgl: dma_addr_t, pub entry_sum_in_chain: u16, pub entry_sum_in_sgl: u16, pub flag: u32, pub serial_num: u64, pub cpuid: u32, pub data_bytes_in_sgl: u32, pub next: *mut sec_hw_sgl, pub reserved: u64, pub sge_entries: [sec_hw_sge; SEC_MAX_SGE_NUM], pub node: [u8; 16] }
#[repr(C)] pub struct dma_pool;
#[repr(C)]
pub struct sec_dev_info { pub sec_id: i32, pub num_saas: i32, pub regs: [*mut core::ffi::c_void; SEC_NUM_ADDR_REGIONS as usize], pub dev_lock: mutex, pub queues_in_use: i32, pub queues: [sec_queue; SEC_Q_NUM], pub dev: *mut device, pub hw_sgl_pool: *mut dma_pool }

extern "C" {
    pub fn sec_queue_send(queue: *mut sec_queue, msg: *mut sec_bd_info, ctx: *mut core::ffi::c_void) -> i32;
    pub fn sec_queue_can_enqueue(queue: *mut sec_queue, num: i32) -> bool;
    pub fn sec_queue_stop_release(queue: *mut sec_queue) -> i32;
    pub fn sec_queue_alloc_start_safe() -> *mut sec_queue;
    pub fn sec_queue_empty(queue: *mut sec_queue) -> bool;
    pub fn sec_alg_callback(resp: *mut sec_bd_info, ctx: *mut core::ffi::c_void);
    pub fn sec_algs_register() -> i32;
    pub fn sec_algs_unregister();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
