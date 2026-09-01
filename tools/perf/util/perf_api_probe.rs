/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf_api_probe.c. External types, constants, and functions
 * come from the corresponding perf headers and libc dependencies.
 */

use core::ffi::{c_char, c_int, c_ulong};

type pid_t = c_int;
type SetupProbeFnT = unsafe fn(*mut evsel);

extern "C" {
    static mut errno: c_int;

    fn close(fd: c_int) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;

    fn perf_event_open_cloexec_flag() -> c_ulong;
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn parse_event(evlist: *mut evlist, str_: *const c_char) -> c_int;
    fn sys_perf_event_open(
        attr: *mut perf_event_attr,
        pid: pid_t,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__cpu(cpus: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_pmus__scan_core(pmu: *mut perf_pmu) -> *mut perf_pmu;
}

extern "C" {
    static PERF_SAMPLE_IDENTIFIER: u64;
    static PERF_TYPE_SOFTWARE: u32;
    static PERF_COUNT_SW_CPU_CLOCK: u64;
}

const EACCES: c_int = 13;
const EAGAIN: c_int = 11;
const EINVAL: c_int = 22;
const E2BIG: c_int = 7;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_type: u64,
    pub exclude_kernel: u64,
    pub comm_exec: u64,
    pub context_switch: u64,
    pub text_poke: u64,
    pub build_id: u64,
    pub cgroup: u64,
    pub aux_sample_size: u32,
}

unsafe fn perf_do_probe_api(fn_: SetupProbeFnT, cpu: perf_cpu, str_: *const c_char) -> c_int {
    let evlist: *mut evlist;
    let evsel: *mut evsel;
    let flags: c_ulong = perf_event_open_cloexec_flag();
    let mut err: c_int = -EAGAIN;
    let mut fd: c_int;
    static mut PID: pid_t = -1;

    evlist = evlist__new();
    if evlist.is_null() {
        return -ENOMEM;
    }

    if parse_event(evlist, str_) != 0 {
        goto_out_delete(evlist, err)
    } else {
        evsel = evlist__first(evlist);

        loop {
            fd = sys_perf_event_open(&mut (*evsel).core.attr, PID, cpu.cpu, -1, flags);
            if fd < 0 {
                if PID == -1 && errno == EACCES {
                    PID = 0;
                    continue;
                }
                return goto_out_delete(evlist, err);
            }
            break;
        }
        close(fd);

        fn_(evsel);

        fd = sys_perf_event_open(&mut (*evsel).core.attr, PID, cpu.cpu, -1, flags);
        if fd < 0 {
            if errno == EINVAL {
                err = -EINVAL;
            }
            return goto_out_delete(evlist, err);
        }
        close(fd);
        err = 0;

        goto_out_delete(evlist, err)
    }
}

unsafe fn goto_out_delete(evlist: *mut evlist, err: c_int) -> c_int {
    evlist__put(evlist);
    err
}

unsafe fn perf_probe_api(fn_: SetupProbeFnT) -> bool {
    let mut pmu: *mut perf_pmu;
    let cpus: *mut perf_cpu_map;
    let cpu: perf_cpu;
    let mut ret: c_int;

    cpus = perf_cpu_map__new_online_cpus();
    if cpus.is_null() {
        return false;
    }
    cpu = perf_cpu_map__cpu(cpus, 0);
    perf_cpu_map__put(cpus);

    ret = perf_do_probe_api(fn_, cpu, b"software/cpu-clock/u\0".as_ptr() as *const c_char);
    if ret == 0 {
        return true;
    }

    pmu = perf_pmus__scan_core(core::ptr::null_mut());
    if !pmu.is_null() {
        let try_: [*const c_char; 3] = [
            b"cycles\0".as_ptr() as *const c_char,
            b"instructions\0".as_ptr() as *const c_char,
            core::ptr::null(),
        ];
        let mut buf = [0 as c_char; 256];
        let mut i: usize = 0;

        while ret == -EAGAIN && !try_[i].is_null() {
            snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                b"%s/%s/u\0".as_ptr() as *const c_char,
                (*pmu).name,
                try_[i],
            );
            i += 1;
            ret = perf_do_probe_api(fn_, cpu, buf.as_ptr());
            if ret == 0 {
                return true;
            }
        }
    }
    false
}

unsafe fn perf_probe_sample_identifier(evsel: *mut evsel) {
    (*evsel).core.attr.sample_type |= PERF_SAMPLE_IDENTIFIER;
}

unsafe fn perf_probe_comm_exec(evsel: *mut evsel) {
    (*evsel).core.attr.comm_exec = 1;
}

unsafe fn perf_probe_context_switch(evsel: *mut evsel) {
    (*evsel).core.attr.context_switch = 1;
}

unsafe fn perf_probe_text_poke(evsel: *mut evsel) {
    (*evsel).core.attr.text_poke = 1;
}

unsafe fn perf_probe_build_id(evsel: *mut evsel) {
    (*evsel).core.attr.build_id = 1;
}

unsafe fn perf_probe_cgroup(evsel: *mut evsel) {
    (*evsel).core.attr.cgroup = 1;
}

#[no_mangle]
pub unsafe extern "C" fn perf_can_sample_identifier() -> bool {
    perf_probe_api(perf_probe_sample_identifier)
}

#[no_mangle]
pub unsafe extern "C" fn perf_can_comm_exec() -> bool {
    perf_probe_api(perf_probe_comm_exec)
}

#[no_mangle]
pub unsafe extern "C" fn perf_can_record_switch_events() -> bool {
    perf_probe_api(perf_probe_context_switch)
}

#[no_mangle]
pub unsafe extern "C" fn perf_can_record_text_poke_events() -> bool {
    perf_probe_api(perf_probe_text_poke)
}

#[no_mangle]
pub unsafe extern "C" fn perf_can_record_cpu_wide() -> bool {
    let mut attr = perf_event_attr {
        type_: PERF_TYPE_SOFTWARE,
        config: PERF_COUNT_SW_CPU_CLOCK,
        exclude_kernel: 1,
        ..core::mem::zeroed()
    };
    let cpus: *mut perf_cpu_map;
    let cpu: perf_cpu;
    let fd: c_int;

    cpus = perf_cpu_map__new_online_cpus();
    if cpus.is_null() {
        return false;
    }

    cpu = perf_cpu_map__cpu(cpus, 0);
    perf_cpu_map__put(cpus);

    fd = sys_perf_event_open(&mut attr, -1, cpu.cpu, -1, 0);
    if fd < 0 {
        return false;
    }
    close(fd);

    true
}

/*
 * Architectures are expected to know if AUX area sampling is supported by the
 * hardware. Here we check for kernel support.
 */
#[no_mangle]
pub unsafe extern "C" fn perf_can_aux_sample() -> bool {
    let mut attr = perf_event_attr {
        size: core::mem::size_of::<perf_event_attr>() as u32,
        exclude_kernel: 1,
        /*
         * Non-zero value causes the kernel to calculate the effective
         * attribute size up to that byte.
         */
        aux_sample_size: 1,
        ..core::mem::zeroed()
    };
    let fd: c_int;

    fd = sys_perf_event_open(&mut attr, -1, 0, -1, 0);
    /*
     * If the kernel attribute is big enough to contain aux_sample_size
     * then we assume that it is supported. We are relying on the kernel to
     * validate the attribute size before anything else that could be wrong.
     */
    if fd < 0 && errno == E2BIG {
        return false;
    }
    if fd >= 0 {
        close(fd);
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn perf_can_record_build_id() -> bool {
    perf_probe_api(perf_probe_build_id)
}

#[no_mangle]
pub unsafe extern "C" fn perf_can_record_cgroup() -> bool {
    perf_probe_api(perf_probe_cgroup)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
