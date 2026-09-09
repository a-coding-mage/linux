// SPDX-License-Identifier: GPL-2.0-only
/* PRCMU clock implementation for ux500 platform. */

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct clk_hw { pub init: *mut clk_init_data }
#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub flags: c_ulong,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
}
pub type c_ulong = usize;
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong }
#[repr(C)] pub struct clk_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> i32>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32>,
}

#[repr(C)] struct clk_prcmu { hw: clk_hw, cg_sel: u8, opp_requested: i32 }
#[repr(C)] struct clk_prcmu_clkout { hw: clk_hw, clkout_id: u8, source: u8, divider: u8 }

extern "C" {
    fn db8500_prcmu_request_clock(u8, bool) -> i32;
    fn prcmu_clock_rate(u8) -> c_ulong;
    fn prcmu_round_clock_rate(u8, c_ulong) -> c_ulong;
    fn prcmu_set_clock_rate(u8, c_ulong) -> i32;
    fn prcmu_qos_add_requirement(i32, *mut c_char, i32) -> i32;
    fn prcmu_qos_remove_requirement(i32, *mut c_char);
    fn db8500_prcmu_request_ape_opp_100_voltage(bool) -> i32;
    fn prcmu_config_clkout(u8, u8, u8) -> i32;
    fn clk_hw_get_name(*mut clk_hw) -> *const c_char;
    fn clk_hw_is_prepared(*mut clk_hw) -> bool;
    fn clk_hw_determine_rate_no_reparent(*mut clk_hw, *mut clk_rate_request) -> i32;
    fn clk_hw_register(*mut c_void, *mut clk_hw) -> i32;
    fn pr_err(fmt: *const c_char, ...);
    fn strcmp(*const c_char, *const c_char) -> i32;
    fn kzalloc(size: usize) -> *mut c_void;
    fn kfree(*mut c_void);
}

const PRCMU_QOS_APE_OPP: i32 = 0;
const CLK_GET_RATE_NOCACHE: c_ulong = 1;

unsafe fn clk_from_hw<T>(hw: *mut clk_hw, offset: usize) -> *mut T {
    (hw as *mut u8).sub(offset) as *mut T
}

unsafe extern "C" fn clk_prcmu_prepare(hw: *mut clk_hw) -> i32 {
    let clk = clk_from_hw::<clk_prcmu>(hw, 0); db8500_prcmu_request_clock((*clk).cg_sel, true)
}
unsafe extern "C" fn clk_prcmu_unprepare(hw: *mut clk_hw) {
    let clk = clk_from_hw::<clk_prcmu>(hw, 0);
    if db8500_prcmu_request_clock((*clk).cg_sel, false) != 0 { pr_err(b"clk_prcmu: failed to disable %s.\0".as_ptr() as _, clk_hw_get_name(hw)); }
}
unsafe extern "C" fn clk_prcmu_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong { prcmu_clock_rate((*clk_from_hw::<clk_prcmu>(hw, 0)).cg_sel) }
unsafe extern "C" fn clk_prcmu_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { (*req).rate = prcmu_round_clock_rate((*clk_from_hw::<clk_prcmu>(hw, 0)).cg_sel, (*req).rate); 0 }
unsafe extern "C" fn clk_prcmu_set_rate(hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong) -> i32 { prcmu_set_clock_rate((*clk_from_hw::<clk_prcmu>(hw, 0)).cg_sel, rate) }

unsafe extern "C" fn clk_prcmu_opp_prepare(hw: *mut clk_hw) -> i32 {
    let clk = clk_from_hw::<clk_prcmu>(hw, 0); let mut err;
    if (*clk).opp_requested == 0 { err = prcmu_qos_add_requirement(PRCMU_QOS_APE_OPP, clk_hw_get_name(hw) as *mut _, 100); if err != 0 { return err; } (*clk).opp_requested = 1; }
    err = db8500_prcmu_request_clock((*clk).cg_sel, true); if err != 0 { prcmu_qos_remove_requirement(PRCMU_QOS_APE_OPP, clk_hw_get_name(hw) as *mut _); (*clk).opp_requested = 0; } err
}
unsafe extern "C" fn clk_prcmu_opp_unprepare(hw: *mut clk_hw) { let clk=clk_from_hw::<clk_prcmu>(hw,0); if db8500_prcmu_request_clock((*clk).cg_sel,false)==0 && (*clk).opp_requested!=0 { prcmu_qos_remove_requirement(PRCMU_QOS_APE_OPP,clk_hw_get_name(hw) as *mut _); (*clk).opp_requested=0; } }
unsafe extern "C" fn clk_prcmu_opp_volt_prepare(hw: *mut clk_hw) -> i32 { let clk=clk_from_hw::<clk_prcmu>(hw,0); if (*clk).opp_requested==0 { let e=db8500_prcmu_request_ape_opp_100_voltage(true); if e!=0{return e;} (*clk).opp_requested=1;} let e=db8500_prcmu_request_clock((*clk).cg_sel,true); if e!=0 {db8500_prcmu_request_ape_opp_100_voltage(false);(*clk).opp_requested=0;} e }
unsafe extern "C" fn clk_prcmu_opp_volt_unprepare(hw:*mut clk_hw){let clk=clk_from_hw::<clk_prcmu>(hw,0);if db8500_prcmu_request_clock((*clk).cg_sel,false)==0&&(*clk).opp_requested!=0{db8500_prcmu_request_ape_opp_100_voltage(false);(*clk).opp_requested=0;}}

static CLK_PRCMU_SCALABLE_OPS: clk_ops = clk_ops{prepare:Some(clk_prcmu_prepare),unprepare:Some(clk_prcmu_unprepare),recalc_rate:Some(clk_prcmu_recalc_rate),determine_rate:Some(clk_prcmu_determine_rate),set_rate:Some(clk_prcmu_set_rate),get_parent:None,set_parent:None};
static CLK_PRCMU_GATE_OPS: clk_ops = clk_ops{prepare:Some(clk_prcmu_prepare),unprepare:Some(clk_prcmu_unprepare),recalc_rate:Some(clk_prcmu_recalc_rate),determine_rate:None,set_rate:None,get_parent:None,set_parent:None};
static CLK_PRCMU_SCALABLE_RATE_OPS: clk_ops = clk_ops{prepare:None,unprepare:None,recalc_rate:Some(clk_prcmu_recalc_rate),determine_rate:Some(clk_prcmu_determine_rate),set_rate:Some(clk_prcmu_set_rate),get_parent:None,set_parent:None};
static CLK_PRCMU_RATE_OPS: clk_ops = clk_ops{prepare:None,unprepare:None,recalc_rate:Some(clk_prcmu_recalc_rate),determine_rate:None,set_rate:None,get_parent:None,set_parent:None};
static CLK_PRCMU_OPP_GATE_OPS: clk_ops = clk_ops{prepare:Some(clk_prcmu_opp_prepare),unprepare:Some(clk_prcmu_opp_unprepare),recalc_rate:Some(clk_prcmu_recalc_rate),determine_rate:None,set_rate:None,get_parent:None,set_parent:None};
static CLK_PRCMU_OPP_VOLT_SCALABLE_OPS: clk_ops = clk_ops{prepare:Some(clk_prcmu_opp_volt_prepare),unprepare:Some(clk_prcmu_opp_volt_unprepare),recalc_rate:Some(clk_prcmu_recalc_rate),determine_rate:Some(clk_prcmu_determine_rate),set_rate:Some(clk_prcmu_set_rate),get_parent:None,set_parent:None};

unsafe fn clk_reg_prcmu(name:*const c_char,parent_name:*const c_char,cg_sel:u8,rate:c_ulong,flags:c_ulong,ops:*const clk_ops)->*mut clk_hw{if name.is_null(){return (-22isize) as *mut clk_hw;}let clk=kzalloc(core::mem::size_of::<clk_prcmu>()) as *mut clk_prcmu;if clk.is_null(){return (-12isize) as *mut clk_hw;}(*clk).cg_sel=cg_sel;(*clk).opp_requested=0;if rate!=0{prcmu_set_clock_rate(cg_sel,rate);}let init=kzalloc(core::mem::size_of::<clk_init_data>()) as *mut clk_init_data;(*init).name=name;(*init).ops=ops;(*init).flags=flags;(*init).parent_names=if parent_name.is_null(){core::ptr::null()}else{&parent_name};(*init).num_parents=if parent_name.is_null(){0}else{1};(*clk).hw.init=init;if clk_hw_register(core::ptr::null_mut(),&mut (*clk).hw)!=0{kfree(clk as *mut _);return (-12isize) as *mut clk_hw;}&mut (*clk).hw}

pub unsafe fn clk_reg_prcmu_scalable(n:*const c_char,p:*const c_char,c:u8,r:c_ulong,f:c_ulong)->*mut clk_hw{clk_reg_prcmu(n,p,c,r,f,&CLK_PRCMU_SCALABLE_OPS)}
pub unsafe fn clk_reg_prcmu_gate(n:*const c_char,p:*const c_char,c:u8,f:c_ulong)->*mut clk_hw{clk_reg_prcmu(n,p,c,0,f,&CLK_PRCMU_GATE_OPS)}
pub unsafe fn clk_reg_prcmu_scalable_rate(n:*const c_char,p:*const c_char,c:u8,r:c_ulong,f:c_ulong)->*mut clk_hw{clk_reg_prcmu(n,p,c,r,f,&CLK_PRCMU_SCALABLE_RATE_OPS)}
pub unsafe fn clk_reg_prcmu_rate(n:*const c_char,p:*const c_char,c:u8,f:c_ulong)->*mut clk_hw{clk_reg_prcmu(n,p,c,0,f,&CLK_PRCMU_RATE_OPS)}
pub unsafe fn clk_reg_prcmu_opp_gate(n:*const c_char,p:*const c_char,c:u8,f:c_ulong)->*mut clk_hw{clk_reg_prcmu(n,p,c,0,f,&CLK_PRCMU_OPP_GATE_OPS)}
pub unsafe fn clk_reg_prcmu_opp_volt_scalable(n:*const c_char,p:*const c_char,c:u8,r:c_ulong,f:c_ulong)->*mut clk_hw{clk_reg_prcmu(n,p,c,r,f,&CLK_PRCMU_OPP_VOLT_SCALABLE_OPS)}

unsafe extern "C" fn clk_prcmu_clkout_prepare(hw:*mut clk_hw)->i32{let c=clk_from_hw::<clk_prcmu_clkout>(hw,0);prcmu_config_clkout((*c).clkout_id,(*c).source,(*c).divider)}
unsafe extern "C" fn clk_prcmu_clkout_unprepare(hw:*mut clk_hw){let c=clk_from_hw::<clk_prcmu_clkout>(hw,0);prcmu_config_clkout((*c).clkout_id,(*c).source,0);}
unsafe extern "C" fn clk_prcmu_clkout_recalc_rate(hw:*mut clk_hw,parent:c_ulong)->c_ulong{parent/((*clk_from_hw::<clk_prcmu_clkout>(hw,0)).divider as c_ulong)}
unsafe extern "C" fn clk_prcmu_clkout_get_parent(hw:*mut clk_hw)->u8{(*clk_from_hw::<clk_prcmu_clkout>(hw,0)).source}
unsafe extern "C" fn clk_prcmu_clkout_set_parent(hw:*mut clk_hw,index:u8)->i32{let c=clk_from_hw::<clk_prcmu_clkout>(hw,0);(*c).source=index;if clk_hw_is_prepared(hw){clk_prcmu_clkout_prepare(hw)}else{0}}
static CLK_PRCMU_CLKOUT_OPS:clk_ops=clk_ops{prepare:Some(clk_prcmu_clkout_prepare),unprepare:Some(clk_prcmu_clkout_unprepare),recalc_rate:Some(clk_prcmu_clkout_recalc_rate),determine_rate:Some(clk_hw_determine_rate_no_reparent),set_rate:None,get_parent:Some(clk_prcmu_clkout_get_parent),set_parent:Some(clk_prcmu_clkout_set_parent)};

pub unsafe fn clk_reg_prcmu_clkout(name:*const c_char,parent_names:*const *const c_char,num_parents:i32,source:u8,divider:u8)->*mut clk_hw{if name.is_null(){return (-22isize)as*mut clk_hw;}let id=if strcmp(name,b"clkout1\0".as_ptr()as _)==0{0}else if strcmp(name,b"clkout2\0".as_ptr()as _)==0{1}else{return (-22isize)as*mut clk_hw;};let c=kzalloc(core::mem::size_of::<clk_prcmu_clkout>())as*mut clk_prcmu_clkout;if c.is_null(){return (-12isize)as*mut clk_hw;}(*c).clkout_id=id;(*c).source=source;(*c).divider=divider;let i=kzalloc(core::mem::size_of::<clk_init_data>())as*mut clk_init_data;(*i).name=name;(*i).ops=&CLK_PRCMU_CLKOUT_OPS;(*i).flags=CLK_GET_RATE_NOCACHE;(*i).parent_names=parent_names;(*i).num_parents=num_parents as u8;(*c).hw.init=i;if clk_hw_register(core::ptr::null_mut(),&mut(*c).hw)!=0{kfree(c as*mut _);return(-12isize)as*mut clk_hw;}&mut(*c).hw}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
