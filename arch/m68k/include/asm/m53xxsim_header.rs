/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 *	m53xxsim.h -- ColdFire 5329 registers
 */

/****************************************************************************/
// #ifndef	m53xxsim_h
#define	m53xxsim_h
/****************************************************************************/

pub const CPU_NAME: &str = "COLDFIRE(m53xx)";
pub const CPU_INSTR_PER_JIFFY: u32 = 3;
pub const MCF_BUSCLK: u32 = (MCF_CLK / 3);

// #include <asm/m53xxacr.h>

pub const MCFINT_VECBASE: u32 = 64;
pub const MCFINT_UART0: u32 = 26;
pub const MCFINT_UART1: u32 = 27;
pub const MCFINT_UART2: u32 = 28;
pub const MCFINT_I2C0: u32 = 30;
pub const MCFINT_QSPI: u32 = 31;
pub const MCFINT_FECRX0: u32 = 36;
pub const MCFINT_FECTX0: u32 = 40;
pub const MCFINT_FECENTC0: u32 = 42;

pub const MCF_IRQ_UART0: u32 = (MCFINT_VECBASE + MCFINT_UART0);
pub const MCF_IRQ_UART1: u32 = (MCFINT_VECBASE + MCFINT_UART1);
pub const MCF_IRQ_UART2: u32 = (MCFINT_VECBASE + MCFINT_UART2);

pub const MCF_IRQ_FECRX0: u32 = (MCFINT_VECBASE + MCFINT_FECRX0);
pub const MCF_IRQ_FECTX0: u32 = (MCFINT_VECBASE + MCFINT_FECTX0);
pub const MCF_IRQ_FECENTC0: u32 = (MCFINT_VECBASE + MCFINT_FECENTC0);

pub const MCF_IRQ_I2C0: u32 = (MCFINT_VECBASE + MCFINT_I2C0);
pub const MCF_IRQ_QSPI: u32 = (MCFINT_VECBASE + MCFINT_QSPI);

pub const MCF_WTM_WCR: u32 = 0xFC098000;

/*
 *	Define the 532x SIM register set addresses.
 */
pub const MCFSIM_IPRL: u32 = 0xFC048004;
pub const MCFSIM_IPRH: u32 = 0xFC048000;
pub const MCFSIM_IPR: u32 = MCFSIM_IPRL;
pub const MCFSIM_IMRL: u32 = 0xFC04800C;
pub const MCFSIM_IMRH: u32 = 0xFC048008;
pub const MCFSIM_IMR: u32 = MCFSIM_IMRL;
pub const MCFSIM_ICR0: u32 = 0xFC048040	;
pub const MCFSIM_ICR1: u32 = 0xFC048041	;
pub const MCFSIM_ICR2: u32 = 0xFC048042	;
pub const MCFSIM_ICR3: u32 = 0xFC048043	;
pub const MCFSIM_ICR4: u32 = 0xFC048044	;
pub const MCFSIM_ICR5: u32 = 0xFC048045	;
pub const MCFSIM_ICR6: u32 = 0xFC048046	;
pub const MCFSIM_ICR7: u32 = 0xFC048047	;
pub const MCFSIM_ICR8: u32 = 0xFC048048	;
pub const MCFSIM_ICR9: u32 = 0xFC048049	;
pub const MCFSIM_ICR10: u32 = 0xFC04804A;
pub const MCFSIM_ICR11: u32 = 0xFC04804B;

/*
 *	Some symbol defines for the above...
 */
pub const MCFSIM_SWDICR: u32 = MCFSIM_ICR0;
pub const MCFSIM_TIMER1ICR: u32 = MCFSIM_ICR1;
pub const MCFSIM_TIMER2ICR: u32 = MCFSIM_ICR2;
pub const MCFSIM_UART1ICR: u32 = MCFSIM_ICR4;
pub const MCFSIM_UART2ICR: u32 = MCFSIM_ICR5;
pub const MCFSIM_DMA0ICR: u32 = MCFSIM_ICR6;
pub const MCFSIM_DMA1ICR: u32 = MCFSIM_ICR7;
pub const MCFSIM_DMA2ICR: u32 = MCFSIM_ICR8;
pub const MCFSIM_DMA3ICR: u32 = MCFSIM_ICR9;


pub const MCFINTC0_SIMR: u32 = 0xFC04801C;
pub const MCFINTC0_CIMR: u32 = 0xFC04801D;
pub const MCFINTC0_ICR0: u32 = 0xFC048040;
pub const MCFINTC1_SIMR: u32 = 0xFC04C01C;
pub const MCFINTC1_CIMR: u32 = 0xFC04C01D;
pub const MCFINTC1_ICR0: u32 = 0xFC04C040;
pub const MCFINTC2_SIMR: u32 = (0);
pub const MCFINTC2_CIMR: u32 = (0);
pub const MCFINTC2_ICR0: u32 = (0);

pub const MCFSIM_ICR_TIMER1: u32 = (0xFC048040+32);
pub const MCFSIM_ICR_TIMER2: u32 = (0xFC048040+33);

/*
 *	Define system peripheral IRQ usage.
 */
pub const MCF_IRQ_TIMER: u32 = (64 + 32);
pub const MCF_IRQ_PROFILER: u32 = (64 + 33);

/*
 *  UART module.
 */
pub const MCFUART_BASE0: u32 = 0xFC060000;
pub const MCFUART_BASE1: u32 = 0xFC064000;
pub const MCFUART_BASE2: u32 = 0xFC068000;

/*
 *  FEC module.
 */
pub const MCFFEC_BASE0: u32 = 0xFC030000;
pub const MCFFEC_SIZE0: u32 = 0x800;

/*
 *  QSPI module.
 */
pub const MCFQSPI_BASE: u32 = 0xFC05C000;
pub const MCFQSPI_SIZE: u32 = 0x40;

pub const MCFQSPI_CS0: u32 = 84;
pub const MCFQSPI_CS1: u32 = 85;
pub const MCFQSPI_CS2: u32 = 86;

/*
 *  Timer module.
 */
pub const MCFTIMER_BASE1: u32 = 0xFC070000;
pub const MCFTIMER_BASE2: u32 = 0xFC074000;
pub const MCFTIMER_BASE3: u32 = 0xFC078000;
pub const MCFTIMER_BASE4: u32 = 0xFC07C000;

/*********************************************************************
 *
 * Reset Controller Module
 *
 *********************************************************************/

pub const MCF_RCR: u32 = 0xFC0A0000;
pub const MCF_RSR: u32 = 0xFC0A0001;

pub const MCF_RCR_SWRESET: u32 = 0x80;
pub const MCF_RCR_FRCSTOUT: u32 = 0x40;


/*
 * Power Management
 */
pub const MCFPM_WCR: u32 = 0xfc040013;
pub const MCFPM_PPMSR0: u32 = 0xfc04002c;
pub const MCFPM_PPMCR0: u32 = 0xfc04002d;
pub const MCFPM_PPMSR1: u32 = 0xfc04002e;
pub const MCFPM_PPMCR1: u32 = 0xfc04002f;
pub const MCFPM_PPMHR0: u32 = 0xfc040030;
pub const MCFPM_PPMLR0: u32 = 0xfc040034;
pub const MCFPM_PPMHR1: u32 = 0xfc040038;
pub const MCFPM_LPCR: u32 = 0xec090007;

/*
 *	The M5329EVB board needs a help getting its devices initialized 
 *	at kernel start time if dBUG doesn't set it up (for example 
 *	it is not used), so we need to do it manually.
 */
// #ifdef __ASSEMBLER__
// .macro m5329EVB_setup
// 	movel	#0xFC098000, %a7
// 	movel	#0x0, (%a7)
pub const CORE_SRAM: u32 = 0x80000000	;
pub const CORE_SRAM_SIZE: u32 = 0x8000;
// 	movel	#CORE_SRAM, %d0
// 	addl	#0x221, %d0
// 	movec	%d0,%RAMBAR1
// 	movel	#CORE_SRAM, %sp
// 	addl	#CORE_SRAM_SIZE, %sp
// 	jsr	sysinit
// .endm
pub const PLATFORM_SETUP: u32 = m5329EVB_setup;

// #endif /* __ASSEMBLER__ */

/*********************************************************************
 *
 * Chip Configuration Module (CCM)
 *
 *********************************************************************/

/* Register read/write macros */
pub const MCF_CCM_CCR: u32 = 0xFC0A0004;
pub const MCF_CCM_RCON: u32 = 0xFC0A0008;
pub const MCF_CCM_CIR: u32 = 0xFC0A000A;
pub const MCF_CCM_MISCCR: u32 = 0xFC0A0010;
pub const MCF_CCM_CDR: u32 = 0xFC0A0012;
pub const MCF_CCM_UHCSR: u32 = 0xFC0A0014;
pub const MCF_CCM_UOCSR: u32 = 0xFC0A0016;

/* Bit definitions and macros for MCF_CCM_CCR */
pub const MCF_CCM_CCR_RESERVED: u32 = (0x0001);
pub const MCF_CCM_CCR_PLL_MODE: u32 = (0x0003);
pub const MCF_CCM_CCR_OSC_MODE: u32 = (0x0005);
#[macro_export]
macro_rules! MCF_CCM_CCR_BOOTPS { (x:expr) => { (((x)&0x0003)<<3|0x0001) }; }
pub const MCF_CCM_CCR_LOAD: u32 = (0x0021);
pub const MCF_CCM_CCR_LIMP: u32 = (0x0041);
#[macro_export]
macro_rules! MCF_CCM_CCR_CSC { (x:expr) => { (((x)&0x0003)<<8|0x0001) }; }

/* Bit definitions and macros for MCF_CCM_RCON */
pub const MCF_CCM_RCON_RESERVED: u32 = (0x0001);
pub const MCF_CCM_RCON_PLL_MODE: u32 = (0x0003);
pub const MCF_CCM_RCON_OSC_MODE: u32 = (0x0005);
#[macro_export]
macro_rules! MCF_CCM_RCON_BOOTPS { (x:expr) => { (((x)&0x0003)<<3|0x0001) }; }
pub const MCF_CCM_RCON_LOAD: u32 = (0x0021);
pub const MCF_CCM_RCON_LIMP: u32 = (0x0041);
#[macro_export]
macro_rules! MCF_CCM_RCON_CSC { (x:expr) => { (((x)&0x0003)<<8|0x0001) }; }

/* Bit definitions and macros for MCF_CCM_CIR */
#[macro_export]
macro_rules! MCF_CCM_CIR_PRN { (x:expr) => { (((x)&0x003F)<<0) }; }
#[macro_export]
macro_rules! MCF_CCM_CIR_PIN { (x:expr) => { (((x)&0x03FF)<<6) }; }

/* Bit definitions and macros for MCF_CCM_MISCCR */
pub const MCF_CCM_MISCCR_USBSRC: u32 = (0x0001);
pub const MCF_CCM_MISCCR_USBDIV: u32 = (0x0002);
pub const MCF_CCM_MISCCR_SSI_SRC: u32 = (0x0010);
pub const MCF_CCM_MISCCR_TIM_DMA: u32 = (0x0020);
pub const MCF_CCM_MISCCR_SSI_PUS: u32 = (0x0040);
pub const MCF_CCM_MISCCR_SSI_PUE: u32 = (0x0080);
pub const MCF_CCM_MISCCR_LCD_CHEN: u32 = (0x0100);
pub const MCF_CCM_MISCCR_LIMP: u32 = (0x1000);
pub const MCF_CCM_MISCCR_PLL_LOCK: u32 = (0x2000);

/* Bit definitions and macros for MCF_CCM_CDR */
#[macro_export]
macro_rules! MCF_CCM_CDR_SSIDIV { (x:expr) => { (((x)&0x000F)<<0) }; }
#[macro_export]
macro_rules! MCF_CCM_CDR_LPDIV { (x:expr) => { (((x)&0x000F)<<8) }; }

/* Bit definitions and macros for MCF_CCM_UHCSR */
pub const MCF_CCM_UHCSR_XPDE: u32 = (0x0001);
pub const MCF_CCM_UHCSR_UHMIE: u32 = (0x0002);
pub const MCF_CCM_UHCSR_WKUP: u32 = (0x0004);
#[macro_export]
macro_rules! MCF_CCM_UHCSR_PORTIND { (x:expr) => { (((x)&0x0003)<<14) }; }

/* Bit definitions and macros for MCF_CCM_UOCSR */
pub const MCF_CCM_UOCSR_XPDE: u32 = (0x0001);
pub const MCF_CCM_UOCSR_UOMIE: u32 = (0x0002);
pub const MCF_CCM_UOCSR_WKUP: u32 = (0x0004);
pub const MCF_CCM_UOCSR_PWRFLT: u32 = (0x0008);
pub const MCF_CCM_UOCSR_SEND: u32 = (0x0010);
pub const MCF_CCM_UOCSR_VVLD: u32 = (0x0020);
pub const MCF_CCM_UOCSR_BVLD: u32 = (0x0040);
pub const MCF_CCM_UOCSR_AVLD: u32 = (0x0080);
pub const MCF_CCM_UOCSR_DPPU: u32 = (0x0100);
pub const MCF_CCM_UOCSR_DCR_VBUS: u32 = (0x0200);
pub const MCF_CCM_UOCSR_CRG_VBUS: u32 = (0x0400);
pub const MCF_CCM_UOCSR_DRV_VBUS: u32 = (0x0800);
pub const MCF_CCM_UOCSR_DMPD: u32 = (0x1000);
pub const MCF_CCM_UOCSR_DPPD: u32 = (0x2000);
#[macro_export]
macro_rules! MCF_CCM_UOCSR_PORTIND { (x:expr) => { (((x)&0x0003)<<14) }; }

/*********************************************************************
 *
 * FlexBus Chip Selects (FBCS)
 *
 *********************************************************************/

/* Register read/write macros */
pub const MCF_FBCS0_CSAR: u32 = 0xFC008000;
pub const MCF_FBCS0_CSMR: u32 = 0xFC008004;
pub const MCF_FBCS0_CSCR: u32 = 0xFC008008;
pub const MCF_FBCS1_CSAR: u32 = 0xFC00800C;
pub const MCF_FBCS1_CSMR: u32 = 0xFC008010;
pub const MCF_FBCS1_CSCR: u32 = 0xFC008014;
pub const MCF_FBCS2_CSAR: u32 = 0xFC008018;
pub const MCF_FBCS2_CSMR: u32 = 0xFC00801C;
pub const MCF_FBCS2_CSCR: u32 = 0xFC008020;
pub const MCF_FBCS3_CSAR: u32 = 0xFC008024;
pub const MCF_FBCS3_CSMR: u32 = 0xFC008028;
pub const MCF_FBCS3_CSCR: u32 = 0xFC00802C;
pub const MCF_FBCS4_CSAR: u32 = 0xFC008030;
pub const MCF_FBCS4_CSMR: u32 = 0xFC008034;
pub const MCF_FBCS4_CSCR: u32 = 0xFC008038;
pub const MCF_FBCS5_CSAR: u32 = 0xFC00803C;
pub const MCF_FBCS5_CSMR: u32 = 0xFC008040;
pub const MCF_FBCS5_CSCR: u32 = 0xFC008044;

/* Bit definitions and macros for MCF_FBCS_CSAR */
#[macro_export]
macro_rules! MCF_FBCS_CSAR_BA { (x:expr) => { ((x)&0xFFFF0000) }; }

/* Bit definitions and macros for MCF_FBCS_CSMR */
pub const MCF_FBCS_CSMR_V: u32 = (0x00000001);
pub const MCF_FBCS_CSMR_WP: u32 = (0x00000100);
#[macro_export]
macro_rules! MCF_FBCS_CSMR_BAM { (x:expr) => { (((x)&0x0000FFFF)<<16) }; }
pub const MCF_FBCS_CSMR_BAM_4G: u32 = (0xFFFF0000);
pub const MCF_FBCS_CSMR_BAM_2G: u32 = (0x7FFF0000);
pub const MCF_FBCS_CSMR_BAM_1G: u32 = (0x3FFF0000);
pub const MCF_FBCS_CSMR_BAM_1024M: u32 = (0x3FFF0000);
pub const MCF_FBCS_CSMR_BAM_512M: u32 = (0x1FFF0000);
pub const MCF_FBCS_CSMR_BAM_256M: u32 = (0x0FFF0000);
pub const MCF_FBCS_CSMR_BAM_128M: u32 = (0x07FF0000);
pub const MCF_FBCS_CSMR_BAM_64M: u32 = (0x03FF0000);
pub const MCF_FBCS_CSMR_BAM_32M: u32 = (0x01FF0000);
pub const MCF_FBCS_CSMR_BAM_16M: u32 = (0x00FF0000);
pub const MCF_FBCS_CSMR_BAM_8M: u32 = (0x007F0000);
pub const MCF_FBCS_CSMR_BAM_4M: u32 = (0x003F0000);
pub const MCF_FBCS_CSMR_BAM_2M: u32 = (0x001F0000);
pub const MCF_FBCS_CSMR_BAM_1M: u32 = (0x000F0000);
pub const MCF_FBCS_CSMR_BAM_1024K: u32 = (0x000F0000);
pub const MCF_FBCS_CSMR_BAM_512K: u32 = (0x00070000);
pub const MCF_FBCS_CSMR_BAM_256K: u32 = (0x00030000);
pub const MCF_FBCS_CSMR_BAM_128K: u32 = (0x00010000);
pub const MCF_FBCS_CSMR_BAM_64K: u32 = (0x00000000);

/* Bit definitions and macros for MCF_FBCS_CSCR */
pub const MCF_FBCS_CSCR_BSTW: u32 = (0x00000008);
pub const MCF_FBCS_CSCR_BSTR: u32 = (0x00000010);
pub const MCF_FBCS_CSCR_BEM: u32 = (0x00000020);
#[macro_export]
macro_rules! MCF_FBCS_CSCR_PS { (x:expr) => { (((x)&0x00000003)<<6) }; }
pub const MCF_FBCS_CSCR_AA: u32 = (0x00000100);
pub const MCF_FBCS_CSCR_SBM: u32 = (0x00000200);
#[macro_export]
macro_rules! MCF_FBCS_CSCR_WS { (x:expr) => { (((x)&0x0000003F)<<10) }; }
#[macro_export]
macro_rules! MCF_FBCS_CSCR_WRAH { (x:expr) => { (((x)&0x00000003)<<16) }; }
#[macro_export]
macro_rules! MCF_FBCS_CSCR_RDAH { (x:expr) => { (((x)&0x00000003)<<18) }; }
#[macro_export]
macro_rules! MCF_FBCS_CSCR_ASET { (x:expr) => { (((x)&0x00000003)<<20) }; }
pub const MCF_FBCS_CSCR_SWSEN: u32 = (0x00800000);
#[macro_export]
macro_rules! MCF_FBCS_CSCR_SWS { (x:expr) => { (((x)&0x0000003F)<<26) }; }
pub const MCF_FBCS_CSCR_PS_8: u32 = (0x0040);
pub const MCF_FBCS_CSCR_PS_16: u32 = (0x0080);
pub const MCF_FBCS_CSCR_PS_32: u32 = (0x0000);

/*********************************************************************
 *
 * General Purpose I/O (GPIO)
 *
 *********************************************************************/

/* Register read/write macros */
pub const MCFGPIO_PODR_FECH: u32 = (0xFC0A4000);
pub const MCFGPIO_PODR_FECL: u32 = (0xFC0A4001);
pub const MCFGPIO_PODR_SSI: u32 = (0xFC0A4002);
pub const MCFGPIO_PODR_BUSCTL: u32 = (0xFC0A4003);
pub const MCFGPIO_PODR_BE: u32 = (0xFC0A4004);
pub const MCFGPIO_PODR_CS: u32 = (0xFC0A4005);
pub const MCFGPIO_PODR_PWM: u32 = (0xFC0A4006);
pub const MCFGPIO_PODR_FECI2C: u32 = (0xFC0A4007);
pub const MCFGPIO_PODR_UART: u32 = (0xFC0A4009);
pub const MCFGPIO_PODR_QSPI: u32 = (0xFC0A400A);
pub const MCFGPIO_PODR_TIMER: u32 = (0xFC0A400B);
pub const MCFGPIO_PODR_LCDDATAH: u32 = (0xFC0A400D);
pub const MCFGPIO_PODR_LCDDATAM: u32 = (0xFC0A400E);
pub const MCFGPIO_PODR_LCDDATAL: u32 = (0xFC0A400F);
pub const MCFGPIO_PODR_LCDCTLH: u32 = (0xFC0A4010);
pub const MCFGPIO_PODR_LCDCTLL: u32 = (0xFC0A4011);
pub const MCFGPIO_PDDR_FECH: u32 = (0xFC0A4014);
pub const MCFGPIO_PDDR_FECL: u32 = (0xFC0A4015);
pub const MCFGPIO_PDDR_SSI: u32 = (0xFC0A4016);
pub const MCFGPIO_PDDR_BUSCTL: u32 = (0xFC0A4017);
pub const MCFGPIO_PDDR_BE: u32 = (0xFC0A4018);
pub const MCFGPIO_PDDR_CS: u32 = (0xFC0A4019);
pub const MCFGPIO_PDDR_PWM: u32 = (0xFC0A401A);
pub const MCFGPIO_PDDR_FECI2C: u32 = (0xFC0A401B);
pub const MCFGPIO_PDDR_UART: u32 = (0xFC0A401C);
pub const MCFGPIO_PDDR_QSPI: u32 = (0xFC0A401E);
pub const MCFGPIO_PDDR_TIMER: u32 = (0xFC0A401F);
pub const MCFGPIO_PDDR_LCDDATAH: u32 = (0xFC0A4021);
pub const MCFGPIO_PDDR_LCDDATAM: u32 = (0xFC0A4022);
pub const MCFGPIO_PDDR_LCDDATAL: u32 = (0xFC0A4023);
pub const MCFGPIO_PDDR_LCDCTLH: u32 = (0xFC0A4024);
pub const MCFGPIO_PDDR_LCDCTLL: u32 = (0xFC0A4025);
pub const MCFGPIO_PPDSDR_FECH: u32 = (0xFC0A4028);
pub const MCFGPIO_PPDSDR_FECL: u32 = (0xFC0A4029);
pub const MCFGPIO_PPDSDR_SSI: u32 = (0xFC0A402A);
pub const MCFGPIO_PPDSDR_BUSCTL: u32 = (0xFC0A402B);
pub const MCFGPIO_PPDSDR_BE: u32 = (0xFC0A402C);
pub const MCFGPIO_PPDSDR_CS: u32 = (0xFC0A402D);
pub const MCFGPIO_PPDSDR_PWM: u32 = (0xFC0A402E);
pub const MCFGPIO_PPDSDR_FECI2C: u32 = (0xFC0A402F);
pub const MCFGPIO_PPDSDR_UART: u32 = (0xFC0A4031);
pub const MCFGPIO_PPDSDR_QSPI: u32 = (0xFC0A4032);
pub const MCFGPIO_PPDSDR_TIMER: u32 = (0xFC0A4033);
pub const MCFGPIO_PPDSDR_LCDDATAH: u32 = (0xFC0A4035);
pub const MCFGPIO_PPDSDR_LCDDATAM: u32 = (0xFC0A4036);
pub const MCFGPIO_PPDSDR_LCDDATAL: u32 = (0xFC0A4037);
pub const MCFGPIO_PPDSDR_LCDCTLH: u32 = (0xFC0A4038);
pub const MCFGPIO_PPDSDR_LCDCTLL: u32 = (0xFC0A4039);
pub const MCFGPIO_PCLRR_FECH: u32 = (0xFC0A403C);
pub const MCFGPIO_PCLRR_FECL: u32 = (0xFC0A403D);
pub const MCFGPIO_PCLRR_SSI: u32 = (0xFC0A403E);
pub const MCFGPIO_PCLRR_BUSCTL: u32 = (0xFC0A403F);
pub const MCFGPIO_PCLRR_BE: u32 = (0xFC0A4040);
pub const MCFGPIO_PCLRR_CS: u32 = (0xFC0A4041);
pub const MCFGPIO_PCLRR_PWM: u32 = (0xFC0A4042);
pub const MCFGPIO_PCLRR_FECI2C: u32 = (0xFC0A4043);
pub const MCFGPIO_PCLRR_UART: u32 = (0xFC0A4045);
pub const MCFGPIO_PCLRR_QSPI: u32 = (0xFC0A4046);
pub const MCFGPIO_PCLRR_TIMER: u32 = (0xFC0A4047);
pub const MCFGPIO_PCLRR_LCDDATAH: u32 = (0xFC0A4049);
pub const MCFGPIO_PCLRR_LCDDATAM: u32 = (0xFC0A404A);
pub const MCFGPIO_PCLRR_LCDDATAL: u32 = (0xFC0A404B);
pub const MCFGPIO_PCLRR_LCDCTLH: u32 = (0xFC0A404C);
pub const MCFGPIO_PCLRR_LCDCTLL: u32 = (0xFC0A404D);
pub const MCFGPIO_PAR_FEC: u32 = (0xFC0A4050);
pub const MCFGPIO_PAR_PWM: u32 = (0xFC0A4051);
pub const MCFGPIO_PAR_BUSCTL: u32 = (0xFC0A4052);
pub const MCFGPIO_PAR_FECI2C: u32 = (0xFC0A4053);
pub const MCFGPIO_PAR_BE: u32 = (0xFC0A4054);
pub const MCFGPIO_PAR_CS: u32 = (0xFC0A4055);
pub const MCFGPIO_PAR_SSI: u32 = (0xFC0A4056);
pub const MCFGPIO_PAR_UART: u32 = (0xFC0A4058);
pub const MCFGPIO_PAR_QSPI: u32 = (0xFC0A405A);
pub const MCFGPIO_PAR_TIMER: u32 = (0xFC0A405C);
pub const MCFGPIO_PAR_LCDDATA: u32 = (0xFC0A405D);
pub const MCFGPIO_PAR_LCDCTL: u32 = (0xFC0A405E);
pub const MCFGPIO_PAR_IRQ: u32 = (0xFC0A4060);
pub const MCFGPIO_MSCR_FLEXBUS: u32 = (0xFC0A4064);
pub const MCFGPIO_MSCR_SDRAM: u32 = (0xFC0A4065);
pub const MCFGPIO_DSCR_I2C: u32 = (0xFC0A4068);
pub const MCFGPIO_DSCR_PWM: u32 = (0xFC0A4069);
pub const MCFGPIO_DSCR_FEC: u32 = (0xFC0A406A);
pub const MCFGPIO_DSCR_UART: u32 = (0xFC0A406B);
pub const MCFGPIO_DSCR_QSPI: u32 = (0xFC0A406C);
pub const MCFGPIO_DSCR_TIMER: u32 = (0xFC0A406D);
pub const MCFGPIO_DSCR_SSI: u32 = (0xFC0A406E);
pub const MCFGPIO_DSCR_LCD: u32 = (0xFC0A406F);
pub const MCFGPIO_DSCR_DEBUG: u32 = (0xFC0A4070);
pub const MCFGPIO_DSCR_CLKRST: u32 = (0xFC0A4071);
pub const MCFGPIO_DSCR_IRQ: u32 = (0xFC0A4072);

/* Bit definitions and macros for MCF_GPIO_PODR_FECH */
pub const MCF_GPIO_PODR_FECH_PODR_FECH0: u32 = (0x01);
pub const MCF_GPIO_PODR_FECH_PODR_FECH1: u32 = (0x02);
pub const MCF_GPIO_PODR_FECH_PODR_FECH2: u32 = (0x04);
pub const MCF_GPIO_PODR_FECH_PODR_FECH3: u32 = (0x08);
pub const MCF_GPIO_PODR_FECH_PODR_FECH4: u32 = (0x10);
pub const MCF_GPIO_PODR_FECH_PODR_FECH5: u32 = (0x20);
pub const MCF_GPIO_PODR_FECH_PODR_FECH6: u32 = (0x40);
pub const MCF_GPIO_PODR_FECH_PODR_FECH7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PODR_FECL */
pub const MCF_GPIO_PODR_FECL_PODR_FECL0: u32 = (0x01);
pub const MCF_GPIO_PODR_FECL_PODR_FECL1: u32 = (0x02);
pub const MCF_GPIO_PODR_FECL_PODR_FECL2: u32 = (0x04);
pub const MCF_GPIO_PODR_FECL_PODR_FECL3: u32 = (0x08);
pub const MCF_GPIO_PODR_FECL_PODR_FECL4: u32 = (0x10);
pub const MCF_GPIO_PODR_FECL_PODR_FECL5: u32 = (0x20);
pub const MCF_GPIO_PODR_FECL_PODR_FECL6: u32 = (0x40);
pub const MCF_GPIO_PODR_FECL_PODR_FECL7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PODR_SSI */
pub const MCF_GPIO_PODR_SSI_PODR_SSI0: u32 = (0x01);
pub const MCF_GPIO_PODR_SSI_PODR_SSI1: u32 = (0x02);
pub const MCF_GPIO_PODR_SSI_PODR_SSI2: u32 = (0x04);
pub const MCF_GPIO_PODR_SSI_PODR_SSI3: u32 = (0x08);
pub const MCF_GPIO_PODR_SSI_PODR_SSI4: u32 = (0x10);

/* Bit definitions and macros for MCF_GPIO_PODR_BUSCTL */
pub const MCF_GPIO_PODR_BUSCTL_POSDR_BUSCTL0: u32 = (0x01);
pub const MCF_GPIO_PODR_BUSCTL_PODR_BUSCTL1: u32 = (0x02);
pub const MCF_GPIO_PODR_BUSCTL_PODR_BUSCTL2: u32 = (0x04);
pub const MCF_GPIO_PODR_BUSCTL_PODR_BUSCTL3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PODR_BE */
pub const MCF_GPIO_PODR_BE_PODR_BE0: u32 = (0x01);
pub const MCF_GPIO_PODR_BE_PODR_BE1: u32 = (0x02);
pub const MCF_GPIO_PODR_BE_PODR_BE2: u32 = (0x04);
pub const MCF_GPIO_PODR_BE_PODR_BE3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PODR_CS */
pub const MCF_GPIO_PODR_CS_PODR_CS1: u32 = (0x02);
pub const MCF_GPIO_PODR_CS_PODR_CS2: u32 = (0x04);
pub const MCF_GPIO_PODR_CS_PODR_CS3: u32 = (0x08);
pub const MCF_GPIO_PODR_CS_PODR_CS4: u32 = (0x10);
pub const MCF_GPIO_PODR_CS_PODR_CS5: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PODR_PWM */
pub const MCF_GPIO_PODR_PWM_PODR_PWM2: u32 = (0x04);
pub const MCF_GPIO_PODR_PWM_PODR_PWM3: u32 = (0x08);
pub const MCF_GPIO_PODR_PWM_PODR_PWM4: u32 = (0x10);
pub const MCF_GPIO_PODR_PWM_PODR_PWM5: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PODR_FECI2C */
pub const MCF_GPIO_PODR_FECI2C_PODR_FECI2C0: u32 = (0x01);
pub const MCF_GPIO_PODR_FECI2C_PODR_FECI2C1: u32 = (0x02);
pub const MCF_GPIO_PODR_FECI2C_PODR_FECI2C2: u32 = (0x04);
pub const MCF_GPIO_PODR_FECI2C_PODR_FECI2C3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PODR_UART */
pub const MCF_GPIO_PODR_UART_PODR_UART0: u32 = (0x01);
pub const MCF_GPIO_PODR_UART_PODR_UART1: u32 = (0x02);
pub const MCF_GPIO_PODR_UART_PODR_UART2: u32 = (0x04);
pub const MCF_GPIO_PODR_UART_PODR_UART3: u32 = (0x08);
pub const MCF_GPIO_PODR_UART_PODR_UART4: u32 = (0x10);
pub const MCF_GPIO_PODR_UART_PODR_UART5: u32 = (0x20);
pub const MCF_GPIO_PODR_UART_PODR_UART6: u32 = (0x40);
pub const MCF_GPIO_PODR_UART_PODR_UART7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PODR_QSPI */
pub const MCF_GPIO_PODR_QSPI_PODR_QSPI0: u32 = (0x01);
pub const MCF_GPIO_PODR_QSPI_PODR_QSPI1: u32 = (0x02);
pub const MCF_GPIO_PODR_QSPI_PODR_QSPI2: u32 = (0x04);
pub const MCF_GPIO_PODR_QSPI_PODR_QSPI3: u32 = (0x08);
pub const MCF_GPIO_PODR_QSPI_PODR_QSPI4: u32 = (0x10);
pub const MCF_GPIO_PODR_QSPI_PODR_QSPI5: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PODR_TIMER */
pub const MCF_GPIO_PODR_TIMER_PODR_TIMER0: u32 = (0x01);
pub const MCF_GPIO_PODR_TIMER_PODR_TIMER1: u32 = (0x02);
pub const MCF_GPIO_PODR_TIMER_PODR_TIMER2: u32 = (0x04);
pub const MCF_GPIO_PODR_TIMER_PODR_TIMER3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PODR_LCDDATAH */
pub const MCF_GPIO_PODR_LCDDATAH_PODR_LCDDATAH0: u32 = (0x01);
pub const MCF_GPIO_PODR_LCDDATAH_PODR_LCDDATAH1: u32 = (0x02);

/* Bit definitions and macros for MCF_GPIO_PODR_LCDDATAM */
pub const MCF_GPIO_PODR_LCDDATAM_PODR_LCDDATAM0: u32 = (0x01);
pub const MCF_GPIO_PODR_LCDDATAM_PODR_LCDDATAM1: u32 = (0x02);
pub const MCF_GPIO_PODR_LCDDATAM_PODR_LCDDATAM2: u32 = (0x04);
pub const MCF_GPIO_PODR_LCDDATAM_PODR_LCDDATAM3: u32 = (0x08);
pub const MCF_GPIO_PODR_LCDDATAM_PODR_LCDDATAM4: u32 = (0x10);
pub const MCF_GPIO_PODR_LCDDATAM_PODR_LCDDATAM5: u32 = (0x20);
pub const MCF_GPIO_PODR_LCDDATAM_PODR_LCDDATAM6: u32 = (0x40);
pub const MCF_GPIO_PODR_LCDDATAM_PODR_LCDDATAM7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PODR_LCDDATAL */
pub const MCF_GPIO_PODR_LCDDATAL_PODR_LCDDATAL0: u32 = (0x01);
pub const MCF_GPIO_PODR_LCDDATAL_PODR_LCDDATAL1: u32 = (0x02);
pub const MCF_GPIO_PODR_LCDDATAL_PODR_LCDDATAL2: u32 = (0x04);
pub const MCF_GPIO_PODR_LCDDATAL_PODR_LCDDATAL3: u32 = (0x08);
pub const MCF_GPIO_PODR_LCDDATAL_PODR_LCDDATAL4: u32 = (0x10);
pub const MCF_GPIO_PODR_LCDDATAL_PODR_LCDDATAL5: u32 = (0x20);
pub const MCF_GPIO_PODR_LCDDATAL_PODR_LCDDATAL6: u32 = (0x40);
pub const MCF_GPIO_PODR_LCDDATAL_PODR_LCDDATAL7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PODR_LCDCTLH */
pub const MCF_GPIO_PODR_LCDCTLH_PODR_LCDCTLH0: u32 = (0x01);

/* Bit definitions and macros for MCF_GPIO_PODR_LCDCTLL */
pub const MCF_GPIO_PODR_LCDCTLL_PODR_LCDCTLL0: u32 = (0x01);
pub const MCF_GPIO_PODR_LCDCTLL_PODR_LCDCTLL1: u32 = (0x02);
pub const MCF_GPIO_PODR_LCDCTLL_PODR_LCDCTLL2: u32 = (0x04);
pub const MCF_GPIO_PODR_LCDCTLL_PODR_LCDCTLL3: u32 = (0x08);
pub const MCF_GPIO_PODR_LCDCTLL_PODR_LCDCTLL4: u32 = (0x10);
pub const MCF_GPIO_PODR_LCDCTLL_PODR_LCDCTLL5: u32 = (0x20);
pub const MCF_GPIO_PODR_LCDCTLL_PODR_LCDCTLL6: u32 = (0x40);
pub const MCF_GPIO_PODR_LCDCTLL_PODR_LCDCTLL7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PDDR_FECH */
pub const MCF_GPIO_PDDR_FECH_PDDR_FECH0: u32 = (0x01);
pub const MCF_GPIO_PDDR_FECH_PDDR_FECH1: u32 = (0x02);
pub const MCF_GPIO_PDDR_FECH_PDDR_FECH2: u32 = (0x04);
pub const MCF_GPIO_PDDR_FECH_PDDR_FECH3: u32 = (0x08);
pub const MCF_GPIO_PDDR_FECH_PDDR_FECH4: u32 = (0x10);
pub const MCF_GPIO_PDDR_FECH_PDDR_FECH5: u32 = (0x20);
pub const MCF_GPIO_PDDR_FECH_PDDR_FECH6: u32 = (0x40);
pub const MCF_GPIO_PDDR_FECH_PDDR_FECH7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PDDR_FECL */
pub const MCF_GPIO_PDDR_FECL_PDDR_FECL0: u32 = (0x01);
pub const MCF_GPIO_PDDR_FECL_PDDR_FECL1: u32 = (0x02);
pub const MCF_GPIO_PDDR_FECL_PDDR_FECL2: u32 = (0x04);
pub const MCF_GPIO_PDDR_FECL_PDDR_FECL3: u32 = (0x08);
pub const MCF_GPIO_PDDR_FECL_PDDR_FECL4: u32 = (0x10);
pub const MCF_GPIO_PDDR_FECL_PDDR_FECL5: u32 = (0x20);
pub const MCF_GPIO_PDDR_FECL_PDDR_FECL6: u32 = (0x40);
pub const MCF_GPIO_PDDR_FECL_PDDR_FECL7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PDDR_SSI */
pub const MCF_GPIO_PDDR_SSI_PDDR_SSI0: u32 = (0x01);
pub const MCF_GPIO_PDDR_SSI_PDDR_SSI1: u32 = (0x02);
pub const MCF_GPIO_PDDR_SSI_PDDR_SSI2: u32 = (0x04);
pub const MCF_GPIO_PDDR_SSI_PDDR_SSI3: u32 = (0x08);
pub const MCF_GPIO_PDDR_SSI_PDDR_SSI4: u32 = (0x10);

/* Bit definitions and macros for MCF_GPIO_PDDR_BUSCTL */
pub const MCF_GPIO_PDDR_BUSCTL_POSDR_BUSCTL0: u32 = (0x01);
pub const MCF_GPIO_PDDR_BUSCTL_PDDR_BUSCTL1: u32 = (0x02);
pub const MCF_GPIO_PDDR_BUSCTL_PDDR_BUSCTL2: u32 = (0x04);
pub const MCF_GPIO_PDDR_BUSCTL_PDDR_BUSCTL3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PDDR_BE */
pub const MCF_GPIO_PDDR_BE_PDDR_BE0: u32 = (0x01);
pub const MCF_GPIO_PDDR_BE_PDDR_BE1: u32 = (0x02);
pub const MCF_GPIO_PDDR_BE_PDDR_BE2: u32 = (0x04);
pub const MCF_GPIO_PDDR_BE_PDDR_BE3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PDDR_CS */
pub const MCF_GPIO_PDDR_CS_PDDR_CS1: u32 = (0x02);
pub const MCF_GPIO_PDDR_CS_PDDR_CS2: u32 = (0x04);
pub const MCF_GPIO_PDDR_CS_PDDR_CS3: u32 = (0x08);
pub const MCF_GPIO_PDDR_CS_PDDR_CS4: u32 = (0x10);
pub const MCF_GPIO_PDDR_CS_PDDR_CS5: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PDDR_PWM */
pub const MCF_GPIO_PDDR_PWM_PDDR_PWM2: u32 = (0x04);
pub const MCF_GPIO_PDDR_PWM_PDDR_PWM3: u32 = (0x08);
pub const MCF_GPIO_PDDR_PWM_PDDR_PWM4: u32 = (0x10);
pub const MCF_GPIO_PDDR_PWM_PDDR_PWM5: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PDDR_FECI2C */
pub const MCF_GPIO_PDDR_FECI2C_PDDR_FECI2C0: u32 = (0x01);
pub const MCF_GPIO_PDDR_FECI2C_PDDR_FECI2C1: u32 = (0x02);
pub const MCF_GPIO_PDDR_FECI2C_PDDR_FECI2C2: u32 = (0x04);
pub const MCF_GPIO_PDDR_FECI2C_PDDR_FECI2C3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PDDR_UART */
pub const MCF_GPIO_PDDR_UART_PDDR_UART0: u32 = (0x01);
pub const MCF_GPIO_PDDR_UART_PDDR_UART1: u32 = (0x02);
pub const MCF_GPIO_PDDR_UART_PDDR_UART2: u32 = (0x04);
pub const MCF_GPIO_PDDR_UART_PDDR_UART3: u32 = (0x08);
pub const MCF_GPIO_PDDR_UART_PDDR_UART4: u32 = (0x10);
pub const MCF_GPIO_PDDR_UART_PDDR_UART5: u32 = (0x20);
pub const MCF_GPIO_PDDR_UART_PDDR_UART6: u32 = (0x40);
pub const MCF_GPIO_PDDR_UART_PDDR_UART7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PDDR_QSPI */
pub const MCF_GPIO_PDDR_QSPI_PDDR_QSPI0: u32 = (0x01);
pub const MCF_GPIO_PDDR_QSPI_PDDR_QSPI1: u32 = (0x02);
pub const MCF_GPIO_PDDR_QSPI_PDDR_QSPI2: u32 = (0x04);
pub const MCF_GPIO_PDDR_QSPI_PDDR_QSPI3: u32 = (0x08);
pub const MCF_GPIO_PDDR_QSPI_PDDR_QSPI4: u32 = (0x10);
pub const MCF_GPIO_PDDR_QSPI_PDDR_QSPI5: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PDDR_TIMER */
pub const MCF_GPIO_PDDR_TIMER_PDDR_TIMER0: u32 = (0x01);
pub const MCF_GPIO_PDDR_TIMER_PDDR_TIMER1: u32 = (0x02);
pub const MCF_GPIO_PDDR_TIMER_PDDR_TIMER2: u32 = (0x04);
pub const MCF_GPIO_PDDR_TIMER_PDDR_TIMER3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PDDR_LCDDATAH */
pub const MCF_GPIO_PDDR_LCDDATAH_PDDR_LCDDATAH0: u32 = (0x01);
pub const MCF_GPIO_PDDR_LCDDATAH_PDDR_LCDDATAH1: u32 = (0x02);

/* Bit definitions and macros for MCF_GPIO_PDDR_LCDDATAM */
pub const MCF_GPIO_PDDR_LCDDATAM_PDDR_LCDDATAM0: u32 = (0x01);
pub const MCF_GPIO_PDDR_LCDDATAM_PDDR_LCDDATAM1: u32 = (0x02);
pub const MCF_GPIO_PDDR_LCDDATAM_PDDR_LCDDATAM2: u32 = (0x04);
pub const MCF_GPIO_PDDR_LCDDATAM_PDDR_LCDDATAM3: u32 = (0x08);
pub const MCF_GPIO_PDDR_LCDDATAM_PDDR_LCDDATAM4: u32 = (0x10);
pub const MCF_GPIO_PDDR_LCDDATAM_PDDR_LCDDATAM5: u32 = (0x20);
pub const MCF_GPIO_PDDR_LCDDATAM_PDDR_LCDDATAM6: u32 = (0x40);
pub const MCF_GPIO_PDDR_LCDDATAM_PDDR_LCDDATAM7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PDDR_LCDDATAL */
pub const MCF_GPIO_PDDR_LCDDATAL_PDDR_LCDDATAL0: u32 = (0x01);
pub const MCF_GPIO_PDDR_LCDDATAL_PDDR_LCDDATAL1: u32 = (0x02);
pub const MCF_GPIO_PDDR_LCDDATAL_PDDR_LCDDATAL2: u32 = (0x04);
pub const MCF_GPIO_PDDR_LCDDATAL_PDDR_LCDDATAL3: u32 = (0x08);
pub const MCF_GPIO_PDDR_LCDDATAL_PDDR_LCDDATAL4: u32 = (0x10);
pub const MCF_GPIO_PDDR_LCDDATAL_PDDR_LCDDATAL5: u32 = (0x20);
pub const MCF_GPIO_PDDR_LCDDATAL_PDDR_LCDDATAL6: u32 = (0x40);
pub const MCF_GPIO_PDDR_LCDDATAL_PDDR_LCDDATAL7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PDDR_LCDCTLH */
pub const MCF_GPIO_PDDR_LCDCTLH_PDDR_LCDCTLH0: u32 = (0x01);

/* Bit definitions and macros for MCF_GPIO_PDDR_LCDCTLL */
pub const MCF_GPIO_PDDR_LCDCTLL_PDDR_LCDCTLL0: u32 = (0x01);
pub const MCF_GPIO_PDDR_LCDCTLL_PDDR_LCDCTLL1: u32 = (0x02);
pub const MCF_GPIO_PDDR_LCDCTLL_PDDR_LCDCTLL2: u32 = (0x04);
pub const MCF_GPIO_PDDR_LCDCTLL_PDDR_LCDCTLL3: u32 = (0x08);
pub const MCF_GPIO_PDDR_LCDCTLL_PDDR_LCDCTLL4: u32 = (0x10);
pub const MCF_GPIO_PDDR_LCDCTLL_PDDR_LCDCTLL5: u32 = (0x20);
pub const MCF_GPIO_PDDR_LCDCTLL_PDDR_LCDCTLL6: u32 = (0x40);
pub const MCF_GPIO_PDDR_LCDCTLL_PDDR_LCDCTLL7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_FECH */
pub const MCF_GPIO_PPDSDR_FECH_PPDSDR_FECH0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_FECH_PPDSDR_FECH1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_FECH_PPDSDR_FECH2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_FECH_PPDSDR_FECH3: u32 = (0x08);
pub const MCF_GPIO_PPDSDR_FECH_PPDSDR_FECH4: u32 = (0x10);
pub const MCF_GPIO_PPDSDR_FECH_PPDSDR_FECH5: u32 = (0x20);
pub const MCF_GPIO_PPDSDR_FECH_PPDSDR_FECH6: u32 = (0x40);
pub const MCF_GPIO_PPDSDR_FECH_PPDSDR_FECH7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_FECL */
pub const MCF_GPIO_PPDSDR_FECL_PPDSDR_FECL0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_FECL_PPDSDR_FECL1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_FECL_PPDSDR_FECL2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_FECL_PPDSDR_FECL3: u32 = (0x08);
pub const MCF_GPIO_PPDSDR_FECL_PPDSDR_FECL4: u32 = (0x10);
pub const MCF_GPIO_PPDSDR_FECL_PPDSDR_FECL5: u32 = (0x20);
pub const MCF_GPIO_PPDSDR_FECL_PPDSDR_FECL6: u32 = (0x40);
pub const MCF_GPIO_PPDSDR_FECL_PPDSDR_FECL7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_SSI */
pub const MCF_GPIO_PPDSDR_SSI_PPDSDR_SSI0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_SSI_PPDSDR_SSI1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_SSI_PPDSDR_SSI2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_SSI_PPDSDR_SSI3: u32 = (0x08);
pub const MCF_GPIO_PPDSDR_SSI_PPDSDR_SSI4: u32 = (0x10);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_BUSCTL */
pub const MCF_GPIO_PPDSDR_BUSCTL_POSDR_BUSCTL0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_BUSCTL_PPDSDR_BUSCTL1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_BUSCTL_PPDSDR_BUSCTL2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_BUSCTL_PPDSDR_BUSCTL3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_BE */
pub const MCF_GPIO_PPDSDR_BE_PPDSDR_BE0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_BE_PPDSDR_BE1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_BE_PPDSDR_BE2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_BE_PPDSDR_BE3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_CS */
pub const MCF_GPIO_PPDSDR_CS_PPDSDR_CS1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_CS_PPDSDR_CS2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_CS_PPDSDR_CS3: u32 = (0x08);
pub const MCF_GPIO_PPDSDR_CS_PPDSDR_CS4: u32 = (0x10);
pub const MCF_GPIO_PPDSDR_CS_PPDSDR_CS5: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_PWM */
pub const MCF_GPIO_PPDSDR_PWM_PPDSDR_PWM2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_PWM_PPDSDR_PWM3: u32 = (0x08);
pub const MCF_GPIO_PPDSDR_PWM_PPDSDR_PWM4: u32 = (0x10);
pub const MCF_GPIO_PPDSDR_PWM_PPDSDR_PWM5: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_FECI2C */
pub const MCF_GPIO_PPDSDR_FECI2C_PPDSDR_FECI2C0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_FECI2C_PPDSDR_FECI2C1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_FECI2C_PPDSDR_FECI2C2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_FECI2C_PPDSDR_FECI2C3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_UART */
pub const MCF_GPIO_PPDSDR_UART_PPDSDR_UART0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_UART_PPDSDR_UART1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_UART_PPDSDR_UART2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_UART_PPDSDR_UART3: u32 = (0x08);
pub const MCF_GPIO_PPDSDR_UART_PPDSDR_UART4: u32 = (0x10);
pub const MCF_GPIO_PPDSDR_UART_PPDSDR_UART5: u32 = (0x20);
pub const MCF_GPIO_PPDSDR_UART_PPDSDR_UART6: u32 = (0x40);
pub const MCF_GPIO_PPDSDR_UART_PPDSDR_UART7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_QSPI */
pub const MCF_GPIO_PPDSDR_QSPI_PPDSDR_QSPI0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_QSPI_PPDSDR_QSPI1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_QSPI_PPDSDR_QSPI2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_QSPI_PPDSDR_QSPI3: u32 = (0x08);
pub const MCF_GPIO_PPDSDR_QSPI_PPDSDR_QSPI4: u32 = (0x10);
pub const MCF_GPIO_PPDSDR_QSPI_PPDSDR_QSPI5: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_TIMER */
pub const MCF_GPIO_PPDSDR_TIMER_PPDSDR_TIMER0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_TIMER_PPDSDR_TIMER1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_TIMER_PPDSDR_TIMER2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_TIMER_PPDSDR_TIMER3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_LCDDATAH */
pub const MCF_GPIO_PPDSDR_LCDDATAH_PPDSDR_LCDDATAH0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_LCDDATAH_PPDSDR_LCDDATAH1: u32 = (0x02);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_LCDDATAM */
pub const MCF_GPIO_PPDSDR_LCDDATAM_PPDSDR_LCDDATAM0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_LCDDATAM_PPDSDR_LCDDATAM1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_LCDDATAM_PPDSDR_LCDDATAM2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_LCDDATAM_PPDSDR_LCDDATAM3: u32 = (0x08);
pub const MCF_GPIO_PPDSDR_LCDDATAM_PPDSDR_LCDDATAM4: u32 = (0x10);
pub const MCF_GPIO_PPDSDR_LCDDATAM_PPDSDR_LCDDATAM5: u32 = (0x20);
pub const MCF_GPIO_PPDSDR_LCDDATAM_PPDSDR_LCDDATAM6: u32 = (0x40);
pub const MCF_GPIO_PPDSDR_LCDDATAM_PPDSDR_LCDDATAM7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_LCDDATAL */
pub const MCF_GPIO_PPDSDR_LCDDATAL_PPDSDR_LCDDATAL0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_LCDDATAL_PPDSDR_LCDDATAL1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_LCDDATAL_PPDSDR_LCDDATAL2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_LCDDATAL_PPDSDR_LCDDATAL3: u32 = (0x08);
pub const MCF_GPIO_PPDSDR_LCDDATAL_PPDSDR_LCDDATAL4: u32 = (0x10);
pub const MCF_GPIO_PPDSDR_LCDDATAL_PPDSDR_LCDDATAL5: u32 = (0x20);
pub const MCF_GPIO_PPDSDR_LCDDATAL_PPDSDR_LCDDATAL6: u32 = (0x40);
pub const MCF_GPIO_PPDSDR_LCDDATAL_PPDSDR_LCDDATAL7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_LCDCTLH */
pub const MCF_GPIO_PPDSDR_LCDCTLH_PPDSDR_LCDCTLH0: u32 = (0x01);

/* Bit definitions and macros for MCF_GPIO_PPDSDR_LCDCTLL */
pub const MCF_GPIO_PPDSDR_LCDCTLL_PPDSDR_LCDCTLL0: u32 = (0x01);
pub const MCF_GPIO_PPDSDR_LCDCTLL_PPDSDR_LCDCTLL1: u32 = (0x02);
pub const MCF_GPIO_PPDSDR_LCDCTLL_PPDSDR_LCDCTLL2: u32 = (0x04);
pub const MCF_GPIO_PPDSDR_LCDCTLL_PPDSDR_LCDCTLL3: u32 = (0x08);
pub const MCF_GPIO_PPDSDR_LCDCTLL_PPDSDR_LCDCTLL4: u32 = (0x10);
pub const MCF_GPIO_PPDSDR_LCDCTLL_PPDSDR_LCDCTLL5: u32 = (0x20);
pub const MCF_GPIO_PPDSDR_LCDCTLL_PPDSDR_LCDCTLL6: u32 = (0x40);
pub const MCF_GPIO_PPDSDR_LCDCTLL_PPDSDR_LCDCTLL7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PCLRR_FECH */
pub const MCF_GPIO_PCLRR_FECH_PCLRR_FECH0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_FECH_PCLRR_FECH1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_FECH_PCLRR_FECH2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_FECH_PCLRR_FECH3: u32 = (0x08);
pub const MCF_GPIO_PCLRR_FECH_PCLRR_FECH4: u32 = (0x10);
pub const MCF_GPIO_PCLRR_FECH_PCLRR_FECH5: u32 = (0x20);
pub const MCF_GPIO_PCLRR_FECH_PCLRR_FECH6: u32 = (0x40);
pub const MCF_GPIO_PCLRR_FECH_PCLRR_FECH7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PCLRR_FECL */
pub const MCF_GPIO_PCLRR_FECL_PCLRR_FECL0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_FECL_PCLRR_FECL1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_FECL_PCLRR_FECL2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_FECL_PCLRR_FECL3: u32 = (0x08);
pub const MCF_GPIO_PCLRR_FECL_PCLRR_FECL4: u32 = (0x10);
pub const MCF_GPIO_PCLRR_FECL_PCLRR_FECL5: u32 = (0x20);
pub const MCF_GPIO_PCLRR_FECL_PCLRR_FECL6: u32 = (0x40);
pub const MCF_GPIO_PCLRR_FECL_PCLRR_FECL7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PCLRR_SSI */
pub const MCF_GPIO_PCLRR_SSI_PCLRR_SSI0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_SSI_PCLRR_SSI1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_SSI_PCLRR_SSI2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_SSI_PCLRR_SSI3: u32 = (0x08);
pub const MCF_GPIO_PCLRR_SSI_PCLRR_SSI4: u32 = (0x10);

/* Bit definitions and macros for MCF_GPIO_PCLRR_BUSCTL */
pub const MCF_GPIO_PCLRR_BUSCTL_POSDR_BUSCTL0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_BUSCTL_PCLRR_BUSCTL1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_BUSCTL_PCLRR_BUSCTL2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_BUSCTL_PCLRR_BUSCTL3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PCLRR_BE */
pub const MCF_GPIO_PCLRR_BE_PCLRR_BE0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_BE_PCLRR_BE1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_BE_PCLRR_BE2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_BE_PCLRR_BE3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PCLRR_CS */
pub const MCF_GPIO_PCLRR_CS_PCLRR_CS1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_CS_PCLRR_CS2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_CS_PCLRR_CS3: u32 = (0x08);
pub const MCF_GPIO_PCLRR_CS_PCLRR_CS4: u32 = (0x10);
pub const MCF_GPIO_PCLRR_CS_PCLRR_CS5: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PCLRR_PWM */
pub const MCF_GPIO_PCLRR_PWM_PCLRR_PWM2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_PWM_PCLRR_PWM3: u32 = (0x08);
pub const MCF_GPIO_PCLRR_PWM_PCLRR_PWM4: u32 = (0x10);
pub const MCF_GPIO_PCLRR_PWM_PCLRR_PWM5: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PCLRR_FECI2C */
pub const MCF_GPIO_PCLRR_FECI2C_PCLRR_FECI2C0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_FECI2C_PCLRR_FECI2C1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_FECI2C_PCLRR_FECI2C2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_FECI2C_PCLRR_FECI2C3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PCLRR_UART */
pub const MCF_GPIO_PCLRR_UART_PCLRR_UART0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_UART_PCLRR_UART1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_UART_PCLRR_UART2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_UART_PCLRR_UART3: u32 = (0x08);
pub const MCF_GPIO_PCLRR_UART_PCLRR_UART4: u32 = (0x10);
pub const MCF_GPIO_PCLRR_UART_PCLRR_UART5: u32 = (0x20);
pub const MCF_GPIO_PCLRR_UART_PCLRR_UART6: u32 = (0x40);
pub const MCF_GPIO_PCLRR_UART_PCLRR_UART7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PCLRR_QSPI */
pub const MCF_GPIO_PCLRR_QSPI_PCLRR_QSPI0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_QSPI_PCLRR_QSPI1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_QSPI_PCLRR_QSPI2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_QSPI_PCLRR_QSPI3: u32 = (0x08);
pub const MCF_GPIO_PCLRR_QSPI_PCLRR_QSPI4: u32 = (0x10);
pub const MCF_GPIO_PCLRR_QSPI_PCLRR_QSPI5: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PCLRR_TIMER */
pub const MCF_GPIO_PCLRR_TIMER_PCLRR_TIMER0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_TIMER_PCLRR_TIMER1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_TIMER_PCLRR_TIMER2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_TIMER_PCLRR_TIMER3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PCLRR_LCDDATAH */
pub const MCF_GPIO_PCLRR_LCDDATAH_PCLRR_LCDDATAH0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_LCDDATAH_PCLRR_LCDDATAH1: u32 = (0x02);

/* Bit definitions and macros for MCF_GPIO_PCLRR_LCDDATAM */
pub const MCF_GPIO_PCLRR_LCDDATAM_PCLRR_LCDDATAM0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_LCDDATAM_PCLRR_LCDDATAM1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_LCDDATAM_PCLRR_LCDDATAM2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_LCDDATAM_PCLRR_LCDDATAM3: u32 = (0x08);
pub const MCF_GPIO_PCLRR_LCDDATAM_PCLRR_LCDDATAM4: u32 = (0x10);
pub const MCF_GPIO_PCLRR_LCDDATAM_PCLRR_LCDDATAM5: u32 = (0x20);
pub const MCF_GPIO_PCLRR_LCDDATAM_PCLRR_LCDDATAM6: u32 = (0x40);
pub const MCF_GPIO_PCLRR_LCDDATAM_PCLRR_LCDDATAM7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PCLRR_LCDDATAL */
pub const MCF_GPIO_PCLRR_LCDDATAL_PCLRR_LCDDATAL0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_LCDDATAL_PCLRR_LCDDATAL1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_LCDDATAL_PCLRR_LCDDATAL2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_LCDDATAL_PCLRR_LCDDATAL3: u32 = (0x08);
pub const MCF_GPIO_PCLRR_LCDDATAL_PCLRR_LCDDATAL4: u32 = (0x10);
pub const MCF_GPIO_PCLRR_LCDDATAL_PCLRR_LCDDATAL5: u32 = (0x20);
pub const MCF_GPIO_PCLRR_LCDDATAL_PCLRR_LCDDATAL6: u32 = (0x40);
pub const MCF_GPIO_PCLRR_LCDDATAL_PCLRR_LCDDATAL7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PCLRR_LCDCTLH */
pub const MCF_GPIO_PCLRR_LCDCTLH_PCLRR_LCDCTLH0: u32 = (0x01);

/* Bit definitions and macros for MCF_GPIO_PCLRR_LCDCTLL */
pub const MCF_GPIO_PCLRR_LCDCTLL_PCLRR_LCDCTLL0: u32 = (0x01);
pub const MCF_GPIO_PCLRR_LCDCTLL_PCLRR_LCDCTLL1: u32 = (0x02);
pub const MCF_GPIO_PCLRR_LCDCTLL_PCLRR_LCDCTLL2: u32 = (0x04);
pub const MCF_GPIO_PCLRR_LCDCTLL_PCLRR_LCDCTLL3: u32 = (0x08);
pub const MCF_GPIO_PCLRR_LCDCTLL_PCLRR_LCDCTLL4: u32 = (0x10);
pub const MCF_GPIO_PCLRR_LCDCTLL_PCLRR_LCDCTLL5: u32 = (0x20);
pub const MCF_GPIO_PCLRR_LCDCTLL_PCLRR_LCDCTLL6: u32 = (0x40);
pub const MCF_GPIO_PCLRR_LCDCTLL_PCLRR_LCDCTLL7: u32 = (0x80);

/* Bit definitions and macros for MCF_GPIO_PAR_FEC */
#[macro_export]
macro_rules! MCF_GPIO_PAR_FEC_PAR_FEC_MII { (x:expr) => { (((x)&0x03)<<0) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_FEC_PAR_FEC_7W { (x:expr) => { (((x)&0x03)<<2) }; }
pub const MCF_GPIO_PAR_FEC_PAR_FEC_7W_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_FEC_PAR_FEC_7W_URTS1: u32 = (0x04);
pub const MCF_GPIO_PAR_FEC_PAR_FEC_7W_FEC: u32 = (0x0C);
pub const MCF_GPIO_PAR_FEC_PAR_FEC_MII_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_FEC_PAR_FEC_MII_UART: u32 = (0x01);
pub const MCF_GPIO_PAR_FEC_PAR_FEC_MII_FEC: u32 = (0x03);

/* Bit definitions and macros for MCF_GPIO_PAR_PWM */
#[macro_export]
macro_rules! MCF_GPIO_PAR_PWM_PAR_PWM1 { (x:expr) => { (((x)&0x03)<<0) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_PWM_PAR_PWM3 { (x:expr) => { (((x)&0x03)<<2) }; }
pub const MCF_GPIO_PAR_PWM_PAR_PWM5: u32 = (0x10);
pub const MCF_GPIO_PAR_PWM_PAR_PWM7: u32 = (0x20);

/* Bit definitions and macros for MCF_GPIO_PAR_BUSCTL */
#[macro_export]
macro_rules! MCF_GPIO_PAR_BUSCTL_PAR_TS { (x:expr) => { (((x)&0x03)<<3) }; }
pub const MCF_GPIO_PAR_BUSCTL_PAR_RWB: u32 = (0x20);
pub const MCF_GPIO_PAR_BUSCTL_PAR_TA: u32 = (0x40);
pub const MCF_GPIO_PAR_BUSCTL_PAR_OE: u32 = (0x80);
pub const MCF_GPIO_PAR_BUSCTL_PAR_OE_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_BUSCTL_PAR_OE_OE: u32 = (0x80);
pub const MCF_GPIO_PAR_BUSCTL_PAR_TA_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_BUSCTL_PAR_TA_TA: u32 = (0x40);
pub const MCF_GPIO_PAR_BUSCTL_PAR_RWB_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_BUSCTL_PAR_RWB_RWB: u32 = (0x20);
pub const MCF_GPIO_PAR_BUSCTL_PAR_TS_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_BUSCTL_PAR_TS_DACK0: u32 = (0x10);
pub const MCF_GPIO_PAR_BUSCTL_PAR_TS_TS: u32 = (0x18);

/* Bit definitions and macros for MCF_GPIO_PAR_FECI2C */
#[macro_export]
macro_rules! MCF_GPIO_PAR_FECI2C_PAR_SDA { (x:expr) => { (((x)&0x03)<<0) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_FECI2C_PAR_SCL { (x:expr) => { (((x)&0x03)<<2) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_FECI2C_PAR_MDIO { (x:expr) => { (((x)&0x03)<<4) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_FECI2C_PAR_MDC { (x:expr) => { (((x)&0x03)<<6) }; }
pub const MCF_GPIO_PAR_FECI2C_PAR_MDC_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_FECI2C_PAR_MDC_UTXD2: u32 = (0x40);
pub const MCF_GPIO_PAR_FECI2C_PAR_MDC_SCL: u32 = (0x80);
pub const MCF_GPIO_PAR_FECI2C_PAR_MDC_EMDC: u32 = (0xC0);
pub const MCF_GPIO_PAR_FECI2C_PAR_MDIO_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_FECI2C_PAR_MDIO_URXD2: u32 = (0x10);
pub const MCF_GPIO_PAR_FECI2C_PAR_MDIO_SDA: u32 = (0x20);
pub const MCF_GPIO_PAR_FECI2C_PAR_MDIO_EMDIO: u32 = (0x30);
pub const MCF_GPIO_PAR_FECI2C_PAR_SCL_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_FECI2C_PAR_SCL_UTXD2: u32 = (0x04);
pub const MCF_GPIO_PAR_FECI2C_PAR_SCL_SCL: u32 = (0x0C);
pub const MCF_GPIO_PAR_FECI2C_PAR_SDA_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_FECI2C_PAR_SDA_URXD2: u32 = (0x02);
pub const MCF_GPIO_PAR_FECI2C_PAR_SDA_SDA: u32 = (0x03);

/* Bit definitions and macros for MCF_GPIO_PAR_BE */
pub const MCF_GPIO_PAR_BE_PAR_BE0: u32 = (0x01);
pub const MCF_GPIO_PAR_BE_PAR_BE1: u32 = (0x02);
pub const MCF_GPIO_PAR_BE_PAR_BE2: u32 = (0x04);
pub const MCF_GPIO_PAR_BE_PAR_BE3: u32 = (0x08);

/* Bit definitions and macros for MCF_GPIO_PAR_CS */
pub const MCF_GPIO_PAR_CS_PAR_CS1: u32 = (0x02);
pub const MCF_GPIO_PAR_CS_PAR_CS2: u32 = (0x04);
pub const MCF_GPIO_PAR_CS_PAR_CS3: u32 = (0x08);
pub const MCF_GPIO_PAR_CS_PAR_CS4: u32 = (0x10);
pub const MCF_GPIO_PAR_CS_PAR_CS5: u32 = (0x20);
pub const MCF_GPIO_PAR_CS_PAR_CS_CS1_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_CS_PAR_CS_CS1_SDCS1: u32 = (0x01);
pub const MCF_GPIO_PAR_CS_PAR_CS_CS1_CS1: u32 = (0x03);

/* Bit definitions and macros for MCF_GPIO_PAR_SSI */
pub const MCF_GPIO_PAR_SSI_PAR_MCLK: u32 = (0x0080);
#[macro_export]
macro_rules! MCF_GPIO_PAR_SSI_PAR_TXD { (x:expr) => { (((x)&0x0003)<<8) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_SSI_PAR_RXD { (x:expr) => { (((x)&0x0003)<<10) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_SSI_PAR_FS { (x:expr) => { (((x)&0x0003)<<12) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_SSI_PAR_BCLK { (x:expr) => { (((x)&0x0003)<<14) }; }

/* Bit definitions and macros for MCF_GPIO_PAR_UART */
pub const MCF_GPIO_PAR_UART_PAR_UTXD0: u32 = (0x0001);
pub const MCF_GPIO_PAR_UART_PAR_URXD0: u32 = (0x0002);
pub const MCF_GPIO_PAR_UART_PAR_URTS0: u32 = (0x0004);
pub const MCF_GPIO_PAR_UART_PAR_UCTS0: u32 = (0x0008);
#[macro_export]
macro_rules! MCF_GPIO_PAR_UART_PAR_UTXD1 { (x:expr) => { (((x)&0x0003)<<4) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_UART_PAR_URXD1 { (x:expr) => { (((x)&0x0003)<<6) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_UART_PAR_URTS1 { (x:expr) => { (((x)&0x0003)<<8) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_UART_PAR_UCTS1 { (x:expr) => { (((x)&0x0003)<<10) }; }
pub const MCF_GPIO_PAR_UART_PAR_UCTS1_GPIO: u32 = (0x0000);
pub const MCF_GPIO_PAR_UART_PAR_UCTS1_SSI_BCLK: u32 = (0x0800);
pub const MCF_GPIO_PAR_UART_PAR_UCTS1_ULPI_D7: u32 = (0x0400);
pub const MCF_GPIO_PAR_UART_PAR_UCTS1_UCTS1: u32 = (0x0C00);
pub const MCF_GPIO_PAR_UART_PAR_URTS1_GPIO: u32 = (0x0000);
pub const MCF_GPIO_PAR_UART_PAR_URTS1_SSI_FS: u32 = (0x0200);
pub const MCF_GPIO_PAR_UART_PAR_URTS1_ULPI_D6: u32 = (0x0100);
pub const MCF_GPIO_PAR_UART_PAR_URTS1_URTS1: u32 = (0x0300);
pub const MCF_GPIO_PAR_UART_PAR_URXD1_GPIO: u32 = (0x0000);
pub const MCF_GPIO_PAR_UART_PAR_URXD1_SSI_RXD: u32 = (0x0080);
pub const MCF_GPIO_PAR_UART_PAR_URXD1_ULPI_D5: u32 = (0x0040);
pub const MCF_GPIO_PAR_UART_PAR_URXD1_URXD1: u32 = (0x00C0);
pub const MCF_GPIO_PAR_UART_PAR_UTXD1_GPIO: u32 = (0x0000);
pub const MCF_GPIO_PAR_UART_PAR_UTXD1_SSI_TXD: u32 = (0x0020);
pub const MCF_GPIO_PAR_UART_PAR_UTXD1_ULPI_D4: u32 = (0x0010);
pub const MCF_GPIO_PAR_UART_PAR_UTXD1_UTXD1: u32 = (0x0030);

/* Bit definitions and macros for MCF_GPIO_PAR_QSPI */
#[macro_export]
macro_rules! MCF_GPIO_PAR_QSPI_PAR_SCK { (x:expr) => { (((x)&0x0003)<<4) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_QSPI_PAR_DOUT { (x:expr) => { (((x)&0x0003)<<6) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_QSPI_PAR_DIN { (x:expr) => { (((x)&0x0003)<<8) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_QSPI_PAR_PCS0 { (x:expr) => { (((x)&0x0003)<<10) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_QSPI_PAR_PCS1 { (x:expr) => { (((x)&0x0003)<<12) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_QSPI_PAR_PCS2 { (x:expr) => { (((x)&0x0003)<<14) }; }

/* Bit definitions and macros for MCF_GPIO_PAR_TIMER */
#[macro_export]
macro_rules! MCF_GPIO_PAR_TIMER_PAR_TIN0 { (x:expr) => { (((x)&0x03)<<0) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_TIMER_PAR_TIN1 { (x:expr) => { (((x)&0x03)<<2) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_TIMER_PAR_TIN2 { (x:expr) => { (((x)&0x03)<<4) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_TIMER_PAR_TIN3 { (x:expr) => { (((x)&0x03)<<6) }; }
pub const MCF_GPIO_PAR_TIMER_PAR_TIN3_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN3_TOUT3: u32 = (0x80);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN3_URXD2: u32 = (0x40);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN3_TIN3: u32 = (0xC0);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN2_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN2_TOUT2: u32 = (0x20);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN2_UTXD2: u32 = (0x10);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN2_TIN2: u32 = (0x30);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN1_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN1_TOUT1: u32 = (0x08);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN1_DACK1: u32 = (0x04);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN1_TIN1: u32 = (0x0C);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN0_GPIO: u32 = (0x00);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN0_TOUT0: u32 = (0x02);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN0_DREQ0: u32 = (0x01);
pub const MCF_GPIO_PAR_TIMER_PAR_TIN0_TIN0: u32 = (0x03);

/* Bit definitions and macros for MCF_GPIO_PAR_LCDDATA */
#[macro_export]
macro_rules! MCF_GPIO_PAR_LCDDATA_PAR_LD7_0 { (x:expr) => { (((x)&0x03)<<0) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_LCDDATA_PAR_LD15_8 { (x:expr) => { (((x)&0x03)<<2) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_LCDDATA_PAR_LD16 { (x:expr) => { (((x)&0x03)<<4) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_LCDDATA_PAR_LD17 { (x:expr) => { (((x)&0x03)<<6) }; }

/* Bit definitions and macros for MCF_GPIO_PAR_LCDCTL */
pub const MCF_GPIO_PAR_LCDCTL_PAR_CLS: u32 = (0x0001);
pub const MCF_GPIO_PAR_LCDCTL_PAR_PS: u32 = (0x0002);
pub const MCF_GPIO_PAR_LCDCTL_PAR_REV: u32 = (0x0004);
pub const MCF_GPIO_PAR_LCDCTL_PAR_SPL_SPR: u32 = (0x0008);
pub const MCF_GPIO_PAR_LCDCTL_PAR_CONTRAST: u32 = (0x0010);
pub const MCF_GPIO_PAR_LCDCTL_PAR_LSCLK: u32 = (0x0020);
pub const MCF_GPIO_PAR_LCDCTL_PAR_LP_HSYNC: u32 = (0x0040);
pub const MCF_GPIO_PAR_LCDCTL_PAR_FLM_VSYNC: u32 = (0x0080);
pub const MCF_GPIO_PAR_LCDCTL_PAR_ACD_OE: u32 = (0x0100);

/* Bit definitions and macros for MCF_GPIO_PAR_IRQ */
#[macro_export]
macro_rules! MCF_GPIO_PAR_IRQ_PAR_IRQ1 { (x:expr) => { (((x)&0x0003)<<4) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_IRQ_PAR_IRQ2 { (x:expr) => { (((x)&0x0003)<<6) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_IRQ_PAR_IRQ4 { (x:expr) => { (((x)&0x0003)<<8) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_IRQ_PAR_IRQ5 { (x:expr) => { (((x)&0x0003)<<10) }; }
#[macro_export]
macro_rules! MCF_GPIO_PAR_IRQ_PAR_IRQ6 { (x:expr) => { (((x)&0x0003)<<12) }; }

/* Bit definitions and macros for MCF_GPIO_MSCR_FLEXBUS */
#[macro_export]
macro_rules! MCF_GPIO_MSCR_FLEXBUS_MSCR_ADDRCTL { (x:expr) => { (((x)&0x03)<<0) }; }
#[macro_export]
macro_rules! MCF_GPIO_MSCR_FLEXBUS_MSCR_DLOWER { (x:expr) => { (((x)&0x03)<<2) }; }
#[macro_export]
macro_rules! MCF_GPIO_MSCR_FLEXBUS_MSCR_DUPPER { (x:expr) => { (((x)&0x03)<<4) }; }

/* Bit definitions and macros for MCF_GPIO_MSCR_SDRAM */
#[macro_export]
macro_rules! MCF_GPIO_MSCR_SDRAM_MSCR_SDRAM { (x:expr) => { (((x)&0x03)<<0) }; }
#[macro_export]
macro_rules! MCF_GPIO_MSCR_SDRAM_MSCR_SDCLK { (x:expr) => { (((x)&0x03)<<2) }; }
#[macro_export]
macro_rules! MCF_GPIO_MSCR_SDRAM_MSCR_SDCLKB { (x:expr) => { (((x)&0x03)<<4) }; }

/* Bit definitions and macros for MCF_GPIO_DSCR_I2C */
#[macro_export]
macro_rules! MCF_GPIO_DSCR_I2C_I2C_DSE { (x:expr) => { (((x)&0x03)<<0) }; }

/* Bit definitions and macros for MCF_GPIO_DSCR_PWM */
#[macro_export]
macro_rules! MCF_GPIO_DSCR_PWM_PWM_DSE { (x:expr) => { (((x)&0x03)<<0) }; }

/* Bit definitions and macros for MCF_GPIO_DSCR_FEC */
#[macro_export]
macro_rules! MCF_GPIO_DSCR_FEC_FEC_DSE { (x:expr) => { (((x)&0x03)<<0) }; }

/* Bit definitions and macros for MCF_GPIO_DSCR_UART */
#[macro_export]
macro_rules! MCF_GPIO_DSCR_UART_UART0_DSE { (x:expr) => { (((x)&0x03)<<0) }; }
#[macro_export]
macro_rules! MCF_GPIO_DSCR_UART_UART1_DSE { (x:expr) => { (((x)&0x03)<<2) }; }

/* Bit definitions and macros for MCF_GPIO_DSCR_QSPI */
#[macro_export]
macro_rules! MCF_GPIO_DSCR_QSPI_QSPI_DSE { (x:expr) => { (((x)&0x03)<<0) }; }

/* Bit definitions and macros for MCF_GPIO_DSCR_TIMER */
#[macro_export]
macro_rules! MCF_GPIO_DSCR_TIMER_TIMER_DSE { (x:expr) => { (((x)&0x03)<<0) }; }

/* Bit definitions and macros for MCF_GPIO_DSCR_SSI */
#[macro_export]
macro_rules! MCF_GPIO_DSCR_SSI_SSI_DSE { (x:expr) => { (((x)&0x03)<<0) }; }

/* Bit definitions and macros for MCF_GPIO_DSCR_LCD */
#[macro_export]
macro_rules! MCF_GPIO_DSCR_LCD_LCD_DSE { (x:expr) => { (((x)&0x03)<<0) }; }

/* Bit definitions and macros for MCF_GPIO_DSCR_DEBUG */
#[macro_export]
macro_rules! MCF_GPIO_DSCR_DEBUG_DEBUG_DSE { (x:expr) => { (((x)&0x03)<<0) }; }

/* Bit definitions and macros for MCF_GPIO_DSCR_CLKRST */
#[macro_export]
macro_rules! MCF_GPIO_DSCR_CLKRST_CLKRST_DSE { (x:expr) => { (((x)&0x03)<<0) }; }

/* Bit definitions and macros for MCF_GPIO_DSCR_IRQ */
#[macro_export]
macro_rules! MCF_GPIO_DSCR_IRQ_IRQ_DSE { (x:expr) => { (((x)&0x03)<<0) }; }

/*
 * Generic GPIO support
 */
pub const MCFGPIO_PODR: u32 = MCFGPIO_PODR_FECH;
pub const MCFGPIO_PDDR: u32 = MCFGPIO_PDDR_FECH;
pub const MCFGPIO_PPDR: u32 = MCFGPIO_PPDSDR_FECH;
pub const MCFGPIO_SETR: u32 = MCFGPIO_PPDSDR_FECH;
pub const MCFGPIO_CLRR: u32 = MCFGPIO_PCLRR_FECH;

pub const MCFGPIO_PIN_MAX: u32 = 136;
pub const MCFGPIO_IRQ_MAX: u32 = 8;
pub const MCFGPIO_IRQ_VECBASE: u32 = MCFINT_VECBASE;

/*********************************************************************
 *
 * Phase Locked Loop (PLL)
 *
 *********************************************************************/

/* Register read/write macros */
pub const MCF_PLL_PODR: u32 = 0xFC0C0000;
pub const MCF_PLL_PLLCR: u32 = 0xFC0C0004;
pub const MCF_PLL_PMDR: u32 = 0xFC0C0008;
pub const MCF_PLL_PFDR: u32 = 0xFC0C000C;

/* Bit definitions and macros for MCF_PLL_PODR */
#[macro_export]
macro_rules! MCF_PLL_PODR_BUSDIV { (x:expr) => { (((x)&0x0F)<<0) }; }
#[macro_export]
macro_rules! MCF_PLL_PODR_CPUDIV { (x:expr) => { (((x)&0x0F)<<4) }; }

/* Bit definitions and macros for MCF_PLL_PLLCR */
#[macro_export]
macro_rules! MCF_PLL_PLLCR_DITHDEV { (x:expr) => { (((x)&0x07)<<0) }; }
pub const MCF_PLL_PLLCR_DITHEN: u32 = (0x80);

/* Bit definitions and macros for MCF_PLL_PMDR */
#[macro_export]
macro_rules! MCF_PLL_PMDR_MODDIV { (x:expr) => { (((x)&0xFF)<<0) }; }

/* Bit definitions and macros for MCF_PLL_PFDR */
#[macro_export]
macro_rules! MCF_PLL_PFDR_MFD { (x:expr) => { (((x)&0xFF)<<0) }; }

/*********************************************************************
 *
 * System Control Module Registers (SCM)
 *
 *********************************************************************/

/* Register read/write macros */
pub const MCF_SCM_MPR: u32 = 0xFC000000;
pub const MCF_SCM_PACRA: u32 = 0xFC000020;
pub const MCF_SCM_PACRB: u32 = 0xFC000024;
pub const MCF_SCM_PACRC: u32 = 0xFC000028;
pub const MCF_SCM_PACRD: u32 = 0xFC00002C;
pub const MCF_SCM_PACRE: u32 = 0xFC000040;
pub const MCF_SCM_PACRF: u32 = 0xFC000044;

pub const MCF_SCM_BCR: u32 = 0xFC040024;

/*********************************************************************
 *
 * SDRAM Controller (SDRAMC)
 *
 *********************************************************************/

/* Register read/write macros */
pub const MCF_SDRAMC_SDMR: u32 = 0xFC0B8000;
pub const MCF_SDRAMC_SDCR: u32 = 0xFC0B8004;
pub const MCF_SDRAMC_SDCFG1: u32 = 0xFC0B8008;
pub const MCF_SDRAMC_SDCFG2: u32 = 0xFC0B800C;
pub const MCF_SDRAMC_LIMP_FIX: u32 = 0xFC0B8080;
pub const MCF_SDRAMC_SDDS: u32 = 0xFC0B8100;
pub const MCF_SDRAMC_SDCS0: u32 = 0xFC0B8110;
pub const MCF_SDRAMC_SDCS1: u32 = 0xFC0B8114;
pub const MCF_SDRAMC_SDCS2: u32 = 0xFC0B8118;
pub const MCF_SDRAMC_SDCS3: u32 = 0xFC0B811C;

/* Bit definitions and macros for MCF_SDRAMC_SDMR */
pub const MCF_SDRAMC_SDMR_CMD: u32 = (0x00010000);
#[macro_export]
macro_rules! MCF_SDRAMC_SDMR_AD { (x:expr) => { (((x)&0x00000FFF)<<18) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDMR_BNKAD { (x:expr) => { (((x)&0x00000003)<<30) }; }
pub const MCF_SDRAMC_SDMR_BNKAD_LMR: u32 = (0x00000000);
pub const MCF_SDRAMC_SDMR_BNKAD_LEMR: u32 = (0x40000000);

/* Bit definitions and macros for MCF_SDRAMC_SDCR */
pub const MCF_SDRAMC_SDCR_IPALL: u32 = (0x00000002);
pub const MCF_SDRAMC_SDCR_IREF: u32 = (0x00000004);
#[macro_export]
macro_rules! MCF_SDRAMC_SDCR_DQS_OE { (x:expr) => { (((x)&0x0000000F)<<8) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCR_PS { (x:expr) => { (((x)&0x00000003)<<12) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCR_RCNT { (x:expr) => { (((x)&0x0000003F)<<16) }; }
pub const MCF_SDRAMC_SDCR_OE_RULE: u32 = (0x00400000);
#[macro_export]
macro_rules! MCF_SDRAMC_SDCR_MUX { (x:expr) => { (((x)&0x00000003)<<24) }; }
pub const MCF_SDRAMC_SDCR_REF: u32 = (0x10000000);
pub const MCF_SDRAMC_SDCR_DDR: u32 = (0x20000000);
pub const MCF_SDRAMC_SDCR_CKE: u32 = (0x40000000);
pub const MCF_SDRAMC_SDCR_MODE_EN: u32 = (0x80000000);
pub const MCF_SDRAMC_SDCR_PS_16: u32 = (0x00002000);
pub const MCF_SDRAMC_SDCR_PS_32: u32 = (0x00000000);

/* Bit definitions and macros for MCF_SDRAMC_SDCFG1 */
#[macro_export]
macro_rules! MCF_SDRAMC_SDCFG1_WTLAT { (x:expr) => { (((x)&0x00000007)<<4) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCFG1_REF2ACT { (x:expr) => { (((x)&0x0000000F)<<8) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCFG1_PRE2ACT { (x:expr) => { (((x)&0x00000007)<<12) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCFG1_ACT2RW { (x:expr) => { (((x)&0x00000007)<<16) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCFG1_RDLAT { (x:expr) => { (((x)&0x0000000F)<<20) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCFG1_SWT2RD { (x:expr) => { (((x)&0x00000007)<<24) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCFG1_SRD2RW { (x:expr) => { (((x)&0x0000000F)<<28) }; }

/* Bit definitions and macros for MCF_SDRAMC_SDCFG2 */
#[macro_export]
macro_rules! MCF_SDRAMC_SDCFG2_BL { (x:expr) => { (((x)&0x0000000F)<<16) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCFG2_BRD2WT { (x:expr) => { (((x)&0x0000000F)<<20) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCFG2_BWT2RW { (x:expr) => { (((x)&0x0000000F)<<24) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCFG2_BRD2PRE { (x:expr) => { (((x)&0x0000000F)<<28) }; }

/* Device Errata - LIMP mode work around */
pub const MCF_SDRAMC_REFRESH: u32 = (0x40000000);

/* Bit definitions and macros for MCF_SDRAMC_SDDS */
#[macro_export]
macro_rules! MCF_SDRAMC_SDDS_SB_D { (x:expr) => { (((x)&0x00000003)<<0) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDDS_SB_S { (x:expr) => { (((x)&0x00000003)<<2) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDDS_SB_A { (x:expr) => { (((x)&0x00000003)<<4) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDDS_SB_C { (x:expr) => { (((x)&0x00000003)<<6) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDDS_SB_E { (x:expr) => { (((x)&0x00000003)<<8) }; }

/* Bit definitions and macros for MCF_SDRAMC_SDCS */
#[macro_export]
macro_rules! MCF_SDRAMC_SDCS_CSSZ { (x:expr) => { (((x)&0x0000001F)<<0) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCS_BASE { (x:expr) => { (((x)&0x00000FFF)<<20) }; }
#[macro_export]
macro_rules! MCF_SDRAMC_SDCS_BA { (x:expr) => { ((x)&0xFFF00000) }; }
pub const MCF_SDRAMC_SDCS_CSSZ_DIABLE: u32 = (0x00000000);
pub const MCF_SDRAMC_SDCS_CSSZ_1MBYTE: u32 = (0x00000013);
pub const MCF_SDRAMC_SDCS_CSSZ_2MBYTE: u32 = (0x00000014);
pub const MCF_SDRAMC_SDCS_CSSZ_4MBYTE: u32 = (0x00000015);
pub const MCF_SDRAMC_SDCS_CSSZ_8MBYTE: u32 = (0x00000016);
pub const MCF_SDRAMC_SDCS_CSSZ_16MBYTE: u32 = (0x00000017);
pub const MCF_SDRAMC_SDCS_CSSZ_32MBYTE: u32 = (0x00000018);
pub const MCF_SDRAMC_SDCS_CSSZ_64MBYTE: u32 = (0x00000019);
pub const MCF_SDRAMC_SDCS_CSSZ_128MBYTE: u32 = (0x0000001A);
pub const MCF_SDRAMC_SDCS_CSSZ_256MBYTE: u32 = (0x0000001B);
pub const MCF_SDRAMC_SDCS_CSSZ_512MBYTE: u32 = (0x0000001C);
pub const MCF_SDRAMC_SDCS_CSSZ_1GBYTE: u32 = (0x0000001D);
pub const MCF_SDRAMC_SDCS_CSSZ_2GBYTE: u32 = (0x0000001E);
pub const MCF_SDRAMC_SDCS_CSSZ_4GBYTE: u32 = (0x0000001F);

/*
 * Edge Port Module (EPORT)
 */
pub const MCFEPORT_EPPAR: u32 = (0xFC094000);
pub const MCFEPORT_EPDDR: u32 = (0xFC094002);
pub const MCFEPORT_EPIER: u32 = (0xFC094003);
pub const MCFEPORT_EPDR: u32 = (0xFC094004);
pub const MCFEPORT_EPPDR: u32 = (0xFC094005);
pub const MCFEPORT_EPFR: u32 = (0xFC094006);

/*
 * I2C Module
 */
pub const MCFI2C_BASE0: u32 = (0xFc058000);
pub const MCFI2C_SIZE0: u32 = 0x40;

/********************************************************************/
// #endif	/* m53xxsim_h */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
