// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/perf_event_attr_fprintf.c.
// C include dependencies intentionally remain external to this translation unit.

use core::ffi::{c_char, c_int, c_void};
use core::mem::{offset_of, size_of, size_of_val};

type size_t = usize;
type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    pub name: *const c_char,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub read_format: u64,
    /* bitfield members share an offset; most are within PERF_ATTR_SIZE_VER0 */
    pub disabled: u64,
    pub inherit: u64,
    pub pinned: u64,
    pub exclusive: u64,
    pub exclude_user: u64,
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
    pub exclude_idle: u64,
    pub mmap: u64,
    pub comm: u64,
    pub freq: u64,
    pub inherit_stat: u64,
    pub enable_on_exec: u64,
    pub task: u64,
    pub watermark: u64,
    pub precise_ip: u64,
    pub mmap_data: u64,
    pub sample_id_all: u64,
    pub exclude_host: u64,
    pub exclude_guest: u64,
    pub exclude_callchain_kernel: u64,
    pub exclude_callchain_user: u64,
    pub mmap2: u64,
    pub comm_exec: u64,
    pub use_clockid: u64,
    pub context_switch: u64,
    pub write_backward: u64,
    pub namespaces: u64,
    pub ksymbol: u64,
    pub bpf_event: u64,
    pub aux_output: u64,
    pub cgroup: u64,
    pub text_poke: u64,
    pub build_id: u64,
    pub inherit_thread: u64,
    pub remove_on_exec: u64,
    pub sigtrap: u64,
    pub defer_callchain: u64,
    pub defer_output: u64,
    pub wakeup_events: u32,
    pub bp_type: u32,
    pub bp_addr: u64,
    pub bp_len: u64,
    pub branch_sample_type: u64,
    pub sample_regs_user: u64,
    pub sample_stack_user: u32,
    pub clockid: c_int,
    pub sample_regs_intr: u64,
    pub aux_watermark: u32,
    pub sample_max_stack: u16,
    pub aux_sample_size: u32,
    pub sig_data: u64,
    pub aux_start_paused: u64,
    pub aux_pause: u64,
    pub aux_resume: u64,
}

#[repr(C)]
struct bit_names {
    bit: c_int,
    name: *const c_char,
}

pub type attr__fprintf_f =
    unsafe extern "C" fn(*mut FILE, *const c_char, *const c_char, *mut c_void) -> c_int;

unsafe extern "C" {
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn tracepoint_id_to_name(id: u64) -> *mut c_char;
    fn perf_pmus__find_by_type(type_: u32) -> *mut perf_pmu;
    fn perf_pmu__name_from_config(pmu: *mut perf_pmu, config: u64) -> *const c_char;
}

unsafe fn __p_bits(mut buf: *mut c_char, size: size_t, value: u64, bits: *mut bit_names) {
    let mut first_bit = true;
    let mut i: c_int = 0;

    loop {
        if value & (*bits.add(i as usize)).bit as u64 != 0 {
            buf = buf.add(scnprintf(
                buf,
                size,
                c"%s%s".as_ptr(),
                if first_bit { c"".as_ptr() } else { c"|".as_ptr() },
                (*bits.add(i as usize)).name,
            ) as usize);
            first_bit = false;
        }
        i += 1;
        if (*bits.add(i as usize)).name.is_null() {
            break;
        }
    }
}

unsafe fn __p_sample_type(buf: *mut c_char, size: size_t, value: u64) {
    let mut bits = [
        bit_names { bit: PERF_SAMPLE_IP as c_int, name: c"IP".as_ptr() },
        bit_names { bit: PERF_SAMPLE_TID as c_int, name: c"TID".as_ptr() },
        bit_names { bit: PERF_SAMPLE_TIME as c_int, name: c"TIME".as_ptr() },
        bit_names { bit: PERF_SAMPLE_ADDR as c_int, name: c"ADDR".as_ptr() },
        bit_names { bit: PERF_SAMPLE_READ as c_int, name: c"READ".as_ptr() },
        bit_names { bit: PERF_SAMPLE_CALLCHAIN as c_int, name: c"CALLCHAIN".as_ptr() },
        bit_names { bit: PERF_SAMPLE_ID as c_int, name: c"ID".as_ptr() },
        bit_names { bit: PERF_SAMPLE_CPU as c_int, name: c"CPU".as_ptr() },
        bit_names { bit: PERF_SAMPLE_PERIOD as c_int, name: c"PERIOD".as_ptr() },
        bit_names { bit: PERF_SAMPLE_STREAM_ID as c_int, name: c"STREAM_ID".as_ptr() },
        bit_names { bit: PERF_SAMPLE_RAW as c_int, name: c"RAW".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_STACK as c_int, name: c"BRANCH_STACK".as_ptr() },
        bit_names { bit: PERF_SAMPLE_REGS_USER as c_int, name: c"REGS_USER".as_ptr() },
        bit_names { bit: PERF_SAMPLE_STACK_USER as c_int, name: c"STACK_USER".as_ptr() },
        bit_names { bit: PERF_SAMPLE_IDENTIFIER as c_int, name: c"IDENTIFIER".as_ptr() },
        bit_names { bit: PERF_SAMPLE_REGS_INTR as c_int, name: c"REGS_INTR".as_ptr() },
        bit_names { bit: PERF_SAMPLE_DATA_SRC as c_int, name: c"DATA_SRC".as_ptr() },
        bit_names { bit: PERF_SAMPLE_WEIGHT as c_int, name: c"WEIGHT".as_ptr() },
        bit_names { bit: PERF_SAMPLE_PHYS_ADDR as c_int, name: c"PHYS_ADDR".as_ptr() },
        bit_names { bit: PERF_SAMPLE_AUX as c_int, name: c"AUX".as_ptr() },
        bit_names { bit: PERF_SAMPLE_CGROUP as c_int, name: c"CGROUP".as_ptr() },
        bit_names { bit: PERF_SAMPLE_DATA_PAGE_SIZE as c_int, name: c"DATA_PAGE_SIZE".as_ptr() },
        bit_names { bit: PERF_SAMPLE_CODE_PAGE_SIZE as c_int, name: c"CODE_PAGE_SIZE".as_ptr() },
        bit_names { bit: PERF_SAMPLE_WEIGHT_STRUCT as c_int, name: c"WEIGHT_STRUCT".as_ptr() },
        bit_names { bit: 0, name: core::ptr::null() },
    ];
    __p_bits(buf, size, value, bits.as_mut_ptr());
}

unsafe fn __p_branch_sample_type(buf: *mut c_char, size: size_t, value: u64) {
    let mut bits = [
        bit_names { bit: PERF_SAMPLE_BRANCH_USER as c_int, name: c"USER".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_KERNEL as c_int, name: c"KERNEL".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_HV as c_int, name: c"HV".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_ANY as c_int, name: c"ANY".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_ANY_CALL as c_int, name: c"ANY_CALL".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_ANY_RETURN as c_int, name: c"ANY_RETURN".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_IND_CALL as c_int, name: c"IND_CALL".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_ABORT_TX as c_int, name: c"ABORT_TX".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_IN_TX as c_int, name: c"IN_TX".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_NO_TX as c_int, name: c"NO_TX".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_COND as c_int, name: c"COND".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_CALL_STACK as c_int, name: c"CALL_STACK".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_IND_JUMP as c_int, name: c"IND_JUMP".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_CALL as c_int, name: c"CALL".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_NO_FLAGS as c_int, name: c"NO_FLAGS".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_NO_CYCLES as c_int, name: c"NO_CYCLES".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_TYPE_SAVE as c_int, name: c"TYPE_SAVE".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_HW_INDEX as c_int, name: c"HW_INDEX".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_PRIV_SAVE as c_int, name: c"PRIV_SAVE".as_ptr() },
        bit_names { bit: PERF_SAMPLE_BRANCH_COUNTERS as c_int, name: c"COUNTERS".as_ptr() },
        bit_names { bit: 0, name: core::ptr::null() },
    ];
    __p_bits(buf, size, value, bits.as_mut_ptr());
}

unsafe fn __p_read_format(buf: *mut c_char, size: size_t, value: u64) {
    let mut bits = [
        bit_names { bit: PERF_FORMAT_TOTAL_TIME_ENABLED as c_int, name: c"TOTAL_TIME_ENABLED".as_ptr() },
        bit_names { bit: PERF_FORMAT_TOTAL_TIME_RUNNING as c_int, name: c"TOTAL_TIME_RUNNING".as_ptr() },
        bit_names { bit: PERF_FORMAT_ID as c_int, name: c"ID".as_ptr() },
        bit_names { bit: PERF_FORMAT_GROUP as c_int, name: c"GROUP".as_ptr() },
        bit_names { bit: PERF_FORMAT_LOST as c_int, name: c"LOST".as_ptr() },
        bit_names { bit: 0, name: core::ptr::null() },
    ];
    __p_bits(buf, size, value, bits.as_mut_ptr());
}

unsafe fn stringify_perf_type_id(pmu: *mut perf_pmu, type_: u32) -> *const c_char {
    match type_ {
        PERF_TYPE_HARDWARE => c"PERF_TYPE_HARDWARE".as_ptr(),
        PERF_TYPE_SOFTWARE => c"PERF_TYPE_SOFTWARE".as_ptr(),
        PERF_TYPE_TRACEPOINT => c"PERF_TYPE_TRACEPOINT".as_ptr(),
        PERF_TYPE_HW_CACHE => c"PERF_TYPE_HW_CACHE".as_ptr(),
        PERF_TYPE_BREAKPOINT => c"PERF_TYPE_BREAKPOINT".as_ptr(),
        PERF_TYPE_RAW => {
            if !pmu.is_null() { (*pmu).name } else { c"PERF_TYPE_RAW".as_ptr() }
        }
        _ => {
            if !pmu.is_null() { (*pmu).name } else { core::ptr::null() }
        }
    }
}

fn stringify_perf_hw_id(value: u64) -> *const c_char {
    match value & PERF_HW_EVENT_MASK {
        PERF_COUNT_HW_CPU_CYCLES => c"PERF_COUNT_HW_CPU_CYCLES".as_ptr(),
        PERF_COUNT_HW_INSTRUCTIONS => c"PERF_COUNT_HW_INSTRUCTIONS".as_ptr(),
        PERF_COUNT_HW_CACHE_REFERENCES => c"PERF_COUNT_HW_CACHE_REFERENCES".as_ptr(),
        PERF_COUNT_HW_CACHE_MISSES => c"PERF_COUNT_HW_CACHE_MISSES".as_ptr(),
        PERF_COUNT_HW_BRANCH_INSTRUCTIONS => c"PERF_COUNT_HW_BRANCH_INSTRUCTIONS".as_ptr(),
        PERF_COUNT_HW_BRANCH_MISSES => c"PERF_COUNT_HW_BRANCH_MISSES".as_ptr(),
        PERF_COUNT_HW_BUS_CYCLES => c"PERF_COUNT_HW_BUS_CYCLES".as_ptr(),
        PERF_COUNT_HW_STALLED_CYCLES_FRONTEND => c"PERF_COUNT_HW_STALLED_CYCLES_FRONTEND".as_ptr(),
        PERF_COUNT_HW_STALLED_CYCLES_BACKEND => c"PERF_COUNT_HW_STALLED_CYCLES_BACKEND".as_ptr(),
        PERF_COUNT_HW_REF_CPU_CYCLES => c"PERF_COUNT_HW_REF_CPU_CYCLES".as_ptr(),
        _ => core::ptr::null(),
    }
}

fn stringify_perf_hw_cache_id(value: u64) -> *const c_char {
    match value {
        PERF_COUNT_HW_CACHE_L1D => c"PERF_COUNT_HW_CACHE_L1D".as_ptr(),
        PERF_COUNT_HW_CACHE_L1I => c"PERF_COUNT_HW_CACHE_L1I".as_ptr(),
        PERF_COUNT_HW_CACHE_LL => c"PERF_COUNT_HW_CACHE_LL".as_ptr(),
        PERF_COUNT_HW_CACHE_DTLB => c"PERF_COUNT_HW_CACHE_DTLB".as_ptr(),
        PERF_COUNT_HW_CACHE_ITLB => c"PERF_COUNT_HW_CACHE_ITLB".as_ptr(),
        PERF_COUNT_HW_CACHE_BPU => c"PERF_COUNT_HW_CACHE_BPU".as_ptr(),
        PERF_COUNT_HW_CACHE_NODE => c"PERF_COUNT_HW_CACHE_NODE".as_ptr(),
        _ => core::ptr::null(),
    }
}

fn stringify_perf_hw_cache_op_id(value: u64) -> *const c_char {
    match value {
        PERF_COUNT_HW_CACHE_OP_READ => c"PERF_COUNT_HW_CACHE_OP_READ".as_ptr(),
        PERF_COUNT_HW_CACHE_OP_WRITE => c"PERF_COUNT_HW_CACHE_OP_WRITE".as_ptr(),
        PERF_COUNT_HW_CACHE_OP_PREFETCH => c"PERF_COUNT_HW_CACHE_OP_PREFETCH".as_ptr(),
        _ => core::ptr::null(),
    }
}

fn stringify_perf_hw_cache_op_result_id(value: u64) -> *const c_char {
    match value {
        PERF_COUNT_HW_CACHE_RESULT_ACCESS => c"PERF_COUNT_HW_CACHE_RESULT_ACCESS".as_ptr(),
        PERF_COUNT_HW_CACHE_RESULT_MISS => c"PERF_COUNT_HW_CACHE_RESULT_MISS".as_ptr(),
        _ => core::ptr::null(),
    }
}

fn stringify_perf_sw_id(value: u64) -> *const c_char {
    match value {
        PERF_COUNT_SW_CPU_CLOCK => c"PERF_COUNT_SW_CPU_CLOCK".as_ptr(),
        PERF_COUNT_SW_TASK_CLOCK => c"PERF_COUNT_SW_TASK_CLOCK".as_ptr(),
        PERF_COUNT_SW_PAGE_FAULTS => c"PERF_COUNT_SW_PAGE_FAULTS".as_ptr(),
        PERF_COUNT_SW_CONTEXT_SWITCHES => c"PERF_COUNT_SW_CONTEXT_SWITCHES".as_ptr(),
        PERF_COUNT_SW_CPU_MIGRATIONS => c"PERF_COUNT_SW_CPU_MIGRATIONS".as_ptr(),
        PERF_COUNT_SW_PAGE_FAULTS_MIN => c"PERF_COUNT_SW_PAGE_FAULTS_MIN".as_ptr(),
        PERF_COUNT_SW_PAGE_FAULTS_MAJ => c"PERF_COUNT_SW_PAGE_FAULTS_MAJ".as_ptr(),
        PERF_COUNT_SW_ALIGNMENT_FAULTS => c"PERF_COUNT_SW_ALIGNMENT_FAULTS".as_ptr(),
        PERF_COUNT_SW_EMULATION_FAULTS => c"PERF_COUNT_SW_EMULATION_FAULTS".as_ptr(),
        PERF_COUNT_SW_DUMMY => c"PERF_COUNT_SW_DUMMY".as_ptr(),
        PERF_COUNT_SW_BPF_OUTPUT => c"PERF_COUNT_SW_BPF_OUTPUT".as_ptr(),
        PERF_COUNT_SW_CGROUP_SWITCHES => c"PERF_COUNT_SW_CGROUP_SWITCHES".as_ptr(),
        _ => core::ptr::null(),
    }
}

unsafe fn print_id_unsigned(buf: *mut c_char, size: size_t, value: u64, s: *const c_char) {
    if s.is_null() {
        snprintf(buf, size, c"%llu".as_ptr(), value);
    } else {
        snprintf(buf, size, c"%llu (%s)".as_ptr(), value, s);
    }
}

unsafe fn print_id_hex(buf: *mut c_char, size: size_t, value: u64, s: *const c_char) {
    if s.is_null() {
        snprintf(buf, size, c"%#llx".as_ptr(), value);
    } else {
        snprintf(buf, size, c"%#llx (%s)".as_ptr(), value, s);
    }
}

unsafe fn __p_type_id(buf: *mut c_char, size: size_t, pmu: *mut perf_pmu, type_: u32) {
    print_id_unsigned(buf, size, type_ as u64, stringify_perf_type_id(pmu, type_));
}

unsafe fn __p_config_hw_id(buf: *mut c_char, size: size_t, pmu: *mut perf_pmu, config: u64) {
    let name = stringify_perf_hw_id(config);

    if name.is_null() {
        if pmu.is_null() {
            snprintf(buf, size, c"%#llx".as_ptr(), config);
        } else {
            snprintf(buf, size, c"%#llx (%s/config=%#llx/)".as_ptr(), config, (*pmu).name, config);
        }
    } else if pmu.is_null() {
        snprintf(buf, size, c"%#llx (%s)".as_ptr(), config, name);
    } else {
        snprintf(buf, size, c"%#llx (%s/%s/)".as_ptr(), config, (*pmu).name, name);
    }
}

unsafe fn __p_config_sw_id(buf: *mut c_char, size: size_t, id: u64) {
    print_id_hex(buf, size, id, stringify_perf_sw_id(id));
}

unsafe fn __p_config_hw_cache_id(buf: *mut c_char, size: size_t, pmu: *mut perf_pmu, config: u64) {
    let hw_cache_str = stringify_perf_hw_cache_id(config & 0xff);
    let hw_cache_op_str = stringify_perf_hw_cache_op_id((config & 0xff00) >> 8);
    let hw_cache_op_result_str = stringify_perf_hw_cache_op_result_id((config & 0xff0000) >> 16);

    if hw_cache_str.is_null() || hw_cache_op_str.is_null() || hw_cache_op_result_str.is_null() {
        if pmu.is_null() {
            snprintf(buf, size, c"%#llx".as_ptr(), config);
        } else {
            snprintf(buf, size, c"%#llx (%s/config=%#llx/)".as_ptr(), config, (*pmu).name, config);
        }
    } else if pmu.is_null() {
        snprintf(
            buf,
            size,
            c"%#llx (%s | %s | %s)".as_ptr(),
            config,
            hw_cache_op_result_str,
            hw_cache_op_str,
            hw_cache_str,
        );
    } else {
        snprintf(
            buf,
            size,
            c"%#llx (%s/%s | %s | %s/)".as_ptr(),
            config,
            (*pmu).name,
            hw_cache_op_result_str,
            hw_cache_op_str,
            hw_cache_str,
        );
    }
}

unsafe fn __p_config_tracepoint_id(buf: *mut c_char, size: size_t, id: u64) {
    let str_ = tracepoint_id_to_name(id);

    print_id_hex(buf, size, id, str_);
    free(str_ as *mut c_void);
}

unsafe fn __p_config_id(pmu: *mut perf_pmu, buf: *mut c_char, size: size_t, type_: u32, config: u64) {
    match type_ {
        PERF_TYPE_HARDWARE => __p_config_hw_id(buf, size, pmu, config),
        PERF_TYPE_SOFTWARE => __p_config_sw_id(buf, size, config),
        PERF_TYPE_HW_CACHE => __p_config_hw_cache_id(buf, size, pmu, config),
        PERF_TYPE_TRACEPOINT => __p_config_tracepoint_id(buf, size, config),
        PERF_TYPE_RAW | PERF_TYPE_BREAKPOINT | _ => {
            print_id_hex(buf, size, config, perf_pmu__name_from_config(pmu, config))
        }
    }
}

const BUF_SIZE: usize = 1024;

macro_rules! p_hex {
    ($buf:expr, $val:expr) => {
        unsafe { snprintf($buf.as_mut_ptr(), BUF_SIZE, c"%#llx".as_ptr(), $val as u64) }
    };
}

macro_rules! p_unsigned {
    ($buf:expr, $val:expr) => {
        unsafe { snprintf($buf.as_mut_ptr(), BUF_SIZE, c"%llu".as_ptr(), $val as u64) }
    };
}

macro_rules! p_signed {
    ($buf:expr, $val:expr) => {
        unsafe { snprintf($buf.as_mut_ptr(), BUF_SIZE, c"%lld".as_ptr(), $val as i64) }
    };
}

macro_rules! p_sample_type {
    ($buf:expr, $val:expr) => {
        unsafe { __p_sample_type($buf.as_mut_ptr(), BUF_SIZE, $val as u64) }
    };
}

macro_rules! p_branch_sample_type {
    ($buf:expr, $val:expr) => {
        unsafe { __p_branch_sample_type($buf.as_mut_ptr(), BUF_SIZE, $val as u64) }
    };
}

macro_rules! p_read_format {
    ($buf:expr, $val:expr) => {
        unsafe { __p_read_format($buf.as_mut_ptr(), BUF_SIZE, $val as u64) }
    };
}

macro_rules! p_type_id {
    ($buf:expr, $pmu:expr, $val:expr) => {
        unsafe { __p_type_id($buf.as_mut_ptr(), BUF_SIZE, $pmu, $val as u32) }
    };
}

macro_rules! p_config_id {
    ($buf:expr, $pmu:expr, $attr:expr, $val:expr) => {
        unsafe { __p_config_id($pmu, $buf.as_mut_ptr(), BUF_SIZE, (*$attr).type_, $val as u64) }
    };
}

macro_rules! PRINT_ATTRn {
    ($ret:ident, $fp:expr, $attr:expr, $attr_size:expr, $attr__fprintf:expr, $priv:expr, $buf:expr, $n:expr, $f:ident, $p:ident, $always:expr) => {{
        if $attr_size >= offset_of!(perf_event_attr, $f) + size_of_val(&(*$attr).$f)
            && ($always || (*$attr).$f != 0)
        {
            $p!($buf, (*$attr).$f);
            $ret += $attr__fprintf($fp, $n.as_ptr() as *const c_char, $buf.as_ptr(), $priv);
        }
    }};
}

macro_rules! PRINT_ATTRn_TYPE {
    ($ret:ident, $fp:expr, $attr:expr, $attr_size:expr, $attr__fprintf:expr, $priv:expr, $buf:expr, $n:expr, $f:ident, $always:expr, $pmu:expr) => {{
        if $attr_size >= offset_of!(perf_event_attr, $f) + size_of_val(&(*$attr).$f)
            && ($always || (*$attr).$f != 0)
        {
            p_type_id!($buf, $pmu, (*$attr).$f);
            $ret += $attr__fprintf($fp, $n.as_ptr() as *const c_char, $buf.as_ptr(), $priv);
        }
    }};
}

macro_rules! PRINT_ATTRn_CONFIG {
    ($ret:ident, $fp:expr, $attr:expr, $attr_size:expr, $attr__fprintf:expr, $priv:expr, $buf:expr, $n:expr, $f:ident, $always:expr, $pmu:expr) => {{
        if $attr_size >= offset_of!(perf_event_attr, $f) + size_of_val(&(*$attr).$f)
            && ($always || (*$attr).$f != 0)
        {
            p_config_id!($buf, $pmu, $attr, (*$attr).$f);
            $ret += $attr__fprintf($fp, $n.as_ptr() as *const c_char, $buf.as_ptr(), $priv);
        }
    }};
}

macro_rules! PRINT_ATTRn_bf {
    ($ret:ident, $fp:expr, $attr:expr, $attr__fprintf:expr, $priv:expr, $buf:expr, $n:expr, $f:ident, $p:ident, $always:expr) => {{
        if $always || (*$attr).$f != 0 {
            $p!($buf, (*$attr).$f);
            $ret += $attr__fprintf($fp, $n.as_ptr() as *const c_char, $buf.as_ptr(), $priv);
        }
    }};
}

macro_rules! PRINT_ATTRf {
    ($ret:ident, $fp:expr, $attr:expr, $attr_size:expr, $attr__fprintf:expr, $priv:expr, $buf:expr, $f:ident, $p:ident) => {
        PRINT_ATTRn!($ret, $fp, $attr, $attr_size, $attr__fprintf, $priv, $buf, concat!(stringify!($f), "\0"), $f, $p, false)
    };
}

macro_rules! PRINT_ATTRf_bf {
    ($ret:ident, $fp:expr, $attr:expr, $attr__fprintf:expr, $priv:expr, $buf:expr, $f:ident, $p:ident) => {
        PRINT_ATTRn_bf!($ret, $fp, $attr, $attr__fprintf, $priv, $buf, concat!(stringify!($f), "\0"), $f, $p, false)
    };
}

#[no_mangle]
pub unsafe extern "C" fn perf_event_attr__fprintf(
    fp: *mut FILE,
    attr: *mut perf_event_attr,
    attr__fprintf: attr__fprintf_f,
    priv_: *mut c_void,
) -> c_int {
    let mut pmu = perf_pmus__find_by_type((*attr).type_);
    /*
     * size == 0 means ABI0 - the producer didn't set attr.size.
     * perf_event__fprintf_attr() may pass the raw mmap'd event
     * before the local copy, so default to PERF_ATTR_SIZE_VER0
     * (the ABI0 footprint) to avoid reading past the attr into
     * the ID array that follows it in HEADER_ATTR events.
     */
    let mut attr_size: u32 = if (*attr).size != 0 { (*attr).size } else { PERF_ATTR_SIZE_VER0 };
    let mut buf = [0 as c_char; BUF_SIZE];
    let mut ret: c_int = 0;

    /*
     * Cap to what we understand: all callers store the attr in a
     * buffer of sizeof(*attr) bytes (perf.data read path copies
     * min(attr.size, sizeof), BPF augmented path copies into a
     * fixed-size value[] array).  A spoofed attr->size larger
     * than sizeof would cause PRINT_ATTRn to read past the
     * actual buffer.
     */
    if attr_size as usize > size_of::<perf_event_attr>() {
        attr_size = size_of::<perf_event_attr>() as u32;
    }

    if pmu.is_null()
        && attr_size as usize >= offset_of!(perf_event_attr, config) + size_of_val(&(*attr).config)
        && ((*attr).type_ == PERF_TYPE_HARDWARE || (*attr).type_ == PERF_TYPE_HW_CACHE)
    {
        let extended_type: u32 = ((*attr).config >> PERF_PMU_TYPE_SHIFT) as u32;

        if extended_type != 0 {
            pmu = perf_pmus__find_by_type(extended_type);
        }
    }

    PRINT_ATTRn_TYPE!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, c"type", type_, true, pmu);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, size, p_unsigned);
    PRINT_ATTRn_CONFIG!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, c"config", config, true, pmu);
    PRINT_ATTRn!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, c"{ sample_period, sample_freq }", sample_period, p_unsigned, false);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, sample_type, p_sample_type);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, read_format, p_read_format);

    /*
     * All bitfields share a single __u64 right after read_format.
     * BPF-captured attrs from perf trace may have a small size
     * when the tracee passes a minimal struct, so skip the
     * entire block when it's not covered.
     */
    if attr_size as usize >= offset_of!(perf_event_attr, wakeup_events) {
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, disabled, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, inherit, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, pinned, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, exclusive, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, exclude_user, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, exclude_kernel, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, exclude_hv, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, exclude_idle, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, mmap, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, comm, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, freq, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, inherit_stat, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, enable_on_exec, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, task, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, watermark, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, precise_ip, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, mmap_data, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, sample_id_all, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, exclude_host, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, exclude_guest, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, exclude_callchain_kernel, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, exclude_callchain_user, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, mmap2, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, comm_exec, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, use_clockid, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, context_switch, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, write_backward, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, namespaces, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, ksymbol, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, bpf_event, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, aux_output, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, cgroup, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, text_poke, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, build_id, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, inherit_thread, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, remove_on_exec, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, sigtrap, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, defer_callchain, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, defer_output, p_unsigned);
    }

    PRINT_ATTRn!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, c"{ wakeup_events, wakeup_watermark }", wakeup_events, p_unsigned, false);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, bp_type, p_unsigned);
    PRINT_ATTRn!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, c"{ bp_addr, config1 }", bp_addr, p_hex, false);
    PRINT_ATTRn!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, c"{ bp_len, config2 }", bp_len, p_hex, false);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, branch_sample_type, p_branch_sample_type);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, sample_regs_user, p_hex);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, sample_stack_user, p_unsigned);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, clockid, p_signed);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, sample_regs_intr, p_hex);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, aux_watermark, p_unsigned);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, sample_max_stack, p_unsigned);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, aux_sample_size, p_unsigned);
    PRINT_ATTRf!(ret, fp, attr, attr_size as usize, attr__fprintf, priv_, buf, sig_data, p_unsigned);
    /* aux_{start_paused,pause,resume} are at byte 116, past VER0 */
    if attr_size as usize >= offset_of!(perf_event_attr, sig_data) {
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, aux_start_paused, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, aux_pause, p_unsigned);
        PRINT_ATTRf_bf!(ret, fp, attr, attr__fprintf, priv_, buf, aux_resume, p_unsigned);
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
