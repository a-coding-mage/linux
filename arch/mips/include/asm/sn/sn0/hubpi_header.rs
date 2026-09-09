/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive.
 * Derived from IRIX <sys/SN/SN0/hubpi.h>, revision 1.28.
 */

// Hub I/O interface register offsets.  Use LOCAL_HUB or REMOTE_HUB to form
// actual addresses.

pub const PI_BASE: u64 = 0x000000;

pub const PI_CPU_PROTECT: u64 = 0x000000;
pub const PI_PROT_OVERRD: u64 = 0x000008;
pub const PI_IO_PROTECT: u64 = 0x000010;
pub const PI_REGION_PRESENT: u64 = 0x000018;
pub const PI_CPU_NUM: u64 = 0x000020;
pub const PI_CALIAS_SIZE: u64 = 0x000028;
pub const PI_MAX_CRB_TIMEOUT: u64 = 0x000030;
pub const PI_CRB_SFACTOR: u64 = 0x000038;

pub const PI_CALIAS_SIZE_0: u64 = 0;
pub const PI_CALIAS_SIZE_4K: u64 = 1;
pub const PI_CALIAS_SIZE_8K: u64 = 2;
pub const PI_CALIAS_SIZE_16K: u64 = 3;
pub const PI_CALIAS_SIZE_32K: u64 = 4;
pub const PI_CALIAS_SIZE_64K: u64 = 5;
pub const PI_CALIAS_SIZE_128K: u64 = 6;
pub const PI_CALIAS_SIZE_256K: u64 = 7;
pub const PI_CALIAS_SIZE_512K: u64 = 8;
pub const PI_CALIAS_SIZE_1M: u64 = 9;
pub const PI_CALIAS_SIZE_2M: u64 = 10;
pub const PI_CALIAS_SIZE_4M: u64 = 11;
pub const PI_CALIAS_SIZE_8M: u64 = 12;
pub const PI_CALIAS_SIZE_16M: u64 = 13;
pub const PI_CALIAS_SIZE_32M: u64 = 14;
pub const PI_CALIAS_SIZE_64M: u64 = 15;

pub const PI_CPU_PRESENT_A: u64 = 0x40;
pub const PI_CPU_PRESENT_B: u64 = 0x48;
pub const PI_CPU_ENABLE_A: u64 = 0x50;
pub const PI_CPU_ENABLE_B: u64 = 0x58;
pub const PI_REPLY_LEVEL: u64 = 0x60;
pub const PI_HARDRESET_BIT: u64 = 0x020068;
pub const PI_NMI_A: u64 = 0x70;
pub const PI_NMI_B: u64 = 0x78;
pub const PI_NMI_OFFSET: u64 = PI_NMI_B - PI_NMI_A;
pub const PI_SOFTRESET: u64 = 0x80;

pub const PI_INT_PEND_MOD: u64 = 0x90;
pub const PI_INT_PEND0: u64 = 0x98;
pub const PI_INT_PEND1: u64 = 0xa0;
pub const PI_INT_MASK0_A: u64 = 0xa8;
pub const PI_INT_MASK1_A: u64 = 0xb0;
pub const PI_INT_MASK0_B: u64 = 0xb8;
pub const PI_INT_MASK1_B: u64 = 0xc0;
pub const PI_INT_MASK_OFFSET: u64 = 0x10;
pub const PI_CC_PEND_SET_A: u64 = 0xc8;
pub const PI_CC_PEND_SET_B: u64 = 0xd0;
pub const PI_CC_PEND_CLR_A: u64 = 0xd8;
pub const PI_CC_PEND_CLR_B: u64 = 0xe0;
pub const PI_CC_MASK: u64 = 0xe8;
pub const PI_INT_SET_OFFSET: u64 = 8;

pub const PI_RT_COUNT: u64 = 0x030100;
pub const PI_RT_COMPARE_A: u64 = 0x108;
pub const PI_RT_COMPARE_B: u64 = 0x110;
pub const PI_PROFILE_COMPARE: u64 = 0x118;
pub const PI_RT_PEND_A: u64 = 0x120;
pub const PI_RT_PEND_B: u64 = 0x128;
pub const PI_PROF_PEND_A: u64 = 0x130;
pub const PI_PROF_PEND_B: u64 = 0x138;
pub const PI_RT_EN_A: u64 = 0x140;
pub const PI_RT_EN_B: u64 = 0x148;
pub const PI_PROF_EN_A: u64 = 0x150;
pub const PI_PROF_EN_B: u64 = 0x158;
pub const PI_RT_LOCAL_CTRL: u64 = 0x160;
pub const PI_RT_FILTER_CTRL: u64 = 0x168;
pub const PI_COUNT_OFFSET: u64 = 8;

pub const PI_BIST_WRITE_DATA: u64 = 0x200;
pub const PI_BIST_READ_DATA: u64 = 0x208;
pub const PI_BIST_COUNT_TARG: u64 = 0x210;
pub const PI_BIST_READY: u64 = 0x218;
pub const PI_BIST_SHIFT_LOAD: u64 = 0x220;
pub const PI_BIST_SHIFT_UNLOAD: u64 = 0x228;
pub const PI_BIST_ENTER_RUN: u64 = 0x230;

pub const PI_GFX_PAGE_A: u64 = 0x300;
pub const PI_GFX_CREDIT_CNTR_A: u64 = 0x308;
pub const PI_GFX_BIAS_A: u64 = 0x310;
pub const PI_GFX_INT_CNTR_A: u64 = 0x318;
pub const PI_GFX_INT_CMP_A: u64 = 0x320;
pub const PI_GFX_PAGE_B: u64 = 0x328;
pub const PI_GFX_CREDIT_CNTR_B: u64 = 0x330;
pub const PI_GFX_BIAS_B: u64 = 0x338;
pub const PI_GFX_INT_CNTR_B: u64 = 0x340;
pub const PI_GFX_INT_CMP_B: u64 = 0x348;
pub const PI_GFX_OFFSET: u64 = PI_GFX_PAGE_B - PI_GFX_PAGE_A;
pub const PI_GFX_PAGE_ENABLE: u64 = 0x0000_1000_0000_0000;

pub const PI_ERR_INT_PEND: u64 = 0x400;
pub const PI_ERR_INT_MASK_A: u64 = 0x408;
pub const PI_ERR_INT_MASK_B: u64 = 0x410;
pub const PI_ERR_STACK_ADDR_A: u64 = 0x418;
pub const PI_ERR_STACK_ADDR_B: u64 = 0x420;
pub const PI_ERR_STACK_SIZE: u64 = 0x428;
pub const PI_ERR_STATUS0_A: u64 = 0x430;
pub const PI_ERR_STATUS0_A_RCLR: u64 = 0x438;
pub const PI_ERR_STATUS1_A: u64 = 0x440;
pub const PI_ERR_STATUS1_A_RCLR: u64 = 0x448;
pub const PI_ERR_STATUS0_B: u64 = 0x450;
pub const PI_ERR_STATUS0_B_RCLR: u64 = 0x458;
pub const PI_ERR_STATUS1_B: u64 = 0x460;
pub const PI_ERR_STATUS1_B_RCLR: u64 = 0x468;
pub const PI_SPOOL_CMP_A: u64 = 0x470;
pub const PI_SPOOL_CMP_B: u64 = 0x478;
pub const PI_CRB_TIMEOUT_A: u64 = 0x480;
pub const PI_CRB_TIMEOUT_B: u64 = 0x488;
pub const PI_SYSAD_ERRCHK_EN: u64 = 0x490;
pub const PI_BAD_CHECK_BIT_A: u64 = 0x498;
pub const PI_BAD_CHECK_BIT_B: u64 = 0x4a0;
pub const PI_NACK_CNT_A: u64 = 0x4a8;
pub const PI_NACK_CNT_B: u64 = 0x4b0;
pub const PI_NACK_CMP: u64 = 0x4b8;
pub const PI_STACKADDR_OFFSET: u64 = PI_ERR_STACK_ADDR_B - PI_ERR_STACK_ADDR_A;
pub const PI_ERRSTAT_OFFSET: u64 = PI_ERR_STATUS0_B - PI_ERR_STATUS0_A;
pub const PI_RDCLR_OFFSET: u64 = PI_ERR_STATUS0_A_RCLR - PI_ERR_STATUS0_A;

pub const PI_ERR_SPOOL_CMP_B: u64 = 0x00000001;
pub const PI_ERR_SPOOL_CMP_A: u64 = 0x00000002;
pub const PI_ERR_SPUR_MSG_B: u64 = 0x00000004;
pub const PI_ERR_SPUR_MSG_A: u64 = 0x00000008;
pub const PI_ERR_WRB_TERR_B: u64 = 0x10;
pub const PI_ERR_WRB_TERR_A: u64 = 0x20;
pub const PI_ERR_WRB_WERR_B: u64 = 0x40;
pub const PI_ERR_WRB_WERR_A: u64 = 0x80;
pub const PI_ERR_SYSSTATE_B: u64 = 0x100;
pub const PI_ERR_SYSSTATE_A: u64 = 0x200;
pub const PI_ERR_SYSAD_DATA_B: u64 = 0x400;
pub const PI_ERR_SYSAD_DATA_A: u64 = 0x800;
pub const PI_ERR_SYSAD_ADDR_B: u64 = 0x1000;
pub const PI_ERR_SYSAD_ADDR_A: u64 = 0x2000;
pub const PI_ERR_SYSCMD_DATA_B: u64 = 0x4000;
pub const PI_ERR_SYSCMD_DATA_A: u64 = 0x8000;
pub const PI_ERR_SYSCMD_ADDR_B: u64 = 0x10000;
pub const PI_ERR_SYSCMD_ADDR_A: u64 = 0x20000;
pub const PI_ERR_BAD_SPOOL_B: u64 = 0x40000;
pub const PI_ERR_BAD_SPOOL_A: u64 = 0x80000;
pub const PI_ERR_UNCAC_UNCORR_B: u64 = 0x100000;
pub const PI_ERR_UNCAC_UNCORR_A: u64 = 0x200000;
pub const PI_ERR_SYSSTATE_TAG_B: u64 = 0x400000;
pub const PI_ERR_SYSSTATE_TAG_A: u64 = 0x800000;
pub const PI_ERR_MD_UNCORR: u64 = 0x1000000;
pub const PI_ERR_CLEAR_ALL_A: u64 = 0x00aaaaaa;
pub const PI_ERR_CLEAR_ALL_B: u64 = 0x00555555;

pub const PI_FATAL_ERR_CPU_A: u64 = PI_ERR_SYSSTATE_TAG_A | PI_ERR_BAD_SPOOL_A | PI_ERR_SYSCMD_ADDR_A | PI_ERR_SYSCMD_DATA_A | PI_ERR_SYSAD_ADDR_A | PI_ERR_SYSAD_DATA_A | PI_ERR_SYSSTATE_A;
pub const PI_MISC_ERR_CPU_A: u64 = PI_ERR_UNCAC_UNCORR_A | PI_ERR_WRB_WERR_A | PI_ERR_WRB_TERR_A | PI_ERR_SPUR_MSG_A | PI_ERR_SPOOL_CMP_A;
pub const PI_FATAL_ERR_CPU_B: u64 = PI_ERR_SYSSTATE_TAG_B | PI_ERR_BAD_SPOOL_B | PI_ERR_SYSCMD_ADDR_B | PI_ERR_SYSCMD_DATA_B | PI_ERR_SYSAD_ADDR_B | PI_ERR_SYSAD_DATA_B | PI_ERR_SYSSTATE_B;
pub const PI_MISC_ERR_CPU_B: u64 = PI_ERR_UNCAC_UNCORR_B | PI_ERR_WRB_WERR_B | PI_ERR_WRB_TERR_B | PI_ERR_SPUR_MSG_B | PI_ERR_SPOOL_CMP_B;
pub const PI_ERR_GENERIC: u64 = PI_ERR_MD_UNCORR;

pub const PI_ERR_ST0_TYPE_MASK: u64 = 0x7;
pub const PI_ERR_ST0_TYPE_SHFT: u64 = 0;
pub const PI_ERR_ST0_REQNUM_MASK: u64 = 0x38;
pub const PI_ERR_ST0_REQNUM_SHFT: u64 = 3;
pub const PI_ERR_ST0_SUPPL_MASK: u64 = 0x1ffc0;
pub const PI_ERR_ST0_SUPPL_SHFT: u64 = 6;
pub const PI_ERR_ST0_CMD_MASK: u64 = 0x1fe0000;
pub const PI_ERR_ST0_CMD_SHFT: u64 = 17;
pub const PI_ERR_ST0_ADDR_MASK: u64 = 0x3ffffffe000000;
pub const PI_ERR_ST0_ADDR_SHFT: u64 = 25;
pub const PI_ERR_ST0_OVERRUN_MASK: u64 = 0x4000000000000000;
pub const PI_ERR_ST0_OVERRUN_SHFT: u64 = 62;
pub const PI_ERR_ST0_VALID_MASK: u64 = 0x8000000000000000;
pub const PI_ERR_ST0_VALID_SHFT: u64 = 63;

pub const PI_ERR_ST1_SPOOL_MASK: u64 = 0x1fffff;
pub const PI_ERR_ST1_SPOOL_SHFT: u64 = 0;
pub const PI_ERR_ST1_TOUTCNT_MASK: u64 = 0x1fe00000;
pub const PI_ERR_ST1_TOUTCNT_SHFT: u64 = 21;
pub const PI_ERR_ST1_INVCNT_MASK: u64 = 0x7fe0000000;
pub const PI_ERR_ST1_INVCNT_SHFT: u64 = 29;
pub const PI_ERR_ST1_CRBNUM_MASK: u64 = 0x3800000000;
pub const PI_ERR_ST1_CRBNUM_SHFT: u64 = 39;
pub const PI_ERR_ST1_WRBRRB_MASK: u64 = 0x4000000000;
pub const PI_ERR_ST1_WRBRRB_SHFT: u64 = 42;
pub const PI_ERR_ST1_CRBSTAT_MASK: u64 = 0x1ff80000000000;
pub const PI_ERR_ST1_CRBSTAT_SHFT: u64 = 43;
pub const PI_ERR_ST1_MSGSRC_MASK: u64 = 0xffe0000000000000;
pub const PI_ERR_ST1_MSGSRC_SHFT: u64 = 53;

pub const PI_ERR_STK_TYPE_MASK: u64 = 3;
pub const PI_ERR_STK_TYPE_SHFT: u64 = 0;
pub const PI_ERR_STK_SUPPL_MASK: u64 = 0x38;
pub const PI_ERR_STK_SUPPL_SHFT: u64 = 3;
pub const PI_ERR_STK_REQNUM_MASK: u64 = 0x1c0;
pub const PI_ERR_STK_REQNUM_SHFT: u64 = 6;
pub const PI_ERR_STK_CRBNUM_MASK: u64 = 0xe00;
pub const PI_ERR_STK_CRBNUM_SHFT: u64 = 9;
pub const PI_ERR_STK_WRBRRB_MASK: u64 = 0x1000;
pub const PI_ERR_STK_WRBRRB_SHFT: u64 = 12;
pub const PI_ERR_STK_CRBSTAT_MASK: u64 = 0x7fe000;
pub const PI_ERR_STK_CRBSTAT_SHFT: u64 = 13;
pub const PI_ERR_STK_CMD_MASK: u64 = 0x7f800000;
pub const PI_ERR_STK_CMD_SHFT: u64 = 23;
pub const PI_ERR_STK_ADDR_MASK: u64 = 0xffffffff80000000;
pub const PI_ERR_STK_ADDR_SHFT: u64 = 31;

pub const PI_ERR_RD_PRERR: u64 = 1;
pub const PI_ERR_RD_DERR: u64 = 2;
pub const PI_ERR_RD_TERR: u64 = 3;
pub const PI_ERR_WR_WERR: u64 = 0;
pub const PI_ERR_WR_PWERR: u64 = 1;
pub const PI_ERR_WR_TERR: u64 = 3;
pub const PI_ERR_RRB: u64 = 0;
pub const PI_ERR_WRB: u64 = 1;
pub const PI_ERR_ANY_CRB: u64 = 2;
pub const ERR_STK_ADDR_SHFT: u64 = 7;
pub const ERR_STAT0_ADDR_SHFT: u64 = 3;
pub const PI_MIN_STACK_SIZE: u64 = 4096;
pub const PI_STACK_SIZE_SHFT: u64 = 12;
#[inline]
pub const fn err_stack_size_bytes(sz: u64) -> u64 { if sz != 0 { PI_MIN_STACK_SIZE << (sz - 1) } else { 0 } }

// C bit-fields are represented by their packed 64-bit storage word; masks and
// shifts above preserve the original field layout and access semantics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ErrStackFormat { pub word: u64 }
#[repr(C)]
pub union PiErrStack { pub pi_stk_word: u64, pub pi_stk_fmt: ErrStackFormat }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ErrStatus0Format { pub word: u64 }
#[repr(C)]
pub union PiErrStat0 { pub pi_stat0_word: u64, pub pi_stat0_fmt: ErrStatus0Format }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ErrStatus1Format { pub word: u64 }
#[repr(C)]
pub union PiErrStat1 { pub pi_stat1_word: u64, pub pi_stat1_fmt: ErrStatus1Format }
pub type PiErrStackT = PiErrStack;
pub type PiErrStat0T = PiErrStat0;
pub type PiErrStat1T = PiErrStat1;
pub type RtcTimeT = u64;

pub const PI_SYSAD_ERRCHK_ECCGEN: u64 = 0x01;
pub const PI_SYSAD_ERRCHK_QUALGEN: u64 = 0x02;
pub const PI_SYSAD_ERRCHK_SADP: u64 = 0x04;
pub const PI_SYSAD_ERRCHK_CMDP: u64 = 0x08;
pub const PI_SYSAD_ERRCHK_STATE: u64 = 0x10;
pub const PI_SYSAD_ERRCHK_QUAL: u64 = 0x20;
pub const PI_SYSAD_CHECK_ALL: u64 = 0x3f;
pub const HUB_IP_PEND0: u64 = 0x0400;
pub const HUB_IP_PEND1_CC: u64 = 0x0800;
pub const HUB_IP_RT: u64 = 0x1000;
pub const HUB_IP_PROF: u64 = 0x2000;
pub const HUB_IP_ERROR: u64 = 0x4000;
pub const HUB_IP_MASK: u64 = 0x7c00;

pub const PRLC_USE_INT_SHFT: u64 = 16;
pub const PRLC_USE_INT_MASK: u64 = 1u64 << 16;
pub const PRLC_USE_INT: u64 = 1u64 << 16;
pub const PRLC_GCLK_SHFT: u64 = 15;
pub const PRLC_GCLK_MASK: u64 = 1u64 << 15;
pub const PRLC_GCLK: u64 = 1u64 << 15;
pub const PRLC_GCLK_COUNT_SHFT: u64 = 8;
pub const PRLC_GCLK_COUNT_MASK: u64 = 0x7f << 8;
pub const PRLC_MAX_COUNT_SHFT: u64 = 1;
pub const PRLC_MAX_COUNT_MASK: u64 = 0x7f << 1;
pub const PRLC_GCLK_EN_SHFT: u64 = 0;
pub const PRLC_GCLK_EN_MASK: u64 = 1;
pub const PRLC_GCLK_EN: u64 = 1;
pub const PI_NACK_CNT_EN_SHFT: u64 = 20;
pub const PI_NACK_CNT_EN_MASK: u64 = 0x100000;
pub const PI_NACK_CNT_MASK: u64 = 0x0fffff;
pub const PI_NACK_CNT_MAX: u64 = 0x0fffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
