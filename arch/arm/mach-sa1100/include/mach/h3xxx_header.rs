/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Definitions for Compaq iPAQ H3100 and H3600 handheld computers
 *
 * (c) 2000 Compaq Computer Corporation. (Author: Jamey Hicks)
 * (c) 2009 Dmitry Artamonow <mad_soft@inbox.ru>
 */

// `hardware.h` supplies `GPIO_MAX`, `SA1100_CS2_PHYS`, `SA1100_CS4_PHYS`,
// and `SA1100_CS5_PHYS`.

/* Physical memory regions corresponding to chip selects */
pub const H3600_EGPIO_PHYS: u32 = SA1100_CS5_PHYS + 0x0100_0000;
pub const H3600_BANK_2_PHYS: u32 = SA1100_CS2_PHYS;
pub const H3600_BANK_4_PHYS: u32 = SA1100_CS4_PHYS;

/* Virtual memory regions corresponding to chip selects 2 & 4 (used on sleeves) */
pub const H3600_EGPIO_VIRT: u32 = 0xf000_0000;
pub const H3600_BANK_2_VIRT: u32 = 0xf100_0000;
pub const H3600_BANK_4_VIRT: u32 = 0xf380_0000;

/*
 * gpiolib numbers for all iPAQs
 */
pub const H3XXX_GPIO_PWR_BUTTON: u32 = 0;
pub const H3XXX_GPIO_PCMCIA_CD1: u32 = 10;
pub const H3XXX_GPIO_PCMCIA_IRQ1: u32 = 11;
pub const H3XXX_GPIO_PCMCIA_CD0: u32 = 17;
pub const H3XXX_GPIO_ACTION_BUTTON: u32 = 18;
pub const H3XXX_GPIO_SYS_CLK: u32 = 19;
pub const H3XXX_GPIO_PCMCIA_IRQ0: u32 = 21;
pub const H3XXX_GPIO_COM_DCD: u32 = 23;
pub const H3XXX_GPIO_OPTION: u32 = 24;
pub const H3XXX_GPIO_COM_CTS: u32 = 25;
pub const H3XXX_GPIO_COM_RTS: u32 = 26;

/* machine-specific gpios */
pub const H3100_GPIO_BT_ON: u32 = 2;
pub const H3100_GPIO_QMUTE: u32 = 4;
pub const H3100_GPIO_LCD_3V_ON: u32 = 5;
pub const H3100_GPIO_AUD_ON: u32 = 6;
pub const H3100_GPIO_AUD_PWR_ON: u32 = 7;
pub const H3100_GPIO_IR_ON: u32 = 8;
pub const H3100_GPIO_IR_FSEL: u32 = 9;

pub const H3600_GPIO_CLK_SET0: u32 = 12; /* audio sample rate clock generator */
pub const H3600_GPIO_CLK_SET1: u32 = 13;
pub const H3600_GPIO_SOFT_RESET: u32 = 20; /* also known as BATT_FAULT */
pub const H3600_GPIO_OPT_LOCK: u32 = 22;
pub const H3600_GPIO_OPT_DET: u32 = 27;

/* H3100 / 3600 EGPIO pins */
pub const H3XXX_EGPIO_BASE: u32 = GPIO_MAX + 1;

pub const H3XXX_EGPIO_VPP_ON: u32 = H3XXX_EGPIO_BASE + 0;
pub const H3XXX_EGPIO_CARD_RESET: u32 = H3XXX_EGPIO_BASE + 1; /* reset the attached pcmcia/compactflash card.  active high. */
pub const H3XXX_EGPIO_OPT_RESET: u32 = H3XXX_EGPIO_BASE + 2; /* reset the attached option pack.  active high. */
pub const H3XXX_EGPIO_CODEC_NRESET: u32 = H3XXX_EGPIO_BASE + 3; /* reset the onboard UDA1341.  active low. */
pub const H3XXX_EGPIO_OPT_NVRAM_ON: u32 = H3XXX_EGPIO_BASE + 4; /* apply power to optionpack nvram, active high. */
pub const H3XXX_EGPIO_OPT_ON: u32 = H3XXX_EGPIO_BASE + 5; /* full power to option pack.  active high. */
pub const H3XXX_EGPIO_LCD_ON: u32 = H3XXX_EGPIO_BASE + 6; /* enable 3.3V to LCD.  active high. */
pub const H3XXX_EGPIO_RS232_ON: u32 = H3XXX_EGPIO_BASE + 7; /* UART3 transceiver force on.  Active high. */

/* H3600 only EGPIO pins */
pub const H3600_EGPIO_LCD_PCI: u32 = H3XXX_EGPIO_BASE + 8; /* LCD control IC enable.  active high. */
pub const H3600_EGPIO_IR_ON: u32 = H3XXX_EGPIO_BASE + 9; /* apply power to IR module.  active high. */
pub const H3600_EGPIO_AUD_AMP_ON: u32 = H3XXX_EGPIO_BASE + 10; /* apply power to audio power amp.  active high. */
pub const H3600_EGPIO_AUD_PWR_ON: u32 = H3XXX_EGPIO_BASE + 11; /* apply power to reset of audio circuit.  active high. */
pub const H3600_EGPIO_QMUTE: u32 = H3XXX_EGPIO_BASE + 12; /* mute control for onboard UDA1341.  active high. */
pub const H3600_EGPIO_IR_FSEL: u32 = H3XXX_EGPIO_BASE + 13; /* IR speed select: 1->fast, 0->slow */
pub const H3600_EGPIO_LCD_5V_ON: u32 = H3XXX_EGPIO_BASE + 14; /* enable 5V to LCD. active high. */
pub const H3600_EGPIO_LVDD_ON: u32 = H3XXX_EGPIO_BASE + 15; /* enable 9V and -6.5V to LCD. */

extern "C" {
    pub fn h3xxx_map_io();
    pub fn h3xxx_mach_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
