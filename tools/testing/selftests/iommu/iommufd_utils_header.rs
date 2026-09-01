/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2021-2022, NVIDIA CORPORATION & AFFILIATES */

/*
 * Rust translation of testing/selftests/iommu/iommufd_utils.h.
 *
 * C include dependencies intentionally remain external:
 * unistd.h, fcntl.h, ioctl.h, assert.h, poll.h, kselftest_harness.h, and
 * drivers/iommu/iommufd/iommufd_test.h.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

pub type __u32 = u32;
pub type __u64 = u64;
pub type size_t = usize;
pub type ssize_t = isize;

/* Hack to make assertions more readable */
#[macro_export]
macro_rules! _IOMMU_TEST_CMD {
    ($x:expr) => {
        IOMMU_TEST_CMD
    };
}

/* Imported from include/asm-generic/bitops/generic-non-atomic.h */
pub const BITS_PER_BYTE: c_ulong = 8;
pub const BITS_PER_LONG: c_ulong = __BITS_PER_LONG as c_ulong;

#[inline]
pub const fn BIT_MASK(nr: c_uint) -> c_ulong {
    1_c_ulong << ((nr as c_ulong) % BITS_PER_LONG)
}

#[inline]
pub const fn BIT_WORD(nr: c_uint) -> usize {
    ((nr as c_ulong) / BITS_PER_LONG) as usize
}

pub const IOPT_PAGES_ACCOUNT_NONE: c_int = 0;
pub const IOPT_PAGES_ACCOUNT_USER: c_int = 1;
pub const IOPT_PAGES_ACCOUNT_MM: c_int = 2;

#[inline]
pub const fn DIV_ROUND_UP(n: usize, d: usize) -> usize {
    (n + d - 1) / d
}

#[inline]
pub unsafe fn set_bit(nr: c_uint, addr: *mut c_ulong) {
    let mask = BIT_MASK(nr);
    let p = addr.add(BIT_WORD(nr));
    *p |= mask;
}

#[inline]
pub unsafe fn test_bit(nr: c_uint, addr: *mut c_ulong) -> bool {
    (1_c_ulong & (*addr.add(BIT_WORD(nr)) >> ((nr as c_ulong) & (BITS_PER_LONG - 1)))) != 0
}

pub static mut buffer: *mut c_void = ptr::null_mut();
pub static mut BUFFER_SIZE: c_ulong = 0;

pub static mut mfd_buffer: *mut c_void = ptr::null_mut();
pub static mut mfd: c_int = 0;

pub static mut PAGE_SIZE: c_ulong = 0;

#[macro_export]
macro_rules! sizeof_field {
    ($ty:ty, $member:tt) => {
        core::mem::size_of_val(&unsafe { (*(core::ptr::null::<$ty>())).$member })
    };
}

#[macro_export]
macro_rules! offsetofend {
    ($ty:ty, $member:tt) => {
        core::mem::offset_of!($ty, $member) + core::mem::size_of_val(&unsafe {
            (*(core::ptr::null::<$ty>())).$member
        })
    };
}

#[macro_export]
macro_rules! EXPECT_ERRNO {
    ($expected_errno:expr, $cmd:expr) => {{
        ASSERT_EQ!(-1, $cmd);
        EXPECT_EQ!($expected_errno, unsafe { errno });
    }};
}

#[macro_export]
macro_rules! test_err_mmap {
    ($self_:expr, $_errno:expr, $length:expr, $offset:expr) => {
        EXPECT_ERRNO!(
            $_errno,
            unsafe {
                mmap(
                    core::ptr::null_mut(),
                    $length,
                    PROT_READ | PROT_WRITE,
                    MAP_SHARED,
                    $self_.fd,
                    $offset,
                ) as c_long
            }
        )
    };
}

pub unsafe fn memfd_mmap(length: size_t, prot: c_int, flags: c_int, mfd_p: *mut c_int) -> *mut c_void {
    let mfd_flags = if (flags & MAP_HUGETLB) != 0 { MFD_HUGETLB } else { 0 };
    let local_mfd = memfd_create(c"buffer".as_ptr(), mfd_flags);
    let mut buf = MAP_FAILED;

    if local_mfd <= 0 {
        return MAP_FAILED;
    }
    if ftruncate(local_mfd, length as i64) != 0 {
        if buf == MAP_FAILED {
            close(local_mfd);
        }
        return buf;
    }
    *mfd_p = local_mfd;
    buf = mmap(ptr::null_mut(), length, prot, flags, local_mfd, 0);
    if buf == MAP_FAILED {
        close(local_mfd);
    }
    buf
}

/*
 * Have the kernel check the refcount on pages. I don't know why a freshly
 * mmap'd anon non-compound page starts out with a ref of 3
 */
#[macro_export]
macro_rules! check_refs {
    ($self_:expr, $_ptr:expr, $_length:expr, $_refs:expr) => {{
        let mut test_cmd = iommu_test_cmd {
            size: core::mem::size_of::<iommu_test_cmd>() as _,
            op: IOMMU_TEST_OP_MD_CHECK_REFS,
            check_refs: iommu_test_cmd_check_refs {
                length: $_length,
                uptr: $_ptr as usize,
                refs: $_refs,
            },
            ..unsafe { core::mem::zeroed() }
        };
        ASSERT_EQ!(
            0,
            unsafe {
                ioctl(
                    $self_.fd,
                    _IOMMU_TEST_CMD!(IOMMU_TEST_OP_MD_CHECK_REFS),
                    &mut test_cmd as *mut _ as *mut c_void,
                )
            }
        );
    }};
}

pub unsafe fn _test_cmd_mock_domain(
    fd: c_int,
    ioas_id: c_uint,
    stdev_id: *mut __u32,
    hwpt_id: *mut __u32,
    idev_id: *mut __u32,
) -> c_int {
    let mut cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_MOCK_DOMAIN,
        id: ioas_id,
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_TEST_CMD, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    if !stdev_id.is_null() {
        *stdev_id = cmd.mock_domain.out_stdev_id;
    }
    assert!(cmd.id != 0);
    if !hwpt_id.is_null() {
        *hwpt_id = cmd.mock_domain.out_hwpt_id;
    }
    if !idev_id.is_null() {
        *idev_id = cmd.mock_domain.out_idev_id;
    }
    0
}

pub unsafe fn _test_cmd_mock_domain_flags(
    fd: c_int,
    ioas_id: c_uint,
    stdev_flags: __u32,
    stdev_id: *mut __u32,
    hwpt_id: *mut __u32,
    idev_id: *mut __u32,
) -> c_int {
    let mut cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_MOCK_DOMAIN_FLAGS,
        id: ioas_id,
        mock_domain_flags: iommu_test_cmd_mock_domain_flags {
            dev_flags: stdev_flags,
            ..core::mem::zeroed()
        },
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_TEST_CMD, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    if !stdev_id.is_null() {
        *stdev_id = cmd.mock_domain_flags.out_stdev_id;
    }
    assert!(cmd.id != 0);
    if !hwpt_id.is_null() {
        *hwpt_id = cmd.mock_domain_flags.out_hwpt_id;
    }
    if !idev_id.is_null() {
        *idev_id = cmd.mock_domain_flags.out_idev_id;
    }
    0
}

pub unsafe fn _test_cmd_mock_domain_replace(
    fd: c_int,
    stdev_id: __u32,
    pt_id: __u32,
    hwpt_id: *mut __u32,
) -> c_int {
    let mut cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_MOCK_DOMAIN_REPLACE,
        id: stdev_id,
        mock_domain_replace: iommu_test_cmd_mock_domain_replace { pt_id },
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_TEST_CMD, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    if !hwpt_id.is_null() {
        *hwpt_id = cmd.mock_domain_replace.pt_id;
    }
    0
}

pub unsafe fn _test_cmd_hwpt_alloc(
    fd: c_int,
    device_id: __u32,
    pt_id: __u32,
    ft_id: __u32,
    flags: __u32,
    hwpt_id: *mut __u32,
    data_type: __u32,
    data: *mut c_void,
    data_len: size_t,
) -> c_int {
    let mut cmd = iommu_hwpt_alloc {
        size: size_of::<iommu_hwpt_alloc>() as _,
        flags,
        dev_id: device_id,
        pt_id,
        data_type,
        data_len,
        data_uptr: data as u64,
        fault_id: ft_id,
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_HWPT_ALLOC, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    if !hwpt_id.is_null() {
        *hwpt_id = cmd.out_hwpt_id;
    }
    0
}

pub unsafe fn _test_cmd_hwpt_invalidate(
    fd: c_int,
    hwpt_id: __u32,
    reqs: *mut c_void,
    data_type: u32,
    lreq: u32,
    nreqs: *mut u32,
) -> c_int {
    let mut cmd = iommu_hwpt_invalidate {
        size: size_of::<iommu_hwpt_invalidate>() as _,
        hwpt_id,
        data_type,
        data_uptr: reqs as u64,
        entry_len: lreq,
        entry_num: *nreqs,
        ..core::mem::zeroed()
    };
    let rc = ioctl(fd, IOMMU_HWPT_INVALIDATE, &mut cmd as *mut _ as *mut c_void);
    *nreqs = cmd.entry_num;
    rc
}

pub unsafe fn _test_cmd_viommu_invalidate(
    fd: c_int,
    viommu_id: __u32,
    reqs: *mut c_void,
    data_type: u32,
    lreq: u32,
    nreqs: *mut u32,
) -> c_int {
    let mut cmd = iommu_hwpt_invalidate {
        size: size_of::<iommu_hwpt_invalidate>() as _,
        hwpt_id: viommu_id,
        data_type,
        data_uptr: reqs as u64,
        entry_len: lreq,
        entry_num: *nreqs,
        ..core::mem::zeroed()
    };
    let rc = ioctl(fd, IOMMU_HWPT_INVALIDATE, &mut cmd as *mut _ as *mut c_void);
    *nreqs = cmd.entry_num;
    rc
}

pub unsafe fn _test_cmd_access_replace_ioas(fd: c_int, access_id: __u32, ioas_id: c_uint) -> c_int {
    let mut cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_ACCESS_REPLACE_IOAS,
        id: access_id,
        access_replace_ioas: iommu_test_cmd_access_replace_ioas { ioas_id },
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_TEST_CMD, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    0
}

pub unsafe fn _test_cmd_set_dirty_tracking(fd: c_int, hwpt_id: __u32, enabled: bool) -> c_int {
    let mut cmd = iommu_hwpt_set_dirty_tracking {
        size: size_of::<iommu_hwpt_set_dirty_tracking>() as _,
        flags: if enabled { IOMMU_HWPT_DIRTY_TRACKING_ENABLE } else { 0 },
        hwpt_id,
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_HWPT_SET_DIRTY_TRACKING, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return -errno;
    }
    0
}

pub unsafe fn _test_cmd_get_dirty_bitmap(
    fd: c_int,
    hwpt_id: __u32,
    length: size_t,
    iova: __u64,
    page_size: size_t,
    bitmap: *mut __u64,
    flags: __u32,
) -> c_int {
    let mut cmd = iommu_hwpt_get_dirty_bitmap {
        size: size_of::<iommu_hwpt_get_dirty_bitmap>() as _,
        hwpt_id,
        flags,
        iova,
        length,
        page_size,
        data: bitmap as usize,
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_HWPT_GET_DIRTY_BITMAP, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    0
}

pub unsafe fn _test_cmd_mock_domain_set_dirty(
    fd: c_int,
    hwpt_id: __u32,
    length: size_t,
    iova: __u64,
    page_size: size_t,
    bitmap: *mut __u64,
    dirty: *mut __u64,
) -> c_int {
    let mut cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_DIRTY,
        id: hwpt_id,
        dirty: iommu_test_cmd_dirty {
            iova,
            length,
            page_size,
            uptr: bitmap as usize,
            ..core::mem::zeroed()
        },
        ..core::mem::zeroed()
    };
    let ret = ioctl(
        fd,
        _IOMMU_TEST_CMD!(IOMMU_TEST_OP_DIRTY),
        &mut cmd as *mut _ as *mut c_void,
    );
    if ret != 0 {
        return -ret;
    }
    if !dirty.is_null() {
        *dirty = cmd.dirty.out_nr_dirty;
    }
    0
}

pub unsafe fn _test_mock_dirty_bitmaps(
    fd: c_int,
    hwpt_id: __u32,
    length: size_t,
    iova: __u64,
    page_size: size_t,
    pte_page_size: size_t,
    bitmap: *mut __u64,
    nbits: __u64,
    flags: __u32,
    _metadata: *mut __test_metadata,
) -> c_int {
    let npte: c_ulong = (pte_page_size / page_size) as c_ulong;
    let pteset: c_ulong = 2 * npte;
    let mut j: c_ulong;
    let mut i: c_ulong;
    let nr: c_ulong = if nbits / pteset as u64 != 0 { (nbits / pteset as u64) as c_ulong } else { 1 };
    let bitmap_size = DIV_ROUND_UP(nbits as usize, BITS_PER_BYTE as usize);
    let mut out_dirty: __u64 = 0;

    /* Mark all even bits as dirty in the mock domain */
    memset(bitmap as *mut c_void, 0, bitmap_size);
    i = 0;
    while i < nbits as c_ulong {
        set_bit(i as c_uint, bitmap as *mut c_ulong);
        i += pteset;
    }

    _test_cmd_mock_domain_set_dirty(fd, hwpt_id, length, iova, page_size, bitmap, &mut out_dirty);
    ASSERT_EQ!(nr, out_dirty);

    /* Expect all even bits as dirty in the user bitmap */
    memset(bitmap as *mut c_void, 0, bitmap_size);
    _test_cmd_get_dirty_bitmap(fd, hwpt_id, length, iova, page_size, bitmap, flags);
    /* Beware ASSERT_EQ() is two statements -- braces are not redundant! */
    i = 0;
    while i < nbits as c_ulong {
        j = 0;
        while j < pteset {
            ASSERT_EQ!(j < npte, test_bit((i + j) as c_uint, bitmap as *mut c_ulong));
            j += 1;
        }
        ASSERT_EQ!(!(i % pteset != 0), test_bit(i as c_uint, bitmap as *mut c_ulong));
        i += pteset;
    }

    memset(bitmap as *mut c_void, 0, bitmap_size);
    _test_cmd_get_dirty_bitmap(fd, hwpt_id, length, iova, page_size, bitmap, flags);

    /* It as read already -- expect all zeroes */
    i = 0;
    while i < nbits as c_ulong {
        j = 0;
        while j < pteset {
            ASSERT_EQ!(
                (j < npte) && ((flags & IOMMU_HWPT_GET_DIRTY_BITMAP_NO_CLEAR) != 0),
                test_bit((i + j) as c_uint, bitmap as *mut c_ulong)
            );
            j += 1;
        }
        i += pteset;
    }

    0
}

pub unsafe fn _test_cmd_create_access(
    fd: c_int,
    ioas_id: c_uint,
    access_id: *mut __u32,
    flags: c_uint,
) -> c_int {
    let mut cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_CREATE_ACCESS,
        id: ioas_id,
        create_access: iommu_test_cmd_create_access { flags, ..core::mem::zeroed() },
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_TEST_CMD, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    *access_id = cmd.create_access.out_access_fd;
    0
}

pub unsafe fn _test_cmd_destroy_access(access_id: c_uint) -> c_int {
    close(access_id as c_int)
}

pub unsafe fn _test_cmd_destroy_access_pages(fd: c_int, access_id: c_uint, access_pages_id: c_uint) -> c_int {
    let mut cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_DESTROY_ACCESS_PAGES,
        id: access_id,
        destroy_access_pages: iommu_test_cmd_destroy_access_pages { access_pages_id },
        ..core::mem::zeroed()
    };
    ioctl(fd, IOMMU_TEST_CMD, &mut cmd as *mut _ as *mut c_void)
}

pub unsafe fn _test_cmd_get_dmabuf(fd: c_int, len: size_t, out_fd: *mut c_int) -> c_int {
    let mut cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_DMABUF_GET,
        dmabuf_get: iommu_test_cmd_dmabuf_get { length: len, open_flags: O_CLOEXEC },
        ..core::mem::zeroed()
    };
    *out_fd = ioctl(fd, IOMMU_TEST_CMD, &mut cmd as *mut _ as *mut c_void);
    if *out_fd < 0 {
        return -1;
    }
    0
}

pub unsafe fn _test_cmd_revoke_dmabuf(fd: c_int, dmabuf_fd: c_int, revoked: bool) -> c_int {
    let mut cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_DMABUF_REVOKE,
        dmabuf_revoke: iommu_test_cmd_dmabuf_revoke { dmabuf_fd, revoked },
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_TEST_CMD, &mut cmd as *mut _ as *mut c_void);
    if ret < 0 {
        return -1;
    }
    0
}

pub unsafe fn _test_ioctl_destroy(fd: c_int, id: c_uint) -> c_int {
    let mut cmd = iommu_destroy { size: size_of::<iommu_destroy>() as _, id };
    ioctl(fd, IOMMU_DESTROY, &mut cmd as *mut _ as *mut c_void)
}

pub unsafe fn _test_ioctl_ioas_alloc(fd: c_int, id: *mut __u32) -> c_int {
    let mut cmd = iommu_ioas_alloc { size: size_of::<iommu_ioas_alloc>() as _, ..core::mem::zeroed() };
    let ret = ioctl(fd, IOMMU_IOAS_ALLOC, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    *id = cmd.out_ioas_id;
    0
}

pub unsafe fn _test_ioctl_ioas_map(
    fd: c_int,
    ioas_id: c_uint,
    buffer: *mut c_void,
    length: size_t,
    iova: *mut __u64,
    flags: c_uint,
) -> c_int {
    let mut cmd = iommu_ioas_map {
        size: size_of::<iommu_ioas_map>() as _,
        flags,
        ioas_id,
        user_va: buffer as usize,
        length,
        ..core::mem::zeroed()
    };
    if (flags & IOMMU_IOAS_MAP_FIXED_IOVA) != 0 {
        cmd.iova = *iova;
    }
    let ret = ioctl(fd, IOMMU_IOAS_MAP, &mut cmd as *mut _ as *mut c_void);
    *iova = cmd.iova;
    ret
}

pub unsafe fn _test_ioctl_ioas_unmap(
    fd: c_int,
    ioas_id: c_uint,
    iova: u64,
    length: size_t,
    out_len: *mut u64,
) -> c_int {
    let mut cmd = iommu_ioas_unmap {
        size: size_of::<iommu_ioas_unmap>() as _,
        ioas_id,
        iova,
        length,
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_IOAS_UNMAP, &mut cmd as *mut _ as *mut c_void);
    if !out_len.is_null() {
        *out_len = cmd.length;
    }
    ret
}

pub unsafe fn _test_ioctl_ioas_map_file(
    fd: c_int,
    ioas_id: c_uint,
    mfd: c_int,
    start: size_t,
    length: size_t,
    iova: *mut __u64,
    flags: c_uint,
) -> c_int {
    let mut cmd = iommu_ioas_map_file {
        size: size_of::<iommu_ioas_map_file>() as _,
        flags,
        ioas_id,
        fd: mfd,
        start,
        length,
        ..core::mem::zeroed()
    };
    if (flags & IOMMU_IOAS_MAP_FIXED_IOVA) != 0 {
        cmd.iova = *iova;
    }
    let ret = ioctl(fd, IOMMU_IOAS_MAP_FILE, &mut cmd as *mut _ as *mut c_void);
    *iova = cmd.iova;
    ret
}

pub unsafe fn _test_ioctl_set_temp_memory_limit(fd: c_int, limit: c_uint) -> c_int {
    let mut memlimit_cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_SET_TEMP_MEMORY_LIMIT,
        memory_limit: iommu_test_cmd_memory_limit { limit },
        ..core::mem::zeroed()
    };
    ioctl(
        fd,
        _IOMMU_TEST_CMD!(IOMMU_TEST_OP_SET_TEMP_MEMORY_LIMIT),
        &mut memlimit_cmd as *mut _ as *mut c_void,
    )
}

pub unsafe fn teardown_iommufd(fd: c_int, _metadata: *mut __test_metadata) {
    let mut test_cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_MD_CHECK_REFS,
        check_refs: iommu_test_cmd_check_refs {
            length: BUFFER_SIZE as _,
            uptr: buffer as usize,
            ..core::mem::zeroed()
        },
        ..core::mem::zeroed()
    };

    if fd == -1 {
        return;
    }

    EXPECT_EQ!(0, close(fd));

    let fd2 = open(c"/dev/iommu".as_ptr(), O_RDWR);
    EXPECT_NE!(-1, fd2);
    EXPECT_EQ!(
        0,
        ioctl(
            fd2,
            _IOMMU_TEST_CMD!(IOMMU_TEST_OP_MD_CHECK_REFS),
            &mut test_cmd as *mut _ as *mut c_void,
        )
    );
    EXPECT_EQ!(0, close(fd2));
}

/* @data can be NULL */
pub unsafe fn _test_cmd_get_hw_info(
    fd: c_int,
    device_id: __u32,
    data_type: __u32,
    data: *mut c_void,
    data_len: size_t,
    capabilities: *mut u32,
    max_pasid: *mut u8,
) -> c_int {
    let info = data as *mut iommu_test_hw_info;
    let mut cmd = iommu_hw_info {
        size: size_of::<iommu_hw_info>() as _,
        dev_id: device_id,
        data_len,
        in_data_type: data_type,
        data_uptr: data as u64,
        out_capabilities: 0,
        ..core::mem::zeroed()
    };
    if data_type != IOMMU_HW_INFO_TYPE_DEFAULT {
        cmd.flags |= IOMMU_HW_INFO_FLAG_INPUT_TYPE;
    }

    let ret = ioctl(fd, IOMMU_GET_HW_INFO, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }

    assert!(cmd.out_data_type == IOMMU_HW_INFO_TYPE_SELFTEST);

    /*
     * The struct iommu_test_hw_info should be the one defined
     * by the current kernel.
     */
    assert!(cmd.data_len == size_of::<iommu_test_hw_info>());

    /*
     * Trailing bytes should be 0 if user buffer is larger than
     * the data that kernel reports.
     */
    if data_len > cmd.data_len {
        let ptr = (data as *mut c_char).add(cmd.data_len);
        let mut idx: c_int = 0;
        while (idx as usize) < data_len - cmd.data_len {
            assert!(*ptr.add(idx as usize) == 0);
            idx += 1;
        }
    }

    if !info.is_null() {
        if data_len >= offset_of!(iommu_test_hw_info, test_reg) + size_of_val_field_test_reg(info) {
            assert!((*info).test_reg == IOMMU_HW_INFO_SELFTEST_REGVAL);
        }
        if data_len >= offset_of!(iommu_test_hw_info, flags) + size_of_val_field_flags(info) {
            assert!((*info).flags == 0);
        }
    }

    if !max_pasid.is_null() {
        *max_pasid = cmd.out_max_pasid_log2;
    }
    if !capabilities.is_null() {
        *capabilities = cmd.out_capabilities;
    }
    0
}

#[inline]
unsafe fn size_of_val_field_test_reg(info: *mut iommu_test_hw_info) -> usize {
    size_of_val(&(*info).test_reg)
}

#[inline]
unsafe fn size_of_val_field_flags(info: *mut iommu_test_hw_info) -> usize {
    size_of_val(&(*info).flags)
}

#[inline]
fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}

pub unsafe fn _test_ioctl_fault_alloc(fd: c_int, fault_id: *mut __u32, fault_fd: *mut __u32) -> c_int {
    let mut cmd = iommu_fault_alloc { size: size_of::<iommu_fault_alloc>() as _, ..core::mem::zeroed() };
    let ret = ioctl(fd, IOMMU_FAULT_QUEUE_ALLOC, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    *fault_id = cmd.out_fault_id;
    *fault_fd = cmd.out_fault_fd;
    0
}

pub unsafe fn _test_cmd_trigger_iopf(fd: c_int, device_id: __u32, pasid: __u32, fault_fd: __u32) -> c_int {
    let mut trigger_iopf_cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_TRIGGER_IOPF,
        trigger_iopf: iommu_test_cmd_trigger_iopf {
            dev_id: device_id,
            pasid,
            grpid: 0x2,
            perm: IOMMU_PGFAULT_PERM_READ | IOMMU_PGFAULT_PERM_WRITE,
            addr: 0xdeadbeaf,
        },
        ..core::mem::zeroed()
    };
    let mut response = iommu_hwpt_page_response {
        code: IOMMUFD_PAGE_RESP_SUCCESS,
        ..core::mem::zeroed()
    };
    let mut fault: iommu_hwpt_pgfault = core::mem::zeroed();

    let ret = ioctl(
        fd,
        _IOMMU_TEST_CMD!(IOMMU_TEST_OP_TRIGGER_IOPF),
        &mut trigger_iopf_cmd as *mut _ as *mut c_void,
    );
    if ret != 0 {
        return ret;
    }

    let mut bytes = read(fault_fd as c_int, &mut fault as *mut _ as *mut c_void, size_of::<iommu_hwpt_pgfault>());
    if bytes <= 0 {
        return -EIO;
    }

    response.cookie = fault.cookie;

    bytes = write(
        fault_fd as c_int,
        &mut response as *mut _ as *mut c_void,
        size_of::<iommu_hwpt_page_response>(),
    );
    if bytes <= 0 {
        return -EIO;
    }
    0
}

pub unsafe fn _test_cmd_viommu_alloc(
    fd: c_int,
    device_id: __u32,
    hwpt_id: __u32,
    flags: __u32,
    ty: __u32,
    data: *mut c_void,
    data_len: __u32,
    viommu_id: *mut __u32,
) -> c_int {
    let mut cmd = iommu_viommu_alloc {
        size: size_of::<iommu_viommu_alloc>() as _,
        flags,
        type_: ty,
        dev_id: device_id,
        hwpt_id,
        data_uptr: data as u64,
        data_len,
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_VIOMMU_ALLOC, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    if !viommu_id.is_null() {
        *viommu_id = cmd.out_viommu_id;
    }
    0
}

pub unsafe fn _test_cmd_vdevice_alloc(
    fd: c_int,
    viommu_id: __u32,
    idev_id: __u32,
    virt_id: __u64,
    vdev_id: *mut __u32,
) -> c_int {
    let mut cmd = iommu_vdevice_alloc {
        size: size_of::<iommu_vdevice_alloc>() as _,
        dev_id: idev_id,
        viommu_id,
        virt_id,
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_VDEVICE_ALLOC, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    if !vdev_id.is_null() {
        *vdev_id = cmd.out_vdevice_id;
    }
    0
}

pub unsafe fn _test_cmd_hw_queue_alloc(
    fd: c_int,
    viommu_id: __u32,
    ty: __u32,
    idx: __u32,
    base_addr: __u64,
    length: __u64,
    hw_queue_id: *mut __u32,
) -> c_int {
    let mut cmd = iommu_hw_queue_alloc {
        size: size_of::<iommu_hw_queue_alloc>() as _,
        viommu_id,
        type_: ty,
        index: idx,
        nesting_parent_iova: base_addr,
        length,
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_HW_QUEUE_ALLOC, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    if !hw_queue_id.is_null() {
        *hw_queue_id = cmd.out_hw_queue_id;
    }
    0
}

pub unsafe fn _test_cmd_veventq_alloc(
    fd: c_int,
    viommu_id: __u32,
    ty: __u32,
    depth: __u32,
    veventq_id: *mut __u32,
    veventq_fd: *mut __u32,
) -> c_int {
    let mut cmd = iommu_veventq_alloc {
        size: size_of::<iommu_veventq_alloc>() as _,
        type_: ty,
        veventq_depth: depth,
        viommu_id,
        ..core::mem::zeroed()
    };
    let ret = ioctl(fd, IOMMU_VEVENTQ_ALLOC, &mut cmd as *mut _ as *mut c_void);
    if ret != 0 {
        return ret;
    }
    if !veventq_id.is_null() {
        *veventq_id = cmd.out_veventq_id;
    }
    if !veventq_fd.is_null() {
        *veventq_fd = cmd.out_veventq_fd;
    }
    0
}

pub unsafe fn _test_cmd_trigger_vevents(fd: c_int, dev_id: __u32, mut nvevents: __u32) -> c_int {
    let mut trigger_vevent_cmd = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_TRIGGER_VEVENT,
        trigger_vevent: iommu_test_cmd_trigger_vevent { dev_id },
        ..core::mem::zeroed()
    };

    while nvevents != 0 {
        nvevents -= 1;
        if ioctl(
            fd,
            _IOMMU_TEST_CMD!(IOMMU_TEST_OP_TRIGGER_VEVENT),
            &mut trigger_vevent_cmd as *mut _ as *mut c_void,
        ) != 0 {
            return -1;
        }
    }
    0
}

pub unsafe fn _test_cmd_read_vevents(
    _fd: c_int,
    event_fd: __u32,
    nvevents: __u32,
    virt_id: __u32,
    prev_seq: *mut c_int,
) -> c_int {
    let mut pollfd_value = pollfd { fd: event_fd as c_int, events: POLLIN, revents: 0 };
    let mut ret = poll(&mut pollfd_value, 1, 1000);
    if ret < 0 {
        return -1;
    }

    let stride = size_of::<iommufd_vevent_header>() + size_of::<iommu_viommu_event_selftest>();
    let data = calloc(nvevents as usize, stride);
    if data.is_null() {
        errno = ENOMEM;
        return -1;
    }

    let bytes = read(event_fd as c_int, data, nvevents as usize * stride);
    if bytes <= 0 {
        errno = EFAULT;
        ret = -1;
        free(data);
        return ret;
    }

    let mut i: c_int = 0;
    while i < nvevents as c_int {
        let hdr = (data as *mut u8).add(i as usize * stride) as *mut iommufd_vevent_header;
        if ((*hdr).flags & IOMMU_VEVENTQ_FLAG_LOST_EVENTS) != 0 || (*hdr).sequence - *prev_seq > 1 {
            *prev_seq = (*hdr).sequence;
            errno = EOVERFLOW;
            ret = -1;
            free(data);
            return ret;
        }
        *prev_seq = (*hdr).sequence;
        let event = (data as *mut u8).add(size_of::<iommufd_vevent_header>()) as *mut iommu_viommu_event_selftest;
        if (*event).virt_id != virt_id {
            errno = EINVAL;
            ret = -1;
            free(data);
            return ret;
        }
        i += 1;
    }

    ret = 0;
    free(data);
    ret
}

pub unsafe fn _test_cmd_pasid_attach(fd: c_int, stdev_id: __u32, pasid: __u32, pt_id: __u32) -> c_int {
    let mut test_attach = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_PASID_ATTACH,
        id: stdev_id,
        pasid_attach: iommu_test_cmd_pasid_attach { pasid, pt_id },
        ..core::mem::zeroed()
    };
    ioctl(
        fd,
        _IOMMU_TEST_CMD!(IOMMU_TEST_OP_PASID_ATTACH),
        &mut test_attach as *mut _ as *mut c_void,
    )
}

pub unsafe fn _test_cmd_pasid_replace(fd: c_int, stdev_id: __u32, pasid: __u32, pt_id: __u32) -> c_int {
    let mut test_replace = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_PASID_REPLACE,
        id: stdev_id,
        pasid_replace: iommu_test_cmd_pasid_replace { pasid, pt_id },
        ..core::mem::zeroed()
    };
    ioctl(
        fd,
        _IOMMU_TEST_CMD!(IOMMU_TEST_OP_PASID_REPLACE),
        &mut test_replace as *mut _ as *mut c_void,
    )
}

pub unsafe fn _test_cmd_pasid_detach(fd: c_int, stdev_id: __u32, pasid: __u32) -> c_int {
    let mut test_detach = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_PASID_DETACH,
        id: stdev_id,
        pasid_detach: iommu_test_cmd_pasid_detach { pasid },
        ..core::mem::zeroed()
    };
    ioctl(
        fd,
        _IOMMU_TEST_CMD!(IOMMU_TEST_OP_PASID_DETACH),
        &mut test_detach as *mut _ as *mut c_void,
    )
}

pub unsafe fn test_cmd_pasid_check_hwpt(fd: c_int, stdev_id: __u32, pasid: __u32, hwpt_id: __u32) -> c_int {
    let mut test_pasid_check = iommu_test_cmd {
        size: size_of::<iommu_test_cmd>() as _,
        op: IOMMU_TEST_OP_PASID_CHECK_HWPT,
        id: stdev_id,
        pasid_check: iommu_test_cmd_pasid_check { pasid, hwpt_id },
        ..core::mem::zeroed()
    };
    ioctl(
        fd,
        _IOMMU_TEST_CMD!(IOMMU_TEST_OP_PASID_CHECK_HWPT),
        &mut test_pasid_check as *mut _ as *mut c_void,
    )
}

/*
 * C test-harness wrapper macros using self->fd/self->ioas_id/self->stdev_id are
 * preserved as intent comments because this isolated Rust header has no local
 * definition for the harness self object or assertion macro syntax.
 *
 * test_cmd_mock_domain, test_err_mock_domain, test_cmd_mock_domain_flags,
 * test_err_mock_domain_flags, test_cmd_mock_domain_replace,
 * test_err_mock_domain_replace, test_cmd_hwpt_alloc,
 * test_cmd_hwpt_alloc_iommupt, test_err_hwpt_alloc,
 * test_cmd_hwpt_alloc_nested, test_err_hwpt_alloc_nested,
 * test_cmd_hwpt_alloc_iopf, test_err_hwpt_alloc_iopf,
 * test_cmd_hwpt_check_iotlb, test_cmd_hwpt_check_iotlb_all,
 * test_cmd_dev_check_cache, test_cmd_dev_check_cache_all,
 * test_cmd_hwpt_invalidate, test_err_hwpt_invalidate,
 * test_cmd_viommu_invalidate, test_err_viommu_invalidate,
 * test_cmd_access_replace_ioas, test_cmd_set_dirty_tracking,
 * test_cmd_get_dirty_bitmap, test_cmd_mock_domain_set_dirty,
 * test_mock_dirty_bitmaps, test_cmd_create_access, test_cmd_destroy_access,
 * test_cmd_destroy_access_pages, test_err_destroy_access_pages,
 * test_cmd_get_dmabuf, test_cmd_revoke_dmabuf, test_ioctl_destroy,
 * test_ioctl_ioas_alloc, test_ioctl_ioas_map, test_err_ioctl_ioas_map,
 * test_ioctl_ioas_map_id, test_ioctl_ioas_map_fixed,
 * test_ioctl_ioas_map_fixed_id, test_err_ioctl_ioas_map_fixed,
 * test_ioctl_ioas_unmap, test_ioctl_ioas_unmap_id,
 * test_err_ioctl_ioas_unmap, test_ioctl_ioas_map_file,
 * test_err_ioctl_ioas_map_file, test_ioctl_ioas_map_id_file,
 * test_ioctl_ioas_map_fixed_file, test_ioctl_set_temp_memory_limit,
 * test_ioctl_set_default_memory_limit, test_cmd_get_hw_info,
 * test_err_get_hw_info, test_cmd_get_hw_capabilities,
 * test_cmd_get_hw_info_pasid, test_ioctl_fault_alloc,
 * test_cmd_trigger_iopf, test_cmd_trigger_iopf_pasid,
 * test_cmd_viommu_alloc, test_err_viommu_alloc, test_cmd_vdevice_alloc,
 * test_err_vdevice_alloc, test_cmd_hw_queue_alloc, test_err_hw_queue_alloc,
 * test_cmd_veventq_alloc, test_err_veventq_alloc, test_cmd_trigger_vevents,
 * test_cmd_read_vevents, test_err_read_vevents, test_cmd_pasid_attach,
 * test_err_pasid_attach, test_cmd_pasid_replace, test_err_pasid_replace,
 * test_cmd_pasid_detach.
 */

unsafe extern "C" {
    pub static mut errno: c_int;

    pub fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    pub fn ftruncate(fd: c_int, length: i64) -> c_int;
    pub fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: i64,
    ) -> *mut c_void;
    pub fn close(fd: c_int) -> c_int;
    pub fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    pub fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    pub fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
