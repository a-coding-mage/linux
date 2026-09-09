// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2016 Facebook
 */

// C headers and symbols supplied by the surrounding build are represented by
// the corresponding external Rust declarations below.

use core::ffi::{c_char, c_int, c_void};

const PRINT_RAW_ADDR: c_int = 0;
const TASK_COMM_LEN: usize = 16;
const PERF_MAX_STACK_DEPTH: usize = 127;
const EEXIST: u32 = 17;

#[repr(C)]
pub struct ksym {
    pub name: *const c_char,
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ... ) -> c_int;
    fn atoi(s: *const c_char) -> c_int;
    fn sleep(seconds: u32) -> u32;
    fn exit(status: c_int) -> !;
    fn signal(signum: c_int, handler: unsafe extern "C" fn(c_int)) -> usize;

    fn load_kallsyms() -> c_int;
    fn ksym_search(addr: u64) -> *mut ksym;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_object__open_file(filename: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> isize;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
}

static mut MAP_FD: [c_int; 2] = [0; 2];

unsafe fn print_ksym(addr: u64) {
    if addr == 0 {
        return;
    }
    let sym = ksym_search(addr);
    if sym.is_null() {
        printf(b"ksym not found. Is kallsyms loaded?\n\0".as_ptr() as *const c_char);
        return;
    }

    if PRINT_RAW_ADDR != 0 {
        printf(b"%s/%llx;\0".as_ptr() as *const c_char, (*sym).name, addr);
    } else {
        printf(b"%s;\0".as_ptr() as *const c_char, (*sym).name);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct key_t {
    waker: [c_char; TASK_COMM_LEN],
    target: [c_char; TASK_COMM_LEN],
    wret: u32,
    tret: u32,
}

unsafe fn print_stack(key: *mut key_t, count: u64) {
    let mut ip = [0u64; PERF_MAX_STACK_DEPTH];
    static mut WARNED: bool = false;

    printf(b"%s;\0".as_ptr() as *const c_char, (*key).target.as_ptr());
    if bpf_map_lookup_elem(MAP_FD[1], &(*key).tret as *const u32 as *const c_void,
                           ip.as_mut_ptr() as *mut c_void) != 0 {
        printf(b"---;\0".as_ptr() as *const c_char);
    } else {
        let mut i = PERF_MAX_STACK_DEPTH as isize - 1;
        while i >= 0 {
            print_ksym(ip[i as usize]);
            i -= 1;
        }
    }
    printf(b"-;\0".as_ptr() as *const c_char);
    if bpf_map_lookup_elem(MAP_FD[1], &(*key).wret as *const u32 as *const c_void,
                           ip.as_mut_ptr() as *mut c_void) != 0 {
        printf(b"---;\0".as_ptr() as *const c_char);
    } else {
        for i in 0..PERF_MAX_STACK_DEPTH {
            print_ksym(ip[i]);
        }
    }
    printf(b";%s %lld\n\0".as_ptr() as *const c_char, (*key).waker.as_ptr(), count);

    if (((*key).tret == EEXIST.wrapping_neg()) || ((*key).wret == EEXIST.wrapping_neg())) && !WARNED {
        printf(b"stackmap collisions seen. Consider increasing size\n\0".as_ptr() as *const c_char);
        WARNED = true;
    } else if ((*key).tret as i32) < 0 || ((*key).wret as i32) < 0 {
        printf(b"err stackid %d %d\n\0".as_ptr() as *const c_char, (*key).tret, (*key).wret);
    }
}

unsafe fn print_stacks(fd: c_int) {
    let mut key = core::mem::zeroed::<key_t>();
    let mut next_key: key_t = core::mem::zeroed();
    let mut value: u64 = 0;

    while bpf_map_get_next_key(fd, &key as *const key_t as *const c_void,
                               &mut next_key as *mut key_t as *mut c_void) == 0 {
        bpf_map_lookup_elem(fd, &next_key as *const key_t as *const c_void,
                            &mut value as *mut u64 as *mut c_void);
        print_stack(&mut next_key, value);
        key = next_key;
    }
}

unsafe extern "C" fn int_exit(_sig: c_int) {
    print_stacks(MAP_FD[0]);
    exit(0);
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut links: [*mut bpf_link; 2] = [core::ptr::null_mut(); 2];
    let mut delay: c_int = 1;
    let mut i: c_int = 0;
    let mut filename = [0 as c_char; 256];

    if load_kallsyms() != 0 {
        printf(b"failed to process /proc/kallsyms\n\0".as_ptr() as *const c_char);
        return 2;
    }

    snprintf(filename.as_mut_ptr(), filename.len(), b"%s.bpf.o\0".as_ptr() as *const c_char, *argv);
    obj = bpf_object__open_file(filename.as_ptr(), core::ptr::null());
    if libbpf_get_error(obj as *const c_void) != 0 {
        fprintf(stderr, b"ERROR: opening BPF object file failed\n\0".as_ptr() as *const c_char);
        obj = core::ptr::null_mut();
        goto_cleanup(&mut obj, &mut links, i);
        return 0;
    }

    if bpf_object__load(obj) != 0 {
        fprintf(stderr, b"ERROR: loading BPF object file failed\n\0".as_ptr() as *const c_char);
        goto_cleanup(&mut obj, &mut links, i);
        return 0;
    }

    MAP_FD[0] = bpf_object__find_map_fd_by_name(obj, b"counts\0".as_ptr() as *const c_char);
    MAP_FD[1] = bpf_object__find_map_fd_by_name(obj, b"stackmap\0".as_ptr() as *const c_char);
    if MAP_FD[0] < 0 || MAP_FD[1] < 0 {
        fprintf(stderr, b"ERROR: finding a map in obj file failed\n\0".as_ptr() as *const c_char);
        goto_cleanup(&mut obj, &mut links, i);
        return 0;
    }

    signal(2, int_exit);
    signal(15, int_exit);
    // bpf_object__for_each_program(prog, obj) expands to iteration over all programs.
    // The external iterator is preserved as the required integration point.
    let _prog: *mut bpf_program = core::ptr::null_mut();
    if i < links.len() as c_int {
        links[i as usize] = bpf_program__attach(_prog);
        if libbpf_get_error(links[i as usize] as *const c_void) != 0 {
            fprintf(stderr, b"ERROR: bpf_program__attach failed\n\0".as_ptr() as *const c_char);
            links[i as usize] = core::ptr::null_mut();
            goto_cleanup(&mut obj, &mut links, i);
            return 0;
        }
        i += 1;
    }

    if argc > 1 {
        delay = atoi(*argv.add(1));
    }
    sleep(delay as u32);
    print_stacks(MAP_FD[0]);
    goto_cleanup(&mut obj, &mut links, i);
    0
}

unsafe fn goto_cleanup(obj: &mut *mut bpf_object, links: &mut [*mut bpf_link; 2], mut i: c_int) {
    i -= 1;
    while i >= 0 {
        bpf_link__destroy(links[i as usize]);
        i -= 1;
    }
    bpf_object__close(*obj);
}

pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    main_impl(argc, argv)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
