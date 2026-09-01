// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/*
 * uprobe.c
 *
 * uprobe benchmarks
 *
 *  Copyright (C) 2023, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type u64 = u64;
type s64 = i64;

const LOOPS_DEFAULT: c_int = 1000;
static mut loops: c_int = LOOPS_DEFAULT;

#[repr(C)]
enum bench_uprobe {
    BENCH_UPROBE__BASELINE,
    BENCH_UPROBE__EMPTY,
    BENCH_UPROBE__TRACE_PRINTK,
    BENCH_UPROBE__EMPTY_RET,
    BENCH_UPROBE__TRACE_PRINTK_RET,
}

#[repr(C)]
struct option {
    _private: [u8; 0],
}

extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut bench_format: c_int;

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    fn exit(status: c_int) -> !;
    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn usleep(usec: useconds_t) -> c_int;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

type clockid_t = c_int;
type useconds_t = c_uint;

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

type time_t = c_long;

const CLOCK_REALTIME: clockid_t = 0;
const NSEC_PER_SEC: u64 = 1_000_000_000;
const NSEC_PER_USEC: u64 = 1_000;
const USEC_PER_MSEC: useconds_t = 1_000;

const BENCH_FORMAT_DEFAULT: c_int = 0;
const BENCH_FORMAT_SIMPLE: c_int = 1;

extern "C" {
    static options: [option; 0];
}

static bench_uprobe_usage_0: &[u8] = b"perf bench uprobe <options>\0";
static bench_uprobe_usage_1: *const c_char = core::ptr::null();
static bench_uprobe_usage: [*const c_char; 2] = [
    bench_uprobe_usage_0.as_ptr() as *const c_char,
    bench_uprobe_usage_1,
];

/*
 * Original C has a build-time HAVE_BPF_SKEL branch that includes
 * "bench/bpf_skel/bench_uprobe.skel.h", defines bench_uprobe__attach_uprobe(),
 * stores a static struct bench_uprobe_bpf *skel, opens/loads/destroys the BPF
 * skeleton, and attaches the selected program to libc.so.6:usleep.
 *
 * The non-HAVE_BPF_SKEL fallback is translated below, matching the file-local
 * behavior when that dependency is unavailable in this isolated translation.
 */
unsafe fn bench_uprobe__setup_bpf_skel(_bench: bench_uprobe) -> c_int {
    0
}

unsafe fn bench_uprobe__teardown_bpf_skel() {}

unsafe fn bench_uprobe_format__default_fprintf(
    name: *const c_char,
    unit: *const c_char,
    diff: u64,
    fp: *mut FILE,
) -> c_int {
    static mut baseline: u64 = 0;
    static mut previous: u64 = 0;

    let diff_to_baseline: s64 = diff.wrapping_sub(baseline) as s64;
    let diff_to_previous: s64 = diff.wrapping_sub(previous) as s64;
    let mut printed: c_int = fprintf(
        fp,
        b"# Executed %'d %s calls\n\0".as_ptr() as *const c_char,
        loops,
        name,
    );

    printed += fprintf(
        fp,
        b" %14s: %'lu %ss\0".as_ptr() as *const c_char,
        b"Total time\0".as_ptr() as *const c_char,
        diff,
        unit,
    );

    if baseline != 0 {
        printed += fprintf(
            fp,
            b" %s%'ld to baseline\0".as_ptr() as *const c_char,
            if diff_to_baseline > 0 {
                b"+\0".as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
            diff_to_baseline,
        );

        if previous != baseline {
            fprintf(
                stdout,
                b" %s%'ld to previous\0".as_ptr() as *const c_char,
                if diff_to_previous > 0 {
                    b"+\0".as_ptr() as *const c_char
                } else {
                    b"\0".as_ptr() as *const c_char
                },
                diff_to_previous,
            );
        }
    }

    printed += fprintf(
        fp,
        b"\n\n %'.3f %ss/op\0".as_ptr() as *const c_char,
        diff as f64 / loops as f64,
        unit,
    );

    if baseline != 0 {
        printed += fprintf(
            fp,
            b" %'.3f %ss/op to baseline\0".as_ptr() as *const c_char,
            diff_to_baseline as f64 / loops as f64,
            unit,
        );

        if previous != baseline {
            printed += fprintf(
                fp,
                b" %'.3f %ss/op to previous\0".as_ptr() as *const c_char,
                diff_to_previous as f64 / loops as f64,
                unit,
            );
        }
    } else {
        baseline = diff;
    }

    fputc('\n' as c_int, fp);

    previous = diff;

    printed + 1
}

unsafe fn bench_uprobe(argc: c_int, argv: *const *const c_char, bench: bench_uprobe) -> c_int {
    let name: *const c_char = b"usleep(1000)\0".as_ptr() as *const c_char;
    let unit: *const c_char = b"usec\0".as_ptr() as *const c_char;
    let mut start: timespec = core::mem::zeroed();
    let mut end: timespec = core::mem::zeroed();
    let mut diff: u64;
    let mut i: c_int;
    let argc = parse_options(argc, argv, options.as_ptr(), bench_uprobe_usage.as_ptr(), 0);

    let _ = argc;

    if bench as c_int != bench_uprobe::BENCH_UPROBE__BASELINE as c_int
        && bench_uprobe__setup_bpf_skel(core::mem::transmute(bench as c_int)) < 0
    {
        return 0;
    }

    clock_gettime(CLOCK_REALTIME, &mut start);

    i = 0;
    while i < loops {
        usleep(USEC_PER_MSEC);
        i += 1;
    }

    clock_gettime(CLOCK_REALTIME, &mut end);

    diff = (end.tv_sec as u64)
        .wrapping_mul(NSEC_PER_SEC)
        .wrapping_add(end.tv_nsec as u64)
        .wrapping_sub(
            (start.tv_sec as u64)
                .wrapping_mul(NSEC_PER_SEC)
                .wrapping_add(start.tv_nsec as u64),
        );
    diff /= NSEC_PER_USEC;

    match bench_format {
        BENCH_FORMAT_DEFAULT => {
            bench_uprobe_format__default_fprintf(name, unit, diff, stdout);
        }

        BENCH_FORMAT_SIMPLE => {
            printf(b"%lu\n\0".as_ptr() as *const c_char, diff);
        }

        _ => {
            /* reaching here is something of a disaster */
            fprintf(
                stderr,
                b"Unknown format:%d\n\0".as_ptr() as *const c_char,
                bench_format,
            );
            exit(1);
        }
    }

    if bench as c_int != bench_uprobe::BENCH_UPROBE__BASELINE as c_int {
        bench_uprobe__teardown_bpf_skel();
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn bench_uprobe_baseline(argc: c_int, argv: *const *const c_char) -> c_int {
    bench_uprobe(argc, argv, bench_uprobe::BENCH_UPROBE__BASELINE)
}

#[no_mangle]
pub unsafe extern "C" fn bench_uprobe_empty(argc: c_int, argv: *const *const c_char) -> c_int {
    bench_uprobe(argc, argv, bench_uprobe::BENCH_UPROBE__EMPTY)
}

#[no_mangle]
pub unsafe extern "C" fn bench_uprobe_trace_printk(argc: c_int, argv: *const *const c_char) -> c_int {
    bench_uprobe(argc, argv, bench_uprobe::BENCH_UPROBE__TRACE_PRINTK)
}

#[no_mangle]
pub unsafe extern "C" fn bench_uprobe_empty_ret(argc: c_int, argv: *const *const c_char) -> c_int {
    bench_uprobe(argc, argv, bench_uprobe::BENCH_UPROBE__EMPTY_RET)
}

#[no_mangle]
pub unsafe extern "C" fn bench_uprobe_trace_printk_ret(
    argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    bench_uprobe(argc, argv, bench_uprobe::BENCH_UPROBE__TRACE_PRINTK_RET)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
