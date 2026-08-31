// SPDX-License-Identifier: GPL-2.0
//
// C dependencies translated as Rust FFI declarations:
// linux/perf_event.h, cpuid.h, errno.h, stdio.h, stdlib.h, string.h,
// sys/ioctl.h, sys/syscall.h, unistd.h

use std::ffi::c_char;
use std::mem;
use std::process;

const EXIT_FAILURE: i32 = 1;
const SYS_PERF_EVENT_OPEN: isize = 298;
const PERF_TYPE_RAW: u32 = 4;
const PERF_EVENT_IOC_ENABLE: u64 = 0x2400;
const PERF_EVENT_IOC_DISABLE: u64 = 0x2401;
const PERF_EVENT_IOC_RESET: u64 = 0x2403;

#[repr(C)]
struct PerfEventAttr {
    type_: u32,
    size: u32,
    config: u64,
    sample_period_or_freq: u64,
    sample_type: u64,
    read_format: u64,
    flags: u64,
}

impl PerfEventAttr {
    const DISABLED: u64 = 1 << 0;
    const EXCLUDE_USER: u64 = 1 << 4;
    const EXCLUDE_HV: u64 = 1 << 6;
}

unsafe extern "C" {
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> i32;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> i32;
    fn syscall(number: isize, ...) -> isize;
    fn ioctl(fd: i32, request: u64, ...) -> i32;
    fn sleep(seconds: u32) -> u32;
    fn read(fd: i32, buf: *mut std::ffi::c_void, count: usize) -> isize;

    static mut stderr: *mut FILE;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

fn main() {
    let mut ret_attr: PerfEventAttr = unsafe { mem::zeroed() };
    let mut mret_attr: PerfEventAttr = unsafe { mem::zeroed() };
    let mut count_rets: i64 = 0;
    let mut count_rets_mispred: i64 = 0;
    let rrets_fd: i32;
    let mrrets_fd: i32;
    let cpuid1_eax: u32;
    let mut b: u32;
    let mut c: u32;
    let mut d: u32;

    unsafe {
        core::arch::asm!(
            "push rbx",
            "cpuid",
            "mov {b:e}, ebx",
            "pop rbx",
            inlateout("eax") 1_u32 => cpuid1_eax,
            b = lateout(reg) b,
            lateout("ecx") c,
            lateout("edx") d,
            options(preserves_flags),
        );
    }

    let _ = (b, c, d);

    if cpuid1_eax < 0x00800f00 || cpuid1_eax > 0x00afffff {
        unsafe {
            fprintf(
                stderr,
                c"This needs to run on a Zen[1-4] machine (CPUID(1).EAX: 0x%x). Exiting...\n"
                    .as_ptr(),
                cpuid1_eax,
            );
        }
        process::exit(EXIT_FAILURE);
    }

    ret_attr.type_ = PERF_TYPE_RAW;
    mret_attr.type_ = PERF_TYPE_RAW;
    ret_attr.size = mem::size_of::<PerfEventAttr>() as u32;
    mret_attr.size = mem::size_of::<PerfEventAttr>() as u32;
    ret_attr.config = 0xc8;
    mret_attr.config = 0xc9;
    ret_attr.flags |= PerfEventAttr::DISABLED;
    mret_attr.flags |= PerfEventAttr::DISABLED;
    ret_attr.flags |= PerfEventAttr::EXCLUDE_USER;
    mret_attr.flags |= PerfEventAttr::EXCLUDE_USER;
    ret_attr.flags |= PerfEventAttr::EXCLUDE_HV;
    mret_attr.flags |= PerfEventAttr::EXCLUDE_HV;

    unsafe {
        rrets_fd = syscall(
            SYS_PERF_EVENT_OPEN,
            &ret_attr as *const PerfEventAttr,
            0,
            -1,
            -1,
            0,
        ) as i32;
    }
    if rrets_fd == -1 {
        unsafe {
            perror(c"opening retired RETs fd".as_ptr());
        }
        process::exit(EXIT_FAILURE);
    }

    unsafe {
        mrrets_fd = syscall(
            SYS_PERF_EVENT_OPEN,
            &mret_attr as *const PerfEventAttr,
            0,
            -1,
            -1,
            0,
        ) as i32;
    }
    if mrrets_fd == -1 {
        unsafe {
            perror(c"opening retired mispredicted RETs fd".as_ptr());
        }
        process::exit(EXIT_FAILURE);
    }

    unsafe {
        ioctl(rrets_fd, PERF_EVENT_IOC_RESET, 0);
        ioctl(mrrets_fd, PERF_EVENT_IOC_RESET, 0);

        ioctl(rrets_fd, PERF_EVENT_IOC_ENABLE, 0);
        ioctl(mrrets_fd, PERF_EVENT_IOC_ENABLE, 0);

        printf(c"Sleeping for 10 seconds\n".as_ptr());
        sleep(10);

        ioctl(rrets_fd, PERF_EVENT_IOC_DISABLE, 0);
        ioctl(mrrets_fd, PERF_EVENT_IOC_DISABLE, 0);

        read(
            rrets_fd,
            &mut count_rets as *mut i64 as *mut std::ffi::c_void,
            mem::size_of::<i64>(),
        );
        read(
            mrrets_fd,
            &mut count_rets_mispred as *mut i64 as *mut std::ffi::c_void,
            mem::size_of::<i64>(),
        );

        printf(
            c"RETs: (%lld retired <-> %lld mispredicted)\n".as_ptr(),
            count_rets,
            count_rets_mispred,
        );
        printf(c"SRSO Safe-RET mitigation works correctly if both counts are almost equal.\n".as_ptr());
    }
}
