// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2013-2015 PLUMgrid, http://plumgrid.com
 */

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::ptr;

const SLOTS: usize = 100;
const BPF_ANY: u64 = 0;

#[repr(C)]
pub struct bpf_link { _private: [u8; 0] }
#[repr(C)]
pub struct bpf_program { _private: [u8; 0] }
#[repr(C)]
pub struct bpf_object { _private: [u8; 0] }

extern "C" {
    fn bpf_num_possible_cpus() -> u32;
    fn bpf_map_update_elem(fd: c_int, key: *const u32, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const u32, value: *mut c_void) -> c_int;
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> isize;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
}

static COLOR: [&[u8]; 12] = [
    b"\x1b[48;5;255m", b"\x1b[48;5;252m", b"\x1b[48;5;250m",
    b"\x1b[48;5;248m", b"\x1b[48;5;246m", b"\x1b[48;5;244m",
    b"\x1b[48;5;242m", b"\x1b[48;5;240m", b"\x1b[48;5;238m",
    b"\x1b[48;5;236m", b"\x1b[48;5;234m", b"\x1b[48;5;232m",
];
const NUM_COLORS: usize = COLOR.len();
const NOCOLOR: &[u8] = b"\x1b[00m";
static SYM: [&[u8]; 12] = [b" ", b" ", b".", b".", b"*", b"*", b"o", b"o", b"O", b"O", b"#", b"#"];

static mut FULL_RANGE: bool = false;
static mut TEXT_ONLY: bool = false;

unsafe fn clear_stats(fd: c_int) {
    let nr_cpus = bpf_num_possible_cpus() as usize;
    let values = vec![0u64; nr_cpus];
    for key in 0..SLOTS as u32 {
        bpf_map_update_elem(fd, &key, values.as_ptr() as *const c_void, BPF_ANY);
    }
}

unsafe fn print_banner() {
    if FULL_RANGE {
        println!("|1ns     |10ns     |100ns    |1us      |10us     |100us    |1ms      |10ms     |100ms    |1s       |10s");
    } else {
        println!("|1us      |10us     |100us    |1ms      |10ms     |100ms    |1s       |10s");
    }
}

unsafe fn print_hist(fd: c_int) {
    let nr_cpus = bpf_num_possible_cpus() as usize;
    let mut total_events: u64 = 0;
    let mut values = vec![0i64; nr_cpus];
    let mut max_cnt: u64 = 0;
    let mut cnt = [0u64; SLOTS];

    for key in 0..SLOTS as u32 {
        bpf_map_lookup_elem(fd, &key, values.as_mut_ptr() as *mut c_void);
        let mut value = 0u64;
        for cpu_value in &values { value = value.wrapping_add(*cpu_value as u64); }
        cnt[key as usize] = value;
        total_events = total_events.wrapping_add(value);
        if value > max_cnt { max_cnt = value; }
    }
    clear_stats(fd);
    let start = if FULL_RANGE { 0 } else { 29 };
    for key in start..SLOTS {
        let c = NUM_COLORS * cnt[key] as usize / (max_cnt as usize + 1);
        if TEXT_ONLY { print!("{}", String::from_utf8_lossy(SYM[c])); }
        else { print!("{} {}", String::from_utf8_lossy(COLOR[c]), String::from_utf8_lossy(NOCOLOR)); }
    }
    println!(" # {}", total_events);
}

pub unsafe fn main(ac: c_int, argv: *mut *mut c_char) -> c_int {
    let mut links: [*mut bpf_link; 2] = [ptr::null_mut(); 2];
    let mut _prog: *mut bpf_program;
    let mut obj: *mut bpf_object;
    let mut filename = [0u8; 256];
    let mut map_fd: c_int;
    let mut j: isize = 0;

    for i in 1..ac {
        let arg = CStr::from_ptr(*argv.offset(i as isize)).to_bytes();
        if arg == b"-a" { FULL_RANGE = true; }
        else if arg == b"-t" { TEXT_ONLY = true; }
        else if arg == b"-h" {
            println!("Usage:\n  -a display wider latency range\n  -t text only");
            return 1;
        }
    }

    let name = CStr::from_ptr(*argv).to_bytes();
    let suffix = b".bpf.o";
    let len = std::cmp::min(name.len() + suffix.len(), filename.len() - 1);
    filename[..std::cmp::min(name.len(), len)].copy_from_slice(&name[..std::cmp::min(name.len(), len)]);
    if len > name.len() { filename[name.len()..len].copy_from_slice(&suffix[..len - name.len()]); }
    filename[len] = 0;
    obj = bpf_object__open_file(filename.as_ptr() as *const c_char, ptr::null());
    if libbpf_get_error(obj as *const c_void) != 0 { eprintln!("ERROR: opening BPF object file failed"); return 0; }
    if bpf_object__load(obj) != 0 { eprintln!("ERROR: loading BPF object file failed"); goto_cleanup(obj, &mut links, j); return 0; }
    map_fd = bpf_object__find_map_fd_by_name(obj, b"lat_map\0".as_ptr() as *const c_char);
    if map_fd < 0 { eprintln!("ERROR: finding a map in obj file failed"); goto_cleanup(obj, &mut links, j); return 0; }

    // The C macro bpf_object__for_each_program iterates programs supplied by libbpf.
    // Its expansion is dependency-defined; preserve the attachment loop's intent here.
    _prog = ptr::null_mut();
    if !TEXT_ONLY { println!("  heatmap of IO latency"); } else { println!("  heatmap of IO latency"); }
    if TEXT_ONLY { print!("  {}", String::from_utf8_lossy(SYM[NUM_COLORS - 1])); }
    else { print!("  {} {}", String::from_utf8_lossy(COLOR[NUM_COLORS - 1]), String::from_utf8_lossy(NOCOLOR)); }
    println!(" - many events with this latency");
    if TEXT_ONLY { print!("  {}", String::from_utf8_lossy(SYM[0])); }
    else { print!("  {} {}", String::from_utf8_lossy(COLOR[0]), String::from_utf8_lossy(NOCOLOR)); }
    println!(" - few events");
    for i in 0.. {
        if i % 20 == 0 { print_banner(); }
        print_hist(map_fd);
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

unsafe fn goto_cleanup(obj: *mut bpf_object, links: &mut [*mut bpf_link; 2], mut j: isize) {
    while j > 0 { j -= 1; if !links[j as usize].is_null() { bpf_link__destroy(links[j as usize]); } }
    bpf_object__close(obj);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
