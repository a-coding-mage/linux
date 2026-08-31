// Translated from testing/selftests/bpf/prog_tests/net_timestamping.c
// Dependencies originally provided by:
// <linux/net_tstamp.h>, <sys/time.h>, <linux/errqueue.h>,
// "test_progs.h", "network_helpers.h", "net_timestamping.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const CG_NAME: &[u8] = b"/net-timestamping-test\0";
const NSEC_PER_SEC: i64 = 1000000000;

static ADDR4_STR: &[u8] = b"127.0.0.1\0";
static ADDR6_STR: &[u8] = b"::1\0";
static mut SKEL: *mut net_timestamping = ptr::null_mut();
static CFG_PAYLOAD_LEN: c_int = 30;
static mut USR_TS: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut DELAY_TOLERANCE_NSEC: u64 = 10000000000; /* 10 seconds */
#[no_mangle]
pub static mut SK_TS_SCHED: c_int = 0;
#[no_mangle]
pub static mut SK_TS_TXSW: c_int = 0;
#[no_mangle]
pub static mut SK_TS_ACK: c_int = 0;

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct scm_timestamping {
    pub ts: [timespec; 3],
}

#[repr(C)]
pub struct sock_extended_err {
    pub ee_errno: u32,
    pub ee_origin: u8,
    pub ee_type: u8,
    pub ee_code: u8,
    pub ee_pad: u8,
    pub ee_info: u32,
    pub ee_data: u32,
}

#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: u32,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: usize,
    pub msg_control: *mut c_void,
    pub msg_controllen: usize,
    pub msg_flags: c_int,
}

#[repr(C)]
pub struct net_timestamping {
    pub links: net_timestamping__links,
    pub progs: net_timestamping__progs,
    pub bss: *mut net_timestamping__bss,
}

#[repr(C)]
pub struct net_timestamping__links {
    pub skops_sockopt: *mut c_void,
}

#[repr(C)]
pub struct net_timestamping__progs {
    pub skops_sockopt: *mut c_void,
}

#[repr(C)]
pub struct net_timestamping__bss {
    pub monitored_pid: c_int,
    pub nr_active: c_int,
    pub nr_snd: c_int,
    pub nr_sched: c_int,
    pub nr_txsw: c_int,
    pub nr_ack: c_int,
}

#[repr(C)]
pub struct netns_obj {
    _private: [u8; 0],
}

extern "C" {
    static mut errno: c_int;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> isize;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: u32,
    ) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> c_int;

    fn CMSG_FIRSTHDR(mhdr: *mut msghdr) -> *mut cmsghdr;
    fn CMSG_NXTHDR(mhdr: *mut msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr;
    fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut u8;

    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ<T, U>(actual: T, expected: U, name: *const c_char) -> bool;
    fn ASSERT_LT<T, U>(actual: T, expected: U, name: *const c_char) -> bool;

    fn netns_new(name: *const c_char, attach: bool) -> *mut netns_obj;
    fn netns_free(ns: *mut netns_obj);
    fn start_server(
        family: c_int,
        socktype: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn connect_to_fd(fd: c_int, timeout_ms: c_int) -> c_int;

    fn net_timestamping__open_and_load() -> *mut net_timestamping;
    fn net_timestamping__attach(obj: *mut net_timestamping) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut c_void, cgroup_fd: c_int) -> *mut c_void;
    fn net_timestamping__destroy(obj: *mut net_timestamping);
}

const SCM_TSTAMP_SCHED: c_int = 1;
const SCM_TSTAMP_SND: c_int = 0;
const SCM_TSTAMP_ACK: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SCM_TIMESTAMPING: c_int = 37;
const SOL_IP: c_int = 0;
const IP_RECVERR: c_int = 11;
const SOL_IPV6: c_int = 41;
const IPV6_RECVERR: c_int = 25;
const SOL_PACKET: c_int = 263;
const PACKET_TX_TIMESTAMP: c_int = 16;
const SO_EE_ORIGIN_TIMESTAMPING: u8 = 4;
const MSG_ERRQUEUE: c_int = 0x2000;
const EAGAIN: c_int = 11;
const SOCK_STREAM: c_int = 1;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SO_TIMESTAMPING: c_int = 37;
const SOF_TIMESTAMPING_TX_SOFTWARE: c_uint = 1 << 1;
const SOF_TIMESTAMPING_TX_SCHED: c_uint = 1 << 8;
const SOF_TIMESTAMPING_SOFTWARE: c_uint = 1 << 4;
const SOF_TIMESTAMPING_OPT_ID: c_uint = 1 << 7;
const SOF_TIMESTAMPING_TX_ACK: c_uint = 1 << 9;
const CLOCK_REALTIME: c_int = 0;

unsafe fn timespec_to_ns64(ts: *mut timespec) -> i64 {
    (*ts).tv_sec * NSEC_PER_SEC + (*ts).tv_nsec
}

unsafe fn validate_key(tskey: c_int, tstype: c_int) {
    static mut EXPECTED_TSKEY: c_int = -1;

    if tstype == SCM_TSTAMP_SCHED {
        EXPECTED_TSKEY = CFG_PAYLOAD_LEN - 1;
    }

    ASSERT_EQ(EXPECTED_TSKEY, tskey, c"tskey mismatch".as_ptr());

    EXPECTED_TSKEY = tskey;
}

unsafe fn validate_timestamp(cur: *mut timespec, prev: *mut timespec) {
    let cur_ns: i64;
    let prev_ns: i64;

    cur_ns = timespec_to_ns64(cur);
    prev_ns = timespec_to_ns64(prev);

    ASSERT_LT(cur_ns - prev_ns, DELAY_TOLERANCE_NSEC, c"latency".as_ptr());
}

unsafe fn test_socket_timestamp(tss: *mut scm_timestamping, tstype: c_int, tskey: c_int) {
    static mut PREV_TS: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    validate_key(tskey, tstype);

    match tstype {
        SCM_TSTAMP_SCHED => {
            validate_timestamp(&mut (*tss).ts[0], &mut USR_TS);
            SK_TS_SCHED += 1;
        }
        SCM_TSTAMP_SND => {
            validate_timestamp(&mut (*tss).ts[0], &mut PREV_TS);
            SK_TS_TXSW += 1;
        }
        SCM_TSTAMP_ACK => {
            validate_timestamp(&mut (*tss).ts[0], &mut PREV_TS);
            SK_TS_ACK += 1;
        }
        _ => {}
    }

    PREV_TS = (*tss).ts[0];
}

unsafe fn test_recv_errmsg_cmsg(msg: *mut msghdr) {
    let mut serr: *mut sock_extended_err = ptr::null_mut();
    let mut tss: *mut scm_timestamping = ptr::null_mut();
    let mut cm: *mut cmsghdr;

    cm = CMSG_FIRSTHDR(msg);
    while !cm.is_null() && (*cm).cmsg_len != 0 {
        if (*cm).cmsg_level == SOL_SOCKET && (*cm).cmsg_type == SCM_TIMESTAMPING {
            tss = CMSG_DATA(cm) as *mut scm_timestamping;
        } else if ((*cm).cmsg_level == SOL_IP && (*cm).cmsg_type == IP_RECVERR)
            || ((*cm).cmsg_level == SOL_IPV6 && (*cm).cmsg_type == IPV6_RECVERR)
            || ((*cm).cmsg_level == SOL_PACKET && (*cm).cmsg_type == PACKET_TX_TIMESTAMP)
        {
            serr = CMSG_DATA(cm) as *mut sock_extended_err;
            ASSERT_EQ(
                (*serr).ee_origin,
                SO_EE_ORIGIN_TIMESTAMPING,
                c"cmsg type".as_ptr(),
            );
        }

        if !serr.is_null() && !tss.is_null() {
            test_socket_timestamp(tss, (*serr).ee_info as c_int, (*serr).ee_data as c_int);
        }

        cm = CMSG_NXTHDR(msg, cm);
    }
}

unsafe fn socket_recv_errmsg(fd: c_int) -> bool {
    static mut CTRL: [c_char; 1024] = [0; 1024 /* overprovision*/];
    let mut data: [c_char; 30] = [0; 30];
    static mut MSG: msghdr = msghdr {
        msg_name: ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: ptr::null_mut(),
        msg_iovlen: 0,
        msg_control: ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut entry: iovec = mem::zeroed();
    let mut n: isize = 0;

    memset(
        &mut MSG as *mut msghdr as *mut c_void,
        0,
        mem::size_of::<msghdr>(),
    );
    memset(
        &mut entry as *mut iovec as *mut c_void,
        0,
        mem::size_of::<iovec>(),
    );
    memset(
        CTRL.as_mut_ptr() as *mut c_void,
        0,
        mem::size_of_val(&CTRL),
    );

    entry.iov_base = data.as_mut_ptr() as *mut c_void;
    entry.iov_len = CFG_PAYLOAD_LEN as usize;
    MSG.msg_iov = &mut entry;
    MSG.msg_iovlen = 1;
    MSG.msg_name = ptr::null_mut();
    MSG.msg_namelen = 0;
    MSG.msg_control = CTRL.as_mut_ptr() as *mut c_void;
    MSG.msg_controllen = mem::size_of_val(&CTRL);

    n = recvmsg(fd, &mut MSG, MSG_ERRQUEUE);
    if n == -1 {
        ASSERT_EQ(errno, EAGAIN, c"recvmsg MSG_ERRQUEUE".as_ptr());
    }

    if n >= 0 {
        test_recv_errmsg_cmsg(&mut MSG);
    }

    n == -1
}

unsafe fn test_socket_timestamping(fd: c_int) {
    while !socket_recv_errmsg(fd) {}

    ASSERT_EQ(SK_TS_SCHED, 1, c"SCM_TSTAMP_SCHED".as_ptr());
    ASSERT_EQ(SK_TS_TXSW, 1, c"SCM_TSTAMP_SND".as_ptr());
    ASSERT_EQ(SK_TS_ACK, 1, c"SCM_TSTAMP_ACK".as_ptr());

    SK_TS_SCHED = 0;
    SK_TS_TXSW = 0;
    SK_TS_ACK = 0;
}

unsafe fn test_tcp(family: c_int, enable_socket_timestamping: bool) {
    let mut bss: *mut net_timestamping__bss;
    let mut buf: [c_char; 30] = [0; 30];
    let mut sfd: c_int = -1;
    let mut cfd: c_int = -1;
    let mut sock_opt: c_uint;
    let mut ns: *mut netns_obj = ptr::null_mut();
    let cg_fd: c_int;
    let mut ret: c_int;

    cg_fd = test__join_cgroup(CG_NAME.as_ptr() as *const c_char);
    if !ASSERT_OK_FD(cg_fd, c"join cgroup".as_ptr()) {
        return;
    }

    ns = netns_new(c"net_timestamping_ns".as_ptr(), true);
    if !ASSERT_OK_PTR(ns as *const c_void, c"create ns".as_ptr()) {
        goto_out(sfd, cfd, ns, cg_fd);
        return;
    }

    SKEL = net_timestamping__open_and_load();
    if !ASSERT_OK_PTR(SKEL as *const c_void, c"open and load skel".as_ptr()) {
        goto_out(sfd, cfd, ns, cg_fd);
        return;
    }

    if !ASSERT_OK(net_timestamping__attach(SKEL), c"attach skel".as_ptr()) {
        goto_out(sfd, cfd, ns, cg_fd);
        return;
    }

    (*SKEL).links.skops_sockopt = bpf_program__attach_cgroup((*SKEL).progs.skops_sockopt, cg_fd);
    if !ASSERT_OK_PTR(
        (*SKEL).links.skops_sockopt as *const c_void,
        c"attach cgroup".as_ptr(),
    ) {
        goto_out(sfd, cfd, ns, cg_fd);
        return;
    }

    bss = (*SKEL).bss;
    memset(
        bss as *mut c_void,
        0,
        mem::size_of::<net_timestamping__bss>(),
    );

    (*(*SKEL).bss).monitored_pid = getpid();

    sfd = start_server(
        family,
        SOCK_STREAM,
        if family == AF_INET6 {
            ADDR6_STR.as_ptr() as *const c_char
        } else {
            ADDR4_STR.as_ptr() as *const c_char
        },
        0,
        0,
    );
    if !ASSERT_OK_FD(sfd, c"start_server".as_ptr()) {
        goto_out(sfd, cfd, ns, cg_fd);
        return;
    }

    cfd = connect_to_fd(sfd, 0);
    if !ASSERT_OK_FD(cfd, c"connect_to_fd_server".as_ptr()) {
        goto_out(sfd, cfd, ns, cg_fd);
        return;
    }

    if enable_socket_timestamping {
        sock_opt = SOF_TIMESTAMPING_SOFTWARE
            | SOF_TIMESTAMPING_OPT_ID
            | SOF_TIMESTAMPING_TX_SCHED
            | SOF_TIMESTAMPING_TX_SOFTWARE
            | SOF_TIMESTAMPING_TX_ACK;
        ret = setsockopt(
            cfd,
            SOL_SOCKET,
            SO_TIMESTAMPING,
            &sock_opt as *const c_uint as *const c_void,
            mem::size_of_val(&sock_opt) as u32,
        );
        if !ASSERT_OK(ret, c"setsockopt SO_TIMESTAMPING".as_ptr()) {
            goto_out(sfd, cfd, ns, cg_fd);
            return;
        }

        ret = clock_gettime(CLOCK_REALTIME, &mut USR_TS);
        if !ASSERT_OK(ret, c"get user time".as_ptr()) {
            goto_out(sfd, cfd, ns, cg_fd);
            return;
        }
    }

    ret = write(cfd, buf.as_mut_ptr() as *const c_void, mem::size_of_val(&buf)) as c_int;
    if !ASSERT_EQ(ret, mem::size_of_val(&buf), c"send to server".as_ptr()) {
        goto_out(sfd, cfd, ns, cg_fd);
        return;
    }

    if enable_socket_timestamping {
        test_socket_timestamping(cfd);
    }

    ASSERT_EQ((*bss).nr_active, 1, c"nr_active".as_ptr());
    ASSERT_EQ((*bss).nr_snd, 2, c"nr_snd".as_ptr());
    ASSERT_EQ((*bss).nr_sched, 1, c"nr_sched".as_ptr());
    ASSERT_EQ((*bss).nr_txsw, 1, c"nr_txsw".as_ptr());
    ASSERT_EQ((*bss).nr_ack, 1, c"nr_ack".as_ptr());

    goto_out(sfd, cfd, ns, cg_fd);
}

unsafe fn goto_out(sfd: c_int, cfd: c_int, ns: *mut netns_obj, cg_fd: c_int) {
    if sfd >= 0 {
        close(sfd);
    }
    if cfd >= 0 {
        close(cfd);
    }
    net_timestamping__destroy(SKEL);
    netns_free(ns);
    close(cg_fd);
}

#[no_mangle]
pub unsafe extern "C" fn test_net_timestamping() {
    if test__start_subtest(c"INET4: bpf timestamping".as_ptr()) {
        test_tcp(AF_INET, false);
    }
    if test__start_subtest(c"INET4: bpf and socket timestamping".as_ptr()) {
        test_tcp(AF_INET, true);
    }
    if test__start_subtest(c"INET6: bpf timestamping".as_ptr()) {
        test_tcp(AF_INET6, false);
    }
    if test__start_subtest(c"INET6: bpf and socket timestamping".as_ptr()) {
        test_tcp(AF_INET6, true);
    }
}
