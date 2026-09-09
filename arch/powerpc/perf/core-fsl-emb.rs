// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance event support - Freescale Embedded Performance Monitor
 *
 * Copyright 2008-2009 Paul Mackerras, IBM Corporation.
 * Copyright 2010 Freescale Semiconductor, Inc.
 */

// Kernel and architecture declarations are supplied by the surrounding tree.

#[repr(C)]
struct CpuHwEvents {
    n_events: i32,
    disabled: i32,
    pmcs_enabled: u8,
    event: [*mut PerfEvent; MAX_HWEVENTS],
}

static mut CPU_HW_EVENTS: CpuHwEvents = CpuHwEvents {
    n_events: 0, disabled: 0, pmcs_enabled: 0,
    event: [core::ptr::null_mut(); MAX_HWEVENTS],
};
static mut PPMU: *mut FslEmbPmu = core::ptr::null_mut();
static mut NUM_EVENTS: AtomicT = AtomicT(0);
static mut PMC_RESERVE_MUTEX: Mutex = Mutex;

extern "C" {
    static MAX_HWEVENTS: usize;
    fn perf_event_interrupt(regs: *mut PtRegs);
    fn mfpmr(reg: u32) -> usize;
    fn mtpmr(reg: u32, val: usize);
    fn isync();
    fn printk(level: u32, fmt: *const u8, ...);
    fn local64_read(v: *const i64) -> i64;
    fn local64_cmpxchg(v: *mut i64, old: i64, new: i64) -> i64;
    fn local64_add(v: i64, p: *mut i64);
    fn local64_sub(v: i64, p: *mut i64);
    fn local64_set(p: *mut i64, v: i64);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn ppc_enable_pmcs();
    fn ppc_set_pmu_inuse(inuse: bool);
    fn perf_pmu_disable(pmu: *mut Pmu);
    fn perf_pmu_enable(pmu: *mut Pmu);
    fn perf_event_update_userpage(event: *mut PerfEvent);
    fn get_cpu_var(v: *mut CpuHwEvents) -> *mut CpuHwEvents;
    fn put_cpu_var(v: *mut CpuHwEvents);
    fn is_software_event(event: *mut PerfEvent) -> bool;
    fn for_each_sibling_event(event: *mut *mut PerfEvent, group: *mut PerfEvent);
    fn atomic_read(v: *const AtomicT) -> i32;
    fn atomic_inc(v: *mut AtomicT);
    fn atomic_inc_not_zero(v: *mut AtomicT) -> bool;
    fn atomic_add_unless(v: *mut AtomicT, a: i32, u: i32) -> bool;
    fn atomic_dec_return(v: *mut AtomicT) -> i32;
    fn mutex_lock(v: *mut Mutex);
    fn mutex_unlock(v: *mut Mutex);
    fn reserve_pmc_hardware(f: unsafe extern "C" fn(*mut PtRegs));
    fn release_pmc_hardware();
    fn warn_on(v: bool);
    fn warn_on_once(v: bool);
    fn perf_pmu_register(pmu: *mut Pmu, name: *const u8, typ: u32);
    fn cpuhp_setup_state(state: i32, name: *const u8,
                         prepare: unsafe extern "C" fn(u32) -> i32,
                         teardown: *const core::ffi::c_void);
    fn memset(dst: *mut core::ffi::c_void, val: i32, n: usize);
    fn pr_info(fmt: *const u8, ...);
    fn perf_sample_data_init(data: *mut PerfSampleData, a: u64, b: u64);
    fn perf_event_overflow(event: *mut PerfEvent, data: *mut PerfSampleData,
                           regs: *mut PtRegs);
    fn mtmsr(v: usize);
    fn mfmsr() -> usize;
}

#[repr(C)] struct AtomicT(i32);
#[repr(C)] struct Mutex;
#[repr(C)] struct PtRegs;
#[repr(C)] struct PerfSampleData;
#[repr(C)] struct Pmu {
    pmu_enable: Option<unsafe extern "C" fn(*mut Pmu)>,
    pmu_disable: Option<unsafe extern "C" fn(*mut Pmu)>,
    event_init: Option<unsafe extern "C" fn(*mut PerfEvent) -> i32>,
    add: Option<unsafe extern "C" fn(*mut PerfEvent, i32) -> i32>,
    del: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>,
    start: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>,
    stop: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>,
    read: Option<unsafe extern "C" fn(*mut PerfEvent)>,
}
#[repr(C)] struct PerfEventAttr { type_: u32, config: u64, exclude_user: bool, exclude_kernel: bool, exclude_idle: bool }
#[repr(C)] struct HwPerfEvent { state: u64, idx: i32, config: u64, config_base: u64, sample_period: u64, last_period: u64, prev_count: i64, period_left: i64 }
#[repr(C)] struct PerfEvent { attr: PerfEventAttr, hw: HwPerfEvent, group_leader: *mut PerfEvent, pmu: *mut Pmu, count: i64, destroy: Option<unsafe extern "C" fn(*mut PerfEvent)> }
#[repr(C)] struct FslEmbPmu { n_counter: i32, n_restricted: i32, n_generic: u64, generic_events: *const u64, cache_events: *const *const *const i32, xlate_event: unsafe extern "C" fn(u64) -> u64, name: *const u8 }

const PERF_HES_STOPPED: u64 = 1;
const PERF_HES_UPTODATE: u64 = 2;
const PERF_EF_START: i32 = 1;
const PERF_EF_RELOAD: i32 = 2;
const FSL_EMB_EVENT_VALID: u64 = 1;
const FSL_EMB_EVENT_RESTRICTED: u64 = 2;
const PMLCA_CE: u64 = 1;
const PMLCA_FCM1: u64 = 2;
const PMLCA_FCU: u64 = 4;
const PMLCA_FCS: u64 = 8;
const PMLCA_EVENT_MASK: u64 = 0xffff;
const PMGC0_FAC: usize = 1;
const PMGC0_PMIE: usize = 2;
const PMGC0_FCECE: usize = 4;
const MSR_PMM: usize = 1;
const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_TYPE_HW_CACHE: u32 = 3;
const PERF_TYPE_RAW: u32 = 4;
const PERF_COUNT_HW_CACHE_MAX: usize = 16;
const PERF_COUNT_HW_CACHE_OP_MAX: usize = 16;
const PERF_COUNT_HW_CACHE_RESULT_MAX: usize = 8;

const PMRN_PMC0: u32 = 0; const PMRN_PMC1: u32 = 1; const PMRN_PMC2: u32 = 2; const PMRN_PMC3: u32 = 3; const PMRN_PMC4: u32 = 4; const PMRN_PMC5: u32 = 5;
const PMRN_PMLCA0: u32 = 6; const PMRN_PMLCA1: u32 = 7; const PMRN_PMLCA2: u32 = 8; const PMRN_PMLCA3: u32 = 9; const PMRN_PMLCA4: u32 = 10; const PMRN_PMLCA5: u32 = 11;
const PMRN_PMLCB0: u32 = 12; const PMRN_PMLCB1: u32 = 13; const PMRN_PMLCB2: u32 = 14; const PMRN_PMLCB3: u32 = 15; const PMRN_PMLCB4: u32 = 16; const PMRN_PMLCB5: u32 = 17;

unsafe fn read_pmc(idx: i32) -> usize { match idx { 0=>mfpmr(PMRN_PMC0),1=>mfpmr(PMRN_PMC1),2=>mfpmr(PMRN_PMC2),3=>mfpmr(PMRN_PMC3),4=>mfpmr(PMRN_PMC4),5=>mfpmr(PMRN_PMC5), _=>{ printk(0, b"oops trying to read PMC%d\0".as_ptr(),idx); 0 } } }
unsafe fn write_pmc(idx: i32, val: usize) { match idx { 0=>mtpmr(PMRN_PMC0,val),1=>mtpmr(PMRN_PMC1,val),2=>mtpmr(PMRN_PMC2,val),3=>mtpmr(PMRN_PMC3,val),4=>mtpmr(PMRN_PMC4,val),5=>mtpmr(PMRN_PMC5,val), _=>printk(0,b"oops trying to write PMC%d\0".as_ptr(),idx) }; isync(); }
unsafe fn write_pmlca(idx: i32, val: usize) { mtpmr(PMRN_PMLCA0 + idx as u32, val); isync(); }
unsafe fn write_pmlcb(idx: i32, val: usize) { mtpmr(PMRN_PMLCB0 + idx as u32, val); isync(); }

unsafe extern "C" fn fsl_emb_pmu_read(event: *mut PerfEvent) { if (*event).hw.state & PERF_HES_STOPPED != 0 { return; } loop { let prev=local64_read(&(*event).hw.prev_count); let val=read_pmc((*event).hw.idx) as i64; if local64_cmpxchg(&mut (*event).hw.prev_count,prev,val)==prev { let d=(val-prev)&0xffff_ffff; local64_add(d,&mut (*event).count); local64_sub(d,&mut (*event).hw.period_left); break; } } }

unsafe extern "C" fn fsl_emb_pmu_disable(_pmu:*mut Pmu) { let mut flags=0; local_irq_save(&mut flags); if CPU_HW_EVENTS.disabled==0 { CPU_HW_EVENTS.disabled=1; if CPU_HW_EVENTS.pmcs_enabled==0 { ppc_enable_pmcs(); CPU_HW_EVENTS.pmcs_enabled=1; } if atomic_read(&NUM_EVENTS)!=0 { mtpmr(0,PMGC0_FAC); isync(); } } local_irq_restore(flags); }
unsafe extern "C" fn fsl_emb_pmu_enable(_pmu:*mut Pmu) { let mut flags=0; local_irq_save(&mut flags); if CPU_HW_EVENTS.disabled!=0 { CPU_HW_EVENTS.disabled=0; ppc_set_pmu_inuse(CPU_HW_EVENTS.n_events!=0); if CPU_HW_EVENTS.n_events>0 { mtpmr(0,PMGC0_PMIE|PMGC0_FCECE); isync(); } } local_irq_restore(flags); }

#[no_mangle] pub unsafe extern "C" fn register_fsl_emb_pmu(pmu:*mut FslEmbPmu)->i32 { if !PPMU.is_null(){return -16;} PPMU=pmu; pr_info(b"%s performance monitor hardware support registered\n\0".as_ptr(),(*pmu).name); perf_pmu_register(core::ptr::null_mut(),b"cpu\0".as_ptr(),PERF_TYPE_RAW); cpuhp_setup_state(0,b"perf/powerpc:prepare\0".as_ptr(),fsl_emb_pmu_prepare_cpu,core::ptr::null()); 0 }
unsafe extern "C" fn fsl_emb_pmu_prepare_cpu(_cpu:u32)->i32 { memset(&mut CPU_HW_EVENTS as *mut _ as *mut _,0,core::mem::size_of::<CpuHwEvents>()); 0 }

unsafe extern "C" fn fsl_emb_pmu_add(event:*mut PerfEvent, flags:i32)->i32 { fsl_emb_pmu_disable((*event).pmu); let n=if (*event).hw.config&FSL_EMB_EVENT_RESTRICTED!=0 {(*PPMU).n_restricted} else {(*PPMU).n_counter}; let mut i=n-1; while i>=0 && !CPU_HW_EVENTS.event[i as usize].is_null(){i-=1;} if i<0 {fsl_emb_pmu_enable((*event).pmu);return -11;} (*event).hw.idx=i; CPU_HW_EVENTS.event[i as usize]=event; CPU_HW_EVENTS.n_events+=1; let mut val=0i64; if (*event).hw.sample_period!=0 {let left=local64_read(&(*event).hw.period_left);if left<0x8000_0000 {val=0x8000_0000-left;}} local64_set(&mut (*event).hw.prev_count,val); if flags&PERF_EF_START==0 {(*event).hw.state=PERF_HES_STOPPED|PERF_HES_UPTODATE;}else{(*event).hw.state&=!(PERF_HES_STOPPED|PERF_HES_UPTODATE);} write_pmc(i,val as usize);perf_event_update_userpage(event);write_pmlcb(i,((*event).hw.config>>32) as usize);write_pmlca(i,(*event).hw.config_base as usize);fsl_emb_pmu_enable((*event).pmu);0 }
unsafe extern "C" fn fsl_emb_pmu_del(event:*mut PerfEvent,_flags:i32){fsl_emb_pmu_disable((*event).pmu);let i=(*event).hw.idx;if i>=0{fsl_emb_pmu_read(event);write_pmlca(i,0);write_pmlcb(i,0);write_pmc(i,0);CPU_HW_EVENTS.event[i as usize]=core::ptr::null_mut();(*event).hw.idx=-1;CPU_HW_EVENTS.n_events-=1;}fsl_emb_pmu_enable((*event).pmu);}
unsafe extern "C" fn fsl_emb_pmu_start(event:*mut PerfEvent,flags:i32){if (*event).hw.idx<0||(*event).hw.sample_period==0||(*event).hw.state&PERF_HES_STOPPED==0{return;}if flags&PERF_EF_RELOAD!=0{warn_on_once((*event).hw.state&PERF_HES_UPTODATE==0);}fsl_emb_pmu_disable((*event).pmu);(*event).hw.state=0;let left=local64_read(&(*event).hw.period_left);let val=if left<0x8000_0000{0x8000_0000-left}else{0};write_pmc((*event).hw.idx,val as usize);perf_event_update_userpage(event);fsl_emb_pmu_enable((*event).pmu);}
unsafe extern "C" fn fsl_emb_pmu_stop(event:*mut PerfEvent,_flags:i32){if (*event).hw.idx<0||(*event).hw.sample_period==0||(*event).hw.state&PERF_HES_STOPPED!=0{return;}fsl_emb_pmu_disable((*event).pmu);fsl_emb_pmu_read(event);(*event).hw.state|=PERF_HES_STOPPED|PERF_HES_UPTODATE;write_pmc((*event).hw.idx,0);perf_event_update_userpage(event);fsl_emb_pmu_enable((*event).pmu);}

#[repr(C)] static mut FSL_EMB_PMU:Pmu=Pmu{pmu_enable:Some(fsl_emb_pmu_enable),pmu_disable:Some(fsl_emb_pmu_disable),event_init:None,add:Some(fsl_emb_pmu_add),del:Some(fsl_emb_pmu_del),start:Some(fsl_emb_pmu_start),stop:Some(fsl_emb_pmu_stop),read:Some(fsl_emb_pmu_read)};

unsafe extern "C" fn hw_perf_event_destroy(_event:*mut PerfEvent){if !atomic_add_unless(&mut NUM_EVENTS,-1,1){mutex_lock(&mut PMC_RESERVE_MUTEX);if atomic_dec_return(&mut NUM_EVENTS)==0{release_pmc_hardware();}mutex_unlock(&mut PMC_RESERVE_MUTEX);}}
unsafe extern "C" fn record_and_restart(event:*mut PerfEvent,val:usize,regs:*mut PtRegs){if (*event).hw.state&PERF_HES_STOPPED!=0{write_pmc((*event).hw.idx,0);return;}let prev=local64_read(&(*event).hw.prev_count);let delta=(val as i64-prev)&0xffff_ffff;local64_add(delta,&mut (*event).count);let period=(*event).hw.sample_period;let mut left=local64_read(&(*event).hw.period_left)-delta;let mut out=0;if period!=0&&left<=0{left+=period;if left<=0{left=period;}(*event).hw.last_period=period;if left<0x8000_0000{out=0x8000_0000-left;}}write_pmc((*event).hw.idx,out as usize);local64_set(&mut (*event).hw.prev_count,out);local64_set(&mut (*event).hw.period_left,left);perf_event_update_userpage(event);}
unsafe extern "C" fn perf_event_interrupt_handler(regs:*mut PtRegs){let mut i=0;while i<(*PPMU).n_counter{let val=read_pmc(i);if (val as i32)<0{let e=CPU_HW_EVENTS.event[i as usize];if !e.is_null(){record_and_restart(e,val,regs)}else{write_pmc(i,0);}}i+=1;}mtmsr(mfmsr()|MSR_PMM);mtpmr(0,PMGC0_PMIE|PMGC0_FCECE);isync();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
