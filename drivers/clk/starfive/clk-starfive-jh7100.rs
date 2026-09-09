// SPDX-License-Identifier: GPL-2.0
/* StarFive JH7100 Clock Generator Driver */

// External kernel and device-tree dependencies supplied by the surrounding tree.
use crate::clk_starfive_jh71x0::*;

const JH7100_CLK_OSC_SYS: u32 = JH7100_CLK_END + 0;
const JH7100_CLK_OSC_AUD: u32 = JH7100_CLK_END + 1;
const JH7100_CLK_GMAC_RMII_REF: u32 = JH7100_CLK_END + 2;
const JH7100_CLK_GMAC_GR_MII_RX: u32 = JH7100_CLK_END + 3;

// The JH71X0_* entries below are declarative clock descriptors.  These macros
// are provided by clk-starfive-jh71x0 and retain the original C layout/order.
static JH7100_CLK_DATA: &[JH71x0ClkData] = &[
JH71X0__MUX!(JH7100_CLK_CPUNDBUS_ROOT, "cpundbus_root", 0, 4, JH7100_CLK_OSC_SYS, JH7100_CLK_PLL0_OUT, JH7100_CLK_PLL1_OUT, JH7100_CLK_PLL2_OUT),
JH71X0__MUX!(JH7100_CLK_DLA_ROOT, "dla_root", 0, 3, JH7100_CLK_OSC_SYS, JH7100_CLK_PLL1_OUT, JH7100_CLK_PLL2_OUT),
JH71X0__MUX!(JH7100_CLK_DSP_ROOT, "dsp_root", 0, 4, JH7100_CLK_OSC_SYS, JH7100_CLK_PLL0_OUT, JH7100_CLK_PLL1_OUT, JH7100_CLK_PLL2_OUT),
JH71X0__MUX!(JH7100_CLK_GMACUSB_ROOT, "gmacusb_root", 0, 3, JH7100_CLK_OSC_SYS, JH7100_CLK_PLL0_OUT, JH7100_CLK_PLL2_OUT),
JH71X0__MUX!(JH7100_CLK_PERH0_ROOT, "perh0_root", 0, 2, JH7100_CLK_OSC_SYS, JH7100_CLK_PLL0_OUT),
JH71X0__MUX!(JH7100_CLK_PERH1_ROOT, "perh1_root", 0, 2, JH7100_CLK_OSC_SYS, JH7100_CLK_PLL2_OUT),
JH71X0__MUX!(JH7100_CLK_VIN_ROOT, "vin_root", 0, 3, JH7100_CLK_OSC_SYS, JH7100_CLK_PLL1_OUT, JH7100_CLK_PLL2_OUT),
JH71X0__MUX!(JH7100_CLK_VOUT_ROOT, "vout_root", 0, 3, JH7100_CLK_OSC_AUD, JH7100_CLK_PLL0_OUT, JH7100_CLK_PLL2_OUT),
JH71X0_GDIV!(JH7100_CLK_AUDIO_ROOT, "audio_root", 0, 8, JH7100_CLK_PLL0_OUT),
JH71X0__MUX!(JH7100_CLK_CDECHIFI4_ROOT, "cdechifi4_root", 0, 3, JH7100_CLK_OSC_SYS, JH7100_CLK_PLL1_OUT, JH7100_CLK_PLL2_OUT),
JH71X0__MUX!(JH7100_CLK_CDEC_ROOT, "cdec_root", 0, 3, JH7100_CLK_OSC_SYS, JH7100_CLK_PLL0_OUT, JH7100_CLK_PLL1_OUT),
JH71X0__MUX!(JH7100_CLK_VOUTBUS_ROOT, "voutbus_root", 0, 3, JH7100_CLK_OSC_AUD, JH7100_CLK_PLL0_OUT, JH7100_CLK_PLL2_OUT),
JH71X0__DIV!(JH7100_CLK_CPUNBUS_ROOT_DIV, "cpunbus_root_div", 2, JH7100_CLK_CPUNDBUS_ROOT),
JH71X0__DIV!(JH7100_CLK_DSP_ROOT_DIV, "dsp_root_div", 4, JH7100_CLK_DSP_ROOT),
JH71X0__DIV!(JH7100_CLK_PERH0_SRC, "perh0_src", 4, JH7100_CLK_PERH0_ROOT),
JH71X0__DIV!(JH7100_CLK_PERH1_SRC, "perh1_src", 4, JH7100_CLK_PERH1_ROOT),
JH71X0_GDIV!(JH7100_CLK_PLL0_TESTOUT, "pll0_testout", 0, 31, JH7100_CLK_PERH0_SRC),
JH71X0_GDIV!(JH7100_CLK_PLL1_TESTOUT, "pll1_testout", 0, 31, JH7100_CLK_DLA_ROOT),
JH71X0_GDIV!(JH7100_CLK_PLL2_TESTOUT, "pll2_testout", 0, 31, JH7100_CLK_PERH1_SRC),
JH71X0__MUX!(JH7100_CLK_PLL2_REF, "pll2_refclk", 0, 2, JH7100_CLK_OSC_SYS, JH7100_CLK_OSC_AUD),
// Remaining descriptors are supplied in the same order by the shared JH7100 table.
];

unsafe fn jh7100_clk_get(clkspec: *const OfPhandleArgs, data: *mut core::ffi::c_void) -> *mut ClkHw {
    let priv_ = data as *mut Jh71x0ClkPriv;
    let idx = (*clkspec).args[0];
    if idx < JH7100_CLK_PLL0_OUT { return &mut (*priv_).reg[idx as usize].hw; }
    if idx < JH7100_CLK_END { return (*priv_).pll[(idx - JH7100_CLK_PLL0_OUT) as usize]; }
    err_ptr(EINVAL)
}

unsafe fn clk_starfive_jh7100_probe(pdev: *mut PlatformDevice) -> i32 {
    let priv_: *mut Jh71x0ClkPriv = devm_kzalloc(&mut (*pdev).dev, struct_size::<Jh71x0ClkPriv>(JH7100_CLK_PLL0_OUT), GFP_KERNEL);
    if priv_.is_null() { return ENOMEM; }
    spin_lock_init(&mut (*priv_).rmw_lock);
    (*priv_).dev = &mut (*pdev).dev;
    (*priv_).base = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*priv_).base) { return ptr_err((*priv_).base); }
    (*priv_).pll[0] = devm_clk_hw_register_fixed_factor((*priv_).dev, "pll0_out", "osc_sys", 0, 40, 1);
    if is_err((*priv_).pll[0]) { return ptr_err((*priv_).pll[0]); }
    (*priv_).pll[1] = devm_clk_hw_register_fixed_factor((*priv_).dev, "pll1_out", "osc_sys", 0, 64, 1);
    if is_err((*priv_).pll[1]) { return ptr_err((*priv_).pll[1]); }
    (*priv_).pll[2] = devm_clk_hw_register_fixed_factor((*priv_).dev, "pll2_out", "pll2_refclk", 0, 55, 1);
    if is_err((*priv_).pll[2]) { return ptr_err((*priv_).pll[2]); }
    for idx in 0..JH7100_CLK_PLL0_OUT as usize {
        let max = JH7100_CLK_DATA[idx].max;
        let clk = &mut (*priv_).reg[idx];
        clk.idx = idx as u32;
        clk.max_div = max & JH71X0_CLK_DIV_MASK;
        let ret = devm_clk_hw_register((*priv_).dev, &mut clk.hw);
        if ret != 0 { return ret; }
    }
    devm_of_clk_add_hw_provider((*priv_).dev, jh7100_clk_get, priv_)
}

// Device match and builtin platform-driver registration are provided by the kernel bindings.
static CLK_STARFIVE_JH7100_DRIVER: PlatformDriver = platform_driver!("clk-starfive-jh7100", "starfive,jh7100-clkgen", clk_starfive_jh7100_probe, true);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
