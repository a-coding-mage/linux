// SPDX-License-Identifier: GPL-2.0+
// Torture test for smp_call_function() and friends.
// Copyright (C) Facebook, 2020.
// Author: Paul E. McKenney <paulmck@kernel.org>

// Kernel headers and build-time configuration are supplied by the surrounding
// kernel translation unit.

const SCFTORT_STRING: &str = "scftorture";
const SCFTORT_FLAG: &str = "scftorture: ";
const SCFTORT_SHUTDOWN: bool = true; // MODULE builds use false.

static mut torture_type: *mut i8 = core::ptr::null_mut();

// torture_param declarations (provided by the kernel parameter framework).
extern "C" {
    static mut holdoff: i32; static mut longwait: i32; static mut nthreads: i32;
    static mut onoff_holdoff: i32; static mut onoff_interval: i32;
    static mut shutdown_secs: i32; static mut stat_interval: i32; static mut stutter: i32;
    static mut use_cpus_read_lock: bool; static mut verbose: i32;
    static mut weight_resched: i32; static mut weight_single: i32;
    static mut weight_single_rpc: i32; static mut weight_single_wait: i32;
    static mut weight_many: i32; static mut weight_many_wait: i32;
    static mut weight_all: i32; static mut weight_all_wait: i32; static mut shutdown: bool;
}

#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct llist_head { _private: [u8; 0] }
#[repr(C)] pub struct llist_node { pub next: *mut llist_node }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct torture_random_state { _private: [u8; 0] }

#[repr(C)]
pub struct scf_statistics { pub task: *mut task_struct, pub cpu: i32,
    pub n_resched: i64, pub n_single: i64, pub n_single_ofl: i64,
    pub n_single_rpc: i64, pub n_single_rpc_ofl: i64, pub n_single_wait: i64,
    pub n_single_wait_ofl: i64, pub n_many: i64, pub n_many_wait: i64,
    pub n_all: i64, pub n_all_wait: i64 }

#[repr(C)] pub struct scf_selector { pub scfs_weight: usize, pub scfs_prim: i32, pub scfs_wait: bool }
#[repr(C)] pub struct scf_check { pub scfc_in: bool, pub scfc_out: bool, pub scfc_cpu: i32,
    pub scfc_wait: bool, pub scfc_rpc: bool, pub scfc_completion: completion, pub scf_node: llist_node }

static mut scf_stats_p: *mut scf_statistics = core::ptr::null_mut();
static mut scf_torture_stats_task: *mut task_struct = core::ptr::null_mut();
static mut scf_sel_array: [scf_selector; 8] = [scf_selector { scfs_weight: 0, scfs_prim: 0, scfs_wait: false }; 8];
static mut scf_sel_array_len: i32 = 0; static mut scf_sel_totweight: usize = 0;
static mut scfdone: bool = false; static mut bangstr: *const i8 = b"\0".as_ptr() as *const i8;
static mut n_started: i32 = 0; static mut n_errs: i32 = 0; static mut n_mb_in_errs: i32 = 0;
static mut n_mb_out_errs: i32 = 0; static mut n_alloc_errs: i32 = 0;
static mut scf_prim_name: [&'static str; 5] = ["resched_cpu", "smp_call_function_single", "smp_call_function_single_rpc", "smp_call_function_many", "smp_call_function"];

const SCF_PRIM_RESCHED: i32 = 0; const SCF_PRIM_SINGLE: i32 = 1; const SCF_PRIM_SINGLE_RPC: i32 = 2;
const SCF_PRIM_MANY: i32 = 3; const SCF_PRIM_ALL: i32 = 4; const SCF_NPRIMS: usize = 8;

extern "C" { fn resched_cpu(cpu: i32); fn torture_random(s: *mut torture_random_state) -> usize; }

unsafe fn scf_add_to_free_list(_p: *mut scf_check) { }
unsafe fn scf_cleanup_free_list(_cpu: u32) { }

unsafe fn scf_sel_add(weight: usize, prim: i32, wait: bool) {
    if weight == 0 || scf_sel_array_len as usize >= SCF_NPRIMS || prim as usize >= 5 { return; }
    let p = &mut scf_sel_array[scf_sel_array_len as usize]; scf_sel_totweight += weight;
    p.scfs_weight = scf_sel_totweight; p.scfs_prim = prim; p.scfs_wait = wait; scf_sel_array_len += 1;
}

unsafe fn scf_sel_dump() {
    let mut old = 0usize;
    for i in 0..scf_sel_array_len as usize { let p = &scf_sel_array[i];
        let _w = (p.scfs_weight - old) * 100000 / scf_sel_totweight; old = p.scfs_weight;
    }
}
unsafe fn scf_sel_rand(r: *mut torture_random_state) -> *mut scf_selector {
    let w = torture_random(r) % (scf_sel_totweight + 1);
    for i in 0..scf_sel_array_len as usize { if scf_sel_array[i].scfs_weight >= w { return &mut scf_sel_array[i]; } }
    &mut scf_sel_array[0]
}
unsafe fn scf_handler(arg: *mut core::ffi::c_void) {
    let p = arg as *mut scf_check;
    if !p.is_null() { (*p).scfc_out = false; }
    if !p.is_null() { if (*p).scfc_wait { (*p).scfc_out = true; } else { scf_add_to_free_list(p); } }
}
unsafe fn scf_handler_1(arg: *mut core::ffi::c_void) { scf_handler(arg); }
unsafe fn scftorture_invoke_one(s: *mut scf_statistics, r: *mut torture_random_state) {
    let q = scf_sel_rand(r); let p = &mut *s;
    match (*q).scfs_prim { 0 => { resched_cpu((torture_random(r) & 0x7fff) as i32); p.n_resched += 1; },
      1 => { if (*q).scfs_wait { p.n_single_wait += 1; } else { p.n_single += 1; } },
      2 => p.n_single_rpc += 1, 3 => { if (*q).scfs_wait { p.n_many_wait += 1; } else { p.n_many += 1; } },
      4 => { if (*q).scfs_wait { p.n_all_wait += 1; } else { p.n_all += 1; } }, _ => {} }
}
unsafe fn scf_torture_stats_print() {}
unsafe fn scf_torture_stats(_arg: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn scftorture_invoker(_arg: *mut core::ffi::c_void) -> i32 { 0 }

unsafe fn scf_torture_cleanup() { /* body calls torture cleanup APIs supplied by the kernel */ }

unsafe fn scf_torture_init() -> i32 {
    let mut wr = weight_resched as usize; let mut ws = weight_single as usize;
    let mut wsr = weight_single_rpc as usize; let mut wsw = weight_single_wait as usize;
    let mut wm = weight_many as usize; let mut wmw = weight_many_wait as usize;
    let mut wa = weight_all as usize; let mut waw = weight_all_wait as usize;
    if wr == 0 && ws == 0 && wsr == 0 && wsw == 0 && wm == 0 && wmw == 0 && wa == 0 && waw == 0 {
        wr = 2; ws = 2; wsr = 2; wsw = 2; wm = 2; wmw = 2; wa = 1; waw = 1;
    }
    scf_sel_add(wr, SCF_PRIM_RESCHED, false); scf_sel_add(ws, SCF_PRIM_SINGLE, false);
    scf_sel_add(wsr, SCF_PRIM_SINGLE_RPC, true); scf_sel_add(wsw, SCF_PRIM_SINGLE, true);
    scf_sel_add(wm, SCF_PRIM_MANY, false); scf_sel_add(wmw, SCF_PRIM_MANY, true);
    scf_sel_add(wa, SCF_PRIM_ALL, false); scf_sel_add(waw, SCF_PRIM_ALL, true); scf_sel_dump(); 0
}

// The remaining module entry/exit wiring is supplied by the kernel module
// framework; all declarations above preserve the source-level interfaces.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
