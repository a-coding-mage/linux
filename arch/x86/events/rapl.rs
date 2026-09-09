// SPDX-License-Identifier: GPL-2.0-only
/* Support Intel/AMD RAPL energy consumption counters. */

// Kernel dependencies supplied by the surrounding repository are intentionally
// left external, as in the original implementation.

const RAPL_EVENT_MASK: u64 = 0xff;
const RAPL_CNTR_WIDTH: i32 = 32;
const RAPL_MSR_MASK: u32 = 0xffff_ffff;

#[repr(i32)]
enum PerfRaplPkgEvents { PerfRaplPp0 = 0, PerfRaplPkg, PerfRaplRam, PerfRaplPp1, PerfRaplPsys, PerfRaplPkgEventsMax }
const NR_RAPL_PKG_DOMAINS: usize = PerfRaplPkgEvents::PerfRaplPkgEventsMax as usize;
const PERF_RAPL_CORE: usize = 0;
const PERF_RAPL_CORE_EVENTS_MAX: usize = 1;
const NR_RAPL_CORE_DOMAINS: usize = PERF_RAPL_CORE_EVENTS_MAX;

static RAPL_PKG_DOMAIN_NAMES: [&str; NR_RAPL_PKG_DOMAINS] = ["pp0-core", "package", "dram", "pp1-gpu", "psys"];
static RAPL_CORE_DOMAIN_NAME: &str = "core";

#[repr(C)]
struct RaplPmu { lock: RawSpinlock, n_active: i32, cpu: i32, active_list: ListHead, pmu: *mut Pmu, timer_interval: Ktime, hrtimer: Hrtimer }
#[repr(C)]
struct RaplPmus { pmu: Pmu, nr_rapl_pmu: u32, cntr_mask: u32, rapl_pmu: *mut *mut RaplPmu }
#[repr(i32)]
enum RaplUnitQuirk { None, IntelHsw, IntelSpr }
#[repr(C)]
struct RaplModel { rapl_pkg_msrs: *mut PerfMsr, rapl_core_msrs: *mut PerfMsr, pkg_events: C_ulong, core_events: C_ulong, msr_power_unit: u32, unit_quirk: RaplUnitQuirk }

static mut RAPL_PKG_HW_UNIT: [i32; NR_RAPL_PKG_DOMAINS] = [0; NR_RAPL_PKG_DOMAINS];
static mut RAPL_CORE_HW_UNIT: i32 = 0;
static mut RAPL_TIMER_MS: u64 = 0;
static mut RAPL_PUS_PKG: *mut RaplPmus = core::ptr::null_mut();
static mut RAPL_PUS_CORE: *mut RaplPmus = core::ptr::null_mut();
static mut RAPL_MODEL: *mut RaplModel = core::ptr::null_mut();

#[inline]
unsafe fn get_rapl_pmu_idx(cpu: i32, scope: i32) -> u32 {
    match scope { PERF_PMU_SCOPE_PKG => topology_logical_package_id(cpu), PERF_PMU_SCOPE_DIE => topology_logical_die_id(cpu), PERF_PMU_SCOPE_CORE => topology_logical_core_id(cpu), _ => (-EINVAL) as u32 }
}
#[inline] unsafe fn rapl_read_counter(event: *mut PerfEvent) -> u64 { let mut raw = 0; rdmsrq((*event).hw.event_base, &mut raw); raw }
#[inline] unsafe fn rapl_scale(v: u64, event: *mut PerfEvent) -> u64 { let mut hw = RAPL_PKG_HW_UNIT[((*event).hw.config - 1) as usize]; if (*(*event).pmu).scope == PERF_PMU_SCOPE_CORE { hw = RAPL_CORE_HW_UNIT; } v << (32 - hw) }
unsafe fn rapl_event_update(event: *mut PerfEvent) -> u64 {
    let hwc = &mut (*event).hw; let prev = local64_read(&hwc.prev_count); let mut new = 0;
    loop { rdmsrq(hwc.event_base, &mut new); if local64_try_cmpxchg(&mut hwc.prev_count, prev, new) { break; } }
    let delta = (((new << RAPL_CNTR_WIDTH) as i64 - (prev << RAPL_CNTR_WIDTH) as i64) >> RAPL_CNTR_WIDTH) as u64;
    local64_add(rapl_scale(delta, event), &mut (*event).count); new
}
unsafe fn rapl_start_hrtimer(pmu: *mut RaplPmu) { hrtimer_start(&mut (*pmu).hrtimer, (*pmu).timer_interval, HRTIMER_MODE_REL_PINNED); }
unsafe fn rapl_hrtimer_handle(hrtimer: *mut Hrtimer) -> HrtimerRestart {
    let pmu = container_of!(hrtimer, RaplPmu, hrtimer); if (*pmu).n_active == 0 { return HrtimerRestart::NoRestart; }
    let mut flags = 0; raw_spin_lock_irqsave(&mut (*pmu).lock, &mut flags);
    list_for_each_entry!(event, &(*pmu).active_list, active_entry, { rapl_event_update(event); });
    raw_spin_unlock_irqrestore(&mut (*pmu).lock, flags); hrtimer_forward_now(hrtimer, (*pmu).timer_interval); HrtimerRestart::Restart
}
unsafe fn rapl_hrtimer_init(pmu: *mut RaplPmu) { hrtimer_setup(&mut (*pmu).hrtimer, rapl_hrtimer_handle, CLOCK_MONOTONIC, HRTIMER_MODE_REL); }
unsafe fn __rapl_pmu_event_start(pmu: *mut RaplPmu, event: *mut PerfEvent) { if warn_on_once!((*event).hw.state & PERF_HES_STOPPED == 0) { return; } (*event).hw.state=0; list_add_tail(&mut (*event).active_entry,&mut (*pmu).active_list); local64_set(&mut (*event).hw.prev_count,rapl_read_counter(event)); (*pmu).n_active+=1; if (*pmu).n_active==1 {rapl_start_hrtimer(pmu);} }
unsafe fn rapl_pmu_event_start(event: *mut PerfEvent, _mode: i32) { let p=(*event).pmu_private as *mut RaplPmu; let mut f=0; raw_spin_lock_irqsave(&mut (*p).lock,&mut f); __rapl_pmu_event_start(p,event); raw_spin_unlock_irqrestore(&mut (*p).lock,f); }
unsafe fn rapl_pmu_event_stop(event: *mut PerfEvent, mode: i32) { let p=(*event).pmu_private as *mut RaplPmu; let h=&mut (*event).hw; let mut f=0; raw_spin_lock_irqsave(&mut (*p).lock,&mut f); if h.state&PERF_HES_STOPPED==0 { (*p).n_active-=1; if (*p).n_active==0 {hrtimer_cancel(&mut (*p).hrtimer);} list_del(&mut (*event).active_entry); h.state|=PERF_HES_STOPPED; } if mode&PERF_EF_UPDATE!=0 && h.state&PERF_HES_UPTODATE==0 {rapl_event_update(event);h.state|=PERF_HES_UPTODATE;} raw_spin_unlock_irqrestore(&mut (*p).lock,f); }
unsafe fn rapl_pmu_event_add(event:*mut PerfEvent,mode:i32)->i32 { let p=(*event).pmu_private as *mut RaplPmu; let mut f=0; raw_spin_lock_irqsave(&mut (*p).lock,&mut f); (*event).hw.state=PERF_HES_UPTODATE|PERF_HES_STOPPED; if mode&PERF_EF_START!=0 {__rapl_pmu_event_start(p,event);} raw_spin_unlock_irqrestore(&mut (*p).lock,f); 0 }
unsafe fn rapl_pmu_event_del(event:*mut PerfEvent,_flags:i32){rapl_pmu_event_stop(event,PERF_EF_UPDATE)}
unsafe fn rapl_pmu_event_read(event:*mut PerfEvent){rapl_event_update(event)}

#[repr(C)] struct Attribute { _private: [u8; 0] }
#[repr(C)] struct AttributeGroup { name: *const u8, attrs: *mut *mut Attribute }
#[repr(C)] struct PerfMsr { msr: u32, attr_group: *const AttributeGroup, test: Option<unsafe extern "C" fn(i32,*mut core::ffi::c_void)->bool>, add: bool, mask: u32 }

// The following declarations preserve the source's event attribute and group
// topology; their concrete kernel representations are provided externally.
extern "C" {
    static mut intel_rapl_msrs: [PerfMsr; NR_RAPL_PKG_DOMAINS];
    static mut intel_rapl_spr_msrs: [PerfMsr; NR_RAPL_PKG_DOMAINS];
    static mut amd_rapl_pkg_msrs: [PerfMsr; NR_RAPL_PKG_DOMAINS];
    static mut amd_rapl_core_msrs: [PerfMsr; NR_RAPL_CORE_DOMAINS];
    fn perf_msr_probe(msrs:*mut PerfMsr,n:i32,flag:bool,data:*mut core::ffi::c_void)->u32;
    fn perf_pmu_register(pmu:*mut Pmu,name:*const u8,cpu:i32)->i32;
    fn perf_pmu_unregister(pmu:*mut Pmu);
    fn topology_max_packages()->i32; fn topology_max_dies_per_package()->i32; fn topology_num_cores_per_package()->i32;
}

#[repr(C)] struct Pmu { scope:i32, attr_groups:*const *const AttributeGroup, attr_update:*const *const AttributeGroup, task_ctx_nr:i32, event_init:Option<unsafe fn(*mut PerfEvent)->i32>, add:Option<unsafe fn(*mut PerfEvent,i32)->i32>, del:Option<unsafe fn(*mut PerfEvent,i32)>, start:Option<unsafe fn(*mut PerfEvent,i32)>, stop:Option<unsafe fn(*mut PerfEvent,i32)>, read:Option<unsafe fn(*mut PerfEvent)> }
#[repr(C)] struct RawSpinlock { _private:[u8;0] }
#[repr(C)] struct ListHead { next:*mut ListHead, prev:*mut ListHead }
#[repr(C)] struct Hrtimer { _private:[u8;0] }
type Ktime=i64; type C_ulong=usize;
#[repr(C)] struct HwPerfEvent { state:i32, config:u64, event_base:u32, idx:i32, prev_count:i64 }
#[repr(C)] struct PerfEvent { attr: PerfEventAttr, pmu:*mut Pmu, pmu_private:*mut core::ffi::c_void, cpu:i32, hw:HwPerfEvent, count:i64, active_entry:ListHead }
#[repr(C)] struct PerfEventAttr { config:u64, type_:u32, sample_period:u64 }
#[repr(i32)] enum HrtimerRestart { NoRestart, Restart }
const PERF_PMU_SCOPE_PKG:i32=1; const PERF_PMU_SCOPE_DIE:i32=2; const PERF_PMU_SCOPE_CORE:i32=3;
const PERF_HES_STOPPED:i32=1; const PERF_HES_UPTODATE:i32=2; const PERF_EF_START:i32=1; const PERF_EF_UPDATE:i32=2; const EINVAL:i32=22;

// Build-time kernel constants and helpers remain external dependencies.
extern "C" {
 fn rdmsrq(msr:u32,out:*mut u64); fn local64_read(v:*const i64)->u64; fn local64_set(v:*mut i64,x:u64); fn local64_add(x:u64,v:*mut i64)->(); fn local64_try_cmpxchg(v:*mut i64,old:u64,new:u64)->bool;
 fn topology_logical_package_id(cpu:i32)->u32; fn topology_logical_die_id(cpu:i32)->u32; fn topology_logical_core_id(cpu:i32)->u32;
 fn hrtimer_start(h:*mut Hrtimer,t:Ktime,m:i32); fn hrtimer_forward_now(h:*mut Hrtimer,t:Ktime); fn hrtimer_cancel(h:*mut Hrtimer); fn hrtimer_setup(h:*mut Hrtimer,f:unsafe fn(*mut Hrtimer)->HrtimerRestart,c:i32,m:i32);
 fn raw_spin_lock_irqsave(l:*mut RawSpinlock,f:*mut usize); fn raw_spin_unlock_irqrestore(l:*mut RawSpinlock,f:usize); fn list_add_tail(e:*mut ListHead,h:*mut ListHead); fn list_del(e:*mut ListHead);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
