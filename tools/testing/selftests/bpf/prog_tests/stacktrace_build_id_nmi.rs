// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external Rust dependencies:
// #include <test_progs.h>
// #include "test_stacktrace_build_id.skel.h"

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

extern "C" {
    static mut errno: c_int;

    fn read_perf_max_sample_freq() -> u64;
    fn test_stacktrace_build_id__open() -> *mut test_stacktrace_build_id;
    fn test_stacktrace_build_id__load(skel: *mut test_stacktrace_build_id) -> c_int;
    fn test_stacktrace_build_id__destroy(skel: *mut test_stacktrace_build_id);
    fn bpf_program__set_type(prog: *mut bpf_program, prog_type: c_int);
    fn syscall(num: c_long, ...) -> c_long;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn test__skip();
    fn bpf_program__attach_perf_event(prog: *mut bpf_program, pfd: c_int) -> *mut bpf_link;
    fn close(fd: c_int) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn compare_map_keys(map1_fd: c_int, map2_fd: c_int) -> c_int;
    fn read_build_id(path: *const c_char, buf: *mut c_char, size: usize) -> c_int;
    fn bpf_map__get_next_key(
        map: *mut bpf_map,
        cur_key: *const c_void,
        next_key: *mut c_void,
        key_sz: usize,
    ) -> c_int;
    fn bpf_map__lookup_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: usize,
        value: *mut c_void,
        value_sz: usize,
        flags: u64,
    ) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn CHECK_FAIL(condition: c_int) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut bpf_link, name: *const c_char) -> bool;
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_stacktrace_build_id_progs {
    pub oncpu: *mut bpf_program,
}

#[repr(C)]
pub struct test_stacktrace_build_id_links {
    pub oncpu: *mut bpf_link,
}

#[repr(C)]
pub struct test_stacktrace_build_id_maps {
    pub control_map: *mut bpf_map,
    pub stackid_hmap: *mut bpf_map,
    pub stackmap: *mut bpf_map,
}

#[repr(C)]
pub struct test_stacktrace_build_id {
    pub progs: test_stacktrace_build_id_progs,
    pub links: test_stacktrace_build_id_links,
    pub maps: test_stacktrace_build_id_maps,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period_or_sample_freq: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
}

impl perf_event_attr {
    unsafe fn set_freq(&mut self, freq: u64) {
        self.flags = (self.flags & !1) | (freq & 1);
    }

    unsafe fn set_sample_freq(&mut self, sample_freq: u64) {
        self.sample_period_or_sample_freq = sample_freq;
    }
}

#[repr(C)]
pub struct bpf_stack_build_id {
    pub status: i32,
    pub build_id: [u8; BPF_BUILD_ID_SIZE],
    pub offset: u64,
}

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const BPF_PROG_TYPE_PERF_EVENT: c_int = 5;
const BPF_BUILD_ID_SIZE: usize = 20;
const PERF_MAX_STACK_DEPTH: usize = 127;
const BPF_STACK_BUILD_ID_VALID: i32 = 0;
const ENOENT: c_int = 2;
const EOPNOTSUPP: c_int = 95;
const __NR_perf_event_open: c_long = 298;

#[no_mangle]
pub unsafe extern "C" fn test_stacktrace_build_id_nmi() {
    let mut control_map_fd: c_int;
    let mut stackid_hmap_fd: c_int;
    let mut stackmap_fd: c_int;
    let mut skel: *mut test_stacktrace_build_id;
    let mut err: c_int;
    let mut pmu_fd: c_int;
    let mut attr = perf_event_attr {
        sample_period_or_sample_freq: 0,
        type_: PERF_TYPE_HARDWARE,
        config: PERF_COUNT_HW_CPU_CYCLES,
        size: 0,
        sample_type: 0,
        read_format: 0,
        flags: 0,
    };
    attr.set_freq(1);
    let mut key: u32 = 0;
    let mut prev_key: u32;
    let mut val: u32;
    let mut duration: u32 = 0;
    let mut buf = [0 as c_char; BPF_BUILD_ID_SIZE];
    let mut id_offs: [bpf_stack_build_id; PERF_MAX_STACK_DEPTH] =
        core::mem::zeroed();
    let mut build_id_matches: c_int = 0;
    let mut build_id_size: c_int;
    let mut i: c_int;
    let mut retry: c_int = 1;

    attr.set_sample_freq(read_perf_max_sample_freq());

    'retry: loop {
        skel = test_stacktrace_build_id__open();
        if CHECK(
            skel.is_null(),
            b"skel_open\0".as_ptr() as *const c_char,
            b"skeleton open failed\n\0".as_ptr() as *const c_char,
        ) {
            return;
        }

        /* override program type */
        bpf_program__set_type((*skel).progs.oncpu, BPF_PROG_TYPE_PERF_EVENT);

        err = test_stacktrace_build_id__load(skel);
        if CHECK(
            err != 0,
            b"skel_load\0".as_ptr() as *const c_char,
            b"skeleton load failed: %d\n\0".as_ptr() as *const c_char,
            err,
        ) {
            break 'retry;
        }

        pmu_fd = syscall(
            __NR_perf_event_open,
            &mut attr as *mut perf_event_attr,
            -1 as c_int, /* pid */
            0 as c_int,  /* cpu 0 */
            -1 as c_int, /* group id */
            0 as c_int,  /* flags */
        ) as c_int;
        if pmu_fd < 0 && (errno == ENOENT || errno == EOPNOTSUPP) {
            printf(
                b"%s:SKIP:no PERF_COUNT_HW_CPU_CYCLES\n\0".as_ptr() as *const c_char,
                b"test_stacktrace_build_id_nmi\0".as_ptr() as *const c_char,
            );
            test__skip();
            break 'retry;
        }
        if CHECK(
            pmu_fd < 0,
            b"perf_event_open\0".as_ptr() as *const c_char,
            b"err %d errno %d\n\0".as_ptr() as *const c_char,
            pmu_fd,
            errno,
        ) {
            break 'retry;
        }

        (*skel).links.oncpu = bpf_program__attach_perf_event((*skel).progs.oncpu, pmu_fd);
        if !ASSERT_OK_PTR(
            (*skel).links.oncpu,
            b"attach_perf_event\0".as_ptr() as *const c_char,
        ) {
            close(pmu_fd);
            break 'retry;
        }

        /* find map fds */
        control_map_fd = bpf_map__fd((*skel).maps.control_map);
        stackid_hmap_fd = bpf_map__fd((*skel).maps.stackid_hmap);
        stackmap_fd = bpf_map__fd((*skel).maps.stackmap);

        if CHECK_FAIL(system(
            b"dd if=/dev/urandom of=/dev/zero count=4 2> /dev/null\0".as_ptr()
                as *const c_char,
        )) {
            break 'retry;
        }
        if CHECK_FAIL(system(
            b"taskset 0x1 ./urandom_read 100000\0".as_ptr() as *const c_char,
        )) {
            break 'retry;
        }
        /* disable stack trace collection */
        key = 0;
        val = 1;
        bpf_map_update_elem(
            control_map_fd,
            &key as *const u32 as *const c_void,
            &val as *const u32 as *const c_void,
            0,
        );

        /* for every element in stackid_hmap, we can find a corresponding one
         * in stackmap, and vice versa.
         */
        err = compare_map_keys(stackid_hmap_fd, stackmap_fd);
        if CHECK(
            err != 0,
            b"compare_map_keys stackid_hmap vs. stackmap\0".as_ptr() as *const c_char,
            b"err %d errno %d\n\0".as_ptr() as *const c_char,
            err,
            errno,
        ) {
            break 'retry;
        }

        err = compare_map_keys(stackmap_fd, stackid_hmap_fd);
        if CHECK(
            err != 0,
            b"compare_map_keys stackmap vs. stackid_hmap\0".as_ptr() as *const c_char,
            b"err %d errno %d\n\0".as_ptr() as *const c_char,
            err,
            errno,
        ) {
            break 'retry;
        }

        build_id_size = read_build_id(
            b"urandom_read\0".as_ptr() as *const c_char,
            buf.as_mut_ptr(),
            size_of_val(&buf),
        );
        err = if build_id_size < 0 { build_id_size } else { 0 };

        if CHECK(
            err != 0,
            b"get build_id with readelf\0".as_ptr() as *const c_char,
            b"err %d errno %d\n\0".as_ptr() as *const c_char,
            err,
            errno,
        ) {
            break 'retry;
        }

        err = bpf_map__get_next_key(
            (*skel).maps.stackmap,
            ptr::null(),
            &mut key as *mut u32 as *mut c_void,
            size_of::<u32>(),
        );
        if CHECK(
            err != 0,
            b"get_next_key from stackmap\0".as_ptr() as *const c_char,
            b"err %d, errno %d\n\0".as_ptr() as *const c_char,
            err,
            errno,
        ) {
            break 'retry;
        }

        loop {
            err = bpf_map__lookup_elem(
                (*skel).maps.stackmap,
                &key as *const u32 as *const c_void,
                size_of::<u32>(),
                id_offs.as_mut_ptr() as *mut c_void,
                size_of_val(&id_offs),
                0,
            );
            if CHECK(
                err != 0,
                b"lookup_elem from stackmap\0".as_ptr() as *const c_char,
                b"err %d, errno %d\n\0".as_ptr() as *const c_char,
                err,
                errno,
            ) {
                break 'retry;
            }
            i = 0;
            while i < PERF_MAX_STACK_DEPTH as c_int {
                if id_offs[i as usize].status == BPF_STACK_BUILD_ID_VALID
                    && id_offs[i as usize].offset != 0
                {
                    if memcmp(
                        buf.as_ptr() as *const c_void,
                        id_offs[i as usize].build_id.as_ptr() as *const c_void,
                        build_id_size as usize,
                    ) == 0
                    {
                        build_id_matches = 1;
                    }
                }
                i += 1;
            }
            prev_key = key;
            if bpf_map__get_next_key(
                (*skel).maps.stackmap,
                &prev_key as *const u32 as *const c_void,
                &mut key as *mut u32 as *mut c_void,
                size_of::<u32>(),
            ) != 0
            {
                break;
            }
        }

        /* stack_map_get_build_id_offset() is racy and sometimes can return
         * BPF_STACK_BUILD_ID_IP instead of BPF_STACK_BUILD_ID_VALID;
         * try it one more time.
         */
        if build_id_matches < 1 && retry != 0 {
            retry -= 1;
            test_stacktrace_build_id__destroy(skel);
            printf(
                b"%s:WARN:Didn't find expected build ID from the map, retrying\n\0".as_ptr()
                    as *const c_char,
                b"test_stacktrace_build_id_nmi\0".as_ptr() as *const c_char,
            );
            continue 'retry;
        }

        if CHECK(
            build_id_matches < 1,
            b"build id match\0".as_ptr() as *const c_char,
            b"Didn't find expected build ID from the map\n\0".as_ptr() as *const c_char,
        ) {
            break 'retry;
        }

        /*
         * We intentionally skip compare_stack_ips(). This is because we
         * only support one in_nmi() ips-to-build_id translation per cpu
         * at any time, thus stack_amap here will always fallback to
         * BPF_STACK_BUILD_ID_IP;
         */

        break 'retry;
    }

    test_stacktrace_build_id__destroy(skel);
}

unsafe fn size_of_val<T: ?Sized>(val: &T) -> usize {
    core::mem::size_of_val(val)
}
