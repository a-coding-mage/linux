/* SPDX-License-Identifier: GPL-2.0 */
// Translated from clk.h. C header dependencies are supplied externally.

extern "C" {
    pub static mut imx_ccm_lock: spinlock_t;
    pub static mut mcore_booted: bool;
    pub fn imx_check_clocks(clks: *mut *mut clk, count: c_uint);
    pub fn imx_check_clk_hws(clks: *mut *mut clk_hw, count: c_uint);
    #[cfg(not(feature = "module"))]
    pub fn imx_register_uart_clocks();
    pub fn imx_mmdc_mask_handshake(ccm_base: *mut c_void, chn: c_uint);
    pub fn imx_unregister_hw_clocks(hws: *mut *mut clk_hw, count: c_uint);
    pub fn imx_cscmr1_fixup(val: *mut u32);
}

#[cfg(feature = "module")]
#[inline]
pub unsafe fn imx_register_uart_clocks() {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum imx_pllv1_type { IMX_PLLV1_IMX1, IMX_PLLV1_IMX21, IMX_PLLV1_IMX25, IMX_PLLV1_IMX27, IMX_PLLV1_IMX31, IMX_PLLV1_IMX35 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum imx_sscg_pll_type { SCCG_PLL1, SCCG_PLL2 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum imx_pll14xx_type { PLL_1416X, PLL_1443X }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum imx_pllv4_type { IMX_PLLV4_IMX7ULP, IMX_PLLV4_IMX8ULP, IMX_PLLV4_IMX8ULP_1GHZ }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum imx_pfdv2_type { IMX_PFDV2_IMX7ULP, IMX_PFDV2_IMX8ULP }

#[repr(C)] pub struct imx_pll14xx_rate_table { pub rate: c_uint, pub pdiv: c_uint, pub mdiv: c_uint, pub sdiv: c_uint, pub kdiv: c_uint }
#[repr(C)] pub struct imx_pll14xx_clk { pub type_: imx_pll14xx_type, pub rate_table: *const imx_pll14xx_rate_table, pub rate_count: c_int, pub flags: c_int }
extern "C" { pub static mut imx_1416x_pll: imx_pll14xx_clk; pub static mut imx_1443x_pll: imx_pll14xx_clk; pub static mut imx_1443x_dram_pll: imx_pll14xx_clk; }
pub const CLK_FRACN_GPPLL_INTEGER: c_uint = 1 << 0;
pub const CLK_FRACN_GPPLL_FRACN: c_uint = 1 << 1;
#[repr(C)] pub struct imx_fracn_gppll_rate_table { pub rate: c_uint, pub mfi: c_uint, pub mfn: c_uint, pub mfd: c_uint, pub rdiv: c_uint, pub odiv: c_uint }
#[repr(C)] pub struct imx_fracn_gppll_clk { pub rate_table: *const imx_fracn_gppll_rate_table, pub rate_count: c_int, pub flags: c_int }
extern "C" { pub fn imx_clk_fracn_gppll(name:*const c_char,parent_name:*const c_char,base:*mut c_void,pll_clk:*const imx_fracn_gppll_clk)->*mut clk_hw; pub fn imx_clk_fracn_gppll_integer(name:*const c_char,parent_name:*const c_char,base:*mut c_void,pll_clk:*const imx_fracn_gppll_clk)->*mut clk_hw; pub static mut imx_fracn_gppll: imx_fracn_gppll_clk; pub static mut imx_fracn_gppll_integer: imx_fracn_gppll_clk; }

#[macro_export] macro_rules! imx_clk_cpu { ($($x:expr),*) => { to_clk(imx_clk_hw_cpu($($x),*)) }; }
#[macro_export] macro_rules! clk_register_gate2 { ($($x:expr),*) => { to_clk(clk_hw_register_gate2($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_pllv3 { ($($x:expr),*) => { to_clk(imx_clk_hw_pllv3($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_pfd { ($($x:expr),*) => { to_clk(imx_clk_hw_pfd($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_gate_exclusive { ($($x:expr),*) => { to_clk(imx_clk_hw_gate_exclusive($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_fixed { ($($x:expr),*) => { to_clk(imx_clk_hw_fixed($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_fixed_factor { ($($x:expr),*) => { to_clk(imx_clk_hw_fixed_factor($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_divider { ($($x:expr),*) => { to_clk(imx_clk_hw_divider($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_divider_flags { ($($x:expr),*) => { to_clk(imx_clk_hw_divider_flags($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_gate { ($($x:expr),*) => { to_clk(imx_clk_hw_gate($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_gate_dis { ($($x:expr),*) => { to_clk(imx_clk_hw_gate_dis($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_gate2 { ($($x:expr),*) => { to_clk(imx_clk_hw_gate2($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_mux { ($($x:expr),*) => { to_clk(imx_clk_hw_mux($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_mux_flags { ($($x:expr),*) => { to_clk(imx_clk_hw_mux_flags($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_mux2_flags { ($($x:expr),*) => { to_clk(imx_clk_hw_mux2_flags($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_pllv1 { ($($x:expr),*) => { to_clk(imx_clk_hw_pllv1($($x),*)) }; }
#[macro_export] macro_rules! imx_clk_pllv2 { ($($x:expr),*) => { to_clk(imx_clk_hw_pllv2($($x),*)) }; }

extern "C" {
    pub fn imx_dev_clk_hw_pll14xx(dev:*mut device,name:*const c_char,parent_name:*const c_char,base:*mut c_void,pll_clk:*const imx_pll14xx_clk)->*mut clk_hw;
    pub fn imx_clk_hw_pllv1(t:imx_pllv1_type,name:*const c_char,parent:*const c_char,base:*mut c_void)->*mut clk_hw;
    pub fn imx_clk_hw_pllv2(name:*const c_char,parent:*const c_char,base:*mut c_void)->*mut clk_hw;
    pub fn imx_clk_hw_frac_pll(name:*const c_char,parent_name:*const c_char,base:*mut c_void)->*mut clk_hw;
    pub fn imx_clk_hw_sscg_pll(name:*const c_char,parent_names:*const *const c_char,num_parents:u8,parent:u8,bypass1:u8,bypass2:u8,base:*mut c_void,flags:c_ulong)->*mut clk_hw;
}

#[repr(C)] #[derive(Copy,Clone)] pub enum imx_pllv3_type { IMX_PLLV3_GENERIC, IMX_PLLV3_SYS, IMX_PLLV3_USB, IMX_PLLV3_USB_VF610, IMX_PLLV3_AV, IMX_PLLV3_ENET, IMX_PLLV3_ENET_IMX7, IMX_PLLV3_SYS_VF610, IMX_PLLV3_DDR_IMX7, IMX_PLLV3_AV_IMX7 }
#[macro_export] macro_rules! PLL_1416X_RATE { ($r:expr,$m:expr,$p:expr,$s:expr) => { imx_pll14xx_rate_table { rate:$r, mdiv:$m, pdiv:$p, sdiv:$s, kdiv:0 } }; }
#[macro_export] macro_rules! PLL_1443X_RATE { ($r:expr,$m:expr,$p:expr,$s:expr,$k:expr) => { imx_pll14xx_rate_table { rate:$r, mdiv:$m, pdiv:$p, sdiv:$s, kdiv:$k } }; }

extern "C" {
    pub fn imx_clk_hw_pllv3(t:imx_pllv3_type,name:*const c_char,parent_name:*const c_char,base:*mut c_void,div_mask:u32)->*mut clk_hw;
    pub fn imx_clk_hw_pllv4(t:imx_pllv4_type,name:*const c_char,parent_name:*const c_char,base:*mut c_void)->*mut clk_hw;
    pub fn clk_hw_register_gate2(dev:*mut device,name:*const c_char,parent_name:*const c_char,flags:c_ulong,reg:*mut c_void,bit_idx:u8,cgr_val:u8,cgr_mask:u8,clk_gate_flags:u8,lock:*mut spinlock_t,share_count:*mut c_uint)->*mut clk_hw;
    pub fn imx_obtain_fixed_clock(name:*const c_char,rate:c_ulong)->*mut clk;
    pub fn imx_obtain_fixed_clock_hw(name:*const c_char,rate:c_ulong)->*mut clk_hw;
    pub fn imx_obtain_fixed_of_clock(np:*mut device_node,name:*const c_char,rate:c_ulong)->*mut clk_hw;
    pub fn imx_get_clk_hw_by_name(np:*mut device_node,name:*const c_char)->*mut clk_hw;
    pub fn imx_clk_hw_gate_exclusive(name:*const c_char,parent:*const c_char,reg:*mut c_void,shift:u8,exclusive_mask:u32)->*mut clk_hw;
    pub fn imx_clk_hw_pfd(name:*const c_char,parent_name:*const c_char,reg:*mut c_void,idx:u8)->*mut clk_hw;
    pub fn imx_clk_hw_pfdv2(t:imx_pfdv2_type,name:*const c_char,parent_name:*const c_char,reg:*mut c_void,idx:u8)->*mut clk_hw;
    pub fn imx_clk_hw_busy_divider(name:*const c_char,parent_name:*const c_char,reg:*mut c_void,shift:u8,width:u8,busy_reg:*mut c_void,busy_shift:u8)->*mut clk_hw;
    pub fn imx_clk_hw_busy_mux(name:*const c_char,reg:*mut c_void,shift:u8,width:u8,busy_reg:*mut c_void,busy_shift:u8,parent_names:*const *const c_char,num_parents:c_int)->*mut clk_hw;
    pub fn imx7ulp_clk_hw_composite(name:*const c_char,parent_names:*const *const c_char,num_parents:c_int,mux_present:bool,rate_present:bool,gate_present:bool,reg:*mut c_void)->*mut clk_hw;
    pub fn imx8ulp_clk_hw_composite(name:*const c_char,parent_names:*const *const c_char,num_parents:c_int,mux_present:bool,rate_present:bool,gate_present:bool,reg:*mut c_void,has_swrst:bool)->*mut clk_hw;
    pub fn imx_clk_hw_fixup_divider(name:*const c_char,parent:*const c_char,reg:*mut c_void,shift:u8,width:u8,fixup:Option<unsafe extern "C" fn(*mut u32)>)->*mut clk_hw;
    pub fn imx_clk_hw_fixup_mux(name:*const c_char,reg:*mut c_void,shift:u8,width:u8,parents:*const *const c_char,num_parents:c_int,fixup:Option<unsafe extern "C" fn(*mut u32)>)->*mut clk_hw;
}

#[inline] pub unsafe fn to_clk(hw:*mut clk_hw)->*mut clk { if hw.is_null() { hw as *mut clk } else { (*hw).clk } }
#[inline] pub unsafe fn imx_clk_hw_fixed(name:*const c_char,rate:c_int)->*mut clk_hw { clk_hw_register_fixed_rate(core::ptr::null_mut(),name,core::ptr::null(),0,rate) }
#[inline] pub unsafe fn imx_clk_hw_fixed_factor(name:*const c_char,parent:*const c_char,mult:c_uint,div:c_uint)->*mut clk_hw { clk_hw_register_fixed_factor(core::ptr::null_mut(),name,parent,CLK_SET_RATE_PARENT,mult,div) }
#[inline] pub unsafe fn __imx_clk_hw_divider(name:*const c_char,parent:*const c_char,reg:*mut c_void,shift:u8,width:u8,flags:c_ulong)->*mut clk_hw { clk_hw_register_divider(core::ptr::null_mut(),name,parent,flags,reg,shift,width,0,&raw mut imx_ccm_lock) }
#[inline] pub unsafe fn __imx_clk_hw_gate(name:*const c_char,parent:*const c_char,reg:*mut c_void,shift:u8,flags:c_ulong,gate_flags:c_ulong)->*mut clk_hw { clk_hw_register_gate(core::ptr::null_mut(),name,parent,flags|CLK_SET_RATE_PARENT,reg,shift,gate_flags,&raw mut imx_ccm_lock) }
#[inline] pub unsafe fn __imx_clk_hw_gate2(name:*const c_char,parent:*const c_char,reg:*mut c_void,shift:u8,cgr_val:u8,flags:c_ulong,share_count:*mut c_uint)->*mut clk_hw { clk_hw_register_gate2(core::ptr::null_mut(),name,parent,flags|CLK_SET_RATE_PARENT,reg,shift,cgr_val,3,0,&raw mut imx_ccm_lock,share_count) }

extern "C" {
    pub fn imx_clk_hw_cpu(name:*const c_char,parent_name:*const c_char,div:*mut clk,mux:*mut clk,pll:*mut clk,step:*mut clk)->*mut clk_hw;
    pub fn __imx8m_clk_hw_composite(name:*const c_char,parent_names:*const *const c_char,num_parents:c_int,reg:*mut c_void,composite_flags:u32,flags:c_ulong)->*mut clk_hw;
    pub fn imx93_clk_composite_flags(name:*const c_char,parent_names:*const *const c_char,num_parents:c_int,reg:*mut c_void,domain_id:u32,flags:c_ulong)->*mut clk_hw;
    pub fn imx93_clk_gate(dev:*mut device,name:*const c_char,parent_name:*const c_char,flags:c_ulong,reg:*mut c_void,bit_idx:u32,val:u32,mask:u32,domain_id:u32,share_count:*mut c_uint)->*mut clk_hw;
    pub fn imx_clk_hw_divider_gate(name:*const c_char,parent_name:*const c_char,flags:c_ulong,reg:*mut c_void,shift:u8,width:u8,clk_divider_flags:u8,table:*const clk_div_table,lock:*mut spinlock_t)->*mut clk_hw;
    pub fn imx_gpr_mux(name:*const c_char,compatible:*const c_char,reg:u32,parent_names:*const *const c_char,num_parents:u8,mux_table:*const u32,mask:u32)->*mut clk_hw;
    pub fn imx_audio_pll_debug_init(hws:*mut *mut clk_hw,num_plls:c_uint);
}

pub const IMX_COMPOSITE_CORE:c_uint=1<<0; pub const IMX_COMPOSITE_BUS:c_uint=1<<1; pub const IMX_COMPOSITE_FW_MANAGED:c_uint=1<<2;
pub const IMX_COMPOSITE_CLK_FLAGS_DEFAULT:c_ulong=CLK_SET_RATE_NO_REPARENT|CLK_OPS_PARENT_ENABLE;
pub const IMX_COMPOSITE_CLK_FLAGS_CRITICAL:c_ulong=IMX_COMPOSITE_CLK_FLAGS_DEFAULT|CLK_IS_CRITICAL;
pub const IMX_COMPOSITE_CLK_FLAGS_GET_RATE_NO_CACHE:c_ulong=IMX_COMPOSITE_CLK_FLAGS_DEFAULT|CLK_GET_RATE_NOCACHE;
pub const IMX_COMPOSITE_CLK_FLAGS_CRITICAL_GET_RATE_NO_CACHE:c_ulong=IMX_COMPOSITE_CLK_FLAGS_GET_RATE_NO_CACHE|CLK_IS_CRITICAL;

#[macro_export] macro_rules! imx_clk_hw_gate { ($n:expr,$p:expr,$r:expr,$s:expr) => { __imx_clk_hw_gate($n,$p,$r,$s,0,0) }; }
#[macro_export] macro_rules! imx_clk_hw_gate_flags { ($n:expr,$p:expr,$r:expr,$s:expr,$f:expr) => { __imx_clk_hw_gate($n,$p,$r,$s,$f,0) }; }
#[macro_export] macro_rules! imx_clk_hw_gate_dis { ($n:expr,$p:expr,$r:expr,$s:expr) => { __imx_clk_hw_gate($n,$p,$r,$s,0,CLK_GATE_SET_TO_DISABLE) }; }
#[macro_export] macro_rules! imx_clk_hw_gate_dis_flags { ($n:expr,$p:expr,$r:expr,$s:expr,$f:expr) => { __imx_clk_hw_gate($n,$p,$r,$s,$f,CLK_GATE_SET_TO_DISABLE) }; }
#[macro_export] macro_rules! imx_clk_hw_gate2 { ($n:expr,$p:expr,$r:expr,$s:expr) => { __imx_clk_hw_gate2($n,$p,$r,$s,3,0,core::ptr::null_mut()) }; }
#[macro_export] macro_rules! imx_clk_hw_gate2_flags { ($n:expr,$p:expr,$r:expr,$s:expr,$f:expr) => { __imx_clk_hw_gate2($n,$p,$r,$s,3,$f,core::ptr::null_mut()) }; }
#[macro_export] macro_rules! imx_clk_hw_gate2_shared { ($n:expr,$p:expr,$r:expr,$s:expr,$c:expr) => { __imx_clk_hw_gate2($n,$p,$r,$s,3,0,$c) }; }
#[macro_export] macro_rules! imx_clk_hw_gate2_shared2 { ($n:expr,$p:expr,$r:expr,$s:expr,$c:expr) => { __imx_clk_hw_gate2($n,$p,$r,$s,3,CLK_OPS_PARENT_ENABLE,$c) }; }
#[macro_export] macro_rules! imx_clk_hw_gate3 { ($($x:expr),*) => { imx_clk_hw_gate3_flags!($($x),*,0) }; }
#[macro_export] macro_rules! imx_clk_hw_gate3_flags { ($n:expr,$p:expr,$r:expr,$s:expr,$f:expr) => { __imx_clk_hw_gate($n,$p,$r,$s,$f|CLK_OPS_PARENT_ENABLE,0) }; }
#[macro_export] macro_rules! imx_clk_hw_gate4 { ($($x:expr),*) => { imx_clk_hw_gate4_flags!($($x),*,0) }; }
#[macro_export] macro_rules! imx_clk_hw_gate4_flags { ($n:expr,$p:expr,$r:expr,$s:expr,$f:expr) => { imx_clk_hw_gate2_flags!($n,$p,$r,$s,$f|CLK_OPS_PARENT_ENABLE) }; }
#[macro_export] macro_rules! imx_clk_hw_divider { ($n:expr,$p:expr,$r:expr,$s:expr,$w:expr) => { __imx_clk_hw_divider($n,$p,$r,$s,$w,CLK_SET_RATE_PARENT) }; }
#[macro_export] macro_rules! imx_clk_hw_divider_flags { ($n:expr,$p:expr,$r:expr,$s:expr,$w:expr,$f:expr) => { __imx_clk_hw_divider($n,$p,$r,$s,$w,$f) }; }
#[macro_export] macro_rules! imx_clk_hw_pll14xx { ($n:expr,$p:expr,$b:expr,$c:expr) => { imx_dev_clk_hw_pll14xx(core::ptr::null_mut(),$n,$p,$b,$c) }; }
#[macro_export] macro_rules! imx8m_clk_hw_composite { ($n:expr,$p:expr,$r:expr) => { __imx8m_clk_hw_composite($n,$p,core::mem::size_of_val($p) as c_int,$r,0,IMX_COMPOSITE_CLK_FLAGS_DEFAULT) }; }
#[macro_export] macro_rules! imx8m_clk_hw_composite_flags { ($n:expr,$p:expr,$r:expr,$f:expr) => { __imx8m_clk_hw_composite($n,$p,core::mem::size_of_val($p) as c_int,$r,0,IMX_COMPOSITE_CLK_FLAGS_DEFAULT|$f) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
