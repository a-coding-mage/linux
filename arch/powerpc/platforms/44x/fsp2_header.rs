// Translated from fsp2.h. The original declarations are active under __KERNEL__.

pub const DCRN_CMU_ADDR: u32 = 0x00C;
pub const DCRN_CMU_DATA: u32 = 0x00D;

// PLB4 Arbiter
pub const DCRN_PLB4_PCBI: u32 = 0x010;
pub const DCRN_PLB4_P0ACR: u32 = 0x011;
pub const DCRN_PLB4_P0ESRL: u32 = 0x012;
pub const DCRN_PLB4_P0ESRH: u32 = 0x013;
pub const DCRN_PLB4_P0EARL: u32 = 0x014;
pub const DCRN_PLB4_P0EARH: u32 = 0x015;
pub const DCRN_PLB4_P0ESRLS: u32 = 0x016;
pub const DCRN_PLB4_P0ESRHS: u32 = 0x017;
pub const DCRN_PLB4_PCBC: u32 = 0x018;
pub const DCRN_PLB4_P1ACR: u32 = 0x019;
pub const DCRN_PLB4_P1ESRL: u32 = 0x01A;
pub const DCRN_PLB4_P1ESRH: u32 = 0x01B;
pub const DCRN_PLB4_P1EARL: u32 = 0x01C;
pub const DCRN_PLB4_P1EARH: u32 = 0x01D;
pub const DCRN_PLB4_P1ESRLS: u32 = 0x01E;
pub const DCRN_PLB4_P1ESRHS: u32 = 0x01F;

pub const DCRN_PLB4OPB0_BASE: u32 = 0x020;
pub const DCRN_PLB4OPB1_BASE: u32 = 0x030;
pub const DCRN_PLB4OPB2_BASE: u32 = 0x040;
pub const DCRN_PLB4OPB3_BASE: u32 = 0x050;
pub const PLB4OPB_GESR0: u32 = 0x0;
pub const PLB4OPB_GEAR: u32 = 0x2;
pub const PLB4OPB_GEARU: u32 = 0x3;
pub const PLB4OPB_GESR1: u32 = 0x4;
pub const PLB4OPB_GESR2: u32 = 0xC;

pub const DCRN_PLB4AHB_BASE: u32 = 0x400;
pub const DCRN_PLB4AHB_SEUAR: u32 = DCRN_PLB4AHB_BASE + 1;
pub const DCRN_PLB4AHB_SELAR: u32 = DCRN_PLB4AHB_BASE + 2;
pub const DCRN_PLB4AHB_ESR: u32 = DCRN_PLB4AHB_BASE + 3;
pub const DCRN_AHBPLB4_ESR: u32 = DCRN_PLB4AHB_BASE + 8;
pub const DCRN_AHBPLB4_EAR: u32 = DCRN_PLB4AHB_BASE + 9;

pub const DCRN_PLB6_BASE: u32 = 0x11111300;
pub const DCRN_PLB6_CR0: u32 = DCRN_PLB6_BASE;
pub const DCRN_PLB6_ERR: u32 = DCRN_PLB6_BASE + 0x0B;
pub const DCRN_PLB6_HD: u32 = DCRN_PLB6_BASE + 0x0E;
pub const DCRN_PLB6_SHD: u32 = DCRN_PLB6_BASE + 0x10;
pub const DCRN_PLB4PLB6_BASE: u32 = 0x11111320;
pub const DCRN_PLB4PLB6_ESR: u32 = DCRN_PLB4PLB6_BASE + 1;
pub const DCRN_PLB4PLB6_EARH: u32 = DCRN_PLB4PLB6_BASE + 3;
pub const DCRN_PLB4PLB6_EARL: u32 = DCRN_PLB4PLB6_BASE + 4;
pub const DCRN_PLB6PLB4_BASE: u32 = 0x11111350;
pub const DCRN_PLB6PLB4_ESR: u32 = DCRN_PLB6PLB4_BASE + 1;
pub const DCRN_PLB6PLB4_EARH: u32 = DCRN_PLB6PLB4_BASE + 3;
pub const DCRN_PLB6PLB4_EARL: u32 = DCRN_PLB6PLB4_BASE + 4;
pub const DCRN_PLB6MCIF_BASE: u32 = 0x11111380;
pub const DCRN_PLB6MCIF_BESR0: u32 = DCRN_PLB6MCIF_BASE;
pub const DCRN_PLB6MCIF_BESR1: u32 = DCRN_PLB6MCIF_BASE + 1;
pub const DCRN_PLB6MCIF_BEARL: u32 = DCRN_PLB6MCIF_BASE + 2;
pub const DCRN_PLB6MCIF_BEARH: u32 = DCRN_PLB6MCIF_BASE + 3;

pub const DCRN_CONF_BASE: u32 = 0x11111400;
pub const DCRN_CONF_FIR_RWC: u32 = DCRN_CONF_BASE + 0x3A;
pub const DCRN_CONF_EIR_RS: u32 = DCRN_CONF_BASE + 0x3E;
pub const DCRN_CONF_RPERR0: u32 = DCRN_CONF_BASE + 0x4D;
pub const DCRN_CONF_RPERR1: u32 = DCRN_CONF_BASE + 0x4E;
pub const DCRN_L2CDCRAI: u32 = 0x11111100;
pub const DCRN_L2CDCRDI: u32 = 0x11111104;

pub const L2MCK: u32 = 0x120; pub const L2MCKEN: u32 = 0x130;
pub const L2INT: u32 = 0x150; pub const L2INTEN: u32 = 0x160;
pub const L2LOG0: u32 = 0x180; pub const L2LOG1: u32 = 0x184;
pub const L2LOG2: u32 = 0x188; pub const L2LOG3: u32 = 0x18C;
pub const L2LOG4: u32 = 0x190; pub const L2LOG5: u32 = 0x194;
pub const L2PLBSTAT0: u32 = 0x300; pub const L2PLBSTAT1: u32 = 0x304;
pub const L2PLBMCKEN0: u32 = 0x330; pub const L2PLBMCKEN1: u32 = 0x334;
pub const L2PLBINTEN0: u32 = 0x360; pub const L2PLBINTEN1: u32 = 0x364;
pub const L2ARRSTAT0: u32 = 0x500; pub const L2ARRSTAT1: u32 = 0x504; pub const L2ARRSTAT2: u32 = 0x508;
pub const L2ARRMCKEN0: u32 = 0x530; pub const L2ARRMCKEN1: u32 = 0x534; pub const L2ARRMCKEN2: u32 = 0x538;
pub const L2ARRINTEN0: u32 = 0x560; pub const L2ARRINTEN1: u32 = 0x564; pub const L2ARRINTEN2: u32 = 0x568;
pub const L2CPUSTAT: u32 = 0x700; pub const L2CPUMCKEN: u32 = 0x730; pub const L2CPUINTEN: u32 = 0x760;
pub const L2RACSTAT0: u32 = 0x900; pub const L2RACMCKEN0: u32 = 0x930; pub const L2RACINTEN0: u32 = 0x960;
pub const L2WACSTAT0: u32 = 0xD00; pub const L2WACSTAT1: u32 = 0xD04; pub const L2WACSTAT2: u32 = 0xD08;
pub const L2WACMCKEN0: u32 = 0xD30; pub const L2WACMCKEN1: u32 = 0xD34; pub const L2WACMCKEN2: u32 = 0xD38;
pub const L2WACINTEN0: u32 = 0xD60; pub const L2WACINTEN1: u32 = 0xD64; pub const L2WACINTEN2: u32 = 0xD68;
pub const L2WDFSTAT: u32 = 0xF00; pub const L2WDFMCKEN: u32 = 0xF30; pub const L2WDFINTEN: u32 = 0xF60;

pub const DCRN_DDR34_BASE: u32 = 0x11120000;
pub const DCRN_DDR34_MCSTAT: u32 = 0x10; pub const DCRN_DDR34_MCOPT1: u32 = 0x20; pub const DCRN_DDR34_MCOPT2: u32 = 0x21;
pub const DCRN_DDR34_PHYSTAT: u32 = 0x32; pub const DCRN_DDR34_CFGR0: u32 = 0x40; pub const DCRN_DDR34_CFGR1: u32 = 0x41;
pub const DCRN_DDR34_CFGR2: u32 = 0x42; pub const DCRN_DDR34_CFGR3: u32 = 0x43; pub const DCRN_DDR34_SCRUB_CNTL: u32 = 0xAA;
pub const DCRN_DDR34_SCRUB_INT: u32 = 0xAB; pub const DCRN_DDR34_SCRUB_START_ADDR: u32 = 0xB0; pub const DCRN_DDR34_SCRUB_END_ADDR: u32 = 0xD0;
pub const DCRN_DDR34_ECCERR_ADDR_PORT0: u32 = 0xE0; pub const DCRN_DDR34_ECCERR_ADDR_PORT1: u32 = 0xE1; pub const DCRN_DDR34_ECCERR_ADDR_PORT2: u32 = 0xE2; pub const DCRN_DDR34_ECCERR_ADDR_PORT3: u32 = 0xE3;
pub const DCRN_DDR34_ECCERR_COUNT_PORT0: u32 = 0xE4; pub const DCRN_DDR34_ECCERR_COUNT_PORT1: u32 = 0xE5; pub const DCRN_DDR34_ECCERR_COUNT_PORT2: u32 = 0xE6; pub const DCRN_DDR34_ECCERR_COUNT_PORT3: u32 = 0xE7;
pub const DCRN_DDR34_ECCERR_PORT0: u32 = 0xF0; pub const DCRN_DDR34_ECCERR_PORT1: u32 = 0xF2; pub const DCRN_DDR34_ECCERR_PORT2: u32 = 0xF4; pub const DCRN_DDR34_ECCERR_PORT3: u32 = 0xF6;
pub const DCRN_DDR34_ECC_CHECK_PORT0: u32 = 0xF8; pub const DCRN_DDR34_ECC_CHECK_PORT1: u32 = 0xF9; pub const DCRN_DDR34_ECC_CHECK_PORT2: u32 = 0xF9; pub const DCRN_DDR34_ECC_CHECK_PORT3: u32 = 0xFB;
pub const DDR34_SCRUB_CNTL_STOP: u32 = 0x00000000; pub const DDR34_SCRUB_CNTL_SCRUB: u32 = 0x80000000; pub const DDR34_SCRUB_CNTL_UE_STOP: u32 = 0x20000000; pub const DDR34_SCRUB_CNTL_CE_STOP: u32 = 0x10000000; pub const DDR34_SCRUB_CNTL_RANK_EN: u32 = 0x00008000;

pub const DCRN_CW_BASE: u32 = 0x11111800;
pub const DCRN_CW_MCER0: u32 = 0x00; pub const DCRN_CW_MCER1: u32 = 0x01; pub const DCRN_CW_MCER_AND0: u32 = 0x02; pub const DCRN_CW_MCER_AND1: u32 = 0x03;
pub const DCRN_CW_MCER_OR0: u32 = 0x04; pub const DCRN_CW_MCER_OR1: u32 = 0x05; pub const DCRN_CW_MCER_MASK0: u32 = 0x06; pub const DCRN_CW_MCER_MASK1: u32 = 0x07;
pub const DCRN_CW_MCER_MASK_AND0: u32 = 0x08; pub const DCRN_CW_MCER_MASK_AND1: u32 = 0x09; pub const DCRN_CW_MCER_MASK_OR0: u32 = 0x0A; pub const DCRN_CW_MCER_MASK_OR1: u32 = 0x0B;
pub const DCRN_CW_MCER_ACTION0: u32 = 0x0C; pub const DCRN_CW_MCER_ACTION1: u32 = 0x0D; pub const DCRN_CW_MCER_WOF0: u32 = 0x0E; pub const DCRN_CW_MCER_WOF1: u32 = 0x0F;
pub const DCRN_CW_LFIR: u32 = 0x10; pub const DCRN_CW_LFIR_AND: u32 = 0x11; pub const DCRN_CW_LFIR_OR: u32 = 0x12; pub const DCRN_CW_LFIR_MASK: u32 = 0x13; pub const DCRN_CW_LFIR_MASK_AND: u32 = 0x14; pub const DCRN_CW_LFIR_MASK_OR: u32 = 0x15;
pub const CW_MCER0_MEM_CE: u32 = 0x00020000;

// CMU addresses
pub const CMUN_CRCS: u32 = 0x00; pub const CMUN_CONFFIR0: u32 = 0x20; pub const CMUN_CONFFIR1: u32 = 0x21; pub const CMUN_CONFFIR2: u32 = 0x22; pub const CMUN_CONFFIR3: u32 = 0x23;
pub const CMUN_URCR3_RS: u32 = 0x24; pub const CMUN_URCR3_C: u32 = 0x25; pub const CMUN_URCR3_P: u32 = 0x26; pub const CMUN_PW0: u32 = 0x2C;
pub const CMUN_URCR0_P: u32 = 0x2D; pub const CMUN_URCR1_P: u32 = 0x2E; pub const CMUN_URCR2_P: u32 = 0x2F; pub const CMUN_CLS_RW: u32 = 0x30; pub const CMUN_CLS_S: u32 = 0x31; pub const CMUN_CLS_C: u32 = 0x32;
pub const CMUN_URCR2_RS: u32 = 0x33; pub const CMUN_URCR2_C: u32 = 0x34; pub const CMUN_CLKEN0: u32 = 0x35; pub const CMUN_CLKEN1: u32 = 0x36; pub const CMUN_PCD0: u32 = 0x37; pub const CMUN_PCD1: u32 = 0x38;
pub const CMUN_TMR0: u32 = 0x39; pub const CMUN_TVS0: u32 = 0x3A; pub const CMUN_TVS1: u32 = 0x3B; pub const CMUN_MCCR: u32 = 0x3C; pub const CMUN_FIR0: u32 = 0x3D; pub const CMUN_FMR0: u32 = 0x3E; pub const CMUN_ETDRB: u32 = 0x3F;

pub const CRCS_STAT_MASK: u32 = 0xF0000000; pub const CRCS_STAT_POR: u32 = 0x10000000; pub const CRCS_STAT_PHR: u32 = 0x20000000; pub const CRCS_STAT_PCIE: u32 = 0x30000000;
pub const CRCS_STAT_CRCS_SYS: u32 = 0x40000000; pub const CRCS_STAT_DBCR_SYS: u32 = 0x50000000; pub const CRCS_STAT_HOST_SYS: u32 = 0x60000000; pub const CRCS_STAT_CHIP_RST_B: u32 = 0x70000000;
pub const CRCS_STAT_CRCS_CHIP: u32 = 0x80000000; pub const CRCS_STAT_DBCR_CHIP: u32 = 0x90000000; pub const CRCS_STAT_HOST_CHIP: u32 = 0xA0000000; pub const CRCS_STAT_PSI_CHIP: u32 = 0xB0000000;
pub const CRCS_STAT_CRCS_CORE: u32 = 0xC0000000; pub const CRCS_STAT_DBCR_CORE: u32 = 0xD0000000; pub const CRCS_STAT_HOST_CORE: u32 = 0xE0000000; pub const CRCS_STAT_PCIE_HOT: u32 = 0xF0000000;
pub const CRCS_STAT_SELF_CORE: u32 = 0x40000000; pub const CRCS_STAT_SELF_CHIP: u32 = 0x50000000; pub const CRCS_WATCHE: u32 = 0x08000000; pub const CRCS_CORE: u32 = 0x04000000;
pub const CRCS_CHIP: u32 = 0x02000000; pub const CRCS_SYS: u32 = 0x01000000; pub const CRCS_WRCR: u32 = 0x00800000; pub const CRCS_EXTCR: u32 = 0x00080000; pub const CRCS_PLOCK: u32 = 0x00000002;

// External DCR accessors supplied by the platform dependency.
extern "C" {
    fn mtdcr(reg: u32, data: u32);
    fn mfdcr(reg: u32) -> u32;
}

#[inline]
pub unsafe fn mtcmu(reg: u32, data: u32) { mtdcr(DCRN_CMU_ADDR, reg); mtdcr(DCRN_CMU_DATA, data); }
#[inline]
pub unsafe fn mfcmu(reg: u32) -> u32 { mtdcr(DCRN_CMU_ADDR, reg); mfdcr(DCRN_CMU_DATA) }
#[inline]
pub unsafe fn mtl2(reg: u32, data: u32) { mtdcr(DCRN_L2CDCRAI, reg); mtdcr(DCRN_L2CDCRDI, data); }
#[inline]
pub unsafe fn mfl2(reg: u32) -> u32 { mtdcr(DCRN_L2CDCRAI, reg); mfdcr(DCRN_L2CDCRDI) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
