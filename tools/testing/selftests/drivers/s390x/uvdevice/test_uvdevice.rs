// SPDX-License-Identifier: GPL-2.0
/*
 *  selftest for the Ultravisor UAPI device
 *
 *  Copyright IBM Corp. 2022
 *  Author(s): Steffen Eiden <seiden@linux.ibm.com>
 */

/*
 * C dependencies translated as external/file-local references:
 * stdint.h, fcntl.h, errno.h, sys/ioctl.h, sys/mman.h,
 * asm/uvdevice.h, kselftest_harness.h.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

type __u64 = u64;

const UV_PATH: &[u8] = b"/dev/uv\0";
const BUFFER_SIZE: usize = 0x200;

extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn getpagesize() -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
    fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

/* External constants/macros from C headers. */
extern "C" {
    static O_ACCMODE: c_int;
    static PROT_NONE: c_int;
    static MAP_ANONYMOUS: c_int;
    static EFAULT: c_int;
    static EINVAL: c_int;
    static ENOTTY: c_int;
    static UVIO_IOCTL_ATT: c_ulong;
    static UVIO_TYPE_UVC: c_int;
    static UVIO_ATT_ADDITIONAL_MAX_LEN: u32;
    static UVIO_ATT_MEASUREMENT_MAX_LEN: u32;
    static UVIO_ATT_ARCB_MAX_LEN: u32;
}

#[repr(C)]
pub struct uvio_ioctl_cb {
    pub flags: u32,
    pub argument_len: u32,
    pub argument_addr: __u64,
    pub reserved14: [u8; 0],
}

#[repr(C)]
pub struct uvio_attest {
    pub arcb_addr: __u64,
    pub meas_addr: __u64,
    pub add_data_addr: __u64,
    pub arcb_len: u32,
    pub meas_len: u32,
    pub add_data_len: u32,
    pub reserved136: u16,
}

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! _IOC_NR {
    ($nr:expr) => {
        (($nr as c_ulong) & 0xff) as u8
    };
}

macro_rules! _IOC {
    ($type_:expr, $nr:expr) => {
        (($type_ as c_ulong) << 8) | ($nr as c_ulong)
    };
}

macro_rules! _IOR {
    ($type_:expr, $nr:expr, $ty:ty) => {
        _IOC!($type_, $nr) | ((core::mem::size_of::<$ty>() as c_ulong) << 16) | (2 << 30)
    };
}

macro_rules! _IOW {
    ($type_:expr, $nr:expr, $ty:ty) => {
        _IOC!($type_, $nr) | ((core::mem::size_of::<$ty>() as c_ulong) << 16) | (1 << 30)
    };
}

macro_rules! _IOWR {
    ($type_:expr, $nr:expr, $ty:ty) => {
        _IOC!($type_, $nr) | ((core::mem::size_of::<$ty>() as c_ulong) << 16) | (3 << 30)
    };
}

#[repr(C)]
pub struct uvio_fixture {
    uv_fd: c_int,
    uvio_ioctl: uvio_ioctl_cb,
    buffer: [u8; BUFFER_SIZE],
    fault_page: __u64,
}

#[repr(C)]
pub struct uvio_fixture_variant {
    ioctl_cmd: c_ulong,
    arg_size: u32,
}

static mut uvio_fixture_att: uvio_fixture_variant = uvio_fixture_variant {
    ioctl_cmd: 0,
    arg_size: core::mem::size_of::<uvio_attest>() as u32,
};

unsafe fn uvio_fixture_setup(self_: *mut uvio_fixture, variant: *const uvio_fixture_variant) {
    (*self_).uv_fd = open(UV_PATH.as_ptr() as *const c_char, O_ACCMODE);

    (*self_).uvio_ioctl.argument_addr = (*self_).buffer.as_mut_ptr() as __u64;
    (*self_).uvio_ioctl.argument_len = (*variant).arg_size;
    (*self_).fault_page = mmap(
        core::ptr::null_mut(),
        getpagesize() as usize,
        PROT_NONE,
        MAP_ANONYMOUS,
        -1,
        0,
    ) as __u64;
}

unsafe fn uvio_fixture_teardown(self_: *mut uvio_fixture) {
    if (*self_).uv_fd != 0 {
        close((*self_).uv_fd);
    }
    munmap((*self_).fault_page as *mut c_void, getpagesize() as usize);
}

unsafe fn uvio_fixture_fault_ioctl_arg(
    _metadata: *mut __test_metadata,
    self_: *mut uvio_fixture,
    variant: *const uvio_fixture_variant,
) {
    let mut rc: c_int;
    let mut errno_cache: c_int;

    rc = ioctl((*self_).uv_fd, (*variant).ioctl_cmd, core::ptr::null::<c_void>());
    errno_cache = errno;
    ASSERT_EQ!(rc, -1);
    ASSERT_EQ!(errno_cache, EFAULT);

    rc = ioctl((*self_).uv_fd, (*variant).ioctl_cmd, (*self_).fault_page);
    errno_cache = errno;
    ASSERT_EQ!(rc, -1);
    ASSERT_EQ!(errno_cache, EFAULT);
}

unsafe fn uvio_fixture_fault_uvio_arg(
    _metadata: *mut __test_metadata,
    self_: *mut uvio_fixture,
    variant: *const uvio_fixture_variant,
) {
    let mut rc: c_int;
    let mut errno_cache: c_int;

    (*self_).uvio_ioctl.argument_addr = 0;
    rc = ioctl((*self_).uv_fd, (*variant).ioctl_cmd, &mut (*self_).uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ!(rc, -1);
    ASSERT_EQ!(errno_cache, EFAULT);

    (*self_).uvio_ioctl.argument_addr = (*self_).fault_page;
    rc = ioctl((*self_).uv_fd, (*variant).ioctl_cmd, &mut (*self_).uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ!(rc, -1);
    ASSERT_EQ!(errno_cache, EFAULT);
}

/*
 * Test to verify that IOCTLs with invalid values in the ioctl_control block
 * are rejected.
 */
unsafe fn uvio_fixture_inval_ioctl_cb(
    _metadata: *mut __test_metadata,
    self_: *mut uvio_fixture,
    variant: *const uvio_fixture_variant,
) {
    let mut rc: c_int;
    let mut errno_cache: c_int;

    (*self_).uvio_ioctl.argument_len = 0;
    rc = ioctl((*self_).uv_fd, (*variant).ioctl_cmd, &mut (*self_).uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ!(rc, -1);
    ASSERT_EQ!(errno_cache, EINVAL);

    (*self_).uvio_ioctl.argument_len = (-1i32) as u32;
    rc = ioctl((*self_).uv_fd, (*variant).ioctl_cmd, &mut (*self_).uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ!(rc, -1);
    ASSERT_EQ!(errno_cache, EINVAL);
    (*self_).uvio_ioctl.argument_len = (*variant).arg_size;

    (*self_).uvio_ioctl.flags = (-1i32) as u32;
    rc = ioctl((*self_).uv_fd, (*variant).ioctl_cmd, &mut (*self_).uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ!(rc, -1);
    ASSERT_EQ!(errno_cache, EINVAL);
    (*self_).uvio_ioctl.flags = 0;

    memset(
        (*self_).uvio_ioctl.reserved14.as_mut_ptr() as *mut c_void,
        0xff,
        core::mem::size_of_val(&(*self_).uvio_ioctl.reserved14),
    );
    rc = ioctl((*self_).uv_fd, (*variant).ioctl_cmd, &mut (*self_).uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ!(rc, -1);
    ASSERT_EQ!(errno_cache, EINVAL);

    memset(
        &mut (*self_).uvio_ioctl as *mut uvio_ioctl_cb as *mut c_void,
        0x11,
        core::mem::size_of::<uvio_ioctl_cb>(),
    );
    rc = ioctl((*self_).uv_fd, (*variant).ioctl_cmd, &mut (*self_).uvio_ioctl);
    ASSERT_EQ!(rc, -1);
}

unsafe fn uvio_fixture_inval_ioctl_cmd(
    _metadata: *mut __test_metadata,
    self_: *mut uvio_fixture,
    variant: *const uvio_fixture_variant,
) {
    let mut rc: c_int;
    let mut errno_cache: c_int;
    let nr: u8 = _IOC_NR!((*variant).ioctl_cmd);
    let cmds: [c_ulong; 5] = [
        _IOWR!(b'a', nr, uvio_ioctl_cb),
        _IOWR!(UVIO_TYPE_UVC, nr, c_int),
        _IOC!(UVIO_TYPE_UVC, nr),
        _IOR!(UVIO_TYPE_UVC, nr, uvio_ioctl_cb),
        _IOW!(UVIO_TYPE_UVC, nr, uvio_ioctl_cb),
    ];

    for i in 0..cmds.len() {
        rc = ioctl((*self_).uv_fd, cmds[i], &mut (*self_).uvio_ioctl);
        errno_cache = errno;
        ASSERT_EQ!(rc, -1);
        ASSERT_EQ!(errno_cache, ENOTTY);
    }
}

#[repr(C)]
pub struct test_attest_buffer {
    arcb: [u8; 0x180],
    meas: [u8; 64],
    add: [u8; 32],
}

#[repr(C)]
pub struct attest_fixture {
    uv_fd: c_int,
    uvio_ioctl: uvio_ioctl_cb,
    uvio_attest: uvio_attest,
    attest_buffer: test_attest_buffer,
    fault_page: __u64,
}

unsafe fn attest_fixture_setup(self_: *mut attest_fixture) {
    (*self_).uv_fd = open(UV_PATH.as_ptr() as *const c_char, O_ACCMODE);

    (*self_).uvio_ioctl.argument_addr = &mut (*self_).uvio_attest as *mut uvio_attest as __u64;
    (*self_).uvio_ioctl.argument_len = core::mem::size_of_val(&(*self_).uvio_attest) as u32;

    (*self_).uvio_attest.arcb_addr = &mut (*self_).attest_buffer.arcb as *mut [u8; 0x180] as __u64;
    (*self_).uvio_attest.arcb_len = core::mem::size_of_val(&(*self_).attest_buffer.arcb) as u32;

    (*self_).uvio_attest.meas_addr = &mut (*self_).attest_buffer.meas as *mut [u8; 64] as __u64;
    (*self_).uvio_attest.meas_len = core::mem::size_of_val(&(*self_).attest_buffer.meas) as u32;

    (*self_).uvio_attest.add_data_addr = &mut (*self_).attest_buffer.add as *mut [u8; 32] as __u64;
    (*self_).uvio_attest.add_data_len =
        core::mem::size_of_val(&(*self_).attest_buffer.add) as u32;
    (*self_).fault_page = mmap(
        core::ptr::null_mut(),
        getpagesize() as usize,
        PROT_NONE,
        MAP_ANONYMOUS,
        -1,
        0,
    ) as __u64;
}

unsafe fn attest_fixture_teardown(self_: *mut attest_fixture) {
    if (*self_).uv_fd != 0 {
        close((*self_).uv_fd);
    }
    munmap((*self_).fault_page as *mut c_void, getpagesize() as usize);
}

unsafe fn att_inval_sizes_test(
    size: *mut u32,
    max_size: u32,
    test_zero: bool,
    _metadata: *mut __test_metadata,
    self_: *mut attest_fixture,
) {
    let mut rc: c_int;
    let mut errno_cache: c_int;
    let tmp: u32 = *size;

    if test_zero {
        *size = 0;
        rc = ioctl((*self_).uv_fd, UVIO_IOCTL_ATT, &mut (*self_).uvio_ioctl);
        errno_cache = errno;
        ASSERT_EQ!(rc, -1);
        ASSERT_EQ!(errno_cache, EINVAL);
    }
    *size = max_size.wrapping_add(1);
    rc = ioctl((*self_).uv_fd, UVIO_IOCTL_ATT, &mut (*self_).uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ!(rc, -1);
    ASSERT_EQ!(errno_cache, EINVAL);
    *size = tmp;
}

/*
 * Test to verify that attestation IOCTLs with invalid values in the UVIO
 * attestation control block are rejected.
 */
unsafe fn attest_fixture_att_inval_request(
    _metadata: *mut __test_metadata,
    self_: *mut attest_fixture,
) {
    let mut rc: c_int;
    let mut errno_cache: c_int;

    att_inval_sizes_test(
        &mut (*self_).uvio_attest.add_data_len,
        UVIO_ATT_ADDITIONAL_MAX_LEN,
        false,
        _metadata,
        self_,
    );
    att_inval_sizes_test(
        &mut (*self_).uvio_attest.meas_len,
        UVIO_ATT_MEASUREMENT_MAX_LEN,
        true,
        _metadata,
        self_,
    );
    att_inval_sizes_test(
        &mut (*self_).uvio_attest.arcb_len,
        UVIO_ATT_ARCB_MAX_LEN,
        true,
        _metadata,
        self_,
    );

    (*self_).uvio_attest.reserved136 = (-1i16) as u16;
    rc = ioctl((*self_).uv_fd, UVIO_IOCTL_ATT, &mut (*self_).uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ!(rc, -1);
    ASSERT_EQ!(errno_cache, EINVAL);

    memset(
        &mut (*self_).uvio_attest as *mut uvio_attest as *mut c_void,
        0x11,
        core::mem::size_of_val(&(*self_).uvio_attest),
    );
    rc = ioctl((*self_).uv_fd, UVIO_IOCTL_ATT, &mut (*self_).uvio_ioctl);
    ASSERT_EQ!(rc, -1);
}

unsafe fn att_inval_addr_test(
    addr: *mut __u64,
    _metadata: *mut __test_metadata,
    self_: *mut attest_fixture,
) {
    let mut rc: c_int;
    let mut errno_cache: c_int;
    let tmp: __u64 = *addr;

    *addr = 0;
    rc = ioctl((*self_).uv_fd, UVIO_IOCTL_ATT, &mut (*self_).uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ!(rc, -1);
    ASSERT_EQ!(errno_cache, EFAULT);
    *addr = (*self_).fault_page;
    rc = ioctl((*self_).uv_fd, UVIO_IOCTL_ATT, &mut (*self_).uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ!(rc, -1);
    ASSERT_EQ!(errno_cache, EFAULT);
    *addr = tmp;
}

unsafe fn attest_fixture_att_inval_addr(
    _metadata: *mut __test_metadata,
    self_: *mut attest_fixture,
) {
    att_inval_addr_test(&mut (*self_).uvio_attest.arcb_addr, _metadata, self_);
    att_inval_addr_test(&mut (*self_).uvio_attest.add_data_addr, _metadata, self_);
    att_inval_addr_test(&mut (*self_).uvio_attest.meas_addr, _metadata, self_);
}

unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let fd: c_int = open(UV_PATH.as_ptr() as *const c_char, O_ACCMODE);

    if fd < 0 {
        ksft_exit_skip(
            b"No uv-device or cannot access /dev/uv\nEnable CONFIG_S390_UV_UAPI and check the access rights on /dev/uv.\n\0"
                .as_ptr() as *const c_char,
        );
    }
    close(fd);
    test_harness_run(argc, argv)
}
