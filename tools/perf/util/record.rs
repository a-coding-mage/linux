// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/record.c. C include dependencies are represented as
// external declarations and opaque/repr(C) types supplied by the surrounding
// repository.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

type bool_ = bool;
type u64 = u64;
type pid_t = c_int;

const UINT_MAX: c_uint = c_uint::MAX;
const ULLONG_MAX: c_ulonglong = c_ulonglong::MAX;
const EACCES: c_int = 13;
const EINVAL: c_int = 22;

const EVSEL__CONFIG_TERM_FREQ: c_int = 0;
const EVSEL__CONFIG_TERM_PERIOD: c_int = 1;
const EVSEL__CONFIG_TERM_OVERWRITE: c_int = 2;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_event_attr {
    pub sample_type: u64,
    pub sample_freq: u64,
    pub sample_period: u64,
    pub freq: c_uint,
    pub write_backward: c_uint,
    pub comm_exec: c_uint,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub config_terms: list_head,
    pub sample_read: bool_,
    pub tracking: bool_,
}

#[repr(C)]
pub struct evsel_config_term {
    pub list: list_head,
    pub type_: c_int,
}

#[repr(C)]
pub struct evlist_core {
    pub user_requested_cpus: *mut perf_cpu_map,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct record_opts {
    pub sample_id: bool_,
    pub full_auxtrace: bool_,
    pub sample_identifier: bool_,
    pub no_inherit: bool_,
    pub user_freq: c_uint,
    pub user_interval: c_ulonglong,
    pub default_interval: c_ulonglong,
    pub freq: c_uint,
    pub strict_freq: bool_,
}

#[repr(C)]
pub struct callchain_param {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub value: *mut c_void,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn evsel__leader(evsel: *mut evsel) -> *mut evsel;
    fn evsel__is_aux_event(evsel: *mut evsel) -> bool_;
    fn arch_topdown_sample_read(evsel: *mut evsel) -> bool_;
    fn is_mem_loads_aux_event(evsel: *mut evsel) -> bool_;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__last(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evsel__config(evsel: *mut evsel, opts: *mut record_opts, callchain: *mut callchain_param);
    fn evsel__set_sample_id(evsel: *mut evsel, use_sample_identifier: bool_);
    fn evlist__set_id_pos(evlist: *mut evlist);
    fn perf_cpu_map__cpu(cpus: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__is_any_cpu_or_is_empty(cpus: *mut perf_cpu_map) -> bool_;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_can_comm_exec() -> bool_;
    fn perf_can_sample_identifier() -> bool_;
    fn sysctl__read_int(path: *const c_char, value: *mut c_int) -> c_int;
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn parse_event(evlist: *mut evlist, str_: *const c_char) -> c_int;
    fn sys_perf_event_open(
        attr: *mut perf_event_attr,
        pid: pid_t,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulonglong,
    ) -> c_int;
    fn perf_event_open_cloexec_flag() -> c_ulonglong;
    fn close(fd: c_int) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
}

unsafe fn evlist_for_each_entry<F>(evlist: *mut evlist, mut f: F)
where
    F: FnMut(*mut evsel),
{
    let mut pos = evlist__first(evlist);
    while !pos.is_null() {
        let next = evlist__next(evlist, pos);
        f(pos);
        pos = next;
    }
}

unsafe fn list_for_each_evsel_config_term<F>(head: *mut list_head, mut f: F)
where
    F: FnMut(*mut evsel_config_term),
{
    let mut pos = (*head).next;
    while !pos.is_null() && pos != head {
        let term = pos as *mut evsel_config_term;
        f(term);
        pos = (*pos).next;
    }
}

/*
 * evsel__config_leader_sampling() uses special rules for leader sampling.
 * However, if the leader is an AUX area event, then assume the event to sample
 * is the next event.
 */
unsafe fn evsel__read_sampler(mut evsel: *mut evsel, evlist: *mut evlist) -> *mut evsel {
    let leader = evsel__leader(evsel);

    if evsel__is_aux_event(leader)
        || arch_topdown_sample_read(leader)
        || is_mem_loads_aux_event(leader)
    {
        let mut found: *mut evsel = core::ptr::null_mut();
        evlist_for_each_entry(evlist, |entry| {
            if found.is_null() && evsel__leader(entry) == leader && entry != evsel__leader(entry) {
                found = entry;
            }
        });
        if !found.is_null() {
            return found;
        }
    }

    leader
}

unsafe fn evsel__config_term_mask(evsel: *mut evsel) -> u64 {
    let config_terms = &mut (*evsel).config_terms as *mut list_head;
    let mut term_types: u64 = 0;

    list_for_each_evsel_config_term(config_terms, |term| {
        term_types |= 1u64 << (*term).type_;
    });
    term_types
}

unsafe fn evsel__config_leader_sampling(evsel: *mut evsel, evlist: *mut evlist) {
    let attr = &mut (*evsel).core.attr as *mut perf_event_attr;
    let leader = evsel__leader(evsel);
    let read_sampler: *mut evsel;
    let term_types: u64;
    let freq_mask: u64;

    if !(*leader).sample_read {
        return;
    }

    read_sampler = evsel__read_sampler(evsel, evlist);

    if evsel == read_sampler {
        return;
    }

    term_types = evsel__config_term_mask(evsel);
    /*
     * Disable sampling for all group members except those with explicit
     * config terms or the leader. In the case of an AUX area event, the 2nd
     * event in the group is the one that 'leads' the sampling.
     */
    freq_mask = (1u64 << EVSEL__CONFIG_TERM_FREQ) | (1u64 << EVSEL__CONFIG_TERM_PERIOD);
    if (term_types & freq_mask) == 0 {
        (*attr).freq = 0;
        (*attr).sample_freq = 0;
        (*attr).sample_period = 0;
    }
    if (term_types & (1u64 << EVSEL__CONFIG_TERM_OVERWRITE)) == 0 {
        (*attr).write_backward = 0;
    }

    /*
     * We don't get a sample for slave events, we make them when delivering
     * the group leader sample. Set the slave event to follow the master
     * sample_type to ease up reporting.
     * An AUX area event also has sample_type requirements, so also include
     * the sample type bits from the leader's sample_type to cover that
     * case.
     */
    (*attr).sample_type =
        (*read_sampler).core.attr.sample_type | (*leader).core.attr.sample_type;
}

#[no_mangle]
pub unsafe extern "C" fn evlist__config(
    evlist: *mut evlist,
    opts: *mut record_opts,
    callchain: *mut callchain_param,
) {
    let mut use_sample_identifier = false;
    let use_comm_exec: bool_;
    let mut sample_id = (*opts).sample_id;

    if perf_cpu_map__cpu((*evlist__core(evlist)).user_requested_cpus, 0).cpu < 0 {
        (*opts).no_inherit = true;
    }

    use_comm_exec = perf_can_comm_exec();

    evlist_for_each_entry(evlist, |evsel| {
        evsel__config(evsel, opts, callchain);
        if (*evsel).tracking && use_comm_exec {
            (*evsel).core.attr.comm_exec = 1;
        }
    });

    /* Configure leader sampling here now that the sample type is known */
    evlist_for_each_entry(evlist, |evsel| {
        evsel__config_leader_sampling(evsel, evlist);
    });

    if (*opts).full_auxtrace || (*opts).sample_identifier {
        /*
         * Need to be able to synthesize and parse selected events with
         * arbitrary sample types, which requires always being able to
         * match the id.
         */
        use_sample_identifier = perf_can_sample_identifier();
        sample_id = true;
    } else if evlist__nr_entries(evlist) > 1 {
        let first = evlist__first(evlist);

        evlist_for_each_entry(evlist, |evsel| {
            if (*evsel).core.attr.sample_type == (*first).core.attr.sample_type {
                return;
            }
            use_sample_identifier = perf_can_sample_identifier();
        });
        sample_id = true;
    }

    if sample_id {
        evlist_for_each_entry(evlist, |evsel| {
            evsel__set_sample_id(evsel, use_sample_identifier);
        });
    }

    evlist__set_id_pos(evlist);
}

unsafe fn get_max_rate(rate: *mut c_uint) -> c_int {
    sysctl__read_int(
        b"kernel/perf_event_max_sample_rate\0".as_ptr() as *const c_char,
        rate as *mut c_int,
    )
}

unsafe fn record_opts__config_freq(opts: *mut record_opts) -> c_int {
    let user_freq = (*opts).user_freq != UINT_MAX;
    let user_interval = (*opts).user_interval != ULLONG_MAX;
    let mut max_rate: c_uint = 0;

    if user_interval && user_freq {
        pr_err(b"cannot set frequency and period at the same time\n\0".as_ptr() as *const c_char);
        return -1;
    }

    if user_interval {
        (*opts).default_interval = (*opts).user_interval;
    }
    if user_freq {
        (*opts).freq = (*opts).user_freq;
    }

    /*
     * User specified count overrides default frequency.
     */
    if (*opts).default_interval != 0 {
        (*opts).freq = 0;
    } else if (*opts).freq != 0 {
        (*opts).default_interval = (*opts).freq as c_ulonglong;
    } else {
        pr_err(b"frequency and count are zero, aborting\n\0".as_ptr() as *const c_char);
        return -1;
    }

    if get_max_rate(&mut max_rate) != 0 {
        return 0;
    }

    /*
     * User specified frequency is over current maximum.
     */
    if user_freq && max_rate < (*opts).freq {
        if (*opts).strict_freq {
            pr_err(
                b"error: Maximum frequency rate (%'u Hz) exceeded.\n       Please use -F freq option with a lower value or consider\n       tweaking /proc/sys/kernel/perf_event_max_sample_rate.\n\0"
                    .as_ptr() as *const c_char,
                max_rate,
            );
            return -1;
        } else {
            pr_warning(
                b"warning: Maximum frequency rate (%'u Hz) exceeded, throttling from %'u Hz to %'u Hz.\n         The limit can be raised via /proc/sys/kernel/perf_event_max_sample_rate.\n         The kernel will lower it when perf's interrupts take too long.\n         Use --strict-freq to disable this throttling, refusing to record.\n\0"
                    .as_ptr() as *const c_char,
                max_rate,
                (*opts).freq,
                max_rate,
            );

            (*opts).freq = max_rate;
        }
    }

    /*
     * Default frequency is over current maximum.
     */
    if max_rate < (*opts).freq {
        pr_warning(
            b"Lowering default frequency rate from %u to %u.\nPlease consider tweaking /proc/sys/kernel/perf_event_max_sample_rate.\n\0"
                .as_ptr() as *const c_char,
            (*opts).freq,
            max_rate,
        );
        (*opts).freq = max_rate;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn record_opts__config(opts: *mut record_opts) -> c_int {
    record_opts__config_freq(opts)
}

#[no_mangle]
pub unsafe extern "C" fn evlist__can_select_event(
    evlist: *mut evlist,
    str_: *const c_char,
) -> bool_ {
    let temp_evlist: *mut evlist;
    let evsel: *mut evsel;
    let mut err: c_int;
    let fd: c_int;
    let mut cpu = perf_cpu { cpu: 0 };
    let mut ret = false;
    let mut pid: pid_t = -1;

    temp_evlist = evlist__new();
    if temp_evlist.is_null() {
        return false;
    }

    err = parse_event(temp_evlist, str_);
    if err != 0 {
        evlist__put(temp_evlist);
        return ret;
    }

    evsel = evlist__last(temp_evlist);

    if evlist.is_null()
        || perf_cpu_map__is_any_cpu_or_is_empty((*evlist__core(evlist)).user_requested_cpus)
    {
        let cpus = perf_cpu_map__new_online_cpus();

        if !cpus.is_null() {
            cpu = perf_cpu_map__cpu(cpus, 0);
        }

        perf_cpu_map__put(cpus);
    } else {
        cpu = perf_cpu_map__cpu((*evlist__core(evlist)).user_requested_cpus, 0);
    }

    loop {
        fd = sys_perf_event_open(
            &mut (*evsel).core.attr,
            pid,
            cpu.cpu,
            -1,
            perf_event_open_cloexec_flag(),
        );
        if fd < 0 {
            if pid == -1 && errno == EACCES {
                pid = 0;
                continue;
            }
            evlist__put(temp_evlist);
            return ret;
        }
        break;
    }
    close(fd);
    ret = true;

    evlist__put(temp_evlist);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn record__parse_freq(
    opt: *const option,
    str_: *const c_char,
    _unset: c_int,
) -> c_int {
    let mut freq: c_uint = 0;
    let opts = (*opt).value as *mut record_opts;

    if str_.is_null() {
        return -EINVAL;
    }

    if strcasecmp(str_, b"max\0".as_ptr() as *const c_char) == 0 {
        if get_max_rate(&mut freq) != 0 {
            pr_err(
                b"couldn't read /proc/sys/kernel/perf_event_max_sample_rate\n\0".as_ptr()
                    as *const c_char,
            );
            return -1;
        }
        pr_info(
            b"info: Using a maximum frequency rate of %'d Hz\n\0".as_ptr() as *const c_char,
            freq,
        );
    } else {
        freq = atoi(str_) as c_uint;
    }

    (*opts).user_freq = freq;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
