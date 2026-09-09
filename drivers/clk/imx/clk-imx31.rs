// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2012 Sascha Hauer <kernel@pengutronix.de>
 */

// Linux kernel headers and "clk.h" provide the declarations used below.

const MX31_CCM_BASE_ADDR: usize = 0x53f80000;
const MX31_GPT1_BASE_ADDR: usize = 0x53f90000;
const MX31_INT_GPT: usize = NR_IRQS_LEGACY + 29;

const MXC_CCM_CCMR: usize = 0x00;
const MXC_CCM_PDR0: usize = 0x04;
const MXC_CCM_PDR1: usize = 0x08;
const MXC_CCM_MPCTL: usize = 0x10;
const MXC_CCM_UPCTL: usize = 0x14;
const MXC_CCM_SRPCTL: usize = 0x18;
const MXC_CCM_CGR0: usize = 0x20;
const MXC_CCM_CGR1: usize = 0x24;
const MXC_CCM_CGR2: usize = 0x28;
const MXC_CCM_PMCR0: usize = 0x5c;

static MCU_MAIN_SEL: [*const core::ffi::c_char; 2] = [b"spll\0".as_ptr() as _, b"mpll\0".as_ptr() as _];
static PER_SEL: [*const core::ffi::c_char; 2] = [b"per_div\0".as_ptr() as _, b"ipg\0".as_ptr() as _];
static CSI_SEL: [*const core::ffi::c_char; 2] = [b"upll\0".as_ptr() as _, b"spll\0".as_ptr() as _];
static FIR_SEL: [*const core::ffi::c_char; 3] = [b"mcu_main\0".as_ptr() as _, b"upll\0".as_ptr() as _, b"spll\0".as_ptr() as _];

#[repr(usize)]
enum Mx31Clks {
    Dummy, Ckih, Ckil, Mpll, Spll, Upll, McuMain, Hsp, Ahb, Nfc, Ipg,
    PerDiv, Per, Csi, Fir, CsiDiv, UsbDivPre, UsbDivPost, FirDivPre,
    FirDivPost, Sdhc1Gate, Sdhc2Gate, GptGate, Epit1Gate, Epit2Gate,
    IimGate, AtaGate, SdmaGate, Cspi3Gate, RngGate, Uart1Gate, Uart2Gate,
    Ssi1Gate, I2c1Gate, I2c2Gate, I2c3Gate, HantroGate, Mstick1Gate,
    Mstick2Gate, CsiGate, RtcGate, WdogGate, PwmGate, SimGate, EctGate,
    UsbGate, KppGate, IpuGate, Uart3Gate, Uart4Gate, Uart5Gate, OwireGate,
    Ssi2Gate, Cspi1Gate, Cspi2Gate, GaccGate, EmiGate, RticGate, FiriGate,
    ClkMax,
}

static mut CLK: [*mut Clk; Mx31Clks::ClkMax as usize] = [core::ptr::null_mut(); Mx31Clks::ClkMax as usize];
static mut CLK_DATA: ClkOnecellData = ClkOnecellData { clks: core::ptr::null_mut(), clk_num: 0 };

unsafe fn _mx31_clocks_init(base: *mut u8, fref: usize) {
    CLK[Mx31Clks::Dummy as usize] = imx_clk_fixed(b"dummy\0".as_ptr() as _, 0);
    CLK[Mx31Clks::Ckih as usize] = imx_clk_fixed(b"ckih\0".as_ptr() as _, fref);
    CLK[Mx31Clks::Ckil as usize] = imx_clk_fixed(b"ckil\0".as_ptr() as _, 32768);
    CLK[Mx31Clks::Mpll as usize] = imx_clk_pllv1(IMX_PLLV1_IMX31, b"mpll\0".as_ptr() as _, b"ckih\0".as_ptr() as _, base.add(MXC_CCM_MPCTL));
    CLK[Mx31Clks::Spll as usize] = imx_clk_pllv1(IMX_PLLV1_IMX31, b"spll\0".as_ptr() as _, b"ckih\0".as_ptr() as _, base.add(MXC_CCM_SRPCTL));
    CLK[Mx31Clks::Upll as usize] = imx_clk_pllv1(IMX_PLLV1_IMX31, b"upll\0".as_ptr() as _, b"ckih\0".as_ptr() as _, base.add(MXC_CCM_UPCTL));
    CLK[Mx31Clks::McuMain as usize] = imx_clk_mux(b"mcu_main\0".as_ptr() as _, base.add(MXC_CCM_PMCR0), 31, 1, MCU_MAIN_SEL.as_ptr(), MCU_MAIN_SEL.len());
    CLK[Mx31Clks::Hsp as usize] = imx_clk_divider(b"hsp\0".as_ptr() as _, b"mcu_main\0".as_ptr() as _, base.add(MXC_CCM_PDR0), 11, 3);
    CLK[Mx31Clks::Ahb as usize] = imx_clk_divider(b"ahb\0".as_ptr() as _, b"mcu_main\0".as_ptr() as _, base.add(MXC_CCM_PDR0), 3, 3);
    CLK[Mx31Clks::Nfc as usize] = imx_clk_divider(b"nfc\0".as_ptr() as _, b"ahb\0".as_ptr() as _, base.add(MXC_CCM_PDR0), 8, 3);
    CLK[Mx31Clks::Ipg as usize] = imx_clk_divider(b"ipg\0".as_ptr() as _, b"ahb\0".as_ptr() as _, base.add(MXC_CCM_PDR0), 6, 2);
    CLK[Mx31Clks::PerDiv as usize] = imx_clk_divider(b"per_div\0".as_ptr() as _, b"upll\0".as_ptr() as _, base.add(MXC_CCM_PDR0), 16, 5);
    CLK[Mx31Clks::Per as usize] = imx_clk_mux(b"per\0".as_ptr() as _, base.add(MXC_CCM_CCMR), 24, 1, PER_SEL.as_ptr(), PER_SEL.len());
    CLK[Mx31Clks::Csi as usize] = imx_clk_mux(b"csi_sel\0".as_ptr() as _, base.add(MXC_CCM_CCMR), 25, 1, CSI_SEL.as_ptr(), CSI_SEL.len());
    CLK[Mx31Clks::Fir as usize] = imx_clk_mux(b"fir_sel\0".as_ptr() as _, base.add(MXC_CCM_CCMR), 11, 2, FIR_SEL.as_ptr(), FIR_SEL.len());
    CLK[Mx31Clks::CsiDiv as usize] = imx_clk_divider(b"csi_div\0".as_ptr() as _, b"csi_sel\0".as_ptr() as _, base.add(MXC_CCM_PDR0), 23, 9);
    CLK[Mx31Clks::UsbDivPre as usize] = imx_clk_divider(b"usb_div_pre\0".as_ptr() as _, b"upll\0".as_ptr() as _, base.add(MXC_CCM_PDR1), 30, 2);
    CLK[Mx31Clks::UsbDivPost as usize] = imx_clk_divider(b"usb_div_post\0".as_ptr() as _, b"usb_div_pre\0".as_ptr() as _, base.add(MXC_CCM_PDR1), 27, 3);
    CLK[Mx31Clks::FirDivPre as usize] = imx_clk_divider(b"fir_div_pre\0".as_ptr() as _, b"fir_sel\0".as_ptr() as _, base.add(MXC_CCM_PDR1), 24, 3);
    CLK[Mx31Clks::FirDivPost as usize] = imx_clk_divider(b"fir_div_post\0".as_ptr() as _, b"fir_div_pre\0".as_ptr() as _, base.add(MXC_CCM_PDR1), 23, 6);

    // The remaining clock gates preserve the original gate names, parents, registers, and bit positions.
    macro_rules! gate { ($n:ident, $name:literal, $parent:literal, $reg:ident, $bit:expr) => {
        CLK[Mx31Clks::$n as usize] = imx_clk_gate2(concat!($name, "\0").as_ptr() as _, concat!($parent, "\0").as_ptr() as _, base.add($reg), $bit);
    }; }
    gate!(Sdhc1Gate, "sdhc1_gate", "per", MXC_CCM_CGR0, 0); gate!(Sdhc2Gate, "sdhc2_gate", "per", MXC_CCM_CGR0, 2); gate!(GptGate, "gpt_gate", "per", MXC_CCM_CGR0, 4); gate!(Epit1Gate, "epit1_gate", "per", MXC_CCM_CGR0, 6); gate!(Epit2Gate, "epit2_gate", "per", MXC_CCM_CGR0, 8);
    gate!(IimGate, "iim_gate", "ipg", MXC_CCM_CGR0, 10); gate!(AtaGate, "ata_gate", "ipg", MXC_CCM_CGR0, 12); gate!(SdmaGate, "sdma_gate", "ahb", MXC_CCM_CGR0, 14); gate!(Cspi3Gate, "cspi3_gate", "ipg", MXC_CCM_CGR0, 16); gate!(RngGate, "rng_gate", "ipg", MXC_CCM_CGR0, 18);
    gate!(Uart1Gate, "uart1_gate", "per", MXC_CCM_CGR0, 20); gate!(Uart2Gate, "uart2_gate", "per", MXC_CCM_CGR0, 22); gate!(Ssi1Gate, "ssi1_gate", "spll", MXC_CCM_CGR0, 24); gate!(I2c1Gate, "i2c1_gate", "per", MXC_CCM_CGR0, 26); gate!(I2c2Gate, "i2c2_gate", "per", MXC_CCM_CGR0, 28); gate!(I2c3Gate, "i2c3_gate", "per", MXC_CCM_CGR0, 30);
    gate!(HantroGate, "hantro_gate", "per", MXC_CCM_CGR1, 0); gate!(Mstick1Gate, "mstick1_gate", "per", MXC_CCM_CGR1, 2); gate!(Mstick2Gate, "mstick2_gate", "per", MXC_CCM_CGR1, 4); gate!(CsiGate, "csi_gate", "csi_div", MXC_CCM_CGR1, 6); gate!(RtcGate, "rtc_gate", "ipg", MXC_CCM_CGR1, 8); gate!(WdogGate, "wdog_gate", "ipg", MXC_CCM_CGR1, 10); gate!(PwmGate, "pwm_gate", "per", MXC_CCM_CGR1, 12); gate!(SimGate, "sim_gate", "per", MXC_CCM_CGR1, 14); gate!(EctGate, "ect_gate", "per", MXC_CCM_CGR1, 16); gate!(UsbGate, "usb_gate", "ahb", MXC_CCM_CGR1, 18); gate!(KppGate, "kpp_gate", "ipg", MXC_CCM_CGR1, 20); gate!(IpuGate, "ipu_gate", "hsp", MXC_CCM_CGR1, 22); gate!(Uart3Gate, "uart3_gate", "per", MXC_CCM_CGR1, 24); gate!(Uart4Gate, "uart4_gate", "per", MXC_CCM_CGR1, 26); gate!(Uart5Gate, "uart5_gate", "per", MXC_CCM_CGR1, 28); gate!(OwireGate, "owire_gate", "per", MXC_CCM_CGR1, 30);
    gate!(Ssi2Gate, "ssi2_gate", "spll", MXC_CCM_CGR2, 0); gate!(Cspi1Gate, "cspi1_gate", "ipg", MXC_CCM_CGR2, 2); gate!(Cspi2Gate, "cspi2_gate", "ipg", MXC_CCM_CGR2, 4); gate!(GaccGate, "gacc_gate", "per", MXC_CCM_CGR2, 6); gate!(EmiGate, "emi_gate", "ahb", MXC_CCM_CGR2, 8); gate!(RticGate, "rtic_gate", "ahb", MXC_CCM_CGR2, 10); gate!(FiriGate, "firi_gate", "upll", MXC_CCM_CGR2, 12);
    imx_check_clocks(CLK.as_mut_ptr(), CLK.len());
    clk_set_parent(CLK[Mx31Clks::Csi as usize], CLK[Mx31Clks::Upll as usize]);
    clk_prepare_enable(CLK[Mx31Clks::EmiGate as usize]); clk_prepare_enable(CLK[Mx31Clks::IimGate as usize]);
    mx31_revision(); clk_disable_unprepare(CLK[Mx31Clks::IimGate as usize]);
}

// Device-tree initialization and declaration are retained as external integration points.
unsafe fn mx31_clocks_init_dt(np: *mut DeviceNode) {
    let mut fref: u32 = 26000000;
    // for_each_compatible_node_scoped(osc_np, NULL, "fixed-clock")
    for osc_np in for_each_compatible_node_scoped(core::ptr::null_mut(), b"fixed-clock\0".as_ptr() as _) {
        if !of_device_is_compatible(osc_np, b"fsl,imx-osc26m\0".as_ptr() as _) { continue; }
        if of_property_read_u32(osc_np, b"clock-frequency\0".as_ptr() as _, &mut fref) == 0 { break; }
    }
    let ccm = of_iomap(np, 0);
    if ccm.is_null() { panic!("%s: failed to map registers\n", "mx31_clocks_init_dt"); }
    _mx31_clocks_init(ccm, fref as usize);
    CLK_DATA.clks = CLK.as_mut_ptr(); CLK_DATA.clk_num = CLK.len();
    of_clk_add_provider(np, of_clk_src_onecell_get, &mut CLK_DATA);
}

// CLK_OF_DECLARE(imx31_ccm, "fsl,imx31-ccm", mx31_clocks_init_dt);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
