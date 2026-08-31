// SPDX-License-Identifier: GPL-2.0

/* Open a tun device.
 *
 * [modifications: use IFF_NAPI_FRAGS, add sk filter]
 *
 * Expects the device to have been configured previously, e.g.:
 *   sudo ip tuntap add name tap1 mode tap
 *   sudo ip link set tap1 up
 *   sudo ip link set dev tap1 addr 02:00:00:00:00:01
 *   sudo ip -6 addr add fdab::1 peer fdab::2 dev tap1 nodad
 *
 * And to avoid premature pskb_may_pull:
 *
 *   sudo ethtool -K tap1 gro off
 *   sudo bash -c 'echo 0 > /proc/sys/net/ipv4/ip_early_demux'
 */

use libc::{
    c_char, c_int, c_short, c_uint, c_ulong, c_void, iovec, msghdr, timeval, uint16_t, uint32_t,
};
use std::ffi::CString;
use std::mem::{size_of, size_of_val, zeroed};
use std::ptr;

const O_RDWR: c_int = libc::O_RDWR;
const AF_INET6: c_int = libc::AF_INET6;
const PF_INET6: c_int = libc::PF_INET6;
const SOCK_RAW: c_int = libc::SOCK_RAW;
const SOL_SOCKET: c_int = libc::SOL_SOCKET;
const SO_ATTACH_FILTER: c_int = 26;
const SO_RCVTIMEO: c_int = libc::SO_RCVTIMEO;
const IPPROTO_UDP: c_int = libc::IPPROTO_UDP;
const IFNAMSIZ: usize = 16;
const IFF_TAP: c_short = 0x0002;
const IFF_NAPI: c_short = 0x0010;
const IFF_NAPI_FRAGS: c_short = 0x0020;
const TUNSETIFF: c_ulong = 0x400454ca;
const ETH_P_IPV6: u16 = 0x86dd;
const PACKET_HOST: c_uint = 0;
const SKF_AD_OFF: c_uint = -0x1000i32 as c_uint;
const SKF_AD_PKTTYPE: c_uint = 4;
const SKF_NET_OFF: c_uint = -0x100000i32 as c_uint;

const BPF_LD: u16 = 0x00;
const BPF_B: u16 = 0x10;
const BPF_H: u16 = 0x08;
const BPF_ABS: u16 = 0x20;
const BPF_JMP: u16 = 0x05;
const BPF_JEQ: u16 = 0x10;
const BPF_K: u16 = 0x00;
const BPF_RET: u16 = 0x06;

static mut cfg_do_filter: bool = false;
static mut cfg_do_frags: bool = false;
static mut cfg_dst_port: c_int = 8000;
static mut cfg_ifname: *mut c_char = ptr::null_mut();

#[repr(C)]
struct ifreq {
    ifr_name: [c_char; IFNAMSIZ],
    ifr_ifru: ifreq_ifru,
}

#[repr(C)]
union ifreq_ifru {
    ifru_flags: c_short,
    pad: [u8; 24],
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
struct tun_pi {
    flags: u16,
    proto: u16,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct ipv6hdr {
    priority_version: u8,
    flow_lbl: [u8; 3],
    payload_len: uint16_t,
    nexthdr: u8,
    hop_limit: u8,
    saddr: in6_addr,
    daddr: in6_addr,
}

impl ipv6hdr {
    fn set_version(&mut self, version: u8) {
        self.priority_version = (self.priority_version & 0x0f) | (version << 4);
    }
}

#[repr(C)]
struct udphdr {
    source: uint16_t,
    dest: uint16_t,
    len: uint16_t,
    check: uint16_t,
}

#[repr(C)]
struct ethhdr {
    h_dest: [u8; 6],
    h_source: [u8; 6],
    h_proto: uint16_t,
}

extern "C" {
    static mut optarg: *mut c_char;
    static mut optopt: c_int;

    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: libc::socklen_t,
    ) -> c_int;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> isize;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> isize;
    fn close(fd: c_int) -> c_int;
}

fn htons(hostshort: u16) -> u16 {
    hostshort.to_be()
}

fn htonl(hostlong: u32) -> u32 {
    hostlong.to_be()
}

fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn error(status: c_int, errnum: c_int, message: &str) -> ! {
    if errnum != 0 {
        eprintln!("{}: {}", message, std::io::Error::from_raw_os_error(errnum));
    } else {
        eprintln!("{}", message);
    }
    std::process::exit(status);
}

fn error_opt(status: c_int, errnum: c_int, fmt: &str, value: c_int) -> ! {
    let message = fmt.replace("%c", &(value as u8 as char).to_string());
    error(status, errnum, &message)
}

fn error_len(status: c_int, errnum: c_int, fmt: &str, value: isize) -> ! {
    let message = fmt.replace("%d", &value.to_string());
    error(status, errnum, message.trim_end())
}

fn bpf_stmt(code: u16, k: c_uint) -> sock_filter {
    sock_filter {
        code,
        jt: 0,
        jf: 0,
        k,
    }
}

fn bpf_jump(code: u16, k: c_uint, jt: u8, jf: u8) -> sock_filter {
    sock_filter { code, jt, jf, k }
}

unsafe fn tun_open(tun_name: *const c_char) -> c_int {
    let mut ifr: ifreq = zeroed();
    let fd: c_int;
    let ret: c_int;

    fd = open(cstr("/dev/net/tun").as_ptr(), O_RDWR);
    if fd == -1 {
        error(1, *libc::__errno_location(), "open /dev/net/tun");
    }

    ifr.ifr_ifru.ifru_flags = IFF_TAP;
    if cfg_do_frags {
        ifr.ifr_ifru.ifru_flags |= IFF_NAPI | IFF_NAPI_FRAGS;
    }

    libc::strncpy(
        ifr.ifr_name.as_mut_ptr(),
        tun_name,
        (IFNAMSIZ - 1) as libc::size_t,
    );

    ret = ioctl(fd, TUNSETIFF, &mut ifr as *mut ifreq);
    if ret != 0 {
        error(1, ret, "ioctl TUNSETIFF");
    }

    fd
}

unsafe fn sk_set_filter(fd: c_int) {
    let offset_proto = 6usize;
    let offset_dport = size_of::<ipv6hdr>() + 2usize;

    /* Filter UDP packets with destination port cfg_dst_port */
    let mut filter_code = [
        bpf_stmt(BPF_LD + BPF_B + BPF_ABS, SKF_AD_OFF.wrapping_add(SKF_AD_PKTTYPE)),
        bpf_jump(BPF_JMP + BPF_JEQ + BPF_K, PACKET_HOST, 0, 4),
        bpf_stmt(BPF_LD + BPF_B + BPF_ABS, SKF_NET_OFF.wrapping_add(offset_proto as c_uint)),
        bpf_jump(BPF_JMP + BPF_JEQ + BPF_K, IPPROTO_UDP as c_uint, 0, 2),
        bpf_stmt(BPF_LD + BPF_H + BPF_ABS, SKF_NET_OFF.wrapping_add(offset_dport as c_uint)),
        bpf_jump(BPF_JMP + BPF_JEQ + BPF_K, cfg_dst_port as c_uint, 1, 0),
        bpf_stmt(BPF_RET + BPF_K, 0),
        bpf_stmt(BPF_RET + BPF_K, 0xffff),
    ];

    let filter = sock_fprog {
        len: (size_of_val(&filter_code) / size_of::<sock_filter>()) as u16,
        filter: filter_code.as_mut_ptr(),
    };

    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_ATTACH_FILTER,
        &filter as *const sock_fprog as *const c_void,
        size_of::<sock_fprog>() as libc::socklen_t,
    ) != 0
    {
        error(1, *libc::__errno_location(), "setsockopt attach filter");
    }
}

unsafe fn raw_open() -> c_int {
    let fd: c_int;

    fd = socket(PF_INET6, SOCK_RAW, IPPROTO_UDP);
    if fd == -1 {
        error(1, *libc::__errno_location(), "socket raw (udp)");
    }

    if cfg_do_filter {
        sk_set_filter(fd);
    }

    fd
}

unsafe fn tun_write(fd: c_int) {
    let eth_src: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
    let eth_dst: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
    let mut pi: tun_pi = zeroed();
    let mut ip6h: ipv6hdr = zeroed();
    let mut uh: udphdr = zeroed();
    let mut eth: ethhdr = zeroed();
    let mut payload: uint32_t;
    let mut iov: [iovec; 5] = zeroed();
    let ret: isize;

    pi.proto = htons(ETH_P_IPV6);

    ptr::copy_nonoverlapping(eth_src.as_ptr(), eth.h_source.as_mut_ptr(), eth_src.len());
    ptr::copy_nonoverlapping(eth_dst.as_ptr(), eth.h_dest.as_mut_ptr(), eth_dst.len());
    eth.h_proto = htons(ETH_P_IPV6);

    ip6h.set_version(6);
    ip6h.payload_len = htons((size_of::<udphdr>() + size_of::<uint32_t>()) as u16);
    ip6h.nexthdr = IPPROTO_UDP as u8;
    ip6h.hop_limit = 8;
    if inet_pton(
        AF_INET6,
        cstr("fdab::2").as_ptr(),
        &mut ip6h.saddr as *mut in6_addr as *mut c_void,
    ) != 1
    {
        error(1, *libc::__errno_location(), "inet_pton src");
    }
    if inet_pton(
        AF_INET6,
        cstr("fdab::1").as_ptr(),
        &mut ip6h.daddr as *mut in6_addr as *mut c_void,
    ) != 1
    {
        error(1, *libc::__errno_location(), "inet_pton src");
    }

    uh.source = htons(8000);
    uh.dest = htons(cfg_dst_port as u16);
    uh.len = ip6h.payload_len;
    uh.check = 0;

    payload = htonl(0xabababab); /* Covered in IPv6 length */

    iov[0].iov_base = &mut pi as *mut tun_pi as *mut c_void;
    iov[0].iov_len = size_of::<tun_pi>();
    iov[1].iov_base = &mut eth as *mut ethhdr as *mut c_void;
    iov[1].iov_len = size_of::<ethhdr>();
    iov[2].iov_base = &mut ip6h as *mut ipv6hdr as *mut c_void;
    iov[2].iov_len = size_of::<ipv6hdr>();
    iov[3].iov_base = &mut uh as *mut udphdr as *mut c_void;
    iov[3].iov_len = size_of::<udphdr>();
    iov[4].iov_base = &mut payload as *mut uint32_t as *mut c_void;
    iov[4].iov_len = size_of::<uint32_t>();

    ret = writev(fd, iov.as_ptr(), (size_of_val(&iov) / size_of::<iovec>()) as c_int);
    if ret <= 0 {
        error(1, *libc::__errno_location(), "writev");
    }
}

unsafe fn raw_read(fd: c_int) {
    let tv = timeval {
        tv_sec: 0,
        tv_usec: 100 * 1000,
    };
    let mut msg: msghdr = zeroed();
    let mut iov: [iovec; 2] = zeroed();
    let mut uh: udphdr = zeroed();
    let mut payload: [uint32_t; 2] = [0; 2];
    let ret: isize;

    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_RCVTIMEO,
        &tv as *const timeval as *const c_void,
        size_of::<timeval>() as libc::socklen_t,
    ) != 0
    {
        error(1, *libc::__errno_location(), "setsockopt rcvtimeo udp");
    }

    iov[0].iov_base = &mut uh as *mut udphdr as *mut c_void;
    iov[0].iov_len = size_of::<udphdr>();

    iov[1].iov_base = payload.as_mut_ptr() as *mut c_void;
    iov[1].iov_len = size_of_val(&payload);

    msg.msg_iov = iov.as_mut_ptr();
    msg.msg_iovlen = (size_of_val(&iov) / size_of::<iovec>()) as _;

    ret = recvmsg(fd, &mut msg as *mut msghdr, 0);
    if ret <= 0 {
        error(1, *libc::__errno_location(), "read raw");
    }
    if ret != (size_of::<udphdr>() + size_of::<uint32_t>()) as isize {
        error_len(1, *libc::__errno_location(), "read raw: len=%d\n", ret);
    }

    eprintln!("raw recv: 0x{:x}", payload[0]);
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut c: c_int;

    loop {
        c = getopt(argc, argv as *const *mut c_char, cstr("fFi:").as_ptr());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            'f' => {
                cfg_do_filter = true;
                println!("bpf filter enabled");
            }
            'F' => {
                cfg_do_frags = true;
                println!("napi frags mode enabled");
            }
            'i' => {
                cfg_ifname = optarg;
            }
            _ => {
                error_opt(1, 0, "unknown option %c", optopt);
            }
        }
    }

    if cfg_ifname.is_null() {
        error(1, 0, "must specify tap interface name (-i)");
    }
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let fdt: c_int;
    let fdr: c_int;

    parse_opts(argc, argv);

    fdr = raw_open();
    fdt = tun_open(cfg_ifname);

    tun_write(fdt);
    raw_read(fdr);

    if close(fdt) != 0 {
        error(1, *libc::__errno_location(), "close tun");
    }
    if close(fdr) != 0 {
        error(1, *libc::__errno_location(), "close udp");
    }

    eprintln!("OK");
    0
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| CString::new(arg).unwrap().into_raw())
        .collect();
    args.push(ptr::null_mut());

    let status = unsafe { c_main((args.len() - 1) as c_int, args.as_mut_ptr()) };

    for arg in args.into_iter().take_while(|arg| !arg.is_null()) {
        unsafe {
            let _ = CString::from_raw(arg);
        }
    }

    std::process::exit(status);
}
