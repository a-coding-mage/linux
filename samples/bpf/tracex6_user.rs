// SPDX-License-Identifier: GPL-2.0
// C headers and project headers are supplied by the surrounding build.

use core::ffi::{c_char, c_int, c_void};

const SAMPLE_PERIOD: u64 = 0x7fffffffffffffff;

// External C/library and libbpf symbols used by this translation.
#[repr(C)]
pub struct perf_event_attr {
    pub freq: u32,
    pub sample_period: u64,
    pub inherit: u32,
    pub type_: u32,
    pub read_format: u64,
    pub sample_type: u64,
    pub config: u64,
}

#[repr(C)]
pub struct bpf_perf_event_value {
    pub counter: u64,
    pub enabled: u64,
    pub running: u64,
}

#[repr(C)]
pub struct bpf_link { _private: [u8; 0] }
#[repr(C)]
pub struct bpf_program { _private: [u8; 0] }
#[repr(C)]
pub struct bpf_object { _private: [u8; 0] }
#[repr(C)]
pub struct cpu_set_t { _private: [u8; 0] }

extern "C" {
    fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
    fn sysconf(name: c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: usize, arg: usize) -> c_int;
    fn exit(status: c_int) -> !;
    fn sys_perf_event_open(attr: *mut perf_event_attr, pid: c_int, cpu: c_int,
                           group_fd: c_int, flags: usize) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> usize;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
}

static mut MAP_FD: [c_int; 3] = [0; 3];

unsafe fn check_on_cpu(cpu: c_int, attr: *mut perf_event_attr) {
    let mut value2 = bpf_perf_event_value { counter: 0, enabled: 0, running: 0 };
    let mut pmu_fd: c_int;
    let mut error: c_int = 0;
    let mut value: u64 = 0;
    let mut set = core::mem::MaybeUninit::<cpu_set_t>::zeroed().assume_init();

    // CPU_ZERO(&set); CPU_SET(cpu, &set);
    assert!(sched_setaffinity(0, core::mem::size_of::<cpu_set_t>(), &set) == 0);
    pmu_fd = sys_perf_event_open(attr, -1, cpu, -1, 0);
    if pmu_fd < 0 {
        eprintln!("sys_perf_event_open failed on CPU {}", cpu);
        error = 1;
        exit_path(error, pmu_fd, cpu);
    }
    assert!(bpf_map_update_elem(MAP_FD[0], &cpu as *const _ as *const c_void,
        &pmu_fd as *const _ as *const c_void, 0) == 0);
    assert!(ioctl(pmu_fd, 0x2400, 0) == 0);
    bpf_map_get_next_key(MAP_FD[1], &cpu as *const _ as *const c_void, core::ptr::null_mut());
    if bpf_map_lookup_elem(MAP_FD[1], &cpu as *const _ as *const c_void,
        &mut value as *mut _ as *mut c_void) != 0 {
        eprintln!("Value missing for CPU {}", cpu);
        error = 1;
    } else { eprintln!("CPU {}: {}", cpu, value); }
    if error == 0 && bpf_map_lookup_elem(MAP_FD[2], &cpu as *const _ as *const c_void,
        &mut value2 as *mut _ as *mut c_void) != 0 {
        eprintln!("Value2 missing for CPU {}", cpu);
        error = 1;
    } else if error == 0 {
        eprintln!("CPU {}: counter: {}, enabled: {}, running: {}", cpu, value2.counter, value2.enabled, value2.running);
    }
    exit_path(error, pmu_fd, cpu);
}

unsafe fn exit_path(error: c_int, pmu_fd: c_int, cpu: c_int) -> ! {
    assert!(bpf_map_delete_elem(MAP_FD[0], &cpu as *const _ as *const c_void) == 0 || error != 0);
    assert!(ioctl(pmu_fd, 0x2401, 0) == 0 || error != 0);
    assert!(close(pmu_fd) == 0 || error != 0);
    assert!(bpf_map_delete_elem(MAP_FD[1], &cpu as *const _ as *const c_void) == 0 || error != 0);
    exit(error)
}

unsafe fn test_perf_event_array(attr: *mut perf_event_attr, name: *const c_char) {
    let nr_cpus = sysconf(83);
    let mut pids = vec![0i32; nr_cpus as usize];
    let mut err = 0;
    // printf("Test reading %s counters\n", name);
    for i in 0..nr_cpus {
        pids[i as usize] = fork();
        assert!(pids[i as usize] >= 0);
        if pids[i as usize] == 0 {
            check_on_cpu(i, attr);
            exit(1);
        }
    }
    for pid in pids {
        let mut status = 0;
        assert!(waitpid(pid, &mut status, 0) == pid);
        err |= status;
    }
    if err != 0 { eprintln!("Test failed"); }
    let _ = name;
}

unsafe fn test_bpf_perf_event() {
    let mut attr = perf_event_attr { freq: 0, sample_period: SAMPLE_PERIOD, inherit: 0,
        type_: 0, read_format: 0, sample_type: 0, config: 0 };
    // Attribute constants and full kernel layout are supplied by the perf-sys dependency.
    attr.type_ = 0; attr.config = 0;
    test_perf_event_array(&mut attr, c"HARDWARE-cycles".as_ptr());
    attr.type_ = 1; test_perf_event_array(&mut attr, c"SOFTWARE-clock".as_ptr());
    attr.type_ = 4; attr.config = 0xc0; test_perf_event_array(&mut attr, c"RAW-instruction-retired".as_ptr());
    attr.type_ = 3; test_perf_event_array(&mut attr, c"HW_CACHE-L1D-load".as_ptr());
    test_perf_event_array(&mut attr, c"HW_CACHE-LLC-miss".as_ptr());
    attr.sample_period = 0; attr.type_ = 7; attr.config = 0;
    test_perf_event_array(&mut attr, c"Dynamic-msr-tsc".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut filename = [0u8; 256];
    let _ = argc;
    // snprintf(filename, sizeof(filename), "%s.bpf.o", argv[0]);
    let obj = bpf_object__open_file(filename.as_ptr() as *const c_char, core::ptr::null());
    if libbpf_get_error(obj as *const c_void) != 0 { eprintln!("ERROR: opening BPF object file failed"); return 0; }
    if bpf_object__load(obj) != 0 { eprintln!("ERROR: loading BPF object file failed"); bpf_object__close(obj); return 0; }
    for (i, n) in [c"counters", c"values", c"values2"].iter().enumerate() {
        MAP_FD[i] = bpf_object__find_map_fd_by_name(obj, n.as_ptr());
    }
    if MAP_FD.iter().any(|&fd| fd < 0) { eprintln!("ERROR: finding a map in obj file failed"); bpf_object__close(obj); return 0; }
    test_bpf_perf_event();
    bpf_object__close(obj);
    let _ = argv;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
