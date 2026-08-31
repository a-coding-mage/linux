// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/time-utils.c. External declarations correspond to
// symbols provided by the surrounding perf sources and C library.

use core::ffi::{c_char, c_double, c_int, c_long, c_ulong, c_void};
use core::{mem, ptr};

type size_t = usize;

const NSEC_PER_SEC: u64 = 1_000_000_000;
const NSEC_PER_USEC: u64 = 1_000;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct perf_time_interval {
    pub start: u64,
    pub end: u64,
}

#[repr(C)]
pub struct perf_session {
    pub evlist: *mut evlist,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    #[cfg(any(target_env = "gnu", target_env = "musl"))]
    pub tm_gmtoff: c_long,
    #[cfg(any(target_env = "gnu", target_env = "musl"))]
    pub tm_zone: *const c_char,
}

unsafe extern "C" {
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
    fn strlen(s: *const c_char) -> size_t;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn round(x: c_double) -> c_double;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn localtime_r(timep: *const c_long, result: *mut tm) -> *mut tm;
    fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const tm) -> size_t;

    fn skip_spaces(str_: *const c_char) -> *mut c_char;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn evlist__first_sample_time(evlist: *mut evlist) -> u64;
    fn evlist__last_sample_time(evlist: *mut evlist) -> u64;
}

unsafe fn isspace(c: c_char) -> bool {
    matches!(
        c as u8,
        b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c
    )
}

unsafe fn BUG_ON(cond: bool) {
    if cond {
        panic!("BUG_ON");
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_nsec_time(str_: *const c_char, ptime: *mut u64) -> c_int {
    let time_sec: u64;
    let time_nsec: u64;
    let mut end: *mut c_char = ptr::null_mut();

    time_sec = strtoull(str_, &mut end, 10) as u64;
    if *end != b'.' as c_char && *end != b'\0' as c_char {
        return -1;
    }

    if *end == b'.' as c_char {
        let mut i: c_int;
        let mut nsec_buf = [0 as c_char; 10];

        end = end.add(1);
        if strlen(end) > 9 {
            return -1;
        }

        strncpy(nsec_buf.as_mut_ptr(), end, 9);
        nsec_buf[9] = b'\0' as c_char;

        /* make it nsec precision */
        i = strlen(nsec_buf.as_ptr()) as c_int;
        while i < 9 {
            nsec_buf[i as usize] = b'0' as c_char;
            i += 1;
        }

        time_nsec = strtoull(nsec_buf.as_ptr(), &mut end, 10) as u64;
        if *end != b'\0' as c_char {
            return -1;
        }
    } else {
        time_nsec = 0;
    }

    *ptime = time_sec.wrapping_mul(NSEC_PER_SEC).wrapping_add(time_nsec);
    0
}

unsafe fn parse_timestr_sec_nsec(
    ptime: *mut perf_time_interval,
    start_str: *mut c_char,
    end_str: *mut c_char,
) -> c_int {
    if !start_str.is_null()
        && *start_str != b'\0' as c_char
        && parse_nsec_time(start_str, &mut (*ptime).start) != 0
    {
        return -1;
    }

    if !end_str.is_null()
        && *end_str != b'\0' as c_char
        && parse_nsec_time(end_str, &mut (*ptime).end) != 0
    {
        return -1;
    }

    0
}

unsafe fn split_start_end(
    start: *mut *mut c_char,
    end: *mut *mut c_char,
    ostr: *const c_char,
    ch: c_char,
) -> c_int {
    let start_str: *mut c_char;
    let end_str: *mut c_char;
    let mut d: *mut c_char;
    let str_: *mut c_char;

    if ostr.is_null() || *ostr == b'\0' as c_char {
        return 0;
    }

    /* copy original string because we need to modify it */
    str_ = strdup(ostr);
    if str_.is_null() {
        return -ENOMEM;
    }

    start_str = str_;
    d = strchr(start_str, ch as c_int);
    if !d.is_null() {
        *d = b'\0' as c_char;
        d = d.add(1);
    }
    end_str = d;

    *start = start_str;
    *end = end_str;

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_time__parse_str(
    ptime: *mut perf_time_interval,
    ostr: *const c_char,
) -> c_int {
    let mut start_str: *mut c_char = ptr::null_mut();
    let mut end_str: *mut c_char = ptr::null_mut();
    let mut rc: c_int;

    rc = split_start_end(&mut start_str, &mut end_str, ostr, b',' as c_char);
    if rc != 0 || start_str.is_null() {
        return rc;
    }

    (*ptime).start = 0;
    (*ptime).end = 0;

    rc = parse_timestr_sec_nsec(ptime, start_str, end_str);

    free(start_str as *mut c_void);

    /* make sure end time is after start time if it was given */
    if rc == 0 && (*ptime).end != 0 && (*ptime).end < (*ptime).start {
        return -EINVAL;
    }

    pr_debug(c"start time %llu, ".as_ptr(), (*ptime).start);
    pr_debug(c"end time %llu\n".as_ptr(), (*ptime).end);

    rc
}

unsafe fn perf_time__parse_strs(
    ptime: *mut perf_time_interval,
    ostr: *const c_char,
    size: c_int,
) -> c_int {
    let mut cp: *const c_char;
    let str_: *mut c_char;
    let mut arg: *mut c_char;
    let mut p: *mut c_char;
    let mut i: c_int;
    let mut num: c_int = 0;
    let mut rc: c_int = 0;

    /* Count the commas */
    cp = ostr;
    while *cp != 0 {
        num += (*cp == b',' as c_char) as c_int;
        cp = cp.add(1);
    }

    if num == 0 {
        return -EINVAL;
    }

    BUG_ON(num > size);

    str_ = strdup(ostr);
    if str_.is_null() {
        return -ENOMEM;
    }

    /* Split the string and parse each piece, except the last */
    i = 0;
    p = str_;
    while i < num - 1 {
        arg = p;
        /* Find next comma, there must be one */
        p = skip_spaces(strchr(p, b',' as c_int).add(1));
        /* Skip the value, must not contain space or comma */
        while *p != 0 && !isspace(*p) {
            let old = p;
            p = p.add(1);
            if *old == b',' as c_char {
                rc = -EINVAL;
                goto_out(str_, rc);
                return rc;
            }
        }
        /* Split and parse */
        if *p != 0 {
            *p = 0;
            p = p.add(1);
        }
        rc = perf_time__parse_str(ptime.add(i as usize), arg);
        if rc < 0 {
            free(str_ as *mut c_void);
            return rc;
        }
        i += 1;
    }

    /* Parse the last piece */
    rc = perf_time__parse_str(ptime.add(i as usize), p);
    if rc < 0 {
        free(str_ as *mut c_void);
        return rc;
    }

    /* Check there is no overlap */
    i = 0;
    while i < num - 1 {
        if (*ptime.add(i as usize)).end >= (*ptime.add((i + 1) as usize)).start {
            rc = -EINVAL;
            free(str_ as *mut c_void);
            return rc;
        }
        i += 1;
    }

    rc = num;
    free(str_ as *mut c_void);

    rc
}

unsafe fn goto_out(str_: *mut c_char, rc: c_int) {
    free(str_ as *mut c_void);
    let _ = rc;
}

unsafe fn parse_percent(pcnt: *mut c_double, str_: *mut c_char) -> c_int {
    let c: *mut c_char;
    let mut endptr: *mut c_char = ptr::null_mut();
    let d: c_double;

    c = strchr(str_, b'%' as c_int);
    if !c.is_null() {
        *c = b'\0' as c_char;
    } else {
        return -1;
    }

    d = strtod(str_, &mut endptr);
    if endptr != str_.add(strlen(str_)) {
        return -1;
    }

    *pcnt = d / 100.0;
    0
}

unsafe fn set_percent_time(
    ptime: *mut perf_time_interval,
    start_pcnt: c_double,
    end_pcnt: c_double,
    start: u64,
    end: u64,
) -> c_int {
    let total: u64 = end.wrapping_sub(start);

    if start_pcnt < 0.0 || start_pcnt > 1.0 || end_pcnt < 0.0 || end_pcnt > 1.0 {
        return -1;
    }

    (*ptime).start = start.wrapping_add(round(start_pcnt * total as c_double) as u64);
    (*ptime).end = start.wrapping_add(round(end_pcnt * total as c_double) as u64);

    if (*ptime).end > (*ptime).start && (*ptime).end != end {
        (*ptime).end = (*ptime).end.wrapping_sub(1);
    }

    0
}

unsafe fn percent_slash_split(
    str_: *mut c_char,
    ptime: *mut perf_time_interval,
    start: u64,
    end: u64,
) -> c_int {
    let mut p: *mut c_char;
    let mut end_str: *mut c_char = ptr::null_mut();
    let pcnt: c_double;
    let start_pcnt: c_double;
    let end_pcnt: c_double;
    let i: c_int;
    let mut pcnt_tmp: c_double = 0.0;

    /*
     * Example:
     * 10%/2: select the second 10% slice and the third 10% slice
     */

    /* We can modify this string since the original one is copied */
    p = strchr(str_, b'/' as c_int);
    if p.is_null() {
        return -1;
    }

    *p = b'\0' as c_char;
    if parse_percent(&mut pcnt_tmp, str_) < 0 {
        return -1;
    }
    pcnt = pcnt_tmp;

    p = p.add(1);
    i = strtol(p, &mut end_str, 10) as c_int;
    if *end_str != 0 {
        return -1;
    }

    if pcnt <= 0.0 {
        return -1;
    }

    start_pcnt = pcnt * (i - 1) as c_double;
    end_pcnt = pcnt * i as c_double;

    set_percent_time(ptime, start_pcnt, end_pcnt, start, end)
}

unsafe fn percent_dash_split(
    str_: *mut c_char,
    ptime: *mut perf_time_interval,
    start: u64,
    end: u64,
) -> c_int {
    let mut start_str: *mut c_char = ptr::null_mut();
    let mut end_str: *mut c_char = ptr::null_mut();
    let mut start_pcnt: c_double = 0.0;
    let mut end_pcnt: c_double = 0.0;
    let ret: c_int;

    /*
     * Example: 0%-10%
     */

    ret = split_start_end(&mut start_str, &mut end_str, str_, b'-' as c_char);
    if ret != 0 || start_str.is_null() {
        return ret;
    }

    if parse_percent(&mut start_pcnt, start_str) != 0 || parse_percent(&mut end_pcnt, end_str) != 0
    {
        free(start_str as *mut c_void);
        return -1;
    }

    free(start_str as *mut c_void);

    set_percent_time(ptime, start_pcnt, end_pcnt, start, end)
}

type time_pecent_split =
    unsafe fn(*mut c_char, *mut perf_time_interval, start: u64, end: u64) -> c_int;

unsafe fn percent_comma_split(
    ptime_buf: *mut perf_time_interval,
    num: c_int,
    ostr: *const c_char,
    start: u64,
    end: u64,
    func: time_pecent_split,
) -> c_int {
    let str_: *mut c_char;
    let mut p1: *mut c_char;
    let mut p2: *mut c_char;
    let len: c_int;
    let mut ret: c_int;
    let mut i: c_int = 0;

    str_ = strdup(ostr);
    if str_.is_null() {
        return -ENOMEM;
    }

    len = strlen(str_) as c_int;
    p1 = str_;

    while p1 < str_.add(len as usize) {
        if i >= num {
            free(str_ as *mut c_void);
            return -1;
        }

        p2 = strchr(p1, b',' as c_int);
        if !p2.is_null() {
            *p2 = b'\0' as c_char;
        }

        ret = func(p1, ptime_buf.add(i as usize), start, end);
        if ret < 0 {
            free(str_ as *mut c_void);
            return -1;
        }

        pr_debug(c"start time %d: %llu, ".as_ptr(), i, (*ptime_buf.add(i as usize)).start);
        pr_debug(c"end time %d: %llu\n".as_ptr(), i, (*ptime_buf.add(i as usize)).end);

        i += 1;

        if !p2.is_null() {
            p1 = p2.add(1);
        } else {
            break;
        }
    }

    free(str_ as *mut c_void);
    i
}

unsafe fn one_percent_convert(
    ptime_buf: *mut perf_time_interval,
    ostr: *const c_char,
    start: u64,
    end: u64,
    c: *const c_char,
) -> c_int {
    let str_: *mut c_char;
    let len: c_int = strlen(ostr) as c_int;
    let mut ret: c_int;

    /*
     * c points to '%'.
     * '%' should be the last character
     */
    if ostr.add((len - 1) as usize) != c {
        return -1;
    }

    /*
     * Construct a string like "xx%/1"
     */
    str_ = malloc((len + 3) as size_t) as *mut c_char;
    if str_.is_null() {
        return -ENOMEM;
    }

    memcpy(str_ as *mut c_void, ostr as *const c_void, len as size_t);
    strcpy(str_.add(len as usize), c"/1".as_ptr());

    ret = percent_slash_split(str_, ptime_buf, start, end);
    if ret == 0 {
        ret = 1;
    }

    free(str_ as *mut c_void);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_time__percent_parse_str(
    ptime_buf: *mut perf_time_interval,
    num: c_int,
    ostr: *const c_char,
    start: u64,
    end: u64,
) -> c_int {
    let mut c: *const c_char;

    /*
     * ostr example:
     * 10%/2,10%/3: select the second 10% slice and the third 10% slice
     * 0%-10%,30%-40%: multiple time range
     * 50%: just one percent
     */

    memset(
        ptime_buf as *mut c_void,
        0,
        mem::size_of::<perf_time_interval>() * num as usize,
    );

    c = strchr(ostr, b'/' as c_int);
    if !c.is_null() {
        return percent_comma_split(ptime_buf, num, ostr, start, end, percent_slash_split);
    }

    c = strchr(ostr, b'-' as c_int);
    if !c.is_null() {
        return percent_comma_split(ptime_buf, num, ostr, start, end, percent_dash_split);
    }

    c = strchr(ostr, b'%' as c_int);
    if !c.is_null() {
        return one_percent_convert(ptime_buf, ostr, start, end, c);
    }

    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_time__range_alloc(
    ostr: *const c_char,
    size: *mut c_int,
) -> *mut perf_time_interval {
    let mut p1: *const c_char;
    let mut p2: *const c_char;
    let mut i: c_int = 1;
    let ptime: *mut perf_time_interval;

    /*
     * At least allocate one time range.
     */
    if ostr.is_null() {
        *size = i;
        ptime = calloc(i as size_t, mem::size_of::<perf_time_interval>()) as *mut perf_time_interval;
        return ptime;
    }

    p1 = ostr;
    while p1 < ostr.add(strlen(ostr)) {
        p2 = strchr(p1, b',' as c_int);
        if p2.is_null() {
            break;
        }

        p1 = p2.add(1);
        i += 1;
    }

    *size = i;
    ptime = calloc(i as size_t, mem::size_of::<perf_time_interval>()) as *mut perf_time_interval;
    ptime
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_time__skip_sample(
    ptime: *mut perf_time_interval,
    timestamp: u64,
) -> bool {
    /* if time is not set don't drop sample */
    if timestamp == 0 {
        return false;
    }

    /* otherwise compare sample time to time window */
    if ((*ptime).start != 0 && timestamp < (*ptime).start)
        || ((*ptime).end != 0 && timestamp > (*ptime).end)
    {
        return true;
    }

    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_time__ranges_skip_sample(
    ptime_buf: *mut perf_time_interval,
    num: c_int,
    timestamp: u64,
) -> bool {
    let mut ptime: *mut perf_time_interval;
    let mut i: c_int;

    if ptime_buf.is_null() || timestamp == 0 || num == 0 {
        return false;
    }

    if num == 1 {
        return perf_time__skip_sample(&mut *ptime_buf.add(0), timestamp);
    }

    /*
     * start/end of multiple time ranges must be valid.
     */
    i = 0;
    while i < num {
        ptime = &mut *ptime_buf.add(i as usize);

        if timestamp >= (*ptime).start && (timestamp <= (*ptime).end || (*ptime).end == 0) {
            return false;
        }
        i += 1;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_time__parse_for_ranges_reltime(
    time_str: *const c_char,
    session: *mut perf_session,
    ranges: *mut *mut perf_time_interval,
    range_size: *mut c_int,
    range_num: *mut c_int,
    reltime: bool,
) -> c_int {
    let has_percent: bool = !strchr(time_str, b'%' as c_int).is_null();
    let ptime_range: *mut perf_time_interval;
    let mut size: c_int = 0;
    let mut num: c_int;
    let ret: c_int = -EINVAL;

    ptime_range = perf_time__range_alloc(time_str, &mut size);
    if ptime_range.is_null() {
        return -ENOMEM;
    }

    if has_percent || reltime {
        if evlist__first_sample_time((*session).evlist) == 0
            && evlist__last_sample_time((*session).evlist) == 0
        {
            pr_err(c"HINT: no first/last sample time found in perf data.\nPlease use latest perf binary to execute 'perf record'\n(if '--buildid-all' is enabled, please set '--timestamp-boundary').\n".as_ptr());
            free(ptime_range as *mut c_void);
            return ret;
        }
    }

    if has_percent {
        num = perf_time__percent_parse_str(
            ptime_range,
            size,
            time_str,
            evlist__first_sample_time((*session).evlist),
            evlist__last_sample_time((*session).evlist),
        );
    } else {
        num = perf_time__parse_strs(ptime_range, time_str, size);
    }

    if num < 0 {
        pr_err(c"Invalid time string\n".as_ptr());
        free(ptime_range as *mut c_void);
        return ret;
    }

    if reltime {
        let mut i: c_int;

        i = 0;
        while i < num {
            (*ptime_range.add(i as usize)).start = (*ptime_range.add(i as usize))
                .start
                .wrapping_add(evlist__first_sample_time((*session).evlist));
            (*ptime_range.add(i as usize)).end = (*ptime_range.add(i as usize))
                .end
                .wrapping_add(evlist__first_sample_time((*session).evlist));
            i += 1;
        }
    }

    *range_size = size;
    *range_num = num;
    *ranges = ptime_range;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_time__parse_for_ranges(
    time_str: *const c_char,
    session: *mut perf_session,
    ranges: *mut *mut perf_time_interval,
    range_size: *mut c_int,
    range_num: *mut c_int,
) -> c_int {
    perf_time__parse_for_ranges_reltime(time_str, session, ranges, range_size, range_num, false)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn timestamp__scnprintf_usec(
    timestamp: u64,
    buf: *mut c_char,
    sz: size_t,
) -> c_int {
    let sec: u64 = timestamp / NSEC_PER_SEC;
    let usec: u64 = (timestamp % NSEC_PER_SEC) / NSEC_PER_USEC;

    scnprintf(buf, sz, c"%llu.%06llu".as_ptr(), sec, usec)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn timestamp__scnprintf_nsec(
    timestamp: u64,
    buf: *mut c_char,
    sz: size_t,
) -> c_int {
    let sec: u64 = timestamp / NSEC_PER_SEC;
    let nsec: u64 = timestamp % NSEC_PER_SEC;

    scnprintf(buf, sz, c"%llu.%09llu".as_ptr(), sec, nsec)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fetch_current_timestamp(buf: *mut c_char, sz: size_t) -> c_int {
    let mut tv: timeval = mem::zeroed();
    let mut tm: tm = mem::zeroed();
    let mut dt = [0 as c_char; 32];

    if gettimeofday(&mut tv, ptr::null_mut()) != 0 || localtime_r(&tv.tv_sec, &mut tm).is_null() {
        return -1;
    }

    if strftime(dt.as_mut_ptr(), dt.len(), c"%Y%m%d%H%M%S".as_ptr(), &tm) == 0 {
        return -1;
    }

    scnprintf(
        buf,
        sz,
        c"%s%02u".as_ptr(),
        dt.as_ptr(),
        (tv.tv_usec as c_uint) / 10000,
    );

    0
}

type c_uint = u32;
