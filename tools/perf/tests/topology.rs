// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/topology.c.
// Original C dependencies included:
// <string.h>, <stdlib.h>, <stdio.h>, <perf/cpumap.h>, "cpumap.h",
// "tests.h", "session.h", "evlist.h", "debug.h", "pmus.h",
// "target.h", <linux/err.h>, "dwarf-regs.h".

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

const TEMPL: &[u8] = b"/tmp/perf-test-XXXXXX\0";
const DATA_SIZE: u64 = 10;

const PERF_DATA_MODE_WRITE: c_int = 1;
const PERF_DATA_MODE_READ: c_int = 0;

const HEADER_CPU_TOPOLOGY: c_int = 0;
const HEADER_NRCPUS: c_int = 1;
const HEADER_ARCH: c_int = 2;

const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = -2;

const EM_S390: u16 = 22;
const EM_AARCH64: u16 = 183;
const EM_PPC64: u16 = 21;

const PATH_MAX: usize = 4096;

#[repr(C)]
pub struct perf_data {
    pub path: *mut c_char,
    pub mode: c_int,
}

#[repr(C)]
pub struct perf_header {
    pub data_size: u64,
}

#[repr(C)]
pub struct perf_session {
    pub header: perf_header,
    pub evlist: *mut evlist,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct target {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct aggr_cpu_id {
    pub socket: c_int,
    pub die: c_int,
    pub core: c_int,
    pub cpu: perf_cpu,
    pub node: c_int,
    pub thread_idx: c_int,
}

#[repr(C)]
pub struct cpu_topology {
    pub core_id: c_int,
    pub socket_id: c_int,
    pub die_id: c_int,
}

#[repr(C)]
pub struct perf_env {
    pub cpu: *mut cpu_topology,
    pub nr_cpus_avail: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

extern "C" {
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn perror(s: *const c_char);
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;

    fn perf_session__new(data: *mut perf_data, repipe: *mut c_void) -> *mut perf_session;
    fn perf_session__delete(session: *mut perf_session);
    fn perf_session__write_header(
        session: *mut perf_session,
        evlist: *mut evlist,
        fd: c_int,
        at_exit: bool,
    ) -> c_int;
    fn perf_session__env(session: *mut perf_session) -> *mut perf_env;

    fn perf_data__fd(data: *mut perf_data) -> c_int;

    fn evlist__new_default(target: *mut target, sample_callchains: bool) -> *mut evlist;
    fn evlist__set_session(evlist: *mut evlist, session: *mut perf_session);
    fn evlist__put(evlist: *mut evlist);

    fn perf_header__set_feat(header: *mut perf_header, feat: c_int);

    fn IS_ERR(ptr: *const c_void) -> bool;

    fn cpu__setup_cpunode_map();
    fn cpu__get_socket_id(cpu: perf_cpu) -> c_int;
    fn cpu__get_node(cpu: perf_cpu) -> c_int;

    fn perf_env__e_machine(env: *mut perf_env, pmu: *mut c_void) -> u16;

    fn perf_cpu_map__cpu(map: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__has(map: *mut perf_cpu_map, cpu: perf_cpu) -> bool;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__nr(map: *mut perf_cpu_map) -> c_int;

    fn aggr_cpu_id__cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn aggr_cpu_id__core(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn aggr_cpu_id__die(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn aggr_cpu_id__socket(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;
    fn aggr_cpu_id__node(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id;

    fn pr_debug(fmt: *const c_char, ...);
}

macro_rules! TEST_ASSERT_VAL {
    ($msg:expr, $cond:expr) => {
        if !$cond {
            return TEST_FAIL;
        }
    };
}

unsafe fn perf_cpu_map_for_each_cpu<F>(map: *mut perf_cpu_map, mut f: F) -> c_int
where
    F: FnMut(perf_cpu, c_uint) -> c_int,
{
    let mut i: c_int = 0;
    let nr = perf_cpu_map__nr(map);

    while i < nr {
        let cpu = perf_cpu_map__cpu(map, i);
        let ret = f(cpu, i as c_uint);
        if ret != 0 {
            return ret;
        }
        i += 1;
    }

    0
}

unsafe fn get_temp(path: *mut c_char) -> c_int {
    let mut fd: c_int;

    strcpy(path, TEMPL.as_ptr() as *const c_char);

    fd = mkstemp(path);
    if fd < 0 {
        perror(c"mkstemp failed".as_ptr());
        return -1;
    }

    close(fd);
    0
}

unsafe fn session_write_header(path: *mut c_char) -> c_int {
    let mut data = perf_data {
        path,
        mode: PERF_DATA_MODE_WRITE,
    };
    let mut target: target = std::mem::zeroed();
    let session: *mut perf_session;

    session = perf_session__new(&mut data, ptr::null_mut());
    TEST_ASSERT_VAL!(c"can't get session", !IS_ERR(session as *const c_void));

    (*session).evlist = evlist__new_default(&mut target, false);
    TEST_ASSERT_VAL!(c"can't get evlist", !(*session).evlist.is_null());
    evlist__set_session((*session).evlist, session);

    perf_header__set_feat(&mut (*session).header, HEADER_CPU_TOPOLOGY);
    perf_header__set_feat(&mut (*session).header, HEADER_NRCPUS);
    perf_header__set_feat(&mut (*session).header, HEADER_ARCH);

    (*session).header.data_size += DATA_SIZE;

    TEST_ASSERT_VAL!(
        c"failed to write header",
        perf_session__write_header(session, (*session).evlist, perf_data__fd(&mut data), true) == 0
    );

    evlist__put((*session).evlist);
    perf_session__delete(session);

    0
}

unsafe fn check_cpu_topology(path: *mut c_char, map: *mut perf_cpu_map) -> c_int {
    let mut data = perf_data {
        path,
        mode: PERF_DATA_MODE_READ,
    };
    let mut i: c_uint;
    let mut id: aggr_cpu_id;
    let mut cpu: perf_cpu = std::mem::zeroed();
    let env: *mut perf_env;
    let e_machine: u16;
    let session: *mut perf_session;

    session = perf_session__new(&mut data, ptr::null_mut());
    TEST_ASSERT_VAL!(c"can't get session", !IS_ERR(session as *const c_void));
    env = perf_session__env(session);
    cpu__setup_cpunode_map();

    /* On platforms with large numbers of CPUs process_cpu_topology()
     * might issue an error while reading the perf.data file section
     * HEADER_CPU_TOPOLOGY and the cpu_topology_map pointed to by member
     * cpu is a NULL pointer.
     * Example: On s390
     *   CPU 0 is on core_id 0 and physical_package_id 6
     *   CPU 1 is on core_id 1 and physical_package_id 3
     *
     *   Core_id and physical_package_id are platform and architecture
     *   dependent and might have higher numbers than the CPU id.
     *   This actually depends on the configuration.
     *
     *  In this case process_cpu_topology() prints error message:
     *  "socket_id number is too big. You may need to upgrade the
     *  perf tool."
     *
     *  This is the reason why this test might be skipped. aarch64 and
     *  s390 always write this part of the header, even when the above
     *  condition is true (see do_core_id_test in header.c). So always
     *  run this test on those platforms.
     */
    e_machine = perf_env__e_machine(env, ptr::null_mut());

    if (*env).cpu.is_null() && e_machine != EM_S390 && e_machine != EM_AARCH64 {
        return TEST_SKIP;
    }

    /*
     * In powerpc pSeries platform, not all the topology information
     * are exposed via sysfs. Due to restriction, detail like
     * physical_package_id will be set to -1. Hence skip this
     * test if physical_package_id returns -1 for cpu from perf_cpu_map.
     */
    if e_machine == EM_PPC64 {
        if cpu__get_socket_id(perf_cpu_map__cpu(map, 0)) == -1 {
            return TEST_SKIP;
        }
    }

    TEST_ASSERT_VAL!(c"Session header CPU map not set", !(*env).cpu.is_null());

    i = 0;
    while i < (*env).nr_cpus_avail as c_uint {
        cpu.cpu = i as c_int;
        if !perf_cpu_map__has(map, cpu) {
            i += 1;
            continue;
        }
        pr_debug(
            c"CPU %d, core %d, socket %d\n".as_ptr(),
            i,
            (*(*env).cpu.add(i as usize)).core_id,
            (*(*env).cpu.add(i as usize)).socket_id,
        );
        i += 1;
    }

    // Test that CPU ID contains socket, die, core and CPU
    let mut ret = perf_cpu_map_for_each_cpu(map, |cpu, _i| {
        id = aggr_cpu_id__cpu(cpu, ptr::null_mut());
        TEST_ASSERT_VAL!(c"Cpu map - CPU ID doesn't match", cpu.cpu == id.cpu.cpu);

        TEST_ASSERT_VAL!(
            c"Cpu map - Core ID doesn't match",
            (*(*env).cpu.add(cpu.cpu as usize)).core_id == id.core
        );
        TEST_ASSERT_VAL!(
            c"Cpu map - Socket ID doesn't match",
            (*(*env).cpu.add(cpu.cpu as usize)).socket_id == id.socket
        );

        TEST_ASSERT_VAL!(
            c"Cpu map - Die ID doesn't match",
            (*(*env).cpu.add(cpu.cpu as usize)).die_id == id.die
        );
        TEST_ASSERT_VAL!(c"Cpu map - Node ID is set", id.node == -1);
        TEST_ASSERT_VAL!(c"Cpu map - Thread IDX is set", id.thread_idx == -1);
        0
    });
    if ret != 0 {
        return ret;
    }

    // Test that core ID contains socket, die and core
    ret = perf_cpu_map_for_each_cpu(map, |cpu, _i| {
        id = aggr_cpu_id__core(cpu, ptr::null_mut());
        TEST_ASSERT_VAL!(
            c"Core map - Core ID doesn't match",
            (*(*env).cpu.add(cpu.cpu as usize)).core_id == id.core
        );

        TEST_ASSERT_VAL!(
            c"Core map - Socket ID doesn't match",
            (*(*env).cpu.add(cpu.cpu as usize)).socket_id == id.socket
        );

        TEST_ASSERT_VAL!(
            c"Core map - Die ID doesn't match",
            (*(*env).cpu.add(cpu.cpu as usize)).die_id == id.die
        );
        TEST_ASSERT_VAL!(c"Core map - Node ID is set", id.node == -1);
        TEST_ASSERT_VAL!(c"Core map - Thread IDX is set", id.thread_idx == -1);
        0
    });
    if ret != 0 {
        return ret;
    }

    // Test that die ID contains socket and die
    ret = perf_cpu_map_for_each_cpu(map, |cpu, _i| {
        id = aggr_cpu_id__die(cpu, ptr::null_mut());
        TEST_ASSERT_VAL!(
            c"Die map - Socket ID doesn't match",
            (*(*env).cpu.add(cpu.cpu as usize)).socket_id == id.socket
        );

        TEST_ASSERT_VAL!(
            c"Die map - Die ID doesn't match",
            (*(*env).cpu.add(cpu.cpu as usize)).die_id == id.die
        );

        TEST_ASSERT_VAL!(c"Die map - Node ID is set", id.node == -1);
        TEST_ASSERT_VAL!(c"Die map - Core is set", id.core == -1);
        TEST_ASSERT_VAL!(c"Die map - CPU is set", id.cpu.cpu == -1);
        TEST_ASSERT_VAL!(c"Die map - Thread IDX is set", id.thread_idx == -1);
        0
    });
    if ret != 0 {
        return ret;
    }

    // Test that socket ID contains only socket
    ret = perf_cpu_map_for_each_cpu(map, |cpu, _i| {
        id = aggr_cpu_id__socket(cpu, ptr::null_mut());
        TEST_ASSERT_VAL!(
            c"Socket map - Socket ID doesn't match",
            (*(*env).cpu.add(cpu.cpu as usize)).socket_id == id.socket
        );

        TEST_ASSERT_VAL!(c"Socket map - Node ID is set", id.node == -1);
        TEST_ASSERT_VAL!(c"Socket map - Die ID is set", id.die == -1);
        TEST_ASSERT_VAL!(c"Socket map - Core is set", id.core == -1);
        TEST_ASSERT_VAL!(c"Socket map - CPU is set", id.cpu.cpu == -1);
        TEST_ASSERT_VAL!(c"Socket map - Thread IDX is set", id.thread_idx == -1);
        0
    });
    if ret != 0 {
        return ret;
    }

    // Test that node ID contains only node
    ret = perf_cpu_map_for_each_cpu(map, |cpu, _i| {
        id = aggr_cpu_id__node(cpu, ptr::null_mut());
        TEST_ASSERT_VAL!(c"Node map - Node ID doesn't match", cpu__get_node(cpu) == id.node);
        TEST_ASSERT_VAL!(c"Node map - Socket is set", id.socket == -1);
        TEST_ASSERT_VAL!(c"Node map - Die ID is set", id.die == -1);
        TEST_ASSERT_VAL!(c"Node map - Core is set", id.core == -1);
        TEST_ASSERT_VAL!(c"Node map - CPU is set", id.cpu.cpu == -1);
        TEST_ASSERT_VAL!(c"Node map - Thread IDX is set", id.thread_idx == -1);
        0
    });
    if ret != 0 {
        return ret;
    }

    perf_session__delete(session);

    0
}

unsafe fn test__session_topology(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut map: *mut perf_cpu_map;
    let mut ret: c_int = TEST_FAIL;

    TEST_ASSERT_VAL!(c"can't get templ file", get_temp(path.as_mut_ptr()) == 0);

    pr_debug(c"templ file: %s\n".as_ptr(), path.as_mut_ptr());

    if session_write_header(path.as_mut_ptr()) != 0 {
        goto_free_path(path.as_mut_ptr(), ret);
        return ret;
    }

    map = perf_cpu_map__new_online_cpus();
    if map.is_null() {
        pr_debug(c"failed to get system cpumap\n".as_ptr());
        goto_free_path(path.as_mut_ptr(), ret);
        return ret;
    }

    ret = check_cpu_topology(path.as_mut_ptr(), map);
    perf_cpu_map__put(map);

    goto_free_path(path.as_mut_ptr(), ret)
}

unsafe fn goto_free_path(path: *mut c_char, ret: c_int) -> c_int {
    unlink(path);
    ret
}

// DEFINE_SUITE("Session topology", session_topology);
#[no_mangle]
pub static mut session_topology: test_suite = test_suite { _private: [] };

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
