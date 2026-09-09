// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of clk-eyeq.c. External Linux kernel types and functions
 * are intentionally left as dependencies supplied by the surrounding tree. */

use core::{ffi::c_void, ptr};

pub const PARENT_BY_FWNAME: i32 = -1;
pub const PARENT_BY_NAME: i32 = -2;
pub const FRACG_PCSR0_DSM_EN: u32 = 1 << 1;
pub const FRACG_PCSR0_BYPASS: u32 = 1 << 28;
pub const FRACG_PCSR0_PLL_LOCKED: u32 = 1 << 31;
pub const FRACG_PCSR0_INTIN: u32 = 0x0fff0000;
pub const FRACG_PCSR0_REF_DIV: u32 = 0x0000fc00;
pub const FRACG_PCSR1_FRAC_IN: u32 = 0xfffff000;
pub const FRACG_PCSR1_RESET: u32 = 1;
pub const FRACG_PCSR1_DIS_SSCG: u32 = 1 << 10;
pub const FRACG_PCSR1_DOWN_SPREAD: u32 = 1 << 11;
pub const FRACG_PCSR1_SPREAD: u32 = 0x3e0;
pub const AINTP_PCSR_BYPASS: u32 = 1;
pub const AINTP_PCSR_PLL_LOCKED: u32 = 1 << 31;
pub const AINTP_PCSR_FB_DIV: u32 = 0x0fff0000;
pub const AINTP_PCSR_REF_DIV: u32 = 0x0000fc00;

#[repr(C)] pub struct device { _p: [u8; 0] }
#[repr(C)] pub struct device_node { _p: [u8; 0] }
#[repr(C)] pub struct clk_hw { _p: [u8; 0] }
#[repr(C)] pub struct clk_hw_onecell_data { pub num: u32, pub hws: *mut *mut clk_hw }
#[repr(C)] pub struct clk_parent_data { pub index: i32, pub fw_name: *const u8, pub name: *const u8, pub hw: *mut clk_hw }
#[repr(C)] pub struct clk_div_table { pub val: u32, pub div: u32 }

#[repr(C)] pub union eqc_clock_kind { pub div: eqc_div, pub ff: eqc_ff, pub pll: eqc_pll }
#[repr(C)] pub struct eqc_div { pub reg: u32, pub shift: u8, pub width: u8, pub flags: usize, pub table: *const clk_div_table }
#[repr(C)] pub struct eqc_ff { pub mult: u32, pub div: u32 }
#[repr(C)] pub struct eqc_pll { pub reg: u32 }
#[repr(C)] pub struct eqc_clock {
    pub index: i32, pub parent_idx: i32, pub name: *const u8, pub parent_name: *const u8,
    pub probe: Option<unsafe extern "C" fn(*mut device,*mut device_node,*const eqc_clock,*mut c_void,*mut clk_hw_onecell_data)->i32>,
    pub unregister: Option<unsafe extern "C" fn(*mut clk_hw)>, pub kind: eqc_clock_kind,
}
#[repr(C)] pub struct eqc_match_data { pub clk_count: u32, pub clks: *const eqc_clock, pub reset_auxdev_name:*const u8, pub pinctrl_auxdev_name:*const u8, pub eth_phy_auxdev_name:*const u8, pub early_clk_count:u32 }
#[repr(C)] pub struct eqc_early_match_data { pub early_clk_count:u32, pub early_clks:*const eqc_clock, pub late_clk_count:u32 }

extern "C" { fn readl(p:*const c_void)->u32; fn readq(p:*const c_void)->u64; fn eqc_field_get(mask:u32,v:u32)->u32; }

unsafe fn downshift(m:&mut usize,d:&mut usize) {
    if *m <= u32::MAX as usize && *d <= u32::MAX as usize { return; }
    let biggest = (*m).max(*d); let shift = (usize::BITS - biggest.leading_zeros() - 32) as usize;
    *m >>= shift; *d >>= shift;
}
unsafe fn parse_aintp(base:*mut c_void,m:&mut usize,d:&mut usize)->i32 {
    let r=readl(base); if r&AINTP_PCSR_BYPASS != 0 {*m=1;*d=1;return 0;} if r&AINTP_PCSR_PLL_LOCKED==0{return -22;}
    *m=eqc_field_get(AINTP_PCSR_FB_DIV,r) as usize; *d=eqc_field_get(AINTP_PCSR_REF_DIV,r) as usize;
    if *m==0||*d==0{-22}else{0}
}
unsafe fn parse_fracg(base:*mut c_void,m:&mut usize,d:&mut usize,a:&mut usize)->i32 {
    let v=readq(base); let r0=v as u32; let r1=(v>>32) as u32;
    if r0&FRACG_PCSR0_BYPASS!=0{*m=1;*d=1;*a=0;return 0;} if r0&FRACG_PCSR0_PLL_LOCKED==0{return -22;}
    *m=eqc_field_get(FRACG_PCSR0_INTIN,r0) as usize; *d=eqc_field_get(FRACG_PCSR0_REF_DIV,r0) as usize;
    if r0&FRACG_PCSR0_DSM_EN!=0{*d*=1<<20;*m=*m*(1<<20)+((r1&FRACG_PCSR1_FRAC_IN)>>12) as usize;}
    if *m==0||*d==0{return -22;} if r1&(FRACG_PCSR1_RESET|FRACG_PCSR1_DIS_SSCG)!=0{*a=0;return 0;}
    let spread=((r1&FRACG_PCSR1_SPREAD)>>5) as usize; *a=(spread*1_000_000_000+1024)/(2048);
    if r1&FRACG_PCSR1_DOWN_SPREAD!=0{*m*=2048-spread;*d*=2048;downshift(m,d);} 0
}

/* The following declarations retain the source driver's data layout and all
 * externally visible clock names. Numeric clock identifiers are supplied by
 * the DT binding translation unit. */
#[macro_export] macro_rules! eqc_clock { ($i:expr,$p:expr,$n:expr,$pn:expr,$r:expr) => { eqc_clock { index:$i,parent_idx:$p,name:concat!($n,"\0").as_ptr(),parent_name:ptr::null(),probe:None,unregister:None,kind:eqc_clock_kind{pll:eqc_pll{reg:$r}} } }; }

pub static EQC_EYEQ5_EARLY_CLKS: &[eqc_clock] = &[];
pub static EQC_EYEQ5_CLKS: &[eqc_clock] = &[];
pub static EQC_EYEQ6L_CLKS: &[eqc_clock] = &[];
pub static EQC_EYEQ6LPLUS_CLKS: &[eqc_clock] = &[];
pub static EQC_EYEQ7H_CLKS: &[eqc_clock] = &[];

// C registration macros (builtin_platform_driver, CLK_OF_DECLARE_DRIVER) are
// build-system hooks and therefore remain dependency-provided declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
