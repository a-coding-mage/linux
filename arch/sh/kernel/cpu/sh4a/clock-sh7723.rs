// SPDX-License-Identifier: GPL-2.0
/* SH7723 clock framework support */

// External kernel clock-framework types, constants, and helpers are supplied
// by the surrounding translation unit.

pub const FRQCR: u32 = 0xa4150000;
pub const VCLKCR: u32 = 0xa4150004;
pub const SCLKACR: u32 = 0xa4150008;
pub const SCLKBCR: u32 = 0xa415000c;
pub const IRDACLKCR: u32 = 0xa4150018;
pub const PLLCR: u32 = 0xa4150024;
pub const MSTPCR0: u32 = 0xa4150030;
pub const MSTPCR1: u32 = 0xa4150034;
pub const MSTPCR2: u32 = 0xa4150038;
pub const DLLFRQ: u32 = 0xa4150050;

static mut r_clk: clk = clk { rate: 32768, ..clk::ZERO };
pub static mut extal_clk: clk = clk { rate: 33333333, ..clk::ZERO };

unsafe fn dll_recalc(clk: *mut clk) -> ulong {
    let mult = if __raw_readl(PLLCR) & 0x1000 != 0 { __raw_readl(DLLFRQ) as ulong } else { 0 };
    (*(*clk).parent).rate.wrapping_mul(mult)
}

static mut dll_clk_ops: sh_clk_ops = sh_clk_ops { recalc: Some(dll_recalc), ..sh_clk_ops::ZERO };
static mut dll_clk: clk = clk { ops: &mut dll_clk_ops, parent: &mut r_clk, flags: CLK_ENABLE_ON_INIT, ..clk::ZERO };

unsafe fn pll_recalc(clk: *mut clk) -> ulong {
    let mut mult: ulong = 1;
    let mut div: ulong = 1;
    if __raw_readl(PLLCR) & 0x4000 != 0 {
        mult = (((__raw_readl(FRQCR) >> 24) & 0x1f) + 1) as ulong;
    } else { div = 2; }
    (*(*clk).parent).rate.wrapping_mul(mult) / div
}

static mut pll_clk_ops: sh_clk_ops = sh_clk_ops { recalc: Some(pll_recalc), ..sh_clk_ops::ZERO };
static mut pll_clk: clk = clk { ops: &mut pll_clk_ops, flags: CLK_ENABLE_ON_INIT, ..clk::ZERO };

pub static mut main_clks: [*mut clk; 4] = [&mut r_clk, &mut extal_clk, &mut dll_clk, &mut pll_clk];

static mut multipliers: [i32; 13] = [1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1];
static mut divisors: [i32; 13] = [1, 3, 2, 5, 3, 4, 5, 6, 8, 10, 12, 16, 20];
static mut div4_div_mult_table: clk_div_mult_table = clk_div_mult_table {
    divisors: divisors.as_ptr(), nr_divisors: 13,
    multipliers: multipliers.as_ptr(), nr_multipliers: 13,
};
static mut div4_table: clk_div4_table = clk_div4_table { div_mult_table: &mut div4_div_mult_table };

pub const DIV4_I: usize = 0; pub const DIV4_U: usize = 1; pub const DIV4_SH: usize = 2;
pub const DIV4_B: usize = 3; pub const DIV4_B3: usize = 4; pub const DIV4_P: usize = 5;
pub const DIV4_NR: usize = 6;
pub const DIV4_IRDA: usize = 0; pub const DIV4_ENABLE_NR: usize = 1;
pub const DIV4_SIUA: usize = 0; pub const DIV4_SIUB: usize = 1; pub const DIV4_REPARENT_NR: usize = 2;
pub const DIV6_V: usize = 0; pub const DIV6_NR: usize = 1;

macro_rules! div4 { ($reg:expr, $bit:expr, $mask:expr, $flags:expr) => { SH_CLK_DIV4(&mut pll_clk, $reg, $bit, $mask, $flags) }; }
pub static mut div4_clks: [clk; DIV4_NR] = [
    div4!(FRQCR,20,0x0dbf,CLK_ENABLE_ON_INIT), div4!(FRQCR,16,0x0dbf,CLK_ENABLE_ON_INIT),
    div4!(FRQCR,12,0x0dbf,CLK_ENABLE_ON_INIT), div4!(FRQCR,8,0x0dbf,CLK_ENABLE_ON_INIT),
    div4!(FRQCR,4,0x0db4,CLK_ENABLE_ON_INIT), div4!(FRQCR,0,0x0dbf,0) ];
pub static mut div4_enable_clks: [clk; DIV4_ENABLE_NR] = [div4!(IRDACLKCR,0,0x0dbf,0)];
pub static mut div4_reparent_clks: [clk; DIV4_REPARENT_NR] = [div4!(SCLKACR,0,0x0dbf,0), div4!(SCLKBCR,0,0x0dbf,0)];
pub static mut div6_clks: [clk; DIV6_NR] = [SH_CLK_DIV6(&mut pll_clk, VCLKCR, 0)];

// The following clock-gate and lookup tables retain the original hardware
// block indices and framework constructors.
pub static mut mstp_clks: [clk; HWBLK_NR] = [
    SH_CLK_MSTP32(&mut div4_clks[DIV4_I],MSTPCR0,31,CLK_ENABLE_ON_INIT), SH_CLK_MSTP32(&mut div4_clks[DIV4_I],MSTPCR0,30,CLK_ENABLE_ON_INIT),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_I],MSTPCR0,29,CLK_ENABLE_ON_INIT), SH_CLK_MSTP32(&mut div4_clks[DIV4_SH],MSTPCR0,28,CLK_ENABLE_ON_INIT),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_I],MSTPCR0,27,CLK_ENABLE_ON_INIT), SH_CLK_MSTP32(&mut div4_clks[DIV4_I],MSTPCR0,24,CLK_ENABLE_ON_INIT),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_I],MSTPCR0,22,CLK_ENABLE_ON_INIT), SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR0,21,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_SH],MSTPCR0,20,CLK_ENABLE_ON_INIT), SH_CLK_MSTP32(&mut div4_clks[DIV4_P],MSTPCR0,19,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_I],MSTPCR0,17,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_P],MSTPCR0,15,0),
    SH_CLK_MSTP32(&mut r_clk,MSTPCR0,14,0), SH_CLK_MSTP32(&mut r_clk,MSTPCR0,13,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR0,12,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_P],MSTPCR0,11,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_P],MSTPCR0,10,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_P],MSTPCR0,9,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_P],MSTPCR0,8,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_P],MSTPCR0,7,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR0,6,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR0,5,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR0,4,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR0,2,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR0,1,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_SH],MSTPCR0,0,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_P],MSTPCR1,9,0), SH_CLK_MSTP32(&mut r_clk,MSTPCR1,8,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_SH],MSTPCR2,28,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_P],MSTPCR2,27,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,25,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_P],MSTPCR2,24,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,22,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,21,CLK_ENABLE_ON_INIT),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,18,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,17,0),
    SH_CLK_MSTP32(&mut r_clk,MSTPCR2,14,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,11,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,10,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,8,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,6,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,5,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,4,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,3,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,2,0), SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,1,0),
    SH_CLK_MSTP32(&mut div4_clks[DIV4_B],MSTPCR2,0,0) ];

pub static mut lookups: [clk_lookup; 50] = [
    CLKDEV_CON_ID("rclk", &mut r_clk), CLKDEV_CON_ID("extal", &mut extal_clk),
    CLKDEV_CON_ID("dll_clk", &mut dll_clk), CLKDEV_CON_ID("pll_clk", &mut pll_clk),
    CLKDEV_CON_ID("cpu_clk", &mut div4_clks[DIV4_I]), CLKDEV_CON_ID("umem_clk", &mut div4_clks[DIV4_U]),
    CLKDEV_CON_ID("shyway_clk", &mut div4_clks[DIV4_SH]), CLKDEV_CON_ID("bus_clk", &mut div4_clks[DIV4_B]),
    CLKDEV_CON_ID("b3_clk", &mut div4_clks[DIV4_B3]), CLKDEV_CON_ID("peripheral_clk", &mut div4_clks[DIV4_P]),
    CLKDEV_CON_ID("irda_clk", &mut div4_enable_clks[DIV4_IRDA]), CLKDEV_CON_ID("siua_clk", &mut div4_reparent_clks[DIV4_SIUA]),
    CLKDEV_CON_ID("siub_clk", &mut div4_reparent_clks[DIV4_SIUB]), CLKDEV_CON_ID("video_clk", &mut div6_clks[DIV6_V]),
    CLKDEV_CON_ID("tlb0", &mut mstp_clks[HWBLK_TLB]), CLKDEV_CON_ID("ic0", &mut mstp_clks[HWBLK_IC]),
    CLKDEV_CON_ID("oc0", &mut mstp_clks[HWBLK_OC]), CLKDEV_CON_ID("l2c0", &mut mstp_clks[HWBLK_L2C]),
    CLKDEV_CON_ID("ilmem0", &mut mstp_clks[HWBLK_ILMEM]), CLKDEV_CON_ID("fpu0", &mut mstp_clks[HWBLK_FPU]),
    CLKDEV_CON_ID("intc0", &mut mstp_clks[HWBLK_INTC]), CLKDEV_DEV_ID("sh-dma-engine.0", &mut mstp_clks[HWBLK_DMAC0]),
    CLKDEV_CON_ID("sh0", &mut mstp_clks[HWBLK_SHYWAY]), CLKDEV_CON_ID("hudi0", &mut mstp_clks[HWBLK_HUDI]),
    CLKDEV_CON_ID("ubc0", &mut mstp_clks[HWBLK_UBC]), CLKDEV_ICK_ID("fck", "sh-cmt-32.0", &mut mstp_clks[HWBLK_CMT]),
    CLKDEV_DEV_ID("sh-wdt.0", &mut mstp_clks[HWBLK_RWDT]), CLKDEV_DEV_ID("sh-dma-engine.1", &mut mstp_clks[HWBLK_DMAC1]),
    CLKDEV_CON_ID("flctl0", &mut mstp_clks[HWBLK_FLCTL]), CLKDEV_DEV_ID("spi_sh_msiof.0", &mut mstp_clks[HWBLK_MSIOF0]),
    CLKDEV_DEV_ID("spi_sh_msiof.1", &mut mstp_clks[HWBLK_MSIOF1]), CLKDEV_DEV_ID("sh_mobile_meram.0", &mut mstp_clks[HWBLK_MERAM]),
    CLKDEV_DEV_ID("i2c-sh_mobile.0", &mut mstp_clks[HWBLK_IIC]), CLKDEV_CON_ID("rtc0", &mut mstp_clks[HWBLK_RTC]),
    CLKDEV_CON_ID("atapi0", &mut mstp_clks[HWBLK_ATAPI]), CLKDEV_CON_ID("adc0", &mut mstp_clks[HWBLK_ADC]),
    CLKDEV_CON_ID("tpu0", &mut mstp_clks[HWBLK_TPU]), CLKDEV_CON_ID("irda0", &mut mstp_clks[HWBLK_IRDA]),
    CLKDEV_CON_ID("tsif0", &mut mstp_clks[HWBLK_TSIF]), CLKDEV_CON_ID("icb0", &mut mstp_clks[HWBLK_ICB]),
    CLKDEV_DEV_ID("sh_mobile_sdhi.0", &mut mstp_clks[HWBLK_SDHI0]), CLKDEV_DEV_ID("sh_mobile_sdhi.1", &mut mstp_clks[HWBLK_SDHI1]),
    CLKDEV_DEV_ID("sh_keysc.0", &mut mstp_clks[HWBLK_KEYSC]), CLKDEV_CON_ID("usb0", &mut mstp_clks[HWBLK_USB]),
    CLKDEV_CON_ID("2dg0", &mut mstp_clks[HWBLK_2DG]), CLKDEV_DEV_ID("siu-pcm-audio", &mut mstp_clks[HWBLK_SIU]),
    CLKDEV_CON_ID("veu1", &mut mstp_clks[HWBLK_VEU2H1]), CLKDEV_DEV_ID("sh-vou.0", &mut mstp_clks[HWBLK_VOU]),
    CLKDEV_CON_ID("beu0", &mut mstp_clks[HWBLK_BEU]), CLKDEV_DEV_ID("ceu.0", &mut mstp_clks[HWBLK_CEU]),
    CLKDEV_CON_ID("veu0", &mut mstp_clks[HWBLK_VEU2H0]), CLKDEV_CON_ID("vpu0", &mut mstp_clks[HWBLK_VPU]),
    CLKDEV_ICK_ID("fck", "sh-tmu.0", &mut mstp_clks[HWBLK_TMU0]), CLKDEV_ICK_ID("fck", "sh-tmu.1", &mut mstp_clks[HWBLK_TMU1]),
    CLKDEV_DEV_ID("sh_mobile_lcdc_fb.0", &mut mstp_clks[HWBLK_LCDC]) ];

pub unsafe fn arch_clk_init() -> i32 {
    if __raw_readl(PLLCR) & 0x1000 != 0 { pll_clk.parent = &mut dll_clk; } else { pll_clk.parent = &mut extal_clk; }
    let mut k = 0; let mut ret = 0;
    while ret == 0 && k < 4 { ret |= clk_register(main_clks[k]); k += 1; }
    clkdev_add_table(lookups.as_mut_ptr(), lookups.len() as i32);
    if ret == 0 { ret = sh_clk_div4_register(div4_clks.as_mut_ptr(), DIV4_NR as i32, &mut div4_table); }
    if ret == 0 { ret = sh_clk_div4_enable_register(div4_enable_clks.as_mut_ptr(), DIV4_ENABLE_NR as i32, &mut div4_table); }
    if ret == 0 { ret = sh_clk_div4_reparent_register(div4_reparent_clks.as_mut_ptr(), DIV4_REPARENT_NR as i32, &mut div4_table); }
    if ret == 0 { ret = sh_clk_div6_register(div6_clks.as_mut_ptr(), DIV6_NR as i32); }
    if ret == 0 { ret = sh_clk_mstp_register(mstp_clks.as_mut_ptr(), HWBLK_NR as i32); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
