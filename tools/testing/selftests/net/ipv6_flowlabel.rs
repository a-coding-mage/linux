// SPDX-License-Identifier: GPL-2.0
/* Test IPV6_FLOWINFO cmsg on send and recv */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(static_mut_refs)]

use libc::*;
use std::ffi::CString;
use std::mem;
use std::ptr;

/* uapi/glibc weirdness may leave this undefined */
const IPV6_FLOWINFO: c_int = 11;

const IPV6_FLOWLABEL_MGR: c_int = 32;
const IPV6_FLOWINFO_SEND: c_int = 33;

const FLOWLABEL_WILDCARD: u32 = -1i32 as u32;

const IPV6_FL_A_GET: u8 = 0;
const IPV6_FL_F_CREATE: u16 = 1;
const IPV6_FL_S_EXCL: u8 = 1;
const ICMPV6_ECHO_REQUEST: u8 = 128;

#[repr(C)]
struct icmp6hdr {
    icmp6_type: u8,
    icmp6_code: u8,
    icmp6_cksum: u16,
    icmp6_dataun: [u8; 4],
}

#[repr(C)]
struct in6_flowlabel_req {
    flr_dst: in6_addr,
    flr_label: u32,
    flr_action: u8,
    flr_share: u8,
    flr_flags: u16,
    flr_expires: u16,
    flr_linger: u16,
    __flr_pad: u32,
}

unsafe extern "C" {
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
}

static cfg_data: &[u8; 2] = b"a\0";
static mut cfg_label: u32 = 1;
static mut use_ping: bool = false;
static mut use_flowinfo_send: bool = false;

static mut icmp6: icmp6hdr = icmp6hdr {
    icmp6_type: ICMPV6_ECHO_REQUEST,
    icmp6_code: 0,
    icmp6_cksum: 0,
    icmp6_dataun: [0; 4],
};

static mut addr: sockaddr_in6 = sockaddr_in6 {
    sin6_family: AF_INET6 as sa_family_t,
    sin6_port: 0,
    sin6_flowinfo: 0,
    sin6_addr: in6_addr {
        s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    },
    sin6_scope_id: 0,
};

unsafe fn cmsg_align(len: usize) -> usize {
    (len + mem::size_of::<usize>() - 1) & !(mem::size_of::<usize>() - 1)
}

unsafe fn cmsg_space(len: usize) -> usize {
    cmsg_align(mem::size_of::<cmsghdr>()) + cmsg_align(len)
}

unsafe fn cmsg_len(len: usize) -> usize {
    cmsg_align(mem::size_of::<cmsghdr>()) + len
}

unsafe fn cmsg_data(cm: *mut cmsghdr) -> *mut u8 {
    (cm as *mut u8).add(cmsg_align(mem::size_of::<cmsghdr>()))
}

unsafe fn do_send(fd: c_int, with_flowlabel: bool, flowlabel: u32) {
    let mut control = [0u8; cmsg_space(mem::size_of_val(&flowlabel))];
    let mut msg: msghdr = mem::zeroed();
    let mut iov = iovec {
        iov_base: cfg_data.as_ptr() as *mut c_void,
        iov_len: mem::size_of_val(cfg_data),
    };
    let ret: c_int;

    if use_ping {
        iov.iov_base = ptr::addr_of_mut!(icmp6) as *mut c_void;
        iov.iov_len = mem::size_of_val(&icmp6);
    }

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;

    if use_flowinfo_send {
        msg.msg_name = ptr::addr_of_mut!(addr) as *mut c_void;
        msg.msg_namelen = mem::size_of_val(&addr) as socklen_t;
    } else if with_flowlabel {
        let cm: *mut cmsghdr;

        cm = control.as_mut_ptr() as *mut cmsghdr;
        (*cm).cmsg_len = cmsg_len(mem::size_of_val(&flowlabel)) as _;
        (*cm).cmsg_level = SOL_IPV6;
        (*cm).cmsg_type = IPV6_FLOWINFO;
        *(cmsg_data(cm) as *mut u32) = htonl(flowlabel);

        msg.msg_control = control.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = mem::size_of_val(&control) as _;
    }

    ret = sendmsg(fd, &msg, 0);
    if ret == -1 {
        error(1, *__errno_location(), c"send".as_ptr());
    }

    if with_flowlabel {
        fprintf(stderr, c"sent with label %u\n".as_ptr(), flowlabel);
    } else {
        fprintf(stderr, c"sent without label\n".as_ptr());
    }
}

unsafe fn do_recv(fd: c_int, with_flowlabel: bool, expect: u32) {
    let mut control = [0u8; cmsg_space(mem::size_of_val(&expect))];
    let mut data = [0u8; 2];
    let mut msg: msghdr = mem::zeroed();
    let mut iov: iovec = mem::zeroed();
    let mut cm: *mut cmsghdr;
    let flowlabel: u32;
    let ret: c_int;

    iov.iov_base = data.as_mut_ptr() as *mut c_void;
    iov.iov_len = mem::size_of_val(&data);

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;

    memset(
        control.as_mut_ptr() as *mut c_void,
        0,
        mem::size_of_val(&control),
    );
    msg.msg_control = control.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = mem::size_of_val(&control) as _;

    ret = recvmsg(fd, &mut msg, 0);
    if ret == -1 {
        error(1, *__errno_location(), c"recv".as_ptr());
    }
    if use_ping {
        // goto parse_cmsg;
    } else {
        if msg.msg_flags & (MSG_TRUNC | MSG_CTRUNC) != 0 {
            error(1, 0, c"recv: truncated".as_ptr());
        }
        if ret != mem::size_of_val(cfg_data) as c_int {
            error(1, 0, c"recv: length mismatch".as_ptr());
        }
        if memcmp(
            data.as_ptr() as *const c_void,
            cfg_data.as_ptr() as *const c_void,
            mem::size_of_val(&data),
        ) != 0
        {
            error(1, 0, c"recv: data mismatch".as_ptr());
        }
    }

    // parse_cmsg:
    cm = CMSG_FIRSTHDR(&msg);
    if with_flowlabel {
        if cm.is_null() {
            error(1, 0, c"recv: missing cmsg".as_ptr());
        }
        if !CMSG_NXTHDR(&msg, cm).is_null() {
            error(1, 0, c"recv: too many cmsg".as_ptr());
        }
        if (*cm).cmsg_level != SOL_IPV6 || (*cm).cmsg_type != IPV6_FLOWINFO {
            error(1, 0, c"recv: unexpected cmsg level or type".as_ptr());
        }

        flowlabel = ntohl(*(cmsg_data(cm) as *mut u32));
        fprintf(stderr, c"recv with label %u\n".as_ptr(), flowlabel);

        if expect != FLOWLABEL_WILDCARD && expect != flowlabel {
            fprintf(
                stderr,
                c"recv: incorrect flowlabel %u != %u\n".as_ptr(),
                flowlabel,
                expect,
            );
            error(1, 0, c"recv: flowlabel is wrong".as_ptr());
        }
    } else {
        fprintf(stderr, c"recv without label\n".as_ptr());
    }
}

unsafe fn get_autoflowlabel_enabled() -> bool {
    let fd: c_int;
    let ret: ssize_t;
    let mut val: c_char = 0;

    fd = open(c"/proc/sys/net/ipv6/auto_flowlabels".as_ptr(), O_RDONLY);
    if fd == -1 {
        error(1, *__errno_location(), c"open sysctl".as_ptr());
    }

    ret = read(fd, &mut val as *mut c_char as *mut c_void, 1);
    if ret == -1 {
        error(1, *__errno_location(), c"read sysctl".as_ptr());
    }
    if ret == 0 {
        error(1, 0, c"read sysctl: 0".as_ptr());
    }

    if close(fd) != 0 {
        error(1, *__errno_location(), c"close sysctl".as_ptr());
    }

    val == b'1' as c_char
}

unsafe fn flowlabel_get(fd: c_int, label: u32, share: u8, flags: u16) {
    let mut req = in6_flowlabel_req {
        flr_action: IPV6_FL_A_GET,
        flr_label: htonl(label),
        flr_flags: flags,
        flr_share: share,
        flr_dst: mem::zeroed(),
        flr_expires: 0,
        flr_linger: 0,
        __flr_pad: 0,
    };

    /* do not pass IPV6_ADDR_ANY or IPV6_ADDR_MAPPED */
    req.flr_dst.s6_addr[0] = 0xfd;
    req.flr_dst.s6_addr[15] = 0x1;

    if setsockopt(
        fd,
        SOL_IPV6,
        IPV6_FLOWLABEL_MGR,
        &req as *const in6_flowlabel_req as *const c_void,
        mem::size_of_val(&req) as socklen_t,
    ) != 0
    {
        error(1, *__errno_location(), c"setsockopt flowlabel get".as_ptr());
    }
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut c: c_int;

    loop {
        c = getopt(argc, argv, c"l:ps".as_ptr());
        if c == -1 {
            break;
        }

        match c as u8 as char {
            'l' => {
                cfg_label = strtoul(optarg, ptr::null_mut(), 0) as u32;
            }
            'p' => {
                use_ping = true;
            }
            's' => {
                use_flowinfo_send = true;
            }
            _ => {
                error(1, 0, c"%s: parse error".as_ptr(), *argv);
            }
        }
    }
}

unsafe fn real_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let one: c_int = 1;
    let fdt: c_int;
    let fdr: c_int;
    let mut prot: c_int = 0;

    addr.sin6_port = htons(8000);

    parse_opts(argc, argv);

    if use_ping {
        fprintf(stderr, c"attempting to use ping sockets\n".as_ptr());
        prot = IPPROTO_ICMPV6;
    }

    fdt = socket(PF_INET6, SOCK_DGRAM, prot);
    if fdt == -1 {
        error(1, *__errno_location(), c"socket t".as_ptr());
    }

    fdr = if use_ping {
        fdt
    } else {
        socket(PF_INET6, SOCK_DGRAM, 0)
    };
    if fdr == -1 {
        error(1, *__errno_location(), c"socket r".as_ptr());
    }

    if connect(
        fdt,
        ptr::addr_of_mut!(addr) as *mut c_void as *const sockaddr,
        mem::size_of_val(&addr) as socklen_t,
    ) != 0
    {
        error(1, *__errno_location(), c"connect".as_ptr());
    }
    if !use_ping
        && bind(
            fdr,
            ptr::addr_of_mut!(addr) as *mut c_void as *const sockaddr,
            mem::size_of_val(&addr) as socklen_t,
        ) != 0
    {
        error(1, *__errno_location(), c"bind".as_ptr());
    }

    flowlabel_get(fdt, cfg_label, IPV6_FL_S_EXCL, IPV6_FL_F_CREATE);

    if setsockopt(
        fdr,
        SOL_IPV6,
        IPV6_FLOWINFO,
        &one as *const c_int as *const c_void,
        mem::size_of_val(&one) as socklen_t,
    ) != 0
    {
        error(1, *__errno_location(), c"setsockopt flowinfo".as_ptr());
    }

    if get_autoflowlabel_enabled() {
        fprintf(stderr, c"send no label: recv auto flowlabel\n".as_ptr());
        do_send(fdt, false, 0);
        do_recv(fdr, true, FLOWLABEL_WILDCARD);
    } else {
        fprintf(stderr, c"send no label: recv no label (auto off)\n".as_ptr());
        do_send(fdt, false, 0);
        do_recv(fdr, false, 0);
    }

    if use_flowinfo_send {
        fprintf(stderr, c"using IPV6_FLOWINFO_SEND to send label\n".as_ptr());
        addr.sin6_flowinfo = htonl(cfg_label);
        if setsockopt(
            fdt,
            SOL_IPV6,
            IPV6_FLOWINFO_SEND,
            &one as *const c_int as *const c_void,
            mem::size_of_val(&one) as socklen_t,
        ) == -1
        {
            error(1, *__errno_location(), c"setsockopt flowinfo_send".as_ptr());
        }
    }

    fprintf(stderr, c"send label\n".as_ptr());
    do_send(fdt, true, cfg_label);
    do_recv(fdr, true, cfg_label);

    if close(fdr) != 0 {
        error(1, *__errno_location(), c"close r".as_ptr());
    }
    if !use_ping && close(fdt) != 0 {
        error(1, *__errno_location(), c"close t".as_ptr());
    }

    0
}

fn main() {
    let args: Vec<CString> = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();
    let mut argv: Vec<*mut c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .collect();
    argv.push(ptr::null_mut());

    unsafe {
        std::process::exit(real_main((argv.len() - 1) as c_int, argv.as_mut_ptr()));
    }
}
