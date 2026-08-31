// SPDX-License-Identifier: GPL-2.0
// Translated from C source:
//   includes: test_progs.h, network_helpers.h,
//   linux/netfilter/nf_conntrack_common.h,
//   test_bpf_nf.skel.h, test_bpf_nf_fail.skel.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const CT_OPTS_ERROR_GUARD: c_int = 0x12345678;

static mut LOG_BUF: [c_char; 1024 * 1024] = [0; 1024 * 1024];

#[repr(C)]
struct TestBpfNfFailTest {
    prog_name: *const c_char,
    err_msg: *const c_char,
}

static TEST_BPF_NF_FAIL_TESTS: [TestBpfNfFailTest; 12] = [
    TestBpfNfFailTest {
        prog_name: c"alloc_release".as_ptr(),
        err_msg: c"kernel function bpf_ct_release R1 expected pointer to STRUCT nf_conn but".as_ptr(),
    },
    TestBpfNfFailTest {
        prog_name: c"insert_insert".as_ptr(),
        err_msg: c"kernel function bpf_ct_insert_entry R1 expected pointer to STRUCT nf_conn___init but".as_ptr(),
    },
    TestBpfNfFailTest {
        prog_name: c"lookup_insert".as_ptr(),
        err_msg: c"kernel function bpf_ct_insert_entry R1 expected pointer to STRUCT nf_conn___init but".as_ptr(),
    },
    TestBpfNfFailTest {
        prog_name: c"set_timeout_after_insert".as_ptr(),
        err_msg: c"kernel function bpf_ct_set_timeout R1 expected pointer to STRUCT nf_conn___init but".as_ptr(),
    },
    TestBpfNfFailTest {
        prog_name: c"set_status_after_insert".as_ptr(),
        err_msg: c"kernel function bpf_ct_set_status R1 expected pointer to STRUCT nf_conn___init but".as_ptr(),
    },
    TestBpfNfFailTest {
        prog_name: c"change_timeout_after_alloc".as_ptr(),
        err_msg: c"kernel function bpf_ct_change_timeout R1 expected pointer to STRUCT nf_conn but".as_ptr(),
    },
    TestBpfNfFailTest {
        prog_name: c"change_status_after_alloc".as_ptr(),
        err_msg: c"kernel function bpf_ct_change_status R1 expected pointer to STRUCT nf_conn but".as_ptr(),
    },
    TestBpfNfFailTest {
        prog_name: c"write_not_allowlisted_field".as_ptr(),
        err_msg: c"no write support to nf_conn at off".as_ptr(),
    },
    TestBpfNfFailTest {
        prog_name: c"lookup_null_bpf_tuple".as_ptr(),
        err_msg: c"Possibly NULL pointer passed to trusted R2".as_ptr(),
    },
    TestBpfNfFailTest {
        prog_name: c"lookup_null_bpf_opts".as_ptr(),
        err_msg: c"Possibly NULL pointer passed to trusted R4".as_ptr(),
    },
    TestBpfNfFailTest {
        prog_name: c"xdp_lookup_null_bpf_tuple".as_ptr(),
        err_msg: c"Possibly NULL pointer passed to trusted R2".as_ptr(),
    },
    TestBpfNfFailTest {
        prog_name: c"xdp_lookup_null_bpf_opts".as_ptr(),
        err_msg: c"Possibly NULL pointer passed to trusted R4".as_ptr(),
    },
];

const TEST_XDP: c_int = 0;
const TEST_TC_BPF: c_int = 1;

const TIMEOUT_MS: c_int = 3000;
const IPS_STATUS_MASK: c_uint = IPS_CONFIRMED
    | IPS_SEEN_REPLY
    | IPS_SRC_NAT_DONE
    | IPS_DST_NAT_DONE
    | IPS_SRC_NAT
    | IPS_DST_NAT;

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const EINVAL: c_int = 22;
const EPROTO: c_int = 71;
const ENONET: c_int = 64;
const ENOENT: c_int = 2;
const EAFNOSUPPORT: c_int = 97;

type SockLenT = c_uint;
type U16 = u16;

#[repr(C)]
struct InAddr {
    s_addr: u32,
}

#[repr(C)]
struct SockAddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct SockAddrIn {
    sin_family: u16,
    sin_port: u16,
    sin_addr: InAddr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct BpfTestRunOpts {
    data_in: *mut c_void,
    data_size_in: u32,
    repeat: u32,
}

#[repr(C)]
struct BpfObjectOpenOpts {
    kernel_log_buf: *mut c_char,
    kernel_log_size: usize,
    kernel_log_level: c_uint,
}

#[repr(C)]
struct TestBpfNfBss {
    saddr: u32,
    sport: u16,
    daddr: u32,
    dport: u16,
    test_einval_reserved: c_int,
    test_einval_reserved_new: c_int,
    test_einval_netns_id: c_int,
    test_einval_len_opts: c_int,
    test_einval_len_opts_small_lookup: c_int,
    test_einval_len_opts_small_alloc: c_int,
    test_eproto_l4proto: c_int,
    test_enonet_netns_id: c_int,
    test_enoent_lookup: c_int,
    test_eafnosupport: c_int,
    test_delta_timeout: c_int,
    test_insert_lookup_mark: c_int,
    test_status: c_uint,
    test_exist_lookup_mark: c_int,
    test_ct_zone_dir_enoent_lookup: c_int,
    test_ct_zone_id_enoent_lookup: c_int,
}

#[repr(C)]
struct TestBpfNfData {
    test_alloc_entry: c_int,
    test_insert_entry: c_int,
    test_succ_lookup: c_int,
    test_exist_lookup: c_int,
    test_snat_addr: c_int,
    test_dnat_addr: c_int,
    test_ct_zone_id_alloc_entry: c_int,
    test_ct_zone_id_insert_entry: c_int,
    test_ct_zone_id_succ_lookup: c_int,
}

#[repr(C)]
struct TestBpfNfProgs {
    nf_xdp_ct_test: *mut BpfProgram,
    nf_skb_ct_test: *mut BpfProgram,
}

#[repr(C)]
struct TestBpfNf {
    bss: *mut TestBpfNfBss,
    data: *mut TestBpfNfData,
    progs: TestBpfNfProgs,
}

#[repr(C)]
struct TestBpfNfFail {
    obj: *mut BpfObject,
}

enum BpfProgram {}
enum BpfObject {}

unsafe extern "C" {
    static mut stdout: *mut c_void;
    static mut stderr: *mut c_void;
    static mut pkt_v4: c_void;

    static IPS_CONFIRMED: c_uint;
    static IPS_SEEN_REPLY: c_uint;
    static IPS_SRC_NAT_DONE: c_uint;
    static IPS_DST_NAT_DONE: c_uint;
    static IPS_SRC_NAT: c_uint;
    static IPS_DST_NAT: c_uint;

    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut SockAddr, len: *mut SockLenT) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_LE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn SYS_NOFAIL(cmd: *const c_char) -> bool;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn connect_fd_to_fd(fd: c_int, srv_fd: c_int, timeout_ms: c_int) -> c_int;
    fn start_server(
        family: c_int,
        ty: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn get_socket_local_port(fd: c_int) -> c_int;

    fn test_bpf_nf__open_and_load() -> *mut TestBpfNf;
    fn test_bpf_nf__destroy(skel: *mut TestBpfNf);
    fn bpf_program__fd(prog: *mut BpfProgram) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut BpfTestRunOpts) -> c_int;

    fn test_bpf_nf_fail__open_opts(opts: *mut BpfObjectOpenOpts) -> *mut TestBpfNfFail;
    fn test_bpf_nf_fail__load(skel: *mut TestBpfNfFail) -> c_int;
    fn test_bpf_nf_fail__destroy(skel: *mut TestBpfNfFail);
    fn bpf_object__find_program_by_name(
        obj: *mut BpfObject,
        name: *const c_char,
    ) -> *mut BpfProgram;
    fn bpf_program__set_autoload(prog: *mut BpfProgram, autoload: bool);
}

unsafe fn connect_to_server(srv_fd: c_int) -> c_int {
    let mut fd: c_int = -1;

    fd = socket(AF_INET, SOCK_STREAM, 0);
    if !ASSERT_GE(fd, 0, c"socket".as_ptr()) {
        return fd;
    }

    if !ASSERT_EQ(
        connect_fd_to_fd(fd, srv_fd, TIMEOUT_MS),
        0,
        c"connect_fd_to_fd".as_ptr(),
    ) {
        close(fd);
        fd = -1;
    }

    fd
}

unsafe fn test_bpf_nf_ct(mode: c_int) {
    let iptables = c"iptables-legacy -t raw %s PREROUTING -j CONNMARK --set-mark 42/0".as_ptr();
    let mut srv_fd: c_int = -1;
    let mut client_fd: c_int = -1;
    let mut srv_client_fd: c_int = -1;
    let mut peer_addr: SockAddrIn = mem::zeroed();
    let skel: *mut TestBpfNf;
    let prog_fd: c_int;
    let mut len: SockLenT;
    let srv_port: U16;
    let mut cmd: [c_char; 128] = [0; 128];
    let mut topts = BpfTestRunOpts {
        data_in: &raw mut pkt_v4 as *mut c_void,
        data_size_in: mem::size_of_val(&pkt_v4) as u32,
        repeat: 1,
    };

    if SYS_NOFAIL(c"iptables-legacy --version".as_ptr()) {
        fprintf(
            stdout,
            c"Missing required iptables-legacy tool\n".as_ptr(),
        );
        test__skip();
        return;
    }

    skel = test_bpf_nf__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_bpf_nf__open_and_load".as_ptr()) {
        return;
    }

    /* Enable connection tracking */
    snprintf(cmd.as_mut_ptr(), cmd.len(), iptables, c"-A".as_ptr());
    if !ASSERT_OK(system(cmd.as_ptr()), cmd.as_ptr()) {
        goto_end(skel, client_fd, srv_client_fd, srv_fd, iptables, &mut cmd);
        return;
    }

    srv_fd = start_server(
        AF_INET,
        SOCK_STREAM,
        c"127.0.0.1".as_ptr(),
        0,
        TIMEOUT_MS,
    );
    if !ASSERT_GE(srv_fd, 0, c"start_server".as_ptr()) {
        goto_end(skel, client_fd, srv_client_fd, srv_fd, iptables, &mut cmd);
        return;
    }

    srv_port = get_socket_local_port(srv_fd) as U16;
    if !ASSERT_GE(srv_port as c_int, 0, c"get_sock_local_port".as_ptr()) {
        goto_end(skel, client_fd, srv_client_fd, srv_fd, iptables, &mut cmd);
        return;
    }

    client_fd = connect_to_server(srv_fd);
    if !ASSERT_GE(client_fd, 0, c"connect_to_server".as_ptr()) {
        goto_end(skel, client_fd, srv_client_fd, srv_fd, iptables, &mut cmd);
        return;
    }

    len = mem::size_of::<SockAddrIn>() as SockLenT;
    srv_client_fd = accept(
        srv_fd,
        &mut peer_addr as *mut SockAddrIn as *mut SockAddr,
        &mut len,
    );
    if !ASSERT_GE(srv_client_fd, 0, c"accept".as_ptr()) {
        goto_end(skel, client_fd, srv_client_fd, srv_fd, iptables, &mut cmd);
        return;
    }
    if !ASSERT_EQ(
        len as c_int,
        mem::size_of::<SockAddrIn>() as c_int,
        c"sockaddr len".as_ptr(),
    ) {
        goto_end(skel, client_fd, srv_client_fd, srv_fd, iptables, &mut cmd);
        return;
    }

    (*(*skel).bss).saddr = peer_addr.sin_addr.s_addr;
    (*(*skel).bss).sport = peer_addr.sin_port;
    (*(*skel).bss).daddr = peer_addr.sin_addr.s_addr;
    (*(*skel).bss).dport = srv_port;

    if mode == TEST_XDP {
        prog_fd = bpf_program__fd((*skel).progs.nf_xdp_ct_test);
    } else {
        prog_fd = bpf_program__fd((*skel).progs.nf_skb_ct_test);
    }

    let err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, c"bpf_prog_test_run".as_ptr()) {
        goto_end(skel, client_fd, srv_client_fd, srv_fd, iptables, &mut cmd);
        return;
    }

    ASSERT_EQ((*(*skel).bss).test_einval_reserved, -EINVAL, c"Test EINVAL for reserved not set to 0".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_einval_reserved_new, -EINVAL, c"Test EINVAL for reserved in new struct not set to 0".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_einval_netns_id, -EINVAL, c"Test EINVAL for netns_id < -1".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_einval_len_opts, -EINVAL, c"Test EINVAL for len__opts != NF_BPF_CT_OPTS_SZ".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_einval_len_opts_small_lookup, CT_OPTS_ERROR_GUARD, c"Test no error write for lookup opts__sz before error field".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_einval_len_opts_small_alloc, CT_OPTS_ERROR_GUARD, c"Test no error write for alloc opts__sz before error field".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_eproto_l4proto, -EPROTO, c"Test EPROTO for l4proto != TCP or UDP".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_enonet_netns_id, -ENONET, c"Test ENONET for bad but valid netns_id".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_enoent_lookup, -ENOENT, c"Test ENOENT for failed lookup".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_eafnosupport, -EAFNOSUPPORT, c"Test EAFNOSUPPORT for invalid len__tuple".as_ptr());
    ASSERT_EQ((*(*skel).data).test_alloc_entry, 0, c"Test for alloc new entry".as_ptr());
    ASSERT_EQ((*(*skel).data).test_insert_entry, 0, c"Test for insert new entry".as_ptr());
    ASSERT_EQ((*(*skel).data).test_succ_lookup, 0, c"Test for successful lookup".as_ptr());
    /* allow some tolerance for test_delta_timeout value to avoid races. */
    ASSERT_GT((*(*skel).bss).test_delta_timeout, 8, c"Test for min ct timeout update".as_ptr());
    ASSERT_LE((*(*skel).bss).test_delta_timeout, 10, c"Test for max ct timeout update".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_insert_lookup_mark, 77, c"Test for insert and lookup mark value".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_status as c_int, IPS_STATUS_MASK as c_int, c"Test for ct status update ".as_ptr());
    ASSERT_EQ((*(*skel).data).test_exist_lookup, 0, c"Test existing connection lookup".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_exist_lookup_mark, 43, c"Test existing connection lookup ctmark".as_ptr());
    ASSERT_EQ((*(*skel).data).test_snat_addr, 0, c"Test for source natting".as_ptr());
    ASSERT_EQ((*(*skel).data).test_dnat_addr, 0, c"Test for destination natting".as_ptr());
    ASSERT_EQ((*(*skel).data).test_ct_zone_id_alloc_entry, 0, c"Test for alloc new entry in specified ct zone".as_ptr());
    ASSERT_EQ((*(*skel).data).test_ct_zone_id_insert_entry, 0, c"Test for insert new entry in specified ct zone".as_ptr());
    ASSERT_EQ((*(*skel).data).test_ct_zone_id_succ_lookup, 0, c"Test for successful lookup in specified ct_zone".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_ct_zone_dir_enoent_lookup, -ENOENT, c"Test ENOENT for lookup with wrong ct zone dir".as_ptr());
    ASSERT_EQ((*(*skel).bss).test_ct_zone_id_enoent_lookup, -ENOENT, c"Test ENOENT for lookup in wrong ct zone".as_ptr());

    goto_end(skel, client_fd, srv_client_fd, srv_fd, iptables, &mut cmd);
}

unsafe fn goto_end(
    skel: *mut TestBpfNf,
    client_fd: c_int,
    srv_client_fd: c_int,
    srv_fd: c_int,
    iptables: *const c_char,
    cmd: &mut [c_char; 128],
) {
    if client_fd != -1 {
        close(client_fd);
    }
    if srv_client_fd != -1 {
        close(srv_client_fd);
    }
    if srv_fd != -1 {
        close(srv_fd);
    }

    snprintf(cmd.as_mut_ptr(), cmd.len(), iptables, c"-D".as_ptr());
    system(cmd.as_ptr());
    test_bpf_nf__destroy(skel);
}

unsafe fn test_bpf_nf_ct_fail(prog_name: *const c_char, err_msg: *const c_char) {
    let mut opts = BpfObjectOpenOpts {
        kernel_log_buf: LOG_BUF.as_mut_ptr(),
        kernel_log_size: mem::size_of_val(&LOG_BUF),
        kernel_log_level: 1,
    };
    let skel: *mut TestBpfNfFail;
    let prog: *mut BpfProgram;
    let ret: c_int;

    skel = test_bpf_nf_fail__open_opts(&mut opts);
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_bpf_nf_fail__open".as_ptr()) {
        return;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
    if !ASSERT_OK_PTR(prog as *const c_void, c"bpf_object__find_program_by_name".as_ptr()) {
        test_bpf_nf_fail__destroy(skel);
        return;
    }

    bpf_program__set_autoload(prog, true);

    ret = test_bpf_nf_fail__load(skel);
    if !ASSERT_ERR(ret, c"test_bpf_nf_fail__load must fail".as_ptr()) {
        test_bpf_nf_fail__destroy(skel);
        return;
    }

    if !ASSERT_OK_PTR(
        strstr(LOG_BUF.as_ptr(), err_msg) as *const c_void,
        c"expected error message".as_ptr(),
    ) {
        fprintf(stderr, c"Expected: %s\n".as_ptr(), err_msg);
        fprintf(stderr, c"Verifier: %s\n".as_ptr(), LOG_BUF.as_ptr());
    }

    test_bpf_nf_fail__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_bpf_nf() {
    let mut i: usize;

    if test__start_subtest(c"xdp-ct".as_ptr()) {
        test_bpf_nf_ct(TEST_XDP);
    }
    if test__start_subtest(c"tc-bpf-ct".as_ptr()) {
        test_bpf_nf_ct(TEST_TC_BPF);
    }
    i = 0;
    while i < TEST_BPF_NF_FAIL_TESTS.len() {
        if test__start_subtest(TEST_BPF_NF_FAIL_TESTS[i].prog_name) {
            test_bpf_nf_ct_fail(
                TEST_BPF_NF_FAIL_TESTS[i].prog_name,
                TEST_BPF_NF_FAIL_TESTS[i].err_msg,
            );
        }
        i += 1;
    }
}
