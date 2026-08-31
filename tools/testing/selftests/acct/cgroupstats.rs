// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of testing/selftests/acct/cgroupstats.c.
// C include dependencies represented here by the C ABI items and Linux layout
// definitions used directly by this file.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;

const EIO: c_int = 5;
const EBADF: c_int = 9;

const O_RDONLY: c_int = 0;
const O_DIRECTORY: c_int = 0o200000;
const MNT_DETACH: c_int = 2;

const NLMSG_NOOP: u16 = 1;
const NLMSG_ERROR: u16 = 2;
const NLM_F_REQUEST: c_int = 0x01;
const NLM_F_ACK: c_int = 0x04;

const NLMSG_ALIGNTO: u32 = 4;
const NLA_ALIGNTO: u16 = 4;

const TASKSTATS_GENL_NAME: *const c_char = b"TASKSTATS\0".as_ptr() as *const c_char;

const CGROUPSTATS_CMD_GET: u8 = 4;
const CGROUPSTATS_CMD_NEW: u8 = 5;
const CGROUPSTATS_CMD_ATTR_FD: u16 = 1;
const CGROUPSTATS_TYPE_CGROUP_STATS: u16 = 1;

const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;

#[repr(C)]
struct nlmsghdr {
    nlmsg_len: u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: u32,
    nlmsg_pid: u32,
}

#[repr(C)]
struct nlmsgerr {
    error: c_int,
    msg: nlmsghdr,
}

#[repr(C)]
struct genlmsghdr {
    cmd: u8,
    version: u8,
    reserved: u16,
}

#[repr(C)]
struct nlattr {
    nla_len: u16,
    nla_type: u16,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
struct cgroupstats {
    nr_sleeping: u64,
    nr_running: u64,
    nr_stopped: u64,
    nr_uninterruptible: u64,
    nr_io_wait: u64,
}

#[repr(C)]
struct cgroupstats_req {
    nlh: nlmsghdr,
    genl: genlmsghdr,
    buf: [c_char; 256],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn close(fd: c_int) -> c_int;
    fn getpid() -> pid_t;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: usize,
        data: *const c_void,
    ) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;

    fn send_request(fd: c_int, req: *const c_void, len: u32) -> c_int;
    fn netlink_open() -> c_int;
    fn get_family_id(fd: c_int, family_name: *const c_char) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result(condition: bool, fmt: *const c_char, ...);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_finished();
    fn ksft_get_fail_cnt() -> c_int;
}

const fn nlmsg_align(len: u32) -> u32 {
    (len + NLMSG_ALIGNTO - 1) & !(NLMSG_ALIGNTO - 1)
}

const fn nla_align(len: u16) -> u16 {
    (len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}

const NLMSG_HDRLEN: u32 = nlmsg_align(mem::size_of::<nlmsghdr>() as u32);
const GENL_HDRLEN: u32 = nlmsg_align(mem::size_of::<genlmsghdr>() as u32);
const NLA_HDRLEN: u16 = nla_align(mem::size_of::<nlattr>() as u16);

const fn nlmsg_length(len: u32) -> u32 {
    len + NLMSG_HDRLEN
}

unsafe fn nlmsg_data(nlh: *mut nlmsghdr) -> *mut c_void {
    (nlh as *mut u8).add(NLMSG_HDRLEN as usize) as *mut c_void
}

unsafe fn nlmsg_ok(nlh: *mut nlmsghdr, len: c_int) -> bool {
    len >= NLMSG_HDRLEN as c_int
        && (*nlh).nlmsg_len >= NLMSG_HDRLEN
        && (*nlh).nlmsg_len <= len as u32
}

unsafe fn nlmsg_next(nlh: *mut nlmsghdr, len: *mut c_int) -> *mut nlmsghdr {
    let aligned = nlmsg_align((*nlh).nlmsg_len) as c_int;
    *len -= aligned;
    (nlh as *mut u8).add(aligned as usize) as *mut nlmsghdr
}

unsafe fn nla_data(na: *mut nlattr) -> *mut c_void {
    (na as *mut u8).add(NLA_HDRLEN as usize) as *mut c_void
}

unsafe fn nla_ok(na: *mut nlattr, rem: c_int) -> bool {
    rem >= mem::size_of::<nlattr>() as c_int
        && (*na).nla_len >= mem::size_of::<nlattr>() as u16
        && (*na).nla_len as c_int <= rem
}

unsafe fn nla_next(na: *mut nlattr, rem: *mut c_int) -> *mut nlattr {
    let totlen = nla_align((*na).nla_len) as c_int;
    *rem -= totlen;
    (na as *mut u8).add(totlen as usize) as *mut nlattr
}

unsafe fn send_cgroupstats_cmd(
    fd: c_int,
    family_id: c_int,
    cgroup_fd: u32,
    flags: c_int,
) -> c_int {
    let mut req: cgroupstats_req = mem::zeroed();
    let na: *mut nlattr;

    req.nlh.nlmsg_len = nlmsg_length(GENL_HDRLEN);
    req.nlh.nlmsg_type = family_id as u16;
    req.nlh.nlmsg_flags = (NLM_F_REQUEST | flags) as u16;
    req.nlh.nlmsg_seq = 2;
    req.nlh.nlmsg_pid = getpid() as u32;

    req.genl.cmd = CGROUPSTATS_CMD_GET;
    req.genl.version = 1;

    na = (&mut req as *mut cgroupstats_req as *mut u8)
        .add(nlmsg_align(req.nlh.nlmsg_len) as usize) as *mut nlattr;
    (*na).nla_type = CGROUPSTATS_CMD_ATTR_FD;
    (*na).nla_len = NLA_HDRLEN + mem::size_of_val(&cgroup_fd) as u16;
    memcpy(
        nla_data(na),
        &cgroup_fd as *const u32 as *const c_void,
        mem::size_of_val(&cgroup_fd),
    );
    req.nlh.nlmsg_len = nlmsg_align(req.nlh.nlmsg_len) + nla_align((*na).nla_len) as u32;

    send_request(fd, &req as *const cgroupstats_req as *const c_void, req.nlh.nlmsg_len)
}

/*
 * Receive and decode a cgroupstats response.
 *
 * Returns:
 *   0             - success, stats filled from CGROUPSTATS_CMD_NEW reply
 *   <0            - NLMSG_ERROR errno (e.g. -EBADF, -EINVAL)
 */
unsafe fn recv_cgroupstats_response(fd: c_int, stats: *mut cgroupstats) -> c_int {
    let mut resp = [0 as c_char; 8192];
    let mut nlh: *mut nlmsghdr;
    let mut genl: *mut genlmsghdr;
    let mut na: *mut nlattr;
    let mut len: c_int;
    let mut rem: c_int;

    memset(
        stats as *mut c_void,
        0,
        mem::size_of::<cgroupstats>(),
    );

    len = recv(fd, resp.as_mut_ptr() as *mut c_void, resp.len(), 0) as c_int;
    if len < 0 {
        return -errno;
    }

    nlh = resp.as_mut_ptr() as *mut nlmsghdr;
    while nlmsg_ok(nlh, len) {
        if (*nlh).nlmsg_type == NLMSG_ERROR {
            let err: *mut nlmsgerr = nlmsg_data(nlh) as *mut nlmsgerr;

            return (*err).error;
        }

        genl = nlmsg_data(nlh) as *mut genlmsghdr;
        if (*genl).cmd == CGROUPSTATS_CMD_NEW {
            rem = (*nlh).nlmsg_len as c_int - NLMSG_HDRLEN as c_int - GENL_HDRLEN as c_int;
            na = (genl as *mut u8).add(GENL_HDRLEN as usize) as *mut nlattr;
            while nla_ok(na, rem) {
                if (*na).nla_type == CGROUPSTATS_TYPE_CGROUP_STATS {
                    memcpy(
                        stats as *mut c_void,
                        nla_data(na),
                        mem::size_of::<cgroupstats>(),
                    );
                    return 0;
                }
                na = nla_next(na, &mut rem);
            }
        }

        nlh = nlmsg_next(nlh, &mut len);
    }

    -EIO
}

/* mkdtemp() modifies the template in place, so this cannot be const. */
static mut cg_mountpoint: [c_char; 32] = [0; 32];
static mut cg_mounted: bool = false;

unsafe fn setup_cgroup_v1() -> c_int {
    strcpy(
        cg_mountpoint.as_mut_ptr(),
        b"/tmp/cgstats_test_XXXXXX\0".as_ptr() as *const c_char,
    );

    if mkdtemp(cg_mountpoint.as_mut_ptr()).is_null() {
        return -errno;
    }

    if mount(
        b"cgstats_test\0".as_ptr() as *const c_char,
        cg_mountpoint.as_ptr(),
        b"cgroup\0".as_ptr() as *const c_char,
        0,
        b"none,name=cgstats_test\0".as_ptr() as *const c_void,
    ) < 0
    {
        let ret = -errno;

        rmdir(cg_mountpoint.as_ptr());
        return ret;
    }

    cg_mounted = true;
    0
}

unsafe fn cleanup_cgroup_v1() {
    if !cg_mounted {
        return;
    }
    umount2(cg_mountpoint.as_ptr(), MNT_DETACH);
    rmdir(cg_mountpoint.as_ptr());
    cg_mounted = false;
}

fn main() {
    unsafe {
        let mut stats: cgroupstats = mem::zeroed();
        let mut total_tasks: u64;
        let family_id: c_int;
        let nl_fd: c_int;
        let mut cg_fd: c_int;
        let mut ret: c_int;

        ksft_print_header();

        nl_fd = netlink_open();
        if nl_fd < 0 {
            ksft_exit_skip(
                b"failed to open generic netlink socket: %s\n\0".as_ptr() as *const c_char,
                strerror(-nl_fd),
            );
        }

        family_id = get_family_id(nl_fd, TASKSTATS_GENL_NAME);
        if family_id < 0 {
            ksft_exit_skip(
                b"taskstats generic netlink family unavailable: %s\n\0".as_ptr() as *const c_char,
                strerror(-family_id),
            );
        }

        ksft_set_plan(3);

        /*
         * Test 1: mount a private cgroup v1 hierarchy, query it, and
         * verify the response contains sane task counts. If the test
         * environment cannot create a private cgroup v1 mount, skip this
         * case and continue with the unprivileged regression checks below.
         */
        ret = setup_cgroup_v1();
        if ret != 0 {
            ksft_test_result_skip(
                b"cgroupstats query: cannot mount cgroup v1: %s\n\0".as_ptr() as *const c_char,
                strerror(-ret),
            );
        } else {
            cg_fd = open(cg_mountpoint.as_ptr(), O_RDONLY | O_DIRECTORY);
            if cg_fd < 0 {
                ksft_test_result_fail(
                    b"cgroupstats query: open mountpoint: %s\n\0".as_ptr() as *const c_char,
                    strerror(errno),
                );
            } else {
                ret = send_cgroupstats_cmd(nl_fd, family_id, cg_fd as u32, 0);
                if ret != 0 {
                    ksft_test_result_fail(
                        b"cgroupstats query: send: %s\n\0".as_ptr() as *const c_char,
                        strerror(-ret),
                    );
                } else {
                    ret = recv_cgroupstats_response(nl_fd, &mut stats);
                    if ret < 0 {
                        ksft_test_result_fail(
                            b"cgroupstats query: %s\n\0".as_ptr() as *const c_char,
                            strerror(-ret),
                        );
                    } else {
                        total_tasks = stats.nr_sleeping
                            + stats.nr_running
                            + stats.nr_stopped
                            + stats.nr_uninterruptible
                            + stats.nr_io_wait;

                        ksft_print_msg(
                            b"cgroupstats query: total_tasks=%llu\n\0".as_ptr() as *const c_char,
                            total_tasks as u64,
                        );

                        ksft_test_result(
                            total_tasks > 0,
                            b"cgroupstats query returns valid stats\n\0".as_ptr()
                                as *const c_char,
                        );
                    }
                }
                close(cg_fd);
            }
        }
        cleanup_cgroup_v1();

        /*
         * Test 2: invalid fd without NLM_F_ACK.  The kernel should
         * return -EBADF via NLMSG_ERROR regardless of whether the
         * client requested an explicit ACK.
         */
        ret = send_cgroupstats_cmd(nl_fd, family_id, 0xFFFFFFFF, 0);
        if ret != 0 {
            ksft_exit_fail_msg(
                b"send test 2 failed: %s\n\0".as_ptr() as *const c_char,
                strerror(-ret),
            );
        }

        ret = recv_cgroupstats_response(nl_fd, &mut stats);
        ksft_print_msg(
            b"bad fd (no ACK): response=%d (%s)\n\0".as_ptr() as *const c_char,
            ret,
            if ret < 0 {
                strerror(-ret)
            } else {
                b"unexpected success\0".as_ptr() as *mut c_char
            },
        );
        ksft_test_result(
            ret == -EBADF,
            b"cgroupstats rejects bad fd without NLM_F_ACK\n\0".as_ptr() as *const c_char,
        );

        /*
         * Test 3: invalid fd with NLM_F_ACK.  Same expectation as
         * test 2, but exercised through a different netlink flag
         * path in the kernel's ack/error handling.
         */
        ret = send_cgroupstats_cmd(nl_fd, family_id, 0xFFFFFFFF, NLM_F_ACK);
        if ret != 0 {
            ksft_exit_fail_msg(
                b"send test 3 failed: %s\n\0".as_ptr() as *const c_char,
                strerror(-ret),
            );
        }

        ret = recv_cgroupstats_response(nl_fd, &mut stats);
        ksft_print_msg(
            b"bad fd (with ACK): response=%d (%s)\n\0".as_ptr() as *const c_char,
            ret,
            if ret < 0 {
                strerror(-ret)
            } else {
                b"unexpected success\0".as_ptr() as *mut c_char
            },
        );
        ksft_test_result(
            ret == -EBADF,
            b"cgroupstats rejects bad fd with NLM_F_ACK\n\0".as_ptr() as *const c_char,
        );

        close(nl_fd);
        ksft_finished();
        std::process::exit(if ksft_get_fail_cnt() != 0 {
            KSFT_FAIL
        } else {
            KSFT_PASS
        });
    }
}
