/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of SA-1100.h.  The original header is a hardware
 * definition header; __REG, __PREG, io_p2v, Fld, FShft, FSize and FAlnMsk
 * are supplied by the surrounding architecture bindings.
 */

/* SA1100 CS line to physical address */
pub const SA1100_CS0_PHYS: u32 = 0x00000000;
pub const SA1100_CS1_PHYS: u32 = 0x08000000;
pub const SA1100_CS2_PHYS: u32 = 0x10000000;
pub const SA1100_CS3_PHYS: u32 = 0x18000000;
pub const SA1100_CS4_PHYS: u32 = 0x40000000;
pub const SA1100_CS5_PHYS: u32 = 0x48000000;

/* PCMCIA spaces */
pub const PCMCIAPrtSp: u32 = 0x04000000;
pub const PCMCIASp: u32 = 4 * PCMCIAPrtSp;
pub const PCMCIAIOSp: u32 = PCMCIAPrtSp;
pub const PCMCIAAttrSp: u32 = PCMCIAPrtSp;
pub const PCMCIAMemSp: u32 = PCMCIAPrtSp;
pub const PCMCIA0Sp: u32 = PCMCIASp;
pub const PCMCIA0IOSp: u32 = PCMCIAIOSp;
pub const PCMCIA0AttrSp: u32 = PCMCIAAttrSp;
pub const PCMCIA0MemSp: u32 = PCMCIAMemSp;
pub const PCMCIA1Sp: u32 = PCMCIASp;
pub const PCMCIA1IOSp: u32 = PCMCIAIOSp;
pub const PCMCIA1AttrSp: u32 = PCMCIAAttrSp;
pub const PCMCIA1MemSp: u32 = PCMCIAMemSp;

#[inline] pub const fn _PCMCIA(nb: u32) -> u32 { 0x20000000 + nb * PCMCIASp }
#[inline] pub const fn _PCMCIAIO(nb: u32) -> u32 { _PCMCIA(nb) }
#[inline] pub const fn _PCMCIAAttr(nb: u32) -> u32 { _PCMCIA(nb) + 2 * PCMCIAPrtSp }
#[inline] pub const fn _PCMCIAMem(nb: u32) -> u32 { _PCMCIA(nb) + 3 * PCMCIAPrtSp }
pub const _PCMCIA0: u32 = _PCMCIA(0); pub const _PCMCIA0IO: u32 = _PCMCIAIO(0);
pub const _PCMCIA0Attr: u32 = _PCMCIAAttr(0); pub const _PCMCIA0Mem: u32 = _PCMCIAMem(0);
pub const _PCMCIA1: u32 = _PCMCIA(1); pub const _PCMCIA1IO: u32 = _PCMCIAIO(1);
pub const _PCMCIA1Attr: u32 = _PCMCIAAttr(1); pub const _PCMCIA1Mem: u32 = _PCMCIAMem(1);

/* Field helpers preserve the bitfield.h interface. */
#[inline] pub const fn Fld(size: u32, shift: u32) -> u32 { ((1u32 << size) - 1) << shift }
#[inline] pub const fn FShft(field: u32) -> u32 { field.trailing_zeros() }
#[inline] pub const fn FSize(field: u32) -> u32 { field.count_ones() }
#[inline] pub const fn FAlnMsk(field: u32) -> u32 { field }

/* UDC */
pub const Ser0UDCCR: u32 = 0x80000000; pub const Ser0UDCAR: u32 = 0x80000004;
pub const Ser0UDCOMP: u32 = 0x80000008; pub const Ser0UDCIMP: u32 = 0x8000000c;
pub const Ser0UDCCS0: u32 = 0x80000010; pub const Ser0UDCCS1: u32 = 0x80000014;
pub const Ser0UDCCS2: u32 = 0x80000018; pub const Ser0UDCD0: u32 = 0x8000001c;
pub const Ser0UDCWC: u32 = 0x80000020; pub const Ser0UDCDR: u32 = 0x80000028;
pub const Ser0UDCSR: u32 = 0x80000030;
pub const UDCCR_UDD:u32=1; pub const UDCCR_UDA:u32=2; pub const UDCCR_RESIM:u32=4;
pub const UDCCR_EIM:u32=8; pub const UDCCR_RIM:u32=16; pub const UDCCR_TIM:u32=32;
pub const UDCCR_SRM:u32=64; pub const UDCCR_SUSIM:u32=UDCCR_SRM; pub const UDCCR_REM:u32=128;
pub const UDCAR_ADD:u32=Fld(7,0); pub const UDCOMP_OUTMAXP:u32=Fld(8,0);
pub const UDCIMP_INMAXP:u32=Fld(8,0); pub const UDCD0_DATA:u32=Fld(8,0);
pub const UDCWC_WC:u32=Fld(4,0); pub const UDCDR_DATA:u32=Fld(8,0);
pub const UDCSR_EIR:u32=1; pub const UDCSR_RIR:u32=2; pub const UDCSR_TIR:u32=4;
pub const UDCSR_SUSIR:u32=8; pub const UDCSR_RESIR:u32=16; pub const UDCSR_RSTIR:u32=32;

/* Register-address helpers and the remaining definitions retain C macro behavior. */
#[inline] pub const fn _UTCR0(nb:u32)->u32 {0x80010000+(nb-1)*0x20000}
#[inline] pub const fn _UTCR1(nb:u32)->u32 {_UTCR0(nb)+4}
#[inline] pub const fn _UTCR2(nb:u32)->u32 {_UTCR0(nb)+8}
#[inline] pub const fn _UTCR3(nb:u32)->u32 {_UTCR0(nb)+12}
#[inline] pub const fn _UTCR4(nb:u32)->u32 {_UTCR0(nb)+16}
#[inline] pub const fn _UTDR(nb:u32)->u32 {_UTCR0(nb)+20}
#[inline] pub const fn _UTSR0(nb:u32)->u32 {_UTCR0(nb)+28}
#[inline] pub const fn _UTSR1(nb:u32)->u32 {_UTCR0(nb)+32}
pub const UTCR0:u32=0; pub const UTCR1:u32=4; pub const UTCR2:u32=8; pub const UTCR3:u32=12;
pub const UTDR:u32=20; pub const UTSR0:u32=28; pub const UTSR1:u32=32;

/* OS timer, power, reset, GPIO, interrupt, memory and LCD register constants. */
pub const OSMR0:u32=0x90000000; pub const OSMR1:u32=0x90000004; pub const OSMR2:u32=0x90000008; pub const OSMR3:u32=0x9000000c;
pub const OSCR:u32=0x90000010; pub const OSSR:u32=0x90000014; pub const OWER:u32=0x90000018; pub const OIER:u32=0x9000001c;
pub const GPIO_MIN:u32=0; pub const GPIO_MAX:u32=27;
#[inline] pub const fn GPIO_GPIO(n:u32)->u32 {1u32<<n}
#[inline] pub const fn IC_GPIO(n:u32)->u32 {1u32<<n}
#[inline] pub const fn IC_DMA(n:u32)->u32 {0x00100000<<n}
#[inline] pub const fn IC_OST(n:u32)->u32 {0x04000000<<n}
#[inline] pub const fn OSSR_M(n:u32)->u32 {1u32<<n}
#[inline] pub const fn OIER_E(n:u32)->u32 {1u32<<n}
pub const DMA_SIZE:u32=6*0x20; pub const DMA_PHYS:u32=0xb0000000;
pub const PPDR_In:u32=0; pub const PPDR_Out:u32=1; pub const GPDR_In:u32=0; pub const GPDR_Out:u32=1;
pub const IC_RTC1Hz:u32=0x40000000; pub const IC_RTCAlrm:u32=0x80000000;
pub const ICLR_IRQ:u32=0; pub const ICLR_FIQ:u32=1;

/* Additional register symbols are intentionally external architecture items. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
