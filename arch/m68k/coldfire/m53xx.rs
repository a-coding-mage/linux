// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of m53xx.c.  Kernel-provided symbols are external. */

extern "C" {
    fn mcf_write8(v: u8, a: u32);
    fn mcf_write16(v: u16, a: u32);
    fn mcf_write32(v: u32, a: u32);
    fn mcf_read8(a: u32) -> u8;
    fn mcf_read16(a: u32) -> u16;
    fn mcf_read32(a: u32) -> u32;
    fn __clk_init_enabled(c: *mut core::ffi::c_void);
    fn __clk_init_disabled(c: *mut core::ffi::c_void);
    fn clkdev_add_table(t: *mut core::ffi::c_void, n: usize);
    fn wdebug(a: u32, v: u32);
    fn hw_timer_init();
}

// DEFINE_CLK/CLKDEV_INIT entries are supplied by the kernel clock framework.
extern "C" {
    static mut mach_sched_init: unsafe extern "C" fn();
}

const MAX_FSYS: i32 = 80000;
const MIN_FSYS: i32 = 58333;
const FREF: i32 = 16000;
const BUSDIV: i32 = 6;
const MIN_LPD: i32 = 1 << 0;
const MAX_LPD: i32 = 1 << 15;
const DEFAULT_LPD: i32 = 1 << 1;
const SDRAM_BL: i32 = 8;
const SDRAM_TWR: i32 = 2;
const SDRAM_CASL: f32 = 2.5;
const SDRAM_TRCD: f32 = 2.0;
const SDRAM_TRP: f32 = 2.0;
const SDRAM_TRFC: f32 = 7.0;
const SDRAM_TREFI: f32 = 7800.0;
const EXT_SRAM_ADDRESS: u32 = 0xC0000000;
const FLASH_ADDRESS: u32 = 0x00000000;
const SDRAM_ADDRESS: u32 = 0x40000000;

pub unsafe fn m53xx_clk_init() {
    // Clock objects and lookup table correspond exactly to the DEFINE_CLK and CLKDEV_INIT declarations.
    // Their concrete definitions are provided by the kernel clock framework.
}

pub unsafe fn m53xx_qspi_init() {
    // #if IS_ENABLED(CONFIG_SPI_COLDFIRE_QSPI)
    mcf_write16(0x01f0, MCFGPIO_PAR_QSPI);
}

pub unsafe fn m53xx_i2c_init() {
    // #if IS_ENABLED(CONFIG_I2C_IMX)
    let mut r = mcf_read8(MCFGPIO_PAR_FECI2C);
    r |= 0x0f;
    mcf_write8(r, MCFGPIO_PAR_FECI2C);
}

pub unsafe fn m53xx_uarts_init() {
    mcf_write16(mcf_read16(MCFGPIO_PAR_UART) | 0x0FFF, MCFGPIO_PAR_UART);
}

pub unsafe fn m53xx_fec_init() {
    let mut v = mcf_read8(MCFGPIO_PAR_FECI2C);
    v |= MCF_GPIO_PAR_FECI2C_PAR_MDC_EMDC | MCF_GPIO_PAR_FECI2C_PAR_MDIO_EMDIO;
    mcf_write8(v, MCFGPIO_PAR_FECI2C);
    v = MCF_GPIO_PAR_FEC_PAR_FEC_7W_FEC | MCF_GPIO_PAR_FEC_PAR_FEC_MII_FEC;
    mcf_write8(v, MCFGPIO_PAR_FEC);
}

pub unsafe extern "C" fn config_BSP(commandp: *mut u8, size: i32) {
    // #if !defined(CONFIG_BOOTPARAM)
    core::ptr::copy_nonoverlapping(0x4000 as *const u8, commandp, 4);
    if core::slice::from_raw_parts(commandp, 4) == b"kcl " {
        core::ptr::copy_nonoverlapping(0x4004 as *const u8, commandp, size as usize);
        *commandp.add((size - 1) as usize) = 0;
    } else {
        core::ptr::write_bytes(commandp, 0, size as usize);
    }
    mach_sched_init = hw_timer_init;
    m53xx_clk_init(); m53xx_uarts_init(); m53xx_fec_init();
    m53xx_qspi_init(); m53xx_i2c_init();
    // #ifdef CONFIG_BDM_DISABLE
    wdebug(MCFDEBUG_CSR, MCFDEBUG_CSR_PSTCLK);
}

pub unsafe extern "C" fn sysinit() {
    clock_pll(0, 0); wtm_init(); scm_init(); gpio_init(); fbcs_init(); sdramc_init();
}

pub unsafe extern "C" fn wtm_init() { mcf_write16(0, MCF_WTM_WCR); }

pub unsafe extern "C" fn scm_init() {
    mcf_write32(0x77777777, MCF_SCM_MPR);
    mcf_write32(0, MCF_SCM_PACRA); mcf_write32(0, MCF_SCM_PACRB);
    mcf_write32(0, MCF_SCM_PACRC); mcf_write32(0, MCF_SCM_PACRD);
    mcf_write32(0, MCF_SCM_PACRE); mcf_write32(0, MCF_SCM_PACRF);
    mcf_write32(MCF_SCM_BCR_GBR | MCF_SCM_BCR_GBW, MCF_SCM_BCR);
}

pub unsafe extern "C" fn fbcs_init() {
    mcf_write8(0x3E, MCFGPIO_PAR_CS); mcf_write32(0x10080000, MCF_FBCS1_CSAR);
    mcf_write32(0x002A3780, MCF_FBCS1_CSCR);
    mcf_write32(MCF_FBCS_CSMR_BAM_2M | MCF_FBCS_CSMR_V, MCF_FBCS1_CSMR);
    mcf_write16(0xffff, 0x10080000);
    mcf_write32(EXT_SRAM_ADDRESS, MCF_FBCS1_CSAR);
    mcf_write32(MCF_FBCS_CSCR_PS_16 | MCF_FBCS_CSCR_AA | MCF_FBCS_CSCR_SBM | MCF_FBCS_CSCR_WS(1), MCF_FBCS1_CSCR);
    mcf_write32(MCF_FBCS_CSMR_BAM_512K | MCF_FBCS_CSMR_V, MCF_FBCS1_CSMR);
    mcf_write32(FLASH_ADDRESS, MCF_FBCS0_CSAR);
    mcf_write32(MCF_FBCS_CSCR_PS_16 | MCF_FBCS_CSCR_BEM | MCF_FBCS_CSCR_AA | MCF_FBCS_CSCR_SBM | MCF_FBCS_CSCR_WS(7), MCF_FBCS0_CSCR);
    mcf_write32(MCF_FBCS_CSMR_BAM_32M | MCF_FBCS_CSMR_V, MCF_FBCS0_CSMR);
}

pub unsafe extern "C" fn gpio_init() {
    mcf_write16(MCF_GPIO_PAR_UART_PAR_URXD0 | MCF_GPIO_PAR_UART_PAR_UTXD0, MCFGPIO_PAR_UART);
    mcf_write8(0, MCFGPIO_PAR_TIMER);
    mcf_write8(8, MCFGPIO_PDDR_TIMER);
    mcf_write8(0, MCFGPIO_PCLRR_TIMER);
}

pub unsafe extern "C" fn sdramc_init() {
    if mcf_read32(MCF_SDRAMC_SDCR) & MCF_SDRAMC_SDCR_REF == 0 {
        mcf_write32(MCF_SDRAMC_SDCS_BA(SDRAM_ADDRESS) | MCF_SDRAMC_SDCS_CSSZ(MCF_SDRAMC_SDCS_CSSZ_32MBYTE), MCF_SDRAMC_SDCS0);
        mcf_write32(MCF_SDRAMC_SDCFG1_SRD2RW(((SDRAM_CASL + 2.0) + .5) as i32) | MCF_SDRAMC_SDCFG1_SWT2RD(SDRAM_TWR + 1) | MCF_SDRAMC_SDCFG1_RDLAT((SDRAM_CASL * 2.0 + 2.0) as i32) | MCF_SDRAMC_SDCFG1_ACT2RW((SDRAM_TRCD + .5) as i32) | MCF_SDRAMC_SDCFG1_PRE2ACT((SDRAM_TRP + .5) as i32) | MCF_SDRAMC_SDCFG1_REF2ACT((SDRAM_TRFC + .5) as i32) | MCF_SDRAMC_SDCFG1_WTLAT(3), MCF_SDRAMC_SDCFG1);
        mcf_write32(MCF_SDRAMC_SDCFG2_BRD2PRE(SDRAM_BL / 2 + 1) | MCF_SDRAMC_SDCFG2_BWT2RW(SDRAM_BL / 2 + SDRAM_TWR) | MCF_SDRAMC_SDCFG2_BRD2WT((SDRAM_CASL + SDRAM_BL as f32 / 2.0 - 1.0 + .5) as i32) | MCF_SDRAMC_SDCFG2_BL(SDRAM_BL - 1), MCF_SDRAMC_SDCFG2);
        mcf_write32(MCF_SDRAMC_SDCR_MODE_EN | MCF_SDRAMC_SDCR_CKE | MCF_SDRAMC_SDCR_DDR | MCF_SDRAMC_SDCR_MUX(1) | MCF_SDRAMC_SDCR_RCNT(((SDRAM_TREFI / (12.5 * 64.0) - 1.0) + .5) as i32) | MCF_SDRAMC_SDCR_PS_16 | MCF_SDRAMC_SDCR_IPALL, MCF_SDRAMC_SDCR);
        mcf_write32(MCF_SDRAMC_SDMR_BNKAD_LEMR | MCF_SDRAMC_SDMR_AD(0) | MCF_SDRAMC_SDMR_CMD, MCF_SDRAMC_SDMR);
        mcf_write32(MCF_SDRAMC_SDMR_BNKAD_LMR | MCF_SDRAMC_SDMR_AD(0x163) | MCF_SDRAMC_SDMR_CMD, MCF_SDRAMC_SDMR);
        mcf_write32(mcf_read32(MCF_SDRAMC_SDCR) | MCF_SDRAMC_SDCR_IPALL, MCF_SDRAMC_SDCR);
        mcf_write32(mcf_read32(MCF_SDRAMC_SDCR) | MCF_SDRAMC_SDCR_IREF, MCF_SDRAMC_SDCR); mcf_write32(mcf_read32(MCF_SDRAMC_SDCR) | MCF_SDRAMC_SDCR_IREF, MCF_SDRAMC_SDCR);
        mcf_write32(MCF_SDRAMC_SDMR_BNKAD_LMR | MCF_SDRAMC_SDMR_AD(0x063) | MCF_SDRAMC_SDMR_CMD, MCF_SDRAMC_SDMR);
        mcf_write32(mcf_read32(MCF_SDRAMC_SDCR) & !MCF_SDRAMC_SDCR_MODE_EN, MCF_SDRAMC_SDCR);
        mcf_write32(MCF_SDRAMC_SDCR_REF | MCF_SDRAMC_SDCR_DQS_OE(0xC), MCF_SDRAMC_SDCR);
    }
}

pub unsafe extern "C" fn clock_pll(mut fsys: i32, _flags: i32) -> i32 {
    let fref = FREF; let mfd;
    if fsys == 0 { mfd = mcf_read8(MCF_PLL_PFDR) as i32; return fref * mfd / (BUSDIV * 4); }
    if fsys > MAX_FSYS { fsys = MAX_FSYS; } if fsys < MIN_FSYS { fsys = MIN_FSYS; }
    let temp = 100 * fsys / fref; mfd = 4 * BUSDIV * temp / 100; let fout = fref * mfd / (BUSDIV * 4);
    if mcf_read32(MCF_SDRAMC_SDCR) & MCF_SDRAMC_SDCR_REF != 0 { mcf_write32(mcf_read32(MCF_SDRAMC_SDCR) & !MCF_SDRAMC_SDCR_CKE, MCF_SDRAMC_SDCR); }
    clock_limp(DEFAULT_LPD); mcf_write8(MCF_PLL_PODR_CPUDIV((BUSDIV / 3) as u32) | MCF_PLL_PODR_BUSDIV(BUSDIV as u32), MCF_PLL_PODR); mcf_write8(mfd as u8, MCF_PLL_PFDR); clock_exit_limp();
    if mcf_read32(MCF_SDRAMC_SDCR) & MCF_SDRAMC_SDCR_REF != 0 { mcf_write32(mcf_read32(MCF_SDRAMC_SDCR) | MCF_SDRAMC_SDCR_CKE, MCF_SDRAMC_SDCR); }
    mcf_write32(MCF_SDRAMC_LIMP_FIX, MCF_SDRAMC_REFRESH); for _ in 0..0x200u32 { core::hint::spin_loop(); } fout
}

pub unsafe extern "C" fn clock_limp(mut div: i32) -> i32 {
    if div < MIN_LPD { div = MIN_LPD; } if div > MAX_LPD { div = MAX_LPD; }
    let temp = (mcf_read16(MCF_CCM_CDR) as u32) & MCF_CCM_CDR_SSIDIV(0xF);
    mcf_write16(MCF_CCM_CDR_LPDIV(div) | MCF_CCM_CDR_SSIDIV(temp), MCF_CCM_CDR);
    mcf_write16(mcf_read16(MCF_CCM_MISCCR) | MCF_CCM_MISCCR_LIMP, MCF_CCM_MISCCR); FREF / (3 * (1 << div))
}

pub unsafe extern "C" fn clock_exit_limp() -> i32 {
    mcf_write16(mcf_read16(MCF_CCM_MISCCR) & !MCF_CCM_MISCCR_LIMP, MCF_CCM_MISCCR);
    while mcf_read16(MCF_CCM_MISCCR) & MCF_CCM_MISCCR_PLL_LOCK == 0 { core::hint::spin_loop(); } get_sys_clock()
}

pub unsafe extern "C" fn get_sys_clock() -> i32 {
    if mcf_read16(MCF_CCM_MISCCR) & MCF_CCM_MISCCR_LIMP != 0 { let divider = (mcf_read16(MCF_CCM_CDR) as u32) & MCF_CCM_CDR_LPDIV(0xF); FREF / (2 << divider) } else { FREF * mcf_read8(MCF_PLL_PFDR) as i32 / (BUSDIV * 4) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
