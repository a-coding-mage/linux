// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025, Kylin Software */

use std::ffi::c_void;
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_ushort};
use std::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type socklen_t = u32;
type size_t = usize;
type ssize_t = isize;

const AF_INET: c_int = 2;
const AF_NETLINK: c_int = 16;
const SOCK_RAW: c_int = 3;
const NETLINK_SOCK_DIAG: c_int = 4;
const IPPROTO_TCP: __u32 = 6;

#[cfg(not(any()))]
const IPPROTO_MPTCP_FROM_HEADERS: __u32 = 262;
const IPPROTO_MPTCP: __u32 = 262;

const SOCK_DIAG_BY_FAMILY: __u16 = 20;
const NLM_F_REQUEST: __u16 = 1;
const NLMSG_NOOP: __u16 = 1;
const NLMSG_ERROR: __u16 = 2;
const NLMSG_DONE: __u16 = 3;

const INET_DIAG_REQ_PROTOCOL: c_int = 1;
const INET_DIAG_INFO: usize = 2;
const INET_DIAG_ULP_INFO: usize = 8;
const INET_DIAG_MAX: usize = 21;
const INET_ULP_INFO_MPTCP: usize = 1;
const INET_ULP_INFO_MAX: usize = 2;
const INET_DIAG_NOCOOKIE: __u32 = 0xffffffff;
const NLA_F_NESTED: c_ushort = 1 << 15;
const EINTR: c_int = 4;

#[repr(C)]
struct params {
    target_token: __u32,
    subflow_addrs: [c_char; 1024],
}

#[repr(C)]
struct mptcp_info {
    mptcpi_subflows: __u8,
    mptcpi_add_addr_signal: __u8,
    mptcpi_add_addr_accepted: __u8,
    mptcpi_subflows_max: __u8,
    mptcpi_add_addr_signal_max: __u8,
    mptcpi_add_addr_accepted_max: __u8,
    mptcpi_flags: __u32,
    mptcpi_token: __u32,
    mptcpi_write_seq: __u64,
    mptcpi_snd_una: __u64,
    mptcpi_rcv_nxt: __u64,
    mptcpi_local_addr_used: __u8,
    mptcpi_local_addr_max: __u8,
    mptcpi_csum_enabled: __u8,
    mptcpi_retransmits: __u32,
    mptcpi_bytes_retrans: __u64,
    mptcpi_bytes_sent: __u64,
    mptcpi_bytes_received: __u64,
    mptcpi_bytes_acked: __u64,
    mptcpi_subflows_total: __u8,
    reserved: [__u8; 3],
    mptcpi_last_data_sent: __u32,
    mptcpi_last_data_recv: __u32,
    mptcpi_last_ack_recv: __u32,
}

const MPTCP_SUBFLOW_ATTR_UNSPEC: usize = 0;
const MPTCP_SUBFLOW_ATTR_TOKEN_REM: usize = 1;
const MPTCP_SUBFLOW_ATTR_TOKEN_LOC: usize = 2;
const MPTCP_SUBFLOW_ATTR_RELWRITE_SEQ: usize = 3;
const MPTCP_SUBFLOW_ATTR_MAP_SEQ: usize = 4;
const MPTCP_SUBFLOW_ATTR_MAP_SFSEQ: usize = 5;
const MPTCP_SUBFLOW_ATTR_SSN_OFFSET: usize = 6;
const MPTCP_SUBFLOW_ATTR_MAP_DATALEN: usize = 7;
const MPTCP_SUBFLOW_ATTR_FLAGS: usize = 8;
const MPTCP_SUBFLOW_ATTR_ID_REM: usize = 9;
const MPTCP_SUBFLOW_ATTR_ID_LOC: usize = 10;
const MPTCP_SUBFLOW_ATTR_PAD: usize = 11;
const __MPTCP_SUBFLOW_ATTR_MAX: usize = 12;

const MPTCP_SUBFLOW_ATTR_MAX: usize = __MPTCP_SUBFLOW_ATTR_MAX - 1;

const fn _BITUL(nr: c_ulong) -> c_ulong {
    1 c_ulong << nr
}

const MPTCP_SUBFLOW_FLAG_MCAP_REM: c_ulong = _BITUL(0);
const MPTCP_SUBFLOW_FLAG_MCAP_LOC: c_ulong = _BITUL(1);
const MPTCP_SUBFLOW_FLAG_JOIN_REM: c_ulong = _BITUL(2);
const MPTCP_SUBFLOW_FLAG_JOIN_LOC: c_ulong = _BITUL(3);
const MPTCP_SUBFLOW_FLAG_BKUP_REM: c_ulong = _BITUL(4);
const MPTCP_SUBFLOW_FLAG_BKUP_LOC: c_ulong = _BITUL(5);
const MPTCP_SUBFLOW_FLAG_FULLY_ESTABLISHED: c_ulong = _BITUL(6);
const MPTCP_SUBFLOW_FLAG_CONNECTED: c_ulong = _BITUL(7);
const MPTCP_SUBFLOW_FLAG_MAPVALID: c_ulong = _BITUL(8);

#[repr(C)]
struct sockaddr_nl {
    nl_family: __u16,
    nl_pad: __u16,
    nl_pid: __u32,
    nl_groups: __u32,
}

#[repr(C)]
struct sockaddr {
    sa_family: __u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct nlmsghdr {
    nlmsg_len: __u32,
    nlmsg_type: __u16,
    nlmsg_flags: __u16,
    nlmsg_seq: __u32,
    nlmsg_pid: __u32,
}

#[repr(C)]
struct nlmsgerr {
    error: c_int,
    msg: nlmsghdr,
}

#[repr(C)]
struct rtattr {
    rta_len: c_ushort,
    rta_type: c_ushort,
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
struct inet_diag_sockid {
    idiag_sport: __u16,
    idiag_dport: __u16,
    idiag_src: [__u32; 4],
    idiag_dst: [__u32; 4],
    idiag_if: __u32,
    idiag_cookie: [__u32; 2],
}

#[repr(C)]
struct inet_diag_req_v2 {
    sdiag_family: __u8,
    sdiag_protocol: __u8,
    idiag_ext: __u8,
    pad: __u8,
    idiag_states: __u32,
    id: inet_diag_sockid,
}

#[repr(C)]
struct inet_diag_msg {
    idiag_family: __u8,
    idiag_state: __u8,
    idiag_timer: __u8,
    idiag_retrans: __u8,
    id: inet_diag_sockid,
    idiag_expires: __u32,
    idiag_rqueue: __u32,
    idiag_wqueue: __u32,
    idiag_uid: __u32,
    idiag_inode: __u32,
}

unsafe extern "C" {
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn htons(hostshort: __u16) -> __u16;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;

    static mut errno: c_int;
    static mut stderr: *mut c_void;
    static mut optarg: *mut c_char;
}

const fn nlmsg_align(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

const fn nlmsg_length(len: usize) -> usize {
    len + nlmsg_align(mem::size_of::<nlmsghdr>())
}

unsafe fn nlmsg_data(nlh: *mut nlmsghdr) -> *mut c_void {
    (nlh as *mut u8).add(nlmsg_length(0)) as *mut c_void
}

unsafe fn nlmsg_ok(nlh: *mut nlmsghdr, len: c_int) -> bool {
    len >= mem::size_of::<nlmsghdr>() as c_int
        && (*nlh).nlmsg_len >= mem::size_of::<nlmsghdr>() as __u32
        && (*nlh).nlmsg_len as c_int <= len
}

unsafe fn nlmsg_next(nlh: *mut nlmsghdr, len: &mut c_int) -> *mut nlmsghdr {
    let aligned = nlmsg_align((*nlh).nlmsg_len as usize) as c_int;
    *len -= aligned;
    (nlh as *mut u8).add(aligned as usize) as *mut nlmsghdr
}

const fn rta_align(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

const fn rta_length(len: usize) -> c_ushort {
    (rta_align(mem::size_of::<rtattr>()) + len) as c_ushort
}

unsafe fn rta_data(rta: *mut rtattr) -> *mut c_void {
    (rta as *mut u8).add(rta_length(0) as usize) as *mut c_void
}

unsafe fn rta_payload(rta: *mut rtattr) -> c_int {
    (*rta).rta_len as c_int - rta_length(0) as c_int
}

unsafe fn rta_ok(rta: *mut rtattr, len: c_int) -> bool {
    len >= mem::size_of::<rtattr>() as c_int
        && (*rta).rta_len >= mem::size_of::<rtattr>() as c_ushort
        && (*rta).rta_len as c_int <= len
}

unsafe fn rta_next(rta: *mut rtattr, len: &mut c_int) -> *mut rtattr {
    let aligned = rta_align((*rta).rta_len as usize) as c_int;
    *len -= aligned;
    (rta as *mut u8).add(aligned as usize) as *mut rtattr
}

unsafe fn rta_getattr<T: Copy>(value: *mut rtattr) -> T {
    *(rta_data(value) as *const T)
}

unsafe fn die_perror(msg: *const c_char) -> ! {
    perror(msg);
    exit(1);
}

unsafe fn die_usage(r: c_int) {
    fprintf(
        stderr,
        c"Usage:\nmptcp_diag -t <token>\nmptcp_diag -s \"<saddr>:<sport> <daddr>:<dport>\"\n".as_ptr(),
    );
    exit(r);
}

unsafe fn send_query(fd: c_int, r: *mut inet_diag_req_v2, proto: __u32) {
    let mut nladdr = sockaddr_nl {
        nl_family: AF_NETLINK as __u16,
        nl_pad: 0,
        nl_pid: 0,
        nl_groups: 0,
    };
    #[repr(C)]
    struct req_t {
        nlh: nlmsghdr,
        r: inet_diag_req_v2,
    }
    let mut req = req_t {
        nlh: nlmsghdr {
            nlmsg_len: mem::size_of::<req_t>() as __u32,
            nlmsg_type: SOCK_DIAG_BY_FAMILY,
            nlmsg_flags: NLM_F_REQUEST,
            nlmsg_seq: 0,
            nlmsg_pid: 0,
        },
        r: ptr::read(r),
    };
    let mut rta_proto: rtattr = mem::zeroed();
    let mut iov: [iovec; 6] = mem::zeroed();
    let mut iovlen: c_int = 0;

    iov[iovlen as usize] = iovec {
        iov_base: &mut req as *mut _ as *mut c_void,
        iov_len: mem::size_of::<req_t>(),
    };
    iovlen += 1;

    if proto == IPPROTO_MPTCP {
        rta_proto.rta_type = INET_DIAG_REQ_PROTOCOL as c_ushort;
        rta_proto.rta_len = rta_length(mem::size_of_val(&proto));

        iov[iovlen as usize] = iovec {
            iov_base: &mut rta_proto as *mut _ as *mut c_void,
            iov_len: mem::size_of_val(&rta_proto),
        };
        iovlen += 1;
        iov[iovlen as usize] = iovec {
            iov_base: &proto as *const _ as *mut c_void,
            iov_len: mem::size_of_val(&proto),
        };
        iovlen += 1;
        req.nlh.nlmsg_len += rta_length(mem::size_of_val(&proto)) as __u32;
    }

    let mut msg = msghdr {
        msg_name: &mut nladdr as *mut _ as *mut c_void,
        msg_namelen: mem::size_of_val(&nladdr) as socklen_t,
        msg_iov: iov.as_mut_ptr(),
        msg_iovlen: iovlen as size_t,
        msg_control: ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };

    loop {
        if sendmsg(fd, &mut msg, 0) < 0 {
            if errno == EINTR {
                continue;
            }
            die_perror(c"sendmsg".as_ptr());
        }
        break;
    }
}

unsafe fn parse_rtattr_flags(
    tb: *mut *mut rtattr,
    max: c_int,
    mut rta: *mut rtattr,
    mut len: c_int,
    flags: c_ushort,
) {
    let mut type_: c_ushort;

    memset(
        tb as *mut c_void,
        0,
        mem::size_of::<*mut rtattr>() * (max as usize + 1),
    );
    while rta_ok(rta, len) {
        type_ = (*rta).rta_type & !flags;
        if type_ as c_int <= max && (*tb.add(type_ as usize)).is_null() {
            *tb.add(type_ as usize) = rta;
        }
        rta = rta_next(rta, &mut len);
    }
}

unsafe fn print_info_msg(info: *mut mptcp_info) {
    printf(c"Token & Flags\n".as_ptr());
    printf(c"token:        %x\n".as_ptr(), (*info).mptcpi_token);
    printf(c"flags:        %x\n".as_ptr(), (*info).mptcpi_flags);
    printf(c"csum_enabled: %u\n".as_ptr(), (*info).mptcpi_csum_enabled as c_uint);

    printf(c"\nBasic Info\n".as_ptr());
    printf(c"subflows:              %u\n".as_ptr(), (*info).mptcpi_subflows as c_uint);
    printf(c"subflows_max:          %u\n".as_ptr(), (*info).mptcpi_subflows_max as c_uint);
    printf(c"subflows_total:        %u\n".as_ptr(), (*info).mptcpi_subflows_total as c_uint);
    printf(c"local_addr_used:       %u\n".as_ptr(), (*info).mptcpi_local_addr_used as c_uint);
    printf(c"local_addr_max:        %u\n".as_ptr(), (*info).mptcpi_local_addr_max as c_uint);
    printf(c"add_addr_signal:       %u\n".as_ptr(), (*info).mptcpi_add_addr_signal as c_uint);
    printf(c"add_addr_accepted:     %u\n".as_ptr(), (*info).mptcpi_add_addr_accepted as c_uint);
    printf(c"add_addr_signal_max:   %u\n".as_ptr(), (*info).mptcpi_add_addr_signal_max as c_uint);
    printf(c"add_addr_accepted_max: %u\n".as_ptr(), (*info).mptcpi_add_addr_accepted_max as c_uint);

    printf(c"\nTransmission Info\n".as_ptr());
    printf(c"write_seq:        %llu\n".as_ptr(), (*info).mptcpi_write_seq);
    printf(c"snd_una:          %llu\n".as_ptr(), (*info).mptcpi_snd_una);
    printf(c"rcv_nxt:          %llu\n".as_ptr(), (*info).mptcpi_rcv_nxt);
    printf(c"last_data_sent:   %u\n".as_ptr(), (*info).mptcpi_last_data_sent);
    printf(c"last_data_recv:   %u\n".as_ptr(), (*info).mptcpi_last_data_recv);
    printf(c"last_ack_recv:    %u\n".as_ptr(), (*info).mptcpi_last_ack_recv);
    printf(c"retransmits:      %u\n".as_ptr(), (*info).mptcpi_retransmits);
    printf(c"retransmit bytes: %llu\n".as_ptr(), (*info).mptcpi_bytes_retrans);
    printf(c"bytes_sent:       %llu\n".as_ptr(), (*info).mptcpi_bytes_sent);
    printf(c"bytes_received:   %llu\n".as_ptr(), (*info).mptcpi_bytes_received);
    printf(c"bytes_acked:      %llu\n".as_ptr(), (*info).mptcpi_bytes_acked);
}

/*
 * 'print_subflow_info' is from 'mptcp_subflow_info'
 * which is a function in 'misc/ss.c' of iproute2.
 */
unsafe fn print_subflow_info(tb: *mut *mut rtattr) {
    let mut flags: u32 = 0;

    printf(c"It's a mptcp subflow, the subflow info:\n".as_ptr());
    if !(*tb.add(MPTCP_SUBFLOW_ATTR_FLAGS)).is_null() {
        let mut caps = [0 as c_char; 32 + 1];
        let mut cap = caps.as_mut_ptr();

        flags = rta_getattr::<__u32>(*tb.add(MPTCP_SUBFLOW_ATTR_FLAGS));

        if flags as c_ulong & MPTCP_SUBFLOW_FLAG_MCAP_REM != 0 {
            *cap = b'M' as c_char;
            cap = cap.add(1);
        }
        if flags as c_ulong & MPTCP_SUBFLOW_FLAG_MCAP_LOC != 0 {
            *cap = b'm' as c_char;
            cap = cap.add(1);
        }
        if flags as c_ulong & MPTCP_SUBFLOW_FLAG_JOIN_REM != 0 {
            *cap = b'J' as c_char;
            cap = cap.add(1);
        }
        if flags as c_ulong & MPTCP_SUBFLOW_FLAG_JOIN_LOC != 0 {
            *cap = b'j' as c_char;
            cap = cap.add(1);
        }
        if flags as c_ulong & MPTCP_SUBFLOW_FLAG_BKUP_REM != 0 {
            *cap = b'B' as c_char;
            cap = cap.add(1);
        }
        if flags as c_ulong & MPTCP_SUBFLOW_FLAG_BKUP_LOC != 0 {
            *cap = b'b' as c_char;
            cap = cap.add(1);
        }
        if flags as c_ulong & MPTCP_SUBFLOW_FLAG_FULLY_ESTABLISHED != 0 {
            *cap = b'e' as c_char;
            cap = cap.add(1);
        }
        if flags as c_ulong & MPTCP_SUBFLOW_FLAG_CONNECTED != 0 {
            *cap = b'c' as c_char;
            cap = cap.add(1);
        }
        if flags as c_ulong & MPTCP_SUBFLOW_FLAG_MAPVALID != 0 {
            *cap = b'v' as c_char;
            cap = cap.add(1);
        }

        if flags != 0 {
            printf(c" flags:%s".as_ptr(), caps.as_ptr());
        }
    }
    if !(*tb.add(MPTCP_SUBFLOW_ATTR_TOKEN_REM)).is_null()
        && !(*tb.add(MPTCP_SUBFLOW_ATTR_TOKEN_LOC)).is_null()
        && !(*tb.add(MPTCP_SUBFLOW_ATTR_ID_REM)).is_null()
        && !(*tb.add(MPTCP_SUBFLOW_ATTR_ID_LOC)).is_null()
    {
        printf(
            c" token:%04x(id:%u)/%04x(id:%u)".as_ptr(),
            rta_getattr::<__u32>(*tb.add(MPTCP_SUBFLOW_ATTR_TOKEN_REM)),
            rta_getattr::<__u8>(*tb.add(MPTCP_SUBFLOW_ATTR_ID_REM)) as c_uint,
            rta_getattr::<__u32>(*tb.add(MPTCP_SUBFLOW_ATTR_TOKEN_LOC)),
            rta_getattr::<__u8>(*tb.add(MPTCP_SUBFLOW_ATTR_ID_LOC)) as c_uint,
        );
    }
    if !(*tb.add(MPTCP_SUBFLOW_ATTR_MAP_SEQ)).is_null() {
        printf(
            c" seq:%llu".as_ptr(),
            rta_getattr::<__u64>(*tb.add(MPTCP_SUBFLOW_ATTR_MAP_SEQ)),
        );
    }
    if !(*tb.add(MPTCP_SUBFLOW_ATTR_MAP_SFSEQ)).is_null() {
        printf(
            c" sfseq:%u".as_ptr(),
            rta_getattr::<__u32>(*tb.add(MPTCP_SUBFLOW_ATTR_MAP_SFSEQ)),
        );
    }
    if !(*tb.add(MPTCP_SUBFLOW_ATTR_SSN_OFFSET)).is_null() {
        printf(
            c" ssnoff:%u".as_ptr(),
            rta_getattr::<__u32>(*tb.add(MPTCP_SUBFLOW_ATTR_SSN_OFFSET)),
        );
    }
    if !(*tb.add(MPTCP_SUBFLOW_ATTR_MAP_DATALEN)).is_null() {
        printf(
            c" maplen:%u".as_ptr(),
            rta_getattr::<__u32>(*tb.add(MPTCP_SUBFLOW_ATTR_MAP_DATALEN)),
        );
    }
    printf(c"\n".as_ptr());
}

unsafe fn parse_nlmsg(nlh: *mut nlmsghdr, proto: __u32) {
    let r = nlmsg_data(nlh) as *mut inet_diag_msg;
    let mut tb: [*mut rtattr; INET_DIAG_MAX + 1] = [ptr::null_mut(); INET_DIAG_MAX + 1];

    parse_rtattr_flags(
        tb.as_mut_ptr(),
        INET_DIAG_MAX as c_int,
        r.add(1) as *mut rtattr,
        (*nlh).nlmsg_len as c_int - nlmsg_length(mem::size_of_val(&*r)) as c_int,
        NLA_F_NESTED,
    );

    if proto == IPPROTO_MPTCP && !tb[INET_DIAG_INFO].is_null() {
        let len = rta_payload(tb[INET_DIAG_INFO]);
        let info: *mut mptcp_info;

        /* workaround fort older kernels with less fields */
        let mut info_buf: mptcp_info = mem::zeroed();
        if len < mem::size_of::<mptcp_info>() as c_int {
            info = &mut info_buf;
            memcpy(
                info as *mut c_void,
                rta_data(tb[INET_DIAG_INFO]),
                len as size_t,
            );
            memset(
                (info as *mut c_char).add(len as usize) as *mut c_void,
                0,
                mem::size_of::<mptcp_info>() - len as usize,
            );
        } else {
            info = rta_data(tb[INET_DIAG_INFO]) as *mut mptcp_info;
        }
        print_info_msg(info);
    }
    if proto == IPPROTO_TCP && !tb[INET_DIAG_ULP_INFO].is_null() {
        let mut ulpinfo: [*mut rtattr; INET_ULP_INFO_MAX + 1] =
            [ptr::null_mut(); INET_ULP_INFO_MAX + 1];

        parse_rtattr_flags(
            ulpinfo.as_mut_ptr(),
            INET_ULP_INFO_MAX as c_int,
            rta_data(tb[INET_DIAG_ULP_INFO]) as *mut rtattr,
            rta_payload(tb[INET_DIAG_ULP_INFO]),
            NLA_F_NESTED,
        );

        if !ulpinfo[INET_ULP_INFO_MPTCP].is_null() {
            let mut sfinfo: [*mut rtattr; MPTCP_SUBFLOW_ATTR_MAX + 1] =
                [ptr::null_mut(); MPTCP_SUBFLOW_ATTR_MAX + 1];

            parse_rtattr_flags(
                sfinfo.as_mut_ptr(),
                MPTCP_SUBFLOW_ATTR_MAX as c_int,
                rta_data(ulpinfo[INET_ULP_INFO_MPTCP]) as *mut rtattr,
                rta_payload(ulpinfo[INET_ULP_INFO_MPTCP]),
                NLA_F_NESTED,
            );
            print_subflow_info(sfinfo.as_mut_ptr());
        } else {
            printf(c"It's a normal TCP!\n".as_ptr());
        }
    }
}

unsafe fn recv_nlmsg(fd: c_int, proto: __u32) {
    let mut rcv_buff = [0 as c_char; 8192];
    let mut nlh = rcv_buff.as_mut_ptr() as *mut nlmsghdr;
    let mut rcv_nladdr = sockaddr_nl {
        nl_family: AF_NETLINK as __u16,
        nl_pad: 0,
        nl_pid: 0,
        nl_groups: 0,
    };
    let mut rcv_iov = iovec {
        iov_base: rcv_buff.as_mut_ptr() as *mut c_void,
        iov_len: mem::size_of_val(&rcv_buff),
    };
    let mut rcv_msg = msghdr {
        msg_name: &mut rcv_nladdr as *mut _ as *mut c_void,
        msg_namelen: mem::size_of_val(&rcv_nladdr) as socklen_t,
        msg_iov: &mut rcv_iov,
        msg_iovlen: 1,
        msg_control: ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut len: c_int;

    len = recvmsg(fd, &mut rcv_msg, 0) as c_int;

    while nlmsg_ok(nlh, len) {
        if (*nlh).nlmsg_type == NLMSG_DONE {
            printf(c"NLMSG_DONE\n".as_ptr());
            break;
        } else if (*nlh).nlmsg_type == NLMSG_ERROR {
            let mut err: *mut nlmsgerr;

            err = nlmsg_data(nlh) as *mut nlmsgerr;
            printf(
                c"Error %d:%s\n".as_ptr(),
                -(*err).error,
                strerror(-(*err).error),
            );
            break;
        }
        parse_nlmsg(nlh, proto);
        nlh = nlmsg_next(nlh, &mut len);
    }
}

unsafe fn get_mptcpinfo(token: __u32) {
    let mut r = inet_diag_req_v2 {
        sdiag_family: AF_INET as __u8,
        /* Real proto is set via INET_DIAG_REQ_PROTOCOL */
        sdiag_protocol: IPPROTO_TCP as __u8,
        idiag_ext: 1 << (INET_DIAG_INFO - 1),
        pad: 0,
        idiag_states: 0,
        id: inet_diag_sockid {
            idiag_sport: 0,
            idiag_dport: 0,
            idiag_src: [0; 4],
            idiag_dst: [0; 4],
            idiag_if: 0,
            idiag_cookie: [token, 0],
        },
    };
    let proto: __u32 = IPPROTO_MPTCP;
    let fd: c_int;

    fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_SOCK_DIAG);
    if fd < 0 {
        die_perror(c"Netlink socket".as_ptr());
    }

    send_query(fd, &mut r, proto);
    recv_nlmsg(fd, proto);

    close(fd);
}

unsafe fn get_subflow_info(subflow_addrs: *mut c_char) {
    let mut r = inet_diag_req_v2 {
        sdiag_family: AF_INET as __u8,
        sdiag_protocol: IPPROTO_TCP as __u8,
        idiag_ext: 1 << (INET_DIAG_INFO - 1),
        pad: 0,
        idiag_states: 0,
        id: inet_diag_sockid {
            idiag_sport: 0,
            idiag_dport: 0,
            idiag_src: [0; 4],
            idiag_dst: [0; 4],
            idiag_if: 0,
            idiag_cookie: [INET_DIAG_NOCOOKIE, INET_DIAG_NOCOOKIE],
        },
    };
    let mut saddr = [0 as c_char; 64];
    let mut daddr = [0 as c_char; 64];
    let mut sport: c_int = 0;
    let mut dport: c_int = 0;
    let ret: c_int;
    let fd: c_int;

    ret = sscanf(
        subflow_addrs,
        c"%63[^:]:%d %63[^:]:%d".as_ptr(),
        saddr.as_mut_ptr(),
        &mut sport,
        daddr.as_mut_ptr(),
        &mut dport,
    );
    if ret != 4 {
        die_perror(c"IP PORT Pairs has style problems!".as_ptr());
    }

    printf(
        c"%s:%d -> %s:%d\n".as_ptr(),
        saddr.as_ptr(),
        sport,
        daddr.as_ptr(),
        dport,
    );

    fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_SOCK_DIAG);
    if fd < 0 {
        die_perror(c"Netlink socket".as_ptr());
    }

    r.id.idiag_sport = htons(sport as __u16);
    r.id.idiag_dport = htons(dport as __u16);

    inet_pton(
        AF_INET,
        saddr.as_ptr(),
        r.id.idiag_src.as_mut_ptr() as *mut c_void,
    );
    inet_pton(
        AF_INET,
        daddr.as_ptr(),
        r.id.idiag_dst.as_mut_ptr() as *mut c_void,
    );
    send_query(fd, &mut r, IPPROTO_TCP);
    recv_nlmsg(fd, IPPROTO_TCP);
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char, p: *mut params) {
    let mut c: c_int;

    if argc < 2 {
        die_usage(1);
    }

    loop {
        c = getopt(argc, argv, c"ht:s:".as_ptr());
        if c == -1 {
            break;
        }
        match c {
            x if x == b'h' as c_int => {
                die_usage(0);
            }
            x if x == b't' as c_int => {
                sscanf(optarg, c"%x".as_ptr(), &mut (*p).target_token);
            }
            x if x == b's' as c_int => {
                strncpy(
                    (*p).subflow_addrs.as_mut_ptr(),
                    optarg,
                    mem::size_of_val(&(*p).subflow_addrs) - 1,
                );
            }
            _ => {
                die_usage(1);
            }
        }
    }
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut p: params = mem::zeroed();

    parse_opts(argc, argv, &mut p);

    if p.target_token != 0 {
        get_mptcpinfo(p.target_token);
    }

    if p.subflow_addrs[0] != 0 {
        get_subflow_info(p.subflow_addrs.as_mut_ptr());
    }

    0
}

fn main() {
    unsafe {
        let mut args: Vec<*mut c_char> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
            .collect();
        args.push(ptr::null_mut());
        std::process::exit(c_main((args.len() - 1) as c_int, args.as_mut_ptr()));
    }
}
