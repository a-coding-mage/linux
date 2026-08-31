// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/kvm-stat-arch/kvm-stat-loongarch.c.
// C include dependencies intentionally remain external to this translation:
// errno.h, memory.h, dwarf-regs.h, ../kvm-stat.h, ../parse-events.h,
// ../debug.h, ../evsel.h, ../evlist.h, ../pmus.h.

use core::ffi::{c_char, c_int};
use core::ptr;

const LOONGARCH_EXCEPTION_INT: u32 = 0;
const LOONGARCH_EXCEPTION_PIL: u32 = 1;
const LOONGARCH_EXCEPTION_PIS: u32 = 2;
const LOONGARCH_EXCEPTION_PIF: u32 = 3;
const LOONGARCH_EXCEPTION_PME: u32 = 4;
const LOONGARCH_EXCEPTION_FPD: u32 = 15;
const LOONGARCH_EXCEPTION_SXD: u32 = 16;
const LOONGARCH_EXCEPTION_ASXD: u32 = 17;
const LOONGARCH_EXCEPTION_GSPR: u32 = 22;
const LOONGARCH_EXCEPTION_CPUCFG: u32 = 100;
const LOONGARCH_EXCEPTION_CSR: u32 = 101;
const LOONGARCH_EXCEPTION_IOCSR: u32 = 102;
const LOONGARCH_EXCEPTION_IDLE: u32 = 103;
const LOONGARCH_EXCEPTION_OTHERS: u32 = 104;
const LOONGARCH_EXCEPTION_HVC: u32 = 23;

static loongarch_exit_reasons: [exit_reasons_table; 15] = [
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_INT as c_int,
        reason: c"Interrupt".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_PIL as c_int,
        reason: c"Mem Read".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_PIS as c_int,
        reason: c"Mem Store".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_PIF as c_int,
        reason: c"Inst Fetch".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_PME as c_int,
        reason: c"Mem Modify".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_FPD as c_int,
        reason: c"FPU".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_SXD as c_int,
        reason: c"LSX".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_ASXD as c_int,
        reason: c"LASX".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_GSPR as c_int,
        reason: c"Privilege Error".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_HVC as c_int,
        reason: c"Hypercall".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_CPUCFG as c_int,
        reason: c"CPUCFG".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_CSR as c_int,
        reason: c"CSR".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_IOCSR as c_int,
        reason: c"IOCSR".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_IDLE as c_int,
        reason: c"Idle".as_ptr(),
    },
    exit_reasons_table {
        exit_code: LOONGARCH_EXCEPTION_OTHERS as c_int,
        reason: c"Others".as_ptr(),
    },
];

static kvm_reenter_trace: *const c_char = c"kvm:kvm_reenter".as_ptr();
static __kvm_events_tp: [*const c_char; 5] = [
    c"kvm:kvm_enter".as_ptr(),
    c"kvm:kvm_reenter".as_ptr(),
    c"kvm:kvm_exit".as_ptr(),
    c"kvm:kvm_exit_gspr".as_ptr(),
    ptr::null(),
];

unsafe extern "C" fn event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool {
    unsafe { exit_event_begin(sample, key) }
}

unsafe extern "C" fn event_end(sample: *mut perf_sample, key: *mut event_key) -> bool {
    /*
     * LoongArch kvm is different with other architectures
     *
     * There is kvm:kvm_reenter or kvm:kvm_enter event adjacent with
     * kvm:kvm_exit event.
     *   kvm:kvm_enter   means returning to vmm and then to guest
     *   kvm:kvm_reenter means returning to guest immediately
     */
    let _ = key;
    unsafe {
        evsel__name_is((*sample).evsel, kvm_entry_trace(EM_LOONGARCH))
            || evsel__name_is((*sample).evsel, kvm_reenter_trace)
    }
}

unsafe extern "C" fn event_gspr_get_key(sample: *mut perf_sample, key: *mut event_key) {
    let insn: u32;

    unsafe {
        (*key).key = LOONGARCH_EXCEPTION_OTHERS as u64;
        insn = perf_sample__intval(sample, c"inst_word".as_ptr()) as u32;

        match insn >> 24 {
            0 => {
                /* CPUCFG inst trap */
                if (insn >> 10) == 0x1b {
                    (*key).key = LOONGARCH_EXCEPTION_CPUCFG as u64;
                }
            }
            4 => {
                /* CSR inst trap */
                (*key).key = LOONGARCH_EXCEPTION_CSR as u64;
            }
            6 => {
                /* IOCSR inst trap */
                if (insn >> 15) == 0xc90 {
                    (*key).key = LOONGARCH_EXCEPTION_IOCSR as u64;
                } else if (insn >> 15) == 0xc91 {
                    /* Idle inst trap */
                    (*key).key = LOONGARCH_EXCEPTION_IDLE as u64;
                }
            }
            _ => {
                (*key).key = LOONGARCH_EXCEPTION_OTHERS as u64;
            }
        }
    }
}

static child_events: [child_event_ops; 2] = [
    child_event_ops {
        name: c"kvm:kvm_exit_gspr".as_ptr(),
        get_key: Some(event_gspr_get_key),
    },
    child_event_ops {
        name: ptr::null(),
        get_key: None,
    },
];

static exit_events: kvm_events_ops = kvm_events_ops {
    is_begin_event: Some(event_begin),
    is_end_event: Some(event_end),
    child_ops: child_events.as_ptr(),
    decode_key: Some(exit_event_decode_key),
    name: c"VM-EXIT".as_ptr(),
};

static __kvm_reg_events_ops: [kvm_reg_events_ops; 2] = [
    kvm_reg_events_ops {
        name: c"vmexit".as_ptr(),
        ops: &exit_events,
    },
    kvm_reg_events_ops {
        name: ptr::null(),
        ops: ptr::null(),
    },
];

static __kvm_skip_events: [*const c_char; 1] = [ptr::null()];

#[no_mangle]
pub unsafe extern "C" fn __cpu_isa_init_loongarch(kvm: *mut perf_kvm_stat) -> c_int {
    unsafe {
        (*kvm).exit_reasons_isa = c"loongarch64".as_ptr();
        (*kvm).exit_reasons = loongarch_exit_reasons.as_ptr();
    }
    0
}

#[no_mangle]
pub extern "C" fn __kvm_events_tp_loongarch() -> *const *const c_char {
    __kvm_events_tp.as_ptr()
}

#[no_mangle]
pub extern "C" fn __kvm_reg_events_ops_loongarch() -> *const kvm_reg_events_ops {
    __kvm_reg_events_ops.as_ptr()
}

#[no_mangle]
pub extern "C" fn __kvm_skip_events_loongarch() -> *const *const c_char {
    __kvm_skip_events.as_ptr()
}
