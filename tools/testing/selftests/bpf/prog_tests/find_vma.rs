// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

use core::ffi::{c_char, c_int, c_long, c_void};

// C dependencies removed from executable Rust:
// <test_progs.h>, <sys/types.h>, <unistd.h>,
// "find_vma.skel.h", "find_vma_fail1.skel.h", "find_vma_fail2.skel.h".

type __u64 = u64;
type uintptr_t = usize;
type size_t = usize;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct find_vma_bss {
    pub found_vm_exec: c_int,
    pub target_pid: c_int,
    pub addr: __u64,
    pub d_iname: [c_char; 0],
}

#[repr(C)]
pub struct find_vma_data {
    pub find_addr_ret: c_int,
    pub find_zero_ret: c_int,
}

#[repr(C)]
pub struct find_vma_progs {
    pub handle_pe: *mut bpf_program,
}

#[repr(C)]
pub struct find_vma {
    pub bss: *mut find_vma_bss,
    pub data: *mut find_vma_data,
    pub progs: find_vma_progs,
}

#[repr(C)]
pub struct find_vma_fail1 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct find_vma_fail2 {
    _private: [u8; 0],
}

// Supplied by Linux/libc/test_progs/libbpf headers in the original C build.
#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period_or_freq: u64,
    pub flags: u64,
}

const PERF_TYPE_SOFTWARE: u32 = 1;
const PERF_COUNT_SW_CPU_CLOCK: u64 = 0;
const PERF_FLAG_FD_CLOEXEC: c_long = 8;
const ENOENT: c_int = 2;
const EOPNOTSUPP: c_int = 95;
const EBUSY: c_int = 16;

// Architecture-specific syscall number from <unistd.h>/<sys/syscall.h> in C.
const __NR_perf_event_open: c_long = 298;

unsafe extern "C" {
    static mut errno: c_int;

    fn syscall(num: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> c_int;
    fn getpgid(pid: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn bpf_program__attach_perf_event(prog: *mut bpf_program, pfd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn find_vma__open_and_load() -> *mut find_vma;
    fn find_vma__attach(skel: *mut find_vma) -> c_int;
    fn find_vma__destroy(skel: *mut find_vma);

    fn find_vma_fail1__open_and_load() -> *mut find_vma_fail1;
    fn find_vma_fail1__destroy(skel: *mut find_vma_fail1);
    fn find_vma_fail2__open_and_load() -> *mut find_vma_fail2;
    fn find_vma_fail2__destroy(skel: *mut find_vma_fail2);

    fn test__skip();
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

unsafe fn test_and_reset_skel(
    skel: *mut find_vma,
    expected_find_zero_ret: c_int,
    need_test: bool,
) {
    if need_test {
        ASSERT_EQ((*(*skel).bss).found_vm_exec, 1, c"found_vm_exec".as_ptr());
        ASSERT_EQ((*(*skel).data).find_addr_ret, 0, c"find_addr_ret".as_ptr());
        ASSERT_EQ(
            (*(*skel).data).find_zero_ret,
            expected_find_zero_ret,
            c"find_zero_ret".as_ptr(),
        );
        ASSERT_OK_PTR(
            strstr((*(*skel).bss).d_iname.as_ptr(), c"test_progs".as_ptr()) as *const c_void,
            c"find_test_progs".as_ptr(),
        );
    }

    (*(*skel).bss).found_vm_exec = 0;
    (*(*skel).data).find_addr_ret = -1;
    (*(*skel).data).find_zero_ret = -1;
    (*(*skel).bss).d_iname[0] = 0;
}

unsafe fn open_pe() -> c_int {
    let mut attr: perf_event_attr = core::mem::zeroed();
    let pfd: c_int;

    /* create perf event */
    attr.size = core::mem::size_of_val(&attr) as u32;
    attr.type_ = PERF_TYPE_SOFTWARE;
    attr.config = PERF_COUNT_SW_CPU_CLOCK;
    attr.flags |= 1 << 10; /* freq = 1 */
    attr.sample_period_or_freq = 1000; /* sample_freq */
    pfd = syscall(
        __NR_perf_event_open,
        &mut attr as *mut perf_event_attr,
        0,
        -1,
        -1,
        PERF_FLAG_FD_CLOEXEC,
    ) as c_int;

    if pfd >= 0 { pfd } else { -errno }
}

unsafe fn find_vma_pe_condition(skel: *mut find_vma) -> bool {
    (*(*skel).bss).found_vm_exec == 0
        || (*(*skel).data).find_addr_ret != 0
        || (*(*skel).data).find_zero_ret == -1
        || strcmp((*(*skel).bss).d_iname.as_ptr(), c"test_progs".as_ptr()) != 0
}

unsafe fn test_find_vma_pe(skel: *mut find_vma) {
    let mut link: *mut bpf_link = core::ptr::null_mut();
    let mut j: c_int = 0;
    let pfd: c_int;
    let mut i: c_int;
    const one_bn: c_int = 1000000000;

    pfd = open_pe();
    if pfd < 0 {
        if pfd == -ENOENT || pfd == -EOPNOTSUPP {
            printf(
                c"%s:SKIP:no PERF_COUNT_HW_CPU_CYCLES\n".as_ptr(),
                c"test_find_vma_pe".as_ptr(),
            );
            test__skip();
            bpf_link__destroy(link);
            close(pfd);
            return;
        }
        if !ASSERT_GE(pfd, 0, c"perf_event_open".as_ptr()) {
            bpf_link__destroy(link);
            close(pfd);
            return;
        }
    }

    link = bpf_program__attach_perf_event((*skel).progs.handle_pe, pfd);
    if !ASSERT_OK_PTR(link as *const c_void, c"attach_perf_event".as_ptr()) {
        bpf_link__destroy(link);
        close(pfd);
        return;
    }

    i = 0;
    while i < one_bn && find_vma_pe_condition(skel) {
        j = j.wrapping_add(1);
        i += 1;
    }
    core::ptr::read_volatile(&j);

    test_and_reset_skel(skel, -EBUSY /* in nmi, irq_work is busy */, i == one_bn);
    bpf_link__destroy(link);
    close(pfd);
}

unsafe fn test_find_vma_kprobe(skel: *mut find_vma) {
    let err: c_int;

    err = find_vma__attach(skel);
    if !ASSERT_OK(err, c"get_branch_snapshot__attach".as_ptr()) {
        return;
    }

    getpgid((*(*skel).bss).target_pid);
    test_and_reset_skel(
        skel,
        -ENOENT, /* could not find vma for ptr 0 */
        true,
    );
}

unsafe fn test_illegal_write_vma() {
    let skel: *mut find_vma_fail1;

    skel = find_vma_fail1__open_and_load();
    if !ASSERT_ERR_PTR(skel as *const c_void, c"find_vma_fail1__open_and_load".as_ptr()) {
        find_vma_fail1__destroy(skel);
    }
}

unsafe fn test_illegal_write_task() {
    let skel: *mut find_vma_fail2;

    skel = find_vma_fail2__open_and_load();
    if !ASSERT_ERR_PTR(skel as *const c_void, c"find_vma_fail2__open_and_load".as_ptr()) {
        find_vma_fail2__destroy(skel);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_find_vma() {
    let skel: *mut find_vma;

    skel = find_vma__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"find_vma__open_and_load".as_ptr()) {
        return;
    }

    (*(*skel).bss).target_pid = getpid();
    (*(*skel).bss).addr = test_find_vma_pe as uintptr_t as __u64;

    test_find_vma_pe(skel);
    test_find_vma_kprobe(skel);

    find_vma__destroy(skel);
    test_illegal_write_vma();
    test_illegal_write_task();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
