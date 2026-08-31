// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/symbols.c. C include dependencies are represented
// by the external declarations below.

use core::ffi::{c_char, c_int, c_uint, c_void};

type SizeT = usize;

const TEST_FAIL: c_int = -1;
const TEST_OK: c_int = 0;
const TEST_SKIP: c_int = 2;
const PATH_MAX: usize = 4096;
const PROT_EXEC: c_int = 0x4;
const DSO_SPACE__USER: c_int = 0;
const STT_FUNC: c_uint = 2;
const STT_GNU_IFUNC: c_uint = 10;

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub rb_node: rb_node,
    pub start: u64,
    pub end: u64,
}

#[repr(C)]
struct test_info {
    host_env: perf_env,
    machine: *mut machine,
    thread: *mut thread,
}

#[repr(C)]
struct dso_map {
    dso: *mut dso,
    map: *mut map,
}

unsafe extern "C" {
    static mut dso_to_test: *const c_char;
    static mut verbose: c_int;
    static mut stderr: *mut FILE;
    static dso_id_empty: dso_id;

    fn perf_env__init(env: *mut perf_env);
    fn perf_env__exit(env: *mut perf_env);
    fn machine__new_host(env: *mut perf_env) -> *mut machine;
    fn machine__delete(machine: *mut machine);
    fn machine__findnew_thread(machine: *mut machine, pid: c_int, tid: c_int) -> *mut thread;
    fn thread__put(thread: *mut thread);
    fn machine__findnew_dso(machine: *mut machine, filename: *mut c_char) -> *mut dso;
    fn machine__for_each_kernel_map(
        machine: *mut machine,
        cb: unsafe extern "C" fn(*mut map, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn machine__for_each_dso(
        machine: *mut machine,
        cb: unsafe extern "C" fn(*mut dso, *mut machine, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__get(map: *mut map) -> *mut map;
    fn map__put(map: *mut map);
    fn map__new(
        machine: *mut machine,
        start: u64,
        len: u64,
        pgoff: u64,
        id: *const dso_id,
        prot: c_int,
        flags: c_uint,
        filename: *mut c_char,
        thread: *mut thread,
    ) -> *mut map;
    fn dso__put(dso: *mut dso);
    fn dso__kernel(dso: *mut dso) -> c_int;
    fn dso__load(dso: *mut dso, map: *mut map) -> c_int;
    fn dso__symbols(dso: *mut dso) -> *mut rb_root_cached;
    fn dso__fprintf(dso: *mut dso, fp: *mut FILE) -> c_int;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn symbol__type(sym: *mut symbol) -> c_uint;
    fn symbol__fprintf(sym: *mut symbol, fp: *mut FILE) -> c_int;
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn rb_entry_symbol(node: *mut rb_node) -> *mut symbol;
    fn strlcpy(dst: *mut c_char, src: *const c_char, size: SizeT) -> SizeT;
    fn perf_exe(filename: *mut c_char, max_sz: SizeT);
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn init_test_info(ti: *mut test_info) -> c_int {
    unsafe {
        perf_env__init(&mut (*ti).host_env);
        (*ti).machine = machine__new_host(&mut (*ti).host_env);
        if (*ti).machine.is_null() {
            pr_debug(c"machine__new_host() failed!\n".as_ptr());
            perf_env__exit(&mut (*ti).host_env);
            return TEST_FAIL;
        }

        /* Create a dummy thread */
        (*ti).thread = machine__findnew_thread((*ti).machine, 100, 100);
        if (*ti).thread.is_null() {
            pr_debug(c"machine__findnew_thread() failed!\n".as_ptr());
            perf_env__exit(&mut (*ti).host_env);
            return TEST_FAIL;
        }

        TEST_OK
    }
}

unsafe fn exit_test_info(ti: *mut test_info) {
    unsafe {
        thread__put((*ti).thread);
        machine__delete((*ti).machine);
        perf_env__exit(&mut (*ti).host_env);
    }
}

unsafe extern "C" fn find_map_cb(map: *mut map, d: *mut c_void) -> c_int {
    unsafe {
        let data = d as *mut dso_map;

        if map__dso(map) != (*data).dso {
            return 0;
        }
        (*data).map = map;
        1
    }
}

unsafe fn find_module_map(machine: *mut machine, dso: *mut dso) -> *mut map {
    unsafe {
        let mut data = dso_map {
            dso,
            map: core::ptr::null_mut(),
        };

        machine__for_each_kernel_map(machine, find_map_cb, &mut data as *mut _ as *mut c_void);

        data.map
    }
}

unsafe fn get_test_dso_filename(filename: *mut c_char, max_sz: SizeT) {
    unsafe {
        if !dso_to_test.is_null() {
            strlcpy(filename, dso_to_test, max_sz);
        } else {
            perf_exe(filename, max_sz);
        }
    }
}

unsafe fn create_map(ti: *mut test_info, filename: *mut c_char, map_p: *mut *mut map) -> c_int {
    unsafe {
        let dso = machine__findnew_dso((*ti).machine, filename);

        /*
         * If 'filename' matches a current kernel module, must use a kernel
         * map. Find the one that already exists.
         */
        if !dso.is_null() && dso__kernel(dso) != DSO_SPACE__USER {
            *map_p = find_module_map((*ti).machine, dso);
            dso__put(dso);
            if (*map_p).is_null() {
                pr_debug(
                    c"Failed to find map for current kernel module %s".as_ptr(),
                    filename,
                );
                return TEST_FAIL;
            }
            map__get(*map_p);
            return TEST_OK;
        }

        dso__put(dso);

        /* Create a dummy map at 0x100000 */
        *map_p = map__new(
            (*ti).machine,
            0x100000,
            0xffffffff,
            0,
            &dso_id_empty,
            PROT_EXEC,
            0,
            filename,
            (*ti).thread,
        );
        if (*map_p).is_null() {
            pr_debug(c"Failed to create map!".as_ptr());
            return TEST_FAIL;
        }

        TEST_OK
    }
}

unsafe fn test_dso(dso: *mut dso) -> c_int {
    unsafe {
        let mut last_sym: *mut symbol = core::ptr::null_mut();
        let mut nd: *mut rb_node;
        let mut ret = TEST_OK;

        /* dso__fprintf() prints all the symbols */
        if verbose > 1 {
            dso__fprintf(dso, stderr);
        }

        nd = rb_first_cached(dso__symbols(dso));
        while !nd.is_null() {
            let sym = rb_entry_symbol(nd);

            if symbol__type(sym) != STT_FUNC && symbol__type(sym) != STT_GNU_IFUNC {
                nd = rb_next(nd);
                continue;
            }

            /* Check for overlapping function symbols */
            if !last_sym.is_null() && (*sym).start < (*last_sym).end {
                pr_debug(c"Overlapping symbols:\n".as_ptr());
                symbol__fprintf(last_sym, stderr);
                symbol__fprintf(sym, stderr);
                ret = TEST_FAIL;
            }
            /* Check for zero-length function symbol */
            if (*sym).start == (*sym).end {
                pr_debug(c"Zero-length symbol:\n".as_ptr());
                symbol__fprintf(sym, stderr);
                ret = TEST_FAIL;
            }
            last_sym = sym;

            nd = rb_next(nd);
        }

        ret
    }
}

unsafe extern "C" fn subdivided_dso_cb(
    dso: *mut dso,
    _machine: *mut machine,
    d: *mut c_void,
) -> c_int {
    unsafe {
        let text_dso = d as *mut dso;

        if dso != text_dso && strstarts(dso__short_name(dso), dso__short_name(text_dso)) {
            if test_dso(dso) != TEST_OK {
                return -1;
            }
        }

        0
    }
}

unsafe fn process_subdivided_dso(machine: *mut machine, dso: *mut dso) -> c_int {
    unsafe {
        let ret: c_int;

        ret = machine__for_each_dso(machine, subdivided_dso_cb, dso as *mut c_void);

        if ret < 0 { TEST_FAIL } else { TEST_OK }
    }
}

unsafe fn test_file(ti: *mut test_info, filename: *mut c_char) -> c_int {
    unsafe {
        let mut map: *mut map = core::ptr::null_mut();
        let mut ret: c_int;
        let nr: c_int;
        let dso: *mut dso;

        pr_debug(c"Testing %s\n".as_ptr(), filename);

        ret = create_map(ti, filename, &mut map);
        if ret != TEST_OK {
            return ret;
        }

        dso = map__dso(map);
        nr = dso__load(dso, map);
        if nr < 0 {
            pr_debug(c"dso__load() failed!\n".as_ptr());
            ret = TEST_FAIL;
            map__put(map);
            return ret;
        }

        if nr == 0 {
            pr_debug(c"DSO has no symbols!\n".as_ptr());
            ret = TEST_SKIP;
            map__put(map);
            return ret;
        }

        ret = test_dso(dso);

        /* Module dso is split into many dsos by section */
        if ret == TEST_OK && dso__kernel(dso) != DSO_SPACE__USER {
            ret = process_subdivided_dso((*ti).machine, dso);
        }

        map__put(map);

        ret
    }
}

unsafe fn test__symbols(_test: *mut test_suite, _subtest: c_int) -> c_int {
    unsafe {
        let mut filename = [0 as c_char; PATH_MAX];
        let mut ti = core::mem::MaybeUninit::<test_info>::uninit();
        let mut ret: c_int;

        ret = init_test_info(ti.as_mut_ptr());
        if ret != TEST_OK {
            return ret;
        }

        get_test_dso_filename(filename.as_mut_ptr(), filename.len());

        ret = test_file(ti.as_mut_ptr(), filename.as_mut_ptr());

        exit_test_info(ti.as_mut_ptr());

        ret
    }
}

// C source used: DEFINE_SUITE("Symbols", symbols);
// Preserve the suite registration intent for the Rust-side test harness.
define_suite!("Symbols", symbols, test__symbols);
