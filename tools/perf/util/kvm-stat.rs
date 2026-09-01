// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/kvm-stat.c.
// External declarations below correspond to dependencies from:
// "debug.h", "env.h", "evsel.h", "kvm-stat.h", <dwarf-regs.h>,
// and <subcmd/parse-options.h>.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u64 = u64;
type uint16_t = u16;

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
}

#[repr(C)]
pub struct exit_reasons_table {
    pub exit_code: u64,
    pub reason: *const c_char,
}

#[repr(C)]
pub struct event_key {
    pub key: u64,
    pub info: u64,
    pub exit_reasons: *mut exit_reasons_table,
}

#[repr(C)]
pub struct perf_kvm_stat {
    pub exit_reasons_isa: *const c_char,
}

#[repr(C)]
pub struct kvm_reg_events_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

// Values are provided by <dwarf-regs.h> / repository headers in the original C.
extern "C" {
    static EM_AARCH64: uint16_t;
    static EM_LOONGARCH: uint16_t;
    static EM_PPC: uint16_t;
    static EM_PPC64: uint16_t;
    static EM_RISCV: uint16_t;
    static EM_S390: uint16_t;
    static EM_X86_64: uint16_t;
    static EM_386: uint16_t;
    static KVM_EVENT_NAME_LEN: usize;
    static PARSE_OPT_KEEP_UNKNOWN: c_uint;

    fn evsel__e_machine(evsel: *mut evsel, e_flags: *mut c_void) -> uint16_t;
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;
    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> u64;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...);

    fn __setup_kvm_events_tp_powerpc(kvm: *mut perf_kvm_stat) -> c_int;

    fn __cpu_isa_init_arm64(kvm: *mut perf_kvm_stat) -> c_int;
    fn __cpu_isa_init_loongarch(kvm: *mut perf_kvm_stat) -> c_int;
    fn __cpu_isa_init_powerpc(kvm: *mut perf_kvm_stat) -> c_int;
    fn __cpu_isa_init_riscv(kvm: *mut perf_kvm_stat) -> c_int;
    fn __cpu_isa_init_s390(kvm: *mut perf_kvm_stat, cpuid: *const c_char) -> c_int;
    fn __cpu_isa_init_x86(kvm: *mut perf_kvm_stat, cpuid: *const c_char) -> c_int;

    fn __kvm_events_tp_arm64() -> *const *const c_char;
    fn __kvm_events_tp_loongarch() -> *const *const c_char;
    fn __kvm_events_tp_powerpc() -> *const *const c_char;
    fn __kvm_events_tp_riscv() -> *const *const c_char;
    fn __kvm_events_tp_s390() -> *const *const c_char;
    fn __kvm_events_tp_x86() -> *const *const c_char;

    fn __kvm_reg_events_ops_arm64() -> *const kvm_reg_events_ops;
    fn __kvm_reg_events_ops_loongarch() -> *const kvm_reg_events_ops;
    fn __kvm_reg_events_ops_powerpc() -> *const kvm_reg_events_ops;
    fn __kvm_reg_events_ops_riscv() -> *const kvm_reg_events_ops;
    fn __kvm_reg_events_ops_s390() -> *const kvm_reg_events_ops;
    fn __kvm_reg_events_ops_x86() -> *const kvm_reg_events_ops;

    fn __kvm_skip_events_arm64() -> *const *const c_char;
    fn __kvm_skip_events_loongarch() -> *const *const c_char;
    fn __kvm_skip_events_powerpc() -> *const *const c_char;
    fn __kvm_skip_events_riscv() -> *const *const c_char;
    fn __kvm_skip_events_s390() -> *const *const c_char;
    fn __kvm_skip_events_x86() -> *const *const c_char;

    fn __kvm_add_default_arch_event_powerpc(argc: *mut c_int, argv: *const *const c_char) -> c_int;
    fn __kvm_add_default_arch_event_x86(argc: *mut c_int, argv: *const *const c_char) -> c_int;
    fn x86__is_intel_cpu() -> bool;

    fn OPT_BOOLEAN(short_name: c_char, long_name: *const c_char, value: *mut bool, help: *const c_char) -> option;
    fn OPT_END() -> option;
    fn parse_options(
        argc: c_int,
        argv: *mut *const c_char,
        options: *const option,
        usagestr: *const c_char,
        flags: c_uint,
    ) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

const NULL: *const c_char = core::ptr::null();

const S_ID: *const c_char = b"id\0".as_ptr() as *const c_char;
const S_VCPU_ID: *const c_char = b"vcpu_id\0".as_ptr() as *const c_char;
const S_RET: *const c_char = b"ret\0".as_ptr() as *const c_char;
const S_REASON: *const c_char = b"reason\0".as_ptr() as *const c_char;
const S_TRAP: *const c_char = b"trap\0".as_ptr() as *const c_char;
const S_SCAUSE: *const c_char = b"scause\0".as_ptr() as *const c_char;
const S_ICPTCODE: *const c_char = b"icptcode\0".as_ptr() as *const c_char;
const S_EXIT_REASON: *const c_char = b"exit_reason\0".as_ptr() as *const c_char;
const S_KVM_ENTRY: *const c_char = b"kvm:kvm_entry\0".as_ptr() as *const c_char;
const S_KVM_ENTER: *const c_char = b"kvm:kvm_enter\0".as_ptr() as *const c_char;
const S_KVM_GUEST_ENTER: *const c_char = b"kvm_hv:kvm_guest_enter\0".as_ptr() as *const c_char;
const S_KVM_S390_SIE_ENTER: *const c_char = b"kvm:kvm_s390_sie_enter\0".as_ptr() as *const c_char;
const S_KVM_EXIT: *const c_char = b"kvm:kvm_exit\0".as_ptr() as *const c_char;
const S_KVM_GUEST_EXIT: *const c_char = b"kvm_hv:kvm_guest_exit\0".as_ptr() as *const c_char;
const S_KVM_S390_SIE_EXIT: *const c_char = b"kvm:kvm_s390_sie_exit\0".as_ptr() as *const c_char;
const S_UNKNOWN: *const c_char = b"UNKNOWN\0".as_ptr() as *const c_char;
const S_EVENT: *const c_char = b"event\0".as_ptr() as *const c_char;
const S_PFM_EVENTS: *const c_char = b"pfm-events\0".as_ptr() as *const c_char;

#[no_mangle]
pub unsafe extern "C" fn kvm_exit_event(evsel: *mut evsel) -> bool {
    let e_machine: uint16_t = evsel__e_machine(evsel, core::ptr::null_mut());

    evsel__name_is(evsel, kvm_exit_trace(e_machine))
}

#[no_mangle]
pub unsafe extern "C" fn exit_event_get_key(sample: *mut perf_sample, key: *mut event_key) {
    let e_machine: uint16_t = evsel__e_machine((*sample).evsel, core::ptr::null_mut());

    (*key).info = 0;
    (*key).key = perf_sample__intval(sample, kvm_exit_reason(e_machine));
}

#[no_mangle]
pub unsafe extern "C" fn exit_event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool {
    if kvm_exit_event((*sample).evsel) {
        exit_event_get_key(sample, key);
        return true;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn kvm_entry_event(evsel: *mut evsel) -> bool {
    let e_machine: uint16_t = evsel__e_machine(evsel, core::ptr::null_mut());

    evsel__name_is(evsel, kvm_entry_trace(e_machine))
}

#[no_mangle]
pub unsafe extern "C" fn exit_event_end(sample: *mut perf_sample, _key: *mut event_key) -> bool {
    kvm_entry_event((*sample).evsel)
}

unsafe fn get_exit_reason(
    kvm: *mut perf_kvm_stat,
    mut tbl: *mut exit_reasons_table,
    exit_code: u64,
) -> *const c_char {
    while !(*tbl).reason.is_null() {
        if (*tbl).exit_code == exit_code {
            return (*tbl).reason;
        }
        tbl = tbl.add(1);
    }

    pr_err(
        b"unknown kvm exit code:%lld on %s\n\0".as_ptr() as *const c_char,
        exit_code as u64,
        (*kvm).exit_reasons_isa,
    );
    S_UNKNOWN
}

#[no_mangle]
pub unsafe extern "C" fn exit_event_decode_key(
    kvm: *mut perf_kvm_stat,
    key: *mut event_key,
    decode: *mut c_char,
) {
    let exit_reason: *const c_char = get_exit_reason(kvm, (*key).exit_reasons, (*key).key);

    scnprintf(
        decode,
        KVM_EVENT_NAME_LEN,
        b"%s\0".as_ptr() as *const c_char,
        exit_reason,
    );
}

#[no_mangle]
pub unsafe extern "C" fn setup_kvm_events_tp(kvm: *mut perf_kvm_stat, e_machine: uint16_t) -> c_int {
    if e_machine == EM_PPC || e_machine == EM_PPC64 {
        return __setup_kvm_events_tp_powerpc(kvm);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn cpu_isa_init(
    kvm: *mut perf_kvm_stat,
    e_machine: uint16_t,
    cpuid: *const c_char,
) -> c_int {
    if e_machine == EM_AARCH64 {
        return __cpu_isa_init_arm64(kvm);
    } else if e_machine == EM_LOONGARCH {
        return __cpu_isa_init_loongarch(kvm);
    } else if e_machine == EM_PPC || e_machine == EM_PPC64 {
        return __cpu_isa_init_powerpc(kvm);
    } else if e_machine == EM_RISCV {
        return __cpu_isa_init_riscv(kvm);
    } else if e_machine == EM_S390 {
        return __cpu_isa_init_s390(kvm, cpuid);
    } else if e_machine == EM_X86_64 || e_machine == EM_386 {
        return __cpu_isa_init_x86(kvm, cpuid);
    } else {
        pr_err(
            b"Unsupported kvm-stat host %d\n\0".as_ptr() as *const c_char,
            e_machine as c_int,
        );
        return -1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn vcpu_id_str(e_machine: uint16_t) -> *const c_char {
    if e_machine == EM_AARCH64 || e_machine == EM_RISCV || e_machine == EM_S390 {
        return S_ID;
    } else if e_machine == EM_LOONGARCH
        || e_machine == EM_PPC
        || e_machine == EM_PPC64
        || e_machine == EM_X86_64
        || e_machine == EM_386
    {
        return S_VCPU_ID;
    } else {
        pr_err(
            b"Unsupported kvm-stat host %d\n\0".as_ptr() as *const c_char,
            e_machine as c_int,
        );
        return core::ptr::null();
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_exit_reason(e_machine: uint16_t) -> *const c_char {
    if e_machine == EM_AARCH64 {
        return S_RET;
    } else if e_machine == EM_LOONGARCH {
        return S_REASON;
    } else if e_machine == EM_PPC || e_machine == EM_PPC64 {
        return S_TRAP;
    } else if e_machine == EM_RISCV {
        return S_SCAUSE;
    } else if e_machine == EM_S390 {
        return S_ICPTCODE;
    } else if e_machine == EM_X86_64 || e_machine == EM_386 {
        return S_EXIT_REASON;
    } else {
        pr_err(
            b"Unsupported kvm-stat host %d\n\0".as_ptr() as *const c_char,
            e_machine as c_int,
        );
        return core::ptr::null();
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_entry_trace(e_machine: uint16_t) -> *const c_char {
    if e_machine == EM_AARCH64 || e_machine == EM_RISCV || e_machine == EM_X86_64 || e_machine == EM_386 {
        return S_KVM_ENTRY;
    } else if e_machine == EM_LOONGARCH {
        return S_KVM_ENTER;
    } else if e_machine == EM_PPC || e_machine == EM_PPC64 {
        return S_KVM_GUEST_ENTER;
    } else if e_machine == EM_S390 {
        return S_KVM_S390_SIE_ENTER;
    } else {
        pr_err(
            b"Unsupported kvm-stat host %d\n\0".as_ptr() as *const c_char,
            e_machine as c_int,
        );
        return core::ptr::null();
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_exit_trace(e_machine: uint16_t) -> *const c_char {
    if e_machine == EM_AARCH64
        || e_machine == EM_LOONGARCH
        || e_machine == EM_RISCV
        || e_machine == EM_X86_64
        || e_machine == EM_386
    {
        return S_KVM_EXIT;
    } else if e_machine == EM_PPC || e_machine == EM_PPC64 {
        return S_KVM_GUEST_EXIT;
    } else if e_machine == EM_S390 {
        return S_KVM_S390_SIE_EXIT;
    } else {
        pr_err(
            b"Unsupported kvm-stat host %d\n\0".as_ptr() as *const c_char,
            e_machine as c_int,
        );
        return core::ptr::null();
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_events_tp(e_machine: uint16_t) -> *const *const c_char {
    if e_machine == EM_AARCH64 {
        return __kvm_events_tp_arm64();
    } else if e_machine == EM_LOONGARCH {
        return __kvm_events_tp_loongarch();
    } else if e_machine == EM_PPC || e_machine == EM_PPC64 {
        return __kvm_events_tp_powerpc();
    } else if e_machine == EM_RISCV {
        return __kvm_events_tp_riscv();
    } else if e_machine == EM_S390 {
        return __kvm_events_tp_s390();
    } else if e_machine == EM_X86_64 || e_machine == EM_386 {
        return __kvm_events_tp_x86();
    } else {
        pr_err(
            b"Unsupported kvm-stat host %d\n\0".as_ptr() as *const c_char,
            e_machine as c_int,
        );
        return core::ptr::null();
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_reg_events_ops(e_machine: uint16_t) -> *const kvm_reg_events_ops {
    if e_machine == EM_AARCH64 {
        return __kvm_reg_events_ops_arm64();
    } else if e_machine == EM_LOONGARCH {
        return __kvm_reg_events_ops_loongarch();
    } else if e_machine == EM_PPC || e_machine == EM_PPC64 {
        return __kvm_reg_events_ops_powerpc();
    } else if e_machine == EM_RISCV {
        return __kvm_reg_events_ops_riscv();
    } else if e_machine == EM_S390 {
        return __kvm_reg_events_ops_s390();
    } else if e_machine == EM_X86_64 || e_machine == EM_386 {
        return __kvm_reg_events_ops_x86();
    } else {
        pr_err(
            b"Unsupported kvm-stat host %d\n\0".as_ptr() as *const c_char,
            e_machine as c_int,
        );
        return core::ptr::null();
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_skip_events(e_machine: uint16_t) -> *const *const c_char {
    if e_machine == EM_AARCH64 {
        return __kvm_skip_events_arm64();
    } else if e_machine == EM_LOONGARCH {
        return __kvm_skip_events_loongarch();
    } else if e_machine == EM_PPC || e_machine == EM_PPC64 {
        return __kvm_skip_events_powerpc();
    } else if e_machine == EM_RISCV {
        return __kvm_skip_events_riscv();
    } else if e_machine == EM_S390 {
        return __kvm_skip_events_s390();
    } else if e_machine == EM_X86_64 || e_machine == EM_386 {
        return __kvm_skip_events_x86();
    } else {
        pr_err(
            b"Unsupported kvm-stat host %d\n\0".as_ptr() as *const c_char,
            e_machine as c_int,
        );
        return core::ptr::null();
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_add_default_arch_event(
    e_machine: uint16_t,
    argc: *mut c_int,
    argv: *const *const c_char,
) -> c_int {
    if e_machine == EM_PPC || e_machine == EM_PPC64 {
        return __kvm_add_default_arch_event_powerpc(argc, argv);
    } else if e_machine == EM_X86_64 || e_machine == EM_386 {
        return __kvm_add_default_arch_event_x86(argc, argv);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn kvm_need_default_arch_event(
    e_machine: uint16_t,
    argc: c_int,
    argv: *const *const c_char,
) -> bool {
    let mut tmp_argv: *mut *const c_char;
    let mut event: bool = false;
    let mut i: c_int;

    let event_options = [
        OPT_BOOLEAN('e' as c_char, S_EVENT, &mut event as *mut bool, NULL),
        OPT_BOOLEAN(0, S_PFM_EVENTS, &mut event as *mut bool, NULL),
        OPT_END(),
    ];

    if e_machine == EM_PPC || e_machine == EM_PPC64 {
    } else if e_machine == EM_X86_64 || e_machine == EM_386 {
        if !x86__is_intel_cpu() {
            return false;
        }
    } else {
        return false;
    }

    /* parse_options() may change the argv, let's make a copy */
    tmp_argv = calloc((argc + 1) as usize, core::mem::size_of::<*const c_char>()) as *mut *const c_char;
    if tmp_argv.is_null() {
        return false;
    }

    i = 0;
    while i < argc {
        *tmp_argv.offset(i as isize) = *argv.offset(i as isize);
        i += 1;
    }

    parse_options(argc, tmp_argv, event_options.as_ptr(), core::ptr::null(), PARSE_OPT_KEEP_UNKNOWN);
    free(tmp_argv as *mut c_void);

    !event
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
