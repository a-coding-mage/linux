// SPDX-License-Identifier: GPL-2.0-only
/*
 * Clk driver for NXP LPC18xx/43xx Configuration Registers (CREG)
 *
 * Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
 */

// External Linux kernel types and functions referenced by this translation
// are supplied by the surrounding kernel bindings.

const LPC18XX_CREG_CREG0: u32 = 0x004;
const LPC18XX_CREG_CREG0_EN1KHZ: u32 = 1 << 0;
const LPC18XX_CREG_CREG0_EN32KHZ: u32 = 1 << 1;
const LPC18XX_CREG_CREG0_RESET32KHZ: u32 = 1 << 2;
const LPC18XX_CREG_CREG0_PD32KHZ: u32 = 1 << 3;

enum {
    CREG_CLK_1KHZ,
    CREG_CLK_32KHZ,
    CREG_CLK_MAX,
}

#[repr(C)]
struct clk_creg_data {
    hw: clk_hw,
    name: *const core::ffi::c_char,
    reg: *mut regmap,
    en_mask: u32,
    ops: *const clk_ops,
}

unsafe fn to_clk_creg(hw: *mut clk_hw) -> *mut clk_creg_data {
    (hw as *mut u8).sub(core::mem::offset_of!(clk_creg_data, hw)) as *mut clk_creg_data
}

unsafe fn clk_creg_32k_prepare(hw: *mut clk_hw) -> i32 {
    let creg = &mut *to_clk_creg(hw);
    let ret = regmap_update_bits(
        creg.reg,
        LPC18XX_CREG_CREG0,
        LPC18XX_CREG_CREG0_PD32KHZ | LPC18XX_CREG_CREG0_RESET32KHZ,
        0,
    );

    /*
     * Powering up the 32k oscillator takes a long while
     * and sadly there aren't any status bit to poll.
     */
    msleep(2500);

    ret
}

unsafe fn clk_creg_32k_unprepare(hw: *mut clk_hw) {
    let creg = &mut *to_clk_creg(hw);
    regmap_update_bits(
        creg.reg,
        LPC18XX_CREG_CREG0,
        LPC18XX_CREG_CREG0_PD32KHZ,
        LPC18XX_CREG_CREG0_PD32KHZ,
    );
}

unsafe fn clk_creg_32k_is_prepared(hw: *mut clk_hw) -> i32 {
    let creg = &mut *to_clk_creg(hw);
    let mut reg: u32 = 0;
    regmap_read(creg.reg, LPC18XX_CREG_CREG0, &mut reg);
    (!(reg & LPC18XX_CREG_CREG0_PD32KHZ != 0)
        && !(reg & LPC18XX_CREG_CREG0_RESET32KHZ != 0)) as i32
}

unsafe fn clk_creg_1k_recalc_rate(_hw: *mut clk_hw, parent_rate: usize) -> usize {
    parent_rate / 32
}

unsafe fn clk_creg_enable(hw: *mut clk_hw) -> i32 {
    let creg = &mut *to_clk_creg(hw);
    regmap_update_bits(creg.reg, LPC18XX_CREG_CREG0, creg.en_mask, creg.en_mask)
}

unsafe fn clk_creg_disable(hw: *mut clk_hw) {
    let creg = &mut *to_clk_creg(hw);
    regmap_update_bits(creg.reg, LPC18XX_CREG_CREG0, creg.en_mask, 0);
}

unsafe fn clk_creg_is_enabled(hw: *mut clk_hw) -> i32 {
    let creg = &mut *to_clk_creg(hw);
    let mut reg: u32 = 0;
    regmap_read(creg.reg, LPC18XX_CREG_CREG0, &mut reg);
    (reg & creg.en_mask != 0) as i32
}

static CLK_CREG_32K: clk_ops = clk_ops {
    enable: Some(clk_creg_enable),
    disable: Some(clk_creg_disable),
    is_enabled: Some(clk_creg_is_enabled),
    prepare: Some(clk_creg_32k_prepare),
    unprepare: Some(clk_creg_32k_unprepare),
    is_prepared: Some(clk_creg_32k_is_prepared),
};

static CLK_CREG_1K: clk_ops = clk_ops {
    enable: Some(clk_creg_enable),
    disable: Some(clk_creg_disable),
    is_enabled: Some(clk_creg_is_enabled),
    recalc_rate: Some(clk_creg_1k_recalc_rate),
};

static mut CLK_CREG_CLOCKS: [clk_creg_data; 2] = [
    clk_creg_data { hw: clk_hw::default(), name: b"1khz_clk\0".as_ptr() as _, reg: core::ptr::null_mut(), en_mask: LPC18XX_CREG_CREG0_EN1KHZ, ops: &CLK_CREG_1K },
    clk_creg_data { hw: clk_hw::default(), name: b"32khz_clk\0".as_ptr() as _, reg: core::ptr::null_mut(), en_mask: LPC18XX_CREG_CREG0_EN32KHZ, ops: &CLK_CREG_32K },
];

unsafe fn clk_register_creg_clk(dev: *mut device, creg_clk: *mut clk_creg_data, parent_name: *const *const core::ffi::c_char, syscon: *mut regmap) -> *mut clk {
    let mut init = clk_init_data::default();
    init.ops = (*creg_clk).ops;
    init.name = (*creg_clk).name;
    init.parent_names = parent_name;
    init.num_parents = 1;
    init.flags = 0;
    (*creg_clk).reg = syscon;
    (*creg_clk).hw.init = &init;
    if !dev.is_null() { devm_clk_register(dev, &mut (*creg_clk).hw) } else { clk_register(core::ptr::null_mut(), &mut (*creg_clk).hw) }
}

static mut CLK_CREG_EARLY: [*mut clk; CREG_CLK_MAX] = [core::ptr::null_mut(); CREG_CLK_MAX];
static mut CLK_CREG_EARLY_DATA: clk_onecell_data = clk_onecell_data { clks: CLK_CREG_EARLY.as_mut_ptr(), clk_num: CREG_CLK_MAX };

unsafe fn lpc18xx_creg_clk_init(np: *mut device_node) {
    let syscon = syscon_node_to_regmap((*np).parent);
    if IS_ERR(syscon) { pr_err("%s: syscon lookup failed\n", "lpc18xx_creg_clk_init"); return; }
    let clk_32khz_parent = of_clk_get_parent_name(np, 0);
    CLK_CREG_EARLY[CREG_CLK_32KHZ] = clk_register_creg_clk(core::ptr::null_mut(), &mut CLK_CREG_CLOCKS[CREG_CLK_32KHZ], &clk_32khz_parent, syscon);
    CLK_CREG_EARLY[CREG_CLK_1KHZ] = ERR_PTR(-EPROBE_DEFER);
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut CLK_CREG_EARLY_DATA);
}

static mut CLK_CREG: [*mut clk; CREG_CLK_MAX] = [core::ptr::null_mut(); CREG_CLK_MAX];
static mut CLK_CREG_DATA: clk_onecell_data = clk_onecell_data { clks: CLK_CREG.as_mut_ptr(), clk_num: CREG_CLK_MAX };

unsafe fn lpc18xx_creg_clk_probe(pdev: *mut platform_device) -> i32 {
    let np = (*pdev).dev.of_node;
    let syscon = syscon_node_to_regmap((*np).parent);
    if IS_ERR(syscon) { dev_err(&mut (*pdev).dev, "syscon lookup failed\n"); return PTR_ERR(syscon); }
    CLK_CREG[CREG_CLK_32KHZ] = CLK_CREG_EARLY[CREG_CLK_32KHZ];
    CLK_CREG[CREG_CLK_1KHZ] = clk_register_creg_clk(core::ptr::null_mut(), &mut CLK_CREG_CLOCKS[CREG_CLK_1KHZ], &CLK_CREG_CLOCKS[CREG_CLK_32KHZ].name, syscon);
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut CLK_CREG_DATA)
}

static LPC18XX_CREG_CLK_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"nxp,lpc1850-creg-clk\0".as_ptr() as _ },
    of_device_id::default(),
];

static mut LPC18XX_CREG_CLK_DRIVER: platform_driver = platform_driver {
    probe: Some(lpc18xx_creg_clk_probe),
    driver: driver { name: b"lpc18xx-creg-clk\0".as_ptr() as _, of_match_table: LPC18XX_CREG_CLK_OF_MATCH.as_ptr() },
};

// Equivalent of builtin_platform_driver(lpc18xx_creg_clk_driver).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
