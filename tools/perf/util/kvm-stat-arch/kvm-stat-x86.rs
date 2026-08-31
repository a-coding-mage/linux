// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/kvm-stat-arch/kvm-stat-x86.c.
// C include dependencies are intentionally left as external declarations.

use core::ffi::{c_char, c_int, c_ulong, c_ulonglong};

const ENOTSUP: c_int = 524;
const KVM_TRACE_MMIO_READ_UNSATISFIED: u64 = 0;
const KVM_TRACE_MMIO_READ: u64 = 1;
const KVM_TRACE_MMIO_WRITE: u64 = 2;

unsafe extern "C" {
    static vmx_exit_reasons: *const c_void;
    static svm_exit_reasons: *const c_void;

    fn exit_event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool;
    fn exit_event_end(sample: *mut perf_sample, key: *mut event_key) -> bool;
    fn exit_event_decode_key(kvm: *mut perf_kvm_stat, key: *mut event_key, decode: *mut c_char);

    fn kvm_exit_event(evsel: *mut evsel) -> bool;
    fn kvm_entry_event(evsel: *mut evsel) -> bool;
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;
    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> u64;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub enum c_void {}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
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
pub struct perf_kvm_stat {
    pub exit_reasons: *const c_void,
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

unsafe extern "C" {
    static KVM_EVENT_NAME_LEN: usize;
}

static EXIT_EVENTS_NAME: &[u8] = b"VM-EXIT\0";

static mut exit_events: kvm_events_ops = kvm_events_ops {
    is_begin_event: Some(exit_event_begin),
    is_end_event: Some(exit_event_end),
    decode_key: Some(exit_event_decode_key),
    name: EXIT_EVENTS_NAME.as_ptr() as *const c_char,
};

/*
 * For the mmio events, we treat:
 * the time of MMIO write: kvm_mmio(KVM_TRACE_MMIO_WRITE...) -> kvm_entry
 * the time of MMIO read: kvm_exit -> kvm_mmio(KVM_TRACE_MMIO_READ...).
 */
unsafe extern "C" fn mmio_event_get_key(sample: *mut perf_sample, key: *mut event_key) {
    (*key).key = perf_sample__intval(sample, c"gpa".as_ptr());
    (*key).info = perf_sample__intval(sample, c"type".as_ptr());
}

unsafe extern "C" fn mmio_event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool {
    /* MMIO read begin event in kernel. */
    if kvm_exit_event((*sample).evsel) {
        return true;
    }

    /* MMIO write begin event in kernel. */
    if evsel__name_is((*sample).evsel, c"kvm:kvm_mmio".as_ptr())
        && perf_sample__intval(sample, c"type".as_ptr()) == KVM_TRACE_MMIO_WRITE
    {
        mmio_event_get_key(sample, key);
        return true;
    }

    false
}

unsafe extern "C" fn mmio_event_end(sample: *mut perf_sample, key: *mut event_key) -> bool {
    /* MMIO write end event in kernel. */
    if kvm_entry_event((*sample).evsel) {
        return true;
    }

    /* MMIO read end event in kernel.*/
    if evsel__name_is((*sample).evsel, c"kvm:kvm_mmio".as_ptr())
        && perf_sample__intval(sample, c"type".as_ptr()) == KVM_TRACE_MMIO_READ
    {
        mmio_event_get_key(sample, key);
        return true;
    }

    false
}

unsafe extern "C" fn mmio_event_decode_key(
    _kvm: *mut perf_kvm_stat,
    key: *mut event_key,
    decode: *mut c_char,
) {
    scnprintf(
        decode,
        KVM_EVENT_NAME_LEN,
        c"%#lx:%s".as_ptr(),
        (*key).key as c_ulong,
        if (*key).info == KVM_TRACE_MMIO_WRITE {
            c"W".as_ptr()
        } else {
            c"R".as_ptr()
        },
    );
}

static MMIO_EVENTS_NAME: &[u8] = b"MMIO Access\0";

static mut mmio_events: kvm_events_ops = kvm_events_ops {
    is_begin_event: Some(mmio_event_begin),
    is_end_event: Some(mmio_event_end),
    decode_key: Some(mmio_event_decode_key),
    name: MMIO_EVENTS_NAME.as_ptr() as *const c_char,
};

/* The time of emulation pio access is from kvm_pio to kvm_entry. */
unsafe extern "C" fn ioport_event_get_key(sample: *mut perf_sample, key: *mut event_key) {
    (*key).key = perf_sample__intval(sample, c"port".as_ptr());
    (*key).info = perf_sample__intval(sample, c"rw".as_ptr());
}

unsafe extern "C" fn ioport_event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool {
    if evsel__name_is((*sample).evsel, c"kvm:kvm_pio".as_ptr()) {
        ioport_event_get_key(sample, key);
        return true;
    }

    false
}

unsafe extern "C" fn ioport_event_end(sample: *mut perf_sample, _key: *mut event_key) -> bool {
    kvm_entry_event((*sample).evsel)
}

unsafe extern "C" fn ioport_event_decode_key(
    _kvm: *mut perf_kvm_stat,
    key: *mut event_key,
    decode: *mut c_char,
) {
    scnprintf(
        decode,
        KVM_EVENT_NAME_LEN,
        c"%#llx:%s".as_ptr(),
        (*key).key as c_ulonglong,
        if (*key).info != 0 {
            c"POUT".as_ptr()
        } else {
            c"PIN".as_ptr()
        },
    );
}

static IOPORT_EVENTS_NAME: &[u8] = b"IO Port Access\0";

static mut ioport_events: kvm_events_ops = kvm_events_ops {
    is_begin_event: Some(ioport_event_begin),
    is_end_event: Some(ioport_event_end),
    decode_key: Some(ioport_event_decode_key),
    name: IOPORT_EVENTS_NAME.as_ptr() as *const c_char,
};

/* The time of emulation msr is from kvm_msr to kvm_entry. */
unsafe extern "C" fn msr_event_get_key(sample: *mut perf_sample, key: *mut event_key) {
    (*key).key = perf_sample__intval(sample, c"ecx".as_ptr());
    (*key).info = perf_sample__intval(sample, c"write".as_ptr());
}

unsafe extern "C" fn msr_event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool {
    if evsel__name_is((*sample).evsel, c"kvm:kvm_msr".as_ptr()) {
        msr_event_get_key(sample, key);
        return true;
    }

    false
}

unsafe extern "C" fn msr_event_end(sample: *mut perf_sample, _key: *mut event_key) -> bool {
    kvm_entry_event((*sample).evsel)
}

unsafe extern "C" fn msr_event_decode_key(
    _kvm: *mut perf_kvm_stat,
    key: *mut event_key,
    decode: *mut c_char,
) {
    scnprintf(
        decode,
        KVM_EVENT_NAME_LEN,
        c"%#llx:%s".as_ptr(),
        (*key).key as c_ulonglong,
        if (*key).info != 0 {
            c"W".as_ptr()
        } else {
            c"R".as_ptr()
        },
    );
}

static MSR_EVENTS_NAME: &[u8] = b"MSR Access\0";

static mut msr_events: kvm_events_ops = kvm_events_ops {
    is_begin_event: Some(msr_event_begin),
    is_end_event: Some(msr_event_end),
    decode_key: Some(msr_event_decode_key),
    name: MSR_EVENTS_NAME.as_ptr() as *const c_char,
};

static mut __kvm_events_tp: [*const c_char; 6] = [
    c"kvm:kvm_entry".as_ptr(),
    c"kvm:kvm_exit".as_ptr(),
    c"kvm:kvm_mmio".as_ptr(),
    c"kvm:kvm_pio".as_ptr(),
    c"kvm:kvm_msr".as_ptr(),
    core::ptr::null(),
];

static VMEXIT_NAME: &[u8] = b"vmexit\0";
static MMIO_NAME: &[u8] = b"mmio\0";
static IOPORT_NAME: &[u8] = b"ioport\0";
static MSR_NAME: &[u8] = b"msr\0";

static mut __kvm_reg_events_ops: [kvm_reg_events_ops; 5] = [
    kvm_reg_events_ops {
        name: VMEXIT_NAME.as_ptr() as *const c_char,
        ops: core::ptr::addr_of!(exit_events),
    },
    kvm_reg_events_ops {
        name: MMIO_NAME.as_ptr() as *const c_char,
        ops: core::ptr::addr_of!(mmio_events),
    },
    kvm_reg_events_ops {
        name: IOPORT_NAME.as_ptr() as *const c_char,
        ops: core::ptr::addr_of!(ioport_events),
    },
    kvm_reg_events_ops {
        name: MSR_NAME.as_ptr() as *const c_char,
        ops: core::ptr::addr_of!(msr_events),
    },
    kvm_reg_events_ops {
        name: core::ptr::null(),
        ops: core::ptr::null(),
    },
];

static mut __kvm_skip_events: [*const c_char; 2] = [c"HLT".as_ptr(), core::ptr::null()];

#[no_mangle]
pub unsafe extern "C" fn __cpu_isa_init_x86(
    kvm: *mut perf_kvm_stat,
    cpuid: *const c_char,
) -> c_int {
    if !strstr(cpuid, c"Intel".as_ptr()).is_null() {
        (*kvm).exit_reasons = vmx_exit_reasons;
        (*kvm).exit_reasons_isa = c"VMX".as_ptr();
    } else if !strstr(cpuid, c"AMD".as_ptr()).is_null()
        || !strstr(cpuid, c"Hygon".as_ptr()).is_null()
    {
        (*kvm).exit_reasons = svm_exit_reasons;
        (*kvm).exit_reasons_isa = c"SVM".as_ptr();
    } else {
        return -ENOTSUP;
    }

    0
}

/*
 * After KVM supports PEBS for guest on Intel platforms
 * (https://lore.kernel.org/all/20220411101946.20262-1-likexu@tencent.com/),
 * host loses the capability to sample guest with PEBS since all PEBS related
 * MSRs are switched to guest value after vm-entry, like IA32_DS_AREA MSR is
 * switched to guest GVA at vm-entry. This would lead to "perf kvm record"
 * fails to sample guest on Intel platforms since "cycles:P" event is used to
 * sample guest by default.
 *
 * So, to avoid this issue explicitly use "cycles" instead of "cycles:P" event
 * by default to sample guest on Intel platforms.
 */
#[no_mangle]
pub unsafe extern "C" fn __kvm_add_default_arch_event_x86(
    argc: *mut c_int,
    argv: *mut *const c_char,
) -> c_int {
    let mut j = *argc;

    *argv.offset(j as isize) = c"-e".as_ptr();
    j += 1;
    *argv.offset(j as isize) = c"cycles".as_ptr();
    *argc += 2;

    0
}

#[no_mangle]
pub unsafe extern "C" fn __kvm_events_tp_x86() -> *const *const c_char {
    core::ptr::addr_of!(__kvm_events_tp) as *const *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn __kvm_reg_events_ops_x86() -> *const kvm_reg_events_ops {
    core::ptr::addr_of!(__kvm_reg_events_ops) as *const kvm_reg_events_ops
}

#[no_mangle]
pub unsafe extern "C" fn __kvm_skip_events_x86() -> *const *const c_char {
    core::ptr::addr_of!(__kvm_skip_events) as *const *const c_char
}
