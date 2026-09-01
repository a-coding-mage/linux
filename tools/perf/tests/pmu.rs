// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/pmu.c. C include dependencies are preserved as
// external declarations below.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type size_t = usize;
type ssize_t = isize;
type u64 = u64;
type __u64 = u64;

const PATH_MAX: usize = 4096;
const O_DIRECTORY: c_int = 0o200000;
const O_WRONLY: c_int = 0o1;
const O_CREAT: c_int = 0o100;
const EINVAL: c_int = 22;
const TEST_FAIL: c_int = -1;
const TEST_OK: c_int = 0;
const TEST_SKIP: c_int = 2;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: c_uint,
    pub size: c_uint,
    pub config: u64,
    pub config1: u64,
    pub config2: u64,
}

#[repr(C)]
pub struct perf_pmu {
    pub list: list_head,
    pub name: *const c_char,
}

#[repr(C)]
pub struct parse_events_terms {
    _private: [u8; 0],
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

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
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: c_ulong,
    pub d_off: c_ulong,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
struct test_format {
    name: *const c_char,
    value: *const c_char,
}

unsafe extern "C" {
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn system(command: *const c_char) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn mkdirat(dirfd: c_int, pathname: *const c_char, mode: c_uint) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn islower(c: c_int) -> c_int;
    fn isupper(c: c_int) -> c_int;
    fn isdigit(c: c_int) -> c_int;

    fn list_del(entry: *mut list_head);
    fn perf_pmu__delete(pmu: *mut perf_pmu);
    fn perf_pmus__add_test_pmu(dirfd: c_int, name: *const c_char) -> *mut perf_pmu;
    fn parse_events_terms__init(terms: *mut parse_events_terms);
    fn parse_events_terms__exit(terms: *mut parse_events_terms);
    fn parse_events_terms(terms: *mut parse_events_terms, str_: *const c_char) -> c_int;
    fn perf_pmu__config_terms(
        pmu: *mut perf_pmu,
        attr: *mut perf_event_attr,
        terms: *mut parse_events_terms,
        zero: bool,
        apply_hardcoded: bool,
        err: *mut c_void,
    ) -> c_int;
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn parse_events_error__init(err: *mut parse_events_error);
    fn parse_events_error__exit(err: *mut parse_events_error);
    fn parse_events(evlist: *mut evlist, str_: *const c_char, err: *mut parse_events_error) -> c_int;
    fn parse_events_error__print(err: *mut parse_events_error, str_: *const c_char);
    fn parse_events_error__contains(err: *mut parse_events_error, str_: *const c_char) -> bool;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evsel__config_exists(evsel: *mut evsel, config: *const c_char) -> bool;
    fn evsel__set_config_if_unset(evsel: *mut evsel, config: *const c_char, val: u64);
    fn evsel__get_config_val(evsel: *mut evsel, config: *const c_char, val: *mut u64) -> c_int;
    fn parse_events__decode_legacy_cache(
        name: *const c_char,
        extended_pmu_type: c_int,
        config: *mut __u64,
    ) -> c_int;
    fn sysfs__mountpoint() -> *const c_char;
    fn file_available(path: *const c_char) -> bool;
    fn pmu_name_len_no_suffix(name: *const c_char) -> size_t;
    fn pmu_name_cmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
    fn perf_pmu__wildcard_match(pmu: *mut perf_pmu, to_match: *const c_char) -> bool;
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn test_assert_equal_int(msg: *const c_char, actual: c_int, expected: c_int) {
    if actual != expected {
        pr_err(c!("Assertion failed: %s\n"), msg);
    }
}

unsafe fn test_assert_equal_bool(msg: *const c_char, actual: bool, expected: bool) {
    if actual != expected {
        pr_err(c!("Assertion failed: %s\n"), msg);
    }
}

unsafe fn test_assert_val(msg: *const c_char, val: bool) {
    if !val {
        pr_err(c!("Assertion failed: %s\n"), msg);
    }
}

/* Cleanup test PMU directory. */
unsafe fn test_pmu_put(dir: *const c_char, pmu: *mut perf_pmu) -> c_int {
    let mut buf = [0 as c_char; PATH_MAX + 20];
    let ret: c_int;

    if scnprintf(buf.as_mut_ptr(), buf.len(), c!("rm -fr %s"), dir) < 0 {
        pr_err(c!("Failure to set up buffer for \"%s\"\n"), dir);
        return -EINVAL;
    }
    ret = system(buf.as_ptr());
    if ret != 0 {
        pr_err(c!("Failure to \"%s\"\n"), buf.as_ptr());
    }

    list_del(&mut (*pmu).list);
    perf_pmu__delete(pmu);
    ret
}

/*
 * Prepare test PMU directory data, normally exported by kernel at
 * /sys/bus/event_source/devices/<pmu>/. Give as input a buffer to hold the file
 * path, the result is PMU loaded using that directory.
 */
unsafe fn test_pmu_get(dir: *mut c_char, sz: size_t) -> *mut perf_pmu {
    /* Simulated format definitions. */
    let test_formats = [
        test_format { name: c!("krava01"), value: c!("config:0-1,62-63\n") },
        test_format { name: c!("krava02"), value: c!("config:10-17\n") },
        test_format { name: c!("krava03"), value: c!("config:5\n") },
        test_format { name: c!("krava11"), value: c!("config1:0,2,4,6,8,20-28\n") },
        test_format { name: c!("krava12"), value: c!("config1:63\n") },
        test_format { name: c!("krava13"), value: c!("config1:45-47\n") },
        test_format { name: c!("krava21"), value: c!("config2:0-3,10-13,20-23,30-33,40-43,50-53,60-63\n") },
        test_format { name: c!("krava22"), value: c!("config2:8,18,48,58\n") },
        test_format { name: c!("krava23"), value: c!("config2:28-29,38\n") },
    ];
    let test_event = c!("krava01=15,krava02=170,krava03=1,krava11=27,krava12=1,krava13=2,krava21=119,krava22=11,krava23=2\n");

    let mut name = [0 as c_char; PATH_MAX];
    let mut dirfd: c_int = -1;
    let mut file: c_int;
    let mut pmu: *mut perf_pmu = core::ptr::null_mut();
    let mut len: ssize_t;

    /* Create equivalent of sysfs mount point. */
    scnprintf(dir, sz, c!("/tmp/perf-pmu-test-XXXXXX"));
    if mkdtemp(dir).is_null() {
        pr_err(c!("mkdtemp failed\n"));
        *dir = 0;
        return core::ptr::null_mut();
    }
    dirfd = open(dir, O_DIRECTORY);
    if dirfd < 0 {
        pr_err(c!("Failed to open test directory \"%s\"\n"), dir);
        goto_err_out(dir, pmu, dirfd);
        return pmu;
    }

    /* Create the test PMU directory and give it a perf_event_attr type number. */
    if mkdirat(dirfd, c!("perf-pmu-test"), 0o755) < 0 {
        pr_err(c!("Failed to mkdir PMU directory\n"));
        goto_err_out(dir, pmu, dirfd);
        return pmu;
    }
    file = openat(dirfd, c!("perf-pmu-test/type"), O_WRONLY | O_CREAT, 0o600);
    if file == 0 {
        pr_err(c!("Failed to open for writing file \"type\"\n"));
        goto_err_out(dir, pmu, dirfd);
        return pmu;
    }
    len = strlen(c!("9999")) as ssize_t;
    if write(file, c!("9999\n") as *const c_void, len as size_t) < len {
        close(file);
        pr_err(c!("Failed to write to 'type' file\n"));
        goto_err_out(dir, pmu, dirfd);
        return pmu;
    }
    close(file);

    /* Create format directory and files. */
    if mkdirat(dirfd, c!("perf-pmu-test/format"), 0o755) < 0 {
        pr_err(c!("Failed to mkdir PMU format directory\n)"));
        goto_err_out(dir, pmu, dirfd);
        return pmu;
    }
    for format in test_formats.iter() {
        if scnprintf(name.as_mut_ptr(), PATH_MAX, c!("perf-pmu-test/format/%s"), format.name) < 0 {
            pr_err(c!("Failure to set up path for \"%s\"\n"), format.name);
            goto_err_out(dir, pmu, dirfd);
            return pmu;
        }
        file = openat(dirfd, name.as_ptr(), O_WRONLY | O_CREAT, 0o600);
        if file == 0 {
            pr_err(c!("Failed to open for writing file \"%s\"\n"), name.as_ptr());
            goto_err_out(dir, pmu, dirfd);
            return pmu;
        }

        if write(file, format.value as *const c_void, strlen(format.value)) < 0 {
            pr_err(c!("Failed to write to file \"%s\"\n"), name.as_ptr());
            close(file);
            goto_err_out(dir, pmu, dirfd);
            return pmu;
        }
        close(file);
    }

    /* Create test event. */
    if mkdirat(dirfd, c!("perf-pmu-test/events"), 0o755) < 0 {
        pr_err(c!("Failed to mkdir PMU events directory\n"));
        goto_err_out(dir, pmu, dirfd);
        return pmu;
    }
    file = openat(dirfd, c!("perf-pmu-test/events/test-event"), O_WRONLY | O_CREAT, 0o600);
    if file == 0 {
        pr_err(c!("Failed to open for writing file \"type\"\n"));
        goto_err_out(dir, pmu, dirfd);
        return pmu;
    }
    len = strlen(test_event) as ssize_t;
    if write(file, test_event as *const c_void, len as size_t) < len {
        close(file);
        pr_err(c!("Failed to write to 'test-event' file\n"));
        goto_err_out(dir, pmu, dirfd);
        return pmu;
    }
    close(file);

    /* Make the PMU reading the files created above. */
    pmu = perf_pmus__add_test_pmu(dirfd, c!("perf-pmu-test"));
    if pmu.is_null() {
        pr_err(c!("Test PMU creation failed\n"));
    }

    goto_err_out(dir, pmu, dirfd);
    pmu
}

unsafe fn goto_err_out(dir: *mut c_char, pmu: *mut perf_pmu, dirfd: c_int) {
    if pmu.is_null() {
        test_pmu_put(dir, pmu);
    }
    if dirfd >= 0 {
        close(dirfd);
    }
}

unsafe extern "C" fn test__pmu_format(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut dir = [0 as c_char; PATH_MAX];
    let mut attr: perf_event_attr = core::mem::zeroed();
    let mut terms: parse_events_terms = core::mem::zeroed();
    let mut ret = TEST_FAIL;
    let pmu = test_pmu_get(dir.as_mut_ptr(), dir.len());

    if pmu.is_null() {
        return TEST_FAIL;
    }

    parse_events_terms__init(&mut terms);
    if parse_events_terms(&mut terms, c!("krava01=15,krava02=170,krava03=1,krava11=27,krava12=1,krava13=2,krava21=119,krava22=11,krava23=2")) != 0 {
        pr_err(c!("Term parsing failed\n"));
        parse_events_terms__exit(&mut terms);
        test_pmu_put(dir.as_ptr(), pmu);
        return ret;
    }

    memset(&mut attr as *mut _ as *mut c_void, 0, core::mem::size_of::<perf_event_attr>());
    ret = perf_pmu__config_terms(pmu, &mut attr, &mut terms, false, false, core::ptr::null_mut());
    if ret != 0 {
        pr_err(c!("perf_pmu__config_terms failed"));
        parse_events_terms__exit(&mut terms);
        test_pmu_put(dir.as_ptr(), pmu);
        return ret;
    }

    if attr.config != 0xc00000000002a823 {
        pr_err(c!("Unexpected config value %llx\n"), attr.config);
        parse_events_terms__exit(&mut terms);
        test_pmu_put(dir.as_ptr(), pmu);
        return ret;
    }
    if attr.config1 != 0x8000400000000145 {
        pr_err(c!("Unexpected config1 value %llx\n"), attr.config1);
        parse_events_terms__exit(&mut terms);
        test_pmu_put(dir.as_ptr(), pmu);
        return ret;
    }
    if attr.config2 != 0x0400000020041d07 {
        pr_err(c!("Unexpected config2 value %llx\n"), attr.config2);
    }

    ret = TEST_OK;
    parse_events_terms__exit(&mut terms);
    test_pmu_put(dir.as_ptr(), pmu);
    ret
}

unsafe extern "C" fn test__pmu_config_helpers(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let event = c!("perf-pmu-test/config=15,config1=4,krava02=170,krava03=1,krava11=27,krava12=1/");
    let mut terms: parse_events_terms = core::mem::zeroed();
    let mut err: parse_events_error = core::mem::zeroed();
    let mut evlist: *mut evlist;
    let pmu: *mut perf_pmu;
    let mut evsel: *mut evsel;
    let mut ret = TEST_FAIL;
    let mut dir = [0 as c_char; PATH_MAX];
    let mut val: u64 = 0;

    pmu = test_pmu_get(dir.as_mut_ptr(), dir.len());
    if pmu.is_null() {
        return TEST_FAIL;
    }

    evlist = evlist__new();
    if evlist.is_null() {
        pr_err(c!("Failed allocation"));
        parse_events_terms__exit(&mut terms);
        evlist__put(evlist);
        test_pmu_put(dir.as_ptr(), pmu);
        return ret;
    }

    parse_events_terms__init(&mut terms);
    ret = parse_events(evlist, event, &mut err);
    if ret != 0 {
        pr_debug(c!("failed to parse event '%s', err %d\n"), event, ret);
        parse_events_error__print(&mut err, event);
        if parse_events_error__contains(&mut err, c!("can't access trace events")) {
            ret = TEST_SKIP;
        }
        parse_events_terms__exit(&mut terms);
        evlist__put(evlist);
        test_pmu_put(dir.as_ptr(), pmu);
        return ret;
    }
    evsel = evlist__first(evlist);

    /* Test evsel__config_exists() */
    test_assert_equal_bool(c!("krava01 should exist"), evsel__config_exists(evsel, c!("krava01")), true);
    test_assert_equal_bool(c!("krava99 should not exist"), evsel__config_exists(evsel, c!("krava99")), false);

    /*
     * Set via config=15, krava01 bits 0-1
     * Set via config1=4, krava11 bit 1
     * Set values: krava02=170, krava03=1, krava11=27, krava12=1
     *
     * Test that already set values aren't overwritten.
     */
    evsel__set_config_if_unset(evsel, c!("krava01"), 16);
    evsel__get_config_val(evsel, c!("krava01"), &mut val);
    test_assert_equal_int(c!("krava01 overwritten"), val as c_int, 15 & 0b11);

    evsel__set_config_if_unset(evsel, c!("krava11"), 45);
    evsel__get_config_val(evsel, c!("krava11"), &mut val);
    test_assert_equal_int(c!("krava11 overwritten"), val as c_int, 27 | (4 << 1));

    evsel__set_config_if_unset(evsel, c!("krava02"), 32);
    evsel__get_config_val(evsel, c!("krava02"), &mut val);
    test_assert_equal_int(c!("krava02 overwritten"), val as c_int, 170);

    evsel__set_config_if_unset(evsel, c!("krava03"), 0);
    evsel__get_config_val(evsel, c!("krava03"), &mut val);
    test_assert_equal_int(c!("krava03 overwritten"), val as c_int, 1);

    /*
     * krava13 doesn't have any bits set by either krava13= or config1=
     * but setting _any_ raw value for config1 implies that krava13
     * shouldn't be overwritten. So it's value should remain as 0.
     */
    evsel__set_config_if_unset(evsel, c!("krava13"), 5);
    evsel__get_config_val(evsel, c!("krava13"), &mut val);
    test_assert_equal_int(c!("krava13 overwritten"), val as c_int, 0);

    /*
     * Unset values: krava21, krava22, krava23
     *
     * Test that unset values are overwritten.
     */
    evsel__set_config_if_unset(evsel, c!("krava21"), 13905);
    evsel__get_config_val(evsel, c!("krava21"), &mut val);
    test_assert_equal_int(c!("krava21 not overwritten"), val as c_int, 13905);

    evsel__set_config_if_unset(evsel, c!("krava22"), 11);
    evsel__get_config_val(evsel, c!("krava22"), &mut val);
    test_assert_equal_int(c!("krava22 not overwritten"), val as c_int, 11);

    evsel__set_config_if_unset(evsel, c!("krava23"), 0);
    evsel__get_config_val(evsel, c!("krava23"), &mut val);
    test_assert_equal_int(c!("krava23 not overwritten"), val as c_int, 0);
    ret = TEST_OK;
    parse_events_terms__exit(&mut terms);
    evlist__put(evlist);
    test_pmu_put(dir.as_ptr(), pmu);
    ret
}

unsafe extern "C" fn test__pmu_events(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut dir = [0 as c_char; PATH_MAX];
    let mut err: parse_events_error = core::mem::zeroed();
    let mut evlist: *mut evlist;
    let mut evsel: *mut evsel;
    let mut attr: *mut perf_event_attr;
    let mut ret = TEST_FAIL;
    let pmu = test_pmu_get(dir.as_mut_ptr(), dir.len());
    let event = c!("perf-pmu-test/test-event/");

    if pmu.is_null() {
        return TEST_FAIL;
    }

    evlist = evlist__new();
    if evlist.is_null() {
        pr_err(c!("Failed allocation"));
        parse_events_error__exit(&mut err);
        evlist__put(evlist);
        test_pmu_put(dir.as_ptr(), pmu);
        return ret;
    }
    parse_events_error__init(&mut err);
    ret = parse_events(evlist, event, &mut err);
    if ret != 0 {
        pr_debug(c!("failed to parse event '%s', err %d\n"), event, ret);
        parse_events_error__print(&mut err, event);
        if parse_events_error__contains(&mut err, c!("can't access trace events")) {
            ret = TEST_SKIP;
        }
        parse_events_error__exit(&mut err);
        evlist__put(evlist);
        test_pmu_put(dir.as_ptr(), pmu);
        return ret;
    }
    evsel = evlist__first(evlist);
    attr = &mut (*evsel).core.attr;
    if (*attr).config != 0xc00000000002a823 {
        pr_err(c!("Unexpected config value %llx\n"), (*attr).config);
        parse_events_error__exit(&mut err);
        evlist__put(evlist);
        test_pmu_put(dir.as_ptr(), pmu);
        return ret;
    }
    if (*attr).config1 != 0x8000400000000145 {
        pr_err(c!("Unexpected config1 value %llx\n"), (*attr).config1);
        parse_events_error__exit(&mut err);
        evlist__put(evlist);
        test_pmu_put(dir.as_ptr(), pmu);
        return ret;
    }
    if (*attr).config2 != 0x0400000020041d07 {
        pr_err(c!("Unexpected config2 value %llx\n"), (*attr).config2);
        parse_events_error__exit(&mut err);
        evlist__put(evlist);
        test_pmu_put(dir.as_ptr(), pmu);
        return ret;
    }

    ret = TEST_OK;
    parse_events_error__exit(&mut err);
    evlist__put(evlist);
    test_pmu_put(dir.as_ptr(), pmu);
    ret
}

unsafe fn permitted_event_name(name: *const c_char) -> bool {
    let mut has_lower = false;
    let mut has_upper = false;
    let mut config: __u64 = 0;

    for i in 0..strlen(name) {
        let c = *name.add(i);

        if islower(c as c_int) != 0 {
            if has_upper {
                return check_legacy(name, &mut config);
            }
            has_lower = true;
            continue;
        }
        if isupper(c as c_int) != 0 {
            if has_lower {
                return check_legacy(name, &mut config);
            }
            has_upper = true;
            continue;
        }
        if isdigit(c as c_int) == 0 && c != b'.' as c_char && c != b'_' as c_char && c != b'-' as c_char {
            return check_legacy(name, &mut config);
        }
    }
    true
}

unsafe fn check_legacy(name: *const c_char, config: *mut __u64) -> bool {
    /*
     * If the event name matches a legacy cache name the legacy encoding
     * will still be used. This isn't quite WAI as sysfs events should take
     * priority, but this case happens on PowerPC and matches the behavior
     * in older perf tools where legacy events were the priority. Be
     * permissive and assume later PMU drivers will use all lower or upper
     * case names.
     */
    if parse_events__decode_legacy_cache(name, 0, config) == 0 {
        pr_warning(c!("sysfs event '%s' should be all lower/upper case, it will be matched using legacy encoding."), name);
        return true;
    }
    false
}

unsafe extern "C" fn test__pmu_event_names(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    let mut pmu_dir: *mut DIR;
    let mut event_dir: *mut DIR;
    let mut pmu_dent: *mut dirent;
    let mut event_dent: *mut dirent;
    let sysfs = sysfs__mountpoint();
    let mut ret = TEST_OK;

    if sysfs.is_null() {
        pr_err(c!("Sysfs not mounted\n"));
        return TEST_FAIL;
    }

    snprintf(path.as_mut_ptr(), path.len(), c!("%s/bus/event_source/devices/"), sysfs);
    pmu_dir = opendir(path.as_ptr());
    if pmu_dir.is_null() {
        pr_err(c!("Error opening \"%s\"\n"), path.as_ptr());
        return TEST_FAIL;
    }
    loop {
        pmu_dent = readdir(pmu_dir);
        if pmu_dent.is_null() {
            break;
        }
        if strcmp((*pmu_dent).d_name.as_ptr(), c!(".")) == 0 || strcmp((*pmu_dent).d_name.as_ptr(), c!("..")) == 0 {
            continue;
        }

        snprintf(path.as_mut_ptr(), path.len(), c!("%s/bus/event_source/devices/%s/type"), sysfs, (*pmu_dent).d_name.as_ptr());

        /* Does it look like a PMU? */
        if !file_available(path.as_ptr()) {
            continue;
        }

        /* Process events. */
        snprintf(path.as_mut_ptr(), path.len(), c!("%s/bus/event_source/devices/%s/events"), sysfs, (*pmu_dent).d_name.as_ptr());

        event_dir = opendir(path.as_ptr());
        if event_dir.is_null() {
            pr_debug(c!("Skipping as no event directory \"%s\"\n"), path.as_ptr());
            continue;
        }
        loop {
            event_dent = readdir(event_dir);
            if event_dent.is_null() {
                break;
            }
            let event_name = (*event_dent).d_name.as_ptr();

            if strcmp(event_name, c!(".")) == 0 || strcmp(event_name, c!("..")) == 0 {
                continue;
            }

            if !permitted_event_name(event_name) {
                pr_err(c!("Invalid sysfs event name: %s/%s\n"), (*pmu_dent).d_name.as_ptr(), event_name);
                ret = TEST_FAIL;
            }
        }
        closedir(event_dir);
    }
    closedir(pmu_dir);
    ret
}

static UNCORE_CHAS: [*const c_char; 32] = [
    c!("uncore_cha_0"),
    c!("uncore_cha_1"),
    c!("uncore_cha_2"),
    c!("uncore_cha_3"),
    c!("uncore_cha_4"),
    c!("uncore_cha_5"),
    c!("uncore_cha_6"),
    c!("uncore_cha_7"),
    c!("uncore_cha_8"),
    c!("uncore_cha_9"),
    c!("uncore_cha_10"),
    c!("uncore_cha_11"),
    c!("uncore_cha_12"),
    c!("uncore_cha_13"),
    c!("uncore_cha_14"),
    c!("uncore_cha_15"),
    c!("uncore_cha_16"),
    c!("uncore_cha_17"),
    c!("uncore_cha_18"),
    c!("uncore_cha_19"),
    c!("uncore_cha_20"),
    c!("uncore_cha_21"),
    c!("uncore_cha_22"),
    c!("uncore_cha_23"),
    c!("uncore_cha_24"),
    c!("uncore_cha_25"),
    c!("uncore_cha_26"),
    c!("uncore_cha_27"),
    c!("uncore_cha_28"),
    c!("uncore_cha_29"),
    c!("uncore_cha_30"),
    c!("uncore_cha_31"),
];

static MRVL_DDRS: [*const c_char; 16] = [
    c!("mrvl_ddr_pmu_87e1b0000000"),
    c!("mrvl_ddr_pmu_87e1b1000000"),
    c!("mrvl_ddr_pmu_87e1b2000000"),
    c!("mrvl_ddr_pmu_87e1b3000000"),
    c!("mrvl_ddr_pmu_87e1b4000000"),
    c!("mrvl_ddr_pmu_87e1b5000000"),
    c!("mrvl_ddr_pmu_87e1b6000000"),
    c!("mrvl_ddr_pmu_87e1b7000000"),
    c!("mrvl_ddr_pmu_87e1b8000000"),
    c!("mrvl_ddr_pmu_87e1b9000000"),
    c!("mrvl_ddr_pmu_87e1ba000000"),
    c!("mrvl_ddr_pmu_87e1bb000000"),
    c!("mrvl_ddr_pmu_87e1bc000000"),
    c!("mrvl_ddr_pmu_87e1bd000000"),
    c!("mrvl_ddr_pmu_87e1be000000"),
    c!("mrvl_ddr_pmu_87e1bf000000"),
];

unsafe extern "C" fn test__name_len(_test: *mut test_suite, _subtest: c_int) -> c_int {
    test_assert_val(c!("cpu"), pmu_name_len_no_suffix(c!("cpu")) == strlen(c!("cpu")));
    test_assert_val(c!("i915"), pmu_name_len_no_suffix(c!("i915")) == strlen(c!("i915")));
    test_assert_val(c!("cpum_cf"), pmu_name_len_no_suffix(c!("cpum_cf")) == strlen(c!("cpum_cf")));
    for i in 0..UNCORE_CHAS.len() {
        test_assert_val(c!("Strips uncore_cha suffix"), pmu_name_len_no_suffix(UNCORE_CHAS[i]) == strlen(c!("uncore_cha")));
    }
    for i in 0..MRVL_DDRS.len() {
        test_assert_val(c!("Strips mrvl_ddr_pmu suffix"), pmu_name_len_no_suffix(MRVL_DDRS[i]) == strlen(c!("mrvl_ddr_pmu")));
    }
    TEST_OK
}

unsafe extern "C" fn test__name_cmp(_test: *mut test_suite, _subtest: c_int) -> c_int {
    test_assert_equal_int(c!("cpu"), pmu_name_cmp(c!("cpu"), c!("cpu")), 0);
    test_assert_equal_int(c!("i915"), pmu_name_cmp(c!("i915"), c!("i915")), 0);
    test_assert_equal_int(c!("cpum_cf"), pmu_name_cmp(c!("cpum_cf"), c!("cpum_cf")), 0);
    test_assert_val(c!("i915"), pmu_name_cmp(c!("cpu"), c!("i915")) < 0);
    test_assert_val(c!("i915"), pmu_name_cmp(c!("i915"), c!("cpu")) > 0);
    test_assert_val(c!("cpum_cf"), pmu_name_cmp(c!("cpum_cf"), c!("cpum_ce")) > 0);
    test_assert_val(c!("cpum_cf"), pmu_name_cmp(c!("cpum_cf"), c!("cpum_d0")) < 0);
    for i in 1..UNCORE_CHAS.len() {
        test_assert_val(c!("uncore_cha suffixes ordered lt"), pmu_name_cmp(UNCORE_CHAS[i - 1], UNCORE_CHAS[i]) < 0);
        test_assert_val(c!("uncore_cha suffixes ordered gt"), pmu_name_cmp(UNCORE_CHAS[i], UNCORE_CHAS[i - 1]) > 0);
    }
    for i in 1..MRVL_DDRS.len() {
        test_assert_val(c!("mrvl_ddr_pmu suffixes ordered lt"), pmu_name_cmp(MRVL_DDRS[i - 1], MRVL_DDRS[i]) < 0);
        test_assert_val(c!("mrvl_ddr_pmu suffixes ordered gt"), pmu_name_cmp(MRVL_DDRS[i], MRVL_DDRS[i - 1]) > 0);
    }
    TEST_OK
}

/**
 * Test perf_pmu__wildcard_match() that's used to search for a PMU given a name passed
 * on the command line. The name that's passed may also be a filename type glob
 * match. If the name does not match, perf_pmu__wildcard_match() attempts to match the
 * alias of the PMU, if provided.
 */
unsafe extern "C" fn test__pmu_match(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut test_pmu = perf_pmu {
        list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() },
        name: c!("pmuname"),
    };

    test_assert_equal_bool(c!("Exact match"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname")), true);
    test_assert_equal_bool(c!("Longer token"), perf_pmu__wildcard_match(&mut test_pmu, c!("longertoken")), false);
    test_assert_equal_bool(c!("Shorter token"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmu")), false);

    test_pmu.name = c!("pmuname_10");
    test_assert_equal_bool(c!("Diff suffix_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_2")), false);
    test_assert_equal_bool(c!("Sub suffix_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_1")), true);
    test_assert_equal_bool(c!("Same suffix_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_10")), true);
    test_assert_equal_bool(c!("No suffix_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname")), true);
    test_assert_equal_bool(c!("Underscore_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_")), true);
    test_assert_equal_bool(c!("Substring_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuna")), false);

    test_pmu.name = c!("pmuname_ab23");
    test_assert_equal_bool(c!("Diff suffix hex_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_2")), false);
    test_assert_equal_bool(c!("Sub suffix hex_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_ab")), true);
    test_assert_equal_bool(c!("Same suffix hex_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_ab23")), true);
    test_assert_equal_bool(c!("No suffix hex_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname")), true);
    test_assert_equal_bool(c!("Underscore hex_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_")), true);
    test_assert_equal_bool(c!("Substring hex_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuna")), false);

    test_pmu.name = c!("pmuname10");
    test_assert_equal_bool(c!("Diff suffix"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname2")), false);
    test_assert_equal_bool(c!("Sub suffix"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname1")), true);
    test_assert_equal_bool(c!("Same suffix"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname10")), true);
    test_assert_equal_bool(c!("No suffix"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname")), true);
    test_assert_equal_bool(c!("Underscore"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_")), false);
    test_assert_equal_bool(c!("Substring"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuna")), false);

    test_pmu.name = c!("pmunameab23");
    test_assert_equal_bool(c!("Diff suffix hex"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname2")), false);
    test_assert_equal_bool(c!("Sub suffix hex"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmunameab")), true);
    test_assert_equal_bool(c!("Same suffix hex"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmunameab23")), true);
    test_assert_equal_bool(c!("No suffix hex"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname")), true);
    test_assert_equal_bool(c!("Underscore hex"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_")), false);
    test_assert_equal_bool(c!("Substring hex"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuna")), false);

    /*
     * 2 hex chars or less are not considered suffixes so it shouldn't be
     * possible to wildcard by skipping the suffix. Therefore there are more
     * false results here than above.
     */
    test_pmu.name = c!("pmuname_a3");
    test_assert_equal_bool(c!("Diff suffix 2 hex_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_2")), false);
    /*
     * This one should be false, but because pmuname_a3 ends in 3 which is
     * decimal, it's not possible to determine if it's a short hex suffix or
     * a normal decimal suffix following text. And we want to match on any
     * length of decimal suffix. Run the test anyway and expect the wrong
     * result. And slightly fuzzy matching shouldn't do too much harm.
     */
    test_assert_equal_bool(c!("Sub suffix 2 hex_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_a")), true);
    test_assert_equal_bool(c!("Same suffix 2 hex_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_a3")), true);
    test_assert_equal_bool(c!("No suffix 2 hex_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname")), false);
    test_assert_equal_bool(c!("Underscore 2 hex_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_")), false);
    test_assert_equal_bool(c!("Substring 2 hex_"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuna")), false);

    test_pmu.name = c!("pmuname_5");
    test_assert_equal_bool(c!("Glob 1"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmu*")), true);
    test_assert_equal_bool(c!("Glob 2"), perf_pmu__wildcard_match(&mut test_pmu, c!("nomatch*")), false);
    test_assert_equal_bool(c!("Seq 1"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_[12345]")), true);
    test_assert_equal_bool(c!("Seq 2"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_[67890]")), false);
    test_assert_equal_bool(c!("? 1"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_?")), true);
    test_assert_equal_bool(c!("? 2"), perf_pmu__wildcard_match(&mut test_pmu, c!("pmuname_1?")), false);

    TEST_OK
}

#[unsafe(no_mangle)]
pub static mut tests__pmu: [test_case; 8] = [
    test_case { name: c!("Parsing with PMU format directory"), run_case: Some(test__pmu_format) },
    test_case { name: c!("Parsing with PMU event"), run_case: Some(test__pmu_events) },
    test_case { name: c!("PMU event names"), run_case: Some(test__pmu_event_names) },
    test_case { name: c!("PMU name combining"), run_case: Some(test__name_len) },
    test_case { name: c!("PMU name comparison"), run_case: Some(test__name_cmp) },
    test_case { name: c!("PMU cmdline match"), run_case: Some(test__pmu_match) },
    test_case { name: c!("PMU config helpers"), run_case: Some(test__pmu_config_helpers) },
    test_case { name: core::ptr::null(), run_case: None },
];

#[unsafe(no_mangle)]
pub static mut suite__pmu: test_suite = test_suite {
    desc: c!("Sysfs PMU tests"),
    test_cases: unsafe { tests__pmu.as_mut_ptr() },
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
