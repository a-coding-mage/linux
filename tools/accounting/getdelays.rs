// SPDX-License-Identifier: GPL-2.0
/* getdelays.c
 *
 * Utility to get per-pid and per-tgid delay accounting statistics
 * Also illustrates usage of the taskstats interface
 *
 * Copyright (C) Shailabh Nagar, IBM Corp. 2005
 * Copyright (C) Balbir Singh, IBM Corp. 2006
 * Copyright (c) Jay Lan, SGI. 2006
 *
 * Original C compile command:
 *	gcc -I/usr/src/linux/include getdelays.c -o getdelays
 */

use libc::*;
use std::ffi::CStr;
use std::mem::{size_of, zeroed};
use std::ptr;

// Dependencies from linux/genetlink.h, linux/taskstats.h, and linux/cgroupstats.h
// are expected to be supplied by surrounding bindings.

const MAX_MSG_SIZE: usize = 2048;
const MAX_CPUS: usize = 32;

#[repr(C)]
struct msgtemplate {
    n: nlmsghdr,
    g: genlmsghdr,
    buf: [c_char; MAX_MSG_SIZE],
}

static mut rcvbufsz: c_int = 0;
static mut name: [c_char; 100] = [0; 100];
static mut dbg: c_int = 0;
static mut print_delays: c_int = 0;
static mut print_io_accounting: c_int = 0;
static mut print_task_context_switch_counts: c_int = 0;
static mut cpumask: [c_char; 100 + 6 * MAX_CPUS] = [0; 100 + 6 * MAX_CPUS];

macro_rules! PRINTF {
    ($($arg:tt)*) => {{
        unsafe {
            if dbg != 0 {
                printf($($arg)*);
            }
        }
    }};
}

macro_rules! err {
    ($code:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            fprintf(stderr, $fmt $(, $arg)*);
            exit($code);
        }
    }};
}

unsafe fn genlmsg_data(glh: *mut nlmsghdr) -> *mut c_void {
    (NLMSG_DATA(glh) as *mut c_char).add(GENL_HDRLEN as usize) as *mut c_void
}

unsafe fn genlmsg_payload(glh: *mut nlmsghdr) -> c_int {
    NLMSG_PAYLOAD(glh, 0) as c_int - GENL_HDRLEN as c_int
}

unsafe fn nla_data(na: *mut nlattr) -> *mut c_void {
    (na as *mut c_char).add(NLA_HDRLEN as usize) as *mut c_void
}

fn nla_payload(len: c_int) -> c_int {
    len - NLA_HDRLEN as c_int
}

fn average_ms(t: f64, c: u64) -> f64 {
    t / 1000000.0 / if c != 0 { c as f64 } else { 1.0 }
}

fn delay_ms(t: f64) -> f64 {
    t / 1000000.0
}

unsafe fn usage() {
    fprintf(
        stderr,
        b"getdelays [-dilv] [-w logfile] [-r bufsize] [-m cpumask] [-t tgid] [-p pid]\n\0".as_ptr() as *const c_char,
    );
    fprintf(stderr, b"  -d: print delayacct stats\n\0".as_ptr() as *const c_char);
    fprintf(
        stderr,
        b"  -i: print IO accounting (works only with -p)\n\0".as_ptr() as *const c_char,
    );
    fprintf(stderr, b"  -l: listen forever\n\0".as_ptr() as *const c_char);
    fprintf(stderr, b"  -v: debug on\n\0".as_ptr() as *const c_char);
    fprintf(stderr, b"  -C: container path\n\0".as_ptr() as *const c_char);
}

/*
 * Create a raw netlink socket and bind
 */
unsafe fn create_nl_socket(protocol: c_int) -> c_int {
    let fd: c_int;
    let mut local: sockaddr_nl = zeroed();

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
            size_of::<c_int>() as socklen_t,
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

    local.nl_family = AF_NETLINK as sa_family_t;

    if bind(
        fd,
        &local as *const sockaddr_nl as *const sockaddr,
        size_of::<sockaddr_nl>() as socklen_t,
    ) < 0
    {
        close(fd);
        return -1;
    }

    fd
}

unsafe fn recv_taskstats_msg(sd: c_int, msg: *mut msgtemplate) -> c_int {
    let mut nladdr: sockaddr_nl = zeroed();
    let mut iov = iovec {
        iov_base: msg as *mut c_void,
        iov_len: size_of::<msgtemplate>(),
    };
    let mut hdr: msghdr = zeroed();
    hdr.msg_name = &mut nladdr as *mut sockaddr_nl as *mut c_void;
    hdr.msg_namelen = size_of::<sockaddr_nl>() as socklen_t;
    hdr.msg_iov = &mut iov;
    hdr.msg_iovlen = 1;

    let ret = recvmsg(sd, &mut hdr, 0);
    if ret < 0 {
        return -1;
    }
    if (hdr.msg_flags & MSG_TRUNC) != 0 {
        *__errno_location() = EMSGSIZE;
        return -1;
    }

    ret as c_int
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
    let mut nladdr: sockaddr_nl = zeroed();
    let mut r: c_int;
    let mut buflen: c_int;
    let mut buf: *mut c_char;
    let mut msg: msgtemplate = zeroed();

    msg.n.nlmsg_len = NLMSG_LENGTH(GENL_HDRLEN as c_int) as u32;
    msg.n.nlmsg_type = nlmsg_type;
    msg.n.nlmsg_flags = NLM_F_REQUEST as u16;
    msg.n.nlmsg_seq = 0;
    msg.n.nlmsg_pid = nlmsg_pid;
    msg.g.cmd = genl_cmd;
    msg.g.version = 0x1;
    na = genlmsg_data(&mut msg.n) as *mut nlattr;
    (*na).nla_type = nla_type;
    (*na).nla_len = (nla_len + NLA_HDRLEN as c_int) as u16;
    memcpy(nla_data(na), nla_data_arg, nla_len as usize);
    msg.n.nlmsg_len += NLMSG_ALIGN((*na).nla_len as c_int) as u32;

    buf = &mut msg as *mut msgtemplate as *mut c_char;
    buflen = msg.n.nlmsg_len as c_int;
    nladdr.nl_family = AF_NETLINK as sa_family_t;
    loop {
        r = sendto(
            sd,
            buf as *const c_void,
            buflen as usize,
            0,
            &nladdr as *const sockaddr_nl as *const sockaddr,
            size_of::<sockaddr_nl>() as socklen_t,
        ) as c_int;
        if r >= buflen {
            break;
        }
        if r > 0 {
            buf = buf.add(r as usize);
            buflen -= r;
        } else if *__errno_location() != EAGAIN {
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

    let mut ans: Ans = zeroed();
    let mut id: c_int = 0;
    let rc: c_int;
    let mut na: *mut nlattr;
    let rep_len: c_int;

    strcpy(name.as_mut_ptr(), TASKSTATS_GENL_NAME.as_ptr() as *const c_char);
    rc = send_cmd(
        sd,
        GENL_ID_CTRL as __u16,
        getpid() as __u32,
        CTRL_CMD_GETFAMILY as __u8,
        CTRL_ATTR_FAMILY_NAME as __u16,
        name.as_mut_ptr() as *mut c_void,
        strlen(TASKSTATS_GENL_NAME.as_ptr() as *const c_char) as c_int + 1,
    );
    if rc < 0 {
        return 0; /* sendto() failure? */
    }

    rep_len = recv(sd, &mut ans as *mut Ans as *mut c_void, size_of::<Ans>(), 0) as c_int;
    if ans.n.nlmsg_type == NLMSG_ERROR as u16 || rep_len < 0 || !NLMSG_OK(&mut ans.n, rep_len) {
        return 0;
    }

    na = genlmsg_data(&mut ans.n) as *mut nlattr;
    na = (na as *mut c_char).add(NLA_ALIGN((*na).nla_len as c_int) as usize) as *mut nlattr;
    if (*na).nla_type == CTRL_ATTR_FAMILY_ID as u16 {
        id = *(nla_data(na) as *mut __u16) as c_int;
    }
    id
}

/*
 * Format __kernel_timespec to human readable string (YYYY-MM-DD HH:MM:SS)
 * Returns formatted string or "N/A" if timestamp is zero
 */
unsafe fn format_timespec(ts: *mut __kernel_timespec) -> *const c_char {
    static mut BUFFER: [c_char; 32] = [0; 32];
    let mut tm_info: tm = zeroed();
    let mut time_sec: __kernel_time_t;

    /* Check if timestamp is zero (not set) */
    if (*ts).tv_sec == 0 && (*ts).tv_nsec == 0 {
        return b"N/A\0".as_ptr() as *const c_char;
    }

    time_sec = (*ts).tv_sec;

    /* Use thread-safe localtime_r */
    if localtime_r(&mut time_sec as *mut __kernel_time_t as *mut time_t, &mut tm_info).is_null() {
        return b"N/A\0".as_ptr() as *const c_char;
    }

    strftime(
        BUFFER.as_mut_ptr(),
        BUFFER.len(),
        b"%Y-%m-%dT%H:%M:%S\0".as_ptr() as *const c_char,
        &tm_info,
    );

    BUFFER.as_ptr()
}

unsafe fn print_cpu_delay(version: u16, t: *mut taskstats) {
    if version >= 17 {
        printf(
            b"%-10s%15s%15s%15s%15s%15s%15s%15s%25s\n\0".as_ptr() as *const c_char,
            b"CPU\0".as_ptr(),
            b"count\0".as_ptr(),
            b"real total\0".as_ptr(),
            b"virtual total\0".as_ptr(),
            b"delay total\0".as_ptr(),
            b"delay average\0".as_ptr(),
            b"delay max\0".as_ptr(),
            b"delay min\0".as_ptr(),
            b"delay max timestamp\0".as_ptr(),
        );
        printf(
            b"          %15llu%15llu%15llu%15llu%15.3fms%13.6fms%13.6fms%23s\n\0".as_ptr() as *const c_char,
            (*t).cpu_count as c_ulonglong,
            (*t).cpu_run_real_total as c_ulonglong,
            (*t).cpu_run_virtual_total as c_ulonglong,
            (*t).cpu_delay_total as c_ulonglong,
            average_ms((*t).cpu_delay_total as f64, (*t).cpu_count as u64),
            delay_ms((*t).cpu_delay_max as f64),
            delay_ms((*t).cpu_delay_min as f64),
            format_timespec(&mut (*t).cpu_delay_max_ts),
        );
    } else if version >= 16 {
        printf(
            b"%-10s%15s%15s%15s%15s%15s%15s%15s\n\0".as_ptr() as *const c_char,
            b"CPU\0".as_ptr(),
            b"count\0".as_ptr(),
            b"real total\0".as_ptr(),
            b"virtual total\0".as_ptr(),
            b"delay total\0".as_ptr(),
            b"delay average\0".as_ptr(),
            b"delay max\0".as_ptr(),
            b"delay min\0".as_ptr(),
        );
        printf(
            b"          %15llu%15llu%15llu%15llu%15.3fms%13.6fms%13.6fms\n\0".as_ptr() as *const c_char,
            (*t).cpu_count as c_ulonglong,
            (*t).cpu_run_real_total as c_ulonglong,
            (*t).cpu_run_virtual_total as c_ulonglong,
            (*t).cpu_delay_total as c_ulonglong,
            average_ms((*t).cpu_delay_total as f64, (*t).cpu_count as u64),
            delay_ms((*t).cpu_delay_max as f64),
            delay_ms((*t).cpu_delay_min as f64),
        );
    } else {
        printf(
            b"%-10s%15s%15s%15s%15s%15s\n\0".as_ptr() as *const c_char,
            b"CPU\0".as_ptr(),
            b"count\0".as_ptr(),
            b"real total\0".as_ptr(),
            b"virtual total\0".as_ptr(),
            b"delay total\0".as_ptr(),
            b"delay average\0".as_ptr(),
        );
        printf(
            b"          %15llu%15llu%15llu%15llu%15.3fms\n\0".as_ptr() as *const c_char,
            (*t).cpu_count as c_ulonglong,
            (*t).cpu_run_real_total as c_ulonglong,
            (*t).cpu_run_virtual_total as c_ulonglong,
            (*t).cpu_delay_total as c_ulonglong,
            average_ms((*t).cpu_delay_total as f64, (*t).cpu_count as u64),
        );
    }
}

macro_rules! print_field_delay {
    ($name:expr, $version:expr, $t:expr, $count:ident, $total:ident, $max:ident, $min:ident) => {{
        if $version >= 16 {
            printf(
                b"%-10s%15s%15s%15s%15s%15s\n\0".as_ptr() as *const c_char,
                $name,
                b"count\0".as_ptr(),
                b"delay total\0".as_ptr(),
                b"delay average\0".as_ptr(),
                b"delay max\0".as_ptr(),
                b"delay min\0".as_ptr(),
            );
            printf(
                b"          %15llu%15llu%15.3fms%13.6fms%13.6fms\n\0".as_ptr() as *const c_char,
                (*$t).$count as c_ulonglong,
                (*$t).$total as c_ulonglong,
                average_ms((*$t).$total as f64, (*$t).$count as u64),
                delay_ms((*$t).$max as f64),
                delay_ms((*$t).$min as f64),
            );
        } else {
            printf(
                b"%-10s%15s%15s%15s\n\0".as_ptr() as *const c_char,
                $name,
                b"count\0".as_ptr(),
                b"delay total\0".as_ptr(),
                b"delay average\0".as_ptr(),
            );
            printf(
                b"          %15llu%15llu%15.3fms\n\0".as_ptr() as *const c_char,
                (*$t).$count as c_ulonglong,
                (*$t).$total as c_ulonglong,
                average_ms((*$t).$total as f64, (*$t).$count as u64),
            );
        }
    }};
}

macro_rules! print_field_delay_with_ts {
    ($name:expr, $version:expr, $t:expr, $count:ident, $total:ident, $max:ident, $min:ident, $max_ts:ident) => {{
        if $version >= 17 {
            printf(
                b"%-10s%15s%15s%15s%15s%15s%25s\n\0".as_ptr() as *const c_char,
                $name,
                b"count\0".as_ptr(),
                b"delay total\0".as_ptr(),
                b"delay average\0".as_ptr(),
                b"delay max\0".as_ptr(),
                b"delay min\0".as_ptr(),
                b"delay max timestamp\0".as_ptr(),
            );
            printf(
                b"          %15llu%15llu%15.3fms%13.6fms%13.6fms%23s\n\0".as_ptr() as *const c_char,
                (*$t).$count as c_ulonglong,
                (*$t).$total as c_ulonglong,
                average_ms((*$t).$total as f64, (*$t).$count as u64),
                delay_ms((*$t).$max as f64),
                delay_ms((*$t).$min as f64),
                format_timespec(&mut (*$t).$max_ts),
            );
        } else {
            print_field_delay!($name, $version, $t, $count, $total, $max, $min);
        }
    }};
}

unsafe fn print_delayacct(t: *mut taskstats) {
    printf(b"\n\n\0".as_ptr() as *const c_char);

    print_cpu_delay((*t).version, t);

    /* Use new macro with timestamp support for version >= 17 */
    if (*t).version >= 17 {
        print_field_delay_with_ts!(b"IO\0".as_ptr(), (*t).version, t, blkio_count, blkio_delay_total, blkio_delay_max, blkio_delay_min, blkio_delay_max_ts);
        print_field_delay_with_ts!(b"SWAP\0".as_ptr(), (*t).version, t, swapin_count, swapin_delay_total, swapin_delay_max, swapin_delay_min, swapin_delay_max_ts);
        print_field_delay_with_ts!(b"RECLAIM\0".as_ptr(), (*t).version, t, freepages_count, freepages_delay_total, freepages_delay_max, freepages_delay_min, freepages_delay_max_ts);
        print_field_delay_with_ts!(b"THRASHING\0".as_ptr(), (*t).version, t, thrashing_count, thrashing_delay_total, thrashing_delay_max, thrashing_delay_min, thrashing_delay_max_ts);

        if (*t).version >= 11 {
            print_field_delay_with_ts!(b"COMPACT\0".as_ptr(), (*t).version, t, compact_count, compact_delay_total, compact_delay_max, compact_delay_min, compact_delay_max_ts);
        }

        if (*t).version >= 13 {
            print_field_delay_with_ts!(b"WPCOPY\0".as_ptr(), (*t).version, t, wpcopy_count, wpcopy_delay_total, wpcopy_delay_max, wpcopy_delay_min, wpcopy_delay_max_ts);
        }

        if (*t).version >= 14 {
            print_field_delay_with_ts!(b"IRQ\0".as_ptr(), (*t).version, t, irq_count, irq_delay_total, irq_delay_max, irq_delay_min, irq_delay_max_ts);
        }
    } else {
        /* Use original macro for older versions */
        print_field_delay!(b"IO\0".as_ptr(), (*t).version, t, blkio_count, blkio_delay_total, blkio_delay_max, blkio_delay_min);
        print_field_delay!(b"SWAP\0".as_ptr(), (*t).version, t, swapin_count, swapin_delay_total, swapin_delay_max, swapin_delay_min);
        print_field_delay!(b"RECLAIM\0".as_ptr(), (*t).version, t, freepages_count, freepages_delay_total, freepages_delay_max, freepages_delay_min);
        print_field_delay!(b"THRASHING\0".as_ptr(), (*t).version, t, thrashing_count, thrashing_delay_total, thrashing_delay_max, thrashing_delay_min);

        if (*t).version >= 11 {
            print_field_delay!(b"COMPACT\0".as_ptr(), (*t).version, t, compact_count, compact_delay_total, compact_delay_max, compact_delay_min);
        }

        if (*t).version >= 13 {
            print_field_delay!(b"WPCOPY\0".as_ptr(), (*t).version, t, wpcopy_count, wpcopy_delay_total, wpcopy_delay_max, wpcopy_delay_min);
        }

        if (*t).version >= 14 {
            print_field_delay!(b"IRQ\0".as_ptr(), (*t).version, t, irq_count, irq_delay_total, irq_delay_max, irq_delay_min);
        }
    }
}

unsafe fn task_context_switch_counts(t: *mut taskstats) {
    printf(
        b"\n\nTask   %15s%15s\n       %15llu%15llu\n\0".as_ptr() as *const c_char,
        b"voluntary\0".as_ptr(),
        b"nonvoluntary\0".as_ptr(),
        (*t).nvcsw as c_ulonglong,
        (*t).nivcsw as c_ulonglong,
    );
}

unsafe fn print_cgroupstats(c: *mut cgroupstats) {
    printf(
        b"sleeping %llu, blocked %llu, running %llu, stopped %llu, uninterruptible %llu\n\0".as_ptr() as *const c_char,
        (*c).nr_sleeping as c_ulonglong,
        (*c).nr_io_wait as c_ulonglong,
        (*c).nr_running as c_ulonglong,
        (*c).nr_stopped as c_ulonglong,
        (*c).nr_uninterruptible as c_ulonglong,
    );
}

unsafe fn print_ioacct(t: *mut taskstats) {
    printf(
        b"%s: read=%llu, write=%llu, cancelled_write=%llu\n\0".as_ptr() as *const c_char,
        (*t).ac_comm.as_ptr(),
        (*t).read_bytes as c_ulonglong,
        (*t).write_bytes as c_ulonglong,
        (*t).cancelled_write_bytes as c_ulonglong,
    );
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut c: c_int;
    let mut rc: c_int;
    let mut rep_len: c_int;
    let mut aggr_len: c_int;
    let mut len2: c_int;
    let mut cmd_type: c_int = TASKSTATS_CMD_ATTR_UNSPEC;
    let mut id: __u16;
    let mut mypid: __u32;

    let mut na: *mut nlattr;
    let mut nl_sd: c_int = -1;
    let mut len: c_int;
    let mut tid: pid_t = 0;
    let mut rtid: pid_t = 0;

    let mut fd: c_int = 0;
    let mut write_file: c_int = 0;
    let mut maskset: c_int = 0;
    let mut logfile: *mut c_char = ptr::null_mut();
    let mut loop_: c_int = 0;
    let mut containerset: c_int = 0;
    let mut containerpath: *mut c_char = ptr::null_mut();
    let mut cfd: c_int = 0;
    let mut forking: c_int = 0;
    let mut sigset: sigset_t = zeroed();

    let mut msg: msgtemplate = zeroed();

    while forking == 0 {
        c = getopt(argc, argv, b"qdiw:r:m:t:p:vlC:c:\0".as_ptr() as *const c_char);
        if c < 0 {
            break;
        }

        match c as u8 as c_char {
            b'd' as c_char => {
                printf(b"print delayacct stats ON\n\0".as_ptr() as *const c_char);
                print_delays = 1;
            }
            b'i' as c_char => {
                printf(b"printing IO accounting\n\0".as_ptr() as *const c_char);
                print_io_accounting = 1;
            }
            b'q' as c_char => {
                printf(b"printing task/process context switch rates\n\0".as_ptr() as *const c_char);
                print_task_context_switch_counts = 1;
            }
            b'C' as c_char => {
                containerset = 1;
                containerpath = optarg;
            }
            b'w' as c_char => {
                logfile = strdup(optarg);
                printf(b"write to file %s\n\0".as_ptr() as *const c_char, logfile);
                write_file = 1;
            }
            b'r' as c_char => {
                rcvbufsz = atoi(optarg);
                printf(b"receive buf size %d\n\0".as_ptr() as *const c_char, rcvbufsz);
                if rcvbufsz < 0 {
                    err!(1, b"Invalid rcv buf size\n\0".as_ptr() as *const c_char);
                }
            }
            b'm' as c_char => {
                strncpy(cpumask.as_mut_ptr(), optarg, cpumask.len());
                cpumask[cpumask.len() - 1] = 0;
                maskset = 1;
                printf(
                    b"cpumask %s maskset %d\n\0".as_ptr() as *const c_char,
                    cpumask.as_ptr(),
                    maskset,
                );
            }
            b't' as c_char => {
                tid = atoi(optarg);
                if tid == 0 {
                    err!(1, b"Invalid tgid\n\0".as_ptr() as *const c_char);
                }
                cmd_type = TASKSTATS_CMD_ATTR_TGID;
            }
            b'p' as c_char => {
                tid = atoi(optarg);
                if tid == 0 {
                    err!(1, b"Invalid pid\n\0".as_ptr() as *const c_char);
                }
                cmd_type = TASKSTATS_CMD_ATTR_PID;
            }
            b'c' as c_char => {
                /* Block SIGCHLD for sigwait() later */
                if sigemptyset(&mut sigset) == -1 {
                    err!(1, b"Failed to empty sigset\0".as_ptr() as *const c_char);
                }
                if sigaddset(&mut sigset, SIGCHLD) != 0 {
                    err!(1, b"Failed to set sigchld in sigset\0".as_ptr() as *const c_char);
                }
                sigprocmask(SIG_BLOCK, &mut sigset, ptr::null_mut());

                /* fork/exec a child */
                tid = fork();
                if tid < 0 {
                    err!(1, b"Fork failed\n\0".as_ptr() as *const c_char);
                }
                if tid == 0 {
                    if execvp(*argv.offset(optind as isize - 1), argv.offset(optind as isize - 1)) < 0 {
                        exit(-1);
                    }
                }

                /* Set the command type and avoid further processing */
                cmd_type = TASKSTATS_CMD_ATTR_PID;
                forking = 1;
            }
            b'v' as c_char => {
                printf(b"debug on\n\0".as_ptr() as *const c_char);
                dbg = 1;
            }
            b'l' as c_char => {
                printf(b"listen forever\n\0".as_ptr() as *const c_char);
                loop_ = 1;
            }
            _ => {
                usage();
                exit(-1);
            }
        }
    }

    if write_file != 0 {
        fd = open(
            logfile,
            O_WRONLY | O_CREAT | O_TRUNC,
            S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH,
        );
        if fd == -1 {
            perror(b"Cannot open output file\n\0".as_ptr() as *const c_char);
            exit(1);
        }
    }

    nl_sd = create_nl_socket(NETLINK_GENERIC);
    if nl_sd < 0 {
        err!(1, b"error creating Netlink socket\n\0".as_ptr() as *const c_char);
    }

    mypid = getpid() as __u32;
    id = get_family_id(nl_sd) as __u16;
    if id == 0 {
        fprintf(
            stderr,
            b"Error getting family id, errno %d\n\0".as_ptr() as *const c_char,
            *__errno_location(),
        );
        goto_err(nl_sd, fd, cfd);
        return 0;
    }
    PRINTF!(b"family id %d\n\0".as_ptr() as *const c_char, id as c_int);

    if maskset != 0 {
        rc = send_cmd(
            nl_sd,
            id,
            mypid,
            TASKSTATS_CMD_GET as __u8,
            TASKSTATS_CMD_ATTR_REGISTER_CPUMASK as __u16,
            cpumask.as_mut_ptr() as *mut c_void,
            strlen(cpumask.as_ptr()) as c_int + 1,
        );
        PRINTF!(b"Sent register cpumask, retval %d\n\0".as_ptr() as *const c_char, rc);
        if rc < 0 {
            fprintf(stderr, b"error sending register cpumask\n\0".as_ptr() as *const c_char);
            goto_err(nl_sd, fd, cfd);
            return 0;
        }
    }

    if tid != 0 && containerset != 0 {
        fprintf(stderr, b"Select either -t or -C, not both\n\0".as_ptr() as *const c_char);
        goto_err(nl_sd, fd, cfd);
        return 0;
    }

    /*
     * If we forked a child, wait for it to exit. Cannot use waitpid()
     * as all the delicious data would be reaped as part of the wait
     */
    if tid != 0 && forking != 0 {
        let mut sig_received: c_int = 0;
        sigwait(&mut sigset, &mut sig_received);
    }

    if tid != 0 {
        rc = send_cmd(
            nl_sd,
            id,
            mypid,
            TASKSTATS_CMD_GET as __u8,
            cmd_type as __u16,
            &mut tid as *mut pid_t as *mut c_void,
            size_of::<__u32>() as c_int,
        );
        PRINTF!(b"Sent pid/tgid, retval %d\n\0".as_ptr() as *const c_char, rc);
        if rc < 0 {
            fprintf(stderr, b"error sending tid/tgid cmd\n\0".as_ptr() as *const c_char);
            done(maskset, nl_sd, id, mypid, fd, cfd);
            return 0;
        }
    }

    if containerset != 0 {
        cfd = open(containerpath, O_RDONLY);
        if cfd < 0 {
            perror(b"error opening container file\0".as_ptr() as *const c_char);
            goto_err(nl_sd, fd, cfd);
            return 0;
        }
        rc = send_cmd(
            nl_sd,
            id,
            mypid,
            CGROUPSTATS_CMD_GET as __u8,
            CGROUPSTATS_CMD_ATTR_FD as __u16,
            &mut cfd as *mut c_int as *mut c_void,
            size_of::<__u32>() as c_int,
        );
        if rc < 0 {
            perror(b"error sending cgroupstats command\0".as_ptr() as *const c_char);
            goto_err(nl_sd, fd, cfd);
            return 0;
        }
    }
    if maskset == 0 && tid == 0 && containerset == 0 {
        usage();
        goto_err(nl_sd, fd, cfd);
        return 0;
    }

    loop {
        rep_len = recv_taskstats_msg(nl_sd, &mut msg);
        PRINTF!(b"received %d bytes\n\0".as_ptr() as *const c_char, rep_len);

        if rep_len < 0 {
            if *__errno_location() == EMSGSIZE {
                fprintf(
                    stderr,
                    b"dropped truncated taskstats netlink message, please increase MAX_MSG_SIZE\n\0".as_ptr() as *const c_char,
                );
            } else {
                fprintf(
                    stderr,
                    b"nonfatal reply error: errno %d\n\0".as_ptr() as *const c_char,
                    *__errno_location(),
                );
            }
            if loop_ == 0 {
                break;
            }
            continue;
        }
        if msg.n.nlmsg_type == NLMSG_ERROR as u16 || !NLMSG_OK(&mut msg.n, rep_len) {
            let errp = NLMSG_DATA(&mut msg.n) as *mut nlmsgerr;
            fprintf(
                stderr,
                b"fatal reply error,  errno %d\n\0".as_ptr() as *const c_char,
                (*errp).error,
            );
            break;
        }

        PRINTF!(
            b"nlmsghdr size=%zu, nlmsg_len=%d, rep_len=%d\n\0".as_ptr() as *const c_char,
            size_of::<nlmsghdr>(),
            msg.n.nlmsg_len,
            rep_len
        );

        rep_len = genlmsg_payload(&mut msg.n);

        na = genlmsg_data(&mut msg.n) as *mut nlattr;
        len = 0;
        while len < rep_len {
            len += NLA_ALIGN((*na).nla_len as c_int) as c_int;
            match (*na).nla_type as c_int {
                TASKSTATS_TYPE_AGGR_TGID | TASKSTATS_TYPE_AGGR_PID => {
                    aggr_len = nla_payload((*na).nla_len as c_int);
                    len2 = 0;
                    /* For nested attributes, na follows */
                    na = nla_data(na) as *mut nlattr;
                    while len2 < aggr_len {
                        match (*na).nla_type as c_int {
                            TASKSTATS_TYPE_PID => {
                                rtid = *(nla_data(na) as *mut c_int);
                                if print_delays != 0 {
                                    printf(b"PID\t%d\n\0".as_ptr() as *const c_char, rtid);
                                }
                            }
                            TASKSTATS_TYPE_TGID => {
                                rtid = *(nla_data(na) as *mut c_int);
                                if print_delays != 0 {
                                    printf(b"TGID\t%d\n\0".as_ptr() as *const c_char, rtid);
                                }
                            }
                            TASKSTATS_TYPE_STATS => {
                                let stats = nla_data(na) as *mut taskstats;
                                PRINTF!(
                                    b"version %u\n\0".as_ptr() as *const c_char,
                                    (*stats).version as c_uint
                                );
                                if print_delays != 0 {
                                    print_delayacct(stats);
                                }
                                if print_io_accounting != 0 {
                                    print_ioacct(stats);
                                }
                                if print_task_context_switch_counts != 0 {
                                    task_context_switch_counts(stats);
                                }
                                if fd != 0 {
                                    if write(fd, nla_data(na), (*na).nla_len as usize) < 0 {
                                        err!(1, b"write error\n\0".as_ptr() as *const c_char);
                                    }
                                }
                                if loop_ == 0 {
                                    done(maskset, nl_sd, id, mypid, fd, cfd);
                                    return 0;
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
                        len2 += NLA_ALIGN((*na).nla_len as c_int) as c_int;
                        na = (na as *mut c_char)
                            .add(NLA_ALIGN((*na).nla_len as c_int) as usize)
                            as *mut nlattr;
                    }
                }
                CGROUPSTATS_TYPE_CGROUP_STATS => {
                    print_cgroupstats(nla_data(na) as *mut cgroupstats);
                }
                TASKSTATS_TYPE_NULL => {}
                _ => {
                    fprintf(
                        stderr,
                        b"Unknown nla_type %d\n\0".as_ptr() as *const c_char,
                        (*na).nla_type as c_int,
                    );
                }
            }
            na = (genlmsg_data(&mut msg.n) as *mut c_char).add(len as usize) as *mut nlattr;
        }
        if loop_ == 0 {
            break;
        }
    }
    done(maskset, nl_sd, id, mypid, fd, cfd);
    0
}

unsafe fn done(maskset: c_int, nl_sd: c_int, id: __u16, mypid: __u32, fd: c_int, cfd: c_int) {
    if maskset != 0 {
        let rc = send_cmd(
            nl_sd,
            id,
            mypid,
            TASKSTATS_CMD_GET as __u8,
            TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK as __u16,
            cpumask.as_mut_ptr() as *mut c_void,
            strlen(cpumask.as_ptr()) as c_int + 1,
        );
        printf(b"Sent deregister mask, retval %d\n\0".as_ptr() as *const c_char, rc);
        if rc < 0 {
            err!(rc, b"error sending deregister cpumask\n\0".as_ptr() as *const c_char);
        }
    }
    goto_err(nl_sd, fd, cfd);
}

unsafe fn goto_err(nl_sd: c_int, fd: c_int, cfd: c_int) {
    close(nl_sd);
    if fd != 0 {
        close(fd);
    }
    if cfd != 0 {
        close(cfd);
    }
}

fn main() {
    unsafe {
        let args: Vec<std::ffi::CString> = std::env::args()
            .map(|s| std::ffi::CString::new(s).unwrap())
            .collect();
        let mut argv: Vec<*mut c_char> = args.iter().map(|s| s.as_ptr() as *mut c_char).collect();
        argv.push(ptr::null_mut());
        std::process::exit(main_0((argv.len() - 1) as c_int, argv.as_mut_ptr()));
    }
}
