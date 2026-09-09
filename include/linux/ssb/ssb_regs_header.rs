/* SPDX-License-Identifier: GPL-2.0 */


/* SiliconBackplane Address Map.
 * All regions may not exist on all chips.
 */
pub const SSB_SDRAM_BASE: u32 = 0x00000000U; /* Physical SDRAM */
pub const SSB_PCI_MEM: u32 = 0x08000000U; /* Host Mode sb2pcitranslation0 (64 MB) */
pub const SSB_PCI_CFG: u32 = 0x0c000000U; /* Host Mode sb2pcitranslation1 (64 MB) */
pub const SSB_SDRAM_SWAPPED: u32 = 0x10000000U; /* Byteswapped Physical SDRAM */
pub const SSB_ENUM_BASE: u32 = 0x18000000U; /* Enumeration space base */
pub const SSB_ENUM_LIMIT: u32 = 0x18010000U; /* Enumeration space limit */

pub const SSB_FLASH2: u32 = 0x1c000000U; /* Flash Region 2 (region 1 shadowed here) */
pub const SSB_FLASH2_SZ: u32 = 0x02000000U; /* Size of Flash Region 2 */

pub const SSB_EXTIF_BASE: u32 = 0x1f000000U; /* External Interface region base address */
pub const SSB_FLASH1: u32 = 0x1fc00000U; /* Flash Region 1 */
pub const SSB_FLASH1_SZ: u32 = 0x00400000U; /* Size of Flash Region 1 */

pub const SSB_PCI_DMA: u32 = 0x40000000U; /* Client Mode sb2pcitranslation2 (1 GB) */
pub const SSB_PCI_DMA_SZ: u32 = 0x40000000U; /* Client Mode sb2pcitranslation2 size in bytes */
pub const SSB_PCIE_DMA_L32: u32 = 0x00000000U; /* PCIE Client Mode sb2pcitranslation2 (2 ZettaBytes), low 32 bits */
pub const SSB_PCIE_DMA_H32: u32 = 0x80000000U; /* PCIE Client Mode sb2pcitranslation2 (2 ZettaBytes), high 32 bits */
pub const SSB_EUART: u32 = (SSB_EXTIF_BASE + 0x00800000);
pub const SSB_LED: u32 = (SSB_EXTIF_BASE + 0x00900000);


/* Enumeration space constants */
pub const SSB_CORE_SIZE: u32 = 0x1000; /* Size of a core MMIO area */
pub const SSB_MAX_NR_CORES: u32 = ((SSB_ENUM_LIMIT - SSB_ENUM_BASE) / SSB_CORE_SIZE);


/* mips address */
pub const SSB_EJTAG: u32 = 0xff200000; /* MIPS EJTAG space (2M) */


/* SSB PCI config space registers. */
pub const SSB_PMCSR: u32 = 0x44;
pub const SSB_PE: u32 = 0x100;
pub const SSB_BAR0_WIN: u32 = 0x80; /* Backplane address space 0 */
pub const SSB_BAR1_WIN: u32 = 0x84; /* Backplane address space 1 */
pub const SSB_SPROMCTL: u32 = 0x88; /* SPROM control */
pub const SSB_SPROMCTL_WE: u32 = 0x10; /* SPROM write enable */
pub const SSB_BAR1_CONTROL: u32 = 0x8c; /* Address space 1 burst control */
pub const SSB_PCI_IRQS: u32 = 0x90; /* PCI interrupts */
pub const SSB_PCI_IRQMASK: u32 = 0x94; /* PCI IRQ control and mask (pcirev >= 6 only) */
pub const SSB_BACKPLANE_IRQS: u32 = 0x98; /* Backplane Interrupts */
pub const SSB_GPIO_IN: u32 = 0xB0; /* GPIO Input (pcirev >= 3 only) */
pub const SSB_GPIO_OUT: u32 = 0xB4; /* GPIO Output (pcirev >= 3 only) */
pub const SSB_GPIO_OUT_ENABLE: u32 = 0xB8; /* GPIO Output Enable/Disable (pcirev >= 3 only) */
pub const SSB_GPIO_SCS: u32 = 0x10; /* PCI config space bit 4 for 4306c0 slow clock source */
pub const SSB_GPIO_HWRAD: u32 = 0x20; /* PCI config space GPIO 13 for hw radio disable */
pub const SSB_GPIO_XTAL: u32 = 0x40; /* PCI config space GPIO 14 for Xtal powerup */
pub const SSB_GPIO_PLL: u32 = 0x80; /* PCI config space GPIO 15 for PLL powerdown */


pub const SSB_BAR0_MAX_RETRIES: u32 = 50;

/* Silicon backplane configuration register definitions */
pub const SSB_IPSFLAG: u32 = 0x0F08;
pub const SSB_IPSFLAG_IRQ1: u32 = 0x0000003F; /* which sbflags get routed to mips interrupt 1 */
pub const SSB_IPSFLAG_IRQ1_SHIFT: u32 = 0;
pub const SSB_IPSFLAG_IRQ2: u32 = 0x00003F00; /* which sbflags get routed to mips interrupt 2 */
pub const SSB_IPSFLAG_IRQ2_SHIFT: u32 = 8;
pub const SSB_IPSFLAG_IRQ3: u32 = 0x003F0000; /* which sbflags get routed to mips interrupt 3 */
pub const SSB_IPSFLAG_IRQ3_SHIFT: u32 = 16;
pub const SSB_IPSFLAG_IRQ4: u32 = 0x3F000000; /* which sbflags get routed to mips interrupt 4 */
pub const SSB_IPSFLAG_IRQ4_SHIFT: u32 = 24;
pub const SSB_TPSFLAG: u32 = 0x0F18;
pub const SSB_TPSFLAG_BPFLAG: u32 = 0x0000003F; /* Backplane flag # */
pub const SSB_TPSFLAG_ALWAYSIRQ: u32 = 0x00000040; /* IRQ is always sent on the Backplane */
pub const SSB_TMERRLOGA: u32 = 0x0F48;
pub const SSB_TMERRLOG: u32 = 0x0F50;
pub const SSB_ADMATCH3: u32 = 0x0F60;
pub const SSB_ADMATCH2: u32 = 0x0F68;
pub const SSB_ADMATCH1: u32 = 0x0F70;
pub const SSB_IMSTATE: u32 = 0x0F90; /* SB Initiator Agent State */
pub const SSB_IMSTATE_PC: u32 = 0x0000000f; /* Pipe Count */
pub const SSB_IMSTATE_AP_MASK: u32 = 0x00000030; /* Arbitration Priority */
pub const SSB_IMSTATE_AP_BOTH: u32 = 0x00000000; /* Use both timeslices and token */
pub const SSB_IMSTATE_AP_TS: u32 = 0x00000010; /* Use timeslices only */
pub const SSB_IMSTATE_AP_TK: u32 = 0x00000020; /* Use token only */
pub const SSB_IMSTATE_AP_RSV: u32 = 0x00000030; /* Reserved */
pub const SSB_IMSTATE_IBE: u32 = 0x00020000; /* In Band Error */
pub const SSB_IMSTATE_TO: u32 = 0x00040000; /* Timeout */
pub const SSB_IMSTATE_BUSY: u32 = 0x01800000; /* Busy (Backplane rev >= 2.3 only) */
pub const SSB_IMSTATE_REJECT: u32 = 0x02000000; /* Reject (Backplane rev >= 2.3 only) */
pub const SSB_INTVEC: u32 = 0x0F94; /* SB Interrupt Mask */
pub const SSB_INTVEC_PCI: u32 = 0x00000001; /* Enable interrupts for PCI */
pub const SSB_INTVEC_ENET0: u32 = 0x00000002; /* Enable interrupts for enet 0 */
pub const SSB_INTVEC_ILINE20: u32 = 0x00000004; /* Enable interrupts for iline20 */
pub const SSB_INTVEC_CODEC: u32 = 0x00000008; /* Enable interrupts for v90 codec */
pub const SSB_INTVEC_USB: u32 = 0x00000010; /* Enable interrupts for usb */
pub const SSB_INTVEC_EXTIF: u32 = 0x00000020; /* Enable interrupts for external i/f */
pub const SSB_INTVEC_ENET1: u32 = 0x00000040; /* Enable interrupts for enet 1 */
pub const SSB_TMSLOW: u32 = 0x0F98; /* SB Target State Low */
pub const SSB_TMSLOW_RESET: u32 = 0x00000001; /* Reset */
pub const SSB_TMSLOW_REJECT: u32 = 0x00000002; /* Reject (Standard Backplane) */
pub const SSB_TMSLOW_REJECT_23: u32 = 0x00000004; /* Reject (Backplane rev 2.3) */
pub const SSB_TMSLOW_CLOCK: u32 = 0x00010000; /* Clock Enable */
pub const SSB_TMSLOW_FGC: u32 = 0x00020000; /* Force Gated Clocks On */
pub const SSB_TMSLOW_PE: u32 = 0x40000000; /* Power Management Enable */
pub const SSB_TMSLOW_BE: u32 = 0x80000000; /* BIST Enable */
pub const SSB_TMSHIGH: u32 = 0x0F9C; /* SB Target State High */
pub const SSB_TMSHIGH_SERR: u32 = 0x00000001; /* S-error */
pub const SSB_TMSHIGH_INT: u32 = 0x00000002; /* Interrupt */
pub const SSB_TMSHIGH_BUSY: u32 = 0x00000004; /* Busy */
pub const SSB_TMSHIGH_TO: u32 = 0x00000020; /* Timeout. Backplane rev >= 2.3 only */
pub const SSB_TMSHIGH_COREFL: u32 = 0x1FFF0000; /* Core specific flags */
pub const SSB_TMSHIGH_COREFL_SHIFT: u32 = 16;
pub const SSB_TMSHIGH_DMA64: u32 = 0x10000000; /* 64bit DMA supported */
pub const SSB_TMSHIGH_GCR: u32 = 0x20000000; /* Gated Clock Request */
pub const SSB_TMSHIGH_BISTF: u32 = 0x40000000; /* BIST Failed */
pub const SSB_TMSHIGH_BISTD: u32 = 0x80000000; /* BIST Done */
pub const SSB_BWA0: u32 = 0x0FA0;
pub const SSB_IMCFGLO: u32 = 0x0FA8;
pub const SSB_IMCFGLO_SERTO: u32 = 0x00000007; /* Service timeout */
pub const SSB_IMCFGLO_REQTO: u32 = 0x00000070; /* Request timeout */
pub const SSB_IMCFGLO_REQTO_SHIFT: u32 = 4;
pub const SSB_IMCFGLO_CONNID: u32 = 0x00FF0000; /* Connection ID */
pub const SSB_IMCFGLO_CONNID_SHIFT: u32 = 16;
pub const SSB_IMCFGHI: u32 = 0x0FAC;
pub const SSB_ADMATCH0: u32 = 0x0FB0;
pub const SSB_TMCFGLO: u32 = 0x0FB8;
pub const SSB_TMCFGHI: u32 = 0x0FBC;
pub const SSB_BCONFIG: u32 = 0x0FC0;
pub const SSB_BSTATE: u32 = 0x0FC8;
pub const SSB_ACTCFG: u32 = 0x0FD8;
pub const SSB_FLAGST: u32 = 0x0FE8;
pub const SSB_IDLOW: u32 = 0x0FF8;
pub const SSB_IDLOW_CFGSP: u32 = 0x00000003; /* Config Space */
pub const SSB_IDLOW_ADDRNGE: u32 = 0x00000038; /* Address Ranges supported */
pub const SSB_IDLOW_ADDRNGE_SHIFT: u32 = 3;
pub const SSB_IDLOW_SYNC: u32 = 0x00000040;
pub const SSB_IDLOW_INITIATOR: u32 = 0x00000080;
pub const SSB_IDLOW_MIBL: u32 = 0x00000F00; /* Minimum Backplane latency */
pub const SSB_IDLOW_MIBL_SHIFT: u32 = 8;
pub const SSB_IDLOW_MABL: u32 = 0x0000F000; /* Maximum Backplane latency */
pub const SSB_IDLOW_MABL_SHIFT: u32 = 12;
pub const SSB_IDLOW_TIF: u32 = 0x00010000; /* This Initiator is first */
pub const SSB_IDLOW_CCW: u32 = 0x000C0000; /* Cycle counter width */
pub const SSB_IDLOW_CCW_SHIFT: u32 = 18;
pub const SSB_IDLOW_TPT: u32 = 0x00F00000; /* Target ports */
pub const SSB_IDLOW_TPT_SHIFT: u32 = 20;
pub const SSB_IDLOW_INITP: u32 = 0x0F000000; /* Initiator ports */
pub const SSB_IDLOW_INITP_SHIFT: u32 = 24;
pub const SSB_IDLOW_SSBREV: u32 = 0xF0000000; /* Sonics Backplane Revision code */
pub const SSB_IDLOW_SSBREV_22: u32 = 0x00000000; /* <= 2.2 */
pub const SSB_IDLOW_SSBREV_23: u32 = 0x10000000; /* 2.3 */
pub const SSB_IDLOW_SSBREV_24: u32 = 0x40000000; /* ?? Found in BCM4328 */
pub const SSB_IDLOW_SSBREV_25: u32 = 0x50000000; /* ?? Not Found yet */
pub const SSB_IDLOW_SSBREV_26: u32 = 0x60000000; /* ?? Found in some BCM4311/2 */
pub const SSB_IDLOW_SSBREV_27: u32 = 0x70000000; /* ?? Found in some BCM4311/2 */
pub const SSB_IDHIGH: u32 = 0x0FFC; /* SB Identification High */
pub const SSB_IDHIGH_RCLO: u32 = 0x0000000F; /* Revision Code (low part) */
pub const SSB_IDHIGH_CC: u32 = 0x00008FF0; /* Core Code */
pub const SSB_IDHIGH_CC_SHIFT: u32 = 4;
pub const SSB_IDHIGH_RCHI: u32 = 0x00007000; /* Revision Code (high part) */
pub const SSB_IDHIGH_RCHI_SHIFT: u32 = 8; /* yes, shift 8 is right */
pub const SSB_IDHIGH_VC: u32 = 0xFFFF0000; /* Vendor Code */
pub const SSB_IDHIGH_VC_SHIFT: u32 = 16;

/* SPROM shadow area. If not otherwise noted, fields are
 * two bytes wide. Note that the SPROM can _only_ be read
 * in two-byte quantities.
 */
pub const SSB_SPROMSIZE_WORDS: u32 = 64;
pub const SSB_SPROMSIZE_BYTES: u32 = (SSB_SPROMSIZE_WORDS * 2);
pub const SSB_SPROMSIZE_WORDS_R123: u32 = 64;
pub const SSB_SPROMSIZE_WORDS_R4: u32 = 220;
pub const SSB_SPROMSIZE_BYTES_R123: u32 = (SSB_SPROMSIZE_WORDS_R123 * 2);
pub const SSB_SPROMSIZE_BYTES_R4: u32 = (SSB_SPROMSIZE_WORDS_R4 * 2);
pub const SSB_SPROMSIZE_WORDS_R10: u32 = 230;
pub const SSB_SPROMSIZE_WORDS_R11: u32 = 234;
pub const SSB_SPROM_BASE1: u32 = 0x1000;
pub const SSB_SPROM_BASE31: u32 = 0x0800;
pub const SSB_SPROM_REVISION: u32 = 0x007E;
pub const SSB_SPROM_REVISION_REV: u32 = 0x00FF; /* SPROM Revision number */
pub const SSB_SPROM_REVISION_CRC: u32 = 0xFF00; /* SPROM CRC8 value */
pub const SSB_SPROM_REVISION_CRC_SHIFT: u32 = 8;

/* SPROM Revision 1 */
pub const SSB_SPROM1_SPID: u32 = 0x0004; /* Subsystem Product ID for PCI */
pub const SSB_SPROM1_SVID: u32 = 0x0006; /* Subsystem Vendor ID for PCI */
pub const SSB_SPROM1_PID: u32 = 0x0008; /* Product ID for PCI */
pub const SSB_SPROM1_IL0MAC: u32 = 0x0048; /* 6 bytes MAC address for 802.11b/g */
pub const SSB_SPROM1_ET0MAC: u32 = 0x004E; /* 6 bytes MAC address for Ethernet */
pub const SSB_SPROM1_ET1MAC: u32 = 0x0054; /* 6 bytes MAC address for 802.11a */
pub const SSB_SPROM1_ETHPHY: u32 = 0x005A; /* Ethernet PHY settings */
pub const SSB_SPROM1_ETHPHY_ET0A: u32 = 0x001F; /* MII Address for enet0 */
pub const SSB_SPROM1_ETHPHY_ET1A: u32 = 0x03E0; /* MII Address for enet1 */
pub const SSB_SPROM1_ETHPHY_ET1A_SHIFT: u32 = 5;
pub const SSB_SPROM1_ETHPHY_ET0M: u32 = (1<<14); /* MDIO for enet0 */
pub const SSB_SPROM1_ETHPHY_ET1M: u32 = (1<<15); /* MDIO for enet1 */
pub const SSB_SPROM1_BINF: u32 = 0x005C; /* Board info */
pub const SSB_SPROM1_BINF_BREV: u32 = 0x00FF; /* Board Revision */
pub const SSB_SPROM1_BINF_CCODE: u32 = 0x0F00; /* Country Code */
pub const SSB_SPROM1_BINF_CCODE_SHIFT: u32 = 8;
pub const SSB_SPROM1_BINF_ANTBG: u32 = 0x3000; /* Available B-PHY and G-PHY antennas */
pub const SSB_SPROM1_BINF_ANTBG_SHIFT: u32 = 12;
pub const SSB_SPROM1_BINF_ANTA: u32 = 0xC000; /* Available A-PHY antennas */
pub const SSB_SPROM1_BINF_ANTA_SHIFT: u32 = 14;
pub const SSB_SPROM1_PA0B0: u32 = 0x005E;
pub const SSB_SPROM1_PA0B1: u32 = 0x0060;
pub const SSB_SPROM1_PA0B2: u32 = 0x0062;
pub const SSB_SPROM1_GPIOA: u32 = 0x0064; /* General Purpose IO pins 0 and 1 */
pub const SSB_SPROM1_GPIOA_P0: u32 = 0x00FF; /* Pin 0 */
pub const SSB_SPROM1_GPIOA_P1: u32 = 0xFF00; /* Pin 1 */
pub const SSB_SPROM1_GPIOA_P1_SHIFT: u32 = 8;
pub const SSB_SPROM1_GPIOB: u32 = 0x0066; /* General Purpuse IO pins 2 and 3 */
pub const SSB_SPROM1_GPIOB_P2: u32 = 0x00FF; /* Pin 2 */
pub const SSB_SPROM1_GPIOB_P3: u32 = 0xFF00; /* Pin 3 */
pub const SSB_SPROM1_GPIOB_P3_SHIFT: u32 = 8;
pub const SSB_SPROM1_MAXPWR: u32 = 0x0068; /* Power Amplifier Max Power */
pub const SSB_SPROM1_MAXPWR_BG: u32 = 0x00FF; /* B-PHY and G-PHY (in dBm Q5.2) */
pub const SSB_SPROM1_MAXPWR_A: u32 = 0xFF00; /* A-PHY (in dBm Q5.2) */
pub const SSB_SPROM1_MAXPWR_A_SHIFT: u32 = 8;
pub const SSB_SPROM1_PA1B0: u32 = 0x006A;
pub const SSB_SPROM1_PA1B1: u32 = 0x006C;
pub const SSB_SPROM1_PA1B2: u32 = 0x006E;
pub const SSB_SPROM1_ITSSI: u32 = 0x0070; /* Idle TSSI Target */
pub const SSB_SPROM1_ITSSI_BG: u32 = 0x00FF; /* B-PHY and G-PHY*/
pub const SSB_SPROM1_ITSSI_A: u32 = 0xFF00; /* A-PHY */
pub const SSB_SPROM1_ITSSI_A_SHIFT: u32 = 8;
pub const SSB_SPROM1_BFLLO: u32 = 0x0072; /* Boardflags (low 16 bits) */
pub const SSB_SPROM1_AGAIN: u32 = 0x0074; /* Antenna Gain (in dBm Q5.2) */
pub const SSB_SPROM1_AGAIN_BG: u32 = 0x00FF; /* B-PHY and G-PHY */
pub const SSB_SPROM1_AGAIN_BG_SHIFT: u32 = 0;
pub const SSB_SPROM1_AGAIN_A: u32 = 0xFF00; /* A-PHY */
pub const SSB_SPROM1_AGAIN_A_SHIFT: u32 = 8;
pub const SSB_SPROM1_CCODE: u32 = 0x0076;

/* SPROM Revision 2 (inherits from rev 1) */
pub const SSB_SPROM2_BFLHI: u32 = 0x0038; /* Boardflags (high 16 bits) */
pub const SSB_SPROM2_MAXP_A: u32 = 0x003A; /* A-PHY Max Power */
pub const SSB_SPROM2_MAXP_A_HI: u32 = 0x00FF; /* Max Power High */
pub const SSB_SPROM2_MAXP_A_LO: u32 = 0xFF00; /* Max Power Low */
pub const SSB_SPROM2_MAXP_A_LO_SHIFT: u32 = 8;
pub const SSB_SPROM2_PA1LOB0: u32 = 0x003C; /* A-PHY PowerAmplifier Low Settings */
pub const SSB_SPROM2_PA1LOB1: u32 = 0x003E; /* A-PHY PowerAmplifier Low Settings */
pub const SSB_SPROM2_PA1LOB2: u32 = 0x0040; /* A-PHY PowerAmplifier Low Settings */
pub const SSB_SPROM2_PA1HIB0: u32 = 0x0042; /* A-PHY PowerAmplifier High Settings */
pub const SSB_SPROM2_PA1HIB1: u32 = 0x0044; /* A-PHY PowerAmplifier High Settings */
pub const SSB_SPROM2_PA1HIB2: u32 = 0x0046; /* A-PHY PowerAmplifier High Settings */
pub const SSB_SPROM2_OPO: u32 = 0x0078; /* OFDM Power Offset from CCK Level */
pub const SSB_SPROM2_OPO_VALUE: u32 = 0x00FF;
pub const SSB_SPROM2_OPO_UNUSED: u32 = 0xFF00;
pub const SSB_SPROM2_CCODE: u32 = 0x007C; /* Two char Country Code */

/* SPROM Revision 3 (inherits most data from rev 2) */
pub const SSB_SPROM3_OFDMAPO: u32 = 0x002C; /* A-PHY OFDM Mid Power Offset (4 bytes, BigEndian) */
pub const SSB_SPROM3_OFDMALPO: u32 = 0x0030; /* A-PHY OFDM Low Power Offset (4 bytes, BigEndian) */
pub const SSB_SPROM3_OFDMAHPO: u32 = 0x0034; /* A-PHY OFDM High Power Offset (4 bytes, BigEndian) */
pub const SSB_SPROM3_GPIOLDC: u32 = 0x0042; /* GPIO LED Powersave Duty Cycle (4 bytes, BigEndian) */
pub const SSB_SPROM3_GPIOLDC_OFF: u32 = 0x0000FF00; /* Off Count */
pub const SSB_SPROM3_GPIOLDC_OFF_SHIFT: u32 = 8;
pub const SSB_SPROM3_GPIOLDC_ON: u32 = 0x00FF0000; /* On Count */
pub const SSB_SPROM3_GPIOLDC_ON_SHIFT: u32 = 16;
pub const SSB_SPROM3_IL0MAC: u32 = 0x004A; /* 6 bytes MAC address for 802.11b/g */
pub const SSB_SPROM3_CCKPO: u32 = 0x0078; /* CCK Power Offset */
pub const SSB_SPROM3_CCKPO_1M: u32 = 0x000F; /* 1M Rate PO */
pub const SSB_SPROM3_CCKPO_2M: u32 = 0x00F0; /* 2M Rate PO */
pub const SSB_SPROM3_CCKPO_2M_SHIFT: u32 = 4;
pub const SSB_SPROM3_CCKPO_55M: u32 = 0x0F00; /* 5.5M Rate PO */
pub const SSB_SPROM3_CCKPO_55M_SHIFT: u32 = 8;
pub const SSB_SPROM3_CCKPO_11M: u32 = 0xF000; /* 11M Rate PO */
pub const SSB_SPROM3_CCKPO_11M_SHIFT: u32 = 12;
pub const SSB_SPROM3_OFDMGPO: u32 = 0x107A; /* G-PHY OFDM Power Offset (4 bytes, BigEndian) */

/* SPROM Revision 4 */
pub const SSB_SPROM4_BOARDREV: u32 = 0x0042; /* Board revision */
pub const SSB_SPROM4_BFLLO: u32 = 0x0044; /* Boardflags (low 16 bits) */
pub const SSB_SPROM4_BFLHI: u32 = 0x0046; /* Board Flags Hi */
pub const SSB_SPROM4_BFL2LO: u32 = 0x0048; /* Board flags 2 (low 16 bits) */
pub const SSB_SPROM4_BFL2HI: u32 = 0x004A; /* Board flags 2 Hi */
pub const SSB_SPROM4_IL0MAC: u32 = 0x004C; /* 6 byte MAC address for a/b/g/n */
pub const SSB_SPROM4_CCODE: u32 = 0x0052; /* Country Code (2 bytes) */
pub const SSB_SPROM4_GPIOA: u32 = 0x0056; /* Gen. Purpose IO # 0 and 1 */
pub const SSB_SPROM4_GPIOA_P0: u32 = 0x00FF; /* Pin 0 */
pub const SSB_SPROM4_GPIOA_P1: u32 = 0xFF00; /* Pin 1 */
pub const SSB_SPROM4_GPIOA_P1_SHIFT: u32 = 8;
pub const SSB_SPROM4_GPIOB: u32 = 0x0058; /* Gen. Purpose IO # 2 and 3 */
pub const SSB_SPROM4_GPIOB_P2: u32 = 0x00FF; /* Pin 2 */
pub const SSB_SPROM4_GPIOB_P3: u32 = 0xFF00; /* Pin 3 */
pub const SSB_SPROM4_GPIOB_P3_SHIFT: u32 = 8;
pub const SSB_SPROM4_ETHPHY: u32 = 0x005A; /* Ethernet PHY settings ?? */
pub const SSB_SPROM4_ETHPHY_ET0A: u32 = 0x001F; /* MII Address for enet0 */
pub const SSB_SPROM4_ETHPHY_ET1A: u32 = 0x03E0; /* MII Address for enet1 */
pub const SSB_SPROM4_ETHPHY_ET1A_SHIFT: u32 = 5;
pub const SSB_SPROM4_ETHPHY_ET0M: u32 = (1<<14); /* MDIO for enet0 */
pub const SSB_SPROM4_ETHPHY_ET1M: u32 = (1<<15); /* MDIO for enet1 */
pub const SSB_SPROM4_ANTAVAIL: u32 = 0x005C; /* Antenna available bitfields */
pub const SSB_SPROM4_ANTAVAIL_BG: u32 = 0x00FF; /* B-PHY and G-PHY bitfield */
pub const SSB_SPROM4_ANTAVAIL_BG_SHIFT: u32 = 0;
pub const SSB_SPROM4_ANTAVAIL_A: u32 = 0xFF00; /* A-PHY bitfield */
pub const SSB_SPROM4_ANTAVAIL_A_SHIFT: u32 = 8;
pub const SSB_SPROM4_AGAIN01: u32 = 0x005E; /* Antenna Gain (in dBm Q5.2) */
pub const SSB_SPROM4_AGAIN0: u32 = 0x00FF; /* Antenna 0 */
pub const SSB_SPROM4_AGAIN0_SHIFT: u32 = 0;
pub const SSB_SPROM4_AGAIN1: u32 = 0xFF00; /* Antenna 1 */
pub const SSB_SPROM4_AGAIN1_SHIFT: u32 = 8;
pub const SSB_SPROM4_AGAIN23: u32 = 0x0060;
pub const SSB_SPROM4_AGAIN2: u32 = 0x00FF; /* Antenna 2 */
pub const SSB_SPROM4_AGAIN2_SHIFT: u32 = 0;
pub const SSB_SPROM4_AGAIN3: u32 = 0xFF00; /* Antenna 3 */
pub const SSB_SPROM4_AGAIN3_SHIFT: u32 = 8;
pub const SSB_SPROM4_TXPID2G01: u32 = 0x0062; /* TX Power Index 2GHz */
pub const SSB_SPROM4_TXPID2G0: u32 = 0x00FF;
pub const SSB_SPROM4_TXPID2G0_SHIFT: u32 = 0;
pub const SSB_SPROM4_TXPID2G1: u32 = 0xFF00;
pub const SSB_SPROM4_TXPID2G1_SHIFT: u32 = 8;
pub const SSB_SPROM4_TXPID2G23: u32 = 0x0064; /* TX Power Index 2GHz */
pub const SSB_SPROM4_TXPID2G2: u32 = 0x00FF;
pub const SSB_SPROM4_TXPID2G2_SHIFT: u32 = 0;
pub const SSB_SPROM4_TXPID2G3: u32 = 0xFF00;
pub const SSB_SPROM4_TXPID2G3_SHIFT: u32 = 8;
pub const SSB_SPROM4_TXPID5G01: u32 = 0x0066; /* TX Power Index 5GHz middle subband */
pub const SSB_SPROM4_TXPID5G0: u32 = 0x00FF;
pub const SSB_SPROM4_TXPID5G0_SHIFT: u32 = 0;
pub const SSB_SPROM4_TXPID5G1: u32 = 0xFF00;
pub const SSB_SPROM4_TXPID5G1_SHIFT: u32 = 8;
pub const SSB_SPROM4_TXPID5G23: u32 = 0x0068; /* TX Power Index 5GHz middle subband */
pub const SSB_SPROM4_TXPID5G2: u32 = 0x00FF;
pub const SSB_SPROM4_TXPID5G2_SHIFT: u32 = 0;
pub const SSB_SPROM4_TXPID5G3: u32 = 0xFF00;
pub const SSB_SPROM4_TXPID5G3_SHIFT: u32 = 8;
pub const SSB_SPROM4_TXPID5GL01: u32 = 0x006A; /* TX Power Index 5GHz low subband */
pub const SSB_SPROM4_TXPID5GL0: u32 = 0x00FF;
pub const SSB_SPROM4_TXPID5GL0_SHIFT: u32 = 0;
pub const SSB_SPROM4_TXPID5GL1: u32 = 0xFF00;
pub const SSB_SPROM4_TXPID5GL1_SHIFT: u32 = 8;
pub const SSB_SPROM4_TXPID5GL23: u32 = 0x006C; /* TX Power Index 5GHz low subband */
pub const SSB_SPROM4_TXPID5GL2: u32 = 0x00FF;
pub const SSB_SPROM4_TXPID5GL2_SHIFT: u32 = 0;
pub const SSB_SPROM4_TXPID5GL3: u32 = 0xFF00;
pub const SSB_SPROM4_TXPID5GL3_SHIFT: u32 = 8;
pub const SSB_SPROM4_TXPID5GH01: u32 = 0x006E; /* TX Power Index 5GHz high subband */
pub const SSB_SPROM4_TXPID5GH0: u32 = 0x00FF;
pub const SSB_SPROM4_TXPID5GH0_SHIFT: u32 = 0;
pub const SSB_SPROM4_TXPID5GH1: u32 = 0xFF00;
pub const SSB_SPROM4_TXPID5GH1_SHIFT: u32 = 8;
pub const SSB_SPROM4_TXPID5GH23: u32 = 0x0070; /* TX Power Index 5GHz high subband */
pub const SSB_SPROM4_TXPID5GH2: u32 = 0x00FF;
pub const SSB_SPROM4_TXPID5GH2_SHIFT: u32 = 0;
pub const SSB_SPROM4_TXPID5GH3: u32 = 0xFF00;
pub const SSB_SPROM4_TXPID5GH3_SHIFT: u32 = 8;

/* There are 4 blocks with power info sharing the same layout */
pub const SSB_SPROM4_PWR_INFO_CORE0: u32 = 0x0080;
pub const SSB_SPROM4_PWR_INFO_CORE1: u32 = 0x00AE;
pub const SSB_SPROM4_PWR_INFO_CORE2: u32 = 0x00DC;
pub const SSB_SPROM4_PWR_INFO_CORE3: u32 = 0x010A;

pub const SSB_SPROM4_2G_MAXP_ITSSI: u32 = 0x00; /* 2 GHz ITSSI and 2 GHz Max Power */
pub const SSB_SPROM4_2G_MAXP: u32 = 0x00FF;
pub const SSB_SPROM4_2G_ITSSI: u32 = 0xFF00;
pub const SSB_SPROM4_2G_ITSSI_SHIFT: u32 = 8;
pub const SSB_SPROM4_2G_PA_0: u32 = 0x02; /* 2 GHz power amp */
pub const SSB_SPROM4_2G_PA_1: u32 = 0x04;
pub const SSB_SPROM4_2G_PA_2: u32 = 0x06;
pub const SSB_SPROM4_2G_PA_3: u32 = 0x08;
pub const SSB_SPROM4_5G_MAXP_ITSSI: u32 = 0x0A; /* 5 GHz ITSSI and 5.3 GHz Max Power */
pub const SSB_SPROM4_5G_MAXP: u32 = 0x00FF;
pub const SSB_SPROM4_5G_ITSSI: u32 = 0xFF00;
pub const SSB_SPROM4_5G_ITSSI_SHIFT: u32 = 8;
pub const SSB_SPROM4_5GHL_MAXP: u32 = 0x0C; /* 5.2 GHz and 5.8 GHz Max Power */
pub const SSB_SPROM4_5GH_MAXP: u32 = 0x00FF;
pub const SSB_SPROM4_5GL_MAXP: u32 = 0xFF00;
pub const SSB_SPROM4_5GL_MAXP_SHIFT: u32 = 8;
pub const SSB_SPROM4_5G_PA_0: u32 = 0x0E; /* 5.3 GHz power amp */
pub const SSB_SPROM4_5G_PA_1: u32 = 0x10;
pub const SSB_SPROM4_5G_PA_2: u32 = 0x12;
pub const SSB_SPROM4_5G_PA_3: u32 = 0x14;
pub const SSB_SPROM4_5GL_PA_0: u32 = 0x16; /* 5.2 GHz power amp */
pub const SSB_SPROM4_5GL_PA_1: u32 = 0x18;
pub const SSB_SPROM4_5GL_PA_2: u32 = 0x1A;
pub const SSB_SPROM4_5GL_PA_3: u32 = 0x1C;
pub const SSB_SPROM4_5GH_PA_0: u32 = 0x1E; /* 5.8 GHz power amp */
pub const SSB_SPROM4_5GH_PA_1: u32 = 0x20;
pub const SSB_SPROM4_5GH_PA_2: u32 = 0x22;
pub const SSB_SPROM4_5GH_PA_3: u32 = 0x24;

/* TODO: Make it deprecated */
pub const SSB_SPROM4_MAXP_BG: u32 = 0x0080; /* Max Power BG in path 1 */
pub const SSB_SPROM4_MAXP_BG_MASK: u32 = 0x00FF; /* Mask for Max Power BG */
pub const SSB_SPROM4_ITSSI_BG: u32 = 0xFF00; /* Mask for path 1 itssi_bg */
pub const SSB_SPROM4_ITSSI_BG_SHIFT: u32 = 8;
pub const SSB_SPROM4_MAXP_A: u32 = 0x008A; /* Max Power A in path 1 */
pub const SSB_SPROM4_MAXP_A_MASK: u32 = 0x00FF; /* Mask for Max Power A */
pub const SSB_SPROM4_ITSSI_A: u32 = 0xFF00; /* Mask for path 1 itssi_a */
pub const SSB_SPROM4_ITSSI_A_SHIFT: u32 = 8;
pub const SSB_SPROM4_PA0B0: u32 = 0x0082; /* The paXbY locations are */
pub const SSB_SPROM4_PA0B1: u32 = 0x0084; /*   only guesses */
pub const SSB_SPROM4_PA0B2: u32 = 0x0086;
pub const SSB_SPROM4_PA1B0: u32 = 0x008E;
pub const SSB_SPROM4_PA1B1: u32 = 0x0090;
pub const SSB_SPROM4_PA1B2: u32 = 0x0092;

/* SPROM Revision 5 (inherits most data from rev 4) */
pub const SSB_SPROM5_CCODE: u32 = 0x0044; /* Country Code (2 bytes) */
pub const SSB_SPROM5_BFLLO: u32 = 0x004A; /* Boardflags (low 16 bits) */
pub const SSB_SPROM5_BFLHI: u32 = 0x004C; /* Board Flags Hi */
pub const SSB_SPROM5_BFL2LO: u32 = 0x004E; /* Board flags 2 (low 16 bits) */
pub const SSB_SPROM5_BFL2HI: u32 = 0x0050; /* Board flags 2 Hi */
pub const SSB_SPROM5_IL0MAC: u32 = 0x0052; /* 6 byte MAC address for a/b/g/n */
pub const SSB_SPROM5_GPIOA: u32 = 0x0076; /* Gen. Purpose IO # 0 and 1 */
pub const SSB_SPROM5_GPIOA_P0: u32 = 0x00FF; /* Pin 0 */
pub const SSB_SPROM5_GPIOA_P1: u32 = 0xFF00; /* Pin 1 */
pub const SSB_SPROM5_GPIOA_P1_SHIFT: u32 = 8;
pub const SSB_SPROM5_GPIOB: u32 = 0x0078; /* Gen. Purpose IO # 2 and 3 */
pub const SSB_SPROM5_GPIOB_P2: u32 = 0x00FF; /* Pin 2 */
pub const SSB_SPROM5_GPIOB_P3: u32 = 0xFF00; /* Pin 3 */
pub const SSB_SPROM5_GPIOB_P3_SHIFT: u32 = 8;

/* SPROM Revision 8 */
pub const SSB_SPROM8_BOARDREV: u32 = 0x0082; /* Board revision */
pub const SSB_SPROM8_BFLLO: u32 = 0x0084; /* Board flags (bits 0-15) */
pub const SSB_SPROM8_BFLHI: u32 = 0x0086; /* Board flags (bits 16-31) */
pub const SSB_SPROM8_BFL2LO: u32 = 0x0088; /* Board flags (bits 32-47) */
pub const SSB_SPROM8_BFL2HI: u32 = 0x008A; /* Board flags (bits 48-63) */
pub const SSB_SPROM8_IL0MAC: u32 = 0x008C; /* 6 byte MAC address */
pub const SSB_SPROM8_CCODE: u32 = 0x0092; /* 2 byte country code */
pub const SSB_SPROM8_GPIOA: u32 = 0x0096; /*Gen. Purpose IO # 0 and 1 */
pub const SSB_SPROM8_GPIOA_P0: u32 = 0x00FF; /* Pin 0 */
pub const SSB_SPROM8_GPIOA_P1: u32 = 0xFF00; /* Pin 1 */
pub const SSB_SPROM8_GPIOA_P1_SHIFT: u32 = 8;
pub const SSB_SPROM8_GPIOB: u32 = 0x0098; /* Gen. Purpose IO # 2 and 3 */
pub const SSB_SPROM8_GPIOB_P2: u32 = 0x00FF; /* Pin 2 */
pub const SSB_SPROM8_GPIOB_P3: u32 = 0xFF00; /* Pin 3 */
pub const SSB_SPROM8_GPIOB_P3_SHIFT: u32 = 8;
pub const SSB_SPROM8_LEDDC: u32 = 0x009A;
pub const SSB_SPROM8_LEDDC_ON: u32 = 0xFF00; /* oncount */
pub const SSB_SPROM8_LEDDC_ON_SHIFT: u32 = 8;
pub const SSB_SPROM8_LEDDC_OFF: u32 = 0x00FF; /* offcount */
pub const SSB_SPROM8_LEDDC_OFF_SHIFT: u32 = 0;
pub const SSB_SPROM8_ANTAVAIL: u32 = 0x009C; /* Antenna available bitfields*/
pub const SSB_SPROM8_ANTAVAIL_A: u32 = 0xFF00; /* A-PHY bitfield */
pub const SSB_SPROM8_ANTAVAIL_A_SHIFT: u32 = 8;
pub const SSB_SPROM8_ANTAVAIL_BG: u32 = 0x00FF; /* B-PHY and G-PHY bitfield */
pub const SSB_SPROM8_ANTAVAIL_BG_SHIFT: u32 = 0;
pub const SSB_SPROM8_AGAIN01: u32 = 0x009E; /* Antenna Gain (in dBm Q5.2) */
pub const SSB_SPROM8_AGAIN0: u32 = 0x00FF; /* Antenna 0 */
pub const SSB_SPROM8_AGAIN0_SHIFT: u32 = 0;
pub const SSB_SPROM8_AGAIN1: u32 = 0xFF00; /* Antenna 1 */
pub const SSB_SPROM8_AGAIN1_SHIFT: u32 = 8;
pub const SSB_SPROM8_AGAIN23: u32 = 0x00A0;
pub const SSB_SPROM8_AGAIN2: u32 = 0x00FF; /* Antenna 2 */
pub const SSB_SPROM8_AGAIN2_SHIFT: u32 = 0;
pub const SSB_SPROM8_AGAIN3: u32 = 0xFF00; /* Antenna 3 */
pub const SSB_SPROM8_AGAIN3_SHIFT: u32 = 8;
pub const SSB_SPROM8_TXRXC: u32 = 0x00A2;
pub const SSB_SPROM8_TXRXC_TXCHAIN: u32 = 0x000f;
pub const SSB_SPROM8_TXRXC_TXCHAIN_SHIFT: u32 = 0;
pub const SSB_SPROM8_TXRXC_RXCHAIN: u32 = 0x00f0;
pub const SSB_SPROM8_TXRXC_RXCHAIN_SHIFT: u32 = 4;
pub const SSB_SPROM8_TXRXC_SWITCH: u32 = 0xff00;
pub const SSB_SPROM8_TXRXC_SWITCH_SHIFT: u32 = 8;
pub const SSB_SPROM8_RSSIPARM2G: u32 = 0x00A4; /* RSSI params for 2GHz */
pub const SSB_SPROM8_RSSISMF2G: u32 = 0x000F;
pub const SSB_SPROM8_RSSISMC2G: u32 = 0x00F0;
pub const SSB_SPROM8_RSSISMC2G_SHIFT: u32 = 4;
pub const SSB_SPROM8_RSSISAV2G: u32 = 0x0700;
pub const SSB_SPROM8_RSSISAV2G_SHIFT: u32 = 8;
pub const SSB_SPROM8_BXA2G: u32 = 0x1800;
pub const SSB_SPROM8_BXA2G_SHIFT: u32 = 11;
pub const SSB_SPROM8_RSSIPARM5G: u32 = 0x00A6; /* RSSI params for 5GHz */
pub const SSB_SPROM8_RSSISMF5G: u32 = 0x000F;
pub const SSB_SPROM8_RSSISMC5G: u32 = 0x00F0;
pub const SSB_SPROM8_RSSISMC5G_SHIFT: u32 = 4;
pub const SSB_SPROM8_RSSISAV5G: u32 = 0x0700;
pub const SSB_SPROM8_RSSISAV5G_SHIFT: u32 = 8;
pub const SSB_SPROM8_BXA5G: u32 = 0x1800;
pub const SSB_SPROM8_BXA5G_SHIFT: u32 = 11;
pub const SSB_SPROM8_TRI25G: u32 = 0x00A8; /* TX isolation 2.4&5.3GHz */
pub const SSB_SPROM8_TRI2G: u32 = 0x00FF; /* TX isolation 2.4GHz */
pub const SSB_SPROM8_TRI5G: u32 = 0xFF00; /* TX isolation 5.3GHz */
pub const SSB_SPROM8_TRI5G_SHIFT: u32 = 8;
pub const SSB_SPROM8_TRI5GHL: u32 = 0x00AA; /* TX isolation 5.2/5.8GHz */
pub const SSB_SPROM8_TRI5GL: u32 = 0x00FF; /* TX isolation 5.2GHz */
pub const SSB_SPROM8_TRI5GH: u32 = 0xFF00; /* TX isolation 5.8GHz */
pub const SSB_SPROM8_TRI5GH_SHIFT: u32 = 8;
pub const SSB_SPROM8_RXPO: u32 = 0x00AC; /* RX power offsets */
pub const SSB_SPROM8_RXPO2G: u32 = 0x00FF; /* 2GHz RX power offset */
pub const SSB_SPROM8_RXPO2G_SHIFT: u32 = 0;
pub const SSB_SPROM8_RXPO5G: u32 = 0xFF00; /* 5GHz RX power offset */
pub const SSB_SPROM8_RXPO5G_SHIFT: u32 = 8;
pub const SSB_SPROM8_FEM2G: u32 = 0x00AE;
pub const SSB_SPROM8_FEM5G: u32 = 0x00B0;
pub const SSB_SROM8_FEM_TSSIPOS: u32 = 0x0001;
pub const SSB_SROM8_FEM_TSSIPOS_SHIFT: u32 = 0;
pub const SSB_SROM8_FEM_EXTPA_GAIN: u32 = 0x0006;
pub const SSB_SROM8_FEM_EXTPA_GAIN_SHIFT: u32 = 1;
pub const SSB_SROM8_FEM_PDET_RANGE: u32 = 0x00F8;
pub const SSB_SROM8_FEM_PDET_RANGE_SHIFT: u32 = 3;
pub const SSB_SROM8_FEM_TR_ISO: u32 = 0x0700;
pub const SSB_SROM8_FEM_TR_ISO_SHIFT: u32 = 8;
pub const SSB_SROM8_FEM_ANTSWLUT: u32 = 0xF800;
pub const SSB_SROM8_FEM_ANTSWLUT_SHIFT: u32 = 11;
pub const SSB_SPROM8_THERMAL: u32 = 0x00B2;
pub const SSB_SPROM8_THERMAL_OFFSET: u32 = 0x00ff;
pub const SSB_SPROM8_THERMAL_OFFSET_SHIFT: u32 = 0;
pub const SSB_SPROM8_THERMAL_TRESH: u32 = 0xff00;
pub const SSB_SPROM8_THERMAL_TRESH_SHIFT: u32 = 8;
/* Temp sense related entries */
pub const SSB_SPROM8_RAWTS: u32 = 0x00B4;
pub const SSB_SPROM8_RAWTS_RAWTEMP: u32 = 0x01ff;
pub const SSB_SPROM8_RAWTS_RAWTEMP_SHIFT: u32 = 0;
pub const SSB_SPROM8_RAWTS_MEASPOWER: u32 = 0xfe00;
pub const SSB_SPROM8_RAWTS_MEASPOWER_SHIFT: u32 = 9;
pub const SSB_SPROM8_OPT_CORRX: u32 = 0x00B6;
pub const SSB_SPROM8_OPT_CORRX_TEMP_SLOPE: u32 = 0x00ff;
pub const SSB_SPROM8_OPT_CORRX_TEMP_SLOPE_SHIFT: u32 = 0;
pub const SSB_SPROM8_OPT_CORRX_TEMPCORRX: u32 = 0xfc00;
pub const SSB_SPROM8_OPT_CORRX_TEMPCORRX_SHIFT: u32 = 10;
pub const SSB_SPROM8_OPT_CORRX_TEMP_OPTION: u32 = 0x0300;
pub const SSB_SPROM8_OPT_CORRX_TEMP_OPTION_SHIFT: u32 = 8;
/* FOC: freiquency offset correction, HWIQ: H/W IOCAL enable, IQSWP: IQ CAL swap disable */
pub const SSB_SPROM8_HWIQ_IQSWP: u32 = 0x00B8;
pub const SSB_SPROM8_HWIQ_IQSWP_FREQ_CORR: u32 = 0x000f;
pub const SSB_SPROM8_HWIQ_IQSWP_FREQ_CORR_SHIFT: u32 = 0;
pub const SSB_SPROM8_HWIQ_IQSWP_IQCAL_SWP: u32 = 0x0010;
pub const SSB_SPROM8_HWIQ_IQSWP_IQCAL_SWP_SHIFT: u32 = 4;
pub const SSB_SPROM8_HWIQ_IQSWP_HW_IQCAL: u32 = 0x0020;
pub const SSB_SPROM8_HWIQ_IQSWP_HW_IQCAL_SHIFT: u32 = 5;
pub const SSB_SPROM8_TEMPDELTA: u32 = 0x00BC;
pub const SSB_SPROM8_TEMPDELTA_PHYCAL: u32 = 0x00ff;
pub const SSB_SPROM8_TEMPDELTA_PHYCAL_SHIFT: u32 = 0;
pub const SSB_SPROM8_TEMPDELTA_PERIOD: u32 = 0x0f00;
pub const SSB_SPROM8_TEMPDELTA_PERIOD_SHIFT: u32 = 8;
pub const SSB_SPROM8_TEMPDELTA_HYSTERESIS: u32 = 0xf000;
pub const SSB_SPROM8_TEMPDELTA_HYSTERESIS_SHIFT: u32 = 12;

/* There are 4 blocks with power info sharing the same layout */
pub const SSB_SROM8_PWR_INFO_CORE0: u32 = 0x00C0;
pub const SSB_SROM8_PWR_INFO_CORE1: u32 = 0x00E0;
pub const SSB_SROM8_PWR_INFO_CORE2: u32 = 0x0100;
pub const SSB_SROM8_PWR_INFO_CORE3: u32 = 0x0120;

pub const SSB_SROM8_2G_MAXP_ITSSI: u32 = 0x00;
pub const SSB_SPROM8_2G_MAXP: u32 = 0x00FF;
pub const SSB_SPROM8_2G_ITSSI: u32 = 0xFF00;
pub const SSB_SPROM8_2G_ITSSI_SHIFT: u32 = 8;
pub const SSB_SROM8_2G_PA_0: u32 = 0x02; /* 2GHz power amp settings */
pub const SSB_SROM8_2G_PA_1: u32 = 0x04;
pub const SSB_SROM8_2G_PA_2: u32 = 0x06;
pub const SSB_SROM8_5G_MAXP_ITSSI: u32 = 0x08; /* 5GHz ITSSI and 5.3GHz Max Power */
pub const SSB_SPROM8_5G_MAXP: u32 = 0x00FF;
pub const SSB_SPROM8_5G_ITSSI: u32 = 0xFF00;
pub const SSB_SPROM8_5G_ITSSI_SHIFT: u32 = 8;
pub const SSB_SPROM8_5GHL_MAXP: u32 = 0x0A; /* 5.2GHz and 5.8GHz Max Power */
pub const SSB_SPROM8_5GH_MAXP: u32 = 0x00FF;
pub const SSB_SPROM8_5GL_MAXP: u32 = 0xFF00;
pub const SSB_SPROM8_5GL_MAXP_SHIFT: u32 = 8;
pub const SSB_SROM8_5G_PA_0: u32 = 0x0C; /* 5.3GHz power amp settings */
pub const SSB_SROM8_5G_PA_1: u32 = 0x0E;
pub const SSB_SROM8_5G_PA_2: u32 = 0x10;
pub const SSB_SROM8_5GL_PA_0: u32 = 0x12; /* 5.2GHz power amp settings */
pub const SSB_SROM8_5GL_PA_1: u32 = 0x14;
pub const SSB_SROM8_5GL_PA_2: u32 = 0x16;
pub const SSB_SROM8_5GH_PA_0: u32 = 0x18; /* 5.8GHz power amp settings */
pub const SSB_SROM8_5GH_PA_1: u32 = 0x1A;
pub const SSB_SROM8_5GH_PA_2: u32 = 0x1C;

/* TODO: Make it deprecated */
pub const SSB_SPROM8_MAXP_BG: u32 = 0x00C0; /* Max Power 2GHz in path 1 */
pub const SSB_SPROM8_MAXP_BG_MASK: u32 = 0x00FF; /* Mask for Max Power 2GHz */
pub const SSB_SPROM8_ITSSI_BG: u32 = 0xFF00; /* Mask for path 1 itssi_bg */
pub const SSB_SPROM8_ITSSI_BG_SHIFT: u32 = 8;
pub const SSB_SPROM8_PA0B0: u32 = 0x00C2; /* 2GHz power amp settings */
pub const SSB_SPROM8_PA0B1: u32 = 0x00C4;
pub const SSB_SPROM8_PA0B2: u32 = 0x00C6;
pub const SSB_SPROM8_MAXP_A: u32 = 0x00C8; /* Max Power 5.3GHz */
pub const SSB_SPROM8_MAXP_A_MASK: u32 = 0x00FF; /* Mask for Max Power 5.3GHz */
pub const SSB_SPROM8_ITSSI_A: u32 = 0xFF00; /* Mask for path 1 itssi_a */
pub const SSB_SPROM8_ITSSI_A_SHIFT: u32 = 8;
pub const SSB_SPROM8_MAXP_AHL: u32 = 0x00CA; /* Max Power 5.2/5.8GHz */
pub const SSB_SPROM8_MAXP_AH_MASK: u32 = 0x00FF; /* Mask for Max Power 5.8GHz */
pub const SSB_SPROM8_MAXP_AL_MASK: u32 = 0xFF00; /* Mask for Max Power 5.2GHz */
pub const SSB_SPROM8_MAXP_AL_SHIFT: u32 = 8;
pub const SSB_SPROM8_PA1B0: u32 = 0x00CC; /* 5.3GHz power amp settings */
pub const SSB_SPROM8_PA1B1: u32 = 0x00CE;
pub const SSB_SPROM8_PA1B2: u32 = 0x00D0;
pub const SSB_SPROM8_PA1LOB0: u32 = 0x00D2; /* 5.2GHz power amp settings */
pub const SSB_SPROM8_PA1LOB1: u32 = 0x00D4;
pub const SSB_SPROM8_PA1LOB2: u32 = 0x00D6;
pub const SSB_SPROM8_PA1HIB0: u32 = 0x00D8; /* 5.8GHz power amp settings */
pub const SSB_SPROM8_PA1HIB1: u32 = 0x00DA;
pub const SSB_SPROM8_PA1HIB2: u32 = 0x00DC;

pub const SSB_SPROM8_CCK2GPO: u32 = 0x0140; /* CCK power offset */
pub const SSB_SPROM8_OFDM2GPO: u32 = 0x0142; /* 2.4GHz OFDM power offset */
pub const SSB_SPROM8_OFDM5GPO: u32 = 0x0146; /* 5.3GHz OFDM power offset */
pub const SSB_SPROM8_OFDM5GLPO: u32 = 0x014A; /* 5.2GHz OFDM power offset */
pub const SSB_SPROM8_OFDM5GHPO: u32 = 0x014E; /* 5.8GHz OFDM power offset */

pub const SSB_SPROM8_2G_MCSPO: u32 = 0x0152;
pub const SSB_SPROM8_5G_MCSPO: u32 = 0x0162;
pub const SSB_SPROM8_5GL_MCSPO: u32 = 0x0172;
pub const SSB_SPROM8_5GH_MCSPO: u32 = 0x0182;

pub const SSB_SPROM8_CDDPO: u32 = 0x0192;
pub const SSB_SPROM8_STBCPO: u32 = 0x0194;
pub const SSB_SPROM8_BW40PO: u32 = 0x0196;
pub const SSB_SPROM8_BWDUPPO: u32 = 0x0198;

/* Values for boardflags_lo read from SPROM */
pub const SSB_BFL_BTCOEXIST: u32 = 0x0001; /* implements Bluetooth coexistance */
pub const SSB_BFL_PACTRL: u32 = 0x0002; /* GPIO 9 controlling the PA */
pub const SSB_BFL_AIRLINEMODE: u32 = 0x0004; /* implements GPIO 13 radio disable indication */
pub const SSB_BFL_RSSI: u32 = 0x0008; /* software calculates nrssi slope. */
pub const SSB_BFL_ENETSPI: u32 = 0x0010; /* has ephy roboswitch spi */
pub const SSB_BFL_XTAL_NOSLOW: u32 = 0x0020; /* no slow clock available */
pub const SSB_BFL_CCKHIPWR: u32 = 0x0040; /* can do high power CCK transmission */
pub const SSB_BFL_ENETADM: u32 = 0x0080; /* has ADMtek switch */
pub const SSB_BFL_ENETVLAN: u32 = 0x0100; /* can do vlan */
pub const SSB_BFL_AFTERBURNER: u32 = 0x0200; /* supports Afterburner mode */
pub const SSB_BFL_NOPCI: u32 = 0x0400; /* board leaves PCI floating */
pub const SSB_BFL_FEM: u32 = 0x0800; /* supports the Front End Module */
pub const SSB_BFL_EXTLNA: u32 = 0x1000; /* has an external LNA */
pub const SSB_BFL_HGPA: u32 = 0x2000; /* had high gain PA */
pub const SSB_BFL_BTCMOD: u32 = 0x4000; /* BFL_BTCOEXIST is given in alternate GPIOs */
pub const SSB_BFL_ALTIQ: u32 = 0x8000; /* alternate I/Q settings */

/* Values for boardflags_hi read from SPROM */
pub const SSB_BFH_NOPA: u32 = 0x0001; /* has no PA */
pub const SSB_BFH_RSSIINV: u32 = 0x0002; /* RSSI uses positive slope (not TSSI) */
pub const SSB_BFH_PAREF: u32 = 0x0004; /* uses the PARef LDO */
pub const SSB_BFH_3TSWITCH: u32 = 0x0008; /* uses a triple throw switch shared with bluetooth */
pub const SSB_BFH_PHASESHIFT: u32 = 0x0010; /* can support phase shifter */
pub const SSB_BFH_BUCKBOOST: u32 = 0x0020; /* has buck/booster */
pub const SSB_BFH_FEM_BT: u32 = 0x0040; /* has FEM and switch to share antenna with bluetooth */

/* Values for boardflags2_lo read from SPROM */
pub const SSB_BFL2_RXBB_INT_REG_DIS: u32 = 0x0001; /* external RX BB regulator present */
pub const SSB_BFL2_APLL_WAR: u32 = 0x0002; /* alternative A-band PLL settings implemented */
pub const SSB_BFL2_TXPWRCTRL_EN: u32 = 0x0004; /* permits enabling TX Power Control */
pub const SSB_BFL2_2X4_DIV: u32 = 0x0008; /* 2x4 diversity switch */
pub const SSB_BFL2_5G_PWRGAIN: u32 = 0x0010; /* supports 5G band power gain */
pub const SSB_BFL2_PCIEWAR_OVR: u32 = 0x0020; /* overrides ASPM and Clkreq settings */
pub const SSB_BFL2_CAESERS_BRD: u32 = 0x0040; /* is Caesers board (unused) */
pub const SSB_BFL2_BTC3WIRE: u32 = 0x0080; /* used 3-wire bluetooth coexist */
pub const SSB_BFL2_SKWRKFEM_BRD: u32 = 0x0100; /* 4321mcm93 uses Skyworks FEM */
pub const SSB_BFL2_SPUR_WAR: u32 = 0x0200; /* has a workaround for clock-harmonic spurs */
pub const SSB_BFL2_GPLL_WAR: u32 = 0x0400; /* altenative G-band PLL settings implemented */

/* Values for SSB_SPROM1_BINF_CCODE */
pub mod ssb_sprom1_ccode {
pub const SSB_SPROM1CCODE_WORLD: u32 = 0;
pub const SSB_SPROM1CCODE_THAILAND: u32 = 1;
pub const SSB_SPROM1CCODE_ISRAEL: u32 = 2;
pub const SSB_SPROM1CCODE_JORDAN: u32 = 3;
pub const SSB_SPROM1CCODE_CHINA: u32 = 4;
pub const SSB_SPROM1CCODE_JAPAN: u32 = 5;
pub const SSB_SPROM1CCODE_USA_CANADA_ANZ: u32 = 6;
pub const SSB_SPROM1CCODE_EUROPE: u32 = 7;
pub const SSB_SPROM1CCODE_USA_LOW: u32 = 8;
pub const SSB_SPROM1CCODE_JAPAN_HIGH: u32 = 9;
pub const SSB_SPROM1CCODE_ALL: u32 = 10;
pub const SSB_SPROM1CCODE_NONE: u32 = 11;
}

/* Address-Match values and masks (SSB_ADMATCHxxx) */
pub const SSB_ADM_TYPE: u32 = 0x00000003; /* Address type */
pub const SSB_ADM_TYPE0: u32 = 0;
pub const SSB_ADM_TYPE1: u32 = 1;
pub const SSB_ADM_TYPE2: u32 = 2;
pub const SSB_ADM_AD64: u32 = 0x00000004;
pub const SSB_ADM_SZ0: u32 = 0x000000F8; /* Type0 size */
pub const SSB_ADM_SZ0_SHIFT: u32 = 3;
pub const SSB_ADM_SZ1: u32 = 0x000001F8; /* Type1 size */
pub const SSB_ADM_SZ1_SHIFT: u32 = 3;
pub const SSB_ADM_SZ2: u32 = 0x000001F8; /* Type2 size */
pub const SSB_ADM_SZ2_SHIFT: u32 = 3;
pub const SSB_ADM_EN: u32 = 0x00000400; /* Enable */
pub const SSB_ADM_NEG: u32 = 0x00000800; /* Negative decode */
pub const SSB_ADM_BASE0: u32 = 0xFFFFFF00; /* Type0 base address */
pub const SSB_ADM_BASE0_SHIFT: u32 = 8;
pub const SSB_ADM_BASE1: u32 = 0xFFFFF000; /* Type1 base address for the core */
pub const SSB_ADM_BASE1_SHIFT: u32 = 12;
pub const SSB_ADM_BASE2: u32 = 0xFFFF0000; /* Type2 base address for the core */
pub const SSB_ADM_BASE2_SHIFT: u32 = 16;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
