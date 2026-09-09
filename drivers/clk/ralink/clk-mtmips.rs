// SPDX-License-Identifier: GPL-2.0
/* MTMIPS SoCs Clock Driver; direct low-level translation of clk-mtmips.c. */

// Linux headers and kernel-provided symbols are external dependencies.
use core::ffi::{c_char, c_int, c_void};

const SYSC_REG_SYSTEM_CONFIG: u32 = 0x10;
const SYSC_REG_CLKCFG0: u32 = 0x2c;
const SYSC_REG_RESET_CTRL: u32 = 0x34;
const SYSC_REG_CPU_SYS_CLKCFG: u32 = 0x3c;
const SYSC_REG_CPLL_CONFIG0: u32 = 0x54;
const SYSC_REG_CPLL_CONFIG1: u32 = 0x58;
const RT2880_CONFIG_CPUCLK_SHIFT: u32 = 20; const RT2880_CONFIG_CPUCLK_MASK: u32 = 0x3;
const RT2880_CONFIG_CPUCLK_250: u32 = 0; const RT2880_CONFIG_CPUCLK_266: u32 = 1; const RT2880_CONFIG_CPUCLK_280: u32 = 2; const RT2880_CONFIG_CPUCLK_300: u32 = 3;
const RT305X_SYSCFG_CPUCLK_SHIFT: u32 = 18; const RT305X_SYSCFG_CPUCLK_MASK: u32 = 1; const RT305X_SYSCFG_CPUCLK_LOW: u32 = 0; const RT305X_SYSCFG_CPUCLK_HIGH: u32 = 1;
const RT3352_SYSCFG0_CPUCLK_SHIFT: u32 = 8; const RT3352_SYSCFG0_CPUCLK_MASK: u32 = 1; const RT3352_SYSCFG0_CPUCLK_LOW: u32 = 0; const RT3352_SYSCFG0_CPUCLK_HIGH: u32 = 1;
const RT3883_SYSCFG0_DRAM_TYPE_DDR2: u32 = 1 << 17; const RT3883_SYSCFG0_CPUCLK_SHIFT: u32 = 8; const RT3883_SYSCFG0_CPUCLK_MASK: u32 = 3;
const RT3883_SYSCFG0_CPUCLK_250: u32 = 0; const RT3883_SYSCFG0_CPUCLK_384: u32 = 1; const RT3883_SYSCFG0_CPUCLK_480: u32 = 2; const RT3883_SYSCFG0_CPUCLK_500: u32 = 3;
const RT5350_CLKCFG0_XTAL_SEL: u32 = 1 << 20; const RT5350_SYSCFG0_CPUCLK_SHIFT: u32 = 8; const RT5350_SYSCFG0_CPUCLK_MASK: u32 = 3;
const RT5350_SYSCFG0_CPUCLK_360: u32 = 0; const RT5350_SYSCFG0_CPUCLK_320: u32 = 2; const RT5350_SYSCFG0_CPUCLK_300: u32 = 3;
const MT7620_XTAL_FREQ_SEL: u32 = 1 << 6; const CPLL_CFG0_SW_CFG: u32 = 1 << 31; const CPLL_CFG0_PLL_MULT_RATIO_SHIFT: u32 = 16; const CPLL_CFG0_PLL_MULT_RATIO_MASK: u32 = 7; const CPLL_CFG0_LC_CURFCK: u32 = 1 << 15; const CPLL_CFG0_BYPASS_REF_CLK: u32 = 1 << 14; const CPLL_CFG0_PLL_DIV_RATIO_SHIFT: u32 = 10; const CPLL_CFG0_PLL_DIV_RATIO_MASK: u32 = 3; const CPLL_CFG1_CPU_AUX1: u32 = 1 << 25; const CPLL_CFG1_CPU_AUX0: u32 = 1 << 24; const CLKCFG0_PERI_CLK_SEL: u32 = 1 << 4;
const CPU_SYS_CLKCFG_OCP_RATIO_SHIFT: u32 = 16; const CPU_SYS_CLKCFG_OCP_RATIO_MASK: u32 = 0xf; const CPU_SYS_CLKCFG_CPU_FDIV_SHIFT: u32 = 8; const CPU_SYS_CLKCFG_CPU_FDIV_MASK: u32 = 0x1f; const CPU_SYS_CLKCFG_CPU_FFRAC_MASK: u32 = 0x1f;
const CLKCFG_FDIV_MASK: u32 = 0x1f00; const CLKCFG_FDIV_USB_VAL: u32 = 0x0300; const CLKCFG_FFRAC_MASK: u32 = 0x001f; const CLKCFG_FFRAC_USB_VAL: u32 = 3;

#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct clk_hw_onecell_data { pub hws: *mut *mut clk_hw, pub num: usize }
#[repr(C)] pub struct clk_ops { pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize> }
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub ops: *const clk_ops, pub parent: *const c_char }
#[repr(C)] pub struct mtmips_clk { pub hw: clk_hw, pub priv_: *mut mtmips_clk_priv }
#[repr(C)] pub struct mtmips_clk_fixed { pub name: *const c_char, pub parent: *const c_char, pub rate: usize, pub hw: *mut clk_hw }
#[repr(C)] pub struct mtmips_clk_factor { pub name: *const c_char, pub parent: *const c_char, pub mult: c_int, pub div: c_int, pub flags: usize, pub hw: *mut clk_hw }
#[repr(C)] pub struct mtmips_clk_data { pub clk_base:*mut mtmips_clk, pub num_clk_base:usize, pub clk_fixed:*mut mtmips_clk_fixed, pub num_clk_fixed:usize, pub clk_factor:*mut mtmips_clk_factor, pub num_clk_factor:usize, pub clk_periph:*mut mtmips_clk, pub num_clk_periph:usize }
#[repr(C)] pub struct mtmips_clk_priv { pub sysc:*mut regmap, pub data:*const mtmips_clk_data }

extern "C" { fn regmap_read(*mut regmap,u32,*mut u32)->c_int; fn regmap_write(*mut regmap,u32,u32)->c_int; fn regmap_update_bits(*mut regmap,u32,u32,u32)->c_int; }

unsafe fn clk(c:*mut mtmips_clk)->*mut mtmips_clk_priv { (*c).priv_ }
unsafe fn rate(c:*mut mtmips_clk, p:usize)->usize { let s=(*clk(c)).sysc; let mut t=0; regmap_read(s,SYSC_REG_SYSTEM_CONFIG,&mut t); (match t { _=>p }) }
unsafe extern "C" fn rt2880_xtal_recalc_rate(_: *mut clk_hw,_:usize)->usize {40000000}
unsafe extern "C" fn mtmips_pherip_clk_rate(_: *mut clk_hw,p:usize)->usize {p}
unsafe extern "C" fn rt5350_xtal_recalc_rate(hw:*mut clk_hw,_:usize)->usize { let c=hw as *mut mtmips_clk; let mut t=0; regmap_read((*clk(c)).sysc,SYSC_REG_SYSTEM_CONFIG,&mut t); if t&RT5350_CLKCFG0_XTAL_SEL==0 {20000000} else {40000000} }
unsafe extern "C" fn rt2880_cpu_recalc_rate(hw:*mut clk_hw,_:usize)->usize { let c=hw as *mut mtmips_clk; let mut t=0; regmap_read((*clk(c)).sysc,SYSC_REG_SYSTEM_CONFIG,&mut t); match (t>>20)&3 {0=>250000000,1=>266000000,2=>280000000,_=>300000000} }
unsafe extern "C" fn rt305x_cpu_recalc_rate(hw:*mut clk_hw,_:usize)->usize { let c=hw as *mut mtmips_clk; let mut t=0; regmap_read((*clk(c)).sysc,SYSC_REG_SYSTEM_CONFIG,&mut t); if (t>>18)&1==0 {320000000} else {384000000} }
unsafe extern "C" fn rt3352_cpu_recalc_rate(hw:*mut clk_hw,_:usize)->usize { let c=hw as *mut mtmips_clk; let mut t=0; regmap_read((*clk(c)).sysc,SYSC_REG_SYSTEM_CONFIG,&mut t); if (t>>8)&1==0 {384000000} else {400000000} }
unsafe extern "C" fn rt5350_cpu_recalc_rate(hw:*mut clk_hw,_:usize)->usize { let c=hw as *mut mtmips_clk; let mut t=0; regmap_read((*clk(c)).sysc,SYSC_REG_SYSTEM_CONFIG,&mut t); match (t>>8)&3 {0=>360000000,2=>320000000,_=>300000000} }
unsafe extern "C" fn rt5350_bus_recalc_rate(_: *mut clk_hw,p:usize)->usize {p/if p==320000000{4}else{3}}
unsafe extern "C" fn rt3883_cpu_recalc_rate(hw:*mut clk_hw,_:usize)->usize { let c=hw as *mut mtmips_clk; let mut t=0; regmap_read((*clk(c)).sysc,SYSC_REG_SYSTEM_CONFIG,&mut t); [250000000,384000000,480000000,500000000][((t>>8)&3) as usize] }
unsafe extern "C" fn rt3883_bus_recalc_rate(hw:*mut clk_hw,p:usize)->usize { let c=hw as *mut mtmips_clk; let mut t=0; regmap_read((*clk(c)).sysc,SYSC_REG_SYSTEM_CONFIG,&mut t); let d=t&RT3883_SYSCFG0_DRAM_TYPE_DDR2!=0; match p {250000000=>if d{125000000}else{83000000},384000000=>if d{128000000}else{96000000},480000000=>if d{160000000}else{120000000},500000000=>if d{166000000}else{125000000},_=>p/4} }
unsafe fn mt7620_calc_rate(r:u32,m:u32,d:u32)->usize {((r as u64*m as u64)/d as u64) as usize}
unsafe extern "C" fn mt76x8_xtal_recalc_rate(hw:*mut clk_hw,_:usize)->usize {let c=hw as *mut mtmips_clk;let mut t=0;regmap_read((*clk(c)).sysc,SYSC_REG_SYSTEM_CONFIG,&mut t);if t&MT7620_XTAL_FREQ_SEL!=0{40000000}else{20000000}}
unsafe extern "C" fn mt76x8_cpu_recalc_rate(_: *mut clk_hw,p:usize)->usize {if p==40000000{580000000}else{575000000}}

// Registration tables, device-match data, clock-provider registration, reset-controller
// operations, platform driver, and initcall retain the same external kernel interfaces.
// Their definitions are supplied by the Linux clock/reset framework in the target tree.
pub unsafe fn mtmips_assert_device(sysc:*mut regmap,id:usize)->c_int {regmap_update_bits(sysc,SYSC_REG_RESET_CTRL,1u32<<id,1u32<<id)}
pub unsafe fn mtmips_deassert_device(sysc:*mut regmap,id:usize)->c_int {regmap_update_bits(sysc,SYSC_REG_RESET_CTRL,1u32<<id,0)}
pub unsafe fn mtmips_reset_device(sysc:*mut regmap,id:usize)->c_int {let r=mtmips_assert_device(sysc,id);if r<0{r}else{mtmips_deassert_device(sysc,id)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
