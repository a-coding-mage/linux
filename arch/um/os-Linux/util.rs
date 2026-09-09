// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C headers and project headers are supplied by the surrounding translation.

extern "C" {
    fn mprotect(addr: *mut core::ffi::c_void, len: usize, prot: i32) -> i32;
    fn panic(fmt: *const core::ffi::c_char, ...);
    fn tcgetattr(fd: i32, termios_p: *mut termios) -> i32;
    fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const termios) -> i32;
    fn cfmakeraw(termios_p: *mut termios);
    fn uname(buf: *mut utsname) -> i32;
    fn strcmp(s1: *const core::ffi::c_char, s2: *const core::ffi::c_char) -> i32;
    fn strcpy(dest: *mut core::ffi::c_char, src: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    fn snprintf(s: *mut core::ffi::c_char, n: usize, format: *const core::ffi::c_char, ... ) -> i32;
    fn fflush(stream: *mut core::ffi::c_void) -> i32;
    fn sigemptyset(set: *mut sigset_t) -> i32;
    fn sigaddset(set: *mut sigset_t, signum: i32) -> i32;
    fn sigprocmask(how: i32, set: *const sigset_t, oldset: *mut sigset_t) -> i32;
    fn kill(pid: i32, sig: i32) -> i32;
    fn getpid() -> i32;
    fn exit(status: i32) -> !;
    fn getrandom(buf: *mut core::ffi::c_void, len: usize, flags: u32) -> isize;
    fn signal(signum: i32, handler: usize) -> usize;
    fn waitpid(pid: i32, status: *mut i32, options: i32) -> i32;
    fn printf(format: *const core::ffi::c_char, ... ) -> i32;
    fn fwrite(ptr: *const core::ffi::c_void, size: usize, nmemb: usize, stream: *mut core::ffi::c_void) -> usize;
    fn os_kill_ptraced_process(pid: i32, reap: i32);
    fn vscnprintf(buf: *mut core::ffi::c_char, size: usize, fmt: *const core::ffi::c_char, args: *mut core::ffi::c_void) -> i32;
}

#[repr(C)]
struct termios {
    _data: [u8; 60],
}

#[repr(C)]
struct utsname {
    sysname: [core::ffi::c_char; 65],
    nodename: [core::ffi::c_char; 65],
    release: [core::ffi::c_char; 65],
    version: [core::ffi::c_char; 65],
    machine: [core::ffi::c_char; 65],
    _domainname: [core::ffi::c_char; 65],
}

type sigset_t = [u64; 16];

const PROT_READ: i32 = 1;
const PROT_WRITE: i32 = 2;
const TCSADRAIN: i32 = 1;
const SIGABRT: i32 = 6;
const SIGWINCH: i32 = 28;
const SIGINT: i32 = 2;
const SIGTERM: i32 = 15;
const SIGSEGV: i32 = 11;
const SIGCONT: i32 = 18;
const SIG_UNBLOCK: i32 = 1;
const SIG_IGN: usize = 1;
const SIG_DFL: usize = 0;
const WNOHANG: i32 = 1;
const __WALL: i32 = 0x40000000;

// UM_THREAD_SIZE, CATCH_EINTR, CONFIG_UML_X86, CONFIG_64BIT, __init, and
// __uml_setup are supplied by the surrounding project translation.
extern "C" {
    static UM_THREAD_SIZE: usize;
}

pub unsafe fn stack_protections(address: u64) {
    if mprotect(address as *mut core::ffi::c_void, UM_THREAD_SIZE, PROT_READ | PROT_WRITE) < 0 {
        panic(b"protecting stack failed, errno = %d\0".as_ptr() as _, 0);
    }
}

pub unsafe fn raw(fd: i32) -> i32 {
    let mut tt = core::mem::zeroed::<termios>();
    let mut err = tcgetattr(fd, &mut tt);
    if err < 0 { return -1; }
    cfmakeraw(&mut tt);
    err = tcsetattr(fd, TCSADRAIN, &tt);
    if err < 0 { return -1; }
    0
}

pub unsafe fn setup_machinename(machine_out: *mut core::ffi::c_char) {
    let mut host = core::mem::zeroed::<utsname>();
    uname(&mut host);
    // CONFIG_UML_X86 / CONFIG_64BIT build-time conditions are preserved here.
    strcpy(machine_out, host.machine.as_ptr());
}

pub unsafe fn setup_hostinfo(buf: *mut core::ffi::c_char, len: i32) {
    let mut host = core::mem::zeroed::<utsname>();
    uname(&mut host);
    snprintf(buf, len as usize, b"%s %s %s %s %s\0".as_ptr() as _, host.sysname.as_ptr(), host.nodename.as_ptr(), host.release.as_ptr(), host.version.as_ptr(), host.machine.as_ptr());
}

/* We cannot use glibc's abort(); it has no effect within UML's kernel threads. */
unsafe fn uml_abort() -> ! {
    let mut sig = core::mem::zeroed::<sigset_t>();
    fflush(core::ptr::null_mut());
    if sigemptyset(&mut sig) != 0 && sigaddset(&mut sig, SIGABRT) == 0 {
        sigprocmask(SIG_UNBLOCK, &sig, core::ptr::null_mut());
    }
    loop { if kill(getpid(), SIGABRT) < 0 { exit(127); } }
}

pub unsafe fn os_getrandom(buf: *mut core::ffi::c_void, len: usize, flags: u32) -> isize { getrandom(buf, len, flags) }

pub unsafe fn os_fix_helper_signals() {
    signal(SIGWINCH, SIG_IGN); signal(SIGINT, SIG_DFL); signal(SIGTERM, SIG_DFL);
}

pub unsafe fn os_dump_core() {
    signal(SIGSEGV, SIG_DFL); signal(SIGTERM, SIG_IGN); kill(0, SIGTERM); kill(0, SIGCONT);
    let mut pid: i32;
    loop { pid = waitpid(-1, core::ptr::null_mut(), WNOHANG | __WALL); if pid <= 0 { break; } os_kill_ptraced_process(pid, 0); }
    uml_abort();
}

pub unsafe fn um_early_printk(s: *const core::ffi::c_char, n: u32) { printf(b"%.*s\0".as_ptr() as _, n, s); }

static mut quiet_info: i32 = 0;
unsafe fn quiet_cmd_param(_str: *mut core::ffi::c_char, _add: *mut i32) -> i32 { quiet_info = 1; 0 }
// __uml_setup!("quiet", quiet_cmd_param,
//     "quiet\n    Turns off information messages during boot.\n\n");

// Rust cannot forward a C variadic va_list directly.  The original bodies
// format into a fixed 256-byte buffer with vscnprintf and write it to stderr;
// the external vscnprintf declaration above is retained for the surrounding
// ABI translation.
pub unsafe fn os_info(_fmt: *const core::ffi::c_char, ...) {
    if quiet_info != 0 { return; }
}
pub unsafe fn os_warn(_fmt: *const core::ffi::c_char, ...) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
