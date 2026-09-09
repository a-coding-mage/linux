/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of comedi/drivers/s626.h. */

pub const S626_DMABUF_SIZE: u32 = 4096;
pub const S626_ADC_CHANNELS: u32 = 16;
pub const S626_DAC_CHANNELS: u32 = 4;
pub const S626_ENCODER_CHANNELS: u32 = 6;
pub const S626_DIO_CHANNELS: u32 = 48;
pub const S626_DIO_BANKS: u32 = 3;
pub const S626_DIO_EXTCHANS: u32 = 40;
pub const S626_NUM_TRIMDACS: u32 = 12;
pub const S626_INTEL: u32 = 1;
pub const S626_MOTOROLA: u32 = 2;
pub const S626_PLATFORM: u32 = S626_INTEL;
pub const S626_RANGE_5V: u32 = 0x10;
pub const S626_RANGE_10V: u32 = 0x00;
pub const S626_EOPL: u32 = 0x80;
pub const S626_GSEL_BIPOLAR5V: u32 = 0x00f0;
pub const S626_GSEL_BIPOLAR10V: u32 = 0x00a0;
pub const S626_ERR_ILLEGAL_PARM: u32 = 0x0001_0000;
pub const S626_ERR_I2C: u32 = 0x0002_0000;
pub const S626_ERR_COUNTERSETUP: u32 = 0x0020_0000;
pub const S626_ERR_DEBI_TIMEOUT: u32 = 0x0040_0000;
pub const S626_ADC_DMABUF_DWORDS: u32 = 40;
pub const S626_DAC_WDMABUF_DWORDS: u32 = 1;
pub const S626_DAC_WDMABUF_OS: u32 = S626_ADC_DMABUF_DWORDS;
pub const S626_IRQ_GPIO3: u32 = 0x40;
pub const S626_IRQ_RPS1: u32 = 0x1000_0000;
pub const S626_ISR_AFOU: u32 = 0x800;
pub const S626_IRQ_COINT1A: u32 = 0x400;
pub const S626_IRQ_COINT1B: u32 = 0x800;
pub const S626_IRQ_COINT2A: u32 = 0x1000;
pub const S626_IRQ_COINT2B: u32 = 0x2000;
pub const S626_IRQ_COINT3A: u32 = 0x4000;
pub const S626_IRQ_COINT3B: u32 = 0x8000;
pub const S626_RPS_CLRSIGNAL: u32 = 0;
pub const S626_RPS_SETSIGNAL: u32 = 0x1000_0000;
pub const S626_RPS_NOP: u32 = 0;
pub const S626_RPS_PAUSE: u32 = 0x2000_0000;
pub const S626_RPS_UPLOAD: u32 = 0x4000_0000;
pub const S626_RPS_JUMP: u32 = 0x8000_0000;
pub const S626_RPS_LDREG: u32 = 0x9000_0100;
pub const S626_RPS_STREG: u32 = 0xa000_0100;
pub const S626_RPS_STOP: u32 = 0x5000_0000;
pub const S626_RPS_IRQ: u32 = 0x6000_0000;
pub const S626_RPS_LOGICAL_OR: u32 = 0x0800_0000;
pub const S626_RPS_INVERT: u32 = 0x0400_0000;
pub const S626_RPS_DEBI: u32 = 2;
pub const S626_RPS_SIG0: u32 = 0x0020_0000;
pub const S626_RPS_SIG1: u32 = 0x0040_0000;
pub const S626_RPS_SIG2: u32 = 0x0080_0000;
pub const S626_RPS_GPIO2: u32 = 0x0008_0000;
pub const S626_RPS_GPIO3: u32 = 0x0010_0000;
pub const S626_RPS_SIGADC: u32 = S626_RPS_SIG0;
pub const S626_RPS_SIGDAC: u32 = S626_RPS_SIG1;
pub const S626_RPSCLK_SCALAR: u32 = 8;
pub const S626_RPSCLK_PER_US: u32 = 33 / S626_RPSCLK_SCALAR;
pub const S626_SBA_RPS_A0: u32 = 0x27;
pub const S626_GPIO_BASE: u32 = 0x1000_4000;
pub const S626_GPIO1_LO: u32 = 0;
pub const S626_GPIO1_HI: u32 = 0x1000;
pub const S626_PSR_DEBI_E: u32 = 0x40000;
pub const S626_PSR_DEBI_S: u32 = 0x80000;
pub const S626_PSR_A2_IN: u32 = 0x8000;
pub const S626_PSR_AFOU: u32 = 0x800;
pub const S626_PSR_GPIO2: u32 = 0x20;
pub const S626_PSR_EC0S: u32 = 1;
pub const S626_SSR_AF2_OUT: u32 = 0x200;
pub const S626_MC1_SOFT_RESET: u32 = 0x8000_0000;
pub const S626_MC1_SHUTDOWN: u32 = 0x3fff_0000;
pub const S626_MC1_ERPS1: u32 = 0x2000;
pub const S626_MC1_ERPS0: u32 = 0x1000;
pub const S626_MC1_DEBI: u32 = 0x800;
pub const S626_MC1_AUDIO: u32 = 0x200;
pub const S626_MC1_I2C: u32 = 0x100;
pub const S626_MC1_A2OUT: u32 = 8;
pub const S626_MC1_A2IN: u32 = 4;
pub const S626_MC1_A1IN: u32 = 1;
pub const S626_MC2_UPLD_DEBI: u32 = 2;
pub const S626_MC2_UPLD_IIC: u32 = 1;
pub const S626_MC2_RPSSIG2: u32 = 0x2000;
pub const S626_MC2_RPSSIG1: u32 = 0x1000;
pub const S626_MC2_RPSSIG0: u32 = 0x800;
pub const S626_MC2_ADC_RPS: u32 = S626_MC2_RPSSIG0;
pub const S626_MC2_DAC_RPS: u32 = S626_MC2_RPSSIG1;

/* PCI bus and local-bus register offsets. */
pub const S626_P_PCI_BT_A: u32 = 0x004c; pub const S626_P_DEBICFG: u32 = 0x007c;
pub const S626_P_DEBICMD: u32 = 0x0080; pub const S626_P_DEBIPAGE: u32 = 0x0084;
pub const S626_P_DEBIAD: u32 = 0x0088; pub const S626_P_I2CCTRL: u32 = 0x008c;
pub const S626_P_I2CSTAT: u32 = 0x0090; pub const S626_P_BASEA2_IN: u32 = 0x00ac;
pub const S626_P_PROTA2_IN: u32 = 0x00b0; pub const S626_P_PAGEA2_IN: u32 = 0x00b4;
pub const S626_P_BASEA2_OUT: u32 = 0x00b8; pub const S626_P_PROTA2_OUT: u32 = 0x00bc;
pub const S626_P_PAGEA2_OUT: u32 = 0x00c0; pub const S626_P_RPSPAGE0: u32 = 0x00c4;
pub const S626_P_RPSPAGE1: u32 = 0x00c8; pub const S626_P_RPS0_TOUT: u32 = 0x00d4;
pub const S626_P_RPS1_TOUT: u32 = 0x00d8; pub const S626_P_IER: u32 = 0x00dc;
pub const S626_P_GPIO: u32 = 0x00e0; pub const S626_P_EC1SSR: u32 = 0x00e4;
pub const S626_P_ECT1R: u32 = 0x00ec; pub const S626_P_ACON1: u32 = 0x00f4;
pub const S626_P_ACON2: u32 = 0x00f8; pub const S626_P_MC1: u32 = 0x00fc;
pub const S626_P_MC2: u32 = 0x0100; pub const S626_P_RPSADDR0: u32 = 0x0104;
pub const S626_P_RPSADDR1: u32 = 0x0108; pub const S626_P_ISR: u32 = 0x010c;
pub const S626_P_PSR: u32 = 0x0110; pub const S626_P_SSR: u32 = 0x0114;
pub const S626_P_EC1R: u32 = 0x0118; pub const S626_P_ADP4: u32 = 0x0138;
pub const S626_P_FB_BUFFER1: u32 = 0x0144; pub const S626_P_FB_BUFFER2: u32 = 0x0148;
pub const S626_P_TSL1: u32 = 0x0180; pub const S626_P_TSL2: u32 = 0x01c0;
pub const S626_LP_DACPOL: u32 = 0x82; pub const S626_LP_GSEL: u32 = 0x84; pub const S626_LP_ISEL: u32 = 0x86;

macro_rules! s626_local_reg { ($name:ident, $base:expr, $x:expr) => { $base + ($x) * 0x10 }; }
pub const S626_LP_MISC1: u32 = 0x88; pub const S626_LP_WRMISC2: u32 = 0x90; pub const S626_LP_RDMISC2: u32 = 0x82;
pub const S626_MISC1_WENABLE: u32 = 0x8000; pub const S626_MISC1_WDISABLE: u32 = 0;
pub const S626_MISC1_EDCAP: u32 = 0x1000; pub const S626_MISC1_NOEDCAP: u32 = 0;
pub const S626_RDMISC1_WDTIMEOUT: u32 = 0x4000; pub const S626_WRMISC2_WDCLEAR: u32 = 0x8000;
pub const S626_WRMISC2_CHARGE_ENABLE: u32 = 0x4000; pub const S626_MISC2_BATT_ENABLE: u32 = 8;
pub const S626_MISC2_WDENABLE: u32 = 4; pub const S626_MISC2_WDPERIOD_MASK: u32 = 3;

pub const S626_A2_RUN: u32 = 0x4000_0000; pub const S626_A1_RUN: u32 = 0x2000_0000;
pub const S626_A1_SWAP: u32 = 0x0020_0000; pub const S626_A2_SWAP: u32 = 0x0010_0000;
pub const S626_WS_MODES: u32 = 0x0001_9999; pub const S626_ACON1_BASE: u32 = S626_WS_MODES | S626_A1_RUN;
pub const S626_ACON1_ADCSTART: u32 = S626_ACON1_BASE; pub const S626_ACON1_DACSTART: u32 = S626_ACON1_BASE | S626_A2_RUN; pub const S626_ACON1_DACSTOP: u32 = S626_ACON1_BASE;
pub const S626_A1_CLKSRC_BCLK1: u32 = 0; pub const S626_A2_CLKSRC_X1: u32 = 0x0080_0000; pub const S626_A2_CLKSRC_X2: u32 = 0x00c0_0000; pub const S626_A2_CLKSRC_X4: u32 = 0x0140_0000;
pub const S626_INVERT_BCLK2: u32 = 0x0010_0000; pub const S626_BCLK2_OE: u32 = 0x0004_0000; pub const S626_ACON2_XORMASK: u32 = 0x000c_0000;
pub const S626_ACON2_INIT: u32 = S626_ACON2_XORMASK ^ (S626_A1_CLKSRC_BCLK1 | S626_A2_CLKSRC_X2 | S626_INVERT_BCLK2 | S626_BCLK2_OE);

pub const S626_WS1: u32 = 0x4000_0000; pub const S626_WS2: u32 = 0x2000_0000; pub const S626_WS3: u32 = 0x1000_0000; pub const S626_WS4: u32 = 0x0800_0000;
pub const S626_RSD1: u32 = 0x0100_0000; pub const S626_SDW_A1: u32 = 0x0080_0000; pub const S626_SIB_A1: u32 = 0x0040_0000; pub const S626_SF_A1: u32 = 0x0020_0000;
pub const S626_XFIFO_0: u32 = 0; pub const S626_XFIFO_1: u32 = 0x10; pub const S626_XFIFO_2: u32 = 0x20; pub const S626_XFIFO_3: u32 = 0x30;
pub const S626_XFB0: u32 = 0x40; pub const S626_XFB1: u32 = 0x50; pub const S626_XFB2: u32 = 0x60; pub const S626_XFB3: u32 = 0x70;
pub const S626_SIB_A2: u32 = 0x200; pub const S626_SF_A2: u32 = 0x100; pub const S626_LF_A2: u32 = 0x80; pub const S626_XSD2: u32 = 8; pub const S626_RSD3: u32 = 0x1800; pub const S626_RSD2: u32 = 0x1000; pub const S626_LOW_A2: u32 = 2; pub const S626_EOS: u32 = 1;
pub const S626_I2C_CLKSEL: u32 = 0x400; pub const S626_I2C_BITRATE: f64 = 68.75; pub const S626_I2C_WRTIME: f64 = 15.0; pub const S626_I2C_RETRIES: f64 = S626_I2C_WRTIME * S626_I2C_BITRATE / 9.0;
pub const S626_I2C_ERR: u32 = 2; pub const S626_I2C_BUSY: u32 = 1; pub const S626_I2C_ABORT: u32 = 0x80; pub const S626_I2C_ATTRSTART: u32 = 3; pub const S626_I2C_ATTRCONT: u32 = 2; pub const S626_I2C_ATTRSTOP: u32 = 1; pub const S626_I2C_ATTRNOP: u32 = 0;
macro_rules! S626_I2C_B2 { ($a:expr,$v:expr) => { (($a) << 6) | (($v) << 24) }; } macro_rules! S626_I2C_B1 { ($a:expr,$v:expr) => { (($a) << 4) | (($v) << 16) }; } macro_rules! S626_I2C_B0 { ($a:expr,$v:expr) => { (($a) << 2) | (($v) << 8) }; }
pub const S626_DEBI_CMD_SIZE16: u32 = 2 << 17; pub const S626_DEBI_CMD_READ: u32 = 0x10000; pub const S626_DEBI_CMD_WRITE: u32 = 0; pub const S626_DEBI_CMD_RDWORD: u32 = S626_DEBI_CMD_READ | S626_DEBI_CMD_SIZE16; pub const S626_DEBI_CMD_WRWORD: u32 = S626_DEBI_CMD_SIZE16;
pub const S626_DEBI_CFG_XIRQ_EN: u32 = 0x8000_0000; pub const S626_DEBI_CFG_XRESUME: u32 = 0x4000_0000; pub const S626_DEBI_CFG_TOQ: u32 = 0x03c0_0000; pub const S626_DEBI_CFG_FAST: u32 = 0x1000_0000; pub const S626_DEBI_CFG_TOUT_BIT: u32 = 22; pub const S626_DEBI_CFG_SWAP_NONE: u32 = 0; pub const S626_DEBI_CFG_SWAP_2: u32 = 0x0010_0000; pub const S626_DEBI_CFG_SWAP_4: u32 = 0x0020_0000; pub const S626_DEBI_CFG_SLAVE16: u32 = 0x80000; pub const S626_DEBI_CFG_INC: u32 = 0x40000; pub const S626_DEBI_CFG_INTEL: u32 = 0x20000; pub const S626_DEBI_CFG_TIMEROFF: u32 = 0x10000; pub const S626_DEBI_TOUT: u32 = 7; pub const S626_DEBI_SWAP: u32 = S626_DEBI_CFG_SWAP_NONE; pub const S626_DEBI_PAGE_DISABLE: u32 = 0;

pub const S626_LOADSRC_INDX: u32=0; pub const S626_LOADSRC_OVER: u32=1; pub const S626_LOADSRCB_OVERA: u32=2; pub const S626_LOADSRC_NONE: u32=3;
pub const S626_INTSRC_NONE: u32=0; pub const S626_INTSRC_OVER: u32=1; pub const S626_INTSRC_INDX: u32=2; pub const S626_INTSRC_BOTH: u32=3;
pub const S626_LATCHSRC_AB_READ: u32=0; pub const S626_LATCHSRC_A_INDXA: u32=1; pub const S626_LATCHSRC_B_INDXB: u32=2; pub const S626_LATCHSRC_B_OVERA: u32=3;
pub const S626_INDXSRC_ENCODER: u32=0; pub const S626_INDXSRC_DIGIN: u32=1; pub const S626_INDXSRC_SOFT: u32=2; pub const S626_INDXSRC_DISABLED: u32=3; pub const S626_INDXPOL_POS: u32=0; pub const S626_INDXPOL_NEG: u32=1;
pub const S626_ENCMODE_COUNTER: u32=0; pub const S626_ENCMODE_TIMER: u32=2; pub const S626_ENCMODE_EXTENDER: u32=3; pub const S626_CNTSRC_ENCODER: u32=0; pub const S626_CNTSRC_DIGIN: u32=1; pub const S626_CNTSRC_SYSCLK: u32=2; pub const S626_CNTSRC_SYSCLK_DOWN: u32=3;
pub const S626_CLKPOL_POS: u32=0; pub const S626_CLKPOL_NEG: u32=1; pub const S626_CNTDIR_UP: u32=0; pub const S626_CNTDIR_DOWN: u32=1; pub const S626_CLKENAB_ALWAYS: u32=0; pub const S626_CLKENAB_INDEX: u32=1; pub const S626_CLKMULT_4X: u32=0; pub const S626_CLKMULT_2X: u32=1; pub const S626_CLKMULT_1X: u32=2; pub const S626_CLKMULT_SPECIAL: u32=3;
pub const S626_NUM_COUNTERS: u32=6; pub const S626_NUM_INTSOURCES: u32=4; pub const S626_NUM_LATCHSOURCES: u32=4; pub const S626_NUM_CLKMULTS: u32=4; pub const S626_NUM_CLKSOURCES: u32=4; pub const S626_NUM_CLKPOLS: u32=2; pub const S626_NUM_INDEXPOLS: u32=2; pub const S626_NUM_INDEXSOURCES: u32=2; pub const S626_NUM_LOADTRIGS: u32=4;

macro_rules! S626_MAKE { ($x:expr,$w:expr,$p:expr) => { (($x) & ((1u32 << ($w)) - 1)) << ($p) }; } macro_rules! S626_UNMAKE { ($v:expr,$w:expr,$p:expr) => { (($v) >> ($p)) & ((1u32 << ($w)) - 1) }; }
pub const S626_CRABIT_INDXSRC_B:u32=14; pub const S626_CRABIT_CNTSRC_B:u32=12; pub const S626_CRABIT_INDXPOL_A:u32=11; pub const S626_CRABIT_LOADSRC_A:u32=9; pub const S626_CRABIT_CLKMULT_A:u32=7; pub const S626_CRABIT_INTSRC_A:u32=5; pub const S626_CRABIT_CLKPOL_A:u32=4; pub const S626_CRABIT_INDXSRC_A:u32=2; pub const S626_CRABIT_CNTSRC_A:u32=0;
pub const S626_CRBBIT_INTRESETCMD:u32=15; pub const S626_CRBBIT_CNTDIR_B:u32=15; pub const S626_CRBBIT_INTRESET_B:u32=14; pub const S626_CRBBIT_OVERDO_A:u32=14; pub const S626_CRBBIT_INTRESET_A:u32=13; pub const S626_CRBBIT_OVERDO_B:u32=13; pub const S626_CRBBIT_CLKENAB_A:u32=12; pub const S626_CRBBIT_INTSRC_B:u32=10; pub const S626_CRBBIT_LATCHSRC:u32=8; pub const S626_CRBBIT_LOADSRC_B:u32=6; pub const S626_CRBBIT_CLEAR_B:u32=7; pub const S626_CRBBIT_CLKMULT_B:u32=3; pub const S626_CRBBIT_CLKENAB_B:u32=2; pub const S626_CRBBIT_INDXPOL_B:u32=1; pub const S626_CRBBIT_CLKPOL_B:u32=0;
pub const S626_STDBIT_INTSRC:u32=13; pub const S626_STDBIT_LATCHSRC:u32=11; pub const S626_STDBIT_LOADSRC:u32=9; pub const S626_STDBIT_INDXSRC:u32=7; pub const S626_STDBIT_INDXPOL:u32=6; pub const S626_STDBIT_ENCMODE:u32=4; pub const S626_STDBIT_CLKPOL:u32=3; pub const S626_STDBIT_CLKMULT:u32=1; pub const S626_STDBIT_CLKENAB:u32=0;
macro_rules! s626_field { ($set:ident,$get:ident,$bit:expr,$width:expr) => { macro_rules! $set { ($x:expr) => { S626_MAKE!($x,$width,$bit) }; } macro_rules! $get { ($x:expr) => { S626_UNMAKE!($x,$width,$bit) }; } }; }
s626_field!(S626_SET_CRA_INDXSRC_B,S626_GET_CRA_INDXSRC_B,14,2); s626_field!(S626_SET_CRA_CNTSRC_B,S626_GET_CRA_CNTSRC_B,12,2); s626_field!(S626_SET_CRA_INDXPOL_A,S626_GET_CRA_INDXPOL_A,11,1); s626_field!(S626_SET_CRA_LOADSRC_A,S626_GET_CRA_LOADSRC_A,9,2); s626_field!(S626_SET_CRA_CLKMULT_A,S626_GET_CRA_CLKMULT_A,7,2); s626_field!(S626_SET_CRA_INTSRC_A,S626_GET_CRA_INTSRC_A,5,2); s626_field!(S626_SET_CRA_CLKPOL_A,S626_GET_CRA_CLKPOL_A,4,1); s626_field!(S626_SET_CRA_INDXSRC_A,S626_GET_CRA_INDXSRC_A,2,2); s626_field!(S626_SET_CRA_CNTSRC_A,S626_GET_CRA_CNTSRC_A,0,2);
s626_field!(S626_SET_CRB_INTRESETCMD,S626_GET_CRB_CNTDIR_B,15,1); s626_field!(S626_SET_CRB_INTRESET_B,S626_GET_CRB_OVERDO_A,14,1); s626_field!(S626_SET_CRB_INTRESET_A,S626_GET_CRB_OVERDO_B,13,1); s626_field!(S626_SET_CRB_CLKENAB_A,S626_GET_CRB_CLKENAB_A,12,1); s626_field!(S626_SET_CRB_INTSRC_B,S626_GET_CRB_INTSRC_B,10,2); s626_field!(S626_SET_CRB_LATCHSRC,S626_GET_CRB_LATCHSRC,8,2); s626_field!(S626_SET_CRB_LOADSRC_B,S626_GET_CRB_LOADSRC_B,6,2); s626_field!(S626_SET_CRB_CLEAR_B,S626_GET_CRB_CLEAR_B,7,1); s626_field!(S626_SET_CRB_CLKMULT_B,S626_GET_CRB_CLKMULT_B,3,2); s626_field!(S626_SET_CRB_CLKENAB_B,S626_GET_CRB_CLKENAB_B,2,1); s626_field!(S626_SET_CRB_INDXPOL_B,S626_GET_CRB_INDXPOL_B,1,1); s626_field!(S626_SET_CRB_CLKPOL_B,S626_GET_CRB_CLKPOL_B,0,1);
pub const S626_CRBMSK_INTCTRL:u32= (1<<15)|(1<<13)|(1<<14);
macro_rules! S626_LP_RDDIN { ($x:expr) => { 0x0040 + ($x)*0x10 }; } macro_rules! S626_LP_WRINTSEL { ($x:expr) => { 0x0042 + ($x)*0x10 }; } macro_rules! S626_LP_WREDGSEL { ($x:expr) => { 0x0044 + ($x)*0x10 }; } macro_rules! S626_LP_WRCAPSEL { ($x:expr) => { 0x0046 + ($x)*0x10 }; } macro_rules! S626_LP_RDCAPFLG { ($x:expr) => { 0x0048 + ($x)*0x10 }; } macro_rules! S626_LP_WRDOUT { ($x:expr) => { 0x0048 + ($x)*0x10 }; } macro_rules! S626_LP_RDINTSEL { ($x:expr) => { 0x004a + ($x)*0x10 }; } macro_rules! S626_LP_RDEDGSEL { ($x:expr) => { 0x004c + ($x)*0x10 }; } macro_rules! S626_LP_RDCAPSEL { ($x:expr) => { 0x004e + ($x)*0x10 }; }
macro_rules! S626_LP_CRA { ($x:expr) => { ((($x)%3)*4) }; } macro_rules! S626_LP_CRB { ($x:expr) => { 2 + ((($x)%3)*4) }; } macro_rules! S626_LP_CNTR { ($x:expr) => { 0x000c + if ($x)<3 {0} else {4} + (($x)%3)*8 }; }
macro_rules! S626_STD_FIELD { ($set:ident,$get:ident,$bit:expr,$width:expr) => { macro_rules! $set { ($x:expr) => { S626_MAKE!($x,$width,$bit) }; } macro_rules! $get { ($x:expr) => { S626_UNMAKE!($x,$width,$bit) }; } }; }
S626_STD_FIELD!(S626_SET_STD_INTSRC,S626_GET_STD_INTSRC,13,2); S626_STD_FIELD!(S626_SET_STD_LATCHSRC,S626_GET_STD_LATCHSRC,11,2); S626_STD_FIELD!(S626_SET_STD_LOADSRC,S626_GET_STD_LOADSRC,9,2); S626_STD_FIELD!(S626_SET_STD_INDXSRC,S626_GET_STD_INDXSRC,7,2); S626_STD_FIELD!(S626_SET_STD_INDXPOL,S626_GET_STD_INDXPOL,6,1); S626_STD_FIELD!(S626_SET_STD_ENCMODE,S626_GET_STD_ENCMODE,4,2); S626_STD_FIELD!(S626_SET_STD_CLKPOL,S626_GET_STD_CLKPOL,3,1); S626_STD_FIELD!(S626_SET_STD_CLKMULT,S626_GET_STD_CLKMULT,1,2); S626_STD_FIELD!(S626_SET_STD_CLKENAB,S626_GET_STD_CLKENAB,0,1);
pub const S626_CRAMSK_INDXSRC_B:u32=(3<<14); pub const S626_CRAMSK_CNTSRC_B:u32=(3<<12); pub const S626_CRAMSK_INDXPOL_A:u32=1<<11; pub const S626_CRAMSK_LOADSRC_A:u32=3<<9; pub const S626_CRAMSK_CLKMULT_A:u32=3<<7; pub const S626_CRAMSK_INTSRC_A:u32=3<<5; pub const S626_CRAMSK_CLKPOL_A:u32=1<<4; pub const S626_CRAMSK_INDXSRC_A:u32=3<<2; pub const S626_CRAMSK_CNTSRC_A:u32=3;
pub const S626_CRBMSK_INTRESETCMD:u32=1<<15; pub const S626_CRBMSK_CNTDIR_B:u32=1<<15; pub const S626_CRBMSK_INTRESET_B:u32=1<<14; pub const S626_CRBMSK_OVERDO_A:u32=1<<14; pub const S626_CRBMSK_INTRESET_A:u32=1<<13; pub const S626_CRBMSK_OVERDO_B:u32=1<<13; pub const S626_CRBMSK_CLKENAB_A:u32=1<<12; pub const S626_CRBMSK_INTSRC_B:u32=3<<10; pub const S626_CRBMSK_LATCHSRC:u32=3<<8; pub const S626_CRBMSK_LOADSRC_B:u32=3<<6; pub const S626_CRBMSK_CLEAR_B:u32=1<<7; pub const S626_CRBMSK_CLKMULT_B:u32=3<<3; pub const S626_CRBMSK_CLKENAB_B:u32=1<<2; pub const S626_CRBMSK_INDXPOL_B:u32=1<<1; pub const S626_CRBMSK_CLKPOL_B:u32=1;
pub const S626_STDMSK_INTSRC:u32=3<<13; pub const S626_STDMSK_LATCHSRC:u32=3<<11; pub const S626_STDMSK_LOADSRC:u32=3<<9; pub const S626_STDMSK_INDXSRC:u32=3<<7; pub const S626_STDMSK_INDXPOL:u32=1<<6; pub const S626_STDMSK_ENCMODE:u32=3<<4; pub const S626_STDMSK_CLKPOL:u32=1<<3; pub const S626_STDMSK_CLKMULT:u32=3<<1; pub const S626_STDMSK_CLKENAB:u32=1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
