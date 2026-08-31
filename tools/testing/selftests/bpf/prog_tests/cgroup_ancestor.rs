// SPDX-License-Identifier: GPL-2.0

// Translated from:
// test_progs.h
// network_helpers.h
// cgroup_helpers.h
// cgroup_ancestor.skel.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const CGROUP_PATH: &[u8] = b"/skb_cgroup_test\0";
const TEST_NS: &[u8] = b"cgroup_ancestor_ns\0";
const NUM_CGROUP_LEVELS: usize = 4;
const WAIT_AUTO_IP_MAX_ATTEMPT: c_int = 10;
const DST_ADDR: &[u8] = b"::1\0";
const DST_PORT: u16 = 1234;
const MAX_ASSERT_NAME: usize = 32;

type __u32 = u32;
type __u64 = u64;
type socklen_t = u32;

const AF_INET6: c_int = 10;
const SOCK_DGRAM: c_int = 2;
const BPF_TC_EGRESS: c_int = 1;

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
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
struct bpf_tc_hook {
    sz: usize,
    ifindex: c_int,
    attach_point: c_int,
}

#[repr(C)]
struct bpf_tc_opts {
    sz: usize,
    prog_fd: c_int,
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
struct cgroup_ancestor_progs {
    log_cgroup_id: *mut bpf_program,
}

#[repr(C)]
struct cgroup_ancestor_bss {
    dport: u16,
    cgroup_ids: [__u64; NUM_CGROUP_LEVELS],
}

#[repr(C)]
struct cgroup_ancestor {
    progs: cgroup_ancestor_progs,
    bss: *mut cgroup_ancestor_bss,
}

#[repr(C)]
struct test_data {
    skel: *mut cgroup_ancestor,
    qdisc: bpf_tc_hook,
    tc_attach: bpf_tc_opts,
    ns: *mut nstoken,
}

unsafe extern "C" {
    fn htons(hostshort: u16) -> u16;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn connect(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn sendto(
        fd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        addr: *const sockaddr,
        addr_len: socklen_t,
    ) -> isize;
    fn close(fd: c_int) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_tc_detach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;

    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn get_cgroup_id(path: *const c_char) -> __u64;
    fn cgroup_setup_and_join(path: *const c_char) -> c_int;
    fn cleanup_cgroup_environment();

    fn cgroup_ancestor__open_and_load() -> *mut cgroup_ancestor;
    fn cgroup_ancestor__destroy(skel: *mut cgroup_ancestor);

    fn ASSERT_EQ<T: Copy>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_NEQ<T: Copy>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;

    fn SYS(format: *const c_char, ...) -> c_int;
    fn SYS_NOFAIL(format: *const c_char, ...) -> c_int;
}

unsafe fn send_datagram() -> c_int {
    let buf = *b"some random test data\0";
    let mut addr = sockaddr_in6 {
        sin6_family: AF_INET6 as u16,
        sin6_port: htons(DST_PORT),
        sin6_flowinfo: 0,
        sin6_addr: in6_addr { s6_addr: [0; 16] },
        sin6_scope_id: 0,
    };
    let sock: c_int;
    let n: isize;

    if !ASSERT_EQ(
        inet_pton(
            AF_INET6,
            DST_ADDR.as_ptr() as *const c_char,
            &mut addr.sin6_addr as *mut _ as *mut c_void,
        ),
        1,
        b"inet_pton\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    sock = socket(AF_INET6, SOCK_DGRAM, 0);
    if !ASSERT_OK_FD(sock, b"create socket\0".as_ptr() as *const c_char) {
        return sock;
    }

    if !ASSERT_OK(
        connect(
            sock,
            &addr as *const _ as *const sockaddr,
            mem::size_of_val(&addr) as socklen_t,
        ),
        b"connect\0".as_ptr() as *const c_char,
    ) {
        close(sock);
        return -1;
    }

    n = sendto(
        sock,
        buf.as_ptr() as *const c_void,
        mem::size_of_val(&buf),
        0,
        &addr as *const _ as *const sockaddr,
        mem::size_of_val(&addr) as socklen_t,
    );
    close(sock);
    if ASSERT_EQ(
        n,
        mem::size_of_val(&buf) as isize,
        b"send data\0".as_ptr() as *const c_char,
    ) {
        0
    } else {
        -1
    }
}

unsafe fn setup_network(t: *mut test_data) -> c_int {
    if SYS(
        b"ip netns add %s\0".as_ptr() as *const c_char,
        TEST_NS.as_ptr() as *const c_char,
    ) != 0
    {
        return 1;
    }
    (*t).ns = open_netns(TEST_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR((*t).ns, b"open netns\0".as_ptr() as *const c_char) {
        SYS_NOFAIL(
            b"ip netns del %s\0".as_ptr() as *const c_char,
            TEST_NS.as_ptr() as *const c_char,
        );
        return 1;
    }

    if SYS(b"ip link set lo up\0".as_ptr() as *const c_char) != 0 {
        close_netns((*t).ns);
        SYS_NOFAIL(
            b"ip netns del %s\0".as_ptr() as *const c_char,
            TEST_NS.as_ptr() as *const c_char,
        );
        return 1;
    }

    memset(
        &mut (*t).qdisc as *mut _ as *mut c_void,
        0,
        mem::size_of_val(&(*t).qdisc),
    );
    (*t).qdisc.sz = mem::size_of_val(&(*t).qdisc);
    (*t).qdisc.attach_point = BPF_TC_EGRESS;
    (*t).qdisc.ifindex = if_nametoindex(b"lo\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_NEQ((*t).qdisc.ifindex, 0, b"if_nametoindex\0".as_ptr() as *const c_char) {
        close_netns((*t).ns);
        SYS_NOFAIL(
            b"ip netns del %s\0".as_ptr() as *const c_char,
            TEST_NS.as_ptr() as *const c_char,
        );
        return 1;
    }
    if !ASSERT_OK(
        bpf_tc_hook_create(&mut (*t).qdisc),
        b"qdisc add\0".as_ptr() as *const c_char,
    ) {
        close_netns((*t).ns);
        SYS_NOFAIL(
            b"ip netns del %s\0".as_ptr() as *const c_char,
            TEST_NS.as_ptr() as *const c_char,
        );
        return 1;
    }

    memset(
        &mut (*t).tc_attach as *mut _ as *mut c_void,
        0,
        mem::size_of_val(&(*t).tc_attach),
    );
    (*t).tc_attach.sz = mem::size_of_val(&(*t).tc_attach);
    (*t).tc_attach.prog_fd = bpf_program__fd((*(*t).skel).progs.log_cgroup_id);
    if !ASSERT_OK(
        bpf_tc_attach(&mut (*t).qdisc, &mut (*t).tc_attach),
        b"filter add\0".as_ptr() as *const c_char,
    ) {
        bpf_tc_hook_destroy(&mut (*t).qdisc);
        close_netns((*t).ns);
        SYS_NOFAIL(
            b"ip netns del %s\0".as_ptr() as *const c_char,
            TEST_NS.as_ptr() as *const c_char,
        );
        return 1;
    }

    0
}

unsafe fn cleanup_network(t: *mut test_data) {
    bpf_tc_detach(&mut (*t).qdisc, &mut (*t).tc_attach);
    bpf_tc_hook_destroy(&mut (*t).qdisc);
    close_netns((*t).ns);
    SYS_NOFAIL(
        b"ip netns del %s\0".as_ptr() as *const c_char,
        TEST_NS.as_ptr() as *const c_char,
    );
}

unsafe fn check_ancestors_ids(t: *mut test_data) {
    let mut expected_ids: [__u64; NUM_CGROUP_LEVELS] = [0; NUM_CGROUP_LEVELS];
    let mut assert_name: [c_char; MAX_ASSERT_NAME] = [0; MAX_ASSERT_NAME];
    let mut level: __u32;

    expected_ids[0] = get_cgroup_id(b"/..\0".as_ptr() as *const c_char); /* root cgroup */
    expected_ids[1] = get_cgroup_id(b"\0".as_ptr() as *const c_char);
    expected_ids[2] = get_cgroup_id(CGROUP_PATH.as_ptr() as *const c_char);
    expected_ids[3] = 0; /* non-existent cgroup */

    level = 0;
    while level < NUM_CGROUP_LEVELS as __u32 {
        snprintf(
            assert_name.as_mut_ptr(),
            MAX_ASSERT_NAME,
            b"ancestor id at level %d\0".as_ptr() as *const c_char,
            level,
        );
        ASSERT_EQ(
            (*(*(*t).skel).bss).cgroup_ids[level as usize],
            expected_ids[level as usize],
            assert_name.as_ptr(),
        );
        level += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_cgroup_ancestor() {
    let mut t: test_data = mem::zeroed();
    let cgroup_fd: c_int;

    t.skel = cgroup_ancestor__open_and_load();
    if !ASSERT_OK_PTR(t.skel, b"open and load\0".as_ptr() as *const c_char) {
        return;
    }

    (*(*t.skel).bss).dport = htons(DST_PORT);
    cgroup_fd = cgroup_setup_and_join(CGROUP_PATH.as_ptr() as *const c_char);
    if cgroup_fd < 0 {
        cgroup_ancestor__destroy(t.skel);
        return;
    }

    if setup_network(&mut t) != 0 {
        close(cgroup_fd);
        cleanup_cgroup_environment();
        cgroup_ancestor__destroy(t.skel);
        return;
    }

    if send_datagram() == 0 {
        check_ancestors_ids(&mut t);
    }

    cleanup_network(&mut t);
    close(cgroup_fd);
    cleanup_cgroup_environment();
    cgroup_ancestor__destroy(t.skel);
}
