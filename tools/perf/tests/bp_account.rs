// SPDX-License-Identifier: GPL-2.0
/*
 * Powerpc needs __SANE_USERSPACE_TYPES__ before <linux/types.h> to select
 * 'int-ll64.h' and avoid compile warnings when printing __u64 with %llu.
 *
 * C source defined __SANE_USERSPACE_TYPES__ before including Linux headers.
 */

/*
 * External dependencies from the original C includes:
 * stdlib.h, stdio.h, unistd.h, string.h, sys/ioctl.h, fcntl.h,
 * linux/hw_breakpoint.h, tests.h, debug.h, event.h, parse-events.h,
 * ../perf-sys.h, cloexec.h.
 */

use core::ffi::{c_int, c_long, c_ulong, c_void};

const PERF_TYPE_BREAKPOINT: u32 = 5;
const PERF_SAMPLE_IP: u64 = 1 << 0;
const HW_BREAKPOINT_X: u32 = 4;
const HW_BREAKPOINT_W: u32 = 2;
const PERF_EVENT_IOC_MODIFY_ATTRIBUTES: c_ulong = 0;

const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = -2;

/*
 * PowerPC and S390 do not support creation of instruction breakpoints using the
 * perf_event interface.
 *
 * Just disable the test for these architectures until these issues are
 * resolved.
 */
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x"))]
const BP_ACCOUNT_IS_SUPPORTED: c_int = 0;
#[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x")))]
const BP_ACCOUNT_IS_SUPPORTED: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub bp_type: u32,
    pub bp_addr: u64,
    pub bp_len: u64,
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn default_breakpoint_len() -> u64;
    fn sys_perf_event_open(
        attr: *mut perf_event_attr,
        pid: c_int,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    fn perf_event_open_cloexec_flag() -> c_ulong;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

macro_rules! pr_debug {
    ($($arg:tt)*) => {
        {
            let _ = format_args!($($arg)*);
        }
    };
}

macro_rules! pr_err {
    ($($arg:tt)*) => {
        {
            let _ = format_args!($($arg)*);
        }
    };
}

macro_rules! TEST_ASSERT_VAL {
    ($msg:expr, $cond:expr) => {
        if !($cond) {
            return TEST_FAIL;
        }
    };
}

static mut the_var: c_long = 0;

#[inline(never)]
unsafe extern "C" fn test_function() -> c_int {
    return 0;
}

unsafe fn __event(is_x: bool, addr: *mut c_void, attr: *mut perf_event_attr) -> c_int {
    let fd: c_int;

    core::ptr::write_bytes(attr as *mut u8, 0, core::mem::size_of::<perf_event_attr>());
    (*attr).type_ = PERF_TYPE_BREAKPOINT;
    (*attr).size = core::mem::size_of::<perf_event_attr>() as u32;

    (*attr).config = 0;
    (*attr).bp_type = if is_x { HW_BREAKPOINT_X } else { HW_BREAKPOINT_W };
    (*attr).bp_addr = addr as c_ulong as u64;
    (*attr).bp_len = if is_x {
        default_breakpoint_len()
    } else {
        core::mem::size_of::<c_long>() as u64
    };

    (*attr).sample_period = 1;
    (*attr).sample_type = PERF_SAMPLE_IP;

    (*attr).exclude_kernel = 1;
    (*attr).exclude_hv = 1;

    fd = sys_perf_event_open(attr, -1, 0, -1, perf_event_open_cloexec_flag());
    if fd < 0 {
        pr_debug!("failed opening event {:x}\n", (*attr).config);
        return TEST_FAIL;
    }

    return fd;
}

unsafe fn wp_event(addr: *mut c_void, attr: *mut perf_event_attr) -> c_int {
    return __event(false, addr, attr);
}

unsafe fn bp_event(addr: *mut c_void, attr: *mut perf_event_attr) -> c_int {
    return __event(true, addr, attr);
}

unsafe fn bp_accounting(wp_cnt: c_int, share: c_int) -> c_int {
    let mut attr: perf_event_attr = core::mem::zeroed();
    let mut attr_mod: perf_event_attr;
    let mut attr_new: perf_event_attr = core::mem::zeroed();
    let mut i: c_int;
    let mut fd: Vec<c_int> = vec![0; wp_cnt as usize];
    let fd_wp: c_int;
    let ret: c_int;

    i = 0;
    while i < wp_cnt {
        fd[i as usize] = wp_event(core::ptr::addr_of_mut!(the_var) as *mut c_void, &mut attr);
        TEST_ASSERT_VAL!("failed to create wp\n", fd[i as usize] != -1);
        pr_debug!("wp {} created\n", i);
        i += 1;
    }

    attr_mod = attr;
    attr_mod.bp_type = HW_BREAKPOINT_X;
    attr_mod.bp_addr = test_function as usize as c_ulong as u64;
    attr_mod.bp_len = default_breakpoint_len();

    ret = ioctl(
        fd[0],
        PERF_EVENT_IOC_MODIFY_ATTRIBUTES,
        &mut attr_mod as *mut perf_event_attr,
    );
    TEST_ASSERT_VAL!("failed to modify wp\n", ret == 0);

    pr_debug!("wp 0 modified to bp\n");

    if share == 0 {
        fd_wp = wp_event(
            core::ptr::addr_of_mut!(the_var) as *mut c_void,
            &mut attr_new,
        );
        TEST_ASSERT_VAL!("failed to create max wp\n", fd_wp != -1);
        pr_debug!("wp max created\n");
        close(fd_wp);
    }

    i = 0;
    while i < wp_cnt {
        close(fd[i as usize]);
        i += 1;
    }

    return 0;
}

unsafe fn detect_cnt(is_x: bool) -> c_int {
    let mut attr: perf_event_attr = core::mem::zeroed();
    let addr: *mut c_void = if is_x {
        test_function as usize as *mut c_void
    } else {
        core::ptr::addr_of_mut!(the_var) as *mut c_void
    };
    let mut fd: [c_int; 100] = [0; 100];
    let mut cnt: c_int = 0;
    let mut i: c_int;

    loop {
        if cnt == 100 {
            pr_debug!("way too many debug registers, fix the test\n");
            return 0;
        }
        fd[cnt as usize] = __event(is_x, addr, &mut attr);

        if fd[cnt as usize] < 0 {
            break;
        }
        cnt += 1;
    }

    i = 0;
    while i < cnt {
        close(fd[i as usize]);
        i += 1;
    }

    return cnt;
}

unsafe fn detect_ioctl() -> c_int {
    let mut attr: perf_event_attr = core::mem::zeroed();
    let fd: c_int;
    let mut ret: c_int = 1;

    fd = wp_event(core::ptr::addr_of_mut!(the_var) as *mut c_void, &mut attr);
    if fd > 0 {
        ret = ioctl(fd, PERF_EVENT_IOC_MODIFY_ATTRIBUTES, &mut attr as *mut perf_event_attr);
        close(fd);
    }

    return if ret != 0 { 0 } else { 1 };
}

unsafe fn detect_share(wp_cnt: c_int, bp_cnt: c_int) -> c_int {
    let mut attr: perf_event_attr = core::mem::zeroed();
    let mut i: c_int;
    let mut fd: *mut c_int = core::ptr::null_mut();
    let mut ret: c_int = -1;

    if wp_cnt + bp_cnt == 0 {
        return 0;
    }

    fd = malloc(core::mem::size_of::<c_int>() * (wp_cnt + bp_cnt) as usize) as *mut c_int;
    if fd.is_null() {
        return -1;
    }

    i = 0;
    while i < wp_cnt {
        *fd.add(i as usize) = wp_event(core::ptr::addr_of_mut!(the_var) as *mut c_void, &mut attr);
        if *fd.add(i as usize) == -1 {
            pr_err!("failed to create wp\n");
            goto_out(&mut i, fd);
            free(fd as *mut c_void);
            return ret;
        }
        i += 1;
    }

    while i < bp_cnt + wp_cnt {
        *fd.add(i as usize) = bp_event(test_function as usize as *mut c_void, &mut attr);
        if *fd.add(i as usize) == -1 {
            break;
        }
        i += 1;
    }

    ret = (i != bp_cnt + wp_cnt) as c_int;

    goto_out(&mut i, fd);

    free(fd as *mut c_void);
    return ret;
}

unsafe fn goto_out(i: &mut c_int, fd: *mut c_int) {
    while *i != 0 {
        *i -= 1;
        close(*fd.add(*i as usize));
    }
}

/*
 * This test does following:
 *   - detects the number of watch/break-points,
 *     skip test if any is missing
 *   - detects PERF_EVENT_IOC_MODIFY_ATTRIBUTES ioctl,
 *     skip test if it's missing
 *   - detects if watchpoints and breakpoints share
 *     same slots
 *   - create all possible watchpoints on cpu 0
 *   - change one of it to breakpoint
 *   - in case wp and bp do not share slots,
 *     we create another watchpoint to ensure
 *     the slot accounting is correct
 */
unsafe fn test__bp_accounting(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let has_ioctl: c_int = detect_ioctl();
    let wp_cnt: c_int = detect_cnt(false);
    let bp_cnt: c_int = detect_cnt(true);
    let share: c_int = detect_share(wp_cnt, bp_cnt);

    if BP_ACCOUNT_IS_SUPPORTED == 0 {
        pr_debug!("Test not supported on this architecture");
        return TEST_SKIP;
    }

    pr_debug!(
        "watchpoints count {}, breakpoints count {}, has_ioctl {}, share {}\n",
        wp_cnt,
        bp_cnt,
        has_ioctl,
        share
    );

    if wp_cnt == 0 || bp_cnt == 0 || has_ioctl == 0 {
        return TEST_SKIP;
    }

    return bp_accounting(wp_cnt, share);
}

/* DEFINE_SUITE("Breakpoint accounting", bp_accounting); */
