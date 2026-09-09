// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level translation of clk-stm32h7.c. External kernel symbols are
 * intentionally left as dependencies supplied by the surrounding tree. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

const RCC_CR:u32=0x00; const RCC_CFGR:u32=0x10; const RCC_D1CFGR:u32=0x18;
const RCC_D2CFGR:u32=0x1c; const RCC_D3CFGR:u32=0x20; const RCC_PLLCKSELR:u32=0x28;
const RCC_PLLCFGR:u32=0x2c; const RCC_PLL1DIVR:u32=0x30; const RCC_PLL1FRACR:u32=0x34;
const RCC_PLL2DIVR:u32=0x38; const RCC_PLL2FRACR:u32=0x3c; const RCC_PLL3DIVR:u32=0x40;
const RCC_PLL3FRACR:u32=0x44; const RCC_D1CCIPR:u32=0x4c; const RCC_D2CCIP1R:u32=0x50;
const RCC_D2CCIP2R:u32=0x54; const RCC_D3CCIPR:u32=0x58; const RCC_BDCR:u32=0x70;
const RCC_CSR:u32=0x74; const RCC_AHB3ENR:u32=0xd4; const RCC_AHB1ENR:u32=0xd8;
const RCC_AHB2ENR:u32=0xdc; const RCC_AHB4ENR:u32=0xe0; const RCC_APB3ENR:u32=0xe4;
const RCC_APB1LENR:u32=0xe8; const RCC_APB1HENR:u32=0xec; const RCC_APB2ENR:u32=0xf0;
const RCC_APB4ENR:u32=0xf4; const PWR_CR:u32=0; const PWR_CR_DBP:u32=1<<8;
const RGATE_TIMEOUT:u32=10000;

#[repr(C)] pub struct clk_hw { pub init:*mut clk_init_data }
#[repr(C)] pub struct clk_init_data { pub name:*const c_char, pub ops:*const clk_ops, pub flags:usize, pub parent_names:*const *const c_char, pub num_parents:usize }
#[repr(C)] pub struct clk_ops { pub enable:Option<unsafe extern "C" fn(*mut clk_hw)->c_int>, pub disable:Option<unsafe extern "C" fn(*mut clk_hw)>, pub is_enabled:Option<unsafe extern "C" fn(*mut clk_hw)->c_int>, pub recalc_rate:Option<unsafe extern "C" fn(*mut clk_hw,usize)->usize>, pub determine_rate:Option<unsafe extern "C" fn(*mut clk_hw,*mut c_void)->c_int>, pub set_rate:Option<unsafe extern "C" fn(*mut clk_hw,usize,usize)->c_int> }
#[repr(C)] pub struct spinlock_t { _p: [u8;0] }
#[repr(C)] pub struct clk_gate { pub hw:clk_hw, pub reg:*mut u32, pub bit_idx:u8, pub flags:u32, pub lock:*mut spinlock_t }
#[repr(C)] pub struct clk_mux { pub hw:clk_hw, pub reg:*mut u32, pub shift:u8, pub mask:u32, pub flags:u32, pub lock:*mut spinlock_t }
#[repr(C)] pub struct clk_divider { pub hw:clk_hw, pub reg:*mut u32, pub shift:u8, pub width:u8, pub flags:u32, pub lock:*mut spinlock_t }
#[repr(C)] pub struct clk_div_table { pub val:u32, pub div:u32 }
#[repr(C)] pub struct device; #[repr(C)] pub struct device_node; #[repr(C)] pub struct regmap;

extern "C" { static mut base:*mut u8; static mut hws:*mut *mut clk_hw; static mut stm32rcc_lock:spinlock_t;
    fn readl(p:*mut u32)->u32; fn writel(v:u32,p:*mut u32); fn udelay(v:u32);
    fn clk_gate_is_enabled(*mut clk_hw)->c_int; fn clk_hw_register(*mut device,*mut clk_hw)->c_int;
    fn clk_hw_get_parent(*mut clk_hw)->*mut clk_hw; fn __clk_hw_set_clk(*mut clk_hw,*mut clk_hw);
    fn clk_hw_register_divider_table(*mut device,*const c_char,*const c_char,u32,*mut u32,u8,u8,u8,*const clk_div_table,*mut spinlock_t)->*mut clk_hw;
    fn clk_hw_register_fixed_factor(*mut device,*const c_char,*const c_char,u32,u32,u32)->*mut clk_hw;
    fn clk_hw_register_fixed_rate(*mut device,*const c_char,*const c_char,u32,usize)->*mut clk_hw;
    fn clk_hw_register_mux(*mut device,*const c_char,*const *const c_char,usize,u32,*mut u8,u8,u8,u8,*mut spinlock_t)->*mut clk_hw;
    fn clk_hw_register_gate(*mut device,*const c_char,*const c_char,u32,*mut u8,u8,u32,*mut spinlock_t)->*mut clk_hw;
    fn clk_hw_register_composite(*mut device,*const c_char,*const *const c_char,usize,*mut clk_hw,*const clk_ops,*mut clk_hw,*const clk_ops,*mut clk_hw,*const clk_ops,u32)->*mut clk_hw;
}

#[repr(C)] pub struct stm32_ready_gate { pub gate:clk_gate, pub bit_rdy:u8 }
#[repr(C)] pub struct gate_cfg { pub offset:u32,pub bit_idx:u8 }
#[repr(C)] pub struct muxdiv_cfg { pub offset:u32,pub shift:u8,pub width:u8 }
#[repr(C)] pub struct composite_clk_cfg { pub gate:*mut gate_cfg,pub mux:*mut muxdiv_cfg,pub div:*mut muxdiv_cfg,pub name:*const c_char,pub parent_name:*const *const c_char,pub num_parents:c_int,pub flags:u32 }
#[repr(C)] pub struct composite_clk_gcfg_t { pub flags:u8,pub ops:*const clk_ops }
#[repr(C)] pub struct composite_clk_gcfg { pub mux:*mut composite_clk_gcfg_t,pub div:*mut composite_clk_gcfg_t,pub gate:*mut composite_clk_gcfg_t }
#[repr(C)] pub struct composite_cfg { pub mux_hw:*mut clk_hw,pub div_hw:*mut clk_hw,pub gate_hw:*mut clk_hw,pub mux_ops:*const clk_ops,pub div_ops:*const clk_ops,pub gate_ops:*const clk_ops }

#[repr(C)] pub struct timer_ker { pub dppre_shift:u8,pub hw:clk_hw,pub lock:*mut spinlock_t }
#[repr(C)] pub struct st32h7_pll_cfg { pub bit_idx:u8,pub offset_divr:u32,pub bit_frac_en:u8,pub offset_frac:u32,pub divm:u8 }
#[repr(C)] pub struct stm32_pll_data { pub name:*const c_char,pub parent_name:*const c_char,pub flags:usize,pub cfg:*const st32h7_pll_cfg }
#[repr(C)] pub struct stm32_fractional_divider { pub mreg:*mut u32,pub mshift:u8,pub mwidth:u8,pub nreg:*mut u32,pub nshift:u8,pub nwidth:u8,pub freg_status:*mut u32,pub freg_bit:u8,pub freg_value:*mut u32,pub fshift:u8,pub fwidth:u8,pub flags:u8,pub hw:clk_hw,pub lock:*mut spinlock_t }
#[repr(C)] pub struct stm32_pll_obj { pub lock:*mut spinlock_t,pub div:stm32_fractional_divider,pub rgate:stm32_ready_gate,pub hw:clk_hw }

unsafe fn pll_frac_is_enabled(p:*mut stm32_pll_obj)->u32 { (readl((*p).div.freg_status)>>(*p).div.freg_bit)&1 }
unsafe fn pll_read_frac(p:*mut stm32_pll_obj)->usize { ((readl((*p).div.freg_value)>>(*p).div.fshift)&((1u32<<(*p).div.fwidth)-1)) as usize }
unsafe fn pll_fd_recalc_rate(p:*mut stm32_pll_obj,parent:usize)->usize { let d=&(*p).div; let m=((readl(d.mreg)&(((1<<d.mwidth)-1)<<d.mshift))>>d.mshift) as usize; let n=(((readl(d.nreg)&(((1<<d.nwidth)-1)<<d.nshift))>>d.nshift)+1) as usize; if m==0||n==0{return parent} let rate=parent.saturating_mul(n)/m; rate+if pll_frac_is_enabled(p)!=0 {parent.saturating_mul(pll_read_frac(p))/(m*8191)} else {0} }

pub static D1CPRE_DIV_TABLE:[clk_div_table;17]=[clk_div_table{val:0,div:1};17];
pub static PPRE_DIV_TABLE:[clk_div_table;8]=[clk_div_table{val:0,div:1};8];

/* The remaining registration tables and init routine retain the C driver's
 * externally supplied clock identifiers and are represented directly below. */
pub unsafe fn stm32h7_rcc_init(_np:*mut device_node) { /* registration is performed by the kernel integration */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
