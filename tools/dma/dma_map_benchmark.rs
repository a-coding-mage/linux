// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 HiSilicon Limited.
 */

// C dependencies removed from executable Rust:
// <fcntl.h>, <stdio.h>, <stdlib.h>, <string.h>, <unistd.h>,
// <sys/ioctl.h>, <sys/mman.h>, <linux/map_benchmark.h>

use std::ffi::c_void;
use std::os::raw::{c_char, c_double, c_int, c_long, c_uint, c_ulong};

const NSEC_PER_MSEC: c_long = 1000000;

// Values supplied by <fcntl.h>.
const O_RDWR: c_int = 0o2;

// Values supplied by <linux/map_benchmark.h>.
const DMA_MAP_BIDIRECTIONAL: c_int = 0;
const DMA_MAP_TO_DEVICE: c_int = 1;
const DMA_MAP_FROM_DEVICE: c_int = 2;
const DMA_MAP_BENCH_SINGLE_MODE: c_int = 0;
const DMA_MAP_BENCH_SG_MODE: c_int = 1;
const DMA_MAP_BENCH_MODE_MAX: c_int = 2;

// Numeric macro values are external UAPI details not present in the isolated
// source file.
unsafe extern "C" {
    static DMA_MAP_MAX_THREADS: c_int;
    static DMA_MAP_MAX_SECONDS: c_int;
    static DMA_MAP_MAX_TRANS_DELAY: c_long;
    static DMA_MAP_BENCHMARK: c_int;
}

#[repr(C)]
struct map_benchmark {
    avg_map_100ns: u64,
    map_stddev: u64,
    avg_unmap_100ns: u64,
    unmap_stddev: u64,
    threads: c_uint,
    seconds: c_uint,
    node: c_int,
    dma_bits: c_uint,
    dma_dir: c_uint,
    dma_trans_ns: c_uint,
    granule: c_uint,
    map_mode: c_uint,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut stderr: *mut c_void;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

static DIRECTIONS_0: &[u8] = b"BIDIRECTIONAL\0";
static DIRECTIONS_1: &[u8] = b"TO_DEVICE\0";
static DIRECTIONS_2: &[u8] = b"FROM_DEVICE\0";

static mut directions: [*const c_char; 3] = [
    DIRECTIONS_0.as_ptr() as *const c_char,
    DIRECTIONS_1.as_ptr() as *const c_char,
    DIRECTIONS_2.as_ptr() as *const c_char,
];

static MODE_0: &[u8] = b"SINGLE_MODE\0";
static MODE_1: &[u8] = b"SG_MODE\0";

static mut mode: [*const c_char; 2] = [
    MODE_0.as_ptr() as *const c_char,
    MODE_1.as_ptr() as *const c_char,
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        let mut map: map_benchmark = std::mem::zeroed();
        let mut fd: c_int;
        let mut opt: c_int;
        /* default single thread, run 20 seconds on NUMA_NO_NODE */
        let mut threads: c_int = 1;
        let mut seconds: c_int = 20;
        let mut node: c_int = -1;
        /* default single map mode */
        let mut map_mode: c_int = DMA_MAP_BENCH_SINGLE_MODE;
        /* default dma mask 32bit, bidirectional DMA */
        let mut bits: c_int = 32;
        let mut xdelay: c_int = 0;
        let mut dir: c_int = DMA_MAP_BIDIRECTIONAL;
        /* default granule 1 PAGESIZE */
        let mut granule: c_int = 1;

        let cmd: c_int = DMA_MAP_BENCHMARK;

        loop {
            opt = getopt(argc, argv, c"t:s:n:b:d:x:g:m:".as_ptr());
            if opt == -1 {
                break;
            }

            match opt as u8 as char {
                't' => {
                    threads = atoi(optarg);
                }
                's' => {
                    seconds = atoi(optarg);
                }
                'n' => {
                    node = atoi(optarg);
                }
                'b' => {
                    bits = atoi(optarg);
                }
                'd' => {
                    dir = atoi(optarg);
                }
                'x' => {
                    xdelay = atoi(optarg);
                }
                'g' => {
                    granule = atoi(optarg);
                }
                'm' => {
                    map_mode = atoi(optarg);
                }
                _ => {
                    return -1;
                }
            }
        }

        if map_mode < 0 || map_mode >= DMA_MAP_BENCH_MODE_MAX {
            fprintf(
                stderr,
                c"invalid map mode, SINGLE_MODE:%d, SG_MODE: %d\n".as_ptr(),
                DMA_MAP_BENCH_SINGLE_MODE,
                DMA_MAP_BENCH_SG_MODE,
            );
            exit(1);
        }

        if threads <= 0 || threads > DMA_MAP_MAX_THREADS {
            fprintf(
                stderr,
                c"invalid number of threads, must be in 1-%d\n".as_ptr(),
                DMA_MAP_MAX_THREADS,
            );
            exit(1);
        }

        if seconds <= 0 || seconds > DMA_MAP_MAX_SECONDS {
            fprintf(
                stderr,
                c"invalid number of seconds, must be in 1-%d\n".as_ptr(),
                DMA_MAP_MAX_SECONDS,
            );
            exit(1);
        }

        if xdelay < 0 || (xdelay as c_long) > DMA_MAP_MAX_TRANS_DELAY {
            fprintf(
                stderr,
                c"invalid transmit delay, must be in 0-%ld\n".as_ptr(),
                DMA_MAP_MAX_TRANS_DELAY,
            );
            exit(1);
        }

        /* suppose the mininum DMA zone is 1MB in the world */
        if bits < 20 || bits > 64 {
            fprintf(stderr, c"invalid dma mask bit, must be in 20-64\n".as_ptr());
            exit(1);
        }

        if dir != DMA_MAP_BIDIRECTIONAL && dir != DMA_MAP_TO_DEVICE && dir != DMA_MAP_FROM_DEVICE {
            fprintf(stderr, c"invalid dma direction\n".as_ptr());
            exit(1);
        }

        if granule < 1 || granule > 1024 {
            fprintf(stderr, c"invalid granule size\n".as_ptr());
            exit(1);
        }

        fd = open(c"/sys/kernel/debug/dma_map_benchmark".as_ptr(), O_RDWR);
        if fd == -1 {
            perror(c"open".as_ptr());
            exit(1);
        }

        map.seconds = seconds as c_uint;
        map.threads = threads as c_uint;
        map.node = node;
        map.dma_bits = bits as c_uint;
        map.dma_dir = dir as c_uint;
        map.dma_trans_ns = xdelay as c_uint;
        map.granule = granule as c_uint;
        map.map_mode = map_mode as c_uint;

        if ioctl(fd, cmd as c_ulong, &mut map as *mut map_benchmark) != 0 {
            perror(c"ioctl".as_ptr());
            exit(1);
        }

        printf(
            c"dma mapping benchmark(%s): threads:%d seconds:%d node:%d dir:%s granule:%d\n"
                .as_ptr(),
            mode[map_mode as usize],
            threads,
            seconds,
            node,
            directions[dir as usize],
            granule,
        );
        printf(
            c"average map latency(us):%.1f standard deviation:%.1f\n".as_ptr(),
            map.avg_map_100ns as c_double / 10.0,
            map.map_stddev as c_double / 10.0,
        );
        printf(
            c"average unmap latency(us):%.1f standard deviation:%.1f\n".as_ptr(),
            map.avg_unmap_100ns as c_double / 10.0,
            map.unmap_stddev as c_double / 10.0,
        );

        return 0;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
