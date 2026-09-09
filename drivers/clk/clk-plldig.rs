// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 NXP
 *
 * Clock driver for LS1028A Display output interfaces(LCD, DPHY).
 */

// Linux kernel dependencies are supplied by the surrounding repository.

const PLLDIG_REG_PLLSR: usize = 0x24;
const PLLDIG_LOCK_MASK: u32 = 1 << 2;
const PLLDIG_REG_PLLDV: usize = 0x28;
const PLLDIG_MFD_MASK: u32 = 0xff;
const PLLDIG_RFDPHI1_MASK: u32 = 0x3f << 25;
const PLLDIG_REG_PLLFM: usize = 0x2c;
const PLLDIG_SSCGBYP_ENABLE: u32 = 1 << 30;
const PLLDIG_REG_PLLFD: usize = 0x30;
const PLLDIG_FDEN: u32 = 1 << 30;
const PLLDIG_FRAC_MASK: u32 = 0xffff;
const PLLDIG_REG_PLLCAL1: usize = 0x38;
const PLLDIG_REG_PLLCAL2: usize = 0x3c;

const PLLDIG_MIN_VCO_FREQ: u32 = 650000000;
const PLLDIG_MAX_VCO_FREQ: u32 = 1300000000;
const PHI1_MIN_FREQ: u64 = 27000000;
const PHI1_MAX_FREQ: u64 = 600000000;
const MAX_RFDPHI1: u64 = 63;
const PLLDIG_DEFAULT_MFD: u32 = 44;
const MFDEN: u64 = 20480;

static parent_data: [clk_parent_data; 1] = [clk_parent_data { index: 0 }];

#[repr(C)]
struct clk_plldig {
    hw: clk_hw,
    regs: *mut core::ffi::c_void,
    vco_freq: u32,
}

unsafe fn to_clk_plldig(hw: *mut clk_hw) -> *mut clk_plldig {
    (hw as *mut u8).sub(core::mem::offset_of!(clk_plldig, hw)) as *mut clk_plldig
}

unsafe fn plldig_enable(hw: *mut clk_hw) -> i32 {
    let data = &mut *to_clk_plldig(hw);
    let mut val = readl(data.regs.add(PLLDIG_REG_PLLFM));
    val |= PLLDIG_SSCGBYP_ENABLE;
    writel(val, data.regs.add(PLLDIG_REG_PLLFM));
    0
}

unsafe fn plldig_disable(hw: *mut clk_hw) {
    let data = &mut *to_clk_plldig(hw);
    let mut val = readl(data.regs.add(PLLDIG_REG_PLLFM));
    val &= !PLLDIG_SSCGBYP_ENABLE;
    val |= field_prep(PLLDIG_SSCGBYP_ENABLE, 0x0);
    writel(val, data.regs.add(PLLDIG_REG_PLLFM));
}

unsafe fn plldig_is_enabled(hw: *mut clk_hw) -> i32 {
    let data = &mut *to_clk_plldig(hw);
    (readl(data.regs.add(PLLDIG_REG_PLLFM)) & PLLDIG_SSCGBYP_ENABLE) as i32
}

unsafe fn plldig_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let data = &mut *to_clk_plldig(hw);
    let val = readl(data.regs.add(PLLDIG_REG_PLLDV));
    if val & PLLDIG_SSCGBYP_ENABLE != 0 {
        return parent_rate;
    }
    let mut rfdphi1 = field_get(PLLDIG_RFDPHI1_MASK, val);
    if rfdphi1 == 0 {
        rfdphi1 = 1;
    }
    (data.vco_freq as u64 + rfdphi1 as u64 - 1) / rfdphi1 as u64
}

fn plldig_calc_target_div(vco_freq: u64, target_rate: u64) -> u64 {
    let mut div = (vco_freq + target_rate / 2) / target_rate;
    div = div.clamp(1, MAX_RFDPHI1);
    div
}

unsafe fn plldig_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let data = &mut *to_clk_plldig(hw);
    (*req).rate = (*req).rate.clamp(PHI1_MIN_FREQ, PHI1_MAX_FREQ);
    let div = plldig_calc_target_div(data.vco_freq as u64, (*req).rate);
    (*req).rate = (data.vco_freq as u64 + div - 1) / div;
    0
}

unsafe fn plldig_set_rate(hw: *mut clk_hw, mut rate: u64, _parent_rate: u64) -> i32 {
    let data = &mut *to_clk_plldig(hw);
    rate = rate.clamp(PHI1_MIN_FREQ, PHI1_MAX_FREQ);
    let rfdphi1 = plldig_calc_target_div(data.vco_freq as u64, rate);
    let mut val = readl(data.regs.add(PLLDIG_REG_PLLDV));
    val &= !PLLDIG_RFDPHI1_MASK;
    val |= field_prep(PLLDIG_RFDPHI1_MASK, rfdphi1 as u32);
    writel(val, data.regs.add(PLLDIG_REG_PLLDV));
    udelay(200);
    let mut cond: u32 = 0;
    readl_poll_timeout_atomic(data.regs.add(PLLDIG_REG_PLLSR), &mut cond,
                               cond & PLLDIG_LOCK_MASK != 0, 0, USEC_PER_MSEC)
}

unsafe fn plldig_init(hw: *mut clk_hw) -> i32 {
    let data = &mut *to_clk_plldig(hw);
    let parent = clk_hw_get_parent(hw);
    if parent.is_null() { return -22; }
    let parent_rate = clk_hw_get_rate(parent);
    let (mfd, fracdiv) = if data.vco_freq != 0 {
        let mfd = data.vco_freq / parent_rate as u32;
        let mut lltmp = (data.vco_freq % parent_rate as u32) as u64 * MFDEN;
        lltmp /= parent_rate;
        (mfd, lltmp as u32)
    } else {
        data.vco_freq = parent_rate as u32 * PLLDIG_DEFAULT_MFD;
        (PLLDIG_DEFAULT_MFD, 0)
    };
    let val = field_prep(PLLDIG_MFD_MASK, mfd);
    writel(val, data.regs.add(PLLDIG_REG_PLLDV));
    if fracdiv != 0 {
        let mut fracval = field_prep(PLLDIG_FRAC_MASK, fracdiv);
        fracval |= PLLDIG_FDEN;
        writel(fracval, data.regs.add(PLLDIG_REG_PLLFD));
    }
    0
}

unsafe fn plldig_clk_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let data = devm_kzalloc(dev, core::mem::size_of::<clk_plldig>(), GFP_KERNEL)
        as *mut clk_plldig;
    if data.is_null() { return -12; }

    (*data).regs = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*data).regs) { return ptr_err((*data).regs); }

    (*data).hw.init = clk_hw_init_parents_data(
        "dpclk", &parent_data, &plldig_clk_ops, 0);

    let mut ret = devm_clk_hw_register(dev, &mut (*data).hw);
    if ret != 0 { return ret; }
    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get,
                                      &mut (*data).hw);
    if ret != 0 { return ret; }

    let mut vco_freq = 0u32;
    if of_property_read_u32((*dev).of_node, "fsl,vco-hz", &mut vco_freq) == 0 {
        if vco_freq < PLLDIG_MIN_VCO_FREQ || vco_freq > PLLDIG_MAX_VCO_FREQ {
            return -22;
        }
        (*data).vco_freq = vco_freq;
    }
    plldig_init(&mut (*data).hw)
}

// External kernel types, functions, and registration macros are supplied by
// the surrounding repository; their declarations are intentionally not
// reimplemented here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
