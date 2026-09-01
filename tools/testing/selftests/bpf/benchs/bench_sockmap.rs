// SPDX-License-Identifier: GPL-2.0

// Translated from bench_sockmap.c. C include dependencies are represented as
// external declarations below.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type socklen_t = u32;
type ssize_t = isize;
type off_t = i64;
type time_t = i64;
type error_t = c_int;

const FILE_SIZE: c_int = 128 * 1024;
const DATA_REPEAT_SIZE: usize = 10;

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_REUSEPORT: c_int = 15;
const SOMAXCONN: c_int = 4096;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const F_GETFL: c_int = 3;
const F_SETFL: c_int = 4;
const O_NONBLOCK: c_int = 0o4000;
const EINPROGRESS: c_int = 115;
const EAGAIN: c_int = 11;
const ENOMEM: c_int = 12;
const ENOBUFS: c_int = 105;
const SEEK_SET: c_int = 0;
const CLOCK_MONOTONIC: c_int = 1;
const BPF_NOEXIST: u64 = 1;
const BPF_SK_SKB_STREAM_PARSER: c_int = 0;
const BPF_SK_SKB_STREAM_VERDICT: c_int = 1;
const BPF_SK_MSG_VERDICT: c_int = 7;
const BPF_F_INGRESS: c_int = 1;
const ARGP_ERR_UNKNOWN: error_t = 7;

static SND_DATA: [c_char; DATA_REPEAT_SIZE] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

/* c1 <-> [p1, p2] <-> c2
 * RX bench(BPF_SK_SKB_STREAM_VERDICT):
 *	ARG_FW_RX_PASS:
 *		send(p2) -> recv(c2) -> bpf skb passthrough -> recv(c2)
 *	ARG_FW_RX_VERDICT_EGRESS:
 *		send(c1) -> verdict skb to tx queuec of p2 -> recv(c2)
 *	ARG_FW_RX_VERDICT_INGRESS:
 *		send(c1) -> verdict skb to rx queuec of c2 -> recv(c2)
 *
 * TX bench(BPF_SK_MSG_VERDIC):
 *	ARG_FW_TX_PASS:
 *		send(p2) -> bpf msg passthrough -> send(p2) -> recv(c2)
 *	ARG_FW_TX_VERDICT_INGRESS:
 *		send(p2) -> verdict msg to rx queue of c2 -> recv(c2)
 *	ARG_FW_TX_VERDICT_EGRESS:
 *		send(p1) -> verdict msg to tx queue of p2 -> recv(c2)
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum SOCKMAP_ARG_FLAG {
    ARG_FW_RX_NORMAL = 11000,
    ARG_FW_RX_PASS,
    ARG_FW_RX_VERDICT_EGRESS,
    ARG_FW_RX_VERDICT_INGRESS,
    ARG_FW_TX_NORMAL,
    ARG_FW_TX_PASS,
    ARG_FW_TX_VERDICT_INGRESS,
    ARG_FW_TX_VERDICT_EGRESS,
    ARG_CTL_RX_STRP,
    ARG_CONSUMER_DELAY_TIME,
    ARG_PRODUCER_DURATION,
}

#[inline]
unsafe fn TXMODE_NORMAL() -> bool {
    CTX.mode == SOCKMAP_ARG_FLAG::ARG_FW_TX_NORMAL
}

#[inline]
unsafe fn TXMODE_BPF_INGRESS() -> bool {
    CTX.mode == SOCKMAP_ARG_FLAG::ARG_FW_TX_VERDICT_INGRESS
}

#[inline]
unsafe fn TXMODE_BPF_EGRESS() -> bool {
    CTX.mode == SOCKMAP_ARG_FLAG::ARG_FW_TX_VERDICT_EGRESS
}

#[inline]
unsafe fn TXMODE_BPF_PASS() -> bool {
    CTX.mode == SOCKMAP_ARG_FLAG::ARG_FW_TX_PASS
}

#[inline]
unsafe fn TXMODE_BPF() -> bool {
    TXMODE_BPF_PASS() || TXMODE_BPF_INGRESS() || TXMODE_BPF_EGRESS()
}

#[inline]
unsafe fn TXMODE() -> bool {
    TXMODE_NORMAL() || TXMODE_BPF()
}

#[inline]
unsafe fn RXMODE_NORMAL() -> bool {
    CTX.mode == SOCKMAP_ARG_FLAG::ARG_FW_RX_NORMAL
}

#[inline]
unsafe fn RXMODE_BPF_PASS() -> bool {
    CTX.mode == SOCKMAP_ARG_FLAG::ARG_FW_RX_PASS
}

#[inline]
unsafe fn RXMODE_BPF_VERDICT_EGRESS() -> bool {
    CTX.mode == SOCKMAP_ARG_FLAG::ARG_FW_RX_VERDICT_EGRESS
}

#[inline]
unsafe fn RXMODE_BPF_VERDICT_INGRESS() -> bool {
    CTX.mode == SOCKMAP_ARG_FLAG::ARG_FW_RX_VERDICT_INGRESS
}

#[inline]
unsafe fn RXMODE_BPF_VERDICT() -> bool {
    RXMODE_BPF_VERDICT_INGRESS() || RXMODE_BPF_VERDICT_EGRESS()
}

#[inline]
unsafe fn RXMODE_BPF() -> bool {
    RXMODE_BPF_PASS() || RXMODE_BPF_VERDICT()
}

#[inline]
unsafe fn RXMODE() -> bool {
    RXMODE_NORMAL() || RXMODE_BPF()
}

#[repr(C)]
struct socmap_ctx {
    skel: *mut bench_sockmap_prog,
    mode: SOCKMAP_ARG_FLAG,
    fds: [c_int; 5],
    send_calls: c_long,
    read_calls: c_long,
    prod_send: c_long,
    user_read: c_long,
    file_size: c_int,
    delay_consumer: c_int,
    prod_run_time: c_int,
    strp_size: c_int,
}

static mut CTX: socmap_ctx = socmap_ctx {
    skel: ptr::null_mut(),
    mode: SOCKMAP_ARG_FLAG::ARG_FW_RX_VERDICT_EGRESS,
    fds: [0; 5],
    send_calls: 0,
    read_calls: 0,
    prod_send: 0,
    user_read: 0,
    file_size: FILE_SIZE,
    delay_consumer: 0,
    prod_run_time: 0,
    strp_size: 0,
};

#[inline]
unsafe fn c1() -> *mut c_int {
    CTX.fds.as_mut_ptr().add(0)
}
#[inline]
unsafe fn p1() -> *mut c_int {
    CTX.fds.as_mut_ptr().add(1)
}
#[inline]
unsafe fn c2() -> *mut c_int {
    CTX.fds.as_mut_ptr().add(2)
}
#[inline]
unsafe fn p2() -> *mut c_int {
    CTX.fds.as_mut_ptr().add(3)
}
#[inline]
unsafe fn sfd() -> *mut c_int {
    CTX.fds.as_mut_ptr().add(4)
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr_storage {
    ss_family: u16,
    __ss_padding: [u8; 118],
    __ss_align: u64,
}

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
struct FILE {
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
struct bench_sockmap_prog_progs {
    prog_skb_parser: *mut bpf_program,
    prog_skb_verdict: *mut bpf_program,
    prog_skb_pass: *mut bpf_program,
    prog_skmsg_pass: *mut bpf_program,
    prog_skmsg_verdict: *mut bpf_program,
}

#[repr(C)]
struct bench_sockmap_prog_maps {
    sock_map_rx: *mut bpf_map,
    sock_map_tx: *mut bpf_map,
}

#[repr(C)]
struct bench_sockmap_prog_bss {
    pkt_size: c_int,
    verdict_dir: c_int,
    process_byte: c_long,
}

#[repr(C)]
struct bench_sockmap_prog {
    progs: bench_sockmap_prog_progs,
    maps: bench_sockmap_prog_maps,
    bss: *mut bench_sockmap_prog_bss,
}

#[repr(C)]
struct bench_env {
    consumer_cnt: c_int,
    producer_cnt: c_int,
    affinity: bool,
}

#[repr(C)]
struct bench_res {
    drops: c_long,
    hits: c_long,
    false_hits: c_long,
    important_hits: c_long,
}

#[repr(C)]
struct argp_option {
    name: *const c_char,
    key: c_int,
    arg: *const c_char,
    flags: c_int,
    doc: *const c_char,
    group: c_int,
}

#[repr(C)]
struct argp_state {
    _private: [u8; 0],
}

#[repr(C)]
struct argp {
    options: *const argp_option,
    parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> error_t>,
}

#[repr(C)]
struct bench {
    name: *const c_char,
    argp: *const argp,
    validate: Option<unsafe extern "C" fn()>,
    setup: Option<unsafe extern "C" fn()>,
    producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    consumer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    report_progress: Option<unsafe extern "C" fn(c_int, *mut bench_res, c_long)>,
    report_final: Option<unsafe extern "C" fn(*mut bench_res, c_int)>,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut env: bench_env;

    fn close(fd: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn htonl(hostlong: u32) -> u32;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    static mut stderr: *mut FILE;
    fn exit(status: c_int) -> !;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, typ: c_int, flags: c_uint) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bench_sockmap_prog__open_and_load() -> *mut bench_sockmap_prog;
    fn bench_sockmap_prog__destroy(obj: *mut bench_sockmap_prog);
    fn atomic_swap(ptr: *mut c_long, val: c_long) -> c_long;
    fn atomic_inc(ptr: *mut c_long);
    fn atomic_add(ptr: *mut c_long, val: c_int);
    fn malloc(size: size_t) -> *mut c_void;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn sleep(seconds: c_uint) -> c_uint;
    fn tmpfile() -> *mut FILE;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    fn fileno(stream: *mut FILE) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn sendfile(out_fd: c_int, in_fd: c_int, offset: *mut off_t, count: size_t) -> ssize_t;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
}

unsafe extern "C" fn bench_sockmap_prog_destroy() {
    let mut i: usize = 0;

    while i < CTX.fds.len() {
        if CTX.fds[i] > 0 {
            close(CTX.fds[i]);
        }
        i += 1;
    }

    bench_sockmap_prog__destroy(CTX.skel);
}

unsafe extern "C" fn init_addr(ss: *mut sockaddr_storage, len: *mut socklen_t) {
    let addr4 = memset(
        ss as *mut c_void,
        0,
        size_of::<sockaddr_storage>(),
    ) as *mut sockaddr_in;

    (*addr4).sin_family = AF_INET as u16;
    (*addr4).sin_port = 0;
    (*addr4).sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    *len = size_of::<sockaddr_in>() as socklen_t;
}

unsafe extern "C" fn set_non_block(fd: c_int, blocking: bool) -> bool {
    let mut flags = fcntl(fd, F_GETFL, 0);

    if flags == -1 {
        return false;
    }
    flags = if blocking {
        flags | O_NONBLOCK
    } else {
        flags & !O_NONBLOCK
    };
    fcntl(fd, F_SETFL, flags) == 0
}

unsafe extern "C" fn create_pair(c: *mut c_int, p: *mut c_int, typ: c_int) -> c_int {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut err: c_int;
    let cfd: c_int;
    let pfd: c_int;
    let mut addr_len: socklen_t = size_of::<sockaddr_storage>() as socklen_t;

    err = getsockname(*sfd(), &mut addr as *mut _ as *mut sockaddr, &mut addr_len);
    if err != 0 {
        fprintf(stderr, c"getsockname error %d\n".as_ptr(), errno);
        return err;
    }
    cfd = socket(AF_INET, typ, 0);
    if cfd < 0 {
        fprintf(stderr, c"socket error %d\n".as_ptr(), errno);
        return err;
    }

    err = connect(cfd, &addr as *const _ as *const sockaddr, addr_len);
    if err != 0 && errno != EINPROGRESS {
        fprintf(stderr, c"connect error %d\n".as_ptr(), errno);
        return err;
    }

    pfd = accept(*sfd(), ptr::null_mut(), ptr::null_mut());
    if pfd < 0 {
        fprintf(stderr, c"accept error %d\n".as_ptr(), errno);
        return err;
    }
    *c = cfd;
    *p = pfd;
    0
}

unsafe extern "C" fn create_sockets() -> c_int {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut err: c_int;
    let one: c_int = 1;
    let mut addr_len: socklen_t = 0;

    init_addr(&mut addr, &mut addr_len);
    *sfd() = socket(AF_INET, SOCK_STREAM, 0);
    if *sfd() < 0 {
        fprintf(stderr, c"socket error:%d\n".as_ptr(), errno);
        return *sfd();
    }
    err = setsockopt(
        *sfd(),
        SOL_SOCKET,
        SO_REUSEPORT,
        &one as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    );
    if err != 0 {
        fprintf(stderr, c"setsockopt error:%d\n".as_ptr(), errno);
        return err;
    }

    err = bind(*sfd(), &addr as *const _ as *const sockaddr, addr_len);
    if err != 0 {
        fprintf(stderr, c"bind error:%d\n".as_ptr(), errno);
        return err;
    }

    err = listen(*sfd(), SOMAXCONN);
    if err != 0 {
        fprintf(stderr, c"listen error:%d\n".as_ptr(), errno);
        return err;
    }

    err = create_pair(c1(), p1(), SOCK_STREAM);
    if err != 0 {
        fprintf(stderr, c"create_pair 1 error\n".as_ptr());
        return err;
    }

    err = create_pair(c2(), p2(), SOCK_STREAM);
    if err != 0 {
        fprintf(stderr, c"create_pair 2 error\n".as_ptr());
        return err;
    }
    printf(
        c"create socket fd c1:%d p1:%d c2:%d p2:%d\n".as_ptr(),
        *c1(),
        *p1(),
        *c2(),
        *p2(),
    );
    0
}

unsafe extern "C" fn validate() {
    if env.consumer_cnt != 2 || env.producer_cnt != 1 || !env.affinity {
        fprintf(stderr, c"argument '-c 2 -p 1 -a' is necessary".as_ptr());
        exit(1);
    }
}

unsafe extern "C" fn setup_rx_sockmap() -> c_int {
    let verdict: c_int;
    let pass: c_int;
    let parser: c_int;
    let map: c_int;
    let zero: c_int = 0;
    let one: c_int = 1;
    let mut err: c_int = 0;

    parser = bpf_program__fd((*CTX.skel).progs.prog_skb_parser);
    verdict = bpf_program__fd((*CTX.skel).progs.prog_skb_verdict);
    pass = bpf_program__fd((*CTX.skel).progs.prog_skb_pass);
    map = bpf_map__fd((*CTX.skel).maps.sock_map_rx);

    if CTX.strp_size != 0 {
        (*(*CTX.skel).bss).pkt_size = CTX.strp_size;
        err = bpf_prog_attach(parser, map, BPF_SK_SKB_STREAM_PARSER, 0);
        if err != 0 {
            return err;
        }
    }

    if RXMODE_BPF_VERDICT() {
        err = bpf_prog_attach(verdict, map, BPF_SK_SKB_STREAM_VERDICT, 0);
    } else if RXMODE_BPF_PASS() {
        err = bpf_prog_attach(pass, map, BPF_SK_SKB_STREAM_VERDICT, 0);
    }
    if err != 0 {
        return err;
    }

    if RXMODE_BPF_PASS() {
        return bpf_map_update_elem(
            map,
            &zero as *const _ as *const c_void,
            c2() as *const c_void,
            BPF_NOEXIST,
        );
    }

    err = bpf_map_update_elem(
        map,
        &zero as *const _ as *const c_void,
        p1() as *const c_void,
        BPF_NOEXIST,
    );
    if err < 0 {
        return err;
    }

    if RXMODE_BPF_VERDICT_INGRESS() {
        (*(*CTX.skel).bss).verdict_dir = BPF_F_INGRESS;
        err = bpf_map_update_elem(
            map,
            &one as *const _ as *const c_void,
            c2() as *const c_void,
            BPF_NOEXIST,
        );
    } else {
        err = bpf_map_update_elem(
            map,
            &one as *const _ as *const c_void,
            p2() as *const c_void,
            BPF_NOEXIST,
        );
    }
    if err < 0 {
        return err;
    }

    0
}

unsafe extern "C" fn setup_tx_sockmap() -> c_int {
    let zero: c_int = 0;
    let one: c_int = 1;
    let prog: c_int;
    let map: c_int;
    let mut err: c_int;

    map = bpf_map__fd((*CTX.skel).maps.sock_map_tx);
    prog = if TXMODE_BPF_PASS() {
        bpf_program__fd((*CTX.skel).progs.prog_skmsg_pass)
    } else {
        bpf_program__fd((*CTX.skel).progs.prog_skmsg_verdict)
    };

    err = bpf_prog_attach(prog, map, BPF_SK_MSG_VERDICT, 0);
    if err != 0 {
        return err;
    }

    if TXMODE_BPF_EGRESS() {
        err = bpf_map_update_elem(
            map,
            &zero as *const _ as *const c_void,
            p1() as *const c_void,
            BPF_NOEXIST,
        );
        err |= bpf_map_update_elem(
            map,
            &one as *const _ as *const c_void,
            p2() as *const c_void,
            BPF_NOEXIST,
        );
    } else {
        (*(*CTX.skel).bss).verdict_dir = BPF_F_INGRESS;
        err = bpf_map_update_elem(
            map,
            &zero as *const _ as *const c_void,
            p2() as *const c_void,
            BPF_NOEXIST,
        );
        err |= bpf_map_update_elem(
            map,
            &one as *const _ as *const c_void,
            c2() as *const c_void,
            BPF_NOEXIST,
        );
    }

    if err < 0 {
        return err;
    }

    0
}

unsafe extern "C" fn setup() {
    let mut err: c_int;

    CTX.skel = bench_sockmap_prog__open_and_load();
    if CTX.skel.is_null() {
        fprintf(stderr, c"error loading skel\n".as_ptr());
        exit(1);
    }

    if create_sockets() != 0 {
        fprintf(stderr, c"create_net_mode error\n".as_ptr());
        bench_sockmap_prog_destroy();
        exit(1);
    }

    if RXMODE_BPF() {
        err = setup_rx_sockmap();
        if err != 0 {
            fprintf(stderr, c"setup_rx_sockmap error:%d\n".as_ptr(), err);
            bench_sockmap_prog_destroy();
            exit(1);
        }
    } else if TXMODE_BPF() {
        err = setup_tx_sockmap();
        if err != 0 {
            fprintf(stderr, c"setup_tx_sockmap error:%d\n".as_ptr(), err);
            bench_sockmap_prog_destroy();
            exit(1);
        }
    } else {
        fprintf(stderr, c"unknown sockmap bench mode: %d\n".as_ptr(), CTX.mode as c_int);
        bench_sockmap_prog_destroy();
        exit(1);
    }
}

unsafe extern "C" fn measure(res: *mut bench_res) {
    (*res).drops = atomic_swap(&mut CTX.prod_send, 0);
    (*res).hits = atomic_swap(&mut (*(*CTX.skel).bss).process_byte, 0);
    (*res).false_hits = atomic_swap(&mut CTX.user_read, 0);
    (*res).important_hits = atomic_swap(&mut CTX.send_calls, 0);
    (*res).important_hits |= atomic_swap(&mut CTX.read_calls, 0) << 32;
}

unsafe extern "C" fn verify_data(check_pos: *mut c_int, buf: *mut c_char, rcv: c_int) {
    let mut i: c_int = 0;
    while i < rcv {
        if *buf.add(i as usize) != SND_DATA[(*check_pos as usize) % DATA_REPEAT_SIZE] {
            fprintf(stderr, c"verify data fail".as_ptr());
            exit(1);
        }
        *check_pos += 1;
        if *check_pos >= FILE_SIZE {
            *check_pos = 0;
        }
        i += 1;
    }
}

unsafe extern "C" fn consumer(input: *mut c_void) -> *mut c_void {
    let mut rcv: c_int;
    let sent: c_int;
    let mut check_pos: c_int = 0;
    let tid: c_int = input as c_long as c_int;
    let recv_buf_size: c_int = FILE_SIZE;
    let buf = malloc(recv_buf_size as size_t) as *mut c_char;
    let mut delay_read: c_int = CTX.delay_consumer;

    if buf.is_null() {
        fprintf(stderr, c"fail to init read buffer".as_ptr());
        return ptr::null_mut();
    }

    loop {
        if tid == 1 {
            /* consumer 1 is unused for tx test and stream verdict test */
            if RXMODE_BPF() || TXMODE() {
                return ptr::null_mut();
            }
            /* it's only for RX_NORMAL which service as reserve-proxy mode */
            rcv = read(*p1(), buf as *mut c_void, recv_buf_size as size_t) as c_int;
            if rcv < 0 {
                fprintf(stderr, c"fail to read p1".as_ptr());
                return ptr::null_mut();
            }

            sent = send(*p2(), buf as *const c_void, recv_buf_size as size_t, 0) as c_int;
            if sent < 0 {
                fprintf(stderr, c"fail to send p2".as_ptr());
                return ptr::null_mut();
            }
        } else {
            if delay_read != 0 {
                if delay_read < 0 {
                    return ptr::null_mut();
                }
                sleep(delay_read as c_uint);
                delay_read = 0;
            }
            /* read real endpoint by consumer 0 */
            atomic_inc(&mut CTX.read_calls);
            rcv = read(*c2(), buf as *mut c_void, recv_buf_size as size_t) as c_int;
            if rcv < 0 && errno != EAGAIN {
                fprintf(stderr, c"%s fail to read c2 %d\n".as_ptr(), c"consumer".as_ptr(), errno);
                return ptr::null_mut();
            }
            verify_data(&mut check_pos, buf, rcv);
            atomic_add(&mut CTX.user_read, rcv);
        }
    }
}

unsafe extern "C" fn producer(_input: *mut c_void) -> *mut c_void {
    let mut off: c_int = 0;
    let fp: c_int;
    let mut need_sent: c_int;
    let mut sent: c_int;
    let file_size: c_int = CTX.file_size;
    let mut ts1: timespec = core::mem::zeroed();
    let mut ts2: timespec = core::mem::zeroed();
    let target: c_int;
    let file: *mut FILE;

    file = tmpfile();
    if file.is_null() {
        fprintf(stderr, c"create file for sendfile".as_ptr());
        return ptr::null_mut();
    }

    /* we need simple verify */
    let mut i: c_int = 0;
    while i < file_size {
        if fwrite(
            SND_DATA.as_ptr().add(off as usize) as *const c_void,
            size_of::<c_char>(),
            1,
            file,
        ) != 1
        {
            fprintf(stderr, c"init tmpfile error".as_ptr());
            return ptr::null_mut();
        }
        off += 1;
        if off as usize >= SND_DATA.len() {
            off = 0;
        }
        i += 1;
    }
    fflush(file);
    fseek(file, 0, SEEK_SET);

    fp = fileno(file);
    need_sent = file_size;
    clock_gettime(CLOCK_MONOTONIC, &mut ts1);

    if RXMODE_BPF_VERDICT() {
        target = *c1();
    } else if TXMODE_BPF_EGRESS() {
        target = *p1();
    } else {
        target = *p2();
    }
    set_non_block(target, true);
    loop {
        if CTX.prod_run_time != 0 {
            clock_gettime(CLOCK_MONOTONIC, &mut ts2);
            if ts2.tv_sec - ts1.tv_sec > CTX.prod_run_time as time_t {
                return ptr::null_mut();
            }
        }

        errno = 0;
        atomic_inc(&mut CTX.send_calls);
        sent = sendfile(target, fp, ptr::null_mut(), need_sent as size_t) as c_int;
        if sent < 0 {
            if errno != EAGAIN && errno != ENOMEM && errno != ENOBUFS {
                fprintf(
                    stderr,
                    c"sendfile return %d, errorno %d:%s\n".as_ptr(),
                    sent,
                    errno,
                    strerror(errno),
                );
                return ptr::null_mut();
            }
            continue;
        } else if sent < need_sent {
            need_sent -= sent;
            atomic_add(&mut CTX.prod_send, sent);
            continue;
        }
        atomic_add(&mut CTX.prod_send, need_sent);
        need_sent = file_size;
        lseek(fp, 0, SEEK_SET);
    }
}

unsafe extern "C" fn report_progress(iter: c_int, res: *mut bench_res, delta_ns: c_long) {
    let speed_mbs: f64;
    let prod_mbs: f64;
    let bpf_mbs: f64;
    let send_hz: f64;
    let read_hz: f64;

    prod_mbs = (*res).drops as f64 / 1000000.0 / (delta_ns as f64 / 1000000000.0);
    speed_mbs = (*res).false_hits as f64 / 1000000.0 / (delta_ns as f64 / 1000000000.0);
    bpf_mbs = (*res).hits as f64 / 1000000.0 / (delta_ns as f64 / 1000000000.0);
    send_hz = ((*res).important_hits & 0xFFFFFFFF) as f64 / (delta_ns as f64 / 1000000000.0);
    read_hz = ((*res).important_hits >> 32) as f64 / (delta_ns as f64 / 1000000000.0);

    printf(
        c"Iter %3d (%7.3lfus): ".as_ptr(),
        iter,
        (delta_ns - 1000000000) as f64 / 1000.0,
    );
    printf(
        c"Send Speed %8.3lf MB/s (%8.3lf calls/s), BPF Speed %8.3lf MB/s, Rcv Speed %8.3lf MB/s (%8.3lf calls/s)\n".as_ptr(),
        prod_mbs,
        send_hz,
        bpf_mbs,
        speed_mbs,
        read_hz,
    );
}

unsafe extern "C" fn report_final(res: *mut bench_res, res_cnt: c_int) {
    let mut verdict_mbs_mean: f64 = 0.0;
    let mut verdict_total: c_long = 0;
    let mut i: c_int = 0;

    while i < res_cnt {
        verdict_mbs_mean += (*res.add(i as usize)).hits as f64 / 1000000.0 / (0.0 + res_cnt as f64);
        verdict_total += ((*res.add(i as usize)).hits as f64 / 1000000.0) as c_long;
        i += 1;
    }

    printf(
        "Summary: total trans %8.3lu MB ± %5.3lf MB/s\n\0".as_ptr() as *const c_char,
        verdict_total,
        verdict_mbs_mean,
    );
}

static OPTS: [argp_option; 12] = [
    argp_option {
        name: c"rx-normal".as_ptr(),
        key: SOCKMAP_ARG_FLAG::ARG_FW_RX_NORMAL as c_int,
        arg: ptr::null(),
        flags: 0,
        doc: c"simple reserve-proxy mode, no bfp enabled".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"rx-pass".as_ptr(),
        key: SOCKMAP_ARG_FLAG::ARG_FW_RX_PASS as c_int,
        arg: ptr::null(),
        flags: 0,
        doc: c"run bpf prog but no redir applied".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"rx-strp".as_ptr(),
        key: SOCKMAP_ARG_FLAG::ARG_CTL_RX_STRP as c_int,
        arg: c"Byte".as_ptr(),
        flags: 0,
        doc: c"enable strparser and set the encapsulation size".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"rx-verdict-egress".as_ptr(),
        key: SOCKMAP_ARG_FLAG::ARG_FW_RX_VERDICT_EGRESS as c_int,
        arg: ptr::null(),
        flags: 0,
        doc: c"forward data with bpf(stream verdict)".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"rx-verdict-ingress".as_ptr(),
        key: SOCKMAP_ARG_FLAG::ARG_FW_RX_VERDICT_INGRESS as c_int,
        arg: ptr::null(),
        flags: 0,
        doc: c"forward data with bpf(stream verdict)".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"tx-normal".as_ptr(),
        key: SOCKMAP_ARG_FLAG::ARG_FW_TX_NORMAL as c_int,
        arg: ptr::null(),
        flags: 0,
        doc: c"simple c-s mode, no bfp enabled".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"tx-pass".as_ptr(),
        key: SOCKMAP_ARG_FLAG::ARG_FW_TX_PASS as c_int,
        arg: ptr::null(),
        flags: 0,
        doc: c"run bpf prog but no redir applied".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"tx-verdict-ingress".as_ptr(),
        key: SOCKMAP_ARG_FLAG::ARG_FW_TX_VERDICT_INGRESS as c_int,
        arg: ptr::null(),
        flags: 0,
        doc: c"forward msg to ingress queue of another socket".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"tx-verdict-egress".as_ptr(),
        key: SOCKMAP_ARG_FLAG::ARG_FW_TX_VERDICT_EGRESS as c_int,
        arg: ptr::null(),
        flags: 0,
        doc: c"forward msg to egress queue of another socket".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"delay-consumer".as_ptr(),
        key: SOCKMAP_ARG_FLAG::ARG_CONSUMER_DELAY_TIME as c_int,
        arg: c"SEC".as_ptr(),
        flags: 0,
        doc: c"delay consumer start".as_ptr(),
        group: 0,
    },
    argp_option {
        name: c"producer-duration".as_ptr(),
        key: SOCKMAP_ARG_FLAG::ARG_PRODUCER_DURATION as c_int,
        arg: c"SEC".as_ptr(),
        flags: 0,
        doc: c"producer duration".as_ptr(),
        group: 0,
    },
    argp_option {
        name: ptr::null(),
        key: 0,
        arg: ptr::null(),
        flags: 0,
        doc: ptr::null(),
        group: 0,
    },
];

unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, _state: *mut argp_state) -> error_t {
    match key {
        x if x >= SOCKMAP_ARG_FLAG::ARG_FW_RX_NORMAL as c_int
            && x <= SOCKMAP_ARG_FLAG::ARG_FW_TX_VERDICT_EGRESS as c_int =>
        {
            CTX.mode = core::mem::transmute::<c_int, SOCKMAP_ARG_FLAG>(key);
        }
        x if x == SOCKMAP_ARG_FLAG::ARG_CONSUMER_DELAY_TIME as c_int => {
            CTX.delay_consumer = strtol(arg, ptr::null_mut(), 10) as c_int;
        }
        x if x == SOCKMAP_ARG_FLAG::ARG_PRODUCER_DURATION as c_int => {
            CTX.prod_run_time = strtol(arg, ptr::null_mut(), 10) as c_int;
        }
        x if x == SOCKMAP_ARG_FLAG::ARG_CTL_RX_STRP as c_int => {
            CTX.strp_size = strtol(arg, ptr::null_mut(), 10) as c_int;
        }
        _ => {
            return ARGP_ERR_UNKNOWN;
        }
    }

    0
}

/* exported into benchmark runner */
#[unsafe(no_mangle)]
pub static bench_sockmap_argp: argp = argp {
    options: OPTS.as_ptr(),
    parser: Some(parse_arg),
};

/* Benchmark performance of creating bpf local storage  */
#[unsafe(no_mangle)]
pub static bench_sockmap: bench = bench {
    name: c"sockmap".as_ptr(),
    argp: &bench_sockmap_argp,
    validate: Some(validate),
    setup: Some(setup),
    producer_thread: Some(producer),
    consumer_thread: Some(consumer),
    measure: Some(measure),
    report_progress: Some(report_progress),
    report_final: Some(report_final),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
