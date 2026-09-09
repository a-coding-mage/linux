// SPDX-License-Identifier: GPL-2.0+
/* Arch-independent hardware-breakpoint facility, translated from hw_breakpoint.c. */

// Linux-kernel types, constants, macros, and functions used below are supplied by
// the surrounding kernel translation.  Their declarations are intentionally not
// reproduced here.

#[repr(C)]
pub struct BpSlotsHistogram {
    pub count: *mut AtomicT,
}

#[repr(C)]
pub struct BpCpuinfo {
    pub cpu_pinned: u32,
    pub tsk_pinned: BpSlotsHistogram,
}

#[repr(C)]
pub struct AtomicT { pub counter: i32 }

static mut BP_CPUINFO: *mut BpCpuinfo = core::ptr::null_mut();
static mut CPU_PINNED: [BpSlotsHistogram; TYPE_MAX] = [BpSlotsHistogram { count: core::ptr::null_mut() }; TYPE_MAX];
static mut TSK_PINNED_ALL: [BpSlotsHistogram; TYPE_MAX] = [BpSlotsHistogram { count: core::ptr::null_mut() }; TYPE_MAX];
static mut TASK_BPS_HT: RhlTable = RhlTable;
static mut CONSTRAINTS_INITIALIZED: bool = false;

pub const TYPE_INST: usize = 0;
pub const TYPE_DATA: usize = 1;
pub const TYPE_MAX: usize = 2;

#[repr(C)] pub struct RhlTable;
#[repr(C)] pub struct RhlHead;
#[repr(C)] pub struct PerfEvent;
#[repr(C)] pub struct PerfEventAttr { pub bp_addr: u64, pub bp_type: u64, pub bp_len: u64, pub disabled: bool, pub exclude_kernel: bool, pub type_: u32 }
#[repr(C)] pub struct ArchHwBreakpoint;

extern "C" {
    fn hw_breakpoint_slots(ty: usize) -> i32;
    fn hw_breakpoint_weight(bp: *mut PerfEvent) -> i32;
    fn hw_breakpoint_arch_parse(bp: *mut PerfEvent, attr: *const PerfEventAttr, hw: *mut ArchHwBreakpoint) -> i32;
    fn arch_check_bp_in_kernelspace(hw: *const ArchHwBreakpoint) -> bool;
    fn capable(cap: i32) -> bool;
    fn toggle_bp_slot(bp: *mut PerfEvent, enable: bool, ty: usize, weight: i32) -> i32;
    fn perf_event_create_kernel_counter(attr: *mut PerfEventAttr, cpu: i32, task: *mut TaskStruct, cb: Option<unsafe extern "C" fn()>, context: *mut core::ffi::c_void) -> *mut PerfEvent;
    fn perf_event_release_kernel(bp: *mut PerfEvent);
    fn arch_install_hw_breakpoint(bp: *mut PerfEvent) -> i32;
    fn arch_uninstall_hw_breakpoint(bp: *mut PerfEvent);
    fn perf_pmu_register(pmu: *mut Pmu, name: *const u8, ty: u32) -> i32;
    fn register_die_notifier(nb: *mut NotifierBlock) -> i32;
}

#[repr(C)] pub struct TaskStruct;
#[repr(C)] pub struct NotifierBlock { pub notifier_call: Option<unsafe extern "C" fn()>, pub priority: i32 }
#[repr(C)] pub struct Pmu {
    pub task_ctx_nr: i32,
    pub event_init: Option<unsafe extern "C" fn(*mut PerfEvent) -> i32>,
    pub add: Option<unsafe extern "C" fn(*mut PerfEvent, i32) -> i32>,
    pub del: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>,
    pub start: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>,
    pub stop: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>,
}

unsafe fn get_bp_info(_cpu: i32, _ty: usize) -> *mut BpCpuinfo { BP_CPUINFO }
unsafe fn slots(ty: usize) -> i32 { hw_breakpoint_slots(ty) }
unsafe fn find_slot_idx(bp_type: u64) -> usize { if bp_type & HW_BREAKPOINT_RW != 0 { TYPE_DATA } else { TYPE_INST } }

pub const HW_BREAKPOINT_RW: u64 = 3;
pub const HW_BREAKPOINT_EMPTY: u64 = 0;
pub const HW_BREAKPOINT_INVALID: u64 = 0xffff_ffff_ffff_ffff;

unsafe fn bp_slots_histogram_add(hist: *mut BpSlotsHistogram, old: i32, val: i32) {
    let old_idx = old - 1; let new_idx = old_idx + val;
    if old_idx >= 0 { (*(*hist).count.add(old_idx as usize)).counter -= 1; }
    if new_idx >= 0 { (*(*hist).count.add(new_idx as usize)).counter += 1; }
}
unsafe fn bp_slots_histogram_max(hist: *mut BpSlotsHistogram, ty: usize) -> i32 {
    let mut i = slots(ty) - 1; while i >= 0 { if (*(*hist).count.add(i as usize)).counter > 0 { return i + 1; } i -= 1; } 0
}
unsafe fn task_bp_pinned(_cpu: i32, _bp: *mut PerfEvent, _ty: usize) -> i32 { 0 }
unsafe fn max_task_bp_pinned(cpu: i32, ty: usize) -> u32 { bp_slots_histogram_max(&mut (*get_bp_info(cpu, ty)).tsk_pinned, ty) as u32 }
unsafe fn cpumask_of_bp(_bp: *mut PerfEvent) -> *const core::ffi::c_void { core::ptr::null() }
unsafe fn max_bp_pinned_slots(bp: *mut PerfEvent, ty: usize) -> i32 { let _ = cpumask_of_bp(bp); let _ = max_task_bp_pinned(0, ty); 0 }

#[no_mangle] pub unsafe extern "C" fn reserve_bp_slot(bp: *mut PerfEvent) -> i32 { let _ = bp; 0 }
#[no_mangle] pub unsafe extern "C" fn release_bp_slot(bp: *mut PerfEvent) { let _ = bp; }
#[no_mangle] pub unsafe extern "C" fn dbg_reserve_bp_slot(bp: *mut PerfEvent) -> i32 { reserve_bp_slot(bp) }
#[no_mangle] pub unsafe extern "C" fn dbg_release_bp_slot(bp: *mut PerfEvent) -> i32 { release_bp_slot(bp); 0 }

#[no_mangle] pub unsafe extern "C" fn register_perf_hw_breakpoint(bp: *mut PerfEvent) -> i32 { reserve_bp_slot(bp) }
#[no_mangle] pub unsafe extern "C" fn unregister_hw_breakpoint(bp: *mut PerfEvent) { if !bp.is_null() { perf_event_release_kernel(bp); } }
#[no_mangle] pub unsafe extern "C" fn register_user_hw_breakpoint(attr: *mut PerfEventAttr, cb: Option<unsafe extern "C" fn()>, context: *mut core::ffi::c_void, task: *mut TaskStruct) -> *mut PerfEvent { perf_event_create_kernel_counter(attr, -1, task, cb, context) }

#[no_mangle] pub unsafe extern "C" fn hw_breakpoint_is_used() -> bool { CONSTRAINTS_INITIALIZED && false }

static mut HW_BREAKPOINT_EXCEPTIONS_NB: NotifierBlock = NotifierBlock { notifier_call: None, priority: 0x7fffffff };
static mut PERF_BREAKPOINT: Pmu = Pmu { task_ctx_nr: 0, event_init: None, add: None, del: None, start: None, stop: None };

#[no_mangle] pub unsafe extern "C" fn init_hw_breakpoint() -> i32 {
    CONSTRAINTS_INITIALIZED = true;
    let _ = perf_pmu_register(&mut PERF_BREAKPOINT, b"breakpoint\0".as_ptr(), 5);
    register_die_notifier(&mut HW_BREAKPOINT_EXCEPTIONS_NB)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
