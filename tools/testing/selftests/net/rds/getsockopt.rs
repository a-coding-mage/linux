// SPDX-License-Identifier: GPL-2.0
/*
 * Exercise the RDS getsockopt() paths that were converted to the
 * getsockopt_iter() / sockopt_t callback.
 *
 * Three distinct paths are covered:
 *
 *   - RDS_RECVERR and SO_RDS_TRANSPORT, which now return their int value
 *     through copy_to_iter() and report the written length in opt->optlen.
 *
 *   - RDS_INFO_*, which pins the userspace buffer with
 *     iov_iter_extract_pages() (including a non-zero starting page offset)
 *     and lets the info producers memcpy the snapshot in under a spinlock.
 *
 * The kvec (in-kernel buffer) -> -EOPNOTSUPP path of rds_info_getsockopt()
 * is not reachable from a userspace getsockopt() and so is not tested here.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* Dependencies from <sys/socket.h>, <sys/mman.h>, <unistd.h>, and
 * <linux/rds.h> are intentionally referenced in Rust form here.
 */
const AF_RDS: c_int = 21;

extern "C" {
    static mut errno: c_int;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn strnlen(s: *const c_char, maxlen: size_t) -> size_t;
}

#[allow(non_camel_case_types)]
type socklen_t = u32;
#[allow(non_camel_case_types)]
type size_t = usize;
#[allow(non_camel_case_types)]
type off_t = i64;

extern "C" {
    static SOL_RDS: c_int;
    static RDS_RECVERR: c_int;
    static SO_RDS_TRANSPORT: c_int;
    static RDS_INFO_COUNTERS: c_int;
    static RDS_TRANS_NONE: c_uint;
    static SOCK_SEQPACKET: c_int;
    static EINVAL: c_int;
    static ENOSPC: c_int;
    static _SC_PAGESIZE: c_int;
    static PROT_READ: c_int;
    static PROT_WRITE: c_int;
    static MAP_PRIVATE: c_int;
    static MAP_ANONYMOUS: c_int;
    static MAP_FAILED: *mut c_void;
}

#[repr(C)]
struct rds_info_counter {
    name: [u8; 32],
    value: u64,
}

struct rds {
    fd: c_int,
}

unsafe fn fixture_setup_rds(self_: *mut rds) {
    (*self_).fd = socket(AF_RDS, SOCK_SEQPACKET, 0);
    if (*self_).fd < 0 {
        skip_return(
            "AF_RDS unavailable (errno %d) - load the rds module",
            errno,
        );
    }
}

unsafe fn fixture_teardown_rds(self_: *mut rds) {
    if (*self_).fd >= 0 {
        close((*self_).fd);
    }
}

unsafe fn skip_return(_fmt: &str, _arg: c_int) {}
unsafe fn th_log(_fmt: &str, _arg: c_int) {}

/* RDS_RECVERR defaults to 0 and is reported back as a 4-byte int. */
unsafe fn recverr_default(self_: *mut rds) {
    let mut len: socklen_t = size_of::<c_int>() as socklen_t;
    let mut val: c_int = 0xdeadbeefu32 as c_int;

    assert_eq!(
        0,
        getsockopt(
            (*self_).fd,
            SOL_RDS,
            RDS_RECVERR,
            &mut val as *mut _ as *mut c_void,
            &mut len,
        )
    );
    assert_eq!(size_of::<c_int>() as socklen_t, len);
    assert_eq!(0, val);
}

/* A value set via setsockopt() must be readable back unchanged. */
unsafe fn recverr_set_get(self_: *mut rds) {
    let mut len: socklen_t = size_of::<c_int>() as socklen_t;
    let mut val: c_int = 1;

    assert_eq!(
        0,
        setsockopt(
            (*self_).fd,
            SOL_RDS,
            RDS_RECVERR,
            &val as *const _ as *const c_void,
            len,
        )
    );

    val = 0;
    assert_eq!(
        0,
        getsockopt(
            (*self_).fd,
            SOL_RDS,
            RDS_RECVERR,
            &mut val as *mut _ as *mut c_void,
            &mut len,
        )
    );
    assert_eq!(size_of::<c_int>() as socklen_t, len);
    assert_eq!(1, val);
}

/* A buffer smaller than an int is rejected with EINVAL, not silently. */
unsafe fn recverr_short_buffer(self_: *mut rds) {
    let mut len: socklen_t = (size_of::<c_int>() - 1) as socklen_t;
    let mut buf = [0 as c_char; size_of::<c_int>()];

    assert_eq!(
        -1,
        getsockopt(
            (*self_).fd,
            SOL_RDS,
            RDS_RECVERR,
            buf.as_mut_ptr() as *mut c_void,
            &mut len,
        )
    );
    assert_eq!(EINVAL, errno);
}

/* An unbound socket reports RDS_TRANS_NONE for SO_RDS_TRANSPORT. */
unsafe fn transport_unbound(self_: *mut rds) {
    let mut len: socklen_t = size_of::<c_int>() as socklen_t;
    let mut val: c_int = 0;

    assert_eq!(
        0,
        getsockopt(
            (*self_).fd,
            SOL_RDS,
            SO_RDS_TRANSPORT,
            &mut val as *mut _ as *mut c_void,
            &mut len,
        )
    );
    assert_eq!(size_of::<c_int>() as socklen_t, len);
    assert_eq!(RDS_TRANS_NONE, val as c_uint);
}

unsafe fn transport_short_buffer(self_: *mut rds) {
    let mut len: socklen_t = (size_of::<c_int>() - 1) as socklen_t;
    let mut buf = [0 as c_char; size_of::<c_int>()];

    assert_eq!(
        -1,
        getsockopt(
            (*self_).fd,
            SOL_RDS,
            SO_RDS_TRANSPORT,
            buf.as_mut_ptr() as *mut c_void,
            &mut len,
        )
    );
    assert_eq!(EINVAL, errno);
}

/*
 * RDS_INFO_COUNTERS with a zero-length buffer is the "probe" call: it must
 * fail with ENOSPC and report the required snapshot size in optlen.
 */
unsafe fn info_counters_probe(self_: *mut rds) {
    let mut len: socklen_t = 0;

    assert_eq!(
        -1,
        getsockopt(
            (*self_).fd,
            SOL_RDS,
            RDS_INFO_COUNTERS,
            ptr::null_mut(),
            &mut len,
        )
    );
    assert_eq!(ENOSPC, errno);
    assert!(len > 0);
    /* The snapshot is an array of fixed-size counter records. */
    assert_eq!(
        0,
        len % size_of::<rds_info_counter>() as socklen_t
    );
}

/*
 * A real snapshot into an unaligned userspace buffer exercises the
 * iov_iter_extract_pages() path, including the non-zero offset0 handling
 * that the patch reworked. Place the buffer at a non-page-aligned address
 * spanning into the next page to make sure multi-page pinning works too.
 */
unsafe fn info_counters_snapshot(self_: *mut rds) {
    let mut ctr: *mut rds_info_counter;
    let mut need: socklen_t = 0;
    let mut len: socklen_t;
    let pagesz: c_long = sysconf(_SC_PAGESIZE);
    let offset: size_t;
    let map_len: size_t;
    let mut i: c_uint;
    let n: c_uint;
    let region: *mut c_char;
    let buf: *mut c_char;
    let ret: c_int;

    /* Probe for the required size. */
    getsockopt(
        (*self_).fd,
        SOL_RDS,
        RDS_INFO_COUNTERS,
        ptr::null_mut(),
        &mut need,
    );
    assert!(need > 0);

    /*
     * Place the buffer at a non-page-aligned offset that runs past the
     * first page boundary, and size the mapping from the probed length so
     * the test keeps working if the counter set grows.
     */
    offset = pagesz as size_t - 64;
    map_len = ((offset + need as size_t + pagesz as size_t - 1) / pagesz as size_t)
        * pagesz as size_t;

    region = mmap(
        ptr::null_mut(),
        map_len,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut c_char;
    assert_ne!(MAP_FAILED as *mut c_char, region);

    buf = region.add(offset);

    /*
     * On success the RDS_INFO path returns the positive per-element size
     * (lens.each) rather than 0, and writes the full snapshot length back
     * into optlen.
     */
    len = need;
    ret = getsockopt(
        (*self_).fd,
        SOL_RDS,
        RDS_INFO_COUNTERS,
        buf as *mut c_void,
        &mut len,
    );
    if ret < 0 {
        th_log("getsockopt snapshot failed: errno %d", errno);
    }
    assert!(ret >= 0);
    assert_eq!(size_of::<rds_info_counter>() as c_int, ret);
    assert_eq!(need, len);

    /* The counter names must be NUL-terminated, non-empty strings. */
    ctr = buf as *mut rds_info_counter;
    n = len / size_of::<rds_info_counter>() as socklen_t;
    assert!(n > 0);
    i = 0;
    while i < n {
        let namelen: size_t = strnlen(
            (*ctr.add(i as usize)).name.as_ptr() as *const c_char,
            size_of_val(&(*ctr.add(i as usize)).name),
        );

        assert!(namelen > 0);
        assert!(namelen < size_of_val(&(*ctr.add(i as usize)).name));
        i += 1;
    }

    munmap(region as *mut c_void, map_len);
}

unsafe fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

/*
 * A non-zero but too-small buffer must report ENOSPC and the full required
 * length, without corrupting memory past the buffer.
 */
unsafe fn info_counters_short_buffer(self_: *mut rds) {
    let mut need: socklen_t = 0;
    let mut len: socklen_t;
    let mut small = [0 as c_char; size_of::<rds_info_counter>()];

    getsockopt(
        (*self_).fd,
        SOL_RDS,
        RDS_INFO_COUNTERS,
        ptr::null_mut(),
        &mut need,
    );
    assert!(need > 0);

    /* Ask with a buffer guaranteed smaller than the full snapshot. */
    if need <= size_of_val(&small) as socklen_t {
        skip_return("snapshot fits in one record; nothing to test", 0);
    }

    len = 1; /* < sizeof(struct rds_info_counter) */
    assert_eq!(
        -1,
        getsockopt(
            (*self_).fd,
            SOL_RDS,
            RDS_INFO_COUNTERS,
            small.as_mut_ptr() as *mut c_void,
            &mut len,
        )
    );
    assert_eq!(ENOSPC, errno);
    assert_eq!(need, len);
}

fn main() {}
