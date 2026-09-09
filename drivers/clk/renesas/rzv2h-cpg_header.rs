/* SPDX-License-Identifier: GPL-2.0 */
/* Renesas RZ/V2H(P) Clock Pulse Generator */

// Translated from rzv2h-cpg.h. Linux types and clock constants are supplied by
// the surrounding translation unit.

#[repr(C)]
pub struct pll {
    pub offset: u32,
    pub has_clkn: u32,
    pub instance: u32,
    pub limits: *const rzv2h_pll_limits,
}

#[macro_export]
macro_rules! PLL_PACK_LIMITS { ($offset:expr, $has_clkn:expr, $instance:expr, $limits:expr) => { pll { offset: $offset, has_clkn: $has_clkn, instance: $instance, limits: $limits } }; }
#[macro_export]
macro_rules! PLL_PACK { ($offset:expr, $has_clkn:expr, $instance:expr) => { PLL_PACK_LIMITS!($offset, $has_clkn, $instance, core::ptr::null()) }; }
pub const PLLCA55: pll = PLL_PACK!(0x60, 1, 0);
pub const PLLGPU: pll = PLL_PACK!(0x120, 1, 0);

#[repr(C)]
pub struct ddiv { pub offset: u32, pub shift: u32, pub width: u32, pub monbit: u32, pub no_rmw: u32 }
#[allow(non_upper_case_globals)]
pub const CSDIV_NO_MON: u32 = 0x1f;
#[macro_export]
macro_rules! DDIV_PACK { ($offset:expr, $shift:expr, $width:expr, $monbit:expr) => { ddiv { offset: $offset, shift: $shift, width: $width, monbit: $monbit, no_rmw: 0 } }; }
#[macro_export]
macro_rules! DDIV_PACK_NO_RMW { ($offset:expr, $shift:expr, $width:expr, $monbit:expr) => { ddiv { offset: $offset, shift: $shift, width: $width, monbit: $monbit, no_rmw: 1 } }; }

#[repr(C)]
pub struct smuxed { pub offset: u32, pub shift: u32, pub width: u32 }
#[macro_export]
macro_rules! SMUX_PACK { ($offset:expr, $shift:expr, $width:expr) => { smuxed { offset: $offset, shift: $shift, width: $width } }; }

#[repr(C)]
pub struct fixed_mod_conf { pub mon_index: u8, pub mon_bit: u8 }
#[macro_export]
macro_rules! FIXED_MOD_CONF_PACK { ($index:expr, $bit:expr) => { fixed_mod_conf { mon_index: $index, mon_bit: $bit } }; }

pub const CPG_SSEL0: u32 = 0x300; pub const CPG_SSEL1: u32 = 0x304; pub const CPG_SSEL3: u32 = 0x30c;
pub const CPG_CDDIV0: u32 = 0x400; pub const CPG_CDDIV1: u32 = 0x404; pub const CPG_CDDIV2: u32 = 0x408; pub const CPG_CDDIV3: u32 = 0x40c; pub const CPG_CDDIV4: u32 = 0x410;
pub const CPG_CSDIV0: u32 = 0x500; pub const CPG_CSDIV1: u32 = 0x504;

macro_rules! ddiv_const { ($n:ident, $m:ident, $s:expr, $w:expr, $b:expr) => { pub const $n: ddiv = DDIV_PACK!($m, $s, $w, $b); }; }
ddiv_const!(CDDIV0_DIVCTL1, CPG_CDDIV0, 4, 3, 1); ddiv_const!(CDDIV0_DIVCTL2, CPG_CDDIV0, 8, 3, 2);
ddiv_const!(CDDIV1_DIVCTL0, CPG_CDDIV1, 0, 2, 4); ddiv_const!(CDDIV1_DIVCTL1, CPG_CDDIV1, 4, 2, 5); ddiv_const!(CDDIV1_DIVCTL2, CPG_CDDIV1, 8, 2, 6); ddiv_const!(CDDIV1_DIVCTL3, CPG_CDDIV1, 12, 2, 7);
ddiv_const!(CDDIV2_DIVCTL3, CPG_CDDIV2, 12, 3, 11); ddiv_const!(CDDIV3_DIVCTL1, CPG_CDDIV3, 4, 3, 13); ddiv_const!(CDDIV3_DIVCTL2, CPG_CDDIV3, 8, 3, 14); ddiv_const!(CDDIV3_DIVCTL3, CPG_CDDIV3, 12, 1, 15);
ddiv_const!(CDDIV4_DIVCTL0, CPG_CDDIV4, 0, 1, 16); ddiv_const!(CDDIV4_DIVCTL1, CPG_CDDIV4, 4, 1, 17); ddiv_const!(CDDIV4_DIVCTL2, CPG_CDDIV4, 8, 1, 18);
ddiv_const!(CSDIV0_DIVCTL0, CPG_CSDIV0, 0, 2, CSDIV_NO_MON); ddiv_const!(CSDIV0_DIVCTL1, CPG_CSDIV0, 4, 2, CSDIV_NO_MON); ddiv_const!(CSDIV0_DIVCTL2, CPG_CSDIV0, 8, 2, CSDIV_NO_MON);
pub const CSDIV0_DIVCTL3: ddiv = DDIV_PACK_NO_RMW!(CPG_CSDIV0, 12, 2, CSDIV_NO_MON);
ddiv_const!(CSDIV1_DIVCTL2, CPG_CSDIV1, 8, 4, CSDIV_NO_MON); ddiv_const!(CSDIV1_DIVCTL3, CPG_CSDIV1, 12, 4, CSDIV_NO_MON);
macro_rules! smux_const { ($n:ident, $m:ident, $s:expr) => { pub const $n: smuxed = SMUX_PACK!($m, $s, 1); }; }
smux_const!(SSEL0_SELCTL2, CPG_SSEL0, 8); smux_const!(SSEL0_SELCTL3, CPG_SSEL0, 12); smux_const!(SSEL1_SELCTL0, CPG_SSEL1, 0); smux_const!(SSEL1_SELCTL1, CPG_SSEL1, 4); smux_const!(SSEL1_SELCTL2, CPG_SSEL1, 8); smux_const!(SSEL1_SELCTL3, CPG_SSEL1, 12); smux_const!(SSEL3_SELCTL0, CPG_SSEL3, 0); smux_const!(SSEL3_SELCTL1, CPG_SSEL3, 4);

pub const BUS_MSTOP_IDX_MASK: u32 = 0xffff0000; pub const BUS_MSTOP_BITS_MASK: u32 = 0xffff; pub const BUS_MSTOP_NONE: u32 = 0xffff_ffff;
#[macro_export]
macro_rules! BUS_MSTOP { ($idx:expr, $mask:expr) => { (($idx as u32) << 16) | (($mask as u32) & 0xffff) }; }
pub const FIXED_MOD_CONF_XSPI: fixed_mod_conf = FIXED_MOD_CONF_PACK!(5, 1);

#[macro_export] macro_rules! DEF_MOD_BASE { ($name:expr,$m:expr,$p:expr,$c:expr,$n:expr,$oi:expr,$ob:expr,$mi:expr,$mb:expr,$x:expr) => { rzv2h_mod_clk { name:$name,mstop_data:$m,parent:$p,critical:$c,no_pm:$n,on_index:$oi,on_bit:$ob,mon_index:$mi,mon_bit:$mb,ext_clk_mux_index:$x } }; }
#[macro_export] macro_rules! DEF_MOD { ($($x:expr),*) => { DEF_MOD_BASE!($($x),*,false,false,-1) }; }
#[macro_export] macro_rules! DEF_MOD_CRITICAL { ($($x:expr),*) => { DEF_MOD_BASE!($($x),*,true,false,-1) }; }
#[macro_export] macro_rules! DEF_MOD_NO_PM { ($($x:expr),*) => { DEF_MOD_BASE!($($x),*,false,true,-1) }; }
#[macro_export] macro_rules! DEF_RST_BASE { ($ri:expr,$rb:expr,$mi:expr,$mb:expr) => { rzv2h_reset { reset_index:$ri,reset_bit:$rb,mon_index:$mi,mon_bit:$mb } }; }
#[macro_export] macro_rules! DEF_RST { ($($x:expr),*) => { DEF_RST_BASE!($($x),*) }; }

#[repr(C)]
pub union cpg_core_clk_cfg { pub conf: u32, pub ddiv: ddiv, pub pll: pll, pub smux: smuxed, pub fixed_mod: fixed_mod_conf }
#[repr(C)]
pub struct cpg_core_clk {
    pub name: *const i8, pub id: u32, pub parent: u32, pub div: u32, pub mult: u32, pub type_: u32,
    pub cfg: cpg_core_clk_cfg, pub dtable: *const clk_div_table, pub parent_names: *const *const i8,
    pub num_parents: u32, pub mux_flags: u8, pub flag: u32,
}

#[repr(u32)]
pub enum clk_types { CLK_TYPE_IN, CLK_TYPE_FF, CLK_TYPE_FF_MOD_STATUS, CLK_TYPE_PLL, CLK_TYPE_DDIV, CLK_TYPE_SMUX, CLK_TYPE_PLLDSI, CLK_TYPE_PLLDSI_DIV, CLK_TYPE_PLLDSI_SMUX }

#[macro_export] macro_rules! DEF_TYPE { ($name:expr,$id:expr,$ty:expr) => { cpg_core_clk { name:$name,id:$id,parent:0,div:0,mult:0,type_:$ty,cfg:cpg_core_clk_cfg{conf:0},dtable:core::ptr::null(),parent_names:core::ptr::null(),num_parents:0,mux_flags:0,flag:0 } }; }
#[macro_export] macro_rules! DEF_INPUT { ($name:expr,$id:expr) => { DEF_TYPE!($name,$id,clk_types::CLK_TYPE_IN) }; }
#[macro_export] macro_rules! DEF_BASE { ($name:expr,$id:expr,$ty:expr,$parent:expr) => {{ let mut c=DEF_TYPE!($name,$id,$ty); c.parent=$parent; c }}; }
#[macro_export] macro_rules! DEF_FIXED { ($name:expr,$id:expr,$parent:expr,$mult:expr,$div:expr) => {{ let mut c=DEF_BASE!($name,$id,clk_types::CLK_TYPE_FF,$parent); c.mult=$mult;c.div=$div;c }}; }
#[macro_export] macro_rules! DEF_PLL { ($name:expr,$id:expr,$parent:expr,$p:expr) => {{ let mut c=DEF_BASE!($name,$id,clk_types::CLK_TYPE_PLL,$parent); c.cfg=cpg_core_clk_cfg{pll:$p};c }}; }
#[macro_export] macro_rules! DEF_DDIV { ($name:expr,$id:expr,$parent:expr,$d:expr,$t:expr) => {{ let mut c=DEF_BASE!($name,$id,clk_types::CLK_TYPE_DDIV,$parent); c.cfg=cpg_core_clk_cfg{ddiv:$d};c.dtable=$t;c }}; }
#[macro_export] macro_rules! DEF_CSDIV { ($($x:expr),*) => { DEF_DDIV!($($x),*) }; }
#[macro_export] macro_rules! DEF_PLLDSI { ($name:expr,$id:expr,$parent:expr,$p:expr) => { DEF_PLL!($name,$id,$parent,$p) }; }

#[repr(C)]
pub struct rzv2h_mod_clk { pub name: *const i8, pub mstop_data: u32, pub parent: u16, pub critical: bool, pub no_pm: bool, pub on_index: u8, pub on_bit: u8, pub mon_index: i8, pub mon_bit: u8, pub ext_clk_mux_index: i8 }
#[repr(C)]
pub struct rzv2h_reset { pub reset_index: u8, pub reset_bit: u8, pub mon_index: u8, pub mon_bit: u8 }
#[repr(C)]
pub struct rzv2h_cpg_info { pub core_clks: *const cpg_core_clk, pub num_core_clks: u32, pub last_dt_core_clk: u32, pub num_total_core_clks: u32, pub mod_clks: *const rzv2h_mod_clk, pub num_mod_clks: u32, pub num_hw_mod_clks: u32, pub resets: *const rzv2h_reset, pub num_resets: u32, pub num_mstop_bits: u32 }

extern "C" {
    pub static r9a09g047_cpg_info: rzv2h_cpg_info;
    pub static r9a09g056_cpg_info: rzv2h_cpg_info;
    pub static r9a09g057_cpg_info: rzv2h_cpg_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
