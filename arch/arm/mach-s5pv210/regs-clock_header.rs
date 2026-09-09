/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2010 Samsung Electronics Co., Ltd.
 *	http://www.samsung.com/
 *
 * S5PV210 - Clock register definitions
 */

/* C header guard: __ASM_ARCH_REGS_CLOCK_H */

pub const S3C_ADDR_BASE: usize = 0xF6000000;
macro_rules! S3C_ADDR { ($x:expr) => { S3C_ADDR_BASE + ($x) }; }
pub const S3C_VA_SYS: usize = S3C_ADDR!(0x00100000);
macro_rules! S5P_CLKREG { ($x:expr) => { S3C_VA_SYS + ($x) }; }

pub const S5P_APLL_LOCK: usize = S5P_CLKREG!(0x00);
pub const S5P_MPLL_LOCK: usize = S5P_CLKREG!(0x08);
pub const S5P_EPLL_LOCK: usize = S5P_CLKREG!(0x10);
pub const S5P_VPLL_LOCK: usize = S5P_CLKREG!(0x20);

pub const S5P_APLL_CON: usize = S5P_CLKREG!(0x100);
pub const S5P_MPLL_CON: usize = S5P_CLKREG!(0x108);
pub const S5P_EPLL_CON: usize = S5P_CLKREG!(0x110);
pub const S5P_EPLL_CON1: usize = S5P_CLKREG!(0x114);
pub const S5P_VPLL_CON: usize = S5P_CLKREG!(0x120);

pub const S5P_CLK_SRC0: usize = S5P_CLKREG!(0x200);
pub const S5P_CLK_SRC1: usize = S5P_CLKREG!(0x204);
pub const S5P_CLK_SRC2: usize = S5P_CLKREG!(0x208);
pub const S5P_CLK_SRC3: usize = S5P_CLKREG!(0x20C);
pub const S5P_CLK_SRC4: usize = S5P_CLKREG!(0x210);
pub const S5P_CLK_SRC5: usize = S5P_CLKREG!(0x214);
pub const S5P_CLK_SRC6: usize = S5P_CLKREG!(0x218);
pub const S5P_CLK_SRC_MASK0: usize = S5P_CLKREG!(0x280);
pub const S5P_CLK_SRC_MASK1: usize = S5P_CLKREG!(0x284);

pub const S5P_CLK_DIV0: usize = S5P_CLKREG!(0x300);
pub const S5P_CLK_DIV1: usize = S5P_CLKREG!(0x304);
pub const S5P_CLK_DIV2: usize = S5P_CLKREG!(0x308);
pub const S5P_CLK_DIV3: usize = S5P_CLKREG!(0x30C);
pub const S5P_CLK_DIV4: usize = S5P_CLKREG!(0x310);
pub const S5P_CLK_DIV5: usize = S5P_CLKREG!(0x314);
pub const S5P_CLK_DIV6: usize = S5P_CLKREG!(0x318);
pub const S5P_CLK_DIV7: usize = S5P_CLKREG!(0x31C);

pub const S5P_CLKGATE_MAIN0: usize = S5P_CLKREG!(0x400);
pub const S5P_CLKGATE_MAIN1: usize = S5P_CLKREG!(0x404);
pub const S5P_CLKGATE_MAIN2: usize = S5P_CLKREG!(0x408);
pub const S5P_CLKGATE_PERI0: usize = S5P_CLKREG!(0x420);
pub const S5P_CLKGATE_PERI1: usize = S5P_CLKREG!(0x424);
pub const S5P_CLKGATE_SCLK0: usize = S5P_CLKREG!(0x440);
pub const S5P_CLKGATE_SCLK1: usize = S5P_CLKREG!(0x444);
pub const S5P_CLKGATE_IP0: usize = S5P_CLKREG!(0x460);
pub const S5P_CLKGATE_IP1: usize = S5P_CLKREG!(0x464);
pub const S5P_CLKGATE_IP2: usize = S5P_CLKREG!(0x468);
pub const S5P_CLKGATE_IP3: usize = S5P_CLKREG!(0x46C);
pub const S5P_CLKGATE_IP4: usize = S5P_CLKREG!(0x470);
pub const S5P_CLKGATE_BLOCK: usize = S5P_CLKREG!(0x480);
pub const S5P_CLKGATE_BUS0: usize = S5P_CLKREG!(0x484);
pub const S5P_CLKGATE_BUS1: usize = S5P_CLKREG!(0x488);
pub const S5P_CLK_OUT: usize = S5P_CLKREG!(0x500);

/* DIV/MUX STATUS */
pub const S5P_CLKDIV_STAT0: usize = S5P_CLKREG!(0x1000);
pub const S5P_CLKDIV_STAT1: usize = S5P_CLKREG!(0x1004);
pub const S5P_CLKMUX_STAT0: usize = S5P_CLKREG!(0x1100);
pub const S5P_CLKMUX_STAT1: usize = S5P_CLKREG!(0x1104);

/* CLKSRC0 */
pub const S5P_CLKSRC0_MUX200_SHIFT: u32 = 16;
pub const S5P_CLKSRC0_MUX200_MASK: u32 = 0x1 << S5P_CLKSRC0_MUX200_SHIFT;
pub const S5P_CLKSRC0_MUX166_MASK: u32 = 0x1 << 20;
pub const S5P_CLKSRC0_MUX133_MASK: u32 = 0x1 << 24;

/* CLKSRC2 */
pub const S5P_CLKSRC2_G3D_SHIFT: u32 = 0;
pub const S5P_CLKSRC2_G3D_MASK: u32 = 0x3 << S5P_CLKSRC2_G3D_SHIFT;
pub const S5P_CLKSRC2_MFC_SHIFT: u32 = 4;
pub const S5P_CLKSRC2_MFC_MASK: u32 = 0x3 << S5P_CLKSRC2_MFC_SHIFT;

/* CLKSRC6 */
pub const S5P_CLKSRC6_ONEDRAM_SHIFT: u32 = 24;
pub const S5P_CLKSRC6_ONEDRAM_MASK: u32 = 0x3 << S5P_CLKSRC6_ONEDRAM_SHIFT;

/* CLKDIV0 */
pub const S5P_CLKDIV0_APLL_SHIFT: u32 = 0;
pub const S5P_CLKDIV0_APLL_MASK: u32 = 0x7 << S5P_CLKDIV0_APLL_SHIFT;
pub const S5P_CLKDIV0_A2M_SHIFT: u32 = 4;
pub const S5P_CLKDIV0_A2M_MASK: u32 = 0x7 << S5P_CLKDIV0_A2M_SHIFT;
pub const S5P_CLKDIV0_HCLK200_SHIFT: u32 = 8;
pub const S5P_CLKDIV0_HCLK200_MASK: u32 = 0x7 << S5P_CLKDIV0_HCLK200_SHIFT;
pub const S5P_CLKDIV0_PCLK100_SHIFT: u32 = 12;
pub const S5P_CLKDIV0_PCLK100_MASK: u32 = 0x7 << S5P_CLKDIV0_PCLK100_SHIFT;
pub const S5P_CLKDIV0_HCLK166_SHIFT: u32 = 16;
pub const S5P_CLKDIV0_HCLK166_MASK: u32 = 0xF << S5P_CLKDIV0_HCLK166_SHIFT;
pub const S5P_CLKDIV0_PCLK83_SHIFT: u32 = 20;
pub const S5P_CLKDIV0_PCLK83_MASK: u32 = 0x7 << S5P_CLKDIV0_PCLK83_SHIFT;
pub const S5P_CLKDIV0_HCLK133_SHIFT: u32 = 24;
pub const S5P_CLKDIV0_HCLK133_MASK: u32 = 0xF << S5P_CLKDIV0_HCLK133_SHIFT;
pub const S5P_CLKDIV0_PCLK66_SHIFT: u32 = 28;
pub const S5P_CLKDIV0_PCLK66_MASK: u32 = 0x7 << S5P_CLKDIV0_PCLK66_SHIFT;

/* CLKDIV2 */
pub const S5P_CLKDIV2_G3D_SHIFT: u32 = 0;
pub const S5P_CLKDIV2_G3D_MASK: u32 = 0xF << S5P_CLKDIV2_G3D_SHIFT;
pub const S5P_CLKDIV2_MFC_SHIFT: u32 = 4;
pub const S5P_CLKDIV2_MFC_MASK: u32 = 0xF << S5P_CLKDIV2_MFC_SHIFT;

/* CLKDIV6 */
pub const S5P_CLKDIV6_ONEDRAM_SHIFT: u32 = 28;
pub const S5P_CLKDIV6_ONEDRAM_MASK: u32 = 0xF << S5P_CLKDIV6_ONEDRAM_SHIFT;

pub const S5P_SWRESET: usize = S5P_CLKREG!(0x2000);
pub const S5P_ARM_MCS_CON: usize = S5P_CLKREG!(0x6100);

/* Registers related to power management */
pub const S5P_PWR_CFG: usize = S5P_CLKREG!(0xC000);
pub const S5P_EINT_WAKEUP_MASK: usize = S5P_CLKREG!(0xC004);
pub const S5P_WAKEUP_MASK: usize = S5P_CLKREG!(0xC008);
pub const S5P_PWR_MODE: usize = S5P_CLKREG!(0xC00C);
pub const S5P_NORMAL_CFG: usize = S5P_CLKREG!(0xC010);
pub const S5P_IDLE_CFG: usize = S5P_CLKREG!(0xC020);
pub const S5P_STOP_CFG: usize = S5P_CLKREG!(0xC030);
pub const S5P_STOP_MEM_CFG: usize = S5P_CLKREG!(0xC034);
pub const S5P_SLEEP_CFG: usize = S5P_CLKREG!(0xC040);
pub const S5P_OSC_FREQ: usize = S5P_CLKREG!(0xC100);
pub const S5P_OSC_STABLE: usize = S5P_CLKREG!(0xC104);
pub const S5P_PWR_STABLE: usize = S5P_CLKREG!(0xC108);
pub const S5P_MTC_STABLE: usize = S5P_CLKREG!(0xC110);
pub const S5P_CLAMP_STABLE: usize = S5P_CLKREG!(0xC114);
pub const S5P_WAKEUP_STAT: usize = S5P_CLKREG!(0xC200);
pub const S5P_BLK_PWR_STAT: usize = S5P_CLKREG!(0xC204);
pub const S5P_OTHERS: usize = S5P_CLKREG!(0xE000);
pub const S5P_OM_STAT: usize = S5P_CLKREG!(0xE100);
pub const S5P_HDMI_PHY_CONTROL: usize = S5P_CLKREG!(0xE804);
pub const S5P_USB_PHY_CONTROL: usize = S5P_CLKREG!(0xE80C);
pub const S5P_DAC_PHY_CONTROL: usize = S5P_CLKREG!(0xE810);
pub const S5P_INFORM0: usize = S5P_CLKREG!(0xF000);
pub const S5P_INFORM1: usize = S5P_CLKREG!(0xF004);
pub const S5P_INFORM2: usize = S5P_CLKREG!(0xF008);
pub const S5P_INFORM3: usize = S5P_CLKREG!(0xF00C);
pub const S5P_INFORM4: usize = S5P_CLKREG!(0xF010);
pub const S5P_INFORM5: usize = S5P_CLKREG!(0xF014);
pub const S5P_INFORM6: usize = S5P_CLKREG!(0xF018);
pub const S5P_INFORM7: usize = S5P_CLKREG!(0xF01C);
pub const S5P_RST_STAT: usize = S5P_CLKREG!(0xA000);
pub const S5P_OSC_CON: usize = S5P_CLKREG!(0x8000);
pub const S5P_MDNIE_SEL: usize = S5P_CLKREG!(0x7008);
pub const S5P_MIPI_PHY_CON0: usize = S5P_CLKREG!(0x7200);
pub const S5P_MIPI_PHY_CON1: usize = S5P_CLKREG!(0x7204);

pub const S5P_IDLE_CFG_TL_MASK: u32 = 3 << 30;
pub const S5P_IDLE_CFG_TM_MASK: u32 = 3 << 28;
pub const S5P_IDLE_CFG_TL_ON: u32 = 2 << 30;
pub const S5P_IDLE_CFG_TM_ON: u32 = 2 << 28;
pub const S5P_IDLE_CFG_DIDLE: u32 = 1 << 0;
pub const S5P_CFG_WFI_CLEAN: u32 = !(3 << 8);
pub const S5P_CFG_WFI_IDLE: u32 = 1 << 8;
pub const S5P_CFG_WFI_STOP: u32 = 2 << 8;
pub const S5P_CFG_WFI_SLEEP: u32 = 3 << 8;
pub const S5P_OTHER_SYS_INT: u32 = 24;
pub const S5P_OTHER_STA_TYPE: u32 = 23;
pub const S5P_OTHER_SYSC_INTOFF: u32 = 1 << 0;
pub const STA_TYPE_EXPON: u32 = 0;
pub const STA_TYPE_SFR: u32 = 1;
pub const S5P_PWR_STA_EXP_SCALE: u32 = 0;
pub const S5P_PWR_STA_CNT: u32 = 4;
pub const S5P_PWR_STABLE_COUNT: u32 = 85500;
pub const S5P_SLEEP_CFG_OSC_EN: u32 = 1 << 0;
pub const S5P_SLEEP_CFG_USBOSC_EN: u32 = 1 << 1;
/* OTHERS Resgister */
pub const S5P_OTHERS_USB_SIG_MASK: u32 = 1 << 16;
/* S5P_DAC_CONTROL */
pub const S5P_DAC_ENABLE: u32 = 1;
pub const S5P_DAC_DISABLE: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
