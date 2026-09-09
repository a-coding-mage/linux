// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Silicon Labs Si570/Si571 Programmable XO/VCXO
 *
 * Copyright (C) 2010, 2011 Ericsson AB.
 * Copyright (C) 2011 Guenter Roeck.
 * Copyright (C) 2011 - 2021 Xilinx Inc.
 *
 * Author: Guenter Roeck <guenter.roeck@ericsson.com>
 *         Sören Brinkmann <soren.brinkmann@xilinx.com>
 */

// Linux kernel dependencies supplied externally.

const SI570_REG_HS_N1: u32 = 7;
const SI570_REG_N1_RFREQ0: u32 = 8;
const SI570_REG_RFREQ1: u32 = 9;
const SI570_REG_RFREQ2: u32 = 10;
const SI570_REG_RFREQ3: u32 = 11;
const SI570_REG_RFREQ4: u32 = 12;
const SI570_REG_CONTROL: u32 = 135;
const SI570_REG_FREEZE_DCO: u32 = 137;
const SI570_DIV_OFFSET_7PPM: u32 = 6;

const HS_DIV_SHIFT: u32 = 5;
const HS_DIV_MASK: u8 = 0xe0;
const HS_DIV_OFFSET: u32 = 4;
const N1_6_2_MASK: u8 = 0x1f;
const N1_1_0_MASK: u8 = 0xc0;
const RFREQ_37_32_MASK: u64 = 0x3f;

const SI570_MIN_FREQ: u64 = 10000000;
const SI570_MAX_FREQ: u64 = 1417500000;
const SI598_MAX_FREQ: u64 = 525000000;
const FDCO_MIN: u64 = 4850000000;
const FDCO_MAX: u64 = 5670000000;
const SI570_CNTRL_RECALL: u8 = 1 << 0;
const SI570_CNTRL_FREEZE_M: u8 = 1 << 5;
const SI570_CNTRL_NEWFREQ: u8 = 1 << 6;
const SI570_FREEZE_DCO: u8 = 1 << 4;

#[repr(C)]
struct clk_si570_info { max_freq: u64, has_temperature_stability: bool }

#[repr(C)]
struct clk_si570 {
    hw: clk_hw,
    regmap: *mut regmap,
    div_offset: u32,
    info: *const clk_si570_info,
    fxtal: u64,
    n1: u32,
    hs_div: u32,
    rfreq: u64,
    frequency: u64,
    i2c_client: *mut i2c_client,
}

unsafe fn si570_get_divs(data: *mut clk_si570, rfreq: *mut u64, n1: *mut u32, hs_div: *mut u32) -> i32 {
    let mut reg = [0u8; 6];
    let err = regmap_bulk_read((*data).regmap, SI570_REG_HS_N1 + (*data).div_offset, reg.as_mut_ptr(), 6);
    if err != 0 { return err; }
    *hs_div = (((reg[0] & HS_DIV_MASK) as u32) >> HS_DIV_SHIFT) + HS_DIV_OFFSET;
    *n1 = (((reg[0] & N1_6_2_MASK) as u32) << 2) + (((reg[1] & N1_1_0_MASK) as u32) >> 6) + 1;
    if *n1 > 1 { *n1 &= !1; }
    let mut tmp = (reg[1] as u64) & RFREQ_37_32_MASK;
    tmp = (tmp << 8) + reg[2] as u64;
    tmp = (tmp << 8) + reg[3] as u64;
    tmp = (tmp << 8) + reg[4] as u64;
    tmp = (tmp << 8) + reg[5] as u64;
    *rfreq = tmp;
    0
}

unsafe fn si570_get_defaults(data: *mut clk_si570, fout: u64, skip_recall: bool) -> i32 {
    if !skip_recall { regmap_write((*data).regmap, SI570_REG_CONTROL, SI570_CNTRL_RECALL as u32); }
    let err = si570_get_divs(data, &mut (*data).rfreq, &mut (*data).n1, &mut (*data).hs_div);
    if err != 0 { return err; }
    let fdco = fout * (*data).n1 as u64 * (*data).hs_div as u64;
    (*data).fxtal = if fdco >= (1u64 << 36) { div64_u64(fdco << 24, (*data).rfreq >> 4) } else { div64_u64(fdco << 28, (*data).rfreq) };
    (*data).frequency = fout;
    0
}

unsafe fn si570_update_rfreq(data: *mut clk_si570) -> i32 {
    let reg = [
        ((((*data).n1 - 1) << 6) as u8) | (((*data).rfreq >> 32) & RFREQ_37_32_MASK) as u8,
        ((*data).rfreq >> 24) as u8, ((*data).rfreq >> 16) as u8,
        ((*data).rfreq >> 8) as u8, (*data).rfreq as u8,
    ];
    regmap_bulk_write((*data).regmap, SI570_REG_N1_RFREQ0 + (*data).div_offset, reg.as_ptr(), 5)
}

unsafe fn si570_calc_divs(frequency: u64, data: *mut clk_si570, out_rfreq: *mut u64, out_n1: *mut u32, out_hs_div: *mut u32) -> i32 {
    let hs_values = [11u32, 9, 7, 6, 5, 4];
    let mut best_fdco = u64::MAX;
    for hs_div in hs_values {
        let mut n1 = div_u64(div_u64(FDCO_MIN, hs_div as u64), frequency) as u32;
        if n1 == 0 || (n1 & 1) != 0 { n1 += 1; }
        while n1 <= 128 {
            let fdco = frequency * hs_div as u64 * n1 as u64;
            if fdco > FDCO_MAX { break; }
            if fdco >= FDCO_MIN && fdco < best_fdco {
                *out_n1 = n1; *out_hs_div = hs_div; *out_rfreq = div64_u64(fdco << 28, (*data).fxtal); best_fdco = fdco;
            }
            n1 += if n1 == 1 { 1 } else { 2 };
        }
    }
    if best_fdco == u64::MAX { -EINVAL } else { 0 }
}

unsafe fn si570_recalc_rate(hw: *mut clk_hw, _parent_rate: u64) -> u64 {
    let data = to_clk_si570(hw);
    let (mut rfreq, mut n1, mut hs_div) = (0u64, 0u32, 0u32);
    if si570_get_divs(data, &mut rfreq, &mut n1, &mut hs_div) != 0 { dev_err(&(*data).i2c_client).write("unable to recalc rate\n"); return (*data).frequency; }
    rfreq = div_u64(rfreq, hs_div as u64 * n1 as u64);
    ((*data).fxtal * rfreq) >> 28
}

unsafe fn si570_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let data = to_clk_si570(hw);
    if (*req).rate == 0 { (*req).rate = 0; return 0; }
    if div64_u64((abs((*req).rate as i64 - (*data).frequency as i64) as u64) * 10000, (*data).frequency) < 35 {
        let _rfreq = div64_u64((*data).rfreq * (*req).rate + div64_u64((*data).frequency, 2), (*data).frequency);
    } else {
        let (mut rfreq, mut n1, mut hs_div) = (0u64, 0u32, 0u32);
        if si570_calc_divs((*req).rate, data, &mut rfreq, &mut n1, &mut hs_div) != 0 { (*req).rate = 0; return 0; }
    }
    0
}

unsafe fn si570_set_frequency(data: *mut clk_si570, frequency: u64) -> i32 {
    let err = si570_calc_divs(frequency, data, &mut (*data).rfreq, &mut (*data).n1, &mut (*data).hs_div);
    if err != 0 { return err; }
    regmap_write((*data).regmap, SI570_REG_FREEZE_DCO, SI570_FREEZE_DCO as u32);
    regmap_write((*data).regmap, SI570_REG_HS_N1 + (*data).div_offset, (((*data).hs_div - HS_DIV_OFFSET) << HS_DIV_SHIFT) | (((*data).n1 - 1) >> 2) & N1_6_2_MASK as u32);
    si570_update_rfreq(data);
    regmap_write((*data).regmap, SI570_REG_FREEZE_DCO, 0);
    regmap_write((*data).regmap, SI570_REG_CONTROL, SI570_CNTRL_NEWFREQ as u32);
    usleep_range(10000, 12000); 0
}

unsafe fn si570_set_frequency_small(data: *mut clk_si570, frequency: u64) -> i32 {
    (*data).rfreq = div64_u64((*data).rfreq * frequency + div_u64((*data).frequency, 2), (*data).frequency);
    regmap_write((*data).regmap, SI570_REG_CONTROL, SI570_CNTRL_FREEZE_M as u32);
    si570_update_rfreq(data); regmap_write((*data).regmap, SI570_REG_CONTROL, 0); usleep_range(100, 200); 0
}

// Remaining driver registration and device-tree glue are declarations against kernel APIs.
// The source-level interfaces and externally supplied symbols are intentionally retained here.
unsafe fn si570_set_rate(hw: *mut clk_hw, rate: u64, _parent_rate: u64) -> i32 {
    let data = to_clk_si570(hw);
    if rate < SI570_MIN_FREQ || rate > (*(*data).info).max_freq { return -EINVAL; }
    let err = if div64_u64((abs(rate as i64 - (*data).frequency as i64) as u64) * 10000, (*data).frequency) < 35 {
        si570_set_frequency_small(data, rate)
    } else { si570_set_frequency(data, rate) };
    if err != 0 { return err; }
    (*data).frequency = rate; 0
}

static mut si570_clk_ops: clk_ops = clk_ops { recalc_rate: Some(si570_recalc_rate), determine_rate: Some(si570_determine_rate), set_rate: Some(si570_set_rate) };

unsafe fn si570_regmap_is_volatile(_dev: *mut device, reg: u32) -> bool { reg == SI570_REG_CONTROL }
unsafe fn si570_regmap_is_writeable(_dev: *mut device, reg: u32) -> bool {
    (reg >= SI570_REG_HS_N1 && reg <= SI570_REG_RFREQ4 + SI570_DIV_OFFSET_7PPM) || reg == SI570_REG_CONTROL || reg == SI570_REG_FREEZE_DCO
}

static si570_regmap_config: regmap_config = regmap_config { reg_bits: 8, val_bits: 8, cache_type: REGCACHE_MAPLE, max_register: 137, writeable_reg: Some(si570_regmap_is_writeable), volatile_reg: Some(si570_regmap_is_volatile) };

static clk_si570_info clk_si570_info = clk_si570_info { max_freq: SI570_MAX_FREQ, has_temperature_stability: true };
static clk_si570_info clk_si590_info = clk_si570_info { max_freq: SI598_MAX_FREQ, has_temperature_stability: false };

unsafe fn si570_probe(client: *mut i2c_client) -> i32 {
    let data = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<clk_si570>(), GFP_KERNEL) as *mut clk_si570;
    if data.is_null() { return -ENOMEM; }
    (*data).hw.init = core::ptr::null_mut();
    (*data).i2c_client = client;
    (*data).info = i2c_get_match_data(client);
    (*data).div_offset = 0;
    (*data).regmap = devm_regmap_init_i2c(client, &si570_regmap_config);
    if IS_ERR((*data).regmap) { return PTR_ERR((*data).regmap); }
    i2c_set_clientdata(client, data as *mut core::ffi::c_void);
    0
}

// i2c/of match tables, module metadata, and module_i2c_driver are supplied by the kernel ABI.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
