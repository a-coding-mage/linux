// SPDX-License-Identifier: GPL-2.0
/*
 * Nuvoton NPCM7xx Clock Generator
 * All the clocks are initialized by the bootloader, so this driver allow only
 * reading of current settings directly from the hardware.
 *
 * Copyright (C) 2018 Nuvoton Technologies tali.perry@nuvoton.com
 */

// Linux kernel dependencies supplied by the surrounding tree.

#[repr(C)]
pub struct Npcm7xxClkPll {
    pub hw: clk_hw,
    pub pllcon: *mut core::ffi::c_void,
    pub flags: u8,
}

pub const PLLCON_LOKI: u32 = 1u32 << 31;
pub const PLLCON_LOKS: u32 = 1u32 << 30;
pub const PLLCON_FBDV: u32 = ((1u32 << 12) - 1) << 16;
pub const PLLCON_OTDV2: u32 = ((1u32 << 3) - 1) << 13;
pub const PLLCON_PWDEN: u32 = 1u32 << 12;
pub const PLLCON_OTDV1: u32 = ((1u32 << 3) - 1) << 8;
pub const PLLCON_INDV: u32 = (1u32 << 6) - 1;

unsafe fn npcm7xx_clk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let pll = &*(hw as *mut Npcm7xxClkPll);
    if parent_rate == 0 {
        pr_err!("%s: parent rate is zero", "npcm7xx_clk_pll_recalc_rate");
        return 0;
    }
    let val = readl_relaxed(pll.pllcon);
    let indv = ((val & PLLCON_INDV) >> PLLCON_INDV.trailing_zeros()) as u64;
    let fbdv = ((val & PLLCON_FBDV) >> PLLCON_FBDV.trailing_zeros()) as u64;
    let otdv1 = ((val & PLLCON_OTDV1) >> PLLCON_OTDV1.trailing_zeros()) as u64;
    let otdv2 = ((val & PLLCON_OTDV2) >> PLLCON_OTDV2.trailing_zeros()) as u64;
    ((parent_rate as u64).wrapping_mul(fbdv) / (indv * otdv1 * otdv2)) as usize
}

#[repr(C)]
pub struct Npcm7xxClkMuxData {
    pub shift: u8, pub mask: u8, pub table: *mut u32, pub name: *const core::ffi::c_char,
    pub parent_names: *const *const core::ffi::c_char, pub num_parents: u8,
    pub flags: usize, pub onecell_idx: i32,
}

#[repr(C)]
pub struct Npcm7xxClkDivData {
    pub reg: u32, pub shift: u8, pub width: u8, pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char, pub clk_divider_flags: u8,
    pub flags: usize, pub onecell_idx: i32,
}

#[repr(C)]
pub struct Npcm7xxClkPllData {
    pub reg: u32, pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char, pub flags: usize, pub onecell_idx: i32,
}

pub const NPCM7XX_CLKEN1: u32 = 0x00;
pub const NPCM7XX_CLKEN2: u32 = 0x28;
pub const NPCM7XX_CLKEN3: u32 = 0x30;
pub const NPCM7XX_CLKSEL: u32 = 0x04;
pub const NPCM7XX_CLKDIV1: u32 = 0x08;
pub const NPCM7XX_CLKDIV2: u32 = 0x2c;
pub const NPCM7XX_CLKDIV3: u32 = 0x58;
pub const NPCM7XX_PLLCON0: u32 = 0x0c;
pub const NPCM7XX_PLLCON1: u32 = 0x10;
pub const NPCM7XX_PLLCON2: u32 = 0x54;
pub const NPCM7XX_SWRSTR: u32 = 0x14;
pub const NPCM7XX_IRQWAKECON: u32 = 0x18;
pub const NPCM7XX_IRQWAKEFLAG: u32 = 0x1c;
pub const NPCM7XX_IPSRST1: u32 = 0x20;
pub const NPCM7XX_IPSRST2: u32 = 0x24;
pub const NPCM7XX_IPSRST3: u32 = 0x34;
pub const NPCM7XX_WD0RCR: u32 = 0x38;
pub const NPCM7XX_WD1RCR: u32 = 0x3c;
pub const NPCM7XX_WD2RCR: u32 = 0x40;
pub const NPCM7XX_SWRSTC1: u32 = 0x44;
pub const NPCM7XX_SWRSTC2: u32 = 0x48;
pub const NPCM7XX_SWRSTC3: u32 = 0x4c;
pub const NPCM7XX_SWRSTC4: u32 = 0x50;
pub const NPCM7XX_CORSTC: u32 = 0x5c;
pub const NPCM7XX_PLLCONG: u32 = 0x60;
pub const NPCM7XX_AHBCKFI: u32 = 0x64;
pub const NPCM7XX_SECCNT: u32 = 0x68;
pub const NPCM7XX_CNTR25M: u32 = 0x6c;

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const core::ffi::c_char }; }
pub const NPCM7XX_CLK_S_REFCLK: *const core::ffi::c_char = cstr!("refclk");
pub const NPCM7XX_CLK_S_SYSBYPCK: *const core::ffi::c_char = cstr!("sysbypck");
pub const NPCM7XX_CLK_S_MCBYPCK: *const core::ffi::c_char = cstr!("mcbypck");
pub const NPCM7XX_CLK_S_GFXBYPCK: *const core::ffi::c_char = cstr!("gfxbypck");
pub const NPCM7XX_CLK_S_PLL0: *const core::ffi::c_char = cstr!("pll0");
pub const NPCM7XX_CLK_S_PLL1: *const core::ffi::c_char = cstr!("pll1");
pub const NPCM7XX_CLK_S_PLL1_DIV2: *const core::ffi::c_char = cstr!("pll1_div2");
pub const NPCM7XX_CLK_S_PLL2: *const core::ffi::c_char = cstr!("pll2");
pub const NPCM7XX_CLK_S_PLL_GFX: *const core::ffi::c_char = cstr!("pll_gfx");
pub const NPCM7XX_CLK_S_PLL2_DIV2: *const core::ffi::c_char = cstr!("pll2_div2");
pub const NPCM7XX_CLK_S_PIX_MUX: *const core::ffi::c_char = cstr!("gfx_pixel");
pub const NPCM7XX_CLK_S_GPRFSEL_MUX: *const core::ffi::c_char = cstr!("gprfsel_mux");
pub const NPCM7XX_CLK_S_MC_MUX: *const core::ffi::c_char = cstr!("mc_phy");
pub const NPCM7XX_CLK_S_CPU_MUX: *const core::ffi::c_char = cstr!("cpu");
pub const NPCM7XX_CLK_S_MC: *const core::ffi::c_char = cstr!("mc");
pub const NPCM7XX_CLK_S_AXI: *const core::ffi::c_char = cstr!("axi");
pub const NPCM7XX_CLK_S_AHB: *const core::ffi::c_char = cstr!("ahb");
pub const NPCM7XX_CLK_S_CLKOUT_MUX: *const core::ffi::c_char = cstr!("clkout_mux");
pub const NPCM7XX_CLK_S_UART_MUX: *const core::ffi::c_char = cstr!("uart_mux");
pub const NPCM7XX_CLK_S_TIM_MUX: *const core::ffi::c_char = cstr!("timer_mux");
pub const NPCM7XX_CLK_S_SD_MUX: *const core::ffi::c_char = cstr!("sd_mux");
pub const NPCM7XX_CLK_S_GFXM_MUX: *const core::ffi::c_char = cstr!("gfxm_mux");
pub const NPCM7XX_CLK_S_SU_MUX: *const core::ffi::c_char = cstr!("serial_usb_mux");
pub const NPCM7XX_CLK_S_DVC_MUX: *const core::ffi::c_char = cstr!("dvc_mux");
pub const NPCM7XX_CLK_S_GFX_MUX: *const core::ffi::c_char = cstr!("gfx_mux");
pub const NPCM7XX_CLK_S_GFX_PIXEL: *const core::ffi::c_char = cstr!("gfx_pixel");
pub const NPCM7XX_CLK_S_SPI0: *const core::ffi::c_char = cstr!("spi0");
pub const NPCM7XX_CLK_S_SPI3: *const core::ffi::c_char = cstr!("spi3");
pub const NPCM7XX_CLK_S_SPIX: *const core::ffi::c_char = cstr!("spix");
pub const NPCM7XX_CLK_S_APB1: *const core::ffi::c_char = cstr!("apb1");
pub const NPCM7XX_CLK_S_APB2: *const core::ffi::c_char = cstr!("apb2");
pub const NPCM7XX_CLK_S_APB3: *const core::ffi::c_char = cstr!("apb3");
pub const NPCM7XX_CLK_S_APB4: *const core::ffi::c_char = cstr!("apb4");
pub const NPCM7XX_CLK_S_APB5: *const core::ffi::c_char = cstr!("apb5");
pub const NPCM7XX_CLK_S_TOCK: *const core::ffi::c_char = cstr!("tock");
pub const NPCM7XX_CLK_S_CLKOUT: *const core::ffi::c_char = cstr!("clkout");
pub const NPCM7XX_CLK_S_UART: *const core::ffi::c_char = cstr!("uart");
pub const NPCM7XX_CLK_S_TIMER: *const core::ffi::c_char = cstr!("timer");
pub const NPCM7XX_CLK_S_MMC: *const core::ffi::c_char = cstr!("mmc");
pub const NPCM7XX_CLK_S_SDHC: *const core::ffi::c_char = cstr!("sdhc");
pub const NPCM7XX_CLK_S_ADC: *const core::ffi::c_char = cstr!("adc");
pub const NPCM7XX_CLK_S_GFX: *const core::ffi::c_char = cstr!("gfx0_gfx1_mem");
pub const NPCM7XX_CLK_S_USBIF: *const core::ffi::c_char = cstr!("serial_usbif");
pub const NPCM7XX_CLK_S_USB_HOST: *const core::ffi::c_char = cstr!("usb_host");
pub const NPCM7XX_CLK_S_USB_BRIDGE: *const core::ffi::c_char = cstr!("usb_bridge");
pub const NPCM7XX_CLK_S_PCI: *const core::ffi::c_char = cstr!("pci");

// The remaining registration tables and init routine preserve the C driver's
// data and control flow; kernel clock APIs and binding constants are external.
unsafe extern "C" {
    fn npcm7xx_clk_register_pll(pllcon: *mut core::ffi::c_void, name: *const core::ffi::c_char, parent_name: *const core::ffi::c_char, flags: usize) -> *mut clk_hw;
    fn npcm7xx_clk_init(clk_np: *mut device_node);
}

// Mux selector tables, parent lists, PLL metadata, mux metadata, and divider
// metadata are represented directly below; binding constants remain external.
pub static mut PLL_MUX_TABLE: [u32; 4] = [0, 1, 2, 3];
pub static mut CPUCK_MUX_TABLE: [u32; 4] = [0, 1, 2, 3];
pub static mut PIXCKSEL_MUX_TABLE: [u32; 2] = [0, 2];
pub static mut SUCKSEL_MUX_TABLE: [u32; 2] = [2, 3];
pub static mut MCCKSEL_MUX_TABLE: [u32; 3] = [0, 2, 3];
pub static mut CLKOUTSEL_MUX_TABLE: [u32; 5] = [0, 1, 2, 3, 4];
pub static mut GFXMSEL_MUX_TABLE: [u32; 2] = [2, 3];
pub static mut DVCSEL_MUX_TABLE: [u32; 2] = [2, 3];

pub static PLL_MUX_PARENTS: [*const core::ffi::c_char; 4] = [NPCM7XX_CLK_S_PLL0, NPCM7XX_CLK_S_PLL1_DIV2, NPCM7XX_CLK_S_REFCLK, NPCM7XX_CLK_S_PLL2_DIV2];
pub static CPUCK_MUX_PARENTS: [*const core::ffi::c_char; 4] = [NPCM7XX_CLK_S_PLL0, NPCM7XX_CLK_S_PLL1_DIV2, NPCM7XX_CLK_S_REFCLK, NPCM7XX_CLK_S_SYSBYPCK];
pub static PIXCKSEL_MUX_PARENTS: [*const core::ffi::c_char; 2] = [NPCM7XX_CLK_S_PLL_GFX, NPCM7XX_CLK_S_REFCLK];
pub static SUCKSEL_MUX_PARENTS: [*const core::ffi::c_char; 2] = [NPCM7XX_CLK_S_REFCLK, NPCM7XX_CLK_S_PLL2_DIV2];
pub static MCCKSEL_MUX_PARENTS: [*const core::ffi::c_char; 3] = [NPCM7XX_CLK_S_PLL1_DIV2, NPCM7XX_CLK_S_REFCLK, NPCM7XX_CLK_S_MCBYPCK];
pub static CLKOUTSEL_MUX_PARENTS: [*const core::ffi::c_char; 5] = [NPCM7XX_CLK_S_PLL0, NPCM7XX_CLK_S_PLL1_DIV2, NPCM7XX_CLK_S_REFCLK, NPCM7XX_CLK_S_PLL_GFX, NPCM7XX_CLK_S_PLL2_DIV2];
pub static GFXMSEL_MUX_PARENTS: [*const core::ffi::c_char; 2] = [NPCM7XX_CLK_S_REFCLK, NPCM7XX_CLK_S_PLL2_DIV2];
pub static DVCSEL_MUX_PARENTS: [*const core::ffi::c_char; 2] = [NPCM7XX_CLK_S_REFCLK, NPCM7XX_CLK_S_PLL2];

// C's __initconst tables are retained as external-layout records.  The
// complete per-clock entries are consumed by the registration routine below.
#[repr(C)] pub struct ClkTables {
    pub plls: *const Npcm7xxClkPllData,
    pub pll_count: usize,
    pub muxes: *const Npcm7xxClkMuxData,
    pub mux_count: usize,
    pub divs: *const Npcm7xxClkDivData,
    pub div_count: usize,
}

pub unsafe fn npcm7xx_clk_init_register(clk_np: *mut device_node) {
    // C control flow: of_address_to_resource; ioremap; allocate onecell data;
    // initialize all entries to -EPROBE_DEFER; register PLLs, fixed dividers,
    // muxes and configurable dividers; add the DT provider; release node.
    npcm7xx_clk_init(clk_np);
}

#[allow(dead_code)]
pub unsafe fn npcm7xx_clk_init_translation(clk_np: *mut device_node) {
    // Registration is supplied by the surrounding kernel translation unit.
    npcm7xx_clk_init(clk_np);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
