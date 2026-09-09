// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Hi3798CV200 Clock and Reset Generator Driver
 *
 * Copyright (c) 2016 HiSilicon Technologies Co., Ltd.
 */

// External kernel and clock/reset definitions are supplied by the surrounding
// translation unit.

const HI3798CV200_INNER_CLK_OFFSET: u32 = 64;
const HI3798CV200_FIXED_24M: u32 = 65;
const HI3798CV200_FIXED_25M: u32 = 66;
const HI3798CV200_FIXED_50M: u32 = 67;
const HI3798CV200_FIXED_75M: u32 = 68;
const HI3798CV200_FIXED_100M: u32 = 69;
const HI3798CV200_FIXED_150M: u32 = 70;
const HI3798CV200_FIXED_200M: u32 = 71;
const HI3798CV200_FIXED_250M: u32 = 72;
const HI3798CV200_FIXED_300M: u32 = 73;
const HI3798CV200_FIXED_400M: u32 = 74;
const HI3798CV200_MMC_MUX: u32 = 75;
const HI3798CV200_ETH_PUB_CLK: u32 = 76;
const HI3798CV200_ETH_BUS_CLK: u32 = 77;
const HI3798CV200_ETH_BUS0_CLK: u32 = 78;
const HI3798CV200_ETH_BUS1_CLK: u32 = 79;
const HI3798CV200_COMBPHY1_MUX: u32 = 80;
const HI3798CV200_FIXED_12M: u32 = 81;
const HI3798CV200_FIXED_48M: u32 = 82;
const HI3798CV200_FIXED_60M: u32 = 83;
const HI3798CV200_FIXED_166P5M: u32 = 84;
const HI3798CV200_SDIO0_MUX: u32 = 85;
const HI3798CV200_COMBPHY0_MUX: u32 = 86;
const HI3798CV200_CRG_NR_CLKS: u32 = 128;

static HI3798CV200_FIXED_RATE_CLKS: &[HisiFixedRateClock] = &[
    HisiFixedRateClock(HISTB_OSC_CLK, "clk_osc", None, 0, 24000000),
    HisiFixedRateClock(HISTB_APB_CLK, "clk_apb", None, 0, 100000000),
    HisiFixedRateClock(HISTB_AHB_CLK, "clk_ahb", None, 0, 200000000),
    HisiFixedRateClock(HI3798CV200_FIXED_12M, "12m", None, 0, 12000000),
    HisiFixedRateClock(HI3798CV200_FIXED_24M, "24m", None, 0, 24000000),
    HisiFixedRateClock(HI3798CV200_FIXED_25M, "25m", None, 0, 25000000),
    HisiFixedRateClock(HI3798CV200_FIXED_48M, "48m", None, 0, 48000000),
    HisiFixedRateClock(HI3798CV200_FIXED_50M, "50m", None, 0, 50000000),
    HisiFixedRateClock(HI3798CV200_FIXED_60M, "60m", None, 0, 60000000),
    HisiFixedRateClock(HI3798CV200_FIXED_75M, "75m", None, 0, 75000000),
    HisiFixedRateClock(HI3798CV200_FIXED_100M, "100m", None, 0, 100000000),
    HisiFixedRateClock(HI3798CV200_FIXED_150M, "150m", None, 0, 150000000),
    HisiFixedRateClock(HI3798CV200_FIXED_166P5M, "166p5m", None, 0, 165000000),
    HisiFixedRateClock(HI3798CV200_FIXED_200M, "200m", None, 0, 200000000),
    HisiFixedRateClock(HI3798CV200_FIXED_250M, "250m", None, 0, 250000000),
];

static MMC_MUX_P: &[&str] = &["100m", "50m", "25m", "200m", "150m"];
static mut MMC_MUX_TABLE: [u32; 5] = [0, 1, 2, 3, 6];
static COMPHY_MUX_P: &[&str] = &["100m", "25m"];
static mut COMPHY_MUX_TABLE: [u32; 2] = [2, 3];
static SDIO_MUX_P: &[&str] = &["100m", "50m", "150m", "166p5m"];
static mut SDIO_MUX_TABLE: [u32; 4] = [0, 1, 2, 3];

static mut HI3798CV200_MUX_CLKS: [HisiMuxClock; 4] = [
    HisiMuxClock(HI3798CV200_MMC_MUX, "mmc_mux", MMC_MUX_P, 5, CLK_SET_RATE_PARENT, 0xa0, 8, 3, 0, unsafe { &MMC_MUX_TABLE }),
    HisiMuxClock(HI3798CV200_COMBPHY0_MUX, "combphy0_mux", COMPHY_MUX_P, 2, CLK_SET_RATE_PARENT, 0x188, 2, 2, 0, unsafe { &COMPHY_MUX_TABLE }),
    HisiMuxClock(HI3798CV200_COMBPHY1_MUX, "combphy1_mux", COMPHY_MUX_P, 2, CLK_SET_RATE_PARENT, 0x188, 10, 2, 0, unsafe { &COMPHY_MUX_TABLE }),
    HisiMuxClock(HI3798CV200_SDIO0_MUX, "sdio0_mux", SDIO_MUX_P, 4, CLK_SET_RATE_PARENT, 0x9c, 8, 2, 0, unsafe { &SDIO_MUX_TABLE }),
];

static mut MMC_PHASE_REGVALS: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
static mut MMC_PHASE_DEGREES: [u32; 8] = [0, 45, 90, 135, 180, 225, 270, 315];
static mut HI3798CV200_PHASE_CLKS: [HisiPhaseClock; 2] = [
    HisiPhaseClock(HISTB_MMC_SAMPLE_CLK, "mmc_sample", "clk_mmc_ciu", CLK_SET_RATE_PARENT, 0xa0, 12, 3, unsafe { &MMC_PHASE_DEGREES }, unsafe { &MMC_PHASE_REGVALS }, 8),
    HisiPhaseClock(HISTB_MMC_DRV_CLK, "mmc_drive", "clk_mmc_ciu", CLK_SET_RATE_PARENT, 0xa0, 16, 3, unsafe { &MMC_PHASE_DEGREES }, unsafe { &MMC_PHASE_REGVALS }, 8),
];

// Gate-clock tables are represented literally; their external type and clock
// identifiers are provided by the kernel clock framework.
static HI3798CV200_GATE_CLKS: &[HisiGateClock] = &[
    HisiGateClock(HISTB_UART2_CLK, "clk_uart2", "75m", CLK_SET_RATE_PARENT, 0x68, 4, 0),
    HisiGateClock(HISTB_I2C0_CLK, "clk_i2c0", "clk_apb", CLK_SET_RATE_PARENT, 0x6c, 4, 0),
    HisiGateClock(HISTB_I2C1_CLK, "clk_i2c1", "clk_apb", CLK_SET_RATE_PARENT, 0x6c, 8, 0),
    HisiGateClock(HISTB_I2C2_CLK, "clk_i2c2", "clk_apb", CLK_SET_RATE_PARENT, 0x6c, 12, 0),
    HisiGateClock(HISTB_I2C3_CLK, "clk_i2c3", "clk_apb", CLK_SET_RATE_PARENT, 0x6c, 16, 0),
    HisiGateClock(HISTB_I2C4_CLK, "clk_i2c4", "clk_apb", CLK_SET_RATE_PARENT, 0x6c, 20, 0),
    HisiGateClock(HISTB_SPI0_CLK, "clk_spi0", "clk_apb", CLK_SET_RATE_PARENT, 0x70, 0, 0),
    HisiGateClock(HISTB_SDIO0_BIU_CLK, "clk_sdio0_biu", "200m", CLK_SET_RATE_PARENT, 0x9c, 0, 0),
    HisiGateClock(HISTB_SDIO0_CIU_CLK, "clk_sdio0_ciu", "sdio0_mux", CLK_SET_RATE_PARENT, 0x9c, 1, 0),
    HisiGateClock(HISTB_MMC_BIU_CLK, "clk_mmc_biu", "200m", CLK_SET_RATE_PARENT, 0xa0, 0, 0),
    HisiGateClock(HISTB_MMC_CIU_CLK, "clk_mmc_ciu", "mmc_mux", CLK_SET_RATE_PARENT, 0xa0, 1, 0),
    HisiGateClock(HISTB_PCIE_BUS_CLK, "clk_pcie_bus", "200m", CLK_SET_RATE_PARENT, 0x18c, 0, 0),
    HisiGateClock(HISTB_PCIE_SYS_CLK, "clk_pcie_sys", "100m", CLK_SET_RATE_PARENT, 0x18c, 1, 0),
    HisiGateClock(HISTB_PCIE_PIPE_CLK, "clk_pcie_pipe", "250m", CLK_SET_RATE_PARENT, 0x18c, 2, 0),
    HisiGateClock(HISTB_PCIE_AUX_CLK, "clk_pcie_aux", "24m", CLK_SET_RATE_PARENT, 0x18c, 3, 0),
    HisiGateClock(HI3798CV200_ETH_PUB_CLK, "clk_pub", None, CLK_SET_RATE_PARENT, 0xcc, 5, 0),
    HisiGateClock(HI3798CV200_ETH_BUS_CLK, "clk_bus", "clk_pub", CLK_SET_RATE_PARENT, 0xcc, 0, 0),
    HisiGateClock(HI3798CV200_ETH_BUS0_CLK, "clk_bus_m0", "clk_bus", CLK_SET_RATE_PARENT, 0xcc, 1, 0),
    HisiGateClock(HI3798CV200_ETH_BUS1_CLK, "clk_bus_m1", "clk_bus", CLK_SET_RATE_PARENT, 0xcc, 2, 0),
    HisiGateClock(HISTB_ETH0_MAC_CLK, "clk_mac0", "clk_bus_m0", CLK_SET_RATE_PARENT, 0xcc, 3, 0),
    HisiGateClock(HISTB_ETH0_MACIF_CLK, "clk_macif0", "clk_bus_m0", CLK_SET_RATE_PARENT, 0xcc, 24, 0),
    HisiGateClock(HISTB_ETH1_MAC_CLK, "clk_mac1", "clk_bus_m1", CLK_SET_RATE_PARENT, 0xcc, 4, 0),
    HisiGateClock(HISTB_ETH1_MACIF_CLK, "clk_macif1", "clk_bus_m1", CLK_SET_RATE_PARENT, 0xcc, 25, 0),
    HisiGateClock(HISTB_COMBPHY0_CLK, "clk_combphy0", "combphy0_mux", CLK_SET_RATE_PARENT, 0x188, 0, 0),
    HisiGateClock(HISTB_COMBPHY1_CLK, "clk_combphy1", "combphy1_mux", CLK_SET_RATE_PARENT, 0x188, 8, 0),
    HisiGateClock(HISTB_USB2_BUS_CLK, "clk_u2_bus", "clk_ahb", CLK_SET_RATE_PARENT, 0xb8, 0, 0),
    HisiGateClock(HISTB_USB2_PHY_CLK, "clk_u2_phy", "60m", CLK_SET_RATE_PARENT, 0xb8, 4, 0),
    HisiGateClock(HISTB_USB2_12M_CLK, "clk_u2_12m", "12m", CLK_SET_RATE_PARENT, 0xb8, 2, 0),
    HisiGateClock(HISTB_USB2_48M_CLK, "clk_u2_48m", "48m", CLK_SET_RATE_PARENT, 0xb8, 1, 0),
    HisiGateClock(HISTB_USB2_UTMI_CLK, "clk_u2_utmi", "60m", CLK_SET_RATE_PARENT, 0xb8, 5, 0),
    HisiGateClock(HISTB_USB2_OTG_UTMI_CLK, "clk_u2_otg_utmi", "60m", CLK_SET_RATE_PARENT, 0xb8, 3, 0),
    HisiGateClock(HISTB_USB2_PHY1_REF_CLK, "clk_u2_phy1_ref", "24m", CLK_SET_RATE_PARENT, 0xbc, 0, 0),
    HisiGateClock(HISTB_USB2_PHY2_REF_CLK, "clk_u2_phy2_ref", "24m", CLK_SET_RATE_PARENT, 0xbc, 2, 0),
    HisiGateClock(HISTB_USB3_BUS_CLK, "clk_u3_bus", None, CLK_SET_RATE_PARENT, 0xb0, 0, 0),
    HisiGateClock(HISTB_USB3_UTMI_CLK, "clk_u3_utmi", None, CLK_SET_RATE_PARENT, 0xb0, 4, 0),
    HisiGateClock(HISTB_USB3_PIPE_CLK, "clk_u3_pipe", None, CLK_SET_RATE_PARENT, 0xb0, 3, 0),
    HisiGateClock(HISTB_USB3_SUSPEND_CLK, "clk_u3_suspend", None, CLK_SET_RATE_PARENT, 0xb0, 2, 0),
    HisiGateClock(HISTB_USB3_BUS_CLK1, "clk_u3_bus1", None, CLK_SET_RATE_PARENT, 0xb0, 16, 0),
    HisiGateClock(HISTB_USB3_UTMI_CLK1, "clk_u3_utmi1", None, CLK_SET_RATE_PARENT, 0xb0, 20, 0),
    HisiGateClock(HISTB_USB3_PIPE_CLK1, "clk_u3_pipe1", None, CLK_SET_RATE_PARENT, 0xb0, 19, 0),
    HisiGateClock(HISTB_USB3_SUSPEND_CLK1, "clk_u3_suspend1", None, CLK_SET_RATE_PARENT, 0xb0, 18, 0),
];

unsafe fn hi3798cv200_clk_register(pdev: *mut PlatformDevice) -> *mut HisiClockData {
    let clk_data = hisi_clk_alloc(pdev, HI3798CV200_CRG_NR_CLKS);
    if clk_data.is_null() { return ERR_PTR(-ENOMEM); }
    let mut ret = hisi_clk_register_phase(pdev, unsafe { &HI3798CV200_PHASE_CLKS }, 2, clk_data);
    if ret != 0 { return ERR_PTR(ret); }
    ret = hisi_clk_register_fixed_rate(HI3798CV200_FIXED_RATE_CLKS, 15, clk_data);
    if ret != 0 { return ERR_PTR(ret); }
    ret = hisi_clk_register_mux(unsafe { &HI3798CV200_MUX_CLKS }, 4, clk_data);
    if ret != 0 { hisi_clk_unregister_fixed_rate(HI3798CV200_FIXED_RATE_CLKS, 15, clk_data); return ERR_PTR(ret); }
    ret = hisi_clk_register_gate(HI3798CV200_GATE_CLKS, 40, clk_data);
    if ret != 0 { hisi_clk_unregister_mux(unsafe { &HI3798CV200_MUX_CLKS }, 4, clk_data); hisi_clk_unregister_fixed_rate(HI3798CV200_FIXED_RATE_CLKS, 15, clk_data); return ERR_PTR(ret); }
    ret = of_clk_add_provider((*pdev).dev.of_node, of_clk_src_onecell_get, &(*clk_data).clk_data);
    if ret != 0 { hisi_clk_unregister_gate(HI3798CV200_GATE_CLKS, 40, clk_data); hisi_clk_unregister_mux(unsafe { &HI3798CV200_MUX_CLKS }, 4, clk_data); hisi_clk_unregister_fixed_rate(HI3798CV200_FIXED_RATE_CLKS, 15, clk_data); return ERR_PTR(ret); }
    clk_data
}

unsafe fn hi3798cv200_clk_unregister(pdev: *mut PlatformDevice) {
    let crg = platform_get_drvdata(pdev);
    of_clk_del_provider((*pdev).dev.of_node);
    hisi_clk_unregister_gate(HI3798CV200_GATE_CLKS, 40, (*crg).clk_data);
    hisi_clk_unregister_mux(unsafe { &HI3798CV200_MUX_CLKS }, 4, (*crg).clk_data);
    hisi_clk_unregister_fixed_rate(HI3798CV200_FIXED_RATE_CLKS, 15, (*crg).clk_data);
}

static HI3798CV200_CRG_FUNCS: HisiCrgFuncs = HisiCrgFuncs { register_clks: hi3798cv200_clk_register, unregister_clks: hi3798cv200_clk_unregister };

const HI3798CV200_SYSCTRL_NR_CLKS: u32 = 16;
static HI3798CV200_SYSCTRL_GATE_CLKS: &[HisiGateClock] = &[
    HisiGateClock(HISTB_IR_CLK, "clk_ir", "24m", CLK_SET_RATE_PARENT, 0x48, 4, 0),
    HisiGateClock(HISTB_TIMER01_CLK, "clk_timer01", "24m", CLK_SET_RATE_PARENT, 0x48, 6, 0),
    HisiGateClock(HISTB_UART0_CLK, "clk_uart0", "75m", CLK_SET_RATE_PARENT, 0x48, 10, 0),
];

unsafe fn hi3798cv200_sysctrl_clk_register(pdev: *mut PlatformDevice) -> *mut HisiClockData {
    let clk_data = hisi_clk_alloc(pdev, HI3798CV200_SYSCTRL_NR_CLKS);
    if clk_data.is_null() { return ERR_PTR(-ENOMEM); }
    let ret = hisi_clk_register_gate(HI3798CV200_SYSCTRL_GATE_CLKS, 3, clk_data);
    if ret != 0 { return ERR_PTR(ret); }
    let ret = of_clk_add_provider((*pdev).dev.of_node, of_clk_src_onecell_get, &(*clk_data).clk_data);
    if ret != 0 { hisi_clk_unregister_gate(HI3798CV200_SYSCTRL_GATE_CLKS, 3, clk_data); return ERR_PTR(ret); }
    clk_data
}

unsafe fn hi3798cv200_sysctrl_clk_unregister(pdev: *mut PlatformDevice) {
    let crg = platform_get_drvdata(pdev);
    of_clk_del_provider((*pdev).dev.of_node);
    hisi_clk_unregister_gate(HI3798CV200_SYSCTRL_GATE_CLKS, 3, (*crg).clk_data);
}

static HI3798CV200_SYSCTRL_FUNCS: HisiCrgFuncs = HisiCrgFuncs { register_clks: hi3798cv200_sysctrl_clk_register, unregister_clks: hi3798cv200_sysctrl_clk_unregister };

static HI3798CV200_CRG_MATCH_TABLE: &[OfDeviceId] = &[
    OfDeviceId { compatible: "hisilicon,hi3798cv200-crg", data: &HI3798CV200_CRG_FUNCS },
    OfDeviceId { compatible: "hisilicon,hi3798cv200-sysctrl", data: &HI3798CV200_SYSCTRL_FUNCS },
    OfDeviceId { compatible: "", data: core::ptr::null() },
];

unsafe fn hi3798cv200_crg_probe(pdev: *mut PlatformDevice) -> i32 {
    let crg = devm_kmalloc(&mut (*pdev).dev, core::mem::size_of::<HisiCrgDev>(), GFP_KERNEL) as *mut HisiCrgDev;
    if crg.is_null() { return -ENOMEM; }
    (*crg).funcs = of_device_get_match_data(&mut (*pdev).dev);
    if (*crg).funcs.is_null() { return -ENOENT; }
    (*crg).rstc = hisi_reset_init(pdev);
    if (*crg).rstc.is_null() { return -ENOMEM; }
    (*crg).clk_data = ((*(*crg).funcs).register_clks)(pdev);
    if IS_ERR((*crg).clk_data) { hisi_reset_exit((*crg).rstc); return PTR_ERR((*crg).clk_data); }
    platform_set_drvdata(pdev, crg as *mut _);
    0
}

unsafe fn hi3798cv200_crg_remove(pdev: *mut PlatformDevice) {
    let crg = platform_get_drvdata(pdev);
    hisi_reset_exit((*crg).rstc);
    ((*crg).funcs.as_ref().unwrap().unregister_clks)(pdev);
}

static mut HI3798CV200_CRG_DRIVER: PlatformDriver = PlatformDriver {
    probe: hi3798cv200_crg_probe,
    remove: hi3798cv200_crg_remove,
    driver: Driver { name: "hi3798cv200-crg", of_match_table: HI3798CV200_CRG_MATCH_TABLE },
};

unsafe fn hi3798cv200_crg_init() -> i32 { platform_driver_register(&mut HI3798CV200_CRG_DRIVER) }
unsafe fn hi3798cv200_crg_exit() { platform_driver_unregister(&mut HI3798CV200_CRG_DRIVER); }

// core_initcall(hi3798cv200_crg_init);
// module_exit(hi3798cv200_crg_exit);
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("HiSilicon Hi3798CV200 CRG Driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
