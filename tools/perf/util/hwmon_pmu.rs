// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// Rust translation of perf/util/hwmon_pmu.c. C include dependencies are
// represented as external types, constants, functions, and macro-equivalent
// comments where their definitions are supplied by other repository files.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_longlong, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type __u32 = u32;

const HWMON_TYPE_CPU: usize = 1;
const HWMON_TYPE_CURR: usize = 2;
const HWMON_TYPE_ENERGY: usize = 3;
const HWMON_TYPE_FAN: usize = 4;
const HWMON_TYPE_HUMIDITY: usize = 5;
const HWMON_TYPE_IN: usize = 6;
const HWMON_TYPE_INTRUSION: usize = 7;
const HWMON_TYPE_POWER: usize = 8;
const HWMON_TYPE_PWM: usize = 9;
const HWMON_TYPE_TEMP: usize = 10;
const HWMON_TYPE_MAX: usize = 11;

const HWMON_ITEM_ACCURACY: usize = 1;
const HWMON_ITEM_ALARM: usize = 2;
const HWMON_ITEM_AUTO_CHANNELS_TEMP: usize = 3;
const HWMON_ITEM_AVERAGE: usize = 4;
const HWMON_ITEM_AVERAGE_HIGHEST: usize = 5;
const HWMON_ITEM_AVERAGE_INTERVAL: usize = 6;
const HWMON_ITEM_AVERAGE_INTERVAL_MAX: usize = 7;
const HWMON_ITEM_AVERAGE_INTERVAL_MIN: usize = 8;
const HWMON_ITEM_AVERAGE_LOWEST: usize = 9;
const HWMON_ITEM_AVERAGE_MAX: usize = 10;
const HWMON_ITEM_AVERAGE_MIN: usize = 11;
const HWMON_ITEM_BEEP: usize = 12;
const HWMON_ITEM_CAP: usize = 13;
const HWMON_ITEM_CAP_HYST: usize = 14;
const HWMON_ITEM_CAP_MAX: usize = 15;
const HWMON_ITEM_CAP_MIN: usize = 16;
const HWMON_ITEM_CRIT: usize = 17;
const HWMON_ITEM_CRIT_HYST: usize = 18;
const HWMON_ITEM_DIV: usize = 19;
const HWMON_ITEM_EMERGENCY: usize = 20;
const HWMON_ITEM_EMERGENCY_HIST: usize = 21;
const HWMON_ITEM_ENABLE: usize = 22;
const HWMON_ITEM_FAULT: usize = 23;
const HWMON_ITEM_FREQ: usize = 24;
const HWMON_ITEM_HIGHEST: usize = 25;
const HWMON_ITEM_INPUT: usize = 26;
const HWMON_ITEM_LABEL: usize = 27;
const HWMON_ITEM_LCRIT: usize = 28;
const HWMON_ITEM_LCRIT_HYST: usize = 29;
const HWMON_ITEM_LOWEST: usize = 30;
const HWMON_ITEM_MAX: usize = 31;
const HWMON_ITEM_MAX_HYST: usize = 32;
const HWMON_ITEM_MIN: usize = 33;
const HWMON_ITEM_MIN_HYST: usize = 34;
const HWMON_ITEM_MOD: usize = 35;
const HWMON_ITEM_OFFSET: usize = 36;
const HWMON_ITEM_PULSES: usize = 37;
const HWMON_ITEM_RATED_MAX: usize = 38;
const HWMON_ITEM_RATED_MIN: usize = 39;
const HWMON_ITEM_RESET_HISTORY: usize = 40;
const HWMON_ITEM_TARGET: usize = 41;
const HWMON_ITEM_TYPE: usize = 42;
const HWMON_ITEM_VID: usize = 43;
const HWMON_ITEM__MAX: usize = 44;

const BITS_PER_LONG: usize = size_of::<c_ulong>() * 8;
const HWMON_ITEM_BITMAP_LONGS: usize = (HWMON_ITEM__MAX + BITS_PER_LONG - 1) / BITS_PER_LONG;

const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;
const O_DIRECTORY: c_int = 0o200000;
const DT_REG: u8 = 8;
const DT_LNK: u8 = 10;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const PATH_MAX: usize = 4096;
const PARSE_EVENTS__TERM_TYPE_USER: c_int = 0;

const PERF_PMU_TYPE_HWMON_START: __u32 = 0; // external constant in C headers
const PERF_PMU_TYPE_HWMON_END: __u32 = 0; // external constant in C headers

/** Strings that correspond to enum hwmon_type. */
static hwmon_type_strs: [*const c_char; HWMON_TYPE_MAX] = [
    ptr::null(),
    b"cpu\0".as_ptr() as *const c_char,
    b"curr\0".as_ptr() as *const c_char,
    b"energy\0".as_ptr() as *const c_char,
    b"fan\0".as_ptr() as *const c_char,
    b"humidity\0".as_ptr() as *const c_char,
    b"in\0".as_ptr() as *const c_char,
    b"intrusion\0".as_ptr() as *const c_char,
    b"power\0".as_ptr() as *const c_char,
    b"pwm\0".as_ptr() as *const c_char,
    b"temp\0".as_ptr() as *const c_char,
];
const LONGEST_HWMON_TYPE_STR: &[u8] = b"intrusion\0";

/** Strings that correspond to enum hwmon_item. */
static hwmon_item_strs: [*const c_char; HWMON_ITEM__MAX] = [
    ptr::null(),
    b"accuracy\0".as_ptr() as *const c_char,
    b"alarm\0".as_ptr() as *const c_char,
    b"auto_channels_temp\0".as_ptr() as *const c_char,
    b"average\0".as_ptr() as *const c_char,
    b"average_highest\0".as_ptr() as *const c_char,
    b"average_interval\0".as_ptr() as *const c_char,
    b"average_interval_max\0".as_ptr() as *const c_char,
    b"average_interval_min\0".as_ptr() as *const c_char,
    b"average_lowest\0".as_ptr() as *const c_char,
    b"average_max\0".as_ptr() as *const c_char,
    b"average_min\0".as_ptr() as *const c_char,
    b"beep\0".as_ptr() as *const c_char,
    b"cap\0".as_ptr() as *const c_char,
    b"cap_hyst\0".as_ptr() as *const c_char,
    b"cap_max\0".as_ptr() as *const c_char,
    b"cap_min\0".as_ptr() as *const c_char,
    b"crit\0".as_ptr() as *const c_char,
    b"crit_hyst\0".as_ptr() as *const c_char,
    b"div\0".as_ptr() as *const c_char,
    b"emergency\0".as_ptr() as *const c_char,
    b"emergency_hist\0".as_ptr() as *const c_char,
    b"enable\0".as_ptr() as *const c_char,
    b"fault\0".as_ptr() as *const c_char,
    b"freq\0".as_ptr() as *const c_char,
    b"highest\0".as_ptr() as *const c_char,
    b"input\0".as_ptr() as *const c_char,
    b"label\0".as_ptr() as *const c_char,
    b"lcrit\0".as_ptr() as *const c_char,
    b"lcrit_hyst\0".as_ptr() as *const c_char,
    b"lowest\0".as_ptr() as *const c_char,
    b"max\0".as_ptr() as *const c_char,
    b"max_hyst\0".as_ptr() as *const c_char,
    b"min\0".as_ptr() as *const c_char,
    b"min_hyst\0".as_ptr() as *const c_char,
    b"mod\0".as_ptr() as *const c_char,
    b"offset\0".as_ptr() as *const c_char,
    b"pulses\0".as_ptr() as *const c_char,
    b"rated_max\0".as_ptr() as *const c_char,
    b"rated_min\0".as_ptr() as *const c_char,
    b"reset_history\0".as_ptr() as *const c_char,
    b"target\0".as_ptr() as *const c_char,
    b"type\0".as_ptr() as *const c_char,
    b"vid\0".as_ptr() as *const c_char,
];
const LONGEST_HWMON_ITEM_STR: &[u8] = b"average_interval_max\0";

static hwmon_units: [*const c_char; HWMON_TYPE_MAX] = [
    ptr::null(),
    b"V\0".as_ptr() as *const c_char,   /* cpu */
    b"A\0".as_ptr() as *const c_char,   /* curr */
    b"J\0".as_ptr() as *const c_char,   /* energy */
    b"rpm\0".as_ptr() as *const c_char, /* fan */
    b"%\0".as_ptr() as *const c_char,   /* humidity */
    b"V\0".as_ptr() as *const c_char,   /* in */
    b"\0".as_ptr() as *const c_char,    /* intrusion */
    b"W\0".as_ptr() as *const c_char,   /* power */
    b"Hz\0".as_ptr() as *const c_char,  /* pwm */
    b"'C\0".as_ptr() as *const c_char,  /* temp */
];

#[repr(C)]
pub struct hwmon_pmu {
    pmu: perf_pmu,
    events: hashmap,
    hwmon_dir: *mut c_char,
}

/**
 * struct hwmon_pmu_event_value: Value in hwmon_pmu->events.
 *
 * Hwmon files are of the form <type><number>_<item> and may have a suffix
 * _alarm.
 */
#[repr(C)]
pub struct hwmon_pmu_event_value {
    /** @items: which item files are present. */
    items: [c_ulong; HWMON_ITEM_BITMAP_LONGS],
    /** @alarm_items: which item files are present. */
    alarm_items: [c_ulong; HWMON_ITEM_BITMAP_LONGS],
    /** @label: contents of <type><number>_label if present. */
    label: *mut c_char,
    /** @name: name computed from label of the form <type>_<label>. */
    name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hwmon_pmu_event_key {
    type_and_num: c_long,
    // C defines .type and .num in hwmon_pmu.h. Their exact bitfield layout is
    // an external dependency; helpers below preserve the file-local operations.
}

unsafe fn key_get_type(key: hwmon_pmu_event_key) -> usize {
    ((key.type_and_num as u64) & 0xffff) as usize
}

unsafe fn key_get_num(key: hwmon_pmu_event_key) -> c_int {
    ((key.type_and_num as u64 >> 16) as i16) as c_int
}

unsafe fn key_set(type_: usize, num: c_int) -> hwmon_pmu_event_key {
    hwmon_pmu_event_key {
        type_and_num: ((num as i64 as u64) << 16 | (type_ as u64 & 0xffff)) as c_long,
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__is_hwmon(pmu: *const perf_pmu) -> bool_ {
    !pmu.is_null()
        && (*pmu).type_ >= PERF_PMU_TYPE_HWMON_START
        && (*pmu).type_ <= PERF_PMU_TYPE_HWMON_END
}

#[no_mangle]
pub unsafe extern "C" fn evsel__is_hwmon(evsel: *const evsel) -> bool_ {
    perf_pmu__is_hwmon((*evsel).pmu)
}

unsafe extern "C" fn hwmon_pmu__event_hashmap_hash(key: c_long, _ctx: *mut c_void) -> size_t {
    hwmon_pmu_event_key { type_and_num: key }.type_and_num as size_t
}

unsafe extern "C" fn hwmon_pmu__event_hashmap_equal(
    key1: c_long,
    key2: c_long,
    _ctx: *mut c_void,
) -> bool_ {
    hwmon_pmu_event_key { type_and_num: key1 }.type_and_num
        == hwmon_pmu_event_key { type_and_num: key2 }.type_and_num
}

unsafe extern "C" fn hwmon_strcmp(a: *const c_void, b: *const c_void) -> c_int {
    let sa = a as *const c_char;
    let sb = b as *const *const c_char;
    strcmp(sa, *sb)
}

#[no_mangle]
pub unsafe extern "C" fn parse_hwmon_filename(
    filename: *const c_char,
    type_: *mut c_int,
    number: *mut c_int,
    item: *mut c_int,
    alarm: *mut bool_,
) -> bool_ {
    let mut fn_type = [0 as c_char; 24];
    let mut fn_item: *const c_char = ptr::null();

    assert!(LONGEST_HWMON_TYPE_STR.len() - 1 < fn_type.len());
    strlcpy(fn_type.as_mut_ptr(), filename, fn_type.len());
    let mut i = 0usize;
    while fn_type[i] != 0 {
        if fn_type[i] >= b'0' as c_char && fn_type[i] <= b'9' as c_char {
            fn_type[i] = 0;
            *number = strtoul(filename.add(i), &mut fn_item as *mut _ as *mut *mut c_char, 10) as c_int;
            if *fn_item == b'_' as c_char {
                fn_item = fn_item.add(1);
            }
            break;
        }
        if fn_type[i] == b'_' as c_char {
            fn_type[i] = 0;
            *number = -1;
            fn_item = filename.add(i + 1);
            break;
        }
        i += 1;
    }
    if fn_item.is_null() || fn_type[0] == 0 || (!item.is_null() && *fn_item == 0) {
        pr_debug3(b"hwmon_pmu: not a hwmon file '%s'\n\0".as_ptr() as *const c_char, filename);
        return false;
    }

    let elem = bsearch(
        &fn_type as *const _ as *const c_void,
        hwmon_type_strs.as_ptr().add(1) as *const c_void,
        HWMON_TYPE_MAX - 1,
        size_of::<*const c_char>(),
        Some(hwmon_strcmp),
    ) as *const *const c_char;
    if elem.is_null() {
        pr_debug3(
            b"hwmon_pmu: not a hwmon type '%s' in file name '%s'\n\0".as_ptr() as *const c_char,
            fn_type.as_ptr(),
            filename,
        );
        return false;
    }

    *type_ = elem.offset_from(hwmon_type_strs.as_ptr()) as c_int;
    if item.is_null() {
        return true;
    }

    *alarm = false;
    let mut fn_item_len = strlen(fn_item);
    if fn_item_len > 6 && strcmp(fn_item.add(fn_item_len - 6), b"_alarm\0".as_ptr() as *const c_char) == 0 {
        assert!(LONGEST_HWMON_ITEM_STR.len() - 1 < fn_type.len());
        /* fn_item_len - 5 strips "_alarm"; clamp to buffer size */
        let n = core::cmp::min(fn_item_len - 5, fn_type.len());
        strlcpy(fn_type.as_mut_ptr(), fn_item, n);
        fn_item = fn_type.as_ptr();
        *alarm = true;
        fn_item_len = strlen(fn_item);
        let _ = fn_item_len;
    }
    let elem = bsearch(
        fn_item as *const c_void,
        hwmon_item_strs.as_ptr().add(1) as *const c_void,
        HWMON_ITEM__MAX - 1,
        size_of::<*const c_char>(),
        Some(hwmon_strcmp),
    ) as *const *const c_char;
    if elem.is_null() {
        pr_debug3(
            b"hwmon_pmu: not a hwmon item '%s' in file name '%s'\n\0".as_ptr() as *const c_char,
            fn_item,
            filename,
        );
        return false;
    }
    *item = elem.offset_from(hwmon_item_strs.as_ptr()) as c_int;
    true
}

unsafe fn fix_name(mut p: *mut c_char) {
    let s = strchr(p, b'\n' as c_int);
    if !s.is_null() {
        *s = 0;
    }

    while *p != 0 {
        if !strchr(b" :,/\n\t\0".as_ptr() as *const c_char, *p as c_int).is_null() {
            *p = b'_' as c_char;
        } else {
            *p = tolower(*p as c_int) as c_char;
        }
        p = p.add(1);
    }
}

unsafe fn hwmon_pmu__read_events(pmu: *mut hwmon_pmu) -> c_int {
    let mut err = 0;
    let mut dir: io_dir = core::mem::zeroed();

    if (*pmu).pmu.sysfs_aliases_loaded {
        return 0;
    }

    /* Use openat so that the directory contents are refreshed. */
    io_dir__init(
        &mut dir,
        open((*pmu).hwmon_dir, O_CLOEXEC | O_DIRECTORY | O_RDONLY),
    );

    if dir.dirfd < 0 {
        return -ENOENT;
    }

    loop {
        let ent = io_dir__readdir(&mut dir);
        if ent.is_null() {
            break;
        }
        let mut type_: c_int = 0;
        let mut number: c_int = 0;
        let mut item: c_int = 0;
        let mut alarm = false;
        let mut value: *mut hwmon_pmu_event_value = ptr::null_mut();

        if (*ent).d_type != DT_REG {
            continue;
        }

        if !parse_hwmon_filename((*ent).d_name.as_ptr(), &mut type_, &mut number, &mut item, &mut alarm) {
            pr_debug3(b"Not a hwmon file '%s'\n\0".as_ptr() as *const c_char, (*ent).d_name.as_ptr());
            continue;
        }
        let key = key_set(type_ as usize, number);
        if !hashmap__find(&mut (*pmu).events, key.type_and_num, &mut value as *mut _ as *mut *mut c_void) {
            value = zalloc(size_of::<hwmon_pmu_event_value>()) as *mut hwmon_pmu_event_value;
            if value.is_null() {
                err = -ENOMEM;
                break;
            }
            err = hashmap__add(&mut (*pmu).events, key.type_and_num, value as *mut c_void);
            if err != 0 {
                free(value as *mut c_void);
                err = -ENOMEM;
                break;
            }
        }
        set_bit(item as usize, if alarm { (*value).alarm_items.as_mut_ptr() } else { (*value).items.as_mut_ptr() });
        if item as usize == HWMON_ITEM_LABEL {
            let mut buf = [0 as c_char; 128];
            let fd = openat(dir.dirfd, (*ent).d_name.as_ptr(), O_RDONLY);
            let mut read_len: ssize_t;

            if fd < 0 {
                continue;
            }

            read_len = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len() - 1);

            while read_len > 0 && buf[(read_len - 1) as usize] == b'\n' as c_char {
                read_len -= 1;
            }

            if read_len <= 0 {
                close(fd);
                continue;
            }
            buf[read_len as usize] = 0;

            if buf[0] == 0 {
                pr_debug(
                    b"hwmon_pmu: empty label file %s %s\n\0".as_ptr() as *const c_char,
                    (*pmu).pmu.name,
                    (*ent).d_name.as_ptr(),
                );
                close(fd);
                continue;
            }
            (*value).label = strdup(buf.as_ptr());
            if (*value).label.is_null() {
                pr_debug(b"hwmon_pmu: memory allocation failure\n\0".as_ptr() as *const c_char);
                close(fd);
                continue;
            }
            snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                b"%s_%s\0".as_ptr() as *const c_char,
                hwmon_type_strs[type_ as usize],
                (*value).label,
            );
            fix_name(buf.as_mut_ptr());
            (*value).name = strdup(buf.as_ptr());
            if (*value).name.is_null() {
                pr_debug(b"hwmon_pmu: memory allocation failure\n\0".as_ptr() as *const c_char);
            }
            close(fd);
        }
    }

    if hashmap__size(&mut (*pmu).events) == 0 {
        pr_debug2(b"hwmon_pmu: %s has no events\n\0".as_ptr() as *const c_char, (*pmu).pmu.name);
    }

    hashmap_for_each_entry_safe(&mut (*pmu).events, Some(remove_event_without_input), pmu as *mut c_void);
    (*pmu).pmu.sysfs_aliases_loaded = true;

    close(dir.dirfd);
    err
}

unsafe extern "C" fn remove_event_without_input(
    map: *mut hashmap,
    cur: *mut hashmap_entry,
    ctx: *mut c_void,
) {
    let pmu = ctx as *mut hwmon_pmu;
    let key = hwmon_pmu_event_key { type_and_num: (*cur).key };
    let value = (*cur).pvalue as *mut hwmon_pmu_event_value;

    if !test_bit(HWMON_ITEM_INPUT, (*value).items.as_ptr()) {
        pr_debug(
            b"hwmon_pmu: %s removing event '%s%d' that has no input file\n\0".as_ptr() as *const c_char,
            (*pmu).pmu.name,
            hwmon_type_strs[key_get_type(key)],
            key_get_num(key),
        );
        hashmap__delete(map, key.type_and_num, ptr::null_mut(), ptr::null_mut());
        zfree(&mut (*value).label as *mut _ as *mut *mut c_void);
        zfree(&mut (*value).name as *mut _ as *mut *mut c_void);
        free(value as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hwmon_pmu__new(
    pmus: *mut list_head,
    hwmon_dir: *const c_char,
    sysfs_name: *const c_char,
    name: *const c_char,
) -> *mut perf_pmu {
    let mut buf = [0 as c_char; 64];
    let type_ = PERF_PMU_TYPE_HWMON_START + strtoul(sysfs_name.add(5), ptr::null_mut(), 10) as __u32;

    if type_ > PERF_PMU_TYPE_HWMON_END {
        pr_err(b"Unable to encode hwmon type from %s in valid PMU type\n\0".as_ptr() as *const c_char, sysfs_name);
        return ptr::null_mut();
    }

    snprintf(buf.as_mut_ptr(), buf.len(), b"hwmon_%s\0".as_ptr() as *const c_char, name);
    fix_name(buf.as_mut_ptr().add(6));

    let hwm = zalloc(size_of::<hwmon_pmu>()) as *mut hwmon_pmu;
    if hwm.is_null() {
        return ptr::null_mut();
    }

    if perf_pmu__init(&mut (*hwm).pmu, type_, buf.as_ptr()) != 0 {
        perf_pmu__delete(&mut (*hwm).pmu);
        return ptr::null_mut();
    }

    (*hwm).hwmon_dir = strdup(hwmon_dir);
    if (*hwm).hwmon_dir.is_null() {
        perf_pmu__delete(&mut (*hwm).pmu);
        return ptr::null_mut();
    }
    (*hwm).pmu.alias_name = strdup(sysfs_name);
    if (*hwm).pmu.alias_name.is_null() {
        perf_pmu__delete(&mut (*hwm).pmu);
        return ptr::null_mut();
    }
    (*hwm).pmu.cpus = perf_cpu_map__new_int(0);
    if (*hwm).pmu.cpus.is_null() {
        perf_pmu__delete(&mut (*hwm).pmu);
        return ptr::null_mut();
    }
    INIT_LIST_HEAD(&mut (*hwm).pmu.format);
    INIT_LIST_HEAD(&mut (*hwm).pmu.caps);
    hashmap__init(
        &mut (*hwm).events,
        Some(hwmon_pmu__event_hashmap_hash),
        Some(hwmon_pmu__event_hashmap_equal),
        ptr::null_mut(),
    );

    list_add_tail(&mut (*hwm).pmu.list, pmus);
    &mut (*hwm).pmu
}

#[no_mangle]
pub unsafe extern "C" fn hwmon_pmu__exit(pmu: *mut perf_pmu) {
    let hwm = container_of_hwmon_pmu(pmu);
    hashmap_for_each_entry_safe(&mut (*hwm).events, Some(free_hwmon_event_value), ptr::null_mut());
    hashmap__clear(&mut (*hwm).events);
    zfree(&mut (*hwm).hwmon_dir as *mut _ as *mut *mut c_void);
}

unsafe extern "C" fn free_hwmon_event_value(
    _map: *mut hashmap,
    cur: *mut hashmap_entry,
    _ctx: *mut c_void,
) {
    let value = (*cur).pvalue as *mut hwmon_pmu_event_value;
    zfree(&mut (*value).label as *mut _ as *mut *mut c_void);
    zfree(&mut (*value).name as *mut _ as *mut *mut c_void);
    free(value as *mut c_void);
}

unsafe fn hwmon_pmu__describe_items(
    hwm: *mut hwmon_pmu,
    out_buf: *mut c_char,
    out_buf_len: size_t,
    key: hwmon_pmu_event_key,
    items: *const c_ulong,
    is_alarm: bool_,
) -> size_t {
    let mut bit = 0usize;
    let mut buf = [0 as c_char; 64];
    let mut len = 0usize;
    let dir = open((*hwm).hwmon_dir, O_CLOEXEC | O_DIRECTORY | O_RDONLY);

    if dir < 0 {
        return 0;
    }

    while bit < HWMON_ITEM__MAX {
        if !test_bit(bit, items) {
            bit += 1;
            continue;
        }
        if bit == HWMON_ITEM_LABEL || bit == HWMON_ITEM_INPUT {
            bit += 1;
            continue;
        }

        snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            b"%s%d_%s%s\0".as_ptr() as *const c_char,
            hwmon_type_strs[key_get_type(key)],
            key_get_num(key),
            hwmon_item_strs[bit],
            if is_alarm { b"_alarm\0".as_ptr() } else { b"\0".as_ptr() } as *const c_char,
        );
        let fd = openat(dir, buf.as_ptr(), O_RDONLY);
        if fd >= 0 {
            let mut read_len = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len() - 1);

            while read_len > 0 && buf[(read_len - 1) as usize] == b'\n' as c_char {
                read_len -= 1;
            }

            if read_len > 0 {
                buf[read_len as usize] = 0;
                let val = strtoll(buf.as_ptr(), ptr::null_mut(), 10);
                len += scnprintf(
                    out_buf.add(len),
                    out_buf_len - len,
                    b"%s%s%s=%g%s\0".as_ptr() as *const c_char,
                    if len == 0 { b" \0".as_ptr() } else { b", \0".as_ptr() } as *const c_char,
                    hwmon_item_strs[bit],
                    if is_alarm { b"_alarm\0".as_ptr() } else { b"\0".as_ptr() } as *const c_char,
                    (val as c_double) / 1000.0,
                    hwmon_units[key_get_type(key)],
                );
            }
            close(fd);
        }
        bit += 1;
    }
    close(dir);
    len
}

#[no_mangle]
pub unsafe extern "C" fn hwmon_pmu__for_each_event(
    pmu: *mut perf_pmu,
    state: *mut c_void,
    cb: pmu_event_callback,
) -> c_int {
    let hwm = container_of_hwmon_pmu(pmu);

    if hwmon_pmu__read_events(hwm) != 0 {
        return false as c_int;
    }

    hashmap_for_each_event(&mut (*hwm).events, pmu, state, cb)
}

#[no_mangle]
pub unsafe extern "C" fn hwmon_pmu__num_events(pmu: *mut perf_pmu) -> size_t {
    let hwm = container_of_hwmon_pmu(pmu);

    hwmon_pmu__read_events(hwm);
    hashmap__size(&mut (*hwm).events)
}

#[no_mangle]
pub unsafe extern "C" fn hwmon_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool_ {
    let hwm = container_of_hwmon_pmu(pmu);
    let mut type_: c_int = 0;
    let mut number: c_int = 0;

    if !parse_hwmon_filename(name, &mut type_, &mut number, ptr::null_mut(), ptr::null_mut()) {
        return false;
    }

    if hwmon_pmu__read_events(hwm) != 0 {
        return false;
    }

    let key = key_set(type_ as usize, number);
    if hashmap_find(&mut (*hwm).events, key.type_and_num, ptr::null_mut()) {
        return true;
    }
    if number != -1 {
        return false;
    }
    /* Item is of form <type>_ which means we should match <type>_<label>. */
    hashmap_have_event_label(&mut (*hwm).events, type_ as usize, name)
}

unsafe fn hwmon_pmu__config_term(
    hwm: *const hwmon_pmu,
    attr: *mut perf_event_attr,
    term: *mut parse_events_term,
    err: *mut parse_events_error,
) -> c_int {
    if (*term).type_term == PARSE_EVENTS__TERM_TYPE_USER {
        let mut type_: c_int = 0;
        let mut number: c_int = 0;

        if parse_hwmon_filename((*term).config, &mut type_, &mut number, ptr::null_mut(), ptr::null_mut()) {
            if number == -1 {
                /*
                 * Item is of form <type>_ which means we should
                 * match <type>_<label>.
                 */
                (*attr).config = 0;
                (*attr).config = hashmap_find_label_config(&(*hwm).events as *const _ as *mut hashmap, type_ as usize, (*term).config);
                if (*attr).config == 0 {
                    return -EINVAL;
                }
            } else {
                let key = key_set(type_ as usize, number);
                (*attr).config = key.type_and_num as u64;
            }
            return 0;
        }
    }
    if !err.is_null() {
        let mut err_str: *mut c_char = ptr::null_mut();

        parse_events_error__handle(
            err,
            (*term).err_val,
            if asprintf(
                &mut err_str,
                b"unexpected hwmon event term (%s) %s\0".as_ptr() as *const c_char,
                parse_events__term_type_str((*term).type_term),
                (*term).config,
            ) < 0 {
                strdup(b"unexpected hwmon event term\0".as_ptr() as *const c_char)
            } else {
                err_str
            },
            ptr::null_mut(),
        );
    }
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn hwmon_pmu__config_terms(
    pmu: *const perf_pmu,
    attr: *mut perf_event_attr,
    terms: *mut parse_events_terms,
    err: *mut parse_events_error,
) -> c_int {
    let hwm = container_of_hwmon_pmu(pmu as *mut perf_pmu);

    let ret = hwmon_pmu__read_events(hwm);
    if ret != 0 {
        return ret;
    }

    list_for_each_parse_events_term(&mut (*terms).terms, Some(config_term_cb), hwm as *mut c_void, attr, err)
}

unsafe extern "C" fn config_term_cb(
    term: *mut parse_events_term,
    hwm: *mut c_void,
    attr: *mut perf_event_attr,
    err: *mut parse_events_error,
) -> c_int {
    if hwmon_pmu__config_term(hwm as *const hwmon_pmu, attr, term, err) != 0 {
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn hwmon_pmu__check_alias(
    terms: *mut parse_events_terms,
    info: *mut perf_pmu_info,
    err: *mut parse_events_error,
) -> c_int {
    let term = list_first_parse_events_term(&mut (*terms).terms);

    if (*term).type_term == PARSE_EVENTS__TERM_TYPE_USER {
        let mut type_: c_int = 0;
        let mut number: c_int = 0;

        if parse_hwmon_filename((*term).config, &mut type_, &mut number, ptr::null_mut(), ptr::null_mut()) {
            (*info).unit = hwmon_units[type_ as usize];
            if type_ as usize == HWMON_TYPE_FAN
                || type_ as usize == HWMON_TYPE_PWM
                || type_ as usize == HWMON_TYPE_INTRUSION
            {
                (*info).scale = 1.0;
            } else {
                (*info).scale = 0.001;
            }
        }
        return 0;
    }
    if !err.is_null() {
        let mut err_str: *mut c_char = ptr::null_mut();

        parse_events_error__handle(
            err,
            (*term).err_val,
            if asprintf(
                &mut err_str,
                b"unexpected hwmon event term (%s) %s\0".as_ptr() as *const c_char,
                parse_events__term_type_str((*term).type_term),
                (*term).config,
            ) < 0 {
                strdup(b"unexpected hwmon event term\0".as_ptr() as *const c_char)
            } else {
                err_str
            },
            ptr::null_mut(),
        );
    }
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__read_hwmon_pmus(pmus: *mut list_head) -> c_int {
    let mut line: *mut c_char = ptr::null_mut();
    let mut class_hwmon_dir: io_dir = core::mem::zeroed();
    let mut buf = [0 as c_char; PATH_MAX];
    let sysfs = sysfs__mountpoint();

    if sysfs.is_null() {
        return 0;
    }

    scnprintf(buf.as_mut_ptr(), buf.len(), b"%s/class/hwmon/\0".as_ptr() as *const c_char, sysfs);
    io_dir__init(
        &mut class_hwmon_dir,
        open(buf.as_ptr(), O_CLOEXEC | O_DIRECTORY | O_RDONLY),
    );

    if class_hwmon_dir.dirfd < 0 {
        return 0;
    }

    loop {
        let class_hwmon_ent = io_dir__readdir(&mut class_hwmon_dir);
        if class_hwmon_ent.is_null() {
            break;
        }
        let mut line_len: size_t = 0;
        let mut io: io = core::mem::zeroed();
        let mut buf2 = [0 as c_char; 128];

        if (*class_hwmon_ent).d_type != DT_LNK {
            continue;
        }

        scnprintf(
            buf.as_mut_ptr(),
            buf.len(),
            b"%s/class/hwmon/%s\0".as_ptr() as *const c_char,
            sysfs,
            (*class_hwmon_ent).d_name.as_ptr(),
        );
        let hwmon_dir = open(buf.as_ptr(), O_DIRECTORY);
        if hwmon_dir == -1 {
            pr_debug(
                b"hwmon_pmu: not a directory: '%s/class/hwmon/%s'\n\0".as_ptr() as *const c_char,
                sysfs,
                (*class_hwmon_ent).d_name.as_ptr(),
            );
            continue;
        }
        let name_fd = openat(hwmon_dir, b"name\0".as_ptr() as *const c_char, O_RDONLY);
        if name_fd == -1 {
            pr_debug(
                b"hwmon_pmu: failure to open '%s/class/hwmon/%s/name'\n\0".as_ptr() as *const c_char,
                sysfs,
                (*class_hwmon_ent).d_name.as_ptr(),
            );
            close(hwmon_dir);
            continue;
        }
        io__init(&mut io, name_fd, buf2.as_mut_ptr(), buf2.len());
        if io__getline(&mut io, &mut line, &mut line_len) > 0 && *line.add(line_len - 1) == b'\n' as c_char {
            *line.add(line_len - 1) = 0;
        }
        hwmon_pmu__new(pmus, buf.as_ptr(), (*class_hwmon_ent).d_name.as_ptr(), line);
        close(name_fd);
        close(hwmon_dir);
    }
    free(line as *mut c_void);
    close(class_hwmon_dir.dirfd);
    0
}

/* #define FD(e, x, y) (*(int *)xyarray__entry(e->core.fd, x, y)) */
unsafe fn FD(e: *mut evsel, x: c_int, y: c_int) -> *mut c_int {
    xyarray__entry((*e).core.fd, x, y) as *mut c_int
}

#[no_mangle]
pub unsafe extern "C" fn evsel__hwmon_pmu_open(
    evsel: *mut evsel,
    threads: *mut perf_thread_map,
    start_cpu_map_idx: c_int,
    end_cpu_map_idx: c_int,
) -> c_int {
    let hwm = container_of_hwmon_pmu((*evsel).pmu);
    let key = hwmon_pmu_event_key {
        type_and_num: (*evsel).core.attr.config as c_long,
    };
    let mut idx: c_int;
    let mut thread: c_int = 0;
    let mut err: c_int = 0;
    let dir = open((*hwm).hwmon_dir, O_CLOEXEC | O_DIRECTORY | O_RDONLY);

    if dir < 0 {
        return -errno_location();
    }

    let nthreads = perf_thread_map__nr(threads);
    idx = start_cpu_map_idx;
    while idx < end_cpu_map_idx {
        thread = 0;
        while thread < nthreads {
            let mut buf = [0 as c_char; 64];
            snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                b"%s%d_input\0".as_ptr() as *const c_char,
                hwmon_type_strs[key_get_type(key)],
                key_get_num(key),
            );

            let fd = openat(dir, buf.as_ptr(), O_RDONLY);
            *FD(evsel, idx, thread) = fd;
            if fd < 0 {
                err = -errno_location();
                break;
            }
            thread += 1;
        }
        if err != 0 {
            break;
        }
        idx += 1;
    }
    if err == 0 {
        close(dir);
        return 0;
    }

    (*threads).err_thread = thread;

    loop {
        thread -= 1;
        while thread >= 0 {
            if *FD(evsel, idx, thread) >= 0 {
                close(*FD(evsel, idx, thread));
            }
            *FD(evsel, idx, thread) = -1;
            thread -= 1;
        }
        thread = nthreads;
        idx -= 1;
        if idx < 0 {
            break;
        }
    }
    close(dir);
    err
}

#[no_mangle]
pub unsafe extern "C" fn evsel__hwmon_pmu_read(
    evsel: *mut evsel,
    cpu_map_idx: c_int,
    thread: c_int,
) -> c_int {
    let mut buf = [0 as c_char; 32];
    let mut old_count: *mut perf_counts_values = ptr::null_mut();

    if !(*evsel).prev_raw_counts.is_null() {
        old_count = perf_counts((*evsel).prev_raw_counts, cpu_map_idx, thread);
    }

    let count = perf_counts((*evsel).counts, cpu_map_idx, thread);
    let fd = *FD(evsel, cpu_map_idx, thread);
    let len = pread(fd, buf.as_mut_ptr() as *mut c_void, buf.len() - 1, 0);
    if len <= 0 {
        (*count).lost += 1;
        return -EINVAL;
    }
    buf[len as usize] = 0;
    if !old_count.is_null() {
        (*count).val = (*old_count).val + strtoll(buf.as_ptr(), ptr::null_mut(), 10);
        (*count).run = (*old_count).run + 1;
        (*count).ena = (*old_count).ena + 1;
    } else {
        (*count).val = strtoll(buf.as_ptr(), ptr::null_mut(), 10);
        (*count).run += 1;
        (*count).ena += 1;
    }
    0
}

#[repr(C)]
pub struct perf_pmu {
    type_: __u32,
    sysfs_aliases_loaded: bool_,
    name: *mut c_char,
    alias_name: *mut c_char,
    cpus: *mut perf_cpu_map,
    format: list_head,
    caps: list_head,
    list: list_head,
}

#[repr(C)]
pub struct evsel {
    pmu: *mut perf_pmu,
    core: evsel_core,
    prev_raw_counts: *mut perf_counts,
    counts: *mut perf_counts,
}

#[repr(C)]
pub struct evsel_core {
    attr: perf_event_attr,
    fd: *mut xyarray,
}

#[repr(C)]
pub struct perf_event_attr {
    config: u64,
}

#[repr(C)]
pub struct perf_counts_values {
    val: c_longlong,
    ena: c_longlong,
    run: c_longlong,
    lost: c_longlong,
}

#[repr(C)]
pub struct io_dir {
    dirfd: c_int,
}

#[repr(C)]
pub struct io_dirent64 {
    d_type: u8,
    d_name: [c_char; 256],
}

#[repr(C)]
pub struct io {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    key: c_long,
    pvalue: *mut c_void,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct perf_thread_map {
    err_thread: c_int,
}

#[repr(C)]
pub struct parse_events_term {
    type_term: c_int,
    config: *mut c_char,
    err_val: c_int,
    list: list_head,
}

#[repr(C)]
pub struct parse_events_terms {
    terms: list_head,
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu_info {
    unit: *const c_char,
    scale: c_double,
}

#[repr(C)]
pub struct pmu_event_info {
    pmu: *mut perf_pmu,
    name: *mut c_char,
    alias: *mut c_char,
    scale_unit: *const c_char,
    desc: *mut c_char,
    long_desc: *mut c_char,
    encoding_desc: *mut c_char,
    topic: *const c_char,
    pmu_name: *mut c_char,
    event_type_desc: *const c_char,
}

pub enum perf_cpu_map {}
pub enum perf_counts {}
pub enum xyarray {}

type pmu_event_callback = Option<unsafe extern "C" fn(*mut c_void, *mut pmu_event_info) -> c_int>;

unsafe fn set_bit(bit: usize, addr: *mut c_ulong) {
    *addr.add(bit / BITS_PER_LONG) |= 1 << (bit % BITS_PER_LONG);
}

unsafe fn test_bit(bit: usize, addr: *const c_ulong) -> bool {
    (*addr.add(bit / BITS_PER_LONG) & (1 << (bit % BITS_PER_LONG))) != 0
}

unsafe fn container_of_hwmon_pmu(pmu: *mut perf_pmu) -> *mut hwmon_pmu {
    pmu as *mut hwmon_pmu
}

extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strlcpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtoll(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_longlong;
    fn tolower(c: c_int) -> c_int;
    fn bsearch(
        key: *const c_void,
        base: *const c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: i64) -> ssize_t;
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn scnprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> size_t;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);

    fn io_dir__init(dir: *mut io_dir, dirfd: c_int);
    fn io_dir__readdir(dir: *mut io_dir) -> *mut io_dirent64;
    fn io__init(io: *mut io, fd: c_int, buf: *mut c_char, size: size_t);
    fn io__getline(io: *mut io, line: *mut *mut c_char, line_len: *mut size_t) -> ssize_t;

    fn hashmap__init(
        map: *mut hashmap,
        hash_fn: Option<unsafe extern "C" fn(c_long, *mut c_void) -> size_t>,
        equal_fn: Option<unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool_>,
        ctx: *mut c_void,
    );
    fn hashmap__find(map: *mut hashmap, key: c_long, value: *mut *mut c_void) -> bool_;
    fn hashmap_find(map: *mut hashmap, key: c_long, value: *mut *mut c_void) -> bool_;
    fn hashmap__add(map: *mut hashmap, key: c_long, value: *mut c_void) -> c_int;
    fn hashmap__delete(map: *mut hashmap, key: c_long, old_key: *mut hwmon_pmu_event_key, old_value: *mut *mut hwmon_pmu_event_value) -> bool_;
    fn hashmap__size(map: *mut hashmap) -> size_t;
    fn hashmap__clear(map: *mut hashmap);

    fn perf_pmu__init(pmu: *mut perf_pmu, type_: __u32, name: *const c_char) -> c_int;
    fn perf_pmu__delete(pmu: *mut perf_pmu);
    fn perf_cpu_map__new_int(cpu: c_int) -> *mut perf_cpu_map;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn sysfs__mountpoint() -> *const c_char;
    fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int;
    fn perf_counts(counts: *mut perf_counts, cpu_map_idx: c_int, thread: c_int) -> *mut perf_counts_values;
    fn xyarray__entry(xy: *mut xyarray, x: c_int, y: c_int) -> *mut c_void;
    fn parse_events_error__handle(err: *mut parse_events_error, idx: c_int, str_: *mut c_char, help: *mut c_char);
    fn parse_events__term_type_str(type_term: c_int) -> *const c_char;
}

unsafe fn errno_location() -> c_int {
    extern "C" {
        fn __errno_location() -> *mut c_int;
    }
    *__errno_location()
}

extern "C" {
    fn hashmap_for_each_entry_safe(
        map: *mut hashmap,
        cb: Option<unsafe extern "C" fn(*mut hashmap, *mut hashmap_entry, *mut c_void)>,
        ctx: *mut c_void,
    );
    fn hashmap_for_each_event(
        map: *mut hashmap,
        pmu: *mut perf_pmu,
        state: *mut c_void,
        cb: pmu_event_callback,
    ) -> c_int;
    fn hashmap_have_event_label(map: *mut hashmap, type_: usize, name: *const c_char) -> bool_;
    fn hashmap_find_label_config(map: *mut hashmap, type_: usize, name: *const c_char) -> u64;
    fn list_for_each_parse_events_term(
        head: *mut list_head,
        cb: Option<unsafe extern "C" fn(*mut parse_events_term, *mut c_void, *mut perf_event_attr, *mut parse_events_error) -> c_int>,
        ctx: *mut c_void,
        attr: *mut perf_event_attr,
        err: *mut parse_events_error,
    ) -> c_int;
    fn list_first_parse_events_term(head: *mut list_head) -> *mut parse_events_term;
}
