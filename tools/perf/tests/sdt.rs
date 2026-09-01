// SPDX-License-Identifier: GPL-2.0
//
// C dependencies intentionally not translated as executable Rust:
// errno.h, limits.h, stdio.h, stdlib.h, unistd.h, sys/epoll.h,
// util/symbol.h, linux/filter.h, tests.h, debug.h, probe-file.h,
// build-id.h, util.h.

use std::ffi::{c_char, c_int, c_void};
use std::mem::size_of_val;
use std::ptr;

const PATH_MAX: usize = 4096;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;

// External constants/types supplied by the surrounding perf test framework.
const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = 2;
const SBUILD_ID_SIZE: usize = 64;

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct probe_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct build_id {
    pub size: c_int,
}

unsafe extern "C" {
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: usize) -> isize;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;

    fn pr_debug(fmt: *const c_char, ...);
    fn filename__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
    fn build_id__snprintf(bid: *const build_id, sbuf: *mut c_char, size: usize) -> c_int;
    fn build_id_cache__add_s(
        sbuild_id: *const c_char,
        filename: *const c_char,
        debugdir: *const c_char,
        is_kallsyms: bool,
        is_vdso: bool,
    ) -> c_int;
    fn probe_cache__new(target: *const c_char, namespace: *const c_char) -> *mut probe_cache;
    fn probe_cache__find_by_name(
        cache: *mut probe_cache,
        group: *const c_char,
        event: *const c_char,
    ) -> *mut c_void;
    fn probe_cache__delete(cache: *mut probe_cache);
    fn set_buildid_dir(dir: *const c_char);
    fn rm_rf(path: *const c_char) -> c_int;
}

// To test SDT event, we need libelf support to scan elf binary.
// Original C conditional: defined(HAVE_SDT_EVENT) && defined(HAVE_LIBELF_SUPPORT)

#[cfg(all(HAVE_SDT_EVENT, HAVE_LIBELF_SUPPORT))]
unsafe fn target_function() -> c_int {
    // Original C: DTRACE_PROBE(perf, test_target);
    DTRACE_PROBE!("perf", "test_target");
    TEST_OK
}

// Copied from builtin-buildid-cache.c
#[cfg(all(HAVE_SDT_EVENT, HAVE_LIBELF_SUPPORT))]
unsafe fn build_id_cache__add_file(filename: *const c_char) -> c_int {
    let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE];
    let mut bid = build_id { size: 0 };
    let mut err: c_int;

    err = filename__read_build_id(filename, &mut bid);
    if err < 0 {
        pr_debug(
            b"Failed to read build id of %s\n\0".as_ptr() as *const c_char,
            filename,
        );
        return err;
    }

    build_id__snprintf(&bid, sbuild_id.as_mut_ptr(), size_of_val(&sbuild_id));
    err = build_id_cache__add_s(sbuild_id.as_ptr(), filename, ptr::null(), false, false);
    if err < 0 {
        pr_debug(
            b"Failed to add build id cache of %s\n\0".as_ptr() as *const c_char,
            filename,
        );
    }
    err
}

#[cfg(all(HAVE_SDT_EVENT, HAVE_LIBELF_SUPPORT))]
unsafe fn get_self_path() -> *mut c_char {
    let buf = calloc(PATH_MAX, size_of_val(&(0 as c_char))) as *mut c_char;

    if !buf.is_null()
        && readlink(
            b"/proc/self/exe\0".as_ptr() as *const c_char,
            buf,
            PATH_MAX - 1,
        ) < 0
    {
        pr_debug(b"Failed to get correct path of perf\n\0".as_ptr() as *const c_char);
        free(buf as *mut c_void);
        return ptr::null_mut();
    }
    buf
}

#[cfg(all(HAVE_SDT_EVENT, HAVE_LIBELF_SUPPORT))]
unsafe fn search_cached_probe(
    target: *const c_char,
    group: *const c_char,
    event: *const c_char,
) -> c_int {
    let cache = probe_cache__new(target, ptr::null());
    let mut ret: c_int = 0;

    if cache.is_null() {
        pr_debug(
            b"Failed to open probe cache of %s\n\0".as_ptr() as *const c_char,
            target,
        );
        return -EINVAL;
    }

    if probe_cache__find_by_name(cache, group, event).is_null() {
        pr_debug(
            b"Failed to find %s:%s in the cache\n\0".as_ptr() as *const c_char,
            group,
            event,
        );
        ret = -ENOENT;
    }
    probe_cache__delete(cache);

    ret
}

#[cfg(all(HAVE_SDT_EVENT, HAVE_LIBELF_SUPPORT))]
unsafe fn test__sdt_event(_test: *mut test_suite, _subtests: c_int) -> c_int {
    let mut ret: c_int = TEST_FAIL;
    let mut __tempdir = *b"./test-buildid-XXXXXX\0";
    let mut tempdir: *mut c_char = ptr::null_mut();
    let myself = get_self_path();

    if myself.is_null() || mkdtemp(__tempdir.as_mut_ptr() as *mut c_char).is_null() {
        pr_debug(
            b"Failed to make a tempdir for build-id cache\n\0".as_ptr() as *const c_char,
        );
        goto_error(tempdir, myself);
        return ret;
    }
    // Note that buildid_dir must be an absolute path.
    tempdir = realpath(__tempdir.as_ptr() as *const c_char, ptr::null_mut());
    if tempdir.is_null() {
        rm_rf(__tempdir.as_ptr() as *const c_char);
        goto_error(tempdir, myself);
        return ret;
    }

    // At first, scan itself.
    set_buildid_dir(tempdir);
    if build_id_cache__add_file(myself) < 0 {
        rm_rf(__tempdir.as_ptr() as *const c_char);
        goto_error(tempdir, myself);
        return ret;
    }

    // Open a cache and make sure the SDT is stored.
    if search_cached_probe(
        myself,
        b"sdt_perf\0".as_ptr() as *const c_char,
        b"test_target\0".as_ptr() as *const c_char,
    ) < 0
    {
        rm_rf(__tempdir.as_ptr() as *const c_char);
        goto_error(tempdir, myself);
        return ret;
    }

    // TBD: probing on the SDT event and collect logs.

    // Call the target and get an event.
    ret = target_function();

    // Cleanup temporary buildid dir.
    rm_rf(__tempdir.as_ptr() as *const c_char);
    goto_error(tempdir, myself);
    ret
}

#[cfg(all(HAVE_SDT_EVENT, HAVE_LIBELF_SUPPORT))]
unsafe fn goto_error(tempdir: *mut c_char, myself: *mut c_char) {
    free(tempdir as *mut c_void);
    free(myself as *mut c_void);
}

#[cfg(not(all(HAVE_SDT_EVENT, HAVE_LIBELF_SUPPORT)))]
unsafe fn test__sdt_event(_test: *mut test_suite, _subtests: c_int) -> c_int {
    pr_debug(
        b"Skip SDT event test because SDT support is not compiled\n\0".as_ptr() as *const c_char,
    );
    TEST_SKIP
}

DEFINE_SUITE!("Probe SDT events", sdt_event);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
