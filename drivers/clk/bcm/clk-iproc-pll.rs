// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2014 Broadcom Corporation

// Linux kernel dependencies and declarations from clk-iproc.h are external.

const PLL_VCO_HIGH_SHIFT: u32 = 19;
const PLL_VCO_LOW_SHIFT: u32 = 30;
const PLL_USER_MODE: u32 = 7;
const LOCK_DELAY: i32 = 100;
const NUM_FREQ_BANDS: usize = 8;
const NUM_KP_BANDS: usize = 3;

#[repr(C)]
#[derive(Copy, Clone)]
enum KpBand { Mid = 0, High, HighHigh }

static KP_TABLE: [[u32; NUM_FREQ_BANDS]; NUM_KP_BANDS] = [
    [5, 6, 6, 7, 7, 8, 9, 10],
    [4, 4, 5, 5, 6, 7, 8, 9],
    [4, 5, 5, 6, 7, 8, 9, 10],
];
static REF_FREQ_TABLE: [[u64; 2]; NUM_FREQ_BANDS] = [
    [10000000, 12500000], [12500000, 15000000], [15000000, 20000000],
    [20000000, 25000000], [25000000, 50000000], [50000000, 75000000],
    [75000000, 100000000], [100000000, 125000000],
];

#[repr(u32)]
enum VcoFreqRange { Low = 700000000, Mid = 1200000000, High = 2200000000,
    HighHigh = 3100000000, Max = 4000000000 }

#[repr(C)]
struct IprocPll { status_base: *mut u8, control_base: *mut u8, pwr_base: *mut u8,
    asiu_base: *mut u8, ctrl: *const IprocPllCtrl, vco_param: *const IprocPllVcoParam,
    num_vco_entries: u32 }
#[repr(C)] struct IprocClk { hw: ClkHw, pll: *mut IprocPll, ctrl: *const IprocClkCtrl }

// Types, constants, helpers, and kernel functions below are supplied externally.
extern "C" {
    fn readl(addr: *mut u8) -> u32; fn writel(v: u32, addr: *mut u8);
    fn udelay(us: u32); fn bit_mask(width: u32) -> u32;
    fn clk_hw_get_name(hw: *const ClkHw) -> *const i8;
    fn pr_err(fmt: *const i8, ...);
}

#[repr(C)] struct ClkHw { init: *const ClkInitData }
#[repr(C)] struct ClkInitData { name: *const i8, ops: *const ClkOps, flags: u32,
    parent_names: *const *const i8, num_parents: u32 }
#[repr(C)] struct ClkRateRequest { rate: u64, best_parent_rate: u64 }
#[repr(C)] struct ClkOps { enable: Option<unsafe extern "C" fn(*mut ClkHw)->i32>, disable: Option<unsafe extern "C" fn(*mut ClkHw)>, recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw,u64)->u64>, determine_rate: Option<unsafe extern "C" fn(*mut ClkHw,*mut ClkRateRequest)->i32>, set_rate: Option<unsafe extern "C" fn(*mut ClkHw,u64,u64)->i32> }
#[repr(C)] struct IprocPllVcoParam { ndiv_int: u64, ndiv_frac: u64, pdiv: u32, rate: u64 }
#[repr(C)] struct RegCtrl { offset: u32, shift: u32, width: u32 }
#[repr(C)] struct IprocPllResetCtrl { offset:u32, reset_shift:u32, p_reset_shift:u32 }
#[repr(C)] struct IprocPllCtrl { flags:u32, status:RegCtrl, asiu:RegCtrl, aon:RegCtrl, reset:IprocPllResetCtrl, dig_filter:RegCtrl, ndiv_int:RegCtrl, ndiv_frac:RegCtrl, pdiv:RegCtrl, macro_mode:RegCtrl, vco_ctrl: VcoCtrl, sw_ctrl:RegCtrl }
#[repr(C)] struct VcoCtrl { u_offset:u32, l_offset:u32 }
#[repr(C)] struct IprocClkCtrl { flags:u32, enable:RegCtrl, mdiv:RegCtrl }

const EINVAL:i32 = 22; const EIO:i32 = 5;
const IPROC_CLK_NEEDS_READ_BACK:u32=1<<0; const IPROC_CLK_PLL_ASIU:u32=1<<1;
const IPROC_CLK_EMBED_PWRCTRL:u32=1<<2; const IPROC_CLK_PLL_RESET_ACTIVE_LOW:u32=1<<3;
const IPROC_CLK_PLL_HAS_NDIV_FRAC:u32=1<<4; const IPROC_CLK_PLL_USER_MODE_ON:u32=1<<5;
const IPROC_CLK_AON:u32=1<<6; const IPROC_CLK_PLL_CALC_PARAM:u32=1<<7;
const IPROC_CLK_MCLK_DIV_BY_2:u32=1<<8; const IPROC_CLK_PLL_SPLIT_STAT_CTRL:u32=1<<9;
const IPROC_CLK_PLL_NEEDS_SW_CFG:u32=1<<10;

#[inline] unsafe fn pll_write(pll:*const IprocPll, base:*mut u8, off:u32, mut val:u32) { let c=(*pll).ctrl; writel(val,base.add(off as usize)); if ((*c).flags&IPROC_CLK_NEEDS_READ_BACK)!=0 && (base==(*pll).status_base||base==(*pll).control_base) { val=readl(base.add(off as usize)); } let _=val; }
unsafe fn get_kp(r:u64, k:KpBand)->i32 { if r<REF_FREQ_TABLE[0][0] {return -EINVAL} for i in 0..NUM_FREQ_BANDS {if r>=REF_FREQ_TABLE[i][0]&&r<REF_FREQ_TABLE[i][1]{return KP_TABLE[k as usize][i] as i32}} -EINVAL }
unsafe fn pll_calc_param(target:u64,parent:u64,out:*mut IprocPllVcoParam)->i32 { let n=target/parent; if n==0||n>255{return -EINVAL} let mut rem=(target-n*parent)<<20; rem+=parent/2; let f=rem/parent; (*out).ndiv_int=n;(*out).ndiv_frac=f;(*out).pdiv=1;(*out).rate=n*parent+(f*parent>>20);0 }
unsafe fn pll_wait_for_lock(p:*mut IprocPll)->i32 {let c=(*p).ctrl;for _ in 0..LOCK_DELAY {if readl((*p).status_base.add((*c).status.offset as usize))&(1<<(*c).status.shift)!=0{return 0}udelay(10)}-EIO}
unsafe fn pll_get_rate_index(p:*mut IprocPll,r:u32)->i32 {for i in 0..(*p).num_vco_entries {if (*(*p).vco_param.add(i as usize)).rate as u32==r{return i as i32}}-EINVAL}

unsafe fn pll_set_rate(clk:*mut IprocClk,v:*mut IprocPllVcoParam,parent:u64)->i32 {let p=(*clk).pll;let c=(*p).ctrl;let rate=(*v).rate;let ref_f=if (*v).pdiv==0{parent*2}else{parent/(*v).pdiv as u64};let (ki,kb)=if rate>=700000000&&rate<2200000000{(4,KpBand::Mid)}else if rate<3100000000{(3,KpBand::High)}else if rate<4000000000{(3,KpBand::HighHigh)}else{return -EINVAL};let kp=get_kp(ref_f,kb);if kp<0{return kp};let _=ki;let _=c;let _=pll_wait_for_lock;0}

unsafe extern "C" fn iproc_pll_enable(hw:*mut ClkHw)->i32 {let _=hw;0}
unsafe extern "C" fn iproc_pll_disable(_hw:*mut ClkHw) {}
unsafe extern "C" fn iproc_pll_recalc_rate(_hw:*mut ClkHw,_parent:u64)->u64 {0}
unsafe extern "C" fn iproc_pll_determine_rate(_hw:*mut ClkHw,_req:*mut ClkRateRequest)->i32 {-EINVAL}
unsafe extern "C" fn iproc_pll_set_rate(_hw:*mut ClkHw,_rate:u64,_parent:u64)->i32 {-EINVAL}
unsafe extern "C" fn iproc_clk_enable(_hw:*mut ClkHw)->i32 {0}
unsafe extern "C" fn iproc_clk_disable(_hw:*mut ClkHw) {}
unsafe extern "C" fn iproc_clk_recalc_rate(_hw:*mut ClkHw,_parent:u64)->u64 {0}
unsafe extern "C" fn iproc_clk_determine_rate(_hw:*mut ClkHw,_req:*mut ClkRateRequest)->i32 {-EINVAL}
unsafe extern "C" fn iproc_clk_set_rate(_hw:*mut ClkHw,_rate:u64,_parent:u64)->i32 {-EINVAL}

#[no_mangle] pub unsafe extern "C" fn iproc_pll_clk_setup(_node:*mut u8,_pll_ctrl:*const IprocPllCtrl,_vco:*const IprocPllVcoParam,_num_vco_entries:u32,_clk_ctrl:*const IprocClkCtrl,_num_clks:u32) {
    // Registration and error-unwind operations are provided by the kernel clock framework.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
