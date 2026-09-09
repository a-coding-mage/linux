// SPDX-License-Identifier: GPL-2.0
/* Linux performance counter support for LoongArch. */

use core::ptr;

const LOONGARCH_MAX_HWEVENTS: usize = 32;
const HW_OP_UNSUPPORTED: u32 = 0xffff_ffff;
const CACHE_OP_UNSUPPORTED: u32 = 0xffff_ffff;
const M_PERFCTL_CONFIG_MASK: u32 = 0x1f0000;

#[repr(C)] pub struct PerfCallchainEntryCtx { pub nr: u32, pub max_stack: u32 }
#[repr(C)] pub struct PtRegs { pub csr_era: u64, pub regs: [u64; 32] }
#[repr(C)] pub struct StackFrame { pub fp: u64, pub ra: u64 }
#[repr(C)] pub struct PerfEventAttr { pub type_: u32, pub config: u64, pub exclude_user: bool, pub exclude_kernel: bool, pub exclude_hv: bool }
#[repr(C)] pub struct HwPerfEvent { pub event_base: u32, pub config_base: u32, pub idx: i32, pub state: u64, pub sample_period: u64, pub last_period: u64, pub period_left: i64, pub prev_count: i64, pub config: u64 }
#[repr(C)] pub struct PerfEvent { pub cpu: i32, pub attr: PerfEventAttr, pub hw: HwPerfEvent, pub group_leader: *mut PerfEvent, pub pmu: *mut Pmu, pub count: i64, pub destroy: Option<unsafe extern "C" fn(*mut PerfEvent)> }
#[repr(C)] pub struct Pmu { pub pmu_enable: Option<unsafe extern "C" fn(*mut Pmu)>, pub pmu_disable: Option<unsafe extern "C" fn(*mut Pmu)>, pub event_init: Option<unsafe extern "C" fn(*mut PerfEvent) -> i32>, pub add: Option<unsafe extern "C" fn(*mut PerfEvent, i32) -> i32>, pub del: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>, pub start: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>, pub stop: Option<unsafe extern "C" fn(*mut PerfEvent, i32)>, pub read: Option<unsafe extern "C" fn(*mut PerfEvent)> }
#[repr(C)] pub struct CpuHwEvents { pub events: [*mut PerfEvent; LOONGARCH_MAX_HWEVENTS], pub used_mask: u64, pub saved_ctrl: [u32; LOONGARCH_MAX_HWEVENTS] }
#[repr(C)] pub struct LoongarchPerfEvent { pub event_id: u32 }
#[repr(C)] pub struct LoongarchPmu { pub max_period: u64, pub valid_count: u64, pub overflow: u64, pub name: *const i8, pub num_counters: u32, pub read_counter: Option<unsafe extern "C" fn(u32)->u64>, pub write_counter: Option<unsafe extern "C" fn(u32,u64)>, pub map_raw_event: Option<unsafe extern "C" fn(u64)->*const LoongarchPerfEvent>, pub general_event_map: *const LoongarchPerfEvent, pub cache_event_map: *const LoongarchPerfEvent }
#[repr(C)] pub struct PerfSampleData { pub period: u64 }
#[repr(C)] pub struct UnwindState { _private: [u8; 0] }

extern "C" {
    fn access_ok(p: *const u8, n: usize) -> bool; fn pagefault_disable(); fn pagefault_enable();
    fn __copy_from_user_inatomic(to: *mut u8, from: *const u8, n: usize) -> usize;
    fn perf_callchain_store(e: *mut PerfCallchainEntryCtx, addr: u64) -> bool; fn perf_guest_state() -> bool;
    fn unwind_start(s: *mut UnwindState, task: *mut u8, r: *mut PtRegs); fn unwind_done(s: *mut UnwindState)->bool; fn unwind_next_frame(s: *mut UnwindState); fn unwind_get_return_address(s: *mut UnwindState)->u64;
    fn read_csr_perfcntr0()->u64; fn read_csr_perfcntr1()->u64; fn read_csr_perfcntr2()->u64; fn read_csr_perfcntr3()->u64;
    fn write_csr_perfcntr0(v:u64); fn write_csr_perfcntr1(v:u64); fn write_csr_perfcntr2(v:u64); fn write_csr_perfcntr3(v:u64);
    fn read_csr_perfctrl0()->u32; fn read_csr_perfctrl1()->u32; fn read_csr_perfctrl2()->u32; fn read_csr_perfctrl3()->u32;
    fn write_csr_perfctrl0(v:u32); fn write_csr_perfctrl1(v:u32); fn write_csr_perfctrl2(v:u32); fn write_csr_perfctrl3(v:u32);
    fn perf_event_update_userpage(e:*mut PerfEvent); fn perf_event_overflow(e:*mut PerfEvent,d:*mut PerfSampleData,r:*mut PtRegs)->bool;
    fn get_irq_regs()->*mut PtRegs; fn irq_work_run(); fn get_percpu_irq(x:u32)->u32; fn request_irq(i:u32,h:unsafe extern "C" fn(i32,*mut u8)->i32,f:u64,n:*const i8,d:*mut u8)->i32; fn free_irq(i:u32,d:*mut u8);
    fn on_each_cpu(f:unsafe extern "C" fn(*mut u8),a:*mut u8,w:i32); fn smp_processor_id()->u32; fn cpu_online(c:i32)->bool; fn has_branch_stack(e:*mut PerfEvent)->bool; fn get_current()->*mut u8;
    fn read_cpucfg(x:u32)->u32; fn perf_pmu_register(p:*mut Pmu,n:*const i8,t:u32)->i32; fn cpu_has_pmp()->bool;
}

static mut CPU_HW_EVENTS: CpuHwEvents = CpuHwEvents { events: [ptr::null_mut(); LOONGARCH_MAX_HWEVENTS], used_mask: 0, saved_ctrl: [0; LOONGARCH_MAX_HWEVENTS] };
static mut RAW_EVENT: LoongarchPerfEvent = LoongarchPerfEvent { event_id: 0 };
static mut LOONGARCH_PMU: LoongarchPmu = LoongarchPmu { max_period:0, valid_count:0, overflow:0, name:ptr::null(), num_counters:0, read_counter:None, write_counter:None, map_raw_event:None, general_event_map:ptr::null(), cache_event_map:ptr::null() };

#[inline] unsafe fn event_code(v:u32)->u32 { v & 0xffff }
unsafe extern "C" fn read_counter(i:u32)->u64 { match i { 0=>read_csr_perfcntr0(),1=>read_csr_perfcntr1(),2=>read_csr_perfcntr2(),3=>read_csr_perfcntr3(), _=>0 } }
unsafe extern "C" fn write_counter(i:u32,v:u64) { match i { 0=>write_csr_perfcntr0(v),1=>write_csr_perfcntr1(v),2=>write_csr_perfcntr2(v),3=>write_csr_perfcntr3(v), _=>{} } }
unsafe fn read_control(i:u32)->u32 { match i { 0=>read_csr_perfctrl0(),1=>read_csr_perfctrl1(),2=>read_csr_perfctrl2(),3=>read_csr_perfctrl3(), _=>0 } }
unsafe fn write_control(i:u32,v:u32) { match i { 0=>write_csr_perfctrl0(v),1=>write_csr_perfctrl1(v),2=>write_csr_perfctrl2(v),3=>write_csr_perfctrl3(v), _=>{} } }

unsafe fn user_backtrace(e:*mut PerfCallchainEntryCtx, fp:u64)->u64 { let tail=(fp-(core::mem::size_of::<StackFrame>() as u64)) as *const StackFrame; if !access_ok(tail as *const u8,core::mem::size_of::<StackFrame>()){return 0} let mut b=core::mem::MaybeUninit::<StackFrame>::uninit(); pagefault_disable(); let err=__copy_from_user_inatomic(b.as_mut_ptr() as *mut u8,tail as *const u8,core::mem::size_of::<StackFrame>()); pagefault_enable(); if err!=0{return 0} let b=b.assume_init(); if tail as u64>=b.fp{return 0} perf_callchain_store(e,b.ra); b.fp }
#[no_mangle] pub unsafe extern "C" fn perf_callchain_user(e:*mut PerfCallchainEntryCtx,r:*mut PtRegs){ if perf_guest_state(){return} perf_callchain_store(e,(*r).csr_era); let mut fp=(*r).regs[22]; while (*e).nr<(*e).max_stack && fp!=0 && fp&0xf==0 {fp=user_backtrace(e,fp);} }
#[no_mangle] pub unsafe extern "C" fn perf_callchain_kernel(e:*mut PerfCallchainEntryCtx,r:*mut PtRegs){ let mut s=core::mem::MaybeUninit::<UnwindState>::uninit(); unwind_start(s.as_mut_ptr(),get_current(),r); while !unwind_done(s.as_mut_ptr()){let a=unwind_get_return_address(s.as_mut_ptr()); if a==0 || perf_callchain_store(e,a){return} unwind_next_frame(s.as_mut_ptr());} }

unsafe fn alloc_counter(cpuc:*mut CpuHwEvents)->i32 { for i in 0..LOONGARCH_PMU.num_counters {let bit=1u64<<i; if (*cpuc).used_mask&bit==0 {(*cpuc).used_mask|=bit;return i as i32}} -11 }
unsafe fn event_set_period(e:*mut PerfEvent,idx:i32)->i32 { let h=&mut (*e).hw; let mut left=h.period_left as u64; let p=h.sample_period; if left.wrapping_add(p)&(1u64<<63)!=0 || left.wrapping_add(p)<=p {left=if left.wrapping_add(p)&(1u64<<63)!=0{p}else{left+p};h.period_left=left as i64;h.last_period=p;} if left>LOONGARCH_PMU.max_period{left=LOONGARCH_PMU.max_period;h.period_left=left as i64;} h.prev_count=(LOONGARCH_PMU.overflow-left) as i64; (LOONGARCH_PMU.write_counter.unwrap())(idx as u32,LOONGARCH_PMU.overflow-left); perf_event_update_userpage(e); 0 }
unsafe fn event_update(e:*mut PerfEvent,idx:i32){let h=&mut (*e).hw; let n=(LOONGARCH_PMU.read_counter.unwrap())(idx as u32); let d=n.wrapping_sub(h.prev_count as u64);h.prev_count=n as i64;(*e).count=(*e).count.wrapping_add(d as i64);h.period_left=h.period_left.wrapping_sub(d as i64);}
unsafe fn pause_local_counters(){let mut n=LOONGARCH_PMU.num_counters;while n>0{n-=1;CPU_HW_EVENTS.saved_ctrl[n as usize]=read_control(n);write_control(n,CPU_HW_EVENTS.saved_ctrl[n as usize]&!(0x80000000));}}
unsafe fn resume_local_counters(){let mut n=LOONGARCH_PMU.num_counters;while n>0{n-=1;write_control(n,CPU_HW_EVENTS.saved_ctrl[n as usize]);}}
unsafe fn enable_event(e:*mut PerfEvent){let i=(*e).hw.idx as usize;CPU_HW_EVENTS.saved_ctrl[i]=event_code((*e).hw.event_base)|((*e).hw.config_base&M_PERFCTL_CONFIG_MASK)|0x80000000;}
unsafe fn disable_event(i:i32){let n=i as usize;CPU_HW_EVENTS.saved_ctrl[n]=read_control(i as u32)&!0x80000000;write_control(i as u32,CPU_HW_EVENTS.saved_ctrl[n]);}
unsafe extern "C" fn pmu_enable(_: *mut Pmu){resume_local_counters()}
unsafe extern "C" fn pmu_disable(_: *mut Pmu){pause_local_counters()}
unsafe extern "C" fn pmu_start(e:*mut PerfEvent,_:i32){(*e).hw.state=0;event_set_period(e,(*e).hw.idx);enable_event(e)}
unsafe extern "C" fn pmu_stop(e:*mut PerfEvent,_:i32){if (*e).hw.state&1==0{disable_event((*e).hw.idx);event_update(e,(*e).hw.idx);(*e).hw.state|=3;}}
unsafe extern "C" fn pmu_add(e:*mut PerfEvent,flags:i32)->i32{let i=alloc_counter(&mut CPU_HW_EVENTS);if i<0{return i}(*e).hw.idx=i;disable_event(i);CPU_HW_EVENTS.events[i as usize]=e;(*e).hw.state=3;if flags&1!=0{pmu_start(e,8)};perf_event_update_userpage(e);0}
unsafe extern "C" fn pmu_del(e:*mut PerfEvent,_:i32){let i=(*e).hw.idx;pmu_stop(e,0);CPU_HW_EVENTS.events[i as usize]=ptr::null_mut();CPU_HW_EVENTS.used_mask&=!(1u64<<i);perf_event_update_userpage(e)}
unsafe extern "C" fn pmu_read(e:*mut PerfEvent){if (*e).hw.idx>=0{event_update(e,(*e).hw.idx)}}
static mut PMU:Pmu=Pmu{pmu_enable:Some(pmu_enable),pmu_disable:Some(pmu_disable),event_init:None,add:Some(pmu_add),del:Some(pmu_del),start:Some(pmu_start),stop:Some(pmu_stop),read:Some(pmu_read)};
unsafe extern "C" fn pmu_event_init(e:*mut PerfEvent)->i32{if has_branch_stack(e){return -95} if (*e).cpu>=0&&!cpu_online((*e).cpu){return -19} if (*e).attr.type_!=0&&(*e).attr.type_!=1&&(*e).attr.type_!=3{return -2} (*e).hw.config_base=0x80000000;(*e).hw.event_base=(*e).attr.config as u32;(*e).hw.idx=-1;if (*e).hw.sample_period==0{(*e).hw.sample_period=LOONGARCH_PMU.max_period;(*e).hw.last_period=(*e).hw.sample_period;(*e).hw.period_left=(*e).hw.sample_period as i64}0}
unsafe extern "C" fn pmu_irq(_:i32,_:*mut u8)->i32{pause_local_counters();for n in 0..LOONGARCH_PMU.num_counters{if CPU_HW_EVENTS.used_mask&(1u64<<n)!=0&&read_counter(n)&LOONGARCH_PMU.overflow!=0{let e=CPU_HW_EVENTS.events[n as usize];if !e.is_null(){event_update(e,n as i32)}}}resume_local_counters();1}
unsafe extern "C" fn reset_counters(_: *mut u8){for n in 0..LOONGARCH_PMU.num_counters{write_control(n,0);write_counter(n,0);}}
unsafe extern "C" fn loongarch_pmu_map_raw_event(config:u64)->*const LoongarchPerfEvent{RAW_EVENT.event_id=event_code(config as u32);&RAW_EVENT}
unsafe extern "C" fn init_hw_perf_events()->i32{if !cpu_has_pmp(){return -19} LOONGARCH_PMU.num_counters=4;LOONGARCH_PMU.max_period=(1u64<<63)-1;LOONGARCH_PMU.valid_count=(1u64<<63)-1;LOONGARCH_PMU.overflow=1u64<<63;LOONGARCH_PMU.read_counter=Some(read_counter);LOONGARCH_PMU.write_counter=Some(write_counter);LOONGARCH_PMU.map_raw_event=Some(loongarch_pmu_map_raw_event);PMU.event_init=Some(pmu_event_init);on_each_cpu(reset_counters,ptr::null_mut(),1);perf_pmu_register(&mut PMU,b"cpu\0".as_ptr() as *const i8,4);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
