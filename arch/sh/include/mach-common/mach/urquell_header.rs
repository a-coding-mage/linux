/* SPDX-License-Identifier: GPL-2.0 */

/*
 * ------ 0x00000000 ------------------------------------
 *  CS0 | (SW1,SW47)    EEPROM, SRAM, NOR FLASH
 * -----+ 0x04000000 ------------------------------------
 *  CS1 | (SW47)        SRAM, SRAM-LAN-PCMCIA, NOR FLASH
 * -----+ 0x08000000 ------------------------------------
 *  CS2 |               DDR3
 *  CS3 |
 * -----+ 0x10000000 ------------------------------------
 *  CS4 |               PCIe
 * -----+ 0x14000000 ------------------------------------
 *  CS5 | (SW47)        LRAM/URAM, SRAM-LAN-PCMCIA
 * -----+ 0x18000000 ------------------------------------
 *  CS6 |               ATA, NAND FLASH
 *  CS7 |               SH7786 register
 * -----+------------------------------------------------
 */

pub const NOR_FLASH_ADDR: u32 = 0x00000000;
pub const NOR_FLASH_SIZE: u32 = 0x04000000;

pub const CS1_BASE: u32 = 0x05000000;
pub const CS5_BASE: u32 = 0x15000000;
pub const FPGA_BASE: u32 = CS1_BASE;

/* C macros BOARDREG(ofs) and UBOARDREG(ofs); pass the corresponding *_OFS constant. */
macro_rules! BOARDREG {
    ($ofs:expr) => {
        FPGA_BASE + $ofs
    };
}

macro_rules! UBOARDREG {
    ($ofs:expr) => {
        0xa0000000u32 + FPGA_BASE + $ofs
    };
}

pub const SRSTR_OFS: u32 = 0x0000; /* System reset register */
pub const BDMR_OFS: u32 = 0x0010; /* Board operating mode resister */
pub const IRL0SR_OFS: u32 = 0x0020; /* IRL0 Status register */
pub const IRL0MSKR_OFS: u32 = 0x0030; /* IRL0 Mask register */
pub const IRL1SR_OFS: u32 = 0x0040; /* IRL1 Status register */
pub const IRL1MSKR_OFS: u32 = 0x0050; /* IRL1 Mask register */
pub const IRL2SR_OFS: u32 = 0x0060; /* IRL2 Status register */
pub const IRL2MSKR_OFS: u32 = 0x0070; /* IRL2 Mask register */
pub const IRL3SR_OFS: u32 = 0x0080; /* IRL3 Status register */
pub const IRL3MSKR_OFS: u32 = 0x0090; /* IRL3 Mask register */
pub const SOFTINTR_OFS: u32 = 0x0120; /* Softwear Interrupt register */
pub const SLEDR_OFS: u32 = 0x0130; /* LED control resister */
pub const MAPSCIFSWR_OFS: u32 = 0x0140; /* Map/SCIF Switch register */
pub const FPVERR_OFS: u32 = 0x0150; /* FPGA Version register */
pub const FPDATER_OFS: u32 = 0x0160; /* FPGA Date register */
pub const FPYEARR_OFS: u32 = 0x0170; /* FPGA Year register */
pub const TCLKCR_OFS: u32 = 0x0180; /* TCLK Control register */
pub const DIPSWMR_OFS: u32 = 0x1000; /* DIPSW monitor register */
pub const FPODR_OFS: u32 = 0x1010; /* Output port data register */
pub const ATACNR_OFS: u32 = 0x1020; /* ATA-CN Control/status register */
pub const FPINDR_OFS: u32 = 0x1030; /* Input port data register */
pub const MDSWMR_OFS: u32 = 0x1040; /* MODE SW monitor register */
pub const DDR3BUPCR_OFS: u32 = 0x1050; /* DDR3 Backup control register */
pub const SSICODECCR_OFS: u32 = 0x1060; /* SSI-CODEC control register */
pub const PCIESLOTSR_OFS: u32 = 0x1070; /* PCIexpress Slot status register */
pub const ETHERPORTSR_OFS: u32 = 0x1080; /* EtherPhy Port status register */
pub const LATCHCR_OFS: u32 = 0x3000; /* Latch control register */
pub const LATCUAR_OFS: u32 = 0x3010; /* Latch upper address register */
pub const LATCLAR_OFS: u32 = 0x3012; /* Latch lower address register */
pub const LATCLUDR_OFS: u32 = 0x3024; /* Latch D31-16 register */
pub const LATCLLDR_OFS: u32 = 0x3026; /* Latch D15-0 register */

pub const CHARLED_OFS: u32 = 0x2000; /* Character LED */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
