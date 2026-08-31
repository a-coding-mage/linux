// SPDX-License-Identifier: GPL-2.0
/*
 * Real Time Clock Driver Test Program
 *
 * Copyright (c) 2018 Alexandre Belloni <alexandre.belloni@bootlin.com>
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const NUM_UIE: c_int = 3;
const ALARM_DELTA: i64 = 3;
const READ_LOOP_DURATION_SEC: i64 = 30;
const READ_LOOP_SLEEP_MS: c_long = 11;

static mut rtc_file: *const c_char = b"/dev/rtc0\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum rtc_alarm_state {
    RTC_ALARM_UNKNOWN,
    RTC_ALARM_ENABLED,
    RTC_ALARM_DISABLED,
    RTC_ALARM_RES_MINUTE,
}

#[repr(C)]
struct rtc {
    fd: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct rtc_time {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
}

#[repr(C)]
struct rtc_wkalrm {
    enabled: c_uchar,
    pending: c_uchar,
    time: rtc_time,
}

type c_uchar = u8;
type time_t = c_long;

#[repr(C)]
#[derive(Clone, Copy)]
struct tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
}

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: c_long,
}

#[repr(C)]
union rtc_param_value {
    uvalue: u64,
    svalue: i64,
    ptr: *mut c_void,
}

#[repr(C)]
struct rtc_param {
    param: u64,
    uvalue: u64,
    index: u32,
    __pad: u32,
}

#[repr(C)]
struct fd_set {
    fds_bits: [c_long; 16],
}

const O_RDONLY: c_int = 0;
const R_OK: c_int = 4;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;

/* Constants normally supplied by <linux/rtc.h>. */
const RTC_RD_TIME: c_ulong = 0x8024_7009;
const RTC_ALM_SET: c_ulong = 0x4024_7007;
const RTC_ALM_READ: c_ulong = 0x8024_7008;
const RTC_AIE_ON: c_ulong = 0x7001;
const RTC_AIE_OFF: c_ulong = 0x7002;
const RTC_UIE_ON: c_ulong = 0x7003;
const RTC_UIE_OFF: c_ulong = 0x7004;
const RTC_WKALM_SET: c_ulong = 0x4028_700f;
const RTC_WKALM_RD: c_ulong = 0x8028_7010;
const RTC_PARAM_GET: c_ulong = 0x8028_7013;
const RTC_PARAM_FEATURES: u64 = 0;
const RTC_FEATURE_ALARM: u64 = 0;
const RTC_FEATURE_ALARM_RES_MINUTE: u64 = 1;

unsafe extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *mut timeval,
    ) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int;
    fn mktime(tm: *mut tm) -> time_t;
    fn timegm(tm: *mut tm) -> time_t;
    fn gmtime_r(timep: *const time_t, result: *mut tm) -> *mut tm;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;

    static mut stderr: *mut c_void;

    fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn ksft_exit_skip(format: *const c_char, ...) -> !;
}

macro_rules! TH_LOG {
    ($($arg:tt)*) => {
        /* Provided by kselftest_harness.h in the original C source. */
    };
}

macro_rules! ASSERT_NE {
    ($($arg:tt)*) => {
        /* Provided by kselftest_harness.h in the original C source. */
    };
}

macro_rules! ASSERT_EQ {
    ($($arg:tt)*) => {
        /* Provided by kselftest_harness.h in the original C source. */
    };
}

macro_rules! ASSERT_LE {
    ($($arg:tt)*) => {
        /* Provided by kselftest_harness.h in the original C source. */
    };
}

macro_rules! ASSERT_GE {
    ($($arg:tt)*) => {
        /* Provided by kselftest_harness.h in the original C source. */
    };
}

macro_rules! EXPECT_EQ {
    ($($arg:tt)*) => {
        /* Provided by kselftest_harness.h in the original C source. */
    };
}

macro_rules! SKIP {
    (return, $($arg:tt)*) => {
        return
    };
}

const fn _BITUL(nr: u64) -> u64 {
    1_u64 << nr
}

unsafe fn FD_ZERO(set: *mut fd_set) {
    (*set).fds_bits = [0; 16];
}

unsafe fn FD_SET(fd: c_int, set: *mut fd_set) {
    let fd = fd as usize;
    let bits_per_long = 8 * core::mem::size_of::<c_long>();
    (*set).fds_bits[fd / bits_per_long] |= 1_c_long << (fd % bits_per_long);
}

unsafe fn fixture_setup_rtc(self_: *mut rtc) {
    (*self_).fd = open(rtc_file, O_RDONLY);
}

unsafe fn fixture_teardown_rtc(self_: *mut rtc) {
    close((*self_).fd);
}

unsafe fn test_rtc_date_read(self_: *mut rtc) {
    let mut rc: c_int;
    let mut rtc_tm: rtc_time = core::mem::zeroed();

    if (*self_).fd == -1 && errno == ENOENT {
        SKIP!(return, "Skipping test since %s does not exist", rtc_file);
    }
    ASSERT_NE!(-1, (*self_).fd);

    /* Read the RTC time/date */
    rc = ioctl((*self_).fd, RTC_RD_TIME, &mut rtc_tm);
    ASSERT_NE!(-1, rc);

    TH_LOG!(
        "Current RTC date/time is %02d/%02d/%02d %02d:%02d:%02d.",
        rtc_tm.tm_mday,
        rtc_tm.tm_mon + 1,
        rtc_tm.tm_year + 1900,
        rtc_tm.tm_hour,
        rtc_tm.tm_min,
        rtc_tm.tm_sec
    );
}

unsafe fn rtc_time_to_timestamp(rtc_time_: *mut rtc_time) -> time_t {
    let mut tm_time = tm {
        tm_sec: (*rtc_time_).tm_sec,
        tm_min: (*rtc_time_).tm_min,
        tm_hour: (*rtc_time_).tm_hour,
        tm_mday: (*rtc_time_).tm_mday,
        tm_mon: (*rtc_time_).tm_mon,
        tm_year: (*rtc_time_).tm_year,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
    };

    mktime(&mut tm_time)
}

unsafe fn nanosleep_with_retries(ns: c_long) {
    let mut req = timespec {
        tv_sec: 0,
        tv_nsec: ns,
    };
    let mut rem: timespec = core::mem::zeroed();

    while nanosleep(&req, &mut rem) != 0 {
        req.tv_sec = rem.tv_sec;
        req.tv_nsec = rem.tv_nsec;
    }
}

unsafe fn get_rtc_alarm_state(fd: c_int, need_seconds: c_int) -> rtc_alarm_state {
    let mut param: rtc_param = core::mem::zeroed();
    let mut rc: c_int;

    /* Validate kernel reflects unsupported RTC alarm state */
    param.param = RTC_PARAM_FEATURES;
    param.index = 0;
    rc = ioctl(fd, RTC_PARAM_GET, &mut param);
    if rc < 0 {
        return rtc_alarm_state::RTC_ALARM_UNKNOWN;
    }

    if (param.uvalue & _BITUL(RTC_FEATURE_ALARM)) == 0 {
        return rtc_alarm_state::RTC_ALARM_DISABLED;
    }

    /* Check if alarm has desired granularity */
    if need_seconds != 0 && (param.uvalue & _BITUL(RTC_FEATURE_ALARM_RES_MINUTE)) != 0 {
        return rtc_alarm_state::RTC_ALARM_RES_MINUTE;
    }

    rtc_alarm_state::RTC_ALARM_ENABLED
}

unsafe fn test_rtc_date_read_loop(self_: *mut rtc) {
    let mut rc: c_int;
    let mut iter_count: c_long = 0;
    let mut rtc_tm: rtc_time = core::mem::zeroed();
    let mut start_rtc_read: time_t;
    let mut prev_rtc_read: time_t;

    if (*self_).fd == -1 && errno == ENOENT {
        SKIP!(return, "Skipping test since %s does not exist", rtc_file);
    }
    ASSERT_NE!(-1, (*self_).fd);

    TH_LOG!(
        "Continuously reading RTC time for %ds (with %dms breaks after every read).",
        READ_LOOP_DURATION_SEC,
        READ_LOOP_SLEEP_MS
    );

    rc = ioctl((*self_).fd, RTC_RD_TIME, &mut rtc_tm);
    ASSERT_NE!(-1, rc);
    start_rtc_read = rtc_time_to_timestamp(&mut rtc_tm);
    prev_rtc_read = start_rtc_read;

    loop {
        let mut rtc_read: time_t;

        rc = ioctl((*self_).fd, RTC_RD_TIME, &mut rtc_tm);
        ASSERT_NE!(-1, rc);

        rtc_read = rtc_time_to_timestamp(&mut rtc_tm);
        /* Time should not go backwards */
        ASSERT_LE!(prev_rtc_read, rtc_read);
        /* Time should not increase more then 1s at a time */
        ASSERT_GE!(prev_rtc_read + 1, rtc_read);

        /* Sleep 11ms to avoid killing / overheating the RTC */
        nanosleep_with_retries(READ_LOOP_SLEEP_MS * 1000000);

        prev_rtc_read = rtc_read;
        iter_count += 1;
        if !(prev_rtc_read <= start_rtc_read + READ_LOOP_DURATION_SEC) {
            break;
        }
    }

    TH_LOG!("Performed %ld RTC time reads.", iter_count);
}

unsafe fn test_rtc_uie_read(self_: *mut rtc) {
    let mut i: c_int;
    let mut rc: c_int;
    let mut irq: c_int = 0;
    let mut data: c_ulong = 0;

    if (*self_).fd == -1 && errno == ENOENT {
        SKIP!(return, "Skipping test since %s does not exist", rtc_file);
    }
    ASSERT_NE!(-1, (*self_).fd);

    /* Turn on update interrupts */
    rc = ioctl((*self_).fd, RTC_UIE_ON, 0);
    if rc == -1 {
        ASSERT_EQ!(EINVAL, errno);
        TH_LOG!("skip update IRQs not supported.");
        return;
    }

    i = 0;
    while i < NUM_UIE {
        /* This read will block */
        rc = read(
            (*self_).fd,
            &mut data as *mut c_ulong as *mut c_void,
            core::mem::size_of_val(&data),
        ) as c_int;
        ASSERT_NE!(-1, rc);
        irq += 1;
        i += 1;
    }

    EXPECT_EQ!(NUM_UIE, irq);

    rc = ioctl((*self_).fd, RTC_UIE_OFF, 0);
    ASSERT_NE!(-1, rc);
}

unsafe fn test_rtc_uie_select(self_: *mut rtc) {
    let mut i: c_int;
    let mut rc: c_int;
    let mut irq: c_int = 0;
    let mut data: c_ulong = 0;

    if (*self_).fd == -1 && errno == ENOENT {
        SKIP!(return, "Skipping test since %s does not exist", rtc_file);
    }
    ASSERT_NE!(-1, (*self_).fd);

    /* Turn on update interrupts */
    rc = ioctl((*self_).fd, RTC_UIE_ON, 0);
    if rc == -1 {
        ASSERT_EQ!(EINVAL, errno);
        TH_LOG!("skip update IRQs not supported.");
        return;
    }

    i = 0;
    while i < NUM_UIE {
        let mut tv = timeval {
            tv_sec: 2,
            tv_usec: 0,
        };
        let mut readfds: fd_set = core::mem::zeroed();

        FD_ZERO(&mut readfds);
        FD_SET((*self_).fd, &mut readfds);
        /* The select will wait until an RTC interrupt happens. */
        rc = select(
            (*self_).fd + 1,
            &mut readfds,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &mut tv,
        );
        ASSERT_NE!(-1, rc);
        ASSERT_NE!(0, rc);

        /* This read won't block */
        rc = read(
            (*self_).fd,
            &mut data as *mut c_ulong as *mut c_void,
            core::mem::size_of::<c_ulong>(),
        ) as c_int;
        ASSERT_NE!(-1, rc);
        irq += 1;
        i += 1;
    }

    EXPECT_EQ!(NUM_UIE, irq);

    rc = ioctl((*self_).fd, RTC_UIE_OFF, 0);
    ASSERT_NE!(-1, rc);
}

unsafe fn test_rtc_alarm_alm_set(self_: *mut rtc) {
    let mut tv = timeval {
        tv_sec: ALARM_DELTA + 2,
        tv_usec: 0,
    };
    let mut data: c_ulong = 0;
    let mut tm_: rtc_time = core::mem::zeroed();
    let mut readfds: fd_set = core::mem::zeroed();
    let mut secs: time_t;
    let mut new: time_t;
    let mut rc: c_int;
    let mut alarm_state = rtc_alarm_state::RTC_ALARM_UNKNOWN;

    if (*self_).fd == -1 && errno == ENOENT {
        SKIP!(return, "Skipping test since %s does not exist", rtc_file);
    }
    ASSERT_NE!(-1, (*self_).fd);

    alarm_state = get_rtc_alarm_state((*self_).fd, 1);
    if alarm_state == rtc_alarm_state::RTC_ALARM_DISABLED {
        SKIP!(return, "Skipping test since alarms are not supported.");
    }
    if alarm_state == rtc_alarm_state::RTC_ALARM_RES_MINUTE {
        SKIP!(
            return,
            "Skipping test since alarms has only minute granularity."
        );
    }

    rc = ioctl((*self_).fd, RTC_RD_TIME, &mut tm_);
    ASSERT_NE!(-1, rc);

    secs = timegm(&mut tm_ as *mut rtc_time as *mut tm) + ALARM_DELTA;
    gmtime_r(&secs, &mut tm_ as *mut rtc_time as *mut tm);

    rc = ioctl((*self_).fd, RTC_ALM_SET, &mut tm_);
    if rc == -1 {
        /*
         * Report error if rtc alarm was enabled. Fallback to check ioctl
         * error number if rtc alarm state is unknown.
         */
        ASSERT_EQ!(rtc_alarm_state::RTC_ALARM_UNKNOWN, alarm_state);
        ASSERT_EQ!(EINVAL, errno);
        TH_LOG!("skip alarms are not supported.");
        return;
    }

    rc = ioctl((*self_).fd, RTC_ALM_READ, &mut tm_);
    ASSERT_NE!(-1, rc);

    TH_LOG!(
        "Alarm time now set to %02d:%02d:%02d.",
        tm_.tm_hour,
        tm_.tm_min,
        tm_.tm_sec
    );

    /* Enable alarm interrupts */
    rc = ioctl((*self_).fd, RTC_AIE_ON, 0);
    ASSERT_NE!(-1, rc);

    FD_ZERO(&mut readfds);
    FD_SET((*self_).fd, &mut readfds);

    rc = select(
        (*self_).fd + 1,
        &mut readfds,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &mut tv,
    );
    ASSERT_NE!(-1, rc);
    ASSERT_NE!(0, rc);

    /* Disable alarm interrupts */
    rc = ioctl((*self_).fd, RTC_AIE_OFF, 0);
    ASSERT_NE!(-1, rc);

    rc = read(
        (*self_).fd,
        &mut data as *mut c_ulong as *mut c_void,
        core::mem::size_of::<c_ulong>(),
    ) as c_int;
    ASSERT_NE!(-1, rc);
    TH_LOG!("data: %lx", data);

    rc = ioctl((*self_).fd, RTC_RD_TIME, &mut tm_);
    ASSERT_NE!(-1, rc);

    new = timegm(&mut tm_ as *mut rtc_time as *mut tm);
    ASSERT_EQ!(new, secs);
}

unsafe fn test_rtc_alarm_wkalm_set(self_: *mut rtc) {
    let mut tv = timeval {
        tv_sec: ALARM_DELTA + 2,
        tv_usec: 0,
    };
    let mut alarm: rtc_wkalrm = core::mem::zeroed();
    let mut tm_: rtc_time = core::mem::zeroed();
    let mut data: c_ulong = 0;
    let mut readfds: fd_set = core::mem::zeroed();
    let mut secs: time_t;
    let mut new: time_t;
    let mut rc: c_int;
    let mut alarm_state = rtc_alarm_state::RTC_ALARM_UNKNOWN;

    if (*self_).fd == -1 && errno == ENOENT {
        SKIP!(return, "Skipping test since %s does not exist", rtc_file);
    }
    ASSERT_NE!(-1, (*self_).fd);

    alarm_state = get_rtc_alarm_state((*self_).fd, 1);
    if alarm_state == rtc_alarm_state::RTC_ALARM_DISABLED {
        SKIP!(return, "Skipping test since alarms are not supported.");
    }
    if alarm_state == rtc_alarm_state::RTC_ALARM_RES_MINUTE {
        SKIP!(
            return,
            "Skipping test since alarms has only minute granularity."
        );
    }

    rc = ioctl((*self_).fd, RTC_RD_TIME, &mut alarm.time);
    ASSERT_NE!(-1, rc);

    secs = timegm(&mut alarm.time as *mut rtc_time as *mut tm) + ALARM_DELTA;
    gmtime_r(&secs, &mut alarm.time as *mut rtc_time as *mut tm);

    alarm.enabled = 1;

    rc = ioctl((*self_).fd, RTC_WKALM_SET, &mut alarm);
    if rc == -1 {
        /*
         * Report error if rtc alarm was enabled. Fallback to check ioctl
         * error number if rtc alarm state is unknown.
         */
        ASSERT_EQ!(rtc_alarm_state::RTC_ALARM_UNKNOWN, alarm_state);
        ASSERT_EQ!(EINVAL, errno);
        TH_LOG!("skip alarms are not supported.");
        return;
    }

    rc = ioctl((*self_).fd, RTC_WKALM_RD, &mut alarm);
    ASSERT_NE!(-1, rc);

    TH_LOG!(
        "Alarm time now set to %02d/%02d/%02d %02d:%02d:%02d.",
        alarm.time.tm_mday,
        alarm.time.tm_mon + 1,
        alarm.time.tm_year + 1900,
        alarm.time.tm_hour,
        alarm.time.tm_min,
        alarm.time.tm_sec
    );

    FD_ZERO(&mut readfds);
    FD_SET((*self_).fd, &mut readfds);

    rc = select(
        (*self_).fd + 1,
        &mut readfds,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &mut tv,
    );
    ASSERT_NE!(-1, rc);
    ASSERT_NE!(0, rc);

    rc = read(
        (*self_).fd,
        &mut data as *mut c_ulong as *mut c_void,
        core::mem::size_of::<c_ulong>(),
    ) as c_int;
    ASSERT_NE!(-1, rc);

    rc = ioctl((*self_).fd, RTC_RD_TIME, &mut tm_);
    ASSERT_NE!(-1, rc);

    new = timegm(&mut tm_ as *mut rtc_time as *mut tm);
    ASSERT_EQ!(new, secs);
}

unsafe fn test_rtc_alarm_alm_set_minute(self_: *mut rtc) {
    let mut tv = timeval {
        tv_sec: 62,
        tv_usec: 0,
    };
    let mut data: c_ulong = 0;
    let mut tm_: rtc_time = core::mem::zeroed();
    let mut readfds: fd_set = core::mem::zeroed();
    let mut secs: time_t;
    let mut new: time_t;
    let mut rc: c_int;
    let mut alarm_state = rtc_alarm_state::RTC_ALARM_UNKNOWN;

    if (*self_).fd == -1 && errno == ENOENT {
        SKIP!(return, "Skipping test since %s does not exist", rtc_file);
    }
    ASSERT_NE!(-1, (*self_).fd);

    alarm_state = get_rtc_alarm_state((*self_).fd, 0);
    if alarm_state == rtc_alarm_state::RTC_ALARM_DISABLED {
        SKIP!(return, "Skipping test since alarms are not supported.");
    }

    rc = ioctl((*self_).fd, RTC_RD_TIME, &mut tm_);
    ASSERT_NE!(-1, rc);

    secs = timegm(&mut tm_ as *mut rtc_time as *mut tm) + 60 - tm_.tm_sec as time_t;
    gmtime_r(&secs, &mut tm_ as *mut rtc_time as *mut tm);

    rc = ioctl((*self_).fd, RTC_ALM_SET, &mut tm_);
    if rc == -1 {
        /*
         * Report error if rtc alarm was enabled. Fallback to check ioctl
         * error number if rtc alarm state is unknown.
         */
        ASSERT_EQ!(rtc_alarm_state::RTC_ALARM_UNKNOWN, alarm_state);
        ASSERT_EQ!(EINVAL, errno);
        TH_LOG!("skip alarms are not supported.");
        return;
    }

    rc = ioctl((*self_).fd, RTC_ALM_READ, &mut tm_);
    ASSERT_NE!(-1, rc);

    TH_LOG!(
        "Alarm time now set to %02d:%02d:%02d.",
        tm_.tm_hour,
        tm_.tm_min,
        tm_.tm_sec
    );

    /* Enable alarm interrupts */
    rc = ioctl((*self_).fd, RTC_AIE_ON, 0);
    ASSERT_NE!(-1, rc);

    FD_ZERO(&mut readfds);
    FD_SET((*self_).fd, &mut readfds);

    rc = select(
        (*self_).fd + 1,
        &mut readfds,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &mut tv,
    );
    ASSERT_NE!(-1, rc);
    ASSERT_NE!(0, rc);

    /* Disable alarm interrupts */
    rc = ioctl((*self_).fd, RTC_AIE_OFF, 0);
    ASSERT_NE!(-1, rc);

    rc = read(
        (*self_).fd,
        &mut data as *mut c_ulong as *mut c_void,
        core::mem::size_of::<c_ulong>(),
    ) as c_int;
    ASSERT_NE!(-1, rc);
    TH_LOG!("data: %lx", data);

    rc = ioctl((*self_).fd, RTC_RD_TIME, &mut tm_);
    ASSERT_NE!(-1, rc);

    new = timegm(&mut tm_ as *mut rtc_time as *mut tm);
    ASSERT_EQ!(new, secs);
}

unsafe fn test_rtc_alarm_wkalm_set_minute(self_: *mut rtc) {
    let mut tv = timeval {
        tv_sec: 62,
        tv_usec: 0,
    };
    let mut alarm: rtc_wkalrm = core::mem::zeroed();
    let mut tm_: rtc_time = core::mem::zeroed();
    let mut data: c_ulong = 0;
    let mut readfds: fd_set = core::mem::zeroed();
    let mut secs: time_t;
    let mut new: time_t;
    let mut rc: c_int;
    let mut alarm_state = rtc_alarm_state::RTC_ALARM_UNKNOWN;

    if (*self_).fd == -1 && errno == ENOENT {
        SKIP!(return, "Skipping test since %s does not exist", rtc_file);
    }
    ASSERT_NE!(-1, (*self_).fd);

    alarm_state = get_rtc_alarm_state((*self_).fd, 0);
    if alarm_state == rtc_alarm_state::RTC_ALARM_DISABLED {
        SKIP!(return, "Skipping test since alarms are not supported.");
    }

    rc = ioctl((*self_).fd, RTC_RD_TIME, &mut alarm.time);
    ASSERT_NE!(-1, rc);

    secs = timegm(&mut alarm.time as *mut rtc_time as *mut tm)
        + 60
        - alarm.time.tm_sec as time_t;
    gmtime_r(&secs, &mut alarm.time as *mut rtc_time as *mut tm);

    alarm.enabled = 1;

    rc = ioctl((*self_).fd, RTC_WKALM_SET, &mut alarm);
    if rc == -1 {
        /*
         * Report error if rtc alarm was enabled. Fallback to check ioctl
         * error number if rtc alarm state is unknown.
         */
        ASSERT_EQ!(rtc_alarm_state::RTC_ALARM_UNKNOWN, alarm_state);
        ASSERT_EQ!(EINVAL, errno);
        TH_LOG!("skip alarms are not supported.");
        return;
    }

    rc = ioctl((*self_).fd, RTC_WKALM_RD, &mut alarm);
    ASSERT_NE!(-1, rc);

    TH_LOG!(
        "Alarm time now set to %02d/%02d/%02d %02d:%02d:%02d.",
        alarm.time.tm_mday,
        alarm.time.tm_mon + 1,
        alarm.time.tm_year + 1900,
        alarm.time.tm_hour,
        alarm.time.tm_min,
        alarm.time.tm_sec
    );

    FD_ZERO(&mut readfds);
    FD_SET((*self_).fd, &mut readfds);

    rc = select(
        (*self_).fd + 1,
        &mut readfds,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &mut tv,
    );
    ASSERT_NE!(-1, rc);
    ASSERT_NE!(0, rc);

    rc = read(
        (*self_).fd,
        &mut data as *mut c_ulong as *mut c_void,
        core::mem::size_of::<c_ulong>(),
    ) as c_int;
    ASSERT_NE!(-1, rc);

    rc = ioctl((*self_).fd, RTC_RD_TIME, &mut tm_);
    ASSERT_NE!(-1, rc);

    new = timegm(&mut tm_ as *mut rtc_time as *mut tm);
    ASSERT_EQ!(new, secs);
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int = -1;

    match argc {
        2 => {
            rtc_file = *argv.add(1);
            /* FALLTHROUGH */
        }
        1 => {}
        _ => {
            fprintf(
                stderr,
                b"usage: %s [rtcdev]\n\0".as_ptr() as *const c_char,
                *argv.add(0),
            );
            return 1;
        }
    }

    /* Run the test if rtc_file is accessible */
    if access(rtc_file, R_OK) == 0 {
        ret = test_harness_run(argc, argv);
    } else {
        ksft_exit_skip(
            b"[SKIP]: Cannot access rtc file %s - Exiting\n\0".as_ptr() as *const c_char,
            rtc_file,
        );
    }

    ret
}

unsafe fn main() {
    unsafe extern "C" {
        static mut __argc: c_int;
        static mut __argv: *mut *mut c_char;
    }

    let _ = main_0(__argc, __argv);
}
