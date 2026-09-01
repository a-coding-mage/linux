// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2021-2022, NVIDIA CORPORATION & AFFILIATES
 *
 * These tests are "kernel integrity" tests. They are looking for kernel
 * WARN/OOPS/kasn/etc splats triggered by kernel sanitizers & debugging
 * features. It does not attempt to verify that the system calls are doing what
 * they are supposed to do.
 *
 * The basic philosophy is to run a sequence of calls that will succeed and then
 * sweep every failure injection point on that call chain to look for
 * interesting things in error handling.
 *
 * This test is best run with:
 *  echo 1 > /proc/sys/kernel/panic_on_warn
 * If something is actually going wrong.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::null_mut;

type size_t = usize;
type ssize_t = isize;
type __u32 = u32;
type __u64 = u64;

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_ANONYMOUS: c_int = 0x20;
const _SC_PAGE_SIZE: c_int = 30;
const EFAULT: c_int = 14;

// Dependencies supplied by linux/vfio.h and iommufd_utils.h in the original C.
unsafe extern "C" {
    static mut PAGE_SIZE: size_t;
    static mut BUFFER_SIZE: size_t;
    static mut buffer: *mut c_void;
    static mut mfd_buffer: *mut c_void;
    static mut mfd: c_int;
    static mut errno: c_int;
    static mut stdout: *mut c_void;

    static IOMMU_IOAS_IOVA_RANGES: c_uint;
    static IOMMU_IOAS_ALLOW_IOVAS: c_uint;
    static IOMMU_IOAS_COPY: c_uint;
    static IOMMU_IOAS_MAP_WRITEABLE: __u32;
    static IOMMU_IOAS_MAP_READABLE: __u32;
    static IOMMU_IOAS_MAP_FIXED_IOVA: __u32;
    static MOCK_ACCESS_RW_WRITE: __u32;
    static MOCK_ACCESS_RW_SLOW_PATH: __u32;
    static MOCK_FLAGS_ACCESS_CREATE_NEEDS_PIN_PAGES: __u32;
    static IOMMU_TEST_OP_ACCESS_RW: __u32;
    static IOMMU_TEST_OP_ACCESS_PAGES: __u32;
    static IOMMU_TEST_IOTLB_DEFAULT: __u32;
    static MOCK_APERTURE_START: __u64;
    static MOCK_FLAGS_DEVICE_PASID: __u32;
    static IOMMU_HW_INFO_TYPE_DEFAULT: __u32;
    static IOMMU_HWPT_ALLOC_PASID: __u32;
    static IOMMU_HWPT_ALLOC_NEST_PARENT: __u32;
    static IOMMU_HWPT_DATA_NONE: __u32;
    static IOMMU_VIOMMU_TYPE_SELFTEST: __u32;
    static IOMMU_HW_QUEUE_TYPE_SELFTEST: __u32;
    static IOMMU_HWPT_FAULT_ID_VALID: __u32;
    static IOMMU_HWPT_DATA_SELFTEST: __u32;
    static IOMMU_VEVENTQ_TYPE_SELFTEST: __u32;

    fn strlen(s: *const c_char) -> size_t;
    fn openat(dfd: c_int, path: *const c_char, oflag: c_int, ...) -> c_int;
    fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn sysconf(name: c_int) -> isize;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn memfd_mmap(size: size_t, prot: c_int, flags: c_int, fd: *mut c_int) -> *mut c_void;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn dirfd(dirp: *mut DIR) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: isize) -> ssize_t;
    fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: isize) -> ssize_t;
    fn fflush(stream: *mut c_void) -> c_int;
    fn ioctl(fd: c_int, request: c_uint, ...) -> c_int;

    fn _test_ioctl_ioas_alloc(fd: c_int, ioas_id: *mut u32) -> c_int;
    fn _test_ioctl_set_temp_memory_limit(fd: c_int, limit: u32) -> c_int;
    fn _test_ioctl_ioas_map(
        fd: c_int,
        ioas_id: u32,
        uptr: *mut c_void,
        length: size_t,
        iova: *mut __u64,
        flags: __u32,
    ) -> c_int;
    fn _test_ioctl_ioas_map_file(
        fd: c_int,
        ioas_id: u32,
        mfd: c_int,
        offset: __u64,
        length: size_t,
        iova: *mut __u64,
        flags: __u32,
    ) -> c_int;
    fn _test_ioctl_ioas_unmap(
        fd: c_int,
        ioas_id: u32,
        iova: __u64,
        length: size_t,
        out_len: *mut __u64,
    ) -> c_int;
    fn _test_ioctl_destroy(fd: c_int, id: u32) -> c_int;
    fn _test_cmd_mock_domain(
        fd: c_int,
        ioas_id: u32,
        stdev_id: *mut __u32,
        hwpt_id: *mut __u32,
        idev_id: *mut __u32,
    ) -> c_int;
    fn _test_cmd_create_access(fd: c_int, ioas_id: u32, access_id: *mut u32, flags: u32) -> c_int;
    fn _test_cmd_destroy_access(access_id: u32) -> c_int;
    fn _test_cmd_destroy_access_pages(fd: c_int, access_id: u32, access_pages_id: u32) -> c_int;
    fn _test_cmd_mock_domain_flags(
        fd: c_int,
        ioas_id: u32,
        flags: u32,
        stdev_id: *mut u32,
        hwpt_id: *mut u32,
        idev_id: *mut u32,
    ) -> c_int;
    fn _test_cmd_get_hw_info(
        fd: c_int,
        idev_id: u32,
        info_type: u32,
        info: *mut iommu_test_hw_info,
        info_len: size_t,
        out_data_type: *mut u32,
        out_data_len: *mut u32,
    ) -> c_int;
    fn _test_cmd_hwpt_alloc(
        fd: c_int,
        idev_id: u32,
        pt_id: u32,
        fault_id: u32,
        flags: u32,
        hwpt_id: *mut u32,
        data_type: u32,
        data: *const c_void,
        data_len: size_t,
    ) -> c_int;
    fn _test_cmd_mock_domain_replace(
        fd: c_int,
        stdev_id: u32,
        pt_id: u32,
        hwpt_id: *mut u32,
    ) -> c_int;
    fn _test_cmd_viommu_alloc(
        fd: c_int,
        idev_id: u32,
        hwpt_id: u32,
        flags: u32,
        viommu_type: u32,
        data: *const c_void,
        data_len: size_t,
        viommu_id: *mut u32,
    ) -> c_int;
    fn _test_cmd_vdevice_alloc(
        fd: c_int,
        viommu_id: u32,
        idev_id: u32,
        virt_id: u32,
        vdev_id: *mut u32,
    ) -> c_int;
    fn _test_cmd_hw_queue_alloc(
        fd: c_int,
        viommu_id: u32,
        queue_type: u32,
        flags: u32,
        uptr: __u64,
        length: size_t,
        hw_queue_id: *mut u32,
    ) -> c_int;
    fn _test_ioctl_fault_alloc(fd: c_int, fault_id: *mut u32, fault_fd: *mut u32) -> c_int;
    fn _test_cmd_veventq_alloc(
        fd: c_int,
        viommu_id: u32,
        veventq_type: u32,
        depth: u32,
        veventq_id: *mut u32,
        veventq_fd: *mut u32,
    ) -> c_int;
    fn _test_cmd_pasid_attach(fd: c_int, stdev_id: u32, pasid: u32, hwpt_id: u32) -> c_int;
    fn _test_cmd_pasid_replace(fd: c_int, stdev_id: u32, pasid: u32, hwpt_id: u32) -> c_int;
    fn _test_cmd_pasid_detach(fd: c_int, stdev_id: u32, pasid: u32) -> c_int;
    fn teardown_iommufd(fd: c_int, metadata: *mut __test_metadata);
}

#[repr(C)]
struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
struct dirent {
    d_ino: u64,
    d_off: i64,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

#[repr(C)]
struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
struct iommu_iova_range {
    start: __u64,
    last: __u64,
}

#[repr(C)]
struct iommu_ioas_iova_ranges {
    size: u32,
    ioas_id: u32,
    num_iovas: u32,
    __reserved: u32,
    allowed_iovas: __u64,
}

#[repr(C)]
struct iommu_ioas_allow_iovas {
    size: u32,
    ioas_id: u32,
    num_iovas: u32,
    __reserved: u32,
    allowed_iovas: __u64,
}

#[repr(C)]
struct iommu_ioas_copy {
    size: u32,
    flags: u32,
    dst_ioas_id: u32,
    src_ioas_id: u32,
    length: __u64,
    dst_iova: __u64,
    src_iova: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct iommu_test_access_rw {
    iova: __u64,
    length: __u64,
    uptr: __u64,
    flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct iommu_test_access_pages {
    iova: __u64,
    length: __u64,
    uptr: __u64,
    out_access_pages_id: u32,
}

#[repr(C)]
union iommu_test_cmd_union {
    access_rw: iommu_test_access_rw,
    access_pages: iommu_test_access_pages,
}

#[repr(C)]
struct iommu_test_cmd {
    size: u32,
    op: u32,
    id: u32,
    __reserved: u32,
    u: iommu_test_cmd_union,
}

#[repr(C)]
struct iommu_hwpt_selftest {
    iotlb: u32,
}

#[repr(C)]
struct iommu_test_hw_info {
    _private: [u8; 0],
}

#[repr(C)]
struct fail_nth_state {
    proc_fd: c_int,
    iteration: c_uint,
}

#[repr(C)]
struct basic_fail_nth {
    fd: c_int,
    access_id: u32,
    stdev_id: u32,
    pasid: u32,
}

static mut have_fault_injection: bool = false;

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        core::mem::size_of_val(&$array) / core::mem::size_of_val(&$array[0])
    };
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_GT {
    ($left:expr, $right:expr) => {
        assert!($left > $right)
    };
}

macro_rules! ASSERT_LT {
    ($left:expr, $right:expr) => {
        assert!($left < $right)
    };
}

unsafe fn _IOMMU_TEST_CMD(op: u32) -> c_uint {
    op as c_uint
}

unsafe fn writeat(dfd: c_int, fn_: *const c_char, val: *const c_char) -> c_int {
    let val_len = strlen(val);
    let res: ssize_t;
    let fd: c_int;

    fd = openat(dfd, fn_, O_WRONLY);
    if fd == -1 {
        return -1;
    }
    res = write(fd, val as *const c_void, val_len);
    assert!(res == val_len as ssize_t);
    close(fd);
    0
}

// Original C used __attribute__((constructor)).
unsafe extern "C" fn setup_buffer() {
    PAGE_SIZE = sysconf(_SC_PAGE_SIZE) as size_t;

    BUFFER_SIZE = 2 * 1024 * 1024;

    buffer = mmap(
        null_mut(),
        BUFFER_SIZE,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    );

    mfd_buffer = memfd_mmap(BUFFER_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, &mut mfd);
}

/*
 * This sets up fail_injection in a way that is useful for this test.
 * It does not attempt to restore things back to how they were.
 */
// Original C used __attribute__((constructor)).
unsafe extern "C" fn setup_fault_injection() {
    let debugfs = opendir(c"/sys/kernel/debug/".as_ptr());
    let mut dent: *mut dirent;

    if debugfs.is_null() {
        return;
    }

    /* Allow any allocation call to be fault injected */
    if writeat(dirfd(debugfs), c"failslab/ignore-gfp-wait".as_ptr(), c"N".as_ptr()) != 0 {
        return;
    }
    writeat(
        dirfd(debugfs),
        c"fail_page_alloc/ignore-gfp-wait".as_ptr(),
        c"N".as_ptr(),
    );
    writeat(
        dirfd(debugfs),
        c"fail_page_alloc/ignore-gfp-highmem".as_ptr(),
        c"N".as_ptr(),
    );

    loop {
        dent = readdir(debugfs);
        if dent.is_null() {
            break;
        }
        let mut fn_: [c_char; 300] = [0; 300];

        if strncmp((*dent).d_name.as_ptr(), c"fail".as_ptr(), 4) != 0 {
            continue;
        }

        /* We are looking for kernel splats, quiet down the log */
        snprintf(
            fn_.as_mut_ptr(),
            size_of::<[c_char; 300]>(),
            c"%s/verbose".as_ptr(),
            (*dent).d_name.as_ptr(),
        );
        writeat(dirfd(debugfs), fn_.as_ptr(), c"0".as_ptr());
    }
    closedir(debugfs);
    have_fault_injection = true;
}

unsafe fn fail_nth_first(_metadata: *mut __test_metadata, nth_state: *mut fail_nth_state) {
    let mut buf: [c_char; 300] = [0; 300];

    snprintf(
        buf.as_mut_ptr(),
        size_of::<[c_char; 300]>(),
        c"/proc/self/task/%u/fail-nth".as_ptr(),
        getpid(),
    );
    (*nth_state).proc_fd = open(buf.as_ptr(), O_RDWR);
    ASSERT_NE!(-1, (*nth_state).proc_fd);
}

unsafe fn fail_nth_next(
    _metadata: *mut __test_metadata,
    nth_state: *mut fail_nth_state,
    test_result: c_int,
) -> bool {
    static disable_nth: [c_char; 2] = [b'0' as c_char, 0];
    let mut buf: [c_char; 300] = [0; 300];

    /*
     * This is just an arbitrary limit based on the current kernel
     * situation. Changes in the kernel can dramatically change the number of
     * required fault injection sites, so if this hits it doesn't
     * necessarily mean a test failure, just that the limit has to be made
     * bigger.
     */
    ASSERT_GT!(1000, (*nth_state).iteration);
    if (*nth_state).iteration != 0 {
        let mut res: ssize_t;
        let mut res2: ssize_t;

        buf[0] = 0;
        /*
         * Annoyingly disabling the nth can also fail. This means
         * the test passed without triggering failure
         */
        res = pread(
            (*nth_state).proc_fd,
            buf.as_mut_ptr() as *mut c_void,
            size_of::<[c_char; 300]>(),
            0,
        );
        if res == -1 && errno == EFAULT {
            buf[0] = b'1' as c_char;
            buf[1] = b'\n' as c_char;
            res = 2;
        }

        res2 = pwrite(
            (*nth_state).proc_fd,
            disable_nth.as_ptr() as *const c_void,
            ARRAY_SIZE!(disable_nth) - 1,
            0,
        );
        if res2 == -1 && errno == EFAULT {
            res2 = pwrite(
                (*nth_state).proc_fd,
                disable_nth.as_ptr() as *const c_void,
                ARRAY_SIZE!(disable_nth) - 1,
                0,
            );
            buf[0] = b'1' as c_char;
            buf[1] = b'\n' as c_char;
        }
        ASSERT_EQ!(ARRAY_SIZE!(disable_nth) - 1, res2 as usize);

        /* printf("  nth %u result=%d nth=%u\n", nth_state->iteration,
           test_result, atoi(buf)); */
        let _ = test_result;
        fflush(stdout);
        ASSERT_LT!(1, res);
        if res != 2 || buf[0] != b'0' as c_char || buf[1] != b'\n' as c_char {
            return false;
        }
    } else {
        /* printf("  nth %u result=%d\n", nth_state->iteration,
           test_result); */
        let _ = test_result;
    }
    (*nth_state).iteration += 1;
    true
}

/*
 * This is called during the test to start failure injection. It allows the test
 * to do some setup that has already been swept and thus reduce the required
 * iterations.
 */
#[no_mangle]
pub unsafe extern "C" fn __fail_nth_enable(
    _metadata: *mut __test_metadata,
    nth_state: *mut fail_nth_state,
) {
    let mut buf: [c_char; 300] = [0; 300];
    let len: size_t;

    if (*nth_state).iteration == 0 {
        return;
    }

    len = snprintf(
        buf.as_mut_ptr(),
        size_of::<[c_char; 300]>(),
        c"%u".as_ptr(),
        (*nth_state).iteration,
    ) as size_t;
    ASSERT_EQ!(
        len,
        pwrite(
            (*nth_state).proc_fd,
            buf.as_ptr() as *const c_void,
            len,
            0,
        ) as size_t
    );
}

macro_rules! fail_nth_enable {
    ($metadata:expr, $nth_state:expr) => {
        __fail_nth_enable($metadata, $nth_state)
    };
}

/*
 * C macro translated by spelling out the generated test_nth_* functions below:
 *
 * TEST_FAIL_NTH(fixture_name, name)
 */

unsafe fn basic_fail_nth_setup(self_: *mut basic_fail_nth) {
    (*self_).fd = -1;
    (*self_).access_id = 0;
    (*self_).stdev_id = 0;
    (*self_).pasid = 0; //test should use a non-zero value
}

unsafe fn basic_fail_nth_teardown(_metadata: *mut __test_metadata, self_: *mut basic_fail_nth) {
    let rc: c_int;

    if (*self_).access_id != 0 {
        /* The access FD holds the iommufd open until it closes */
        rc = _test_cmd_destroy_access((*self_).access_id);
        assert!(rc == 0);
    }
    if (*self_).pasid != 0 && (*self_).stdev_id != 0 {
        _test_cmd_pasid_detach((*self_).fd, (*self_).stdev_id, (*self_).pasid);
    }
    teardown_iommufd((*self_).fd, _metadata);
}

/* Cover ioas.c */
unsafe fn test_nth_basic(
    _metadata: *mut __test_metadata,
    self_: *mut basic_fail_nth,
    _nth_state: *mut fail_nth_state,
) -> c_int {
    let mut ranges: [iommu_iova_range; 10] = zeroed();
    let mut ioas_id: u32 = 0;
    let mut iova: __u64 = 0;

    fail_nth_enable!(_metadata, _nth_state);

    (*self_).fd = open(c"/dev/iommu".as_ptr(), O_RDWR);
    if (*self_).fd == -1 {
        return -1;
    }

    if _test_ioctl_ioas_alloc((*self_).fd, &mut ioas_id) != 0 {
        return -1;
    }

    {
        let mut ranges_cmd = iommu_ioas_iova_ranges {
            size: size_of::<iommu_ioas_iova_ranges>() as u32,
            num_iovas: ARRAY_SIZE!(ranges) as u32,
            ioas_id,
            __reserved: 0,
            allowed_iovas: ranges.as_mut_ptr() as usize as __u64,
        };
        if ioctl((*self_).fd, IOMMU_IOAS_IOVA_RANGES, &mut ranges_cmd) != 0 {
            return -1;
        }
    }

    {
        let mut allow_cmd = iommu_ioas_allow_iovas {
            size: size_of::<iommu_ioas_allow_iovas>() as u32,
            ioas_id,
            num_iovas: 1,
            __reserved: 0,
            allowed_iovas: ranges.as_mut_ptr() as usize as __u64,
        };

        ranges[0].start = 16 * 1024;
        ranges[0].last = BUFFER_SIZE as __u64 + 16 * 1024 * 600 - 1;
        if ioctl((*self_).fd, IOMMU_IOAS_ALLOW_IOVAS, &mut allow_cmd) != 0 {
            return -1;
        }
    }

    if _test_ioctl_ioas_map(
        (*self_).fd,
        ioas_id,
        buffer,
        BUFFER_SIZE,
        &mut iova,
        IOMMU_IOAS_MAP_WRITEABLE | IOMMU_IOAS_MAP_READABLE,
    ) != 0
    {
        return -1;
    }

    {
        let mut copy_cmd = iommu_ioas_copy {
            size: size_of::<iommu_ioas_copy>() as u32,
            flags: IOMMU_IOAS_MAP_WRITEABLE | IOMMU_IOAS_MAP_READABLE,
            dst_ioas_id: ioas_id,
            src_ioas_id: ioas_id,
            src_iova: iova,
            length: size_of::<[iommu_iova_range; 10]>() as __u64,
            dst_iova: 0,
        };

        if ioctl((*self_).fd, IOMMU_IOAS_COPY, &mut copy_cmd) != 0 {
            return -1;
        }
    }

    if _test_ioctl_ioas_unmap((*self_).fd, ioas_id, iova, BUFFER_SIZE, null_mut()) != 0 {
        return -1;
    }
    /* Failure path of no IOVA to unmap */
    _test_ioctl_ioas_unmap((*self_).fd, ioas_id, iova, BUFFER_SIZE, null_mut());
    0
}

/* iopt_area_fill_domains() and iopt_area_fill_domain() */
unsafe fn test_nth_map_domain(
    _metadata: *mut __test_metadata,
    self_: *mut basic_fail_nth,
    _nth_state: *mut fail_nth_state,
) -> c_int {
    let mut ioas_id: u32 = 0;
    let mut stdev_id: __u32 = 0;
    let mut hwpt_id: __u32 = 0;
    let mut iova: __u64 = 0;

    (*self_).fd = open(c"/dev/iommu".as_ptr(), O_RDWR);
    if (*self_).fd == -1 {
        return -1;
    }

    if _test_ioctl_ioas_alloc((*self_).fd, &mut ioas_id) != 0 {
        return -1;
    }

    if _test_ioctl_set_temp_memory_limit((*self_).fd, 32) != 0 {
        return -1;
    }

    fail_nth_enable!(_metadata, _nth_state);

    if _test_cmd_mock_domain((*self_).fd, ioas_id, &mut stdev_id, &mut hwpt_id, null_mut()) != 0 {
        return -1;
    }

    if _test_ioctl_ioas_map(
        (*self_).fd,
        ioas_id,
        buffer,
        262144,
        &mut iova,
        IOMMU_IOAS_MAP_WRITEABLE | IOMMU_IOAS_MAP_READABLE,
    ) != 0
    {
        return -1;
    }

    if _test_ioctl_destroy((*self_).fd, stdev_id) != 0 {
        return -1;
    }

    if _test_cmd_mock_domain((*self_).fd, ioas_id, &mut stdev_id, &mut hwpt_id, null_mut()) != 0 {
        return -1;
    }
    0
}

/* iopt_area_fill_domains() and iopt_area_fill_domain() */
unsafe fn test_nth_map_file_domain(
    _metadata: *mut __test_metadata,
    self_: *mut basic_fail_nth,
    _nth_state: *mut fail_nth_state,
) -> c_int {
    let mut ioas_id: u32 = 0;
    let mut stdev_id: __u32 = 0;
    let mut hwpt_id: __u32 = 0;
    let mut iova: __u64 = 0;

    (*self_).fd = open(c"/dev/iommu".as_ptr(), O_RDWR);
    if (*self_).fd == -1 {
        return -1;
    }

    if _test_ioctl_ioas_alloc((*self_).fd, &mut ioas_id) != 0 {
        return -1;
    }

    if _test_ioctl_set_temp_memory_limit((*self_).fd, 32) != 0 {
        return -1;
    }

    fail_nth_enable!(_metadata, _nth_state);

    if _test_cmd_mock_domain((*self_).fd, ioas_id, &mut stdev_id, &mut hwpt_id, null_mut()) != 0 {
        return -1;
    }

    if _test_ioctl_ioas_map_file(
        (*self_).fd,
        ioas_id,
        mfd,
        0,
        262144,
        &mut iova,
        IOMMU_IOAS_MAP_WRITEABLE | IOMMU_IOAS_MAP_READABLE,
    ) != 0
    {
        return -1;
    }

    if _test_ioctl_destroy((*self_).fd, stdev_id) != 0 {
        return -1;
    }

    if _test_cmd_mock_domain((*self_).fd, ioas_id, &mut stdev_id, &mut hwpt_id, null_mut()) != 0 {
        return -1;
    }
    0
}

unsafe fn test_nth_map_two_domains(
    _metadata: *mut __test_metadata,
    self_: *mut basic_fail_nth,
    _nth_state: *mut fail_nth_state,
) -> c_int {
    let mut ioas_id: u32 = 0;
    let mut stdev_id2: __u32 = 0;
    let mut stdev_id: __u32 = 0;
    let mut hwpt_id2: __u32 = 0;
    let mut hwpt_id: __u32 = 0;
    let mut iova: __u64 = 0;

    (*self_).fd = open(c"/dev/iommu".as_ptr(), O_RDWR);
    if (*self_).fd == -1 {
        return -1;
    }

    if _test_ioctl_ioas_alloc((*self_).fd, &mut ioas_id) != 0 {
        return -1;
    }

    if _test_ioctl_set_temp_memory_limit((*self_).fd, 32) != 0 {
        return -1;
    }

    if _test_cmd_mock_domain((*self_).fd, ioas_id, &mut stdev_id, &mut hwpt_id, null_mut()) != 0 {
        return -1;
    }

    fail_nth_enable!(_metadata, _nth_state);

    if _test_cmd_mock_domain((*self_).fd, ioas_id, &mut stdev_id2, &mut hwpt_id2, null_mut()) != 0 {
        return -1;
    }

    if _test_ioctl_ioas_map(
        (*self_).fd,
        ioas_id,
        buffer,
        262144,
        &mut iova,
        IOMMU_IOAS_MAP_WRITEABLE | IOMMU_IOAS_MAP_READABLE,
    ) != 0
    {
        return -1;
    }

    if _test_ioctl_destroy((*self_).fd, stdev_id) != 0 {
        return -1;
    }

    if _test_ioctl_destroy((*self_).fd, stdev_id2) != 0 {
        return -1;
    }

    if _test_cmd_mock_domain((*self_).fd, ioas_id, &mut stdev_id, &mut hwpt_id, null_mut()) != 0 {
        return -1;
    }
    if _test_cmd_mock_domain((*self_).fd, ioas_id, &mut stdev_id2, &mut hwpt_id2, null_mut()) != 0 {
        return -1;
    }
    0
}

unsafe fn test_nth_access_rw(
    _metadata: *mut __test_metadata,
    self_: *mut basic_fail_nth,
    _nth_state: *mut fail_nth_state,
) -> c_int {
    let mut tmp_big: [u64; 4096] = [0; 4096];
    let mut ioas_id: u32 = 0;
    let mut tmp: [u16; 32] = [0; 32];
    let mut iova: __u64 = 0;

    (*self_).fd = open(c"/dev/iommu".as_ptr(), O_RDWR);
    if (*self_).fd == -1 {
        return -1;
    }

    if _test_ioctl_ioas_alloc((*self_).fd, &mut ioas_id) != 0 {
        return -1;
    }

    if _test_ioctl_set_temp_memory_limit((*self_).fd, 32) != 0 {
        return -1;
    }

    if _test_ioctl_ioas_map(
        (*self_).fd,
        ioas_id,
        buffer,
        262144,
        &mut iova,
        IOMMU_IOAS_MAP_WRITEABLE | IOMMU_IOAS_MAP_READABLE,
    ) != 0
    {
        return -1;
    }

    fail_nth_enable!(_metadata, _nth_state);

    if _test_cmd_create_access((*self_).fd, ioas_id, &mut (*self_).access_id, 0) != 0 {
        return -1;
    }

    {
        let mut access_cmd = iommu_test_cmd {
            size: size_of::<iommu_test_cmd>() as u32,
            op: IOMMU_TEST_OP_ACCESS_RW,
            id: (*self_).access_id,
            __reserved: 0,
            u: iommu_test_cmd_union {
                access_rw: iommu_test_access_rw {
                    iova,
                    length: size_of::<[u16; 32]>() as __u64,
                    uptr: tmp.as_mut_ptr() as usize as __u64,
                    flags: 0,
                },
            },
        };

        // READ
        if ioctl((*self_).fd, _IOMMU_TEST_CMD(IOMMU_TEST_OP_ACCESS_RW), &mut access_cmd) != 0 {
            return -1;
        }

        access_cmd.u.access_rw.flags = MOCK_ACCESS_RW_WRITE;
        if ioctl((*self_).fd, _IOMMU_TEST_CMD(IOMMU_TEST_OP_ACCESS_RW), &mut access_cmd) != 0 {
            return -1;
        }

        access_cmd.u.access_rw.flags = MOCK_ACCESS_RW_SLOW_PATH;
        if ioctl((*self_).fd, _IOMMU_TEST_CMD(IOMMU_TEST_OP_ACCESS_RW), &mut access_cmd) != 0 {
            return -1;
        }
        access_cmd.u.access_rw.flags = MOCK_ACCESS_RW_SLOW_PATH | MOCK_ACCESS_RW_WRITE;
        if ioctl((*self_).fd, _IOMMU_TEST_CMD(IOMMU_TEST_OP_ACCESS_RW), &mut access_cmd) != 0 {
            return -1;
        }
    }

    {
        let mut access_cmd = iommu_test_cmd {
            size: size_of::<iommu_test_cmd>() as u32,
            op: IOMMU_TEST_OP_ACCESS_RW,
            id: (*self_).access_id,
            __reserved: 0,
            u: iommu_test_cmd_union {
                access_rw: iommu_test_access_rw {
                    iova,
                    flags: MOCK_ACCESS_RW_SLOW_PATH,
                    length: size_of::<[u64; 4096]>() as __u64,
                    uptr: tmp_big.as_mut_ptr() as usize as __u64,
                },
            },
        };

        if ioctl((*self_).fd, _IOMMU_TEST_CMD(IOMMU_TEST_OP_ACCESS_RW), &mut access_cmd) != 0 {
            return -1;
        }
    }
    if _test_cmd_destroy_access((*self_).access_id) != 0 {
        return -1;
    }
    (*self_).access_id = 0;
    0
}

/* pages.c access functions */
unsafe fn test_nth_access_pin(
    _metadata: *mut __test_metadata,
    self_: *mut basic_fail_nth,
    _nth_state: *mut fail_nth_state,
) -> c_int {
    let access_pages_id: u32;
    let mut ioas_id: u32 = 0;
    let mut iova: __u64 = 0;

    (*self_).fd = open(c"/dev/iommu".as_ptr(), O_RDWR);
    if (*self_).fd == -1 {
        return -1;
    }

    if _test_ioctl_ioas_alloc((*self_).fd, &mut ioas_id) != 0 {
        return -1;
    }

    if _test_ioctl_set_temp_memory_limit((*self_).fd, 32) != 0 {
        return -1;
    }

    if _test_ioctl_ioas_map(
        (*self_).fd,
        ioas_id,
        buffer,
        BUFFER_SIZE,
        &mut iova,
        IOMMU_IOAS_MAP_WRITEABLE | IOMMU_IOAS_MAP_READABLE,
    ) != 0
    {
        return -1;
    }

    if _test_cmd_create_access(
        (*self_).fd,
        ioas_id,
        &mut (*self_).access_id,
        MOCK_FLAGS_ACCESS_CREATE_NEEDS_PIN_PAGES,
    ) != 0
    {
        return -1;
    }

    fail_nth_enable!(_metadata, _nth_state);

    {
        let mut access_cmd = iommu_test_cmd {
            size: size_of::<iommu_test_cmd>() as u32,
            op: IOMMU_TEST_OP_ACCESS_PAGES,
            id: (*self_).access_id,
            __reserved: 0,
            u: iommu_test_cmd_union {
                access_pages: iommu_test_access_pages {
                    iova,
                    length: BUFFER_SIZE as __u64,
                    uptr: buffer as usize as __u64,
                    out_access_pages_id: 0,
                },
            },
        };

        if ioctl((*self_).fd, _IOMMU_TEST_CMD(IOMMU_TEST_OP_ACCESS_RW), &mut access_cmd) != 0 {
            return -1;
        }
        access_pages_id = access_cmd.u.access_pages.out_access_pages_id;
    }

    if _test_cmd_destroy_access_pages((*self_).fd, (*self_).access_id, access_pages_id) != 0 {
        return -1;
    }

    if _test_cmd_destroy_access((*self_).access_id) != 0 {
        return -1;
    }
    (*self_).access_id = 0;
    0
}

/* iopt_pages_fill_xarray() */
unsafe fn test_nth_access_pin_domain(
    _metadata: *mut __test_metadata,
    self_: *mut basic_fail_nth,
    _nth_state: *mut fail_nth_state,
) -> c_int {
    let access_pages_id: u32;
    let mut ioas_id: u32 = 0;
    let mut stdev_id: __u32 = 0;
    let mut hwpt_id: __u32 = 0;
    let mut iova: __u64 = 0;

    (*self_).fd = open(c"/dev/iommu".as_ptr(), O_RDWR);
    if (*self_).fd == -1 {
        return -1;
    }

    if _test_ioctl_ioas_alloc((*self_).fd, &mut ioas_id) != 0 {
        return -1;
    }

    if _test_ioctl_set_temp_memory_limit((*self_).fd, 32) != 0 {
        return -1;
    }

    if _test_cmd_mock_domain((*self_).fd, ioas_id, &mut stdev_id, &mut hwpt_id, null_mut()) != 0 {
        return -1;
    }

    if _test_ioctl_ioas_map(
        (*self_).fd,
        ioas_id,
        buffer,
        BUFFER_SIZE,
        &mut iova,
        IOMMU_IOAS_MAP_WRITEABLE | IOMMU_IOAS_MAP_READABLE,
    ) != 0
    {
        return -1;
    }

    if _test_cmd_create_access(
        (*self_).fd,
        ioas_id,
        &mut (*self_).access_id,
        MOCK_FLAGS_ACCESS_CREATE_NEEDS_PIN_PAGES,
    ) != 0
    {
        return -1;
    }

    fail_nth_enable!(_metadata, _nth_state);

    {
        let mut access_cmd = iommu_test_cmd {
            size: size_of::<iommu_test_cmd>() as u32,
            op: IOMMU_TEST_OP_ACCESS_PAGES,
            id: (*self_).access_id,
            __reserved: 0,
            u: iommu_test_cmd_union {
                access_pages: iommu_test_access_pages {
                    iova,
                    length: BUFFER_SIZE as __u64,
                    uptr: buffer as usize as __u64,
                    out_access_pages_id: 0,
                },
            },
        };

        if ioctl((*self_).fd, _IOMMU_TEST_CMD(IOMMU_TEST_OP_ACCESS_RW), &mut access_cmd) != 0 {
            return -1;
        }
        access_pages_id = access_cmd.u.access_pages.out_access_pages_id;
    }

    if _test_cmd_destroy_access_pages((*self_).fd, (*self_).access_id, access_pages_id) != 0 {
        return -1;
    }

    if _test_cmd_destroy_access((*self_).access_id) != 0 {
        return -1;
    }
    (*self_).access_id = 0;

    if _test_ioctl_destroy((*self_).fd, stdev_id) != 0 {
        return -1;
    }
    0
}

/* device.c */
unsafe fn test_nth_device(
    _metadata: *mut __test_metadata,
    self_: *mut basic_fail_nth,
    _nth_state: *mut fail_nth_state,
) -> c_int {
    let mut data = iommu_hwpt_selftest {
        iotlb: IOMMU_TEST_IOTLB_DEFAULT,
    };
    let mut info: iommu_test_hw_info = zeroed();
    let mut fault_id: u32 = 0;
    let mut fault_fd: u32 = 0;
    let mut veventq_id: u32 = 0;
    let mut veventq_fd: u32 = 0;
    let mut fault_hwpt_id: u32 = 0;
    let mut test_hwpt_id: u32 = 0;
    let mut ioas_id: u32 = 0;
    let mut ioas_id2: u32 = 0;
    let mut idev_id: u32 = 0;
    let mut hwpt_id: u32 = 0;
    let mut viommu_id: u32 = 0;
    let mut hw_queue_id: u32 = 0;
    let mut vdev_id: u32 = 0;
    let mut iova: __u64;

    (*self_).fd = open(c"/dev/iommu".as_ptr(), O_RDWR);
    if (*self_).fd == -1 {
        return -1;
    }

    if _test_ioctl_ioas_alloc((*self_).fd, &mut ioas_id) != 0 {
        return -1;
    }

    if _test_ioctl_ioas_alloc((*self_).fd, &mut ioas_id2) != 0 {
        return -1;
    }

    iova = MOCK_APERTURE_START;
    if _test_ioctl_ioas_map(
        (*self_).fd,
        ioas_id,
        buffer,
        PAGE_SIZE,
        &mut iova,
        IOMMU_IOAS_MAP_FIXED_IOVA | IOMMU_IOAS_MAP_WRITEABLE | IOMMU_IOAS_MAP_READABLE,
    ) != 0
    {
        return -1;
    }
    if _test_ioctl_ioas_map(
        (*self_).fd,
        ioas_id2,
        buffer,
        PAGE_SIZE,
        &mut iova,
        IOMMU_IOAS_MAP_FIXED_IOVA | IOMMU_IOAS_MAP_WRITEABLE | IOMMU_IOAS_MAP_READABLE,
    ) != 0
    {
        return -1;
    }

    fail_nth_enable!(_metadata, _nth_state);

    if _test_cmd_mock_domain_flags(
        (*self_).fd,
        ioas_id,
        MOCK_FLAGS_DEVICE_PASID,
        &mut (*self_).stdev_id,
        null_mut(),
        &mut idev_id,
    ) != 0
    {
        return -1;
    }

    if _test_cmd_get_hw_info(
        (*self_).fd,
        idev_id,
        IOMMU_HW_INFO_TYPE_DEFAULT,
        &mut info,
        size_of::<iommu_test_hw_info>(),
        null_mut(),
        null_mut(),
    ) != 0
    {
        return -1;
    }

    if _test_cmd_hwpt_alloc(
        (*self_).fd,
        idev_id,
        ioas_id,
        0,
        IOMMU_HWPT_ALLOC_PASID,
        &mut hwpt_id,
        IOMMU_HWPT_DATA_NONE,
        null_mut(),
        0,
    ) != 0
    {
        return -1;
    }

    if _test_cmd_mock_domain_replace((*self_).fd, (*self_).stdev_id, ioas_id2, null_mut()) != 0 {
        return -1;
    }

    if _test_cmd_mock_domain_replace((*self_).fd, (*self_).stdev_id, hwpt_id, null_mut()) != 0 {
        return -1;
    }

    if _test_cmd_hwpt_alloc(
        (*self_).fd,
        idev_id,
        ioas_id,
        0,
        IOMMU_HWPT_ALLOC_NEST_PARENT | IOMMU_HWPT_ALLOC_PASID,
        &mut hwpt_id,
        IOMMU_HWPT_DATA_NONE,
        null_mut(),
        0,
    ) != 0
    {
        return -1;
    }

    if _test_cmd_viommu_alloc(
        (*self_).fd,
        idev_id,
        hwpt_id,
        0,
        IOMMU_VIOMMU_TYPE_SELFTEST,
        null_mut(),
        0,
        &mut viommu_id,
    ) != 0
    {
        return -1;
    }

    if _test_cmd_vdevice_alloc((*self_).fd, viommu_id, idev_id, 0, &mut vdev_id) != 0 {
        return -1;
    }

    if _test_cmd_hw_queue_alloc(
        (*self_).fd,
        viommu_id,
        IOMMU_HW_QUEUE_TYPE_SELFTEST,
        0,
        iova,
        PAGE_SIZE,
        &mut hw_queue_id,
    ) != 0
    {
        return -1;
    }

    if _test_ioctl_fault_alloc((*self_).fd, &mut fault_id, &mut fault_fd) != 0 {
        return -1;
    }
    close(fault_fd as c_int);

    if _test_cmd_hwpt_alloc(
        (*self_).fd,
        idev_id,
        hwpt_id,
        fault_id,
        IOMMU_HWPT_FAULT_ID_VALID,
        &mut fault_hwpt_id,
        IOMMU_HWPT_DATA_SELFTEST,
        &mut data as *mut iommu_hwpt_selftest as *const c_void,
        size_of::<iommu_hwpt_selftest>(),
    ) != 0
    {
        return -1;
    }

    if _test_cmd_veventq_alloc(
        (*self_).fd,
        viommu_id,
        IOMMU_VEVENTQ_TYPE_SELFTEST,
        2,
        &mut veventq_id,
        &mut veventq_fd,
    ) != 0
    {
        return -1;
    }
    close(veventq_fd as c_int);

    if _test_cmd_hwpt_alloc(
        (*self_).fd,
        idev_id,
        ioas_id,
        0,
        IOMMU_HWPT_ALLOC_PASID,
        &mut test_hwpt_id,
        IOMMU_HWPT_DATA_NONE,
        null_mut(),
        0,
    ) != 0
    {
        return -1;
    }

    /* Tests for pasid attach/replace/detach */

    (*self_).pasid = 200;

    if _test_cmd_pasid_attach((*self_).fd, (*self_).stdev_id, (*self_).pasid, hwpt_id) != 0 {
        (*self_).pasid = 0;
        return -1;
    }

    if _test_cmd_pasid_replace((*self_).fd, (*self_).stdev_id, (*self_).pasid, test_hwpt_id) != 0 {
        return -1;
    }

    if _test_cmd_pasid_detach((*self_).fd, (*self_).stdev_id, (*self_).pasid) != 0 {
        return -1;
    }

    (*self_).pasid = 0;

    0
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
