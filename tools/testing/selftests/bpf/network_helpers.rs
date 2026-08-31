// SPDX-License-Identifier: GPL-2.0-only
// Translated from network_helpers.c. C includes are represented by external
// declarations and constants expected from the surrounding build.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

pub type size_t = usize;
pub type ssize_t = isize;
pub type socklen_t = u32;
pub type sa_family_t = u16;
pub type __u16 = u16;
pub type __u64 = u64;
pub type u8 = u8;
pub type u16 = u16;
pub type u32 = u32;
pub type pthread_t = c_ulong;
pub type va_list = *mut c_void;
pub type u_char = u8;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOCK_TYPE_MASK: c_int = 0xf;
const SOL_SOCKET: c_int = 1;
const SO_RCVTIMEO: c_int = 20;
const SO_SNDTIMEO: c_int = 21;
const SO_REUSEADDR: c_int = 2;
const SO_REUSEPORT: c_int = 15;
const SO_TYPE: c_int = 3;
const SO_PROTOCOL: c_int = 38;
const MSG_FASTOPEN: c_int = 0x20000000;
const IPPROTO_TCP: c_int = 6;
const IPPROTO_UDP: c_int = 17;
const IPPROTO_ICMP: c_int = 1;
const IPPROTO_ICMPV6: c_int = 58;
const IPPROTO_MPTCP: c_int = 262;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86DD;
const MAGIC_BYTES: u16 = 123;
const PATH_MAX: usize = 4096;
const IF_NAMESIZE: usize = 16;
const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_CLOEXEC: c_int = 0o2000000;
const O_NONBLOCK: c_int = 0o0004000;
const F_SETFL: c_int = 4;
const CLONE_NEWNET: c_int = 0x40000000;
const IFF_TUN: c_int = 0x0001;
const IFF_TAP: c_int = 0x0002;
const IFF_NO_PI: c_int = 0x1000;
const TUNSETIFF: c_ulong = 0x400454ca;
const SIOCETHTOOL: c_ulong = 0x8946;
const ETHTOOL_GRINGPARAM: u32 = 0x00000010;
const ETHTOOL_SRINGPARAM: u32 = 0x00000011;
const EINTR: c_int = 4;
const E2BIG: c_int = 7;
const EEXIST: c_int = 17;
const DLT_LINUX_SLL2: c_int = 276;
const PCAP_ERRBUF_SIZE: usize = 256;
const MAX_FLAGS_STRLEN: usize = 21;
const PCAP_DIR: &[u8] = b"/tmp/tmon_pcap\0";

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
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
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __data: [u8; 126],
}

#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [c_char; 108],
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: u8,
    pub tos: u8,
    pub tot_len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: u16,
    pub saddr: u32,
    pub daddr: u32,
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: u8,
    pub flow_lbl: [u8; 3],
    pub payload_len: u16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
pub struct tcphdr {
    pub source: u16,
    pub dest: u16,
    pub seq: u32,
    pub ack_seq: u32,
    pub doff_res_flags: u16,
    pub window: u16,
    pub check: u16,
    pub urg_ptr: u16,
}

impl tcphdr {
    unsafe fn fin(&self) -> bool {
        ntohs(self.doff_res_flags) & 0x0001 != 0
    }
    unsafe fn syn(&self) -> bool {
        ntohs(self.doff_res_flags) & 0x0002 != 0
    }
    unsafe fn rst(&self) -> bool {
        ntohs(self.doff_res_flags) & 0x0004 != 0
    }
    unsafe fn ack(&self) -> bool {
        ntohs(self.doff_res_flags) & 0x0010 != 0
    }
}

#[repr(C)]
pub struct udphdr {
    pub source: u16,
    pub dest: u16,
    pub len: u16,
    pub check: u16,
}

#[repr(C)]
pub struct ipv4_packet {
    pub eth: ethhdr,
    pub iph: iphdr,
    pub tcp: tcphdr,
}

#[repr(C)]
pub struct ipv6_packet {
    pub eth: ethhdr,
    pub iph: ipv6hdr,
    pub tcp: tcphdr,
}

#[repr(C)]
pub struct network_helper_opts {
    pub timeout_ms: c_int,
    pub proto: c_int,
    pub backlog: c_int,
    pub post_socket_cb: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
    pub cb_opts: *mut c_void,
}

#[repr(C)]
pub struct nstoken {
    pub orig_netns_fd: c_int,
}

#[repr(C)]
pub struct ifreq {
    pub ifr_name: [c_char; IF_NAMESIZE],
    pub ifr_data: *mut c_char,
}

#[repr(C)]
pub struct ethtool_ringparam {
    pub cmd: u32,
}

#[repr(C)]
pub struct send_recv_arg {
    pub fd: c_int,
    pub bytes: u32,
    pub stop: c_int,
}

#[repr(C)]
pub struct fd_set {
    pub fds_bits: [c_long; 16],
}

#[repr(C)]
pub struct pcap_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pcap_dumper_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pcap_pkthdr {
    pub ts: timeval,
    pub caplen: u32,
    pub len: u32,
}

#[repr(C)]
pub struct tmonitor_ctx {
    pub pcap: *mut pcap_t,
    pub dumper: *mut pcap_dumper_t,
    pub thread: pthread_t,
    pub wake_fd: c_int,
    pub done: bool,
    pub pkt_fname: [c_char; PATH_MAX],
    pub pcap_fd: c_int,
}

pub type tm_print_fn_t = Option<unsafe extern "C" fn(*const c_char, va_list) -> c_int>;

unsafe extern "C" {
    static mut errno: c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut c_void, format: *const c_char, arg: va_list) -> c_int;
    static mut stderr: *mut c_void;
    static mut stdout: *mut c_void;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(socket: c_int, level: c_int, option_name: c_int, option_value: *const c_void, option_len: socklen_t) -> c_int;
    fn getsockopt(socket: c_int, level: c_int, option_name: c_int, option_value: *mut c_void, option_len: *mut socklen_t) -> c_int;
    fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn listen(socket: c_int, backlog: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
    fn sendto(socket: c_int, message: *const c_void, length: size_t, flags: c_int, dest_addr: *const sockaddr, dest_len: socklen_t) -> ssize_t;
    fn send(socket: c_int, buffer: *const c_void, length: size_t, flags: c_int) -> ssize_t;
    fn recv(socket: c_int, buffer: *mut c_void, length: size_t, flags: c_int) -> ssize_t;
    fn connect(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char, size: socklen_t) -> *const c_char;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;
    fn ntohl(netlong: u32) -> u32;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;
    fn sys_gettid() -> c_long;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: size_t) -> ssize_t;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
    fn pcap_create(source: *const c_char, errbuf: *mut c_char) -> *mut pcap_t;
    fn pcap_set_snaplen(p: *mut pcap_t, snaplen: c_int) -> c_int;
    fn pcap_set_immediate_mode(p: *mut pcap_t, immediate_mode: c_int) -> c_int;
    fn pcap_setnonblock(p: *mut pcap_t, nonblock: c_int, errbuf: *mut c_char) -> c_int;
    fn pcap_activate(p: *mut pcap_t) -> c_int;
    fn pcap_set_datalink(p: *mut pcap_t, dlt: c_int) -> c_int;
    fn pcap_geterr(p: *mut pcap_t) -> *const c_char;
    fn pcap_close(p: *mut pcap_t);
    fn pcap_get_selectable_fd(p: *mut pcap_t) -> c_int;
    fn pcap_dump_open(p: *mut pcap_t, fname: *const c_char) -> *mut pcap_dumper_t;
    fn pcap_dump_close(p: *mut pcap_dumper_t);
    fn pcap_dump(user: *mut u_char, h: *const pcap_pkthdr, sp: *const u_char);
    fn pcap_next(p: *mut pcap_t, h: *mut pcap_pkthdr) -> *const u_char;
    fn eventfd(initval: c_uint, flags: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *mut timeval) -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn getpid() -> c_int;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
}

#[repr(C)]
pub struct bpf_tc_hook {
    pub sz: size_t,
    pub ifindex: c_int,
    pub attach_point: c_int,
}

#[repr(C)]
pub struct bpf_tc_opts {
    pub sz: size_t,
    pub handle: u32,
    pub priority: u32,
    pub prog_fd: c_int,
}

const BPF_TC_INGRESS: c_int = 1;
const BPF_TC_EGRESS: c_int = 2;

unsafe fn clean_errno() -> *mut c_char {
    if errno == 0 {
        c"None".as_ptr() as *mut c_char
    } else {
        strerror(errno)
    }
}

macro_rules! log_err {
    ($msg:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            let __save = errno;
            fprintf(
                stderr,
                concat!("(%s:%d: errno: %s) ", $msg, "\n\0").as_ptr() as *const c_char,
                file!().as_ptr() as *const c_char,
                line!() as c_int,
                clean_errno()
                $(, $arg)*,
            );
            errno = __save;
        }
    }};
}

unsafe fn save_errno_close(fd: c_int) {
    let __save = errno;
    close(fd);
    errno = __save;
}

fn bpf_constant_htons(x: u16) -> u16 {
    x.to_be()
}

#[unsafe(no_mangle)]
pub static mut pkt_v4: ipv4_packet = ipv4_packet {
    eth: ethhdr {
        h_dest: [0; 6],
        h_source: [0; 6],
        h_proto: ETH_P_IP.to_be(),
    },
    iph: iphdr {
        ihl_version: 5,
        tos: 0,
        tot_len: MAGIC_BYTES.to_be(),
        id: 0,
        frag_off: 0,
        ttl: 0,
        protocol: IPPROTO_TCP as u8,
        check: 0,
        saddr: 0,
        daddr: 0,
    },
    tcp: tcphdr {
        source: 0,
        dest: 0,
        seq: 0,
        ack_seq: 0,
        doff_res_flags: 5u16 << 12,
        window: 0,
        check: 0,
        urg_ptr: 123,
    },
};

#[unsafe(no_mangle)]
pub static mut pkt_v6: ipv6_packet = ipv6_packet {
    eth: ethhdr {
        h_dest: [0; 6],
        h_source: [0; 6],
        h_proto: ETH_P_IPV6.to_be(),
    },
    iph: ipv6hdr {
        priority_version: 0,
        flow_lbl: [0; 3],
        payload_len: MAGIC_BYTES.to_be(),
        nexthdr: IPPROTO_TCP as u8,
        hop_limit: 0,
        saddr: in6_addr { s6_addr: [0; 16] },
        daddr: in6_addr { s6_addr: [0; 16] },
    },
    tcp: tcphdr {
        source: 0,
        dest: 0,
        seq: 0,
        ack_seq: 0,
        doff_res_flags: 5u16 << 12,
        window: 0,
        check: 0,
        urg_ptr: 123,
    },
};

static default_opts: network_helper_opts = network_helper_opts {
    timeout_ms: 0,
    proto: 0,
    backlog: 0,
    post_socket_cb: None,
    cb_opts: ptr::null_mut(),
};

unsafe fn max_i(a: c_int, b: c_int) -> c_int {
    if a > b { a } else { b }
}

unsafe fn min_usize(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

unsafe fn err_ptr(err: c_int) -> *mut c_void {
    err as isize as *mut c_void
}

unsafe fn is_err(ptr: *mut c_void) -> bool {
    (ptr as usize) >= (!4095usize)
}

unsafe fn ptr_err(ptr: *mut c_void) -> c_long {
    ptr as isize as c_long
}

unsafe fn ASSERT_GE(a: c_int, b: c_int, _msg: *const c_char) -> bool {
    a >= b
}

unsafe fn ASSERT_OK(a: c_int, _msg: *const c_char) -> bool {
    a == 0
}

unsafe fn ASSERT_TRUE(a: bool, _msg: *const c_char) -> bool {
    a
}

unsafe fn ASSERT_NEQ(a: c_uint, b: c_uint, _msg: *const c_char) -> bool {
    a != b
}

unsafe fn READ_ONCE_i(v: *const c_int) -> c_int {
    ptr::read_volatile(v)
}

unsafe fn WRITE_ONCE_i(v: *mut c_int, val: c_int) {
    ptr::write_volatile(v, val);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn settimeo(fd: c_int, timeout_ms: c_int) -> c_int {
    let mut timeout = timeval { tv_sec: 3, tv_usec: 0 };

    if timeout_ms > 0 {
        timeout.tv_sec = (timeout_ms / 1000) as c_long;
        timeout.tv_usec = ((timeout_ms % 1000) * 1000) as c_long;
    }

    if setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &timeout as *const _ as *const c_void, size_of::<timeval>() as socklen_t) != 0 {
        log_err!("Failed to set SO_RCVTIMEO");
        return -1;
    }

    if setsockopt(fd, SOL_SOCKET, SO_SNDTIMEO, &timeout as *const _ as *const c_void, size_of::<timeval>() as socklen_t) != 0 {
        log_err!("Failed to set SO_SNDTIMEO");
        return -1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_server_addr(type_: c_int, mut addr: *const sockaddr_storage, addrlen: socklen_t, mut opts: *const network_helper_opts) -> c_int {
    let on: c_int = 1;
    let fd: c_int;

    if opts.is_null() {
        opts = &default_opts;
    }

    fd = socket((*addr).ss_family as c_int, type_, (*opts).proto);
    if fd < 0 {
        log_err!("Failed to create server socket");
        return -1;
    }

    if settimeo(fd, (*opts).timeout_ms) != 0 {
        goto_error_close(fd);
        return -1;
    }

    if (type_ & SOCK_TYPE_MASK) == SOCK_STREAM
        && setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &on as *const _ as *const c_void, size_of::<c_int>() as socklen_t) != 0
    {
        log_err!("Failed to enable SO_REUSEADDR");
        goto_error_close(fd);
        return -1;
    }

    if let Some(cb) = (*opts).post_socket_cb {
        if cb(fd, (*opts).cb_opts) != 0 {
            log_err!("Failed to call post_socket_cb");
            goto_error_close(fd);
            return -1;
        }
    }

    if bind(fd, addr as *const sockaddr, addrlen) < 0 {
        log_err!("Failed to bind socket");
        goto_error_close(fd);
        return -1;
    }

    if (type_ & SOCK_TYPE_MASK) == SOCK_STREAM {
        if listen(fd, if (*opts).backlog != 0 { max_i((*opts).backlog, 0) } else { 1 }) < 0 {
            log_err!("Failed to listed on socket");
            goto_error_close(fd);
            return -1;
        }
    }

    fd
}

unsafe fn goto_error_close(fd: c_int) {
    save_errno_close(fd);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_server_str(family: c_int, type_: c_int, addr_str: *const c_char, port: __u16, mut opts: *const network_helper_opts) -> c_int {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut addrlen: socklen_t = 0;

    if opts.is_null() {
        opts = &default_opts;
    }

    if make_sockaddr(family, addr_str, port, &mut addr, &mut addrlen) != 0 {
        return -1;
    }

    start_server_addr(type_, &addr, addrlen, opts)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_server(family: c_int, type_: c_int, addr_str: *const c_char, port: __u16, timeout_ms: c_int) -> c_int {
    let opts = network_helper_opts {
        timeout_ms,
        proto: 0,
        backlog: 0,
        post_socket_cb: None,
        cb_opts: ptr::null_mut(),
    };

    start_server_str(family, type_, addr_str, port, &opts)
}

unsafe extern "C" fn reuseport_cb(fd: c_int, _opts: *mut c_void) -> c_int {
    let on: c_int = 1;
    setsockopt(fd, SOL_SOCKET, SO_REUSEPORT, &on as *const _ as *const c_void, size_of::<c_int>() as socklen_t)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_reuseport_server(family: c_int, type_: c_int, addr_str: *const c_char, port: __u16, timeout_ms: c_int, nr_listens: c_uint) -> *mut c_int {
    let opts = network_helper_opts {
        timeout_ms,
        proto: 0,
        backlog: 0,
        post_socket_cb: Some(reuseport_cb),
        cb_opts: ptr::null_mut(),
    };
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut nr_fds: c_uint = 0;
    let mut addrlen: socklen_t = 0;
    let fds: *mut c_int;

    if nr_listens == 0 {
        return ptr::null_mut();
    }

    if make_sockaddr(family, addr_str, port, &mut addr, &mut addrlen) != 0 {
        return ptr::null_mut();
    }

    fds = malloc(size_of::<c_int>() * nr_listens as usize) as *mut c_int;
    if fds.is_null() {
        return ptr::null_mut();
    }

    *fds.add(0) = start_server_addr(type_, &addr, addrlen, &opts);
    if *fds.add(0) == -1 {
        free_fds(fds, nr_fds);
        return ptr::null_mut();
    }
    nr_fds = 1;

    if getsockname(*fds.add(0), &mut addr as *mut _ as *mut sockaddr, &mut addrlen) != 0 {
        free_fds(fds, nr_fds);
        return ptr::null_mut();
    }

    while nr_fds < nr_listens {
        *fds.add(nr_fds as usize) = start_server_addr(type_, &addr, addrlen, &opts);
        if *fds.add(nr_fds as usize) == -1 {
            free_fds(fds, nr_fds);
            return ptr::null_mut();
        }
        nr_fds += 1;
    }

    fds
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_fds(fds: *mut c_int, mut nr_close_fds: c_uint) {
    if !fds.is_null() {
        while nr_close_fds != 0 {
            nr_close_fds -= 1;
            close(*fds.add(nr_close_fds as usize));
        }
        free(fds as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fastopen_connect(server_fd: c_int, data: *const c_char, data_len: c_uint, timeout_ms: c_int) -> c_int {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut addrlen: socklen_t = size_of::<sockaddr_storage>() as socklen_t;
    let addr_in: *mut sockaddr_in;
    let fd: c_int;
    let ret: c_int;

    if getsockname(server_fd, &mut addr as *mut _ as *mut sockaddr, &mut addrlen) != 0 {
        log_err!("Failed to get server addr");
        return -1;
    }

    addr_in = &mut addr as *mut _ as *mut sockaddr_in;
    fd = socket((*addr_in).sin_family as c_int, SOCK_STREAM, 0);
    if fd < 0 {
        log_err!("Failed to create client socket");
        return -1;
    }

    if settimeo(fd, timeout_ms) != 0 {
        save_errno_close(fd);
        return -1;
    }

    ret = sendto(fd, data as *const c_void, data_len as size_t, MSG_FASTOPEN, &addr as *const _ as *const sockaddr, addrlen) as c_int;
    if ret != data_len as c_int {
        log_err!("sendto(data, %u) != %d\n", data_len, ret);
        save_errno_close(fd);
        return -1;
    }

    fd
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn client_socket(family: c_int, type_: c_int, mut opts: *const network_helper_opts) -> c_int {
    let fd: c_int;

    if opts.is_null() {
        opts = &default_opts;
    }

    fd = socket(family, type_, (*opts).proto);
    if fd < 0 {
        log_err!("Failed to create client socket");
        return -1;
    }

    if settimeo(fd, (*opts).timeout_ms) != 0 {
        save_errno_close(fd);
        return -1;
    }

    if let Some(cb) = (*opts).post_socket_cb {
        if cb(fd, (*opts).cb_opts) != 0 {
            save_errno_close(fd);
            return -1;
        }
    }

    fd
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn connect_to_addr(type_: c_int, addr: *const sockaddr_storage, addrlen: socklen_t, mut opts: *const network_helper_opts) -> c_int {
    let fd: c_int;

    if opts.is_null() {
        opts = &default_opts;
    }

    fd = client_socket((*addr).ss_family as c_int, type_, opts);
    if fd < 0 {
        log_err!("Failed to create client socket");
        return -1;
    }

    if connect(fd, addr as *const sockaddr, addrlen) != 0 {
        log_err!("Failed to connect to server");
        save_errno_close(fd);
        return -1;
    }

    fd
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn connect_to_addr_str(family: c_int, type_: c_int, addr_str: *const c_char, port: __u16, mut opts: *const network_helper_opts) -> c_int {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut addrlen: socklen_t = 0;

    if opts.is_null() {
        opts = &default_opts;
    }

    if make_sockaddr(family, addr_str, port, &mut addr, &mut addrlen) != 0 {
        return -1;
    }

    connect_to_addr(type_, &addr, addrlen, opts)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn connect_to_fd_opts(server_fd: c_int, mut opts: *const network_helper_opts) -> c_int {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut addrlen: socklen_t;
    let mut optlen: socklen_t;
    let mut type_: c_int = 0;

    if opts.is_null() {
        opts = &default_opts;
    }

    optlen = size_of::<c_int>() as socklen_t;
    if getsockopt(server_fd, SOL_SOCKET, SO_TYPE, &mut type_ as *mut _ as *mut c_void, &mut optlen) != 0 {
        log_err!("getsockopt(SOL_TYPE)");
        return -1;
    }

    addrlen = size_of::<sockaddr_storage>() as socklen_t;
    if getsockname(server_fd, &mut addr as *mut _ as *mut sockaddr, &mut addrlen) != 0 {
        log_err!("Failed to get server addr");
        return -1;
    }

    connect_to_addr(type_, &addr, addrlen, opts)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int {
    let mut opts = network_helper_opts {
        timeout_ms,
        proto: 0,
        backlog: 0,
        post_socket_cb: None,
        cb_opts: ptr::null_mut(),
    };
    let mut optlen: socklen_t;
    let mut protocol: c_int = 0;

    optlen = size_of::<c_int>() as socklen_t;
    if getsockopt(server_fd, SOL_SOCKET, SO_PROTOCOL, &mut protocol as *mut _ as *mut c_void, &mut optlen) != 0 {
        log_err!("getsockopt(SOL_PROTOCOL)");
        return -1;
    }
    opts.proto = protocol;

    connect_to_fd_opts(server_fd, &opts)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn connect_fd_to_fd(client_fd: c_int, server_fd: c_int, timeout_ms: c_int) -> c_int {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut len: socklen_t = size_of::<sockaddr_storage>() as socklen_t;

    if settimeo(client_fd, timeout_ms) != 0 {
        return -1;
    }

    if getsockname(server_fd, &mut addr as *mut _ as *mut sockaddr, &mut len) != 0 {
        log_err!("Failed to get server addr");
        return -1;
    }

    if connect(client_fd, &addr as *const _ as *const sockaddr, len) != 0 {
        log_err!("Failed to connect to server");
        return -1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_sockaddr(family: c_int, addr_str: *const c_char, port: __u16, addr: *mut sockaddr_storage, len: *mut socklen_t) -> c_int {
    if family == AF_INET {
        let sin = addr as *mut sockaddr_in;

        memset(addr as *mut c_void, 0, size_of::<sockaddr_in>());
        (*sin).sin_family = AF_INET as sa_family_t;
        (*sin).sin_port = htons(port);
        if !addr_str.is_null() && inet_pton(AF_INET, addr_str, &mut (*sin).sin_addr as *mut _ as *mut c_void) != 1 {
            log_err!("inet_pton(AF_INET, %s)", addr_str);
            return -1;
        }
        if !len.is_null() {
            *len = size_of::<sockaddr_in>() as socklen_t;
        }
        return 0;
    } else if family == AF_INET6 {
        let sin6 = addr as *mut sockaddr_in6;

        memset(addr as *mut c_void, 0, size_of::<sockaddr_in6>());
        (*sin6).sin6_family = AF_INET6 as sa_family_t;
        (*sin6).sin6_port = htons(port);
        if !addr_str.is_null() && inet_pton(AF_INET6, addr_str, &mut (*sin6).sin6_addr as *mut _ as *mut c_void) != 1 {
            log_err!("inet_pton(AF_INET6, %s)", addr_str);
            return -1;
        }
        if !len.is_null() {
            *len = size_of::<sockaddr_in6>() as socklen_t;
        }
        return 0;
    } else if family == AF_UNIX {
        /*
         * Note that we always use abstract unix sockets to avoid having
         * to clean up leftover files.
         */
        let sun = addr as *mut sockaddr_un;

        memset(addr as *mut c_void, 0, size_of::<sockaddr_un>());
        (*sun).sun_family = family as sa_family_t;
        (*sun).sun_path[0] = 0;
        strscpy((*sun).sun_path.as_mut_ptr().add(1), addr_str, size_of::<[c_char; 108]>() - 1);
        if !len.is_null() {
            *len = (offset_of!(sockaddr_un, sun_path) + 1 + strlen(addr_str)) as socklen_t;
        }
        return 0;
    }
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ping_command(family: c_int) -> *mut c_char {
    if family == AF_INET6 {
        /* On some systems 'ping' doesn't support IPv6, so use ping6 if it is present. */
        if system(c"which ping6 >/dev/null 2>&1".as_ptr()) == 0 {
            return c"ping6".as_ptr() as *mut c_char;
        } else {
            return c"ping -6".as_ptr() as *mut c_char;
        }
    }
    c"ping".as_ptr() as *mut c_char
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn append_tid(str_: *mut c_char, sz: size_t) -> c_int {
    let end: size_t;

    if str_.is_null() {
        return -1;
    }

    end = strlen(str_);
    if end + 8 > sz {
        return -1;
    }

    sprintf(str_.add(end), c"%07ld".as_ptr(), sys_gettid());
    *str_.add(end + 7) = 0;

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove_netns(name: *const c_char) -> c_int {
    let mut cmd: *mut c_char = ptr::null_mut();
    let mut r: c_int;

    r = asprintf(&mut cmd, c"ip netns del %s >/dev/null 2>&1".as_ptr(), name);
    if r < 0 {
        log_err!("Failed to malloc cmd");
        return -1;
    }

    r = system(cmd);
    free(cmd as *mut c_void);
    r
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_netns(name: *const c_char) -> c_int {
    let mut cmd: *mut c_char = ptr::null_mut();
    let mut r: c_int;

    r = asprintf(&mut cmd, c"ip netns add %s".as_ptr(), name);
    if r < 0 {
        log_err!("Failed to malloc cmd");
        return -1;
    }

    r = system(cmd);
    free(cmd as *mut c_void);

    if r != 0 {
        return r;
    }

    r = asprintf(&mut cmd, c"ip -n %s link set lo up".as_ptr(), name);
    if r < 0 {
        log_err!("Failed to malloc cmd for setting up lo");
        remove_netns(name);
        return -1;
    }

    r = system(cmd);
    free(cmd as *mut c_void);

    r
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn open_netns(name: *const c_char) -> *mut nstoken {
    let nsfd: c_int;
    let mut nspath = [0 as c_char; PATH_MAX];
    let err: c_int;
    let token: *mut nstoken;

    token = calloc(1, size_of::<nstoken>()) as *mut nstoken;
    if token.is_null() {
        log_err!("Failed to malloc token");
        return ptr::null_mut();
    }

    (*token).orig_netns_fd = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
    if (*token).orig_netns_fd == -1 {
        log_err!("Failed to open(/proc/self/ns/net)");
        free(token as *mut c_void);
        return ptr::null_mut();
    }

    snprintf(nspath.as_mut_ptr(), nspath.len(), c"%s/%s".as_ptr(), c"/var/run/netns".as_ptr(), name);
    nsfd = open(nspath.as_ptr(), O_RDONLY | O_CLOEXEC);
    if nsfd == -1 {
        log_err!("Failed to open(%s)", nspath.as_ptr());
        if (*token).orig_netns_fd != -1 {
            close((*token).orig_netns_fd);
        }
        free(token as *mut c_void);
        return ptr::null_mut();
    }

    err = setns(nsfd, CLONE_NEWNET);
    close(nsfd);
    if err != 0 {
        log_err!("Failed to setns(nsfd)");
        if (*token).orig_netns_fd != -1 {
            close((*token).orig_netns_fd);
        }
        free(token as *mut c_void);
        return ptr::null_mut();
    }

    token
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn close_netns(token: *mut nstoken) {
    if token.is_null() {
        return;
    }

    if setns((*token).orig_netns_fd, CLONE_NEWNET) != 0 {
        log_err!("Failed to setns(orig_netns_fd)");
    }
    close((*token).orig_netns_fd);
    free(token as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn open_tuntap(dev_name: *const c_char, need_mac: bool) -> c_int {
    let mut err: c_int = 0;
    let mut ifr: ifreq = core::mem::zeroed();
    let fd = open(c"/dev/net/tun".as_ptr(), O_RDWR);

    if !ASSERT_GE(fd, 0, c"open(/dev/net/tun)".as_ptr()) {
        return -1;
    }

    ifr.ifr_data = (IFF_NO_PI | if need_mac { IFF_TAP } else { IFF_TUN }) as isize as *mut c_char;
    strscpy(ifr.ifr_name.as_mut_ptr(), dev_name, IF_NAMESIZE);

    err = ioctl(fd, TUNSETIFF, &mut ifr);
    if !ASSERT_OK(err, c"ioctl(TUNSETIFF)".as_ptr()) {
        close(fd);
        return -1;
    }

    err = fcntl(fd, F_SETFL, O_NONBLOCK);
    if !ASSERT_OK(err, c"fcntl(O_NONBLOCK)".as_ptr()) {
        close(fd);
        return -1;
    }

    fd
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_socket_local_port(sock_fd: c_int) -> c_int {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut addrlen: socklen_t = size_of::<sockaddr_storage>() as socklen_t;
    let err: c_int;

    err = getsockname(sock_fd, &mut addr as *mut _ as *mut sockaddr, &mut addrlen);
    if err < 0 {
        return err;
    }

    if addr.ss_family as c_int == AF_INET {
        let sin = &mut addr as *mut _ as *mut sockaddr_in;
        return (*sin).sin_port as c_int;
    } else if addr.ss_family as c_int == AF_INET6 {
        let sin6 = &mut addr as *mut _ as *mut sockaddr_in6;
        return (*sin6).sin6_port as c_int;
    }

    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_hw_ring_size(ifname: *mut c_char, ring_param: *mut ethtool_ringparam) -> c_int {
    let mut ifr: ifreq = core::mem::zeroed();
    let sockfd: c_int;
    let err: c_int;

    sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if sockfd < 0 {
        return -errno;
    }

    memcpy(ifr.ifr_name.as_mut_ptr() as *mut c_void, ifname as *const c_void, IF_NAMESIZE);

    (*ring_param).cmd = ETHTOOL_GRINGPARAM;
    ifr.ifr_data = ring_param as *mut c_char;

    if ioctl(sockfd, SIOCETHTOOL, &mut ifr) < 0 {
        err = errno;
        close(sockfd);
        return -err;
    }

    close(sockfd);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_hw_ring_size(ifname: *mut c_char, ring_param: *mut ethtool_ringparam) -> c_int {
    let mut ifr: ifreq = core::mem::zeroed();
    let sockfd: c_int;
    let err: c_int;

    sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if sockfd < 0 {
        return -errno;
    }

    memcpy(ifr.ifr_name.as_mut_ptr() as *mut c_void, ifname as *const c_void, IF_NAMESIZE);

    (*ring_param).cmd = ETHTOOL_SRINGPARAM;
    ifr.ifr_data = ring_param as *mut c_char;

    if ioctl(sockfd, SIOCETHTOOL, &mut ifr) < 0 {
        err = errno;
        close(sockfd);
        return -err;
    }

    close(sockfd);
    0
}

unsafe extern "C" fn send_recv_server(arg: *mut c_void) -> *mut c_void {
    let a = arg as *mut send_recv_arg;
    let mut nr_sent: ssize_t = 0;
    let mut bytes: ssize_t = 0;
    let batch = [0 as c_char; 1500];
    let mut err: c_int = 0;
    let mut fd: c_int;

    fd = accept((*a).fd, ptr::null_mut(), ptr::null_mut());
    while fd == -1 {
        if errno == EINTR {
            continue;
        }
        err = -errno;
        break;
    }

    if err == 0 && settimeo(fd, 0) != 0 {
        err = -errno;
    }

    while err == 0 && bytes < (*a).bytes as ssize_t && READ_ONCE_i(&(*a).stop) == 0 {
        nr_sent = send(fd, batch.as_ptr() as *const c_void, min_usize(((*a).bytes as ssize_t - bytes) as usize, size_of_val(&batch)), 0);
        if nr_sent == -1 && errno == EINTR {
            continue;
        }
        if nr_sent == -1 {
            err = -errno;
            break;
        }
        bytes += nr_sent;
    }

    if bytes != (*a).bytes as ssize_t {
        log_err!("send %zd expected %u", bytes, (*a).bytes);
        if err == 0 {
            err = if bytes > (*a).bytes as ssize_t { -E2BIG } else { -EINTR };
        }
    }

    if fd >= 0 {
        close(fd);
    }
    if err != 0 {
        WRITE_ONCE_i(&mut (*a).stop, 1);
        return err_ptr(err);
    }
    ptr::null_mut()
}

fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn send_recv_data(lfd: c_int, fd: c_int, total_bytes: u32) -> c_int {
    let mut nr_recv: ssize_t = 0;
    let mut bytes: ssize_t = 0;
    let mut arg = send_recv_arg {
        fd: lfd,
        bytes: total_bytes,
        stop: 0,
    };
    let mut srv_thread: pthread_t = 0;
    let mut thread_ret: *mut c_void = ptr::null_mut();
    let mut batch = [0 as c_char; 1500];
    let mut err: c_int = 0;

    err = pthread_create(&mut srv_thread, ptr::null(), send_recv_server, &mut arg as *mut _ as *mut c_void);
    if err != 0 {
        log_err!("Failed to pthread_create");
        return err;
    }

    /* recv total_bytes */
    while bytes < total_bytes as ssize_t && READ_ONCE_i(&arg.stop) == 0 {
        nr_recv = recv(fd, batch.as_mut_ptr() as *mut c_void, min_usize((total_bytes as ssize_t - bytes) as usize, size_of_val(&batch)), 0);
        if nr_recv == -1 && errno == EINTR {
            continue;
        }
        if nr_recv == -1 {
            err = -errno;
            break;
        }
        bytes += nr_recv;
    }

    if bytes != total_bytes as ssize_t {
        log_err!("recv %zd expected %u", bytes, total_bytes);
        if err == 0 {
            err = if bytes > total_bytes as ssize_t { -E2BIG } else { -EINTR };
        }
    }

    WRITE_ONCE_i(&mut arg.stop, 1);
    pthread_join(srv_thread, &mut thread_ret);
    if is_err(thread_ret) {
        log_err!("Failed in thread_ret %ld", ptr_err(thread_ret));
        err = if err != 0 { err } else { ptr_err(thread_ret) as c_int };
    }

    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tc_prog_attach(dev: *const c_char, ingress_fd: c_int, egress_fd: c_int) -> c_int {
    let ifindex: c_int;
    let mut ret: c_int;

    if !ASSERT_TRUE(ingress_fd >= 0 || egress_fd >= 0, c"at least one program fd is valid".as_ptr()) {
        return -1;
    }

    ifindex = if_nametoindex(dev) as c_int;
    if !ASSERT_NEQ(ifindex as c_uint, 0, c"get ifindex".as_ptr()) {
        return -1;
    }

    let mut hook = bpf_tc_hook {
        sz: size_of::<bpf_tc_hook>(),
        ifindex,
        attach_point: BPF_TC_INGRESS | BPF_TC_EGRESS,
    };
    let mut opts1 = bpf_tc_opts {
        sz: size_of::<bpf_tc_opts>(),
        handle: 1,
        priority: 1,
        prog_fd: ingress_fd,
    };
    let mut opts2 = bpf_tc_opts {
        sz: size_of::<bpf_tc_opts>(),
        handle: 1,
        priority: 1,
        prog_fd: egress_fd,
    };

    ret = bpf_tc_hook_create(&mut hook);
    if !ASSERT_OK(ret, c"create tc hook".as_ptr()) {
        return ret;
    }

    if ingress_fd >= 0 {
        hook.attach_point = BPF_TC_INGRESS;
        ret = bpf_tc_attach(&mut hook, &mut opts1);
        if !ASSERT_OK(ret, c"bpf_tc_attach".as_ptr()) {
            bpf_tc_hook_destroy(&mut hook);
            return ret;
        }
    }

    if egress_fd >= 0 {
        hook.attach_point = BPF_TC_EGRESS;
        ret = bpf_tc_attach(&mut hook, &mut opts2);
        if !ASSERT_OK(ret, c"bpf_tc_attach".as_ptr()) {
            bpf_tc_hook_destroy(&mut hook);
            return ret;
        }
    }

    0
}

// The following block preserves code under the C build-time TRAFFIC_MONITOR
// condition. Enable it from the Rust build when translating that C define.

static mut __tm_pr: tm_print_fn_t = Some(__base_pr);

unsafe extern "C" fn __base_pr(format: *const c_char, args: va_list) -> c_int {
    vfprintf(stdout, format, args)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn traffic_monitor_set_print(fn_: tm_print_fn_t) -> tm_print_fn_t {
    let old_print_fn: tm_print_fn_t;

    old_print_fn = __tm_pr;
    __tm_pr = fn_;

    old_print_fn
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tm_print(format: *const c_char, mut _args: ...) {
    let print_fn: tm_print_fn_t;
    let args: va_list = ptr::null_mut();

    print_fn = __tm_pr;
    if print_fn.is_none() {
        return;
    }

    print_fn.unwrap()(format, args);
}

/* Is this packet captured with a Ethernet protocol type? */
unsafe fn is_ethernet(packet: *const u_char) -> bool {
    let mut arphdr_type: u16 = 0;

    memcpy(&mut arphdr_type as *mut _ as *mut c_void, packet.add(8) as *const c_void, 2);
    arphdr_type = ntohs(arphdr_type);

    /*
     * Except the following cases, the protocol type contains the
     * Ethernet protocol type for the packet.
     *
     * https://www.tcpdump.org/linktypes/LINKTYPE_LINUX_SLL2.html
     */
    match arphdr_type {
        770 | 778 | 803 => {
            tm_print(c"Packet captured: arphdr_type=%d\n".as_ptr(), arphdr_type as c_int);
            false
        }
        _ => true,
    }
}

static pkt_types: [*const c_char; 5] = [
    c"In".as_ptr(),
    c"B".as_ptr(), /* Broadcast */
    c"M".as_ptr(), /* Multicast */
    c"C".as_ptr(), /* Captured with the promiscuous mode */
    c"Out".as_ptr(),
];

unsafe fn pkt_type_str(pkt_type: u16) -> *const c_char {
    if (pkt_type as usize) < pkt_types.len() {
        return pkt_types[pkt_type as usize];
    }
    c"Unknown".as_ptr()
}

/* Show the information of the transport layer in the packet */
unsafe fn show_transport(packet: *const u_char, len: u16, ifindex: u32, src_addr: *const c_char, dst_addr: *const c_char, proto: u16, ipv6: bool, pkt_type: u8) {
    let mut _ifname = [0 as c_char; IF_NAMESIZE];
    let mut flags = [0 as c_char; MAX_FLAGS_STRLEN];
    let transport_str: *const c_char;
    let src_port: u16;
    let dst_port: u16;
    let udp: *mut udphdr;
    let tcp: *mut tcphdr;
    let mut ifname = if_indextoname(ifindex, _ifname.as_mut_ptr());
    if ifname.is_null() {
        snprintf(_ifname.as_mut_ptr(), _ifname.len(), c"unknown(%d)".as_ptr(), ifindex);
        ifname = _ifname.as_mut_ptr();
    }

    if proto as c_int == IPPROTO_UDP {
        udp = packet as *mut udphdr;
        src_port = ntohs((*udp).source);
        dst_port = ntohs((*udp).dest);
        transport_str = c"UDP".as_ptr();
        tcp = ptr::null_mut();
    } else if proto as c_int == IPPROTO_TCP {
        tcp = packet as *mut tcphdr;
        src_port = ntohs((*tcp).source);
        dst_port = ntohs((*tcp).dest);
        transport_str = c"TCP".as_ptr();
    } else if proto as c_int == IPPROTO_ICMP {
        tm_print(c"%-7s %-3s IPv4 %s > %s: ICMP, length %d, type %d, code %d\n".as_ptr(), ifname, pkt_type_str(pkt_type as u16), src_addr, dst_addr, len as c_int, *packet.add(0) as c_int, *packet.add(1) as c_int);
        return;
    } else if proto as c_int == IPPROTO_ICMPV6 {
        tm_print(c"%-7s %-3s IPv6 %s > %s: ICMPv6, length %d, type %d, code %d\n".as_ptr(), ifname, pkt_type_str(pkt_type as u16), src_addr, dst_addr, len as c_int, *packet.add(0) as c_int, *packet.add(1) as c_int);
        return;
    } else {
        tm_print(c"%-7s %-3s %s %s > %s: protocol %d\n".as_ptr(), ifname, pkt_type_str(pkt_type as u16), if ipv6 { c"IPv6".as_ptr() } else { c"IPv4".as_ptr() }, src_addr, dst_addr, proto as c_int);
        return;
    }

    /* TCP or UDP*/
    if proto as c_int == IPPROTO_TCP {
        snprintf(
            flags.as_mut_ptr(),
            MAX_FLAGS_STRLEN,
            c"%s%s%s%s".as_ptr(),
            if (*tcp).fin() { c", FIN".as_ptr() } else { c"".as_ptr() },
            if (*tcp).syn() { c", SYN".as_ptr() } else { c"".as_ptr() },
            if (*tcp).rst() { c", RST".as_ptr() } else { c"".as_ptr() },
            if (*tcp).ack() { c", ACK".as_ptr() } else { c"".as_ptr() },
        );
    }

    if ipv6 {
        tm_print(c"%-7s %-3s IPv6 %s.%d > %s.%d: %s, length %d%s\n".as_ptr(), ifname, pkt_type_str(pkt_type as u16), src_addr, src_port as c_int, dst_addr, dst_port as c_int, transport_str, len as c_int, flags.as_ptr());
    } else {
        tm_print(c"%-7s %-3s IPv4 %s:%d > %s:%d: %s, length %d%s\n".as_ptr(), ifname, pkt_type_str(pkt_type as u16), src_addr, src_port as c_int, dst_addr, dst_port as c_int, transport_str, len as c_int, flags.as_ptr());
    }
}

unsafe fn show_ipv6_packet(packet: *const u_char, ifindex: u32, pkt_type: u8) {
    let mut src_buf = [0 as c_char; 46];
    let mut dst_buf = [0 as c_char; 46];
    let pkt = packet as *mut ipv6hdr;
    let mut src: *const c_char;
    let mut dst: *const c_char;
    let proto: u_char;

    src = inet_ntop(AF_INET6, &(*pkt).saddr as *const _ as *const c_void, src_buf.as_mut_ptr(), src_buf.len() as socklen_t);
    if src.is_null() {
        src = c"<invalid>".as_ptr();
    }
    dst = inet_ntop(AF_INET6, &(*pkt).daddr as *const _ as *const c_void, dst_buf.as_mut_ptr(), dst_buf.len() as socklen_t);
    if dst.is_null() {
        dst = c"<invalid>".as_ptr();
    }
    proto = (*pkt).nexthdr;
    show_transport(packet.add(size_of::<ipv6hdr>()), ntohs((*pkt).payload_len), ifindex, src, dst, proto as u16, true, pkt_type);
}

unsafe fn show_ipv4_packet(packet: *const u_char, ifindex: u32, pkt_type: u8) {
    let mut src_buf = [0 as c_char; 16];
    let mut dst_buf = [0 as c_char; 16];
    let pkt = packet as *mut iphdr;
    let mut src: *const c_char;
    let mut dst: *const c_char;
    let proto: u_char;

    src = inet_ntop(AF_INET, &(*pkt).saddr as *const _ as *const c_void, src_buf.as_mut_ptr(), src_buf.len() as socklen_t);
    if src.is_null() {
        src = c"<invalid>".as_ptr();
    }
    dst = inet_ntop(AF_INET, &(*pkt).daddr as *const _ as *const c_void, dst_buf.as_mut_ptr(), dst_buf.len() as socklen_t);
    if dst.is_null() {
        dst = c"<invalid>".as_ptr();
    }
    proto = (*pkt).protocol;
    show_transport(packet.add(size_of::<iphdr>()), ntohs((*pkt).tot_len), ifindex, src, dst, proto as u16, false, pkt_type);
}

unsafe fn FD_ZERO(set: *mut fd_set) {
    memset(set as *mut c_void, 0, size_of::<fd_set>());
}

unsafe fn FD_SET(fd: c_int, set: *mut fd_set) {
    let idx = (fd / (8 * size_of::<c_long>() as c_int)) as usize;
    let bit = fd % (8 * size_of::<c_long>() as c_int);
    (*set).fds_bits[idx] |= 1 << bit;
}

unsafe extern "C" fn traffic_monitor_thread(arg: *mut c_void) -> *mut c_void {
    let mut _ifname = [0 as c_char; IF_NAMESIZE];
    let mut ifname: *mut c_char;
    let mut packet: *const u_char;
    let payload: *const u_char;
    let ctx = arg as *mut tmonitor_ctx;
    let dumper = (*ctx).dumper;
    let fd = (*ctx).pcap_fd;
    let nfds: c_int;
    let mut r: c_int;
    let wake_fd = (*ctx).wake_fd;
    let mut header: pcap_pkthdr = core::mem::zeroed();
    let pcap = (*ctx).pcap;
    let mut ifindex: u32 = 0;
    let mut fds: fd_set = core::mem::zeroed();
    let mut proto: u16 = 0;
    let ptype: u8;

    nfds = (if fd > wake_fd { fd } else { wake_fd }) + 1;
    FD_ZERO(&mut fds);

    while !ptr::read_volatile(&(*ctx).done) {
        FD_SET(fd, &mut fds);
        FD_SET(wake_fd, &mut fds);
        r = select(nfds, &mut fds, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
        if r == 0 {
            continue;
        }
        if r < 0 {
            if errno == EINTR {
                continue;
            }
            log_err!("Fail to select on pcap fd and wake fd");
            break;
        }

        /* This instance of pcap is non-blocking */
        packet = pcap_next(pcap, &mut header);
        if packet.is_null() {
            continue;
        }

        /*
         * According to the man page of pcap_dump(), first argument
         * is the pcap_dumper_t pointer even it's argument type is
         * u_char *.
         */
        pcap_dump(dumper as *mut u_char, &header, packet);

        /*
         * Not sure what other types of packets look like. Here, we
         * parse only Ethernet and compatible packets.
         */
        if !is_ethernet(packet) {
            continue;
        }

        /*
         * Skip SLL2 header
         * https://www.tcpdump.org/linktypes/LINKTYPE_LINUX_SLL2.html
         *
         * Although the document doesn't mention that, the payload
         * doesn't include the Ethernet header. The payload starts
         * from the first byte of the network layer header.
         */
        payload = packet.add(20);

        memcpy(&mut proto as *mut _ as *mut c_void, packet as *const c_void, 2);
        proto = ntohs(proto);
        memcpy(&mut ifindex as *mut _ as *mut c_void, packet.add(4) as *const c_void, 4);
        ifindex = ntohl(ifindex);
        ptype = *packet.add(10);

        if proto == ETH_P_IPV6 {
            show_ipv6_packet(payload, ifindex, ptype);
        } else if proto == ETH_P_IP {
            show_ipv4_packet(payload, ifindex, ptype);
        } else {
            ifname = if_indextoname(ifindex, _ifname.as_mut_ptr());
            if ifname.is_null() {
                snprintf(_ifname.as_mut_ptr(), _ifname.len(), c"unknown(%d)".as_ptr(), ifindex);
                ifname = _ifname.as_mut_ptr();
            }

            tm_print(c"%-7s %-3s Unknown network protocol type 0x%x\n".as_ptr(), ifname, pkt_type_str(ptype as u16), proto as c_int);
        }
    }

    ptr::null_mut()
}

/*
 * Prepare the pcap handle to capture packets.
 *
 * This pcap is non-blocking and immediate mode is enabled to receive
 * captured packets as soon as possible.  The snaplen is set to 1024 bytes
 * to limit the size of captured content. The format of the link-layer
 * header is set to DLT_LINUX_SLL2 to enable handling various link-layer
 * technologies.
 */
unsafe fn traffic_monitor_prepare_pcap() -> *mut pcap_t {
    let mut errbuf = [0 as c_char; PCAP_ERRBUF_SIZE];
    let pcap: *mut pcap_t;
    let mut r: c_int;

    /* Listen on all NICs in the namespace */
    pcap = pcap_create(c"any".as_ptr(), errbuf.as_mut_ptr());
    if pcap.is_null() {
        log_err!("Failed to open pcap: %s", errbuf.as_ptr());
        return ptr::null_mut();
    }
    /* Limit the size of the packet (first N bytes) */
    r = pcap_set_snaplen(pcap, 1024);
    if r != 0 {
        log_err!("Failed to set snaplen: %s", pcap_geterr(pcap));
        pcap_close(pcap);
        return ptr::null_mut();
    }
    /* To receive packets as fast as possible */
    r = pcap_set_immediate_mode(pcap, 1);
    if r != 0 {
        log_err!("Failed to set immediate mode: %s", pcap_geterr(pcap));
        pcap_close(pcap);
        return ptr::null_mut();
    }
    r = pcap_setnonblock(pcap, 1, errbuf.as_mut_ptr());
    if r != 0 {
        log_err!("Failed to set nonblock: %s", errbuf.as_ptr());
        pcap_close(pcap);
        return ptr::null_mut();
    }
    r = pcap_activate(pcap);
    if r != 0 {
        log_err!("Failed to activate pcap: %s", pcap_geterr(pcap));
        pcap_close(pcap);
        return ptr::null_mut();
    }
    /* Determine the format of the link-layer header */
    r = pcap_set_datalink(pcap, DLT_LINUX_SLL2);
    if r != 0 {
        log_err!("Failed to set datalink: %s", pcap_geterr(pcap));
        pcap_close(pcap);
        return ptr::null_mut();
    }

    pcap
}

unsafe fn encode_test_name(buf: *mut c_char, len: size_t, test_name: *const c_char, subtest_name: *const c_char) {
    let mut p: *mut c_char;

    if !subtest_name.is_null() {
        snprintf(buf, len, c"%s__%s".as_ptr(), test_name, subtest_name);
    } else {
        snprintf(buf, len, c"%s".as_ptr(), test_name);
    }
    loop {
        p = strchr(buf, '/' as c_int);
        if p.is_null() {
            break;
        }
        *p = '_' as c_char;
    }
    loop {
        p = strchr(buf, ' ' as c_int);
        if p.is_null() {
            break;
        }
        *p = '_' as c_char;
    }
}

/*
 * Start to monitor the network traffic in the given network namespace.
 *
 * netns: the name of the network namespace to monitor. If NULL, the
 *        current network namespace is monitored.
 * test_name: the name of the running test.
 * subtest_name: the name of the running subtest if there is. It should be
 *               NULL if it is not a subtest.
 *
 * This function will start a thread to capture packets going through NICs
 * in the give network namespace.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn traffic_monitor_start(netns: *const c_char, test_name: *const c_char, subtest_name: *const c_char) -> *mut tmonitor_ctx {
    static mut tmon_seq: c_int = 0;
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let ctx: *mut tmonitor_ctx;
    let mut test_name_buf = [0 as c_char; 64];
    let mut r: c_int;

    if !netns.is_null() {
        nstoken = open_netns(netns);
        if nstoken.is_null() {
            return ptr::null_mut();
        }
    }
    ctx = malloc(size_of::<tmonitor_ctx>()) as *mut tmonitor_ctx;
    if ctx.is_null() {
        log_err!("Failed to malloc ctx");
        close_netns(nstoken);
        return ptr::null_mut();
    }
    memset(ctx as *mut c_void, 0, size_of::<tmonitor_ctx>());

    encode_test_name(test_name_buf.as_mut_ptr(), test_name_buf.len(), test_name, subtest_name);
    snprintf(
        (*ctx).pkt_fname.as_mut_ptr(),
        (*ctx).pkt_fname.len(),
        c"/tmp/tmon_pcap/packets-%d-%d-%s-%s.log".as_ptr(),
        getpid(),
        tmon_seq,
        test_name_buf.as_ptr(),
        if !netns.is_null() { netns } else { c"unknown".as_ptr() },
    );
    tmon_seq += 1;

    r = mkdir(PCAP_DIR.as_ptr() as *const c_char, 0o755);
    if r != 0 && errno != EEXIST {
        log_err!("Failed to create /tmp/tmon_pcap");
        free(ctx as *mut c_void);
        close_netns(nstoken);
        return ptr::null_mut();
    }

    (*ctx).pcap = traffic_monitor_prepare_pcap();
    if (*ctx).pcap.is_null() {
        free(ctx as *mut c_void);
        close_netns(nstoken);
        return ptr::null_mut();
    }
    (*ctx).pcap_fd = pcap_get_selectable_fd((*ctx).pcap);
    if (*ctx).pcap_fd < 0 {
        log_err!("Failed to get pcap fd");
        pcap_close((*ctx).pcap);
        free(ctx as *mut c_void);
        close_netns(nstoken);
        return ptr::null_mut();
    }

    /* Create a packet file */
    (*ctx).dumper = pcap_dump_open((*ctx).pcap, (*ctx).pkt_fname.as_ptr());
    if (*ctx).dumper.is_null() {
        log_err!("Failed to open pcap dump: %s", (*ctx).pkt_fname.as_ptr());
        pcap_close((*ctx).pcap);
        free(ctx as *mut c_void);
        close_netns(nstoken);
        return ptr::null_mut();
    }

    /* Create an eventfd to wake up the monitor thread */
    (*ctx).wake_fd = eventfd(0, 0);
    if (*ctx).wake_fd < 0 {
        log_err!("Failed to create eventfd");
        pcap_dump_close((*ctx).dumper);
        unlink((*ctx).pkt_fname.as_ptr());
        pcap_close((*ctx).pcap);
        free(ctx as *mut c_void);
        close_netns(nstoken);
        return ptr::null_mut();
    }

    r = pthread_create(&mut (*ctx).thread, ptr::null(), traffic_monitor_thread, ctx as *mut c_void);
    if r != 0 {
        log_err!("Failed to create thread");
        close((*ctx).wake_fd);
        pcap_dump_close((*ctx).dumper);
        unlink((*ctx).pkt_fname.as_ptr());
        pcap_close((*ctx).pcap);
        free(ctx as *mut c_void);
        close_netns(nstoken);
        return ptr::null_mut();
    }

    close_netns(nstoken);

    ctx
}

unsafe fn traffic_monitor_release(ctx: *mut tmonitor_ctx) {
    pcap_close((*ctx).pcap);
    pcap_dump_close((*ctx).dumper);

    close((*ctx).wake_fd);

    free(ctx as *mut c_void);
}

/*
 * Stop the network traffic monitor.
 *
 * ctx: the context returned by traffic_monitor_start()
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn traffic_monitor_stop(ctx: *mut tmonitor_ctx) {
    let w: __u64 = 1;

    if ctx.is_null() {
        return;
    }

    /* Stop the monitor thread */
    ptr::write_volatile(&mut (*ctx).done, true);
    /* Wake up the background thread. */
    write((*ctx).wake_fd, &w as *const _ as *const c_void, size_of::<__u64>());
    pthread_join((*ctx).thread, ptr::null_mut());

    tm_print(c"Packet file: %s\n".as_ptr(), strchr((*ctx).pkt_fname.as_ptr(), '/' as c_int).add(1));

    traffic_monitor_release(ctx);
}
