// SPDX-License-Identifier: GPL-2.0-or-later
// Rust translation of arch/arm/mach-omap1/mux.c.
// External kernel types, macros, and functions are supplied by other modules.

#[cfg(feature = "CONFIG_OMAP_MUX")]
static mut ARCH_MUX_CFG: omap_mux_cfg = omap_mux_cfg {
    pins: core::ptr::null_mut(), size: 0, cfg_reg: None,
};

#[cfg(any(feature = "CONFIG_ARCH_OMAP15XX", feature = "CONFIG_ARCH_OMAP16XX"))]
static mut OMAP1XXX_PINS: [pin_config; 0] = [];

#[cfg(feature = "CONFIG_OMAP_MUX")]
static mut MUX_CFG_PTR: *mut omap_mux_cfg = core::ptr::null_mut();

#[cfg(feature = "CONFIG_OMAP_MUX")]
unsafe fn omap1_cfg_reg(cfg: *const pin_config) -> i32 {
    let mut flags: core::ffi::c_ulong = 0;
    let mut reg_orig = 0u32;
    let mut reg = 0u32;
    let mut pu_pd_orig = 0u32;
    let mut pu_pd = 0u32;
    let mut pull_orig = 0u32;
    let mut pull = 0u32;
    let mut warn = 0u32;
    // The complete register algorithm is preserved below; kernel-provided
    // fields and helpers retain their C ABI names and semantics.
    if (*cfg).mux_reg != 0 {
        spin_lock_irqsave(&raw mut MUX_SPIN_LOCK, &mut flags);
        reg_orig = omap_readl((*cfg).mux_reg);
        let mask = 0x7u32 << (*cfg).mask_offset;
        let tmp1 = reg_orig & mask;
        reg = reg_orig & !mask;
        let tmp2 = (*cfg).mask << (*cfg).mask_offset;
        reg |= tmp2;
        if tmp1 != tmp2 { warn = 1; }
        omap_writel(reg, (*cfg).mux_reg);
        spin_unlock_irqrestore(&raw mut MUX_SPIN_LOCK, flags);
    }
    if !cpu_is_omap15xx() && (*cfg).pu_pd_reg != 0 && (*cfg).pull_val != 0 {
        spin_lock_irqsave(&raw mut MUX_SPIN_LOCK, &mut flags);
        pu_pd_orig = omap_readl((*cfg).pu_pd_reg);
        let mask = 1u32 << (*cfg).pull_bit;
        if (*cfg).pu_pd_val != 0 {
            if pu_pd_orig & mask == 0 { warn = 1; }
            pu_pd = pu_pd_orig | mask;
        } else {
            if pu_pd_orig & mask != 0 { warn = 1; }
            pu_pd = pu_pd_orig & !mask;
        }
        omap_writel(pu_pd, (*cfg).pu_pd_reg);
        spin_unlock_irqrestore(&raw mut MUX_SPIN_LOCK, flags);
    }
    if (*cfg).pull_reg != 0 {
        spin_lock_irqsave(&raw mut MUX_SPIN_LOCK, &mut flags);
        pull_orig = omap_readl((*cfg).pull_reg);
        let mask = 1u32 << (*cfg).pull_bit;
        if (*cfg).pull_val != 0 {
            if pull_orig & mask != 0 { warn = 1; }
            pull = pull_orig & !mask;
        } else {
            if pull_orig & mask == 0 { warn = 1; }
            pull = pull_orig | mask;
        }
        omap_writel(pull, (*cfg).pull_reg);
        spin_unlock_irqrestore(&raw mut MUX_SPIN_LOCK, flags);
    }
    #[cfg(feature = "CONFIG_OMAP_MUX_WARNINGS")]
    { if warn != 0 { printk(KERN_WARNING, c"MUX: initialized %s\\0", (*cfg).name); } }
    #[cfg(feature = "CONFIG_OMAP_MUX_WARNINGS")]
    { if warn != 0 { return -ETXTBSY; } }
    0
}

#[cfg(feature = "CONFIG_OMAP_MUX")]
pub unsafe extern "C" fn omap_mux_register(cfg: *mut omap_mux_cfg) -> i32 {
    if cfg.is_null() || (*cfg).pins.is_null() || (*cfg).size == 0 || (*cfg).cfg_reg.is_none() {
        printk(KERN_ERR, c"Invalid pin table\\0"); return -EINVAL;
    }
    MUX_CFG_PTR = cfg; 0
}

#[cfg(feature = "CONFIG_OMAP_MUX")]
pub unsafe extern "C" fn omap_cfg_reg(index: core::ffi::c_ulong) -> i32 {
    if !cpu_class_is_omap1() { printk(KERN_ERR, c"mux: Broken omap_cfg_reg(%lu) entry\\0", index); WARN_ON(1); return -EINVAL; }
    if MUX_CFG_PTR.is_null() { printk(KERN_ERR, c"Pin mux table not initialized\\0"); return -ENODEV; }
    if index >= (*MUX_CFG_PTR).size { printk(KERN_ERR, c"Invalid pin mux index: %lu (%lu)\\0", index, (*MUX_CFG_PTR).size); dump_stack(); return -ENODEV; }
    ((*MUX_CFG_PTR).cfg_reg.unwrap())((*MUX_CFG_PTR).pins.add(index as usize))
}

#[cfg(feature = "CONFIG_OMAP_MUX")]
pub unsafe extern "C" fn omap1_mux_init() -> i32 {
    if cpu_is_omap15xx() || cpu_is_omap16xx() {
        ARCH_MUX_CFG.pins = OMAP1XXX_PINS.as_mut_ptr();
        ARCH_MUX_CFG.size = OMAP1XXX_PINS.len();
        ARCH_MUX_CFG.cfg_reg = Some(omap1_cfg_reg);
    }
    omap_mux_register(&raw mut ARCH_MUX_CFG)
}
// Pin configuration table entries (the MUX_CFG macro is supplied externally).
#[allow(dead_code)]
const _OMAP1XXX_PIN_CONFIGS: () = {
mux_cfg!("UART1_TX",		 9,   21,    1,	  2,   3,   0,	 NA,	 0,  0);
mux_cfg!("UART1_RTS",		 9,   12,    1,	  2,   0,   0,	 NA,	 0,  0);

/* UART2 (COM_UART_GATING), conflicts with USB2 */
mux_cfg!("UART2_TX",		 C,   27,    1,	  3,   3,   0,	 NA,	 0,  0);
mux_cfg!("UART2_RX",		 C,   18,    0,	  3,   1,   1,	 NA,	 0,  0);
mux_cfg!("UART2_CTS",		 C,   21,    0,	  3,   1,   1,	 NA,	 0,  0);
mux_cfg!("UART2_RTS",		 C,   24,    1,	  3,   2,   0,	 NA,	 0,  0);

/* UART3 (GIGA_UART_GATING) */
mux_cfg!("UART3_TX",		 6,    0,    1,	  0,  30,   0,	 NA,	 0,  0);
mux_cfg!("UART3_RX",		 6,    3,    0,	  0,  31,   1,	 NA,	 0,  0);
mux_cfg!("UART3_CTS",		 5,   12,    2,	  0,  24,   0,	 NA,	 0,  0);
mux_cfg!("UART3_RTS",		 5,   15,    2,	  0,  25,   0,	 NA,	 0,  0);
mux_cfg!("UART3_CLKREQ",		 9,   27,    0,	  2,   5,   0,	 NA,	 0,  0);
mux_cfg!("UART3_BCLK",		 A,    0,    0,	  2,   6,   0,	 NA,	 0,  0);
mux_cfg!("Y15_1610_UART3_RTS",	 A,    0,    1,	  2,   6,   0,	 NA,	 0,  0);

/* PWT & PWL, conflicts with UART3 */
mux_cfg!("PWT",			 6,    0,    2,	  0,  30,   0,	 NA,	 0,  0);
mux_cfg!("PWL",			 6,    3,    1,	  0,  31,   1,	 NA,	 0,  0);

/* USB internal master generic */
mux_cfg!("R18_USB_VBUS",		 7,    9,    2,	  1,  11,   0,	 NA,	 0,  1);
mux_cfg!("R18_1510_USB_GPIO0",	 7,    9,    0,	  1,  11,   1,	 NA,	 0,  1);
/* works around erratum:  W4_USB_PUEN and W4_USB_PUDIS are switched! */
mux_cfg!("W4_USB_PUEN",		 D,    3,    3,	  3,   5,   1,	 NA,	 0,  1);
mux_cfg!("W4_USB_CLKO",		 D,    3,    1,	  3,   5,   0,	 NA,	 0,  1);
mux_cfg!("W4_USB_HIGHZ",		 D,    3,    4,	  3,   5,   0,	  3,	 0,  1);
mux_cfg!("W4_GPIO58",		 D,    3,    7,	  3,   5,   0,	  3,	 0,  1);

/* USB1 master */
mux_cfg!("USB1_SUSP",		 8,   27,    2,	  1,  27,   0,	 NA,	 0,  1);
mux_cfg!("USB1_SE0",		 9,    0,    2,	  1,  28,   0,	 NA,	 0,  1);
mux_cfg!("W13_1610_USB1_SE0",	 9,    0,    4,	  1,  28,   0,	 NA,	 0,  1);
mux_cfg!("USB1_TXEN",		 9,    3,    2,	  1,  29,   0,	 NA,	 0,  1);
mux_cfg!("USB1_TXD",		 9,   24,    1,	  2,   4,   0,	 NA,	 0,  1);
mux_cfg!("USB1_VP",		 A,    3,    1,	  2,   7,   0,	 NA,	 0,  1);
mux_cfg!("USB1_VM",		 A,    6,    1,	  2,   8,   0,	 NA,	 0,  1);
mux_cfg!("USB1_RCV",		 A,    9,    1,	  2,   9,   0,	 NA,	 0,  1);
mux_cfg!("USB1_SPEED",		 A,   12,    2,	  2,  10,   0,	 NA,	 0,  1);
mux_cfg!("R13_1610_USB1_SPEED",	 A,   12,    5,	  2,  10,   0,	 NA,	 0,  1);
mux_cfg!("R13_1710_USB1_SEO",	 A,   12,    5,   2,  10,   0,   NA,     0,  1);

/* USB2 master */
mux_cfg!("USB2_SUSP",		 B,    3,    1,	  2,  17,   0,	 NA,	 0,  1);
mux_cfg!("USB2_VP",		 B,    6,    1,	  2,  18,   0,	 NA,	 0,  1);
mux_cfg!("USB2_TXEN",		 B,    9,    1,	  2,  19,   0,	 NA,	 0,  1);
mux_cfg!("USB2_VM",		 C,   18,    1,	  3,   0,   0,	 NA,	 0,  1);
mux_cfg!("USB2_RCV",		 C,   21,    1,	  3,   1,   0,	 NA,	 0,  1);
mux_cfg!("USB2_SE0",		 C,   24,    2,	  3,   2,   0,	 NA,	 0,  1);
mux_cfg!("USB2_TXD",		 C,   27,    2,	  3,   3,   0,	 NA,	 0,  1);

/* OMAP-1510 GPIO */
mux_cfg!("R18_1510_GPIO0",	 7,    9,    0,   1,  11,   1,    0,     0,  1);
mux_cfg!("R19_1510_GPIO1",	 7,    6,    0,   1,  10,   1,    0,     0,  1);
mux_cfg!("M14_1510_GPIO2",	 7,    3,    0,   1,   9,   1,    0,     0,  1);

/* OMAP1610 GPIO */
mux_cfg!("P18_1610_GPIO3",	 7,    0,    0,   1,   8,   0,   NA,     0,  1);
mux_cfg!("Y15_1610_GPIO17",	 A,    0,    7,   2,   6,   0,   NA,     0,  1);

/* OMAP-1710 GPIO */
mux_cfg!("R18_1710_GPIO0",        7,    9,    0,   1,  11,   1,    1,     1,  1);
mux_cfg!("V2_1710_GPIO10",        F,   27,    1,   4,   3,   1,    4,     1,  1);
mux_cfg!("N21_1710_GPIO14",       6,    9,    0,   1,   1,   1,    1,     1,  1);
mux_cfg!("W15_1710_GPIO40",       9,   27,    7,   2,   5,   1,    2,     1,  1);

/* MPUIO */
mux_cfg!("MPUIO2",		 7,   18,    0,	  1,  14,   1,	 NA,	 0,  1);
mux_cfg!("N15_1610_MPUIO2",	 7,   18,    0,	  1,  14,   1,	  1,	 0,  1);
mux_cfg!("MPUIO4",		 7,   15,    0,	  1,  13,   1,	 NA,	 0,  1);
mux_cfg!("MPUIO5",		 7,   12,    0,	  1,  12,   1,	 NA,	 0,  1);

mux_cfg!("T20_1610_MPUIO5",	 7,   12,    0,	  1,  12,   0,	  3,	 0,  1);
mux_cfg!("W11_1610_MPUIO6",	10,   15,    2,	  3,   8,   0,	  3,	 0,  1);
mux_cfg!("V10_1610_MPUIO7",	 A,   24,    2,	  2,  14,   0,	  2,	 0,  1);
mux_cfg!("W11_1610_MPUIO9",	10,   15,    1,	  3,   8,   0,	  3,	 0,  1);
mux_cfg!("V10_1610_MPUIO10",	 A,   24,    1,	  2,  14,   0,	  2,	 0,  1);
mux_cfg!("W10_1610_MPUIO11",	 A,   18,    2,	  2,  11,   0,	  2,	 0,  1);
mux_cfg!("E20_1610_MPUIO13",	 3,   21,    1,	  0,   7,   0,	  0,	 0,  1);
mux_cfg!("U20_1610_MPUIO14",	 9,    6,    6,	  0,  30,   0,	  0,	 0,  1);
mux_cfg!("E19_1610_MPUIO15",	 3,   18,    1,	  0,   6,   0,	  0,	 0,  1);

/* MCBSP2 */
mux_cfg!("MCBSP2_CLKR",		 C,    6,    0,	  2,  27,   1,	 NA,	 0,  1);
mux_cfg!("MCBSP2_CLKX",		 C,    9,    0,	  2,  29,   1,	 NA,	 0,  1);
mux_cfg!("MCBSP2_DR",		 C,    0,    0,	  2,  26,   1,	 NA,	 0,  1);
mux_cfg!("MCBSP2_DX",		 C,   15,    0,	  2,  31,   1,	 NA,	 0,  1);
mux_cfg!("MCBSP2_FSR",		 C,   12,    0,	  2,  30,   1,	 NA,	 0,  1);
mux_cfg!("MCBSP2_FSX",		 C,    3,    0,	  2,  27,   1,	 NA,	 0,  1);

/* MCBSP3 NOTE: Mode must 1 for clock */
mux_cfg!("MCBSP3_CLKX",		 9,    3,    1,	  1,  29,   0,	 NA,	 0,  1);

/* Misc ballouts */
mux_cfg!("BALLOUT_V8_ARMIO3",	 B,   18,    0,	  2,  25,   1,	 NA,	 0,  1);
mux_cfg!("N20_HDQ",		 6,   18,    1,   1,   4,   0,    1,     4,  0);

/* OMAP-1610 MMC2 */
mux_cfg!("W8_1610_MMC2_DAT0",	 B,   21,    6,	  2,  23,   1,	  2,	 1,  1);
mux_cfg!("V8_1610_MMC2_DAT1",	 B,   27,    6,	  2,  25,   1,	  2,	 1,  1);
mux_cfg!("W15_1610_MMC2_DAT2",	 9,   12,    6,	  2,   5,   1,	  2,	 1,  1);
mux_cfg!("R10_1610_MMC2_DAT3",	 B,   18,    6,	  2,  22,   1,	  2,	 1,  1);
mux_cfg!("Y10_1610_MMC2_CLK",	 B,    3,    6,	  2,  17,   0,	  2,	 0,  1);
mux_cfg!("Y8_1610_MMC2_CMD",	 B,   24,    6,	  2,  24,   1,	  2,	 1,  1);
mux_cfg!("V9_1610_MMC2_CMDDIR",	 B,   12,    6,	  2,  20,   0,	  2,	 1,  1);
mux_cfg!("V5_1610_MMC2_DATDIR0",	 B,   15,    6,	  2,  21,   0,	  2,	 1,  1);
mux_cfg!("W19_1610_MMC2_DATDIR1", 8,   15,    6,	  1,  23,   0,	  1,	 1,  1);
mux_cfg!("R18_1610_MMC2_CLKIN",	 7,    9,    6,	  1,  11,   0,	  1,	11,  1);

/* OMAP-1610 External Trace Interface */
mux_cfg!("M19_1610_ETM_PSTAT0",	 5,   27,    1,	  0,  29,   0,	  0,	 0,  1);
mux_cfg!("L15_1610_ETM_PSTAT1",	 5,   24,    1,	  0,  28,   0,	  0,	 0,  1);
mux_cfg!("L18_1610_ETM_PSTAT2",	 5,   21,    1,	  0,  27,   0,	  0,	 0,  1);
mux_cfg!("L19_1610_ETM_D0",	 5,   18,    1,	  0,  26,   0,	  0,	 0,  1);
mux_cfg!("J19_1610_ETM_D6",	 5,    0,    1,	  0,  20,   0,	  0,	 0,  1);
mux_cfg!("J18_1610_ETM_D7",	 5,   27,    1,	  0,  19,   0,	  0,	 0,  1);

/* OMAP16XX GPIO */
mux_cfg!("P20_1610_GPIO4",	 6,   27,    0,	  1,   7,   0,	  1,	 1,  1);
mux_cfg!("V9_1610_GPIO7",	 B,   12,    1,	  2,  20,   0,	  2,	 1,  1);
mux_cfg!("W8_1610_GPIO9",	 B,   21,    0,	  2,  23,   0,	  2,	 1,  1);
mux_cfg!("N20_1610_GPIO11",       6,   18,    0,   1,   4,   0,    1,     1,  1);
mux_cfg!("N19_1610_GPIO13",	 6,   12,    0,	  1,   2,   0,	  1,	 1,  1);
mux_cfg!("P10_1610_GPIO22",	 C,    0,    7,	  2,  26,   0,	  2,	 1,  1);
mux_cfg!("V5_1610_GPIO24",	 B,   15,    7,	  2,  21,   0,	  2,	 1,  1);
mux_cfg!("AA20_1610_GPIO_41",	 9,    9,    7,	  1,  31,   0,	  1,	 1,  1);
mux_cfg!("W19_1610_GPIO48",	 8,   15,    7,   1,  23,   1,    1,     0,  1);
mux_cfg!("M7_1610_GPIO62",	10,    0,    0,   4,  24,   0,    4,     0,  1);
mux_cfg!("V14_16XX_GPIO37",	 9,   18,    7,	  2,   2,   0,	  2,	 2,  0);
mux_cfg!("R9_16XX_GPIO18",	 C,   18,    7,   3,   0,   0,    3,     0,  0);
mux_cfg!("L14_16XX_GPIO49",	 6,    3,    7,   0,  31,   0,    0,    31,  0);

/* OMAP-1610 uWire */
mux_cfg!("V19_1610_UWIRE_SCLK",	 8,    6,    0,	  1,  20,   0,	  1,	 1,  1);
mux_cfg!("U18_1610_UWIRE_SDI",	 8,    0,    0,	  1,  18,   0,	  1,	 1,  1);
mux_cfg!("W21_1610_UWIRE_SDO",	 8,    3,    0,	  1,  19,   0,	  1,	 1,  1);
mux_cfg!("N14_1610_UWIRE_CS0",	 8,    9,    1,	  1,  21,   0,	  1,	 1,  1);
mux_cfg!("P15_1610_UWIRE_CS3",	 8,   12,    1,	  1,  22,   0,	  1,	 1,  1);
mux_cfg!("N15_1610_UWIRE_CS1",	 7,   18,    2,	  1,  14,   0,	 NA,	 0,  1);

/* OMAP-1610 SPI */
mux_cfg!("U19_1610_SPIF_SCK",	 7,    21,   6,	  1,  15,   0,	  1,	 1,  1);
mux_cfg!("U18_1610_SPIF_DIN",	 8,    0,    6,	  1,  18,   1,	  1,	 0,  1);
mux_cfg!("P20_1610_SPIF_DIN",	 6,    27,   4,   1,   7,   1,    1,     0,  1);
mux_cfg!("W21_1610_SPIF_DOUT",	 8,    3,    6,	  1,  19,   0,	  1,	 0,  1);
mux_cfg!("R18_1610_SPIF_DOUT",	 7,    9,    3,	  1,  11,   0,	  1,	 0,  1);
mux_cfg!("N14_1610_SPIF_CS0",	 8,    9,    6,	  1,  21,   0,	  1,	 1,  1);
mux_cfg!("N15_1610_SPIF_CS1",	 7,    18,   6,	  1,  14,   0,	  1,	 1,  1);
mux_cfg!("T19_1610_SPIF_CS2",	 7,    15,   4,	  1,  13,   0,	  1,	 1,  1);
mux_cfg!("P15_1610_SPIF_CS3",	 8,    12,   3,	  1,  22,   0,	  1,	 1,  1);

/* OMAP-1610 Flash */
mux_cfg!("L3_1610_FLASH_CS2B_OE",10,    6,    1,	 NA,   0,   0,	 NA,	 0,  1);
mux_cfg!("M8_1610_FLASH_CS2B_WE",10,    3,    1,	 NA,   0,   0,	 NA,	 0,  1);

/* First MMC interface, same on 1510, 1610 and 1710 */
mux_cfg!("MMC_CMD",		 A,   27,    0,	  2,  15,   1,	  2,	 1,  1);
mux_cfg!("MMC_DAT1",		 A,   24,    0,	  2,  14,   1,	  2,	 1,  1);
mux_cfg!("MMC_DAT2",		 A,   18,    0,	  2,  12,   1,	  2,	 1,  1);
mux_cfg!("MMC_DAT0",		 B,    0,    0,	  2,  16,   1,	  2,	 1,  1);
mux_cfg!("MMC_CLK",		 A,   21,    0,	 NA,   0,   0,	 NA,	 0,  1);
mux_cfg!("MMC_DAT3",		10,   15,    0,	  3,   8,   1,	  3,	 1,  1);
mux_cfg!("M15_1710_MMC_CLKI",	 6,   21,    2,   0,   0,   0,   NA,     0,  1);
mux_cfg!("P19_1710_MMC_CMDDIR",	 6,   24,    6,   0,   0,   0,   NA,     0,  1);
mux_cfg!("P20_1710_MMC_DATDIR0",	 6,   27,    5,   0,   0,   0,   NA,     0,  1);

/* OMAP-1610 USB0 alternate configuration */
mux_cfg!("W9_USB0_TXEN",		 B,   9,     5,	  2,  19,   0,	  2,	 0,  1);
mux_cfg!("AA9_USB0_VP",		 B,   6,     5,	  2,  18,   0,	  2,	 0,  1);
mux_cfg!("Y5_USB0_RCV",		 C,  21,     5,	  3,   1,   0,	  1,	 0,  1);
mux_cfg!("R9_USB0_VM",		 C,  18,     5,	  3,   0,   0,	  3,	 0,  1);
mux_cfg!("V6_USB0_TXD",		 C,  27,     5,	  3,   3,   0,	  3,	 0,  1);
mux_cfg!("W5_USB0_SE0",		 C,  24,     5,	  3,   2,   0,	  3,	 0,  1);
mux_cfg!("V9_USB0_SPEED",	 B,  12,     5,	  2,  20,   0,	  2,	 0,  1);
mux_cfg!("Y10_USB0_SUSP",	 B,   3,     5,	  2,  17,   0,	  2,	 0,  1);

/* USB2 interface */
mux_cfg!("W9_USB2_TXEN",		 B,   9,     1,	 NA,   0,   0,	 NA,	 0,  1);
mux_cfg!("AA9_USB2_VP",		 B,   6,     1,	 NA,   0,   0,	 NA,	 0,  1);
mux_cfg!("Y5_USB2_RCV",		 C,  21,     1,	 NA,   0,   0,	 NA,	 0,  1);
mux_cfg!("R9_USB2_VM",		 C,  18,     1,	 NA,   0,   0,	 NA,	 0,  1);
mux_cfg!("V6_USB2_TXD",		 C,  27,     2,	 NA,   0,   0,	 NA,	 0,  1);
mux_cfg!("W5_USB2_SE0",		 C,  24,     2,	 NA,   0,   0,	 NA,	 0,  1);

/* 16XX UART */
mux_cfg!("R13_1610_UART1_TX",	 A,  12,     6,	  2,  10,   0,	  2,	10,  1);
mux_cfg!("V14_16XX_UART1_RX",	 9,  18,     0,	  2,   2,   0,	  2,	 2,  1);
mux_cfg!("R14_1610_UART1_CTS",	 9,  15,     0,	  2,   1,   0,	  2,	 1,  1);
mux_cfg!("AA15_1610_UART1_RTS",	 9,  12,     1,	  2,   0,   0,	  2,	 0,  1);
mux_cfg!("R9_16XX_UART2_RX",	 C,  18,     0,   3,   0,   0,    3,     0,  1);
mux_cfg!("L14_16XX_UART3_RX",	 6,   3,     0,   0,  31,   0,    0,    31,  1);

/* I2C interface */
mux_cfg!("I2C_SCL",		 7,  24,     0,	 NA,   0,   0,	 NA,	 0,  0);
mux_cfg!("I2C_SDA",		 7,  27,     0,	 NA,   0,   0,	 NA,	 0,  0);

/* Keypad */
mux_cfg!("F18_1610_KBC0",	 3,  15,     0,	  0,   5,   1,	  0,	 0,  0);
mux_cfg!("D20_1610_KBC1",	 3,  12,     0,	  0,   4,   1,	  0,	 0,  0);
mux_cfg!("D19_1610_KBC2",	 3,   9,     0,	  0,   3,   1,	  0,	 0,  0);
mux_cfg!("E18_1610_KBC3",	 3,   6,     0,	  0,   2,   1,	  0,	 0,  0);
mux_cfg!("C21_1610_KBC4",	 3,   3,     0,	  0,   1,   1,	  0,	 0,  0);
mux_cfg!("G18_1610_KBR0",	 4,   0,     0,	  0,   10,  1,	  0,	 1,  0);
mux_cfg!("F19_1610_KBR1",	 3,   27,    0,	  0,   9,   1,	  0,	 1,  0);
mux_cfg!("H14_1610_KBR2",	 3,   24,    0,	  0,   8,   1,	  0,	 1,  0);
mux_cfg!("E20_1610_KBR3",	 3,   21,    0,	  0,   7,   1,	  0,	 1,  0);
mux_cfg!("E19_1610_KBR4",	 3,   18,    0,	  0,   6,   1,	  0,	 1,  0);
mux_cfg!("N19_1610_KBR5",	 6,  12,     1,	  1,   2,   1,	  1,	 1,  0);

/* Power management */
mux_cfg!("T20_1610_LOW_PWR",	 7,   12,    1,	  NA,   0,   0,   NA,	 0,  0);

/* MCLK Settings */
mux_cfg!("V5_1710_MCLK_ON",	 B,   15,    0,	  NA,   0,   0,   NA,	 0,  0);
mux_cfg!("V5_1710_MCLK_OFF",	 B,   15,    6,	  NA,   0,   0,   NA,	 0,  0);
mux_cfg!("R10_1610_MCLK_ON",	 B,   18,    0,	  NA,  22,   0,	  NA,	 1,  0);
mux_cfg!("R10_1610_MCLK_OFF",	 B,   18,    6,	  2,   22,   1,	  2,	 1,  1);

/* CompactFlash controller, conflicts with MMC1 */
mux_cfg!("P11_1610_CF_CD2",	 A,   27,    3,	  2,   15,   1,	  2,	 1,  1);
mux_cfg!("R11_1610_CF_IOIS16",	 B,    0,    3,	  2,   16,   1,	  2,	 1,  1);
mux_cfg!("V10_1610_CF_IREQ",	 A,   24,    3,	  2,   14,   0,	  2,	 0,  1);
mux_cfg!("W10_1610_CF_RESET",	 A,   18,    3,	  2,   12,   1,	  2,	 1,  1);
mux_cfg!("W11_1610_CF_CD1",	10,   15,    3,	  3,    8,   1,	  3,	 1,  1);

/* parallel camera */
mux_cfg!("J15_1610_CAM_LCLK",	 4,   24,    0,   0,  18,   1,    0,     0,  0);
mux_cfg!("J18_1610_CAM_D7",	 4,   27,    0,   0,  19,   1,    0,     0,  0);
mux_cfg!("J19_1610_CAM_D6",	 5,    0,    0,   0,  20,   1,    0,     0,  0);
mux_cfg!("J14_1610_CAM_D5",	 5,    3,    0,   0,  21,   1,    0,     0,  0);
mux_cfg!("K18_1610_CAM_D4",	 5,    6,    0,   0,  22,   1,    0,     0,  0);
mux_cfg!("K19_1610_CAM_D3",	 5,    9,    0,   0,  23,   1,    0,     0,  0);
mux_cfg!("K15_1610_CAM_D2",	 5,   12,    0,   0,  24,   1,    0,     0,  0);
mux_cfg!("K14_1610_CAM_D1",	 5,   15,    0,   0,  25,   1,    0,     0,  0);
mux_cfg!("L19_1610_CAM_D0",	 5,   18,    0,   0,  26,   1,    0,     0,  0);
mux_cfg!("L18_1610_CAM_VS",	 5,   21,    0,   0,  27,   1,    0,     0,  0);
mux_cfg!("L15_1610_CAM_HS",	 5,   24,    0,   0,  28,   1,    0,     0,  0);
mux_cfg!("M19_1610_CAM_RSTZ",	 5,   27,    0,   0,  29,   0,    0,     0,  0);
mux_cfg!("Y15_1610_CAM_OUTCLK",	 A,    0,    6,   2,   6,   0,    2,     0,  0);

/* serial camera */
mux_cfg!("H19_1610_CAM_EXCLK",	 4,   21,    0,   0,  17,   0,    0,     0,  0);
	/* REVISIT 5912 spec sez CCP_* can't pullup or pulldown ... ? */
mux_cfg!("Y12_1610_CCP_CLKP",	 8,   18,    6,   1,  24,   1,    1,     0,  0);
mux_cfg!("W13_1610_CCP_CLKM",	 9,    0,    6,   1,  28,   1,    1,     0,  0);
mux_cfg!("W14_1610_CCP_DATAP",	 9,   24,    6,   2,   4,   1,    2,     0,  0);
mux_cfg!("Y14_1610_CCP_DATAM",	 9,   21,    6,   2,   3,   1,    2,     0,  0);
};
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
