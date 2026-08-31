// SPDX-License-Identifier: GPL-2.0
/*
 * Test for sockmap/sockhash redirection.
 *
 * BPF_MAP_TYPE_SOCKMAP
 * BPF_MAP_TYPE_SOCKHASH
 *	x
 * sk_msg-to-egress
 * sk_msg-to-ingress
 * sk_skb-to-egress
 * sk_skb-to-ingress
 *	x
 * AF_INET, SOCK_STREAM
 * AF_INET6, SOCK_STREAM
 * AF_INET, SOCK_DGRAM
 * AF_INET6, SOCK_DGRAM
 * AF_UNIX, SOCK_STREAM
 * AF_UNIX, SOCK_DGRAM
 * AF_VSOCK, SOCK_STREAM
 * AF_VSOCK, SOCK_SEQPACKET
 */

/* C includes translated as external dependencies:
 * errno.h, error.h, sched.h, stdio.h, unistd.h,
 * netinet/in.h, sys/socket.h, sys/types.h, sys/un.h,
 * linux/string.h, linux/vm_sockets.h,
 * bpf/bpf.h, bpf/libbpf.h,
 * linux/const.h, test_progs.h, sockmap_helpers.h,
 * test_sockmap_redir.skel.h
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

const SUPPORTED: c_int = 1 << 0;

/* Note on sk_skb-to-ingress ->af_vsock:
 *
 * Peer socket may receive the packet some time after the return from sendmsg().
 * In a typical usage scenario, recvmsg() will block until the redirected packet
 * appears in the destination queue, or timeout if the packet was dropped. By
 * that point, the verdict map has already been updated to reflect what has
 * happened.
 *
 * But sk_skb-to-ingress/af_vsock is an unsupported combination, so no recvmsg()
 * takes place. Which means we may race the execution of the verdict logic and
 * read map_verd before it has been updated, i.e. we might observe
 * map_verd[SK_DROP]=0 instead of map_verd[SK_DROP]=1.
 *
 * This confuses the selftest logic: if there was no packet dropped, where's the
 * packet? So here's a heuristic: on map_verd[SK_DROP]=map_verd[SK_PASS]=0
 * (which implies the verdict program has not been ran) just re-read the verdict
 * map again.
 */
const UNSUPPORTED_RACY_VERD: c_int = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum prog_type {
    SK_MSG_EGRESS,
    SK_MSG_INGRESS,
    SK_SKB_EGRESS,
    SK_SKB_INGRESS,
}

const SEND_INNER: c_int = 0;
const SEND_OUTER: c_int = 1;

const RECV_INNER: c_int = 0;
const RECV_OUTER: c_int = 1;

#[repr(C)]
struct maps {
    r#in: c_int,
    out: c_int,
    verd: c_int,
}

#[repr(C)]
struct combo_spec {
    prog_type: prog_type,
    r#in: *const c_char,
    out: *const c_char,
}

#[repr(C)]
struct redir_spec {
    name: *const c_char,
    idx_send: c_int,
    idx_recv: c_int,
    prog_type: prog_type,
}

#[repr(C)]
struct socket_spec {
    family: c_int,
    sotype: c_int,
    send_flags: c_int,
    r#in: [c_int; 2],
    out: [c_int; 2],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct test_sockmap_redir {
    progs: test_sockmap_redir_progs,
    maps: test_sockmap_redir_maps,
    bss: *mut test_sockmap_redir_bss,
}

#[repr(C)]
struct test_sockmap_redir_progs {
    prog_msg_verdict: *mut bpf_program,
    prog_skb_verdict: *mut bpf_program,
}

#[repr(C)]
struct test_sockmap_redir_maps {
    nop_map: *mut bpf_map,
    sock_map: *mut bpf_map,
    nop_hash: *mut bpf_map,
    sock_hash: *mut bpf_map,
    verdict_map: *mut bpf_map,
}

#[repr(C)]
struct test_sockmap_redir_bss {
    redirect_type: bpf_map_type,
    redirect_flags: c_int,
}

type ssize_t = isize;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum bpf_attach_type {
    BPF_SK_MSG_VERDICT,
    BPF_SK_SKB_VERDICT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum bpf_map_type {
    BPF_MAP_TYPE_SOCKMAP,
    BPF_MAP_TYPE_SOCKHASH,
}

extern "C" {
    static mut errno: c_int;

    static AF_INET: c_int;
    static AF_INET6: c_int;
    static AF_UNIX: c_int;
    static AF_VSOCK: c_int;
    static SOCK_STREAM: c_int;
    static SOCK_DGRAM: c_int;
    static SOCK_SEQPACKET: c_int;
    static MSG_DONTWAIT: c_int;
    static MSG_OOB: c_int;
    static BPF_ANY: c_int;
    static BPF_NOEXIST: c_int;
    static BPF_F_INGRESS: c_int;
    static SK_DROP: c_int;
    static SK_PASS: c_int;
    static EACCES: c_int;
    static IO_TIMEOUT_SEC: c_int;
    static MAX_TEST_NAME: usize;

    fn create_socket_pairs(
        family: c_int,
        sotype: c_int,
        in0: *mut c_int,
        out0: *mut c_int,
        in1: *mut c_int,
        out1: *mut c_int,
    ) -> c_int;
    fn xclose(fd: c_int);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn recv(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> ssize_t;
    fn recv_timeout(fd: c_int, buf: *mut c_void, len: usize, flags: c_int, timeout_sec: c_int) -> ssize_t;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> ssize_t;
    fn sched_yield() -> c_int;
    fn xbpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn xbpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: c_int) -> c_int;
    fn xbpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn xbpf_prog_attach(prog_fd: c_int, target_fd: c_int, attach_type: bpf_attach_type, flags: c_uint) -> c_int;
    fn xbpf_prog_detach2(prog_fd: c_int, target_fd: c_int, attach_type: bpf_attach_type) -> c_int;
    fn socket_kind_to_str(fd: c_int) -> *const c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn test_sockmap_redir__open_and_load() -> *mut test_sockmap_redir;
    fn test_sockmap_redir__destroy(skel: *mut test_sockmap_redir);

    fn FAIL(format: *const c_char, ...);
    fn FAIL_ERRNO(format: *const c_char, ...);
}

unsafe fn u32(v: c_int) -> u32 {
    v as u32
}

unsafe fn u64(v: c_int) -> u64 {
    v as u64
}

unsafe fn socket_spec_pairs(s: *mut socket_spec) -> c_int {
    create_socket_pairs(
        (*s).family,
        (*s).sotype,
        &mut (*s).r#in[0],
        &mut (*s).out[0],
        &mut (*s).r#in[1],
        &mut (*s).out[1],
    )
}

unsafe fn socket_spec_close(s: *mut socket_spec) {
    xclose((*s).r#in[0]);
    xclose((*s).r#in[1]);
    xclose((*s).out[0]);
    xclose((*s).out[1]);
}

unsafe fn get_redir_params(
    redir: *mut redir_spec,
    skel: *mut test_sockmap_redir,
    prog_fd: *mut c_int,
    attach_type: *mut bpf_attach_type,
    redirect_flags: *mut c_int,
) {
    let type_ = (*redir).prog_type;
    let prog: *mut bpf_program;
    let sk_msg: bool;

    sk_msg = type_ == prog_type::SK_MSG_INGRESS || type_ == prog_type::SK_MSG_EGRESS;
    prog = if sk_msg {
        (*skel).progs.prog_msg_verdict
    } else {
        (*skel).progs.prog_skb_verdict
    };

    *prog_fd = bpf_program__fd(prog);
    *attach_type = if sk_msg {
        bpf_attach_type::BPF_SK_MSG_VERDICT
    } else {
        bpf_attach_type::BPF_SK_SKB_VERDICT
    };

    if type_ == prog_type::SK_MSG_INGRESS || type_ == prog_type::SK_SKB_INGRESS {
        *redirect_flags = BPF_F_INGRESS;
    } else {
        *redirect_flags = 0;
    }
}

unsafe fn try_recv(prefix: *const c_char, fd: c_int, flags: c_int, expect_success: bool) {
    let n: ssize_t;
    let mut buf: c_char = 0;

    errno = 0;
    n = recv(fd, &mut buf as *mut _ as *mut c_void, 1, flags);
    if n < 0 && expect_success {
        FAIL_ERRNO(c"%s: unexpected failure: retval=%zd".as_ptr(), prefix, n);
    }
    if n == 0 && !expect_success {
        FAIL(c"%s: expected failure: retval=%zd".as_ptr(), prefix, n);
    }
}

unsafe fn handle_unsupported(
    sd_send: c_int,
    sd_peer: c_int,
    sd_in: c_int,
    sd_out: c_int,
    sd_recv: c_int,
    map_verd: c_int,
    status: c_int,
) {
    let mut drop: c_uint = 0;
    let mut pass: c_uint = 0;
    let mut recv_buf: c_char = 0;
    let mut n: ssize_t;

    loop {
        if xbpf_map_lookup_elem(map_verd, &u32(SK_DROP) as *const _ as *const c_void, &mut drop as *mut _ as *mut c_void) != 0
            || xbpf_map_lookup_elem(map_verd, &u32(SK_PASS) as *const _ as *const c_void, &mut pass as *mut _ as *mut c_void) != 0
        {
            return;
        }

        if pass == 0 && drop == 0 && (status & UNSUPPORTED_RACY_VERD) != 0 {
            sched_yield();
            continue;
        }
        break;
    }

    if pass != 0 {
        FAIL(c"unsupported: wanted verdict pass 0, have %u".as_ptr(), pass);
        return;
    }

    /* If nothing was dropped, packet should have reached the peer */
    if drop == 0 {
        errno = 0;
        n = recv_timeout(sd_peer, &mut recv_buf as *mut _ as *mut c_void, 1, 0, IO_TIMEOUT_SEC);
        if n != 1 {
            FAIL_ERRNO(c"unsupported: packet missing, retval=%zd".as_ptr(), n);
        }
    }

    /* Ensure queues are empty */
    try_recv(c"bpf.recv(sd_send)".as_ptr(), sd_send, MSG_DONTWAIT, false);
    if sd_in != sd_send {
        try_recv(c"bpf.recv(sd_in)".as_ptr(), sd_in, MSG_DONTWAIT, false);
    }

    try_recv(c"bpf.recv(sd_out)".as_ptr(), sd_out, MSG_DONTWAIT, false);
    if sd_recv != sd_out {
        try_recv(c"bpf.recv(sd_recv)".as_ptr(), sd_recv, MSG_DONTWAIT, false);
    }
}

unsafe fn test_send_redir_recv(
    sd_send: c_int,
    send_flags: c_int,
    sd_peer: c_int,
    sd_in: c_int,
    sd_out: c_int,
    sd_recv: c_int,
    maps: *mut maps,
    status: c_int,
) {
    let mut drop: c_uint = 0;
    let mut pass: c_uint = 0;
    let send_buf = c"ab".as_ptr() as *mut c_char;
    let mut recv_buf: c_char = 0;
    let mut n: ssize_t;
    let mut len: ssize_t = 1;

    /* Zero out the verdict map */
    if xbpf_map_update_elem((*maps).verd, &u32(SK_DROP) as *const _ as *const c_void, &u32(0) as *const _ as *const c_void, BPF_ANY) != 0
        || xbpf_map_update_elem((*maps).verd, &u32(SK_PASS) as *const _ as *const c_void, &u32(0) as *const _ as *const c_void, BPF_ANY) != 0
    {
        return;
    }

    if xbpf_map_update_elem((*maps).r#in, &u32(0) as *const _ as *const c_void, &u64(sd_in) as *const _ as *const c_void, BPF_NOEXIST) != 0 {
        return;
    }

    if xbpf_map_update_elem((*maps).out, &u32(0) as *const _ as *const c_void, &u64(sd_out) as *const _ as *const c_void, BPF_NOEXIST) != 0 {
        xbpf_map_delete_elem((*maps).r#in, &u32(0) as *const _ as *const c_void);
        return;
    }

    /* Last byte is OOB data when send_flags has MSG_OOB bit set */
    if (send_flags & MSG_OOB) != 0 {
        len += 1;
    }
    n = send(sd_send, send_buf as *const c_void, len as usize, send_flags);
    if n >= 0 && n < len {
        FAIL(c"incomplete send".as_ptr());
    }
    if n < 0 {
        /* sk_msg redirect combo not supported? */
        if (status & SUPPORTED) != 0 || errno != EACCES {
            FAIL_ERRNO(c"send".as_ptr());
        }
        xbpf_map_delete_elem((*maps).out, &u32(0) as *const _ as *const c_void);
        xbpf_map_delete_elem((*maps).r#in, &u32(0) as *const _ as *const c_void);
        return;
    }

    if (status & SUPPORTED) == 0 {
        handle_unsupported(sd_send, sd_peer, sd_in, sd_out, sd_recv, (*maps).verd, status);
        xbpf_map_delete_elem((*maps).out, &u32(0) as *const _ as *const c_void);
        xbpf_map_delete_elem((*maps).r#in, &u32(0) as *const _ as *const c_void);
        return;
    }

    errno = 0;
    n = recv_timeout(sd_recv, &mut recv_buf as *mut _ as *mut c_void, 1, 0, IO_TIMEOUT_SEC);
    if n != 1 {
        FAIL_ERRNO(c"recv_timeout()".as_ptr());
        xbpf_map_delete_elem((*maps).out, &u32(0) as *const _ as *const c_void);
        xbpf_map_delete_elem((*maps).r#in, &u32(0) as *const _ as *const c_void);
        return;
    }

    /* Check verdict _after_ recv(); af_vsock may need time to catch up */
    if xbpf_map_lookup_elem((*maps).verd, &u32(SK_DROP) as *const _ as *const c_void, &mut drop as *mut _ as *mut c_void) != 0
        || xbpf_map_lookup_elem((*maps).verd, &u32(SK_PASS) as *const _ as *const c_void, &mut pass as *mut _ as *mut c_void) != 0
    {
        xbpf_map_delete_elem((*maps).out, &u32(0) as *const _ as *const c_void);
        xbpf_map_delete_elem((*maps).r#in, &u32(0) as *const _ as *const c_void);
        return;
    }

    if drop != 0 || pass != 1 {
        FAIL(c"unexpected verdict drop/pass: wanted 0/1, have %u/%u".as_ptr(), drop, pass);
    }

    if recv_buf != *send_buf {
        FAIL(c"recv(): payload check, %02x != %02x".as_ptr(), recv_buf as c_int, *send_buf as c_int);
    }

    if (send_flags & MSG_OOB) != 0 {
        /* Fail reading OOB while in sockmap */
        try_recv(c"bpf.recv(sd_out, MSG_OOB)".as_ptr(), sd_out, MSG_OOB | MSG_DONTWAIT, false);

        /* Remove sd_out from sockmap */
        xbpf_map_delete_elem((*maps).out, &u32(0) as *const _ as *const c_void);

        /* Check that OOB was dropped on redirect */
        try_recv(c"recv(sd_out, MSG_OOB)".as_ptr(), sd_out, MSG_OOB | MSG_DONTWAIT, false);

        xbpf_map_delete_elem((*maps).r#in, &u32(0) as *const _ as *const c_void);
        return;
    }

    xbpf_map_delete_elem((*maps).out, &u32(0) as *const _ as *const c_void);
    xbpf_map_delete_elem((*maps).r#in, &u32(0) as *const _ as *const c_void);
}

unsafe fn is_redir_supported(type_: prog_type, in_: *const c_char, out: *const c_char) -> c_int {
    /* Matching based on strings returned by socket_kind_to_str():
     * tcp4, udp4, tcp6, udp6, u_str, u_dgr, v_str, v_seq
     * Plus a wildcard: any
     * Not in use: u_seq, v_dgr
     */
    let combos = [
        /* Send to local: TCP -> any, but vsock */
        combo_spec { prog_type: prog_type::SK_MSG_INGRESS, r#in: c"tcp".as_ptr(), out: c"tcp".as_ptr() },
        combo_spec { prog_type: prog_type::SK_MSG_INGRESS, r#in: c"tcp".as_ptr(), out: c"udp".as_ptr() },
        combo_spec { prog_type: prog_type::SK_MSG_INGRESS, r#in: c"tcp".as_ptr(), out: c"u_str".as_ptr() },
        combo_spec { prog_type: prog_type::SK_MSG_INGRESS, r#in: c"tcp".as_ptr(), out: c"u_dgr".as_ptr() },

        /* Send to egress: TCP -> TCP */
        combo_spec { prog_type: prog_type::SK_MSG_EGRESS, r#in: c"tcp".as_ptr(), out: c"tcp".as_ptr() },

        /* Ingress to egress: any -> any */
        combo_spec { prog_type: prog_type::SK_SKB_EGRESS, r#in: c"any".as_ptr(), out: c"any".as_ptr() },

        /* Ingress to local: any -> any, but vsock */
        combo_spec { prog_type: prog_type::SK_SKB_INGRESS, r#in: c"any".as_ptr(), out: c"tcp".as_ptr() },
        combo_spec { prog_type: prog_type::SK_SKB_INGRESS, r#in: c"any".as_ptr(), out: c"udp".as_ptr() },
        combo_spec { prog_type: prog_type::SK_SKB_INGRESS, r#in: c"any".as_ptr(), out: c"u_str".as_ptr() },
        combo_spec { prog_type: prog_type::SK_SKB_INGRESS, r#in: c"any".as_ptr(), out: c"u_dgr".as_ptr() },
    ];

    for c in combos.iter() {
        if c.prog_type == type_
            && (strcmp(c.r#in, c"any".as_ptr()) == 0 || strstarts(in_, c.r#in))
            && (strcmp(c.out, c"any".as_ptr()) == 0 || strstarts(out, c.out))
        {
            return SUPPORTED;
        }
    }

    0
}

unsafe fn get_support_status(type_: prog_type, in_: *const c_char, out: *const c_char) -> c_int {
    let mut status = is_redir_supported(type_, in_, out);

    if type_ == prog_type::SK_SKB_INGRESS && strstarts(out, c"v_".as_ptr()) {
        status |= UNSUPPORTED_RACY_VERD;
    }

    status
}

unsafe fn test_socket(
    type_: bpf_map_type,
    redir: *mut redir_spec,
    maps: *mut maps,
    s_in: *mut socket_spec,
    s_out: *mut socket_spec,
) {
    let fd_in: c_int;
    let fd_out: c_int;
    let fd_send: c_int;
    let fd_peer: c_int;
    let fd_recv: c_int;
    let flags: c_int;
    let status: c_int;
    let in_str: *const c_char;
    let out_str: *const c_char;
    let mut s = [0 as c_char; 128];

    fd_in = (*s_in).r#in[0];
    fd_out = (*s_out).out[0];
    fd_send = (*s_in).r#in[(*redir).idx_send as usize];
    fd_peer = (*s_in).r#in[((*redir).idx_send ^ 1) as usize];
    fd_recv = (*s_out).out[(*redir).idx_recv as usize];
    flags = (*s_in).send_flags;

    in_str = socket_kind_to_str(fd_in);
    out_str = socket_kind_to_str(fd_out);
    status = get_support_status((*redir).prog_type, in_str, out_str);

    snprintf(
        s.as_mut_ptr(),
        s.len(),
        c"%-4s %-17s %-5s %s %-5s%6s".as_ptr(),
        /* hash sk_skb-to-ingress u_str -> v_str (OOB) */
        if type_ == bpf_map_type::BPF_MAP_TYPE_SOCKMAP { c"map".as_ptr() } else { c"hash".as_ptr() },
        (*redir).name,
        in_str,
        if (status & SUPPORTED) != 0 { c"->".as_ptr() } else { c" ".as_ptr() },
        out_str,
        if (flags & MSG_OOB) != 0 { c"(OOB)".as_ptr() } else { c"".as_ptr() },
    );

    if !test__start_subtest(s.as_ptr()) {
        return;
    }

    test_send_redir_recv(fd_send, flags, fd_peer, fd_in, fd_out, fd_recv, maps, status);
}

unsafe fn test_redir(type_: bpf_map_type, redir: *mut redir_spec, maps: *mut maps) {
    let mut sockets = [
        socket_spec { family: AF_INET, sotype: SOCK_STREAM, send_flags: 0, r#in: [0; 2], out: [0; 2] },
        // socket_spec { family: AF_INET, sotype: SOCK_STREAM, send_flags: MSG_OOB, r#in: [0; 2], out: [0; 2] }, /* Known to be broken */
        socket_spec { family: AF_INET6, sotype: SOCK_STREAM, send_flags: 0, r#in: [0; 2], out: [0; 2] },
        socket_spec { family: AF_INET, sotype: SOCK_DGRAM, send_flags: 0, r#in: [0; 2], out: [0; 2] },
        socket_spec { family: AF_INET6, sotype: SOCK_DGRAM, send_flags: 0, r#in: [0; 2], out: [0; 2] },
        socket_spec { family: AF_UNIX, sotype: SOCK_STREAM, send_flags: 0, r#in: [0; 2], out: [0; 2] },
        socket_spec { family: AF_UNIX, sotype: SOCK_STREAM, send_flags: MSG_OOB, r#in: [0; 2], out: [0; 2] },
        socket_spec { family: AF_UNIX, sotype: SOCK_DGRAM, send_flags: 0, r#in: [0; 2], out: [0; 2] },
        // socket_spec { family: AF_UNIX, sotype: SOCK_SEQPACKET, send_flags: 0, r#in: [0; 2], out: [0; 2] }, /* Unsupported BPF_MAP_UPDATE_ELEM */
        socket_spec { family: AF_VSOCK, sotype: SOCK_STREAM, send_flags: 0, r#in: [0; 2], out: [0; 2] },
        // socket_spec { family: AF_VSOCK, sotype: SOCK_DGRAM, send_flags: 0, r#in: [0; 2], out: [0; 2] }, /* Unsupported socket() */
        socket_spec { family: AF_VSOCK, sotype: SOCK_SEQPACKET, send_flags: 0, r#in: [0; 2], out: [0; 2] },
    ];
    let mut s: isize = 0;

    while s < sockets.len() as isize {
        if socket_spec_pairs(sockets.as_mut_ptr().offset(s)) != 0 {
            break;
        }
        s += 1;
    }
    if s < sockets.len() as isize {
        while {
            s -= 1;
            s >= 0
        } {
            socket_spec_close(sockets.as_mut_ptr().offset(s));
        }
        return;
    }

    /* Intra-proto */
    s = 0;
    while s < sockets.len() as isize {
        test_socket(type_, redir, maps, sockets.as_mut_ptr().offset(s), sockets.as_mut_ptr().offset(s));
        s += 1;
    }

    /* Cross-proto */
    for i in 0..sockets.len() {
        for j in 0..sockets.len() {
            let out = sockets.as_mut_ptr().add(j);
            let in_ = sockets.as_mut_ptr().add(i);

            /* Skip intra-proto and between variants */
            if (*out).send_flags != 0 || ((*in_).family == (*out).family && (*in_).sotype == (*out).sotype) {
                continue;
            }

            test_socket(type_, redir, maps, in_, out);
        }
    }

    s = sockets.len() as isize;
    while {
        s -= 1;
        s >= 0
    } {
        socket_spec_close(sockets.as_mut_ptr().offset(s));
    }
}

unsafe fn test_map(type_: bpf_map_type) {
    let mut redirs = [
        redir_spec { name: c"sk_msg-to-ingress".as_ptr(), idx_send: SEND_INNER, idx_recv: RECV_INNER, prog_type: prog_type::SK_MSG_INGRESS },
        redir_spec { name: c"sk_msg-to-egress".as_ptr(), idx_send: SEND_INNER, idx_recv: RECV_OUTER, prog_type: prog_type::SK_MSG_EGRESS },
        redir_spec { name: c"sk_skb-to-egress".as_ptr(), idx_send: SEND_OUTER, idx_recv: RECV_OUTER, prog_type: prog_type::SK_SKB_EGRESS },
        redir_spec { name: c"sk_skb-to-ingress".as_ptr(), idx_send: SEND_OUTER, idx_recv: RECV_INNER, prog_type: prog_type::SK_SKB_INGRESS },
    ];

    for r in redirs.iter_mut() {
        let mut attach_type: bpf_attach_type = bpf_attach_type::BPF_SK_MSG_VERDICT;
        let skel: *mut test_sockmap_redir;
        let mut maps = maps { r#in: 0, out: 0, verd: 0 };
        let mut prog_fd: c_int = 0;

        skel = test_sockmap_redir__open_and_load();
        if skel.is_null() {
            FAIL(c"open_and_load".as_ptr());
            return;
        }

        match type_ {
            bpf_map_type::BPF_MAP_TYPE_SOCKMAP => {
                maps.r#in = bpf_map__fd((*skel).maps.nop_map);
                maps.out = bpf_map__fd((*skel).maps.sock_map);
            }
            bpf_map_type::BPF_MAP_TYPE_SOCKHASH => {
                maps.r#in = bpf_map__fd((*skel).maps.nop_hash);
                maps.out = bpf_map__fd((*skel).maps.sock_hash);
            }
        }

        (*(*skel).bss).redirect_type = type_;
        maps.verd = bpf_map__fd((*skel).maps.verdict_map);
        get_redir_params(r, skel, &mut prog_fd, &mut attach_type, &mut (*(*skel).bss).redirect_flags);

        if xbpf_prog_attach(prog_fd, maps.r#in, attach_type, 0) != 0 {
            return;
        }

        test_redir(type_, r, &mut maps);

        if xbpf_prog_detach2(prog_fd, maps.r#in, attach_type) != 0 {
            return;
        }

        test_sockmap_redir__destroy(skel);
    }
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_sockmap_redir() {
    test_map(bpf_map_type::BPF_MAP_TYPE_SOCKMAP);
    test_map(bpf_map_type::BPF_MAP_TYPE_SOCKHASH);
}
