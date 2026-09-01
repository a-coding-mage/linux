/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Rust translation of pci/cs46xx/cs46xx.h.
 * C include dependencies are represented by opaque external types below.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub type c_int = i32;
pub type c_uint = u32;
pub type c_ulong = u64;
pub type u32 = ::core::primitive::u32;

#[repr(C)] pub struct snd_dma_buffer { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_indirect { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_ac97_bus { _private: [u8; 0] }
#[repr(C)] pub struct snd_ac97 { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct snd_card { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm { _private: [u8; 0] }
#[repr(C)] pub struct snd_rawmidi { _private: [u8; 0] }
#[repr(C)] pub struct snd_rawmidi_substream { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct gameport { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct dsp_spos_instance { _private: [u8; 0] }
#[repr(C)] pub struct dsp_module_desc { _private: [u8; 0] }
#[repr(C)] pub struct dsp_pcm_channel_descriptor { _private: [u8; 0] }
#[repr(C)] pub struct ba1_struct { _private: [u8; 0] }
#[repr(C)] pub struct dev_pm_ops { _private: [u8; 0] }

// C header included <sound/pcm.h>, <sound/pcm-indirect.h>, <sound/rawmidi.h>,
// <sound/ac97_codec.h>, and "cs46xx_dsp_spos.h".
// Header guards and preprocessor conditionals are preserved as comments where needed.

// #ifndef __SOUND_CS46XX_H
pub const BA0_HISR: u32 = 0x00000000; 
pub const BA0_HSR0: u32 = 0x00000004; 
pub const BA0_HICR: u32 = 0x00000008; 
pub const BA0_DMSR: u32 = 0x00000100; 
pub const BA0_HSAR: u32 = 0x00000110; 
pub const BA0_HDAR: u32 = 0x00000114; 
pub const BA0_HDMR: u32 = 0x00000118; 
pub const BA0_HDCR: u32 = 0x0000011C; 
pub const BA0_PFMC: u32 = 0x00000200; 
pub const BA0_PFCV1: u32 = 0x00000204; 
pub const BA0_PFCV2: u32 = 0x00000208; 
pub const BA0_PCICFG00: u32 = 0x00000300; 
pub const BA0_PCICFG04: u32 = 0x00000304; 
pub const BA0_PCICFG08: u32 = 0x00000308; 
pub const BA0_PCICFG0C: u32 = 0x0000030C; 
pub const BA0_PCICFG10: u32 = 0x00000310; 
pub const BA0_PCICFG14: u32 = 0x00000314; 
pub const BA0_PCICFG18: u32 = 0x00000318; 
pub const BA0_PCICFG1C: u32 = 0x0000031C; 
pub const BA0_PCICFG20: u32 = 0x00000320; 
pub const BA0_PCICFG24: u32 = 0x00000324; 
pub const BA0_PCICFG28: u32 = 0x00000328; 
pub const BA0_PCICFG2C: u32 = 0x0000032C; 
pub const BA0_PCICFG30: u32 = 0x00000330; 
pub const BA0_PCICFG34: u32 = 0x00000334; 
pub const BA0_PCICFG38: u32 = 0x00000338; 
pub const BA0_PCICFG3C: u32 = 0x0000033C; 
pub const BA0_CLKCR1: u32 = 0x00000400; 
pub const BA0_CLKCR2: u32 = 0x00000404; 
pub const BA0_PLLM: u32 = 0x00000408; 
pub const BA0_PLLCC: u32 = 0x0000040C; 
pub const BA0_FRR: u32 = 0x00000410; 
pub const BA0_CFL1: u32 = 0x00000414; 
pub const BA0_CFL2: u32 = 0x00000418; 
pub const BA0_SERMC1: u32 = 0x00000420; 
pub const BA0_SERMC2: u32 = 0x00000424; 
pub const BA0_SERC1: u32 = 0x00000428; 
pub const BA0_SERC2: u32 = 0x0000042C; 
pub const BA0_SERC3: u32 = 0x00000430; 
pub const BA0_SERC4: u32 = 0x00000434; 
pub const BA0_SERC5: u32 = 0x00000438; 
pub const BA0_SERBSP: u32 = 0x0000043C; 
pub const BA0_SERBST: u32 = 0x00000440; 
pub const BA0_SERBCM: u32 = 0x00000444; 
pub const BA0_SERBAD: u32 = 0x00000448; 
pub const BA0_SERBCF: u32 = 0x0000044C; 
pub const BA0_SERBWP: u32 = 0x00000450; 
pub const BA0_SERBRP: u32 = 0x00000454; 
// #ifndef NO_CS4612
pub const BA0_ASER_FADDR: u32 = 0x00000458; 
// #endif
pub const BA0_ACCTL: u32 = 0x00000460; 
pub const BA0_ACSTS: u32 = 0x00000464; 
pub const BA0_ACOSV: u32 = 0x00000468; 
pub const BA0_ACCAD: u32 = 0x0000046C; 
pub const BA0_ACCDA: u32 = 0x00000470; 
pub const BA0_ACISV: u32 = 0x00000474; 
pub const BA0_ACSAD: u32 = 0x00000478; 
pub const BA0_ACSDA: u32 = 0x0000047C; 
pub const BA0_JSPT: u32 = 0x00000480; 
pub const BA0_JSCTL: u32 = 0x00000484; 
pub const BA0_JSC1: u32 = 0x00000488; 
pub const BA0_JSC2: u32 = 0x0000048C; 
pub const BA0_MIDCR: u32 = 0x00000490; 
pub const BA0_MIDSR: u32 = 0x00000494; 
pub const BA0_MIDWP: u32 = 0x00000498; 
pub const BA0_MIDRP: u32 = 0x0000049C; 
pub const BA0_JSIO: u32 = 0x000004A0; 
// #ifndef NO_CS4612
pub const BA0_ASER_MASTER: u32 = 0x000004A4; 
// #endif
pub const BA0_CFGI: u32 = 0x000004B0; 
pub const BA0_SSVID: u32 = 0x000004B4; 
pub const BA0_GPIOR: u32 = 0x000004B8; 
// #ifndef NO_CS4612
pub const BA0_EGPIODR: u32 = 0x000004BC; 
pub const BA0_EGPIOPTR: u32 = 0x000004C0; 
pub const BA0_EGPIOTR: u32 = 0x000004C4; 
pub const BA0_EGPIOWR: u32 = 0x000004C8; 
pub const BA0_EGPIOSR: u32 = 0x000004CC; 
pub const BA0_SERC6: u32 = 0x000004D0; 
pub const BA0_SERC7: u32 = 0x000004D4; 
pub const BA0_SERACC: u32 = 0x000004D8; 
pub const BA0_ACCTL2: u32 = 0x000004E0; 
pub const BA0_ACSTS2: u32 = 0x000004E4; 
pub const BA0_ACOSV2: u32 = 0x000004E8; 
pub const BA0_ACCAD2: u32 = 0x000004EC; 
pub const BA0_ACCDA2: u32 = 0x000004F0; 
pub const BA0_ACISV2: u32 = 0x000004F4; 
pub const BA0_ACSAD2: u32 = 0x000004F8; 
pub const BA0_ACSDA2: u32 = 0x000004FC; 
pub const BA0_IOTAC0: u32 = 0x00000500; 
pub const BA0_IOTAC1: u32 = 0x00000504; 
pub const BA0_IOTAC2: u32 = 0x00000508; 
pub const BA0_IOTAC3: u32 = 0x0000050C; 
pub const BA0_IOTAC4: u32 = 0x00000510; 
pub const BA0_IOTAC5: u32 = 0x00000514; 
pub const BA0_IOTAC6: u32 = 0x00000518; 
pub const BA0_IOTAC7: u32 = 0x0000051C; 
pub const BA0_IOTAC8: u32 = 0x00000520; 
pub const BA0_IOTAC9: u32 = 0x00000524; 
pub const BA0_IOTAC10: u32 = 0x00000528; 
pub const BA0_IOTAC11: u32 = 0x0000052C; 
pub const BA0_IOTFR0: u32 = 0x00000540; 
pub const BA0_IOTFR1: u32 = 0x00000544; 
pub const BA0_IOTFR2: u32 = 0x00000548; 
pub const BA0_IOTFR3: u32 = 0x0000054C; 
pub const BA0_IOTFR4: u32 = 0x00000550; 
pub const BA0_IOTFR5: u32 = 0x00000554; 
pub const BA0_IOTFR6: u32 = 0x00000558; 
pub const BA0_IOTFR7: u32 = 0x0000055C; 
pub const BA0_IOTFIFO: u32 = 0x00000580; 
pub const BA0_IOTRRD: u32 = 0x00000584; 
pub const BA0_IOTFP: u32 = 0x00000588; 
pub const BA0_IOTCR: u32 = 0x0000058C; 
pub const BA0_DPCID: u32 = 0x00000590; 
pub const BA0_DPCIA: u32 = 0x00000594; 
pub const BA0_DPCIC: u32 = 0x00000598; 
pub const BA0_PCPCIR: u32 = 0x00000600; 
pub const BA0_PCPCIG: u32 = 0x00000604; 
pub const BA0_PCPCIEN: u32 = 0x00000608; 
pub const BA0_EPCIPMC: u32 = 0x00000610; 
// #endif
pub const BA1_SP_DMEM0: u32 = 0x00000000; 
pub const BA1_SP_DMEM1: u32 = 0x00010000; 
pub const BA1_SP_PMEM: u32 = 0x00020000; 
pub const BA1_SP_REG: u32 = 0x00030000; 
pub const BA1_SPCR: u32 = 0x00030000; 
pub const BA1_DREG: u32 = 0x00030004; 
pub const BA1_DSRWP: u32 = 0x00030008; 
pub const BA1_TWPR: u32 = 0x0003000C; 
pub const BA1_SPWR: u32 = 0x00030010; 
pub const BA1_SPIR: u32 = 0x00030014; 
pub const BA1_FGR1: u32 = 0x00030020; 
pub const BA1_SPCS: u32 = 0x00030028; 
pub const BA1_SDSR: u32 = 0x0003002C; 
pub const BA1_FRMT: u32 = 0x00030030; 
pub const BA1_FRCC: u32 = 0x00030034; 
pub const BA1_FRSC: u32 = 0x00030038; 
pub const BA1_OMNI_MEM: u32 = 0x000E0000; 
pub const HISR_VC_MASK: u32 = 0x0000FFFF; 
pub const HISR_VC0: u32 = 0x00000001; 
pub const HISR_VC1: u32 = 0x00000002; 
pub const HISR_VC2: u32 = 0x00000004; 
pub const HISR_VC3: u32 = 0x00000008; 
pub const HISR_VC4: u32 = 0x00000010; 
pub const HISR_VC5: u32 = 0x00000020; 
pub const HISR_VC6: u32 = 0x00000040; 
pub const HISR_VC7: u32 = 0x00000080; 
pub const HISR_VC8: u32 = 0x00000100; 
pub const HISR_VC9: u32 = 0x00000200; 
pub const HISR_VC10: u32 = 0x00000400; 
pub const HISR_VC11: u32 = 0x00000800; 
pub const HISR_VC12: u32 = 0x00001000; 
pub const HISR_VC13: u32 = 0x00002000; 
pub const HISR_VC14: u32 = 0x00004000; 
pub const HISR_VC15: u32 = 0x00008000; 
pub const HISR_INT0: u32 = 0x00010000; 
pub const HISR_INT1: u32 = 0x00020000; 
pub const HISR_DMAI: u32 = 0x00040000; 
pub const HISR_FROVR: u32 = 0x00080000; 
pub const HISR_MIDI: u32 = 0x00100000; 
// #ifdef NO_CS4612
pub const HISR_RESERVED: u32 = 0x0FE00000; 
// #else
pub const HISR_SBINT: u32 = 0x00200000; 
pub const HISR_RESERVED: u32 = 0x0FC00000; 
// #endif
pub const HISR_H0P: u32 = 0x40000000; 
pub const HISR_INTENA: u32 = 0x80000000; 
pub const HSR0_VC_MASK: u32 = 0xFFFFFFFF; 
pub const HSR0_VC16: u32 = 0x00000001; 
pub const HSR0_VC17: u32 = 0x00000002; 
pub const HSR0_VC18: u32 = 0x00000004; 
pub const HSR0_VC19: u32 = 0x00000008; 
pub const HSR0_VC20: u32 = 0x00000010; 
pub const HSR0_VC21: u32 = 0x00000020; 
pub const HSR0_VC22: u32 = 0x00000040; 
pub const HSR0_VC23: u32 = 0x00000080; 
pub const HSR0_VC24: u32 = 0x00000100; 
pub const HSR0_VC25: u32 = 0x00000200; 
pub const HSR0_VC26: u32 = 0x00000400; 
pub const HSR0_VC27: u32 = 0x00000800; 
pub const HSR0_VC28: u32 = 0x00001000; 
pub const HSR0_VC29: u32 = 0x00002000; 
pub const HSR0_VC30: u32 = 0x00004000; 
pub const HSR0_VC31: u32 = 0x00008000; 
pub const HSR0_VC32: u32 = 0x00010000; 
pub const HSR0_VC33: u32 = 0x00020000; 
pub const HSR0_VC34: u32 = 0x00040000; 
pub const HSR0_VC35: u32 = 0x00080000; 
pub const HSR0_VC36: u32 = 0x00100000; 
pub const HSR0_VC37: u32 = 0x00200000; 
pub const HSR0_VC38: u32 = 0x00400000; 
pub const HSR0_VC39: u32 = 0x00800000; 
pub const HSR0_VC40: u32 = 0x01000000; 
pub const HSR0_VC41: u32 = 0x02000000; 
pub const HSR0_VC42: u32 = 0x04000000; 
pub const HSR0_VC43: u32 = 0x08000000; 
pub const HSR0_VC44: u32 = 0x10000000; 
pub const HSR0_VC45: u32 = 0x20000000; 
pub const HSR0_VC46: u32 = 0x40000000; 
pub const HSR0_VC47: u32 = 0x80000000; 
pub const HICR_IEV: u32 = 0x00000001; 
pub const HICR_CHGM: u32 = 0x00000002; 
pub const DMSR_HP: u32 = 0x00000001; 
pub const DMSR_HR: u32 = 0x00000002; 
pub const DMSR_SP: u32 = 0x00000004; 
pub const DMSR_SR: u32 = 0x00000008; 
pub const HSAR_HOST_ADDR_MASK: u32 = 0xFFFFFFFF; 
pub const HSAR_DSP_ADDR_MASK: u32 = 0x0000FFFF; 
pub const HSAR_MEMID_MASK: u32 = 0x000F0000; 
pub const HSAR_MEMID_SP_DMEM0: u32 = 0x00000000; 
pub const HSAR_MEMID_SP_DMEM1: u32 = 0x00010000; 
pub const HSAR_MEMID_SP_PMEM: u32 = 0x00020000; 
pub const HSAR_MEMID_SP_DEBUG: u32 = 0x00030000; 
pub const HSAR_MEMID_OMNI_MEM: u32 = 0x000E0000; 
pub const HSAR_END: u32 = 0x40000000; 
pub const HSAR_ERR: u32 = 0x80000000; 
pub const HDAR_HOST_ADDR_MASK: u32 = 0xFFFFFFFF; 
pub const HDAR_DSP_ADDR_MASK: u32 = 0x0000FFFF; 
pub const HDAR_MEMID_MASK: u32 = 0x000F0000; 
pub const HDAR_MEMID_SP_DMEM0: u32 = 0x00000000; 
pub const HDAR_MEMID_SP_DMEM1: u32 = 0x00010000; 
pub const HDAR_MEMID_SP_PMEM: u32 = 0x00020000; 
pub const HDAR_MEMID_SP_DEBUG: u32 = 0x00030000; 
pub const HDAR_MEMID_OMNI_MEM: u32 = 0x000E0000; 
pub const HDAR_END: u32 = 0x40000000; 
pub const HDAR_ERR: u32 = 0x80000000; 
pub const HDMR_AC_MASK: u32 = 0x0000F000; 
pub const HDMR_AC_8_16: u32 = 0x00001000; 
pub const HDMR_AC_M_S: u32 = 0x00002000; 
pub const HDMR_AC_B_L: u32 = 0x00004000; 
pub const HDMR_AC_S_U: u32 = 0x00008000; 
pub const HDCR_COUNT_MASK: u32 = 0x000003FF; 
pub const HDCR_DONE: u32 = 0x00004000; 
pub const HDCR_OPT: u32 = 0x00008000; 
pub const HDCR_WBD: u32 = 0x00400000; 
pub const HDCR_WBS: u32 = 0x00800000; 
pub const HDCR_DMS_MASK: u32 = 0x07000000; 
pub const HDCR_DMS_LINEAR: u32 = 0x00000000; 
pub const HDCR_DMS_16_DWORDS: u32 = 0x01000000; 
pub const HDCR_DMS_32_DWORDS: u32 = 0x02000000; 
pub const HDCR_DMS_64_DWORDS: u32 = 0x03000000; 
pub const HDCR_DMS_128_DWORDS: u32 = 0x04000000; 
pub const HDCR_DMS_256_DWORDS: u32 = 0x05000000; 
pub const HDCR_DMS_512_DWORDS: u32 = 0x06000000; 
pub const HDCR_DMS_1024_DWORDS: u32 = 0x07000000; 
pub const HDCR_DH: u32 = 0x08000000; 
pub const HDCR_SMS_MASK: u32 = 0x70000000; 
pub const HDCR_SMS_LINEAR: u32 = 0x00000000; 
pub const HDCR_SMS_16_DWORDS: u32 = 0x10000000; 
pub const HDCR_SMS_32_DWORDS: u32 = 0x20000000; 
pub const HDCR_SMS_64_DWORDS: u32 = 0x30000000; 
pub const HDCR_SMS_128_DWORDS: u32 = 0x40000000; 
pub const HDCR_SMS_256_DWORDS: u32 = 0x50000000; 
pub const HDCR_SMS_512_DWORDS: u32 = 0x60000000; 
pub const HDCR_SMS_1024_DWORDS: u32 = 0x70000000; 
pub const HDCR_SH: u32 = 0x80000000; 
pub const HDCR_COUNT_SHIFT: u32 = 0; 
pub const PFMC_C1SS_MASK: u32 = 0x0000001F; 
pub const PFMC_C1EV: u32 = 0x00000020; 
pub const PFMC_C1RS: u32 = 0x00008000; 
pub const PFMC_C2SS_MASK: u32 = 0x001F0000; 
pub const PFMC_C2EV: u32 = 0x00200000; 
pub const PFMC_C2RS: u32 = 0x80000000; 
pub const PFMC_C1SS_SHIFT: u32 = 0; 
pub const PFMC_C2SS_SHIFT: u32 = 16; 
pub const PFMC_BUS_GRANT: u32 = 0; 
pub const PFMC_GRANT_AFTER_REQ: u32 = 1; 
pub const PFMC_TRANSACTION: u32 = 2; 
pub const PFMC_DWORD_TRANSFER: u32 = 3; 
pub const PFMC_SLAVE_READ: u32 = 4; 
pub const PFMC_SLAVE_WRITE: u32 = 5; 
pub const PFMC_PREEMPTION: u32 = 6; 
pub const PFMC_DISCONNECT_RETRY: u32 = 7; 
pub const PFMC_INTERRUPT: u32 = 8; 
pub const PFMC_BUS_OWNERSHIP: u32 = 9; 
pub const PFMC_TRANSACTION_LAG: u32 = 10; 
pub const PFMC_PCI_CLOCK: u32 = 11; 
pub const PFMC_SERIAL_CLOCK: u32 = 12; 
pub const PFMC_SP_CLOCK: u32 = 13; 
pub const PFCV1_PC1V_MASK: u32 = 0xFFFFFFFF; 
pub const PFCV1_PC1V_SHIFT: u32 = 0; 
pub const PFCV2_PC2V_MASK: u32 = 0xFFFFFFFF; 
pub const PFCV2_PC2V_SHIFT: u32 = 0; 
pub const CLKCR1_OSCS: u32 = 0x00000001; 
pub const CLKCR1_OSCP: u32 = 0x00000002; 
pub const CLKCR1_PLLSS_MASK: u32 = 0x0000000C; 
pub const CLKCR1_PLLSS_SERIAL: u32 = 0x00000000; 
pub const CLKCR1_PLLSS_CRYSTAL: u32 = 0x00000004; 
pub const CLKCR1_PLLSS_PCI: u32 = 0x00000008; 
pub const CLKCR1_PLLSS_RESERVED: u32 = 0x0000000C; 
pub const CLKCR1_PLLP: u32 = 0x00000010; 
pub const CLKCR1_SWCE: u32 = 0x00000020; 
pub const CLKCR1_PLLOS: u32 = 0x00000040; 
pub const CLKCR2_PDIVS_MASK: u32 = 0x0000000F; 
pub const CLKCR2_PDIVS_1: u32 = 0x00000001; 
pub const CLKCR2_PDIVS_2: u32 = 0x00000002; 
pub const CLKCR2_PDIVS_4: u32 = 0x00000004; 
pub const CLKCR2_PDIVS_7: u32 = 0x00000007; 
pub const CLKCR2_PDIVS_8: u32 = 0x00000008; 
pub const CLKCR2_PDIVS_16: u32 = 0x00000000; 
pub const PLLM_MASK: u32 = 0x000000FF; 
pub const PLLM_SHIFT: u32 = 0; 
pub const PLLCC_CDR_MASK: u32 = 0x00000007; 
// #ifndef NO_CS4610
pub const PLLCC_CDR_240_350_MHZ: u32 = 0x00000000; 
pub const PLLCC_CDR_184_265_MHZ: u32 = 0x00000001; 
pub const PLLCC_CDR_144_205_MHZ: u32 = 0x00000002; 
pub const PLLCC_CDR_111_160_MHZ: u32 = 0x00000003; 
pub const PLLCC_CDR_87_123_MHZ: u32 = 0x00000004; 
pub const PLLCC_CDR_67_96_MHZ: u32 = 0x00000005; 
pub const PLLCC_CDR_52_74_MHZ: u32 = 0x00000006; 
pub const PLLCC_CDR_45_58_MHZ: u32 = 0x00000007; 
// #endif
// #ifndef NO_CS4612
pub const PLLCC_CDR_271_398_MHZ: u32 = 0x00000000; 
pub const PLLCC_CDR_227_330_MHZ: u32 = 0x00000001; 
pub const PLLCC_CDR_167_239_MHZ: u32 = 0x00000002; 
pub const PLLCC_CDR_150_215_MHZ: u32 = 0x00000003; 
pub const PLLCC_CDR_107_154_MHZ: u32 = 0x00000004; 
pub const PLLCC_CDR_98_140_MHZ: u32 = 0x00000005; 
pub const PLLCC_CDR_73_104_MHZ: u32 = 0x00000006; 
pub const PLLCC_CDR_63_90_MHZ: u32 = 0x00000007; 
// #endif
pub const PLLCC_LPF_MASK: u32 = 0x000000F8; 
// #ifndef NO_CS4610
pub const PLLCC_LPF_23850_60000_KHZ: u32 = 0x00000000; 
pub const PLLCC_LPF_7960_26290_KHZ: u32 = 0x00000008; 
pub const PLLCC_LPF_4160_10980_KHZ: u32 = 0x00000018; 
pub const PLLCC_LPF_1740_4580_KHZ: u32 = 0x00000038; 
pub const PLLCC_LPF_724_1910_KHZ: u32 = 0x00000078; 
pub const PLLCC_LPF_317_798_KHZ: u32 = 0x000000F8; 
// #endif
// #ifndef NO_CS4612
pub const PLLCC_LPF_25580_64530_KHZ: u32 = 0x00000000; 
pub const PLLCC_LPF_14360_37270_KHZ: u32 = 0x00000008; 
pub const PLLCC_LPF_6100_16020_KHZ: u32 = 0x00000018; 
pub const PLLCC_LPF_2540_6690_KHZ: u32 = 0x00000038; 
pub const PLLCC_LPF_1050_2780_KHZ: u32 = 0x00000078; 
pub const PLLCC_LPF_450_1160_KHZ: u32 = 0x000000F8; 
// #endif
pub const FRR_FAB_MASK: u32 = 0x00000003; 
pub const FRR_MASK_MASK: u32 = 0x0000001C; 
// #ifdef NO_CS4612
pub const FRR_CFOP_MASK: u32 = 0x000000E0; 
// #else
pub const FRR_CFOP_MASK: u32 = 0x00000FE0; 
// #endif
pub const FRR_CFOP_NOT_DVD: u32 = 0x00000020; 
pub const FRR_CFOP_A3D: u32 = 0x00000040; 
pub const FRR_CFOP_128_PIN: u32 = 0x00000080; 
// #ifndef NO_CS4612
pub const FRR_CFOP_CS4280: u32 = 0x00000800; 
// #endif
pub const FRR_FAB_SHIFT: u32 = 0; 
pub const FRR_MASK_SHIFT: u32 = 2; 
pub const FRR_CFOP_SHIFT: u32 = 5; 
pub const CFL1_CLOCK_SOURCE_MASK: u32 = 0x00000003; 
pub const CFL1_CLOCK_SOURCE_CS423X: u32 = 0x00000000; 
pub const CFL1_CLOCK_SOURCE_AC97: u32 = 0x00000001; 
pub const CFL1_CLOCK_SOURCE_CRYSTAL: u32 = 0x00000002; 
pub const CFL1_CLOCK_SOURCE_DUAL_AC97: u32 = 0x00000003; 
pub const CFL1_VALID_DATA_MASK: u32 = 0x000000FF; 
pub const CFL2_VALID_DATA_MASK: u32 = 0x000000FF; 
pub const SERMC1_MSPE: u32 = 0x00000001; 
pub const SERMC1_PTC_MASK: u32 = 0x0000000E; 
pub const SERMC1_PTC_CS423X: u32 = 0x00000000; 
pub const SERMC1_PTC_AC97: u32 = 0x00000002; 
pub const SERMC1_PTC_DAC: u32 = 0x00000004; 
pub const SERMC1_PLB: u32 = 0x00000010; 
pub const SERMC1_XLB: u32 = 0x00000020; 
pub const SERMC2_LROE: u32 = 0x00000001; 
pub const SERMC2_MCOE: u32 = 0x00000002; 
pub const SERMC2_MCDIV: u32 = 0x00000004; 
pub const SERC1_SO1EN: u32 = 0x00000001; 
pub const SERC1_SO1F_MASK: u32 = 0x0000000E; 
pub const SERC1_SO1F_CS423X: u32 = 0x00000000; 
pub const SERC1_SO1F_AC97: u32 = 0x00000002; 
pub const SERC1_SO1F_DAC: u32 = 0x00000004; 
pub const SERC1_SO1F_SPDIF: u32 = 0x00000006; 
pub const SERC2_SI1EN: u32 = 0x00000001; 
pub const SERC2_SI1F_MASK: u32 = 0x0000000E; 
pub const SERC2_SI1F_CS423X: u32 = 0x00000000; 
pub const SERC2_SI1F_AC97: u32 = 0x00000002; 
pub const SERC2_SI1F_ADC: u32 = 0x00000004; 
pub const SERC2_SI1F_SPDIF: u32 = 0x00000006; 
pub const SERC3_SO2EN: u32 = 0x00000001; 
pub const SERC3_SO2F_MASK: u32 = 0x00000006; 
pub const SERC3_SO2F_DAC: u32 = 0x00000000; 
pub const SERC3_SO2F_SPDIF: u32 = 0x00000002; 
pub const SERC4_SO3EN: u32 = 0x00000001; 
pub const SERC4_SO3F_MASK: u32 = 0x00000006; 
pub const SERC4_SO3F_DAC: u32 = 0x00000000; 
pub const SERC4_SO3F_SPDIF: u32 = 0x00000002; 
pub const SERC5_SI2EN: u32 = 0x00000001; 
pub const SERC5_SI2F_MASK: u32 = 0x00000006; 
pub const SERC5_SI2F_ADC: u32 = 0x00000000; 
pub const SERC5_SI2F_SPDIF: u32 = 0x00000002; 
pub const SERBSP_FSP_MASK: u32 = 0x0000000F; 
pub const SERBSP_FSP_SHIFT: u32 = 0; 
pub const SERBST_RRDY: u32 = 0x00000001; 
pub const SERBST_WBSY: u32 = 0x00000002; 
pub const SERBCM_RDC: u32 = 0x00000001; 
pub const SERBCM_WRC: u32 = 0x00000002; 
// #ifdef NO_CS4612
pub const SERBAD_FAD_MASK: u32 = 0x000000FF; 
// #else
pub const SERBAD_FAD_MASK: u32 = 0x000001FF; 
// #endif
pub const SERBAD_FAD_SHIFT: u32 = 0; 
pub const SERBCF_HBP: u32 = 0x00000001; 
pub const SERBWP_FWD_MASK: u32 = 0x000FFFFF; 
pub const SERBWP_FWD_SHIFT: u32 = 0; 
pub const SERBRP_FRD_MASK: u32 = 0x000FFFFF; 
pub const SERBRP_FRD_SHIFT: u32 = 0; 
// #ifndef NO_CS4612
pub const ASER_FADDR_A1_MASK: u32 = 0x000001FF; 
pub const ASER_FADDR_EN1: u32 = 0x00008000; 
pub const ASER_FADDR_A2_MASK: u32 = 0x01FF0000; 
pub const ASER_FADDR_EN2: u32 = 0x80000000; 
pub const ASER_FADDR_A1_SHIFT: u32 = 0; 
pub const ASER_FADDR_A2_SHIFT: u32 = 16; 
// #endif
pub const ACCTL_RSTN: u32 = 0x00000001; 
pub const ACCTL_ESYN: u32 = 0x00000002; 
pub const ACCTL_VFRM: u32 = 0x00000004; 
pub const ACCTL_DCV: u32 = 0x00000008; 
pub const ACCTL_CRW: u32 = 0x00000010; 
pub const ACCTL_ASYN: u32 = 0x00000020; 
// #ifndef NO_CS4612
pub const ACCTL_TC: u32 = 0x00000040; 
// #endif
pub const ACSTS_CRDY: u32 = 0x00000001; 
pub const ACSTS_VSTS: u32 = 0x00000002; 
// #ifndef NO_CS4612
pub const ACSTS_WKUP: u32 = 0x00000004; 
// #endif
pub const ACOSV_SLV3: u32 = 0x00000001; 
pub const ACOSV_SLV4: u32 = 0x00000002; 
pub const ACOSV_SLV5: u32 = 0x00000004; 
pub const ACOSV_SLV6: u32 = 0x00000008; 
pub const ACOSV_SLV7: u32 = 0x00000010; 
pub const ACOSV_SLV8: u32 = 0x00000020; 
pub const ACOSV_SLV9: u32 = 0x00000040; 
pub const ACOSV_SLV10: u32 = 0x00000080; 
pub const ACOSV_SLV11: u32 = 0x00000100; 
pub const ACOSV_SLV12: u32 = 0x00000200; 
pub const ACCAD_CI_MASK: u32 = 0x0000007F; 
pub const ACCAD_CI_SHIFT: u32 = 0; 
pub const ACCDA_CD_MASK: u32 = 0x0000FFFF; 
pub const ACCDA_CD_SHIFT: u32 = 0; 
pub const ACISV_ISV3: u32 = 0x00000001; 
pub const ACISV_ISV4: u32 = 0x00000002; 
pub const ACISV_ISV5: u32 = 0x00000004; 
pub const ACISV_ISV6: u32 = 0x00000008; 
pub const ACISV_ISV7: u32 = 0x00000010; 
pub const ACISV_ISV8: u32 = 0x00000020; 
pub const ACISV_ISV9: u32 = 0x00000040; 
pub const ACISV_ISV10: u32 = 0x00000080; 
pub const ACISV_ISV11: u32 = 0x00000100; 
pub const ACISV_ISV12: u32 = 0x00000200; 
pub const ACSAD_SI_MASK: u32 = 0x0000007F; 
pub const ACSAD_SI_SHIFT: u32 = 0; 
pub const ACSDA_SD_MASK: u32 = 0x0000FFFF; 
pub const ACSDA_SD_SHIFT: u32 = 0; 
pub const JSPT_CAX: u32 = 0x00000001; 
pub const JSPT_CAY: u32 = 0x00000002; 
pub const JSPT_CBX: u32 = 0x00000004; 
pub const JSPT_CBY: u32 = 0x00000008; 
pub const JSPT_BA1: u32 = 0x00000010; 
pub const JSPT_BA2: u32 = 0x00000020; 
pub const JSPT_BB1: u32 = 0x00000040; 
pub const JSPT_BB2: u32 = 0x00000080; 
pub const JSCTL_SP_MASK: u32 = 0x00000003; 
pub const JSCTL_SP_SLOW: u32 = 0x00000000; 
pub const JSCTL_SP_MEDIUM_SLOW: u32 = 0x00000001; 
pub const JSCTL_SP_MEDIUM_FAST: u32 = 0x00000002; 
pub const JSCTL_SP_FAST: u32 = 0x00000003; 
pub const JSCTL_ARE: u32 = 0x00000004; 
pub const JSC1_Y1V_MASK: u32 = 0x0000FFFF; 
pub const JSC1_X1V_MASK: u32 = 0xFFFF0000; 
pub const JSC1_Y1V_SHIFT: u32 = 0; 
pub const JSC1_X1V_SHIFT: u32 = 16; 
pub const JSC2_Y2V_MASK: u32 = 0x0000FFFF; 
pub const JSC2_X2V_MASK: u32 = 0xFFFF0000; 
pub const JSC2_Y2V_SHIFT: u32 = 0; 
pub const JSC2_X2V_SHIFT: u32 = 16; 
pub const MIDCR_TXE: u32 = 0x00000001; /* Enable transmitting. */
pub const MIDCR_RXE: u32 = 0x00000002; /* Enable receiving. */
pub const MIDCR_RIE: u32 = 0x00000004; /* Interrupt upon tx ready. */
pub const MIDCR_TIE: u32 = 0x00000008; /* Interrupt upon rx ready. */
pub const MIDCR_MLB: u32 = 0x00000010; /* Enable midi loopback. */
pub const MIDCR_MRST: u32 = 0x00000020; /* Reset interface. */
pub const MIDSR_TBF: u32 = 0x00000001; /* Tx FIFO is full. */
pub const MIDSR_RBE: u32 = 0x00000002; /* Rx FIFO is empty. */
pub const MIDWP_MWD_MASK: u32 = 0x000000FF; 
pub const MIDWP_MWD_SHIFT: u32 = 0; 
pub const MIDRP_MRD_MASK: u32 = 0x000000FF; 
pub const MIDRP_MRD_SHIFT: u32 = 0; 
pub const JSIO_DAX: u32 = 0x00000001; 
pub const JSIO_DAY: u32 = 0x00000002; 
pub const JSIO_DBX: u32 = 0x00000004; 
pub const JSIO_DBY: u32 = 0x00000008; 
pub const JSIO_AXOE: u32 = 0x00000010; 
pub const JSIO_AYOE: u32 = 0x00000020; 
pub const JSIO_BXOE: u32 = 0x00000040; 
pub const JSIO_BYOE: u32 = 0x00000080; 
// #ifndef NO_CS4612
pub const ASER_MASTER_ME: u32 = 0x00000001; 
// #endif
pub const CFGI_CLK: u32 = 0x00000001; 
pub const CFGI_DOUT: u32 = 0x00000002; 
pub const CFGI_DIN_EEN: u32 = 0x00000004; 
pub const CFGI_EELD: u32 = 0x00000008; 
pub const SSVID_VID_MASK: u32 = 0x0000FFFF; 
pub const SSVID_SID_MASK: u32 = 0xFFFF0000; 
pub const SSVID_VID_SHIFT: u32 = 0; 
pub const SSVID_SID_SHIFT: u32 = 16; 
pub const GPIOR_VOLDN: u32 = 0x00000001; 
pub const GPIOR_VOLUP: u32 = 0x00000002; 
pub const GPIOR_SI2D: u32 = 0x00000004; 
pub const GPIOR_SI2OE: u32 = 0x00000008; 
// #ifndef NO_CS4612
pub const EGPIODR_GPOE0: u32 = 0x00000001; 
pub const EGPIODR_GPOE1: u32 = 0x00000002; 
pub const EGPIODR_GPOE2: u32 = 0x00000004; 
pub const EGPIODR_GPOE3: u32 = 0x00000008; 
pub const EGPIODR_GPOE4: u32 = 0x00000010; 
pub const EGPIODR_GPOE5: u32 = 0x00000020; 
pub const EGPIODR_GPOE6: u32 = 0x00000040; 
pub const EGPIODR_GPOE7: u32 = 0x00000080; 
pub const EGPIODR_GPOE8: u32 = 0x00000100; 
// #endif
// #ifndef NO_CS4612
pub const EGPIOPTR_GPPT0: u32 = 0x00000001; 
pub const EGPIOPTR_GPPT1: u32 = 0x00000002; 
pub const EGPIOPTR_GPPT2: u32 = 0x00000004; 
pub const EGPIOPTR_GPPT3: u32 = 0x00000008; 
pub const EGPIOPTR_GPPT4: u32 = 0x00000010; 
pub const EGPIOPTR_GPPT5: u32 = 0x00000020; 
pub const EGPIOPTR_GPPT6: u32 = 0x00000040; 
pub const EGPIOPTR_GPPT7: u32 = 0x00000080; 
pub const EGPIOPTR_GPPT8: u32 = 0x00000100; 
// #endif
// #ifndef NO_CS4612
pub const EGPIOTR_GPS0: u32 = 0x00000001; 
pub const EGPIOTR_GPS1: u32 = 0x00000002; 
pub const EGPIOTR_GPS2: u32 = 0x00000004; 
pub const EGPIOTR_GPS3: u32 = 0x00000008; 
pub const EGPIOTR_GPS4: u32 = 0x00000010; 
pub const EGPIOTR_GPS5: u32 = 0x00000020; 
pub const EGPIOTR_GPS6: u32 = 0x00000040; 
pub const EGPIOTR_GPS7: u32 = 0x00000080; 
pub const EGPIOTR_GPS8: u32 = 0x00000100; 
// #endif
// #ifndef NO_CS4612
pub const EGPIOWR_GPW0: u32 = 0x00000001; 
pub const EGPIOWR_GPW1: u32 = 0x00000002; 
pub const EGPIOWR_GPW2: u32 = 0x00000004; 
pub const EGPIOWR_GPW3: u32 = 0x00000008; 
pub const EGPIOWR_GPW4: u32 = 0x00000010; 
pub const EGPIOWR_GPW5: u32 = 0x00000020; 
pub const EGPIOWR_GPW6: u32 = 0x00000040; 
pub const EGPIOWR_GPW7: u32 = 0x00000080; 
pub const EGPIOWR_GPW8: u32 = 0x00000100; 
// #endif
// #ifndef NO_CS4612
pub const EGPIOSR_GPS0: u32 = 0x00000001; 
pub const EGPIOSR_GPS1: u32 = 0x00000002; 
pub const EGPIOSR_GPS2: u32 = 0x00000004; 
pub const EGPIOSR_GPS3: u32 = 0x00000008; 
pub const EGPIOSR_GPS4: u32 = 0x00000010; 
pub const EGPIOSR_GPS5: u32 = 0x00000020; 
pub const EGPIOSR_GPS6: u32 = 0x00000040; 
pub const EGPIOSR_GPS7: u32 = 0x00000080; 
pub const EGPIOSR_GPS8: u32 = 0x00000100; 
// #endif
// #ifndef NO_CS4612
pub const SERC6_ASDO2EN: u32 = 0x00000001; 
// #endif
// #ifndef NO_CS4612
pub const SERC7_ASDI2EN: u32 = 0x00000001; 
pub const SERC7_POSILB: u32 = 0x00000002; 
pub const SERC7_SIPOLB: u32 = 0x00000004; 
pub const SERC7_SOSILB: u32 = 0x00000008; 
pub const SERC7_SISOLB: u32 = 0x00000010; 
// #endif
// #ifndef NO_CS4612
pub const SERACC_CHIP_TYPE_MASK: u32 = 0x00000001; 
pub const SERACC_CHIP_TYPE_1_03: u32 = 0x00000000; 
pub const SERACC_CHIP_TYPE_2_0: u32 = 0x00000001; 
pub const SERACC_TWO_CODECS: u32 = 0x00000002; 
pub const SERACC_MDM: u32 = 0x00000004; 
pub const SERACC_HSP: u32 = 0x00000008; 
pub const SERACC_ODT: u32 = 0x00000010; /* only CS4630 */
// #endif
// #ifndef NO_CS4612
pub const ACCTL2_RSTN: u32 = 0x00000001; 
pub const ACCTL2_ESYN: u32 = 0x00000002; 
pub const ACCTL2_VFRM: u32 = 0x00000004; 
pub const ACCTL2_DCV: u32 = 0x00000008; 
pub const ACCTL2_CRW: u32 = 0x00000010; 
pub const ACCTL2_ASYN: u32 = 0x00000020; 
// #endif
// #ifndef NO_CS4612
pub const ACSTS2_CRDY: u32 = 0x00000001; 
pub const ACSTS2_VSTS: u32 = 0x00000002; 
// #endif
// #ifndef NO_CS4612
pub const ACOSV2_SLV3: u32 = 0x00000001; 
pub const ACOSV2_SLV4: u32 = 0x00000002; 
pub const ACOSV2_SLV5: u32 = 0x00000004; 
pub const ACOSV2_SLV6: u32 = 0x00000008; 
pub const ACOSV2_SLV7: u32 = 0x00000010; 
pub const ACOSV2_SLV8: u32 = 0x00000020; 
pub const ACOSV2_SLV9: u32 = 0x00000040; 
pub const ACOSV2_SLV10: u32 = 0x00000080; 
pub const ACOSV2_SLV11: u32 = 0x00000100; 
pub const ACOSV2_SLV12: u32 = 0x00000200; 
// #endif
// #ifndef NO_CS4612
pub const ACCAD2_CI_MASK: u32 = 0x0000007F; 
pub const ACCAD2_CI_SHIFT: u32 = 0; 
// #endif
// #ifndef NO_CS4612
pub const ACCDA2_CD_MASK: u32 = 0x0000FFFF; 
pub const ACCDA2_CD_SHIFT: u32 = 0; 
// #endif
// #ifndef NO_CS4612
pub const ACISV2_ISV3: u32 = 0x00000001; 
pub const ACISV2_ISV4: u32 = 0x00000002; 
pub const ACISV2_ISV5: u32 = 0x00000004; 
pub const ACISV2_ISV6: u32 = 0x00000008; 
pub const ACISV2_ISV7: u32 = 0x00000010; 
pub const ACISV2_ISV8: u32 = 0x00000020; 
pub const ACISV2_ISV9: u32 = 0x00000040; 
pub const ACISV2_ISV10: u32 = 0x00000080; 
pub const ACISV2_ISV11: u32 = 0x00000100; 
pub const ACISV2_ISV12: u32 = 0x00000200; 
// #endif
// #ifndef NO_CS4612
pub const ACSAD2_SI_MASK: u32 = 0x0000007F; 
pub const ACSAD2_SI_SHIFT: u32 = 0; 
// #endif
// #ifndef NO_CS4612
pub const ACSDA2_SD_MASK: u32 = 0x0000FFFF; 
pub const ACSDA2_SD_SHIFT: u32 = 0; 
// #endif
// #ifndef NO_CS4612
pub const IOTAC_SA_MASK: u32 = 0x0000FFFF; 
pub const IOTAC_MSK_MASK: u32 = 0x000F0000; 
pub const IOTAC_IODC_MASK: u32 = 0x06000000; 
pub const IOTAC_IODC_16_BIT: u32 = 0x00000000; 
pub const IOTAC_IODC_10_BIT: u32 = 0x02000000; 
pub const IOTAC_IODC_12_BIT: u32 = 0x04000000; 
pub const IOTAC_WSPI: u32 = 0x08000000; 
pub const IOTAC_RSPI: u32 = 0x10000000; 
pub const IOTAC_WSE: u32 = 0x20000000; 
pub const IOTAC_WE: u32 = 0x40000000; 
pub const IOTAC_RE: u32 = 0x80000000; 
pub const IOTAC_SA_SHIFT: u32 = 0; 
pub const IOTAC_MSK_SHIFT: u32 = 16; 
// #endif
// #ifndef NO_CS4612
pub const IOTFR_D_MASK: u32 = 0x0000FFFF; 
pub const IOTFR_A_MASK: u32 = 0x000F0000; 
pub const IOTFR_R_MASK: u32 = 0x0F000000; 
pub const IOTFR_ALL: u32 = 0x40000000; 
pub const IOTFR_VL: u32 = 0x80000000; 
pub const IOTFR_D_SHIFT: u32 = 0; 
pub const IOTFR_A_SHIFT: u32 = 16; 
pub const IOTFR_R_SHIFT: u32 = 24; 
// #endif
// #ifndef NO_CS4612
pub const IOTFIFO_BA_MASK: u32 = 0x00003FFF; 
pub const IOTFIFO_S_MASK: u32 = 0x00FF0000; 
pub const IOTFIFO_OF: u32 = 0x40000000; 
pub const IOTFIFO_SPIOF: u32 = 0x80000000; 
pub const IOTFIFO_BA_SHIFT: u32 = 0; 
pub const IOTFIFO_S_SHIFT: u32 = 16; 
// #endif
// #ifndef NO_CS4612
pub const IOTRRD_D_MASK: u32 = 0x0000FFFF; 
pub const IOTRRD_RDV: u32 = 0x80000000; 
pub const IOTRRD_D_SHIFT: u32 = 0; 
// #endif
// #ifndef NO_CS4612
pub const IOTFP_CA_MASK: u32 = 0x00003FFF; 
pub const IOTFP_PA_MASK: u32 = 0x3FFF0000; 
pub const IOTFP_CA_SHIFT: u32 = 0; 
pub const IOTFP_PA_SHIFT: u32 = 16; 
// #endif
// #ifndef NO_CS4612
pub const IOTCR_ITD: u32 = 0x00000001; 
pub const IOTCR_HRV: u32 = 0x00000002; 
pub const IOTCR_SRV: u32 = 0x00000004; 
pub const IOTCR_DTI: u32 = 0x00000008; 
pub const IOTCR_DFI: u32 = 0x00000010; 
pub const IOTCR_DDP: u32 = 0x00000020; 
pub const IOTCR_JTE: u32 = 0x00000040; 
pub const IOTCR_PPE: u32 = 0x00000080; 
// #endif
// #ifndef NO_CS4612
pub const DPCID_D_MASK: u32 = 0xFFFFFFFF; 
pub const DPCID_D_SHIFT: u32 = 0; 
// #endif
// #ifndef NO_CS4612
pub const DPCIA_A_MASK: u32 = 0xFFFFFFFF; 
pub const DPCIA_A_SHIFT: u32 = 0; 
// #endif
// #ifndef NO_CS4612
pub const DPCIC_C_MASK: u32 = 0x0000000F; 
pub const DPCIC_C_IOREAD: u32 = 0x00000002; 
pub const DPCIC_C_IOWRITE: u32 = 0x00000003; 
pub const DPCIC_BE_MASK: u32 = 0x000000F0; 
// #endif
// #ifndef NO_CS4612
pub const PCPCIR_RDC_MASK: u32 = 0x00000007; 
pub const PCPCIR_C_MASK: u32 = 0x00007000; 
pub const PCPCIR_REQ: u32 = 0x00008000; 
pub const PCPCIR_RDC_SHIFT: u32 = 0; 
pub const PCPCIR_C_SHIFT: u32 = 12; 
// #endif
// #ifndef NO_CS4612
pub const PCPCIG_GDC_MASK: u32 = 0x00000007; 
pub const PCPCIG_VL: u32 = 0x00008000; 
pub const PCPCIG_GDC_SHIFT: u32 = 0; 
// #endif
// #ifndef NO_CS4612
pub const PCPCIEN_EN: u32 = 0x00000001; 
// #endif
// #ifndef NO_CS4612
pub const EPCIPMC_GWU: u32 = 0x00000001; 
pub const EPCIPMC_FSPC: u32 = 0x00000002; 
// #endif 
pub const SPCR_RUN: u32 = 0x00000001; 
pub const SPCR_STPFR: u32 = 0x00000002; 
pub const SPCR_RUNFR: u32 = 0x00000004; 
pub const SPCR_TICK: u32 = 0x00000008; 
pub const SPCR_DRQEN: u32 = 0x00000020; 
pub const SPCR_RSTSP: u32 = 0x00000040; 
pub const SPCR_OREN: u32 = 0x00000080; 
// #ifndef NO_CS4612
pub const SPCR_PCIINT: u32 = 0x00000100; 
pub const SPCR_OINTD: u32 = 0x00000200; 
pub const SPCR_CRE: u32 = 0x00008000; 
// #endif
pub const DREG_REGID_MASK: u32 = 0x0000007F; 
pub const DREG_DEBUG: u32 = 0x00000080; 
pub const DREG_RGBK_MASK: u32 = 0x00000700; 
pub const DREG_TRAP: u32 = 0x00000800; 
// #if !defined(NO_CS4612)
// #if !defined(NO_CS4615)
pub const DREG_TRAPX: u32 = 0x00001000; 
// #endif
// #endif
pub const DREG_REGID_SHIFT: u32 = 0; 
pub const DREG_RGBK_SHIFT: u32 = 8; 
pub const DREG_RGBK_REGID_MASK: u32 = 0x0000077F; 
pub const DREG_REGID_R0: u32 = 0x00000010; 
pub const DREG_REGID_R1: u32 = 0x00000011; 
pub const DREG_REGID_R2: u32 = 0x00000012; 
pub const DREG_REGID_R3: u32 = 0x00000013; 
pub const DREG_REGID_R4: u32 = 0x00000014; 
pub const DREG_REGID_R5: u32 = 0x00000015; 
pub const DREG_REGID_R6: u32 = 0x00000016; 
pub const DREG_REGID_R7: u32 = 0x00000017; 
pub const DREG_REGID_R8: u32 = 0x00000018; 
pub const DREG_REGID_R9: u32 = 0x00000019; 
pub const DREG_REGID_RA: u32 = 0x0000001A; 
pub const DREG_REGID_RB: u32 = 0x0000001B; 
pub const DREG_REGID_RC: u32 = 0x0000001C; 
pub const DREG_REGID_RD: u32 = 0x0000001D; 
pub const DREG_REGID_RE: u32 = 0x0000001E; 
pub const DREG_REGID_RF: u32 = 0x0000001F; 
pub const DREG_REGID_RA_BUS_LOW: u32 = 0x00000020; 
pub const DREG_REGID_RA_BUS_HIGH: u32 = 0x00000038; 
pub const DREG_REGID_YBUS_LOW: u32 = 0x00000050; 
pub const DREG_REGID_YBUS_HIGH: u32 = 0x00000058; 
pub const DREG_REGID_TRAP_0: u32 = 0x00000100; 
pub const DREG_REGID_TRAP_1: u32 = 0x00000101; 
pub const DREG_REGID_TRAP_2: u32 = 0x00000102; 
pub const DREG_REGID_TRAP_3: u32 = 0x00000103; 
pub const DREG_REGID_TRAP_4: u32 = 0x00000104; 
pub const DREG_REGID_TRAP_5: u32 = 0x00000105; 
pub const DREG_REGID_TRAP_6: u32 = 0x00000106; 
pub const DREG_REGID_TRAP_7: u32 = 0x00000107; 
pub const DREG_REGID_INDIRECT_ADDRESS: u32 = 0x0000010E; 
pub const DREG_REGID_TOP_OF_STACK: u32 = 0x0000010F; 
// #if !defined(NO_CS4612)
// #if !defined(NO_CS4615)
pub const DREG_REGID_TRAP_8: u32 = 0x00000110; 
pub const DREG_REGID_TRAP_9: u32 = 0x00000111; 
pub const DREG_REGID_TRAP_10: u32 = 0x00000112; 
pub const DREG_REGID_TRAP_11: u32 = 0x00000113; 
pub const DREG_REGID_TRAP_12: u32 = 0x00000114; 
pub const DREG_REGID_TRAP_13: u32 = 0x00000115; 
pub const DREG_REGID_TRAP_14: u32 = 0x00000116; 
pub const DREG_REGID_TRAP_15: u32 = 0x00000117; 
pub const DREG_REGID_TRAP_16: u32 = 0x00000118; 
pub const DREG_REGID_TRAP_17: u32 = 0x00000119; 
pub const DREG_REGID_TRAP_18: u32 = 0x0000011A; 
pub const DREG_REGID_TRAP_19: u32 = 0x0000011B; 
pub const DREG_REGID_TRAP_20: u32 = 0x0000011C; 
pub const DREG_REGID_TRAP_21: u32 = 0x0000011D; 
pub const DREG_REGID_TRAP_22: u32 = 0x0000011E; 
pub const DREG_REGID_TRAP_23: u32 = 0x0000011F; 
// #endif
// #endif
pub const DREG_REGID_RSA0_LOW: u32 = 0x00000200; 
pub const DREG_REGID_RSA0_HIGH: u32 = 0x00000201; 
pub const DREG_REGID_RSA1_LOW: u32 = 0x00000202; 
pub const DREG_REGID_RSA1_HIGH: u32 = 0x00000203; 
pub const DREG_REGID_RSA2: u32 = 0x00000204; 
pub const DREG_REGID_RSA3: u32 = 0x00000205; 
pub const DREG_REGID_RSI0_LOW: u32 = 0x00000206; 
pub const DREG_REGID_RSI0_HIGH: u32 = 0x00000207; 
pub const DREG_REGID_RSI1: u32 = 0x00000208; 
pub const DREG_REGID_RSI2: u32 = 0x00000209; 
pub const DREG_REGID_SAGUSTATUS: u32 = 0x0000020A; 
pub const DREG_REGID_RSCONFIG01_LOW: u32 = 0x0000020B; 
pub const DREG_REGID_RSCONFIG01_HIGH: u32 = 0x0000020C; 
pub const DREG_REGID_RSCONFIG23_LOW: u32 = 0x0000020D; 
pub const DREG_REGID_RSCONFIG23_HIGH: u32 = 0x0000020E; 
pub const DREG_REGID_RSDMA01E: u32 = 0x0000020F; 
pub const DREG_REGID_RSDMA23E: u32 = 0x00000210; 
pub const DREG_REGID_RSD0_LOW: u32 = 0x00000211; 
pub const DREG_REGID_RSD0_HIGH: u32 = 0x00000212; 
pub const DREG_REGID_RSD1_LOW: u32 = 0x00000213; 
pub const DREG_REGID_RSD1_HIGH: u32 = 0x00000214; 
pub const DREG_REGID_RSD2_LOW: u32 = 0x00000215; 
pub const DREG_REGID_RSD2_HIGH: u32 = 0x00000216; 
pub const DREG_REGID_RSD3_LOW: u32 = 0x00000217; 
pub const DREG_REGID_RSD3_HIGH: u32 = 0x00000218; 
pub const DREG_REGID_SRAR_HIGH: u32 = 0x0000021A; 
pub const DREG_REGID_SRAR_LOW: u32 = 0x0000021B; 
pub const DREG_REGID_DMA_STATE: u32 = 0x0000021C; 
pub const DREG_REGID_CURRENT_DMA_STREAM: u32 = 0x0000021D; 
pub const DREG_REGID_NEXT_DMA_STREAM: u32 = 0x0000021E; 
pub const DREG_REGID_CPU_STATUS: u32 = 0x00000300; 
pub const DREG_REGID_MAC_MODE: u32 = 0x00000301; 
pub const DREG_REGID_STACK_AND_REPEAT: u32 = 0x00000302; 
pub const DREG_REGID_INDEX0: u32 = 0x00000304; 
pub const DREG_REGID_INDEX1: u32 = 0x00000305; 
pub const DREG_REGID_DMA_STATE_0_3: u32 = 0x00000400; 
pub const DREG_REGID_DMA_STATE_4_7: u32 = 0x00000404; 
pub const DREG_REGID_DMA_STATE_8_11: u32 = 0x00000408; 
pub const DREG_REGID_DMA_STATE_12_15: u32 = 0x0000040C; 
pub const DREG_REGID_DMA_STATE_16_19: u32 = 0x00000410; 
pub const DREG_REGID_DMA_STATE_20_23: u32 = 0x00000414; 
pub const DREG_REGID_DMA_STATE_24_27: u32 = 0x00000418; 
pub const DREG_REGID_DMA_STATE_28_31: u32 = 0x0000041C; 
pub const DREG_REGID_DMA_STATE_32_35: u32 = 0x00000420; 
pub const DREG_REGID_DMA_STATE_36_39: u32 = 0x00000424; 
pub const DREG_REGID_DMA_STATE_40_43: u32 = 0x00000428; 
pub const DREG_REGID_DMA_STATE_44_47: u32 = 0x0000042C; 
pub const DREG_REGID_DMA_STATE_48_51: u32 = 0x00000430; 
pub const DREG_REGID_DMA_STATE_52_55: u32 = 0x00000434; 
pub const DREG_REGID_DMA_STATE_56_59: u32 = 0x00000438; 
pub const DREG_REGID_DMA_STATE_60_63: u32 = 0x0000043C; 
pub const DREG_REGID_DMA_STATE_64_67: u32 = 0x00000440; 
pub const DREG_REGID_DMA_STATE_68_71: u32 = 0x00000444; 
pub const DREG_REGID_DMA_STATE_72_75: u32 = 0x00000448; 
pub const DREG_REGID_DMA_STATE_76_79: u32 = 0x0000044C; 
pub const DREG_REGID_DMA_STATE_80_83: u32 = 0x00000450; 
pub const DREG_REGID_DMA_STATE_84_87: u32 = 0x00000454; 
pub const DREG_REGID_DMA_STATE_88_91: u32 = 0x00000458; 
pub const DREG_REGID_DMA_STATE_92_95: u32 = 0x0000045C; 
pub const DREG_REGID_TRAP_SELECT: u32 = 0x00000500; 
pub const DREG_REGID_TRAP_WRITE_0: u32 = 0x00000500; 
pub const DREG_REGID_TRAP_WRITE_1: u32 = 0x00000501; 
pub const DREG_REGID_TRAP_WRITE_2: u32 = 0x00000502; 
pub const DREG_REGID_TRAP_WRITE_3: u32 = 0x00000503; 
pub const DREG_REGID_TRAP_WRITE_4: u32 = 0x00000504; 
pub const DREG_REGID_TRAP_WRITE_5: u32 = 0x00000505; 
pub const DREG_REGID_TRAP_WRITE_6: u32 = 0x00000506; 
pub const DREG_REGID_TRAP_WRITE_7: u32 = 0x00000507; 
// #if !defined(NO_CS4612)
// #if !defined(NO_CS4615)
pub const DREG_REGID_TRAP_WRITE_8: u32 = 0x00000510; 
pub const DREG_REGID_TRAP_WRITE_9: u32 = 0x00000511; 
pub const DREG_REGID_TRAP_WRITE_10: u32 = 0x00000512; 
pub const DREG_REGID_TRAP_WRITE_11: u32 = 0x00000513; 
pub const DREG_REGID_TRAP_WRITE_12: u32 = 0x00000514; 
pub const DREG_REGID_TRAP_WRITE_13: u32 = 0x00000515; 
pub const DREG_REGID_TRAP_WRITE_14: u32 = 0x00000516; 
pub const DREG_REGID_TRAP_WRITE_15: u32 = 0x00000517; 
pub const DREG_REGID_TRAP_WRITE_16: u32 = 0x00000518; 
pub const DREG_REGID_TRAP_WRITE_17: u32 = 0x00000519; 
pub const DREG_REGID_TRAP_WRITE_18: u32 = 0x0000051A; 
pub const DREG_REGID_TRAP_WRITE_19: u32 = 0x0000051B; 
pub const DREG_REGID_TRAP_WRITE_20: u32 = 0x0000051C; 
pub const DREG_REGID_TRAP_WRITE_21: u32 = 0x0000051D; 
pub const DREG_REGID_TRAP_WRITE_22: u32 = 0x0000051E; 
pub const DREG_REGID_TRAP_WRITE_23: u32 = 0x0000051F; 
// #endif
// #endif
pub const DREG_REGID_MAC0_ACC0_LOW: u32 = 0x00000600; 
pub const DREG_REGID_MAC0_ACC1_LOW: u32 = 0x00000601; 
pub const DREG_REGID_MAC0_ACC2_LOW: u32 = 0x00000602; 
pub const DREG_REGID_MAC0_ACC3_LOW: u32 = 0x00000603; 
pub const DREG_REGID_MAC1_ACC0_LOW: u32 = 0x00000604; 
pub const DREG_REGID_MAC1_ACC1_LOW: u32 = 0x00000605; 
pub const DREG_REGID_MAC1_ACC2_LOW: u32 = 0x00000606; 
pub const DREG_REGID_MAC1_ACC3_LOW: u32 = 0x00000607; 
pub const DREG_REGID_MAC0_ACC0_MID: u32 = 0x00000608; 
pub const DREG_REGID_MAC0_ACC1_MID: u32 = 0x00000609; 
pub const DREG_REGID_MAC0_ACC2_MID: u32 = 0x0000060A; 
pub const DREG_REGID_MAC0_ACC3_MID: u32 = 0x0000060B; 
pub const DREG_REGID_MAC1_ACC0_MID: u32 = 0x0000060C; 
pub const DREG_REGID_MAC1_ACC1_MID: u32 = 0x0000060D; 
pub const DREG_REGID_MAC1_ACC2_MID: u32 = 0x0000060E; 
pub const DREG_REGID_MAC1_ACC3_MID: u32 = 0x0000060F; 
pub const DREG_REGID_MAC0_ACC0_HIGH: u32 = 0x00000610; 
pub const DREG_REGID_MAC0_ACC1_HIGH: u32 = 0x00000611; 
pub const DREG_REGID_MAC0_ACC2_HIGH: u32 = 0x00000612; 
pub const DREG_REGID_MAC0_ACC3_HIGH: u32 = 0x00000613; 
pub const DREG_REGID_MAC1_ACC0_HIGH: u32 = 0x00000614; 
pub const DREG_REGID_MAC1_ACC1_HIGH: u32 = 0x00000615; 
pub const DREG_REGID_MAC1_ACC2_HIGH: u32 = 0x00000616; 
pub const DREG_REGID_MAC1_ACC3_HIGH: u32 = 0x00000617; 
pub const DREG_REGID_RSHOUT_LOW: u32 = 0x00000620; 
pub const DREG_REGID_RSHOUT_MID: u32 = 0x00000628; 
pub const DREG_REGID_RSHOUT_HIGH: u32 = 0x00000630; 
pub const DSRWP_DSR_MASK: u32 = 0x0000000F; 
pub const DSRWP_DSR_BG_RQ: u32 = 0x00000001; 
pub const DSRWP_DSR_PRIORITY_MASK: u32 = 0x00000006; 
pub const DSRWP_DSR_PRIORITY_0: u32 = 0x00000000; 
pub const DSRWP_DSR_PRIORITY_1: u32 = 0x00000002; 
pub const DSRWP_DSR_PRIORITY_2: u32 = 0x00000004; 
pub const DSRWP_DSR_PRIORITY_3: u32 = 0x00000006; 
pub const DSRWP_DSR_RQ_PENDING: u32 = 0x00000008; 
pub const TWPR_TW_MASK: u32 = 0x0000FFFF; 
pub const TWPR_TW_SHIFT: u32 = 0; 
pub const SPWR_STKP_MASK: u32 = 0x0000000F; 
pub const SPWR_STKP_SHIFT: u32 = 0; 
pub const SPIR_FRI: u32 = 0x00000001; 
pub const SPIR_DOI: u32 = 0x00000002; 
pub const SPIR_GPI2: u32 = 0x00000004; 
pub const SPIR_GPI3: u32 = 0x00000008; 
pub const SPIR_IP0: u32 = 0x00000010; 
pub const SPIR_IP1: u32 = 0x00000020; 
pub const SPIR_IP2: u32 = 0x00000040; 
pub const SPIR_IP3: u32 = 0x00000080; 
pub const FGR1_F1S_MASK: u32 = 0x0000FFFF; 
pub const FGR1_F1S_SHIFT: u32 = 0; 
pub const SPCS_FRI: u32 = 0x00000001; 
pub const SPCS_DOI: u32 = 0x00000002; 
pub const SPCS_GPI2: u32 = 0x00000004; 
pub const SPCS_GPI3: u32 = 0x00000008; 
pub const SPCS_IP0: u32 = 0x00000010; 
pub const SPCS_IP1: u32 = 0x00000020; 
pub const SPCS_IP2: u32 = 0x00000040; 
pub const SPCS_IP3: u32 = 0x00000080; 
pub const SPCS_SPRUN: u32 = 0x00000100; 
pub const SPCS_SLEEP: u32 = 0x00000200; 
pub const SPCS_FG: u32 = 0x00000400; 
pub const SPCS_ORUN: u32 = 0x00000800; 
pub const SPCS_IRQ: u32 = 0x00001000; 
pub const SPCS_FGN_MASK: u32 = 0x0000E000; 
pub const SPCS_FGN_SHIFT: u32 = 13; 
pub const SDSR_DCS_MASK: u32 = 0x000000FF; 
pub const SDSR_DCS_SHIFT: u32 = 0; 
pub const SDSR_DCS_NONE: u32 = 0x00000007; 
pub const FRMT_FTV_MASK: u32 = 0x0000FFFF; 
pub const FRMT_FTV_SHIFT: u32 = 0; 
pub const FRCC_FCC_MASK: u32 = 0x0000FFFF; 
pub const FRCC_FCC_SHIFT: u32 = 0; 
pub const FRSC_FCS_MASK: u32 = 0x0000FFFF; 
pub const FRSC_FCS_SHIFT: u32 = 0; 
pub const DMA_SG_NEXT_ENTRY_MASK: u32 = 0x00000FF8; 
pub const DMA_SG_SAMPLE_END_MASK: u32 = 0x0FFF0000; 
pub const DMA_SG_SAMPLE_END_FLAG: u32 = 0x10000000; 
pub const DMA_SG_LOOP_END_FLAG: u32 = 0x20000000; 
pub const DMA_SG_SIGNAL_END_FLAG: u32 = 0x40000000; 
pub const DMA_SG_SIGNAL_PAGE_FLAG: u32 = 0x80000000; 
pub const DMA_SG_NEXT_ENTRY_SHIFT: u32 = 3; 
pub const DMA_SG_SAMPLE_END_SHIFT: u32 = 16; 
pub const DMA_RQ_CONTROL1: u32 = 0x00000000; 
pub const DMA_RQ_CONTROL2: u32 = 0x00000004; 
pub const DMA_RQ_SOURCE_ADDR: u32 = 0x00000008; 
pub const DMA_RQ_DESTINATION_ADDR: u32 = 0x0000000C; 
pub const DMA_RQ_NEXT_PAGE_ADDR: u32 = 0x00000010; 
pub const DMA_RQ_NEXT_PAGE_SGDESC: u32 = 0x00000014; 
pub const DMA_RQ_LOOP_START_ADDR: u32 = 0x00000018; 
pub const DMA_RQ_POST_LOOP_ADDR: u32 = 0x0000001C; 
pub const DMA_RQ_PAGE_MAP_ADDR: u32 = 0x00000020; 
pub const DMA_RQ_C1_COUNT_MASK: u32 = 0x000003FF; 
pub const DMA_RQ_C1_DESTINATION_SCATTER: u32 = 0x00001000; 
pub const DMA_RQ_C1_SOURCE_GATHER: u32 = 0x00002000; 
pub const DMA_RQ_C1_DONE_FLAG: u32 = 0x00004000; 
pub const DMA_RQ_C1_OPTIMIZE_STATE: u32 = 0x00008000; 
pub const DMA_RQ_C1_SAMPLE_END_STATE_MASK: u32 = 0x00030000; 
pub const DMA_RQ_C1_FULL_PAGE: u32 = 0x00000000; 
pub const DMA_RQ_C1_BEFORE_SAMPLE_END: u32 = 0x00010000; 
pub const DMA_RQ_C1_PAGE_MAP_ERROR: u32 = 0x00020000; 
pub const DMA_RQ_C1_AT_SAMPLE_END: u32 = 0x00030000; 
pub const DMA_RQ_C1_LOOP_END_STATE_MASK: u32 = 0x000C0000; 
pub const DMA_RQ_C1_NOT_LOOP_END: u32 = 0x00000000; 
pub const DMA_RQ_C1_BEFORE_LOOP_END: u32 = 0x00040000; 
pub const DMA_RQ_C1_2PAGE_LOOP_BEGIN: u32 = 0x00080000; 
pub const DMA_RQ_C1_LOOP_BEGIN: u32 = 0x000C0000; 
pub const DMA_RQ_C1_PAGE_MAP_MASK: u32 = 0x00300000; 
pub const DMA_RQ_C1_PM_NONE_PENDING: u32 = 0x00000000; 
pub const DMA_RQ_C1_PM_NEXT_PENDING: u32 = 0x00100000; 
pub const DMA_RQ_C1_PM_RESERVED: u32 = 0x00200000; 
pub const DMA_RQ_C1_PM_LOOP_NEXT_PENDING: u32 = 0x00300000; 
pub const DMA_RQ_C1_WRITEBACK_DEST_FLAG: u32 = 0x00400000; 
pub const DMA_RQ_C1_WRITEBACK_SRC_FLAG: u32 = 0x00800000; 
pub const DMA_RQ_C1_DEST_SIZE_MASK: u32 = 0x07000000; 
pub const DMA_RQ_C1_DEST_LINEAR: u32 = 0x00000000; 
pub const DMA_RQ_C1_DEST_MOD16: u32 = 0x01000000; 
pub const DMA_RQ_C1_DEST_MOD32: u32 = 0x02000000; 
pub const DMA_RQ_C1_DEST_MOD64: u32 = 0x03000000; 
pub const DMA_RQ_C1_DEST_MOD128: u32 = 0x04000000; 
pub const DMA_RQ_C1_DEST_MOD256: u32 = 0x05000000; 
pub const DMA_RQ_C1_DEST_MOD512: u32 = 0x06000000; 
pub const DMA_RQ_C1_DEST_MOD1024: u32 = 0x07000000; 
pub const DMA_RQ_C1_DEST_ON_HOST: u32 = 0x08000000; 
pub const DMA_RQ_C1_SOURCE_SIZE_MASK: u32 = 0x70000000; 
pub const DMA_RQ_C1_SOURCE_LINEAR: u32 = 0x00000000; 
pub const DMA_RQ_C1_SOURCE_MOD16: u32 = 0x10000000; 
pub const DMA_RQ_C1_SOURCE_MOD32: u32 = 0x20000000; 
pub const DMA_RQ_C1_SOURCE_MOD64: u32 = 0x30000000; 
pub const DMA_RQ_C1_SOURCE_MOD128: u32 = 0x40000000; 
pub const DMA_RQ_C1_SOURCE_MOD256: u32 = 0x50000000; 
pub const DMA_RQ_C1_SOURCE_MOD512: u32 = 0x60000000; 
pub const DMA_RQ_C1_SOURCE_MOD1024: u32 = 0x70000000; 
pub const DMA_RQ_C1_SOURCE_ON_HOST: u32 = 0x80000000; 
pub const DMA_RQ_C1_COUNT_SHIFT: u32 = 0; 
pub const DMA_RQ_C2_VIRTUAL_CHANNEL_MASK: u32 = 0x0000003F; 
pub const DMA_RQ_C2_VIRTUAL_SIGNAL_MASK: u32 = 0x00000300; 
pub const DMA_RQ_C2_NO_VIRTUAL_SIGNAL: u32 = 0x00000000; 
pub const DMA_RQ_C2_SIGNAL_EVERY_DMA: u32 = 0x00000100; 
pub const DMA_RQ_C2_SIGNAL_SOURCE_PINGPONG: u32 = 0x00000200; 
pub const DMA_RQ_C2_SIGNAL_DEST_PINGPONG: u32 = 0x00000300; 
pub const DMA_RQ_C2_AUDIO_CONVERT_MASK: u32 = 0x0000F000; 
pub const DMA_RQ_C2_AC_NONE: u32 = 0x00000000; 
pub const DMA_RQ_C2_AC_8_TO_16_BIT: u32 = 0x00001000; 
pub const DMA_RQ_C2_AC_MONO_TO_STEREO: u32 = 0x00002000; 
pub const DMA_RQ_C2_AC_ENDIAN_CONVERT: u32 = 0x00004000; 
pub const DMA_RQ_C2_AC_SIGNED_CONVERT: u32 = 0x00008000; 
pub const DMA_RQ_C2_LOOP_END_MASK: u32 = 0x0FFF0000; 
pub const DMA_RQ_C2_LOOP_MASK: u32 = 0x30000000; 
pub const DMA_RQ_C2_NO_LOOP: u32 = 0x00000000; 
pub const DMA_RQ_C2_ONE_PAGE_LOOP: u32 = 0x10000000; 
pub const DMA_RQ_C2_TWO_PAGE_LOOP: u32 = 0x20000000; 
pub const DMA_RQ_C2_MULTI_PAGE_LOOP: u32 = 0x30000000; 
pub const DMA_RQ_C2_SIGNAL_LOOP_BACK: u32 = 0x40000000; 
pub const DMA_RQ_C2_SIGNAL_POST_BEGIN_PAGE: u32 = 0x80000000; 
pub const DMA_RQ_C2_VIRTUAL_CHANNEL_SHIFT: u32 = 0; 
pub const DMA_RQ_C2_LOOP_END_SHIFT: u32 = 16; 
pub const DMA_RQ_SD_ADDRESS_MASK: u32 = 0x0000FFFF; 
pub const DMA_RQ_SD_MEMORY_ID_MASK: u32 = 0x000F0000; 
pub const DMA_RQ_SD_SP_PARAM_ADDR: u32 = 0x00000000; 
pub const DMA_RQ_SD_SP_SAMPLE_ADDR: u32 = 0x00010000; 
pub const DMA_RQ_SD_SP_PROGRAM_ADDR: u32 = 0x00020000; 
pub const DMA_RQ_SD_SP_DEBUG_ADDR: u32 = 0x00030000; 
pub const DMA_RQ_SD_OMNIMEM_ADDR: u32 = 0x000E0000; 
pub const DMA_RQ_SD_END_FLAG: u32 = 0x40000000; 
pub const DMA_RQ_SD_ERROR_FLAG: u32 = 0x80000000; 
pub const DMA_RQ_SD_ADDRESS_SHIFT: u32 = 0; 
pub const DMA_RQ_PMA_LOOP_THIRD_PAGE_ENTRY_MASK: u32 = 0x00000FF8; 
pub const DMA_RQ_PMA_PAGE_TABLE_MASK: u32 = 0xFFFFF000; 
pub const DMA_RQ_PMA_LOOP_THIRD_PAGE_ENTRY_SHIFT: u32 = 3; 
pub const DMA_RQ_PMA_PAGE_TABLE_SHIFT: u32 = 12; 
pub const BA1_VARIDEC_BUF_1: u32 = 0x000; 
pub const BA1_PDTC: u32 = 0x0c0; /* BA1_PLAY_DMA_TRANSACTION_COUNT_REG */
pub const BA1_PFIE: u32 = 0x0c4; /* BA1_PLAY_FORMAT_&_INTERRUPT_ENABLE_REG */
pub const BA1_PBA: u32 = 0x0c8; /* BA1_PLAY_BUFFER_ADDRESS */
pub const BA1_PVOL: u32 = 0x0f8; /* BA1_PLAY_VOLUME_REG */
pub const BA1_PSRC: u32 = 0x288; /* BA1_PLAY_SAMPLE_RATE_CORRECTION_REG */
pub const BA1_PCTL: u32 = 0x2a4; /* BA1_PLAY_CONTROL_REG */
pub const BA1_PPI: u32 = 0x2b4; /* BA1_PLAY_PHASE_INCREMENT_REG */
pub const BA1_CCTL: u32 = 0x064; /* BA1_CAPTURE_CONTROL_REG */
pub const BA1_CIE: u32 = 0x104; /* BA1_CAPTURE_INTERRUPT_ENABLE_REG */
pub const BA1_CBA: u32 = 0x10c; /* BA1_CAPTURE_BUFFER_ADDRESS */
pub const BA1_CSRC: u32 = 0x2c8; /* BA1_CAPTURE_SAMPLE_RATE_CORRECTION_REG */
pub const BA1_CCI: u32 = 0x2d8; /* BA1_CAPTURE_COEFFICIENT_INCREMENT_REG */
pub const BA1_CD: u32 = 0x2e0; /* BA1_CAPTURE_DELAY_REG */
pub const BA1_CPI: u32 = 0x2f4; /* BA1_CAPTURE_PHASE_INCREMENT_REG */
pub const BA1_CVOL: u32 = 0x2f8; /* BA1_CAPTURE_VOLUME_REG */
pub const BA1_CFG1: u32 = 0x134; /* BA1_CAPTURE_FRAME_GROUP_1_REG */
pub const BA1_CFG2: u32 = 0x138; /* BA1_CAPTURE_FRAME_GROUP_2_REG */
pub const BA1_CCST: u32 = 0x13c; /* BA1_CAPTURE_CONSTANT_REG */
pub const BA1_CSPB: u32 = 0x340; /* BA1_CAPTURE_SPB_ADDRESS */
pub const CS46XX_MODE_OUTPUT: u32 = (1u32 << 0); /* MIDI UART - output */ 
pub const CS46XX_MODE_INPUT: u32 = (1u32 << 1); /* MIDI UART - input */
pub const SAVE_REG_MAX: u32 = 0x10; 
pub const POWER_DOWN_ALL: u32 = 0x7f0f; 
pub const MAX_NR_AC97: usize = 4usize; 
pub const CS46XX_PRIMARY_CODEC_INDEX: u32 = 0; 
pub const CS46XX_SECONDARY_CODEC_INDEX: u32 = 1; 
pub const CS46XX_SECONDARY_CODEC_OFFSET: u32 = 0x80; 
pub const CS46XX_DSP_CAPTURE_CHANNEL: u32 = 1; 
pub const CS46XX_DSP_CAPTURE_CHANNEL: u32 = 1; 
pub const CS46XX_MIXER_SPDIF_INPUT_ELEMENT: u32 = 1; 
pub const CS46XX_MIXER_SPDIF_OUTPUT_ELEMENT: u32 = 2; 
// #ifdef CONFIG_SND_CS46XX_NEW_DSP
pub const CS46XX_DSP_MODULES: usize = 5usize; 
// #else /* for compatibility */
// #endif
// #ifdef CONFIG_PM_SLEEP
// #endif
// #endif /* __SOUND_CS46XX_H */

#[repr(C)]
pub struct snd_cs46xx_pcm {
    pub hw_buf: snd_dma_buffer,
    pub ctl: c_uint,
    pub shift: c_uint, /* Shift count to trasform frames in bytes */
    pub pcm_rec: snd_pcm_indirect,
    pub substream: *mut snd_pcm_substream,
    pub pcm_channel: *mut dsp_pcm_channel_descriptor,
    pub pcm_channel_id: c_int, /* Fron Rear, Center Lfe  ... */
}

#[repr(C)]
pub struct snd_cs46xx_region {
    pub name: [::core::ffi::c_char; 24],
    pub base: c_ulong,
    pub remap_addr: *mut ::core::ffi::c_void,
    pub size: c_ulong,
}

#[repr(C)]
pub struct snd_cs46xx_region_name {
    pub ba0: snd_cs46xx_region,
    pub data0: snd_cs46xx_region,
    pub data1: snd_cs46xx_region,
    pub pmem: snd_cs46xx_region,
    pub reg: snd_cs46xx_region,
}

#[repr(C)]
pub union snd_cs46xx_region_union {
    pub name: ::core::mem::ManuallyDrop<snd_cs46xx_region_name>,
    pub idx: ::core::mem::ManuallyDrop<[snd_cs46xx_region; 5]>,
}

#[repr(C)]
pub struct snd_cs46xx_capt {
    pub hw_buf: snd_dma_buffer,
    pub ctl: c_uint,
    pub shift: c_uint, /* Shift count to trasform frames in bytes */
    pub pcm_rec: snd_pcm_indirect,
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_cs46xx {
    pub irq: c_int,
    pub ba0_addr: c_ulong,
    pub ba1_addr: c_ulong,
    pub region: snd_cs46xx_region_union,
    pub mode: c_uint,
    pub capt: snd_cs46xx_capt,
    pub nr_ac97_codecs: c_int,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: [*mut snd_ac97; MAX_NR_AC97],
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub rmidi: *mut snd_rawmidi,
    pub midi_input: *mut snd_rawmidi_substream,
    pub midi_output: *mut snd_rawmidi_substream,
    pub reg_lock: spinlock_t,
    pub midcr: c_uint,
    pub uartm: c_uint,
    pub amplifier: c_int,
    pub amplifier_ctrl: Option<unsafe extern "C" fn(*mut snd_cs46xx, c_int)>,
    pub active_ctrl: Option<unsafe extern "C" fn(*mut snd_cs46xx, c_int)>,
    pub mixer_init: Option<unsafe extern "C" fn(*mut snd_cs46xx)>,
    pub acpi_port: c_int,
    pub eapd_switch: *mut snd_kcontrol, /* for amplifier hack */
    pub accept_valid: c_int, /* accept mmap valid (for OSS) */
    pub in_suspend: c_int,
    pub gameport: *mut gameport,

    // #ifdef CONFIG_SND_CS46XX_NEW_DSP
    pub spos_mutex: mutex,
    pub dsp_spos_instance: *mut dsp_spos_instance,
    pub pcm_rear: *mut snd_pcm,
    pub pcm_center_lfe: *mut snd_pcm,
    pub pcm_iec958: *mut snd_pcm,
    pub modules: [*mut dsp_module_desc; CS46XX_DSP_MODULES],
    // #else /* for compatibility */
    pub playback_pcm: *mut snd_cs46xx_pcm,
    pub play_ctl: c_uint,
    pub ba1: *mut ba1_struct,
    // #endif

    // #ifdef CONFIG_PM_SLEEP
    pub saved_regs: *mut u32,
    // #endif
}

unsafe extern "C" {
    pub fn snd_cs46xx_create(
        card: *mut snd_card,
        pci: *mut pci_dev,
        external_amp: c_int,
        thinkpad: c_int,
    ) -> c_int;
    pub static snd_cs46xx_pm: dev_pm_ops;

    pub fn snd_cs46xx_pcm(chip: *mut snd_cs46xx, device: c_int) -> c_int;
    pub fn snd_cs46xx_pcm_rear(chip: *mut snd_cs46xx, device: c_int) -> c_int;
    pub fn snd_cs46xx_pcm_iec958(chip: *mut snd_cs46xx, device: c_int) -> c_int;
    pub fn snd_cs46xx_pcm_center_lfe(chip: *mut snd_cs46xx, device: c_int) -> c_int;
    pub fn snd_cs46xx_mixer(chip: *mut snd_cs46xx, spdif_device: c_int) -> c_int;
    pub fn snd_cs46xx_midi(chip: *mut snd_cs46xx, device: c_int) -> c_int;
    pub fn snd_cs46xx_start_dsp(chip: *mut snd_cs46xx) -> c_int;
    pub fn snd_cs46xx_gameport(chip: *mut snd_cs46xx) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
