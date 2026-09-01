// Translated from testing/selftests/net/rxtimestamp.c.
// C include dependencies: errno.h, error.h, getopt.h, stdbool.h, stdio.h,
// stdlib.h, string.h, unistd.h, sys/time.h, sys/socket.h, sys/select.h,
// sys/ioctl.h, arpa/inet.h, net/if.h, asm/types.h, linux/net_tstamp.h,
// linux/errqueue.h, kselftest.h.

use libc::*;
use std::ffi::CString;
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct options {
    pub so_timestamp: c_int,
    pub so_timestampns: c_int,
    pub so_timestamping: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstamps {
    pub tstamp: bool,
    pub tstampns: bool,
    pub swtstamp: bool,
    pub hwtstamp: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket_type {
    pub friendly_name: *mut c_char,
    pub type_: c_int,
    pub protocol: c_int,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_case {
    pub sockopt: options,
    pub expected: tstamps,
    pub enabled: bool,
    pub warn_on_fail: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_flag {
    pub mask: c_int,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct scm_timestamping {
    pub ts: [timespec; 3],
}

#[repr(C)]
union sockaddr_any {
    addr6: sockaddr_in6,
    addr4: sockaddr_in,
    addr_un: sockaddr,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static in6addr_loopback: in6_addr;

    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn htonl(hostlong: u32) -> u32;
    fn htons(hostshort: u16) -> u16;
}

const SOF_TIMESTAMPING_SOFTWARE: c_int = 1 << 4;
const SOF_TIMESTAMPING_RX_SOFTWARE: c_int = 1 << 3;
const SOF_TIMESTAMPING_RX_HARDWARE: c_int = 1 << 2;
const SOF_TIMESTAMPING_OPT_RX_FILTER: c_int = 1 << 17;
const SOF_TIMESTAMPING_RAW_HARDWARE: c_int = 1 << 6;

const SO_TIMESTAMP: c_int = 29;
const SO_TIMESTAMPNS: c_int = 35;
const SO_TIMESTAMPING: c_int = 37;
const SCM_TIMESTAMP: c_int = SO_TIMESTAMP;
const SCM_TIMESTAMPNS: c_int = SO_TIMESTAMPNS;
const SCM_TIMESTAMPING: c_int = SO_TIMESTAMPING;
const IPPROTO_EGP: c_int = 8;
const INADDR_LOOPBACK: u32 = 0x7f000001;

const fn opt(
    so_timestamp: c_int,
    so_timestampns: c_int,
    so_timestamping: c_int,
) -> options {
    options {
        so_timestamp,
        so_timestampns,
        so_timestamping,
    }
}

const fn ts(tstamp: bool, tstampns: bool, swtstamp: bool, hwtstamp: bool) -> tstamps {
    tstamps {
        tstamp,
        tstampns,
        swtstamp,
        hwtstamp,
    }
}

static mut sof_flags: [sof_flag; 5] = [
    sof_flag {
        mask: SOF_TIMESTAMPING_SOFTWARE,
        name: b"SOF_TIMESTAMPING_SOFTWARE\0".as_ptr() as *mut c_char,
    },
    sof_flag {
        mask: SOF_TIMESTAMPING_RX_SOFTWARE,
        name: b"SOF_TIMESTAMPING_RX_SOFTWARE\0".as_ptr() as *mut c_char,
    },
    sof_flag {
        mask: SOF_TIMESTAMPING_RX_HARDWARE,
        name: b"SOF_TIMESTAMPING_RX_HARDWARE\0".as_ptr() as *mut c_char,
    },
    sof_flag {
        mask: SOF_TIMESTAMPING_OPT_RX_FILTER,
        name: b"SOF_TIMESTAMPING_OPT_RX_FILTER\0".as_ptr() as *mut c_char,
    },
    sof_flag {
        mask: SOF_TIMESTAMPING_RAW_HARDWARE,
        name: b"SOF_TIMESTAMPING_RAW_HARDWARE\0".as_ptr() as *mut c_char,
    },
];

static mut socket_types: [socket_type; 3] = [
    socket_type {
        friendly_name: b"ip\0".as_ptr() as *mut c_char,
        type_: SOCK_RAW,
        protocol: IPPROTO_EGP,
        enabled: false,
    },
    socket_type {
        friendly_name: b"udp\0".as_ptr() as *mut c_char,
        type_: SOCK_DGRAM,
        protocol: IPPROTO_UDP,
        enabled: false,
    },
    socket_type {
        friendly_name: b"tcp\0".as_ptr() as *mut c_char,
        type_: SOCK_STREAM,
        protocol: IPPROTO_TCP,
        enabled: false,
    },
];

static mut test_cases: [test_case; 13] = [
    test_case {
        sockopt: opt(0, 0, 0),
        expected: ts(false, false, false, false),
        enabled: false,
        warn_on_fail: false,
    },
    test_case {
        sockopt: opt(1, 0, 0),
        expected: ts(true, false, false, false),
        enabled: false,
        warn_on_fail: false,
    },
    test_case {
        sockopt: opt(0, 1, 0),
        expected: ts(false, true, false, false),
        enabled: false,
        warn_on_fail: false,
    },
    test_case {
        sockopt: opt(1, 1, 0),
        expected: ts(false, true, false, false),
        enabled: false,
        warn_on_fail: false,
    },
    test_case {
        sockopt: opt(0, 0, SOF_TIMESTAMPING_RX_SOFTWARE),
        expected: ts(false, false, false, false),
        enabled: false,
        warn_on_fail: false,
    },
    test_case {
        /* Loopback device does not support hw timestamps. */
        sockopt: opt(0, 0, SOF_TIMESTAMPING_RX_HARDWARE),
        expected: ts(false, false, false, false),
        enabled: false,
        warn_on_fail: false,
    },
    test_case {
        sockopt: opt(0, 0, SOF_TIMESTAMPING_SOFTWARE),
        expected: ts(false, false, false, false),
        enabled: false,
        warn_on_fail: true,
    },
    test_case {
        sockopt: opt(
            0,
            0,
            SOF_TIMESTAMPING_RX_SOFTWARE | SOF_TIMESTAMPING_RX_HARDWARE,
        ),
        expected: ts(false, false, false, false),
        enabled: false,
        warn_on_fail: false,
    },
    test_case {
        sockopt: opt(
            0,
            0,
            SOF_TIMESTAMPING_RAW_HARDWARE | SOF_TIMESTAMPING_OPT_RX_FILTER,
        ),
        expected: ts(false, false, false, false),
        enabled: false,
        warn_on_fail: false,
    },
    test_case {
        sockopt: opt(
            0,
            0,
            SOF_TIMESTAMPING_SOFTWARE | SOF_TIMESTAMPING_OPT_RX_FILTER,
        ),
        expected: ts(false, false, false, false),
        enabled: false,
        warn_on_fail: false,
    },
    test_case {
        sockopt: opt(
            0,
            0,
            SOF_TIMESTAMPING_SOFTWARE
                | SOF_TIMESTAMPING_RX_SOFTWARE
                | SOF_TIMESTAMPING_OPT_RX_FILTER,
        ),
        expected: ts(false, false, true, false),
        enabled: false,
        warn_on_fail: false,
    },
    test_case {
        sockopt: opt(
            0,
            0,
            SOF_TIMESTAMPING_SOFTWARE | SOF_TIMESTAMPING_RX_SOFTWARE,
        ),
        expected: ts(false, false, true, false),
        enabled: false,
        warn_on_fail: false,
    },
    test_case {
        sockopt: opt(
            1,
            0,
            SOF_TIMESTAMPING_SOFTWARE | SOF_TIMESTAMPING_RX_SOFTWARE,
        ),
        expected: ts(true, false, true, false),
        enabled: false,
        warn_on_fail: false,
    },
];

static mut long_options: [option; 10] = [
    option {
        name: b"list_tests\0".as_ptr() as *const c_char,
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: 'l' as c_int,
    },
    option {
        name: b"test_num\0".as_ptr() as *const c_char,
        has_arg: required_argument,
        flag: ptr::null_mut(),
        val: 'n' as c_int,
    },
    option {
        name: b"op_size\0".as_ptr() as *const c_char,
        has_arg: required_argument,
        flag: ptr::null_mut(),
        val: 's' as c_int,
    },
    option {
        name: b"tcp\0".as_ptr() as *const c_char,
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: 't' as c_int,
    },
    option {
        name: b"udp\0".as_ptr() as *const c_char,
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: 'u' as c_int,
    },
    option {
        name: b"ip\0".as_ptr() as *const c_char,
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: 'i' as c_int,
    },
    option {
        name: b"strict\0".as_ptr() as *const c_char,
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: 'S' as c_int,
    },
    option {
        name: b"ipv4\0".as_ptr() as *const c_char,
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: '4' as c_int,
    },
    option {
        name: b"ipv6\0".as_ptr() as *const c_char,
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: '6' as c_int,
    },
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

static mut next_port: c_int = 19999;
static mut op_size: c_int = 10 * 1024;

unsafe fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

pub unsafe fn print_test_case(t: *mut test_case) {
    let mut f: c_int = 0;

    printf(cstr("sockopts {").as_ptr());
    if (*t).sockopt.so_timestamp != 0 {
        printf(cstr(" SO_TIMESTAMP ").as_ptr());
    }
    if (*t).sockopt.so_timestampns != 0 {
        printf(cstr(" SO_TIMESTAMPNS ").as_ptr());
    }
    if (*t).sockopt.so_timestamping != 0 {
        printf(cstr(" SO_TIMESTAMPING: {").as_ptr());
        while f < sof_flags.len() as c_int {
            if ((*t).sockopt.so_timestamping & sof_flags[f as usize].mask) != 0 {
                printf(cstr(" %s |").as_ptr(), sof_flags[f as usize].name);
            }
            f += 1;
        }
        printf(cstr("}").as_ptr());
    }
    printf(cstr("} expected cmsgs: {").as_ptr());
    if (*t).expected.tstamp {
        printf(cstr(" SCM_TIMESTAMP ").as_ptr());
    }
    if (*t).expected.tstampns {
        printf(cstr(" SCM_TIMESTAMPNS ").as_ptr());
    }
    if (*t).expected.swtstamp || (*t).expected.hwtstamp {
        printf(cstr(" SCM_TIMESTAMPING {").as_ptr());
        if (*t).expected.swtstamp {
            printf(cstr("0").as_ptr());
        }
        if (*t).expected.swtstamp && (*t).expected.hwtstamp {
            printf(cstr(",").as_ptr());
        }
        if (*t).expected.hwtstamp {
            printf(cstr("2").as_ptr());
        }
        printf(cstr("}").as_ptr());
    }
    printf(cstr("}\n").as_ptr());
}

pub unsafe fn do_send(src: c_int) {
    let mut r: c_int;
    let buf = malloc(op_size as size_t) as *mut c_char;

    memset(buf as *mut c_void, 'z' as c_int, op_size as size_t);
    r = write(src, buf as *const c_void, op_size as size_t) as c_int;
    if r < 0 {
        error(1, *__errno_location(), cstr("Failed to sendmsg").as_ptr());
    }

    free(buf as *mut c_void);
}

unsafe fn cmsg_align(len: size_t) -> size_t {
    (len + mem::size_of::<size_t>() - 1) & !(mem::size_of::<size_t>() - 1)
}

unsafe fn cmsg_firsthdr(mhdr: *const msghdr) -> *mut cmsghdr {
    if (*mhdr).msg_controllen < mem::size_of::<cmsghdr>() as size_t {
        ptr::null_mut()
    } else {
        (*mhdr).msg_control as *mut cmsghdr
    }
}

unsafe fn cmsg_nxthdr(mhdr: *const msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr {
    let next = (cmsg as *mut u8).add(cmsg_align((*cmsg).cmsg_len as size_t)) as *mut cmsghdr;
    let max = ((*mhdr).msg_control as *mut u8).add((*mhdr).msg_controllen as usize);
    if (next as *mut u8).add(mem::size_of::<cmsghdr>()) > max {
        ptr::null_mut()
    } else {
        next
    }
}

unsafe fn cmsg_data(cmsg: *mut cmsghdr) -> *mut c_uchar {
    (cmsg as *mut u8).add(cmsg_align(mem::size_of::<cmsghdr>() as size_t)) as *mut c_uchar
}

unsafe fn validate(field_name: &str, expected_field: bool, actual_field: bool, failed: *mut bool) {
    if expected_field != actual_field {
        if expected_field {
            error(
                0,
                0,
                cstr(&format!("Expected {} to be set.", field_name)).as_ptr(),
            );
        } else {
            error(
                0,
                0,
                cstr(&format!("Expected {} to not be set.", field_name)).as_ptr(),
            );
        }
        *failed = true;
    }
}

pub unsafe fn do_recv(rcv: c_int, read_size: c_int, expected: tstamps) -> bool {
    const CMSG_SIZE: usize = 1024;

    let mut ts: *mut scm_timestamping;
    let mut actual: tstamps = mem::zeroed();
    let mut cmsg_buf: [c_char; CMSG_SIZE] = [0; CMSG_SIZE];
    let mut recv_iov: iovec = mem::zeroed();
    let mut cmsg: *mut cmsghdr;
    let mut failed = false;
    let mut hdr: msghdr = mem::zeroed();
    let flags: c_int = 0;
    let mut r: c_int;

    memset(
        &mut hdr as *mut msghdr as *mut c_void,
        0,
        mem::size_of::<msghdr>(),
    );
    hdr.msg_iov = &mut recv_iov;
    hdr.msg_iovlen = 1;
    recv_iov.iov_base = malloc(read_size as size_t);
    recv_iov.iov_len = read_size as size_t;

    hdr.msg_control = cmsg_buf.as_mut_ptr() as *mut c_void;
    hdr.msg_controllen = mem::size_of_val(&cmsg_buf) as size_t;

    r = recvmsg(rcv, &mut hdr, flags) as c_int;
    if r < 0 {
        error(1, *__errno_location(), cstr("Failed to recvmsg").as_ptr());
    }
    if r != read_size {
        error(
            1,
            0,
            cstr("Only received %d bytes of payload.").as_ptr(),
            r,
        );
    }

    if (hdr.msg_flags & (MSG_TRUNC | MSG_CTRUNC)) != 0 {
        error(1, 0, cstr("Message was truncated.").as_ptr());
    }

    cmsg = cmsg_firsthdr(&hdr);
    while !cmsg.is_null() {
        if (*cmsg).cmsg_level != SOL_SOCKET {
            error(
                1,
                0,
                cstr("Unexpected cmsg_level %d").as_ptr(),
                (*cmsg).cmsg_level,
            );
        }
        match (*cmsg).cmsg_type {
            SCM_TIMESTAMP => {
                actual.tstamp = true;
            }
            SCM_TIMESTAMPNS => {
                actual.tstampns = true;
            }
            SCM_TIMESTAMPING => {
                ts = cmsg_data(cmsg) as *mut scm_timestamping;
                actual.swtstamp = (*ts).ts[0].tv_sec != 0;
                if (*ts).ts[1].tv_sec != 0 {
                    error(0, 0, cstr("ts[1] should not be set.").as_ptr());
                }
                actual.hwtstamp = (*ts).ts[2].tv_sec != 0;
            }
            _ => {
                error(
                    1,
                    0,
                    cstr("Unexpected cmsg_type %d").as_ptr(),
                    (*cmsg).cmsg_type,
                );
            }
        }
        cmsg = cmsg_nxthdr(&hdr, cmsg);
    }

    validate("tstamp", expected.tstamp, actual.tstamp, &mut failed);
    validate("tstampns", expected.tstampns, actual.tstampns, &mut failed);
    validate("swtstamp", expected.swtstamp, actual.swtstamp, &mut failed);
    validate("hwtstamp", expected.hwtstamp, actual.hwtstamp, &mut failed);

    free(recv_iov.iov_base);

    failed
}

pub unsafe fn config_so_flags(rcv: c_int, o: options) {
    let on: c_int = 1;

    if setsockopt(
        rcv,
        SOL_SOCKET,
        SO_REUSEADDR,
        &on as *const c_int as *const c_void,
        mem::size_of_val(&on) as socklen_t,
    ) < 0
    {
        error(
            1,
            *__errno_location(),
            cstr("Failed to enable SO_REUSEADDR").as_ptr(),
        );
    }

    if o.so_timestamp != 0
        && setsockopt(
            rcv,
            SOL_SOCKET,
            SO_TIMESTAMP,
            &o.so_timestamp as *const c_int as *const c_void,
            mem::size_of_val(&o.so_timestamp) as socklen_t,
        ) < 0
    {
        error(
            1,
            *__errno_location(),
            cstr("Failed to enable SO_TIMESTAMP").as_ptr(),
        );
    }

    if o.so_timestampns != 0
        && setsockopt(
            rcv,
            SOL_SOCKET,
            SO_TIMESTAMPNS,
            &o.so_timestampns as *const c_int as *const c_void,
            mem::size_of_val(&o.so_timestampns) as socklen_t,
        ) < 0
    {
        error(
            1,
            *__errno_location(),
            cstr("Failed to enable SO_TIMESTAMPNS").as_ptr(),
        );
    }

    if o.so_timestamping != 0
        && setsockopt(
            rcv,
            SOL_SOCKET,
            SO_TIMESTAMPING,
            &o.so_timestamping as *const c_int as *const c_void,
            mem::size_of_val(&o.so_timestamping) as socklen_t,
        ) < 0
    {
        error(
            1,
            *__errno_location(),
            cstr("Failed to set SO_TIMESTAMPING").as_ptr(),
        );
    }
}

pub unsafe fn run_test_case(
    s: *mut socket_type,
    test_num: c_int,
    ip_version: c_char,
    strict: bool,
) -> bool {
    let mut addr: sockaddr_any = mem::zeroed();
    let mut read_size = op_size;
    let mut src: c_int;
    let mut dst: c_int;
    let mut rcv: c_int;
    let port: c_int;
    let mut addr_size: socklen_t;
    let mut failed = false;

    port = if (*s).type_ == SOCK_RAW {
        0
    } else {
        let p = next_port;
        next_port += 1;
        p
    };
    memset(
        &mut addr as *mut sockaddr_any as *mut c_void,
        0,
        mem::size_of::<sockaddr_any>(),
    );
    if ip_version == '4' as c_char {
        addr.addr4.sin_family = AF_INET as sa_family_t;
        addr.addr4.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
        addr.addr4.sin_port = htons(port as u16);
        addr_size = mem::size_of::<sockaddr_in>() as socklen_t;
        if (*s).type_ == SOCK_RAW {
            read_size += 20; /* for IPv4 header */
        }
    } else {
        addr.addr6.sin6_family = AF_INET6 as sa_family_t;
        addr.addr6.sin6_addr = in6addr_loopback;
        addr.addr6.sin6_port = htons(port as u16);
        addr_size = mem::size_of::<sockaddr_in6>() as socklen_t;
    }
    printf(
        cstr("Starting testcase %d over ipv%c...\n").as_ptr(),
        test_num,
        ip_version as c_int,
    );
    src = socket(addr.addr_un.sa_family as c_int, (*s).type_, (*s).protocol);
    if src < 0 {
        error(
            1,
            *__errno_location(),
            cstr("Failed to open src socket").as_ptr(),
        );
    }

    dst = socket(addr.addr_un.sa_family as c_int, (*s).type_, (*s).protocol);
    if dst < 0 {
        error(
            1,
            *__errno_location(),
            cstr("Failed to open dst socket").as_ptr(),
        );
    }

    if bind(dst, &addr.addr_un as *const sockaddr, addr_size) < 0 {
        error(
            1,
            *__errno_location(),
            cstr("Failed to bind to port %d").as_ptr(),
            port,
        );
    }

    if (*s).type_ == SOCK_STREAM && listen(dst, 1) < 0 {
        error(1, *__errno_location(), cstr("Failed to listen").as_ptr());
    }

    if connect(src, &addr.addr_un as *const sockaddr, addr_size) < 0 {
        error(
            1,
            *__errno_location(),
            cstr("Failed to connect").as_ptr(),
        );
    }

    if (*s).type_ == SOCK_STREAM {
        rcv = accept(dst, ptr::null_mut(), ptr::null_mut());
        if rcv < 0 {
            error(1, *__errno_location(), cstr("Failed to accept").as_ptr());
        }
        close(dst);
    } else {
        rcv = dst;
    }

    config_so_flags(rcv, test_cases[test_num as usize].sockopt);
    usleep(20000); /* setsockopt for SO_TIMESTAMPING is asynchronous */
    do_send(src);

    failed = do_recv(
        rcv,
        read_size,
        test_cases[test_num as usize].expected,
    );

    close(rcv);
    close(src);

    if failed {
        printf(
            cstr("FAILURE in testcase %d over ipv%c ").as_ptr(),
            test_num,
            ip_version as c_int,
        );
        print_test_case(&mut test_cases[test_num as usize]);
        if !strict && test_cases[test_num as usize].warn_on_fail {
            failed = false;
        }
    }
    failed
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut all_protocols = true;
    let mut all_tests = true;
    let mut cfg_ipv4 = false;
    let mut cfg_ipv6 = false;
    let mut strict = false;
    let mut arg_index: c_int = 0;
    let mut failures: c_int = 0;
    let mut s: c_int;
    let mut t: c_int;
    let mut opt_ch: c_int;

    loop {
        opt_ch = getopt_long(
            argc,
            argv,
            cstr("").as_ptr(),
            long_options.as_ptr(),
            &mut arg_index,
        );
        if opt_ch == -1 {
            break;
        }
        match opt_ch {
            x if x == 'l' as c_int => {
                t = 0;
                while t < test_cases.len() as c_int {
                    printf(cstr("%d\t").as_ptr(), t);
                    print_test_case(&mut test_cases[t as usize]);
                    t += 1;
                }
                return 0;
            }
            x if x == 'n' as c_int => {
                t = atoi(optarg);
                if t >= test_cases.len() as c_int {
                    error(1, 0, cstr("Invalid test case: %d").as_ptr(), t);
                }
                all_tests = false;
                test_cases[t as usize].enabled = true;
            }
            x if x == 's' as c_int => {
                op_size = atoi(optarg);
            }
            x if x == 't' as c_int => {
                all_protocols = false;
                socket_types[2].enabled = true;
            }
            x if x == 'u' as c_int => {
                all_protocols = false;
                socket_types[1].enabled = true;
            }
            x if x == 'i' as c_int => {
                all_protocols = false;
                socket_types[0].enabled = true;
            }
            x if x == 'S' as c_int => {
                strict = true;
            }
            x if x == '4' as c_int => {
                cfg_ipv4 = true;
            }
            x if x == '6' as c_int => {
                cfg_ipv6 = true;
            }
            _ => {
                error(1, 0, cstr("Failed to parse parameters.").as_ptr());
            }
        }
    }

    s = 0;
    while s < socket_types.len() as c_int {
        if !all_protocols && !socket_types[s as usize].enabled {
            s += 1;
            continue;
        }

        printf(
            cstr("Testing %s...\n").as_ptr(),
            socket_types[s as usize].friendly_name,
        );
        t = 0;
        while t < test_cases.len() as c_int {
            if !all_tests && !test_cases[t as usize].enabled {
                t += 1;
                continue;
            }
            if cfg_ipv4 || !cfg_ipv6 {
                if run_test_case(&mut socket_types[s as usize], t, '4' as c_char, strict) {
                    failures += 1;
                }
            }
            if cfg_ipv6 || !cfg_ipv4 {
                if run_test_case(&mut socket_types[s as usize], t, '6' as c_char, strict) {
                    failures += 1;
                }
            }
            t += 1;
        }
        s += 1;
    }
    if failures == 0 {
        printf(cstr("PASSED.\n").as_ptr());
    }
    failures
}

fn main() {
    unsafe {
        std::process::exit(main_impl(
            std::env::args().count() as c_int,
            std::env::args()
                .map(|arg| CString::new(arg).unwrap().into_raw())
                .collect::<Vec<*mut c_char>>()
                .as_mut_ptr(),
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
