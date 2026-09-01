// SPDX-License-Identifier: GPL-2.0

// C source dependency intent:
// _GNU_SOURCE plus system networking headers: arpa/inet.h, error.h, errno.h,
// net/if.h, linux/in.h, linux/netlink.h, linux/rtnetlink.h,
// netinet/if_ether.h, netinet/ip.h, netinet/ip6.h, netinet/udp.h,
// sys/ioctl.h, sys/socket.h, sys/stat.h, sys/time.h, sys/types.h, unistd.h.

use std::ffi::c_void;
use std::mem;
use std::ptr;

const ETH_MAX_MTU: usize = 0xFFFF;
const UDP_SEGMENT: libc::c_int = 103;
const UDP_MAX_SEGMENTS: usize = 1 << 7;

const CONST_MTU_TEST: libc::c_uint = 1500;

const CONST_HDRLEN_V4: usize = mem::size_of::<libc::iphdr>() + mem::size_of::<libc::udphdr>();
const CONST_HDRLEN_V6: usize = mem::size_of::<libc::ip6_hdr>() + mem::size_of::<libc::udphdr>();

const CONST_MSS_V4: usize = CONST_MTU_TEST as usize - CONST_HDRLEN_V4;
const CONST_MSS_V6: usize = CONST_MTU_TEST as usize - CONST_HDRLEN_V6;

const CONST_MAX_SEGS_V4: usize = ETH_MAX_MTU / CONST_MSS_V4;
const CONST_MAX_SEGS_V6: usize = ETH_MAX_MTU / CONST_MSS_V6;

const IP6_MAX_MTU: usize = ETH_MAX_MTU + mem::size_of::<libc::ip6_hdr>();

static mut CFG_DO_IPV4: bool = false;
static mut CFG_DO_IPV6: bool = false;
static mut CFG_DO_CONNECTED: bool = false;
static mut CFG_DO_CONNECTIONLESS: bool = false;
static mut CFG_DO_MSGMORE: bool = false;
static mut CFG_DO_RECV: bool = true;
static mut CFG_DO_SETSOCKOPT: bool = false;
static mut CFG_SPECIFIC_TEST_ID: libc::c_int = -1;

static mut CFG_PORT: libc::c_ushort = 9000;

static mut BUF: [libc::c_char; ETH_MAX_MTU] = [0; ETH_MAX_MTU];

#[repr(C)]
#[derive(Copy, Clone)]
struct testcase {
    tlen: libc::c_int,       /* send() buffer size, may exceed mss */
    tfail: bool,             /* send() call is expected to fail */
    gso_len: libc::c_int,    /* mss after applying gso */
    r_num_mss: libc::c_int,  /* recv(): number of calls of full mss */
    r_len_last: libc::c_int, /* recv(): size of last non-mss dgram, if any */
    v6_ext_hdr: bool,        /* send() dgrams with IPv6 extension headers */
}

const ADDR6: libc::in6_addr = libc::in6_addr {
    s6_addr: [0xfd, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], /* fd00::1 */
};

const ADDR4: libc::in_addr = libc::in_addr {
    s_addr: u32::from_be(0x0a000001), /* 10.0.0.1 */
};

static IPV6_HOPOPTS_PAD1: [libc::c_char; 8] = [0; 8];

static mut TESTCASES_V4: [testcase; 18] = [
    testcase {
        /* no GSO: send a single byte */
        tlen: 1,
        tfail: false,
        gso_len: 0,
        r_num_mss: 0,
        r_len_last: 1,
        v6_ext_hdr: false,
    },
    testcase {
        /* no GSO: send a single MSS */
        tlen: CONST_MSS_V4 as libc::c_int,
        tfail: false,
        gso_len: 0,
        r_num_mss: 1,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* no GSO: send a single MSS + 1B: fail */
        tlen: (CONST_MSS_V4 + 1) as libc::c_int,
        tfail: true,
        gso_len: 0,
        r_num_mss: 0,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send a single MSS: will fall back to no GSO */
        tlen: CONST_MSS_V4 as libc::c_int,
        tfail: false,
        gso_len: CONST_MSS_V4 as libc::c_int,
        r_num_mss: 1,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* datalen <= MSS < gso_len: will fall back to no GSO */
        tlen: CONST_MSS_V4 as libc::c_int,
        tfail: false,
        gso_len: (CONST_MSS_V4 + 1) as libc::c_int,
        r_num_mss: 0,
        r_len_last: CONST_MSS_V4 as libc::c_int,
        v6_ext_hdr: false,
    },
    testcase {
        /* MSS < datalen < gso_len: fail */
        tlen: (CONST_MSS_V4 + 1) as libc::c_int,
        tfail: true,
        gso_len: (CONST_MSS_V4 + 2) as libc::c_int,
        r_num_mss: 0,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send a single MSS + 1B */
        tlen: (CONST_MSS_V4 + 1) as libc::c_int,
        tfail: false,
        gso_len: CONST_MSS_V4 as libc::c_int,
        r_num_mss: 1,
        r_len_last: 1,
        v6_ext_hdr: false,
    },
    testcase {
        /* send exactly 2 MSS */
        tlen: (CONST_MSS_V4 * 2) as libc::c_int,
        tfail: false,
        gso_len: CONST_MSS_V4 as libc::c_int,
        r_num_mss: 2,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send 2 MSS + 1B */
        tlen: ((CONST_MSS_V4 * 2) + 1) as libc::c_int,
        tfail: false,
        gso_len: CONST_MSS_V4 as libc::c_int,
        r_num_mss: 2,
        r_len_last: 1,
        v6_ext_hdr: false,
    },
    testcase {
        /* send MAX segs */
        tlen: ((ETH_MAX_MTU / CONST_MSS_V4) * CONST_MSS_V4) as libc::c_int,
        tfail: false,
        gso_len: CONST_MSS_V4 as libc::c_int,
        r_num_mss: (ETH_MAX_MTU / CONST_MSS_V4) as libc::c_int,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send MAX bytes */
        tlen: (ETH_MAX_MTU - CONST_HDRLEN_V4) as libc::c_int,
        tfail: false,
        gso_len: CONST_MSS_V4 as libc::c_int,
        r_num_mss: CONST_MAX_SEGS_V4 as libc::c_int,
        r_len_last: (ETH_MAX_MTU - CONST_HDRLEN_V4 - (CONST_MAX_SEGS_V4 * CONST_MSS_V4))
            as libc::c_int,
        v6_ext_hdr: false,
    },
    testcase {
        /* send MAX + 1: fail */
        tlen: (ETH_MAX_MTU - CONST_HDRLEN_V4 + 1) as libc::c_int,
        tfail: true,
        gso_len: CONST_MSS_V4 as libc::c_int,
        r_num_mss: 0,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send a single 1B MSS: will fall back to no GSO */
        tlen: 1,
        tfail: false,
        gso_len: 1,
        r_num_mss: 1,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send 2 1B segments */
        tlen: 2,
        tfail: false,
        gso_len: 1,
        r_num_mss: 2,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send 2B + 2B + 1B segments */
        tlen: 5,
        tfail: false,
        gso_len: 2,
        r_num_mss: 2,
        r_len_last: 1,
        v6_ext_hdr: false,
    },
    testcase {
        /* send max number of min sized segments */
        tlen: UDP_MAX_SEGMENTS as libc::c_int,
        tfail: false,
        gso_len: 1,
        r_num_mss: UDP_MAX_SEGMENTS as libc::c_int,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send max number + 1 of min sized segments: fail */
        tlen: (UDP_MAX_SEGMENTS + 1) as libc::c_int,
        tfail: true,
        gso_len: 1,
        r_num_mss: 0,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* EOL */
        tlen: 0,
        tfail: false,
        gso_len: 0,
        r_num_mss: 0,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
];

static mut TESTCASES_V6: [testcase; 19] = [
    testcase {
        /* no GSO: send a single byte */
        tlen: 1,
        tfail: false,
        gso_len: 0,
        r_num_mss: 0,
        r_len_last: 1,
        v6_ext_hdr: false,
    },
    testcase {
        /* no GSO: send a single MSS */
        tlen: CONST_MSS_V6 as libc::c_int,
        tfail: false,
        gso_len: 0,
        r_num_mss: 1,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* no GSO: send a single MSS + 1B: fail */
        tlen: (CONST_MSS_V6 + 1) as libc::c_int,
        tfail: true,
        gso_len: 0,
        r_num_mss: 0,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send a single MSS: will fall back to no GSO */
        tlen: CONST_MSS_V6 as libc::c_int,
        tfail: false,
        gso_len: CONST_MSS_V6 as libc::c_int,
        r_num_mss: 1,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* datalen <= MSS < gso_len: will fall back to no GSO */
        tlen: CONST_MSS_V6 as libc::c_int,
        tfail: false,
        gso_len: (CONST_MSS_V6 + 1) as libc::c_int,
        r_num_mss: 0,
        r_len_last: CONST_MSS_V6 as libc::c_int,
        v6_ext_hdr: false,
    },
    testcase {
        /* MSS < datalen < gso_len: fail */
        tlen: (CONST_MSS_V6 + 1) as libc::c_int,
        tfail: true,
        gso_len: (CONST_MSS_V6 + 2) as libc::c_int,
        r_num_mss: 0,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send a single MSS + 1B */
        tlen: (CONST_MSS_V6 + 1) as libc::c_int,
        tfail: false,
        gso_len: CONST_MSS_V6 as libc::c_int,
        r_num_mss: 1,
        r_len_last: 1,
        v6_ext_hdr: false,
    },
    testcase {
        /* send exactly 2 MSS */
        tlen: (CONST_MSS_V6 * 2) as libc::c_int,
        tfail: false,
        gso_len: CONST_MSS_V6 as libc::c_int,
        r_num_mss: 2,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send 2 MSS + 1B */
        tlen: ((CONST_MSS_V6 * 2) + 1) as libc::c_int,
        tfail: false,
        gso_len: CONST_MSS_V6 as libc::c_int,
        r_num_mss: 2,
        r_len_last: 1,
        v6_ext_hdr: false,
    },
    testcase {
        /* send MAX segs */
        tlen: ((IP6_MAX_MTU / CONST_MSS_V6) * CONST_MSS_V6) as libc::c_int,
        tfail: false,
        gso_len: CONST_MSS_V6 as libc::c_int,
        r_num_mss: (IP6_MAX_MTU / CONST_MSS_V6) as libc::c_int,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send MAX bytes */
        tlen: (IP6_MAX_MTU - CONST_HDRLEN_V6) as libc::c_int,
        tfail: false,
        gso_len: CONST_MSS_V6 as libc::c_int,
        r_num_mss: CONST_MAX_SEGS_V6 as libc::c_int,
        r_len_last: (IP6_MAX_MTU - CONST_HDRLEN_V6 - (CONST_MAX_SEGS_V6 * CONST_MSS_V6))
            as libc::c_int,
        v6_ext_hdr: false,
    },
    testcase {
        /* send MAX + 1: fail */
        tlen: (IP6_MAX_MTU - CONST_HDRLEN_V6 + 1) as libc::c_int,
        tfail: true,
        gso_len: CONST_MSS_V6 as libc::c_int,
        r_num_mss: 0,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send a single 1B MSS: will fall back to no GSO */
        tlen: 1,
        tfail: false,
        gso_len: 1,
        r_num_mss: 1,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send 2 1B segments */
        tlen: 2,
        tfail: false,
        gso_len: 1,
        r_num_mss: 2,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send 2 1B segments with extension headers */
        tlen: 2,
        tfail: false,
        gso_len: 1,
        r_num_mss: 2,
        r_len_last: 0,
        v6_ext_hdr: true,
    },
    testcase {
        /* send 2B + 2B + 1B segments */
        tlen: 5,
        tfail: false,
        gso_len: 2,
        r_num_mss: 2,
        r_len_last: 1,
        v6_ext_hdr: false,
    },
    testcase {
        /* send max number of min sized segments */
        tlen: UDP_MAX_SEGMENTS as libc::c_int,
        tfail: false,
        gso_len: 1,
        r_num_mss: UDP_MAX_SEGMENTS as libc::c_int,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* send max number + 1 of min sized segments: fail */
        tlen: (UDP_MAX_SEGMENTS + 1) as libc::c_int,
        tfail: true,
        gso_len: 1,
        r_num_mss: 0,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
    testcase {
        /* EOL */
        tlen: 0,
        tfail: false,
        gso_len: 0,
        r_num_mss: 0,
        r_len_last: 0,
        v6_ext_hdr: false,
    },
];

unsafe fn set_pmtu_discover(fd: libc::c_int, is_ipv4: bool) {
    let level: libc::c_int;
    let name: libc::c_int;
    let val: libc::c_int;

    if is_ipv4 {
        level = libc::SOL_IP;
        name = libc::IP_MTU_DISCOVER;
        val = libc::IP_PMTUDISC_DO;
    } else {
        level = libc::SOL_IPV6;
        name = libc::IPV6_MTU_DISCOVER;
        val = libc::IPV6_PMTUDISC_DO;
    }

    if libc::setsockopt(
        fd,
        level,
        name,
        &val as *const _ as *const c_void,
        mem::size_of_val(&val) as libc::socklen_t,
    ) != 0
    {
        libc::error(1, *libc::__errno_location(), c"setsockopt path mtu".as_ptr());
    }
}

unsafe fn get_path_mtu(fd: libc::c_int, is_ipv4: bool) -> libc::c_uint {
    let mut vallen: libc::socklen_t;
    let mut mtu: libc::c_uint = 0;
    let ret: libc::c_int;

    vallen = mem::size_of_val(&mtu) as libc::socklen_t;
    if is_ipv4 {
        ret = libc::getsockopt(
            fd,
            libc::SOL_IP,
            libc::IP_MTU,
            &mut mtu as *mut _ as *mut c_void,
            &mut vallen,
        );
    } else {
        ret = libc::getsockopt(
            fd,
            libc::SOL_IPV6,
            libc::IPV6_MTU,
            &mut mtu as *mut _ as *mut c_void,
            &mut vallen,
        );
    }

    if ret != 0 {
        libc::error(1, *libc::__errno_location(), c"getsockopt mtu".as_ptr());
    }

    libc::fprintf(libc::stderr, c"path mtu (read):  %u\n".as_ptr(), mtu);
    mtu
}

const fn cmsg_align(len: usize) -> usize {
    (len + mem::size_of::<usize>() - 1) & !(mem::size_of::<usize>() - 1)
}

const fn cmsg_space(len: usize) -> usize {
    cmsg_align(mem::size_of::<libc::cmsghdr>()) + cmsg_align(len)
}

const fn cmsg_len(len: usize) -> usize {
    cmsg_align(mem::size_of::<libc::cmsghdr>()) + len
}

unsafe fn cmsg_firsthdr(msg: *mut libc::msghdr) -> *mut libc::cmsghdr {
    if (*msg).msg_controllen as usize >= mem::size_of::<libc::cmsghdr>() {
        (*msg).msg_control as *mut libc::cmsghdr
    } else {
        ptr::null_mut()
    }
}

unsafe fn cmsg_data(cmsg: *mut libc::cmsghdr) -> *mut libc::c_uchar {
    (cmsg as *mut libc::c_uchar).add(cmsg_align(mem::size_of::<libc::cmsghdr>()))
}

unsafe fn __send_one(fd: libc::c_int, msg: *mut libc::msghdr, flags: libc::c_int) -> bool {
    let ret: libc::c_int;

    ret = libc::sendmsg(fd, msg, flags) as libc::c_int;
    if ret == -1
        && (*libc::__errno_location() == libc::EMSGSIZE
            || *libc::__errno_location() == libc::ENOMEM
            || *libc::__errno_location() == libc::EINVAL)
    {
        return false;
    }
    if ret == -1 {
        libc::error(1, *libc::__errno_location(), c"sendmsg".as_ptr());
    }
    if ret as usize != (*(*msg).msg_iov).iov_len {
        libc::error(
            1,
            0,
            c"sendto: %d != %llu".as_ptr(),
            ret,
            (*(*msg).msg_iov).iov_len as libc::c_ulonglong,
        );
    }
    if (*msg).msg_flags != 0 {
        libc::error(
            1,
            0,
            c"sendmsg: return flags 0x%x\n".as_ptr(),
            (*msg).msg_flags,
        );
    }

    true
}

unsafe fn send_one(
    fd: libc::c_int,
    len: libc::c_int,
    gso_len: libc::c_int,
    addr: *mut libc::sockaddr,
    alen: libc::socklen_t,
) -> bool {
    let mut control: [libc::c_char; cmsg_space(mem::size_of::<u16>())] =
        [0; cmsg_space(mem::size_of::<u16>())];
    let mut msg: libc::msghdr = mem::zeroed();
    let mut iov: libc::iovec = mem::zeroed();
    let cm: *mut libc::cmsghdr;

    iov.iov_base = BUF.as_mut_ptr() as *mut c_void;
    iov.iov_len = len as usize;

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;

    msg.msg_name = addr as *mut c_void;
    msg.msg_namelen = alen;

    if gso_len != 0 && !CFG_DO_SETSOCKOPT {
        msg.msg_control = control.as_mut_ptr() as *mut c_void;
        msg.msg_controllen = mem::size_of_val(&control);

        cm = cmsg_firsthdr(&mut msg);
        (*cm).cmsg_level = libc::SOL_UDP;
        (*cm).cmsg_type = UDP_SEGMENT;
        (*cm).cmsg_len = cmsg_len(mem::size_of::<u16>());
        *(cmsg_data(cm) as *mut u16) = gso_len as u16;
    }

    /* If MSG_MORE, send 1 byte followed by remainder */
    if CFG_DO_MSGMORE && len > 1 {
        iov.iov_len = 1;
        if !__send_one(fd, &mut msg, libc::MSG_MORE) {
            libc::error(1, 0, c"send 1B failed".as_ptr());
        }

        iov.iov_base = (iov.iov_base as *mut libc::c_char).add(1) as *mut c_void;
        iov.iov_len = (len - 1) as usize;
    }

    __send_one(fd, &mut msg, 0)
}

unsafe fn recv_one(fd: libc::c_int, flags: libc::c_int) -> libc::c_int {
    let ret: libc::c_int;

    ret = libc::recv(
        fd,
        BUF.as_mut_ptr() as *mut c_void,
        mem::size_of_val(&BUF),
        flags,
    ) as libc::c_int;
    if ret == -1 && *libc::__errno_location() == libc::EAGAIN && (flags & libc::MSG_DONTWAIT) != 0
    {
        return 0;
    }
    if ret == -1 {
        libc::error(1, *libc::__errno_location(), c"recv".as_ptr());
    }

    ret
}

unsafe fn run_one(
    test: *mut testcase,
    fdt: libc::c_int,
    fdr: libc::c_int,
    addr: *mut libc::sockaddr,
    alen: libc::socklen_t,
) {
    let mut i: libc::c_int;
    let mut ret: libc::c_int;
    let mut val: libc::c_int;
    let mss: libc::c_int;
    let sent: bool;

    libc::fprintf(
        libc::stderr,
        c"ipv%d tx:%d gso:%d %s%s\n".as_ptr(),
        if (*addr).sa_family as libc::c_int == libc::AF_INET {
            4
        } else {
            6
        },
        (*test).tlen,
        (*test).gso_len,
        if (*test).v6_ext_hdr {
            c"ext-hdr ".as_ptr()
        } else {
            c"".as_ptr()
        },
        if (*test).tfail {
            c"(fail)".as_ptr()
        } else {
            c"".as_ptr()
        },
    );

    if (*test).v6_ext_hdr {
        if libc::setsockopt(
            fdt,
            libc::IPPROTO_IPV6,
            libc::IPV6_HOPOPTS,
            IPV6_HOPOPTS_PAD1.as_ptr() as *const c_void,
            mem::size_of_val(&IPV6_HOPOPTS_PAD1) as libc::socklen_t,
        ) != 0
        {
            libc::error(
                1,
                *libc::__errno_location(),
                c"setsockopt ipv6 hopopts".as_ptr(),
            );
        }
    }

    val = (*test).gso_len;
    if CFG_DO_SETSOCKOPT {
        if libc::setsockopt(
            fdt,
            libc::SOL_UDP,
            UDP_SEGMENT,
            &val as *const _ as *const c_void,
            mem::size_of_val(&val) as libc::socklen_t,
        ) != 0
        {
            libc::error(
                1,
                *libc::__errno_location(),
                c"setsockopt udp segment".as_ptr(),
            );
        }
    }

    sent = send_one(fdt, (*test).tlen, (*test).gso_len, addr, alen);
    if sent && (*test).tfail {
        libc::error(1, 0, c"send succeeded while expecting failure".as_ptr());
    }
    if !sent && !(*test).tfail {
        libc::error(1, 0, c"send failed while expecting success".as_ptr());
    }

    if (*test).v6_ext_hdr {
        if libc::setsockopt(
            fdt,
            libc::IPPROTO_IPV6,
            libc::IPV6_HOPOPTS,
            ptr::null(),
            0,
        ) != 0
        {
            libc::error(
                1,
                *libc::__errno_location(),
                c"setsockopt ipv6 hopopts clear".as_ptr(),
            );
        }
    }

    if !sent {
        return;
    }

    if !CFG_DO_RECV {
        return;
    }

    if (*test).gso_len != 0 {
        mss = (*test).gso_len;
    } else {
        mss = if (*addr).sa_family as libc::c_int == libc::AF_INET {
            CONST_MSS_V4 as libc::c_int
        } else {
            CONST_MSS_V6 as libc::c_int
        };
    }

    /* Recv all full MSS datagrams */
    i = 0;
    while i < (*test).r_num_mss {
        ret = recv_one(fdr, 0);
        if ret != mss {
            libc::error(1, 0, c"recv.%d: %d != %d".as_ptr(), i, ret, mss);
        }
        i += 1;
    }

    /* Recv the non-full last datagram, if tlen was not a multiple of mss */
    if (*test).r_len_last != 0 {
        ret = recv_one(fdr, 0);
        if ret != (*test).r_len_last {
            libc::error(
                1,
                0,
                c"recv.%d: %d != %d (last)".as_ptr(),
                i,
                ret,
                (*test).r_len_last,
            );
        }
    }

    /* Verify received all data */
    ret = recv_one(fdr, libc::MSG_DONTWAIT);
    if ret != 0 {
        libc::error(1, 0, c"recv: unexpected datagram".as_ptr());
    }
}

unsafe fn run_all(
    fdt: libc::c_int,
    fdr: libc::c_int,
    addr: *mut libc::sockaddr,
    alen: libc::socklen_t,
) {
    let tests: *mut testcase;
    let mut test: *mut testcase;

    tests = if (*addr).sa_family as libc::c_int == libc::AF_INET {
        TESTCASES_V4.as_mut_ptr()
    } else {
        TESTCASES_V6.as_mut_ptr()
    };

    test = tests;
    while (*test).tlen != 0 {
        /* if a specific test is given, then skip all others */
        if CFG_SPECIFIC_TEST_ID == -1 || CFG_SPECIFIC_TEST_ID == test.offset_from(tests) as i32 {
            run_one(test, fdt, fdr, addr, alen);
        }
        test = test.add(1);
    }
}

unsafe fn run_test(addr: *mut libc::sockaddr, alen: libc::socklen_t) {
    let tv: libc::timeval = libc::timeval {
        tv_sec: 0,
        tv_usec: 100 * 1000,
    };
    let fdr: libc::c_int;
    let fdt: libc::c_int;
    let mut val: libc::c_int;

    fdr = libc::socket((*addr).sa_family as libc::c_int, libc::SOCK_DGRAM, 0);
    if fdr == -1 {
        libc::error(1, *libc::__errno_location(), c"socket r".as_ptr());
    }

    if CFG_DO_RECV {
        if libc::bind(fdr, addr, alen) != 0 {
            libc::error(1, *libc::__errno_location(), c"bind".as_ptr());
        }
    }

    /* Have tests fail quickly instead of hang */
    if libc::setsockopt(
        fdr,
        libc::SOL_SOCKET,
        libc::SO_RCVTIMEO,
        &tv as *const _ as *const c_void,
        mem::size_of_val(&tv) as libc::socklen_t,
    ) != 0
    {
        libc::error(
            1,
            *libc::__errno_location(),
            c"setsockopt rcv timeout".as_ptr(),
        );
    }

    fdt = libc::socket((*addr).sa_family as libc::c_int, libc::SOCK_DGRAM, 0);
    if fdt == -1 {
        libc::error(1, *libc::__errno_location(), c"socket t".as_ptr());
    }

    /* Do not fragment these datagrams: only succeed if GSO works */
    set_pmtu_discover(fdt, (*addr).sa_family as libc::c_int == libc::AF_INET);

    if CFG_DO_CONNECTIONLESS {
        run_all(fdt, fdr, addr, alen);
    }

    if CFG_DO_CONNECTED {
        if libc::connect(fdt, addr, alen) != 0 {
            libc::error(1, *libc::__errno_location(), c"connect".as_ptr());
        }

        val = get_path_mtu(fdt, (*addr).sa_family as libc::c_int == libc::AF_INET) as libc::c_int;
        if val as libc::c_uint != CONST_MTU_TEST {
            libc::error(1, 0, c"bad path mtu %u\n".as_ptr(), val);
        }

        run_all(fdt, fdr, addr, 0 /* use connected addr */);
    }

    if libc::close(fdt) != 0 {
        libc::error(1, *libc::__errno_location(), c"close t".as_ptr());
    }
    if libc::close(fdr) != 0 {
        libc::error(1, *libc::__errno_location(), c"close r".as_ptr());
    }
}

unsafe fn run_test_v4() {
    let mut addr: libc::sockaddr_in = mem::zeroed();

    addr.sin_family = libc::AF_INET as libc::sa_family_t;
    addr.sin_port = libc::htons(CFG_PORT);
    addr.sin_addr = ADDR4;

    run_test(
        &mut addr as *mut _ as *mut libc::sockaddr,
        mem::size_of_val(&addr) as libc::socklen_t,
    );
}

unsafe fn run_test_v6() {
    let mut addr: libc::sockaddr_in6 = mem::zeroed();

    addr.sin6_family = libc::AF_INET6 as libc::sa_family_t;
    addr.sin6_port = libc::htons(CFG_PORT);
    addr.sin6_addr = ADDR6;

    run_test(
        &mut addr as *mut _ as *mut libc::sockaddr,
        mem::size_of_val(&addr) as libc::socklen_t,
    );
}

unsafe fn parse_opts(argc: libc::c_int, argv: *mut *mut libc::c_char) {
    let mut c: libc::c_int;

    loop {
        c = libc::getopt(argc, argv, c"46cCmRst:".as_ptr());
        if c == -1 {
            break;
        }
        match c as u8 as char {
            '4' => {
                CFG_DO_IPV4 = true;
            }
            '6' => {
                CFG_DO_IPV6 = true;
            }
            'c' => {
                CFG_DO_CONNECTED = true;
            }
            'C' => {
                CFG_DO_CONNECTIONLESS = true;
            }
            'm' => {
                CFG_DO_MSGMORE = true;
            }
            'R' => {
                CFG_DO_RECV = false;
            }
            's' => {
                CFG_DO_SETSOCKOPT = true;
            }
            't' => {
                CFG_SPECIFIC_TEST_ID =
                    libc::strtoul(libc::optarg, ptr::null_mut(), 0) as libc::c_int;
            }
            _ => {
                libc::error(1, 0, c"%s: parse error".as_ptr(), *argv);
            }
        }
    }
}

unsafe fn main_0(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
    parse_opts(argc, argv);

    if CFG_DO_IPV4 {
        run_test_v4();
    }
    if CFG_DO_IPV6 {
        run_test_v6();
    }

    libc::fprintf(libc::stderr, c"OK\n".as_ptr());
    0
}

fn main() {
    unsafe {
        let mut argv: Vec<*mut libc::c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        argv.push(ptr::null_mut());
        let argc = (argv.len() - 1) as libc::c_int;
        let ret = main_0(argc, argv.as_mut_ptr());
        for arg in argv.into_iter().take(argc as usize) {
            let _ = std::ffi::CString::from_raw(arg);
        }
        std::process::exit(ret);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
