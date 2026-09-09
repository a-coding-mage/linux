// SPDX-License-Identifier: GPL-2.0-only
/*
 * sampleip: sample instruction pointer and frequency count in a BPF map.
 *
 * Copyright 2016 Netflix, Inc.
 */

// C headers and local headers are external dependencies of this translation.

const DEFAULT_FREQ: i32 = 99;
const DEFAULT_SECS: i32 = 5;
const MAX_IPS: usize = 8192;

static mut map_fd: i32 = 0;
static mut nr_cpus: i32 = 0;
static mut _text_addr: libc::c_long = 0;

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
    pub name: *const libc::c_char,
}
#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
    pub _rest: [u8; 0],
}

#[repr(C)]
struct ipcount {
    ip: u64,
    count: u32,
}

extern "C" {
    fn sys_perf_event_open(attr: *const perf_event_attr, pid: i32, cpu: i32,
                           group_fd: i32, flags: u64) -> i32;
    fn bpf_program__attach_perf_event(prog: *mut bpf_program, pfd: i32) -> *mut bpf_link;
    fn libbpf_get_error(ptr: *const libc::c_void) -> libc::c_long;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_map_get_next_key(fd: i32, key: *const u64, next_key: *mut u64) -> i32;
    fn bpf_map_lookup_elem(fd: i32, key: *const u64, value: *mut u32) -> i32;
    fn load_kallsyms() -> i32;
    fn ksym_get_addr(name: *const libc::c_char) -> libc::c_long;
    fn ksym_search(addr: u64) -> *mut ksym;
    fn bpf_object__open_file(filename: *const libc::c_char, opts: *const libc::c_void) -> *mut bpf_object;
    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const libc::c_char) -> *mut bpf_program;
    fn bpf_object__load(obj: *mut bpf_object) -> i32;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const libc::c_char) -> i32;
    fn bpf_object__close(obj: *mut bpf_object);
}

static mut counts: [ipcount; MAX_IPS] = [ipcount { ip: 0, count: 0 }; MAX_IPS];

unsafe fn usage() {
    println!("USAGE: sampleip [-F freq] [duration]");
    println!("       -F freq    # sample frequency (Hertz), default 99");
    println!("       duration   # sampling duration (seconds), default 5");
}

unsafe fn sampling_start(freq: i32, prog: *mut bpf_program, links: *mut *mut bpf_link) -> i32 {
    let pe_sample_attr = perf_event_attr { type_: 1, size: 0, config: 0, sample_period: freq as u64,
        sample_type: 0, read_format: 0, flags: 1, _rest: [] };
    for i in 0..nr_cpus {
        let pmu_fd = sys_perf_event_open(&pe_sample_attr, -1, i, -1, 0);
        if pmu_fd < 0 { eprintln!("ERROR: Initializing perf sampling"); return 1; }
        let link = bpf_program__attach_perf_event(prog, pmu_fd);
        if libbpf_get_error(link as *const libc::c_void) != 0 {
            eprintln!("ERROR: Attach perf event"); *links.add(i as usize) = core::ptr::null_mut(); libc::close(pmu_fd); return 1;
        }
        *links.add(i as usize) = link;
    }
    0
}

unsafe fn sampling_end(links: *mut *mut bpf_link) {
    for i in 0..nr_cpus { bpf_link__destroy(*links.add(i as usize)); }
}

unsafe fn count_cmp(p1: *const libc::c_void, p2: *const libc::c_void) -> i32 {
    ((*((p1 as *const ipcount))).count as i32) - ((*((p2 as *const ipcount))).count as i32)
}

unsafe fn print_ip_map(fd: i32) {
    println!("{:<19} {:<32} {}", "ADDR", "KSYM", "COUNT");
    let mut key = 0u64; let mut next_key = 0u64; let mut i = 0usize;
    while bpf_map_get_next_key(fd, &key, &mut next_key) == 0 {
        key = next_key; let mut value = 0u32;
        bpf_map_lookup_elem(fd, &next_key, &mut value);
        counts[i].ip = next_key; counts[i].count = value; i += 1;
    }
    libc::qsort(counts.as_mut_ptr() as *mut libc::c_void, i, core::mem::size_of::<ipcount>(), Some(core::mem::transmute(count_cmp as unsafe fn(*const libc::c_void, *const libc::c_void) -> i32)));
    for j in 0..i { if counts[j].ip > _text_addr as u64 { let sym = ksym_search(counts[j].ip); if sym.is_null() { println!("ksym not found. Is kallsyms loaded?"); continue; } println!("0x{: <17x} {: <32} {}", counts[j].ip, "(ksym)", counts[j].count); } else { println!("0x{: <17x} {: <32} {}", counts[j].ip, "(user)", counts[j].count); } }
    if i == MAX_IPS { println!("WARNING: IP hash was full (max {} entries); may have dropped samples", i); }
}

unsafe extern "C" fn int_exit(_sig: i32) { println!(); print_ip_map(map_fd); libc::exit(0); }

pub unsafe fn main(argc: i32, argv: *mut *mut libc::c_char) -> i32 {
    let mut opt: i32; let mut freq = DEFAULT_FREQ; let mut secs = DEFAULT_SECS; let mut error = 1;
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut links: *mut *mut bpf_link = core::ptr::null_mut();
    let mut filename = [0i8; 256];
    while { opt = libc::getopt(argc, argv, b"F:h\0".as_ptr() as *const libc::c_char); opt != -1 } {
        match opt { 70 => freq = libc::atoi(libc::optarg), 104 | _ => { usage(); return 0; } }
    }
    if argc - libc::optind == 1 { secs = libc::atoi(*argv.add(libc::optind as usize)); }
    if freq == 0 || secs == 0 { usage(); return 1; }
    if load_kallsyms() != 0 { eprintln!("ERROR: loading /proc/kallsyms"); return 2; }
    _text_addr = ksym_get_addr(b"_text\0".as_ptr() as *const libc::c_char);
    if _text_addr == 0 { eprintln!("ERROR: no '_text' in /proc/kallsyms"); return 3; }
    nr_cpus = libc::sysconf(libc::_SC_NPROCESSORS_ONLN) as i32;
    links = libc::calloc(nr_cpus as usize, core::mem::size_of::<*mut bpf_link>()) as *mut *mut bpf_link;
    if links.is_null() { eprintln!("ERROR: malloc of links"); return error; }
    libc::snprintf(filename.as_mut_ptr(), filename.len(), b"%s_kern.o\0".as_ptr() as *const libc::c_char, *argv);
    obj = bpf_object__open_file(filename.as_ptr(), core::ptr::null());
    if libbpf_get_error(obj as *const libc::c_void) != 0 { eprintln!("ERROR: opening BPF object file failed"); obj = core::ptr::null_mut(); }
    if obj.is_null() { libc::free(links as *mut libc::c_void); return error; }
    let prog = bpf_object__find_program_by_name(obj, b"do_sample\0".as_ptr() as *const libc::c_char);
    if prog.is_null() { eprintln!("ERROR: finding a prog in obj file failed"); } else if bpf_object__load(obj) != 0 { eprintln!("ERROR: loading BPF object file failed"); } else { map_fd = bpf_object__find_map_fd_by_name(obj, b"ip_map\0".as_ptr() as *const libc::c_char); if map_fd < 0 { eprintln!("ERROR: finding a map in obj file failed"); } else { libc::signal(2, Some(int_exit)); libc::signal(15, Some(int_exit)); println!("Sampling at {} Hertz for {} seconds. Ctrl-C also ends.", freq, secs); if sampling_start(freq, prog, links) == 0 { libc::sleep(secs as u32); error = 0; } } }
    sampling_end(links); if error == 0 { print_ip_map(map_fd); } libc::free(links as *mut libc::c_void); bpf_object__close(obj); error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
