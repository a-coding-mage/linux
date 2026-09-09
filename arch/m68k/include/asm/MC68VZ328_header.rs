/* SPDX-License-Identifier: GPL-2.0 */

/* include/asm-m68knommu/MC68VZ328.h: 'VZ328 control registers
 *
 * Copyright (c) 2000-2001	Lineo Inc. <www.lineo.com>
 * Copyright (c) 2000-2001	Lineo Canada Corp. <www.lineo.ca>
 * Copyright (C) 1999		Vladimir Gurevich <vgurevic@cisco.com>
 * 				Bare & Hare Software, Inc.
 * Based on include/asm-m68knommu/MC68332.h
 * Copyright (C) 1998  Kenneth Albanowski <kjahds@kjahds.com>,
 *                     The Silver Hammer Group, Ltd.
 *
 * M68VZ328 fixes by Evan Stawnyczy <evan@lineo.com>
 * vz multiport fixes by Michael Leslie <mleslie@lineo.com>
 */


pub const fn BYTE_REF(addr: u32) -> u32 { (*((volatile unsigned char*)addr)) }
pub const fn WORD_REF(addr: u32) -> u32 { (*((volatile unsigned short*)addr)) }
pub const fn LONG_REF(addr: u32) -> u32 { (*((volatile unsigned long*)addr)) }

pub const fn PUT_FIELD(field: u32, val: u32) -> u32 { (((val) << field##_SHIFT) & field##_MASK) }
pub const fn GET_FIELD(reg: u32, field: u32) -> u32 { (((reg) & field##_MASK) >> field##_SHIFT) }

/********** 
 *
 * 0xFFFFF0xx -- System Control
 *
 **********/
 
/*
 * System Control Register (SCR)
 */
pub const SCR_ADDR: u32 = 0xfffff000;
pub unsafe fn SCR() -> &'static mut u8 { &mut *(SCR_ADDR as *mut u8) }

pub const SCR_WDTH8: u32 = 0x01;
pub const SCR_DMAP: u32 = 0x04;
pub const SCR_SO: u32 = 0x08;
pub const SCR_BETEN: u32 = 0x10;
pub const SCR_PRV: u32 = 0x20;
pub const SCR_WPV: u32 = 0x40;
pub const SCR_BETO: u32 = 0x80;

/*
 * Silicon ID Register (Mask Revision Register (MRR) for '328 Compatibility)
 */
pub const MRR_ADDR: u32 = 0xfffff004;
pub unsafe fn MRR() -> &'static mut u32 { &mut *(MRR_ADDR as *mut u32) }

/********** 
 *
 * 0xFFFFF1xx -- Chip-Select logic
 *
 **********/
 
/*
 * Chip Select Group Base Registers 
 */
pub const CSGBA_ADDR: u32 = 0xfffff100;
pub const CSGBB_ADDR: u32 = 0xfffff102;

pub const CSGBC_ADDR: u32 = 0xfffff104;
pub const CSGBD_ADDR: u32 = 0xfffff106;

pub unsafe fn CSGBA() -> &'static mut u16 { &mut *(CSGBA_ADDR as *mut u16) }
pub unsafe fn CSGBB() -> &'static mut u16 { &mut *(CSGBB_ADDR as *mut u16) }
pub unsafe fn CSGBC() -> &'static mut u16 { &mut *(CSGBC_ADDR as *mut u16) }
pub unsafe fn CSGBD() -> &'static mut u16 { &mut *(CSGBD_ADDR as *mut u16) }

/*
 * Chip Select Registers 
 */
pub const CSA_ADDR: u32 = 0xfffff110;
pub const CSB_ADDR: u32 = 0xfffff112;
pub const CSC_ADDR: u32 = 0xfffff114;
pub const CSD_ADDR: u32 = 0xfffff116;

pub unsafe fn CSA() -> &'static mut u16 { &mut *(CSA_ADDR as *mut u16) }
pub unsafe fn CSB() -> &'static mut u16 { &mut *(CSB_ADDR as *mut u16) }
pub unsafe fn CSC() -> &'static mut u16 { &mut *(CSC_ADDR as *mut u16) }
pub unsafe fn CSD() -> &'static mut u16 { &mut *(CSD_ADDR as *mut u16) }

pub const CSA_EN: u32 = 0x0001;
pub const CSA_SIZ_MASK: u32 = 0x000e;
pub const CSA_SIZ_SHIFT: u32 = 1;
pub const CSA_WS_MASK: u32 = 0x0070;
pub const CSA_WS_SHIFT: u32 = 4;
pub const CSA_BSW: u32 = 0x0080;
pub const CSA_FLASH: u32 = 0x0100;
pub const CSA_RO: u32 = 0x8000;

pub const CSB_EN: u32 = 0x0001;
pub const CSB_SIZ_MASK: u32 = 0x000e;
pub const CSB_SIZ_SHIFT: u32 = 1;
pub const CSB_WS_MASK: u32 = 0x0070;
pub const CSB_WS_SHIFT: u32 = 4;
pub const CSB_BSW: u32 = 0x0080;
pub const CSB_FLASH: u32 = 0x0100;
pub const CSB_UPSIZ_MASK: u32 = 0x1800;
pub const CSB_UPSIZ_SHIFT: u32 = 11;
pub const CSB_ROP: u32 = 0x2000;
pub const CSB_SOP: u32 = 0x4000;
pub const CSB_RO: u32 = 0x8000;

pub const CSC_EN: u32 = 0x0001;
pub const CSC_SIZ_MASK: u32 = 0x000e;
pub const CSC_SIZ_SHIFT: u32 = 1;
pub const CSC_WS_MASK: u32 = 0x0070;
pub const CSC_WS_SHIFT: u32 = 4;
pub const CSC_BSW: u32 = 0x0080;
pub const CSC_FLASH: u32 = 0x0100;
pub const CSC_UPSIZ_MASK: u32 = 0x1800;
pub const CSC_UPSIZ_SHIFT: u32 = 11;
pub const CSC_ROP: u32 = 0x2000;
pub const CSC_SOP: u32 = 0x4000;
pub const CSC_RO: u32 = 0x8000;

pub const CSD_EN: u32 = 0x0001;
pub const CSD_SIZ_MASK: u32 = 0x000e;
pub const CSD_SIZ_SHIFT: u32 = 1;
pub const CSD_WS_MASK: u32 = 0x0070;
pub const CSD_WS_SHIFT: u32 = 4;
pub const CSD_BSW: u32 = 0x0080;
pub const CSD_FLASH: u32 = 0x0100;
pub const CSD_DRAM: u32 = 0x0200;
pub const CSD_COMB: u32 = 0x0400;
pub const CSD_UPSIZ_MASK: u32 = 0x1800;
pub const CSD_UPSIZ_SHIFT: u32 = 11;
pub const CSD_ROP: u32 = 0x2000;
pub const CSD_SOP: u32 = 0x4000;
pub const CSD_RO: u32 = 0x8000;

/*
 * Emulation Chip-Select Register 
 */
pub const EMUCS_ADDR: u32 = 0xfffff118;
pub unsafe fn EMUCS() -> &'static mut u16 { &mut *(EMUCS_ADDR as *mut u16) }

pub const EMUCS_WS_MASK: u32 = 0x0070;
pub const EMUCS_WS_SHIFT: u32 = 4;

/********** 
 *
 * 0xFFFFF2xx -- Phase Locked Loop (PLL) & Power Control
 *
 **********/

/*
 * PLL Control Register 
 */
pub const PLLCR_ADDR: u32 = 0xfffff200;
pub unsafe fn PLLCR() -> &'static mut u16 { &mut *(PLLCR_ADDR as *mut u16) }

pub const PLLCR_DISPLL: u32 = 0x0008;
pub const PLLCR_CLKEN: u32 = 0x0010;
pub const PLLCR_PRESC: u32 = 0x0020;
pub const PLLCR_SYSCLK_SEL_MASK: u32 = 0x0700;
pub const PLLCR_SYSCLK_SEL_SHIFT: u32 = 8;
pub const PLLCR_LCDCLK_SEL_MASK: u32 = 0x3800;
pub const PLLCR_LCDCLK_SEL_SHIFT: u32 = 11;

/* '328-compatible definitions */
pub const PLLCR_PIXCLK_SEL_MASK: u32 = PLLCR_LCDCLK_SEL_MASK;
pub const PLLCR_PIXCLK_SEL_SHIFT: u32 = PLLCR_LCDCLK_SEL_SHIFT;

/*
 * PLL Frequency Select Register
 */
pub const PLLFSR_ADDR: u32 = 0xfffff202;
pub unsafe fn PLLFSR() -> &'static mut u16 { &mut *(PLLFSR_ADDR as *mut u16) }

pub const PLLFSR_PC_MASK: u32 = 0x00ff;
pub const PLLFSR_PC_SHIFT: u32 = 0;
pub const PLLFSR_QC_MASK: u32 = 0x0f00;
pub const PLLFSR_QC_SHIFT: u32 = 8;
pub const PLLFSR_PROT: u32 = 0x4000;
pub const PLLFSR_CLK32: u32 = 0x8000;

/*
 * Power Control Register
 */
pub const PCTRL_ADDR: u32 = 0xfffff207;
pub unsafe fn PCTRL() -> &'static mut u8 { &mut *(PCTRL_ADDR as *mut u8) }

pub const PCTRL_WIDTH_MASK: u32 = 0x1f;
pub const PCTRL_WIDTH_SHIFT: u32 = 0;
pub const PCTRL_PCEN: u32 = 0x80;

/**********
 *
 * 0xFFFFF3xx -- Interrupt Controller
 *
 **********/

/* 
 * Interrupt Vector Register
 */
pub const IVR_ADDR: u32 = 0xfffff300;
pub unsafe fn IVR() -> &'static mut u8 { &mut *(IVR_ADDR as *mut u8) }

pub const IVR_VECTOR_MASK: u32 = 0xF8;

/*
 * Interrupt control Register
 */
pub const ICR_ADDR: u32 = 0xfffff302;
pub unsafe fn ICR() -> &'static mut u16 { &mut *(ICR_ADDR as *mut u16) }

pub const ICR_POL5: u32 = 0x0080;
pub const ICR_ET6: u32 = 0x0100;
pub const ICR_ET3: u32 = 0x0200;
pub const ICR_ET2: u32 = 0x0400;
pub const ICR_ET1: u32 = 0x0800;
pub const ICR_POL6: u32 = 0x1000;
pub const ICR_POL3: u32 = 0x2000;
pub const ICR_POL2: u32 = 0x4000;
pub const ICR_POL1: u32 = 0x8000;

/*
 * Interrupt Mask Register
 */
pub const IMR_ADDR: u32 = 0xfffff304;
pub unsafe fn IMR() -> &'static mut u32 { &mut *(IMR_ADDR as *mut u32) }

/*
 * Define the names for bit positions first. This is useful for 
 * request_irq
 */
pub const SPI2_IRQ_NUM: u32 = 0;
pub const TMR_IRQ_NUM: u32 = 1;
pub const UART1_IRQ_NUM: u32 = 2;
pub const WDT_IRQ_NUM: u32 = 3;
pub const RTC_IRQ_NUM: u32 = 4;
pub const TMR2_IRQ_NUM: u32 = 5;
pub const KB_IRQ_NUM: u32 = 6;
pub const PWM1_IRQ_NUM: u32 = 7;
pub const INT0_IRQ_NUM: u32 = 8;
pub const INT1_IRQ_NUM: u32 = 9;
pub const INT2_IRQ_NUM: u32 = 10;
pub const INT3_IRQ_NUM: u32 = 11;
pub const UART2_IRQ_NUM: u32 = 12;
pub const PWM2_IRQ_NUM: u32 = 13;
pub const IRQ1_IRQ_NUM: u32 = 16;
pub const IRQ2_IRQ_NUM: u32 = 17;
pub const IRQ3_IRQ_NUM: u32 = 18;
pub const IRQ6_IRQ_NUM: u32 = 19;
pub const IRQ5_IRQ_NUM: u32 = 20;
pub const SPI1_IRQ_NUM: u32 = 21;
pub const SAM_IRQ_NUM: u32 = 22;
pub const EMIQ_IRQ_NUM: u32 = 23;

pub const SPI_IRQ_NUM: u32 = SPI2_IRQ_NUM;

/* '328-compatible definitions */
pub const SPIM_IRQ_NUM: u32 = SPI_IRQ_NUM;
pub const TMR1_IRQ_NUM: u32 = TMR_IRQ_NUM;
pub const UART_IRQ_NUM: u32 = UART1_IRQ_NUM;

/* 
 * Here go the bitmasks themselves
 */
pub const IMR_MSPI: u32 = (1 << SPI_IRQ_NUM);
pub const IMR_MTMR: u32 = (1 << TMR_IRQ_NUM);
pub const IMR_MUART: u32 = (1 << UART_IRQ_NUM);
pub const IMR_MWDT: u32 = (1 << WDT_IRQ_NUM);
pub const IMR_MRTC: u32 = (1 << RTC_IRQ_NUM);
pub const IMR_MKB: u32 = (1 << KB_IRQ_NUM);
pub const IMR_MPWM: u32 = (1 << PWM_IRQ_NUM);
pub const IMR_MINT0: u32 = (1 << INT0_IRQ_NUM);
pub const IMR_MINT1: u32 = (1 << INT1_IRQ_NUM);
pub const IMR_MINT2: u32 = (1 << INT2_IRQ_NUM);
pub const IMR_MINT3: u32 = (1 << INT3_IRQ_NUM);
pub const IMR_MIRQ1: u32 = (1 << IRQ1_IRQ_NUM);
pub const IMR_MIRQ2: u32 = (1 << IRQ2_IRQ_NUM);
pub const IMR_MIRQ3: u32 = (1 << IRQ3_IRQ_NUM);
pub const IMR_MIRQ6: u32 = (1 << IRQ6_IRQ_NUM);
pub const IMR_MIRQ5: u32 = (1 << IRQ5_IRQ_NUM);
pub const IMR_MSAM: u32 = (1 << SAM_IRQ_NUM);
pub const IMR_MEMIQ: u32 = (1 << EMIQ_IRQ_NUM);

/* '328-compatible definitions */
pub const IMR_MSPIM: u32 = IMR_MSPI;
pub const IMR_MTMR1: u32 = IMR_MTMR;

/* 
 * Interrupt Status Register 
 */
pub const ISR_ADDR: u32 = 0xfffff30c;
pub unsafe fn ISR() -> &'static mut u32 { &mut *(ISR_ADDR as *mut u32) }

pub const ISR_SPI: u32 = (1 << SPI_IRQ_NUM);
pub const ISR_TMR: u32 = (1 << TMR_IRQ_NUM);
pub const ISR_UART: u32 = (1 << UART_IRQ_NUM);
pub const ISR_WDT: u32 = (1 << WDT_IRQ_NUM);
pub const ISR_RTC: u32 = (1 << RTC_IRQ_NUM);
pub const ISR_KB: u32 = (1 << KB_IRQ_NUM);
pub const ISR_PWM: u32 = (1 << PWM_IRQ_NUM);
pub const ISR_INT0: u32 = (1 << INT0_IRQ_NUM);
pub const ISR_INT1: u32 = (1 << INT1_IRQ_NUM);
pub const ISR_INT2: u32 = (1 << INT2_IRQ_NUM);
pub const ISR_INT3: u32 = (1 << INT3_IRQ_NUM);
pub const ISR_IRQ1: u32 = (1 << IRQ1_IRQ_NUM);
pub const ISR_IRQ2: u32 = (1 << IRQ2_IRQ_NUM);
pub const ISR_IRQ3: u32 = (1 << IRQ3_IRQ_NUM);
pub const ISR_IRQ6: u32 = (1 << IRQ6_IRQ_NUM);
pub const ISR_IRQ5: u32 = (1 << IRQ5_IRQ_NUM);
pub const ISR_SAM: u32 = (1 << SAM_IRQ_NUM);
pub const ISR_EMIQ: u32 = (1 << EMIQ_IRQ_NUM);

/* '328-compatible definitions */
pub const ISR_SPIM: u32 = ISR_SPI;
pub const ISR_TMR1: u32 = ISR_TMR;

/* 
 * Interrupt Pending Register 
 */
pub const IPR_ADDR: u32 = 0xfffff30c;
pub unsafe fn IPR() -> &'static mut u32 { &mut *(IPR_ADDR as *mut u32) }

pub const IPR_SPI: u32 = (1 << SPI_IRQ_NUM);
pub const IPR_TMR: u32 = (1 << TMR_IRQ_NUM);
pub const IPR_UART: u32 = (1 << UART_IRQ_NUM);
pub const IPR_WDT: u32 = (1 << WDT_IRQ_NUM);
pub const IPR_RTC: u32 = (1 << RTC_IRQ_NUM);
pub const IPR_KB: u32 = (1 << KB_IRQ_NUM);
pub const IPR_PWM: u32 = (1 << PWM_IRQ_NUM);
pub const IPR_INT0: u32 = (1 << INT0_IRQ_NUM);
pub const IPR_INT1: u32 = (1 << INT1_IRQ_NUM);
pub const IPR_INT2: u32 = (1 << INT2_IRQ_NUM);
pub const IPR_INT3: u32 = (1 << INT3_IRQ_NUM);
pub const IPR_IRQ1: u32 = (1 << IRQ1_IRQ_NUM);
pub const IPR_IRQ2: u32 = (1 << IRQ2_IRQ_NUM);
pub const IPR_IRQ3: u32 = (1 << IRQ3_IRQ_NUM);
pub const IPR_IRQ6: u32 = (1 << IRQ6_IRQ_NUM);
pub const IPR_IRQ5: u32 = (1 << IRQ5_IRQ_NUM);
pub const IPR_SAM: u32 = (1 << SAM_IRQ_NUM);
pub const IPR_EMIQ: u32 = (1 << EMIQ_IRQ_NUM);

/* '328-compatible definitions */
pub const IPR_SPIM: u32 = IPR_SPI;
pub const IPR_TMR1: u32 = IPR_TMR;

/**********
 *
 * 0xFFFFF4xx -- Parallel Ports
 *
 **********/

/*
 * Port A
 */
pub const PADIR_ADDR: u32 = 0xfffff400;
pub const PADATA_ADDR: u32 = 0xfffff401;
pub const PAPUEN_ADDR: u32 = 0xfffff402;

pub unsafe fn PADIR() -> &'static mut u8 { &mut *(PADIR_ADDR as *mut u8) }
pub unsafe fn PADATA() -> &'static mut u8 { &mut *(PADATA_ADDR as *mut u8) }
pub unsafe fn PAPUEN() -> &'static mut u8 { &mut *(PAPUEN_ADDR as *mut u8) }

pub const fn PA(x: u32) -> u32 { (1u32 << (x)) }

/* 
 * Port B
 */
pub const PBDIR_ADDR: u32 = 0xfffff408;
pub const PBDATA_ADDR: u32 = 0xfffff409;
pub const PBPUEN_ADDR: u32 = 0xfffff40a;
pub const PBSEL_ADDR: u32 = 0xfffff40b;

pub unsafe fn PBDIR() -> &'static mut u8 { &mut *(PBDIR_ADDR as *mut u8) }
pub unsafe fn PBDATA() -> &'static mut u8 { &mut *(PBDATA_ADDR as *mut u8) }
pub unsafe fn PBPUEN() -> &'static mut u8 { &mut *(PBPUEN_ADDR as *mut u8) }
pub unsafe fn PBSEL() -> &'static mut u8 { &mut *(PBSEL_ADDR as *mut u8) }

pub const fn PB(x: u32) -> u32 { (1u32 << (x)) }

pub const PB_CSB0: u32 = 0x01;
pub const PB_CSB1: u32 = 0x02;
pub const PB_CSC0_RAS0: u32 = 0x04;
pub const PB_CSC1_RAS1: u32 = 0x08;
pub const PB_CSD0_CAS0: u32 = 0x10;
pub const PB_CSD1_CAS1: u32 = 0x20;
pub const PB_TIN_TOUT: u32 = 0x40;
pub const PB_PWMO: u32 = 0x80;

/* 
 * Port C
 */
pub const PCDIR_ADDR: u32 = 0xfffff410;
pub const PCDATA_ADDR: u32 = 0xfffff411;
pub const PCPDEN_ADDR: u32 = 0xfffff412;
pub const PCSEL_ADDR: u32 = 0xfffff413;

pub unsafe fn PCDIR() -> &'static mut u8 { &mut *(PCDIR_ADDR as *mut u8) }
pub unsafe fn PCDATA() -> &'static mut u8 { &mut *(PCDATA_ADDR as *mut u8) }
pub unsafe fn PCPDEN() -> &'static mut u8 { &mut *(PCPDEN_ADDR as *mut u8) }
pub unsafe fn PCSEL() -> &'static mut u8 { &mut *(PCSEL_ADDR as *mut u8) }

pub const fn PC(x: u32) -> u32 { (1u32 << (x)) }

pub const PC_LD0: u32 = 0x01;
pub const PC_LD1: u32 = 0x02;
pub const PC_LD2: u32 = 0x04;
pub const PC_LD3: u32 = 0x08;
pub const PC_LFLM: u32 = 0x10;
pub const PC_LLP: u32 = 0x20;
pub const PC_LCLK: u32 = 0x40;
pub const PC_LACD: u32 = 0x80;

/* 
 * Port D
 */
pub const PDDIR_ADDR: u32 = 0xfffff418;
pub const PDDATA_ADDR: u32 = 0xfffff419;
pub const PDPUEN_ADDR: u32 = 0xfffff41a;
pub const PDSEL_ADDR: u32 = 0xfffff41b;
pub const PDPOL_ADDR: u32 = 0xfffff41c;
pub const PDIRQEN_ADDR: u32 = 0xfffff41d;
pub const PDKBEN_ADDR: u32 = 0xfffff41e;
pub const PDIQEG_ADDR: u32 = 0xfffff41f;

pub unsafe fn PDDIR() -> &'static mut u8 { &mut *(PDDIR_ADDR as *mut u8) }
pub unsafe fn PDDATA() -> &'static mut u8 { &mut *(PDDATA_ADDR as *mut u8) }
pub unsafe fn PDPUEN() -> &'static mut u8 { &mut *(PDPUEN_ADDR as *mut u8) }
pub unsafe fn PDSEL() -> &'static mut u8 { &mut *(PDSEL_ADDR as *mut u8) }
pub unsafe fn PDPOL() -> &'static mut u8 { &mut *(PDPOL_ADDR as *mut u8) }
pub unsafe fn PDIRQEN() -> &'static mut u8 { &mut *(PDIRQEN_ADDR as *mut u8) }
pub unsafe fn PDKBEN() -> &'static mut u8 { &mut *(PDKBEN_ADDR as *mut u8) }
pub unsafe fn PDIQEG() -> &'static mut u8 { &mut *(PDIQEG_ADDR as *mut u8) }

pub const fn PD(x: u32) -> u32 { (1u32 << (x)) }

pub const PD_INT0: u32 = 0x01;
pub const PD_INT1: u32 = 0x02;
pub const PD_INT2: u32 = 0x04;
pub const PD_INT3: u32 = 0x08;
pub const PD_IRQ1: u32 = 0x10;
pub const PD_IRQ2: u32 = 0x20;
pub const PD_IRQ3: u32 = 0x40;
pub const PD_IRQ6: u32 = 0x80;

/* 
 * Port E
 */
pub const PEDIR_ADDR: u32 = 0xfffff420;
pub const PEDATA_ADDR: u32 = 0xfffff421;
pub const PEPUEN_ADDR: u32 = 0xfffff422;
pub const PESEL_ADDR: u32 = 0xfffff423;

pub unsafe fn PEDIR() -> &'static mut u8 { &mut *(PEDIR_ADDR as *mut u8) }
pub unsafe fn PEDATA() -> &'static mut u8 { &mut *(PEDATA_ADDR as *mut u8) }
pub unsafe fn PEPUEN() -> &'static mut u8 { &mut *(PEPUEN_ADDR as *mut u8) }
pub unsafe fn PESEL() -> &'static mut u8 { &mut *(PESEL_ADDR as *mut u8) }

pub const fn PE(x: u32) -> u32 { (1u32 << (x)) }

pub const PE_SPMTXD: u32 = 0x01;
pub const PE_SPMRXD: u32 = 0x02;
pub const PE_SPMCLK: u32 = 0x04;
pub const PE_DWE: u32 = 0x08;
pub const PE_RXD: u32 = 0x10;
pub const PE_TXD: u32 = 0x20;
pub const PE_RTS: u32 = 0x40;
pub const PE_CTS: u32 = 0x80;

/* 
 * Port F
 */
pub const PFDIR_ADDR: u32 = 0xfffff428;
pub const PFDATA_ADDR: u32 = 0xfffff429;
pub const PFPUEN_ADDR: u32 = 0xfffff42a;
pub const PFSEL_ADDR: u32 = 0xfffff42b;

pub unsafe fn PFDIR() -> &'static mut u8 { &mut *(PFDIR_ADDR as *mut u8) }
pub unsafe fn PFDATA() -> &'static mut u8 { &mut *(PFDATA_ADDR as *mut u8) }
pub unsafe fn PFPUEN() -> &'static mut u8 { &mut *(PFPUEN_ADDR as *mut u8) }
pub unsafe fn PFSEL() -> &'static mut u8 { &mut *(PFSEL_ADDR as *mut u8) }

pub const fn PF(x: u32) -> u32 { (1u32 << (x)) }

pub const PF_LCONTRAST: u32 = 0x01;
pub const PF_IRQ5: u32 = 0x02;
pub const PF_CLKO: u32 = 0x04;
pub const PF_A20: u32 = 0x08;
pub const PF_A21: u32 = 0x10;
pub const PF_A22: u32 = 0x20;
pub const PF_A23: u32 = 0x40;
pub const PF_CSA1: u32 = 0x80;

/* 
 * Port G
 */
pub const PGDIR_ADDR: u32 = 0xfffff430;
pub const PGDATA_ADDR: u32 = 0xfffff431;
pub const PGPUEN_ADDR: u32 = 0xfffff432;
pub const PGSEL_ADDR: u32 = 0xfffff433;

pub unsafe fn PGDIR() -> &'static mut u8 { &mut *(PGDIR_ADDR as *mut u8) }
pub unsafe fn PGDATA() -> &'static mut u8 { &mut *(PGDATA_ADDR as *mut u8) }
pub unsafe fn PGPUEN() -> &'static mut u8 { &mut *(PGPUEN_ADDR as *mut u8) }
pub unsafe fn PGSEL() -> &'static mut u8 { &mut *(PGSEL_ADDR as *mut u8) }

pub const fn PG(x: u32) -> u32 { (1u32 << (x)) }

pub const PG_BUSW_DTACK: u32 = 0x01;
pub const PG_A0: u32 = 0x02;
pub const PG_EMUIRQ: u32 = 0x04;
pub const PG_HIZ_P_D: u32 = 0x08;
pub const PG_EMUCS: u32 = 0x10;
pub const PG_EMUBRK: u32 = 0x20;

/* 
 * Port J
 */
pub const PJDIR_ADDR: u32 = 0xfffff438;
pub const PJDATA_ADDR: u32 = 0xfffff439;
pub const PJPUEN_ADDR: u32 = 0xfffff43A;
pub const PJSEL_ADDR: u32 = 0xfffff43B;

pub unsafe fn PJDIR() -> &'static mut u8 { &mut *(PJDIR_ADDR as *mut u8) }
pub unsafe fn PJDATA() -> &'static mut u8 { &mut *(PJDATA_ADDR as *mut u8) }
pub unsafe fn PJPUEN() -> &'static mut u8 { &mut *(PJPUEN_ADDR as *mut u8) }
pub unsafe fn PJSEL() -> &'static mut u8 { &mut *(PJSEL_ADDR as *mut u8) }

pub const fn PJ(x: u32) -> u32 { (1u32 << (x)) }

/*
 * Port K
 */
pub const PKDIR_ADDR: u32 = 0xfffff440;
pub const PKDATA_ADDR: u32 = 0xfffff441;
pub const PKPUEN_ADDR: u32 = 0xfffff442;
pub const PKSEL_ADDR: u32 = 0xfffff443;

pub unsafe fn PKDIR() -> &'static mut u8 { &mut *(PKDIR_ADDR as *mut u8) }
pub unsafe fn PKDATA() -> &'static mut u8 { &mut *(PKDATA_ADDR as *mut u8) }
pub unsafe fn PKPUEN() -> &'static mut u8 { &mut *(PKPUEN_ADDR as *mut u8) }
pub unsafe fn PKSEL() -> &'static mut u8 { &mut *(PKSEL_ADDR as *mut u8) }

pub const fn PK(x: u32) -> u32 { (1u32 << (x)) }

pub const PK_DATAREADY: u32 = 0x01;
pub const PK_PWM2: u32 = 0x01;
pub const PK_R_W: u32 = 0x02;
pub const PK_LDS: u32 = 0x04;
pub const PK_UDS: u32 = 0x08;
pub const PK_LD4: u32 = 0x10;
pub const PK_LD5: u32 = 0x20;
pub const PK_LD6: u32 = 0x40;
pub const PK_LD7: u32 = 0x80;

pub const PJDIR_ADDR: u32 = 0xfffff438;
pub const PJDATA_ADDR: u32 = 0xfffff439;
pub const PJPUEN_ADDR: u32 = 0xfffff43A;
pub const PJSEL_ADDR: u32 = 0xfffff43B;

pub unsafe fn PJDIR() -> &'static mut u8 { &mut *(PJDIR_ADDR as *mut u8) }
pub unsafe fn PJDATA() -> &'static mut u8 { &mut *(PJDATA_ADDR as *mut u8) }
pub unsafe fn PJPUEN() -> &'static mut u8 { &mut *(PJPUEN_ADDR as *mut u8) }
pub unsafe fn PJSEL() -> &'static mut u8 { &mut *(PJSEL_ADDR as *mut u8) }

pub const fn PJ(x: u32) -> u32 { (1u32 << (x)) }

pub const PJ_MOSI: u32 = 0x01;
pub const PJ_MISO: u32 = 0x02;
pub const PJ_SPICLK1: u32 = 0x04;
pub const PJ_SS: u32 = 0x08;
pub const PJ_RXD2: u32 = 0x10;
pub const PJ_TXD2: u32 = 0x20;
pub const PJ_RTS2: u32 = 0x40;
pub const PJ_CTS2: u32 = 0x80;

/*
 * Port M
 */
pub const PMDIR_ADDR: u32 = 0xfffff448;
pub const PMDATA_ADDR: u32 = 0xfffff449;
pub const PMPUEN_ADDR: u32 = 0xfffff44a;
pub const PMSEL_ADDR: u32 = 0xfffff44b;

pub unsafe fn PMDIR() -> &'static mut u8 { &mut *(PMDIR_ADDR as *mut u8) }
pub unsafe fn PMDATA() -> &'static mut u8 { &mut *(PMDATA_ADDR as *mut u8) }
pub unsafe fn PMPUEN() -> &'static mut u8 { &mut *(PMPUEN_ADDR as *mut u8) }
pub unsafe fn PMSEL() -> &'static mut u8 { &mut *(PMSEL_ADDR as *mut u8) }

pub const fn PM(x: u32) -> u32 { (1u32 << (x)) }

pub const PM_SDCLK: u32 = 0x01;
pub const PM_SDCE: u32 = 0x02;
pub const PM_DQMH: u32 = 0x04;
pub const PM_DQML: u32 = 0x08;
pub const PM_SDA10: u32 = 0x10;
pub const PM_DMOE: u32 = 0x20;

/**********
 *
 * 0xFFFFF5xx -- Pulse-Width Modulator (PWM)
 *
 **********/

/*
 * PWM Control Register
 */
pub const PWMC_ADDR: u32 = 0xfffff500;
pub unsafe fn PWMC() -> &'static mut u16 { &mut *(PWMC_ADDR as *mut u16) }

pub const PWMC_CLKSEL_MASK: u32 = 0x0003;
pub const PWMC_CLKSEL_SHIFT: u32 = 0;
pub const PWMC_REPEAT_MASK: u32 = 0x000c;
pub const PWMC_REPEAT_SHIFT: u32 = 2;
pub const PWMC_EN: u32 = 0x0010;
pub const PMNC_FIFOAV: u32 = 0x0020;
pub const PWMC_IRQEN: u32 = 0x0040;
pub const PWMC_IRQ: u32 = 0x0080;
pub const PWMC_PRESCALER_MASK: u32 = 0x7f00;
pub const PWMC_PRESCALER_SHIFT: u32 = 8;
pub const PWMC_CLKSRC: u32 = 0x8000;

/* '328-compatible definitions */
pub const PWMC_PWMEN: u32 = PWMC_EN;

/*
 * PWM Sample Register 
 */
pub const PWMS_ADDR: u32 = 0xfffff502;
pub unsafe fn PWMS() -> &'static mut u16 { &mut *(PWMS_ADDR as *mut u16) }

/*
 * PWM Period Register
 */
pub const PWMP_ADDR: u32 = 0xfffff504;
pub unsafe fn PWMP() -> &'static mut u8 { &mut *(PWMP_ADDR as *mut u8) }

/*
 * PWM Counter Register
 */
pub const PWMCNT_ADDR: u32 = 0xfffff505;
pub unsafe fn PWMCNT() -> &'static mut u8 { &mut *(PWMCNT_ADDR as *mut u8) }

/**********
 *
 * 0xFFFFF6xx -- General-Purpose Timer
 *
 **********/

/* 
 * Timer Control register
 */
pub const TCTL_ADDR: u32 = 0xfffff600;
pub unsafe fn TCTL() -> &'static mut u16 { &mut *(TCTL_ADDR as *mut u16) }

pub const TCTL_TEN: u32 = 0x0001;
pub const TCTL_CLKSOURCE_MASK: u32 = 0x000e;
pub const TCTL_CLKSOURCE_STOP: u32 = 0x0000;
pub const TCTL_CLKSOURCE_SYSCLK: u32 = 0x0002;
pub const TCTL_CLKSOURCE_SYSCLK_16: u32 = 0x0004;
pub const TCTL_CLKSOURCE_TIN: u32 = 0x0006;
pub const TCTL_CLKSOURCE_32KHZ: u32 = 0x0008;
pub const TCTL_IRQEN: u32 = 0x0010;
pub const TCTL_OM: u32 = 0x0020;
pub const TCTL_CAP_MASK: u32 = 0x00c0;
pub const TCTL_CAP_RE: u32 = 0x0040;
pub const TCTL_CAP_FE: u32 = 0x0080;
pub const TCTL_FRR: u32 = 0x0010;

/* '328-compatible definitions */
pub const TCTL1_ADDR: u32 = TCTL_ADDR;
pub const TCTL1: u32 = TCTL;

/*
 * Timer Prescaler Register
 */
pub const TPRER_ADDR: u32 = 0xfffff602;
pub unsafe fn TPRER() -> &'static mut u16 { &mut *(TPRER_ADDR as *mut u16) }

/* '328-compatible definitions */
pub const TPRER1_ADDR: u32 = TPRER_ADDR;
pub const TPRER1: u32 = TPRER;

/*
 * Timer Compare Register
 */
pub const TCMP_ADDR: u32 = 0xfffff604;
pub unsafe fn TCMP() -> &'static mut u16 { &mut *(TCMP_ADDR as *mut u16) }

/* '328-compatible definitions */
pub const TCMP1_ADDR: u32 = TCMP_ADDR;
pub const TCMP1: u32 = TCMP;

/*
 * Timer Capture register
 */
pub const TCR_ADDR: u32 = 0xfffff606;
pub unsafe fn TCR() -> &'static mut u16 { &mut *(TCR_ADDR as *mut u16) }

/* '328-compatible definitions */
pub const TCR1_ADDR: u32 = TCR_ADDR;
pub const TCR1: u32 = TCR;

/*
 * Timer Counter Register
 */
pub const TCN_ADDR: u32 = 0xfffff608;
pub unsafe fn TCN() -> &'static mut u16 { &mut *(TCN_ADDR as *mut u16) }

/* '328-compatible definitions */
pub const TCN1_ADDR: u32 = TCN_ADDR;
pub const TCN1: u32 = TCN;

/*
 * Timer Status Register
 */
pub const TSTAT_ADDR: u32 = 0xfffff60a;
pub unsafe fn TSTAT() -> &'static mut u16 { &mut *(TSTAT_ADDR as *mut u16) }

pub const TSTAT_COMP: u32 = 0x0001;
pub const TSTAT_CAPT: u32 = 0x0001;

/* '328-compatible definitions */
pub const TSTAT1_ADDR: u32 = TSTAT_ADDR;
pub const TSTAT1: u32 = TSTAT;

/**********
 *
 * 0xFFFFF8xx -- Serial Peripheral Interface Master (SPIM)
 *
 **********/

/*
 * SPIM Data Register
 */
pub const SPIMDATA_ADDR: u32 = 0xfffff800;
pub unsafe fn SPIMDATA() -> &'static mut u16 { &mut *(SPIMDATA_ADDR as *mut u16) }

/*
 * SPIM Control/Status Register
 */
pub const SPIMCONT_ADDR: u32 = 0xfffff802;
pub unsafe fn SPIMCONT() -> &'static mut u16 { &mut *(SPIMCONT_ADDR as *mut u16) }

pub const SPIMCONT_BIT_COUNT_MASK: u32 = 0x000f;
pub const SPIMCONT_BIT_COUNT_SHIFT: u32 = 0;
pub const SPIMCONT_POL: u32 = 0x0010;
pub const SPIMCONT_PHA: u32 = 0x0020;
pub const SPIMCONT_IRQEN: u32 = 0x0040;
pub const SPIMCONT_IRQ: u32 = 0x0080;
pub const SPIMCONT_XCH: u32 = 0x0100;
pub const SPIMCONT_ENABLE: u32 = 0x0200;
pub const SPIMCONT_DATA_RATE_MASK: u32 = 0xe000;
pub const SPIMCONT_DATA_RATE_SHIFT: u32 = 13;

/* '328-compatible definitions */
pub const SPIMCONT_SPIMIRQ: u32 = SPIMCONT_IRQ;
pub const SPIMCONT_SPIMEN: u32 = SPIMCONT_ENABLE;

/**********
 *
 * 0xFFFFF9xx -- UART
 *
 **********/

/*
 * UART Status/Control Register
 */

pub const USTCNT_ADDR: u32 = 0xfffff900;
pub unsafe fn USTCNT() -> &'static mut u16 { &mut *(USTCNT_ADDR as *mut u16) }

pub const USTCNT_TXAE: u32 = 0x0001;
pub const USTCNT_TXHE: u32 = 0x0002;
pub const USTCNT_TXEE: u32 = 0x0004;
pub const USTCNT_RXRE: u32 = 0x0008;
pub const USTCNT_RXHE: u32 = 0x0010;
pub const USTCNT_RXFE: u32 = 0x0020;
pub const USTCNT_CTSD: u32 = 0x0040;
pub const USTCNT_ODEN: u32 = 0x0080;
pub const USTCNT_8_7: u32 = 0x0100;
pub const USTCNT_STOP: u32 = 0x0200;
pub const USTCNT_ODD: u32 = 0x0400;
pub const USTCNT_PEN: u32 = 0x0800;
pub const USTCNT_CLKM: u32 = 0x1000;
pub const USTCNT_TXEN: u32 = 0x2000;
pub const USTCNT_RXEN: u32 = 0x4000;
pub const USTCNT_UEN: u32 = 0x8000;

/* '328-compatible definitions */
pub const USTCNT_TXAVAILEN: u32 = USTCNT_TXAE;
pub const USTCNT_TXHALFEN: u32 = USTCNT_TXHE;
pub const USTCNT_TXEMPTYEN: u32 = USTCNT_TXEE;
pub const USTCNT_RXREADYEN: u32 = USTCNT_RXRE;
pub const USTCNT_RXHALFEN: u32 = USTCNT_RXHE;
pub const USTCNT_RXFULLEN: u32 = USTCNT_RXFE;
pub const USTCNT_CTSDELTAEN: u32 = USTCNT_CTSD;
pub const USTCNT_ODD_EVEN: u32 = USTCNT_ODD;
pub const USTCNT_PARITYEN: u32 = USTCNT_PEN;
pub const USTCNT_CLKMODE: u32 = USTCNT_CLKM;
pub const USTCNT_UARTEN: u32 = USTCNT_UEN;

/*
 * UART Baud Control Register
 */
pub const UBAUD_ADDR: u32 = 0xfffff902;
pub unsafe fn UBAUD() -> &'static mut u16 { &mut *(UBAUD_ADDR as *mut u16) }

pub const UBAUD_PRESCALER_MASK: u32 = 0x003f;
pub const UBAUD_PRESCALER_SHIFT: u32 = 0;
pub const UBAUD_DIVIDE_MASK: u32 = 0x0700;
pub const UBAUD_DIVIDE_SHIFT: u32 = 8;
pub const UBAUD_BAUD_SRC: u32 = 0x0800;
pub const UBAUD_UCLKDIR: u32 = 0x2000;

/*
 * UART Receiver Register 
 */
pub const URX_ADDR: u32 = 0xfffff904;
pub unsafe fn URX() -> &'static mut u16 { &mut *(URX_ADDR as *mut u16) }

pub const URX_RXDATA_ADDR: u32 = 0xfffff905;
pub unsafe fn URX_RXDATA() -> &'static mut u8 { &mut *(URX_RXDATA_ADDR as *mut u8) }

pub const URX_RXDATA_MASK: u32 = 0x00ff;
pub const URX_RXDATA_SHIFT: u32 = 0;
pub const URX_PARITY_ERROR: u32 = 0x0100;
pub const URX_BREAK: u32 = 0x0200;
pub const URX_FRAME_ERROR: u32 = 0x0400;
pub const URX_OVRUN: u32 = 0x0800;
pub const URX_OLD_DATA: u32 = 0x1000;
pub const URX_DATA_READY: u32 = 0x2000;
pub const URX_FIFO_HALF: u32 = 0x4000;
pub const URX_FIFO_FULL: u32 = 0x8000;

/*
 * UART Transmitter Register 
 */
pub const UTX_ADDR: u32 = 0xfffff906;
pub unsafe fn UTX() -> &'static mut u16 { &mut *(UTX_ADDR as *mut u16) }

pub const UTX_TXDATA_ADDR: u32 = 0xfffff907;
pub unsafe fn UTX_TXDATA() -> &'static mut u8 { &mut *(UTX_TXDATA_ADDR as *mut u8) }

pub const UTX_TXDATA_MASK: u32 = 0x00ff;
pub const UTX_TXDATA_SHIFT: u32 = 0;
pub const UTX_CTS_DELTA: u32 = 0x0100;
pub const UTX_CTS_STAT: u32 = 0x0200;
pub const UTX_BUSY: u32 = 0x0400;
pub const UTX_NOCTS: u32 = 0x0800;
pub const UTX_SEND_BREAK: u32 = 0x1000;
pub const UTX_TX_AVAIL: u32 = 0x2000;
pub const UTX_FIFO_HALF: u32 = 0x4000;
pub const UTX_FIFO_EMPTY: u32 = 0x8000;

/* '328-compatible definitions */
pub const UTX_CTS_STATUS: u32 = UTX_CTS_STAT;
pub const UTX_IGNORE_CTS: u32 = UTX_NOCTS;

/*
 * UART Miscellaneous Register 
 */
pub const UMISC_ADDR: u32 = 0xfffff908;
pub unsafe fn UMISC() -> &'static mut u16 { &mut *(UMISC_ADDR as *mut u16) }

pub const UMISC_TX_POL: u32 = 0x0004;
pub const UMISC_RX_POL: u32 = 0x0008;
pub const UMISC_IRDA_LOOP: u32 = 0x0010;
pub const UMISC_IRDA_EN: u32 = 0x0020;
pub const UMISC_RTS: u32 = 0x0040;
pub const UMISC_RTSCONT: u32 = 0x0080;
pub const UMISC_IR_TEST: u32 = 0x0400;
pub const UMISC_BAUD_RESET: u32 = 0x0800;
pub const UMISC_LOOP: u32 = 0x1000;
pub const UMISC_FORCE_PERR: u32 = 0x2000;
pub const UMISC_CLKSRC: u32 = 0x4000;
pub const UMISC_BAUD_TEST: u32 = 0x8000;

/* 
 * UART Non-integer Prescaler Register
 */
pub const NIPR_ADDR: u32 = 0xfffff90a;
pub unsafe fn NIPR() -> &'static mut u16 { &mut *(NIPR_ADDR as *mut u16) }

pub const NIPR_STEP_VALUE_MASK: u32 = 0x00ff;
pub const NIPR_STEP_VALUE_SHIFT: u32 = 0;
pub const NIPR_SELECT_MASK: u32 = 0x0700;
pub const NIPR_SELECT_SHIFT: u32 = 8;
pub const NIPR_PRE_SEL: u32 = 0x8000;


/* generalization of uart control registers to support multiple ports: */
#[repr(C, packed)]
pub struct m68328_uart {
    pub ustcnt: u16,
    pub ubaud: u16,
    pub urx: UartRegister,
    pub utx: UartRegister,
    pub umisc: u16,
    pub nipr: u16,
    pub hmark: u16,
    pub unused: u16,
}
#[repr(C)]
pub union UartRegister { pub w: u16, pub b: UartBytes }
#[repr(C)]
pub struct UartBytes { pub status: u8, pub rxdata: u8 }




/**********
 *
 * 0xFFFFFAxx -- LCD Controller
 *
 **********/

/*
 * LCD Screen Starting Address Register 
 */
pub const LSSA_ADDR: u32 = 0xfffffa00;
pub unsafe fn LSSA() -> &'static mut u32 { &mut *(LSSA_ADDR as *mut u32) }

pub const LSSA_SSA_MASK: u32 = 0x1ffffffe;

/*
 * LCD Virtual Page Width Register 
 */
pub const LVPW_ADDR: u32 = 0xfffffa05;
pub unsafe fn LVPW() -> &'static mut u8 { &mut *(LVPW_ADDR as *mut u8) }

/*
 * LCD Screen Width Register (not compatible with '328 !!!) 
 */
pub const LXMAX_ADDR: u32 = 0xfffffa08;
pub unsafe fn LXMAX() -> &'static mut u16 { &mut *(LXMAX_ADDR as *mut u16) }

pub const LXMAX_XM_MASK: u32 = 0x02f0;

/*
 * LCD Screen Height Register
 */
pub const LYMAX_ADDR: u32 = 0xfffffa0a;
pub unsafe fn LYMAX() -> &'static mut u16 { &mut *(LYMAX_ADDR as *mut u16) }

pub const LYMAX_YM_MASK: u32 = 0x01ff;

/*
 * LCD Cursor X Position Register
 */
pub const LCXP_ADDR: u32 = 0xfffffa18;
pub unsafe fn LCXP() -> &'static mut u16 { &mut *(LCXP_ADDR as *mut u16) }

pub const LCXP_CC_MASK: u32 = 0xc000;
pub const LCXP_CC_TRAMSPARENT: u32 = 0x0000;
pub const LCXP_CC_BLACK: u32 = 0x4000;
pub const LCXP_CC_REVERSED: u32 = 0x8000;
pub const LCXP_CC_WHITE: u32 = 0xc000;
pub const LCXP_CXP_MASK: u32 = 0x02ff;

/*
 * LCD Cursor Y Position Register
 */
pub const LCYP_ADDR: u32 = 0xfffffa1a;
pub unsafe fn LCYP() -> &'static mut u16 { &mut *(LCYP_ADDR as *mut u16) }

pub const LCYP_CYP_MASK: u32 = 0x01ff;

/*
 * LCD Cursor Width and Heigth Register
 */
pub const LCWCH_ADDR: u32 = 0xfffffa1c;
pub unsafe fn LCWCH() -> &'static mut u16 { &mut *(LCWCH_ADDR as *mut u16) }

pub const LCWCH_CH_MASK: u32 = 0x001f;
pub const LCWCH_CH_SHIFT: u32 = 0;
pub const LCWCH_CW_MASK: u32 = 0x1f00;
pub const LCWCH_CW_SHIFT: u32 = 8;

/*
 * LCD Blink Control Register
 */
pub const LBLKC_ADDR: u32 = 0xfffffa1f;
pub unsafe fn LBLKC() -> &'static mut u8 { &mut *(LBLKC_ADDR as *mut u8) }

pub const LBLKC_BD_MASK: u32 = 0x7f;
pub const LBLKC_BD_SHIFT: u32 = 0;
pub const LBLKC_BKEN: u32 = 0x80;

/*
 * LCD Panel Interface Configuration Register 
 */
pub const LPICF_ADDR: u32 = 0xfffffa20;
pub unsafe fn LPICF() -> &'static mut u8 { &mut *(LPICF_ADDR as *mut u8) }

pub const LPICF_GS_MASK: u32 = 0x03;
pub const LPICF_GS_BW: u32 = 0x00;
pub const LPICF_GS_GRAY_4: u32 = 0x01;
pub const LPICF_GS_GRAY_16: u32 = 0x02;
pub const LPICF_PBSIZ_MASK: u32 = 0x0c;
pub const LPICF_PBSIZ_1: u32 = 0x00;
pub const LPICF_PBSIZ_2: u32 = 0x04;
pub const LPICF_PBSIZ_4: u32 = 0x08;

/*
 * LCD Polarity Configuration Register 
 */
pub const LPOLCF_ADDR: u32 = 0xfffffa21;
pub unsafe fn LPOLCF() -> &'static mut u8 { &mut *(LPOLCF_ADDR as *mut u8) }

pub const LPOLCF_PIXPOL: u32 = 0x01;
pub const LPOLCF_LPPOL: u32 = 0x02;
pub const LPOLCF_FLMPOL: u32 = 0x04;
pub const LPOLCF_LCKPOL: u32 = 0x08;

/*
 * LACD (LCD Alternate Crystal Direction) Rate Control Register
 */
pub const LACDRC_ADDR: u32 = 0xfffffa23;
pub unsafe fn LACDRC() -> &'static mut u8 { &mut *(LACDRC_ADDR as *mut u8) }

pub const LACDRC_ACDSLT: u32 = 0x80;
pub const LACDRC_ACD_MASK: u32 = 0x0f;
pub const LACDRC_ACD_SHIFT: u32 = 0;

/*
 * LCD Pixel Clock Divider Register
 */
pub const LPXCD_ADDR: u32 = 0xfffffa25;
pub unsafe fn LPXCD() -> &'static mut u8 { &mut *(LPXCD_ADDR as *mut u8) }

pub const LPXCD_PCD_MASK: u32 = 0x3f;
pub const LPXCD_PCD_SHIFT: u32 = 0;

/*
 * LCD Clocking Control Register
 */
pub const LCKCON_ADDR: u32 = 0xfffffa27;
pub unsafe fn LCKCON() -> &'static mut u8 { &mut *(LCKCON_ADDR as *mut u8) }

pub const LCKCON_DWS_MASK: u32 = 0x0f;
pub const LCKCON_DWS_SHIFT: u32 = 0;
pub const LCKCON_DWIDTH: u32 = 0x40;
pub const LCKCON_LCDON: u32 = 0x80;

/* '328-compatible definitions */
pub const LCKCON_DW_MASK: u32 = LCKCON_DWS_MASK;
pub const LCKCON_DW_SHIFT: u32 = LCKCON_DWS_SHIFT;
 
/*
 * LCD Refresh Rate Adjustment Register 
 */
pub const LRRA_ADDR: u32 = 0xfffffa29;
pub unsafe fn LRRA() -> &'static mut u8 { &mut *(LRRA_ADDR as *mut u8) }

/*
 * LCD Panning Offset Register
 */
pub const LPOSR_ADDR: u32 = 0xfffffa2d;
pub unsafe fn LPOSR() -> &'static mut u8 { &mut *(LPOSR_ADDR as *mut u8) }

pub const LPOSR_POS_MASK: u32 = 0x0f;
pub const LPOSR_POS_SHIFT: u32 = 0;

/*
 * LCD Frame Rate Control Modulation Register
 */
pub const LFRCM_ADDR: u32 = 0xfffffa31;
pub unsafe fn LFRCM() -> &'static mut u8 { &mut *(LFRCM_ADDR as *mut u8) }

pub const LFRCM_YMOD_MASK: u32 = 0x0f;
pub const LFRCM_YMOD_SHIFT: u32 = 0;
pub const LFRCM_XMOD_MASK: u32 = 0xf0;
pub const LFRCM_XMOD_SHIFT: u32 = 4;

/*
 * LCD Gray Palette Mapping Register
 */
pub const LGPMR_ADDR: u32 = 0xfffffa33;
pub unsafe fn LGPMR() -> &'static mut u8 { &mut *(LGPMR_ADDR as *mut u8) }

pub const LGPMR_G1_MASK: u32 = 0x0f;
pub const LGPMR_G1_SHIFT: u32 = 0;
pub const LGPMR_G2_MASK: u32 = 0xf0;
pub const LGPMR_G2_SHIFT: u32 = 4;

/* 
 * PWM Contrast Control Register
 */
pub const PWMR_ADDR: u32 = 0xfffffa36;
pub unsafe fn PWMR() -> &'static mut u16 { &mut *(PWMR_ADDR as *mut u16) }

pub const PWMR_PW_MASK: u32 = 0x00ff;
pub const PWMR_PW_SHIFT: u32 = 0;
pub const PWMR_CCPEN: u32 = 0x0100;
pub const PWMR_SRC_MASK: u32 = 0x0600;
pub const PWMR_SRC_LINE: u32 = 0x0000;
pub const PWMR_SRC_PIXEL: u32 = 0x0200;
pub const PWMR_SRC_LCD: u32 = 0x4000;

/**********
 *
 * 0xFFFFFBxx -- Real-Time Clock (RTC)
 *
 **********/

/*
 * RTC Hours Minutes and Seconds Register
 */
pub const RTCTIME_ADDR: u32 = 0xfffffb00;
pub unsafe fn RTCTIME() -> &'static mut u32 { &mut *(RTCTIME_ADDR as *mut u32) }

pub const RTCTIME_SECONDS_MASK: u32 = 0x0000003f;
pub const RTCTIME_SECONDS_SHIFT: u32 = 0;
pub const RTCTIME_MINUTES_MASK: u32 = 0x003f0000;
pub const RTCTIME_MINUTES_SHIFT: u32 = 16;
pub const RTCTIME_HOURS_MASK: u32 = 0x1f000000;
pub const RTCTIME_HOURS_SHIFT: u32 = 24;

/*
 *  RTC Alarm Register 
 */
pub const RTCALRM_ADDR: u32 = 0xfffffb04;
pub unsafe fn RTCALRM() -> &'static mut u32 { &mut *(RTCALRM_ADDR as *mut u32) }

pub const RTCALRM_SECONDS_MASK: u32 = 0x0000003f;
pub const RTCALRM_SECONDS_SHIFT: u32 = 0;
pub const RTCALRM_MINUTES_MASK: u32 = 0x003f0000;
pub const RTCALRM_MINUTES_SHIFT: u32 = 16;
pub const RTCALRM_HOURS_MASK: u32 = 0x1f000000;
pub const RTCALRM_HOURS_SHIFT: u32 = 24;

/*
 * Watchdog Timer Register 
 */
pub const WATCHDOG_ADDR: u32 = 0xfffffb0a;
pub unsafe fn WATCHDOG() -> &'static mut u16 { &mut *(WATCHDOG_ADDR as *mut u16) }

pub const WATCHDOG_EN: u32 = 0x0001;
pub const WATCHDOG_ISEL: u32 = 0x0002;
pub const WATCHDOG_INTF: u32 = 0x0080;
pub const WATCHDOG_CNT_MASK: u32 = 0x0300;
pub const WATCHDOG_CNT_SHIFT: u32 = 8;

/*
 * RTC Control Register
 */
pub const RTCCTL_ADDR: u32 = 0xfffffb0c;
pub unsafe fn RTCCTL() -> &'static mut u16 { &mut *(RTCCTL_ADDR as *mut u16) }

pub const RTCCTL_XTL: u32 = 0x0020;
pub const RTCCTL_EN: u32 = 0x0080;

/* '328-compatible definitions */
pub const RTCCTL_384: u32 = RTCCTL_XTL;
pub const RTCCTL_ENABLE: u32 = RTCCTL_EN;

/*
 * RTC Interrupt Status Register 
 */
pub const RTCISR_ADDR: u32 = 0xfffffb0e;
pub unsafe fn RTCISR() -> &'static mut u16 { &mut *(RTCISR_ADDR as *mut u16) }

pub const RTCISR_SW: u32 = 0x0001;
pub const RTCISR_MIN: u32 = 0x0002;
pub const RTCISR_ALM: u32 = 0x0004;
pub const RTCISR_DAY: u32 = 0x0008;
pub const RTCISR_1HZ: u32 = 0x0010;
pub const RTCISR_HR: u32 = 0x0020;
pub const RTCISR_SAM0: u32 = 0x0100;
pub const RTCISR_SAM1: u32 = 0x0200;
pub const RTCISR_SAM2: u32 = 0x0400;
pub const RTCISR_SAM3: u32 = 0x0800;
pub const RTCISR_SAM4: u32 = 0x1000;
pub const RTCISR_SAM5: u32 = 0x2000;
pub const RTCISR_SAM6: u32 = 0x4000;
pub const RTCISR_SAM7: u32 = 0x8000;

/*
 * RTC Interrupt Enable Register
 */
pub const RTCIENR_ADDR: u32 = 0xfffffb10;
pub unsafe fn RTCIENR() -> &'static mut u16 { &mut *(RTCIENR_ADDR as *mut u16) }

pub const RTCIENR_SW: u32 = 0x0001;
pub const RTCIENR_MIN: u32 = 0x0002;
pub const RTCIENR_ALM: u32 = 0x0004;
pub const RTCIENR_DAY: u32 = 0x0008;
pub const RTCIENR_1HZ: u32 = 0x0010;
pub const RTCIENR_HR: u32 = 0x0020;
pub const RTCIENR_SAM0: u32 = 0x0100;
pub const RTCIENR_SAM1: u32 = 0x0200;
pub const RTCIENR_SAM2: u32 = 0x0400;
pub const RTCIENR_SAM3: u32 = 0x0800;
pub const RTCIENR_SAM4: u32 = 0x1000;
pub const RTCIENR_SAM5: u32 = 0x2000;
pub const RTCIENR_SAM6: u32 = 0x4000;
pub const RTCIENR_SAM7: u32 = 0x8000;

/* 
 * Stopwatch Minutes Register
 */
pub const STPWCH_ADDR: u32 = 0xfffffb12;
pub unsafe fn STPWCH() -> &'static mut u16 { &mut *(STPWCH_ADDR as *mut u16) }

pub const STPWCH_CNT_MASK: u32 = 0x003f;
pub const SPTWCH_CNT_SHIFT: u32 = 0;

/*
 * RTC Day Count Register 
 */
pub const DAYR_ADDR: u32 = 0xfffffb1a;
pub unsafe fn DAYR() -> &'static mut u16 { &mut *(DAYR_ADDR as *mut u16) }

pub const DAYR_DAYS_MASK: u32 = 0x1ff;
pub const DAYR_DAYS_SHIFT: u32 = 0;

/*
 * RTC Day Alarm Register 
 */
pub const DAYALARM_ADDR: u32 = 0xfffffb1c;
pub unsafe fn DAYALARM() -> &'static mut u16 { &mut *(DAYALARM_ADDR as *mut u16) }

pub const DAYALARM_DAYSAL_MASK: u32 = 0x01ff;
pub const DAYALARM_DAYSAL_SHIFT: u32 = 0;

/**********
 *
 * 0xFFFFFCxx -- DRAM Controller
 *
 **********/

/*
 * DRAM Memory Configuration Register 
 */
pub const DRAMMC_ADDR: u32 = 0xfffffc00;
pub unsafe fn DRAMMC() -> &'static mut u16 { &mut *(DRAMMC_ADDR as *mut u16) }

pub const DRAMMC_ROW12_MASK: u32 = 0xc000;
pub const DRAMMC_ROW12_PA10: u32 = 0x0000;
pub const DRAMMC_ROW12_PA21: u32 = 0x4000;
pub const DRAMMC_ROW12_PA23: u32 = 0x8000;
pub const DRAMMC_ROW0_MASK: u32 = 0x3000;
pub const DRAMMC_ROW0_PA11: u32 = 0x0000;
pub const DRAMMC_ROW0_PA22: u32 = 0x1000;
pub const DRAMMC_ROW0_PA23: u32 = 0x2000;
pub const DRAMMC_ROW11: u32 = 0x0800;
pub const DRAMMC_ROW10: u32 = 0x0400;
pub const DRAMMC_ROW9: u32 = 0x0200;
pub const DRAMMC_ROW8: u32 = 0x0100;
pub const DRAMMC_COL10: u32 = 0x0080;
pub const DRAMMC_COL9: u32 = 0x0040;
pub const DRAMMC_COL8: u32 = 0x0020;
pub const DRAMMC_REF_MASK: u32 = 0x001f;
pub const DRAMMC_REF_SHIFT: u32 = 0;

/*
 * DRAM Control Register
 */
pub const DRAMC_ADDR: u32 = 0xfffffc02;
pub unsafe fn DRAMC() -> &'static mut u16 { &mut *(DRAMC_ADDR as *mut u16) }

pub const DRAMC_DWE: u32 = 0x0001;
pub const DRAMC_RST: u32 = 0x0002;
pub const DRAMC_LPR: u32 = 0x0004;
pub const DRAMC_SLW: u32 = 0x0008;
pub const DRAMC_LSP: u32 = 0x0010;
pub const DRAMC_MSW: u32 = 0x0020;
pub const DRAMC_WS_MASK: u32 = 0x00c0;
pub const DRAMC_WS_SHIFT: u32 = 6;
pub const DRAMC_PGSZ_MASK: u32 = 0x0300;
pub const DRAMC_PGSZ_SHIFT: u32 = 8;
pub const DRAMC_PGSZ_256K: u32 = 0x0000;
pub const DRAMC_PGSZ_512K: u32 = 0x0100;
pub const DRAMC_PGSZ_1024K: u32 = 0x0200;
pub const DRAMC_PGSZ_2048K: u32 = 0x0300;
pub const DRAMC_EDO: u32 = 0x0400;
pub const DRAMC_CLK: u32 = 0x0800;
pub const DRAMC_BC_MASK: u32 = 0x3000;
pub const DRAMC_BC_SHIFT: u32 = 12;
pub const DRAMC_RM: u32 = 0x4000;
pub const DRAMC_EN: u32 = 0x8000;


/**********
 *
 * 0xFFFFFDxx -- In-Circuit Emulation (ICE)
 *
 **********/

/*
 * ICE Module Address Compare Register
 */
pub const ICEMACR_ADDR: u32 = 0xfffffd00;
pub unsafe fn ICEMACR() -> &'static mut u32 { &mut *(ICEMACR_ADDR as *mut u32) }

/*
 * ICE Module Address Mask Register
 */
pub const ICEMAMR_ADDR: u32 = 0xfffffd04;
pub unsafe fn ICEMAMR() -> &'static mut u32 { &mut *(ICEMAMR_ADDR as *mut u32) }

/*
 * ICE Module Control Compare Register
 */
pub const ICEMCCR_ADDR: u32 = 0xfffffd08;
pub unsafe fn ICEMCCR() -> &'static mut u16 { &mut *(ICEMCCR_ADDR as *mut u16) }

pub const ICEMCCR_PD: u32 = 0x0001;
pub const ICEMCCR_RW: u32 = 0x0002;

/*
 * ICE Module Control Mask Register
 */
pub const ICEMCMR_ADDR: u32 = 0xfffffd0a;
pub unsafe fn ICEMCMR() -> &'static mut u16 { &mut *(ICEMCMR_ADDR as *mut u16) }

pub const ICEMCMR_PDM: u32 = 0x0001;
pub const ICEMCMR_RWM: u32 = 0x0002;

/*
 * ICE Module Control Register 
 */
pub const ICEMCR_ADDR: u32 = 0xfffffd0c;
pub unsafe fn ICEMCR() -> &'static mut u16 { &mut *(ICEMCR_ADDR as *mut u16) }

pub const ICEMCR_CEN: u32 = 0x0001;
pub const ICEMCR_PBEN: u32 = 0x0002;
pub const ICEMCR_SB: u32 = 0x0004;
pub const ICEMCR_HMDIS: u32 = 0x0008;
pub const ICEMCR_BBIEN: u32 = 0x0010;

/*
 * ICE Module Status Register 
 */
pub const ICEMSR_ADDR: u32 = 0xfffffd0e;
pub unsafe fn ICEMSR() -> &'static mut u16 { &mut *(ICEMSR_ADDR as *mut u16) }

pub const ICEMSR_EMUEN: u32 = 0x0001;
pub const ICEMSR_BRKIRQ: u32 = 0x0002;
pub const ICEMSR_BBIRQ: u32 = 0x0004;
pub const ICEMSR_EMIRQ: u32 = 0x0008;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
