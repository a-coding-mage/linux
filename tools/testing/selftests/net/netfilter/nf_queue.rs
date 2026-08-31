// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use std::ffi::{c_char, c_int, c_long, c_uint, c_void};
use std::mem::size_of;
use std::ptr;

type uint8_t = u8;
type uint32_t = u32;

// Constants from the C headers included by nf_queue.c.
const AF_UNSPEC: c_int = 0;
const AF_INET: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_RCVTIMEO: c_int = 20;
const NETLINK_NETFILTER: c_int = 12;
const EXIT_FAILURE: c_int = 1;

const MNL_CB_OK: c_int = 1;
const MNL_CB_ERROR: c_int = -1;
const MNL_SOCKET_AUTOPID: c_uint = 0;
const MNL_SOCKET_BUFFER_SIZE: c_uint = 8192;
const MNL_TYPE_UNSPEC: c_int = 0;
const MNL_TYPE_U32: c_int = 4;

const NLM_F_REQUEST: u16 = 1;
const NF_ACCEPT: uint32_t = 1;
const NF_QUEUE: uint32_t = 3;
const NFNETLINK_V0: u8 = 0;
const NFNL_SUBSYS_QUEUE: u16 = 3;
const NFQNL_MSG_CONFIG: u16 = 1;
const NFQNL_MSG_VERDICT: u16 = 2;
const NFQNL_CFG_CMD_BIND: uint8_t = 1;
const NFQNL_COPY_PACKET: uint8_t = 2;

const NFQA_UNSPEC: usize = 0;
const NFQA_PACKET_HDR: usize = 1;
const NFQA_VERDICT_HDR: usize = 2;
const NFQA_MARK: usize = 3;
const NFQA_TIMESTAMP: usize = 4;
const NFQA_IFINDEX_INDEV: usize = 5;
const NFQA_IFINDEX_OUTDEV: usize = 6;
const NFQA_IFINDEX_PHYSINDEV: usize = 7;
const NFQA_IFINDEX_PHYSOUTDEV: usize = 8;
const NFQA_HWADDR: usize = 9;
const NFQA_PAYLOAD: usize = 10;
const NFQA_SKB_INFO: usize = 11;
const NFQA_MAX: usize = 12;

const NFQA_CFG_CMD: u16 = 1;
const NFQA_CFG_PARAMS: u16 = 2;
const NFQA_CFG_FLAGS: u16 = 4;
const NFQA_CFG_MASK: u16 = 5;

const NFQA_CFG_F_FAIL_OPEN: uint32_t = 0x01;
const NFQA_CFG_F_GSO: uint32_t = 0x02;
const NFQA_CFG_F_UID_GID: uint32_t = 0x08;

const NFQA_SKB_CSUMNOTREADY: uint32_t = 1 << 0;
const NFQA_SKB_GSO: uint32_t = 1 << 1;
const NFQA_SKB_CSUM_NOTVERIFIED: uint32_t = 1 << 2;

const ENOENT: c_int = 2;
const EINTR: c_int = 4;
const EAGAIN: c_int = 11;
const ENOBUFS: c_int = 105;

#[repr(C)]
struct options {
    count_packets: bool,
    gso_enabled: bool,
    failopen: bool,
    out_of_order: bool,
    bogus_verdict: bool,
    verbose: c_int,
    queue_num: c_uint,
    timeout: c_uint,
    verdict: uint32_t,
    delay_ms: uint32_t,
}

#[repr(C)]
struct nlattr {
    nla_len: u16,
    nla_type: u16,
}

#[repr(C)]
struct nlmsghdr {
    nlmsg_len: u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: u32,
    nlmsg_pid: u32,
}

#[repr(C)]
struct nfgenmsg {
    nfgen_family: u8,
    version: u8,
    res_id: u16,
}

#[repr(C)]
struct nfqnl_msg_config_cmd {
    command: uint8_t,
    pf: u16,
}

#[repr(C)]
struct nfqnl_msg_config_params {
    copy_range: u32,
    copy_mode: uint8_t,
}

#[repr(C)]
struct nfqnl_msg_verdict_hdr {
    verdict: u32,
    id: u32,
}

#[repr(C)]
struct nfqnl_msg_packet_hdr {
    packet_id: u32,
    hw_protocol: u16,
    hook: u8,
}

#[repr(C)]
struct nfqnl_msg_packet_timestamp {
    sec: u64,
    usec: u64,
}

#[repr(C)]
struct nfqnl_msg_packet_hw {
    hw_addrlen: u16,
    _pad: u16,
    hw_addr: [u8; 8],
}

#[repr(C)]
struct mnl_socket {
    _private: [u8; 0],
}

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

type mnl_attr_cb_t = Option<unsafe extern "C" fn(*const nlattr, *mut c_void) -> c_int>;
type mnl_cb_t = Option<unsafe extern "C" fn(*const nlmsghdr, *mut c_void) -> c_int>;

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut stderr: *mut c_void;

    fn __errno_location() -> *mut c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn htonl(hostlong: u32) -> u32;
    fn htons(hostshort: u16) -> u16;
    fn malloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int;
    fn ntohl(netlong: u32) -> u32;
    fn ntohs(netshort: u16) -> u16;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: u32,
    ) -> c_int;

    fn mnl_attr_get_payload(attr: *const nlattr) -> *mut c_void;
    fn mnl_attr_get_type(attr: *const nlattr) -> c_int;
    fn mnl_attr_get_u32(attr: *const nlattr) -> u32;
    fn mnl_attr_parse(
        nlh: *const nlmsghdr,
        offset: usize,
        cb: mnl_attr_cb_t,
        data: *mut c_void,
    ) -> c_int;
    fn mnl_attr_put(nlh: *mut nlmsghdr, attr_type: u16, len: usize, data: *const c_void);
    fn mnl_attr_put_u32(nlh: *mut nlmsghdr, attr_type: u16, data: u32);
    fn mnl_attr_type_valid(attr: *const nlattr, max: u16) -> c_int;
    fn mnl_attr_validate(attr: *const nlattr, attr_type: c_int) -> c_int;
    fn mnl_attr_validate2(attr: *const nlattr, attr_type: c_int, len: usize) -> c_int;
    fn mnl_cb_run(
        buf: *const c_void,
        numbytes: c_int,
        seq: c_uint,
        portid: c_uint,
        cb_data: mnl_cb_t,
        data: *mut c_void,
    ) -> c_int;
    fn mnl_nlmsg_put_extra_header(nlh: *mut nlmsghdr, size: usize) -> *mut c_void;
    fn mnl_nlmsg_put_header(buf: *mut c_void) -> *mut nlmsghdr;
    fn mnl_socket_bind(nl: *mut mnl_socket, groups: c_uint, pid: c_uint) -> c_int;
    fn mnl_socket_close(nl: *mut mnl_socket) -> c_int;
    fn mnl_socket_get_fd(nl: *const mnl_socket) -> c_int;
    fn mnl_socket_get_portid(nl: *const mnl_socket) -> c_uint;
    fn mnl_socket_open(bus: c_int) -> *mut mnl_socket;
    fn mnl_socket_recvfrom(nl: *const mnl_socket, buf: *mut c_void, bufsiz: usize) -> c_int;
    fn mnl_socket_sendto(nl: *const mnl_socket, buf: *const c_void, len: usize) -> c_int;
}

static mut queue_stats: [c_uint; 5] = [0; 5];
static mut opts: options = options {
    count_packets: false,
    gso_enabled: false,
    failopen: false,
    out_of_order: false,
    bogus_verdict: false,
    verbose: 0,
    queue_num: 0,
    timeout: 0,
    verdict: 0,
    delay_ms: 0,
};

unsafe fn errno() -> *mut c_int {
    __errno_location()
}

unsafe fn help(p: *const c_char) {
    printf(
        b"Usage: %s [-c|-v [-vv] ] [-o] [-O] [-b] [-t timeout] [-q queue_num] [-Qdst_queue ] [ -d ms_delay ] [-G]\n\0"
            .as_ptr() as *const c_char,
        p,
    );
}

unsafe extern "C" fn parse_attr_cb(attr: *const nlattr, data: *mut c_void) -> c_int {
    let tb = data as *mut *const nlattr;
    let type_ = mnl_attr_get_type(attr);

    /* skip unsupported attribute in user-space */
    if mnl_attr_type_valid(attr, NFQA_MAX as u16) < 0 {
        return MNL_CB_OK;
    }

    match type_ as usize {
        NFQA_MARK
        | NFQA_IFINDEX_INDEV
        | NFQA_IFINDEX_OUTDEV
        | NFQA_IFINDEX_PHYSINDEV
        | NFQA_IFINDEX_PHYSOUTDEV => {
            if mnl_attr_validate(attr, MNL_TYPE_U32) < 0 {
                perror(b"mnl_attr_validate\0".as_ptr() as *const c_char);
                return MNL_CB_ERROR;
            }
        }
        NFQA_TIMESTAMP => {
            if mnl_attr_validate2(
                attr,
                MNL_TYPE_UNSPEC,
                size_of::<nfqnl_msg_packet_timestamp>(),
            ) < 0
            {
                perror(b"mnl_attr_validate2\0".as_ptr() as *const c_char);
                return MNL_CB_ERROR;
            }
        }
        NFQA_HWADDR => {
            if mnl_attr_validate2(attr, MNL_TYPE_UNSPEC, size_of::<nfqnl_msg_packet_hw>()) < 0 {
                perror(b"mnl_attr_validate2\0".as_ptr() as *const c_char);
                return MNL_CB_ERROR;
            }
        }
        NFQA_PAYLOAD => {}
        _ => {}
    }
    *tb.add(type_ as usize) = attr;
    MNL_CB_OK
}

unsafe extern "C" fn queue_cb(nlh: *const nlmsghdr, data: *mut c_void) -> c_int {
    let mut tb: [*const nlattr; NFQA_MAX + 1] = [ptr::null(); NFQA_MAX + 1];
    let mut ph: *mut nfqnl_msg_packet_hdr = ptr::null_mut();
    let mut id: uint32_t = 0;

    let _ = data;

    mnl_attr_parse(
        nlh,
        size_of::<nfgenmsg>(),
        Some(parse_attr_cb),
        tb.as_mut_ptr() as *mut c_void,
    );
    if !tb[NFQA_PACKET_HDR].is_null() {
        ph = mnl_attr_get_payload(tb[NFQA_PACKET_HDR]) as *mut nfqnl_msg_packet_hdr;
        id = ntohl((*ph).packet_id);

        if opts.verbose > 0 {
            printf(
                b"packet hook=%u, hwproto 0x%x\0".as_ptr() as *const c_char,
                ntohs((*ph).hw_protocol) as c_uint,
                (*ph).hook as c_uint,
            );
        }

        if (*ph).hook >= 5 {
            fprintf(
                stderr,
                b"Unknown hook %d\n\0".as_ptr() as *const c_char,
                (*ph).hook as c_int,
            );
            return MNL_CB_ERROR;
        }

        if opts.verbose > 0 {
            let mut skbinfo: uint32_t = 0;

            if !tb[NFQA_SKB_INFO].is_null() {
                skbinfo = ntohl(mnl_attr_get_u32(tb[NFQA_SKB_INFO]));
            }
            if skbinfo & NFQA_SKB_CSUMNOTREADY != 0 {
                printf(b" csumnotready\0".as_ptr() as *const c_char);
            }
            if skbinfo & NFQA_SKB_GSO != 0 {
                printf(b" gso\0".as_ptr() as *const c_char);
            }
            if skbinfo & NFQA_SKB_CSUM_NOTVERIFIED != 0 {
                printf(b" csumnotverified\0".as_ptr() as *const c_char);
            }
            puts(b"\0".as_ptr() as *const c_char);
        }

        if opts.count_packets {
            queue_stats[(*ph).hook as usize] = queue_stats[(*ph).hook as usize].wrapping_add(1);
        }
    }

    MNL_CB_OK + id as c_int
}

unsafe fn nfq_build_cfg_request(
    buf: *mut c_char,
    command: uint8_t,
    queue_num: c_int,
) -> *mut nlmsghdr {
    let nlh = mnl_nlmsg_put_header(buf as *mut c_void);
    let cmd = nfqnl_msg_config_cmd {
        command,
        pf: htons(AF_INET as u16),
    };
    let nfg: *mut nfgenmsg;

    (*nlh).nlmsg_type = (NFNL_SUBSYS_QUEUE << 8) | NFQNL_MSG_CONFIG;
    (*nlh).nlmsg_flags = NLM_F_REQUEST;

    nfg = mnl_nlmsg_put_extra_header(nlh, size_of::<nfgenmsg>()) as *mut nfgenmsg;

    (*nfg).nfgen_family = AF_UNSPEC as u8;
    (*nfg).version = NFNETLINK_V0;
    (*nfg).res_id = htons(queue_num as u16);

    mnl_attr_put(
        nlh,
        NFQA_CFG_CMD,
        size_of::<nfqnl_msg_config_cmd>(),
        &cmd as *const _ as *const c_void,
    );

    nlh
}

unsafe fn nfq_build_cfg_params(
    buf: *mut c_char,
    mode: uint8_t,
    range: c_int,
    queue_num: c_int,
) -> *mut nlmsghdr {
    let nlh = mnl_nlmsg_put_header(buf as *mut c_void);
    let params = nfqnl_msg_config_params {
        copy_range: htonl(range as u32),
        copy_mode: mode,
    };
    let nfg: *mut nfgenmsg;

    (*nlh).nlmsg_type = (NFNL_SUBSYS_QUEUE << 8) | NFQNL_MSG_CONFIG;
    (*nlh).nlmsg_flags = NLM_F_REQUEST;

    nfg = mnl_nlmsg_put_extra_header(nlh, size_of::<nfgenmsg>()) as *mut nfgenmsg;
    (*nfg).nfgen_family = AF_UNSPEC as u8;
    (*nfg).version = NFNETLINK_V0;
    (*nfg).res_id = htons(queue_num as u16);

    mnl_attr_put(
        nlh,
        NFQA_CFG_PARAMS,
        size_of::<nfqnl_msg_config_params>(),
        &params as *const _ as *const c_void,
    );

    nlh
}

unsafe fn nfq_build_verdict(
    buf: *mut c_char,
    id: c_int,
    queue_num: c_int,
    verd: uint32_t,
) -> *mut nlmsghdr {
    let vh = nfqnl_msg_verdict_hdr {
        verdict: htonl(verd),
        id: htonl(id as u32),
    };
    let nlh: *mut nlmsghdr;
    let nfg: *mut nfgenmsg;

    nlh = mnl_nlmsg_put_header(buf as *mut c_void);
    (*nlh).nlmsg_type = (NFNL_SUBSYS_QUEUE << 8) | NFQNL_MSG_VERDICT;
    (*nlh).nlmsg_flags = NLM_F_REQUEST;
    nfg = mnl_nlmsg_put_extra_header(nlh, size_of::<nfgenmsg>()) as *mut nfgenmsg;
    (*nfg).nfgen_family = AF_UNSPEC as u8;
    (*nfg).version = NFNETLINK_V0;
    (*nfg).res_id = htons(queue_num as u16);

    mnl_attr_put(
        nlh,
        NFQA_VERDICT_HDR,
        size_of::<nfqnl_msg_verdict_hdr>(),
        &vh as *const _ as *const c_void,
    );

    nlh
}

unsafe fn print_stats() {
    let mut last: c_uint;
    let mut total: c_uint;
    let mut i: c_int;

    total = 0;
    last = queue_stats[0];

    i = 0;
    while i < 5 {
        printf(
            b"hook %d packets %08u\n\0".as_ptr() as *const c_char,
            i,
            queue_stats[i as usize],
        );
        last = queue_stats[i as usize];
        total = total.wrapping_add(last);
        i += 1;
    }

    printf(
        b"%u packets total\n\0".as_ptr() as *const c_char,
        total,
    );
}

unsafe fn open_queue() -> *mut mnl_socket {
    let mut buf: [c_char; MNL_SOCKET_BUFFER_SIZE as usize] = [0; MNL_SOCKET_BUFFER_SIZE as usize];
    let queue_num: c_uint;
    let nl: *mut mnl_socket;
    let mut nlh: *mut nlmsghdr;
    let mut tv: timeval = std::mem::zeroed();
    let mut flags: uint32_t;

    nl = mnl_socket_open(NETLINK_NETFILTER);
    if nl.is_null() {
        perror(b"mnl_socket_open\0".as_ptr() as *const c_char);
        exit(EXIT_FAILURE);
    }

    if mnl_socket_bind(nl, 0, MNL_SOCKET_AUTOPID) < 0 {
        perror(b"mnl_socket_bind\0".as_ptr() as *const c_char);
        exit(EXIT_FAILURE);
    }

    queue_num = opts.queue_num;
    nlh = nfq_build_cfg_request(buf.as_mut_ptr(), NFQNL_CFG_CMD_BIND, queue_num as c_int);

    if mnl_socket_sendto(nl, nlh as *const c_void, (*nlh).nlmsg_len as usize) < 0 {
        perror(b"mnl_socket_sendto\0".as_ptr() as *const c_char);
        exit(EXIT_FAILURE);
    }

    nlh = nfq_build_cfg_params(buf.as_mut_ptr(), NFQNL_COPY_PACKET, 0xFFFF, queue_num as c_int);

    flags = if opts.gso_enabled { NFQA_CFG_F_GSO } else { 0 };
    flags |= NFQA_CFG_F_UID_GID;
    if opts.failopen {
        flags |= NFQA_CFG_F_FAIL_OPEN;
    }
    mnl_attr_put_u32(nlh, NFQA_CFG_FLAGS, htonl(flags));
    mnl_attr_put_u32(nlh, NFQA_CFG_MASK, htonl(flags));

    if mnl_socket_sendto(nl, nlh as *const c_void, (*nlh).nlmsg_len as usize) < 0 {
        perror(b"mnl_socket_sendto\0".as_ptr() as *const c_char);
        exit(EXIT_FAILURE);
    }

    memset(
        &mut tv as *mut _ as *mut c_void,
        0,
        size_of::<timeval>(),
    );
    tv.tv_sec = opts.timeout as c_long;
    if opts.timeout != 0
        && setsockopt(
            mnl_socket_get_fd(nl),
            SOL_SOCKET,
            SO_RCVTIMEO,
            &tv as *const _ as *const c_void,
            size_of::<timeval>() as u32,
        ) != 0
    {
        perror(b"setsockopt(SO_RCVTIMEO)\0".as_ptr() as *const c_char);
        exit(EXIT_FAILURE);
    }

    nl
}

unsafe fn sleep_ms(mut delay: uint32_t) {
    let mut ts = timespec {
        tv_sec: (delay / 1000) as c_long,
        tv_nsec: 0,
    };

    delay %= 1000;

    ts.tv_nsec = (delay as u64 * 1000u64 * 1000u64) as c_long;

    nanosleep(&ts, ptr::null_mut());
}

unsafe fn mainloop() -> c_int {
    let buflen: c_uint = 64 * 1024 + MNL_SOCKET_BUFFER_SIZE;
    let nl: *mut mnl_socket;
    let mut nlh: *mut nlmsghdr;
    let mut ooo_ids: [uint32_t; 16] = [0; 16];
    let portid: c_uint;
    let mut ooo_count: c_int = 0;
    let buf: *mut c_char;
    let mut ret: c_int;

    buf = malloc(buflen as usize) as *mut c_char;
    if buf.is_null() {
        perror(b"malloc\0".as_ptr() as *const c_char);
        exit(EXIT_FAILURE);
    }

    nl = open_queue();
    portid = mnl_socket_get_portid(nl);

    loop {
        let id: uint32_t;

        ret = mnl_socket_recvfrom(nl, buf as *mut c_void, buflen as usize);
        if ret == -1 {
            if *errno() == ENOBUFS || *errno() == EINTR {
                continue;
            }

            if *errno() == EAGAIN {
                *errno() = 0;
                ret = 0;
                break;
            }

            perror(b"mnl_socket_recvfrom\0".as_ptr() as *const c_char);
            exit(EXIT_FAILURE);
        }

        ret = mnl_cb_run(
            buf as *const c_void,
            ret,
            0,
            portid,
            Some(queue_cb),
            ptr::null_mut(),
        );
        if ret < 0 {
            /* bogus verdict mode will generate ENOENT error messages */
            if opts.bogus_verdict && *errno() == ENOENT {
                continue;
            }
            perror(b"mnl_cb_run\0".as_ptr() as *const c_char);
            exit(EXIT_FAILURE);
        }

        id = (ret - MNL_CB_OK) as uint32_t;
        if opts.delay_ms != 0 {
            sleep_ms(opts.delay_ms);
        }

        if opts.bogus_verdict {
            let mut i: c_int = 0;
            while i < 50 {
                nlh = nfq_build_verdict(
                    buf,
                    id.wrapping_add(0x7FFFFFFF).wrapping_add(i as u32) as c_int,
                    opts.queue_num as c_int,
                    opts.verdict,
                );
                mnl_socket_sendto(nl, nlh as *const c_void, (*nlh).nlmsg_len as usize);
                i += 1;
            }
        }

        if opts.out_of_order {
            ooo_ids[ooo_count as usize] = id;
            if ooo_count >= 15 {
                while ooo_count >= 0 {
                    nlh = nfq_build_verdict(
                        buf,
                        ooo_ids[ooo_count as usize] as c_int,
                        opts.queue_num as c_int,
                        opts.verdict,
                    );
                    if mnl_socket_sendto(nl, nlh as *const c_void, (*nlh).nlmsg_len as usize) < 0 {
                        perror(b"mnl_socket_sendto\0".as_ptr() as *const c_char);
                        exit(EXIT_FAILURE);
                    }
                    ooo_count -= 1;
                }
                ooo_count = 0;
            } else {
                ooo_count += 1;
            }
        } else {
            nlh = nfq_build_verdict(buf, id as c_int, opts.queue_num as c_int, opts.verdict);
            if mnl_socket_sendto(nl, nlh as *const c_void, (*nlh).nlmsg_len as usize) < 0 {
                perror(b"mnl_socket_sendto\0".as_ptr() as *const c_char);
                exit(EXIT_FAILURE);
            }
        }
    }

    mnl_socket_close(nl);

    ret
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut c: c_int;

    loop {
        c = getopt(
            argc,
            argv as *const *mut c_char,
            b"chvoObt:q:Q:d:G\0".as_ptr() as *const c_char,
        );
        if c == -1 {
            break;
        }
        match c as u8 as char {
            'c' => {
                opts.count_packets = true;
            }
            'h' => {
                help(*argv.add(0));
                exit(0);
            }
            'q' => {
                opts.queue_num = atoi(optarg) as c_uint;
                if opts.queue_num > 0xffff {
                    opts.queue_num = 0;
                }
            }
            'Q' => {
                opts.verdict = atoi(optarg) as uint32_t;
                if opts.verdict > 0xffff {
                    fprintf(
                        stderr,
                        b"Expected destination queue number\n\0".as_ptr() as *const c_char,
                    );
                    exit(1);
                }

                opts.verdict <<= 16;
                opts.verdict |= NF_QUEUE;
            }
            'd' => {
                opts.delay_ms = atoi(optarg) as uint32_t;
                if opts.delay_ms == 0 {
                    fprintf(
                        stderr,
                        b"Expected nonzero delay (in milliseconds)\n\0".as_ptr()
                            as *const c_char,
                    );
                    exit(1);
                }
            }
            't' => {
                opts.timeout = atoi(optarg) as c_uint;
            }
            'G' => {
                opts.gso_enabled = false;
            }
            'o' => {
                opts.failopen = true;
            }
            'v' => {
                opts.verbose += 1;
            }
            'O' => {
                opts.out_of_order = true;
            }
            'b' => {
                opts.bogus_verdict = true;
            }
            _ => {}
        }
    }

    if opts.verdict != NF_ACCEPT && (opts.verdict >> 16 == opts.queue_num) {
        fprintf(
            stderr,
            b"Cannot use same destination and source queue\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }
}

fn main() {
    unsafe {
        let mut ret: c_int;
        let args: Vec<*mut c_char> = std::env::args()
            .map(|arg| {
                std::ffi::CString::new(arg)
                    .unwrap()
                    .into_raw()
            })
            .collect();

        opts.verdict = NF_ACCEPT;
        opts.gso_enabled = true;

        ret = mainloop_with_args(args.len() as c_int, args.as_ptr() as *mut *mut c_char);
        std::process::exit(ret);
    }
}

unsafe fn mainloop_with_args(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let ret: c_int;

    parse_opts(argc, argv);

    ret = mainloop();
    if opts.count_packets {
        print_stats();
    }

    ret
}
