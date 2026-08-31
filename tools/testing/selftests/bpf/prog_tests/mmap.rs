// SPDX-License-Identifier: GPL-2.0
// C dependencies in the original file:
// #include <test_progs.h>
// #include <sys/mman.h>
// #include "test_mmap.skel.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{null_mut, write_bytes};

type size_t = usize;
type __u32 = u32;
type __u64 = u64;
type u64 = u64;

const _SC_PAGE_SIZE: c_int = 30;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PROT_EXEC: c_int = 0x4;
const MAP_SHARED: c_int = 0x01;
const MAP_FIXED: c_int = 0x10;
const MAP_ANONYMOUS: c_int = 0x20;
const EBUSY: c_int = 16;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

macro_rules! goto_cleanup {
    ($bss_mmaped:expr, $bss_sz:expr, $map_mmaped:expr, $map_sz:expr, $skel:expr) => {{
        if !$bss_mmaped.is_null() {
            CHECK_FAIL(munmap($bss_mmaped, $bss_sz) != 0);
        }
        if !$map_mmaped.is_null() {
            CHECK_FAIL(munmap($map_mmaped, $map_sz) != 0);
        }
        test_mmap__destroy($skel);
    }};
}

#[repr(C)]
struct map_data {
    val: [__u64; 512 * 4],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map_info {
    _private: [u8; 0],
    id: __u32,
}

#[repr(C)]
struct test_mmap__bss {
    in_val: __u64,
    out_val: __u64,
}

#[repr(C)]
struct test_mmap__maps {
    rdonly_map: *mut bpf_map,
    data_map: *mut bpf_map,
    bss: *mut bpf_map,
}

#[repr(C)]
struct test_mmap {
    maps: test_mmap__maps,
    bss: *mut test_mmap__bss,
}

unsafe extern "C" {
    fn sysconf(name: c_int) -> c_long;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn test_mmap__open() -> *mut test_mmap;
    fn test_mmap__load(obj: *mut test_mmap) -> c_int;
    fn test_mmap__attach(obj: *mut test_mmap) -> c_int;
    fn test_mmap__destroy(obj: *mut test_mmap);

    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: __u32) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, info_len: *mut __u32) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_freeze(fd: c_int) -> c_int;
    fn bpf_map_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_map_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;

    fn CHECK(cond: bool, name: *const u8, fmt: *const u8, ...) -> bool;
    fn CHECK_FAIL(cond: bool) -> bool;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn roundup_page(sz: size_t) -> size_t {
    let page_size: c_long = sysconf(_SC_PAGE_SIZE);
    (sz + page_size as size_t - 1) / page_size as size_t * page_size as size_t
}

pub unsafe fn test_mmap() {
    let bss_sz: size_t = roundup_page(size_of::<test_mmap__bss>());
    let map_sz: size_t = roundup_page(size_of::<map_data>());
    let zero: c_int = 0;
    let one: c_int = 1;
    let two: c_int = 2;
    let far: c_int = 1500;
    let page_size: c_long = sysconf(_SC_PAGE_SIZE);
    let mut err: c_int;
    let mut duration: c_int = 0;
    let mut i: c_int;
    let data_map_fd: c_int;
    let data_map_id: c_int;
    let mut tmp_fd: c_int;
    let rdmap_fd: c_int;
    let data_map: *mut bpf_map;
    let bss_map: *mut bpf_map;
    let mut bss_mmaped: *mut c_void = null_mut();
    let mut map_mmaped: *mut c_void = null_mut();
    let mut tmp0: *mut c_void;
    let mut tmp1: *mut c_void;
    let mut tmp2: *mut c_void;
    let mut bss_data: *mut test_mmap__bss;
    let mut map_info: bpf_map_info = core::mem::zeroed();
    let mut map_info_sz: __u32 = size_of::<bpf_map_info>() as __u32;
    let mut map_data: *mut map_data;
    let mut skel: *mut test_mmap;
    let mut val: __u64 = 0;

    skel = test_mmap__open();
    if CHECK(skel.is_null(), c"skel_open".as_ptr() as *const u8, c"skeleton open failed\n".as_ptr() as *const u8) {
        return;
    }

    err = bpf_map__set_max_entries((*skel).maps.rdonly_map, page_size as __u32);
    if CHECK(err != 0, c"bpf_map__set_max_entries".as_ptr() as *const u8, c"bpf_map__set_max_entries failed\n".as_ptr() as *const u8) {
        goto_cleanup!(
            bss_mmaped,
            bss_sz,
            map_mmaped,
            map_sz,
            skel
        );
        return;
    }

    /* at least 4 pages of data */
    err = bpf_map__set_max_entries(
        (*skel).maps.data_map,
        (4 * (page_size as size_t / size_of::<u64>())) as __u32,
    );
    if CHECK(err != 0, c"bpf_map__set_max_entries".as_ptr() as *const u8, c"bpf_map__set_max_entries failed\n".as_ptr() as *const u8) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    err = test_mmap__load(skel);
    if CHECK(err != 0, c"skel_load".as_ptr() as *const u8, c"skeleton load failed\n".as_ptr() as *const u8) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    bss_map = (*skel).maps.bss;
    data_map = (*skel).maps.data_map;
    data_map_fd = bpf_map__fd(data_map);

    rdmap_fd = bpf_map__fd((*skel).maps.rdonly_map);
    tmp1 = mmap(null_mut(), page_size as size_t, PROT_READ | PROT_WRITE, MAP_SHARED, rdmap_fd, 0);
    if CHECK(tmp1 != MAP_FAILED, c"rdonly_write_mmap".as_ptr() as *const u8, c"unexpected success\n".as_ptr() as *const u8) {
        munmap(tmp1, page_size as size_t);
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }
    /* now double-check if it's mmap()'able at all */
    tmp1 = mmap(null_mut(), page_size as size_t, PROT_READ, MAP_SHARED, rdmap_fd, 0);
    if CHECK(tmp1 == MAP_FAILED, c"rdonly_read_mmap".as_ptr() as *const u8, c"failed: %d\n".as_ptr() as *const u8, errno()) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    /* get map's ID */
    write_bytes((&mut map_info as *mut bpf_map_info).cast::<u8>(), 0, map_info_sz as usize);
    err = bpf_map_get_info_by_fd(data_map_fd, &mut map_info, &mut map_info_sz);
    if CHECK(err != 0, c"map_get_info".as_ptr() as *const u8, c"failed %d\n".as_ptr() as *const u8, errno()) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }
    data_map_id = map_info.id as c_int;

    /* mmap BSS map */
    bss_mmaped = mmap(
        null_mut(),
        bss_sz,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        bpf_map__fd(bss_map),
        0,
    );
    if CHECK(bss_mmaped == MAP_FAILED, c"bss_mmap".as_ptr() as *const u8, c".bss mmap failed: %d\n".as_ptr() as *const u8, errno()) {
        bss_mmaped = null_mut();
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }
    /* map as R/W first */
    map_mmaped = mmap(null_mut(), map_sz, PROT_READ | PROT_WRITE, MAP_SHARED, data_map_fd, 0);
    if CHECK(map_mmaped == MAP_FAILED, c"data_mmap".as_ptr() as *const u8, c"data_map mmap failed: %d\n".as_ptr() as *const u8, errno()) {
        map_mmaped = null_mut();
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    bss_data = bss_mmaped.cast::<test_mmap__bss>();
    map_data = map_mmaped.cast::<map_data>();

    CHECK_FAIL((*bss_data).in_val != 0);
    CHECK_FAIL((*bss_data).out_val != 0);
    CHECK_FAIL((*(*skel).bss).in_val != 0);
    CHECK_FAIL((*(*skel).bss).out_val != 0);
    CHECK_FAIL((*map_data).val[0] != 0);
    CHECK_FAIL((*map_data).val[1] != 0);
    CHECK_FAIL((*map_data).val[2] != 0);
    CHECK_FAIL((*map_data).val[far as usize] != 0);

    err = test_mmap__attach(skel);
    if CHECK(err != 0, c"attach_raw_tp".as_ptr() as *const u8, c"err %d\n".as_ptr() as *const u8, err) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    (*bss_data).in_val = 123;
    val = 111;
    CHECK_FAIL(bpf_map_update_elem(data_map_fd, (&zero as *const c_int).cast::<c_void>(), (&val as *const __u64).cast::<c_void>(), 0) != 0);

    usleep(1);

    CHECK_FAIL((*bss_data).in_val != 123);
    CHECK_FAIL((*bss_data).out_val != 123);
    CHECK_FAIL((*(*skel).bss).in_val != 123);
    CHECK_FAIL((*(*skel).bss).out_val != 123);
    CHECK_FAIL((*map_data).val[0] != 111);
    CHECK_FAIL((*map_data).val[1] != 222);
    CHECK_FAIL((*map_data).val[2] != 123);
    CHECK_FAIL((*map_data).val[far as usize] != 3 * 123);

    CHECK_FAIL(bpf_map_lookup_elem(data_map_fd, (&zero as *const c_int).cast::<c_void>(), (&mut val as *mut __u64).cast::<c_void>()) != 0);
    CHECK_FAIL(val != 111);
    CHECK_FAIL(bpf_map_lookup_elem(data_map_fd, (&one as *const c_int).cast::<c_void>(), (&mut val as *mut __u64).cast::<c_void>()) != 0);
    CHECK_FAIL(val != 222);
    CHECK_FAIL(bpf_map_lookup_elem(data_map_fd, (&two as *const c_int).cast::<c_void>(), (&mut val as *mut __u64).cast::<c_void>()) != 0);
    CHECK_FAIL(val != 123);
    CHECK_FAIL(bpf_map_lookup_elem(data_map_fd, (&far as *const c_int).cast::<c_void>(), (&mut val as *mut __u64).cast::<c_void>()) != 0);
    CHECK_FAIL(val != 3 * 123);

    /* data_map freeze should fail due to R/W mmap() */
    err = bpf_map_freeze(data_map_fd);
    if CHECK(err == 0 || errno() != EBUSY, c"no_freeze".as_ptr() as *const u8, c"data_map freeze succeeded: err=%d, errno=%d\n".as_ptr() as *const u8, err, errno()) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    err = mprotect(map_mmaped, map_sz, PROT_READ);
    if CHECK(err != 0, c"mprotect_ro".as_ptr() as *const u8, c"mprotect to r/o failed %d\n".as_ptr() as *const u8, errno()) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    /* unmap R/W mapping */
    err = munmap(map_mmaped, map_sz);
    map_mmaped = null_mut();
    if CHECK(err != 0, c"data_map_munmap".as_ptr() as *const u8, c"data_map munmap failed: %d\n".as_ptr() as *const u8, errno()) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    /* re-map as R/O now */
    map_mmaped = mmap(null_mut(), map_sz, PROT_READ, MAP_SHARED, data_map_fd, 0);
    if CHECK(map_mmaped == MAP_FAILED, c"data_mmap".as_ptr() as *const u8, c"data_map R/O mmap failed: %d\n".as_ptr() as *const u8, errno()) {
        map_mmaped = null_mut();
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }
    err = mprotect(map_mmaped, map_sz, PROT_WRITE);
    if CHECK(err == 0, c"mprotect_wr".as_ptr() as *const u8, c"mprotect() succeeded unexpectedly!\n".as_ptr() as *const u8) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }
    err = mprotect(map_mmaped, map_sz, PROT_EXEC);
    if CHECK(err == 0, c"mprotect_ex".as_ptr() as *const u8, c"mprotect() succeeded unexpectedly!\n".as_ptr() as *const u8) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }
    map_data = map_mmaped.cast::<map_data>();

    /* map/unmap in a loop to test ref counting */
    i = 0;
    while i < 10 {
        let flags: c_int = if i % 2 != 0 { PROT_READ } else { PROT_WRITE };
        let p: *mut c_void;

        p = mmap(null_mut(), map_sz, flags, MAP_SHARED, data_map_fd, 0);
        if CHECK_FAIL(p == MAP_FAILED) {
            goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
            return;
        }
        err = munmap(p, map_sz);
        if CHECK_FAIL(err != 0) {
            goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
            return;
        }
        i += 1;
    }

    /* data_map freeze should now succeed due to no R/W mapping */
    err = bpf_map_freeze(data_map_fd);
    if CHECK(err != 0, c"freeze".as_ptr() as *const u8, c"data_map freeze failed: err=%d, errno=%d\n".as_ptr() as *const u8, err, errno()) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    /* mapping as R/W now should fail */
    tmp1 = mmap(null_mut(), map_sz, PROT_READ | PROT_WRITE, MAP_SHARED, data_map_fd, 0);
    if CHECK(tmp1 != MAP_FAILED, c"data_mmap".as_ptr() as *const u8, c"mmap succeeded\n".as_ptr() as *const u8) {
        munmap(tmp1, map_sz);
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    (*bss_data).in_val = 321;
    usleep(1);
    CHECK_FAIL((*bss_data).in_val != 321);
    CHECK_FAIL((*bss_data).out_val != 321);
    CHECK_FAIL((*(*skel).bss).in_val != 321);
    CHECK_FAIL((*(*skel).bss).out_val != 321);
    CHECK_FAIL((*map_data).val[0] != 111);
    CHECK_FAIL((*map_data).val[1] != 222);
    CHECK_FAIL((*map_data).val[2] != 321);
    CHECK_FAIL((*map_data).val[far as usize] != 3 * 321);

    /* check some more advanced mmap() manipulations */

    tmp0 = mmap(null_mut(), (4 * page_size) as size_t, PROT_READ, MAP_SHARED | MAP_ANONYMOUS, -1, 0);
    if CHECK(tmp0 == MAP_FAILED, c"adv_mmap0".as_ptr() as *const u8, c"errno %d\n".as_ptr() as *const u8, errno()) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    /* map all but last page: pages 1-3 mapped */
    tmp1 = mmap(tmp0, (3 * page_size) as size_t, PROT_READ, MAP_SHARED | MAP_FIXED, data_map_fd, 0);
    if CHECK(tmp0 != tmp1, c"adv_mmap1".as_ptr() as *const u8, c"tmp0: %p, tmp1: %p\n".as_ptr() as *const u8, tmp0, tmp1) {
        munmap(tmp0, (4 * page_size) as size_t);
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    /* unmap second page: pages 1, 3 mapped */
    err = munmap(tmp1.cast::<u8>().add(page_size as usize).cast::<c_void>(), page_size as size_t);
    if CHECK(err != 0, c"adv_mmap2".as_ptr() as *const u8, c"errno %d\n".as_ptr() as *const u8, errno()) {
        munmap(tmp1, (4 * page_size) as size_t);
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    /* map page 2 back */
    tmp2 = mmap(
        tmp1.cast::<u8>().add(page_size as usize).cast::<c_void>(),
        page_size as size_t,
        PROT_READ,
        MAP_SHARED | MAP_FIXED,
        data_map_fd,
        0,
    );
    if CHECK(tmp2 == MAP_FAILED, c"adv_mmap3".as_ptr() as *const u8, c"errno %d\n".as_ptr() as *const u8, errno()) {
        munmap(tmp1, page_size as size_t);
        munmap(tmp1.cast::<u8>().add((2 * page_size) as usize).cast::<c_void>(), (2 * page_size) as size_t);
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }
    CHECK(
        tmp1.cast::<u8>().add(page_size as usize).cast::<c_void>() != tmp2,
        c"adv_mmap4".as_ptr() as *const u8,
        c"tmp1: %p, tmp2: %p\n".as_ptr() as *const u8,
        tmp1,
        tmp2,
    );

    /* re-map all 4 pages */
    tmp2 = mmap(tmp1, (4 * page_size) as size_t, PROT_READ, MAP_SHARED | MAP_FIXED, data_map_fd, 0);
    if CHECK(tmp2 == MAP_FAILED, c"adv_mmap5".as_ptr() as *const u8, c"errno %d\n".as_ptr() as *const u8, errno()) {
        munmap(tmp1, (4 * page_size) as size_t); /* unmap page 1 */
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }
    CHECK(tmp1 != tmp2, c"adv_mmap6".as_ptr() as *const u8, c"tmp1: %p, tmp2: %p\n".as_ptr() as *const u8, tmp1, tmp2);

    map_data = tmp2.cast::<map_data>();
    CHECK_FAIL((*bss_data).in_val != 321);
    CHECK_FAIL((*bss_data).out_val != 321);
    CHECK_FAIL((*(*skel).bss).in_val != 321);
    CHECK_FAIL((*(*skel).bss).out_val != 321);
    CHECK_FAIL((*map_data).val[0] != 111);
    CHECK_FAIL((*map_data).val[1] != 222);
    CHECK_FAIL((*map_data).val[2] != 321);
    CHECK_FAIL((*map_data).val[far as usize] != 3 * 321);

    munmap(tmp2, (4 * page_size) as size_t);

    /* map all 4 pages, but with pg_off=1 page, should fail */
    tmp1 = mmap(null_mut(), (4 * page_size) as size_t, PROT_READ, MAP_SHARED | MAP_FIXED, data_map_fd, page_size);
    if CHECK(tmp1 != MAP_FAILED, c"adv_mmap7".as_ptr() as *const u8, c"unexpected success".as_ptr() as *const u8) {
        munmap(tmp1, (4 * page_size) as size_t);
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    tmp1 = mmap(null_mut(), map_sz, PROT_READ, MAP_SHARED, data_map_fd, 0);
    if CHECK(tmp1 == MAP_FAILED, c"last_mmap".as_ptr() as *const u8, c"failed %d\n".as_ptr() as *const u8, errno()) {
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    test_mmap__destroy(skel);
    skel = null_mut();
    CHECK_FAIL(munmap(bss_mmaped, bss_sz) != 0);
    bss_mmaped = null_mut();
    CHECK_FAIL(munmap(map_mmaped, map_sz) != 0);
    map_mmaped = null_mut();

    /* map should be still held by active mmap */
    tmp_fd = bpf_map_get_fd_by_id(data_map_id as __u32);
    if CHECK(tmp_fd < 0, c"get_map_by_id".as_ptr() as *const u8, c"failed %d\n".as_ptr() as *const u8, errno()) {
        munmap(tmp1, map_sz);
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }
    close(tmp_fd);

    /* this should release data map finally */
    munmap(tmp1, map_sz);

    /* we need to wait for RCU grace period */
    i = 0;
    while i < 10000 {
        let mut id: __u32 = data_map_id as __u32 - 1;
        if bpf_map_get_next_id(id, &mut id) != 0 || id > data_map_id as __u32 {
            break;
        }
        usleep(1);
        i += 1;
    }

    /* should fail to get map FD by non-existing ID */
    tmp_fd = bpf_map_get_fd_by_id(data_map_id as __u32);
    if CHECK(tmp_fd >= 0, c"get_map_by_id_after".as_ptr() as *const u8, c"unexpectedly succeeded %d\n".as_ptr() as *const u8, tmp_fd) {
        close(tmp_fd);
        goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
        return;
    }

    goto_cleanup!(bss_mmaped, bss_sz, map_mmaped, map_sz, skel);
}
