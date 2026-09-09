// SPDX-License-Identifier: GPL-2.0
/* Driver for Silicon Labs Si544/Si549 Programmable Oscillator */

// Linux dependencies supplied externally: clk-provider, delay, math64, module,
// i2c, regmap, and slab APIs.

const SI544_REG_CONTROL: u32 = 7;
const SI544_REG_OE_STATE: u32 = 17;
const SI544_REG_HS_DIV: u32 = 23;
const SI544_REG_LS_HS_DIV: u32 = 24;
const SI544_REG_FBDIV0: u32 = 26;
const SI544_REG_FBDIV8: u32 = 27;
const SI544_REG_FBDIV16: u32 = 28;
const SI544_REG_FBDIV24: u32 = 29;
const SI544_REG_FBDIV32: u32 = 30;
const SI544_REG_FBDIV40: u32 = 31;
const SI544_REG_FCAL_OVR: u32 = 69;
const SI544_REG_ADPLL_DELTA_M0: u32 = 231;
const SI544_REG_ADPLL_DELTA_M8: u32 = 232;
const SI544_REG_ADPLL_DELTA_M16: u32 = 233;
const SI544_REG_PAGE_SELECT: u32 = 255;
const SI544_CONTROL_RESET: u32 = 1 << 7;
const SI544_CONTROL_MS_ICAL2: u32 = 1 << 3;
const SI544_OE_STATE_ODC_OE: u32 = 1;
const SI544_MIN_FREQ: u32 = 200000;
const SI544_XO_FREQ: u32 = 55050000;
const SI549_XO_FREQ: u32 = 152600000;
const FVCO_MIN: u64 = 10800000000;
const HS_DIV_MAX: u32 = 2046;
const HS_DIV_MAX_ODD: u32 = 33;
const MIN_HSDIV_FREQ: u64 = FVCO_MIN / HS_DIV_MAX as u64;
const DELTA_M_MAX: i64 = 8161512;
const DELTA_M_FRAC_NUM: i64 = 19;
const DELTA_M_FRAC_DEN: i64 = 20000;

#[repr(C)]
pub struct si544_clk_desc { pub max_freq: usize, pub xo_freq: usize }

#[repr(C)]
pub struct clk_si544 {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub i2c_client: *mut i2c_client,
    pub chip_info: *const si544_clk_desc,
}

#[repr(C)]
pub struct clk_si544_muldiv {
    pub fb_div_frac: u32,
    pub fb_div_int: u16,
    pub hs_div: u16,
    pub ls_div_bits: u8,
    pub delta_m: i32,
    pub xo_freq: u32,
}

unsafe fn si544_enable_output(data: *mut clk_si544, enable: bool) -> i32 {
    regmap_update_bits((*data).regmap, SI544_REG_OE_STATE, SI544_OE_STATE_ODC_OE,
        if enable { SI544_OE_STATE_ODC_OE } else { 0 })
}

unsafe fn si544_prepare(hw: *mut clk_hw) -> i32 {
    si544_enable_output(container_of!(hw, clk_si544, hw), true)
}
unsafe fn si544_unprepare(hw: *mut clk_hw) {
    si544_enable_output(container_of!(hw, clk_si544, hw), false);
}
unsafe fn si544_is_prepared(hw: *mut clk_hw) -> i32 {
    let data = container_of!(hw, clk_si544, hw); let mut val = 0u32;
    let err = regmap_read((*data).regmap, SI544_REG_OE_STATE, &mut val);
    if err < 0 { return err; } (val & SI544_OE_STATE_ODC_OE != 0) as i32
}

unsafe fn si544_get_muldiv(data: *mut clk_si544, settings: *mut clk_si544_muldiv) -> i32 {
    let mut err; let mut reg = [0u8; 6];
    err = regmap_bulk_read((*data).regmap, SI544_REG_HS_DIV, reg.as_mut_ptr(), 2); if err != 0 { return err; }
    (*settings).ls_div_bits = (reg[1] >> 4) & 7;
    (*settings).hs_div = (((reg[1] & 7) as u16) << 8) | reg[0] as u16;
    err = regmap_bulk_read((*data).regmap, SI544_REG_FBDIV0, reg.as_mut_ptr(), 6); if err != 0 { return err; }
    (*settings).fb_div_int = reg[4] as u16 | ((reg[5] & 7) as u16) << 8;
    (*settings).fb_div_frac = reg[0] as u32 | (reg[1] as u32) << 8 | (reg[2] as u32) << 16 | (reg[3] as u32) << 24;
    err = regmap_bulk_read((*data).regmap, SI544_REG_ADPLL_DELTA_M0, reg.as_mut_ptr(), 3); if err != 0 { return err; }
    (*settings).delta_m = ((reg[0] as i32) << 8 | (reg[1] as i32) << 16 | (reg[2] as i32) << 24) >> 8;
    (*settings).xo_freq = (*(*data).chip_info).xo_freq as u32; 0
}

unsafe fn si544_set_delta_m(data: *mut clk_si544, delta_m: i32) -> i32 {
    let reg = [delta_m as u8, (delta_m >> 8) as u8, (delta_m >> 16) as u8];
    regmap_bulk_write((*data).regmap, SI544_REG_ADPLL_DELTA_M0, reg.as_ptr(), 3)
}
unsafe fn si544_set_muldiv(data: *mut clk_si544, settings: *mut clk_si544_muldiv) -> i32 {
    let mut reg = [0u8; 6];
    reg[0] = (*settings).hs_div as u8; reg[1] = ((*settings).hs_div >> 8) as u8 | (*settings).ls_div_bits << 4;
    let err = regmap_bulk_write((*data).regmap, SI544_REG_HS_DIV, reg.as_ptr(), 2); if err < 0 { return err; }
    reg[0] = (*settings).fb_div_frac as u8; reg[1] = ((*settings).fb_div_frac >> 8) as u8; reg[2] = ((*settings).fb_div_frac >> 16) as u8; reg[3] = ((*settings).fb_div_frac >> 24) as u8;
    reg[4] = (*settings).fb_div_int as u8; reg[5] = ((*settings).fb_div_int >> 8) as u8;
    regmap_bulk_write((*data).regmap, SI544_REG_FBDIV0, reg.as_ptr(), 6)
}
unsafe fn is_valid_frequency(data: *const clk_si544, frequency: usize) -> bool { frequency >= SI544_MIN_FREQ as usize && frequency <= (*(*data).chip_info).max_freq }

unsafe fn si544_calc_muldiv(settings: *mut clk_si544_muldiv, frequency: usize) -> i32 {
    let fxo = (*settings).xo_freq as u64; let mut ls_freq = frequency as u64; (*settings).ls_div_bits = 0;
    if (frequency as u64) < MIN_HSDIV_FREQ { let mut res = 1u8; let mut tmp = 2 * HS_DIV_MAX; while tmp <= HS_DIV_MAX * 32 { if (frequency as u64) * tmp as u64 >= FVCO_MIN { break; } res += 1; tmp <<= 1; } (*settings).ls_div_bits = res; ls_freq <<= res; }
    let mut vco = (FVCO_MIN + ls_freq - 1) / ls_freq; (*settings).hs_div = vco as u16;
    if (*settings).hs_div & 1 != 0 && ((*settings).hs_div as u32 > HS_DIV_MAX_ODD || (*settings).ls_div_bits != 0) { (*settings).hs_div += 1; }
    vco = ls_freq * (*settings).hs_div as u64; let int = vco / fxo; let rem = vco % fxo; (*settings).fb_div_int = int as u16;
    (*settings).fb_div_frac = ((rem << 32) + fxo / 2) / fxo as u64 as u32; (*settings).delta_m = 0; 0
}
unsafe fn si544_calc_center_rate(s: *const clk_si544_muldiv) -> usize {
    let d = (*s).hs_div as u64 * (1u64 << (*s).ls_div_bits); let fxo = (*s).xo_freq as u64;
    let vco = (((*s).fb_div_frac as u64 * fxo + fxo / 2) >> 32) + (*s).fb_div_int as u64 * fxo;
    (vco / d) as usize
}
unsafe fn si544_calc_rate(s: *const clk_si544_muldiv) -> usize {
    let rate = si544_calc_center_rate(s) as i64; let mut delta = rate * (DELTA_M_FRAC_NUM * (*s).delta_m as i64);
    let half = DELTA_M_MAX * DELTA_M_FRAC_DEN / 2; if (*s).delta_m < 0 { delta -= half; } else { delta += half; }
    (rate + delta / (DELTA_M_MAX * DELTA_M_FRAC_DEN)) as usize
}

unsafe fn si544_recalc_rate(hw: *mut clk_hw, _parent_rate: usize) -> usize {
    let data = container_of!(hw, clk_si544, hw); let mut s = core::mem::zeroed();
    if si544_get_muldiv(data, &mut s) != 0 { return 0; } si544_calc_rate(&s)
}
unsafe fn si544_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let data = container_of!(hw, clk_si544, hw); if !is_valid_frequency(data, (*req).rate) { return -22; } 0
}
unsafe fn si544_max_delta(rate: usize) -> usize { rate * DELTA_M_FRAC_NUM as usize / DELTA_M_FRAC_DEN as usize }
fn si544_calc_delta(delta: i32, max_delta: i32) -> i32 { ((delta as i64 * DELTA_M_MAX) / max_delta as i64) as i32 }
unsafe fn si544_set_rate(hw: *mut clk_hw, rate: usize, _parent_rate: usize) -> i32 {
    let data = container_of!(hw, clk_si544, hw); if !is_valid_frequency(data, rate) { return -22; }
    let mut s: clk_si544_muldiv = core::mem::zeroed(); let mut err = si544_get_muldiv(data, &mut s); if err != 0 { return err; }
    let center = si544_calc_center_rate(&s); let max_delta = si544_max_delta(center); let delta = rate as isize - center as isize;
    if delta.unsigned_abs() <= max_delta { return si544_set_delta_m(data, si544_calc_delta(delta as i32, max_delta as i32)); }
    err = si544_calc_muldiv(&mut s, rate); if err != 0 { return err; }
    let mut old = 0u32; err = regmap_read((*data).regmap, SI544_REG_OE_STATE, &mut old); if err != 0 { return err; }
    si544_enable_output(data, false); err = regmap_write((*data).regmap, SI544_REG_FCAL_OVR, 0); if err < 0 { return err; }
    err = si544_set_delta_m(data, s.delta_m); if err < 0 { return err; }
    err = si544_set_muldiv(data, &mut s); if err < 0 { return err; }
    err = regmap_write((*data).regmap, SI544_REG_CONTROL, SI544_CONTROL_MS_ICAL2); if err < 0 { return err; }
    usleep_range(10000, 12000); if old & SI544_OE_STATE_ODC_OE != 0 { si544_enable_output(data, true); } err
}

unsafe fn si544_regmap_is_volatile(_dev: *mut device, reg: u32) -> bool { reg == SI544_REG_CONTROL || reg == SI544_REG_FCAL_OVR }

static clk_si544a_info: si544_clk_desc = si544_clk_desc { xo_freq: SI544_XO_FREQ as usize, max_freq: 1500000000 };
static clk_si544b_info: si544_clk_desc = si544_clk_desc { xo_freq: SI544_XO_FREQ as usize, max_freq: 800000000 };
static clk_si544c_info: si544_clk_desc = si544_clk_desc { xo_freq: SI544_XO_FREQ as usize, max_freq: 325000000 };
static clk_si549a_info: si544_clk_desc = si544_clk_desc { xo_freq: SI549_XO_FREQ as usize, max_freq: 1500000000 };
static clk_si549b_info: si544_clk_desc = si544_clk_desc { xo_freq: SI549_XO_FREQ as usize, max_freq: 800000000 };
static clk_si549c_info: si544_clk_desc = si544_clk_desc { xo_freq: SI549_XO_FREQ as usize, max_freq: 325000000 };

#[repr(C)]
struct i2c_device_id { name: *const u8, driver_data: usize }
#[repr(C)]
struct of_device_id { compatible: *const u8, data: *const si544_clk_desc }

static si544_id: &[i2c_device_id] = &[
    i2c_device_id { name: b"si544a\0".as_ptr(), driver_data: &clk_si544a_info as *const _ as usize },
    i2c_device_id { name: b"si544b\0".as_ptr(), driver_data: &clk_si544b_info as *const _ as usize },
    i2c_device_id { name: b"si544c\0".as_ptr(), driver_data: &clk_si544c_info as *const _ as usize },
    i2c_device_id { name: b"si549a\0".as_ptr(), driver_data: &clk_si549a_info as *const _ as usize },
    i2c_device_id { name: b"si549b\0".as_ptr(), driver_data: &clk_si549b_info as *const _ as usize },
    i2c_device_id { name: b"si549c\0".as_ptr(), driver_data: &clk_si549c_info as *const _ as usize },
];
static clk_si544_of_match: &[of_device_id] = &[
    of_device_id { compatible: b"silabs,si544a\0".as_ptr(), data: &clk_si544a_info },
    of_device_id { compatible: b"silabs,si544b\0".as_ptr(), data: &clk_si544b_info },
    of_device_id { compatible: b"silabs,si544c\0".as_ptr(), data: &clk_si544c_info },
    of_device_id { compatible: b"silabs,si549a\0".as_ptr(), data: &clk_si549a_info },
    of_device_id { compatible: b"silabs,si549b\0".as_ptr(), data: &clk_si549b_info },
    of_device_id { compatible: b"silabs,si549c\0".as_ptr(), data: &clk_si549c_info },
];

// Equivalent external driver registration: .name = "si544", .probe = si544_probe,
// .id_table = si544_id, and .of_match_table = clk_si544_of_match.

// External kernel ABI types and registration macros are supplied by other files.
extern "C" {
    fn si544_probe(client: *mut i2c_client) -> i32;
    fn usleep_range(min: u32, max: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
