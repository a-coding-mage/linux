/* Translated from include/asm-ppc/mpc52xx_psc.h. */

/* CONFIG_PPC_MPC512x selects 12; otherwise the maximum is 6. */
pub const MPC52XX_PSC_MAXNUM: u32 = 6;

pub const MPC52XX_PSC_SR_UNEX_RX: u16 = 0x0001;
pub const MPC52XX_PSC_SR_DATA_VAL: u16 = 0x0002;
pub const MPC52XX_PSC_SR_DATA_OVR: u16 = 0x0004;
pub const MPC52XX_PSC_SR_CMDSEND: u16 = 0x0008;
pub const MPC52XX_PSC_SR_CDE: u16 = 0x0080;
pub const MPC52XX_PSC_SR_RXRDY: u16 = 0x0100;
pub const MPC52XX_PSC_SR_RXFULL: u16 = 0x0200;
pub const MPC52XX_PSC_SR_TXRDY: u16 = 0x0400;
pub const MPC52XX_PSC_SR_TXEMP: u16 = 0x0800;
pub const MPC52XX_PSC_SR_OE: u16 = 0x1000;
pub const MPC52XX_PSC_SR_PE: u16 = 0x2000;
pub const MPC52XX_PSC_SR_FE: u16 = 0x4000;
pub const MPC52XX_PSC_SR_RB: u16 = 0x8000;

pub const MPC52XX_PSC_RX_ENABLE: u16 = 0x0001;
pub const MPC52XX_PSC_RX_DISABLE: u16 = 0x0002;
pub const MPC52XX_PSC_TX_ENABLE: u16 = 0x0004;
pub const MPC52XX_PSC_TX_DISABLE: u16 = 0x0008;
pub const MPC52XX_PSC_SEL_MODE_REG_1: u16 = 0x0010;
pub const MPC52XX_PSC_RST_RX: u16 = 0x0020;
pub const MPC52XX_PSC_RST_TX: u16 = 0x0030;
pub const MPC52XX_PSC_RST_ERR_STAT: u16 = 0x0040;
pub const MPC52XX_PSC_RST_BRK_CHG_INT: u16 = 0x0050;
pub const MPC52XX_PSC_START_BRK: u16 = 0x0060;
pub const MPC52XX_PSC_STOP_BRK: u16 = 0x0070;

pub const MPC52XX_PSC_RXTX_FIFO_ERR: u16 = 0x0040;
pub const MPC52XX_PSC_RXTX_FIFO_UF: u16 = 0x0020;
pub const MPC52XX_PSC_RXTX_FIFO_OF: u16 = 0x0010;
pub const MPC52XX_PSC_RXTX_FIFO_FR: u16 = 0x0008;
pub const MPC52XX_PSC_RXTX_FIFO_FULL: u16 = 0x0004;
pub const MPC52XX_PSC_RXTX_FIFO_ALARM: u16 = 0x0002;
pub const MPC52XX_PSC_RXTX_FIFO_EMPTY: u16 = 0x0001;

pub const MPC52XX_PSC_IMR_UNEX_RX_SLOT: u16 = 0x0001;
pub const MPC52XX_PSC_IMR_DATA_VALID: u16 = 0x0002;
pub const MPC52XX_PSC_IMR_DATA_OVR: u16 = 0x0004;
pub const MPC52XX_PSC_IMR_CMD_SEND: u16 = 0x0008;
pub const MPC52XX_PSC_IMR_ERROR: u16 = 0x0040;
pub const MPC52XX_PSC_IMR_DEOF: u16 = 0x0080;
pub const MPC52XX_PSC_IMR_TXRDY: u16 = 0x0100;
pub const MPC52XX_PSC_IMR_RXRDY: u16 = 0x0200;
pub const MPC52XX_PSC_IMR_DB: u16 = 0x0400;
pub const MPC52XX_PSC_IMR_TXEMP: u16 = 0x0800;
pub const MPC52XX_PSC_IMR_ORERR: u16 = 0x1000;
pub const MPC52XX_PSC_IMR_IPC: u16 = 0x8000;

pub const MPC52XX_PSC_CTS: u8 = 0x01; pub const MPC52XX_PSC_DCD: u8 = 0x02;
pub const MPC52XX_PSC_D_CTS: u8 = 0x10; pub const MPC52XX_PSC_D_DCD: u8 = 0x20;
pub const MPC52XX_PSC_IEC_CTS: u8 = 0x01; pub const MPC52XX_PSC_IEC_DCD: u8 = 0x02;
pub const MPC52XX_PSC_OP_RTS: u8 = 0x01; pub const MPC52XX_PSC_OP_RES: u8 = 0x02;

pub const MPC52XX_PSC_MODE_5_BITS: u8 = 0x00; pub const MPC52XX_PSC_MODE_6_BITS: u8 = 0x01;
pub const MPC52XX_PSC_MODE_7_BITS: u8 = 0x02; pub const MPC52XX_PSC_MODE_8_BITS: u8 = 0x03;
pub const MPC52XX_PSC_MODE_BITS_MASK: u8 = 0x03; pub const MPC52XX_PSC_MODE_PAREVEN: u8 = 0x00;
pub const MPC52XX_PSC_MODE_PARODD: u8 = 0x04; pub const MPC52XX_PSC_MODE_PARFORCE: u8 = 0x08;
pub const MPC52XX_PSC_MODE_PARNONE: u8 = 0x10; pub const MPC52XX_PSC_MODE_ERR: u8 = 0x20;
pub const MPC52XX_PSC_MODE_FFULL: u8 = 0x40; pub const MPC52XX_PSC_MODE_RXRTS: u8 = 0x80;
pub const MPC52XX_PSC_MODE_ONE_STOP_5_BITS: u8 = 0x00; pub const MPC52XX_PSC_MODE_ONE_STOP: u8 = 0x07;
pub const MPC52XX_PSC_MODE_TWO_STOP: u8 = 0x0f; pub const MPC52XX_PSC_MODE_TXCTS: u8 = 0x10;
pub const MPC52XX_PSC_RFNUM_MASK: u16 = 0x01ff;

pub const MPC52XX_PSC_SICR_DTS1: u32 = 1 << 29; pub const MPC52XX_PSC_SICR_SHDR: u32 = 1 << 28;
pub const MPC52XX_PSC_SICR_SIM_MASK: u32 = 0xf << 24; pub const MPC52XX_PSC_SICR_SIM_UART: u32 = 0x0 << 24;
pub const MPC52XX_PSC_SICR_SIM_UART_DCD: u32 = 0x8 << 24; pub const MPC52XX_PSC_SICR_SIM_CODEC_8: u32 = 0x1 << 24;
pub const MPC52XX_PSC_SICR_SIM_CODEC_16: u32 = 0x2 << 24; pub const MPC52XX_PSC_SICR_SIM_AC97: u32 = 0x3 << 24;
pub const MPC52XX_PSC_SICR_SIM_SIR: u32 = 0x8 << 24; pub const MPC52XX_PSC_SICR_SIM_SIR_DCD: u32 = 0xc << 24;
pub const MPC52XX_PSC_SICR_SIM_MIR: u32 = 0x5 << 24; pub const MPC52XX_PSC_SICR_SIM_FIR: u32 = 0x6 << 24;
pub const MPC52XX_PSC_SICR_SIM_CODEC_24: u32 = 0x7 << 24; pub const MPC52XX_PSC_SICR_SIM_CODEC_32: u32 = 0xf << 24;
pub const MPC52XX_PSC_SICR_ACRB: u32 = 0x8 << 24; pub const MPC52XX_PSC_SICR_AWR: u32 = 1 << 30;
pub const MPC52XX_PSC_SICR_GENCLK: u32 = 1 << 23; pub const MPC52XX_PSC_SICR_I2S: u32 = 1 << 22;
pub const MPC52XX_PSC_SICR_CLKPOL: u32 = 1 << 21; pub const MPC52XX_PSC_SICR_SYNCPOL: u32 = 1 << 20;
pub const MPC52XX_PSC_SICR_CELLSLAVE: u32 = 1 << 19; pub const MPC52XX_PSC_SICR_CELL2XCLK: u32 = 1 << 18;
pub const MPC52XX_PSC_SICR_ESAI: u32 = 1 << 17; pub const MPC52XX_PSC_SICR_ENAC97: u32 = 1 << 16;
pub const MPC52XX_PSC_SICR_SPI: u32 = 1 << 15; pub const MPC52XX_PSC_SICR_MSTR: u32 = 1 << 14;
pub const MPC52XX_PSC_SICR_CPOL: u32 = 1 << 13; pub const MPC52XX_PSC_SICR_CPHA: u32 = 1 << 12;
pub const MPC52XX_PSC_SICR_USEEOF: u32 = 1 << 11; pub const MPC52XX_PSC_SICR_DISABLEEOF: u32 = 1 << 10;

#[repr(C)] pub union Mpc52xxPscMode { pub mode: u8, pub mr2: u8 }
#[repr(C)] pub union Mpc52xxPscSrCsr { pub status: u16, pub clock_select: u16 }
#[repr(C)] pub union Mpc52xxPscBuffer { pub buffer_8: u8, pub buffer_16: u16, pub buffer_32: u32 }
#[repr(C)] pub union Mpc52xxPscIpcrAcr { pub ipcr: u8, pub acr: u8 }
#[repr(C)] pub union Mpc52xxPscIsrImr { pub isr: u16, pub imr: u16 }

#[repr(C)] pub struct Mpc52xxPsc {
    pub mode: Mpc52xxPscMode, pub reserved0: [u8;3], pub sr_csr: Mpc52xxPscSrCsr, pub reserved1: u16,
    pub command: u8, pub reserved2: [u8;3], pub buffer: Mpc52xxPscBuffer, pub ipcr_acr: Mpc52xxPscIpcrAcr,
    pub reserved3: [u8;3], pub isr_imr: Mpc52xxPscIsrImr, pub reserved4: u16, pub ctur: u8, pub reserved5: [u8;3],
    pub ctlr: u8, pub reserved6: [u8;3], pub ccr: u32, pub ac97_slots: u32, pub ac97_cmd: u32, pub ac97_data: u32,
    pub ivr: u8, pub reserved8: [u8;3], pub ip: u8, pub reserved9: [u8;3], pub op1: u8, pub reserved10: [u8;3],
    pub op0: u8, pub reserved11: [u8;3], pub sicr: u32, pub ircr1: u8, pub reserved13: [u8;3], pub ircr2: u8,
    pub reserved14: [u8;3], pub irsdr: u8, pub reserved15: [u8;3], pub irmdr: u8, pub reserved16: [u8;3],
    pub irfdr: u8, pub reserved17: [u8;3],
}

#[repr(C)] pub struct Mpc52xxPscFifo {
    pub rfnum:u16, pub reserved18:u16, pub tfnum:u16, pub reserved19:u16, pub rfdata:u32, pub rfstat:u16, pub reserved20:u16,
    pub rfcntl:u8, pub reserved21:[u8;5], pub rfalarm:u16, pub reserved22:u16, pub rfrptr:u16, pub reserved23:u16,
    pub rfwptr:u16, pub reserved24:u16, pub rflrfptr:u16, pub reserved25:u16, pub rflwfptr:u16, pub tfdata:u32,
    pub tfstat:u16, pub reserved26:u16, pub tfcntl:u8, pub reserved27:[u8;5], pub tfalarm:u16, pub reserved28:u16,
    pub tfrptr:u16, pub reserved29:u16, pub tfwptr:u16, pub reserved30:u16, pub tflrfptr:u16, pub reserved31:u16, pub tflwfptr:u16,
}

pub const MPC512X_PSC_FIFO_EOF:u32=0x100; pub const MPC512X_PSC_FIFO_RESET_SLICE:u32=0x80;
pub const MPC512X_PSC_FIFO_ENABLE_SLICE:u32=0x01; pub const MPC512X_PSC_FIFO_ENABLE_DMA:u32=0x04;
pub const MPC512X_PSC_FIFO_EMPTY:u32=1; pub const MPC512X_PSC_FIFO_FULL:u32=2; pub const MPC512X_PSC_FIFO_ALARM:u32=4; pub const MPC512X_PSC_FIFO_URERR:u32=8;

#[repr(C)] pub union Mpc512xPscData { pub txdata_8:u8, pub txdata_16:u16, pub txdata_32:u32 }
#[repr(C)] pub union Mpc512xPscRxData { pub rxdata_8:u8, pub rxdata_16:u16, pub rxdata_32:u32 }
#[repr(C)] pub struct Mpc512xPscFifo {
    pub reserved1:[u32;10], pub txcmd:u32, pub txalarm:u32, pub txsr:u32, pub txisr:u32, pub tximr:u32, pub txcnt:u32, pub txptr:u32, pub txsz:u32,
    pub reserved2:[u32;7], pub txdata:Mpc512xPscData, pub rxcmd:u32, pub rxalarm:u32, pub rxsr:u32, pub rxisr:u32, pub rximr:u32, pub rxcnt:u32, pub rxptr:u32, pub rxsz:u32,
    pub reserved3:[u32;7], pub rxdata:Mpc512xPscRxData,
}

#[repr(C)] pub struct Mpc5125PscSrCsr { pub status:u16, pub reserved2:[u8;2], pub clock_select:u8, pub reserved3:[u8;3] }
#[repr(C)] pub struct Mpc5125PscIpcrAcr { pub ipcr:u8, pub reserved5:[u8;3], pub acr:u8, pub reserved6:[u8;3] }
#[repr(C)] pub struct Mpc5125PscIsrImr { pub isr:u16, pub reserved7:[u8;2], pub imr:u16, pub reserved8:[u8;2] }
#[repr(C)] pub struct Mpc5125Psc {
    pub mr1:u8, pub reserved0:[u8;3], pub mr2:u8, pub reserved1:[u8;3], pub sr_csr:Mpc5125PscSrCsr, pub command:u8, pub reserved4:[u8;3],
    pub buffer:Mpc52xxPscBuffer, pub ipcr_acr:Mpc5125PscIpcrAcr, pub isr_imr:Mpc5125PscIsrImr, pub ctur:u8, pub reserved9:[u8;3], pub ctlr:u8, pub reserved10:[u8;3],
    pub ccr:u32, pub ac97slots:u32, pub ac97cmd:u32, pub ac97data:u32, pub reserved11:[u8;4], pub ip:u8, pub reserved12:[u8;3], pub op1:u8, pub reserved13:[u8;3], pub op0:u8, pub reserved14:[u8;3], pub sicr:u32, pub reserved15:[u8;4],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
