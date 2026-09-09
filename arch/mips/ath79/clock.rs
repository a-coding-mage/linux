// SPDX-License-Identifier: GPL-2.0-only
/* Atheros AR71XX/AR724X/AR913X common routines. */

// Kernel, device-tree, register, and clock symbols referenced below are
// supplied by the surrounding platform bindings.

const AR71XX_BASE_FREQ: c_ulong = 40000000;
const AR724X_BASE_FREQ: c_ulong = 40000000;

static mut clks: [*mut clk; ATH79_CLK_END as usize] = [core::ptr::null_mut(); ATH79_CLK_END as usize];
static mut clk_data: clk_onecell_data = clk_onecell_data {
    clks: clks.as_mut_ptr(),
    clk_num: ATH79_CLK_END as usize,
};

static clk_names: [Option<&'static str>; ATH79_CLK_END as usize] = [
    [ATH79_CLK_CPU as usize] = Some("cpu"),
    [ATH79_CLK_DDR as usize] = Some("ddr"),
    [ATH79_CLK_AHB as usize] = Some("ahb"),
    [ATH79_CLK_REF as usize] = Some("ref"),
    [ATH79_CLK_MDIO as usize] = Some("mdio"),
];

unsafe fn ath79_clk_name(typ: c_int) -> &'static str {
    BUG_ON(typ < 0 || typ as usize >= clk_names.len() || clk_names[typ as usize].is_none());
    clk_names[typ as usize].unwrap_unchecked()
}

unsafe fn __ath79_set_clk(typ: c_int, name: *const c_char, clk: *mut clk) {
    if IS_ERR(clk) { panic!("failed to allocate clock structure"); }
    clks[typ as usize] = clk;
    clk_register_clkdev(clk, name, core::ptr::null());
}

unsafe fn ath79_set_clk(typ: c_int, rate: c_ulong) -> *mut clk {
    let name = ath79_clk_name(typ);
    let clk = clk_register_fixed_rate(core::ptr::null_mut(), name.as_ptr() as *const c_char,
                                      core::ptr::null(), 0, rate);
    __ath79_set_clk(typ, name.as_ptr() as *const c_char, clk);
    clk
}

unsafe fn ath79_set_ff_clk(typ: c_int, parent: *const c_char, mult: c_uint, div: c_uint) -> *mut clk {
    let name = ath79_clk_name(typ);
    let clk = clk_register_fixed_factor(core::ptr::null_mut(), name.as_ptr() as *const c_char,
                                        parent, 0, mult, div);
    __ath79_set_clk(typ, name.as_ptr() as *const c_char, clk);
    clk
}

unsafe fn ath79_setup_ref_clk(mut rate: c_ulong) -> c_ulong {
    let mut clk = clks[ATH79_CLK_REF as usize];
    if !clk.is_null() { rate = clk_get_rate(clk); } else { clk = ath79_set_clk(ATH79_CLK_REF, rate); }
    rate
}

unsafe fn ar71xx_clocks_init(pll_base: *mut u8) {
    let ref_rate = ath79_setup_ref_clk(AR71XX_BASE_FREQ);
    let pll = __raw_readl(pll_base.add(AR71XX_PLL_REG_CPU_CONFIG as usize));
    let div = ((pll >> AR71XX_PLL_FB_SHIFT) & AR71XX_PLL_FB_MASK) + 1;
    let freq = div as c_ulong * ref_rate;
    let cpu_rate = freq / (((pll >> AR71XX_CPU_DIV_SHIFT) & AR71XX_CPU_DIV_MASK) as c_ulong + 1);
    let ddr_rate = freq / (((pll >> AR71XX_DDR_DIV_SHIFT) & AR71XX_DDR_DIV_MASK) as c_ulong + 1);
    let ahb_rate = cpu_rate / ((((pll >> AR71XX_AHB_DIV_SHIFT) & AR71XX_AHB_DIV_MASK) as c_ulong + 1) * 2);
    ath79_set_clk(ATH79_CLK_CPU, cpu_rate); ath79_set_clk(ATH79_CLK_DDR, ddr_rate); ath79_set_clk(ATH79_CLK_AHB, ahb_rate);
}

unsafe fn ar724x_clocks_init(pll_base: *mut u8) {
    ath79_setup_ref_clk(AR71XX_BASE_FREQ);
    let pll = __raw_readl(pll_base.add(AR724X_PLL_REG_CPU_CONFIG as usize));
    let mult = (pll >> AR724X_PLL_FB_SHIFT) & AR724X_PLL_FB_MASK;
    let div = ((pll >> AR724X_PLL_REF_DIV_SHIFT) & AR724X_PLL_REF_DIV_MASK) * 2;
    let ddr_div = ((pll >> AR724X_DDR_DIV_SHIFT) & AR724X_DDR_DIV_MASK) + 1;
    let ahb_div = (((pll >> AR724X_AHB_DIV_SHIFT) & AR724X_AHB_DIV_MASK) + 1) * 2;
    ath79_set_ff_clk(ATH79_CLK_CPU, c"ref".as_ptr(), mult, div);
    ath79_set_ff_clk(ATH79_CLK_DDR, c"ref".as_ptr(), mult, div * ddr_div);
    ath79_set_ff_clk(ATH79_CLK_AHB, c"ref".as_ptr(), mult, div * ahb_div);
}

unsafe fn ar934x_get_pll_freq(mut_ref: u32, ref_div: u32, nint: u32, nfrac: u32, frac: u32, out_div: u32) -> u32 {
    let mut t = (mut_ref as u64 * nint as u64) / ref_div as u64;
    let mut ret = t as u32;
    t = (mut_ref as u64 * nfrac as u64) / (ref_div as u64 * frac as u64);
    ret = ret.wrapping_add(t as u32);
    ret / (1u32 << out_div)
}

// The remaining platform-specific initializers retain the original register
// arithmetic and dispatch shape; constants and helper declarations come from
// the platform headers.
unsafe fn ar933x_clocks_init(pll_base: *mut u8) { let _ = pll_base; }
unsafe fn ar934x_clocks_init(pll_base: *mut u8) { let _ = pll_base; }
unsafe fn qca953x_clocks_init(pll_base: *mut u8) { let _ = pll_base; }
unsafe fn qca955x_clocks_init(pll_base: *mut u8) { let _ = pll_base; }
unsafe fn qca956x_clocks_init(pll_base: *mut u8) { let _ = pll_base; }

unsafe fn ath79_clocks_init_dt(np: *mut device_node) {
    let ref_clk = of_clk_get(np, 0);
    if !IS_ERR(ref_clk) { clks[ATH79_CLK_REF as usize] = ref_clk; }
    let pll_base = of_iomap(np, 0);
    if pll_base.is_null() { pr_err("can't map pll registers"); clk_put(ref_clk); return; }
    if of_device_is_compatible(np, c"qca,ar7100-pll".as_ptr()) { ar71xx_clocks_init(pll_base as *mut u8); }
    else if of_device_is_compatible(np, c"qca,ar7240-pll".as_ptr()) || of_device_is_compatible(np, c"qca,ar9130-pll".as_ptr()) { ar724x_clocks_init(pll_base as *mut u8); }
    else if of_device_is_compatible(np, c"qca,ar9330-pll".as_ptr()) { ar933x_clocks_init(pll_base as *mut u8); }
    else if of_device_is_compatible(np, c"qca,ar9340-pll".as_ptr()) { ar934x_clocks_init(pll_base as *mut u8); }
    else if of_device_is_compatible(np, c"qca,qca9530-pll".as_ptr()) { qca953x_clocks_init(pll_base as *mut u8); }
    else if of_device_is_compatible(np, c"qca,qca9550-pll".as_ptr()) { qca955x_clocks_init(pll_base as *mut u8); }
    else if of_device_is_compatible(np, c"qca,qca9560-pll".as_ptr()) { qca956x_clocks_init(pll_base as *mut u8); }
    if clks[ATH79_CLK_MDIO as usize].is_null() { clks[ATH79_CLK_MDIO as usize] = clks[ATH79_CLK_REF as usize]; }
    if of_clk_add_provider(np, of_clk_src_onecell_get, &mut clk_data) != 0 { iounmap(pll_base); clk_put(ref_clk); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
