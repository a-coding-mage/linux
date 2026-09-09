/* SPDX-License-Identifier: GPL-2.0 */
/* AF_XDP internal functions
 * Copyright(c) 2018 Intel Corporation.
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left as external Rust declarations.

pub const XDP_UMEM_SG_FLAG: u32 = BIT(3);

pub struct net_device;
pub struct xsk_queue;
pub struct xdp_buff;

#[repr(C)]
pub struct xdp_umem {
    pub addrs: *mut core::ffi::c_void,
    pub size: u64,
    pub headroom: u32,
    pub chunk_size: u32,
    pub chunks: u32,
    pub npgs: u32,
    pub user: *mut user_struct,
    pub users: refcount_t,
    pub flags: u8,
    pub tx_metadata_len: u8,
    pub zc: bool,
    pub pgs: *mut *mut page,
    pub id: i32,
    pub xsk_dma_list: list_head,
    pub work: work_struct,
}

#[repr(C)]
pub struct xsk_map {
    pub map: bpf_map,
    pub lock: spinlock_t, /* Synchronize map updates */
    pub count: atomic_t,
    pub xsk_map: [*mut xdp_sock; 0],
}

#[repr(C)]
pub struct xdp_sock {
    /* struct sock must be the first member of struct xdp_sock */
    pub sk: sock,
    pub rx: *mut xsk_queue,
    pub dev: *mut net_device,
    pub umem: *mut xdp_umem,
    pub flush_node: list_head,
    pub pool: *mut xsk_buff_pool,
    pub queue_id: u16,
    pub zc: bool,
    pub sg: bool,
    pub state: xdp_sock_state,

    pub tx: *mut xsk_queue,
    pub tx_list: list_head,
    /* record the number of tx descriptors sent by this xsk and
     * when it exceeds MAX_PER_SOCKET_BUDGET, an opportunity needs
     * to be given to other xsks for sending tx descriptors, thereby
     * preventing other XSKs from being starved.
     */
    pub tx_budget_spent: u32,

    /* Statistics */
    pub rx_dropped: u64,
    pub rx_queue_full: u64,

    /* When __xsk_generic_xmit() must return before it sees the EOP descriptor for the current
     * packet, the partially built skb is saved here so that packet building can resume in next
     * call of __xsk_generic_xmit().
     */
    pub skb: *mut sk_buff,
    pub drain_cont: bool,

    pub map_list: list_head,
    /* Protects map_list */
    pub map_list_lock: spinlock_t,
    pub max_tx_budget: u32,
    /* Protects multiple processes in the control path */
    pub mutex: mutex,
    pub fq_tmp: *mut xsk_queue, /* Only as tmp storage before bind */
    pub cq_tmp: *mut xsk_queue, /* Only as tmp storage before bind */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum xdp_sock_state {
    XSK_READY = 0,
    XSK_BOUND,
    XSK_UNBOUND,
}

/* AF_XDP TX metadata hooks for network devices. */
#[repr(C)]
pub struct xsk_tx_metadata_ops {
    pub tmo_request_timestamp: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void)>,
    pub tmo_fill_timestamp: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void) -> u64>,
    pub tmo_request_checksum: Option<unsafe extern "C" fn(csum_start: u16, csum_offset: u16, priv_: *mut core::ffi::c_void)>,
    pub tmo_request_launch_time: Option<unsafe extern "C" fn(launch_time: u64, priv_: *mut core::ffi::c_void)>,
}

#[cfg(feature = "CONFIG_XDP_SOCKETS")]
extern "C" {
    pub fn xsk_generic_rcv(xs: *mut xdp_sock, xdp: *mut xdp_buff) -> i32;
    pub fn __xsk_map_redirect(xs: *mut xdp_sock, xdp: *mut xdp_buff) -> i32;
    pub fn __xsk_map_flush(flush_list: *mut list_head);
    pub fn xsk_destruct_skb(skb: *mut sk_buff);
}

#[cfg(feature = "CONFIG_XDP_SOCKETS")]
#[inline]
pub unsafe fn xsk_tx_metadata_to_compl(meta: *mut xsk_tx_metadata, compl: *mut xsk_tx_metadata_compl) {
    (*compl).tx_timestamp = core::ptr::null_mut();
    if meta.is_null() { return; }
    (*compl).tx_timestamp = &mut (*meta).completion.tx_timestamp;
}

#[cfg(feature = "CONFIG_XDP_SOCKETS")]
#[inline]
pub unsafe fn xsk_tx_metadata_complete(compl: *mut xsk_tx_metadata_compl, ops: *const xsk_tx_metadata_ops, priv_: *mut core::ffi::c_void) {
    if compl.is_null() || (*compl).tx_timestamp.is_null() { return; }
    *(*compl).tx_timestamp = ((*ops).tmo_fill_timestamp.unwrap())(priv_);
}

#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))]
#[inline]
pub unsafe fn xsk_generic_rcv(_xs: *mut xdp_sock, _xdp: *mut xdp_buff) -> i32 { -ENOTSUPP }

#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))]
#[inline]
pub unsafe fn __xsk_map_redirect(_xs: *mut xdp_sock, _xdp: *mut xdp_buff) -> i32 { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))]
#[inline]
pub unsafe fn __xsk_map_flush(_flush_list: *mut list_head) {}

#[cfg(all(not(feature = "CONFIG_XDP_SOCKETS"), feature = "CONFIG_MITIGATION_RETPOLINE"))]
#[inline]
pub unsafe fn xsk_destruct_skb(_skb: *mut sk_buff) {}

#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))]
#[inline]
pub unsafe fn xsk_tx_metadata_to_compl(_meta: *mut xsk_tx_metadata, _compl: *mut xsk_tx_metadata_compl) {}

#[cfg(not(feature = "CONFIG_XDP_SOCKETS"))]
#[inline]
pub unsafe fn xsk_tx_metadata_complete(_compl: *mut xsk_tx_metadata_compl, _ops: *const xsk_tx_metadata_ops, _priv_: *mut core::ffi::c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
