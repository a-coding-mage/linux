// SPDX-License-Identifier: GPL-2.0

// Translated from testing/selftests/bpf/prog_tests/uprobe_syscall.c.
// C includes are external dependencies supplied by the selftest harness:
// test_progs.h, uprobe_syscall*.skel.h, usdt.h, and libbpf internals.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type bool_ = bool;
type size_t = usize;
type useconds_t = u32;
type pthread_t = c_ulong;
type __u8 = u8;
type __s32 = i32;
type __u64 = u64;

#[cfg(target_arch = "x86_64")]
const BPF_TESTMOD_UPROBE_TEST_FILE: &[u8] = b"/sys/kernel/bpf_testmod_uprobe\0";
#[cfg(target_arch = "x86_64")]
const TRAMP: &[u8] = b"[uprobes-trampoline]\0";
#[cfg(target_arch = "x86_64")]
const __NR_uretprobe: c_long = 335;
#[cfg(target_arch = "x86_64")]
const __NR_uprobe: c_long = 336;
#[cfg(target_arch = "x86_64")]
const ARCH_SHSTK_ENABLE: c_long = 0x5001;
#[cfg(target_arch = "x86_64")]
const ARCH_SHSTK_DISABLE: c_long = 0x5002;
#[cfg(target_arch = "x86_64")]
const ARCH_SHSTK_SHSTK: c_long = 1 << 0;
#[cfg(target_arch = "x86_64")]
const O_WRONLY: c_int = 1;
#[cfg(target_arch = "x86_64")]
const SIGILL: c_int = 4;
#[cfg(target_arch = "x86_64")]
const SIGCHLD: c_int = 17;
#[cfg(target_arch = "x86_64")]
const CLONE_VM: c_int = 0x00000100;
#[cfg(target_arch = "x86_64")]
const EPROTO: c_int = 71;
#[cfg(target_arch = "x86_64")]
const STT_OBJECT: c_int = 1;

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct pt_regs {
    r15: c_ulong,
    r14: c_ulong,
    r13: c_ulong,
    r12: c_ulong,
    rbp: c_ulong,
    rbx: c_ulong,
    r11: c_ulong,
    r10: c_ulong,
    r9: c_ulong,
    r8: c_ulong,
    rax: c_ulong,
    rcx: c_ulong,
    rdx: c_ulong,
    rsi: c_ulong,
    rdi: c_ulong,
    orig_rax: c_ulong,
    rip: c_ulong,
    cs: c_ulong,
    eflags: c_ulong,
    rsp: c_ulong,
    ss: c_ulong,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}
#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}
#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct uprobe_syscall_bss {
    regs: pt_regs,
}
#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct uprobe_syscall_progs {
    probe: *mut bpf_program,
}
#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct uprobe_syscall_links {
    probe: *mut bpf_link,
}
#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct uprobe_syscall {
    progs: uprobe_syscall_progs,
    links: uprobe_syscall_links,
    bss: *mut uprobe_syscall_bss,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct uprobe_syscall_executed_bss {
    pid: c_int,
    executed: c_int,
}
#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct uprobe_syscall_executed_progs {
    test_uretprobe_multi: *mut bpf_program,
    test_uprobe: *mut bpf_program,
    test_uprobe_multi: *mut bpf_program,
    test_uretprobe: *mut bpf_program,
    test_uprobe_session: *mut bpf_program,
    test_usdt: *mut bpf_program,
}
#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct uprobe_syscall_executed_links {
    test_uretprobe_multi: *mut bpf_link,
    test_uprobe: *mut bpf_link,
}
#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct uprobe_syscall_executed {
    progs: uprobe_syscall_executed_progs,
    links: uprobe_syscall_executed_links,
    bss: *mut uprobe_syscall_executed_bss,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct bpf_uprobe_opts {
    retprobe: bool_,
    ref_ctr_offset: c_ulong,
}
#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct bpf_uprobe_multi_opts {
    retprobe: bool_,
    session: bool_,
    offsets: *mut c_ulong,
    cnt: size_t,
}

#[cfg(target_arch = "x86_64")]
#[repr(C, packed)]
struct __arch_relative_insn {
    op: __u8,
    raddr: __s32,
}

#[cfg(target_arch = "x86_64")]
type trigger_t = unsafe extern "C" fn();

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    static mut errno: c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;

    fn ASSERT_GE(actual: c_long, expected: c_long, name: *const c_char) -> bool_;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool_;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool_;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool_;
    fn ASSERT_EQ(actual: c_ulong, expected: c_ulong, name: *const c_char) -> bool_;
    fn ASSERT_NEQ<T>(actual: *mut T, expected: *mut T, name: *const c_char) -> bool_;
    fn ASSERT_FALSE(actual: bool_, name: *const c_char) -> bool_;
    fn test__start_subtest(name: *const c_char) -> bool_;
    fn test__skip();

    fn get_uprobe_offset(addr: *const c_void) -> c_ulong;
    fn uprobe_syscall__open_and_load() -> *mut uprobe_syscall;
    fn uprobe_syscall__destroy(skel: *mut uprobe_syscall);
    fn uprobe_syscall_executed__open_and_load() -> *mut uprobe_syscall_executed;
    fn uprobe_syscall_executed__destroy(skel: *mut uprobe_syscall_executed);
    fn bpf_program__attach_uprobe_opts(prog: *mut bpf_program, pid: c_int, path: *const c_char, offset: c_ulong, opts: *const bpf_uprobe_opts) -> *mut bpf_link;
    fn bpf_program__attach_uprobe_multi(prog: *mut bpf_program, pid: c_int, path: *const c_char, func_name: *const c_char, opts: *const bpf_uprobe_multi_opts) -> *mut bpf_link;
    fn bpf_program__attach_usdt(prog: *mut bpf_program, pid: c_int, path: *const c_char, provider: *const c_char, name: *const c_char, opts: *mut c_void) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn elf_resolve_syms_offsets(path: *const c_char, cnt: c_int, syms: *mut *const c_char, offsets: *mut *mut c_ulong, typ: c_int) -> c_int;
    fn libbpf_num_possible_cpus() -> c_int;

    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn close(fd: c_int) -> c_int;
    fn pipe(fds: *mut c_int) -> c_int;
    fn fork() -> c_int;
    fn clone(f: unsafe extern "C" fn(*mut c_void) -> c_int, child_stack: *mut c_void, flags: c_int, arg: *mut c_void) -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn _exit(status: c_int) -> !;
    fn getpid() -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn usleep(usec: useconds_t) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn free(ptr: *mut c_void);
    fn sys_gettid() -> c_long;

    fn USDT_SEMA_IS_ACTIVE_race() -> bool_;
}

#[cfg(target_arch = "x86_64")]
static mut nop10: [u8; 10] = [0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];
#[cfg(target_arch = "x86_64")]
static mut lea_rsp: [u8; 5] = [0x48, 0x8d, 0x64, 0x24, 0x80];
#[cfg(target_arch = "x86_64")]
static mut race_stop: bool_ = false;

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_regs_trigger() -> c_ulong {
    asm!(
        ".byte 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00",
        "mov rax, 0xdeadbeef",
        "ret",
        options(noreturn)
    );
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_regs(_before: *mut pt_regs, _after: *mut pt_regs) {
    asm!(
        "mov [rdi +   0], r15", "mov [rdi +   8], r14", "mov [rdi +  16], r13",
        "mov [rdi +  24], r12", "mov [rdi +  32], rbp", "mov [rdi +  40], rbx",
        "mov [rdi +  48], r11", "mov [rdi +  56], r10", "mov [rdi +  64], r9",
        "mov [rdi +  72], r8",  "mov [rdi +  80], rax", "mov [rdi +  88], rcx",
        "mov [rdi +  96], rdx", "mov [rdi + 104], rsi", "mov [rdi + 112], rdi",
        "mov qword ptr [rdi + 120], 0", "mov qword ptr [rdi + 128], 0", "mov qword ptr [rdi + 136], 0",
        "push rax", "pushfq", "pop rax", "mov [rdi + 144], rax", "pop rax",
        "mov [rdi + 152], rsp", "mov qword ptr [rdi + 160], 0",
        "push rsi", "call uprobe_regs_trigger", "push rax", "mov rax, [rsp + 8]",
        "mov [rax +   0], r15", "mov [rax +   8], r14", "mov [rax +  16], r13",
        "mov [rax +  24], r12", "mov [rax +  32], rbp", "mov [rax +  40], rbx",
        "mov [rax +  48], r11", "mov [rax +  56], r10", "mov [rax +  64], r9",
        "mov [rax +  72], r8",  "mov [rax +  88], rcx", "mov [rax +  96], rdx",
        "mov [rax + 104], rsi", "mov [rax + 112], rdi", "mov qword ptr [rax + 120], 0",
        "mov qword ptr [rax + 128], 0", "mov qword ptr [rax + 136], 0",
        "pop rax", "pop rsi", "mov [rsi + 80], rax", "pushfq", "pop rax",
        "mov [rsi + 144], rax", "mov [rsi + 152], rsp", "mov qword ptr [rsi + 160], 0",
        "ret",
        options(noreturn)
    );
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_uprobe_regs_equal(retprobe: bool_) {
    let mut opts = bpf_uprobe_opts { retprobe, ref_ctr_offset: 0 };
    let mut skel: *mut uprobe_syscall = ptr::null_mut();
    let mut before: pt_regs = core::mem::zeroed();
    let mut after: pt_regs = core::mem::zeroed();
    let pb = &mut before as *mut _ as *mut c_ulong;
    let pa = &mut after as *mut _ as *mut c_ulong;
    let offset = get_uprobe_offset(uprobe_regs_trigger as *const c_void);
    if !ASSERT_GE(offset as c_long, 0, c"get_uprobe_offset".as_ptr()) { return; }
    skel = uprobe_syscall__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_syscall__open_and_load".as_ptr()) { goto_cleanup_uprobe_syscall(skel); return; }
    (*skel).links.probe = bpf_program__attach_uprobe_opts((*skel).progs.probe, 0, c"/proc/self/exe".as_ptr(), offset, &mut opts);
    if !ASSERT_OK_PTR((*skel).links.probe, c"bpf_program__attach_uprobe_opts".as_ptr()) { goto_cleanup_uprobe_syscall(skel); return; }
    if !retprobe { uprobe_regs_trigger(); }
    uprobe_regs(&mut before, &mut after);
    let pp = &mut (*(*skel).bss).regs as *mut _ as *mut c_ulong;
    let cnt = size_of::<pt_regs>() / size_of::<c_ulong>();
    for i in 0..cnt {
        let offset = (i * size_of::<c_ulong>()) as u32;
        match offset as usize {
            x if x == offset_of!(pt_regs, rax) => { ASSERT_EQ(*pa.add(i), 0xdeadbeef, c"return value".as_ptr()); }
            _ => if !ASSERT_EQ(*pb.add(i), *pa.add(i), c"register before-after value check".as_ptr()) {
                fprintf(stdout, c"failed register offset %u\n".as_ptr(), offset);
            },
        }
        match offset as usize {
            x if x == offset_of!(pt_regs, orig_rax) || x == offset_of!(pt_regs, rip) || x == offset_of!(pt_regs, cs) || x == offset_of!(pt_regs, rsp) || x == offset_of!(pt_regs, ss) => {}
            x if x == offset_of!(pt_regs, rax) && !retprobe => { ASSERT_EQ(*pp.add(i), *pb.add(i), c"uprobe rax prog-before value check".as_ptr()); }
            _ => if !ASSERT_EQ(*pp.add(i), *pa.add(i), c"register prog-after value check".as_ptr()) {
                fprintf(stdout, c"failed register offset %u\n".as_ptr(), offset);
            },
        }
    }
    goto_cleanup_uprobe_syscall(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn goto_cleanup_uprobe_syscall(skel: *mut uprobe_syscall) {
    uprobe_syscall__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn write_bpf_testmod_uprobe(offset: c_ulong) -> c_int {
    let mut buf = [0 as c_char; 30];
    let n = sprintf(buf.as_mut_ptr(), c"%lu".as_ptr(), offset) as size_t;
    let fd = open(BPF_TESTMOD_UPROBE_TEST_FILE.as_ptr() as *const c_char, O_WRONLY);
    if fd < 0 { return -errno; }
    let ret = write(fd, buf.as_ptr() as *const c_void, n);
    close(fd);
    if ret as size_t != n { ret as c_int } else { 0 }
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_regs_change() {
    let mut before: pt_regs = core::mem::zeroed();
    let mut after: pt_regs = core::mem::zeroed();
    let pb = &mut before as *mut _ as *mut c_ulong;
    let pa = &mut after as *mut _ as *mut c_ulong;
    let cnt = size_of::<pt_regs>() / size_of::<c_ulong>();
    let offset = get_uprobe_offset(uprobe_regs_trigger as *const c_void);
    let mut err = write_bpf_testmod_uprobe(offset);
    if !ASSERT_OK(err, c"register_uprobe".as_ptr()) { return; }
    uprobe_regs_trigger();
    uprobe_regs(&mut before, &mut after);
    err = write_bpf_testmod_uprobe(0);
    if !ASSERT_OK(err, c"unregister_uprobe".as_ptr()) { return; }
    for i in 0..cnt {
        let offset = (i * size_of::<c_ulong>()) as u32;
        match offset as usize {
            x if x == offset_of!(pt_regs, rax) => { ASSERT_EQ(*pa.add(i), 0x12345678deadbeef, c"rax".as_ptr()); }
            x if x == offset_of!(pt_regs, rcx) => { ASSERT_EQ(*pa.add(i), 0x87654321feebdaed, c"rcx".as_ptr()); }
            x if x == offset_of!(pt_regs, r11) => { ASSERT_EQ(*pa.add(i), (-1i64) as __u64 as c_ulong, c"r11".as_ptr()); }
            _ => if !ASSERT_EQ(*pa.add(i), *pb.add(i), c"register before-after value check".as_ptr()) {
                fprintf(stdout, c"failed register offset %u\n".as_ptr(), offset);
            },
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uretprobe_syscall_call_1() -> c_ulong {
    asm!(
        "push rax", "push rcx", "push r11", "mov rax, 335", "syscall",
        "pop r11", "pop rcx", "ret",
        options(noreturn)
    );
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uretprobe_syscall_call() -> c_ulong {
    asm!("call uretprobe_syscall_call_1", "ret", options(noreturn));
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_test() {
    asm!(".byte 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00", "ret", options(noreturn));
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn usdt_test() {
    // USDT(optimized_uprobe, usdt);
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_red_zone_test() -> c_ulong {
    asm!(
        "movabs rax, 0x1111111111111111", "mov [rsp - 8], rax",
        "movabs rax, 0x2222222222222222", "mov [rsp - 16], rax",
        "movabs rax, 0x3333333333333333", "mov [rsp - 24], rax",
        ".byte 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00",
        "movabs rax, 0x1111111111111111", "cmp [rsp - 8], rax", "jne 2f",
        "movabs rax, 0x2222222222222222", "cmp [rsp - 16], rax", "jne 2f",
        "movabs rax, 0x3333333333333333", "cmp [rsp - 24], rax", "jne 2f",
        "xor eax, eax", "ret", "2:", "mov eax, 1", "ret",
        options(noreturn)
    );
}

#[cfg(target_arch = "x86_64")]
unsafe fn find_nop10(fn_: *mut c_void) -> *mut c_void {
    for i in 0..128isize {
        if memcmp(nop10.as_ptr() as *const c_void, (fn_ as *mut u8).offset(i) as *const c_void, 10) == 0 {
            return (fn_ as *mut u8).offset(i) as *mut c_void;
        }
    }
    ptr::null_mut()
}

#[cfg(target_arch = "x86_64")]
unsafe fn find_uprobes_trampoline(tramp_addr: *mut c_void) -> c_int {
    let mut start: *mut c_void = ptr::null_mut();
    let mut end: *mut c_void = ptr::null_mut();
    let mut line = [0 as c_char; 128];
    let mut ret = -1;
    let maps = fopen(c"/proc/self/maps".as_ptr(), c"r".as_ptr());
    if maps.is_null() {
        fprintf(stderr, c"cannot open maps\n".as_ptr());
        return -1;
    }
    while !fgets(line.as_mut_ptr(), line.len() as c_int, maps).is_null() {
        let mut m: c_int = -1;
        if sscanf(line.as_ptr(), c"%p-%p r-xp %*x %*x:%*x %*u %n".as_ptr(), &mut start, &mut end, &mut m) != 2 { continue; }
        if m < 0 { continue; }
        if strncmp(line.as_ptr().add(m as usize), TRAMP.as_ptr() as *const c_char, TRAMP.len() - 1) == 0 && start == tramp_addr {
            ret = 0;
            break;
        }
    }
    fclose(maps);
    ret
}

#[cfg(target_arch = "x86_64")]
unsafe fn check_attach_notrigger(skel: *mut uprobe_syscall_executed, addr: *mut c_void, executed: c_int) {
    let op = addr as *mut u8;
    ASSERT_EQ((*(*skel).bss).executed as c_ulong, executed as c_ulong, c"executed".as_ptr());
    ASSERT_EQ(*op as c_ulong, 0xcc, c"int3".as_ptr());
}

#[cfg(target_arch = "x86_64")]
unsafe fn check_attach(skel: *mut uprobe_syscall_executed, trigger: trigger_t, addr: *mut c_void, executed: c_int) -> *mut c_void {
    trigger();
    trigger();
    ASSERT_EQ((*(*skel).bss).executed as c_ulong, executed as c_ulong, c"executed".as_ptr());
    ASSERT_OK(memcmp(addr, lea_rsp.as_ptr() as *const c_void, 5), c"lea_rsp".as_ptr());
    let call = (addr as *mut u8).add(5) as *mut __arch_relative_insn;
    let tramp = call.add(1).cast::<u8>().offset((*call).raddr as isize) as *mut c_void;
    ASSERT_EQ((*call).op as c_ulong, 0xe8, c"call".as_ptr());
    ASSERT_OK(find_uprobes_trampoline(tramp), c"uprobes_trampoline".as_ptr());
    tramp
}

#[cfg(target_arch = "x86_64")]
unsafe fn check_detach(addr: *mut c_void, tramp: *mut c_void) -> bool_ {
    static nop10_prefix: [u8; 5] = [0x66, 0x2e, 0x0f, 0x1f, 0x84];
    let mut ok = true;
    ok &= ASSERT_OK(find_uprobes_trampoline(tramp), c"uprobes_trampoline".as_ptr());
    ok &= ASSERT_OK(memcmp(addr, nop10_prefix.as_ptr() as *const c_void, 5), c"nop10_prefix".as_ptr());
    ok
}

#[cfg(target_arch = "x86_64")]
unsafe fn check(skel: *mut uprobe_syscall_executed, link: *mut bpf_link, trigger: trigger_t, addr: *mut c_void, executed: c_int) -> *mut c_void {
    let tramp = check_attach(skel, trigger, addr, executed);
    bpf_link__destroy(link);
    check_detach(addr, tramp);
    tramp
}

#[cfg(target_arch = "x86_64")]
unsafe fn WIFSIGNALED(status: c_int) -> c_ulong { (((status & 0x7f) + 1) >> 1 > 0) as c_ulong }
#[cfg(target_arch = "x86_64")]
unsafe fn WTERMSIG(status: c_int) -> c_ulong { (status & 0x7f) as c_ulong }
#[cfg(target_arch = "x86_64")]
unsafe fn WIFEXITED(status: c_int) -> c_ulong { (WTERMSIG(status) == 0) as c_ulong }
#[cfg(target_arch = "x86_64")]
unsafe fn WEXITSTATUS(status: c_int) -> c_ulong { ((status & 0xff00) >> 8) as c_ulong }

#[cfg(target_arch = "x86_64")]
unsafe fn ARCH_PRCTL(arg1: c_long, arg2: c_long) -> c_long {
    let ret: c_long;
    asm!("syscall", inlateout("rax") 158 as c_long => ret, in("rdi") arg1, in("rsi") arg2, lateout("rcx") _, lateout("r11") _, options(nostack));
    ret
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_uretprobe_shadow_stack() {
    if ARCH_PRCTL(ARCH_SHSTK_ENABLE, ARCH_SHSTK_SHSTK) != 0 {
        test__skip();
        return;
    }
    test_uprobe_regs_equal(false);
    test_uprobe_regs_equal(true);
    test_uretprobe_syscall_call();
    test_uprobe_legacy();
    test_uprobe_multi();
    test_uprobe_session();
    test_uprobe_usdt();
    test_regs_change();
    ARCH_PRCTL(ARCH_SHSTK_DISABLE, ARCH_SHSTK_SHSTK);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_uretprobe_syscall_call() {
    let mut opts = bpf_uprobe_multi_opts { retprobe: true, session: false, offsets: ptr::null_mut(), cnt: 0 };
    let mut skel: *mut uprobe_syscall_executed = ptr::null_mut();
    let mut pid: c_int;
    let mut status: c_int = 0;
    let mut go = [0 as c_int; 2];
    let mut c: c_int = 0;
    if !ASSERT_OK(pipe(go.as_mut_ptr()), c"pipe".as_ptr()) { return; }
    skel = uprobe_syscall_executed__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_syscall_executed__open_and_load".as_ptr()) { cleanup_executed_pipe(skel, &go); return; }
    pid = fork();
    if !ASSERT_GE(pid as c_long, 0, c"fork".as_ptr()) { cleanup_executed_pipe(skel, &go); return; }
    if pid == 0 {
        close(go[1]);
        if read(go[0], &mut c as *mut _ as *mut c_void, 1) != 1 { exit(-1); }
        uretprobe_syscall_call();
        _exit(0);
    }
    (*(*skel).bss).pid = pid;
    let link = bpf_program__attach_uprobe_multi((*skel).progs.test_uretprobe_multi, pid, c"/proc/self/exe".as_ptr(), c"uretprobe_syscall_call".as_ptr(), &mut opts);
    if !ASSERT_OK_PTR(link, c"bpf_program__attach_uprobe_multi".as_ptr()) { cleanup_executed_pipe(skel, &go); return; }
    (*skel).links.test_uretprobe_multi = link;
    write(go[1], &mut c as *mut _ as *const c_void, 1);
    let err = waitpid(pid, &mut status, 0);
    ASSERT_EQ(err as c_ulong, pid as c_ulong, c"waitpid".as_ptr());
    ASSERT_EQ(WIFSIGNALED(status), 1, c"WIFSIGNALED".as_ptr());
    ASSERT_EQ(WTERMSIG(status), SIGILL as c_ulong, c"WTERMSIG".as_ptr());
    ASSERT_EQ((*(*skel).bss).executed as c_ulong, 0, c"executed".as_ptr());
    cleanup_executed_pipe(skel, &go);
}

#[cfg(target_arch = "x86_64")]
unsafe fn cleanup_executed_pipe(skel: *mut uprobe_syscall_executed, go: &[c_int; 2]) {
    uprobe_syscall_executed__destroy(skel);
    close(go[1]);
    close(go[0]);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_uprobe_legacy() {
    let mut skel: *mut uprobe_syscall_executed = ptr::null_mut();
    let opts = bpf_uprobe_opts { retprobe: true, ref_ctr_offset: 0 };
    let offset = get_uprobe_offset(uprobe_test as *const c_void);
    if !ASSERT_GE(offset as c_long, 0, c"get_uprobe_offset".as_ptr()) { uprobe_syscall_executed__destroy(skel); return; }
    skel = uprobe_syscall_executed__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_syscall_executed__open_and_load".as_ptr()) { return; }
    (*(*skel).bss).pid = getpid();
    let mut link = bpf_program__attach_uprobe_opts((*skel).progs.test_uprobe, 0, c"/proc/self/exe".as_ptr(), offset, ptr::null());
    if !ASSERT_OK_PTR(link, c"bpf_program__attach_uprobe_opts".as_ptr()) { uprobe_syscall_executed__destroy(skel); return; }
    let tramp = check(skel, link, uprobe_test, uprobe_test as *mut c_void, 2);
    link = bpf_program__attach_uprobe_opts((*skel).progs.test_uprobe, 0, c"/proc/self/exe".as_ptr(), offset, ptr::null());
    if ASSERT_OK_PTR(link, c"bpf_program__attach_uprobe_opts".as_ptr()) {
        check_attach_notrigger(skel, uprobe_test as *mut c_void, 2);
        bpf_link__destroy(link);
        if check_detach(uprobe_test as *mut c_void, tramp) {
            uprobe_test();
            ASSERT_EQ((*(*skel).bss).executed as c_ulong, 2, c"executed_no_probe".as_ptr());
        }
    }
    link = bpf_program__attach_uprobe_opts((*skel).progs.test_uprobe, 0, c"/proc/self/exe".as_ptr(), offset, ptr::null());
    if ASSERT_OK_PTR(link, c"bpf_program__attach_uprobe_opts".as_ptr()) { check(skel, link, uprobe_test, uprobe_test as *mut c_void, 4); }
    (*(*skel).bss).executed = 0;
    link = bpf_program__attach_uprobe_opts((*skel).progs.test_uretprobe, 0, c"/proc/self/exe".as_ptr(), offset, &opts);
    if ASSERT_OK_PTR(link, c"bpf_program__attach_uprobe_opts".as_ptr()) { check(skel, link, uprobe_test, uprobe_test as *mut c_void, 2); }
    uprobe_syscall_executed__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_uprobe_multi() {
    let mut opts = bpf_uprobe_multi_opts { retprobe: false, session: false, offsets: ptr::null_mut(), cnt: 0 };
    let mut offset = get_uprobe_offset(uprobe_test as *const c_void);
    if !ASSERT_GE(offset as c_long, 0, c"get_uprobe_offset".as_ptr()) { return; }
    opts.offsets = &mut offset;
    opts.cnt = 1;
    let skel = uprobe_syscall_executed__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_syscall_executed__open_and_load".as_ptr()) { return; }
    (*(*skel).bss).pid = getpid();
    let mut link = bpf_program__attach_uprobe_multi((*skel).progs.test_uprobe_multi, 0, c"/proc/self/exe".as_ptr(), ptr::null(), &mut opts);
    if !ASSERT_OK_PTR(link, c"bpf_program__attach_uprobe_multi".as_ptr()) { uprobe_syscall_executed__destroy(skel); return; }
    let tramp = check(skel, link, uprobe_test, uprobe_test as *mut c_void, 2);
    link = bpf_program__attach_uprobe_multi((*skel).progs.test_uprobe_multi, 0, c"/proc/self/exe".as_ptr(), ptr::null(), &mut opts);
    if ASSERT_OK_PTR(link, c"bpf_program__attach_uprobe_multi".as_ptr()) {
        check_attach_notrigger(skel, uprobe_test as *mut c_void, 2);
        bpf_link__destroy(link);
        if check_detach(uprobe_test as *mut c_void, tramp) {
            uprobe_test();
            ASSERT_EQ((*(*skel).bss).executed as c_ulong, 2, c"executed_no_probe".as_ptr());
        }
    }
    link = bpf_program__attach_uprobe_multi((*skel).progs.test_uprobe_multi, 0, c"/proc/self/exe".as_ptr(), ptr::null(), &mut opts);
    if ASSERT_OK_PTR(link, c"bpf_program__attach_uprobe_multi".as_ptr()) { check(skel, link, uprobe_test, uprobe_test as *mut c_void, 4); }
    (*(*skel).bss).executed = 0;
    opts.retprobe = true;
    link = bpf_program__attach_uprobe_multi((*skel).progs.test_uretprobe_multi, 0, c"/proc/self/exe".as_ptr(), ptr::null(), &mut opts);
    if ASSERT_OK_PTR(link, c"bpf_program__attach_uprobe_multi".as_ptr()) { check(skel, link, uprobe_test, uprobe_test as *mut c_void, 2); }
    uprobe_syscall_executed__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_uprobe_session() {
    let mut opts = bpf_uprobe_multi_opts { retprobe: false, session: true, offsets: ptr::null_mut(), cnt: 0 };
    let mut offset = get_uprobe_offset(uprobe_test as *const c_void);
    if !ASSERT_GE(offset as c_long, 0, c"get_uprobe_offset".as_ptr()) { return; }
    opts.offsets = &mut offset; opts.cnt = 1;
    let skel = uprobe_syscall_executed__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_syscall_executed__open_and_load".as_ptr()) { return; }
    (*(*skel).bss).pid = getpid();
    let mut link = bpf_program__attach_uprobe_multi((*skel).progs.test_uprobe_session, 0, c"/proc/self/exe".as_ptr(), ptr::null(), &mut opts);
    if !ASSERT_OK_PTR(link, c"bpf_program__attach_uprobe_multi".as_ptr()) { uprobe_syscall_executed__destroy(skel); return; }
    let tramp = check(skel, link, uprobe_test, uprobe_test as *mut c_void, 4);
    link = bpf_program__attach_uprobe_multi((*skel).progs.test_uprobe_session, 0, c"/proc/self/exe".as_ptr(), ptr::null(), &mut opts);
    if ASSERT_OK_PTR(link, c"bpf_program__attach_uprobe_multi".as_ptr()) {
        check_attach_notrigger(skel, uprobe_test as *mut c_void, 4);
        bpf_link__destroy(link);
        if check_detach(uprobe_test as *mut c_void, tramp) {
            uprobe_test();
            ASSERT_EQ((*(*skel).bss).executed as c_ulong, 4, c"executed_no_probe".as_ptr());
        }
    }
    link = bpf_program__attach_uprobe_multi((*skel).progs.test_uprobe_session, 0, c"/proc/self/exe".as_ptr(), ptr::null(), &mut opts);
    if ASSERT_OK_PTR(link, c"bpf_program__attach_uprobe_multi".as_ptr()) { check(skel, link, uprobe_test, uprobe_test as *mut c_void, 8); }
    uprobe_syscall_executed__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_uprobe_usdt() {
    errno = 0;
    let addr = find_nop10(usdt_test as *mut c_void);
    if !ASSERT_OK_PTR(addr, c"find_nop10".as_ptr()) { return; }
    let skel = uprobe_syscall_executed__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_syscall_executed__open_and_load".as_ptr()) { return; }
    (*(*skel).bss).pid = getpid();
    let mut link = bpf_program__attach_usdt((*skel).progs.test_usdt, -1, c"/proc/self/exe".as_ptr(), c"optimized_uprobe".as_ptr(), c"usdt".as_ptr(), ptr::null_mut());
    if !ASSERT_OK_PTR(link, c"bpf_program__attach_usdt".as_ptr()) { uprobe_syscall_executed__destroy(skel); return; }
    let tramp = check(skel, link, usdt_test, addr, 2);
    link = bpf_program__attach_usdt((*skel).progs.test_usdt, -1, c"/proc/self/exe".as_ptr(), c"optimized_uprobe".as_ptr(), c"usdt".as_ptr(), ptr::null_mut());
    if ASSERT_OK_PTR(link, c"bpf_program__attach_usdt".as_ptr()) {
        check_attach_notrigger(skel, addr, 2);
        bpf_link__destroy(link);
        if check_detach(addr, tramp) {
            usdt_test();
            ASSERT_EQ((*(*skel).bss).executed as c_ulong, 2, c"executed_no_probe".as_ptr());
        }
    }
    link = bpf_program__attach_usdt((*skel).progs.test_usdt, -1, c"/proc/self/exe".as_ptr(), c"optimized_uprobe".as_ptr(), c"usdt".as_ptr(), ptr::null_mut());
    if ASSERT_OK_PTR(link, c"bpf_program__attach_usdt".as_ptr()) { check(skel, link, usdt_test, addr, 4); }
    uprobe_syscall_executed__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn worker_trigger(_arg: *mut c_void) -> *mut c_void {
    let mut rounds: c_ulong = 0;
    while !race_stop {
        uprobe_test();
        rounds += 1;
    }
    printf(c"tid %ld trigger rounds: %lu\n".as_ptr(), sys_gettid(), rounds);
    ptr::null_mut()
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn worker_attach(_arg: *mut c_void) -> *mut c_void {
    let mut opts = bpf_uprobe_opts { retprobe: false, ref_ctr_offset: 0 };
    let mut rounds: c_ulong = 0;
    let offset = get_uprobe_offset(uprobe_test as *const c_void);
    if !ASSERT_GE(offset as c_long, 0, c"get_uprobe_offset".as_ptr()) { return ptr::null_mut(); }
    let mut sema = [c"USDT_SEMA(race)".as_ptr(), ptr::null()];
    let mut ref_: *mut c_ulong = ptr::null_mut();
    let err = elf_resolve_syms_offsets(c"/proc/self/exe".as_ptr(), 1, sema.as_mut_ptr(), &mut ref_, STT_OBJECT);
    if !ASSERT_OK(err, c"elf_resolve_syms_offsets_sema".as_ptr()) { return ptr::null_mut(); }
    opts.ref_ctr_offset = *ref_;
    let skel = uprobe_syscall_executed__open_and_load();
    if !ASSERT_OK_PTR(skel, c"uprobe_syscall_executed__open_and_load".as_ptr()) { return ptr::null_mut(); }
    (*(*skel).bss).pid = getpid();
    while !race_stop {
        (*skel).links.test_uprobe = bpf_program__attach_uprobe_opts((*skel).progs.test_uprobe, 0, c"/proc/self/exe".as_ptr(), offset, &opts);
        if !ASSERT_OK_PTR((*skel).links.test_uprobe, c"bpf_program__attach_uprobe_opts".as_ptr()) { break; }
        bpf_link__destroy((*skel).links.test_uprobe);
        (*skel).links.test_uprobe = ptr::null_mut();
        rounds += 1;
    }
    printf(c"tid %ld attach rounds: %lu hits: %d\n".as_ptr(), sys_gettid(), rounds, (*(*skel).bss).executed);
    uprobe_syscall_executed__destroy(skel);
    free(ref_ as *mut c_void);
    ptr::null_mut()
}

#[cfg(target_arch = "x86_64")]
unsafe fn race_msec() -> useconds_t {
    let env = getenv(c"BPF_SELFTESTS_UPROBE_SYSCALL_RACE_MSEC".as_ptr());
    if !env.is_null() { return atoi(env) as useconds_t; }
    500
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_uprobe_race() {
    let mut nr_threads = libbpf_num_possible_cpus();
    if !ASSERT_GT(nr_threads, 0, c"libbpf_num_possible_cpus".as_ptr()) { return; }
    nr_threads = core::cmp::max(2, nr_threads);
    let mut threads = vec![0 as pthread_t; nr_threads as usize];
    let mut i = 0;
    while i < nr_threads {
        let start = if i % 2 != 0 { worker_trigger } else { worker_attach };
        let err = pthread_create(&mut threads[i as usize], ptr::null(), start, ptr::null_mut());
        if !ASSERT_OK(err, c"pthread_create".as_ptr()) { break; }
        i += 1;
    }
    usleep(race_msec() * 1000);
    race_stop = true;
    let joined = i;
    i = 0;
    while i < joined {
        pthread_join(threads[i as usize], ptr::null_mut());
        i += 1;
    }
    ASSERT_FALSE(USDT_SEMA_IS_ACTIVE_race(), c"race_semaphore".as_ptr());
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_uprobe_red_zone() {
    let nop10_addr = find_nop10(uprobe_red_zone_test as *mut c_void);
    if !ASSERT_NEQ(nop10_addr, ptr::null_mut(), c"find_nop10".as_ptr()) { return; }
    let skel = uprobe_syscall_executed__open_and_load();
    if !ASSERT_OK_PTR(skel, c"open_and_load".as_ptr()) { return; }
    let offset = get_uprobe_offset(nop10_addr);
    let link = bpf_program__attach_uprobe_opts((*skel).progs.test_uprobe, 0, c"/proc/self/exe".as_ptr(), offset, ptr::null());
    if ASSERT_OK_PTR(link, c"attach_uprobe".as_ptr()) {
        for _ in 0..10 {
            ASSERT_EQ(uprobe_red_zone_test(), 0, c"red_zone_intact".as_ptr());
        }
        bpf_link__destroy(link);
    }
    uprobe_syscall_executed__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_uprobe_error() {
    let err = syscall(__NR_uprobe);
    ASSERT_EQ(err as c_ulong, (-1i64) as c_ulong, c"error".as_ptr());
    ASSERT_EQ(errno as c_ulong, EPROTO as c_ulong, c"errno".as_ptr());
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_fork_test() {
    asm!(".byte 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00", "ret", options(noreturn));
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn child_func(arg: *mut c_void) -> c_int {
    let skel = arg as *mut uprobe_syscall_executed;
    if memcmp(uprobe_fork_test as *const c_void, lea_rsp.as_ptr() as *const c_void, lea_rsp.len()) != 0 { _exit(1); }
    (*(*skel).bss).pid = getpid();
    uprobe_fork_test();
    if (*(*skel).bss).executed != 3 { _exit(2); }
    _exit(0);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_uprobe_fork_optimized(clone_vm: bool_) {
    let mut skel: *mut uprobe_syscall_executed = ptr::null_mut();
    let offset = get_uprobe_offset(uprobe_fork_test as *const c_void);
    if !ASSERT_GE(offset as c_long, 0, c"get_uprobe_offset".as_ptr()) { return; }
    skel = uprobe_syscall_executed__open_and_load();
    if !ASSERT_OK_PTR(skel, c"open_and_load".as_ptr()) { uprobe_syscall_executed__destroy(skel); return; }
    (*skel).links.test_uprobe = bpf_program__attach_uprobe_opts((*skel).progs.test_uprobe, -1, c"/proc/self/exe".as_ptr(), offset, ptr::null());
    if !ASSERT_OK_PTR((*skel).links.test_uprobe, c"attach_uprobe".as_ptr()) { uprobe_syscall_executed__destroy(skel); return; }
    (*(*skel).bss).pid = getpid();
    uprobe_fork_test();
    uprobe_fork_test();
    if !ASSERT_OK(memcmp(uprobe_fork_test as *const c_void, lea_rsp.as_ptr() as *const c_void, lea_rsp.len()), c"optimized".as_ptr()) {
        uprobe_syscall_executed__destroy(skel);
        return;
    }
    let mut stack = [0 as c_char; 65535];
    let pid = if clone_vm {
        clone(child_func, stack.as_mut_ptr().add(stack.len()) as *mut c_void, CLONE_VM | SIGCHLD, skel as *mut c_void)
    } else {
        let p = fork();
        if !ASSERT_GE(p as c_long, 0, c"fork".as_ptr()) { uprobe_syscall_executed__destroy(skel); return; }
        if p == 0 { child_func(skel as *mut c_void); }
        p
    };
    if clone_vm && !ASSERT_GT(pid, 0, c"clone".as_ptr()) { uprobe_syscall_executed__destroy(skel); return; }
    let mut status = 0;
    let err = waitpid(pid, &mut status, 0);
    if ASSERT_EQ(err as c_ulong, pid as c_ulong, c"waitpid".as_ptr()) {
        ASSERT_EQ(WIFEXITED(status), 1, c"child_exited".as_ptr());
        ASSERT_EQ(WEXITSTATUS(status), 0, c"child_exit_code".as_ptr());
    }
    uprobe_syscall_executed__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn __test_uprobe_syscall() {
    if test__start_subtest(c"uretprobe_regs_equal".as_ptr()) { test_uprobe_regs_equal(true); }
    if test__start_subtest(c"uretprobe_syscall_call".as_ptr()) { test_uretprobe_syscall_call(); }
    if test__start_subtest(c"uretprobe_shadow_stack".as_ptr()) { test_uretprobe_shadow_stack(); }
    if test__start_subtest(c"uprobe_legacy".as_ptr()) { test_uprobe_legacy(); }
    if test__start_subtest(c"uprobe_multi".as_ptr()) { test_uprobe_multi(); }
    if test__start_subtest(c"uprobe_session".as_ptr()) { test_uprobe_session(); }
    if test__start_subtest(c"uprobe_usdt".as_ptr()) { test_uprobe_usdt(); }
    if test__start_subtest(c"uprobe_race".as_ptr()) { test_uprobe_race(); }
    if test__start_subtest(c"uprobe_red_zone".as_ptr()) { test_uprobe_red_zone(); }
    if test__start_subtest(c"uprobe_optimized_fork".as_ptr()) { test_uprobe_fork_optimized(false); }
    if test__start_subtest(c"uprobe_optimized_clone_vm".as_ptr()) { test_uprobe_fork_optimized(true); }
    if test__start_subtest(c"uprobe_error".as_ptr()) { test_uprobe_error(); }
    if test__start_subtest(c"uprobe_regs_equal".as_ptr()) { test_uprobe_regs_equal(false); }
    if test__start_subtest(c"regs_change".as_ptr()) { test_regs_change(); }
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn __test_uprobe_syscall() {
    unsafe extern "C" { fn test__skip(); }
    unsafe { test__skip(); }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_uprobe_syscall() {
    __test_uprobe_syscall();
}
