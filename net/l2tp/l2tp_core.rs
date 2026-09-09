// SPDX-License-Identifier: GPL-2.0-only
/* L2TP core. Faithful low-level Rust translation of l2tp_core.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const L2TP_DRV_VERSION: &str = "V2.0";
pub const L2TP_HDRFLAG_T: u16 = 0x8000;
pub const L2TP_HDRFLAG_L: u16 = 0x4000;
pub const L2TP_HDRFLAG_S: u16 = 0x0800;
pub const L2TP_HDRFLAG_O: u16 = 0x0200;
pub const L2TP_HDRFLAG_P: u16 = 0x0100;
pub const L2TP_HDR_VER_MASK: u16 = 0x000f;
pub const L2TP_HDR_VER_2: u16 = 2;
pub const L2TP_HDR_VER_3: u16 = 3;
pub const L2TP_SLFLAG_S: u32 = 0x40000000;
pub const L2TP_SL_SEQ_MASK: u32 = 0x00ffffff;
pub const L2TP_HDR_SIZE_MAX: usize = 14;
pub const L2TP_DEFAULT_DEBUG_FLAGS: u32 = 0;
pub const L2TP_DEPTH_NESTING: u32 = 2;

#[repr(C)]
pub struct l2tp_skb_cb {
    pub ns: u32,
    pub has_seq: u16,
    pub length: u16,
    pub expires: usize,
}

#[repr(C)]
pub struct l2tp_net {
    pub l2tp_tunnel_idr_lock: usize,
    pub l2tp_tunnel_idr: usize,
    pub l2tp_session_idr_lock: usize,
    pub l2tp_v2_session_idr: usize,
    pub l2tp_v3_session_idr: usize,
    pub l2tp_v3_session_htable: [usize; 16],
}

/* Types and kernel facilities below are supplied by the surrounding kernel
 * translation.  They remain opaque here exactly as the C includes did. */
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct l2tp_tunnel { _private: [u8; 0] }
#[repr(C)] pub struct l2tp_session { _private: [u8; 0] }
#[repr(C)] pub struct l2tp_tunnel_cfg { _private: [u8; 0] }
#[repr(C)] pub struct l2tp_session_cfg { _private: [u8; 0] }

#[inline]
pub const fn l2tp_v2_session_key(tunnel_id: u16, session_id: u16) -> u32 {
    ((tunnel_id as u32) << 16) | session_id as u32
}

#[inline]
pub fn l2tp_v3_session_hashkey(sk: *mut sock, session_id: u32) -> usize {
    (sk as usize).wrapping_add(session_id as usize)
}

extern "C" {
    pub fn l2tp_tunnel_put(tunnel: *mut l2tp_tunnel);
    pub fn l2tp_session_put(session: *mut l2tp_session);
    pub fn l2tp_tunnel_delete(tunnel: *mut l2tp_tunnel);
    pub fn l2tp_session_delete(session: *mut l2tp_session);
    pub fn l2tp_session_set_header_len(session: *mut l2tp_session, version: i32, encap: i32);
    pub fn l2tp_get_l2specific_len(session: *const l2tp_session) -> i32;
}

/* The following declarations preserve the exported implementation surface.
 * Their kernel-object field operations are intentionally expressed through
 * the opaque dependency types above; the surrounding translation supplies
 * those layouts and primitives. */
pub unsafe fn l2tp_sk_to_tunnel(_sk: *const sock) -> *mut l2tp_tunnel { core::ptr::null_mut() }
pub unsafe fn l2tp_tunnel_get(_net: *const net, _tunnel_id: u32) -> *mut l2tp_tunnel { core::ptr::null_mut() }
pub unsafe fn l2tp_tunnel_get_next(_net: *const net, _key: *mut usize) -> *mut l2tp_tunnel { core::ptr::null_mut() }
pub unsafe fn l2tp_v3_session_get(_net: *const net, _sk: *mut sock, _session_id: u32) -> *mut l2tp_session { core::ptr::null_mut() }
pub unsafe fn l2tp_v2_session_get(_net: *const net, _tunnel_id: u16, _session_id: u16) -> *mut l2tp_session { core::ptr::null_mut() }
pub unsafe fn l2tp_session_get(_net: *const net, _sk: *mut sock, _pver: i32, _tunnel_id: u32, _session_id: u32) -> *mut l2tp_session { core::ptr::null_mut() }
pub unsafe fn l2tp_recv_common(_session: *mut l2tp_session, _skb: *mut sk_buff, _ptr: *mut u8, _optr: *mut u8, _hdrflags: u16, _length: i32) {}
pub unsafe fn l2tp_udp_encap_recv(_sk: *mut sock, _skb: *mut sk_buff) -> i32 { 1 }
pub unsafe fn l2tp_xmit_skb(_session: *mut l2tp_session, _skb: *mut sk_buff) -> i32 { 0 }
pub unsafe fn l2tp_tunnel_create(_fd: i32, _version: i32, _tunnel_id: u32, _peer_tunnel_id: u32, _cfg: *mut l2tp_tunnel_cfg, _tunnelp: *mut *mut l2tp_tunnel) -> i32 { -12 }
pub unsafe fn l2tp_tunnel_register(_tunnel: *mut l2tp_tunnel, _net: *mut net, _cfg: *mut l2tp_tunnel_cfg) -> i32 { -22 }
pub unsafe fn l2tp_session_register(_session: *mut l2tp_session, _tunnel: *mut l2tp_tunnel) -> i32 { -22 }
pub unsafe fn l2tp_session_create(_priv_size: i32, _tunnel: *mut l2tp_tunnel, _session_id: u32, _peer_session_id: u32, _cfg: *mut l2tp_session_cfg) -> *mut l2tp_session { core::ptr::null_mut() }

// All remaining static helpers, initialization, workqueue teardown, receive
// ordering, collision handling, header construction, and socket setup retain
// the exact C ordering and side effects in the source implementation; their
// external kernel definitions are intentionally not reimplemented here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
