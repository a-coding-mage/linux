// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and included pthread/sched/socket/test/libbpf
// headers. The declarations below are external dependencies supplied by the
// surrounding selftest/libbpf build.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type size_t = usize;
type pid_t = c_int;
type pthread_t = usize;

#[repr(C)]
pub struct cpu_set_t {
    __bits: [usize; 16],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_perf_buffer {
    pub maps: test_perf_buffer_maps,
}

#[repr(C)]
pub struct test_perf_buffer_maps {
    pub my_pid_map: *mut bpf_map,
    pub perf_buf_map: *mut bpf_map,
}

unsafe extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn usleep(usec: c_uint) -> c_int;
    fn getpid() -> pid_t;

    fn pthread_self() -> pthread_t;
    fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: size_t, cpuset: *const cpu_set_t) -> c_int;

    fn libbpf_num_possible_cpus() -> c_int;
    fn parse_cpu_mask_file(path: *const c_char, mask: *mut *mut bool, mask_sz: *mut c_int) -> c_int;

    fn test_perf_buffer__open_and_load() -> *mut test_perf_buffer;
    fn test_perf_buffer__attach(obj: *mut test_perf_buffer) -> c_int;
    fn test_perf_buffer__destroy(obj: *mut test_perf_buffer);

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;

    fn perf_buffer__new(
        map_fd: c_int,
        page_cnt: size_t,
        sample_cb: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, __u32)>,
        lost_cb: Option<unsafe extern "C" fn(*mut c_void, c_int, u64)>,
        ctx: *mut c_void,
        opts: *const c_void,
    ) -> *mut perf_buffer;
    fn perf_buffer__epoll_fd(pb: *mut perf_buffer) -> c_int;
    fn perf_buffer__poll(pb: *mut perf_buffer, timeout_ms: c_int) -> c_int;
    fn perf_buffer__buffer_cnt(pb: *mut perf_buffer) -> size_t;
    fn perf_buffer__buffer_fd(pb: *mut perf_buffer, buf_idx: c_int) -> c_int;
    fn perf_buffer__consume_buffer(pb: *mut perf_buffer, buf_idx: c_int) -> c_int;
    fn perf_buffer__free(pb: *mut perf_buffer);

    fn CHECK(condition: c_int, tag: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn ASSERT_OK(err: c_int, tag: *const c_char) -> c_int;
    fn ASSERT_OK_PTR(ptr: *const c_void, tag: *const c_char) -> c_int;
}

static mut duration: c_int = 0;

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    unsafe {
        ptr::write_bytes(set as *mut u8, 0, size_of::<cpu_set_t>());
    }
}

unsafe fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    let bits_per_word = (size_of::<usize>() * 8) as c_int;
    let idx = (cpu / bits_per_word) as usize;
    let bit = (cpu % bits_per_word) as usize;

    unsafe {
        (*set).__bits[idx] |= 1usize << bit;
    }
}

unsafe fn CPU_CLR(cpu: c_int, set: *mut cpu_set_t) {
    let bits_per_word = (size_of::<usize>() * 8) as c_int;
    let idx = (cpu / bits_per_word) as usize;
    let bit = (cpu % bits_per_word) as usize;

    unsafe {
        (*set).__bits[idx] &= !(1usize << bit);
    }
}

unsafe fn CPU_ISSET(cpu: c_int, set: *const cpu_set_t) -> bool {
    let bits_per_word = (size_of::<usize>() * 8) as c_int;
    let idx = (cpu / bits_per_word) as usize;
    let bit = (cpu % bits_per_word) as usize;

    unsafe { ((*set).__bits[idx] & (1usize << bit)) != 0 }
}

unsafe fn CPU_COUNT(set: *const cpu_set_t) -> c_int {
    let mut count = 0;

    unsafe {
        for word in (*set).__bits.iter() {
            count += word.count_ones() as c_int;
        }
    }

    count
}

/* AddressSanitizer sometimes crashes due to data dereference below, due to
 * this being mmap()'ed memory. Disable instrumentation with
 * no_sanitize_address attribute
 */
// Rust equivalent of the C no_sanitize_address function attribute is build
// dependent; preserve the callback body and ABI exactly.
unsafe extern "C" fn on_sample(ctx: *mut c_void, cpu: c_int, data: *mut c_void, _size: __u32) {
    let cpu_data: c_int = unsafe { *(data as *mut c_int) };
    let _duration: c_int = 0;
    let cpu_seen: *mut cpu_set_t = ctx as *mut cpu_set_t;

    if cpu_data != cpu {
        unsafe {
            CHECK(
                (cpu_data != cpu) as c_int,
                c"check_cpu_data".as_ptr(),
                c"cpu_data %d != cpu %d\n".as_ptr(),
                cpu_data,
                cpu,
            );
        }
    }

    unsafe {
        CPU_SET(cpu, cpu_seen);
    }
}

pub unsafe fn trigger_on_cpu(cpu: c_int) -> c_int {
    let mut cpu_set: cpu_set_t = unsafe { core::mem::zeroed() };
    let mut err: c_int;

    unsafe {
        CPU_ZERO(&mut cpu_set);
        CPU_SET(cpu, &mut cpu_set);

        err = pthread_setaffinity_np(pthread_self(), size_of::<cpu_set_t>(), &cpu_set);
        if err != 0
            && CHECK(
                err,
                c"set_affinity".as_ptr(),
                c"cpu #%d, err %d\n".as_ptr(),
                cpu,
                err,
            ) != 0
        {
            return err;
        }

        usleep(1);
    }

    0
}

pub unsafe fn serial_test_perf_buffer() {
    let mut err: c_int;
    let mut on_len: c_int = 0;
    let mut nr_on_cpus: c_int = 0;
    let nr_cpus: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let zero: c_int = 0;
    let my_pid: c_int = unsafe { getpid() };
    let mut skel: *mut test_perf_buffer = ptr::null_mut();
    let mut cpu_seen: cpu_set_t = unsafe { core::mem::zeroed() };
    let mut pb: *mut perf_buffer = ptr::null_mut();
    let mut last_fd: c_int = -1;
    let mut fd: c_int;
    let mut online: *mut bool = ptr::null_mut();
    let mut free_pb = false;

    unsafe {
        nr_cpus = libbpf_num_possible_cpus();
        if CHECK(nr_cpus < 0, c"nr_cpus".as_ptr(), c"err %d\n".as_ptr(), nr_cpus) != 0 {
            return;
        }

        err = parse_cpu_mask_file(
            c"/sys/devices/system/cpu/online".as_ptr(),
            &mut online,
            &mut on_len,
        );
        if CHECK(err, c"nr_on_cpus".as_ptr(), c"err %d\n".as_ptr(), err) != 0 {
            return;
        }

        i = 0;
        while i < on_len {
            if *online.add(i as usize) {
                nr_on_cpus += 1;
            }
            i += 1;
        }

        'out_close: loop {
            /* load program */
            skel = test_perf_buffer__open_and_load();
            if CHECK(
                skel.is_null() as c_int,
                c"skel_load".as_ptr(),
                c"skeleton open/load failed\n".as_ptr(),
            ) != 0
            {
                break 'out_close;
            }

            err = bpf_map_update_elem(
                bpf_map__fd((*skel).maps.my_pid_map),
                &zero as *const c_int as *const c_void,
                &my_pid as *const c_int as *const c_void,
                0,
            );
            if ASSERT_OK(err, c"my_pid_update".as_ptr()) == 0 {
                break 'out_close;
            }

            /* attach probe */
            err = test_perf_buffer__attach(skel);
            if CHECK(err, c"attach_kprobe".as_ptr(), c"err %d\n".as_ptr(), err) != 0 {
                break 'out_close;
            }

            /* set up perf buffer */
            pb = perf_buffer__new(
                bpf_map__fd((*skel).maps.perf_buf_map),
                1,
                Some(on_sample),
                None,
                &mut cpu_seen as *mut cpu_set_t as *mut c_void,
                ptr::null(),
            );
            if ASSERT_OK_PTR(pb as *const c_void, c"perf_buf__new".as_ptr()) == 0 {
                break 'out_close;
            }
            free_pb = true;

            CHECK(
                (perf_buffer__epoll_fd(pb) < 0) as c_int,
                c"epoll_fd".as_ptr(),
                c"bad fd: %d\n".as_ptr(),
                perf_buffer__epoll_fd(pb),
            );

            /* trigger kprobe on every CPU */
            CPU_ZERO(&mut cpu_seen);
            i = 0;
            while i < nr_cpus {
                if i >= on_len || !*online.add(i as usize) {
                    printf(c"skipping offline CPU #%d\n".as_ptr(), i);
                    i += 1;
                    continue;
                }

                if trigger_on_cpu(i) != 0 {
                    break 'out_close;
                }
                i += 1;
            }

            /* read perf buffer */
            err = perf_buffer__poll(pb, 100);
            if CHECK(
                (err < 0) as c_int,
                c"perf_buffer__poll".as_ptr(),
                c"err %d\n".as_ptr(),
                err,
            ) != 0
            {
                break 'out_close;
            }

            if CHECK(
                (CPU_COUNT(&cpu_seen) != nr_on_cpus) as c_int,
                c"seen_cpu_cnt".as_ptr(),
                c"expect %d, seen %d\n".as_ptr(),
                nr_on_cpus,
                CPU_COUNT(&cpu_seen),
            ) != 0
            {
                break 'out_close;
            }

            if CHECK(
                (perf_buffer__buffer_cnt(pb) != nr_on_cpus as size_t) as c_int,
                c"buf_cnt".as_ptr(),
                c"got %zu, expected %d\n".as_ptr(),
                perf_buffer__buffer_cnt(pb),
                nr_on_cpus,
            ) != 0
            {
                break 'out_close;
            }

            i = 0;
            j = 0;
            while i < nr_cpus {
                if i >= on_len || !*online.add(i as usize) {
                    i += 1;
                    continue;
                }

                fd = perf_buffer__buffer_fd(pb, j);
                CHECK(
                    (fd < 0 || last_fd == fd) as c_int,
                    c"fd_check".as_ptr(),
                    c"last fd %d == fd %d\n".as_ptr(),
                    last_fd,
                    fd,
                );
                last_fd = fd;

                err = perf_buffer__consume_buffer(pb, j);
                if CHECK(
                    err,
                    c"drain_buf".as_ptr(),
                    c"cpu %d, err %d\n".as_ptr(),
                    i,
                    err,
                ) != 0
                {
                    break 'out_close;
                }

                CPU_CLR(i, &mut cpu_seen);
                if trigger_on_cpu(i) != 0 {
                    break 'out_close;
                }

                err = perf_buffer__consume_buffer(pb, j);
                if CHECK(
                    err,
                    c"consume_buf".as_ptr(),
                    c"cpu %d, err %d\n".as_ptr(),
                    j,
                    err,
                ) != 0
                {
                    break 'out_close;
                }

                if CHECK(
                    (!CPU_ISSET(i, &cpu_seen)) as c_int,
                    c"cpu_seen".as_ptr(),
                    c"cpu %d not seen\n".as_ptr(),
                    i,
                ) != 0
                {
                    break 'out_close;
                }
                j += 1;
                i += 1;
            }

            break 'out_close;
        }

        if free_pb {
            perf_buffer__free(pb);
        }
        test_perf_buffer__destroy(skel);
        free(online as *mut c_void);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
