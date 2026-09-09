/* Rust translation of hubmd.h. */

pub const CACHE_SLINE_SIZE: usize = 128;
pub const MAX_REGIONS: usize = 64;
pub const MD_PAGE_SIZE: usize = 4096;
pub const MD_PAGE_NUM_SHFT: u32 = 12;

pub const MD_BASE: u64 = 0x200000;
pub const MD_BASE_PERF: u64 = 0x210000;
pub const MD_BASE_JUNK: u64 = 0x220000;
pub const MD_IO_PROTECT: u64 = 0x200000;
pub const MD_IO_PROT_OVRRD: u64 = 0x200008;
pub const MD_HSPEC_PROTECT: u64 = 0x200010;
pub const MD_MEMORY_CONFIG: u64 = 0x200018;
pub const MD_REFRESH_CONTROL: u64 = 0x200020;
pub const MD_FANDOP_CAC_STAT: u64 = 0x200028;
pub const MD_MIG_DIFF_THRESH: u64 = 0x200030;
pub const MD_MIG_VALUE_THRESH: u64 = 0x200038;
pub const MD_MIG_CANDIDATE: u64 = 0x200040;
pub const MD_MIG_CANDIDATE_CLR: u64 = 0x200048;
pub const MD_DIR_ERROR: u64 = 0x200050;
pub const MD_DIR_ERROR_CLR: u64 = 0x200058;
pub const MD_PROTOCOL_ERROR: u64 = 0x200060;
pub const MD_PROTOCOL_ERROR_CLR: u64 = 0x200068;
pub const MD_MEM_ERROR: u64 = 0x200070;
pub const MD_MEM_ERROR_CLR: u64 = 0x200078;
pub const MD_MISC_ERROR: u64 = 0x200080;
pub const MD_MISC_ERROR_CLR: u64 = 0x200088;
pub const MD_MEM_DIMM_INIT: u64 = 0x200090;
pub const MD_DIR_DIMM_INIT: u64 = 0x200098;
pub const MD_MOQ_SIZE: u64 = 0x2000a0;
pub const MD_MLAN_CTL: u64 = 0x2000a8;
pub const MD_PERF_SEL: u64 = 0x210000;
pub const MD_PERF_CNT0: u64 = 0x210010;
pub const MD_PERF_CNT1: u64 = 0x210018;
pub const MD_PERF_CNT2: u64 = 0x210020;
pub const MD_PERF_CNT3: u64 = 0x210028;
pub const MD_PERF_CNT4: u64 = 0x210030;
pub const MD_PERF_CNT5: u64 = 0x210038;

pub const MD_UREG0_0: u64 = 0x220000;
pub const MD_UREG0_1: u64 = 0x220008;
pub const MD_UREG0_2: u64 = 0x220010;
pub const MD_UREG0_3: u64 = 0x220018;
pub const MD_UREG0_4: u64 = 0x220020;
pub const MD_UREG0_5: u64 = 0x220028;
pub const MD_UREG0_6: u64 = 0x220030;
pub const MD_UREG0_7: u64 = 0x220038;
pub const MD_SLOTID_USTAT: u64 = 0x220048;
pub const MD_LED0: u64 = 0x220050;
pub const MD_LED1: u64 = 0x220058;

pub const MD_UREG1_0: u64 = 0x220080;
pub const MD_UREG1_1: u64 = 0x220088;
pub const MD_UREG1_2: u64 = 0x220090;
pub const MD_UREG1_3: u64 = 0x220098;
pub const MD_UREG1_4: u64 = 0x2200a0;
pub const MD_UREG1_5: u64 = 0x2200a8;
pub const MD_UREG1_6: u64 = 0x2200b0;
pub const MD_UREG1_7: u64 = 0x2200b8;
pub const MD_UREG1_8: u64 = 0x2200c0;
pub const MD_UREG1_9: u64 = 0x2200c8;
pub const MD_UREG1_10: u64 = 0x2200d0;
pub const MD_UREG1_11: u64 = 0x2200d8;
pub const MD_UREG1_12: u64 = 0x2200e0;
pub const MD_UREG1_13: u64 = 0x2200e8;
pub const MD_UREG1_14: u64 = 0x2200f0;
pub const MD_UREG1_15: u64 = 0x2200f8;

// CONFIG_SGI_SN_N_MODE selects four banks in N mode; M mode has eight.
pub const MD_MEM_BANKS: u64 = 8;
pub const MD_SIZE_EMPTY: u64 = 0;
pub const MD_SIZE_8MB: u64 = 1;
pub const MD_SIZE_16MB: u64 = 2;
pub const MD_SIZE_32MB: u64 = 3;
pub const MD_SIZE_64MB: u64 = 4;
pub const MD_SIZE_128MB: u64 = 5;
pub const MD_SIZE_256MB: u64 = 6;
pub const MD_SIZE_512MB: u64 = 7;
pub const MD_SIZE_1GB: u64 = 8;
pub const MD_SIZE_2GB: u64 = 9;
pub const MD_SIZE_4GB: u64 = 10;
#[inline] pub const fn MD_SIZE_BYTES(size: u64) -> u64 { if size == 0 { 0 } else { 0x400000u64 << size } }
#[inline] pub const fn MD_SIZE_MBYTES(size: u64) -> u64 { if size == 0 { 0 } else { 4u64 << size } }

pub const MMC_FPROM_CYC_SHFT:u32=49; pub const MMC_FPROM_CYC_MASK:u64=31u64<<49;
pub const MMC_FPROM_WR_SHFT:u32=44; pub const MMC_FPROM_WR_MASK:u64=31u64<<44;
pub const MMC_UCTLR_CYC_SHFT:u32=39; pub const MMC_UCTLR_CYC_MASK:u64=31u64<<39;
pub const MMC_UCTLR_WR_SHFT:u32=34; pub const MMC_UCTLR_WR_MASK:u64=31u64<<34;
pub const MMC_DIMM0_SEL_SHFT:u32=32; pub const MMC_DIMM0_SEL_MASK:u64=3u64<<32;
pub const MMC_IO_PROT_EN_SHFT:u32=31; pub const MMC_IO_PROT_EN_MASK:u64=1u64<<31; pub const MMC_IO_PROT:u64=1u64<<31;
pub const MMC_ARB_MLSS_SHFT:u32=30; pub const MMC_ARB_MLSS_MASK:u64=1u64<<30; pub const MMC_ARB_MLSS:u64=1u64<<30;
pub const MMC_IGNORE_ECC_SHFT:u32=29; pub const MMC_IGNORE_ECC_MASK:u64=1u64<<29; pub const MMC_IGNORE_ECC:u64=1u64<<29;
pub const MMC_DIR_PREMIUM_SHFT:u32=28; pub const MMC_DIR_PREMIUM_MASK:u64=1u64<<28; pub const MMC_DIR_PREMIUM:u64=1u64<<28;
pub const MMC_REPLY_GUAR_SHFT:u32=24; pub const MMC_REPLY_GUAR_MASK:u64=15u64<<24;
#[inline] pub const fn MMC_BANK_SHFT(b:u64)->u32 {(b*3) as u32}
#[inline] pub const fn MMC_BANK_MASK(b:u64)->u64 {7u64 << MMC_BANK_SHFT(b)}
pub const MMC_BANK_ALL_MASK:u64=0xffffff;
pub const MMC_RESET_DEFAULTS:u64=(0x0f<<49)|(0x07<<44)|(0x1f<<39)|(0x0f<<34)|MMC_IGNORE_ECC|MMC_DIR_PREMIUM|(0x0f<<24)|MMC_BANK_ALL_MASK;

pub const MRC_ENABLE_SHFT:u32=63; pub const MRC_ENABLE_MASK:u64=1u64<<63; pub const MRC_ENABLE:u64=1u64<<63;
pub const MRC_COUNTER_SHFT:u32=12; pub const MRC_COUNTER_MASK:u64=0xfffu64<<12; pub const MRC_CNT_THRESH_MASK:u64=0xfff; pub const MRC_RESET_DEFAULTS:u64=0x400;
pub const MDI_SELECT_SHFT:u32=32; pub const MDI_SELECT_MASK:u64=0xfu64<<32; pub const MDI_DIMM_MODE_MASK:u64=0xfff;
pub const MMS_RP_SIZE_SHFT:u32=8; pub const MMS_RP_SIZE_MASK:u64=0x3fu64<<8; pub const MMS_RQ_SIZE_SHFT:u32=0; pub const MMS_RQ_SIZE_MASK:u64=0x1f; pub const MMS_RESET_DEFAULTS:u64=0x3212;
pub const MFC_VALID_SHFT:u32=63; pub const MFC_VALID_MASK:u64=1u64<<63; pub const MFC_VALID:u64=1u64<<63; pub const MFC_ADDR_SHFT:u32=6; pub const MFC_ADDR_MASK:u64=0x3ffffff;
pub const MLAN_PHI1_SHFT:u32=27; pub const MLAN_PHI1_MASK:u64=0x7fu64<<27; pub const MLAN_PHI0_SHFT:u32=20; pub const MLAN_PHI0_MASK:u64=0x7fu64<<27; pub const MLAN_PULSE_SHFT:u32=10; pub const MLAN_PULSE_MASK:u64=0x3ffu64<<10; pub const MLAN_SAMPLE_SHFT:u32=2; pub const MLAN_SAMPLE_MASK:u64=0xffu64<<2; pub const MLAN_DONE_SHFT:u32=1; pub const MLAN_DONE_MASK:u64=2; pub const MLAN_DONE:u64=2; pub const MLAN_RD_DATA:u64=1; pub const MLAN_RESET_DEFAULTS:u64=(0x31<<27)|(0x31<<20);

pub const MSU_CORECLK_TST_SHFT:u32=7; pub const MSU_CORECLK_TST_MASK:u64=1<<7; pub const MSU_CORECLK_TST:u64=1<<7; pub const MSU_CORECLK_SHFT:u32=6; pub const MSU_CORECLK_MASK:u64=1<<6; pub const MSU_CORECLK:u64=1<<6; pub const MSU_NETSYNC_SHFT:u32=5; pub const MSU_NETSYNC_MASK:u64=1<<5; pub const MSU_NETSYNC:u64=1<<5; pub const MSU_FPROMRDY_SHFT:u32=4; pub const MSU_FPROMRDY_MASK:u64=1<<4; pub const MSU_FPROMRDY:u64=1<<4; pub const MSU_I2CINTR_SHFT:u32=3; pub const MSU_I2CINTR_MASK:u64=1<<3; pub const MSU_I2CINTR:u64=1<<3; pub const MSU_SLOTID_MASK:u64=0xff; pub const MSU_SN0_SLOTID_SHFT:u32=0; pub const MSU_SN0_SLOTID_MASK:u64=7; pub const MSU_SN00_SLOTID_SHFT:u32=7; pub const MSU_SN00_SLOTID_MASK:u64=0x80; pub const MSU_PIMM_PSC_SHFT:u32=4; pub const MSU_PIMM_PSC_MASK:u64=0xf<<4;

pub const MD_MIG_DIFF_THRES_VALID_MASK:u64=1<<63; pub const MD_MIG_DIFF_THRES_VALID_SHFT:u32=63; pub const MD_MIG_DIFF_THRES_VALUE_MASK:u64=0xfffff;
pub const MD_MIG_VALUE_THRES_VALID_MASK:u64=1<<63; pub const MD_MIG_VALUE_THRES_VALID_SHFT:u32=63; pub const MD_MIG_VALUE_THRES_VALUE_MASK:u64=0xfffff;
pub const MD_MIG_CANDIDATE_VALID_MASK:u64=1<<63; pub const MD_MIG_CANDIDATE_VALID_SHFT:u32=63; pub const MD_MIG_CANDIDATE_TYPE_MASK:u64=1<<30; pub const MD_MIG_CANDIDATE_TYPE_SHFT:u32=30; pub const MD_MIG_CANDIDATE_OVERRUN_MASK:u64=1<<29; pub const MD_MIG_CANDIDATE_OVERRUN_SHFT:u32=29; pub const MD_MIG_CANDIDATE_INITIATOR_MASK:u64=0x7ff<<18; pub const MD_MIG_CANDIDATE_INITIATOR_SHFT:u32=18; pub const MD_MIG_CANDIDATE_NODEID_MASK:u64=0x1ff<<20; pub const MD_MIG_CANDIDATE_NODEID_SHFT:u32=20; pub const MD_MIG_CANDIDATE_ADDR_MASK:u64=0x3ffff; pub const MD_MIG_CANDIDATE_ADDR_SHFT:u32=14;
pub const MD_BANK_SHFT:u32=29; pub const MD_BANK_MASK:u64=7<<29; pub const MD_BANK_SIZE:u64=1<<29; #[inline] pub const fn MD_BANK_OFFSET(b:u64)->u64 {b<<MD_BANK_SHFT}
pub const MD_DIR_SHARED:u64=0; pub const MD_DIR_POISONED:u64=1; pub const MD_DIR_EXCLUSIVE:u64=2; pub const MD_DIR_BUSY_SHARED:u64=3; pub const MD_DIR_BUSY_EXCL:u64=4; pub const MD_DIR_WAIT:u64=5; pub const MD_DIR_UNOWNED:u64=7; pub const MD_DIR_FORCE_ECC:u64=1<<63;

pub const MD_PDIR_MASK:u64=0xffffffffffff; pub const MD_PDIR_ECC_SHFT:u32=0; pub const MD_PDIR_ECC_MASK:u64=0x7f; pub const MD_PDIR_PRIO_SHFT:u32=8; pub const MD_PDIR_PRIO_MASK:u64=0xf<<8; pub const MD_PDIR_AX_SHFT:u32=7; pub const MD_PDIR_AX_MASK:u64=1<<7; pub const MD_PDIR_AX:u64=1<<7; pub const MD_PDIR_FINE_SHFT:u32=12; pub const MD_PDIR_FINE_MASK:u64=1<<12; pub const MD_PDIR_FINE:u64=1<<12; pub const MD_PDIR_OCT_SHFT:u32=13; pub const MD_PDIR_OCT_MASK:u64=7<<13; pub const MD_PDIR_STATE_SHFT:u32=13; pub const MD_PDIR_STATE_MASK:u64=7<<13; pub const MD_PDIR_ONECNT_SHFT:u32=16; pub const MD_PDIR_ONECNT_MASK:u64=0x3f<<16; pub const MD_PDIR_PTR_SHFT:u32=22; pub const MD_PDIR_PTR_MASK:u64=0x7ff<<22; pub const MD_PDIR_VECMSB_SHFT:u32=22; pub const MD_PDIR_VECMSB_BITMASK:u64=0x3ffffff; pub const MD_PDIR_VECMSB_BITSHFT:u32=27; pub const MD_PDIR_VECMSB_MASK:u64=MD_PDIR_VECMSB_BITMASK<<22; pub const MD_PDIR_CWOFF_SHFT:u32=7; pub const MD_PDIR_CWOFF_MASK:u64=7<<7; pub const MD_PDIR_VECLSB_SHFT:u32=10; pub const MD_PDIR_VECLSB_BITMASK:u64=0x3fffffffff; pub const MD_PDIR_VECLSB_BITSHFT:u32=0; pub const MD_PDIR_VECLSB_MASK:u64=MD_PDIR_VECLSB_BITMASK<<10;
pub const MD_PDIR_INIT_LO:u64=(MD_DIR_UNOWNED<<MD_PDIR_STATE_SHFT)|MD_PDIR_AX; pub const MD_PDIR_INIT_HI:u64=0;
pub const MD_SDIR_MASK:u64=0xffff; pub const MD_SDIR_ECC_SHFT:u32=0; pub const MD_SDIR_ECC_MASK:u64=0x1f; pub const MD_SDIR_PRIO_SHFT:u32=6; pub const MD_SDIR_PRIO_MASK:u64=1<<6; pub const MD_SDIR_AX_SHFT:u32=5; pub const MD_SDIR_AX_MASK:u64=1<<5; pub const MD_SDIR_AX:u64=1<<5; pub const MD_SDIR_STATE_SHFT:u32=7; pub const MD_SDIR_STATE_MASK:u64=7<<7; pub const MD_SDIR_PTR_SHFT:u32=10; pub const MD_SDIR_PTR_MASK:u64=0x3f<<10; pub const MD_SDIR_CWOFF_SHFT:u32=5; pub const MD_SDIR_CWOFF_MASK:u64=7<<5; pub const MD_SDIR_VECMSB_SHFT:u32=11; pub const MD_SDIR_VECMSB_BITMASK:u64=0x1f; pub const MD_SDIR_VECMSB_BITSHFT:u32=7; pub const MD_SDIR_VECMSB_MASK:u64=MD_SDIR_VECMSB_BITMASK<<11; pub const MD_SDIR_VECLSB_SHFT:u32=5; pub const MD_SDIR_VECLSB_BITMASK:u64=0x7ff; pub const MD_SDIR_VECLSB_BITSHFT:u32=0; pub const MD_SDIR_VECLSB_MASK:u64=MD_SDIR_VECLSB_BITMASK<<5;
pub const MD_SDIR_INIT_LO:u64=(MD_DIR_UNOWNED<<MD_SDIR_STATE_SHFT)|MD_SDIR_AX; pub const MD_SDIR_INIT_HI:u64=0;
pub const MD_PROT_RW:u64=6; pub const MD_PROT_RO:u64=3; pub const MD_PROT_NO:u64=0; pub const MD_PROT_BAD:u64=5; pub const MD_PPROT_SHFT:u32=0; pub const MD_PPROT_MASK:u64=7; pub const MD_PPROT_MIGMD_SHFT:u32=3; pub const MD_PPROT_MIGMD_MASK:u64=3<<3; pub const MD_PPROT_REFCNT_SHFT:u32=5; pub const MD_PPROT_REFCNT_WIDTH:u64=0x7ffff; pub const MD_PPROT_REFCNT_MASK:u64=MD_PPROT_REFCNT_WIDTH<<5; pub const MD_PPROT_IO_SHFT:u32=45; pub const MD_PPROT_IO_MASK:u64=7<<45; pub const MD_SPROT_SHFT:u32=0; pub const MD_SPROT_MASK:u64=7; pub const MD_SPROT_MIGMD_SHFT:u32=3; pub const MD_SPROT_MIGMD_MASK:u64=3<<3; pub const MD_SPROT_REFCNT_SHFT:u32=5; pub const MD_SPROT_REFCNT_WIDTH:u64=0x7ff; pub const MD_SPROT_REFCNT_MASK:u64=MD_SPROT_REFCNT_WIDTH<<5; pub const MD_PROT_MIGMD_IREL:u64=3<<3; pub const MD_PROT_MIGMD_IABS:u64=2<<3; pub const MD_PROT_MIGMD_PREL:u64=1<<3; pub const MD_PROT_MIGMD_OFF:u64=0;
pub const MD_PDIR_INIT_PROT:u64=(MD_PROT_RW<<MD_PPROT_IO_SHFT)|(MD_PROT_RW<<MD_PPROT_SHFT); pub const MD_SDIR_INIT_PROT:u64=MD_PROT_RW<<MD_SPROT_SHFT;

#[inline] pub const fn MD_MIG_CANDIDATE_HWPFN(value:u64)->u64 { value & MD_MIG_CANDIDATE_ADDR_MASK }
#[inline] pub const fn MD_MIG_CANDIDATE_NODEID(value:u64)->u64 { (value & MD_MIG_CANDIDATE_NODEID_MASK)>>MD_MIG_CANDIDATE_NODEID_SHFT }
#[inline] pub const fn MD_MIG_CANDIDATE_TYPE(value:u64)->u64 { (value & MD_MIG_CANDIDATE_TYPE_MASK)>>MD_MIG_CANDIDATE_TYPE_SHFT }
#[inline] pub const fn MD_MIG_CANDIDATE_VALID(value:u64)->u64 { (value & MD_MIG_CANDIDATE_VALID_MASK)>>MD_MIG_CANDIDATE_VALID_SHFT }
#[inline] pub const fn MD_PPROT_REFCNT_GET(value:u64)->u64 { (value&MD_PPROT_REFCNT_MASK)>>MD_PPROT_REFCNT_SHFT }
#[inline] pub const fn MD_PPROT_MIGMD_GET(value:u64)->u64 { (value&MD_PPROT_MIGMD_MASK)>>MD_PPROT_MIGMD_SHFT }
#[inline] pub const fn MD_SPROT_REFCNT_GET(value:u64)->u64 { (value&MD_SPROT_REFCNT_MASK)>>MD_SPROT_REFCNT_SHFT }
#[inline] pub const fn MD_SPROT_MIGMD_GET(value:u64)->u64 { (value&MD_SPROT_MIGMD_MASK)>>MD_SPROT_MIGMD_SHFT }

#[repr(C)] #[derive(Copy, Clone)] pub struct dir_error_reg { pub value: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union md_dir_error { pub derr_reg:u64, pub derr_fmt:dir_error_reg }
#[repr(C)] #[derive(Copy, Clone)] pub struct mem_error_reg { pub value:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union md_mem_error { pub merr_reg:u64, pub merr_fmt:mem_error_reg }
#[repr(C)] #[derive(Copy, Clone)] pub struct proto_error_reg { pub value:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union md_proto_error { pub perr_reg:u64, pub perr_fmt:proto_error_reg }
#[repr(C)] #[derive(Copy, Clone)] pub struct md_sdir_high_fmt { pub sd_hi_bvec:u16, pub sd_hi_ecc:u16 }
#[repr(C)] #[derive(Copy, Clone)] pub union md_sdir_high { pub sd_hi_val:u16, pub sd_hi_fmt:md_sdir_high_fmt }
#[repr(C)] #[derive(Copy, Clone)] pub struct md_sdir_low_shared_fmt { pub value:u16 }
#[repr(C)] #[derive(Copy, Clone)] pub struct md_sdir_low_exclusive_fmt { pub value:u16 }
#[repr(C)] #[derive(Copy, Clone)] pub union md_sdir_low { pub sd_lo_val:u16, pub sde_lo_fmt:md_sdir_low_exclusive_fmt, pub sds_lo_fmt:md_sdir_low_shared_fmt }
#[repr(C)] #[derive(Copy, Clone)] pub struct md_pdir_high_fmt { pub value:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union md_pdir_high { pub pd_hi_val:u64, pub pd_hi_fmt:md_pdir_high_fmt }
#[repr(C)] #[derive(Copy, Clone)] pub struct md_pdir_low_shared_fmt { pub value:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct md_pdir_low_exclusive_fmt { pub value:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union md_pdir_low { pub pd_lo_val:u64, pub pde_lo_fmt:md_pdir_low_exclusive_fmt, pub pds_lo_fmt:md_pdir_low_shared_fmt }
#[repr(C)] #[derive(Copy, Clone)] pub union md_dir_high { pub md_sdir_high:md_sdir_high, pub md_pdir_high:md_pdir_high }
#[repr(C)] #[derive(Copy, Clone)] pub union md_dir_low { pub md_sdir_low:md_sdir_low, pub md_pdir_low:md_pdir_low }
#[repr(C)] #[derive(Copy, Clone)] pub struct bddir_entry { pub md_dir_low:md_dir_low, pub md_dir_high:md_dir_high }
#[repr(C)] #[derive(Copy, Clone)] pub struct dir_mem_entry { pub prcpf:[u64;MAX_REGIONS], pub directory_words:[bddir_entry;MD_PAGE_SIZE/CACHE_SLINE_SIZE] }
#[repr(C)] #[derive(Copy, Clone)] pub struct md_perf_sel_bits { pub value:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union md_perf_sel { pub perf_sel_reg:u64, pub perf_sel_bits:md_perf_sel_bits }
#[repr(C)] #[derive(Copy, Clone)] pub struct md_perf_cnt_bits { pub value:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union md_perf_cnt { pub perf_cnt:u64, pub perf_cnt_bits:md_perf_cnt_bits }

pub const DIR_ERROR_VALID_MASK:u64=0xe000000000000000; pub const DIR_ERROR_VALID_SHFT:u32=61; pub const DIR_ERROR_VALID_UCE:u64=0x8000000000000000; pub const DIR_ERROR_VALID_AE:u64=0x4000000000000000; pub const DIR_ERROR_VALID_CE:u64=0x2000000000000000;
pub const MEM_ERROR_VALID_MASK:u64=0xc000000000000000; pub const MEM_ERROR_VALID_SHFT:u32=62; pub const MEM_ERROR_VALID_UCE:u64=0x8000000000000000; pub const MEM_ERROR_VALID_CE:u64=0x4000000000000000; pub const PROTO_ERROR_VALID_MASK:u64=0x8000000000000000; pub const MISC_ERROR_VALID_MASK:u64=0x3ff;
pub const DIR_ERR_HSPEC_MASK:u64=0x3ffffff8; pub const ERROR_HSPEC_MASK:u64=0x3ffffff8; pub const ERROR_HSPEC_SHFT:u32=3; pub const ERROR_ADDR_MASK:u64=0xfffffff8; pub const ERROR_ADDR_SHFT:u32=3;
pub const MMCE_VALID_MASK:u64=0x3ff; pub const MMCE_ILL_MSG_SHFT:u32=8; pub const MMCE_ILL_MSG_MASK:u64=3<<8; pub const MMCE_ILL_REV_SHFT:u32=6; pub const MMCE_ILL_REV_MASK:u64=3<<6; pub const MMCE_LONG_PACK_SHFT:u32=4; pub const MMCE_LONG_PACK_MASK:u64=3<<4; pub const MMCE_SHORT_PACK_SHFT:u32=2; pub const MMCE_SHORT_PACK_MASK:u64=3<<2; pub const MMCE_BAD_DATA_SHFT:u32=0; pub const MMCE_BAD_DATA_MASK:u64=3;
pub const MD_PERF_COUNTERS:u64=6; pub const MD_PERF_SETS:u64=6; pub const MEM_DIMM_MASK:u64=0xe0000000; pub const MEM_DIMM_SHFT:u32=29;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
