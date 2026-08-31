// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2024 Meta

// Translated from C. External symbols come from the BPF selftest harness,
// network helpers, libc, and the generated sock_iter_batch skeleton.

use core::ffi::{c_char, c_int, c_short, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type __u16 = u16;
type __u64 = u64;
type socklen_t = c_uint;

const TEST_NS: &[u8] = b"sock_iter_batch_netns\0";
const TEST_CHILD_NS: &[u8] = b"sock_iter_batch_child_netns\0";

static init_batch_size: c_int = 16;
static nr_soreuse: c_int = 4;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_COOKIE: c_int = 57;
const POLLIN: c_short = 0x0001;
const TCP_LISTEN: c_int = 10;

#[repr(C, packed)]
struct iter_out {
    idx: c_int,
    cookie: __u64,
}

#[repr(C)]
struct sock_count {
    cookie: __u64,
    count: c_int,
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
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
struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
struct sock_iter_batch_rodata {
    destroy_cookie: __u64,
    ports: [__u16; 2],
    sf: c_int,
    ss: c_int,
}

#[repr(C)]
struct sock_iter_batch_bss {
    bucket: [c_int; 2],
}

#[repr(C)]
struct sock_iter_batch_progs {
    iter_tcp_destroy: *mut bpf_program,
    iter_tcp_soreuse: *mut bpf_program,
    iter_udp_soreuse: *mut bpf_program,
}

#[repr(C)]
struct sock_iter_batch {
    rodata: *mut sock_iter_batch_rodata,
    bss: *mut sock_iter_batch_bss,
    progs: sock_iter_batch_progs,
}

type TestFn = unsafe fn(
    family: c_int,
    sock_type: c_int,
    addr: *const c_char,
    port: __u16,
    socks: *mut c_int,
    socks_len: c_int,
    established_socks: *mut c_int,
    established_socks_len: c_int,
    counts: *mut sock_count,
    counts_len: c_int,
    link: *mut bpf_link,
    iter_fd: c_int,
);

#[repr(C)]
struct test_case {
    test: TestFn,
    description: *const c_char,
    ehash_buckets: c_int,
    connections: c_int,
    init_socks: c_int,
    max_socks: c_int,
    sock_type: c_int,
    family: c_int,
}

extern "C" {
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn getsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> c_int;
    fn poll(fds: *mut pollfd, nfds: usize, timeout: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut c_void, addrlen: *mut socklen_t) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn ntohs(netshort: __u16) -> __u16;

    fn ASSERT_EQ(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_TRUE(cond: bool, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn SYS(label: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn SYS_NOFAIL(fmt: *const c_char, ...) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn start_reuseport_server(
        family: c_int,
        sock_type: c_int,
        addr: *const c_char,
        port: __u16,
        timeout_ms: c_int,
        nr_listens: c_int,
    ) -> *mut c_int;
    fn connect_to_addr_str(
        family: c_int,
        sock_type: c_int,
        addr: *const c_char,
        port: __u16,
        opts: *mut c_void,
    ) -> c_int;
    fn free_fds(fds: *mut c_int, nr_close_fds: c_int);
    fn get_socket_local_port(fd: c_int) -> c_int;

    fn sock_iter_batch__open() -> *mut sock_iter_batch;
    fn sock_iter_batch__load(obj: *mut sock_iter_batch) -> c_int;
    fn sock_iter_batch__destroy(obj: *mut sock_iter_batch);
    fn bpf_program__attach_iter(prog: *mut bpf_program, opts: *mut c_void) -> *mut bpf_link;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_iter_create(link_fd: c_int) -> c_int;
}

unsafe fn insert(cookie: __u64, counts: *mut sock_count, counts_len: c_int) -> c_int {
    let mut insert = -1;
    let mut i = 0;

    while i < counts_len {
        if (*counts.add(i as usize)).cookie == 0 {
            insert = i;
        } else if (*counts.add(i as usize)).cookie == cookie {
            insert = i;
            break;
        }
        i += 1;
    }
    if insert < 0 {
        return insert;
    }

    (*counts.add(insert as usize)).cookie = cookie;
    (*counts.add(insert as usize)).count += 1;

    (*counts.add(insert as usize)).count
}

unsafe fn read_n(iter_fd: c_int, n: c_int, counts: *mut sock_count, counts_len: c_int) -> c_int {
    let mut out: iter_out = core::mem::zeroed();
    let mut nread: isize = 1;
    let mut i = 0;

    while nread > 0 && (n < 0 || i < n) {
        nread = read(iter_fd, &mut out as *mut _ as *mut c_void, size_of::<iter_out>());
        if nread == 0 || !ASSERT_EQ(nread, size_of::<iter_out>() as isize, c"nread".as_ptr()) {
            break;
        }
        ASSERT_GE(insert(out.cookie, counts, counts_len) as isize, 0, c"insert".as_ptr());
        i += 1;
    }

    ASSERT_TRUE(n < 0 || i == n, c"n < 0 || i == n".as_ptr());

    i
}

unsafe fn socket_cookie(fd: c_int) -> __u64 {
    let mut cookie: __u64 = 0;
    let mut cookie_len: socklen_t = size_of::<__u64>() as socklen_t;

    if !ASSERT_OK(
        getsockopt(
            fd,
            SOL_SOCKET,
            SO_COOKIE,
            &mut cookie as *mut _ as *mut c_void,
            &mut cookie_len,
        ),
        c"getsockopt(SO_COOKIE)".as_ptr(),
    ) {
        return 0;
    }
    cookie
}

unsafe fn was_seen(fd: c_int, counts: *mut sock_count, counts_len: c_int) -> bool {
    let cookie = socket_cookie(fd);
    let mut i = 0;

    while cookie != 0 && i < counts_len {
        if cookie == (*counts.add(i as usize)).cookie {
            return true;
        }
        i += 1;
    }

    false
}

unsafe fn get_seen_socket(fds: *mut c_int, counts: *mut sock_count, n: c_int) -> c_int {
    let mut i = 0;

    while i < n {
        if was_seen(*fds.add(i as usize), counts, n) {
            return i;
        }
        i += 1;
    }
    -1
}

unsafe fn get_nth_socket(fds: *mut c_int, fds_len: c_int, link: *mut bpf_link, mut n: c_int) -> c_int {
    let mut i;
    let mut nread;
    let iter_fd;
    let mut nth_sock_idx = -1;
    let mut out: iter_out = core::mem::zeroed();

    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if !ASSERT_OK_FD(iter_fd, c"bpf_iter_create".as_ptr()) {
        return -1;
    }

    while n >= 0 {
        nread = read(iter_fd, &mut out as *mut _ as *mut c_void, size_of::<iter_out>());
        if nread == 0 || !ASSERT_GE(nread, 1, c"nread".as_ptr()) {
            close(iter_fd);
            return nth_sock_idx;
        }
        n -= 1;
    }

    i = 0;
    while i < fds_len && nth_sock_idx < 0 {
        if *fds.add(i as usize) >= 0 && socket_cookie(*fds.add(i as usize)) == out.cookie {
            nth_sock_idx = i;
        }
        i += 1;
    }
    close(iter_fd);
    nth_sock_idx
}

unsafe fn destroy(fd: c_int) {
    let mut skel: *mut sock_iter_batch = ptr::null_mut();
    let cookie = socket_cookie(fd);
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut iter_fd = -1;
    let mut nread;
    let mut out: __u64 = 0;

    skel = sock_iter_batch__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"sock_iter_batch__open".as_ptr()) {
        close(fd);
        return;
    }

    (*(*skel).rodata).destroy_cookie = cookie;

    if !ASSERT_OK(sock_iter_batch__load(skel), c"sock_iter_batch__load".as_ptr()) {
        bpf_link__destroy(link);
        sock_iter_batch__destroy(skel);
        close(fd);
        return;
    }

    link = bpf_program__attach_iter((*skel).progs.iter_tcp_destroy, ptr::null_mut());
    if !ASSERT_OK_PTR(link as *const c_void, c"bpf_program__attach_iter".as_ptr()) {
        bpf_link__destroy(link);
        sock_iter_batch__destroy(skel);
        close(fd);
        return;
    }

    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if ASSERT_OK_FD(iter_fd, c"bpf_iter_create".as_ptr()) {
        /* Delete matching socket. */
        nread = read(iter_fd, &mut out as *mut _ as *mut c_void, size_of::<__u64>());
        ASSERT_GE(nread, 0, c"nread".as_ptr());
        if nread != 0 {
            ASSERT_EQ(out as isize, cookie as isize, c"cookie matches".as_ptr());
        }
    }

    if iter_fd >= 0 {
        close(iter_fd);
    }
    bpf_link__destroy(link);
    sock_iter_batch__destroy(skel);
    close(fd);
}

unsafe fn get_seen_count(fd: c_int, counts: *mut sock_count, n: c_int) -> c_int {
    let cookie = socket_cookie(fd);
    let mut count = 0;
    let mut i = 0;

    while cookie != 0 && count == 0 && i < n {
        if cookie == (*counts.add(i as usize)).cookie {
            count = (*counts.add(i as usize)).count;
        }
        i += 1;
    }

    count
}

unsafe fn check_n_were_seen_once(
    fds: *mut c_int,
    fds_len: c_int,
    n: c_int,
    counts: *mut sock_count,
    counts_len: c_int,
) {
    let mut seen_once = 0;
    let mut seen_cnt;
    let mut i = 0;

    while i < fds_len {
        /* Skip any sockets that were closed or that weren't seen
         * exactly once.
         */
        if *fds.add(i as usize) >= 0 {
            seen_cnt = get_seen_count(*fds.add(i as usize), counts, counts_len);
            if seen_cnt != 0 && ASSERT_EQ(seen_cnt as isize, 1, c"seen_cnt".as_ptr()) {
                seen_once += 1;
            }
        }
        i += 1;
    }

    ASSERT_EQ(seen_once as isize, n as isize, c"seen_once".as_ptr());
}

unsafe fn accept_from_one(server_poll_fds: *mut pollfd, server_poll_fds_len: c_int) -> c_int {
    static poll_timeout_ms: c_int = 5000; /* 5s */
    let ret;
    let mut i;

    ret = poll(server_poll_fds, server_poll_fds_len as usize, poll_timeout_ms);
    if !ASSERT_EQ(ret as isize, 1, c"poll".as_ptr()) {
        return -1;
    }

    i = 0;
    while i < server_poll_fds_len {
        if ((*server_poll_fds.add(i as usize)).revents & POLLIN) != 0 {
            return accept((*server_poll_fds.add(i as usize)).fd, ptr::null_mut(), ptr::null_mut());
        }
        i += 1;
    }

    -1
}

unsafe fn connect_to_server(
    family: c_int,
    sock_type: c_int,
    addr: *const c_char,
    port: __u16,
    mut nr_connects: c_int,
    server_fds: *mut c_int,
    server_fds_len: c_int,
) -> *mut c_int {
    let mut server_poll_fds: *mut pollfd = ptr::null_mut();
    let mut established_socks: *mut c_int = ptr::null_mut();
    let mut i;

    server_poll_fds = calloc(server_fds_len as usize, size_of::<pollfd>()) as *mut pollfd;
    if !ASSERT_OK_PTR(server_poll_fds as *const c_void, c"server_poll_fds".as_ptr()) {
        return ptr::null_mut();
    }

    i = 0;
    while i < server_fds_len {
        (*server_poll_fds.add(i as usize)).fd = *server_fds.add(i as usize);
        (*server_poll_fds.add(i as usize)).events = POLLIN;
        i += 1;
    }

    i = 0;

    established_socks = malloc(size_of::<c_int>() * (nr_connects as usize) * 2) as *mut c_int;
    if !ASSERT_OK_PTR(established_socks as *const c_void, c"established_socks".as_ptr()) {
        free(server_poll_fds as *mut c_void);
        return ptr::null_mut();
    }

    while nr_connects != 0 {
        (*established_socks.add(i as usize)) =
            connect_to_addr_str(family, sock_type, addr, port, ptr::null_mut());
        if !ASSERT_OK_FD(*established_socks.add(i as usize), c"connect_to_addr_str".as_ptr()) {
            free_fds(established_socks, i);
            free(server_poll_fds as *mut c_void);
            return ptr::null_mut();
        }
        i += 1;
        (*established_socks.add(i as usize)) = accept_from_one(server_poll_fds, server_fds_len);
        if !ASSERT_OK_FD(*established_socks.add(i as usize), c"accept_from_one".as_ptr()) {
            free_fds(established_socks, i);
            free(server_poll_fds as *mut c_void);
            return ptr::null_mut();
        }
        i += 1;
        nr_connects -= 1;
    }

    free(server_poll_fds as *mut c_void);
    established_socks
}

unsafe fn remove_seen(
    _family: c_int,
    _sock_type: c_int,
    _addr: *const c_char,
    _port: __u16,
    socks: *mut c_int,
    socks_len: c_int,
    _established_socks: *mut c_int,
    _established_socks_len: c_int,
    counts: *mut sock_count,
    counts_len: c_int,
    _link: *mut bpf_link,
    iter_fd: c_int,
) {
    let close_idx;

    /* Iterate through the first socks_len - 1 sockets. */
    read_n(iter_fd, socks_len - 1, counts, counts_len);

    /* Make sure we saw socks_len - 1 sockets exactly once. */
    check_n_were_seen_once(socks, socks_len, socks_len - 1, counts, counts_len);

    /* Close a socket we've already seen to remove it from the bucket. */
    close_idx = get_seen_socket(socks, counts, counts_len);
    if !ASSERT_GE(close_idx as isize, 0, c"close_idx".as_ptr()) {
        return;
    }
    close(*socks.add(close_idx as usize));
    *socks.add(close_idx as usize) = -1;

    /* Iterate through the rest of the sockets. */
    read_n(iter_fd, -1, counts, counts_len);

    /* Make sure the last socket wasn't skipped and that there were no
     * repeats.
     */
    check_n_were_seen_once(socks, socks_len, socks_len - 1, counts, counts_len);
}

unsafe fn remove_seen_established(
    _family: c_int,
    _sock_type: c_int,
    _addr: *const c_char,
    _port: __u16,
    listen_socks: *mut c_int,
    listen_socks_len: c_int,
    established_socks: *mut c_int,
    established_socks_len: c_int,
    counts: *mut sock_count,
    counts_len: c_int,
    link: *mut bpf_link,
    iter_fd: c_int,
) {
    let close_idx;

    /* Iterate through all listening sockets. */
    read_n(iter_fd, listen_socks_len, counts, counts_len);
    check_n_were_seen_once(listen_socks, listen_socks_len, listen_socks_len, counts, counts_len);
    read_n(iter_fd, established_socks_len - 1, counts, counts_len);

    close_idx = get_nth_socket(established_socks, established_socks_len, link, listen_socks_len + 1);
    if !ASSERT_GE(close_idx as isize, 0, c"close_idx".as_ptr()) {
        return;
    }
    destroy(*established_socks.add(close_idx as usize));
    *established_socks.add(close_idx as usize) = -1;

    read_n(iter_fd, -1, counts, counts_len);
    check_n_were_seen_once(established_socks, established_socks_len, established_socks_len - 1, counts, counts_len);
}

unsafe fn remove_unseen(
    _family: c_int,
    _sock_type: c_int,
    _addr: *const c_char,
    _port: __u16,
    socks: *mut c_int,
    socks_len: c_int,
    _established_socks: *mut c_int,
    _established_socks_len: c_int,
    counts: *mut sock_count,
    counts_len: c_int,
    link: *mut bpf_link,
    iter_fd: c_int,
) {
    let close_idx;

    /* Iterate through the first socket. */
    read_n(iter_fd, 1, counts, counts_len);
    check_n_were_seen_once(socks, socks_len, 1, counts, counts_len);

    /* Close what would be the next socket in the bucket to exercise the
     * condition where we need to skip past the first cookie we remembered.
     */
    close_idx = get_nth_socket(socks, socks_len, link, 1);
    if !ASSERT_GE(close_idx as isize, 0, c"close_idx".as_ptr()) {
        return;
    }
    close(*socks.add(close_idx as usize));
    *socks.add(close_idx as usize) = -1;

    read_n(iter_fd, -1, counts, counts_len);
    check_n_were_seen_once(socks, socks_len, socks_len - 1, counts, counts_len);
}

unsafe fn remove_unseen_established(
    _family: c_int,
    _sock_type: c_int,
    _addr: *const c_char,
    _port: __u16,
    listen_socks: *mut c_int,
    listen_socks_len: c_int,
    established_socks: *mut c_int,
    established_socks_len: c_int,
    counts: *mut sock_count,
    counts_len: c_int,
    link: *mut bpf_link,
    iter_fd: c_int,
) {
    let close_idx;

    read_n(iter_fd, listen_socks_len, counts, counts_len);
    check_n_were_seen_once(listen_socks, listen_socks_len, listen_socks_len, counts, counts_len);
    read_n(iter_fd, 1, counts, counts_len);
    check_n_were_seen_once(established_socks, established_socks_len, 1, counts, counts_len);

    close_idx = get_nth_socket(established_socks, established_socks_len, link, listen_socks_len + 1);
    if !ASSERT_GE(close_idx as isize, 0, c"close_idx".as_ptr()) {
        return;
    }

    destroy(*established_socks.add(close_idx as usize));
    *established_socks.add(close_idx as usize) = -1;

    read_n(iter_fd, -1, counts, counts_len);
    check_n_were_seen_once(established_socks, established_socks_len, established_socks_len - 1, counts, counts_len);
}

unsafe fn remove_all(
    _family: c_int,
    _sock_type: c_int,
    _addr: *const c_char,
    _port: __u16,
    socks: *mut c_int,
    socks_len: c_int,
    _established_socks: *mut c_int,
    _established_socks_len: c_int,
    counts: *mut sock_count,
    counts_len: c_int,
    link: *mut bpf_link,
    iter_fd: c_int,
) {
    let mut close_idx;
    let mut i;

    read_n(iter_fd, 1, counts, counts_len);
    check_n_were_seen_once(socks, socks_len, 1, counts, counts_len);

    i = 0;
    while i < socks_len - 1 {
        close_idx = get_nth_socket(socks, socks_len, link, 1);
        if !ASSERT_GE(close_idx as isize, 0, c"close_idx".as_ptr()) {
            return;
        }
        close(*socks.add(close_idx as usize));
        *socks.add(close_idx as usize) = -1;
        i += 1;
    }

    ASSERT_EQ(read_n(iter_fd, -1, counts, counts_len) as isize, 0, c"read_n".as_ptr());
}

unsafe fn remove_all_established(
    _family: c_int,
    _sock_type: c_int,
    _addr: *const c_char,
    _port: __u16,
    listen_socks: *mut c_int,
    listen_socks_len: c_int,
    established_socks: *mut c_int,
    established_socks_len: c_int,
    counts: *mut sock_count,
    counts_len: c_int,
    link: *mut bpf_link,
    iter_fd: c_int,
) {
    let mut close_idx: *mut c_int = ptr::null_mut();
    let mut i;

    read_n(iter_fd, listen_socks_len, counts, counts_len);
    check_n_were_seen_once(listen_socks, listen_socks_len, listen_socks_len, counts, counts_len);
    read_n(iter_fd, 1, counts, counts_len);
    check_n_were_seen_once(established_socks, established_socks_len, 1, counts, counts_len);

    close_idx = malloc(size_of::<c_int>() * (established_socks_len - 1) as usize) as *mut c_int;
    if !ASSERT_OK_PTR(close_idx as *const c_void, c"close_idx malloc".as_ptr()) {
        return;
    }
    i = 0;
    while i < established_socks_len - 1 {
        *close_idx.add(i as usize) =
            get_nth_socket(established_socks, established_socks_len, link, listen_socks_len + i);
        if !ASSERT_GE(*close_idx.add(i as usize) as isize, 0, c"close_idx".as_ptr()) {
            return;
        }
        i += 1;
    }

    i = 0;
    while i < established_socks_len - 1 {
        destroy(*established_socks.add(*close_idx.add(i as usize) as usize));
        *established_socks.add(*close_idx.add(i as usize) as usize) = -1;
        i += 1;
    }

    ASSERT_EQ(read_n(iter_fd, -1, counts, counts_len) as isize, 0, c"read_n".as_ptr());
    free(close_idx as *mut c_void);
}

unsafe fn add_some(
    family: c_int,
    sock_type: c_int,
    addr: *const c_char,
    port: __u16,
    socks: *mut c_int,
    socks_len: c_int,
    _established_socks: *mut c_int,
    _established_socks_len: c_int,
    counts: *mut sock_count,
    counts_len: c_int,
    _link: *mut bpf_link,
    iter_fd: c_int,
) {
    let mut new_socks: *mut c_int = ptr::null_mut();

    read_n(iter_fd, socks_len - 1, counts, counts_len);
    check_n_were_seen_once(socks, socks_len, socks_len - 1, counts, counts_len);

    new_socks = start_reuseport_server(family, sock_type, addr, port, 0, socks_len);
    if ASSERT_OK_PTR(new_socks as *const c_void, c"start_reuseport_server".as_ptr()) {
        read_n(iter_fd, -1, counts, counts_len);
        check_n_were_seen_once(socks, socks_len, socks_len, counts, counts_len);
    }
    free_fds(new_socks, socks_len);
}

unsafe fn add_some_established(
    family: c_int,
    sock_type: c_int,
    addr: *const c_char,
    port: __u16,
    listen_socks: *mut c_int,
    listen_socks_len: c_int,
    established_socks: *mut c_int,
    established_socks_len: c_int,
    counts: *mut sock_count,
    counts_len: c_int,
    _link: *mut bpf_link,
    iter_fd: c_int,
) {
    let mut new_socks: *mut c_int = ptr::null_mut();

    read_n(iter_fd, listen_socks_len, counts, counts_len);
    check_n_were_seen_once(listen_socks, listen_socks_len, listen_socks_len, counts, counts_len);
    read_n(iter_fd, established_socks_len - 1, counts, counts_len);
    check_n_were_seen_once(established_socks, established_socks_len, established_socks_len - 1, counts, counts_len);

    new_socks = connect_to_server(
        family,
        sock_type,
        addr,
        port,
        established_socks_len / 2,
        listen_socks,
        listen_socks_len,
    );
    if ASSERT_OK_PTR(new_socks as *const c_void, c"connect_to_server".as_ptr()) {
        read_n(iter_fd, -1, counts, counts_len);
        check_n_were_seen_once(listen_socks, listen_socks_len, listen_socks_len, counts, counts_len);
        check_n_were_seen_once(established_socks, established_socks_len, established_socks_len, counts, counts_len);
    }
    free_fds(new_socks, established_socks_len);
}

unsafe fn force_realloc(
    family: c_int,
    sock_type: c_int,
    addr: *const c_char,
    port: __u16,
    socks: *mut c_int,
    socks_len: c_int,
    _established_socks: *mut c_int,
    _established_socks_len: c_int,
    counts: *mut sock_count,
    counts_len: c_int,
    _link: *mut bpf_link,
    iter_fd: c_int,
) {
    let mut new_socks: *mut c_int = ptr::null_mut();

    read_n(iter_fd, 1, counts, counts_len);

    new_socks = start_reuseport_server(family, sock_type, addr, port, 0, socks_len);
    if ASSERT_OK_PTR(new_socks as *const c_void, c"start_reuseport_server".as_ptr()) {
        read_n(iter_fd, -1, counts, counts_len);
        check_n_were_seen_once(socks, socks_len, socks_len, counts, counts_len);
    }
    free_fds(new_socks, socks_len);
}

unsafe fn force_realloc_established(
    _family: c_int,
    _sock_type: c_int,
    _addr: *const c_char,
    _port: __u16,
    listen_socks: *mut c_int,
    listen_socks_len: c_int,
    established_socks: *mut c_int,
    established_socks_len: c_int,
    counts: *mut sock_count,
    counts_len: c_int,
    _link: *mut bpf_link,
    iter_fd: c_int,
) {
    /* Iterate through all sockets to trigger a realloc. */
    read_n(iter_fd, -1, counts, counts_len);

    /* Make sure each socket was seen exactly once. */
    check_n_were_seen_once(listen_socks, listen_socks_len, listen_socks_len, counts, counts_len);
    check_n_were_seen_once(established_socks, established_socks_len, established_socks_len, counts, counts_len);
}

static mut resume_tests: [test_case; 15] = [
    test_case { description: c"udp: resume after removing a seen socket".as_ptr(), init_socks: 4, max_socks: 4, sock_type: SOCK_DGRAM, family: AF_INET6, test: remove_seen, ehash_buckets: 0, connections: 0 },
    test_case { description: c"udp: resume after removing one unseen socket".as_ptr(), init_socks: 4, max_socks: 4, sock_type: SOCK_DGRAM, family: AF_INET6, test: remove_unseen, ehash_buckets: 0, connections: 0 },
    test_case { description: c"udp: resume after removing all unseen sockets".as_ptr(), init_socks: 4, max_socks: 4, sock_type: SOCK_DGRAM, family: AF_INET6, test: remove_all, ehash_buckets: 0, connections: 0 },
    test_case { description: c"udp: resume after adding a few sockets".as_ptr(), init_socks: 4, max_socks: 4, sock_type: SOCK_DGRAM, family: AF_INET, test: add_some, ehash_buckets: 0, connections: 0 },
    test_case { description: c"udp: force a realloc to occur".as_ptr(), init_socks: 16, max_socks: 32, sock_type: SOCK_DGRAM, family: AF_INET6, test: force_realloc, ehash_buckets: 0, connections: 0 },
    test_case { description: c"tcp: resume after removing a seen socket (listening)".as_ptr(), init_socks: 4, max_socks: 4, sock_type: SOCK_STREAM, family: AF_INET6, test: remove_seen, ehash_buckets: 0, connections: 0 },
    test_case { description: c"tcp: resume after removing one unseen socket (listening)".as_ptr(), init_socks: 4, max_socks: 4, sock_type: SOCK_STREAM, family: AF_INET6, test: remove_unseen, ehash_buckets: 0, connections: 0 },
    test_case { description: c"tcp: resume after removing all unseen sockets (listening)".as_ptr(), init_socks: 4, max_socks: 4, sock_type: SOCK_STREAM, family: AF_INET6, test: remove_all, ehash_buckets: 0, connections: 0 },
    test_case { description: c"tcp: resume after adding a few sockets (listening)".as_ptr(), init_socks: 4, max_socks: 4, sock_type: SOCK_STREAM, family: AF_INET, test: add_some, ehash_buckets: 0, connections: 0 },
    test_case { description: c"tcp: force a realloc to occur (listening)".as_ptr(), init_socks: 16, max_socks: 32, sock_type: SOCK_STREAM, family: AF_INET6, test: force_realloc, ehash_buckets: 0, connections: 0 },
    test_case { description: c"tcp: resume after removing a seen socket (established)".as_ptr(), ehash_buckets: 1, connections: 4, init_socks: 4, max_socks: 12, sock_type: SOCK_STREAM, family: AF_INET6, test: remove_seen_established },
    test_case { description: c"tcp: resume after removing one unseen socket (established)".as_ptr(), ehash_buckets: 1, connections: 4, init_socks: 4, max_socks: 12, sock_type: SOCK_STREAM, family: AF_INET6, test: remove_unseen_established },
    test_case { description: c"tcp: resume after removing all unseen sockets (established)".as_ptr(), ehash_buckets: 1, connections: 4, init_socks: 4, max_socks: 12, sock_type: SOCK_STREAM, family: AF_INET6, test: remove_all_established },
    test_case { description: c"tcp: resume after adding a few sockets (established)".as_ptr(), ehash_buckets: 1, connections: 4, init_socks: 4, max_socks: 12, sock_type: SOCK_STREAM, family: AF_INET6, test: add_some_established },
    test_case { description: c"tcp: force a realloc to occur (established)".as_ptr(), ehash_buckets: 1, connections: 16, init_socks: 4, max_socks: 36, sock_type: SOCK_STREAM, family: AF_INET6, test: force_realloc_established },
];

unsafe fn do_resume_test(tc: *mut test_case) {
    let mut skel: *mut sock_iter_batch = ptr::null_mut();
    let mut counts: *mut sock_count = ptr::null_mut();
    static port: __u16 = 10001;
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut established_fds: *mut c_int = ptr::null_mut();
    let mut err;
    let mut iter_fd = -1;
    let addr: *const c_char;
    let mut fds: *mut c_int = ptr::null_mut();

    if (*tc).ehash_buckets != 0 {
        SYS_NOFAIL(c"ip netns del sock_iter_batch_child_netns".as_ptr());
        SYS(c"done".as_ptr(), c"sysctl -wq net.ipv4.tcp_child_ehash_entries=%d".as_ptr(), (*tc).ehash_buckets);
        SYS(c"done".as_ptr(), c"ip netns add %s".as_ptr(), TEST_CHILD_NS.as_ptr());
        SYS(c"done".as_ptr(), c"ip -net %s link set dev lo up".as_ptr(), TEST_CHILD_NS.as_ptr());
        nstoken = open_netns(TEST_CHILD_NS.as_ptr() as *const c_char);
        if !ASSERT_OK_PTR(nstoken as *const c_void, c"open_child_netns".as_ptr()) {
            goto_done_resume(nstoken, counts, fds, established_fds, tc, iter_fd, link, skel);
            return;
        }
    }

    counts = calloc((*tc).max_socks as usize, size_of::<sock_count>()) as *mut sock_count;
    if !ASSERT_OK_PTR(counts as *const c_void, c"counts".as_ptr()) {
        goto_done_resume(nstoken, counts, fds, established_fds, tc, iter_fd, link, skel);
        return;
    }
    skel = sock_iter_batch__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"sock_iter_batch__open".as_ptr()) {
        goto_done_resume(nstoken, counts, fds, established_fds, tc, iter_fd, link, skel);
        return;
    }

    /* Prepare a bucket of sockets in the kernel hashtable */
    addr = if (*tc).family == AF_INET6 { c"::1".as_ptr() } else { c"127.0.0.1".as_ptr() };
    fds = start_reuseport_server((*tc).family, (*tc).sock_type, addr, port, 0, (*tc).init_socks);
    if !ASSERT_OK_PTR(fds as *const c_void, c"start_reuseport_server".as_ptr()) {
        goto_done_resume(nstoken, counts, fds, established_fds, tc, iter_fd, link, skel);
        return;
    }
    if (*tc).connections != 0 {
        established_fds = connect_to_server(
            (*tc).family,
            (*tc).sock_type,
            addr,
            port,
            (*tc).connections,
            fds,
            (*tc).init_socks,
        );
        if !ASSERT_OK_PTR(established_fds as *const c_void, c"connect_to_server".as_ptr()) {
            goto_done_resume(nstoken, counts, fds, established_fds, tc, iter_fd, link, skel);
            return;
        }
    }
    (*(*skel).rodata).ports[0] = 0;
    (*(*skel).rodata).ports[1] = 0;
    (*(*skel).rodata).sf = (*tc).family;
    (*(*skel).rodata).ss = 0;

    err = sock_iter_batch__load(skel);
    if !ASSERT_OK(err, c"sock_iter_batch__load".as_ptr()) {
        goto_done_resume(nstoken, counts, fds, established_fds, tc, iter_fd, link, skel);
        return;
    }

    link = bpf_program__attach_iter(
        if (*tc).sock_type == SOCK_STREAM {
            (*skel).progs.iter_tcp_soreuse
        } else {
            (*skel).progs.iter_udp_soreuse
        },
        ptr::null_mut(),
    );
    if !ASSERT_OK_PTR(link as *const c_void, c"bpf_program__attach_iter".as_ptr()) {
        goto_done_resume(nstoken, counts, fds, established_fds, tc, iter_fd, link, skel);
        return;
    }

    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if !ASSERT_OK_FD(iter_fd, c"bpf_iter_create".as_ptr()) {
        goto_done_resume(nstoken, counts, fds, established_fds, tc, iter_fd, link, skel);
        return;
    }

    ((*tc).test)(
        (*tc).family,
        (*tc).sock_type,
        addr,
        port,
        fds,
        (*tc).init_socks,
        established_fds,
        (*tc).connections * 2,
        counts,
        (*tc).max_socks,
        link,
        iter_fd,
    );

    goto_done_resume(nstoken, counts, fds, established_fds, tc, iter_fd, link, skel);
}

unsafe fn goto_done_resume(
    nstoken_: *mut nstoken,
    counts: *mut sock_count,
    fds: *mut c_int,
    established_fds: *mut c_int,
    tc: *mut test_case,
    iter_fd: c_int,
    link: *mut bpf_link,
    skel: *mut sock_iter_batch,
) {
    close_netns(nstoken_);
    SYS_NOFAIL(c"ip netns del sock_iter_batch_child_netns".as_ptr());
    SYS_NOFAIL(c"sysctl -w net.ipv4.tcp_child_ehash_entries=0".as_ptr());
    free(counts as *mut c_void);
    free_fds(fds, (*tc).init_socks);
    free_fds(established_fds, (*tc).connections * 2);
    if iter_fd >= 0 {
        close(iter_fd);
    }
    bpf_link__destroy(link);
    sock_iter_batch__destroy(skel);
}

unsafe fn do_resume_tests() {
    let mut i = 0usize;

    while i < resume_tests.len() {
        if test__start_subtest(resume_tests[i].description) {
            do_resume_test(&mut resume_tests[i]);
        }
        i += 1;
    }
}

unsafe fn do_test(sock_type: c_int, onebyone: bool) {
    let mut err;
    let mut i;
    let mut nread: isize = 0;
    let mut to_read;
    let mut total_read;
    let mut iter_fd = -1;
    let mut outputs: [iter_out; 4] = core::mem::zeroed();
    let mut link: *mut bpf_link = ptr::null_mut();
    let skel: *mut sock_iter_batch;
    let mut first_idx;
    let second_idx;
    let mut fds: [*mut c_int; 2] = [ptr::null_mut(); 2];

    skel = sock_iter_batch__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"sock_iter_batch__open".as_ptr()) {
        return;
    }

    /* Prepare 2 buckets of sockets in the kernel hashtable */
    i = 0;
    while i < fds.len() as c_int {
        let local_port;

        fds[i as usize] = start_reuseport_server(AF_INET6, sock_type, c"::1".as_ptr(), 0, 0, nr_soreuse);
        if !ASSERT_OK_PTR(fds[i as usize] as *const c_void, c"start_reuseport_server".as_ptr()) {
            goto_done_test(&mut fds, iter_fd, link, skel);
            return;
        }
        local_port = get_socket_local_port(*fds[i as usize]);
        if !ASSERT_GE(local_port as isize, 0, c"get_socket_local_port".as_ptr()) {
            goto_done_test(&mut fds, iter_fd, link, skel);
            return;
        }
        (*(*skel).rodata).ports[i as usize] = ntohs(local_port as __u16);
        i += 1;
    }
    (*(*skel).rodata).sf = AF_INET6;
    if sock_type == SOCK_STREAM {
        (*(*skel).rodata).ss = TCP_LISTEN;
    }

    err = sock_iter_batch__load(skel);
    if !ASSERT_OK(err, c"sock_iter_batch__load".as_ptr()) {
        goto_done_test(&mut fds, iter_fd, link, skel);
        return;
    }

    link = bpf_program__attach_iter(
        if sock_type == SOCK_STREAM {
            (*skel).progs.iter_tcp_soreuse
        } else {
            (*skel).progs.iter_udp_soreuse
        },
        ptr::null_mut(),
    );
    if !ASSERT_OK_PTR(link as *const c_void, c"bpf_program__attach_iter".as_ptr()) {
        goto_done_test(&mut fds, iter_fd, link, skel);
        return;
    }

    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if !ASSERT_GE(iter_fd as isize, 0, c"bpf_iter_create".as_ptr()) {
        goto_done_test(&mut fds, iter_fd, link, skel);
        return;
    }

    /* Test reading a bucket (either from fds[0] or fds[1]).
     * Only read "nr_soreuse - 1" number of sockets
     * from a bucket and leave one socket out from
     * that bucket on purpose.
     */
    to_read = (nr_soreuse - 1) * size_of::<iter_out>() as c_int;
    total_read = 0;
    first_idx = -1;
    loop {
        nread = read(
            iter_fd,
            outputs.as_mut_ptr() as *mut c_void,
            if onebyone { size_of::<iter_out>() } else { to_read as usize },
        );
        if nread <= 0 || nread % size_of::<iter_out>() as isize != 0 {
            break;
        }
        total_read += nread as c_int;

        if first_idx == -1 {
            first_idx = outputs[0].idx;
        }
        i = 0;
        while i < (nread as usize / size_of::<iter_out>()) as c_int {
            ASSERT_EQ(outputs[i as usize].idx as isize, first_idx as isize, c"first_idx".as_ptr());
            i += 1;
        }
        if total_read >= to_read {
            break;
        }
    }
    ASSERT_EQ(nread, if onebyone { size_of::<iter_out>() as isize } else { to_read as isize }, c"nread".as_ptr());
    ASSERT_EQ(total_read as isize, to_read as isize, c"total_read".as_ptr());

    free_fds(fds[first_idx as usize], nr_soreuse);
    fds[first_idx as usize] = ptr::null_mut();

    /* Read the "whole" second bucket */
    to_read = nr_soreuse * size_of::<iter_out>() as c_int;
    total_read = 0;
    second_idx = if first_idx == 0 { 1 } else { 0 };
    loop {
        nread = read(
            iter_fd,
            outputs.as_mut_ptr() as *mut c_void,
            if onebyone { size_of::<iter_out>() } else { to_read as usize },
        );
        if nread <= 0 || nread % size_of::<iter_out>() as isize != 0 {
            break;
        }
        total_read += nread as c_int;

        i = 0;
        while i < (nread as usize / size_of::<iter_out>()) as c_int {
            ASSERT_EQ(outputs[i as usize].idx as isize, second_idx as isize, c"second_idx".as_ptr());
            i += 1;
        }
        if total_read > to_read {
            break;
        }
    }
    ASSERT_EQ(nread, 0, c"nread".as_ptr());
    /* Both so_reuseport ports should be in different buckets, so
     * total_read must equal to the expected to_read.
     *
     * For a very unlikely case, both ports collide at the same bucket,
     * the bucket offset (i.e. 3) will be skipped and it cannot
     * expect the to_read number of bytes.
     */
    if (*(*skel).bss).bucket[0] != (*(*skel).bss).bucket[1] {
        ASSERT_EQ(total_read as isize, to_read as isize, c"total_read".as_ptr());
    }

    goto_done_test(&mut fds, iter_fd, link, skel);
}

unsafe fn goto_done_test(
    fds: &mut [*mut c_int; 2],
    iter_fd: c_int,
    link: *mut bpf_link,
    skel: *mut sock_iter_batch,
) {
    let mut i = 0usize;
    while i < fds.len() {
        free_fds(fds[i], nr_soreuse);
        i += 1;
    }
    if iter_fd < 0 {
        close(iter_fd);
    }
    bpf_link__destroy(link);
    sock_iter_batch__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_sock_iter_batch() {
    let mut nstoken: *mut nstoken = ptr::null_mut();

    SYS_NOFAIL(c"ip netns del sock_iter_batch_netns".as_ptr());
    SYS(c"done".as_ptr(), c"ip netns add %s".as_ptr(), TEST_NS.as_ptr());
    SYS(c"done".as_ptr(), c"ip -net %s link set dev lo up".as_ptr(), TEST_NS.as_ptr());

    nstoken = open_netns(TEST_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *const c_void, c"open_netns".as_ptr()) {
        SYS_NOFAIL(c"ip netns del sock_iter_batch_netns".as_ptr());
        return;
    }

    if test__start_subtest(c"tcp".as_ptr()) {
        do_test(SOCK_STREAM, true);
        do_test(SOCK_STREAM, false);
    }
    if test__start_subtest(c"udp".as_ptr()) {
        do_test(SOCK_DGRAM, true);
        do_test(SOCK_DGRAM, false);
    }
    do_resume_tests();
    close_netns(nstoken);

    SYS_NOFAIL(c"ip netns del sock_iter_batch_netns".as_ptr());
}
