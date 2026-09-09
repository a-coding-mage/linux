// SPDX-License-Identifier: GPL-2.0+
// Actions Semi Owl S500 SoC clock driver
// Rust translation of the C implementation; external kernel types/macros are dependencies.

const CMU_COREPLL: u32 = 0x0000; const CMU_DEVPLL: u32 = 0x0004;
const CMU_DDRPLL: u32 = 0x0008; const CMU_NANDPLL: u32 = 0x000c;
const CMU_DISPLAYPLL: u32 = 0x0010; const CMU_AUDIOPLL: u32 = 0x0014;
const CMU_TVOUTPLL: u32 = 0x0018; const CMU_BUSCLK: u32 = 0x001c;
const CMU_SENSORCLK: u32 = 0x0020; const CMU_LCDCLK: u32 = 0x0024;
const CMU_DSICLK: u32 = 0x0028; const CMU_CSICLK: u32 = 0x002c;
const CMU_DECLK: u32 = 0x0030; const CMU_BISPCLK: u32 = 0x0034;
const CMU_BUSCLK1: u32 = 0x0038; const CMU_VDECLK: u32 = 0x0040;
const CMU_VCECLK: u32 = 0x0044; const CMU_NANDCCLK: u32 = 0x004c;
const CMU_SD0CLK: u32 = 0x0050; const CMU_SD1CLK: u32 = 0x0054;
const CMU_SD2CLK: u32 = 0x0058; const CMU_UART0CLK: u32 = 0x005c;
const CMU_UART1CLK: u32 = 0x0060; const CMU_UART2CLK: u32 = 0x0064;
const CMU_PWM4CLK: u32 = 0x0068; const CMU_PWM5CLK: u32 = 0x006c;
const CMU_PWM0CLK: u32 = 0x0070; const CMU_PWM1CLK: u32 = 0x0074;
const CMU_PWM2CLK: u32 = 0x0078; const CMU_PWM3CLK: u32 = 0x007c;
const CMU_USBPLL: u32 = 0x0080; const CMU_ETHERNETPLL: u32 = 0x0084;
const CMU_CVBSPLL: u32 = 0x0088; const CMU_LENSCLK: u32 = 0x008c;
const CMU_GPU3DCLK: u32 = 0x0090; const CMU_CORECTL: u32 = 0x009c;
const CMU_DEVCLKEN0: u32 = 0x00a0; const CMU_DEVCLKEN1: u32 = 0x00a4;
const CMU_DEVRST0: u32 = 0x00a8; const CMU_DEVRST1: u32 = 0x00ac;
const CMU_UART3CLK: u32 = 0x00b0; const CMU_UART4CLK: u32 = 0x00b4;
const CMU_UART5CLK: u32 = 0x00b8; const CMU_UART6CLK: u32 = 0x00bc;
const CMU_SSCLK: u32 = 0x00c0; const CMU_DIGITALDEBUG: u32 = 0x00d0;
const CMU_ANALOGDEBUG: u32 = 0x00d4; const CMU_COREPLLDEBUG: u32 = 0x00d8;
const CMU_DEVPLLDEBUG: u32 = 0x00dc; const CMU_DDRPLLDEBUG: u32 = 0x00e0;
const CMU_NANDPLLDEBUG: u32 = 0x00e4; const CMU_DISPLAYPLLDEBUG: u32 = 0x00e8;
const CMU_TVOUTPLLDEBUG: u32 = 0x00ec; const CMU_DEEPCOLORPLLDEBUG: u32 = 0x00f4;
const CMU_AUDIOPLL_ETHPLLDEBUG: u32 = 0x00f8; const CMU_CVBSPLLDEBUG: u32 = 0x00fc;
const OWL_S500_COREPLL_DELAY: u32 = 150; const OWL_S500_DDRPLL_DELAY: u32 = 63;
const OWL_S500_DEVPLL_DELAY: u32 = 28; const OWL_S500_NANDPLL_DELAY: u32 = 44;
const OWL_S500_DISPLAYPLL_DELAY: u32 = 57; const OWL_S500_ETHERNETPLL_DELAY: u32 = 25;
const OWL_S500_AUDIOPLL_DELAY: u32 = 100;

static clk_audio_pll_table: [clk_pll_table; 3] = [clk_pll_table { val: 0, rate: 45158400 }, clk_pll_table { val: 1, rate: 49152000 }, clk_pll_table::sentinel()];

owl_pll_no_parent_delay!(ethernet_pll_clk, "ethernet_pll_clk", CMU_ETHERNETPLL, 500000000, 0, 0, 0, 0, 0, OWL_S500_ETHERNETPLL_DELAY, None, CLK_IGNORE_UNUSED);
owl_pll_no_parent_delay!(core_pll_clk, "core_pll_clk", CMU_COREPLL, 12000000, 9, 0, 8, 4, 134, OWL_S500_COREPLL_DELAY, None, CLK_IGNORE_UNUSED);
owl_pll_no_parent_delay!(ddr_pll_clk, "ddr_pll_clk", CMU_DDRPLL, 12000000, 8, 0, 8, 1, 67, OWL_S500_DDRPLL_DELAY, None, CLK_IGNORE_UNUSED);
owl_pll_no_parent_delay!(nand_pll_clk, "nand_pll_clk", CMU_NANDPLL, 6000000, 8, 0, 7, 2, 86, OWL_S500_NANDPLL_DELAY, None, CLK_IGNORE_UNUSED);
owl_pll_no_parent_delay!(display_pll_clk, "display_pll_clk", CMU_DISPLAYPLL, 6000000, 8, 0, 8, 2, 126, OWL_S500_DISPLAYPLL_DELAY, None, CLK_IGNORE_UNUSED);
owl_pll_no_parent_delay!(dev_pll_clk, "dev_pll_clk", CMU_DEVPLL, 6000000, 8, 0, 7, 8, 126, OWL_S500_DEVPLL_DELAY, None, CLK_IGNORE_UNUSED);
owl_pll_no_parent_delay!(audio_pll_clk, "audio_pll_clk", CMU_AUDIOPLL, 0, 4, 0, 1, 0, 0, OWL_S500_AUDIOPLL_DELAY, Some(&clk_audio_pll_table), CLK_IGNORE_UNUSED);

static dev_clk_mux_p: &[&str] = &["hosc", "dev_pll_clk"]; static bisp_clk_mux_p: &[&str] = &["display_pll_clk", "dev_clk"];
static sensor_clk_mux_p: &[&str] = &["hosc", "bisp_clk"]; static sd_clk_mux_p: &[&str] = &["dev_clk", "nand_pll_clk"];
static pwm_clk_mux_p: &[&str] = &["losc", "hosc"]; static ahbprediv_clk_mux_p: &[&str] = &["dev_clk", "display_pll_clk", "nand_pll_clk", "ddr_pll_clk"];
static nic_clk_mux_p: &[&str] = ahbprediv_clk_mux_p; static uart_clk_mux_p: &[&str] = &["hosc", "dev_pll_clk"];
static de_clk_mux_p: &[&str] = &["display_pll_clk", "dev_clk"]; static i2s_clk_mux_p: &[&str] = &["audio_pll_clk"];
static hde_clk_mux_p: &[&str] = ahbprediv_clk_mux_p; static nand_clk_mux_p: &[&str] = &["nand_pll_clk", "display_pll_clk", "dev_clk", "ddr_pll_clk"];

static sd_factor_table: &[clk_factor_table] = &[
    factor!(0,1,1),factor!(1,1,2),factor!(2,1,3),factor!(3,1,4),factor!(4,1,5),factor!(5,1,6),factor!(6,1,7),factor!(7,1,8),factor!(8,1,9),factor!(9,1,10),factor!(10,1,11),factor!(11,1,12),factor!(12,1,13),factor!(13,1,14),factor!(14,1,15),factor!(15,1,16),factor!(16,1,17),factor!(17,1,18),factor!(18,1,19),factor!(19,1,20),factor!(20,1,21),factor!(21,1,22),factor!(22,1,23),factor!(23,1,24),factor!(24,1,25),
    factor!(256,1,128),factor!(257,1,256),factor!(258,1,384),factor!(259,1,512),factor!(260,1,640),factor!(261,1,768),factor!(262,1,896),factor!(263,1,1024),factor!(264,1,1152),factor!(265,1,1280),factor!(266,1,1408),factor!(267,1,1536),factor!(268,1,1664),factor!(269,1,1792),factor!(270,1,1920),factor!(271,1,2048),factor!(272,1,2176),factor!(273,1,2304),factor!(274,1,2432),factor!(275,1,2560),factor!(276,1,2688),factor!(277,1,2816),factor!(278,1,2944),factor!(279,1,3072),factor!(280,1,3200), sentinel!()
];
static de_factor_table: &[clk_factor_table] = &[factor!(0,1,1),factor!(1,2,3),factor!(2,1,2),factor!(3,2,5),factor!(4,1,3),factor!(5,1,4),factor!(6,1,6),factor!(7,1,8),factor!(8,1,12),sentinel!()];
static hde_factor_table: &[clk_factor_table] = &[factor!(0,1,1),factor!(1,2,3),factor!(2,1,2),factor!(3,2,5),factor!(4,1,3),factor!(5,1,4),factor!(6,1,6),factor!(7,1,8),sentinel!()];
static rmii_ref_div_table: &[clk_div_table] = &[div!(0,4),div!(1,10),sentinel!()];
static std12rate_div_table: &[clk_div_table] = &[div!(0,1),div!(1,2),div!(2,3),div!(3,4),div!(4,5),div!(5,6),div!(6,7),div!(7,8),div!(8,9),div!(9,10),div!(10,11),div!(11,12),sentinel!()];
static i2s_div_table: &[clk_div_table] = &[div!(0,1),div!(1,2),div!(2,3),div!(3,4),div!(4,6),div!(5,8),div!(6,12),div!(7,16),div!(8,24),sentinel!()];
static nand_div_table: &[clk_div_table] = &[div!(0,1),div!(1,2),div!(2,4),div!(3,6),div!(4,8),div!(5,10),div!(6,12),div!(7,14),div!(8,16),div!(9,18),div!(10,20),div!(11,22),sentinel!()];

owl_mux!(dev_clk,"dev_clk",dev_clk_mux_p,CMU_DEVPLL,12,1,CLK_SET_RATE_PARENT);
owl_gate!(gpio_clk,"gpio_clk","apb_clk",CMU_DEVCLKEN0,18,0,0); owl_gate!(dmac_clk,"dmac_clk","h_clk",CMU_DEVCLKEN0,1,0,0);
owl_gate!(spi0_clk,"spi0_clk","ahb_clk",CMU_DEVCLKEN1,10,0,CLK_IGNORE_UNUSED); owl_gate!(spi1_clk,"spi1_clk","ahb_clk",CMU_DEVCLKEN1,11,0,CLK_IGNORE_UNUSED); owl_gate!(spi2_clk,"spi2_clk","ahb_clk",CMU_DEVCLKEN1,12,0,CLK_IGNORE_UNUSED); owl_gate!(spi3_clk,"spi3_clk","ahb_clk",CMU_DEVCLKEN1,13,0,CLK_IGNORE_UNUSED);
owl_gate!(timer_clk,"timer_clk","hosc",CMU_DEVCLKEN1,27,0,0); owl_gate!(hdmi_clk,"hdmi_clk","hosc",CMU_DEVCLKEN1,3,0,0);
owl_divider!(h_clk,"h_clk","ahbprediv_clk",CMU_BUSCLK1,2,2,None,0,0); owl_divider!(apb_clk,"apb_clk","nic_clk",CMU_BUSCLK1,14,2,None,0,0); owl_divider!(rmii_ref_clk,"rmii_ref_clk","ethernet_pll_clk",CMU_ETHERNETPLL,1,1,Some(rmii_ref_div_table),0,0);
owl_factor!(de1_clk,"de_clk1","de_clk",CMU_DECLK,0,4,de_factor_table,0,0); owl_factor!(de2_clk,"de_clk2","de_clk",CMU_DECLK,4,4,de_factor_table,0,0);

// Composite declarations preserve the original hardware mux, gate, divider, and factor layout.
owl_comp_div!(nic_clk,"nic_clk",nic_clk_mux_p,mux_hw!(CMU_BUSCLK1,4,3),gate_none!(),div_hw!(CMU_BUSCLK1,16,2,0,None),0);
owl_comp_div!(ahbprediv_clk,"ahbprediv_clk",ahbprediv_clk_mux_p,mux_hw!(CMU_BUSCLK1,8,3),gate_none!(),div_hw!(CMU_BUSCLK1,12,2,0,None),CLK_SET_RATE_PARENT);
owl_comp_fixed_factor!(ahb_clk,"ahb_clk","h_clk",gate_none!(),1,1,0);
owl_comp_factor!(vce_clk,"vce_clk",hde_clk_mux_p,mux_hw!(CMU_VCECLK,4,2),gate_hw!(CMU_DEVCLKEN0,26,0),factor_hw!(CMU_VCECLK,0,3,0,hde_factor_table),0);
owl_comp_factor!(vde_clk,"vde_clk",hde_clk_mux_p,mux_hw!(CMU_VDECLK,4,2),gate_hw!(CMU_DEVCLKEN0,25,0),factor_hw!(CMU_VDECLK,0,3,0,hde_factor_table),0);
owl_comp_div!(bisp_clk,"bisp_clk",bisp_clk_mux_p,mux_hw!(CMU_BISPCLK,4,1),gate_hw!(CMU_DEVCLKEN0,14,0),div_hw!(CMU_BISPCLK,0,4,0,Some(std12rate_div_table)),0);
owl_comp_div!(sensor0_clk,"sensor0_clk",sensor_clk_mux_p,mux_hw!(CMU_SENSORCLK,4,1),gate_hw!(CMU_DEVCLKEN0,14,0),div_hw!(CMU_SENSORCLK,0,4,0,Some(std12rate_div_table)),0);
owl_comp_div!(sensor1_clk,"sensor1_clk",sensor_clk_mux_p,mux_hw!(CMU_SENSORCLK,4,1),gate_hw!(CMU_DEVCLKEN0,14,0),div_hw!(CMU_SENSORCLK,8,4,0,Some(std12rate_div_table)),0);

// Remaining repetitive clock declarations retain the exact C names and register fields.
owl_comp_factor!(sd0_clk,"sd0_clk",sd_clk_mux_p,mux_hw!(CMU_SD0CLK,9,1),gate_hw!(CMU_DEVCLKEN0,5,0),factor_hw!(CMU_SD0CLK,0,9,0,sd_factor_table),0); owl_comp_factor!(sd1_clk,"sd1_clk",sd_clk_mux_p,mux_hw!(CMU_SD1CLK,9,1),gate_hw!(CMU_DEVCLKEN0,6,0),factor_hw!(CMU_SD1CLK,0,9,0,sd_factor_table),0); owl_comp_factor!(sd2_clk,"sd2_clk",sd_clk_mux_p,mux_hw!(CMU_SD2CLK,9,1),gate_hw!(CMU_DEVCLKEN0,7,0),factor_hw!(CMU_SD2CLK,0,9,0,sd_factor_table),0);
owl_comp_div!(pwm0_clk,"pwm0_clk",pwm_clk_mux_p,mux_hw!(CMU_PWM0CLK,12,1),gate_hw!(CMU_DEVCLKEN1,23,0),div_hw!(CMU_PWM0CLK,0,10,0,None),0); owl_comp_div!(pwm1_clk,"pwm1_clk",pwm_clk_mux_p,mux_hw!(CMU_PWM1CLK,12,1),gate_hw!(CMU_DEVCLKEN1,24,0),div_hw!(CMU_PWM1CLK,0,10,0,None),0); owl_comp_div!(pwm2_clk,"pwm2_clk",pwm_clk_mux_p,mux_hw!(CMU_PWM2CLK,12,1),gate_hw!(CMU_DEVCLKEN1,25,0),div_hw!(CMU_PWM2CLK,0,10,0,None),0); owl_comp_div!(pwm3_clk,"pwm3_clk",pwm_clk_mux_p,mux_hw!(CMU_PWM3CLK,12,1),gate_hw!(CMU_DEVCLKEN1,26,0),div_hw!(CMU_PWM3CLK,0,10,0,None),0); owl_comp_div!(pwm4_clk,"pwm4_clk",pwm_clk_mux_p,mux_hw!(CMU_PWM4CLK,12,1),gate_hw!(CMU_DEVCLKEN0,11,0),div_hw!(CMU_PWM4CLK,0,10,0,None),0); owl_comp_div!(pwm5_clk,"pwm5_clk",pwm_clk_mux_p,mux_hw!(CMU_PWM5CLK,12,1),gate_hw!(CMU_DEVCLKEN0,0,0),div_hw!(CMU_PWM5CLK,0,10,0,None),0);
owl_comp_pass!(de_clk,"de_clk",de_clk_mux_p,mux_hw!(CMU_DECLK,12,1),gate_hw!(CMU_DEVCLKEN0,8,0),0);
owl_comp_fixed_factor!(i2c0_clk,"i2c0_clk","ethernet_pll_clk",gate_hw!(CMU_DEVCLKEN1,14,0),1,5,0); owl_comp_fixed_factor!(i2c1_clk,"i2c1_clk","ethernet_pll_clk",gate_hw!(CMU_DEVCLKEN1,15,0),1,5,0); owl_comp_fixed_factor!(i2c2_clk,"i2c2_clk","ethernet_pll_clk",gate_hw!(CMU_DEVCLKEN1,30,0),1,5,0); owl_comp_fixed_factor!(i2c3_clk,"i2c3_clk","ethernet_pll_clk",gate_hw!(CMU_DEVCLKEN1,31,0),1,5,0); owl_comp_fixed_factor!(ethernet_clk,"ethernet_clk","ethernet_pll_clk",gate_hw!(CMU_DEVCLKEN1,22,0),1,20,0);

macro_rules! uart { ($n:ident,$s:literal,$r:ident,$b:expr,$f:expr) => { owl_comp_div!($n,$s,uart_clk_mux_p,mux_hw!($r,16,1),gate_hw!(CMU_DEVCLKEN1,$b,0),div_hw!($r,0,8,CLK_DIVIDER_ROUND_CLOSEST,None),CLK_IGNORE_UNUSED); }; }
uart!(uart0_clk,"uart0_clk",CMU_UART0CLK,6,0); uart!(uart1_clk,"uart1_clk",CMU_UART1CLK,7,0); uart!(uart2_clk,"uart2_clk",CMU_UART2CLK,8,0); uart!(uart3_clk,"uart3_clk",CMU_UART3CLK,19,0); uart!(uart4_clk,"uart4_clk",CMU_UART4CLK,20,0); uart!(uart5_clk,"uart5_clk",CMU_UART5CLK,21,0); uart!(uart6_clk,"uart6_clk",CMU_UART6CLK,18,0);
owl_comp_div!(i2srx_clk,"i2srx_clk",i2s_clk_mux_p,mux_hw!(CMU_AUDIOPLL,24,1),gate_hw!(CMU_DEVCLKEN0,21,0),div_hw!(CMU_AUDIOPLL,20,4,0,Some(i2s_div_table)),0); owl_comp_div!(i2stx_clk,"i2stx_clk",i2s_clk_mux_p,mux_hw!(CMU_AUDIOPLL,24,1),gate_hw!(CMU_DEVCLKEN0,20,0),div_hw!(CMU_AUDIOPLL,16,4,0,Some(i2s_div_table)),0); owl_comp_div!(hdmia_clk,"hdmia_clk",i2s_clk_mux_p,mux_hw!(CMU_AUDIOPLL,24,1),gate_hw!(CMU_DEVCLKEN0,22,0),div_hw!(CMU_AUDIOPLL,24,4,0,Some(i2s_div_table)),0); owl_comp_div!(spdif_clk,"spdif_clk",i2s_clk_mux_p,mux_hw!(CMU_AUDIOPLL,24,1),gate_hw!(CMU_DEVCLKEN0,23,0),div_hw!(CMU_AUDIOPLL,28,4,0,Some(i2s_div_table)),0);
owl_comp_div!(nand_clk,"nand_clk",nand_clk_mux_p,mux_hw!(CMU_NANDCCLK,8,2),gate_hw!(CMU_DEVCLKEN0,4,0),div_hw!(CMU_NANDCCLK,0,3,0,Some(nand_div_table)),CLK_SET_RATE_PARENT); owl_comp_div!(ecc_clk,"ecc_clk",nand_clk_mux_p,mux_hw!(CMU_NANDCCLK,8,2),gate_hw!(CMU_DEVCLKEN0,4,0),div_hw!(CMU_NANDCCLK,4,3,0,Some(nand_div_table)),CLK_SET_RATE_PARENT);

static s500_clks: &[&owl_clk_common] = &[&ethernet_pll_clk.common,&core_pll_clk.common,&ddr_pll_clk.common,&dev_pll_clk.common,&nand_pll_clk.common,&audio_pll_clk.common,&display_pll_clk.common,&dev_clk.common,&timer_clk.common,&i2c0_clk.common,&i2c1_clk.common,&i2c2_clk.common,&i2c3_clk.common,&uart0_clk.common,&uart1_clk.common,&uart2_clk.common,&uart3_clk.common,&uart4_clk.common,&uart5_clk.common,&uart6_clk.common,&pwm0_clk.common,&pwm1_clk.common,&pwm2_clk.common,&pwm3_clk.common,&pwm4_clk.common,&pwm5_clk.common,&sensor0_clk.common,&sensor1_clk.common,&sd0_clk.common,&sd1_clk.common,&sd2_clk.common,&bisp_clk.common,&ahb_clk.common,&ahbprediv_clk.common,&h_clk.common,&spi0_clk.common,&spi1_clk.common,&spi2_clk.common,&spi3_clk.common,&rmii_ref_clk.common,&de_clk.common,&de1_clk.common,&de2_clk.common,&i2srx_clk.common,&i2stx_clk.common,&hdmia_clk.common,&hdmi_clk.common,&vce_clk.common,&vde_clk.common,&spdif_clk.common,&nand_clk.common,&ecc_clk.common,&apb_clk.common,&dmac_clk.common,&gpio_clk.common,&nic_clk.common,&ethernet_clk.common];

static s500_resets: &[owl_reset_map] = &[
    reset!(RESET_DMAC,CMU_DEVRST0,0),reset!(RESET_NORIF,CMU_DEVRST0,1),reset!(RESET_DDR,CMU_DEVRST0,2),reset!(RESET_NANDC,CMU_DEVRST0,3),reset!(RESET_SD0,CMU_DEVRST0,4),reset!(RESET_SD1,CMU_DEVRST0,5),reset!(RESET_PCM1,CMU_DEVRST0,6),reset!(RESET_DE,CMU_DEVRST0,7),reset!(RESET_LCD,CMU_DEVRST0,8),reset!(RESET_SD2,CMU_DEVRST0,9),reset!(RESET_DSI,CMU_DEVRST0,10),reset!(RESET_CSI,CMU_DEVRST0,11),reset!(RESET_BISP,CMU_DEVRST0,12),reset!(RESET_KEY,CMU_DEVRST0,14),reset!(RESET_GPIO,CMU_DEVRST0,15),reset!(RESET_AUDIO,CMU_DEVRST0,17),reset!(RESET_PCM0,CMU_DEVRST0,18),reset!(RESET_VDE,CMU_DEVRST0,19),reset!(RESET_VCE,CMU_DEVRST0,20),reset!(RESET_GPU3D,CMU_DEVRST0,22),reset!(RESET_NIC301,CMU_DEVRST0,23),reset!(RESET_LENS,CMU_DEVRST0,26),reset!(RESET_PERIPHRESET,CMU_DEVRST0,27),
    reset!(RESET_USB2_0,CMU_DEVRST1,0),reset!(RESET_TVOUT,CMU_DEVRST1,1),reset!(RESET_HDMI,CMU_DEVRST1,2),reset!(RESET_HDCP2TX,CMU_DEVRST1,3),reset!(RESET_UART6,CMU_DEVRST1,4),reset!(RESET_UART0,CMU_DEVRST1,5),reset!(RESET_UART1,CMU_DEVRST1,6),reset!(RESET_UART2,CMU_DEVRST1,7),reset!(RESET_SPI0,CMU_DEVRST1,8),reset!(RESET_SPI1,CMU_DEVRST1,9),reset!(RESET_SPI2,CMU_DEVRST1,10),reset!(RESET_SPI3,CMU_DEVRST1,11),reset!(RESET_I2C0,CMU_DEVRST1,12),reset!(RESET_I2C1,CMU_DEVRST1,13),reset!(RESET_USB3,CMU_DEVRST1,14),reset!(RESET_UART3,CMU_DEVRST1,15),reset!(RESET_UART4,CMU_DEVRST1,16),reset!(RESET_UART5,CMU_DEVRST1,17),reset!(RESET_I2C2,CMU_DEVRST1,18),reset!(RESET_I2C3,CMU_DEVRST1,19),reset!(RESET_ETHERNET,CMU_DEVRST1,20),reset!(RESET_CHIPID,CMU_DEVRST1,21),reset!(RESET_USB2_1,CMU_DEVRST1,22),reset!(RESET_WD0RESET,CMU_DEVRST1,24),reset!(RESET_WD1RESET,CMU_DEVRST1,25),reset!(RESET_WD2RESET,CMU_DEVRST1,26),reset!(RESET_WD3RESET,CMU_DEVRST1,27),reset!(RESET_DBG0RESET,CMU_DEVRST1,28),reset!(RESET_DBG1RESET,CMU_DEVRST1,29),reset!(RESET_DBG2RESET,CMU_DEVRST1,30),reset!(RESET_DBG3RESET,CMU_DEVRST1,31)
];

static mut s500_clk_desc: owl_clk_desc = owl_clk_desc { clks: s500_clks, num_clks: ARRAY_SIZE!(s500_clks), hw_clks: &s500_hw_clks, resets: s500_resets, num_resets: ARRAY_SIZE!(s500_resets) };

unsafe fn s500_clk_probe(pdev: *mut platform_device) -> i32 {
    let desc: *mut owl_clk_desc = &raw mut s500_clk_desc;
    owl_clk_regmap_init(pdev, desc);
    let reset = devm_kzalloc((*pdev).dev(), core::mem::size_of::<owl_reset>(), GFP_KERNEL);
    if reset.is_null() { return -ENOMEM; }
    (*reset).rcdev.of_node = (*pdev).dev().of_node; (*reset).rcdev.ops = &owl_reset_ops;
    (*reset).rcdev.nr_resets = (*desc).num_resets; (*reset).reset_map = (*desc).resets; (*reset).regmap = (*desc).regmap;
    let ret = devm_reset_controller_register(pdev, &mut (*reset).rcdev);
    if ret != 0 { dev_err(pdev, "Failed to register reset controller\\n"); }
    owl_clk_probe(pdev, (*desc).hw_clks)
}

static s500_clk_of_match: &[of_device_id] = &[of_device_id { compatible: "actions,s500-cmu" }, of_device_id::sentinel()];
static mut s500_clk_driver: platform_driver = platform_driver { probe: Some(s500_clk_probe), driver: driver { name: "s500-cmu", of_match_table: s500_clk_of_match } };
unsafe fn s500_clk_init() -> i32 { platform_driver_register(&raw mut s500_clk_driver) }
core_initcall!(s500_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
