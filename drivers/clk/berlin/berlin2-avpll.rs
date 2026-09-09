// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014 Marvell Technology Group Ltd.
 *
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Alexandre Belloni <alexandre.belloni@free-electrons.com>
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

/*
 * Berlin2 SoCs comprise up to two PLLs called AVPLL built upon a
 * VCO with 8 channels each, channel 8 is the odd-one-out and does
 * not provide mul/div.
 *
 * Unfortunately, its registers are not named but just numbered. To
 * get in at least some kind of structure, we split each AVPLL into
 * the VCOs and each channel into separate clock drivers.
 *
 * Also, here and there the VCO registers are a bit different with
 * respect to bit shifts. Make sure to add a comment for those.
 */
const NUM_CHANNELS: usize = 8;

const fn avpll_ctrl(x: u32) -> u32 { x * 0x4 }

const VCO_CTRL0: u32 = avpll_ctrl(0);
// BG2/BG2CDs VCO_B has an additional shift of 4 for its VCO_CTRL0 reg
const VCO_RESET: u32 = 1 << 0;
const VCO_POWERUP: u32 = 1 << 1;
const VCO_INTERPOL_SHIFT: u32 = 2;
const VCO_INTERPOL_MASK: u32 = 0xf << VCO_INTERPOL_SHIFT;
const VCO_REG1V45_SEL_SHIFT: u32 = 6;
const fn vco_reg1v45_sel(x: u32) -> u32 { x << VCO_REG1V45_SEL_SHIFT }
const VCO_REG1V45_SEL_1V40: u32 = vco_reg1v45_sel(0);
const VCO_REG1V45_SEL_1V45: u32 = vco_reg1v45_sel(1);
const VCO_REG1V45_SEL_1V50: u32 = vco_reg1v45_sel(2);
const VCO_REG1V45_SEL_1V55: u32 = vco_reg1v45_sel(3);
const VCO_REG1V45_SEL_MASK: u32 = vco_reg1v45_sel(3);
const VCO_REG0V9_SEL_SHIFT: u32 = 8;
const VCO_REG0V9_SEL_MASK: u32 = 0xf << VCO_REG0V9_SEL_SHIFT;
const VCO_VTHCAL_SHIFT: u32 = 12;
const fn vco_vthcal(x: u32) -> u32 { x << VCO_VTHCAL_SHIFT }
const VCO_VTHCAL_0V90: u32 = vco_vthcal(0);
const VCO_VTHCAL_0V95: u32 = vco_vthcal(1);
const VCO_VTHCAL_1V00: u32 = vco_vthcal(2);
const VCO_VTHCAL_1V05: u32 = vco_vthcal(3);
const VCO_VTHCAL_MASK: u32 = vco_vthcal(3);
const VCO_KVCOEXT_SHIFT: u32 = 14;
const VCO_KVCOEXT_MASK: u32 = 0x3 << VCO_KVCOEXT_SHIFT;
const VCO_KVCOEXT_ENABLE: u32 = 1 << 17;
const VCO_V2IEXT_SHIFT: u32 = 18;
const VCO_V2IEXT_MASK: u32 = 0xf << VCO_V2IEXT_SHIFT;
const VCO_V2IEXT_ENABLE: u32 = 1 << 22;
const VCO_SPEED_SHIFT: u32 = 23;
const fn vco_speed(x: u32) -> u32 { x << VCO_SPEED_SHIFT }
const VCO_SPEED_1G08_1G21: u32 = vco_speed(0);
const VCO_SPEED_1G21_1G40: u32 = vco_speed(1);
const VCO_SPEED_1G40_1G61: u32 = vco_speed(2);
const VCO_SPEED_1G61_1G86: u32 = vco_speed(3);
const VCO_SPEED_1G86_2G00: u32 = vco_speed(4);
const VCO_SPEED_2G00_2G22: u32 = vco_speed(5);
const VCO_SPEED_2G22: u32 = vco_speed(6);
const VCO_SPEED_MASK: u32 = vco_speed(0x7);
const VCO_CLKDET_ENABLE: u32 = 1 << 26;
const VCO_CTRL1: u32 = avpll_ctrl(1);
const VCO_REFDIV_SHIFT: u32 = 0;
const fn vco_refdiv(x: u32) -> u32 { x << VCO_REFDIV_SHIFT }
const VCO_REFDIV_1: u32 = vco_refdiv(0);
const VCO_REFDIV_2: u32 = vco_refdiv(1);
const VCO_REFDIV_4: u32 = vco_refdiv(2);
const VCO_REFDIV_3: u32 = vco_refdiv(3);
const VCO_REFDIV_MASK: u32 = vco_refdiv(0x3f);
const VCO_FBDIV_SHIFT: u32 = 6;
const fn vco_fbdiv(x: u32) -> u32 { x << VCO_FBDIV_SHIFT }
const VCO_FBDIV_MASK: u32 = vco_fbdiv(0xff);
const VCO_ICP_SHIFT: u32 = 14;
// PLL Charge Pump Current = 10uA * (x + 1)
const fn vco_icp(x: u32) -> u32 { x << VCO_ICP_SHIFT }
const VCO_ICP_MASK: u32 = vco_icp(0xf);
const VCO_LOAD_CAP: u32 = 1 << 18;
const VCO_CALIBRATION_START: u32 = 1 << 19;
const fn vco_freqoffsetn(x: u32) -> u32 { avpll_ctrl(3 + x) }
const VCO_FREQOFFSET_MASK: u32 = 0x7ffff;
const VCO_CTRL10: u32 = avpll_ctrl(10);
const VCO_POWERUP_CH1: u32 = 1 << 20;
const VCO_CTRL11: u32 = avpll_ctrl(11);
const VCO_CTRL12: u32 = avpll_ctrl(12);
const VCO_CTRL13: u32 = avpll_ctrl(13);
const VCO_CTRL14: u32 = avpll_ctrl(14);
const VCO_CTRL15: u32 = avpll_ctrl(15);
const fn vco_sync1n(x: u32) -> u32 { avpll_ctrl(15 + x) }
const VCO_SYNC1_MASK: u32 = 0x1ffff;
const fn vco_sync2n(x: u32) -> u32 { avpll_ctrl(23 + x) }
const VCO_SYNC2_MASK: u32 = 0x1ffff;
const VCO_CTRL30: u32 = avpll_ctrl(30);
const VCO_DPLL_CH1_ENABLE: u32 = 1 << 17;

#[repr(C)]
pub struct berlin2_avpll_vco {
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
    pub flags: u8,
}

#[repr(C)]
pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)]
pub struct clk_init_data {
    pub name: *const core::ffi::c_char,
    pub ops: *const clk_ops,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: u8,
    pub flags: usize,
}
#[repr(C)]
pub struct clk_ops {
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
}

unsafe extern "C" {
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn kzalloc(size: usize) -> *mut core::ffi::c_void;
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> i32;
}

static VCO_REFDIV: [u8; 4] = [1, 2, 4, 3];

unsafe fn vco_from_hw(hw: *mut clk_hw) -> *mut berlin2_avpll_vco {
    hw as *mut berlin2_avpll_vco
}

unsafe extern "C" fn berlin2_avpll_vco_is_enabled(hw: *mut clk_hw) -> i32 {
    let vco = &*vco_from_hw(hw);
    let mut reg = readl_relaxed(vco.base.add(VCO_CTRL0 as usize));
    if vco.flags & BERLIN2_AVPLL_BIT_QUIRK != 0 { reg >>= 4; }
    if reg & VCO_POWERUP != 0 { 1 } else { 0 }
}

unsafe extern "C" fn berlin2_avpll_vco_enable(hw: *mut clk_hw) -> i32 {
    let vco = &*vco_from_hw(hw);
    let mut reg = readl_relaxed(vco.base.add(VCO_CTRL0 as usize));
    reg |= if vco.flags & BERLIN2_AVPLL_BIT_QUIRK != 0 { VCO_POWERUP << 4 } else { VCO_POWERUP };
    writel_relaxed(reg, vco.base.add(VCO_CTRL0 as usize));
    0
}

unsafe extern "C" fn berlin2_avpll_vco_disable(hw: *mut clk_hw) {
    let vco = &*vco_from_hw(hw);
    let mut reg = readl_relaxed(vco.base.add(VCO_CTRL0 as usize));
    reg &= if vco.flags & BERLIN2_AVPLL_BIT_QUIRK != 0 { !(VCO_POWERUP << 4) } else { !VCO_POWERUP };
    writel_relaxed(reg, vco.base.add(VCO_CTRL0 as usize));
}

unsafe extern "C" fn berlin2_avpll_vco_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let vco = &*vco_from_hw(hw);
    let reg = readl_relaxed(vco.base.add(VCO_CTRL1 as usize));
    let refdiv = VCO_REFDIV[((reg & VCO_REFDIV_MASK) >> VCO_REFDIV_SHIFT) as usize] as usize;
    let fbdiv = ((reg & VCO_FBDIV_MASK) >> VCO_FBDIV_SHIFT) as usize;
    parent_rate.wrapping_mul(fbdiv) / refdiv
}

static BERLIN2_AVPLL_VCO_OPS: clk_ops = clk_ops {
    is_enabled: Some(berlin2_avpll_vco_is_enabled), enable: Some(berlin2_avpll_vco_enable),
    disable: Some(berlin2_avpll_vco_disable), recalc_rate: Some(berlin2_avpll_vco_recalc_rate),
};

pub const BERLIN2_AVPLL_BIT_QUIRK: u8 = 1 << 0;
pub const BERLIN2_AVPLL_SCRAMBLE_QUIRK: u8 = 1 << 1;

pub unsafe extern "C" fn berlin2_avpll_vco_register(
    base: *mut core::ffi::c_void, name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char, vco_flags: u8, flags: usize,
) -> i32 {
    let vco = kzalloc(core::mem::size_of::<berlin2_avpll_vco>()) as *mut berlin2_avpll_vco;
    if vco.is_null() { return -12; }
    let mut init = clk_init_data { name, ops: &BERLIN2_AVPLL_VCO_OPS, parent_names: &parent_name, num_parents: 1, flags };
    (*vco).base = base; (*vco).flags = vco_flags; (*vco).hw.init = &mut init;
    clk_hw_register(core::ptr::null_mut(), &mut (*vco).hw)
}

#[repr(C)]
pub struct berlin2_avpll_channel { pub hw: clk_hw, pub base: *mut core::ffi::c_void, pub flags: u8, pub index: u8 }

static DIV_HDMI: [u8; 4] = [1, 2, 4, 6];
static DIV_AV1: [u8; 4] = [1, 2, 5, 5];

unsafe fn channel_from_hw(hw: *mut clk_hw) -> *mut berlin2_avpll_channel { hw as *mut berlin2_avpll_channel }

unsafe extern "C" fn berlin2_avpll_channel_is_enabled(hw: *mut clk_hw) -> i32 {
    let ch = &*channel_from_hw(hw); if ch.index == 7 { return 1; }
    let reg = readl_relaxed(ch.base.add(VCO_CTRL10 as usize)) & (VCO_POWERUP_CH1 << ch.index);
    if reg != 0 { 1 } else { 0 }
}
unsafe extern "C" fn berlin2_avpll_channel_enable(hw: *mut clk_hw) -> i32 {
    let ch = &*channel_from_hw(hw); let mut reg = readl_relaxed(ch.base.add(VCO_CTRL10 as usize));
    reg |= VCO_POWERUP_CH1 << ch.index; writel_relaxed(reg, ch.base.add(VCO_CTRL10 as usize)); 0
}
unsafe extern "C" fn berlin2_avpll_channel_disable(hw: *mut clk_hw) {
    let ch = &*channel_from_hw(hw); let mut reg = readl_relaxed(ch.base.add(VCO_CTRL10 as usize));
    reg &= !(VCO_POWERUP_CH1 << ch.index); writel_relaxed(reg, ch.base.add(VCO_CTRL10 as usize));
}
unsafe extern "C" fn berlin2_avpll_channel_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let ch = &*channel_from_hw(hw); let mut reg = readl_relaxed(ch.base.add(VCO_CTRL30 as usize));
    if reg & (VCO_DPLL_CH1_ENABLE << ch.index) == 0 { return parent_rate; }
    let mut divider: usize;
    reg = readl_relaxed(ch.base.add(vco_sync1n(ch.index as u32) as usize));
    if ch.flags & BERLIN2_AVPLL_BIT_QUIRK != 0 && ch.index == 0 { reg >>= 4; }
    divider = (reg & VCO_SYNC1_MASK) as usize;
    reg = readl_relaxed(ch.base.add(vco_sync2n(ch.index as u32) as usize));
    let mut freq = (parent_rate as u64) * (reg & VCO_SYNC2_MASK) as u64;
    if ch.index == 7 { return (freq / divider as u64) as usize; }
    reg = readl_relaxed(ch.base.add(VCO_CTRL11 as usize)) >> 7; reg >>= (ch.index * 3) as u32;
    if reg & (1 << 2) != 0 { divider *= DIV_HDMI[(reg & 3) as usize] as usize; }
    if ch.index == 0 { reg = readl_relaxed(ch.base.add(VCO_CTRL11 as usize)) >> 28; }
    else { reg = readl_relaxed(ch.base.add(VCO_CTRL12 as usize)) >> ((ch.index - 1) * 3); }
    if reg & (1 << 2) != 0 { divider *= DIV_AV1[(reg & 3) as usize] as usize; }
    let div_av2: usize;
    if ch.index < 2 { reg = readl_relaxed(ch.base.add(VCO_CTRL12 as usize)) >> (18 + ch.index * 7); }
    else if ch.index < 7 { reg = readl_relaxed(ch.base.add(VCO_CTRL13 as usize)) >> ((ch.index - 2) * 7); }
    else { reg = readl_relaxed(ch.base.add(VCO_CTRL14 as usize)); }
    div_av2 = (reg & 0x7f) as usize; if div_av2 != 0 { divider *= div_av2; }
    if ch.index < 6 { reg = readl_relaxed(ch.base.add(VCO_CTRL14 as usize)) >> (7 + ch.index * 4); }
    else { reg = readl_relaxed(ch.base.add(VCO_CTRL15 as usize)); }
    let div_av3 = reg & 0xf; if div_av2 != 0 && div_av3 != 0 { freq *= 2; }
    (freq / divider as u64) as usize
}

static BERLIN2_AVPLL_CHANNEL_OPS: clk_ops = clk_ops {
    is_enabled: Some(berlin2_avpll_channel_is_enabled), enable: Some(berlin2_avpll_channel_enable),
    disable: Some(berlin2_avpll_channel_disable), recalc_rate: Some(berlin2_avpll_channel_recalc_rate),
};

// On some production SoCs, AVPLL channels are scrambled with respect to the
// channel numbering in the registers but still referenced by original numbers.
static QUIRK_INDEX: [u8; 8] = [0, 6, 5, 4, 3, 2, 1, 7];

pub unsafe extern "C" fn berlin2_avpll_channel_register(
    base: *mut core::ffi::c_void, name: *const core::ffi::c_char, index: u8,
    parent_name: *const core::ffi::c_char, ch_flags: u8, flags: usize,
) -> i32 {
    let ch = kzalloc(core::mem::size_of::<berlin2_avpll_channel>()) as *mut berlin2_avpll_channel;
    if ch.is_null() { return -12; }
    (*ch).base = base; (*ch).index = if ch_flags & BERLIN2_AVPLL_SCRAMBLE_QUIRK != 0 { QUIRK_INDEX[index as usize] } else { index };
    (*ch).flags = ch_flags;
    let mut init = clk_init_data { name, ops: &BERLIN2_AVPLL_CHANNEL_OPS, parent_names: &parent_name, num_parents: 1, flags };
    (*ch).hw.init = &mut init;
    clk_hw_register(core::ptr::null_mut(), &mut (*ch).hw)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
