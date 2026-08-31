// SPDX-License-Identifier: GPL-2.0
/*
 * tcpdevmem netcat. Works similarly to netcat but does device memory TCP
 * instead of regular TCP. Uses udmabuf to mock a dmabuf provider.
 *
 * Usage:
 *
 *     On server:
 *     ncdevmem -s <server IP> [-c <client IP>] -f eth1 -l -p 5201
 *
 *     On client:
 *     echo -n "hello\nworld" | \
 *		ncdevmem -s <server IP> [-c <client IP>] -p 5201 -f eth1
 *
 * Note this is compatible with regular netcat. i.e. the sender or receiver can
 * be replaced with regular netcat to test the RX or TX path in isolation.
 *
 * Test data validation (devmem TCP on RX only):
 *
 *     On server:
 *     ncdevmem -s <server IP> [-c <client IP>] -f eth1 -l -p 5201 -v 7
 *
 *     On client:
 *     yes $(echo -e \\x01\\x02\\x03\\x04\\x05\\x06) | \
 *             head -c 1G | \
 *             nc <server IP> 5201 -p 5201
 *
 * Test data validation (devmem TCP on RX and TX, validation happens on RX):
 *
 *	On server:
 *	ncdevmem -s <server IP> [-c <client IP>] -l -p 5201 -v 8 -f eth1
 *
 *	On client:
 *	yes $(echo -e \\x01\\x02\\x03\\x04\\x05\\x06\\x07) | \
 *		head -c 1M | \
 *		ncdevmem -s <server IP> [-c <client IP>] -p 5201 -f eth1
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type FILE = c_void;
type va_list = *mut c_void;
type uint32_t = u32;
type uint64_t = u64;
type __u32 = u32;
type int64_t = i64;

const PAGE_SHIFT: c_int = 12;
const TEST_PREFIX: &[u8] = b"ncdevmem\0";
const NUM_PAGES: size_t = 16000;
const MSG_SOCK_DEVMEM: c_int = 0x2000000;
const MAX_IOV: usize = 1024;
const MAX_FLOWS: usize = 8;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const SO_BINDTODEVICE: c_int = 25;
const SO_REUSEPORT: c_int = 15;
const SO_ZEROCOPY: c_int = 60;
const SO_DEVMEM_DONTNEED: c_int = 97;
const SCM_DEVMEM_DMABUF: c_int = 98;
const SCM_DEVMEM_LINEAR: c_int = 99;
const MSG_ZEROCOPY: c_int = 0x4000000;
const MSG_ERRQUEUE: c_int = 0x2000;
const MSG_CTRUNC: c_int = 0x8;
const SOL_IP: c_int = 0;
const SOL_IPV6: c_int = 41;
const IP_RECVERR: c_int = 11;
const IPV6_RECVERR: c_int = 25;
const SO_EE_ORIGIN_ZEROCOPY: u8 = 5;
const POLLERR: i16 = 0x008;
const EAGAIN: c_int = 11;
const EWOULDBLOCK: c_int = EAGAIN;
const EFAULT: c_int = 14;
const ERANGE: c_int = 34;
const INT_MAX: c_long = 2147483647;
const ULONG_MAX: c_ulong = c_ulong::MAX;
const UINT32_MAX: c_ulong = u32::MAX as c_ulong;
const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const MFD_ALLOW_SEALING: c_uint = 0x0002;
const MFD_HUGETLB: c_uint = 0x0004;
const MFD_HUGE_2MB: c_uint = 21 << 26;
const F_ADD_SEALS: c_int = 1033;
const F_SEAL_SHRINK: c_int = 0x0002;
const UDMABUF_CREATE: c_ulong = 0x40087542;
const DMA_BUF_IOCTL_SYNC: c_ulong = 0x40086200;
const DMA_BUF_SYNC_READ: u64 = 1 << 0;
const DMA_BUF_SYNC_WRITE: u64 = 2 << 0;
const DMA_BUF_SYNC_START: u64 = 0 << 2;
const DMA_BUF_SYNC_END: u64 = 1 << 2;
const ETHTOOL_TCP_DATA_SPLIT_UNKNOWN: c_int = 0;
const ETHTOOL_TCP_DATA_SPLIT_ENABLED: c_int = 2;
const NETDEV_QUEUE_TYPE_RX: c_int = 0;

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
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
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
struct cmsghdr {
    cmsg_len: size_t,
    cmsg_level: c_int,
    cmsg_type: c_int,
}

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
}

#[repr(C)]
struct dma_buf_sync {
    flags: u64,
}

#[repr(C)]
struct udmabuf_create {
    memfd: u32,
    flags: u32,
    offset: u64,
    size: u64,
}

#[repr(C)]
struct dmabuf_cmsg {
    frag_offset: u64,
    frag_size: u32,
    frag_token: u32,
    dmabuf_id: u32,
}

#[repr(C)]
struct dmabuf_token {
    token_start: u32,
    token_count: u32,
}

#[repr(C)]
struct sock_extended_err {
    ee_errno: u32,
    ee_origin: u8,
    ee_type: u8,
    ee_code: u8,
    ee_pad: u8,
    ee_info: u32,
    ee_data: u32,
}

#[repr(C)]
struct ynl_error {
    msg: *const c_char,
}

#[repr(C)]
struct ynl_sock {
    err: ynl_error,
}

#[repr(C)]
struct present_bits {
    hds_thresh: bool,
    combined_count: bool,
    rx_count: bool,
    tx_count: bool,
    id: bool,
}

#[repr(C)]
struct ethtool_rings_get_req {
    _private: [u8; 0],
}
#[repr(C)]
struct ethtool_rings_set_req {
    _private: [u8; 0],
}
#[repr(C)]
struct ethtool_rings_get_rsp {
    _present: present_bits,
    hds_thresh: c_uint,
    tcp_data_split: c_int,
}
#[repr(C)]
struct ethtool_channels_get_req {
    _private: [u8; 0],
}
#[repr(C)]
struct ethtool_channels_set_req {
    _private: [u8; 0],
}
#[repr(C)]
struct ethtool_channels_get_rsp {
    _present: present_bits,
    rx_count: c_uint,
    tx_count: c_uint,
    combined_count: c_uint,
}
#[repr(C)]
struct netdev_queue_id {
    _private: [u8; 0],
}
#[repr(C)]
struct netdev_bind_rx_req {
    _private: [u8; 0],
}
#[repr(C)]
struct netdev_bind_rx_rsp {
    _present: present_bits,
    id: c_uint,
}
#[repr(C)]
struct netdev_bind_tx_req {
    _private: [u8; 0],
}
#[repr(C)]
struct netdev_bind_tx_rsp {
    _present: present_bits,
    id: c_uint,
}

#[repr(C)]
struct memory_buffer {
    fd: c_int,
    size: size_t,
    devfd: c_int,
    memfd: c_int,
    buf_mem: *mut c_char,
}

#[repr(C)]
struct memory_provider {
    alloc: unsafe extern "C" fn(size: size_t) -> *mut memory_buffer,
    free: unsafe extern "C" fn(ctx: *mut memory_buffer),
    memcpy_to_device:
        unsafe extern "C" fn(dst: *mut memory_buffer, off: size_t, src: *mut c_void, n: c_int),
    memcpy_from_device:
        unsafe extern "C" fn(dst: *mut c_void, src: *mut memory_buffer, off: size_t, n: c_int),
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut optopt: c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static ynl_ethtool_family: c_void;
    static ynl_netdev_family: c_void;

    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, fmt: *const c_char, args: va_list) -> c_int;
    fn perror(s: *const c_char);
    fn strerror(errnum: c_int) -> *mut c_char;
    fn putchar(c: c_int) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn atoi(nptr: *const c_char) -> c_int;
    fn atoll(nptr: *const c_char) -> i64;
    fn vsnprintf(s: *mut c_char, n: size_t, format: *const c_char, arg: va_list) -> c_int;
    fn popen(command: *const c_char, mode: *const c_char) -> *mut FILE;
    fn pclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn ftruncate(fd: c_int, length: c_long) -> c_int;
    fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn getpagesize() -> c_int;
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
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char, size: socklen_t) -> *const c_char;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;

    fn ynl_sock_create(family: *const c_void, yerr: *mut ynl_error) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);
    fn ethtool_channels_get_req_alloc() -> *mut ethtool_channels_get_req;
    fn ethtool_channels_get_req_set_header_dev_index(req: *mut ethtool_channels_get_req, ifindex: c_uint);
    fn ethtool_channels_get(ys: *mut ynl_sock, req: *mut ethtool_channels_get_req) -> *mut ethtool_channels_get_rsp;
    fn ethtool_channels_get_req_free(req: *mut ethtool_channels_get_req);
    fn ethtool_channels_get_rsp_free(rsp: *mut ethtool_channels_get_rsp);
    fn ethtool_channels_set_req_alloc() -> *mut ethtool_channels_set_req;
    fn ethtool_channels_set_req_set_header_dev_index(req: *mut ethtool_channels_set_req, ifindex: c_uint);
    fn ethtool_channels_set_req_set_rx_count(req: *mut ethtool_channels_set_req, val: c_uint);
    fn ethtool_channels_set_req_set_tx_count(req: *mut ethtool_channels_set_req, val: c_uint);
    fn ethtool_channels_set_req_set_combined_count(req: *mut ethtool_channels_set_req, val: c_uint);
    fn ethtool_channels_set(ys: *mut ynl_sock, req: *mut ethtool_channels_set_req) -> c_int;
    fn ethtool_channels_set_req_free(req: *mut ethtool_channels_set_req);
    fn ethtool_rings_get_req_alloc() -> *mut ethtool_rings_get_req;
    fn ethtool_rings_get_req_set_header_dev_index(req: *mut ethtool_rings_get_req, ifindex: c_uint);
    fn ethtool_rings_get(ys: *mut ynl_sock, req: *mut ethtool_rings_get_req) -> *mut ethtool_rings_get_rsp;
    fn ethtool_rings_get_req_free(req: *mut ethtool_rings_get_req);
    fn ethtool_rings_get_rsp_free(rsp: *mut ethtool_rings_get_rsp);
    fn ethtool_rings_set_req_alloc() -> *mut ethtool_rings_set_req;
    fn ethtool_rings_set_req_set_header_dev_index(req: *mut ethtool_rings_set_req, ifindex: c_uint);
    fn ethtool_rings_set_req_set_tcp_data_split(req: *mut ethtool_rings_set_req, val: c_int);
    fn ethtool_rings_set_req_set_hds_thresh(req: *mut ethtool_rings_set_req, val: c_uint);
    fn ethtool_rings_set(ys: *mut ynl_sock, req: *mut ethtool_rings_set_req) -> c_int;
    fn ethtool_rings_set_req_free(req: *mut ethtool_rings_set_req);
    fn netdev_queue_id_alloc(n: c_int) -> *mut netdev_queue_id;
    fn netdev_queue_id_free(q: *mut netdev_queue_id);
    fn netdev_queue_id_set_type(q: *mut netdev_queue_id, typ: c_int);
    fn netdev_queue_id_set_id(q: *mut netdev_queue_id, id: c_int);
    fn netdev_bind_rx_req_alloc() -> *mut netdev_bind_rx_req;
    fn netdev_bind_rx_req_set_ifindex(req: *mut netdev_bind_rx_req, ifindex: c_uint);
    fn netdev_bind_rx_req_set_fd(req: *mut netdev_bind_rx_req, fd: c_uint);
    fn __netdev_bind_rx_req_set_queues(req: *mut netdev_bind_rx_req, queues: *mut netdev_queue_id, n: c_uint);
    fn netdev_bind_rx_req_set_rx_page_size(req: *mut netdev_bind_rx_req, rx_page_size: c_uint);
    fn netdev_bind_rx(ys: *mut ynl_sock, req: *mut netdev_bind_rx_req) -> *mut netdev_bind_rx_rsp;
    fn netdev_bind_rx_req_free(req: *mut netdev_bind_rx_req);
    fn netdev_bind_rx_rsp_free(rsp: *mut netdev_bind_rx_rsp);
    fn netdev_bind_tx_req_alloc() -> *mut netdev_bind_tx_req;
    fn netdev_bind_tx_req_set_ifindex(req: *mut netdev_bind_tx_req, ifindex: c_uint);
    fn netdev_bind_tx_req_set_fd(req: *mut netdev_bind_tx_req, fd: c_uint);
    fn netdev_bind_tx(ys: *mut ynl_sock, req: *mut netdev_bind_tx_req) -> *mut netdev_bind_tx_rsp;
    fn netdev_bind_tx_req_free(req: *mut netdev_bind_tx_req);
    fn netdev_bind_tx_rsp_free(rsp: *mut netdev_bind_tx_rsp);
}

static mut max_chunk: size_t = 0;
static mut server_ip: *mut c_char = ptr::null_mut();
static mut client_ip: *mut c_char = ptr::null_mut();
static mut port: *mut c_char = ptr::null_mut();
static mut do_validation: size_t = 0;
static mut start_queue: c_int = -1;
static mut num_queues: c_int = -1;
static mut skip_config: c_int = 0;
static mut ifname: *mut c_char = ptr::null_mut();
static mut ifindex: c_uint = 0;
static mut dmabuf_id: c_uint = 0;
static mut tx_dmabuf_id: uint32_t = 0;
static mut waittime_ms: c_int = 500;
static mut fail_on_linear: bool = false;
static mut rx_page_size: uint32_t = 0;

/* System state loaded by current_config_load() */
static mut ntuple_ids: [c_int; MAX_FLOWS] = [-1, -1, -1, -1, -1, -1, -1, -1];

unsafe fn MB(x: size_t) -> size_t {
    x << 20
}

unsafe fn cmsg_align(len: size_t) -> size_t {
    let align = size_of::<size_t>();
    (len + align - 1) & !(align - 1)
}

unsafe fn CMSG_SPACE(len: size_t) -> size_t {
    cmsg_align(size_of::<cmsghdr>()) + cmsg_align(len)
}

unsafe fn CMSG_LEN(len: size_t) -> size_t {
    cmsg_align(size_of::<cmsghdr>()) + len
}

unsafe fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut c_void {
    (cmsg as *mut u8).add(cmsg_align(size_of::<cmsghdr>())) as *mut c_void
}

unsafe fn CMSG_FIRSTHDR(msg: *mut msghdr) -> *mut cmsghdr {
    if (*msg).msg_controllen >= size_of::<cmsghdr>() {
        (*msg).msg_control as *mut cmsghdr
    } else {
        ptr::null_mut()
    }
}

unsafe fn CMSG_NXTHDR(msg: *mut msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr {
    let next = (cmsg as *mut u8).add(cmsg_align((*cmsg).cmsg_len)) as *mut cmsghdr;
    let max = ((*msg).msg_control as *mut u8).add((*msg).msg_controllen);
    if (next as *mut u8).add(size_of::<cmsghdr>()) > max {
        ptr::null_mut()
    } else {
        next
    }
}

unsafe fn IN6_IS_ADDR_V4MAPPED(a: *const in6_addr) -> bool {
    (*a).s6_addr[0..10].iter().all(|&b| b == 0) && (*a).s6_addr[10] == 0xff && (*a).s6_addr[11] == 0xff
}

unsafe fn s6_addr32(a: *mut in6_addr, idx: usize) -> *mut u32 {
    (*a).s6_addr.as_mut_ptr().cast::<u32>().add(idx)
}

unsafe fn s6_addr16(a: *mut in6_addr, idx: usize) -> *mut u16 {
    (*a).s6_addr.as_mut_ptr().cast::<u16>().add(idx)
}

unsafe extern "C" fn pr_err(fmt: *const c_char, mut args: ...) {
    fprintf(stderr, b"%s: \0".as_ptr() as *const c_char, TEST_PREFIX.as_ptr());
    vfprintf(stderr, fmt, args.as_va_list());
    if errno != 0 {
        fprintf(stderr, b": %s\0".as_ptr() as *const c_char, strerror(errno));
    }
    fprintf(stderr, b"\n\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn udmabuf_alloc(mut size: size_t) -> *mut memory_buffer {
    let mut create: udmabuf_create = zeroed();
    let ctx: *mut memory_buffer;
    let mut memfd_flags: c_uint;
    let mut ret: c_int;

    ctx = malloc(size_of::<memory_buffer>()) as *mut memory_buffer;
    if ctx.is_null() {
        return ptr::null_mut();
    }

    (*ctx).size = size;

    (*ctx).devfd = open(b"/dev/udmabuf\0".as_ptr() as *const c_char, O_RDONLY);
    if (*ctx).devfd < 0 {
        pr_err(b"[skip,no-udmabuf: Unable to access DMA buffer device file]\0".as_ptr() as *const c_char);
        free(ctx as *mut c_void);
        return ptr::null_mut();
    }

    memfd_flags = MFD_ALLOW_SEALING;
    if rx_page_size > getpagesize() as u32 {
        memfd_flags |= MFD_HUGETLB | MFD_HUGE_2MB;
    }

    (*ctx).memfd = memfd_create(b"udmabuf-test\0".as_ptr() as *const c_char, memfd_flags);
    if (*ctx).memfd < 0 {
        pr_err(
            b"[skip,no-memfd%s]\0".as_ptr() as *const c_char,
            if (memfd_flags & MFD_HUGETLB) != 0 {
                b" (need hugepages)\0".as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
        );
        close((*ctx).devfd);
        free(ctx as *mut c_void);
        return ptr::null_mut();
    }

    ret = fcntl((*ctx).memfd, F_ADD_SEALS, F_SEAL_SHRINK);
    if ret < 0 {
        pr_err(b"[skip,fcntl-add-seals]\0".as_ptr() as *const c_char);
        close((*ctx).memfd);
        close((*ctx).devfd);
        free(ctx as *mut c_void);
        return ptr::null_mut();
    }

    if (memfd_flags & MFD_HUGETLB) != 0 {
        let m = MB(2);
        size = ((size + m - 1) / m) * m;
        (*ctx).size = size;
    }

    ret = ftruncate((*ctx).memfd, size as c_long);
    if ret == -1 {
        pr_err(b"[FAIL,memfd-truncate]\0".as_ptr() as *const c_char);
        close((*ctx).memfd);
        close((*ctx).devfd);
        free(ctx as *mut c_void);
        return ptr::null_mut();
    }

    memset(&mut create as *mut _ as *mut c_void, 0, size_of::<udmabuf_create>());
    create.memfd = (*ctx).memfd as u32;
    create.offset = 0;
    create.size = size as u64;
    (*ctx).fd = ioctl((*ctx).devfd, UDMABUF_CREATE, &mut create as *mut _);
    if (*ctx).fd < 0 {
        pr_err(b"[FAIL, create udmabuf]\0".as_ptr() as *const c_char);
        close((*ctx).fd);
        close((*ctx).memfd);
        close((*ctx).devfd);
        free(ctx as *mut c_void);
        return ptr::null_mut();
    }

    (*ctx).buf_mem = mmap(ptr::null_mut(), size, PROT_READ | PROT_WRITE, MAP_SHARED, (*ctx).fd, 0) as *mut c_char;
    if (*ctx).buf_mem as *mut c_void == MAP_FAILED {
        pr_err(b"[FAIL, map udmabuf]\0".as_ptr() as *const c_char);
        close((*ctx).fd);
        close((*ctx).memfd);
        close((*ctx).devfd);
        free(ctx as *mut c_void);
        return ptr::null_mut();
    }

    ctx
}

unsafe extern "C" fn udmabuf_free(ctx: *mut memory_buffer) {
    munmap((*ctx).buf_mem as *mut c_void, (*ctx).size);
    close((*ctx).fd);
    close((*ctx).memfd);
    close((*ctx).devfd);
    free(ctx as *mut c_void);
}

unsafe extern "C" fn udmabuf_memcpy_to_device(dst: *mut memory_buffer, off: size_t, src: *mut c_void, n: c_int) {
    let mut sync: dma_buf_sync = zeroed();

    sync.flags = DMA_BUF_SYNC_START | DMA_BUF_SYNC_WRITE;
    ioctl((*dst).fd, DMA_BUF_IOCTL_SYNC, &mut sync as *mut _);

    memcpy((*dst).buf_mem.add(off) as *mut c_void, src, n as size_t);

    sync.flags = DMA_BUF_SYNC_END | DMA_BUF_SYNC_WRITE;
    ioctl((*dst).fd, DMA_BUF_IOCTL_SYNC, &mut sync as *mut _);
}

unsafe extern "C" fn udmabuf_memcpy_from_device(dst: *mut c_void, src: *mut memory_buffer, off: size_t, n: c_int) {
    let mut sync: dma_buf_sync = zeroed();

    sync.flags = DMA_BUF_SYNC_START;
    ioctl((*src).fd, DMA_BUF_IOCTL_SYNC, &mut sync as *mut _);

    memcpy(dst, (*src).buf_mem.add(off) as *const c_void, n as size_t);

    sync.flags = DMA_BUF_SYNC_END;
    ioctl((*src).fd, DMA_BUF_IOCTL_SYNC, &mut sync as *mut _);
}

static mut udmabuf_memory_provider: memory_provider = memory_provider {
    alloc: udmabuf_alloc,
    free: udmabuf_free,
    memcpy_to_device: udmabuf_memcpy_to_device,
    memcpy_from_device: udmabuf_memcpy_from_device,
};

static mut provider: *mut memory_provider = ptr::addr_of_mut!(udmabuf_memory_provider);

unsafe fn print_nonzero_bytes(ptr_: *mut c_void, size: size_t) {
    let p = ptr_ as *mut u8;
    let mut i: c_uint = 0;

    while (i as size_t) < size {
        putchar(*p.add(i as usize) as c_int);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn validate_buffer(line: *mut c_void, size: size_t) -> c_int {
    static mut seed: u8 = 1;
    static mut errors: c_int = 0;
    let ptr_ = line as *mut u8;
    let mut expected: u8;
    let mut i: size_t = 0;

    while i < size {
        expected = if seed != 0 { seed } else { b'\n' };
        if *ptr_.add(i) != expected {
            fprintf(
                stderr,
                b"Failed validation: expected=%u, actual=%u, index=%lu\n\0".as_ptr() as *const c_char,
                expected as c_uint,
                *ptr_.add(i) as c_uint,
                i as c_ulong,
            );
            errors += 1;
            if errors > 20 {
                pr_err(b"validation failed\0".as_ptr() as *const c_char);
                return -1;
            }
        }
        seed = seed.wrapping_add(1);
        if seed as size_t == do_validation {
            seed = 0;
        }
        i += 1;
    }

    fprintf(stdout, b"Validated buffer\n\0".as_ptr() as *const c_char);
    0
}

unsafe fn __run_command(out: *mut c_char, outlen: size_t, cmd: *const c_char, args: va_list) -> c_int {
    let mut command = [0 as c_char; 256];
    let fp: *mut FILE;

    vsnprintf(command.as_mut_ptr(), command.len(), cmd, args);
    fprintf(stderr, b"Running: %s\n\0".as_ptr() as *const c_char, command.as_ptr());
    fp = popen(command.as_ptr(), b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        return -1;
    }
    if !out.is_null() {
        let len: size_t;
        if fgets(out, outlen as c_int, fp).is_null() {
            return -1;
        }

        /* Remove trailing newline if present */
        len = strlen(out);
        if len != 0 && *out.add(len - 1) == b'\n' as c_char {
            *out.add(len - 1) = b'\0' as c_char;
        }
    }
    pclose(fp)
}

unsafe extern "C" fn run_command(cmd: *const c_char, args: ...) -> c_int {
    __run_command(ptr::null_mut(), 0, cmd, args.as_va_list())
}

unsafe extern "C" fn ethtool_add_flow(format: *const c_char, args: ...) -> c_int {
    let mut local_output = [0 as c_char; 256];
    let mut cmd = [0 as c_char; 256];
    let mut id_start: *const c_char;
    let mut flow_idx: c_int = 0;
    let ret: c_int;
    let mut endptr: *mut c_char = ptr::null_mut();
    let flow_id: c_long;

    while flow_idx < MAX_FLOWS as c_int {
        if ntuple_ids[flow_idx as usize] == -1 {
            break;
        }
        flow_idx += 1;
    }
    if flow_idx == MAX_FLOWS as c_int {
        fprintf(stderr, b"Error: too many flows\n\0".as_ptr() as *const c_char);
        return -1;
    }

    snprintf_shim(
        cmd.as_mut_ptr(),
        cmd.len(),
        b"ethtool -N %s %s\0".as_ptr() as *const c_char,
        ifname,
        format,
    );

    ret = __run_command(local_output.as_mut_ptr(), local_output.len(), cmd.as_ptr(), args.as_va_list());
    if ret != 0 {
        return ret;
    }

    /* Extract the ID from the output */
    id_start = strstr(local_output.as_ptr(), b"Added rule with ID \0".as_ptr() as *const c_char);
    if id_start.is_null() {
        return -1;
    }
    id_start = id_start.add(strlen(b"Added rule with ID \0".as_ptr() as *const c_char));

    flow_id = strtol(id_start, &mut endptr, 10);
    if endptr == id_start as *mut c_char || flow_id < 0 || flow_id > INT_MAX {
        return -1;
    }

    fprintf(stderr, b"Added flow rule with ID %ld\n\0".as_ptr() as *const c_char, flow_id);
    ntuple_ids[flow_idx as usize] = flow_id as c_int;
    flow_id as c_int
}

unsafe fn snprintf_shim(dst: *mut c_char, len: size_t, fmt: *const c_char, a: *mut c_char, b: *const c_char) {
    unsafe extern "C" {
        fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    }
    snprintf(dst, len, fmt, a, b);
}

unsafe fn rxq_num(ifindex_: c_int) -> c_int {
    let req: *mut ethtool_channels_get_req;
    let rsp: *mut ethtool_channels_get_rsp;
    let mut yerr: ynl_error = zeroed();
    let ys: *mut ynl_sock;
    let mut num: c_int = -1;

    ys = ynl_sock_create(ptr::addr_of!(ynl_ethtool_family), &mut yerr);
    if ys.is_null() {
        fprintf(stderr, b"YNL: %s\n\0".as_ptr() as *const c_char, yerr.msg);
        return -1;
    }

    req = ethtool_channels_get_req_alloc();
    ethtool_channels_get_req_set_header_dev_index(req, ifindex_ as c_uint);
    rsp = ethtool_channels_get(ys, req);
    if !rsp.is_null() {
        num = ((*rsp).rx_count + (*rsp).combined_count) as c_int;
    }
    ethtool_channels_get_req_free(req);
    ethtool_channels_get_rsp_free(rsp);
    ynl_sock_destroy(ys);
    num
}

unsafe fn reset_flow_steering() {
    let mut i = 0;
    while i < MAX_FLOWS {
        if ntuple_ids[i] != -1 {
            run_command(b"ethtool -N %s delete %d\0".as_ptr() as *const c_char, ifname, ntuple_ids[i]);
            ntuple_ids[i] = -1;
        }
        i += 1;
    }
}

unsafe fn tcp_data_split_str(val: c_int) -> *const c_char {
    match val {
        0 => b"off\0".as_ptr() as *const c_char,
        1 => b"auto\0".as_ptr() as *const c_char,
        2 => b"on\0".as_ptr() as *const c_char,
        _ => b"?\0".as_ptr() as *const c_char,
    }
}

unsafe fn get_ring_config() -> *mut ethtool_rings_get_rsp {
    let get_req: *mut ethtool_rings_get_req;
    let get_rsp: *mut ethtool_rings_get_rsp;
    let mut yerr: ynl_error = zeroed();
    let ys: *mut ynl_sock;

    ys = ynl_sock_create(ptr::addr_of!(ynl_ethtool_family), &mut yerr);
    if ys.is_null() {
        fprintf(stderr, b"YNL: %s\n\0".as_ptr() as *const c_char, yerr.msg);
        return ptr::null_mut();
    }

    get_req = ethtool_rings_get_req_alloc();
    ethtool_rings_get_req_set_header_dev_index(get_req, ifindex);
    get_rsp = ethtool_rings_get(ys, get_req);
    ethtool_rings_get_req_free(get_req);
    ynl_sock_destroy(ys);
    get_rsp
}

unsafe fn restore_ring_config(config: *const ethtool_rings_get_rsp) {
    let get_req: *mut ethtool_rings_get_req;
    let get_rsp: *mut ethtool_rings_get_rsp;
    let req: *mut ethtool_rings_set_req;
    let mut yerr: ynl_error = zeroed();
    let ys: *mut ynl_sock;
    let mut ret: c_int;

    if config.is_null() {
        return;
    }

    ys = ynl_sock_create(ptr::addr_of!(ynl_ethtool_family), &mut yerr);
    if ys.is_null() {
        fprintf(stderr, b"YNL: %s\n\0".as_ptr() as *const c_char, yerr.msg);
        return;
    }

    req = ethtool_rings_set_req_alloc();
    ethtool_rings_set_req_set_header_dev_index(req, ifindex);
    ethtool_rings_set_req_set_tcp_data_split(req, ETHTOOL_TCP_DATA_SPLIT_UNKNOWN);
    if (*config)._present.hds_thresh {
        ethtool_rings_set_req_set_hds_thresh(req, (*config).hds_thresh);
    }

    ret = ethtool_rings_set(ys, req);
    if ret < 0 {
        fprintf(stderr, b"YNL restoring HDS cfg: %s\n\0".as_ptr() as *const c_char, (*ys).err.msg);
    }

    get_req = ethtool_rings_get_req_alloc();
    ethtool_rings_get_req_set_header_dev_index(get_req, ifindex);
    get_rsp = ethtool_rings_get(ys, get_req);
    ethtool_rings_get_req_free(get_req);

    /* use explicit value if UKNOWN didn't give us the previous */
    if !get_rsp.is_null() && (*get_rsp).tcp_data_split != (*config).tcp_data_split {
        ethtool_rings_set_req_set_tcp_data_split(req, (*config).tcp_data_split);
        ret = ethtool_rings_set(ys, req);
        if ret < 0 {
            fprintf(stderr, b"YNL restoring expl HDS cfg: %s\n\0".as_ptr() as *const c_char, (*ys).err.msg);
        }
    }

    ethtool_rings_get_rsp_free(get_rsp);
    ethtool_rings_set_req_free(req);
    ynl_sock_destroy(ys);
}

unsafe fn configure_headersplit(old: *const ethtool_rings_get_rsp, on: bool) -> c_int {
    let get_req: *mut ethtool_rings_get_req;
    let get_rsp: *mut ethtool_rings_get_rsp;
    let req: *mut ethtool_rings_set_req;
    let mut yerr: ynl_error = zeroed();
    let ys: *mut ynl_sock;
    let ret: c_int;

    ys = ynl_sock_create(ptr::addr_of!(ynl_ethtool_family), &mut yerr);
    if ys.is_null() {
        fprintf(stderr, b"YNL: %s\n\0".as_ptr() as *const c_char, yerr.msg);
        return -1;
    }

    req = ethtool_rings_set_req_alloc();
    ethtool_rings_set_req_set_header_dev_index(req, ifindex);
    if on {
        ethtool_rings_set_req_set_tcp_data_split(req, ETHTOOL_TCP_DATA_SPLIT_ENABLED);
        if (*old)._present.hds_thresh {
            ethtool_rings_set_req_set_hds_thresh(req, 0);
        }
    } else {
        ethtool_rings_set_req_set_tcp_data_split(req, ETHTOOL_TCP_DATA_SPLIT_UNKNOWN);
    }
    ret = ethtool_rings_set(ys, req);
    if ret < 0 {
        fprintf(stderr, b"YNL failed: %s\n\0".as_ptr() as *const c_char, (*ys).err.msg);
    }
    ethtool_rings_set_req_free(req);

    if ret == 0 {
        get_req = ethtool_rings_get_req_alloc();
        ethtool_rings_get_req_set_header_dev_index(get_req, ifindex);
        get_rsp = ethtool_rings_get(ys, get_req);
        ethtool_rings_get_req_free(get_req);
        if !get_rsp.is_null() {
            fprintf(stderr, b"TCP header split: %s\n\0".as_ptr() as *const c_char, tcp_data_split_str((*get_rsp).tcp_data_split));
        }
        ethtool_rings_get_rsp_free(get_rsp);
    }

    ynl_sock_destroy(ys);
    ret
}

unsafe fn configure_rss() -> c_int {
    run_command(b"ethtool -X %s equal %d >&2\0".as_ptr() as *const c_char, ifname, start_queue)
}

unsafe fn reset_rss() {
    run_command(b"ethtool -X %s default >&2\0".as_ptr() as *const c_char, ifname, start_queue);
}

unsafe fn check_changing_channels(rx: c_uint, tx: c_uint) -> c_int {
    let gchan: *mut ethtool_channels_get_req;
    let schan: *mut ethtool_channels_set_req;
    let chan: *mut ethtool_channels_get_rsp;
    let mut yerr: ynl_error = zeroed();
    let ys: *mut ynl_sock;
    let mut ret: c_int;

    fprintf(stderr, b"setting channel count rx:%u tx:%u\n\0".as_ptr() as *const c_char, rx, tx);

    ys = ynl_sock_create(ptr::addr_of!(ynl_ethtool_family), &mut yerr);
    if ys.is_null() {
        fprintf(stderr, b"YNL: %s\n\0".as_ptr() as *const c_char, yerr.msg);
        return -1;
    }

    gchan = ethtool_channels_get_req_alloc();
    if gchan.is_null() {
        ynl_sock_destroy(ys);
        return -1;
    }

    ethtool_channels_get_req_set_header_dev_index(gchan, ifindex);
    chan = ethtool_channels_get(ys, gchan);
    ethtool_channels_get_req_free(gchan);
    if chan.is_null() {
        fprintf(stderr, b"YNL get channels: %s\n\0".as_ptr() as *const c_char, (*ys).err.msg);
        ynl_sock_destroy(ys);
        return -1;
    }

    schan = ethtool_channels_set_req_alloc();
    if schan.is_null() {
        ethtool_channels_get_rsp_free(chan);
        ynl_sock_destroy(ys);
        return -1;
    }

    ethtool_channels_set_req_set_header_dev_index(schan, ifindex);

    if (*chan)._present.combined_count {
        if (*chan)._present.rx_count || (*chan)._present.tx_count {
            ethtool_channels_set_req_set_rx_count(schan, 0);
            ethtool_channels_set_req_set_tx_count(schan, 0);
        }

        if rx == tx {
            ethtool_channels_set_req_set_combined_count(schan, rx);
        } else if rx > tx {
            ethtool_channels_set_req_set_combined_count(schan, tx);
            ethtool_channels_set_req_set_rx_count(schan, rx - tx);
        } else {
            ethtool_channels_set_req_set_combined_count(schan, rx);
            ethtool_channels_set_req_set_tx_count(schan, tx - rx);
        }
    } else if (*chan)._present.rx_count {
        ethtool_channels_set_req_set_rx_count(schan, rx);
        ethtool_channels_set_req_set_tx_count(schan, tx);
    } else {
        fprintf(stderr, b"Error: device has neither combined nor rx channels\n\0".as_ptr() as *const c_char);
        ethtool_channels_set_req_free(schan);
        ethtool_channels_get_rsp_free(chan);
        ynl_sock_destroy(ys);
        return -1;
    }

    ret = ethtool_channels_set(ys, schan);
    if ret != 0 {
        fprintf(stderr, b"YNL set channels: %s\n\0".as_ptr() as *const c_char, (*ys).err.msg);
    } else {
        /* We were expecting a failure, go back to previous settings */
        ethtool_channels_set_req_set_combined_count(schan, (*chan).combined_count);
        ethtool_channels_set_req_set_rx_count(schan, (*chan).rx_count);
        ethtool_channels_set_req_set_tx_count(schan, (*chan).tx_count);

        ret = ethtool_channels_set(ys, schan);
        if ret != 0 {
            fprintf(stderr, b"YNL un-setting channels: %s\n\0".as_ptr() as *const c_char, (*ys).err.msg);
        }
    }

    ethtool_channels_set_req_free(schan);
    ethtool_channels_get_rsp_free(chan);
    ynl_sock_destroy(ys);
    ret
}

unsafe fn configure_flow_steering(server_sin: *mut sockaddr_in6) -> c_int {
    let mut typ = b"tcp6\0".as_ptr() as *const c_char;
    let mut server_addr: *const c_char;
    let mut buf = [0 as c_char; 40];
    let mut flow_id: c_int;

    inet_ntop(AF_INET6, ptr::addr_of!((*server_sin).sin6_addr) as *const c_void, buf.as_mut_ptr(), buf.len() as socklen_t);
    server_addr = buf.as_ptr();

    if IN6_IS_ADDR_V4MAPPED(ptr::addr_of!((*server_sin).sin6_addr)) {
        typ = b"tcp4\0".as_ptr() as *const c_char;
        server_addr = strrchr(server_addr, b':' as c_int).add(1);
    }

    /* Try configure 5-tuple */
    flow_id = ethtool_add_flow(
        b"flow-type %s %s %s dst-ip %s %s %s dst-port %s queue %d\0".as_ptr() as *const c_char,
        typ,
        if !client_ip.is_null() { b"src-ip\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char },
        if !client_ip.is_null() { client_ip } else { b"\0".as_ptr() as *const c_char as *mut c_char },
        server_addr,
        if !client_ip.is_null() { b"src-port\0".as_ptr() as *const c_char } else { b"\0".as_ptr() as *const c_char },
        if !client_ip.is_null() { port } else { b"\0".as_ptr() as *const c_char as *mut c_char },
        port,
        start_queue,
    );
    if flow_id < 0 {
        /* If that fails, try configure 3-tuple */
        flow_id = ethtool_add_flow(
            b"flow-type %s dst-ip %s dst-port %s queue %d\0".as_ptr() as *const c_char,
            typ,
            server_addr,
            port,
            start_queue,
        );
        if flow_id < 0 {
            /* If that fails, return error */
            return -1;
        }
    }

    0
}

unsafe fn bind_rx_queue(
    ifindex_: c_uint,
    dmabuf_fd: c_uint,
    queues: *mut netdev_queue_id,
    n_queue_index: c_uint,
    ys: *mut *mut ynl_sock,
) -> c_int {
    let mut req: *mut netdev_bind_rx_req = ptr::null_mut();
    let mut rsp: *mut netdev_bind_rx_rsp = ptr::null_mut();
    let mut yerr: ynl_error = zeroed();

    *ys = ynl_sock_create(ptr::addr_of!(ynl_netdev_family), &mut yerr);
    if (*ys).is_null() {
        netdev_queue_id_free(queues);
        fprintf(stderr, b"YNL: %s\n\0".as_ptr() as *const c_char, yerr.msg);
        return -1;
    }

    req = netdev_bind_rx_req_alloc();
    netdev_bind_rx_req_set_ifindex(req, ifindex_);
    netdev_bind_rx_req_set_fd(req, dmabuf_fd);
    __netdev_bind_rx_req_set_queues(req, queues, n_queue_index);
    if rx_page_size != 0 {
        netdev_bind_rx_req_set_rx_page_size(req, rx_page_size);
    }

    rsp = netdev_bind_rx(*ys, req);
    if rsp.is_null() {
        perror(b"netdev_bind_rx\0".as_ptr() as *const c_char);
        fprintf(stderr, b"YNL failed: %s\n\0".as_ptr() as *const c_char, (**ys).err.msg);
        netdev_bind_rx_req_free(req);
        ynl_sock_destroy(*ys);
        return -1;
    }

    if !(*rsp)._present.id {
        perror(b"id not present\0".as_ptr() as *const c_char);
        fprintf(stderr, b"YNL failed: %s\n\0".as_ptr() as *const c_char, (**ys).err.msg);
        netdev_bind_rx_req_free(req);
        ynl_sock_destroy(*ys);
        return -1;
    }

    fprintf(stderr, b"got dmabuf id=%d\n\0".as_ptr() as *const c_char, (*rsp).id);
    dmabuf_id = (*rsp).id;

    netdev_bind_rx_req_free(req);
    netdev_bind_rx_rsp_free(rsp);
    0
}

unsafe fn bind_tx_queue(ifindex_: c_uint, dmabuf_fd: c_uint, ys: *mut *mut ynl_sock) -> c_int {
    let req: *mut netdev_bind_tx_req;
    let rsp: *mut netdev_bind_tx_rsp;
    let mut yerr: ynl_error = zeroed();

    *ys = ynl_sock_create(ptr::addr_of!(ynl_netdev_family), &mut yerr);
    if (*ys).is_null() {
        fprintf(stderr, b"YNL: %s\n\0".as_ptr() as *const c_char, yerr.msg);
        return -1;
    }

    req = netdev_bind_tx_req_alloc();
    netdev_bind_tx_req_set_ifindex(req, ifindex_);
    netdev_bind_tx_req_set_fd(req, dmabuf_fd);
    rsp = netdev_bind_tx(*ys, req);
    if rsp.is_null() {
        perror(b"netdev_bind_tx\0".as_ptr() as *const c_char);
        fprintf(stderr, b"YNL failed: %s\n\0".as_ptr() as *const c_char, (**ys).err.msg);
        netdev_bind_tx_req_free(req);
        ynl_sock_destroy(*ys);
        return -1;
    }

    if !(*rsp)._present.id {
        perror(b"id not present\0".as_ptr() as *const c_char);
        fprintf(stderr, b"YNL failed: %s\n\0".as_ptr() as *const c_char, (**ys).err.msg);
        netdev_bind_tx_req_free(req);
        ynl_sock_destroy(*ys);
        return -1;
    }

    fprintf(stderr, b"got tx dmabuf id=%d\n\0".as_ptr() as *const c_char, (*rsp).id);
    tx_dmabuf_id = (*rsp).id;

    netdev_bind_tx_req_free(req);
    netdev_bind_tx_rsp_free(rsp);
    0
}

unsafe fn enable_reuseaddr(fd: c_int) -> c_int {
    let opt: c_int = 1;
    let mut ret: c_int;

    ret = setsockopt(fd, SOL_SOCKET, SO_REUSEPORT, &opt as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
    if ret != 0 {
        pr_err(b"SO_REUSEPORT failed\0".as_ptr() as *const c_char);
        return -1;
    }

    ret = setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &opt as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
    if ret != 0 {
        pr_err(b"SO_REUSEADDR failed\0".as_ptr() as *const c_char);
        return -1;
    }

    0
}

unsafe fn parse_address(str_: *const c_char, port_: c_int, sin6: *mut sockaddr_in6) -> c_int {
    let mut ret: c_int;

    (*sin6).sin6_family = AF_INET6 as u16;
    (*sin6).sin6_port = htons(port_ as u16);

    ret = inet_pton((*sin6).sin6_family as c_int, str_, ptr::addr_of_mut!((*sin6).sin6_addr) as *mut c_void);
    if ret != 1 {
        /* fallback to plain IPv4 */
        ret = inet_pton(AF_INET, str_, s6_addr32(ptr::addr_of_mut!((*sin6).sin6_addr), 3) as *mut c_void);
        if ret != 1 {
            return -1;
        }

        /* add ::ffff prefix */
        *s6_addr32(ptr::addr_of_mut!((*sin6).sin6_addr), 0) = 0;
        *s6_addr32(ptr::addr_of_mut!((*sin6).sin6_addr), 1) = 0;
        *s6_addr16(ptr::addr_of_mut!((*sin6).sin6_addr), 4) = 0;
        *s6_addr16(ptr::addr_of_mut!((*sin6).sin6_addr), 5) = 0xffff;
    }

    0
}

unsafe fn create_queues() -> *mut netdev_queue_id {
    let queues: *mut netdev_queue_id;
    let mut i: size_t = 0;

    queues = netdev_queue_id_alloc(num_queues);
    while i < num_queues as size_t {
        netdev_queue_id_set_type(queues.add(i), NETDEV_QUEUE_TYPE_RX);
        netdev_queue_id_set_id(queues.add(i), start_queue + i as c_int);
        i += 1;
    }
    queues
}

unsafe fn do_server(mem: *mut memory_buffer) -> c_int {
    let mut ring_config: *mut ethtool_rings_get_rsp = ptr::null_mut();
    let mut ctrl_data = [0 as c_char; size_of::<c_int>() * 20000];
    let mut non_page_aligned_frags: size_t = 0;
    let mut client_addr: sockaddr_in6 = zeroed();
    let mut server_sin: sockaddr_in6 = zeroed();
    let mut page_aligned_frags: size_t = 0;
    let mut total_received: size_t = 0;
    let mut client_addr_len: socklen_t;
    let mut is_devmem: bool = false;
    let mut tmp_mem: *mut c_char = ptr::null_mut();
    let mut ys: *mut ynl_sock = ptr::null_mut();
    let mut iobuf = [0 as c_char; 819200];
    let mut ret: c_int;
    let mut err: c_int = -1;
    let mut buffer = [0 as c_char; 256];
    let socket_fd: c_int;
    let client_fd: c_int;

    ret = parse_address(server_ip, atoi(port), &mut server_sin);
    if ret < 0 {
        pr_err(b"parse server address\0".as_ptr() as *const c_char);
        return -1;
    }

    if skip_config == 0 {
        ring_config = get_ring_config();
        if ring_config.is_null() {
            pr_err(b"Failed to get current ring configuration\0".as_ptr() as *const c_char);
            return -1;
        }
        if configure_headersplit(ring_config, true) != 0 {
            pr_err(b"Failed to enable TCP header split\0".as_ptr() as *const c_char);
            ethtool_rings_get_rsp_free(ring_config);
            return -1;
        }
        if configure_rss() != 0 {
            pr_err(b"Failed to configure rss\0".as_ptr() as *const c_char);
            restore_ring_config(ring_config);
            ethtool_rings_get_rsp_free(ring_config);
            return -1;
        }
        if configure_flow_steering(&mut server_sin) != 0 {
            pr_err(b"Failed to configure flow steering\0".as_ptr() as *const c_char);
            reset_rss();
            restore_ring_config(ring_config);
            ethtool_rings_get_rsp_free(ring_config);
            return -1;
        }
    }

    if bind_rx_queue(ifindex, (*mem).fd as c_uint, create_queues(), num_queues as c_uint, &mut ys) != 0 {
        pr_err(b"Failed to bind\0".as_ptr() as *const c_char);
        if skip_config == 0 {
            reset_flow_steering();
            reset_rss();
            restore_ring_config(ring_config);
            ethtool_rings_get_rsp_free(ring_config);
        }
        return -1;
    }

    tmp_mem = malloc((*mem).size) as *mut c_char;
    if tmp_mem.is_null() {
        ynl_sock_destroy(ys);
        return -1;
    }

    socket_fd = socket(AF_INET6, SOCK_STREAM, 0);
    if socket_fd < 0 || enable_reuseaddr(socket_fd) != 0 {
        free(tmp_mem as *mut c_void);
        ynl_sock_destroy(ys);
        return -1;
    }

    fprintf(stderr, b"binding to address %s:%d\n\0".as_ptr() as *const c_char, server_ip, ntohs(server_sin.sin6_port));
    ret = bind(socket_fd, &server_sin as *const _ as *const sockaddr, size_of::<sockaddr_in6>() as socklen_t);
    if ret != 0 {
        pr_err(b"Failed to bind\0".as_ptr() as *const c_char);
        close(socket_fd);
        free(tmp_mem as *mut c_void);
        ynl_sock_destroy(ys);
        return -1;
    }

    ret = listen(socket_fd, 1);
    if ret != 0 {
        pr_err(b"Failed to listen\0".as_ptr() as *const c_char);
        close(socket_fd);
        free(tmp_mem as *mut c_void);
        ynl_sock_destroy(ys);
        return -1;
    }

    client_addr_len = size_of::<sockaddr_in6>() as socklen_t;
    inet_ntop(AF_INET6, ptr::addr_of!(server_sin.sin6_addr) as *const c_void, buffer.as_mut_ptr(), buffer.len() as socklen_t);
    fprintf(stderr, b"Waiting or connection on %s:%d\n\0".as_ptr() as *const c_char, buffer.as_ptr(), ntohs(server_sin.sin6_port));
    client_fd = accept(socket_fd, &mut client_addr as *mut _ as *mut sockaddr, &mut client_addr_len);
    if client_fd < 0 {
        pr_err(b"Failed to accept\0".as_ptr() as *const c_char);
        close(socket_fd);
        free(tmp_mem as *mut c_void);
        ynl_sock_destroy(ys);
        return -1;
    }

    inet_ntop(AF_INET6, ptr::addr_of!(client_addr.sin6_addr) as *const c_void, buffer.as_mut_ptr(), buffer.len() as socklen_t);
    fprintf(stderr, b"Got connection from %s:%d\n\0".as_ptr() as *const c_char, buffer.as_ptr(), ntohs(client_addr.sin6_port));

    loop {
        let mut iov = iovec { iov_base: iobuf.as_mut_ptr() as *mut c_void, iov_len: iobuf.len() };
        let mut dmabuf_cmsg: *mut dmabuf_cmsg;
        let mut cm: *mut cmsghdr;
        let mut msg: msghdr = zeroed();
        let mut token: dmabuf_token = zeroed();
        let rret: ssize_t;

        is_devmem = false;
        msg.msg_iov = &mut iov;
        msg.msg_iovlen = 1;
        msg.msg_control = ctrl_data.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = ctrl_data.len();
        rret = recvmsg(client_fd, &mut msg, MSG_SOCK_DEVMEM);
        fprintf(stderr, b"recvmsg ret=%ld\n\0".as_ptr() as *const c_char, rret as c_long);
        if rret < 0 && (errno == EAGAIN || errno == EWOULDBLOCK) {
            continue;
        }
        if rret < 0 {
            perror(b"recvmsg\0".as_ptr() as *const c_char);
            if errno == EFAULT {
                pr_err(b"received EFAULT, won't recover\0".as_ptr() as *const c_char);
                break;
            }
            continue;
        }
        if rret == 0 {
            errno = 0;
            pr_err(b"client exited\0".as_ptr() as *const c_char);
            err = 0;
            break;
        }

        cm = CMSG_FIRSTHDR(&mut msg);
        while !cm.is_null() {
            if (*cm).cmsg_level != SOL_SOCKET || ((*cm).cmsg_type != SCM_DEVMEM_DMABUF && (*cm).cmsg_type != SCM_DEVMEM_LINEAR) {
                fprintf(stderr, b"skipping non-devmem cmsg\n\0".as_ptr() as *const c_char);
                cm = CMSG_NXTHDR(&mut msg, cm);
                continue;
            }

            dmabuf_cmsg = CMSG_DATA(cm) as *mut dmabuf_cmsg;
            is_devmem = true;

            if (*cm).cmsg_type == SCM_DEVMEM_LINEAR {
                /* TODO: process data copied from skb's linear
                 * buffer.
                 */
                fprintf(stderr, b"SCM_DEVMEM_LINEAR. dmabuf_cmsg->frag_size=%u\n\0".as_ptr() as *const c_char, (*dmabuf_cmsg).frag_size);
                if fail_on_linear {
                    pr_err(b"received SCM_DEVMEM_LINEAR but --fail-on-linear (-L) set\0".as_ptr() as *const c_char);
                    err = -1;
                    break;
                }
                cm = CMSG_NXTHDR(&mut msg, cm);
                continue;
            }

            token.token_start = (*dmabuf_cmsg).frag_token;
            token.token_count = 1;
            total_received += (*dmabuf_cmsg).frag_size as size_t;
            fprintf(
                stderr,
                b"received frag_page=%llu, in_page_offset=%llu, frag_offset=%llu, frag_size=%u, token=%u, total_received=%lu, dmabuf_id=%u\n\0".as_ptr() as *const c_char,
                (*dmabuf_cmsg).frag_offset >> PAGE_SHIFT,
                (*dmabuf_cmsg).frag_offset % getpagesize() as u64,
                (*dmabuf_cmsg).frag_offset,
                (*dmabuf_cmsg).frag_size,
                (*dmabuf_cmsg).frag_token,
                total_received as c_ulong,
                (*dmabuf_cmsg).dmabuf_id,
            );

            if (*dmabuf_cmsg).dmabuf_id != dmabuf_id {
                pr_err(b"received on wrong dmabuf_id: flow steering error\0".as_ptr() as *const c_char);
                err = -1;
                break;
            }

            if ((*dmabuf_cmsg).frag_size as c_int % getpagesize()) != 0 {
                non_page_aligned_frags += 1;
            } else {
                page_aligned_frags += 1;
            }

            ((*provider).memcpy_from_device)(tmp_mem as *mut c_void, mem, (*dmabuf_cmsg).frag_offset as size_t, (*dmabuf_cmsg).frag_size as c_int);

            if do_validation != 0 {
                if validate_buffer(tmp_mem as *mut c_void, (*dmabuf_cmsg).frag_size as size_t) != 0 {
                    err = -1;
                    break;
                }
            } else {
                print_nonzero_bytes(tmp_mem as *mut c_void, (*dmabuf_cmsg).frag_size as size_t);
            }

            ret = setsockopt(client_fd, SOL_SOCKET, SO_DEVMEM_DONTNEED, &token as *const _ as *const c_void, size_of::<dmabuf_token>() as socklen_t);
            if ret != 1 {
                pr_err(b"SO_DEVMEM_DONTNEED not enough tokens\0".as_ptr() as *const c_char);
                err = -1;
                break;
            }
            cm = CMSG_NXTHDR(&mut msg, cm);
        }
        if err == -1 && !is_devmem {
            pr_err(b"flow steering error\0".as_ptr() as *const c_char);
            break;
        }
        if err == -1 {
            break;
        }

        fprintf(stderr, b"total_received=%lu\n\0".as_ptr() as *const c_char, total_received as c_ulong);
    }

    fprintf(stderr, b"%s: ok\n\0".as_ptr() as *const c_char, TEST_PREFIX.as_ptr());
    fprintf(stderr, b"page_aligned_frags=%lu, non_page_aligned_frags=%lu\n\0".as_ptr() as *const c_char, page_aligned_frags as c_ulong, non_page_aligned_frags as c_ulong);

    close(client_fd);
    close(socket_fd);
    free(tmp_mem as *mut c_void);
    ynl_sock_destroy(ys);
    if skip_config == 0 {
        reset_flow_steering();
        reset_rss();
        restore_ring_config(ring_config);
        ethtool_rings_get_rsp_free(ring_config);
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn run_devmem_tests() -> c_int {
    let ring_config: *mut ethtool_rings_get_rsp;
    let mut queues: *mut netdev_queue_id;
    let mem: *mut memory_buffer;
    let mut ys: *mut ynl_sock = ptr::null_mut();
    let mut err: c_int = -1;

    mem = ((*provider).alloc)(getpagesize() as size_t * NUM_PAGES);
    if mem.is_null() {
        pr_err(b"Failed to allocate memory buffer\0".as_ptr() as *const c_char);
        return -1;
    }

    ring_config = get_ring_config();
    if ring_config.is_null() {
        pr_err(b"Failed to get current ring configuration\0".as_ptr() as *const c_char);
        ((*provider).free)(mem);
        return -1;
    }

    /* Configure RSS to divert all traffic from our devmem queues */
    if configure_rss() != 0 {
        pr_err(b"rss error\0".as_ptr() as *const c_char);
        ethtool_rings_get_rsp_free(ring_config);
        ((*provider).free)(mem);
        return -1;
    }

    if configure_headersplit(ring_config, true) != 0 {
        pr_err(b"Failed to configure header split\0".as_ptr() as *const c_char);
        reset_rss();
        ethtool_rings_get_rsp_free(ring_config);
        ((*provider).free)(mem);
        return -1;
    }

    queues = netdev_queue_id_alloc(num_queues);
    if queues.is_null() {
        pr_err(b"Failed to allocate empty queues array\0".as_ptr() as *const c_char);
        restore_ring_config(ring_config);
        reset_rss();
        ethtool_rings_get_rsp_free(ring_config);
        ((*provider).free)(mem);
        return -1;
    }

    if bind_rx_queue(ifindex, (*mem).fd as c_uint, queues, num_queues as c_uint, &mut ys) == 0 {
        pr_err(b"Binding empty queues array should have failed\0".as_ptr() as *const c_char);
        ynl_sock_destroy(ys);
        restore_ring_config(ring_config);
        reset_rss();
        ethtool_rings_get_rsp_free(ring_config);
        ((*provider).free)(mem);
        return -1;
    }

    if configure_headersplit(ring_config, false) != 0 {
        pr_err(b"Failed to configure header split\0".as_ptr() as *const c_char);
        restore_ring_config(ring_config);
        reset_rss();
        ethtool_rings_get_rsp_free(ring_config);
        ((*provider).free)(mem);
        return -1;
    }

    queues = create_queues();
    if queues.is_null() {
        pr_err(b"Failed to create queues\0".as_ptr() as *const c_char);
        restore_ring_config(ring_config);
        reset_rss();
        ethtool_rings_get_rsp_free(ring_config);
        ((*provider).free)(mem);
        return -1;
    }

    if bind_rx_queue(ifindex, (*mem).fd as c_uint, queues, num_queues as c_uint, &mut ys) == 0 {
        pr_err(b"Configure dmabuf with header split off should have failed\0".as_ptr() as *const c_char);
        ynl_sock_destroy(ys);
        restore_ring_config(ring_config);
        reset_rss();
        ethtool_rings_get_rsp_free(ring_config);
        ((*provider).free)(mem);
        return -1;
    }

    if configure_headersplit(ring_config, true) != 0 {
        pr_err(b"Failed to configure header split\0".as_ptr() as *const c_char);
        restore_ring_config(ring_config);
        reset_rss();
        ethtool_rings_get_rsp_free(ring_config);
        ((*provider).free)(mem);
        return -1;
    }

    queues = create_queues();
    if queues.is_null() {
        pr_err(b"Failed to create queues\0".as_ptr() as *const c_char);
        restore_ring_config(ring_config);
        reset_rss();
        ethtool_rings_get_rsp_free(ring_config);
        ((*provider).free)(mem);
        return -1;
    }

    if bind_rx_queue(ifindex, (*mem).fd as c_uint, queues, num_queues as c_uint, &mut ys) != 0 {
        pr_err(b"Failed to bind\0".as_ptr() as *const c_char);
        restore_ring_config(ring_config);
        reset_rss();
        ethtool_rings_get_rsp_free(ring_config);
        ((*provider).free)(mem);
        return -1;
    }

    /* Deactivating a bound queue should not be legal */
    if check_changing_channels(num_queues as c_uint, num_queues as c_uint) == 0 {
        pr_err(b"Deactivating a bound queue should be illegal\0".as_ptr() as *const c_char);
    } else {
        err = 0;
    }

    ynl_sock_destroy(ys);
    restore_ring_config(ring_config);
    reset_rss();
    ethtool_rings_get_rsp_free(ring_config);
    ((*provider).free)(mem);
    err
}

unsafe fn gettimeofday_ms() -> uint64_t {
    let mut tv: timeval = zeroed();
    gettimeofday(&mut tv, ptr::null_mut());
    (tv.tv_sec as uint64_t * 1000u64) + (tv.tv_usec as uint64_t / 1000u64)
}

unsafe fn do_poll(fd: c_int) -> c_int {
    let mut pfd: pollfd = zeroed();
    let ret: c_int;

    pfd.revents = 0;
    pfd.fd = fd;
    ret = poll(&mut pfd, 1, waittime_ms);
    if ret == -1 {
        pr_err(b"poll\0".as_ptr() as *const c_char);
        return -1;
    }
    (ret != 0 && (pfd.revents & POLLERR) != 0) as c_int
}

unsafe fn wait_compl(fd: c_int) -> c_int {
    let tstop: int64_t = gettimeofday_ms() as int64_t + waittime_ms as int64_t;
    let mut control = [0 as c_char; 128];
    let mut serr: *mut sock_extended_err;
    let mut msg: msghdr = zeroed();
    let mut cm: *mut cmsghdr;
    let mut hi: __u32;
    let mut lo: __u32;
    let mut ret: c_int;

    msg.msg_control = control.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = control.len();

    while (gettimeofday_ms() as int64_t) < tstop {
        ret = do_poll(fd);
        if ret < 0 {
            return ret;
        }
        if ret == 0 {
            continue;
        }

        ret = recvmsg(fd, &mut msg, MSG_ERRQUEUE) as c_int;
        if ret < 0 {
            if errno == EAGAIN {
                continue;
            }
            pr_err(b"recvmsg(MSG_ERRQUEUE)\0".as_ptr() as *const c_char);
            return -1;
        }
        if (msg.msg_flags & MSG_CTRUNC) != 0 {
            pr_err(b"MSG_CTRUNC\0".as_ptr() as *const c_char);
            return -1;
        }

        cm = CMSG_FIRSTHDR(&mut msg);
        while !cm.is_null() {
            if (*cm).cmsg_level != SOL_IP && (*cm).cmsg_level != SOL_IPV6 {
                cm = CMSG_NXTHDR(&mut msg, cm);
                continue;
            }
            if (*cm).cmsg_level == SOL_IP && (*cm).cmsg_type != IP_RECVERR {
                cm = CMSG_NXTHDR(&mut msg, cm);
                continue;
            }
            if (*cm).cmsg_level == SOL_IPV6 && (*cm).cmsg_type != IPV6_RECVERR {
                cm = CMSG_NXTHDR(&mut msg, cm);
                continue;
            }

            serr = CMSG_DATA(cm) as *mut sock_extended_err;
            if (*serr).ee_origin != SO_EE_ORIGIN_ZEROCOPY {
                pr_err(b"wrong origin %u\0".as_ptr() as *const c_char, (*serr).ee_origin as c_uint);
                return -1;
            }
            if (*serr).ee_errno != 0 {
                pr_err(b"wrong errno %d\0".as_ptr() as *const c_char, (*serr).ee_errno as c_int);
                return -1;
            }

            hi = (*serr).ee_data;
            lo = (*serr).ee_info;
            fprintf(stderr, b"tx complete [%d,%d]\n\0".as_ptr() as *const c_char, lo, hi);
            return 0;
        }
    }

    pr_err(b"did not receive tx completion\0".as_ptr() as *const c_char);
    -1
}

unsafe fn do_client(mem: *mut memory_buffer) -> c_int {
    let mut ctrl_data = [0 as c_char; 32];
    let mut server_sin: sockaddr_in6 = zeroed();
    let mut client_sin: sockaddr_in6 = zeroed();
    let mut ys: *mut ynl_sock = ptr::null_mut();
    let mut iov: [iovec; MAX_IOV] = core::array::from_fn(|_| iovec { iov_base: ptr::null_mut(), iov_len: 0 });
    let mut msg: msghdr = zeroed();
    let mut line_size: ssize_t = 0;
    let mut cmsg: *mut cmsghdr;
    let mut line: *mut c_char = ptr::null_mut();
    let mut ret: c_int;
    let mut err: c_int = -1;
    let mut len: size_t = 0;
    let socket_fd: c_int;
    let mut ddmabuf: __u32;
    let opt: c_int = 1;

    ret = parse_address(server_ip, atoi(port), &mut server_sin);
    if ret < 0 {
        pr_err(b"parse server address\0".as_ptr() as *const c_char);
        return -1;
    }

    if !client_ip.is_null() {
        ret = parse_address(client_ip, atoi(port), &mut client_sin);
        if ret < 0 {
            pr_err(b"parse client address\0".as_ptr() as *const c_char);
            return ret;
        }
    }

    socket_fd = socket(AF_INET6, SOCK_STREAM, 0);
    if socket_fd < 0 {
        pr_err(b"create socket\0".as_ptr() as *const c_char);
        return -1;
    }

    if enable_reuseaddr(socket_fd) != 0 {
        close(socket_fd);
        return -1;
    }

    ret = setsockopt(socket_fd, SOL_SOCKET, SO_BINDTODEVICE, ifname as *const c_void, (strlen(ifname) + 1) as socklen_t);
    if ret != 0 {
        pr_err(b"bindtodevice\0".as_ptr() as *const c_char);
        close(socket_fd);
        return -1;
    }

    if bind_tx_queue(ifindex, (*mem).fd as c_uint, &mut ys) != 0 {
        pr_err(b"Failed to bind\0".as_ptr() as *const c_char);
        close(socket_fd);
        return -1;
    }

    if !client_ip.is_null() {
        ret = bind(socket_fd, &client_sin as *const _ as *const sockaddr, size_of::<sockaddr_in6>() as socklen_t);
        if ret != 0 {
            pr_err(b"bind\0".as_ptr() as *const c_char);
            ynl_sock_destroy(ys);
            close(socket_fd);
            return -1;
        }
    }

    ret = setsockopt(socket_fd, SOL_SOCKET, SO_ZEROCOPY, &opt as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
    if ret != 0 {
        pr_err(b"set sock opt\0".as_ptr() as *const c_char);
        ynl_sock_destroy(ys);
        close(socket_fd);
        return -1;
    }

    fprintf(stderr, b"Connect to %s %d (via %s)\n\0".as_ptr() as *const c_char, server_ip, ntohs(server_sin.sin6_port), ifname);
    ret = connect(socket_fd, &server_sin as *const _ as *const sockaddr, size_of::<sockaddr_in6>() as socklen_t);
    if ret != 0 {
        pr_err(b"connect\0".as_ptr() as *const c_char);
        ynl_sock_destroy(ys);
        close(socket_fd);
        return -1;
    }

    loop {
        free(line as *mut c_void);
        line = ptr::null_mut();
        line_size = getline(&mut line, &mut len, stdin);

        if line_size < 0 {
            break;
        }

        if max_chunk != 0 {
            msg.msg_iovlen = ((line_size as size_t + max_chunk - 1) / max_chunk) as size_t;
            if msg.msg_iovlen > MAX_IOV {
                pr_err(b"can't partition %zd bytes into maximum of %d chunks\0".as_ptr() as *const c_char, line_size, MAX_IOV as c_int);
                break;
            }

            let mut i: size_t = 0;
            while i < msg.msg_iovlen {
                iov[i].iov_base = (i * max_chunk) as *mut c_void;
                iov[i].iov_len = max_chunk;
                i += 1;
            }

            iov[msg.msg_iovlen - 1].iov_len = line_size as size_t - (msg.msg_iovlen - 1) * max_chunk;
        } else {
            iov[0].iov_base = ptr::null_mut();
            iov[0].iov_len = line_size as size_t;
            msg.msg_iovlen = 1;
        }

        msg.msg_iov = iov.as_mut_ptr();
        ((*provider).memcpy_to_device)(mem, 0, line as *mut c_void, line_size as c_int);

        msg.msg_control = ctrl_data.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = ctrl_data.len();
        cmsg = CMSG_FIRSTHDR(&mut msg);
        (*cmsg).cmsg_level = SOL_SOCKET;
        (*cmsg).cmsg_type = SCM_DEVMEM_DMABUF;
        (*cmsg).cmsg_len = CMSG_LEN(size_of::<__u32>());

        ddmabuf = tx_dmabuf_id;
        *(CMSG_DATA(cmsg) as *mut __u32) = ddmabuf;

        ret = sendmsg(socket_fd, &msg, MSG_ZEROCOPY) as c_int;
        if ret < 0 {
            pr_err(b"Failed sendmsg\0".as_ptr() as *const c_char);
            break;
        }

        fprintf(stderr, b"sendmsg_ret=%d\n\0".as_ptr() as *const c_char, ret);
        if ret as ssize_t != line_size {
            pr_err(b"Did not send all bytes %d vs %zd\0".as_ptr() as *const c_char, ret, line_size);
            break;
        }

        if wait_compl(socket_fd) != 0 {
            break;
        }
    }

    fprintf(stderr, b"%s: tx ok\n\0".as_ptr() as *const c_char, TEST_PREFIX.as_ptr());
    err = 0;

    free(line as *mut c_void);
    ynl_sock_destroy(ys);
    close(socket_fd);
    err
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mem: *mut memory_buffer;
    let mut is_server: c_int = 0;
    let mut opt: c_int;
    let mut ret: c_int;
    let mut err: c_int = 1;

    loop {
        opt = getopt(argc, argv, b"Lls:c:p:v:q:t:f:z:nb:\0".as_ptr() as *const c_char);
        if opt == -1 {
            break;
        }
        match opt as u8 as char {
            'L' => fail_on_linear = true,
            'l' => is_server = 1,
            's' => server_ip = optarg,
            'c' => client_ip = optarg,
            'p' => port = optarg,
            'v' => do_validation = atoll(optarg) as size_t,
            'q' => num_queues = atoi(optarg),
            't' => start_queue = atoi(optarg),
            'f' => ifname = optarg,
            'z' => max_chunk = atoi(optarg) as size_t,
            'n' => skip_config = 1,
            'b' => {
                let val: c_ulong;
                errno = 0;
                val = strtoul(optarg, ptr::null_mut(), 0);
                if (val == ULONG_MAX && errno == ERANGE) || val > UINT32_MAX {
                    pr_err(b"invalid rx_page_size: %s\0".as_ptr() as *const c_char, optarg);
                    return 1;
                }
                rx_page_size = val as uint32_t;
            }
            '?' => {
                fprintf(stderr, b"unknown option: %c\n\0".as_ptr() as *const c_char, optopt);
            }
            _ => {}
        }
    }

    if ifname.is_null() {
        pr_err(b"Missing -f argument\0".as_ptr() as *const c_char);
        return 1;
    }

    ifindex = if_nametoindex(ifname);
    fprintf(stderr, b"using ifindex=%u\n\0".as_ptr() as *const c_char, ifindex);

    if server_ip.is_null() && client_ip.is_null() {
        if start_queue < 0 && num_queues < 0 {
            num_queues = rxq_num(ifindex as c_int);
            if num_queues < 0 {
                pr_err(b"couldn't detect number of queues\0".as_ptr() as *const c_char);
                return 1;
            }
            if num_queues < 2 {
                pr_err(b"number of device queues is too low\0".as_ptr() as *const c_char);
                return 1;
            }
            /* make sure can bind to multiple queues */
            start_queue = num_queues / 2;
            num_queues /= 2;
        }

        if start_queue < 0 || num_queues < 0 {
            pr_err(b"Both -t and -q are required\0".as_ptr() as *const c_char);
            return 1;
        }

        return run_devmem_tests();
    }

    if start_queue < 0 && num_queues < 0 {
        num_queues = rxq_num(ifindex as c_int);
        if num_queues < 2 {
            pr_err(b"number of device queues is too low\0".as_ptr() as *const c_char);
            return 1;
        }

        num_queues = 1;
        start_queue = rxq_num(ifindex as c_int) - num_queues;
        if start_queue < 0 {
            pr_err(b"couldn't detect number of queues\0".as_ptr() as *const c_char);
            return 1;
        }

        fprintf(stderr, b"using queues %d..%d\n\0".as_ptr() as *const c_char, start_queue, start_queue + num_queues);
    }

    while optind < argc {
        fprintf(stderr, b"extra arguments: %s\n\0".as_ptr() as *const c_char, *argv.add(optind as usize));
        optind += 1;
    }

    if start_queue < 0 {
        pr_err(b"Missing -t argument\0".as_ptr() as *const c_char);
        return 1;
    }
    if num_queues < 0 {
        pr_err(b"Missing -q argument\0".as_ptr() as *const c_char);
        return 1;
    }
    if server_ip.is_null() {
        pr_err(b"Missing -s argument\0".as_ptr() as *const c_char);
        return 1;
    }
    if port.is_null() {
        pr_err(b"Missing -p argument\0".as_ptr() as *const c_char);
        return 1;
    }

    mem = ((*provider).alloc)(getpagesize() as size_t * NUM_PAGES);
    if mem.is_null() {
        pr_err(b"Failed to allocate memory buffer\0".as_ptr() as *const c_char);
        return 1;
    }

    ret = if is_server != 0 { do_server(mem) } else { do_client(mem) };
    if ret == 0 {
        err = 0;
    }

    ((*provider).free)(mem);
    err
}
