// SPDX-License-Identifier: GPL-2.0
/* procacct.c
 *
 * Demonstrator of fetching resource data on task exit, as a way
 * to accumulate accurate program resource usage statistics, without
 * prior identification of the programs. For that, the fields for
 * device and inode of the program executable binary file are also
 * extracted in addition to the command string.
 *
 * The TGID together with the PID and the AGROUP flag allow
 * identification of threads in a process and single-threaded processes.
 * The ac_tgetime field gives proper whole-process walltime.
 *
 * Written (changed) by Thomas Orgis, University of Hamburg in 2022
 *
 * This is a cheap derivation (inheriting the style) of getdelays.c:
 *
 * Utility to get per-pid and per-tgid delay accounting statistics
 * Also illustrates usage of the taskstats interface
 *
 * Copyright (C) Shailabh Nagar, IBM Corp. 2005
 * Copyright (C) Balbir Singh, IBM Corp. 2006
 * Copyright (c) Jay Lan, SGI. 2006
 */

use libc::{
    c_char, c_int, c_short, c_uint, c_ulong, c_ushort, c_void, iovec, msghdr, pid_t, sockaddr,
    sockaddr_nl, size_t, ssize_t, socklen_t,
};
use std::mem;
use std::ptr;

/*
 * C headers used by the original source:
 * stdio.h, stdlib.h, errno.h, unistd.h, poll.h, string.h, fcntl.h,
 * sys/types.h, sys/stat.h, sys/socket.h, sys/wait.h, signal.h,
 * linux/genetlink.h, linux/acct.h, linux/taskstats.h, linux/kdev_t.h
 */

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;

extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut FILE;
    static mut optarg: *mut c_char;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> ssize_t;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    fn getpid() -> pid_t;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, mode: c_uint) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
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
struct genlmsghdr {
    cmd: __u8,
    version: __u8,
    reserved: __u16,
}

#[repr(C)]
struct nlattr {
    nla_len: __u16,
    nla_type: __u16,
}

#[repr(C)]
struct nlmsgerr {
    error: c_int,
    msg: nlmsghdr,
}

#[repr(C)]
struct taskstats {
    version: __u16,
    ac_exitcode: __u32,
    ac_flag: __u8,
    ac_nice: __u8,
    cpu_count: __u64,
    cpu_delay_total: __u64,
    blkio_count: __u64,
    blkio_delay_total: __u64,
    swapin_count: __u64,
    swapin_delay_total: __u64,
    cpu_run_real_total: __u64,
    cpu_run_virtual_total: __u64,
    ac_comm: [c_char; 32],
    ac_sched: __u8,
    ac_pad: [__u8; 3],
    ac_uid: __u32,
    ac_gid: __u32,
    ac_pid: __u32,
    ac_ppid: __u32,
    ac_btime: __u32,
    ac_etime: __u64,
    ac_utime: __u64,
    ac_stime: __u64,
    ac_minflt: __u64,
    ac_majflt: __u64,
    coremem: __u64,
    virtmem: __u64,
    hiwater_rss: __u64,
    hiwater_vm: __u64,
    read_char: __u64,
    write_char: __u64,
    read_syscalls: __u64,
    write_syscalls: __u64,
    read_bytes: __u64,
    write_bytes: __u64,
    cancelled_write_bytes: __u64,
    nvcsw: __u64,
    nivcsw: __u64,
    ac_utimescaled: __u64,
    ac_stimescaled: __u64,
    cpu_scaled_run_real_total: __u64,
    freepages_count: __u64,
    freepages_delay_total: __u64,
    thrashing_count: __u64,
    thrashing_delay_total: __u64,
    ac_btime64: __u64,
    compact_count: __u64,
    compact_delay_total: __u64,
    ac_tgid: __u32,
    ac_tgetime: __u64,
    ac_exe_dev: __u64,
    ac_exe_inode: __u64,
    wpcopy_count: __u64,
    wpcopy_delay_total: __u64,
    irq_count: __u64,
    irq_delay_total: __u64,
}

const AF_NETLINK: c_int = 16;
const SOCK_RAW: c_int = 3;
const SOL_SOCKET: c_int = 1;
const SO_RCVBUF: c_int = 8;
const NETLINK_GENERIC: c_int = 16;
const MSG_TRUNC: c_int = 0x20;
const EMSGSIZE: c_int = 90;
const EAGAIN: c_int = 11;
const NLM_F_REQUEST: __u16 = 1;
const NLMSG_ERROR: __u16 = 2;
const GENL_ID_CTRL: __u16 = 0x10;
const CTRL_CMD_GETFAMILY: __u8 = 3;
const CTRL_ATTR_FAMILY_ID: __u16 = 1;
const CTRL_ATTR_FAMILY_NAME: __u16 = 2;
const TASKSTATS_CMD_GET: __u8 = 1;
const TASKSTATS_CMD_ATTR_REGISTER_CPUMASK: __u16 = 4;
const TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK: __u16 = 5;
const TASKSTATS_TYPE_NULL: __u16 = 0;
const TASKSTATS_TYPE_PID: __u16 = 1;
const TASKSTATS_TYPE_TGID: __u16 = 2;
const TASKSTATS_TYPE_STATS: __u16 = 3;
const TASKSTATS_TYPE_AGGR_PID: __u16 = 4;
const TASKSTATS_TYPE_AGGR_TGID: __u16 = 5;
const AGROUP: __u8 = 0x40;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;

const TASKSTATS_GENL_NAME: &[u8] = b"TASKSTATS\0";

/*
 * Generic macros for dealing with netlink sockets. Might be duplicated
 * elsewhere. It is recommended that commercial grade applications use
 * libnl or libnetlink and use the interfaces provided by the library
 */
const NLMSG_ALIGNTO: usize = 4;
const NLA_ALIGNTO: usize = 4;
const NLMSG_HDRLEN: usize = nlmsg_align(mem::size_of::<nlmsghdr>());
const GENL_HDRLEN: usize = nlmsg_align(mem::size_of::<genlmsghdr>());
const NLA_HDRLEN: usize = nla_align(mem::size_of::<nlattr>());

const fn nlmsg_align(len: usize) -> usize {
    (len + NLMSG_ALIGNTO - 1) & !(NLMSG_ALIGNTO - 1)
}

const fn nla_align(len: usize) -> usize {
    (len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}

unsafe fn nlmsg_data(nlh: *const nlmsghdr) -> *mut c_char {
    (nlh as *mut c_char).add(NLMSG_HDRLEN)
}

unsafe fn nlmsg_length(len: usize) -> __u32 {
    (len + NLMSG_HDRLEN) as __u32
}

unsafe fn nlmsg_payload(nlh: *const nlmsghdr, len: usize) -> c_int {
    ((*nlh).nlmsg_len as usize - NLMSG_HDRLEN - len) as c_int
}

unsafe fn nlmsg_ok(nlh: *const nlmsghdr, len: c_int) -> bool {
    len >= mem::size_of::<nlmsghdr>() as c_int
        && (*nlh).nlmsg_len >= mem::size_of::<nlmsghdr>() as __u32
        && (*nlh).nlmsg_len as c_int <= len
}

unsafe fn genlmsg_data(glh: *const nlmsghdr) -> *mut c_void {
    nlmsg_data(glh).add(GENL_HDRLEN) as *mut c_void
}

unsafe fn genlmsg_payload(glh: *const nlmsghdr) -> c_int {
    nlmsg_payload(glh, 0) - GENL_HDRLEN as c_int
}

unsafe fn nla_data(na: *const nlattr) -> *mut c_void {
    (na as *mut c_char).add(NLA_HDRLEN) as *mut c_void
}

fn nla_payload(len: __u16) -> c_int {
    len as c_int - NLA_HDRLEN as c_int
}

fn major(dev: __u64) -> __u32 {
    (((dev >> 8) & 0xfff) | ((dev >> 32) & !0xfff)) as __u32
}

fn minor(dev: __u64) -> __u32 {
    ((dev & 0xff) | ((dev >> 12) & !0xff)) as __u32
}

unsafe fn err(code: c_int, fmt: *const c_char) -> ! {
    fprintf(stderr, fmt);
    exit(code);
}

static mut rcvbufsz: c_int = 0;
static mut name: [c_char; 100] = [0; 100];
static mut dbg: c_int = 0;
static mut print_delays: c_int = 0;
static mut print_io_accounting: c_int = 0;
static mut print_task_context_switch_counts: c_int = 0;

unsafe fn print_debug0(fmt: *const c_char) {
    if dbg != 0 {
        printf(fmt);
    }
}

unsafe fn print_debug1<T>(fmt: *const c_char, a: T) {
    if dbg != 0 {
        printf(fmt, a);
    }
}

unsafe fn print_debug3<A, B, C>(fmt: *const c_char, a: A, b: B, c: C) {
    if dbg != 0 {
        printf(fmt, a, b, c);
    }
}

/* Maximum size of response requested or message sent */
const MAX_MSG_SIZE: usize = 2048;
/* Maximum number of cpus expected to be specified in a cpumask */
const MAX_CPUS: usize = 32;

#[repr(C)]
struct msgtemplate {
    n: nlmsghdr,
    g: genlmsghdr,
    buf: [c_char; MAX_MSG_SIZE],
}

static mut cpumask: [c_char; 100 + 6 * MAX_CPUS] = [0; 100 + 6 * MAX_CPUS];

unsafe fn usage() {
    fprintf(
        stderr,
        b"procacct [-v] [-w logfile] [-r bufsize] [-m cpumask]\n\0".as_ptr() as *const c_char,
    );
    fprintf(stderr, b"  -v: debug on\n\0".as_ptr() as *const c_char);
}

/*
 * Create a raw netlink socket and bind
 */
unsafe fn create_nl_socket(protocol: c_int) -> c_int {
    let fd: c_int;
    let mut local: sockaddr_nl = mem::zeroed();

    fd = socket(AF_NETLINK, SOCK_RAW, protocol);
    if fd < 0 {
        return -1;
    }

    if rcvbufsz != 0 {
        if setsockopt(
            fd,
            SOL_SOCKET,
            SO_RCVBUF,
            &rcvbufsz as *const c_int as *const c_void,
            mem::size_of_val(&rcvbufsz) as socklen_t,
        ) < 0
        {
            fprintf(
                stderr,
                b"Unable to set socket rcv buf size to %d\n\0".as_ptr() as *const c_char,
                rcvbufsz,
            );
            close(fd);
            return -1;
        }
    }

    memset(
        &mut local as *mut sockaddr_nl as *mut c_void,
        0,
        mem::size_of_val(&local),
    );
    local.nl_family = AF_NETLINK as c_ushort;

    if bind(
        fd,
        &local as *const sockaddr_nl as *const sockaddr,
        mem::size_of_val(&local) as socklen_t,
    ) < 0
    {
        close(fd);
        return -1;
    }

    fd
}

unsafe fn recv_taskstats_msg(sd: c_int, msg: *mut msgtemplate) -> c_int {
    let mut nladdr: sockaddr_nl = mem::zeroed();
    let mut iov = iovec {
        iov_base: msg as *mut c_void,
        iov_len: mem::size_of::<msgtemplate>(),
    };
    let mut hdr: msghdr = mem::zeroed();
    hdr.msg_name = &mut nladdr as *mut sockaddr_nl as *mut c_void;
    hdr.msg_namelen = mem::size_of_val(&nladdr) as socklen_t;
    hdr.msg_iov = &mut iov as *mut iovec;
    hdr.msg_iovlen = 1;
    let ret: c_int;

    ret = recvmsg(sd, &mut hdr, 0) as c_int;
    if ret < 0 {
        return -1;
    }
    if (hdr.msg_flags & MSG_TRUNC) != 0 {
        errno = EMSGSIZE;
        return -1;
    }

    ret
}

unsafe fn send_cmd(
    sd: c_int,
    nlmsg_type: __u16,
    nlmsg_pid: __u32,
    genl_cmd: __u8,
    nla_type: __u16,
    nla_data_arg: *mut c_void,
    nla_len: c_int,
) -> c_int {
    let mut na: *mut nlattr;
    let mut nladdr: sockaddr_nl = mem::zeroed();
    let mut r: c_int;
    let mut buflen: c_int;
    let mut buf: *mut c_char;

    let mut msg: msgtemplate = mem::zeroed();

    msg.n.nlmsg_len = nlmsg_length(GENL_HDRLEN);
    msg.n.nlmsg_type = nlmsg_type;
    msg.n.nlmsg_flags = NLM_F_REQUEST;
    msg.n.nlmsg_seq = 0;
    msg.n.nlmsg_pid = nlmsg_pid;
    msg.g.cmd = genl_cmd;
    msg.g.version = 0x1;
    na = genlmsg_data(&msg.n) as *mut nlattr;
    (*na).nla_type = nla_type;
    (*na).nla_len = (nla_len + 1 + NLA_HDRLEN as c_int) as __u16;
    memcpy(nla_data(na), nla_data_arg, nla_len as size_t);
    msg.n.nlmsg_len = msg
        .n
        .nlmsg_len
        .wrapping_add(nlmsg_align((*na).nla_len as usize) as __u32);

    buf = &mut msg as *mut msgtemplate as *mut c_char;
    buflen = msg.n.nlmsg_len as c_int;
    memset(
        &mut nladdr as *mut sockaddr_nl as *mut c_void,
        0,
        mem::size_of_val(&nladdr),
    );
    nladdr.nl_family = AF_NETLINK as c_ushort;
    loop {
        r = sendto(
            sd,
            buf as *const c_void,
            buflen as size_t,
            0,
            &nladdr as *const sockaddr_nl as *const sockaddr,
            mem::size_of_val(&nladdr) as socklen_t,
        ) as c_int;
        if r >= buflen {
            break;
        }
        if r > 0 {
            buf = buf.add(r as usize);
            buflen -= r;
        } else if errno != EAGAIN {
            return -1;
        }
    }
    0
}

/*
 * Probe the controller in genetlink to find the family id
 * for the TASKSTATS family
 */
unsafe fn get_family_id(sd: c_int) -> c_int {
    #[repr(C)]
    struct Ans {
        n: nlmsghdr,
        g: genlmsghdr,
        buf: [c_char; 256],
    }

    let mut ans: Ans = mem::zeroed();

    let mut id: c_int = 0;
    let rc: c_int;
    let mut na: *mut nlattr;
    let rep_len: c_int;

    strcpy(name.as_mut_ptr(), TASKSTATS_GENL_NAME.as_ptr() as *const c_char);
    rc = send_cmd(
        sd,
        GENL_ID_CTRL,
        getpid() as __u32,
        CTRL_CMD_GETFAMILY,
        CTRL_ATTR_FAMILY_NAME,
        name.as_mut_ptr() as *mut c_void,
        strlen(TASKSTATS_GENL_NAME.as_ptr() as *const c_char) as c_int + 1,
    );
    if rc < 0 {
        return 0; /* sendto() failure? */
    }

    rep_len = recv(
        sd,
        &mut ans as *mut Ans as *mut c_void,
        mem::size_of_val(&ans),
        0,
    ) as c_int;
    if ans.n.nlmsg_type == NLMSG_ERROR || rep_len < 0 || !nlmsg_ok(&ans.n, rep_len) {
        return 0;
    }

    na = genlmsg_data(&ans.n) as *mut nlattr;
    na = (na as *mut c_char).add(nla_align((*na).nla_len as usize)) as *mut nlattr;
    if (*na).nla_type == CTRL_ATTR_FAMILY_ID {
        id = *(nla_data(na) as *mut __u16) as c_int;
    }

    id
}

fn average_ms(t: __u64, c: __u64) -> __u64 {
    t / 1000000u64 / if c != 0 { c } else { 1 }
}

unsafe fn print_procacct(t: *mut taskstats) {
    /* First letter: T is a mere thread, G the last in a group, U  unknown. */
    printf(
        b"%c pid=%lu tgid=%lu uid=%lu wall=%llu gwall=%llu cpu=%llu vmpeak=%llu rsspeak=%llu dev=%lu:%lu inode=%llu comm=%s\n\0"
            .as_ptr() as *const c_char,
        if (*t).version >= 12 {
            if ((*t).ac_flag & AGROUP) != 0 {
                'P' as c_int
            } else {
                'T' as c_int
            }
        } else {
            '?' as c_int
        },
        (*t).ac_pid as c_ulong,
        if (*t).version >= 12 { (*t).ac_tgid } else { 0 } as c_ulong,
        (*t).ac_uid as c_ulong,
        (*t).ac_etime as c_ulong,
        if (*t).version >= 12 { (*t).ac_tgetime } else { 0 } as c_ulong,
        (*t).ac_utime.wrapping_add((*t).ac_stime) as c_ulong,
        (*t).hiwater_vm as c_ulong,
        (*t).hiwater_rss as c_ulong,
        if (*t).version >= 12 {
            major((*t).ac_exe_dev)
        } else {
            0
        } as c_ulong,
        if (*t).version >= 12 {
            minor((*t).ac_exe_dev)
        } else {
            0
        } as c_ulong,
        if (*t).version >= 12 {
            (*t).ac_exe_inode
        } else {
            0
        } as c_ulong,
        (*t).ac_comm.as_ptr(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn handle_aggr(mother: c_int, mut na: *mut nlattr, fd: c_int) {
    let aggr_len: c_int = nla_payload((*na).nla_len);
    let mut len2: c_int = 0;
    let mut rtid: pid_t = 0;

    na = nla_data(na) as *mut nlattr;
    while len2 < aggr_len {
        match (*na).nla_type {
            TASKSTATS_TYPE_PID => {
                rtid = *(nla_data(na) as *mut c_int);
                print_debug1(b"PID\t%d\n\0".as_ptr() as *const c_char, rtid);
            }
            TASKSTATS_TYPE_TGID => {
                rtid = *(nla_data(na) as *mut c_int);
                print_debug1(b"TGID\t%d\n\0".as_ptr() as *const c_char, rtid);
            }
            TASKSTATS_TYPE_STATS => {
                print_debug1(
                    b"version %u\n\0".as_ptr() as *const c_char,
                    (*(nla_data(na) as *mut taskstats)).version as c_uint,
                );
                if mother == TASKSTATS_TYPE_AGGR_PID as c_int {
                    print_procacct(nla_data(na) as *mut taskstats);
                }
                if fd != 0 {
                    if write(fd, nla_data(na), (*na).nla_len as size_t) < 0 {
                        err(1, b"write error\n\0".as_ptr() as *const c_char);
                    }
                }
            }
            TASKSTATS_TYPE_NULL => {}
            _ => {
                fprintf(
                    stderr,
                    b"Unknown nested nla_type %d\n\0".as_ptr() as *const c_char,
                    (*na).nla_type as c_int,
                );
            }
        }
        len2 += nla_align((*na).nla_len as usize) as c_int;
        na = (na as *mut c_char).add(nla_align((*na).nla_len as usize)) as *mut nlattr;
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut c: c_int;
    let mut rc: c_int;
    let mut rep_len: c_int;
    let id: __u16;
    let mypid: __u32;

    let mut na: *mut nlattr;
    let mut nl_sd: c_int = -1;
    let mut len: c_int;

    let mut fd: c_int = 0;
    let mut write_file: c_int = 0;
    let mut maskset: c_int = 0;
    let mut logfile: *mut c_char = ptr::null_mut();
    let cfd: c_int = 0;

    let mut msg: msgtemplate = mem::zeroed();

    loop {
        c = getopt(argc, argv, b"m:vr:w:\0".as_ptr() as *const c_char);
        if c < 0 {
            break;
        }

        match c {
            x if x == 'w' as c_int => {
                logfile = strdup(optarg);
                printf(b"write to file %s\n\0".as_ptr() as *const c_char, logfile);
                write_file = 1;
            }
            x if x == 'r' as c_int => {
                rcvbufsz = atoi(optarg);
                printf(
                    b"receive buf size %d\n\0".as_ptr() as *const c_char,
                    rcvbufsz,
                );
                if rcvbufsz < 0 {
                    err(1, b"Invalid rcv buf size\n\0".as_ptr() as *const c_char);
                }
            }
            x if x == 'm' as c_int => {
                strncpy(cpumask.as_mut_ptr(), optarg, mem::size_of_val(&cpumask));
                cpumask[mem::size_of_val(&cpumask) - 1] = 0;
                maskset = 1;
            }
            x if x == 'v' as c_int => {
                printf(b"debug on\n\0".as_ptr() as *const c_char);
                dbg = 1;
            }
            _ => {
                usage();
                exit(-1);
            }
        }
    }
    if maskset == 0 {
        maskset = 1;
        strncpy(
            cpumask.as_mut_ptr(),
            b"1\0".as_ptr() as *const c_char,
            mem::size_of_val(&cpumask),
        );
        cpumask[mem::size_of_val(&cpumask) - 1] = 0;
    }
    printf(
        b"cpumask %s maskset %d\n\0".as_ptr() as *const c_char,
        cpumask.as_ptr(),
        maskset,
    );

    if write_file != 0 {
        fd = open(logfile, O_WRONLY | O_CREAT | O_TRUNC, 0o644);
        if fd == -1 {
            perror(b"Cannot open output file\n\0".as_ptr() as *const c_char);
            exit(1);
        }
    }

    nl_sd = create_nl_socket(NETLINK_GENERIC);
    if nl_sd < 0 {
        err(
            1,
            b"error creating Netlink socket\n\0".as_ptr() as *const c_char,
        );
    }

    mypid = getpid() as __u32;
    id = get_family_id(nl_sd) as __u16;
    if id == 0 {
        fprintf(
            stderr,
            b"Error getting family id, errno %d\n\0".as_ptr() as *const c_char,
            errno,
        );
        close(nl_sd);
        if fd != 0 {
            close(fd);
        }
        if cfd != 0 {
            close(cfd);
        }
        return 0;
    }
    print_debug1(b"family id %d\n\0".as_ptr() as *const c_char, id as c_int);

    if maskset != 0 {
        rc = send_cmd(
            nl_sd,
            id,
            mypid,
            TASKSTATS_CMD_GET,
            TASKSTATS_CMD_ATTR_REGISTER_CPUMASK,
            cpumask.as_mut_ptr() as *mut c_void,
            strlen(cpumask.as_ptr()) as c_int + 1,
        );
        print_debug1(
            b"Sent register cpumask, retval %d\n\0".as_ptr() as *const c_char,
            rc,
        );
        if rc < 0 {
            fprintf(
                stderr,
                b"error sending register cpumask\n\0".as_ptr() as *const c_char,
            );
            close(nl_sd);
            if fd != 0 {
                close(fd);
            }
            if cfd != 0 {
                close(cfd);
            }
            return 0;
        }
    }

    loop {
        rep_len = recv_taskstats_msg(nl_sd, &mut msg);
        print_debug1(
            b"received %d bytes\n\0".as_ptr() as *const c_char,
            rep_len,
        );

        if rep_len < 0 {
            if errno == EMSGSIZE {
                fprintf(
                    stderr,
                    b"dropped truncated taskstats netlink message, please increase MAX_MSG_SIZE\n\0"
                        .as_ptr() as *const c_char,
                );
            } else {
                fprintf(
                    stderr,
                    b"nonfatal reply error: errno %d\n\0".as_ptr() as *const c_char,
                    errno,
                );
            }
            continue;
        }
        if msg.n.nlmsg_type == NLMSG_ERROR || !nlmsg_ok(&msg.n, rep_len) {
            let errp: *mut nlmsgerr = nlmsg_data(&msg.n) as *mut nlmsgerr;

            fprintf(
                stderr,
                b"fatal reply error,  errno %d\n\0".as_ptr() as *const c_char,
                (*errp).error,
            );
            break;
        }

        print_debug3(
            b"nlmsghdr size=%zu, nlmsg_len=%d, rep_len=%d\n\0".as_ptr() as *const c_char,
            mem::size_of::<nlmsghdr>(),
            msg.n.nlmsg_len as c_int,
            rep_len,
        );

        rep_len = genlmsg_payload(&msg.n);

        na = genlmsg_data(&msg.n) as *mut nlattr;
        len = 0;
        while len < rep_len {
            len += nla_align((*na).nla_len as usize) as c_int;
            let mother: c_int = (*na).nla_type as c_int;

            print_debug1(b"mother=%i\n\0".as_ptr() as *const c_char, mother);
            match (*na).nla_type {
                TASKSTATS_TYPE_AGGR_PID | TASKSTATS_TYPE_AGGR_TGID => {
                    /* For nested attributes, na follows */
                    handle_aggr(mother, na, fd);
                }
                TASKSTATS_TYPE_NULL => {}
                _ => {
                    fprintf(
                        stderr,
                        b"Unexpected nla_type %d\n\0".as_ptr() as *const c_char,
                        (*na).nla_type as c_int,
                    );
                }
            }
            na = (genlmsg_data(&msg.n) as *mut c_char).add(len as usize) as *mut nlattr;
        }
    }

    if maskset != 0 {
        rc = send_cmd(
            nl_sd,
            id,
            mypid,
            TASKSTATS_CMD_GET,
            TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK,
            cpumask.as_mut_ptr() as *mut c_void,
            strlen(cpumask.as_ptr()) as c_int + 1,
        );
        printf(
            b"Sent deregister mask, retval %d\n\0".as_ptr() as *const c_char,
            rc,
        );
        if rc < 0 {
            err(
                rc,
                b"error sending deregister cpumask\n\0".as_ptr() as *const c_char,
            );
        }
    }
    close(nl_sd);
    if fd != 0 {
        close(fd);
    }
    if cfd != 0 {
        close(cfd);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
