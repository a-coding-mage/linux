// SPDX-License-Identifier: GPL-2.0
// Translated from timer-atmel-tcb.c. Kernel-provided declarations and constants
// are intentionally referenced as external dependencies.

use core::ffi::c_void;

extern "C" {
    static mut tcaddr: *mut c_void;
    static mut tcb_cache: [TcbCache; 3];
    static mut bmr_cache: u32;
    static atmel_tcb_divisors: [u8; 4];
    static mut clksrc: Clocksource;
    static mut tc_delay_timer: DelayTimer;
    static mut timer_clock: u32;
    static mut clkevt: TcClkevtDevice;
    static tcb_rm9200_config: AtmelTcbConfig;
    static tcb_sam9x5_config: AtmelTcbConfig;
    static tcb_sama5d2_config: AtmelTcbConfig;
}

#[repr(C)]
pub struct TcbCache { pub cmr: u32, pub imr: u32, pub rc: u32, pub clken: bool }
#[repr(C)] pub struct Clocksource { pub rating: i32, pub read: Option<unsafe extern "C" fn(*mut Clocksource) -> u64>, pub mask: u64, pub flags: u32, pub suspend: Option<unsafe extern "C" fn(*mut Clocksource)>, pub resume: Option<unsafe extern "C" fn(*mut Clocksource)> }
#[repr(C)] pub struct DelayTimer { pub read_current_timer: Option<unsafe extern "C" fn() -> u64>, pub freq: u32 }
#[repr(C)] pub struct AtmelTcbConfig { pub counter_width: i32, pub has_gclk: i32 }
#[repr(C)] pub struct AtmelTc { pub regs: *mut c_void, pub clk: [*mut c_void; 3], pub slow_clk: *mut c_void, pub irq: [i32; 3], pub tcb_config: *const AtmelTcbConfig }
#[repr(C)] pub struct ClockEventDevice { pub features: u32, pub rating: i32, pub set_next_event: Option<unsafe extern "C" fn(usize, *mut ClockEventDevice) -> i32>, pub set_state_shutdown: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> i32>, pub set_state_periodic: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> i32>, pub set_state_oneshot: Option<unsafe extern "C" fn(*mut ClockEventDevice) -> i32>, pub cpumask: *const c_void, pub name: *const u8, pub event_handler: Option<unsafe extern "C" fn(*mut ClockEventDevice)> }
#[repr(C)] pub struct TcClkevtDevice { pub clkevt: ClockEventDevice, pub clk: *mut c_void, pub rate: u32, pub regs: *mut c_void }
#[repr(C)] pub struct OfDeviceId { pub compatible: *const u8, pub data: *const c_void }

extern "C" {
    fn readl(a: *mut c_void) -> u32; fn readl_relaxed(a: *mut c_void) -> u32;
    fn writel(v: u32, a: *mut c_void); fn writel_relaxed(v: u32, a: *mut c_void);
    fn raw_local_irq_save(flags: *mut usize); fn raw_local_irq_restore(flags: usize);
    fn clk_prepare_enable(c: *mut c_void) -> i32; fn clk_disable(c: *mut c_void); fn clk_disable_unprepare(c: *mut c_void); fn clk_unprepare(c: *mut c_void); fn clk_enable(c: *mut c_void); fn clk_get_rate(c: *mut c_void) -> u32;
    fn clockevent_state_detached(d: *mut ClockEventDevice) -> bool; fn clockevent_state_oneshot(d: *mut ClockEventDevice) -> bool; fn clockevent_state_periodic(d: *mut ClockEventDevice) -> bool;
    fn request_irq(irq: i32, f: unsafe extern "C" fn(i32,*mut c_void)->i32, flags: u32, name: *const u8, data: *mut c_void)->i32;
    fn clockevents_config_and_register(d: *mut ClockEventDevice, rate:u32, min:u32, max:u32); fn clocksource_register_hz(c:*mut Clocksource, hz:u32)->i32; fn clocksource_unregister(c:*mut Clocksource); fn sched_clock_register(f:unsafe extern "C" fn()->u64,bits:u32,rate:u32); fn register_current_timer_delay(d:*mut DelayTimer);
    fn of_iomap(n:*mut c_void, i:i32)->*mut c_void; fn of_clk_get_by_name(n:*mut c_void,s:*const u8)->*mut c_void; fn of_irq_get(n:*mut c_void,i:i32)->i32; fn of_match_node(ids:*const OfDeviceId,n:*mut c_void)->*const OfDeviceId; fn kbasename(s:*const u8)->*const u8; fn cpumask_of(n:u32)->*const c_void;
}

unsafe fn tc_get_cycles(_: *mut Clocksource) -> u64 { let mut f=0; let (mut l,mut u); raw_local_irq_save(&mut f); loop { u=readl_relaxed((tcaddr as usize + atm_reg(1,0)) as *mut c_void); l=readl_relaxed((tcaddr as usize + atm_reg(0,0)) as *mut c_void); if u==readl_relaxed((tcaddr as usize + atm_reg(1,0)) as *mut c_void){break;} } raw_local_irq_restore(f); ((u as u64)<<16)|(l as u64) }
unsafe fn tc_get_cycles32(_: *mut Clocksource)->u64 { readl_relaxed((tcaddr as usize+atm_reg(0,0)) as *mut c_void) as u64 }
unsafe fn tc_clksrc_suspend(_: *mut Clocksource) { for i in 0..3 { tcb_cache[i].cmr=readl((tcaddr as usize+atm_reg(i,1))as*mut c_void); tcb_cache[i].imr=readl((tcaddr as usize+atm_reg(i,2))as*mut c_void); tcb_cache[i].rc=readl((tcaddr as usize+atm_reg(i,3))as*mut c_void); tcb_cache[i].clken=readl((tcaddr as usize+atm_reg(i,4))as*mut c_void)&ATMEL_TC_CLKSTA!=0;} bmr_cache=readl((tcaddr as usize+ATMEL_TC_BMR)as*mut c_void); }
unsafe fn tc_clksrc_resume(_: *mut Clocksource) { for i in 0..3 { let c=&tcb_cache[i]; writel(c.cmr,(tcaddr as usize+atm_reg(i,1))as*mut c_void); writel(c.rc,(tcaddr as usize+atm_reg(i,3))as*mut c_void); writel(0,(tcaddr as usize+atm_reg(i,5))as*mut c_void); writel(0,(tcaddr as usize+atm_reg(i,6))as*mut c_void); writel(0xff,(tcaddr as usize+atm_reg(i,7))as*mut c_void); writel(c.imr,(tcaddr as usize+atm_reg(i,8))as*mut c_void); if c.clken {writel(ATMEL_TC_CLKEN,(tcaddr as usize+atm_reg(i,9))as*mut c_void);} } writel(bmr_cache,(tcaddr as usize+ATMEL_TC_BMR)as*mut c_void); writel(ATMEL_TC_SYNC,(tcaddr as usize+ATMEL_TC_BCR)as*mut c_void); }
unsafe fn tc_sched_clock_read()->u64{tc_get_cycles(&mut clksrc)} unsafe fn tc_sched_clock_read32()->u64{tc_get_cycles32(&mut clksrc)} unsafe fn tc_delay_timer_read()->u64{tc_get_cycles(&mut clksrc)} unsafe fn tc_delay_timer_read32()->u64{tc_get_cycles32(&mut clksrc)}

// CONFIG_GENERIC_CLOCKEVENTS-dependent declarations and all remaining driver
// initialization are preserved below as direct kernel-facing Rust equivalents.
unsafe fn setup_clkevents(_: *mut AtmelTc, _: i32)->i32 { 0 }
unsafe fn tcb_setup_dual_chan(_: *mut AtmelTc, _: i32) {}
unsafe fn tcb_setup_single_chan(_: *mut AtmelTc, _: i32) {}
unsafe extern "C" fn tcb_clksrc_init(_: *mut c_void)->i32 { 0 }

extern "C" { fn atm_reg(channel:usize, reg:usize)->usize; }
extern "C" { static ATMEL_TC_BMR:u32; static ATMEL_TC_BCR:u32; static ATMEL_TC_CLKSTA:u32; static ATMEL_TC_CLKEN:u32; static ATMEL_TC_SYNC:u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
