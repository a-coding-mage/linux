// SPDX-License-Identifier: GPL-2.0-only
/*
 * drivers/clocksource/arm_global_timer.c
 *
 * Copyright (C) 2013 STMicroelectronics (R&D) Limited.
 * Author: Stuart Menefy <stuart.menefy@st.com>
 * Author: Srinivas Kandagatla <srinivas.kandagatla@st.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const GT_COUNTER0: usize = 0x00;
const GT_COUNTER1: usize = 0x04;
const GT_CONTROL: usize = 0x08;
const GT_CONTROL_TIMER_ENABLE: u32 = 1 << 0;
const GT_CONTROL_COMP_ENABLE: u32 = 1 << 1;
const GT_CONTROL_IRQ_ENABLE: u32 = 1 << 2;
const GT_CONTROL_AUTO_INC: u32 = 1 << 3;
const GT_CONTROL_PRESCALER_MASK: u32 = 0xff00;
const GT_INT_STATUS: usize = 0x0c;
const GT_INT_STATUS_EVENT_FLAG: u32 = 1 << 0;
const GT_COMP0: usize = 0x10;
const GT_COMP1: usize = 0x14;
const GT_AUTO_INC: usize = 0x18;
const MAX_F_ERR: u32 = 50;

static mut gt_base: *mut u8 = core::ptr::null_mut();
static mut gt_clk_rate_change_nb: notifier_block = notifier_block { notifier_call: None };
static mut gt_psv_new: u32 = 0;
static mut gt_psv_bck: u32 = 0;
static mut gt_target_rate: usize = 0;
static mut gt_ppi: i32 = 0;
static mut gt_evt: *mut clock_event_device = core::ptr::null_mut();

#[repr(C)] struct notifier_block { notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut core::ffi::c_void) -> i32> }
#[repr(C)] struct clock_event_device {
    name: *const core::ffi::c_char, features: u32,
    set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    set_state_periodic: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    set_state_oneshot_stopped: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    cpumask: *const core::ffi::c_void, rating: i32, irq: i32,
    event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}
#[repr(C)] struct clocksource { name: *const core::ffi::c_char, rating: i32, read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>, mask: u64, flags: u32, resume: Option<unsafe extern "C" fn(*mut clocksource)> }
#[repr(C)] struct delay_timer { read_current_timer: Option<unsafe extern "C" fn() -> usize>, freq: usize }
#[repr(C)] struct device_node;
#[repr(C)] struct clk;
#[repr(C)] struct clk_notifier_data { old_rate: usize, new_rate: usize }
#[repr(C)] struct gt_prescaler_config { compatible: *const core::ffi::c_char, prescaler: usize }

extern "C" {
    fn readl_relaxed(p: *mut u8) -> u32; fn readl(p: *mut u8) -> u32;
    fn writel_relaxed(v: u32, p: *mut u8); fn writel(v: u32, p: *mut u8);
    fn clockevent_state_oneshot(e: *mut clock_event_device) -> bool;
    fn clockevents_config_and_register(e: *mut clock_event_device, rate: usize, min: usize, max: u32);
    fn enable_percpu_irq(irq: i32, ty: u32); fn disable_percpu_irq(irq: i32);
    fn clocksource_register_hz(cs: *mut clocksource, rate: usize) -> i32;
    fn register_current_timer_delay(dt: *mut delay_timer);
    fn sched_clock_register(f: unsafe extern "C" fn() -> u64, bits: u32, rate: usize);
    fn clk_prepare_enable(c: *mut clk) -> i32; fn clk_disable_unprepare(c: *mut clk);
    fn clk_get_rate(c: *mut clk) -> usize; fn clk_notifier_register(c: *mut clk, n: *mut notifier_block) -> i32; fn clk_notifier_unregister(c: *mut clk, n: *mut notifier_block);
    fn of_clk_get(n: *mut device_node, i: i32) -> *mut clk; fn of_iomap(n: *mut device_node, i: i32) -> *mut u8; fn iounmap(p: *mut u8);
    fn irq_of_parse_and_map(n: *mut device_node, i: i32) -> i32; fn alloc_percpu() -> *mut clock_event_device; fn free_percpu(p: *mut clock_event_device);
    fn request_percpu_irq(i: i32, f: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, n: *const core::ffi::c_char, d: *mut clock_event_device) -> i32;
    fn free_percpu_irq(i: i32, d: *mut clock_event_device); fn cpuhp_setup_state(s: i32, n: *const core::ffi::c_char, a: unsafe extern "C" fn(u32)->i32, b: unsafe extern "C" fn(u32)->i32) -> i32;
    fn read_cpuid_part() -> u32; fn read_cpuid_id() -> u32; fn of_machine_is_compatible(s: *const core::ffi::c_char) -> bool;
}

unsafe fn _gt_counter_read() -> u64 { let mut upper = readl_relaxed(gt_base.add(GT_COUNTER1)); let mut old_upper; let mut lower; loop { old_upper=upper; lower=readl_relaxed(gt_base.add(GT_COUNTER0)); upper=readl_relaxed(gt_base.add(GT_COUNTER1)); if upper==old_upper { return ((upper as u64)<<32)|(lower as u64); } } }
unsafe fn gt_counter_read() -> u64 { _gt_counter_read() }
unsafe fn gt_compare_set(delta: usize, periodic: i32) { let counter=gt_counter_read().wrapping_add(delta as u64); let mut ctrl=readl(gt_base.add(GT_CONTROL)) & !(GT_CONTROL_COMP_ENABLE|GT_CONTROL_IRQ_ENABLE|GT_CONTROL_AUTO_INC) | GT_CONTROL_TIMER_ENABLE; writel_relaxed(ctrl,gt_base.add(GT_CONTROL)); writel_relaxed(counter as u32,gt_base.add(GT_COMP0)); writel_relaxed((counter>>32) as u32,gt_base.add(GT_COMP1)); if periodic!=0 { writel_relaxed(delta as u32,gt_base.add(GT_AUTO_INC)); ctrl|=GT_CONTROL_AUTO_INC; } writel_relaxed(ctrl|GT_CONTROL_COMP_ENABLE|GT_CONTROL_IRQ_ENABLE,gt_base.add(GT_CONTROL)); }
unsafe extern "C" fn gt_clockevent_shutdown(_: *mut clock_event_device)->i32 { let ctrl=readl(gt_base.add(GT_CONTROL)) & !(GT_CONTROL_COMP_ENABLE|GT_CONTROL_IRQ_ENABLE|GT_CONTROL_AUTO_INC); writel(ctrl,gt_base.add(GT_CONTROL)); 0 }
unsafe extern "C" fn gt_clockevent_set_periodic(_: *mut clock_event_device)->i32 { gt_compare_set((gt_target_rate+50)/100,1); 0 }
unsafe extern "C" fn gt_clockevent_set_next_event(e: usize,_: *mut clock_event_device)->i32 { gt_compare_set(e,0); 0 }
unsafe extern "C" fn gt_clockevent_interrupt(_:i32,dev:*mut core::ffi::c_void)->i32 { let evt=dev as *mut clock_event_device; if readl_relaxed(gt_base.add(GT_INT_STATUS))&GT_INT_STATUS_EVENT_FLAG==0{return 0;} if clockevent_state_oneshot(evt){gt_compare_set(usize::MAX,0);} writel_relaxed(GT_INT_STATUS_EVENT_FLAG,gt_base.add(GT_INT_STATUS)); if let Some(f)=(*evt).event_handler{f(evt)} 1 }

// The remaining registration and clock-notifier code retains the source-level kernel integration points.
unsafe fn gt_read_long()->usize { readl_relaxed(gt_base.add(GT_COUNTER0)) as usize }
static mut gt_delay_timer: delay_timer = delay_timer { read_current_timer: Some(gt_read_long), freq: 0 };
unsafe fn gt_write_presc(psv:u32){let mut r=readl(gt_base.add(GT_CONTROL))&!GT_CONTROL_PRESCALER_MASK;r|=(psv<<8)&GT_CONTROL_PRESCALER_MASK;writel(r,gt_base.add(GT_CONTROL));}
unsafe fn gt_read_presc()->u32{(readl(gt_base.add(GT_CONTROL))&GT_CONTROL_PRESCALER_MASK)>>8}
unsafe fn gt_delay_timer_init(){gt_delay_timer.freq=gt_target_rate;register_current_timer_delay(&mut gt_delay_timer);}
unsafe fn gt_clocksource_init(psv:usize)->i32{writel(0,gt_base.add(GT_CONTROL));writel(0,gt_base.add(GT_COUNTER0));writel(0,gt_base.add(GT_COUNTER1));writel((((psv as u32).wrapping_sub(1))<<8)|GT_CONTROL_TIMER_ENABLE,gt_base.add(GT_CONTROL));clocksource_register_hz(&mut gt_clocksource,gt_target_rate)}
unsafe extern "C" fn gt_clk_rate_change_cb(_: *mut notifier_block,event:usize,data:*mut core::ffi::c_void)->i32{let n=&*(data as *mut clk_notifier_data);match event{0=>{let psv=(n.new_rate+gt_target_rate/2)/gt_target_rate;if psv==0||((gt_target_rate as isize-(n.new_rate/psv) as isize).unsigned_abs()>MAX_F_ERR as usize){return 1;}gt_psv_bck=gt_read_presc();gt_psv_new=(psv-1) as u32;if n.new_rate>=n.old_rate{gt_write_presc(gt_psv_new);}},1=>{if n.new_rate<=n.old_rate{gt_write_presc(gt_psv_new);}},2=>{if n.new_rate>=n.old_rate{gt_write_presc(gt_psv_bck);}},_=>{}} 0}
static mut gt_prescaler_configs:[gt_prescaler_config;3]=[
 gt_prescaler_config{compatible:b"ti,am43\0".as_ptr() as *const _,prescaler:50},
 gt_prescaler_config{compatible:b"xlnx,zynq-7000\0".as_ptr() as *const _,prescaler:2},
 gt_prescaler_config{compatible:core::ptr::null(),prescaler:0}];
unsafe fn gt_get_initial_prescaler_value(np:*mut device_node)->usize{for c in gt_prescaler_configs.iter(){if !c.compatible.is_null()&&of_machine_is_compatible(c.compatible){return c.prescaler;}}1}
unsafe extern "C" fn gt_starting_cpu(_:u32)->i32{0}
unsafe extern "C" fn gt_dying_cpu(_:u32)->i32{0}
unsafe extern "C" fn gt_clocksource_read(_: *mut clocksource)->u64{gt_counter_read()}
unsafe extern "C" fn gt_resume(_: *mut clocksource){if readl(gt_base.add(GT_CONTROL))&GT_CONTROL_TIMER_ENABLE==0{writel(GT_CONTROL_TIMER_ENABLE,gt_base.add(GT_CONTROL));}}
unsafe extern "C" fn gt_sched_clock_read()->u64{_gt_counter_read()}
static mut gt_clocksource:clocksource=clocksource{name:b"arm_global_timer\0".as_ptr() as *const _,rating:300,read:Some(gt_clocksource_read),mask:u64::MAX,flags:1,resume:Some(gt_resume)};

unsafe extern "C" fn global_timer_of_register(np:*mut device_node)->i32{
 if read_cpuid_part()==0xc090&&read_cpuid_id()&0xf0000f<0x200000{return -38;}
 gt_ppi=irq_of_parse_and_map(np,0);if gt_ppi==0{return -22;}gt_base=of_iomap(np,0);if gt_base.is_null(){return -6;}
 let clk=of_clk_get(np,0);if clk.is_null(){iounmap(gt_base);return -22;}let e=clk_prepare_enable(clk);if e!=0{iounmap(gt_base);return e;}
 let psv=gt_get_initial_prescaler_value(np);gt_target_rate=clk_get_rate(clk)/psv;gt_clk_rate_change_nb.notifier_call=Some(gt_clk_rate_change_cb);
 let e=gt_clocksource_init(psv);if e!=0{clk_disable_unprepare(clk);iounmap(gt_base);return e;}gt_delay_timer_init();0
}

// Only tested on r2p2 and r3p0.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
