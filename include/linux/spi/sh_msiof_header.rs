/* SPDX-License-Identifier: GPL-2.0 */

/* Linux bitfield helpers translated as direct masks. */
pub const SITMDR1: u32 = 0x00; /* Transmit Mode Register 1 */
pub const SITMDR2: u32 = 0x04; /* Transmit Mode Register 2 */
pub const SITMDR3: u32 = 0x08; /* Transmit Mode Register 3 */
pub const SIRMDR1: u32 = 0x10; /* Receive Mode Register 1 */
pub const SIRMDR2: u32 = 0x14; /* Receive Mode Register 2 */
pub const SIRMDR3: u32 = 0x18; /* Receive Mode Register 3 */
pub const SITSCR: u32 = 0x20; /* Transmit Clock Select Register */
pub const SIRSCR: u32 = 0x22; /* Receive Clock Select Register (SH, A1, APE6) */
pub const SICTR: u32 = 0x28; /* Control Register */
pub const SIFCTR: u32 = 0x30; /* FIFO Control Register */
pub const SISTR: u32 = 0x40; /* Status Register */
pub const SIIER: u32 = 0x44; /* Interrupt Enable Register */
pub const SITDR1: u32 = 0x48; /* Transmit Control Data Register 1 (SH, A1) */
pub const SITDR2: u32 = 0x4c; /* Transmit Control Data Register 2 (SH, A1) */
pub const SITFDR: u32 = 0x50; /* Transmit FIFO Data Register */
pub const SIRDR1: u32 = 0x58; /* Receive Control Data Register 1 (SH, A1) */
pub const SIRDR2: u32 = 0x5c; /* Receive Control Data Register 2 (SH, A1) */
pub const SIRFDR: u32 = 0x60; /* Receive FIFO Data Register */

pub const SIMDR1_TRMD: u32 = 1 << 31;
pub const SIMDR1_SYNCMD: u32 = 0x3 << 28;
pub const SIMDR1_SYNCMD_PULSE: u32 = 0;
pub const SIMDR1_SYNCMD_SPI: u32 = 2;
pub const SIMDR1_SYNCMD_LR: u32 = 3;
pub const SIMDR1_SYNCAC: u32 = 1 << 25;
pub const SIMDR1_BITLSB: u32 = 1 << 24;
pub const SIMDR1_DTDL: u32 = 0x7 << 20;
pub const SIMDR1_SYNCDL: u32 = 0x7 << 16;
pub const SIMDR1_FLD: u32 = 0x3 << 2;
pub const SIMDR1_XXSTP: u32 = 1;
pub const SITMDR1_PCON: u32 = 1 << 30;
pub const SITMDR1_SYNCCH: u32 = 0x3 << 26;

pub const SIMDR2_GRP: u32 = 0x3 << 30;
pub const SIMDR2_BITLEN1: u32 = 0x1f << 24;
pub const SIMDR2_WDLEN1: u32 = 0xff << 16;
pub const SIMDR2_GRPMASK: u32 = 0xf;
pub const SIMDR3_BITLEN2: u32 = 0x1f << 24;
pub const SIMDR3_WDLEN2: u32 = 0xff << 16;

pub const SISCR_BRPS: u32 = 0x1f << 8;
pub const SISCR_BRDV: u32 = 0x7;

pub const SICTR_TSCKIZ: u32 = 0x3 << 30;
pub const SICTR_TSCKIZ_SCK: u32 = 1 << 31;
pub const SICTR_TSCKIZ_POL: u32 = 1 << 30;
pub const SICTR_RSCKIZ: u32 = 0x3 << 28;
pub const SICTR_RSCKIZ_SCK: u32 = 1 << 29;
pub const SICTR_RSCKIZ_POL: u32 = 1 << 28;
pub const SICTR_TEDG: u32 = 1 << 27;
pub const SICTR_REDG: u32 = 1 << 26;
pub const SICTR_TXDIZ: u32 = 0x3 << 22;
pub const SICTR_TXDIZ_LOW: u32 = 0;
pub const SICTR_TXDIZ_HIGH: u32 = 1;
pub const SICTR_TXDIZ_HIZ: u32 = 2;
pub const SICTR_TSCKE: u32 = 1 << 15;
pub const SICTR_TFSE: u32 = 1 << 14;
pub const SICTR_TXE: u32 = 1 << 9;
pub const SICTR_RXE: u32 = 1 << 8;
pub const SICTR_TXRST: u32 = 1 << 1;
pub const SICTR_RXRST: u32 = 1;

pub const SIFCTR_TFWM: u32 = 0x7 << 29;
pub const SIFCTR_TFWM_64: u32 = 0;
pub const SIFCTR_TFWM_32: u32 = 1;
pub const SIFCTR_TFWM_24: u32 = 2;
pub const SIFCTR_TFWM_16: u32 = 3;
pub const SIFCTR_TFWM_12: u32 = 4;
pub const SIFCTR_TFWM_8: u32 = 5;
pub const SIFCTR_TFWM_4: u32 = 6;
pub const SIFCTR_TFWM_1: u32 = 7;
pub const SIFCTR_TFUA: u32 = 0x1ff << 20;
pub const SIFCTR_RFWM: u32 = 0x7 << 13;
pub const SIFCTR_RFWM_1: u32 = 0;
pub const SIFCTR_RFWM_4: u32 = 1;
pub const SIFCTR_RFWM_8: u32 = 2;
pub const SIFCTR_RFWM_16: u32 = 3;
pub const SIFCTR_RFWM_32: u32 = 4;
pub const SIFCTR_RFWM_64: u32 = 5;
pub const SIFCTR_RFWM_128: u32 = 6;
pub const SIFCTR_RFWM_256: u32 = 7;
pub const SIFCTR_RFUA: u32 = 0x1ff << 4;

pub const SISTR_TFEMP: u32 = 1 << 29;
pub const SISTR_TDREQ: u32 = 1 << 28;
pub const SISTR_TEOF: u32 = 1 << 23;
pub const SISTR_TFSERR: u32 = 1 << 21;
pub const SISTR_TFOVF: u32 = 1 << 20;
pub const SISTR_TFUDF: u32 = 1 << 19;
pub const SISTR_RFFUL: u32 = 1 << 13;
pub const SISTR_RDREQ: u32 = 1 << 12;
pub const SISTR_REOF: u32 = 1 << 7;
pub const SISTR_RFSERR: u32 = 1 << 5;
pub const SISTR_RFUDF: u32 = 1 << 4;
pub const SISTR_RFOVF: u32 = 1 << 3;

pub const SIIER_TDMAE: u32 = 1 << 31;
pub const SIIER_TFEMPE: u32 = 1 << 29;
pub const SIIER_TDREQE: u32 = 1 << 28;
pub const SIIER_TEOFE: u32 = 1 << 23;
pub const SIIER_TFSERRE: u32 = 1 << 21;
pub const SIIER_TFOVFE: u32 = 1 << 20;
pub const SIIER_TFUDFE: u32 = 1 << 19;
pub const SIIER_RDMAE: u32 = 1 << 15;
pub const SIIER_RFFULE: u32 = 1 << 13;
pub const SIIER_RDREQE: u32 = 1 << 12;
pub const SIIER_REOFE: u32 = 1 << 7;
pub const SIIER_RFSERRE: u32 = 1 << 5;
pub const SIIER_RFUDFE: u32 = 1 << 4;
pub const SIIER_RFOVFE: u32 = 1 << 3;

#[repr(i32)]
pub enum MsiofSpi {
    MSIOF_SPI_HOST,
    MSIOF_SPI_TARGET,
}

#[repr(C)]
pub struct ShMsiofSpiInfo {
    pub tx_fifo_override: i32,
    pub rx_fifo_override: i32,
    pub num_chipselect: u16,
    pub mode: i32,
    pub dma_tx_id: u32,
    pub dma_rx_id: u32,
    pub dtdl: u32,
    pub syncdl: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
