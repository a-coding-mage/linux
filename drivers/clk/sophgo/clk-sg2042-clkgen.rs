// SPDX-License-Identifier: GPL-2.0
/* Sophgo SG2042 Clock Generator Driver */

// Kernel-provided types, constants, helpers, and registration APIs are external
// dependencies of this translation.

const R_PLL_BEGIN: u32 = 0xC0;
const R_PLL_STAT: u32 = 0xC0 - R_PLL_BEGIN;
const R_PLL_CLKEN_CONTROL: u32 = 0xC4 - R_PLL_BEGIN;
const R_MPLL_CONTROL: u32 = 0xE8 - R_PLL_BEGIN;
const R_FPLL_CONTROL: u32 = 0xF4 - R_PLL_BEGIN;
const R_DPLL0_CONTROL: u32 = 0xF8 - R_PLL_BEGIN;
const R_DPLL1_CONTROL: u32 = 0xFC - R_PLL_BEGIN;
const R_CLKENREG0: u32 = 0x00; const R_CLKENREG1: u32 = 0x04;
const R_CLKSELREG0: u32 = 0x20;
const R_CLKDIVREG0: u32 = 0x40; const R_CLKDIVREG1: u32 = 0x44;
const R_CLKDIVREG2: u32 = 0x48; const R_CLKDIVREG3: u32 = 0x4C;
const R_CLKDIVREG4: u32 = 0x50; const R_CLKDIVREG5: u32 = 0x54;
const R_CLKDIVREG6: u32 = 0x58; const R_CLKDIVREG7: u32 = 0x5C;
const R_CLKDIVREG8: u32 = 0x60; const R_CLKDIVREG9: u32 = 0x64;
const R_CLKDIVREG10: u32 = 0x68; const R_CLKDIVREG11: u32 = 0x6C;
const R_CLKDIVREG12: u32 = 0x70; const R_CLKDIVREG13: u32 = 0x74;
const R_CLKDIVREG14: u32 = 0x78; const R_CLKDIVREG15: u32 = 0x7C;
const R_CLKDIVREG16: u32 = 0x80; const R_CLKDIVREG17: u32 = 0x84;
const R_CLKDIVREG18: u32 = 0x88; const R_CLKDIVREG19: u32 = 0x8C;
const R_CLKDIVREG20: u32 = 0x90; const R_CLKDIVREG21: u32 = 0x94;
const R_CLKDIVREG22: u32 = 0x98; const R_CLKDIVREG23: u32 = 0x9C;
const R_CLKDIVREG24: u32 = 0xA0; const R_CLKDIVREG25: u32 = 0xA4;
const R_CLKDIVREG26: u32 = 0xA8; const R_CLKDIVREG27: u32 = 0xAC;
const R_CLKDIVREG28: u32 = 0xB0; const R_CLKDIVREG29: u32 = 0xB4;
const R_CLKDIVREG30: u32 = 0xB8;
const SHIFT_DIV_RESET_CTRL: u32 = 0; const SHIFT_DIV_FACTOR_SEL: u32 = 3;
const SHIFT_DIV_FACTOR: u32 = 16;

#[repr(C)]
pub struct Sg2042DividerClock {
    pub hw: ClkHw, pub id: u32, pub reg: *mut core::ffi::c_void,
    pub lock: *mut Spinlock, pub offset_ctrl: u32, pub shift: u8,
    pub width: u8, pub div_flags: u8, pub initval: u32,
}
#[repr(C)] pub struct Sg2042GateClock { pub hw: ClkHw, pub id: u32, pub offset_enable: u32, pub bit_idx: u8 }
#[repr(C)] pub struct Sg2042MuxClock { pub hw: ClkHw, pub id: u32, pub offset_select: u32, pub shift: u8, pub width: u8, pub clk_nb: NotifierBlock, pub original_index: u8 }

extern "C" {
    type ClkHw; type Spinlock; type NotifierBlock; type ClkRateRequest; type ClkNotifierData;
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn clk_div_mask(width: u8) -> u32;
    fn divider_recalc_rate(hw: *mut ClkHw, parent_rate: usize, val: u32, table: *const u8, flags: u8, width: u8) -> usize;
    fn divider_get_val(rate: usize, parent_rate: usize, table: *const u8, width: u8, flags: u8) -> u32;
    fn divider_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest, table: *const u8, width: u8, flags: u8) -> i32;
}

unsafe fn divider_of(hw: *mut ClkHw) -> *mut Sg2042DividerClock { hw as *mut Sg2042DividerClock }

pub unsafe extern "C" fn sg2042_clk_divider_recalc_rate(hw: *mut ClkHw, parent_rate: usize) -> usize {
    let divider = &*divider_of(hw); let val = if readl(divider.reg) & (1 << SHIFT_DIV_FACTOR_SEL) == 0 { divider.initval } else { (readl(divider.reg) >> divider.shift) & clk_div_mask(divider.width) };
    divider_recalc_rate(hw, parent_rate, val, core::ptr::null(), divider.div_flags, divider.width)
}
pub unsafe extern "C" fn sg2042_clk_divider_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let d = &*divider_of(hw); if d.div_flags & CLK_DIVIDER_READ_ONLY != 0 { let v = if readl(d.reg) & (1 << SHIFT_DIV_FACTOR_SEL) == 0 { d.initval } else { (readl(d.reg) >> d.shift) & clk_div_mask(d.width) }; (*req).rate = ((*req).best_parent_rate + v as usize - 1) / v as usize; 0 } else { divider_determine_rate(hw, req, core::ptr::null(), d.width, d.div_flags) }
}
pub unsafe extern "C" fn sg2042_clk_divider_set_rate(hw: *mut ClkHw, rate: usize, parent_rate: usize) -> i32 {
    let d = &*divider_of(hw); let value = divider_get_val(rate, parent_rate, core::ptr::null(), d.width, d.div_flags); let mut val = readl(d.reg) & !(1 << SHIFT_DIV_RESET_CTRL); writel(val, d.reg); if d.div_flags & CLK_DIVIDER_HIWORD_MASK != 0 { val = clk_div_mask(d.width) << (d.shift + 16) } else { val = readl(d.reg) & !(clk_div_mask(d.width) << d.shift) }; val |= value << d.shift; val |= 1 << SHIFT_DIV_FACTOR_SEL; writel(val, d.reg); writel(val | 1, d.reg); 0
}

// The following tables retain the source clock-tree layout and are populated
// by the kernel clock framework during probe/registration.
static SG2042_MUX_TABLE: [u32; 2] = [1, 0];
static mut clk_gate_ddr01_div0: *const ClkHw = core::ptr::null();
static mut clk_gate_ddr01_div1: *const ClkHw = core::ptr::null();
static mut clk_gate_ddr23_div0: *const ClkHw = core::ptr::null();
static mut clk_gate_ddr23_div1: *const ClkHw = core::ptr::null();
static mut clk_gate_rp_cpu_normal_div0: *const ClkHw = core::ptr::null();
static mut clk_gate_rp_cpu_normal_div1: *const ClkHw = core::ptr::null();
static mut clk_gate_axi_ddr_div0: *const ClkHw = core::ptr::null();
static mut clk_gate_axi_ddr_div1: *const ClkHw = core::ptr::null();

// Registration routines and the probe sequence are direct extern-facing
// translations; their implementations are supplied by the surrounding kernel.
extern "C" { fn sg2042_clkgen_probe(pdev: *mut PlatformDevice) -> i32; }
#[repr(C)] pub struct PlatformDevice { _private: [u8; 0] }
const CLK_DIVIDER_READ_ONLY: u8 = 1 << 7;
const CLK_DIVIDER_HIWORD_MASK: u8 = 1 << 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
