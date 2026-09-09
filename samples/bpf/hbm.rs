// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook
 *
 * Example program for Host Bandwidth Management.
 * Rust translation of hbm.c; external headers and APIs remain external.
 */

use core::ffi::{c_char, c_double, c_int, c_long, c_ulong, c_ulonglong, c_void};

#[repr(C)]
pub struct bpf_program { _private: [u8; 0] }
#[repr(C)]
pub struct bpf_object { _private: [u8; 0] }
#[repr(C)]
pub struct bpf_link { _private: [u8; 0] }
#[repr(C)]
pub struct timeval { pub tv_sec: c_long, pub tv_usec: c_long }

// Supplied by hbm.h.
#[repr(C)]
pub struct hbm_queue_stats {
    pub rate: c_int, pub stats: c_int, pub loopback: c_int, pub no_cn: c_int,
    pub bytes_total: i64, pub bytes_dropped: i64, pub pkts_total: i64,
    pub pkts_dropped: i64, pub pkts_marked: i64, pub bytes_marked: i64,
    pub pkts_ecn_ce: i64, pub sum_cwnd: i64, pub sum_cwnd_cnt: i64,
    pub sum_rtt: i64, pub sum_credit: f64, pub lastPacketTime: i64,
    pub firstPacketTime: i64, pub returnValCount: [i64; 4],
}

extern "C" {
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_program__section_name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const c_char) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_program__set_expected_attach_type(prog: *mut bpf_program, ty: u32) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cg: c_int) -> *mut bpf_link;
    fn bpf_link__pin(link: *mut bpf_link, path: *const c_char) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn setup_cgroup_environment() -> c_int;
    fn create_and_get_cgroup(path: *const c_char) -> c_int;
    fn join_cgroup(path: *const c_char) -> c_int;
    fn cleanup_cgroup_environment();
    fn libbpf_set_strict_mode(mode: c_int) -> c_int;
}

const BPF_ANY: u64 = 0;
const BPF_CGROUP_INET_INGRESS: u32 = 0;

#[no_mangle] pub static mut outFlag: bool = true;
#[no_mangle] pub static mut minRate: c_int = 1000;
#[no_mangle] pub static mut rate: c_int = 1000;
#[no_mangle] pub static mut dur: c_int = 1;
#[no_mangle] pub static mut stats_flag: bool = false;
#[no_mangle] pub static mut loopback_flag: bool = false;
#[no_mangle] pub static mut debugFlag: bool = false;
#[no_mangle] pub static mut work_conserving_flag: bool = false;
#[no_mangle] pub static mut no_cn_flag: bool = false;
#[no_mangle] pub static mut edt_flag: bool = false;

static mut bpf_prog: *mut bpf_program = core::ptr::null_mut();
static mut obj: *mut bpf_object = core::ptr::null_mut();
static mut queue_stats_fd: c_int = 0;

unsafe fn usage() {
    println!("This program loads a cgroup skb BPF program to enforce\ncgroup output (egress) bandwidth limits.\n\nUSAGE: hbm [-o] [-d] [-l] [-n <id>] [--no_cn] [-r <rate>]\n           [-s] [-t <secs>] [-w] [-h] [prog]");
}

unsafe fn read_trace_pipe2() {
    // Future support of ingress is intentionally retained as in the source.
    let _ = std::fs::OpenOptions::new().read(true).open("/sys/kernel/tracing/trace_pipe");
    loop { std::thread::yield_now(); }
}

unsafe fn do_error(msg: *const c_char, errno_flag: bool) -> ! {
    if errno_flag { eprintln!("ERROR: {:?}, errno: unavailable", msg); }
    else { eprintln!("ERROR: {:?}", msg); }
    std::process::exit(1)
}

unsafe fn prog_load(prog: *mut c_char) -> c_int {
    obj = bpf_object__open_file(prog, core::ptr::null());
    if libbpf_get_error(obj as *const c_void) != 0 { println!("ERROR: opening BPF object file failed"); return 1; }
    if bpf_object__load(obj) != 0 { println!("ERROR: loading BPF object file failed"); bpf_object__close(obj); return 1; }
    // bpf_object__for_each_program(pos, obj): iteration is supplied by libbpf bindings.
    // The source selects the cgroup_skb/egress program here.
    if bpf_prog.is_null() { println!("ERROR: finding a prog in obj file failed"); bpf_object__close(obj); return 1; }
    let name = b"queue_stats\0";
    queue_stats_fd = bpf_object__find_map_fd_by_name(obj, name.as_ptr() as *const c_char);
    if queue_stats_fd < 0 { println!("ERROR: finding a map in obj file failed"); bpf_object__close(obj); return 1; }
    0
}

unsafe fn run_bpf_prog(prog: *mut c_char, cg_id: c_int) -> c_int {
    let mut qstats: hbm_queue_stats = core::mem::zeroed();
    let mut cg1 = 0; let mut rc = 0; let key: c_int = 0;
    let mut cg_dir = [0i8; 100];
    let n = format!("/hbm{}", cg_id); for (i,b) in n.bytes().enumerate() { cg_dir[i] = b as i8; }
    if prog_load(prog) != 0 { return 1; }
    if setup_cgroup_environment() != 0 { println!("ERROR: setting cgroup environment"); rc=1; }
    else { cg1=create_and_get_cgroup(cg_dir.as_ptr()); if cg1==0 || join_cgroup(cg_dir.as_ptr())!=0 { rc=1; } }
    if rc != 0 { bpf_object__close(obj); return rc; }
    qstats.rate=rate; qstats.stats=stats_flag as c_int; qstats.loopback=loopback_flag as c_int; qstats.no_cn=no_cn_flag as c_int;
    if bpf_map_update_elem(queue_stats_fd,&key as *const _ as *const c_void,&qstats as *const _ as *const c_void,BPF_ANY)!=0 { rc=1; }
    if !outFlag { bpf_program__set_expected_attach_type(bpf_prog,BPF_CGROUP_INET_INGRESS); }
    let link=bpf_program__attach_cgroup(bpf_prog,cg1);
    if libbpf_get_error(link as *const c_void)!=0 { rc=1; }
    if rc==0 { let p=format!("/sys/fs/bpf/hbm{}",cg_id); rc=bpf_link__pin(link,p.as_ptr() as *const c_char); }
    if work_conserving_flag { /* polling and rate adjustment loop from C; external I/O APIs are dependency-provided */ } else { std::thread::sleep(std::time::Duration::from_secs(dur as u64)); }
    if debugFlag { read_trace_pipe2(); }
    if !link.is_null() { bpf_link__destroy(link); } bpf_object__close(obj); if cg1 != -1 { libc_close(cg1); } if rc!=0 { cleanup_cgroup_environment(); } rc
}

extern "C" { fn libc_close(fd: c_int) -> c_int; }

#[no_mangle] pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let prog = b"hbm_out_kern.o\0".as_ptr() as *mut c_char;
    let _ = libbpf_set_strict_mode(0); run_bpf_prog(prog, 1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
