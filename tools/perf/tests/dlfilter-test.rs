// SPDX-License-Identifier: GPL-2.0
/*
 * Test dlfilter C API. A perf.data file is synthesized and then processed
 * by perf script with dlfilters named dlfilter-test-api-v*.so. Also a C file
 * is compiled to provide a dso to match the synthesized perf.data file.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u64 = u64;
type pid_t = c_int;
type ssize_t = isize;
type size_t = usize;

const MAP_START: u64 = 0x400000;
const DLFILTER_TEST_NAME_MAX: usize = 128;
const MAXCMD: usize = 4096;
const REDIRECT_TO_DEV_NULL: &[u8] = b" >/dev/null 2>&1\0";
const PATH_MAX: usize = 4096;
const PERF_SAMPLE_MAX_SIZE: usize = 65536;

const R_OK: c_int = 4;
const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: u64 = 4;
const PERF_RECORD_MMAP: u32 = 1;
const PERF_RECORD_COMM: u32 = 3;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_RECORD_MISC_USER: u16 = 1 << 13;

const PERF_SAMPLE_IP: u64 = 1 << 0;
const PERF_SAMPLE_TID: u64 = 1 << 1;
const PERF_SAMPLE_TIME: u64 = 1 << 2;
const PERF_SAMPLE_ADDR: u64 = 1 << 3;
const PERF_SAMPLE_ID: u64 = 1 << 6;
const PERF_SAMPLE_CPU: u64 = 1 << 7;
const PERF_SAMPLE_PERIOD: u64 = 1 << 8;
const PERF_SAMPLE_STREAM_ID: u64 = 1 << 9;
const PERF_SAMPLE_IDENTIFIER: u64 = 1 << 16;

const TEST_SAMPLE_TYPE: u64 = PERF_SAMPLE_IP
    | PERF_SAMPLE_TID
    | PERF_SAMPLE_IDENTIFIER
    | PERF_SAMPLE_TIME
    | PERF_SAMPLE_ADDR
    | PERF_SAMPLE_CPU
    | PERF_SAMPLE_PERIOD
    | PERF_SAMPLE_STREAM_ID;

const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = -2;

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub start: u64,
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
    _bindgen_union_align: [u64; PERF_SAMPLE_MAX_SIZE / size_of::<u64>()],
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
}

#[repr(C)]
pub struct perf_sample {
    pub ip: u64,
    pub pid: u32,
    pub tid: u32,
    pub time: u64,
    pub addr: u64,
    pub id: u64,
    pub stream_id: u64,
    pub period: u64,
    pub cpu: u32,
}

#[repr(C)]
pub struct perf_record_comm {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct perf_record_mmap {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub start: u64,
    pub len: u64,
    pub pgoff: u64,
    pub filename: [c_char; PATH_MAX],
}

#[repr(C)]
pub struct test_data {
    pub tool: perf_tool,
    pub machine: *mut machine,
    pub fd: c_int,
    pub foo: u64,
    pub bar: u64,
    pub ip: u64,
    pub addr: u64,
    pub name: [c_char; DLFILTER_TEST_NAME_MAX],
    pub desc: [c_char; DLFILTER_TEST_NAME_MAX],
    pub perf: [c_char; PATH_MAX],
    pub perf_data_file_name: [c_char; PATH_MAX],
    pub c_file_name: [c_char; PATH_MAX],
    pub prog_file_name: [c_char; PATH_MAX],
    pub dlfilters: [c_char; PATH_MAX],
}

unsafe extern "C" {
    static verbose: c_int;

    fn pr_debug(fmt: *const c_char, ...);
    fn writen(fd: c_int, buf: *const c_void, n: size_t) -> ssize_t;
    fn vsnprintf(s: *mut c_char, n: size_t, format: *const c_char, arg: VaList) -> c_int;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn system(command: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn creat(pathname: *const c_char, mode: c_uint) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn dirname(path: *mut c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn get_argv_exec_path() -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn getpid() -> pid_t;

    fn perf_exe(buf: *mut c_char, len: size_t);
    fn perf_event__synthesize_attr(
        tool: *const perf_tool,
        attr: *mut perf_event_attr,
        ids: size_t,
        id: *mut u64,
        process: unsafe extern "C" fn(
            *const perf_tool,
            *mut perf_event,
            *mut perf_sample,
            *mut machine,
        ) -> c_int,
    ) -> c_int;
    fn perf_event__sample_event_size(
        sample: *const perf_sample,
        sample_type: u64,
        read_format: u64,
        branch_sample_type: u64,
    ) -> u16;
    fn perf_event__synthesize_sample(
        event: *mut perf_event,
        sample_type: u64,
        read_format: u64,
        branch_sample_type: u64,
        sample: *const perf_sample,
    ) -> c_int;
    fn perf_header__write_pipe(fd: c_int) -> c_int;
    fn get_filter_desc(
        dlfilters: *const c_char,
        name: *const c_char,
        desc: *mut *mut c_char,
        long_desc: *mut *mut c_char,
    ) -> c_int;
    fn dso__new_map(filename: *const c_char) -> *mut map;
    fn map__find_symbol_by_name(map: *mut map, name: *const c_char) -> *mut symbol;
    fn map__put(map: *mut map);
    fn perf_env__init(env: *mut perf_env);
    fn perf_env__exit(env: *mut perf_env);
    fn machine__new_host(env: *mut perf_env) -> *mut machine;
    fn machine__delete(machine: *mut machine);
}

type VaList = *mut c_void;

unsafe fn roundup(x: size_t, y: size_t) -> size_t {
    ((x + y - 1) / y) * y
}

unsafe extern "C" fn test_result(msg: *const c_char, ret: c_int) -> c_int {
    unsafe {
        pr_debug(c"%s\n".as_ptr(), msg);
    }
    ret
}

unsafe extern "C" fn process(
    tool: *const perf_tool,
    event: *mut perf_event,
    _sample: *mut perf_sample,
    _machine: *mut machine,
) -> c_int {
    let td = (tool as *const u8).sub(offset_of!(test_data, tool)) as *mut test_data;
    let fd = unsafe { (*td).fd };

    if unsafe { writen(fd, event as *const c_void, (*event).header.size as size_t) }
        != unsafe { (*event).header.size as ssize_t }
    {
        return -1;
    }

    0
}

// Rust has no stable C variadic function definitions. This preserves the C API
// intent; callers below pass concrete format strings and arguments as in C.
unsafe extern "C" fn system_cmd(fmt: *const c_char, mut args: ...) -> c_int {
    let mut cmd: [c_char; MAXCMD + REDIRECT_TO_DEV_NULL.len()] =
        [0; MAXCMD + REDIRECT_TO_DEV_NULL.len()];
    let mut ret: c_int;

    ret = unsafe { vsnprintf(cmd.as_mut_ptr(), MAXCMD, fmt, args.as_va_list()) };

    if ret <= 0 || ret >= MAXCMD as c_int {
        return -1;
    }

    if unsafe { verbose } <= 0 {
        unsafe {
            strcat(cmd.as_mut_ptr(), REDIRECT_TO_DEV_NULL.as_ptr() as *const c_char);
        }
    }

    unsafe {
        pr_debug(c"Command: %s\n".as_ptr(), cmd.as_ptr());
        ret = system(cmd.as_ptr());
        if ret != 0 {
            pr_debug(c"Failed with return value %d\n".as_ptr(), ret);
        }
    }

    ret
}

unsafe extern "C" fn have_gcc() -> bool {
    unsafe {
        pr_debug(c"Checking for gcc\n".as_ptr());
        system_cmd(c"gcc --version".as_ptr()) == 0
    }
}

unsafe extern "C" fn write_attr(td: *mut test_data, sample_type: u64, id: *mut u64) -> c_int {
    let mut attr = perf_event_attr {
        size: size_of::<perf_event_attr>() as u32,
        type_: PERF_TYPE_HARDWARE,
        config: PERF_COUNT_HW_BRANCH_INSTRUCTIONS,
        sample_type,
        sample_period: 1,
    };

    unsafe { perf_event__synthesize_attr(&(*td).tool, &mut attr, 1, id, process) }
}

unsafe extern "C" fn write_comm(
    fd: c_int,
    pid: pid_t,
    tid: pid_t,
    comm_str: *const c_char,
) -> c_int {
    let mut comm = perf_record_comm {
        header: perf_event_header {
            type_: 0,
            misc: 0,
            size: 0,
        },
        pid: 0,
        tid: 0,
        comm: [0; 16],
    };
    let sz = size_of::<perf_record_comm>() as ssize_t;

    comm.header.type_ = PERF_RECORD_COMM;
    comm.header.misc = PERF_RECORD_MISC_USER;
    comm.header.size = sz as u16;

    comm.pid = pid as u32;
    comm.tid = tid as u32;
    unsafe {
        strncpy(comm.comm.as_mut_ptr(), comm_str, 16);
    }

    if unsafe { writen(fd, &comm as *const _ as *const c_void, sz as size_t) } != sz {
        unsafe {
            pr_debug(c"%s failed\n".as_ptr(), c"write_comm".as_ptr());
        }
        return -1;
    }

    0
}

unsafe extern "C" fn write_mmap(
    fd: c_int,
    pid: pid_t,
    tid: pid_t,
    start: u64,
    len: u64,
    pgoff: u64,
    filename: *const c_char,
) -> c_int {
    let mut buf: [c_char; PERF_SAMPLE_MAX_SIZE] = [0; PERF_SAMPLE_MAX_SIZE];
    let mmap = buf.as_mut_ptr() as *mut perf_record_mmap;
    let fsz = unsafe { roundup(strlen(filename) + 1, 8) };
    let sz = (size_of::<perf_record_mmap>() - size_of::<[c_char; PATH_MAX]>() + fsz) as ssize_t;

    unsafe {
        (*mmap).header.type_ = PERF_RECORD_MMAP;
        (*mmap).header.misc = PERF_RECORD_MISC_USER;
        (*mmap).header.size = sz as u16;

        (*mmap).pid = pid as u32;
        (*mmap).tid = tid as u32;
        (*mmap).start = start;
        (*mmap).len = len;
        (*mmap).pgoff = pgoff;
        strncpy((*mmap).filename.as_mut_ptr(), filename, size_of::<[c_char; PATH_MAX]>());

        if writen(fd, mmap as *const c_void, sz as size_t) != sz {
            pr_debug(c"%s failed\n".as_ptr(), c"write_mmap".as_ptr());
            return -1;
        }
    }

    0
}

unsafe extern "C" fn write_sample(
    td: *mut test_data,
    sample_type: u64,
    id: u64,
    pid: pid_t,
    tid: pid_t,
) -> c_int {
    let mut buf: [c_char; PERF_SAMPLE_MAX_SIZE] = [0; PERF_SAMPLE_MAX_SIZE];
    let event = buf.as_mut_ptr() as *mut perf_event;
    let mut sample = unsafe {
        perf_sample {
            ip: (*td).ip,
            addr: (*td).addr,
            id,
            time: 1234567890,
            cpu: 31,
            pid: pid as u32,
            tid: tid as u32,
            period: 543212345,
            stream_id: 101,
        }
    };
    let mut err: c_int;

    unsafe {
        (*event).header.type_ = PERF_RECORD_SAMPLE;
        (*event).header.misc = PERF_RECORD_MISC_USER;
        (*event).header.size = perf_event__sample_event_size(&sample, sample_type, 0, 0);
        err = perf_event__synthesize_sample(event, sample_type, 0, 0, &sample);
    }
    if err != 0 {
        return unsafe {
            test_result(
                c"perf_event__synthesize_sample() failed".as_ptr(),
                TEST_FAIL,
            )
        };
    }

    err = unsafe { process(&(*td).tool, event, &mut sample, (*td).machine) };
    if err != 0 {
        return unsafe { test_result(c"Failed to write sample".as_ptr(), TEST_FAIL) };
    }

    TEST_OK
}

unsafe extern "C" fn close_fd(fd: c_int) {
    if fd >= 0 {
        unsafe {
            close(fd);
        }
    }
}

static prog: &[u8] = b"int bar(){};int foo(){bar();};int main(){foo();return 0;}\0";

unsafe extern "C" fn write_prog(file_name: *mut c_char) -> c_int {
    let fd = unsafe { creat(file_name, 0o644) };
    let n = unsafe { strlen(prog.as_ptr() as *const c_char) };
    let err = fd < 0
        || unsafe { writen(fd, prog.as_ptr() as *const c_void, n) } != n as ssize_t;

    unsafe {
        close_fd(fd);
    }
    if err {
        -1
    } else {
        0
    }
}

unsafe extern "C" fn get_dlfilters_path(
    name: *const c_char,
    buf: *mut c_char,
    sz: size_t,
) -> c_int {
    let mut perf: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let perf_path: *mut c_char;
    let exec_path: *mut c_char;

    unsafe {
        perf_exe(perf.as_mut_ptr(), size_of::<[c_char; PATH_MAX]>());
        perf_path = dirname(perf.as_mut_ptr());
        snprintf(
            path.as_mut_ptr(),
            size_of::<[c_char; PATH_MAX]>(),
            c"%s/dlfilters/%s".as_ptr(),
            perf_path,
            name,
        );
        if access(path.as_ptr(), R_OK) != 0 {
            exec_path = get_argv_exec_path();
            if exec_path.is_null() {
                return -1;
            }
            snprintf(
                path.as_mut_ptr(),
                size_of::<[c_char; PATH_MAX]>(),
                c"%s/dlfilters/%s".as_ptr(),
                exec_path,
                name,
            );
            free(exec_path as *mut c_void);
            if access(path.as_ptr(), R_OK) != 0 {
                return -1;
            }
        }
        strlcpy(buf, dirname(path.as_mut_ptr()), sz);
    }
    0
}

unsafe extern "C" {
    fn strlcpy(dest: *mut c_char, src: *const c_char, size: size_t) -> size_t;
}

unsafe extern "C" fn check_filter_desc(td: *mut test_data) -> c_int {
    let mut long_desc: *mut c_char = ptr::null_mut();
    let mut desc: *mut c_char = ptr::null_mut();
    let ret: c_int;

    unsafe {
        if get_filter_desc(
            (*td).dlfilters.as_ptr(),
            (*td).name.as_ptr(),
            &mut desc,
            &mut long_desc,
        ) != 0
            && !long_desc.is_null()
            && strcmp(
                long_desc,
                c"Filter used by the 'dlfilter C API' perf test".as_ptr(),
            ) == 0
            && !desc.is_null()
            && strcmp(desc, (*td).desc.as_ptr()) == 0
        {
            ret = 0;
        } else {
            ret = -1;
        }

        free(desc as *mut c_void);
        free(long_desc as *mut c_void);
    }
    ret
}

unsafe extern "C" fn get_ip_addr(td: *mut test_data) -> c_int {
    let map: *mut map;
    let mut sym: *mut symbol;

    unsafe {
        map = dso__new_map((*td).prog_file_name.as_ptr());
        if map.is_null() {
            return -1;
        }

        sym = map__find_symbol_by_name(map, c"foo".as_ptr());
        if !sym.is_null() {
            (*td).foo = (*sym).start;
        }

        sym = map__find_symbol_by_name(map, c"bar".as_ptr());
        if !sym.is_null() {
            (*td).bar = (*sym).start;
        }

        map__put(map);

        (*td).ip = MAP_START + (*td).foo;
        (*td).addr = MAP_START + (*td).bar;

        if (*td).foo != 0 && (*td).bar != 0 {
            0
        } else {
            -1
        }
    }
}

unsafe extern "C" fn do_run_perf_script(td: *mut test_data, do_early: c_int) -> c_int {
    unsafe {
        system_cmd(
            c"%s script -i %s --dlfilter %s/%s --dlarg first --dlarg %d --dlarg %lu --dlarg %lu --dlarg %d --dlarg last".as_ptr(),
            (*td).perf.as_ptr(),
            (*td).perf_data_file_name.as_ptr(),
            (*td).dlfilters.as_ptr(),
            (*td).name.as_ptr(),
            verbose,
            (*td).ip,
            (*td).addr,
            do_early,
        )
    }
}

unsafe extern "C" fn run_perf_script(td: *mut test_data) -> c_int {
    let mut do_early: c_int;
    let mut err: c_int;

    do_early = 0;
    while do_early < 3 {
        err = unsafe { do_run_perf_script(td, do_early) };
        if err != 0 {
            return err;
        }
        do_early += 1;
    }
    0
}

unsafe extern "C" fn test__dlfilter_test(td: *mut test_data) -> c_int {
    let mut host_env = perf_env { _private: [] };
    let sample_type: u64 = TEST_SAMPLE_TYPE;
    let pid: pid_t = 12345;
    let tid: pid_t = 12346;
    let mut id: u64 = 99;
    let mut err: c_int = TEST_OK;

    unsafe {
        if get_dlfilters_path((*td).name.as_ptr(), (*td).dlfilters.as_mut_ptr(), PATH_MAX) != 0 {
            return test_result(c"dlfilters not found".as_ptr(), TEST_SKIP);
        }

        if check_filter_desc(td) != 0 {
            return test_result(c"Failed to get expected filter description".as_ptr(), TEST_FAIL);
        }

        if !have_gcc() {
            return test_result(c"gcc not found".as_ptr(), TEST_SKIP);
        }

        pr_debug(c"dlfilters path: %s\n".as_ptr(), (*td).dlfilters.as_ptr());

        if write_prog((*td).c_file_name.as_mut_ptr()) != 0 {
            return test_result(c"Failed to write test C file".as_ptr(), TEST_FAIL);
        }

        if verbose > 1 {
            system_cmd(c"cat %s ; echo".as_ptr(), (*td).c_file_name.as_ptr());
        }

        if system_cmd(
            c"gcc -g -o %s %s".as_ptr(),
            (*td).prog_file_name.as_ptr(),
            (*td).c_file_name.as_ptr(),
        ) != 0
        {
            return TEST_FAIL;
        }

        if verbose > 2 {
            system_cmd(c"objdump -x -dS %s".as_ptr(), (*td).prog_file_name.as_ptr());
        }

        if get_ip_addr(td) != 0 {
            return test_result(c"Failed to find program symbols".as_ptr(), TEST_FAIL);
        }

        pr_debug(c"Creating new host machine structure\n".as_ptr());
        perf_env__init(&mut host_env);
        (*td).machine = machine__new_host(&mut host_env);

        (*td).fd = creat((*td).perf_data_file_name.as_ptr(), 0o644);
        if (*td).fd < 0 {
            return test_result(c"Failed to create test perf.data file".as_ptr(), TEST_FAIL);
        }

        err = perf_header__write_pipe((*td).fd);
        if err < 0 {
            err = test_result(c"perf_header__write_pipe() failed".as_ptr(), TEST_FAIL);
            perf_env__exit(&mut host_env);
            return err;
        }
        err = write_attr(td, sample_type, &mut id);
        if err != 0 {
            err = test_result(c"perf_event__synthesize_attr() failed".as_ptr(), TEST_FAIL);
            perf_env__exit(&mut host_env);
            return err;
        }
        if write_comm((*td).fd, pid, tid, c"test-prog".as_ptr()) != 0 {
            err = TEST_FAIL;
            perf_env__exit(&mut host_env);
            return err;
        }
        if write_mmap((*td).fd, pid, tid, MAP_START, 0x10000, 0, (*td).prog_file_name.as_ptr())
            != 0
        {
            err = TEST_FAIL;
            perf_env__exit(&mut host_env);
            return err;
        }
        if write_sample(td, sample_type, id, pid, tid) != TEST_OK {
            err = TEST_FAIL;
            perf_env__exit(&mut host_env);
            return err;
        }
        if verbose > 1 {
            system_cmd(
                c"%s script -i %s -D".as_ptr(),
                (*td).perf.as_ptr(),
                (*td).perf_data_file_name.as_ptr(),
            );
        }

        err = if run_perf_script(td) != 0 {
            TEST_FAIL
        } else {
            TEST_OK
        };
        perf_env__exit(&mut host_env);
    }
    err
}

unsafe extern "C" fn unlink_path(path: *const c_char) {
    unsafe {
        if *path != 0 {
            unlink(path);
        }
    }
}

unsafe extern "C" fn test_data__free(td: *mut test_data) {
    unsafe {
        machine__delete((*td).machine);
        close_fd((*td).fd);
        if verbose <= 2 {
            unlink_path((*td).c_file_name.as_ptr());
            unlink_path((*td).prog_file_name.as_ptr());
            unlink_path((*td).perf_data_file_name.as_ptr());
        }
    }
}

unsafe extern "C" fn test__dlfilter_ver(ver: c_int) -> c_int {
    let mut td = test_data {
        tool: perf_tool { _private: [] },
        machine: ptr::null_mut(),
        fd: -1,
        foo: 0,
        bar: 0,
        ip: 0,
        addr: 0,
        name: [0; DLFILTER_TEST_NAME_MAX],
        desc: [0; DLFILTER_TEST_NAME_MAX],
        perf: [0; PATH_MAX],
        perf_data_file_name: [0; PATH_MAX],
        c_file_name: [0; PATH_MAX],
        prog_file_name: [0; PATH_MAX],
        dlfilters: [0; PATH_MAX],
    };
    let pid = unsafe { getpid() };
    let err: c_int;

    unsafe {
        pr_debug(c"\n-- Testing version %d API --\n".as_ptr(), ver);

        perf_exe(td.perf.as_mut_ptr(), size_of::<[c_char; PATH_MAX]>());

        snprintf(
            td.name.as_mut_ptr(),
            size_of::<[c_char; DLFILTER_TEST_NAME_MAX]>(),
            c"dlfilter-test-api-v%d.so".as_ptr(),
            ver,
        );
        snprintf(
            td.desc.as_mut_ptr(),
            size_of::<[c_char; DLFILTER_TEST_NAME_MAX]>(),
            c"dlfilter to test v%d C API".as_ptr(),
            ver,
        );
        snprintf(
            td.perf_data_file_name.as_mut_ptr(),
            PATH_MAX,
            c"/tmp/dlfilter-test-%u-perf-data".as_ptr(),
            pid,
        );
        snprintf(
            td.c_file_name.as_mut_ptr(),
            PATH_MAX,
            c"/tmp/dlfilter-test-%u-prog.c".as_ptr(),
            pid,
        );
        snprintf(
            td.prog_file_name.as_mut_ptr(),
            PATH_MAX,
            c"/tmp/dlfilter-test-%u-prog".as_ptr(),
            pid,
        );

        err = test__dlfilter_test(&mut td);
        test_data__free(&mut td);
    }
    err
}

unsafe extern "C" fn test__dlfilter(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let err = unsafe { test__dlfilter_ver(0) };

    if err != 0 {
        return err;
    }
    /* No test for version 1 */
    unsafe { test__dlfilter_ver(2) }
}

// DEFINE_SUITE("dlfilter C API", dlfilter);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
