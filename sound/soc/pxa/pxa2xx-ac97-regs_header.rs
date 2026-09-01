/* SPDX-License-Identifier: GPL-2.0 */

/*
 * AC97 Controller registers
 */

pub const POCR: u32 = 0x0000; /* PCM Out Control Register */
pub const POCR_FEIE: u32 = 1 << 3; /* FIFO Error Interrupt Enable */
pub const POCR_FSRIE: u32 = 1 << 1; /* FIFO Service Request Interrupt Enable */

pub const PICR: u32 = 0x0004; /* PCM In Control Register */
pub const PICR_FEIE: u32 = 1 << 3; /* FIFO Error Interrupt Enable */
pub const PICR_FSRIE: u32 = 1 << 1; /* FIFO Service Request Interrupt Enable */

pub const MCCR: u32 = 0x0008; /* Mic In Control Register */
pub const MCCR_FEIE: u32 = 1 << 3; /* FIFO Error Interrupt Enable */
pub const MCCR_FSRIE: u32 = 1 << 1; /* FIFO Service Request Interrupt Enable */

pub const GCR: u32 = 0x000C; /* Global Control Register */
/* Defined in C only when CONFIG_PXA3xx is enabled. */
pub const GCR_CLKBPB: u32 = 1 << 31; /* Internal clock enable */
pub const GCR_nDMAEN: u32 = 1 << 24; /* non DMA Enable */
pub const GCR_CDONE_IE: u32 = 1 << 19; /* Command Done Interrupt Enable */
pub const GCR_SDONE_IE: u32 = 1 << 18; /* Status Done Interrupt Enable */
pub const GCR_SECRDY_IEN: u32 = 1 << 9; /* Secondary Ready Interrupt Enable */
pub const GCR_PRIRDY_IEN: u32 = 1 << 8; /* Primary Ready Interrupt Enable */
pub const GCR_SECRES_IEN: u32 = 1 << 5; /* Secondary Resume Interrupt Enable */
pub const GCR_PRIRES_IEN: u32 = 1 << 4; /* Primary Resume Interrupt Enable */
pub const GCR_ACLINK_OFF: u32 = 1 << 3; /* AC-link Shut Off */
pub const GCR_WARM_RST: u32 = 1 << 2; /* AC97 Warm Reset */
pub const GCR_COLD_RST: u32 = 1 << 1; /* AC'97 Cold Reset (0 = active) */
pub const GCR_GIE: u32 = 1 << 0; /* Codec GPI Interrupt Enable */

pub const POSR: u32 = 0x0010; /* PCM Out Status Register */
pub const POSR_FIFOE: u32 = 1 << 4; /* FIFO error */
pub const POSR_FSR: u32 = 1 << 2; /* FIFO Service Request */

pub const PISR: u32 = 0x0014; /* PCM In Status Register */
pub const PISR_FIFOE: u32 = 1 << 4; /* FIFO error */
pub const PISR_EOC: u32 = 1 << 3; /* DMA End-of-Chain (exclusive clear) */
pub const PISR_FSR: u32 = 1 << 2; /* FIFO Service Request */

pub const MCSR: u32 = 0x0018; /* Mic In Status Register */
pub const MCSR_FIFOE: u32 = 1 << 4; /* FIFO error */
pub const MCSR_EOC: u32 = 1 << 3; /* DMA End-of-Chain (exclusive clear) */
pub const MCSR_FSR: u32 = 1 << 2; /* FIFO Service Request */

pub const GSR: u32 = 0x001C; /* Global Status Register */
pub const GSR_CDONE: u32 = 1 << 19; /* Command Done */
pub const GSR_SDONE: u32 = 1 << 18; /* Status Done */
pub const GSR_RDCS: u32 = 1 << 15; /* Read Completion Status */
pub const GSR_BIT3SLT12: u32 = 1 << 14; /* Bit 3 of slot 12 */
pub const GSR_BIT2SLT12: u32 = 1 << 13; /* Bit 2 of slot 12 */
pub const GSR_BIT1SLT12: u32 = 1 << 12; /* Bit 1 of slot 12 */
pub const GSR_SECRES: u32 = 1 << 11; /* Secondary Resume Interrupt */
pub const GSR_PRIRES: u32 = 1 << 10; /* Primary Resume Interrupt */
pub const GSR_SCR: u32 = 1 << 9; /* Secondary Codec Ready */
pub const GSR_PCR: u32 = 1 << 8; /*  Primary Codec Ready */
pub const GSR_MCINT: u32 = 1 << 7; /* Mic In Interrupt */
pub const GSR_POINT: u32 = 1 << 6; /* PCM Out Interrupt */
pub const GSR_PIINT: u32 = 1 << 5; /* PCM In Interrupt */
pub const GSR_ACOFFD: u32 = 1 << 3; /* AC-link Shut Off Done */
pub const GSR_MOINT: u32 = 1 << 2; /* Modem Out Interrupt */
pub const GSR_MIINT: u32 = 1 << 1; /* Modem In Interrupt */
pub const GSR_GSCI: u32 = 1 << 0; /* Codec GPI Status Change Interrupt */

pub const CAR: u32 = 0x0020; /* CODEC Access Register */
pub const CAR_CAIP: u32 = 1 << 0; /* Codec Access In Progress */

pub const PCDR: u32 = 0x0040; /* PCM FIFO Data Register */
pub const MCDR: u32 = 0x0060; /* Mic-in FIFO Data Register */

pub const MOCR: u32 = 0x0100; /* Modem Out Control Register */
pub const MOCR_FEIE: u32 = 1 << 3; /* FIFO Error */
pub const MOCR_FSRIE: u32 = 1 << 1; /* FIFO Service Request Interrupt Enable */

pub const MICR: u32 = 0x0108; /* Modem In Control Register */
pub const MICR_FEIE: u32 = 1 << 3; /* FIFO Error */
pub const MICR_FSRIE: u32 = 1 << 1; /* FIFO Service Request Interrupt Enable */

pub const MOSR: u32 = 0x0110; /* Modem Out Status Register */
pub const MOSR_FIFOE: u32 = 1 << 4; /* FIFO error */
pub const MOSR_FSR: u32 = 1 << 2; /* FIFO Service Request */

pub const MISR: u32 = 0x0118; /* Modem In Status Register */
pub const MISR_FIFOE: u32 = 1 << 4; /* FIFO error */
pub const MISR_EOC: u32 = 1 << 3; /* DMA End-of-Chain (exclusive clear) */
pub const MISR_FSR: u32 = 1 << 2; /* FIFO Service Request */

pub const MODR: u32 = 0x0140; /* Modem FIFO Data Register */

pub const PAC_REG_BASE: u32 = 0x0200; /* Primary Audio Codec */
pub const SAC_REG_BASE: u32 = 0x0300; /* Secondary Audio Codec */
pub const PMC_REG_BASE: u32 = 0x0400; /* Primary Modem Codec */
pub const SMC_REG_BASE: u32 = 0x0500; /* Secondary Modem Codec */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
