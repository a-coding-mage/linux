// SPDX-License-Identifier: GPL-2.0
/* StarFive JH7110 PLL Clock Generator Driver */

// External kernel types, functions, constants, and clock bindings are supplied by other files.

const JH7110_PLL_OSC_RATE: u64 = 24000000;
const JH7110_PLL0_PD_OFFSET: u32 = 0x18;
const JH7110_PLL0_DACPD_SHIFT: u32 = 24;
const JH7110_PLL0_DACPD_MASK: u32 = 1 << 24;
const JH7110_PLL0_DSMPD_SHIFT: u32 = 25;
const JH7110_PLL0_DSMPD_MASK: u32 = 1 << 25;
const JH7110_PLL0_FBDIV_OFFSET: u32 = 0x1c;
const JH7110_PLL0_FBDIV_SHIFT: u32 = 0;
const JH7110_PLL0_FBDIV_MASK: u32 = 0xfff;
const JH7110_PLL0_FRAC_OFFSET: u32 = 0x20;
const JH7110_PLL0_PREDIV_OFFSET: u32 = 0x24;
const JH7110_PLL1_PD_OFFSET: u32 = 0x24;
const JH7110_PLL1_DACPD_SHIFT: u32 = 15;
const JH7110_PLL1_DACPD_MASK: u32 = 1 << 15;
const JH7110_PLL1_DSMPD_SHIFT: u32 = 16;
const JH7110_PLL1_DSMPD_MASK: u32 = 1 << 16;
const JH7110_PLL1_FBDIV_OFFSET: u32 = 0x24;
const JH7110_PLL1_FBDIV_SHIFT: u32 = 17;
const JH7110_PLL1_FBDIV_MASK: u32 = 0xfff << 17;
const JH7110_PLL1_FRAC_OFFSET: u32 = 0x28;
const JH7110_PLL1_PREDIV_OFFSET: u32 = 0x2c;
const JH7110_PLL2_PD_OFFSET: u32 = 0x2c;
const JH7110_PLL2_DACPD_SHIFT: u32 = 15;
const JH7110_PLL2_DACPD_MASK: u32 = 1 << 15;
const JH7110_PLL2_DSMPD_SHIFT: u32 = 16;
const JH7110_PLL2_DSMPD_MASK: u32 = 1 << 16;
const JH7110_PLL2_FBDIV_OFFSET: u32 = 0x2c;
const JH7110_PLL2_FBDIV_SHIFT: u32 = 17;
const JH7110_PLL2_FBDIV_MASK: u32 = 0xfff << 17;
const JH7110_PLL2_FRAC_OFFSET: u32 = 0x30;
const JH7110_PLL2_PREDIV_OFFSET: u32 = 0x34;
const JH7110_PLL_FRAC_SHIFT: u32 = 0;
const JH7110_PLL_FRAC_MASK: u32 = 0xffffff;
const JH7110_PLL_POSTDIV1_SHIFT: u32 = 28;
const JH7110_PLL_POSTDIV1_MASK: u32 = 0x3 << 28;
const JH7110_PLL_PREDIV_SHIFT: u32 = 0;
const JH7110_PLL_PREDIV_MASK: u32 = 0x3f;

#[repr(C)]
#[derive(Copy, Clone)]
enum JH7110PllMode { Fraction = 0, Integer = 1 }

#[repr(C)]
struct Jh7110PllPreset { freq: u64, frac: u32, fbdiv: u16, prediv: u8, postdiv1: u8, mode: u8 }

#[repr(C)]
struct Jh7110PllInfo {
    name: *mut i8, presets: *const Jh7110PllPreset, npresets: u32,
    offsets: PllOffsets, masks: PllMasks, shifts: PllShifts,
}
#[repr(C)] struct PllOffsets { pd: u32, fbdiv: u32, frac: u32, prediv: u32 }
#[repr(C)] struct PllMasks { dacpd: u32, dsmpd: u32, fbdiv: u32 }
#[repr(C)] struct PllShifts { dacpd: i8, dsmpd: i8, fbdiv: i8 }
#[repr(C)] struct Jh7110PllData { hw: ClkHw, idx: u32 }
#[repr(C)] struct Jh7110PllPriv { dev: *mut Device, regmap: *mut Regmap, pll: [Jh7110PllData; JH7110_PLLCLK_END as usize] }
#[repr(C)] struct Jh7110PllRegvals { dacpd: u32, dsmpd: u32, fbdiv: u32, frac: u32, postdiv1: u32, prediv: u32 }

// Kernel-provided declarations.
extern "C" {
    static jh7110_plls: [Jh7110PllInfo; JH7110_PLLCLK_END as usize];
    fn regmap_read(r: *mut Regmap, off: u32, val: *mut u32) -> i32;
    fn regmap_update_bits(r: *mut Regmap, off: u32, mask: u32, val: u32) -> i32;
}

unsafe fn jh7110_pll_regvals_get(regmap: *mut Regmap, info: *const Jh7110PllInfo, ret: *mut Jh7110PllRegvals) {
    let mut val = 0u32;
    regmap_read(regmap, (*info).offsets.pd, &mut val);
    (*ret).dacpd = (val & (*info).masks.dacpd) >> (*info).shifts.dacpd;
    (*ret).dsmpd = (val & (*info).masks.dsmpd) >> (*info).shifts.dsmpd;
    regmap_read(regmap, (*info).offsets.fbdiv, &mut val);
    (*ret).fbdiv = (val & (*info).masks.fbdiv) >> (*info).shifts.fbdiv;
    regmap_read(regmap, (*info).offsets.frac, &mut val);
    (*ret).frac = val & JH7110_PLL_FRAC_MASK;
    (*ret).postdiv1 = (val & JH7110_PLL_POSTDIV1_MASK) >> JH7110_PLL_POSTDIV1_SHIFT;
    regmap_read(regmap, (*info).offsets.prediv, &mut val);
    (*ret).prediv = val & JH7110_PLL_PREDIV_MASK;
}

unsafe fn jh7110_pll_recalc_rate(hw: *mut ClkHw, parent_rate: u64) -> u64 {
    let pll = jh7110_pll_data_from(hw); let priv_ = jh7110_pll_priv_from(pll); let mut val = Jh7110PllRegvals { dacpd:0, dsmpd:0, fbdiv:0, frac:0, postdiv1:0, prediv:0 };
    jh7110_pll_regvals_get((*priv_).regmap, &jh7110_plls[(*pll).idx as usize], &mut val);
    let mut rate = if val.dacpd == 0 && val.dsmpd == 0 { parent_rate * val.frac as u64 / (1u64 << 24) } else if val.dacpd == 1 && val.dsmpd == 1 { 0 } else { return 0 };
    rate += parent_rate * val.fbdiv as u64; rate / ((val.prediv as u64) << val.postdiv1)
}

unsafe fn jh7110_pll_data_from(hw: *mut ClkHw) -> *mut Jh7110PllData { hw as *mut Jh7110PllData }
unsafe fn jh7110_pll_priv_from(pll: *mut Jh7110PllData) -> *mut Jh7110PllPriv { (pll as *mut u8).sub(std::mem::offset_of!(Jh7110PllPriv, pll)) as *mut Jh7110PllPriv }

unsafe fn jh7110_pll_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let pll = jh7110_pll_data_from(hw); let info = &jh7110_plls[(*pll).idx as usize];
    if (*req).best_parent_rate != JH7110_PLL_OSC_RATE { (*req).rate = jh7110_pll_recalc_rate(hw, (*req).best_parent_rate); return 0; }
    let mut selected = info.presets; for idx in 1..info.npresets { let val = &*info.presets.add(idx as usize); if (*req).rate < val.freq { break; } selected = val; }
    (*req).rate = (*selected).freq; 0
}

unsafe fn jh7110_pll_set_rate(_hw: *mut ClkHw, _rate: u64, _parent_rate: u64) -> i32 { -22 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
