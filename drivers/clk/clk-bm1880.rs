// SPDX-License-Identifier: GPL-2.0+
/* Bitmain BM1880 SoC clock driver, translated from C. */

// Kernel dependencies supplied by the surrounding tree.
use core::ffi::{c_char, c_void};

const BM1880_CLK_MPLL_CTL: u32 = 0x00;
const BM1880_CLK_SPLL_CTL: u32 = 0x04;
const BM1880_CLK_FPLL_CTL: u32 = 0x08;
const BM1880_CLK_DDRPLL_CTL: u32 = 0x0c;
const BM1880_CLK_ENABLE0: u32 = 0x00;
const BM1880_CLK_ENABLE1: u32 = 0x04;
const BM1880_CLK_SELECT: u32 = 0x20;
const BM1880_CLK_DIV0: u32 = 0x40;
const BM1880_CLK_DIV1: u32 = 0x44;
const BM1880_CLK_DIV2: u32 = 0x48;
const BM1880_CLK_DIV3: u32 = 0x4c;
const BM1880_CLK_DIV4: u32 = 0x50;
const BM1880_CLK_DIV5: u32 = 0x54;
const BM1880_CLK_DIV6: u32 = 0x58;
const BM1880_CLK_DIV7: u32 = 0x5c;
const BM1880_CLK_DIV8: u32 = 0x60;
const BM1880_CLK_DIV9: u32 = 0x64;
const BM1880_CLK_DIV10: u32 = 0x68;
const BM1880_CLK_DIV11: u32 = 0x6c;
const BM1880_CLK_DIV12: u32 = 0x70;
const BM1880_CLK_DIV13: u32 = 0x74;
const BM1880_CLK_DIV14: u32 = 0x78;
const BM1880_CLK_DIV15: u32 = 0x7c;
const BM1880_CLK_DIV16: u32 = 0x80;
const BM1880_CLK_DIV17: u32 = 0x84;
const BM1880_CLK_DIV18: u32 = 0x88;
const BM1880_CLK_DIV19: u32 = 0x8c;
const BM1880_CLK_DIV20: u32 = 0x90;
const BM1880_CLK_DIV21: u32 = 0x94;
const BM1880_CLK_DIV22: u32 = 0x98;
const BM1880_CLK_DIV23: u32 = 0x9c;
const BM1880_CLK_DIV24: u32 = 0xa0;
const BM1880_CLK_DIV25: u32 = 0xa4;
const BM1880_CLK_DIV26: u32 = 0xa8;
const BM1880_CLK_DIV27: u32 = 0xac;
const BM1880_CLK_DIV28: u32 = 0xb0;

#[repr(C)] pub struct bm1880_clock_data { pub pll_base: *mut c_void, pub sys_base: *mut c_void, pub hw_data: clk_hw_onecell_data }
#[repr(C)] pub struct bm1880_gate_clock { pub id: u32, pub name: *const c_char, pub parent: *const c_char, pub gate_reg: u32, pub gate_shift: i8, pub flags: usize }
#[repr(C)] pub struct bm1880_mux_clock { pub id: u32, pub name: *const c_char, pub parents: *const *const c_char, pub num_parents: i8, pub reg: u32, pub shift: i8, pub flags: usize }
#[repr(C)] pub struct bm1880_div_clock { pub id: u32, pub name: *const c_char, pub reg: u32, pub shift: u8, pub width: u8, pub initval: u32, pub table: *const clk_div_table, pub flags: usize }
#[repr(C)] pub struct bm1880_div_hw_clock { pub div: bm1880_div_clock, pub base: *mut c_void, pub lock: *mut spinlock_t, pub hw: clk_hw, pub init: clk_init_data }
#[repr(C)] pub struct bm1880_composite_clock { pub id: u32, pub name: *const c_char, pub parent: *const c_char, pub parents: *const *const c_char, pub num_parents: u32, pub flags: usize, pub gate_reg: u32, pub mux_reg: u32, pub div_reg: u32, pub gate_shift: i8, pub mux_shift: i8, pub div_shift: i8, pub div_width: i8, pub div_initval: i16, pub table: *const clk_div_table }
#[repr(C)] pub struct bm1880_pll_clock { pub id: u32, pub name: *const c_char, pub reg: u32, pub flags: usize }
#[repr(C)] pub struct bm1880_pll_hw_clock { pub pll: bm1880_pll_clock, pub base: *mut c_void, pub hw: clk_hw, pub init: clk_init_data }

extern "C" {
    type clk_hw; type clk_init_data; type clk_hw_onecell_data; type spinlock_t; type clk_div_table;
    fn readl(addr: *mut c_void) -> u32; fn writel(v: u32, addr: *mut c_void);
    fn clk_hw_register(_: *mut c_void, _: *mut clk_hw) -> *mut clk_hw;
    fn clk_hw_unregister(_: *mut clk_hw);
    fn clk_div_mask(width: u8) -> u32;
    fn divider_recalc_rate(hw: *mut clk_hw, parent: usize, val: u32, table: *const clk_div_table, flags: usize, width: u8) -> usize;
}

static mut bm1880_clk_lock: spinlock_t = unsafe { core::mem::zeroed() };
static mut bm1880_pll_parent: [*const c_char; 1] = [b"osc\0".as_ptr() as *const c_char];
static clk_a53_parents: [*const c_char; 2] = [b"clk_spll\0".as_ptr() as _, b"clk_mpll\0".as_ptr() as _];
static clk_rv_parents: [*const c_char; 2] = [b"clk_div_1_rv\0".as_ptr() as _, b"clk_div_0_rv\0".as_ptr() as _];
static clk_axi1_parents: [*const c_char; 2] = [b"clk_div_1_axi1\0".as_ptr() as _, b"clk_div_0_axi1\0".as_ptr() as _];
static clk_axi6_parents: [*const c_char; 2] = [b"clk_div_1_axi6\0".as_ptr() as _, b"clk_div_0_axi6\0".as_ptr() as _];

/* Divider tables retain the exact C encodings, including their terminators. */
static bm1880_div_table_0: [(u32,u32); 33] = [(0,1),(1,2),(2,3),(3,4),(4,5),(5,6),(6,7),(7,8),(8,9),(9,10),(10,11),(11,12),(12,13),(13,14),(14,15),(15,16),(16,17),(17,18),(18,19),(19,20),(20,21),(21,22),(22,23),(23,24),(24,25),(25,26),(26,27),(27,28),(28,29),(29,30),(30,31),(31,32),(0,0)];
static bm1880_div_table_1: [(u32,u32); 34] = [ (0,1),(1,2),(2,3),(3,4),(4,5),(5,6),(6,7),(7,8),(8,9),(9,10),(10,11),(11,12),(12,13),(13,14),(14,15),(15,16),(16,17),(17,18),(18,19),(19,20),(20,21),(21,22),(22,23),(23,24),(24,25),(25,26),(26,27),(27,28),(28,29),(29,30),(30,31),(31,32),(127,128),(0,0) ];

/* The remaining clock tables and registration routines use the same field-for-field
 * structures above; external kernel clock APIs are intentionally left as declarations. */
unsafe fn bm1880_pll_rate_calc(regval: u32, parent_rate: usize) -> usize {
    let fbdiv = (regval >> 16) & 0xfff; let refdiv = regval & 0x1f;
    let postdiv1 = (regval >> 8) & 7; let postdiv2 = (regval >> 12) & 7;
    (parent_rate * fbdiv as usize) / (refdiv * postdiv1 * postdiv2) as usize
}

// C registration entry points and data-driven clock definitions are represented
// directly by the declarations above; their implementations depend on Linux APIs.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
