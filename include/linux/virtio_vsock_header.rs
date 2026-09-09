/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// referenced here rather than reimplemented.

pub const VIRTIO_VSOCK_SKB_HEADROOM: usize = core::mem::size_of::<virtio_vsock_hdr>();

#[repr(C)]
pub struct virtio_vsock_skb_cb {
    pub reply: bool,
    pub tap_delivered: bool,
    pub offset: u32,
}

#[inline]
pub unsafe fn VIRTIO_VSOCK_SKB_CB(skb: *mut sk_buff) -> *mut virtio_vsock_skb_cb {
    (*skb).cb.as_mut_ptr() as *mut virtio_vsock_skb_cb
}

#[inline]
pub unsafe fn virtio_vsock_hdr(skb: *mut sk_buff) -> *mut virtio_vsock_hdr {
    (*skb).head as *mut virtio_vsock_hdr
}

#[inline]
pub unsafe fn virtio_vsock_skb_reply(skb: *mut sk_buff) -> bool {
    (*VIRTIO_VSOCK_SKB_CB(skb)).reply
}

#[inline]
pub unsafe fn virtio_vsock_skb_set_reply(skb: *mut sk_buff) {
    (*VIRTIO_VSOCK_SKB_CB(skb)).reply = true;
}

#[inline]
pub unsafe fn virtio_vsock_skb_tap_delivered(skb: *mut sk_buff) -> bool {
    (*VIRTIO_VSOCK_SKB_CB(skb)).tap_delivered
}

#[inline]
pub unsafe fn virtio_vsock_skb_set_tap_delivered(skb: *mut sk_buff) {
    (*VIRTIO_VSOCK_SKB_CB(skb)).tap_delivered = true;
}

#[inline]
pub unsafe fn virtio_vsock_skb_clear_tap_delivered(skb: *mut sk_buff) {
    (*VIRTIO_VSOCK_SKB_CB(skb)).tap_delivered = false;
}

#[inline]
pub unsafe fn virtio_vsock_skb_put(skb: *mut sk_buff, len: u32) {
    DEBUG_NET_WARN_ON_ONCE((*skb).len);
    if skb_is_nonlinear(skb) {
        (*skb).len = len;
    } else {
        skb_put(skb, len);
    }
}

#[inline]
pub unsafe fn __virtio_vsock_alloc_skb_with_frags(
    header_len: u32, data_len: u32, mask: gfp_t,
) -> *mut sk_buff {
    let mut err: i32 = 0;
    let skb = alloc_skb_with_frags(header_len, data_len, PAGE_ALLOC_COSTLY_ORDER, &mut err, mask);
    if skb.is_null() { return core::ptr::null_mut(); }
    skb_reserve(skb, VIRTIO_VSOCK_SKB_HEADROOM as u32);
    (*skb).data_len = data_len;
    skb
}

#[inline]
pub unsafe fn virtio_vsock_alloc_linear_skb(size: u32, mask: gfp_t) -> *mut sk_buff {
    __virtio_vsock_alloc_skb_with_frags(size, 0, mask)
}

#[inline]
pub unsafe fn virtio_vsock_alloc_skb(mut size: u32, mask: gfp_t) -> *mut sk_buff {
    if size as usize <= SKB_WITH_OVERHEAD(PAGE_SIZE << PAGE_ALLOC_COSTLY_ORDER) {
        return virtio_vsock_alloc_linear_skb(size, mask);
    }
    size -= VIRTIO_VSOCK_SKB_HEADROOM as u32;
    __virtio_vsock_alloc_skb_with_frags(VIRTIO_VSOCK_SKB_HEADROOM as u32, size, mask)
}

#[inline]
pub unsafe fn virtio_vsock_skb_queue_head(list: *mut sk_buff_head, skb: *mut sk_buff) {
    spin_lock_bh(&mut (*list).lock);
    __skb_queue_head(list, skb);
    spin_unlock_bh(&mut (*list).lock);
}

#[inline]
pub unsafe fn virtio_vsock_skb_queue_tail(list: *mut sk_buff_head, skb: *mut sk_buff) {
    spin_lock_bh(&mut (*list).lock);
    __skb_queue_tail(list, skb);
    spin_unlock_bh(&mut (*list).lock);
}

#[inline]
pub unsafe fn virtio_vsock_skb_dequeue(list: *mut sk_buff_head) -> *mut sk_buff {
    spin_lock_bh(&mut (*list).lock);
    let skb = __skb_dequeue(list);
    spin_unlock_bh(&mut (*list).lock);
    skb
}

#[inline]
pub unsafe fn virtio_vsock_skb_queue_purge(list: *mut sk_buff_head) {
    spin_lock_bh(&mut (*list).lock);
    __skb_queue_purge(list);
    spin_unlock_bh(&mut (*list).lock);
}

#[inline]
pub unsafe fn virtio_vsock_skb_len(skb: *mut sk_buff) -> usize {
    ((*skb).end_pointer() as usize).wrapping_sub((*skb).head as usize)
}

// Dimension the RX SKB so that the entire thing fits exactly into a single
// 4KiB page, avoiding wasted memory and higher-order pages in the RX queue.
pub const VIRTIO_VSOCK_DEFAULT_RX_BUF_SIZE: usize = SKB_WITH_OVERHEAD(1024 * 4);
pub const VIRTIO_VSOCK_MAX_BUF_SIZE: u64 = 0xFFFFFFFF;
pub const VIRTIO_VSOCK_MAX_PKT_BUF_SIZE: u32 = 1024 * 64;

pub const VSOCK_VQ_RX: u32 = 0;
pub const VSOCK_VQ_TX: u32 = 1;
pub const VSOCK_VQ_EVENT: u32 = 2;
pub const VSOCK_VQ_MAX: u32 = 3;

#[repr(C)]
pub struct virtio_vsock_sock {
    pub vsk: *mut vsock_sock,
    pub tx_lock: spinlock_t,
    pub rx_lock: spinlock_t,
    pub tx_cnt: u32,
    pub peer_fwd_cnt: u32,
    pub peer_buf_alloc: u32,
    pub bytes_unsent: usize,
    pub fwd_cnt: u32,
    pub last_fwd_cnt: u32,
    pub rx_bytes: u32,
    pub buf_alloc: u32,
    pub buf_used: u32,
    pub rx_queue: sk_buff_head,
    pub msg_count: u32,
}

#[repr(C)]
pub struct virtio_vsock_pkt_info {
    pub remote_cid: u32,
    pub remote_port: u32,
    pub vsk: *mut vsock_sock,
    pub msg: *mut msghdr,
    pub net: *mut net,
    pub pkt_len: u32,
    pub type_: u16,
    pub op: u16,
    pub flags: u32,
    pub reply: bool,
}

#[repr(C)]
pub struct virtio_transport {
    pub transport: vsock_transport,
    pub send_pkt: Option<unsafe extern "C" fn(*mut sk_buff, *mut net) -> i32>,
    pub can_msgzerocopy: Option<unsafe extern "C" fn(i32) -> bool>,
}

extern "C" {
    pub fn virtio_transport_stream_dequeue(vsk: *mut vsock_sock, msg: *mut msghdr, len: usize, type_: i32) -> isize;
    pub fn virtio_transport_dgram_dequeue(vsk: *mut vsock_sock, msg: *mut msghdr, len: usize, flags: i32) -> i32;
    pub fn virtio_transport_seqpacket_enqueue(vsk: *mut vsock_sock, msg: *mut msghdr, len: usize) -> i32;
    pub fn virtio_transport_seqpacket_dequeue(vsk: *mut vsock_sock, msg: *mut msghdr, flags: i32) -> isize;
    pub fn virtio_transport_stream_has_data(vsk: *mut vsock_sock) -> i64;
    pub fn virtio_transport_stream_has_space(vsk: *mut vsock_sock) -> i64;
    pub fn virtio_transport_seqpacket_has_data(vsk: *mut vsock_sock) -> u32;
    pub fn virtio_transport_unsent_bytes(vsk: *mut vsock_sock) -> isize;
    pub fn virtio_transport_consume_skb_sent(skb: *mut sk_buff, consume: bool);
    pub fn virtio_transport_do_socket_init(vsk: *mut vsock_sock, psk: *mut vsock_sock) -> i32;
    pub fn virtio_transport_notify_poll_in(vsk: *mut vsock_sock, target: usize, data_ready_now: *mut bool) -> i32;
    pub fn virtio_transport_notify_poll_out(vsk: *mut vsock_sock, target: usize, space_available_now: *mut bool) -> i32;
    pub fn virtio_transport_notify_recv_init(vsk: *mut vsock_sock, target: usize, data: *mut vsock_transport_recv_notify_data) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
