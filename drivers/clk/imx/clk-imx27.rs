// SPDX-License-Identifier: GPL-2.0
// Translated from clk-imx27.c. Kernel includes and externally supplied symbols
// are intentionally left as external dependencies.

const MX27_CCM_BASE_ADDR: usize = 0x10027000;
const MX27_GPT1_BASE_ADDR: usize = 0x10003000;
const MX27_INT_GPT1: usize = NR_IRQS_LEGACY + 26;

static mut ccm: *mut core::ffi::c_void = core::ptr::null_mut();

// Register offsets
macro_rules! CCM_CSCR { () => { unsafe { (ccm as *mut u8).add(0x00) } }; }
macro_rules! CCM_MPCTL0 { () => { unsafe { (ccm as *mut u8).add(0x04) } }; }
macro_rules! CCM_MPCTL1 { () => { unsafe { (ccm as *mut u8).add(0x08) } }; }
macro_rules! CCM_SPCTL0 { () => { unsafe { (ccm as *mut u8).add(0x0c) } }; }
macro_rules! CCM_SPCTL1 { () => { unsafe { (ccm as *mut u8).add(0x10) } }; }
macro_rules! CCM_PCDR0 { () => { unsafe { (ccm as *mut u8).add(0x18) } }; }
macro_rules! CCM_PCDR1 { () => { unsafe { (ccm as *mut u8).add(0x1c) } }; }
macro_rules! CCM_PCCR0 { () => { unsafe { (ccm as *mut u8).add(0x20) } }; }
macro_rules! CCM_PCCR1 { () => { unsafe { (ccm as *mut u8).add(0x24) } }; }
macro_rules! CCM_CCSR { () => { unsafe { (ccm as *mut u8).add(0x28) } }; }

static mut vpu_sel_clks: [&'static core::ffi::CStr; 2] = [cstr("spll"), cstr("mpll_main2")];
static mut cpu_sel_clks: [&'static core::ffi::CStr; 2] = [cstr("mpll_main2"), cstr("mpll")];
static mut mpll_sel_clks: [&'static core::ffi::CStr; 2] = [cstr("fpm"), cstr("mpll_osc_sel")];
static mut mpll_osc_sel_clks: [&'static core::ffi::CStr; 2] = [cstr("ckih_gate"), cstr("ckih_div1p5")];
static mut clko_sel_clks: [&'static core::ffi::CStr; 23] = [cstr("ckil"), cstr("fpm"), cstr("ckih_gate"), cstr("ckih_gate"), cstr("ckih_gate"), cstr("mpll"), cstr("spll"), cstr("cpu_div"), cstr("ahb"), cstr("ipg"), cstr("per1_div"), cstr("per2_div"), cstr("per3_div"), cstr("per4_div"), cstr("ssi1_div"), cstr("ssi2_div"), cstr("nfc_div"), cstr("mshc_div"), cstr("vpu_div"), cstr("60m"), cstr("32k"), cstr("usb_div"), cstr("dptc")];
static mut ssi_sel_clks: [&'static core::ffi::CStr; 2] = [cstr("spll_gate"), cstr("mpll")];

static mut clk: [*mut clk; IMX27_CLK_MAX as usize] = [core::ptr::null_mut(); IMX27_CLK_MAX as usize];
static mut clk_data: clk_onecell_data = clk_onecell_data { clks: core::ptr::null_mut(), clk_num: 0 };

unsafe fn _mx27_clocks_init(fref: usize) {
    BUG_ON(ccm.is_null());
    clk[IMX27_CLK_DUMMY as usize] = imx_clk_fixed(cstr("dummy"), 0);
    clk[IMX27_CLK_CKIH as usize] = imx_clk_fixed(cstr("ckih"), fref);
    clk[IMX27_CLK_CKIL as usize] = imx_clk_fixed(cstr("ckil"), 32768);
    clk[IMX27_CLK_FPM as usize] = imx_clk_fixed_factor(cstr("fpm"), cstr("ckil"), 1024, 1);
    clk[IMX27_CLK_CKIH_DIV1P5 as usize] = imx_clk_fixed_factor(cstr("ckih_div1p5"), cstr("ckih_gate"), 2, 3);
    clk[IMX27_CLK_CKIH_GATE as usize] = imx_clk_gate_dis(cstr("ckih_gate"), cstr("ckih"), CCM_CSCR!(), 3);
    clk[IMX27_CLK_MPLL_OSC_SEL as usize] = imx_clk_mux(cstr("mpll_osc_sel"), CCM_CSCR!(), 4, 1, mpll_osc_sel_clks.as_mut_ptr(), 2);
    clk[IMX27_CLK_MPLL_SEL as usize] = imx_clk_mux(cstr("mpll_sel"), CCM_CSCR!(), 16, 1, mpll_sel_clks.as_mut_ptr(), 2);
    clk[IMX27_CLK_MPLL as usize] = imx_clk_pllv1(IMX_PLLV1_IMX27, cstr("mpll"), cstr("mpll_sel"), CCM_MPCTL0!());
    clk[IMX27_CLK_SPLL as usize] = imx_clk_pllv1(IMX_PLLV1_IMX27, cstr("spll"), cstr("ckih_gate"), CCM_SPCTL0!());
    clk[IMX27_CLK_SPLL_GATE as usize] = imx_clk_gate(cstr("spll_gate"), cstr("spll"), CCM_CSCR!(), 1);
    clk[IMX27_CLK_MPLL_MAIN2 as usize] = imx_clk_fixed_factor(cstr("mpll_main2"), cstr("mpll"), 2, 3);
    if mx27_revision() >= IMX_CHIP_REVISION_2_0 { clk[IMX27_CLK_AHB as usize] = imx_clk_divider(cstr("ahb"), cstr("mpll_main2"), CCM_CSCR!(), 8, 2); clk[IMX27_CLK_IPG as usize] = imx_clk_fixed_factor(cstr("ipg"), cstr("ahb"), 1, 2); }
    else { clk[IMX27_CLK_AHB as usize] = imx_clk_divider(cstr("ahb"), cstr("mpll_main2"), CCM_CSCR!(), 9, 4); clk[IMX27_CLK_IPG as usize] = imx_clk_divider(cstr("ipg"), cstr("ahb"), CCM_CSCR!(), 8, 1); }
    macro_rules! div { ($i:expr,$n:expr,$p:expr,$r:expr,$s:expr) => { clk[$i as usize] = imx_clk_divider(cstr($n), cstr($p), $r, $s.0, $s.1); }; }
    macro_rules! gate { ($i:expr,$n:expr,$p:expr,$r:expr,$b:expr) => { clk[$i as usize] = imx_clk_gate(cstr($n), cstr($p), $r, $b); }; }
    div!(IMX27_CLK_MSHC_DIV,"mshc_div","ahb",CCM_PCDR0!(),(0,6)); div!(IMX27_CLK_NFC_DIV,"nfc_div","ahb",CCM_PCDR0!(),(6,4));
    div!(IMX27_CLK_PER1_DIV,"per1_div","mpll_main2",CCM_PCDR1!(),(0,6)); div!(IMX27_CLK_PER2_DIV,"per2_div","mpll_main2",CCM_PCDR1!(),(8,6)); div!(IMX27_CLK_PER3_DIV,"per3_div","mpll_main2",CCM_PCDR1!(),(16,6)); div!(IMX27_CLK_PER4_DIV,"per4_div","mpll_main2",CCM_PCDR1!(),(24,6));
    clk[IMX27_CLK_VPU_SEL as usize]=imx_clk_mux(cstr("vpu_sel"),CCM_CSCR!(),21,1,vpu_sel_clks.as_mut_ptr(),2); div!(IMX27_CLK_VPU_DIV,"vpu_div","vpu_sel",CCM_PCDR0!(),(10,6)); div!(IMX27_CLK_USB_DIV,"usb_div","spll_gate",CCM_CSCR!(),(28,3));
    clk[IMX27_CLK_CPU_SEL as usize]=imx_clk_mux(cstr("cpu_sel"),CCM_CSCR!(),15,1,cpu_sel_clks.as_mut_ptr(),2); clk[IMX27_CLK_CLKO_SEL as usize]=imx_clk_mux(cstr("clko_sel"),CCM_CCSR!(),0,5,clko_sel_clks.as_mut_ptr(),23);
    if mx27_revision() >= IMX_CHIP_REVISION_2_0 { div!(IMX27_CLK_CPU_DIV,"cpu_div","cpu_sel",CCM_CSCR!(),(12,2)); } else { div!(IMX27_CLK_CPU_DIV,"cpu_div","cpu_sel",CCM_CSCR!(),(13,3)); }
    div!(IMX27_CLK_CLKO_DIV,"clko_div","clko_sel",CCM_PCDR0!(),(22,3)); clk[IMX27_CLK_SSI1_SEL as usize]=imx_clk_mux(cstr("ssi1_sel"),CCM_CSCR!(),22,1,ssi_sel_clks.as_mut_ptr(),2); clk[IMX27_CLK_SSI2_SEL as usize]=imx_clk_mux(cstr("ssi2_sel"),CCM_CSCR!(),23,1,ssi_sel_clks.as_mut_ptr(),2); div!(IMX27_CLK_SSI1_DIV,"ssi1_div","ssi1_sel",CCM_PCDR0!(),(16,6)); div!(IMX27_CLK_SSI2_DIV,"ssi2_div","ssi2_sel",CCM_PCDR0!(),(26,6));
    let gates = [(IMX27_CLK_CLKO_EN,"clko_en","clko_div",CCM_PCCR0!(),0),(IMX27_CLK_SSI2_IPG_GATE,"ssi2_ipg_gate","ipg",CCM_PCCR0!(),0),(IMX27_CLK_SSI1_IPG_GATE,"ssi1_ipg_gate","ipg",CCM_PCCR0!(),1),(IMX27_CLK_SLCDC_IPG_GATE,"slcdc_ipg_gate","ipg",CCM_PCCR0!(),2),(IMX27_CLK_SDHC3_IPG_GATE,"sdhc3_ipg_gate","ipg",CCM_PCCR0!(),3),(IMX27_CLK_SDHC2_IPG_GATE,"sdhc2_ipg_gate","ipg",CCM_PCCR0!(),4),(IMX27_CLK_SDHC1_IPG_GATE,"sdhc1_ipg_gate","ipg",CCM_PCCR0!(),5),(IMX27_CLK_SCC_IPG_GATE,"scc_ipg_gate","ipg",CCM_PCCR0!(),6),(IMX27_CLK_SAHARA_IPG_GATE,"sahara_ipg_gate","ipg",CCM_PCCR0!(),7),(IMX27_CLK_RTIC_IPG_GATE,"rtic_ipg_gate","ipg",CCM_PCCR0!(),8),(IMX27_CLK_RTC_IPG_GATE,"rtc_ipg_gate","ipg",CCM_PCCR0!(),9),(IMX27_CLK_PWM_IPG_GATE,"pwm_ipg_gate","ipg",CCM_PCCR0!(),11),(IMX27_CLK_OWIRE_IPG_GATE,"owire_ipg_gate","ipg",CCM_PCCR0!(),12),(IMX27_CLK_MSHC_IPG_GATE,"mshc_ipg_gate","ipg",CCM_PCCR0!(),13),(IMX27_CLK_LCDC_IPG_GATE,"lcdc_ipg_gate","ipg",CCM_PCCR0!(),14),(IMX27_CLK_KPP_IPG_GATE,"kpp_ipg_gate","ipg",CCM_PCCR0!(),15),(IMX27_CLK_IIM_IPG_GATE,"iim_ipg_gate","ipg",CCM_PCCR0!(),16),(IMX27_CLK_I2C2_IPG_GATE,"i2c2_ipg_gate","ipg",CCM_PCCR0!(),17),(IMX27_CLK_I2C1_IPG_GATE,"i2c1_ipg_gate","ipg",CCM_PCCR0!(),18)]; for (i,n,p,r,b) in gates { gate!(i,n,p,r,b); }
    imx_check_clocks(clk.as_mut_ptr(), clk.len()); clk_register_clkdev(clk[IMX27_CLK_CPU_DIV as usize], core::ptr::null(), cstr("cpu0")); clk_prepare_enable(clk[IMX27_CLK_EMI_AHB_GATE as usize]); imx_register_uart_clocks(); imx_print_silicon_rev(cstr("i.MX27"), mx27_revision());
}

unsafe fn mx27_clocks_init_dt(np: *mut device_node) { let mut fref: u32=26000000; ccm=of_iomap(np,0); _mx27_clocks_init(fref as usize); clk_data.clks=clk.as_mut_ptr(); clk_data.clk_num=clk.len(); of_clk_add_provider(np,of_clk_src_onecell_get,&mut clk_data); }
// CLK_OF_DECLARE(imx27_ccm, "fsl,imx27-ccm", mx27_clocks_init_dt)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
