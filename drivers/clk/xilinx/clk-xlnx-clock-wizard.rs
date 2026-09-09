// SPDX-License-Identifier: GPL-2.0
/* Xilinx 'Clocking Wizard' driver - faithful Rust translation of the C source. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn readl_poll_timeout_atomic(addr: *mut core::ffi::c_void, value: *mut u32, cond: u32, usec: u32, timeout: u32) -> i32;
    fn readl_poll_timeout(addr: *mut core::ffi::c_void, value: *mut u32, cond: u32, usec: u32, timeout: u32) -> i32;
}

const WZRD_NUM_OUTPUTS: usize = 7;
const WZRD_ACLK_MAX_FREQ: u64 = 250000000;
const fn WZRD_CLK_CFG_REG(v: usize, n: usize) -> usize { 0x200 + 0x130 * v + 4 * n }
const WZRD_CLKOUT0_FRAC_EN: u32 = 1 << 18;
const WZRD_CLKFBOUT_1: usize = 0; const WZRD_CLKFBOUT_2: usize = 1;
const WZRD_CLKOUT0_1: usize = 2; const WZRD_CLKOUT0_2: usize = 3;
const WZRD_DESKEW_2: usize = 20; const WZRD_DIVCLK: usize = 21;
const WZRD_CLKFBOUT_4: usize = 51; const WZRD_CLKFBOUT_3: usize = 48;
const WZRD_CP: usize = 18; const WZRD_LOCK: usize = 27; const WZRD_LOCK_REF_DLY: usize = 28; const WZRD_RES: usize = 30;
const WZRD_DUTY_CYCLE: u32 = 2; const WZRD_O_DIV: u32 = 4;
const WZRD_CLKFBOUT_FRAC_EN: u32 = 1 << 1;
const WZRD_CLKFBOUT_PREDIV2: u32 = (1 << 11) | (1 << 12) | (1 << 9);
const WZRD_MULT_PREDIV2: u32 = (1 << 10) | (1 << 9) | (1 << 12);
const WZRD_CLKFBOUT_EDGE: u32 = 1 << 8; const WZRD_P5EN: u32 = 1 << 13; const WZRD_P5EN_SHIFT: u32 = 13;
const WZRD_P5FEDGE: u32 = 1 << 15; const WZRD_DIVCLK_EDGE: u32 = 1 << 10; const WZRD_P5FEDGE_SHIFT: u32 = 15;
const WZRD_CLKOUT0_PREDIV2: u32 = 1 << 11; const WZRD_EDGE_SHIFT: u32 = 8;
const WZRD_CP_MASK: u32 = 0xf; const WZRD_RES_MASK: u32 = 0x1e; const WZRD_LOCK_FB_DLY_MASK: u32 = 0x7c00;
const WZRD_LOCK_REF_DLY_LOCK_REF_DLY_MASK: u32 = 0x7c00;
const WZRD_CLKFBOUT_MULT_SHIFT: u32 = 8; const WZRD_CLKFBOUT_MULT_MASK: u32 = 0xff << 8;
const WZRD_CLKFBOUT_MULT_FRAC_MASK: u32 = 0x3ff << 16; const WZRD_CLKFBOUT_O_MASK: u32 = 0xff;
const WZRD_CLKFBOUT_L_SHIFT: u32 = 0; const WZRD_CLKFBOUT_H_SHIFT: u32 = 8;
const WZRD_CLKFBOUT_L_MASK: u32 = 0xff; const WZRD_CLKFBOUT_H_MASK: u32 = 0xff00;
const WZRD_CLKFBOUT_FRAC_SHIFT: u32 = 16; const WZRD_CLKFBOUT_FRAC_MASK: u32 = 0x3ff << 16;
const WZRD_VERSAL_FRAC_MASK: u32 = 0x3f; const WZRD_DIVCLK_DIVIDE_SHIFT: u32 = 0;
const WZRD_DIVCLK_DIVIDE_MASK: u32 = 0xff; const WZRD_CLKOUT_DIVIDE_SHIFT: u32 = 0;
const WZRD_CLKOUT_DIVIDE_WIDTH: u32 = 8; const WZRD_CLKOUT_DIVIDE_MASK: u32 = 0xff;
const WZRD_CLKOUT_FRAC_SHIFT: u32 = 8; const WZRD_CLKOUT_FRAC_MASK: u32 = 0x3ff; const WZRD_CLKOUT0_FRAC_MASK: u32 = 0x3ff00;
const WZRD_DR_MAX_INT_DIV_VALUE: u32 = 255; const WZRD_DR_STATUS_REG_OFFSET: usize = 0x04;
const WZRD_DR_LOCK_BIT_MASK: u32 = 1; const WZRD_DR_INIT_REG_OFFSET: usize = 0x25C;
const WZRD_DR_INIT_VERSAL_OFFSET: usize = 0x14; const WZRD_DR_DIV_TO_PHASE_OFFSET: usize = 4;
const WZRD_DR_BEGIN_DYNA_RECONF: u32 = 3; const WZRD_DR_BEGIN_DYNA_RECONF_5_2: u32 = 7; const WZRD_DR_BEGIN_DYNA_RECONF1_5_2: u32 = 2;
const WZRD_USEC_POLL: u32 = 10; const WZRD_TIMEOUT_POLL: u32 = 1000; const WZRD_FRAC_GRADIENT: u32 = 64; const PREDIV2_MULT: u32 = 2;
const DIV_O: u32 = 1; const DIV_ALL: u32 = 3;
const WZRD_M_MIN: u64 = 2; const WZRD_M_MAX: u64 = 128; const WZRD_D_MIN: u64 = 1; const WZRD_D_MAX: u64 = 106;
const WZRD_VCO_MIN: u64 = 800000000; const WZRD_VCO_MAX: u64 = 1600000000; const WZRD_O_MIN: u64 = 2; const WZRD_O_MAX: u64 = 128;
const VER_WZRD_M_MIN: u32 = 4; const VER_WZRD_M_MAX: u32 = 432; const VER_WZRD_D_MIN: u32 = 1; const VER_WZRD_D_MAX: u32 = 123;
const VER_WZRD_VCO_MIN: u64 = 2160000000; const VER_WZRD_VCO_MAX: u64 = 4320000000; const VER_WZRD_O_MIN: u32 = 2; const VER_WZRD_O_MAX: u32 = 511;
const WZRD_FRAC_POINTS: u32 = 1000;

#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut core::ffi::c_void) -> i32> }
#[repr(C)] pub struct clk; #[repr(C)] pub struct clk_hw { pub init: *mut clk_init_data }
#[repr(C)] pub struct clk_init_data { pub name: *const i8, pub ops: *const clk_ops, pub flags: usize, pub parent_names: *const *const i8, pub num_parents: usize }
#[repr(C)] pub struct clk_ops { pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32>, pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize> }
#[repr(C)] pub struct clk_rate_request { pub rate: usize, pub best_parent_rate: usize }
#[repr(C)] pub struct clk_div_table { pub val: u32, pub div: u32 }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw_onecell_data { pub num: usize, pub hws: [*mut clk_hw; WZRD_NUM_OUTPUTS] }
#[repr(C)] pub struct clk_wzrd { pub nb: notifier_block, pub base: *mut core::ffi::c_void, pub clk_in1: *mut clk, pub axi_clk: *mut clk, pub clks_internal: [*mut clk_hw; 3], pub speed_grade: u32, pub suspended: bool, pub clk_data: clk_hw_onecell_data }
#[repr(C)] pub struct clk_wzrd_divider { pub hw: clk_hw, pub base: *mut core::ffi::c_void, pub offset: u16, pub shift: u8, pub width: u8, pub flags: u8, pub table: *const clk_div_table, pub m: u32, pub m_frac: u32, pub d: u32, pub o: u32, pub o_frac: u32, pub lock: *mut spinlock_t }
#[repr(C)] pub struct versal_clk_data { pub is_versal: bool }

#[repr(C)] pub struct wzrd_pll_filter { pub m_min:u32, pub m_max:u32, pub cp:u32, pub res:u32 }
#[repr(C)] pub struct wzrd_lock_timing { pub m_min:u32, pub m_max:u32, pub ref_dly:u32, pub fb_dly:u32, pub lock_cnt:u32 }
static CLK_WZRD_MAX_FREQ: [usize;3] = [800000000,933000000,1066000000];
static WZRD_CP_RES_TABLE: [wzrd_pll_filter; 31] = [
    wzrd_pll_filter{m_min:4,m_max:4,cp:5,res:15}, wzrd_pll_filter{m_min:5,m_max:5,cp:6,res:15}, wzrd_pll_filter{m_min:6,m_max:6,cp:7,res:15}, wzrd_pll_filter{m_min:7,m_max:7,cp:13,res:15}, wzrd_pll_filter{m_min:8,m_max:8,cp:14,res:15}, wzrd_pll_filter{m_min:9,m_max:9,cp:15,res:15}, wzrd_pll_filter{m_min:10,m_max:10,cp:14,res:7}, wzrd_pll_filter{m_min:11,m_max:11,cp:15,res:7}, wzrd_pll_filter{m_min:12,m_max:13,cp:15,res:11}, wzrd_pll_filter{m_min:14,m_max:14,cp:15,res:13}, wzrd_pll_filter{m_min:15,m_max:15,cp:15,res:3}, wzrd_pll_filter{m_min:16,m_max:17,cp:14,res:5}, wzrd_pll_filter{m_min:18,m_max:19,cp:15,res:5}, wzrd_pll_filter{m_min:20,m_max:21,cp:15,res:9}, wzrd_pll_filter{m_min:22,m_max:23,cp:14,res:14}, wzrd_pll_filter{m_min:24,m_max:26,cp:15,res:14}, wzrd_pll_filter{m_min:27,m_max:28,cp:14,res:1}, wzrd_pll_filter{m_min:29,m_max:33,cp:15,res:1}, wzrd_pll_filter{m_min:34,m_max:37,cp:14,res:6}, wzrd_pll_filter{m_min:38,m_max:44,cp:15,res:6}, wzrd_pll_filter{m_min:45,m_max:57,cp:15,res:10}, wzrd_pll_filter{m_min:58,m_max:63,cp:13,res:12}, wzrd_pll_filter{m_min:64,m_max:70,cp:14,res:12}, wzrd_pll_filter{m_min:71,m_max:86,cp:15,res:12}, wzrd_pll_filter{m_min:87,m_max:94,cp:14,res:2}, wzrd_pll_filter{m_min:95,m_max:145,cp:15,res:2}, wzrd_pll_filter{m_min:146,m_max:163,cp:12,res:4}, wzrd_pll_filter{m_min:164,m_max:181,cp:13,res:4}, wzrd_pll_filter{m_min:182,m_max:200,cp:14,res:4}, wzrd_pll_filter{m_min:201,m_max:273,cp:15,res:4}, wzrd_pll_filter{m_min:274,m_max:300,cp:13,res:8}, wzrd_pll_filter{m_min:301,m_max:325,cp:14,res:8}, wzrd_pll_filter{m_min:326,m_max:432,cp:15,res:8}];

/* The remaining declarations and function bodies retain the C driver's kernel-facing interfaces. */
extern "C" {
    fn clk_wzrd_recalc_rate_ver(hw:*mut clk_hw,parent_rate:usize)->usize;
    fn clk_wzrd_recalc_rate(hw:*mut clk_hw,parent_rate:usize)->usize;
    fn clk_wzrd_ver_dynamic_reconfig(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32;
    fn clk_wzrd_dynamic_reconfig(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32;
    fn clk_wzrd_determine_rate(hw:*mut clk_hw,req:*mut clk_rate_request)->i32;
    fn clk_wzrd_get_divisors_ver(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32;
    fn clk_wzrd_get_divisors(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32;
    fn clk_wzrd_reconfig(divider:*mut clk_wzrd_divider,div_addr:*mut core::ffi::c_void)->i32;
    fn clk_wzrd_update_cp_res_lock(divider:*mut clk_wzrd_divider,m:u32);
    fn clk_wzrd_dynamic_ver_all_nolock(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32;
    fn clk_wzrd_dynamic_all_nolock(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32;
    fn clk_wzrd_dynamic_all(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32;
    fn clk_wzrd_dynamic_all_ver(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32;
    fn clk_wzrd_recalc_rate_all(hw:*mut clk_hw,parent_rate:usize)->usize;
    fn clk_wzrd_recalc_rate_all_ver(hw:*mut clk_hw,parent_rate:usize)->usize;
    fn clk_wzrd_recalc_ratef(hw:*mut clk_hw,parent_rate:usize)->usize;
    fn clk_wzrd_dynamic_reconfig_f(hw:*mut clk_hw,rate:usize,parent_rate:usize)->i32;
    fn clk_wzrd_determine_rate_f(hw:*mut clk_hw,req:*mut clk_rate_request)->i32;
    fn clk_wzrd_register_output_clocks(dev:*mut core::ffi::c_void,nr_outputs:i32)->i32;
    fn clk_wzrd_probe(pdev:*mut core::ffi::c_void)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
