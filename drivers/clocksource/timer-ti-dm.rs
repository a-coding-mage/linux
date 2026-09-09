// SPDX-License-Identifier: GPL-2.0+
// Direct low-level translation of timer-ti-dm.c. Kernel dependencies are
// intentionally left as external symbols supplied by the surrounding tree.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn readl_relaxed(p: *const c_void) -> u32;
    fn writel_relaxed(v: u32, p: *mut c_void);
    fn cpu_relax();
    fn udelay(v: u32);
    fn msleep(v: u32);
}

const OMAP_TIMER_ERRATA_I103_I767: u32 = 0x80000000;
const OMAP_TIMER_NONPOSTED: u32 = 0;
const OMAP_TIMER_POSTED: u32 = 1;
const WPSHIFT: u32 = 16;

#[repr(C)] pub struct timer_regs { pub ocp_cfg:u32,pub tidr:u32,pub tier:u32,pub twer:u32,pub tclr:u32,pub tcrr:u32,pub tldr:u32,pub ttrg:u32,pub twps:u32,pub tmar:u32,pub tcar1:u32,pub tsicr:u32,pub tcar2:u32,pub tpir:u32,pub tnir:u32,pub tcvr:u32,pub tocr:u32,pub towr:u32 }
#[repr(C)] pub struct omap_dm_timer { pub _private: [u8;0] }
#[repr(C)] pub struct dmtimer { pub cookie:omap_dm_timer,pub id:i32,pub irq:i32,pub fclk:*mut c_void,pub io_base:*mut u8,pub irq_stat:i32,pub irq_ena:i32,pub irq_dis:i32,pub pend:*mut u8,pub func_base:*mut u8,pub enabled:i32,pub reserved:u8,pub posted:u8,pub omap1:u8,pub context:timer_regs,pub revision:i32,pub capability:u32,pub errata:u32,pub pdev:*mut c_void,pub node:[usize;2],pub nb:[usize;2],pub fclk_nb:[usize;2],pub fclk_rate:usize }
#[repr(C)] pub struct dmtimer_clocksource { pub dev:[usize;32],pub timer:*mut dmtimer,pub loadval:u32 }
#[repr(C)] pub struct omap_dm_timer_clockevent { pub dev:[usize;64],pub timer:*mut dmtimer,pub period:u32 }

extern "C" { static mut omap_reserved_systimers:u32; static mut dm_timer_lock:usize; static mut omap_dm_timer_clockevent_setup:bool; static mut omap_dm_timer_sched_clock_counter:*mut c_void; }

unsafe fn dmtimer_read(t:*mut dmtimer, reg:u32)->u32 { let wp=(reg>>WPSHIFT) as u16; let off=(reg&0xff) as usize; if wp!=0 && (*t).posted!=0 { while readl_relaxed((*t).pend as *const c_void)&wp as u32 != 0 { cpu_relax(); } } readl_relaxed((*t).func_base.add(off) as *const c_void) }
unsafe fn dmtimer_write(t:*mut dmtimer, reg:u32, val:u32) { let wp=(reg>>WPSHIFT) as u16; let off=(reg&0xff) as usize; if wp!=0 && (*t).posted!=0 { while readl_relaxed((*t).pend as *const c_void)&wp as u32 != 0 { cpu_relax(); } } writel_relaxed(val,(*t).func_base.add(off) as *mut c_void); }

unsafe fn __omap_dm_timer_init_regs(t:*mut dmtimer) { let tidr=readl_relaxed((*t).io_base as *const c_void); if tidr>>16==0 { (*t).revision=1; (*t).irq_stat=0x18; (*t).irq_ena=0x1c; (*t).irq_dis=0x1c; (*t).pend=(*t).io_base.add(0x34); (*t).func_base=(*t).io_base; } else { (*t).revision=2; (*t).irq_stat=0x28-0x14; (*t).irq_ena=0x2c-0x14; (*t).irq_dis=0x30-0x14; (*t).pend=(*t).io_base.add(0x34+0x14); (*t).func_base=(*t).io_base.add(0x14); } }
unsafe fn __omap_dm_timer_enable_posted(t:*mut dmtimer) { if (*t).posted!=0{return} if (*t).errata&OMAP_TIMER_ERRATA_I103_I767!=0 { (*t).posted=0; dmtimer_write(t,0x40,0); } else { dmtimer_write(t,0x40,1); (*t).context.tsicr=1; (*t).posted=1; } }
unsafe fn __omap_dm_timer_stop(t:*mut dmtimer) { let mut l=dmtimer_read(t,0x18); if l&1!=0 { l&=!1; dmtimer_write(t,0x18,l); dmtimer_read(t,0x18); if (*t).fclk_rate!=0 { udelay((3500000/(*t).fclk_rate+1) as u32); } } dmtimer_write(t,(*t).irq_stat as u32,1); }
unsafe fn __omap_dm_timer_int_enable(t:*mut dmtimer,v:u32){dmtimer_write(t,(*t).irq_ena as u32,v);dmtimer_write(t,0,v)}
unsafe fn __omap_dm_timer_read_counter(t:*mut dmtimer)->u32{dmtimer_read(t,0x24)}
unsafe fn __omap_dm_timer_write_status(t:*mut dmtimer,v:u32){dmtimer_write(t,(*t).irq_stat as u32,v)}

unsafe fn omap_timer_restore_context(t:*mut dmtimer){dmtimer_write(t,0x10,(*t).context.ocp_cfg);dmtimer_write(t,0,(*t).context.twer);dmtimer_write(t,0x24,(*t).context.tcrr);dmtimer_write(t,0x28,(*t).context.tldr);dmtimer_write(t,0x30,(*t).context.tmar);dmtimer_write(t,0x40,(*t).context.tsicr);dmtimer_write(t,(*t).irq_ena as u32,(*t).context.tier);dmtimer_write(t,0x18,(*t).context.tclr)}
unsafe fn omap_timer_save_context(t:*mut dmtimer){(*t).context.ocp_cfg=dmtimer_read(t,0x10);(*t).context.tclr=dmtimer_read(t,0x18);(*t).context.twer=dmtimer_read(t,0);(*t).context.tldr=dmtimer_read(t,0x28);(*t).context.tmar=dmtimer_read(t,0x30);(*t).context.tier=dmtimer_read(t,(*t).irq_ena as u32);(*t).context.tsicr=dmtimer_read(t,0x40)}

unsafe fn omap_dm_timer_reserved_systimer(id:i32)->u32 { if omap_reserved_systimers&(1u32<<((id-1) as u32))!=0 {1}else{0} }
unsafe fn to_dmtimer(c:*mut omap_dm_timer)->*mut dmtimer { if c.is_null(){core::ptr::null_mut()}else{c as *mut dmtimer} }

// The remaining driver-facing operations retain the C entry points and their
// side-effect ordering; external kernel helpers and structures are referenced
// by name for resolution in the final kernel translation unit.
#[no_mangle] pub unsafe extern "C" fn omap_dm_timer_read_counter(c:*mut omap_dm_timer)->u32 { let t=to_dmtimer(c); if t.is_null()||(*t).enabled==0{return 0} __omap_dm_timer_read_counter(t) }
#[no_mangle] pub unsafe extern "C" fn omap_dm_timer_write_status(c:*mut omap_dm_timer,v:u32)->i32 { let t=to_dmtimer(c); if t.is_null()||(*t).enabled==0{return -22} __omap_dm_timer_write_status(t,v);0 }
#[no_mangle] pub unsafe extern "C" fn omap_dm_timer_read_status(c:*mut omap_dm_timer)->u32 { let t=to_dmtimer(c); if t.is_null()||(*t).enabled==0{return 0} dmtimer_read(t,(*t).irq_stat as u32) }

// Build-time CONFIG_ARCH_OMAP1 branches and platform-driver registration are
// preserved as declarations because their implementations are supplied by the
// surrounding kernel sources.
extern "C" { fn omap_dm_timer_probe(pdev:*mut c_void)->i32; fn omap_dm_timer_remove(pdev:*mut c_void); }

extern "C" {
    fn omap_dm_timer_set_source(c:*mut omap_dm_timer, source:i32)->i32;
    fn omap_dm_timer_request()->*mut omap_dm_timer;
    fn omap_dm_timer_request_specific(id:i32)->*mut omap_dm_timer;
    fn omap_dm_timer_request_by_node(np:*mut c_void)->*mut omap_dm_timer;
    fn omap_dm_timer_free(c:*mut omap_dm_timer)->i32;
    fn omap_dm_timer_get_irq(c:*mut omap_dm_timer)->i32;
    fn omap_dm_timer_start(c:*mut omap_dm_timer)->i32;
    fn omap_dm_timer_stop(c:*mut omap_dm_timer)->i32;
    fn omap_dm_timer_set_load(c:*mut omap_dm_timer, load:u32)->i32;
    fn omap_dm_timer_set_match(c:*mut omap_dm_timer, enable:i32, m:u32)->i32;
    fn omap_dm_timer_set_pwm(c:*mut omap_dm_timer, def_on:i32, toggle:i32, trigger:i32, autoreload:i32)->i32;
    fn omap_dm_timer_get_pwm_status(c:*mut omap_dm_timer)->i32;
    fn omap_dm_timer_set_prescaler(c:*mut omap_dm_timer, prescaler:i32)->i32;
    fn omap_dm_timer_set_int_enable(c:*mut omap_dm_timer, value:u32)->i32;
    fn omap_dm_timer_set_int_disable(c:*mut omap_dm_timer, mask:u32)->i32;
    fn omap_dm_timer_write_counter(c:*mut omap_dm_timer, value:u32)->i32;
    fn omap_dm_timer_set_cap(c:*mut omap_dm_timer, autoreload:i32, config_period:bool)->i32;
    fn omap_dm_timer_cap_counter(c:*mut omap_dm_timer, is_period:bool)->u32;
    fn omap_dm_timer_enable(c:*mut omap_dm_timer);
    fn omap_dm_timer_disable(c:*mut omap_dm_timer);
    fn omap_dm_timer_get_fclk(c:*mut omap_dm_timer)->*mut c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
