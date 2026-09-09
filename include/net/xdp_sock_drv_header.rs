/* SPDX-License-Identifier: GPL-2.0 */
/* Interface for implementing AF_XDP zero-copy support in drivers. */

/* Dependencies supplied by the corresponding kernel headers. */
use core::ffi::c_void;

pub const XDP_UMEM_MIN_CHUNK_SHIFT: u32 = 11;
pub const XDP_UMEM_MIN_CHUNK_SIZE: u32 = 1 << XDP_UMEM_MIN_CHUNK_SHIFT;

/* NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT | NETDEV_XDP_ACT_XSK_ZEROCOPY */
pub const NETDEV_XDP_ACT_XSK: u32 =
    NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT | NETDEV_XDP_ACT_XSK_ZEROCOPY;

#[repr(C)]
pub struct xsk_cb_desc {
    pub src: *mut c_void,
    pub off: u8,
    pub bytes: u8,
}

/* External types and constants are supplied by net/xdp_sock.h and net/xsk_buff_pool.h. */
#[repr(C)] pub struct xsk_buff_pool { pub headroom: u32, pub chunk_size: u32, pub umem: *mut xdp_umem, pub dev: *mut c_void, pub unaligned: bool, pub tx_metadata_len: u32 }
#[repr(C)] pub struct xdp_umem { pub flags: u32, pub pgs: *mut c_void, pub npgs: u32 }
#[repr(C)] pub struct xdp_desc { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct xdp_rxq_info { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct xdp_buff { pub data: *mut u8, pub data_meta: *mut u8, pub data_end: *mut u8, pub data_hard_start: *mut u8, pub frame_sz: u32, pub flags: u32 }
#[repr(C)] pub struct xdp_buff_xsk { pub xdp: xdp_buff, _private: [u8; 0] }
#[repr(C)] pub struct xdp_desc_ctx { _private: [u8; 0] }
#[repr(C)] pub struct xsk_tx_metadata { pub flags: u64, pub request: xsk_tx_metadata_request }
#[repr(C)] pub union xsk_tx_metadata_request { pub launch_time: u64, pub csum: xsk_tx_metadata_csum }
#[repr(C)] pub struct xsk_tx_metadata_csum { pub csum_start: u16, pub csum_offset: u16 }
#[repr(C)] pub struct xsk_tx_metadata_ops {
    pub tmo_request_launch_time: Option<unsafe extern "C" fn(u64, *mut c_void)>,
    pub tmo_request_timestamp: Option<unsafe extern "C" fn(*mut c_void)>,
    pub tmo_request_checksum: Option<unsafe extern "C" fn(u16, u16, *mut c_void)>,
}
pub type dma_addr_t = u64;

pub const XDP_UMEM_SG_FLAG: u32 = 1 << 0;
pub const XDP_TXMD_FLAGS_TIMESTAMP: u64 = 1 << 0;
pub const XDP_TXMD_FLAGS_CHECKSUM: u64 = 1 << 1;
pub const XDP_TXMD_FLAGS_LAUNCH_TIME: u64 = 1 << 2;
pub const XDP_TX_METADATA: u32 = 1 << 0;
pub const XDP_PACKET_HEADROOM: u32 = 256;

#[cfg(feature = "CONFIG_XDP_SOCKETS")]
extern "C" {
    pub fn xsk_tx_completed(pool: *mut xsk_buff_pool, nb_entries: u32);
    pub fn xsk_tx_peek_desc(pool: *mut xsk_buff_pool, desc: *mut xdp_desc) -> bool;
    pub fn xsk_tx_peek_release_desc_batch(pool: *mut xsk_buff_pool, max: u32) -> u32;
    pub fn xsk_tx_release(pool: *mut xsk_buff_pool);
    pub fn xsk_get_pool_from_qid(dev: *mut net_device, queue_id: u16) -> *mut xsk_buff_pool;
    pub fn xsk_set_rx_need_wakeup(pool: *mut xsk_buff_pool);
    pub fn xsk_set_tx_need_wakeup(pool: *mut xsk_buff_pool);
    pub fn xsk_clear_rx_need_wakeup(pool: *mut xsk_buff_pool);
    pub fn xsk_clear_tx_need_wakeup(pool: *mut xsk_buff_pool);
    pub fn xsk_uses_need_wakeup(pool: *mut xsk_buff_pool) -> bool;
}

#[inline]
pub unsafe fn xsk_pool_get_headroom(pool: *mut xsk_buff_pool) -> u32 { XDP_PACKET_HEADROOM + (*pool).headroom }
#[inline]
pub unsafe fn xsk_pool_get_chunk_size(pool: *mut xsk_buff_pool) -> u32 { (*pool).chunk_size }
#[inline]
pub unsafe fn __xsk_pool_get_rx_frame_size(pool: *mut xsk_buff_pool) -> u32 { xsk_pool_get_chunk_size(pool) - xsk_pool_get_headroom(pool) }
#[inline]
pub unsafe fn xsk_pool_get_rx_frag_step(pool: *mut xsk_buff_pool) -> u32 { if (*pool).unaligned { 0 } else { xsk_pool_get_chunk_size(pool) } }

/* The following helpers retain the source interfaces; their implementations are external kernel helpers. */
#[cfg(feature = "CONFIG_XDP_SOCKETS")]
extern "C" {
    pub fn xp_set_rxq_info(pool: *mut xsk_buff_pool, rxq: *mut xdp_rxq_info);
    pub fn xp_fill_cb(pool: *mut xsk_buff_pool, desc: *mut xsk_cb_desc);
    pub fn xp_dma_unmap(pool: *mut xsk_buff_pool, attrs: usize);
    pub fn xp_dma_map(pool: *mut xsk_buff_pool, dev: *mut device, attrs: usize, pgs: *mut c_void, npgs: u32) -> i32;
    pub fn xp_get_dma(xskb: *mut xdp_buff_xsk) -> dma_addr_t;
    pub fn xp_get_frame_dma(xskb: *mut xdp_buff_xsk) -> dma_addr_t;
    pub fn xp_alloc(pool: *mut xsk_buff_pool) -> *mut xdp_buff;
    pub fn xp_mb_desc(desc: *const xdp_desc) -> bool;
    pub fn xp_alloc_batch(pool: *mut xsk_buff_pool, xdp: *mut *mut xdp_buff, max: u32) -> u32;
    pub fn xp_can_alloc(pool: *mut xsk_buff_pool, count: u32) -> bool;
    pub fn xp_free(xskb: *mut xdp_buff_xsk);
    pub fn xp_raw_get_dma(pool: *mut xsk_buff_pool, addr: u64) -> dma_addr_t;
    pub fn xp_raw_get_data(pool: *mut xsk_buff_pool, addr: u64) -> *mut u8;
    pub fn xp_raw_get_ctx(pool: *const xsk_buff_pool, addr: u64, options: u32) -> xdp_desc_ctx;
    pub fn xp_dma_sync_for_cpu(xskb: *mut xdp_buff_xsk);
    pub fn xp_dma_sync_for_device(pool: *mut xsk_buff_pool, dma: dma_addr_t, size: usize);
}

#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))]
pub unsafe fn xsk_tx_completed(_pool: *mut xsk_buff_pool, _nb_entries: u32) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_tx_peek_desc(_pool: *mut xsk_buff_pool, _desc: *mut xdp_desc) -> bool { false }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_tx_peek_release_desc_batch(_pool: *mut xsk_buff_pool, _max: u32) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_tx_release(_pool: *mut xsk_buff_pool) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_get_pool_from_qid(_dev: *mut net_device, _queue_id: u16) -> *mut xsk_buff_pool { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_set_rx_need_wakeup(_pool: *mut xsk_buff_pool) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_set_tx_need_wakeup(_pool: *mut xsk_buff_pool) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_clear_rx_need_wakeup(_pool: *mut xsk_buff_pool) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_clear_tx_need_wakeup(_pool: *mut xsk_buff_pool) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_uses_need_wakeup(_pool: *mut xsk_buff_pool) -> bool { false }

/* CONFIG_XDP_SOCKETS fallback helpers from the header. */
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_pool_get_headroom(_pool: *mut xsk_buff_pool) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_pool_get_chunk_size(_pool: *mut xsk_buff_pool) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_pool_get_rx_frame_size(_pool: *mut xsk_buff_pool) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_pool_get_rx_frag_step(_pool: *mut xsk_buff_pool) -> u32 { 0 }

#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))]
pub unsafe fn xsk_pool_set_rxq_info(_pool: *mut xsk_buff_pool, _rxq: *mut xdp_rxq_info) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_pool_fill_cb(_pool: *mut xsk_buff_pool, _desc: *mut xsk_cb_desc) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_pool_dma_unmap(_pool: *mut xsk_buff_pool, _attrs: usize) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_pool_dma_map(_pool: *mut xsk_buff_pool, _dev: *mut device, _attrs: usize) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_xdp_get_dma(_xdp: *mut xdp_buff) -> dma_addr_t { 0 }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_xdp_get_frame_dma(_xdp: *mut xdp_buff) -> dma_addr_t { 0 }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_alloc(_pool: *mut xsk_buff_pool) -> *mut xdp_buff { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_is_eop_desc(_desc: *const xdp_desc) -> bool { false }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_alloc_batch(_pool: *mut xsk_buff_pool, _xdp: *mut *mut xdp_buff, _max: u32) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_can_alloc(_pool: *mut xsk_buff_pool, _count: u32) -> bool { false }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_free(_xdp: *mut xdp_buff) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_add_frag(_head: *mut xdp_buff, _xdp: *mut xdp_buff) -> bool { false }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_get_frag(_first: *const xdp_buff) -> *mut xdp_buff { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_del_frag(_xdp: *mut xdp_buff) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_get_head(_first: *mut xdp_buff) -> *mut xdp_buff { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_get_tail(_first: *mut xdp_buff) -> *mut xdp_buff { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_set_size(_xdp: *mut xdp_buff, _size: u32) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_raw_get_dma(_pool: *mut xsk_buff_pool, _addr: u64) -> dma_addr_t { 0 }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_raw_get_data(_pool: *mut xsk_buff_pool, _addr: u64) -> *mut c_void { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_raw_get_ctx(_pool: *const xsk_buff_pool, _addr: u64, _options: u32) -> xdp_desc_ctx { xdp_desc_ctx { _private: [] } }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_valid_tx_metadata(_pool: *const xsk_buff_pool, _meta: *const xsk_tx_metadata, _flags: *mut u64) -> bool { false }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_tx_metadata_request(_pool: *const xsk_buff_pool, _pmeta: *mut *mut xsk_tx_metadata, _ops: *const xsk_tx_metadata_ops, _priv: *mut c_void) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn __xsk_buff_get_metadata(_pool: *const xsk_buff_pool, _data: *mut c_void, _options: u32) -> *mut xsk_tx_metadata { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_get_metadata(_pool: *mut xsk_buff_pool, _addr: u64, _options: u32) -> *mut xsk_tx_metadata { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_dma_sync_for_cpu(_xdp: *mut xdp_buff) {}
#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))] pub unsafe fn xsk_buff_raw_dma_sync_for_device(_pool: *mut xsk_buff_pool, _dma: dma_addr_t, _size: usize) {}

/* Enabled-configuration bodies delegate to the corresponding kernel pool primitives. */
#[cfg(feature = "CONFIG_XDP_SOCKETS")]
extern "C" {
    pub fn xsk_pool_set_rxq_info(pool: *mut xsk_buff_pool, rxq: *mut xdp_rxq_info);
    pub fn xsk_pool_fill_cb(pool: *mut xsk_buff_pool, desc: *mut xsk_cb_desc);
    pub fn xsk_pool_dma_unmap(pool: *mut xsk_buff_pool, attrs: usize);
    pub fn xsk_pool_dma_map(pool: *mut xsk_buff_pool, dev: *mut device, attrs: usize) -> i32;
    pub fn xsk_buff_xdp_get_dma(xdp: *mut xdp_buff) -> dma_addr_t;
    pub fn xsk_buff_xdp_get_frame_dma(xdp: *mut xdp_buff) -> dma_addr_t;
    pub fn xsk_buff_alloc(pool: *mut xsk_buff_pool) -> *mut xdp_buff;
    pub fn xsk_is_eop_desc(desc: *const xdp_desc) -> bool;
    pub fn xsk_buff_alloc_batch(pool: *mut xsk_buff_pool, xdp: *mut *mut xdp_buff, max: u32) -> u32;
    pub fn xsk_buff_can_alloc(pool: *mut xsk_buff_pool, count: u32) -> bool;
    pub fn xsk_buff_free(xdp: *mut xdp_buff);
    pub fn xsk_buff_add_frag(head: *mut xdp_buff, xdp: *mut xdp_buff) -> bool;
    pub fn xsk_buff_get_frag(first: *const xdp_buff) -> *mut xdp_buff;
    pub fn xsk_buff_del_frag(xdp: *mut xdp_buff);
    pub fn xsk_buff_get_head(first: *mut xdp_buff) -> *mut xdp_buff;
    pub fn xsk_buff_get_tail(first: *mut xdp_buff) -> *mut xdp_buff;
    pub fn xsk_buff_set_size(xdp: *mut xdp_buff, size: u32);
    pub fn xsk_buff_raw_get_dma(pool: *mut xsk_buff_pool, addr: u64) -> dma_addr_t;
    pub fn xsk_buff_raw_get_data(pool: *mut xsk_buff_pool, addr: u64) -> *mut c_void;
    pub fn xsk_buff_raw_get_ctx(pool: *const xsk_buff_pool, addr: u64, options: u32) -> xdp_desc_ctx;
    pub fn xsk_buff_valid_tx_metadata(pool: *const xsk_buff_pool, meta: *const xsk_tx_metadata, flags: *mut u64) -> bool;
    pub fn xsk_tx_metadata_request(pool: *const xsk_buff_pool, pmeta: *mut *mut xsk_tx_metadata, ops: *const xsk_tx_metadata_ops, priv_: *mut c_void);
    pub fn __xsk_buff_get_metadata(pool: *const xsk_buff_pool, data: *mut c_void, options: u32) -> *mut xsk_tx_metadata;
    pub fn xsk_buff_get_metadata(pool: *mut xsk_buff_pool, addr: u64, options: u32) -> *mut xsk_tx_metadata;
    pub fn xsk_buff_dma_sync_for_cpu(xdp: *mut xdp_buff);
    pub fn xsk_buff_raw_dma_sync_for_device(pool: *mut xsk_buff_pool, dma: dma_addr_t, size: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
