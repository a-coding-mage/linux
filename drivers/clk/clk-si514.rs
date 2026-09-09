// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Silicon Labs Si514 Programmable Oscillator
 *
 * Copyright (C) 2015 Topic Embedded Products
 *
 * Author: Mike Looijmans <mike.looijmans@topic.nl>
 */

/* Dependencies supplied by the surrounding kernel translation. */

const SI514_REG_LP: u32 = 0;
const SI514_REG_M_FRAC1: u32 = 5;
const SI514_REG_M_FRAC2: u32 = 6;
const SI514_REG_M_FRAC3: u32 = 7;
const SI514_REG_M_INT_FRAC: u32 = 8;
const SI514_REG_M_INT: u32 = 9;
const SI514_REG_HS_DIV: u32 = 10;
const SI514_REG_LS_HS_DIV: u32 = 11;
const SI514_REG_OE_STATE: u32 = 14;
const SI514_REG_RESET: u32 = 128;
const SI514_REG_CONTROL: u32 = 132;

const SI514_RESET_RST: u8 = 1 << 7;
const SI514_CONTROL_FCAL: u8 = 1 << 0;
const SI514_CONTROL_OE: u8 = 1 << 2;
const SI514_MIN_FREQ: u32 = 100000;
const SI514_MAX_FREQ: u32 = 250000000;
const FXO: u32 = 31980000;
const FVCO_MIN: u32 = 2080000000;
const FVCO_MAX: u32 = 2500000000;
const HS_DIV_MAX: u32 = 1022;

#[repr(C)]
struct clk_si514 {
    hw: clk_hw,
    regmap: *mut regmap,
    i2c_client: *mut i2c_client,
}

#[inline]
unsafe fn to_clk_si514(hw: *mut clk_hw) -> *mut clk_si514 {
    container_of!(hw, clk_si514, hw)
}

#[repr(C)]
struct clk_si514_muldiv {
    m_frac: u32,
    m_int: u8,
    ls_div_bits: u8,
    hs_div: u16,
}

unsafe fn si514_enable_output(data: *mut clk_si514, enable: bool) -> i32 {
    regmap_update_bits((*data).regmap, SI514_REG_CONTROL,
        SI514_CONTROL_OE as u32, if enable { SI514_CONTROL_OE as u32 } else { 0 })
}

unsafe fn si514_prepare(hw: *mut clk_hw) -> i32 {
    si514_enable_output(to_clk_si514(hw), true)
}

unsafe fn si514_unprepare(hw: *mut clk_hw) {
    si514_enable_output(to_clk_si514(hw), false);
}

unsafe fn si514_is_prepared(hw: *mut clk_hw) -> i32 {
    let data = to_clk_si514(hw);
    let mut val: u32 = 0;
    let err = regmap_read((*data).regmap, SI514_REG_CONTROL, &mut val);
    if err < 0 { return err; }
    if val & SI514_CONTROL_OE as u32 != 0 { 1 } else { 0 }
}

/* Retrieve clock multiplier and dividers from hardware */
unsafe fn si514_get_muldiv(data: *mut clk_si514, settings: *mut clk_si514_muldiv) -> i32 {
    let mut reg = [0u8; 7];
    let err = regmap_bulk_read((*data).regmap, SI514_REG_M_FRAC1, reg.as_mut_ptr(), 7);
    if err != 0 { return err; }
    (*settings).m_frac = reg[0] as u32 | (reg[1] as u32) << 8 | (reg[2] as u32) << 16 |
        ((reg[3] & 0x1f) as u32) << 24;
    (*settings).m_int = ((reg[4] & 0x3f) << 3) | (reg[3] >> 5);
    (*settings).ls_div_bits = (reg[6] >> 4) & 0x07;
    (*settings).hs_div = (((reg[6] & 0x03) as u16) << 8) | reg[5] as u16;
    0
}

unsafe fn si514_set_muldiv(data: *mut clk_si514, settings: *mut clk_si514_muldiv) -> i32 {
    let m = (*settings).m_int as u32;
    let f = (*settings).m_frac;
    let lp: u8 = if m < 65 || (m == 65 && f <= 139575831) { 0x22 }
        else if m < 67 || (m == 67 && f <= 461581994) { 0x23 }
        else if m < 72 || (m == 72 && f <= 503383578) { 0x33 }
        else if m < 75 || (m == 75 && f <= 452724474) { 0x34 } else { 0x44 };
    let mut reg = [0u8; 7];
    let mut err = regmap_write((*data).regmap, SI514_REG_LP, lp as u32);
    if err < 0 { return err; }
    reg[0] = f as u8; reg[1] = (f >> 8) as u8; reg[2] = (f >> 16) as u8;
    reg[3] = (f >> 24) as u8 | ((*settings).m_int << 5);
    reg[4] = (*settings).m_int >> 3;
    reg[5] = (*settings).hs_div as u8;
    reg[6] = ((*settings).hs_div >> 8) as u8 | ((*settings).ls_div_bits << 4);
    err = regmap_bulk_write((*data).regmap, SI514_REG_HS_DIV, reg[5..].as_mut_ptr(), 2);
    if err < 0 { return err; }
    /* Writing to SI514_REG_M_INT_FRAC triggers the clock change, so that must be written last */
    regmap_bulk_write((*data).regmap, SI514_REG_M_FRAC1, reg.as_mut_ptr(), 5)
}

unsafe fn si514_calc_muldiv(settings: *mut clk_si514_muldiv, frequency: u64) -> i32 {
    if frequency < SI514_MIN_FREQ as u64 || frequency > SI514_MAX_FREQ as u64 { return -22; }
    let mut ls_freq = frequency as u32;
    if frequency >= (FVCO_MIN / HS_DIV_MAX) as u64 { (*settings).ls_div_bits = 0; }
    else {
        let mut res: u8 = 1; let mut tmp = 2 * HS_DIV_MAX;
        while tmp <= HS_DIV_MAX * 32 { if frequency * tmp as u64 >= FVCO_MIN as u64 { break; } res += 1; tmp <<= 1; }
        (*settings).ls_div_bits = res; ls_freq = (frequency as u32) << res;
    }
    (*settings).hs_div = (((FVCO_MIN >> 1) + ls_freq - 1) / ls_freq << 1) as u16;
    let mut m = ((ls_freq as u64 * (*settings).hs_div as u64) << 29) + FXO as u64 / 2;
    m /= FXO as u64;
    (*settings).m_frac = m as u32 & ((1u32 << 29) - 1);
    (*settings).m_int = (m >> 29) as u8;
    0
}

unsafe fn si514_calc_rate(settings: *mut clk_si514_muldiv) -> u64 {
    let m = (*settings).m_frac as u64 | ((*settings).m_int as u64 << 29);
    let d = (*settings).hs_div as u64 * (1u64 << (*settings).ls_div_bits);
    (((m * FXO as u64) + FXO as u64 / 2) >> 29) / d
}

unsafe fn si514_recalc_rate(hw: *mut clk_hw, _parent_rate: u64) -> u64 {
    let data = to_clk_si514(hw); let mut settings = core::mem::zeroed();
    if si514_get_muldiv(data, &mut settings) != 0 { return 0; }
    si514_calc_rate(&mut settings)
}

unsafe fn si514_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    if (*req).rate == 0 { (*req).rate = 0; return 0; }
    let mut settings = core::mem::zeroed();
    let err = si514_calc_muldiv(&mut settings, (*req).rate);
    if err != 0 { (*req).rate = err as u64; return 0; }
    (*req).rate = si514_calc_rate(&mut settings); 0
}

unsafe fn si514_set_rate(hw: *mut clk_hw, rate: u64, _parent_rate: u64) -> i32 {
    let data = to_clk_si514(hw); let mut settings = core::mem::zeroed();
    let mut err = si514_calc_muldiv(&mut settings, rate); if err != 0 { return err; }
    let mut old_oe_state = 0u32;
    err = regmap_read((*data).regmap, SI514_REG_CONTROL, &mut old_oe_state); if err != 0 { return err; }
    si514_enable_output(data, false);
    err = si514_set_muldiv(data, &mut settings); if err < 0 { return err; }
    err = regmap_write((*data).regmap, SI514_REG_CONTROL, SI514_CONTROL_FCAL as u32); if err < 0 { return err; }
    usleep_range(10000, 12000);
    if old_oe_state & SI514_CONTROL_OE as u32 != 0 { si514_enable_output(data, true); }
    err
}

unsafe fn si514_regmap_is_volatile(_dev: *mut device, reg: u32) -> bool {
    reg == SI514_REG_CONTROL || reg == SI514_REG_RESET
}

unsafe fn si514_regmap_is_writeable(_dev: *mut device, reg: u32) -> bool {
    reg == SI514_REG_LP || (SI514_REG_M_FRAC1..=SI514_REG_LS_HS_DIV).contains(&reg) ||
        reg == SI514_REG_OE_STATE || reg == SI514_REG_RESET || reg == SI514_REG_CONTROL
}

/* Kernel registration tables and module metadata retain their C ABI shape. */
static SI514_DRIVER_NAME: &[u8] = b"si514\0";
static SI514_COMPATIBLE: &[u8] = b"silabs,si514\0";
static SI514_AUTHOR: &[u8] = b"Mike Looijmans <mike.looijmans@topic.nl>\0";
static SI514_DESCRIPTION: &[u8] = b"Si514 driver\0";
static SI514_LICENSE: &[u8] = b"GPL\0";

unsafe fn si514_probe(client: *mut i2c_client) -> i32 {
    let data = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<clk_si514>(), GFP_KERNEL);
    if data.is_null() { return -12; }
    /* init.ops = &si514_clk_ops; init.flags = 0; init.num_parents = 0; */
    i2c_set_clientdata(client, data);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
