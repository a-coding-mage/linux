// Translated from C source:
// #include <bpf/libbpf.h>
// #include <internal/xyarray.h>
// #include "bpf_skel/augmented_raw_syscalls.skel.h"
// #include "debug.h"
// #include "evlist.h"
// #include "parse-events.h"
// #include "trace_augment.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type pid_t = c_int;
type size_t = usize;
type __u32 = u32;

const BPF_ANY: u64 = 0;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xyarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct evsel_core {
    pub cpus: *mut c_void,
    pub fd: *mut xyarray,
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
pub struct augmented_raw_syscalls_bpf_progs {
    pub sys_enter: *mut bpf_program,
    pub sys_exit: *mut bpf_program,
    pub syscall_unaugmented: *mut bpf_program,
}

#[repr(C)]
pub struct augmented_raw_syscalls_bpf_maps {
    pub __augmented_syscalls__: *mut bpf_map,
    pub pids_filtered: *mut bpf_map,
    pub syscalls_sys_enter: *mut bpf_map,
    pub syscalls_sys_exit: *mut bpf_map,
    pub beauty_map_enter: *mut bpf_map,
}

#[repr(C)]
pub struct augmented_raw_syscalls_bpf {
    pub obj: *mut bpf_object,
    pub progs: augmented_raw_syscalls_bpf_progs,
    pub maps: augmented_raw_syscalls_bpf_maps,
}

static mut skel: *mut augmented_raw_syscalls_bpf = ptr::null_mut();
static mut bpf_output: *mut evsel = ptr::null_mut();

unsafe extern "C" {
    fn augmented_raw_syscalls_bpf__open() -> *mut augmented_raw_syscalls_bpf;
    fn augmented_raw_syscalls_bpf__load(skel: *mut augmented_raw_syscalls_bpf) -> c_int;
    fn augmented_raw_syscalls_bpf__attach(skel: *mut augmented_raw_syscalls_bpf);
    fn augmented_raw_syscalls_bpf__destroy(skel: *mut augmented_raw_syscalls_bpf);

    fn libbpf_strerror(err: c_int, buf: *mut c_char, size: size_t) -> c_int;
    fn bpf_program__set_autoattach(prog: *mut bpf_program, autoattach: bool);
    fn bpf_program__section_name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_map__update_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: size_t,
        value: *const c_void,
        value_sz: size_t,
        flags: u64,
    ) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;

    fn parse_event(evlist: *mut evlist, str_: *const c_char) -> c_int;
    fn evlist__last(evlist: *mut evlist) -> *mut evsel;
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;
    fn xyarray__entry(xy: *mut xyarray, x: c_int, y: c_int) -> *mut c_void;
    fn perf_cpu_map__nr(cpus: *mut c_void) -> c_int;
    fn perf_cpu_map__cpu(cpus: *mut c_void, idx: c_int) -> perf_cpu;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    static mut errno: c_int;
}

unsafe extern "C" {
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

unsafe fn bpf_object__next_program(
    obj: *mut bpf_object,
    prev: *mut bpf_program,
) -> *mut bpf_program {
    unsafe extern "C" {
        fn bpf_object__next_program(
            obj: *mut bpf_object,
            prev: *mut bpf_program,
        ) -> *mut bpf_program;
    }

    bpf_object__next_program(obj, prev)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn augmented_syscalls__prepare() -> c_int {
    let mut prog: *mut bpf_program;
    let mut buf = [0 as c_char; 128];
    let err: c_int;

    skel = augmented_raw_syscalls_bpf__open();
    if skel.is_null() {
        pr_debug(c"Failed to open augmented syscalls BPF skeleton\n".as_ptr());
        return -errno;
    }

    /*
     * Disable attaching the BPF programs except for sys_enter and
     * sys_exit that tail call into this as necessary.
     */
    prog = bpf_object__next_program((*skel).obj, ptr::null_mut());
    while !prog.is_null() {
        if prog != (*skel).progs.sys_enter && prog != (*skel).progs.sys_exit {
            bpf_program__set_autoattach(prog, false);
        }
        prog = bpf_object__next_program((*skel).obj, prog);
    }

    err = augmented_raw_syscalls_bpf__load(skel);
    if err < 0 {
        libbpf_strerror(err, buf.as_mut_ptr(), size_of::<[c_char; 128]>());
        pr_debug(
            c"Failed to load augmented syscalls BPF skeleton: %s\n".as_ptr(),
            buf.as_ptr(),
        );
        return err;
    }

    augmented_raw_syscalls_bpf__attach(skel);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn augmented_syscalls__create_bpf_output(evlist: *mut evlist) -> c_int {
    let err = parse_event(
        evlist,
        c"bpf-output/no-inherit=1,name=__augmented_syscalls__/".as_ptr(),
    );

    if err != 0 {
        pr_err(c"ERROR: Setup BPF output event failed: %d\n".as_ptr(), err);
        return err;
    }

    bpf_output = evlist__last(evlist);
    debug_assert!(evsel__name_is(
        bpf_output,
        c"__augmented_syscalls__".as_ptr()
    ));

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn augmented_syscalls__setup_bpf_output() {
    let mut cpu: perf_cpu;
    let mut i: c_uint;

    if bpf_output.is_null() {
        return;
    }

    /*
     * Set up the __augmented_syscalls__ BPF map to hold for each
     * CPU the bpf-output event's file descriptor.
     */
    i = 0;
    while (i as c_int) < perf_cpu_map__nr((*bpf_output).core.cpus) {
        cpu = perf_cpu_map__cpu((*bpf_output).core.cpus, i as c_int);
        let mycpu = cpu.cpu;

        bpf_map__update_elem(
            (*skel).maps.__augmented_syscalls__,
            &mycpu as *const c_int as *const c_void,
            size_of::<c_int>(),
            xyarray__entry((*bpf_output).core.fd, mycpu, 0),
            size_of::<__u32>(),
            BPF_ANY,
        );
        i = i.wrapping_add(1);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn augmented_syscalls__set_filter_pids(
    nr: c_uint,
    pids: *mut pid_t,
) -> c_int {
    let value = true;
    let mut err = 0;

    if skel.is_null() {
        return 0;
    }

    let mut i: size_t = 0;
    while i < nr as size_t {
        err = bpf_map__update_elem(
            (*skel).maps.pids_filtered,
            pids.add(i) as *const c_void,
            size_of::<pid_t>(),
            &value as *const bool as *const c_void,
            size_of::<bool>(),
            BPF_ANY,
        );
        if err != 0 {
            break;
        }
        i += 1;
    }
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn augmented_syscalls__get_map_fds(
    enter_fd: *mut c_int,
    exit_fd: *mut c_int,
    beauty_fd: *mut c_int,
) -> c_int {
    if skel.is_null() {
        return -1;
    }

    *enter_fd = bpf_map__fd((*skel).maps.syscalls_sys_enter);
    *exit_fd = bpf_map__fd((*skel).maps.syscalls_sys_exit);
    *beauty_fd = bpf_map__fd((*skel).maps.beauty_map_enter);

    if *enter_fd < 0 || *exit_fd < 0 || *beauty_fd < 0 {
        pr_err(c"Error: failed to get syscall or beauty map fd\n".as_ptr());
        return -1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn augmented_syscalls__unaugmented() -> *mut bpf_program {
    (*skel).progs.syscall_unaugmented
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn augmented_syscalls__find_by_title(
    name: *const c_char,
) -> *mut bpf_program {
    let mut pos: *mut bpf_program;
    let mut sec_name: *const c_char;

    if (*skel).obj.is_null() {
        return ptr::null_mut();
    }

    pos = bpf_object__next_program((*skel).obj, ptr::null_mut());
    while !pos.is_null() {
        sec_name = bpf_program__section_name(pos);
        if !sec_name.is_null() && strcmp(sec_name, name) == 0 {
            return pos;
        }
        pos = bpf_object__next_program((*skel).obj, pos);
    }

    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn augmented_syscalls__cleanup() {
    augmented_raw_syscalls_bpf__destroy(skel);
}
