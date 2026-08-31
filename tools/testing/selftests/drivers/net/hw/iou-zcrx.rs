// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type sa_family_t = u16;
type in_port_t = u16;

const SKIP_CODE: c_int = 42;

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
union in6_addr_union {
    s6_addr: [u8; 16],
    s6_addr16: [u16; 8],
    s6_addr32: [u32; 4],
}

#[repr(C)]
struct in6_addr {
    __in6_u: in6_addr_union,
}

#[repr(C)]
struct sockaddr {
    sa_family: sa_family_t,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: sa_family_t,
    sin6_port: in_port_t,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct ipv6hdr {
    _private: [u8; 40],
}

#[repr(C)]
struct tcphdr {
    _private: [u8; 20],
}

#[repr(C)]
struct io_uring {
    _private: [u8; 0],
}

#[repr(C)]
struct io_uring_sqe {
    opcode: u8,
    flags: u8,
    ioprio: u16,
    fd: i32,
    off: u64,
    addr: u64,
    len: u32,
    rw_flags: u32,
    user_data: u64,
}

#[repr(C)]
struct io_uring_cqe {
    user_data: u64,
    res: i32,
    flags: u32,
}

#[repr(C)]
struct io_uring_zcrx_offsets {
    head: __u32,
    tail: __u32,
    rqes: __u32,
}

#[repr(C)]
struct io_uring_zcrx_rqe {
    off: __u64,
    len: __u32,
    _pad: __u32,
}

#[repr(C)]
struct io_uring_zcrx_rq {
    khead: *mut c_uint,
    ktail: *mut c_uint,
    rqes: *mut io_uring_zcrx_rqe,
    rq_tail: c_uint,
    ring_entries: c_uint,
}

#[repr(C)]
struct io_uring_zcrx_cqe {
    off: __u64,
}

#[repr(C)]
struct io_uring_region_desc {
    user_addr: __u64,
    size: __u64,
    flags: __u32,
    id: __u32,
    mmap_offset: __u64,
}

#[repr(C)]
struct io_uring_zcrx_area_reg {
    addr: __u64,
    len: __u64,
    rq_area_token: __u64,
    flags: __u32,
    _pad: __u32,
}

#[repr(C)]
struct t_io_uring_zcrx_ifq_reg {
    if_idx: __u32,
    if_rxq: __u32,
    rq_entries: __u32,
    flags: __u32,

    area_ptr: __u64,   /* pointer to struct io_uring_zcrx_area_reg */
    region_ptr: __u64, /* struct io_uring_region_desc * */

    offsets: io_uring_zcrx_offsets,
    zcrx_id: __u32,
    rx_buf_len: __u32,
    __resv: [__u64; 3],
}

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static in6addr_any: in6_addr;

    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn sysconf(name: c_int) -> c_long;
    fn posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...) -> !;

    fn io_uring_register_ifq(ring: *mut io_uring, arg: *mut c_void) -> c_int;
    fn io_uring_get_sqe(ring: *mut io_uring) -> *mut io_uring_sqe;
    fn io_uring_prep_accept(
        sqe: *mut io_uring_sqe,
        fd: c_int,
        addr: *mut sockaddr,
        addrlen: *mut socklen_t,
        flags: c_int,
    );
    fn io_uring_prep_rw(
        op: c_int,
        sqe: *mut io_uring_sqe,
        fd: c_int,
        addr: *const c_void,
        len: c_uint,
        offset: u64,
    );
    fn io_uring_submit_and_wait(ring: *mut io_uring, wait_nr: c_uint) -> c_int;
    fn io_uring_cq_advance(ring: *mut io_uring, nr: c_uint);
    fn io_uring_queue_init(entries: c_uint, ring: *mut io_uring, flags: c_uint) -> c_int;
    fn io_uring_smp_store_release(p: *mut c_uint, v: c_uint);
    fn io_uring_peek_cqe(ring: *mut io_uring, cqe_ptr: *mut *mut io_uring_cqe) -> c_int;
}

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_HUGETLB: c_int = 0x40000;
const MAP_HUGE_SHIFT: c_int = 26;
const MAP_HUGE_2MB: c_int = 21 << MAP_HUGE_SHIFT;
const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const ERANGE: c_int = 34;
const IORING_MEM_REGION_TYPE_USER: __u32 = 1;
const IORING_OP_RECV_ZC: c_int = 72;
const IORING_RECV_MULTISHOT: u16 = 1 << 1;
const IORING_CQE_F_MORE: u32 = 1 << 1;
const IORING_SETUP_COOP_TASKRUN: c_uint = 1 << 8;
const IORING_SETUP_SINGLE_ISSUER: c_uint = 1 << 12;
const IORING_SETUP_DEFER_TASKRUN: c_uint = 1 << 13;
const IORING_SETUP_SUBMIT_ALL: c_uint = 1 << 7;
const IORING_SETUP_CQE32: c_uint = 1 << 11;
const IORING_ZCRX_AREA_SHIFT: c_int = 48;
const IORING_ZCRX_AREA_MASK: u64 = !((1u64 << IORING_ZCRX_AREA_SHIFT) - 1);
const _SC_PAGESIZE: c_int = 30;

static mut page_size: c_long = 0;

unsafe fn AREA_SIZE() -> c_long {
    8192 * page_size
}

const SEND_SIZE: c_int = 512 * 4096;

fn min_t<T: Ord>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

fn ALIGN_UP(v: size_t, align: size_t) -> size_t {
    (v + align - 1) & !(align - 1)
}

static mut cfg_server: c_int = 0;
static mut cfg_client: c_int = 0;
static mut cfg_port: c_int = 8000;
static mut cfg_payload_len: c_int = 0;
static mut cfg_ifname: *const c_char = ptr::null();
static mut cfg_queue_id: c_int = -1;
static mut cfg_oneshot: bool = false;
static mut cfg_oneshot_recvs: c_int = 0;
static mut cfg_send_size: c_int = SEND_SIZE;
static mut cfg_addr: sockaddr_in6 = sockaddr_in6 {
    sin6_family: 0,
    sin6_port: 0,
    sin6_flowinfo: 0,
    sin6_addr: in6_addr { __in6_u: in6_addr_union { s6_addr: [0; 16] } },
    sin6_scope_id: 0,
};
static mut cfg_rx_buf_len: c_uint = 0;
static mut cfg_dry_run: bool = false;

static mut payload: *mut c_char = ptr::null_mut();
static mut area_ptr: *mut c_void = ptr::null_mut();
static mut ring_ptr: *mut c_void = ptr::null_mut();
static mut ring_size: size_t = 0;
static mut rq_ring: io_uring_zcrx_rq = io_uring_zcrx_rq {
    khead: ptr::null_mut(),
    ktail: ptr::null_mut(),
    rqes: ptr::null_mut(),
    rq_tail: 0,
    ring_entries: 0,
};
static mut area_token: c_ulong = 0;
static mut connfd: c_int = 0;
static mut stop: bool = false;
static mut received: size_t = 0;

unsafe fn gettimeofday_ms() -> c_ulong {
    let mut tv: timeval = mem::zeroed();

    gettimeofday(&mut tv, ptr::null_mut());
    (tv.tv_sec as c_ulong * 1000) + (tv.tv_usec as c_ulong / 1000)
}

unsafe fn parse_address(str_: *const c_char, port: c_int, sin6: *mut sockaddr_in6) -> c_int {
    let mut ret: c_int;

    (*sin6).sin6_family = AF_INET6 as sa_family_t;
    (*sin6).sin6_port = htons(port as u16);

    ret = inet_pton((*sin6).sin6_family as c_int, str_, &mut (*sin6).sin6_addr as *mut _ as *mut c_void);
    if ret != 1 {
        /* fallback to plain IPv4 */
        ret = inet_pton(
            AF_INET,
            str_,
            &mut (*sin6).sin6_addr.__in6_u.s6_addr32[3] as *mut _ as *mut c_void,
        );
        if ret != 1 {
            return -1;
        }

        /* add ::ffff prefix */
        (*sin6).sin6_addr.__in6_u.s6_addr32[0] = 0;
        (*sin6).sin6_addr.__in6_u.s6_addr32[1] = 0;
        (*sin6).sin6_addr.__in6_u.s6_addr16[4] = 0;
        (*sin6).sin6_addr.__in6_u.s6_addr16[5] = 0xffff;
    }

    0
}

unsafe fn get_refill_ring_size(rq_entries: c_uint) -> size_t {
    ring_size = rq_entries as size_t * mem::size_of::<io_uring_zcrx_rqe>();
    /* add space for the header (head/tail/etc.) */
    ring_size += page_size as size_t;
    ALIGN_UP(ring_size, page_size as size_t)
}

unsafe fn setup_zcrx(ring: *mut io_uring) {
    let ifindex: c_uint;
    let rq_entries: c_uint = 4096;
    let mut ret: c_int;

    ifindex = if_nametoindex(cfg_ifname);
    if ifindex == 0 {
        error(1, 0, c"bad interface name: %s".as_ptr(), cfg_ifname);
    }

    if cfg_rx_buf_len != 0 && cfg_rx_buf_len as c_long != page_size {
        area_ptr = mmap(
            ptr::null_mut(),
            AREA_SIZE() as size_t,
            PROT_READ | PROT_WRITE,
            MAP_ANONYMOUS | MAP_PRIVATE | MAP_HUGETLB | MAP_HUGE_2MB,
            -1,
            0,
        );
        if area_ptr == (-1isize) as *mut c_void {
            printf(c"Can't allocate huge pages\n".as_ptr());
            exit(SKIP_CODE);
        }
    } else {
        area_ptr = mmap(
            ptr::null_mut(),
            AREA_SIZE() as size_t,
            PROT_READ | PROT_WRITE,
            MAP_ANONYMOUS | MAP_PRIVATE,
            0,
            0,
        );
        if area_ptr == (-1isize) as *mut c_void {
            error(1, 0, c"mmap(): zero copy area".as_ptr());
        }
    }

    ring_size = get_refill_ring_size(rq_entries);
    ring_ptr = mmap(
        ptr::null_mut(),
        ring_size,
        PROT_READ | PROT_WRITE,
        MAP_ANONYMOUS | MAP_PRIVATE,
        0,
        0,
    );

    let mut region_reg = io_uring_region_desc {
        size: ring_size as __u64,
        user_addr: ring_ptr as c_ulong as __u64,
        flags: IORING_MEM_REGION_TYPE_USER,
        id: 0,
        mmap_offset: 0,
    };

    let mut area_reg = io_uring_zcrx_area_reg {
        addr: area_ptr as c_ulong as __u64,
        len: AREA_SIZE() as __u64,
        flags: 0,
        rq_area_token: 0,
        _pad: 0,
    };

    let mut reg = t_io_uring_zcrx_ifq_reg {
        if_idx: ifindex,
        if_rxq: cfg_queue_id as __u32,
        rq_entries,
        flags: 0,
        area_ptr: &mut area_reg as *mut _ as c_ulong as __u64,
        region_ptr: &mut region_reg as *mut _ as c_ulong as __u64,
        offsets: mem::zeroed(),
        zcrx_id: 0,
        rx_buf_len: cfg_rx_buf_len,
        __resv: [0; 3],
    };

    ret = io_uring_register_ifq(ring, &mut reg as *mut _ as *mut c_void);
    if cfg_rx_buf_len != 0 && (ret == -EINVAL || ret == -EOPNOTSUPP || ret == -ERANGE) {
        printf(c"Large chunks are not supported %i\n".as_ptr(), ret);
        exit(SKIP_CODE);
    } else if ret != 0 {
        error(1, 0, c"io_uring_register_ifq(): %d".as_ptr(), ret);
    }

    rq_ring.khead = (ring_ptr as *mut c_char).add(reg.offsets.head as usize) as *mut c_uint;
    rq_ring.ktail = (ring_ptr as *mut c_char).add(reg.offsets.tail as usize) as *mut c_uint;
    rq_ring.rqes = (ring_ptr as *mut c_char).add(reg.offsets.rqes as usize) as *mut io_uring_zcrx_rqe;
    rq_ring.rq_tail = 0;
    rq_ring.ring_entries = reg.rq_entries;

    area_token = area_reg.rq_area_token as c_ulong;
}

unsafe fn add_accept(ring: *mut io_uring, sockfd: c_int) {
    let sqe: *mut io_uring_sqe;

    sqe = io_uring_get_sqe(ring);

    io_uring_prep_accept(sqe, sockfd, ptr::null_mut(), ptr::null_mut(), 0);
    (*sqe).user_data = 1;
}

unsafe fn add_recvzc(ring: *mut io_uring, sockfd: c_int) {
    let sqe: *mut io_uring_sqe;

    sqe = io_uring_get_sqe(ring);

    io_uring_prep_rw(IORING_OP_RECV_ZC, sqe, sockfd, ptr::null(), 0, 0);
    (*sqe).ioprio |= IORING_RECV_MULTISHOT;
    (*sqe).user_data = 2;
}

unsafe fn add_recvzc_oneshot(ring: *mut io_uring, sockfd: c_int, len: size_t) {
    let sqe: *mut io_uring_sqe;

    sqe = io_uring_get_sqe(ring);

    io_uring_prep_rw(IORING_OP_RECV_ZC, sqe, sockfd, ptr::null(), len as c_uint, 0);
    (*sqe).ioprio |= IORING_RECV_MULTISHOT;
    (*sqe).user_data = 2;
}

unsafe fn process_accept(ring: *mut io_uring, cqe: *mut io_uring_cqe) {
    if (*cqe).res < 0 {
        error(1, 0, c"accept()".as_ptr());
    }
    if connfd != 0 {
        error(1, 0, c"Unexpected second connection".as_ptr());
    }

    connfd = (*cqe).res;
    if cfg_oneshot {
        add_recvzc_oneshot(ring, connfd, page_size as size_t);
    } else {
        add_recvzc(ring, connfd);
    }
}

unsafe fn process_recvzc(ring: *mut io_uring, cqe: *mut io_uring_cqe) {
    let rq_mask: c_uint = rq_ring.ring_entries - 1;
    let rcqe: *mut io_uring_zcrx_cqe;
    let rqe: *mut io_uring_zcrx_rqe;
    let mask: u64;
    let data: *mut c_char;
    let n: ssize_t;
    let mut i: c_int;

    if (*cqe).res == 0 && (*cqe).flags == 0 && cfg_oneshot_recvs == 0 {
        stop = true;
        return;
    }

    if (*cqe).res < 0 {
        error(1, 0, c"recvzc(): %d".as_ptr(), (*cqe).res);
    }

    if cfg_oneshot {
        if (*cqe).res == 0 && (*cqe).flags == 0 && cfg_oneshot_recvs != 0 {
            add_recvzc_oneshot(ring, connfd, page_size as size_t);
            cfg_oneshot_recvs -= 1;
        }
    } else if ((*cqe).flags & IORING_CQE_F_MORE) == 0 {
        add_recvzc(ring, connfd);
    }

    rcqe = cqe.add(1) as *mut io_uring_zcrx_cqe;

    n = (*cqe).res as ssize_t;
    mask = (1u64 << IORING_ZCRX_AREA_SHIFT) - 1;
    data = (area_ptr as *mut c_char).add(((*rcqe).off & mask) as usize);

    i = 0;
    while i < n as c_int {
        if *data.add(i as usize) != *payload.add(received + i as usize) {
            error(1, 0, c"payload mismatch at %d".as_ptr(), i);
        }
        i += 1;
    }
    received += n as size_t;

    rqe = rq_ring.rqes.add((rq_ring.rq_tail & rq_mask) as usize);
    (*rqe).off = ((*rcqe).off & !IORING_ZCRX_AREA_MASK) | area_token as u64;
    (*rqe).len = (*cqe).res as __u32;
    rq_ring.rq_tail = rq_ring.rq_tail.wrapping_add(1);
    io_uring_smp_store_release(rq_ring.ktail, rq_ring.rq_tail);
}

unsafe fn server_loop(ring: *mut io_uring) {
    let mut cqe: *mut io_uring_cqe = ptr::null_mut();
    let mut count: c_uint = 0;

    io_uring_submit_and_wait(ring, 1);

    while io_uring_peek_cqe(ring, &mut cqe) == 0 {
        if (*cqe).user_data == 1 {
            process_accept(ring, cqe);
        } else if (*cqe).user_data == 2 {
            process_recvzc(ring, cqe);
        } else {
            error(1, 0, c"unknown cqe".as_ptr());
        }
        count += 1;
        if count >= 1 {
            break;
        }
    }
    io_uring_cq_advance(ring, count);
}

unsafe fn run_server() {
    let mut flags: c_uint = 0;
    let mut ring: io_uring = mem::zeroed();
    let fd: c_int;
    let mut enable: c_int;
    let mut ret: c_int;
    let tstop: u64;

    fd = socket(AF_INET6, SOCK_STREAM, 0);
    if fd == -1 {
        error(1, 0, c"socket()".as_ptr());
    }

    enable = 1;
    ret = setsockopt(
        fd,
        SOL_SOCKET,
        SO_REUSEADDR,
        &enable as *const _ as *const c_void,
        mem::size_of::<c_int>() as socklen_t,
    );
    if ret < 0 {
        error(1, 0, c"setsockopt(SO_REUSEADDR)".as_ptr());
    }

    ret = bind(
        fd,
        &cfg_addr as *const _ as *const sockaddr,
        mem::size_of_val(&cfg_addr) as socklen_t,
    );
    if ret < 0 {
        error(1, 0, c"bind()".as_ptr());
    }

    flags |= IORING_SETUP_COOP_TASKRUN;
    flags |= IORING_SETUP_SINGLE_ISSUER;
    flags |= IORING_SETUP_DEFER_TASKRUN;
    flags |= IORING_SETUP_SUBMIT_ALL;
    flags |= IORING_SETUP_CQE32;

    io_uring_queue_init(512, &mut ring, flags);

    setup_zcrx(&mut ring);
    if cfg_dry_run {
        return;
    }

    if listen(fd, 1024) < 0 {
        error(1, 0, c"listen()".as_ptr());
    }

    add_accept(&mut ring, fd);

    tstop = gettimeofday_ms() as u64 + 5000;
    while !stop && (gettimeofday_ms() as u64) < tstop {
        server_loop(&mut ring);
    }

    if !stop {
        error(1, 0, c"test failed\n".as_ptr());
    }
}

unsafe fn run_client() {
    let mut to_send: ssize_t = cfg_send_size as ssize_t;
    let mut sent: ssize_t = 0;
    let mut chunk: ssize_t;
    let mut res: ssize_t;
    let fd: c_int;

    fd = socket(AF_INET6, SOCK_STREAM, 0);
    if fd == -1 {
        error(1, 0, c"socket()".as_ptr());
    }

    if connect(
        fd,
        &cfg_addr as *const _ as *const sockaddr,
        mem::size_of_val(&cfg_addr) as socklen_t,
    ) != 0
    {
        error(1, 0, c"connect()".as_ptr());
    }

    while to_send != 0 {
        let src: *mut c_void = &mut *payload.add(sent as usize) as *mut _ as *mut c_void;

        chunk = min_t(cfg_payload_len as ssize_t, to_send);
        res = send(fd, src, chunk as size_t, 0);
        if res < 0 {
            error(1, 0, c"send(): %zd".as_ptr(), sent);
        }
        sent += res;
        to_send -= res;
    }

    close(fd);
}

unsafe fn usage(filepath: *const c_char) {
    error(
        1,
        0,
        c"Usage: %s (-4|-6) (-s|-c) -h<server_ip> -p<port> -l<payload_size> -i<ifname> -q<rxq_id>".as_ptr(),
        filepath,
    );
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let max_payload_len: c_int = SEND_SIZE
        - mem::size_of::<ipv6hdr>() as c_int
        - mem::size_of::<tcphdr>() as c_int
        - 40 /* max tcp options */;
    let addr6: *mut sockaddr_in6 = &mut cfg_addr as *mut _;
    let mut addr: *mut c_char = ptr::null_mut();
    let mut ret: c_int;
    let mut c: c_int;

    if argc <= 1 {
        usage(*argv.add(0));
    }
    cfg_payload_len = max_payload_len;

    loop {
        c = getopt(argc, argv, c"sch:p:l:i:q:o:z:x:d".as_ptr());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            's' => {
                if cfg_client != 0 {
                    error(1, 0, c"Pass one of -s or -c".as_ptr());
                }
                cfg_server = 1;
            }
            'c' => {
                if cfg_server != 0 {
                    error(1, 0, c"Pass one of -s or -c".as_ptr());
                }
                cfg_client = 1;
            }
            'h' => {
                addr = optarg;
            }
            'p' => {
                cfg_port = strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'l' => {
                cfg_payload_len = strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'i' => {
                cfg_ifname = optarg;
            }
            'q' => {
                cfg_queue_id = strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'o' => {
                cfg_oneshot = true;
                cfg_oneshot_recvs = strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'z' => {
                cfg_send_size = strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'x' => {
                cfg_rx_buf_len = (page_size as c_ulong * strtoul(optarg, ptr::null_mut(), 0)) as c_uint;
            }
            'd' => {
                cfg_dry_run = true;
            }
            _ => {}
        }
    }

    if cfg_server != 0 && !addr.is_null() {
        error(1, 0, c"Receiver cannot have -h specified".as_ptr());
    }

    memset(addr6 as *mut c_void, 0, mem::size_of_val(&*addr6));
    (*addr6).sin6_family = AF_INET6 as sa_family_t;
    (*addr6).sin6_port = htons(cfg_port as u16);
    (*addr6).sin6_addr = in6addr_any;
    if !addr.is_null() {
        ret = parse_address(addr, cfg_port, addr6);
        if ret != 0 {
            error(1, 0, c"receiver address parse error: %s".as_ptr(), addr);
        }
    }

    if cfg_payload_len > max_payload_len {
        error(1, 0, c"-l: payload exceeds max (%d)".as_ptr(), max_payload_len);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let cfg_test: *const c_char = *argv.add((argc - 1) as usize);
    let mut i: c_int;

    page_size = sysconf(_SC_PAGESIZE);
    if page_size < 0 {
        return 1;
    }

    if posix_memalign(
        &mut payload as *mut _ as *mut *mut c_void,
        page_size as size_t,
        SEND_SIZE as size_t,
    ) != 0
    {
        return 1;
    }

    parse_opts(argc, argv);

    i = 0;
    while i < SEND_SIZE {
        *payload.add(i as usize) = (b'a' + (i % 26) as u8) as c_char;
        i += 1;
    }

    if cfg_server != 0 {
        run_server();
    } else if cfg_client != 0 {
        run_client();
    }

    0
}
