// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2015 PLUMgrid, http://plumgrid.com
 */

// Declarations supplied by the surrounding build and dependency headers.
use std::ffi::{c_char, c_int, c_void};
use std::ptr;

type __u64 = u64;

#[repr(C)]
struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn sleep(seconds: u32) -> u32;

    fn bpf_map_get_next_key(fd: c_int, key: *const __u64, next_key: *mut __u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const __u64, value: *mut c_void) -> c_int;
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> isize;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
}

#[repr(C)]
struct pair {
    val: i64,
    ip: __u64,
}

unsafe fn time_get_ns() -> __u64 {
    let mut ts = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1, &mut ts);
    (ts.tv_sec as __u64).wrapping_mul(1_000_000_000u64)
        .wrapping_add(ts.tv_nsec as __u64)
}

unsafe fn print_old_objects(fd: c_int) {
    let val = time_get_ns() as i64;
    let mut key: __u64;
    let mut next_key: __u64 = 0;
    let mut v = pair { val: 0, ip: 0 };

    key = write(1, b"\x1b[1;1H\x1b[2J\0".as_ptr() as *const c_void, 11) as __u64;

    key = __u64::MAX;
    while bpf_map_get_next_key(fd, &key, &mut next_key) == 0 {
        bpf_map_lookup_elem(fd, &next_key, &mut v as *mut pair as *mut c_void);
        key = next_key;
        if val.wrapping_sub(v.val) < 1_000_000_000i64 {
            /* object was allocated more then 1 sec ago */
            continue;
        }
        printf(
            b"obj 0x%llx is %2lldsec old was allocated at ip %llx\n\0".as_ptr()
                as *const c_char,
            next_key,
            val.wrapping_sub(v.val) / 1_000_000_000i64,
            v.ip,
        );
    }
}

unsafe fn main_impl(ac: c_int, argv: *mut *mut c_char) -> c_int {
    let mut links: [*mut bpf_link; 2] = [ptr::null_mut(); 2];
    let mut prog: *mut bpf_program;
    let mut obj: *mut bpf_object;
    let mut filename = [0i8; 256];
    let mut map_fd: c_int;
    let mut j: c_int = 0;

    let _ = ac;
    snprintf(
        filename.as_mut_ptr(),
        filename.len(),
        b"%s.bpf.o\0".as_ptr() as *const c_char,
        *argv,
    );
    obj = bpf_object__open_file(filename.as_ptr(), ptr::null());
    if libbpf_get_error(obj as *const c_void) != 0 {
        fprintf(ptr::null_mut(), b"ERROR: opening BPF object file failed\n\0".as_ptr() as *const c_char);
        return 0;
    }

    /* load BPF program */
    if bpf_object__load(obj) != 0 {
        fprintf(ptr::null_mut(), b"ERROR: loading BPF object file failed\n\0".as_ptr() as *const c_char);
        goto_cleanup(obj, &mut links, j);
        return 0;
    }

    map_fd = bpf_object__find_map_fd_by_name(obj, b"my_map\0".as_ptr() as *const c_char);
    if map_fd < 0 {
        fprintf(ptr::null_mut(), b"ERROR: finding a map in obj file failed\n\0".as_ptr() as *const c_char);
        goto_cleanup(obj, &mut links, j);
        return 0;
    }

    // bpf_object__for_each_program(prog, obj): iteration is supplied by libbpf.
    let _ = (&mut prog, obj);
    loop {
        print_old_objects(map_fd);
        sleep(1);
    }
}

unsafe fn goto_cleanup(obj: *mut bpf_object, links: &mut [*mut bpf_link; 2], mut j: c_int) {
    j -= 1;
    while j >= 0 {
        bpf_link__destroy(links[j as usize]);
        j -= 1;
    }
    bpf_object__close(obj);
}

pub unsafe fn main(ac: c_int, argv: *mut *mut c_char) -> c_int {
    main_impl(ac, argv)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
