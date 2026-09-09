// SPDX-License-Identifier: GPL-2.0+
/* Actions Semi S700 clock driver. Rust translation of the C implementation. */

// External Linux/kernel types, functions, constants, and clock/reset macros are
// supplied by the surrounding translation unit.

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
const CMU_DSIPLLCLK: u32 = 0x0028;
const CMU_CSICLK: u32 = 0x002c;
const CMU_DECLK: u32 = 0x0030;
const CMU_SICLK: u32 = 0x0034;
const CMU_BUSCLK1: u32 = 0x0038;
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
const CMU_UART3CLK: u32 = 0x0068;
const CMU_UART4CLK: u32 = 0x006c;
const CMU_UART5CLK: u32 = 0x0070;
const CMU_UART6CLK: u32 = 0x0074;
const CMU_PWM0CLK: u32 = 0x0078;
const CMU_PWM1CLK: u32 = 0x007c;
const CMU_PWM2CLK: u32 = 0x0080;
const CMU_PWM3CLK: u32 = 0x0084;
const CMU_PWM4CLK: u32 = 0x0088;
const CMU_PWM5CLK: u32 = 0x008c;
const CMU_GPU3DCLK: u32 = 0x0090;
const CMU_CORECTL: u32 = 0x009c;
const CMU_DEVCLKEN0: u32 = 0x00a0;
const CMU_DEVCLKEN1: u32 = 0x00a4;
const CMU_DEVRST0: u32 = 0x00a8;
const CMU_DEVRST1: u32 = 0x00ac;
const CMU_USBPLL: u32 = 0x00b0;
const CMU_ETHERNETPLL: u32 = 0x00b4;
const CMU_CVBSPLL: u32 = 0x00b8;
const CMU_SSTSCLK: u32 = 0x00c0;

static mut clk_audio_pll_table: [clk_pll_table; 3] = [
    clk_pll_table { val: 0, rate: 45158400 },
    clk_pll_table { val: 1, rate: 49152000 },
    clk_pll_table { val: 0, rate: 0 },
];
static mut clk_cvbs_pll_table: [clk_pll_table; 18] = [
    clk_pll_table { val: 27, rate: 29 * 12000000 }, clk_pll_table { val: 28, rate: 30 * 12000000 },
    clk_pll_table { val: 29, rate: 31 * 12000000 }, clk_pll_table { val: 30, rate: 32 * 12000000 },
    clk_pll_table { val: 31, rate: 33 * 12000000 }, clk_pll_table { val: 32, rate: 34 * 12000000 },
    clk_pll_table { val: 33, rate: 35 * 12000000 }, clk_pll_table { val: 34, rate: 36 * 12000000 },
    clk_pll_table { val: 35, rate: 37 * 12000000 }, clk_pll_table { val: 36, rate: 38 * 12000000 },
    clk_pll_table { val: 37, rate: 39 * 12000000 }, clk_pll_table { val: 38, rate: 40 * 12000000 },
    clk_pll_table { val: 39, rate: 41 * 12000000 }, clk_pll_table { val: 40, rate: 42 * 12000000 },
    clk_pll_table { val: 41, rate: 43 * 12000000 }, clk_pll_table { val: 42, rate: 44 * 12000000 },
    clk_pll_table { val: 43, rate: 45 * 12000000 }, clk_pll_table { val: 0, rate: 0 },
];

static OWL_PLL_NO_PARENT!(clk_core_pll, "core_pll", CMU_COREPLL, 12000000, 9, 0, 8, 4, 174, None, CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(clk_dev_pll, "dev_pll", CMU_DEVPLL, 6000000, 8, 0, 8, 8, 126, None, CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(clk_ddr_pll, "ddr_pll", CMU_DDRPLL, 6000000, 8, 0, 8, 2, 180, None, CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(clk_nand_pll, "nand_pll", CMU_NANDPLL, 6000000, 8, 0, 8, 2, 86, None, CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(clk_display_pll, "display_pll", CMU_DISPLAYPLL, 6000000, 8, 0, 8, 2, 140, None, CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(clk_cvbs_pll, "cvbs_pll", CMU_CVBSPLL, 0, 8, 0, 8, 27, 43, Some(clk_cvbs_pll_table), CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(clk_audio_pll, "audio_pll", CMU_AUDIOPLL, 0, 4, 0, 1, 0, 0, Some(clk_audio_pll_table), CLK_IGNORE_UNUSED);
static OWL_PLL_NO_PARENT!(clk_ethernet_pll, "ethernet_pll", CMU_ETHERNETPLL, 500000000, 0, 0, 0, 0, 0, None, CLK_IGNORE_UNUSED);

static cpu_clk_mux_p: [&str; 4] = ["losc", "hosc", "core_pll", "noc1_clk_div"];
static dev_clk_p: [&str; 2] = ["hosc", "dev_pll"];
static noc_clk_mux_p: [&str; 5] = ["dev_clk", "display_pll", "nand_pll", "ddr_pll", "cvbs_pll"];
static csi_clk_mux_p: [&str; 2] = ["display_pll", "dev_clk"];
static de_clk_mux_p: [&str; 2] = ["display_pll", "dev_clk"];
static hde_clk_mux_p: [&str; 4] = ["dev_clk", "display_pll", "nand_pll", "ddr_pll"];
static nand_clk_mux_p: [&str; 4] = ["nand_pll", "display_pll", "dev_clk", "ddr_pll"];
static sd_clk_mux_p: [&str; 2] = ["dev_clk", "nand_pll"];
static uart_clk_mux_p: [&str; 2] = ["hosc", "dev_pll"];
static pwm_clk_mux_p: [&str; 2] = ["losc", "hosc"];
static gpu_clk_mux_p: [&str; 5] = ["dev_clk", "display_pll", "nand_pll", "ddr_clk", "cvbs_pll"];
static lcd_clk_mux_p: [&str; 2] = ["display_pll", "dev_clk"];
static i2s_clk_mux_p: [&str; 1] = ["audio_pll"];
static sensor_clk_mux_p: [&str; 2] = ["hosc", "si"];

static OWL_MUX!(clk_cpu, "cpu_clk", cpu_clk_mux_p, CMU_BUSCLK, 0, 2, CLK_SET_RATE_PARENT);
static OWL_MUX!(clk_dev, "dev_clk", dev_clk_p, CMU_DEVPLL, 12, 1, CLK_SET_RATE_PARENT);
static OWL_MUX!(clk_noc0_clk_mux, "noc0_clk_mux", noc_clk_mux_p, CMU_BUSCLK, 4, 3, CLK_SET_RATE_PARENT);
static OWL_MUX!(clk_noc1_clk_mux, "noc1_clk_mux", noc_clk_mux_p, CMU_BUSCLK1, 4, 3, CLK_SET_RATE_PARENT);
static OWL_MUX!(clk_hp_clk_mux, "hp_clk_mux", noc_clk_mux_p, CMU_BUSCLK1, 8, 3, CLK_SET_RATE_PARENT);

static sd_factor_table: [clk_factor_table; 54] = [
    /* bit0 ~ 4 */
    {0,1,1},{1,1,2},{2,1,3},{3,1,4},{4,1,5},{5,1,6},{6,1,7},{7,1,8},
    {8,1,9},{9,1,10},{10,1,11},{11,1,12},{12,1,13},{13,1,14},{14,1,15},{15,1,16},
    {16,1,17},{17,1,18},{18,1,19},{19,1,20},{20,1,21},{21,1,22},{22,1,23},{23,1,24},
    {24,1,25},{25,1,26},
    /* bit8: /128 */
    {256,1,128},{257,1,256},{258,1,384},{259,1,512},{260,1,640},{261,1,768},{262,1,896},{263,1,1024},
    {264,1,1152},{265,1,1280},{266,1,1408},{267,1,1536},{268,1,1664},{269,1,1792},{270,1,1920},{271,1,2048},
    {272,1,2176},{273,1,2304},{274,1,2432},{275,1,2560},{276,1,2688},{277,1,2816},{278,1,2944},{279,1,3072},
    {280,1,3200},{281,1,3328}, {0,0,0},
];
static lcd_factor_table: [clk_factor_table; 25] = [
    /* bit0 ~ 3 */
    {0,1,1},{1,1,2},{2,1,3},{3,1,4},{4,1,5},{5,1,6},{6,1,7},{7,1,8},{8,1,9},{9,1,10},{10,1,11},{11,1,12},
    /* bit8: /7 */
    {256,1,7},{257,1,14},{258,1,21},{259,1,28},{260,1,35},{261,1,42},{262,1,49},{263,1,56},{264,1,63},{265,1,70},{266,1,77},{267,1,84},{0,0,0},
];
static hdmia_div_table: [clk_div_table; 10] = [{0,1},{1,2},{2,3},{3,4},{4,6},{5,8},{6,12},{7,16},{8,24},{0,0}];
static rmii_div_table: [clk_div_table; 3] = [{0,4},{1,10},{0,0}];
static de_factor_table: [clk_factor_table; 10] = [{0,1,1},{1,2,3},{2,1,2},{3,2,5},{4,1,3},{5,1,4},{6,1,6},{7,1,8},{8,1,12},{0,0,0}];
static hde_factor_table: [clk_factor_table; 9] = [{0,1,1},{1,2,3},{2,1,2},{3,2,5},{4,1,3},{5,1,4},{6,1,6},{7,1,8},{0,0,0}];

static OWL_DIVIDER!(clk_noc0, "noc0_clk", "noc0_clk_mux", CMU_BUSCLK, 16, 2, None, 0, 0);
static OWL_DIVIDER!(clk_noc1, "noc1_clk", "noc1_clk_mux", CMU_BUSCLK1, 16, 2, None, 0, 0);
static OWL_DIVIDER!(clk_noc1_clk_div, "noc1_clk_div", "noc1_clk", CMU_BUSCLK1, 20, 1, None, 0, 0);
static OWL_DIVIDER!(clk_hp_clk_div, "hp_clk_div", "hp_clk_mux", CMU_BUSCLK1, 12, 2, None, 0, 0);
static OWL_DIVIDER!(clk_ahb, "ahb_clk", "hp_clk_div", CMU_BUSCLK1, 2, 2, None, 0, 0);
static OWL_DIVIDER!(clk_apb, "apb_clk", "ahb_clk", CMU_BUSCLK1, 14, 2, None, 0, 0);
static OWL_DIVIDER!(clk_sensor0, "sensor0", "sensor_src", CMU_SENSORCLK, 0, 4, None, 0, 0);
static OWL_DIVIDER!(clk_sensor1, "sensor1", "sensor_src", CMU_SENSORCLK, 8, 4, None, 0, 0);
static OWL_DIVIDER!(clk_rmii_ref, "rmii_ref", "ethernet_pll", CMU_ETHERNETPLL, 2, 1, Some(rmii_div_table), 0, 0);

static OWL_GATE!(clk_gpio,"gpio","apb_clk",CMU_DEVCLKEN1,25,0,0); static OWL_GATE!(clk_dmac,"dmac","hp_clk_div",CMU_DEVCLKEN0,17,0,0); static OWL_GATE!(clk_timer,"timer","hosc",CMU_DEVCLKEN1,22,0,0);
static OWL_GATE_NO_PARENT!(clk_dsi,"dsi_clk",CMU_DEVCLKEN0,2,0,0); static OWL_GATE_NO_PARENT!(clk_tvout,"tvout_clk",CMU_DEVCLKEN0,3,0,0); static OWL_GATE_NO_PARENT!(clk_hdmi_dev,"hdmi_dev",CMU_DEVCLKEN0,5,0,0);
static OWL_GATE_NO_PARENT!(clk_usb3_480mpll0,"usb3_480mpll0",CMU_USBPLL,3,0,0); static OWL_GATE_NO_PARENT!(clk_usb3_480mphy0,"usb3_480mphy0",CMU_USBPLL,2,0,0); static OWL_GATE_NO_PARENT!(clk_usb3_5gphy,"usb3_5gphy",CMU_USBPLL,1,0,0); static OWL_GATE_NO_PARENT!(clk_usb3_cce,"usb3_cce",CMU_DEVCLKEN0,25,0,0);
static OWL_GATE!(clk_i2c0,"i2c0","hosc",CMU_DEVCLKEN1,0,0,0); static OWL_GATE!(clk_i2c1,"i2c1","hosc",CMU_DEVCLKEN1,1,0,0); static OWL_GATE!(clk_i2c2,"i2c2","hosc",CMU_DEVCLKEN1,2,0,0); static OWL_GATE!(clk_i2c3,"i2c3","hosc",CMU_DEVCLKEN1,3,0,0);
static OWL_GATE!(clk_spi0,"spi0","ahb_clk",CMU_DEVCLKEN1,4,0,0); static OWL_GATE!(clk_spi1,"spi1","ahb_clk",CMU_DEVCLKEN1,5,0,0); static OWL_GATE!(clk_spi2,"spi2","ahb_clk",CMU_DEVCLKEN1,6,0,0); static OWL_GATE!(clk_spi3,"spi3","ahb_clk",CMU_DEVCLKEN1,7,0,0);
static OWL_GATE_NO_PARENT!(clk_usb2h0_pllen,"usbh0_pllen",CMU_USBPLL,12,0,0); static OWL_GATE_NO_PARENT!(clk_usb2h0_phy,"usbh0_phy",CMU_USBPLL,10,0,0); static OWL_GATE_NO_PARENT!(clk_usb2h0_cce,"usbh0_cce",CMU_DEVCLKEN0,26,0,0);
static OWL_GATE_NO_PARENT!(clk_usb2h1_pllen,"usbh1_pllen",CMU_USBPLL,13,0,0); static OWL_GATE_NO_PARENT!(clk_usb2h1_phy,"usbh1_phy",CMU_USBPLL,11,0,0); static OWL_GATE_NO_PARENT!(clk_usb2h1_cce,"usbh1_cce",CMU_DEVCLKEN0,27,0,0); static OWL_GATE_NO_PARENT!(clk_irc_switch,"irc_switch",CMU_DEVCLKEN1,15,0,0);

static OWL_COMP_DIV!(clk_csi,"csi",csi_clk_mux_p,OWL_MUX_HW!(CMU_CSICLK,4,1),OWL_GATE_HW!(CMU_DEVCLKEN0,13,0),OWL_DIVIDER_HW!(CMU_CSICLK,0,4,0,None),0);
static OWL_COMP_DIV!(clk_si,"si",csi_clk_mux_p,OWL_MUX_HW!(CMU_SICLK,4,1),OWL_GATE_HW!(CMU_DEVCLKEN0,14,0),OWL_DIVIDER_HW!(CMU_SICLK,0,4,0,None),0);
static OWL_COMP_FACTOR!(clk_de,"de",de_clk_mux_p,OWL_MUX_HW!(CMU_DECLK,12,1),OWL_GATE_HW!(CMU_DEVCLKEN0,0,0),OWL_FACTOR_HW!(CMU_DECLK,0,3,0,de_factor_table),0);
static OWL_COMP_FACTOR!(clk_hde,"hde",hde_clk_mux_p,OWL_MUX_HW!(CMU_HDECLK,4,2),OWL_GATE_HW!(CMU_DEVCLKEN0,9,0),OWL_FACTOR_HW!(CMU_HDECLK,0,3,0,hde_factor_table),0);
static OWL_COMP_FACTOR!(clk_vde,"vde",hde_clk_mux_p,OWL_MUX_HW!(CMU_VDECLK,4,2),OWL_GATE_HW!(CMU_DEVCLKEN0,10,0),OWL_FACTOR_HW!(CMU_VDECLK,0,3,0,hde_factor_table),0);
static OWL_COMP_FACTOR!(clk_vce,"vce",hde_clk_mux_p,OWL_MUX_HW!(CMU_VCECLK,4,2),OWL_GATE_HW!(CMU_DEVCLKEN0,11,0),OWL_FACTOR_HW!(CMU_VCECLK,0,3,0,hde_factor_table),0);
static OWL_COMP_DIV!(clk_nand,"nand",nand_clk_mux_p,OWL_MUX_HW!(CMU_NANDCCLK,8,2),OWL_GATE_HW!(CMU_DEVCLKEN0,21,0),OWL_DIVIDER_HW!(CMU_NANDCCLK,0,3,0,None),CLK_SET_RATE_PARENT);
static OWL_COMP_FACTOR!(clk_sd0,"sd0",sd_clk_mux_p,OWL_MUX_HW!(CMU_SD0CLK,9,1),OWL_GATE_HW!(CMU_DEVCLKEN0,22,0),OWL_FACTOR_HW!(CMU_SD0CLK,0,9,0,sd_factor_table),0);
static OWL_COMP_FACTOR!(clk_sd1,"sd1",sd_clk_mux_p,OWL_MUX_HW!(CMU_SD1CLK,9,1),OWL_GATE_HW!(CMU_DEVCLKEN0,23,0),OWL_FACTOR_HW!(CMU_SD1CLK,0,9,0,sd_factor_table),0);
static OWL_COMP_FACTOR!(clk_sd2,"sd2",sd_clk_mux_p,OWL_MUX_HW!(CMU_SD2CLK,9,1),OWL_GATE_HW!(CMU_DEVCLKEN0,24,0),OWL_FACTOR_HW!(CMU_SD2CLK,0,9,0,sd_factor_table),0);
static OWL_COMP_DIV!(clk_uart0,"uart0",uart_clk_mux_p,OWL_MUX_HW!(CMU_UART0CLK,16,1),OWL_GATE_HW!(CMU_DEVCLKEN1,8,0),OWL_DIVIDER_HW!(CMU_UART0CLK,0,9,CLK_DIVIDER_ROUND_CLOSEST,None),0);
static OWL_COMP_DIV!(clk_uart1,"uart1",uart_clk_mux_p,OWL_MUX_HW!(CMU_UART1CLK,16,1),OWL_GATE_HW!(CMU_DEVCLKEN1,9,0),OWL_DIVIDER_HW!(CMU_UART1CLK,0,9,CLK_DIVIDER_ROUND_CLOSEST,None),0);
static OWL_COMP_DIV!(clk_uart2,"uart2",uart_clk_mux_p,OWL_MUX_HW!(CMU_UART2CLK,16,1),OWL_GATE_HW!(CMU_DEVCLKEN1,10,0),OWL_DIVIDER_HW!(CMU_UART2CLK,0,9,CLK_DIVIDER_ROUND_CLOSEST,None),0);
static OWL_COMP_DIV!(clk_uart3,"uart3",uart_clk_mux_p,OWL_MUX_HW!(CMU_UART3CLK,16,1),OWL_GATE_HW!(CMU_DEVCLKEN1,11,0),OWL_DIVIDER_HW!(CMU_UART3CLK,0,9,CLK_DIVIDER_ROUND_CLOSEST,None),0);
static OWL_COMP_DIV!(clk_uart4,"uart4",uart_clk_mux_p,OWL_MUX_HW!(CMU_UART4CLK,16,1),OWL_GATE_HW!(CMU_DEVCLKEN1,12,0),OWL_DIVIDER_HW!(CMU_UART4CLK,0,9,CLK_DIVIDER_ROUND_CLOSEST,None),0);
static OWL_COMP_DIV!(clk_uart5,"uart5",uart_clk_mux_p,OWL_MUX_HW!(CMU_UART5CLK,16,1),OWL_GATE_HW!(CMU_DEVCLKEN1,13,0),OWL_DIVIDER_HW!(CMU_UART5CLK,0,9,CLK_DIVIDER_ROUND_CLOSEST,None),0);
static OWL_COMP_DIV!(clk_uart6,"uart6",uart_clk_mux_p,OWL_MUX_HW!(CMU_UART6CLK,16,1),OWL_GATE_HW!(CMU_DEVCLKEN1,14,0),OWL_DIVIDER_HW!(CMU_UART6CLK,0,9,CLK_DIVIDER_ROUND_CLOSEST,None),0);
static OWL_COMP_DIV!(clk_pwm0,"pwm0",pwm_clk_mux_p,OWL_MUX_HW!(CMU_PWM0CLK,12,1),OWL_GATE_HW!(CMU_DEVCLKEN1,16,0),OWL_DIVIDER_HW!(CMU_PWM0CLK,0,10,0,None),CLK_IGNORE_UNUSED);
static OWL_COMP_DIV!(clk_pwm1,"pwm1",pwm_clk_mux_p,OWL_MUX_HW!(CMU_PWM1CLK,12,1),OWL_GATE_HW!(CMU_DEVCLKEN1,17,0),OWL_DIVIDER_HW!(CMU_PWM1CLK,0,10,0,None),0);
static OWL_COMP_DIV!(clk_pwm2,"pwm2",pwm_clk_mux_p,OWL_MUX_HW!(CMU_PWM2CLK,12,1),OWL_GATE_HW!(CMU_DEVCLKEN1,18,0),OWL_DIVIDER_HW!(CMU_PWM2CLK,0,10,0,None),0);
static OWL_COMP_DIV!(clk_pwm3,"pwm3",pwm_clk_mux_p,OWL_MUX_HW!(CMU_PWM3CLK,12,1),OWL_GATE_HW!(CMU_DEVCLKEN1,19,0),OWL_DIVIDER_HW!(CMU_PWM3CLK,0,10,0,None),0);
static OWL_COMP_DIV!(clk_pwm4,"pwm4",pwm_clk_mux_p,OWL_MUX_HW!(CMU_PWM4CLK,12,1),OWL_GATE_HW!(CMU_DEVCLKEN1,20,0),OWL_DIVIDER_HW!(CMU_PWM4CLK,0,10,0,None),0);
static OWL_COMP_DIV!(clk_pwm5,"pwm5",pwm_clk_mux_p,OWL_MUX_HW!(CMU_PWM5CLK,12,1),OWL_GATE_HW!(CMU_DEVCLKEN1,21,0),OWL_DIVIDER_HW!(CMU_PWM5CLK,0,10,0,None),0);
static OWL_COMP_FACTOR!(clk_gpu3d,"gpu3d",gpu_clk_mux_p,OWL_MUX_HW!(CMU_GPU3DCLK,4,3),OWL_GATE_HW!(CMU_DEVCLKEN0,8,0),OWL_FACTOR_HW!(CMU_GPU3DCLK,0,3,0,hde_factor_table),0);
static OWL_COMP_FACTOR!(clk_lcd,"lcd",lcd_clk_mux_p,OWL_MUX_HW!(CMU_LCDCLK,12,2),OWL_GATE_HW!(CMU_DEVCLKEN0,1,0),OWL_FACTOR_HW!(CMU_LCDCLK,0,9,0,lcd_factor_table),0);
static OWL_COMP_DIV!(clk_hdmi_audio,"hdmia",i2s_clk_mux_p,OWL_MUX_HW!(CMU_AUDIOPLL,24,1),OWL_GATE_HW!(CMU_DEVCLKEN1,28,0),OWL_DIVIDER_HW!(CMU_AUDIOPLL,24,4,0,hdmia_div_table),0);
static OWL_COMP_DIV!(clk_i2srx,"i2srx",i2s_clk_mux_p,OWL_MUX_HW!(CMU_AUDIOPLL,24,1),OWL_GATE_HW!(CMU_DEVCLKEN1,27,0),OWL_DIVIDER_HW!(CMU_AUDIOPLL,20,4,0,hdmia_div_table),0);
static OWL_COMP_DIV!(clk_i2stx,"i2stx",i2s_clk_mux_p,OWL_MUX_HW!(CMU_AUDIOPLL,24,1),OWL_GATE_HW!(CMU_DEVCLKEN1,26,0),OWL_DIVIDER_HW!(CMU_AUDIOPLL,16,4,0,hdmia_div_table),0);

static OWL_COMP_FIXED_FACTOR!(clk_pcm1,"pcm1","audio_pll",OWL_GATE_HW!(CMU_DEVCLKEN1,31,0),1,2,0);
static OWL_COMP_DIV!(clk_sensor_src,"sensor_src",sensor_clk_mux_p,OWL_MUX_HW!(CMU_SENSORCLK,4,1),{0},OWL_DIVIDER_HW!(CMU_SENSORCLK,5,2,0,None),0);
static OWL_COMP_FIXED_FACTOR!(clk_ethernet,"ethernet","ethernet_pll",OWL_GATE_HW!(CMU_DEVCLKEN1,23,0),1,20,0);
static OWL_COMP_DIV_FIXED!(clk_thermal_sensor,"thermal_sensor","hosc",OWL_GATE_HW!(CMU_DEVCLKEN0,31,0),OWL_DIVIDER_HW!(CMU_SSTSCLK,20,10,0,None),0);

// The following arrays and descriptor retain the C driver's externally visible
// registration topology; their element types are provided by the kernel port.
static s700_clks: [&owl_clk_common; 80] = [
    &clk_core_pll.common,&clk_dev_pll.common,&clk_ddr_pll.common,&clk_nand_pll.common,&clk_display_pll.common,&clk_cvbs_pll.common,&clk_audio_pll.common,&clk_ethernet_pll.common,&clk_cpu.common,&clk_dev.common,&clk_ahb.common,&clk_apb.common,&clk_dmac.common,&clk_noc0_clk_mux.common,&clk_noc1_clk_mux.common,&clk_hp_clk_mux.common,&clk_hp_clk_div.common,&clk_noc1_clk_div.common,&clk_noc0.common,&clk_noc1.common,&clk_sensor_src.common,&clk_gpio.common,&clk_timer.common,&clk_dsi.common,&clk_csi.common,&clk_si.common,&clk_de.common,&clk_hde.common,&clk_vde.common,&clk_vce.common,&clk_nand.common,&clk_sd0.common,&clk_sd1.common,&clk_sd2.common,&clk_uart0.common,&clk_uart1.common,&clk_uart2.common,&clk_uart3.common,&clk_uart4.common,&clk_uart5.common,&clk_uart6.common,&clk_pwm0.common,&clk_pwm1.common,&clk_pwm2.common,&clk_pwm3.common,&clk_pwm4.common,&clk_pwm5.common,&clk_gpu3d.common,&clk_i2c0.common,&clk_i2c1.common,&clk_i2c2.common,&clk_i2c3.common,&clk_spi0.common,&clk_spi1.common,&clk_spi2.common,&clk_spi3.common,&clk_usb3_480mpll0.common,&clk_usb3_480mphy0.common,&clk_usb3_5gphy.common,&clk_usb3_cce.common,&clk_lcd.common,&clk_hdmi_audio.common,&clk_i2srx.common,&clk_i2stx.common,&clk_sensor0.common,&clk_sensor1.common,&clk_hdmi_dev.common,&clk_ethernet.common,&clk_rmii_ref.common,&clk_usb2h0_pllen.common,&clk_usb2h0_phy.common,&clk_usb2h0_cce.common,&clk_usb2h1_pllen.common,&clk_usb2h1_phy.common,&clk_usb2h1_cce.common,&clk_tvout.common,&clk_thermal_sensor.common,&clk_irc_switch.common,&clk_pcm1.common,
];
static mut s700_hw_clks: clk_hw_onecell_data = clk_hw_onecell_data {
    hws: [
        [CLK_CORE_PLL] = &clk_core_pll.common.hw, [CLK_DEV_PLL] = &clk_dev_pll.common.hw,
        [CLK_DDR_PLL] = &clk_ddr_pll.common.hw, [CLK_NAND_PLL] = &clk_nand_pll.common.hw,
        [CLK_DISPLAY_PLL] = &clk_display_pll.common.hw, [CLK_CVBS_PLL] = &clk_cvbs_pll.common.hw,
        [CLK_AUDIO_PLL] = &clk_audio_pll.common.hw, [CLK_ETHERNET_PLL] = &clk_ethernet_pll.common.hw,
        [CLK_CPU] = &clk_cpu.common.hw, [CLK_DEV] = &clk_dev.common.hw, [CLK_AHB] = &clk_ahb.common.hw,
        [CLK_APB] = &clk_apb.common.hw, [CLK_DMAC] = &clk_dmac.common.hw, [CLK_NOC0_CLK_MUX] = &clk_noc0_clk_mux.common.hw,
        [CLK_NOC1_CLK_MUX] = &clk_noc1_clk_mux.common.hw, [CLK_HP_CLK_MUX] = &clk_hp_clk_mux.common.hw,
        [CLK_HP_CLK_DIV] = &clk_hp_clk_div.common.hw, [CLK_NOC1_CLK_DIV] = &clk_noc1_clk_div.common.hw,
        [CLK_NOC0] = &clk_noc0.common.hw, [CLK_NOC1] = &clk_noc1.common.hw, [CLK_SENOR_SRC] = &clk_sensor_src.common.hw,
        [CLK_GPIO] = &clk_gpio.common.hw, [CLK_TIMER] = &clk_timer.common.hw, [CLK_DSI] = &clk_dsi.common.hw,
        [CLK_CSI] = &clk_csi.common.hw, [CLK_SI] = &clk_si.common.hw, [CLK_DE] = &clk_de.common.hw,
        [CLK_HDE] = &clk_hde.common.hw, [CLK_VDE] = &clk_vde.common.hw, [CLK_VCE] = &clk_vce.common.hw,
        [CLK_NAND] = &clk_nand.common.hw, [CLK_SD0] = &clk_sd0.common.hw, [CLK_SD1] = &clk_sd1.common.hw,
        [CLK_SD2] = &clk_sd2.common.hw, [CLK_UART0] = &clk_uart0.common.hw, [CLK_UART1] = &clk_uart1.common.hw,
        [CLK_UART2] = &clk_uart2.common.hw, [CLK_UART3] = &clk_uart3.common.hw, [CLK_UART4] = &clk_uart4.common.hw,
        [CLK_UART5] = &clk_uart5.common.hw, [CLK_UART6] = &clk_uart6.common.hw, [CLK_PWM0] = &clk_pwm0.common.hw,
        [CLK_PWM1] = &clk_pwm1.common.hw, [CLK_PWM2] = &clk_pwm2.common.hw, [CLK_PWM3] = &clk_pwm3.common.hw,
        [CLK_PWM4] = &clk_pwm4.common.hw, [CLK_PWM5] = &clk_pwm5.common.hw, [CLK_GPU3D] = &clk_gpu3d.common.hw,
        [CLK_I2C0] = &clk_i2c0.common.hw, [CLK_I2C1] = &clk_i2c1.common.hw, [CLK_I2C2] = &clk_i2c2.common.hw,
        [CLK_I2C3] = &clk_i2c3.common.hw, [CLK_SPI0] = &clk_spi0.common.hw, [CLK_SPI1] = &clk_spi1.common.hw,
        [CLK_SPI2] = &clk_spi2.common.hw, [CLK_SPI3] = &clk_spi3.common.hw, [CLK_USB3_480MPLL0] = &clk_usb3_480mpll0.common.hw,
        [CLK_USB3_480MPHY0] = &clk_usb3_480mphy0.common.hw, [CLK_USB3_5GPHY] = &clk_usb3_5gphy.common.hw,
        [CLK_USB3_CCE] = &clk_usb3_cce.common.hw, [CLK_LCD] = &clk_lcd.common.hw, [CLK_HDMI_AUDIO] = &clk_hdmi_audio.common.hw,
        [CLK_I2SRX] = &clk_i2srx.common.hw, [CLK_I2STX] = &clk_i2stx.common.hw, [CLK_SENSOR0] = &clk_sensor0.common.hw,
        [CLK_SENSOR1] = &clk_sensor1.common.hw, [CLK_HDMI_DEV] = &clk_hdmi_dev.common.hw, [CLK_ETHERNET] = &clk_ethernet.common.hw,
        [CLK_RMII_REF] = &clk_rmii_ref.common.hw, [CLK_USB2H0_PLLEN] = &clk_usb2h0_pllen.common.hw,
        [CLK_USB2H0_PHY] = &clk_usb2h0_phy.common.hw, [CLK_USB2H0_CCE] = &clk_usb2h0_cce.common.hw,
        [CLK_USB2H1_PLLEN] = &clk_usb2h1_pllen.common.hw, [CLK_USB2H1_PHY] = &clk_usb2h1_phy.common.hw,
        [CLK_USB2H1_CCE] = &clk_usb2h1_cce.common.hw, [CLK_TVOUT] = &clk_tvout.common.hw,
        [CLK_THERMAL_SENSOR] = &clk_thermal_sensor.common.hw, [CLK_IRC_SWITCH] = &clk_irc_switch.common.hw,
        [CLK_PCM1] = &clk_pcm1.common.hw,
    ], num: CLK_NR_CLKS,
};

static s700_resets: [owl_reset_map; 21] = [
    [RESET_DE] = owl_reset_map { reg: CMU_DEVRST0, bit: BIT(0) },
    [RESET_LCD0] = owl_reset_map { reg: CMU_DEVRST0, bit: BIT(1) },
    [RESET_DSI] = owl_reset_map { reg: CMU_DEVRST0, bit: BIT(2) },
    [RESET_CSI] = owl_reset_map { reg: CMU_DEVRST0, bit: BIT(13) },
    [RESET_SI] = owl_reset_map { reg: CMU_DEVRST0, bit: BIT(14) },
    [RESET_I2C0] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(0) }, [RESET_I2C1] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(1) }, [RESET_I2C2] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(2) }, [RESET_I2C3] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(3) },
    [RESET_SPI0] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(4) }, [RESET_SPI1] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(5) }, [RESET_SPI2] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(6) }, [RESET_SPI3] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(7) },
    [RESET_UART0] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(8) }, [RESET_UART1] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(9) }, [RESET_UART2] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(10) }, [RESET_UART3] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(11) }, [RESET_UART4] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(12) }, [RESET_UART5] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(13) }, [RESET_UART6] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(14) },
    [RESET_KEY] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(24) }, [RESET_GPIO] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(25) }, [RESET_AUDIO] = owl_reset_map { reg: CMU_DEVRST1, bit: BIT(29) },
];

static mut s700_clk_desc: owl_clk_desc = owl_clk_desc { clks: s700_clks, num_clks: ARRAY_SIZE!(s700_clks), hw_clks: &s700_hw_clks, resets: s700_resets, num_resets: ARRAY_SIZE!(s700_resets) };

unsafe fn s700_clk_probe(pdev: *mut platform_device) -> i32 {
    let desc: *mut owl_clk_desc = &raw mut s700_clk_desc;
    owl_clk_regmap_init(pdev, desc);
    /* FIXME: Reset controller registration should be moved to common code. */
    let reset = devm_kzalloc(&(*pdev).dev, core::mem::size_of::<owl_reset>(), GFP_KERNEL);
    if reset.is_null() { return -ENOMEM; }
    (*reset).rcdev.of_node = (*pdev).dev.of_node;
    (*reset).rcdev.ops = &owl_reset_ops;
    (*reset).rcdev.nr_resets = (*desc).num_resets;
    (*reset).reset_map = (*desc).resets;
    (*reset).regmap = (*desc).regmap;
    let ret = devm_reset_controller_register(&(*pdev).dev, &mut (*reset).rcdev);
    if ret != 0 { dev_err(&(*pdev).dev, "Failed to register reset controller\n"); }
    owl_clk_probe(&(*pdev).dev, (*desc).hw_clks)
}

static s700_clk_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "actions,s700-cmu" }, of_device_id { compatible: "" },
];
static mut s700_clk_driver: platform_driver = platform_driver { probe: Some(s700_clk_probe), driver: driver { name: "s700-cmu", of_match_table: s700_clk_of_match } };

unsafe fn s700_clk_init() -> i32 { platform_driver_register(&mut s700_clk_driver) }
core_initcall!(s700_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
