// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017, Intel Corporation
 */

// Dependencies supplied by the surrounding kernel translation.

const SOCFPGA_CS_PDBG_CLK: &str = "cs_pdbg_clk";
const SOCFPGA_EMAC0_CLK: &str = "emac0_clk";
const SOCFPGA_EMAC1_CLK: &str = "emac1_clk";
const SOCFPGA_EMAC2_CLK: &str = "emac2_clk";
const AGILEX_BYPASS_OFFSET: usize = 0xC;
const STRATIX10_BYPASS_OFFSET: usize = 0x2C;
const BOOTCLK_BYPASS: u8 = 2;

unsafe fn to_socfpga_gate_clk(p: *mut clk_hw) -> *mut socfpga_gate_clk {
    container_of!(p, socfpga_gate_clk, hw.hw)
}

unsafe fn socfpga_gate_clk_recalc_rate(hwclk: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let socfpgaclk = &*to_socfpga_gate_clk(hwclk);
    let mut div: u32 = 1;
    let val: u32;

    if socfpgaclk.fixed_div != 0 {
        div = socfpgaclk.fixed_div;
    } else if !socfpgaclk.div_reg.is_null() {
        val = readl(socfpgaclk.div_reg) >> socfpgaclk.shift;
        let val = val & genmask(socfpgaclk.width - 1, 0);
        div = 1u32 << val;
    }
    parent_rate / div as c_ulong
}

unsafe fn socfpga_dbg_clk_recalc_rate(hwclk: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let socfpgaclk = &*to_socfpga_gate_clk(hwclk);
    let mut val = readl(socfpgaclk.div_reg) >> socfpgaclk.shift;
    val &= genmask(socfpgaclk.width - 1, 0);
    let mut div = 1u32 << val;
    div = if div != 0 { 4 } else { 1 };
    parent_rate / div as c_ulong
}

unsafe fn socfpga_gate_get_parent(hwclk: *mut clk_hw) -> u8 {
    let socfpgaclk = &*to_socfpga_gate_clk(hwclk);
    let mut parent: u8 = 0;
    let name = clk_hw_get_name(hwclk);

    if !socfpgaclk.bypass_reg.is_null() {
        let mask = 0x1u32 << socfpgaclk.bypass_shift;
        parent = ((readl(socfpgaclk.bypass_reg) & mask) >> socfpgaclk.bypass_shift) as u8;
    }

    if streq(name, SOCFPGA_EMAC0_CLK) || streq(name, SOCFPGA_EMAC1_CLK) || streq(name, SOCFPGA_EMAC2_CLK) {
        let second_bypass = readl(socfpgaclk.bypass_reg.byte_offset(-(STRATIX10_BYPASS_OFFSET as isize)));
        // EMACA bypass to bootclk @0xB0 offset
        if second_bypass & 0x1 != 0 && parent == 0 { parent = BOOTCLK_BYPASS; }
        if second_bypass & 0x2 != 0 && parent == 1 { parent = BOOTCLK_BYPASS; }
    }
    parent
}

unsafe fn socfpga_agilex_gate_get_parent(hwclk: *mut clk_hw) -> u8 {
    let socfpgaclk = &*to_socfpga_gate_clk(hwclk);
    let mut parent: u8 = 0;
    let name = clk_hw_get_name(hwclk);

    if !socfpgaclk.bypass_reg.is_null() {
        let mask = 0x1u32 << socfpgaclk.bypass_shift;
        parent = ((readl(socfpgaclk.bypass_reg) & mask) >> socfpgaclk.bypass_shift) as u8;
    }
    if streq(name, SOCFPGA_EMAC0_CLK) || streq(name, SOCFPGA_EMAC1_CLK) || streq(name, SOCFPGA_EMAC2_CLK) {
        let second_bypass = readl(socfpgaclk.bypass_reg.byte_offset(-(AGILEX_BYPASS_OFFSET as isize)));
        // EMACA bypass to bootclk @0x88 offset
        if second_bypass & 0x1 != 0 && parent == 0 { parent = BOOTCLK_BYPASS; }
        if second_bypass & 0x2 != 0 && parent == 1 { parent = BOOTCLK_BYPASS; }
    }
    parent
}

static mut gateclk_ops: clk_ops = clk_ops {
    recalc_rate: Some(socfpga_gate_clk_recalc_rate),
    get_parent: Some(socfpga_gate_get_parent),
    ..clk_ops::zeroed()
};
static agilex_gateclk_ops: clk_ops = clk_ops {
    recalc_rate: Some(socfpga_gate_clk_recalc_rate),
    get_parent: Some(socfpga_agilex_gate_get_parent),
    ..clk_ops::zeroed()
};
static dbgclk_ops: clk_ops = clk_ops {
    recalc_rate: Some(socfpga_dbg_clk_recalc_rate),
    get_parent: Some(socfpga_gate_get_parent),
    ..clk_ops::zeroed()
};

unsafe fn register_gate_common(clks: *const stratix10_gate_clock, regbase: *mut u8, ops: *const clk_ops) -> *mut clk_hw {
    let socfpga_clk = kzalloc_obj::<socfpga_gate_clk>();
    if socfpga_clk.is_null() { return core::ptr::null_mut(); }
    (*socfpga_clk).hw.reg = regbase.add((*clks).gate_reg);
    (*socfpga_clk).hw.bit_idx = (*clks).gate_idx;
    (*gateclk_ops_mut()).enable = clk_gate_ops.enable;
    (*gateclk_ops_mut()).disable = clk_gate_ops.disable;
    (*socfpga_clk).fixed_div = (*clks).fixed_div;
    (*socfpga_clk).div_reg = if (*clks).div_reg != 0 { regbase.add((*clks).div_reg) } else { core::ptr::null_mut() };
    (*socfpga_clk).width = (*clks).div_width;
    (*socfpga_clk).shift = (*clks).div_offset;
    (*socfpga_clk).bypass_reg = if (*clks).bypass_reg != 0 { regbase.add((*clks).bypass_reg) } else { core::ptr::null_mut() };
    (*socfpga_clk).bypass_shift = (*clks).bypass_shift;
    let mut init: clk_init_data = core::mem::zeroed();
    init.ops = ops;
    init.name = (*clks).name;
    init.flags = (*clks).flags;
    init.num_parents = (*clks).num_parents;
    let parent_name = (*clks).parent_name;
    init.parent_names = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    if init.parent_names.is_null() { init.parent_data = (*clks).parent_data; }
    (*socfpga_clk).hw.hw.init = &init;
    let hw_clk = &mut (*socfpga_clk).hw.hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if ret != 0 { kfree(socfpga_clk as *mut c_void); return err_ptr(ret); }
    hw_clk
}

unsafe fn s10_register_gate(clks: *const stratix10_gate_clock, regbase: *mut u8) -> *mut clk_hw {
    let ops = if streq((*clks).name, SOCFPGA_CS_PDBG_CLK) { &dbgclk_ops } else { &gateclk_ops };
    register_gate_common(clks, regbase, ops)
}

unsafe fn agilex_register_gate(clks: *const stratix10_gate_clock, regbase: *mut u8) -> *mut clk_hw {
    let ops = if streq((*clks).name, SOCFPGA_CS_PDBG_CLK) { &dbgclk_ops } else { &agilex_gateclk_ops };
    register_gate_common(clks, regbase, ops)
}

unsafe fn agilex5_register_gate(clks: *const agilex5_gate_clock, regbase: *mut u8) -> *mut clk_hw {
    let socfpga_clk = kzalloc_obj::<socfpga_gate_clk>();
    if socfpga_clk.is_null() { return core::ptr::null_mut(); }
    (*socfpga_clk).hw.reg = regbase.add((*clks).gate_reg);
    (*socfpga_clk).hw.bit_idx = (*clks).gate_idx;
    (*socfpga_clk).fixed_div = (*clks).fixed_div;
    (*socfpga_clk).div_reg = if (*clks).div_reg != 0 { regbase.add((*clks).div_reg) } else { core::ptr::null_mut() };
    (*socfpga_clk).width = (*clks).div_width;
    (*socfpga_clk).shift = (*clks).div_offset;
    (*socfpga_clk).bypass_reg = if (*clks).bypass_reg != 0 { regbase.add((*clks).bypass_reg) } else { core::ptr::null_mut() };
    (*socfpga_clk).bypass_shift = (*clks).bypass_shift;
    let mut init: clk_init_data = core::mem::zeroed();
    init.ops = if streq((*clks).name, SOCFPGA_CS_PDBG_CLK) { &dbgclk_ops } else { &agilex_gateclk_ops };
    init.name = (*clks).name; init.flags = (*clks).flags; init.num_parents = (*clks).num_parents; init.parent_names = (*clks).parent_names;
    (*socfpga_clk).hw.hw.init = &init;
    let hw_clk = &mut (*socfpga_clk).hw.hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if ret != 0 { kfree(socfpga_clk as *mut c_void); return err_ptr(ret); }
    hw_clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
