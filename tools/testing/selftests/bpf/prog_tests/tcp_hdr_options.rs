// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/* Translated from C implementation source.  C includes are represented by
 * external declarations below; dependent symbols are expected from the test
 * harness/libbpf skeleton bindings.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u32 = u32;
type socklen_t = u32;

const LO_ADDR6: &[u8] = b"::1\0";
const CG_NAME: &[u8] = b"/tcpbpf-hdr-opt-test\0";

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SHUT_WR: c_int = 1;
const MSG_EOR: c_int = 0x80;
const CLONE_NEWNET: c_int = 0x40000000;
const TCPOPT_EXP: c_int = 254;

/* Constants supplied by the BPF TCP header option test headers. */
extern "C" {
    static OPTION_F_RAND: __u32;
    static OPTION_F_MAX_DELACK_MS: __u32;
    static OPTION_F_RESEND: __u32;
    static BPF_SOCK_OPS_PARSE_UNKNOWN_HDR_OPT_CB_FLAG: __u32;
    static BPF_SOCK_OPS_WRITE_HDR_OPT_CB_FLAG: __u32;
    static BPF_SOCK_OPS_STATE_CB_FLAG: __u32;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_test_option {
    flags: __u32,
    max_delack_ms: __u32,
    rand: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct hdr_stg {
    active: bool,
    resend_syn: bool,
    syncookie: bool,
    fastopen: bool,
}

#[repr(C)]
struct linum_err {
    linum: c_uint,
    err: c_int,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
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
struct test_tcp_hdr_options_bss {
    passive_synack_out: bpf_test_option,
    passive_fin_out: bpf_test_option,
    passive_estab_in: bpf_test_option,
    passive_fin_in: bpf_test_option,
    active_syn_out: bpf_test_option,
    active_fin_out: bpf_test_option,
    active_estab_in: bpf_test_option,
    active_fin_in: bpf_test_option,
    inherit_cb_flags: __u32,
}

#[repr(C)]
struct test_tcp_hdr_options_data {
    test_kind: c_int,
    test_magic: c_int,
}

#[repr(C)]
struct test_tcp_hdr_options_maps {
    hdr_stg_map: *mut bpf_map,
    lport_linum_map: *mut bpf_map,
}

#[repr(C)]
struct test_tcp_hdr_options_progs {
    estab: *mut bpf_program,
}

#[repr(C)]
struct test_tcp_hdr_options {
    bss: *mut test_tcp_hdr_options_bss,
    data: *mut test_tcp_hdr_options_data,
    maps: test_tcp_hdr_options_maps,
    progs: test_tcp_hdr_options_progs,
}

#[repr(C)]
struct test_misc_tcp_hdr_options_bss {
    nr_syn: c_uint,
    nr_data: c_uint,
    nr_pure_ack: c_uint,
    nr_fin: c_uint,
    nr_hwtstamp: c_uint,
    nodelay_est_ok: bool,
    nodelay_hdr_len_reject: bool,
    nodelay_write_hdr_reject: bool,
}

#[repr(C)]
struct test_misc_tcp_hdr_options_maps {
    lport_linum_map: *mut bpf_map,
}

#[repr(C)]
struct test_misc_tcp_hdr_options_progs {
    misc_estab: *mut bpf_program,
}

#[repr(C)]
struct test_misc_tcp_hdr_options {
    bss: *mut test_misc_tcp_hdr_options_bss,
    maps: test_misc_tcp_hdr_options_maps,
    progs: test_misc_tcp_hdr_options_progs,
}

#[repr(C)]
struct sk_fds {
    srv_fd: c_int,
    passive_fd: c_int,
    active_fd: c_int,
    passive_lport: c_int,
    active_lport: c_int,
}

#[repr(C)]
struct test {
    desc: *const c_char,
    run: Option<unsafe extern "C" fn()>,
}

extern "C" {
    static mut stderr: *mut c_void;

    fn unshare(flags: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn shutdown(fd: c_int, how: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn accept(fd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn ntohs(netshort: u16) -> u16;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_FALSE(actual: c_uint, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_TRUE(actual: bool, name: *const c_char) -> bool;
    fn CHECK(condition: bool, name: *const c_char, format: *const c_char, ...) -> bool;

    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn fastopen_connect(fd: c_int, data: *const c_void, data_len: usize, flags: c_int) -> c_int;
    fn connect_to_fd(fd: c_int, flags: c_int) -> c_int;
    fn write_sysctl(path: *const c_char, value: *const c_char) -> c_int;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn test_tcp_hdr_options__open_and_load() -> *mut test_tcp_hdr_options;
    fn test_tcp_hdr_options__destroy(obj: *mut test_tcp_hdr_options);
    fn test_misc_tcp_hdr_options__open_and_load() -> *mut test_misc_tcp_hdr_options;
    fn test_misc_tcp_hdr_options__destroy(obj: *mut test_misc_tcp_hdr_options);
}

static mut exp_passive_estab_in: bpf_test_option = bpf_test_option { flags: 0, max_delack_ms: 0, rand: 0 };
static mut exp_active_estab_in: bpf_test_option = bpf_test_option { flags: 0, max_delack_ms: 0, rand: 0 };
static mut exp_passive_fin_in: bpf_test_option = bpf_test_option { flags: 0, max_delack_ms: 0, rand: 0 };
static mut exp_active_fin_in: bpf_test_option = bpf_test_option { flags: 0, max_delack_ms: 0, rand: 0 };
static mut exp_passive_hdr_stg: hdr_stg = hdr_stg { active: false, resend_syn: false, syncookie: false, fastopen: false };
static mut exp_active_hdr_stg: hdr_stg = hdr_stg { active: true, resend_syn: false, syncookie: false, fastopen: false };

static mut misc_skel: *mut test_misc_tcp_hdr_options = core::ptr::null_mut();
static mut skel: *mut test_tcp_hdr_options = core::ptr::null_mut();
static mut lport_linum_map_fd: c_int = 0;
static mut hdr_stg_map_fd: c_int = 0;
static mut duration: __u32 = 0;
static mut cg_fd: c_int = 0;

unsafe extern "C" fn create_netns() -> c_int {
    if !ASSERT_OK(unshare(CLONE_NEWNET), b"create netns\0".as_ptr() as *const c_char) {
        return -1;
    }

    if !ASSERT_OK(system(b"ip link set dev lo up\0".as_ptr() as *const c_char), b"run ip cmd\0".as_ptr() as *const c_char) {
        return -1;
    }

    0
}

unsafe extern "C" fn print_hdr_stg(hdr_stg_: *const hdr_stg, prefix: *const c_char) {
    fprintf(
        stderr,
        b"%s{active:%u, resend_syn:%u, syncookie:%u, fastopen:%u}\n\0".as_ptr() as *const c_char,
        if !prefix.is_null() { prefix } else { b"\0".as_ptr() as *const c_char },
        (*hdr_stg_).active as c_uint,
        (*hdr_stg_).resend_syn as c_uint,
        (*hdr_stg_).syncookie as c_uint,
        (*hdr_stg_).fastopen as c_uint,
    );
}

unsafe extern "C" fn print_option(opt: *const bpf_test_option, prefix: *const c_char) {
    fprintf(
        stderr,
        b"%s{flags:0x%x, max_delack_ms:%u, rand:0x%x}\n\0".as_ptr() as *const c_char,
        if !prefix.is_null() { prefix } else { b"\0".as_ptr() as *const c_char },
        (*opt).flags,
        (*opt).max_delack_ms,
        (*opt).rand,
    );
}

unsafe extern "C" fn sk_fds_close(sk_fds_: *mut sk_fds) {
    close((*sk_fds_).srv_fd);
    close((*sk_fds_).passive_fd);
    close((*sk_fds_).active_fd);
}

unsafe extern "C" fn sk_fds_shutdown(sk_fds_: *mut sk_fds) -> c_int {
    let mut ret: c_int;
    let mut abyte: c_int = 0;

    shutdown((*sk_fds_).active_fd, SHUT_WR);
    ret = read((*sk_fds_).passive_fd, &mut abyte as *mut _ as *mut c_void, core::mem::size_of_val(&abyte)) as c_int;
    if !ASSERT_EQ(ret, 0, b"read-after-shutdown(passive_fd):\0".as_ptr() as *const c_char) {
        return -1;
    }

    shutdown((*sk_fds_).passive_fd, SHUT_WR);
    ret = read((*sk_fds_).active_fd, &mut abyte as *mut _ as *mut c_void, core::mem::size_of_val(&abyte)) as c_int;
    if !ASSERT_EQ(ret, 0, b"read-after-shutdown(active_fd):\0".as_ptr() as *const c_char) {
        return -1;
    }

    0
}

unsafe extern "C" fn sk_fds_connect(sk_fds_: *mut sk_fds, fast_open: bool) -> c_int {
    let fast = *b"FAST!!!\0";
    let mut addr6: sockaddr_in6 = core::mem::zeroed();
    let mut len: socklen_t;

    (*sk_fds_).srv_fd = start_server(AF_INET6, SOCK_STREAM, LO_ADDR6.as_ptr() as *const c_char, 0, 0);
    if !ASSERT_NEQ((*sk_fds_).srv_fd, -1, b"start_server\0".as_ptr() as *const c_char) {
        goto_error(sk_fds_);
        return -1;
    }

    if fast_open {
        (*sk_fds_).active_fd = fastopen_connect((*sk_fds_).srv_fd, fast.as_ptr() as *const c_void, fast.len(), 0);
    } else {
        (*sk_fds_).active_fd = connect_to_fd((*sk_fds_).srv_fd, 0);
    }

    if !ASSERT_NEQ((*sk_fds_).active_fd, -1, b"\0".as_ptr() as *const c_char) {
        close((*sk_fds_).srv_fd);
        goto_error(sk_fds_);
        return -1;
    }

    len = core::mem::size_of_val(&addr6) as socklen_t;
    if !ASSERT_OK(
        getsockname((*sk_fds_).srv_fd, &mut addr6 as *mut _ as *mut sockaddr, &mut len),
        b"getsockname(srv_fd)\0".as_ptr() as *const c_char,
    ) {
        goto_error_close(sk_fds_);
        return -1;
    }
    (*sk_fds_).passive_lport = ntohs(addr6.sin6_port) as c_int;

    len = core::mem::size_of_val(&addr6) as socklen_t;
    if !ASSERT_OK(
        getsockname((*sk_fds_).active_fd, &mut addr6 as *mut _ as *mut sockaddr, &mut len),
        b"getsockname(active_fd)\0".as_ptr() as *const c_char,
    ) {
        goto_error_close(sk_fds_);
        return -1;
    }
    (*sk_fds_).active_lport = ntohs(addr6.sin6_port) as c_int;

    (*sk_fds_).passive_fd = accept((*sk_fds_).srv_fd, core::ptr::null_mut(), core::ptr::null_mut());
    if !ASSERT_NEQ((*sk_fds_).passive_fd, -1, b"accept(srv_fd)\0".as_ptr() as *const c_char) {
        goto_error_close(sk_fds_);
        return -1;
    }

    if fast_open {
        let mut bytes_in = [0 as c_char; 8];
        let ret: c_int = read((*sk_fds_).passive_fd, bytes_in.as_mut_ptr() as *mut c_void, bytes_in.len()) as c_int;
        if !ASSERT_EQ(ret, fast.len() as c_int, b"read fastopen syn data\0".as_ptr() as *const c_char) {
            close((*sk_fds_).passive_fd);
            goto_error_close(sk_fds_);
            return -1;
        }
    }

    0
}

unsafe fn goto_error_close(sk_fds_: *mut sk_fds) {
    close((*sk_fds_).active_fd);
    close((*sk_fds_).srv_fd);
    goto_error(sk_fds_);
}

unsafe fn goto_error(sk_fds_: *mut sk_fds) {
    memset(sk_fds_ as *mut c_void, -1, core::mem::size_of::<sk_fds>());
}

unsafe extern "C" fn check_hdr_opt(exp: *const bpf_test_option, act: *const bpf_test_option, hdr_desc: *const c_char) -> c_int {
    if !ASSERT_EQ(memcmp(exp as *const c_void, act as *const c_void, core::mem::size_of::<bpf_test_option>()), 0, hdr_desc) {
        print_option(exp, b"expected: \0".as_ptr() as *const c_char);
        print_option(act, b"  actual: \0".as_ptr() as *const c_char);
        return -1;
    }

    0
}

unsafe extern "C" fn check_hdr_stg(exp: *const hdr_stg, fd: c_int, stg_desc: *const c_char) -> c_int {
    let mut act: hdr_stg = core::mem::zeroed();

    if !ASSERT_OK(
        bpf_map_lookup_elem(hdr_stg_map_fd, &fd as *const _ as *const c_void, &mut act as *mut _ as *mut c_void),
        b"map_lookup(hdr_stg_map_fd)\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    if !ASSERT_EQ(memcmp(exp as *const c_void, &act as *const _ as *const c_void, core::mem::size_of::<hdr_stg>()), 0, stg_desc) {
        print_hdr_stg(exp, b"expected: \0".as_ptr() as *const c_char);
        print_hdr_stg(&act, b"  actual: \0".as_ptr() as *const c_char);
        return -1;
    }

    0
}

unsafe extern "C" fn check_error_linum(sk_fds_: *const sk_fds) -> c_uint {
    let mut nr_errors: c_uint = 0;
    let mut linum_err_: linum_err = core::mem::zeroed();
    let mut lport: c_int;

    lport = (*sk_fds_).passive_lport;
    if bpf_map_lookup_elem(lport_linum_map_fd, &lport as *const _ as *const c_void, &mut linum_err_ as *mut _ as *mut c_void) == 0 {
        fprintf(
            stderr,
            b"bpf prog error out at lport:passive(%d), linum:%u err:%d\n\0".as_ptr() as *const c_char,
            lport,
            linum_err_.linum,
            linum_err_.err,
        );
        nr_errors += 1;
    }

    lport = (*sk_fds_).active_lport;
    if bpf_map_lookup_elem(lport_linum_map_fd, &lport as *const _ as *const c_void, &mut linum_err_ as *mut _ as *mut c_void) == 0 {
        fprintf(
            stderr,
            b"bpf prog error out at lport:active(%d), linum:%u err:%d\n\0".as_ptr() as *const c_char,
            lport,
            linum_err_.linum,
            linum_err_.err,
        );
        nr_errors += 1;
    }

    nr_errors
}

unsafe extern "C" fn check_hdr_and_close_fds(sk_fds_: *mut sk_fds) {
    let expected_inherit_cb_flags: __u32 =
        BPF_SOCK_OPS_PARSE_UNKNOWN_HDR_OPT_CB_FLAG |
        BPF_SOCK_OPS_WRITE_HDR_OPT_CB_FLAG |
        BPF_SOCK_OPS_STATE_CB_FLAG;

    if sk_fds_shutdown(sk_fds_) != 0 {
        ASSERT_FALSE(check_error_linum(sk_fds_), b"check_error_linum\0".as_ptr() as *const c_char);
        sk_fds_close(sk_fds_);
        return;
    }

    if !ASSERT_EQ(expected_inherit_cb_flags as c_int, (*(*skel).bss).inherit_cb_flags as c_int, b"inherit_cb_flags\0".as_ptr() as *const c_char) {
        ASSERT_FALSE(check_error_linum(sk_fds_), b"check_error_linum\0".as_ptr() as *const c_char);
        sk_fds_close(sk_fds_);
        return;
    }

    if check_hdr_stg(&exp_passive_hdr_stg, (*sk_fds_).passive_fd, b"passive_hdr_stg\0".as_ptr() as *const c_char) != 0 ||
       check_hdr_stg(&exp_active_hdr_stg, (*sk_fds_).active_fd, b"active_hdr_stg\0".as_ptr() as *const c_char) != 0 ||
       check_hdr_opt(&exp_passive_estab_in, &(*(*skel).bss).passive_estab_in, b"passive_estab_in\0".as_ptr() as *const c_char) != 0 ||
       check_hdr_opt(&exp_active_estab_in, &(*(*skel).bss).active_estab_in, b"active_estab_in\0".as_ptr() as *const c_char) != 0 ||
       check_hdr_opt(&exp_passive_fin_in, &(*(*skel).bss).passive_fin_in, b"passive_fin_in\0".as_ptr() as *const c_char) != 0 {
        ASSERT_FALSE(check_error_linum(sk_fds_), b"check_error_linum\0".as_ptr() as *const c_char);
        sk_fds_close(sk_fds_);
        return;
    }

    check_hdr_opt(&exp_active_fin_in, &(*(*skel).bss).active_fin_in, b"active_fin_in\0".as_ptr() as *const c_char);

    ASSERT_FALSE(check_error_linum(sk_fds_), b"check_error_linum\0".as_ptr() as *const c_char);
    sk_fds_close(sk_fds_);
}

unsafe extern "C" fn prepare_out() {
    (*(*skel).bss).active_syn_out = exp_passive_estab_in;
    (*(*skel).bss).passive_synack_out = exp_active_estab_in;

    (*(*skel).bss).active_fin_out = exp_passive_fin_in;
    (*(*skel).bss).passive_fin_out = exp_active_fin_in;
}

unsafe extern "C" fn reset_test() {
    let optsize = core::mem::size_of::<bpf_test_option>();
    let mut lport: c_int = 0;
    let mut err: c_int;

    memset(&mut (*(*skel).bss).passive_synack_out as *mut _ as *mut c_void, 0, optsize);
    memset(&mut (*(*skel).bss).passive_fin_out as *mut _ as *mut c_void, 0, optsize);

    memset(&mut (*(*skel).bss).passive_estab_in as *mut _ as *mut c_void, 0, optsize);
    memset(&mut (*(*skel).bss).passive_fin_in as *mut _ as *mut c_void, 0, optsize);

    memset(&mut (*(*skel).bss).active_syn_out as *mut _ as *mut c_void, 0, optsize);
    memset(&mut (*(*skel).bss).active_fin_out as *mut _ as *mut c_void, 0, optsize);

    memset(&mut (*(*skel).bss).active_estab_in as *mut _ as *mut c_void, 0, optsize);
    memset(&mut (*(*skel).bss).active_fin_in as *mut _ as *mut c_void, 0, optsize);

    (*(*skel).bss).inherit_cb_flags = 0;

    (*(*skel).data).test_kind = TCPOPT_EXP;
    (*(*skel).data).test_magic = 0xeB9F;

    memset(&mut exp_passive_estab_in as *mut _ as *mut c_void, 0, optsize);
    memset(&mut exp_active_estab_in as *mut _ as *mut c_void, 0, optsize);
    memset(&mut exp_passive_fin_in as *mut _ as *mut c_void, 0, optsize);
    memset(&mut exp_active_fin_in as *mut _ as *mut c_void, 0, optsize);

    memset(&mut exp_passive_hdr_stg as *mut _ as *mut c_void, 0, core::mem::size_of_val(&exp_passive_hdr_stg));
    memset(&mut exp_active_hdr_stg as *mut _ as *mut c_void, 0, core::mem::size_of_val(&exp_active_hdr_stg));
    exp_active_hdr_stg.active = true;

    err = bpf_map_get_next_key(lport_linum_map_fd, core::ptr::null(), &mut lport as *mut _ as *mut c_void);
    while err == 0 {
        bpf_map_delete_elem(lport_linum_map_fd, &lport as *const _ as *const c_void);
        err = bpf_map_get_next_key(lport_linum_map_fd, &lport as *const _ as *const c_void, &mut lport as *mut _ as *mut c_void);
    }
}

unsafe extern "C" fn fastopen_estab() {
    let mut link: *mut bpf_link;
    let mut sk_fds_: sk_fds = core::mem::zeroed();

    hdr_stg_map_fd = bpf_map__fd((*skel).maps.hdr_stg_map);
    lport_linum_map_fd = bpf_map__fd((*skel).maps.lport_linum_map);

    exp_passive_estab_in.flags = OPTION_F_RAND | OPTION_F_MAX_DELACK_MS;
    exp_passive_estab_in.rand = 0xfa;
    exp_passive_estab_in.max_delack_ms = 11;

    exp_active_estab_in.flags = OPTION_F_RAND | OPTION_F_MAX_DELACK_MS;
    exp_active_estab_in.rand = 0xce;
    exp_active_estab_in.max_delack_ms = 22;

    exp_passive_hdr_stg.fastopen = true;

    prepare_out();

    /* Allow fastopen without fastopen cookie */
    if write_sysctl(b"/proc/sys/net/ipv4/tcp_fastopen\0".as_ptr() as *const c_char, b"1543\0".as_ptr() as *const c_char) != 0 {
        return;
    }

    link = bpf_program__attach_cgroup((*skel).progs.estab, cg_fd);
    if !ASSERT_OK_PTR(link as *mut c_void, b"attach_cgroup(estab)\0".as_ptr() as *const c_char) {
        return;
    }

    if sk_fds_connect(&mut sk_fds_, true) != 0 {
        bpf_link__destroy(link);
        return;
    }

    check_hdr_and_close_fds(&mut sk_fds_);
    bpf_link__destroy(link);
}

unsafe extern "C" fn syncookie_estab() {
    let mut link: *mut bpf_link;
    let mut sk_fds_: sk_fds = core::mem::zeroed();

    hdr_stg_map_fd = bpf_map__fd((*skel).maps.hdr_stg_map);
    lport_linum_map_fd = bpf_map__fd((*skel).maps.lport_linum_map);

    exp_passive_estab_in.flags = OPTION_F_RAND | OPTION_F_MAX_DELACK_MS;
    exp_passive_estab_in.rand = 0xfa;
    exp_passive_estab_in.max_delack_ms = 11;

    exp_active_estab_in.flags = OPTION_F_RAND | OPTION_F_MAX_DELACK_MS | OPTION_F_RESEND;
    exp_active_estab_in.rand = 0xce;
    exp_active_estab_in.max_delack_ms = 22;

    exp_passive_hdr_stg.syncookie = true;
    exp_active_hdr_stg.resend_syn = true;

    prepare_out();

    /* Clear the RESEND to ensure the bpf prog can learn
     * want_cookie and set the RESEND by itself.
     */
    (*(*skel).bss).passive_synack_out.flags &= !OPTION_F_RESEND;

    /* Enforce syncookie mode */
    if write_sysctl(b"/proc/sys/net/ipv4/tcp_syncookies\0".as_ptr() as *const c_char, b"2\0".as_ptr() as *const c_char) != 0 {
        return;
    }

    link = bpf_program__attach_cgroup((*skel).progs.estab, cg_fd);
    if !ASSERT_OK_PTR(link as *mut c_void, b"attach_cgroup(estab)\0".as_ptr() as *const c_char) {
        return;
    }

    if sk_fds_connect(&mut sk_fds_, false) != 0 {
        bpf_link__destroy(link);
        return;
    }

    check_hdr_and_close_fds(&mut sk_fds_);
    bpf_link__destroy(link);
}

unsafe extern "C" fn fin() {
    let mut link: *mut bpf_link;
    let mut sk_fds_: sk_fds = core::mem::zeroed();

    hdr_stg_map_fd = bpf_map__fd((*skel).maps.hdr_stg_map);
    lport_linum_map_fd = bpf_map__fd((*skel).maps.lport_linum_map);

    exp_passive_fin_in.flags = OPTION_F_RAND;
    exp_passive_fin_in.rand = 0xfa;

    exp_active_fin_in.flags = OPTION_F_RAND;
    exp_active_fin_in.rand = 0xce;

    prepare_out();

    if write_sysctl(b"/proc/sys/net/ipv4/tcp_syncookies\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) != 0 {
        return;
    }

    link = bpf_program__attach_cgroup((*skel).progs.estab, cg_fd);
    if !ASSERT_OK_PTR(link as *mut c_void, b"attach_cgroup(estab)\0".as_ptr() as *const c_char) {
        return;
    }

    if sk_fds_connect(&mut sk_fds_, false) != 0 {
        bpf_link__destroy(link);
        return;
    }

    check_hdr_and_close_fds(&mut sk_fds_);
    bpf_link__destroy(link);
}

unsafe extern "C" fn __simple_estab(exprm: bool) {
    let mut link: *mut bpf_link;
    let mut sk_fds_: sk_fds = core::mem::zeroed();

    hdr_stg_map_fd = bpf_map__fd((*skel).maps.hdr_stg_map);
    lport_linum_map_fd = bpf_map__fd((*skel).maps.lport_linum_map);

    exp_passive_estab_in.flags = OPTION_F_RAND | OPTION_F_MAX_DELACK_MS;
    exp_passive_estab_in.rand = 0xfa;
    exp_passive_estab_in.max_delack_ms = 11;

    exp_active_estab_in.flags = OPTION_F_RAND | OPTION_F_MAX_DELACK_MS;
    exp_active_estab_in.rand = 0xce;
    exp_active_estab_in.max_delack_ms = 22;

    prepare_out();

    if !exprm {
        (*(*skel).data).test_kind = 0xB9;
        (*(*skel).data).test_magic = 0;
    }

    if write_sysctl(b"/proc/sys/net/ipv4/tcp_syncookies\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) != 0 {
        return;
    }

    link = bpf_program__attach_cgroup((*skel).progs.estab, cg_fd);
    if !ASSERT_OK_PTR(link as *mut c_void, b"attach_cgroup(estab)\0".as_ptr() as *const c_char) {
        return;
    }

    if sk_fds_connect(&mut sk_fds_, false) != 0 {
        bpf_link__destroy(link);
        return;
    }

    check_hdr_and_close_fds(&mut sk_fds_);
    bpf_link__destroy(link);
}

unsafe extern "C" fn no_exprm_estab() {
    __simple_estab(false);
}

unsafe extern "C" fn simple_estab() {
    __simple_estab(true);
}

unsafe extern "C" fn misc() {
    let send_msg = *b"MISC!!!\0";
    let mut recv_msg = [0 as c_char; 8];
    let nr_data: c_uint = 2;
    let mut link: *mut bpf_link;
    let mut sk_fds_: sk_fds = core::mem::zeroed();
    let mut i: c_int;
    let mut ret: c_int;

    lport_linum_map_fd = bpf_map__fd((*misc_skel).maps.lport_linum_map);

    if write_sysctl(b"/proc/sys/net/ipv4/tcp_syncookies\0".as_ptr() as *const c_char, b"1\0".as_ptr() as *const c_char) != 0 {
        return;
    }

    link = bpf_program__attach_cgroup((*misc_skel).progs.misc_estab, cg_fd);
    if !ASSERT_OK_PTR(link as *mut c_void, b"attach_cgroup(misc_estab)\0".as_ptr() as *const c_char) {
        return;
    }

    if sk_fds_connect(&mut sk_fds_, false) != 0 {
        bpf_link__destroy(link);
        return;
    }

    i = 0;
    while i < nr_data as c_int {
        /* MSG_EOR to ensure skb will not be combined */
        ret = send((*(&mut sk_fds_ as *mut sk_fds)).active_fd, send_msg.as_ptr() as *const c_void, send_msg.len(), MSG_EOR) as c_int;
        if !ASSERT_EQ(ret, send_msg.len() as c_int, b"send(msg)\0".as_ptr() as *const c_char) {
            ASSERT_FALSE(check_error_linum(&sk_fds_), b"check_error_linum\0".as_ptr() as *const c_char);
            sk_fds_close(&mut sk_fds_);
            bpf_link__destroy(link);
            return;
        }

        ret = read(sk_fds_.passive_fd, recv_msg.as_mut_ptr() as *mut c_void, recv_msg.len()) as c_int;
        if !ASSERT_EQ(ret, send_msg.len() as c_int, b"read(msg)\0".as_ptr() as *const c_char) {
            ASSERT_FALSE(check_error_linum(&sk_fds_), b"check_error_linum\0".as_ptr() as *const c_char);
            sk_fds_close(&mut sk_fds_);
            bpf_link__destroy(link);
            return;
        }
        i += 1;
    }

    if sk_fds_shutdown(&mut sk_fds_) != 0 {
        ASSERT_FALSE(check_error_linum(&sk_fds_), b"check_error_linum\0".as_ptr() as *const c_char);
        sk_fds_close(&mut sk_fds_);
        bpf_link__destroy(link);
        return;
    }

    ASSERT_EQ((*(*misc_skel).bss).nr_syn as c_int, 1, b"unexpected nr_syn\0".as_ptr() as *const c_char);

    ASSERT_EQ((*(*misc_skel).bss).nr_data as c_int, nr_data as c_int, b"unexpected nr_data\0".as_ptr() as *const c_char);

    /* The last ACK may have been delayed, so it is either 1 or 2. */
    CHECK(
        (*(*misc_skel).bss).nr_pure_ack != 1 && (*(*misc_skel).bss).nr_pure_ack != 2,
        b"unexpected nr_pure_ack\0".as_ptr() as *const c_char,
        b"expected (1 or 2) != actual (%u)\n\0".as_ptr() as *const c_char,
        (*(*misc_skel).bss).nr_pure_ack,
    );

    ASSERT_EQ((*(*misc_skel).bss).nr_fin as c_int, 1, b"unexpected nr_fin\0".as_ptr() as *const c_char);

    ASSERT_EQ((*(*misc_skel).bss).nr_hwtstamp as c_int, 0, b"nr_hwtstamp\0".as_ptr() as *const c_char);

    ASSERT_TRUE((*(*misc_skel).bss).nodelay_est_ok, b"nodelay_est_ok\0".as_ptr() as *const c_char);
    ASSERT_TRUE((*(*misc_skel).bss).nodelay_hdr_len_reject, b"nodelay_hdr_len_reject\0".as_ptr() as *const c_char);
    ASSERT_TRUE((*(*misc_skel).bss).nodelay_write_hdr_reject, b"nodelay_write_hdr_reject\0".as_ptr() as *const c_char);

    ASSERT_FALSE(check_error_linum(&sk_fds_), b"check_error_linum\0".as_ptr() as *const c_char);
    sk_fds_close(&mut sk_fds_);
    bpf_link__destroy(link);
}

/* #define DEF_TEST(name) { #name, name } */
static mut tests: [test; 6] = [
    test { desc: b"simple_estab\0".as_ptr() as *const c_char, run: Some(simple_estab) },
    test { desc: b"no_exprm_estab\0".as_ptr() as *const c_char, run: Some(no_exprm_estab) },
    test { desc: b"syncookie_estab\0".as_ptr() as *const c_char, run: Some(syncookie_estab) },
    test { desc: b"fastopen_estab\0".as_ptr() as *const c_char, run: Some(fastopen_estab) },
    test { desc: b"fin\0".as_ptr() as *const c_char, run: Some(fin) },
    test { desc: b"misc\0".as_ptr() as *const c_char, run: Some(misc) },
];

#[no_mangle]
pub unsafe extern "C" fn test_tcp_hdr_options() {
    let mut i: usize;

    skel = test_tcp_hdr_options__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, b"open and load skel\0".as_ptr() as *const c_char) {
        return;
    }

    misc_skel = test_misc_tcp_hdr_options__open_and_load();
    if !ASSERT_OK_PTR(misc_skel as *mut c_void, b"open and load misc test skel\0".as_ptr() as *const c_char) {
        test_misc_tcp_hdr_options__destroy(misc_skel);
        test_tcp_hdr_options__destroy(skel);
        return;
    }

    cg_fd = test__join_cgroup(CG_NAME.as_ptr() as *const c_char);
    if !ASSERT_GE(cg_fd, 0, b"join_cgroup\0".as_ptr() as *const c_char) {
        test_misc_tcp_hdr_options__destroy(misc_skel);
        test_tcp_hdr_options__destroy(skel);
        return;
    }

    i = 0;
    while i < tests.len() {
        if !test__start_subtest(tests[i].desc) {
            i += 1;
            continue;
        }

        if create_netns() != 0 {
            break;
        }

        if let Some(run) = tests[i].run {
            run();
        }

        reset_test();
        i += 1;
    }

    close(cg_fd);
    test_misc_tcp_hdr_options__destroy(misc_skel);
    test_tcp_hdr_options__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
