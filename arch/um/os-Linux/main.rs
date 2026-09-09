// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C headers and project headers are supplied by the surrounding build.
use std::os::raw::{c_char, c_int, c_long, c_uchar, c_ulong, c_void};
use std::ptr;

const STACKSIZE: c_ulong = 8 * 1024 * 1024;

// Build-time project constants and declarations supplied by the included headers.
extern "C" {
    static mut errno: c_int;
    static mut uml_exitcode: c_int;
    static mut kmalloc_ok: c_int;
    static mut uml_physmem: c_ulong;
    static mut high_physmem: c_ulong;
    static mut start_vm: c_ulong;
    static mut end_vm: c_ulong;

    fn getrlimit(resource: c_int, rlim: *mut Rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const Rlimit) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn sigemptyset(set: *mut Sigset) -> c_int;
    fn sigaction(signum: c_int, act: *const Sigaction, oldact: *mut Sigaction) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn putenv(string: *mut c_char) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn personality(persona: c_ulong) -> c_long;
    fn readlink(path: *const c_char, buf: *mut c_char, size: usize) -> isize;
    fn execve(path: *const c_char, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int;
    fn setsid() -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn execvp(file: *const c_char, argv: *const *mut c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn uml_cleanup();
    fn os_warn(format: *const c_char, ...);
    fn os_info(format: *const c_char, ...);
    fn scan_elf_aux(envp: *const *mut c_char);
    fn change_sig(signum: c_int, handler: c_int);
    fn os_timer_disable(arg: c_int);
    fn deactivate_all_fds() -> c_int;
    fn unblock_signals();
    fn linux_main(argc: c_int, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int;
    fn uml_kmalloc(size: c_int, flags: c_int) -> *mut c_void;
    fn vmalloc(size: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn vfree(ptr: *mut c_void);
    fn __real_malloc(size: c_int) -> *mut c_void;
    fn __real_free(ptr: *mut c_void);
}

#[repr(C)]
struct Rlimit { rlim_cur: c_ulong, rlim_max: c_ulong }
#[repr(C)] struct Sigset { _data: [c_ulong; 16] }
#[repr(C)] struct Sigaction {
    sa_mask: Sigset,
    sa_flags: c_ulong,
    sa_restorer: *mut c_void,
    sa_handler: Option<unsafe extern "C" fn(c_int)>,
}

const RLIMIT_STACK: c_int = 3;
const RLIM_INFINITY: c_ulong = !0;
const SA_RESETHAND: c_ulong = 0x80000000;
const SA_NODEFER: c_ulong = 0x40000000;
const PER_LINUX: c_ulong = 0;
const ADDR_NO_RANDOMIZE: c_ulong = 0x0040000;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const SIGPIPE: c_int = 13;
const SIGPROF: c_int = 27;
const ENOMEM: c_int = 12;
const UM_GFP_KERNEL: c_int = 0;
const UM_KERN_PAGE_SIZE: c_int = 4096;

unsafe extern "C" fn set_stklim() {
    let mut lim = Rlimit { rlim_cur: 0, rlim_max: 0 };
    if getrlimit(RLIMIT_STACK, &mut lim) < 0 { perror(b"getrlimit\0".as_ptr() as *const c_char); exit(1); }
    if lim.rlim_cur == RLIM_INFINITY || lim.rlim_cur > STACKSIZE {
        lim.rlim_cur = STACKSIZE;
        if setrlimit(RLIMIT_STACK, &lim) < 0 { perror(b"setrlimit\0".as_ptr() as *const c_char); exit(1); }
    }
}

unsafe extern "C" fn last_ditch_exit(_sig: c_int) { uml_cleanup(); exit(1); }

unsafe extern "C" fn install_fatal_handler(sig: c_int) {
    let mut action = Sigaction { sa_mask: Sigset { _data: [0; 16] }, sa_flags: SA_RESETHAND | SA_NODEFER, sa_restorer: ptr::null_mut(), sa_handler: Some(last_ditch_exit) };
    sigemptyset(&mut action.sa_mask);
    if sigaction(sig, &action, ptr::null_mut()) < 0 {
        os_warn(b"failed to install handler for signal %d - errno = %d\n\0".as_ptr() as *const c_char, sig, errno);
        exit(1);
    }
}

// UML_LIB_PATH is ":" OS_LIB_PATH "/uml"; OS_LIB_PATH is supplied by the build.
unsafe extern "C" fn setup_env_path() {
    let old_path = getenv(b"PATH\0".as_ptr() as *const c_char);
    if old_path.is_null() || strlen(old_path) == 0 {
        let p = b"PATH=:/bin:/usr/bin/:/usr/lib/uml\0";
        if putenv(p.as_ptr() as *mut c_char) != 0 { perror(b"couldn't putenv\0".as_ptr() as *const c_char); }
        return;
    }
    let path_len = strlen(old_path) + strlen(b"PATH=:/usr/lib/uml\0".as_ptr() as *const c_char) + 1;
    let new_path = malloc(path_len) as *mut c_char;
    if new_path.is_null() { perror(b"couldn't malloc to set a new PATH\0".as_ptr() as *const c_char); return; }
    snprintf(new_path, path_len, b"PATH=%s:/usr/lib/uml\0".as_ptr() as *const c_char, old_path);
    if putenv(new_path) != 0 { perror(b"couldn't putenv to set a new PATH\0".as_ptr() as *const c_char); free(new_path as *mut c_void); }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int {
    let ret = personality(PER_LINUX | ADDR_NO_RANDOMIZE);
    if ret >= 0 && (ret as c_ulong & (PER_LINUX | ADDR_NO_RANDOMIZE)) != (PER_LINUX | ADDR_NO_RANDOMIZE) {
        let mut buf = [0u8; 4096];
        let n = readlink(b"/proc/self/exe\0".as_ptr() as *const c_char, buf.as_mut_ptr() as *mut c_char, buf.len());
        if n < 0 || n >= buf.len() as isize { perror(b"readlink failure\0".as_ptr() as *const c_char); exit(1); }
        execve(buf.as_ptr() as *const c_char, argv, envp);
    }
    set_stklim(); setup_env_path(); setsid();
    let new_argv = malloc(((argc + 1) as usize) * std::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
    if new_argv.is_null() { perror(b"Mallocing argv\0".as_ptr() as *const c_char); exit(1); }
    for i in 0..argc { *new_argv.add(i as usize) = strdup(*argv.add(i as usize)); if (*new_argv.add(i as usize)).is_null() { perror(b"Mallocing an arg\0".as_ptr() as *const c_char); exit(1); } }
    *new_argv.add(argc as usize) = ptr::null_mut();
    install_fatal_handler(SIGINT); install_fatal_handler(SIGTERM); scan_elf_aux(envp);
    change_sig(SIGPIPE, 0); let mut ret = linux_main(argc, argv, envp); change_sig(SIGPROF, 0);
    os_timer_disable(0); let err = deactivate_all_fds(); if err != 0 { os_warn(b"deactivate_all_fds failed, errno = %d\n\0".as_ptr() as *const c_char, -err); }
    unblock_signals(); os_info(b"\n\0".as_ptr() as *const c_char);
    if ret != 0 { execvp(*new_argv, new_argv); perror(b"Failed to exec kernel\0".as_ptr() as *const c_char); ret = 1; }
    uml_exitcode
}

pub unsafe extern "C" fn __wrap_malloc(size: c_int) -> *mut c_void {
    let ret = if kmalloc_ok == 0 { __real_malloc(size) } else if size <= UM_KERN_PAGE_SIZE { uml_kmalloc(size, UM_GFP_KERNEL) } else { vmalloc(size) };
    if ret.is_null() { errno = ENOMEM; } ret
}
pub unsafe extern "C" fn __wrap_calloc(n: c_int, size: c_int) -> *mut c_void {
    let ptr = __wrap_malloc(n * size); if ptr.is_null() { return ptr::null_mut(); } memset(ptr, 0, (n * size) as usize); ptr
}
pub unsafe extern "C" fn __wrap_free(ptr: *mut c_void) {
    let addr = ptr as c_ulong;
    if addr >= uml_physmem && addr < high_physmem { if kmalloc_ok != 0 { kfree(ptr); } }
    else if addr >= start_vm && addr < end_vm { if kmalloc_ok != 0 { vfree(ptr); } }
    else { __real_free(ptr); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
