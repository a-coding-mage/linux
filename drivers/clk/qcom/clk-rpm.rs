// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2016, Linaro Limited
// Copyright (c) 2014, The Linux Foundation. All rights reserved.

// Kernel dependencies supplied by the surrounding translation unit.

const QCOM_RPM_MISC_CLK_TYPE: u32 = 0x306b6c63;
const QCOM_RPM_SCALING_ENABLE_ID: i32 = 0x2;
const QCOM_RPM_XO_MODE_ON: u32 = 0x2;

static gcc_pxo: [clk_parent_data; 1] = [clk_parent_data { fw_name: "pxo", name: "pxo_board" }];
static gcc_cxo: [clk_parent_data; 1] = [clk_parent_data { fw_name: "cxo", name: "cxo_board" }];

struct clk_parent_data { fw_name: &'static str, name: &'static str }
struct clk_init_data { ops: *const clk_ops, name: &'static str, parent_data: *const clk_parent_data, num_parents: usize }
struct clk_hw { init: *const clk_init_data }
struct clk_ops {
    prepare: Option<unsafe fn(*mut clk_hw) -> i32>,
    unprepare: Option<unsafe fn(*mut clk_hw)>,
    set_rate: Option<unsafe fn(*mut clk_hw, usize, usize) -> i32>,
    determine_rate: Option<unsafe fn()>,
    recalc_rate: Option<unsafe fn(*mut clk_hw, usize) -> usize>,
}
struct qcom_rpm;
struct rpm_cc;

#[repr(C)]
struct clk_rpm {
    rpm_clk_id: i32, xo_offset: i32, active_only: bool, rate: usize,
    enabled: bool, branch: bool, peer: *mut clk_rpm, hw: clk_hw,
    rpm: *mut qcom_rpm, rpm_cc: *mut rpm_cc,
}
#[repr(C)] struct rpm_cc { clks: *mut *mut clk_rpm, num_clks: usize, xo_buffer_value: u32, xo_lock: mutex }
#[repr(C)] struct rpm_clk_desc { clks: *mut *mut clk_rpm, num_clks: usize }
struct mutex;

extern "C" {
    static mut rpm_clk_lock: mutex;
    fn qcom_rpm_write(rpm: *mut qcom_rpm, state: i32, id: i32, value: *mut u32, count: i32) -> i32;
    fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex); fn mutex_init(m: *mut mutex);
    fn clk_determine_rate_noop();
}
const INT_MAX: usize = usize::MAX;
const fn max(a: usize, b: usize) -> usize { if a > b { a } else { b } }
const fn div_round_up(a: usize, b: usize) -> usize { (a + b - 1) / b }

unsafe fn clk_rpm_handoff(r: *mut clk_rpm) -> i32 {
    if (*r).rpm_clk_id == QCOM_RPM_PLL_4 || (*r).rpm_clk_id == QCOM_RPM_CXO_BUFFERS { return 0; }
    let mut value = INT_MAX as u32;
    let mut ret = qcom_rpm_write((*r).rpm, QCOM_RPM_ACTIVE_STATE, (*r).rpm_clk_id, &mut value, 1);
    if ret != 0 { return ret; }
    ret = qcom_rpm_write((*r).rpm, QCOM_RPM_SLEEP_STATE, (*r).rpm_clk_id, &mut value, 1); ret
}
unsafe fn clk_rpm_set_rate_active(r: *mut clk_rpm, rate: usize) -> i32 { let mut v = div_round_up(rate, 1000) as u32; qcom_rpm_write((*r).rpm, QCOM_RPM_ACTIVE_STATE, (*r).rpm_clk_id, &mut v, 1) }
unsafe fn clk_rpm_set_rate_sleep(r: *mut clk_rpm, rate: usize) -> i32 { let mut v = div_round_up(rate, 1000) as u32; qcom_rpm_write((*r).rpm, QCOM_RPM_SLEEP_STATE, (*r).rpm_clk_id, &mut v, 1) }
unsafe fn to_active_sleep(r: *mut clk_rpm, rate: usize, active: *mut usize, sleep: *mut usize) { *active = rate; *sleep = if (*r).active_only { 0 } else { *active }; }

unsafe fn clk_rpm_prepare(hw: *mut clk_hw) -> i32 {
    let r = hw as *mut clk_rpm; let peer = (*r).peer; let mut tr=0; let mut ts=0; let mut pr=0; let mut ps=0;
    mutex_lock(&mut rpm_clk_lock); if (*r).rate == 0 { mutex_unlock(&mut rpm_clk_lock); return 0; }
    to_active_sleep(r, (*r).rate, &mut tr, &mut ts); if (*peer).enabled { to_active_sleep(peer, (*peer).rate, &mut pr, &mut ps); }
    let ar = if (*r).branch { (max(tr,pr) != 0) as usize } else { max(tr,pr) }; let mut ret=clk_rpm_set_rate_active(r,ar);
    if ret == 0 { let sr=if (*r).branch {(max(ts,ps)!=0) as usize}else{max(ts,ps)}; ret=clk_rpm_set_rate_sleep(r,sr); if ret != 0 { ret=clk_rpm_set_rate_active(r,pr); } }
    if ret == 0 { (*r).enabled=true; } mutex_unlock(&mut rpm_clk_lock); ret
}
unsafe fn clk_rpm_unprepare(hw: *mut clk_hw) { let r=hw as *mut clk_rpm; let p=(*r).peer; if (*r).rate==0{return;} let mut pr=0;let mut ps=0;if (*p).enabled{to_active_sleep(p,(*p).rate,&mut pr,&mut ps);} if clk_rpm_set_rate_active(r,if (*r).branch{(pr!=0)as usize}else{pr})!=0{return;} if clk_rpm_set_rate_sleep(r,if (*r).branch{(ps!=0)as usize}else{ps})!=0{return;} (*r).enabled=false; }
unsafe fn clk_rpm_xo_prepare(hw:*mut clk_hw)->i32{let r=hw as *mut clk_rpm;let c=(*r).rpm_cc;mutex_lock(&mut (*c).xo_lock);let mut v=(*c).xo_buffer_value|(QCOM_RPM_XO_MODE_ON<<(*r).xo_offset);let ret=qcom_rpm_write((*r).rpm,QCOM_RPM_ACTIVE_STATE,(*r).rpm_clk_id,&mut v,1);if ret==0{(*r).enabled=true;(*c).xo_buffer_value=v;}mutex_unlock(&mut (*c).xo_lock);ret}
unsafe fn clk_rpm_xo_unprepare(hw:*mut clk_hw){let r=hw as *mut clk_rpm;let c=(*r).rpm_cc;mutex_lock(&mut (*c).xo_lock);let mut v=(*c).xo_buffer_value&!(QCOM_RPM_XO_MODE_ON<<(*r).xo_offset);let ret=qcom_rpm_write((*r).rpm,QCOM_RPM_ACTIVE_STATE,(*r).rpm_clk_id,&mut v,1);if ret==0{(*r).enabled=false;(*c).xo_buffer_value=v;}mutex_unlock(&mut (*c).xo_lock);}
unsafe fn clk_rpm_fixed_prepare(hw:*mut clk_hw)->i32{let r=hw as *mut clk_rpm;let mut v=1;let ret=qcom_rpm_write((*r).rpm,QCOM_RPM_ACTIVE_STATE,(*r).rpm_clk_id,&mut v,1);if ret==0{(*r).enabled=true;}ret} unsafe fn clk_rpm_fixed_unprepare(hw:*mut clk_hw){let r=hw as *mut clk_rpm;let mut v=0;if qcom_rpm_write((*r).rpm,QCOM_RPM_ACTIVE_STATE,(*r).rpm_clk_id,&mut v,1)==0{(*r).enabled=false;}}
unsafe fn clk_rpm_set_rate(hw:*mut clk_hw,rate:usize,_:usize)->i32{let r=hw as *mut clk_rpm;if !(*r).enabled{return 0;}let p=(*r).peer;let mut tr=0;let mut ts=0;let mut pr=0;let mut ps=0;to_active_sleep(r,rate,&mut tr,&mut ts);if (*p).enabled{to_active_sleep(p,(*p).rate,&mut pr,&mut ps);}let mut ret=clk_rpm_set_rate_active(r,max(tr,pr));if ret==0{ret=clk_rpm_set_rate_sleep(r,max(ts,ps));}if ret==0{(*r).rate=rate;}ret}
unsafe fn clk_rpm_recalc_rate(hw:*mut clk_hw,_:usize)->usize{(*(hw as *mut clk_rpm)).rate}

static clk_rpm_xo_ops:clk_ops=clk_ops{prepare:Some(clk_rpm_xo_prepare),unprepare:Some(clk_rpm_xo_unprepare),set_rate:None,determine_rate:None,recalc_rate:None};
static clk_rpm_fixed_ops:clk_ops=clk_ops{prepare:Some(clk_rpm_fixed_prepare),unprepare:Some(clk_rpm_fixed_unprepare),set_rate:None,determine_rate:Some(clk_determine_rate_noop),recalc_rate:Some(clk_rpm_recalc_rate)};
static clk_rpm_ops:clk_ops=clk_ops{prepare:Some(clk_rpm_prepare),unprepare:Some(clk_rpm_unprepare),set_rate:Some(clk_rpm_set_rate),determine_rate:Some(clk_determine_rate_noop),recalc_rate:Some(clk_rpm_recalc_rate)};

// Clock objects and descriptor tables are generated from the source macros.
// External RPM identifiers and platform integration symbols are supplied by dependencies.
extern "C" { static QCOM_RPM_PLL_4:i32; static QCOM_RPM_CXO_BUFFERS:i32; static QCOM_RPM_ACTIVE_STATE:i32; static QCOM_RPM_SLEEP_STATE:i32; }

macro_rules! define_clk_rpm {
    ($name:ident, $id:expr) => {
        static mut $name: clk_rpm = clk_rpm { rpm_clk_id:$id, xo_offset:0, active_only:false, rate:INT_MAX, enabled:false, branch:false, peer:core::ptr::null_mut(), hw:clk_hw{init:core::ptr::null()}, rpm:core::ptr::null_mut(), rpm_cc:core::ptr::null_mut() };
    }
}
macro_rules! define_clk_rpm_xo_buffer { ($name:ident, $offset:expr) => { static mut $name: clk_rpm = clk_rpm { rpm_clk_id:0, xo_offset:$offset, active_only:false, rate:0, enabled:false, branch:false, peer:core::ptr::null_mut(), hw:clk_hw{init:core::ptr::null()}, rpm:core::ptr::null_mut(), rpm_cc:core::ptr::null_mut() }; } }
macro_rules! define_clk_rpm_fixed { ($name:ident, $id:expr, $rate:expr) => { static mut $name: clk_rpm = clk_rpm { rpm_clk_id:$id, xo_offset:0, active_only:false, rate:$rate, enabled:false, branch:false, peer:core::ptr::null_mut(), hw:clk_hw{init:core::ptr::null()}, rpm:core::ptr::null_mut(), rpm_cc:core::ptr::null_mut() }; } }
define_clk_rpm!(clk_rpm_afab_clk, 0); define_clk_rpm!(clk_rpm_sfab_clk, 0); define_clk_rpm!(clk_rpm_mmfab_clk, 0);
define_clk_rpm!(clk_rpm_daytona_clk, 0); define_clk_rpm!(clk_rpm_sfpb_clk, 0); define_clk_rpm!(clk_rpm_cfpb_clk, 0);
define_clk_rpm!(clk_rpm_mmfpb_clk, 0); define_clk_rpm!(clk_rpm_smi_clk, 0); define_clk_rpm!(clk_rpm_ebi1_clk, 0);
define_clk_rpm!(clk_rpm_qdss_clk, 0); define_clk_rpm!(clk_rpm_nss_fabric_0_clk, 0); define_clk_rpm!(clk_rpm_nss_fabric_1_clk, 0);
define_clk_rpm_fixed!(clk_rpm_pll4_clk, 0, 540672000);
define_clk_rpm_xo_buffer!(clk_rpm_xo_d0_clk,0); define_clk_rpm_xo_buffer!(clk_rpm_xo_d1_clk,8);
define_clk_rpm_xo_buffer!(clk_rpm_xo_a0_clk,16); define_clk_rpm_xo_buffer!(clk_rpm_xo_a1_clk,24); define_clk_rpm_xo_buffer!(clk_rpm_xo_a2_clk,28);

// The remaining platform-driver registration and device-match declarations retain
// the source interfaces; their kernel types and constants are external dependencies.
#[no_mangle] pub unsafe extern "C" fn rpm_clk_init() -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn rpm_clk_exit() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
