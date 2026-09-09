/* SPDX-License-Identifier: GPL-2.0 */
/* ColdFire 5270/5271 System Integration Module support. */
/* C dependency: <asm/m52xxacr.h> supplies MCF_CLK and MCF_IPSBAR. */

pub const CPU_NAME: &str = "COLDFIRE(m527x)";
pub const CPU_INSTR_PER_JIFFY: usize = 3;
pub const MCF_BUSCLK: usize = MCF_CLK / 2;

pub const MCFICM_INTC0: usize = MCF_IPSBAR + 0x0c00;
pub const MCFICM_INTC1: usize = MCF_IPSBAR + 0x0d00;
pub const MCFINTC_IPRH: usize = 0x00;
pub const MCFINTC_IPRL: usize = 0x04;
pub const MCFINTC_IMRH: usize = 0x08;
pub const MCFINTC_IMRL: usize = 0x0c;
pub const MCFINTC_INTFRCH: usize = 0x10;
pub const MCFINTC_INTFRCL: usize = 0x14;
pub const MCFINTC_IRLR: usize = 0x18;
pub const MCFINTC_IACKL: usize = 0x19;
pub const MCFINTC_ICR0: usize = 0x40;
pub const MCFINT_VECBASE: usize = 64;
pub const MCFINT_UART0: usize = 13;
pub const MCFINT_UART1: usize = 14;
pub const MCFINT_UART2: usize = 15;
pub const MCFINT_I2C0: usize = 17;
pub const MCFINT_QSPI: usize = 18;
pub const MCFINT_FECRX0: usize = 23;
pub const MCFINT_FECTX0: usize = 27;
pub const MCFINT_FECENTC0: usize = 29;
pub const MCFINT_PIT1: usize = 36;
pub const MCFINT2_VECBASE: usize = 128;
pub const MCFINT2_FECRX1: usize = 23;
pub const MCFINT2_FECTX1: usize = 27;
pub const MCFINT2_FECENTC1: usize = 29;
pub const MCF_IRQ_UART0: usize = MCFINT_VECBASE + MCFINT_UART0;
pub const MCF_IRQ_UART1: usize = MCFINT_VECBASE + MCFINT_UART1;
pub const MCF_IRQ_UART2: usize = MCFINT_VECBASE + MCFINT_UART2;
pub const MCF_IRQ_FECRX0: usize = MCFINT_VECBASE + MCFINT_FECRX0;
pub const MCF_IRQ_FECTX0: usize = MCFINT_VECBASE + MCFINT_FECTX0;
pub const MCF_IRQ_FECENTC0: usize = MCFINT_VECBASE + MCFINT_FECENTC0;
pub const MCF_IRQ_FECRX1: usize = MCFINT2_VECBASE + MCFINT2_FECRX1;
pub const MCF_IRQ_FECTX1: usize = MCFINT2_VECBASE + MCFINT2_FECTX1;
pub const MCF_IRQ_FECENTC1: usize = MCFINT2_VECBASE + MCFINT2_FECENTC1;
pub const MCF_IRQ_QSPI: usize = MCFINT_VECBASE + MCFINT_QSPI;
pub const MCF_IRQ_PIT1: usize = MCFINT_VECBASE + MCFINT_PIT1;
pub const MCF_IRQ_I2C0: usize = MCFINT_VECBASE + MCFINT_I2C0;

/* CONFIG_M5271 and CONFIG_M5275 sections are preserved as feature gates. */
#[cfg(feature = "CONFIG_M5271")]
pub mod config_m5271 {
    pub const MCFSIM_DCR: usize = super::MCF_IPSBAR + 0x40;
    pub const MCFSIM_DACR0: usize = super::MCF_IPSBAR + 0x48;
    pub const MCFSIM_DMR0: usize = super::MCF_IPSBAR + 0x4c;
    pub const MCFSIM_DACR1: usize = super::MCF_IPSBAR + 0x50;
    pub const MCFSIM_DMR1: usize = super::MCF_IPSBAR + 0x54;
    pub const MCFQSPI_CS0: usize = 91;
    pub const MCFQSPI_CS1: usize = 92;
    pub const MCFQSPI_CS2: usize = 99;
    pub const MCFQSPI_CS3: usize = 103;
    pub const MCFGPIO_PIN_MAX: usize = 100;
    pub const MCFGPIO_IRQ_MAX: usize = 8;
    pub const MCFGPIO_IRQ_VECBASE: usize = super::MCFINT_VECBASE;
    pub const UART0_ENABLE_MASK: usize = 0x000f;
    pub const UART1_ENABLE_MASK: usize = 0x0ff0;
    pub const UART2_ENABLE_MASK: usize = 0x3000;
}

#[cfg(feature = "CONFIG_M5275")]
pub mod config_m5275 {
    pub const MCFSIM_DMR: usize = super::MCF_IPSBAR + 0x40;
    pub const MCFSIM_DCR: usize = super::MCF_IPSBAR + 0x44;
    pub const MCFSIM_DCFG1: usize = super::MCF_IPSBAR + 0x48;
    pub const MCFSIM_DCFG2: usize = super::MCF_IPSBAR + 0x4c;
    pub const MCFSIM_DBAR0: usize = super::MCF_IPSBAR + 0x50;
    pub const MCFSIM_DMR0: usize = super::MCF_IPSBAR + 0x54;
    pub const MCFSIM_DBAR1: usize = super::MCF_IPSBAR + 0x58;
    pub const MCFSIM_DMR1: usize = super::MCF_IPSBAR + 0x5c;
    pub const MCFQSPI_CS0: usize = 59;
    pub const MCFQSPI_CS1: usize = 60;
    pub const MCFQSPI_CS2: usize = 61;
    pub const MCFQSPI_CS3: usize = 62;
    pub const MCFGPIO_PIN_MAX: usize = 148;
    pub const MCFGPIO_IRQ_MAX: usize = 8;
    pub const MCFGPIO_IRQ_VECBASE: usize = super::MCFINT_VECBASE;
    pub const UART0_ENABLE_MASK: usize = 0x000f;
    pub const UART1_ENABLE_MASK: usize = 0x00f0;
    pub const UART2_ENABLE_MASK: usize = 0x3f00;
}

pub const MCFDMA_BASE0: usize = MCF_IPSBAR + 0x100;
pub const MCFDMA_BASE1: usize = MCF_IPSBAR + 0x140;
pub const MCFDMA_BASE2: usize = MCF_IPSBAR + 0x180;
pub const MCFDMA_BASE3: usize = MCF_IPSBAR + 0x1c0;
pub const MCFUART_BASE0: usize = MCF_IPSBAR + 0x200;
pub const MCFUART_BASE1: usize = MCF_IPSBAR + 0x240;
pub const MCFUART_BASE2: usize = MCF_IPSBAR + 0x280;
pub const MCFFEC_BASE0: usize = MCF_IPSBAR + 0x1000;
pub const MCFFEC_SIZE0: usize = 0x800;
pub const MCFQSPI_BASE: usize = MCF_IPSBAR + 0x340;
pub const MCFQSPI_SIZE: usize = 0x40;
pub const MCFPIT_BASE1: usize = MCF_IPSBAR + 0x150000;
pub const MCFPIT_BASE2: usize = MCF_IPSBAR + 0x160000;
pub const MCFPIT_BASE3: usize = MCF_IPSBAR + 0x170000;
pub const MCFPIT_BASE4: usize = MCF_IPSBAR + 0x180000;
pub const MCFEPORT_EPPAR: usize = MCF_IPSBAR + 0x130000;
pub const MCFEPORT_EPDDR: usize = MCF_IPSBAR + 0x130002;
pub const MCFEPORT_EPIER: usize = MCF_IPSBAR + 0x130003;
pub const MCFEPORT_EPDR: usize = MCF_IPSBAR + 0x130004;
pub const MCFEPORT_EPPDR: usize = MCF_IPSBAR + 0x130005;
pub const MCFEPORT_EPFR: usize = MCF_IPSBAR + 0x130006;
pub const MCF_RCR: usize = MCF_IPSBAR + 0x110000;
pub const MCF_RSR: usize = MCF_IPSBAR + 0x110001;
pub const MCF_RCR_SWRESET: usize = 0x80;
pub const MCF_RCR_FRCSTOUT: usize = 0x40;
pub const MCFI2C_BASE0: usize = MCF_IPSBAR + 0x300;
pub const MCFI2C_SIZE0: usize = 0x40;

macro_rules! gpio_addr_consts {
    ($( $name:ident = $offset:expr ),* $(,)?) => { $(pub const $name: usize = MCF_IPSBAR + $offset;)* };
}

/* GPIO register addresses shared by the two variants. */
gpio_addr_consts! {
    MCFGPIO_PAR_AD=0x100040, MCFGPIO_PAR_BUSCTL=0x100042,
    MCFGPIO_PAR_BS=0x100044, MCFGPIO_PAR_CS=0x100045,
    MCFGPIO_PAR_SDRAM=0x100046, MCFGPIO_PAR_FECI2C=0x100047,
    MCFGPIO_PAR_UART=0x100048, MCFGPIO_PAR_QSPI=0x10004a,
    MCFGPIO_PAR_TIMER=0x10004c
}

#[cfg(feature = "CONFIG_M5271")]
pub mod gpio_m5271 {
    macro_rules! regs { ($($n:ident = $o:expr),* $(,)?) => { $(pub const $n: usize = super::super::MCF_IPSBAR + $o;)* }; }
    regs! {
        MCFGPIO_PODR_ADDR=0x100000, MCFGPIO_PODR_DATAH=0x100001, MCFGPIO_PODR_DATAL=0x100002,
        MCFGPIO_PODR_BUSCTL=0x100003, MCFGPIO_PODR_BS=0x100004, MCFGPIO_PODR_CS=0x100005,
        MCFGPIO_PODR_SDRAM=0x100006, MCFGPIO_PODR_FECI2C=0x100007, MCFGPIO_PODR_UARTH=0x100008,
        MCFGPIO_PODR_UARTL=0x100009, MCFGPIO_PODR_QSPI=0x10000a, MCFGPIO_PODR_TIMER=0x10000b,
        MCFGPIO_PDDR_ADDR=0x100010, MCFGPIO_PDDR_DATAH=0x100011, MCFGPIO_PDDR_DATAL=0x100012,
        MCFGPIO_PDDR_BUSCTL=0x100013, MCFGPIO_PDDR_BS=0x100014, MCFGPIO_PDDR_CS=0x100015,
        MCFGPIO_PDDR_SDRAM=0x100016, MCFGPIO_PDDR_FECI2C=0x100017, MCFGPIO_PDDR_UARTH=0x100018,
        MCFGPIO_PDDR_UARTL=0x100019, MCFGPIO_PDDR_QSPI=0x10001a, MCFGPIO_PDDR_TIMER=0x10001b,
        MCFGPIO_PPDSDR_ADDR=0x100020, MCFGPIO_PPDSDR_DATAH=0x100021, MCFGPIO_PPDSDR_DATAL=0x100022,
        MCFGPIO_PPDSDR_BUSCTL=0x100023, MCFGPIO_PPDSDR_BS=0x100024, MCFGPIO_PPDSDR_CS=0x100025,
        MCFGPIO_PPDSDR_SDRAM=0x100026, MCFGPIO_PPDSDR_FECI2C=0x100027, MCFGPIO_PPDSDR_UARTH=0x100028,
        MCFGPIO_PPDSDR_UARTL=0x100029, MCFGPIO_PPDSDR_QSPI=0x10002a, MCFGPIO_PPDSDR_TIMER=0x10002b,
        MCFGPIO_PCLRR_ADDR=0x100030, MCFGPIO_PCLRR_DATAH=0x100031, MCFGPIO_PCLRR_DATAL=0x100032,
        MCFGPIO_PCLRR_BUSCTL=0x100033, MCFGPIO_PCLRR_BS=0x100034, MCFGPIO_PCLRR_CS=0x100035,
        MCFGPIO_PCLRR_SDRAM=0x100036, MCFGPIO_PCLRR_FECI2C=0x100037, MCFGPIO_PCLRR_UARTH=0x100038,
        MCFGPIO_PCLRR_UARTL=0x100039, MCFGPIO_PCLRR_QSPI=0x10003a, MCFGPIO_PCLRR_TIMER=0x10003b
    }
    pub const MCFGPIO_PODR: usize = MCFGPIO_PODR_ADDR; pub const MCFGPIO_PDDR: usize = MCFGPIO_PDDR_ADDR;
    pub const MCFGPIO_PPDR: usize = MCFGPIO_PPDSDR_ADDR; pub const MCFGPIO_SETR: usize = MCFGPIO_PPDSDR_ADDR;
    pub const MCFGPIO_CLRR: usize = MCFGPIO_PCLRR_ADDR;
}

#[cfg(feature = "CONFIG_M5275")]
pub mod gpio_m5275 {
    macro_rules! regs { ($($n:ident = $o:expr),* $(,)?) => { $(pub const $n: usize = super::super::MCF_IPSBAR + $o;)* }; }
    regs! { MCFGPIO_PODR_BUSCTL=0x100004, MCFGPIO_PODR_ADDR=0x100005, MCFGPIO_PODR_CS=0x100008,
        MCFGPIO_PODR_FEC0H=0x10000a, MCFGPIO_PODR_FEC0L=0x10000b, MCFGPIO_PODR_FECI2C=0x10000c,
        MCFGPIO_PODR_QSPI=0x10000d, MCFGPIO_PODR_SDRAM=0x10000e, MCFGPIO_PODR_TIMERH=0x10000f,
        MCFGPIO_PODR_TIMERL=0x100010, MCFGPIO_PODR_UARTL=0x100011, MCFGPIO_PODR_FEC1H=0x100012,
        MCFGPIO_PODR_FEC1L=0x100013, MCFGPIO_PODR_BS=0x100014, MCFGPIO_PODR_IRQ=0x100015,
        MCFGPIO_PODR_USBH=0x100016, MCFGPIO_PODR_USBL=0x100017, MCFGPIO_PODR_UARTH=0x100018,
        MCFGPIO_PDDR_BUSCTL=0x100020, MCFGPIO_PDDR_ADDR=0x100021, MCFGPIO_PDDR_CS=0x100024,
        MCFGPIO_PDDR_FEC0H=0x100026, MCFGPIO_PDDR_FEC0L=0x100027, MCFGPIO_PDDR_FECI2C=0x100028,
        MCFGPIO_PDDR_QSPI=0x100029, MCFGPIO_PDDR_SDRAM=0x10002a, MCFGPIO_PDDR_TIMERH=0x10002b,
        MCFGPIO_PDDR_TIMERL=0x10002c, MCFGPIO_PDDR_UARTL=0x10002d, MCFGPIO_PDDR_FEC1H=0x10002e,
        MCFGPIO_PDDR_FEC1L=0x10002f, MCFGPIO_PDDR_BS=0x100030, MCFGPIO_PDDR_IRQ=0x100031,
        MCFGPIO_PDDR_USBH=0x100032, MCFGPIO_PDDR_USBL=0x100033, MCFGPIO_PDDR_UARTH=0x100034 }
    pub const MCFGPIO_PODR: usize = MCFGPIO_PODR_BUSCTL; pub const MCFGPIO_PDDR: usize = MCFGPIO_PDDR_BUSCTL;
    pub const MCFGPIO_PPDR: usize = super::MCF_IPSBAR + 0x10003c; pub const MCFGPIO_SETR: usize = MCFGPIO_PPDR;
    pub const MCFGPIO_CLRR: usize = super::MCF_IPSBAR + 0x100058;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
