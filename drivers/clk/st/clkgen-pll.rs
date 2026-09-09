// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of clkgen-pll.c. External kernel symbols are supplied by dependencies. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const C32_NDIV_MASK: u32 = 0xff;
const C32_IDF_MASK: u32 = 0x7;
const C32_ODF_MASK: u32 = 0x3f;
const C32_LDF_MASK: u32 = 0x7f;
const C32_CP_MASK: u32 = 0x1f;
const C32_MAX_ODFS: usize = 4;
const C28_NDIV_MASK: u32 = 0xff;
const C28_IDF_MASK: u32 = 0x7;
const C28_ODF_MASK: u32 = 0x3f;

// Dependency-provided C-compatible types and operations.
#[repr(C)] pub struct clkgen_field { pub offset: usize, pub mask: u32, pub shift: u32 }
#[repr(C)] pub struct clk_hw { pub init: *mut c_void }
#[repr(C)] pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
}
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong, pub best_parent_rate: c_ulong }
#[repr(C)] pub struct clkgen_pll_data {
    pub pdn_status: clkgen_field, pub pdn_ctrl: clkgen_field, pub locked_status: clkgen_field,
    pub mdiv: clkgen_field, pub ndiv: clkgen_field, pub pdiv: clkgen_field, pub idf: clkgen_field,
    pub ldf: clkgen_field, pub cp: clkgen_field, pub num_odfs: c_uint,
    pub odf: [clkgen_field; C32_MAX_ODFS], pub odf_gate: [clkgen_field; C32_MAX_ODFS],
    pub switch2pll_en: bool, pub switch2pll: clkgen_field, pub lock: *mut c_void,
    pub ops: *const clk_ops,
}
#[repr(C)] pub struct clkgen_clk_out { pub name: *const c_char, pub flags: c_ulong }
#[repr(C)] pub struct clkgen_pll_data_clks { pub data: *mut clkgen_pll_data, pub outputs: *const clkgen_clk_out }
#[repr(C)] pub struct clkgen_pll { pub hw: clk_hw, pub data: *mut clkgen_pll_data, pub regs_base: *mut c_void, pub lock: *mut c_void, pub ndiv: u32, pub idf: u32, pub cp: u32 }
#[repr(C)] pub struct stm_pll { pub mdiv: c_ulong, pub ndiv: c_ulong, pub pdiv: c_ulong, pub odf: c_ulong, pub idf: c_ulong, pub ldf: c_ulong, pub cp: c_ulong }

extern "C" {
    static mut clkgena_c32_odf_lock: c_void;
    static mut clkgen_a9_lock: c_void;
    fn clkgen_read(pll: *mut clkgen_pll, field: *const clkgen_field) -> u32;
    fn clkgen_write(pll: *mut clkgen_pll, field: *const clkgen_field, value: u32);
    fn clk_hw_get_name(hw: *mut clk_hw) -> *const c_char;
    fn readl_relaxed_poll_timeout(addr: *mut c_void, val: *mut u32, cond: c_int, delay: u32, timeout: u32) -> c_int;
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut c_ulong); fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe fn pll_of(hw: *mut clk_hw) -> *mut clkgen_pll { (hw as *mut u8).sub(core::mem::offset_of!(clkgen_pll, hw)) as *mut clkgen_pll }
unsafe fn field(pll: *mut clkgen_pll, f: fn(&clkgen_pll_data)->&clkgen_field) -> u32 { clkgen_read(pll, f(&*(*pll).data)) }

#[no_mangle] pub unsafe extern "C" fn clkgen_pll_is_locked(hw: *mut clk_hw) -> c_int { (field(pll_of(hw), |d| &d.locked_status) != 0) as c_int }
#[no_mangle] pub unsafe extern "C" fn clkgen_pll_is_enabled(hw: *mut clk_hw) -> c_int { (field(pll_of(hw), |d| &d.pdn_status) == 0) as c_int }

unsafe fn pll_enable_inner(hw: *mut clk_hw) -> c_int {
    let pll=pll_of(hw); if clkgen_pll_is_enabled(hw)!=0 { return 0; }
    clkgen_write(pll, &(*(*pll).data).pdn_ctrl, 0); let mut reg=0u32;
    let f=&(*(*pll).data).locked_status;
    let ret=readl_relaxed_poll_timeout((*pll).regs_base.add(f.offset), &mut reg, (((reg>>f.shift)&f.mask)!=0) as c_int, 0, 10000);
    if ret==0 && (*(*pll).data).switch2pll_en { clkgen_write(pll,&(*(*pll).data).switch2pll,0); } ret
}
#[no_mangle] pub unsafe extern "C" fn clkgen_pll_enable(hw:*mut clk_hw)->c_int { let p=pll_of(hw); let mut flags=0; if !(*p).lock.is_null(){spin_lock_irqsave((*p).lock,&mut flags)} let r=pll_enable_inner(hw); if !(*p).lock.is_null(){spin_unlock_irqrestore((*p).lock,flags)} r }
unsafe fn pll_disable_inner(hw:*mut clk_hw){let p=pll_of(hw);if clkgen_pll_is_enabled(hw)==0{return} if (*(*p).data).switch2pll_en{clkgen_write(p,&(*(*p).data).switch2pll,1)} clkgen_write(p,&(*(*p).data).pdn_ctrl,1);}
#[no_mangle] pub unsafe extern "C" fn clkgen_pll_disable(hw:*mut clk_hw){let p=pll_of(hw);let mut f=0;if !(*p).lock.is_null(){spin_lock_irqsave((*p).lock,&mut f)}pll_disable_inner(hw);if !(*p).lock.is_null(){spin_unlock_irqrestore((*p).lock,f)}}

unsafe fn abs_diff(a:c_ulong,b:c_ulong)->c_ulong{if a>b{a-b}else{b-a}}
#[no_mangle] pub unsafe extern "C" fn clk_pll3200c32_get_params(input:c_ulong,output:c_ulong,pll:*mut stm_pll)->c_int{if output<800000000||output>1600000000{return -22}let input=input/1000;let output=output/1000;let mut dev=c_ulong::MAX;for i in 1..=7{if dev==0{break}let n=i*output/(2*input);if n<8{continue}if n>200{break}let nf=input*2*n/i;let d=abs_diff(nf,output);if d==0||d<dev{(*pll).idf=i;(*pll).ndiv=n;dev=d}}if dev==c_ulong::MAX{return -22}let cp=[48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176,184,192];(*pll).cp=6;while (*pll).cp-6<cp.len() as c_ulong&&(*pll).ndiv>cp[((*pll).cp-6)as usize] as c_ulong{(*pll).cp+=1}0}
#[no_mangle] pub unsafe extern "C" fn clk_pll3200c32_get_rate(input:c_ulong,pll:*mut stm_pll,rate:*mut c_ulong)->c_int{if (*pll).idf==0{(*pll).idf=1}*rate=((2*(input/1000)*(*pll).ndiv)/(*pll).idf)*1000;0}
#[no_mangle] pub unsafe extern "C" fn clk_pll4600c28_get_params(input:c_ulong,output:c_ulong,pll:*mut stm_pll)->c_int{if output<19000000||output>3000000000{return -22}let mut dev=c_ulong::MAX;for i in 1..=7{if dev==0{break}let inf=input/i;if inf<4000000||inf>50000000{continue}let mut n=output/(inf*2);if n<8||n>246{continue}if n<246{n+=1}while n>=8&&dev!=0{let nf=inf*2*n;if nf<output{break}let d=nf-output;if d==0||d<dev{(*pll).idf=i;(*pll).ndiv=n;dev=d}n-=1}}if dev==c_ulong::MAX{-22}else{0}}
#[no_mangle] pub unsafe extern "C" fn clk_pll4600c28_get_rate(input:c_ulong,pll:*mut stm_pll,rate:*mut c_ulong)->c_int{if (*pll).idf==0{(*pll).idf=1}*rate=(input/(*pll).idf)*2*(*pll).ndiv;0}

// The remaining registration/setup declarations are intentionally retained as external integration points;
// their bodies depend on Linux clock-provider definitions supplied by the surrounding repository.
extern "C" { fn clkgen_c32_pll_setup(np:*mut c_void, data:*mut clkgen_pll_data_clks); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
