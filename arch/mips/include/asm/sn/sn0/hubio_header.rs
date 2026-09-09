/* Rust translation of hubio.h. */

pub const IIO_LLP_CSR_IS_UP: u64 = 0x0000_2000;
pub const IIO_LLP_CSR_LLP_STAT_MASK: u64 = 0x0000_3000;
pub const IIO_LLP_CSR_LLP_STAT_SHFT: u32 = 12;
pub const IIO_PROTECT_OVRRD_KEY: u64 = 0x5349_4772_756c_6573;

/* Hardware register names and their friendly aliases. */
pub const IIO_BASE: u64 = 0x400000;
pub const IIO_BASE_BTE0: u64 = 0x410000;
pub const IIO_BASE_BTE1: u64 = 0x420000;
pub const IIO_BASE_PERF: u64 = 0x430000;
pub const IIO_PERF_CNT: u64 = 0x430008;
pub const IO_PERF_SETS: u64 = 32;
pub const IIO_WID: u64 = 0x400000;
pub const IIO_WSTAT: u64 = 0x400008;
pub const IIO_WCR: u64 = 0x400020;
pub const IIO_WSTAT_ECRAZY: u64 = 1u64 << 32;
pub const IIO_WSTAT_TXRETRY: u64 = 1u64 << 9;
pub const IIO_WSTAT_TXRETRY_MASK: u64 = 0x7f;
pub const IIO_WSTAT_TXRETRY_SHFT: u32 = 16;
#[inline] pub const fn IIO_WSTAT_TXRETRY_CNT(w: u64) -> u64 { (w >> IIO_WSTAT_TXRETRY_SHFT) & IIO_WSTAT_TXRETRY_MASK }
pub const IIO_ILAPR: u64 = 0x400100;
pub const IIO_ILAPO: u64 = 0x400108;
pub const IIO_IOWA: u64 = 0x400110;
pub const IIO_IIWA: u64 = 0x400118;
pub const IIO_IIDEM: u64 = 0x400120;
pub const IIO_ILCSR: u64 = 0x400128;
pub const IIO_ILLR: u64 = 0x400130;
pub const IIO_IIDSR: u64 = 0x400138;
pub const IIO_IIBUSERR: u64 = 0x1400208;
pub const IIO_IIDSR_SENT_SHIFT: u32 = 28; pub const IIO_IIDSR_SENT_MASK: u64 = 0x10000000;
pub const IIO_IIDSR_ENB_SHIFT: u32 = 24; pub const IIO_IIDSR_ENB_MASK: u64 = 0x01000000;
pub const IIO_IIDSR_NODE_SHIFT: u32 = 8; pub const IIO_IIDSR_NODE_MASK: u64 = 0x0000ff00;
pub const IIO_IIDSR_LVL_SHIFT: u32 = 0; pub const IIO_IIDSR_LVL_MASK: u64 = 0x3f;
pub const IIO_IGFX_0: u64 = 0x400140; pub const IIO_IGFX_1: u64 = 0x400148;
pub const IIO_IGFX_W_NUM_BITS: u64 = 4; pub const IIO_IGFX_W_NUM_MASK: u64 = (1 << 4) - 1; pub const IIO_IGFX_W_NUM_SHIFT: u32 = 0;
pub const IIO_IGFX_N_NUM_BITS: u64 = 9; pub const IIO_IGFX_N_NUM_MASK: u64 = (1 << 9) - 1; pub const IIO_IGFX_N_NUM_SHIFT: u32 = 4;
pub const IIO_IGFX_P_NUM_BITS: u64 = 1; pub const IIO_IGFX_P_NUM_MASK: u64 = 1; pub const IIO_IGFX_P_NUM_SHIFT: u32 = 16;
pub const IIO_IGFX_VLD_BITS: u64 = 1; pub const IIO_IGFX_VLD_MASK: u64 = 1; pub const IIO_IGFX_VLD_SHIFT: u32 = 20;
#[inline] pub const fn IIO_IGFX_INIT(widget:u64,node:u64,cpu:u64,valid:u64)->u64 { ((widget&IIO_IGFX_W_NUM_MASK)<<IIO_IGFX_W_NUM_SHIFT)|((node&IIO_IGFX_N_NUM_MASK)<<IIO_IGFX_N_NUM_SHIFT)|((cpu&IIO_IGFX_P_NUM_MASK)<<IIO_IGFX_P_NUM_SHIFT)|((valid&IIO_IGFX_VLD_MASK)<<IIO_IGFX_VLD_SHIFT) }
pub const IIO_SCRATCH_REG0:u64=0x400150; pub const IIO_SCRATCH_REG1:u64=0x400158; pub const IIO_SCRATCH_MASK:u64=0x0000000f00f11fff;
pub const IIO_SCRATCH_BIT0_0:u64=0x0000000800000000; pub const IIO_SCRATCH_BIT0_1:u64=0x0000000400000000; pub const IIO_SCRATCH_BIT0_2:u64=0x0000000200000000; pub const IIO_SCRATCH_BIT0_3:u64=0x0000000100000000;
pub const IIO_SCRATCH_BIT0_4:u64=0x0000000000800000; pub const IIO_SCRATCH_BIT0_5:u64=0x0000000000400000; pub const IIO_SCRATCH_BIT0_6:u64=0x0000000000200000; pub const IIO_SCRATCH_BIT0_7:u64=0x0000000000100000; pub const IIO_SCRATCH_BIT0_8:u64=0x0000000000010000; pub const IIO_SCRATCH_BIT0_9:u64=0x0000000000001000; pub const IIO_SCRATCH_BIT0_R:u64=0xfff;
pub const IIO_NUM_ITTES:u64=7; pub const HUB_NUM_BIG_WINDOW:u64=IIO_NUM_ITTES-1; pub const SWIN0_BIGWIN:u64=HUB_NUM_BIG_WINDOW; pub const ILCSR_WARM_RESET:u64=0x100;

/* C bit-field unions retain their register-sized representation here. */
#[repr(C)] #[derive(Copy,Clone)] pub union hubii_wid_u { pub wid_reg_value:u64, pub wid_fields_s:[u8;8] } pub type hubii_wid_t=hubii_wid_u;
#[repr(C)] #[derive(Copy,Clone)] pub union hubii_wcr_u { pub wcr_reg_value:u64, pub wcr_fields_s:[u8;8] } pub type hubii_wcr_t=hubii_wcr_u;
#[repr(C)] #[derive(Copy,Clone)] pub union hubii_wstat_u { pub reg_value:u64, pub wstat_fields_s:[u8;8] } pub type hubii_wstat_t=hubii_wstat_u;
#[repr(C)] #[derive(Copy,Clone)] pub union hubii_ilcsr_u { pub icsr_reg_value:u64, pub icsr_fields_s:[u8;8] } pub type hubii_ilcsr_t=hubii_ilcsr_u;
#[repr(C)] #[derive(Copy,Clone)] pub union hubii_iowa_u { pub iowa_reg_value:u64, pub iowa_fields_s:[u8;8] } pub type hubii_iowa_t=hubii_iowa_u;
#[repr(C)] #[derive(Copy,Clone)] pub union hubii_iiwa_u { pub iiwa_reg_value:u64, pub iiwa_fields_s:[u8;8] } pub type hubii_iiwa_t=hubii_iiwa_u;
#[repr(C)] #[derive(Copy,Clone)] pub union hubii_illr_u { pub illr_reg_value:u64, pub illr_fields_s:[u8;8] } pub type hubii_illr_t=hubii_illr_u;
#[repr(C)] #[derive(Copy,Clone)] pub union io_perf_sel { pub perf_sel_reg:u64, pub perf_sel_bits:[u8;8] } pub type io_perf_sel_t=io_perf_sel;
#[repr(C)] #[derive(Copy,Clone)] pub union io_perf_cnt { pub perf_cnt:u64, pub perf_cnt_bits:[u8;8] } pub type io_perf_cnt_t=io_perf_cnt;

pub const LNK_STAT_WORKING:u64=2; pub const IIO_LLP_CB_MAX:u64=0xffff; pub const IIO_LLP_SN_MAX:u64=0xffff;
pub const IIO_NUM_IPRBS:u64=9; pub const IIO_IOPRB_0:u64=0x400198; pub const IIO_IOPRB_8:u64=0x4001a0; pub const IIO_IOPRB_9:u64=0x4001a8; pub const IIO_IOPRB_A:u64=0x4001b0; pub const IIO_IOPRB_B:u64=0x4001b8; pub const IIO_IOPRB_C:u64=0x4001c0; pub const IIO_IOPRB_D:u64=0x4001c8; pub const IIO_IOPRB_E:u64=0x4001d0; pub const IIO_IOPRB_F:u64=0x4001d8;
pub const IIO_IXCC:u64=0x4001e0; pub const IIO_IXTCC:u64=IIO_IXCC; pub const IIO_IMEM:u64=0x4001e8; pub const IIO_IXTT:u64=0x4001f0; pub const IIO_IECLR:u64=0x4001f8; pub const IIO_IBCN:u64=0x400200;
pub const IIO_IMEM_W0ESD:u64=1; pub const IIO_IMEM_B0ESD:u64=1<<4; pub const IIO_IMEM_B1ESD:u64=1<<8;
pub const IIO_IPCA:u64=0x400300; pub const IIO_NUM_PRTES:u64=8; pub const IIO_PRTE_0:u64=0x400308; #[inline] pub const fn IIO_PRTE(x:u64)->u64{IIO_PRTE_0+8*x} #[inline] pub const fn IIO_WIDPRTE(x:u64)->u64{IIO_PRTE(x-8)} pub const IIO_IPDR:u64=0x400388; pub const IIO_ICDR:u64=0x400390; pub const IIO_IFDR:u64=0x400398; pub const IIO_IIAP:u64=0x4003a0; pub const IIO_IMMR:u64=IIO_IIAP; pub const IIO_ICMR:u64=0x4003a8; pub const IIO_ICCR:u64=0x4003b0; pub const IIO_ICTO:u64=0x4003b8; pub const IIO_ICTP:u64=0x4003c0;
pub const IIO_ICMR_PC_VLD_SHFT:u32=36; pub const IIO_ICMR_PC_VLD_MASK:u64=0x7fff<<36; pub const IIO_ICMR_CRB_VLD_SHFT:u32=20; pub const IIO_ICMR_CRB_VLD_MASK:u64=0x7fff<<20; pub const IIO_ICMR_FC_CNT_SHFT:u32=16; pub const IIO_ICMR_FC_CNT_MASK:u64=0xf<<16; pub const IIO_ICMR_C_CNT_SHFT:u32=4; pub const IIO_ICMR_C_CNT_MASK:u64=0xf<<4; pub const IIO_ICMR_P_CNT_SHFT:u32=0; pub const IIO_ICMR_P_CNT_MASK:u64=0xf; pub const IIO_ICMR_PRECISE:u64=1<<52; pub const IIO_ICMR_CLR_RPPD:u64=1<<13; pub const IIO_ICMR_CLR_RQPD:u64=1<<12;
pub const IIO_IPDR_PND:u64=1<<4; pub const IIO_ICDR_PND:u64=1<<4; pub const IIO_ICCR_PENDING:u64=0x10000; pub const IIO_ICCR_CMD_MASK:u64=0xff; pub const IIO_ICCR_CMD_SHFT:u32=7; pub const IIO_ICCR_CMD_NOP:u64=0; pub const IIO_ICCR_CMD_WAKE:u64=0x100; pub const IIO_ICCR_CMD_TIMEOUT:u64=0x200; pub const IIO_ICCR_CMD_EJECT:u64=0x400; pub const IIO_ICCR_CMD_FLUSH:u64=0x800;
pub const IIO_NUM_CRBS:u64=15; pub const IIO_NUM_NORMAL_CRBS:u64=12; pub const IIO_NUM_PC_CRBS:u64=4; pub const IIO_ICRB_OFFSET:u64=8; pub const IIO_ICRB_0:u64=0x400400; #[inline] pub const fn IIO_ICRB_A(x:u64)->u64{IIO_ICRB_0+4*IIO_ICRB_OFFSET*x} #[inline] pub const fn IIO_ICRB_B(x:u64)->u64{IIO_ICRB_A(x)+IIO_ICRB_OFFSET} #[inline] pub const fn IIO_ICRB_C(x:u64)->u64{IIO_ICRB_A(x)+2*IIO_ICRB_OFFSET} #[inline] pub const fn IIO_ICRB_D(x:u64)->u64{IIO_ICRB_A(x)+3*IIO_ICRB_OFFSET}

/* CRB and remaining register unions. */
macro_rules! register_union { ($u:ident,$t:ident,$raw:ident,$view:ident) => { #[repr(C)] #[derive(Copy,Clone)] pub union $u { pub $raw:u64, pub $view:[u8;8] } pub type $t=$u; }; }
register_union!(icrba_u,icrba_t,reg_value,icrba_fields_s); register_union!(h1_icrba_u,h1_icrba_t,reg_value,h1_icrba_fields_s); register_union!(icrbb_u,icrbb_t,reg_value,icrbb_field_s); register_union!(h1_icrbb_u,h1_icrbb_t,reg_value,h1_icrbb_field_s); register_union!(icrbc_s,icrbc_t,reg_value,icrbc_field_s); register_union!(icrbd_s,icrbd_t,reg_value,icrbd_field_s); register_union!(hubii_ifdr_u,hubii_ifdr_t,hi_ifdr_value,hi_ifdr_fields); register_union!(iprte_a,iprte_a_t,entry,iprte_fields); register_union!(iprb_u,iprb_t,reg_value,iprb_fields_s); register_union!(icrbp_a,icrbp_a_t,ip_reg,ip_fmt); register_union!(hubii_idsr,hubii_idsr_t,iin_reg,iin_fmt);
pub const IIO_ICRB_ADDR_SHFT:u32=2; pub const IIO_ICRB_ECODE_DERR:u64=0; pub const IIO_ICRB_ECODE_PERR:u64=1; pub const IIO_ICRB_ECODE_WERR:u64=2; pub const IIO_ICRB_ECODE_AERR:u64=3; pub const IIO_ICRB_ECODE_PWERR:u64=4; pub const IIO_ICRB_ECODE_PRERR:u64=5; pub const IIO_ICRB_ECODE_TOUT:u64=6; pub const IIO_ICRB_ECODE_XTERR:u64=7;
pub const IIO_ICRB_XTSIZE_DW:u64=0; pub const IIO_ICRB_XTSIZE_32:u64=1; pub const IIO_ICRB_XTSIZE_128:u64=2; pub const IIO_ICRB_PROC0:u64=0; pub const IIO_ICRB_PROC1:u64=1; pub const IIO_ICRB_GB_REQ:u64=2; pub const IIO_ICRB_IO_REQ:u64=3; pub const IIO_ICRB_IMSGT_XTALK:u64=0; pub const IIO_ICRB_IMSGT_BTE:u64=1; pub const IIO_ICRB_IMSGT_SN0NET:u64=2; pub const IIO_ICRB_IMSGT_CRB:u64=3; pub const IIO_ICRB_INIT_XTALK:u64=0; pub const IIO_ICRB_INIT_BTE0:u64=1; pub const IIO_ICRB_INIT_SN0NET:u64=2; pub const IIO_ICRB_INIT_CRB:u64=3; pub const IIO_ICRB_INIT_BTE1:u64=5;
pub const IIO_ICRB_REQ_DWRD:u64=0; pub const IIO_ICRB_REQ_QCLRD:u64=1; pub const IIO_ICRB_REQ_BLKRD:u64=2; pub const IIO_ICRB_REQ_RSHU:u64=6; pub const IIO_ICRB_REQ_REXU:u64=7; pub const IIO_ICRB_REQ_RDEX:u64=8; pub const IIO_ICRB_REQ_WINC:u64=9; pub const IIO_ICRB_REQ_BWINV:u64=10; pub const IIO_ICRB_REQ_PIORD:u64=11; pub const IIO_ICRB_REQ_PIOWR:u64=12; pub const IIO_ICRB_REQ_PRDM:u64=13; pub const IIO_ICRB_REQ_PWRM:u64=14; pub const IIO_ICRB_REQ_PTPWR:u64=15; pub const IIO_ICRB_REQ_WB:u64=16; pub const IIO_ICRB_REQ_DEX:u64=17;
pub const ICRBN_A_CERR_SHFT:u32=54; pub const ICRBN_A_ERR_MASK:u64=0x3ff; pub const ICRBP_A_CERR_SHFT:u32=54; pub const ICRBP_A_ERR_MASK:u64=0x3ff;
pub const IECLR_BTE1:u64=1<<18; pub const IECLR_BTE0:u64=1<<17; pub const IECLR_CRAZY:u64=1<<16; pub const IECLR_PRB_F:u64=1<<15; pub const IECLR_PRB_E:u64=1<<14; pub const IECLR_PRB_D:u64=1<<13; pub const IECLR_PRB_C:u64=1<<12; pub const IECLR_PRB_B:u64=1<<11; pub const IECLR_PRB_A:u64=1<<10; pub const IECLR_PRB_9:u64=1<<9; pub const IECLR_PRB_8:u64=1<<8; pub const IECLR_PRB_0:u64=1;
pub const IIO_BTE_STAT_0:u64=IIO_IBLS_0; pub const IIO_BTE_SRC_0:u64=IIO_IBSA_0; pub const IIO_BTE_DEST_0:u64=IIO_IBDA_0; pub const IIO_BTE_CTRL_0:u64=IIO_IBCT_0; pub const IIO_BTE_NOTIFY_0:u64=IIO_IBNA_0; pub const IIO_BTE_INT_0:u64=IIO_IBIA_0; pub const IIO_BTE_STAT_1:u64=IIO_IBLS_1; pub const IIO_BTE_SRC_1:u64=IIO_IBSA_1; pub const IIO_BTE_DEST_1:u64=IIO_IBDA_1; pub const IIO_BTE_CTRL_1:u64=IIO_IBCT_1; pub const IIO_BTE_NOTIFY_1:u64=IIO_IBNA_1; pub const IIO_BTE_INT_1:u64=IIO_IBIA_1;
pub const IPRB_MODE_NORMAL:u64=0; pub const IPRB_MODE_COLLECT_A:u64=1; pub const IPRB_MODE_SERVICE_A:u64=2; pub const IPRB_MODE_SERVICE_B:u64=3; pub const IPRTE_ADDRSHFT:u32=3;
pub const IIO_IBLS_0:u64=0x410000; pub const IIO_IBSA_0:u64=0x410008; pub const IIO_IBDA_0:u64=0x410010; pub const IIO_IBCT_0:u64=0x410018; pub const IIO_IBNA_0:u64=0x410020; pub const IIO_IBNR_0:u64=IIO_IBNA_0; pub const IIO_IBIA_0:u64=0x410028; pub const IIO_IBLS_1:u64=0x420000; pub const IIO_IBSA_1:u64=0x420008; pub const IIO_IBDA_1:u64=0x420010; pub const IIO_IBCT_1:u64=0x420018; pub const IIO_IBNA_1:u64=0x420020; pub const IIO_IBNR_1:u64=IIO_IBNA_1; pub const IIO_IBIA_1:u64=0x420028;
pub const IIO_BTE_OFF_0:u64=0; pub const IIO_BTE_OFF_1:u64=IIO_IBLS_1-IIO_IBLS_0; pub const BTEOFF_STAT:u64=0; pub const BTEOFF_SRC:u64=8; pub const BTEOFF_DEST:u64=16; pub const BTEOFF_CTRL:u64=24; pub const BTEOFF_NOTIFY:u64=32; pub const BTEOFF_INT:u64=40;
pub const IIO_IPCR:u64=0x430000; pub const IIO_IPPR:u64=0x430008; pub const IBLS_BUSY:u64=1<<20; pub const IBLS_ERROR_SHFT:u32=16; pub const IBLS_ERROR:u64=1<<16; pub const IBLS_LENGTH_MASK:u64=0xffff; pub const IBCT_POISON:u64=1<<8; pub const IBCT_NOTIFY:u64=1<<4; pub const IBCT_ZFIL_MODE:u64=1; pub const IBIA_LEVEL_SHFT:u32=16; pub const IBIA_LEVEL_MASK:u64=0x7f<<16; pub const IBIA_NODE_ID_SHFT:u32=0; pub const IBIA_NODE_ID_MASK:u64=0x1ff;
pub const HUB_NUM_WIDGET:u64=9; pub const HUB_WIDGET_ID_MIN:u64=8; pub const HUB_WIDGET_ID_MAX:u64=0xf; pub const HUB_WIDGET_PART_NUM:u64=0xc101; pub const MAX_HUBS_PER_XBOW:u64=2; pub const HUBII_XBOW_CREDIT:u64=3; pub const HUBII_XBOW_REV2_CREDIT:u64=4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
