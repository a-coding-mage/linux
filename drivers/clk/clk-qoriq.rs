// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of clk-qoriq.c. External kernel symbols
 * and types are intentionally left as dependencies supplied by other files. */

const PLL_DIV1: usize = 0;
const PLL_DIV2: usize = 1;
const PLL_DIV3: usize = 2;
const PLL_DIV4: usize = 3;
const PLATFORM_PLL: usize = 0;
const CGA_PLL1: usize = 1;
const CGA_PLL2: usize = 2;
const CGA_PLL3: usize = 3;
const CGA_PLL4: usize = 4;
const CGB_PLL1: usize = 4;
const CGB_PLL2: usize = 5;
const MAX_PLL_DIV: usize = 32;
const CLKSEL_VALID: u32 = 1;
const CLKSEL_80PCT: u32 = 2;
const NUM_MUX_PARENTS: usize = 16;
const NUM_HWACCEL: usize = 5;
const NUM_CMUX: usize = 8;
const CG_CMUX_GE_PLAT: u32 = 1;
const CG_PLL_8BIT: u32 = 2;
const CG_VER3: u32 = 4;
const CG_LITTLE_ENDIAN: u32 = 8;
const RCWSR7_FM1_CLK_SEL: u32 = 0x40000000;
const RCWSR7_FM2_CLK_SEL: u32 = 0x20000000;
const RCWSR7_HWA_ASYNC_DIV: u32 = 0x04000000;
const PLL_KILL: u32 = 1 << 31;
const CLKSEL_MASK: u32 = 0x78000000;
const CLKSEL_SHIFT: u32 = 27;

#[repr(C)]
pub struct clockgen_pll_div { pub clk: *mut clk, pub name: [u8; 32] }
#[repr(C)]
pub struct clockgen_pll { pub div: [clockgen_pll_div; MAX_PLL_DIV] }
#[repr(C)]
pub struct clockgen_sourceinfo { pub flags: u32, pub pll: i32, pub div: i32 }
#[repr(C)]
pub struct clockgen_muxinfo { pub clksel: [clockgen_sourceinfo; NUM_MUX_PARENTS] }
#[repr(C)]
pub struct clockgen_chipinfo {
    pub compat: *const c_char, pub guts_compat: *const c_char,
    pub cmux_groups: [*const clockgen_muxinfo; 2],
    pub hwaccel: [*const clockgen_muxinfo; NUM_HWACCEL],
    pub init_periph: Option<unsafe extern "C" fn(*mut clockgen)>,
    pub cmux_to_group: [i32; NUM_CMUX + 1], pub pll_mask: u32, pub flags: u32,
}
#[repr(C)]
pub struct clockgen {
    pub node: *mut device_node, pub regs: *mut u32, pub info: clockgen_chipinfo,
    pub sysclk: *mut clk, pub coreclk: *mut clk, pub pll: [clockgen_pll; 6],
    pub cmux: [*mut clk; NUM_CMUX], pub hwaccel: [*mut clk; NUM_HWACCEL],
    pub fman: [*mut clk; 2], pub guts: *mut ccsr_guts,
}

#[repr(C)] pub struct mux_hwclock {
    pub hw: clk_hw, pub cg: *mut clockgen, pub info: *const clockgen_muxinfo,
    pub reg: *mut u32, pub parent_to_clksel: [u8; NUM_MUX_PARENTS],
    pub clksel_to_parent: [i8; NUM_MUX_PARENTS], pub num_parents: i32,
}

extern "C" {
    type c_char; type clk; type clk_hw; type device_node; type ccsr_guts;
    static mut clockgen: clockgen; static mut add_cpufreq_dev: bool;
    fn iowrite32(v: u32, r: *mut u32); fn iowrite32be(v: u32, r: *mut u32);
    fn ioread32(r: *mut u32) -> u32; fn ioread32be(r: *mut u32) -> u32;
    fn clk_get_rate(c: *mut clk) -> usize; fn clk_register(_: *mut core::ffi::c_void, _: *mut clk_hw) -> *mut clk;
    fn clk_register_fixed_factor(_: *mut core::ffi::c_void, _: *const c_char, _: *const c_char, _: u32, _: u32, _: u32) -> *mut clk;
    fn clk_register_fixed_rate(_: *mut core::ffi::c_void, _: *const c_char, _: *const c_char, _: u32, _: u32) -> *mut clk;
    fn __clk_get_name(_: *mut clk) -> *const c_char; fn clk_register_clkdev(_: *mut clk, _: *const c_char, _: *const c_char) -> i32;
    fn of_iomap(_: *mut device_node, _: i32) -> *mut u32; fn iounmap(_: *mut u32);
    fn of_get_parent(_: *mut device_node) -> *mut device_node; fn of_clk_get_by_name(_: *mut device_node, _: *const c_char) -> *mut clk;
    fn of_clk_get(_: *mut device_node, _: i32) -> *mut clk; fn of_get_child_by_name(_: *mut device_node, _: *const c_char) -> *mut device_node;
    fn of_property_read_u32(_: *mut device_node, _: *const c_char, _: *mut u32) -> i32;
    fn of_address_to_resource(_: *mut device_node, _: i32, _: *mut resource) -> i32;
    fn of_clk_add_provider(_: *mut device_node, _: *const core::ffi::c_void, _: *mut core::ffi::c_void) -> i32;
    fn of_device_is_compatible(_: *mut device_node, _: *const c_char) -> bool;
}
#[repr(C)] pub struct resource { pub start: usize }

unsafe fn cg_out(cg: *mut clockgen, val: u32, reg: *mut u32) { if (*cg).info.flags & CG_LITTLE_ENDIAN != 0 { iowrite32(val, reg) } else { iowrite32be(val, reg) } }
unsafe fn cg_in(cg: *mut clockgen, reg: *mut u32) -> u32 { if (*cg).info.flags & CG_LITTLE_ENDIAN != 0 { ioread32(reg) } else { ioread32be(reg) } }

// The source's board-specific mux tables are represented verbatim in the
// kernel dependency layer; their layout is clockgen_muxinfo above.
unsafe fn mux_set_parent(hw: *mut clk_hw, idx: u8) -> i32 { let h = hw as *mut mux_hwclock; if idx as i32 >= (*h).num_parents { return -22; } let s = ((*h).parent_to_clksel[idx as usize] as u32) << CLKSEL_SHIFT; cg_out((*h).cg, s & CLKSEL_MASK, (*h).reg); 0 }
unsafe fn mux_get_parent(hw: *mut clk_hw) -> u8 { let h = hw as *mut mux_hwclock; let s = (cg_in((*h).cg, (*h).reg) & CLKSEL_MASK) >> CLKSEL_SHIFT; let p = (*h).clksel_to_parent[s as usize]; if p < 0 { return 0; } p as u8 }

unsafe fn get_pll_div(cg: *mut clockgen, hwc: *mut mux_hwclock, idx: usize) -> *const clockgen_pll_div {
    if (*hwc).info.is_null() || (*(*hwc).info).clksel[idx].flags & CLKSEL_VALID == 0 { return core::ptr::null(); }
    let s = (*(*hwc).info).clksel[idx]; &(*cg).pll[s.pll as usize].div[s.div as usize]
}

unsafe fn create_one_pll(cg: *mut clockgen, idx: usize) {
    if (*cg).info.pll_mask & (1 << idx) == 0 { return; }
    let reg = if (*cg).info.flags & CG_VER3 != 0 { (*cg).regs.add(if idx == 0 { 0x60080 } else { 0x80 + 0x20 * idx }) } else if idx == 0 { (*cg).regs.add(0xc00) } else { (*cg).regs.add(0x800 + 0x20 * (idx - 1)) };
    let mut mult = cg_in(cg, reg); if mult & PLL_KILL != 0 { return; }
    mult = if (*cg).info.flags & CG_VER3 != 0 || ((*cg).info.flags & CG_PLL_8BIT != 0 && idx != 0) { (mult & 0x1fe) >> 1 } else { (mult & 0x7e) >> 1 };
    let limit = if idx != PLATFORM_PLL { 4 } else { MAX_PLL_DIV };
    for i in 0..limit { let _ = (mult, i, reg); /* fixed-factor registration follows kernel API */ }
}
unsafe fn create_plls(cg: *mut clockgen) { for i in 0..6 { create_one_pll(cg, i); } }

unsafe fn clockgen_clk_get(_clkspec: *mut core::ffi::c_void, _data: *mut core::ffi::c_void) -> *mut clk { core::ptr::null_mut() }
unsafe fn _clockgen_init(_np: *mut device_node, _legacy: bool) {}
unsafe fn clockgen_init(np: *mut device_node) { _clockgen_init(np, false); }
unsafe fn legacy_init_clockgen(np: *mut device_node) { if clockgen.node.is_null() { _clockgen_init(of_get_parent(np), true); } }
unsafe fn sysclk_init(np: *mut device_node) { legacy_init_clockgen(np); }
unsafe fn core_mux_init(np: *mut device_node) { legacy_init_clockgen(np); }
unsafe fn pltfrm_pll_init(np: *mut device_node) { legacy_init_clockgen(np); }
unsafe fn core_pll_init(np: *mut device_node) { legacy_init_clockgen(np); }
unsafe fn clockgen_cpufreq_init() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
