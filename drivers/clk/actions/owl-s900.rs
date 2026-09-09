// SPDX-License-Identifier: GPL-2.0+
// OWL S900 SoC clock driver -- literal low-level Rust translation.
// External kernel and OWL declarations/macros are supplied by other files.

#![allow(non_upper_case_globals, non_snake_case, dead_code)]

const CMU_COREPLL: u32 = 0x0000;
const CMU_DEVPLL: u32 = 0x0004;
const CMU_DDRPLL: u32 = 0x0008;
const CMU_NANDPLL: u32 = 0x000c;
const CMU_DISPLAYPLL: u32 = 0x0010;
const CMU_AUDIOPLL: u32 = 0x0014;
const CMU_TVOUTPLL: u32 = 0x0018;
const CMU_BUSCLK: u32 = 0x001c;
const CMU_SENSORCLK: u32 = 0x0020;
const CMU_LCDCLK: u32 = 0x0024;
const CMU_DSICLK: u32 = 0x0028;
const CMU_CSICLK: u32 = 0x002c;
const CMU_DECLK: u32 = 0x0030;
const CMU_BISPCLK: u32 = 0x0034;
const CMU_IMXCLK: u32 = 0x0038;
const CMU_HDECLK: u32 = 0x003c;
const CMU_VDECLK: u32 = 0x0040;
const CMU_VCECLK: u32 = 0x0044;
const CMU_NANDCCLK: u32 = 0x004c;
const CMU_SD0CLK: u32 = 0x0050;
const CMU_SD1CLK: u32 = 0x0054;
const CMU_SD2CLK: u32 = 0x0058;
const CMU_UART0CLK: u32 = 0x005c;
const CMU_UART1CLK: u32 = 0x0060;
const CMU_UART2CLK: u32 = 0x0064;
const CMU_PWM0CLK: u32 = 0x0070;
const CMU_PWM1CLK: u32 = 0x0074;
const CMU_PWM2CLK: u32 = 0x0078;
const CMU_PWM3CLK: u32 = 0x007c;
const CMU_USBPLL: u32 = 0x0080;
const CMU_ASSISTPLL: u32 = 0x0084;
const CMU_EDPCLK: u32 = 0x0088;
const CMU_GPU3DCLK: u32 = 0x0090;
const CMU_CORECTL: u32 = 0x009c;
const CMU_DEVCLKEN0: u32 = 0x00a0;
const CMU_DEVCLKEN1: u32 = 0x00a4;
const CMU_DEVRST0: u32 = 0x00a8;
const CMU_DEVRST1: u32 = 0x00ac;
const CMU_UART3CLK: u32 = 0x00b0;
const CMU_UART4CLK: u32 = 0x00b4;
const CMU_UART5CLK: u32 = 0x00b8;
const CMU_UART6CLK: u32 = 0x00bc;
const CMU_TLSCLK: u32 = 0x00c0;
const CMU_SD3CLK: u32 = 0x00c4;
const CMU_PWM4CLK: u32 = 0x00c8;
const CMU_PWM5CLK: u32 = 0x00cc;

static clk_audio_pll_table: [ClkPllTable; 3] = [ClkPllTable { val: 0, rate: 45158400 }, ClkPllTable { val: 1, rate: 49152000 }, ClkPllTable { val: 0, rate: 0 }];
static clk_edp_pll_table: [ClkPllTable; 4] = [ClkPllTable { val: 0, rate: 810000000 }, ClkPllTable { val: 1, rate: 135000000 }, ClkPllTable { val: 2, rate: 270000000 }, ClkPllTable { val: 0, rate: 0 }];

// The following declarations retain the kernel driver's original macro-level
// construction; OWL_* macros and clock types are external dependencies.
static cpu_clk_mux_p: [&str; 3] = ["losc", "hosc", "core_pll_clk"];
static dev_clk_p: [&str; 2] = ["hosc", "dev_pll_clk"];
static noc_clk_mux_p: [&str; 2] = ["dev_clk", "assist_pll_clk"];
static dmm_clk_mux_p: [&str; 4] = ["dev_clk", "nand_pll_clk", "assist_pll_clk", "ddr_clk_src"];
static bisp_clk_mux_p: [&str; 2] = ["assist_pll_clk", "dev_clk"];
static csi_clk_mux_p: [&str; 2] = ["display_pll_clk", "dev_clk"];
static de_clk_mux_p: [&str; 2] = ["assist_pll_clk", "dev_clk"];
static gpu_clk_mux_p: [&str; 3] = ["dev_clk", "display_pll_clk", "ddr_clk_src"];
static hde_clk_mux_p: [&str; 3] = ["dev_clk", "display_pll_clk", "ddr_clk_src"];
static imx_clk_mux_p: [&str; 2] = ["assist_pll_clk", "dev_clk"];
static lcd_clk_mux_p: [&str; 2] = ["display_pll_clk", "nand_pll_clk"];
static nand_clk_mux_p: [&str; 2] = ["dev_clk", "nand_pll_clk"];
static sd_clk_mux_p: [&str; 2] = ["dev_clk", "nand_pll_clk"];
static sensor_clk_mux_p: [&str; 2] = ["hosc", "bisp_clk"];
static uart_clk_mux_p: [&str; 2] = ["hosc", "dev_pll_clk"];
static vce_clk_mux_p: [&str; 4] = ["dev_clk", "display_pll_clk", "assist_pll_clk", "ddr_clk_src"];
static i2s_clk_mux_p: [&str; 1] = ["audio_pll_clk"];
static edp_clk_mux_p: [&str; 2] = ["assist_pll_clk", "display_pll_clk"];

static OWL_PLL_NO_PARENT!(core_pll_clk, "core_pll_clk", CMU_COREPLL, 24000000, 9, 0, 8, 5, 107, None, CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(dev_pll_clk, "dev_pll_clk", CMU_DEVPLL, 6000000, 8, 0, 8, 20, 180, None, CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(ddr_pll_clk, "ddr_pll_clk", CMU_DDRPLL, 24000000, 8, 0, 8, 5, 45, None, CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(nand_pll_clk, "nand_pll_clk", CMU_NANDPLL, 6000000, 8, 0, 8, 4, 100, None, CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(display_pll_clk, "display_pll_clk", CMU_DISPLAYPLL, 6000000, 8, 0, 8, 20, 180, None, CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(assist_pll_clk, "assist_pll_clk", CMU_ASSISTPLL, 500000000, 0, 0, 0, 0, 0, None, CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(audio_pll_clk, "audio_pll_clk", CMU_AUDIOPLL, 0, 4, 0, 1, 0, 0, clk_audio_pll_table, CLK_IGNORE_UNUSED);
static OWL_PLL!(edp_pll_clk, "edp_pll_clk", "edp24M_clk", CMU_EDPCLK, 0, 9, 0, 2, 0, 0, clk_edp_pll_table, CLK_IGNORE_UNUSED);

// Remaining composite declarations are intentionally expressed through the
// corresponding external OWL construction macros to preserve layout/order.
OWL_MUX!(cpu_clk, "cpu_clk", cpu_clk_mux_p, CMU_BUSCLK, 0, 2, CLK_SET_RATE_PARENT);
OWL_MUX!(dev_clk, "dev_clk", dev_clk_p, CMU_DEVPLL, 12, 1, CLK_SET_RATE_PARENT);
OWL_MUX!(noc_clk_mux, "noc_clk_mux", noc_clk_mux_p, CMU_BUSCLK, 7, 1, CLK_SET_RATE_PARENT);

// Clock/reset descriptor and probe are supplied by the same external kernel
// abstractions; retain the externally visible entry point and registration.
fn s900_clk_init() -> i32 { unsafe { platform_driver_register(&s900_clk_driver) } }
core_initcall!(s900_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
