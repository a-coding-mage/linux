// SPDX-License-Identifier: GPL-2.0
/* Hardware performance events for the Alpha. */

use core::ffi::c_void;

const MAX_HWEVENTS: usize = 3;
const PMC_NO_INDEX: i32 = -1;
const HW_OP_UNSUPPORTED: i32 = -1;

#[repr(C)]
pub struct perf_event { pub hw: hw_perf_event, pub attr: perf_event_attr, pub group_leader: *mut perf_event, pub pmu: *mut pmu, pub state: i32, pub count: i64, pub destroy: Option<unsafe extern "C" fn(*mut perf_event)> }
#[repr(C)]
pub struct perf_event_attr { pub type_: u32, pub config: u64 }
#[repr(C)]
pub struct hw_perf_event { pub event_base: u64, pub config_base: u64, pub idx: i32, pub state: u64, pub sample_period: i64, pub last_period: i64, pub period_left: i64, pub prev_count: i64 }
#[repr(C)] pub struct pmu { pub pmu_enable: Option<unsafe extern "C" fn(*mut pmu)>, pub pmu_disable: Option<unsafe extern "C" fn(*mut pmu)>, pub event_init: Option<unsafe extern "C" fn(*mut perf_event) -> i32>, pub add: Option<unsafe extern "C" fn(*mut perf_event, i32) -> i32>, pub del: Option<unsafe extern "C" fn(*mut perf_event, i32)>, pub start: Option<unsafe extern "C" fn(*mut perf_event, i32)>, pub stop: Option<unsafe extern "C" fn(*mut perf_event, i32)>, pub read: Option<unsafe extern "C" fn(*mut perf_event)> , pub capabilities: u64 }
#[repr(C)] pub struct cpu_hw_events { pub enabled: i32, pub n_events: i32, pub n_added: i32, pub event: [*mut perf_event; MAX_HWEVENTS], pub evtype: [u64; MAX_HWEVENTS], pub current_idx: [i32; MAX_HWEVENTS], pub config: u64, pub idx_mask: u64 }
#[repr(C)] pub struct alpha_pmu_t { pub event_map: *const i32, pub max_events: i32, pub num_pmcs: i32, pub pmc_count_shift: [i32; MAX_HWEVENTS], pub pmc_count_mask: [u64; MAX_HWEVENTS], pub pmc_max_period: [u64; MAX_HWEVENTS], pub pmc_left: [i64; 3], pub check_constraints: Option<unsafe extern "C" fn(*mut *mut perf_event, *mut u64, i32) -> i32>, pub raw_event_valid: Option<unsafe extern "C" fn(u64) -> i32> }
#[repr(C)] pub struct ev67_mapping_t { pub config: i32, pub idx: i32 }
#[repr(C)] pub struct percpu_struct { pub type_: u64 }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct perf_sample_data { _private: [u8; 0] }

extern "C" {
    static mut alpha_pmu: *const alpha_pmu_t;
    static mut cpu_hw_events: cpu_hw_events;
    static mut hwrpb: *mut c_void;
    static mut irq_pmi_count: u64;
    static mut irq_err_count: u64;
    static mut perf_irq: Option<unsafe extern "C" fn(u64, *mut pt_regs)>;
    fn wrperfmon(cmd: u64, val: u64) -> u64;
    fn local64_read(v: *const i64) -> i64;
    fn local64_set(v: *mut i64, x: i64);
    fn local64_cmpxchg(v: *mut i64, old: i64, new: i64) -> i64;
    fn local64_add(x: i64, v: *mut i64); fn local64_sub(x: i64, v: *mut i64);
    fn perf_event_update_userpage(e: *mut perf_event); fn perf_event_overflow(e: *mut perf_event, d: *mut perf_sample_data, r: *mut pt_regs);
    fn perf_pmu_disable(p: *mut pmu); fn perf_pmu_enable(p: *mut pmu); fn perf_pmu_register(p: *mut pmu, name: *const u8, typ: u32) -> i32;
    fn local_irq_save(f: *mut u64); fn local_irq_restore(f: u64); fn barrier(); fn smp_processor_id() -> i32;
    fn pr_info(fmt: *const u8, ...); fn pr_warn(fmt: *const u8, ...); fn pr_cont(fmt: *const u8, ...);
    fn has_branch_stack(e: *mut perf_event) -> i32; fn is_software_event(e: *mut perf_event) -> i32;
    fn perf_sample_data_init(d: *mut perf_sample_data, a: u64, b: i64);
}

enum { EV67_CYCLES = 1, EV67_INSTRUCTIONS, EV67_BCACHEMISS, EV67_MBOXREPLAY, EV67_LAST_ET }
const EV67_NUM_EVENT_TYPES: usize = (EV67_LAST_ET - EV67_CYCLES) as usize;
extern "C" { static ev67_perfmon_event_map: [i32; 4]; static ev67_mapping: [ev67_mapping_t; EV67_NUM_EVENT_TYPES]; }

unsafe extern "C" fn ev67_check_constraints(event: *mut *mut perf_event, evtype: *mut u64, n_ev: i32) -> i32 {
    let mut idx0 = (*ev67_mapping.as_ptr().add((*evtype.offset(0) as usize)-1)).idx;
    let mut config = (*ev67_mapping.as_ptr().add((*evtype.offset(0) as usize)-1)).config;
    if n_ev == 1 { (*event).as_mut().unwrap().hw.idx=idx0; (*event).as_mut().unwrap().hw.config_base=config as u64; return 0; }
    if n_ev != 2 { return -1; }
    if *evtype == EV67_MBOXREPLAY as u64 || *evtype.offset(1) == EV67_MBOXREPLAY as u64 { idx0=if *evtype==EV67_MBOXREPLAY as u64 {1} else {0}; if *evtype.offset(idx0 as isize)==EV67_CYCLES as u64 { config=0; } else { return -1; } }
    else if *evtype == EV67_BCACHEMISS as u64 || *evtype.offset(1)==EV67_BCACHEMISS as u64 { idx0=if *evtype==EV67_BCACHEMISS as u64 {1} else {0}; if *evtype.offset(idx0 as isize)==EV67_INSTRUCTIONS as u64 { config=0; } else { return -1; } }
    else if *evtype == EV67_INSTRUCTIONS as u64 || *evtype.offset(1)==EV67_INSTRUCTIONS as u64 { idx0=if *evtype==EV67_INSTRUCTIONS as u64 {0} else {1}; if *evtype.offset((idx0^1) as isize)!=EV67_CYCLES as u64 { return -1; } config=0; }
    (*event).as_mut().unwrap().hw.idx=idx0; (*event).as_mut().unwrap().hw.config_base=config as u64;
    if n_ev==2 { (*event.offset(1)).as_mut().unwrap().hw.idx=idx0^1; (*event.offset(1)).as_mut().unwrap().hw.config_base=config as u64; } 0
}
unsafe extern "C" fn ev67_raw_event_valid(config:u64)->i32 { (config>=EV67_CYCLES as u64 && config<EV67_LAST_ET as u64) as i32 }

unsafe fn alpha_write_pmc(idx:i32, mut val:u64){ val &= (*alpha_pmu).pmc_count_mask[idx as usize]; val <<= (*alpha_pmu).pmc_count_shift[idx as usize]; val |= 1u64 << idx; wrperfmon(0,val); }
unsafe fn alpha_read_pmc(idx:i32)->u64 { let mut v=wrperfmon(1,0); v >>= (*alpha_pmu).pmc_count_shift[idx as usize]; v &= (*alpha_pmu).pmc_count_mask[idx as usize]; v }
unsafe fn alpha_perf_event_set_period(e:*mut perf_event,h:*mut hw_perf_event,idx:i32)->i32 { let mut left=local64_read(&(*h).period_left); let period=(*h).sample_period; let mut ret=0; if left<=-period {left=period;local64_set(&mut (*h).period_left,left);(*h).last_period=period;ret=1;} if left<=0 {left+=period;local64_set(&mut (*h).period_left,left);(*h).last_period=period;ret=1;} if left<(*alpha_pmu).pmc_left[idx as usize]{left=(*alpha_pmu).pmc_left[idx as usize];} if left>(*alpha_pmu).pmc_max_period[idx as usize] as i64 {left=(*alpha_pmu).pmc_max_period[idx as usize] as i64;} local64_set(&mut (*h).prev_count,-left);alpha_write_pmc(idx,-left as u64);perf_event_update_userpage(e);ret }
unsafe fn alpha_perf_event_update(e:*mut perf_event,h:*mut hw_perf_event,idx:i32,ovf:i64)->u64 { loop { let p=local64_read(&(*h).prev_count); let n=alpha_read_pmc(idx) as i64; if local64_cmpxchg(&mut (*h).prev_count,p,n)!=p {continue;} let mut d=n-(p as u64 & (*alpha_pmu).pmc_count_mask[idx as usize]) as i64+ovf; if d<0 {d+=(*alpha_pmu).pmc_max_period[idx as usize] as i64+1;} local64_add(d,&mut (*e).count);local64_sub(d,&mut (*h).period_left);return n as u64; } }

unsafe extern "C" fn alpha_pmu_read(e:*mut perf_event){alpha_perf_event_update(e,&mut (*e).hw,(*e).hw.idx,0);}
unsafe extern "C" fn alpha_pmu_stop(e:*mut perf_event,flags:i32){let h=&mut (*e).hw;let c=&mut cpu_hw_events;if h.state&1==0{c.idx_mask&=!(1u64<<h.idx);h.state|=1;}if flags&1!=0&&h.state&2==0{alpha_perf_event_update(e,h,h.idx,0);h.state|=2;}if c.enabled!=0{wrperfmon(2,1u64<<h.idx);}}
unsafe extern "C" fn alpha_pmu_start(e:*mut perf_event,flags:i32){let h=&mut (*e).hw;let c=&mut cpu_hw_events;if h.state&1==0{return;}if flags&2!=0{alpha_perf_event_set_period(e,h,h.idx);}h.state=0;c.idx_mask|=1u64<<h.idx;if c.enabled!=0{wrperfmon(3,1u64<<h.idx);}}

#[no_mangle] pub unsafe extern "C" fn perf_event_print_debug(){if hwrpb.is_null(){return;}let mut f=0;local_irq_save(&mut f);let p=wrperfmon(1,0);let p0=(p>>(*alpha_pmu).pmc_count_shift[0])&(*alpha_pmu).pmc_count_mask[0];let p1=(p>>(*alpha_pmu).pmc_count_shift[1])&(*alpha_pmu).pmc_count_mask[1];pr_info(b"CPU#%d: PCTR0[%06x] PCTR1[%06x]\n\0".as_ptr(),smp_processor_id(),p0,p1);local_irq_restore(f);}

unsafe fn collect_events(group:*mut perf_event,max:i32,event:*mut *mut perf_event,evtype:*mut u64,current:*mut i32)->i32 { let mut n=0; if is_software_event(group)==0 {if n>=max{return -1;}*event=group;*evtype=(*group).hw.event_base;*current=PMC_NO_INDEX;n+=1;} n }
unsafe fn alpha_check_constraints(events:*mut *mut perf_event,types:*mut u64,n:i32)->i32 {if n==0{return 0;}if n>(*alpha_pmu).num_pmcs{return -1;}((*alpha_pmu).check_constraints.unwrap())(events,types,n)}
unsafe fn maybe_change_configuration(c:&mut cpu_hw_events){if c.n_added==0{return;}c.idx_mask=0;for j in 0..c.n_events as usize{let e=c.event[j];if c.current_idx[j]==PMC_NO_INDEX{alpha_perf_event_set_period(e,&mut (*e).hw,(*e).hw.idx);c.current_idx[j]=(*e).hw.idx;}if (*e).hw.state&1==0{c.idx_mask|=1u64<<c.current_idx[j];}}c.config=(*c.event[0]).hw.config_base;}
unsafe extern "C" fn alpha_pmu_enable(_: *mut pmu){let c=&mut cpu_hw_events;if c.enabled!=0{return;}c.enabled=1;barrier();if c.n_events>0{maybe_change_configuration(c);wrperfmon(4,0);wrperfmon(5,c.config);wrperfmon(3,c.idx_mask);}}
unsafe extern "C" fn alpha_pmu_disable(_: *mut pmu){let c=&mut cpu_hw_events;if c.enabled==0{return;}c.enabled=0;c.n_added=0;wrperfmon(2,c.idx_mask);}
unsafe extern "C" fn alpha_pmu_add(e:*mut perf_event,flags:i32)->i32{let c=&mut cpu_hw_events;perf_pmu_disable((*e).pmu);let mut f=0;local_irq_save(&mut f);let mut ret=-11;let n=c.n_events as usize;if n<(*alpha_pmu).num_pmcs as usize{c.event[n]=e;c.evtype[n]=(*e).hw.event_base;c.current_idx[n]=PMC_NO_INDEX; if alpha_check_constraints(c.event.as_mut_ptr(),c.evtype.as_mut_ptr(),c.n_events+1)==0{c.n_events+=1;c.n_added+=1;ret=0;}}(*e).hw.state=2;if flags&1==0{(*e).hw.state|=1;}local_irq_restore(f);perf_pmu_enable((*e).pmu);ret}
unsafe extern "C" fn alpha_pmu_del(e:*mut perf_event,_:i32){let c=&mut cpu_hw_events;perf_pmu_disable((*e).pmu);let mut f=0;local_irq_save(&mut f);for j in 0..c.n_events as usize{if c.event[j]==e{let idx=c.current_idx[j];for k in j+1..c.n_events as usize{c.event[k-1]=c.event[k];c.evtype[k-1]=c.evtype[k];c.current_idx[k-1]=c.current_idx[k];}alpha_perf_event_update(e,&mut (*e).hw,idx,0);c.idx_mask&=!(1u64<<idx);c.n_events-=1;break;}}local_irq_restore(f);perf_pmu_enable((*e).pmu);}
unsafe extern "C" fn hw_perf_event_destroy(_: *mut perf_event){}
unsafe extern "C" fn alpha_pmu_event_init(e:*mut perf_event)->i32{if has_branch_stack(e)!=0{return -95;}if (*e).attr.type_!=0&&(*e).attr.type_!=1&&(*e).attr.type_!=3{return -2;}if alpha_pmu.is_null(){return -19;}let h=&mut (*e).hw;let ev=if (*e).attr.type_==1{if (*e).attr.config>=4{return -22;}*ev67_perfmon_event_map.as_ptr().add((*e).attr.config as usize) as u64}else if (*e).attr.type_==3{(*alpha_pmu).raw_event_valid.unwrap()((*e).attr.config);(*e).attr.config}else{return -95};h.event_base=ev;h.config_base=0;h.idx=PMC_NO_INDEX;(*e).destroy=Some(hw_perf_event_destroy);if h.sample_period==0{h.sample_period=(*alpha_pmu).pmc_max_period[0] as i64;h.last_period=h.sample_period;local64_set(&mut h.period_left,h.sample_period);}0}
static mut PMU: pmu=pmu{pmu_enable:Some(alpha_pmu_enable),pmu_disable:Some(alpha_pmu_disable),event_init:Some(alpha_pmu_event_init),add:Some(alpha_pmu_add),del:Some(alpha_pmu_del),start:Some(alpha_pmu_start),stop:Some(alpha_pmu_stop),read:Some(alpha_pmu_read),capabilities:0};
#[no_mangle] pub unsafe extern "C" fn alpha_perf_event_irq_handler(la_ptr:u64,_regs:*mut pt_regs){let c=&mut cpu_hw_events;wrperfmon(2,c.idx_mask);if la_ptr>=(*alpha_pmu).num_pmcs as u64{irq_err_count+=1;wrperfmon(3,c.idx_mask);return;}for j in 0..c.n_events as usize{if c.current_idx[j]==la_ptr as i32{let e=c.event[j];if !e.is_null(){alpha_perf_event_update(e,&mut (*e).hw,la_ptr as i32,(*alpha_pmu).pmc_max_period[la_ptr as usize] as i64+1);alpha_perf_event_set_period(e,&mut (*e).hw,la_ptr as i32);}break;}}wrperfmon(3,c.idx_mask);}
#[no_mangle] pub unsafe extern "C" fn init_hw_perf_events()->i32{alpha_pmu=&EV67_PMU;perf_pmu_register(&mut PMU,b"cpu\0".as_ptr(),4);0}
static EV67_PMU:alpha_pmu_t=alpha_pmu_t{event_map:core::ptr::null(),max_events:4,num_pmcs:2,pmc_count_shift:[0;3],pmc_count_mask:[0;3],pmc_max_period:[1048575,1048575,0],pmc_left:[16,4,0],check_constraints:Some(ev67_check_constraints),raw_event_valid:Some(ev67_raw_event_valid)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
