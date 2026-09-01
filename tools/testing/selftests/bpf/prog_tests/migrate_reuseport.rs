// SPDX-License-Identifier: GPL-2.0
/*
 * Check if we can migrate child sockets.
 *
 *   1. call listen() for 4 server sockets.
 *   2. call connect() for 25 client sockets.
 *   3. call listen() for 1 server socket. (migration target)
 *   4. update a map to migrate all child sockets
 *        to the last server socket (migrate_map[cookie] = 4)
 *   5. for TCP_ESTABLISHED and TCP_SYN_RECV cases, verify via epoll
 *        that the last server socket is not ready before migration.
 *   6. call shutdown() for first 4 server sockets
 *        and migrate the requests in the accept queue
 *        to the last server socket.
 *   7. for TCP_ESTABLISHED and TCP_SYN_RECV cases, verify via epoll
 *        that the last server socket is ready after migration.
 *   8. call listen() for the second server socket.
 *   9. call shutdown() for the last server
 *        and migrate the requests in the accept queue
 *        to the second server socket.
 *  10. call listen() for the last server.
 *  11. call shutdown() for the second server
 *        and migrate the requests in the accept queue
 *        to the last server socket.
 *  12. call accept() for the last server socket.
 *
 * Author: Kuniyuki Iwashima <kuniyu@amazon.co.jp>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

// C includes translated as external dependencies:
// <bpf/bpf.h>, <bpf/libbpf.h>, <sys/epoll.h>,
// "test_progs.h", "test_migrate_reuseport.skel.h", "network_helpers.h".

const TCP_FASTOPEN_CONNECT: c_int = 30;

const IFINDEX_LO: c_int = 1;

const NR_SERVERS: usize = 5;
const NR_CLIENTS: usize = NR_SERVERS * 5;
const MIGRATED_TO: usize = NR_SERVERS - 1;

/* fastopenq->max_qlen and sk->sk_max_ack_backlog */
const QLEN: c_int = (NR_CLIENTS * 5) as c_int;

const MSG: &[u8; 12] = b"Hello World\0";
const MSGLEN: usize = 12;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const IPPROTO_TCP: c_int = 6;
const SOL_SOCKET: c_int = 1;
const SOL_TCP: c_int = 6;
const SO_REUSEPORT: c_int = 15;
const SO_ATTACH_REUSEPORT_EBPF: c_int = 52;
const TCP_FASTOPEN: c_int = 23;
const O_RDWR: c_int = 0o2;
const SEEK_SET: c_int = 0;
const EPOLLIN: u32 = 0x001;
const EPOLL_CTL_ADD: c_int = 1;
const SHUT_RDWR: c_int = 2;
const BPF_NOEXIST: u64 = 1;

const BPF_TCP_ESTABLISHED: c_int = 1;
const BPF_TCP_SYN_RECV: c_int = 3;
const BPF_TCP_NEW_SYN_RECV: c_int = 12;

type S64 = i64;
type U64 = u64;
type SocklenT = u32;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: u16,
    pub __data: [u8; 126],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
pub union epoll_data {
    pub ptr: *mut c_void,
    pub fd: c_int,
    pub u32_: u32,
    pub u64_: u64,
}

#[repr(C)]
pub struct epoll_event {
    pub events: u32,
    pub data: epoll_data,
}

#[repr(C)]
pub struct test_migrate_reuseport_bss {
    pub server_port: u16,
    pub migrated_at_close: c_int,
    pub migrated_at_close_fastopen: c_int,
    pub migrated_at_send_synack: c_int,
    pub migrated_at_recv_ack: c_int,
}

#[repr(C)]
pub struct test_migrate_reuseport_progs {
    pub drop_ack: *mut bpf_program,
    pub migrate_reuseport: *mut bpf_program,
}

#[repr(C)]
pub struct test_migrate_reuseport_maps {
    pub reuseport_map: *mut bpf_map,
    pub migrate_map: *mut bpf_map,
}

#[repr(C)]
pub struct test_migrate_reuseport {
    pub bss: *mut test_migrate_reuseport_bss,
    pub progs: test_migrate_reuseport_progs,
    pub maps: test_migrate_reuseport_maps,
}

#[repr(C)]
struct migrate_reuseport_test_case {
    name: *const c_char,
    servers: [S64; NR_SERVERS],
    clients: [S64; NR_CLIENTS],
    addr: sockaddr_storage,
    addrlen: SocklenT,
    family: c_int,
    state: c_int,
    drop_ack: bool,
    expire_synack_timer: bool,
    fastopen: bool,
    link: *mut bpf_link,
}

unsafe extern "C" {
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn lseek(fd: c_int, offset: isize, whence: c_int) -> isize;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: SocklenT,
    ) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: SocklenT) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut SocklenT) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: SocklenT) -> c_int;
    fn shutdown(sockfd: c_int, how: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut SocklenT) -> c_int;
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn bpf_program__attach_xdp(prog: *mut bpf_program, ifindex: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

    fn make_sockaddr(
        family: c_int,
        addr_str: *const c_char,
        port: u16,
        addr: *mut sockaddr_storage,
        addrlen: *mut SocklenT,
    );
    fn settimeo(fd: c_int, timeout_ms: c_int) -> c_int;
    fn test__start_subtest(name: *const c_char);
    fn test_migrate_reuseport__open_and_load() -> *mut test_migrate_reuseport;
    fn test_migrate_reuseport__destroy(skel: *mut test_migrate_reuseport);

    fn ASSERT_NEQ(actual: S64, expected: S64, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
}

static mut TEST_CASES: [migrate_reuseport_test_case; 8] = [
    migrate_reuseport_test_case {
        name: c"IPv4 TCP_ESTABLISHED  inet_csk_listen_stop".as_ptr(),
        servers: [0; NR_SERVERS],
        clients: [0; NR_CLIENTS],
        addr: sockaddr_storage { ss_family: 0, __data: [0; 126] },
        addrlen: 0,
        family: AF_INET,
        state: BPF_TCP_ESTABLISHED,
        drop_ack: false,
        expire_synack_timer: false,
        fastopen: false,
        link: ptr::null_mut(),
    },
    migrate_reuseport_test_case {
        name: c"IPv4 TCP_SYN_RECV     inet_csk_listen_stop".as_ptr(),
        servers: [0; NR_SERVERS],
        clients: [0; NR_CLIENTS],
        addr: sockaddr_storage { ss_family: 0, __data: [0; 126] },
        addrlen: 0,
        family: AF_INET,
        state: BPF_TCP_SYN_RECV,
        drop_ack: true,
        expire_synack_timer: false,
        fastopen: true,
        link: ptr::null_mut(),
    },
    migrate_reuseport_test_case {
        name: c"IPv4 TCP_NEW_SYN_RECV reqsk_timer_handler".as_ptr(),
        servers: [0; NR_SERVERS],
        clients: [0; NR_CLIENTS],
        addr: sockaddr_storage { ss_family: 0, __data: [0; 126] },
        addrlen: 0,
        family: AF_INET,
        state: BPF_TCP_NEW_SYN_RECV,
        drop_ack: true,
        expire_synack_timer: true,
        fastopen: false,
        link: ptr::null_mut(),
    },
    migrate_reuseport_test_case {
        name: c"IPv4 TCP_NEW_SYN_RECV inet_csk_complete_hashdance".as_ptr(),
        servers: [0; NR_SERVERS],
        clients: [0; NR_CLIENTS],
        addr: sockaddr_storage { ss_family: 0, __data: [0; 126] },
        addrlen: 0,
        family: AF_INET,
        state: BPF_TCP_NEW_SYN_RECV,
        drop_ack: true,
        expire_synack_timer: false,
        fastopen: false,
        link: ptr::null_mut(),
    },
    migrate_reuseport_test_case {
        name: c"IPv6 TCP_ESTABLISHED  inet_csk_listen_stop".as_ptr(),
        servers: [0; NR_SERVERS],
        clients: [0; NR_CLIENTS],
        addr: sockaddr_storage { ss_family: 0, __data: [0; 126] },
        addrlen: 0,
        family: AF_INET6,
        state: BPF_TCP_ESTABLISHED,
        drop_ack: false,
        expire_synack_timer: false,
        fastopen: false,
        link: ptr::null_mut(),
    },
    migrate_reuseport_test_case {
        name: c"IPv6 TCP_SYN_RECV     inet_csk_listen_stop".as_ptr(),
        servers: [0; NR_SERVERS],
        clients: [0; NR_CLIENTS],
        addr: sockaddr_storage { ss_family: 0, __data: [0; 126] },
        addrlen: 0,
        family: AF_INET6,
        state: BPF_TCP_SYN_RECV,
        drop_ack: true,
        expire_synack_timer: false,
        fastopen: true,
        link: ptr::null_mut(),
    },
    migrate_reuseport_test_case {
        name: c"IPv6 TCP_NEW_SYN_RECV reqsk_timer_handler".as_ptr(),
        servers: [0; NR_SERVERS],
        clients: [0; NR_CLIENTS],
        addr: sockaddr_storage { ss_family: 0, __data: [0; 126] },
        addrlen: 0,
        family: AF_INET6,
        state: BPF_TCP_NEW_SYN_RECV,
        drop_ack: true,
        expire_synack_timer: true,
        fastopen: false,
        link: ptr::null_mut(),
    },
    migrate_reuseport_test_case {
        name: c"IPv6 TCP_NEW_SYN_RECV inet_csk_complete_hashdance".as_ptr(),
        servers: [0; NR_SERVERS],
        clients: [0; NR_CLIENTS],
        addr: sockaddr_storage { ss_family: 0, __data: [0; 126] },
        addrlen: 0,
        family: AF_INET6,
        state: BPF_TCP_NEW_SYN_RECV,
        drop_ack: true,
        expire_synack_timer: false,
        fastopen: false,
        link: ptr::null_mut(),
    },
];

unsafe fn init_fds(fds: *mut S64, len: c_int) {
    let mut i: c_int = 0;

    while i < len {
        *fds.add(i as usize) = -1;
        i += 1;
    }
}

unsafe fn close_fds(fds: *mut S64, len: c_int) {
    let mut i: c_int = 0;

    while i < len {
        if *fds.add(i as usize) != -1 {
            close(*fds.add(i as usize) as c_int);
            *fds.add(i as usize) = -1;
        }
        i += 1;
    }
}

unsafe fn setup_fastopen(buf: *mut c_char, size: c_int, saved_len: *mut c_int, restore: bool) -> c_int {
    let mut err: c_int = 0;
    let fd: c_int;
    let mut len: c_int;

    fd = open(c"/proc/sys/net/ipv4/tcp_fastopen".as_ptr(), O_RDWR);
    if !ASSERT_NEQ(fd as S64, -1, c"open".as_ptr()) {
        return -1;
    }

    if restore {
        len = write(fd, buf as *const c_void, *saved_len as usize) as c_int;
        if !ASSERT_EQ(len, *saved_len, c"write - restore".as_ptr()) {
            err = -1;
        }
    } else {
        *saved_len = read(fd, buf as *mut c_void, size as usize) as c_int;
        if !ASSERT_GE(*saved_len, 1, c"read".as_ptr()) {
            err = -1;
            goto_close(fd);
            return err;
        }

        err = lseek(fd, 0, SEEK_SET) as c_int;
        if !ASSERT_OK(err, c"lseek".as_ptr()) {
            goto_close(fd);
            return err;
        }

        /* (TFO_CLIENT_ENABLE | TFO_SERVER_ENABLE |
         *  TFO_CLIENT_NO_COOKIE | TFO_SERVER_COOKIE_NOT_REQD)
         */
        len = write(fd, c"519".as_ptr() as *const c_void, 3) as c_int;
        if !ASSERT_EQ(len, 3, c"write - setup".as_ptr()) {
            err = -1;
        }
    }

    goto_close(fd);

    err
}

unsafe fn goto_close(fd: c_int) {
    close(fd);
}

unsafe fn drop_ack(test_case: *mut migrate_reuseport_test_case, skel: *mut test_migrate_reuseport) -> c_int {
    if (*test_case).family == AF_INET {
        (*(*skel).bss).server_port = (*((&mut (*test_case).addr) as *mut sockaddr_storage as *mut sockaddr_in)).sin_port;
    } else {
        (*(*skel).bss).server_port = (*((&mut (*test_case).addr) as *mut sockaddr_storage as *mut sockaddr_in6)).sin6_port;
    }

    (*test_case).link = bpf_program__attach_xdp((*skel).progs.drop_ack, IFINDEX_LO);
    if !ASSERT_OK_PTR((*test_case).link as *mut c_void, c"bpf_program__attach_xdp".as_ptr()) {
        return -1;
    }

    0
}

unsafe fn pass_ack(test_case: *mut migrate_reuseport_test_case) -> c_int {
    let err: c_int;

    err = bpf_link__destroy((*test_case).link);
    if !ASSERT_OK(err, c"bpf_link__destroy".as_ptr()) {
        return -1;
    }

    (*test_case).link = ptr::null_mut();

    0
}

unsafe fn start_servers(test_case: *mut migrate_reuseport_test_case, skel: *mut test_migrate_reuseport) -> c_int {
    let mut i: c_int;
    let mut err: c_int;
    let prog_fd: c_int;
    let reuseport: c_int = 1;
    let qlen: c_int = QLEN;

    prog_fd = bpf_program__fd((*skel).progs.migrate_reuseport);

    make_sockaddr(
        (*test_case).family,
        if (*test_case).family == AF_INET { c"127.0.0.1".as_ptr() } else { c"::1".as_ptr() },
        0,
        &mut (*test_case).addr,
        &mut (*test_case).addrlen,
    );

    i = 0;
    while i < NR_SERVERS as c_int {
        (*test_case).servers[i as usize] = socket((*test_case).family, SOCK_STREAM, IPPROTO_TCP) as S64;
        if !ASSERT_NEQ((*test_case).servers[i as usize], -1, c"socket".as_ptr()) {
            return -1;
        }

        err = setsockopt(
            (*test_case).servers[i as usize] as c_int,
            SOL_SOCKET,
            SO_REUSEPORT,
            &reuseport as *const c_int as *const c_void,
            size_of::<c_int>() as SocklenT,
        );
        if !ASSERT_OK(err, c"setsockopt - SO_REUSEPORT".as_ptr()) {
            return -1;
        }

        err = bind(
            (*test_case).servers[i as usize] as c_int,
            &(*test_case).addr as *const sockaddr_storage as *const sockaddr,
            (*test_case).addrlen,
        );
        if !ASSERT_OK(err, c"bind".as_ptr()) {
            return -1;
        }

        if i == 0 {
            err = setsockopt(
                (*test_case).servers[i as usize] as c_int,
                SOL_SOCKET,
                SO_ATTACH_REUSEPORT_EBPF,
                &prog_fd as *const c_int as *const c_void,
                size_of::<c_int>() as SocklenT,
            );
            if !ASSERT_OK(err, c"setsockopt - SO_ATTACH_REUSEPORT_EBPF".as_ptr()) {
                return -1;
            }

            err = getsockname(
                (*test_case).servers[i as usize] as c_int,
                &mut (*test_case).addr as *mut sockaddr_storage as *mut sockaddr,
                &mut (*test_case).addrlen,
            );
            if !ASSERT_OK(err, c"getsockname".as_ptr()) {
                return -1;
            }
        }

        if (*test_case).fastopen {
            err = setsockopt(
                (*test_case).servers[i as usize] as c_int,
                SOL_TCP,
                TCP_FASTOPEN,
                &qlen as *const c_int as *const c_void,
                size_of::<c_int>() as SocklenT,
            );
            if !ASSERT_OK(err, c"setsockopt - TCP_FASTOPEN".as_ptr()) {
                return -1;
            }
        }

        /* All requests will be tied to the first four listeners */
        if i != MIGRATED_TO as c_int {
            err = listen((*test_case).servers[i as usize] as c_int, qlen);
            if !ASSERT_OK(err, c"listen".as_ptr()) {
                return -1;
            }
        }

        i += 1;
    }

    0
}

unsafe fn start_clients(test_case: *mut migrate_reuseport_test_case) -> c_int {
    let buf: [c_char; MSGLEN] = *MSG as [u8; MSGLEN] as [c_char; MSGLEN];
    let mut i: c_int;
    let mut err: c_int;

    i = 0;
    while i < NR_CLIENTS as c_int {
        (*test_case).clients[i as usize] = socket((*test_case).family, SOCK_STREAM, IPPROTO_TCP) as S64;
        if !ASSERT_NEQ((*test_case).clients[i as usize], -1, c"socket".as_ptr()) {
            return -1;
        }

        /* The attached XDP program drops only the final ACK, so
         * clients will transition to TCP_ESTABLISHED immediately.
         */
        err = settimeo((*test_case).clients[i as usize] as c_int, 100);
        if !ASSERT_OK(err, c"settimeo".as_ptr()) {
            return -1;
        }

        if (*test_case).fastopen {
            let fastopen: c_int = 1;

            err = setsockopt(
                (*test_case).clients[i as usize] as c_int,
                IPPROTO_TCP,
                TCP_FASTOPEN_CONNECT,
                &fastopen as *const c_int as *const c_void,
                size_of::<c_int>() as SocklenT,
            );
            if !ASSERT_OK(err, c"setsockopt - TCP_FASTOPEN_CONNECT".as_ptr()) {
                return -1;
            }
        }

        err = connect(
            (*test_case).clients[i as usize] as c_int,
            &(*test_case).addr as *const sockaddr_storage as *const sockaddr,
            (*test_case).addrlen,
        );
        if !ASSERT_OK(err, c"connect".as_ptr()) {
            return -1;
        }

        err = write((*test_case).clients[i as usize] as c_int, buf.as_ptr() as *const c_void, MSGLEN) as c_int;
        if !ASSERT_EQ(err, MSGLEN as c_int, c"write".as_ptr()) {
            return -1;
        }

        i += 1;
    }

    0
}

unsafe fn update_maps(test_case: *mut migrate_reuseport_test_case, skel: *mut test_migrate_reuseport) -> c_int {
    let mut i: c_int;
    let mut err: c_int;
    let migrated_to: c_int = MIGRATED_TO as c_int;
    let reuseport_map_fd: c_int;
    let migrate_map_fd: c_int;
    let mut value: U64;

    reuseport_map_fd = bpf_map__fd((*skel).maps.reuseport_map);
    migrate_map_fd = bpf_map__fd((*skel).maps.migrate_map);

    i = 0;
    while i < NR_SERVERS as c_int {
        value = (*test_case).servers[i as usize] as U64;
        err = bpf_map_update_elem(
            reuseport_map_fd,
            &i as *const c_int as *const c_void,
            &value as *const U64 as *const c_void,
            BPF_NOEXIST,
        );
        if !ASSERT_OK(err, c"bpf_map_update_elem - reuseport_map".as_ptr()) {
            return -1;
        }

        err = bpf_map_lookup_elem(
            reuseport_map_fd,
            &i as *const c_int as *const c_void,
            &mut value as *mut U64 as *mut c_void,
        );
        if !ASSERT_OK(err, c"bpf_map_lookup_elem - reuseport_map".as_ptr()) {
            return -1;
        }

        err = bpf_map_update_elem(
            migrate_map_fd,
            &value as *const U64 as *const c_void,
            &migrated_to as *const c_int as *const c_void,
            BPF_NOEXIST,
        );
        if !ASSERT_OK(err, c"bpf_map_update_elem - migrate_map".as_ptr()) {
            return -1;
        }

        i += 1;
    }

    0
}

unsafe fn migrate_dance(test_case: *mut migrate_reuseport_test_case) -> c_int {
    let mut ev = epoll_event {
        events: EPOLLIN,
        data: epoll_data { u64_: 0 },
    };
    let mut epoll: c_int = -1;
    let mut nfds: c_int;
    let mut i: c_int;
    let mut err: c_int;

    if (*test_case).state != BPF_TCP_NEW_SYN_RECV {
        epoll = epoll_create1(0);
        if !ASSERT_NEQ(epoll as S64, -1, c"epoll_create1".as_ptr()) {
            return -1;
        }

        ev.data.fd = (*test_case).servers[MIGRATED_TO] as c_int;
        if !ASSERT_OK(
            epoll_ctl(epoll, EPOLL_CTL_ADD, (*test_case).servers[MIGRATED_TO] as c_int, &mut ev),
            c"epoll_ctl".as_ptr(),
        ) {
            return close_epoll(epoll);
        }

        nfds = epoll_wait(epoll, &mut ev, 1, 0);
        if !ASSERT_EQ(nfds, 0, c"epoll_wait 1".as_ptr()) {
            return close_epoll(epoll);
        }
    }

    /* Migrate TCP_ESTABLISHED and TCP_SYN_RECV requests
     * to the last listener based on eBPF.
     */
    i = 0;
    while i < MIGRATED_TO as c_int {
        err = shutdown((*test_case).servers[i as usize] as c_int, SHUT_RDWR);
        if !ASSERT_OK(err, c"shutdown".as_ptr()) {
            return close_epoll(epoll);
        }
        i += 1;
    }

    /* No dance for TCP_NEW_SYN_RECV to migrate based on eBPF */
    if (*test_case).state == BPF_TCP_NEW_SYN_RECV {
        return 0;
    }

    nfds = epoll_wait(epoll, &mut ev, 1, 0);
    if !ASSERT_EQ(nfds, 1, c"epoll_wait 2".as_ptr()) {
        return close_epoll(epoll);
    }

    close(epoll);

    /* Note that we use the second listener instead of the
     * first one here.
     *
     * The fist listener is bind()ed with port 0 and,
     * SOCK_BINDPORT_LOCK is not set to sk_userlocks, so
     * calling listen() again will bind() the first listener
     * on a new ephemeral port and detach it from the existing
     * reuseport group.  (See: __inet_bind(), tcp_set_state())
     *
     * OTOH, the second one is bind()ed with a specific port,
     * and SOCK_BINDPORT_LOCK is set. Thus, re-listen() will
     * resurrect the listener on the existing reuseport group.
     */
    err = listen((*test_case).servers[1] as c_int, QLEN);
    if !ASSERT_OK(err, c"listen".as_ptr()) {
        return -1;
    }

    /* Migrate from the last listener to the second one.
     *
     * All listeners were detached out of the reuseport_map,
     * so migration will be done by kernel random pick from here.
     */
    err = shutdown((*test_case).servers[MIGRATED_TO] as c_int, SHUT_RDWR);
    if !ASSERT_OK(err, c"shutdown".as_ptr()) {
        return -1;
    }

    /* Back to the existing reuseport group */
    err = listen((*test_case).servers[MIGRATED_TO] as c_int, QLEN);
    if !ASSERT_OK(err, c"listen".as_ptr()) {
        return -1;
    }

    /* Migrate back to the last one from the second one */
    err = shutdown((*test_case).servers[1] as c_int, SHUT_RDWR);
    if !ASSERT_OK(err, c"shutdown".as_ptr()) {
        return -1;
    }

    0
}

unsafe fn close_epoll(epoll: c_int) -> c_int {
    if epoll >= 0 {
        close(epoll);
    }
    -1
}

unsafe fn count_requests(test_case: *mut migrate_reuseport_test_case, skel: *mut test_migrate_reuseport) {
    let mut addr: sockaddr_storage = zeroed();
    let mut len: SocklenT = size_of::<sockaddr_storage>() as SocklenT;
    let mut err: c_int;
    let mut cnt: c_int = 0;
    let mut client: c_int;
    let mut buf: [c_char; MSGLEN] = [0; MSGLEN];

    err = settimeo((*test_case).servers[MIGRATED_TO] as c_int, 4000);
    if !ASSERT_OK(err, c"settimeo".as_ptr()) {
        ASSERT_EQ(cnt, NR_CLIENTS as c_int, c"count in userspace".as_ptr());
        cnt = count_from_bpf(test_case, skel);
        ASSERT_EQ(cnt, NR_CLIENTS as c_int, c"count in BPF prog".as_ptr());
        return;
    }

    while cnt < NR_CLIENTS as c_int {
        client = accept(
            (*test_case).servers[MIGRATED_TO] as c_int,
            &mut addr as *mut sockaddr_storage as *mut sockaddr,
            &mut len,
        );
        if !ASSERT_NEQ(client as S64, -1, c"accept".as_ptr()) {
            break;
        }

        memset(buf.as_mut_ptr() as *mut c_void, 0, MSGLEN);
        read(client, buf.as_mut_ptr() as *mut c_void, MSGLEN);
        close(client);

        if !ASSERT_STREQ(buf.as_ptr(), MSG.as_ptr() as *const c_char, c"read".as_ptr()) {
            break;
        }

        cnt += 1;
    }

    ASSERT_EQ(cnt, NR_CLIENTS as c_int, c"count in userspace".as_ptr());

    cnt = count_from_bpf(test_case, skel);

    ASSERT_EQ(cnt, NR_CLIENTS as c_int, c"count in BPF prog".as_ptr());
}

unsafe fn count_from_bpf(test_case: *mut migrate_reuseport_test_case, skel: *mut test_migrate_reuseport) -> c_int {
    match (*test_case).state {
        BPF_TCP_ESTABLISHED => (*(*skel).bss).migrated_at_close,
        BPF_TCP_SYN_RECV => (*(*skel).bss).migrated_at_close_fastopen,
        BPF_TCP_NEW_SYN_RECV => {
            if (*test_case).expire_synack_timer {
                (*(*skel).bss).migrated_at_send_synack
            } else {
                (*(*skel).bss).migrated_at_recv_ack
            }
        }
        _ => 0,
    }
}

unsafe fn run_test(test_case: *mut migrate_reuseport_test_case, skel: *mut test_migrate_reuseport) {
    let mut err: c_int;
    let mut saved_len: c_int = 0;
    let mut buf: [c_char; 16] = [0; 16];

    (*(*skel).bss).migrated_at_close = 0;
    (*(*skel).bss).migrated_at_close_fastopen = 0;
    (*(*skel).bss).migrated_at_send_synack = 0;
    (*(*skel).bss).migrated_at_recv_ack = 0;

    init_fds((*test_case).servers.as_mut_ptr(), NR_SERVERS as c_int);
    init_fds((*test_case).clients.as_mut_ptr(), NR_CLIENTS as c_int);

    if (*test_case).fastopen {
        memset(buf.as_mut_ptr() as *mut c_void, 0, size_of::<[c_char; 16]>());

        err = setup_fastopen(buf.as_mut_ptr(), size_of::<[c_char; 16]>() as c_int, &mut saved_len, false);
        if !ASSERT_OK(err, c"setup_fastopen - setup".as_ptr()) {
            return;
        }
    }

    err = start_servers(test_case, skel);
    if !ASSERT_OK(err, c"start_servers".as_ptr()) {
        close_servers(test_case, skel, &mut buf, &mut saved_len);
        return;
    }

    if (*test_case).drop_ack {
        /* Drop the final ACK of the 3-way handshake and stick the
         * in-flight requests on TCP_SYN_RECV or TCP_NEW_SYN_RECV.
         */
        err = drop_ack(test_case, skel);
        if !ASSERT_OK(err, c"drop_ack".as_ptr()) {
            close_servers(test_case, skel, &mut buf, &mut saved_len);
            return;
        }
    }

    /* Tie requests to the first four listeners */
    err = start_clients(test_case);
    if !ASSERT_OK(err, c"start_clients".as_ptr()) {
        close_clients(test_case, skel, &mut buf, &mut saved_len);
        return;
    }

    err = listen((*test_case).servers[MIGRATED_TO] as c_int, QLEN);
    if !ASSERT_OK(err, c"listen".as_ptr()) {
        close_clients(test_case, skel, &mut buf, &mut saved_len);
        return;
    }

    err = update_maps(test_case, skel);
    if !ASSERT_OK(err, c"fill_maps".as_ptr()) {
        close_clients(test_case, skel, &mut buf, &mut saved_len);
        return;
    }

    /* Migrate the requests in the accept queue only.
     * TCP_NEW_SYN_RECV requests are not migrated at this point.
     */
    err = migrate_dance(test_case);
    if !ASSERT_OK(err, c"migrate_dance".as_ptr()) {
        close_clients(test_case, skel, &mut buf, &mut saved_len);
        return;
    }

    if (*test_case).expire_synack_timer {
        /* Wait for SYN+ACK timers to expire so that
         * reqsk_timer_handler() migrates TCP_NEW_SYN_RECV requests.
         */
        sleep(1);
    }

    if !(*test_case).link.is_null() {
        /* Resume 3WHS and migrate TCP_NEW_SYN_RECV requests */
        err = pass_ack(test_case);
        if !ASSERT_OK(err, c"pass_ack".as_ptr()) {
            close_clients(test_case, skel, &mut buf, &mut saved_len);
            return;
        }
    }

    count_requests(test_case, skel);

    close_clients(test_case, skel, &mut buf, &mut saved_len);
}

unsafe fn close_clients(
    test_case: *mut migrate_reuseport_test_case,
    skel: *mut test_migrate_reuseport,
    buf: *mut [c_char; 16],
    saved_len: *mut c_int,
) {
    let mut err: c_int;

    close_fds((*test_case).clients.as_mut_ptr(), NR_CLIENTS as c_int);

    if !(*test_case).link.is_null() {
        err = pass_ack(test_case);
        ASSERT_OK(err, c"pass_ack - clean up".as_ptr());
    }

    close_servers(test_case, skel, buf, saved_len);
}

unsafe fn close_servers(
    test_case: *mut migrate_reuseport_test_case,
    _skel: *mut test_migrate_reuseport,
    buf: *mut [c_char; 16],
    saved_len: *mut c_int,
) {
    let mut err: c_int;

    close_fds((*test_case).servers.as_mut_ptr(), NR_SERVERS as c_int);

    if (*test_case).fastopen {
        err = setup_fastopen((*buf).as_mut_ptr(), size_of::<[c_char; 16]>() as c_int, saved_len, true);
        ASSERT_OK(err, c"setup_fastopen - restore".as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_migrate_reuseport() {
    let skel: *mut test_migrate_reuseport;
    let mut i: usize;

    skel = test_migrate_reuseport__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"open_and_load".as_ptr()) {
        return;
    }

    i = 0;
    while i < TEST_CASES.len() {
        test__start_subtest(TEST_CASES[i].name);
        run_test(&mut TEST_CASES[i], skel);
        i += 1;
    }

    test_migrate_reuseport__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
