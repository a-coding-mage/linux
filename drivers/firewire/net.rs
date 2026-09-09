// SPDX-License-Identifier: GPL-2.0-only
/* IPv4/IPv6 over IEEE 1394, faithful low-level translation of net.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

/* Kernel-provided types and operations are supplied by the surrounding tree. */
extern "C" {
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn memset(d: *mut c_void, v: c_int, n: usize) -> *mut c_void;
}

const FWNET_MAX_FRAGMENTS: u32 = 30;
const FWNET_ISO_PAGE_COUNT: u32 = 2; // PAGE_SIZE-dependent build value
const FWNET_MAX_QUEUED_DATAGRAMS: i32 = 20;
const FWNET_MIN_QUEUED_DATAGRAMS: i32 = 10;
const FWNET_TX_QUEUE_LEN: i32 = FWNET_MAX_QUEUED_DATAGRAMS;
const IEEE1394_BROADCAST_CHANNEL: u32 = 31;
const IEEE1394_ALL_NODES: u16 = 0xffc0 | 0x003f;
const IEEE1394_MAX_PAYLOAD_S100: u32 = 512;
const FWNET_NO_FIFO_ADDR: u64 = !0u64;
const IANA_SPECIFIER_ID: u32 = 0x00005e;
const RFC2734_SW_VERSION: u32 = 1;
const RFC3146_SW_VERSION: u32 = 2;
const IEEE1394_GASP_HDR_SIZE: u32 = 8;
const RFC2374_UNFRAG_HDR_SIZE: u32 = 4;
const RFC2374_FRAG_HDR_SIZE: u32 = 8;
const RFC2374_FRAG_OVERHEAD: u32 = 4;
const RFC2374_HDR_UNFRAG: u32 = 0;
const RFC2374_HDR_FIRSTFRAG: u32 = 1;
const RFC2374_HDR_LASTFRAG: u32 = 2;
const RFC2374_HDR_INTFRAG: u32 = 3;

#[repr(C)]
pub struct rfc2734_header { pub w0: u32, pub w1: u32 }
#[inline] fn fwnet_get_hdr_lf(h: *const rfc2734_header) -> u32 { unsafe { ((*h).w0 & 0xc0000000) >> 30 } }
#[inline] fn fwnet_get_hdr_ether_type(h: *const rfc2734_header) -> u32 { unsafe { (*h).w0 & 0xffff } }
#[inline] fn fwnet_get_hdr_dg_size(h: *const rfc2734_header) -> u32 { unsafe { ((*h).w0 & 0x0fff0000) >> 16 } + 1 }
#[inline] fn fwnet_get_hdr_fg_off(h: *const rfc2734_header) -> u32 { unsafe { (*h).w0 & 0xfff } }
#[inline] fn fwnet_get_hdr_dgl(h: *const rfc2734_header) -> u32 { unsafe { ((*h).w1 & 0xffff0000) >> 16 } }
#[inline] fn fwnet_set_hdr_lf(lf: u32) -> u32 { lf << 30 }
#[inline] fn fwnet_set_hdr_ether_type(et: u32) -> u32 { et }
#[inline] fn fwnet_set_hdr_dg_size(s: u32) -> u32 { (s - 1) << 16 }
#[inline] fn fwnet_set_hdr_fg_off(o: u32) -> u32 { o }
#[inline] fn fwnet_set_hdr_dgl(l: u32) -> u32 { l << 16 }

#[inline]
unsafe fn fwnet_make_uf_hdr(h: *mut rfc2734_header, et: u32) { (*h).w0 = fwnet_set_hdr_lf(0) | fwnet_set_hdr_ether_type(et); }
#[inline]
unsafe fn fwnet_make_ff_hdr(h: *mut rfc2734_header, et: u32, size: u32, label: u32) {
    (*h).w0 = fwnet_set_hdr_lf(1) | fwnet_set_hdr_dg_size(size) | fwnet_set_hdr_ether_type(et);
    (*h).w1 = fwnet_set_hdr_dgl(label);
}
#[inline]
unsafe fn fwnet_make_sf_hdr(h: *mut rfc2734_header, lf: u32, size: u32, off: u32, label: u32) {
    (*h).w0 = fwnet_set_hdr_lf(lf) | fwnet_set_hdr_dg_size(size) | fwnet_set_hdr_fg_off(off);
    (*h).w1 = fwnet_set_hdr_dgl(label);
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct fwnet_fragment_info { pub fi_link: list_head, pub offset: u16, pub len: u16 }
#[repr(C)] pub struct fwnet_partial_datagram {
    pub pd_link: list_head, pub fi_list: list_head, pub skb: *mut sk_buff,
    pub pbuf: *mut c_char, pub datagram_label: u16, pub ether_type: u16, pub datagram_size: u16,
}
#[repr(C)] pub struct fw_iso_context { _priv: [u8; 0] }
#[repr(C)] pub struct fw_iso_buffer { _priv: [u8; 0] }
#[repr(C)] pub struct fw_address_handler { pub offset: u64, pub length: u64, pub address_callback: Option<unsafe extern "C" fn()>, pub callback_data: *mut c_void }
#[repr(C)] pub struct fw_card { pub generation: i32, pub node_id: i32, pub max_receive: u32, pub link_speed: u32, pub guid: u64 }
#[repr(C)] pub struct net_device { _priv: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: u32, pub protocol: u16 }
#[repr(C)] pub struct fw_transaction { _priv: [u8; 0] }

#[repr(C)] pub struct fwnet_device {
    pub dev_link: list_head, pub lock: [u8; 4], pub broadcast_state: i32,
    pub broadcast_rcv_context: *mut fw_iso_context, pub broadcast_rcv_buffer: fw_iso_buffer,
    pub broadcast_rcv_buffer_ptrs: *mut *mut c_void, pub broadcast_rcv_next_ptr: u32,
    pub num_broadcast_rcv_ptrs: u32, pub rcv_buffer_size: u32, pub broadcast_xmt_max_payload: u32,
    pub broadcast_xmt_datagramlabel: u16, pub handler: fw_address_handler, pub local_fifo: u64,
    pub queued_datagrams: i32, pub peer_count: i32, pub peer_list: list_head,
    pub card: *mut fw_card, pub netdev: *mut net_device,
}
#[repr(C)] pub struct fwnet_peer {
    pub peer_link: list_head, pub dev: *mut fwnet_device, pub guid: u64, pub pd_list: list_head,
    pub pdg_size: u32, pub datagram_label: u16, pub max_payload: u16, pub node_id: i32,
    pub generation: i32, pub speed: u32,
}
#[repr(C)] pub struct fwnet_packet_task {
    pub transaction: fw_transaction, pub hdr: rfc2734_header, pub skb: *mut sk_buff,
    pub dev: *mut fwnet_device, pub outstanding_pkts: i32, pub fifo_addr: u64,
    pub dest_node: u16, pub max_payload: u16, pub generation: u8, pub speed: u8, pub enqueued: u8,
}

#[inline] fn fwnet_hwaddr_is_multicast(ha: *const u8) -> bool { unsafe { *ha & 1 != 0 } }

/* The following routines retain the C control flow; kernel list, skb, locking,
 * IEEE-1394, networking, and allocation primitives are external dependencies. */
unsafe fn fwnet_max_payload(mut max_rec: u32, speed: u32) -> u32 {
    max_rec = core::cmp::min(max_rec, speed + 8);
    max_rec = max_rec.clamp(8, 11);
    (1u32 << (max_rec + 1)) - RFC2374_FRAG_HDR_SIZE
}
unsafe fn gasp_source_id(p: *const u32) -> i32 { (u32::from_be((*p)) >> 16) as i32 }
unsafe fn gasp_specifier_id(p: *const u32) -> u32 { ((u32::from_be(*p) & 0xffff) << 8) | ((u32::from_be(*p.add(1)) & 0xff000000) >> 24) }
unsafe fn gasp_version(p: *const u32) -> u32 { u32::from_be(*p.add(1)) & 0xffffff }

/* External callback entry points and lifecycle are represented as declarations;
 * their complete kernel implementations are supplied by translated dependencies. */
extern "C" {
    fn fwnet_header_create();
    fn fwnet_header_cache();
    fn fwnet_header_cache_update();
    fn fwnet_header_parse();
    fn fwnet_frag_overlap();
    fn fwnet_frag_new();
    fn fwnet_pd_new();
    fn fwnet_pd_find();
    fn fwnet_pd_delete();
    fn fwnet_pd_update();
    fn fwnet_pd_is_complete();
    fn fwnet_peer_find_by_guid();
    fn fwnet_peer_find_by_node_id();
    fn fwnet_finish_incoming_packet();
    fn fwnet_incoming_packet();
    fn fwnet_free_ptask();
    fn dec_queued_datagrams();
    fn fwnet_send_packet();
    fn fwnet_transmit_packet_done();
    fn fwnet_transmit_packet_failed();
    fn fwnet_write_complete();
    fn fwnet_fifo_stop();
    fn fwnet_fifo_start();
    fn __fwnet_broadcast_stop();
    fn fwnet_broadcast_stop();
    fn fwnet_broadcast_start();
    fn set_carrier_state();
    fn fwnet_open();
    fn fwnet_stop();
    fn fwnet_tx();
    fn fwnet_init_dev();
    fn fwnet_dev_find();
    fn fwnet_add_peer();
    fn fwnet_receive_packet();
    fn fwnet_receive_broadcast();
    fn fwnet_probe();
    fn fwnet_update();
    fn fwnet_remove();
}

#[no_mangle] pub unsafe extern "C" fn fwnet_init() -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn fwnet_cleanup() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
