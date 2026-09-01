// SPDX-License-Identifier: GPL-2.0-or-later

/* Basic per-epoll context busy poll test.
 *
 * Only tests the ioctls, but should be expanded to test two connected hosts in
 * the future
 */

/* C dependencies:
 * _GNU_SOURCE
 * <error.h>
 * <errno.h>
 * <inttypes.h>
 * <limits.h>
 * <stdio.h>
 * <stdlib.h>
 * <string.h>
 * <unistd.h>
 * <sys/capability.h>
 * <sys/epoll.h>
 * <sys/ioctl.h>
 * <sys/socket.h>
 * "kselftest_harness.h"
 */

use core::ffi::{c_int, c_ulong, c_void};
use core::mem::size_of;

type cap_t = *mut c_void;
type cap_value_t = c_int;
type cap_flag_t = c_int;
type cap_flag_value_t = c_int;

const AF_UNIX: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const ENOTTY: c_int = 25;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const EPERM: c_int = 1;
const INT_MAX: c_int = 2147483647;
const UINT16_MAX: u16 = u16::MAX;
const CAP_NET_ADMIN: cap_value_t = 12;
const CAP_EFFECTIVE: cap_flag_t = 0;
const CAP_SET: cap_flag_value_t = 1;
const CAP_CLEAR: cap_flag_value_t = 0;

/* if the headers haven't been updated, we need to define some things */
#[repr(C)]
struct epoll_params {
    busy_poll_usecs: u32,
    busy_poll_budget: u16,
    prefer_busy_poll: u8,

    /* pad the struct to a multiple of 64bits */
    __pad: u8,
}

const EPOLL_IOC_TYPE: u32 = 0x8A;

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn _ioc(dir: u32, ty: u32, nr: u32, size: u32) -> c_ulong {
    ((dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)) as c_ulong
}

const fn _iow<T>(ty: u32, nr: u32) -> c_ulong {
    _ioc(IOC_WRITE, ty, nr, size_of::<T>() as u32)
}

const fn _ior<T>(ty: u32, nr: u32) -> c_ulong {
    _ioc(IOC_READ, ty, nr, size_of::<T>() as u32)
}

const EPIOCSPARAMS: c_ulong = _iow::<epoll_params>(EPOLL_IOC_TYPE, 0x01);
const EPIOCGPARAMS: c_ulong = _ior::<epoll_params>(EPOLL_IOC_TYPE, 0x02);

unsafe extern "C" {
    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn epoll_create1(flags: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn cap_get_proc() -> cap_t;
    fn cap_free(cap: cap_t) -> c_int;
    fn cap_get_flag(
        cap_p: cap_t,
        cap: cap_value_t,
        flag: cap_flag_t,
        value_p: *mut cap_flag_value_t,
    ) -> c_int;
    fn cap_set_flag(
        cap_p: cap_t,
        flag: cap_flag_t,
        ncap: c_int,
        caps: *const cap_value_t,
        value: cap_flag_value_t,
    ) -> c_int;
    fn cap_set_proc(cap_p: cap_t) -> c_int;
}

unsafe extern "C" {
    static mut errno: c_int;
}

struct invalid_fd {
    invalid_fd: c_int,
    params: epoll_params,
}

unsafe fn invalid_fd_setup(self_: *mut invalid_fd) {
    let mut ret: c_int;

    ret = socket(AF_UNIX, SOCK_DGRAM, 0);
    EXPECT_NE!(-1, ret, "error creating unix socket");

    (*self_).invalid_fd = ret;
}

unsafe fn invalid_fd_teardown(self_: *mut invalid_fd) {
    let mut ret: c_int;

    ret = close((*self_).invalid_fd);
    EXPECT_EQ!(0, ret);
}

unsafe fn invalid_fd_test_invalid_fd(self_: *mut invalid_fd) {
    let mut ret: c_int;

    ret = ioctl((*self_).invalid_fd, EPIOCGPARAMS, &mut (*self_).params);

    EXPECT_EQ!(
        -1,
        ret,
        "EPIOCGPARAMS on invalid epoll FD should error"
    );

    EXPECT_EQ!(
        ENOTTY,
        errno,
        "EPIOCGPARAMS on invalid epoll FD should set errno to ENOTTY"
    );

    memset(
        &mut (*self_).params as *mut epoll_params as *mut c_void,
        0,
        size_of::<epoll_params>(),
    );

    ret = ioctl((*self_).invalid_fd, EPIOCSPARAMS, &mut (*self_).params);

    EXPECT_EQ!(
        -1,
        ret,
        "EPIOCSPARAMS on invalid epoll FD should error"
    );

    EXPECT_EQ!(
        ENOTTY,
        errno,
        "EPIOCSPARAMS on invalid epoll FD should set errno to ENOTTY"
    );
}

struct epoll_busy_poll {
    fd: c_int,
    params: epoll_params,
    invalid_params: *mut epoll_params,
    caps: cap_t,
}

unsafe fn epoll_busy_poll_setup(self_: *mut epoll_busy_poll) {
    let mut ret: c_int;

    ret = epoll_create1(0);
    EXPECT_NE!(-1, ret, "epoll_create1 failed?");

    (*self_).fd = ret;

    (*self_).caps = cap_get_proc();
    EXPECT_NE!(core::ptr::null_mut::<c_void>(), (*self_).caps);
}

unsafe fn epoll_busy_poll_teardown(self_: *mut epoll_busy_poll) {
    let mut ret: c_int;

    ret = close((*self_).fd);
    EXPECT_EQ!(0, ret);

    ret = cap_free((*self_).caps);
    EXPECT_NE!(-1, ret, "unable to free capabilities");
}

unsafe fn epoll_busy_poll_test_get_params(self_: *mut epoll_busy_poll) {
    /* begin by getting the epoll params from the kernel
     *
     * the default should be default and all fields should be zero'd by the
     * kernel, so set params fields to garbage to test this.
     */
    let mut ret: c_int = 0;

    (*self_).params.busy_poll_usecs = 0xff;
    (*self_).params.busy_poll_budget = 0xff;
    (*self_).params.prefer_busy_poll = 1;
    (*self_).params.__pad = 0xf;

    ret = ioctl((*self_).fd, EPIOCGPARAMS, &mut (*self_).params);
    EXPECT_EQ!(0, ret, "ioctl EPIOCGPARAMS should succeed");

    EXPECT_EQ!(
        0,
        (*self_).params.busy_poll_usecs,
        "EPIOCGPARAMS busy_poll_usecs should have been 0"
    );

    EXPECT_EQ!(
        0,
        (*self_).params.busy_poll_budget,
        "EPIOCGPARAMS busy_poll_budget should have been 0"
    );

    EXPECT_EQ!(
        0,
        (*self_).params.prefer_busy_poll,
        "EPIOCGPARAMS prefer_busy_poll should have been 0"
    );

    EXPECT_EQ!(
        0,
        (*self_).params.__pad,
        "EPIOCGPARAMS __pad should have been 0"
    );

    (*self_).invalid_params = 0xdeadbeefusize as *mut epoll_params;
    ret = ioctl((*self_).fd, EPIOCGPARAMS, (*self_).invalid_params);

    EXPECT_EQ!(-1, ret, "EPIOCGPARAMS should error with invalid params");

    EXPECT_EQ!(
        EFAULT,
        errno,
        "EPIOCGPARAMS with invalid params should set errno to EFAULT"
    );
}

unsafe fn epoll_busy_poll_test_set_invalid(self_: *mut epoll_busy_poll) {
    let mut ret: c_int;

    memset(
        &mut (*self_).params as *mut epoll_params as *mut c_void,
        0,
        size_of::<epoll_params>(),
    );

    (*self_).params.__pad = 1;

    ret = ioctl((*self_).fd, EPIOCSPARAMS, &mut (*self_).params);

    EXPECT_EQ!(-1, ret, "EPIOCSPARAMS non-zero __pad should error");

    EXPECT_EQ!(
        EINVAL,
        errno,
        "EPIOCSPARAMS non-zero __pad errno should be EINVAL"
    );

    (*self_).params.__pad = 0;
    (*self_).params.busy_poll_usecs = (INT_MAX as u32).wrapping_add(1);

    ret = ioctl((*self_).fd, EPIOCSPARAMS, &mut (*self_).params);

    EXPECT_EQ!(
        -1,
        ret,
        "EPIOCSPARAMS should error busy_poll_usecs > S32_MAX"
    );

    EXPECT_EQ!(
        EINVAL,
        errno,
        "EPIOCSPARAMS busy_poll_usecs > S32_MAX errno should be EINVAL"
    );

    (*self_).params.__pad = 0;
    (*self_).params.busy_poll_usecs = 32;
    (*self_).params.prefer_busy_poll = 2;

    ret = ioctl((*self_).fd, EPIOCSPARAMS, &mut (*self_).params);

    EXPECT_EQ!(
        -1,
        ret,
        "EPIOCSPARAMS should error prefer_busy_poll > 1"
    );

    EXPECT_EQ!(
        EINVAL,
        errno,
        "EPIOCSPARAMS prefer_busy_poll > 1 errno should be EINVAL"
    );

    (*self_).params.__pad = 0;
    (*self_).params.busy_poll_usecs = 32;
    (*self_).params.prefer_busy_poll = 1;

    /* set budget well above kernel's NAPI_POLL_WEIGHT of 64 */
    (*self_).params.busy_poll_budget = UINT16_MAX;

    /* test harness should run with CAP_NET_ADMIN, but let's make sure */
    let mut tmp: cap_flag_value_t = 0;

    ret = cap_get_flag((*self_).caps, CAP_NET_ADMIN, CAP_EFFECTIVE, &mut tmp);
    EXPECT_EQ!(0, ret, "unable to get CAP_NET_ADMIN cap flag");

    EXPECT_EQ!(
        CAP_SET,
        tmp,
        "expecting CAP_NET_ADMIN to be set for the test harness"
    );

    /* at this point we know CAP_NET_ADMIN is available, so setting the
     * params with a busy_poll_budget > NAPI_POLL_WEIGHT should succeed
     */
    ret = ioctl((*self_).fd, EPIOCSPARAMS, &mut (*self_).params);

    EXPECT_EQ!(
        0,
        ret,
        "EPIOCSPARAMS should allow busy_poll_budget > NAPI_POLL_WEIGHT"
    );

    /* remove CAP_NET_ADMIN from our effective set */
    let net_admin: [cap_value_t; 1] = [CAP_NET_ADMIN];

    ret = cap_set_flag(
        (*self_).caps,
        CAP_EFFECTIVE,
        1,
        net_admin.as_ptr(),
        CAP_CLEAR,
    );
    EXPECT_EQ!(0, ret, "couldn't clear CAP_NET_ADMIN");

    ret = cap_set_proc((*self_).caps);
    EXPECT_EQ!(0, ret, "cap_set_proc should drop CAP_NET_ADMIN");

    /* this is now expected to fail */
    ret = ioctl((*self_).fd, EPIOCSPARAMS, &mut (*self_).params);

    EXPECT_EQ!(
        -1,
        ret,
        "EPIOCSPARAMS should error busy_poll_budget > NAPI_POLL_WEIGHT"
    );

    EXPECT_EQ!(
        EPERM,
        errno,
        "EPIOCSPARAMS errno should be EPERM busy_poll_budget > NAPI_POLL_WEIGHT"
    );

    /* restore CAP_NET_ADMIN to our effective set */
    ret = cap_set_flag(
        (*self_).caps,
        CAP_EFFECTIVE,
        1,
        net_admin.as_ptr(),
        CAP_SET,
    );
    EXPECT_EQ!(0, ret, "couldn't restore CAP_NET_ADMIN");

    ret = cap_set_proc((*self_).caps);
    EXPECT_EQ!(0, ret, "cap_set_proc should set  CAP_NET_ADMIN");

    (*self_).invalid_params = 0xdeadbeefusize as *mut epoll_params;
    ret = ioctl((*self_).fd, EPIOCSPARAMS, (*self_).invalid_params);

    EXPECT_EQ!(
        -1,
        ret,
        "EPIOCSPARAMS should error when epoll_params is invalid"
    );

    EXPECT_EQ!(
        EFAULT,
        errno,
        "EPIOCSPARAMS should set errno to EFAULT when epoll_params is invalid"
    );
}

unsafe fn epoll_busy_poll_test_set_and_get_valid(self_: *mut epoll_busy_poll) {
    let mut ret: c_int;

    memset(
        &mut (*self_).params as *mut epoll_params as *mut c_void,
        0,
        size_of::<epoll_params>(),
    );

    (*self_).params.busy_poll_usecs = 25;
    (*self_).params.busy_poll_budget = 16;
    (*self_).params.prefer_busy_poll = 1;

    ret = ioctl((*self_).fd, EPIOCSPARAMS, &mut (*self_).params);

    EXPECT_EQ!(
        0,
        ret,
        "EPIOCSPARAMS with valid params should not error"
    );

    /* check that the kernel returns the same values back */

    memset(
        &mut (*self_).params as *mut epoll_params as *mut c_void,
        0,
        size_of::<epoll_params>(),
    );

    ret = ioctl((*self_).fd, EPIOCGPARAMS, &mut (*self_).params);

    EXPECT_EQ!(0, ret, "EPIOCGPARAMS should not error");

    EXPECT_EQ!(
        25,
        (*self_).params.busy_poll_usecs,
        "params.busy_poll_usecs incorrect"
    );

    EXPECT_EQ!(
        16,
        (*self_).params.busy_poll_budget,
        "params.busy_poll_budget incorrect"
    );

    EXPECT_EQ!(
        1,
        (*self_).params.prefer_busy_poll,
        "params.prefer_busy_poll incorrect"
    );

    EXPECT_EQ!(0, (*self_).params.__pad, "params.__pad was not 0");
}

unsafe fn epoll_busy_poll_test_invalid_ioctl(self_: *mut epoll_busy_poll) {
    let invalid_ioctl: c_ulong = EPIOCGPARAMS + 10;
    let mut ret: c_int;

    ret = ioctl((*self_).fd, invalid_ioctl, &mut (*self_).params);

    EXPECT_EQ!(-1, ret, "invalid ioctl should return error");

    EXPECT_EQ!(EINVAL, errno, "invalid ioctl should set errno to EINVAL");
}

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
