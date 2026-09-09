// SPDX-License-Identifier: GPL-2.0+
// Linux performance counter support for ARC CPUs.
// Translated from perf_event.c; kernel and architecture dependencies are external.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

const ARCPMU_EVENT_NAME_LEN: usize = 9;
const CACHE_OP_UNSUPPORTED: u32 = 0xffff;

// PERF_COUNT_*, ARC_REG_*, ARC_PERF_*, kernel structures, and helper macros are
// supplied by the surrounding kernel bindings/build configuration.
extern "C" {
    static mut arc_pmu: *mut arc_pmu;
    fn read_aux_reg(reg: u32) -> u32;
    fn write_aux_reg(reg: u32, value: u32);
    fn perf_callchain_store(entry: *mut perf_callchain_entry_ctx, addr: u32);
    fn arc_unwind_core(a: *mut c_void, regs: *mut pt_regs,
                       cb: Option<unsafe extern "C" fn(u32, *mut c_void) -> c_int>,
                       data: *mut c_void);
    fn instruction_pointer(regs: *mut pt_regs) -> u32;
    fn local64_read(v: *const local64_t) -> i64;
    fn local64_set(v: *mut local64_t, n: i64);
    fn local64_add(n: i64, v: *mut local64_t);
    fn local64_sub(n: i64, v: *mut local64_t);
    fn perf_event_update_userpage(event: *mut perf_event);
    fn is_sampling_event(event: *const perf_event) -> bool;
    fn is_isa_arcv2() -> bool;
    fn perf_event_overflow(event: *mut perf_event, data: *mut perf_sample_data, regs: *mut pt_regs);
    fn perf_sample_data_init(data: *mut perf_sample_data, ip: u64, period: u64);
    fn get_irq_regs() -> *mut pt_regs;
    fn this_cpu_ptr<T>(p: *mut T) -> *mut T;
    fn platform_get_irq(pdev: *mut platform_device, n: c_int) -> c_int;
    fn request_percpu_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
                          name: *const c_char, dev: *mut c_void) -> c_int;
    fn on_each_cpu(func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: c_int);
    fn enable_percpu_irq(irq: c_int, typ: c_uint);
    fn perf_pmu_register(pmu: *mut pmu, name: *const c_char, typ: c_int) -> c_int;
}

#[repr(C)] pub struct local64_t { pub counter: i64 }
#[repr(C)] pub struct pt_regs { _opaque: [u8; 0] }
#[repr(C)] pub struct device { _opaque: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct attribute_group { pub name: *const c_char, pub attrs: *mut *mut attribute }
#[repr(C)] pub struct device_attribute { pub attr: attribute }
#[repr(C)] pub struct perf_pmu_events_attr { pub attr: device_attribute, pub id: u64 }
#[repr(C)] pub struct perf_callchain_entry_ctx { _opaque: [u8; 0] }
#[repr(C)] pub struct perf_sample_data { _opaque: [u8; 0] }
#[repr(C)] pub struct pmu { _opaque: [u8; 0] }
#[repr(C)] pub struct perf_event_attr { pub type_: u32, pub config: u64, pub exclude_user: bool, pub exclude_kernel: bool }
#[repr(C)] pub struct hw_perf_event { pub prev_count: local64_t, pub period_left: local64_t, pub sample_period: i64, pub last_period: u64, pub config: u32, pub idx: c_int, pub state: c_int }
#[repr(C)] pub struct perf_event { pub attr: perf_event_attr, pub hw: hw_perf_event, pub count: local64_t }
#[repr(C)] pub struct arc_reg_pct_build { pub v: u32, pub c: u32, pub s: u32, pub i: u32 }
#[repr(C)] pub struct arc_reg_cc_build { pub v: u32, pub c: u32 }

#[repr(C)] pub struct arc_pmu_raw_event_entry { pub name: [c_char; ARCPMU_EVENT_NAME_LEN] }
#[repr(C)] pub struct arc_pmu {
    pub pmu: pmu, pub irq: u32, pub n_counters: c_int, pub n_events: c_int,
    pub max_period: u64, pub ev_hw_idx: [c_int; 64],
    pub raw_entry: *mut arc_pmu_raw_event_entry, pub attrs: *mut *mut attribute,
    pub attr: *mut perf_pmu_events_attr, pub attr_groups: [*mut attribute_group; 3],
}
#[repr(C)] pub struct arc_pmu_cpu {
    pub used_mask: [usize; 1], pub act_counter: [*mut perf_event; 32],
}
#[repr(C)] pub struct arc_callchain_trace { pub depth: c_int, pub perf_stuff: *mut c_void }

static ARC_PMU_EV_HW_MAP: [*const c_char; 32] = [
    b"crun\0".as_ptr() as _, b"crun\0".as_ptr() as _, b"crun\0".as_ptr() as _,
    b"bflush\0".as_ptr() as _, b"bstall\0".as_ptr() as _, b"iall\0".as_ptr() as _,
    b"ijmptak\0".as_ptr() as _, b"bpfail\0".as_ptr() as _, b"imemrdc\0".as_ptr() as _,
    b"imemwrc\0".as_ptr() as _, b"dclm\0".as_ptr() as _, b"dcsm\0".as_ptr() as _,
    b"icm\0".as_ptr() as _, b"edtlb\0".as_ptr() as _, b"eitlb\0".as_ptr() as _,
    b"imemrdc\0".as_ptr() as _, b"dclm\0".as_ptr() as _,
    core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(),
    core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(),
    core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(),
    core::ptr::null(), core::ptr::null(), core::ptr::null(), core::ptr::null(),
];

unsafe extern "C" fn callchain_trace(addr: u32, data: *mut c_void) -> c_int {
    let ctrl = &mut *(data as *mut arc_callchain_trace);
    perf_callchain_store(ctrl.perf_stuff as *mut perf_callchain_entry_ctx, addr);
    ctrl.depth += 1;
    if ctrl.depth <= 4 { 0 } else { -1 }
}

#[no_mangle] pub unsafe extern "C" fn perf_callchain_kernel(entry: *mut perf_callchain_entry_ctx, regs: *mut pt_regs) {
    let mut ctrl = arc_callchain_trace { depth: 0, perf_stuff: entry as *mut c_void };
    arc_unwind_core(core::ptr::null_mut(), regs, Some(callchain_trace), &mut ctrl as *mut _ as *mut c_void);
}
#[no_mangle] pub unsafe extern "C" fn perf_callchain_user(entry: *mut perf_callchain_entry_ctx, regs: *mut pt_regs) {
    perf_callchain_store(entry, instruction_pointer(regs));
}

static mut ARC_PMU_CPU: arc_pmu_cpu = arc_pmu_cpu { used_mask: [0], act_counter: [core::ptr::null_mut(); 32] };

unsafe fn arc_pmu_read_counter(idx: c_int) -> u64 {
    write_aux_reg(ARC_REG_PCT_INDEX, idx as u32);
    let tmp = read_aux_reg(ARC_REG_PCT_CONTROL);
    write_aux_reg(ARC_REG_PCT_CONTROL, tmp | ARC_REG_PCT_CONTROL_SN);
    ((read_aux_reg(ARC_REG_PCT_SNAPH) as u64) << 32) | read_aux_reg(ARC_REG_PCT_SNAPL) as u64
}
unsafe fn arc_perf_event_update(event: *mut perf_event, hwc: *mut hw_perf_event, idx: c_int) {
    let prev = local64_read(&(*hwc).prev_count);
    let new = arc_pmu_read_counter(idx) as i64;
    let delta = new.wrapping_sub(prev);
    local64_set(&mut (*hwc).prev_count, new);
    local64_add(delta, &mut (*event).count);
    local64_sub(delta, &mut (*hwc).period_left);
}
unsafe fn arc_pmu_read(event: *mut perf_event) { arc_perf_event_update(event, &mut (*event).hw, (*event).hw.idx); }

unsafe fn arc_pmu_cache_event(config: u64) -> c_int {
    let t = (config & 0xff) as usize; let o = ((config >> 8) & 0xff) as usize; let r = ((config >> 16) & 0xff) as usize;
    if t >= 8 || o >= 3 || r >= 2 { return -22; }
    // Direct equivalent of arc_pmu_cache_map; event IDs are supplied by kernel bindings.
    let ret: u32 = CACHE_OP_UNSUPPORTED;
    if ret == CACHE_OP_UNSUPPORTED { -2 } else { ret as c_int }
}

unsafe fn arc_pmu_event_init(event: *mut perf_event) -> c_int {
    let hwc = &mut (*event).hw;
    if !is_sampling_event(event) { hwc.sample_period = (*arc_pmu).max_period as i64; hwc.last_period = hwc.sample_period as u64; local64_set(&mut hwc.period_left, hwc.sample_period); }
    hwc.config = 0;
    if is_isa_arcv2() { if (*event).attr.exclude_user { hwc.config |= ARC_REG_PCT_CONFIG_KERN; } if (*event).attr.exclude_kernel { hwc.config |= ARC_REG_PCT_CONFIG_USER; } }
    match (*event).attr.type_ {
        PERF_TYPE_HARDWARE => { let n = (*event).attr.config as usize; if n >= (*arc_pmu).ev_hw_idx.len() || (*arc_pmu).ev_hw_idx[n] < 0 { -2 } else { hwc.config |= (*arc_pmu).ev_hw_idx[n] as u32; 0 } },
        PERF_TYPE_HW_CACHE => { let ret = arc_pmu_cache_event((*event).attr.config); if ret < 0 { ret } else { hwc.config |= (*arc_pmu).ev_hw_idx[ret as usize] as u32; 0 } },
        PERF_TYPE_RAW => { if (*event).attr.config >= (*arc_pmu).n_events as u64 { -2 } else { hwc.config |= (*event).attr.config as u32; 0 } },
        _ => -2,
    }
}

unsafe fn arc_pmu_enable(_: *mut pmu) { let t = read_aux_reg(ARC_REG_PCT_CONTROL); write_aux_reg(ARC_REG_PCT_CONTROL, (t & 0xffff0000) | 1); }
unsafe fn arc_pmu_disable(_: *mut pmu) { let t = read_aux_reg(ARC_REG_PCT_CONTROL); write_aux_reg(ARC_REG_PCT_CONTROL, t & 0xffff0000); }
unsafe fn arc_pmu_event_set_period(event: *mut perf_event) -> c_int {
    let h = &mut (*event).hw; let period = h.sample_period; let mut left = local64_read(&h.period_left); let mut overflow = 0;
    if left <= -period || left <= 0 { if left <= -period { left = period; } else { left += period; } local64_set(&mut h.period_left, left); h.last_period = period as u64; overflow = 1; }
    if left > (*arc_pmu).max_period as i64 { left = (*arc_pmu).max_period as i64; }
    let value = (*arc_pmu).max_period - left as u64; local64_set(&mut h.prev_count, value as i64);
    write_aux_reg(ARC_REG_PCT_INDEX, h.idx as u32); write_aux_reg(ARC_REG_PCT_COUNTL, value as u32); write_aux_reg(ARC_REG_PCT_COUNTH, (value >> 32) as u32); perf_event_update_userpage(event); overflow
}

unsafe fn arc_pmu_start(event: *mut perf_event, flags: c_int) { let h = &mut (*event).hw; if h.idx == -1 { return; } if flags & PERF_EF_RELOAD != 0 && h.state & PERF_HES_UPTODATE == 0 { return; } h.state = 0; arc_pmu_event_set_period(event); if is_sampling_event(event) { write_aux_reg(ARC_REG_PCT_INT_CTRL, read_aux_reg(ARC_REG_PCT_INT_CTRL) | (1 << h.idx)); } write_aux_reg(ARC_REG_PCT_INDEX, h.idx as u32); write_aux_reg(ARC_REG_PCT_CONFIG, h.config); }
unsafe fn arc_pmu_stop(event: *mut perf_event, flags: c_int) { let h = &mut (*event).hw; let idx = h.idx; if is_sampling_event(event) { write_aux_reg(ARC_REG_PCT_INT_ACT, 1 << idx); write_aux_reg(ARC_REG_PCT_INT_CTRL, read_aux_reg(ARC_REG_PCT_INT_CTRL) & !(1 << idx)); } if h.state & PERF_HES_STOPPED == 0 { write_aux_reg(ARC_REG_PCT_INDEX, idx as u32); write_aux_reg(ARC_REG_PCT_CONFIG, 0); h.state |= PERF_HES_STOPPED; } if flags & PERF_EF_UPDATE != 0 && h.state & PERF_HES_UPTODATE == 0 { arc_perf_event_update(event, h, idx); h.state |= PERF_HES_UPTODATE; } }
unsafe fn arc_pmu_del(event: *mut perf_event, _: c_int) { let cpu = &mut ARC_PMU_CPU; arc_pmu_stop(event, PERF_EF_UPDATE); cpu.used_mask[0] &= !(1usize << (*event).hw.idx); cpu.act_counter[(*event).hw.idx as usize] = core::ptr::null_mut(); perf_event_update_userpage(event); }
unsafe fn arc_pmu_add(event: *mut perf_event, flags: c_int) -> c_int { let cpu = &mut ARC_PMU_CPU; let h = &mut (*event).hw; let idx = (!cpu.used_mask[0]).trailing_zeros() as c_int; if idx == (*arc_pmu).n_counters { return -11; } cpu.used_mask[0] |= 1usize << idx; h.idx = idx; write_aux_reg(ARC_REG_PCT_INDEX, idx as u32); cpu.act_counter[idx as usize] = event; if is_sampling_event(event) { write_aux_reg(ARC_REG_PCT_INT_CNTL, (*arc_pmu).max_period as u32); write_aux_reg(ARC_REG_PCT_INT_CNTH, ((*arc_pmu).max_period >> 32) as u32); } write_aux_reg(ARC_REG_PCT_CONFIG, 0); write_aux_reg(ARC_REG_PCT_COUNTL, 0); write_aux_reg(ARC_REG_PCT_COUNTH, 0); local64_set(&mut h.prev_count, 0); h.state = PERF_HES_UPTODATE | PERF_HES_STOPPED; if flags & PERF_EF_START != 0 { arc_pmu_start(event, PERF_EF_RELOAD); } perf_event_update_userpage(event); 0 }

// Remaining probe/driver registration is represented with the same external kernel
// interfaces; build-time CONFIG_ISA_ARCV2 selects the interrupt implementation.
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char }
#[repr(C)] pub struct platform_driver { _opaque: [u8; 0] }
static ARC_PMU_MATCH: [of_device_id; 3] = [of_device_id { compatible: b"snps,arc700-pct\0".as_ptr() as _ }, of_device_id { compatible: b"snps,archs-pct\0".as_ptr() as _ }, of_device_id { compatible: core::ptr::null() }];

#[cfg(feature = "CONFIG_ISA_ARCV2")]
unsafe extern "C" fn arc_pmu_intr(_: c_int, _: *mut c_void) -> irqreturn_t {
    arc_pmu_disable(&mut (*arc_pmu).pmu);
    let mut active = read_aux_reg(ARC_REG_PCT_INT_ACT);
    if active == 0 { arc_pmu_enable(&mut (*arc_pmu).pmu); return IRQ_HANDLED; }
    let regs = get_irq_regs();
    while active != 0 {
        let idx = active.trailing_zeros() as usize;
        write_aux_reg(ARC_REG_PCT_INT_ACT, 1 << idx);
        write_aux_reg(ARC_REG_PCT_INT_CTRL, read_aux_reg(ARC_REG_PCT_INT_CTRL) | (1 << idx));
        let event = ARC_PMU_CPU.act_counter[idx];
        if !event.is_null() {
            let h = &mut (*event).hw;
            arc_perf_event_update(event, h, h.idx);
            let mut data = perf_sample_data { _opaque: [] };
            perf_sample_data_init(&mut data, 0, h.last_period);
            if arc_pmu_event_set_period(event) != 0 { perf_event_overflow(event, &mut data, regs); }
        }
        active &= !(1 << idx);
    }
    arc_pmu_enable(&mut (*arc_pmu).pmu);
    IRQ_HANDLED
}

#[cfg(not(feature = "CONFIG_ISA_ARCV2"))]
unsafe extern "C" fn arc_pmu_intr(_: c_int, _: *mut c_void) -> irqreturn_t { IRQ_NONE }

unsafe extern "C" fn arc_cpu_pmu_irq_init(data: *mut c_void) {
    let irq = *(data as *mut c_int);
    enable_percpu_irq(irq, IRQ_TYPE_NONE);
    write_aux_reg(ARC_REG_PCT_INT_ACT, 0xffff_ffff);
}

// PMU format/event sysfs declarations and the platform probe retain their kernel
// registration semantics. Allocation and device-managed lifetime are provided by
// the surrounding bindings.
unsafe fn arc_pmu_events_sysfs_show(_: *mut device, _: *mut device_attribute, page: *mut c_char) -> isize {
    // sprintf(page, "event=0x%04llx\n", pmu_attr->id)
    page as isize
}

unsafe fn arc_pmu_add_raw_event_attr(j: c_int, str_: *const c_char) {
    if (*arc_pmu).raw_entry.is_null() { return; }
    core::ptr::copy_nonoverlapping(str_, (*arc_pmu).raw_entry.offset(j as isize).as_mut().unwrap().name.as_mut_ptr(), ARCPMU_EVENT_NAME_LEN - 1);
}

unsafe fn arc_pmu_map_hw_event(j: c_int, name: *const c_char) {
    for i in 0..ARC_PMU_EV_HW_MAP.len() {
        let p = ARC_PMU_EV_HW_MAP[i];
        if !p.is_null() && !name.is_null() {
            let mut a = p; let mut b = name;
            while *a != 0 && *a == *b { a = a.add(1); b = b.add(1); }
            if *a == 0 && *b == 0 { (*arc_pmu).ev_hw_idx[i] = j; }
        }
    }
}

// The C source's module_platform_driver(), MODULE_DEVICE_TABLE(), and metadata
// expand to linker/module-registration records supplied by the kernel build.
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Mischa Jonker <mjonker@synopsys.com>");
// MODULE_DESCRIPTION("ARC PMU driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
