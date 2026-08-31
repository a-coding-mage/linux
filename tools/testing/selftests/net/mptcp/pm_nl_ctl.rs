// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{copy_nonoverlapping, null, null_mut};

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type u_int8_t = u8;
type u_int16_t = u16;
type u_int32_t = u32;
type int32_t = i32;
type pid_t = i32;
type socklen_t = u32;

const IPPROTO_MPTCP: c_int = 262;
const MPTCP_PM_ADDR_FLAG_UNKNOWN: u32 = 1 << 7;

const AF_UNSPEC: c_int = 0;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const AF_NETLINK: c_int = 16;
const SOCK_STREAM: c_int = 1;
const SOCK_RAW: c_int = 3;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const SOL_NETLINK: c_int = 270;
const NETLINK_GENERIC: c_int = 16;
const NETLINK_ADD_MEMBERSHIP: c_int = 1;
const FD_SETSIZE: c_int = 1024;
const INET6_ADDRSTRLEN: usize = 46;
const IF_NAMESIZE: usize = 16;

const NLM_F_REQUEST: u16 = 0x01;
const NLM_F_ACK: u16 = 0x04;
const NLM_F_DUMP: u16 = 0x300;
const NLMSG_ERROR: u16 = 0x2;
const NLMSG_DONE: u16 = 0x3;
const GENL_ID_CTRL: u16 = 0x10;
const CTRL_CMD_NEWFAMILY: u8 = 1;
const CTRL_CMD_GETFAMILY: u8 = 3;
const CTRL_ATTR_FAMILY_ID: u16 = 1;
const CTRL_ATTR_FAMILY_NAME: u16 = 2;
const CTRL_ATTR_MCAST_GROUPS: u16 = 7;
const CTRL_ATTR_MCAST_GRP_NAME: u16 = 1;
const CTRL_ATTR_MCAST_GRP_ID: u16 = 2;
const NLA_F_NESTED: u16 = 1 << 15;
const GENL_HDRLEN: usize = 4;

// Constants normally supplied by linux/mptcp.h.
unsafe extern "C" {
    static MPTCP_PM_NAME: c_char;
    static MPTCP_PM_EV_GRP_NAME: c_char;
}

extern "C" {
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn error(status: c_int, errnum: c_int, fmt: *const c_char, ...) -> !;
    fn exit(status: c_int) -> !;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strtok(s: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn atoi(nptr: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn recv(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *mut c_void) -> c_int;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char, size: socklen_t) -> *const c_char;
    fn ntohl(netlong: u32) -> u32;
    fn ntohs(netshort: u16) -> u16;
    fn htons(hostshort: u16) -> u16;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;
    fn getpid() -> pid_t;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn pause() -> c_int;
    fn close(fd: c_int) -> c_int;
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut errno: c_int;
}

enum FILE {}

#[repr(C)]
struct nlmsghdr {
    nlmsg_len: u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: u32,
    nlmsg_pid: u32,
}

#[repr(C)]
struct genlmsghdr {
    cmd: u8,
    version: u8,
    reserved: u16,
}

#[repr(C)]
struct nlmsgerr {
    error: c_int,
    msg: nlmsghdr,
}

#[repr(C)]
struct rtattr {
    rta_len: u16,
    rta_type: u16,
}

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
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
struct sockaddr_storage {
    ss_family: u16,
    __ss_padding: [u8; 118],
    __ss_align: u64,
}

#[repr(C)]
struct fd_set {
    fds_bits: [isize; 16],
}

const MPTCP_PM_VER: c_int = 1;
const MPTCP_PM_CMD_ADD_ADDR: c_int = 1;
const MPTCP_PM_CMD_DEL_ADDR: c_int = 2;
const MPTCP_PM_CMD_GET_ADDR: c_int = 3;
const MPTCP_PM_CMD_FLUSH_ADDRS: c_int = 4;
const MPTCP_PM_CMD_SET_LIMITS: c_int = 5;
const MPTCP_PM_CMD_GET_LIMITS: c_int = 6;
const MPTCP_PM_CMD_SET_FLAGS: c_int = 7;
const MPTCP_PM_CMD_ANNOUNCE: c_int = 8;
const MPTCP_PM_CMD_REMOVE: c_int = 9;
const MPTCP_PM_CMD_SUBFLOW_CREATE: c_int = 10;
const MPTCP_PM_CMD_SUBFLOW_DESTROY: c_int = 11;

const MPTCP_PM_ATTR_ADDR: u16 = 1;
const MPTCP_PM_ATTR_RCV_ADD_ADDRS: u16 = 2;
const MPTCP_PM_ATTR_SUBFLOWS: u16 = 3;
const MPTCP_PM_ATTR_TOKEN: u16 = 4;
const MPTCP_PM_ATTR_LOC_ID: u16 = 5;
const MPTCP_PM_ATTR_ADDR_REMOTE: u16 = 6;

const MPTCP_PM_ADDR_ATTR_FAMILY: u16 = 1;
const MPTCP_PM_ADDR_ATTR_ID: u16 = 2;
const MPTCP_PM_ADDR_ATTR_ADDR4: u16 = 3;
const MPTCP_PM_ADDR_ATTR_ADDR6: u16 = 4;
const MPTCP_PM_ADDR_ATTR_PORT: u16 = 5;
const MPTCP_PM_ADDR_ATTR_FLAGS: u16 = 6;
const MPTCP_PM_ADDR_ATTR_IF_IDX: u16 = 7;

const MPTCP_PM_ADDR_FLAG_SIGNAL: u32 = 1;
const MPTCP_PM_ADDR_FLAG_SUBFLOW: u32 = 2;
const MPTCP_PM_ADDR_FLAG_BACKUP: u32 = 4;
const MPTCP_PM_ADDR_FLAG_FULLMESH: u32 = 8;
const MPTCP_PM_ADDR_FLAG_IMPLICIT: u32 = 16;
const MPTCP_PM_ADDR_FLAG_LAMINAR: u32 = 32;

const MPTCP_ATTR_TOKEN: u16 = 1;
const MPTCP_ATTR_FAMILY: u16 = 2;
const MPTCP_ATTR_LOC_ID: u16 = 3;
const MPTCP_ATTR_REM_ID: u16 = 4;
const MPTCP_ATTR_SADDR4: u16 = 5;
const MPTCP_ATTR_SADDR6: u16 = 6;
const MPTCP_ATTR_DADDR4: u16 = 7;
const MPTCP_ATTR_DADDR6: u16 = 8;
const MPTCP_ATTR_SPORT: u16 = 9;
const MPTCP_ATTR_DPORT: u16 = 10;
const MPTCP_ATTR_BACKUP: u16 = 11;
const MPTCP_ATTR_ERROR: u16 = 12;
const MPTCP_ATTR_FLAGS: u16 = 13;
const MPTCP_ATTR_SERVER_SIDE: u16 = 14;
const MPTCP_PM_EV_FLAG_DENY_JOIN_ID0: u16 = 1;
const MPTCP_PM_EV_FLAG_SERVER_SIDE: u16 = 2;
const NLMSGERR_ATTR_MSG: u16 = 1;
const NLMSGERR_ATTR_OFFS: u16 = 2;

const fn nlmsg_align(len: usize) -> usize { (len + 3) & !3 }
const fn rta_align(len: usize) -> usize { (len + 3) & !3 }
const fn nlmsg_length(len: usize) -> usize { nlmsg_align(size_of::<nlmsghdr>()) + len }
const fn rta_length(len: usize) -> u16 { (rta_align(size_of::<rtattr>()) + len) as u16 }

unsafe fn nlmsg_data(nlh: *mut nlmsghdr) -> *mut c_void {
    (nlh as *mut u8).add(nlmsg_length(0)) as *mut c_void
}
unsafe fn rta_data(rta: *mut rtattr) -> *mut c_void {
    (rta as *mut u8).add(rta_length(0) as usize) as *mut c_void
}
unsafe fn rta_payload(rta: *mut rtattr) -> c_int {
    (*rta).rta_len as c_int - rta_length(0) as c_int
}
unsafe fn nlmsg_ok(nlh: *mut nlmsghdr, len: c_int) -> bool {
    len >= size_of::<nlmsghdr>() as c_int && (*nlh).nlmsg_len >= size_of::<nlmsghdr>() as u32 && (*nlh).nlmsg_len as c_int <= len
}
unsafe fn nlmsg_next(nlh: *mut nlmsghdr, len: &mut c_int) -> *mut nlmsghdr {
    let aligned = nlmsg_align((*nlh).nlmsg_len as usize) as c_int;
    *len -= aligned;
    (nlh as *mut u8).add(aligned as usize) as *mut nlmsghdr
}
unsafe fn rta_ok(rta: *mut rtattr, len: c_int) -> bool {
    len >= size_of::<rtattr>() as c_int && (*rta).rta_len >= size_of::<rtattr>() as u16 && (*rta).rta_len as c_int <= len
}
unsafe fn rta_next(rta: *mut rtattr, len: &mut c_int) -> *mut rtattr {
    let aligned = rta_align((*rta).rta_len as usize) as c_int;
    *len -= aligned;
    (rta as *mut u8).add(aligned as usize) as *mut rtattr
}
unsafe fn fd_zero(set: *mut fd_set) { (*set).fds_bits = [0; 16]; }
unsafe fn fd_set_bit(fd: c_int, set: *mut fd_set) {
    let bits = 8 * size_of::<isize>();
    (*set).fds_bits[fd as usize / bits] |= 1isize << (fd as usize % bits);
}

unsafe fn cstr(s: &'static [u8]) -> *const c_char { s.as_ptr() as *const c_char }

unsafe fn syntax(argv: *mut *mut c_char) -> ! {
    fprintf(stderr, cstr(b"%s add|ann|rem|csf|dsf|get|set|del|flush|dump|events|listen|accept [<args>]\n\0"), *argv);
    fprintf(stderr, cstr(b"\tadd [flags signal|subflow|backup|fullmesh] [id <nr>] [dev <name>] <ip>\n\0"));
    fprintf(stderr, cstr(b"\tann <local-ip> id <local-id> token <token> [port <local-port>] [dev <name>]\n\0"));
    fprintf(stderr, cstr(b"\trem id <local-id> token <token>\n\0"));
    fprintf(stderr, cstr(b"\tcsf lip <local-ip> lid <local-id> rip <remote-ip> rport <remote-port> token <token>\n\0"));
    fprintf(stderr, cstr(b"\tdsf lip <local-ip> lport <local-port> rip <remote-ip> rport <remote-port> token <token>\n\0"));
    fprintf(stderr, cstr(b"\tdel <id> [<ip>]\n\0"));
    fprintf(stderr, cstr(b"\tget <id>\n\0"));
    fprintf(stderr, cstr(b"\tset [<ip>] [id <nr>] flags [no]backup|[no]fullmesh [port <nr>] [token <token>] [rip <ip>] [rport <port>]\n\0"));
    fprintf(stderr, cstr(b"\tflush\n\0"));
    fprintf(stderr, cstr(b"\tdump\n\0"));
    fprintf(stderr, cstr(b"\tlimits [<rcv addr max> <subflow max>]\n\0"));
    fprintf(stderr, cstr(b"\tevents\n\0"));
    fprintf(stderr, cstr(b"\tlisten <local-ip> <local-port>\n\0"));
    exit(0);
}

unsafe fn init_genl_req(data: *mut c_char, family: c_int, cmd: c_int, version: c_int) -> c_int {
    let nh = data as *mut nlmsghdr;
    let mut off = 0;
    (*nh).nlmsg_type = family as u16;
    (*nh).nlmsg_flags = NLM_F_REQUEST;
    (*nh).nlmsg_len = nlmsg_length(GENL_HDRLEN) as u32;
    off += nlmsg_align(size_of::<nlmsghdr>()) as c_int;
    let gh = data.add(off as usize) as *mut genlmsghdr;
    (*gh).cmd = cmd as u8;
    (*gh).version = version as u8;
    off += nlmsg_align(size_of::<genlmsghdr>()) as c_int;
    off
}

unsafe fn nl_error(nh: *mut nlmsghdr) -> c_int {
    let err = nlmsg_data(nh) as *mut nlmsgerr;
    let mut len = (*nh).nlmsg_len as c_int - size_of::<nlmsghdr>() as c_int;
    let mut off: u32 = 0;
    if len < size_of::<nlmsgerr>() as c_int {
        error(1, 0, cstr(b"netlink error message truncated %d min %ld\0"), len, size_of::<nlmsgerr>());
    }
    if (*err).error != 0 {
        let mut attrs = nlmsg_data(nh) as *mut rtattr;
        fprintf(stderr, cstr(b"netlink error %d (%s)\n\0"), (*err).error, strerror(-(*err).error));
        while rta_ok(attrs, len) {
            if (*attrs).rta_type == NLMSGERR_ATTR_MSG {
                fprintf(stderr, cstr(b"netlink ext ack msg: %s\n\0"), rta_data(attrs) as *mut c_char);
            }
            if (*attrs).rta_type == NLMSGERR_ATTR_OFFS {
                memcpy(&mut off as *mut _ as *mut c_void, rta_data(attrs), 4);
                fprintf(stderr, cstr(b"netlink err off %d\n\0"), off as c_int);
            }
            attrs = rta_next(attrs, &mut len);
        }
        return -1;
    }
    0
}

unsafe fn do_nl_req(fd: c_int, mut nh: *mut nlmsghdr, len: c_int, mut max: c_int) -> c_int {
    let data = nh as *mut c_void;
    let mut err = 0;
    if max == 0 {
        (*nh).nlmsg_flags |= NLM_F_ACK;
        max = 1024;
    }
    (*nh).nlmsg_len = len as u32;
    let mut ret = send(fd, data, len as usize, 0) as c_int;
    if ret != len {
        error(1, errno, cstr(b"send netlink: %uB != %uB\n\0"), ret, len);
    }
    ret = recv(fd, data, max as usize, 0) as c_int;
    if ret < 0 {
        error(1, errno, cstr(b"recv netlink: %uB\n\0"), ret);
    }
    let mut rem = ret;
    while nlmsg_ok(nh, rem) {
        if (*nh).nlmsg_type == NLMSG_DONE { break; }
        if (*nh).nlmsg_type == NLMSG_ERROR && nl_error(nh) != 0 { err = 1; }
        nh = nlmsg_next(nh, &mut rem);
    }
    if err != 0 {
        error(1, 0, cstr(b"bailing out due to netlink error[s]\0"));
    }
    ret
}

unsafe fn addattr(data: *mut c_char, off: &mut c_int, typ: u16, src: *const c_void, len: usize) -> *mut rtattr {
    let rta = data.add(*off as usize) as *mut rtattr;
    (*rta).rta_type = typ;
    (*rta).rta_len = rta_length(len);
    if len != 0 {
        memcpy(rta_data(rta), src, len);
    }
    *off += nlmsg_align((*rta).rta_len as usize) as c_int;
    rta
}

unsafe fn add_addr_attr_ip(data: *mut c_char, off: &mut c_int, ip: *const c_char, family: &mut u_int16_t) {
    let rta = data.add(*off as usize) as *mut rtattr;
    if inet_pton(AF_INET, ip, rta_data(rta)) != 0 {
        *family = AF_INET as u16;
        (*rta).rta_type = MPTCP_PM_ADDR_ATTR_ADDR4;
        (*rta).rta_len = rta_length(4);
    } else if inet_pton(AF_INET6, ip, rta_data(rta)) != 0 {
        *family = AF_INET6 as u16;
        (*rta).rta_type = MPTCP_PM_ADDR_ATTR_ADDR6;
        (*rta).rta_len = rta_length(16);
    } else {
        error(1, errno, cstr(b"can't parse ip %s\0"), ip);
    }
    *off += nlmsg_align((*rta).rta_len as usize) as c_int;
}

unsafe fn capture_events(fd: c_int, event_group: c_int) -> c_int {
    let mut buffer = [0u8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let mut rfds: fd_set = zeroed();
    if setsockopt(fd, SOL_NETLINK, NETLINK_ADD_MEMBERSHIP, &event_group as *const _ as *const c_void, size_of::<c_int>() as socklen_t) < 0 {
        error(1, errno, cstr(b"could not join the mptcp_pm_events mcast group\0"));
    }
    loop {
        let mut server_side = false;
        fd_zero(&mut rfds);
        fd_set_bit(fd, &mut rfds);
        let mut res_len = buffer.len() as c_int;
        let ret = select(FD_SETSIZE, &mut rfds, null_mut(), null_mut(), null_mut());
        if ret < 0 { error(1, ret, cstr(b"error in select() on NL socket\0")); }
        res_len = recv(fd, buffer.as_mut_ptr() as *mut c_void, res_len as usize, 0) as c_int;
        if res_len < 0 { error(1, res_len, cstr(b"error on recv() from NL socket\0")); }
        let mut nh = buffer.as_mut_ptr() as *mut nlmsghdr;
        while nlmsg_ok(nh, res_len) {
            if (*nh).nlmsg_type == NLMSG_ERROR { error(1, NLMSG_ERROR as c_int, cstr(b"received invalid NL message\0")); }
            let ghdr = nlmsg_data(nh) as *mut genlmsghdr;
            if (*ghdr).cmd != 0 {
                fprintf(stderr, cstr(b"type:%d\0"), (*ghdr).cmd as c_int);
                let mut msg_len = (*nh).nlmsg_len as c_int - nlmsg_length(GENL_HDRLEN) as c_int;
                let mut attrs = (ghdr as *mut c_char).add(GENL_HDRLEN) as *mut rtattr;
                while rta_ok(attrs, msg_len) {
                    match (*attrs).rta_type {
                        MPTCP_ATTR_TOKEN => fprintf(stderr, cstr(b",token:%u\0"), *(rta_data(attrs) as *mut __u32)),
                        MPTCP_ATTR_FAMILY => fprintf(stderr, cstr(b",family:%u\0"), *(rta_data(attrs) as *mut __u16) as c_int),
                        MPTCP_ATTR_LOC_ID => fprintf(stderr, cstr(b",loc_id:%u\0"), *(rta_data(attrs) as *mut __u8) as c_int),
                        MPTCP_ATTR_REM_ID => fprintf(stderr, cstr(b",rem_id:%u\0"), *(rta_data(attrs) as *mut __u8) as c_int),
                        MPTCP_ATTR_SADDR4 | MPTCP_ATTR_DADDR4 => {
                            let a = ntohl(*(rta_data(attrs) as *mut __u32));
                            fprintf(stderr, if (*attrs).rta_type == MPTCP_ATTR_SADDR4 { cstr(b",saddr4:%u.%u.%u.%u\0") } else { cstr(b",daddr4:%u.%u.%u.%u\0") }, a >> 24, (a >> 16) & 0xff, (a >> 8) & 0xff, a & 0xff);
                        }
                        MPTCP_ATTR_SADDR6 | MPTCP_ATTR_DADDR6 => {
                            let mut buf = [0i8; INET6_ADDRSTRLEN];
                            if !inet_ntop(AF_INET6, rta_data(attrs), buf.as_mut_ptr(), buf.len() as socklen_t).is_null() {
                                fprintf(stderr, if (*attrs).rta_type == MPTCP_ATTR_SADDR6 { cstr(b",saddr6:%s\0") } else { cstr(b",daddr6:%s\0") }, buf.as_mut_ptr());
                            }
                        }
                        MPTCP_ATTR_SPORT => fprintf(stderr, cstr(b",sport:%u\0"), ntohs(*(rta_data(attrs) as *mut __u16)) as c_int),
                        MPTCP_ATTR_DPORT => fprintf(stderr, cstr(b",dport:%u\0"), ntohs(*(rta_data(attrs) as *mut __u16)) as c_int),
                        MPTCP_ATTR_BACKUP => fprintf(stderr, cstr(b",backup:%u\0"), *(rta_data(attrs) as *mut __u8) as c_int),
                        MPTCP_ATTR_ERROR => fprintf(stderr, cstr(b",error:%u\0"), *(rta_data(attrs) as *mut __u8) as c_int),
                        MPTCP_ATTR_SERVER_SIDE => server_side = *(rta_data(attrs) as *mut __u8) != 0,
                        MPTCP_ATTR_FLAGS => {
                            let flags = *(rta_data(attrs) as *mut __u16);
                            if flags & MPTCP_PM_EV_FLAG_DENY_JOIN_ID0 != 0 { fprintf(stderr, cstr(b",deny_join_id0:1\0")); }
                            if flags & MPTCP_PM_EV_FLAG_SERVER_SIDE != 0 { server_side = true; }
                        }
                        _ => {}
                    }
                    attrs = rta_next(attrs, &mut msg_len);
                }
            }
            nh = nlmsg_next(nh, &mut res_len);
        }
        if server_side { fprintf(stderr, cstr(b",server_side:1\0")); }
        fprintf(stderr, cstr(b"\n\0"));
    }
}

unsafe fn genl_parse_getfamily(nlh: *mut nlmsghdr, pm_family: *mut c_int, events_mcast_grp: *mut c_int) -> c_int {
    let ghdr = nlmsg_data(nlh) as *mut genlmsghdr;
    let mut len = (*nlh).nlmsg_len as c_int;
    if (*nlh).nlmsg_type != GENL_ID_CTRL { error(1, errno, cstr(b"Not a controller message, len=%d type=0x%x\n\0"), (*nlh).nlmsg_len, (*nlh).nlmsg_type as c_int); }
    len -= nlmsg_length(GENL_HDRLEN) as c_int;
    if len < 0 { error(1, errno, cstr(b"wrong controller message len %d\n\0"), len); }
    if (*ghdr).cmd != CTRL_CMD_NEWFAMILY { error(1, errno, cstr(b"Unknown controller command %d\n\0"), (*ghdr).cmd as c_int); }
    let mut attrs = (ghdr as *mut c_char).add(GENL_HDRLEN) as *mut rtattr;
    let mut got_family = 0;
    let mut got_events_grp = 0;
    while rta_ok(attrs, len) {
        if (*attrs).rta_type == CTRL_ATTR_FAMILY_ID {
            *pm_family = *(rta_data(attrs) as *mut __u16) as c_int;
            got_family = 1;
        } else if (*attrs).rta_type == CTRL_ATTR_MCAST_GROUPS {
            let mut grps = rta_data(attrs) as *mut rtattr;
            let mut grps_len = rta_payload(attrs);
            while rta_ok(grps, grps_len) {
                let mut grp = rta_data(grps) as *mut rtattr;
                let mut grp_len = rta_payload(grps);
                got_events_grp = 0;
                while rta_ok(grp, grp_len) {
                    if (*grp).rta_type == CTRL_ATTR_MCAST_GRP_ID {
                        *events_mcast_grp = *(rta_data(grp) as *mut __u32) as c_int;
                    } else if (*grp).rta_type == CTRL_ATTR_MCAST_GRP_NAME && strcmp(rta_data(grp) as *mut c_char, &MPTCP_PM_EV_GRP_NAME as *const c_char) == 0 {
                        got_events_grp = 1;
                    }
                    grp = rta_next(grp, &mut grp_len);
                }
                if got_events_grp != 0 { break; }
                grps = rta_next(grps, &mut grps_len);
            }
        }
        if got_family != 0 && got_events_grp != 0 { return 0; }
        attrs = rta_next(attrs, &mut len);
    }
    error(1, errno, cstr(b"can't find CTRL_ATTR_FAMILY_ID attr\0"));
}

unsafe fn resolve_mptcp_pm_netlink(fd: c_int, pm_family: *mut c_int, events_mcast_grp: *mut c_int) -> c_int {
    let mut data = [0i8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let nh = data.as_mut_ptr() as *mut nlmsghdr;
    memset(data.as_mut_ptr() as *mut c_void, 0, data.len());
    let mut off = init_genl_req(data.as_mut_ptr(), GENL_ID_CTRL as c_int, CTRL_CMD_GETFAMILY as c_int, 0);
    let rta = data.as_mut_ptr().add(off as usize) as *mut rtattr;
    let namelen = strlen(&MPTCP_PM_NAME as *const c_char) + 1;
    (*rta).rta_type = CTRL_ATTR_FAMILY_NAME;
    (*rta).rta_len = rta_length(namelen);
    memcpy(rta_data(rta), &MPTCP_PM_NAME as *const c_char as *const c_void, namelen);
    off += nlmsg_align((*rta).rta_len as usize) as c_int;
    do_nl_req(fd, nh, off, data.len() as c_int);
    genl_parse_getfamily(data.as_mut_ptr() as *mut nlmsghdr, pm_family, events_mcast_grp)
}

unsafe fn parse_flags(flags_arg: *mut c_char, allow_add: bool) -> u32 {
    let mut flags = 0u32;
    let mut strp = flags_arg;
    loop {
        let tok = strtok(strp, cstr(b",\0"));
        if tok.is_null() { break; }
        strp = null_mut();
        if strcmp(tok, cstr(b"subflow\0")) == 0 && allow_add { flags |= MPTCP_PM_ADDR_FLAG_SUBFLOW; }
        else if strcmp(tok, cstr(b"signal\0")) == 0 && allow_add { flags |= MPTCP_PM_ADDR_FLAG_SIGNAL; }
        else if strcmp(tok, cstr(b"laminar\0")) == 0 && allow_add { flags |= MPTCP_PM_ADDR_FLAG_LAMINAR; }
        else if strcmp(tok, cstr(b"backup\0")) == 0 { flags |= MPTCP_PM_ADDR_FLAG_BACKUP; }
        else if strcmp(tok, cstr(b"fullmesh\0")) == 0 { flags |= MPTCP_PM_ADDR_FLAG_FULLMESH; }
        else if strcmp(tok, cstr(b"unknown\0")) == 0 && allow_add { flags |= MPTCP_PM_ADDR_FLAG_UNKNOWN; }
        else if !allow_add && (strcmp(tok, cstr(b"nobackup\0")) == 0 || strcmp(tok, cstr(b"nofullmesh\0")) == 0) {}
        else { error(1, errno, cstr(b"unknown flag %s\0"), flags_arg); }
    }
    flags
}

unsafe fn add_nested_addr(data: *mut c_char, off: &mut c_int, typ: u16, ip: *const c_char, port_or_id: *const c_char, is_port: bool, flags: u32) {
    let addr_start = *off;
    let addr = data.add(*off as usize) as *mut rtattr;
    (*addr).rta_type = NLA_F_NESTED | typ;
    (*addr).rta_len = rta_length(0);
    *off += nlmsg_align((*addr).rta_len as usize) as c_int;
    let mut family = 0u16;
    add_addr_attr_ip(data, off, ip, &mut family);
    addattr(data, off, MPTCP_PM_ADDR_ATTR_FAMILY, &family as *const _ as *const c_void, 2);
    if !port_or_id.is_null() {
        if is_port {
            let port: u_int16_t = atoi(port_or_id) as u16;
            addattr(data, off, MPTCP_PM_ADDR_ATTR_PORT, &port as *const _ as *const c_void, 2);
        } else {
            let id: u_int8_t = atoi(port_or_id) as u8;
            addattr(data, off, MPTCP_PM_ADDR_ATTR_ID, &id as *const _ as *const c_void, 1);
        }
    }
    if flags != u32::MAX {
        addattr(data, off, MPTCP_PM_ADDR_ATTR_FLAGS, &flags as *const _ as *const c_void, 4);
    }
    (*addr).rta_len = (*off - addr_start) as u16;
}

#[no_mangle]
pub unsafe extern "C" fn dsf(fd: c_int, pm_family: c_int, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut data = [0i8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let mut params: [*mut c_char; 5] = [null_mut(); 5];
    let nh = data.as_mut_ptr() as *mut nlmsghdr;
    memset(data.as_mut_ptr() as *mut c_void, 0, data.len());
    let mut off = init_genl_req(data.as_mut_ptr(), pm_family, MPTCP_PM_CMD_SUBFLOW_DESTROY, MPTCP_PM_VER);
    if argc < 12 { syntax(argv); }
    let mut arg = 2;
    while arg < argc {
        let slot = if strcmp(*argv.add(arg as usize), cstr(b"lip\0")) == 0 { 0 } else if strcmp(*argv.add(arg as usize), cstr(b"lport\0")) == 0 { 1 } else if strcmp(*argv.add(arg as usize), cstr(b"rip\0")) == 0 { 2 } else if strcmp(*argv.add(arg as usize), cstr(b"rport\0")) == 0 { 3 } else if strcmp(*argv.add(arg as usize), cstr(b"token\0")) == 0 { 4 } else { error(1, 0, cstr(b"unknown keyword %s\0"), *argv.add(arg as usize)); };
        arg += 1; if arg >= argc { error(1, 0, cstr(b" missing value\0")); }
        params[slot] = *argv.add(arg as usize); arg += 1;
    }
    add_nested_addr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_ADDR, params[0], params[1], true, u32::MAX);
    add_nested_addr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_ADDR_REMOTE, params[2], params[3], true, u32::MAX);
    let token: u_int32_t = strtoul(params[4], null_mut(), 10) as u32;
    addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_TOKEN, &token as *const _ as *const c_void, 4);
    do_nl_req(fd, nh, off, 0);
    0
}

#[no_mangle]
pub unsafe extern "C" fn csf(fd: c_int, pm_family: c_int, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut data = [0i8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let flags: u_int32_t = MPTCP_PM_ADDR_FLAG_SUBFLOW;
    let mut params: [*mut c_char; 5] = [null_mut(); 5];
    let nh = data.as_mut_ptr() as *mut nlmsghdr;
    memset(data.as_mut_ptr() as *mut c_void, 0, data.len());
    let mut off = init_genl_req(data.as_mut_ptr(), pm_family, MPTCP_PM_CMD_SUBFLOW_CREATE, MPTCP_PM_VER);
    if argc < 12 { syntax(argv); }
    let mut arg = 2;
    while arg < argc {
        let slot = if strcmp(*argv.add(arg as usize), cstr(b"lip\0")) == 0 { 0 } else if strcmp(*argv.add(arg as usize), cstr(b"lid\0")) == 0 { 1 } else if strcmp(*argv.add(arg as usize), cstr(b"rip\0")) == 0 { 2 } else if strcmp(*argv.add(arg as usize), cstr(b"rport\0")) == 0 { 3 } else if strcmp(*argv.add(arg as usize), cstr(b"token\0")) == 0 { 4 } else { error(1, 0, cstr(b"unknown param %s\0"), *argv.add(arg as usize)); };
        arg += 1; if arg >= argc { error(1, 0, cstr(b" missing value\0")); }
        params[slot] = *argv.add(arg as usize); arg += 1;
    }
    add_nested_addr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_ADDR, params[0], params[1], false, flags);
    add_nested_addr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_ADDR_REMOTE, params[2], params[3], true, flags);
    let token: u_int32_t = strtoul(params[4], null_mut(), 10) as u32;
    addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_TOKEN, &token as *const _ as *const c_void, 4);
    do_nl_req(fd, nh, off, 0);
    0
}

#[no_mangle]
pub unsafe extern "C" fn remove_addr(fd: c_int, pm_family: c_int, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut data = [0i8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let nh = data.as_mut_ptr() as *mut nlmsghdr;
    memset(data.as_mut_ptr() as *mut c_void, 0, data.len());
    let mut off = init_genl_req(data.as_mut_ptr(), pm_family, MPTCP_PM_CMD_REMOVE, MPTCP_PM_VER);
    if argc < 6 { syntax(argv); }
    let mut arg = 2;
    while arg < argc {
        if strcmp(*argv.add(arg as usize), cstr(b"id\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing id value\0")); }
            let id: u_int8_t = atoi(*argv.add(arg as usize)) as u8;
            addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_LOC_ID, &id as *const _ as *const c_void, 1);
        } else if strcmp(*argv.add(arg as usize), cstr(b"token\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing token value\0")); }
            let token: u_int32_t = strtoul(*argv.add(arg as usize), null_mut(), 10) as u32;
            addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_TOKEN, &token as *const _ as *const c_void, 4);
        } else { error(1, 0, cstr(b"unknown keyword %s\0"), *argv.add(arg as usize)); }
        arg += 1;
    }
    do_nl_req(fd, nh, off, 0);
    0
}

#[no_mangle]
pub unsafe extern "C" fn announce_addr(fd: c_int, pm_family: c_int, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut data = [0i8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let flags: u_int32_t = MPTCP_PM_ADDR_FLAG_SIGNAL;
    let mut token: u_int32_t = u32::MAX;
    let mut id: u_int32_t = u32::MAX;
    let nh = data.as_mut_ptr() as *mut nlmsghdr;
    memset(data.as_mut_ptr() as *mut c_void, 0, data.len());
    let mut off = init_genl_req(data.as_mut_ptr(), pm_family, MPTCP_PM_CMD_ANNOUNCE, MPTCP_PM_VER);
    if argc < 7 { syntax(argv); }
    let addr_start = off;
    let addr = data.as_mut_ptr().add(off as usize) as *mut rtattr;
    (*addr).rta_type = NLA_F_NESTED | MPTCP_PM_ATTR_ADDR;
    (*addr).rta_len = rta_length(0);
    off += nlmsg_align((*addr).rta_len as usize) as c_int;
    let mut family = 0u16;
    add_addr_attr_ip(data.as_mut_ptr(), &mut off, *argv.add(2), &mut family);
    addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_FAMILY, &family as *const _ as *const c_void, 2);
    let mut arg = 3;
    while arg < argc {
        if strcmp(*argv.add(arg as usize), cstr(b"id\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing id value\0")); }
            id = atoi(*argv.add(arg as usize)) as u32;
            let id8 = id as u8;
            addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_ID, &id8 as *const _ as *const c_void, 1);
        } else if strcmp(*argv.add(arg as usize), cstr(b"dev\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing dev name\0")); }
            let ifindex: int32_t = if_nametoindex(*argv.add(arg as usize)) as i32;
            if ifindex == 0 { error(1, errno, cstr(b"unknown device %s\0"), *argv.add(arg as usize)); }
            addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_IF_IDX, &ifindex as *const _ as *const c_void, 4);
        } else if strcmp(*argv.add(arg as usize), cstr(b"port\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing port value\0")); }
            let port: u_int16_t = atoi(*argv.add(arg as usize)) as u16;
            addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_PORT, &port as *const _ as *const c_void, 2);
        } else if strcmp(*argv.add(arg as usize), cstr(b"token\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing token value\0")); }
            token = strtoul(*argv.add(arg as usize), null_mut(), 10) as u32;
        } else { error(1, 0, cstr(b"unknown keyword %s\0"), *argv.add(arg as usize)); }
        arg += 1;
    }
    addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_FLAGS, &flags as *const _ as *const c_void, 4);
    (*addr).rta_len = (off - addr_start) as u16;
    if id == u32::MAX || token == u32::MAX { error(1, 0, cstr(b" missing mandatory inputs\0")); }
    addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_TOKEN, &token as *const _ as *const c_void, 4);
    do_nl_req(fd, nh, off, 0);
    0
}

#[no_mangle]
pub unsafe extern "C" fn add_addr(fd: c_int, pm_family: c_int, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut data = [0i8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let nh = data.as_mut_ptr() as *mut nlmsghdr;
    let mut flags: u_int32_t = 0;
    memset(data.as_mut_ptr() as *mut c_void, 0, data.len());
    let mut off = init_genl_req(data.as_mut_ptr(), pm_family, MPTCP_PM_CMD_ADD_ADDR, MPTCP_PM_VER);
    if argc < 3 { syntax(argv); }
    let nest_start = off;
    let nest = data.as_mut_ptr().add(off as usize) as *mut rtattr;
    (*nest).rta_type = NLA_F_NESTED | MPTCP_PM_ATTR_ADDR;
    (*nest).rta_len = rta_length(0);
    off += nlmsg_align((*nest).rta_len as usize) as c_int;
    let mut family = 0u16;
    add_addr_attr_ip(data.as_mut_ptr(), &mut off, *argv.add(2), &mut family);
    addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_FAMILY, &family as *const _ as *const c_void, 2);
    let mut arg = 3;
    while arg < argc {
        if strcmp(*argv.add(arg as usize), cstr(b"flags\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing flags value\0")); }
            flags = parse_flags(*argv.add(arg as usize), true);
            if flags & MPTCP_PM_ADDR_FLAG_SIGNAL != 0 && flags & MPTCP_PM_ADDR_FLAG_FULLMESH != 0 { error(1, errno, cstr(b"error flag fullmesh\0")); }
            addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_FLAGS, &flags as *const _ as *const c_void, 4);
        } else if strcmp(*argv.add(arg as usize), cstr(b"id\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing id value\0")); }
            let id: u_int8_t = atoi(*argv.add(arg as usize)) as u8;
            addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_ID, &id as *const _ as *const c_void, 1);
        } else if strcmp(*argv.add(arg as usize), cstr(b"dev\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing dev name\0")); }
            let ifindex: int32_t = if_nametoindex(*argv.add(arg as usize)) as i32;
            if ifindex == 0 { error(1, errno, cstr(b"unknown device %s\0"), *argv.add(arg as usize)); }
            addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_IF_IDX, &ifindex as *const _ as *const c_void, 4);
        } else if strcmp(*argv.add(arg as usize), cstr(b"port\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing port value\0")); }
            if flags & MPTCP_PM_ADDR_FLAG_SIGNAL == 0 { error(1, 0, cstr(b" flags must be signal when using port\0")); }
            let port: u_int16_t = atoi(*argv.add(arg as usize)) as u16;
            addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_PORT, &port as *const _ as *const c_void, 2);
        } else { error(1, 0, cstr(b"unknown keyword %s\0"), *argv.add(arg as usize)); }
        arg += 1;
    }
    (*nest).rta_len = (off - nest_start) as u16;
    do_nl_req(fd, nh, off, 0);
    0
}

#[no_mangle]
pub unsafe extern "C" fn del_addr(fd: c_int, pm_family: c_int, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut data = [0i8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let nh = data.as_mut_ptr() as *mut nlmsghdr;
    memset(data.as_mut_ptr() as *mut c_void, 0, data.len());
    let mut off = init_genl_req(data.as_mut_ptr(), pm_family, MPTCP_PM_CMD_DEL_ADDR, MPTCP_PM_VER);
    if argc != 3 && argc != 4 { syntax(argv); }
    let id: u_int8_t = atoi(*argv.add(2)) as u8;
    if id == 0 && argc != 4 { syntax(argv); }
    let nest_start = off;
    let nest = data.as_mut_ptr().add(off as usize) as *mut rtattr;
    (*nest).rta_type = NLA_F_NESTED | MPTCP_PM_ATTR_ADDR;
    (*nest).rta_len = rta_length(0);
    off += nlmsg_align((*nest).rta_len as usize) as c_int;
    addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_ID, &id as *const _ as *const c_void, 1);
    if id == 0 {
        let mut family = 0u16;
        add_addr_attr_ip(data.as_mut_ptr(), &mut off, *argv.add(3), &mut family);
        addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_FAMILY, &family as *const _ as *const c_void, 2);
    }
    (*nest).rta_len = (off - nest_start) as u16;
    do_nl_req(fd, nh, off, 0);
    0
}

unsafe fn print_addr(mut attrs: *mut rtattr, mut len: c_int) {
    let mut family: u16 = 0;
    let mut port: u16 = 0;
    let mut strbuf = [0i8; 1024];
    let mut flags: u32;
    let mut id: u8 = 0;
    while rta_ok(attrs, len) {
        if (*attrs).rta_type == MPTCP_PM_ADDR_ATTR_FAMILY { memcpy(&mut family as *mut _ as *mut c_void, rta_data(attrs), 2); }
        if (*attrs).rta_type == MPTCP_PM_ADDR_ATTR_PORT { memcpy(&mut port as *mut _ as *mut c_void, rta_data(attrs), 2); }
        if (*attrs).rta_type == MPTCP_PM_ADDR_ATTR_ADDR4 || (*attrs).rta_type == MPTCP_PM_ADDR_ATTR_ADDR6 {
            let af = if (*attrs).rta_type == MPTCP_PM_ADDR_ATTR_ADDR4 { AF_INET } else { AF_INET6 };
            if family as c_int != af { error(1, errno, cstr(b"wrong IP for family %d\0"), family as c_int); }
            inet_ntop(af, rta_data(attrs), strbuf.as_mut_ptr(), strbuf.len() as socklen_t);
            printf(cstr(b"%s\0"), strbuf.as_mut_ptr());
            if port != 0 { printf(cstr(b" %d\0"), port as c_int); }
        }
        if (*attrs).rta_type == MPTCP_PM_ADDR_ATTR_ID {
            memcpy(&mut id as *mut _ as *mut c_void, rta_data(attrs), 1);
            printf(cstr(b"id %d \0"), id as c_int);
        }
        if (*attrs).rta_type == MPTCP_PM_ADDR_ATTR_FLAGS {
            memcpy(&mut flags as *mut _ as *mut c_void, rta_data(attrs), 4);
            printf(cstr(b"flags \0"));
            let names = [(MPTCP_PM_ADDR_FLAG_SIGNAL, cstr(b"signal\0")), (MPTCP_PM_ADDR_FLAG_SUBFLOW, cstr(b"subflow\0")), (MPTCP_PM_ADDR_FLAG_LAMINAR, cstr(b"laminar\0")), (MPTCP_PM_ADDR_FLAG_BACKUP, cstr(b"backup\0")), (MPTCP_PM_ADDR_FLAG_FULLMESH, cstr(b"fullmesh\0")), (MPTCP_PM_ADDR_FLAG_IMPLICIT, cstr(b"implicit\0")), (MPTCP_PM_ADDR_FLAG_UNKNOWN, cstr(b"unknown\0"))];
            for (bit, name) in names {
                if flags & bit != 0 {
                    printf(cstr(b"%s\0"), name);
                    flags &= !bit;
                    if flags != 0 { printf(cstr(b",\0")); }
                }
            }
            if flags != 0 { printf(cstr(b"0x%x\0"), flags); }
            printf(cstr(b" \0"));
        }
        if (*attrs).rta_type == MPTCP_PM_ADDR_ATTR_IF_IDX {
            let mut name = [0i8; IF_NAMESIZE];
            let mut ifindex: int32_t = 0;
            memcpy(&mut ifindex as *mut _ as *mut c_void, rta_data(attrs), 4);
            let ret = if_indextoname(ifindex as c_uint, name.as_mut_ptr());
            if !ret.is_null() { printf(cstr(b"dev %s \0"), ret); } else { printf(cstr(b"dev unknown/%d\0"), ifindex); }
        }
        attrs = rta_next(attrs, &mut len);
    }
    printf(cstr(b"\n\0"));
}

unsafe fn print_addrs(mut nh: *mut nlmsghdr, pm_family: c_int, mut total_len: c_int) {
    while nlmsg_ok(nh, total_len) {
        let mut len = (*nh).nlmsg_len as c_int;
        if (*nh).nlmsg_type == NLMSG_DONE { break; }
        if (*nh).nlmsg_type == NLMSG_ERROR { nl_error(nh); }
        if (*nh).nlmsg_type == pm_family as u16 {
            len -= nlmsg_length(GENL_HDRLEN) as c_int;
            let mut attrs = (nlmsg_data(nh) as *mut c_char).add(GENL_HDRLEN) as *mut rtattr;
            while rta_ok(attrs, len) {
                if (*attrs).rta_type == (MPTCP_PM_ATTR_ADDR | NLA_F_NESTED) {
                    print_addr(rta_data(attrs) as *mut rtattr, (*attrs).rta_len as c_int);
                }
                attrs = rta_next(attrs, &mut len);
            }
        }
        nh = nlmsg_next(nh, &mut total_len);
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_addr(fd: c_int, pm_family: c_int, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut data = [0i8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let nh = data.as_mut_ptr() as *mut nlmsghdr;
    let mut token: u_int32_t = 0;
    memset(data.as_mut_ptr() as *mut c_void, 0, data.len());
    let mut off = init_genl_req(data.as_mut_ptr(), pm_family, MPTCP_PM_CMD_GET_ADDR, MPTCP_PM_VER);
    if argc != 3 && argc != 5 { syntax(argv); }
    let id: u_int8_t = atoi(*argv.add(2)) as u8;
    if argc == 5 && strcmp(*argv.add(3), cstr(b"token\0")) == 0 { token = strtoul(*argv.add(4), null_mut(), 10) as u32; }
    let nest_start = off;
    let nest = data.as_mut_ptr().add(off as usize) as *mut rtattr;
    (*nest).rta_type = NLA_F_NESTED | MPTCP_PM_ATTR_ADDR;
    (*nest).rta_len = rta_length(0);
    off += nlmsg_align((*nest).rta_len as usize) as c_int;
    addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_ID, &id as *const _ as *const c_void, 1);
    (*nest).rta_len = (off - nest_start) as u16;
    if token != 0 { addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_TOKEN, &token as *const _ as *const c_void, 4); }
    let len = do_nl_req(fd, nh, off, data.len() as c_int);
    print_addrs(nh, pm_family, len);
    0
}

#[no_mangle]
pub unsafe extern "C" fn dump_addrs(fd: c_int, pm_family: c_int, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut data = [0i8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let nh = data.as_mut_ptr() as *mut nlmsghdr;
    let mut token: u_int32_t = 0;
    if argc != 2 && argc != 4 { syntax(argv); }
    if argc == 4 && strcmp(*argv.add(2), cstr(b"token\0")) == 0 { token = strtoul(*argv.add(3), null_mut(), 10) as u32; }
    memset(data.as_mut_ptr() as *mut c_void, 0, data.len());
    let mut off = init_genl_req(data.as_mut_ptr(), pm_family, MPTCP_PM_CMD_GET_ADDR, MPTCP_PM_VER);
    (*nh).nlmsg_flags |= NLM_F_DUMP;
    (*nh).nlmsg_seq = 1;
    (*nh).nlmsg_pid = getpid() as u32;
    (*nh).nlmsg_len = off as u32;
    if token != 0 { addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_TOKEN, &token as *const _ as *const c_void, 4); }
    let len = do_nl_req(fd, nh, off, data.len() as c_int);
    print_addrs(nh, pm_family, len);
    0
}

#[no_mangle]
pub unsafe extern "C" fn flush_addrs(fd: c_int, pm_family: c_int, _argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut data = [0i8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let nh = data.as_mut_ptr() as *mut nlmsghdr;
    memset(data.as_mut_ptr() as *mut c_void, 0, data.len());
    let off = init_genl_req(data.as_mut_ptr(), pm_family, MPTCP_PM_CMD_FLUSH_ADDRS, MPTCP_PM_VER);
    do_nl_req(fd, nh, off, 0);
    0
}

unsafe fn print_limits(mut nh: *mut nlmsghdr, pm_family: c_int, mut total_len: c_int) {
    while nlmsg_ok(nh, total_len) {
        let mut len = (*nh).nlmsg_len as c_int;
        if (*nh).nlmsg_type == NLMSG_DONE { break; }
        if (*nh).nlmsg_type == NLMSG_ERROR { nl_error(nh); }
        if (*nh).nlmsg_type == pm_family as u16 {
            len -= nlmsg_length(GENL_HDRLEN) as c_int;
            let mut attrs = (nlmsg_data(nh) as *mut c_char).add(GENL_HDRLEN) as *mut rtattr;
            while rta_ok(attrs, len) {
                let typ = (*attrs).rta_type;
                if typ == MPTCP_PM_ATTR_RCV_ADD_ADDRS || typ == MPTCP_PM_ATTR_SUBFLOWS {
                    let mut max: u32 = 0;
                    memcpy(&mut max as *mut _ as *mut c_void, rta_data(attrs), 4);
                    printf(cstr(b"%s %u\n\0"), if typ == MPTCP_PM_ATTR_SUBFLOWS { cstr(b"subflows\0") } else { cstr(b"accept\0") }, max);
                }
                attrs = rta_next(attrs, &mut len);
            }
        }
        nh = nlmsg_next(nh, &mut total_len);
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_set_limits(fd: c_int, pm_family: c_int, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut data = [0i8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let mut rcv_addr: u32 = 0;
    let mut subflows: u32 = 0;
    let mut len = data.len() as c_int;
    let cmd;
    if argc == 4 {
        rcv_addr = atoi(*argv.add(2)) as u32;
        subflows = atoi(*argv.add(3)) as u32;
        cmd = MPTCP_PM_CMD_SET_LIMITS;
    } else { cmd = MPTCP_PM_CMD_GET_LIMITS; }
    let nh = data.as_mut_ptr() as *mut nlmsghdr;
    memset(data.as_mut_ptr() as *mut c_void, 0, data.len());
    let mut off = init_genl_req(data.as_mut_ptr(), pm_family, cmd, MPTCP_PM_VER);
    if cmd == MPTCP_PM_CMD_SET_LIMITS {
        addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_RCV_ADD_ADDRS, &rcv_addr as *const _ as *const c_void, 4);
        addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_SUBFLOWS, &subflows as *const _ as *const c_void, 4);
        len = 0;
    }
    len = do_nl_req(fd, nh, off, len);
    if cmd == MPTCP_PM_CMD_GET_LIMITS { print_limits(nh, pm_family, len); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn add_listener(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut addr: sockaddr_storage = zeroed();
    let a4 = &mut addr as *mut _ as *mut sockaddr_in;
    let a6 = &mut addr as *mut _ as *mut sockaddr_in6;
    let mut family: u_int16_t = AF_UNSPEC as u16;
    let enable: c_int = 1;
    if argc < 4 { syntax(argv); }
    if inet_pton(AF_INET, *argv.add(2), &mut (*a4).sin_addr as *mut _ as *mut c_void) != 0 {
        family = AF_INET as u16;
        (*a4).sin_family = family;
        (*a4).sin_port = htons(atoi(*argv.add(3)) as u16);
    } else if inet_pton(AF_INET6, *argv.add(2), &mut (*a6).sin6_addr as *mut _ as *mut c_void) != 0 {
        family = AF_INET6 as u16;
        (*a6).sin6_family = family;
        (*a6).sin6_port = htons(atoi(*argv.add(3)) as u16);
    } else { error(1, errno, cstr(b"can't parse ip %s\0"), *argv.add(2)); }
    let sock = socket(family as c_int, SOCK_STREAM, IPPROTO_MPTCP);
    if sock < 0 { error(1, errno, cstr(b"can't create listener sock\n\0")); }
    if setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &enable as *const _ as *const c_void, size_of::<c_int>() as socklen_t) != 0 {
        close(sock);
        error(1, errno, cstr(b"can't set SO_REUSEADDR on listener sock\n\0"));
    }
    let err = bind(sock, &addr as *const _ as *const sockaddr, if family as c_int == AF_INET { size_of::<sockaddr_in>() } else { size_of::<sockaddr_in6>() } as socklen_t);
    if err == 0 && listen(sock, 30) == 0 { pause(); }
    close(sock);
    0
}

#[no_mangle]
pub unsafe extern "C" fn set_flags(fd: c_int, pm_family: c_int, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut data = [0i8; nlmsg_align(size_of::<nlmsghdr>()) + nlmsg_align(size_of::<genlmsghdr>()) + 1024];
    let nh = data.as_mut_ptr() as *mut nlmsghdr;
    let mut flags: u_int32_t = 0;
    let mut token: u_int32_t = 0;
    let mut rport: u_int16_t = 0;
    let mut rip: *mut c_void = null_mut();
    let mut use_id = 0;
    memset(data.as_mut_ptr() as *mut c_void, 0, data.len());
    let mut off = init_genl_req(data.as_mut_ptr(), pm_family, MPTCP_PM_CMD_SET_FLAGS, MPTCP_PM_VER);
    if argc < 3 { syntax(argv); }
    let nest_start = off;
    let nest = data.as_mut_ptr().add(off as usize) as *mut rtattr;
    (*nest).rta_type = NLA_F_NESTED | MPTCP_PM_ATTR_ADDR;
    (*nest).rta_len = rta_length(0);
    off += nlmsg_align((*nest).rta_len as usize) as c_int;
    let mut arg = 2;
    if strcmp(*argv.add(arg as usize), cstr(b"id\0")) == 0 {
        arg += 1; if arg >= argc { error(1, 0, cstr(b" missing id value\0")); }
        use_id = 1;
        let id: u_int8_t = atoi(*argv.add(arg as usize)) as u8;
        addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_ID, &id as *const _ as *const c_void, 1);
    } else {
        let mut family = 0u16;
        add_addr_attr_ip(data.as_mut_ptr(), &mut off, *argv.add(arg as usize), &mut family);
        addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_FAMILY, &family as *const _ as *const c_void, 2);
    }
    arg += 1;
    if arg >= argc { error(1, 0, cstr(b" missing flags keyword\0")); }
    while arg < argc {
        if strcmp(*argv.add(arg as usize), cstr(b"token\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing token value\0")); }
            token = strtoul(*argv.add(arg as usize), null_mut(), 10) as u32;
        } else if strcmp(*argv.add(arg as usize), cstr(b"flags\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing flags value\0")); }
            flags = parse_flags(*argv.add(arg as usize), false);
            addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_FLAGS, &flags as *const _ as *const c_void, 4);
        } else if strcmp(*argv.add(arg as usize), cstr(b"port\0")) == 0 {
            if use_id != 0 { error(1, 0, cstr(b" port can't be used with id\0")); }
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing port value\0")); }
            let port: u_int16_t = atoi(*argv.add(arg as usize)) as u16;
            addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_PORT, &port as *const _ as *const c_void, 2);
        } else if strcmp(*argv.add(arg as usize), cstr(b"rport\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing remote port\0")); }
            rport = atoi(*argv.add(arg as usize)) as u16;
        } else if strcmp(*argv.add(arg as usize), cstr(b"rip\0")) == 0 {
            arg += 1; if arg >= argc { error(1, 0, cstr(b" missing remote ip\0")); }
            rip = *argv.add(arg as usize) as *mut c_void;
        } else { error(1, 0, cstr(b"unknown keyword %s\0"), *argv.add(arg as usize)); }
        arg += 1;
    }
    (*nest).rta_len = (off - nest_start) as u16;
    if token != 0 { addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ATTR_TOKEN, &token as *const _ as *const c_void, 4); }
    if !rip.is_null() {
        let remote_start = off;
        let remote = data.as_mut_ptr().add(off as usize) as *mut rtattr;
        (*remote).rta_type = NLA_F_NESTED | MPTCP_PM_ATTR_ADDR_REMOTE;
        (*remote).rta_len = rta_length(0);
        off += nlmsg_align((*remote).rta_len as usize) as c_int;
        let mut family = 0u16;
        add_addr_attr_ip(data.as_mut_ptr(), &mut off, rip as *const c_char, &mut family);
        addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_FAMILY, &family as *const _ as *const c_void, 2);
        if rport != 0 { addattr(data.as_mut_ptr(), &mut off, MPTCP_PM_ADDR_ATTR_PORT, &rport as *const _ as *const c_void, 2); }
        (*remote).rta_len = (off - remote_start) as u16;
    }
    do_nl_req(fd, nh, off, 0);
    0
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut events_mcast_grp: c_int = 0;
    let mut pm_family: c_int = 0;
    if argc < 2 { syntax(argv); }
    let fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    if fd == -1 { error(1, errno, cstr(b"socket netlink\0")); }
    resolve_mptcp_pm_netlink(fd, &mut pm_family, &mut events_mcast_grp);
    let cmd = *argv.add(1);
    if strcmp(cmd, cstr(b"add\0")) == 0 { return add_addr(fd, pm_family, argc, argv); }
    else if strcmp(cmd, cstr(b"ann\0")) == 0 { return announce_addr(fd, pm_family, argc, argv); }
    else if strcmp(cmd, cstr(b"rem\0")) == 0 { return remove_addr(fd, pm_family, argc, argv); }
    else if strcmp(cmd, cstr(b"csf\0")) == 0 { return csf(fd, pm_family, argc, argv); }
    else if strcmp(cmd, cstr(b"dsf\0")) == 0 { return dsf(fd, pm_family, argc, argv); }
    else if strcmp(cmd, cstr(b"del\0")) == 0 { return del_addr(fd, pm_family, argc, argv); }
    else if strcmp(cmd, cstr(b"flush\0")) == 0 { return flush_addrs(fd, pm_family, argc, argv); }
    else if strcmp(cmd, cstr(b"get\0")) == 0 { return get_addr(fd, pm_family, argc, argv); }
    else if strcmp(cmd, cstr(b"dump\0")) == 0 { return dump_addrs(fd, pm_family, argc, argv); }
    else if strcmp(cmd, cstr(b"limits\0")) == 0 { return get_set_limits(fd, pm_family, argc, argv); }
    else if strcmp(cmd, cstr(b"set\0")) == 0 { return set_flags(fd, pm_family, argc, argv); }
    else if strcmp(cmd, cstr(b"events\0")) == 0 { return capture_events(fd, events_mcast_grp); }
    else if strcmp(cmd, cstr(b"listen\0")) == 0 { return add_listener(argc, argv); }
    fprintf(stderr, cstr(b"unknown sub-command: %s\0"), cmd);
    syntax(argv);
}
