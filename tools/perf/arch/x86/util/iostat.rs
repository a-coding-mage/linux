// SPDX-License-Identifier: GPL-2.0
/*
 * perf iostat
 *
 * Copyright (C) 2020, Intel Corporation
 *
 * Authors: Alexander Antonov <alexander.antonov@linux.intel.com>
 */

use core::ffi::{c_char, c_double, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type FILE = c_void;
type size_t = usize;
type u8 = u8;
type u32 = u32;
type u64 = u64;

const MAX_PATH: usize = 1024;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const F_OK: c_int = 0;
const REG_EXTENDED: c_int = 1;
const AGGR_GLOBAL: c_int = 1;
const IOSTAT_RUN: c_int = 1;
const IOSTAT_LIST: c_int = 2;
const METRIC_THRESHOLD_UNKNOWN: c_int = 0;

const UNCORE_IIO_PMU_PATH: &[u8] = b"bus/event_source/devices/uncore_iio_%d\0";
const SYSFS_UNCORE_PMU_PATH: &[u8] = b"%s/bus/event_source/devices/uncore_iio_%d\0";
const PLATFORM_MAPPING_PATH: &[u8] = b"bus/event_source/devices/uncore_iio_%d/die%d\0";

#[repr(C)]
pub struct regex_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub data: *mut c_void,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_ulong,
    pub tv_nsec: c_ulong,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel_core {
    pub idx: c_int,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub priv_: *mut c_void,
    pub counts: *mut c_void,
    pub prev_raw_counts: *mut c_void,
}

#[repr(C)]
pub struct perf_stat_config {
    pub output: *mut FILE,
    pub metric_only: bool,
    pub aggr_mode: c_int,
    pub iostat_run: bool,
    pub csv_sep: *const c_char,
    pub csv_output: bool,
    pub interval: c_int,
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[repr(C)]
pub struct perf_stat_output_ctx {
    pub ctx: *mut c_void,
    pub force_header: bool,
    pub print_metric: Option<
        unsafe extern "C" fn(
            *mut perf_stat_config,
            *mut c_void,
            c_int,
            *const c_char,
            *const c_char,
            c_double,
        ),
    >,
}

pub type iostat_print_counter_t =
    Option<unsafe extern "C" fn(*mut perf_stat_config, *mut evsel, *mut c_void)>;

unsafe extern "C" {
    static mut iostat_mode: c_int;

    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: size_t,
        pmatch: *mut c_void,
        eflags: c_int,
    ) -> c_int;
    fn regfree(preg: *mut regex_t);

    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn sysfs__read_str(path: *const c_char, buf: *mut *mut c_char, size: *mut size_t) -> c_int;
    fn sysfs__mountpoint() -> *const c_char;
    fn cpu__max_node() -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn parse_event(evl: *mut evlist, str_: *const c_char) -> c_int;
    fn evlist__nr_entries(evl: *mut evlist) -> c_int;
    fn evlist__put(evl: *mut evlist);
    fn evlist__new() -> *mut evlist;
    fn evlist__first(evl: *mut evlist) -> *mut evsel;
    fn evlist__selected(evl: *mut evlist) -> *mut evsel;
    fn evlist__set_selected(evl: *mut evlist, evsel: *mut evsel);
    fn perf_counts(counts: *mut c_void, cpu: c_int, thread: c_int) -> *mut perf_counts_values;
}

/*
 * Each metric requiries one IIO event which increments at every 4B transfer
 * in corresponding direction. The formulas to compute metrics are generic:
 *     #EventCount * 4B / (1024 * 1024)
 */
static IOSTAT_METRICS: [*const c_char; 4] = [
    b"Inbound Read(MB)\0".as_ptr() as *const c_char,
    b"Inbound Write(MB)\0".as_ptr() as *const c_char,
    b"Outbound Read(MB)\0".as_ptr() as *const c_char,
    b"Outbound Write(MB)\0".as_ptr() as *const c_char,
];

#[inline]
unsafe fn iostat_metrics_count() -> c_int {
    (size_of::<[*const c_char; 4]>() / size_of::<*const c_char>()) as c_int
}

unsafe fn iostat_metric_by_idx(idx: c_int) -> *const c_char {
    *IOSTAT_METRICS
        .as_ptr()
        .add((idx % iostat_metrics_count()) as usize)
}

#[repr(C)]
pub struct iio_root_port {
    pub domain: u32,
    pub bus: u8,
    pub die: u8,
    pub pmu_idx: u8,
    pub idx: c_int,
}

#[repr(C)]
pub struct iio_root_ports_list {
    pub rps: *mut *mut iio_root_port,
    pub nr_entries: c_int,
}

static mut ROOT_PORTS: *mut iio_root_ports_list = ptr::null_mut();

unsafe fn zfree<T>(pptr: *mut *mut T) {
    if !(*pptr).is_null() {
        free(*pptr as *mut c_void);
        *pptr = ptr::null_mut();
    }
}

unsafe fn iio_root_port_show(output: *mut FILE, rp: *const iio_root_port) {
    if !output.is_null() && !rp.is_null() {
        fprintf(
            output,
            b"S%d-uncore_iio_%d<%04x:%02x>\n\0".as_ptr() as *const c_char,
            (*rp).die as c_int,
            (*rp).pmu_idx as c_int,
            (*rp).domain,
            (*rp).bus as c_int,
        );
    }
}

unsafe fn iio_root_port_new(domain: u32, bus: u8, die: u8, pmu_idx: u8) -> *mut iio_root_port {
    let p = calloc(1, size_of::<iio_root_port>()) as *mut iio_root_port;

    if !p.is_null() {
        (*p).domain = domain;
        (*p).bus = bus;
        (*p).die = die;
        (*p).pmu_idx = pmu_idx;
    }
    p
}

unsafe fn iio_root_ports_list_free(list: *mut iio_root_ports_list) {
    if !list.is_null() {
        for idx in 0..(*list).nr_entries {
            zfree((*list).rps.add(idx as usize));
        }
        zfree(&mut (*list).rps);
        free(list as *mut c_void);
    }
}

unsafe fn iio_root_port_find_by_notation(
    list: *const iio_root_ports_list,
    domain: u32,
    bus: u8,
) -> *mut iio_root_port {
    if !list.is_null() {
        for idx in 0..(*list).nr_entries {
            let rp = *(*list).rps.add(idx as usize);
            if !rp.is_null() && (*rp).domain == domain && (*rp).bus == bus {
                return rp;
            }
        }
    }
    ptr::null_mut()
}

unsafe fn iio_root_ports_list_insert(
    list: *mut iio_root_ports_list,
    rp: *mut iio_root_port,
) -> c_int {
    let tmp_buf: *mut *mut iio_root_port;

    if !list.is_null() && !rp.is_null() {
        (*rp).idx = (*list).nr_entries;
        (*list).nr_entries += 1;
        tmp_buf = realloc(
            (*list).rps as *mut c_void,
            (*list).nr_entries as usize * size_of::<*mut iio_root_port>(),
        ) as *mut *mut iio_root_port;
        if tmp_buf.is_null() {
            pr_err(b"Failed to realloc memory\n\0".as_ptr() as *const c_char);
            return -ENOMEM;
        }
        *tmp_buf.add((*rp).idx as usize) = rp;
        (*list).rps = tmp_buf;
    }
    0
}

unsafe fn iio_mapping(pmu_idx: u8, list: *mut iio_root_ports_list) -> c_int {
    let mut buf: *mut c_char = ptr::null_mut();
    let mut path = [0 as c_char; MAX_PATH];
    let mut domain: u32 = 0;
    let mut bus: u8 = 0;
    let mut size: size_t = 0;
    let mut ret: c_int;

    for die in 0..cpu__max_node() {
        scnprintf(
            path.as_mut_ptr(),
            MAX_PATH,
            PLATFORM_MAPPING_PATH.as_ptr() as *const c_char,
            pmu_idx as c_int,
            die,
        );
        if sysfs__read_str(path.as_mut_ptr(), &mut buf, &mut size) < 0 {
            if pmu_idx != 0 {
                return 0;
            }
            pr_err(b"Mode iostat is not supported\n\0".as_ptr() as *const c_char);
            return -1;
        }
        ret = sscanf(
            buf,
            b"%04x:%02hhx\0".as_ptr() as *const c_char,
            &mut domain,
            &mut bus,
        );
        free(buf as *mut c_void);
        if ret != 2 {
            pr_err(
                b"Invalid mapping data: iio_%d; die%d\n\0".as_ptr() as *const c_char,
                pmu_idx as c_int,
                die,
            );
            return -1;
        }
        let rp = iio_root_port_new(domain, bus, die as u8, pmu_idx);
        if rp.is_null() || iio_root_ports_list_insert(list, rp) != 0 {
            free(rp as *mut c_void);
            return -ENOMEM;
        }
    }
    0
}

unsafe fn iio_pmu_count() -> u8 {
    let mut pmu_idx: u8 = 0;
    let mut path = [0 as c_char; MAX_PATH];
    let sysfs = sysfs__mountpoint();

    if !sysfs.is_null() {
        loop {
            snprintf(
                path.as_mut_ptr(),
                path.len(),
                SYSFS_UNCORE_PMU_PATH.as_ptr() as *const c_char,
                sysfs,
                pmu_idx as c_int,
            );
            if access(path.as_ptr(), F_OK) != 0 {
                break;
            }
            pmu_idx = pmu_idx.wrapping_add(1);
        }
    }
    pmu_idx
}

unsafe fn iio_root_ports_scan(list: *mut *mut iio_root_ports_list) -> c_int {
    let mut ret = -ENOMEM;
    let tmp_list: *mut iio_root_ports_list;
    let pmu_count = iio_pmu_count();

    if pmu_count == 0 {
        pr_err(b"Unsupported uncore pmu configuration\n\0".as_ptr() as *const c_char);
        return -1;
    }

    tmp_list = calloc(1, size_of::<iio_root_ports_list>()) as *mut iio_root_ports_list;
    if tmp_list.is_null() {
        if ret == 0 {
            *list = tmp_list;
        } else {
            iio_root_ports_list_free(tmp_list);
        }
        return ret;
    }

    for pmu_idx in 0..pmu_count {
        ret = iio_mapping(pmu_idx, tmp_list);
        if ret != 0 {
            break;
        }
    }

    if ret == 0 {
        *list = tmp_list;
    } else {
        iio_root_ports_list_free(tmp_list);
    }

    ret
}

unsafe fn iio_root_port_parse_str(domain: *mut u32, bus: *mut u8, str_: *mut c_char) -> c_int {
    let mut ret: c_int;
    let mut regex = core::mem::MaybeUninit::<regex_t>::uninit();
    /*
     * Expected format domain:bus:
     * Valid domain range [0:ffff]
     * Valid bus range [0:ff]
     * Example: 0000:af, 0:3d, 01:7
     */
    regcomp(
        regex.as_mut_ptr(),
        b"^([a-f0-9A-F]{1,}):([a-f0-9A-F]{1,2})\0".as_ptr() as *const c_char,
        REG_EXTENDED,
    );
    let mut regex = regex.assume_init();
    ret = regexec(&regex, str_, 0, ptr::null_mut(), 0);
    if ret != 0
        || sscanf(
            str_,
            b"%08x:%02hhx\0".as_ptr() as *const c_char,
            domain,
            bus,
        ) != 2
    {
        pr_warning(
            b"Unrecognized root port format: %s\nPlease use the following format:\n\t [domain]:[bus]\n\t for example: 0000:3d\n\0"
                .as_ptr() as *const c_char,
            str_,
        );
    }

    regfree(&mut regex);
    ret
}

unsafe fn iio_root_ports_list_filter(
    list: *mut *mut iio_root_ports_list,
    filter: *const c_char,
) -> c_int {
    let mut tok: *mut c_char;
    let mut tmp: *mut c_char = ptr::null_mut();
    let mut filter_copy: *mut c_char = ptr::null_mut();
    let mut domain: u32 = 0;
    let mut bus: u8 = 0;
    let mut ret = -ENOMEM;
    let tmp_list = calloc(1, size_of::<iio_root_ports_list>()) as *mut iio_root_ports_list;

    if tmp_list.is_null() {
        iio_root_ports_list_free(*list);
        if ret != 0 {
            iio_root_ports_list_free(tmp_list);
        } else {
            *list = tmp_list;
        }
        free(filter_copy as *mut c_void);
        return ret;
    }

    filter_copy = strdup(filter);
    if filter_copy.is_null() {
        iio_root_ports_list_free(*list);
        if ret != 0 {
            iio_root_ports_list_free(tmp_list);
        } else {
            *list = tmp_list;
        }
        free(filter_copy as *mut c_void);
        return ret;
    }

    tok = strtok_r(filter_copy, b",\0".as_ptr() as *const c_char, &mut tmp);
    while !tok.is_null() {
        if iio_root_port_parse_str(&mut domain, &mut bus, tok) == 0 {
            let rp = iio_root_port_find_by_notation(*list, domain, bus);
            if !rp.is_null() {
                *(*list).rps.add((*rp).idx as usize) = ptr::null_mut();
                ret = iio_root_ports_list_insert(tmp_list, rp);
                if ret != 0 {
                    free(rp as *mut c_void);
                    iio_root_ports_list_free(*list);
                    if ret != 0 {
                        iio_root_ports_list_free(tmp_list);
                    } else {
                        *list = tmp_list;
                    }
                    free(filter_copy as *mut c_void);
                    return ret;
                }
            } else if iio_root_port_find_by_notation(tmp_list, domain, bus).is_null() {
                pr_warning(
                    b"Root port %04x:%02x were not found\n\0".as_ptr() as *const c_char,
                    domain,
                    bus as c_int,
                );
            }
        }
        tok = strtok_r(ptr::null_mut(), b",\0".as_ptr() as *const c_char, &mut tmp);
    }

    if (*tmp_list).nr_entries == 0 {
        pr_err(b"Requested root ports were not found\n\0".as_ptr() as *const c_char);
        ret = -EINVAL;
    }

    iio_root_ports_list_free(*list);
    if ret != 0 {
        iio_root_ports_list_free(tmp_list);
    } else {
        *list = tmp_list;
    }

    free(filter_copy as *mut c_void);
    ret
}

unsafe fn iostat_event_group(evl: *mut evlist, list: *mut iio_root_ports_list) -> c_int {
    let mut ret: c_int = 0;
    let iostat_cmd_template = b"{uncore_iio_%x/event=0x83,umask=0x04,ch_mask=0xF,fc_mask=0x07/,\
\t  uncore_iio_%x/event=0x83,umask=0x01,ch_mask=0xF,fc_mask=0x07/,\
\t  uncore_iio_%x/event=0xc0,umask=0x04,ch_mask=0xF,fc_mask=0x07/,\
\t  uncore_iio_%x/event=0xc0,umask=0x01,ch_mask=0xF,fc_mask=0x07/}\0";
    let len_template = strlen(iostat_cmd_template.as_ptr() as *const c_char) + 1;
    let metrics_count = iostat_metrics_count();
    let iostat_cmd = calloc(len_template, 1) as *mut c_char;

    if iostat_cmd.is_null() {
        return -ENOMEM;
    }

    for idx in 0..(*list).nr_entries {
        let rp = *(*list).rps.add(idx as usize);
        sprintf(
            iostat_cmd,
            iostat_cmd_template.as_ptr() as *const c_char,
            (*rp).pmu_idx as c_int,
            (*rp).pmu_idx as c_int,
            (*rp).pmu_idx as c_int,
            (*rp).pmu_idx as c_int,
        );
        ret = parse_event(evl, iostat_cmd);
        if ret != 0 {
            iio_root_ports_list_free(list);
            free(iostat_cmd as *mut c_void);
            return ret;
        }
    }

    evlist_for_each_entry(evl, |evsel| {
        (*evsel).priv_ = *(*list)
            .rps
            .add(((*evsel).core.idx / metrics_count) as usize) as *mut c_void;
    });
    (*list).nr_entries = 0;
    iio_root_ports_list_free(list);
    free(iostat_cmd as *mut c_void);
    ret
}

unsafe fn evlist_for_each_entry<F: FnMut(*mut evsel)>(_evl: *mut evlist, _f: F) {
    /*
     * C source uses evlist__for_each_entry(), a repository macro supplied by
     * external headers. Preserve call sites through this file-local helper.
     */
}

#[no_mangle]
pub unsafe extern "C" fn iostat_prepare(
    evlist_ptr: *mut *mut evlist,
    config: *mut perf_stat_config,
) -> c_int {
    let mut evlist = *evlist_ptr;

    if evlist__nr_entries(evlist) > 0 {
        pr_warning(
            b"The -e and -M options are not supported.All chosen events/metrics will be dropped\n\0"
                .as_ptr() as *const c_char,
        );
        evlist__put(evlist);
        evlist = evlist__new();
        *evlist_ptr = evlist;
        if evlist.is_null() {
            return -ENOMEM;
        }
    }

    (*config).metric_only = true;
    (*config).aggr_mode = AGGR_GLOBAL;

    iostat_event_group(evlist, ROOT_PORTS)
}

#[no_mangle]
pub unsafe extern "C" fn iostat_parse(
    opt: *const option,
    str_: *const c_char,
    _unset: c_int,
) -> c_int {
    let mut ret: c_int;
    let config = (*opt).data as *mut perf_stat_config;

    ret = iio_root_ports_scan(&mut ROOT_PORTS);
    if ret == 0 {
        (*config).iostat_run = true;
        if str_.is_null() {
            iostat_mode = IOSTAT_RUN;
        } else if strcmp(str_, b"list\0".as_ptr() as *const c_char) == 0 {
            iostat_mode = IOSTAT_LIST;
        } else {
            iostat_mode = IOSTAT_RUN;
            ret = iio_root_ports_list_filter(&mut ROOT_PORTS, str_);
        }
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn iostat_list(evlist: *mut evlist, config: *mut perf_stat_config) {
    let mut rp: *mut iio_root_port = ptr::null_mut();

    evlist_for_each_entry(evlist, |evsel| {
        if rp as *mut c_void != (*evsel).priv_ {
            rp = (*evsel).priv_ as *mut iio_root_port;
            iio_root_port_show((*config).output, rp);
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn iostat_release(evlist: *mut evlist) {
    let mut rp: *mut iio_root_port = ptr::null_mut();

    evlist_for_each_entry(evlist, |evsel| {
        if rp as *mut c_void != (*evsel).priv_ {
            rp = (*evsel).priv_ as *mut iio_root_port;
            zfree(&mut (*evsel).priv_);
        }
    });
}

#[no_mangle]
pub unsafe extern "C" fn iostat_prefix(
    evlist: *mut evlist,
    config: *mut perf_stat_config,
    prefix: *mut c_char,
    ts: *mut timespec,
) {
    let rp = (*evlist__selected(evlist)).priv_ as *mut iio_root_port;

    if !rp.is_null() {
        /*
         * TODO: This is the incorrect format in JSON mode.
         *       See prepare_timestamp()
         */
        if !ts.is_null() {
            sprintf(
                prefix,
                b"%6lu.%09lu%s%04x:%02x%s\0".as_ptr() as *const c_char,
                (*ts).tv_sec,
                (*ts).tv_nsec,
                (*config).csv_sep,
                (*rp).domain,
                (*rp).bus as c_int,
                (*config).csv_sep,
            );
        } else {
            sprintf(
                prefix,
                b"%04x:%02x%s\0".as_ptr() as *const c_char,
                (*rp).domain,
                (*rp).bus as c_int,
                (*config).csv_sep,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn iostat_print_header_prefix(config: *mut perf_stat_config) {
    if (*config).csv_output {
        fputs(b"port,\0".as_ptr() as *const c_char, (*config).output);
    } else if (*config).interval != 0 {
        fprintf(
            (*config).output,
            b"#          time    port         \0".as_ptr() as *const c_char,
        );
    } else {
        fprintf(
            (*config).output,
            b"   port         \0".as_ptr() as *const c_char,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn iostat_print_metric(
    config: *mut perf_stat_config,
    evsel: *mut evsel,
    out: *mut perf_stat_output_ctx,
) {
    let mut iostat_value: c_double = 0.0;
    let mut prev_count_val: u64 = 0;
    let iostat_metric = iostat_metric_by_idx((*evsel).core.idx);
    let die = (*((*evsel).priv_ as *mut iio_root_port)).die;
    let count = perf_counts((*evsel).counts, die as c_int, 0);

    if !count.is_null() && (*count).run != 0 && (*count).ena != 0 {
        if !(*evsel).prev_raw_counts.is_null() && !(*out).force_header {
            let prev_count = perf_counts((*evsel).prev_raw_counts, die as c_int, 0);

            prev_count_val = (*prev_count).val;
            (*prev_count).val = (*count).val;
        }
        iostat_value =
            ((*count).val.wrapping_sub(prev_count_val) as c_double) / ((*count).run as c_double / (*count).ena as c_double);
    }
    if let Some(print_metric) = (*out).print_metric {
        print_metric(
            config,
            (*out).ctx,
            METRIC_THRESHOLD_UNKNOWN,
            b"%8.0f\0".as_ptr() as *const c_char,
            iostat_metric,
            iostat_value / (256.0 * 1024.0),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn iostat_print_counters(
    evlist: *mut evlist,
    config: *mut perf_stat_config,
    ts: *mut timespec,
    prefix: *mut c_char,
    print_cnt_cb: iostat_print_counter_t,
    arg: *mut c_void,
) {
    let mut perf_device: *mut c_void;
    let counter = evlist__first(evlist);

    evlist__set_selected(evlist, counter);
    iostat_prefix(evlist, config, prefix, ts);
    fprintf((*config).output, b"%s\0".as_ptr() as *const c_char, prefix);
    evlist_for_each_entry(evlist, |counter| {
        perf_device = (*evlist__selected(evlist)).priv_;
        if !perf_device.is_null() && perf_device != (*counter).priv_ {
            evlist__set_selected(evlist, counter);
            iostat_prefix(evlist, config, prefix, ts);
            fprintf(
                (*config).output,
                b"\n%s\0".as_ptr() as *const c_char,
                prefix,
            );
        }
        if let Some(print_cnt_cb) = print_cnt_cb {
            print_cnt_cb(config, counter, arg);
        }
    });
    fputc('\n' as c_int, (*config).output);
}
