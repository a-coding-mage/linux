/* SPDX-License-Identifier: GPL-2.0 */
/* MPC8xx Communication Processor Module. */
/* C header dependencies: linux/init.h, asm/8xx_immap.h, asm/ptrace.h, asm/cpm.h */

pub type ushort = u16;
pub type uint = u32;
pub type u_char = u8;

pub const CPM_CR_RST: ushort = 0x8000;
pub const CPM_CR_OPCODE: ushort = 0x0f00;
pub const CPM_CR_CHAN: ushort = 0x00f0;
pub const CPM_CR_FLG: ushort = 0x0001;
pub const CPM_CR_CH_SCC1: ushort = 0x0000;
pub const CPM_CR_CH_I2C: ushort = 0x0001;
pub const CPM_CR_CH_SCC2: ushort = 0x0004;
pub const CPM_CR_CH_SPI: ushort = 0x0005;
pub const CPM_CR_CH_TIMER: ushort = CPM_CR_CH_SPI;
pub const CPM_CR_CH_SCC3: ushort = 0x0008;
pub const CPM_CR_CH_SMC1: ushort = 0x0009;
pub const CPM_CR_CH_SCC4: ushort = 0x000c;
pub const CPM_CR_CH_SMC2: ushort = 0x000d;
#[inline] pub const fn mk_cr_cmd(ch: ushort, cmd: ushort) -> ushort { (cmd << 8) | (ch << 4) }

extern "C" {
    pub static mut cpmp: *mut cpm8xx_t;
    pub fn cpm_setbrg(brg: uint, rate: uint);
    pub fn cpm_load_patch(cp: *mut cpm8xx_t);
    pub fn cpm_reset();
}
#[repr(C)] pub struct cpm8xx_t { _private: [u8; 0] }

pub const PROFF_SCC1:uint=0x0000; pub const PROFF_IIC:uint=0x0080; pub const PROFF_SCC2:uint=0x0100;
pub const PROFF_SPI:uint=0x0180; pub const PROFF_SCC3:uint=0x0200; pub const PROFF_SMC1:uint=0x0280;
pub const PROFF_DSP1:uint=0x02c0; pub const PROFF_SCC4:uint=0x0300; pub const PROFF_SMC2:uint=0x0380;

#[repr(C)] pub struct smc_uart_t {
 pub smc_rbase:ushort,pub smc_tbase:ushort,pub smc_rfcr:u_char,pub smc_tfcr:u_char,pub smc_mrblr:ushort,
 pub smc_rstate:uint,pub smc_idp:uint,pub smc_rbptr:ushort,pub smc_ibc:ushort,pub smc_rxtmp:uint,
 pub smc_tstate:uint,pub smc_tdp:uint,pub smc_tbptr:ushort,pub smc_tbc:ushort,pub smc_txtmp:uint,
 pub smc_maxidl:ushort,pub smc_tmpidl:ushort,pub smc_brklen:ushort,pub smc_brkec:ushort,pub smc_brkcr:ushort,
 pub smc_rmask:ushort,pub res1:[i8;8],pub smc_rpbase:ushort,
}
pub const SMC_EB:u_char=0x10;
pub const SMCMR_REN:ushort=1; pub const SMCMR_TEN:ushort=2; pub const SMCMR_DM:ushort=0x000c;
pub const SMCMR_SM_GCI:ushort=0; pub const SMCMR_SM_UART:ushort=0x20; pub const SMCMR_SM_TRANS:ushort=0x30;
pub const SMCMR_SM_MASK:ushort=0x30; pub const SMCMR_PM_EVEN:ushort=0x100; pub const SMCMR_REVD:ushort=SMCMR_PM_EVEN;
pub const SMCMR_PEN:ushort=0x200; pub const SMCMR_BS:ushort=SMCMR_PEN; pub const SMCMR_SL:ushort=0x400;
pub const SMCR_CLEN_MASK:ushort=0x7800; #[inline] pub const fn smcr_mk_clen(c:ushort)->ushort {(c<<11)&SMCR_CLEN_MASK}

#[repr(C)] pub struct smc_cent_t {
 pub scent_rbase:ushort,pub scent_tbase:ushort,pub scent_cfcr:u_char,pub scent_smask:u_char,pub scent_mrblr:ushort,
 pub scent_rstate:uint,pub scent_r_ptr:uint,pub scent_rbptr:ushort,pub scent_r_cnt:ushort,pub scent_rtemp:uint,
 pub scent_tstate:uint,pub scent_t_ptr:uint,pub scent_tbptr:ushort,pub scent_t_cnt:ushort,pub scent_ttemp:uint,
 pub scent_max_sl:ushort,pub scent_sl_cnt:ushort,pub scent_character1:ushort,pub scent_character2:ushort,pub scent_character3:ushort,pub scent_character4:ushort,pub scent_character5:ushort,pub scent_character6:ushort,pub scent_character7:ushort,pub scent_character8:ushort,pub scent_rccm:ushort,pub scent_rccr:ushort,
}
pub const SMC_CENT_F:u_char=8; pub const SMC_CENT_PE:u_char=4; pub const SMC_CENT_S:u_char=2;
pub const SMCM_BRKE:u_char=0x40; pub const SMCM_BRK:u_char=0x10; pub const SMCM_TXE:u_char=0x10; pub const SMCM_BSY:u_char=4; pub const SMCM_TX:u_char=2; pub const SMCM_RX:u_char=1;

/* Register constants retain their original integer widths and names. */
pub const CPM_BRG_RST:uint=0x00020000; pub const CPM_BRG_EN:uint=0x00010000; pub const CPM_BRG_EXTC_INT:uint=0; pub const CPM_BRG_EXTC_CLK2:uint=0x4000; pub const CPM_BRG_EXTC_CLK6:uint=0x8000; pub const CPM_BRG_ATB:uint=0x2000; pub const CPM_BRG_CD_MASK:uint=0x1ffe; pub const CPM_BRG_DIV16:uint=1;
pub const SICR_RCLK_SCC1_BRG1:uint=0; pub const SICR_TCLK_SCC1_BRG1:uint=0; pub const SICR_RCLK_SCC2_BRG2:uint=0x800; pub const SICR_TCLK_SCC2_BRG2:uint=0x100; pub const SICR_RCLK_SCC3_BRG3:uint=0x100000; pub const SICR_TCLK_SCC3_BRG3:uint=0x20000; pub const SICR_RCLK_SCC4_BRG4:uint=0x18000000; pub const SICR_TCLK_SCC4_BRG4:uint=0x03000000;
pub const SCC_TODR_TOD:ushort=0x8000; pub const SCCM_TXE:u_char=0x10; pub const SCCM_BSY:u_char=4; pub const SCCM_TX:u_char=2; pub const SCCM_RX:u_char=1;
pub const SCCE_ENET_GRA:ushort=0x80; pub const SCCE_ENET_TXE:ushort=0x10; pub const SCCE_ENET_RXF:ushort=8; pub const SCCE_ENET_BSY:ushort=4; pub const SCCE_ENET_TXB:ushort=2; pub const SCCE_ENET_RXB:ushort=1;
pub const SCC_PSMR_HBC:ushort=0x8000; pub const SCC_PSMR_FC:ushort=0x4000; pub const SCC_PSMR_RSH:ushort=0x2000; pub const SCC_PSMR_IAM:ushort=0x1000; pub const SCC_PSMR_ENCRC:ushort=0x800; pub const SCC_PSMR_PRO:ushort=0x200; pub const SCC_PSMR_BRO:ushort=0x100; pub const SCC_PSMR_SBT:ushort=0x80; pub const SCC_PSMR_LPB:ushort=0x40; pub const SCC_PSMR_SIP:ushort=0x20; pub const SCC_PSMR_LCW:ushort=0x10; pub const SCC_PSMR_NIB22:ushort=10; pub const SCC_PSMR_FDE:ushort=1;
pub const UART_SCCM_GLR:ushort=0x1000; pub const UART_SCCM_GLT:ushort=0x800; pub const UART_SCCM_AB:ushort=0x200; pub const UART_SCCM_IDL:ushort=0x100; pub const UART_SCCM_GRA:ushort=0x80; pub const UART_SCCM_BRKE:ushort=0x40; pub const UART_SCCM_BRKS:ushort=0x20; pub const UART_SCCM_CCR:ushort=8; pub const UART_SCCM_BSY:ushort=4; pub const UART_SCCM_TX:ushort=2; pub const UART_SCCM_RX:ushort=1;
pub const SCU_PSMR_FLC:ushort=0x8000; pub const SCU_PSMR_SL:ushort=0x4000; pub const SCU_PSMR_CL:ushort=0x3000; pub const SCU_PSMR_UM:ushort=0x0c00; pub const SCU_PSMR_FRZ:ushort=0x200; pub const SCU_PSMR_RZS:ushort=0x100; pub const SCU_PSMR_SYN:ushort=0x80; pub const SCU_PSMR_DRT:ushort=0x40; pub const SCU_PSMR_PEN:ushort=0x10; pub const SCU_PSMR_RPM:ushort=0x0c; pub const SCU_PSMR_REVP:ushort=8; pub const SCU_PSMR_TPM:ushort=3; pub const SCU_PSMR_TEVP:ushort=2;
pub const CICR_SCD_SCC4:uint=0x00c00000; pub const CICR_SCC_SCC3:uint=0x00200000; pub const CICR_SCB_SCC2:uint=0x00040000; pub const CICR_SCA_SCC1:uint=0; pub const CICR_IRL_MASK:uint=0x0000e000; pub const CICR_HP_MASK:uint=0x00001f00; pub const CICR_IEN:uint=0x80; pub const CICR_SPS:uint=1;
#[repr(C)] pub struct sccp_t { pub scc_rbase:ushort,pub scc_tbase:ushort,pub scc_rfcr:u_char,pub scc_tfcr:u_char,pub scc_mrblr:ushort,pub scc_rstate:uint,pub scc_idp:uint,pub scc_rbptr:ushort,pub scc_ibc:ushort,pub scc_rxtmp:uint,pub scc_tstate:uint,pub scc_tdp:uint,pub scc_tbptr:ushort,pub scc_tbc:ushort,pub scc_txtmp:uint,pub scc_rcrc:uint,pub scc_tcrc:uint }
pub const SCC_EB:u_char=0x10;
#[repr(C)] pub struct scc_enet_t { pub sen_genscc:sccp_t,pub sen_cpres:uint,pub sen_cmask:uint,pub sen_crcec:uint,pub sen_alec:uint,pub sen_disfc:uint,pub sen_pads:ushort,pub sen_retlim:ushort,pub sen_retcnt:ushort,pub sen_maxflr:ushort,pub sen_minflr:ushort,pub sen_maxd1:ushort,pub sen_maxd2:ushort,pub sen_maxd:ushort,pub sen_dmacnt:ushort,pub sen_maxb:ushort,pub sen_gaddr1:ushort,pub sen_gaddr2:ushort,pub sen_gaddr3:ushort,pub sen_gaddr4:ushort,pub sen_tbuf0data0:uint,pub sen_tbuf0data1:uint,pub sen_tbuf0rba:uint,pub sen_tbuf0crc:uint,pub sen_tbuf0bcnt:ushort,pub sen_paddrh:ushort,pub sen_paddrm:ushort,pub sen_paddrl:ushort,pub sen_pper:ushort,pub sen_rfbdptr:ushort,pub sen_tfbdptr:ushort,pub sen_tlbdptr:ushort,pub sen_tbuf1data0:uint,pub sen_tbuf1data1:uint,pub sen_tbuf1rba:uint,pub sen_tbuf1crc:uint,pub sen_tbuf1bcnt:ushort,pub sen_txlen:ushort,pub sen_iaddr1:ushort,pub sen_iaddr2:ushort,pub sen_iaddr3:ushort,pub sen_iaddr4:ushort,pub sen_boffcnt:ushort,pub sen_taddrh:ushort,pub sen_taddrm:ushort,pub sen_taddrl:ushort }
#[repr(C)] pub struct scc_uart_t { pub scc_genscc:sccp_t,pub res1:[i8;8],pub scc_maxidl:ushort,pub scc_idlc:ushort,pub scc_brkcr:ushort,pub scc_parec:ushort,pub scc_frmec:ushort,pub scc_nosec:ushort,pub scc_brkec:ushort,pub scc_brkln:ushort,pub scc_uaddr1:ushort,pub scc_uaddr2:ushort,pub scc_rtemp:ushort,pub scc_toseq:ushort,pub scc_char1:ushort,pub scc_char2:ushort,pub scc_char3:ushort,pub scc_char4:ushort,pub scc_char5:ushort,pub scc_char6:ushort,pub scc_char7:ushort,pub scc_char8:ushort,pub scc_rccm:ushort,pub scc_rccr:ushort,pub scc_rlbc:ushort }
#[repr(C)] pub struct scc_trans_t { pub st_genscc:sccp_t,pub st_cpres:uint,pub st_cmask:uint }
#[repr(C)] pub struct iic_t { pub iic_rbase:ushort,pub iic_tbase:ushort,pub iic_rfcr:u_char,pub iic_tfcr:u_char,pub iic_mrblr:ushort,pub iic_rstate:uint,pub iic_rdp:uint,pub iic_rbptr:ushort,pub iic_rbc:ushort,pub iic_rxtmp:uint,pub iic_tstate:uint,pub iic_tdp:uint,pub iic_tbptr:ushort,pub iic_tbc:ushort,pub iic_txtmp:uint,pub res1:[i8;4],pub iic_rpbase:ushort,pub res2:[i8;2] }
pub const PROFF_RTMR:uint=0x01b0; #[repr(C)] pub struct rt_pram_t { pub tm_base:ushort,pub tm_ptr:ushort,pub r_tmr:ushort,pub r_tmv:ushort,pub tm_cmd:unsigned_long,pub tm_cnt:unsigned_long }
pub type unsigned_long=u32;
pub const RCCR_TIME:u32=0x8000; #[inline] pub const fn RCCR_TIMEP(t:u32)->u32 {(t&0x3f)<<8} pub const RCCR_TIME_MASK:u32=0xff;
#[inline] pub const fn TM_CMD_NUM(n:u32)->u32 {(n&0xf)<<16} #[inline] pub const fn TM_CMD_PERIOD(p:u32)->u32 {p&0xffff} pub const TM_CMD_VALID:u32=0x80000000; pub const TM_CMD_RESTART:u32=0x40000000; pub const TM_CMD_PWM:u32=0x20000000;
pub const CPMVEC_NR:i32=32; pub const CPMVEC_PIO_PC15:ushort=0x1f; pub const CPMVEC_SCC1:ushort=0x1e; pub const CPMVEC_SCC2:ushort=0x1d; pub const CPMVEC_SCC3:ushort=0x1c; pub const CPMVEC_SCC4:ushort=0x1b; pub const CPMVEC_TIMER1:ushort=0x19; pub const CPMVEC_TIMER2:ushort=0x12; pub const CPMVEC_RISCTIMER:ushort=0x11; pub const CPMVEC_I2C:ushort=0x10; pub const CPMVEC_TIMER3:ushort=0x0c; pub const CPMVEC_TIMER4:ushort=7; pub const CPMVEC_SPI:ushort=5; pub const CPMVEC_SMC1:ushort=4; pub const CPMVEC_SMC2:ushort=3; pub const CPMVEC_ERROR:ushort=0;
pub const CPM_PIN_INPUT:i32=0; pub const CPM_PIN_OUTPUT:i32=1; pub const CPM_PIN_PRIMARY:i32=0; pub const CPM_PIN_SECONDARY:i32=2; pub const CPM_PIN_GPIO:i32=4; pub const CPM_PIN_OPENDRAIN:i32=8; pub const CPM_PIN_FALLEDGE:i32=16; pub const CPM_PIN_ANYEDGE:i32=0;
#[repr(C)] pub enum cpm_port { CPM_PORTA,CPM_PORTB,CPM_PORTC,CPM_PORTD,CPM_PORTE }
#[repr(C)] pub enum cpm_clk_dir { CPM_CLK_RX,CPM_CLK_TX,CPM_CLK_RTX }
#[repr(C)] pub enum cpm_clk_target { CPM_CLK_SCC1,CPM_CLK_SCC2,CPM_CLK_SCC3,CPM_CLK_SCC4,CPM_CLK_SMC1,CPM_CLK_SMC2 }
#[repr(C)] pub enum cpm_clk { CPM_BRG1,CPM_BRG2,CPM_BRG3,CPM_BRG4,CPM_CLK1,CPM_CLK2,CPM_CLK3,CPM_CLK4,CPM_CLK5,CPM_CLK6,CPM_CLK7,CPM_CLK8 }
extern "C" { pub fn cpm1_set_pin(port:cpm_port,pin:i32,flags:i32); pub fn cpm1_clk_setup(target:cpm_clk_target,clock:i32,mode:i32)->i32; pub fn cpm1_gpiochip_add16(dev:*mut device)->i32; pub fn cpm1_gpiochip_add32(dev:*mut device)->i32; }
#[repr(C)] pub struct device { _private:[u8;0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
