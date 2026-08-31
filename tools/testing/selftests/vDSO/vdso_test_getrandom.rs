// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022-2024 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

/* Translated from C. External declarations correspond to system/libc headers and
 * local kselftest/vDSO helper headers included by the original source.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(static_mut_refs)]
#![no_main]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type pthread_t = c_ulong;
type pthread_mutex_t = [usize; 5];

const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = [0; 5];
const CLOCK_MONOTONIC: c_int = 1;
const AT_SYSINFO_EHDR: c_ulong = 33;
const _SC_NPROCESSORS_ONLN: c_int = 84;
const _SC_LEVEL1_DCACHE_LINESIZE: c_int = 190;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const ENOSYS: ssize_t = 38;
const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWTIME: c_int = 0x00000080;
const SIGSTOP: c_int = 19;
const SIGTRAP: c_int = 5;
const PTRACE_TRACEME: c_int = 0;
const PTRACE_SYSCALL: c_int = 24;
const PTRACE_SETOPTIONS: c_int = 0x4200;
const PTRACE_GET_SYSCALL_INFO: c_int = 0x420e;
const PTRACE_O_TRACESYSGOOD: c_int = 0x00000001;
const PTRACE_SYSCALL_INFO_ENTRY: u8 = 1;
const __NR_getrandom: c_long = 318;

const VDSO_VERSION: usize = 0;
const VDSO_NAMES: usize = 0;

const TRIALS: size_t = 25000000;
const THREADS: size_t = 256;

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct vgetrandom_opaque_params {
    size_of_opaque_state: size_t,
    mmap_prot: c_int,
    mmap_flags: c_int,
}

type vgetrandom_fn = unsafe extern "C" fn(
    *mut c_void,
    size_t,
    c_ulong,
    *mut c_void,
    size_t,
) -> ssize_t;

#[repr(C)]
struct Vgrnd {
    lock: pthread_mutex_t,
    states: *mut *mut c_void,
    len: size_t,
    cap: size_t,
    fn_: Option<vgetrandom_fn>,
    params: vgetrandom_opaque_params,
}

#[repr(C)]
union ptrace_syscall_info_data {
    entry: ptrace_syscall_info_entry,
    _bindgen_union_align: [u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ptrace_syscall_info_entry {
    nr: u64,
    args: [u64; 6],
}

#[repr(C)]
struct ptrace_syscall_info {
    op: u8,
    pad: [u8; 3],
    arch: u32,
    instruction_pointer: u64,
    stack_pointer: u64,
    data: ptrace_syscall_info_data,
}

impl ptrace_syscall_info {
    unsafe fn entry(&self) -> ptrace_syscall_info_entry {
        self.data.entry
    }
}

unsafe extern "C" {
    static versions: *const *const c_char;
    static names: *const *const *const c_char;

    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn getpagesize() -> c_int;
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
    fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn vdso_init_from_sysinfo_ehdr(sysinfo_ehdr: c_ulong);
    fn vdso_sym(version: *const c_char, name: *const c_char) -> *mut c_void;
    fn ksft_exit_skip(fmt: *const c_char, ...);
    fn ksft_exit_fail_msg(fmt: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_exit_pass();
    fn getrandom(buf: *mut c_void, buflen: size_t, flags: c_uint) -> ssize_t;
    fn syscall(num: c_long, ...) -> c_long;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn printf(fmt: *const c_char, ...);
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...);
    static mut stderr: *mut c_void;
    fn unshare(flags: c_int) -> c_int;
    fn fork() -> pid_t;
    fn getpid() -> pid_t;
    fn ptrace(request: c_int, ...) -> c_long;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn _exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

type c_uint = u32;

static mut vgrnd: Vgrnd = Vgrnd {
    lock: PTHREAD_MUTEX_INITIALIZER,
    states: ptr::null_mut(),
    len: 0,
    cap: 0,
    fn_: None,
    params: vgetrandom_opaque_params {
        size_of_opaque_state: 0,
        mmap_prot: 0,
        mmap_flags: 0,
    },
};

thread_local! {
    static state: core::cell::Cell<*mut c_void> = core::cell::Cell::new(ptr::null_mut());
}

unsafe fn timespecsub(tsp: *const timespec, usp: *const timespec, vsp: *mut timespec) {
    (*vsp).tv_sec = (*tsp).tv_sec - (*usp).tv_sec;
    (*vsp).tv_nsec = (*tsp).tv_nsec - (*usp).tv_nsec;
    if (*vsp).tv_nsec < 0 {
        (*vsp).tv_sec -= 1;
        (*vsp).tv_nsec += 1000000000L;
    }
}

unsafe fn ksft_assert(condition: bool, condition_str: *const c_char) {
    if !condition {
        ksft_exit_fail_msg(c"Assertion failed: %s\n".as_ptr(), condition_str);
    }
}

unsafe fn VDSO_CALL(
    fn_: Option<vgetrandom_fn>,
    _nr: c_int,
    arg0: *mut c_void,
    arg1: size_t,
    arg2: c_ulong,
    arg3: *mut c_void,
    arg4: size_t,
) -> ssize_t {
    fn_.unwrap()(arg0, arg1, arg2, arg3, arg4)
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

unsafe fn WSTOPSIG(status: c_int) -> c_int {
    WEXITSTATUS(status)
}

unsafe fn vgetrandom_get_state() -> *mut c_void {
    let mut state: *mut c_void = ptr::null_mut();

    pthread_mutex_lock(&mut vgrnd.lock);
    if vgrnd.len == 0 {
        let page_size: size_t = getpagesize() as size_t;
        let new_cap: size_t;
        let alloc_size: size_t;
        let mut num: size_t = sysconf(_SC_NPROCESSORS_ONLN) as size_t; /* Just a decent heuristic. */
        let state_size_aligned: size_t;
        let cache_line_sysconf = sysconf(_SC_LEVEL1_DCACHE_LINESIZE);
        let cache_line_size: size_t = if cache_line_sysconf != 0 {
            cache_line_sysconf as size_t
        } else {
            1
        };
        let mut new_block: *mut c_void;
        let new_states: *mut c_void;

        state_size_aligned =
            (vgrnd.params.size_of_opaque_state + cache_line_size - 1) & !(cache_line_size - 1);
        alloc_size = (num * state_size_aligned + page_size - 1) & !(page_size - 1);
        num = (page_size / state_size_aligned) * (alloc_size / page_size);
        new_block = mmap(
            ptr::null_mut(),
            alloc_size,
            vgrnd.params.mmap_prot,
            vgrnd.params.mmap_flags,
            -1,
            0,
        );
        if new_block == MAP_FAILED {
            goto_out();
            pthread_mutex_unlock(&mut vgrnd.lock);
            return state;
        }

        new_cap = vgrnd.cap + num;
        new_states = reallocarray(
            vgrnd.states as *mut c_void,
            new_cap,
            mem::size_of::<*mut c_void>(),
        );
        if new_states.is_null() {
            munmap(new_block, alloc_size);
            pthread_mutex_unlock(&mut vgrnd.lock);
            return state;
        }
        vgrnd.cap = new_cap;
        vgrnd.states = new_states as *mut *mut c_void;

        for i in 0..num {
            if ((new_block as usize) & (page_size - 1)) + vgrnd.params.size_of_opaque_state
                > page_size
            {
                new_block =
                    (((new_block as usize) + page_size - 1) & !(page_size - 1)) as *mut c_void;
            }
            *vgrnd.states.add(i) = new_block;
            new_block = (new_block as *mut u8).add(state_size_aligned) as *mut c_void;
        }
        vgrnd.len = num;
    }

    vgrnd.len -= 1;
    state = *vgrnd.states.add(vgrnd.len);

    pthread_mutex_unlock(&mut vgrnd.lock);
    state
}

unsafe fn goto_out() {}

#[allow(unused)]
/* Example for libc implementors */
unsafe fn vgetrandom_put_state(state: *mut c_void) {
    if state.is_null() {
        return;
    }
    pthread_mutex_lock(&mut vgrnd.lock);
    *vgrnd.states.add(vgrnd.len) = state;
    vgrnd.len += 1;
    pthread_mutex_unlock(&mut vgrnd.lock);
}

unsafe fn vgetrandom_init() {
    let version: *const c_char = *versions.add(VDSO_VERSION);
    let name: *const c_char = *(*names.add(VDSO_NAMES)).add(6);
    let sysinfo_ehdr: c_ulong = getauxval(AT_SYSINFO_EHDR);
    let ret: ssize_t;

    if sysinfo_ehdr == 0 {
        ksft_exit_skip(c"AT_SYSINFO_EHDR is not present\n".as_ptr());
    }
    vdso_init_from_sysinfo_ehdr(sysinfo_ehdr);
    vgrnd.fn_ = mem::transmute::<*mut c_void, Option<vgetrandom_fn>>(vdso_sym(version, name));
    if vgrnd.fn_.is_none() {
        ksft_exit_skip(c"%s@%s symbol is missing from vDSO\n".as_ptr(), name, version);
    }
    ret = VDSO_CALL(
        vgrnd.fn_,
        5,
        ptr::null_mut(),
        0,
        0,
        &mut vgrnd.params as *mut _ as *mut c_void,
        !0 as c_ulong,
    );
    if ret == -ENOSYS {
        ksft_exit_skip(c"CPU does not have runtime support\n".as_ptr());
    } else if ret != 0 {
        ksft_exit_fail_msg(c"Failed to fetch vgetrandom params: %zd\n".as_ptr(), ret);
    }
}

unsafe fn vgetrandom(buf: *mut c_void, len: size_t, flags: c_ulong) -> ssize_t {
    let mut local_state = state.with(|s| s.get());

    if local_state.is_null() {
        local_state = vgetrandom_get_state();
        ksft_assert(!local_state.is_null(), c"state".as_ptr());
        state.with(|s| s.set(local_state));
    }
    VDSO_CALL(
        vgrnd.fn_,
        5,
        buf,
        len,
        flags,
        local_state,
        vgrnd.params.size_of_opaque_state,
    )
}

unsafe extern "C" fn test_vdso_getrandom(_ctx: *mut c_void) -> *mut c_void {
    for _i in 0..TRIALS {
        let mut val: c_uint = 0;
        let ret: ssize_t = vgetrandom(
            &mut val as *mut _ as *mut c_void,
            mem::size_of_val(&val),
            0,
        );
        ksft_assert(ret == mem::size_of_val(&val) as ssize_t, c"ret == sizeof(val)".as_ptr());
    }
    ptr::null_mut()
}

unsafe extern "C" fn test_libc_getrandom(_ctx: *mut c_void) -> *mut c_void {
    for _i in 0..TRIALS {
        let mut val: c_uint = 0;
        let ret: ssize_t = getrandom(&mut val as *mut _ as *mut c_void, mem::size_of_val(&val), 0);
        ksft_assert(ret == mem::size_of_val(&val) as ssize_t, c"ret == sizeof(val)".as_ptr());
    }
    ptr::null_mut()
}

unsafe extern "C" fn test_syscall_getrandom(_ctx: *mut c_void) -> *mut c_void {
    for _i in 0..TRIALS {
        let mut val: c_uint = 0;
        let ret: ssize_t = syscall(
            __NR_getrandom,
            &mut val as *mut _ as *mut c_void,
            mem::size_of_val(&val),
            0,
        ) as ssize_t;
        ksft_assert(ret == mem::size_of_val(&val) as ssize_t, c"ret == sizeof(val)".as_ptr());
    }
    ptr::null_mut()
}

unsafe fn bench_single() {
    let mut start: timespec = mem::zeroed();
    let mut end: timespec = mem::zeroed();
    let mut diff: timespec = mem::zeroed();

    clock_gettime(CLOCK_MONOTONIC, &mut start);
    test_vdso_getrandom(ptr::null_mut());
    clock_gettime(CLOCK_MONOTONIC, &mut end);
    timespecsub(&end, &start, &mut diff);
    printf(c"   vdso: %u times in %lu.%09lu seconds\n".as_ptr(), TRIALS as c_uint, diff.tv_sec as c_ulong, diff.tv_nsec as c_ulong);

    clock_gettime(CLOCK_MONOTONIC, &mut start);
    test_libc_getrandom(ptr::null_mut());
    clock_gettime(CLOCK_MONOTONIC, &mut end);
    timespecsub(&end, &start, &mut diff);
    printf(c"   libc: %u times in %lu.%09lu seconds\n".as_ptr(), TRIALS as c_uint, diff.tv_sec as c_ulong, diff.tv_nsec as c_ulong);

    clock_gettime(CLOCK_MONOTONIC, &mut start);
    test_syscall_getrandom(ptr::null_mut());
    clock_gettime(CLOCK_MONOTONIC, &mut end);
    timespecsub(&end, &start, &mut diff);
    printf(c"syscall: %u times in %lu.%09lu seconds\n".as_ptr(), TRIALS as c_uint, diff.tv_sec as c_ulong, diff.tv_nsec as c_ulong);
}

unsafe fn bench_multi() {
    let mut start: timespec = mem::zeroed();
    let mut end: timespec = mem::zeroed();
    let mut diff: timespec = mem::zeroed();
    let mut threads: [pthread_t; THREADS] = [0; THREADS];

    clock_gettime(CLOCK_MONOTONIC, &mut start);
    for i in 0..THREADS {
        ksft_assert(
            pthread_create(&mut threads[i], ptr::null(), test_vdso_getrandom, ptr::null_mut()) == 0,
            c"pthread_create(&threads[i], NULL, test_vdso_getrandom, NULL) == 0".as_ptr(),
        );
    }
    for i in 0..THREADS {
        pthread_join(threads[i], ptr::null_mut());
    }
    clock_gettime(CLOCK_MONOTONIC, &mut end);
    timespecsub(&end, &start, &mut diff);
    printf(c"   vdso: %u x %u times in %lu.%09lu seconds\n".as_ptr(), TRIALS as c_uint, THREADS as c_uint, diff.tv_sec as c_ulong, diff.tv_nsec as c_ulong);

    clock_gettime(CLOCK_MONOTONIC, &mut start);
    for i in 0..THREADS {
        ksft_assert(
            pthread_create(&mut threads[i], ptr::null(), test_libc_getrandom, ptr::null_mut()) == 0,
            c"pthread_create(&threads[i], NULL, test_libc_getrandom, NULL) == 0".as_ptr(),
        );
    }
    for i in 0..THREADS {
        pthread_join(threads[i], ptr::null_mut());
    }
    clock_gettime(CLOCK_MONOTONIC, &mut end);
    timespecsub(&end, &start, &mut diff);
    printf(c"   libc: %u x %u times in %lu.%09lu seconds\n".as_ptr(), TRIALS as c_uint, THREADS as c_uint, diff.tv_sec as c_ulong, diff.tv_nsec as c_ulong);

    clock_gettime(CLOCK_MONOTONIC, &mut start);
    for i in 0..THREADS {
        ksft_assert(
            pthread_create(&mut threads[i], ptr::null(), test_syscall_getrandom, ptr::null_mut()) == 0,
            c"pthread_create(&threads[i], NULL, test_syscall_getrandom, NULL) == 0".as_ptr(),
        );
    }
    for i in 0..THREADS {
        pthread_join(threads[i], ptr::null_mut());
    }
    clock_gettime(CLOCK_MONOTONIC, &mut end);
    timespecsub(&end, &start, &mut diff);
    printf(c"   syscall: %u x %u times in %lu.%09lu seconds\n".as_ptr(), TRIALS as c_uint, THREADS as c_uint, diff.tv_sec as c_ulong, diff.tv_nsec as c_ulong);
}

unsafe fn fill() {
    let mut weird_size: [u8; 323929] = [0; 323929];
    loop {
        vgetrandom(
            weird_size.as_mut_ptr() as *mut c_void,
            mem::size_of_val(&weird_size),
            0,
        );
    }
}

unsafe fn kselftest() {
    let mut weird_size: [u8; 1263] = [0; 1263];
    let mut child: pid_t;

    ksft_print_header();
    vgetrandom_init();
    ksft_set_plan(2);

    for _i in 0..1000 {
        let ret: ssize_t = vgetrandom(
            weird_size.as_mut_ptr() as *mut c_void,
            mem::size_of_val(&weird_size),
            0,
        );
        ksft_assert(
            ret == mem::size_of_val(&weird_size) as ssize_t,
            c"ret == sizeof(weird_size)".as_ptr(),
        );
    }

    ksft_test_result_pass(c"getrandom: PASS\n".as_ptr());

    unshare(CLONE_NEWUSER);
    ksft_assert(unshare(CLONE_NEWTIME) == 0, c"unshare(CLONE_NEWTIME) == 0".as_ptr());
    child = fork();
    ksft_assert(child >= 0, c"child >= 0".as_ptr());
    if child == 0 {
        vgetrandom_init();
        child = getpid();
        ksft_assert(
            ptrace(PTRACE_TRACEME, 0, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) == 0,
            c"ptrace(PTRACE_TRACEME, 0, NULL, NULL) == 0".as_ptr(),
        );
        ksft_assert(kill(child, SIGSTOP) == 0, c"kill(child, SIGSTOP) == 0".as_ptr());
        ksft_assert(
            vgetrandom(
                weird_size.as_mut_ptr() as *mut c_void,
                mem::size_of_val(&weird_size),
                0,
            ) == mem::size_of_val(&weird_size) as ssize_t,
            c"vgetrandom(weird_size, sizeof(weird_size), 0) == sizeof(weird_size)".as_ptr(),
        );
        _exit(0);
    }
    loop {
        let mut info: ptrace_syscall_info = mem::zeroed();
        let mut status: c_int = 0;
        ksft_assert(waitpid(child, &mut status, 0) >= 0, c"waitpid(child, &status, 0) >= 0".as_ptr());
        if WIFEXITED(status) {
            ksft_assert(WEXITSTATUS(status) == 0, c"WEXITSTATUS(status) == 0".as_ptr());
            break;
        }
        ksft_assert(WIFSTOPPED(status), c"WIFSTOPPED(status)".as_ptr());
        if WSTOPSIG(status) == SIGSTOP {
            ksft_assert(
                ptrace(PTRACE_SETOPTIONS, child, 0, PTRACE_O_TRACESYSGOOD) == 0,
                c"ptrace(PTRACE_SETOPTIONS, child, 0, PTRACE_O_TRACESYSGOOD) == 0".as_ptr(),
            );
        } else if WSTOPSIG(status) == (SIGTRAP | 0x80) {
            ksft_assert(
                ptrace(
                    PTRACE_GET_SYSCALL_INFO,
                    child,
                    mem::size_of_val(&info),
                    &mut info as *mut _,
                ) > 0,
                c"ptrace(PTRACE_GET_SYSCALL_INFO, child, sizeof(info), &info) > 0".as_ptr(),
            );
            let entry = info.entry();
            if info.op == PTRACE_SYSCALL_INFO_ENTRY
                && entry.nr == __NR_getrandom as u64
                && entry.args[0] == weird_size.as_mut_ptr() as usize as u64
                && entry.args[1] == mem::size_of_val(&weird_size) as u64
            {
                ksft_exit_fail_msg(
                    c"vgetrandom passed buffer to syscall getrandom unexpectedly\n".as_ptr(),
                );
            }
        }
        ksft_assert(
            ptrace(PTRACE_SYSCALL, child, 0, 0) == 0,
            c"ptrace(PTRACE_SYSCALL, child, 0, 0) == 0".as_ptr(),
        );
    }

    ksft_test_result_pass(c"getrandom timens: PASS\n".as_ptr());

    ksft_exit_pass();
}

unsafe fn usage(argv0: *const c_char) {
    fprintf(stderr, c"Usage: %s [bench-single|bench-multi|fill]\n".as_ptr(), argv0);
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if argc == 1 {
        kselftest();
        return 0;
    }

    if argc != 2 {
        usage(*argv.add(0));
        return 1;
    }

    vgetrandom_init();

    if strcmp(*argv.add(1), c"bench-single".as_ptr()) == 0 {
        bench_single();
    } else if strcmp(*argv.add(1), c"bench-multi".as_ptr()) == 0 {
        bench_multi();
    } else if strcmp(*argv.add(1), c"fill".as_ptr()) == 0 {
        fill();
    } else {
        usage(*argv.add(0));
        return 1;
    }
    0
}
