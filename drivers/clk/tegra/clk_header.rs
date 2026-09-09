/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of clk.h. External kernel types and symbols are supplied by dependencies. */

pub const CLK_OUT_ENB_L: u32 = 0x010;
pub const CLK_OUT_ENB_H: u32 = 0x014;
pub const CLK_OUT_ENB_U: u32 = 0x018;
pub const CLK_OUT_ENB_V: u32 = 0x360;
pub const CLK_OUT_ENB_W: u32 = 0x364;
pub const CLK_OUT_ENB_X: u32 = 0x280;
pub const CLK_OUT_ENB_Y: u32 = 0x298;
pub const CLK_ENB_PLLP_OUT_CPU: u32 = 1 << 31;
pub const CLK_OUT_ENB_SET_L: u32 = 0x320; pub const CLK_OUT_ENB_CLR_L: u32 = 0x324;
pub const CLK_OUT_ENB_SET_H: u32 = 0x328; pub const CLK_OUT_ENB_CLR_H: u32 = 0x32c;
pub const CLK_OUT_ENB_SET_U: u32 = 0x330; pub const CLK_OUT_ENB_CLR_U: u32 = 0x334;
pub const CLK_OUT_ENB_SET_V: u32 = 0x440; pub const CLK_OUT_ENB_CLR_V: u32 = 0x444;
pub const CLK_OUT_ENB_SET_W: u32 = 0x448; pub const CLK_OUT_ENB_CLR_W: u32 = 0x44c;
pub const CLK_OUT_ENB_SET_X: u32 = 0x284; pub const CLK_OUT_ENB_CLR_X: u32 = 0x288;
pub const CLK_OUT_ENB_SET_Y: u32 = 0x29c; pub const CLK_OUT_ENB_CLR_Y: u32 = 0x2a0;
pub const RST_DEVICES_L: u32 = 0x004; pub const RST_DEVICES_H: u32 = 0x008;
pub const RST_DEVICES_U: u32 = 0x00c; pub const RST_DEVICES_V: u32 = 0x358;
pub const RST_DEVICES_W: u32 = 0x35c; pub const RST_DEVICES_X: u32 = 0x28c;
pub const RST_DEVICES_Y: u32 = 0x2a4; pub const RST_DEVICES_SET_L: u32 = 0x300;
pub const RST_DEVICES_CLR_L: u32 = 0x304; pub const RST_DEVICES_SET_H: u32 = 0x308;
pub const RST_DEVICES_CLR_H: u32 = 0x30c; pub const RST_DEVICES_SET_U: u32 = 0x310;
pub const RST_DEVICES_CLR_U: u32 = 0x314; pub const RST_DEVICES_SET_V: u32 = 0x430;
pub const RST_DEVICES_CLR_V: u32 = 0x434; pub const RST_DEVICES_SET_W: u32 = 0x438;
pub const RST_DEVICES_CLR_W: u32 = 0x43c; pub const RST_DEVICES_SET_X: u32 = 0x290;
pub const RST_DEVICES_CLR_X: u32 = 0x294; pub const RST_DEVICES_SET_Y: u32 = 0x2a8;
pub const RST_DEVICES_CLR_Y: u32 = 0x2ac;
pub const TEGRA210_CLK_ENB_VLD_MSK_L: u32 = 0xdcd7dff9;
pub const TEGRA210_CLK_ENB_VLD_MSK_H: u32 = 0x87d1f3e7;
pub const TEGRA210_CLK_ENB_VLD_MSK_U: u32 = 0xf3fed3fa;
pub const TEGRA210_CLK_ENB_VLD_MSK_V: u32 = 0xffc18cfb;
pub const TEGRA210_CLK_ENB_VLD_MSK_W: u32 = 0x793fb7ff;
pub const TEGRA210_CLK_ENB_VLD_MSK_X: u32 = 0x3fe66fff;
pub const TEGRA210_CLK_ENB_VLD_MSK_Y: u32 = 0xfc1fc7ff;

#[repr(C)] pub struct tegra_clk_sync_source { pub hw: clk_hw, pub rate: c_ulong, pub max_rate: c_ulong }
#[repr(C)] pub struct tegra_clk_frac_div { pub hw: clk_hw, pub reg: *mut c_void, pub flags: u8, pub shift: u8, pub width: u8, pub frac_width: u8, pub lock: *mut spinlock_t }
#[repr(C)] pub struct tegra_clk_pll_freq_table { pub input_rate: c_ulong, pub output_rate: c_ulong, pub n: u32, pub m: u32, pub p: u8, pub cpcon: u8, pub sdm_data: u16 }
#[repr(C)] pub struct pdiv_map { pub pdiv: u8, pub hw_val: u8 }
#[repr(C)] pub struct div_nmp { pub divn_shift:u8,pub divn_width:u8,pub divm_shift:u8,pub divm_width:u8,pub divp_shift:u8,pub divp_width:u8,pub override_divn_shift:u8,pub override_divm_shift:u8,pub override_divp_shift:u8 }
pub const MAX_PLL_MISC_REG_COUNT: usize = 6;

#[repr(C)] pub struct tegra_clk_pll_params {
 pub input_min:c_ulong,pub input_max:c_ulong,pub cf_min:c_ulong,pub cf_max:c_ulong,pub vco_min:c_ulong,pub vco_max:c_ulong,
 pub base_reg:u32,pub misc_reg:u32,pub lock_reg:u32,pub lock_mask:u32,pub lock_enable_bit_idx:u32,pub iddq_reg:u32,pub iddq_bit_idx:u32,pub reset_reg:u32,pub reset_bit_idx:u32,pub sdm_din_reg:u32,pub sdm_din_mask:u32,pub sdm_ctrl_reg:u32,pub sdm_ctrl_en_mask:u32,pub ssc_ctrl_reg:u32,pub ssc_ctrl_en_mask:u32,pub aux_reg:u32,pub dyn_ramp_reg:u32,pub ext_misc_reg:[u32;6],pub pmc_divnm_reg:u32,pub pmc_divp_reg:u32,pub flags:u32,pub stepa_shift:c_int,pub stepb_shift:c_int,pub lock_delay:c_int,pub max_p:c_int,pub defaults_set:bool,pub pdiv_tohw:*const pdiv_map,pub div_nmp:*mut div_nmp,pub freq_table:*mut tegra_clk_pll_freq_table,pub fixed_rate:c_ulong,pub mdiv_default:u16,
 pub round_p_to_pdiv:Option<unsafe extern "C" fn(u32,*mut u32)->u32>,pub set_gain:Option<unsafe extern "C" fn(*mut tegra_clk_pll_freq_table)>,pub calc_rate:Option<unsafe extern "C" fn(*mut clk_hw,*mut tegra_clk_pll_freq_table,c_ulong,c_ulong)->c_int>,pub adjust_vco:Option<unsafe extern "C" fn(*mut tegra_clk_pll_params,c_ulong)->c_ulong>,pub set_defaults:Option<unsafe extern "C" fn(*mut tegra_clk_pll)>,pub dyn_ramp:Option<unsafe extern "C" fn(*mut tegra_clk_pll,*mut tegra_clk_pll_freq_table)->c_int>,pub pre_rate_change:Option<unsafe extern "C" fn()->c_int>,pub post_rate_change:Option<unsafe extern "C" fn()>
}
#[repr(C)] pub struct tegra_clk_pll { pub hw:clk_hw,pub clk_base:*mut c_void,pub pmc:*mut c_void,pub lock:*mut spinlock_t,pub params:*mut tegra_clk_pll_params }
#[repr(C)] pub struct tegra_audio_clk_info { pub name:*mut c_char,pub pll_params:*mut tegra_clk_pll_params,pub clk_id:c_int,pub parent:*mut c_char }
#[repr(C)] pub struct tegra_clk_pll_out { pub hw:clk_hw,pub reg:*mut c_void,pub enb_bit_idx:u8,pub rst_bit_idx:u8,pub lock:*mut spinlock_t,pub flags:u8 }
#[repr(C)] pub struct tegra_clk_periph_regs { pub enb_reg:u32,pub enb_set_reg:u32,pub enb_clr_reg:u32,pub rst_reg:u32,pub rst_set_reg:u32,pub rst_clr_reg:u32 }
#[repr(C)] pub struct tegra_clk_periph_gate { pub magic:u32,pub hw:clk_hw,pub clk_base:*mut c_void,pub flags:u8,pub clk_num:c_int,pub enable_refcnt:*mut c_int,pub regs:*const tegra_clk_periph_regs }
#[repr(C)] pub struct tegra_clk_periph_fixed { pub hw:clk_hw,pub base:*mut c_void,pub regs:*const tegra_clk_periph_regs,pub mul:c_uint,pub div:c_uint,pub num:c_uint }
#[repr(C)] pub struct tegra_clk_periph { pub magic:u32,pub hw:clk_hw,pub mux:clk_mux,pub divider:tegra_clk_frac_div,pub gate:tegra_clk_periph_gate,pub mux_ops:*const clk_ops,pub div_ops:*const clk_ops,pub gate_ops:*const clk_ops }
#[repr(C)] pub union tegra_periph_parent { pub parent_names:*const *const c_char,pub parent_name:*const c_char }
#[repr(C)] pub struct tegra_periph_init_data { pub name:*const c_char,pub clk_id:c_int,pub p:tegra_periph_parent,pub num_parents:c_int,pub periph:tegra_clk_periph,pub offset:u32,pub con_id:*const c_char,pub dev_id:*const c_char,pub flags:c_ulong }
#[repr(C)] pub struct tegra_clk_super_mux { pub hw:clk_hw,pub reg:*mut c_void,pub frac_div:tegra_clk_frac_div,pub div_ops:*const clk_ops,pub width:u8,pub flags:u8,pub div2_index:u8,pub pllx_index:u8,pub lock:*mut spinlock_t }
#[repr(C)] pub struct tegra_sdmmc_mux { pub hw:clk_hw,pub reg:*mut c_void,pub lock:*mut spinlock_t,pub gate_ops:*const clk_ops,pub gate:tegra_clk_periph_gate,pub div_flags:u8 }
#[repr(C)] pub struct tegra_clk_init_table { pub clk_id:c_uint,pub parent_id:c_uint,pub rate:c_ulong,pub state:c_int }
#[repr(C)] pub struct tegra_clk_duplicate { pub clk_id:c_int,pub lookup:clk_lookup }
#[repr(C)] pub struct tegra_clk { pub dt_id:c_int,pub present:bool }
#[repr(C)] pub struct tegra_devclk { pub dt_id:c_int,pub dev_id:*mut c_char,pub con_id:*mut c_char }

pub const TEGRA_DIVIDER_ROUND_UP:u8=1<<0; pub const TEGRA_DIVIDER_FIXED:u8=1<<1; pub const TEGRA_DIVIDER_INT:u8=1<<2; pub const TEGRA_DIVIDER_UART:u8=1<<3;
pub const TEGRA_PLL_USE_LOCK:u32=1<<0; pub const TEGRA_PLL_HAS_CPCON:u32=1<<1; pub const TEGRA_PLL_SET_LFCON:u32=1<<2; pub const TEGRA_PLL_SET_DCCON:u32=1<<3; pub const TEGRA_PLLU:u32=1<<4; pub const TEGRA_PLLM:u32=1<<5; pub const TEGRA_PLL_FIXED:u32=1<<6; pub const TEGRA_PLLE_CONFIGURE:u32=1<<7; pub const TEGRA_PLL_LOCK_MISC:u32=1<<8; pub const TEGRA_PLL_BYPASS:u32=1<<9; pub const TEGRA_PLL_HAS_LOCK_ENABLE:u32=1<<10; pub const TEGRA_MDIV_NEW:u32=1<<11; pub const TEGRA_PLLMB:u32=1<<12; pub const TEGRA_PLL_VCO_OUT:u32=1<<13;
pub const TEGRA_CLK_PERIPH_GATE_MAGIC:u32=0x17760309; pub const TEGRA_PERIPH_NO_RESET:u8=1<<0; pub const TEGRA_PERIPH_ON_APB:u8=1<<2; pub const TEGRA_PERIPH_WAR_1005168:u8=1<<3; pub const TEGRA_PERIPH_NO_DIV:u8=1<<4; pub const TEGRA_PERIPH_NO_GATE:u8=1<<5; pub const TEGRA_CLK_PERIPH_MAGIC:u32=0x18221223; pub const TEGRA_DIVIDER_2:u8=1; pub const TEGRA210_CPU_CLK:u8=2; pub const TEGRA20_SUPER_CLK:u8=4;

/* Declaration-only external kernel types. */
use core::ffi::{c_char,c_int,c_uint,c_ulong,c_void};
extern "C" { pub static mut periph_clk_enb_refcnt:*mut c_int; }
extern "C" { pub fn tegra_clk_register_sync_source(name:*const c_char,max_rate:c_ulong)->*mut clk; pub fn tegra_clk_register_divider(name:*const c_char,parent_name:*const c_char,reg:*mut c_void,flags:c_ulong,clk_divider_flags:u8,shift:u8,width:u8,frac_width:u8,lock:*mut spinlock_t)->*mut clk; pub fn tegra_clk_register_mc(name:*const c_char,parent_name:*const c_char,reg:*mut c_void,lock:*mut spinlock_t)->*mut clk; }
extern "C" { pub fn tegra_clk_register_pll(name:*const c_char,parent_name:*const c_char,clk_base:*mut c_void,pmc:*mut c_void,flags:c_ulong,params:*mut tegra_clk_pll_params,lock:*mut spinlock_t)->*mut clk; pub fn tegra_clk_register_plle(name:*const c_char,parent_name:*const c_char,clk_base:*mut c_void,pmc:*mut c_void,flags:c_ulong,params:*mut tegra_clk_pll_params,lock:*mut spinlock_t)->*mut clk; }
extern "C" {
 pub fn tegra_clk_register_pllxc(name:*const c_char,parent_name:*const c_char,base:*mut c_void,pmc:*mut c_void,flags:c_ulong,p:*mut tegra_clk_pll_params,l:*mut spinlock_t)->*mut clk;
 pub fn tegra_clk_register_pllm(name:*const c_char,parent_name:*const c_char,base:*mut c_void,pmc:*mut c_void,flags:c_ulong,p:*mut tegra_clk_pll_params,l:*mut spinlock_t)->*mut clk;
 pub fn tegra_clk_register_pllc(name:*const c_char,parent_name:*const c_char,base:*mut c_void,pmc:*mut c_void,flags:c_ulong,p:*mut tegra_clk_pll_params,l:*mut spinlock_t)->*mut clk;
 pub fn tegra_clk_register_pllre(name:*const c_char,parent_name:*const c_char,base:*mut c_void,pmc:*mut c_void,flags:c_ulong,p:*mut tegra_clk_pll_params,l:*mut spinlock_t,parent_rate:c_ulong)->*mut clk;
 pub fn tegra_clk_register_pll_out(name:*const c_char,parent_name:*const c_char,reg:*mut c_void,enb:u8,rst:u8,flags:c_ulong,pll_div_flags:u8,lock:*mut spinlock_t)->*mut clk;
 pub fn tegra_clk_register_periph_gate(name:*const c_char,parent:*const c_char,gate_flags:u8,base:*mut c_void,flags:c_ulong,clk_num:c_int,refs:*mut c_int)->*mut clk;
 pub fn tegra_clk_register_periph_fixed(name:*const c_char,parent:*const c_char,flags:c_ulong,base:*mut c_void,mul:c_uint,div:c_uint,num:c_uint)->*mut clk;
 pub fn tegra_clk_register_periph_data(base:*mut c_void,init:*mut tegra_periph_init_data)->*mut clk;
 pub fn tegra_clk_register_super_mux(name:*const c_char,parents:*const *const c_char,num:u8,flags:c_ulong,reg:*mut c_void,super_flags:u8,width:u8,pllx:u8,div2:u8,lock:*mut spinlock_t)->*mut clk;
 pub fn tegra_clk_register_sdmmc_mux_div(name:*const c_char,base:*mut c_void,offset:u32,clk_num:u32,div_flags:u8,flags:c_ulong,lock:*mut c_void)->*mut clk;
 pub fn tegra_init_special_resets(num:c_uint,assert_fn:Option<unsafe extern "C" fn(c_ulong)->c_int>,deassert_fn:Option<unsafe extern "C" fn(c_ulong)->c_int>);
 pub fn tegra_init_from_table(tbl:*mut tegra_clk_init_table,clks:*mut *mut clk,max:c_int);
 pub fn tegra_init_dup_clks(tbl:*mut tegra_clk_duplicate,clks:*mut *mut clk,max:c_int);
 pub fn get_reg_bank(clkid:c_int)->*const tegra_clk_periph_regs;
 pub fn tegra_clk_init(base:*mut c_void,num:c_int,banks:c_int)->*mut *mut clk;
 pub fn tegra_fixed_clk_init(clks:*mut tegra_clk); pub fn tegra_clk_osc_resume(base:*mut c_void); pub fn tegra_clk_set_pllp_out_cpu(enable:bool); pub fn tegra_clk_periph_suspend(); pub fn tegra_clk_periph_resume();
 pub fn tegra_pll_wait_for_lock(pll:*mut tegra_clk_pll)->c_int; pub fn tegra_pll_get_fixed_mdiv(hw:*mut clk_hw,input:c_ulong)->u16; pub fn tegra_pll_p_div_to_hw(pll:*mut tegra_clk_pll,p:u8)->c_int;
 pub fn div_frac_get(rate:c_ulong,parent_rate:c_uint,width:u8,frac_width:u8,flags:u8)->c_int;
}
/* Remaining declarations retain the C ABI through the source dependency; these aliases preserve header-level layout. */
#[allow(non_camel_case_types)] pub type clk= c_void; #[allow(non_camel_case_types)] pub type clk_hw=c_void; #[allow(non_camel_case_types)] pub type clk_ops=c_void; #[allow(non_camel_case_types)] pub type clk_mux=c_void; #[allow(non_camel_case_types)] pub type spinlock_t=c_void; #[allow(non_camel_case_types)] pub type clk_lookup=c_void; #[allow(non_camel_case_types)] pub type tegra_clk_pll_opaque=c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
