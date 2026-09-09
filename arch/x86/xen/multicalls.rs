// SPDX-License-Identifier: GPL-2.0
/*
 * Xen hypercall batching.
 *
 * Xen allows multiple hypercalls to be issued at once, using the
 * multicall interface.  This allows the cost of trapping into the
 * hypervisor to be amortized over several calls.
 *
 * This file implements a simple interface for multicalls.  There's a
 * per-cpu buffer of outstanding multicalls.  When you want to queue a
 * multicall for issuing, you can allocate a multicall slot for the
 * call and its arguments, along with storage for space which is
 * pointed to by the arguments (for passing pointers to structures,
 * etc).  When the multicall is actually issued, all the space for the
 * commands and allocated memory is freed for reuse.
 *
 * Multicalls are flushed whenever any of the buffers get full, or
 * when explicitly requested.  There's no way to get per-multicall
 * return results back.  It will BUG if any of the multicalls fail.
 *
 * Jeremy Fitzhardinge <jeremy@xensource.com>, XenSource Inc, 2007
 */

const MC_BATCH: usize = 32;
const MC_ARGS: usize = MC_BATCH * 16;

#[repr(C)]
pub struct multicall_entry {
    pub op: c_ulong,
    pub result: c_long,
    pub args: [c_ulong; 5],
}

#[repr(C)]
pub struct callback {
    pub fn_: Option<unsafe extern "C" fn(*mut c_void)>,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct mc_buffer {
    pub mcidx: c_uint,
    pub argidx: c_uint,
    pub cbidx: c_uint,
    pub entries: [multicall_entry; MC_BATCH],
    pub args: [u8; MC_ARGS],
    pub callbacks: [callback; MC_BATCH],
}

#[repr(C)]
pub struct mc_debug_data {
    pub entries: [multicall_entry; MC_BATCH],
    pub caller: [*mut c_void; MC_BATCH],
    pub argsz: [usize; MC_BATCH],
    pub args: [*mut c_ulong; MC_BATCH],
}

#[repr(C)]
pub struct multicall_space {
    pub mc: *mut multicall_entry,
    pub args: *mut u8,
}

pub type c_void = core::ffi::c_void;
pub type c_ulong = usize;
pub type c_long = isize;
pub type c_uint = u32;

extern "C" {
    static mut mc_buffer: mc_buffer;
    static mut mc_debug_data_early: mc_debug_data;
    static mut mc_debug_data_ptr: *mut mc_debug_data;
    static mut xen_mc_irq_flags: c_ulong;
    static mut mc_debug: u8;
    static mut mc_debug_enabled: bool;

    fn this_cpu_ptr<T>(ptr: *mut T) -> *mut T;
    fn static_key_slow_inc(key: *mut u8);
    fn static_key_slow_dec(key: *mut u8);
    fn alloc_percpu<T>() -> *mut T;
    fn xen_mc_flush();
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn xen_single_call(op: c_ulong, a0: c_ulong, a1: c_ulong, a2: c_ulong,
                       a3: c_ulong, a4: c_ulong) -> c_long;
    fn hypervisor_multicall(entries: *mut multicall_entry, count: c_uint) -> c_int;
    fn smp_processor_id() -> c_int;
    fn trace_xen_mc_flush(mcidx: c_uint, argidx: c_uint, cbidx: c_uint);
    fn trace_xen_mc_entry_alloc(args: usize);
    fn trace_xen_mc_flush_reason(reason: c_int);
    fn trace_xen_mc_extend_args(op: c_ulong, size: usize, result: c_int);
    fn trace_xen_mc_callback(fn_: Option<unsafe extern "C" fn(*mut c_void)>, data: *mut c_void);
    fn pr_err(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn pr_cont(fmt: *const u8, ...);
}

pub type c_int = i32;

const XEN_MC_FL_BATCH: c_int = 0;
const XEN_MC_FL_ARGS: c_int = 1;
const XEN_MC_FL_CALLBACK: c_int = 2;
const XEN_MC_XE_BAD_OP: c_int = 0;
const XEN_MC_XE_NO_SPACE: c_int = 1;
const XEN_MC_XE_OK: c_int = 2;

static hpcpars: [u8; 6] = [4, 2, 1, 2, 3, 4];

unsafe fn get_mc_debug() -> *mut mc_debug_data {
    if mc_debug_data_ptr.is_null() {
        &raw mut mc_debug_data_early
    } else {
        this_cpu_ptr(mc_debug_data_ptr)
    }
}

unsafe extern "C" fn xen_parse_mc_debug(_arg: *mut u8) -> c_int {
    mc_debug_enabled = true;
    static_key_slow_inc(&raw mut mc_debug);
    0
}

unsafe extern "C" fn mc_debug_enable() -> c_int {
    let mut flags: c_ulong = 0;
    if !mc_debug_enabled {
        return 0;
    }
    let mcdb = alloc_percpu::<mc_debug_data>();
    if mcdb.is_null() {
        pr_err(b"xen_mc_debug inactive\0".as_ptr());
        static_key_slow_dec(&raw mut mc_debug);
        return -12;
    }
    local_irq_save(&raw mut flags);
    xen_mc_flush();
    mc_debug_data_ptr = mcdb;
    local_irq_restore(flags);
    pr_info(b"xen_mc_debug active\0".as_ptr());
    0
}

unsafe fn print_debug_data(b: *mut mc_buffer, mcdb: *mut mc_debug_data, idx: usize) {
    let entry = &(*mcdb).entries[idx];
    let opidx = (entry.op & 0xff) as usize;
    let pars = if opidx < hpcpars.len() { hpcpars[opidx] } else { 0 };
    pr_err(b"  call %2d: op=%lu result=%ld caller=%pS \0".as_ptr(), idx + 1,
           entry.op, (*b).entries[idx].result, (*mcdb).caller[idx]);
    if pars != 0 {
        pr_cont(b"pars=\0".as_ptr());
        for arg in 0..pars as usize { pr_cont(b"%lx \0".as_ptr(), (*mcdb).args[idx].add(arg).read()); }
    }
    if (*mcdb).argsz[idx] != 0 {
        pr_cont(b"args=\0".as_ptr());
        for arg in 0..(*mcdb).argsz[idx] / 8 { pr_cont(b"%lx \0".as_ptr(), (*mcdb).args[idx].add(arg).read()); }
    }
    pr_cont(b"\n\0".as_ptr());
}

pub unsafe fn xen_mc_flush() {
    let b = this_cpu_ptr(&raw mut mc_buffer);
    let mut mcdb: *mut mc_debug_data = core::ptr::null_mut();
    let mut ret = 0;
    let mut flags: c_ulong = 0;
    local_irq_save(&raw mut flags);
    trace_xen_mc_flush((*b).mcidx, (*b).argidx, (*b).cbidx);
    if mc_debug != 0 { mcdb = get_mc_debug(); (*mcdb).entries[..(*b).mcidx as usize].copy_from_slice(&(*b).entries[..(*b).mcidx as usize]); }
    match (*b).mcidx {
        0 => {},
        1 => { let e = &mut (*b).entries[0]; e.result = xen_single_call(e.op, e.args[0], e.args[1], e.args[2], e.args[3], e.args[4]); ret = (e.result < 0) as i32; },
        _ => { if hypervisor_multicall((*b).entries.as_mut_ptr(), (*b).mcidx) != 0 { core::hint::unreachable_unchecked(); } for i in 0..(*b).mcidx as usize { if (*b).entries[i].result < 0 { ret += 1; } } }
    }
    if ret != 0 { pr_err(b"%d of %d multicall(s) failed: cpu %d\n\0".as_ptr(), ret, (*b).mcidx, smp_processor_id()); for i in 0..(*b).mcidx as usize { if mc_debug != 0 { print_debug_data(b, mcdb, i); } } }
    (*b).mcidx = 0; (*b).argidx = 0;
    for i in 0..(*b).cbidx as usize { let cb = &(*b).callbacks[i]; if let Some(f) = cb.fn_ { f(cb.data); } }
    (*b).cbidx = 0;
    local_irq_restore(flags);
}

pub unsafe fn __xen_mc_entry(args: usize) -> multicall_space {
    let b = this_cpu_ptr(&raw mut mc_buffer);
    let argidx = ((*b).argidx as usize + 7) & !7;
    trace_xen_mc_entry_alloc(args);
    if (*b).mcidx as usize == MC_BATCH || argidx + args >= MC_ARGS { trace_xen_mc_flush_reason(if (*b).mcidx as usize == MC_BATCH { XEN_MC_FL_BATCH } else { XEN_MC_FL_ARGS }); xen_mc_flush(); }
    let idx = (*b).mcidx as usize;
    (*b).mcidx += 1;
    (*b).argidx = argidx as c_uint + args as c_uint;
    multicall_space { mc: &raw mut (*b).entries[idx], args: (*b).args.as_mut_ptr().add(argidx) }
}

pub unsafe fn xen_mc_extend_args(op: c_ulong, size: usize) -> multicall_space {
    let b = this_cpu_ptr(&raw mut mc_buffer);
    if (*b).mcidx == 0 || (*b).entries[(*b).mcidx as usize - 1].op != op || (*b).argidx as usize + size >= MC_ARGS { return multicall_space { mc: core::ptr::null_mut(), args: core::ptr::null_mut() }; }
    let ret = multicall_space { mc: &raw mut (*b).entries[(*b).mcidx as usize - 1], args: (*b).args.as_mut_ptr().add((*b).argidx as usize) };
    (*b).argidx += size as c_uint; ret
}

pub unsafe fn xen_mc_callback(fn_: Option<unsafe extern "C" fn(*mut c_void)>, data: *mut c_void) {
    let b = this_cpu_ptr(&raw mut mc_buffer);
    if (*b).cbidx as usize == MC_BATCH { trace_xen_mc_flush_reason(XEN_MC_FL_CALLBACK); xen_mc_flush(); }
    trace_xen_mc_callback(fn_, data);
    let cb = &mut (*b).callbacks[(*b).cbidx as usize]; (*b).cbidx += 1; cb.fn_ = fn_; cb.data = data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
