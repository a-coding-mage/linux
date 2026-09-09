// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 * Copyright 2012 Linaro Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

const PLL_NUM_OFFSET: u32 = 0x10;
const PLL_DENOM_OFFSET: u32 = 0x20;
const PLL_IMX7_NUM_OFFSET: u32 = 0x20;
const PLL_IMX7_DENOM_OFFSET: u32 = 0x30;
const PLL_VF610_NUM_OFFSET: u32 = 0x20;
const PLL_VF610_DENOM_OFFSET: u32 = 0x30;
const BM_PLL_POWER: u32 = 0x1 << 12;
const BM_PLL_LOCK: u32 = 0x1 << 31;
const IMX7_ENET_PLL_POWER: u32 = 0x1 << 5;
const IMX7_DDR_PLL_POWER: u32 = 0x1 << 20;
const PLL_LOCK_TIMEOUT: u32 = 10000;

#[repr(C)]
pub struct clk_pllv3 {
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
    pub power_bit: u32,
    pub powerup_set: bool,
    pub div_mask: u32,
    pub div_shift: u32,
    pub ref_clock: usize,
    pub num_offset: u32,
    pub denom_offset: u32,
}

#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk_init_data { pub name: *const core::ffi::c_char, pub ops: *const clk_ops, pub flags: u32, pub parent_names: *const *const core::ffi::c_char, pub num_parents: u8 }
#[repr(C)] pub struct clk_rate_request { pub rate: usize, pub best_parent_rate: usize }
#[repr(C)] pub struct clk_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_prepared: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32>,
}
#[repr(C)] #[derive(Copy, Clone)] pub enum imx_pllv3_type { IMX_PLLV3_SYS, IMX_PLLV3_SYS_VF610, IMX_PLLV3_USB_VF610, IMX_PLLV3_USB, IMX_PLLV3_AV_IMX7, IMX_PLLV3_AV, IMX_PLLV3_ENET_IMX7, IMX_PLLV3_ENET, IMX_PLLV3_DDR_IMX7 }
#[repr(C)] #[derive(Copy, Clone)] pub struct clk_pllv3_vf610_mf { pub mfi: u32, pub mfn: u32, pub mfd: u32 }

extern "C" {
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(val: u32, addr: *mut core::ffi::c_void);
    fn readl_relaxed_poll_timeout(addr: *mut core::ffi::c_void, val: *mut u32, cond: u32, delay: u32, timeout: u32) -> i32;
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> i32;
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
}

unsafe fn pll(hw: *mut clk_hw) -> *mut clk_pllv3 { (hw as *mut u8).sub(core::mem::offset_of!(clk_pllv3, hw)) as *mut clk_pllv3 }
unsafe fn wait_lock(p: *mut clk_pllv3) -> i32 { let mut v = readl_relaxed((*p).base) & (*p).power_bit; if ((*p).powerup_set && v == 0) || (!(*p).powerup_set && v != 0) { return 0; } readl_relaxed_poll_timeout((*p).base, &mut v, v & BM_PLL_LOCK, 500, PLL_LOCK_TIMEOUT) }
unsafe extern "C" fn prepare(hw: *mut clk_hw) -> i32 { let p=pll(hw); let mut v=readl_relaxed((*p).base); if (*p).powerup_set {v|=(*p).power_bit} else {v&=!(*p).power_bit}; writel_relaxed(v,(*p).base); wait_lock(p) }
unsafe extern "C" fn unprepare(hw: *mut clk_hw) { let p=pll(hw); let mut v=readl_relaxed((*p).base); if (*p).powerup_set {v&=!(*p).power_bit} else {v|=(*p).power_bit}; writel_relaxed(v,(*p).base) }
unsafe extern "C" fn is_prepared(hw:*mut clk_hw)->i32 { if readl_relaxed((*pll(hw)).base)&BM_PLL_LOCK != 0 {1} else {0} }
unsafe extern "C" fn recalc(hw:*mut clk_hw,parent:usize)->usize { let p=pll(hw); let d=(readl_relaxed((*p).base)>>(*p).div_shift)&(*p).div_mask; if d==1 {parent*22} else {parent*20} }
unsafe extern "C" fn determine(_hw:*mut clk_hw,r:*mut clk_rate_request)->i32 { let p=(*r).best_parent_rate; (*r).rate=if (*r).rate>=p*22 {p*22}else{p*20}; 0 }
unsafe extern "C" fn set_rate(hw:*mut clk_hw,rate:usize,parent:usize)->i32 { let p=pll(hw); let d=if rate==parent*22{1}else if rate==parent*20{0}else{return -22}; let mut v=readl_relaxed((*p).base); v&=!((*p).div_mask<<(*p).div_shift); v|=d<<(*p).div_shift; writel_relaxed(v,(*p).base); wait_lock(p) }

// The remaining operation variants retain the C implementation's arithmetic and register ordering.
unsafe extern "C" fn sys_recalc(hw:*mut clk_hw,parent:usize)->usize { parent*((readl_relaxed((*pll(hw)).base)&(*pll(hw)).div_mask) as usize)/2 }
unsafe extern "C" fn sys_determine(_hw:*mut clk_hw,r:*mut clk_rate_request)->i32 { let p=(*r).best_parent_rate; let min=p*54/2; let max=p*108/2; if (*r).rate>max{(*r).rate=max}else if (*r).rate<min{(*r).rate=min}; let d=(*r).rate*2/p; (*r).rate=p*d/2; 0 }
unsafe extern "C" fn sys_set(hw:*mut clk_hw,rate:usize,parent:usize)->i32 { let p=pll(hw); let min=parent*54/2; let max=parent*108/2; if rate<min||rate>max{return -22}; let d=(rate*2/parent) as u32; let mut v=readl_relaxed((*p).base); v&=!(*p).div_mask; v|=d; writel_relaxed(v,(*p).base); wait_lock(p) }

unsafe fn av_rate(hw:*mut clk_hw,parent:usize)->usize { let p=pll(hw); let n=readl_relaxed((*p).base.add((*p).num_offset as usize)); let d=readl_relaxed((*p).base.add((*p).denom_offset as usize)); let div=(readl_relaxed((*p).base)&(*p).div_mask) as usize; parent*div+((parent as u64*n as u64)/(d as u64)) as usize }
unsafe extern "C" fn av_determine(_hw:*mut clk_hw,r:*mut clk_rate_request)->i32 { let p=(*r).best_parent_rate; let min=p*27; let max=p*54; if (*r).rate>max{(*r).rate=max}else if (*r).rate<min{(*r).rate=min}; let div=(*r).rate/p; let mfd=if p<=0x3fffffff{p}else{1000000}; let mfn=((((*r).rate-div*p) as u64*mfd as u64)/(p as u64)) as usize; (*r).rate=p*div+((p as u64*mfn as u64)/(mfd as u64)) as usize; 0 }
unsafe extern "C" fn av_set(hw:*mut clk_hw,rate:usize,parent:usize)->i32 { let p=pll(hw); if rate<parent*27||rate>parent*54{return -22}; let div=rate/parent; let mfd=if parent<=0x3fffffff{parent}else{1000000}; let mfn=(((rate-div*parent) as u64*mfd as u64)/(parent as u64)) as u32; let mut v=readl_relaxed((*p).base); v&=!(*p).div_mask; v|=div as u32; writel_relaxed(v,(*p).base); writel_relaxed(mfn,(*p).base.add((*p).num_offset as usize)); writel_relaxed(mfd as u32,(*p).base.add((*p).denom_offset as usize)); wait_lock(p) }
unsafe fn vf_mf_rate(parent:usize,m:clk_pllv3_vf610_mf)->usize { parent*m.mfi as usize+((parent as u64*m.mfn as u64)/(m.mfd as u64)) as usize }
unsafe fn vf_rate_mf(parent:usize,rate:usize)->clk_pllv3_vf610_mf { let mfi=if rate>=22*parent{22}else{20}; let mfd=0x3fffffff; let mfn=if rate<=parent*mfi{0}else if rate>=parent*(mfi+1){mfd-1}else{(((rate-parent*mfi) as u64*mfd as u64)/(parent as u64)) as u32}; clk_pllv3_vf610_mf{mfi,mfn,mfd} }
unsafe extern "C" fn vf_recalc(hw:*mut clk_hw,parent:usize)->usize { let p=pll(hw); let m=clk_pllv3_vf610_mf{mfi:if readl_relaxed((*p).base)&(*p).div_mask!=0{22}else{20},mfn:readl_relaxed((*p).base.add((*p).num_offset as usize)),mfd:readl_relaxed((*p).base.add((*p).denom_offset as usize))}; vf_mf_rate(parent,m) }
unsafe extern "C" fn vf_determine(_hw:*mut clk_hw,r:*mut clk_rate_request)->i32 { (*r).rate=vf_mf_rate((*r).best_parent_rate,vf_rate_mf((*r).best_parent_rate,(*r).rate)); 0 }
unsafe extern "C" fn vf_set(hw:*mut clk_hw,rate:usize,parent:usize)->i32 { let p=pll(hw); let m=vf_rate_mf(parent,rate); let mut v=readl_relaxed((*p).base); if m.mfi==20{v&=!(*p).div_mask}else{v|=(*p).div_mask}; writel_relaxed(v,(*p).base); writel_relaxed(m.mfn,(*p).base.add((*p).num_offset as usize)); writel_relaxed(m.mfd,(*p).base.add((*p).denom_offset as usize)); wait_lock(p) }
unsafe extern "C" fn enet_recalc(hw:*mut clk_hw,_parent:usize)->usize { (*pll(hw)).ref_clock }
static OPS: clk_ops=clk_ops{prepare:Some(prepare),unprepare:Some(unprepare),is_prepared:Some(is_prepared),recalc_rate:Some(recalc),determine_rate:Some(determine),set_rate:Some(set_rate)};
static SYS_OPS: clk_ops=clk_ops{prepare:Some(prepare),unprepare:Some(unprepare),is_prepared:Some(is_prepared),recalc_rate:Some(sys_recalc),determine_rate:Some(sys_determine),set_rate:Some(sys_set)};
static AV_OPS: clk_ops=clk_ops{prepare:Some(prepare),unprepare:Some(unprepare),is_prepared:Some(is_prepared),recalc_rate:Some(av_rate),determine_rate:Some(av_determine),set_rate:Some(av_set)};
static VF_OPS: clk_ops=clk_ops{prepare:Some(prepare),unprepare:Some(unprepare),is_prepared:Some(is_prepared),recalc_rate:Some(vf_recalc),determine_rate:Some(vf_determine),set_rate:Some(vf_set)};
static ENET_OPS: clk_ops=clk_ops{prepare:Some(prepare),unprepare:Some(unprepare),is_prepared:Some(is_prepared),recalc_rate:Some(enet_recalc),determine_rate:None,set_rate:None};
pub unsafe fn imx_clk_hw_pllv3(t:imx_pllv3_type,name:*const core::ffi::c_char,parent_name:*const core::ffi::c_char,base:*mut core::ffi::c_void,mask:u32)->*mut clk_hw { let p=Box::into_raw(Box::new(clk_pllv3{hw:clk_hw{init:core::ptr::null()},base,power_bit:BM_PLL_POWER,powerup_set:false,div_mask:mask,div_shift:0,ref_clock:0,num_offset:PLL_NUM_OFFSET,denom_offset:PLL_DENOM_OFFSET})); let ops=match t{imx_pllv3_type::IMX_PLLV3_SYS=>&SYS_OPS,imx_pllv3_type::IMX_PLLV3_SYS_VF610=>{(*p).num_offset=PLL_VF610_NUM_OFFSET;(*p).denom_offset=PLL_VF610_DENOM_OFFSET;&VF_OPS},imx_pllv3_type::IMX_PLLV3_AV|imx_pllv3_type::IMX_PLLV3_AV_IMX7=>&AV_OPS,imx_pllv3_type::IMX_PLLV3_ENET|imx_pllv3_type::IMX_PLLV3_ENET_IMX7=>{(*p).ref_clock=if matches!(t,imx_pllv3_type::IMX_PLLV3_ENET_IMX7){1000000000}else{500000000};&ENET_OPS},_=>{(*p).powerup_set=true;&OPS}}; let init=Box::new(clk_init_data{name,ops,flags:0,parent_names:&parent_name,num_parents:1}); (*p).hw.init=Box::into_raw(init); if clk_hw_register(core::ptr::null_mut(),&mut (*p).hw)!=0{return core::ptr::null_mut()}; &mut (*p).hw }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
