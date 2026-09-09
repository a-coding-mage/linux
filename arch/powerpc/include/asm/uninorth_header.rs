/* SPDX-License-Identifier: GPL-2.0 */
/*
 * uninorth.h: definitions for using the "UniNorth" host bridge chip
 *             from Apple. This chip is used on "Core99" machines
 *             This also includes U2 used on more recent MacRISC2/3
 *             machines and U3 (G5)
 */
/* The original declarations are available only when building the kernel. */

/* Uni-N and U3 config space register definitions (little endian). */

/* Address ranges selection. This one should work with Bandit too. Not U3. */
pub const UNI_N_ADDR_SELECT: u32 = 0x48;
pub const UNI_N_ADDR_COARSE_MASK: u32 = 0xffff0000; /* 256Mb regions at *0000000 */
pub const UNI_N_ADDR_FINE_MASK: u32 = 0x0000ffff; /* 16Mb regions at f*000000 */

/* AGP registers. Not U3. */
pub const UNI_N_CFG_GART_BASE: u32 = 0x8c;
pub const UNI_N_CFG_AGP_BASE: u32 = 0x90;
pub const UNI_N_CFG_GART_CTRL: u32 = 0x94;
pub const UNI_N_CFG_INTERNAL_STATUS: u32 = 0x98;
pub const UNI_N_CFG_GART_DUMMY_PAGE: u32 = 0xa4;

/* UNI_N_CFG_GART_CTRL bits definitions. */
pub const UNI_N_CFG_GART_INVAL: u32 = 0x00000001;
pub const UNI_N_CFG_GART_ENABLE: u32 = 0x00000100;
pub const UNI_N_CFG_GART_2xRESET: u32 = 0x00010000;
pub const UNI_N_CFG_GART_DISSBADET: u32 = 0x00020000;
/* The following seems to only be used only on U3 <j.glisse@gmail.com>. */
pub const U3_N_CFG_GART_SYNCMODE: u32 = 0x00040000;
pub const U3_N_CFG_GART_PERFRD: u32 = 0x00080000;
pub const U3_N_CFG_GART_B2BGNT: u32 = 0x00200000;
pub const U3_N_CFG_GART_FASTDDR: u32 = 0x00400000;

/*
 * My understanding of UniNorth AGP as of UniNorth rev 1.0x,
 * revision 1.5 (x4 AGP) may need further changes.
 *
 * AGP_BASE register contains the base address of the AGP aperture on
 * the AGP bus. It doesn't seem to be visible to the CPU as of UniNorth 1.x,
 * even if decoding of this address range is enabled in the address select
 * register. Apparently, the only supported bases are 256Mb multiples
 * (high 4 bits of that register).
 *
 * GART_BASE register appears to contain the physical address of the GART
 * in system memory in the high address bits (page aligned), and the
 * GART size in the low order bits (number of GART pages).
 *
 * The GART format itself is one 32bits word per physical memory page.
 * This word contains, in little-endian format (!!!), the physical address
 * of the page in the high bits, and what appears to be an "enable" bit
 * in the LSB bit (0) that must be set to 1 when the entry is valid.
 *
 * Obviously, the GART is not cache coherent and so any change to it
 * must be flushed to memory (or maybe just make the GART space non
 * cachable). AGP memory itself doesn't seem to be cache coherent neither.
 *
 * In order to invalidate the GART (which is probably necessary to inval
 * the bridge internal TLBs), the following sequence has to be written,
 * in order, to the GART_CTRL register:
 *
 *   UNI_N_CFG_GART_ENABLE | UNI_N_CFG_GART_INVAL
 *   UNI_N_CFG_GART_ENABLE
 *   UNI_N_CFG_GART_ENABLE | UNI_N_CFG_GART_2xRESET
 *   UNI_N_CFG_GART_ENABLE
 */

/* Uni-N memory mapped register definitions. Those registers are Big-Endian. */

/* Version of the UniNorth chip. Known versions: 3, 7 and 8. */
pub const UNI_N_VERSION: u32 = 0x0000;
pub const UNI_N_VERSION_107: u32 = 0x0003; /* 1.0.7 */
pub const UNI_N_VERSION_10A: u32 = 0x0007; /* 1.0.10 */
pub const UNI_N_VERSION_150: u32 = 0x0011; /* 1.5 */
pub const UNI_N_VERSION_200: u32 = 0x0024; /* 2.0 */
pub const UNI_N_VERSION_PANGEA: u32 = 0x00C0; /* Integrated U1 + K */
pub const UNI_N_VERSION_INTREPID: u32 = 0x00D2; /* Integrated U2 + K */
pub const UNI_N_VERSION_300: u32 = 0x0030; /* 3.0 (U3 on G5) */

/* This register is used to enable/disable various clocks. */
pub const UNI_N_CLOCK_CNTL: u32 = 0x0020;
pub const UNI_N_CLOCK_CNTL_PCI: u32 = 0x00000001; /* PCI2 clock control */
pub const UNI_N_CLOCK_CNTL_GMAC: u32 = 0x00000002; /* GMAC clock control */
pub const UNI_N_CLOCK_CNTL_FW: u32 = 0x00000004; /* FireWire clock control */
pub const UNI_N_CLOCK_CNTL_ATA100: u32 = 0x00000010; /* ATA-100 clock control (U2) */

/* Power Management control. */
pub const UNI_N_POWER_MGT: u32 = 0x0030;
pub const UNI_N_POWER_MGT_NORMAL: u32 = 0x00;
pub const UNI_N_POWER_MGT_IDLE2: u32 = 0x01;
pub const UNI_N_POWER_MGT_SLEEP: u32 = 0x02;

/* This register is configured by Darwin depending on the UniN revision. */
pub const UNI_N_ARB_CTRL: u32 = 0x0040;
pub const UNI_N_ARB_CTRL_QACK_DELAY_SHIFT: u32 = 15;
pub const UNI_N_ARB_CTRL_QACK_DELAY_MASK: u32 = 0x0e1f8000;
pub const UNI_N_ARB_CTRL_QACK_DELAY: u32 = 0x30;
pub const UNI_N_ARB_CTRL_QACK_DELAY105: u32 = 0x00;

/* This one might return the CPU number of the CPU reading it. */
pub const UNI_N_CPU_NUMBER: u32 = 0x0050;

/* This register appears to be read by the bootROM for non-recoverable reset. */
pub const UNI_N_HWINIT_STATE: u32 = 0x0070;
pub const UNI_N_HWINIT_STATE_SLEEPING: u32 = 0x01;
pub const UNI_N_HWINIT_STATE_RUNNING: u32 = 0x02;
/* Used by the bootROM to know the second CPU has started. */
pub const UNI_N_HWINIT_STATE_CPU1_FLAG: u32 = 0x10000000;

/* This register controls AACK delay. */
pub const UNI_N_AACK_DELAY: u32 = 0x0100;
pub const UNI_N_AACK_DELAY_ENABLE: u32 = 0x00000001;

/* Clock status for Intrepid. */
pub const UNI_N_CLOCK_STOP_STATUS0: u32 = 0x0150;
pub const UNI_N_CLOCK_STOPPED_EXTAGP: u32 = 0x00200000;
pub const UNI_N_CLOCK_STOPPED_AGPDEL: u32 = 0x00100000;
pub const UNI_N_CLOCK_STOPPED_I2S0_45_49: u32 = 0x00080000;
pub const UNI_N_CLOCK_STOPPED_I2S0_18: u32 = 0x00040000;
pub const UNI_N_CLOCK_STOPPED_I2S1_45_49: u32 = 0x00020000;
pub const UNI_N_CLOCK_STOPPED_I2S1_18: u32 = 0x00010000;
pub const UNI_N_CLOCK_STOPPED_TIMER: u32 = 0x00008000;
pub const UNI_N_CLOCK_STOPPED_SCC_RTCLK18: u32 = 0x00004000;
pub const UNI_N_CLOCK_STOPPED_SCC_RTCLK32: u32 = 0x00002000;
pub const UNI_N_CLOCK_STOPPED_SCC_VIA32: u32 = 0x00001000;
pub const UNI_N_CLOCK_STOPPED_SCC_SLOT0: u32 = 0x00000800;
pub const UNI_N_CLOCK_STOPPED_SCC_SLOT1: u32 = 0x00000400;
pub const UNI_N_CLOCK_STOPPED_SCC_SLOT2: u32 = 0x00000200;
pub const UNI_N_CLOCK_STOPPED_PCI_FBCLKO: u32 = 0x00000100;
pub const UNI_N_CLOCK_STOPPED_VEO0: u32 = 0x00000080;
pub const UNI_N_CLOCK_STOPPED_VEO1: u32 = 0x00000040;
pub const UNI_N_CLOCK_STOPPED_USB0: u32 = 0x00000020;
pub const UNI_N_CLOCK_STOPPED_USB1: u32 = 0x00000010;
pub const UNI_N_CLOCK_STOPPED_USB2: u32 = 0x00000008;
pub const UNI_N_CLOCK_STOPPED_32: u32 = 0x00000004;
pub const UNI_N_CLOCK_STOPPED_45: u32 = 0x00000002;
pub const UNI_N_CLOCK_STOPPED_49: u32 = 0x00000001;

pub const UNI_N_CLOCK_STOP_STATUS1: u32 = 0x0160;
pub const UNI_N_CLOCK_STOPPED_PLL4REF: u32 = 0x00080000;
pub const UNI_N_CLOCK_STOPPED_CPUDEL: u32 = 0x00040000;
pub const UNI_N_CLOCK_STOPPED_CPU: u32 = 0x00020000;
pub const UNI_N_CLOCK_STOPPED_BUF_REFCKO: u32 = 0x00010000;
pub const UNI_N_CLOCK_STOPPED_PCI2: u32 = 0x00008000;
pub const UNI_N_CLOCK_STOPPED_FW: u32 = 0x00004000;
pub const UNI_N_CLOCK_STOPPED_GB: u32 = 0x00002000;
pub const UNI_N_CLOCK_STOPPED_ATA66: u32 = 0x00001000;
pub const UNI_N_CLOCK_STOPPED_ATA100: u32 = 0x00000800;
pub const UNI_N_CLOCK_STOPPED_MAX: u32 = 0x00000400;
pub const UNI_N_CLOCK_STOPPED_PCI1: u32 = 0x00000200;
pub const UNI_N_CLOCK_STOPPED_KLPCI: u32 = 0x00000100;
pub const UNI_N_CLOCK_STOPPED_USB0PCI: u32 = 0x00000080;
pub const UNI_N_CLOCK_STOPPED_USB1PCI: u32 = 0x00000040;
pub const UNI_N_CLOCK_STOPPED_USB2PCI: u32 = 0x00000020;
pub const UNI_N_CLOCK_STOPPED_7PCI1: u32 = 0x00000008;
pub const UNI_N_CLOCK_STOPPED_AGP: u32 = 0x00000004;
pub const UNI_N_CLOCK_STOPPED_PCI0: u32 = 0x00000002;
pub const UNI_N_CLOCK_STOPPED_18: u32 = 0x00000001;

/* Intrepid register to OF do-platform-clockspreading. */
pub const UNI_N_CLOCK_SPREADING: u32 = 0x190;

/* Uninorth 1.5 rev. has additional perf. monitor registers at 0xf00-0xf50. */

/* U3 specific registers. */
pub const U3_TOGGLE_REG: u32 = 0x00e0;
pub const U3_PMC_START_STOP: u32 = 0x0001;
pub const U3_MPIC_RESET: u32 = 0x0002;
pub const U3_MPIC_OUTPUT_ENABLE: u32 = 0x0004;

/* U3 API PHY Config 1. */
pub const U3_API_PHY_CONFIG_1: u32 = 0x23030;

/* U3 HyperTransport registers. */
pub const U3_HT_CONFIG_BASE: u32 = 0x70000;
pub const U3_HT_LINK_COMMAND: u32 = 0x100;
pub const U3_HT_LINK_CONFIG: u32 = 0x110;
pub const U3_HT_LINK_FREQ: u32 = 0x120;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
