// SPDX-License-Identifier: GPL-2.0

// C headers and libbpf dependencies are supplied by the surrounding build.
use std::ffi::{c_char, c_int, c_void};
use std::ptr;

const MAX_CPU: usize = 8;
const MAX_PSTATE_ENTRIES: usize = 5;
const MAX_CSTATE_ENTRIES: usize = 3;
const MAX_STARS: usize = 40;

const CPUFREQ_MAX_SYSFS_PATH: &[u8] = b"/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq\0";
const CPUFREQ_LOWEST_FREQ: &[u8] = b"208000";
const CPUFREQ_HIGHEST_FREQ: &[u8] = b"12000000";

#[repr(C)]
struct CpuStatData {
    cstate: [c_ulong; MAX_CSTATE_ENTRIES],
    pstate: [c_ulong; MAX_PSTATE_ENTRIES],
}

type c_ulong = u64;

#[repr(C)]
struct BpfLink;
#[repr(C)]
struct BpfProgram;
#[repr(C)]
struct BpfObject;

extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn sprintf(string: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(string: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn strlen(string: *const c_char) -> usize;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn sleep(seconds: u32) -> u32;
    fn signal(signum: c_int, handler: Option<unsafe extern "C" fn(c_int)>) -> Option<unsafe extern "C" fn(c_int)>;
    fn sysconf(name: c_int) -> c_long;
    fn sched_getcpu() -> c_int;
    fn sched_getaffinity(pid: c_int, size: usize, set: *mut CpuSet) -> c_int;
    fn sched_setaffinity(pid: c_int, size: usize, set: *const CpuSet) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut BpfObject;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn bpf_object__find_program_by_name(obj: *mut BpfObject, name: *const c_char) -> *mut BpfProgram;
    fn bpf_object__load(obj: *mut BpfObject) -> c_int;
    fn bpf_object__find_map_fd_by_name(obj: *mut BpfObject, name: *const c_char) -> c_int;
    fn bpf_program__attach(prog: *mut BpfProgram) -> *mut BpfLink;
    fn bpf_link__destroy(link: *mut BpfLink);
    fn bpf_object__close(obj: *mut BpfObject);
}

type c_long = i64;

#[repr(C)]
#[derive(Copy, Clone)]
struct CpuSet { bits: [usize; 16] }

static mut CSTATE_MAP_FD: c_int = 0;
static mut PSTATE_MAP_FD: c_int = 0;
static mut STAT_DATA: [CpuStatData; MAX_CPU] = [const { CpuStatData { cstate: [0; MAX_CSTATE_ENTRIES], pstate: [0; MAX_PSTATE_ENTRIES] } }; MAX_CPU];

unsafe fn cpu_stat_print() {
    let mut state_str = [0 as c_char; 9];
    printf(b"\x1b[2J\0".as_ptr() as *const c_char);
    printf(b"\nCPU states statistics:\n\0".as_ptr() as *const c_char);
    printf(b"%-10s \0".as_ptr() as *const c_char, b"state(ms)\0".as_ptr());
    for i in 0..MAX_CSTATE_ENTRIES { sprintf(state_str.as_mut_ptr(), b"cstate-%d\0".as_ptr() as *const c_char, i as c_int); printf(b"%-11s \0".as_ptr() as *const c_char, state_str.as_ptr()); }
    for i in 0..MAX_PSTATE_ENTRIES { sprintf(state_str.as_mut_ptr(), b"pstate-%d\0".as_ptr() as *const c_char, i as c_int); printf(b"%-11s \0".as_ptr() as *const c_char, state_str.as_ptr()); }
    printf(b"\n\0".as_ptr() as *const c_char);
    for j in 0..MAX_CPU {
        printf(b"CPU-%-6d \0".as_ptr() as *const c_char, j as c_int);
        for i in 0..MAX_CSTATE_ENTRIES { printf(b"%-11lu \0".as_ptr() as *const c_char, STAT_DATA[j].cstate[i] / 1_000_000); }
        for i in 0..MAX_PSTATE_ENTRIES { printf(b"%-11lu \0".as_ptr() as *const c_char, STAT_DATA[j].pstate[i] / 1_000_000); }
        printf(b"\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn cpu_stat_update(cstate_fd: c_int, pstate_fd: c_int) {
    let mut value: c_ulong = 0;
    for c in 0..MAX_CPU {
        for i in 0..MAX_CSTATE_ENTRIES { let key = (c * MAX_CSTATE_ENTRIES + i) as c_ulong; bpf_map_lookup_elem(cstate_fd, &key as *const _ as *const c_void, &mut value as *mut _ as *mut c_void); STAT_DATA[c].cstate[i] = value; }
        for i in 0..MAX_PSTATE_ENTRIES { let key = (c * MAX_PSTATE_ENTRIES + i) as c_ulong; bpf_map_lookup_elem(pstate_fd, &key as *const _ as *const c_void, &mut value as *mut _ as *mut c_void); STAT_DATA[c].pstate[i] = value; }
    }
}

// This function is copied from idlestat_wake_all() in idlestate.c.
unsafe fn cpu_stat_inject_cpu_idle_event() -> c_int { let ret = sysconf(83); if ret < 0 { return -1; } let rcpu = sched_getcpu(); if rcpu < 0 { return -1; } let mut original = CpuSet { bits: [0; 16] }; let mut mask = CpuSet { bits: [0; 16] }; sched_getaffinity(0, std::mem::size_of::<CpuSet>(), &mut original); for i in 0..ret as usize { if i as c_int == rcpu { continue; } if (original.bits[i / usize::BITS as usize] & (1 << (i % usize::BITS as usize))) == 0 { continue; } mask.bits = [0; 16]; mask.bits[i / usize::BITS as usize] |= 1 << (i % usize::BITS as usize); sched_setaffinity(0, std::mem::size_of::<CpuSet>(), &mask); } sched_setaffinity(0, std::mem::size_of::<CpuSet>(), &original); 0 }

unsafe fn cpu_stat_inject_cpu_frequency_event() -> c_int { let fd = open(CPUFREQ_MAX_SYSFS_PATH.as_ptr() as *const c_char, 1); if fd < 0 { printf(b"failed to open scaling_max_freq, errno=%d\n\0".as_ptr() as *const c_char, errno); return fd; } let mut len = write(fd, CPUFREQ_LOWEST_FREQ.as_ptr() as *const c_void, CPUFREQ_LOWEST_FREQ.len()); if len < 0 { printf(b"failed to open scaling_max_freq, errno=%d\n\0".as_ptr() as *const c_char, errno); close(fd); return len as c_int; } len = write(fd, CPUFREQ_HIGHEST_FREQ.as_ptr() as *const c_void, CPUFREQ_HIGHEST_FREQ.len()); if len < 0 { printf(b"failed to open scaling_max_freq, errno=%d\n\0".as_ptr() as *const c_char, errno); } close(fd); len as c_int }

unsafe extern "C" fn int_exit(_sig: c_int) { cpu_stat_inject_cpu_idle_event(); cpu_stat_inject_cpu_frequency_event(); cpu_stat_update(CSTATE_MAP_FD, PSTATE_MAP_FD); cpu_stat_print(); exit(0); }

fn main() {
    unsafe {
        let args: Vec<std::ffi::CString> = std::env::args().map(|s| std::ffi::CString::new(s).unwrap()).collect();
        let argv0 = args[0].as_ptr();
        let mut filename = [0 as c_char; 256];
        snprintf(filename.as_mut_ptr(), filename.len(), b"%s_kern.o\0".as_ptr() as *const c_char, argv0);
        let obj = bpf_object__open_file(filename.as_ptr(), ptr::null());
        if libbpf_get_error(obj as *const c_void) != 0 { fprintf(stderr, b"ERROR: opening BPF object file failed\n\0".as_ptr() as *const c_char); return; }
        let prog = bpf_object__find_program_by_name(obj, b"bpf_prog1\0".as_ptr() as *const c_char);
        if prog.is_null() { printf(b"finding a prog in obj file failed\n\0".as_ptr() as *const c_char); bpf_link__destroy(ptr::null_mut()); bpf_object__close(obj); return; }
        if bpf_object__load(obj) != 0 { fprintf(stderr, b"ERROR: loading BPF object file failed\n\0".as_ptr() as *const c_char); bpf_object__close(obj); return; }
        CSTATE_MAP_FD = bpf_object__find_map_fd_by_name(obj, b"cstate_duration\0".as_ptr() as *const c_char);
        PSTATE_MAP_FD = bpf_object__find_map_fd_by_name(obj, b"pstate_duration\0".as_ptr() as *const c_char);
        if CSTATE_MAP_FD < 0 || PSTATE_MAP_FD < 0 { fprintf(stderr, b"ERROR: finding a map in obj file failed\n\0".as_ptr() as *const c_char); bpf_object__close(obj); return; }
        let mut link = bpf_program__attach(prog);
        if libbpf_get_error(link as *const c_void) != 0 { fprintf(stderr, b"ERROR: bpf_program__attach failed\n\0".as_ptr() as *const c_char); link = ptr::null_mut(); bpf_object__close(obj); return; }
        if cpu_stat_inject_cpu_idle_event() < 0 || cpu_stat_inject_cpu_frequency_event() < 0 { return; }
        signal(2, Some(int_exit)); signal(15, Some(int_exit));
        loop { cpu_stat_update(CSTATE_MAP_FD, PSTATE_MAP_FD); cpu_stat_print(); sleep(5); }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
