// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2016 Facebook
 */

// External headers and symbols are supplied by the surrounding build.

const SAMPLE_FREQ: u64 = 50;
const TASK_COMM_LEN: usize = 16;

static mut PID: i32 = 0;
/* counts, stackmap */
static mut MAP_FD: [i32; 2] = [0; 2];
static mut PROG: *mut bpf_program = core::ptr::null_mut();
static mut SYS_READ_SEEN: bool = false;
static mut SYS_WRITE_SEEN: bool = false;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ksym {
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct perf_event_attr {
    pub sample_freq: u64,
    pub freq: u64,
    pub type_: u32,
    pub config: u64,
    pub sample_type: u64,
    pub precise_ip: u64,
    pub inherit: u32,
}

#[repr(C)]
pub struct key_t {
    pub comm: [core::ffi::c_char; TASK_COMM_LEN],
    pub kernstack: u32,
    pub userstack: u32,
}

const PERF_MAX_STACK_DEPTH: usize = 127;
const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_TYPE_SOFTWARE: u32 = 1;
const PERF_TYPE_HW_CACHE: u32 = 3;
const PERF_TYPE_RAW: u32 = 4;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const PERF_COUNT_SW_CPU_CLOCK: u64 = 0;
const PERF_COUNT_HW_CACHE_L1D: u64 = 1;
const PERF_COUNT_HW_CACHE_BPU: u64 = 5;
const PERF_COUNT_HW_CACHE_OP_READ: u64 = 0;
const PERF_COUNT_HW_CACHE_RESULT_ACCESS: u64 = 0;
const PERF_COUNT_HW_CACHE_RESULT_MISS: u64 = 1;
const PERF_SAMPLE_ADDR: u64 = 0x800;
const EEXIST: i32 = 17;
const SIGKILL: i32 = 9;

extern "C" {
    fn ksym_search(addr: u64) -> *mut ksym;
    fn bpf_map_lookup_elem(fd: i32, key: *const core::ffi::c_void, value: *mut core::ffi::c_void) -> i32;
    fn bpf_map_get_next_key(fd: i32, key: *const core::ffi::c_void, next_key: *mut core::ffi::c_void) -> i32;
    fn bpf_map_delete_elem(fd: i32, key: *const core::ffi::c_void) -> i32;
    fn kill(pid: i32, sig: i32) -> i32;
    fn exit(status: i32) -> !;
    fn system(command: *const core::ffi::c_char) -> i32;
    fn strerror(errnum: i32) -> *const core::ffi::c_char;
    fn sysconf(name: i32) -> i64;
    fn calloc(nmemb: usize, size: usize) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::ffi::c_void);
    fn close(fd: i32) -> i32;
    fn sys_perf_event_open(attr: *mut perf_event_attr, pid: i32, cpu: i32, group_fd: i32, flags: u64) -> i32;
    fn bpf_program__attach_perf_event(prog: *mut bpf_program, pfd: i32) -> *mut bpf_link;
    fn libbpf_get_error(ptr: *const core::ffi::c_void) -> i64;
    fn bpf_link__destroy(link: *mut bpf_link) -> i32;
    fn load_kallsyms() -> i32;
    fn read_trace_pipe();
    fn bpf_object__open_file(filename: *const core::ffi::c_char, opts: *const core::ffi::c_void) -> *mut bpf_object;
    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const core::ffi::c_char) -> *mut bpf_program;
    fn bpf_object__load(obj: *mut bpf_object) -> i32;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const core::ffi::c_char) -> i32;
    fn bpf_object__close(obj: *mut bpf_object);
    fn signal(sig: i32, handler: unsafe extern "C" fn(i32)) -> unsafe extern "C" fn(i32);
    fn printf(format: *const core::ffi::c_char, ...) -> i32;
    fn snprintf(buffer: *mut core::ffi::c_char, size: usize, format: *const core::ffi::c_char, ...) -> i32;
    fn fork() -> i32;
}

unsafe fn print_ksym(addr: u64) {
    if addr == 0 { return; }
    let sym = ksym_search(addr);
    if sym.is_null() {
        printf(b"ksym not found. Is kallsyms loaded?\n\0".as_ptr() as _);
        return;
    }
    printf(b"%s;\0".as_ptr() as _, (*sym).name);
    if libc_strstr((*sym).name, b"sys_read\0".as_ptr() as _) == core::ptr::null() {
        SYS_READ_SEEN = true;
    } else if libc_strstr((*sym).name, b"sys_write\0".as_ptr() as _) == core::ptr::null() {
        SYS_WRITE_SEEN = true;
    }
}

unsafe fn print_addr(addr: u64) {
    if addr != 0 { printf(b"%llx;\0".as_ptr() as _, addr); }
}

unsafe fn print_stack(key: *mut key_t, count: u64) {
    let mut ip = [0u64; PERF_MAX_STACK_DEPTH];
    static mut WARNED: bool = false;
    printf(b"%3lld %s;\0".as_ptr() as _, count, (*key).comm.as_ptr());
    if bpf_map_lookup_elem(MAP_FD[1], &(*key).kernstack as *const _ as _, ip.as_mut_ptr() as _) != 0 { printf(b"---;\0".as_ptr() as _); }
    else { for i in (0..PERF_MAX_STACK_DEPTH).rev() { print_ksym(ip[i]); } }
    printf(b"-;\0".as_ptr() as _);
    if bpf_map_lookup_elem(MAP_FD[1], &(*key).userstack as *const _ as _, ip.as_mut_ptr() as _) != 0 { printf(b"---;\0".as_ptr() as _); }
    else { for i in (0..PERF_MAX_STACK_DEPTH).rev() { print_addr(ip[i]); } }
    if count < 6 { printf(b"\r\0".as_ptr() as _); } else { printf(b"\n\0".as_ptr() as _); }
    if (*key).kernstack as i32 == -EEXIST && !WARNED { printf(b"stackmap collisions seen. Consider increasing size\n\0".as_ptr() as _); WARNED = true; }
    else if (*key).kernstack as i32 as i32 < 0 && (*key).userstack as i32 < 0 { printf(b"err stackid %d %d\n\0".as_ptr() as _, (*key).kernstack, (*key).userstack); }
}

unsafe fn err_exit(err: i32) { kill(PID, SIGKILL); exit(err); }

unsafe fn print_stacks() {
    let mut key = core::mem::zeroed::<key_t>(); let mut next_key = core::mem::zeroed::<key_t>();
    let mut value = 0u64; let mut stackid = 0u32; let mut next_id = 0u32; let error = 1;
    SYS_READ_SEEN = false; SYS_WRITE_SEEN = false;
    while bpf_map_get_next_key(MAP_FD[0], &key as *const _ as _, &mut next_key as *mut _ as _) == 0 {
        bpf_map_lookup_elem(MAP_FD[0], &next_key as *const _ as _, &mut value as *mut _ as _); print_stack(&mut next_key, value); bpf_map_delete_elem(MAP_FD[0], &next_key as *const _ as _); key = next_key;
    }
    printf(b"\n\0".as_ptr() as _);
    if !SYS_READ_SEEN || !SYS_WRITE_SEEN { printf(b"BUG kernel stack doesn't contain sys_read() and sys_write()\n\0".as_ptr() as _); err_exit(error); }
    while bpf_map_get_next_key(MAP_FD[1], &stackid as *const _ as _, &mut next_id as *mut _ as _) == 0 { bpf_map_delete_elem(MAP_FD[1], &next_id as *const _ as _); stackid = next_id; }
}

unsafe fn generate_load() -> i32 { if system(b"dd if=/dev/zero of=/dev/null count=5000k status=none\0".as_ptr() as _) < 0 { printf(b"failed to generate some load with dd: %s\n\0".as_ptr() as _, strerror(*core::ptr::addr_of!(errno))); return -1; } 0 }
static mut errno: i32 = 0;

unsafe fn test_perf_event_all_cpu(attr: *mut perf_event_attr) {
    let nr_cpus = sysconf(84) as i32;
    let links = calloc(nr_cpus as usize, core::mem::size_of::<*mut bpf_link>()) as *mut *mut bpf_link;
    let mut error = 1;
    if links.is_null() { printf(b"malloc of links failed\n\0".as_ptr() as _); err_exit(error); }
    (*attr).inherit = 0;
    let mut i = 0;
    while i < nr_cpus {
        let pmu_fd = sys_perf_event_open(attr, -1, i, -1, 0);
        if pmu_fd < 0 { printf(b"sys_perf_event_open failed\n\0".as_ptr() as _); break; }
        *links.add(i as usize) = bpf_program__attach_perf_event(PROG, pmu_fd);
        if libbpf_get_error(*links.add(i as usize) as _) != 0 { printf(b"bpf_program__attach_perf_event failed\n\0".as_ptr() as _); *links.add(i as usize) = core::ptr::null_mut(); close(pmu_fd); break; }
        i += 1;
    }
    if i == nr_cpus && generate_load() >= 0 { print_stacks(); error = 0; }
    while i > 0 { i -= 1; bpf_link__destroy(*links.add(i as usize)); }
    free(links as _); if error != 0 { err_exit(error); }
}

unsafe fn test_perf_event_task(attr: *mut perf_event_attr) {
    let mut link: *mut bpf_link = core::ptr::null_mut(); let mut error = 1;
    (*attr).inherit = 1;
    let pmu_fd = sys_perf_event_open(attr, 0, -1, -1, 0);
    if pmu_fd >= 0 { link = bpf_program__attach_perf_event(PROG, pmu_fd); if libbpf_get_error(link as _) == 0 && generate_load() >= 0 { print_stacks(); error = 0; } else { close(pmu_fd); } }
    if pmu_fd < 0 || error != 0 { printf(b"sys_perf_event_open failed\n\0".as_ptr() as _); }
    bpf_link__destroy(link); if error != 0 { err_exit(error); }
}

unsafe fn test_bpf_perf_event() {
    let mut a = perf_event_attr { sample_freq: SAMPLE_FREQ, freq: 1, type_: PERF_TYPE_HARDWARE, config: PERF_COUNT_HW_CPU_CYCLES, sample_type: 0, precise_ip: 0, inherit: 0 };
    printf(b"Test HW_CPU_CYCLES\n\0".as_ptr() as _); test_perf_event_all_cpu(&mut a); test_perf_event_task(&mut a);
    a.type_ = PERF_TYPE_SOFTWARE; a.config = PERF_COUNT_SW_CPU_CLOCK;
    printf(b"Test SW_CPU_CLOCK\n\0".as_ptr() as _); test_perf_event_all_cpu(&mut a); test_perf_event_task(&mut a);
    a.type_ = PERF_TYPE_HW_CACHE; a.config = PERF_COUNT_HW_CACHE_L1D | (PERF_COUNT_HW_CACHE_OP_READ << 8) | (PERF_COUNT_HW_CACHE_RESULT_ACCESS << 16);
    printf(b"Test HW_CACHE_L1D\n\0".as_ptr() as _); test_perf_event_all_cpu(&mut a); test_perf_event_task(&mut a);
    a.config = PERF_COUNT_HW_CACHE_BPU | (PERF_COUNT_HW_CACHE_OP_READ << 8) | (PERF_COUNT_HW_CACHE_RESULT_MISS << 16);
    printf(b"Test HW_CACHE_BPU\n\0".as_ptr() as _); test_perf_event_all_cpu(&mut a); test_perf_event_task(&mut a);
    a.type_ = PERF_TYPE_RAW; a.config = 0xc0; a.sample_type = 0; a.precise_ip = 0;
    printf(b"Test Instruction Retired\n\0".as_ptr() as _); test_perf_event_all_cpu(&mut a); test_perf_event_task(&mut a);
    a.config = 0x21d0; a.sample_type = PERF_SAMPLE_ADDR; a.precise_ip = 2;
    printf(b"Test Lock Load\n\0".as_ptr() as _); test_perf_event_all_cpu(&mut a); test_perf_event_task(&mut a); printf(b"*** PASS ***\n\0".as_ptr() as _);
}

pub unsafe fn main(argc: i32, argv: *mut *mut core::ffi::c_char) -> i32 {
    let mut filename = [0i8; 256]; let mut error = 1; let mut obj: *mut bpf_object = core::ptr::null_mut();
    snprintf(filename.as_mut_ptr(), filename.len(), b"%s_kern.o\0".as_ptr() as _, *argv);
    if load_kallsyms() != 0 { printf(b"failed to process /proc/kallsyms\n\0".as_ptr() as _); } else { obj = bpf_object__open_file(filename.as_ptr() as _, core::ptr::null()); if libbpf_get_error(obj as _) == 0 { PROG = bpf_object__find_program_by_name(obj, b"bpf_prog1\0".as_ptr() as _); if !PROG.is_null() && bpf_object__load(obj) == 0 { MAP_FD[0] = bpf_object__find_map_fd_by_name(obj, b"counts\0".as_ptr() as _); MAP_FD[1] = bpf_object__find_map_fd_by_name(obj, b"stackmap\0".as_ptr() as _); if MAP_FD[0] >= 0 && MAP_FD[1] >= 0 { PID = fork(); if PID == 0 { read_trace_pipe(); return 0; } if PID > 0 { test_bpf_perf_event(); error = 0; } } } } }
    bpf_object__close(obj); err_exit(error); let _ = argc; 0
}

extern "C" { fn libc_strstr(haystack: *const core::ffi::c_char, needle: *const core::ffi::c_char) -> *mut core::ffi::c_char; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
