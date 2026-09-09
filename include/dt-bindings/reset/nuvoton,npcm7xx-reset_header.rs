/* SPDX-License-Identifier: GPL-2.0 */
// Copyright (c) 2019 Nuvoton Technology corporation.

pub const NPCM7XX_RESET_IPSRST1: u32 = 0x20;
pub const NPCM7XX_RESET_IPSRST2: u32 = 0x24;
pub const NPCM7XX_RESET_IPSRST3: u32 = 0x34;

/* Reset lines on IP1 reset module (NPCM7XX_RESET_IPSRST1) */
pub const NPCM7XX_RESET_FIU3: u32 = 1;
pub const NPCM7XX_RESET_UDC1: u32 = 5;
pub const NPCM7XX_RESET_EMC1: u32 = 6;
pub const NPCM7XX_RESET_UART_2_3: u32 = 7;
pub const NPCM7XX_RESET_UDC2: u32 = 8;
pub const NPCM7XX_RESET_PECI: u32 = 9;
pub const NPCM7XX_RESET_AES: u32 = 10;
pub const NPCM7XX_RESET_UART_0_1: u32 = 11;
pub const NPCM7XX_RESET_MC: u32 = 12;
pub const NPCM7XX_RESET_SMB2: u32 = 13;
pub const NPCM7XX_RESET_SMB3: u32 = 14;
pub const NPCM7XX_RESET_SMB4: u32 = 15;
pub const NPCM7XX_RESET_SMB5: u32 = 16;
pub const NPCM7XX_RESET_PWM_M0: u32 = 18;
pub const NPCM7XX_RESET_TIMER_0_4: u32 = 19;
pub const NPCM7XX_RESET_TIMER_5_9: u32 = 20;
pub const NPCM7XX_RESET_EMC2: u32 = 21;
pub const NPCM7XX_RESET_UDC4: u32 = 22;
pub const NPCM7XX_RESET_UDC5: u32 = 23;
pub const NPCM7XX_RESET_UDC6: u32 = 24;
pub const NPCM7XX_RESET_UDC3: u32 = 25;
pub const NPCM7XX_RESET_ADC: u32 = 27;
pub const NPCM7XX_RESET_SMB6: u32 = 28;
pub const NPCM7XX_RESET_SMB7: u32 = 29;
pub const NPCM7XX_RESET_SMB0: u32 = 30;
pub const NPCM7XX_RESET_SMB1: u32 = 31;

/* Reset lines on IP2 reset module (NPCM7XX_RESET_IPSRST2) */
pub const NPCM7XX_RESET_MFT0: u32 = 0;
pub const NPCM7XX_RESET_MFT1: u32 = 1;
pub const NPCM7XX_RESET_MFT2: u32 = 2;
pub const NPCM7XX_RESET_MFT3: u32 = 3;
pub const NPCM7XX_RESET_MFT4: u32 = 4;
pub const NPCM7XX_RESET_MFT5: u32 = 5;
pub const NPCM7XX_RESET_MFT6: u32 = 6;
pub const NPCM7XX_RESET_MFT7: u32 = 7;
pub const NPCM7XX_RESET_MMC: u32 = 8;
pub const NPCM7XX_RESET_SDHC: u32 = 9;
pub const NPCM7XX_RESET_GFX_SYS: u32 = 10;
pub const NPCM7XX_RESET_AHB_PCIBRG: u32 = 11;
pub const NPCM7XX_RESET_VDMA: u32 = 12;
pub const NPCM7XX_RESET_ECE: u32 = 13;
pub const NPCM7XX_RESET_VCD: u32 = 14;
pub const NPCM7XX_RESET_OTP: u32 = 16;
pub const NPCM7XX_RESET_SIOX1: u32 = 18;
pub const NPCM7XX_RESET_SIOX2: u32 = 19;
pub const NPCM7XX_RESET_3DES: u32 = 21;
pub const NPCM7XX_RESET_PSPI1: u32 = 22;
pub const NPCM7XX_RESET_PSPI2: u32 = 23;
pub const NPCM7XX_RESET_GMAC2: u32 = 25;
pub const NPCM7XX_RESET_USB_HOST: u32 = 26;
pub const NPCM7XX_RESET_GMAC1: u32 = 28;
pub const NPCM7XX_RESET_CP: u32 = 31;

/* Reset lines on IP3 reset module (NPCM7XX_RESET_IPSRST3) */
pub const NPCM7XX_RESET_PWM_M1: u32 = 0;
pub const NPCM7XX_RESET_SMB12: u32 = 1;
pub const NPCM7XX_RESET_SPIX: u32 = 2;
pub const NPCM7XX_RESET_SMB13: u32 = 3;
pub const NPCM7XX_RESET_UDC0: u32 = 4;
pub const NPCM7XX_RESET_UDC7: u32 = 5;
pub const NPCM7XX_RESET_UDC8: u32 = 6;
pub const NPCM7XX_RESET_UDC9: u32 = 7;
pub const NPCM7XX_RESET_PCI_MAILBOX: u32 = 9;
pub const NPCM7XX_RESET_SMB14: u32 = 12;
pub const NPCM7XX_RESET_SHA: u32 = 13;
pub const NPCM7XX_RESET_SEC_ECC: u32 = 14;
pub const NPCM7XX_RESET_PCIE_RC: u32 = 15;
pub const NPCM7XX_RESET_TIMER_10_14: u32 = 16;
pub const NPCM7XX_RESET_RNG: u32 = 17;
pub const NPCM7XX_RESET_SMB15: u32 = 18;
pub const NPCM7XX_RESET_SMB8: u32 = 19;
pub const NPCM7XX_RESET_SMB9: u32 = 20;
pub const NPCM7XX_RESET_SMB10: u32 = 21;
pub const NPCM7XX_RESET_SMB11: u32 = 22;
pub const NPCM7XX_RESET_ESPI: u32 = 23;
pub const NPCM7XX_RESET_USB_PHY_1: u32 = 24;
pub const NPCM7XX_RESET_USB_PHY_2: u32 = 25;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
