// SPDX-License-Identifier: GPL-2.0
/*
 * ldt_gdt.c - Test cases for LDT and GDT access
 * Copyright (c) 2015 Andrew Lutomirski
 *
 * Rust source-level translation of testing/selftests/x86/ldt_gdt.c.
 * C includes removed; this file expects the corresponding libc/Linux ABI
 * symbols and helpers to be supplied by the surrounding build.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, size_of_val, zeroed};
use core::ptr::{null, null_mut};

const AR_ACCESSED: u32 = 1 << 8;

const AR_TYPE_RODATA: u32 = 0 * (1 << 9);
const AR_TYPE_RWDATA: u32 = 1 * (1 << 9);
const AR_TYPE_RODATA_EXPDOWN: u32 = 2 * (1 << 9);
const AR_TYPE_RWDATA_EXPDOWN: u32 = 3 * (1 << 9);
const AR_TYPE_XOCODE: u32 = 4 * (1 << 9);
const AR_TYPE_XRCODE: u32 = 5 * (1 << 9);
const AR_TYPE_XOCODE_CONF: u32 = 6 * (1 << 9);
const AR_TYPE_XRCODE_CONF: u32 = 7 * (1 << 9);

const AR_DPL3: u32 = 3 * (1 << 13);

const AR_S: u32 = 1 << 12;
const AR_P: u32 = 1 << 15;
const AR_AVL: u32 = 1 << 20;
const AR_L: u32 = 1 << 21;
const AR_DB: u32 = 1 << 22;
const AR_G: u32 = 1 << 23;

const ENOSYS: c_int = 38;
const SIGSEGV: c_int = 11;
const SIGILL: c_int = 4;
const FUTEX_WAIT: c_int = 0;
const FUTEX_WAKE: c_int = 1;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_32BIT: c_int = 0x40;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const ARCH_SET_GS: c_int = 0x1001;
const ARCH_SET_FS: c_int = 0x1002;
const ARCH_GET_FS: c_int = 0x1003;
const ARCH_GET_GS: c_int = 0x1004;
const SYS_modify_ldt: c_long = 154;
const SYS_set_thread_area: c_long = 243;
const SYS_futex: c_long = 202;
const SYS_arch_prctl: c_long = 158;
const SYS_rt_sigaction: c_long = 13;
const SA_RESTORER: c_ulong = 0x04000000;

type pid_t = c_int;
type pthread_t = c_ulong;
/* jmp_buf is supplied by the C runtime/helper environment in the original test. */
type jmp_buf = [c_long; 8];

#[repr(C)]
#[derive(Copy, Clone)]
struct user_desc {
    entry_number: c_uint,
    base_addr: c_uint,
    limit: c_uint,
    seg_32bit: c_uint,
    contents: c_uint,
    read_exec_only: c_uint,
    limit_in_pages: c_uint,
    seg_not_present: c_uint,
    useable: c_uint,
    #[cfg(target_arch = "x86_64")]
    lm: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct cpu_set_t {
    bits: [c_ulong; 16],
}

#[repr(C)]
struct siginfo_t {
    _private: [u8; 128],
}

#[repr(C)]
struct fake_ksigaction {
    handler: *mut c_void, /* the real type is nasty */
    sa_flags: c_ulong,
    sa_restorer: Option<unsafe extern "C" fn()>,
    sigset: [u8; 8],
}

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn execl(path: *const c_char, arg0: *const c_char, ...) -> c_int;
    fn mmap(addr: *mut c_void, len: usize, prot: c_int, flags: c_int, fd: c_int, off: c_long) -> *mut c_void;
    fn sched_setaffinity(pid: pid_t, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn sigsetjmp(env: *mut jmp_buf, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut jmp_buf, val: c_int) -> !;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn __errno_location() -> *mut c_int;

    /* From helpers.h in the original selftest. */
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn sethandler(sig: c_int, handler: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void), flags: c_int);
}

static mut nerrs: c_int = 0;

/* Points to an array of 1024 ints, each holding its own index. */
static mut counter_page: *const c_uint = null();
static mut low_user_desc: *mut user_desc = null_mut();
static mut low_user_desc_clear: *mut user_desc = null_mut(); /* Use to delete GDT entry */
static mut gdt_entry_num: c_int = 0;

/* 0: thread is idle; 1: armed; 2: clear LDT entry 0; 3: exit */
static mut ftx: c_uint = 0;
static mut jmpbuf: jmp_buf = [0; 8];

unsafe fn errno_set(v: c_int) {
    *__errno_location() = v;
}

unsafe fn errno_get() -> c_int {
    *__errno_location()
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    (*set).bits = [0; 16];
}

unsafe fn CPU_SET(cpu: usize, set: *mut cpu_set_t) {
    (*set).bits[cpu / (8 * size_of::<c_ulong>())] |= 1usize.wrapping_shl((cpu % (8 * size_of::<c_ulong>())) as u32) as c_ulong;
}

unsafe fn check_invalid_segment(index: u16, ldt: c_int) {
    let mut has_limit: u32 = 0;
    let mut has_ar: u32 = 0;
    let mut limit: u32;
    let mut ar: u32;
    let selector: u32 = ((index as u32) << 3) | ((ldt as u32) << 2) | 3;

    asm!("lsl {selector:e}, {limit:e}", "jnz 2f", "mov {has_limit:e}, 1", "2:",
         selector = in(reg) selector, limit = lateout(reg) limit, has_limit = inout(reg) has_limit);
    asm!("lar {selector:e}, {ar:e}", "jnz 2f", "mov {has_ar:e}, 1", "2:",
         selector = in(reg) selector, ar = lateout(reg) ar, has_ar = inout(reg) has_ar);

    if has_limit != 0 || has_ar != 0 {
        println!("[FAIL]\t{} entry {} is valid but should be invalid", if ldt != 0 { "LDT" } else { "GDT" }, index);
        nerrs += 1;
    } else {
        println!("[OK]\t{} entry {} is invalid", if ldt != 0 { "LDT" } else { "GDT" }, index);
    }
}

unsafe fn check_valid_segment(index: u16, ldt: c_int, expected_ar: u32, expected_limit: u32, verbose: bool) {
    let mut has_limit: u32 = 0;
    let mut has_ar: u32 = 0;
    let mut limit: u32;
    let mut ar: u32;
    let selector: u32 = ((index as u32) << 3) | ((ldt as u32) << 2) | 3;

    asm!("lsl {selector:e}, {limit:e}", "jnz 2f", "mov {has_limit:e}, 1", "2:",
         selector = in(reg) selector, limit = lateout(reg) limit, has_limit = inout(reg) has_limit);
    asm!("lar {selector:e}, {ar:e}", "jnz 2f", "mov {has_ar:e}, 1", "2:",
         selector = in(reg) selector, ar = lateout(reg) ar, has_ar = inout(reg) has_ar);

    if has_limit == 0 || has_ar == 0 {
        println!("[FAIL]\t{} entry {} is invalid but should be valid", if ldt != 0 { "LDT" } else { "GDT" }, index);
        nerrs += 1;
        return;
    }

    /* The SDM says "bits 19:16 are undefined".  Thanks. */
    ar &= !0xF0000;

    /*
     * NB: Different Linux versions do different things with the
     * accessed bit in set_thread_area().
     */
    if ar != expected_ar && ar != (expected_ar | AR_ACCESSED) {
        println!("[FAIL]\t{} entry {} has AR 0x{:08X} but expected 0x{:08X}", if ldt != 0 { "LDT" } else { "GDT" }, index, ar, expected_ar);
        nerrs += 1;
    } else if limit != expected_limit {
        println!("[FAIL]\t{} entry {} has limit 0x{:08X} but expected 0x{:08X}", if ldt != 0 { "LDT" } else { "GDT" }, index, limit, expected_limit);
        nerrs += 1;
    } else if verbose {
        println!("[OK]\t{} entry {} has AR 0x{:08X} and limit 0x{:08X}", if ldt != 0 { "LDT" } else { "GDT" }, index, ar, limit);
    }
}

unsafe fn install_valid_mode(d: *const user_desc, ar: u32, oldmode: bool, ldt: bool) -> bool {
    let mut desc = *d;
    let ret: c_long;

    if !ldt {
        #[cfg(not(target_arch = "x86"))]
        {
            /* No point testing set_thread_area in a 64-bit build */
            return false;
        }
        #[cfg(target_arch = "x86")]
        {
            if gdt_entry_num == 0 {
                return false;
            }
            desc.entry_number = gdt_entry_num as c_uint;
            ret = syscall(SYS_set_thread_area, &mut desc as *mut user_desc);
        }
    } else {
        ret = syscall(SYS_modify_ldt, if oldmode { 1 } else { 0x11 }, &mut desc as *mut user_desc, size_of::<user_desc>());
        if ret < -1 {
            errno_set((-ret) as c_int);
        }
        if ret != 0 && errno_get() == ENOSYS {
            println!("[OK]\tmodify_ldt returned -ENOSYS");
            return false;
        }
    }

    if ret == 0 {
        let mut limit = desc.limit;
        if desc.limit_in_pages != 0 {
            limit = (limit << 12) + 4095;
        }
        check_valid_segment(desc.entry_number as u16, ldt as c_int, ar, limit, true);
        true
    } else if desc.seg_32bit != 0 {
        println!("[FAIL]\tUnexpected {} failure {}", if ldt { "modify_ldt" } else { "set_thread_area" }, errno_get());
        nerrs += 1;
        false
    } else {
        println!("[OK]\t{} rejected 16 bit segment", if ldt { "modify_ldt" } else { "set_thread_area" });
        false
    }
}

unsafe fn install_valid(desc: *const user_desc, ar: u32) -> bool {
    let ret = install_valid_mode(desc, ar, false, true);
    if (*desc).contents <= 1 && (*desc).seg_32bit != 0 && (*desc).seg_not_present == 0 {
        /* Should work in the GDT, too. */
        install_valid_mode(desc, ar, false, false);
    }
    ret
}

unsafe fn install_invalid(desc: *const user_desc, oldmode: bool) {
    let ret = syscall(SYS_modify_ldt, if oldmode { 1 } else { 0x11 }, desc, size_of::<user_desc>());
    if ret < -1 {
        errno_set((-ret) as c_int);
    }
    if ret == 0 {
        check_invalid_segment((*desc).entry_number as u16, 1);
    } else if errno_get() == ENOSYS {
        println!("[OK]\tmodify_ldt returned -ENOSYS");
    } else if (*desc).seg_32bit != 0 {
        println!("[FAIL]\tUnexpected modify_ldt failure {}", errno_get());
        nerrs += 1;
    } else {
        println!("[OK]\tmodify_ldt rejected 16 bit segment");
    }
}

unsafe fn safe_modify_ldt(func: c_int, ptr: *mut user_desc, bytecount: c_ulong) -> c_int {
    let ret = syscall(SYS_modify_ldt, 0x11, ptr, bytecount) as c_int;
    if ret < -1 {
        errno_set(-ret);
    }
    ret
}

unsafe fn fail_install(desc: *mut user_desc) {
    if safe_modify_ldt(0x11, desc, size_of::<user_desc>() as c_ulong) == 0 {
        println!("[FAIL]\tmodify_ldt accepted a bad descriptor");
        nerrs += 1;
    } else if errno_get() == ENOSYS {
        println!("[OK]\tmodify_ldt returned -ENOSYS");
    } else {
        println!("[OK]\tmodify_ldt failure {}", errno_get());
    }
}

unsafe fn do_simple_tests() {
    let mut desc: user_desc = zeroed();
    desc.entry_number = 0;
    desc.base_addr = 0;
    desc.limit = 10;
    desc.seg_32bit = 1;
    desc.contents = 2; /* Code, not conforming */
    desc.read_exec_only = 0;
    desc.limit_in_pages = 0;
    desc.seg_not_present = 0;
    desc.useable = 0;

    install_valid(&desc, AR_DPL3 | AR_TYPE_XRCODE | AR_S | AR_P | AR_DB);
    desc.limit_in_pages = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_XRCODE | AR_S | AR_P | AR_DB | AR_G);
    check_invalid_segment(1, 1);
    desc.entry_number = 2;
    install_valid(&desc, AR_DPL3 | AR_TYPE_XRCODE | AR_S | AR_P | AR_DB | AR_G);
    check_invalid_segment(1, 1);
    desc.base_addr = 0xf0000000;
    install_valid(&desc, AR_DPL3 | AR_TYPE_XRCODE | AR_S | AR_P | AR_DB | AR_G);
    desc.useable = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_XRCODE | AR_S | AR_P | AR_DB | AR_G | AR_AVL);
    desc.seg_not_present = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_XRCODE | AR_S | AR_DB | AR_G | AR_AVL);
    desc.seg_32bit = 0;
    install_valid(&desc, AR_DPL3 | AR_TYPE_XRCODE | AR_S | AR_G | AR_AVL);
    desc.seg_32bit = 1;
    desc.contents = 0;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RWDATA | AR_S | AR_DB | AR_G | AR_AVL);
    desc.read_exec_only = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RODATA | AR_S | AR_DB | AR_G | AR_AVL);
    desc.contents = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RODATA_EXPDOWN | AR_S | AR_DB | AR_G | AR_AVL);
    desc.read_exec_only = 0;
    desc.limit_in_pages = 0;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RWDATA_EXPDOWN | AR_S | AR_DB | AR_AVL);
    desc.contents = 3;
    install_valid(&desc, AR_DPL3 | AR_TYPE_XRCODE_CONF | AR_S | AR_DB | AR_AVL);
    desc.read_exec_only = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_XOCODE_CONF | AR_S | AR_DB | AR_AVL);
    desc.read_exec_only = 0;
    desc.contents = 2;
    install_valid(&desc, AR_DPL3 | AR_TYPE_XRCODE | AR_S | AR_DB | AR_AVL);
    desc.read_exec_only = 1;

    #[cfg(target_arch = "x86_64")]
    {
        desc.lm = 1;
        install_valid(&desc, AR_DPL3 | AR_TYPE_XOCODE | AR_S | AR_DB | AR_AVL);
        desc.lm = 0;
    }

    let entry1_okay = install_valid(&desc, AR_DPL3 | AR_TYPE_XOCODE | AR_S | AR_DB | AR_AVL);
    if entry1_okay {
        println!("[RUN]\tTest fork");
        let child = fork();
        if child == 0 {
            nerrs = 0;
            check_valid_segment(desc.entry_number as u16, 1, AR_DPL3 | AR_TYPE_XOCODE | AR_S | AR_DB | AR_AVL, desc.limit, true);
            check_invalid_segment(1, 1);
            exit(if nerrs != 0 { 1 } else { 0 });
        } else {
            let mut status = 0;
            if waitpid(child, &mut status, 0) != child || !WIFEXITED(status) {
                println!("[FAIL]\tChild died");
                nerrs += 1;
            } else if WEXITSTATUS(status) != 0 {
                println!("[FAIL]\tChild failed");
                nerrs += 1;
            } else {
                println!("[OK]\tChild succeeded");
            }
        }

        println!("[RUN]\tTest size");
        let mut i = 0;
        while i < 8192 {
            desc.entry_number = i;
            desc.limit = i;
            if safe_modify_ldt(0x11, &mut desc, size_of::<user_desc>() as c_ulong) != 0 {
                println!("[FAIL]\tFailed to install entry {}", i);
                nerrs += 1;
                break;
            }
            i += 1;
        }
        let mut j = 0;
        while j < i {
            check_valid_segment(j as u16, 1, AR_DPL3 | AR_TYPE_XOCODE | AR_S | AR_DB | AR_AVL, j, false);
            j += 1;
        }
        println!("[DONE]\tSize test");
    } else {
        println!("[SKIP]\tSkipping fork and size tests because we have no LDT");
    }

    /* Test entry_number too high. */
    desc.entry_number = 8192;
    fail_install(&mut desc);

    /* Test deletion and actions mistakeable for deletion. */
    desc = zeroed();
    install_valid(&desc, AR_DPL3 | AR_TYPE_RWDATA | AR_S | AR_P);
    desc.seg_not_present = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RWDATA | AR_S);
    desc.seg_not_present = 0;
    desc.read_exec_only = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RODATA | AR_S | AR_P);
    desc.read_exec_only = 0;
    desc.seg_not_present = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RWDATA | AR_S);
    desc.read_exec_only = 1;
    desc.limit = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RODATA | AR_S);
    desc.limit = 0;
    desc.base_addr = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RODATA | AR_S);
    desc.base_addr = 0;
    install_invalid(&desc, false);
    desc.seg_not_present = 0;
    desc.seg_32bit = 1;
    desc.read_exec_only = 0;
    desc.limit = 0xfffff;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RWDATA | AR_S | AR_P | AR_DB);
    desc.limit_in_pages = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RWDATA | AR_S | AR_P | AR_DB | AR_G);
    desc.read_exec_only = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RODATA | AR_S | AR_P | AR_DB | AR_G);
    desc.contents = 1;
    desc.read_exec_only = 0;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RWDATA_EXPDOWN | AR_S | AR_P | AR_DB | AR_G);
    desc.read_exec_only = 1;
    install_valid(&desc, AR_DPL3 | AR_TYPE_RODATA_EXPDOWN | AR_S | AR_P | AR_DB | AR_G);
    desc.limit = 0;
    install_invalid(&desc, true);
}

unsafe extern "C" fn threadproc(ctx: *mut c_void) -> *mut c_void {
    let mut cpuset: cpu_set_t = zeroed();
    CPU_ZERO(&mut cpuset);
    CPU_SET(1, &mut cpuset);
    if sched_setaffinity(0, size_of::<cpu_set_t>(), &cpuset) != 0 {
        err(1, b"sched_setaffinity to CPU 1\0".as_ptr() as *const c_char); /* should never fail */
    }

    loop {
        syscall(SYS_futex, &raw mut ftx, FUTEX_WAIT, 0, null::<c_void>(), null::<c_void>(), 0);
        while core::ptr::read_volatile(&raw const ftx) != 2 {
            if core::ptr::read_volatile(&raw const ftx) >= 3 {
                return null_mut();
            }
        }

        /* clear LDT entry 0 */
        let desc: user_desc = zeroed();
        if syscall(SYS_modify_ldt, 1, &desc as *const user_desc, size_of::<user_desc>()) != 0 {
            err(1, b"modify_ldt\0".as_ptr() as *const c_char);
        }

        /* If ftx == 2, set it to zero.  If ftx == 100, quit. */
        let mut x: c_uint = (-2i32) as c_uint;
        asm!("lock xadd dword ptr [{ftx}], {x:e}", ftx = in(reg) &raw mut ftx, x = inout(reg) x);
        if x != 2 {
            return null_mut();
        }
    }
}

#[cfg(target_arch = "x86")]
unsafe fn fix_sa_restorer(sig: c_int) {
    let mut ksa: fake_ksigaction = zeroed();
    if syscall(SYS_rt_sigaction, sig, null::<c_void>(), &mut ksa as *mut fake_ksigaction, 8) == 0 {
        /*
         * glibc has a nasty bug: it sometimes writes garbage to
         * sa_restorer.  This interacts quite badly with anything
         * that fiddles with SS because it can trigger legacy
         * stack switching.  Patch it up.  See:
         *
         * https://sourceware.org/bugzilla/show_bug.cgi?id=21269
         */
        if (ksa.sa_flags & SA_RESTORER) == 0 && ksa.sa_restorer.is_some() {
            ksa.sa_restorer = None;
            if syscall(SYS_rt_sigaction, sig, &mut ksa as *mut fake_ksigaction, null::<c_void>(), size_of_val(&ksa.sigset)) != 0 {
                err(1, b"rt_sigaction\0".as_ptr() as *const c_char);
            }
        }
    }
}

#[cfg(not(target_arch = "x86"))]
unsafe fn fix_sa_restorer(sig: c_int) {
    /* 64-bit glibc works fine. */
}

unsafe extern "C" fn sigsegv(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    siglongjmp(&raw mut jmpbuf, 1);
}

unsafe fn do_multicpu_tests() {
    let mut cpuset: cpu_set_t = zeroed();
    let mut thread: pthread_t = 0;
    let mut failures = 0;
    let iters = 5;
    let mut orig_ss: u16;

    CPU_ZERO(&mut cpuset);
    CPU_SET(1, &mut cpuset);
    if sched_setaffinity(0, size_of::<cpu_set_t>(), &cpuset) != 0 {
        println!("[SKIP]\tCannot set affinity to CPU 1");
        return;
    }

    CPU_ZERO(&mut cpuset);
    CPU_SET(0, &mut cpuset);
    if sched_setaffinity(0, size_of::<cpu_set_t>(), &cpuset) != 0 {
        println!("[SKIP]\tCannot set affinity to CPU 0");
        return;
    }

    sethandler(SIGSEGV, sigsegv, 0);
    fix_sa_restorer(SIGSEGV);
    #[cfg(target_arch = "x86")]
    {
        /* True 32-bit kernels send SIGILL instead of SIGSEGV on IRET faults. */
        sethandler(SIGILL, sigsegv, 0);
        fix_sa_restorer(SIGILL);
    }

    println!("[RUN]\tCross-CPU LDT invalidation");
    if pthread_create(&mut thread, null(), threadproc, null_mut()) != 0 {
        err(1, b"pthread_create\0".as_ptr() as *const c_char);
    }

    asm!("mov {0:x}, ss", out(reg) orig_ss);

    for _i in 0..5 {
        if sigsetjmp(&raw mut jmpbuf, 1) != 0 {
            continue;
        }

        /* Make sure the thread is ready after the last test. */
        while core::ptr::read_volatile(&raw const ftx) != 0 {}

        let mut desc: user_desc = zeroed();
        desc.entry_number = 0;
        desc.base_addr = 0;
        desc.limit = 0xfffff;
        desc.seg_32bit = 1;
        desc.contents = 0; /* Data */
        desc.read_exec_only = 0;
        desc.limit_in_pages = 1;
        desc.seg_not_present = 0;
        desc.useable = 0;

        if safe_modify_ldt(0x11, &mut desc, size_of::<user_desc>() as c_ulong) != 0 {
            if errno_get() != ENOSYS {
                err(1, b"modify_ldt\0".as_ptr() as *const c_char);
            }
            println!("[SKIP]\tmodify_ldt unavailable");
            break;
        }

        /* Arm the thread. */
        core::ptr::write_volatile(&raw mut ftx, 1);
        syscall(SYS_futex, &raw mut ftx, FUTEX_WAKE, 0, null::<c_void>(), null::<c_void>(), 0);

        asm!("mov ss, {0:x}", in(reg) 0x7u16);

        /* Go! */
        core::ptr::write_volatile(&raw mut ftx, 2);
        while core::ptr::read_volatile(&raw const ftx) != 0 {}

        /*
         * On success, modify_ldt will segfault us synchronously,
         * and we'll escape via siglongjmp.
         */
        failures += 1;
        asm!("mov ss, {0:x}", in(reg) orig_ss);
    }

    core::ptr::write_volatile(&raw mut ftx, 100); /* Kill the thread. */
    syscall(SYS_futex, &raw mut ftx, FUTEX_WAKE, 0, null::<c_void>(), null::<c_void>(), 0);
    if pthread_join(thread, null_mut()) != 0 {
        err(1, b"pthread_join\0".as_ptr() as *const c_char);
    }

    if failures != 0 {
        println!("[FAIL]\t{} of {} iterations failed", failures, iters);
        nerrs += 1;
    } else {
        println!("[OK]\tAll {} iterations succeeded", iters);
    }
}

unsafe fn finish_exec_test() -> c_int {
    /*
     * Older kernel versions did inherit the LDT on exec() which is
     * wrong because exec() starts from a clean state.
     */
    check_invalid_segment(0, 1);
    if nerrs != 0 { 1 } else { 0 }
}

unsafe fn do_exec_test() {
    println!("[RUN]\tTest exec");
    let mut desc: user_desc = zeroed();
    desc.entry_number = 0;
    desc.base_addr = 0;
    desc.limit = 42;
    desc.seg_32bit = 1;
    desc.contents = 2; /* Code, not conforming */
    desc.read_exec_only = 0;
    desc.limit_in_pages = 0;
    desc.seg_not_present = 0;
    desc.useable = 0;
    install_valid(&desc, AR_DPL3 | AR_TYPE_XRCODE | AR_S | AR_P | AR_DB);

    let child = fork();
    if child == 0 {
        execl(b"/proc/self/exe\0".as_ptr() as *const c_char, b"ldt_gdt_test_exec\0".as_ptr() as *const c_char, null::<c_char>());
        println!("[FAIL]\tCould not exec self");
        exit(1); /* exec failed */
    } else {
        let mut status = 0;
        if waitpid(child, &mut status, 0) != child || !WIFEXITED(status) {
            println!("[FAIL]\tChild died");
            nerrs += 1;
        } else if WEXITSTATUS(status) != 0 {
            println!("[FAIL]\tChild failed");
            nerrs += 1;
        } else {
            println!("[OK]\tChild succeeded");
        }
    }
}

unsafe fn setup_counter_page() {
    let page = mmap(null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE | MAP_32BIT, -1, 0) as *mut c_uint;
    if page as *mut c_void == MAP_FAILED {
        err(1, b"mmap\0".as_ptr() as *const c_char);
    }
    for i in 0..1024 {
        *page.add(i) = i as c_uint;
    }
    counter_page = page;
}

unsafe fn invoke_set_thread_area() -> c_int {
    let mut ret: c_int;
    #[cfg(target_arch = "x86_64")]
    asm!("int 0x80", inlateout("eax") 243_i32 => ret, in("ebx") low_user_desc as u32, inout("m") low_user_desc);
    #[cfg(target_arch = "x86")]
    asm!("int 0x80", inlateout("eax") 243_i32 => ret, in("ebx") low_user_desc, inout("m") low_user_desc);
    ret
}

unsafe fn setup_low_user_desc() {
    low_user_desc = mmap(null_mut(), 2 * size_of::<user_desc>(), PROT_READ | PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE | MAP_32BIT, -1, 0) as *mut user_desc;
    if low_user_desc as *mut c_void == MAP_FAILED {
        err(1, b"mmap\0".as_ptr() as *const c_char);
    }

    (*low_user_desc).entry_number = (-1i32) as c_uint;
    (*low_user_desc).base_addr = counter_page.add(1) as c_ulong as c_uint;
    (*low_user_desc).limit = 0xfffff;
    (*low_user_desc).seg_32bit = 1;
    (*low_user_desc).contents = 0; /* Data, grow-up*/
    (*low_user_desc).read_exec_only = 0;
    (*low_user_desc).limit_in_pages = 1;
    (*low_user_desc).seg_not_present = 0;
    (*low_user_desc).useable = 0;

    if invoke_set_thread_area() == 0 {
        gdt_entry_num = (*low_user_desc).entry_number as c_int;
        println!("[NOTE]\tset_thread_area is available; will use GDT index {}", gdt_entry_num);
    } else {
        println!("[NOTE]\tset_thread_area is unavailable");
    }

    low_user_desc_clear = low_user_desc.add(1);
    (*low_user_desc_clear).entry_number = gdt_entry_num as c_uint;
    (*low_user_desc_clear).read_exec_only = 1;
    (*low_user_desc_clear).seg_not_present = 1;
}

unsafe fn test_gdt_invalidation() {
    if gdt_entry_num == 0 {
        return; /* 64-bit only system -- we can't use set_thread_area */
    }

    let mut prev_sel: u16;
    let mut sel: u16;
    let mut eax: c_uint;
    let mut result: &str;
    #[cfg(target_arch = "x86_64")]
    let mut saved_base: c_ulong = 0;
    #[cfg(target_arch = "x86_64")]
    let mut new_base: c_ulong = 0;

    /* Test DS */
    invoke_set_thread_area();
    eax = 243;
    sel = ((gdt_entry_num << 3) | 3) as u16;
    #[cfg(target_arch = "x86_64")]
    asm!("mov {prev:x}, ds", "mov ds, {sel:x}", "mov ebx, {arg1:e}", "int 0x80", "mov {sel:x}, ds", "mov ds, {prev:x}",
         prev = lateout(reg) prev_sel, sel = inout(reg) sel, inout("eax") eax, arg1 = in(reg) low_user_desc_clear as u32);
    #[cfg(target_arch = "x86")]
    asm!("mov {prev:x}, ds", "mov ds, {sel:x}", "push ebx", "mov ebx, {arg1:e}", "int 0x80", "pop ebx", "mov {sel:x}, ds", "mov ds, {prev:x}",
         prev = lateout(reg) prev_sel, sel = inout(reg) sel, inout("eax") eax, arg1 = in(reg) low_user_desc_clear as u32);
    if sel != 0 {
        result = "FAIL";
        nerrs += 1;
    } else {
        result = "OK";
    }
    println!("[{}]\tInvalidate DS with set_thread_area: new DS = 0x{:x}", result, sel);

    /* Test ES */
    invoke_set_thread_area();
    eax = 243;
    sel = ((gdt_entry_num << 3) | 3) as u16;
    #[cfg(target_arch = "x86_64")]
    asm!("mov {prev:x}, es", "mov es, {sel:x}", "mov ebx, {arg1:e}", "int 0x80", "mov {sel:x}, es", "mov es, {prev:x}",
         prev = lateout(reg) prev_sel, sel = inout(reg) sel, inout("eax") eax, arg1 = in(reg) low_user_desc_clear as u32);
    #[cfg(target_arch = "x86")]
    asm!("mov {prev:x}, es", "mov es, {sel:x}", "push ebx", "mov ebx, {arg1:e}", "int 0x80", "pop ebx", "mov {sel:x}, es", "mov es, {prev:x}",
         prev = lateout(reg) prev_sel, sel = inout(reg) sel, inout("eax") eax, arg1 = in(reg) low_user_desc_clear as u32);
    if sel != 0 {
        result = "FAIL";
        nerrs += 1;
    } else {
        result = "OK";
    }
    println!("[{}]\tInvalidate ES with set_thread_area: new ES = 0x{:x}", result, sel);

    /* Test FS */
    invoke_set_thread_area();
    eax = 243;
    sel = ((gdt_entry_num << 3) | 3) as u16;
    #[cfg(target_arch = "x86_64")]
    {
        syscall(SYS_arch_prctl, ARCH_GET_FS, &mut saved_base as *mut c_ulong);
    }
    asm!("mov {prev:x}, fs", "mov fs, {sel:x}", "mov ebx, {arg1:e}", "int 0x80", "mov {sel:x}, fs",
         prev = lateout(reg) prev_sel, sel = inout(reg) sel, inout("eax") eax, arg1 = in(reg) low_user_desc_clear as u32);
    #[cfg(target_arch = "x86_64")]
    {
        syscall(SYS_arch_prctl, ARCH_GET_FS, &mut new_base as *mut c_ulong);
    }
    /* Restore FS/BASE for glibc */
    asm!("mov fs, {0:x}", in(reg) prev_sel);
    #[cfg(target_arch = "x86_64")]
    if saved_base != 0 {
        syscall(SYS_arch_prctl, ARCH_SET_FS, saved_base);
    }
    if sel != 0 {
        result = "FAIL";
        nerrs += 1;
    } else {
        result = "OK";
    }
    println!("[{}]\tInvalidate FS with set_thread_area: new FS = 0x{:x}", result, sel);
    #[cfg(target_arch = "x86_64")]
    if sel == 0 && new_base != 0 {
        nerrs += 1;
        println!("[FAIL]\tNew FSBASE was 0x{:x}", new_base);
    } else {
        println!("[OK]\tNew FSBASE was zero");
    }

    /* Test GS */
    invoke_set_thread_area();
    eax = 243;
    sel = ((gdt_entry_num << 3) | 3) as u16;
    #[cfg(target_arch = "x86_64")]
    {
        syscall(SYS_arch_prctl, ARCH_GET_GS, &mut saved_base as *mut c_ulong);
    }
    asm!("mov {prev:x}, gs", "mov gs, {sel:x}", "mov ebx, {arg1:e}", "int 0x80", "mov {sel:x}, gs",
         prev = lateout(reg) prev_sel, sel = inout(reg) sel, inout("eax") eax, arg1 = in(reg) low_user_desc_clear as u32);
    #[cfg(target_arch = "x86_64")]
    {
        syscall(SYS_arch_prctl, ARCH_GET_GS, &mut new_base as *mut c_ulong);
    }
    /* Restore GS/BASE for glibc */
    asm!("mov gs, {0:x}", in(reg) prev_sel);
    #[cfg(target_arch = "x86_64")]
    if saved_base != 0 {
        syscall(SYS_arch_prctl, ARCH_SET_GS, saved_base);
    }
    if sel != 0 {
        result = "FAIL";
        nerrs += 1;
    } else {
        result = "OK";
    }
    println!("[{}]\tInvalidate GS with set_thread_area: new GS = 0x{:x}", result, sel);
    #[cfg(target_arch = "x86_64")]
    if sel == 0 && new_base != 0 {
        nerrs += 1;
        println!("[FAIL]\tNew GSBASE was 0x{:x}", new_base);
    } else {
        println!("[OK]\tNew GSBASE was zero");
    }
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if argc == 1 && strcmp(*argv, b"ldt_gdt_test_exec\0".as_ptr() as *const c_char) == 0 {
        return finish_exec_test();
    }

    setup_counter_page();
    setup_low_user_desc();
    do_simple_tests();
    do_multicpu_tests();
    do_exec_test();
    test_gdt_invalidation();
    if nerrs != 0 { 1 } else { 0 }
}

fn main() {
    unsafe {
        unsafe extern "C" {
            static mut environ: *mut *mut c_char;
        }
        let _ = environ;
        /* Rust main does not receive argc/argv directly without runtime hooks.
         * The translated implementation entry remains main_impl above.
         */
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
