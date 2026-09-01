// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) Meta Platforms, Inc. and affiliates. */

/* Translated from C. Dependencies originally came from:
 * vmlinux.h, bpf_tracing_net.h, bpf_core_read.h, bpf_helpers.h,
 * bpf_tracing.h, and bpf_misc.h.
 */

use core::ffi::{c_char, c_void};
use core::mem::{size_of, size_of_val};

extern "C" {
    #[link_name = "CONFIG_HZ"]
    static CONFIG_HZ: libc::c_ulong;

    fn bpf_getsockopt(
        ctx: *mut c_void,
        level: libc::c_int,
        optname: libc::c_int,
        optval: *mut c_void,
        optlen: libc::c_int,
    ) -> libc::c_int;
    fn bpf_setsockopt(
        ctx: *mut c_void,
        level: libc::c_int,
        optname: libc::c_int,
        optval: *mut c_void,
        optlen: libc::c_int,
    ) -> libc::c_int;
    fn bpf_strncmp(
        s1: *const c_char,
        s1_sz: u32,
        s2: *const c_char,
    ) -> libc::c_int;
    fn bpf_loop(
        nr_loops: u32,
        callback_fn: unsafe extern "C" fn(u32, *mut loop_ctx) -> libc::c_int,
        callback_ctx: *mut loop_ctx,
        flags: u64,
    ) -> libc::c_int;
    fn bpf_skc_to_tcp_sock(sk: *mut bpf_sock) -> *mut c_void;
}

extern "C" {
    fn bpf_core_cast_tcp_sock(sk: *mut bpf_sock) -> *mut tcp_sock;
}

#[repr(C)]
pub struct sock_common {
    pub skc_family: u16,
}

#[repr(C)]
pub struct sock {
    pub __sk_common: sock_common,
    pub sk_type: i32,
    pub sk_protocol: u16,
    pub sk_family: u16,
    pub sk_state: i32,
}

#[repr(C)]
pub struct socket {
    pub sk: *mut sock,
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sockopt {
    pub sk: *mut bpf_sock,
    pub level: i32,
    pub optname: i32,
    pub optval: *mut i32,
    pub optval_end: *mut c_void,
    pub retval: i32,
}

#[repr(C)]
pub struct tcp_sock {
    pub bpf_sock_ops_cb_flags: i32,
}

#[repr(C)]
pub struct bpf_sock_ops {
    pub sk: *mut bpf_sock,
    pub op: i32,
    pub family: i32,
    pub bpf_sock_ops_cb_flags: i32,
    pub args: [i32; 4],
}

/* const volatile in C. */
#[no_mangle]
pub static mut veth: [c_char; IFNAMSIZ] = [0; IFNAMSIZ];
#[no_mangle]
pub static mut veth_ifindex: i32 = 0;

#[no_mangle]
pub static mut nr_listen: i32 = 0;
#[no_mangle]
pub static mut nr_passive: i32 = 0;
#[no_mangle]
pub static mut nr_active: i32 = 0;
#[no_mangle]
pub static mut nr_connect: i32 = 0;
#[no_mangle]
pub static mut nr_binddev: i32 = 0;
#[no_mangle]
pub static mut nr_socket_post_create: i32 = 0;
#[no_mangle]
pub static mut nr_fin_wait1: i32 = 0;

#[repr(C)]
pub struct sockopt_test {
    pub opt: i32,
    pub r#new: i32,
    pub restore: i32,
    pub expected: i32,
    pub tcp_expected: i32,
    pub flip: u32,
}

static not_exist_cc: &[u8; 10] = b"not_exist\0";
static cubic_cc: &[u8; 6] = b"cubic\0";
static reno_cc: &[u8; 5] = b"reno\0";

static sol_socket_tests: [sockopt_test; 11] = [
    sockopt_test { opt: SO_REUSEADDR, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 1 },
    sockopt_test { opt: SO_SNDBUF, r#new: 8123, restore: 0, expected: 8123 * 2, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: SO_RCVBUF, r#new: 8123, restore: 0, expected: 8123 * 2, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: SO_KEEPALIVE, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 1 },
    sockopt_test { opt: SO_PRIORITY, r#new: 0xeb9f, restore: 0, expected: 0xeb9f, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: SO_REUSEPORT, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 1 },
    sockopt_test { opt: SO_RCVLOWAT, r#new: 8123, restore: 0, expected: 8123, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: SO_MARK, r#new: 0xeb9f, restore: 0, expected: 0xeb9f, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: SO_MAX_PACING_RATE, r#new: 0xeb9f, restore: 0, expected: 0xeb9f, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: SO_TXREHASH, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 1 },
    sockopt_test { opt: 0, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 0 },
];

static sol_tcp_tests: [sockopt_test; 15] = [
    sockopt_test { opt: TCP_NODELAY, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 1 },
    sockopt_test { opt: TCP_KEEPIDLE, r#new: 123, restore: 321, expected: 123, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: TCP_KEEPINTVL, r#new: 123, restore: 321, expected: 123, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: TCP_KEEPCNT, r#new: 123, restore: 124, expected: 123, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: TCP_SYNCNT, r#new: 123, restore: 124, expected: 123, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: TCP_WINDOW_CLAMP, r#new: 8123, restore: 8124, expected: 8123, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: TCP_CONGESTION, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: TCP_THIN_LINEAR_TIMEOUTS, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 1 },
    sockopt_test { opt: TCP_USER_TIMEOUT, r#new: 123400, restore: 0, expected: 123400, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: TCP_NOTSENT_LOWAT, r#new: 1314, restore: 0, expected: 1314, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: TCP_BPF_SOCK_OPS_CB_FLAGS, r#new: BPF_SOCK_OPS_ALL_CB_FLAGS, restore: 0, expected: BPF_SOCK_OPS_ALL_CB_FLAGS, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: TCP_BPF_DELACK_MAX, r#new: 30000, restore: 0, expected: 30000, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: TCP_BPF_RTO_MIN, r#new: 30000, restore: 0, expected: 30000, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: TCP_RTO_MAX_MS, r#new: 2000, restore: 0, expected: 2000, tcp_expected: 0, flip: 0 },
    sockopt_test { opt: 0, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 0 },
];

static sol_ip_tests: [sockopt_test; 3] = [
    sockopt_test { opt: IP_TOS, r#new: 0xe1, restore: 0, expected: 0xe1, tcp_expected: 0xe0, flip: 0 },
    sockopt_test { opt: IP_TRANSPARENT, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 1 },
    sockopt_test { opt: 0, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 0 },
];

static sol_ipv6_tests: [sockopt_test; 4] = [
    sockopt_test { opt: IPV6_TCLASS, r#new: 0xe1, restore: 0, expected: 0xe1, tcp_expected: 0xe0, flip: 0 },
    sockopt_test { opt: IPV6_AUTOFLOWLABEL, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 1 },
    sockopt_test { opt: IPV6_TRANSPARENT, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 1 },
    sockopt_test { opt: 0, r#new: 0, restore: 0, expected: 0, tcp_expected: 0, flip: 0 },
];

#[repr(C)]
pub struct loop_ctx {
    pub ctx: *mut c_void,
    pub sk: *mut sock,
}

unsafe fn sk_is_tcp(sk: *mut sock) -> bool {
    ((*sk).__sk_common.skc_family as i32 == AF_INET
        || (*sk).__sk_common.skc_family as i32 == AF_INET6)
        && (*sk).sk_type == SOCK_STREAM
        && (*sk).sk_protocol as i32 == IPPROTO_TCP
}

unsafe fn bpf_test_sockopt_flip(
    ctx: *mut c_void,
    sk: *mut sock,
    t: *const sockopt_test,
    level: i32,
) -> i32 {
    let mut old: i32 = 0;
    let mut tmp: i32 = 0;
    let mut opt: i32 = (*t).opt;

    opt = (*t).opt;

    if opt == SO_TXREHASH && !sk_is_tcp(sk) {
        return 0;
    }

    if bpf_getsockopt(ctx, level, opt, &mut old as *mut _ as *mut c_void, size_of_val(&old) as i32) != 0 {
        return 1;
    }
    /* kernel initialized txrehash to 255 */
    if level == SOL_SOCKET && opt == SO_TXREHASH && old != 0 && old != 1 {
        old = 1;
    }

    let mut new: i32 = if old == 0 { 1 } else { 0 };
    if bpf_setsockopt(ctx, level, opt, &mut new as *mut _ as *mut c_void, size_of_val(&new) as i32) != 0 {
        return 1;
    }
    if bpf_getsockopt(ctx, level, opt, &mut tmp as *mut _ as *mut c_void, size_of_val(&tmp) as i32) != 0
        || tmp != new
    {
        return 1;
    }

    if bpf_setsockopt(ctx, level, opt, &mut old as *mut _ as *mut c_void, size_of_val(&old) as i32) != 0 {
        return 1;
    }

    0
}

unsafe fn bpf_test_sockopt_int(
    ctx: *mut c_void,
    sk: *mut sock,
    t: *const sockopt_test,
    level: i32,
) -> i32 {
    let mut old: i32 = 0;
    let mut tmp: i32 = 0;
    let opt: i32 = (*t).opt;
    let mut new: i32 = (*t).r#new;
    let expected: i32;

    if (*sk).sk_type == SOCK_STREAM && (*t).tcp_expected != 0 {
        expected = (*t).tcp_expected;
    } else {
        expected = (*t).expected;
    }

    if bpf_getsockopt(ctx, level, opt, &mut old as *mut _ as *mut c_void, size_of_val(&old) as i32) != 0
        || old == new
    {
        return 1;
    }

    if bpf_setsockopt(ctx, level, opt, &mut new as *mut _ as *mut c_void, size_of_val(&new) as i32) != 0 {
        return 1;
    }
    if bpf_getsockopt(ctx, level, opt, &mut tmp as *mut _ as *mut c_void, size_of_val(&tmp) as i32) != 0
        || tmp != expected
    {
        return 1;
    }

    if (*t).restore != 0 {
        old = (*t).restore;
    }
    if bpf_setsockopt(ctx, level, opt, &mut old as *mut _ as *mut c_void, size_of_val(&old) as i32) != 0 {
        return 1;
    }

    0
}

unsafe extern "C" fn bpf_test_socket_sockopt(i: u32, lc: *mut loop_ctx) -> i32 {
    let t: *const sockopt_test;

    if i as usize >= sol_socket_tests.len() {
        return 1;
    }

    t = &sol_socket_tests[i as usize] as *const _;
    if (*t).opt == 0 {
        return 1;
    }

    if (*t).flip != 0 {
        return bpf_test_sockopt_flip((*lc).ctx, (*lc).sk, t, SOL_SOCKET);
    }

    bpf_test_sockopt_int((*lc).ctx, (*lc).sk, t, SOL_SOCKET)
}

unsafe extern "C" fn bpf_test_ip_sockopt(i: u32, lc: *mut loop_ctx) -> i32 {
    let t: *const sockopt_test;

    if i as usize >= sol_ip_tests.len() {
        return 1;
    }

    t = &sol_ip_tests[i as usize] as *const _;
    if (*t).opt == 0 {
        return 1;
    }

    if (*t).flip != 0 {
        return bpf_test_sockopt_flip((*lc).ctx, (*lc).sk, t, IPPROTO_IP);
    }

    bpf_test_sockopt_int((*lc).ctx, (*lc).sk, t, IPPROTO_IP)
}

unsafe extern "C" fn bpf_test_ipv6_sockopt(i: u32, lc: *mut loop_ctx) -> i32 {
    let t: *const sockopt_test;

    if i as usize >= sol_ipv6_tests.len() {
        return 1;
    }

    t = &sol_ipv6_tests[i as usize] as *const _;
    if (*t).opt == 0 {
        return 1;
    }

    if (*t).flip != 0 {
        return bpf_test_sockopt_flip((*lc).ctx, (*lc).sk, t, IPPROTO_IPV6);
    }

    bpf_test_sockopt_int((*lc).ctx, (*lc).sk, t, IPPROTO_IPV6)
}

unsafe extern "C" fn bpf_test_tcp_sockopt(i: u32, lc: *mut loop_ctx) -> i32 {
    let t: *const sockopt_test;
    let sk: *mut sock;
    let ctx: *mut c_void;

    if i as usize >= sol_tcp_tests.len() {
        return 1;
    }

    t = &sol_tcp_tests[i as usize] as *const _;
    if (*t).opt == 0 {
        return 1;
    }

    ctx = (*lc).ctx;
    sk = (*lc).sk;

    if (*t).opt == TCP_CONGESTION {
        let mut old_cc: [c_char; 16] = [0; 16];
        let mut tmp_cc: [c_char; 16] = [0; 16];
        let new_cc: *const c_char;
        let new_cc_len: i32;

        if bpf_setsockopt(
            ctx,
            IPPROTO_TCP,
            TCP_CONGESTION,
            not_exist_cc.as_ptr() as *mut c_void,
            size_of_val(not_exist_cc) as i32,
        ) == 0
        {
            return 1;
        }
        if bpf_getsockopt(
            ctx,
            IPPROTO_TCP,
            TCP_CONGESTION,
            old_cc.as_mut_ptr() as *mut c_void,
            size_of_val(&old_cc) as i32,
        ) != 0
        {
            return 1;
        }
        if bpf_strncmp(old_cc.as_ptr(), size_of_val(&old_cc) as u32, cubic_cc.as_ptr() as *const c_char) == 0 {
            new_cc = reno_cc.as_ptr() as *const c_char;
            new_cc_len = size_of_val(reno_cc) as i32;
        } else {
            new_cc = cubic_cc.as_ptr() as *const c_char;
            new_cc_len = size_of_val(cubic_cc) as i32;
        }

        if bpf_setsockopt(ctx, IPPROTO_TCP, TCP_CONGESTION, new_cc as *mut c_void, new_cc_len) != 0 {
            return 1;
        }
        if bpf_getsockopt(
            ctx,
            IPPROTO_TCP,
            TCP_CONGESTION,
            tmp_cc.as_mut_ptr() as *mut c_void,
            size_of_val(&tmp_cc) as i32,
        ) != 0
        {
            return 1;
        }
        if bpf_strncmp(tmp_cc.as_ptr(), size_of_val(&tmp_cc) as u32, new_cc) != 0 {
            return 1;
        }
        if bpf_setsockopt(
            ctx,
            IPPROTO_TCP,
            TCP_CONGESTION,
            old_cc.as_mut_ptr() as *mut c_void,
            size_of_val(&old_cc) as i32,
        ) != 0
        {
            return 1;
        }
        return 0;
    }

    if (*t).flip != 0 {
        return bpf_test_sockopt_flip(ctx, sk, t, IPPROTO_TCP);
    }

    bpf_test_sockopt_int(ctx, sk, t, IPPROTO_TCP)
}

unsafe fn bpf_test_sockopt(ctx: *mut c_void, sk: *mut sock) -> i32 {
    let mut lc = loop_ctx { ctx, sk };
    let family: u16;
    let proto: u16;
    let mut n: i32;

    family = (*sk).sk_family;
    proto = (*sk).sk_protocol;

    n = bpf_loop(sol_socket_tests.len() as u32, bpf_test_socket_sockopt, &mut lc, 0);
    if n != sol_socket_tests.len() as i32 {
        return -1;
    }

    if proto as i32 == IPPROTO_TCP {
        n = bpf_loop(sol_tcp_tests.len() as u32, bpf_test_tcp_sockopt, &mut lc, 0);
        if n != sol_tcp_tests.len() as i32 {
            return -1;
        }
    }

    if family as i32 == AF_INET {
        n = bpf_loop(sol_ip_tests.len() as u32, bpf_test_ip_sockopt, &mut lc, 0);
        if n != sol_ip_tests.len() as i32 {
            return -1;
        }
    } else {
        n = bpf_loop(sol_ipv6_tests.len() as u32, bpf_test_ipv6_sockopt, &mut lc, 0);
        if n != sol_ipv6_tests.len() as i32 {
            return -1;
        }
    }

    0
}

unsafe fn binddev_test(ctx: *mut c_void) -> i32 {
    let empty_ifname: [c_char; 1] = [0];
    let mut ifindex: i32 = 0;
    let mut zero: i32 = 0;

    if bpf_setsockopt(ctx, SOL_SOCKET, SO_BINDTODEVICE, veth.as_ptr() as *mut c_void, size_of_val(&veth) as i32) != 0 {
        return -1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTOIFINDEX,
        &mut ifindex as *mut _ as *mut c_void,
        size_of::<i32>() as i32,
    ) != 0
        || ifindex != veth_ifindex
    {
        return -1;
    }

    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTODEVICE,
        empty_ifname.as_ptr() as *mut c_void,
        size_of_val(&empty_ifname) as i32,
    ) != 0
    {
        return -1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTOIFINDEX,
        &mut ifindex as *mut _ as *mut c_void,
        size_of::<i32>() as i32,
    ) != 0
        || ifindex != 0
    {
        return -1;
    }

    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTOIFINDEX,
        &veth_ifindex as *const _ as *mut c_void,
        size_of::<i32>() as i32,
    ) != 0
    {
        return -1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTOIFINDEX,
        &mut ifindex as *mut _ as *mut c_void,
        size_of::<i32>() as i32,
    ) != 0
        || ifindex != veth_ifindex
    {
        return -1;
    }

    if bpf_setsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTOIFINDEX,
        &mut zero as *mut _ as *mut c_void,
        size_of::<i32>() as i32,
    ) != 0
    {
        return -1;
    }
    if bpf_getsockopt(
        ctx,
        SOL_SOCKET,
        SO_BINDTOIFINDEX,
        &mut ifindex as *mut _ as *mut c_void,
        size_of::<i32>() as i32,
    ) != 0
        || ifindex != 0
    {
        return -1;
    }

    0
}

unsafe fn test_tcp_maxseg(ctx: *mut c_void, sk: *mut sock) -> i32 {
    let mut val: i32 = 1314;
    let mut tmp: i32 = 0;

    if (*sk).sk_state != TCP_ESTABLISHED {
        return bpf_setsockopt(
            ctx,
            IPPROTO_TCP,
            TCP_MAXSEG,
            &mut val as *mut _ as *mut c_void,
            size_of_val(&val) as i32,
        );
    }

    if bpf_getsockopt(ctx, IPPROTO_TCP, TCP_MAXSEG, &mut tmp as *mut _ as *mut c_void, size_of_val(&tmp) as i32) != 0
        || tmp > val
    {
        return -1;
    }

    0
}

unsafe fn test_tcp_saved_syn(ctx: *mut c_void, sk: *mut sock) -> i32 {
    let mut saved_syn: [u8; 20] = [0; 20];
    let mut one: i32 = 1;

    if (*sk).sk_state == TCP_LISTEN {
        return bpf_setsockopt(
            ctx,
            IPPROTO_TCP,
            TCP_SAVE_SYN,
            &mut one as *mut _ as *mut c_void,
            size_of_val(&one) as i32,
        );
    }

    bpf_getsockopt(
        ctx,
        IPPROTO_TCP,
        TCP_SAVED_SYN,
        saved_syn.as_mut_ptr() as *mut c_void,
        size_of_val(&saved_syn) as i32,
    )
}

/* SEC("lsm_cgroup/socket_post_create") */
#[no_mangle]
pub unsafe extern "C" fn socket_post_create(
    sock: *mut socket,
    _family: i32,
    _type: i32,
    _protocol: i32,
    _kern: i32,
) -> i32 {
    let sk: *mut sock = (*sock).sk;

    if sk.is_null() {
        return 1;
    }

    nr_socket_post_create += (bpf_test_sockopt(sk as *mut c_void, sk) == 0) as i32;
    nr_binddev += (binddev_test(sk as *mut c_void) == 0) as i32;

    1
}

/* SEC("cgroup/getsockopt") */
#[no_mangle]
pub unsafe extern "C" fn _getsockopt(ctx: *mut bpf_sockopt) -> i32 {
    let sk: *mut bpf_sock = (*ctx).sk;
    let optval: *mut i32 = (*ctx).optval;
    let tp: *mut tcp_sock;

    if sk.is_null() || (*ctx).level != SOL_TCP || (*ctx).optname != TCP_BPF_SOCK_OPS_CB_FLAGS {
        return 1;
    }

    tp = bpf_core_cast_tcp_sock(sk);
    if ((*ctx).optval as *mut u8).add(size_of::<i32>()) <= (*ctx).optval_end as *mut u8 {
        *optval = (*tp).bpf_sock_ops_cb_flags;
        (*ctx).retval = 0;
    }
    1
}

#[no_mangle]
pub static mut v4mapped_v6_ip_tos_enable: i32 = 0;
#[no_mangle]
pub static mut v4mapped_v6_ip_tos_ret: i32 = 0;
#[no_mangle]
pub static mut v4mapped_v6_ip_tos_cnt: i32 = 0;
#[no_mangle]
pub static mut v4mapped_v6_ip_tos_val: i32 = 0;

unsafe fn test_v4mapped_v6_ip_tos(skops: *mut bpf_sock_ops) {
    let mut tos: i32 = v4mapped_v6_ip_tos_val;

    if v4mapped_v6_ip_tos_enable == 0 || (*skops).op != BPF_SOCK_OPS_TCP_CONNECT_CB {
        return;
    }
    if (*skops).family != AF_INET6 {
        return;
    }

    v4mapped_v6_ip_tos_cnt += 1;
    v4mapped_v6_ip_tos_ret = bpf_setsockopt(
        skops as *mut c_void,
        IPPROTO_IP,
        IP_TOS,
        &mut tos as *mut _ as *mut c_void,
        size_of_val(&tos) as i32,
    );
}

/* SEC("sockops") */
#[no_mangle]
pub unsafe extern "C" fn skops_sockopt(skops: *mut bpf_sock_ops) -> i32 {
    let bpf_sk: *mut bpf_sock = (*skops).sk;
    let sk: *mut sock;
    let mut flags: i32;

    if bpf_sk.is_null() {
        return 1;
    }

    sk = bpf_skc_to_tcp_sock(bpf_sk) as *mut sock;
    if sk.is_null() {
        return 1;
    }

    if v4mapped_v6_ip_tos_enable != 0 {
        test_v4mapped_v6_ip_tos(skops);
        return 1;
    }

    match (*skops).op {
        BPF_SOCK_OPS_TCP_LISTEN_CB => {
            nr_listen += ((bpf_test_sockopt(skops as *mut c_void, sk) != 0
                || test_tcp_maxseg(skops as *mut c_void, sk) != 0
                || test_tcp_saved_syn(skops as *mut c_void, sk) != 0) == false) as i32;
        }
        BPF_SOCK_OPS_TCP_CONNECT_CB => {
            nr_connect += ((bpf_test_sockopt(skops as *mut c_void, sk) != 0
                || test_tcp_maxseg(skops as *mut c_void, sk) != 0) == false) as i32;
        }
        BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB => {
            nr_active += ((bpf_test_sockopt(skops as *mut c_void, sk) != 0
                || test_tcp_maxseg(skops as *mut c_void, sk) != 0) == false) as i32;
        }
        BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB => {
            nr_passive += ((bpf_test_sockopt(skops as *mut c_void, sk) != 0
                || test_tcp_maxseg(skops as *mut c_void, sk) != 0
                || test_tcp_saved_syn(skops as *mut c_void, sk) != 0) == false) as i32;
            flags = (*skops).bpf_sock_ops_cb_flags | BPF_SOCK_OPS_STATE_CB_FLAG;
            bpf_setsockopt(
                skops as *mut c_void,
                SOL_TCP,
                TCP_BPF_SOCK_OPS_CB_FLAGS,
                &mut flags as *mut _ as *mut c_void,
                size_of_val(&flags) as i32,
            );
        }
        BPF_SOCK_OPS_STATE_CB => {
            if (*skops).args[1] == BPF_TCP_CLOSE_WAIT {
                nr_fin_wait1 += (bpf_test_sockopt(skops as *mut c_void, sk) == 0) as i32;
            }
        }
        _ => {}
    }

    1
}

/* SEC("license") */
#[no_mangle]
pub static _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
