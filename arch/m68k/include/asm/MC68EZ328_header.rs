/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of MC68EZ328.h.  Memory-mapped register access is volatile. */

#[inline] pub unsafe fn byte_ref(addr: usize) -> u8 { core::ptr::read_volatile(addr as *const u8) }
#[inline] pub unsafe fn word_ref(addr: usize) -> u16 { core::ptr::read_volatile(addr as *const u16) }
#[inline] pub unsafe fn long_ref(addr: usize) -> usize { core::ptr::read_volatile(addr as *const usize) }
#[inline] pub const fn put_field(field: usize, val: usize, shift: u32, mask: usize) -> usize { (val << shift) & mask }
#[inline] pub const fn get_field(reg: usize, shift: u32, mask: usize) -> usize { (reg & mask) >> shift }

macro_rules! addr { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: usize = $v;)* }; }
macro_rules! bits { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: usize = $v;)* }; }
macro_rules! reg { ($($n:ident : $t:ty = $a:ident),* $(,)?) => { $(#[inline] pub unsafe fn $n() -> $t { core::ptr::read_volatile($a as *const $t) })* }; }

addr!(SCR_ADDR=0xfffff000, MRR_ADDR=0xfffff004, CSGBA_ADDR=0xfffff100, CSGBB_ADDR=0xfffff102, CSGBC_ADDR=0xfffff104, CSGBD_ADDR=0xfffff106,
 CSA_ADDR=0xfffff110, CSB_ADDR=0xfffff112, CSC_ADDR=0xfffff114, CSD_ADDR=0xfffff116, EMUCS_ADDR=0xfffff118,
 PLLCR_ADDR=0xfffff200, PLLFSR_ADDR=0xfffff202, PCTRL_ADDR=0xfffff207, IVR_ADDR=0xfffff300, ICR_ADDR=0xfffff302, IMR_ADDR=0xfffff304, ISR_ADDR=0xfffff30c, IPR_ADDR=0xfffff30c,
 PADIR_ADDR=0xfffff400, PADATA_ADDR=0xfffff401, PAPUEN_ADDR=0xfffff402, PBDIR_ADDR=0xfffff408, PBDATA_ADDR=0xfffff409, PBPUEN_ADDR=0xfffff40a, PBSEL_ADDR=0xfffff40b,
 PCDIR_ADDR=0xfffff410, PCDATA_ADDR=0xfffff411, PCPDEN_ADDR=0xfffff412, PCSEL_ADDR=0xfffff413, PDDIR_ADDR=0xfffff418, PDDATA_ADDR=0xfffff419, PDPUEN_ADDR=0xfffff41a, PDSEL_ADDR=0xfffff41b, PDPOL_ADDR=0xfffff41c, PDIRQEN_ADDR=0xfffff41d, PDKBEN_ADDR=0xfffff41e, PDIQEG_ADDR=0xfffff41f,
 PEDIR_ADDR=0xfffff420, PEDATA_ADDR=0xfffff421, PEPUEN_ADDR=0xfffff422, PESEL_ADDR=0xfffff423, PFDIR_ADDR=0xfffff428, PFDATA_ADDR=0xfffff429, PFPUEN_ADDR=0xfffff42a, PFSEL_ADDR=0xfffff42b, PGDIR_ADDR=0xfffff430, PGDATA_ADDR=0xfffff431, PGPUEN_ADDR=0xfffff432, PGSEL_ADDR=0xfffff433,
 PWMC_ADDR=0xfffff500, PWMS_ADDR=0xfffff502, PWMP_ADDR=0xfffff504, PWMCNT_ADDR=0xfffff505, TCTL_ADDR=0xfffff600, TPRER_ADDR=0xfffff602, TCMP_ADDR=0xfffff604, TCR_ADDR=0xfffff606, TCN_ADDR=0xfffff608, TSTAT_ADDR=0xfffff60a,
 SPIMDATA_ADDR=0xfffff800, SPIMCONT_ADDR=0xfffff802, USTCNT_ADDR=0xfffff900, UBAUD_ADDR=0xfffff902, URX_ADDR=0xfffff904, URX_RXDATA_ADDR=0xfffff905, UTX_ADDR=0xfffff906, UTX_TXDATA_ADDR=0xfffff907, UMISC_ADDR=0xfffff908, NIPR_ADDR=0xfffff90a,
 LSSA_ADDR=0xfffffa00, LVPW_ADDR=0xfffffa05, LXMAX_ADDR=0xfffffa08, LYMAX_ADDR=0xfffffa0a, LCXP_ADDR=0xfffffa18, LCYP_ADDR=0xfffffa1a, LCWCH_ADDR=0xfffffa1c, LBLKC_ADDR=0xfffffa1f, LPICF_ADDR=0xfffffa20, LPOLCF_ADDR=0xfffffa21, LACDRC_ADDR=0xfffffa23, LPXCD_ADDR=0xfffffa25, LCKCON_ADDR=0xfffffa27, LRRA_ADDR=0xfffffa29, LPOSR_ADDR=0xfffffa2d, LFRCM_ADDR=0xfffffa31, LGPMR_ADDR=0xfffffa33, PWMR_ADDR=0xfffffa36,
 RTCTIME_ADDR=0xfffffb00, RTCALRM_ADDR=0xfffffb04, WATCHDOG_ADDR=0xfffffb0a, RTCCTL_ADDR=0xfffffb0c, RTCISR_ADDR=0xfffffb0e, RTCIENR_ADDR=0xfffffb10, STPWCH_ADDR=0xfffffb12, DAYR_ADDR=0xfffffb1a, DAYALARM_ADDR=0xfffffb1c, DRAMMC_ADDR=0xfffffc00, DRAMC_ADDR=0xfffffc02,
 ICEMACR_ADDR=0xfffffd00, ICEMAMR_ADDR=0xfffffd04, ICEMCCR_ADDR=0xfffffd08, ICEMCMR_ADDR=0xfffffd0a, ICEMCR_ADDR=0xfffffd0c, ICEMSR_ADDR=0xfffffd0e);

bits!(SCR_WDTH8=0x01,SCR_DMAP=0x04,SCR_SO=0x08,SCR_BETEN=0x10,SCR_PRV=0x20,SCR_WPV=0x40,SCR_BETO=0x80,
 CSA_EN=1,CSA_SIZ_MASK=0x000e,CSA_SIZ_SHIFT=1,CSA_WS_MASK=0x0070,CSA_WS_SHIFT=4,CSA_BSW=0x80,CSA_FLASH=0x100,CSA_RO=0x8000,
 PLLCR_DISPLL=8,PLLCR_CLKEN=0x10,PLLCR_PRESC=0x20,PLLCR_SYSCLK_SEL_MASK=0x700,PLLCR_SYSCLK_SEL_SHIFT=8,PLLCR_LCDCLK_SEL_MASK=0x3800,PLLCR_LCDCLK_SEL_SHIFT=11,
 PLLFSR_PC_MASK=0xff,PLLFSR_PC_SHIFT=0,PLLFSR_QC_MASK=0xf00,PLLFSR_QC_SHIFT=8,PLLFSR_PROT=0x4000,PLLFSR_CLK32=0x8000,
 PCTRL_WIDTH_MASK=0x1f,PCTRL_WIDTH_SHIFT=0,PCTRL_PCEN=0x80,IVR_VECTOR_MASK=0xf8,ICR_POL5=0x80,ICR_ET6=0x100,ICR_ET3=0x200,ICR_ET2=0x400,ICR_ET1=0x800,ICR_POL6=0x1000,ICR_POL3=0x2000,ICR_POL2=0x4000,ICR_POL1=0x8000);

pub const SPI_IRQ_NUM: usize=0; pub const TMR_IRQ_NUM: usize=1; pub const UART_IRQ_NUM: usize=2; pub const WDT_IRQ_NUM: usize=3; pub const RTC_IRQ_NUM: usize=4; pub const KB_IRQ_NUM: usize=6; pub const PWM_IRQ_NUM: usize=7; pub const INT0_IRQ_NUM: usize=8; pub const INT1_IRQ_NUM: usize=9; pub const INT2_IRQ_NUM: usize=10; pub const INT3_IRQ_NUM: usize=11; pub const IRQ1_IRQ_NUM: usize=16; pub const IRQ2_IRQ_NUM: usize=17; pub const IRQ3_IRQ_NUM: usize=18; pub const IRQ6_IRQ_NUM: usize=19; pub const IRQ5_IRQ_NUM: usize=20; pub const SAM_IRQ_NUM: usize=22; pub const EMIQ_IRQ_NUM: usize=23;
 pub const SPIM_IRQ_NUM:usize=SPI_IRQ_NUM; pub const TMR1_IRQ_NUM:usize=TMR_IRQ_NUM;
 macro_rules! irq_bits { ($($n:ident:$v:ident),*) => { $(pub const $n:usize=1usize<<$v;)* }; }
 irq_bits!(IMR_MSPI:SPI_IRQ_NUM,IMR_MTMR:TMR_IRQ_NUM,IMR_MUART:UART_IRQ_NUM,IMR_MWDT:WDT_IRQ_NUM,IMR_MRTC:RTC_IRQ_NUM,IMR_MKB:KB_IRQ_NUM,IMR_MPWM:PWM_IRQ_NUM,IMR_MINT0:INT0_IRQ_NUM,IMR_MINT1:INT1_IRQ_NUM,IMR_MINT2:INT2_IRQ_NUM,IMR_MINT3:INT3_IRQ_NUM,IMR_MIRQ1:IRQ1_IRQ_NUM,IMR_MIRQ2:IRQ2_IRQ_NUM,IMR_MIRQ3:IRQ3_IRQ_NUM,IMR_MIRQ6:IRQ6_IRQ_NUM,IMR_MIRQ5:IRQ5_IRQ_NUM,IMR_MSAM:SAM_IRQ_NUM,IMR_MEMIQ:EMIQ_IRQ_NUM);
 pub const IMR_MSPIM:usize=IMR_MSPI; pub const IMR_MTMR1:usize=IMR_MTMR;

#[repr(C, packed)] pub struct m68328_uart { pub ustcnt:u16, pub ubaud:u16, pub urx: RegisterUnion, pub utx:RegisterUnion, pub umisc:u16, pub nipr:u16, pub pad1:u16, pub pad2:u16 }
#[repr(C)] pub union RegisterUnion { pub w:u16, pub b:RegisterBytes }
#[repr(C)] pub struct RegisterBytes { pub status:u8, pub data:u8 }

reg!(SCR:u8=SCR_ADDR, MRR:usize=MRR_ADDR, CSGBA:u16=CSGBA_ADDR,CSGBB:u16=CSGBB_ADDR,CSGBC:u16=CSGBC_ADDR,CSGBD:u16=CSGBD_ADDR,CSA:u16=CSA_ADDR,CSB:u16=CSB_ADDR,CSC:u16=CSC_ADDR,CSD:u16=CSD_ADDR,EMUCS:u16=EMUCS_ADDR,PLLCR:u16=PLLCR_ADDR,PLLFSR:u16=PLLFSR_ADDR,PCTRL:u8=PCTRL_ADDR,IVR:u8=IVR_ADDR,ICR:u16=ICR_ADDR,IMR:usize=IMR_ADDR,ISR:usize=ISR_ADDR,IPR:usize=IPR_ADDR);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
