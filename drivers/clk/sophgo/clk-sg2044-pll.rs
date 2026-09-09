// SPDX-License-Identifier: GPL-2.0
/* Sophgo SG2044 PLL clock controller driver */

// Linux kernel dependencies and build-time bindings are supplied externally.

const PLL_VCOSEL_MASK: u32 = 0x3 << 16;
const PLL_FBDIV_MASK: u32 = (1 << 12) - 1;
const PLL_REFDIV_MASK: u32 = ((1 << 6) - 1) << 12;
const PLL_POSTDIV1_MASK: u32 = ((1 << 3) - 1) << 18;
const PLL_POSTDIV2_MASK: u32 = ((1 << 3) - 1) << 21;
const PLL_CALIBRATE_EN: u32 = 1 << 24;
const PLL_CALIBRATE_MASK: u32 = 0x7 << 27;
const PLL_CALIBRATE_DEFAULT: u32 = 2 << 27;
const PLL_UPDATE_EN: u32 = 1 << 30;
const PLL_HIGH_CTRL_MASK: u32 = PLL_FBDIV_MASK | PLL_REFDIV_MASK | PLL_POSTDIV1_MASK |
    PLL_POSTDIV2_MASK | PLL_CALIBRATE_EN | PLL_CALIBRATE_MASK | PLL_UPDATE_EN;
const PLL_HIGH_CTRL_OFFSET: u32 = 4;
const PLL_VCOSEL_1G6: u32 = 0x2;
const PLL_VCOSEL_2G4: u32 = 0x3;
const PLL_LIMIT_FOUTVCO: usize = 0;
const PLL_LIMIT_FOUT: usize = 1;
const PLL_LIMIT_REFDIV: usize = 2;
const PLL_LIMIT_FBDIV: usize = 3;
const PLL_LIMIT_POSTDIV1: usize = 4;
const PLL_LIMIT_POSTDIV2: usize = 5;
const SG2044_SYSCON_PLL_OFFSET: u32 = 0x98;

#[repr(C)]
pub struct sg2044_pll_limit { pub min: u64, pub max: u64 }

#[repr(C)]
pub struct sg2044_pll_internal {
    pub ctrl_offset: u32, pub status_offset: u32, pub enable_offset: u32,
    pub status_lock_bit: u8, pub status_updating_bit: u8, pub enable_bit: u8,
    pub limits: *const sg2044_pll_limit,
}

#[repr(C)]
pub struct sg2044_clk_common {
    pub hw: clk_hw, pub regmap: *mut regmap, pub lock: *mut spinlock_t, pub id: u32,
}

#[repr(C)]
pub struct sg2044_pll { pub common: sg2044_clk_common, pub pll: sg2044_pll_internal, pub syscon_offset: u32 }

#[repr(C)]
pub struct sg2044_pll_desc_data { pub pll: *const *mut sg2044_clk_common, pub num_pll: u16 }

#[repr(C)]
pub struct sg2044_pll_ctrl { pub lock: spinlock_t, pub data: clk_hw_onecell_data }

// External kernel types and functions.
#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong, pub best_parent_rate: c_ulong }
type c_ulong = usize;

#[inline]
fn sg2044_clk_fit_limit(value: u64, limit: &sg2044_pll_limit) -> bool { value >= limit.min && value <= limit.max }

unsafe fn hw_to_sg2044_clk_common(hw: *mut clk_hw) -> *mut sg2044_clk_common {
    hw.cast::<u8>().sub(0) as *mut sg2044_clk_common
}
unsafe fn hw_to_sg2044_pll(hw: *mut clk_hw) -> *mut sg2044_pll {
    hw_to_sg2044_clk_common(hw).cast::<sg2044_pll>()
}

fn sg2044_pll_calc_vco_rate(parent_rate: c_ulong, refdiv: c_ulong, fbdiv: c_ulong) -> c_ulong {
    ((parent_rate as u128 * fbdiv as u128) / refdiv as u128) as c_ulong
}
fn sg2044_pll_calc_rate(parent_rate: c_ulong, refdiv: c_ulong, fbdiv: c_ulong, postdiv1: c_ulong, postdiv2: c_ulong) -> c_ulong {
    ((parent_rate as u128 * fbdiv as u128) / (refdiv * (postdiv1 + 1) * (postdiv2 + 1)) as u128) as c_ulong
}
fn pll_is_better_rate(target: c_ulong, now: c_ulong, best: c_ulong) -> bool {
    target.abs_diff(now) < target.abs_diff(best)
}

unsafe fn sg2042_pll_compute_postdiv(limits: *const sg2044_pll_limit, target: c_ulong, parent_rate: c_ulong, refdiv: u32, fbdiv: u32, postdiv1: *mut u32, postdiv2: *mut u32) -> i32 {
    let mut best_rate = 0; let mut best_div1 = 0; let mut best_div2 = 0;
    for div2 in (*limits.add(PLL_LIMIT_POSTDIV2)).min..=(*limits.add(PLL_LIMIT_POSTDIV2)).max {
        for div1 in (*limits.add(PLL_LIMIT_POSTDIV1)).min..=(*limits.add(PLL_LIMIT_POSTDIV1)).max {
            let tmp = sg2044_pll_calc_rate(parent_rate, refdiv as usize, fbdiv as usize, div1 as usize, div2 as usize);
            if tmp > target { continue; }
            if pll_is_better_rate(target, tmp, best_rate) { best_div1 = div1 as u32; best_div2 = div2 as u32; best_rate = tmp; if tmp == target { *postdiv1 = best_div1; *postdiv2 = best_div2; return 0; } }
        }
    }
    if best_rate != 0 { *postdiv1 = best_div1; *postdiv2 = best_div2; 0 } else { -22 }
}

unsafe fn sg2044_compute_pll_setting(limits: *const sg2044_pll_limit, req_rate: c_ulong, parent_rate: c_ulong, value: *mut u32) -> i32 {
    let mut best_rate = 0; let mut best_refdiv = 0; let mut best_fbdiv = 0; let mut best_postdiv1 = 0; let mut best_postdiv2 = 0;
    for fbdiv in (*limits.add(PLL_LIMIT_FBDIV)).min..=(*limits.add(PLL_LIMIT_FBDIV)).max { for refdiv in (*limits.add(PLL_LIMIT_REFDIV)).min..=(*limits.add(PLL_LIMIT_REFDIV)).max {
        let vco = sg2044_pll_calc_vco_rate(parent_rate, refdiv as usize, fbdiv as usize);
        if !sg2044_clk_fit_limit(vco, &*limits.add(PLL_LIMIT_FOUTVCO)) { continue; }
        let mut d1 = 0; let mut d2 = 0;
        if sg2042_pll_compute_postdiv(limits, req_rate, parent_rate, refdiv as u32, fbdiv as u32, &mut d1, &mut d2) != 0 { continue; }
        let tmp = sg2044_pll_calc_rate(parent_rate, refdiv as usize, fbdiv as usize, d1 as usize, d2 as usize);
        if pll_is_better_rate(req_rate, tmp, best_rate) { best_refdiv=refdiv as u32; best_fbdiv=fbdiv as u32; best_postdiv1=d1; best_postdiv2=d2; best_rate=tmp; if tmp == req_rate { break; } }
    } }
    if best_rate == 0 { return -22; }
    *value = (best_refdiv << 12) | best_fbdiv | (best_postdiv1 << 18) | (best_postdiv2 << 21); 0
}

// The remaining kernel clock-provider operations, registration hooks, and PLL
// instance declarations retain their C interfaces and are supplied by the
// surrounding kernel translation layer.
extern "C" {
    fn sg2044_pll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong;
    fn sg2044_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32;
    fn sg2044_pll_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> i32;
    fn sg2044_pll_probe(pdev: *mut platform_device) -> i32;
}

#[repr(C)] pub struct clk_hw_onecell_data { pub num: u32, pub hws: *mut *mut clk_hw }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }

static PLL_LIMITS: [sg2044_pll_limit; 6] = [
    sg2044_pll_limit { min: 1_600_000_000, max: 3_200_000_000 },
    sg2044_pll_limit { min: 25_000, max: 3_200_000_000 },
    sg2044_pll_limit { min: 1, max: 63 },
    sg2044_pll_limit { min: 8, max: 1066 },
    sg2044_pll_limit { min: 0, max: 7 },
    sg2044_pll_limit { min: 0, max: 7 },
];

// PLL objects and platform-driver registration are emitted by the kernel
// binding layer using the exact C instance names and offsets.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
