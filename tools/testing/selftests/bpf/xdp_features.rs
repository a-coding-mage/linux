// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/xdp_features.c.
// C include dependencies preserved as extern declarations below:
// uapi/linux/bpf.h, uapi/linux/netdev.h, linux/if_link.h, signal.h, argp.h,
// net/if.h, sys/socket.h, netinet/in.h, netinet/tcp.h, unistd.h, arpa/inet.h,
// bpf/bpf.h, bpf/libbpf.h, pthread.h, network_helpers.h, bpf_util.h,
// xdp_features.skel.h, xdp_features.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void, VaList};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

const IF_NAMESIZE: usize = 16;
const BUFSIZE: usize = 128;

const AF_INET6: c_int = 10;
const SOCK_DGRAM: c_int = 2;
const SOCK_STREAM: c_int = 1;
const MSG_WAITALL: c_int = 0x100;
const MSG_NOSIGNAL: c_int = 0x4000;
const MSG_CONFIRM: c_int = 0x800;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;

const ARGP_KEY_ARG: c_int = 0;
const ARGP_ERR_UNKNOWN: c_int = 7;

const LIBBPF_DEBUG: libbpf_print_level = 0;
const LIBBPF_STRICT_ALL: c_int = 0xffffffffu32 as c_int;

const XDP_FLAGS_UPDATE_IF_NOEXIST: c_int = 1;
const XDP_FLAGS_DRV_MODE: c_int = 4;

const XDP_ABORTED: xdp_action = 0;
const XDP_DROP: xdp_action = 1;
const XDP_PASS: xdp_action = 2;
const XDP_TX: xdp_action = 3;
const XDP_REDIRECT: xdp_action = 4;

const NETDEV_XDP_ACT_BASIC: netdev_xdp_act = 1 << 0;
const NETDEV_XDP_ACT_REDIRECT: netdev_xdp_act = 1 << 1;
const NETDEV_XDP_ACT_NDO_XMIT: netdev_xdp_act = 1 << 2;

type size_t = usize;
type socklen_t = u32;
type pthread_t = usize;
type error_t = c_int;
type bool_t = bool;
type xdp_action = c_int;
type netdev_xdp_act = c_ulong;
type libbpf_print_level = c_int;
type test_commands = c_int;

const CMD_START: test_commands = 1;
const CMD_STOP: test_commands = 2;
const CMD_GET_XDP_CAP: test_commands = 3;
const CMD_GET_STATS: test_commands = 4;
const CMD_ACK: test_commands = 5;
const CMD_ECHO: test_commands = 6;
const DUT_ECHO_PORT: u16 = 48878;
const DUT_CTRL_PORT: u16 = 48879;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [u8; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
struct sockaddr_storage {
    ss_family: u16,
    __data: [u8; 126],
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
    args_doc: *const c_char,
    doc: *const c_char,
    children: *const c_void,
    help_filter: *const c_void,
    argp_domain: *const c_char,
}

#[repr(C)]
struct tlv_hdr {
    type_: u16,
    len: u16,
    data: [u8; 0],
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
struct xdp_features_maps {
    dev_map: *mut bpf_map,
    cpu_map: *mut bpf_map,
    dut_stats: *mut bpf_map,
    stats: *mut bpf_map,
}

#[repr(C)]
struct xdp_features_progs {
    xdp_do_redirect_cpumap: *mut bpf_program,
    xdp_do_tx: *mut bpf_program,
    xdp_do_drop: *mut bpf_program,
    xdp_do_aborted: *mut bpf_program,
    xdp_do_pass: *mut bpf_program,
    xdp_do_redirect: *mut bpf_program,
    xdp_tester_check_tx: *mut bpf_program,
    xdp_tester_check_rx: *mut bpf_program,
}

#[repr(C)]
struct xdp_features_rodata {
    tester_addr: in6_addr,
    dut_addr: in6_addr,
}

#[repr(C)]
struct xdp_features {
    maps: xdp_features_maps,
    progs: xdp_features_progs,
    rodata: *mut xdp_features_rodata,
}

#[repr(C)]
struct bpf_devmap_val {
    ifindex: c_uint,
}

#[repr(C)]
struct bpf_cpumap_val_bpf_prog {
    fd: c_int,
}

#[repr(C)]
struct bpf_cpumap_val {
    qsize: c_uint,
    bpf_prog: bpf_cpumap_val_bpf_prog,
}

#[repr(C)]
struct bpf_xdp_query_opts {
    sz: size_t,
    feature_flags: u64,
}

#[repr(C)]
struct Feature {
    drv_feature: netdev_xdp_act,
    action: xdp_action,
}

#[repr(C)]
struct Env {
    verbosity: bool_t,
    ifname: [c_char; IF_NAMESIZE],
    ifindex: c_int,
    is_tester: bool_t,
    feature: Feature,
    dut_ctrl_addr: sockaddr_storage,
    dut_addr: sockaddr_storage,
    tester_addr: sockaddr_storage,
}

static mut env: Env = Env {
    verbosity: false,
    ifname: [0; IF_NAMESIZE],
    ifindex: 0,
    is_tester: false,
    feature: Feature {
        drv_feature: 0,
        action: 0,
    },
    dut_ctrl_addr: sockaddr_storage {
        ss_family: 0,
        __data: [0; 126],
    },
    dut_addr: sockaddr_storage {
        ss_family: 0,
        __data: [0; 126],
    },
    tester_addr: sockaddr_storage {
        ss_family: 0,
        __data: [0; 126],
    },
};

static mut exiting: bool = false;

#[no_mangle]
pub static argp_program_version: &[u8; 17] = b"xdp-features 0.0\0";

#[no_mangle]
pub static argp_program_doc: &[u8] = b"XDP features detection application.\n\
\n\
XDP features application checks the XDP advertised features match detected ones.\n\
\n\
USAGE: ./xdp-features [-vt] [-f <xdp-feature>] [-D <dut-data-ip>] [-T <tester-data-ip>] [-C <dut-ctrl-ip>] <iface-name>\n\
\n\
dut-data-ip, tester-data-ip, dut-ctrl-ip: IPv6 or IPv4-mapped-IPv6 addresses;\n\
\n\
XDP features\n:\
- XDP_PASS\n\
- XDP_DROP\n\
- XDP_ABORTED\n\
- XDP_REDIRECT\n\
- XDP_NDO_XMIT\n\
- XDP_TX\n\0";

static OPTS: [argp_option; 7] = [
    argp_option { name: b"verbose\0".as_ptr() as *const c_char, key: b'v' as c_int, arg: null(), flags: 0, doc: b"Verbose debug output\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: b"tester\0".as_ptr() as *const c_char, key: b't' as c_int, arg: null(), flags: 0, doc: b"Tester mode\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: b"feature\0".as_ptr() as *const c_char, key: b'f' as c_int, arg: b"XDP-FEATURE\0".as_ptr() as *const c_char, flags: 0, doc: b"XDP feature to test\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: b"dut_data_ip\0".as_ptr() as *const c_char, key: b'D' as c_int, arg: b"DUT-DATA-IP\0".as_ptr() as *const c_char, flags: 0, doc: b"DUT IP data channel\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: b"dut_ctrl_ip\0".as_ptr() as *const c_char, key: b'C' as c_int, arg: b"DUT-CTRL-IP\0".as_ptr() as *const c_char, flags: 0, doc: b"DUT IP control channel\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: b"tester_data_ip\0".as_ptr() as *const c_char, key: b'T' as c_int, arg: b"TESTER-DATA-IP\0".as_ptr() as *const c_char, flags: 0, doc: b"Tester IP data channel\0".as_ptr() as *const c_char, group: 0 },
    argp_option { name: null(), key: 0, arg: null(), flags: 0, doc: null(), group: 0 },
];

static ARGP: argp = argp {
    options: OPTS.as_ptr(),
    parser: Some(parse_arg),
    args_doc: null(),
    doc: argp_program_doc.as_ptr() as *const c_char,
    children: null(),
    help_filter: null(),
    argp_domain: null(),
};

extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut c_void;
    static mut stdout: *mut c_void;

    fn vfprintf(stream: *mut c_void, format: *const c_char, args: VaList) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: size_t) -> isize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn recvfrom(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int, src_addr: *mut sockaddr, addrlen: *mut socklen_t) -> isize;
    fn sendto(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int, dest_addr: *const sockaddr, addrlen: socklen_t) -> isize;
    fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> isize;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;
    fn htonl(hostlong: u32) -> u32;
    fn ntohl(netlong: u32) -> u32;
    fn htobe64(host_64bits: u64) -> u64;
    fn be64toh(big_endian_64bits: u64) -> u64;
    fn argp_usage(state: *mut argp_state);
    fn argp_parse(argp: *const argp, argc: c_int, argv: *mut *mut c_char, flags: c_uint, arg_index: *mut c_int, input: *mut c_void) -> error_t;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn libbpf_set_strict_mode(mode: c_int);
    fn libbpf_set_print(print_fn: unsafe extern "C" fn(libbpf_print_level, *const c_char, VaList) -> c_int);
    fn bpf_map__update_elem(map: *mut bpf_map, key: *const c_void, key_sz: size_t, value: *const c_void, value_sz: size_t, flags: u64) -> c_int;
    fn bpf_map__lookup_elem(map: *mut bpf_map, key: *const c_void, key_sz: size_t, value: *mut c_void, value_sz: size_t, flags: u64) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: c_int, opts: *const c_void) -> c_int;
    fn bpf_xdp_detach(ifindex: c_int, flags: c_int, opts: *const c_void) -> c_int;
    fn bpf_xdp_query(ifindex: c_int, flags: c_int, opts: *mut bpf_xdp_query_opts) -> c_int;
    fn make_sockaddr(family: c_int, addr_str: *const c_char, port: u16, addr: *mut sockaddr_storage, len: *mut socklen_t) -> c_int;
    fn start_reuseport_server(family: c_int, type_: c_int, addr: *const c_void, port: u16, timeout_ms: c_int, reuseport: c_int) -> *mut c_int;
    fn free_fds(fds: *mut c_int, count: c_int);
    fn settimeo(fd: c_int, timeout_ms: c_int) -> c_int;
    fn xdp_features__open() -> *mut xdp_features;
    fn xdp_features__load(skel: *mut xdp_features) -> c_int;
    fn xdp_features__attach(skel: *mut xdp_features) -> c_int;
    fn xdp_features__destroy(skel: *mut xdp_features);
}

#[no_mangle]
pub unsafe extern "C" fn test__fail() {
    /* for network_helpers.c */
}

unsafe extern "C" fn libbpf_print_fn(level: libbpf_print_level, format: *const c_char, args: VaList) -> c_int {
    if level == LIBBPF_DEBUG && !env.verbosity {
        return 0;
    }
    vfprintf(stderr, format, args)
}

unsafe extern "C" fn sig_handler(_sig: c_int) {
    exiting = true;
}

unsafe fn get_xdp_feature(arg: *const c_char) -> c_int {
    if strcmp(arg, b"XDP_PASS\0".as_ptr() as *const c_char) == 0 {
        env.feature.action = XDP_PASS;
        env.feature.drv_feature = NETDEV_XDP_ACT_BASIC;
    } else if strcmp(arg, b"XDP_DROP\0".as_ptr() as *const c_char) == 0 {
        env.feature.drv_feature = NETDEV_XDP_ACT_BASIC;
        env.feature.action = XDP_DROP;
    } else if strcmp(arg, b"XDP_ABORTED\0".as_ptr() as *const c_char) == 0 {
        env.feature.drv_feature = NETDEV_XDP_ACT_BASIC;
        env.feature.action = XDP_ABORTED;
    } else if strcmp(arg, b"XDP_TX\0".as_ptr() as *const c_char) == 0 {
        env.feature.drv_feature = NETDEV_XDP_ACT_BASIC;
        env.feature.action = XDP_TX;
    } else if strcmp(arg, b"XDP_REDIRECT\0".as_ptr() as *const c_char) == 0 {
        env.feature.drv_feature = NETDEV_XDP_ACT_REDIRECT;
        env.feature.action = XDP_REDIRECT;
    } else if strcmp(arg, b"XDP_NDO_XMIT\0".as_ptr() as *const c_char) == 0 {
        env.feature.drv_feature = NETDEV_XDP_ACT_NDO_XMIT;
    } else {
        return -EINVAL;
    }

    0
}

unsafe fn get_xdp_feature_str() -> *const c_char {
    match env.feature.action {
        XDP_PASS => b"\x1b[0;33mXDP_PASS\x1b[0m\0".as_ptr() as *const c_char,
        XDP_DROP => b"\x1b[0;33mXDP_DROP\x1b[0m\0".as_ptr() as *const c_char,
        XDP_ABORTED => b"\x1b[0;33mXDP_ABORTED\x1b[0m\0".as_ptr() as *const c_char,
        XDP_TX => b"\x1b[0;33mXDP_TX\x1b[0m\0".as_ptr() as *const c_char,
        XDP_REDIRECT => b"\x1b[0;33mXDP_REDIRECT\x1b[0m\0".as_ptr() as *const c_char,
        _ => {
            if env.feature.drv_feature == NETDEV_XDP_ACT_NDO_XMIT {
                return b"\x1b[0;33mXDP_NDO_XMIT\x1b[0m\0".as_ptr() as *const c_char;
            }
            b"\0".as_ptr() as *const c_char
        }
    }
}

unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    match key {
        x if x == b'v' as c_int => env.verbosity = true,
        x if x == b't' as c_int => env.is_tester = true,
        x if x == b'f' as c_int => {
            if get_xdp_feature(arg) < 0 {
                fprintf(stderr, b"Invalid xdp feature: %s\n\0".as_ptr() as *const c_char, arg);
                argp_usage(state);
                return ARGP_ERR_UNKNOWN;
            }
        }
        x if x == b'D' as c_int => {
            if make_sockaddr(AF_INET6, arg, DUT_ECHO_PORT, &mut env.dut_addr, null_mut()) != 0 {
                fprintf(stderr, b"Invalid address assigned to the Device Under Test: %s\n\0".as_ptr() as *const c_char, arg);
                return ARGP_ERR_UNKNOWN;
            }
        }
        x if x == b'C' as c_int => {
            if make_sockaddr(AF_INET6, arg, DUT_CTRL_PORT, &mut env.dut_ctrl_addr, null_mut()) != 0 {
                fprintf(stderr, b"Invalid address assigned to the Device Under Test: %s\n\0".as_ptr() as *const c_char, arg);
                return ARGP_ERR_UNKNOWN;
            }
        }
        x if x == b'T' as c_int => {
            if make_sockaddr(AF_INET6, arg, 0, &mut env.tester_addr, null_mut()) != 0 {
                fprintf(stderr, b"Invalid address assigned to the Tester device: %s\n\0".as_ptr() as *const c_char, arg);
                return ARGP_ERR_UNKNOWN;
            }
        }
        ARGP_KEY_ARG => {
            errno = 0;
            if strlen(arg) >= IF_NAMESIZE {
                fprintf(stderr, b"Invalid device name: %s\n\0".as_ptr() as *const c_char, arg);
                argp_usage(state);
                return ARGP_ERR_UNKNOWN;
            }

            env.ifindex = if_nametoindex(arg) as c_int;
            if env.ifindex == 0 {
                env.ifindex = strtoul(arg, null_mut(), 0) as c_int;
            }
            if env.ifindex == 0 || if_indextoname(env.ifindex as c_uint, env.ifname.as_mut_ptr()).is_null() {
                fprintf(stderr, b"Bad interface index or name (%d): %s\n\0".as_ptr() as *const c_char, errno, strerror(errno));
                argp_usage(state);
                return ARGP_ERR_UNKNOWN;
            }
        }
        _ => return ARGP_ERR_UNKNOWN,
    }

    0
}

unsafe fn set_env_default() {
    env.feature.drv_feature = NETDEV_XDP_ACT_NDO_XMIT;
    env.feature.action = -EINVAL;
    env.ifindex = -ENODEV;
    strscpy(env.ifname.as_mut_ptr(), b"unknown\0".as_ptr() as *const c_char, IF_NAMESIZE);
    make_sockaddr(AF_INET6, b"::ffff:127.0.0.1\0".as_ptr() as *const c_char, DUT_CTRL_PORT, &mut env.dut_ctrl_addr, null_mut());
    make_sockaddr(AF_INET6, b"::ffff:127.0.0.1\0".as_ptr() as *const c_char, DUT_ECHO_PORT, &mut env.dut_addr, null_mut());
    make_sockaddr(AF_INET6, b"::ffff:127.0.0.1\0".as_ptr() as *const c_char, 0, &mut env.tester_addr, null_mut());
}

unsafe extern "C" fn dut_echo_thread(arg: *mut c_void) -> *mut c_void {
    let mut buf = [0u8; size_of::<tlv_hdr>()];
    let sockfd = *(arg as *mut c_int);

    while !exiting {
        let tlv = buf.as_mut_ptr() as *mut tlv_hdr;
        let mut addr: sockaddr_storage = zeroed();
        let mut addrlen: socklen_t = 0;
        let n = recvfrom(sockfd, buf.as_mut_ptr() as *mut c_void, buf.len(), MSG_WAITALL, &mut addr as *mut _ as *mut sockaddr, &mut addrlen);
        if n as usize != ntohs((*tlv).len) as usize {
            continue;
        }

        if ntohs((*tlv).type_) as c_int != CMD_ECHO {
            continue;
        }

        sendto(sockfd, buf.as_ptr() as *const c_void, buf.len(), MSG_NOSIGNAL | MSG_CONFIRM, &addr as *const _ as *const sockaddr, addrlen);
    }

    pthread_exit(null_mut());
}

unsafe fn dut_run_echo_thread(t: *mut pthread_t, mut sockfd: *mut c_int) -> c_int {
    let mut err: c_int;

    sockfd = start_reuseport_server(AF_INET6, SOCK_DGRAM, null(), DUT_ECHO_PORT, 0, 1);
    if sockfd.is_null() {
        fprintf(stderr, b"Failed creating data UDP socket on device %s\n\0".as_ptr() as *const c_char, env.ifname.as_ptr());
        return -errno;
    }

    /* start echo channel */
    err = pthread_create(t, null(), dut_echo_thread, sockfd as *mut c_void);
    if err != 0 {
        fprintf(stderr, b"Failed creating data UDP thread on device %s: %s\n\0".as_ptr() as *const c_char, env.ifname.as_ptr(), strerror(-err));
        free_fds(sockfd, 1);
        return -EINVAL;
    }

    0
}

unsafe fn dut_attach_xdp_prog(skel: *mut xdp_features, flags: c_int) -> c_int {
    let mut action = env.feature.action;
    let mut prog: *mut bpf_program;
    let key: c_uint = 0;
    let mut fd: c_int = 0;

    if env.feature.drv_feature == NETDEV_XDP_ACT_NDO_XMIT {
        let entry = bpf_devmap_val {
            ifindex: env.ifindex as c_uint,
        };

        let err = bpf_map__update_elem((*skel).maps.dev_map, &key as *const _ as *const c_void, size_of::<c_uint>(), &entry as *const _ as *const c_void, size_of::<bpf_devmap_val>(), 0);
        if err < 0 {
            return err;
        }

        fd = bpf_program__fd((*skel).progs.xdp_do_redirect_cpumap);
        action = XDP_REDIRECT;
    }

    match action {
        XDP_TX => prog = (*skel).progs.xdp_do_tx,
        XDP_DROP => prog = (*skel).progs.xdp_do_drop,
        XDP_ABORTED => prog = (*skel).progs.xdp_do_aborted,
        XDP_PASS => prog = (*skel).progs.xdp_do_pass,
        XDP_REDIRECT => {
            let entry = bpf_cpumap_val {
                qsize: 2048,
                bpf_prog: bpf_cpumap_val_bpf_prog { fd },
            };

            let err = bpf_map__update_elem((*skel).maps.cpu_map, &key as *const _ as *const c_void, size_of::<c_uint>(), &entry as *const _ as *const c_void, size_of::<bpf_cpumap_val>(), 0);
            if err < 0 {
                return err;
            }

            prog = (*skel).progs.xdp_do_redirect;
        }
        _ => return -EINVAL,
    }

    let err = bpf_xdp_attach(env.ifindex, bpf_program__fd(prog), flags, null());
    if err != 0 {
        fprintf(stderr, b"Failed attaching XDP program to device %s\n\0".as_ptr() as *const c_char, env.ifname.as_ptr());
    }
    err
}

unsafe fn recv_msg(sockfd: c_int, buf: *mut c_void, bufsize: size_t, val: *mut c_void, val_size: size_t) -> c_int {
    let tlv = buf as *mut tlv_hdr;
    let mut len = recv(sockfd, buf, bufsize, 0) as size_t;
    if len != ntohs((*tlv).len) as size_t || len < size_of::<tlv_hdr>() {
        return -EINVAL;
    }

    if !val.is_null() {
        len -= size_of::<tlv_hdr>();
        if len > val_size {
            return -ENOMEM;
        }

        memcpy(val, (*tlv).data.as_ptr() as *const c_void, len);
    }

    0
}

unsafe fn dut_run(skel: *mut xdp_features) -> c_int {
    let flags = XDP_FLAGS_UPDATE_IF_NOEXIST | XDP_FLAGS_DRV_MODE;
    let mut state: c_int = 0;
    let mut err: c_int = 0;
    let sockfd: *mut c_int;
    let ctrl_sockfd: c_int;
    let mut echo_sockfd: c_int = 0;
    let mut ctrl_addr: sockaddr_storage = zeroed();
    let mut dut_thread: pthread_t = 0;
    let mut addrlen: socklen_t = 0;

    sockfd = start_reuseport_server(AF_INET6, SOCK_STREAM, null(), DUT_CTRL_PORT, 0, 1);
    if sockfd.is_null() {
        fprintf(stderr, b"Failed creating control socket on device %s\n\0".as_ptr() as *const c_char, env.ifname.as_ptr());
        return -errno;
    }

    ctrl_sockfd = accept(*sockfd, &mut ctrl_addr as *mut _ as *mut sockaddr, &mut addrlen);
    if ctrl_sockfd < 0 {
        fprintf(stderr, b"Failed accepting connections on device %s control socket\n\0".as_ptr() as *const c_char, env.ifname.as_ptr());
        free_fds(sockfd, 1);
        return -errno;
    }

    /* CTRL loop */
    while !exiting {
        let mut buf = [0u8; BUFSIZE];
        let tlv = buf.as_mut_ptr() as *mut tlv_hdr;

        err = recv_msg(ctrl_sockfd, buf.as_mut_ptr() as *mut c_void, BUFSIZE, null_mut(), 0);
        if err != 0 {
            continue;
        }

        match ntohs((*tlv).type_) as c_int {
            CMD_START => {
                if state == CMD_START {
                    continue;
                }

                state = CMD_START;
                /* Load the XDP program on the DUT */
                err = dut_attach_xdp_prog(skel, flags);
                if err != 0 {
                    break;
                }

                err = dut_run_echo_thread(&mut dut_thread, &mut echo_sockfd);
                if err < 0 {
                    break;
                }

                (*tlv).type_ = htons(CMD_ACK as u16);
                (*tlv).len = htons(size_of::<tlv_hdr>() as u16);
                err = send(ctrl_sockfd, buf.as_ptr() as *const c_void, size_of::<tlv_hdr>(), 0);
                if err < 0 {
                    pthread_join(dut_thread, null_mut());
                    bpf_xdp_detach(env.ifindex, flags, null());
                    close(ctrl_sockfd);
                    free_fds(sockfd, 1);
                    return err;
                }
            }
            CMD_STOP => {
                if state != CMD_START {
                    continue;
                }

                state = CMD_STOP;

                exiting = true;
                bpf_xdp_detach(env.ifindex, flags, null());

                (*tlv).type_ = htons(CMD_ACK as u16);
                (*tlv).len = htons(size_of::<tlv_hdr>() as u16);
                err = send(ctrl_sockfd, buf.as_ptr() as *const c_void, size_of::<tlv_hdr>(), 0);
                pthread_join(dut_thread, null_mut());
                bpf_xdp_detach(env.ifindex, flags, null());
                close(ctrl_sockfd);
                free_fds(sockfd, 1);
                return err;
            }
            CMD_GET_XDP_CAP => {
                let mut opts = bpf_xdp_query_opts {
                    sz: size_of::<bpf_xdp_query_opts>(),
                    feature_flags: 0,
                };
                let val: u64;
                let n: size_t;

                err = bpf_xdp_query(env.ifindex, XDP_FLAGS_DRV_MODE, &mut opts);
                if err != 0 {
                    fprintf(stderr, b"Failed querying XDP cap for device %s\n\0".as_ptr() as *const c_char, env.ifname.as_ptr());
                    pthread_join(dut_thread, null_mut());
                    bpf_xdp_detach(env.ifindex, flags, null());
                    close(ctrl_sockfd);
                    free_fds(sockfd, 1);
                    return err;
                }

                (*tlv).type_ = htons(CMD_ACK as u16);
                n = size_of::<tlv_hdr>() + size_of::<u64>();
                (*tlv).len = htons(n as u16);

                val = htobe64(opts.feature_flags);
                memcpy((*tlv).data.as_mut_ptr() as *mut c_void, &val as *const _ as *const c_void, size_of::<u64>());

                err = send(ctrl_sockfd, buf.as_ptr() as *const c_void, n, 0);
                if err < 0 {
                    pthread_join(dut_thread, null_mut());
                    bpf_xdp_detach(env.ifindex, flags, null());
                    close(ctrl_sockfd);
                    free_fds(sockfd, 1);
                    return err;
                }
            }
            CMD_GET_STATS => {
                let key: c_uint = 0;
                let mut val: c_uint = 0;
                let n: size_t;

                err = bpf_map__lookup_elem((*skel).maps.dut_stats, &key as *const _ as *const c_void, size_of::<c_uint>(), &mut val as *mut _ as *mut c_void, size_of::<c_uint>(), 0);
                if err != 0 {
                    fprintf(stderr, b"bpf_map_lookup_elem failed (%d)\n\0".as_ptr() as *const c_char, err);
                    pthread_join(dut_thread, null_mut());
                    bpf_xdp_detach(env.ifindex, flags, null());
                    close(ctrl_sockfd);
                    free_fds(sockfd, 1);
                    return err;
                }

                (*tlv).type_ = htons(CMD_ACK as u16);
                n = size_of::<tlv_hdr>() + size_of::<c_uint>();
                (*tlv).len = htons(n as u16);

                val = htonl(val);
                memcpy((*tlv).data.as_mut_ptr() as *mut c_void, &val as *const _ as *const c_void, size_of::<c_uint>());

                err = send(ctrl_sockfd, buf.as_ptr() as *const c_void, n, 0);
                if err < 0 {
                    pthread_join(dut_thread, null_mut());
                    bpf_xdp_detach(env.ifindex, flags, null());
                    close(ctrl_sockfd);
                    free_fds(sockfd, 1);
                    return err;
                }
            }
            _ => {}
        }
    }

    pthread_join(dut_thread, null_mut());
    bpf_xdp_detach(env.ifindex, flags, null());
    close(ctrl_sockfd);
    free_fds(sockfd, 1);

    err
}

unsafe fn tester_collect_detected_cap(skel: *mut xdp_features, dut_stats: c_uint) -> bool {
    let key: c_uint = 0;
    let mut val: c_uint = 0;

    if dut_stats == 0 {
        return false;
    }

    let err = bpf_map__lookup_elem((*skel).maps.stats, &key as *const _ as *const c_void, size_of::<c_uint>(), &mut val as *mut _ as *mut c_void, size_of::<c_uint>(), 0);
    if err != 0 {
        fprintf(stderr, b"bpf_map_lookup_elem failed (%d)\n\0".as_ptr() as *const c_char, err);
        return false;
    }

    match env.feature.action {
        XDP_PASS | XDP_TX | XDP_REDIRECT => return val > 0,
        XDP_DROP | XDP_ABORTED => return val == 0,
        _ => {}
    }

    if env.feature.drv_feature == NETDEV_XDP_ACT_NDO_XMIT {
        return val > 0;
    }

    false
}

unsafe fn send_and_recv_msg(sockfd: c_int, cmd: test_commands, val: *mut c_void, val_size: size_t) -> c_int {
    let mut buf = [0u8; BUFSIZE];
    let tlv = buf.as_mut_ptr() as *mut tlv_hdr;
    let mut err: c_int;

    (*tlv).type_ = htons(cmd as u16);
    (*tlv).len = htons(size_of::<tlv_hdr>() as u16);

    err = send(sockfd, buf.as_ptr() as *const c_void, size_of::<tlv_hdr>(), 0);
    if err < 0 {
        return err;
    }

    err = recv_msg(sockfd, buf.as_mut_ptr() as *mut c_void, BUFSIZE, val, val_size);
    if err < 0 {
        return err;
    }

    if ntohs((*tlv).type_) as c_int == CMD_ACK { 0 } else { -EINVAL }
}

unsafe fn send_echo_msg() -> c_int {
    let mut buf = [0u8; size_of::<tlv_hdr>()];
    let tlv = buf.as_mut_ptr() as *mut tlv_hdr;
    let sockfd: c_int;
    let n: c_int;

    sockfd = socket(AF_INET6, SOCK_DGRAM, 0);
    if sockfd < 0 {
        fprintf(stderr, b"Failed creating data UDP socket on device %s\n\0".as_ptr() as *const c_char, env.ifname.as_ptr());
        return -errno;
    }

    (*tlv).type_ = htons(CMD_ECHO as u16);
    (*tlv).len = htons(size_of::<tlv_hdr>() as u16);

    n = sendto(sockfd, buf.as_ptr() as *const c_void, size_of::<tlv_hdr>(), MSG_NOSIGNAL | MSG_CONFIRM, &env.dut_addr as *const _ as *const sockaddr, size_of::<sockaddr_storage>() as socklen_t) as c_int;
    close(sockfd);

    if n == ntohs((*tlv).len) as c_int { 0 } else { -EINVAL }
}

unsafe fn tester_run(skel: *mut xdp_features) -> c_int {
    let flags = XDP_FLAGS_UPDATE_IF_NOEXIST | XDP_FLAGS_DRV_MODE;
    let mut advertised_feature: u64 = 0;
    let prog: *mut bpf_program;
    let mut stats: c_uint = 0;
    let mut err: c_int;
    let sockfd: c_int;
    let detected_cap: bool;

    sockfd = socket(AF_INET6, SOCK_STREAM, 0);
    if sockfd < 0 {
        fprintf(stderr, b"Failed creating tester service control socket\n\0".as_ptr() as *const c_char);
        return -errno;
    }

    if settimeo(sockfd, 1000) < 0 {
        return -EINVAL;
    }

    err = connect(sockfd, &env.dut_ctrl_addr as *const _ as *const sockaddr, size_of::<sockaddr_storage>() as socklen_t);
    if err != 0 {
        fprintf(stderr, b"Failed connecting to the Device Under Test control socket\n\0".as_ptr() as *const c_char);
        return -errno;
    }

    err = send_and_recv_msg(sockfd, CMD_GET_XDP_CAP, &mut advertised_feature as *mut _ as *mut c_void, size_of::<u64>());
    if err < 0 {
        close(sockfd);
        return err;
    }

    advertised_feature = be64toh(advertised_feature);

    if env.feature.drv_feature == NETDEV_XDP_ACT_NDO_XMIT || env.feature.action == XDP_TX {
        prog = (*skel).progs.xdp_tester_check_tx;
    } else {
        prog = (*skel).progs.xdp_tester_check_rx;
    }

    err = bpf_xdp_attach(env.ifindex, bpf_program__fd(prog), flags, null());
    if err != 0 {
        fprintf(stderr, b"Failed attaching XDP program to device %s\n\0".as_ptr() as *const c_char, env.ifname.as_ptr());
        bpf_xdp_detach(env.ifindex, flags, null());
        close(sockfd);
        return if err < 0 { err } else { 0 };
    }

    err = send_and_recv_msg(sockfd, CMD_START, null_mut(), 0);
    if err != 0 {
        bpf_xdp_detach(env.ifindex, flags, null());
        close(sockfd);
        return if err < 0 { err } else { 0 };
    }

    let mut i = 0;
    while i < 10 && !exiting {
        err = send_echo_msg();
        if err < 0 {
            bpf_xdp_detach(env.ifindex, flags, null());
            close(sockfd);
            return err;
        }

        sleep(1);
        i += 1;
    }

    err = send_and_recv_msg(sockfd, CMD_GET_STATS, &mut stats as *mut _ as *mut c_void, size_of::<c_uint>());
    if err != 0 {
        bpf_xdp_detach(env.ifindex, flags, null());
        close(sockfd);
        return if err < 0 { err } else { 0 };
    }

    /* stop the test */
    err = send_and_recv_msg(sockfd, CMD_STOP, null_mut(), 0);
    /* send a new echo message to wake echo thread of the dut */
    send_echo_msg();

    detected_cap = tester_collect_detected_cap(skel, ntohl(stats));

    fprintf(stdout, b"Feature %s: [%s][%s]\n\0".as_ptr() as *const c_char,
        get_xdp_feature_str(),
        if detected_cap { b"\x1b[0;32mDETECTED\x1b[0m\0".as_ptr() as *const c_char } else { b"\x1b[0;31mNOT DETECTED\x1b[0m\0".as_ptr() as *const c_char },
        if (env.feature.drv_feature & advertised_feature as netdev_xdp_act) != 0 { b"\x1b[0;32mADVERTISED\x1b[0m\0".as_ptr() as *const c_char } else { b"\x1b[0;31mNOT ADVERTISED\x1b[0m\0".as_ptr() as *const c_char });

    bpf_xdp_detach(env.ifindex, flags, null());
    close(sockfd);
    if err < 0 { err } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let skel: *mut xdp_features;
    let mut err: c_int;

    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    libbpf_set_print(libbpf_print_fn);

    signal(SIGINT, sig_handler);
    signal(SIGTERM, sig_handler);

    set_env_default();

    /* Parse command line arguments */
    err = argp_parse(&ARGP, argc, argv, 0, null_mut(), null_mut());
    if err != 0 {
        return err;
    }

    if env.ifindex < 0 {
        fprintf(stderr, b"Invalid device name %s\n\0".as_ptr() as *const c_char, env.ifname.as_ptr());
        return -ENODEV;
    }

    /* Load and verify BPF application */
    skel = xdp_features__open();
    if skel.is_null() {
        fprintf(stderr, b"Failed to open and load BPF skeleton\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    (*(*skel).rodata).tester_addr = (*( &env.tester_addr as *const _ as *const sockaddr_in6)).sin6_addr;
    (*(*skel).rodata).dut_addr = (*( &env.dut_addr as *const _ as *const sockaddr_in6)).sin6_addr;

    /* Load & verify BPF programs */
    err = xdp_features__load(skel);
    if err != 0 {
        fprintf(stderr, b"Failed to load and verify BPF skeleton\n\0".as_ptr() as *const c_char);
        xdp_features__destroy(skel);
        return if err < 0 { -err } else { 0 };
    }

    err = xdp_features__attach(skel);
    if err != 0 {
        fprintf(stderr, b"Failed to attach BPF skeleton\n\0".as_ptr() as *const c_char);
        xdp_features__destroy(skel);
        return if err < 0 { -err } else { 0 };
    }

    if env.is_tester {
        /* Tester */
        fprintf(stdout, b"Starting tester service on device %s\n\0".as_ptr() as *const c_char, env.ifname.as_ptr());
        err = tester_run(skel);
    } else {
        /* DUT */
        fprintf(stdout, b"Starting test on device %s\n\0".as_ptr() as *const c_char, env.ifname.as_ptr());
        err = dut_run(skel);
    }

    xdp_features__destroy(skel);

    if err < 0 { -err } else { 0 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
