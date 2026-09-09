// SPDX-License-Identifier: GPL-2.0
// Direct low-level translation of tick-broadcast.c. Kernel-provided types and
// functions are intentionally referenced externally.

use core::ffi::c_void;

#[repr(C)] pub struct tick_device { pub evtdev: *mut clock_event_device, pub mode: i32 }
#[repr(C)] pub struct clock_event_device {
    pub features: u64, pub rating: i32, pub next_event: ktime_t,
    pub next_event_forced: i32, pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
    pub broadcast: Option<unsafe extern "C" fn(*mut cpumask)>, pub cpumask: *const cpumask,
    pub irq: i32, pub bound_on: i32, pub owner: *mut c_void,
}
#[repr(C)] pub struct cpumask { _private: [usize; 0] }
pub type ktime_t = i64;
pub type cpumask_var_t = *mut cpumask;
pub type tick_broadcast_mode = i32;
pub type tick_broadcast_state = i32;
pub const TICKDEV_MODE_PERIODIC: i32 = 0;
pub const TICKDEV_MODE_ONESHOT: i32 = 1;
pub const TICK_BROADCAST_FORCE: i32 = 0;
pub const TICK_BROADCAST_ON: i32 = 1;
pub const TICK_BROADCAST_OFF: i32 = 2;
pub const TICK_BROADCAST_ENTER: i32 = 0;
pub const TICK_BROADCAST_EXIT: i32 = 1;
pub const CLOCK_EVT_FEAT_DUMMY: u64 = 1 << 0;
pub const CLOCK_EVT_FEAT_PERCPU: u64 = 1 << 1;
pub const CLOCK_EVT_FEAT_C3STOP: u64 = 1 << 2;
pub const CLOCK_EVT_FEAT_ONESHOT: u64 = 1 << 3;
pub const CLOCK_EVT_FEAT_HRTIMER: u64 = 1 << 4;
pub const CLOCK_EVT_FEAT_DYNIRQ: u64 = 1 << 5;
pub const CLOCK_EVT_STATE_ONESHOT: i32 = 3;
pub const CLOCK_EVT_STATE_ONESHOT_STOPPED: i32 = 4;
pub const CLOCK_EVT_STATE_SHUTDOWN: i32 = 0;
pub const KTIME_MAX: ktime_t = i64::MAX;
pub const ENODEV: i32 = 19;
pub const EINVAL: i32 = 22;
pub const EBUSY: i32 = 16;

extern "C" {
    static mut tick_cpu_device: tick_device;
    static mut cpu_online_mask: *mut cpumask;
    static mut jiffies_lock: c_void; static mut tick_next_period: ktime_t;
    fn smp_processor_id() -> i32; fn ktime_get() -> ktime_t;
    fn cpumask_test_cpu(i: i32, m: *const cpumask) -> bool; fn cpumask_set_cpu(i: i32,m:*mut cpumask);
    fn cpumask_clear_cpu(i:i32,m:*mut cpumask); fn cpumask_empty(m:*const cpumask)->bool;
    fn cpumask_test_and_set_cpu(i:i32,m:*mut cpumask)->bool; fn cpumask_test_and_clear_cpu(i:i32,m:*mut cpumask)->bool;
    fn cpumask_equal(a:*const cpumask,b:*const cpumask)->bool; fn cpumask_subset(a:*const cpumask,b:*const cpumask)->bool;
    fn cpumask_copy(d:*mut cpumask,s:*const cpumask); fn cpumask_and(d:*mut cpumask,a:*const cpumask,b:*const cpumask);
    fn cpumask_or(d:*mut cpumask,a:*const cpumask,b:*const cpumask); fn cpumask_of(i:i32)->*const cpumask;
    fn raw_spin_lock(_: *mut c_void); fn raw_spin_unlock(_: *mut c_void);
    fn raw_spin_lock_irqsave(_: *mut c_void, _: *mut usize); fn raw_spin_unlock_irqrestore(_: *mut c_void, _: usize);
    fn tick_setup_periodic(d:*mut clock_event_device, broadcast:i32); fn tick_broadcast(_: *mut cpumask);
    fn tick_handle_periodic(_: *mut clock_event_device); fn tick_clock_notify(); fn tick_receive_broadcast()->i32;
    fn tick_device_is_functional(_: *mut clock_event_device)->bool; fn clockevents_shutdown(_: *mut clock_event_device);
    fn clockevents_exchange_device(_: *mut clock_event_device,*mut clock_event_device); fn clockevents_handle_noop(_: *mut clock_event_device);
    fn clockevent_state_shutdown(_: *mut clock_event_device)->bool; fn clockevent_state_oneshot(_: *mut clock_event_device)->bool;
    fn clockevents_program_event(_: *mut clock_event_device,ktime_t,bool)->i32; fn clockevents_tick_resume(_: *mut clock_event_device);
    fn clockevents_switch_state(_: *mut clock_event_device,i32); fn tick_program_event(ktime_t,bool)->i32;
    fn __clockevents_update_freq(_: *mut clock_event_device,u32)->i32; fn try_module_get(_: *mut c_void)->bool;
}

static mut tick_broadcast_device: tick_device = tick_device { evtdev: core::ptr::null_mut(), mode: TICKDEV_MODE_PERIODIC };
static mut tick_broadcast_mask: cpumask_var_t = core::ptr::null_mut();
static mut tick_broadcast_on: cpumask_var_t = core::ptr::null_mut();
static mut tmpmask: cpumask_var_t = core::ptr::null_mut();
static mut tick_broadcast_forced: i32 = 0;
static mut tick_broadcast_lock: c_void = c_void{};

#[no_mangle] pub unsafe extern "C" fn tick_get_broadcast_device()->*mut tick_device { &raw mut tick_broadcast_device }
#[no_mangle] pub unsafe extern "C" fn tick_get_broadcast_mask()->*mut cpumask { tick_broadcast_mask }
unsafe fn tick_broadcast_start_periodic(bc:*mut clock_event_device) { if !bc.is_null() { (*bc).next_event_forced=0; tick_setup_periodic(bc,1); } }
unsafe fn tick_check_broadcast_device(cur:*mut clock_event_device,new:*mut clock_event_device)->bool {
    if (*new).features & (CLOCK_EVT_FEAT_DUMMY|CLOCK_EVT_FEAT_PERCPU|CLOCK_EVT_FEAT_C3STOP)!=0 { return false; }
    if tick_broadcast_device.mode==TICKDEV_MODE_ONESHOT && (*new).features&CLOCK_EVT_FEAT_ONESHOT==0 { return false; }
    cur.is_null() || (*new).rating>(*cur).rating
}
#[no_mangle] pub unsafe extern "C" fn tick_is_broadcast_device(d:*mut clock_event_device)->i32 { (!d.is_null() && tick_broadcast_device.evtdev==d) as i32 }
#[no_mangle] pub unsafe extern "C" fn tick_broadcast_update_freq(d:*mut clock_event_device,f:u32)->i32 { if tick_is_broadcast_device(d)!=0 { raw_spin_lock(&raw mut tick_broadcast_lock); let r=__clockevents_update_freq(d,f); raw_spin_unlock(&raw mut tick_broadcast_lock); r } else { -ENODEV } }
unsafe fn tick_device_setup_broadcast_func(d:*mut clock_event_device) { if (*d).broadcast.is_none() { (*d).broadcast=Some(tick_broadcast); } }
#[no_mangle] pub unsafe extern "C" fn tick_receive_broadcast_local()->i32 { let e=(*(&raw mut tick_cpu_device)).evtdev; if e.is_null(){return -ENODEV} if let Some(h)=(*e).event_handler {h(e);0} else {-EINVAL} }
#[no_mangle] pub unsafe extern "C" fn tick_device_uses_broadcast(d:*mut clock_event_device,cpu:i32)->i32 { let mut f=0usize; raw_spin_lock_irqsave(&raw mut tick_broadcast_lock,&raw mut f); let bc=tick_broadcast_device.evtdev; if !tick_device_is_functional(d) {(*d).event_handler=Some(tick_handle_periodic);tick_device_setup_broadcast_func(d);cpumask_set_cpu(cpu,tick_broadcast_mask);if tick_broadcast_device.mode==TICKDEV_MODE_PERIODIC{tick_broadcast_start_periodic(bc)};f=1}else if (*d).features&CLOCK_EVT_FEAT_C3STOP==0{cpumask_clear_cpu(cpu,tick_broadcast_mask)}else{tick_device_setup_broadcast_func(d)};raw_spin_unlock_irqrestore(&raw mut tick_broadcast_lock,f);f as i32 }
#[no_mangle] pub unsafe extern "C" fn tick_set_periodic_handler(d:*mut clock_event_device,b:i32){(*d).event_handler=Some(if b==0{tick_handle_periodic}else{tick_handle_periodic_broadcast});}
unsafe extern "C" fn tick_handle_periodic_broadcast(_: *mut clock_event_device) { }

#[no_mangle] pub unsafe extern "C" fn tick_broadcast_control(mode:tick_broadcast_mode){let mut f=0usize;raw_spin_lock_irqsave(&raw mut tick_broadcast_lock,&raw mut f);let d=(*(&raw mut tick_cpu_device)).evtdev;if !d.is_null()&&(*d).features&CLOCK_EVT_FEAT_C3STOP!=0&&tick_device_is_functional(d){let c=smp_processor_id();match mode{TICK_BROADCAST_FORCE=>{tick_broadcast_forced=1;cpumask_set_cpu(c,tick_broadcast_on);cpumask_set_cpu(c,tick_broadcast_mask)},TICK_BROADCAST_ON=>{cpumask_set_cpu(c,tick_broadcast_on);cpumask_set_cpu(c,tick_broadcast_mask)},TICK_BROADCAST_OFF=>if tick_broadcast_forced==0{cpumask_clear_cpu(c,tick_broadcast_on);cpumask_clear_cpu(c,tick_broadcast_mask)}, _=>{}}}raw_spin_unlock_irqrestore(&raw mut tick_broadcast_lock,f)}
#[no_mangle] pub unsafe extern "C" fn tick_broadcast_oneshot_active()->i32{(tick_broadcast_device.mode==TICKDEV_MODE_ONESHOT)as i32}
#[no_mangle] pub unsafe extern "C" fn tick_broadcast_oneshot_available()->bool{let b=tick_broadcast_device.evtdev;!b.is_null()&&(*b).features&CLOCK_EVT_FEAT_ONESHOT!=0}
#[no_mangle] pub unsafe extern "C" fn tick_broadcast_switch_to_oneshot(){tick_broadcast_device.mode=TICKDEV_MODE_ONESHOT;}
#[no_mangle] pub unsafe extern "C" fn __tick_broadcast_oneshot_control(_:tick_broadcast_state)->i32{-EBUSY}
#[no_mangle] pub unsafe extern "C" fn tick_suspend_broadcast(){let b=tick_broadcast_device.evtdev;if !b.is_null(){clockevents_shutdown(b)}}
#[no_mangle] pub unsafe extern "C" fn tick_resume_check_broadcast()->bool{tick_broadcast_device.mode!=TICKDEV_MODE_ONESHOT&&cpumask_test_cpu(smp_processor_id(),tick_broadcast_mask)}
#[no_mangle] pub unsafe extern "C" fn tick_install_broadcast_device(d:*mut clock_event_device,cpu:i32){let cur=tick_broadcast_device.evtdev;if d.is_null()||!tick_check_broadcast_device(cur,d)||!try_module_get((*d).owner){return}clockevents_exchange_device(cur,d);if !cur.is_null(){(*cur).event_handler=Some(clockevents_handle_noop)}tick_broadcast_device.evtdev=d;if !cpumask_empty(tick_broadcast_mask){tick_broadcast_start_periodic(d)};let _=cpu;}
#[no_mangle] pub unsafe extern "C" fn tick_broadcast_init(){/* cpumask allocation is supplied by the kernel */}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
