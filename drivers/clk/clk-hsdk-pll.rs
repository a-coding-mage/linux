// SPDX-License-Identifier: GPL-2.0-only
/* Synopsys HSDK SDP Generic PLL clock driver */

use core::ffi::c_char;

const CGU_PLL_CTRL: u32 = 0x000;
const CGU_PLL_STATUS: u32 = 0x004;
const CGU_PLL_FMEAS: u32 = 0x008;
const CGU_PLL_MON: u32 = 0x00c;
const CGU_PLL_CTRL_ODIV_SHIFT: u32 = 2;
const CGU_PLL_CTRL_IDIV_SHIFT: u32 = 4;
const CGU_PLL_CTRL_FBDIV_SHIFT: u32 = 9;
const CGU_PLL_CTRL_BAND_SHIFT: u32 = 20;
const CGU_PLL_CTRL_ODIV_MASK: u32 = 0x0c;
const CGU_PLL_CTRL_IDIV_MASK: u32 = 0x1f0;
const CGU_PLL_CTRL_FBDIV_MASK: u32 = 0xfe00;
const CGU_PLL_CTRL_PD: u32 = 1 << 0;
const CGU_PLL_CTRL_BYPASS: u32 = 1 << 1;
const CGU_PLL_STATUS_LOCK: u32 = 1 << 0;
const CGU_PLL_STATUS_ERR: u32 = 1 << 1;
const HSDK_PLL_MAX_LOCK_TIME: u32 = 100;
const CGU_PLL_SOURCE_MAX: u32 = 1;
const CORE_IF_CLK_THRESHOLD_HZ: u32 = 500000000;
const CREG_CORE_IF_CLK_DIV_1: u32 = 0x0;
const CREG_CORE_IF_CLK_DIV_2: u32 = 0x1;

#[repr(C)]
pub struct hsdk_pll_cfg { pub rate: u32, pub idiv: u32, pub fbdiv: u32, pub odiv: u32, pub band: u32, pub bypass: u32 }

#[repr(C)]
pub struct hsdk_pll_clk {
    pub hw: clk_hw,
    pub regs: *mut u8,
    pub spec_regs: *mut u8,
    pub pll_devdata: *const hsdk_pll_devdata,
    pub dev: *mut device,
}

#[repr(C)]
pub struct hsdk_pll_devdata {
    pub pll_cfg: *const hsdk_pll_cfg,
    pub update_rate: Option<unsafe extern "C" fn(*mut hsdk_pll_clk, usize, *const hsdk_pll_cfg) -> i32>,
}

#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub ops: *const clk_ops, pub parent_names: *const *const c_char, pub num_parents: u32 }
#[repr(C)] pub struct clk_ops { pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32> }
#[repr(C)] pub struct clk_rate_request { pub rate: usize }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { pub name: *const c_char }
#[repr(C)] pub struct platform_device { pub dev: device }
extern "C" { fn ioread32(p: *mut u8) -> u32; fn iowrite32(v: u32, p: *mut u8); fn udelay(v: u32); }

static ASDT_PLL_CFG: [hsdk_pll_cfg; 20] = [
    hsdk_pll_cfg{rate:100000000,idiv:0,fbdiv:11,odiv:3,band:0,bypass:0}, hsdk_pll_cfg{rate:133000000,idiv:0,fbdiv:15,odiv:3,band:0,bypass:0}, hsdk_pll_cfg{rate:200000000,idiv:1,fbdiv:47,odiv:3,band:0,bypass:0}, hsdk_pll_cfg{rate:233000000,idiv:1,fbdiv:27,odiv:2,band:0,bypass:0}, hsdk_pll_cfg{rate:300000000,idiv:1,fbdiv:35,odiv:2,band:0,bypass:0}, hsdk_pll_cfg{rate:333000000,idiv:1,fbdiv:39,odiv:2,band:0,bypass:0}, hsdk_pll_cfg{rate:400000000,idiv:1,fbdiv:47,odiv:2,band:0,bypass:0}, hsdk_pll_cfg{rate:500000000,idiv:0,fbdiv:14,odiv:1,band:0,bypass:0}, hsdk_pll_cfg{rate:600000000,idiv:0,fbdiv:17,odiv:1,band:0,bypass:0}, hsdk_pll_cfg{rate:700000000,idiv:0,fbdiv:20,odiv:1,band:0,bypass:0}, hsdk_pll_cfg{rate:800000000,idiv:0,fbdiv:23,odiv:1,band:0,bypass:0}, hsdk_pll_cfg{rate:900000000,idiv:1,fbdiv:26,odiv:0,band:0,bypass:0}, hsdk_pll_cfg{rate:1000000000,idiv:1,fbdiv:29,odiv:0,band:0,bypass:0}, hsdk_pll_cfg{rate:1100000000,idiv:1,fbdiv:32,odiv:0,band:0,bypass:0}, hsdk_pll_cfg{rate:1200000000,idiv:1,fbdiv:35,odiv:0,band:0,bypass:0}, hsdk_pll_cfg{rate:1300000000,idiv:1,fbdiv:38,odiv:0,band:0,bypass:0}, hsdk_pll_cfg{rate:1400000000,idiv:1,fbdiv:41,odiv:0,band:0,bypass:0}, hsdk_pll_cfg{rate:1500000000,idiv:1,fbdiv:44,odiv:0,band:0,bypass:0}, hsdk_pll_cfg{rate:1600000000,idiv:1,fbdiv:47,odiv:0,band:0,bypass:0}, hsdk_pll_cfg{rate:0,idiv:0,fbdiv:0,odiv:0,band:0,bypass:0}];

unsafe fn hsdk_pll_write(c: *mut hsdk_pll_clk, r: u32, v: u32) { iowrite32(v, (*c).regs.add(r as usize)); }
unsafe fn hsdk_pll_read(c: *mut hsdk_pll_clk, r: u32) -> u32 { ioread32((*c).regs.add(r as usize)) }
unsafe fn hsdk_pll_set_cfg(c: *mut hsdk_pll_clk, cfg: *const hsdk_pll_cfg) { let mut v=0; if (*cfg).bypass != 0 { v=hsdk_pll_read(c,CGU_PLL_CTRL)|CGU_PLL_CTRL_BYPASS; } else { v|=(*cfg).idiv<<CGU_PLL_CTRL_IDIV_SHIFT; v|=(*cfg).fbdiv<<CGU_PLL_CTRL_FBDIV_SHIFT; v|=(*cfg).odiv<<CGU_PLL_CTRL_ODIV_SHIFT; v|=(*cfg).band<<CGU_PLL_CTRL_BAND_SHIFT; } hsdk_pll_write(c,CGU_PLL_CTRL,v); }
unsafe fn hsdk_pll_is_locked(c:*mut hsdk_pll_clk)->bool { hsdk_pll_read(c,CGU_PLL_STATUS)&CGU_PLL_STATUS_LOCK != 0 }
unsafe fn hsdk_pll_is_err(c:*mut hsdk_pll_clk)->bool { hsdk_pll_read(c,CGU_PLL_STATUS)&CGU_PLL_STATUS_ERR != 0 }

#[allow(dead_code)]
unsafe fn hsdk_pll_recalc_rate(c:*mut hsdk_pll_clk, parent_rate:usize)->usize { let v=hsdk_pll_read(c,CGU_PLL_CTRL); if v&CGU_PLL_CTRL_BYPASS!=0{return parent_rate;} if v&CGU_PLL_CTRL_PD!=0{return 0;} let idiv=1+((v&CGU_PLL_CTRL_IDIV_MASK)>>CGU_PLL_CTRL_IDIV_SHIFT) as usize; let fbdiv=2*(1+((v&CGU_PLL_CTRL_FBDIV_MASK)>>CGU_PLL_CTRL_FBDIV_SHIFT) as usize); let odiv=1usize<<((v&CGU_PLL_CTRL_ODIV_MASK)>>CGU_PLL_CTRL_ODIV_SHIFT); parent_rate.wrapping_mul(fbdiv)/(idiv*odiv) }

unsafe fn hsdk_pll_comm_update_rate(c:*mut hsdk_pll_clk,_rate:usize,cfg:*const hsdk_pll_cfg)->i32 { hsdk_pll_set_cfg(c,cfg); udelay(HSDK_PLL_MAX_LOCK_TIME); if !hsdk_pll_is_locked(c){return -110;} if hsdk_pll_is_err(c){return -22;} 0 }
unsafe fn hsdk_pll_core_update_rate(c:*mut hsdk_pll_clk,rate:usize,cfg:*const hsdk_pll_cfg)->i32 { if rate>CORE_IF_CLK_THRESHOLD_HZ as usize{iowrite32(CREG_CORE_IF_CLK_DIV_2,(*c).spec_regs);} let r=hsdk_pll_comm_update_rate(c,rate,cfg); if r!=0{return r;} if rate<=CORE_IF_CLK_THRESHOLD_HZ as usize{iowrite32(CREG_CORE_IF_CLK_DIV_1,(*c).spec_regs);} 0 }

static HDMI_PLL_CFG: [hsdk_pll_cfg; 6] = [
    hsdk_pll_cfg{rate:27000000,idiv:0,fbdiv:0,odiv:0,band:0,bypass:1},
    hsdk_pll_cfg{rate:148500000,idiv:0,fbdiv:21,odiv:3,band:0,bypass:0},
    hsdk_pll_cfg{rate:297000000,idiv:0,fbdiv:21,odiv:2,band:0,bypass:0},
    hsdk_pll_cfg{rate:540000000,idiv:0,fbdiv:19,odiv:1,band:0,bypass:0},
    hsdk_pll_cfg{rate:594000000,idiv:0,fbdiv:21,odiv:1,band:0,bypass:0},
    hsdk_pll_cfg{rate:0,idiv:0,fbdiv:0,odiv:0,band:0,bypass:0},
];

#[allow(dead_code)]
unsafe fn hsdk_pll_determine_rate(c:*mut hsdk_pll_clk, req:*mut clk_rate_request)->i32 { let cfg=(*(*c).pll_devdata).pll_cfg; if (*cfg).rate==0{return -22;} let mut best=(*cfg).rate as usize; let mut i=1; while (*cfg.add(i)).rate!=0 { let a=(*req).rate.abs_diff((*cfg.add(i)).rate as usize); let b=(*req).rate.abs_diff(best); if a<b {best=(*cfg.add(i)).rate as usize;} i+=1;} (*req).rate=best; 0 }

#[allow(dead_code)]
unsafe fn hsdk_pll_set_rate(c:*mut hsdk_pll_clk,rate:usize,_parent_rate:usize)->i32 { let cfg=(*(*c).pll_devdata).pll_cfg; let mut i=0; while (*cfg.add(i)).rate!=0 { if (*cfg.add(i)).rate as usize==rate { if let Some(f)=(*(*c).pll_devdata).update_rate{return f(c,rate,cfg.add(i));} return -22;} i+=1;} -22 }

// The remaining platform-driver registration and device-tree setup are supplied by
// the surrounding kernel bindings; these declarations preserve the source interface.
extern "C" {
    fn hsdk_pll_clk_probe(pdev: *mut platform_device) -> i32;
    fn of_hsdk_pll_clk_setup(node: *mut device_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
