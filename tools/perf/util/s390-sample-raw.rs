// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2019
 * Author(s): Thomas Richter <tmricht@linux.ibm.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License (version 2 only)
 * as published by the Free Software Foundation.
 *
 * Architecture specific trace_event function. Save event's bc000 raw data
 * to file. File name is aux.ctr.## where ## stands for the CPU number the
 * sample was taken from.
 */

/* Translated from C. External types, constants, byte-order helpers, and perf
 * support routines are supplied by the surrounding perf sources.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type u16 = u16;
type u32 = u32;
type u64 = u64;

const PERF_COLOR_BLUE: *const c_char = b"blue\0".as_ptr() as *const c_char;
const S390_CPUMCF_DIAG_DEF: u16 = 0xc;
const CPUMF_CTR_SET_BASIC: c_int = 0;
const CPUMF_CTR_SET_USER: c_int = 1;
const CPUMF_CTR_SET_CRYPTO: c_int = 2;
const CPUMF_CTR_SET_EXT: c_int = 3;
const CPUMF_CTR_SET_MT_DIAG: c_int = 4;
const PERF_EVENT_PAI_NNPA_ALL: c_int = 0x180;
const PERF_EVENT_PAI_CRYPTO_ALL: c_int = 0x181;
const PERF_EVENT_CPUM_CF_DIAG: u64 = 0xbc000;
const PERF_RECORD_SAMPLE: u32 = 9;

#[repr(C)]
pub struct cf_ctrset_entry {
    pub def: u16,
    pub set: u16,
    pub ctr: u16,
    pub res1: u16,
}

#[repr(C)]
pub struct cf_trailer_entry {
    pub flags: u64,
    pub cfvn: u16,
    pub csvn: u16,
    pub cpu_speed: u32,
    pub timestamp: u64,
    pub progusage1: u64,
    pub progusage2: u64,
    pub progusage3: u64,
    pub tod_base: u64,
    pub mach_type: u16,
    pub res1: u16,
    pub res2: u32,
}

impl cf_trailer_entry {
    unsafe fn clock_base(&self) -> bool {
        self.flags & 0x1 != 0
    }

    unsafe fn speed(&self) -> bool {
        self.flags & 0x2 != 0
    }

    unsafe fn mtda(&self) -> bool {
        self.flags & 0x4 != 0
    }

    unsafe fn caca(&self) -> bool {
        self.flags & 0x8 != 0
    }

    unsafe fn lcda(&self) -> bool {
        self.flags & 0x10 != 0
    }
}

#[repr(C)]
pub struct perf_sample {
    pub raw_size: size_t,
    pub raw_data: *mut u8,
    pub evsel: *mut evsel,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub config: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub pmu: *mut perf_pmu,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pmu_event_info {
    pub str_: *const c_char,
    pub name: *const c_char,
}

#[repr(C, packed)]
struct pai_data {
    /* Event number and value */
    event_nr: u16,
    event_val: u64,
}

unsafe extern "C" {
    static mut stdout: *mut c_void;

    fn be16_to_cpu(val: u16) -> u16;
    fn be32_to_cpu(val: u32) -> u32;
    fn be64_to_cpu(val: u64) -> u64;
    fn color_fprintf(stream: *mut c_void, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn hashmap__find(map: *mut hashmap, key: c_long, value: *mut *mut c_char) -> bool;
    fn hashmap__new(
        hash_fn: Option<unsafe extern "C" fn(c_long, *mut c_void) -> size_t>,
        equal_fn: Option<unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool>,
        ctx: *mut c_void,
    ) -> *mut hashmap;
    fn hashmap__set(
        map: *mut hashmap,
        key: c_long,
        value: *mut c_char,
        old_key: *mut c_long,
        old_value: *mut *mut c_char,
    ) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn perf_pmu__for_each_event(
        pmu: *mut perf_pmu,
        skip_duplicate_pmus: bool,
        data: *mut c_void,
        fn_: Option<unsafe extern "C" fn(*mut c_void, *mut pmu_event_info) -> c_int>,
    );
    fn evlist__event2evsel(evlist: *mut evlist, event: *mut perf_event) -> *mut evsel;
    fn evsel__get(evsel: *mut evsel);
    fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;
    fn perf_pmus__find_by_type(type_: u32) -> *mut perf_pmu;
}

unsafe fn ctrset_size(set: *mut cf_ctrset_entry) -> size_t {
    size_of::<cf_ctrset_entry>() + (*set).ctr as size_t * size_of::<u64>()
}

unsafe fn ctrset_valid(set: *mut cf_ctrset_entry) -> bool {
    (*set).def == S390_CPUMCF_DIAG_DEF
}

/* CPU Measurement Counter Facility raw data is a byte stream. It is 8 byte
 * aligned and might have trailing padding bytes.
 * Display the raw data on screen.
 */
unsafe fn s390_cpumcfdg_testctr(sample: *mut perf_sample) -> bool {
    let len: size_t = (*sample).raw_size;
    let mut offset: size_t = 0;
    let buf: *mut u8 = (*sample).raw_data;
    let mut cep: *mut cf_ctrset_entry;
    let mut ce = cf_ctrset_entry {
        def: 0,
        set: 0,
        ctr: 0,
        res1: 0,
    };

    while offset < len {
        cep = buf.add(offset) as *mut cf_ctrset_entry;
        ce.def = be16_to_cpu((*cep).def);
        ce.set = be16_to_cpu((*cep).set);
        ce.ctr = be16_to_cpu((*cep).ctr);
        ce.res1 = be16_to_cpu((*cep).res1);

        if !ctrset_valid(&mut ce) || offset + ctrset_size(&mut ce) > len {
            /* Raw data for counter sets are always multiple of 8
             * bytes. Prepending a 4 bytes size field to the
             * raw data block in the sample causes the perf tool
             * to append 4 padding bytes to make the raw data part
             * of the sample a multiple of eight bytes again.
             *
             * If the last entry (trailer) is 4 bytes off the raw
             * area data end, all is good.
             */
            if len - offset - size_of::<cf_trailer_entry>() == 4 {
                break;
            }
            pr_err(b"Invalid counter set entry at %zd\n\0".as_ptr() as *const c_char, offset);
            return false;
        }
        offset += ctrset_size(&mut ce);
    }
    true
}

/* Dump event bc000 on screen, already tested on correctness. */
unsafe fn s390_cpumcfdg_dumptrail(color: *const c_char, offset: size_t, tep: *mut cf_trailer_entry) {
    let mut te = cf_trailer_entry {
        flags: 0,
        cfvn: 0,
        csvn: 0,
        cpu_speed: 0,
        timestamp: 0,
        progusage1: 0,
        progusage2: 0,
        progusage3: 0,
        tod_base: 0,
        mach_type: 0,
        res1: 0,
        res2: 0,
    };

    te.flags = be64_to_cpu((*tep).flags);
    te.cfvn = be16_to_cpu((*tep).cfvn);
    te.csvn = be16_to_cpu((*tep).csvn);
    te.cpu_speed = be32_to_cpu((*tep).cpu_speed);
    te.timestamp = be64_to_cpu((*tep).timestamp);
    te.progusage1 = be64_to_cpu((*tep).progusage1);
    te.progusage2 = be64_to_cpu((*tep).progusage2);
    te.progusage3 = be64_to_cpu((*tep).progusage3);
    te.tod_base = be64_to_cpu((*tep).tod_base);
    te.mach_type = be16_to_cpu((*tep).mach_type);
    te.res1 = be16_to_cpu((*tep).res1);
    te.res2 = be32_to_cpu((*tep).res2);

    color_fprintf(
        stdout,
        color,
        b"    [%#08zx] Trailer:%c%c%c%c%c Cfvn:%d Csvn:%d Speed:%d TOD:%#lx\n\0".as_ptr()
            as *const c_char,
        offset,
        if te.clock_base() { 'T' as c_int } else { ' ' as c_int },
        if te.speed() { 'S' as c_int } else { ' ' as c_int },
        if te.mtda() { 'M' as c_int } else { ' ' as c_int },
        if te.caca() { 'C' as c_int } else { ' ' as c_int },
        if te.lcda() { 'L' as c_int } else { ' ' as c_int },
        te.cfvn as c_int,
        te.csvn as c_int,
        te.cpu_speed as c_int,
        te.timestamp as c_ulong,
    );
    color_fprintf(
        stdout,
        color,
        b"\t\t1:%lx 2:%lx 3:%lx TOD-Base:%#lx Type:%x\n\n\0".as_ptr() as *const c_char,
        te.progusage1 as c_ulong,
        te.progusage2 as c_ulong,
        te.progusage3 as c_ulong,
        te.tod_base as c_ulong,
        te.mach_type as c_uint,
    );
}

/* Return starting number of a counter set */
fn get_counterset_start(setnr: c_int) -> c_int {
    match setnr {
        CPUMF_CTR_SET_BASIC => 0,   /* Basic counter set */
        CPUMF_CTR_SET_USER => 32,   /* Problem state counter set */
        CPUMF_CTR_SET_CRYPTO => 64, /* Crypto counter set */
        CPUMF_CTR_SET_EXT => 128,   /* Extended counter set */
        CPUMF_CTR_SET_MT_DIAG => 448, /* Diagnostic counter set */
        PERF_EVENT_PAI_NNPA_ALL | PERF_EVENT_PAI_CRYPTO_ALL => setnr,
        _ => -1,
    }
}

#[repr(C)]
struct get_counter_name_data {
    wanted: c_long,
    result: *const c_char,
}

unsafe extern "C" fn get_counter_name_callback(
    vdata: *mut c_void,
    info: *mut pmu_event_info,
) -> c_int {
    let data = vdata as *mut get_counter_name_data;
    let mut event_nr: c_int = 0;
    let event_str: *const c_char;

    if (*info).str_.is_null() {
        return 0;
    }

    event_str = strstr((*info).str_, b"event=\0".as_ptr() as *const c_char);
    if event_str.is_null() {
        return 0;
    }

    let rc = sscanf(
        event_str,
        b"event=%x\0".as_ptr() as *const c_char,
        &mut event_nr as *mut c_int,
    );
    if rc == 1 && event_nr as c_long == (*data).wanted {
        (*data).result = (*info).name;
        return 1; /* Terminate the search. */
    }
    0
}

unsafe extern "C" fn get_counter_name_hash_fn(key: c_long, _ctx: *mut c_void) -> size_t {
    key as size_t
}

unsafe extern "C" fn get_counter_name_hashmap_equal_fn(
    key1: c_long,
    key2: c_long,
    _ctx: *mut c_void,
) -> bool {
    key1 == key2
}

/* Scan the PMU and extract the logical name of a counter from the event. Input
 * is the counter set and counter number with in the set. Construct the event
 * number and use this as key. If they match return the name of this counter.
 * If no match is found a NULL pointer is returned.
 */
unsafe fn get_counter_name(set: c_int, nr: size_t, pmu: *mut perf_pmu) -> *mut c_char {
    static mut CACHE: *mut hashmap = ptr::null_mut();
    static mut CACHE_PMU: *mut perf_pmu = ptr::null_mut();
    let cache_key: c_long = (get_counterset_start(set) as size_t + nr) as c_long;
    let mut data = get_counter_name_data {
        wanted: cache_key,
        result: ptr::null(),
    };
    let mut result: *mut c_char = ptr::null_mut();

    if pmu.is_null() {
        return ptr::null_mut();
    }

    if CACHE_PMU == pmu && hashmap__find(CACHE, cache_key, &mut result) {
        return strdup(result);
    }

    perf_pmu__for_each_event(
        pmu,
        true,
        &mut data as *mut get_counter_name_data as *mut c_void,
        Some(get_counter_name_callback),
    );

    result = strdup(if !data.result.is_null() {
        data.result
    } else {
        b"<unknown>\0".as_ptr() as *const c_char
    });

    if CACHE_PMU.is_null() {
        let tmp = hashmap__new(
            Some(get_counter_name_hash_fn),
            Some(get_counter_name_hashmap_equal_fn),
            ptr::null_mut(),
        );

        if !IS_ERR(tmp as *const c_void) {
            CACHE = tmp;
            CACHE_PMU = pmu;
        }
    }

    if CACHE_PMU == pmu && !result.is_null() {
        let mut old_value: *mut c_char = ptr::null_mut();
        let new_value = strdup(result);

        if !new_value.is_null() {
            hashmap__set(CACHE, cache_key, new_value, ptr::null_mut(), &mut old_value);
            /*
             * Free in case of a race, but resizing would be broken
             * in that case.
             */
            free(old_value as *mut c_void);
        }
    }
    result
}

unsafe fn s390_cpumcfdg_dump(sample: *mut perf_sample) {
    let pmu = (*(*sample).evsel).pmu;
    let mut i: size_t;
    let len: size_t = (*sample).raw_size;
    let mut offset: size_t = 0;
    let buf: *mut u8 = (*sample).raw_data;
    let color: *const c_char = PERF_COLOR_BLUE;
    let mut cep: *mut cf_ctrset_entry;
    let mut ce = cf_ctrset_entry {
        def: 0,
        set: 0,
        ctr: 0,
        res1: 0,
    };
    let mut p: *mut u64;

    while offset < len {
        cep = buf.add(offset) as *mut cf_ctrset_entry;

        ce.def = be16_to_cpu((*cep).def);
        ce.set = be16_to_cpu((*cep).set);
        ce.ctr = be16_to_cpu((*cep).ctr);
        ce.res1 = be16_to_cpu((*cep).res1);

        if !ctrset_valid(&mut ce) {
            /* Print trailer */
            s390_cpumcfdg_dumptrail(color, offset, cep as *mut cf_trailer_entry);
            return;
        }

        color_fprintf(
            stdout,
            color,
            b"    [%#08zx] Counterset:%d Counters:%d\n\0".as_ptr() as *const c_char,
            offset,
            ce.set as c_int,
            ce.ctr as c_int,
        );
        i = 0;
        p = cep.add(1) as *mut u64;
        while i < ce.ctr as size_t {
            let ev_name = get_counter_name(ce.set as c_int, i, pmu);

            color_fprintf(
                stdout,
                color,
                b"\tCounter:%03zd %s Value:%#018lx\n\0".as_ptr() as *const c_char,
                i,
                if !ev_name.is_null() {
                    ev_name as *const c_char
                } else {
                    b"<unknown>\0".as_ptr() as *const c_char
                },
                be64_to_cpu(*p) as c_ulong,
            );
            free(ev_name as *mut c_void);
            i += 1;
            p = p.add(1);
        }
        offset += ctrset_size(&mut ce);
    }
}

/*
 * Test for valid raw data. At least one PAI event should be in the raw
 * data section.
 */
unsafe fn s390_pai_all_test(sample: *mut perf_sample) -> bool {
    let len: size_t = (*sample).raw_size;

    if len < 0xa {
        return false;
    }
    true
}

unsafe fn s390_pai_all_dump(sample: *mut perf_sample) {
    let evsel = (*sample).evsel;
    let len: size_t = (*sample).raw_size;
    let mut offset: size_t = 0;
    let mut p: *mut u8 = (*sample).raw_data;
    let color: *const c_char = PERF_COLOR_BLUE;
    let mut pai_data = pai_data {
        event_nr: 0,
        event_val: 0,
    };
    let mut ev_name: *mut c_char;

    while offset < len {
        memcpy(
            &mut pai_data.event_nr as *mut u16 as *mut c_void,
            p as *const c_void,
            size_of::<u16>(),
        );
        pai_data.event_nr = be16_to_cpu(pai_data.event_nr);
        p = p.add(size_of::<u16>());
        offset += size_of::<u16>();

        memcpy(
            &mut pai_data.event_val as *mut u64 as *mut c_void,
            p as *const c_void,
            size_of::<u64>(),
        );
        pai_data.event_val = be64_to_cpu(pai_data.event_val);
        p = p.add(size_of::<u64>());
        offset += size_of::<u64>();

        ev_name = get_counter_name(
            (*evsel).core.attr.config as c_int,
            pai_data.event_nr as size_t,
            (*evsel).pmu,
        );
        color_fprintf(
            stdout,
            color,
            b"\tCounter:%03d %s Value:%#018lx\n\0".as_ptr() as *const c_char,
            pai_data.event_nr as c_int,
            if !ev_name.is_null() {
                ev_name as *const c_char
            } else {
                b"<unknown>\0".as_ptr() as *const c_char
            },
            pai_data.event_val as c_ulong,
        );
        free(ev_name as *mut c_void);

        if offset + 0xa > len {
            break;
        }
    }
    color_fprintf(stdout, color, b"\n\0".as_ptr() as *const c_char);
}

/* S390 specific trace event function. Check for PERF_RECORD_SAMPLE events
 * and if the event was triggered by a
 * - counter set diagnostic event
 * - processor activity assist (PAI) crypto counter event
 * - processor activity assist (PAI) neural network processor assist (NNPA)
 *   counter event
 * display its raw data.
 * The function is only invoked when the dump flag -D is set.
 *
 * Function evlist__s390_sample_raw() is defined as call back after it has
 * been verified that the perf.data file was created on s390 platform.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn evlist__s390_sample_raw(
    evlist: *mut evlist,
    event: *mut perf_event,
    sample: *mut perf_sample,
) {
    let pai_name: *const c_char;

    if (*event).header.type_ != PERF_RECORD_SAMPLE {
        return;
    }

    if (*sample).evsel.is_null() {
        (*sample).evsel = evlist__event2evsel(evlist, event);
        if (*sample).evsel.is_null() {
            return;
        }
        evsel__get((*sample).evsel);
    }

    /* Check for raw data in sample */
    if (*sample).raw_size == 0 || (*sample).raw_data.is_null() {
        return;
    }

    /* Display raw data on screen */
    if (*(*sample).evsel).core.attr.config == PERF_EVENT_CPUM_CF_DIAG {
        if (*(*sample).evsel).pmu.is_null() {
            (*(*sample).evsel).pmu = perf_pmus__find(b"cpum_cf\0".as_ptr() as *const c_char);
        }
        if !s390_cpumcfdg_testctr(sample) {
            pr_err(b"Invalid counter set data encountered\n\0".as_ptr() as *const c_char);
        } else {
            s390_cpumcfdg_dump(sample);
        }
        return;
    }

    match (*(*sample).evsel).core.attr.config as c_int {
        PERF_EVENT_PAI_NNPA_ALL => {
            pai_name = b"NNPA_ALL\0".as_ptr() as *const c_char;
        }
        PERF_EVENT_PAI_CRYPTO_ALL => {
            pai_name = b"CRYPTO_ALL\0".as_ptr() as *const c_char;
        }
        _ => {
            return;
        }
    }

    if !s390_pai_all_test(sample) {
        pr_err(
            b"Invalid %s raw data encountered\n\0".as_ptr() as *const c_char,
            pai_name,
        );
    } else {
        if (*(*sample).evsel).pmu.is_null() {
            (*(*sample).evsel).pmu =
                perf_pmus__find_by_type((*(*sample).evsel).core.attr.type_);
        }
        s390_pai_all_dump(sample);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
