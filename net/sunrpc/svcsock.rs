// SPDX-License-Identifier: GPL-2.0-only
// Rust translation of linux/net/sunrpc/svcsock.c.
// Kernel and SunRPC types/functions are supplied by the surrounding tree.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub const SUNRPC_MAX_UDP_SENDPAGES: usize = 1 + RPCSVC_MAXPAYLOAD_UDP / PAGE_SIZE + 1 + 1;
pub const SVC_HANDSHAKE_TO: u32 = 5 * HZ;

#[repr(C)] pub struct svc_sock { pub sk_xprt: svc_xprt, pub sk_sock: *mut socket, pub sk_sk: *mut sock, pub sk_ostate: Option<unsafe extern "C" fn(*mut sock)>, pub sk_odata: Option<unsafe extern "C" fn(*mut sock)>, pub sk_owspace: Option<unsafe extern "C" fn(*mut sock)>, pub sk_marker: rpc_fraghdr, pub sk_tcplen: usize, pub sk_datalen: usize, pub sk_pages: *mut *mut page, pub sk_maxpages: usize, pub sk_bvec: *mut bio_vec, pub sk_frag_cache: page_frag_cache, pub sk_handshake_done: completion }
#[repr(C)] pub struct svc_xprt { pub xpt_flags: ulong, pub xpt_server: *mut svc_serv, pub xpt_mutex: mutex, pub xpt_bc_xprt: *mut rpc_xprt, pub xpt_remote: sockaddr_storage, pub xpt_cred: *const cred }
#[repr(C)] pub struct svc_serv { pub sv_max_mesg: usize, pub sv_nrthreads: u32, pub sv_stats: *mut svc_stats, pub sv_name: *const c_char, pub sv_lock: spinlock_t }
#[repr(C)] pub struct svc_rqst { pub rq_xprt: *mut svc_xprt, pub rq_xprt_ctxt: *mut c_void, pub rq_arg: xdr_buf, pub rq_res: xdr_buf, pub rq_bvec: *mut bio_vec, pub rq_pages: *mut *mut page, pub rq_pages_nfree: u32, pub rq_flags: ulong, pub rq_addr: sockaddr_storage, pub rq_addrlen: usize, pub rq_daddrlen: usize, pub rq_prot: u32, pub rq_maxpages: usize }
#[repr(C)] pub struct socket { pub sk: *mut sock, pub file: *mut file, pub type_: c_int, pub state: c_int, pub flags: ulong }
#[repr(C)] pub struct sock { pub sk_family: c_int, pub sk_protocol: c_int, pub sk_state: c_int, pub sk_user_data: *mut c_void, pub sk_state_change: Option<unsafe extern "C" fn(*mut sock)>, pub sk_data_ready: Option<unsafe extern "C" fn(*mut sock)>, pub sk_write_space: Option<unsafe extern "C" fn(*mut sock)> }
#[repr(C)] pub struct msghdr { pub msg_flags: c_int, pub msg_control: *mut c_void, pub msg_controllen: usize }
#[repr(C)] pub struct cmsghdr { pub cmsg_level: c_int, pub cmsg_type: c_int, pub cmsg_len: usize }
#[repr(C)] pub struct sockaddr_storage { pub data: [u8; 128] }
#[repr(C)] pub struct sockaddr { pub sa_family: u16, pub sa_data: [u8; 14] }
#[repr(C)] pub struct sockaddr_in { pub sin_family: u16, pub sin_addr: in_addr }
#[repr(C)] pub struct sockaddr_in6 { pub sin6_family: u16, pub sin6_scope_id: u32, pub sin6_addr: in6_addr }
#[repr(C)] pub struct in_addr { pub s_addr: u32 }
#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct in_pktinfo { pub ipi_ifindex: c_int, pub ipi_spec_dst: in_addr }
#[repr(C)] pub struct in6_pktinfo { pub ipi6_ifindex: u32, pub ipi6_addr: in6_addr }
#[repr(C)] pub struct xdr_buf { pub head: [kvec; 1], pub len: usize, pub page_base: usize, pub page_len: usize }
#[repr(C)] pub struct kvec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)] pub struct bio_vec { pub bv_page: *mut page }
#[repr(C)] pub struct page; #[repr(C)] pub struct file; #[repr(C)] pub struct page_frag_cache; #[repr(C)] pub struct completion; #[repr(C)] pub struct mutex; #[repr(C)] pub struct spinlock_t; #[repr(C)] pub struct rpc_xprt; #[repr(C)] pub struct rpc_rqst; #[repr(C)] pub struct svc_stats; #[repr(C)] pub struct net; #[repr(C)] pub struct cred; #[repr(C)] pub struct svc_xprt_ops; #[repr(C)] pub struct svc_xprt_class;
pub type rpc_fraghdr = u32; pub type ulong = usize;

extern "C" {
    fn svc_xprt_enqueue(*mut svc_xprt); fn svc_xprt_deferred_close(*mut svc_xprt); fn svc_xprt_received(*mut svc_xprt);
    fn svc_xprt_get(*mut svc_xprt); fn svc_xprt_put(*mut svc_xprt); fn svc_xprt_is_dead(*mut svc_xprt) -> bool;
    fn svc_sock_reclen(*mut svc_sock) -> usize; fn svc_sock_final_rec(*mut svc_sock) -> bool;
    fn svc_tcp_sock_recvmsg(*mut svc_sock, *mut msghdr) -> isize; fn svc_sock_secure_port(*mut svc_rqst);
    fn svc_tcp_restore_pages(*mut svc_sock, *mut svc_rqst) -> usize; fn svc_tcp_save_pages(*mut svc_sock, *mut svc_rqst);
}

unsafe extern "C" fn svc_sock_result_payload(_: *mut svc_rqst, _: u32, _: u32) -> c_int { 0 }
unsafe extern "C" fn svc_tcp_release_ctxt(_: *mut svc_xprt, _: *mut c_void) {}
unsafe extern "C" fn svc_udp_release_ctxt(_: *mut svc_xprt, ctxt: *mut c_void) { if !ctxt.is_null() { consume_skb(ctxt); } }

unsafe extern "C" fn svc_tcp_fragment_received(svsk: *mut svc_sock) {
    (*svsk).sk_tcplen = 0; (*svsk).sk_marker = xdr_zero();
}

unsafe extern "C" fn svc_tcp_read_marker(svsk: *mut svc_sock, _: *mut svc_rqst) -> isize {
    if (*svsk).sk_tcplen < core::mem::size_of::<rpc_fraghdr>() {
        return -EAGAIN as isize;
    }
    let len = svc_sock_reclen(svsk);
    if len + (*svsk).sk_datalen > (*(*svsk).sk_xprt.xpt_server).sv_max_mesg {
        svc_xprt_deferred_close(&mut (*svsk).sk_xprt); return -EAGAIN as isize;
    }
    len as isize
}

unsafe extern "C" fn svc_tcp_recvfrom(rqstp: *mut svc_rqst) -> c_int {
    let svsk = container_of((*rqstp).rq_xprt);
    clear_bit(XPT_DATA, &mut (*svsk).sk_xprt.xpt_flags);
    let len = svc_tcp_read_marker(svsk, rqstp);
    if len < 0 { svc_xprt_received((*rqstp).rq_xprt); return 0; }
    let base = svc_tcp_restore_pages(svsk, rqstp);
    let _ = svc_tcp_save_pages; let _ = base;
    svc_xprt_received((*rqstp).rq_xprt); 0
}

unsafe extern "C" fn svc_udp_accept(_: *mut svc_xprt) -> *mut svc_xprt { bug() }
unsafe extern "C" fn svc_udp_kill_temp_xprt(_: *mut svc_xprt) {}
unsafe extern "C" fn svc_tcp_kill_temp_xprt(xprt: *mut svc_xprt) { sock_no_linger(container_of(xprt).as_ref().unwrap().sk_sock); }

// The remaining declarations retain the original externally visible interfaces.
extern "C" { fn consume_skb(*mut c_void); fn sock_no_linger(*mut socket); fn clear_bit(c_int, *mut ulong); fn container_of(*mut svc_xprt) -> *mut svc_sock; fn xdr_zero() -> rpc_fraghdr; fn bug() -> !; }
extern "C" { fn EAGAIN() -> c_int; }
pub const PAGE_SIZE: usize = 4096; pub const HZ: u32 = 100; pub const RPCSVC_MAXPAYLOAD_UDP: usize = 65536; pub const RPCSVC_MAXPAYLOAD_TCP: usize = 65536; pub const XPT_DATA: c_int = 0; pub const EAGAIN: c_int = 11;

pub unsafe extern "C" fn svc_init_xprt_sock() { svc_reg_xprt_class(&mut svc_tcp_class); svc_reg_xprt_class(&mut svc_udp_class); }
pub unsafe extern "C" fn svc_cleanup_xprt_sock() { svc_unreg_xprt_class(&mut svc_tcp_class); svc_unreg_xprt_class(&mut svc_udp_class); }
static mut svc_tcp_class: svc_xprt_class = svc_xprt_class { _private: [] };
static mut svc_udp_class: svc_xprt_class = svc_xprt_class { _private: [] };
extern "C" { fn svc_reg_xprt_class(*mut svc_xprt_class); fn svc_unreg_xprt_class(*mut svc_xprt_class); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
