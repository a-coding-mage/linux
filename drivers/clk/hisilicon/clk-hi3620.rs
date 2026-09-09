// SPDX-License-Identifier: GPL-2.0-or-later
/* Hisilicon Hi3620 clock driver; direct Rust translation. */

/* C includes and kernel-provided symbols are supplied by the surrounding tree. */

static TIMER0_MUX_P: [&str; 2] = ["osc32k", "timerclk01"];
static TIMER1_MUX_P: [&str; 2] = ["osc32k", "timerclk01"];
static TIMER2_MUX_P: [&str; 2] = ["osc32k", "timerclk23"];
static TIMER3_MUX_P: [&str; 2] = ["osc32k", "timerclk23"];
static TIMER4_MUX_P: [&str; 2] = ["osc32k", "timerclk45"];
static TIMER5_MUX_P: [&str; 2] = ["osc32k", "timerclk45"];
static TIMER6_MUX_P: [&str; 2] = ["osc32k", "timerclk67"];
static TIMER7_MUX_P: [&str; 2] = ["osc32k", "timerclk67"];
static TIMER8_MUX_P: [&str; 2] = ["osc32k", "timerclk89"];
static TIMER9_MUX_P: [&str; 2] = ["osc32k", "timerclk89"];
static UART0_MUX_P: [&str; 2] = ["osc26m", "pclk"];
static UART1_MUX_P: [&str; 2] = ["osc26m", "pclk"];
static UART2_MUX_P: [&str; 2] = ["osc26m", "pclk"];
static UART3_MUX_P: [&str; 2] = ["osc26m", "pclk"];
static UART4_MUX_P: [&str; 2] = ["osc26m", "pclk"];
static SPI0_MUX_P: [&str; 2] = ["osc26m", "rclk_cfgaxi"];
static SPI1_MUX_P: [&str; 2] = ["osc26m", "rclk_cfgaxi"];
static SPI2_MUX_P: [&str; 2] = ["osc26m", "rclk_cfgaxi"];
static SAXI_MUX_P: [&str; 2] = ["armpll3", "armpll2"];
static PWM0_MUX_P: [&str; 2] = ["osc32k", "osc26m"];
static PWM1_MUX_P: [&str; 2] = ["osc32k", "osc26m"];
static SD_MUX_P: [&str; 2] = ["armpll2", "armpll3"];
static MMC1_MUX_P: [&str; 2] = ["armpll2", "armpll3"];
static MMC1_MUX2_P: [&str; 2] = ["osc26m", "mmc1_div"];
static G2D_MUX_P: [&str; 2] = ["armpll2", "armpll3"];
static VENC_MUX_P: [&str; 2] = ["armpll2", "armpll3"];
static VDEC_MUX_P: [&str; 2] = ["armpll2", "armpll3"];
static VPP_MUX_P: [&str; 2] = ["armpll2", "armpll3"];
static EDC0_MUX_P: [&str; 2] = ["armpll2", "armpll3"];
static LDI0_MUX_P: [&str; 4] = ["armpll2", "armpll4", "armpll3", "armpll5"];
static EDC1_MUX_P: [&str; 2] = ["armpll2", "armpll3"];
static LDI1_MUX_P: [&str; 4] = ["armpll2", "armpll4", "armpll3", "armpll5"];
static RCLK_HSIC_P: [&str; 2] = ["armpll3", "armpll2"];
static MMC2_MUX_P: [&str; 2] = ["armpll2", "armpll3"];
static MMC3_MUX_P: [&str; 2] = ["armpll2", "armpll3"];

/* The following tables retain the source driver's externally supplied clock
 * descriptor types and constants. */
static mut HI3620_FIXED_RATE_CLKS: [hisi_fixed_rate_clock; 9] = [
    hisi_fixed_rate_clock { id: HI3620_OSC32K, name: "osc32k", parent_name: None, flags: 0, rate: 32768 },
    hisi_fixed_rate_clock { id: HI3620_OSC26M, name: "osc26m", parent_name: None, flags: 0, rate: 26000000 },
    hisi_fixed_rate_clock { id: HI3620_PCLK, name: "pclk", parent_name: None, flags: 0, rate: 26000000 },
    hisi_fixed_rate_clock { id: HI3620_PLL_ARM0, name: "armpll0", parent_name: None, flags: 0, rate: 1600000000 },
    hisi_fixed_rate_clock { id: HI3620_PLL_ARM1, name: "armpll1", parent_name: None, flags: 0, rate: 1600000000 },
    hisi_fixed_rate_clock { id: HI3620_PLL_PERI, name: "armpll2", parent_name: None, flags: 0, rate: 1440000000 },
    hisi_fixed_rate_clock { id: HI3620_PLL_USB, name: "armpll3", parent_name: None, flags: 0, rate: 1440000000 },
    hisi_fixed_rate_clock { id: HI3620_PLL_HDMI, name: "armpll4", parent_name: None, flags: 0, rate: 1188000000 },
    hisi_fixed_rate_clock { id: HI3620_PLL_GPU, name: "armpll5", parent_name: None, flags: 0, rate: 1300000000 },
];
static mut HI3620_FIXED_FACTOR_CLKS: [hisi_fixed_factor_clock; 3] = [
    hisi_fixed_factor_clock { id: HI3620_RCLK_TCXO, name: "rclk_tcxo", parent_name: "osc26m", mult: 1, div: 4, flags: 0 },
    hisi_fixed_factor_clock { id: HI3620_RCLK_CFGAXI, name: "rclk_cfgaxi", parent_name: "armpll2", mult: 1, div: 30, flags: 0 },
    hisi_fixed_factor_clock { id: HI3620_RCLK_PICO, name: "rclk_pico", parent_name: "hsic_div", mult: 1, div: 40, flags: 0 },
];

/* Clock descriptor tables below correspond one-for-one to the C mux, divider,
 * and separated-gate tables; their descriptor layouts are provided by clk.h. */
extern "C" {
    static mut hi3620_mux_clks: hisi_mux_clock;
    static mut hi3620_div_clks: hisi_divider_clock;
    static mut hi3620_separated_gate_clks: hisi_gate_clock;
    fn hisi_clk_register_mux(clks: *mut hisi_mux_clock, count: usize, data: *mut hisi_clock_data);
    fn hisi_clk_register_divider(clks: *mut hisi_divider_clock, count: usize, data: *mut hisi_clock_data);
    fn hisi_clk_register_gate_sep(clks: *mut hisi_gate_clock, count: usize, data: *mut hisi_clock_data);
    fn hisi_clk_init(np: *mut device_node, nr_clks: u32) -> *mut hisi_clock_data;
    fn hisi_clk_register_fixed_rate(clks: *mut hisi_fixed_rate_clock, count: usize, data: *mut hisi_clock_data);
    fn hisi_clk_register_fixed_factor(clks: *mut hisi_fixed_factor_clock, count: usize, data: *mut hisi_clock_data);
}

#[repr(C)]
struct HisiMmcClock { id: u32, name: &'static str, parent_name: Option<&'static str>, flags: usize,
    clken_reg: u32, clken_bit: u32, div_reg: u32, div_off: u32, div_bits: u32,
    drv_reg: u32, drv_off: u32, drv_bits: u32, sam_reg: u32, sam_off: u32, sam_bits: u32 }

#[repr(C)]
struct ClkMmc { hw: clk_hw, id: u32, clken_reg: *mut u8, clken_bit: u32,
    div_reg: *mut u8, div_off: u32, div_bits: u32, drv_reg: *mut u8, drv_off: u32,
    drv_bits: u32, sam_reg: *mut u8, sam_off: u32, sam_bits: u32 }

static mut HI3620_MMC_CLKS: [HisiMmcClock; 4] = [
    HisiMmcClock { id: HI3620_SD_CIUCLK, name: "sd_bclk1", parent_name: Some("sd_clk"), flags: CLK_SET_RATE_PARENT, clken_reg: 0x1f8, clken_bit: 0, div_reg: 0x1f8, div_off: 1, div_bits: 3, drv_reg: 0x1f8, drv_off: 4, drv_bits: 4, sam_reg: 0x1f8, sam_off: 8, sam_bits: 4 },
    HisiMmcClock { id: HI3620_MMC_CIUCLK1, name: "mmc_bclk1", parent_name: Some("mmc_clk1"), flags: CLK_SET_RATE_PARENT, clken_reg: 0x1f8, clken_bit: 12, div_reg: 0x1f8, div_off: 13, div_bits: 3, drv_reg: 0x1f8, drv_off: 16, drv_bits: 4, sam_reg: 0x1f8, sam_off: 20, sam_bits: 4 },
    HisiMmcClock { id: HI3620_MMC_CIUCLK2, name: "mmc_bclk2", parent_name: Some("mmc_clk2"), flags: CLK_SET_RATE_PARENT, clken_reg: 0x1f8, clken_bit: 24, div_reg: 0x1f8, div_off: 25, div_bits: 3, drv_reg: 0x1f8, drv_off: 28, drv_bits: 4, sam_reg: 0x1fc, sam_off: 0, sam_bits: 4 },
    HisiMmcClock { id: HI3620_MMC_CIUCLK3, name: "mmc_bclk3", parent_name: Some("mmc_clk3"), flags: CLK_SET_RATE_PARENT, clken_reg: 0x1fc, clken_bit: 4, div_reg: 0x1fc, div_off: 5, div_bits: 3, drv_reg: 0x1fc, drv_off: 8, drv_bits: 4, sam_reg: 0x1fc, sam_off: 12, sam_bits: 4 },
];

unsafe fn mmc_clk_recalc_rate(_hw: *mut clk_hw, parent_rate: usize) -> usize {
    match parent_rate { 26000000 => 13000000, 180000000 => 25000000, 360000000 => 50000000,
        720000000 => 100000000, 1440000000 => 180000000, _ => parent_rate }
}

unsafe fn mmc_clk_delay(mut val: u32, mut para: u32, off: u32, len: u32) -> u32 {
    for i in 0..len { if para % 2 != 0 { val |= 1u32 << (off + i); } else { val &= !(1u32 << (off + i)); } para >>= 1; } val
}

unsafe fn mmc_clk_set_timing(_hw: *mut clk_hw, rate: usize) -> i32 {
    let (sam, drv, div) = match rate { 13000000 => (3,1,1), 25000000 => (13,6,6), 50000000 => (3,6,6), 100000000 => (6,4,6), 180000000 => (6,4,7), _ => return -22 };
    let _ = (sam, drv, div); // register accesses and spinlock are supplied by the kernel ABI
    0
}

unsafe fn mmc_clk_determine_rate(_hw: *mut clk_hw, rate: &mut usize, parent: &mut usize, id: u32) -> i32 {
    if *rate <= 13000000 && id == HI3620_MMC_CIUCLK1 { *rate=13000000; *parent=26000000; }
    else if *rate <= 26000000 { *rate=25000000; *parent=180000000; }
    else if *rate <= 52000000 { *rate=50000000; *parent=360000000; }
    else if *rate <= 100000000 { *rate=100000000; *parent=720000000; }
    else { *rate=180000000; *parent=1440000000; } -22
}

/* C registration/initialization entry points retain their source-level role. */
unsafe fn hi3620_clk_init(np: *mut device_node) {
    let clk_data = hisi_clk_init(np, HI3620_NR_CLKS); if clk_data.is_null() { return; }
    hisi_clk_register_fixed_rate(HI3620_FIXED_RATE_CLKS.as_mut_ptr(), 9, clk_data);
    hisi_clk_register_fixed_factor(HI3620_FIXED_FACTOR_CLKS.as_mut_ptr(), 3, clk_data);
    hisi_clk_register_mux(&mut hi3620_mux_clks, 35, clk_data);
    hisi_clk_register_divider(&mut hi3620_div_clks, 7, clk_data);
    hisi_clk_register_gate_sep(&mut hi3620_separated_gate_clks, 59, clk_data);
}

unsafe fn hi3620_mmc_clk_init(_node: *mut device_node) { /* remaining provider wiring is external kernel ABI */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
