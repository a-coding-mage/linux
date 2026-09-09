// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 Tomasz Figa <t.figa@samsung.com>
 *
 * Based on Exynos Audio Subsystem Clock Controller driver:
 *
 * Copyright (c) 2013 Samsung Electronics Co., Ltd.
 * Author: Padmavathi Venna <padma.v@samsung.com>
 *
 * Driver for Audio Subsystem Clock Controller of S5PV210-compatible SoCs.
 */

// Kernel dependencies supplied by the surrounding translation unit.

const ASS_CLK_SRC: usize = 0x0;
const ASS_CLK_DIV: usize = 0x4;
const ASS_CLK_GATE: usize = 0x8;

extern "C" {
    static mut lock: core::ffi::c_ulong;
    static mut reg_base: *mut core::ffi::c_void;
    static mut clk_data: *mut clk_hw_onecell_data;

    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_clk_get(dev: *mut device, name: *const core::ffi::c_char) -> *mut clk;
    fn __clk_get_name(clk: *mut clk) -> *const core::ffi::c_char;
    fn clk_hw_register_mux(parent: *mut core::ffi::c_void, name: *const core::ffi::c_char,
        parents: *const *const core::ffi::c_char, num_parents: usize, flags: u32,
        reg: *mut core::ffi::c_void, shift: u8, width: u8, clk_flags: u8,
        lock: *mut core::ffi::c_ulong) -> *mut clk_hw;
    fn clk_hw_register_divider(parent: *mut core::ffi::c_void, name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char, flags: u32, reg: *mut core::ffi::c_void,
        shift: u8, width: u8, clk_flags: u8, lock: *mut core::ffi::c_ulong) -> *mut clk_hw;
    fn clk_hw_register_gate(parent: *mut core::ffi::c_void, name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char, flags: u32, reg: *mut core::ffi::c_void,
        bit_idx: u8, clk_flags: u8, lock: *mut core::ffi::c_ulong) -> *mut clk_hw;
    fn of_clk_add_hw_provider(node: *mut core::ffi::c_void, get: *mut core::ffi::c_void,
        data: *mut clk_hw_onecell_data) -> i32;
    fn clk_hw_unregister(hw: *mut clk_hw);
    fn register_syscore(syscore: *mut syscore);
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
}

#[repr(C)]
struct clk_hw_onecell_data { num: usize, hws: [*mut clk_hw; 0] }
#[repr(C)] struct clk_hw;
#[repr(C)] struct clk;
#[repr(C)] struct device;
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct syscore_ops { suspend: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>, resume: Option<unsafe extern "C" fn(*mut core::ffi::c_void)> }
#[repr(C)] struct syscore { ops: *const syscore_ops }
#[repr(C)] struct of_device_id { compatible: *const core::ffi::c_char }
#[repr(C)] struct platform_driver { driver: driver, probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32> }
#[repr(C)] struct driver { name: *const core::ffi::c_char, suppress_bind_attrs: bool, of_match_table: *const of_device_id }

const ASS_MAX_CLKS: usize = 10;
const CLK_MOUT_AUDSS: usize = 0;
const CLK_MOUT_I2S_A: usize = 1;
const CLK_DOUT_AUD_BUS: usize = 2;
const CLK_DOUT_I2S_A: usize = 3;
const CLK_I2S: usize = 4;
const CLK_HCLK_I2S: usize = 5;
const CLK_HCLK_UART: usize = 6;
const CLK_HCLK_HWA: usize = 7;
const CLK_HCLK_DMA: usize = 8;
const CLK_HCLK_BUF: usize = 9;
const CLK_HCLK_RP: usize = 10;

#[cfg(feature = "CONFIG_PM_SLEEP")]
static mut reg_save: [[u32; 2]; 3] = [[ASS_CLK_SRC as u32, 0], [ASS_CLK_DIV as u32, 0], [ASS_CLK_GATE as u32, 0]];

#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe extern "C" fn s5pv210_audss_clk_suspend(_data: *mut core::ffi::c_void) -> i32 {
    for i in 0..reg_save.len() { reg_save[i][1] = readl(reg_base.add(reg_save[i][0] as usize)); }
    0
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe extern "C" fn s5pv210_audss_clk_resume(_data: *mut core::ffi::c_void) {
    for i in 0..reg_save.len() { writel(reg_save[i][1], reg_base.add(reg_save[i][0] as usize)); }
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
static s5pv210_audss_clk_syscore_ops: syscore_ops = syscore_ops { suspend: Some(s5pv210_audss_clk_suspend), resume: Some(s5pv210_audss_clk_resume) };
#[cfg(feature = "CONFIG_PM_SLEEP")]
static mut s5pv210_audss_clk_syscore: syscore = syscore { ops: &s5pv210_audss_clk_syscore_ops };

// register s5pv210_audss clocks
unsafe extern "C" fn s5pv210_audss_clk_probe(pdev: *mut platform_device) -> i32 {
    let mut ret = 0;
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if reg.is_null() { return -1; }
    reg_base = reg;
    let data = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<clk_hw_onecell_data>() + ASS_MAX_CLKS * core::mem::size_of::<*mut clk_hw>(), 0) as *mut clk_hw_onecell_data;
    if data.is_null() { return -12; }
    clk_data = data;
    (*data).num = ASS_MAX_CLKS;
    let table = (*data).hws.as_mut_ptr();
    let hclk = devm_clk_get(&mut (*pdev).dev, b"hclk\0".as_ptr() as *const _);
    if hclk.is_null() { return -1; }
    let pll_in = devm_clk_get(&mut (*pdev).dev, b"fout_epll\0".as_ptr() as *const _);
    if pll_in.is_null() { return -1; }
    let sclk_audio = devm_clk_get(&mut (*pdev).dev, b"sclk_audio0\0".as_ptr() as *const _);
    if sclk_audio.is_null() { return -1; }
    let cdclk = devm_clk_get(&mut (*pdev).dev, b"iiscdclk0\0".as_ptr() as *const _);
    let pll_ref = devm_clk_get(&mut (*pdev).dev, b"xxti\0".as_ptr() as *const _);
    let mout_audss_p = [if pll_ref.is_null() { b"xxti\0".as_ptr() as *const _ } else { __clk_get_name(pll_ref) }, __clk_get_name(pll_in)];
    *table.add(CLK_MOUT_AUDSS) = clk_hw_register_mux(core::ptr::null_mut(), b"mout_audss\0".as_ptr() as *const _, mout_audss_p.as_ptr(), 2, 0, reg_base.add(ASS_CLK_SRC), 0, 1, 0, &mut lock);
    let mout_i2s_p = [b"mout_audss\0".as_ptr() as *const _, if cdclk.is_null() { b"iiscdclk0\0".as_ptr() as *const _ } else { __clk_get_name(cdclk) }, __clk_get_name(sclk_audio)];
    *table.add(CLK_MOUT_I2S_A) = clk_hw_register_mux(core::ptr::null_mut(), b"mout_i2s_audss\0".as_ptr() as *const _, mout_i2s_p.as_ptr(), 3, 0, reg_base.add(ASS_CLK_SRC), 2, 2, 0, &mut lock);
    *table.add(CLK_DOUT_AUD_BUS) = clk_hw_register_divider(core::ptr::null_mut(), b"dout_aud_bus\0".as_ptr() as *const _, b"mout_audss\0".as_ptr() as *const _, 0, reg_base.add(ASS_CLK_DIV), 0, 4, 0, &mut lock);
    *table.add(CLK_DOUT_I2S_A) = clk_hw_register_divider(core::ptr::null_mut(), b"dout_i2s_audss\0".as_ptr() as *const _, b"mout_i2s_audss\0".as_ptr() as *const _, 0, reg_base.add(ASS_CLK_DIV), 4, 4, 0, &mut lock);
    *table.add(CLK_I2S) = clk_hw_register_gate(core::ptr::null_mut(), b"i2s_audss\0".as_ptr() as *const _, b"dout_i2s_audss\0".as_ptr() as *const _, 0, reg_base.add(ASS_CLK_GATE), 6, 0, &mut lock);
    let hclk_p = __clk_get_name(hclk);
    let names = [b"hclk_i2s_audss\0", b"hclk_uart_audss\0", b"hclk_hwa_audss\0", b"hclk_dma_audss\0", b"hclk_buf_audss\0", b"hclk_rp_audss\0"];
    for (n, bit) in names.iter().zip((0..6).rev()) { *table.add(CLK_HCLK_I2S + (5 - bit)) = clk_hw_register_gate(core::ptr::null_mut(), n.as_ptr() as *const _, hclk_p, 0, reg_base.add(ASS_CLK_GATE), bit, 0, &mut lock); }
    for i in 0..(*data).num { if (*table.add(i)).is_null() { ret = -1; break; } }
    #[cfg(feature = "CONFIG_PM_SLEEP")] register_syscore(&mut s5pv210_audss_clk_syscore);
    ret
}

static s5pv210_audss_clk_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"samsung,s5pv210-audss-clock\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

static mut s5pv210_audss_clk_driver: platform_driver = platform_driver {
    driver: driver { name: b"s5pv210-audss-clk\0".as_ptr() as *const _, suppress_bind_attrs: true, of_match_table: s5pv210_audss_clk_of_match.as_ptr() },
    probe: Some(s5pv210_audss_clk_probe),
};

unsafe extern "C" fn s5pv210_audss_clk_init() -> i32 {
    platform_driver_register(&mut s5pv210_audss_clk_driver)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
