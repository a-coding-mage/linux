/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translation of request_sock.h. External kernel types and functions are supplied elsewhere. */

use core::ffi::{c_char, c_int, c_void};

// External/kernel-provided types.
#[repr(C)] pub struct sock_common { pub skc_refcnt: refcount_t, pub skc_hash: u32, pub skc_listener: *mut sock, pub skc_window_clamp: u32, pub skc_rcv_wnd: u32 }
#[repr(C)] pub struct sock { pub __sk_common: sock_common, pub sk_state: c_int }
#[repr(C)] pub struct sk_buff { pub sk: *mut sock, pub destructor: Option<unsafe extern "C" fn(*mut sk_buff)> }
#[repr(C)] pub struct dst_entry;
#[repr(C)] pub struct proto;
#[repr(C)] pub struct kmem_cache;
#[repr(C)] pub struct timer_list;
#[repr(C)] pub struct tcp_fastopen_context;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct refcount_t { pub refs: c_int }
pub type bool_ = bool;
pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type enum_sk_rst_reason = c_int;

#[repr(C)]
pub struct request_sock_ops {
    pub family: c_int,
    pub obj_size: u32,
    pub slab: *mut kmem_cache,
    pub slab_name: *mut c_char,
    pub send_ack: Option<unsafe extern "C" fn(*const sock, *mut sk_buff, *mut request_sock)>,
    pub send_reset: Option<unsafe extern "C" fn(*const sock, *mut sk_buff, enum_sk_rst_reason)>,
    pub destructor: Option<unsafe extern "C" fn(*mut request_sock)>,
}

#[repr(C)]
pub struct saved_syn { pub mac_hdrlen: u32, pub network_hdrlen: u32, pub tcp_hdrlen: u32, pub data: [u8; 0] }

#[repr(C)]
pub struct request_sock {
    pub __req_common: sock_common,
    pub dl_next: *mut request_sock,
    pub mss: u16,
    pub syncookie_num_timeout: u8, // C bitfield: syncookie:1, num_timeout:7
    pub ts_recent: u32,
    pub rsk_timer: timer_list,
    pub rsk_ops: *const request_sock_ops,
    pub sk: *mut sock,
    pub saved_syn: *mut saved_syn,
    pub secid: u32,
    pub peer_secid: u32,
    pub timeout: u32,
}

impl request_sock {
    #[inline] pub unsafe fn syncookie(&self) -> u8 { self.syncookie_num_timeout & 1 }
    #[inline] pub unsafe fn num_timeout(&self) -> u8 { self.syncookie_num_timeout >> 1 }
    #[inline] pub unsafe fn rsk_refcnt(&self) -> *const refcount_t { &self.__req_common.skc_refcnt }
    #[inline] pub unsafe fn rsk_listener(&self) -> *mut sock { self.__req_common.skc_listener }
}

#[inline] pub unsafe fn inet_reqsk(sk: *const sock) -> *mut request_sock { sk as *mut request_sock }
#[inline] pub unsafe fn req_to_sk(req: *mut request_sock) -> *mut sock { req as *mut sock }

#[inline]
pub unsafe fn skb_steal_sock(skb: *mut sk_buff, refcounted: *mut bool, prefetched: *mut bool) -> *mut sock {
    let mut sk = (*skb).sk;
    if sk.is_null() { *prefetched = false; *refcounted = false; return core::ptr::null_mut(); }
    *prefetched = skb_sk_is_prefetched(skb);
    if *prefetched {
        // #if IS_ENABLED(CONFIG_SYN_COOKIES): retained as a build-time dependency.
        if (*sk).sk_state == TCP_NEW_SYN_RECV && (*inet_reqsk(sk)).syncookie() != 0 {
            let req = inet_reqsk(sk); *refcounted = false; sk = (*req).rsk_listener(); (*req).__req_common.skc_listener = core::ptr::null_mut(); return sk;
        }
        *refcounted = sk_is_refcounted(sk);
    } else { *refcounted = true; }
    (*skb).destructor = None; (*skb).sk = core::ptr::null_mut(); sk
}

unsafe extern "C" { pub fn __reqsk_free(req: *mut request_sock); pub fn reqsk_fastopen_remove(sk: *mut sock, req: *mut request_sock, reset: bool); }

#[inline] pub unsafe fn reqsk_free(req: *mut request_sock) { DEBUG_NET_WARN_ON_ONCE(refcount_read(&(*req).__req_common.skc_refcnt) != 0); __reqsk_free(req); }
#[inline] pub unsafe fn reqsk_put(req: *mut request_sock) { if refcount_dec_and_test(&mut (*req).__req_common.skc_refcnt) { __reqsk_free(req); } }

#[repr(C)] pub struct fastopen_queue { pub rskq_rst_head: *mut request_sock, pub rskq_rst_tail: *mut request_sock, pub lock: spinlock_t, pub qlen: c_int, pub max_qlen: c_int, pub ctx: *mut tcp_fastopen_context }
#[repr(C)] pub struct request_sock_queue { pub rskq_lock: spinlock_t, pub rskq_defer_accept: u8, pub synflood_warned: u8, pub qlen: atomic_t, pub young: atomic_t, pub rskq_accept_head: *mut request_sock, pub rskq_accept_tail: *mut request_sock, pub fastopenq: fastopen_queue }

#[inline] pub unsafe fn reqsk_queue_empty(queue: *const request_sock_queue) -> bool { READ_ONCE((*queue).rskq_accept_head).is_null() }
#[inline] pub unsafe fn reqsk_queue_remove(queue: *mut request_sock_queue, parent: *mut sock) -> *mut request_sock { spin_lock_bh(&mut (*queue).rskq_lock); let req = (*queue).rskq_accept_head; if !req.is_null() { sk_acceptq_removed(parent); WRITE_ONCE(&mut (*queue).rskq_accept_head, (*req).dl_next); if (*queue).rskq_accept_head.is_null() { (*queue).rskq_accept_tail = core::ptr::null_mut(); } } spin_unlock_bh(&mut (*queue).rskq_lock); req }
#[inline] pub unsafe fn reqsk_queue_removed(queue: *mut request_sock_queue, req: *const request_sock) { if (*req).num_timeout() == 0 { atomic_dec(&mut (*queue).young); } atomic_dec(&mut (*queue).qlen); }
#[inline] pub unsafe fn reqsk_queue_added(queue: *mut request_sock_queue) { atomic_inc(&mut (*queue).young); atomic_inc(&mut (*queue).qlen); }
#[inline] pub unsafe fn reqsk_queue_len(queue: *const request_sock_queue) -> c_int { atomic_read(&(*queue).qlen) }
#[inline] pub unsafe fn reqsk_queue_len_young(queue: *const request_sock_queue) -> c_int { atomic_read(&(*queue).young) }
#[inline] pub unsafe fn tcp_synack_window(req: *const request_sock) -> u32 { core::cmp::min((*req).__req_common.skc_rcv_wnd, 65535) }

// External helpers/constants supplied by the kernel translation environment.
unsafe extern "C" { fn skb_sk_is_prefetched(skb: *mut sk_buff) -> bool; fn sk_is_refcounted(sk: *mut sock) -> bool; fn refcount_read(r: *const refcount_t) -> c_int; fn refcount_dec_and_test(r: *mut refcount_t) -> bool; fn DEBUG_NET_WARN_ON_ONCE(v: bool); fn READ_ONCE<T: Copy>(p: T) -> T; fn WRITE_ONCE<T>(p: *mut T, v: T); fn spin_lock_bh(l: *mut spinlock_t); fn spin_unlock_bh(l: *mut spinlock_t); fn sk_acceptq_removed(sk: *mut sock); fn atomic_inc(a: *mut atomic_t); fn atomic_dec(a: *mut atomic_t); fn atomic_read(a: *const atomic_t) -> c_int; }
pub const TCP_NEW_SYN_RECV: c_int = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
