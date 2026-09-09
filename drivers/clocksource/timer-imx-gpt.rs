// SPDX-License-Identifier: GPL-2.0+
//
//  Copyright (C) 2000-2001 Deep Blue Solutions
//  Copyright (C) 2002 Shane Nay (shane@minirl.com)
//  Copyright (C) 2006-2007 Pavel Pisa (ppisa@pikron.com)
//  Copyright (C) 2008 Juergen Beisert (kernel@pengutronix.de)

// Linux kernel dependencies supplied by other translation units.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum imx_gpt_type { GPT_TYPE_IMX1, GPT_TYPE_IMX21, GPT_TYPE_IMX31, GPT_TYPE_IMX6DL }

const MXC_TCTL: usize = 0x00;
const MXC_TCTL_TEN: u32 = 1 << 0;
const MXC_TPRER: usize = 0x04;
const MX1_2_TCTL_CLK_PCLK1: u32 = 1 << 1;
const MX1_2_TCTL_IRQEN: u32 = 1 << 4;
const MX1_2_TCTL_FRR: u32 = 1 << 8;
const MX1_2_TCMP: usize = 0x08;
const MX1_2_TCN: usize = 0x10;
const MX1_2_TSTAT: usize = 0x14;
const MX2_TSTAT_CAPT: u32 = 1 << 1;
const MX2_TSTAT_COMP: u32 = 1 << 0;
const V2_TCTL_WAITEN: u32 = 1 << 3;
const V2_TCTL_CLK_IPG: u32 = 1 << 6;
const V2_TCTL_CLK_PER: u32 = 2 << 6;
const V2_TCTL_CLK_OSC_DIV8: u32 = 5 << 6;
const V2_TCTL_FRR: u32 = 1 << 9;
const V2_TCTL_24MEN: u32 = 1 << 10;
const V2_TPRER_PRE24M: u32 = 12;
const V2_IR: usize = 0x0c;
const V2_TSTAT: usize = 0x08;
const V2_TSTAT_OF1: u32 = 1 << 0;
const V2_TCN: usize = 0x24;
const V2_TCMP: usize = 0x10;
const V2_TIMER_RATE_OSC_DIV8: u32 = 3000000;

#[repr(C)]
pub struct imx_timer {
    pub type_: imx_gpt_type,
    pub base: *mut u8,
    pub irq: i32,
    pub clk_per: *mut clk,
    pub clk_ipg: *mut clk,
    pub gpt: *const imx_gpt_data,
    pub ced: clock_event_device,
}

#[repr(C)]
pub struct imx_gpt_data {
    pub reg_tstat: i32,
    pub reg_tcn: i32,
    pub reg_tcmp: i32,
    pub gpt_setup_tctl: Option<unsafe extern "C" fn(*mut imx_timer)>,
    pub gpt_irq_enable: Option<unsafe extern "C" fn(*mut imx_timer)>,
    pub gpt_irq_disable: Option<unsafe extern "C" fn(*mut imx_timer)>,
    pub gpt_irq_acknowledge: Option<unsafe extern "C" fn(*mut imx_timer)>,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
}

#[repr(C)] pub struct clk;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct clock_event_device {
    pub name: *const u8, pub features: u32,
    pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_state_oneshot: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub tick_resume: Option<unsafe extern "C" fn(*mut clock_event_device) -> i32>,
    pub set_next_event: Option<unsafe extern "C" fn(usize, *mut clock_event_device) -> i32>,
    pub rating: i32, pub cpumask: *const u8, pub irq: i32,
    pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>,
}
#[repr(C)] pub struct delay_timer { pub read_current_timer: Option<unsafe extern "C" fn() -> usize>, pub freq: u32 }

extern "C" {
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: u32);
    fn clocksource_mmio_init(reg: *mut u8, name: *const u8, rate: u32, rating: u32, bits: u32, read: *const u8) -> i32;
    fn register_current_timer_delay(timer: *mut delay_timer);
    fn clockevent_state_oneshot(ced: *mut clock_event_device) -> bool;
    fn clockevents_config_and_register(ced: *mut clock_event_device, rate: u32, min: u32, max: u32);
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut u8) -> i32, flags: u32, name: *const u8, dev: *mut u8) -> i32;
    fn cpumask_of(cpu: u32) -> *const u8;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut u8;
    fn irq_of_parse_and_map(np: *mut device_node, index: i32) -> i32;
    fn of_clk_get_by_name(np: *mut device_node, name: *const u8) -> *mut clk;
    fn of_machine_is_compatible(name: *const u8) -> bool;
    fn kzalloc(size: usize) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn pr_err(msg: *const u8);
}

static mut sched_clock_reg: *mut u8 = core::ptr::null_mut();
static mut imx_delay_timer: delay_timer = delay_timer { read_current_timer: None, freq: 0 };

unsafe fn to_imx_timer(ced: *mut clock_event_device) -> *mut imx_timer {
    (ced as *mut u8).sub(core::mem::offset_of!(imx_timer, ced)) as *mut imx_timer
}
unsafe extern "C" fn imx1_gpt_irq_disable(t: *mut imx_timer) { let v=readl_relaxed((*t).base.add(MXC_TCTL)); writel_relaxed(v & !MX1_2_TCTL_IRQEN, (*t).base.add(MXC_TCTL)); }
unsafe extern "C" fn imx31_gpt_irq_disable(t: *mut imx_timer) { writel_relaxed(0, (*t).base.add(V2_IR)); }
unsafe extern "C" fn imx1_gpt_irq_enable(t: *mut imx_timer) { let v=readl_relaxed((*t).base.add(MXC_TCTL)); writel_relaxed(v | MX1_2_TCTL_IRQEN, (*t).base.add(MXC_TCTL)); }
unsafe extern "C" fn imx31_gpt_irq_enable(t: *mut imx_timer) { writel_relaxed(1, (*t).base.add(V2_IR)); }
unsafe extern "C" fn imx1_gpt_irq_acknowledge(t: *mut imx_timer) { writel_relaxed(0, (*t).base.add(MX1_2_TSTAT)); }
unsafe extern "C" fn imx21_gpt_irq_acknowledge(t: *mut imx_timer) { writel_relaxed(MX2_TSTAT_CAPT|MX2_TSTAT_COMP, (*t).base.add(MX1_2_TSTAT)); }
unsafe extern "C" fn imx31_gpt_irq_acknowledge(t: *mut imx_timer) { writel_relaxed(V2_TSTAT_OF1, (*t).base.add(V2_TSTAT)); }
unsafe extern "C" fn mxc_read_sched_clock() -> u64 { if sched_clock_reg.is_null() { 0 } else { readl_relaxed(sched_clock_reg) as u64 } }
unsafe extern "C" fn imx_read_current_timer() -> usize { readl_relaxed(sched_clock_reg) as usize }

unsafe extern "C" fn mx1_2_set_next_event(evt: usize, ced: *mut clock_event_device) -> i32 { let t=to_imx_timer(ced); let cmp=readl_relaxed((*t).base.add(MX1_2_TCN)).wrapping_add(evt as u32); writel_relaxed(cmp,(*t).base.add(MX1_2_TCMP)); if (cmp.wrapping_sub(readl_relaxed((*t).base.add(MX1_2_TCN))) as i32)<0 {-62} else {0} }
unsafe extern "C" fn v2_set_next_event(evt: usize, ced: *mut clock_event_device) -> i32 { let t=to_imx_timer(ced); let cmp=readl_relaxed((*t).base.add(V2_TCN)).wrapping_add(evt as u32); writel_relaxed(cmp,(*t).base.add(V2_TCMP)); if evt<0x7fffffff && (cmp.wrapping_sub(readl_relaxed((*t).base.add(V2_TCN))) as i32)<0 {-62} else {0} }

// The remaining timer framework registration and device-tree declarations retain the C source's interfaces.
// External kernel types and registration macros are supplied by the surrounding translation unit.

unsafe extern "C" fn mxc_shutdown(ced: *mut clock_event_device) -> i32 { let t=to_imx_timer(ced); let g=&*(*t).gpt; (g.gpt_irq_disable.unwrap())(t); let n=readl_relaxed((*t).base.add(g.reg_tcn as usize)); writel_relaxed(n.wrapping_sub(3),(*t).base.add(g.reg_tcmp as usize)); (g.gpt_irq_acknowledge.unwrap())(t); 0 }
unsafe extern "C" fn mxc_set_oneshot(ced: *mut clock_event_device) -> i32 { let t=to_imx_timer(ced); let g=&*(*t).gpt; (g.gpt_irq_disable.unwrap())(t); if !clockevent_state_oneshot(ced) { let n=readl_relaxed((*t).base.add(g.reg_tcn as usize)); writel_relaxed(n.wrapping_sub(3),(*t).base.add(g.reg_tcmp as usize)); (g.gpt_irq_acknowledge.unwrap())(t); } (g.gpt_irq_enable.unwrap())(t); 0 }
unsafe extern "C" fn mxc_timer_interrupt(_irq: i32, dev_id: *mut u8) -> i32 { let ced=dev_id as *mut clock_event_device; let t=to_imx_timer(ced); let g=&*(*t).gpt; readl_relaxed((*t).base.add(g.reg_tstat as usize)); (g.gpt_irq_acknowledge.unwrap())(t); ( (*ced).event_handler.unwrap())(ced); 1 }

unsafe extern "C" fn mxc_clocksource_init(t: *mut imx_timer) -> i32 { let c=clk_get_rate((*t).clk_per); let reg=(*t).base.add((*(*t).gpt).reg_tcn as usize); sched_clock_reg=reg; sched_clock_register(mxc_read_sched_clock,32,c); clocksource_mmio_init(reg,b"mxc_timer1\0".as_ptr(),c,200,32,core::ptr::null()) }
unsafe extern "C" fn mxc_clockevent_init(t: *mut imx_timer) -> i32 { let c=&mut (*t).ced; c.name=b"mxc_timer1\0".as_ptr(); c.set_state_shutdown=Some(mxc_shutdown); c.set_state_oneshot=Some(mxc_set_oneshot); c.tick_resume=Some(mxc_shutdown); c.set_next_event=(*(*t).gpt).set_next_event; c.rating=200; c.cpumask=cpumask_of(0); c.irq=(*t).irq; clockevents_config_and_register(c,clk_get_rate((*t).clk_per),0xff,0xfffffffe); request_irq((*t).irq,mxc_timer_interrupt,0,b"i.MX Timer Tick\0".as_ptr(),c as *mut _ as *mut u8) }
unsafe extern "C" fn imx1_gpt_setup_tctl(t:*mut imx_timer) { writel_relaxed(MX1_2_TCTL_FRR|MX1_2_TCTL_CLK_PCLK1|MXC_TCTL_TEN,(*t).base.add(MXC_TCTL)); }
unsafe extern "C" fn imx31_gpt_setup_tctl(t:*mut imx_timer) { let mut v=V2_TCTL_FRR|V2_TCTL_WAITEN|MXC_TCTL_TEN; v|=if clk_get_rate((*t).clk_per)==V2_TIMER_RATE_OSC_DIV8 {V2_TCTL_CLK_OSC_DIV8} else {V2_TCTL_CLK_PER}; writel_relaxed(v,(*t).base.add(MXC_TCTL)); }
unsafe extern "C" fn imx6dl_gpt_setup_tctl(t:*mut imx_timer) { let mut v=V2_TCTL_FRR|V2_TCTL_WAITEN|MXC_TCTL_TEN; if clk_get_rate((*t).clk_per)==V2_TIMER_RATE_OSC_DIV8 {v|=V2_TCTL_CLK_OSC_DIV8; writel_relaxed(7<<V2_TPRER_PRE24M,(*t).base.add(MXC_TPRER)); v|=V2_TCTL_24MEN;} else {v|=V2_TCTL_CLK_PER;} writel_relaxed(v,(*t).base.add(MXC_TCTL)); }

static imx1_gpt_data: imx_gpt_data = imx_gpt_data {reg_tstat:MX1_2_TSTAT as i32,reg_tcn:MX1_2_TCN as i32,reg_tcmp:MX1_2_TCMP as i32,gpt_setup_tctl:Some(imx1_gpt_setup_tctl),gpt_irq_enable:Some(imx1_gpt_irq_enable),gpt_irq_disable:Some(imx1_gpt_irq_disable),gpt_irq_acknowledge:Some(imx1_gpt_irq_acknowledge),set_next_event:Some(mx1_2_set_next_event)};
static imx21_gpt_data: imx_gpt_data = imx_gpt_data {gpt_irq_acknowledge:Some(imx21_gpt_irq_acknowledge),..imx1_gpt_data};
static imx31_gpt_data: imx_gpt_data = imx_gpt_data {reg_tstat:V2_TSTAT as i32,reg_tcn:V2_TCN as i32,reg_tcmp:V2_TCMP as i32,gpt_setup_tctl:Some(imx31_gpt_setup_tctl),gpt_irq_enable:Some(imx31_gpt_irq_enable),gpt_irq_disable:Some(imx31_gpt_irq_disable),gpt_irq_acknowledge:Some(imx31_gpt_irq_acknowledge),set_next_event:Some(v2_set_next_event)};
static imx6dl_gpt_data: imx_gpt_data = imx_gpt_data {gpt_setup_tctl:Some(imx6dl_gpt_setup_tctl),..imx31_gpt_data};

unsafe extern "C" fn _mxc_timer_init(t:*mut imx_timer)->i32 { (*t).gpt=match (*t).type_ {imx_gpt_type::GPT_TYPE_IMX1=>&imx1_gpt_data,imx_gpt_type::GPT_TYPE_IMX21=>&imx21_gpt_data,imx_gpt_type::GPT_TYPE_IMX31=>&imx31_gpt_data,imx_gpt_type::GPT_TYPE_IMX6DL=>&imx6dl_gpt_data}; writel_relaxed(0,(*t).base.add(MXC_TCTL)); writel_relaxed(0,(*t).base.add(MXC_TPRER)); ((*(*t).gpt).gpt_setup_tctl.unwrap())(t); let r=mxc_clocksource_init(t); if r!=0 {return r} mxc_clockevent_init(t) }
static mut initialized: i32=0;
unsafe extern "C" fn mxc_timer_init_dt(np:*mut device_node, ty:imx_gpt_type)->i32 { if initialized!=0{return 0} let t=kzalloc(core::mem::size_of::<imx_timer>()) as *mut imx_timer; if t.is_null(){return -12} (*t).base=of_iomap(np,0); (*t).irq=irq_of_parse_and_map(np,0); (*t).clk_ipg=of_clk_get_by_name(np,b"ipg\0".as_ptr()); (*t).clk_per=of_clk_get_by_name(np,b"osc_per\0".as_ptr()); (*t).type_=ty; let r=_mxc_timer_init(t); if r!=0{kfree(t as *mut u8);return r} initialized=1; 0 }
unsafe extern "C" fn imx1_timer_init_dt(np:*mut device_node)->i32{mxc_timer_init_dt(np,imx_gpt_type::GPT_TYPE_IMX1)}
unsafe extern "C" fn imx21_timer_init_dt(np:*mut device_node)->i32{mxc_timer_init_dt(np,imx_gpt_type::GPT_TYPE_IMX21)}
unsafe extern "C" fn imx31_timer_init_dt(np:*mut device_node)->i32{let ty=if of_machine_is_compatible(b"fsl,imx6dl\0".as_ptr()){imx_gpt_type::GPT_TYPE_IMX6DL}else{imx_gpt_type::GPT_TYPE_IMX31};mxc_timer_init_dt(np,ty)}
unsafe extern "C" fn imx6dl_timer_init_dt(np:*mut device_node)->i32{mxc_timer_init_dt(np,imx_gpt_type::GPT_TYPE_IMX6DL)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
