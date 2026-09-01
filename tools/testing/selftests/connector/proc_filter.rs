// SPDX-License-Identifier: GPL-2.0-only

// Translated from testing/selftests/connector/proc_filter.c.
// C include dependencies preserved as Rust extern/type references:
// sys/types.h, sys/epoll.h, sys/socket.h, linux/netlink.h,
// linux/connector.h, linux/cn_proc.h, stddef.h, stdio.h, stdlib.h,
// unistd.h, strings.h, errno.h, signal.h, string.h, and kselftest.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const NL_MESSAGE_SIZE: usize =
    size_of::<nlmsghdr>() + size_of::<cn_msg>() + size_of::<proc_input>();
const NL_MESSAGE_SIZE_NF: usize =
    size_of::<nlmsghdr>() + size_of::<cn_msg>() + size_of::<c_int>();

const MAX_EVENTS: usize = 1;

static mut INTERRUPTED: c_int = 0;
static mut NL_SOCK: c_int = 0;
static mut RET_ERRNO: c_int = 0;
static mut TCOUNT: c_int = 0;
static mut EVN: epoll_event = epoll_event {
    events: 0,
    data: epoll_data_t { fd: 0 },
};

static mut FILTER: c_int = 0;

// #ifdef ENABLE_PRINTS: Printf maps to printf.
// #else: Printf maps to ksft_print_msg.
#[cfg(ENABLE_PRINTS)]
unsafe fn Printf(fmt: *const c_char, mut args: ...) -> c_int {
    printf(fmt, args)
}

#[cfg(not(ENABLE_PRINTS))]
unsafe fn Printf(fmt: *const c_char, mut args: ...) -> c_int {
    ksft_print_msg(fmt, args)
}

unsafe fn send_message(pinp: *mut c_void) -> c_int {
    let mut buff = [0 as c_char; NL_MESSAGE_SIZE];
    let hdr: *mut nlmsghdr;
    let msg: *mut cn_msg;

    hdr = buff.as_mut_ptr() as *mut nlmsghdr;
    if FILTER != 0 {
        (*hdr).nlmsg_len = NL_MESSAGE_SIZE as __u32;
    } else {
        (*hdr).nlmsg_len = NL_MESSAGE_SIZE_NF as __u32;
    }
    (*hdr).nlmsg_type = NLMSG_DONE as __u16;
    (*hdr).nlmsg_flags = 0;
    (*hdr).nlmsg_seq = 0;
    (*hdr).nlmsg_pid = getpid() as __u32;

    msg = NLMSG_DATA(hdr) as *mut cn_msg;
    (*msg).id.idx = CN_IDX_PROC;
    (*msg).id.val = CN_VAL_PROC;
    (*msg).seq = 0;
    (*msg).ack = 0;
    (*msg).flags = 0;

    if FILTER != 0 {
        (*msg).len = size_of::<proc_input>() as __u16;
        (*(msg_data_mut(msg) as *mut proc_input)).mcast_op =
            (*(pinp as *mut proc_input)).mcast_op;
        (*(msg_data_mut(msg) as *mut proc_input)).event_type =
            (*(pinp as *mut proc_input)).event_type;
    } else {
        (*msg).len = size_of::<c_int>() as __u16;
        *(msg_data_mut(msg) as *mut c_int) = *(pinp as *mut proc_cn_mcast_op) as c_int;
    }

    if send(NL_SOCK, hdr as *const c_void, (*hdr).nlmsg_len as usize, 0) == -1 {
        RET_ERRNO = *__errno_location();
        perror(c"send failed".as_ptr());
        return -3;
    }
    0
}

unsafe fn register_proc_netlink(efd: *mut c_int, input: *mut c_void) -> c_int {
    let mut sa_nl: sockaddr_nl = core::mem::zeroed();
    let mut err: c_int = 0;
    let epoll_fd: c_int;

    NL_SOCK = socket(PF_NETLINK, SOCK_DGRAM, NETLINK_CONNECTOR);

    if NL_SOCK == -1 {
        RET_ERRNO = *__errno_location();
        perror(c"socket failed".as_ptr());
        return -1;
    }

    bzero(
        &mut sa_nl as *mut sockaddr_nl as *mut c_void,
        size_of::<sockaddr_nl>(),
    );
    sa_nl.nl_family = AF_NETLINK as sa_family_t;
    sa_nl.nl_groups = CN_IDX_PROC;
    sa_nl.nl_pid = getpid() as __u32;

    if bind(
        NL_SOCK,
        &mut sa_nl as *mut sockaddr_nl as *mut sockaddr,
        size_of::<sockaddr_nl>() as socklen_t,
    ) == -1
    {
        RET_ERRNO = *__errno_location();
        perror(c"bind failed".as_ptr());
        return -2;
    }

    epoll_fd = epoll_create1(EPOLL_CLOEXEC);
    if epoll_fd < 0 {
        RET_ERRNO = *__errno_location();
        perror(c"epoll_create1 failed".as_ptr());
        return -2;
    }

    err = send_message(input);

    if err < 0 {
        return err;
    }

    EVN.events = EPOLLIN as u32;
    EVN.data.fd = NL_SOCK;
    if epoll_ctl(epoll_fd, EPOLL_CTL_ADD, NL_SOCK, &mut EVN) < 0 {
        RET_ERRNO = *__errno_location();
        perror(c"epoll_ctl failed".as_ptr());
        return -3;
    }
    *efd = epoll_fd;
    0
}

unsafe extern "C" fn sigint(_sig: c_int) {
    INTERRUPTED = 1;
}

unsafe fn handle_packet(buff: *mut c_char, _fd: c_int, mut event: *mut proc_event) -> c_int {
    let hdr: *mut nlmsghdr;

    hdr = buff as *mut nlmsghdr;

    if (*hdr).nlmsg_type as c_int == NLMSG_ERROR {
        perror(c"NLMSG_ERROR error\n".as_ptr());
        return -3;
    } else if (*hdr).nlmsg_type as c_int == NLMSG_DONE {
        event = (*(NLMSG_DATA(hdr) as *mut cn_msg)).data.as_mut_ptr() as *mut proc_event;
        TCOUNT += 1;
        match (*event).what as c_uint {
            PROC_EVENT_EXIT => {
                Printf(
                    c"Exit process %d (tgid %d) with code %d, signal %d\n".as_ptr(),
                    (*event).event_data.exit.process_pid,
                    (*event).event_data.exit.process_tgid,
                    (*event).event_data.exit.exit_code,
                    (*event).event_data.exit.exit_signal,
                );
            }
            PROC_EVENT_FORK => {
                Printf(
                    c"Fork process %d (tgid %d), parent %d (tgid %d)\n".as_ptr(),
                    (*event).event_data.fork.child_pid,
                    (*event).event_data.fork.child_tgid,
                    (*event).event_data.fork.parent_pid,
                    (*event).event_data.fork.parent_tgid,
                );
            }
            PROC_EVENT_EXEC => {
                Printf(
                    c"Exec process %d (tgid %d)\n".as_ptr(),
                    (*event).event_data.exec.process_pid,
                    (*event).event_data.exec.process_tgid,
                );
            }
            PROC_EVENT_UID => {
                Printf(
                    c"UID process %d (tgid %d) uid %d euid %d\n".as_ptr(),
                    (*event).event_data.id.process_pid,
                    (*event).event_data.id.process_tgid,
                    (*event).event_data.id.r.ruid,
                    (*event).event_data.id.e.euid,
                );
            }
            PROC_EVENT_GID => {
                Printf(
                    c"GID process %d (tgid %d) gid %d egid %d\n".as_ptr(),
                    (*event).event_data.id.process_pid,
                    (*event).event_data.id.process_tgid,
                    (*event).event_data.id.r.rgid,
                    (*event).event_data.id.e.egid,
                );
            }
            PROC_EVENT_SID => {
                Printf(
                    c"SID process %d (tgid %d)\n".as_ptr(),
                    (*event).event_data.sid.process_pid,
                    (*event).event_data.sid.process_tgid,
                );
            }
            PROC_EVENT_PTRACE => {
                Printf(
                    c"Ptrace process %d (tgid %d), Tracer %d (tgid %d)\n".as_ptr(),
                    (*event).event_data.ptrace.process_pid,
                    (*event).event_data.ptrace.process_tgid,
                    (*event).event_data.ptrace.tracer_pid,
                    (*event).event_data.ptrace.tracer_tgid,
                );
            }
            PROC_EVENT_COMM => {
                Printf(
                    c"Comm process %d (tgid %d) comm %s\n".as_ptr(),
                    (*event).event_data.comm.process_pid,
                    (*event).event_data.comm.process_tgid,
                    (*event).event_data.comm.comm.as_ptr(),
                );
            }
            PROC_EVENT_COREDUMP => {
                Printf(
                    c"Coredump process %d (tgid %d) parent %d, (tgid %d)\n".as_ptr(),
                    (*event).event_data.coredump.process_pid,
                    (*event).event_data.coredump.process_tgid,
                    (*event).event_data.coredump.parent_pid,
                    (*event).event_data.coredump.parent_tgid,
                );
            }
            _ => {}
        }
    }
    0
}

unsafe fn handle_events(epoll_fd: c_int, pev: *mut proc_event) -> c_int {
    let mut buff = [0 as c_char; CONNECTOR_MAX_MSG_SIZE];
    let mut ev = [epoll_event {
        events: 0,
        data: epoll_data_t { fd: 0 },
    }; MAX_EVENTS];
    let mut i: c_int;
    let mut event_count: c_int = 0;
    let mut err: c_int = 0;

    event_count = epoll_wait(epoll_fd, ev.as_mut_ptr(), MAX_EVENTS as c_int, -1);
    if event_count < 0 {
        RET_ERRNO = *__errno_location();
        if RET_ERRNO != EINTR {
            perror(c"epoll_wait failed".as_ptr());
        }
        return -3;
    }
    i = 0;
    while i < event_count {
        if ev[i as usize].events & EPOLLIN as u32 == 0 {
            i += 1;
            continue;
        }
        if recv(
            ev[i as usize].data.fd,
            buff.as_mut_ptr() as *mut c_void,
            size_of_val(&buff),
            0,
        ) == -1
        {
            RET_ERRNO = *__errno_location();
            perror(c"recv failed".as_ptr());
            return -3;
        }
        err = handle_packet(buff.as_mut_ptr(), ev[i as usize].data.fd, pev);
        if err < 0 {
            return err;
        }
        i += 1;
    }
    0
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut epoll_fd: c_int = 0;
    let mut err: c_int;
    let mut proc_ev: proc_event = core::mem::zeroed();
    let mut input: proc_input = core::mem::zeroed();

    signal(SIGINT, Some(sigint));

    if argc > 2 {
        printf(c"Expected 0(assume no-filter) or 1 argument(-f)\n".as_ptr());
        exit(KSFT_SKIP);
    }

    if argc == 2 {
        if strcmp(*argv.add(1), c"-f".as_ptr()) == 0 {
            FILTER = 1;
        } else {
            printf(c"Valid option : -f (for filter feature)\n".as_ptr());
            exit(KSFT_SKIP);
        }
    }

    if FILTER != 0 {
        input.event_type = PROC_EVENT_NONZERO_EXIT;
        input.mcast_op = PROC_CN_MCAST_LISTEN;
        err = register_proc_netlink(&mut epoll_fd, &mut input as *mut proc_input as *mut c_void);
    } else {
        let mut op: proc_cn_mcast_op = PROC_CN_MCAST_LISTEN;
        err = register_proc_netlink(&mut epoll_fd, &mut op as *mut proc_cn_mcast_op as *mut c_void);
    }

    if err < 0 {
        if err == -2 {
            close(NL_SOCK);
        }
        if err == -3 {
            close(NL_SOCK);
            close(epoll_fd);
        }
        exit(1);
    }

    while INTERRUPTED == 0 {
        err = handle_events(epoll_fd, &mut proc_ev);
        if err < 0 {
            if RET_ERRNO == EINTR {
                continue;
            }
            if err == -2 {
                close(NL_SOCK);
            }
            if err == -3 {
                close(NL_SOCK);
                close(epoll_fd);
            }
            exit(1);
        }
    }

    if FILTER != 0 {
        input.mcast_op = PROC_CN_MCAST_IGNORE;
        send_message(&mut input as *mut proc_input as *mut c_void);
    } else {
        let mut op: proc_cn_mcast_op = PROC_CN_MCAST_IGNORE;
        send_message(&mut op as *mut proc_cn_mcast_op as *mut c_void);
    }

    close(epoll_fd);
    close(NL_SOCK);

    printf(c"Done total count: %d\n".as_ptr(), TCOUNT);
    exit(0);
}

fn main() {
    unsafe {
        extern "C" {
            static mut __argc: c_int;
            static mut __argv: *mut *mut c_char;
        }
        main_0(__argc, __argv);
    }
}

unsafe fn msg_data_mut(msg: *mut cn_msg) -> *mut c_void {
    (*msg).data.as_mut_ptr() as *mut c_void
}

unsafe fn NLMSG_DATA(nlh: *mut nlmsghdr) -> *mut c_void {
    (nlh as *mut c_char).add(NLMSG_HDRLEN) as *mut c_void
}

const NLMSG_HDRLEN: usize = size_of::<nlmsghdr>();

extern "C" {
    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(
        epfd: c_int,
        events: *mut epoll_event,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;
    fn getpid() -> pid_t;
    fn close(fd: c_int) -> c_int;
    fn bzero(s: *mut c_void, n: usize);
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn signal(signum: c_int, handler: Option<unsafe extern "C" fn(c_int)>)
        -> Option<unsafe extern "C" fn(c_int)>;
    fn exit(status: c_int) -> !;
    fn ksft_print_msg(format: *const c_char, ...) -> c_int;
    fn __errno_location() -> *mut c_int;
}

type __u16 = u16;
type __u32 = u32;
type pid_t = c_int;
type socklen_t = c_uint;
type sa_family_t = u16;
type proc_cn_mcast_op = c_int;

#[repr(C)]
struct sockaddr {
    sa_family: sa_family_t,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_nl {
    nl_family: sa_family_t,
    nl_pad: __u16,
    nl_pid: __u32,
    nl_groups: __u32,
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
#[derive(Copy, Clone)]
union epoll_data_t {
    ptr: *mut c_void,
    fd: c_int,
    u32_: u32,
    u64_: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct epoll_event {
    events: u32,
    data: epoll_data_t,
}

#[repr(C)]
struct cb_id {
    idx: __u32,
    val: __u32,
}

#[repr(C)]
struct cn_msg {
    id: cb_id,
    seq: __u32,
    ack: __u32,
    len: __u16,
    flags: __u16,
    data: [u8; 0],
}

#[repr(C)]
struct proc_input {
    mcast_op: proc_cn_mcast_op,
    event_type: c_uint,
}

#[repr(C)]
struct proc_event {
    what: c_uint,
    cpu: __u32,
    timestamp_ns: u64,
    event_data: proc_event_data,
}

#[repr(C)]
union proc_event_data {
    fork: fork_proc_event,
    exec: exec_proc_event,
    id: id_proc_event,
    sid: sid_proc_event,
    ptrace: ptrace_proc_event,
    comm: comm_proc_event,
    coredump: coredump_proc_event,
    exit: exit_proc_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct fork_proc_event {
    parent_pid: pid_t,
    parent_tgid: pid_t,
    child_pid: pid_t,
    child_tgid: pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct exec_proc_event {
    process_pid: pid_t,
    process_tgid: pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct id_proc_event {
    process_pid: pid_t,
    process_tgid: pid_t,
    r: id_proc_event_r,
    e: id_proc_event_e,
}

#[repr(C)]
#[derive(Copy, Clone)]
union id_proc_event_r {
    ruid: __u32,
    rgid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
union id_proc_event_e {
    euid: __u32,
    egid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sid_proc_event {
    process_pid: pid_t,
    process_tgid: pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ptrace_proc_event {
    process_pid: pid_t,
    process_tgid: pid_t,
    tracer_pid: pid_t,
    tracer_tgid: pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct comm_proc_event {
    process_pid: pid_t,
    process_tgid: pid_t,
    comm: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct coredump_proc_event {
    process_pid: pid_t,
    process_tgid: pid_t,
    parent_pid: pid_t,
    parent_tgid: pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct exit_proc_event {
    process_pid: pid_t,
    process_tgid: pid_t,
    exit_code: __u32,
    exit_signal: __u32,
}

const PF_NETLINK: c_int = 16;
const AF_NETLINK: c_int = PF_NETLINK;
const SOCK_DGRAM: c_int = 2;
const NETLINK_CONNECTOR: c_int = 11;
const NLMSG_ERROR: c_int = 2;
const NLMSG_DONE: c_int = 3;
const CN_IDX_PROC: __u32 = 0x1;
const CN_VAL_PROC: __u32 = 0x1;
const EPOLLIN: c_int = 0x001;
const EPOLL_CTL_ADD: c_int = 1;
const EPOLL_CLOEXEC: c_int = 0o2000000;
const EINTR: c_int = 4;
const SIGINT: c_int = 2;
const KSFT_SKIP: c_int = 4;
const CONNECTOR_MAX_MSG_SIZE: usize = 16 * 1024;

const PROC_CN_MCAST_LISTEN: proc_cn_mcast_op = 1;
const PROC_CN_MCAST_IGNORE: proc_cn_mcast_op = 2;

const PROC_EVENT_FORK: c_uint = 0x00000001;
const PROC_EVENT_EXEC: c_uint = 0x00000002;
const PROC_EVENT_UID: c_uint = 0x00000004;
const PROC_EVENT_GID: c_uint = 0x00000040;
const PROC_EVENT_SID: c_uint = 0x00000080;
const PROC_EVENT_PTRACE: c_uint = 0x00000100;
const PROC_EVENT_COMM: c_uint = 0x00000200;
const PROC_EVENT_COREDUMP: c_uint = 0x40000000;
const PROC_EVENT_EXIT: c_uint = 0x80000000;
const PROC_EVENT_NONZERO_EXIT: c_uint = 0x20000000;

fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
