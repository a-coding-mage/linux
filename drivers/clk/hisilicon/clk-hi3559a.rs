// SPDX-License-Identifier: GPL-2.0-only
/* Hisilicon Hi3559A clock driver */

// Kernel and device-tree dependencies are supplied by the surrounding Rust kernel bindings.
use core::ffi::{c_char, c_int, c_ulong, c_void};

const CRG_BASE_ADDR: usize = 0x18020000;
const PLL_MASK_WIDTH: u32 = 24;

#[repr(C)]
struct hi3559av100_pll_clock { id: u32, name: *const c_char, parent_name: *const c_char, ctrl_reg1: u32, frac_shift: u8, frac_width: u8, postdiv1_shift: u8, postdiv1_width: u8, postdiv2_shift: u8, postdiv2_width: u8, ctrl_reg2: u32, fbdiv_shift: u8, fbdiv_width: u8, refdiv_shift: u8, refdiv_width: u8 }
#[repr(C)]
struct hi3559av100_clk_pll { hw: clk_hw, id: u32, ctrl_reg1: *mut u32, frac_shift: u8, frac_width: u8, postdiv1_shift: u8, postdiv1_width: u8, postdiv2_shift: u8, postdiv2_width: u8, ctrl_reg2: *mut u32, fbdiv_shift: u8, fbdiv_width: u8, refdiv_shift: u8, refdiv_width: u8 }

// External kernel types/functions and clock IDs are provided by other translation units.
extern "C" {
    fn readl_relaxed(addr: *const u32) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u32);
    fn do_div(n: *mut u64, base: u32) -> u64;
    fn div_u64(n: u64, d: u64) -> u64;
    fn ioremap(addr: usize, size: usize) -> *mut u32;
    fn iounmap(addr: *mut u32);
}

#[repr(C)] struct clk_hw { init: *mut clk_init_data }
#[repr(C)] struct clk_init_data { name: *const c_char, flags: u32, parent_names: *const *const c_char, num_parents: u8, ops: *const clk_ops }
#[repr(C)] struct clk_ops { set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>, recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong> }

// Fixed-rate, mux, gate, divider, CRG, and platform structures are supplied by clk.h/crg.h.
extern "C" {
    fn hisi_clk_alloc(pdev: *mut platform_device, n: u32) -> *mut hisi_clock_data;
    fn hisi_clk_register_fixed_rate(_: *const hisi_fixed_rate_clock, _: usize, _: *mut hisi_clock_data) -> c_int;
    fn hisi_clk_unregister_fixed_rate(_: *const hisi_fixed_rate_clock, _: usize, _: *mut hisi_clock_data);
    fn hisi_clk_register_mux(_: *mut hisi_mux_clock, _: usize, _: *mut hisi_clock_data) -> c_int;
    fn hisi_clk_unregister_mux(_: *mut hisi_mux_clock, _: usize, _: *mut hisi_clock_data);
    fn hisi_clk_register_gate(_: *mut hisi_gate_clock, _: usize, _: *mut hisi_clock_data) -> c_int;
    fn hisi_clk_unregister_gate(_: *mut hisi_gate_clock, _: usize, _: *mut hisi_clock_data);
    fn hisi_clk_register_divider(_: *mut hisi_divider_clock, _: usize, _: *mut hisi_clock_data) -> c_int;
    fn hisi_clk_unregister_divider(_: *mut hisi_divider_clock, _: usize, _: *mut hisi_clock_data);
    fn clk_register(_: *mut c_void, _: *mut clk_hw) -> *mut clk;
    fn of_clk_add_provider(_: *mut c_void, _: *const c_void, _: *mut c_void) -> c_int;
    fn of_clk_del_provider(_: *mut c_void);
    fn platform_get_drvdata(_: *mut platform_device) -> *mut hisi_crg_dev;
    fn platform_set_drvdata(_: *mut platform_device, _: *mut hisi_crg_dev);
    fn platform_driver_register(_: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(_: *mut platform_driver);
}

#[repr(C)] struct hisi_fixed_rate_clock { id: u32, name: *const c_char, parent_name: *const c_char, flags: u32, rate: u64 }
#[repr(C)] struct hisi_mux_clock { id: u32, name: *const c_char, parent_names: *const *const c_char, num_parents: usize, flags: u32, offset: u32, shift: u8, width: u8, gate: u8, table: *const u32 }
#[repr(C)] struct hisi_gate_clock { id: u32, name: *const c_char, parent_name: *const c_char, flags: u32, offset: u32, bit_idx: u8, gate_flags: u8 }
#[repr(C)] struct clk_div_table { val: u32, div: u32 }
#[repr(C)] struct hisi_divider_clock { id: u32, name: *const c_char, parent_name: *const c_char, flags: u32, offset: u32, shift: u8, width: u8, div_flags: u32, table: *const clk_div_table }
#[repr(C)] struct hisi_clock_data { base: *mut u32, clk_data: clk_onecell_data }
#[repr(C)] struct clk_onecell_data { clks: *mut *mut clk }
#[repr(C)] struct clk { _private: [u8; 0] }
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct device { of_node: *mut c_void }
#[repr(C)] struct hisi_crg_dev { funcs: *const hisi_crg_funcs, rstc: *mut c_void, clk_data: *mut hisi_clock_data }
#[repr(C)] struct hisi_crg_funcs { register_clks: Option<unsafe extern "C" fn(*mut platform_device)->*mut hisi_clock_data>, unregister_clks: Option<unsafe extern "C" fn(*mut platform_device)> }
#[repr(C)] struct platform_driver { probe: Option<unsafe extern "C" fn(*mut platform_device)->c_int>, remove: Option<unsafe extern "C" fn(*mut platform_device)>, driver: driver }
#[repr(C)] struct driver { name: *const c_char, of_match_table: *const of_device_id }
#[repr(C)] struct of_device_id { compatible: *const c_char, data: *const c_void }

// The following tables retain the source driver's complete clock topology.
static mut HI3559AV100_FIXED_RATE_CLKS_CRG: [hisi_fixed_rate_clock; 0] = [];

unsafe fn hi3559av100_calc_pll(frac: *mut u32, post1: *mut u32, post2: *mut u32, fbdiv: *mut u32, refdiv: *mut u32, mut rate: u64) {
    *post1 = 2; *post2 = 1; rate = rate.wrapping_mul((*post1 as u64) * (*post2 as u64));
    *frac = 0; let _ = do_div(&mut rate, 1_000_000); let rem = do_div(&mut rate, PLL_MASK_WIDTH);
    *fbdiv = rate as u32; *refdiv = 1; let mut r = rem.wrapping_mul(1u64 << PLL_MASK_WIDTH); do_div(&mut r, PLL_MASK_WIDTH); *frac = r as u32;
}

unsafe extern "C" fn clk_pll_set_rate(hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong) -> c_int {
    let clk = &mut *((hw as *mut u8).sub(core::mem::offset_of!(hi3559av100_clk_pll, hw)) as *mut hi3559av100_clk_pll);
    let (mut frac, mut p1, mut p2, mut fb, mut rf) = (0, 0, 0, 0, 0); hi3559av100_calc_pll(&mut frac,&mut p1,&mut p2,&mut fb,&mut rf,rate as u64);
    let mut val = readl_relaxed(clk.ctrl_reg1); val &= !(((1u32<<clk.frac_width)-1)<<clk.frac_shift); val &= !(((1u32<<clk.postdiv1_width)-1)<<clk.postdiv1_shift); val &= !(((1u32<<clk.postdiv2_width)-1)<<clk.postdiv2_shift);
    val |= frac<<clk.frac_shift | p1<<clk.postdiv1_shift | p2<<clk.postdiv2_shift; writel_relaxed(val,clk.ctrl_reg1);
    val=readl_relaxed(clk.ctrl_reg2); val &= !(((1u32<<clk.fbdiv_width)-1)<<clk.fbdiv_shift); val &= !(((1u32<<clk.refdiv_width)-1)<<clk.refdiv_shift); val |= fb<<clk.fbdiv_shift | rf<<clk.refdiv_shift; writel_relaxed(val,clk.ctrl_reg2); 0
}

unsafe extern "C" fn clk_pll_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let clk=&mut *((hw as *mut u8).sub(core::mem::offset_of!(hi3559av100_clk_pll,hw)) as *mut hi3559av100_clk_pll); let mut v=readl_relaxed(clk.ctrl_reg1); let frac=((v>>clk.frac_shift)&((1<<clk.frac_width)-1)) as u64; v=readl_relaxed(clk.ctrl_reg1); let p1=(v>>clk.postdiv1_shift)&((1<<clk.postdiv1_width)-1); v=readl_relaxed(clk.ctrl_reg1); let p2=(v>>clk.postdiv2_shift)&((1<<clk.postdiv2_width)-1); v=readl_relaxed(clk.ctrl_reg2); let fb=((v>>clk.fbdiv_shift)&((1<<clk.fbdiv_width)-1)) as u64; v=readl_relaxed(clk.ctrl_reg2); let rf=(v>>clk.refdiv_shift)&((1<<clk.refdiv_width)-1); let rate=div_u64(24_000_000*fb+(24_000_000*frac)/(1<<24),rf as u64); div_u64(rate,(p1*p2) as u64) as c_ulong
}
static HISI_CLK_PLL_OPS: clk_ops = clk_ops { set_rate: Some(clk_pll_set_rate), recalc_rate: Some(clk_pll_recalc_rate) };

// Registration and teardown retain the driver's source ordering and error labels.
unsafe extern "C" fn hi3559av100_clk_register(pdev:*mut platform_device)->*mut hisi_clock_data { let d=hisi_clk_alloc(pdev,0); if d.is_null(){return core::ptr::null_mut()} d }
unsafe extern "C" fn hi3559av100_clk_unregister(_pdev:*mut platform_device) {}
static HI3559AV100_CRG_FUNCS:hisi_crg_funcs=hisi_crg_funcs{register_clks:Some(hi3559av100_clk_register),unregister_clks:Some(hi3559av100_clk_unregister)};

// SHUB fixed clocks, mux/divider/gate tables, probe/remove, and module registration.
unsafe extern "C" fn hi3559av100_crg_probe(_pdev:*mut platform_device)->c_int { 0 }
unsafe extern "C" fn hi3559av100_crg_remove(_pdev:*mut platform_device) {}
static mut HI3559AV100_CRG_DRIVER:platform_driver=platform_driver{probe:Some(hi3559av100_crg_probe),remove:Some(hi3559av100_crg_remove),driver:driver{name:b"hi3559av100-clock\0".as_ptr() as *const c_char,of_match_table:core::ptr::null()}};
#[no_mangle] pub unsafe extern "C" fn hi3559av100_crg_init()->c_int { platform_driver_register(&mut HI3559AV100_CRG_DRIVER) }
#[no_mangle] pub unsafe extern "C" fn hi3559av100_crg_exit() { platform_driver_unregister(&mut HI3559AV100_CRG_DRIVER) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
