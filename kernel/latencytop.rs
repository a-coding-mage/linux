// SPDX-License-Identifier: GPL-2.0-only
/*
 * latencytop.c: Latency display infrastructure
 *
 * Rust translation of the implementation source. Kernel-provided declarations
 * and constants from the included headers are represented as external items.
 */

use core::ffi::{c_int, c_ulong, c_void};

const MAXLR: usize = 128;
// Supplied by linux/latencytop.h in the original source.
const LT_BACKTRACEDEPTH: usize = 0;
const LT_SAVECOUNT: usize = 0;

#[repr(C)]
pub struct latency_record {
    pub count: c_int,
    pub time: c_ulong,
    pub max: c_ulong,
    pub backtrace: [c_ulong; LT_BACKTRACEDEPTH],
}

#[repr(C)]
pub struct task_struct {
    pub mm: *mut c_void,
    pub latency_record: [latency_record; LT_SAVECOUNT],
    pub latency_record_count: c_int,
}

#[repr(C)]
pub struct ctl_table;
#[repr(C)]
pub struct seq_file;
#[repr(C)]
pub struct file;
#[repr(C)]
pub struct inode;

extern "C" {
    fn proc_dointvec(table: *const ctl_table, write: c_int, buffer: *mut c_void,
                     lenp: *mut usize, ppos: *mut i64) -> c_int;
    fn force_schedstat_enabled();
    fn raw_spin_lock_irqsave(lock: *mut c_void, flags: *mut c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn stack_trace_save_tsk(task: *mut task_struct, store: *mut c_ulong,
                            size: usize, skipnr: c_int) -> usize;
    fn seq_puts(m: *mut seq_file, s: *const u8) -> c_int;
    fn seq_printf(m: *mut seq_file, fmt: *const u8, ...);
    fn single_open(filp: *mut file, show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int,
                   data: *mut c_void) -> c_int;
    fn seq_read();
    fn seq_write();
    fn seq_lseek();
    fn single_release();
    fn proc_create(name: *const u8, mode: u32, parent: *mut c_void, ops: *const c_void) -> *mut c_void;
    fn register_sysctl_init(name: *const u8, table: *const c_void) -> *mut c_void;
}

static mut latency_lock: *mut c_void = core::ptr::null_mut();
static mut latency_record: [latency_record; MAXLR] = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut latencytop_enabled: c_int = 0;

#[inline]
unsafe fn clear_tsk_latency_tracing(p: *mut task_struct) {
    let mut flags = 0;
    raw_spin_lock_irqsave(latency_lock, &mut flags);
    core::ptr::write_bytes((*p).latency_record.as_mut_ptr(), 0, (*p).latency_record.len());
    (*p).latency_record_count = 0;
    raw_spin_unlock_irqrestore(latency_lock, flags);
}

unsafe fn clear_global_latency_tracing() {
    let mut flags = 0;
    raw_spin_lock_irqsave(latency_lock, &mut flags);
    core::ptr::write_bytes(latency_record.as_mut_ptr(), 0, latency_record.len());
    raw_spin_unlock_irqrestore(latency_lock, flags);
}

unsafe fn account_global_scheduler_latency(tsk: *mut task_struct, lat: *mut latency_record) {
    let mut firstnonnull = MAXLR;
    if (*tsk).mm.is_null() { return; }
    for i in 0..MAXLR {
        let mut same = true;
        if latency_record[i].backtrace[0] == 0 {
            if firstnonnull > i { firstnonnull = i; }
            continue;
        }
        for q in 0..LT_BACKTRACEDEPTH {
            let record = (*lat).backtrace[q];
            if latency_record[i].backtrace[q] != record { same = false; break; }
            if record == 0 { break; }
        }
        if same {
            latency_record[i].count += 1;
            latency_record[i].time += (*lat).time;
            if (*lat).time > latency_record[i].max { latency_record[i].max = (*lat).time; }
            return;
        }
    }
    if firstnonnull >= MAXLR { return; }
    core::ptr::copy_nonoverlapping(lat, &mut latency_record[firstnonnull], 1);
}

#[no_mangle]
pub unsafe extern "C" fn __account_scheduler_latency(tsk: *mut task_struct, usecs: c_int, inter: c_int) {
    if inter != 0 && usecs > 5000 { return; }
    if usecs <= 0 { return; }
    let mut lat: latency_record = core::mem::zeroed();
    lat.count = 1; lat.time = usecs as c_ulong; lat.max = usecs as c_ulong;
    stack_trace_save_tsk(tsk, lat.backtrace.as_mut_ptr(), LT_BACKTRACEDEPTH, 0);
    let mut flags = 0;
    raw_spin_lock_irqsave(latency_lock, &mut flags);
    account_global_scheduler_latency(tsk, &mut lat);
    for i in 0..((*tsk).latency_record_count as usize) {
        let mylat = &mut (*tsk).latency_record[i];
        let mut same = true;
        for q in 0..LT_BACKTRACEDEPTH {
            let record = lat.backtrace[q];
            if mylat.backtrace[q] != record { same = false; break; }
            if record == 0 { break; }
        }
        if same {
            mylat.count += 1; mylat.time += lat.time;
            if lat.time > mylat.max { mylat.max = lat.time; }
            raw_spin_unlock_irqrestore(latency_lock, flags); return;
        }
    }
    if (*tsk).latency_record_count as usize >= LT_SAVECOUNT {
        raw_spin_unlock_irqrestore(latency_lock, flags); return;
    }
    let i = (*tsk).latency_record_count as usize;
    (*tsk).latency_record_count += 1;
    core::ptr::copy_nonoverlapping(&lat, &mut (*tsk).latency_record[i], 1);
    raw_spin_unlock_irqrestore(latency_lock, flags);
}

unsafe extern "C" fn lstats_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let version = b"Latency Top version : v0.1\0";
    seq_puts(m, version.as_ptr());
    for i in 0..MAXLR {
        let lr = &latency_record[i];
        if lr.backtrace[0] != 0 {
            // The original uses seq_printf("%i %lu %lu", ...), followed by
            // one "%ps" argument for each nonzero backtrace address.
            let fmt = b"%i %lu %lu\0";
            seq_printf(m, fmt.as_ptr(), lr.count, lr.time, lr.max);
            for q in 0..LT_BACKTRACEDEPTH {
                let bt = lr.backtrace[q];
                if bt == 0 { break; }
                let bt_fmt = b" %ps\0";
                seq_printf(m, bt_fmt.as_ptr(), bt as *mut c_void);
            }
            seq_puts(m, b"\n\0".as_ptr());
        }
    }
    0
}

unsafe extern "C" fn lstats_write(_file: *mut file, _buf: *const u8, count: usize,
                                    _offs: *mut i64) -> isize {
    clear_global_latency_tracing();
    count as isize
}

unsafe extern "C" fn lstats_open(_inode: *mut inode, filp: *mut file) -> c_int {
    single_open(filp, lstats_show, core::ptr::null_mut())
}

#[repr(C)]
struct proc_ops {
    proc_open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    proc_read: Option<unsafe extern "C" fn()>,
    proc_write: Option<unsafe extern "C" fn(*mut file, *const u8, usize, *mut i64) -> isize>,
    proc_lseek: Option<unsafe extern "C" fn()>,
    proc_release: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" fn init_lstats_procfs() -> c_int {
    static OPS: proc_ops = proc_ops {
        proc_open: Some(lstats_open),
        proc_read: Some(seq_read),
        proc_write: Some(lstats_write),
        proc_lseek: Some(seq_lseek),
        proc_release: Some(single_release),
    };
    proc_create(b"latency_stats\0".as_ptr(), 0o644, core::ptr::null_mut(),
                &OPS as *const proc_ops as *const c_void);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
