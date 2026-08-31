// SPDX-License-Identifier: GPL-2.0
// C dependencies translated from includes:
// sched.h, sys/syscall.h, sys/mman.h, sys/ioctl.h, sys/utsname.h, string.h,
// arch-tests.h, linux/perf_event.h, tests/tests.h, perf-sys.h, pmu.h, pmus.h,
// debug.h, util.h, strbuf.h, util/env.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

static mut page_size: c_int = 0;

const PERF_MMAP_DATA_PAGES: c_long = 32;

unsafe fn PERF_MMAP_DATA_SIZE() -> c_ulong {
    (PERF_MMAP_DATA_PAGES as c_ulong).wrapping_mul(page_size as c_ulong)
}

unsafe fn PERF_MMAP_DATA_MASK() -> c_ulong {
    PERF_MMAP_DATA_SIZE().wrapping_sub(1)
}

unsafe fn PERF_MMAP_TOTAL_PAGES() -> c_ulong {
    (PERF_MMAP_DATA_PAGES + 1) as c_ulong
}

unsafe fn PERF_MMAP_TOTAL_SIZE() -> c_ulong {
    PERF_MMAP_TOTAL_PAGES().wrapping_mul(page_size as c_ulong)
}

#[inline(always)]
unsafe fn rmb() {
    core::arch::asm!("lfence", options(nostack, preserves_flags));
}

const FD_ERROR: c_int = 0;
const FD_SUCCESS: c_int = 1;

const IBS_FETCH: c_int = 0;
const IBS_OP: c_int = 1;

#[repr(C)]
pub struct perf_pmu {
    pub type_: c_uint,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: c_uint,
    pub size: c_uint,
    pub config: c_ulonglong,
    pub sample_period: c_ulong,
    pub sample_type: c_ulonglong,
    pub disabled: c_ulonglong,
    pub freq: c_ulonglong,
}

#[repr(C)]
pub struct perf_event_mmap_page {
    pub data_head: c_ulong,
    pub data_tail: c_ulong,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: c_uint,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strbuf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

static mut fetch_pmu: *mut perf_pmu = ptr::null_mut();
static mut op_pmu: *mut perf_pmu = ptr::null_mut();
static mut perf_event_max_sample_rate: c_uint = 0;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PROT_EXEC: c_int = 0x4;
const MAP_PRIVATE: c_int = 0x02;
const MAP_SHARED: c_int = 0x01;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const PERF_RECORD_SAMPLE: c_uint = 9;
const PERF_SAMPLE_PERIOD: c_ulonglong = 1 << 8;
const PERF_EVENT_IOC_ENABLE: c_ulong = 9216;
const PERF_EVENT_IOC_DISABLE: c_ulong = 9217;
const PERF_EVENT_IOC_RESET: c_ulong = 9219;
const PERF_EVENT_IOC_PERIOD: c_ulong = 1074275332;
const __NR_perf_event_open: c_long = 298;
const _SC_PAGESIZE: c_int = 30;
const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = -2;
const PATH_MAX: usize = 4096;
const EOF: c_int = -1;

unsafe extern "C" {
    fn mmap(addr: *mut c_void, length: c_ulong, prot: c_int, flags: c_int, fd: c_int, offset: c_long) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: c_ulong) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn system(command: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
    fn syscall(num: c_long, ...) -> c_long;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> c_int;
    fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const c_void) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn uname(buf: *mut utsname) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strbuf_init(sb: *mut strbuf, hint: usize);
    fn strbuf_add(sb: *mut strbuf, data: *const c_void, len: usize);
    fn strbuf_detach(sb: *mut strbuf, sz: *mut usize) -> *mut c_char;
    fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;
    fn perf_pmu__has_format(pmu: *mut perf_pmu, name: *const c_char) -> bool;
    fn perf_exe(buf: *mut c_char, len: usize);
    fn x86__is_amd_cpu() -> bool;
    fn pr_debug(fmt: *const c_char, ...);
}

/* Dummy workload to generate IBS samples. */
unsafe fn dummy_workload_1(mut count: c_ulong) -> c_int {
    let mut ret: c_int = 0;
    let insn1: [u8; 7] = [0xb8, 0x01, 0x00, 0x00, 0x00, 0xc3, 0xcc];
    let insn2: [u8; 7] = [0xb8, 0x02, 0x00, 0x00, 0x00, 0xc3, 0xcc];
    let func_mem = mmap(ptr::null_mut(), page_size as c_ulong, PROT_READ | PROT_WRITE | PROT_EXEC,
                        MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if func_mem == MAP_FAILED {
        pr_debug(c"mmap() failed. %m\n".as_ptr());
        return -1;
    }
    let func: extern "C" fn() -> c_int = core::mem::transmute(func_mem);

    if count < 100000 {
        count = 100000;
    } else if count > 10000000 {
        count = 10000000;
    }
    while count != 0 {
        count = count.wrapping_sub(1);
        memcpy(func_mem, insn1.as_ptr() as *const c_void, size_of_val(&insn1));
        if func() != 1 {
            pr_debug(c"ERROR insn1\n".as_ptr());
            ret = -1;
            break;
        }
        memcpy(func_mem, insn2.as_ptr() as *const c_void, size_of_val(&insn2));
        if func() != 2 {
            pr_debug(c"ERROR insn2\n".as_ptr());
            ret = -1;
            break;
        }
    }

    munmap(func_mem, page_size as c_ulong);
    ret
}

/* Another dummy workload to generate IBS samples. */
unsafe fn dummy_workload_2(perf: *mut c_char) {
    let bench = c" bench sched messaging -g 10 -l 5000 > /dev/null 2>&1";
    let taskset = c"taskset -c 0 ";
    let mut sb = core::mem::MaybeUninit::<strbuf>::uninit();
    strbuf_init(sb.as_mut_ptr(), 0);
    strbuf_add(sb.as_mut_ptr(), taskset.as_ptr() as *const c_void, strlen(taskset.as_ptr()));
    strbuf_add(sb.as_mut_ptr(), perf as *const c_void, strlen(perf));
    strbuf_add(sb.as_mut_ptr(), bench.as_ptr() as *const c_void, strlen(bench.as_ptr()));
    let cmd = strbuf_detach(sb.as_mut_ptr(), ptr::null_mut());
    let _ret = system(cmd);
    free(cmd as *mut c_void);
}

unsafe fn sched_affine(cpu: c_int) -> c_int {
    let mut set = [0u8; 128];
    ptr::write_bytes(set.as_mut_ptr(), 0, set.len());
    let byte = (cpu / 8) as usize;
    let bit = (cpu % 8) as u8;
    set[byte] |= 1u8 << bit;
    if sched_setaffinity(getpid(), size_of_val(&set), set.as_ptr() as *const c_void) == -1 {
        pr_debug(c"sched_setaffinity() failed. [%m]".as_ptr());
        return -1;
    }
    0
}

unsafe fn copy_sample_data(src: *mut c_void, offset: c_ulong, dest: *mut c_void, size: usize) {
    let mut chunk1_size: usize;
    let chunk2_size: usize;

    if offset.wrapping_add(size as c_ulong) < PERF_MMAP_DATA_SIZE() {
        memcpy(dest, (src as *mut u8).add(offset as usize) as *const c_void, size);
    } else {
        chunk1_size = PERF_MMAP_DATA_SIZE().wrapping_sub(offset) as usize;
        chunk2_size = size.wrapping_sub(chunk1_size);
        memcpy(dest, (src as *mut u8).add(offset as usize) as *const c_void, chunk1_size);
        memcpy((dest as *mut u8).add(chunk1_size) as *mut c_void, src as *const c_void, chunk2_size);
    }
}

unsafe fn rb_read(rb: *mut perf_event_mmap_page, dest: *mut c_void, size: usize) -> c_int {
    /* Casting to (void *) is needed. */
    let base = (rb as *mut u8).add(page_size as usize) as *mut c_void;
    let data_head = (*rb).data_head;
    rmb();
    let mut data_tail = (*rb).data_tail;

    if data_head.wrapping_sub(data_tail) < size as c_ulong {
        return -1;
    }

    data_tail &= PERF_MMAP_DATA_MASK();
    copy_sample_data(base, data_tail, dest, size);
    (*rb).data_tail = (*rb).data_tail.wrapping_add(size as c_ulong);
    0
}

unsafe fn rb_skip(rb: *mut perf_event_mmap_page, size: usize) {
    let data_head = (*rb).data_head as usize;
    rmb();
    if ((*rb).data_tail as usize).wrapping_add(size) > data_head {
        (*rb).data_tail = data_head as c_ulong;
    } else {
        (*rb).data_tail = (*rb).data_tail.wrapping_add(size as c_ulong);
    }
}

/* Sample period value taken from perf sample must match with expected value. */
fn period_equal(exp_period: c_ulong, act_period: c_ulong) -> c_int {
    if exp_period == act_period { 0 } else { -1 }
}

/*
 * Sample period value taken from perf sample must be >= minimum sample period
 * supported by IBS HW.
 */
fn period_higher(min_period: c_ulong, act_period: c_ulong) -> c_int {
    if min_period <= act_period { 0 } else { -1 }
}

unsafe fn rb_drain_samples(
    rb: *mut perf_event_mmap_page,
    exp_period: c_ulong,
    nr_samples: *mut c_int,
    callback: fn(c_ulong, c_ulong) -> c_int,
) -> c_int {
    let mut hdr = core::mem::MaybeUninit::<perf_event_header>::uninit();
    let mut ret: c_int = 0;

    /*
     * PERF_RECORD_SAMPLE:
     * struct {
     *      struct perf_event_header hdr;
     *      { u64                    period;     } && PERF_SAMPLE_PERIOD
     * };
     */
    loop {
        if rb_read(rb, hdr.as_mut_ptr() as *mut c_void, size_of::<perf_event_header>()) != 0 {
            return ret;
        }
        let hdr = hdr.assume_init();
        if hdr.type_ == PERF_RECORD_SAMPLE {
            *nr_samples += 1;
            let mut period: c_ulong = 0;
            if rb_read(rb, &mut period as *mut _ as *mut c_void, size_of::<c_ulong>()) != 0 {
                pr_debug(c"rb_read(period) error. [%m]".as_ptr());
            }
            ret |= callback(exp_period, period);
        } else {
            rb_skip(rb, (hdr.size as usize).wrapping_sub(size_of::<perf_event_header>()));
        }
    }
}

unsafe fn perf_event_open(attr: *mut perf_event_attr, pid: c_int, cpu: c_int, group_fd: c_int, flags: c_ulong) -> c_long {
    syscall(__NR_perf_event_open, attr, pid, cpu, group_fd, flags)
}

unsafe fn fetch_prepare_attr(attr: *mut perf_event_attr, config: c_ulonglong, freq: c_int, sample_period: c_ulong) {
    memset(attr as *mut c_void, 0, size_of::<perf_event_attr>());
    (*attr).type_ = (*fetch_pmu).type_;
    (*attr).size = size_of::<perf_event_attr>() as c_uint;
    (*attr).config = config;
    (*attr).disabled = 1;
    (*attr).sample_type = PERF_SAMPLE_PERIOD;
    (*attr).freq = freq as c_ulonglong;
    (*attr).sample_period = sample_period; /* = ->sample_freq */
}

unsafe fn op_prepare_attr(attr: *mut perf_event_attr, config: c_ulong, freq: c_int, sample_period: c_ulong) {
    memset(attr as *mut c_void, 0, size_of::<perf_event_attr>());
    (*attr).type_ = (*op_pmu).type_;
    (*attr).size = size_of::<perf_event_attr>() as c_uint;
    (*attr).config = config as c_ulonglong;
    (*attr).disabled = 1;
    (*attr).sample_type = PERF_SAMPLE_PERIOD;
    (*attr).freq = freq as c_ulonglong;
    (*attr).sample_period = sample_period; /* = ->sample_freq */
}

#[repr(C)]
struct ibs_configs {
    /* Input */
    config: c_ulong,
    /* Expected output */
    period: c_ulong,
    fd: c_int,
}

/*
 * Somehow first Fetch event with sample period = 0x10 causes 0
 * samples. So start with large period and decrease it gradually.
 */
static mut fetch_configs: [ibs_configs; 6] = [
    ibs_configs { config: 0xffff, period: 0xffff0, fd: FD_SUCCESS },
    ibs_configs { config: 0x1000, period: 0x10000, fd: FD_SUCCESS },
    ibs_configs { config: 0xff, period: 0xff0, fd: FD_SUCCESS },
    ibs_configs { config: 0x1, period: 0x10, fd: FD_SUCCESS },
    ibs_configs { config: 0x0, period: !0, fd: FD_ERROR },
    ibs_configs { config: 0x10000, period: !0, fd: FD_ERROR },
];

static mut op_configs: [ibs_configs; 15] = [
    ibs_configs { config: 0x0, period: !0, fd: FD_ERROR },
    ibs_configs { config: 0x1, period: !0, fd: FD_ERROR },
    ibs_configs { config: 0x8, period: !0, fd: FD_ERROR },
    ibs_configs { config: 0x9, period: 0x90, fd: FD_SUCCESS },
    ibs_configs { config: 0xf, period: 0xf0, fd: FD_SUCCESS },
    ibs_configs { config: 0x1000, period: 0x10000, fd: FD_SUCCESS },
    ibs_configs { config: 0xffff, period: 0xffff0, fd: FD_SUCCESS },
    ibs_configs { config: 0x10000, period: !0, fd: FD_ERROR },
    ibs_configs { config: 0x100000, period: 0x100000, fd: FD_SUCCESS },
    ibs_configs { config: 0xf00000, period: 0xf00000, fd: FD_SUCCESS },
    ibs_configs { config: 0xf0ffff, period: 0xfffff0, fd: FD_SUCCESS },
    ibs_configs { config: 0x1f0ffff, period: 0x1fffff0, fd: FD_SUCCESS },
    ibs_configs { config: 0x7f0ffff, period: 0x7fffff0, fd: FD_SUCCESS },
    ibs_configs { config: 0x8f0ffff, period: !0, fd: FD_ERROR },
    ibs_configs { config: 0x17f0ffff, period: !0, fd: FD_ERROR },
];

unsafe fn __ibs_config_test(ibs_type: c_int, config: *mut ibs_configs, nr_samples: *mut c_int) -> c_int {
    let mut attr = core::mem::MaybeUninit::<perf_event_attr>::uninit();
    let mut ret: c_int = 0;

    if ibs_type == IBS_FETCH {
        fetch_prepare_attr(attr.as_mut_ptr(), (*config).config as c_ulonglong, 0, 0);
    } else {
        op_prepare_attr(attr.as_mut_ptr(), (*config).config, 0, 0);
    }

    /* CPU0, All processes */
    let fd = perf_event_open(attr.as_mut_ptr(), -1, 0, -1, 0) as c_int;
    if (*config).fd == FD_ERROR {
        if fd != -1 {
            close(fd);
            return -1;
        }
        return 0;
    }
    if fd <= -1 {
        return -1;
    }

    let rb = mmap(ptr::null_mut(), PERF_MMAP_TOTAL_SIZE(), PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if rb == MAP_FAILED {
        pr_debug(c"mmap() failed. [%m]\n".as_ptr());
        return -1;
    }

    ioctl(fd, PERF_EVENT_IOC_RESET, 0);
    ioctl(fd, PERF_EVENT_IOC_ENABLE, 0);

    let mut i = 5;
    while i != 0 {
        i -= 1;
        dummy_workload_1(1000000);
        ret = rb_drain_samples(rb as *mut perf_event_mmap_page, (*config).period, nr_samples, period_equal);
        if ret != 0 {
            break;
        }
    }

    ioctl(fd, PERF_EVENT_IOC_DISABLE, 0);
    munmap(rb, PERF_MMAP_TOTAL_SIZE());
    close(fd);
    ret
}

unsafe fn ibs_config_test() -> c_int {
    let mut nr_samples: c_int = 0;
    let mut ret: c_int = 0;

    pr_debug(c"\nIBS config tests:\n".as_ptr());
    pr_debug(c"-----------------\n".as_ptr());

    pr_debug(c"Fetch PMU tests:\n".as_ptr());
    let mut i = 0;
    while i < fetch_configs.len() {
        nr_samples = 0;
        let r = __ibs_config_test(IBS_FETCH, &mut fetch_configs[i], &mut nr_samples);
        if fetch_configs[i].fd == FD_ERROR {
            pr_debug(c"0x%-16lx: %-4s\n".as_ptr(), fetch_configs[i].config, if r == 0 { c"Ok".as_ptr() } else { c"Fail".as_ptr() });
        } else {
            /*
             * Although nr_samples == 0 is reported as Fail here,
             * the failure status is not cascaded up because, we
             * can not decide whether test really failed or not
             * without actual samples.
             */
            pr_debug(c"0x%-16lx: %-4s (nr samples: %d)\n".as_ptr(), fetch_configs[i].config,
                     if r == 0 && nr_samples != 0 { c"Ok".as_ptr() } else { c"Fail".as_ptr() }, nr_samples);
        }
        ret |= r;
        i += 1;
    }

    pr_debug(c"Op PMU tests:\n".as_ptr());
    i = 0;
    while i < op_configs.len() {
        nr_samples = 0;
        let r = __ibs_config_test(IBS_OP, &mut op_configs[i], &mut nr_samples);
        if op_configs[i].fd == FD_ERROR {
            pr_debug(c"0x%-16lx: %-4s\n".as_ptr(), op_configs[i].config, if r == 0 { c"Ok".as_ptr() } else { c"Fail".as_ptr() });
        } else {
            /*
             * Although nr_samples == 0 is reported as Fail here,
             * the failure status is not cascaded up because, we
             * can not decide whether test really failed or not
             * without actual samples.
             */
            pr_debug(c"0x%-16lx: %-4s (nr samples: %d)\n".as_ptr(), op_configs[i].config,
                     if r == 0 && nr_samples != 0 { c"Ok".as_ptr() } else { c"Fail".as_ptr() }, nr_samples);
        }
        ret |= r;
        i += 1;
    }

    ret
}

#[repr(C)]
struct ibs_period {
    /* Input */
    freq: c_int,
    sample_freq: c_ulong,
    /* Output */
    ret: c_int,
    period: c_ulong,
}

static mut fetch_period: [ibs_period; 29] = [
    ibs_period { freq: 0, sample_freq: 0, ret: FD_ERROR, period: !0 }, ibs_period { freq: 0, sample_freq: 1, ret: FD_ERROR, period: !0 },
    ibs_period { freq: 0, sample_freq: 0xf, ret: FD_ERROR, period: !0 }, ibs_period { freq: 0, sample_freq: 0x10, ret: FD_SUCCESS, period: 0x10 },
    ibs_period { freq: 0, sample_freq: 0x11, ret: FD_SUCCESS, period: 0x10 }, ibs_period { freq: 0, sample_freq: 0x8f, ret: FD_SUCCESS, period: 0x80 },
    ibs_period { freq: 0, sample_freq: 0x90, ret: FD_SUCCESS, period: 0x90 }, ibs_period { freq: 0, sample_freq: 0x91, ret: FD_SUCCESS, period: 0x90 },
    ibs_period { freq: 0, sample_freq: 0x4d2, ret: FD_SUCCESS, period: 0x4d0 }, ibs_period { freq: 0, sample_freq: 0x1007, ret: FD_SUCCESS, period: 0x1000 },
    ibs_period { freq: 0, sample_freq: 0xfff0, ret: FD_SUCCESS, period: 0xfff0 }, ibs_period { freq: 0, sample_freq: 0xffff, ret: FD_SUCCESS, period: 0xfff0 },
    ibs_period { freq: 0, sample_freq: 0x10010, ret: FD_SUCCESS, period: 0x10010 }, ibs_period { freq: 0, sample_freq: 0x7fffff, ret: FD_SUCCESS, period: 0x7ffff0 },
    ibs_period { freq: 0, sample_freq: 0xfffffff, ret: FD_SUCCESS, period: 0xffffff0 }, ibs_period { freq: 1, sample_freq: 0, ret: FD_ERROR, period: !0 },
    ibs_period { freq: 1, sample_freq: 1, ret: FD_SUCCESS, period: 0x10 }, ibs_period { freq: 1, sample_freq: 0xf, ret: FD_SUCCESS, period: 0x10 },
    ibs_period { freq: 1, sample_freq: 0x10, ret: FD_SUCCESS, period: 0x10 }, ibs_period { freq: 1, sample_freq: 0x11, ret: FD_SUCCESS, period: 0x10 },
    ibs_period { freq: 1, sample_freq: 0x8f, ret: FD_SUCCESS, period: 0x10 }, ibs_period { freq: 1, sample_freq: 0x90, ret: FD_SUCCESS, period: 0x10 },
    ibs_period { freq: 1, sample_freq: 0x91, ret: FD_SUCCESS, period: 0x10 }, ibs_period { freq: 1, sample_freq: 0x4d2, ret: FD_SUCCESS, period: 0x10 },
    ibs_period { freq: 1, sample_freq: 0x1007, ret: FD_SUCCESS, period: 0x10 }, ibs_period { freq: 1, sample_freq: 0xfff0, ret: FD_SUCCESS, period: 0x10 },
    ibs_period { freq: 1, sample_freq: 0xffff, ret: FD_SUCCESS, period: 0x10 }, ibs_period { freq: 1, sample_freq: 0x10010, ret: FD_SUCCESS, period: 0x10 },
    /* ret=FD_ERROR because freq > default perf_event_max_sample_rate (100000) */
    ibs_period { freq: 1, sample_freq: 0x7fffff, ret: FD_ERROR, period: !0 },
];

static mut op_period: [ibs_period; 29] = [
    ibs_period { freq: 0, sample_freq: 0, ret: FD_ERROR, period: !0 }, ibs_period { freq: 0, sample_freq: 1, ret: FD_ERROR, period: !0 },
    ibs_period { freq: 0, sample_freq: 0xf, ret: FD_ERROR, period: !0 }, ibs_period { freq: 0, sample_freq: 0x10, ret: FD_ERROR, period: !0 },
    ibs_period { freq: 0, sample_freq: 0x11, ret: FD_ERROR, period: !0 }, ibs_period { freq: 0, sample_freq: 0x8f, ret: FD_ERROR, period: !0 },
    ibs_period { freq: 0, sample_freq: 0x90, ret: FD_SUCCESS, period: 0x90 }, ibs_period { freq: 0, sample_freq: 0x91, ret: FD_SUCCESS, period: 0x90 },
    ibs_period { freq: 0, sample_freq: 0x4d2, ret: FD_SUCCESS, period: 0x4d0 }, ibs_period { freq: 0, sample_freq: 0x1007, ret: FD_SUCCESS, period: 0x1000 },
    ibs_period { freq: 0, sample_freq: 0xfff0, ret: FD_SUCCESS, period: 0xfff0 }, ibs_period { freq: 0, sample_freq: 0xffff, ret: FD_SUCCESS, period: 0xfff0 },
    ibs_period { freq: 0, sample_freq: 0x10010, ret: FD_SUCCESS, period: 0x10010 }, ibs_period { freq: 0, sample_freq: 0x7fffff, ret: FD_SUCCESS, period: 0x7ffff0 },
    ibs_period { freq: 0, sample_freq: 0xfffffff, ret: FD_SUCCESS, period: 0xffffff0 }, ibs_period { freq: 1, sample_freq: 0, ret: FD_ERROR, period: !0 },
    ibs_period { freq: 1, sample_freq: 1, ret: FD_SUCCESS, period: 0x90 }, ibs_period { freq: 1, sample_freq: 0xf, ret: FD_SUCCESS, period: 0x90 },
    ibs_period { freq: 1, sample_freq: 0x10, ret: FD_SUCCESS, period: 0x90 }, ibs_period { freq: 1, sample_freq: 0x11, ret: FD_SUCCESS, period: 0x90 },
    ibs_period { freq: 1, sample_freq: 0x8f, ret: FD_SUCCESS, period: 0x90 }, ibs_period { freq: 1, sample_freq: 0x90, ret: FD_SUCCESS, period: 0x90 },
    ibs_period { freq: 1, sample_freq: 0x91, ret: FD_SUCCESS, period: 0x90 }, ibs_period { freq: 1, sample_freq: 0x4d2, ret: FD_SUCCESS, period: 0x90 },
    ibs_period { freq: 1, sample_freq: 0x1007, ret: FD_SUCCESS, period: 0x90 }, ibs_period { freq: 1, sample_freq: 0xfff0, ret: FD_SUCCESS, period: 0x90 },
    ibs_period { freq: 1, sample_freq: 0xffff, ret: FD_SUCCESS, period: 0x90 }, ibs_period { freq: 1, sample_freq: 0x10010, ret: FD_SUCCESS, period: 0x90 },
    /* ret=FD_ERROR because freq > default perf_event_max_sample_rate (100000) */
    ibs_period { freq: 1, sample_freq: 0x7fffff, ret: FD_ERROR, period: !0 },
];

unsafe fn __ibs_period_constraint_test(ibs_type: c_int, period: *mut ibs_period, nr_samples: *mut c_int) -> c_int {
    let mut attr = core::mem::MaybeUninit::<perf_event_attr>::uninit();
    let mut ret: c_int = 0;

    if (*period).freq != 0 && (*period).sample_freq > perf_event_max_sample_rate as c_ulong {
        (*period).ret = FD_ERROR;
    }
    if ibs_type == IBS_FETCH {
        fetch_prepare_attr(attr.as_mut_ptr(), 0, (*period).freq, (*period).sample_freq);
    } else {
        op_prepare_attr(attr.as_mut_ptr(), 0, (*period).freq, (*period).sample_freq);
    }

    /* CPU0, All processes */
    let fd = perf_event_open(attr.as_mut_ptr(), -1, 0, -1, 0) as c_int;
    if (*period).ret == FD_ERROR {
        if fd != -1 {
            close(fd);
            return -1;
        }
        return 0;
    }
    if fd <= -1 {
        return -1;
    }

    let rb = mmap(ptr::null_mut(), PERF_MMAP_TOTAL_SIZE(), PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if rb == MAP_FAILED {
        pr_debug(c"mmap() failed. [%m]\n".as_ptr());
        close(fd);
        return -1;
    }

    ioctl(fd, PERF_EVENT_IOC_RESET, 0);
    ioctl(fd, PERF_EVENT_IOC_ENABLE, 0);

    if (*period).freq != 0 {
        dummy_workload_1(100000);
        ret = rb_drain_samples(rb as *mut perf_event_mmap_page, (*period).period, nr_samples, period_higher);
    } else {
        dummy_workload_1((*period).sample_freq.wrapping_mul(10));
        ret = rb_drain_samples(rb as *mut perf_event_mmap_page, (*period).period, nr_samples, period_equal);
    }

    ioctl(fd, PERF_EVENT_IOC_DISABLE, 0);
    munmap(rb, PERF_MMAP_TOTAL_SIZE());
    close(fd);
    ret
}

unsafe fn ibs_period_constraint_test() -> c_int {
    let mut nr_samples: c_int;
    let mut ret: c_int = 0;

    pr_debug(c"\nIBS sample period constraint tests:\n".as_ptr());
    pr_debug(c"-----------------------------------\n".as_ptr());
    pr_debug(c"Fetch PMU test:\n".as_ptr());
    let mut i = 0;
    while i < fetch_period.len() {
        nr_samples = 0;
        let r = __ibs_period_constraint_test(IBS_FETCH, &mut fetch_period[i], &mut nr_samples);
        if fetch_period[i].ret == FD_ERROR {
            pr_debug(c"freq %d, sample_freq %9ld: %-4s\n".as_ptr(), fetch_period[i].freq, fetch_period[i].sample_freq, if r == 0 { c"Ok".as_ptr() } else { c"Fail".as_ptr() });
        } else {
            /*
             * Although nr_samples == 0 is reported as Fail here,
             * the failure status is not cascaded up because, we
             * can not decide whether test really failed or not
             * without actual samples.
             */
            pr_debug(c"freq %d, sample_freq %9ld: %-4s (nr samples: %d)\n".as_ptr(), fetch_period[i].freq, fetch_period[i].sample_freq, if r == 0 && nr_samples != 0 { c"Ok".as_ptr() } else { c"Fail".as_ptr() }, nr_samples);
        }
        ret |= r;
        i += 1;
    }

    pr_debug(c"Op PMU test:\n".as_ptr());
    i = 0;
    while i < op_period.len() {
        nr_samples = 0;
        let r = __ibs_period_constraint_test(IBS_OP, &mut op_period[i], &mut nr_samples);
        if op_period[i].ret == FD_ERROR {
            pr_debug(c"freq %d, sample_freq %9ld: %-4s\n".as_ptr(), op_period[i].freq, op_period[i].sample_freq, if r == 0 { c"Ok".as_ptr() } else { c"Fail".as_ptr() });
        } else {
            /*
             * Although nr_samples == 0 is reported as Fail here,
             * the failure status is not cascaded up because, we
             * can not decide whether test really failed or not
             * without actual samples.
             */
            pr_debug(c"freq %d, sample_freq %9ld: %-4s (nr samples: %d)\n".as_ptr(), op_period[i].freq, op_period[i].sample_freq, if r == 0 && nr_samples != 0 { c"Ok".as_ptr() } else { c"Fail".as_ptr() }, nr_samples);
        }
        ret |= r;
        i += 1;
    }
    ret
}

#[repr(C)]
struct ibs_ioctl {
    /* Input */
    freq: c_int,
    period: c_ulong,
    /* Expected output */
    ret: c_int,
}

static mut fetch_ioctl: [ibs_ioctl; 29] = [
    ibs_ioctl { freq: 0, period: 0x0, ret: FD_ERROR }, ibs_ioctl { freq: 0, period: 0x1, ret: FD_ERROR },
    ibs_ioctl { freq: 0, period: 0xf, ret: FD_ERROR }, ibs_ioctl { freq: 0, period: 0x10, ret: FD_SUCCESS },
    ibs_ioctl { freq: 0, period: 0x11, ret: FD_ERROR }, ibs_ioctl { freq: 0, period: 0x1f, ret: FD_ERROR },
    ibs_ioctl { freq: 0, period: 0x20, ret: FD_SUCCESS }, ibs_ioctl { freq: 0, period: 0x80, ret: FD_SUCCESS },
    ibs_ioctl { freq: 0, period: 0x8f, ret: FD_ERROR }, ibs_ioctl { freq: 0, period: 0x90, ret: FD_SUCCESS },
    ibs_ioctl { freq: 0, period: 0x91, ret: FD_ERROR }, ibs_ioctl { freq: 0, period: 0x100, ret: FD_SUCCESS },
    ibs_ioctl { freq: 0, period: 0xfff0, ret: FD_SUCCESS }, ibs_ioctl { freq: 0, period: 0xffff, ret: FD_ERROR },
    ibs_ioctl { freq: 0, period: 0x10000, ret: FD_SUCCESS }, ibs_ioctl { freq: 0, period: 0x1fff0, ret: FD_SUCCESS },
    ibs_ioctl { freq: 0, period: 0x1fff5, ret: FD_ERROR }, ibs_ioctl { freq: 1, period: 0x0, ret: FD_ERROR },
    ibs_ioctl { freq: 1, period: 0x1, ret: FD_SUCCESS }, ibs_ioctl { freq: 1, period: 0xf, ret: FD_SUCCESS },
    ibs_ioctl { freq: 1, period: 0x10, ret: FD_SUCCESS }, ibs_ioctl { freq: 1, period: 0x11, ret: FD_SUCCESS },
    ibs_ioctl { freq: 1, period: 0x1f, ret: FD_SUCCESS }, ibs_ioctl { freq: 1, period: 0x20, ret: FD_SUCCESS },
    ibs_ioctl { freq: 1, period: 0x80, ret: FD_SUCCESS }, ibs_ioctl { freq: 1, period: 0x8f, ret: FD_SUCCESS },
    ibs_ioctl { freq: 1, period: 0x90, ret: FD_SUCCESS }, ibs_ioctl { freq: 1, period: 0x91, ret: FD_SUCCESS },
    ibs_ioctl { freq: 1, period: 0x100, ret: FD_SUCCESS },
];

static mut op_ioctl: [ibs_ioctl; 29] = [
    ibs_ioctl { freq: 0, period: 0x0, ret: FD_ERROR }, ibs_ioctl { freq: 0, period: 0x1, ret: FD_ERROR },
    ibs_ioctl { freq: 0, period: 0xf, ret: FD_ERROR }, ibs_ioctl { freq: 0, period: 0x10, ret: FD_ERROR },
    ibs_ioctl { freq: 0, period: 0x11, ret: FD_ERROR }, ibs_ioctl { freq: 0, period: 0x1f, ret: FD_ERROR },
    ibs_ioctl { freq: 0, period: 0x20, ret: FD_ERROR }, ibs_ioctl { freq: 0, period: 0x80, ret: FD_ERROR },
    ibs_ioctl { freq: 0, period: 0x8f, ret: FD_ERROR }, ibs_ioctl { freq: 0, period: 0x90, ret: FD_SUCCESS },
    ibs_ioctl { freq: 0, period: 0x91, ret: FD_ERROR }, ibs_ioctl { freq: 0, period: 0x100, ret: FD_SUCCESS },
    ibs_ioctl { freq: 0, period: 0xfff0, ret: FD_SUCCESS }, ibs_ioctl { freq: 0, period: 0xffff, ret: FD_ERROR },
    ibs_ioctl { freq: 0, period: 0x10000, ret: FD_SUCCESS }, ibs_ioctl { freq: 0, period: 0x1fff0, ret: FD_SUCCESS },
    ibs_ioctl { freq: 0, period: 0x1fff5, ret: FD_ERROR }, ibs_ioctl { freq: 1, period: 0x0, ret: FD_ERROR },
    ibs_ioctl { freq: 1, period: 0x1, ret: FD_SUCCESS }, ibs_ioctl { freq: 1, period: 0xf, ret: FD_SUCCESS },
    ibs_ioctl { freq: 1, period: 0x10, ret: FD_SUCCESS }, ibs_ioctl { freq: 1, period: 0x11, ret: FD_SUCCESS },
    ibs_ioctl { freq: 1, period: 0x1f, ret: FD_SUCCESS }, ibs_ioctl { freq: 1, period: 0x20, ret: FD_SUCCESS },
    ibs_ioctl { freq: 1, period: 0x80, ret: FD_SUCCESS }, ibs_ioctl { freq: 1, period: 0x8f, ret: FD_SUCCESS },
    ibs_ioctl { freq: 1, period: 0x90, ret: FD_SUCCESS }, ibs_ioctl { freq: 1, period: 0x91, ret: FD_SUCCESS },
    ibs_ioctl { freq: 1, period: 0x100, ret: FD_SUCCESS },
];

unsafe fn __ibs_ioctl_test(ibs_type: c_int, ibs_ioctl: *mut ibs_ioctl) -> c_int {
    let mut attr = core::mem::MaybeUninit::<perf_event_attr>::uninit();
    let mut ret: c_int = 0;

    if ibs_type == IBS_FETCH {
        fetch_prepare_attr(attr.as_mut_ptr(), 0, (*ibs_ioctl).freq, 1000);
    } else {
        op_prepare_attr(attr.as_mut_ptr(), 0, (*ibs_ioctl).freq, 1000);
    }

    /* CPU0, All processes */
    let fd = perf_event_open(attr.as_mut_ptr(), -1, 0, -1, 0) as c_int;
    if fd <= -1 {
        pr_debug(c"event_open() Failed\n".as_ptr());
        return -1;
    }

    let r = ioctl(fd, PERF_EVENT_IOC_PERIOD, &mut (*ibs_ioctl).period as *mut c_ulong);
    if ((*ibs_ioctl).ret == FD_SUCCESS && r <= -1) || ((*ibs_ioctl).ret == FD_ERROR && r >= 0) {
        ret = -1;
    }
    close(fd);
    ret
}

unsafe fn ibs_ioctl_test() -> c_int {
    let mut ret: c_int = 0;
    pr_debug(c"\nIBS ioctl() tests:\n".as_ptr());
    pr_debug(c"------------------\n".as_ptr());

    pr_debug(c"Fetch PMU tests\n".as_ptr());
    let mut i = 0;
    while i < fetch_ioctl.len() {
        let r = __ibs_ioctl_test(IBS_FETCH, &mut fetch_ioctl[i]);
        pr_debug(c"ioctl(%s = 0x%-7lx): %s\n".as_ptr(), if fetch_ioctl[i].freq != 0 { c"freq  ".as_ptr() } else { c"period".as_ptr() }, fetch_ioctl[i].period, if r != 0 { c"Fail".as_ptr() } else { c"Ok".as_ptr() });
        ret |= r;
        i += 1;
    }

    pr_debug(c"Op PMU tests\n".as_ptr());
    i = 0;
    while i < op_ioctl.len() {
        let r = __ibs_ioctl_test(IBS_OP, &mut op_ioctl[i]);
        pr_debug(c"ioctl(%s = 0x%-7lx): %s\n".as_ptr(), if op_ioctl[i].freq != 0 { c"freq  ".as_ptr() } else { c"period".as_ptr() }, op_ioctl[i].period, if r != 0 { c"Fail".as_ptr() } else { c"Ok".as_ptr() });
        ret |= r;
        i += 1;
    }
    ret
}

unsafe fn ibs_freq_neg_test() -> c_int {
    let mut attr = core::mem::MaybeUninit::<perf_event_attr>::uninit();

    pr_debug(c"\nIBS freq (negative) tests:\n".as_ptr());
    pr_debug(c"--------------------------\n".as_ptr());

    /*
     * Assuming perf_event_max_sample_rate <= 100000,
     * config: 0x300D40 ==> MaxCnt: 200000
     */
    op_prepare_attr(attr.as_mut_ptr(), 0x300D40, 1, 0);

    /* CPU0, All processes */
    let fd = perf_event_open(attr.as_mut_ptr(), -1, 0, -1, 0) as c_int;
    if fd != -1 {
        pr_debug(c"freq 1, sample_freq 200000: Fail\n".as_ptr());
        close(fd);
        return -1;
    }

    pr_debug(c"freq 1, sample_freq 200000: Ok\n".as_ptr());
    0
}

#[repr(C)]
struct ibs_l3missonly {
    /* Input */
    freq: c_int,
    sample_freq: c_ulong,
    /* Expected output */
    ret: c_int,
    min_period: c_ulong,
}

static mut fetch_l3missonly: ibs_l3missonly = ibs_l3missonly {
    freq: 1,
    sample_freq: 10000,
    ret: FD_SUCCESS,
    min_period: 0x10,
};

static mut op_l3missonly: ibs_l3missonly = ibs_l3missonly {
    freq: 1,
    sample_freq: 10000,
    ret: FD_SUCCESS,
    min_period: 0x90,
};

unsafe fn __ibs_l3missonly_test(perf: *mut c_char, ibs_type: c_int, nr_samples: *mut c_int, l3missonly: *mut ibs_l3missonly) -> c_int {
    let mut attr = core::mem::MaybeUninit::<perf_event_attr>::uninit();
    let mut ret: c_int = 0;

    if (*l3missonly).sample_freq > perf_event_max_sample_rate as c_ulong {
        (*l3missonly).ret = FD_ERROR;
    }
    if ibs_type == IBS_FETCH {
        fetch_prepare_attr(attr.as_mut_ptr(), 0x800000000000000, (*l3missonly).freq, (*l3missonly).sample_freq);
    } else {
        op_prepare_attr(attr.as_mut_ptr(), 0x10000, (*l3missonly).freq, (*l3missonly).sample_freq);
    }

    /* CPU0, All processes */
    let fd = perf_event_open(attr.as_mut_ptr(), -1, 0, -1, 0) as c_int;
    if (*l3missonly).ret == FD_ERROR {
        if fd != -1 {
            close(fd);
            return -1;
        }
        return 0;
    }
    if fd == -1 {
        pr_debug(c"perf_event_open() failed. [%m]\n".as_ptr());
        return -1;
    }

    let rb = mmap(ptr::null_mut(), PERF_MMAP_TOTAL_SIZE(), PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if rb == MAP_FAILED {
        pr_debug(c"mmap() failed. [%m]\n".as_ptr());
        close(fd);
        return -1;
    }

    ioctl(fd, PERF_EVENT_IOC_RESET, 0);
    ioctl(fd, PERF_EVENT_IOC_ENABLE, 0);
    dummy_workload_2(perf);
    ioctl(fd, PERF_EVENT_IOC_DISABLE, 0);
    ret = rb_drain_samples(rb as *mut perf_event_mmap_page, (*l3missonly).min_period, nr_samples, period_higher);
    munmap(rb, PERF_MMAP_TOTAL_SIZE());
    close(fd);
    ret
}

unsafe fn ibs_l3missonly_test(perf: *mut c_char) -> c_int {
    let mut nr_samples: c_int = 0;
    let mut ret: c_int = 0;

    pr_debug(c"\nIBS L3MissOnly test: (takes a while)\n".as_ptr());
    pr_debug(c"--------------------\n".as_ptr());

    if perf_pmu__has_format(fetch_pmu, c"l3missonly".as_ptr()) {
        nr_samples = 0;
        let r = __ibs_l3missonly_test(perf, IBS_FETCH, &mut nr_samples, &mut fetch_l3missonly);
        if fetch_l3missonly.ret == FD_ERROR {
            pr_debug(c"Fetch L3MissOnly: %-4s\n".as_ptr(), if r == 0 { c"Ok".as_ptr() } else { c"Fail".as_ptr() });
        } else {
            /*
             * Although nr_samples == 0 is reported as Fail here,
             * the failure status is not cascaded up because, we
             * can not decide whether test really failed or not
             * without actual samples.
             */
            pr_debug(c"Fetch L3MissOnly: %-4s (nr_samples: %d)\n".as_ptr(), if r == 0 && nr_samples != 0 { c"Ok".as_ptr() } else { c"Fail".as_ptr() }, nr_samples);
        }
        ret |= r;
    }

    if perf_pmu__has_format(op_pmu, c"l3missonly".as_ptr()) {
        nr_samples = 0;
        let r = __ibs_l3missonly_test(perf, IBS_OP, &mut nr_samples, &mut op_l3missonly);
        if op_l3missonly.ret == FD_ERROR {
            pr_debug(c"Op L3MissOnly:    %-4s\n".as_ptr(), if r == 0 { c"Ok".as_ptr() } else { c"Fail".as_ptr() });
        } else {
            /*
             * Although nr_samples == 0 is reported as Fail here,
             * the failure status is not cascaded up because, we
             * can not decide whether test really failed or not
             * without actual samples.
             */
            pr_debug(c"Op L3MissOnly:    %-4s (nr_samples: %d)\n".as_ptr(), if r == 0 && nr_samples != 0 { c"Ok".as_ptr() } else { c"Fail".as_ptr() }, nr_samples);
        }
        ret |= r;
    }

    ret
}

unsafe fn get_perf_event_max_sample_rate() -> c_uint {
    let mut max_sample_rate: c_uint = 100000;
    let fp = fopen(c"/proc/sys/kernel/perf_event_max_sample_rate".as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        pr_debug(c"Can't open perf_event_max_sample_rate. Assuming %d\n".as_ptr(), max_sample_rate);
        return max_sample_rate;
    }

    let ret = fscanf(fp, c"%d".as_ptr(), &mut max_sample_rate);
    if ret == EOF {
        pr_debug(c"Can't read perf_event_max_sample_rate. Assuming 100000\n".as_ptr());
        max_sample_rate = 100000;
    }
    fclose(fp);
    max_sample_rate
}

/*
 * Bunch of IBS sample period fixes that this test exercise went in v6.15.
 * Skip the test on older kernels to distinguish between test failure due
 * to a new bug vs known failure due to older kernel.
 */
unsafe fn kernel_v6_15_or_newer() -> bool {
    let mut utsname = core::mem::MaybeUninit::<utsname>::uninit();
    let mut endptr: *mut c_char = ptr::null_mut();

    if uname(utsname.as_mut_ptr()) < 0 {
        pr_debug(c"uname() failed. [%m]".as_ptr());
        return false;
    }

    let utsname = utsname.assume_init();
    let major = strtol(utsname.release.as_ptr(), &mut endptr, 10);
    endptr = endptr.add(1);
    let minor = strtol(endptr, ptr::null_mut(), 10);

    major > 6 || (major == 6 && minor >= 15)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test__amd_ibs_period(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut perf: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut ret: c_int = TEST_OK;

    page_size = sysconf(_SC_PAGESIZE) as c_int;

    /*
     * Reading perf_event_max_sample_rate only once _might_ cause some
     * of the test to fail if kernel changes it after reading it here.
     */
    perf_event_max_sample_rate = get_perf_event_max_sample_rate();
    fetch_pmu = perf_pmus__find(c"ibs_fetch".as_ptr());
    op_pmu = perf_pmus__find(c"ibs_op".as_ptr());

    if !x86__is_amd_cpu() || fetch_pmu.is_null() || op_pmu.is_null() {
        return TEST_SKIP;
    }

    if !kernel_v6_15_or_newer() {
        pr_debug(c"Need v6.15 or newer kernel. Skipping.\n".as_ptr());
        return TEST_SKIP;
    }

    perf_exe(perf.as_mut_ptr(), size_of_val(&perf));

    if sched_affine(0) != 0 {
        return TEST_FAIL;
    }

    /*
     * Perf event can be opened in two modes:
     * 1 Freq mode
     *   perf_event_attr->freq = 1, ->sample_freq = <frequency>
     * 2 Sample period mode
     *   perf_event_attr->freq = 0, ->sample_period = <period>
     *
     * Instead of using above interface, IBS event in 'sample period mode'
     * can also be opened by passing <period> value directly in a MaxCnt
     * bitfields of perf_event_attr->config. Test this IBS specific special
     * interface.
     */
    if ibs_config_test() != 0 {
        ret = TEST_FAIL;
    }

    /*
     * IBS Fetch and Op PMUs have HW constraints on minimum sample period.
     * Also, sample period value must be in multiple of 0x10. Test that IBS
     * driver honors HW constraints for various possible values in Freq as
     * well as Sample Period mode IBS events.
     */
    if ibs_period_constraint_test() != 0 {
        ret = TEST_FAIL;
    }

    /*
     * Test ioctl() with various sample period values for IBS event.
     */
    if ibs_ioctl_test() != 0 {
        ret = TEST_FAIL;
    }

    /*
     * Test that opening of freq mode IBS event fails when the freq value
     * is passed through ->config, not explicitly in ->sample_freq. Also
     * use high freq value (beyond perf_event_max_sample_rate) to test IBS
     * driver do not bypass perf_event_max_sample_rate checks.
     */
    if ibs_freq_neg_test() != 0 {
        ret = TEST_FAIL;
    }

    /*
     * L3MissOnly is a post-processing filter, i.e. IBS HW checks for L3
     * Miss at the completion of the tagged uOp. The sample is discarded
     * if the tagged uOp did not cause L3Miss. Also, IBS HW internally
     * resets CurCnt to a small pseudo-random value and resumes counting.
     * A new uOp is tagged once CurCnt reaches to MaxCnt. But the process
     * repeats until the tagged uOp causes an L3 Miss.
     *
     * With the freq mode event, the next sample period is calculated by
     * generic kernel on every sample to achieve desired freq of samples.
     *
     * Since the number of times HW internally reset CurCnt and the pseudo-
     * random value of CurCnt for all those occurrences are not known to SW,
     * the sample period adjustment by kernel goes for a toes for freq mode
     * IBS events. Kernel will set very small period for the next sample if
     * the window between current sample and prev sample is too high due to
     * multiple samples being discarded internally by IBS HW.
     *
     * Test that IBS sample period constraints are honored when L3MissOnly
     * is ON.
     */
    if ibs_l3missonly_test(perf.as_mut_ptr()) != 0 {
        ret = TEST_FAIL;
    }

    ret
}
