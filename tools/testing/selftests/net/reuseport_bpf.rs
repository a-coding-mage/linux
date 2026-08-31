/*
 * Test functionality of BPF filters for SO_REUSEPORT.  The tests below will use
 * a BPF program (both classic and extended) to read the first word from an
 * incoming packet (expected to be in network byte-order), calculate a modulus
 * of that number, and then dispatch the packet to the Nth socket using the
 * result.  These tests are run for each supported address family and protocol.
 * Additionally, a few edge cases in the implementation are tested.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, size_of_val, zeroed};
use core::ptr::null_mut;

type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type sa_family_t = u16;
type uint16_t = u16;
type uint32_t = u32;
type rlim_t = c_ulong;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SOL_TCP: c_int = 6;
const SO_REUSEADDR: c_int = 2;
const SO_REUSEPORT: c_int = 15;
const SO_ATTACH_REUSEPORT_CBPF: c_int = 51;
const SO_ATTACH_REUSEPORT_EBPF: c_int = 52;
const TCP_FASTOPEN: c_int = 23;
const MSG_FASTOPEN: c_int = 0x20000000;
const EPOLLIN: u32 = 0x001;
const EPOLL_CTL_ADD: c_int = 1;
const EADDRINUSE: c_int = 98;
const EINVAL: c_int = 22;
const O_RDWR: c_int = 0o2;
const RLIMIT_MEMLOCK: c_int = 8;
const RLIM_INFINITY: rlim_t = !0;
const CLONE_NEWNET: c_int = 0x40000000;
const INADDR_ANY: u32 = 0;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const __NR_BPF: c_long = 321;

const BPF_PROG_LOAD: c_int = 5;
const BPF_PROG_TYPE_SOCKET_FILTER: u32 = 1;

const BPF_LD: u16 = 0x00;
const BPF_LDX: u16 = 0x01;
const BPF_ST: u16 = 0x02;
const BPF_STX: u16 = 0x03;
const BPF_ALU: u16 = 0x04;
const BPF_JMP: u16 = 0x05;
const BPF_RET: u16 = 0x06;
const BPF_MISC: u16 = 0x07;
const BPF_W: u16 = 0x00;
const BPF_ABS: u16 = 0x20;
const BPF_ALU64: u16 = 0x07;
const BPF_MOV: u16 = 0xb0;
const BPF_X: u16 = 0x08;
const BPF_K: u16 = 0x00;
const BPF_MOD: u16 = 0x90;
const BPF_EXIT: u16 = 0x90;
const BPF_A: u16 = 0x10;
const BPF_REG_0: u8 = 0;
const BPF_REG_1: u8 = 1;
const BPF_REG_6: u8 = 6;

#[repr(C)]
#[derive(Copy, Clone)]
struct test_params {
    recv_family: c_int,
    send_family: c_int,
    protocol: c_int,
    recv_socks: size_t,
    recv_port: uint16_t,
    send_port_min: uint16_t,
}

#[repr(C)]
struct sockaddr {
    sa_family: sa_family_t,
    sa_data: [u8; 14],
}

#[repr(C)]
struct in_addr {
    s_addr: uint32_t,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: sa_family_t,
    sin_port: uint16_t,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: sa_family_t,
    sin6_port: uint16_t,
    sin6_flowinfo: uint32_t,
    sin6_addr: in6_addr,
    sin6_scope_id: uint32_t,
}

#[repr(C, align(8))]
struct sockaddr_storage {
    data: [u8; 128],
}

#[repr(C)]
struct bpf_insn {
    code: u8,
    dst_src: u8,
    off: i16,
    imm: i32,
}

impl bpf_insn {
    const fn new(code: u16, dst_reg: u8, src_reg: u8, off: i16, imm: i32) -> bpf_insn {
        bpf_insn {
            code: code as u8,
            dst_src: (dst_reg & 0x0f) | ((src_reg & 0x0f) << 4),
            off,
            imm,
        }
    }
}

#[repr(C)]
struct bpf_attr_prog_load {
    prog_type: u32,
    insn_cnt: u32,
    insns: u64,
    license: u64,
    log_level: u32,
    log_size: u32,
    log_buf: u64,
    kern_version: u32,
}

#[repr(C)]
union bpf_attr {
    prog_load: bpf_attr_prog_load,
    pad: [u8; 128],
}

#[repr(C)]
struct sock_filter {
    code: u16,
    jt: u8,
    jf: u8,
    k: u32,
}

#[repr(C)]
struct sock_fprog {
    len: u16,
    filter: *mut sock_filter,
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: socklen_t,
    msg_iov: *mut iovec,
    msg_iovlen: size_t,
    msg_control: *mut c_void,
    msg_controllen: size_t,
    msg_flags: c_int,
}

#[repr(C)]
union epoll_data {
    ptr: *mut c_void,
    fd: c_int,
    u32_: u32,
    u64_: u64,
}

#[repr(C, packed)]
struct epoll_event {
    events: u32,
    data: epoll_data,
}

#[repr(C)]
struct rlimit {
    rlim_cur: rlim_t,
    rlim_max: rlim_t,
}

static mut RLIM_OLD: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
};

unsafe extern "C" {
    static in6addr_any: in6_addr;
    static in6addr_loopback: in6_addr;

    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn htonl(hostlong: uint32_t) -> uint32_t;
    fn htons(hostshort: uint16_t) -> uint16_t;
    fn ntohl(netlong: uint32_t) -> uint32_t;
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
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn epoll_create(size: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(
        epfd: c_int,
        events: *mut epoll_event,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn atoi(nptr: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...) -> !;
    fn __errno_location() -> *mut c_int;

    static mut stderr: *mut c_void;
}

unsafe fn errno_value() -> c_int {
    *__errno_location()
}

unsafe fn set_errno_value(value: c_int) {
    *__errno_location() = value;
}

fn sockaddr_size() -> size_t {
    size_of::<sockaddr_storage>()
}

unsafe fn new_any_sockaddr(family: c_int, port: uint16_t) -> *mut sockaddr {
    let addr = malloc(size_of::<sockaddr_storage>()) as *mut sockaddr_storage;
    memset(addr as *mut c_void, 0, size_of::<sockaddr_storage>());

    match family {
        AF_INET => {
            let addr4 = addr as *mut sockaddr_in;
            (*addr4).sin_family = AF_INET as sa_family_t;
            (*addr4).sin_addr.s_addr = htonl(INADDR_ANY);
            (*addr4).sin_port = htons(port);
        }
        AF_INET6 => {
            let addr6 = addr as *mut sockaddr_in6;
            (*addr6).sin6_family = AF_INET6 as sa_family_t;
            (*addr6).sin6_addr = in6addr_any;
            (*addr6).sin6_port = htons(port);
        }
        _ => {
            error(
                1,
                0,
                b"Unsupported family %d\0".as_ptr() as *const c_char,
                family,
            );
        }
    }
    addr as *mut sockaddr
}

unsafe fn new_loopback_sockaddr(family: c_int, port: uint16_t) -> *mut sockaddr {
    let addr = new_any_sockaddr(family, port);

    match family {
        AF_INET => {
            let addr4 = addr as *mut sockaddr_in;
            (*addr4).sin_addr.s_addr = htonl(INADDR_LOOPBACK);
        }
        AF_INET6 => {
            let addr6 = addr as *mut sockaddr_in6;
            (*addr6).sin6_addr = in6addr_loopback;
        }
        _ => {
            error(
                1,
                0,
                b"Unsupported family %d\0".as_ptr() as *const c_char,
                family,
            );
        }
    }
    addr
}

unsafe fn attach_ebpf(fd: c_int, mod_: uint16_t) {
    static mut BPF_LOG_BUF: [c_char; 65536] = [0; 65536];
    static BPF_LICENSE: &[u8; 4] = b"GPL\0";

    let prog = [
        /* BPF_MOV64_REG(BPF_REG_6, BPF_REG_1) */
        bpf_insn::new(BPF_ALU64 | BPF_MOV | BPF_X, BPF_REG_6, BPF_REG_1, 0, 0),
        /* BPF_LD_ABS(BPF_W, 0) R0 = (uint32_t)skb[0] */
        bpf_insn::new(BPF_LD | BPF_ABS | BPF_W, 0, 0, 0, 0),
        /* BPF_ALU64_IMM(BPF_MOD, BPF_REG_0, mod) */
        bpf_insn::new(
            BPF_ALU64 | BPF_MOD | BPF_K,
            BPF_REG_0,
            0,
            0,
            mod_ as i32,
        ),
        /* BPF_EXIT_INSN() */
        bpf_insn::new(BPF_JMP | BPF_EXIT, 0, 0, 0, 0),
    ];
    let mut attr: bpf_attr = zeroed();

    attr.prog_load.prog_type = BPF_PROG_TYPE_SOCKET_FILTER;
    attr.prog_load.insn_cnt = prog.len() as u32;
    attr.prog_load.insns = &prog as *const _ as u64;
    attr.prog_load.license = BPF_LICENSE.as_ptr() as u64;
    attr.prog_load.log_buf = core::ptr::addr_of_mut!(BPF_LOG_BUF) as *mut c_char as u64;
    attr.prog_load.log_size = size_of_val(&BPF_LOG_BUF) as u32;
    attr.prog_load.log_level = 1;
    attr.prog_load.kern_version = 0;

    let bpf_fd = syscall(
        __NR_BPF,
        BPF_PROG_LOAD,
        &mut attr as *mut bpf_attr,
        size_of::<bpf_attr>(),
    ) as c_int;
    if bpf_fd < 0 {
        error(
            1,
            errno_value(),
            b"ebpf error. log:\n%s\n\0".as_ptr() as *const c_char,
            core::ptr::addr_of_mut!(BPF_LOG_BUF) as *mut c_char,
        );
    }

    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_ATTACH_REUSEPORT_EBPF,
        &bpf_fd as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        error(
            1,
            errno_value(),
            b"failed to set SO_ATTACH_REUSEPORT_EBPF\0".as_ptr() as *const c_char,
        );
    }

    close(bpf_fd);
}

unsafe fn attach_cbpf(fd: c_int, mod_: uint16_t) {
    let mut code = [
        /* A = (uint32_t)skb[0] */
        sock_filter {
            code: BPF_LD | BPF_W | BPF_ABS,
            jt: 0,
            jf: 0,
            k: 0,
        },
        /* A = A % mod */
        sock_filter {
            code: BPF_ALU | BPF_MOD,
            jt: 0,
            jf: 0,
            k: mod_ as u32,
        },
        /* return A */
        sock_filter {
            code: BPF_RET | BPF_A,
            jt: 0,
            jf: 0,
            k: 0,
        },
    ];
    let mut p = sock_fprog {
        len: code.len() as u16,
        filter: code.as_mut_ptr(),
    };

    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_ATTACH_REUSEPORT_CBPF,
        &mut p as *mut sock_fprog as *const c_void,
        size_of::<sock_fprog>() as socklen_t,
    ) != 0
    {
        error(
            1,
            errno_value(),
            b"failed to set SO_ATTACH_REUSEPORT_CBPF\0".as_ptr() as *const c_char,
        );
    }
}

unsafe fn build_recv_group(
    p: test_params,
    fd: *mut c_int,
    mod_: uint16_t,
    attach_bpf: unsafe fn(c_int, uint16_t),
) {
    let addr = new_any_sockaddr(p.recv_family, p.recv_port);
    let mut i: size_t = 0;
    let mut opt: c_int;

    while i < p.recv_socks {
        *fd.add(i) = socket(p.recv_family, p.protocol, 0);
        if *fd.add(i) < 0 {
            error(
                1,
                errno_value(),
                b"failed to create recv %d\0".as_ptr() as *const c_char,
                i as c_int,
            );
        }

        opt = 1;
        if setsockopt(
            *fd.add(i),
            SOL_SOCKET,
            SO_REUSEPORT,
            &opt as *const c_int as *const c_void,
            size_of::<c_int>() as socklen_t,
        ) != 0
        {
            error(
                1,
                errno_value(),
                b"failed to set SO_REUSEPORT on %d\0".as_ptr() as *const c_char,
                i as c_int,
            );
        }

        if i == 0 {
            attach_bpf(*fd.add(i), mod_);
        }

        if bind(*fd.add(i), addr, sockaddr_size() as socklen_t) != 0 {
            error(
                1,
                errno_value(),
                b"failed to bind recv socket %d\0".as_ptr() as *const c_char,
                i as c_int,
            );
        }

        if p.protocol == SOCK_STREAM {
            opt = 4;
            if setsockopt(
                *fd.add(i),
                SOL_TCP,
                TCP_FASTOPEN,
                &opt as *const c_int as *const c_void,
                size_of::<c_int>() as socklen_t,
            ) != 0
            {
                error(
                    1,
                    errno_value(),
                    b"failed to set TCP_FASTOPEN on %d\0".as_ptr() as *const c_char,
                    i as c_int,
                );
            }
            if listen(*fd.add(i), (p.recv_socks * 10) as c_int) != 0 {
                error(
                    1,
                    errno_value(),
                    b"failed to listen on socket\0".as_ptr() as *const c_char,
                );
            }
        }
        i += 1;
    }
    free(addr as *mut c_void);
}

unsafe fn send_from(p: test_params, sport: uint16_t, buf: *mut c_char, len: size_t) {
    let saddr = new_any_sockaddr(p.send_family, sport);
    let daddr = new_loopback_sockaddr(p.send_family, p.recv_port);
    let fd = socket(p.send_family, p.protocol, 0);
    let one: c_int = 1;

    if fd < 0 {
        error(
            1,
            errno_value(),
            b"failed to create send socket\0".as_ptr() as *const c_char,
        );
    }

    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_REUSEADDR,
        &one as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        error(
            1,
            errno_value(),
            b"failed to set reuseaddr\0".as_ptr() as *const c_char,
        );
    }

    if bind(fd, saddr, sockaddr_size() as socklen_t) != 0 {
        error(
            1,
            errno_value(),
            b"failed to bind send socket\0".as_ptr() as *const c_char,
        );
    }

    if sendto(fd, buf as *const c_void, len, MSG_FASTOPEN, daddr, sockaddr_size() as socklen_t) < 0
    {
        error(
            1,
            errno_value(),
            b"failed to send message\0".as_ptr() as *const c_char,
        );
    }

    close(fd);
    free(saddr as *mut c_void);
    free(daddr as *mut c_void);
}

unsafe fn test_recv_order(p: test_params, fd: *mut c_int, mod_: c_int) {
    let mut recv_buf = [0 as c_char; 8];
    let mut send_buf = [0 as c_char; 8];
    let mut msg: msghdr = zeroed();
    let mut recv_io = iovec {
        iov_base: recv_buf.as_mut_ptr() as *mut c_void,
        iov_len: 8,
    };
    let mut ev: epoll_event = zeroed();
    let mut i: c_int;
    let mut data: uint32_t = 0;
    let mut ndata: uint32_t;

    let epfd = epoll_create(1);
    if epfd < 0 {
        error(
            1,
            errno_value(),
            b"failed to create epoll\0".as_ptr() as *const c_char,
        );
    }
    i = 0;
    while (i as size_t) < p.recv_socks {
        ev.events = EPOLLIN;
        ev.data.fd = *fd.add(i as usize);
        if epoll_ctl(epfd, EPOLL_CTL_ADD, *fd.add(i as usize), &mut ev) != 0 {
            error(
                1,
                errno_value(),
                b"failed to register sock %d epoll\0".as_ptr() as *const c_char,
                i,
            );
        }
        i += 1;
    }

    memset(
        &mut msg as *mut msghdr as *mut c_void,
        0,
        size_of::<msghdr>(),
    );
    msg.msg_iov = &mut recv_io;
    msg.msg_iovlen = 1;

    while data < (p.recv_socks * 2) as uint32_t {
        let sport = p.send_port_min.wrapping_add(data as uint16_t);
        ndata = htonl(data);
        memcpy(
            send_buf.as_mut_ptr() as *mut c_void,
            &ndata as *const uint32_t as *const c_void,
            size_of::<uint32_t>(),
        );
        send_from(p, sport, send_buf.as_mut_ptr(), size_of::<uint32_t>());

        i = epoll_wait(epfd, &mut ev, 1, -1);
        if i < 0 {
            error(
                1,
                errno_value(),
                b"epoll wait failed\0".as_ptr() as *const c_char,
            );
        }

        if p.protocol == SOCK_STREAM {
            let conn = accept(ev.data.fd, null_mut(), null_mut());
            if conn < 0 {
                error(
                    1,
                    errno_value(),
                    b"error accepting\0".as_ptr() as *const c_char,
                );
            }
            i = recvmsg(conn, &mut msg, 0) as c_int;
            close(conn);
        } else {
            i = recvmsg(ev.data.fd, &mut msg, 0) as c_int;
        }
        if i < 0 {
            error(
                1,
                errno_value(),
                b"recvmsg error\0".as_ptr() as *const c_char,
            );
        }
        if i as usize != size_of::<uint32_t>() {
            error(
                1,
                0,
                b"expected size %zd got %d\0".as_ptr() as *const c_char,
                size_of::<uint32_t>(),
                i,
            );
        }

        i = 0;
        while (i as size_t) < p.recv_socks {
            if ev.data.fd == *fd.add(i as usize) {
                break;
            }
            i += 1;
        }
        memcpy(
            &mut ndata as *mut uint32_t as *mut c_void,
            recv_buf.as_ptr() as *const c_void,
            size_of::<uint32_t>(),
        );
        fprintf(
            stderr,
            b"Socket %d: %d\n\0".as_ptr() as *const c_char,
            i,
            ntohl(ndata),
        );

        let expected = (sport as c_int) % mod_;
        if i != expected {
            error(
                1,
                0,
                b"expected socket %d\0".as_ptr() as *const c_char,
                expected,
            );
        }
        data += 1;
    }
}

unsafe fn test_reuseport_ebpf(mut p: test_params) {
    let mut fd = vec![0 as c_int; p.recv_socks];

    fprintf(
        stderr,
        b"Testing EBPF mod %zd...\n\0".as_ptr() as *const c_char,
        p.recv_socks,
    );
    build_recv_group(p, fd.as_mut_ptr(), p.recv_socks as uint16_t, attach_ebpf);
    test_recv_order(p, fd.as_mut_ptr(), p.recv_socks as c_int);

    p.send_port_min = p.send_port_min.wrapping_add((p.recv_socks * 2) as uint16_t);
    fprintf(
        stderr,
        b"Reprograming, testing mod %zd...\n\0".as_ptr() as *const c_char,
        p.recv_socks / 2,
    );
    attach_ebpf(fd[0], (p.recv_socks / 2) as uint16_t);
    test_recv_order(p, fd.as_mut_ptr(), (p.recv_socks / 2) as c_int);

    let mut i: size_t = 0;
    while i < p.recv_socks {
        close(fd[i]);
        i += 1;
    }
}

unsafe fn test_reuseport_cbpf(mut p: test_params) {
    let mut fd = vec![0 as c_int; p.recv_socks];

    fprintf(
        stderr,
        b"Testing CBPF mod %zd...\n\0".as_ptr() as *const c_char,
        p.recv_socks,
    );
    build_recv_group(p, fd.as_mut_ptr(), p.recv_socks as uint16_t, attach_cbpf);
    test_recv_order(p, fd.as_mut_ptr(), p.recv_socks as c_int);

    p.send_port_min = p.send_port_min.wrapping_add((p.recv_socks * 2) as uint16_t);
    fprintf(
        stderr,
        b"Reprograming, testing mod %zd...\n\0".as_ptr() as *const c_char,
        p.recv_socks / 2,
    );
    attach_cbpf(fd[0], (p.recv_socks / 2) as uint16_t);
    test_recv_order(p, fd.as_mut_ptr(), (p.recv_socks / 2) as c_int);

    let mut i: size_t = 0;
    while i < p.recv_socks {
        close(fd[i]);
        i += 1;
    }
}

unsafe fn test_extra_filter(p: test_params) {
    let addr = new_any_sockaddr(p.recv_family, p.recv_port);
    let mut opt: c_int;

    fprintf(
        stderr,
        b"Testing too many filters...\n\0".as_ptr() as *const c_char,
    );
    let fd1 = socket(p.recv_family, p.protocol, 0);
    if fd1 < 0 {
        error(
            1,
            errno_value(),
            b"failed to create socket 1\0".as_ptr() as *const c_char,
        );
    }
    let fd2 = socket(p.recv_family, p.protocol, 0);
    if fd2 < 0 {
        error(
            1,
            errno_value(),
            b"failed to create socket 2\0".as_ptr() as *const c_char,
        );
    }

    opt = 1;
    if setsockopt(
        fd1,
        SOL_SOCKET,
        SO_REUSEPORT,
        &opt as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        error(
            1,
            errno_value(),
            b"failed to set SO_REUSEPORT on socket 1\0".as_ptr() as *const c_char,
        );
    }
    if setsockopt(
        fd2,
        SOL_SOCKET,
        SO_REUSEPORT,
        &opt as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        error(
            1,
            errno_value(),
            b"failed to set SO_REUSEPORT on socket 2\0".as_ptr() as *const c_char,
        );
    }

    attach_ebpf(fd1, 10);
    attach_ebpf(fd2, 10);

    if bind(fd1, addr, sockaddr_size() as socklen_t) != 0 {
        error(
            1,
            errno_value(),
            b"failed to bind recv socket 1\0".as_ptr() as *const c_char,
        );
    }

    if bind(fd2, addr, sockaddr_size() as socklen_t) == 0 || errno_value() != EADDRINUSE {
        error(
            1,
            errno_value(),
            b"bind socket 2 should fail with EADDRINUSE\0".as_ptr() as *const c_char,
        );
    }

    free(addr as *mut c_void);
}

unsafe fn test_filter_no_reuseport(p: test_params) {
    let addr = new_any_sockaddr(p.recv_family, p.recv_port);
    let bpf_license = *b"GPL\0";
    let ecode = [
        bpf_insn::new(BPF_ALU64 | BPF_MOV | BPF_K, BPF_REG_0, 0, 0, 10),
        bpf_insn::new(BPF_JMP | BPF_EXIT, 0, 0, 0, 0),
    ];
    let mut ccode = [sock_filter {
        code: BPF_RET | BPF_A,
        jt: 0,
        jf: 0,
        k: 0,
    }];
    let mut eprog: bpf_attr = zeroed();
    let mut cprog: sock_fprog = zeroed();

    fprintf(
        stderr,
        b"Testing filters on non-SO_REUSEPORT socket...\n\0".as_ptr() as *const c_char,
    );

    eprog.prog_load.prog_type = BPF_PROG_TYPE_SOCKET_FILTER;
    eprog.prog_load.insn_cnt = ecode.len() as u32;
    eprog.prog_load.insns = &ecode as *const _ as u64;
    eprog.prog_load.license = bpf_license.as_ptr() as u64;
    eprog.prog_load.kern_version = 0;

    cprog.len = ccode.len() as u16;
    cprog.filter = ccode.as_mut_ptr();

    let bpf_fd = syscall(
        __NR_BPF,
        BPF_PROG_LOAD,
        &mut eprog as *mut bpf_attr,
        size_of::<bpf_attr>(),
    ) as c_int;
    if bpf_fd < 0 {
        error(1, errno_value(), b"ebpf error\0".as_ptr() as *const c_char);
    }
    let fd = socket(p.recv_family, p.protocol, 0);
    if fd < 0 {
        error(
            1,
            errno_value(),
            b"failed to create socket 1\0".as_ptr() as *const c_char,
        );
    }

    if bind(fd, addr, sockaddr_size() as socklen_t) != 0 {
        error(
            1,
            errno_value(),
            b"failed to bind recv socket 1\0".as_ptr() as *const c_char,
        );
    }

    set_errno_value(0);
    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_ATTACH_REUSEPORT_EBPF,
        &bpf_fd as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) == 0
        || errno_value() != EINVAL
    {
        error(
            1,
            errno_value(),
            b"setsockopt should have returned EINVAL\0".as_ptr() as *const c_char,
        );
    }

    set_errno_value(0);
    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_ATTACH_REUSEPORT_CBPF,
        &cprog as *const sock_fprog as *const c_void,
        size_of::<sock_fprog>() as socklen_t,
    ) == 0
        || errno_value() != EINVAL
    {
        error(
            1,
            errno_value(),
            b"setsockopt should have returned EINVAL\0".as_ptr() as *const c_char,
        );
    }

    free(addr as *mut c_void);
}

unsafe fn test_filter_without_bind() {
    let opt: c_int = 1;

    fprintf(
        stderr,
        b"Testing filter add without bind...\n\0".as_ptr() as *const c_char,
    );
    let fd1 = socket(AF_INET, SOCK_DGRAM, 0);
    if fd1 < 0 {
        error(
            1,
            errno_value(),
            b"failed to create socket 1\0".as_ptr() as *const c_char,
        );
    }
    let fd2 = socket(AF_INET, SOCK_DGRAM, 0);
    if fd2 < 0 {
        error(
            1,
            errno_value(),
            b"failed to create socket 2\0".as_ptr() as *const c_char,
        );
    }
    if setsockopt(
        fd1,
        SOL_SOCKET,
        SO_REUSEPORT,
        &opt as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        error(
            1,
            errno_value(),
            b"failed to set SO_REUSEPORT on socket 1\0".as_ptr() as *const c_char,
        );
    }
    if setsockopt(
        fd2,
        SOL_SOCKET,
        SO_REUSEPORT,
        &opt as *const c_int as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        error(
            1,
            errno_value(),
            b"failed to set SO_REUSEPORT on socket 2\0".as_ptr() as *const c_char,
        );
    }

    attach_ebpf(fd1, 10);
    attach_cbpf(fd2, 10);

    close(fd1);
    close(fd2);
}

unsafe fn enable_fastopen() {
    let mut fd = open(
        b"/proc/sys/net/ipv4/tcp_fastopen\0".as_ptr() as *const c_char,
        0,
    );
    let rw_mask = 3; /* bit 1: client side; bit-2 server side */
    let mut buf = [0 as c_char; 16];

    if fd < 0 {
        error(
            1,
            errno_value(),
            b"Unable to open tcp_fastopen sysctl\0".as_ptr() as *const c_char,
        );
    }
    if read(fd, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf)) <= 0 {
        error(
            1,
            errno_value(),
            b"Unable to read tcp_fastopen sysctl\0".as_ptr() as *const c_char,
        );
    }
    let mut val = atoi(buf.as_ptr());
    close(fd);

    if (val & rw_mask) != rw_mask {
        fd = open(
            b"/proc/sys/net/ipv4/tcp_fastopen\0".as_ptr() as *const c_char,
            O_RDWR,
        );
        if fd < 0 {
            error(
                1,
                errno_value(),
                b"Unable to open tcp_fastopen sysctl for writing\0".as_ptr() as *const c_char,
            );
        }
        val |= rw_mask;
        let size = snprintf(
            buf.as_mut_ptr(),
            16,
            b"%d\0".as_ptr() as *const c_char,
            val,
        );
        if write(fd, buf.as_ptr() as *const c_void, size as size_t) <= 0 {
            error(
                1,
                errno_value(),
                b"Unable to write tcp_fastopen sysctl\0".as_ptr() as *const c_char,
            );
        }
        close(fd);
    }
}

/* C used __attribute__((constructor)) for this function. */
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
static MAIN_CTOR: unsafe extern "C" fn() = main_ctor;

unsafe extern "C" fn main_ctor() {
    getrlimit(RLIMIT_MEMLOCK, core::ptr::addr_of_mut!(RLIM_OLD));

    if RLIM_OLD.rlim_cur != RLIM_INFINITY {
        let mut rlim_new: rlimit = zeroed();

        rlim_new.rlim_cur = RLIM_OLD.rlim_cur.wrapping_add(1usize.wrapping_shl(20) as rlim_t);
        rlim_new.rlim_max = RLIM_OLD.rlim_max.wrapping_add(1usize.wrapping_shl(20) as rlim_t);
        setrlimit(RLIMIT_MEMLOCK, &rlim_new);
    }
}

/* C used __attribute__((destructor)) for this function. */
#[used]
#[cfg_attr(target_os = "linux", link_section = ".fini_array")]
static MAIN_DTOR: unsafe extern "C" fn() = main_dtor;

unsafe extern "C" fn main_dtor() {
    setrlimit(RLIMIT_MEMLOCK, core::ptr::addr_of!(RLIM_OLD));
}

unsafe fn setup_netns() {
    if unshare(CLONE_NEWNET) != 0 {
        error(
            1,
            errno_value(),
            b"failed to unshare netns\0".as_ptr() as *const c_char,
        );
    }
    if system(b"ip link set lo up\0".as_ptr() as *const c_char) != 0 {
        error(
            1,
            0,
            b"failed to bring up lo interface in netns\0".as_ptr() as *const c_char,
        );
    }
}

unsafe fn main_impl() -> c_int {
    setup_netns();

    fprintf(stderr, b"---- IPv4 UDP ----\n\0".as_ptr() as *const c_char);
    /* NOTE: UDP socket lookups traverse a different code path when there
     * are > 10 sockets in a group.  Run the bpf test through both paths.
     */
    test_reuseport_ebpf(test_params {
        recv_family: AF_INET,
        send_family: AF_INET,
        protocol: SOCK_DGRAM,
        recv_socks: 10,
        recv_port: 8000,
        send_port_min: 9000,
    });
    test_reuseport_ebpf(test_params {
        recv_family: AF_INET,
        send_family: AF_INET,
        protocol: SOCK_DGRAM,
        recv_socks: 20,
        recv_port: 8000,
        send_port_min: 9000,
    });
    test_reuseport_cbpf(test_params {
        recv_family: AF_INET,
        send_family: AF_INET,
        protocol: SOCK_DGRAM,
        recv_socks: 10,
        recv_port: 8001,
        send_port_min: 9020,
    });
    test_reuseport_cbpf(test_params {
        recv_family: AF_INET,
        send_family: AF_INET,
        protocol: SOCK_DGRAM,
        recv_socks: 20,
        recv_port: 8001,
        send_port_min: 9020,
    });
    test_extra_filter(test_params {
        recv_family: AF_INET,
        send_family: 0,
        protocol: SOCK_DGRAM,
        recv_socks: 0,
        recv_port: 8002,
        send_port_min: 0,
    });
    test_filter_no_reuseport(test_params {
        recv_family: AF_INET,
        send_family: 0,
        protocol: SOCK_DGRAM,
        recv_socks: 0,
        recv_port: 8008,
        send_port_min: 0,
    });

    fprintf(stderr, b"---- IPv6 UDP ----\n\0".as_ptr() as *const c_char);
    test_reuseport_ebpf(test_params {
        recv_family: AF_INET6,
        send_family: AF_INET6,
        protocol: SOCK_DGRAM,
        recv_socks: 10,
        recv_port: 8003,
        send_port_min: 9040,
    });
    test_reuseport_ebpf(test_params {
        recv_family: AF_INET6,
        send_family: AF_INET6,
        protocol: SOCK_DGRAM,
        recv_socks: 20,
        recv_port: 8003,
        send_port_min: 9040,
    });
    test_reuseport_cbpf(test_params {
        recv_family: AF_INET6,
        send_family: AF_INET6,
        protocol: SOCK_DGRAM,
        recv_socks: 10,
        recv_port: 8004,
        send_port_min: 9060,
    });
    test_reuseport_cbpf(test_params {
        recv_family: AF_INET6,
        send_family: AF_INET6,
        protocol: SOCK_DGRAM,
        recv_socks: 20,
        recv_port: 8004,
        send_port_min: 9060,
    });
    test_extra_filter(test_params {
        recv_family: AF_INET6,
        send_family: 0,
        protocol: SOCK_DGRAM,
        recv_socks: 0,
        recv_port: 8005,
        send_port_min: 0,
    });
    test_filter_no_reuseport(test_params {
        recv_family: AF_INET6,
        send_family: 0,
        protocol: SOCK_DGRAM,
        recv_socks: 0,
        recv_port: 8009,
        send_port_min: 0,
    });

    fprintf(
        stderr,
        b"---- IPv6 UDP w/ mapped IPv4 ----\n\0".as_ptr() as *const c_char,
    );
    test_reuseport_ebpf(test_params {
        recv_family: AF_INET6,
        send_family: AF_INET,
        protocol: SOCK_DGRAM,
        recv_socks: 20,
        recv_port: 8006,
        send_port_min: 9080,
    });
    test_reuseport_ebpf(test_params {
        recv_family: AF_INET6,
        send_family: AF_INET,
        protocol: SOCK_DGRAM,
        recv_socks: 10,
        recv_port: 8006,
        send_port_min: 9080,
    });
    test_reuseport_cbpf(test_params {
        recv_family: AF_INET6,
        send_family: AF_INET,
        protocol: SOCK_DGRAM,
        recv_socks: 10,
        recv_port: 8007,
        send_port_min: 9100,
    });
    test_reuseport_cbpf(test_params {
        recv_family: AF_INET6,
        send_family: AF_INET,
        protocol: SOCK_DGRAM,
        recv_socks: 20,
        recv_port: 8007,
        send_port_min: 9100,
    });

    /* TCP fastopen is required for the TCP tests */
    enable_fastopen();
    fprintf(stderr, b"---- IPv4 TCP ----\n\0".as_ptr() as *const c_char);
    test_reuseport_ebpf(test_params {
        recv_family: AF_INET,
        send_family: AF_INET,
        protocol: SOCK_STREAM,
        recv_socks: 10,
        recv_port: 8008,
        send_port_min: 9120,
    });
    test_reuseport_cbpf(test_params {
        recv_family: AF_INET,
        send_family: AF_INET,
        protocol: SOCK_STREAM,
        recv_socks: 10,
        recv_port: 8009,
        send_port_min: 9160,
    });
    test_extra_filter(test_params {
        recv_family: AF_INET,
        send_family: 0,
        protocol: SOCK_STREAM,
        recv_socks: 0,
        recv_port: 8010,
        send_port_min: 0,
    });
    test_filter_no_reuseport(test_params {
        recv_family: AF_INET,
        send_family: 0,
        protocol: SOCK_STREAM,
        recv_socks: 0,
        recv_port: 8011,
        send_port_min: 0,
    });

    fprintf(stderr, b"---- IPv6 TCP ----\n\0".as_ptr() as *const c_char);
    test_reuseport_ebpf(test_params {
        recv_family: AF_INET6,
        send_family: AF_INET6,
        protocol: SOCK_STREAM,
        recv_socks: 10,
        recv_port: 8012,
        send_port_min: 9200,
    });
    test_reuseport_cbpf(test_params {
        recv_family: AF_INET6,
        send_family: AF_INET6,
        protocol: SOCK_STREAM,
        recv_socks: 10,
        recv_port: 8013,
        send_port_min: 9240,
    });
    test_extra_filter(test_params {
        recv_family: AF_INET6,
        send_family: 0,
        protocol: SOCK_STREAM,
        recv_socks: 0,
        recv_port: 8014,
        send_port_min: 0,
    });
    test_filter_no_reuseport(test_params {
        recv_family: AF_INET6,
        send_family: 0,
        protocol: SOCK_STREAM,
        recv_socks: 0,
        recv_port: 8015,
        send_port_min: 0,
    });

    fprintf(
        stderr,
        b"---- IPv6 TCP w/ mapped IPv4 ----\n\0".as_ptr() as *const c_char,
    );
    test_reuseport_ebpf(test_params {
        recv_family: AF_INET6,
        send_family: AF_INET,
        protocol: SOCK_STREAM,
        recv_socks: 10,
        recv_port: 8016,
        send_port_min: 9320,
    });
    test_reuseport_cbpf(test_params {
        recv_family: AF_INET6,
        send_family: AF_INET,
        protocol: SOCK_STREAM,
        recv_socks: 10,
        recv_port: 8017,
        send_port_min: 9360,
    });

    test_filter_without_bind();

    fprintf(stderr, b"SUCCESS\n\0".as_ptr() as *const c_char);
    0
}

fn main() {
    unsafe {
        main_impl();
    }
}
