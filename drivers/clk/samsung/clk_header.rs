/* SPDX-License-Identifier: GPL-2.0-only */
/* Common Clock Framework support for all Samsung platforms. */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct samsung_clk_provider {
    pub reg_base: *mut c_void,
    pub dev: *mut device,
    pub sysreg: *mut regmap,
    pub lock: spinlock_t,
    pub auto_clock_gate: bool,
    pub gate_dbg_offset: u32,
    pub option_offset: u32,
    pub drcg_offset: u32,
    pub memclk_offset: u32,
    // Must be the last entry due to variable-length `hws` array.
    pub clk_data: clk_hw_onecell_data,
}

#[repr(C)]
pub struct samsung_clock_alias {
    pub id: u32,
    pub dev_name: *const core::ffi::c_char,
    pub alias: *const core::ffi::c_char,
}

#[macro_export]
macro_rules! ALIAS {
    ($id:expr, $dname:expr, $a:expr) => {
        samsung_clock_alias { id: $id, dev_name: $dname, alias: $a }
    };
}

pub const MHZ: u32 = 1000 * 1000;

#[repr(C)]
pub struct samsung_fixed_rate_clock {
    pub id: u32,
    pub name: *mut core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
    pub flags: usize,
    pub fixed_rate: usize,
}

#[macro_export]
macro_rules! FRATE {
    ($id:expr, $cname:expr, $pname:expr, $f:expr, $frate:expr) => {
        samsung_fixed_rate_clock { id: $id, name: $cname, parent_name: $pname, flags: $f, fixed_rate: $frate }
    };
}

#[repr(C)]
pub struct samsung_fixed_factor_clock {
    pub id: u32,
    pub name: *mut core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
    pub mult: usize,
    pub div: usize,
    pub flags: usize,
}

#[macro_export]
macro_rules! FFACTOR {
    ($id:expr, $cname:expr, $pname:expr, $m:expr, $d:expr, $f:expr) => {
        samsung_fixed_factor_clock { id: $id, name: $cname, parent_name: $pname, mult: $m, div: $d, flags: $f }
    };
}

#[repr(C)]
pub struct samsung_mux_clock {
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: u8,
    pub flags: usize,
    pub offset: usize,
    pub shift: u8,
    pub width: u8,
    pub mux_flags: u8,
}

#[macro_export]
macro_rules! __MUX {
    ($id:expr, $cname:expr, $pnames:expr, $o:expr, $s:expr, $w:expr, $f:expr, $mf:expr) => {
        samsung_mux_clock { id: $id, name: $cname, parent_names: $pnames.as_ptr(), num_parents: $pnames.len() as u8, flags: $f, offset: $o, shift: $s, width: $w, mux_flags: $mf }
    };
}
#[macro_export]
macro_rules! MUX { ($($x:tt)*) => { __MUX!($($x)*, CLK_SET_RATE_NO_REPARENT, 0) }; }
#[macro_export]
macro_rules! MUX_F { ($id:expr,$c:expr,$p:expr,$o:expr,$s:expr,$w:expr,$f:expr,$mf:expr) => { __MUX!($id,$c,$p,$o,$s,$w,($f)|CLK_SET_RATE_NO_REPARENT,$mf) }; }
#[macro_export]
macro_rules! nMUX { ($($x:tt)*) => { __MUX!($($x)*, 0, 0) }; }
#[macro_export]
macro_rules! nMUX_F { ($id:expr,$c:expr,$p:expr,$o:expr,$s:expr,$w:expr,$f:expr,$mf:expr) => { __MUX!($id,$c,$p,$o,$s,$w,$f,$mf) }; }

#[repr(C)]
pub struct samsung_div_clock {
    pub id: u32, pub name: *const core::ffi::c_char, pub parent_name: *const core::ffi::c_char,
    pub flags: usize, pub offset: usize, pub shift: u8, pub width: u8, pub div_flags: u8,
    pub table: *mut clk_div_table,
}
#[macro_export]
macro_rules! __DIV { ($id:expr,$c:expr,$p:expr,$o:expr,$s:expr,$w:expr,$f:expr,$df:expr,$t:expr) => { samsung_div_clock { id:$id,name:$c,parent_name:$p,flags:$f,offset:$o,shift:$s,width:$w,div_flags:$df,table:$t } }; }
#[macro_export] macro_rules! DIV { ($($x:tt)*) => { __DIV!($($x)*,0,0,core::ptr::null_mut()) }; }
#[macro_export] macro_rules! DIV_F { ($id:expr,$c:expr,$p:expr,$o:expr,$s:expr,$w:expr,$f:expr,$df:expr) => { __DIV!($id,$c,$p,$o,$s,$w,$f,$df,core::ptr::null_mut()) }; }
#[macro_export] macro_rules! DIV_T { ($id:expr,$c:expr,$p:expr,$o:expr,$s:expr,$w:expr,$t:expr) => { __DIV!($id,$c,$p,$o,$s,$w,0,0,$t) }; }

#[repr(C)]
pub struct samsung_gate_clock { pub id:u32, pub name:*const core::ffi::c_char, pub parent_name:*const core::ffi::c_char, pub flags:usize, pub offset:usize, pub bit_idx:u8, pub gate_flags:u8 }
#[macro_export] macro_rules! __GATE { ($id:expr,$c:expr,$p:expr,$o:expr,$b:expr,$f:expr,$gf:expr) => { samsung_gate_clock{id:$id,name:$c,parent_name:$p,flags:$f,offset:$o,bit_idx:$b,gate_flags:$gf} }; }
#[macro_export] macro_rules! GATE { ($($x:tt)*) => { __GATE!($($x)*) }; }
// PNAME(x) declares a static const parent-name array in C.

#[repr(C)] pub struct samsung_clk_reg_dump { pub offset:u32, pub value:u32 }
#[repr(C)] pub struct samsung_pll_clock { pub id:u32, pub name:*const core::ffi::c_char, pub parent_name:*const core::ffi::c_char, pub flags:usize, pub con_offset:i32, pub lock_offset:i32, pub type_:samsung_pll_type, pub rate_table:*const samsung_pll_rate_table }
#[macro_export] macro_rules! __PLL { ($t:expr,$id:expr,$n:expr,$p:expr,$f:expr,$l:expr,$c:expr,$r:expr) => { samsung_pll_clock{id:$id,type_:$t,name:$n,parent_name:$p,flags:$f,con_offset:$c,lock_offset:$l,rate_table:$r} }; }
#[macro_export] macro_rules! PLL { ($t:expr,$id:expr,$n:expr,$p:expr,$l:expr,$c:expr,$r:expr) => { __PLL!($t,$id,$n,$p,CLK_GET_RATE_NOCACHE,$l,$c,$r) }; }

#[repr(C)] pub struct samsung_cpu_clock { pub id:u32,pub name:*const core::ffi::c_char,pub parent_id:u32,pub alt_parent_id:u32,pub flags:usize,pub offset:i32,pub reg_layout:exynos_cpuclk_layout,pub cfg:*const exynos_cpuclk_cfg_data }
#[macro_export] macro_rules! CPU_CLK { ($id:expr,$n:expr,$p:expr,$a:expr,$f:expr,$o:expr,$l:expr,$c:expr) => { samsung_cpu_clock{id:$id,name:$n,parent_id:$p,alt_parent_id:$a,flags:$f,offset:$o,reg_layout:$l,cfg:$c} }; }

#[repr(C)] pub struct samsung_clock_reg_cache { pub node:list_head,pub reg_base:*mut c_void,pub sysreg:*mut regmap,pub rsuspend:*const samsung_clk_reg_dump,pub rsuspend_num:u32,pub rd_num:u32,pub rdump:[samsung_clk_reg_dump;0] }

#[repr(C)] pub struct samsung_cmu_info {
    pub pll_clks:*const samsung_pll_clock,pub nr_pll_clks:u32,pub mux_clks:*const samsung_mux_clock,pub nr_mux_clks:u32,pub div_clks:*const samsung_div_clock,pub nr_div_clks:u32,pub gate_clks:*const samsung_gate_clock,pub nr_gate_clks:u32,pub fixed_clks:*const samsung_fixed_rate_clock,pub nr_fixed_clks:u32,pub fixed_factor_clks:*const samsung_fixed_factor_clock,pub nr_fixed_factor_clks:u32,pub nr_clk_ids:u32,pub cpu_clks:*const samsung_cpu_clock,pub nr_cpu_clks:u32,
    pub clk_regs:*const usize,pub nr_clk_regs:u32,pub suspend_regs:*const samsung_clk_reg_dump,pub nr_suspend_regs:u32,pub clk_name:*const core::ffi::c_char,pub sysreg_clk_regs:*const usize,pub nr_sysreg_clk_regs:u32,
    pub manual_plls:bool,pub auto_clock_gate:bool,pub gate_dbg_offset:u32,pub option_offset:u32,pub drcg_offset:u32,pub memclk_offset:u32,
}

// The following declarations are external kernel functions. Their definitions are supplied elsewhere.
extern "C" {
    pub fn samsung_clk_init(dev:*mut device, base:*mut c_void, nr_clks:usize) -> *mut samsung_clk_provider;
    pub fn samsung_clk_of_add_provider(np:*mut device_node, ctx:*mut samsung_clk_provider);
    pub fn samsung_clk_of_register_fixed_ext(ctx:*mut samsung_clk_provider, fixed_rate_clk:*mut samsung_fixed_rate_clock, nr_fixed_rate_clk:u32, clk_matches:*const of_device_id);
    pub fn samsung_clk_add_lookup(ctx:*mut samsung_clk_provider, clk_hw:*mut clk_hw, id:u32);
    pub fn samsung_clk_register_alias(ctx:*mut samsung_clk_provider,list:*const samsung_clock_alias,nr_clk:u32);
    pub fn samsung_clk_register_fixed_rate(ctx:*mut samsung_clk_provider,list:*const samsung_fixed_rate_clock,nr_clk:u32);
    pub fn samsung_clk_register_fixed_factor(ctx:*mut samsung_clk_provider,list:*const samsung_fixed_factor_clock,nr_clk:u32);
    pub fn samsung_clk_register_mux(ctx:*mut samsung_clk_provider,list:*const samsung_mux_clock,nr_clk:u32);
    pub fn samsung_clk_register_div(ctx:*mut samsung_clk_provider,list:*const samsung_div_clock,nr_clk:u32);
    pub fn samsung_clk_register_gate(ctx:*mut samsung_clk_provider,list:*const samsung_gate_clock,nr_clk:u32);
    pub fn samsung_clk_register_pll(ctx:*mut samsung_clk_provider,list:*const samsung_pll_clock,nr_clk:u32);
    pub fn samsung_clk_register_cpu(ctx:*mut samsung_clk_provider,list:*const samsung_cpu_clock,nr_clk:u32);
    pub fn samsung_cmu_register_clocks(ctx:*mut samsung_clk_provider,cmu:*const samsung_cmu_info,np:*mut device_node);
    pub fn samsung_cmu_register_one(np:*mut device_node,cmu:*const samsung_cmu_info)->*mut samsung_clk_provider;
    pub fn samsung_clk_extended_sleep_init(reg_base:*mut c_void,sysreg:*mut regmap,rdump:*const usize,nr_rdump:usize,rsuspend:*const samsung_clk_reg_dump,nr_rsuspend:usize);
    pub fn samsung_clk_save(base:*mut c_void,regmap:*mut regmap,rd:*mut samsung_clk_reg_dump,num_regs:u32);
    pub fn samsung_clk_restore(base:*mut c_void,regmap:*mut regmap,rd:*const samsung_clk_reg_dump,num_regs:u32);
    pub fn samsung_clk_alloc_reg_dump(rdump:*const usize,nr_rdump:usize)->*mut samsung_clk_reg_dump;
    pub fn samsung_en_dyn_root_clk_gating(np:*mut device_node,ctx:*mut samsung_clk_provider,cmu:*const samsung_cmu_info,cmu_has_pm:bool);
}

#[macro_export]
macro_rules! samsung_clk_sleep_init { ($base:expr,$sysreg:expr,$rdump:expr,$nr:expr) => { samsung_clk_extended_sleep_init($base,$sysreg,$rdump,$nr,core::ptr::null(),0) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
