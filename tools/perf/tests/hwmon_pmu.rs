// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Translated from perf/tests/hwmon_pmu.c.
// C include dependencies intentionally remain external to this isolated file:
// hwmon_pmu.h, debug.h, evlist.h, parse-events.h, pmus.h, tests.h, and libc/kernel APIs.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type u64 = u64;

const PATH_MAX: usize = 4096;
const O_PATH: c_int = 0o10000000;
const O_DIRECTORY: c_int = 0o200000;
const O_WRONLY: c_int = 0o1;
const O_CREAT: c_int = 0o100;
const EINVAL: c_int = 22;
const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_pmu {
    pub list: list_head,
    pub name: *const c_char,
}

#[repr(C)]
pub struct perf_event_attr {
    pub config: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub pmu: *mut perf_pmu,
    pub core: evsel_core,
}

#[repr(C)]
pub struct evlist;

#[repr(C)]
pub struct parse_events_error;

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_type {
    HWMON_TYPE_NONE = 0,
    HWMON_TYPE_CPU,
    HWMON_TYPE_TEMP,
    HWMON_TYPE_FAN,
    HWMON_TYPE_POWER,
    HWMON_TYPE_INTRUSION,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_item {
    HWMON_ITEM_NONE = 0,
    HWMON_ITEM_ACCURACY,
    HWMON_ITEM_INPUT,
    HWMON_ITEM_VID,
    HWMON_ITEM_CRIT,
    HWMON_ITEM_AVERAGE_INTERVAL_MIN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwmon_pmu_event_key_fields {
    pub num: c_int,
    pub type_: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hwmon_pmu_event_key {
    pub type_and_num: c_long,
    pub fields: hwmon_pmu_event_key_fields,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct test_event {
    name: *const c_char,
    alias: *const c_char,
    key: hwmon_pmu_event_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct test_item {
    name: *const c_char,
    value: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct hwmon_parse_test {
    filename: *const c_char,
    type_: hwmon_type,
    number: c_int,
    item: hwmon_item,
    alarm: bool,
    parse_ok: bool,
}

extern "C" {
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn strlcpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn strncat(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn mkdirat(dirfd: c_int, pathname: *const c_char, mode: c_ulong) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn list_del(entry: *mut list_head);
    fn perf_pmu__delete(pmu: *mut perf_pmu);
    fn perf_pmus__add_test_hwmon_pmu(
        dir: *const c_char,
        name: *const c_char,
        sysfs_name: *const c_char,
    ) -> *mut perf_pmu;
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evsel__next(evsel: *mut evsel) -> *mut evsel;
    fn parse_events_error__init(err: *mut parse_events_error);
    fn parse_events_error__print(err: *mut parse_events_error, event: *const c_char);
    fn parse_events_error__exit(err: *mut parse_events_error);
    fn parse_events(evlist: *mut evlist, str_: *const c_char, err: *mut parse_events_error) -> c_int;
    fn parse_hwmon_filename(
        filename: *const c_char,
        type_: *mut hwmon_type,
        number: *mut c_int,
        item: *mut hwmon_item,
        alarm: *mut bool,
    ) -> bool;
    fn test_assert_equal(
        file: *const c_char,
        line: c_int,
        name: *const c_char,
        actual: c_long,
        expected: c_long,
    ) -> c_int;
}

static TEST_EVENTS: [test_event; 2] = [
    test_event {
        name: b"temp_test_hwmon_event1\0".as_ptr() as *const c_char,
        alias: b"temp1\0".as_ptr() as *const c_char,
        key: hwmon_pmu_event_key {
            fields: hwmon_pmu_event_key_fields { num: 1, type_: 10 },
        },
    },
    test_event {
        name: b"temp_test_hwmon_event2\0".as_ptr() as *const c_char,
        alias: b"temp2\0".as_ptr() as *const c_char,
        key: hwmon_pmu_event_key {
            fields: hwmon_pmu_event_key_fields { num: 2, type_: 10 },
        },
    },
];

/* Cleanup test PMU directory. */
unsafe extern "C" fn test_pmu_put(dir: *const c_char, hwm: *mut perf_pmu) -> c_int {
    let mut buf = [0 as c_char; PATH_MAX + 20];
    let ret: c_int;

    if scnprintf(
        buf.as_mut_ptr(),
        buf.len(),
        b"rm -fr %s\0".as_ptr() as *const c_char,
        dir,
    ) < 0
    {
        pr_err(
            b"Failure to set up buffer for \"%s\"\n\0".as_ptr() as *const c_char,
            dir,
        );
        return -EINVAL;
    }
    ret = system(buf.as_ptr());
    if ret != 0 {
        pr_err(b"Failure to \"%s\"\n\0".as_ptr() as *const c_char, buf.as_ptr());
    }

    list_del(&mut (*hwm).list);
    perf_pmu__delete(hwm);
    ret
}

/*
 * Prepare test PMU directory data, normally exported by kernel at
 * /sys/class/hwmon/hwmon<number>/. Give as input a buffer to hold the file
 * path, the result is PMU loaded using that directory.
 */
unsafe extern "C" fn test_pmu_get(dir: *mut c_char, sz: size_t) -> *mut perf_pmu {
    let test_hwmon_name_nl = b"A test hwmon PMU\n\0".as_ptr() as *const c_char;
    let test_hwmon_name = b"A test hwmon PMU\0".as_ptr() as *const c_char;
    /* Simulated hwmon items. */
    let test_items = [
        test_item {
            name: b"temp1_label\0".as_ptr() as *const c_char,
            value: b"test hwmon event1\n\0".as_ptr() as *const c_char,
        },
        test_item {
            name: b"temp1_input\0".as_ptr() as *const c_char,
            value: b"40000\n\0".as_ptr() as *const c_char,
        },
        test_item {
            name: b"temp2_label\0".as_ptr() as *const c_char,
            value: b"test hwmon event2\n\0".as_ptr() as *const c_char,
        },
        test_item {
            name: b"temp2_input\0".as_ptr() as *const c_char,
            value: b"50000\n\0".as_ptr() as *const c_char,
        },
    ];
    let mut hwmon_dirfd: c_int = -1;
    let mut test_dirfd: c_int = -1;
    let mut file: c_int;
    let mut hwm: *mut perf_pmu = ptr::null_mut();
    let mut len: ssize_t;

    /* Create equivalent of sysfs mount point. */
    scnprintf(
        dir,
        sz,
        b"/tmp/perf-hwmon-pmu-test-XXXXXX\0".as_ptr() as *const c_char,
    );
    if mkdtemp(dir).is_null() {
        pr_err(b"mkdtemp failed\n\0".as_ptr() as *const c_char);
        *dir = 0;
        return ptr::null_mut();
    }
    test_dirfd = open(dir, O_PATH | O_DIRECTORY);
    if test_dirfd < 0 {
        pr_err(
            b"Failed to open test directory \"%s\"\n\0".as_ptr() as *const c_char,
            dir,
        );
        goto_err_out(dir, hwm, test_dirfd, hwmon_dirfd)
    } else {
        /* Create the test hwmon directory and give it a name. */
        if mkdirat(
            test_dirfd,
            b"hwmon1234\0".as_ptr() as *const c_char,
            0o755,
        ) < 0
        {
            pr_err(b"Failed to mkdir hwmon directory\n\0".as_ptr() as *const c_char);
            return goto_err_out(dir, hwm, test_dirfd, hwmon_dirfd);
        }
        strncat(
            dir,
            b"/hwmon1234\0".as_ptr() as *const c_char,
            sz - strlen(dir),
        );
        hwmon_dirfd = open(dir, O_PATH | O_DIRECTORY);
        if hwmon_dirfd < 0 {
            pr_err(
                b"Failed to open test hwmon directory \"%s\"\n\0".as_ptr() as *const c_char,
                dir,
            );
            return goto_err_out(dir, hwm, test_dirfd, hwmon_dirfd);
        }
        file = openat(
            hwmon_dirfd,
            b"name\0".as_ptr() as *const c_char,
            O_WRONLY | O_CREAT,
            0o600,
        );
        if file < 0 {
            pr_err(b"Failed to open for writing file \"name\"\n\0".as_ptr() as *const c_char);
            return goto_err_out(dir, hwm, test_dirfd, hwmon_dirfd);
        }
        len = strlen(test_hwmon_name_nl) as ssize_t;
        if write(file, test_hwmon_name_nl as *const c_void, len as size_t) < len {
            close(file);
            pr_err(b"Failed to write to 'name' file\n\0".as_ptr() as *const c_char);
            return goto_err_out(dir, hwm, test_dirfd, hwmon_dirfd);
        }
        close(file);

        /* Create test hwmon files. */
        let mut i: size_t = 0;
        while i < test_items.len() {
            let item = &test_items[i];

            file = openat(hwmon_dirfd, item.name, O_WRONLY | O_CREAT, 0o600);
            if file < 0 {
                pr_err(
                    b"Failed to open for writing file \"%s\"\n\0".as_ptr() as *const c_char,
                    item.name,
                );
                return goto_err_out(dir, hwm, test_dirfd, hwmon_dirfd);
            }

            if write(
                file,
                item.value as *const c_void,
                strlen(item.value),
            ) < 0
            {
                pr_err(
                    b"Failed to write to file \"%s\"\n\0".as_ptr() as *const c_char,
                    item.name,
                );
                close(file);
                return goto_err_out(dir, hwm, test_dirfd, hwmon_dirfd);
            }
            close(file);
            i += 1;
        }

        /* Make the PMU reading the files created above. */
        hwm = perf_pmus__add_test_hwmon_pmu(
            dir,
            b"hwmon1234\0".as_ptr() as *const c_char,
            test_hwmon_name,
        );
        if hwm.is_null() {
            pr_err(b"Test hwmon creation failed\n\0".as_ptr() as *const c_char);
        }

        return goto_err_out(dir, hwm, test_dirfd, hwmon_dirfd);
    }
}

unsafe fn goto_err_out(
    dir: *mut c_char,
    hwm: *mut perf_pmu,
    test_dirfd: c_int,
    hwmon_dirfd: c_int,
) -> *mut perf_pmu {
    if hwm.is_null() {
        test_pmu_put(dir, hwm);
    }
    if test_dirfd >= 0 {
        close(test_dirfd);
    }
    if hwmon_dirfd >= 0 {
        close(hwmon_dirfd);
    }
    hwm
}

unsafe extern "C" fn do_test(i: size_t, with_pmu: bool, with_alias: bool) -> c_int {
    let test_event = if with_alias {
        TEST_EVENTS[i].alias
    } else {
        TEST_EVENTS[i].name
    };
    let evlist = evlist__new();
    let mut evsel: *mut evsel;
    let mut err: parse_events_error = mem::zeroed();
    let mut ret: c_int;
    let mut str_ = [0 as c_char; 128];
    let mut found = false;

    if evlist.is_null() {
        pr_err(b"evlist allocation failed\n\0".as_ptr() as *const c_char);
        return TEST_FAIL;
    }

    if with_pmu {
        snprintf(
            str_.as_mut_ptr(),
            str_.len(),
            b"hwmon_a_test_hwmon_pmu/%s/\0".as_ptr() as *const c_char,
            test_event,
        );
    } else {
        strlcpy(str_.as_mut_ptr(), test_event, str_.len());
    }

    pr_debug(b"Testing '%s'\n\0".as_ptr() as *const c_char, str_.as_ptr());
    parse_events_error__init(&mut err);
    ret = parse_events(evlist, str_.as_ptr(), &mut err);
    if ret != 0 {
        pr_debug(
            b"FAILED %s:%d failed to parse event '%s', err %d\n\0".as_ptr() as *const c_char,
            b"hwmon_pmu.c\0".as_ptr() as *const c_char,
            line!() as c_int,
            str_.as_ptr(),
            ret,
        );
        parse_events_error__print(&mut err, str_.as_ptr());
        ret = TEST_FAIL;
        parse_events_error__exit(&mut err);
        evlist__put(evlist);
        return ret;
    }

    ret = TEST_OK;
    if if with_pmu {
        evlist__nr_entries(evlist) != 1
    } else {
        evlist__nr_entries(evlist) < 1
    } {
        pr_debug(
            b"FAILED %s:%d Unexpected number of events for '%s' of %d\n\0".as_ptr()
                as *const c_char,
            b"hwmon_pmu.c\0".as_ptr() as *const c_char,
            line!() as c_int,
            str_.as_ptr(),
            evlist__nr_entries(evlist),
        );
        ret = TEST_FAIL;
        parse_events_error__exit(&mut err);
        evlist__put(evlist);
        return ret;
    }

    evsel = evlist__first(evlist);
    while !evsel.is_null() {
        if !(*evsel).pmu.is_null()
            && !(*(*evsel).pmu).name.is_null()
            && strcmp(
                (*(*evsel).pmu).name,
                b"hwmon_a_test_hwmon_pmu\0".as_ptr() as *const c_char,
            ) == 0
        {
            if (*evsel).core.attr.config != TEST_EVENTS[i].key.type_and_num as u64 {
                pr_debug(
                    b"FAILED %s:%d Unexpected config for '%s', %lu != %ld\n\0".as_ptr()
                        as *const c_char,
                    b"hwmon_pmu.c\0".as_ptr() as *const c_char,
                    line!() as c_int,
                    str_.as_ptr(),
                    (*evsel).core.attr.config,
                    TEST_EVENTS[i].key.type_and_num,
                );
                ret = TEST_FAIL;
                parse_events_error__exit(&mut err);
                evlist__put(evlist);
                return ret;
            }
            found = true;
        }

        evsel = evsel__next(evsel);
    }

    if !found {
        pr_debug(
            b"FAILED %s:%d Didn't find hwmon event '%s' in parsed evsels\n\0".as_ptr()
                as *const c_char,
            b"hwmon_pmu.c\0".as_ptr() as *const c_char,
            line!() as c_int,
            str_.as_ptr(),
        );
        ret = TEST_FAIL;
    }

    parse_events_error__exit(&mut err);
    evlist__put(evlist);
    ret
}

unsafe extern "C" fn test__hwmon_pmu(with_pmu: bool) -> c_int {
    let mut dir = [0 as c_char; PATH_MAX];
    let pmu = test_pmu_get(dir.as_mut_ptr(), dir.len());
    let mut ret = TEST_OK;

    if pmu.is_null() {
        return TEST_FAIL;
    }

    let mut i: size_t = 0;
    while i < TEST_EVENTS.len() {
        ret = do_test(i, with_pmu, false);

        if ret != TEST_OK {
            break;
        }

        ret = do_test(i, with_pmu, true);

        if ret != TEST_OK {
            break;
        }
        i += 1;
    }
    test_pmu_put(dir.as_ptr(), pmu);
    ret
}

unsafe extern "C" fn test__hwmon_pmu_without_pmu(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    test__hwmon_pmu(false)
}

unsafe extern "C" fn test__hwmon_pmu_with_pmu(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    test__hwmon_pmu(true)
}

unsafe extern "C" fn test__parse_hwmon_filename(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let tests = [
        hwmon_parse_test {
            filename: b"cpu0_accuracy\0".as_ptr() as *const c_char,
            type_: hwmon_type::HWMON_TYPE_CPU,
            number: 0,
            item: hwmon_item::HWMON_ITEM_ACCURACY,
            alarm: false,
            parse_ok: true,
        },
        hwmon_parse_test {
            filename: b"temp1_input\0".as_ptr() as *const c_char,
            type_: hwmon_type::HWMON_TYPE_TEMP,
            number: 1,
            item: hwmon_item::HWMON_ITEM_INPUT,
            alarm: false,
            parse_ok: true,
        },
        hwmon_parse_test {
            filename: b"fan2_vid\0".as_ptr() as *const c_char,
            type_: hwmon_type::HWMON_TYPE_FAN,
            number: 2,
            item: hwmon_item::HWMON_ITEM_VID,
            alarm: false,
            parse_ok: true,
        },
        hwmon_parse_test {
            filename: b"power3_crit_alarm\0".as_ptr() as *const c_char,
            type_: hwmon_type::HWMON_TYPE_POWER,
            number: 3,
            item: hwmon_item::HWMON_ITEM_CRIT,
            alarm: true,
            parse_ok: true,
        },
        hwmon_parse_test {
            filename: b"intrusion4_average_interval_min_alarm\0".as_ptr() as *const c_char,
            type_: hwmon_type::HWMON_TYPE_INTRUSION,
            number: 4,
            item: hwmon_item::HWMON_ITEM_AVERAGE_INTERVAL_MIN,
            alarm: true,
            parse_ok: true,
        },
        hwmon_parse_test {
            filename: b"badtype5_baditem\0".as_ptr() as *const c_char,
            type_: hwmon_type::HWMON_TYPE_NONE,
            number: 5,
            item: hwmon_item::HWMON_ITEM_NONE,
            alarm: false,
            parse_ok: false,
        },
        hwmon_parse_test {
            filename: b"humidity6_baditem\0".as_ptr() as *const c_char,
            type_: hwmon_type::HWMON_TYPE_NONE,
            number: 6,
            item: hwmon_item::HWMON_ITEM_NONE,
            alarm: false,
            parse_ok: false,
        },
    ];

    let mut i: size_t = 0;
    while i < tests.len() {
        let mut type_: hwmon_type = mem::zeroed();
        let mut number: c_int = 0;
        let mut item: hwmon_item = mem::zeroed();
        let mut alarm = false;

        test_assert_equal(
            b"hwmon_pmu.c\0".as_ptr() as *const c_char,
            line!() as c_int,
            b"parse_hwmon_filename\0".as_ptr() as *const c_char,
            parse_hwmon_filename(
                tests[i].filename,
                &mut type_,
                &mut number,
                &mut item,
                &mut alarm,
            ) as c_long,
            tests[i].parse_ok as c_long,
        );
        if tests[i].parse_ok {
            test_assert_equal(
                b"hwmon_pmu.c\0".as_ptr() as *const c_char,
                line!() as c_int,
                b"parse_hwmon_filename type\0".as_ptr() as *const c_char,
                type_ as c_long,
                tests[i].type_ as c_long,
            );
            test_assert_equal(
                b"hwmon_pmu.c\0".as_ptr() as *const c_char,
                line!() as c_int,
                b"parse_hwmon_filename number\0".as_ptr() as *const c_char,
                number as c_long,
                tests[i].number as c_long,
            );
            test_assert_equal(
                b"hwmon_pmu.c\0".as_ptr() as *const c_char,
                line!() as c_int,
                b"parse_hwmon_filename item\0".as_ptr() as *const c_char,
                item as c_long,
                tests[i].item as c_long,
            );
            test_assert_equal(
                b"hwmon_pmu.c\0".as_ptr() as *const c_char,
                line!() as c_int,
                b"parse_hwmon_filename alarm\0".as_ptr() as *const c_char,
                alarm as c_long,
                tests[i].alarm as c_long,
            );
        }
        i += 1;
    }
    TEST_OK
}

#[no_mangle]
pub static mut tests__hwmon_pmu: [test_case; 4] = [
    test_case {
        name: b"Basic parsing test\0".as_ptr() as *const c_char,
        run_case: Some(test__parse_hwmon_filename),
    },
    test_case {
        name: b"Parsing without PMU name\0".as_ptr() as *const c_char,
        run_case: Some(test__hwmon_pmu_without_pmu),
    },
    test_case {
        name: b"Parsing with PMU name\0".as_ptr() as *const c_char,
        run_case: Some(test__hwmon_pmu_with_pmu),
    },
    test_case {
        name: ptr::null(),
        run_case: None,
    },
];

#[no_mangle]
pub static mut suite__hwmon_pmu: test_suite = test_suite {
    desc: b"Hwmon PMU\0".as_ptr() as *const c_char,
    test_cases: unsafe { tests__hwmon_pmu.as_mut_ptr() },
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
