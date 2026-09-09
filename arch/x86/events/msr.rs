// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/perf_event.h, linux/sysfs.h, linux/nospec.h,
// asm/msr.h, and "probe.h" supply the referenced kernel types, constants,
// macros, and functions.

#[repr(C)]
#[derive(Copy, Clone)]
enum PerfMsrId {
    PerfMsrTsc = 0,
    PerfMsrAperf = 1,
    PerfMsrMperf = 2,
    PerfMsrPperf = 3,
    PerfMsrSmi = 4,
    PerfMsrPtsc = 5,
    PerfMsrIrperf = 6,
    PerfMsrTherm = 7,
    PerfMsrEventMax,
}

unsafe extern "C" {
    static mut boot_cpu_data: BootCpuData;
    static mut msr_mask: ::core::ffi::c_ulong;

    fn boot_cpu_has(feature: u32) -> bool;
    fn perf_msr_probe(msr: *mut PerfMsr, max: u32, check: bool, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong;
    fn perf_pmu_register(pmu: *mut Pmu, name: *const u8, context: i32) -> i32;
    fn rdtsc_ordered() -> u64;
    fn rdmsrq(msr: u32, value: &mut u64);
    fn local64_read(value: *const Local64) -> u64;
    fn local64_try_cmpxchg(value: *mut Local64, old: *mut u64, new: u64) -> bool;
    fn local64_add(value: i64, count: *mut Local64);
    fn local64_set(count: *mut Local64, value: u64);
    fn sign_extend64(value: u64, index: u32) -> i64;
}

#[repr(C)]
struct BootCpuData {
    x86_vendor: u32,
}

#[repr(C)]
struct Local64 {
    value: u64,
}

#[repr(C)]
struct PerfMsr {
    msr: u32,
    group: *mut AttributeGroup,
    test: Option<unsafe extern "C" fn(i32, *mut ::core::ffi::c_void) -> bool>,
    no_check: bool,
}

#[repr(C)]
struct PerfEvent {
    attr: PerfEventAttr,
    pmu: *mut Pmu,
    hw: HwPerfEvent,
    count: Local64,
}

#[repr(C)]
struct PerfEventAttr {
    config: u64,
    type_: u32,
    sample_period: u64,
}

#[repr(C)]
struct HwPerfEvent {
    idx: i32,
    event_base: u32,
    config: u64,
    prev_count: Local64,
}

#[repr(C)]
struct Pmu {
    task_ctx_nr: i32,
    attr_groups: *const *const AttributeGroup,
    event_init: Option<unsafe extern "C" fn(*mut PerfEvent) -> i32>,
    add: Option<unsafe extern "C" fn(*mut PerfEvent, i32) -> i32>,
    del: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>,
    start: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>,
    stop: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>,
    read: Option<unsafe extern "C" fn(*mut PerfEvent)>,
    capabilities: u64,
    attr_update: *const *const AttributeGroup,
}

#[repr(C)]
struct AttributeGroup {
    name: *const u8,
    attrs: *const *mut Attribute,
}

#[repr(C)]
struct Attribute;

const PERF_MSR_EVENT_MAX: u64 = PerfMsrId::PerfMsrEventMax as u64;
const PERF_EF_START: i32 = 1;
const PERF_EF_UPDATE: i32 = 1;
const ENOENT: i32 = 2;
const EINVAL: i32 = 22;

static mut msr: [PerfMsr; PerfMsrId::PerfMsrEventMax as usize] = [
    PerfMsr { msr: 0, group: core::ptr::null_mut(), test: None, no_check: true },
    PerfMsr { msr: MSR_IA32_APERF, group: &raw mut group_aperf, test: Some(test_aperfmperf), no_check: false },
    PerfMsr { msr: MSR_IA32_MPERF, group: &raw mut group_mperf, test: Some(test_aperfmperf), no_check: false },
    PerfMsr { msr: MSR_PPERF, group: &raw mut group_pperf, test: Some(test_intel), no_check: false },
    PerfMsr { msr: MSR_SMI_COUNT, group: &raw mut group_smi, test: Some(test_intel), no_check: false },
    PerfMsr { msr: MSR_F15H_PTSC, group: &raw mut group_ptsc, test: Some(test_ptsc), no_check: false },
    PerfMsr { msr: MSR_F17H_IRPERF, group: &raw mut group_irperf, test: Some(test_irperf), no_check: false },
    PerfMsr { msr: MSR_IA32_THERM_STATUS, group: &raw mut group_therm, test: Some(test_therm_status), no_check: false },
];

unsafe extern "C" fn test_aperfmperf(_idx: i32, _data: *mut ::core::ffi::c_void) -> bool { boot_cpu_has(X86_FEATURE_APERFMPERF) }
unsafe extern "C" fn test_ptsc(_idx: i32, _data: *mut ::core::ffi::c_void) -> bool { boot_cpu_has(X86_FEATURE_PTSC) }
unsafe extern "C" fn test_irperf(_idx: i32, _data: *mut ::core::ffi::c_void) -> bool { boot_cpu_has(X86_FEATURE_IRPERF) }
unsafe extern "C" fn test_therm_status(_idx: i32, _data: *mut ::core::ffi::c_void) -> bool { boot_cpu_has(X86_FEATURE_DTHERM) }
unsafe extern "C" fn test_intel(_idx: i32, _data: *mut ::core::ffi::c_void) -> bool {
    if boot_cpu_data.x86_vendor != X86_VENDOR_INTEL { return false; }
    true
}

static mut group_aperf: AttributeGroup = AttributeGroup { name: b"events\0".as_ptr(), attrs: core::ptr::null() };
static mut group_mperf: AttributeGroup = AttributeGroup { name: b"events\0".as_ptr(), attrs: core::ptr::null() };
static mut group_pperf: AttributeGroup = AttributeGroup { name: b"events\0".as_ptr(), attrs: core::ptr::null() };
static mut group_smi: AttributeGroup = AttributeGroup { name: b"events\0".as_ptr(), attrs: core::ptr::null() };
static mut group_ptsc: AttributeGroup = AttributeGroup { name: b"events\0".as_ptr(), attrs: core::ptr::null() };
static mut group_irperf: AttributeGroup = AttributeGroup { name: b"events\0".as_ptr(), attrs: core::ptr::null() };
static mut group_therm: AttributeGroup = AttributeGroup { name: b"events\0".as_ptr(), attrs: core::ptr::null() };

unsafe fn msr_read_counter(event: *mut PerfEvent) -> u64 {
    let mut now: u64 = 0;
    if (*event).hw.event_base != 0 { rdmsrq((*event).hw.event_base, &mut now); } else { now = rdtsc_ordered(); }
    now
}

unsafe extern "C" fn msr_event_init(event: *mut PerfEvent) -> i32 {
    let mut cfg = (*event).attr.config;
    if (*event).attr.type_ != (*(*event).pmu).task_ctx_nr as u32 { return -ENOENT; }
    if (*event).attr.sample_period != 0 || cfg >= PERF_MSR_EVENT_MAX { return -EINVAL; }
    cfg = array_index_nospec(cfg as usize, PERF_MSR_EVENT_MAX as usize) as u64;
    if (msr_mask & (1 << cfg)) == 0 { return -EINVAL; }
    (*event).hw.idx = -1;
    (*event).hw.event_base = msr[cfg as usize].msr;
    (*event).hw.config = cfg;
    0
}

unsafe extern "C" fn msr_event_update(event: *mut PerfEvent) {
    let mut prev = local64_read(&(*event).hw.prev_count);
    let now;
    loop { let current = msr_read_counter(event); if local64_try_cmpxchg(&mut (*event).hw.prev_count, &mut prev, current) { now = current; break; } }
    let mut delta = now.wrapping_sub(prev) as i64;
    if (*event).hw.event_base == MSR_SMI_COUNT { delta = sign_extend64(delta as u64, 31); local64_add(delta, &mut (*event).count); }
    else if (*event).hw.event_base == MSR_IA32_THERM_STATUS { let value = if now & (1u64 << 31) != 0 { (now >> 16) & 0x3f } else { u64::MAX }; local64_set(&mut (*event).count, value); }
    else { local64_add(delta, &mut (*event).count); }
}

unsafe extern "C" fn msr_event_start(event: *mut PerfEvent, _flags: i32) { let now = msr_read_counter(event); local64_set(&mut (*event).hw.prev_count, now); }
unsafe extern "C" fn msr_event_stop(event: *mut PerfEvent, _flags: i32) { msr_event_update(event); }
unsafe extern "C" fn msr_event_del(event: *mut PerfEvent, _flags: i32) { msr_event_stop(event, PERF_EF_UPDATE); }
unsafe extern "C" fn msr_event_add(event: *mut PerfEvent, flags: i32) -> i32 { if flags & PERF_EF_START != 0 { msr_event_start(event, flags); } 0 }

static mut pmu_msr: Pmu = Pmu { task_ctx_nr: 0, attr_groups: core::ptr::null(), event_init: Some(msr_event_init), add: Some(msr_event_add), del: Some(msr_event_del), start: Some(msr_event_start), stop: Some(msr_event_stop), read: Some(msr_event_update), capabilities: PERF_PMU_CAP_NO_INTERRUPT | PERF_PMU_CAP_NO_EXCLUDE, attr_update: core::ptr::null() };

unsafe extern "C" fn msr_init() -> i32 {
    if !boot_cpu_has(X86_FEATURE_TSC) { return 0; }
    msr_mask = perf_msr_probe(msr.as_mut_ptr(), PERF_MSR_EVENT_MAX as u32, true, core::ptr::null_mut());
    perf_pmu_register(&raw mut pmu_msr, b"msr\0".as_ptr(), -1);
    0
}

// device_initcall(msr_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
