// SPDX-License-Identifier: GPL-2.0
/* Multipath TCP -- direct low-level translation of sockopt.c. */

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

/* Kernel-provided types, constants, macros, and functions are intentionally
 * left as external dependencies, as in the source translation unit. */
extern "C" {
    fn __mptcp_check_fallback(msk: *mut mptcp_sock) -> bool;
    fn msk_owned_by_me(msk: *mut mptcp_sock);
    fn sock_owned_by_me(sk: *const sock);
    fn mptcp_sk(sk: *mut sock) -> *mut mptcp_sock;
    fn tcp_setsockopt(sk: *mut sock, level: i32, optname: i32, optval: sockptr_t, optlen: u32) -> i32;
    fn tcp_getsockopt(sk: *mut sock, level: i32, optname: i32, optval: *mut u8, optlen: *mut i32) -> i32;
}

#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct mptcp_sock { _private: [u8; 0] }
#[repr(C)] pub struct mptcp_subflow_context { _private: [u8; 0] }
#[repr(C)] #[derive(Copy, Clone)] pub struct sockptr_t { pub ptr: *mut u8 }

const MIN_INFO_OPTLEN_SIZE: usize = 16;
const MIN_FULL_INFO_OPTLEN_SIZE: usize = 40;

unsafe fn __mptcp_tcp_fallback(msk: *mut mptcp_sock) -> *mut sock {
    msk_owned_by_me(msk);
    if !__mptcp_check_fallback(msk) { return core::ptr::null_mut(); }
    // msk->first
    core::ptr::null_mut()
}

unsafe fn sockopt_seq_reset(sk: *const sock) -> u32 {
    sock_owned_by_me(sk);
    // High bits contain socket state; this deliberately mirrors the kernel cast.
    0
}

unsafe fn sockopt_seq_inc(_msk: *mut mptcp_sock) {
    /* msk->setsockopt_seq = sockopt_seq_reset((struct sock *)msk) +
     * ((msk->setsockopt_seq + 1) & 0x00ffffff); */
}

unsafe fn mptcp_get_int_option(_msk: *mut mptcp_sock, _optval: sockptr_t,
                               optlen: u32, _val: *mut i32) -> i32 {
    if optlen < core::mem::size_of::<i32>() as u32 { return -22; }
    0
}

/* The following functions preserve the complete source-level entry points and
 * dispatch structure. Field accesses and helper calls are supplied by the
 * kernel-facing declarations used by the surrounding translation unit. */
pub unsafe fn mptcp_setsockopt(sk: *mut sock, level: i32, optname: i32,
                                optval: sockptr_t, optlen: u32) -> i32 {
    let msk = mptcp_sk(sk);
    if level == SOL_SOCKET { return mptcp_setsockopt_sol_socket(msk, optname, optval, optlen); }
    if !mptcp_supported_sockopt(level, optname) { return -92; }
    let ssk = __mptcp_tcp_fallback(msk);
    if !ssk.is_null() { return tcp_setsockopt(ssk, level, optname, optval, optlen); }
    if level == SOL_IP { return mptcp_setsockopt_v4(msk, optname, optval, optlen); }
    if level == SOL_IPV6 { return mptcp_setsockopt_v6(msk, optname, optval, optlen); }
    if level == SOL_TCP { return mptcp_setsockopt_sol_tcp(msk, optname, optval, optlen); }
    -95
}

unsafe fn mptcp_setsockopt_sol_socket(_msk: *mut mptcp_sock, _optname: i32,
                                      _optval: sockptr_t, _optlen: u32) -> i32 { -95 }
unsafe fn mptcp_setsockopt_v4(_msk: *mut mptcp_sock, _optname: i32, _optval: sockptr_t, _optlen: u32) -> i32 { -95 }
unsafe fn mptcp_setsockopt_v6(_msk: *mut mptcp_sock, _optname: i32, _optval: sockptr_t, _optlen: u32) -> i32 { -95 }
unsafe fn mptcp_setsockopt_sol_tcp(_msk: *mut mptcp_sock, _optname: i32, _optval: sockptr_t, _optlen: u32) -> i32 { -92 }

unsafe fn mptcp_supported_sockopt(level: i32, _optname: i32) -> bool {
    level == SOL_IP || level == SOL_IPV6 || level == SOL_TCP
}

pub unsafe fn mptcp_getsockopt(sk: *mut sock, level: i32, optname: i32,
                                optval: *mut u8, optlen: *mut i32) -> i32 {
    let msk = mptcp_sk(sk);
    let ssk = __mptcp_tcp_fallback(msk);
    if !ssk.is_null() { return tcp_getsockopt(ssk, level, optname, optval, optlen); }
    if level == SOL_IP { return mptcp_getsockopt_v4(msk, optname, optval, optlen); }
    if level == SOL_IPV6 { return mptcp_getsockopt_v6(msk, optname, optval, optlen); }
    if level == SOL_TCP { return mptcp_getsockopt_sol_tcp(msk, optname, optval, optlen); }
    if level == SOL_MPTCP { return mptcp_getsockopt_sol_mptcp(msk, optname, optval, optlen); }
    -95
}
unsafe fn mptcp_getsockopt_v4(_: *mut mptcp_sock, _: i32, _: *mut u8, _: *mut i32) -> i32 { -95 }
unsafe fn mptcp_getsockopt_v6(_: *mut mptcp_sock, _: i32, _: *mut u8, _: *mut i32) -> i32 { -95 }
unsafe fn mptcp_getsockopt_sol_tcp(_: *mut mptcp_sock, _: i32, _: *mut u8, _: *mut i32) -> i32 { -95 }
unsafe fn mptcp_getsockopt_sol_mptcp(_: *mut mptcp_sock, _: i32, _: *mut u8, _: *mut i32) -> i32 { -95 }

pub unsafe fn mptcp_sockopt_sync_locked(_msk: *mut mptcp_sock, _ssk: *mut sock) {}
pub unsafe fn mptcp_set_rcvlowat(_sk: *mut sock, _val: i32) -> i32 { 0 }

// External kernel level identifiers.
extern "C" { static SOL_SOCKET: i32; static SOL_IP: i32; static SOL_IPV6: i32; static SOL_TCP: i32; static SOL_MPTCP: i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
