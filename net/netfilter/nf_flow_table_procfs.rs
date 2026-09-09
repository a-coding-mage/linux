// SPDX-License-Identifier: GPL-2.0-only
// Translated from nf_flow_table_procfs.c. Kernel declarations are supplied by
// the surrounding build and are intentionally not implemented here.

use core::ffi::c_void;

pub type LoffT = i64;

#[repr(C)]
pub struct SeqFile {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ProcDirEntry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct NfFlowTableStat {
    pub count_wq_add: i32,
    pub count_wq_del: i32,
    pub count_wq_stats: i32,
}

#[repr(C)]
pub struct NetFt {
    pub stat: *mut NfFlowTableStat,
}

#[repr(C)]
pub struct Net {
    pub ft: NetFt,
    pub proc_net_stat: *mut ProcDirEntry,
}

pub const SEQ_START_TOKEN: *mut c_void = 1usize as *mut c_void;
pub const NR_CPU_IDS: i32 = 0; // supplied by the kernel build configuration

#[repr(C)]
pub struct SeqOperations {
    pub start: Option<unsafe extern "C" fn(*mut SeqFile, *mut LoffT) -> *mut c_void>,
    pub next: Option<unsafe extern "C" fn(*mut SeqFile, *mut c_void, *mut LoffT) -> *mut c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut SeqFile, *mut c_void)>,
    pub show: Option<unsafe extern "C" fn(*mut SeqFile, *mut c_void) -> i32>,
}

extern "C" {
    fn seq_file_net(seq: *mut SeqFile) -> *mut Net;
    fn cpu_possible(cpu: i32) -> bool;
    fn per_cpu_ptr(ptr: *mut NfFlowTableStat, cpu: i32) -> *mut NfFlowTableStat;
    fn seq_puts(seq: *mut SeqFile, text: *const u8);
    fn seq_printf(seq: *mut SeqFile, format: *const u8, ...);
    fn proc_create_net(
        name: *const u8,
        mode: u16,
        parent: *mut ProcDirEntry,
        ops: *const SeqOperations,
        size: usize,
    ) -> *mut ProcDirEntry;
    fn remove_proc_entry(name: *const u8, parent: *mut ProcDirEntry);
}

unsafe extern "C" fn nf_flow_table_cpu_seq_start(
    seq: *mut SeqFile,
    pos: *mut LoffT,
) -> *mut c_void {
    let net = seq_file_net(seq);
    let mut cpu: i32;

    if *pos == 0 {
        return SEQ_START_TOKEN;
    }

    cpu = (*pos - 1) as i32;
    while cpu < NR_CPU_IDS {
        if !cpu_possible(cpu) {
            cpu += 1;
            continue;
        }
        *pos = (cpu + 1) as LoffT;
        return per_cpu_ptr((*net).ft.stat, cpu) as *mut c_void;
    }

    core::ptr::null_mut()
}

unsafe extern "C" fn nf_flow_table_cpu_seq_next(
    seq: *mut SeqFile,
    _v: *mut c_void,
    pos: *mut LoffT,
) -> *mut c_void {
    let net = seq_file_net(seq);
    let mut cpu = *pos as i32;

    while cpu < NR_CPU_IDS {
        if !cpu_possible(cpu) {
            cpu += 1;
            continue;
        }
        *pos = (cpu + 1) as LoffT;
        return per_cpu_ptr((*net).ft.stat, cpu) as *mut c_void;
    }
    *pos += 1;
    core::ptr::null_mut()
}

unsafe extern "C" fn nf_flow_table_cpu_seq_stop(_seq: *mut SeqFile, _v: *mut c_void) {}

unsafe extern "C" fn nf_flow_table_cpu_seq_show(
    seq: *mut SeqFile,
    v: *mut c_void,
) -> i32 {
    if v == SEQ_START_TOKEN {
        seq_puts(seq, b"wq_add   wq_del   wq_stats\n\0".as_ptr());
        return 0;
    }

    let st = v as *const NfFlowTableStat;
    seq_printf(
        seq,
        b"%8d %8d %8d\n\0".as_ptr(),
        (*st).count_wq_add,
        (*st).count_wq_del,
        (*st).count_wq_stats,
    );
    0
}

static NF_FLOW_TABLE_CPU_SEQ_OPS: SeqOperations = SeqOperations {
    start: Some(nf_flow_table_cpu_seq_start),
    next: Some(nf_flow_table_cpu_seq_next),
    stop: Some(nf_flow_table_cpu_seq_stop),
    show: Some(nf_flow_table_cpu_seq_show),
};

#[no_mangle]
pub unsafe extern "C" fn nf_flow_table_init_proc(net: *mut Net) -> i32 {
    let pde = proc_create_net(
        b"nf_flowtable\0".as_ptr(),
        0o444,
        (*net).proc_net_stat,
        &NF_FLOW_TABLE_CPU_SEQ_OPS,
        core::mem::size_of::<SeqNetPrivate>(),
    );
    if !pde.is_null() { 0 } else { -12 }
}

#[repr(C)]
pub struct SeqNetPrivate {
    _private: [u8; 0],
}

#[no_mangle]
pub unsafe extern "C" fn nf_flow_table_fini_proc(net: *mut Net) {
    remove_proc_entry(b"nf_flowtable\0".as_ptr(), (*net).proc_net_stat);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
