// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/kvm-stat-arch/kvm-stat-powerpc.c.
// C include dependencies are represented here as external declarations.

use core::ffi::{c_char, c_int, c_ulonglong};

const NR_TPS: usize = 4;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

extern "C" {
    static kvm_trace_symbol_exit: exit_reasons_table;
    static kvm_trace_symbol_hcall: exit_reasons_table;

    /*
     * C used:
     * define_exit_reasons_table(hv_exit_reasons, kvm_trace_symbol_exit);
     * define_exit_reasons_table(hcall_reasons, kvm_trace_symbol_hcall);
     */
    static hv_exit_reasons: exit_reasons_table;
    static hcall_reasons: exit_reasons_table;

    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> u64;
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);

    fn exit_event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool;
    fn exit_event_end(sample: *mut perf_sample, key: *mut event_key) -> bool;
    fn exit_event_decode_key(
        kvm: *mut perf_kvm_stat,
        key: *mut event_key,
        decode: *mut c_char,
    );

    fn parse_events_error__init(err: *mut parse_events_error);
    fn parse_events(evlist: *mut evlist, str_: *const c_char, err: *mut parse_events_error)
        -> c_int;
    fn parse_events_error__print(err: *mut parse_events_error, str_: *const c_char);
    fn parse_events_error__exit(err: *mut parse_events_error);
    fn evlist__new() -> *mut evlist;
    fn perf_pmus__have_event(pmu: *const c_char, name: *const c_char) -> bool;
}

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
}

#[repr(C)]
pub struct event_key {
    pub key: u64,
    pub info: u64,
}

#[repr(C)]
pub struct exit_reasons_table {
    pub exit_code: u64,
    pub reason: *const c_char,
}

#[repr(C)]
pub struct perf_kvm_stat {
    pub exit_reasons: *const exit_reasons_table,
    pub exit_reasons_isa: *const c_char,
}

#[repr(C)]
pub struct kvm_events_ops {
    pub is_begin_event: Option<unsafe extern "C" fn(*mut perf_sample, *mut event_key) -> bool>,
    pub is_end_event: Option<unsafe extern "C" fn(*mut perf_sample, *mut event_key) -> bool>,
    pub decode_key: Option<unsafe extern "C" fn(*mut perf_kvm_stat, *mut event_key, *mut c_char)>,
    pub name: *const c_char,
}

#[repr(C)]
pub struct kvm_reg_events_ops {
    pub name: *const c_char,
    pub ops: *const kvm_events_ops,
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

const KVM_EVENT_NAME_LEN: usize = 256;

/* Tracepoints specific to ppc_book3s_hv */
static PPC_BOOK3S_HV_KVM_TP_0: &[u8] = b"kvm_hv:kvm_guest_enter\0";
static PPC_BOOK3S_HV_KVM_TP_1: &[u8] = b"kvm_hv:kvm_guest_exit\0";
static PPC_BOOK3S_HV_KVM_TP_2: &[u8] = b"kvm_hv:kvm_hcall_enter\0";
static PPC_BOOK3S_HV_KVM_TP_3: &[u8] = b"kvm_hv:kvm_hcall_exit\0";

static ppc_book3s_hv_kvm_tp: [*const c_char; NR_TPS + 1] = [
    PPC_BOOK3S_HV_KVM_TP_0.as_ptr() as *const c_char,
    PPC_BOOK3S_HV_KVM_TP_1.as_ptr() as *const c_char,
    PPC_BOOK3S_HV_KVM_TP_2.as_ptr() as *const c_char,
    PPC_BOOK3S_HV_KVM_TP_3.as_ptr() as *const c_char,
    core::ptr::null(),
];

/* 1 extra placeholder for NULL */
static mut __kvm_events_tp: [*const c_char; NR_TPS + 1] = [core::ptr::null(); NR_TPS + 1];

unsafe extern "C" fn hcall_event_get_key(sample: *mut perf_sample, key: *mut event_key) {
    (*key).info = 0;
    (*key).key = perf_sample__intval(sample, b"req\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn get_hcall_exit_reason(exit_code: u64) -> *const c_char {
    let mut tbl = &hcall_reasons as *const exit_reasons_table;

    while !(*tbl).reason.is_null() {
        if (*tbl).exit_code == exit_code {
            return (*tbl).reason;
        }
        tbl = tbl.add(1);
    }

    pr_debug(
        b"Unknown hcall code: %lld\n\0".as_ptr() as *const c_char,
        exit_code as c_ulonglong,
    );
    b"UNKNOWN\0".as_ptr() as *const c_char
}

unsafe extern "C" fn hcall_event_end(
    sample: *mut perf_sample,
    _key: *mut event_key,
) -> bool {
    evsel__name_is((*sample).evsel, __kvm_events_tp[3])
}

unsafe extern "C" fn hcall_event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool {
    if evsel__name_is((*sample).evsel, __kvm_events_tp[2]) {
        hcall_event_get_key(sample, key);
        return true;
    }

    false
}

unsafe extern "C" fn hcall_event_decode_key(
    _kvm: *mut perf_kvm_stat,
    key: *mut event_key,
    decode: *mut c_char,
) {
    let hcall_reason = get_hcall_exit_reason((*key).key);

    scnprintf(
        decode,
        KVM_EVENT_NAME_LEN,
        b"%s\0".as_ptr() as *const c_char,
        hcall_reason,
    );
}

static hcall_events: kvm_events_ops = kvm_events_ops {
    is_begin_event: Some(hcall_event_begin),
    is_end_event: Some(hcall_event_end),
    decode_key: Some(hcall_event_decode_key),
    name: b"HCALL-EVENT\0".as_ptr() as *const c_char,
};

static exit_events: kvm_events_ops = kvm_events_ops {
    is_begin_event: Some(exit_event_begin),
    is_end_event: Some(exit_event_end),
    decode_key: Some(exit_event_decode_key),
    name: b"VM-EXIT\0".as_ptr() as *const c_char,
};

static __kvm_reg_events_ops: [kvm_reg_events_ops; 3] = [
    kvm_reg_events_ops {
        name: b"vmexit\0".as_ptr() as *const c_char,
        ops: &exit_events as *const kvm_events_ops,
    },
    kvm_reg_events_ops {
        name: b"hcall\0".as_ptr() as *const c_char,
        ops: &hcall_events as *const kvm_events_ops,
    },
    kvm_reg_events_ops {
        name: core::ptr::null(),
        ops: core::ptr::null(),
    },
];

static __kvm_skip_events: [*const c_char; 1] = [core::ptr::null()];

unsafe extern "C" fn is_tracepoint_available(str_: *const c_char, evlist: *mut evlist) -> c_int {
    let mut err: parse_events_error = core::mem::zeroed();
    let ret: c_int;

    parse_events_error__init(&mut err);
    ret = parse_events(evlist, str_, &mut err);
    if ret != 0 {
        parse_events_error__print(&mut err, b"tracepoint\0".as_ptr() as *const c_char);
    }
    parse_events_error__exit(&mut err);
    ret
}

unsafe extern "C" fn ppc__setup_book3s_hv(
    kvm: *mut perf_kvm_stat,
    evlist: *mut evlist,
) -> c_int {
    let mut events_ptr = ppc_book3s_hv_kvm_tp.as_ptr();
    let mut i: c_int;
    let mut nr_tp: c_int = 0;
    let mut err: c_int = -1;

    /* Check for book3s_hv tracepoints */
    while !(*events_ptr).is_null() {
        err = is_tracepoint_available(*events_ptr, evlist);
        if err != 0 {
            return -1;
        }
        nr_tp += 1;
        events_ptr = events_ptr.add(1);
    }

    i = 0;
    while i < nr_tp {
        __kvm_events_tp[i as usize] = ppc_book3s_hv_kvm_tp[i as usize];
        i += 1;
    }

    __kvm_events_tp[i as usize] = core::ptr::null();
    (*kvm).exit_reasons = &hv_exit_reasons as *const exit_reasons_table;
    (*kvm).exit_reasons_isa = b"HV\0".as_ptr() as *const c_char;

    0
}

/* Wrapper to setup kvm tracepoints */
unsafe extern "C" fn ppc__setup_kvm_tp(kvm: *mut perf_kvm_stat) -> c_int {
    let evlist = evlist__new();

    if evlist.is_null() {
        return -ENOMEM;
    }

    /* Right now, only supported on book3s_hv */
    ppc__setup_book3s_hv(kvm, evlist)
}

#[no_mangle]
pub unsafe extern "C" fn __setup_kvm_events_tp_powerpc(kvm: *mut perf_kvm_stat) -> c_int {
    ppc__setup_kvm_tp(kvm)
}

#[no_mangle]
pub unsafe extern "C" fn __cpu_isa_init_powerpc(kvm: *mut perf_kvm_stat) -> c_int {
    let ret: c_int;

    ret = ppc__setup_kvm_tp(kvm);
    if ret != 0 {
        (*kvm).exit_reasons = core::ptr::null();
        (*kvm).exit_reasons_isa = core::ptr::null();
    }

    ret
}

/*
 * In case of powerpc architecture, pmu registers are programmable
 * by guest kernel. So monitoring guest via host may not provide
 * valid samples with default 'cycles' event. It is better to use
 * 'trace_imc/trace_cycles' event for guest profiling, since it
 * can track the guest instruction pointer in the trace-record.
 *
 * Function to parse the arguments and return appropriate values.
 */
#[no_mangle]
pub unsafe extern "C" fn __kvm_add_default_arch_event_powerpc(
    argc: *mut c_int,
    argv: *mut *const c_char,
) -> c_int {
    let mut j = *argc;

    if !perf_pmus__have_event(
        b"trace_imc\0".as_ptr() as *const c_char,
        b"trace_cycles\0".as_ptr() as *const c_char,
    ) {
        return -EINVAL;
    }

    *argv.add(j as usize) = b"-e\0".as_ptr() as *const c_char;
    j += 1;
    *argv.add(j as usize) = b"trace_imc/trace_cycles/\0".as_ptr() as *const c_char;
    *argc += 2;

    0
}

#[no_mangle]
pub unsafe extern "C" fn __kvm_events_tp_powerpc() -> *const *const c_char {
    __kvm_events_tp.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn __kvm_reg_events_ops_powerpc() -> *const kvm_reg_events_ops {
    __kvm_reg_events_ops.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn __kvm_skip_events_powerpc() -> *const *const c_char {
    __kvm_skip_events.as_ptr()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
