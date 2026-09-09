/*
 * m54xxpci.h -- ColdFire 547x and 548x PCI bus support
 *
 * C translation of the original header. CONFIG_MBAR is supplied by the
 * surrounding target configuration.
 */

// The core set of PCI support registers are mapped into the MBAR region.
pub const PCIIDR: u32 = CONFIG_MBAR + 0xb00; // PCI device/vendor ID
pub const PCISCR: u32 = CONFIG_MBAR + 0xb04; // PCI status/command
pub const PCICCRIR: u32 = CONFIG_MBAR + 0xb08; // PCI class/revision
pub const PCICR1: u32 = CONFIG_MBAR + 0xb0c; // PCI configuration 1
pub const PCIBAR0: u32 = CONFIG_MBAR + 0xb10; // PCI base address 0
pub const PCIBAR1: u32 = CONFIG_MBAR + 0xb14; // PCI base address 1
pub const PCICCPR: u32 = CONFIG_MBAR + 0xb28; // PCI cardbus CIS pointer
pub const PCISID: u32 = CONFIG_MBAR + 0xb2c; // PCI subsystem IDs
pub const PCIERBAR: u32 = CONFIG_MBAR + 0xb30; // PCI expansion ROM
pub const PCICPR: u32 = CONFIG_MBAR + 0xb34; // PCI capabilities pointer
pub const PCICR2: u32 = CONFIG_MBAR + 0xb3c; // PCI configuration 2

pub const PCIGSCR: u32 = CONFIG_MBAR + 0xb60; // Global status/control
pub const PCITBATR0: u32 = CONFIG_MBAR + 0xb64; // Target base translation 0
pub const PCITBATR1: u32 = CONFIG_MBAR + 0xb68; // Target base translation 1
pub const PCITCR: u32 = CONFIG_MBAR + 0xb6c; // Target control
pub const PCIIW0BTAR: u32 = CONFIG_MBAR + 0xb70; // Initiator window 0
pub const PCIIW1BTAR: u32 = CONFIG_MBAR + 0xb74; // Initiator window 1
pub const PCIIW2BTAR: u32 = CONFIG_MBAR + 0xb78; // Initiator window 2
pub const PCIIWCR: u32 = CONFIG_MBAR + 0xb80; // Initiator window config
pub const PCIICR: u32 = CONFIG_MBAR + 0xb84; // Initiator control
pub const PCIISR: u32 = CONFIG_MBAR + 0xb88; // Initiator status
pub const PCICAR: u32 = CONFIG_MBAR + 0xbf8; // Configuration address

pub const PCITPSR: u32 = CONFIG_MBAR + 0x8400; // TX packet size
pub const PCITSAR: u32 = CONFIG_MBAR + 0x8404; // TX start address
pub const PCITTCR: u32 = CONFIG_MBAR + 0x8408; // TX transaction control
pub const PCITER: u32 = CONFIG_MBAR + 0x840c; // TX enables
pub const PCITNAR: u32 = CONFIG_MBAR + 0x8410; // TX next address
pub const PCITLWR: u32 = CONFIG_MBAR + 0x8414; // TX last word
pub const PCITDCR: u32 = CONFIG_MBAR + 0x8418; // TX done counts
pub const PCITSR: u32 = CONFIG_MBAR + 0x841c; // TX status
pub const PCITFDR: u32 = CONFIG_MBAR + 0x8440; // TX FIFO data
pub const PCITFSR: u32 = CONFIG_MBAR + 0x8444; // TX FIFO status
pub const PCITFCR: u32 = CONFIG_MBAR + 0x8448; // TX FIFO control
pub const PCITFAR: u32 = CONFIG_MBAR + 0x844c; // TX FIFO alarm
pub const PCITFRPR: u32 = CONFIG_MBAR + 0x8450; // TX FIFO read pointer
pub const PCITFWPR: u32 = CONFIG_MBAR + 0x8454; // TX FIFO write pointer

pub const PCIRPSR: u32 = CONFIG_MBAR + 0x8480; // RX packet size
pub const PCIRSAR: u32 = CONFIG_MBAR + 0x8484; // RX start address
pub const PCIRTCR: u32 = CONFIG_MBAR + 0x8488; // RX transaction control
pub const PCIRER: u32 = CONFIG_MBAR + 0x848c; // RX enables
pub const PCIRNAR: u32 = CONFIG_MBAR + 0x8490; // RX next address
pub const PCIRDCR: u32 = CONFIG_MBAR + 0x8498; // RX done counts
pub const PCIRSR: u32 = CONFIG_MBAR + 0x849c; // RX status
pub const PCIRFDR: u32 = CONFIG_MBAR + 0x84c0; // RX FIFO data
pub const PCIRFSR: u32 = CONFIG_MBAR + 0x84c4; // RX FIFO status
pub const PCIRFCR: u32 = CONFIG_MBAR + 0x84c8; // RX FIFO control
pub const PCIRFAR: u32 = CONFIG_MBAR + 0x84cc; // RX FIFO alarm
pub const PCIRFRPR: u32 = CONFIG_MBAR + 0x84d0; // RX FIFO read pointer
pub const PCIRFWPR: u32 = CONFIG_MBAR + 0x84d4; // RX FIFO write pointer

pub const PACR: u32 = CONFIG_MBAR + 0xc00; // PCI arbiter control
pub const PASR: u32 = CONFIG_MBAR + 0xc04; // PCI arbiter status

// Definitions for the Global status and control register.
pub const PCIGSCR_PE: u32 = 0x20000000;
pub const PCIGSCR_SE: u32 = 0x10000000;
pub const PCIGSCR_XCLKBIN: u32 = 0x07000000;
pub const PCIGSCR_PEE: u32 = 0x00002000;
pub const PCIGSCR_SEE: u32 = 0x00001000;
pub const PCIGSCR_RESET: u32 = 0x00000001;

// Bit definitions for the PCICAR configuration address register.
pub const PCICAR_E: u32 = 0x80000000;
pub const PCICAR_BUSN: u32 = 16;
pub const PCICAR_DEVFNN: u32 = 8;
pub const PCICAR_DWORDN: u32 = 0;

// Creates initiator-window register values from the desired addresses.
#[macro_export]
macro_rules! WXBTAR {
    ($hostaddr:expr, $pciaddr:expr, $size:expr) => {
        (($hostaddr & 0xff000000) |
            ((($size - 1) & 0xff000000) >> 8) |
            (($pciaddr & 0xff000000) >> 16))
    };
}

pub const PCIIWCR_W0_MEM: u32 = 0x00000000;
pub const PCIIWCR_W0_IO: u32 = 0x08000000;
pub const PCIIWCR_W0_MRD: u32 = 0x00000000;
pub const PCIIWCR_W0_MRDL: u32 = 0x02000000;
pub const PCIIWCR_W0_MRDM: u32 = 0x04000000;
pub const PCIIWCR_W0_E: u32 = 0x01000000;
pub const PCIIWCR_W1_MEM: u32 = 0x00000000;
pub const PCIIWCR_W1_IO: u32 = 0x00080000;
pub const PCIIWCR_W1_MRD: u32 = 0x00000000;
pub const PCIIWCR_W1_MRDL: u32 = 0x00020000;
pub const PCIIWCR_W1_MRDM: u32 = 0x00040000;
pub const PCIIWCR_W1_E: u32 = 0x00010000;

// Bit definitions for the PCIBATR registers.
pub const PCITBATR0_E: u32 = 0x00000001;
pub const PCITBATR1_E: u32 = 0x00000001;

// PCI arbiter support definitions and macros.
pub const PACR_INTMPRI: u32 = 0x00000001;
#[inline]
pub const fn PACR_EXTMPRI(x: u32) -> u32 { (x & 0x1f) << 1 }
pub const PACR_INTMINTE: u32 = 0x00010000;
#[inline]
pub const fn PACR_EXTMINTE(x: u32) -> u32 { (x & 0x1f) << 17 }
pub const PACR_PKMD: u32 = 0x40000000;
pub const PACR_DS: u32 = 0x80000000;

#[inline]
pub const fn PCICR1_CL(x: u32) -> u32 { x & 0xf }
#[inline]
pub const fn PCICR1_LT(x: u32) -> u32 { (x & 0xff) << 8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
