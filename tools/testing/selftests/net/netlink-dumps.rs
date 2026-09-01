// SPDX-License-Identifier: GPL-2.0

// C source dependencies:
// _GNU_SOURCE, fcntl.h, stdio.h, string.h, sys/socket.h, sys/stat.h,
// sys/syscall.h, sys/types.h, unistd.h, linux/genetlink.h,
// linux/neighbour.h, linux/netdevice.h, linux/netlink.h, linux/mqueue.h,
// linux/rtnetlink.h, kselftest_harness.h, ynl.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type __u16 = u16;
type __u32 = u32;

#[repr(C)]
pub struct nlmsghdr {
    pub nlmsg_len: __u32,
    pub nlmsg_type: __u16,
    pub nlmsg_flags: __u16,
    pub nlmsg_seq: __u32,
    pub nlmsg_pid: __u32,
}

#[repr(C)]
pub struct nlattr {
    pub nla_len: __u16,
    pub nla_type: __u16,
}

#[repr(C)]
pub struct ndmsg {
    pub ndm_family: u8,
    pub ndm_pad1: u8,
    pub ndm_pad2: __u16,
    pub ndm_ifindex: c_int,
    pub ndm_state: __u16,
    pub ndm_flags: u8,
    pub ndm_type: u8,
}

#[repr(C)]
pub struct genlmsghdr {
    pub cmd: u8,
    pub version: u8,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: c_int,
    pub sival_ptr: *mut c_void,
}

#[repr(C)]
pub struct sigevent {
    pub sigev_value: sigval,
    pub sigev_signo: c_int,
    pub sigev_notify: c_int,
    pub _sigev_un: [usize; 12],
}

#[repr(C)]
pub struct ext_ack {
    pub err: c_int,
    pub attr_offs: __u32,
    pub miss_type: __u32,
    pub miss_nest: __u32,
    pub str_: *const c_char,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum get_ea_ret {
    ERROR = -1,
    NO_CTRL = 0,
    FOUND_DONE,
    FOUND_ERR,
    FOUND_EXTACK,
}

const AF_NETLINK: c_int = 16;
const SOCK_RAW: c_int = 3;
const NETLINK_ROUTE: c_int = 0;
const NETLINK_GENERIC: c_int = 16;
const SOL_NETLINK: c_int = 270;
const NETLINK_CAP_ACK: c_int = 10;
const NETLINK_EXT_ACK: c_int = 11;
const NETLINK_GET_STRICT_CHK: c_int = 12;
const MSG_DONTWAIT: c_int = 0x40;
const NLMSG_ERROR: __u16 = 0x2;
const NLMSG_DONE: __u16 = 0x3;
const NLM_F_REQUEST: __u16 = 0x01;
const NLM_F_ACK: __u16 = 0x04;
const NLM_F_DUMP: __u16 = 0x300;
const NLM_F_ACK_TLVS: __u16 = 0x200;
const RTM_GETNEIGH: __u16 = 30;
const NDA_FLAGS_EXT: __u16 = 8;
const GENL_ID_CTRL: __u16 = 0x10;
const CTRL_CMD_GETPOLICY: u8 = 10;
const CTRL_ATTR_FAMILY_ID: __u16 = 1;
const NLMSGERR_ATTR_MSG: __u16 = 1;
const NLMSGERR_ATTR_OFFS: __u16 = 2;
const NLMSGERR_ATTR_MISS_TYPE: __u16 = 3;
const NLMSGERR_ATTR_MISS_NEST: __u16 = 4;
const ENOBUFS: c_int = 105;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const O_CREAT: c_int = 0o100;
const O_WRONLY: c_int = 0o1;
const SIGEV_THREAD: c_int = 2;
const NOTIFY_COOKIE_LEN: usize = 32;
const __NR_mq_open: c_long = 240;
const __NR_mq_notify: c_long = 244;

unsafe extern "C" {
    static mut errno: c_int;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: c_uint,
    ) -> c_int;
    fn send(socket: c_int, buffer: *const c_void, length: size_t, flags: c_int) -> ssize_t;
    fn recv(socket: c_int, buffer: *mut c_void, length: size_t, flags: c_int) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn usleep(usec: c_uint) -> c_int;

    fn ynl_attr_type(attr: *const nlattr) -> __u16;
    fn ynl_attr_get_u32(attr: *const nlattr) -> __u32;
    fn ynl_attr_get_str(attr: *const nlattr) -> *const c_char;
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

const fn nlmsg_align(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

unsafe fn NLMSG_DATA(nlh: *const nlmsghdr) -> *mut c_void {
    (nlh as *mut u8).add(nlmsg_align(size_of::<nlmsghdr>())) as *mut c_void
}

unsafe fn NLMSG_OK(nlh: *const nlmsghdr, len: ssize_t) -> bool {
    len >= size_of::<nlmsghdr>() as ssize_t
        && (*nlh).nlmsg_len >= size_of::<nlmsghdr>() as __u32
        && (*nlh).nlmsg_len as ssize_t <= len
}

unsafe fn NLMSG_NEXT(nlh: *const nlmsghdr, len: &mut ssize_t) -> *const nlmsghdr {
    let aligned = nlmsg_align((*nlh).nlmsg_len as usize) as ssize_t;
    *len -= aligned;
    (nlh as *const u8).offset(aligned) as *const nlmsghdr
}

unsafe fn nlattr_align(len: usize) -> usize {
    (len + 4 - 1) & !(4 - 1)
}

unsafe fn nlattr_ok(attr: *const nlattr, rem: ssize_t) -> bool {
    rem >= size_of::<nlattr>() as ssize_t
        && (*attr).nla_len >= size_of::<nlattr>() as __u16
        && (*attr).nla_len as ssize_t <= rem
}

unsafe fn nl_get_extack(buf: *mut c_char, n: size_t, ea: *mut ext_ack) -> get_ea_ret {
    let mut ret = get_ea_ret::NO_CTRL;
    let mut nlh: *const nlmsghdr;
    let mut attr: *const nlattr;
    let mut rem: ssize_t;

    rem = n as ssize_t;
    while rem > 0 {
        nlh = buf.add(n - rem as usize) as *const nlmsghdr;
        if !NLMSG_OK(nlh, rem) {
            return get_ea_ret::ERROR;
        }

        if (*nlh).nlmsg_type == NLMSG_ERROR {
            ret = get_ea_ret::FOUND_ERR;
        } else if (*nlh).nlmsg_type == NLMSG_DONE {
            ret = get_ea_ret::FOUND_DONE;
        } else {
            nlh = NLMSG_NEXT(nlh, &mut rem);
            let _ = nlh;
            continue;
        }

        (*ea).err = -*(NLMSG_DATA(nlh) as *const c_int);

        if ((*nlh).nlmsg_flags & NLM_F_ACK_TLVS) == 0 {
            return ret;
        }

        let base = NLMSG_DATA(nlh).add(size_of::<c_int>()) as *const u8;
        let hdr_and_err = nlmsg_align(size_of::<nlmsghdr>()) + size_of::<c_int>();
        let mut attr_rem = (*nlh).nlmsg_len as ssize_t - hdr_and_err as ssize_t;
        attr = base as *const nlattr;
        while nlattr_ok(attr, attr_rem) {
            match ynl_attr_type(attr) {
                NLMSGERR_ATTR_OFFS => {
                    (*ea).attr_offs = ynl_attr_get_u32(attr);
                }
                NLMSGERR_ATTR_MISS_TYPE => {
                    (*ea).miss_type = ynl_attr_get_u32(attr);
                }
                NLMSGERR_ATTR_MISS_NEST => {
                    (*ea).miss_nest = ynl_attr_get_u32(attr);
                }
                NLMSGERR_ATTR_MSG => {
                    (*ea).str_ = ynl_attr_get_str(attr);
                }
                _ => {}
            }

            let aligned = nlattr_align((*attr).nla_len as usize) as ssize_t;
            attr_rem -= aligned;
            attr = (attr as *const u8).offset(aligned) as *const nlattr;
        }

        return get_ea_ret::FOUND_EXTACK;
    }

    ret
}

#[repr(C)]
pub struct dump_neigh_bad_t {
    pub nlhdr: nlmsghdr,
    pub ndm: ndmsg,
    pub ahdr: nlattr,
    pub val: __u32,
}

static dump_neigh_bad: dump_neigh_bad_t = dump_neigh_bad_t {
    nlhdr: nlmsghdr {
        nlmsg_len: size_of::<dump_neigh_bad_t>() as __u32,
        nlmsg_type: RTM_GETNEIGH,
        nlmsg_flags: NLM_F_REQUEST | NLM_F_ACK | NLM_F_DUMP,
        nlmsg_seq: 1,
        nlmsg_pid: 0,
    },
    ndm: ndmsg {
        ndm_family: 123,
        ndm_pad1: 0,
        ndm_pad2: 0,
        ndm_ifindex: 0,
        ndm_state: 0,
        ndm_flags: 0,
        ndm_type: 0,
    },
    ahdr: nlattr {
        nla_len: 4 + 4,
        nla_type: NDA_FLAGS_EXT,
    },
    val: -1i32 as __u32, // should fail MASK validation
};

pub unsafe fn dump_extack() {
    let netlink_sock: c_int;
    let mut i: c_int;
    let cnt: c_int;
    let mut ret: get_ea_ret;
    let mut buf = [0 as c_char; 8192];
    let one: c_int = 1;
    let mut n: ssize_t;

    netlink_sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE);
    ASSERT_GE!(netlink_sock, 0);

    n = setsockopt(
        netlink_sock,
        SOL_NETLINK,
        NETLINK_CAP_ACK,
        &one as *const c_int as *const c_void,
        size_of::<c_int>() as c_uint,
    ) as ssize_t;
    ASSERT_EQ!(n, 0);
    n = setsockopt(
        netlink_sock,
        SOL_NETLINK,
        NETLINK_EXT_ACK,
        &one as *const c_int as *const c_void,
        size_of::<c_int>() as c_uint,
    ) as ssize_t;
    ASSERT_EQ!(n, 0);
    n = setsockopt(
        netlink_sock,
        SOL_NETLINK,
        NETLINK_GET_STRICT_CHK,
        &one as *const c_int as *const c_void,
        size_of::<c_int>() as c_uint,
    ) as ssize_t;
    ASSERT_EQ!(n, 0);

    /* Dump so many times we fill up the buffer */
    cnt = 80;
    i = 0;
    while i < cnt {
        n = send(
            netlink_sock,
            &dump_neigh_bad as *const dump_neigh_bad_t as *const c_void,
            size_of::<dump_neigh_bad_t>(),
            0,
        );
        ASSERT_EQ!(n, size_of::<dump_neigh_bad_t>() as ssize_t);
        i += 1;
    }

    /* Read out the ENOBUFS */
    n = recv(
        netlink_sock,
        buf.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 8192]>(),
        MSG_DONTWAIT,
    );
    EXPECT_EQ!(n, -1);
    EXPECT_EQ!(errno, ENOBUFS);

    ret = get_ea_ret::NO_CTRL;
    i = 0;
    while i < cnt {
        let mut ea: ext_ack = zeroed();

        n = recv(
            netlink_sock,
            buf.as_mut_ptr() as *mut c_void,
            size_of::<[c_char; 8192]>(),
            MSG_DONTWAIT,
        );
        if n < 0 {
            ASSERT_GE!(i, 10);
            break;
        }
        ASSERT_GE!(n, size_of::<nlmsghdr>() as ssize_t);

        ret = nl_get_extack(buf.as_mut_ptr(), n as size_t, &mut ea);
        /* Once we fill the buffer we'll see one ENOBUFS followed
         * by a number of EBUSYs. Then the last recv() will finally
         * trigger and complete the dump.
         */
        if ret == get_ea_ret::FOUND_ERR && (ea.err == ENOBUFS || ea.err == EBUSY) {
            i += 1;
            continue;
        }
        EXPECT_EQ!(ret as c_int, get_ea_ret::FOUND_EXTACK as c_int);
        EXPECT_EQ!(ea.err, EINVAL);
        EXPECT_EQ!(
            ea.attr_offs,
            (size_of::<nlmsghdr>() + size_of::<ndmsg>()) as __u32
        );
        i += 1;
    }
    /* Make sure last message was a full DONE+extack */
    EXPECT_EQ!(ret as c_int, get_ea_ret::FOUND_EXTACK as c_int);
}

#[repr(C)]
pub struct dump_policies_t {
    pub nlhdr: nlmsghdr,
    pub genlhdr: genlmsghdr,
    pub ahdr: nlattr,
    pub val: __u16,
    pub pad: __u16,
}

static dump_policies: dump_policies_t = dump_policies_t {
    nlhdr: nlmsghdr {
        nlmsg_len: size_of::<dump_policies_t>() as __u32,
        nlmsg_type: GENL_ID_CTRL,
        nlmsg_flags: NLM_F_REQUEST | NLM_F_ACK | NLM_F_DUMP,
        nlmsg_seq: 1,
        nlmsg_pid: 0,
    },
    genlhdr: genlmsghdr {
        cmd: CTRL_CMD_GETPOLICY,
        version: 2,
        reserved: 0,
    },
    ahdr: nlattr {
        nla_len: 6,
        nla_type: CTRL_ATTR_FAMILY_ID,
    },
    val: GENL_ID_CTRL,
    pad: 0,
};

// Sanity check for the test itself, make sure the dump doesn't fit in one msg
pub unsafe fn test_sanity() {
    let netlink_sock: c_int;
    let mut buf = [0 as c_char; 8192];
    let mut n: ssize_t;

    netlink_sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    ASSERT_GE!(netlink_sock, 0);

    n = send(
        netlink_sock,
        &dump_policies as *const dump_policies_t as *const c_void,
        size_of::<dump_policies_t>(),
        0,
    );
    ASSERT_EQ!(n, size_of::<dump_policies_t>() as ssize_t);

    n = recv(
        netlink_sock,
        buf.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 8192]>(),
        MSG_DONTWAIT,
    );
    ASSERT_GE!(n, size_of::<nlmsghdr>() as ssize_t);

    n = recv(
        netlink_sock,
        buf.as_mut_ptr() as *mut c_void,
        size_of::<[c_char; 8192]>(),
        MSG_DONTWAIT,
    );
    ASSERT_GE!(n, size_of::<nlmsghdr>() as ssize_t);

    close(netlink_sock);
}

pub unsafe fn close_in_progress() {
    let netlink_sock: c_int;
    let n: ssize_t;

    netlink_sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    ASSERT_GE!(netlink_sock, 0);

    n = send(
        netlink_sock,
        &dump_policies as *const dump_policies_t as *const c_void,
        size_of::<dump_policies_t>(),
        0,
    );
    ASSERT_EQ!(n, size_of::<dump_policies_t>() as ssize_t);

    close(netlink_sock);
}

pub unsafe fn close_with_ref() {
    let mut cookie = [0 as c_char; NOTIFY_COOKIE_LEN];
    let netlink_sock: c_int;
    let mq_fd: c_int;
    let mut sigev: sigevent;
    let n: ssize_t;

    netlink_sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    ASSERT_GE!(netlink_sock, 0);

    n = send(
        netlink_sock,
        &dump_policies as *const dump_policies_t as *const c_void,
        size_of::<dump_policies_t>(),
        0,
    );
    ASSERT_EQ!(n, size_of::<dump_policies_t>() as ssize_t);

    mq_fd = syscall(
        __NR_mq_open,
        b"sed\0".as_ptr() as *const c_char,
        O_CREAT | O_WRONLY,
        0o600 as c_int,
        0 as c_int,
    ) as c_int;
    ASSERT_GE!(mq_fd, 0);

    sigev = zeroed();
    memset(
        &mut sigev as *mut sigevent as *mut c_void,
        0,
        size_of::<sigevent>(),
    );
    sigev.sigev_notify = SIGEV_THREAD;
    sigev.sigev_value.sival_ptr = cookie.as_mut_ptr() as *mut c_void;
    sigev.sigev_signo = netlink_sock;

    syscall(
        __NR_mq_notify,
        mq_fd,
        &mut sigev as *mut sigevent,
    );

    close(netlink_sock);

    // give mqueue time to fire
    usleep(100 * 1000);
}

pub fn main() {
    unsafe {
        dump_extack();
        test_sanity();
        close_in_progress();
        close_with_ref();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
