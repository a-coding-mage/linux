/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/mach-pxa/include/mach/pxa3xx-regs.h
 *
 * PXA3xx specific register definitions
 *
 * Copyright (C) 2007 Marvell International Ltd.
 */

// Dependency: `io_p2v` and `__REG` are supplied by pxa-regs.

/* Oscillator Configuration Register (OSCC) */
pub const OSCC: usize = io_p2v(0x41350000);
pub const OSCC_PEN: u32 = 1u32 << 11; // 13MHz POUT

/* Service Power Management Unit (MPMU) */
pub const PMCR: usize = __REG(0x40F50000); // Power Manager Control Register
pub const PSR: usize = __REG(0x40F50004); // Power Manager S2 Status Register
pub const PSPR: usize = __REG(0x40F50008); // Power Manager Scratch Pad Register
pub const PCFR: usize = __REG(0x40F5000C); // Power Manager General Configuration Register
pub const PWER: usize = __REG(0x40F50010); // Power Manager Wake-up Enable Register
pub const PWSR: usize = __REG(0x40F50014); // Power Manager Wake-up Status Register
pub const PECR: usize = __REG(0x40F50018); // Power Manager EXT_WAKEUP[1:0] Control Register
pub const DCDCSR: usize = __REG(0x40F50080); // DC-DC Controller Status Register
pub const PVCR: usize = __REG(0x40F50100); // Power Manager Voltage Change Control Register

#[inline]
pub const fn PCMD(x: usize) -> usize {
    __REG(0x40F50110 + (x << 2))
}

/* Slave Power Management Unit */
pub const ASCR: usize = __REG(0x40f40000); // Application Subsystem Power Status/Configuration
pub const ARSR: usize = __REG(0x40f40004); // Application Subsystem Reset Status
pub const AD3ER: usize = __REG(0x40f40008); // Application Subsystem Wake-Up from D3 Enable
pub const AD3SR: usize = __REG(0x40f4000c); // Application Subsystem Wake-Up from D3 Status
pub const AD2D0ER: usize = __REG(0x40f40010); // Application Subsystem Wake-Up from D2 to D0 Enable
pub const AD2D0SR: usize = __REG(0x40f40014); // Application Subsystem Wake-Up from D2 to D0 Status
pub const AD2D1ER: usize = __REG(0x40f40018); // Application Subsystem Wake-Up from D2 to D1 Enable
pub const AD2D1SR: usize = __REG(0x40f4001c); // Application Subsystem Wake-Up from D2 to D1 Status
pub const AD1D0ER: usize = __REG(0x40f40020); // Application Subsystem Wake-Up from D1 to D0 Enable
pub const AD1D0SR: usize = __REG(0x40f40024); // Application Subsystem Wake-Up from D1 to D0 Status
pub const AGENP: usize = __REG(0x40f4002c); // Application Subsystem General Purpose
pub const AD3R: usize = __REG(0x40f40030); // Application Subsystem D3 Configuration
pub const AD2R: usize = __REG(0x40f40034); // Application Subsystem D2 Configuration
pub const AD1R: usize = __REG(0x40f40038); // Application Subsystem D1 Configuration

/* Application Subsystem Configuration bits. */
pub const ASCR_RDH: u32 = 1u32 << 31;
pub const ASCR_D1S: u32 = 1u32 << 2;
pub const ASCR_D2S: u32 = 1u32 << 1;
pub const ASCR_D3S: u32 = 1u32 << 0;

/* Application Reset Status bits. */
pub const ARSR_GPR: u32 = 1u32 << 3;
pub const ARSR_LPMR: u32 = 1u32 << 2;
pub const ARSR_WDT: u32 = 1u32 << 1;
pub const ARSR_HWR: u32 = 1u32 << 0;

/* Application Subsystem Wake-Up bits. */
pub const ADXER_WRTC: u32 = 1u32 << 31; // RTC
pub const ADXER_WOST: u32 = 1u32 << 30; // OS Timer
pub const ADXER_WTSI: u32 = 1u32 << 29; // Touchscreen
pub const ADXER_WUSBH: u32 = 1u32 << 28; // USB host
pub const ADXER_WUSB2: u32 = 1u32 << 26; // USB client 2.0
pub const ADXER_WMSL0: u32 = 1u32 << 24; // MSL port 0
pub const ADXER_WDMUX3: u32 = 1u32 << 23; // USB EDMUX3
pub const ADXER_WDMUX2: u32 = 1u32 << 22; // USB EDMUX2
pub const ADXER_WKP: u32 = 1u32 << 21; // Keypad
pub const ADXER_WUSIM1: u32 = 1u32 << 20; // USIM Port 1
pub const ADXER_WUSIM0: u32 = 1u32 << 19; // USIM Port 0
pub const ADXER_WOTG: u32 = 1u32 << 16; // USBOTG input
pub const ADXER_MFP_WFLASH: u32 = 1u32 << 15; // MFP: Data flash busy
pub const ADXER_MFP_GEN12: u32 = 1u32 << 14; // MFP: MMC3/GPIO/OST inputs
pub const ADXER_MFP_WMMC2: u32 = 1u32 << 13; // MFP: MMC2
pub const ADXER_MFP_WMMC1: u32 = 1u32 << 12; // MFP: MMC1
pub const ADXER_MFP_WI2C: u32 = 1u32 << 11; // MFP: I2C
pub const ADXER_MFP_WSSP4: u32 = 1u32 << 10; // MFP: SSP4
pub const ADXER_MFP_WSSP3: u32 = 1u32 << 9; // MFP: SSP3
pub const ADXER_MFP_WMAXTRIX: u32 = 1u32 << 8; // MFP: matrix keypad
pub const ADXER_MFP_WUART3: u32 = 1u32 << 7; // MFP: UART3
pub const ADXER_MFP_WUART2: u32 = 1u32 << 6; // MFP: UART2
pub const ADXER_MFP_WUART1: u32 = 1u32 << 5; // MFP: UART1
pub const ADXER_MFP_WSSP2: u32 = 1u32 << 4; // MFP: SSP2
pub const ADXER_MFP_WSSP1: u32 = 1u32 << 3; // MFP: SSP1
pub const ADXER_MFP_WAC97: u32 = 1u32 << 2; // MFP: AC97
pub const ADXER_WEXTWAKE1: u32 = 1u32 << 1; // External Wake 1
pub const ADXER_WEXTWAKE0: u32 = 1u32 << 0; // External Wake 0

/* AD3R/AD2R/AD1R bits. R2-R5 are only defined for PXA320. */
pub const ADXR_L2: u32 = 1u32 << 8;
pub const ADXR_R5: u32 = 1u32 << 5;
pub const ADXR_R4: u32 = 1u32 << 4;
pub const ADXR_R3: u32 = 1u32 << 3;
pub const ADXR_R2: u32 = 1u32 << 2;
pub const ADXR_R1: u32 = 1u32 << 1;
pub const ADXR_R0: u32 = 1u32 << 0;

/* Values for PWRMODE CP15 register */
pub const PXA3xx_PM_S3D4C4: u32 = 0x07; // aka deep sleep
pub const PXA3xx_PM_S2D3C4: u32 = 0x06; // aka sleep
pub const PXA3xx_PM_S0D2C2: u32 = 0x03; // aka standby
pub const PXA3xx_PM_S0D1C2: u32 = 0x02; // aka LCD refresh
pub const PXA3xx_PM_S0D0C1: u32 = 0x01;

/* Application Subsystem Clock */
pub const ACCR: usize = __REG(0x41340000); // Application Subsystem Clock Configuration Register
pub const ACSR: usize = __REG(0x41340004); // Application Subsystem Clock Status Register
pub const AICSR: usize = __REG(0x41340008); // Application Subsystem Interrupt Control/Status Register
pub const CKENA: usize = __REG(0x4134000C); // A Clock Enable Register
pub const CKENB: usize = __REG(0x41340010); // B Clock Enable Register
pub const CKENC: usize = __REG(0x41340024); // C Clock Enable Register
pub const AC97_DIV: usize = __REG(0x41340014); // AC97 clock divisor value register

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
