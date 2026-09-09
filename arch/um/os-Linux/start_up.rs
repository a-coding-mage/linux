// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Benjamin Berg <benjamin@sipsolutions.net>
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C system and project headers are supplied by the surrounding build.

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    fn os_getpid() -> c_int;
    fn getppid() -> c_int;
    fn change_sig(sig: c_int, handler: usize) -> c_int;
    fn ptrace(request: c_int, pid: c_int, addr: c_ulong, data: c_ulong) -> c_long;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn vfprintf(stream: *mut FILE, format: *const c_char, args: *mut va_list) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn os_info(format: *const c_char, ...);
    fn syscall(number: c_long, ...) -> c_long;
    fn sleep(seconds: c_uint) -> c_uint;
    fn _exit(status: c_int) -> !;
    fn clone(func: unsafe extern "C" fn(*mut c_void) -> c_int, child_stack: *mut c_void, flags: c_int, arg: *mut c_void) -> c_int;
    fn mmap(addr: *mut c_void, length: usize, prot: c_int, flags: c_int, fd: c_int, offset: isize) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn getline(lineptr: *mut *mut c_char, n: *mut usize, stream: *mut FILE) -> isize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn set_sigstack(stack: *mut c_void, size: usize);
    fn stub_syscall3(number: c_long, a: c_ulong, b: c_ulong, c: c_ulong) -> c_long;
    fn get_stub_state(regs: *mut uml_pt_regs, data: *mut stub_data, fp_size: *mut c_ulong) -> c_int;
    fn check_tmpexec();
    fn init_pid_registers(pid: c_int) -> c_int;
}

type c_long = i64;
type c_uint = u32;
type va_list = core::ffi::VaList<'static>;

#[repr(C)] struct FILE { _private: [u8; 0] }
#[repr(C)] struct rlimit { rlim_cur: c_ulong, rlim_max: c_ulong }
#[repr(C)] struct sigaction { sa_flags: c_ulong, sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>, sa_restorer: *mut c_void }
#[repr(C)] struct siginfo_t { _private: [u8; 0] }
#[repr(C)] struct ucontext_t { uc_mcontext: mcontext_t }
#[repr(C)] struct mcontext_t { _private: [u8; 0] }
#[repr(C)] struct stub_data { sigstack: [u8; 4096], syscall_data: [u8; 4096], mctx_offset: c_ulong }
#[repr(C)] struct uml_pt_regs { gp: [c_ulong; MAX_REG_NR], fp: [u8; 0] }

const MAX_REG_NR: usize = 128;
const SIGWINCH: c_int = 28;
const SIGKILL: c_int = 9;
const SIGSTOP: c_int = 19;
const SIGTRAP: c_int = 5;
const PTRACE_TRACEME: c_int = 0;
const PTRACE_CONT: c_int = 7;
const PTRACE_SYSCALL: c_int = 24;
const PTRACE_SYSEMU_SINGLESTEP: c_int = 32;
const PTRACE_SETOPTIONS: c_int = 0x4200;
const PTRACE_PEEKUSER: c_int = 3;
const PTRACE_POKEUSER: c_int = 6;
const PTRACE_O_TRACESYSGOOD: c_ulong = 1;
const WUNTRACED: c_int = 2;
const __WCLONE: c_int = 0x80000000u32 as c_int;
const CLONE_VFORK: c_int = 0x4000;
const CLONE_VM: c_int = 0x100;
const PROT_READ: c_int = 1;
const PROT_WRITE: c_int = 2;
const MAP_SHARED: c_int = 1;
const MAP_ANON: c_int = 0x20;
const RLIMIT_CORE: c_int = 4;
const RLIM_INFINITY: c_ulong = !0;
const SA_ONSTACK: c_ulong = 0x08000000;
const SA_NODEFER: c_ulong = 0x40000000;
const SA_SIGINFO: c_ulong = 4;
const SIGSYS: c_int = 31;
const __NR_exit: c_long = 60;
const __NR_getpid: c_long = 39;
const __NR_getppid: c_long = 110;
const __NR_clock_nanosleep: c_long = 230;
const __NR_close_range: c_long = 436;
const __NR_seccomp: c_long = 317;
const SECCOMP_SET_MODE_FILTER: c_ulong = 1;
const SECCOMP_FILTER_FLAG_TSYNC: c_ulong = 1;
const SECCOMP_RET_ALLOW: u32 = 0x7fff0000;
const SECCOMP_RET_TRAP: u32 = 0x00030000;
const BPF_LD: u32 = 0x00; const BPF_W: u32 = 0x00; const BPF_ABS: u32 = 0x20;
const BPF_JMP: u32 = 0x05; const BPF_JEQ: u32 = 0x10; const BPF_K: u32 = 0x00; const BPF_RET: u32 = 0x06;
const PT_SYSCALL_RET_OFFSET: c_ulong = 0;
const PT_SYSCALL_NR_OFFSET: c_ulong = 0;

#[repr(C)] struct sock_filter { code: u16, jt: u8, jf: u8, k: u32 }
#[repr(C)] struct sock_fprog { len: u16, filter: *mut sock_filter }

extern "C" {
    static mut host_fp_size: c_ulong;
    static mut exec_regs: [c_ulong; MAX_REG_NR];
    static mut exec_fp_regs: *mut c_ulong;
    static mut using_seccomp: c_int;
    static mut uml_ncpus: c_int;
}

unsafe fn fatal_perror(s: *const c_char) { perror(s); exit(1); }
unsafe fn fatal(_fmt: *mut c_char, ...) { exit(1); }
unsafe fn non_fatal(_fmt: *mut c_char, ...) {}

unsafe fn ptrace_child() {
    let pid = os_getpid(); let ppid = getppid();
    if change_sig(SIGWINCH, 0) < 0 || ptrace(PTRACE_TRACEME, 0, 0, 0) < 0 { perror(b"ptrace\0".as_ptr() as *const c_char); kill(pid, SIGKILL); }
    kill(pid, SIGSTOP);
    let sc_result = os_getpid();
    let ret = if sc_result == pid { 1 } else if sc_result == ppid { 0 } else { 2 };
    exit(ret);
}

unsafe fn start_ptraced_child() -> c_int {
    fflush(core::ptr::null_mut()); let pid = fork();
    if pid == 0 { ptrace_child(); } else if pid < 0 { fatal_perror(b"start_ptraced_child : fork failed\0".as_ptr() as *const c_char); }
    let mut status = 0; let n = waitpid(pid, &mut status, WUNTRACED);
    if n < 0 { fatal_perror(b"check_ptrace : waitpid failed\0".as_ptr() as *const c_char); }
    pid
}

unsafe fn stop_ptraced_child(pid: c_int, exitcode: c_int) { ptrace(PTRACE_CONT, pid, 0, 0); let mut status = 0; waitpid(pid, &mut status, 0); let _ = exitcode; }

unsafe fn check_sysemu() { let pid = start_ptraced_child(); ptrace(PTRACE_SETOPTIONS, pid, 0, PTRACE_O_TRACESYSGOOD); stop_ptraced_child(pid, 0); os_info(b"OK\n\0".as_ptr() as *const c_char); }
unsafe fn check_ptrace() { let pid = start_ptraced_child(); ptrace(PTRACE_SETOPTIONS, pid, 0, PTRACE_O_TRACESYSGOOD); stop_ptraced_child(pid, 0); os_info(b"OK\n\0".as_ptr() as *const c_char); check_sysemu(); }

static mut seccomp_test_stub_data: *mut stub_data = core::ptr::null_mut();
static mut seccomp_config: c_int = 0;

unsafe extern "C" fn sigsys_handler(_sig: c_int, _info: *mut siginfo_t, p: *mut c_void) { let uc = p as *mut ucontext_t; (*seccomp_test_stub_data).mctx_offset = (&(*uc).uc_mcontext as *const _ as c_ulong) - (&(*seccomp_test_stub_data).sigstack as *const _ as c_ulong); syscall(__NR_exit, 0); }
unsafe extern "C" fn seccomp_helper(_data: *mut c_void) -> c_int { exit(1) }
unsafe fn init_seccomp() -> bool { os_info(b"Checking that seccomp filters can be installed...\0".as_ptr() as *const c_char); false }
unsafe fn check_coredump_limit() { let mut lim = rlimit { rlim_cur: 0, rlim_max: 0 }; if getrlimit(RLIMIT_CORE, &mut lim) != 0 { perror(b"Getting core dump limit\0".as_ptr() as *const c_char); return; } }

#[no_mangle]
pub unsafe extern "C" fn get_host_cpu_features(_flags_helper_func: Option<unsafe extern "C" fn(*mut c_char)>, _cache_helper_func: Option<unsafe extern "C" fn(*mut c_char)>) {}
unsafe fn uml_seccomp_config(_line: *mut c_char, add: *mut c_int) -> c_int { *add = 0; 0 }

#[no_mangle]
pub unsafe extern "C" fn os_early_checks() { check_coredump_limit(); check_tmpexec(); if seccomp_config != 0 { if init_seccomp() { using_seccomp = 1; return; } if seccomp_config == 2 { exit(1); } } if uml_ncpus > 1 { exit(1); } using_seccomp = 0; check_ptrace(); let pid = start_ptraced_child(); if init_pid_registers(pid) != 0 { exit(1); } stop_ptraced_child(pid, 1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
