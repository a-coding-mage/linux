/*
 * Copyright (c) 2016-2017, Mellanox Technologies. All rights reserved.
 * Copyright (c) 2016-2017, Dave Watson <davejwatson@fb.com>. All rights reserved.
 *
 * This software is available under a choice of one of two licenses. You may
 * choose to be licensed under the terms of the GNU General Public License
 * (GPL) Version 2, or the OpenIB.org BSD license.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const TLS_MAX_PAYLOAD_SIZE: usize = 1usize << 14;
pub const TLS_MIN_RECORD_SIZE_LIM: usize = 1usize << 6;
pub const TLS_HEADER_SIZE: usize = 5;
pub const TLS_NONCE_OFFSET: usize = TLS_HEADER_SIZE;
pub const TLS_HANDSHAKE_KEYUPDATE: u32 = 24;
pub const TLS_AAD_SPACE_SIZE: usize = 13;
pub const TLS_MAX_IV_SIZE: usize = 16;
pub const TLS_MAX_SALT_SIZE: usize = 4;
pub const TLS_TAG_SIZE: usize = 16;
pub const TLS_MAX_REC_SEQ_SIZE: usize = 8;
pub const TLS_MAX_AAD_SIZE: usize = TLS_AAD_SPACE_SIZE;
pub const TLS_AES_CCM_IV_B0_BYTE: u8 = 2;
pub const TLS_SM4_CCM_IV_B0_BYTE: u8 = 2;

pub const TLS_BASE: u32 = 0;
pub const TLS_SW: u32 = 1;
pub const TLS_HW: u32 = 2;
pub const TLS_NUM_CONFIG: u32 = 3;

pub const BIT_TX_SCHEDULED: u32 = 0;
pub const BIT_TX_CLOSING: u32 = 1;
pub const TLS_DRIVER_STATE_SIZE_TX: usize = 16;
pub const TLS_RX_DEV_DEGRADED: u32 = 0;
pub const TLS_TX_SYNC_SCHED: u32 = 1;
pub const TLS_RX_DEV_CLOSED: u32 = 2;
pub const TLS_DEVICE_RESYNC_NH_START_IVAL: u32 = 2;
pub const TLS_DEVICE_RESYNC_NH_MAX_IVAL: u32 = 128;
pub const TLS_DEVICE_RESYNC_ASYNC_LOGMAX: usize = 13;
pub const TLS_DRIVER_STATE_SIZE_RX: usize = 8;
pub const RESYNC_REQ: u64 = 1;
pub const RESYNC_REQ_ASYNC: u64 = 2;

#[repr(C)] pub struct tls_rec;

#[repr(C)] pub struct tx_work { pub work: delayed_work, pub sk: *mut sock }

#[repr(C)] pub struct tls_sw_context_tx {
    pub aead_send: *mut crypto_aead, pub async_wait: crypto_wait, pub tx_work: tx_work,
    pub open_rec: *mut tls_rec, pub tx_list: list_head, pub encrypt_pending: atomic_t,
    pub async_capable: u8, pub tx_bitmask: c_ulong,
}

#[repr(C)] pub struct tls_strparser {
    pub sk: *mut sock, pub mark: u32, pub stopped: u32, pub copy_mode: u32,
    pub mixed_decrypted: u32, pub msg_announced: u32, pub msg_ready: bool,
    pub stm: strp_msg, pub anchor: *mut sk_buff, pub work: work_struct,
}

#[repr(C)] pub struct tls_sw_context_rx {
    pub aead_recv: *mut crypto_aead, pub async_wait: crypto_wait, pub rx_list: sk_buff_head,
    pub saved_data_ready: Option<unsafe extern "C" fn(*mut sock)>, pub reader_present: u8,
    pub async_capable: u8, pub zc_capable: u8, pub reader_contended: u8,
    pub key_update_pending: bool, pub strp: tls_strparser, pub decrypt_pending: atomic_t,
    pub async_hold: sk_buff_head, pub wq: wait_queue_head,
}

#[repr(C)] pub struct tls_record_info {
    pub list: list_head, pub end_seq: u32, pub len: c_int, pub num_frags: c_int,
    pub frags: [skb_frag_t; MAX_SKB_FRAGS],
}

#[repr(C)] pub struct tls_offload_context_tx {
    pub aead_send: *mut crypto_aead, pub lock: spinlock_t, pub records_list: list_head,
    pub open_record: *mut tls_record_info, pub retransmit_hint: *mut tls_record_info,
    pub hint_record_sn: u64, pub unacked_record_sn: u64,
    pub sg_tx_data: [scatterlist; MAX_SKB_FRAGS],
    pub sk_destruct: Option<unsafe extern "C" fn(*mut sock)>, pub destruct_work: work_struct,
    pub ctx: *mut tls_context, pub driver_state: [u8; TLS_DRIVER_STATE_SIZE_TX],
}

#[repr(C)] pub struct cipher_context { pub iv: [c_char; TLS_MAX_IV_SIZE + TLS_MAX_SALT_SIZE], pub rec_seq: [c_char; TLS_MAX_REC_SEQ_SIZE] }

#[repr(C)] pub union tls_crypto_context {
    pub info: tls_crypto_info,
    pub aes_gcm_128: tls12_crypto_info_aes_gcm_128,
    pub aes_gcm_256: tls12_crypto_info_aes_gcm_256,
    pub chacha20_poly1305: tls12_crypto_info_chacha20_poly1305,
    pub sm4_gcm: tls12_crypto_info_sm4_gcm,
    pub sm4_ccm: tls12_crypto_info_sm4_ccm,
}

#[repr(C)] pub struct tls_prot_info { pub version: u16, pub cipher_type: u16, pub prepend_size: u16, pub tag_size: u16, pub overhead_size: u16, pub iv_size: u16, pub salt_size: u16, pub rec_seq_size: u16, pub aad_size: u16, pub tail_size: u16 }

#[repr(C)] pub struct tls_context {
    pub prot_info: tls_prot_info, pub tx_conf: u8, pub rx_conf: u8, pub zerocopy_sendfile: u8,
    pub rx_no_pad: u8, pub tx_max_payload_len: u16,
    pub push_pending_record: Option<unsafe extern "C" fn(*mut sock, c_int) -> c_int>,
    pub sk_write_space: Option<unsafe extern "C" fn(*mut sock)>, pub priv_ctx_tx: *mut c_void,
    pub priv_ctx_rx: *mut c_void, pub netdev: *mut net_device, pub tx: cipher_context,
    pub rx: cipher_context, pub partially_sent_record: *mut scatterlist, pub partially_sent_offset: u16,
    pub splicing_pages: bool, pub pending_open_record_frags: bool, pub tx_lock: mutex,
    pub flags: c_ulong, pub sk_proto: *mut proto, pub sk: *mut sock,
    pub sk_destruct: Option<unsafe extern "C" fn(*mut sock)>, pub crypto_send: tls_crypto_context,
    pub crypto_recv: tls_crypto_context, pub list: list_head, pub refcount: refcount_t, pub rcu: rcu_head,
}

pub const TLS_OFFLOAD_CTX_DIR_RX: u32 = 0;
pub const TLS_OFFLOAD_CTX_DIR_TX: u32 = 1;
pub const TLS_OFFLOAD_SYNC_TYPE_DRIVER_REQ: u32 = 0;
pub const TLS_OFFLOAD_SYNC_TYPE_CORE_NEXT_HINT: u32 = 1;
pub const TLS_OFFLOAD_SYNC_TYPE_DRIVER_REQ_ASYNC: u32 = 2;

#[repr(C)] pub struct tlsdev_ops {
    pub tls_dev_add: Option<unsafe extern "C" fn(*mut net_device, *mut sock, u32, *mut tls_crypto_info, u32) -> c_int>,
    pub tls_dev_del: Option<unsafe extern "C" fn(*mut net_device, *mut tls_context, u32)>,
    pub tls_dev_resync: Option<unsafe extern "C" fn(*mut net_device, *mut sock, u32, *mut u8, u32) -> c_int>,
}
#[repr(C)] pub struct tls_offload_resync_async { pub req: atomic64_t, pub loglen: u16, pub rcd_delta: u16, pub log: [u32; TLS_DEVICE_RESYNC_ASYNC_LOGMAX] }

#[repr(C)] pub struct tls_offload_context_rx { pub sw: tls_sw_context_rx, pub resync_type: u32, pub resync_nh_reset: u8, pub resync_nh_do_now: u8, pub resync: tls_offload_resync_union, pub driver_state: [u8; TLS_DRIVER_STATE_SIZE_RX] }
#[repr(C)] pub union tls_offload_resync_union { pub resync_req: atomic64_t, pub resync_nh: tls_resync_nh, pub resync_async: *mut tls_offload_resync_async }
#[repr(C)] pub struct tls_resync_nh { pub decrypted_failed: u32, pub decrypted_tgt: u32 }

extern "C" {
    pub fn tls_get_record(context: *mut tls_offload_context_tx, seq: u32, p_record_sn: *mut u64) -> *mut tls_record_info;
    pub fn tls_validate_xmit_skb(sk: *mut sock, dev: *mut net_device, skb: *mut sk_buff) -> *mut sk_buff;
    pub fn tls_validate_xmit_skb_sw(sk: *mut sock, dev: *mut net_device, skb: *mut sk_buff) -> *mut sk_buff;
    pub fn tls_encrypt_skb(skb: *mut sk_buff) -> *mut sk_buff;
}

#[inline] pub unsafe fn tls_record_is_start_marker(rec: *mut tls_record_info) -> bool { (*rec).len == 0 }
#[inline] pub unsafe fn tls_record_start_seq(rec: *mut tls_record_info) -> u32 { (*rec).end_seq.wrapping_sub((*rec).len as u32) }
#[inline] pub unsafe fn tls_sw_ctx_rx(ctx: *const tls_context) -> *mut tls_sw_context_rx { (*ctx).priv_ctx_rx as *mut tls_sw_context_rx }
#[inline] pub unsafe fn tls_sw_ctx_tx(ctx: *const tls_context) -> *mut tls_sw_context_tx { (*ctx).priv_ctx_tx as *mut tls_sw_context_tx }
#[inline] pub unsafe fn tls_offload_ctx_tx(ctx: *const tls_context) -> *mut tls_offload_context_tx { (*ctx).priv_ctx_tx as *mut tls_offload_context_tx }

#[inline] pub unsafe fn tls_offload_ctx_rx(ctx: *const tls_context) -> *mut tls_offload_context_rx { (*ctx).priv_ctx_rx as *mut tls_offload_context_rx }
#[inline] pub unsafe fn __tls_driver_ctx(ctx: *mut tls_context, direction: u32) -> *mut u8 {
    if direction == TLS_OFFLOAD_CTX_DIR_TX { (*tls_offload_ctx_tx(ctx)).driver_state.as_mut_ptr() }
    else { (*tls_offload_ctx_rx(ctx)).driver_state.as_mut_ptr() }
}

// The following helpers preserve the header's externally supplied kernel
// predicates and atomic operations.
extern "C" {
    pub fn tls_get_ctx(sk: *const sock) -> *mut tls_context;
    pub fn tls_offload_rx_resync_request(sk: *mut sock, seq: __be32);
    pub fn tls_offload_rx_resync_async_request_start(r: *mut tls_offload_resync_async, seq: __be32, len: u16);
    pub fn tls_offload_rx_resync_async_request_end(r: *mut tls_offload_resync_async, seq: __be32);
    pub fn tls_offload_rx_resync_async_request_cancel(r: *mut tls_offload_resync_async);
    pub fn tls_offload_rx_resync_set_type(sk: *mut sock, ty: u32);
    pub fn tls_offload_tx_resync_pending(sk: *mut sock) -> bool;
}

// CONFIG_TLS_DEVICE-dependent helpers and kernel synchronization operations are
// declared here; their implementations are supplied by the translated kernel.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
