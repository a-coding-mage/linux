// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2017, The Linux Foundation. All rights reserved. */

// Linux kernel dependencies from the original translation unit are external.

const REG_DIV_CTL1: u32 = 0x43;
const DIV_CTL1_DIV_FACTOR_MASK: u32 = 0x7;
const REG_EN_CTL: u32 = 0x46;
const REG_EN_MASK: u32 = 1 << 7;

#[repr(C)]
struct clkdiv {
    regmap: *mut regmap,
    base: u16,
    lock: spinlock_t,
    hw: clk_hw,
    cxo_period_ns: u32,
}

#[inline]
unsafe fn to_clkdiv(hw: *mut clk_hw) -> *mut clkdiv {
    // Equivalent to container_of(hw, struct clkdiv, hw).
    (hw as *mut u8).sub(core::mem::offset_of!(clkdiv, hw)) as *mut clkdiv
}

#[inline]
fn div_factor_to_div(mut div_factor: u32) -> u32 {
    if div_factor == 0 {
        div_factor = 1;
    }
    1u32 << (div_factor - 1)
}

#[inline]
fn div_to_div_factor(div: u32) -> u32 {
    core::cmp::min(31 - div.leading_zeros() + 1, 7)
}

unsafe fn is_spmi_pmic_clkdiv_enabled(clkdiv: *mut clkdiv) -> bool {
    let mut val: u32 = 0;
    regmap_read((*clkdiv).regmap, (*clkdiv).base as u32 + REG_EN_CTL, &mut val);
    val & REG_EN_MASK != 0
}

unsafe fn __spmi_pmic_clkdiv_set_enable_state(
    clkdiv: *mut clkdiv,
    enable: bool,
    div_factor: u32,
) -> i32 {
    let ns = (*clkdiv).cxo_period_ns;
    let div = div_factor_to_div(div_factor);
    let ret = regmap_update_bits(
        (*clkdiv).regmap,
        (*clkdiv).base as u32 + REG_EN_CTL,
        REG_EN_MASK,
        if enable { REG_EN_MASK } else { 0 },
    );
    if ret != 0 {
        return ret;
    }
    if enable {
        ndelay((2 + 3 * div) * ns);
    } else {
        ndelay(3 * div * ns);
    }
    0
}

unsafe fn spmi_pmic_clkdiv_set_enable_state(clkdiv: *mut clkdiv, enable: bool) -> i32 {
    let mut div_factor: u32 = 0;
    regmap_read(
        (*clkdiv).regmap,
        (*clkdiv).base as u32 + REG_DIV_CTL1,
        &mut div_factor,
    );
    div_factor &= DIV_CTL1_DIV_FACTOR_MASK;
    __spmi_pmic_clkdiv_set_enable_state(clkdiv, enable, div_factor)
}

unsafe fn clk_spmi_pmic_div_enable(hw: *mut clk_hw) -> i32 {
    let clkdiv = to_clkdiv(hw);
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*clkdiv).lock, &mut flags);
    let ret = spmi_pmic_clkdiv_set_enable_state(clkdiv, true);
    spin_unlock_irqrestore(&mut (*clkdiv).lock, flags);
    ret
}

unsafe fn clk_spmi_pmic_div_disable(hw: *mut clk_hw) {
    let clkdiv = to_clkdiv(hw);
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*clkdiv).lock, &mut flags);
    spmi_pmic_clkdiv_set_enable_state(clkdiv, false);
    spin_unlock_irqrestore(&mut (*clkdiv).lock, flags);
}

unsafe fn clk_spmi_pmic_div_determine_rate(
    _hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let mut div = ((*req).best_parent_rate + (*req).rate - 1) / (*req).rate;
    let div_factor = div_to_div_factor(div as u32);
    div = div_factor_to_div(div_factor as u32) as ulong;
    (*req).rate = (*req).best_parent_rate / div;
    0
}

unsafe fn clk_spmi_pmic_div_recalc_rate(_hw: *mut clk_hw, parent_rate: ulong) -> ulong {
    let clkdiv = to_clkdiv(_hw);
    let mut div_factor: u32 = 0;
    regmap_read((*clkdiv).regmap, (*clkdiv).base as u32 + REG_DIV_CTL1, &mut div_factor);
    div_factor &= DIV_CTL1_DIV_FACTOR_MASK;
    parent_rate / div_factor_to_div(div_factor) as ulong
}

unsafe fn clk_spmi_pmic_div_set_rate(
    hw: *mut clk_hw,
    rate: ulong,
    parent_rate: ulong,
) -> i32 {
    let clkdiv = to_clkdiv(hw);
    let div_factor = div_to_div_factor((parent_rate / rate) as u32);
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*clkdiv).lock, &mut flags);
    let enabled = is_spmi_pmic_clkdiv_enabled(clkdiv);
    if enabled {
        let ret = spmi_pmic_clkdiv_set_enable_state(clkdiv, false);
        if ret != 0 {
            spin_unlock_irqrestore(&mut (*clkdiv).lock, flags);
            return ret;
        }
    }
    let ret = regmap_update_bits(
        (*clkdiv).regmap,
        (*clkdiv).base as u32 + REG_DIV_CTL1,
        DIV_CTL1_DIV_FACTOR_MASK,
        div_factor,
    );
    if ret != 0 {
        spin_unlock_irqrestore(&mut (*clkdiv).lock, flags);
        return ret;
    }
    let ret = if enabled {
        __spmi_pmic_clkdiv_set_enable_state(clkdiv, true, div_factor)
    } else {
        0
    };
    spin_unlock_irqrestore(&mut (*clkdiv).lock, flags);
    ret
}

static mut clk_spmi_pmic_div_ops: clk_ops = clk_ops {
    enable: Some(clk_spmi_pmic_div_enable),
    disable: Some(clk_spmi_pmic_div_disable),
    set_rate: Some(clk_spmi_pmic_div_set_rate),
    recalc_rate: Some(clk_spmi_pmic_div_recalc_rate),
    determine_rate: Some(clk_spmi_pmic_div_determine_rate),
};

#[repr(C)]
struct spmi_pmic_div_clk_cc {
    nclks: i32,
    clks: [clkdiv; 0],
}

unsafe fn spmi_pmic_div_clk_hw_get(
    clkspec: *mut of_phandle_args,
    data: *mut core::ffi::c_void,
) -> *mut clk_hw {
    let cc = data as *mut spmi_pmic_div_clk_cc;
    let idx = (*clkspec).args[0] as i32 - 1;
    if idx < 0 || idx >= (*cc).nclks {
        pr_err("spmi_pmic_div_clk_hw_get: index value is invalid; allowed range [1, nclks]");
        return err_ptr(-22);
    }
    &mut (*cc).clks.as_mut_ptr().add(idx as usize).as_mut().unwrap().hw
}

unsafe fn spmi_pmic_clkdiv_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let of_node = dev.of_node;
    let mut start: u32 = 0;
    let mut nclks: i32 = 0;
    let mut ret = of_property_read_u32(of_node, "reg", &mut start);
    if ret < 0 { dev_err(dev, "reg property reading failed"); return ret; }
    let regmap = dev_get_regmap((*dev).parent, core::ptr::null());
    if regmap.is_null() { dev_err(dev, "Couldn't get parent's regmap"); return -22; }
    ret = of_property_read_u32(of_node, "qcom,num-clkdivs", &mut nclks as *mut i32 as *mut u32);
    if ret < 0 { dev_err(dev, "qcom,num-clkdivs property reading failed"); return ret; }
    if nclks == 0 { return -22; }
    let cc = devm_kzalloc(dev, struct_size_clkdiv(nclks as usize), GFP_KERNEL) as *mut spmi_pmic_div_clk_cc;
    if cc.is_null() { return -12; }
    (*cc).nclks = nclks;
    let cxo = clk_get(dev, "xo");
    if is_err(cxo) { return ptr_err(cxo); }
    let cxo_hz = clk_get_rate(cxo);
    clk_put(cxo);
    let mut name = [0i8; 20];
    let mut init: clk_init_data = core::mem::zeroed();
    init.parent_data = &mut clk_parent_data { index: 0 };
    init.num_parents = 1;
    init.ops = &raw mut clk_spmi_pmic_div_ops;
    for i in 0..nclks as usize {
        snprintf(name.as_mut_ptr(), name.len(), "div_clk%d", i as i32 + 1);
        let clkdiv = (*cc).clks.as_mut_ptr().add(i);
        spin_lock_init(&mut (*clkdiv).lock);
        (*clkdiv).base = (start + (i as u32) * 0x100) as u16;
        (*clkdiv).regmap = regmap;
        (*clkdiv).cxo_period_ns = NSEC_PER_SEC / cxo_hz;
        init.name = name.as_ptr();
        (*clkdiv).hw.init = &init;
        ret = devm_clk_hw_register(dev, &mut (*clkdiv).hw);
        if ret != 0 { return ret; }
    }
    devm_of_clk_add_hw_provider(dev, Some(spmi_pmic_div_clk_hw_get), cc as *mut core::ffi::c_void)
}

static spmi_pmic_clkdiv_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "qcom,spmi-clkdiv" },
    of_device_id { compatible: core::ptr::null() },
];

static mut spmi_pmic_clkdiv_driver: platform_driver = platform_driver {
    driver: device_driver { name: "qcom,spmi-pmic-clkdiv", of_match_table: spmi_pmic_clkdiv_match_table.as_ptr() },
    probe: Some(spmi_pmic_clkdiv_probe),
};

// MODULE_DEVICE_TABLE(of, spmi_pmic_clkdiv_match_table);
// module_platform_driver(spmi_pmic_clkdiv_driver);
// MODULE_DESCRIPTION("QCOM SPMI PMIC clkdiv driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
