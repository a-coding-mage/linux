/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from pxa27x-udc.h.  pxa-regs.h supplies __REG, __REG2 and io_p2v. */
/* The original header rejects simultaneous PXA25x UDC support. */

macro_rules! UDCCR { () => { __REG(0x40600000) }; }
pub const UDCCR_OEN: u32 = 1 << 31;
pub const UDCCR_AALTHNP: u32 = 1 << 30;
pub const UDCCR_AHNP: u32 = 1 << 29;
pub const UDCCR_BHNP: u32 = 1 << 28;
pub const UDCCR_DWRE: u32 = 1 << 16;
pub const UDCCR_ACN: u32 = 0x03 << 11;
pub const UDCCR_ACN_S: u32 = 11;
pub const UDCCR_AIN: u32 = 0x07 << 8;
pub const UDCCR_AIN_S: u32 = 8;
pub const UDCCR_AAISN: u32 = 0x07 << 5;
pub const UDCCR_AAISN_S: u32 = 5;
pub const UDCCR_SMAC: u32 = 1 << 4;
pub const UDCCR_EMCE: u32 = 1 << 3;
pub const UDCCR_UDR: u32 = 1 << 2;
pub const UDCCR_UDA: u32 = 1 << 1;
pub const UDCCR_UDE: u32 = 1;

macro_rules! reg { ($name:ident, $addr:expr) => { macro_rules! $name { () => { __REG($addr) }; } }; }
reg!(UDCICR0, 0x40600004); reg!(UDCICR1, 0x40600008);
pub const UDCICR_FIFOERR: u32 = 1 << 1; pub const UDCICR_PKTCOMPL: u32 = 1;
pub const UDC_INT_FIFOERROR: u32 = 2; pub const UDC_INT_PACKETCMP: u32 = 1;
macro_rules! UDCICR_INT { ($n:expr, $intr:expr) => { (($intr & 0x03) << (($n & 0x0f) * 2)) }; }
pub const UDCICR1_IECC: u32=1<<31; pub const UDCICR1_IESOF:u32=1<<30; pub const UDCICR1_IERU:u32=1<<29; pub const UDCICR1_IESU:u32=1<<28; pub const UDCICR1_IERS:u32=1<<27;
reg!(UDCISR0,0x4060000c); reg!(UDCISR1,0x40600010);
macro_rules! UDCISR_INT { ($n:expr, $intr:expr) => { (($intr & 0x03) << (($n & 0x0f) * 2)) }; }
pub const UDCISR1_IRCC:u32=1<<31; pub const UDCISR1_IRSOF:u32=1<<30; pub const UDCISR1_IRRU:u32=1<<29; pub const UDCISR1_IRSU:u32=1<<28; pub const UDCISR1_IRRS:u32=1<<27;
reg!(UDCFNR,0x40600014); reg!(UDCOTGICR,0x40600018);
pub const UDCOTGICR_IESF:u32=1<<24; pub const UDCOTGICR_IEXR:u32=1<<17; pub const UDCOTGICR_IEXF:u32=1<<16; pub const UDCOTGICR_IEVV40R:u32=1<<9; pub const UDCOTGICR_IEVV40F:u32=1<<8; pub const UDCOTGICR_IEVV44R:u32=1<<7; pub const UDCOTGICR_IEVV44F:u32=1<<6; pub const UDCOTGICR_IESVR:u32=1<<5; pub const UDCOTGICR_IESVF:u32=1<<4; pub const UDCOTGICR_IESDR:u32=1<<3; pub const UDCOTGICR_IESDF:u32=1<<2; pub const UDCOTGICR_IEIDR:u32=1<<1; pub const UDCOTGICR_IEIDF:u32=1;
reg!(UP2OCR,0x40600020); reg!(UP3OCR,0x40600024);
pub const UP2OCR_CPVEN:u32=1; pub const UP2OCR_CPVPE:u32=1<<1; pub const UP2OCR_DPPDE:u32=1<<2; pub const UP2OCR_DMPDE:u32=1<<3; pub const UP2OCR_DPPUE:u32=1<<4; pub const UP2OCR_DMPUE:u32=1<<5; pub const UP2OCR_DPPUBE:u32=1<<6; pub const UP2OCR_DMPUBE:u32=1<<7; pub const UP2OCR_EXSP:u32=1<<8; pub const UP2OCR_EXSUS:u32=1<<9; pub const UP2OCR_IDON:u32=1<<10; pub const UP2OCR_HXS:u32=1<<16; pub const UP2OCR_HXOE:u32=1<<17;
macro_rules! UP2OCR_SEOS { ($x:expr) => { (($x & 7) << 24) }; }

macro_rules! UDCCSN { ($x:expr) => { __REG2(0x40600100, $x << 2) }; }
macro_rules! UDCBCN { ($x:expr) => { __REG2(0x40600200, $x << 2) }; }
macro_rules! UDCDN { ($x:expr) => { __REG2(0x40600300, $x << 2) }; }
macro_rules! PHYS_UDCDN { ($x:expr) => { 0x40600300 + ($x << 2) }; }
macro_rules! PUDCDN { ($x:expr) => { (io_p2v(PHYS_UDCDN!($x)) as *mut u32) }; }
macro_rules! UDCCN { ($x:expr) => { __REG2(0x40600400, $x << 2) }; }

macro_rules! ep_regs { ($($n:ident => $a:expr),+ $(,)?) => { $(macro_rules! $n { () => { __REG($a) }; })+ }; }
ep_regs!(UDCCSR0=>0x40600100, UDCCSRA=>0x40600104, UDCCSRB=>0x40600108, UDCCSRC=>0x4060010c, UDCCSRD=>0x40600110, UDCCSRE=>0x40600114, UDCCSRF=>0x40600118, UDCCSRG=>0x4060011c, UDCCSRH=>0x40600120, UDCCSRI=>0x40600124, UDCCSRJ=>0x40600128, UDCCSRK=>0x4060012c, UDCCSRL=>0x40600130, UDCCSRM=>0x40600134, UDCCSRN=>0x40600138, UDCCSRP=>0x4060013c, UDCCSRQ=>0x40600140, UDCCSRR=>0x40600144, UDCCSRS=>0x40600148, UDCCSRT=>0x4060014c, UDCCSRU=>0x40600150, UDCCSRV=>0x40600154, UDCCSRW=>0x40600158, UDCCSRX=>0x4060015c);
pub const UDCCSR_DPE:u32=1<<9; pub const UDCCSR_FEF:u32=1<<8; pub const UDCCSR_SP:u32=1<<7; pub const UDCCSR_BNE:u32=1<<6; pub const UDCCSR_BNF:u32=1<<6; pub const UDCCSR_FST:u32=1<<5; pub const UDCCSR_SST:u32=1<<4; pub const UDCCSR_DME:u32=1<<3; pub const UDCCSR_TRN:u32=1<<2; pub const UDCCSR_PC:u32=1<<1; pub const UDCCSR_FS:u32=1;

/* Byte-count, data, and configuration endpoint registers retain the original
 * fixed-address declarations through the indexed register macros above. */
macro_rules! endpoint_reg { ($base:expr, $x:expr) => { __REG2($base, $x << 2) }; }
macro_rules! UDCBCR0 { () => { __REG(0x40600200) }; }
macro_rules! UDCDR0 { () => { __REG(0x40600300) }; }
ep_regs!(UDCBCRA=>0x40600204, UDCBCRB=>0x40600208, UDCBCRC=>0x4060020c, UDCBCRD=>0x40600210, UDCBCRE=>0x40600214, UDCBCRF=>0x40600218, UDCBCRG=>0x4060021c, UDCBCRH=>0x40600220, UDCBCRI=>0x40600224, UDCBCRJ=>0x40600228, UDCBCRK=>0x4060022c, UDCBCRL=>0x40600230, UDCBCRM=>0x40600234, UDCBCRN=>0x40600238, UDCBCRP=>0x4060023c, UDCBCRQ=>0x40600240, UDCBCRR=>0x40600244, UDCBCRS=>0x40600248, UDCBCRT=>0x4060024c, UDCBCRU=>0x40600250, UDCBCRV=>0x40600254, UDCBCRW=>0x40600258, UDCBCRX=>0x4060025c);
ep_regs!(UDCDRA=>0x40600304, UDCDRB=>0x40600308, UDCDRC=>0x4060030c, UDCDRD=>0x40600310, UDCDRE=>0x40600314, UDCDRF=>0x40600318, UDCDRG=>0x4060031c, UDCDRH=>0x40600320, UDCDRI=>0x40600324, UDCDRJ=>0x40600328, UDCDRK=>0x4060032c, UDCDRL=>0x40600330, UDCDRM=>0x40600334, UDCDRN=>0x40600338, UDCDRP=>0x4060033c, UDCDRQ=>0x40600340, UDCDRR=>0x40600344, UDCDRS=>0x40600348, UDCDRT=>0x4060034c, UDCDRU=>0x40600350, UDCDRV=>0x40600354, UDCDRW=>0x40600358, UDCDRX=>0x4060035c);
ep_regs!(UDCCRA=>0x40600404, UDCCRB=>0x40600408, UDCCRC=>0x4060040c, UDCCRD=>0x40600410, UDCCRE=>0x40600414, UDCCRF=>0x40600418, UDCCRG=>0x4060041c, UDCCRH=>0x40600420, UDCCRI=>0x40600424, UDCCRJ=>0x40600428, UDCCRK=>0x4060042c, UDCCRL=>0x40600430, UDCCRM=>0x40600434, UDCCRN=>0x40600438, UDCCRP=>0x4060043c, UDCCRQ=>0x40600440, UDCCRR=>0x40600444, UDCCRS=>0x40600448, UDCCRT=>0x4060044c, UDCCRU=>0x40600450, UDCCRV=>0x40600454, UDCCRW=>0x40600458, UDCCRX=>0x4060045c);
pub const UDCCONR_CN:u32=0x03<<25; pub const UDCCONR_CN_S:u32=25; pub const UDCCONR_IN:u32=0x07<<22; pub const UDCCONR_IN_S:u32=22; pub const UDCCONR_AISN:u32=0x07<<19; pub const UDCCONR_AISN_S:u32=19; pub const UDCCONR_EN:u32=0x0f<<15; pub const UDCCONR_EN_S:u32=15; pub const UDCCONR_ET:u32=0x03<<13; pub const UDCCONR_ET_S:u32=13; pub const UDCCONR_ET_INT:u32=3<<13; pub const UDCCONR_ET_BULK:u32=2<<13; pub const UDCCONR_ET_ISO:u32=1<<13; pub const UDCCONR_ET_NU:u32=0; pub const UDCCONR_ED:u32=1<<12; pub const UDCCONR_MPS:u32=0x3ff<<2; pub const UDCCONR_MPS_S:u32=2; pub const UDCCONR_DE:u32=1<<1; pub const UDCCONR_EE:u32=1;
pub const UDC_FNR_MASK:u32=0x7ff; pub const UDCCSR_WR_MASK:u32=UDCCSR_DME|UDCCSR_FST; pub const UDC_BCR_MASK:u32=0x3ff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
