/* SPDX-License-Identifier: GPL-2.0-or-later */
/* RapidIO register definitions, translated from rio_regs.h. */

pub const RIO_MAINT_SPACE_SZ: u32 = 0x1000000;

pub const RIO_DEV_ID_CAR: u32 = 0x00;
pub const RIO_DEV_INFO_CAR: u32 = 0x04;
pub const RIO_ASM_ID_CAR: u32 = 0x08;
pub const RIO_ASM_ID_MASK: u32 = 0xffff0000;
pub const RIO_ASM_VEN_ID_MASK: u32 = 0x0000ffff;
pub const RIO_ASM_INFO_CAR: u32 = 0x0c;
pub const RIO_ASM_REV_MASK: u32 = 0xffff0000;
pub const RIO_EXT_FTR_PTR_MASK: u32 = 0x0000ffff;
pub const RIO_PEF_CAR: u32 = 0x10;
pub const RIO_PEF_BRIDGE: u32 = 0x80000000;
pub const RIO_PEF_MEMORY: u32 = 0x40000000;
pub const RIO_PEF_PROCESSOR: u32 = 0x20000000;
pub const RIO_PEF_SWITCH: u32 = 0x10000000;
pub const RIO_PEF_MULTIPORT: u32 = 0x08000000;
pub const RIO_PEF_INB_MBOX: u32 = 0x00f00000;
pub const RIO_PEF_INB_MBOX0: u32 = 0x00800000;
pub const RIO_PEF_INB_MBOX1: u32 = 0x00400000;
pub const RIO_PEF_INB_MBOX2: u32 = 0x00200000;
pub const RIO_PEF_INB_MBOX3: u32 = 0x00100000;
pub const RIO_PEF_INB_DOORBELL: u32 = 0x00080000;
pub const RIO_PEF_DEV32: u32 = 0x00001000;
pub const RIO_PEF_EXT_RT: u32 = 0x00000200;
pub const RIO_PEF_STD_RT: u32 = 0x00000100;
pub const RIO_PEF_CTLS: u32 = 0x00000010;
pub const RIO_PEF_DEV16: u32 = 0x00000010;
pub const RIO_PEF_EXT_FEATURES: u32 = 0x00000008;
pub const RIO_PEF_ADDR_66: u32 = 0x00000004;
pub const RIO_PEF_ADDR_50: u32 = 0x00000002;
pub const RIO_PEF_ADDR_34: u32 = 0x00000001;
pub const RIO_SWP_INFO_CAR: u32 = 0x14;
pub const RIO_SWP_INFO_PORT_TOTAL_MASK: u32 = 0x0000ff00;
pub const RIO_SWP_INFO_PORT_NUM_MASK: u32 = 0x000000ff;
pub const RIO_SRC_OPS_CAR: u32 = 0x18;
pub const RIO_DST_OPS_CAR: u32 = 0x1c;

pub const RIO_OPS_READ: u32 = 0x00008000;
pub const RIO_OPS_WRITE: u32 = 0x00004000;
pub const RIO_OPS_STREAM_WRITE: u32 = 0x00002000;
pub const RIO_OPS_WRITE_RESPONSE: u32 = 0x00001000;
pub const RIO_OPS_DATA_MSG: u32 = 0x00000800;
pub const RIO_OPS_DOORBELL: u32 = 0x00000400;
pub const RIO_OPS_ATOMIC_TST_SWP: u32 = 0x00000100;
pub const RIO_OPS_ATOMIC_INC: u32 = 0x00000080;
pub const RIO_OPS_ATOMIC_DEC: u32 = 0x00000040;
pub const RIO_OPS_ATOMIC_SET: u32 = 0x00000020;
pub const RIO_OPS_ATOMIC_CLR: u32 = 0x00000010;
pub const RIO_OPS_PORT_WRITE: u32 = 0x00000004;

pub const RIO_SRC_OPS_READ:u32=RIO_OPS_READ; pub const RIO_SRC_OPS_WRITE:u32=RIO_OPS_WRITE; pub const RIO_SRC_OPS_STREAM_WRITE:u32=RIO_OPS_STREAM_WRITE; pub const RIO_SRC_OPS_WRITE_RESPONSE:u32=RIO_OPS_WRITE_RESPONSE; pub const RIO_SRC_OPS_DATA_MSG:u32=RIO_OPS_DATA_MSG; pub const RIO_SRC_OPS_DOORBELL:u32=RIO_OPS_DOORBELL; pub const RIO_SRC_OPS_ATOMIC_TST_SWP:u32=RIO_OPS_ATOMIC_TST_SWP; pub const RIO_SRC_OPS_ATOMIC_INC:u32=RIO_OPS_ATOMIC_INC; pub const RIO_SRC_OPS_ATOMIC_DEC:u32=RIO_OPS_ATOMIC_DEC; pub const RIO_SRC_OPS_ATOMIC_SET:u32=RIO_OPS_ATOMIC_SET; pub const RIO_SRC_OPS_ATOMIC_CLR:u32=RIO_OPS_ATOMIC_CLR; pub const RIO_SRC_OPS_PORT_WRITE:u32=RIO_OPS_PORT_WRITE;
pub const RIO_DST_OPS_READ:u32=RIO_OPS_READ; pub const RIO_DST_OPS_WRITE:u32=RIO_OPS_WRITE; pub const RIO_DST_OPS_STREAM_WRITE:u32=RIO_OPS_STREAM_WRITE; pub const RIO_DST_OPS_WRITE_RESPONSE:u32=RIO_OPS_WRITE_RESPONSE; pub const RIO_DST_OPS_DATA_MSG:u32=RIO_OPS_DATA_MSG; pub const RIO_DST_OPS_DOORBELL:u32=RIO_OPS_DOORBELL; pub const RIO_DST_OPS_ATOMIC_TST_SWP:u32=RIO_OPS_ATOMIC_TST_SWP; pub const RIO_DST_OPS_ATOMIC_INC:u32=RIO_OPS_ATOMIC_INC; pub const RIO_DST_OPS_ATOMIC_DEC:u32=RIO_OPS_ATOMIC_DEC; pub const RIO_DST_OPS_ATOMIC_SET:u32=RIO_OPS_ATOMIC_SET; pub const RIO_DST_OPS_ATOMIC_CLR:u32=RIO_OPS_ATOMIC_CLR; pub const RIO_DST_OPS_PORT_WRITE:u32=RIO_OPS_PORT_WRITE;
pub const RIO_SWITCH_RT_LIMIT: u32 = 0x34;
pub const RIO_RT_MAX_DESTID: u32 = 0x0000ffff;
pub const RIO_MBOX_CSR: u32 = 0x40;
pub const RIO_WRITE_PORT_CSR: u32 = 0x44;
pub const RIO_DOORBELL_CSR: u32 = 0x44;
pub const RIO_PELL_CTRL_CSR: u32 = 0x4c;
pub const RIO_PELL_ADDR_66: u32 = 4;
pub const RIO_PELL_ADDR_50: u32 = 2;
pub const RIO_PELL_ADDR_34: u32 = 1;
pub const RIO_LCSH_BA: u32 = 0x58;
pub const RIO_LCSL_BA: u32 = 0x5c;
pub const RIO_DID_CSR: u32 = 0x60;
pub const RIO_HOST_DID_LOCK_CSR: u32 = 0x68;
pub const RIO_COMPONENT_TAG_CSR: u32 = 0x6c;
pub const RIO_STD_RTE_CONF_DESTID_SEL_CSR: u32 = 0x70;
pub const RIO_STD_RTE_CONF_EXTCFGEN: u32 = 0x80000000;
pub const RIO_STD_RTE_CONF_PORT_SEL_CSR: u32 = 0x74;
pub const RIO_STD_RTE_DEFAULT_PORT: u32 = 0x78;

pub const RIO_EFB_PTR_MASK: u32 = 0xffff0000;
pub const RIO_EFB_ID_MASK: u32 = 0x0000ffff;
pub const RIO_EFB_SER_EP_M1_ID: u32 = 1;
pub const RIO_EFB_SER_EP_SW_M1_ID: u32 = 2;
pub const RIO_EFB_SER_EPF_M1_ID: u32 = 3;
pub const RIO_EFB_SER_EP_ID: u32 = 4;
pub const RIO_EFB_SER_EP_REC_ID: u32 = 5;
pub const RIO_EFB_SER_EP_FREE_ID: u32 = 6;
pub const RIO_EFB_ERR_MGMNT: u32 = 7;
pub const RIO_EFB_SER_EPF_SW_M1_ID: u32 = 9;
pub const RIO_EFB_SW_ROUTING_TBL: u32 = 0x000e;
pub const RIO_EFB_SER_EP_M2_ID: u32 = 0x11;
pub const RIO_EFB_SER_EP_SW_M2_ID: u32 = 0x12;
pub const RIO_EFB_SER_EPF_M2_ID: u32 = 0x13;
pub const RIO_EFB_ERR_MGMNT_HS: u32 = 0x17;
pub const RIO_EFB_SER_EPF_SW_M2_ID: u32 = 0x19;

#[inline] pub const fn rio_get_total_ports(x: u32) -> u32 { (x & 0x0000ff00) >> 8 }
#[inline] pub const fn rio_get_port_num(x: u32) -> u32 { x & 0xff }
#[inline] pub const fn rio_get_block_ptr(x: u32) -> u32 { (x & RIO_EFB_PTR_MASK) >> 16 }
#[inline] pub const fn rio_get_block_id(x: u32) -> u32 { x & RIO_EFB_ID_MASK }

pub const RIO_PORT_MNT_HEADER: u32 = 0;
pub const RIO_PORT_REQ_CTL_CSR: u32 = 0x20;
pub const RIO_PORT_RSP_CTL_CSR: u32 = 0x24;
pub const RIO_PORT_LINKTO_CTL_CSR: u32 = 0x20;
pub const RIO_PORT_RSPTO_CTL_CSR: u32 = 0x24;
pub const RIO_PORT_GEN_CTL_CSR: u32 = 0x3c;
pub const RIO_PORT_GEN_HOST: u32 = 0x80000000;
pub const RIO_PORT_GEN_MASTER: u32 = 0x40000000;
pub const RIO_PORT_GEN_DISCOVERED: u32 = 0x20000000;
#[inline] pub const fn rio_port_n_mnt_req_csr(n:u32,m:u32)->u32 { 0x40+n*(0x20*m) }
#[inline] pub const fn rio_port_n_mnt_rsp_csr(n:u32,m:u32)->u32 { 0x44+n*(0x20*m) }
#[inline] pub const fn rio_port_n_ack_sts_csr(n:u32)->u32 { 0x48+n*0x20 }
#[inline] pub const fn rio_port_n_ctl2_csr(n:u32,m:u32)->u32 { 0x54+n*(0x20*m) }
#[inline] pub const fn rio_port_n_err_sts_csr(n:u32,m:u32)->u32 { 0x58+n*(0x20*m) }
#[inline] pub const fn rio_port_n_ctl_csr(n:u32,m:u32)->u32 { 0x5c+n*(0x20*m) }
#[inline] pub const fn rio_port_n_ob_ack_csr(n:u32)->u32 { 0x60+n*0x40 }
#[inline] pub const fn rio_port_n_ib_ack_csr(n:u32)->u32 { 0x64+n*0x40 }
pub const RIO_MNT_REQ_CMD_RD:u32=3; pub const RIO_MNT_REQ_CMD_IS:u32=4;
pub const RIO_PORT_N_MNT_RSP_RVAL:u32=0x80000000; pub const RIO_PORT_N_MNT_RSP_ASTAT:u32=0x7e0; pub const RIO_PORT_N_MNT_RSP_LSTAT:u32=0x1f;
pub const RIO_PORT_N_ACK_CLEAR:u32=0x80000000; pub const RIO_PORT_N_ACK_INBOUND:u32=0x3f000000; pub const RIO_PORT_N_ACK_OUTSTAND:u32=0x3f00; pub const RIO_PORT_N_ACK_OUTBOUND:u32=0x3f;
pub const RIO_PORT_N_CTL2_SEL_BAUD:u32=0xf0000000;
pub const RIO_PORT_N_ERR_STS_OUT_ES:u32=0x10000; pub const RIO_PORT_N_ERR_STS_INP_ES:u32=0x100; pub const RIO_PORT_N_ERR_STS_PW_PEND:u32=0x10; pub const RIO_PORT_N_ERR_STS_PORT_UA:u32=8; pub const RIO_PORT_N_ERR_STS_PORT_ERR:u32=4; pub const RIO_PORT_N_ERR_STS_PORT_OK:u32=2; pub const RIO_PORT_N_ERR_STS_PORT_UNINIT:u32=1;
pub const RIO_PORT_N_CTL_PWIDTH:u32=0xc0000000; pub const RIO_PORT_N_CTL_PWIDTH_1:u32=0; pub const RIO_PORT_N_CTL_PWIDTH_4:u32=0x40000000; pub const RIO_PORT_N_CTL_IPW:u32=0x38000000; pub const RIO_PORT_N_CTL_P_TYP_SER:u32=1; pub const RIO_PORT_N_CTL_LOCKOUT:u32=2; pub const RIO_PORT_N_CTL_EN_RX:u32=0x200000; pub const RIO_PORT_N_CTL_EN_TX:u32=0x400000;
pub const RIO_PORT_N_OB_ACK_CLEAR:u32=0x80000000; pub const RIO_PORT_N_OB_ACK_OUTSTD:u32=0x00fff000; pub const RIO_PORT_N_OB_ACK_OUTBND:u32=0xfff; pub const RIO_PORT_N_IB_ACK_INBND:u32=0xfff;

/* Device helper macros retain C-style field access and pointer arithmetic. */
macro_rules! RIO_DEV_PORT_N_MNT_REQ_CSR { ($d:expr,$n:expr) => { ($d.phys_efptr + rio_port_n_mnt_req_csr($n, $d.phys_rmap)) }; }
macro_rules! RIO_DEV_PORT_N_MNT_RSP_CSR { ($d:expr,$n:expr) => { ($d.phys_efptr + rio_port_n_mnt_rsp_csr($n, $d.phys_rmap)) }; }
macro_rules! RIO_DEV_PORT_N_ACK_STS_CSR { ($d:expr,$n:expr) => { ($d.phys_efptr + rio_port_n_ack_sts_csr($n)) }; }
macro_rules! RIO_DEV_PORT_N_CTL2_CSR { ($d:expr,$n:expr) => { ($d.phys_efptr + rio_port_n_ctl2_csr($n, $d.phys_rmap)) }; }
macro_rules! RIO_DEV_PORT_N_ERR_STS_CSR { ($d:expr,$n:expr) => { ($d.phys_efptr + rio_port_n_err_sts_csr($n, $d.phys_rmap)) }; }
macro_rules! RIO_DEV_PORT_N_CTL_CSR { ($d:expr,$n:expr) => { ($d.phys_efptr + rio_port_n_ctl_csr($n, $d.phys_rmap)) }; }
macro_rules! RIO_DEV_PORT_N_OB_ACK_CSR { ($d:expr,$n:expr) => { ($d.phys_efptr + rio_port_n_ob_ack_csr($n)) }; }
macro_rules! RIO_DEV_PORT_N_IB_ACK_CSR { ($d:expr,$n:expr) => { ($d.phys_efptr + rio_port_n_ib_ack_csr($n)) }; }

pub const RIO_EM_EFB_HEADER:u32=0; pub const RIO_EM_EMHS_CAR:u32=4; pub const RIO_EM_LTL_ERR_DETECT:u32=8; pub const RIO_EM_LTL_ERR_EN:u32=0xc;
pub const REM_LTL_ERR_ILLTRAN:u32=0x08000000; pub const REM_LTL_ERR_UNSOLR:u32=0x00800000; pub const REM_LTL_ERR_UNSUPTR:u32=0x00400000; pub const REM_LTL_ERR_IMPSPEC:u32=0xff;
pub const RIO_EM_LTL_HIADDR_CAP:u32=0x10; pub const RIO_EM_LTL_ADDR_CAP:u32=0x14; pub const RIO_EM_LTL_DEVID_CAP:u32=0x18; pub const RIO_EM_LTL_CTRL_CAP:u32=0x1c; pub const RIO_EM_LTL_DID32_CAP:u32=0x20; pub const RIO_EM_LTL_SID32_CAP:u32=0x24; pub const RIO_EM_PW_TGT_DEVID:u32=0x28; pub const RIO_EM_PKT_TTL:u32=0x2c; pub const RIO_EM_PW_TGT32_DEVID:u32=0x30; pub const RIO_EM_PW_TX_CTRL:u32=0x34;
pub const RIO_EM_PW_TGT_DEVID_D16M:u32=0xff000000; pub const RIO_EM_PW_TGT_DEVID_D8:u32=0xff0000; pub const RIO_EM_PW_TGT_DEVID_DEV16:u32=0x8000; pub const RIO_EM_PW_TGT_DEVID_DEV32:u32=0x4000; pub const RIO_EM_PKT_TTL_VAL:u32=0xffff0000; pub const RIO_EM_PW_TX_CTRL_PW_DIS:u32=1;
#[inline] pub const fn rio_em_pn_err_detect(x:u32)->u32{0x40+x*0x40} #[inline] pub const fn rio_em_pn_errrate_en(x:u32)->u32{0x44+x*0x40} #[inline] pub const fn rio_em_pn_attrib_cap(x:u32)->u32{0x48+x*0x40} #[inline] pub const fn rio_em_pn_pkt_cap_0(x:u32)->u32{0x4c+x*0x40} #[inline] pub const fn rio_em_pn_pkt_cap_1(x:u32)->u32{0x50+x*0x40} #[inline] pub const fn rio_em_pn_pkt_cap_2(x:u32)->u32{0x54+x*0x40} #[inline] pub const fn rio_em_pn_pkt_cap_3(x:u32)->u32{0x58+x*0x40} #[inline] pub const fn rio_em_pn_errrate(x:u32)->u32{0x68+x*0x40} #[inline] pub const fn rio_em_pn_errrate_tr(x:u32)->u32{0x6c+x*0x40} #[inline] pub const fn rio_em_pn_link_udt(x:u32)->u32{0x70+x*0x40}
pub const REM_PED_IMPL_SPEC:u32=0x80000000; pub const REM_PED_LINK_OK2U:u32=0x40000000; pub const REM_PED_LINK_UPDA:u32=0x20000000; pub const REM_PED_LINK_U2OK:u32=0x10000000; pub const REM_PED_LINK_TO:u32=1; pub const RIO_EM_PN_ERRRATE_EN_OK2U:u32=0x40000000; pub const RIO_EM_PN_ERRRATE_EN_UPDA:u32=0x20000000; pub const RIO_EM_PN_ERRRATE_EN_U2OK:u32=0x10000000; pub const RIO_EM_PN_LINK_UDT_TO:u32=0xffffff00;

pub const RIO_BC_RT_CTL_CSR:u32=0x20; pub const RIO_RT_CTL_THREE_LVL:u32=0x80000000; pub const RIO_RT_CTL_DEV32_RT_CTRL:u32=0x40000000; pub const RIO_RT_CTL_MC_MASK_SZ:u32=0x03000000;
pub const RIO_BC_RT_LVL0_INFO_CSR:u32=0x30; pub const RIO_BC_RT_LVL1_INFO_CSR:u32=0x34; pub const RIO_BC_RT_LVL2_INFO_CSR:u32=0x38;
pub const RIO_RT_L0I_NUM_GR:u32=0xff000000; pub const RIO_RT_L0I_GR_PTR:u32=0x00fffc00; pub const RIO_RT_L1I_NUM_GR:u32=0xff000000; pub const RIO_RT_L1I_GR_PTR:u32=0x00fffc00; pub const RIO_RT_L2I_NUM_GR:u32=0xff000000; pub const RIO_RT_L2I_GR_PTR:u32=0x00fffc00;
#[inline] pub const fn rio_spx_rt_ctl_csr(x:u32)->u32{0x40+0x20*x} #[inline] pub const fn rio_spx_rt_lvl0_info_csr(x:u32)->u32{0x50+0x20*x} #[inline] pub const fn rio_spx_rt_lvl1_info_csr(x:u32)->u32{0x54+0x20*x} #[inline] pub const fn rio_spx_rt_lvl2_info_csr(x:u32)->u32{0x58+0x20*x}
pub const RIO_RT_LN_ENTRY_IMPL_DEF:u32=0xf0000000; pub const RIO_RT_LN_ENTRY_RTE_VAL:u32=0x3ff; pub const RIO_RT_ENTRY_DROP_PKT:u32=0x300;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
