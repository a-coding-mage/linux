// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Hi3516CV300 Clock and Reset Generator Driver
 *
 * Copyright (c) 2016 HiSilicon Technologies Co., Ltd.
 */

// C dependencies: <dt-bindings/clock/hi3516cv300-clock.h>,
// <linux/clk-provider.h>, <linux/module.h>, <linux/of.h>,
// <linux/platform_device.h>, "clk.h", "crg.h", and "reset.h".

const HI3516CV300_INNER_CLK_OFFSET: u32 = 64;
const HI3516CV300_FIXED_3M: u32 = 65;
const HI3516CV300_FIXED_6M: u32 = 66;
const HI3516CV300_FIXED_24M: u32 = 67;
const HI3516CV300_FIXED_49P5: u32 = 68;
const HI3516CV300_FIXED_50M: u32 = 69;
const HI3516CV300_FIXED_83P3M: u32 = 70;
const HI3516CV300_FIXED_99M: u32 = 71;
const HI3516CV300_FIXED_100M: u32 = 72;
const HI3516CV300_FIXED_148P5M: u32 = 73;
const HI3516CV300_FIXED_198M: u32 = 74;
const HI3516CV300_FIXED_297M: u32 = 75;
const HI3516CV300_UART_MUX: u32 = 76;
const HI3516CV300_FMC_MUX: u32 = 77;
const HI3516CV300_MMC0_MUX: u32 = 78;
const HI3516CV300_MMC1_MUX: u32 = 79;
const HI3516CV300_MMC2_MUX: u32 = 80;
const HI3516CV300_MMC3_MUX: u32 = 81;
const HI3516CV300_PWM_MUX: u32 = 82;
const HI3516CV300_CRG_NR_CLKS: u32 = 128;
const HI3516CV300_SYSCTRL_NR_CLKS: u32 = 16;

static HI3516CV300_FIXED_RATE_CLKS: [HisiFixedRateClock; 12] = [
    HisiFixedRateClock { id: HI3516CV300_FIXED_3M, name: "3m", parent: core::ptr::null(), flags: 0, rate: 3000000 },
    HisiFixedRateClock { id: HI3516CV300_FIXED_6M, name: "6m", parent: core::ptr::null(), flags: 0, rate: 6000000 },
    HisiFixedRateClock { id: HI3516CV300_FIXED_24M, name: "24m", parent: core::ptr::null(), flags: 0, rate: 24000000 },
    HisiFixedRateClock { id: HI3516CV300_FIXED_49P5, name: "49.5m", parent: core::ptr::null(), flags: 0, rate: 49500000 },
    HisiFixedRateClock { id: HI3516CV300_FIXED_50M, name: "50m", parent: core::ptr::null(), flags: 0, rate: 50000000 },
    HisiFixedRateClock { id: HI3516CV300_FIXED_83P3M, name: "83.3m", parent: core::ptr::null(), flags: 0, rate: 83300000 },
    HisiFixedRateClock { id: HI3516CV300_FIXED_99M, name: "99m", parent: core::ptr::null(), flags: 0, rate: 99000000 },
    HisiFixedRateClock { id: HI3516CV300_FIXED_100M, name: "100m", parent: core::ptr::null(), flags: 0, rate: 100000000 },
    HisiFixedRateClock { id: HI3516CV300_FIXED_148P5M, name: "148.5m", parent: core::ptr::null(), flags: 0, rate: 148500000 },
    HisiFixedRateClock { id: HI3516CV300_FIXED_198M, name: "198m", parent: core::ptr::null(), flags: 0, rate: 198000000 },
    HisiFixedRateClock { id: HI3516CV300_FIXED_297M, name: "297m", parent: core::ptr::null(), flags: 0, rate: 297000000 },
    HisiFixedRateClock { id: HI3516CV300_APB_CLK, name: "apb", parent: core::ptr::null(), flags: 0, rate: 50000000 },
];

static UART_MUX_P: [&str; 2] = ["24m", "6m"];
static FMC_MUX_P: [&str; 5] = ["24m", "83.3m", "148.5m", "198m", "297m"];
static MMC_MUX_P: [&str; 1] = ["49.5m"];
static MMC2_MUX_P: [&str; 2] = ["99m", "49.5m"];
static PWM_MUX_P: [&str; 4] = ["3m", "50m", "24m", "24m"];
static UART_MUX_TABLE: [u32; 2] = [0, 1];
static FMC_MUX_TABLE: [u32; 5] = [0, 1, 2, 3, 4];
static MMC_MUX_TABLE: [u32; 1] = [0];
static MMC2_MUX_TABLE: [u32; 2] = [0, 2];
static PWM_MUX_TABLE: [u32; 4] = [0, 1, 2, 3];

static HI3516CV300_MUX_CLKS: [HisiMuxClock; 7] = [
    HisiMuxClock { id: HI3516CV300_UART_MUX, name: "uart_mux", parents: &UART_MUX_P, flags: CLK_SET_RATE_PARENT, offset: 0xe4, shift: 19, width: 1, table: &UART_MUX_TABLE },
    HisiMuxClock { id: HI3516CV300_FMC_MUX, name: "fmc_mux", parents: &FMC_MUX_P, flags: CLK_SET_RATE_PARENT, offset: 0xc0, shift: 2, width: 3, table: &FMC_MUX_TABLE },
    HisiMuxClock { id: HI3516CV300_MMC0_MUX, name: "mmc0_mux", parents: &MMC_MUX_P, flags: CLK_SET_RATE_PARENT, offset: 0xc4, shift: 4, width: 2, table: &MMC_MUX_TABLE },
    HisiMuxClock { id: HI3516CV300_MMC1_MUX, name: "mmc1_mux", parents: &MMC_MUX_P, flags: CLK_SET_RATE_PARENT, offset: 0xc4, shift: 12, width: 2, table: &MMC_MUX_TABLE },
    HisiMuxClock { id: HI3516CV300_MMC2_MUX, name: "mmc2_mux", parents: &MMC2_MUX_P, flags: CLK_SET_RATE_PARENT, offset: 0xc4, shift: 20, width: 2, table: &MMC2_MUX_TABLE },
    HisiMuxClock { id: HI3516CV300_MMC3_MUX, name: "mmc3_mux", parents: &MMC_MUX_P, flags: CLK_SET_RATE_PARENT, offset: 0xc8, shift: 4, width: 2, table: &MMC_MUX_TABLE },
    HisiMuxClock { id: HI3516CV300_PWM_MUX, name: "pwm_mux", parents: &PWM_MUX_P, flags: CLK_SET_RATE_PARENT, offset: 0x38, shift: 2, width: 2, table: &PWM_MUX_TABLE },
];

// Gate table is kept in the source-level form; clock identifiers and external
// clock framework structures are supplied by the translated dependency files.
static HI3516CV300_GATE_CLKS: [HisiGateClock; 20] = [
    HisiGateClock { id: HI3516CV300_UART0_CLK, name: "clk_uart0", parent: "uart_mux", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit: 15 },
    HisiGateClock { id: HI3516CV300_UART1_CLK, name: "clk_uart1", parent: "uart_mux", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit: 16 },
    HisiGateClock { id: HI3516CV300_UART2_CLK, name: "clk_uart2", parent: "uart_mux", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit: 17 },
    HisiGateClock { id: HI3516CV300_SPI0_CLK, name: "clk_spi0", parent: "100m", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit: 13 },
    HisiGateClock { id: HI3516CV300_SPI1_CLK, name: "clk_spi1", parent: "100m", flags: CLK_SET_RATE_PARENT, offset: 0xe4, bit: 14 },
    HisiGateClock { id: HI3516CV300_FMC_CLK, name: "clk_fmc", parent: "fmc_mux", flags: CLK_SET_RATE_PARENT, offset: 0xc0, bit: 1 },
    HisiGateClock { id: HI3516CV300_MMC0_CLK, name: "clk_mmc0", parent: "mmc0_mux", flags: CLK_SET_RATE_PARENT, offset: 0xc4, bit: 1 },
    HisiGateClock { id: HI3516CV300_MMC1_CLK, name: "clk_mmc1", parent: "mmc1_mux", flags: CLK_SET_RATE_PARENT, offset: 0xc4, bit: 9 },
    HisiGateClock { id: HI3516CV300_MMC2_CLK, name: "clk_mmc2", parent: "mmc2_mux", flags: CLK_SET_RATE_PARENT, offset: 0xc4, bit: 17 },
    HisiGateClock { id: HI3516CV300_MMC3_CLK, name: "clk_mmc3", parent: "mmc3_mux", flags: CLK_SET_RATE_PARENT, offset: 0xc8, bit: 1 },
    HisiGateClock { id: HI3516CV300_ETH_CLK, name: "clk_eth", parent: core::ptr::null(), flags: 0, offset: 0xec, bit: 1 },
    HisiGateClock { id: HI3516CV300_DMAC_CLK, name: "clk_dmac", parent: core::ptr::null(), flags: 0, offset: 0xd8, bit: 5 },
    HisiGateClock { id: HI3516CV300_PWM_CLK, name: "clk_pwm", parent: "pwm_mux", flags: CLK_SET_RATE_PARENT, offset: 0x38, bit: 1 },
    HisiGateClock { id: HI3516CV300_USB2_BUS_CLK, name: "clk_usb2_bus", parent: core::ptr::null(), flags: 0, offset: 0xb8, bit: 0 },
    HisiGateClock { id: HI3516CV300_USB2_OHCI48M_CLK, name: "clk_usb2_ohci48m", parent: core::ptr::null(), flags: 0, offset: 0xb8, bit: 1 },
    HisiGateClock { id: HI3516CV300_USB2_OHCI12M_CLK, name: "clk_usb2_ohci12m", parent: core::ptr::null(), flags: 0, offset: 0xb8, bit: 2 },
    HisiGateClock { id: HI3516CV300_USB2_OTG_UTMI_CLK, name: "clk_usb2_otg_utmi", parent: core::ptr::null(), flags: 0, offset: 0xb8, bit: 3 },
    HisiGateClock { id: HI3516CV300_USB2_HST_PHY_CLK, name: "clk_usb2_hst_phy", parent: core::ptr::null(), flags: 0, offset: 0xb8, bit: 4 },
    HisiGateClock { id: HI3516CV300_USB2_UTMI0_CLK, name: "clk_usb2_utmi0", parent: core::ptr::null(), flags: 0, offset: 0xb8, bit: 5 },
    HisiGateClock { id: HI3516CV300_USB2_PHY_CLK, name: "clk_usb2_phy", parent: core::ptr::null(), flags: 0, offset: 0xb8, bit: 7 },
];

extern "C" {
    fn hisi_clk_alloc(pdev: *mut PlatformDevice, nr_clks: u32) -> *mut HisiClockData;
    fn hisi_clk_register_fixed_rate(clks: *const HisiFixedRateClock, count: usize, data: *mut HisiClockData) -> i32;
    fn hisi_clk_register_mux(clks: *const HisiMuxClock, count: usize, data: *mut HisiClockData) -> i32;
    fn hisi_clk_register_gate(clks: *const HisiGateClock, count: usize, data: *mut HisiClockData) -> i32;
    fn hisi_clk_unregister_fixed_rate(clks: *const HisiFixedRateClock, count: usize, data: *mut HisiClockData);
    fn hisi_clk_unregister_mux(clks: *const HisiMuxClock, count: usize, data: *mut HisiClockData);
    fn hisi_clk_unregister_gate(clks: *const HisiGateClock, count: usize, data: *mut HisiClockData);
    fn hisi_reset_init(pdev: *mut PlatformDevice) -> *mut HisiResetController;
    fn hisi_reset_exit(rstc: *mut HisiResetController);
    fn of_clk_add_provider(node: *mut OfNode, get: unsafe extern "C" fn(), data: *mut core::ffi::c_void) -> i32;
    fn of_clk_del_provider(node: *mut OfNode);
}

unsafe fn hi3516cv300_clk_register(pdev: *mut PlatformDevice) -> *mut HisiClockData {
    let data = hisi_clk_alloc(pdev, HI3516CV300_CRG_NR_CLKS);
    if data.is_null() { return core::ptr::null_mut(); }
    let mut ret = hisi_clk_register_fixed_rate(HI3516CV300_FIXED_RATE_CLKS.as_ptr(), HI3516CV300_FIXED_RATE_CLKS.len(), data);
    if ret != 0 { return core::ptr::null_mut(); }
    ret = hisi_clk_register_mux(HI3516CV300_MUX_CLKS.as_ptr(), HI3516CV300_MUX_CLKS.len(), data);
    if ret != 0 { hisi_clk_unregister_fixed_rate(HI3516CV300_FIXED_RATE_CLKS.as_ptr(), HI3516CV300_FIXED_RATE_CLKS.len(), data); return core::ptr::null_mut(); }
    ret = hisi_clk_register_gate(HI3516CV300_GATE_CLKS.as_ptr(), HI3516CV300_GATE_CLKS.len(), data);
    if ret != 0 { hisi_clk_unregister_mux(HI3516CV300_MUX_CLKS.as_ptr(), HI3516CV300_MUX_CLKS.len(), data); hisi_clk_unregister_fixed_rate(HI3516CV300_FIXED_RATE_CLKS.as_ptr(), HI3516CV300_FIXED_RATE_CLKS.len(), data); return core::ptr::null_mut(); }
    data
}

unsafe fn hi3516cv300_clk_unregister(pdev: *mut PlatformDevice) {
    let crg = platform_get_drvdata(pdev);
    of_clk_del_provider((*pdev).dev.of_node);
    hisi_clk_unregister_gate(HI3516CV300_GATE_CLKS.as_ptr(), HI3516CV300_GATE_CLKS.len(), (*crg).clk_data);
    hisi_clk_unregister_mux(HI3516CV300_MUX_CLKS.as_ptr(), HI3516CV300_MUX_CLKS.len(), (*crg).clk_data);
    hisi_clk_unregister_fixed_rate(HI3516CV300_FIXED_RATE_CLKS.as_ptr(), HI3516CV300_FIXED_RATE_CLKS.len(), (*crg).clk_data);
}

static WDT_MUX_P: [&str; 2] = ["3m", "apb"];
static WDT_MUX_TABLE: [u32; 2] = [0, 1];
static HI3516CV300_SYSCTRL_MUX_CLKS: [HisiMuxClock; 1] = [HisiMuxClock { id: HI3516CV300_WDT_CLK, name: "wdt", parents: &WDT_MUX_P, flags: CLK_SET_RATE_PARENT, offset: 0, shift: 23, width: 1, table: &WDT_MUX_TABLE }];

// Device matching, probe/remove, platform-driver registration, init/exit, and
// module metadata are emitted by the kernel binding layer using these symbols.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
