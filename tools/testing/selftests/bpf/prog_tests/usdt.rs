// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */
// C dependencies translated as external/file-local requirements:
// <test_progs.h>, ../sdt.h, test_usdt.skel.h, test_urandom_usdt.skel.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uchar, c_ulong, c_void};
use core::ptr;

type __u8 = u8;
type __u64 = u64;
type size_t = usize;
type FILE = c_void;

const _SDT_HAS_SEMAPHORES: c_int = 1;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const E2BIG: c_int = 7;

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_usdt_opts {
    usdt_cookie: __u64,
}

#[repr(C)]
struct test_usdt__bss {
    my_pid: c_int,
    usdt0_called: c_int,
    usdt3_called: c_int,
    usdt12_called: c_int,
    usdt0_cookie: __u64,
    usdt0_arg_cnt: c_int,
    usdt0_arg_ret: c_int,
    usdt0_arg_size: c_int,
    usdt3_cookie: __u64,
    usdt3_arg_cnt: c_int,
    usdt3_arg_rets: [c_int; 3],
    usdt3_args: [__u64; 3],
    usdt3_arg_sizes: [c_int; 3],
    usdt12_cookie: __u64,
    usdt12_arg_cnt: c_int,
    usdt12_args: [__u64; 12],
    usdt12_arg_sizes: [c_int; 12],
    usdt_sib_called: c_int,
    usdt_sib_cookie: __u64,
    usdt_sib_arg_cnt: c_int,
    usdt_sib_arg: c_int,
    usdt_sib_arg_ret: c_int,
    usdt_sib_arg_size: c_int,
    expected_ip: c_ulong,
    executed: c_int,
    expected_arg: [c_int; 3],
    expected_pid: c_int,
    arg_total: c_int,
    arg_bad: c_int,
    arg_last: [c_int; 3],
    usdt_100_called: c_int,
    usdt_100_sum: c_int,
}

#[repr(C)]
struct test_usdt__links {
    usdt0: *mut bpf_link,
    usdt3: *mut bpf_link,
    usdt12: *mut bpf_link,
    usdt_sib: *mut bpf_link,
    usdt_executed: *mut bpf_link,
    usdt_check_arg: *mut bpf_link,
    usdt_100: *mut bpf_link,
}

#[repr(C)]
struct test_usdt__progs {
    usdt0: *mut bpf_program,
    usdt3: *mut bpf_program,
    usdt12: *mut bpf_program,
    usdt_sib: *mut bpf_program,
    usdt_executed: *mut bpf_program,
    usdt_check_arg: *mut bpf_program,
    usdt_100: *mut bpf_program,
}

#[repr(C)]
struct test_usdt {
    bss: *mut test_usdt__bss,
    links: test_usdt__links,
    progs: test_usdt__progs,
}

#[repr(C)]
struct test_urandom_usdt__bss {
    urand_pid: c_int,
    urand_read_without_sema_call_cnt: c_int,
    urand_read_without_sema_buf_sz_sum: c_int,
    urand_read_with_sema_call_cnt: c_int,
    urand_read_with_sema_buf_sz_sum: c_int,
    urandlib_read_without_sema_call_cnt: c_int,
    urandlib_read_without_sema_buf_sz_sum: c_int,
    urandlib_read_with_sema_call_cnt: c_int,
    urandlib_read_with_sema_buf_sz_sum: c_int,
}

#[repr(C)]
struct test_urandom_usdt__links {
    urand_read_without_sema: *mut bpf_link,
    urand_read_with_sema: *mut bpf_link,
    urandlib_read_without_sema: *mut bpf_link,
    urandlib_read_with_sema: *mut bpf_link,
}

#[repr(C)]
struct test_urandom_usdt__progs {
    urand_read_without_sema: *mut bpf_program,
    urand_read_with_sema: *mut bpf_program,
    urandlib_read_without_sema: *mut bpf_program,
    urandlib_read_with_sema: *mut bpf_program,
}

#[repr(C)]
struct test_urandom_usdt {
    bss: *mut test_urandom_usdt__bss,
    links: test_urandom_usdt__links,
    progs: test_urandom_usdt__progs,
}

unsafe extern "C" {
    fn lets_test_this(_: c_int) -> c_int;
    fn getpid() -> c_int;
    fn memcmp(_: *const c_void, _: *const c_void, _: size_t) -> c_int;
    fn popen(_: *const c_char, _: *const c_char) -> *mut FILE;
    fn pclose(_: *mut FILE) -> c_int;
    fn fscanf(_: *mut FILE, _: *const c_char, ...) -> c_int;
    static mut errno: c_int;

    fn test_usdt__open_and_load() -> *mut test_usdt;
    fn test_usdt__attach(_: *mut test_usdt) -> c_int;
    fn test_usdt__destroy(_: *mut test_usdt);
    fn test_urandom_usdt__open_and_load() -> *mut test_urandom_usdt;
    fn test_urandom_usdt__attach(_: *mut test_urandom_usdt) -> c_int;
    fn test_urandom_usdt__destroy(_: *mut test_urandom_usdt);

    fn bpf_program__attach_usdt(
        prog: *mut bpf_program,
        pid: c_int,
        binary_path: *const c_char,
        usdt_provider: *const c_char,
        usdt_name: *const c_char,
        opts: *const bpf_usdt_opts,
    ) -> *mut bpf_link;
    fn bpf_link__destroy(_: *mut bpf_link);

    fn test__start_subtest(_: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_NULL<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char);
    fn ASSERT_MEMEQ(actual: *const c_void, expected: *const c_void, cnt: size_t, name: *const c_char);

    fn STAP_PROBE(provider: *const c_char, name: *const c_char);
    fn STAP_PROBE1(provider: *const c_char, name: *const c_char, arg1: c_int);
    fn STAP_PROBE3(provider: *const c_char, name: *const c_char, arg1: c_int, arg2: c_long, arg3: *const __u64);
    fn STAP_PROBE12(
        provider: *const c_char,
        name: *const c_char,
        arg1: c_int,
        arg2: c_int,
        arg3: c_long,
        arg4: c_long,
        arg5: c_int,
        arg6: c_long,
        arg7: __u64,
        arg8: *const __u64,
        arg9: c_int,
        arg10: c_short,
        arg11: c_short,
        arg12: c_char,
    );
}

type c_short = i16;

static mut idx: c_int = 2;
static mut bla: __u64 = 0xFEDCBA9876543210u64;
static mut nums: [c_short; 4] = [-1, -2, -3, -4];

#[repr(C)]
struct t1_struct {
    x: c_int,
    y: c_char,
}

static mut t1: t1_struct = t1_struct { x: 1, y: -127 };

// SEC(name) was __attribute__((section(name), used)) in C.
#[unsafe(link_section = ".probes")]
#[used]
static mut test_usdt0_semaphore: u16 = 0;
#[unsafe(link_section = ".probes")]
#[used]
static mut test_usdt3_semaphore: u16 = 0;
#[unsafe(link_section = ".probes")]
#[used]
static mut test_usdt12_semaphore: u16 = 0;

#[inline(always)]
unsafe fn trigger_func(x: c_int) {
    let y: c_long = 42;

    if core::ptr::read_volatile(core::ptr::addr_of!(test_usdt0_semaphore)) != 0 {
        STAP_PROBE(c"test".as_ptr(), c"usdt0".as_ptr());
    }
    if core::ptr::read_volatile(core::ptr::addr_of!(test_usdt3_semaphore)) != 0 {
        STAP_PROBE3(c"test".as_ptr(), c"usdt3".as_ptr(), x, y, core::ptr::addr_of!(bla));
    }
    if core::ptr::read_volatile(core::ptr::addr_of!(test_usdt12_semaphore)) != 0 {
        STAP_PROBE12(
            c"test".as_ptr(),
            c"usdt12".as_ptr(),
            x,
            x + 1,
            y,
            x as c_long + y,
            5,
            y / 7,
            core::ptr::read_volatile(core::ptr::addr_of!(bla)),
            core::ptr::addr_of!(bla),
            -9,
            core::ptr::read_volatile(core::ptr::addr_of!(nums[x as usize])),
            core::ptr::read_volatile(core::ptr::addr_of!(nums[core::ptr::read_volatile(core::ptr::addr_of!(idx)) as usize])),
            core::ptr::read_volatile(core::ptr::addr_of!(t1.y)),
        );
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
static mut array: [c_short; 4] = [-1, -2, -3, -4];

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[unsafe(link_section = ".probes")]
#[used]
static mut test_usdt_sib_semaphore: u16 = 0;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
unsafe fn trigger_sib_spec() {
    /*
     * Force SIB addressing with inline assembly.
     *
     * The C USDT_SIB_ARG_SPEC macro expands to -2@(%%rdx,%%rax,2) on
     * x86_64 and -2@(%%edx,%%eax,2) on i386, embedded by STAP_PROBE_ASM.
     */
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!(
        ".pushsection .note.stapsdt,\"?\",\"note\"",
        ".balign 4",
        ".4byte 992f-991f, 994f-993f, 3",
        "991: .asciz \"stapsdt\"",
        "992: .balign 4",
        "993: .8byte 990f",
        ".8byte 0",
        ".8byte 0",
        ".asciz \"test\"",
        ".asciz \"usdt_sib\"",
        ".asciz \"-2@(%rdx,%rax,2)\"",
        "994: .balign 4",
        ".popsection",
        "990:",
        in("rdx") core::ptr::addr_of!(array),
        in("rax") 0usize,
        options(nostack, preserves_flags)
    );
    #[cfg(target_arch = "x86")]
    core::arch::asm!(
        ".pushsection .note.stapsdt,\"?\",\"note\"",
        ".balign 4",
        ".4byte 992f-991f, 994f-993f, 3",
        "991: .asciz \"stapsdt\"",
        "992: .balign 4",
        "993: .4byte 990f",
        ".4byte 0",
        ".4byte 0",
        ".asciz \"test\"",
        ".asciz \"usdt_sib\"",
        ".asciz \"-2@(%edx,%eax,2)\"",
        "994: .balign 4",
        ".popsection",
        "990:",
        in("edx") core::ptr::addr_of!(array),
        in("eax") 0usize,
        options(nostack, preserves_flags)
    );
}

unsafe fn subtest_basic_usdt(optimized: bool) {
    let mut opts = bpf_usdt_opts { usdt_cookie: 0 };
    let mut skel: *mut test_usdt;
    let bss: *mut test_usdt__bss;
    let mut err: c_int;
    let mut i: c_int;
    let mut called: c_int;
    const expected_cookie: __u64 = 0xcafedeadbeeffeed;

    skel = test_usdt__open_and_load();
    if !ASSERT_OK_PTR(skel, c"skel_open".as_ptr()) {
        return;
    }

    bss = (*skel).bss;
    (*bss).my_pid = getpid();

    err = test_usdt__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        test_usdt__destroy(skel);
        return;
    }

    /* usdt0 won't be auto-attached */
    opts.usdt_cookie = expected_cookie;
    (*skel).links.usdt0 = bpf_program__attach_usdt((*skel).progs.usdt0, 0, c"/proc/self/exe".as_ptr(), c"test".as_ptr(), c"usdt0".as_ptr(), &opts);
    if !ASSERT_OK_PTR((*skel).links.usdt0, c"usdt0_link".as_ptr()) {
        test_usdt__destroy(skel);
        return;
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        opts.usdt_cookie = expected_cookie;
        (*skel).links.usdt_sib = bpf_program__attach_usdt((*skel).progs.usdt_sib, 0, c"/proc/self/exe".as_ptr(), c"test".as_ptr(), c"usdt_sib".as_ptr(), &opts);
        if !ASSERT_OK_PTR((*skel).links.usdt_sib, c"usdt_sib_link".as_ptr()) {
            test_usdt__destroy(skel);
            return;
        }
    }

    called = {
        trigger_func(1);
        if optimized {
            trigger_func(1);
        }
        if optimized { 2 } else { 1 }
    };

    ASSERT_EQ((*bss).usdt0_called, called, c"usdt0_called".as_ptr());
    ASSERT_EQ((*bss).usdt3_called, called, c"usdt3_called".as_ptr());
    ASSERT_EQ((*bss).usdt12_called, called, c"usdt12_called".as_ptr());

    ASSERT_EQ((*bss).usdt0_cookie, expected_cookie, c"usdt0_cookie".as_ptr());
    ASSERT_EQ((*bss).usdt0_arg_cnt, 0, c"usdt0_arg_cnt".as_ptr());
    ASSERT_EQ((*bss).usdt0_arg_ret, -ENOENT, c"usdt0_arg_ret".as_ptr());
    ASSERT_EQ((*bss).usdt0_arg_size, -ENOENT, c"usdt0_arg_size".as_ptr());

    /* auto-attached usdt3 gets default zero cookie value */
    ASSERT_EQ((*bss).usdt3_cookie, 0, c"usdt3_cookie".as_ptr());
    ASSERT_EQ((*bss).usdt3_arg_cnt, 3, c"usdt3_arg_cnt".as_ptr());

    ASSERT_EQ((*bss).usdt3_arg_rets[0], 0, c"usdt3_arg1_ret".as_ptr());
    ASSERT_EQ((*bss).usdt3_arg_rets[1], 0, c"usdt3_arg2_ret".as_ptr());
    ASSERT_EQ((*bss).usdt3_arg_rets[2], 0, c"usdt3_arg3_ret".as_ptr());
    ASSERT_EQ((*bss).usdt3_args[0], 1, c"usdt3_arg1".as_ptr());
    ASSERT_EQ((*bss).usdt3_args[1], 42, c"usdt3_arg2".as_ptr());
    ASSERT_EQ((*bss).usdt3_args[2], core::ptr::addr_of!(bla) as usize as __u64, c"usdt3_arg3".as_ptr());
    ASSERT_EQ((*bss).usdt3_arg_sizes[0], 4, c"usdt3_arg1_size".as_ptr());
    ASSERT_EQ((*bss).usdt3_arg_sizes[1], 8, c"usdt3_arg2_size".as_ptr());
    ASSERT_EQ((*bss).usdt3_arg_sizes[2], 8, c"usdt3_arg3_size".as_ptr());

    /* auto-attached usdt12 gets default zero cookie value */
    ASSERT_EQ((*bss).usdt12_cookie, 0, c"usdt12_cookie".as_ptr());
    ASSERT_EQ((*bss).usdt12_arg_cnt, 12, c"usdt12_arg_cnt".as_ptr());

    ASSERT_EQ((*bss).usdt12_args[0], 1, c"usdt12_arg1".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[1], 1 + 1, c"usdt12_arg2".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[2], 42, c"usdt12_arg3".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[3], 42 + 1, c"usdt12_arg4".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[4], 5, c"usdt12_arg5".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[5], 42 / 7, c"usdt12_arg6".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[6], core::ptr::read_volatile(core::ptr::addr_of!(bla)), c"usdt12_arg7".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[7], core::ptr::addr_of!(bla) as usize as __u64, c"usdt12_arg8".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[8], -9i64 as __u64, c"usdt12_arg9".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[9], core::ptr::read_volatile(core::ptr::addr_of!(nums[1])) as __u64, c"usdt12_arg10".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[10], core::ptr::read_volatile(core::ptr::addr_of!(nums[core::ptr::read_volatile(core::ptr::addr_of!(idx)) as usize])) as __u64, c"usdt12_arg11".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[11], core::ptr::read_volatile(core::ptr::addr_of!(t1.y)) as __u64, c"usdt12_arg12".as_ptr());

    let usdt12_expected_arg_sizes: [c_int; 12] = [4, 4, 8, 8, 4, 8, 8, 8, 4, 2, 2, 1];

    i = 0;
    while i < 12 {
        ASSERT_EQ((*bss).usdt12_arg_sizes[i as usize], usdt12_expected_arg_sizes[i as usize], c"usdt12_arg_size".as_ptr());
        i += 1;
    }

    /* trigger_func() is marked __always_inline, so USDT invocations will be
     * inlined in two different places, meaning that each USDT will have
     * at least 2 different places to be attached to. This verifies that
     * bpf_program__attach_usdt() handles this properly and attaches to
     * all possible places of USDT invocation.
     */
    called += {
        trigger_func(2);
        if optimized {
            trigger_func(2);
        }
        if optimized { 2 } else { 1 }
    };

    ASSERT_EQ((*bss).usdt0_called, called, c"usdt0_called".as_ptr());
    ASSERT_EQ((*bss).usdt3_called, called, c"usdt3_called".as_ptr());
    ASSERT_EQ((*bss).usdt12_called, called, c"usdt12_called".as_ptr());

    /* only check values that depend on trigger_func()'s input value */
    ASSERT_EQ((*bss).usdt3_args[0], 2, c"usdt3_arg1".as_ptr());

    ASSERT_EQ((*bss).usdt12_args[0], 2, c"usdt12_arg1".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[1], 2 + 1, c"usdt12_arg2".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[3], 42 + 2, c"usdt12_arg4".as_ptr());
    ASSERT_EQ((*bss).usdt12_args[9], core::ptr::read_volatile(core::ptr::addr_of!(nums[2])) as __u64, c"usdt12_arg10".as_ptr());

    /* detach and re-attach usdt3 */
    bpf_link__destroy((*skel).links.usdt3);

    opts.usdt_cookie = 0xBADC00C51E;
    (*skel).links.usdt3 = bpf_program__attach_usdt((*skel).progs.usdt3, -1, c"/proc/self/exe".as_ptr(), c"test".as_ptr(), c"usdt3".as_ptr(), &opts);
    if !ASSERT_OK_PTR((*skel).links.usdt3, c"usdt3_reattach".as_ptr()) {
        test_usdt__destroy(skel);
        return;
    }

    called += {
        trigger_func(3);
        if optimized {
            trigger_func(3);
        }
        if optimized { 2 } else { 1 }
    };

    ASSERT_EQ((*bss).usdt3_called, called, c"usdt3_called".as_ptr());
    /* this time usdt3 has custom cookie */
    ASSERT_EQ((*bss).usdt3_cookie, 0xBADC00C51E, c"usdt3_cookie".as_ptr());
    ASSERT_EQ((*bss).usdt3_arg_cnt, 3, c"usdt3_arg_cnt".as_ptr());

    ASSERT_EQ((*bss).usdt3_arg_rets[0], 0, c"usdt3_arg1_ret".as_ptr());
    ASSERT_EQ((*bss).usdt3_arg_rets[1], 0, c"usdt3_arg2_ret".as_ptr());
    ASSERT_EQ((*bss).usdt3_arg_rets[2], 0, c"usdt3_arg3_ret".as_ptr());
    ASSERT_EQ((*bss).usdt3_args[0], 3, c"usdt3_arg1".as_ptr());
    ASSERT_EQ((*bss).usdt3_args[1], 42, c"usdt3_arg2".as_ptr());
    ASSERT_EQ((*bss).usdt3_args[2], core::ptr::addr_of!(bla) as usize as __u64, c"usdt3_arg3".as_ptr());

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        trigger_sib_spec();
        ASSERT_EQ((*bss).usdt_sib_called, 1, c"usdt_sib_called".as_ptr());
        ASSERT_EQ((*bss).usdt_sib_cookie, expected_cookie, c"usdt_sib_cookie".as_ptr());
        ASSERT_EQ((*bss).usdt_sib_arg_cnt, 1, c"usdt_sib_arg_cnt".as_ptr());
        ASSERT_EQ((*bss).usdt_sib_arg, core::ptr::read_volatile(core::ptr::addr_of!(nums[0])) as c_int, c"usdt_sib_arg".as_ptr());
        ASSERT_EQ((*bss).usdt_sib_arg_ret, 0, c"usdt_sib_arg_ret".as_ptr());
        ASSERT_EQ((*bss).usdt_sib_arg_size, core::mem::size_of_val(&nums[0]) as c_int, c"usdt_sib_arg_size".as_ptr());
    }

    test_usdt__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn usdt_1();
    fn usdt_2();
    fn usdt_red_zone_trigger();
}

#[cfg(target_arch = "x86_64")]
static mut nop1: [c_uchar; 1] = [0x90];
#[cfg(target_arch = "x86_64")]
static mut nop1_nop10_combo: [c_uchar; 11] = [0x90, 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];

#[cfg(target_arch = "x86_64")]
unsafe fn find_instr(fn_: *mut c_void, instr: *mut c_uchar, cnt: size_t) -> *mut c_void {
    let mut i: c_int = 0;

    while i < 10 {
        if memcmp(instr as *const c_void, (fn_ as *mut c_uchar).add(i as usize) as *const c_void, cnt) == 0 {
            return (fn_ as *mut c_uchar).add(i as usize) as *mut c_void;
        }
        i += 1;
    }
    ptr::null_mut()
}

#[cfg(target_arch = "x86_64")]
unsafe fn subtest_optimized_attach() {
    let skel: *mut test_usdt;
    let mut addr_1: *mut __u8;
    let addr_2: *mut __u8;

    /* usdt_1 USDT probe has single nop instruction */
    addr_1 = find_instr(usdt_1 as *mut c_void, core::ptr::addr_of_mut!(nop1_nop10_combo) as *mut c_uchar, 11) as *mut __u8;
    if !ASSERT_NULL(addr_1, c"usdt_1_find_nop1_nop10_combo".as_ptr()) {
        return;
    }

    addr_1 = find_instr(usdt_1 as *mut c_void, core::ptr::addr_of_mut!(nop1) as *mut c_uchar, 1) as *mut __u8;
    if !ASSERT_OK_PTR(addr_1, c"usdt_1_find_nop1".as_ptr()) {
        return;
    }

    /* usdt_2 USDT probe has nop,nop10 instructions combo */
    addr_2 = find_instr(usdt_2 as *mut c_void, core::ptr::addr_of_mut!(nop1_nop10_combo) as *mut c_uchar, 11) as *mut __u8;
    if !ASSERT_OK_PTR(addr_2, c"usdt_2_find_nop1_nop10_combo".as_ptr()) {
        return;
    }

    skel = test_usdt__open_and_load();
    if !ASSERT_OK_PTR(skel, c"test_usdt__open_and_load".as_ptr()) {
        return;
    }

    (*(*skel).bss).expected_ip = addr_1 as c_ulong;

    /*
     * Attach program on top of usdt_1 which is single nop probe,
     * so the probe won't get optimized.
     */
    (*skel).links.usdt_executed = bpf_program__attach_usdt((*skel).progs.usdt_executed, 0, c"/proc/self/exe".as_ptr(), c"optimized_attach".as_ptr(), c"usdt_1".as_ptr(), ptr::null());
    if !ASSERT_OK_PTR((*skel).links.usdt_executed, c"bpf_program__attach_usdt".as_ptr()) {
        test_usdt__destroy(skel);
        return;
    }

    usdt_1();
    usdt_1();

    /* int3 is on addr_1 address */
    ASSERT_EQ(*addr_1, 0xcc, c"int3".as_ptr());
    ASSERT_EQ((*(*skel).bss).executed, 2, c"executed".as_ptr());

    bpf_link__destroy((*skel).links.usdt_executed);

    /* we expect the nop10 ip */
    (*(*skel).bss).expected_ip = addr_2 as c_ulong + 1;

    /*
     * Attach program on top of usdt_2 which is probe defined on top
     * of nop1,nop10 combo, so the probe gets optimized on top of nop10.
     */
    (*skel).links.usdt_executed = bpf_program__attach_usdt((*skel).progs.usdt_executed, 0, c"/proc/self/exe".as_ptr(), c"optimized_attach".as_ptr(), c"usdt_2".as_ptr(), ptr::null());
    if !ASSERT_OK_PTR((*skel).links.usdt_executed, c"bpf_program__attach_usdt".as_ptr()) {
        test_usdt__destroy(skel);
        return;
    }

    usdt_2();
    usdt_2();

    /* nop stays on addr_2 address */
    ASSERT_EQ(*addr_2, 0x90, c"nop".as_ptr());

    /*
     * lea -0x80(%rsp), %rsp
     * call ...
     */
    static expected: [c_uchar; 6] = [0x48, 0x8d, 0x64, 0x24, 0x80, 0xe8];

    ASSERT_MEMEQ(addr_2.add(1) as *const c_void, expected.as_ptr() as *const c_void, expected.len(), c"lea_and_call".as_ptr());
    ASSERT_EQ((*(*skel).bss).executed, 4, c"executed".as_ptr());

    test_usdt__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn subtest_optimized_red_zone() {
    let skel: *mut test_usdt;
    let mut i: c_int;

    skel = test_usdt__open_and_load();
    if !ASSERT_OK_PTR(skel, c"open_and_load".as_ptr()) {
        return;
    }

    (*(*skel).bss).expected_arg[0] = 0xDEADBEEF_u32 as c_int;
    (*(*skel).bss).expected_arg[1] = 0xCAFEBABE_u32 as c_int;
    (*(*skel).bss).expected_arg[2] = 0xFEEDFACE_u32 as c_int;
    (*(*skel).bss).expected_pid = getpid();

    (*skel).links.usdt_check_arg = bpf_program__attach_usdt(
        (*skel).progs.usdt_check_arg,
        0,
        c"/proc/self/exe".as_ptr(),
        c"optimized_attach".as_ptr(),
        c"usdt_red_zone".as_ptr(),
        ptr::null(),
    );
    if !ASSERT_OK_PTR((*skel).links.usdt_check_arg, c"attach_usdt_red_zone".as_ptr()) {
        test_usdt__destroy(skel);
        return;
    }

    i = 0;
    while i < 10 {
        usdt_red_zone_trigger();
        i += 1;
    }

    ASSERT_EQ((*(*skel).bss).arg_total, 10, c"arg_total".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg_bad, 0, c"arg_bad".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg_last[0], 0xDEADBEEF_u32 as c_int, c"arg_last_1".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg_last[1], 0xCAFEBABE_u32 as c_int, c"arg_last_2".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg_last[2], 0xFEEDFACE_u32 as c_int, c"arg_last_3".as_ptr());

    test_usdt__destroy(skel);
}

#[unsafe(link_section = ".probes")]
#[used]
static mut test_usdt_100_semaphore: u16 = 0;
#[unsafe(link_section = ".probes")]
#[used]
static mut test_usdt_300_semaphore: u16 = 0;
#[unsafe(link_section = ".probes")]
#[used]
static mut test_usdt_400_semaphore: u16 = 0;

#[inline(always)]
unsafe fn f100(x: c_int) {
    STAP_PROBE1(c"test".as_ptr(), c"usdt_100".as_ptr(), x);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trigger_100_usdts() {
    let mut x = 0;
    while x < 100 {
        f100(x);
        x += 1;
    }
}

#[inline(always)]
unsafe fn f300(x: c_int) {
    STAP_PROBE1(c"test".as_ptr(), c"usdt_300".as_ptr(), x);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trigger_300_usdts() {
    let mut x = 0;
    while x < 300 {
        f300(x);
        x += 1;
    }
}

#[inline(always)]
unsafe fn f400(_x: c_int) {
    STAP_PROBE1(c"test".as_ptr(), c"usdt_400".as_ptr(), 400);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trigger_400_usdts() {
    let mut x = 0;
    while x < 400 {
        f400(x);
        x += 1;
    }
}

unsafe fn subtest_multispec_usdt() {
    let opts = bpf_usdt_opts { usdt_cookie: 0 };
    let skel: *mut test_usdt;
    let bss: *mut test_usdt__bss;
    let mut err: c_int;
    let mut i: c_int;

    skel = test_usdt__open_and_load();
    if !ASSERT_OK_PTR(skel, c"skel_open".as_ptr()) {
        return;
    }

    bss = (*skel).bss;
    (*bss).my_pid = getpid();

    err = test_usdt__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        test_usdt__destroy(skel);
        return;
    }

    trigger_100_usdts();

    ASSERT_EQ((*bss).usdt_100_called, 100, c"usdt_100_called".as_ptr());
    ASSERT_EQ((*bss).usdt_100_sum, 99 * 100 / 2, c"usdt_100_sum".as_ptr());

    i = 0;
    while i < 2 {
        bpf_link__destroy((*skel).links.usdt_100);

        (*skel).links.usdt_100 = bpf_program__attach_usdt((*skel).progs.usdt_100, -1, c"/proc/self/exe".as_ptr(), c"test".as_ptr(), c"usdt_100".as_ptr(), ptr::null());
        if !ASSERT_OK_PTR((*skel).links.usdt_100, c"usdt_100_reattach".as_ptr()) {
            test_usdt__destroy(skel);
            return;
        }

        (*bss).usdt_100_sum = 0;
        trigger_100_usdts();

        ASSERT_EQ((*bss).usdt_100_called, (i + 1) * 100 + 100, c"usdt_100_called".as_ptr());
        ASSERT_EQ((*bss).usdt_100_sum, 99 * 100 / 2, c"usdt_100_sum".as_ptr());
        i += 1;
    }

    trigger_300_usdts();

    bpf_link__destroy((*skel).links.usdt_100);

    (*bss).usdt_100_called = 0;
    (*bss).usdt_100_sum = 0;

    #[cfg(not(all(target_arch = "aarch64", feature = "clang")))]
    {
        /* we'll reuse usdt_100 BPF program for usdt_300 test */
        (*skel).links.usdt_100 = bpf_program__attach_usdt((*skel).progs.usdt_100, -1, c"/proc/self/exe".as_ptr(), c"test".as_ptr(), c"usdt_300".as_ptr(), ptr::null());
        err = -errno;
        if !ASSERT_ERR_PTR((*skel).links.usdt_100, c"usdt_300_bad_attach".as_ptr()) {
            test_usdt__destroy(skel);
            return;
        }
        ASSERT_EQ(err, -E2BIG, c"usdt_300_attach_err".as_ptr());

        /* let's check that there are no "dangling" BPF programs attached due
         * to partial success of the above test:usdt_300 attachment
         */
        f300(777); /* this is 301st instance of usdt_300 */

        ASSERT_EQ((*bss).usdt_100_called, 0, c"usdt_301_called".as_ptr());
        ASSERT_EQ((*bss).usdt_100_sum, 0, c"usdt_301_sum".as_ptr());
    }

    (*skel).links.usdt_100 = bpf_program__attach_usdt((*skel).progs.usdt_100, -1, c"/proc/self/exe".as_ptr(), c"test".as_ptr(), c"usdt_400".as_ptr(), ptr::null());
    if !ASSERT_OK_PTR((*skel).links.usdt_100, c"usdt_400_attach".as_ptr()) {
        test_usdt__destroy(skel);
        return;
    }

    trigger_400_usdts();

    ASSERT_EQ((*bss).usdt_100_called, 400, c"usdt_400_called".as_ptr());
    ASSERT_EQ((*bss).usdt_100_sum, 400 * 400, c"usdt_400_sum".as_ptr());

    let _ = opts;
    test_usdt__destroy(skel);
}

unsafe fn urand_spawn(pid: *mut c_int) -> *mut FILE {
    let f: *mut FILE;

    /* urandom_read's stdout is wired into f */
    f = popen(c"./urandom_read 1 report-pid".as_ptr(), c"r".as_ptr());
    if f.is_null() {
        return ptr::null_mut();
    }

    if fscanf(f, c"%d".as_ptr(), pid) != 1 {
        pclose(f);
        errno = EINVAL;
        return ptr::null_mut();
    }

    f
}

unsafe fn urand_trigger(urand_pipe: *mut *mut FILE) -> c_int {
    let exit_code: c_int;

    /* pclose() waits for child process to exit and returns their exit code */
    exit_code = pclose(*urand_pipe);
    *urand_pipe = ptr::null_mut();

    exit_code
}

unsafe fn subtest_urandom_usdt(auto_attach: bool) {
    let skel: *mut test_urandom_usdt;
    let bss: *mut test_urandom_usdt__bss;
    let mut l: *mut bpf_link;
    let mut urand_pipe: *mut FILE = ptr::null_mut();
    let mut err: c_int;
    let mut urand_pid: c_int = 0;

    skel = test_urandom_usdt__open_and_load();
    if !ASSERT_OK_PTR(skel, c"skel_open".as_ptr()) {
        return;
    }

    urand_pipe = urand_spawn(&mut urand_pid);
    if !ASSERT_OK_PTR(urand_pipe, c"urand_spawn".as_ptr()) {
        test_urandom_usdt__destroy(skel);
        return;
    }

    bss = (*skel).bss;
    (*bss).urand_pid = urand_pid;

    if auto_attach {
        err = test_urandom_usdt__attach(skel);
        if !ASSERT_OK(err, c"skel_auto_attach".as_ptr()) {
            if !urand_pipe.is_null() {
                pclose(urand_pipe);
            }
            test_urandom_usdt__destroy(skel);
            return;
        }
    } else {
        l = bpf_program__attach_usdt((*skel).progs.urand_read_without_sema, urand_pid, c"./urandom_read".as_ptr(), c"urand".as_ptr(), c"read_without_sema".as_ptr(), ptr::null());
        if !ASSERT_OK_PTR(l, c"urand_without_sema_attach".as_ptr()) {
            if !urand_pipe.is_null() {
                pclose(urand_pipe);
            }
            test_urandom_usdt__destroy(skel);
            return;
        }
        (*skel).links.urand_read_without_sema = l;

        l = bpf_program__attach_usdt((*skel).progs.urand_read_with_sema, urand_pid, c"./urandom_read".as_ptr(), c"urand".as_ptr(), c"read_with_sema".as_ptr(), ptr::null());
        if !ASSERT_OK_PTR(l, c"urand_with_sema_attach".as_ptr()) {
            if !urand_pipe.is_null() {
                pclose(urand_pipe);
            }
            test_urandom_usdt__destroy(skel);
            return;
        }
        (*skel).links.urand_read_with_sema = l;

        l = bpf_program__attach_usdt((*skel).progs.urandlib_read_without_sema, urand_pid, c"./liburandom_read.so".as_ptr(), c"urandlib".as_ptr(), c"read_without_sema".as_ptr(), ptr::null());
        if !ASSERT_OK_PTR(l, c"urandlib_without_sema_attach".as_ptr()) {
            if !urand_pipe.is_null() {
                pclose(urand_pipe);
            }
            test_urandom_usdt__destroy(skel);
            return;
        }
        (*skel).links.urandlib_read_without_sema = l;

        l = bpf_program__attach_usdt((*skel).progs.urandlib_read_with_sema, urand_pid, c"./liburandom_read.so".as_ptr(), c"urandlib".as_ptr(), c"read_with_sema".as_ptr(), ptr::null());
        if !ASSERT_OK_PTR(l, c"urandlib_with_sema_attach".as_ptr()) {
            if !urand_pipe.is_null() {
                pclose(urand_pipe);
            }
            test_urandom_usdt__destroy(skel);
            return;
        }
        (*skel).links.urandlib_read_with_sema = l;
    }

    /* trigger urandom_read USDTs */
    ASSERT_OK(urand_trigger(&mut urand_pipe), c"urand_exit_code".as_ptr());

    ASSERT_EQ((*bss).urand_read_without_sema_call_cnt, 1, c"urand_wo_sema_cnt".as_ptr());
    ASSERT_EQ((*bss).urand_read_without_sema_buf_sz_sum, 256, c"urand_wo_sema_sum".as_ptr());

    ASSERT_EQ((*bss).urand_read_with_sema_call_cnt, 1, c"urand_w_sema_cnt".as_ptr());
    ASSERT_EQ((*bss).urand_read_with_sema_buf_sz_sum, 256, c"urand_w_sema_sum".as_ptr());

    ASSERT_EQ((*bss).urandlib_read_without_sema_call_cnt, 1, c"urandlib_wo_sema_cnt".as_ptr());
    ASSERT_EQ((*bss).urandlib_read_without_sema_buf_sz_sum, 256, c"urandlib_wo_sema_sum".as_ptr());

    ASSERT_EQ((*bss).urandlib_read_with_sema_call_cnt, 1, c"urandlib_w_sema_cnt".as_ptr());
    ASSERT_EQ((*bss).urandlib_read_with_sema_buf_sz_sum, 256, c"urandlib_w_sema_sum".as_ptr());

    if !urand_pipe.is_null() {
        pclose(urand_pipe);
    }
    test_urandom_usdt__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_usdt() {
    if test__start_subtest(c"basic".as_ptr()) {
        subtest_basic_usdt(false);
    }
    #[cfg(target_arch = "x86_64")]
    {
        if test__start_subtest(c"basic_optimized".as_ptr()) {
            subtest_basic_usdt(true);
        }
        if test__start_subtest(c"optimized_attach".as_ptr()) {
            subtest_optimized_attach();
        }
        if test__start_subtest(c"optimized_red_zone".as_ptr()) {
            subtest_optimized_red_zone();
        }
    }
    if test__start_subtest(c"multispec".as_ptr()) {
        subtest_multispec_usdt();
    }
    if test__start_subtest(c"urand_auto_attach".as_ptr()) {
        subtest_urandom_usdt(true /* auto_attach */);
    }
    if test__start_subtest(c"urand_pid_attach".as_ptr()) {
        subtest_urandom_usdt(false /* auto_attach */);
    }
}
