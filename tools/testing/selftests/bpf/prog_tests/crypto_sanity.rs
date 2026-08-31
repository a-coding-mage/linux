// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// Translated from C implementation source. External test, libbpf, networking,
// libc, and skeleton symbols are expected to be supplied by the surrounding
// selftest harness.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const NS_TEST: &[u8] = b"crypto_sanity_ns\0";
const IPV6_IFACE_ADDR: &[u8] = b"face::1\0";

static crypto_key: &[u8; 17] = b"testtest12345678\0";
static plain_text: &[u8; 17] = b"stringtoencrypt0\0";
static mut opfd: c_int = -1;
static mut tfmfd: c_int = -1;
static algo: &[u8; 9] = b"ecb(aes)\0";

unsafe extern "C" {
    static mut errno: c_int;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn setsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> ssize_t;

    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn make_sockaddr(
        family: c_int,
        addr: *const c_char,
        port: u16,
        sockaddr: *mut c_void,
        sockaddr_len: *mut socklen_t,
    ) -> c_int;

    fn crypto_sanity__open_and_load() -> *mut crypto_sanity;
    fn crypto_sanity__destroy(obj: *mut crypto_sanity);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_tc_detach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(res: c_uint, val: c_uint, name: *const c_char) -> bool;
    fn ASSERT_NEQ(res: c_int, val: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(res: ssize_t, val: ssize_t, name: *const c_char) -> bool;
    fn ASSERT_STRNEQ(actual: *const c_char, expected: *const c_char, len: usize, name: *const c_char)
        -> bool;
    fn RUN_TESTS_crypto_basic();
    fn SYS_fail(format: *const c_char, ...);
    fn SYS_NOFAIL(format: *const c_char, ...);
}

type socklen_t = c_uint;
type ssize_t = isize;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in6 {
    _data: [u8; 28],
}

#[repr(C)]
struct sockaddr_alg {
    salg_family: u16,
    salg_type: [u8; 14],
    salg_feat: u32,
    salg_mask: u32,
    salg_name: [u8; 64],
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}

#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: socklen_t,
    msg_iov: *mut iovec,
    msg_iovlen: usize,
    msg_control: *mut c_void,
    msg_controllen: usize,
    msg_flags: c_int,
}

#[repr(C)]
struct cmsghdr {
    cmsg_len: usize,
    cmsg_level: c_int,
    cmsg_type: c_int,
}

#[repr(C)]
struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_tc_hook {
    sz: usize,
    ifindex: c_uint,
    attach_point: c_int,
}

#[repr(C)]
struct bpf_tc_opts {
    sz: usize,
    prog_fd: c_int,
    flags: c_uint,
    prog_id: u32,
}

#[repr(C)]
struct bpf_test_run_opts {
    sz: usize,
    retval: c_int,
}

#[repr(C)]
struct crypto_sanity_bss {
    key_len: u32,
    authsize: u32,
    key: [u8; 128],
    algo: [c_char; 128],
    status: c_int,
    dst: [c_char; 16],
}

#[repr(C)]
struct crypto_sanity_data {
    udp_test_port: u16,
}

#[repr(C)]
struct crypto_sanity_progs {
    skb_crypto_setup: *mut bpf_program,
    encrypt_sanity: *mut bpf_program,
    decrypt_sanity: *mut bpf_program,
}

#[repr(C)]
struct crypto_sanity {
    bss: *mut crypto_sanity_bss,
    data: *mut crypto_sanity_data,
    progs: crypto_sanity_progs,
}

const AF_ALG: c_int = 38;
const AF_INET6: c_int = 10;
const SOCK_SEQPACKET: c_int = 5;
const SOCK_DGRAM: c_int = 2;
const SOL_ALG: c_int = 279;
const ALG_SET_KEY: c_int = 1;
const ALG_SET_OP: c_int = 3;
const ALG_OP_DECRYPT: u32 = 0;
const ALG_OP_ENCRYPT: u32 = 1;
const BPF_TC_EGRESS: c_int = 2;

unsafe fn cmsg_align(len: usize) -> usize {
    (len + mem::size_of::<usize>() - 1) & !(mem::size_of::<usize>() - 1)
}

unsafe fn cmsg_space(len: usize) -> usize {
    cmsg_align(mem::size_of::<cmsghdr>()) + cmsg_align(len)
}

unsafe fn cmsg_len(len: usize) -> usize {
    cmsg_align(mem::size_of::<cmsghdr>()) + len
}

unsafe fn cmsg_firsthdr(msg: *mut msghdr) -> *mut cmsghdr {
    if (*msg).msg_controllen >= mem::size_of::<cmsghdr>() {
        (*msg).msg_control as *mut cmsghdr
    } else {
        ptr::null_mut()
    }
}

unsafe fn cmsg_data(cmsg: *mut cmsghdr) -> *mut u8 {
    (cmsg as *mut u8).add(cmsg_align(mem::size_of::<cmsghdr>()))
}

unsafe fn init_afalg() -> c_int {
    let mut sa: sockaddr_alg = mem::zeroed();
    sa.salg_family = AF_ALG as u16;
    ptr::copy_nonoverlapping(b"skcipher\0".as_ptr(), sa.salg_type.as_mut_ptr(), b"skcipher\0".len());
    ptr::copy_nonoverlapping(b"ecb(aes)\0".as_ptr(), sa.salg_name.as_mut_ptr(), b"ecb(aes)\0".len());

    tfmfd = socket(AF_ALG, SOCK_SEQPACKET, 0);
    if tfmfd == -1 {
        return errno;
    }
    if bind(
        tfmfd,
        &sa as *const sockaddr_alg as *const sockaddr,
        mem::size_of_val(&sa) as socklen_t,
    ) == -1
    {
        return errno;
    }
    if setsockopt(
        tfmfd,
        SOL_ALG,
        ALG_SET_KEY,
        crypto_key.as_ptr() as *const c_void,
        16,
    ) == -1
    {
        return errno;
    }
    opfd = accept(tfmfd, ptr::null_mut(), ptr::null_mut());
    if opfd == -1 {
        return errno;
    }
    0
}

unsafe fn deinit_afalg() {
    if tfmfd != -1 {
        close(tfmfd);
    }
    if opfd != -1 {
        close(opfd);
    }
}

unsafe fn do_crypt_afalg(src: *const c_void, dst: *mut c_void, size: c_int, encrypt: bool) {
    let mut msg: msghdr = mem::zeroed();
    let mut cbuf = [0u8; cmsg_space(4)];
    let mut iov: iovec = mem::zeroed();

    msg.msg_control = cbuf.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = mem::size_of_val(&cbuf);

    let cmsg = cmsg_firsthdr(&mut msg);
    (*cmsg).cmsg_level = SOL_ALG;
    (*cmsg).cmsg_type = ALG_SET_OP;
    (*cmsg).cmsg_len = cmsg_len(4);
    *(cmsg_data(cmsg) as *mut u32) = if encrypt { ALG_OP_ENCRYPT } else { ALG_OP_DECRYPT };

    iov.iov_base = src as *mut c_void;
    iov.iov_len = size as usize;

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;

    sendmsg(opfd, &msg, 0);
    read(opfd, dst, size as usize);
}

#[no_mangle]
pub unsafe extern "C" fn test_crypto_basic() {
    RUN_TESTS_crypto_basic();
}

#[no_mangle]
pub unsafe extern "C" fn test_crypto_sanity() {
    let mut qdisc_hook = bpf_tc_hook {
        sz: mem::size_of::<bpf_tc_hook>(),
        ifindex: 0,
        attach_point: BPF_TC_EGRESS,
    };
    let mut tc_attach_enc: bpf_tc_opts = mem::zeroed();
    tc_attach_enc.sz = mem::size_of::<bpf_tc_opts>();
    let mut tc_attach_dec: bpf_tc_opts = mem::zeroed();
    tc_attach_dec.sz = mem::size_of::<bpf_tc_opts>();
    let mut opts: bpf_test_run_opts = mem::zeroed();
    opts.sz = mem::size_of::<bpf_test_run_opts>();
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let skel: *mut crypto_sanity;
    let mut afalg_plain = [0 as c_char; 16];
    let mut afalg_dst = [0 as c_char; 16];
    let mut addr: sockaddr_in6 = mem::zeroed();
    let mut sockfd: c_int;
    let mut err: c_int;
    let mut pfd: c_int;
    let mut addrlen: socklen_t;
    let udp_test_port: u16;

    skel = crypto_sanity__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel open\0".as_ptr() as *const c_char) {
        return;
    }

    SYS_fail(b"ip netns add %s\0".as_ptr() as *const c_char, NS_TEST.as_ptr());
    SYS_fail(
        b"ip -net %s -6 addr add %s/128 dev lo nodad\0".as_ptr() as *const c_char,
        NS_TEST.as_ptr(),
        IPV6_IFACE_ADDR.as_ptr(),
    );
    SYS_fail(
        b"ip -net %s link set dev lo up\0".as_ptr() as *const c_char,
        NS_TEST.as_ptr(),
    );

    nstoken = open_netns(NS_TEST.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *const c_void, b"open_netns\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }

    err = init_afalg();
    if !ASSERT_OK(err, b"AF_ALG init fail\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }

    qdisc_hook.ifindex = if_nametoindex(b"lo\0".as_ptr() as *const c_char);
    if !ASSERT_GT(qdisc_hook.ifindex, 0, b"if_nametoindex lo\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }

    (*(*skel).bss).key_len = 16;
    (*(*skel).bss).authsize = 0;
    udp_test_port = (*(*skel).data).udp_test_port;
    memcpy(
        (*(*skel).bss).key.as_mut_ptr() as *mut c_void,
        crypto_key.as_ptr() as *const c_void,
        mem::size_of_val(crypto_key),
    );
    snprintf(
        (*(*skel).bss).algo.as_mut_ptr(),
        128,
        b"%s\0".as_ptr() as *const c_char,
        algo.as_ptr(),
    );
    pfd = bpf_program__fd((*skel).progs.skb_crypto_setup);
    if !ASSERT_GT(pfd as c_uint, 0, b"skb_crypto_setup fd\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }

    err = bpf_prog_test_run_opts(pfd, &mut opts);
    if !ASSERT_OK(err, b"skb_crypto_setup\0".as_ptr() as *const c_char)
        || !ASSERT_OK(opts.retval, b"skb_crypto_setup retval\0".as_ptr() as *const c_char)
    {
        goto_fail(nstoken, skel);
        return;
    }

    if !ASSERT_OK((*(*skel).bss).status, b"skb_crypto_setup status\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }

    err = bpf_tc_hook_create(&mut qdisc_hook);
    if !ASSERT_OK(err, b"create qdisc hook\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }

    addrlen = mem::size_of_val(&addr) as socklen_t;
    err = make_sockaddr(
        AF_INET6,
        IPV6_IFACE_ADDR.as_ptr() as *const c_char,
        udp_test_port,
        &mut addr as *mut sockaddr_in6 as *mut c_void,
        &mut addrlen,
    );
    if !ASSERT_OK(err, b"make_sockaddr\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }

    tc_attach_enc.prog_fd = bpf_program__fd((*skel).progs.encrypt_sanity);
    err = bpf_tc_attach(&mut qdisc_hook, &mut tc_attach_enc);
    if !ASSERT_OK(err, b"attach encrypt filter\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }

    sockfd = socket(AF_INET6, SOCK_DGRAM, 0);
    if !ASSERT_NEQ(sockfd, -1, b"encrypt socket\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }
    err = sendto(
        sockfd,
        plain_text.as_ptr() as *const c_void,
        mem::size_of_val(plain_text),
        0,
        &addr as *const sockaddr_in6 as *const sockaddr,
        addrlen,
    ) as c_int;
    close(sockfd);
    if !ASSERT_EQ(
        err as ssize_t,
        mem::size_of_val(plain_text) as ssize_t,
        b"encrypt send\0".as_ptr() as *const c_char,
    ) {
        goto_fail(nstoken, skel);
        return;
    }

    do_crypt_afalg(
        plain_text.as_ptr() as *const c_void,
        afalg_dst.as_mut_ptr() as *mut c_void,
        mem::size_of_val(&afalg_dst) as c_int,
        true,
    );

    if !ASSERT_OK((*(*skel).bss).status, b"encrypt status\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }
    if !ASSERT_STRNEQ(
        (*(*skel).bss).dst.as_ptr(),
        afalg_dst.as_ptr(),
        mem::size_of_val(&afalg_dst),
        b"encrypt AF_ALG\0".as_ptr() as *const c_char,
    ) {
        goto_fail(nstoken, skel);
        return;
    }

    tc_attach_enc.flags = 0;
    tc_attach_enc.prog_fd = 0;
    tc_attach_enc.prog_id = 0;
    err = bpf_tc_detach(&mut qdisc_hook, &mut tc_attach_enc);
    if !ASSERT_OK(err, b"bpf_tc_detach encrypt\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }

    tc_attach_dec.prog_fd = bpf_program__fd((*skel).progs.decrypt_sanity);
    err = bpf_tc_attach(&mut qdisc_hook, &mut tc_attach_dec);
    if !ASSERT_OK(err, b"attach decrypt filter\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }

    sockfd = socket(AF_INET6, SOCK_DGRAM, 0);
    if !ASSERT_NEQ(sockfd, -1, b"decrypt socket\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }
    err = sendto(
        sockfd,
        afalg_dst.as_ptr() as *const c_void,
        mem::size_of_val(&afalg_dst),
        0,
        &addr as *const sockaddr_in6 as *const sockaddr,
        addrlen,
    ) as c_int;
    close(sockfd);
    if !ASSERT_EQ(
        err as ssize_t,
        mem::size_of_val(&afalg_dst) as ssize_t,
        b"decrypt send\0".as_ptr() as *const c_char,
    ) {
        goto_fail(nstoken, skel);
        return;
    }

    do_crypt_afalg(
        afalg_dst.as_ptr() as *const c_void,
        afalg_plain.as_mut_ptr() as *mut c_void,
        mem::size_of_val(&afalg_plain) as c_int,
        false,
    );

    if !ASSERT_OK((*(*skel).bss).status, b"decrypt status\0".as_ptr() as *const c_char) {
        goto_fail(nstoken, skel);
        return;
    }
    if !ASSERT_STRNEQ(
        (*(*skel).bss).dst.as_ptr(),
        afalg_plain.as_ptr(),
        mem::size_of_val(&afalg_plain),
        b"decrypt AF_ALG\0".as_ptr() as *const c_char,
    ) {
        goto_fail(nstoken, skel);
        return;
    }

    tc_attach_dec.flags = 0;
    tc_attach_dec.prog_fd = 0;
    tc_attach_dec.prog_id = 0;
    err = bpf_tc_detach(&mut qdisc_hook, &mut tc_attach_dec);
    ASSERT_OK(err, b"bpf_tc_detach decrypt\0".as_ptr() as *const c_char);

    goto_fail(nstoken, skel);
}

unsafe fn goto_fail(nstoken: *mut nstoken, skel: *mut crypto_sanity) {
    close_netns(nstoken);
    deinit_afalg();
    SYS_NOFAIL(b"ip netns del crypto_sanity_ns &> /dev/null\0".as_ptr() as *const c_char);
    crypto_sanity__destroy(skel);
}
